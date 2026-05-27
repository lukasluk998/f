use std::env;
use std::fs;
use std::path::Path;

/// Build script for randomizing executable name
/// 
/// This prevents signature-based detection by process name.
/// Each build generates a different random executable name.
/// 
/// Example names:
/// - win_service_4a3f2b1c.exe
/// - system_update_7e9d2f8a.exe
/// - discord_helper_1b4c9e2a.exe
/// 
/// EAC cannot blacklist process names if they're different every build.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    
    // Generate random process name
    let random_name = generate_random_process_name();
    
    println!("cargo:rustc-env=RANDOM_PROCESS_NAME={}", random_name);
    println!("cargo:warning=Randomized process name: {}", random_name);
    
    // After build, we'll rename the executable
    // This happens in a post-build step (see rename_executable function)
}

fn generate_random_process_name() -> String {
    // Use build timestamp as seed for deterministic builds
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    // Simple LCG random number generator (deterministic per build)
    let mut seed = timestamp;
    let random = |s: &mut u64| -> u32 {
        *s = s.wrapping_mul(1664525).wrapping_add(1013904223);
        (*s >> 32) as u32
    };
    
    // List of innocent-sounding prefixes
    let prefixes = [
        "win_service",
        "system_update",
        "discord_helper",
        "nvidia_telemetry",
        "steam_overlay",
        "obs_helper",
        "driver_check",
        "audio_service",
        "network_monitor",
        "game_overlay",
        "security_update",
        "windows_defender",
        "chrome_helper",
        "explorer_extension",
        "antimalware_service",
    ];
    
    let prefix_idx = (random(&mut seed) as usize) % prefixes.len();
    let prefix = prefixes[prefix_idx];
    
    // Generate random hex suffix
    let suffix = format!("{:08x}", random(&mut seed));
    
    format!("{}_{}.exe", prefix, suffix)
}

// Note: The actual executable rename needs to happen post-build
// We can't do it here because the .exe doesn't exist yet during build.rs
// 
// To rename after build, add this to a shell script or do it manually:
// 
// Windows (PowerShell):
//   $name = cargo build --release 2>&1 | Select-String "Randomized process name:" | % { $_.ToString().Split(": ")[1] }
//   Move-Item target/release/rust-game-cheat.exe "target/release/$name"
// 
// Or use the included rename_build.sh / rename_build.ps1 scripts
