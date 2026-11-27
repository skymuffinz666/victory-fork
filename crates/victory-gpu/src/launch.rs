//! Mesh records read by an amplification stage instead of a separate
//! compute clamp between global UAV barriers. Adapted from Kelvaroq 0041,
//! e062a82, copyright 2026 Stellarr Studio SAS and contributors.
//! SPDX-License-Identifier: GPL-3.0-or-later
//!
//! Counts, bounds and total list budget match the existing clamp. When the
//! budget is exhausted, concurrent launches may choose a different subset
//! of records to empty; the total work remains bounded. Multi-record and
//! count-buffer calls retain the original pipeline and clamp.
//!
//! On by default after the 2026-11-27 A/B (5.13 -> 6.62 fps on the RTX
//! 5080, GPU 59.8 -> 32.6 ms); `VICTORY_MESH_LAUNCH=0` restores the
//! per-record clamp for a comparison.

use windows::Win32::Graphics::Direct3D12::*;
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};
use crate::host::HostList;

const CODE: &[u8] = include_bytes!("launch.dxil");
static LAUNCHED: [AtomicU64; 2] = [const { AtomicU64::new(0) }; 2];

pub(crate) fn on() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| std::env::var_os("VICTORY_MESH_LAUNCH").is_none_or(|v| v != "0"))
}

pub(crate) fn code() -> D3D12_SHADER_BYTECODE {
    D3D12_SHADER_BYTECODE { pShaderBytecode: CODE.as_ptr().cast(), BytecodeLength: CODE.len() }
}
pub(crate) fn bytes() -> &'static [u8] { CODE }

pub(crate) fn params() -> [D3D12_ROOT_PARAMETER1; 3] {
    let descriptor = |kind| D3D12_ROOT_PARAMETER1 {
        ParameterType: kind,
        Anonymous: D3D12_ROOT_PARAMETER1_0 {
            Descriptor: D3D12_ROOT_DESCRIPTOR1 {
                ShaderRegister: 0, RegisterSpace: 9,
                Flags: D3D12_ROOT_DESCRIPTOR_FLAG_DATA_VOLATILE,
            },
        },
        ShaderVisibility: D3D12_SHADER_VISIBILITY_AMPLIFICATION,
    };
    [
        D3D12_ROOT_PARAMETER1 {
            ParameterType: D3D12_ROOT_PARAMETER_TYPE_32BIT_CONSTANTS,
            Anonymous: D3D12_ROOT_PARAMETER1_0 {
                Constants: D3D12_ROOT_CONSTANTS { ShaderRegister: 0, RegisterSpace: 9, Num32BitValues: 1 },
            },
            ShaderVisibility: D3D12_SHADER_VISIBILITY_AMPLIFICATION,
        },
        descriptor(D3D12_ROOT_PARAMETER_TYPE_SRV), descriptor(D3D12_ROOT_PARAMETER_TYPE_UAV),
    ]
}

#[derive(Clone, Copy)]
pub(crate) enum Launch { Record(u64), Direct(u32) }

pub(crate) fn record(list: &HostList, roots: [u32; 3], what: Launch) -> bool {
    use windows::core::Interface;
    let Ok(l6) = list.list.cast::<ID3D12GraphicsCommandList6>() else { return false; };
    let budget = list.budget();
    let (direct, address, budget) = match what {
        Launch::Record(address) => {
            let Some(budget) = budget else { return false; };
            (u32::MAX, address, budget)
        }
        Launch::Direct(x) => (x, budget.unwrap_or(0), budget.unwrap_or(0)),
    };
    // SAFETY: open list, bound mesh pipeline and root signature; source and
    // budget buffers are retained by the list until execution completes.
    unsafe {
        list.list.SetGraphicsRoot32BitConstant(roots[0], direct, 0);
        list.list.SetGraphicsRootShaderResourceView(roots[1], address);
        list.list.SetGraphicsRootUnorderedAccessView(roots[2], budget);
        l6.DispatchMesh(1, 1, 1);
    }
    LAUNCHED[usize::from(matches!(what, Launch::Direct(_)))].fetch_add(1, Relaxed);
    true
}

pub(crate) fn line(frames: f64) {
    let [records, direct] = LAUNCHED.each_ref().map(|n| n.swap(0, Relaxed));
    if records + direct > 0 {
        tracing::info!("mesh launches per frame: {:.1} indirect, {:.1} direct", records as f64 / frames, direct as f64 / frames);
    }
}
