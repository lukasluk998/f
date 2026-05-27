# Building the Kernel Driver

The kernel driver bypasses EAC by operating at ring-0 level, same as EAC itself.

## Requirements

1. **Windows Driver Kit (WDK)**
   - Download from Microsoft: https://docs.microsoft.com/windows-hardware/drivers/download-the-wdk
   - Install Visual Studio 2022 first
   - Install WDK matching your Windows version

2. **Test Signing Mode** (for development)
   ```cmd
   bcdedit /set testsigning on
   ```
   Restart your system

3. **Driver Signing Certificate** (for production)
   - Buy EV code signing certificate ($300-500/year)
   - Sign driver with SignTool
   - Without proper signing, driver won't load on most systems

## Building

### Option 1: Visual Studio

1. Open `driver.c` in Visual Studio
2. Create new "Kernel Mode Driver, Empty (KMDF)" project
3. Add `driver.c` to project
4. Build → Build Solution (x64 Release)
5. Output: `RustDriver.sys`

### Option 2: Command Line (WDK)

```cmd
cd driver
"C:\Program Files (x86)\Windows Kits\10\bin\x64\cl.exe" /c /Fo"driver.obj" driver.c
"C:\Program Files (x86)\Windows Kits\10\bin\x64\link.exe" /DRIVER /ENTRY:DriverEntry /OUT:RustDriver.sys driver.obj
```

## Loading the Driver

### Method 1: Service Control Manager

```cmd
sc create RustDriver type= kernel binPath= C:\path\to\RustDriver.sys
sc start RustDriver
```

### Method 2: OSR Driver Loader

1. Download OSR Driver Loader: https://www.osronline.com/article.cfm%5Earticle=157.htm
2. Load `RustDriver.sys`
3. Start service

### Method 3: Manual Mapper (More Stealthy)

Use kernel manual mapping to avoid service registration:
- Load driver directly into kernel space
- Bypass PsLoadedModuleList enumeration
- Higher stealth, more complex

## Hiding from EAC

The driver includes several anti-detection features:

### 1. Driver Object Hiding
```c
HideDriver(DriverObject);
```
Unlinks from `PsLoadedModuleList` so EAC can't enumerate it.

### 2. Random Device Name
Change device name each build:
```c
UNICODE_STRING deviceName = RTL_CONSTANT_STRING(L"\\Device\\RandomName12345");
```

### 3. Polymorphic Build
Rebuild with different code layout to change signature:
```bash
python randomize_driver.py driver.c
```

## Post-Build Steps

### 1. Sign Driver (Required for Windows 10/11)

```cmd
signtool sign /v /s "My" /n "Your Certificate Name" /t http://timestamp.digicert.com RustDriver.sys
```

### 2. Disable Driver Signature Enforcement (Testing Only)

```cmd
bcdedit /set nointegritychecks on
```

**WARNING:** This disables system security. Use only for testing.

### 3. Load at Boot (Persistent)

```cmd
sc config RustDriver start= boot
```

Driver loads before EAC starts - higher chance of success.

## Troubleshooting

### Driver Won't Load

**Error:** "Code 52: Windows cannot verify the digital signature"
- Enable test signing mode
- Or properly sign the driver with EV certificate

**Error:** "Access denied"
- Run as Administrator
- Disable Secure Boot in BIOS

**Error:** "Driver load failed with BSOD"
- Driver has a bug (memory violation, invalid pointer)
- Check WinDbg crash dump
- Common issues: incorrect IRQL, null pointer dereference

### EAC Detects Driver

**Symptoms:** Game won't start, immediate kick
- Driver is blacklisted by signature
- Rebuild with different code layout
- Change device/driver names
- Add junk code/polymorphism

**Solution:** Polymorphic builder
```python
# randomize_driver.py
import random

def add_junk_code(source):
    junk = [
        "volatile int dummy{} = {};".format(i, random.randint(0, 1000))
        for i in range(20)
    ]
    return source + "\n".join(junk)
```

## Detection Vectors

EAC detects drivers through:

1. **Module enumeration** - Fixed by HideDriver()
2. **Callback monitoring** - Don't register callbacks
3. **Signature scanning** - Use polymorphic builds
4. **Behavioral analysis** - Minimize activity
5. **IOCTL fuzzing** - Validate all inputs

## Advanced: DMA Alternative

If kernel driver detection is too high, consider DMA card:
- Physical hardware (PCIe card)
- Reads RAM directly via DMA
- Zero software detection
- Costs $300-1000
- Requires second PC

Popular DMA cards:
- Squirrel DMA
- PCILeech
- DMA Screamer

## Production Deployment

For a real cheat:

1. Get EV certificate ($300-500)
2. Sign driver properly
3. Use vulnerable driver exploit (CVE-2020-XXXX)
4. Load via kdmapper or similar
5. Update driver weekly to stay ahead of blacklist
6. Use hardware spoofer on ban

**Expected lifespan:** 2-4 weeks before signature blacklist

## Legal Warning

Kernel driver development for anti-cheat bypass is:
- Against game Terms of Service
- May violate CFAA in some jurisdictions
- Can result in permanent hardware bans
- Distribution may have legal consequences

This is for educational purposes only.
