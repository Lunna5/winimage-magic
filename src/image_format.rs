use core::fmt;

use clap::ValueEnum;
use image::ImageFormat;


#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
#[clap(rename_all = "kebab_case")]
pub enum ClapImageFormat {
    /// An Image in PNG Format
    Png,

    /// An Image in JPEG Format
    Jpeg,

    /// An Image in GIF Format
    Gif,

    /// An Image in WEBP Format
    WebP,

    /// An Image in general PNM Format
    Pnm,

    /// An Image in TIFF Format
    Tiff,

    /// An Image in TGA Format
    Tga,

    /// An Image in DDS Format
    Dds,

    /// An Image in BMP Format
    Bmp,

    /// An Image in ICO Format
    Ico,

    /// An Image in Radiance HDR Format
    Hdr,

    /// An Image in OpenEXR Format
    OpenExr,

    /// An Image in farbfeld Format
    Farbfeld,

    /// An Image in AVIF Format
    Avif,

    /// An Image in QOI Format
    Qoi,

    /// An Image in PCX Format
    Pcx,
}

impl Into<ImageFormat> for ClapImageFormat {
    fn into(self) -> ImageFormat {
        match self {
            ClapImageFormat::Png => ImageFormat::Png,
            ClapImageFormat::Jpeg => ImageFormat::Jpeg,
            ClapImageFormat::Gif => ImageFormat::Gif,
            ClapImageFormat::WebP => ImageFormat::WebP,
            ClapImageFormat::Pnm => ImageFormat::Pnm,
            ClapImageFormat::Tiff => ImageFormat::Tiff,
            ClapImageFormat::Tga => ImageFormat::Tga,
            ClapImageFormat::Dds => ImageFormat::Dds,
            ClapImageFormat::Bmp => ImageFormat::Bmp,
            ClapImageFormat::Ico => ImageFormat::Ico,
            ClapImageFormat::Hdr => ImageFormat::Hdr,
            ClapImageFormat::OpenExr => ImageFormat::OpenExr,
            ClapImageFormat::Farbfeld => ImageFormat::Farbfeld,
            ClapImageFormat::Avif => ImageFormat::Avif,
            ClapImageFormat::Qoi => ImageFormat::Qoi,
            ClapImageFormat::Pcx => ImageFormat::Pcx,
        }
    }
}

impl fmt::Display for ClapImageFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClapImageFormat::Png => write!(f, "png"),
            ClapImageFormat::Jpeg => write!(f, "jpeg"),
            ClapImageFormat::Gif => write!(f, "gif"),
            ClapImageFormat::WebP => write!(f, "webp"),
            ClapImageFormat::Pnm => write!(f, "pnm"),
            ClapImageFormat::Tiff => write!(f, "tiff"),
            ClapImageFormat::Tga => write!(f, "tga"),
            ClapImageFormat::Dds => write!(f, "dds"),
            ClapImageFormat::Bmp => write!(f, "bmp"),
            ClapImageFormat::Ico => write!(f, "ico"),
            ClapImageFormat::Hdr => write!(f, "hdr"),
            ClapImageFormat::OpenExr => write!(f, "openexr"),
            ClapImageFormat::Farbfeld => write!(f, "farbfeld"),
            ClapImageFormat::Avif => write!(f, "avif"),
            ClapImageFormat::Qoi => write!(f, "qoi"),
            ClapImageFormat::Pcx => write!(f, "pcx"),
        }
    }
}

impl ClapImageFormat {
    pub fn registry_association_key(&self) -> String {
        format!(".{}", self.to_string())
    }

    pub fn supported_formats() -> Vec<ClapImageFormat> {
        vec![
            ClapImageFormat::Png,
            ClapImageFormat::Jpeg,
            ClapImageFormat::Gif,
            ClapImageFormat::WebP,
            ClapImageFormat::Pnm,
            ClapImageFormat::Tiff,
            ClapImageFormat::Tga,
            ClapImageFormat::Dds,
            ClapImageFormat::Bmp,
            ClapImageFormat::Ico,
            ClapImageFormat::Hdr,
            ClapImageFormat::OpenExr,
            ClapImageFormat::Farbfeld,
            ClapImageFormat::Avif,
            ClapImageFormat::Qoi,
            ClapImageFormat::Pcx,
        ]
    }

    pub fn supported_convertable_formats(&self) -> Vec<ClapImageFormat> {
        ClapImageFormat::supported_formats()
            .into_iter()
            .filter(|f| f != self)
            .collect()
    }
}