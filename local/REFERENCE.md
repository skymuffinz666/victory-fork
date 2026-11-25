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
- Default translation fix for data-only descriptors: `descriptor_data()` now
  calls `unused_descriptor_kind` whenever a load has no image/buffer use or
  twin. For an 8-dword load it requires a dimension field (dwords 1-4) to be
  read and rebuilds width/height/levels from `getDimensions` on the view the
  load's own index selects; address and format dwords remain `Val::Missing`
  and fail at the consumer rather than becoming invented numbers. For a
  4-dword load it requires dword 2 (record count). Offline, with no switches,
  all seven saved refused containers (6 pixel + `ceb5eb68cd99cb07`) now
  translate, validate with `dxil.dll`, and create host pipelines. None was
  executed: pipeline creation is not proof of console-equivalent pixels.
  Full shader suite: 211 passed, 4 pre-existing one-ULP BVH failures (also
  present at `fc1e67c`, before this change), 3 ignored.
- In-game verification `translated1` (900 s runner, all experimental switches
  off): translation failures fell from 10 in `prologue1` to **1**
  (`926c46bab3b86bd6`, "a lane mask holding lanes elsewhere read as data",
  `0x13c8`); the seven data-only pixel/compute shaders no longer fail. Seven
  compute pipelines are still refused on wave size. No unhandled exception,
  device removal, or `id=538` copy error. Late frames (~3.4–4.5 fps) show the
  characters with clean faces and clothing and no block artifacts. The last
  failing shader has no saved container, so it cannot be reproduced offline
  yet; the run was stopped by the runner (`exited=True`), not a natural exit.
- Hy4's frame-timing review found that the logged CPU/GPU counters overlap
  and must not be summed. The menu reaches 30 fps; world scenes reached
  roughly 3.5–5.7 fps in the saved logs, with texture upload and card-copy
  passes as the large variable costs. One measured, contained candidate was
  the hundreds of separate GPU rehash passes per frame; touching 64 KiB
  pieces are now grouped into one pass per contiguous run by default. This
  preserves the exact piece set and cannot lose a detected shader write,
  but the change has only passed a build check; no FPS claim follows.
- Default translation fix: the structure pass already recorded non-EXEC
  derived lane masks in its `other` table, but the lowering pass only read
  `masks`. `Structured::select_mask` now reads both in the two places that
  already require a mask (`v_cndmask_b32` / carry-in, and select masks in
  general); mask classification is unchanged. All 8 wave64 tests pass, and
  offline compute `f133e35efed2a48a`, previously refused with "a lane mask
  exchanged where not every lane runs", now validates and creates a host
  pipeline (`VICTORY_HOST_MAX_WAVE=32`, `--jobs 1`). The shader was not
  executed; this is not proof of console-equivalent results or a visual fix.
  `926c46bab3b86bd6` has no saved container and was not reproduced.
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
- Diner scene in `translated1` (`frame-1411.png`, all experimental overrides
  off): the standing character's face is black with multicolour noise specks,
  its shirt shows fine speckled noise, and blue/white blocks appear along the
  bottom edge; the room is much darker than the console's. The seated
  character's head in the same frame renders with plausible skin and hair. The
  same log reports 8441 textures passed over unmapped and 356 draws/frame
  dropped without a host pipeline, while `CS_Visibility`,
  `CS_Tiling_Generate_Opaque` and the tiled-transmission programs were refused.
  This localises the symptom to texture data that never reached the host and
  to dropped tiled-lighting work, but does not establish which mechanism causes
  the face's colours. Neighbouring frames show the scene itself:
  `frame-1475.png` (prison window) has the same block artifacts in its lower
  corners. The loading-screen art frames (`frame-480/832/1184/1264/1344.png`)
  render with clean geometry and full-colour textures, so the defect is not in
  the plain-textured draws that carry those screens.
- Frame-time accounting read against the code (Hy4 investigation, no code
  changed): the printed `88.9 preparing` is `SUBMISSION_US[0]` (`host.rs:342`),
  `copies 46.0 ms` is `PREPARING[0]/[1]` from `Step::Copy`/`Step::Fill` in
  `run_deferred` (`host.rs:5204-5224`), `textures 14.0 ms` is `Staging::refresh`
  (`texture.rs:898`), `85.0 on the GPU` is `SUBMISSION_US[1]`, and `141.0
  copying back` is the relay path (`relay_land` `host.rs:3472`,
  `relay_card_back` `host.rs:3349`). The readback is **not** per-resource
  `Map`/memcpy: one command list with one `CopyBufferRegion` per flagged piece
  into a persistently mapped buffer, then one synchronous fence wait per
  landing; the log attributes 104–133 ms/frame to those fetch waits (7–12
  landings/frame) and 33–47 ms to idle-card waiting, so the relay's cost is
  fence serialization, not staging. `heap buffers 120.0 ms` is `draw.rs:1399-1550`
  **including** `heap_textures` at `draw.rs:1532`: buffer refresh and texture
  staging share one timer and are not split yet. `descriptor::sync_heaps`
  (`descriptor.rs:172`, once per submission, one kernel read per dirty
  64 KiB page — `watch::PAGE` is `64 << 10` at `watch.rs:32`, not 4 KiB — and
  ~195 dirty pages and ~40k mirrored slots at a measured median frame) runs
  before the `preparing` timer starts at `host.rs:4020`, so its 20–33 ms/frame
  is unattributed in the frame line. A least-squares fit over the 975 frame
  lines from the four runs built after `467f7eb` (`c_quadhelpers1`,
  `c_translated1`, `c_shadersave2`, `c_wide4`) separates the costs:
  ~0.038 ms per page read, ~0.011 ms per thousand slots looked at and
  ~0.369 ms per thousand slots mirrored (R² 0.93), so **the per-slot mirror
  work is ~74% of that cost and the kernel page reads ~30%**: batching whole
  dirty runs into single reads is worth at most ~6.7 ms/frame and only if the
  ~6 dirty pages per submission are adjacent, whereas `mirror_record`
  (`descriptor/host.rs:1879`, locking `copied` then `made` per record) is the
  measured hot path. The fit is confounded for pre-`467f7eb` runs, which hashed
  each record with BLAKE3 and cost ~22 ms/frame more.
  Card limits (`shadow.rs:154-158`: 1 MiB/1024 MB) already equal `boot.ps1`'s
  `-speed` values, so those two settings are no-ops. Many paths Hy4 checked are
  already default-on (`VICTORY_HEAP_SYNC`, `VICTORY_VIEW_INLINE`,
  `VICTORY_ASK_ONCE`, `VICTORY_WIDE_BACK`, `VICTORY_CARD_BUFFERS`, relay stage
  2), and `VICTORY_WIDE_MERGE` does not apply while the relay is on.
- Upstream reconciliation (Hy4 review of fetched refs; nothing merged): all
  eleven upstream SHAs resolve in this repository. The upstream perf chain is
  `9376fd6` → `f85101b` → `a253706` → `0b0074e` → `3973179` → `465b94f` →
  `ca8829f` → `d510ee4` (merge base with our line is `a26859f`), and PR #1845 is
  a separate chain `4db3b01` → `3014591` → `e716e31` → `351e728` → `c624d98`.
  None of the eight perf commits re-enables `VICTORY_INDIRECT_WINDOW`,
  `VICTORY_WAVE_WIDE`, `VICTORY_PIXEL_QUAD_HELPERS` or `VICTORY_RAW_SRV_OWN_COPY`,
  and none mentions the two removed descriptor assumptions or an AMD constant-light
  substitute; upstream keeps `VICTORY_INDIRECT_WINDOW` opt-in and adds a driver
  watch gate to it. Conflicts that block a straight port: `9376fd6` and
  `a253706` rewrite `descriptor/host.rs::mirror_copied`/`mirror_copied_page` onto
  a multiply-fold record fingerprint (`record_print`) and default to *skipping*
  mirror work for slots read through tables lately, which loses our exact
  `[tag, index, generation]` tracking and first-seen-zero clearing; `a253706`
  reintroduces a `rehash_runs()` gate on the card rehash while our `host.rs:3183`
  is deliberately always joined. PR #1845's `c624d98` additionally replaces the
  fork's `translate_for_wave`-free `shader_cache.rs:47-82` with
  `translate_for_wave(c, max_wave)`, which on a 32-wave device makes the
  wave-wide lowering unconditional, i.e. it defeats our `emit/mod.rs:428` gate.
  Therefore the only upstream items safe without re-deriving our semantics are
  `d510ee4` (README), `ca8829f` (fps overlay plus off-by-default
  `VICTORY_COPY_CHANGED`), `465b94f` (off-by-default `VICTORY_INDIRECT_GATHER`),
  then `0b0074e` and `3973179` (thread affinity, meters, off-by-default
  `VICTORY_JOIN_PARTS`/`VICTORY_BACK_OWN_VIEW`) once the chain is reconciled.
  `f85101b` needs its default-on `VICTORY_FILL_WAIT` reviewed before adoption.
- Last translation refusal `926c46bab3b86bd6` (`CS_TransmissionVolumetricsSubmerged`)
  root-caused offline (`VICTORY_HOST_MAX_WAVE=32`, one job, with a temporary
  `VICTORY_MASK_WHY` print added, read and reverted): the mask whose dword is
  read as data is **loop-carried**, and its Bool support is a single
  `Atom::Phi` whose incoming edges are a `Lane` atom (a VALU compare bit, on
  the back edge) and an `Opaque` atom. The mask is therefore genuinely
  lane-dependent: its dword has to be the ballot of the lanes, and no uniform
  shortcut applies. `Structured.active` never takes such a phi (only EXEC phis
  at `structure.rs:677-680`), while the `s_cselect_b64 s[2:3], exec, 0` *inside*
  the loop does reach `active` at `structure.rs:1213-1220`: the failing value is
  the phi at the second entry of the rotated loop (`s_branch label_13C8` at
  `0x13a8`, back edge `s_cbranch_scc0 label_13B0` at `0x42a0`), and `fill_phis`
  (phi/carry-in lowering) is what reports the refusal at the block offset
  `0x13c8`. This program also has no cross-half exchange (`exchange=None`): the
  wave64 lowering at `lower.rs:1334` rejected splitting because one per-lane
  branch region holds **both** an LDS read and an LDS write, so `lds_meetings`
  returned `None` at `lower.rs:885-887` (a group barrier cannot be placed inside
  a region one half of the console wave may skip). Completing translation
  requires replicated LDS with a sound sync point for such regions, which the
  fork does not have; the uniform-mask shortcut written on the way was reverted
  because those edges are `Lane`/`Opaque` and it changed no saved container.
- The seven compute pipeline refusals were re-measured offline (`VICTORY_HOST_MAX_WAVE=32`,
  `VICTORY_WAVE_WHY=1`): all four single-console-wave-group programs are refused
  first by `lds_meetings()` returning `None` at `lower.rs:885-887`, because one
  per-lane branch region holds **both** an LDS read and an LDS write; their
  console ISA contains **no `s_barrier` at all**, so the title relies on the
  console's 64-lane wave lockstep for that ordering. `CS_Visibility`
  (`369cbe906ea72764`) is explicit: a lane-0-only region `0x58`–`0x80` writes
  LDS at `0x68`/`0x70`/`0x78`, and a second region `0x494`–`0x4cc` has
  `ds_read2_b32` at `0x4a0` with `ds_write*` at `0x4a8`, `0x4c4` and `0x4cc`;
  `CS_Tiling_Generate_Opaque` (`2cc4c197ee29e291`) likewise writes at `0x198`
  and `0x488` and reads at `0x49c`/`0x4dc`/`0x758` with dozens of `v_cmpx_*`
  narrowing EXEC. Additionally `ds_add_rtn_u32`/`ds_add_u32` at
  `0x390`/`0x3a8`/`0x434`/`0x43c` in the visibility program are LDS atomics that
  the classification at `lower.rs:878-882` calls writes, so the code falls back
  on the "two writes in one region do not alias" shortcut documented at
  `lower.rs:838` — which an LDS atomic invalidates. This is why the digest-gated
  experiment that forced the exchange lowering for those four produced
  pipelines but rendered the diner wrong: the two host waves race on the same
  LDS addresses with no barrier. Completing the refusals needs an ordering
  primitive for intra-region LDS read-after-write across two host waves
  (replicated LDS with a sound, uniform sync point), not a switch.
- Phase instrumentation (`phases1` run, 126 frame lines, removed again after
  the measurement because it cost frame time): at the median frame
  `sync_heaps` took 8.78 ms (p90 46.51 ms), of which 0.62 ms was kernel page
  reads, 0.00 ms the timed record lookups and 1.13 ms `mirror_record`;
  **7.36 ms was in `mirror_copied` calls that returned early** (the record's
  `[tag, index, generation]` was unchanged), about 70 ns each over ~105k such
  looks a frame. The earlier cross-run fit attributed 74% to `mirror_record`;
  that counted changed records only, whose share varies greatly (median 11.5k,
  p90 48.6k). The measured hot path is therefore the per-looked-record `copied`
  mutex, not the page reads or view creation.
- Texture-keeping fix verification (`keepface1`, 700 s, same settings and no
  overrides): the diner close-ups `frame-1247.png` and `frame-1287.png` and
  the prison frames `frame-1351/1399/1447/1495.png` show clean, correctly
  coloured faces and hair, with no black head or speckled face of the kind in
  `translated1/frame-1411.png`. The same frames of `phases1` (before this fix)
  also show clean faces, so this is **not** a controlled A/B proof that the
  fix removed the black-face symptom; it establishes only that keeping the
  last host copy when the title unmaps the page does not visibly regress these
  scenes. No device removal, exception or `id=538` error was logged.
- Atomic descriptor-record cache (`cb8be21`, uncommitted predecessor of this
  note measured in `atomic1`): replacing the per-record `copied` mutex with two
  exact atomic words per slot changed the median `sync_heaps` line from 23.31
  ms (`keepface1`, 202 samples) to 22.26 ms (`atomic1`, 397 samples) with
  slightly MORE work per frame (209 vs 192 pages, 37.9k vs 35.1k mirrored),
  and p90 from 35.18 to 27.45 ms. That is a modest improvement, not the ~7 ms
  the phase instrumentation suggested; scenes and capture schedules differ, so
  no fps claim follows. No device removal, exception or `id=538` error.
- LDS read-modify-write refinement: the first version of `lds_atomic` counted
  every `ds_add`/`ds_min`/`ds_or` etc. as a read, which made three previously
  accepted programs refuse (`89d08e561897d4e5` and `55679a0547649d0b`, tree
  batch cull; `1f1fbd86759b3da2`, projection decal classification). Their
  regions use *commutative RMW without a returned old value*, where two host
  waves leave the same dword regardless of order, so those now stay writes;
  `_rtn` forms and non-commutative RMWs (`ds_sub`, `ds_cmpst`, `ds_wrxchg`,
  `ds_swap`, `ds_reset`, `ds_bfi`) count as reads. Offline (`translate --list
  --validate --pipelines`, host wave 32) all three validate and create host
  pipelines again, while the original seven wave64 programs still validate
  but create 0 pipelines.
- Borrowed from DexNom's public fork (Radeon-based, no transferable reflection
  fix; only architecture-neutral items taken, no AMD/RT switches): (1) cube
  gradient LOD now uses DXIL `log2` directly and applies the console's minimum
  level for `_cl` samples where `sampleLevel` has no clamp operand, instead of
  `ln * 1/ln2` (`lower.rs:9940`); (2) `TextureQueue::due` alternates retries
  with first looks and rotates the first choice across frames, so a backlog of
  unmapped/partial textures cannot spend the whole staging pass before the
  textures of the shot on screen (`descriptor/host.rs:570`); (3) `mapped_bytes`
  no longer returns zero when both ends are uncommitted but interior mips are
  resident, and the VirtualQuery walk skips `PAGE_NOACCESS`/`PAGE_GUARD` runs
  (`texture.rs:1296`); (4) pipelines with no colour target and depth writes on
  are queued ahead of other background work, so shadow/depth passes are ready
  for later draws (`pipeline/make.rs:221`). `cargo check -p victory-gpu` and
  139 victory-gpu tests pass; the shader suite remains 211 pass / 4 pre-existing
  one-ULP BVH failures. In-game verification of these four is pending (the
  `mirror2` dump run used the previous binary).
- Save/Continue returning to the prologue after the early robbery is **expected
  game behaviour**: the title cannot save until a later story point. It is not
  a translation-layer save bug and needs no investigation.
- Verification of the four borrowed fixes (`borrow1`, 1000 s with target dumps):
  final frames reached only 2191, so the late dumps were still in the prison
  sequence and no robbery-scene target was captured. One translation failure
  (`926c46bab3b86bd6`) and eight compute refusals (the seven plus
  `caf96a3229145fa6` = `CS_CollisionRaytraced`, reached in this scene) remained;
  no device removal, exception or `id=538` error. Median `sync_heaps` was
  26.66 ms for 236 pages/48k mirrored slots, versus 22.26 ms for 209 pages/38k
  in `atomic1`; the scenes and capture schedules differ, so this does not show
  a regression or a gain. Median fps 3.9 with 7006 draws in the last logged
  interval (dumps distort timing). The earlier `mirror2` run reached frame
  4732 and the first robbery: `frame-4300/4500/4700.png` show the teller scene
  heavily overexposed (white hair, blown-out ATM/counter lights) with flat
  cutout reflections in the glass. `CS_CollisionRaytraced` had not been seen in
  earlier, shorter runs.
- Translation-side GPU work (`cec8e0a` and the following uncommitted edits):
  (1) mesh quad swizzles and within-quad DPP8 use single
  `quadReadLaneAt.i32`/`quadOp.i32` instructions instead of four reads plus
  three selects (the pixel path is unchanged unless `VICTORY_PIXEL_QUAD_HELPERS=1`);
  (2) integer-format render targets drop `BlendEnable` before PSO creation;
  (3) `first_lane` skips the `waveActiveBallot` and guarded lane read when the
  guard is constant true (453 such ballots in the saved cache); (4)
  translator-added barriers in an all-lane block are coalesced while no LDS
  write or atomic has intervened, with the flag cleared at every block start
  and every store/atomic, while region-boundary meetings stay unconditional.
  Sampling the saved cache: the top barrier program `0a5d4f9047b3dce4` emits
  **50** barriers instead of 128; `b8f3f7979d6bbc50` 418 instead of 419;
  `eae3498e6553afa8` unchanged at 429. All three still validate with
  `dxil.dll` and create pipelines. Unit suites unchanged (211 shader pass /
  4 pre-existing BVH one-ULP failures, 139 GPU tests pass). The tally behind
  these choices: 4,165 barriers in 208 programs, 1,476 pairs with only LDS
  reads between and 205 back-to-back; 2,841,213 guarded `select i1` and
  3,459 `mul_hi` products remain for later work.
- In-game verification of the translation-side changes (`optcheck1`, 450 s,
  no dumps, no overrides): one translation failure and the same eight compute
  refusals, no device removal, unhandled exception or `id=538` error. Median
  fps 4.4 over 78 intervals; the last logged frame was 5.3 fps with 67.4 ms
  on the GPU and 4772 draws, against 5.3/88.1 ms/7006 draws in `borrow1` —
  different scenes and durations, so **no fps gain is claimed**. Diner
  close-ups (`frame-1254.png`) and the prison window (`frame-1502/1902.png`)
  have clean faces and hair with no new artefact; the blue/white blocks along
  the bottom edge remain unchanged.
- Full offline validation of all saved containers on the final build
  (`translate --list --jobs 8 --validate --pipelines`, 5726 files, 385
  duplicates): **2867/2867 programs translated**; VS/PS 1267 emitted, 1258
  validated, 1267 pipelines; compute 1068 emitted, 1068 validated, 1060
  pipelines; mesh 532 emitted, 527 validated, 532 pipelines. The 14
  validation failures are all signature classes (5 PS input-signature, 5 VS
  output-signature, 2 mesh output-size, 2 mesh interpolation mismatch), and
  every one of those programs still gets a pipeline on the NVIDIA driver; the
  only programs without pipelines are the 8 wave64 computes already known.
  Two candidate fixes were tried and reverted because they did not earn their
  place: DexNom's VS PSV0 interpolation pass left all four sampled VS
  output-signature failures unchanged, and the proposed `mul_hi` rewrite has
  no DXIL opcode to target (Microsoft's `DxilConstants.h` defines `IMul=41`
  and `UMul=42` as the full 32×32→64 multiply, with no UMulHi/SMulHi binary
  op), so the existing `zext`/`mul i64`/shift form is already correct.
