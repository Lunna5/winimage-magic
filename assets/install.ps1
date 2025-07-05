function Download-FromGitHubRelease {
    param (
        [Parameter(Mandatory)]
        [string]$DownloadUrl,

        [Parameter(Mandatory)]
        [string]$DestinationPath
    )

    Write-Host "Downloading from $DownloadUrl to $DestinationPath ..."
    try {
        Invoke-WebRequest -Uri $DownloadUrl -OutFile $DestinationPath -ErrorAction Stop
        Write-Host "Downloaded successfully."
    }
    catch {
        Write-Error "Failed to download $DownloadUrl. $_"
        throw $_
    }
}

# Check if running as administrator
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

# --- Script continues here as admin ---

# Destination folder where the .exe will be installed
$destinationFolder = "C:\Program Files\WinImageMagic"

# Create the destination folder if it doesn't exist
if (-not (Test-Path -Path $destinationFolder)) {
    Write-Host "Creating destination folder: $destinationFolder"
    New-Item -ItemType Directory -Path $destinationFolder | Out-Null
}

# Example files to download (you can add more here)
$filesToDownload = @(
    "https://github.com/Lunna5/winimage-magic/releases/latest/download/winimage-magic.exe"
    "https://github.com/Lunna5/winimage-magic/releases/latest/download/uninstall.ps1"
    "https://github.com/Lunna5/winimage-magic/releases/latest/download/uninstall.reg"
)

foreach ($fileUrl in $filesToDownload) {
    $exeName = [System.IO.Path]::GetFileName($fileUrl)
    $destinationPath = Join-Path $destinationFolder $exeName

    Download-FromGitHubRelease -DownloadUrl $fileUrl -DestinationPath $destinationPath
}

# Run the main .exe with the --install argument
$exeFullPath = Join-Path $destinationFolder "winimage-magic.exe"
Write-Host "Executing winimage-magic.exe with --install..."
& "$exeFullPath" --install

# Define Application details for uninstallation
$appKey = "WinImageMagic"
$displayName = "WinImageMagic"
$appFolder = "C:\Program Files\WinImageMagic"
$uninstallScript = Join-Path $appFolder "uninstall.ps1"
$displayIcon = Join-Path $appFolder "winimage-magic.exe"
$displayVersion = "0.1.1"
$publisher = "Lunna5"
$installLocation = $appFolder

$regPath = "HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\$appKey"

if (-not (Test-Path $regPath)) {
    New-Item -Path $regPath -Force | Out-Null
}

$uninstallString = "powershell.exe -NoProfile -ExecutionPolicy Bypass -File `"$uninstallScript`""

Set-ItemProperty -Path $regPath -Name "DisplayName" -Value $displayName
Set-ItemProperty -Path $regPath -Name "UninstallString" -Value $uninstallString
Set-ItemProperty -Path $regPath -Name "DisplayIcon" -Value $displayIcon
Set-ItemProperty -Path $regPath -Name "DisplayVersion" -Value $displayVersion
Set-ItemProperty -Path $regPath -Name "Publisher" -Value $publisher
Set-ItemProperty -Path $regPath -Name "InstallLocation" -Value $installLocation

Write-Host "Installation completed."

Write-Host "Press any key to continue..."
[void][System.Console]::ReadKey($true)
