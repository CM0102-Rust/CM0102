// ---------- cx02_14: the group cups' entrant list in the saved game (gre_cup.cpp, ire_lge_cup.cpp, ita_c_cup.cpp) ----------

/// The entrant list of a cup built on a league with group stages, read from its temporary file: the count
/// (`ita_cup_fields::ENTRANT_CLUB_COUNT`, 4 bytes; `exe::crt_fread`); a short read shows the box of `lines.0` and
/// returns 0. With `empty_is_none` (the Dutch Cup) a count of 0 leaves the list null and returns 1. Otherwise a new
/// list of count × 4 bytes (`exe::crt_malloc`, into `ita_cup_fields::ENTRANT_CLUBS`; out of memory the box of
/// `lines.1` and 0) and the count's club ids read into it (short: the box of `lines.2` and 0); each id becomes its club
/// (`club_at`, the club table read each time), −1 a null slot (the count read again each time). Returns 1. The boxes
/// are the source file `file_name`'s.
pub(crate) unsafe fn load_group_cup_entrants(this: usize, file: usize, file_name: usize, lines: (u32, u32, u32), empty_is_none: bool) -> u32 {
    let cup = ItaCupFields(this);
    if exe::crt_fread(this + ita_cup_fields::ENTRANT_CLUB_COUNT, 4, 1, file) != 1 {
        show_error_at(file_name, lines.0);
        return 0;
    }
    let count = || read_i32(this + ita_cup_fields::ENTRANT_CLUB_COUNT);
    if empty_is_none && count() == 0 {
        cup.set_entrant_clubs(0);
        return 1;
    }
    let list = exe::crt_malloc((count() as u32).wrapping_mul(4));
    cup.set_entrant_clubs(list);
    if list == 0 {
        show_error_at(file_name, lines.1);
        return 0;
    }
    if exe::crt_fread(list, 4, count() as u32, file) != count() as u32 {
        show_error_at(file_name, lines.2);
        return 0;
    }
    covered!("cx02_14 group cup entrants loaded");
    let mut k = 0i32;
    while k < count() {
        let slot = cup.entrant_clubs() + 4 * k as usize;
        let id = read_i32(slot);
        write_usize(slot, if id == -1 { 0 } else { club_at(id as isize).addr() });
        k += 1;
    }
    1
}

/// The Greek Cup entrant list (`load_group_cup_entrants`, gre_cup.cpp lines 0x383, 0x38b and 0x393).
/// Original: `GreCup::load_extra_data` (0x5e9df0, gre_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn gre_cup_load_extra_data(this: usize, file: usize) -> u32 {
    load_group_cup_entrants(this, file, 0x9cdab0, (0x383, 0x38b, 0x393), false)
}

/// The Irish League Cup entrant list (`load_group_cup_entrants`, ire_lge_cup.cpp lines 0x381, 0x389 and 0x391).
/// Original: `IreLgeCup::load_extra_data` (0x638260, ire_lge_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn ire_lge_cup_load_extra_data(this: usize, file: usize) -> u32 {
    load_group_cup_entrants(this, file, 0x9e9f4c, (0x381, 0x389, 0x391), false)
}

/// The Serie C cup entrant list (`load_group_cup_entrants`, ita_c_cup.cpp lines 0x3cc, 0x3d4 and 0x3dc).
/// Original: `ItaCCup::load_extra_data` (0x63e3c0, ita_c_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn ita_c_cup_load_extra_data(this: usize, file: usize) -> u32 {
    load_group_cup_entrants(this, file, 0x9ea204, (0x3cc, 0x3d4, 0x3dc), false)
}

/// Comparer of the group cups' entrant list readers: the original reads first (its list kept); then the file goes
/// back to where it was and the copy reads the same bytes into a list of its own; the returns, the counts, the file
/// positions and the lists' clubs must match. The copy's list is freed and the original's put back.
pub(crate) unsafe fn compare_group_cup_entrants_load(name: &str, this: usize, file: usize, original: impl FnOnce() -> u32, copy: impl FnOnce() -> u32) -> u32 {
    let cup = ItaCupFields(this);
    let at = exe::crt_ftell(file);
    let r = original();
    if r == 0 {
        return r; // the error box was shown: the copy is not run again
    }
    let (end_o, list_o, count_o) = (exe::crt_ftell(file), cup.entrant_clubs(), read_i32(this + ita_cup_fields::ENTRANT_CLUB_COUNT));
    exe::crt_fseek(file, at, 0);
    let c = copy();
    let (end_c, list_c, count_c) = (exe::crt_ftell(file), cup.entrant_clubs(), read_i32(this + ita_cup_fields::ENTRANT_CLUB_COUNT));
    let n = if r == 1 && c == 1 && count_o == count_c && list_o != 0 && list_c != 0 { count_o.max(0) as usize } else { 0 };
    let same = (0..n).all(|k| read_usize(list_o + 4 * k) == read_usize(list_c + 4 * k));
    if list_c != 0 && list_c != list_o {
        exe::crt_free(list_c);
    }
    cup.set_entrant_clubs(list_o);
    write_i32(this + ita_cup_fields::ENTRANT_CLUB_COUNT, count_o);
    exe::crt_fseek(file, end_o, 0);
    SCOREBOARD.record(r == c && count_o == count_c && end_o == end_c && same && (list_o == 0) == (list_c == 0),
        || format!("{}({:#x}) returned {} x {}, count {} x {}, file at {} x {} (from {}), lists equal {}", name, this, r, c, count_o, count_c, end_o, end_c, at, same));
    r
}

pub(crate) unsafe extern "thiscall" fn compare_gre_cup_load_extra_data(this: usize, file: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%);
    compare_group_cup_entrants_load("gre_cup_load_extra_data", this, file, || o(this, file), || gre_cup_load_extra_data(this, file))
}

pub(crate) unsafe extern "thiscall" fn compare_ire_lge_cup_load_extra_data(this: usize, file: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I1%%);
    compare_group_cup_entrants_load("ire_lge_cup_load_extra_data", this, file, || o(this, file), || ire_lge_cup_load_extra_data(this, file))
}

pub(crate) unsafe extern "thiscall" fn compare_ita_c_cup_load_extra_data(this: usize, file: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I2%%);
    compare_group_cup_entrants_load("ita_c_cup_load_extra_data", this, file, || o(this, file), || ita_c_cup_load_extra_data(this, file))
}
