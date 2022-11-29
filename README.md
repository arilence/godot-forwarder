# Godot Forwarder

Keeps the Godot process alive when launching through Steam. This allows Steam to track hours spent inside projects.

By default, Steam does not follow the new process when a project is opened via Godot's project manager. This means that Steam only tracks when the project manager is open and thinks Godot has closed once a project has been opened.

This workaround puts itself between Steam and the official Godot executable to follow the new process.

## Notes

-   Currently only works on Windows
-   Godot Forwarder must be reinstalled anytime Steam updates the Godot executable
-   Due to this program launching another executable, antivirus software may flag it as malicious

## Quick Start

1. Install Godot through Steam

2. Install Godot Forwarder

    ```
    cargo run --release
    ```

3. Launch Godot through Steam
