param([Parameter(Mandatory=$true)][string]$Keys)
$path = "D:\AIProjects\GTA6PC\c_keys_live.txt"
foreach ($k in $Keys -split ',') {
    Set-Content -Path $path -Value $k.Trim() -NoNewline
    Start-Sleep -Milliseconds 500
    Set-Content -Path $path -Value "" -NoNewline
    Start-Sleep -Milliseconds 250
}
