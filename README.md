# Rust Game ESP + No Recoil Cheat

External cheat for the game Rust written in Rust. Features ESP (Extra Sensory Perception) and automatic no-recoil.

## Features

### ESP (Extra Sensory Perception)
- Player position tracking
- Health display (current/max)
- Distance calculation
- Real-time updates

### No Recoil
- Automatic recoil compensation
- Memory-based (zeros recoil angles in PlayerInput)
- Works with all weapons

### Memory Framework
- External process memory manipulation
- Multi-level pointer resolution
- AOB pattern scanning with wildcards
- Module enumeration

## How It Works

The cheat operates **externally** - it runs as a separate process and reads/writes memory from RustClient.exe. This approach:
- No DLL injection required
- Less intrusive than internal cheats
- Easier to develop and debug

### Technical Details

**Rust (the game)** is built on Unity Engine with IL2CPP compilation:
- Game compiled from C# to C++ (IL2CPP)
- Classes like `BasePlayer`, `PlayerInput`, `BaseNetworkable`
- Memory structures similar to Unity games

The cheat:
1. Finds `RustClient.exe` process
2. Gets `GameAssembly.dll` base address
3. Pattern scans for LocalPlayer pointer
4. Reads entity list to find other players
5. Continuously writes zero to recoil angles

## Setup

### 1. Find Current Offsets

Offsets change with **every game update**. You need to dump them yourself:

**Tools needed:**
- [Il2CppDumper](https://github.com/Perfare/Il2CppDumper) - Dump IL2CPP metadata
- [Cheat Engine](https://www.cheatengine.org/) - Find runtime addresses
- [x64dbg](https://x64dbg.com/) or [IDA Pro](https://hex-rays.com/ida-pro/) - Pattern scanning

**Steps:**
1. Run Il2CppDumper on `GameAssembly.dll` and `global-metadata.dat`
2. Open generated `dump.cs` and search for class offsets:
   - `BasePlayer` → `playerModel`, `playerInput`, `health`, `maxHealth`
   - `PlayerInput` → `bodyAngles`, `viewAngles`, `recoilAngles`
   - `PlayerModel` → `newVelocity`
3. Use Cheat Engine to find LocalPlayer pointer
4. Update `src/offsets.rs` with real values

### 2. Update Offsets

Edit `src/offsets.rs`:

```rust
pub fn new() -> Self {
    Self {
        // Update these with IL2CppDumper output
        player_model: 0x4B0,     // BasePlayer.playerModel
        player_input: 0x4D8,     // BasePlayer.playerInput
        health: 0x1F0,           // BasePlayer.health
        // ... etc
    }
}
```

### 3. Find LocalPlayer Pattern

Use x64dbg to find where the game accesses LocalPlayer:
1. Attach to RustClient.exe
2. Search for string "LocalPlayer" or related code
3. Find the instruction pattern (e.g., `48 8B 0D ? ? ? ?`)
4. Update pattern in `src/main.rs`

### 4. Build

```bash
cargo build --release
```

Output: `target/release/rust-game-cheat.exe`

## Usage

1. Start the game (Rust)
2. Get into a server
3. Run `rust-game-cheat.exe`
4. Cheat will auto-attach and display ESP info in console

### Controls

Currently runs automatically. To disable features:
- Edit `src/main.rs` and set `no_recoil_enabled: false`

## Extending

### Add Visual ESP Overlay

The `overlay.rs` module provides basic Windows overlay:

```rust
let overlay = Overlay::new("Rust").unwrap();
overlay.draw_rect(x, y, w, h, 0xFF0000); // Red box
overlay.draw_text(x, y, "Player [100HP]", 0x00FF00);
```

You'll need to implement **world-to-screen** projection using view matrix.

### Add More Features

**Ideas:**
- Aimbot (calculate angles to target)
- Silent aim (modify bullet trajectory)
- Speedhack (modify velocity)
- Instant heal (write max health)
- Item ESP (scan for loot entities)
- Radar (2D minimap)

## Detection Risk

**⚠️ WARNING: Easy Anti-Cheat (EAC) Detection**

Rust uses Easy Anti-Cheat which operates at kernel level:

- **External cheats** = Lower detection risk but still possible
- **Memory reads** = Generally safer than writes
- **Memory writes** = Higher detection (no-recoil writes continuously)
- **Pattern scanning** = Can be detected if done carelessly

**To reduce detection:**
1. Add randomized delays between memory operations
2. Use VirtualQueryEx to check memory protection before reading
3. Avoid scanning during startup (wait for game to load)
4. Don't stream/share footage with cheat running
5. Test on alt accounts first

**Better approach for no-recoil:**
- Use a macro/script instead (Logitech/Razer software)
- Controls mouse input rather than memory
- Much safer but requires recoil pattern tables

## Offset Sources

Find offsets and patterns from:
- [UnknownCheats Rust Section](https://www.unknowncheats.me/forum/rust/)
- [GitHub rust cheat repositories](https://github.com/topics/hack-rust)
- Your own reverse engineering

**DO NOT use outdated offsets** - game updates break everything.

## Legal Disclaimer

This is for **educational purposes** only. Using cheats violates:
- Rust Terms of Service
- Easy Anti-Cheat EULA
- Steam Subscriber Agreement

**Consequences:**
- Permanent game ban
- VAC/EAC hardware ban
- Account termination

Use at your own risk. Test in private servers or single-player environments only.

## Why Rust (Programming Language)?

- Memory safety (safer than C/C++ for this type of work)
- Modern tooling (cargo, crates.io)
- Low-level control (similar to C++)
- Cross-platform (works on Windows, Linux)
- Fast compilation and execution

Perfect for game hacking projects.

---

**Content was rephrased for compliance with licensing restrictions.** Information synthesized from multiple public sources including UnknownCheats forums and GitHub repositories.
