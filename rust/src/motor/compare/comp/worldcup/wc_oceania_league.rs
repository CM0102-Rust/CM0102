use super::*;

// The comparers of the routines in ../comp/worldcup/wc_oceania_league.rs.
// [pt] Os comparadores das rotinas de ../comp/worldcup/wc_oceania_league.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `wc_oceania_league_update_staff_records_for_status`.
pub(crate) unsafe extern "thiscall" fn compare_wc_oceania_league_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, out: usize, game: usize, a6: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, usize, u32) -> u32>(IDX_WC_OCEANIA_LEAGUE_UPDATE_STAFF_RECORDS_FOR_STATUS);
    let slot = stage as u8 == 2 && outcome as u8 == 1 && out != 0;
    let mundial = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP).wrapping_mul(4)));
    if slot && mundial == 0 {
        return o(this, club, outcome, stage, out, game, a6);
    }
    let n = if slot { read_i16(mundial + league::ENTRANT_COUNT) } else { 0 };
    let cabe = slot && n < read_i16(mundial + league::ENTRANT_CAPACITY);
    let r = o(this, club, outcome, stage, out, game, a6);
    let same = r == 0 && (!slot || (*(out as *const u8) == 4 && read_i16(mundial + league::ENTRANT_COUNT) == if cabe { n + 1 } else { n }));
    SCOREBOARD.record(same, || format!("wc_oceania_league_update_staff_records_for_status({:#x}, {}, {}) devolveu {} vaga={} lista {}", club, outcome as u8, stage as u8, r, slot, n));
    r
}

/// Comparer for `wc_oceania_league_set_up_rules`.
pub(crate) unsafe extern "thiscall" fn compare_wc_oceania_league_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WC_OCEANIA_LEAGUE_SET_UP_RULES);
    check_set_up_rules("wc_oceania_league_set_up_rules", this, &OCEANIA_RULE_FIELDS, (0xf, 0, 0, 5), 0, || o(this))
}
