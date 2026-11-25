$ErrorActionPreference = "Stop"
$root = "D:\AIProjects\GTA6PC\VICTORY-5080"
$cap = "D:\AIProjects\GTA6PC\c_captures_live"
if (-not (Test-Path $cap)) { New-Item -ItemType Directory -Path $cap | Out-Null }
Set-Content -Path "D:\AIProjects\GTA6PC\c_keys_live.txt" -Value "" -NoNewline
$env:VICTORY_CAPTURE = $cap
$env:VICTORY_CAPTURE_MENUS = "1"
$env:VICTORY_CAPTURE_WORLD = "1"
$env:VICTORY_LOG = "info"
$env:VICTORY_GAME_DIR = "D:\GTA6.Xbox.v1.0.2.1"
$env:VICTORY_KEYS_FILE = "D:\AIProjects\GTA6PC\c_keys_live.txt"
Remove-Item Env:\VICTORY_KEYS -ErrorAction SilentlyContinue
Remove-Item Env:\VICTORY_KEYS_MASH -ErrorAction SilentlyContinue
Remove-Item Env:\VICTORY_KEYS_HOLD_MS -ErrorAction SilentlyContinue
Remove-Item Env:\VICTORY_KEYS_MASH_EVERY -ErrorAction SilentlyContinue
$args = @("run", "D:\GTA6.Xbox.v1.0.2.1\game_scarlett_master.exe",
          "--execute", "--console", "series-x")
$p = Start-Process -FilePath "$root\target\diag\victory.exe" -ArgumentList $args `
    -WorkingDirectory $root `
    -RedirectStandardOutput "D:\AIProjects\GTA6PC\c_live_stdout.txt" `
    -RedirectStandardError "D:\AIProjects\GTA6PC\c_live_stderr.txt" -PassThru
$p.Id | Set-Content "D:\AIProjects\GTA6PC\c_live_pid.txt"
"launched pid=$($p.Id)"
