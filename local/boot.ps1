param([string]$tag = "deep9", [int]$seconds = 150,
      [string]$probe = "1", [string]$sample_every = "1000", [string]$full = "2,8",
      [string]$capture_at = "500,3000")
$ErrorActionPreference = "Continue"
$base = @{ LEONIDA_D3D12 = "vulkan"; DISABLE_RTSS_LAYER = "1"; DISABLE_VULKAN_OBS_CAPTURE = "1" }
foreach ($k in $base.Keys) { Set-Item -Path "Env:\$k" -Value $base[$k] }
$env:LEONIDA_CAPTURE = "1"
$env:LEONIDA_CAPTURE_AT = $capture_at
$env:LEONIDA_XS_FSPOBJ_DUMP = "1"
if ($probe -ne "0") {
    $env:LEONIDA_PRESENT_PROBE = "1"
    $env:LEONIDA_PRESENT_PROBE_SAMPLE_EVERY = $sample_every
    if ($full) { $env:LEONIDA_PRESENT_PROBE_FULL = $full }
} else {
    foreach ($k in "LEONIDA_PRESENT_PROBE", "LEONIDA_PRESENT_PROBE_SAMPLE_EVERY", "LEONIDA_PRESENT_PROBE_FULL") {
        Remove-Item Env:\\$k -ErrorAction SilentlyContinue
    }
}
foreach ($k in "LEONIDA_THREADS", "LEONIDA_WATCH", "LEONIDA_SKIP_INT3") {
    Remove-Item Env:\\$k -ErrorAction SilentlyContinue
}
$a = @("--run", "D:\GTA6.Xbox.v1.0.2.1\Leonida\game_scarlett_master.exe",
       "--game-dir", "D:\GTA6.Xbox.v1.0.2.1", "--overlay", "D:\AIProjects\GTA6PC\overlay",
       "--trace", "D:\AIProjects\GTA6PC\trace_$tag.jsonl")
$p = Start-Process -FilePath "D:\AIProjects\GTA6PC\gta6rt.exe" -ArgumentList $a `
    -WorkingDirectory "D:\AIProjects\GTA6PC" `
    -RedirectStandardOutput "D:\AIProjects\GTA6PC\trace_${tag}_stdout.txt" `
    -RedirectStandardError "D:\AIProjects\GTA6PC\trace_${tag}_stderr.txt" -PassThru
Start-Sleep -Seconds $seconds
if (-not $p.HasExited) { Stop-Process -Id $p.Id -Force }
Start-Sleep -Seconds 3
$err = (Get-Item "D:\AIProjects\GTA6PC\trace_${tag}_stderr.txt").Length
$pres = (Select-String -Path "D:\AIProjects\GTA6PC\trace_${tag}_stderr.txt" -Pattern "PresentX calls=" | Select-Object -Last 1)
$cmd  = (Select-String -Path "D:\AIProjects\GTA6PC\trace_${tag}_stderr.txt" -Pattern "list commands=" | Select-Object -Last 1)
$drw  = (Select-String -Path "D:\AIProjects\GTA6PC\trace_${tag}_stderr.txt" -Pattern "replay: draws=" | Select-Object -Last 1)
"${tag}: stopped pid $($p.Id) stderr=$err last '$($pres.Line)' last '$($cmd.Line)' last '$($drw.Line)'"
