// ---------- sx21_19: the squad limits of Belgian and German matches and who a German club may sign in the second window (belgium_rules.cpp, germany_rules.cpp) ----------

/// Belgium's squad limits of a match: an age rule (byte +6 21) with two players outside it (byte +8).
///
/// `BELGIUM_RULES::fill_squad_restrictions(fixture, out)`: the 42-byte record at 0xff, byte +6 21 and byte +8 2
/// [inference: an age and how many may break it]. The fixture is not read.
/// Original: `BELGIUM_RULES::fill_squad_restrictions` (0x424f80, belgium_rules.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn belgium_rules_fill_squad_restrictions(_this: usize, _fixture: usize, out: usize) {
    covered!("sx21_19 belgium squad limits");
    fill_squad_restrictions_with(out, &[(6, 0x15), (8, 2)]); // sx21_19: two in Belgium
}

/// Comparer for `belgium_rules_fill_squad_restrictions`.
pub(crate) unsafe extern "thiscall" fn compare_belgium_rules_fill_squad_restrictions(this: usize, fixture: usize, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I0%%);
    compare_squad_restrictions("belgium_rules_fill_squad_restrictions", o, belgium_rules_fill_squad_restrictions, this, fixture, out);
}

/// Germany's squad limits of a match: the non-EU players (byte +5): three in the German Cup, five elsewhere.
///
/// `GERMANY_RULES::fill_squad_restrictions(fixture, out)`: the 42-byte record at 0xff and byte +5 [inference: the non-EU
/// limit] 3 with a fixture whose `comp_id()` is `KEY_CLUB_COMP_GERMAN_CUP`, else 5.
/// Original: `GERMANY_RULES::fill_squad_restrictions` (0x5e02d0, germany_rules.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn germany_rules_fill_squad_restrictions(_this: usize, fixture: usize, out: usize) {
    covered!("sx21_19 germany squad limits");
    let cup = fixture != 0 && Fixture(fixture).comp_id() as u32 == read_u32(KEY_CLUB_COMP_GERMAN_CUP);
    fill_squad_restrictions_with(out, &[(5, if cup { 3 } else { 5 })]); // sx21_19: three in the German Cup
}

/// Comparer for `germany_rules_fill_squad_restrictions`.
pub(crate) unsafe extern "thiscall" fn compare_germany_rules_fill_squad_restrictions(this: usize, fixture: usize, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I1%%);
    compare_squad_restrictions("germany_rules_fill_squad_restrictions", o, germany_rules_fill_squad_restrictions, this, fixture, out);
}

/// Whether a club may sign a player under the German rules: outside the windows no (9); in the first window the base
/// rules; in the second window a German club outside the top two divisions may not sign (9), nor buy from a German
/// club of a less reputed division or of none (9), and otherwise the base rules.
///
/// `GERMANY_RULES::can_sign_player(club, player, seller, date, reason)`, the vtable slot
/// `transfer_rules_vtable::CAN_SIGN_PLAYER` of Germany. No window open on `date` (`exe::transfer_rules_find_open_window`):
/// reason 9 and 0. The date in window 0 (`exe::transfer_window_contains_date` with the table at +4): the base answer
/// (`transfer_rules_can_sign_player`). Not in window 1 (+0xc either): reason 9 and 0. In window 1: a club of the rule
/// set's nation (`transfer_rules::NATION_ID`) with a division other than `KEY_CLUB_COMP_GERMAN_FIRST_DIVISION` and
/// `KEY_CLUB_COMP_GERMAN_SECOND_DIVISION`: 9. A selling club of the buyer's nation, with the buyer in a division, the
/// seller without one or with a division of lower `reputation()`: 9. Otherwise the base answer.
/// Original: `GERMANY_RULES::can_sign_player` (0x5e00e0, germany_rules.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn germany_rules_can_sign_player(this: usize, club: usize, player: usize, seller: usize, date: usize, reason: *mut u16) -> u32 {
    let refuse = || {
        if !reason.is_null() {
            *reason = 9;
        }
        0
    };
    let mut window = 0usize;
    if exe::transfer_rules_find_open_window(this, date, &mut window as *mut usize as usize as u32) == 0 {
        return refuse();
    }
    if exe::transfer_window_contains_date(this, date, read_usize(this + 4)) != 0 {
        return transfer_rules_can_sign_player(this, club, player, seller as u32, date, reason);
    }
    if exe::transfer_window_contains_date(this, date, read_usize(this + 4) + 0xc) == 0 {
        return refuse();
    }
    let buyer = Club(club);
    if club != 0 && buyer.nation().addr() == read_usize(this + transfer_rules::NATION_ID) {
        let division = buyer.division().addr();
        if division != 0 && read_u32(division) != read_u32(KEY_CLUB_COMP_GERMAN_FIRST_DIVISION) && read_u32(division) != read_u32(KEY_CLUB_COMP_GERMAN_SECOND_DIVISION) {
            covered!("sx21_19 germany lower division in the second window");
            return refuse(); // sx21_19: only the top two divisions buy in the second window
        }
    }
    let from = Club(seller);
    if seller != 0 && from.nation().addr() == buyer.nation().addr() && buyer.division().addr() != 0
        && (from.division().addr() == 0 || from.division().reputation() < buyer.division().reputation()) {
        covered!("sx21_19 germany from a lower division");
        return refuse();
    }
    transfer_rules_can_sign_player(this, club, player, seller as u32, date, reason)
}

/// Comparer for `germany_rules_can_sign_player` (`compare_rule_check`).
pub(crate) unsafe extern "thiscall" fn compare_germany_rules_can_sign_player(this: usize, club: usize, player: usize, seller: usize, date: usize, reason: *mut u16) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, *mut u16) -> u32>(%%I2%%);
    compare_rule_check("germany_rules_can_sign_player", reason, || o(this, club, player, seller, date, reason), || germany_rules_can_sign_player(this, club, player, seller, date, reason), || format!("rules {:#x}, club {:#x}, player {:#x}, seller {:#x}", this, club, player, seller))
}
