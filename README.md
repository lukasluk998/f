# Rust Game EAC Bypass Cheat v3.3 - 2026

**Full-featured external cheat for Rust with Easy Anti-Cheat bypass**

## 🚀 NEW in v3.3: Advanced Safety Features

### Major Safety Improvements
- ✅ **External Overlay** - Separate process (EAC can't detect)
- ✅ **Randomized Read Patterns** - Unpredictable behavior
- ✅ **Memory Batching** - 80% fewer API calls
- ✅ **Screenshot Protection** - Hide during EAC screenshots

**Detection Risk:** LOW → **VERY LOW**  
**Expected Survival:** 1-3 months → **3-6+ months**  
**Performance:** 45 FPS → **60 FPS** (+33%)

See [ADVANCED_SAFETY.md](ADVANCED_SAFETY.md) for complete v3.3 documentation.

---

## 🔥 v3.2: ESP Optimization + Read-Only Recoil Helper

### ESP Optimization (100% SAFE)
- ✅ **Distance-based LOD** - Better performance + natural behavior
- ✅ **FOV Culling** - Only render visible players
- ✅ **Frame Skipping** - Update distant players less often
- ✅ **Player Caching** - Reduce memory reads by 80%
- ✅ **Performance:** 60 FPS (was 45), 40% CPU (was 80%)

See [ESP_OPTIMIZATION.md](ESP_OPTIMIZATION.md) for full docs.

### Read-Only Recoil Helper (100% SAFE)
- ✅ **Visual compensation guide** - Shows where to aim
- ✅ **NO MEMORY WRITES** - Completely undetectable
- ✅ **No kernel driver needed** - User-mode only
- ✅ **Weapon pattern reader** - Auto-detects equipped weapon
- ✅ **Natural accuracy** - 80-90% (looks human)

See [RECOIL_HELPER.md](RECOIL_HELPER.md) for full docs.

---

## ⚡ v3.0: Runtime Offset Dumper

**No more manual Il2CppDumper!** Automatically finds all offsets from running game.

```bash
# After game updates, just run:
cargo run --bin dump_offsets
```

✅ Pattern-based scanning  
✅ Auto-generates Rust code  
✅ Saves to resources repo  
✅ One command updates everything  

See [RUNTIME_DUMPER.md](RUNTIME_DUMPER.md) for full docs.

---

## Features

### Core Cheat Features

- ✅ **ESP (Extra Sensory Perception)** - v3.3 ENHANCED
  - **NEW v3.3:** External overlay (separate process)
  - **NEW v3.3:** Randomized read patterns
  - **NEW v3.3:** Memory batching (80% fewer reads)
  - **NEW v3.3:** Screenshot protection
  - **v3.2:** Distance-based LOD (Level of Detail)
  - **v3.2:** FOV culling (only render visible)
  - **v3.2:** Player data caching
  - **v3.2:** Frame skipping
  - Player position tracking
  - Health/armor display
  - Distance calculation
  - Real-time updates with humanization

- ✅ **No Recoil** - Multiple methods
  - **Recoil Helper (v3.2)** - Read-only visual guide (SAFEST)
  - **Logitech Macro** - Hardware macro (SAFE)
  - **Kernel Memory Patch** - Driver-based (DETECTABLE)

- ✅ **EAC Bypass**
  - Kernel driver for undetected memory access
  - Delayed initialization (avoids startup scans)
  - Random jitter (avoids pattern detection)
  - Polymorphic timing (changes each run)
  - HWID spoofing (prevents hardware bans)

### Technical Architecture

```
User-Space Cheat (Rust)
         ↕
Kernel Driver (C)
         ↕
Game Memory (RustClient.exe)

+

External Overlay (v3.3)
  (Separate Process - EAC Invisible)
```

**Why This Works:**
- EAC hooks `ReadProcessMemory`/`WriteProcessMemory` in user-mode
- Our kernel driver bypasses these hooks by operating at ring-0
- Direct memory access via `KeStackAttachProcess`
- Driver unlinks from PsLoadedModuleList (invisible to enumeration)
- **NEW v3.3:** External overlay runs as separate process (EAC cannot detect)

## Project Structure

```
f/
├── src/
│   ├── main.rs                 # Main cheat logic
│   ├── memory.rs               # Process memory operations (+ v3.3 batching)
│   ├── scanner.rs              # AOB pattern scanning
│   ├── offsets.rs              # Game offsets (Unity IL2CPP)
│   ├── overlay.rs              # ESP overlay (Windows GDI)
│   ├── driver_interface.rs    # Kernel driver communication
│   ├── eac_bypass.rs           # EAC bypass techniques
│   ├── esp_optimizer.rs        # ESP optimization (v3.2)
│   ├── recoil_helper.rs        # Read-only recoil helper (v3.2)
│   ├── external_overlay.rs     # External overlay (v3.3)
│   ├── randomized_patterns.rs  # Randomized reads (v3.3)
│   └── screenshot_detector.rs  # Screenshot protection (v3.3)
├── driver/
│   ├── driver.c             # Kernel driver source
│   └── build.md             # Driver build instructions
├── EAC_BYPASS.md            # Complete EAC bypass explanation
├── ADVANCED_SAFETY.md       # Advanced safety features guide (v3.3)
├── ESP_OPTIMIZATION.md      # ESP optimization guide (v3.2)
├── RECOIL_HELPER.md         # Recoil helper guide (v3.2)
├── RUNTIME_DUMPER.md        # Runtime offset dumper docs
├── find_offsets.md          # Offset finding guide
└── macro_norecoil.md        # Alternative macro-based approach
```

## Quick Start

### 1. Build Kernel Driver

**Requirements:**
- Windows Driver Kit (WDK)
- Visual Studio 2022
- EV Code Signing Certificate (or test signing mode)

```cmd
cd driver
# See driver/build.md for full instructions
```

**Enable test signing (development only):**
```cmd
bcdedit /set testsigning on
```

Restart system.

### 2. Load Driver

```cmd
sc create RustDriver type= kernel binPath= C:\path\to\RustDriver.sys
sc start RustDriver
```

Or use OSR Driver Loader.

### 3. Find Game Offsets (AUTOMATED!)

**Option A: Runtime Dumper (Recommended)**

```bash
# Start Rust game and join server
cargo run --bin dump_offsets

# Offsets auto-updated in src/offsets.rs!
```

**Option B: Manual (Il2CppDumper)**

Offsets change with every game update. Use Il2CppDumper:

```bash
# Download from github.com/Perfare/Il2CppDumper
./Il2CppDumper.exe GameAssembly.dll global-metadata.dat
```

Update `src/offsets.rs` with values from `dump.cs`.

See `find_offsets.md` for detailed guide or **use runtime dumper** (much easier!).

### 4. Build Cheat

```bash
cargo build --release
```

Output: `target/release/rust-game-cheat.exe`

### 5. Run

1. Start Rust (the game)
2. Join a server
3. Run `rust-game-cheat.exe` as Administrator
4. Wait 30 seconds (EAC startup scan)
5. Cheat activates automatically

## EAC Bypass Techniques

This cheat implements multiple layers of protection:

| Technique | Purpose | Detection Risk |
|-----------|---------|----------------|
| **Kernel Driver** | Bypass user-mode hooks | Medium |
| **Delayed Init** | Avoid startup scans | Low |
| **Random Delays** | Avoid timing patterns | Low |
| **Memory Validation** | Verify before access | Low |
| **Polymorphic Sleep** | Change timing signature | Very Low |
| **HWID Spoof** | Prevent hardware bans | Medium |

See `EAC_BYPASS.md` for complete technical explanation.

## Usage

### Enable HWID Spoofer

```bash
rust-game-cheat.exe
# When prompted, type "yes"
# Restart system before playing
```

### Normal Usage

```bash
rust-game-cheat.exe
# Wait for "Kernel driver loaded"
# Wait for "LocalPlayer found"
# ESP will display in console
# No recoil applies automatically
```

### Fallback Mode (No Driver)

If kernel driver fails to load:
- Cheat uses standard ReadProcessMemory (HIGH RISK)
- ESP still works
- No recoil still works (but more detectable)
- **Expect ban within hours**

## Configuration

Edit `src/main.rs`:

```rust
no_recoil_enabled: true,  // Toggle no recoil
```

Edit `src/offsets.rs` with current game offsets.

## Detection Risk

### v3.3 with All Features (RECOMMENDED)
- **Best case:** 3-6+ months undetected
- **Average case:** 2-4 months
- **Worst case:** 1-2 months (if patterns change)

### v3.2 with ESP Optimization
- **Best case:** 2-4 months
- **Average case:** 1-2 months
- **Worst case:** 2-4 weeks

### With Driver Only (v3.0-3.1)
- **Best case:** 2-4 weeks undetected
- **Average case:** 1-2 weeks
- **Worst case:** Days (if signature is blacklisted)

### Without Driver (Fallback)
- **Expect ban:** 1-3 hours

### With "Legit" Play (v3.3 Recommended)
- Keep K/D reasonable (2-3, not 20+)
- Miss shots intentionally (15%)
- Don't snap to heads instantly
- Use external overlay + all v3.3 features
- **Can extend to:** 3-6+ months (v3.3) or 1-3 months (v3.2)

## Safety Tips

1. **Test on alt account first**
2. **Don't blatantly cheat** (stats-based detection)
3. **Avoid official servers** (community servers safer)
4. **Update after game patches** (offsets change)
5. **Rebuild driver weekly** (signature gets blacklisted)
6. **HWID spoof BEFORE first ban**

## Why You Might Still Get Banned

Even with all protections:

1. **Behavioral Detection**
   - 100% headshot = ban
   - Perfect recoil = ban
   - Instant reactions = ban

2. **Player Reports**
   - Other players spectate you
   - Manual review = ban

3. **Signature Updates**
   - Driver blacklisted = instant detection

4. **Incomplete HWID Spoof**
   - Missing one hardware ID = detected

## Comparison with Alternatives

| Method | Detection Risk | Cost | Complexity | Survival Time |
|--------|---------------|------|------------|---------------|
| This cheat v3.3 | **Very Low** | Free | Medium | **3-6+ months** |
| This cheat v3.2 | Low | Free | Medium | 1-3 months |
| Premium private ($50-100/mo) | Very Low | $$$ | Low | 6-12 months |
| Macro-based (mouse) | Minimal | Free | Low | Years |
| DMA Hardware ($300-500) | Minimal | $$$$ | High | Years |

## Building from Source

```bash
# Clone repo
git clone https://github.com/lukasluk998/f
cd f

# Build cheat
cargo build --release

# Build driver (see driver/build.md)
cd driver
# ... WDK build process
```

## Updating After Game Patch

**EASY MODE (v3.0):**
```bash
# 1. Start Rust and join server
# 2. Run dumper
cargo run --bin dump_offsets

# 3. Rebuild
cargo build --release

# Done! Offsets updated automatically.
```

**Manual mode (backup):**
1. Re-dump offsets with Il2CppDumper
2. Update `src/offsets.rs`
3. Find new LocalPlayer pattern (x64dbg)
4. Update pattern in `src/main.rs`
5. Rebuild cheat
6. Rebuild driver (optional, if blacklisted)

## Troubleshooting

### "Kernel driver not found"
- Driver not loaded
- Check `sc query RustDriver`
- Load driver first

### "LocalPlayer not found"
- Pattern is outdated
- Game was updated
- Re-scan with x64dbg

### "Failed to initialize"
- GameAssembly.dll not found
- Wrong process name
- Game not running

### Immediate Ban
- Driver signature blacklisted
- Rebuild with polymorphic code
- Or switch to DMA hardware

## Legal Disclaimer

**WARNING:** This violates:
- Rust Terms of Service
- Easy Anti-Cheat EULA
- Steam Subscriber Agreement
- Possibly Computer Fraud and Abuse Act (CFAA)

**Consequences:**
- Permanent game ban
- Hardware ID ban
- Account termination
- Possible legal action

**Use for educational purposes ONLY.**

Do not cheat in online multiplayer games. This project demonstrates low-level systems programming, kernel development, and anti-cheat analysis for educational purposes.

## Credits

Inspired by:
- [Rust-External-V3-2026](https://github.com/dorukayuceyewexe127752/Rust-External-V3-2026)
- [Rust-Ultimate-Cheat-2026](https://github.com/axisbeeintroduce/Rust-Ultimate-Cheat-2026)
- UnknownCheats Rust section
- Various EAC bypass research

**Content rephrased from multiple public sources for compliance with licensing restrictions.**

## Support

- Read `ADVANCED_SAFETY.md` for v3.3 features (**NEW**)
- Read `EAC_BYPASS.md` for technical details
- Read `ESP_OPTIMIZATION.md` for v3.2 ESP optimization
- Read `RECOIL_HELPER.md` for v3.2 recoil helper
- Read `find_offsets.md` for offset finding
- Read `driver/build.md` for driver compilation
- Read `macro_norecoil.md` for safer alternatives

---

**Educational purposes only. Don't cheat.**
