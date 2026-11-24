// ---------- sx21_17: Scotland's squad limits and cup-tied players (scotland_rules.cpp) ----------

/// Scotland's squad limits of a match: the reserve age rule for Premier Division clubs, and an age rule in the Premier
/// Division.
///
/// `SCOTLAND_RULES::fill_squad_restrictions(fixture, out)`: the 42-byte record at 0xff. When the id of the fixture's
/// `competition()` is `KEY_CLUB_COMP_RESERVE` and the home club's `division()` is
/// `KEY_CLUB_COMP_SCOTTISH_PREMIER_DIVISION`: byte +0x16 21, +0x17 3 [inference: an age limit and how many may be
/// older] and the dword +0x26 the reserve key's value. Otherwise, when that id is the Premier Division's: byte +6 21,
/// +8 2 [inference: the same pair of the first block] and +0x26 the Premier Division key's value.
/// Original: `SCOTLAND_RULES::fill_squad_restrictions` (0x7f64c0, scotland_rules.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn scotland_rules_fill_squad_restrictions(_this: usize, fixture: usize, out: usize) {
    covered!("sx21_17 scotland squad limits");
    fill_squad_restrictions_with(out, &[]);
    let game = Fixture(fixture);
    let id = read_u32(game.competition().addr());
    let premier = read_u32(KEY_CLUB_COMP_SCOTTISH_PREMIER_DIVISION);
    if id == read_u32(KEY_CLUB_COMP_RESERVE) {
        let division = game.home_club().division().addr();
        if division != 0 && read_u32(division) == premier {
            write_u8(out + 0x16, 0x15);
            write_u8(out + 0x17, 3); // sx21_17: three older reserves
            write_u32(out + 0x26, read_u32(KEY_CLUB_COMP_RESERVE));
            return;
        }
    }
    if id == premier {
        write_u8(out + 6, 0x15);
        write_u8(out + 8, 2); // sx21_17: two in the Premier Division
        write_u32(out + 0x26, read_u32(KEY_CLUB_COMP_SCOTTISH_PREMIER_DIVISION));
    }
}

/// Comparer for `scotland_rules_fill_squad_restrictions`.
pub(crate) unsafe extern "thiscall" fn compare_scotland_rules_fill_squad_restrictions(this: usize, fixture: usize, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I0%%);
    compare_squad_restrictions("scotland_rules_fill_squad_restrictions", o, scotland_rules_fill_squad_restrictions, this, fixture, out);
}

/// Why a player may not play a Scottish match, or 0: cup-tied in the Scottish Cup or the League Cup before the round
/// two after his last appearance (1), else the base rules.
///
/// `SCOTLAND_RULES::get_ineligibility_code(player, fixture)`, the vtable slot
/// `transfer_rules_vtable::GET_INELIGIBILITY_CODE` of Scotland: in `KEY_CLUB_COMP_SCOTTISH_CUP` or
/// `KEY_CLUB_COMP_SCOTTISH_LEAGUE_CUP`, a player who played that cup for another club (`cup_tied_entry`) gets 1 while
/// the fixture's `round_number()` is below the entry's byte +0x1c (signed) [inference: the last round he played] plus
/// 2; any other case, the base answer (`base_ineligibility_code`).
/// Original: `SCOTLAND_RULES::get_ineligibility_code` (0x7f65d0, scotland_rules.cpp, thiscall, ret 8, al).
pub(crate) unsafe extern "thiscall" fn scotland_rules_get_ineligibility_code(this: usize, player: usize, fixture: usize) -> u32 {
    let entry = cup_tied_entry(player, fixture, &[KEY_CLUB_COMP_SCOTTISH_CUP, KEY_CLUB_COMP_SCOTTISH_LEAGUE_CUP]);
    if entry != 0 && (Fixture(fixture).round_number() as i32) < read_u8(entry + 0x1c) as i8 as i32 + 2 { // sx21_17: two rounds later
        covered!("sx21_17 scotland cup-tied");
        return 1;
    }
    base_ineligibility_code(this, player, fixture)
}

/// Comparer for `scotland_rules_get_ineligibility_code` (`compare_ineligibility`).
pub(crate) unsafe extern "thiscall" fn compare_scotland_rules_get_ineligibility_code(this: usize, player: usize, fixture: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I1%%);
    compare_ineligibility("scotland_rules_get_ineligibility_code", o, scotland_rules_get_ineligibility_code, this, player, fixture)
}
