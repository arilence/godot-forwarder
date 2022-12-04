use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};

mod config;

fn main() {
    println!("Welcome to Godot Forwarder");
    let config_location = config::AppConfig::get_path();
    let config = config::AppConfig::load_or_create(config_location);

    print_command_arguments();

    // Only open Godot if we're executing from the steamapps folder
    if is_steamapps(&config.steamapps_directory) {
        println!("Steamapps detected\nOpening Godot Engine");
        open_godot(&config.steamapps_directory);
        return;
    }

    // If we're executing from anywhere else, install us into the steamapps folder
    println!("Not executing from Steamapps\nInstalling now");
    install(&config.steamapps_directory);
}

fn print_command_arguments() {
    let args: Vec<String> = env::args().collect();
    println!("Command line arguments: {:?}", args);
}

fn is_steamapps(steamapps_directory: &PathBuf) -> bool {
    let mut godot_exe = steamapps_directory.clone();
    godot_exe.push(r"common\Godot Engine\godot.windows.opt.tools.64.exe");

    // Becomes an empty path if the executable does not exist
    godot_exe = godot_exe.canonicalize().unwrap_or_default();

    let current_exe = env::current_exe()
        .unwrap_or_default()
        .canonicalize()
        .unwrap_or_default();

    if current_exe == godot_exe {
        return true;
    }

    return false;
}

// Windows: common\Godot Engine\godot.windows.opt.tools.64.exe
// MacOS: common/Godot Engine/Godot.app/Contents/MacOS/Godot
fn install(steamapps_directory: &PathBuf) {
    let mut godot_directory = steamapps_directory.clone();
    godot_directory.push(r"common\Godot Engine");

    let current_exe = env::current_exe().unwrap_or_default();

    let mut main_exe = godot_directory.clone();
    main_exe.push("godot.windows.opt.tools.64.exe");

    let mut sub_exe = godot_directory.clone();
    sub_exe.push("original_godot.windows.opt.tools.64.exe");

    let file_size = fs::metadata(&main_exe).expect("Error finding godot.windows.opt.tools.64.exe");

    // Naive way to determine if Godot Forwarder has already been installed
    // The assumption here is that Godot will never be less than 1 MB and Godot Forwarder will never be more than 1 MB
    // 1 MB
    if file_size.len() < 1000000 {
        panic!("Godot Forwarder is already installed");
    }

    // This will OVERWRITE any files that already exist
    fs::rename(&main_exe, sub_exe).expect("Could not rename godot exe to original_godot exe");
    fs::copy(current_exe, &main_exe).expect("Could not copy Godot Forwarder to steamapps");
}

fn open_godot(steamapps_directory: &PathBuf) {
    let mut godot_exe = steamapps_directory.clone();
    godot_exe.push(r"common\Godot Engine\original_godot.windows.opt.tools.64.exe");

    if !godot_exe.exists() {
        panic!("original_godot.windows.opt.tools.64.exe does not exist");
    }

    // Returns an empty path if the executable does not exist
    godot_exe = godot_exe.canonicalize().unwrap_or_default();

    let godot_exe_string = godot_exe.into_os_string().into_string().unwrap();

    // TODO: Get powershell location instead of assuming powershell 7
    // We should use an absolute location instead of simply `pwsh.exe` to avoid potential malicious behaviour
    let output = Command::new(r"C:\Program Files\PowerShell\7\pwsh.exe")
        .args([
            "-Command",
            "Start-Process",
            "-FilePath",
            &format!("\"{}\"", godot_exe_string),
            "-NoNewWindow",
            "-Wait",
            "-ArgumentList",
            // TODO: Read launch options from steam rather than hardcoding here
            "\"--path $($env:userprofile) -p\"",
        ])
        .output()
        .expect("Failed to execute child process");

    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}
