// ---------- cx03_11: the Scottish and Welsh league cups' own data in the saved game (sco_lge_cup.cpp, wel_lge_cup.cpp, wel_prm_cup.cpp) ----------

/// Reads the Scottish League Cup's own data from its temporary file: the byes byte (+0xb2, the clubs in the European
/// cups) (`exe::crt_fread` of 1 byte, cdecl); a short read: the box of line 0x2b8 of sco_lge_cup.cpp and 0. Returns 1.
/// Original: `ScoLgeCup::load_extra_data` (0x7f0ae0, sco_lge_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn sco_lge_cup_load_extra_data(this: usize, file: usize) -> u32 {
    if exe::crt_fread(this + 0xb2, 1, 1, file) != 1 {
        show_error_at(0xa7af38, 0x2b8);
        return 0;
    }
    covered!("cx03_11 scottish league cup data loaded");
    1
}

/// The body of the Welsh league cups' `load_extra_data`: the entrant count (`ita_cup_fields::ENTRANT_CLUB_COUNT`, 4 bytes;
/// a short read: the box of line `rows[0]` and 0), the entrant list (`exe::crt_malloc` of 4 per entrant into
/// `ita_cup_fields::ENTRANT_CLUBS`; out of memory, the box of `rows[1]` and 0) read whole (a short read: the box of
/// `rows[2]` and 0), then each club index back into a club pointer (`club_at`; −1, a free slot, becomes null). Returns
/// 1.
unsafe fn load_welsh_cup_entrants(this: usize, file: usize, source: usize, rows: [u32; 3], branch: &'static str) -> u32 {
    let cup = ItaCupFields(this);
    if exe::crt_fread(this + ita_cup_fields::ENTRANT_CLUB_COUNT, 4, 1, file) != 1 {
        show_error_at(source, rows[0]);
        return 0;
    }
    let list = exe::crt_malloc(cup.entrant_club_count() as u32 * 4);
    cup.set_entrant_clubs(list);
    if list == 0 {
        show_error_at(source, rows[1]);
        return 0;
    }
    if exe::crt_fread(list, 4, cup.entrant_club_count() as u32, file) != cup.entrant_club_count() as u32 {
        show_error_at(source, rows[2]);
        return 0;
    }
    covered!(branch);
    let mut k = 0i32;
    while k < cup.entrant_club_count() as i32 {
        let slot = cup.entrant_clubs() + k as usize * 4;
        let index = read_i32(slot);
        write_usize(slot, if index == -1 { 0 } else { club_at(index as isize).addr() });
        k += 1;
    }
    1
}

/// Reads the Welsh League Cup's entrants from its temporary file (`load_welsh_cup_entrants`, the boxes of lines 0x348,
/// 0x350 and 0x358 of wel_lge_cup.cpp).
/// Original: `WelLgeCup::load_extra_data` (0x927330, wel_lge_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn wel_lge_cup_load_extra_data(this: usize, file: usize) -> u32 {
    load_welsh_cup_entrants(this, file, 0xad56f4, [0x348, 0x350, 0x358], "cx03_11 welsh league cup entrants loaded")
}

/// Reads the Welsh Premier Cup's entrants from its temporary file (`load_welsh_cup_entrants`, the boxes of lines 0x36e,
/// 0x376 and 0x37e of wel_prm_cup.cpp).
/// Original: `WelPrmCup::load_extra_data` (0x928c40, wel_prm_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn wel_prm_cup_load_extra_data(this: usize, file: usize) -> u32 {
    load_welsh_cup_entrants(this, file, 0xad572c, [0x36e, 0x376, 0x37e], "cx03_11 welsh premier cup entrants loaded")
}

pub(crate) unsafe extern "thiscall" fn compare_sco_lge_cup_load_extra_data(this: usize, file: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%);
    compare_cup_extra_data_load("sco_lge_cup_load_extra_data", this, file, &[(0xb2, 1)], || o(this, file), || sco_lge_cup_load_extra_data(this, file))
}

/// Comparer of the Welsh entrant readers: the original reads first; then the file goes back to where it was and the
/// copy reads the same bytes into a list of its own; the returns, the counts, the lists and the file positions must
/// match. The copy's list is freed and the original's put back.
unsafe fn compare_welsh_cup_entrants(name: &str, this: usize, file: usize, original: impl FnOnce() -> u32, copy: impl FnOnce() -> u32) -> u32 {
    let cup = ItaCupFields(this);
    let at = exe::crt_ftell(file);
    let r = original();
    let (end_o, list_o, n_o) = (exe::crt_ftell(file), cup.entrant_clubs(), cup.entrant_club_count());
    exe::crt_fseek(file, at, 0);
    let c = copy();
    let (end_c, list_c, n_c) = (exe::crt_ftell(file), cup.entrant_clubs(), cup.entrant_club_count());
    let k = 4 * (n_o as i32).max(0) as usize;
    let same = n_o == n_c && (r == 0 || (list_o != 0 && list_c != 0
        && core::slice::from_raw_parts(list_o as *const u8, k) == core::slice::from_raw_parts(list_c as *const u8, k)));
    if list_c != 0 && list_c != list_o {
        exe::crt_free(list_c);
    }
    cup.set_entrant_clubs(list_o);
    cup.set_entrant_club_count(n_o);
    exe::crt_fseek(file, end_o, 0);
    SCOREBOARD.record(same && r == c && end_o == end_c, || format!("{}({:#x}) returned {} x {}, file at {} x {} (from {}), entrants {} x {}", name, this, r, c, end_o, end_c, at, n_o, n_c));
    r
}

pub(crate) unsafe extern "thiscall" fn compare_wel_lge_cup_load_extra_data(this: usize, file: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I1%%);
    compare_welsh_cup_entrants("wel_lge_cup_load_extra_data", this, file, || o(this, file), || wel_lge_cup_load_extra_data(this, file))
}

pub(crate) unsafe extern "thiscall" fn compare_wel_prm_cup_load_extra_data(this: usize, file: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I2%%);
    compare_welsh_cup_entrants("wel_prm_cup_load_extra_data", this, file, || o(this, file), || wel_prm_cup_load_extra_data(this, file))
}
