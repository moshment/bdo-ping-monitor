// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::time::Instant;
use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct PingResult {
    success: bool,
    ping: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone)]
struct ServerConfig {
    name: String,
    ip: String,
    port: u16,
}

#[tauri::command]
async fn measure_ping(ip: String, port: u16) -> PingResult {
    let addr = format!("{}:{}", ip, port);
    let start = Instant::now();

    match timeout(Duration::from_secs(3), TcpStream::connect(&addr)).await {
        Ok(Ok(_)) => {
            let elapsed = start.elapsed();
            let ping_ms = elapsed.as_secs_f64() * 1000.0;
            PingResult {
                success: true,
                ping: Some(ping_ms),
            }
        }
        Ok(Err(_)) | Err(_) => PingResult {
            success: false,
            ping: None,
        },
    }
}

#[tauri::command]
fn get_servers(_app_handle: tauri::AppHandle) -> Vec<ServerConfig> {
    let mut servers = Vec::new();

    // Try to read servers.txt from the same directory as the executable
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let txt_path = exe_dir.join("servers.txt");
            if let Ok(contents) = fs::read_to_string(&txt_path) {
                for line in contents.lines() {
                    let line = line.trim();
                    if line.is_empty() || line.starts_with('#') {
                        continue;
                    }
                    
                    let parts: Vec<&str> = line.split(':').collect();
                    if parts.len() == 2 {
                        servers.push(ServerConfig {
                            name: parts[0].trim().to_string(),
                            ip: parts[1].trim().to_string(),
                            port: 8889, // Default port, modify if you want to specify port in txt
                        });
                    }
                }
            }
        }
    }
    
    // Fallback if servers.txt is missing or empty
    if servers.is_empty() {
        servers.push(ServerConfig {
            name: "malni-2ch".to_string(),
            ip: "20.222.139.220".to_string(),
            port: 8889,
        });
    }

    servers
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![measure_ping, get_servers])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
