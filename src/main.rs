use clap::Parser;
use log::{debug, info};
use winimage_magic::converter::convert_image_to_format_and_write;
use winimage_magic::image_format::ClapImageFormat;
use winimage_magic::text_util::strip_format;
use winimage_magic::windows_registry_install::WindowsRegistryInstaller;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "false")]
    install: bool,
    #[arg(short, long, required = false)]
    file: Option<String>,
    #[arg(long)]
    convert: Option<ClapImageFormat>,
    #[arg(long, default_value = "false")]
    compress: bool,
    #[arg(long, default_value = "9")]
    compression_level: u8,
    #[arg(short, long, default_value = "false")]
    debug: bool,
}

fn main() {
    if Args::parse().debug {
        unsafe {
            std::env::set_var("RUST_LOG", "debug");
        }
    } else { 
        unsafe {
            std::env::set_var("RUST_LOG", "info");
        }
    }

    env_logger::init();

    if Args::parse().install {
        let installer = WindowsRegistryInstaller::new();
        installer.install().expect("Failed to install WindowsRegistryInstaller");
        return;
    }

    info!("Starting winimage-magic...");
    let args = Args::parse();
    debug!("Parsed arguments: {:?}", args);
    
    if let Some(file) = &args.file {
        info!("Processing file: {}", file);
    } else {
        info!("No file specified, exiting.");
        return;
    }

    if let Some(convert_format) = args.convert {
        info!("Converting image to format: {:?}", convert_format);
        let file = args.file.expect("File must be specified for conversion");

        let final_path = strip_format(&file) + "." + &convert_format.to_string();

        convert_image_to_format_and_write(&file, &final_path, convert_format.into())
            .expect("Failed to convert image format");

        info!("Image converted and saved to: {}", final_path);
    } else {
        info!("No conversion format specified, processing file: {}", args.file.as_ref().unwrap());
    }
}
