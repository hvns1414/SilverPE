#![allow(non_snake_case)]

mod loader;
mod logger;
mod pe_structures;

use loader::{X64PeLoader, X86PeLoader};
use logger::{log_error, log_info, log_ok};
use std::env;
use std::fs;

use crate::loader::load_x64;
use crate::loader::load_x86;

const PINK: &str = "\x1b[95m";
const RESET: &str = "\x1b[0m";

const BANNER: &str = r#"
  _________.__  .__                  _____________________
 /   _____/|  | |__|__  __ __________\______   \_   _____/
 \_____  \ |  | |  \  \/ // __ \_  __ \     ___/|    __)_ 
 /        \|  |_|  |\   /\  ___/|  | \/    |    |        \
/_______  /|____/__| \_/  \___  >__|  |____|   /_______  /
        \/                    \/                       \/ 
      (SilverPE - Advanced Loader)
"#;

const DESCRIPTION: &str = "Author: Sh3d0w\nGitHub: https://github.com/hvns1414/SilverPE";

const USAGE: &str = "Usage:
\tSilverPE.exe --x86 <FilePath>
\tSilverPE.exe --x64 <FilePath>
\tSilverPE.exe --fighter
";

const FIGHTER: &str = r#"
   \    /
_____-/\-_____                             \    /
    \_\/_/                              _____-/\-_____
                                            \_\/_/
                
  
                  \            /
                   \    __    /
        ____________\.-|__|-./____________
            + + ---\__| \/ |__/--- + +
                       \__/
"#;

fn main() {
    #[cfg(windows)]
    {
        use windows::Win32::System::Console::{
            GetConsoleMode, GetStdHandle, SetConsoleMode,
            ENABLE_VIRTUAL_TERMINAL_PROCESSING, STD_OUTPUT_HANDLE,
        };
        unsafe {
            let handle = GetStdHandle(STD_OUTPUT_HANDLE).unwrap();
            let mut mode = windows::Win32::System::Console::CONSOLE_MODE(0);
            let _ = GetConsoleMode(handle, &mut mode);
            let _ = SetConsoleMode(handle, mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
        }
    }

    println!("{}", BANNER);
    println!("{}", DESCRIPTION);

    let arch = if cfg!(target_pointer_width = "64") { "x64" } else { "x86" };
    log_info(&format!("The current process architecture is: {}", arch));

    let args: Vec<String> = env::args().collect();

    let valid = args.len() >= 3 || (args.len() == 2 && args[1] == "--fighter");
    if !valid {
        println!("{}", USAGE);
        return;
    }

    if let Err(e) = run(&args) {
        log_error(&e);
    }
}

fn run(args: &[String]) -> Result<(), String> {
    match args[1].as_str() {
        "--fighter" => {
            println!("{}", FIGHTER);
            Ok(())
        }
        "--x86" => {
            log_info("Action => x86 loading...");
            if cfg!(target_pointer_width = "64") {
                return Err("Current process is x64, cannot load an x86 PE from a 64-bit process.".to_string());
            }
            let path = &args[2];
            let bytes = read_file(path)?;
            let pe = X86PeLoader::new(bytes)?;
            if !pe.is_32bit() {
                return Err("This is not an x86 (PE32) file.".to_string());
            }
            log_ok(&format!("Image base = {:#X}", pe.optional_header.image_base));
            load_x86(&pe)
        }
        "--x64" => {
            log_info("Action => x64 loading...");
            if cfg!(target_pointer_width = "32") {
                return Err("Current process is x86, cannot load an x64 PE from a 32-bit process.".to_string());
            }
            let path = &args[2];
            let bytes = read_file(path)?;
            let pe = X64PeLoader::new(bytes)?;
            if pe.is_32bit_header() {
                return Err("This is not an x64 PE file.".to_string());
            }
            log_ok(&format!("Image base = {:#X}", pe.optional_header64.image_base));
            load_x64(&pe)
        }
        other => Err(format!("Unknown command: {}", other)),
    }
}

fn read_file(path: &str) -> Result<Vec<u8>, String> {
    if !std::path::Path::new(path).exists() {
        return Err(format!("File not found: {}", path));
    }
    let bytes = fs::read(path).map_err(|e| format!("Failed to read file '{}': {}", path, e))?;
    log_ok(&format!("Read file successfully. Length: {}", bytes.len()));
    Ok(bytes)
}
