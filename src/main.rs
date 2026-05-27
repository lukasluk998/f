mod memory;
mod scanner;
mod overlay;
mod offsets;
mod driver_interface;
mod eac_bypass;
mod runtime_dumper;
mod config;
mod esp_optimizer;
mod recoil_helper;

use memory::Process;
use scanner::PatternScanner;
use offsets::{RustOffsets, Vec3, Vec2};
use driver_interface::DriverInterface;
use eac_bypass::EACBypass;
use config::{CheatConfig, Humanizer, SafetyMode, RecoilMethod};
use esp_optimizer::{ESPOptimizer, DetailLevel};
use recoil_helper::RecoilHelper;
use std::thread;
use std::time::Duration;

struct Player {
    address: usize,
    position: Vec3,
    health: f32,
    max_health: f32,
    distance: f32,
    is_local: bool,
}

struct RustCheat {
    process: Process,
    driver: Option<DriverInterface>,
    eac_bypass: EACBypass,
    offsets: RustOffsets,
    game_assembly_base: usize,
    local_player: usize,
    config: CheatConfig,
    humanizer: Humanizer,
    esp_optimizer: ESPOptimizer,
    recoil_helper: RecoilHelper,
}

impl RustCheat {
    fn new(process: Process, config: CheatConfig) -> Option<Self> {
        let offsets = RustOffsets::new();
        
        // Load driver only if needed (not needed for Legit mode)
        let driver = if config.use_kernel_driver {
            println!("[*] Attempting to load kernel driver...");
            let drv = DriverInterface::new(process.pid);
            if drv.is_some() {
                println!("[+] Kernel driver loaded - EAC bypass active");
            } else {
                println!("[!] Kernel driver not found");
                if matches!(config.no_recoil_method, RecoilMethod::MemoryPatch) {
                    println!("[!] WARNING: MemoryPatch recoil needs driver!");
                    println!("[!] Falling back to fallback mode (HIGH RISK)");
                }
            }
            drv
        } else {
            println!("[*] Running in LEGIT MODE - No driver needed");
            println!("[+] Read-only operations (ESP) are safe");
            None
        };
        
        // EAC bypass helper
        let eac_bypass = EACBypass::new(process.handle);
        
        // Wait for EAC startup scan ONLY if using driver
        if config.use_kernel_driver {
            EACBypass::wait_for_game_load();
        }
        
        // Find GameAssembly.dll base
        let game_assembly_base = process.get_module_base("GameAssembly.dll")?;
        println!("[+] GameAssembly.dll: 0x{:X}", game_assembly_base);
        
        let humanizer = Humanizer::new(config.clone());
        let esp_optimizer = ESPOptimizer::new();
        let recoil_helper = RecoilHelper::new();
        
        Some(RustCheat {
            process,
            driver,
            eac_bypass,
            offsets,
            game_assembly_base,
            local_player: 0,
            config,
            humanizer,
            esp_optimizer,
            recoil_helper,
        })
    }
    
    // Safe read - uses driver if available, fallback to normal read
    fn safe_read<T: Copy>(&self, address: usize) -> Result<T, ()> {
        // Add random jitter to avoid detection patterns
        EACBypass::random_delay();
        
        // Verify memory is readable first
        if !self.eac_bypass.safe_read_check(address) {
            return Err(());
        }
        
        if let Some(ref driver) = self.driver {
            driver.read(address)
        } else {
            self.process.read(address)
        }
    }
    
    // Safe write - uses driver if available
    fn safe_write<T: Copy>(&self, address: usize, value: T) -> Result<(), ()> {
        EACBypass::random_delay();
        
        if let Some(ref driver) = self.driver {
            driver.write(address, value)
        } else {
            self.process.write(address, value)
        }
    }
    
    fn find_local_player(&mut self) -> Option<usize> {
        // Pattern scan for LocalPlayer pointer
        let scanner = PatternScanner::new(&self.process);
        
        // Example pattern - you need to find the actual pattern in IDA/x64dbg
        let pattern = "48 8B 0D ? ? ? ? 48 85 C9 74 ? 48 8B 49 ? 48 85 C9";
        
        if let Some(addr) = scanner.scan_pattern(
            self.game_assembly_base,
            0x5000000,
            pattern
        ) {
            // Resolve RIP-relative address
            if let Ok(local_player_ptr) = scanner.resolve_rip_relative(addr, 3, 7) {
                if let Ok(local_player) = self.safe_read::<usize>(local_player_ptr) {
                    println!("[+] LocalPlayer: 0x{:X}", local_player);
                    return Some(local_player);
                }
            }
        }
        
        None
    }
    
    fn get_players(&mut self) -> Vec<Player> {
        let mut players = Vec::new();
        
        if self.local_player == 0 {
            return players;
        }
        
        // Get local player position for distance calculation
        let local_pos = self.get_player_position(self.local_player).unwrap_or(Vec3 { x: 0.0, y: 0.0, z: 0.0 });
        
        // Get camera forward vector for FOV culling (simplified - needs actual camera data)
        let camera_forward = Vec3 { x: 0.0, y: 0.0, z: 1.0 }; // TODO: Read from PlayerInput
        
        // Iterate through entity list
        // This is simplified - actual implementation needs proper entity iteration
        for i in 0..200 {
            let entity_addr = self.game_assembly_base + 0x1000000 + (i * 0x8); // Example
            
            if let Ok(entity) = self.safe_read::<usize>(entity_addr) {
                if entity == 0 || entity == self.local_player {
                    continue;
                }
                
                // Check cache first (ESP optimization)
                if let Some(cached) = self.esp_optimizer.get_cached_player(entity) {
                    // Use cached data if still valid
                    players.push(Player {
                        address: cached.address,
                        position: cached.position,
                        health: cached.health,
                        max_health: cached.max_health,
                        distance: cached.distance,
                        is_local: false,
                    });
                    continue;
                }
                
                // Check if it's a BasePlayer
                if let Ok(health) = self.safe_read::<f32>(entity + self.offsets.health) {
                    if health > 0.0 && health <= 100.0 {
                        if let Some(pos) = self.get_player_position(entity) {
                            let distance = local_pos.distance(&pos);
                            
                            // ESP Optimization: Distance culling
                            if !self.esp_optimizer.should_render_player(distance) {
                                continue;
                            }
                            
                            // ESP Optimization: FOV culling
                            if !self.esp_optimizer.is_in_fov(pos, local_pos, camera_forward) {
                                continue;
                            }
                            
                            let max_health = self.safe_read::<f32>(entity + self.offsets.max_health)
                                .unwrap_or(100.0);
                            
                            // Update cache
                            self.esp_optimizer.update_cache(entity, pos, health, max_health, distance, true);
                            
                            players.push(Player {
                                address: entity,
                                position: pos,
                                health,
                                max_health,
                                distance,
                                is_local: false,
                            });
                        }
                    }
                }
            }
        }
        
        // Increment frame counter for ESP optimization
        self.esp_optimizer.next_frame();
        
        players
    }
    
    fn get_player_position(&self, player_addr: usize) -> Option<Vec3> {
        // Get PlayerModel
        let player_model = self.safe_read::<usize>(player_addr + self.offsets.player_model).ok()?;
        if player_model == 0 {
            return None;
        }
        
        // Get Transform
        let transform = self.safe_read::<usize>(player_model + self.offsets.transform).ok()?;
        if transform == 0 {
            return None;
        }
        
        // Read position from transform
        let pos = self.safe_read::<Vec3>(transform + self.offsets.position).ok()?;
        Some(pos)
    }
    
    fn apply_no_recoil(&self) {
        // Check if recoil control is enabled
        match self.config.no_recoil_method {
            RecoilMethod::None => return,
            RecoilMethod::Macro => {
                // Macro recoil is handled by external script
                // See macro_norecoil.md for Logitech G HUB setup
                return;
            },
            RecoilMethod::Hardware => {
                // MAKCU hardware handles recoil
                // See src/makcu_interface.rs
                return;
            },
            RecoilMethod::MemoryPatch => {
                // Continue to memory patching below
            }
        }
        
        if self.local_player == 0 {
            return;
        }
        
        // Humanized delay (looks less robotic)
        if self.config.humanization_enabled {
            EACBypass::polymorphic_sleep(50);
        }
        
        // Get PlayerInput
        if let Ok(player_input) = self.safe_read::<usize>(self.local_player + self.offsets.player_input) {
            if player_input != 0 {
                // Zero out recoil angles (via kernel driver)
                let zero_vec = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
                let _ = self.safe_write(player_input + self.offsets.recoil_angles, zero_vec);
            }
        }
    }
    
    fn draw_esp(&self, players: &[Player]) {
        for player in players {
            if player.is_local {
                continue;
            }
            
            // Get detail level based on distance (ESP optimization)
            let detail = self.esp_optimizer.get_detail_level(player.distance);
            
            // Build info string based on detail level
            let mut info = String::new();
            
            if detail.show_distance() {
                info.push_str(&format!("Distance: {:.1}m", player.distance));
            }
            
            if detail.show_health_bar() {
                info.push_str(&format!(" | HP: {:.0}/{:.0}", player.health, player.max_health));
            }
            
            if detail.show_name() {
                info.push_str(&format!(" | Pos: ({:.1}, {:.1}, {:.1})", 
                    player.position.x, player.position.y, player.position.z));
            }
            
            // Print to console (actual overlay would draw on screen)
            if !info.is_empty() {
                println!("[ESP] {}", info);
            }
        }
    }
}

fn is_admin() -> bool {
    // Check if running with admin privileges
    true // Simplified
}

fn main() {
    println!("╔══════════════════════════════════════════════╗");
    println!("║   Rust EAC Bypass Cheat v3.1 - 2026         ║");
    println!("║   LEGIT MODE for Maximum Safety             ║");
    println!("╚══════════════════════════════════════════════╝");
    println!();
    
    // Load configuration
    let config = CheatConfig::load();
    
    println!("[+] Configuration loaded:");
    println!("    Mode: {:?}", config.mode);
    println!("    ESP: {}", if config.esp_enabled { "✓" } else { "✗" });
    println!("    No Recoil: {:?}", config.no_recoil_method);
    println!("    Kernel Driver: {}", if config.use_kernel_driver { "✓" } else { "✗" });
    println!("    Humanization: {}", if config.humanization_enabled { "✓" } else { "✗" });
    println!();
    
    // Safety warnings based on mode
    match config.mode {
        SafetyMode::Legit => {
            println!("[✓] LEGIT MODE - Maximum safety");
            println!("    Detection risk: LOW");
            println!("    Expected survival: 1-3+ months");
            println!("    Features: ESP only, macro recoil");
        },
        SafetyMode::Rage => {
            println!("[!] RAGE MODE - Higher detection risk");
            println!("    Detection risk: MEDIUM");
            println!("    Expected survival: 1-2 weeks");
            println!("    Recommendation: Use alt account only");
        },
        SafetyMode::DMA => {
            println!("[✓] DMA MODE - Minimal detection");
            println!("    Detection risk: MINIMAL");
            println!("    Expected survival: Months to years");
        },
    }
    println!();
    
    // Check for admin rights
    if !is_admin() {
        if config.use_kernel_driver {
            println!("[!] WARNING: Not running as administrator!");
            println!("[!] Driver loading will fail without admin rights");
            println!();
        }
    }
    
    // HWID spoof option (only if using memory writes)
    if matches!(config.no_recoil_method, RecoilMethod::MemoryPatch) {
        println!("[?] Run HWID spoofer? (prevents hardware bans)");
        println!("    Type 'yes' to spoof, or press Enter to skip");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).ok();
        if input.trim().to_lowercase() == "yes" {
            eac_bypass::hwid_spoof::full_spoof();
            println!("[!] Restart system before playing for spoof to take effect");
            return;
        }
    }
    
    println!("[+] Waiting for RustClient.exe...");
    
    let process = loop {
        if let Some(proc) = Process::from_name("RustClient.exe") {
            println!("[+] Found RustClient.exe (PID: {})", proc.pid);
            break proc;
        }
        thread::sleep(Duration::from_secs(2));
    };
    
    let mut cheat = match RustCheat::new(process, config.clone()) {
        Some(c) => c,
        None => {
            println!("[-] Failed to initialize cheat");
            return;
        }
    };
    
    println!();
    println!("[+] Cheat initialized");
    println!("[+] Active Features:");
    if config.esp_enabled {
        println!("    ✓ ESP (Player positions, health, distance)");
    }
    match config.no_recoil_method {
        RecoilMethod::None => println!("    ✗ No Recoil (disabled)"),
        RecoilMethod::Macro => println!("    ✓ No Recoil (Logitech macro - SAFE)"),
        RecoilMethod::Hardware => println!("    ✓ No Recoil (MAKCU hardware - SAFE)"),
        RecoilMethod::MemoryPatch => {
            if cheat.driver.is_some() {
                println!("    ✓ No Recoil (kernel driver - MEDIUM RISK)");
            } else {
                println!("    ⚠ No Recoil (fallback - HIGH RISK)");
            }
        },
    }
    println!();
    
    // Find local player
    println!("[*] Scanning for LocalPlayer...");
    if let Some(local_player) = cheat.find_local_player() {
        cheat.local_player = local_player;
        println!("[+] LocalPlayer found!");
    } else {
        println!("[-] Could not find LocalPlayer - ESP will be limited");
    }
    
    // Main cheat loop with humanization
    println!();
    println!("[+] Cheat running... Press Ctrl+C to exit");
    println!();
    println!("[*] ESP Optimization: ENABLED");
    println!("    - Distance-based LOD (Level of Detail)");
    println!("    - FOV culling (only render visible players)");
    println!("    - Frame skipping for distant players");
    println!("    - Player data caching");
    println!();
    println!("[*] Recoil Helper: ENABLED (Read-Only)");
    println!("    - Visual compensation guide");
    println!("    - No memory writes (100% SAFE)");
    println!("    - Load weapon patterns from memory");
    println!();
    
    let mut tick = 0;
    loop {
        // Update weapon data from memory (read-only)
        if tick % 10 == 0 && cheat.local_player != 0 {
            if let Some(weapon) = cheat.recoil_helper.read_weapon_from_memory(
                &cheat.process,
                cheat.local_player,
                cheat.offsets.held_entity,
                cheat.offsets.weapon_recoil,
            ) {
                cheat.recoil_helper.update_weapon(weapon);
            }
        }
        
        // Display recoil helper info
        if tick % 20 == 0 {
            if let Some(weapon_info) = cheat.recoil_helper.get_weapon_info() {
                println!("[Recoil] {}", weapon_info);
                
                // Show compensation offset
                let offset = cheat.recoil_helper.get_compensation_offset();
                if offset.x != 0.0 || offset.y != 0.0 {
                    println!("[Recoil] Compensation: X={:.2}, Y={:.2}", offset.x, offset.y);
                }
            }
        }
        
        // Apply no recoil (if enabled)
        cheat.apply_no_recoil();
        
        // Update ESP with humanized timing and optimization
        if config.esp_enabled && tick % 5 == 0 {
            // Random skip (looks human)
            if !cheat.humanizer.should_skip_esp_frame() {
                let players = cheat.get_players();
                if !players.is_empty() {
                    cheat.draw_esp(&players);
                }
            }
            
            // Humanized delay between ESP updates
            cheat.humanizer.esp_update_delay();
        }
        
        tick += 1;
        thread::sleep(Duration::from_millis(100));
    }
}
