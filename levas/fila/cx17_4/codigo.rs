// ---------- cx17_4: the South American qualifying's and the World Cup's own data in the saved game (wc_south_american_league.cpp, world_cup.cpp) ----------

/// Reads the South American qualifying's own data from the saved game (`exe::crt_fread`, cdecl): the exempt count
/// (`wc_south_american_league_fields::EXEMPT_COUNT`, a byte); short, the error box of line 0x2ff and 0. Returns 1.
/// Original: `WcSouthAmericanLeague::load_extra_data` (0x921590, wc_south_american_league.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn wc_south_american_league_load_extra_data(this: usize, file: usize) -> u32 {
    if exe::crt_fread(this + wc_south_american_league_fields::EXEMPT_COUNT, 1, 1, file) != 1 {
        show_error_at(wc_turnover_consts::WC_SOUTH_AMERICAN_LEAGUE_CPP, 0x2ff);
        return 0;
    }
    covered!("cx17_4 south american qualifying loaded");
    1
}

/// The World Cup's saved fields, in the order the game writes and reads them: the 32 draw slots of 5 bytes
/// (`world_cup_fields::DRAW_SLOT_CLUB`, club and pot), the seeding day and its year offset (a word each).
const WORLD_CUP_SAVED_FIELDS: [(usize, u32, u32); 3] = [(world_cup_fields::DRAW_SLOT_CLUB, 5, 0x20), (world_cup_fields::SEEDING_DAY, 2, 1), (world_cup_fields::SEEDING_YEAR_OFFSET, 2, 1)];

/// Reads the World Cup's own data from the saved game (`exe::crt_fread`, cdecl), the fields of `WORLD_CUP_SAVED_FIELDS`
/// in order; a short read shows the world_cup.cpp box (lines 0x53e, 0x546 and 0x54d) and returns 0. Returns 1.
/// Original: `WorldCup::load_extra_data` (0x92f000, world_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn world_cup_load_extra_data(this: usize, file: usize) -> u32 {
    for ((at, size, count), line) in WORLD_CUP_SAVED_FIELDS.into_iter().zip([0x53e, 0x546, 0x54d]) {
        if exe::crt_fread(this + at, size, count, file) != count {
            show_error_at(world_cup_knockout_consts::WORLD_CUP_CPP, line);
            return 0;
        }
    }
    covered!("cx17_4 world cup loaded");
    1
}

/// Writes the World Cup's own data to the saved game (`exe::crt_fwrite`, cdecl, unchecked), the fields of
/// `WORLD_CUP_SAVED_FIELDS` in order. Returns what the last write returned.
/// Original: `WorldCup::save_extra_data` (0x92f150, world_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn world_cup_save_extra_data(this: usize, file: usize) -> u32 {
    covered!("cx17_4 world cup saved");
    let mut r = 0;
    for (at, size, count) in WORLD_CUP_SAVED_FIELDS {
        r = exe::crt_fwrite(this + at, size, count, file); // cx17_4: the World Cup's fields
    }
    r
}

/// Comparer of a reader of fixed fields: the original reads first; the file goes back to where it was, the fields to
/// what they held, and the copy reads the same bytes again; the returns, the `fields` bytes and the file positions must
/// match. Then the original's fields and position are put back.
pub(crate) unsafe fn compare_fixed_fields_load(name: &str, this: usize, file: usize, fields: &[(usize, usize)], original: impl FnOnce() -> u32, copy: impl FnOnce() -> u32) -> u32 {
    let snapshot = || -> Vec<u8> { fields.iter().flat_map(|&(o, n)| core::slice::from_raw_parts((this + o) as *const u8, n).to_vec()).collect() };
    let restore = |bytes: &[u8]| {
        let mut k = 0;
        for &(o, n) in fields {
            core::ptr::copy_nonoverlapping(bytes.as_ptr().add(k), (this + o) as *mut u8, n);
            k += n;
        }
    };
    let before = snapshot();
    let at = exe::crt_ftell(file);
    let r = original();
    let (end_o, a) = (exe::crt_ftell(file), snapshot());
    restore(&before);
    exe::crt_fseek(file, at, 0);
    let c = copy();
    let (end_c, b) = (exe::crt_ftell(file), snapshot());
    restore(&a);
    exe::crt_fseek(file, end_o, 0);
    SCOREBOARD.record(r == c && a == b && end_o == end_c, || format!("{}({:#x}): original {} {:x?} at {}, copy {} {:x?} at {} (from {})", name, this, r, a, end_o, c, b, end_c, at));
    r
}

pub(crate) unsafe extern "thiscall" fn compare_wc_south_american_league_load_extra_data(this: usize, file: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%);
    compare_fixed_fields_load("wc_south_american_league_load_extra_data", this, file, &[(wc_south_american_league_fields::EXEMPT_COUNT, 1)],
        || o(this, file), || wc_south_american_league_load_extra_data(this, file))
}

pub(crate) unsafe extern "thiscall" fn compare_world_cup_load_extra_data(this: usize, file: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I1%%);
    compare_fixed_fields_load("world_cup_load_extra_data", this, file, &[(world_cup_fields::DRAW_SLOT_CLUB, 0xa0), (world_cup_fields::SEEDING_DAY, 4)],
        || o(this, file), || world_cup_load_extra_data(this, file))
}

/// Comparer for `world_cup_save_extra_data`: it writes the saved game (it cannot run twice), so only the original runs;
/// the copy is proven by the save proof.
pub(crate) unsafe extern "thiscall" fn compare_world_cup_save_extra_data(this: usize, file: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I2%%)(this, file)
}
