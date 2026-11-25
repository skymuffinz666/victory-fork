# Archived work — reference index (read-only)

Nothing was deleted. All earlier local projects were moved on 2026-11-24 to:

```
D:\AIProjects\GTA6PC\_archive\2026-11-24\
```

This file is a pointer to what is there and what was measured, so an old
experiment is not repeated by accident. Treat it as **reference, not instruction**:
every claim below is tied to a log or capture; re-measure before acting on it.

Branches: `boot/5080` is the verified baseline (blend fix, reference index). Ongoing
work happens on `next/5080`, branched from it, so the known-good state is not disturbed.

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

### Clean verification without the debug layer (`final1`, 2026-11-25)
- `c_final1_stderr.txt` (12,281 lines), 651 captures. Same switches, no `VICTORY_D3D12_DEBUG`.
- World at frame 978; `host mesh pipeline refused` **0**, `id=676` **0**, no exceptions.
- `dropped .../frame without a host pipeline: refused` summaries fell from 29 entries per
  compute hash in `base1` to 12–13 in `final1`; the mesh PSO entry (8 in `base1`) is gone.
- fps in the late world was 0.8–2.0 with 3,965–5,000 draws/frame; the debug-layer run
  `blend1` was 1.0–3.6 with 1,020–3,496 draws. Scene mix differs between runs, so these
  are not a controlled speed comparison; the reliable result is the absence of the
  mesh PSO refusal and the visible character heads.
- Breakdown of the large per-frame drop count (report at log line 11968 of `final1`):
  `1491.0/frame` are `a stage of the ray-tracing structure builder, which is off`
  (intentional), `9.0/frame` are pixel `6b6db9f1a526daeb` (descriptor read only as data),
  and 1/frame each for four refused wave64 compute hashes. So there is no large pool of
  additional missing-pipeline reasons in that particular report. This does **not**
  establish that the disabled builder or refused shaders have no visual impact.

### Speed switch and descriptor-diagnostic probes (2026-11-25)
- `VICTORY_D3D12_BACKGROUND=0` (`c_bg0_stderr.txt`) logged
  `the host's background shader work is switched off ... ok=false`: this driver refused
  `SetBackgroundProcessingMode`. Its higher fps median (1.20 vs 0.80 on world frames) is
  not attributable to the switch; do not add it to the default speed set.
- `VICTORY_DESC_WHY=1` (`c_descwhy_stderr.txt`) measured the remaining pixel-shader
  translation failures: all are `s_load_dwordx8` loads at offsets `0x1c14`–`0x1e2c`,
  reading 8-dword descriptors from root params 288/289 with a dynamic index
  (`Value(8184)`–`Value(9225)`), `lane_copied=false`. The translator refuses them as
  `a descriptor read only as data` (6 failures in that run). Supporting them would need
  a host-side descriptor representation, not a flag; not attempted.

### Upstream PR #1845 — prefix reuse of fresh descriptor tables (merged)
- Cherry-picking `e716e31` as a whole conflicts with our newer null-slot view-heap
  capping (`probe_view_limit`) and hash-keyed `fresh_tables`. Only its self-contained
  part was ported: `draw.rs` now keeps `FreshTables`, matching copied `[u32; 3]`
  descriptor records by exact prefix, so stages/draws binding a shorter unchanged
  prefix share the same dynamic slots. The capped-heap split part was **not** taken;
  our heap handling is different and already avoids the 65k-slot fallback.
- `cargo test -p victory-gpu --lib fresh_tables` passes (unit test from the commit).
- Verified boot `c_merge1_stderr.txt` (300 s, speed switches): world at frame 1023,
  `host mesh pipeline refused` **0**, no exceptions, dynamic tables asked per report
  median ~5,193 (was ~7,590 in `final1`) with 0 refused. Scene mix differs, so this
  is indicative rather than a controlled speed result; no regression was measured.
- PR #1845's other two commits were **not** merged. `351e728` (wave64 lowering) is
  already in our base. `c624d98` (uniform/constant lane masks in selects and phis)
  was re-tried with `cherry-pick -n` to merge deliberately: it rewrites the select-mask
  path our base has since extended with `MaskData::Console` constant halves, wave64
  high-SGPR handling and back-edge `phi_joins`, so the conflicts are in core lowering
  logic, not tests. A union cannot be verified as correct without re-porting those
  semantics; taking either side alone would silently miscompile. No change landed.

### Intro visual baseline and texture-budget caveat
- `ceb5eb68cd99cb07` fails translation at `0x38` with "a descriptor's
  address or format read as data". Offline native disassembly shows the
  earlier `s_load_dwordx4` at `0x28` loads a buffer descriptor and the
  comparison at `0x34` reads its third dword (`s2`) for zero before the
  branch at `0x38`. Its resource metadata describes an SRV with stride 16.
  This identifies a record-count use, not proof that the title reads an
  address or format there. A correct implementation still needs the actual
  bound view and its record count: inferring a raw host view from the stride
  alone is unsafe because the host may bind a structured view instead.
- Offline, enabling `VICTORY_ASSUME_DATA_DESCRIPTOR_2D=1` also enabled the
  upstream raw-buffer assumption and made `ceb5eb68cd99cb07` validate and
  create a host pipeline; it did not execute or establish correct results.
  `VICTORY_ASSUME_DATA_BUFFER_RAW=1` now selects only the four-dword buffer
  assumption, separately from the 2D-texture assumption. It remains off by
  default until the bound view and actual count are verified.
- `quadhelpers1` ran with `VICTORY_PIXEL_QUAD_HELPERS=1` and was stopped at
  the user's request; `exited=True` in its runner output reflects that stop,
  not a natural exit. It reached the prison corridor (`frame-3106.png`), with
  seven compute pipeline refusals and ten shader translation failures; no
  unhandled exception or device removal was logged. Final diagnostic intervals
  were approximately 3.6–4.4 fps. The run used different scenes and player
  progress from the baseline, so neither a visual improvement from quad reads
  nor a frame-rate improvement has been established. The option remains off
  by default. The later raw-SRV dedicated-copy experiment was not in the binary.
- The `desccompare1` run was manually stopped in the prison scene. It used
  exact tag/index/generation comparison in the copied-descriptor sweep in
  place of a truncated BLAKE3 fingerprint, with no experimental pixel-quad
  option. Seven compute pipelines were refused and ten shaders failed
  translation; no unhandled exception or device removal was logged before
  stopping. Its partial data and differing scene do not establish a speedup
  or visual equivalence. Afterward, first-seen zero records were made to
  clear previously populated host slots, and direct mirrors were made to
  invalidate conflicting sweep records; these later edits only passed a
  build check and have not run in the game. Exact records use more memory,
  and invalidation takes the copied-record lock on direct mirrors.
- `prologue1`, built from `94a8a0e`, ran for the timed 900 seconds and
  reached the prison scene (`frame-3512.png`). Later captures remain at nearly
  the same camera position; playable control or a wait for input has not been
  established. The frame still has visible block artifacts. The last three
  reported intervals were approximately 4.2 fps; seven compute pipelines were
  refused, ten shaders failed translation, and no unhandled exception or
  device removal was logged. The timed runner stopped the process; it did not
  exit naturally. The descriptor snapshot edit made during the run was not in
  the running binary.
- `upstreamroom1` booted integration commit `7369f49` with normal scissors,
  experimental wave-wide lowering off, and the guessed 2D descriptor fallback
  off. `frame-1412.png` shows the room wall and both characters' torsos,
  unlike the earlier missing-room/torso captures. `frame-1372.png` still has
  substantial block artifacts. Seven compute pipelines were refused and
  eight shaders failed translation; no unhandled exception or device removal
  was logged. The game was stopped by the timed runner, not a natural exit.
  Later diagnostic intervals reported 1.3–5.1 fps; they are not a controlled
  performance comparison. Descriptor-sync logs report copied heap records
  being mirrored, but this multi-change integration does not establish which
  change caused the visual improvement. Full correctness and 30 fps remain
  unverified. The subsequent executable-hash cache-key edit was not in this
  run's binary.
- Seven refused compute shaders were independently tested offline with a
  32-lane host limit. All seven validate as DXIL but their default output
  requests wave64, which the RTX 5080 cannot create. With experimental
  wave-wide lowering, visibility (`369cbe906ea72764`), opaque tiling
  (`2cc4c197ee29e291`), tiled transmission (`5dc3b25986622949`) and its
  local-lights variant (`6f225b641c5b0708`) create pipelines offline; none
  was dispatched or shown correct. Defragmentation (`02384a60fda6f2d0`),
  alpha prefilter (`2da4beb386509b77`) and meshlet triangle collection
  (`6b82bc7d22016cb5`) have multiple console waves per group and still
  require wave64. On the hardware device two existing small wave-wide branch
  fixtures passed their output checks, including both skip forms and the SCC
  join case; a separate wave64 reduction fixture also matched its expected
  console-tree result. The branch fixture additionally covers zero active
  lanes and the 31/32/33-lane boundary on hardware. These fixtures do not
  establish safety of the title shaders.
- Full-translation caveat: the default wave64 retry allows explicitly
  approximate half-wave lowering (`emit/mod.rs`, `VICTORY_WAVE_LOCAL` unset).
  The `cachefix1` fresh-translation log contains 22 successful approximate
  retry messages; concurrent log ordering does not by itself associate all
  of them with shader hashes. A single-thread offline probe confirmed compute
  `2561af9577c47289` uses approximate wave32 lowering by default (DXIL
  validates and a pipeline is created), whereas `VICTORY_WAVE_LOCAL=idiom`
  refuses it at `0x377c` (wave reduction under divergent execution). Neither
  path was executed in that probe; no console-equivalence or causal relation
  to the visual defect is established. Strict refusal avoids that approximate
  fallback but also drops work; it is not full translation or a visual fix.
- Hy4's offline review reproduced all six `cachefix1` pixel translation
  failures on the saved containers with `VICTORY_DESC_WHY=1`. Their scalar
  descriptor loads are used as data but cannot be matched by the current
  `descriptor_data` view-kind lookup to an image instruction or equal-address
  load. Disassembly shows bit extraction consistent with dimension fields;
  the reported failure offset is a later consumer. The different dynamic
  indices and the view's dimension, binding, and base-mip semantics must be
  established before reconstructing those fields. No changes or claims of
  visual repair follow from translation or pipeline creation alone.
- In `intro_trace1` frame 1247 the missing torso and room also appear as
  black regions in the captured albedo-like intermediate target
  `frame-1247-13-0xfc739a6d40-2560x1440-f27.png`. This places the symptom
  at or before that target, but does not distinguish missing draws from
  clipping, depth rejection, or other causes. The same frame's diagnostic
  summary reports many main-pass draws with a small scissor rectangle.
  `VICTORY_SCISSOR_TEST=full` is an existing, off-by-default probe that
  widens title scissors globally. It changes intended rendering and can
  affect other passes, so it is not a fix and its result is not conclusive
  on its own. Compare the matching scene and intermediate targets.
- `scissorfull1` tested that global override: `frame-1276.png` shows a
  wall-like surface in the kitchen scene, but the torso is still missing and
  severe block and line artifacts are visible; `frame-1316.png` also shows
  missing torso and room. Seven compute pipelines were refused, with no
  logged unhandled exception or device removal. Globally widened scissors
  alter unrelated passes and this is not evidence of a correct wall or a
  scissor implementation defect. The override remains off by default.
- `nodump1` ran with intermediate target dumping disabled and produced 148
  final-frame images and no intermediate target images. Its `frame-1246.png`
  still shows the missing torso and room, like the earlier kitchen capture;
  seven compute pipelines were refused and no unhandled exception or device
  removal was logged. Later diagnostic intervals reported roughly 6.6–7.5 fps,
  but differing scenes, capture schedules and cache conditions prevent a
  controlled before/after renderer performance claim. Final-frame captures and
  other diagnostics remained enabled. `local/boot.ps1 -dumpTargets` opts back
  into intermediate dumps.
- `c_blend1/frame-1245.png` and `frame-1253.png` show missing portions of a
  character and the room; `frame-1261.png` also shows visible block artifacts.
  Visible heads and zero mesh-PSO refusals did not establish a correct intro.
- The `tex384` and `tex512` experiments used different scenes and timings from
  `merge1`; their texture upload rates and fps cannot establish a causal speed or
  visual-quality improvement. The proposed 384 MB default was not retained;
  the default remains 192 MB. A larger upload total does not demonstrate that
  the textures needed by the visible scene arrived correctly.
- The debug-layer `blend1` log reports D3D12 CopyResource destination-state
  errors (id 538) later in the run. Their relation to the missing geometry
  or textures has not been established.
- `copytrace1` with the D3D12 debug layer and resource-copy tracing reported
  119 id 538 errors for a single host resource; corresponding title copy
  records had that same host resource as source and destination. `copyfix1`
  preserves the source upload but treats a whole-resource self-copy as a
  no-op: 57 self-copies were skipped, with zero id 538 errors, no logged
  unhandled exception or device removal, and seven compute pipeline refusals.
  `frame-1372.png` still shows the missing room. This verifies the observed
  debug-layer error was avoided, not that overall rendering is correct.
- `intro_trace1` reproduced the defect in `frame-1247.png`: missing torso and
  room geometry. `frame-1247-ops.txt` recorded four refused compute dispatches,
  135 disabled BVH-builder dispatches, and no refused pixel shader in that
  frame. `VICTORY_WALLS_TRACE=1` reported no failed storage reads in the run.
  These observations do not identify which missing work caused the defect.
- Shader-cache correctness fix: the cache key now includes the host maximum
  wave width, `VICTORY_WAVE_LOCAL`, and other translation-affecting switches
  previously absent. Cache tests exercise distinct wave widths and local
  policies. This prevents inappropriate reuse across those configurations; it
  has not been shown to repair the intro or improve performance.
- Fresh-translation boot `cachefix1`: reached the intro without a logged
  unhandled exception or device removal, but `frame-1367.png` still shows the
  missing torso and room. The same seven compute pipelines were refused.
  Console containers are saved locally in `work/intro_shadercache_1` (do not
  commit game data). Names embedded in four refused containers identify them as
  `CS_DefragHeap_ComputeRelocations` (`02384a60fda6f2d0`), `CS_Visibility`
  (`369cbe906ea72764`), `CS_Tiling_Generate_Opaque` (`2cc4c197ee29e291`) and
  `CS_AlphTransmissionPrefilterMip` (`2da4beb386509b77`). Names alone do not
  prove their relation to the visible defects.
- Offline, with host wave limit 32 and `VICTORY_WAVE_WIDE=1`, the visibility
  and opaque-tiling shaders emitted wave32 DXIL that validated and produced
  host pipelines; the other two still failed pipeline creation. No shader was
  executed in this probe. Wave-wide lowering is unsafe to enable globally:
  earlier runs lost the device on different shaders with barriers in divergent
  control flow. Pipeline creation is not proof of correct or safe execution.
