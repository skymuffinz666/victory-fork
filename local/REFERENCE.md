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
- Feasibility verdict for the refused wave64 computes (Hy4 design review against
  XCL's wave64-on-wave32 doc and Marvisak #1132): the only missing primitive is
  a **sound intra-region LDS ordering point that both host halves reach**;
  thread remapping, exchange slots and predicated LDS writes already exist here
  (`lower.rs:3680-3808`, `structure.rs:473-480`, `lower.rs:4652-4673`). The
  earlier digest-gated widening produced pipelines but wrong pixels because it
  set `wave=32` *without* `split_lds`, so neither the per-access barriers
  (`lower.rs:4862-4875`) nor the region meetings (`lower.rs:2288-2305`) fired;
  the two halves raced on the same LDS dwords. Of the refused programs, only
  `369cbe906ea72764` (CS_Visibility) and `2cc4c197ee29e291`
  (CS_Tiling_Generate_Opaque) use constant LDS dwords with write→read in one
  region; `5dc…`/`6f…` additionally need readlane exchange and 4/7 widened
  branches, `caf96a3229145fa6` uses computed addresses and `ds_wrxchg_rtn`
  (read+write by definition), the 128/256/1024-thread programs need
  double-buffered exchange slots and arrival counters that this tree lacks,
  and `926c46bab3b86bd6` needs both the LDS region and the `Lane`/`Opaque`
  loop-carried mask phi solved. A sound landing would have to be a narrow,
  opt-in lowering with fixtures that prove both halves see the write (sweep
  1/31/32/33/64 writers), must keep `VICTORY_GROUP_SYNC=2` and the approximate
  local fallback off, and must be aborted on any device loss or slower
  submissions. No code was changed from this review.
- First narrow landing of that idea (`VICTORY_LDS_FLOW=1`, opt-in, digest
  allowlist containing only `369cbe906ea72764`): the gate enables `widen()`
  for this program and makes the approximate half-wave fallback strict, so a
  pipeline can only come from the widened branch form with `split_lds` group
  barriers. Offline on RTX 5080 host wave 32: `WAVE64-WIDE 1 branches taken
  wave-wide`, wave 32, 1/1 DXIL validated, 1/1 pipeline; 19 `dx.op.barrier`
  calls (mode 9) around the LDS stores in the emitted DXIL. All seven other
  refused computes still emit/validate but create 0 pipelines, and the unit
  suites are unchanged. The gate is in the translation cache key. An in-game
  run (`ldsflow1`, 1000 s) is running to check for device loss, slow
  submissions and any new image artefact; this is NOT yet a default fix.
- `ldsflow1` result: no device removal, exception or `id=538`; refusals fell
  from 8 to **7** (CS_Visibility now creates a pipeline). Median fps 4.1,
  last logged frame 4.1 fps with 118.3 ms on the GPU. The blue/white pixel
  boxes along the bottom edge are still present (`frame-1248/1504/1856.png`)
  and match the same artefacts in earlier runs without the gate, so they are
  not caused by the visibility program. Tier 2 adds
  `2cc4c197ee29e291` (CS_Tiling_Generate_Opaque) to the same opt-in allowlist:
  offline it also widens one branch, lowers to wave 32, validates and creates
  a pipeline. In-game verification of the two-program set is pending.
- `ldsflow2` result (two allowlisted programs, 1000 s): no device removal,
  exception or `id=538`; refusals fell from 8 to **6**. Median fps 3.9 over
  72 intervals against 4.1 in `ldsflow1`, but the scenes differ (the last
  `ldsflow2` interval was 2.9 fps at 6931 draws against 4.1 fps at 4793
  draws), so no speed claim is made. Diner close-up `frame-1250.png` and the
  prison window `frame-1530/2202.png` show clean faces and no new artefact;
  the blue/white boxes along the bottom edge are unchanged, so neither
  program feeds that symptom. The gate stays opt-in: pipeline creation and
  unchanged screenshots do not prove that both host halves observe each
  other's LDS writes, which needs the GPU fixture with swept writer counts.

### Full safe upstream support port (`integration/upstream-all` → `next/5080`, 2026-11-25)

- Worktree branch `integration/upstream-all` forked from `next/5080` @ `8df1055` and fast-forwarded `next/5080` to `fc3391b`. Everything architecture-neutral from upstream `085ee7c..32b31f7` is carried; the excluded switches stay out.
- Ported: PR #999 texture readback (`crates/victory-gpu/src/texback.rs`, `VICTORY_TEXTURE_BACK=1`, off unless set) together with the `04df435` flight/relay fence-and-range bookkeeping; the 64-thread card hash with `VICTORY_CARD_HASH_WIDE=1` (off unless set) and **both** dispatch sites routed through `card_compare_groups`; opt-in `VICTORY_CARD_CHANGED_LIST` changed-copy list with `take_dirty` preserving the relay's sent-note retain; `changed_runs` 64-byte SSE2 mask walk (default-on, byte-exact against the old walk with edge-case tests) with `VICTORY_CHANGED_RUNS=0|alt`; dormant opt-ins `VICTORY_CARD_HASH_LEN=1` and `VICTORY_CARD_BUFFERS_MIN_KB` (our default stays 1 MiB / 1024 MB) plus the placement-stats line; diagnostics-only `VICTORY_BACK_READS` with all read/store hooks; tests proving the texture-back parser is off for unset/`=0`.
- Excluded: `VICTORY_SHADER_OPT` bit 64 (upstream run 277 blackened water on NVIDIA; out-of-range four-dword load is driver-dependent), `VICTORY_ABSENT_MS=told` and its missing `ABSENT_ONCE`/absence-note prerequisites, upstream's descriptor fingerprint/skip-table stack, wave-wide/LDS widening outside our two-program `VICTORY_LDS_FLOW` allowlist, and frame-shot renaming. No `victory-shader` source was changed.
- Checks: `cargo check -p victory-gpu` and `--features diagnostics` both pass; `cargo test -p victory-gpu --lib` 142 pass (148 with diagnostics).
- Full offline validation on the combined build (`VICTORY_HOST_MAX_WAVE=32`, `translate --list --jobs 8 --validate --pipelines` on `intro_shadercache_1`, log `work/upstream_all_validate_wave32.txt`): 5726 containers / 385 duplicates, **2867/2867 translated**; VS/PS 1267 emitted, 1258 validated, 1267 pipelines; compute 1068/1068/1060; mesh 532/527/532; the same 8 wave64 compute refusals and 14 signature validation failures as the recorded baseline, i.e. **no translation or pipeline regression**. Without `VICTORY_HOST_MAX_WAVE=32` the same build translates for wave64 and reports 168 compute PSO failures; that setting is required for any comparison.
- In-game verification `upstreamall1` (1000 s requested, user-terminated after ~437 s with a clean `window closed` → `title did not finish suspending in time; ending the title`; no crash): every new switch off and no shader-opt/wave/dump/debug override. No device removal, `0x887A0005`, unhandled exception or `id=538`; the only PSO refusals are the known eight wave64 computes, with no new one. `textures of these kinds … copied back` never logged, so texture-back stayed off. Final frame lines: ~4.5-4.7 fps, 213-224 ms/frame (preparing 84-89 ms, GPU 74-80 ms, copying back 139-149 ms, ~64 MB), ~4,850 draws — no timing regression against the earlier ~240 ms runs (different scene/duration, so no speed claim). 256 captures. The blue/white boxes along the bottom edge are still present (`frame-2595.png`) and the stylised cutscene frame (`frame-992.png`) is clean, i.e. no new artefact from the port.

### LDS-flow GPU fixture (`f461ae0`)

- New hardware test `emit::wave64_tests::lds_write_and_read_in_one_widened_branch_are_visible_across_halves`: a synthetic console wave64 compute whose LDS write and read are in one lane branch (`ds_or_b32` into slot `t >> 5`, then `ds_read_b32` of both the lane's own and the other half's slot, two stores per lane). Per-lane lowering keeps wave 64 and `branches_around` names the branch; widening lowers to wave 32 (both asserted via a test-only `LAST_WAVE`). On the RTX 5080 with writer counts **1/31/32/33/64** every lane's two dwords match the expected OR of its own and the opposite half's writers — notably `33` (one writer in the high half) and `64` require the high half's writes to be visible to low-half reads. This closes the gap left by pipeline creation and screenshots: the two-host-half LDS visibility now has a direct GPU fixture.
- Caveat: a negative control with `VICTORY_GROUP_SYNC=0` (barriers keep the shared-memory fence but lose the group wait) also passed on this card. So the fixture proves the writes are visible in the widened path, but it does **not** distinguish a lost group wait from a real one on this hardware; a missing barrier entirely would still be caught by the expected values on a memory model that leaves stale zeros. `VICTORY_LDS_FLOW=1` therefore stays opt-in: the fixture covers the mechanism's shape, not the two production programs' pixels.
- `cargo test -p victory-shader --lib`: 212 pass; the 4 failures are the pre-existing `emit::intersect_tests` WARP one-ulp cases (`batch 4 thread 25`, `1.999999` vs `1.9999989`), reproduced identically at the parent commit, so they are not caused by the fixture.

### LDS barrier-coalescing regression fix

- Reproduced an unguarded LDS write→read bug with a new deterministic lowering test: the emitted order was `meeting, write, read, read`, missing the cross-half meeting after the write. The earlier widened-branch hardware fixture hid this case because guarded writes create a fresh DXIL join block.
- `lower.rs` now invalidates `meet_clean` after emitting an LDS write/RMW, not just before it. The regression test requires `meeting, write, meeting, read, read`: write→read ordering is restored while consecutive reads still coalesce. Test-only emission tracing avoids relying on GPU scheduling to expose a race (the previous fence-only negative control passed on this card).
- All 8 `emit::wave64_tests` pass, including hardware cross-half visibility for 1/31/32/33/64 writers; diagnostics executable builds. Full saved-pack validation with host wave 32 (`work/lds_barrier_fix_validate.txt`) matches the recorded baseline: 2867/2867 translated; VS/PS 1267 emitted / 1258 validated / 1267 pipelines, compute 1068/1068/1060, mesh 532/527/532. No experimental defaults changed; in-game visual/performance effects are not yet measured.
- Expanded the deterministic regression to cover plain LDS stores, commutative OR atomics and returning exchange atomics. The hardware fixture now explicitly initializes both LDS slots: uninitialized D3D12 groupshared contents are not a valid test oracle. All 8 tests still pass; one unrelated wave64-only hardware case reports a skip, not a successful execution.

### Texture staging safety and partial-refresh follow-up

- `staging_bytes` now uses checked arithmetic and rejects mismatched footprint/row arrays, multiplication/addition overflow and allocations beyond the CPU slice `isize::MAX` bound. Added negative cases alongside the last-row-padding regression; the allocation path propagates failure rather than constructing an invalid mapped slice.
- The previously deferred DexNom `partial_due` change is now implemented with a regression test. A guard-reported page write previously advanced `asked` and then could be discarded by the unconditional 30-frame partial-texture throttle. Reported writes now refresh immediately; unchanged partial textures retain the 30-frame poll. No extra polling switch or periodic all-texture scan was added.
- All 5 texture title-memory tests pass, and `cargo check --workspace` passes. In-game effects and cost of the partial-refresh change remain unmeasured; do not infer a mirror or bottom-edge fix from these tests.

### Exact SSA check of `926c46bab3b86bd6`

- A read-only review suggested the `0x13c8` refusal might be a numeric loop counter spuriously classified as a mask. Temporary instrumentation at `mask_dword` disproved that candidate: failing SSA values `5682` / `4906` (different lowering attempts) are defined by `s_mov_b64 s[24:25], s[96:97]` at native `0x1a54`, in the block starting at `0x1a50`, not by `s47`'s numeric increment. Disassembly shows `s[96:97]` saves `s[24:25]` at `0x163c`; `s[24:25]` is a real compare mask at `0x136c` and is later consumed by `v_cndmask_b32` at `0x1ae0`.
- Evidence: `work/926_mask_debug.txt` and `work/926c46bab3b86bd6_disasm.txt` (outside Git). Temporary instrumentation removed. No scalar/mask classification change or bypass landed; the exact refusal remains. A review's successful CPU-only `VICTORY_WAVE_WIDE=1` emission (29 widened branches) is not GPU/correctness evidence and does not justify enabling it.
- CPU shader tests excluding `emit::`: 103/103 pass.

### Combined stability verification follow-up

- Independent read-only audit of `de21419` and `ba3b631` found no blocker in barrier ordering, staging-span safety or partial-write refresh. The per-access test trace is deliberately not a complete trace of title/region barriers. The hardware fixture now initializes every output lane as well as its LDS slots, removing dependency on fresh UAV contents.
- Strengthened `a_title_rewrite_clears_this_frames_read_mark` with direct read-bit and same-frame next-draw consumption assertions. Negative control with the mark-clearing fix temporarily disabled failed (`TABLED_READ` remained 2147483648); restored code passes. Negative and positive logs are `work/descriptor_rewrite_negative_control.txt` and `work/descriptor_rewrite_positive_control.txt`; no temporary disabling remains.
- GPU crate diagnostics suite: 153 pass / 3 ignored. Full shader suite: 213 pass / 4 fail / 3 ignored; all four failures are the unchanged pre-existing WARP one-ULP `intersect_tests` cases, not waived or loosened. Texture-layout tests: 10/10 pass. Logs: `work/stability_gpu_tests.txt`, `work/stability_shader_tests.txt`.
- `stabilityfix1` requested 600 s but ended after ~174 s through `window closed` → suspension timeout; no device removal, unhandled exception, `id=538` or detiling warning, and the same 8 known pipeline refusals. This is not a completed 600-second stability test. 117 captures; `frame-1512.png` still shows bottom-edge block corruption and unresolved lighting. Final timing intervals were slow (1.0–2.2 fps, 448–1014 ms/frame; texture preparation 26–151 ms, heap passes 342–670 ms), so neither visual correctness nor a performance non-regression is established.
- CPU-only `intro_shadercache_2` scan completed (`work/stability_cache2_translate.txt`): 6369 containers / 462 duplicates, 3185/3185 programs decoded to IR, graphics 1399/1399 emitted, compute 1137/1138 emitted, mesh 648/648 emitted. Only DXIL emission refusal: known `926c46bab3b86bd6` at `0x13c8`, mask containing lanes elsewhere read as data. No validator or pipeline creation requested; zero validation/pipeline counts are not failures and do not establish pixel correctness.

### Upstream round `32b31f7..988aa41`, Zen 2 and RDNA 2 studies (2026-11-26)

- Upstream added ~40 commits (mostly diagnostics). Two independent reviews separated correctness from performance. Ported three switch-free correctness fixes: alias copies from a GPU-written texture into a placed view even when a title upload already used that view (fork PR #1241; the 42→67 packed-light path is live in our logs); CBV sizes capped at 64 KiB before 256-byte rounding (PR #911/#1206; invalid larger views remove NVIDIA devices); dynamic descriptor slots find a free run across the search's start (ring tables were refused with room). All three have tests; the alias and slot tests fail with the pre-fix code. `VICTORY_ABSENT_MS=told` and `VICTORY_TARGET_VARIANTS` were deliberately left out (no prerequisite / switch default-on upstream, no local evidence).
- Every upstream perf switch in this round was left out: `FILLS_PLAIN`, `STEP_INDEX`, `LIGHT_REFRESH`, `GATHER_BY_COPY`, `HEAP_SYNC_CUT`, `HEAP_PASS_MARKED`/`LIST_REUSE`, `BVH_NO_HIT`, `VIEWS_REMADE_QUIET`, `WAVE_WIDE=last`. Only `LOOKUP_CACHE` had positive controlled evidence (~14 ms ±7 on a 4070/14700KF), but it needs its paired correctness fix and an audit of our extra `EXACT`/`inplace` tables, so it was not ported. `CARD_SYNC_BATCH` has a real card-time saving but no frame proof. All upstream numbers are on different hardware; none is claimed here.
- Zen 2 study: our machine is Zen 5 with one CCD and 96 MB shared L3, so Zen 2's CCX capacity and memory-renaming lessons do not transfer. The measured CPU cost is kernel reads and D3D12 view creation in `sync_heaps`, not atomic contention. Added a diagnostics-only split of `sync_heaps` time into page reads versus slot mirroring; no atomic padding, no AVX-512 walk, no relay affinity change. Sources: AMD Zen 2/Zen 5 optimization guides via x86-64.net, AnandTech Zen 2, TechPowerUp 9800X3D.
- RDNA 2 study: wave64 EXEC/LDS/scalar facts confirm the existing split/exchange/barrier design. Added one new **opt-in** `VICTORY_BARRIER_COALESCE=1`: a title `s_barrier` is dropped when the translator emitted the same group wait and shared fence in the same block with no store or LDS write since. A deterministic test proves both forms and the write-invalidation negative control; 9 wave64 tests pass. `VICTORY_COMPUTE_QUAD_FAST` and `VICTORY_BARRIER_UAV_SCOPE` were designed but not implemented; no wave-wide default, no GDS relaxation, no descriptor invention. Sources: AMD RDNA 2 ISA, ChipsAndCheese RDNA 2/RDNA 3, XCL wave64-on-wave32.
- Visual diagnosis: bottom-edge corruption is in the 3840×2160 present source, not in any dumped 2560×1440 G-buffer; the present `CopyResource` is verbatim. The corrupted band begins near the recorded `scissor [548 0 1754 1334]` cut, but no HDR or 3840×2160 composite target was dumped at that frame, so the first bad pass is still unknown. No visual fix was attempted or claimed.
- Frame rate remains poor (2.9-4.4 fps in the diagnostic dump run, which distorts timing) and no stability or performance success is claimed. Full saved-pack validation after these changes (`work/upstream_fixes_validate.txt`, host wave 32): **2867/2867 translated**; VS/PS 1267 emitted / 1258 validated / 1267 pipelines, compute 1068/1068/1060, mesh 532/527/532; the same 8 wave64 compute refusals and 14 signature-validation failures as the baseline. No translation or pipeline-creation regression from the correctness fixes or the opt-in barrier change (off in this run).
- **cgbj-reupload ports (2026-11-26):** per-instruction EXEC guard in descriptor liveness (`resource.rs`, commit `1ae6ba8`) is switch-free; its clone bug journal ties the missing guard to `CS_AlphTransmissionPrefilterMip` (`2da4beb386509b77`) being dropped under wave32, which zeroed the transmission buffer and produced black squares/corrupted blending. **Measured:** the `fencebase` boot (guard on, all experimental switches off) had **7** pipeline refusals, not 8; `2da4beb386509b77` is the one that stopped being refused (~4.7-5.8 fps, 89 ms fetch wait, otherwise clean). The remaining seven are `369cbe906ea72764`, `caf96a3229145fa6`, `2cc4c197ee29e291`, `02384a60fda6f2d0`, `6b82bc7d22016cb5`, `5dc3b25986622949`, `6f225b641c5b0708`. Full `victory-shader` suite: 215 passed, only the 4 known WARP `emit::intersect_tests` one-ULP failures. Blocking null-event fence wait via `WaitOnAddress`/`WakeByAddressAll` (`fence.rs`, `VICTORY_FENCE_BLOCK=1`, opt-in) and immediate release of detached dynamic descriptor slots on GPU completion (`list.rs`/`host.rs`, from `f4b48ad`) are ported but not yet validated in-game; neither is a default. `releasetables` (all switches off): clean, 7 refusals, no "list ran again" warning, ~4.5-5.6 fps and ~90 ms fetch wait against `fencebase`'s ~4.7-5.8 and ~89 ms — neutral here, kept as a correctness fix for NVIDIA dynamic-slot exhaustion rather than an FPS claim. `fenceblock` (`VICTORY_FENCE_BLOCK=1`): clean, 7 refusals, zero "not implemented" and zero "timed out" warnings (the wait was used and reached its value), ~4.8-5.7 fps and ~69 ms fetch wait. a second run (`fenceblock2`) gave ~4.7-5.7 fps and ~79 ms. **Conclusion: inert here** — `blocking fence wait not implemented` was zero in *all four* boots (fencebase, releasetables, fenceblock, fenceblock2), so the null-event path was never taken and the fetch-wait differences are scene noise. The switch stays off; no default change. Upstream card-sync batching (`VICTORY_CARD_SYNC_BATCH=1`, port of `6aed078`) was also A/B'd: `syncbatch` was clean (7 refusals, no corruption), ~4.9-5.3 fps with 53-66 ms GPU and 76 ms fetch wait, against baselines of 4.5-5.8 fps, 45-79 ms GPU and 89-90 ms fetch wait. No measured win on this machine; it stays opt-in and off, like upstream. The AMD ambient stand-in, 32 GiB residency staging budgets, Series S two-plane stride, hardcoded scene hashes and `VICTORY_LDS_FLOW`-forcing scene kernels were reviewed and deliberately left out.
- **Default change:** the 64-thread card hash is now on by default; `VICTORY_CARD_HASH_WIDE=0` restores the old one-thread form. Two opt-in runs (`hashwide1`, `hashwide2`) gave ~5.0–6.0 and ~4.9–5.7 fps with ~76 and ~64 ms fetch wait; the following baseline (`baseline_after`) gave ~4.3–5.2 fps with ~83 ms, and `fixes1`/`inflight1`/`barriercoal1` were ~4.1–5.0 with ~82–118 ms. Scenes differ, but the direction was consistent across two runs each way; both forms were clean with the same 8 refusals. This is a measured local decision, not an upstream claim.
- Controlled A/B boots on this machine with the same `-speed` settings, all against the `fixes1` baseline (~4.3–4.5 fps, fetch wait ~100–118 ms): `inflight1` with `VICTORY_CARD_FETCH_INFLIGHT=1` gave ~4.1–5.0 fps and ~99–103 ms fetch wait (no gain); `barriercoal1` with `VICTORY_BARRIER_COALESCE=1` gave ~3.9–4.5 fps and ~101 ms fetch wait (no gain). Both switches remain off. A `VICTORY_CARD_HASH_WIDE=1` run is in progress. No default was changed on a negative result.
- In-game `fixes1` (600 s, all experimental switches off): no device removal, `0x887A0005`, unhandled exception, `id=538` or detiling warning; only the same 8 known pipeline refusals. Final frame lines ~4.3–4.5 fps, 221–232 ms/frame (preparing 73–76 ms, GPU 77–97 ms, copy-back 139–163 ms), 343 captures. The bottom-edge blocks are still present in `frame-3244.png`: these correctness fixes do not fix that artifact. Descriptor timing split (`descriptor records the title copied...`): ~197–205 changed heap pages, ~195k–225k slots looked at, ~38k–40k mirrored, 23.3–23.6 ms/frame, of which only **0.83–0.86 ms is kernel page reads and ~20.7–21.0 ms is resolving/mirroring slots**. This disproves the syscall-coalescing hypothesis; any further CPU gain must come from the mirror/view-creation loop, not from merging reads. No claim of a performance or visual fix.
- Mirror-kind counters added (`062d3b0`) and measured in `mirror1`: of ~219k–244k slots looked at per frame, the mirror loop handled ~2.4k–3.4k texture views actually made, **~48k–57k buffer views left for their draws**, ~21k–25k nulls and ~70k–110k records whose key was already made. So D3D12 texture view creation is not the bottleneck; the per-slot bookkeeping (multiple heap locks, `note_buffer`, `changed`/`buffers` sets) is. No invasive lock-free rewrite was attempted without a measured target; the byte-level descriptor cache remains as previously validated. `mirror1` also had exactly the 8 known refusals and no device removal/exception.
- Tried a bit-exact barrier on the `p*q - r*s` products in `emit/intersect.rs` to stop WARP's undocumented contraction. It moved the one-ULP failure from the fourth dword to the first on a different ray: the contraction happens across the shear arithmetic too. A partial barrier is misleading, a full barrier set would change every float chain in every translated program, and NVIDIA hardware already passes the reference. The change was reverted; the 4 WARP one-ULP failures remain open, not waived.
- Range capture `range1` (`VICTORY_CAPTURE_FRAMES=1320..1400`, opt-in target dumps) finally recorded the lighting/composite lists. At frame 1322 the HDR ping-pong source `0xface3919f0` (`frame-1322-210-...-f10.png`) has stale pixels of an earlier close-up scene in its bottom rows; the next fullscreen pass (ops 5565, pixel `f79e8a2e8c379374`) reads it into `0xface391b90` (dump 208), and the 3840×2160 present source (dump 260) and final `frame-1322.png` carry the same blocks. The source is written by ops 5557 (pixel `4faa43fd81d9aae1`, reading depth `0xfa459f2180` and target B `0xface391b90`); that draw is present with full `scissor [0 0 2560 1440]` / viewport and `indirect kind 0 max 1`, not dropped. So the artifact first appears in the HDR ping-pong target, not in the present copy or the geometry G-buffer. Why those rows were not overwritten is still unknown; no speculative render-target or alias change was made. `range1` had exactly the 8 known refusals and no device removal/exception.
- **Decisive follow-up:** the emitted DXIL for `4faa43fd81d9aae1` was captured via `VICTORY_SHADER_SAVE` (`work/savedshaders/4faa43fd81d9aae1.pixel.dxil`) and disassembled with `dxc.exe -dumpbin ... -Fc ...` (`work/4faa43_emitted.ll`). There is **no `discard`** in the emitted shader; it has 12 `dx.op.textureLoad.f32` calls and four trailing `dx.op.storeOutput.f32` calls. So the synthetic export-guard finding is not the cause of this shader's stale rows; no export-side fix can help it. The stale values must come from the ping-pong inputs the shader samples.
- A deterministic test showed a real guard on a synthetic HDR-like export: native `v_cmpx_ne_u32` narrowing plus `s_cbranch_execz` followed by `s_mov_b64 exec, s[0:1]` left `st.guards[export] = Some(...)`, which lowering turns into `discard`—a mechanism that could matter for other titles, but **not** for this one. A structure-pass fix (track EXEC at block exit for masked branches that join a restore block, and recognize `_b64` `s_mov` in wave32) made the test pass, but full validation then showed **85 pixel programs refused with "a conditional export"** instead of the baseline 0, and guarded instructions rose from 197,896 to 211,199. Narrowing the `_b64` handling did not remove the regression; a still narrower phi-only variant did not fix the test. All attempts were reverted (`e9f0cc6` and a clean tree afterwards); no shader regression is shipped. The root cause is still unproven and the visual artifact remains open. A safe fix needs a per-export reconvergence analysis that leaves all other guards untouched.
- RTV clears are now recorded in frame ops and counted by outcome (diagnostics only). `clearwhy1`: 30 clears asked and 30 issued per frame, none dropped. `clearwho` names the cleared resource: **no clear of `0xface3919f0`** occurs. The HDR target's bottom band is numerically near-identical across 1321–1336 (mean ≈9.2, nonzero share ≈0.347), and a scan of all dumped targets finds no other surface holding those pixels—they are the target's own older history. Controlled A/B `scissorfull1` with `VICTORY_SCISSOR_TEST=full` did **not** remove the blocks (they grew with more stale texels): the title's scissor rectangle is not the cause. The switch was set only for that run and stays off by default. Native disassembly of pixel `4faa43fd81d9aae1` has exactly one `exp mrt0 ... done compr vm` at `0x4ec` with no branch around it, so the shader exports every pixel; the stale values are therefore written from stale sampled inputs (the two HDR ping-pong targets feed each other), not skipped by a discard. Root cause of the wrong sampled inputs remains unproven; no visual fix is claimed.
- Started targeted `lightingtrace1` (480 s requested) with opt-in intermediate-target dumps and world-relative frames 118/120/160. This is correctness diagnosis, not a benchmark. No approximate wave/scalar/RT compensation switch was enabled.

### DexNom fork re-check (2026-11-25, `538568d`)

- `DexNom/VIctory` moved `2e01ab4 → 64bd58f` (4 commits today): `ab34a8b` "Publish the FSR Quality baseline as DexNom RayTime" (54 files, +9,443/−1,228), then three README-only commits (`e0ce365` rename, `f08fee5` RayTime priorities, `64bd58f` experimental NVIDIA modes). Two independent reviews (README/claims and code) found **no NVIDIA support claim and no mirror/glass/exposure fix**: the new README says "untested NVIDIA launch options" and "NVIDIA gameplay is untested"; all numbers are single runs on an RX 7900 XTX / i9-285K (or synthetic shader tests), `BENCHMARK.md` was not touched, and the overexposed doorway/silhouetted-character issue is listed as still open. FSR 4.1.2 plumbing (`amd_fidelityfx_upscaler_dx12.dll`), RT master/static-BVH switches, `VICTORY_AMBIENT_FALLBACK` material fill, raw-store probes, upload pooling, `tiles.rs`/`ahead.rs` sparse paths and the `lower.rs` test relocation were all reviewed and **avoided** (AMD/RT/driver-specific, visual tuning, or switch-gated without proof). Our capture already has the equivalent float-texel exposure log.
- Three architecture-neutral, switch-free correctness fixes were ported from `ab34a8b` with unit tests:
  1. `texture.rs` staging bytes now grow past `GetCopyableFootprints`' total so every subresource's complete pitched rows (including the last row's padding) fit the buffer — detiling addresses whole rows, so the old sizing could be short (and constructing the slice past the allocation was UB). Test `staging_includes_the_last_rows_padding`.
  2. `texture.rs` `Last::advance_watch` keeps the newest of the query and any published write stamp instead of overwriting it with an older query — a write published during a submission can no longer be passed over as unwritten (and redundant re-reads are avoided). Test `a_newer_write_stamp_is_not_lost`.
  3. `descriptor/host.rs` clears this frame's `TABLED_READ` mark when a slot is rewritten before a draw reads it, so the replacement view stages again instead of inheriting the old view's "up to date" state. Test `a_title_rewrite_clears_this_frames_read_mark`.
- DexNom's write-driven `partial_due` (replacing our 30-frame poll for partly mapped textures) and its zero-read `asked = 0` retry reset were **not** ported: neither is proven here, and both would change refresh cost/frequency. No switch was added and no default was changed beyond these fixes.
- Checks: `cargo test -p victory-gpu --lib` 145 pass (152 with diagnostics); `cargo check --workspace` passes.
- In-game verification `dexnfix1` (600 s, stopped by the runner at the timeout, all experimental switches unset, `-speed` settings only): no device removal, `0x887A0005`, unhandled exception or `id=538`; the only PSO refusals are the same eight wave64 computes; no `texture not detiled` warning and texture-back stayed off. Final frame lines ~4.2-5.1 fps, 195-236 ms/frame (preparing 75-79 ms, GPU 73-96 ms, copying back 113-161 ms, ~53-65 MB) — in the same range as `upstreamall1`, so no cost regression from the fixes. 350 captures. The bottom-edge boxes are still present in `frame-3303.png` and match the earlier runs; no new artefact appeared.
