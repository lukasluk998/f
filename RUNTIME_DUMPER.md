# Runtime Offset Dumper

**No more manual Il2CppDumper!** This tool automatically finds all offsets from the running game.

## Features

✅ **Fully Automatic** - Scans game memory and finds all offsets  
✅ **No Manual Work** - No need to extract DLLs or use Il2CppDumper  
✅ **Pattern-Based** - Uses AOB signatures resistant to patches  
✅ **Smart Detection** - Validates offsets with heuristics  
✅ **Auto-Updates Code** - Generates ready-to-use Rust code  
✅ **Git Integration** - Optionally commits to resources repo  

## Usage

### Step 1: Start Rust Game
```bash
# Launch Rust
# Join any server
# Spawn as a player (important!)
```

### Step 2: Run Dumper
```bash
cd f
cargo run --bin dump_offsets
```

### Step 3: Follow Prompts
```
[*] Waiting for RustClient.exe...
[+] Found RustClient.exe (PID: 12345)
[+] GameAssembly.dll: 0x7FF6A2B0000

[!] IMPORTANT: Join a server and spawn in as a player!
[!] Press ENTER when you're in-game and spawned...
```

**Why spawn in?**
- LocalPlayer only exists after spawning
- Health/position values needed for validation
- More accurate offset detection

### Step 4: Automatic Dump
```
[*] Starting automated offset dump...

[1/5] Finding base addresses...
  [+] LocalPlayer: 0x2F3A1C0 (resolved: 0x2F3A1C0)
  [+] LocalPlayer instance: 0x1A3B4C5D0

[2/5] Analyzing BasePlayer structure...
  [*] Analyzing BasePlayer at 0x1A3B4C5D0
  [+] health: 0x1F0 (value: 100.0)
  [+] maxHealth: 0x1F4 (value: 100.0)
  [+] playerModel: 0x4B0 (value: 0x1A3B4D000)
  [+] playerInput: 0x4D8 (value: 0x1A3B4D100)

[3/5] Analyzing PlayerInput structure...
  [*] Analyzing PlayerInput at 0x1A3B4D100
  [+] bodyAngles: 0x3C (angles: 45.32, -12.45)
  [+] viewAngles: 0x44 (angles: 45.30, -12.47)
  [+] recoilAngles: 0x4C (angles: 0.00, 0.00)

[4/5] Analyzing PlayerModel structure...
  [+] newVelocity: 0x1D4 (value: 2.35)
  [+] transform: 0x30 (value: 0x1A3B4D200)

[5/5] Analyzing networking structures...
  [+] BaseNetworkable: 0x2F3A1D0

[+] ═══════════════════════════════════════
[+] DUMP COMPLETED SUCCESSFULLY
[+] ═══════════════════════════════════════
```

### Step 5: Files Created
```
../resources/offsets/2026-05-27/
  ├── offsets.json     # JSON dump
  ├── offsets.rs       # Rust code
  └── notes.md         # Info about dump

../resources/offsets/latest.json  # Always current

src/offsets.rs       # Auto-updated!
src/offsets.rs.backup_2026-05-27  # Old version backed up
```

### Step 6: Rebuild & Test
```bash
cargo build --release
./target/release/rust-game-cheat.exe
```

## How It Works

### 1. Pattern Scanning
Finds critical pointers using AOB signatures:
```rust
// LocalPlayer pattern
"48 8B 0D ? ? ? ? 48 85 C9 74 ? 48 8B 49"
```

### 2. RIP-Relative Resolution
Resolves x64 RIP-relative addresses:
```rust
// Instruction: MOV rcx, [rip+offset]
// At 0x7FF6A2B0123: 48 8B 0D 12 34 56 78
// Resolves to: (0x7FF6A2B0123 + 7) + 0x78563412
```

### 3. Heuristic Validation
Checks if values make sense:
```rust
// Health should be 0-200
if hp >= 0.0 && hp <= 200.0 { valid }

// Pointers should be in valid range
if addr > 0x10000 && addr < 0x7FFFFFFFFFFF { valid }

// Angles should be -360 to 360
if angle.abs() < 360.0 { valid }
```

### 4. Structure Analysis
Follows pointer chains:
```
LocalPlayer → BasePlayer
    ├→ PlayerModel → Transform → Position
    ├→ PlayerInput → Angles
    └→ Health/MaxHealth
```

## After Game Updates

When Rust patches:

```bash
# 1. Update game via Steam
# 2. Launch game and join server
# 3. Re-run dumper
cargo run --bin dump_offsets

# 4. Rebuild cheat
cargo build --release

# 5. Done!
```

**Offsets are automatically:**
- Saved to resources repo (dated folder)
- Updated in `src/offsets.rs`
- Old version backed up
- Git committed (optional)

## Troubleshooting

### "LocalPlayer not found"
**Fix:** Make sure you're **spawned in-game** as a player, not spectating or in menu.

### "Failed to analyze BasePlayer"
**Fix:** 
1. Ensure you have full health (not dead/wounded)
2. Stand still when running dumper
3. Try on different server

### "Pattern scan failed"
**Fix:**
1. Game updated but pattern didn't change - report issue
2. Verify GameAssembly.dll loaded correctly
3. Check if using modded/custom server

### Offsets seem wrong
**Verify manually:**
1. Open Cheat Engine
2. Attach to RustClient.exe
3. Search for your health value (float, 100.0)
4. Compare offset to dump
5. Report if mismatch

## Comparison to Il2CppDumper

| Feature | Runtime Dumper | Il2CppDumper |
|---------|---------------|--------------|
| No file extraction | ✅ | ❌ |
| Works while playing | ✅ | ❌ |
| Auto-generates Rust code | ✅ | ❌ |
| Finds runtime values | ✅ | ❌ |
| Full class definitions | ❌ | ✅ |
| Works offline | ❌ | ✅ |

**Use Il2CppDumper when:**
- You need full class definitions
- Game has anti-debug
- You don't have game running

**Use Runtime Dumper when:**
- Quick update after patch
- You're already in-game
- Want most accurate offsets

## Advanced Usage

### Custom Patterns
Edit patterns in `src/runtime_dumper.rs`:

```rust
let patterns = vec![
    ("LocalPlayer", "48 8B 0D ? ? ? ? 48 85 C9 74"),
    ("YourPattern", "?? ?? ?? ? ? ? ?"),
];
```

### Export Format
Change export in `dump_offsets.rs`:

```rust
// JSON only
dumper.save_to_file(&dump, "offsets.json")?;

// C++ header
let cpp_code = generate_cpp_code(&dump);
fs::write("offsets.h", cpp_code)?;

// Python
let py_code = generate_python_code(&dump);
fs::write("offsets.py", py_code)?;
```

### Continuous Updates
Auto-run dumper every N minutes:

```rust
loop {
    let dump = dumper.dump_all_offsets()?;
    dumper.save_to_file(&dump, "offsets.json")?;
    thread::sleep(Duration::from_secs(300)); // 5 min
}
```

## Files

| File | Purpose |
|------|---------|
| `src/runtime_dumper.rs` | Core dumper logic |
| `src/bin/dump_offsets.rs` | Standalone tool binary |
| `src/scanner.rs` | Pattern scanner (AOB) |
| `src/memory.rs` | Process memory reader |

## Resources

Dumps are saved to:
- https://github.com/lukasluk998/resources

Main cheat:
- https://github.com/lukasluk998/f

## Detection Risk

Runtime dumper:
- ✅ Read-only (no writes to game)
- ✅ External process (not injected)
- ✅ No hooks or patches
- ✅ Same risk as Cheat Engine

**Safe to use** - EAC can't detect memory reading from external process.

## Summary

**Before (Manual):**
1. Download Il2CppDumper
2. Extract GameAssembly.dll
3. Extract global-metadata.dat
4. Run Il2CppDumper
5. Open dump.cs
6. Find offsets manually
7. Update src/offsets.rs
8. Rebuild

**After (Runtime Dumper):**
1. Run `cargo run --bin dump_offsets`
2. Done!

Enjoy the automation clack! 🔥
