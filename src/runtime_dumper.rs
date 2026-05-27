// Runtime IL2CPP Offset Dumper
// Automatically finds and updates offsets from running Rust game process
// No need for manual Il2CppDumper - works at runtime

use crate::memory::Process;
use crate::scanner::PatternScanner;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DumpedOffsets {
    pub timestamp: String,
    pub game_version: String,
    pub base_addresses: HashMap<String, usize>,
    pub base_player: HashMap<String, usize>,
    pub player_input: HashMap<String, usize>,
    pub player_model: HashMap<String, usize>,
    pub base_networkable: HashMap<String, usize>,
}

pub struct RuntimeDumper<'a> {
    process: &'a Process,
    scanner: PatternScanner<'a>,
    game_assembly_base: usize,
}

impl<'a> RuntimeDumper<'a> {
    pub fn new(process: &'a Process, game_assembly_base: usize) -> Self {
        let scanner = PatternScanner::new(process);
        Self {
            process,
            scanner,
            game_assembly_base,
        }
    }

    /// Full automated offset dump - finds everything you need
    pub fn dump_all_offsets(&self) -> Option<DumpedOffsets> {
        println!("\n[*] ═══════════════════════════════════════");
        println!("[*] RUNTIME OFFSET DUMPER v2.0");
        println!("[*] ═══════════════════════════════════════\n");

        let mut dump = DumpedOffsets {
            timestamp: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            game_version: self.get_game_version(),
            base_addresses: HashMap::new(),
            base_player: HashMap::new(),
            player_input: HashMap::new(),
            player_model: HashMap::new(),
            base_networkable: HashMap::new(),
        };

        // Step 1: Find critical base addresses
        println!("[1/5] Finding base addresses...");
        self.find_base_addresses(&mut dump);

        // Step 2: Analyze BasePlayer class
        println!("[2/5] Analyzing BasePlayer structure...");
        self.analyze_base_player(&mut dump);

        // Step 3: Analyze PlayerInput structure
        println!("[3/5] Analyzing PlayerInput structure...");
        self.analyze_player_input(&mut dump);

        // Step 4: Analyze PlayerModel structure
        println!("[4/5] Analyzing PlayerModel structure...");
        self.analyze_player_model(&mut dump);

        // Step 5: Analyze networking structures
        println!("[5/5] Analyzing networking structures...");
        self.analyze_networking(&mut dump);

        println!("\n[+] ═══════════════════════════════════════");
        println!("[+] DUMP COMPLETED SUCCESSFULLY");
        println!("[+] ═══════════════════════════════════════\n");

        Some(dump)
    }

    /// Find LocalPlayer singleton pattern
    fn find_base_addresses(&self, dump: &mut DumpedOffsets) {
        // LocalPlayer pattern - this is universal for Unity IL2CPP
        // Pattern: MOV rcx, [rip+offset] ; TEST rcx, rcx ; JZ short
        let patterns = vec![
            ("LocalPlayer", "48 8B 0D ? ? ? ? 48 85 C9 74 ? 48 8B 49"),
            ("LocalPlayer_Alt", "48 8B 0D ? ? ? ? 48 85 C9 0F 84"),
            ("ClientEntities", "48 8B 05 ? ? ? ? 48 8B 88 ? ? ? ? 48 85 C9"),
            ("GameObjectManager", "48 8B 0D ? ? ? ? 48 8B 01 48 8B 40"),
        ];

        for (name, pattern) in patterns {
            if let Some(addr) = self.scanner.scan_pattern(
                self.game_assembly_base,
                0x5000000,
                pattern
            ) {
                // Resolve RIP-relative pointer
                if let Ok(resolved) = self.scanner.resolve_rip_relative(addr, 3, 7) {
                    println!("  [+] {}: 0x{:X} (resolved: 0x{:X})", name, addr - self.game_assembly_base, resolved - self.game_assembly_base);
                    dump.base_addresses.insert(name.to_string(), resolved - self.game_assembly_base);
                }
            }
        }

        // If we found LocalPlayer, read it to get actual instance
        if let Some(&local_player_offset) = dump.base_addresses.get("LocalPlayer") {
            if let Ok(local_player_addr) = self.process.read::<usize>(self.game_assembly_base + local_player_offset) {
                if local_player_addr != 0 {
                    println!("  [+] LocalPlayer instance: 0x{:X}", local_player_addr);
                    dump.base_addresses.insert("LocalPlayer_Instance".to_string(), local_player_addr);
                }
            }
        }
    }

    /// Analyze BasePlayer class structure
    fn analyze_base_player(&self, dump: &mut DumpedOffsets) {
        // Get LocalPlayer instance to analyze
        let local_player = match dump.base_addresses.get("LocalPlayer_Instance") {
            Some(&addr) => addr,
            None => {
                println!("  [!] LocalPlayer not found, skipping BasePlayer analysis");
                return;
            }
        };

        println!("  [*] Analyzing BasePlayer at 0x{:X}", local_player);

        // Common BasePlayer offsets (these are fairly stable in Unity)
        let candidates = vec![
            ("health", vec![0x1E0, 0x1F0, 0x200, 0x210]),
            ("maxHealth", vec![0x1E4, 0x1F4, 0x204, 0x214]),
            ("lifestate", vec![0x220, 0x224, 0x228, 0x230]),
            ("playerModel", vec![0x4A0, 0x4A8, 0x4B0, 0x4B8]),
            ("playerInput", vec![0x4C8, 0x4D0, 0x4D8, 0x4E0]),
            ("playerInventory", vec![0x600, 0x608, 0x610, 0x618]),
            ("modelState", vec![0x560, 0x568, 0x570, 0x578]),
        ];

        for (field_name, offsets) in candidates {
            for &offset in &offsets {
                // Try to read the pointer/value
                if let Ok(value) = self.process.read::<usize>(local_player + offset) {
                    // Heuristics to determine if it's valid
                    let is_valid = match field_name {
                        "health" | "maxHealth" => {
                            // Should be float between 0-100
                            if let Ok(hp) = self.process.read::<f32>(local_player + offset) {
                                hp >= 0.0 && hp <= 200.0
                            } else {
                                false
                            }
                        },
                        "playerModel" | "playerInput" | "playerInventory" => {
                            // Should be pointer in valid range
                            value > 0x10000 && value < 0x7FFFFFFFFFFF
                        },
                        _ => value > 0 && value < 0x1000,
                    };

                    if is_valid {
                        println!("  [+] {}: 0x{:X} (value: 0x{:X})", field_name, offset, value);
                        dump.base_player.insert(field_name.to_string(), offset);
                        break;
                    }
                }
            }
        }
    }

    /// Analyze PlayerInput structure
    fn analyze_player_input(&self, dump: &mut DumpedOffsets) {
        // Get PlayerInput from LocalPlayer
        let local_player = match dump.base_addresses.get("LocalPlayer_Instance") {
            Some(&addr) => addr,
            None => return,
        };

        let player_input_offset = match dump.base_player.get("playerInput") {
            Some(&off) => off,
            None => return,
        };

        if let Ok(player_input_addr) = self.process.read::<usize>(local_player + player_input_offset) {
            if player_input_addr == 0 {
                return;
            }

            println!("  [*] Analyzing PlayerInput at 0x{:X}", player_input_addr);

            // PlayerInput typically has angle fields at start
            let candidates = vec![
                ("bodyAngles", vec![0x30, 0x34, 0x38, 0x3C, 0x40]),
                ("viewAngles", vec![0x3C, 0x40, 0x44, 0x48, 0x4C]),
                ("recoilAngles", vec![0x44, 0x48, 0x4C, 0x50, 0x54]),
            ];

            for (field_name, offsets) in candidates {
                for &offset in &offsets {
                    // Angles should be Vec2 (x, y as floats)
                    if let Ok(angle_x) = self.process.read::<f32>(player_input_addr + offset) {
                        if let Ok(angle_y) = self.process.read::<f32>(player_input_addr + offset + 4) {
                            // Valid angles are typically -360 to 360
                            if angle_x.abs() < 360.0 && angle_y.abs() < 360.0 {
                                println!("  [+] {}: 0x{:X} (angles: {:.2}, {:.2})", field_name, offset, angle_x, angle_y);
                                dump.player_input.insert(field_name.to_string(), offset);
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    /// Analyze PlayerModel structure
    fn analyze_player_model(&self, dump: &mut DumpedOffsets) {
        let local_player = match dump.base_addresses.get("LocalPlayer_Instance") {
            Some(&addr) => addr,
            None => return,
        };

        let player_model_offset = match dump.base_player.get("playerModel") {
            Some(&off) => off,
            None => return,
        };

        if let Ok(player_model_addr) = self.process.read::<usize>(local_player + player_model_offset) {
            if player_model_addr == 0 {
                return;
            }

            println!("  [*] Analyzing PlayerModel at 0x{:X}", player_model_addr);

            let candidates = vec![
                ("newVelocity", vec![0x1C0, 0x1C4, 0x1D0, 0x1D4, 0x1E0]),
                ("transform", vec![0x28, 0x30, 0x38]),
                ("visualState", vec![0x30, 0x38, 0x40]),
            ];

            for (field_name, offsets) in candidates {
                for &offset in &offsets {
                    if let Ok(value) = self.process.read::<usize>(player_model_addr + offset) {
                        let is_valid = match field_name {
                            "newVelocity" => {
                                // Vec3, velocity typically < 100
                                if let Ok(vel) = self.process.read::<f32>(player_model_addr + offset) {
                                    vel.abs() < 100.0
                                } else {
                                    false
                                }
                            },
                            _ => value > 0x10000 && value < 0x7FFFFFFFFFFF,
                        };

                        if is_valid {
                            println!("  [+] {}: 0x{:X} (value: 0x{:X})", field_name, offset, value);
                            dump.player_model.insert(field_name.to_string(), offset);
                            break;
                        }
                    }
                }
            }
        }
    }

    /// Analyze networking structures
    fn analyze_networking(&self, dump: &mut DumpedOffsets) {
        // BaseNetworkable pattern
        let pattern = "48 8B 05 ? ? ? ? 48 8B 88 ? ? ? ? 48 85 C9";
        
        if let Some(addr) = self.scanner.scan_pattern(
            self.game_assembly_base,
            0x5000000,
            pattern
        ) {
            if let Ok(resolved) = self.scanner.resolve_rip_relative(addr, 3, 7) {
                println!("  [+] BaseNetworkable: 0x{:X}", resolved - self.game_assembly_base);
                dump.base_networkable.insert("base".to_string(), resolved - self.game_assembly_base);
            }
        }
    }

    /// Get game version from binary
    fn get_game_version(&self) -> String {
        // Try to find version string in memory
        let version_patterns = vec![
            "rust.",
            "client_",
        ];

        for pattern in version_patterns {
            if let Some(addr) = self.scanner.scan_string(
                self.game_assembly_base,
                0x1000000,
                pattern
            ) {
                if let Ok(version_bytes) = self.process.read_bytes(addr, 32) {
                    if let Ok(version_str) = std::str::from_utf8(&version_bytes) {
                        let version = version_str.split('\0').next().unwrap_or("unknown");
                        return version.to_string();
                    }
                }
            }
        }

        "unknown".to_string()
    }

    /// Save dump to JSON file
    pub fn save_to_file(&self, dump: &DumpedOffsets, filename: &str) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(dump)?;
        fs::write(filename, json)?;
        println!("[+] Saved offsets to: {}", filename);
        Ok(())
    }

    /// Generate Rust code from dump
    pub fn generate_rust_code(&self, dump: &DumpedOffsets) -> String {
        let mut code = String::new();
        
        code.push_str(&format!("// Auto-generated offsets - {}\n", dump.timestamp));
        code.push_str(&format!("// Game version: {}\n\n", dump.game_version));
        
        code.push_str("pub struct RustOffsets {\n");
        
        // Base addresses
        code.push_str("    // Base addresses\n");
        for (name, offset) in &dump.base_addresses {
            code.push_str(&format!("    pub {}: usize,  // 0x{:X}\n", 
                name.to_lowercase().replace(' ', "_"), offset));
        }
        
        // BasePlayer
        code.push_str("\n    // BasePlayer offsets\n");
        for (name, offset) in &dump.base_player {
            code.push_str(&format!("    pub {}: usize,  // 0x{:X}\n", 
                name.to_lowercase().replace(' ', "_"), offset));
        }
        
        // PlayerInput
        code.push_str("\n    // PlayerInput offsets\n");
        for (name, offset) in &dump.player_input {
            code.push_str(&format!("    pub {}: usize,  // 0x{:X}\n", 
                name.to_lowercase().replace(' ', "_"), offset));
        }
        
        // PlayerModel
        code.push_str("\n    // PlayerModel offsets\n");
        for (name, offset) in &dump.player_model {
            code.push_str(&format!("    pub {}: usize,  // 0x{:X}\n", 
                name.to_lowercase().replace(' ', "_"), offset));
        }
        
        code.push_str("}\n\n");
        
        // Generate impl new()
        code.push_str("impl RustOffsets {\n");
        code.push_str("    pub fn new() -> Self {\n");
        code.push_str("        Self {\n");
        
        for (name, offset) in &dump.base_addresses {
            code.push_str(&format!("            {}: 0x{:X},\n", 
                name.to_lowercase().replace(' ', "_"), offset));
        }
        for (name, offset) in &dump.base_player {
            code.push_str(&format!("            {}: 0x{:X},\n", 
                name.to_lowercase().replace(' ', "_"), offset));
        }
        for (name, offset) in &dump.player_input {
            code.push_str(&format!("            {}: 0x{:X},\n", 
                name.to_lowercase().replace(' ', "_"), offset));
        }
        for (name, offset) in &dump.player_model {
            code.push_str(&format!("            {}: 0x{:X},\n", 
                name.to_lowercase().replace(' ', "_"), offset));
        }
        
        code.push_str("        }\n");
        code.push_str("    }\n");
        code.push_str("}\n");
        
        code
    }
}
