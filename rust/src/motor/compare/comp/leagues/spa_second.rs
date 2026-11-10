use super::*;

// The comparers of the routines in ../comp/leagues/spa_second.rs.
// [pt] Os comparadores das rotinas de ../comp/leagues/spa_second.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `spa_second_create_schedule`.
pub(crate) unsafe extern "thiscall" fn compare_spa_second_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_SPA_SECOND_CREATE_SCHEDULE);
    compare_league_round_table("spa_second_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| spa_second_create_schedule(this, stage, r, e, l))
}

/// Comparer for `construct_spa_second`.
pub(crate) unsafe extern "thiscall" fn compare_construct_spa_second(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_SPA_SECOND);
    compare_division_league_ctor("construct_spa_second", League(this), year, record, &SPANISH_SEGUNDA, || o(this, year, record))
}

/// Comparer for `spa_second_set_up_rules`.
pub(crate) unsafe extern "thiscall" fn compare_spa_second_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SPA_SECOND_SET_UP_RULES);
    spa_first_compare_league_rules("spa_second_set_up_rules", League(this), segunda_rules(this), || o(this), || spa_second_set_up_rules(this))
}

/// Comparer for `spa_second_create_table`.
///
/// [pt] Aloca: só a original. O plano, feito antes: os clubes da Segunda na ordem do banco (até 22) e a marca dos times B.
/// Depois: a devolução, +0x3e e as linhas (o clube e o destino −4 dos marcados).
pub(crate) unsafe extern "thiscall" fn compare_spa_second_create_table(this: usize) -> u32 {
    let this = League(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SPA_SECOND_CREATE_TABLE);
    let n = read_usize(G_CLUB_COUNT) as i32;
    let base = read_usize(G_CLUB_TABLE);
    let mut plan: Vec<(usize, bool)> = Vec::new();
    for i in 0..n.max(0) as usize {
        let c = base + i * club::SIZE;
        if spa_lower_is_in_comp(Club(c), KEY_CLUB_COMP_SPANISH_SECOND_DIVISION) {
            plan.push((c, is_b_team_of_comp_club(c, KEY_CLUB_COMP_SPANISH_FIRST_DIVISION)));
            if plan.len() == 0x16 {
                break;
            }
        }
    }
    let r = o(this.addr());
    let t = this.table();
    let mut ok = r == (plan.len() == 0x16) as u32 && this.club_count() == 0x16 && t != 0;
    if ok {
        for (k, &(c, b)) in plan.iter().enumerate() {
            let l = LeagueTableRow(t + k * 0x3b);
            ok = ok && read_usize(l.addr()) == c && (!b || l.status() == 0xfc);
        }
    }
    SCOREBOARD.record(ok, || format!("spa_second_create_table({:#x}): returned {}, plan {:x?}", this.addr(), r, plan));
    r
}
