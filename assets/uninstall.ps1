$currentUser = [Security.Principal.WindowsIdentity]::GetCurrent()
$principal = New-Object Security.Principal.WindowsPrincipal($currentUser)
if (-not $principal.IsInRole([Security.Principal.WindowsBuiltinRole]::Administrator)) {
    Write-Host "Not running as administrator. Restarting with elevated privileges..."

    # Relaunch the script as administrator
    $psi = New-Object System.Diagnostics.ProcessStartInfo
    $psi.FileName = "powershell.exe"
    $psi.Arguments = "-NoProfile -ExecutionPolicy Bypass -File `"$PSCommandPath`""
    $psi.Verb = "runas"  # This triggers the UAC prompt
    try {
        [System.Diagnostics.Process]::Start($psi) | Out-Null
    } catch {
        Write-Error "The script requires administrator privileges to run."
    }
    exit
}

reg import "C:\Program Files\WinImageMagic\uninstall.reg"

Remove-Item -Path "C:\Program Files\WinImageMagic" -Recurse -Force

$appKey = "WinImageMagic"
$regPath = "HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\$appKey"

if (Test-Path $regPath) {
    Remove-Item -Path $regPath -Recurse -Force
    Write-Host "Uninstall registry entry removed."
} else {
    Write-Host "Uninstall registry entry not found."
}

Write-Host "Press any key to continue..."
[void][System.Console]::ReadKey($true)