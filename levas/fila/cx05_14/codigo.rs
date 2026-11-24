// ---------- cx05_14: the European Championship, mini cup and mini league data written to the saved game ----------

/// The European Championship's seeded list (16 entries of 5 bytes) and its seeding date (`EURO_CHAMP_SEEDING_DAY`,
/// `EURO_CHAMP_SEEDING_YEAR_OFFSET`).
const EURO_CHAMP_SEEDED_LIST: usize = 0xee;

/// Writes the European Championship's own data into its temporary file: the seeded list (`exe::crt_fwrite` of 16
/// entries of 5 bytes), then the seeding day and the seeding year offset (2 bytes each), in the order the load reads
/// them. Returns the last fwrite's result (the original leaves it in eax).
/// Original: `EuroChamp::save_extra_data` (0x57c5d0, euro_champ.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn euro_champ_save_extra_data(this: usize, file: usize) -> u32 {
    covered!("cx05_14 euro saved");
    exe::crt_fwrite(this + EURO_CHAMP_SEEDED_LIST, 5, 0x10, file);
    exe::crt_fwrite(this + EURO_CHAMP_SEEDING_DAY, 2, 1, file); // cx05_14: the seeding day
    exe::crt_fwrite(this + EURO_CHAMP_SEEDING_YEAR_OFFSET, 2, 1, file)
}

/// Writes the mini cup's own data into its temporary file: the start date (8 bytes) and the tie rules word (2 bytes).
/// Returns the last fwrite's result.
/// Original: `MiniCup::save_extra_data` (0x7664c0, mini_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn mini_cup_save_extra_data(this: usize, file: usize) -> u32 {
    covered!("cx05_14 mini cup saved");
    exe::crt_fwrite(this + 0xb2, 8, 1, file); // cx05_14: the mini cup's start date
    exe::crt_fwrite(this + 0xba, 2, 1, file) // the tie rules word
}

/// Writes the mini league's own data into its temporary file: the start date (8 bytes). Returns the fwrite's result.
/// Original: `MiniLeague::save_extra_data` (0x766d90, mini_league.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn mini_league_save_extra_data(this: usize, file: usize) -> u32 {
    covered!("cx05_14 mini league saved");
    exe::crt_fwrite(this + 0xee, 8, 1, file) // cx05_14: the mini league's start date
}

/// Comparers of the writers: they write the saved game (they cannot run twice), so only the original runs; the copy
/// is proven by the save proof.
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_save_extra_data(this: usize, file: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%)(this, file)
}

pub(crate) unsafe extern "thiscall" fn compare_mini_cup_save_extra_data(this: usize, file: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I1%%)(this, file)
}

pub(crate) unsafe extern "thiscall" fn compare_mini_league_save_extra_data(this: usize, file: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I2%%)(this, file)
}
