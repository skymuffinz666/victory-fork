$ErrorActionPreference = "Stop"
$repo = Split-Path -Parent $PSScriptRoot
$dir = "D:\AIProjects\GTA6PC\_backup"
if (-not (Test-Path $dir)) { New-Item -ItemType Directory -Path $dir -Force | Out-Null }
$file = Join-Path $dir "VICTORY-5080-boot-5080.bundle"
$refs = @(git -C $repo for-each-ref --format='%(refname)' refs/heads)
if ($refs.Count -eq 0) { throw "no local branches to back up" }
Remove-Item -LiteralPath $file -Force -ErrorAction SilentlyContinue
git -C $repo bundle create $file @refs | Out-Null
$mb = '{0:N2}' -f ((Get-Item $file).Length / 1MB)
"backup updated: $file ($mb MB, $($refs.Count) refs) @ $(git -C $repo rev-parse --short HEAD)"
