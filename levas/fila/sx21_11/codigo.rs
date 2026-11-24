// ---------- sx21_11: the squad limits of Russian and Turkish matches and the Greek loan check (russia_rules.cpp, turkey_rules.cpp, greece_rules.cpp) ----------

/// Russia's squad limits of a match: three foreign players (byte +2) in the Russian First Division, no limit elsewhere.
///
/// `RUSSIA_RULES::fill_squad_restrictions(fixture, out)`: the 42-byte record at 0xff and, with a fixture whose
/// `comp_id()` is `KEY_CLUB_COMP_RUSSIAN_FIRST_DIVISION`, byte +2 [inference: the foreign limit in the match squad] 3.
/// Original: `RUSSIA_RULES::fill_squad_restrictions` (0x7ec950, russia_rules.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn russia_rules_fill_squad_restrictions(_this: usize, fixture: usize, out: usize) {
    covered!("sx21_11 russia squad limits");
    fill_squad_restrictions_with(out, &[]);
    if fixture != 0 && Fixture(fixture).comp_id() as u32 == read_u32(KEY_CLUB_COMP_RUSSIAN_FIRST_DIVISION) {
        write_u8(out + 2, 3); // sx21_11: three foreign players in the Russian top flight
    }
}

/// Comparer for `russia_rules_fill_squad_restrictions`.
pub(crate) unsafe extern "thiscall" fn compare_russia_rules_fill_squad_restrictions(this: usize, fixture: usize, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I0%%);
    compare_squad_restrictions("russia_rules_fill_squad_restrictions", o, russia_rules_fill_squad_restrictions, this, fixture, out);
}

/// Turkey's squad limits of a match: six foreign players (byte +2) in the Premier Division and the FA Cup, none in the
/// division below, and an age rule in reserve matches.
///
/// `TURKEY_RULES::fill_squad_restrictions(fixture, out)`: the 42-byte record at 0xff; by the fixture's `comp_id()`:
/// `KEY_CLUB_COMP_TURKISH_PREMIER_DIVISION` or `KEY_CLUB_COMP_TURKISH_FA_CUP`, byte +2 [inference: the foreign limit in
/// the match squad] 6; `KEY_CLUB_COMP_TURKISH_UNMATCHED_A` (the division below), byte +2 0; otherwise, when the id of
/// the fixture's `competition()` is `KEY_CLUB_COMP_RESERVE`: byte +0x16 21 and +0x17 0 [inference: an age limit and how
/// many may be older] and the dword +0x26 that key's value [inference: the competition the rule belongs to].
/// Original: `TURKEY_RULES::fill_squad_restrictions` (0x9021d0, turkey_rules.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn turkey_rules_fill_squad_restrictions(_this: usize, fixture: usize, out: usize) {
    covered!("sx21_11 turkey squad limits");
    fill_squad_restrictions_with(out, &[]);
    let game = Fixture(fixture);
    let id = game.comp_id() as u32;
    if id == read_u32(KEY_CLUB_COMP_TURKISH_PREMIER_DIVISION) || id == read_u32(KEY_CLUB_COMP_TURKISH_FA_CUP) {
        write_u8(out + 2, 6); // sx21_11: six foreign players in the Turkish top flight
    } else if id == read_u32(KEY_CLUB_COMP_TURKISH_UNMATCHED_A) {
        write_u8(out + 2, 0);
    } else if read_u32(game.competition().addr()) == read_u32(KEY_CLUB_COMP_RESERVE) {
        write_u8(out + 0x16, 0x15); // sx21_11: the reserve age rule
        write_u8(out + 0x17, 0);
        write_u32(out + 0x26, read_u32(KEY_CLUB_COMP_RESERVE));
    }
}

/// Comparer for `turkey_rules_fill_squad_restrictions`.
pub(crate) unsafe extern "thiscall" fn compare_turkey_rules_fill_squad_restrictions(this: usize, fixture: usize, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I1%%);
    compare_squad_restrictions("turkey_rules_fill_squad_restrictions", o, turkey_rules_fill_squad_restrictions, this, fixture, out);
}

/// Greece's loan check: every loan is allowed.
///
/// `GREECE_RULES::can_loan_player(_club, _player, _msg, _reason)`, the vtable slot `transfer_rules_vtable::CAN_LOAN_PLAYER`
/// shared by the Greek, Croatian and Polish rules: returns 1.
/// Original: `GREECE_RULES::can_loan_player` (0x5ed060, greece_rules.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn greece_rules_can_loan_player(_this: usize, _club: usize, _player: usize, _msg: usize, _reason: *mut u16) -> u32 {
    covered!("sx21_11 greece loan check");
    1 // sx21_11: Greek loans are allowed
}

/// Comparer for `greece_rules_can_loan_player`: the value of both.
pub(crate) unsafe extern "thiscall" fn compare_greece_rules_can_loan_player(this: usize, club: usize, player: usize, msg: usize, reason: *mut u16) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32>(%%I2%%)(this, club, player, msg, reason);
    let c = greece_rules_can_loan_player(this, club, player, msg, reason);
    SCOREBOARD.record(r == c, || format!("greece_rules_can_loan_player(club {:#x}, player {:#x}) original={} rust={}", club, player, r, c));
    r
}
