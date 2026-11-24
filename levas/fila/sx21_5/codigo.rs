// ---------- sx21_5: England's squad limits and cup-tied players, and a transfer window's date (england_rules.cpp, argentina_rules.cpp) ----------

/// Fills a 42-byte match squad restriction record: every byte 0xff (no limit), then the given bytes.
pub(crate) unsafe fn fill_squad_restrictions_with(out: usize, limits: &[(usize, u8)]) {
    for k in 0..42 {
        write_u8(out + k, 0xff);
    }
    for &(at, v) in limits {
        write_u8(out + at, v);
    }
}

/// Side-by-side comparer of a `fill_squad_restrictions` over the 42 bytes.
pub(crate) unsafe fn compare_squad_restrictions(name: &str, o: unsafe extern "thiscall" fn(usize, usize, usize), r: unsafe extern "thiscall" fn(usize, usize, usize), this: usize, fixture: usize, out: usize) {
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&[(out, 42)], || o(this, fixture, out), || r(this, fixture, out));
    SCOREBOARD.record(equal, || format!("{}(fixture {:#x}) original={:x?} rust={:x?}", name, fixture, mo, mr));
}

/// England's squad limits of a match: three non-EU players (byte +5) in every competition.
///
/// `ENGLAND_RULES::fill_squad_restrictions(fixture, out)`, the vtable slot
/// `transfer_rules_vtable::FILL_SQUAD_RESTRICTIONS` of England: the 42-byte record `out` at 0xff (no limit) and byte
/// +5 [inference: the non-EU limit] 3. The fixture is not read.
/// Original: `ENGLAND_RULES::fill_squad_restrictions` (0x579c50, england_rules.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn england_rules_fill_squad_restrictions(_this: usize, _fixture: usize, out: usize) {
    covered!("sx21_5 england squad limits");
    fill_squad_restrictions_with(out, &[(5, 3)]); // sx21_5: three non-EU players
}

/// Comparer for `england_rules_fill_squad_restrictions`.
pub(crate) unsafe extern "thiscall" fn compare_england_rules_fill_squad_restrictions(this: usize, fixture: usize, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I0%%);
    compare_squad_restrictions("england_rules_fill_squad_restrictions", o, england_rules_fill_squad_restrictions, this, fixture, out);
}

/// TRANSFER_RULES::get_ineligibility_code (0x8e8a60, thiscall(player, fixture), ret 8, al).
const FN_TRANSFER_RULES_GET_INELIGIBILITY_CODE: usize = 0x8e8a60;

/// The base eligibility (`TRANSFER_RULES::get_ineligibility_code`), the answer of every country's own version when
/// its rule does not apply.
pub(crate) unsafe fn base_ineligibility_code(this: usize, player: usize, fixture: usize) -> u32 {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_TRANSFER_RULES_GET_INELIGIBILITY_CODE);
    f(this, player, fixture)
}

/// The player's statistics entry in the fixture's competition, when that competition is one of the `cups` keys and he
/// played in it for a club other than his current one (cup-tied); otherwise 0.
///
/// The fixture's `competition()` (none: 0), its id among `cups`; the player's sheet (`staff::PLAYER`, none: 0); his
/// entry in the competition's statistics (`exe::comp_stats_find_player_entry` on the Comp_Stats at +8 of the
/// competition's object in `G_COMP_OBJECT_TABLE`; none: 0); with a current club (`club_job()`; none: 0), the entry's
/// club id (+8) other than that club's.
pub(crate) unsafe fn cup_tied_entry(player: usize, fixture: usize, cups: &[usize]) -> usize {
    let comp = Fixture(fixture).competition().addr();
    if comp == 0 {
        return 0;
    }
    let id = read_u32(comp);
    if !cups.iter().any(|&k| read_u32(k) == id) {
        return 0;
    }
    let sheet = Staff(player).player().addr();
    if sheet == 0 {
        return 0;
    }
    let object = read_usize(read_usize(G_COMP_OBJECT_TABLE) + id as usize * 4);
    let entry = exe::comp_stats_find_player_entry(read_usize(object + league::COMP_STATS), sheet);
    let club = Staff(player).club_job().addr();
    if entry == 0 || club == 0 || read_u32(entry + 8) == read_u32(club) { // entry +8: the club he played for
        return 0;
    }
    entry
}

/// Why a player may not play an English match, or 0: cup-tied in the FA Cup or the League Cup (1), else the base
/// rules.
///
/// `ENGLAND_RULES::get_ineligibility_code(player, fixture)`, the vtable slot
/// `transfer_rules_vtable::GET_INELIGIBILITY_CODE` of England: in `KEY_CLUB_COMP_ENGLISH_FA_CUP` or
/// `KEY_CLUB_COMP_ENGLISH_LEAGUE_CUP`, a player who already played that cup for another club (`cup_tied_entry`) gets 1;
/// any other case, the base answer (`TRANSFER_RULES::get_ineligibility_code`).
/// Original: `ENGLAND_RULES::get_ineligibility_code` (0x57a090, england_rules.cpp, thiscall, ret 8, al).
pub(crate) unsafe extern "thiscall" fn england_rules_get_ineligibility_code(this: usize, player: usize, fixture: usize) -> u32 {
    if cup_tied_entry(player, fixture, &[KEY_CLUB_COMP_ENGLISH_FA_CUP, KEY_CLUB_COMP_ENGLISH_LEAGUE_CUP]) != 0 {
        covered!("sx21_5 england cup-tied");
        return 1; // sx21_5: cup-tied
    }
    base_ineligibility_code(this, player, fixture)
}

/// Comparer of a `get_ineligibility_code`: the code (al) of both and `G_CONTRACT_LOOKUP_RESULT`, side by side.
pub(crate) unsafe fn compare_ineligibility(name: &str, o: unsafe extern "thiscall" fn(usize, usize, usize) -> u32, r: unsafe extern "thiscall" fn(usize, usize, usize) -> u32, this: usize, player: usize, fixture: usize) -> u32 {
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&[(G_CONTRACT_LOOKUP_RESULT, 4)], || o(this, player, fixture), || r(this, player, fixture));
    SCOREBOARD.record(equal && ro as u8 == rr as u8, || format!("{}(player {:#x}, fixture {:#x}) original={} {:x?} rust={} {:x?}", name, player, fixture, ro as u8, mo, rr as u8, mr));
    ro
}

/// Comparer for `england_rules_get_ineligibility_code`.
pub(crate) unsafe extern "thiscall" fn compare_england_rules_get_ineligibility_code(this: usize, player: usize, fixture: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I1%%);
    compare_ineligibility("england_rules_get_ineligibility_code", o, england_rules_get_ineligibility_code, this, player, fixture)
}

/// The date of a transfer window entry in a given year.
///
/// `transfer_window_get_date(out, year)`, on a 6-byte window date (`this`): the Date (`exe::date_date_549f70`) with the
/// day (+3, signed), the month (+4), `year` and the weekday the date moves to (+2, signed; -1 none) [inference].
/// Returns `out`.
/// Original: `transfer_window_get_date` (0x40a7d0, argentina_rules.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn transfer_window_get_date(this: usize, out: usize, year: u32) -> usize {
    covered!("sx21_5 window date");
    exe::date_date_549f70(out as *mut [u16; 4], read_u8(this + 3) as i8 as i16 as u16 as u32, read_u8(this + transfer_window::MONTH) as u32, year, read_u8(this + 2) as i8 as i16 as u16 as u32); // sx21_5: the day and the month
    out
}

/// Comparer for `transfer_window_get_date`: side by side over the 8 bytes of the date.
pub(crate) unsafe extern "thiscall" fn compare_transfer_window_get_date(this: usize, out: usize, year: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(%%I2%%);
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&[(out, 8)], || o(this, out, year), || transfer_window_get_date(this, out, year));
    SCOREBOARD.record(equal && ro == rr, || format!("transfer_window_get_date(window {:#x}, year {}) original={:x?} rust={:x?}", this, year as u16, mo, mr));
    ro
}
