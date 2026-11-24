// ---------- batch %%LEVA%%: the ability order and the game's static dates (game.cpp) ----------

/// `compare_shorts_ascending(a, b)`: qsort comparison of two 16-bit values, ascending: `*a − *b`, both sign-extended.
/// Used by the daily person pass (`game_daily_staff_update` 0x5ce2d0) to sort the two tables of the day's abilities
/// (current and potential) that the players' development reads.
/// Original: `compare_shorts_ascending` (0x5d6200, game.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_shorts_ascending(a: *const i16, b: *const i16) -> i32 {
    read_i16(a as usize) as i32 - read_i16(b as usize) as i32
}

/// Comparer for `compare_shorts_ascending`: the order of the original and of the copy.
pub(crate) unsafe extern "C" fn compare_compare_shorts_ascending(a: *const i16, b: *const i16) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(*const i16, *const i16) -> i32>(%%I0%%)(a, b);
    let n = compare_shorts_ascending(a, b);
    SCOREBOARD.record(r == n, || format!("compare_shorts_ascending({}, {}) original={} rust={}", read_i16(a as usize), read_i16(b as usize), r, n));
    r
}

/// Builds a game date with the default `Date` (`exe::date_date_54a110`: day 40 of 1972).
unsafe fn construct_static_date(date: usize) {
    exe::date_date_54a110(date);
}

/// `game_date_static_init()`: static initialiser of the exe (runs in the CRT start-up, before WinMain, with the detours
/// already installed by the DLL load): builds the game date `G_CURRENT_DATE` with the default `Date` and registers its
/// destructor (`game_date_static_dtor` 0x5cc670, an empty body) with `exe::crt_atexit`. The game date is set for real
/// when a game is created or loaded.
/// Original: `game_date_static_init` (0x5cc650, game.cpp, cdecl).
pub(crate) unsafe extern "C" fn game_date_static_init() {
    construct_static_date(G_CURRENT_DATE);
    exe::crt_atexit(0x5cc670usize); // game_date_static_dtor: an empty body
}

/// `last_processed_date_static_init()`: static initialiser of the exe, like `game_date_static_init`, for the last
/// processed date `G_LAST_PROCESSED_DATE` (destructor `last_processed_date_static_dtor` 0x5cc6a0, an empty body).
/// Original: `last_processed_date_static_init` (0x5cc680, game.cpp, cdecl).
pub(crate) unsafe extern "C" fn last_processed_date_static_init() {
    construct_static_date(G_LAST_PROCESSED_DATE);
    exe::crt_atexit(0x5cc6a0usize); // last_processed_date_static_dtor: an empty body
}

/// Comparer of a static date initialiser: the original runs (it registers the destructor once); the copy builds the date
/// into a local and its 8 bytes are compared with the global's.
unsafe fn compare_static_date(name: &str, original: unsafe extern "C" fn(), global: usize) {
    original();
    let mut copy = [0u16; 4];
    construct_static_date(copy.as_mut_ptr() as usize);
    let orig: [u16; 4] = core::array::from_fn(|k| read_u16(global + 2 * k));
    SCOREBOARD.record(copy == orig, || format!("{}: original={:x?} rust={:x?}", name, orig, copy));
}

/// Comparer for `game_date_static_init`.
pub(crate) unsafe extern "C" fn compare_game_date_static_init() {
    compare_static_date("game_date_static_init", original_fn::<unsafe extern "C" fn()>(%%I1%%), G_CURRENT_DATE);
}

/// Comparer for `last_processed_date_static_init`.
pub(crate) unsafe extern "C" fn compare_last_processed_date_static_init() {
    compare_static_date("last_processed_date_static_init", original_fn::<unsafe extern "C" fn()>(%%I2%%), G_LAST_PROCESSED_DATE);
}
