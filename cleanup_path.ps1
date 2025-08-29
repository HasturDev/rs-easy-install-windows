# Clean up PATH environment variables
$userPath = [Environment]::GetEnvironmentVariable('PATH', 'User')
$newUserPath = ($userPath -split ';' | Where-Object { $_ -notlike '*cargo*' -and $_ -notlike '*rustup*' }) -join ';'
[Environment]::SetEnvironmentVariable('PATH', $newUserPath, 'User')
Write-Host "Cleaned user PATH"

$systemPath = [Environment]::GetEnvironmentVariable('PATH', 'Machine')
$newSystemPath = ($systemPath -split ';' | Where-Object { $_ -notlike '*cargo*' -and $_ -notlike '*rustup*' }) -join ';'
[Environment]::SetEnvironmentVariable('PATH', $newSystemPath, 'Machine')
Write-Host "Cleaned system PATH"

Write-Host "PATH cleanup complete. Please restart your terminal."