use super::*;

// The comparers of the routines in ../comp/leagues/jap_j1.rs.
// [pt] Os comparadores das rotinas de ../comp/leagues/jap_j1.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `jap_j1_advance_stage`.
///
/// [pt] O comparador da fase seguinte: só a original (simplification: ao chegar a 2 monta a final, que aloca e sorteia), com o
/// plano: na última fase, +0x30 igual; senão +0x30 + 1 (a final grava 2, o mesmo valor).
pub(crate) unsafe extern "thiscall" fn compare_jap_j1_advance_stage(this: usize) {
    let this = League(this);
    let before = this.current_stage() as i32;
    let last = before >= (this.stage_count() as i32).wrapping_sub(1);
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_JAP_J1_ADVANCE_STAGE)(this.addr());
    let after = this.current_stage() as i32;
    SCOREBOARD.record(after == if last { before } else { before + 1 }, || format!("jap_j1_advance_stage({:#x}) antes={} depois={}", this.addr(), before, after));
}

/// Comparer for `create_championship_final_stage`.
///
/// Only the original runs (it allocates the list, the table and the stage and draws the round), checked against the
/// plan made beforehand: the champions of the two halves. Afterwards, the stage of the third slot exists, with +0x3e =
/// 2, +4 = the league's +4 and the two champions in its list (+0xa7, 6 bytes; as a set), and +0x30 = 2.
///
/// [pt] O comparador da final: só a original (aloca a lista, a tabela e a fase e sorteia a rodada), com o plano feito antes:
/// os campeões dos turnos. Depois, a fase da terceira vaga existe, com +0x3e = 2, +4 = o +4 da liga e os dois campeões
/// na lista dela (+0xa7, 6 bytes; como conjunto), e +0x30 = 2.
pub(crate) unsafe extern "thiscall" fn compare_create_championship_final_stage(this: usize) {
    let this = League(this);
    let (a, b) = (half_champion(this, 0), half_champion(this, 1));
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CREATE_CHAMPIONSHIP_FINAL_STAGE)(this.addr());
    let stage = Cup(read_usize(this.stages() + 8));
    let same = !stage.is_null() && {
        let l = stage.team_table();
        let (x, y) = (read_usize(l), read_usize(l + 6));
        stage.team_count() == 2 && read_usize(stage.addr() + 4) == this.competition().addr() && ((x, y) == (a, b) || (x, y) == (b, a)) && this.current_stage() == 2
    };
    SCOREBOARD.record(same, || format!("create_championship_final_stage({:#x}) campeoes={:#x},{:#x} fase={:#x} +0x30={}", this.addr(), a, b, stage.addr(), this.current_stage()));
}

/// Comparer for `jap_j1_start_new_season`.
///
/// [pt] O comparador da virada: só a original (simplification: libera e aloca tabelas e fases, sorteia jogos e árbitros e vira a
/// J2; não pode rodar duas vezes), conferindo o que só ela garante: o ano sobe um e +0x2c = 3.
pub(crate) unsafe extern "thiscall" fn compare_jap_j1_start_new_season(this: usize) -> u32 {
    let this = League(this);
    let before = this.year();
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_JAP_J1_START_NEW_SEASON)(this.addr());
    let after = this.year();
    SCOREBOARD.record(after == before.wrapping_add(1) && this.stage_count() == 3, || format!("jap_j1_start_new_season({:#x}) ano antes={} depois={} +0x2c={}", this.addr(), before, after, this.stage_count()));
    r
}

/// Comparer for `jap_j1_create_table`.
pub(crate) unsafe extern "thiscall" fn compare_jap_j1_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_JAP_J1_CREATE_TABLE);
    compare_japanese_table("jap_j1_create_table", League(this), 0x10, KEY_CLUB_COMP_JAPANESE_J_LEAGUE_1, true, 0x186a0, || o(this))
}

/// Comparador: como o do Goiano.
pub(crate) unsafe extern "thiscall" fn compare_jap_j1_set_up_rules(this: usize) -> u32 {
    let reg = league_regions_without_schedule(this);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_JAP_J1_SET_UP_RULES)(this), || {
        j1_rules(this);
        1
    });
    SCOREBOARD.record(igual && r == n, || format!("jap_j1_set_up_rules({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}

/// Comparer for `jap_j1_create_half_season_stage`.
pub(crate) unsafe extern "thiscall" fn compare_jap_j1_create_half_season_stage(this: usize, k: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_JAP_J1_CREATE_HALF_SEASON_STAGE);
    compare_half_season_stage("jap_j1_create_half_season_stage", League(this), k, || o(this, k))
}
