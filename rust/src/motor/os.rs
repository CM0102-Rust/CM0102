use super::*;

/// The total physical memory in MB: `dwTotalPhys` as a signed number divided by 2^20 (rounded toward zero).
/// Original: `os_get_total_physical_memory_mb` (0x7a16d0, os.cpp, cdecl).
pub(crate) unsafe extern "C" fn os_get_total_physical_memory_mb() -> i32 {
    os_memory_status()[2] as i32 / 0x100000
}

/// Comparer for `os_get_total_physical_memory_mb`: the same value.
pub(crate) unsafe extern "C" fn compare_os_get_total_physical_memory_mb() -> i32 {
    let r = original_fn::<unsafe extern "C" fn() -> i32>(IDX_OS_GET_TOTAL_PHYSICAL_MEMORY_MB)();
    let n = os_get_total_physical_memory_mb();
    SCOREBOARD.record(r == n, || format!("os_get_total_physical_memory_mb: original={} rust={}", r, n));
    r
}

/// The free memory in MB for the game's checks: `dwAvailPhys` + `dwAvailPageFile` (32-bit sum) shifted by 20.
/// Original: `os_get_free_memory_mb` (0x7a1710, os.cpp, cdecl).
pub(crate) unsafe extern "C" fn os_get_free_memory_mb() -> u32 {
    let m = os_memory_status();
    m[3].wrapping_add(m[5]) >> 20
}

/// Comparer for `os_get_free_memory_mb` (the reading may move between the calls).
pub(crate) unsafe extern "C" fn compare_os_get_free_memory_mb() -> u32 {
    let o = original_fn::<unsafe extern "C" fn() -> u32>(IDX_OS_GET_FREE_MEMORY_MB);
    compare_os_reading("os_get_free_memory_mb", || o(), || os_get_free_memory_mb())
}

/// The CPU speed class of the machine for the default settings: 0 fast, 1 middle, 2 slow.
///
/// The speed in MHz from `cpuinf32.dll` in CM3_DATA (`utils_build_file_path`, the path copied into the shared buffer
/// 0xde1f64 with "%s", `LoadLibraryA`, its `cpuspeed(&scratch, 0)`, then `FreeLibrary`): the third dword of the
/// answer, or 150 when the first two are 0 or the library does not load. Then `GetSystemInfo`: a processor other than
/// a 586 (386, 486, anything else) is slow (2); a 586 above 300 MHz is fast (0), up to 200 slow (2), between them 1
/// (the speed compared as a signed word).
/// Defect of the original: `GetProcAddress` is not checked (a library without `cpuspeed` crashes the game).
/// Original: `os_get_cpu_speed_class` (0x7a1750, os.cpp, cdecl, the answer in al).
pub(crate) unsafe extern "C" fn os_get_cpu_speed_class() -> u8 {
    let load: unsafe extern "system" fn(usize) -> usize = core::mem::transmute(read_usize(os_imports::LOAD_LIBRARY_A));
    let proc_address: unsafe extern "system" fn(usize, usize) -> usize = core::mem::transmute(read_usize(os_imports::GET_PROC_ADDRESS));
    let free: unsafe extern "system" fn(usize) -> u32 = core::mem::transmute(read_usize(os_imports::FREE_LIBRARY));
    let system_info: unsafe extern "system" fn(*mut u32) = core::mem::transmute(read_usize(os_imports::GET_SYSTEM_INFO));
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let path = exe::utils_build_file_path(0x9b5348usize, 0xa6a3ccusize); // "CM3_DATA", "cpuinf32.dll"
    sprintf(GUI_SCRATCH_TEXT, 0x991944, path); // "%s"
    let library = load(GUI_SCRATCH_TEXT);
    let mut speed = OS_UNKNOWN as i16;
    if library != 0 {
        let cpuspeed: unsafe extern "C" fn(*mut u32, u32) -> *const u32 = core::mem::transmute(proc_address(library, 0xa6a3c0)); // "cpuspeed"
        let mut scratch = [0u32; 8];
        let answer = cpuspeed(scratch.as_mut_ptr(), 0);
        let (first, second) = (*answer, *answer.add(1));
        speed = *answer.add(2) as i16;
        free(library);
        if first == 0 && second == 0 {
            speed = OS_UNKNOWN as i16;
        }
    }
    let mut info = [0u32; 9];
    system_info(info.as_mut_ptr());
    match info[6] {
        0x24a if speed > 0x12c => 0, // a 586 above 300 MHz
        0x24a => 1 + (speed <= 0xc8) as u8,
        _ => 2,
    }
}

/// Comparer for `os_get_cpu_speed_class`: the same class (the speed is measured again; the classes are wide).
pub(crate) unsafe extern "C" fn compare_os_get_cpu_speed_class() -> u8 {
    let o = original_fn::<unsafe extern "C" fn() -> u8>(IDX_OS_GET_CPU_SPEED_CLASS);
    compare_os_reading("os_get_cpu_speed_class", || o(), || os_get_cpu_speed_class())
}

/// The free disk space of a path in MB.
///
/// `GetVersionExA` (148 bytes) failing: 150. Windows NT, or Windows 9x above 4.0 build 1000: `GetDiskFreeSpaceExA`
/// looked up in KERNEL32.DLL (`LoadLibraryA`, `GetProcAddress`); the space free to the caller shifted by 20, or 4096
/// when it is 4 GB or more; the library freed. The old Windows 95: `GetDiskFreeSpaceA` (sectors per cluster × bytes
/// per sector × free clusters, 32-bit, shifted by 20). Any other platform: 0. A failure shows the os.cpp "Error" box
/// (no KERNEL32 line 0x13d, no function 0x14b, the call 0x158) and answers 150; the old call failing answers 150 without
/// a box.
/// Original: `os_get_free_disk_space_mb` (0x7a1820, os.cpp, cdecl).
pub(crate) unsafe extern "C" fn os_get_free_disk_space_mb(path: usize) -> u32 {
    let version_ex: unsafe extern "system" fn(*mut u32) -> u32 = core::mem::transmute(read_usize(os_imports::GET_VERSION_EX_A));
    let mut version = [0u32; 0x25];
    version[0] = 0x94;
    if version_ex(version.as_mut_ptr()) == 0 {
        return OS_UNKNOWN;
    }
    let (major, minor, build, platform) = (version[1], version[2], version[3] & 0xffff, version[4]);
    let old_95 = platform == 1 && (major < 4 || (major == 4 && minor == 0)) && build <= 0x3e8;
    if platform != 2 && platform != 1 {
        return 0;
    }
    if old_95 {
        covered!("sx16_6 old Windows 95 disk space");
        let free_space: unsafe extern "system" fn(usize, *mut u32, *mut u32, *mut u32, *mut u32) -> u32 = core::mem::transmute(read_usize(os_imports::GET_DISK_FREE_SPACE_A));
        let (mut per_cluster, mut per_sector, mut clusters, mut total) = (0u32, 0u32, 0u32, 0u32);
        if free_space(path, &mut per_cluster, &mut per_sector, &mut clusters, &mut total) == 0 {
            return OS_UNKNOWN;
        }
        return per_cluster.wrapping_mul(per_sector).wrapping_mul(clusters) >> 20;
    }
    let load: unsafe extern "system" fn(usize) -> usize = core::mem::transmute(read_usize(os_imports::LOAD_LIBRARY_A));
    let proc_address: unsafe extern "system" fn(usize, usize) -> usize = core::mem::transmute(read_usize(os_imports::GET_PROC_ADDRESS));
    let free: unsafe extern "system" fn(usize) -> u32 = core::mem::transmute(read_usize(os_imports::FREE_LIBRARY));
    let kernel = load(0xa6a3f0); // "KERNEL32.DLL"
    if kernel == 0 {
        show_error_box(OS_CPP_PATH, 0x13d);
        return OS_UNKNOWN;
    }
    let function = proc_address(kernel, 0xa6a3dc); // "GetDiskFreeSpaceExA"
    if function == 0 {
        show_error_box(OS_CPP_PATH, 0x14b);
        free(kernel);
        return OS_UNKNOWN;
    }
    let free_space_ex: unsafe extern "system" fn(usize, *mut u64, *mut u64, *mut u64) -> u32 = core::mem::transmute(function);
    let (mut available, mut total, mut free_total) = (0u64, 0u64, 0u64);
    if free_space_ex(path, &mut available, &mut total, &mut free_total) == 0 {
        show_error_box(OS_CPP_PATH, 0x158);
        free(kernel);
        return OS_UNKNOWN;
    }
    covered!("sx16_6 disk space");
    free(kernel);
    if available >> 32 != 0 { 0x1000 } else { (available as u32) >> 20 }
}

/// Comparer for `os_get_free_disk_space_mb` (the reading may move between the calls; the error boxes only through
/// the original, which shows them first).
pub(crate) unsafe extern "C" fn compare_os_get_free_disk_space_mb(path: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_OS_GET_FREE_DISK_SPACE_MB);
    compare_os_reading("os_get_free_disk_space_mb", || o(path), || os_get_free_disk_space_mb(path))
}

/// `GlobalMemoryStatus` into a MEMORYSTATUS (32 bytes; only the length is set before the call, as in the original).
pub(crate) unsafe fn os_memory_status() -> [u32; 8] {
    let status: unsafe extern "system" fn(*mut u32) = core::mem::transmute(read_usize(os_imports::GLOBAL_MEMORY_STATUS));
    let mut m = [0u32; 8];
    m[0] = 0x20;
    status(m.as_mut_ptr());
    m
}

/// The comparer of a machine reading that may move between two calls (free memory, free disk): the original, the copy
/// and the original again; the copy must give one of the two original answers.
pub(crate) unsafe fn compare_os_reading<R: PartialEq + Copy + core::fmt::Debug>(name: &str, original: impl Fn() -> R, rust: impl Fn() -> R) -> R {
    let r = original();
    let n = rust();
    let again = if n == r { r } else { original() };
    SCOREBOARD.record(n == r || n == again, || format!("{}: original={:?}/{:?} rust={:?}", name, r, again, n));
    r
}

/// The source path of os.cpp in the "Error" box ("E:\dev\CM3\cm3 00-01\si\code\os.cpp").
pub(crate) const OS_CPP_PATH: usize = 0xa6a39c;

/// What the machine checks answer when they cannot measure: 150 (MB of disk, MHz of CPU).
pub(crate) const OS_UNKNOWN: u32 = 0x96;
