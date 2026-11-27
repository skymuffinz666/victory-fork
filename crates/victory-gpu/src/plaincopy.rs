//! Fault-protected reads into host-owned memory.
//!
//! Adapted from Kelvaroq patch 0044-copy-plain, revision e062a82.
//! Copyright (c) 2026 Stellarr Studio SAS and the Kelvaroq contributors.
//! SPDX-License-Identifier: GPL-3.0-or-later
//!
//! Only the read at our REP MOVSB instruction can recover. Destination
//! faults and unrelated exceptions remain with their owners. A failed copy
//! retries the entire range through the existing kernel/sparse read path.

use std::sync::atomic::{AtomicU64, Ordering::Relaxed};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Path { Plain, Fault, Small, Cooling, Disabled }

static COUNTS: [AtomicU64; 5] = [const { AtomicU64::new(0) }; 5];

pub(crate) fn per_frame(frames: f64) -> [f64; 5] {
    COUNTS.each_ref().map(|n| n.swap(0, Relaxed) as f64 / frames)
}

/// On false, retry the original range through the kernel.
///
/// # Safety
/// `dst` is a writable host allocation of `len` bytes, disjoint from title
/// memory. The title may change or unmap `src` during the read.
pub(crate) unsafe fn copy(src: *const u8, dst: *mut u8, len: usize) -> bool {
    #[cfg(target_arch = "x86_64")]
    let path = {
        static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
        if *ON.get_or_init(|| std::env::var_os("VICTORY_COPY_PLAIN").is_some_and(|v| v == "1") && x64::install()) {
            // SAFETY: caller supplies host memory, and the handler is installed.
            unsafe { x64::attempt(src, dst, len, &x64::FAULTS) }
        } else {
            Path::Disabled
        }
    };
    #[cfg(not(target_arch = "x86_64"))]
    let path = Path::Disabled;
    COUNTS[path as usize].fetch_add(1, Relaxed);
    path == Path::Plain
}

#[cfg(target_arch = "x86_64")]
mod x64 {
    use super::*;
    use std::collections::BTreeMap;
    use std::sync::{OnceLock, RwLock};
    use std::time::Instant;
    use windows_sys::Win32::Foundation::{EXCEPTION_ACCESS_VIOLATION, EXCEPTION_GUARD_PAGE, EXCEPTION_IN_PAGE_ERROR};
    use windows_sys::Win32::System::Diagnostics::Debug::{
        AddVectoredExceptionHandler, CONTEXT, EXCEPTION_CONTINUE_EXECUTION,
        EXCEPTION_CONTINUE_SEARCH, EXCEPTION_POINTERS, EXCEPTION_RECORD,
    };

    // Preserve nonvolatile registers and Windows x64 unwind information on
    // both exits, including when REP has copied only a prefix.
    core::arch::global_asm!(
        ".text",
        ".globl victory_plain_copy",
        ".globl victory_plain_copy_at",
        ".globl victory_plain_copy_exit",
        ".seh_proc victory_plain_copy",
        "victory_plain_copy:",
        "push rsi",
        ".seh_pushreg rsi",
        "push rdi",
        ".seh_pushreg rdi",
        "sub rsp, 8",
        ".seh_stackalloc 8",
        ".seh_endprologue",
        ".seh_handler {unclaimed_guard}, @except",
        "mov rdi, rcx",
        "mov rsi, rdx",
        "mov rcx, r8",
        "victory_plain_copy_at:",
        "rep movsb",
        "xor eax, eax",
        "victory_plain_copy_exit:",
        "add rsp, 8",
        "pop rdi",
        "pop rsi",
        "ret",
        ".seh_endproc",
        unclaimed_guard = sym unclaimed_guard,
    );

    unsafe extern "C" {
        fn victory_plain_copy(dst: *mut u8, src: *const u8, len: usize) -> usize;
        static victory_plain_copy_at: u8;
        static victory_plain_copy_exit: u8;
    }

    fn recover_read(record: &EXCEPTION_RECORD, context: &mut CONTEXT) -> bool {
        if context.Rip != (&raw const victory_plain_copy_at) as u64
            || record.NumberParameters < 2 || record.ExceptionInformation[0] != 0 {
            return false;
        }
        context.Rax = (record.ExceptionInformation[1] as u64 >> 16) + 1;
        context.Rip = (&raw const victory_plain_copy_exit) as u64;
        true
    }

    unsafe extern "system" fn on_fault(info: *mut EXCEPTION_POINTERS) -> i32 {
        // SAFETY: Windows supplies the exception and resumable context.
        let (record, context) = unsafe { (&*(*info).ExceptionRecord, &mut *(*info).ContextRecord) };
        if matches!(record.ExceptionCode, EXCEPTION_ACCESS_VIOLATION | EXCEPTION_IN_PAGE_ERROR)
            && recover_read(record, context) {
            EXCEPTION_CONTINUE_EXECUTION
        } else {
            EXCEPTION_CONTINUE_SEARCH
        }
    }

    // PAGE_GUARD notifications reach every vectored handler first. Only
    // an unclaimed source-read guard reaches this assembly frame handler.
    unsafe extern "system" fn unclaimed_guard(
        record: *const EXCEPTION_RECORD, _frame: *mut std::ffi::c_void,
        context: *mut CONTEXT, _dispatcher: *mut std::ffi::c_void,
    ) -> i32 {
        use windows_sys::Win32::System::Kernel::{ExceptionContinueExecution, ExceptionContinueSearch};
        // SAFETY: Windows supplies this assembly frame's exception state.
        let (record, context) = unsafe { (&*record, &mut *context) };
        if record.ExceptionCode == EXCEPTION_GUARD_PAGE && recover_read(record, context) {
            ExceptionContinueExecution
        } else {
            ExceptionContinueSearch
        }
    }

    pub(super) fn install() -> bool {
        static INSTALLED: OnceLock<bool> = OnceLock::new();
        *INSTALLED.get_or_init(|| {
            // SAFETY: the handler stays loaded for the process lifetime.
            let installed = !unsafe { AddVectoredExceptionHandler(1, Some(on_fault)) }.is_null();
            if !installed {
                tracing::warn!("could not install the plain copy handler, using ReadProcessMemory");
            }
            installed
        })
    }

    fn now_ms() -> u64 {
        static START: OnceLock<Instant> = OnceLock::new();
        START.get_or_init(Instant::now).elapsed().as_millis() as u64
    }

    pub(super) struct Faults {
        latest: AtomicU64,
        recent: RwLock<Recent>,
    }
    struct Recent { deadlines: BTreeMap<usize, u64>, prune_at: u64 }
    pub(super) static FAULTS: Faults = Faults::new();

    impl Faults {
        const fn new() -> Self {
            Self { latest: AtomicU64::new(0), recent: RwLock::new(Recent { deadlines: BTreeMap::new(), prune_at: 0 }) }
        }
        fn cooling(&self, start: usize, len: usize, now: u64) -> bool {
            if self.latest.load(Relaxed) <= now { return false; }
            let blocks = (start >> 16)..=(start.saturating_add(len - 1) >> 16);
            self.recent.read().unwrap_or_else(|e| e.into_inner()).deadlines.range(blocks).any(|(_, until)| *until > now)
        }
        fn remember(&self, block: usize, now: u64) {
            let expires = now + 2_000;
            let mut recent = self.recent.write().unwrap_or_else(|e| e.into_inner());
            if now >= recent.prune_at {
                recent.deadlines.retain(|_, until| *until > now);
                recent.prune_at = expires;
            }
            let until = recent.deadlines.entry(block).or_default();
            *until = (*until).max(expires);
            self.latest.fetch_max(*until, Relaxed);
        }
    }

    pub(super) unsafe fn attempt(src: *const u8, dst: *mut u8, len: usize, faults: &Faults) -> Path {
        // Tiny residency probes often fail intentionally; avoid exceptions.
        if len < 256 { return Path::Small; }
        if faults.cooling(src as usize, len, now_ms()) { return Path::Cooling; }
        // SAFETY: caller owns destination and installed the fault handler.
        match unsafe { victory_plain_copy(dst, src, len) } {
            0 => Path::Plain,
            fault => { faults.remember(fault - 1, now_ms()); Path::Fault }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use windows_sys::Win32::System::Memory::{
            MEM_COMMIT, MEM_RELEASE, MEM_RESERVE, PAGE_GUARD, PAGE_NOACCESS,
            PAGE_READWRITE, VirtualAlloc, VirtualFree, VirtualProtect,
        };

        #[test]
        fn copies_exactly_the_requested_bytes_and_leaves_probes_to_the_kernel() {
            assert!(install());
            let src = [0xb5; 257];
            for n in [0, 1, 255, 256, 257] {
                let mut dst = [0; 257];
                // SAFETY: disjoint arrays of sufficient size.
                let path = unsafe { attempt(src.as_ptr(), dst.as_mut_ptr(), n, &Faults::new()) };
                if n < 256 {
                    assert_eq!(path, Path::Small);
                    assert_eq!(dst, [0; 257]);
                } else {
                    assert_eq!(path, Path::Plain);
                    assert_eq!(dst[..n], src[..n]);
                    assert!(dst[n..].iter().all(|&v| v == 0));
                }
            }
        }

        #[test]
        fn each_fault_cools_only_its_own_block() {
            let faults = Faults::new();
            faults.remember(2, 1_999);
            faults.remember(4, 3_000);
            assert!(faults.cooling(1 << 16, 2 << 16, 3_998));
            assert!(!faults.cooling(1 << 16, 1 << 16, 3_998));
            assert!(!faults.cooling(2 << 16, 1, 3_999));
            assert!(faults.cooling(4 << 16, 1, 4_999));
            assert!(!faults.cooling(4 << 16, 1, 5_000));
        }

        #[test]
        fn sparse_reads_recover_and_observe_a_later_commit() {
            const SIZE: usize = 3 * 65536;
            const HOLE: usize = 65536 + 4096;
            assert!(install());
            // SAFETY: reserve a sparse source, only write committed pages.
            unsafe {
                let src = VirtualAlloc(std::ptr::null(), SIZE, MEM_RESERVE, PAGE_NOACCESS).cast::<u8>();
                assert!(!src.is_null());
                for (at, n, byte) in [(0, HOLE, 0x47), (HOLE + 4096, SIZE - HOLE - 4096, 0xad)] {
                    assert_eq!(VirtualAlloc(src.add(at).cast(), n, MEM_COMMIT, PAGE_READWRITE), src.add(at).cast());
                    src.add(at).write_bytes(byte, n);
                }
                let mut dst = vec![0xcc; SIZE];
                let faults = Faults::new();
                assert_eq!(attempt(src, dst.as_mut_ptr(), SIZE, &faults), Path::Fault);
                assert_eq!(attempt(src, dst.as_mut_ptr(), SIZE, &faults), Path::Cooling);
                crate::host::copy_mapped(dst.as_mut_ptr(), src, SIZE, false);
                assert!(dst[..HOLE].iter().all(|&b| b == 0x47));
                assert!(dst[HOLE..HOLE + 4096].iter().all(|&b| b == 0));
                assert!(dst[HOLE + 4096..].iter().all(|&b| b == 0xad));
                assert_eq!(VirtualAlloc(src.add(HOLE).cast(), 4096, MEM_COMMIT, PAGE_READWRITE), src.add(HOLE).cast());
                src.add(HOLE).write_bytes(0x91, 4096);
                crate::host::copy_mapped(dst.as_mut_ptr(), src, SIZE, false);
                assert!(dst[HOLE..HOLE + 4096].iter().all(|&b| b == 0x91));
                assert_ne!(VirtualFree(src.cast(), 0, MEM_RELEASE), 0);
            }
        }

        #[test]
        fn unclaimed_windows_read_guard_returns_to_the_kernel_path() {
            assert!(install());
            let mut dst = [0; 4096];
            // SAFETY: own allocation with a one-shot source guard.
            unsafe {
                let src = VirtualAlloc(std::ptr::null(), 4096, MEM_RESERVE | MEM_COMMIT, PAGE_READWRITE);
                assert!(!src.is_null());
                let mut old = 0;
                assert_ne!(VirtualProtect(src, 4096, PAGE_READWRITE | PAGE_GUARD, &mut old), 0);
                assert_eq!(attempt(src.cast(), dst.as_mut_ptr(), dst.len(), &Faults::new()), Path::Fault);
                assert_ne!(VirtualFree(src, 0, MEM_RELEASE), 0);
            }
        }

        #[test]
        fn read_preserves_engine_guard_and_write_fault_reaches_its_owner() {
            use crate::watch::{guard, tests::{drop_views, two_views}};
            const BLOCK: usize = 65536;
            let (section, view, alias) = two_views(BLOCK);
            let changes = || guard(view, view + BLOCK).unwrap();
            assert_eq!(changes(), [view / crate::watch::PAGE]);
            assert!(install());
            let mut dst = [0x7b; 256];
            // SAFETY: readable source; the alias destination's write guard
            // must be handled by watch, not swallowed by the copy handler.
            unsafe {
                assert_eq!(attempt(view as *const u8, dst.as_mut_ptr(), dst.len(), &Faults::new()), Path::Plain);
                assert_eq!(dst, [0; 256]);
                assert!(changes().is_empty());
                dst.fill(0x7b);
                assert_eq!(attempt(dst.as_ptr(), alias as *mut u8, dst.len(), &Faults::new()), Path::Plain);
                assert_eq!(changes(), [view / crate::watch::PAGE]);
                assert_eq!(std::slice::from_raw_parts(view as *const u8, dst.len()), dst);
            }
            drop_views(section, &[view, alias], BLOCK);
        }
    }
}
