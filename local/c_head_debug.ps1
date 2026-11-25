$ErrorActionPreference = "Continue"
$root = "D:\AIProjects\GTA6PC\VICTORY-5080"
$cap = "D:\AIProjects\GTA6PC\c_captures_head"
if (-not (Test-Path $cap)) { New-Item -ItemType Directory -Path $cap | Out-Null }
$env:VICTORY_CAPTURE = $cap
$env:VICTORY_CAPTURE_MENUS = "1"
$env:VICTORY_CAPTURE_WORLD = "1"
$env:VICTORY_DUMP_TARGETS = "1"
$env:VICTORY_LOG = "info"
$env:VICTORY_GAME_DIR = "D:\GTA6.Xbox.v1.0.2.1"
$env:VICTORY_D3D12_DEBUG = "1"
$env:VICTORY_KEYS_HOLD_MS = "500"
$env:VICTORY_KEYS = "3:Enter,8:Enter,13:Enter,18:Enter,23:Enter,30:Down,31:Down,32:Enter,38:Down,39:Enter,45:Enter,46:Down,47:Enter,48:Down,49:Enter"
Remove-Item Env:\VICTORY_KEYS_FILE -ErrorAction SilentlyContinue
Remove-Item Env:\VICTORY_KEYS_MASH -ErrorAction SilentlyContinue
$args = @("run", "D:\GTA6.Xbox.v1.0.2.1\game_scarlett_master.exe",
          "--execute", "--console", "series-x")
$p = Start-Process -FilePath "$root\target\diag\victory.exe" -ArgumentList $args `
    -WorkingDirectory $root `
    -RedirectStandardOutput "D:\AIProjects\GTA6PC\c_head_stdout.txt" `
    -RedirectStandardError "D:\AIProjects\GTA6PC\c_head_stderr.txt" -PassThru
$seconds = 300
for ($i = 0; $i -lt $seconds; $i += 10) {
    Start-Sleep -Seconds 10
    if ($p.HasExited) { break }
}
if (-not $p.HasExited) { Stop-Process -Id $p.Id -Force }
Start-Sleep -Seconds 2
"exit=$($p.HasExited) stderr=$((Get-Item 'D:\AIProjects\GTA6PC\c_head_stderr.txt').Length) captures=$((Get-ChildItem $cap -File | Measure-Object).Count)"
