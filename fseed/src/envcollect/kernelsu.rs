//! Detection and interaction logic for the KernelSU root solution.
//!
//! This module provides a unified API to interface with KernelSU, abstracting away the
//! underlying communication details. It is designed with backwards compatibility in mind,
//! supporting both the modern `ioctl` interface and the legacy `prctl` interface.
//!
//! The core design revolves around a one-shot detection mechanism. On the first call to
//! any function in this module, `detect_version`'s initialization logic runs once.
//! It first attempts to connect using the modern `ioctl` method. If that fails, it falls
//! back to the legacy `prctl` method. The result of this detection—including the determined
//! communication method and version status—is cached globally for the lifetime of the process.
//! All subsequent calls are then dispatched to the correct implementation instantly,
//! avoiding any repeated detection overhead.

use std::ffi::c_char;
use std::fs;
use std::os::fd::RawFd;
use std::path::Path;
use std::sync::OnceLock;

// --- KernelSU Communication Method Enum & Cached State ---

/// Represents the complete, immutable result of the one-time detection process.
/// Caching this entire struct ensures all necessary information is available atomically.
#[derive(Clone, Copy)]
struct DetectionResult {
    /// The determined version status of KernelSU.
    version: u32,
}

/// Lazily initialized detection result. This `OnceLock` is the cornerstone of the module's
/// one-shot detection and caching strategy. It ensures that the detection logic is
/// thread-safe and executes exactly once.
static KSU_RESULT: OnceLock<Option<DetectionResult>> = OnceLock::new();

// --- Modern `ioctl` Interface Constants and Structs ---

/// Magic numbers used in the `reboot` syscall to request a KernelSU driver file descriptor.
const KSU_INSTALL_MAGIC1: u32 = 0xDEADBEEF;
const KSU_INSTALL_MAGIC2: u32 = 0xCAFEBABE;

/*
 * --- How to Calculate IOCTL Command Numbers ---
 * The ioctl command number is a 32-bit integer encoded with information about the
 * direction of data transfer, a magic 'type' character, and a sequence number. The size
 * field is deliberately set to 0 by KernelSU's driver, bypassing the kernel's size check.
 *
 * The formula, from `<asm-generic/ioctl.h>`, is:
 *   (((dir) << 30) | ((type) << 8) | (nr) | ((size) << 16))
 *
 * - dir: Data direction (_IOC_NONE=0, _IOC_WRITE=1, _IOC_READ=2).
 * - type: An 8-bit magic number. For KernelSU, this is 'K' (ASCII 75).
 * - nr: The 8-bit command sequence number.
 * - size: The size of the argument. KernelSU explicitly uses 0.
 *
 * Example Calculation for KSU_IOCTL_GET_INFO:
 * - C Definition: _IOC(_IOC_READ, 'K', 2, 0)
 * - dir=2, type='K'=75, nr=2, size=0
 * - Value = (2 << 30) | (75 << 8) | 2 = 0x80004B02
 */

// Calculated IOCTL command codes, matching `supercalls.h`'s use of _IOC(..., 0).
const KSU_IOCTL_GET_INFO: u32 = 0x80004B02;          // nr=2, dir=R

/// Data structures for ioctl commands.
/// The `#[repr(C)]` attribute is critical to ensure that the memory layout of these
/// Rust structs is identical to their C counterparts in the kernel, preventing data
/// corruption during FFI calls.
#[repr(C)]
struct KsuGetInfoCmd {
    version: u32,
    flags: u32,
    features: u32,
}

// --- Legacy `prctl` Interface Constants ---

/// The magic number identifying KernelSU-specific prctl commands.
const KERNEL_SU_OPTION: i32 = 0xdeadbeefu32 as i32;
/// prctl command codes for the legacy interface.
const CMD_GET_VERSION: usize = 2;
const CMD_GET_MANAGER_UID: usize = 16;
/// Special prctl command to detect KernelSU variants (e.g., "Next").
const CMD_HOOK_MODE: usize = 0xC0DEAD1A;

/// Represents detected variants of KernelSU, which had slightly different behavior
/// in the legacy prctl implementation.
#[derive(Clone, Copy, Debug)]
enum KernelSuVariant {
    Official,
    Next,
}

/// Lazily initialized variant for the legacy prctl method. Only used if fallback occurs.
static LEGACY_VARIANT: OnceLock<KernelSuVariant> = OnceLock::new();
/// Lazily initialized capability flag for the legacy prctl method. Only used if fallback occurs.
static LEGACY_SUPPORTS_MANAGER_UID: OnceLock<bool> = OnceLock::new();

// --- Core Detection and Dispatch Logic ---

/// Detects if KernelSU is active and its version, determining the correct communication method.
/// This function implements the "ioctl-first, prctl-fallback" strategy and caches the result.
pub fn detect() -> Option<u32> {
    // `get_or_init` ensures the complex detection logic within the closure runs exactly once.
    // The closure's return value of type `Option<DetectionResult>` is then cached in `KSU_RESULT`.
    let result = KSU_RESULT.get_or_init(|| {
        // --- Stage 1: Attempt to use the modern ioctl interface ---
        // This is the preferred method for modern KernelSU versions.
        if let Some(fd) = init_driver_fd() {
            let mut cmd = KsuGetInfoCmd {
                version: 0,
                flags: 0,
                features: 0,
            };
            if ksuctl_ioctl(fd, KSU_IOCTL_GET_INFO, &mut cmd).is_ok() {
                let version_code = cmd.version;
                if version_code > 0 {
                    if Path::new("/data/adb/ksud").exists() {
                        // Version is supported and ksud exists. Cache the result and finish.
                        return Some(DetectionResult {
                            version: version_code
                        });
                    }
                }
            }
        }

        // --- Stage 2: Fallback to the legacy prctl interface ---
        // This block only executes if the ioctl method fails to yield a valid result.
        let mut version_code = 0;
        unsafe {
            // Safety: This is an FFI call. We provide a valid pointer to a stack variable.
            libc::prctl(
                KERNEL_SU_OPTION,
                CMD_GET_VERSION,
                &mut version_code as *mut i32,
                0,
                0,
            );
        }

        if version_code > 0 {
            // Success with prctl. We must now probe for legacy capabilities.
            init_legacy_variant_probe();
            return Some(DetectionResult {
                version: version_code as u32,
            });
        }

        // --- Stage 3: Failure ---
        // If both the ioctl and prctl methods fail, KernelSU is not detected.
        None
    });

    result.as_ref().map(|r| r.version)
}

// --- `ioctl` Implementation Details ---

/// Scans `/proc/self/fd` to find an existing driver file descriptor.
/// This is an important optimization to avoid the `reboot` syscall if the fd
/// has already been opened, for example, by a parent process.
fn scan_driver_fd() -> Option<RawFd> {
    let fd_dir = fs::read_dir("/proc/self/fd").ok()?;
    for entry in fd_dir.flatten() {
        if let Ok(target) = fs::read_link(entry.path()) {
            if target.to_string_lossy().contains("[ksu_driver]") {
                return entry.file_name().to_string_lossy().parse().ok();
            }
        }
    }
    None
}

/// Initializes the driver file descriptor. It first attempts to scan for an
/// existing one and falls back to the `reboot` syscall "secret knock" if none is found.
fn init_driver_fd() -> Option<RawFd> {
    if let Some(fd) = scan_driver_fd() {
        return Some(fd);
    }

    let mut fd: RawFd = -1;
    unsafe {
        // Safety: This is a raw syscall. The kernel expects specific magic numbers
        // and a valid pointer to write the resulting file descriptor into.
        libc::syscall(
            libc::SYS_reboot,
            KSU_INSTALL_MAGIC1,
            KSU_INSTALL_MAGIC2,
            0,
            &mut fd,
        );
    }
    if fd >= 0 { Some(fd) } else { None }
}

/// A safe, generic wrapper around the `ioctl` syscall, matching the style of the
/// official KernelSU Manager for consistency.
fn ksuctl_ioctl<T>(fd: RawFd, request: u32, arg: *mut T) -> std::io::Result<()> {
    // Safety: FFI call. `fd` must be a valid file descriptor, and `arg` must
    // point to a valid memory region for a `#[repr(C)]` struct.
    let ret = unsafe { libc::ioctl(fd, request as _, arg) };
    if ret < 0 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(())
    }
}

// --- `prctl` Implementation Details ---

/// Probes and caches capabilities for the legacy prctl method.
/// This is necessary because the old API was not unified, and different KernelSU
/// versions or variants had different feature sets that must be detected at runtime.
fn init_legacy_variant_probe() {
    LEGACY_VARIANT.get_or_init(|| {
        let mut mode: [c_char; 16] = [0; 16];
        unsafe {
            // Safety: FFI call. `mode.as_mut_ptr()` provides a valid buffer.
            libc::prctl(
                KERNEL_SU_OPTION,
                CMD_HOOK_MODE,
                mode.as_mut_ptr() as usize,
                0,
                0,
            );
        }
        if mode[0] != 0 {
            KernelSuVariant::Next
        } else {
            KernelSuVariant::Official
        }
    });

    LEGACY_SUPPORTS_MANAGER_UID.get_or_init(|| {
        let mut result_ok: i32 = 0;
        unsafe {
            // Safety: FFI call. We provide a valid pointer to check for support.
            libc::prctl(
                KERNEL_SU_OPTION,
                CMD_GET_MANAGER_UID,
                0,
                0,
                &mut result_ok as *mut _ as usize,
            );
        }
        // The prctl interface confirms support by writing back the magic number.
        result_ok as u32 == KERNEL_SU_OPTION as u32
    });
}