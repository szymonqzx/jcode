---
name: windows-api-validation
description: Windows API programming patterns, HRESULT error handling, and validation in Rust
---

# Windows API Development

Comprehensive patterns for Windows API programming in Rust using the windows crate, including HRESULT error handling, handle management, and validation techniques.

## When to Use
- Adding new Windows API calls to the codebase
- Modifying Win32 API usage or error handling
- Investigating Windows-specific bugs
- Reviewing code for Windows API best practices
- Supporting new Windows versions
- Auditing unsafe blocks and handle management
- Creating junction points or reparse points
- Managing process handles and lifecycle

## Key Patterns

### HRESULT Error Handling

```rust
use windows::Win32::Foundation::HRESULT;

let result = unsafe { SomeWindowsAPI() };
if !SUCCEEDED(result) {
    return Err(Error::from_hresult(result));
}
```

Always check HRESULT return values from Windows APIs. Use `SUCCEEDED()` and `FAILED()` macros or `.ok()` for conversion.

### Handle Management with scopeguard

```rust
use windows::Win32::Foundation::HANDLE;
use scopeguard::defer;

let handle = unsafe { CreateHandle() };
defer! {
    unsafe { CloseHandle(handle); }
}
```

Use scopeguard or similar patterns to ensure handles are closed even on early returns or panics.

### Junction Creation Windows APIs

```rust
use windows::Win32::Storage::FileSystem::*;

// Open directory handle for junction creation
let handle = unsafe {
    CreateFileW(
        target_path,
        GENERIC_READ | GENERIC_WRITE,
        FILE_SHARE_READ | FILE_SHARE_WRITE,
        None,
        OPEN_EXISTING,
        FILE_FLAG_BACKUP_SEMANTICS,
        HANDLE::default(),
    )
}?;

// Set reparse point data for junction
unsafe {
    DeviceIoControl(
        handle,
        FSCTL_SET_REPARSE_POINT,
        reparse_data,
        // ... additional parameters
    )
}?;
```

### Process Management Windows APIs

```rust
use windows::Win32::System::Threading::*;

// Open process handle for termination
let process_handle = unsafe {
    OpenProcess(PROCESS_TERMINATE, false, pid)
}?;

// Terminate process
unsafe {
    TerminateProcess(process_handle, 1);
}

// Close handle with scopeguard
use scopeguard::defer;
defer! {
    unsafe { let _ = CloseHandle(process_handle); }
}
```

### Safe Wrappers for Windows APIs

```rust
/// # Safety
///
/// Caller must ensure:
/// - The pointer is valid and aligned
/// - The memory is properly initialized
/// - No other thread is accessing this memory concurrently
unsafe fn windows_api_wrapper() -> Result<()> {
    let result = unsafe { DangerousWindowsAPI() };
    if !SUCCEEDED(result) {
        return Err(Error::from_hresult(result));
    }
    Ok(())
}
```

### Handle Management with RAII

```rust
use windows::Win32::Foundation::HANDLE;

// Implement Drop for custom handle types
struct ScopedHandle(HANDLE);

impl Drop for ScopedHandle {
    fn drop(&mut self) {
        unsafe { let _ = CloseHandle(self.0); }
    }
}
```

### UTF-16 String Conversion

```rust
use std::os::windows::ffi::OsStrExt;

fn to_wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}
```

Windows APIs often use UTF-16 (W suffix). Use proper encoding/decoding with `OsString`, `OsStr`, or manual UTF-16 conversion.

### Unsafe Block Documentation

```rust
/// # Safety
///
/// Caller must ensure:
/// - The pointer is valid and aligned
/// - The memory is properly initialized
/// - No other thread is accessing this memory concurrently
unsafe fn dangerous_function(ptr: *mut u8) {
    // ...
}
```

All unsafe blocks must have safety comments documenting the invariants that must be upheld.

### Windows Version Compatibility

```rust
// Check for API availability at runtime
use windows::Win32::Foundation::GetProcAddress;

let module = LoadLibraryW("kernel32.dll")?;
let func = GetProcAddress(module, w!("ReOpenFile"))?;
if func.is_null() {
    // API not available on this Windows version
    return Err(Error::UnsupportedWindowsVersion);
}
```

Document minimum Windows version requirements for version-specific APIs and handle unavailability gracefully.

## Validation Techniques

### 1. HRESULT Checking

Check all Windows API calls that return HRESULT:

```powershell
# Find unchecked HRESULT calls
rg "HRESULT" src/ --type rust -A 2 | rg -v "SUCCEEDED|FAILED|\.ok\(\)|unwrap\(\)|expect\("
```

### 2. Handle Leak Detection

Identify potentially unclosed Windows handles:

```powershell
# Find handle declarations without cleanup
rg ": HANDLE|: HWND|: HINSTANCE" src/ --type rust
# Verify each has corresponding CloseHandle/DestroyWindow/etc.
```

### 3. Version-Specific API Documentation

Check for APIs requiring specific Windows versions:

- `ReOpenFile` - Windows Vista+
- `GetFinalPathNameByHandleW` - Windows Vista+
- `CreateSymbolicLinkW` - Windows Vista+
- `SetFileInformationByHandle` - Windows Vista+

Document version requirements in code comments or documentation.

### 4. UTF-16 String Handling

Verify proper encoding for Windows APIs:

```powershell
# Find UTF-16 API calls
rg "W\(" src/ --type rust
# Check for proper encode_wide/encode_utf16 usage
```

### 5. Security API Checks

Avoid dangerous API patterns:

- `strcpy`, `strcat`, `sprintf` - Buffer overflows
- `CopyMemory`, `RtlCopyMemory` - No size checking
- `GetVersion()` - Deprecated
- `CreateFile.*GENERIC_ALL` - Overly permissive

### 6. Unsafe Block Validation

Ensure all unsafe blocks have safety comments:

```powershell
# Find undocumented unsafe blocks
rg "unsafe" src/ --type rust -B 1 -A 5
# Verify each has "# Safety:" or similar comment
```

## Edge Case Handling
- **API availability**: Windows version doesn't support API - check availability at runtime
- **Handle invalidation**: Handle becomes invalid after operation - recreate handle if needed
- **String encoding**: UTF-8 to UTF-16 conversion failures - handle encoding errors gracefully
- **Error propagation**: HRESULT errors need conversion to Rust errors - use consistent error types
- **Resource limits**: System resource exhaustion (handles, memory) - handle allocation failures

## Failure Modes
- **Handle leaks**: Not closing handles leads to resource exhaustion - use scopeguard or RAII
- **Buffer overflows**: Unsafe string operations cause memory corruption - use safe Rust alternatives
- **Race conditions**: Asynchronous operations without proper synchronization - use Windows synchronization primitives
- **Version incompatibility**: Using APIs not available on target Windows version - check version requirements
- **Invalid handles**: Using closed or invalid handles - validate handles before use

## Performance Considerations
- Handle reuse: Reuse handles when possible to reduce allocation overhead
- Batch operations: Group multiple API calls to reduce round-trips
- Async I/O: Use overlapped I/O for better performance with file/network operations
- Memory mapping: Use memory-mapped files for large file operations
- Caching: Cache frequently accessed system information

## Security Notes
- **Privilege escalation**: Ensure APIs are called with appropriate privileges
- **Symbolic link attacks**: Validate paths to prevent symlink attacks
- **Handle hijacking**: Protect handle inheritance to prevent unauthorized access
- **DLL injection**: Validate DLL loading paths to prevent malicious DLL loading
- **Token impersonation**: Carefully manage impersonation tokens to prevent privilege escalation

## Common Pitfalls

- **Unchecked HRESULT** - Forgetting to check Windows API return values
- **Handle leaks** - Not calling CloseHandle or equivalent cleanup functions
- **UTF-16 mishandling** - Incorrect string encoding for Windows APIs expecting wide strings
- **Missing safety comments** - Unsafe blocks without documenting safety invariants
- **Version assumptions** - Using APIs without checking Windows version or documenting requirements
- **Buffer overflows** - Using unsafe C-style string functions instead of safe Rust alternatives
- **Race conditions** - Not handling asynchronous Windows operations properly

## Best Practices

1. **Always check HRESULT** - Never ignore Windows API return values
2. **Use scopeguard for handles** - Ensure cleanup even on early returns
3. **Document unsafe blocks** - Every unsafe block needs a safety comment
4. **Check Windows version** - Handle version-specific APIs gracefully
5. **Prefer safe APIs** - Use Rust-safe alternatives when available
6. **Test on multiple Windows versions** - Verify compatibility across Windows 10/11
7. **Use Windows SDK tools** - Application Verifier, DebugDiag for deep debugging

## Focus Areas for Windows API Development

When working with Windows APIs, prioritize:

1. **Process management** - Spawning, monitoring, terminating processes
2. **File system operations** - Junctions, symlinks, reparse points
3. **Handle management** - Proper cleanup and RAII patterns
4. **Error handling** - HRESULT conversion and context preservation
5. **String encoding** - UTF-16/UTF-8 conversion for Windows APIs
6. **Version compatibility** - Check API availability and document requirements
7. **Safety documentation** - Document invariants for all unsafe blocks

## Related Workflows

- `../workflows/code-fix-loop.md` - For fixing Windows API issues found during validation
- `../workflows/debug.md` - For debugging Windows handle leaks
- `../workflows/code-review-checklist/SKILL.md` - For reviewing Windows API usage

## Related Skills

- `../skills/error-handling/SKILL.md` - For HRESULT error handling patterns
- `../skills/filesystem-operations/SKILL.md` - For junction/symlink operations
- `../skills/rust-pro/SKILL.md` - For general Rust systems programming
