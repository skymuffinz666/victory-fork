# Archived work — reference index (read-only)

Nothing was deleted. All earlier local projects were moved on 2026-11-24 to:

```
D:\AIProjects\GTA6PC\_archive\2026-11-24\
```

This file is a pointer to what is there and what was measured, so an old
experiment is not repeated by accident. Treat it as **reference, not instruction**:
every claim below is tied to a log or capture; re-measure before acting on it.

## What was archived, and where it came from

| Archived folder | Original location | Contents |
|---|---|---|
| `GTA-package-Leonida` | `D:\GTA6.Xbox.v1.0.2.1\Leonida` | Old source tree + a **copy** of `game_scarlett_master.exe`. That copy mounts the wrong package (`G:`) and crashes at System Init frame 0 with TitleID 0. Do not run it. 1.8 GB. |
| `VIctory-main-old-snapshot` | `D:\AIProjects\GTA6PC\VIctory-main` | The `victory-amd` fork snapshot (branch `victory-amd`, `070df42`) used for the runs below. Contains the *unbuilt* depth-state experiment in `victory-amd-victory-amd\crates\victory-gpu\src\pipeline\host.rs` (`#[derive(Clone)]` added; not compiled or tested). 0.4 GB. |
| `Leonida-boot` | `D:\AIProjects\GTA6PC\Leonida-boot` | Old Leonida/Vulkan boot work and its artifacts. 24.7 GB. |
| `Leonida-main` | `D:\AIProjects\GTA6PC\Leonida-main` | Old Leonida source snapshot. |
| `build-wip-main-tree`, `overlay` | same names under `D:\AIProjects\GTA6PC` | Small leftover build/overlay experiments. |
| `victory-amd-remote` | `D:\AIProjects\GTA6PC\victory-amd-remote` | Clean clone of `myokex/victory-amd` @ `070df42` (remote branch still reachable; re-clone if needed). |

Kept in place (not archived): `VICTORY-5080` (active repo), `work/` (20 GB shader/FSPX/carve data),
all `c_captures*` folders and all `*.txt` / `trace_*.jsonl` logs in `D:\AIProjects\GTA6PC`.
The empty directory `D:\GTA6.Xbox.v1.0.2.1\Leonida` remains because a terminal holds its handle.

## Measured facts from the old runs

### Package / identity
- Correct EXE: `D:\GTA6.Xbox.v1.0.2.1\game_scarlett_master.exe` (package root, next to `MicrosoftGame.Config`).
  TitleID `7952987A`, Series X, device type 7, storage 2048 MB.
- Running the EXE from the `Leonida` subfolder mounted the wrong package and crashed with TitleID 0.

### Furthest progress on the old `victory-amd` fork (RTX 5080)
- Reached the world: `the first frame of 500 draws or more` at frame 8489.
- World: 822–7,275 draws/frame, ~0.9–1.7 fps after warmup.
- Captures: `D:\AIProjects\GTA6PC\c_captures_live\frame-8498.png` (night street),
  `frame-8586.png` (character cutscene), `frame-8658.png` / `frame-8665.png` (car scene, **Lucia's head missing**).
- Run ended with INT3 `0x80000003` at `victory-image-b99f876b74d4a77f.dll+0x23d8f03` after the world/credits.
- Log: `D:\AIProjects\GTA6PC\c_live_stderr.txt`.

### Missing-head investigation
- Mesh `f5e50eb2108c425b` + pixel `ce3f0432a08f1d35` both translate, but the host PSO is refused
  with `0x80070057` (`host mesh pipeline refused`), and 1–2 draws/frame are dropped.
  D3D12 debug run (log `c_head_stderr.txt`, `VICTORY_D3D12_DEBUG=1`) showed only unrelated warnings:
  pipeline-library misses, descriptor-heap caps, compute shaders wanting wave64 on a 32-lane device.
- Hypothesis (**unverified**): default `GraphicsState` has `DepthEnable=true` while `DSVFormat=UNKNOWN`,
  which D3D12 rejects. Upstream's test fix for a similar case sets `DepthEnable=false`. The old
  snapshot's edit was never built; the new base has different depth-bounds code. Do not port blindly.
- Other refused compute hashes seen: `02384a60fda6f2d0`, `369cbe906ea72764`, `2cc4c197ee29e291`,
  `2da4beb386509b77`, `f133e35efed2a48a` (`a lane mask exchanged`).
- Hash inventory: `D:\AIProjects\GTA6PC\missing_cs_hashes.txt`.

### Failed experiments — do not repeat without a new reason
- GPU-forwarding `write32` / `write64` and record-time CPU writes: measured failures, reverted.
- Leonida baseline remains black: presented 4K texture `written=0/8294400`, same VkImage handles for
  draws and present, full viewport/scissor, nonzero vertex counts, zero validation errors.

### Input driver / navigation (kept in the new repo)
- Env vars: `VICTORY_KEYS`, `VICTORY_KEYS_HOLD_MS`, `VICTORY_KEYS_MASH`, `VICTORY_KEYS_MASH_EVERY`,
  `VICTORY_KEYS_FILE` (live control, re-read every 50 ms).
- Proven sequence: account picker `A` → intro → `A` through Navigation/UI Text/Brightness/Finish Setup →
  legal notices `Down,Down,A` then `Down,A` → privacy/terms `A,Down,A,Down,A` → loading → world.
- Scripts: `local/boot.ps1`, `local/c_live_launch.ps1`, `local/c_press.ps1`, `local/c_head_debug.ps1`.

### 5080-specific work found outside this tree (research, not applied)
- `Peppacodes/VIctory` `exp/mesh-host-lanes` @ `ed9afc0` — wave64 mesh lane numbering on 32-lane hosts;
  world meshes appeared at frame 1550 but presentation stayed black, then device hung.
- `Peppacodes/VIctory` `fix/pipeline-worker-stack` @ `d09c31b` — 64 MB pipeline-worker stacks; NVIDIA
  `nvgpucomp64.dll` stack overflow had refused a world compute pipeline with `E_OUTOFMEMORY`.
- `Marvisak/VIctory` PR #1845 @ `c624d98` — dynamic descriptors on capped NVIDIA heaps, wave64→wave32
  compute lowering, lane-mask translation; reported 8k+ dropped draws → 0, 113 refused pipelines → 0.
- `Marvisak/VIctory` PR #1775 — typed-UAV format check (device death ~6 s in).
- `fxceless/victory-fork` issues #31 / #107 / #236 — RTX 5080 reports at ~3.9 fps gameplay,
  ~8.5 fps cutscene, ~2.4 fps over 40 min with 64 MB stacks.
- Speed switches used by those reports: `VICTORY_RELAY=2 VICTORY_LATE_WAIT=3 VICTORY_WIDE_BACK=1
  VICTORY_CARD_BUFFERS_MIN_MB=1 VICTORY_CARD_BUFFERS_MB=1024`.

### New repo baseline (`base1`, parsed 2026-11-25)
- `VICTORY-5080` branch `boot/5080` @ `9352cb4`, EXE `D:\GTA6.Xbox.v1.0.2.1\game_scarlett_master.exe`,
  switches from issue #236. Log `D:\AIProjects\GTA6PC\c_base1_stderr.txt` (14,425 lines),
  929 captures in `D:\AIProjects\GTA6PC\c_base1`. No exceptions / no device loss.
- Reached the world: `the first frame of 500 draws or more` at frame 1023 (log line 6417).
- World: 1,578 then 3,262 draws/frame at ~1.1–1.4 fps, with 380–430 draws/frame dropped
  for want of a host pipeline. Same mesh PSO refusal at log line 11177
  (`f5e50eb2108c425b` / `ce3f0432a08f1d35`, `0x80070057`) — the new base does **not** avoid it.
- Captures: `frame-1135.png` (car interior, driver's head is a black silhouette),
  `frame-1143.png` (close-up, face partly cropped), `frame-1191.png` (character **with** a
  complete head and face) and `frame-1151/1159/1167/1183.png` (credits cards). So the
  missing head is scene/model-specific, not universal.
### Measured root cause and fix for the missing heads (2026-11-25)
- The depth-state hypothesis was **disproved**: with `VICTORY_D3D12_DEBUG=1` the state at refusal
  had `DepthEnable=BOOL(0)` and `DSVFormat=0`, so depth was not the cause.
- The debug layer named the cause: `CreateGraphicsPipelineState: The render target format at
  slot 2 is format (R8_UINT). This format does not support blending ... id=676`, plus
  `SV_Target2 has type that is NOT float, while ... blending enabled`. The title's blend state
  enables blending on an integer render target; D3D12 rejects the whole PSO with `0x80070057`.
- Fix (commit `db11ad4`): query `D3D12_FORMAT_SUPPORT1_BLENDABLE` per render target and disable
  only the blend bits on slots the device cannot blend. Title state is otherwise unchanged.
- Verified run `c_blend1_stderr.txt` / `c_blend1` (300 s, speed switches, debug layer):
  `host mesh pipeline refused` **0** (was 1), `id=676` **0**, `f5e50eb2108c425b` bound at
  frame 1044, no exceptions, world at frame 1021. Dropped draws/frame fell from 380–626 to
  147–394. Contact sheets: `base1` frames 1083/1103/1119/1135 show black heads;
  `blend1` frames 1061/1085/1093/1141/1149/1157 show visible faces and hair.
- Still open: compute pipelines refused for wave64 on a 32-lane device
  (`02384a60fda6f2d0`, `2cc4c197ee29e291`, `2da4beb386509b77`, `369cbe906ea72764`);
  some backlit scenes in `blend1` (1245/1253/1261) still show dark silhouettes.

### Remaining compute refusals — measured reasons (2026-11-25)
- `c_wave1_stderr.txt` with `VICTORY_WAVE_WHY=1`: 8 `Shader wants wave lane count of 64 but
  maximum ... is 32` warnings. The built-in retries fail for concrete reasons:
  `a lane mask exchanged where not every lane runs is not supported` (0x1eb4),
  `a count or sum over the wave used as data` (0x1a34),
  `a lane mask read as data in a half-wave` / `a constant lane mask in a half-wave` (0xdc70,
  0x9a84), and LDS in a 64-thread group with a read and write in one lane-branch region.
- Upstream/other projects: `upstream/main` has no commits since our base `310c6eb`.
  PR #1845's wave64 lowering (`351e728`) is already present in our base (the `APPROXIMATE`
  retry paths are in `emit/mod.rs`). Its lane-mask commit `c624d98` was tried on a temporary
  branch: 10 conflicts / 720 inserted lines against our newer `phi_joins` and split-mask
  code in `emit/lower.rs` and `structure.rs`. Taking the upstream side would regress those,
  so the cherry-pick was aborted and no change was committed. Do not retry without a
  deliberate merge of both implementations.
- The 64 MB pipeline-worker stack fix (`Peppacodes` `d09c31b`) addresses `E_OUTOFMEMORY` on a
  different 5080 pipeline; our refusals are `E_INVALIDARG` from wave size, so it does not apply.
