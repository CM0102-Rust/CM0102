// ---------- cx05_15: the European Championship, mini cup and mini league data read from the saved game ----------

/// Reads the European Championship's own data from its temporary file: the seeded list (`exe::crt_fread` of 16
/// entries of 5 bytes at +0xee; short: the box at line 0x404 and 0), the seeding day (2 bytes; short: line 0x40c and
/// 0) and the seeding year offset (2 bytes; short: line 0x413 and 0). Returns 1.
/// Original: `EuroChamp::load_extra_data` (0x57c480, euro_champ.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn euro_champ_load_extra_data(this: usize, file: usize) -> u32 {
    if exe::crt_fread(this + 0xee, 5, 0x10, file) != 0x10 { // cx05_15: the 16 seeded nations
        show_error_at(0x9c4638, 0x404);
        return 0;
    }
    if exe::crt_fread(this + EURO_CHAMP_SEEDING_DAY, 2, 1, file) != 1 {
        show_error_at(0x9c4638, 0x40c);
        return 0;
    }
    if exe::crt_fread(this + EURO_CHAMP_SEEDING_YEAR_OFFSET, 2, 1, file) != 1 {
        show_error_at(0x9c4638, 0x413);
        return 0;
    }
    covered!("cx05_15 euro loaded");
    1
}

/// Reads the mini cup's own data from its temporary file: the start date (8 bytes at +0xb2) and the tie rules word (2
/// bytes at +0xba); a short read shows the box (lines 0x13b and 0x143) and ends the game (`fatal_error_at`). Returns 1.
/// Original: `MiniCup::load_extra_data` (0x766380, mini_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn mini_cup_load_extra_data(this: usize, file: usize) -> u32 {
    if exe::crt_fread(this + 0xb2, 8, 1, file) != 1 { // cx05_15: the mini cup's start date
        fatal_error_at(0xa5ab00, 0x13b);
    }
    if exe::crt_fread(this + 0xba, 2, 1, file) != 1 {
        fatal_error_at(0xa5ab00, 0x143);
    }
    covered!("cx05_15 mini cup loaded");
    1
}

/// Reads the mini league's own data from its temporary file: the start date (8 bytes at +0xee); short: the box at line
/// 0x116 and 0. Returns 1.
/// Original: `MiniLeague::load_extra_data` (0x766cf0, mini_league.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn mini_league_load_extra_data(this: usize, file: usize) -> u32 {
    if exe::crt_fread(this + 0xee, 8, 1, file) != 1 { // cx05_15: the mini league's start date
        show_error_at(0xa5ab3c, 0x116);
        return 0;
    }
    covered!("cx05_15 mini league loaded");
    1
}

/// Comparers of the readers: side by side with the file put back at its position (the same bytes read twice), the
/// fields, the result and the final position (`compare_cup_extra_data_load` of cx02_13).
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_load_extra_data(this: usize, file: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%);
    compare_cup_extra_data_load("euro_champ_load_extra_data", this, file, &[(0xee, 0x54)], || o(this, file), || euro_champ_load_extra_data(this, file))
}

pub(crate) unsafe extern "thiscall" fn compare_mini_cup_load_extra_data(this: usize, file: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I1%%);
    compare_cup_extra_data_load("mini_cup_load_extra_data", this, file, &[(0xb2, 0xa)], || o(this, file), || mini_cup_load_extra_data(this, file))
}

pub(crate) unsafe extern "thiscall" fn compare_mini_league_load_extra_data(this: usize, file: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I2%%);
    compare_cup_extra_data_load("mini_league_load_extra_data", this, file, &[(0xee, 8)], || o(this, file), || mini_league_load_extra_data(this, file))
}
