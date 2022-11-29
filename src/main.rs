use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};

fn main() {
    // Only open Godot if we're executing from the steamapps folder
    if is_steamapps() {
        open_godot();
        return;
    }

    // If we're executing from anywhere else, install us into the steamapps folder
    install();
}

fn is_steamapps() -> bool {
    let current_exe = env::current_exe()
        .unwrap_or_default()
        .canonicalize()
        .unwrap_or_default();

    let steamapps_exe = PathBuf::from(
        r"C:\Program Files (x86)\Steam\steamapps\common\Godot Engine\godot.windows.opt.tools.64.exe",
    ).canonicalize().unwrap_or_default();

    if current_exe == steamapps_exe {
        return true;
    }

    return false;
}

fn install() {
    let steamapps_path =
        PathBuf::from(r"C:\Program Files (x86)\Steam\steamapps\common\Godot Engine");

    let current_exe = env::current_exe().unwrap_or_default();

    let mut main_exe = steamapps_path.clone();
    main_exe.push("godot.windows.opt.tools.64.exe");

    let mut sub_exe = steamapps_path.clone();
    sub_exe.push("original_godot.windows.opt.tools.64.exe");

    let file_size = fs::metadata(&main_exe).expect("Error finding main_exe");

    // Naive way to determine if Godot Forwarder has already been installed
    // The assumption here is that Godot will never be less than 1 MB and Godot Forwarder will never be more than 1 MB
    // 1 MB
    if file_size.len() < 1000000 {
        panic!("Godot Forwarder is already installed");
    }

    fs::rename(&main_exe, sub_exe).expect("Could not rename godot exe to original_godot exe");
    fs::copy(current_exe, &main_exe).expect("Could not copy Godot Forwarder to steamapps");
}

fn open_godot() {
    let godot_exe = PathBuf::from(
        r"C:\Program Files (x86)\Steam\steamapps\common\Godot Engine\original_godot.windows.opt.tools.64.exe",
    );

    let godot_exe_string = godot_exe.into_os_string().into_string().unwrap();

    let output = Command::new(r"C:\Program Files\PowerShell\7\pwsh.exe")
        .args([
            "-Command",
            "Start-Process",
            // Use debug formatting to enclose the path with quotes
            &format!("{:?}", godot_exe_string),
            "-NoNewWindow",
            "-Wait",
            "-ArgumentList",
            "'-p'",
        ])
        .output()
        .expect("Failed to execute child process");

    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}
