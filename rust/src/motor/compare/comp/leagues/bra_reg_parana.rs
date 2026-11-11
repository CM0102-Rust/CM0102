use super::*;

// The comparers of the routines in ../comp/leagues/bra_reg_parana.rs.
// [pt] Os comparadores das rotinas de ../comp/leagues/bra_reg_parana.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `bra_reg_parana_advance_stage` (comparison mode: see the header of motor.rs): the comparer of the next
/// stage; only the original routine runs (simplification: on reaching 2 it builds the final, which allocates and makes
/// a random draw; it cannot run twice), checking the plan.
///
/// At the last stage, `+0x30` unchanged; otherwise `+0x30` + 1, or, on reaching 2, 2 (with a final) or 1 (the same
/// champion in both halves, which the final records).
///
/// [pt] O comparador da fase seguinte: só a original (simplification: ao chegar a 2 monta a final, que aloca e sorteia; não pode
/// rodar duas vezes), conferindo o plano: na última fase, +0x30 igual; senão +0x30 + 1, ou, ao chegar a 2, 2 (com
/// final) ou 1 (o mesmo campeão nos dois turnos, que a final grava).
pub(crate) unsafe extern "thiscall" fn compare_bra_reg_parana_advance_stage(this: usize) {
    let this = League(this);
    let before = this.current_stage() as i32;
    let last = before >= (this.stage_count() as i32).wrapping_sub(1);
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_REG_PARANA_ADVANCE_STAGE)(this.addr());
    let after = this.current_stage() as i32;
    let same = if last { after == before } else if before + 1 == 2 { after == 2 || after == 1 } else { after == before + 1 };
    SCOREBOARD.record(same, || format!("bra_reg_parana_advance_stage({:#x}) before={} after={}", this.addr(), before, after));
}

/// Comparer for `bra_reg_parana_create_final_stage`.
///
/// The final's comparer: only the original routine runs (it allocates the list, the table and the stage and draws the
/// round), checking the plan made beforehand: the champions of the two halves.
///
/// Same club in both: +0x30 = 1, +0x2c = 2 and status 5 on the first row of each half and on its row of the league
/// table. Different: the stage in the third slot exists, with +0x3e = 2, +4 = the league's +4 and the two champions in
/// its list (+0xa7, 6 bytes; as a set: the round draw may reorder them), and +0x30 = 2.
///
/// [pt] O comparador da final: só a original (aloca a lista, a tabela e a fase e sorteia a rodada), conferindo o plano feito
/// antes: os campeões dos turnos. O mesmo nos dois: +0x30 = 1, +0x2c = 2 e a situação 5 na primeira linha de cada turno
/// e na linha dele da tabela da liga. Diferentes: a fase da terceira vaga existe, com +0x3e = 2, +4 = o +4 da liga e os
/// dois campeões na lista dela (+0xa7, 6 bytes; como conjunto: o sorteio da rodada pode reordenar), e +0x30 = 2.
pub(crate) unsafe extern "thiscall" fn compare_bra_reg_parana_create_final_stage(this: usize) {
    let this = League(this);
    let (a, b) = (half_champion(this, 0), half_champion(this, 1));
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_REG_PARANA_CREATE_FINAL_STAGE)(this.addr());
    let same = if a == b {
        let mark = |k: usize| read_u8(read_usize(read_usize(this.stages() + 4 * k) + 0xb1) + 0x37) == 5;
        let in_league = (0..this.club_count().max(0) as usize).all(|i| {
            let row = LeagueTableRow(this.table() + i * 0x3b);
            read_usize(row.addr()) != a || row.status() == table_status::CHAMPIONS
        });
        this.current_stage() == 1 && this.stage_count() == 2 && mark(0) && mark(1) && in_league
    } else {
        let stage = Cup(read_usize(this.stages() + 8));
        if stage.is_null() {
            false
        } else {
            let l = stage.team_table();
            let (x, y) = (read_usize(l), read_usize(l + 6));
            stage.team_count() == 2 && read_usize(stage.addr() + 4) == this.competition().addr() && ((x, y) == (a, b) || (x, y) == (b, a)) && this.current_stage() == 2
        }
    };
    SCOREBOARD.record(same, || format!("bra_reg_parana_create_final_stage({:#x}) champions={:#x},{:#x} +0x30={} +0x2c={}", this.addr(), a, b, this.current_stage(), this.stage_count()));
}

/// Comparer for `bra_reg_parana_record_season_honours` (end of season; comparison mode: see the header of motor.rs).
///
/// Only the original runs (simplification: it writes the history, which allocates the record, pays the prizes and sends
/// the news item, and may set up the Champions Cup; it cannot run twice). The copy runs in Rust mode (counters "cqo_8
/// historico da final" and "cqo_8 prizes") and is proved by the mutations and by the two Rust careers.
///
/// [pt] Comparador do fim da temporada: só a original (simplification: grava o histórico, que aloca o registro, paga os prêmios e
/// manda a notícia, e pode montar a Copa dos Campeões; não pode rodar duas vezes). A cópia roda no modo Rust (contadores
/// "cqo_8 final history" e "cqo_8 prizes") e é provada pelas mutações e pelas duas carreiras Rust.
pub(crate) unsafe extern "thiscall" fn compare_bra_reg_parana_record_season_honours(this: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_REG_PARANA_RECORD_SEASON_HONOURS)(this)
}

/// Comparer for `bra_reg_parana_set_up_rules`.
///
/// [pt] Comparador: lado a lado com a cópia em ensaio (as regras sem o calendário, que aloca), sobre o objeto menos as
/// saídas do calendário; a devolução.
pub(crate) unsafe extern "thiscall" fn compare_bra_reg_parana_set_up_rules(this: usize) -> u32 {
    let reg = league_regions_without_schedule(this);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_REG_PARANA_SET_UP_RULES)(this), || {
        parana_rules(this);
        1
    });
    SCOREBOARD.record(igual && r == n, || format!("bra_reg_parana_set_up_rules({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}

/// Comparer for `construct_bra_reg_parana`.
pub(crate) unsafe extern "thiscall" fn compare_construct_bra_reg_parana(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_BRA_REG_PARANA);
    bra_reg_northeast_compare_state_league_ctor("construct_bra_reg_parana", League(this), year, record, &PARANA_CHAMPIONSHIP, || o(this, year, record))
}

/// Comparer for `bra_reg_parana_create_table`.
pub(crate) unsafe extern "thiscall" fn compare_bra_reg_parana_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_REG_PARANA_CREATE_TABLE);
    bra_reg_rio_compare_state_league_table("bra_reg_parana_create_table", League(this), 0xc, KEY_CLUB_COMP_BRAZILIAN_PARANA_STATE_CHAMPIONSHIP, || o(this))
}

/// Comparer for `create_half_stage`.
pub(crate) unsafe extern "thiscall" fn compare_create_half_stage(this: usize, k: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_CREATE_HALF_STAGE);
    compare_half_season_stage("create_half_stage", League(this), k, || o(this, k))
}

/// Comparer for `bra_reg_parana_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_bra_reg_parana_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_REG_PARANA_START_NEW_SEASON);
    compare_new_season("bra_reg_parana_start_new_season", League(this), || o(this))
}
