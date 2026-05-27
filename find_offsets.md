# How to Find Rust Game Offsets

This guide explains how to find current offsets for the Rust cheat.

## Tools Required

1. **Il2CppDumper** - https://github.com/Perfare/Il2CppDumper
2. **Cheat Engine** - https://www.cheatengine.org/
3. **x64dbg** or **IDA Pro** - For pattern scanning
4. **ReClass.NET** (optional) - For reconstructing structures

## Step 1: Dump IL2CPP Classes

### Find Game Files

Rust installation folder (Steam):
```
C:\Program Files (x86)\Steam\steamapps\common\Rust\
```

You need:
- `GameAssembly.dll` (in Rust folder)
- `global-metadata.dat` (in `Rust_Data\il2cpp_data\Metadata\`)

### Run Il2CppDumper

1. Download latest Il2CppDumper release
2. Extract and run `Il2CppDumper.exe`
3. Select `GameAssembly.dll`
4. Select `global-metadata.dat`
5. Choose Unity version (check game properties)
6. Wait for dump to complete

### Output Files

- `dump.cs` - All class definitions with offsets
- `script.json` - For Cheat Engine/IDA

## Step 2: Find Class Offsets

Open `dump.cs` and search for these classes:

### BasePlayer Class

```csharp
// Search for: "public class BasePlayer"
public class BasePlayer : BaseCombatEntity
{
    // 0x4B0
    public PlayerModel playerModel;
    
    // 0x4D8
    public PlayerInput playerInput;
    
    // 0x610
    public PlayerInventory playerInventory;
    
    // 0x570
    public ModelState modelState;
    
    // 0x224
    public LifeState lifeState;
    
    // Add offsets shown in dump.cs comments
}
```

### PlayerInput Class

```csharp
// Search for: "public class PlayerInput"
public class PlayerInput : EntityComponent<BasePlayer>
{
    // 0x3C
    public Vector3 bodyAngles;
    
    // 0x44
    public Vector3 viewAngles;
    
    // 0x4C
    public Vector3 recoilAngles; // THIS IS KEY FOR NO RECOIL
}
```

### PlayerModel Class

```csharp
public class PlayerModel : ListComponent<BasePlayer>
{
    // 0x1D4
    public Vector3 newVelocity;
}
```

### BaseCombatEntity (Parent of BasePlayer)

```csharp
public class BaseCombatEntity : BaseEntity
{
    // 0x1F0
    public float health;
    
    // 0x1F4
    public float maxHealth;
}
```

## Step 3: Find LocalPlayer Pointer

### Using Cheat Engine

1. Launch Rust and join a server
2. Open Cheat Engine, attach to `RustClient.exe`
3. Search for your current health (float value, usually 100)
4. Take damage or heal
5. "Next Scan" with new health value
6. Repeat until you find the address
7. Right-click address → "Find out what accesses this address"
8. Look for the instruction that reads health
9. Work backwards to find the base pointer

### Example

If you find health at `2345678.1F0`:
- The base object (BasePlayer) is at `23456780`
- Find what writes to this address
- Look for patterns like `mov rax, [rcx+XXX]`

### Using x64dbg

1. Attach to RustClient.exe
2. Set breakpoint on `GameAssembly.dll` base
3. Search for string references to "LocalPlayer"
4. Find code that accesses the local player instance
5. Note the pattern bytes

Common pattern example:
```asm
48 8B 0D ? ? ? ?    ; mov rcx, [rip+offset]
48 85 C9            ; test rcx, rcx
74 XX               ; je skip
48 8B 49 XX         ; mov rcx, [rcx+XX]
```

The `? ? ? ?` are wildcards - these are the bytes we scan for.

## Step 4: Get Pattern Bytes

In x64dbg:
1. Find the LocalPlayer access instruction
2. Right-click → Copy → Pattern
3. Copy bytes like: `48 8B 0D ? ? ? ? 48 85 C9`

Update in `src/main.rs`:
```rust
let pattern = "48 8B 0D ? ? ? ? 48 85 C9 74 ? 48 8B 49 ? 48 85 C9";
```

## Step 5: Update Code

Edit `src/offsets.rs`:

```rust
impl RustOffsets {
    pub fn new() -> Self {
        Self {
            // Update from dump.cs
            player_model: 0x4B0,      // From BasePlayer.playerModel
            player_input: 0x4D8,      // From BasePlayer.playerInput
            health: 0x1F0,            // From BaseCombatEntity.health
            max_health: 0x1F4,        // From BaseCombatEntity.maxHealth
            recoil_angles: 0x4C,      // From PlayerInput.recoilAngles
            // ... rest
        }
    }
}
```

## Step 6: Test

1. Build the cheat: `cargo build --release`
2. Run Rust (the game)
3. Run the cheat executable
4. Check console output for "LocalPlayer found!"

If it fails:
- Pattern might be wrong (game was updated)
- Offsets might be wrong
- Need to re-dump with Il2CppDumper

## Finding Entity List

The entity list is trickier. You need to find `BaseNetworkable.clientEntities` or similar.

In Cheat Engine:
1. Find an enemy player's health
2. Find what accesses it
3. Work backwards to find the list structure
4. Usually stored as `List<BaseEntity>` or similar

## Alternative: Use ReClass.NET

1. Open ReClass.NET
2. Attach to RustClient.exe
3. Add BasePlayer structure
4. Manually add fields and verify offsets
5. Helps visualize memory layout

## Keeping Updated

Game updates **BREAK EVERYTHING**. After each Rust update:

1. Re-run Il2CppDumper (offsets might change)
2. Re-scan for LocalPlayer pattern (code might be recompiled)
3. Update offsets in code
4. Rebuild cheat

## Community Resources

Check these for updated offsets:
- UnknownCheats Rust section
- GitHub repos tagged with "hack-rust"
- Cheat forums (use discretion)

**Note:** Never trust random executables. Always dump offsets yourself.

## Quick Reference

| What | Tool | Time |
|------|------|------|
| Class offsets | Il2CppDumper | 5 min |
| LocalPlayer pointer | Cheat Engine + x64dbg | 30 min |
| Pattern bytes | x64dbg | 10 min |
| Entity list | Cheat Engine (hard) | 1-2 hours |

Total time: ~2-3 hours for a complete offset update.

---

This is the most tedious part of game hacking. Once you have correct offsets, the cheat will work until the next game update.
