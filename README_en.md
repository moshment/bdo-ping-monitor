# BDO Ping Monitor

[日本語 (Japanese)](README.md) | [English](README_en.md)

![Screenshot](ss.png)

An unofficial application to measure and visualize network latency (TCP ping) to specific PC Black Desert Online (BDO) servers in real-time.
It was originally created to objectively verify if specific PvP channels (like the Japanese "Marni-2ch" server) are actually experiencing lag.

> **⚠️ IMPORTANT FOR GLOBAL USERS**
> This tool was originally developed for the Japanese community. The default main server is hardcoded to the **Japanese Marni-2ch server**. 
> If you are playing in other regions (NA/EU, KR, SEA, etc.), please use the `servers.txt` feature to add your region's server IPs, or modify the source code yourself to change the default server and rebuild the app.

## Features
- **Real-time Monitoring:** Performs a TCP connection test approximately every 5 seconds and displays the ping history on a chart.
- **Lightweight & Portable:** Built with Rust & Tauri. It runs as a standalone `.exe` without any installation.
- **Custom Servers:** You can add any server IP as a secondary server by editing the included `servers.txt`.

## Usage (For General Users)

Download the latest `BDO-Ping-Monitor-Portable.zip` from the [Releases](../../releases) page and extract it. 
Just run `bdo-ping-monitor.exe`. No installation is required.

### Adding Custom Servers (`servers.txt`)
You can add your region's servers by editing `servers.txt` located in the same folder as the executable.

**Example:**
```text
# BDO Custom Server List
# Format: ServerName:IP_Address
NA-Valencia2: 12.34.56.78
EU-Valencia2: 98.76.54.32
```
After saving the file and restarting the app, you can select your newly added server from the dropdown menu.

## For Developers (Build from Source)

This project is built using Rust and Tauri (v2). 
If you want to change the default hardcoded main server, please edit the `get_servers` function in `src-tauri/src/main.rs`.

### Prerequisites
- [Rust / Cargo](https://rustup.rs/) installed.
- [Tauri v2 prerequisites](https://v2.tauri.app/start/prerequisites/) (Visual Studio C++ Build Tools, etc.).

### Build Steps
1. Clone the repository.
2. Navigate to the `src-tauri` directory:
```bash
cd src-tauri
```
3. Run the release build:
```bash
cargo build --release
```
4. The executable will be generated at `src-tauri/target/release/bdo-ping-monitor.exe`.

## Disclaimer

This software is provided "As-Is" without any warranties, express or implied.
The author is not responsible for any direct or indirect damages, loss of data, PC issues, game account bans/restrictions, or any other troubles caused by using or being unable to use this software.
Use entirely at your **own risk**.

## Transparency
To avoid false positives from antivirus software and to assure users that this is not a malicious tool (such as a keylogger or account stealer), the entire source code is public.

## License
[MIT License](LICENSE)
