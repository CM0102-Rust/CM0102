// ---------- cx05_16: the friendly mini cup and mini league rebuilt from the saved game (mini_cup.cpp, mini_league.cpp) ----------

/// Builds a mini cup from the saved game: Cup::Cup (`exe::new_cup`), the start date (+0xb2) constructed
/// (`exe::date_date_54a110`), the mini cup's vtable 0x96d8ac, the competition record and the cup's state read from its
/// temporary file (`exe::cup_load_from_disk`(this, 1); the result is not read). Returns this.
/// Original: `MiniCup::MiniCup` (0x765e30, mini_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn load_mini_cup(this: usize, record: usize) -> usize {
    exe::new_cup(this);
    exe::date_date_54a110(this + 0xb2); // the start date
    write_usize(this, 0x96d8ac); // the mini cup's vtable
    write_usize(this + league::COMPETITION, record); // cx05_16: the mini cup's competition record
    covered!("cx05_16 mini cup loaded");
    exe::cup_load_from_disk(this, 1);
    this
}

/// Builds a mini league from the saved game: League::League (`exe::new_league`), the start date (+0xee) constructed,
/// the mini league's vtable 0x96d94c, 4 clubs, 3 rounds, the competition record and the league's state read from its
/// temporary file (`exe::league_load_from_temp_file`(this, 1); 0: the box at line 0x85). Returns this.
/// Original: `MiniLeague::MiniLeague` (0x766800, mini_league.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn load_mini_league(this: usize, record: usize) -> usize {
    exe::new_league(this);
    exe::date_date_54a110(this + 0xee); // the start date
    write_usize(this, 0x96d94c); // cx05_16: the mini league's vtable
    write_u16(this + league::CLUB_COUNT, 4);
    write_u16(this + league::ROUND_COUNT, 3);
    write_usize(this + league::COMPETITION, record);
    if exe::league_load_from_temp_file(this, 1) == 0 {
        show_error_at(0xa5ab3c, 0x85);
    }
    covered!("cx05_16 mini league loaded");
    this
}

/// Comparers of the two loaders: they read the temporary file and allocate (they cannot run twice); only the original
/// runs, then the plan: this, the vtable and the record.
pub(crate) unsafe extern "thiscall" fn compare_load_mini_cup(this: usize, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, record);
    let ok = r == this && read_usize(this) == 0x96d8ac && read_usize(this + league::COMPETITION) == record;
    SCOREBOARD.record(ok, || format!("load_mini_cup({:#x}): vtable {:#x}", this, read_usize(this)));
    r
}

pub(crate) unsafe extern "thiscall" fn compare_load_mini_league(this: usize, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I1%%)(this, record);
    let ok = r == this && read_usize(this) == 0x96d94c && read_usize(this + league::COMPETITION) == record;
    SCOREBOARD.record(ok, || format!("load_mini_league({:#x}): vtable {:#x}", this, read_usize(this)));
    r
}
