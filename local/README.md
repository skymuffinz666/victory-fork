# Local boot tooling (RTX 5080 work)

Scripts carried over from the `victory-amd` investigation. All capture/log output goes to
`D:\AIProjects\GTA6PC` (outside this repository, per `docs/asset-policy.md`).

Diagnostics build:

```powershell
cargo +stable build --profile diag -p victory --features diagnostics
```

- `boot.ps1` — timed scripted-key boot (`VICTORY_KEYS`) with captures and the D3D12 debug option.
- `c_live_launch.ps1` — launches a run with `VICTORY_KEYS_FILE` control and captures, returns immediately.
- `c_press.ps1 -Keys Down,Down,Enter` — holds each named key for ~0.5 s through the control file, so the
  run can be driven live while reading its newest PNG in `c_captures_live`.
- `c_head_debug.ps1` — the same navigation with `VICTORY_D3D12_DEBUG=1`, used to investigate the
  mesh pipeline refused with `0x80070057` and the missing character geometry.
- `parse_head.py` — summarises the D3D12 debug messages in `c_head_stderr.txt`.

Input-driver additions in `gameinput.rs` (diagnostics builds only):

- `VICTORY_KEYS_HOLD_MS` — hold length (default 200 ms).
- `VICTORY_KEYS_MASH` / `VICTORY_KEYS_MASH_EVERY` — cycle keys on a timer.
- `VICTORY_KEYS_FILE` — hold comma-separated keys listed in a file, re-read every 50 ms (live control).

Measured navigation on the retail 1.0.2.1 package (EXE at the package root, not in `Leonida`):
account picker `A` → intro cutscene → `A` through Navigation/UI-text/Brightness/Finish Setup →
legal notices `Down,Down,A` then `Down,A` → privacy/terms checkboxes `A,Down,A,Down,A` → loading →
world. The first frame with ≥500 draws was logged as `the first frame of 500 draws or more`.
