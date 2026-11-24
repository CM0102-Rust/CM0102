use super::*;

/// Builds the Europe-wide transfer rules: the base rules, the Europe vtable and a cleared window count.
///
/// `EUROPE_RULES::EUROPE_RULES(a1..a5)`, once, when the transfer manager builds its rules (0x8d26fd). The base
/// constructor (`exe::new_transfer_rules`) with the five arguments, then the vtable 0x96a188 and the byte +8 (the
/// transfer window count) at 0; the window table (+4) stays as the base left it. Returns the object.
/// Original: `EUROPE_RULES::EUROPE_RULES` (0x583280, european_awards.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn construct_europe_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    covered!("sx03_3 europe rules");
    exe::new_transfer_rules(this, a1, a2, a3, a4, a5);
    write_u32(this, 0x96a188); // sx03_3: the Europe vtable
    write_u8(this + 8, 0);
    this
}

/// Comparer for `construct_europe_rules` (`compare_transfer_rules_ctor`: the original on the object, the copy on a
/// copy of its first bytes).
pub(crate) unsafe extern "thiscall" fn compare_construct_europe_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(IDX_CONSTRUCT_EUROPE_RULES);
    let mut r = 0usize;
    compare_transfer_rules_ctor("construct_europe_rules", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        construct_europe_rules(s, a1, a2, a3, a4, a5);
    });
    r
}

/// The default squad limits of a match: none.
///
/// `TRANSFER_RULES::fill_squad_restrictions(fixture, out)`, the vtable slot `transfer_rules_vtable::FILL_SQUAD_RESTRICTIONS`
/// of the rules without limits of their own (Europe's among them). The 42-byte restriction record `out` is filled with
/// 0xff (ten dwords and a word); the fixture is not read.
/// Original: `TRANSFER_RULES::fill_squad_restrictions` (0x5832c0, european_awards.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn default_squad_restrictions(_this: usize, _fixture: usize, out: usize) {
    covered!("sx03_3 default squad restrictions");
    for k in 0..42 { // sx03_3: the 42-byte record
        write_u8(out + k, 0xff);
    }
}

/// Comparer for `default_squad_restrictions`: side by side over the 42 bytes.
pub(crate) unsafe extern "thiscall" fn compare_default_squad_restrictions(this: usize, fixture: usize, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_DEFAULT_SQUAD_RESTRICTIONS);
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&[(out, 42)], || o(this, fixture, out), || default_squad_restrictions(this, fixture, out));
    SCOREBOARD.record(equal, || format!("default_squad_restrictions original={:x?} rust={:x?}", mo, mr));
}

/// Why a player may not play a European match, or 0: the base rules, plus the UEFA rule that a player over 19
/// already in the squad of another club's European campaign is cup-tied (8).
///
/// `EUROPE_RULES::get_ineligibility_code(player, fixture)`, the vtable slot
/// `transfer_rules_vtable::GET_INELIGIBILITY_CODE` of the Europe rules. The base answer
/// (`TRANSFER_RULES::get_ineligibility_code`) for: a virtual person (id at or above `G_STAFF_COUNT`); a player under
/// 20 (`staff::AGE`, signed byte; a missing player is read anyway, as in the original); the Super Cup; a competition
/// that is not the Intertoto, the UEFA Cup, the Champions League or the Cup Winners' Cup; a competition whose season
/// year (`cup::YEAR` of its object in `G_COMP_OBJECT_TABLE`) is not the word at the start of the squad manager
/// (`G_SQUAD_MANAGER`). Otherwise, by the round (`fixture::ROUND_NAME`, without its variant digit:
/// `exe::comp_util_get_base_round_id`): in the Intertoto, round 0x96 gives the European rule and the others the base
/// answer; elsewhere rounds 0xe6 and 0xa0 give 0 and the others the European rule. The European rule: 8 when the
/// player's `staff::SQUAD_SELECTED_FOR` is not 2, else 0.
/// Original: `EUROPE_RULES::get_ineligibility_code` (0x5832e0, european_awards.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn europe_rules_get_ineligibility_code(this: usize, player: usize, fixture: usize) -> u32 {
    let base = || {
        let f: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_TRANSFER_RULES_GET_INELIGIBILITY_CODE);
        f(this, player, fixture)
    };
    let person = Staff(player);
    if player != 0 && read_i32(player) >= read_i32(G_STAFF_COUNT) {
        return base();
    }
    if (person.age() as i8) < 0x14 { // sx03_3: players under 20
        return base();
    }
    let game = Fixture(fixture);
    let comp = read_u32(game.competition().addr());
    if comp == read_u32(KEY_CLUB_COMP_EUROPEAN_SUPER_CUP) {
        return base();
    }
    let european = [KEY_CLUB_COMP_INTER_TOTO_CUP, KEY_CLUB_COMP_UEFA_CUP, KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE, KEY_CLUB_COMP_EUROPEAN_CUP_WINNERS_CUP];
    if !european.iter().any(|&k| read_u32(k) == comp) {
        return base();
    }
    let object = read_usize(read_usize(G_COMP_OBJECT_TABLE) + comp as usize * 4);
    if Cup(object).year() as i32 != read_u16(read_usize(G_SQUAD_MANAGER)) as i32 {
        return base();
    }
    let round = || exe::comp_util_get_base_round_id(game.round_name() as u16 as u32) as u16;
    if comp == read_u32(KEY_CLUB_COMP_INTER_TOTO_CUP) {
        if round() != 0x96 {
            return base();
        }
    } else if round() == 0xe6 || round() == 0xa0 {
        covered!("sx03_3 rounds outside the rule");
        return 0;
    }
    covered!("sx03_3 cup-tied rule");
    if person.squad_selected_for() != 2 { 8 } else { 0 } // sx03_3: cup-tied
}

/// Comparer for `europe_rules_get_ineligibility_code`: the code (al) of both.
pub(crate) unsafe extern "thiscall" fn compare_europe_rules_get_ineligibility_code(this: usize, player: usize, fixture: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_EUROPE_RULES_GET_INELIGIBILITY_CODE)(this, player, fixture);
    let c = europe_rules_get_ineligibility_code(this, player, fixture);
    SCOREBOARD.record(r as u8 == c as u8, || format!("europe_rules_get_ineligibility_code(player {:#x}, fixture {:#x}) original={} rust={}", player, fixture, r as u8, c as u8));
    r
}

/// TRANSFER_RULES::get_ineligibility_code (0x8e8a60, thiscall(player, fixture), ret 8, al): the base eligibility.
pub(crate) const FN_TRANSFER_RULES_GET_INELIGIBILITY_CODE: usize = 0x8e8a60;
