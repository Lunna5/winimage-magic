# WinImage-Magic

A small utility for Windows that adds powerful image conversion options to your context menu.

## Features

-   **Easy to use:** Right-click on any image file to access the conversion options.
-   **Multiple formats supported:** Convert your images to a wide variety of formats.
-   **Lightweight:** No heavy dependencies or complex installation processes.

## Installation

1. Download the latest [install.ps1 clicking here](https://github.com/Lunna5/winimage-magic/releases/latest/download/install.ps1) or from the [release tab](https://github.com/Lunna5/winimage-magic/releases/) or just run the following command:
   ```ps
   curl https://github.com/Lunna5/winimage-magic/releases/latest/download/install.ps1 -o $env:TEMP\install-winimage-magic.ps1; powershell -ExecutionPolicy Bypass -File "$env:TEMP\install-winimage-magic.ps1"
   ```
3. Right-click the downloaded file and select "Run with PowerShell" to install.
4. After installation, you can access the conversion options by right-clicking on any image file in Windows Explorer.
5. To uninstall, go to settings and search for "Add or remove programs", find "WinImage-Magic" in the list, search WinImage-Magic in the search bar, and click "Uninstall."

## Supported Formats

-   PNG
-   JPEG
-   GIF
-   WebP
-   PNM
-   TIFF
-   TGA
-   DDS
-   BMP
-   ICO
-   HDR
-   OpenEXR
-   Farbfeld
-   AVIF
-   QOI
-   PCX

## Contributing

Contributions are welcome! If you have any ideas, suggestions, or bug reports, please open an issue or create a pull request.

## License

This project is licensed under the [GPL License](LICENSE).
