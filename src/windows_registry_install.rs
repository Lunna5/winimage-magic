use std::path::PathBuf;
use log::info;
use winreg::enums::{HKEY_CLASSES_ROOT, KEY_WRITE};
use winreg::RegKey;
use crate::image_format::ClapImageFormat;

pub struct WindowsRegistryInstaller {
    program_path: PathBuf,
}

impl WindowsRegistryInstaller {
    pub fn new() -> Self {
        let program_path = std::env::current_exe();

        if program_path.is_err() {
            panic!("Failed to get current executable path: {}", program_path.err().unwrap());
        }

        Self {
            program_path: program_path.unwrap(),
        }
    }

    pub fn install(&self) -> Result<(), String> {
        for format in ClapImageFormat::supported_formats() {
            self.install_format(format);
        }

        Ok(())
    }

    pub fn install_format(&self, format: ClapImageFormat) {
        let classes = RegKey::predef(HKEY_CLASSES_ROOT);

        let base = classes
            .open_subkey_with_flags("SystemFileAssociations", KEY_WRITE)
            .unwrap();

        println!("Installing format: {}", format.to_string());

        let shell = base
            .create_subkey_with_flags(format!("{}\\shell", format.registry_association_key()), KEY_WRITE)
            .unwrap().0;

        let group = shell
            .create_subkey("winimage-magic-convert")
            .unwrap();

        group.0.set_value("MUIVerb", &"WinImage-Magic").unwrap();
        group.0.set_value("Icon", &format!("{},{}", self.program_path.display(), 0)).unwrap();
        group.0.set_value("SubCommands", &"").unwrap();
        
        let subcommands_shell = group.0
            .create_subkey("shell")
            .unwrap();
        
        let convert_to = subcommands_shell.0
            .create_subkey("ConvertTo")
            .unwrap();
        
        convert_to.0.set_value("MUIVerb", &"Convert to").unwrap();
        convert_to.0.set_value("SubCommands", &"").unwrap();

        for sub_format in format.supported_convertable_formats() {
            let sub_key = convert_to.0
                .create_subkey_with_flags(format!("shell\\{}", sub_format), KEY_WRITE)
                .unwrap();

            sub_key.0.set_value("MUIVerb", &format!("Convert to {}", sub_format.to_string())).unwrap();
            
            let command_string = format!("\"{}\" --file \"%1\" --convert {}", self.program_path.display(), sub_format.to_string());
            
            let command = sub_key.0
                .create_subkey("command")
                .unwrap();
            
            command.0.set_value("", &command_string).unwrap();
        }
    }
}