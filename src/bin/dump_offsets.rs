// Standalone offset dumper tool
// Run this anytime Rust game updates to automatically dump new offsets
// 
// Usage:
//   cargo run --bin dump_offsets
//
// This will:
//   1. Attach to RustClient.exe
//   2. Scan for all offsets automatically
//   3. Save to resources/offsets/YYYY-MM-DD/
//   4. Generate updated offsets.rs code
//   5. Optionally commit to git

use std::thread;
use std::time::Duration;
use std::fs;
use std::path::Path;

// Import from main crate
use f::memory::Process;
use f::runtime_dumper::RuntimeDumper;

fn main() {
    println!("╔══════════════════════════════════════════════════╗");
    println!("║   RUST GAME OFFSET DUMPER                        ║");
    println!("║   Auto-updates offsets after game patches       ║");
    println!("╚══════════════════════════════════════════════════╝");
    println!();

    // Wait for game
    println!("[*] Waiting for RustClient.exe...");
    println!("    (Start Rust and join a server)");
    
    let process = loop {
        if let Some(proc) = Process::from_name("RustClient.exe") {
            println!("[+] Found RustClient.exe (PID: {})", proc.pid);
            break proc;
        }
        thread::sleep(Duration::from_secs(2));
    };

    // Get GameAssembly.dll base
    let game_assembly_base = match process.get_module_base("GameAssembly.dll") {
        Some(base) => {
            println!("[+] GameAssembly.dll: 0x{:X}", base);
            base
        },
        None => {
            println!("[-] Could not find GameAssembly.dll");
            return;
        }
    };

    // Wait for game to fully load (join server, spawn in)
    println!();
    println!("[!] IMPORTANT: Join a server and spawn in as a player!");
    println!("[!] Press ENTER when you're in-game and spawned...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok();

    // Create dumper
    let dumper = RuntimeDumper::new(&process, game_assembly_base);

    // Dump all offsets
    println!("\n[*] Starting automated offset dump...\n");
    let dump = match dumper.dump_all_offsets() {
        Some(d) => d,
        None => {
            println!("[-] Failed to dump offsets");
            return;
        }
    };

    // Create output directory structure
    let date = chrono::Local::now().format("%Y-%m-%d").to_string();
    let output_dir = format!("../resources/offsets/{}", date);
    
    if !Path::new("../resources").exists() {
        println!("[!] Resources repo not found at ../resources");
        println!("[*] Creating local output directory: ./dump_output/");
        fs::create_dir_all("./dump_output/offsets").ok();
        
        // Save locally
        let json_file = format!("./dump_output/offsets/{}_offsets.json", date);
        let rust_file = format!("./dump_output/offsets/{}_offsets.rs", date);
        
        dumper.save_to_file(&dump, &json_file).ok();
        
        let rust_code = dumper.generate_rust_code(&dump);
        fs::write(&rust_file, rust_code).ok();
        
        println!("\n[+] ═══════════════════════════════════════");
        println!("[+] DUMP SAVED LOCALLY");
        println!("[+] ═══════════════════════════════════════");
        println!("[+] JSON: {}", json_file);
        println!("[+] Rust: {}", rust_file);
        println!();
        println!("[*] Copy the generated Rust code to src/offsets.rs");
        return;
    }

    // Save to resources repo
    fs::create_dir_all(&output_dir).ok();
    
    let json_file = format!("{}/offsets.json", output_dir);
    let rust_file = format!("{}/offsets.rs", output_dir);
    let notes_file = format!("{}/notes.md", output_dir);
    
    // Save JSON dump
    dumper.save_to_file(&dump, &json_file).ok();
    
    // Save Rust code
    let rust_code = dumper.generate_rust_code(&dump);
    fs::write(&rust_file, &rust_code).ok();
    
    // Generate notes
    let notes = format!(
        "# Offset Dump - {}\n\n\
        ## Game Version\n{}\n\n\
        ## Base Addresses Found\n{}\n\n\
        ## BasePlayer Offsets\n{}\n\n\
        ## PlayerInput Offsets\n{}\n\n\
        ## PlayerModel Offsets\n{}\n\n\
        ## Usage\n\
        1. Copy `offsets.rs` to main cheat `src/offsets.rs`\n\
        2. Rebuild cheat: `cargo build --release`\n\
        3. Test in-game\n\n\
        ## Detection Notes\n\
        - These offsets are valid for game version: {}\n\
        - Pattern-based scanning is more resistant to patches\n\
        - Re-dump after each game update\n",
        dump.timestamp,
        dump.game_version,
        dump.base_addresses.len(),
        dump.base_player.len(),
        dump.player_input.len(),
        dump.player_model.len(),
        dump.game_version
    );
    fs::write(&notes_file, notes).ok();
    
    // Copy to latest.json
    let latest_file = "../resources/offsets/latest.json";
    fs::copy(&json_file, latest_file).ok();
    
    // Copy to src/offsets.rs (with backup)
    if Path::new("src/offsets.rs").exists() {
        let backup = format!("src/offsets.rs.backup_{}", date);
        fs::copy("src/offsets.rs", backup).ok();
    }
    fs::write("src/offsets.rs", &rust_code).ok();

    println!("\n[+] ═══════════════════════════════════════");
    println!("[+] DUMP COMPLETED SUCCESSFULLY");
    println!("[+] ═══════════════════════════════════════");
    println!("[+] JSON:   {}", json_file);
    println!("[+] Rust:   {}", rust_file);
    println!("[+] Notes:  {}", notes_file);
    println!("[+] Latest: {}", latest_file);
    println!("[+] Updated: src/offsets.rs (old backed up)");
    println!();

    // Ask about git commit
    println!("[?] Commit to resources git repo? (yes/no)");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok();
    
    if input.trim().to_lowercase() == "yes" {
        println!("\n[*] Committing to git...");
        
        use std::process::Command;
        
        // Git add
        Command::new("git")
            .current_dir("../resources")
            .args(&["add", "."])
            .output()
            .ok();
        
        // Git commit
        let commit_msg = format!("Update offsets for {}", date);
        Command::new("git")
            .current_dir("../resources")
            .args(&["commit", "-m", &commit_msg])
            .output()
            .ok();
        
        // Git push
        Command::new("git")
            .current_dir("../resources")
            .args(&["push"])
            .output()
            .ok();
        
        println!("[+] Committed and pushed to resources repo");
    }
    
    println!("\n[+] Done! Rebuild cheat with: cargo build --release");
}
