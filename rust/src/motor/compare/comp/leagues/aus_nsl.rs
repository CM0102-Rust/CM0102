use super::*;

// The comparers of the routines in ../comp/leagues/aus_nsl.rs.
// [pt] Os comparadores das rotinas de ../comp/leagues/aus_nsl.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `aus_nsl_advance_stage`.
pub(crate) unsafe extern "thiscall" fn compare_aus_nsl_advance_stage(this: usize) {
    let this = League(this);
    let before = this.current_stage() as i32;
    let last = before >= (this.stage_count() as i32).wrapping_sub(1);
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_AUS_NSL_ADVANCE_STAGE)(this.addr());
    let after = this.current_stage() as i32;
    SCOREBOARD.record(after == if last { before } else { before + 1 }, || format!("aus_nsl_advance_stage({:#x}) antes={} depois={}", this.addr(), before, after));
}

/// Comparer for `create_finals_series_stage`.
///
/// The finals series comparer: only the original routine runs (it allocates the round table and the stage and sends the
/// news item), with the plan made beforehand: the top six of the table.
///
/// Afterwards, return value 1, the stage in slot 0 with +0x3e = 6, +4 = the league's +4 and the six in its table
/// (+0xb1, as a set), and +0x30 = 0.
///
/// [pt] O comparador da série final: só a original (aloca a tabela das rodadas e a fase e manda a notícia), com o plano feito
/// antes: os seis primeiros da tabela. Depois, a devolução 1, a fase na vaga 0 com +0x3e = 6, +4 = o +4 da liga e os
/// seis na tabela dela (+0xb1, como conjunto), e +0x30 = 0.
pub(crate) unsafe extern "thiscall" fn compare_create_finals_series_stage(this: usize) -> u32 {
    let this = League(this);
    let mut six = [0usize; 6];
    for (i, c) in six.iter_mut().enumerate() {
        *c = aus_nsl_table_row_club(this, i);
    }
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CREATE_FINALS_SERIES_STAGE)(this.addr());
    let stage = League(read_usize(this.stages()));
    let same = r == 1 && !stage.is_null() && stage.club_count() == 6 && stage.competition() == this.competition() && this.current_stage() == 0 && {
        let mut in_it = [0usize; 6];
        for (i, c) in in_it.iter_mut().enumerate() {
            *c = aus_nsl_table_row_club(stage, i);
        }
        let (mut a, mut b) = (six, in_it);
        a.sort_unstable();
        b.sort_unstable();
        a == b
    };
    SCOREBOARD.record(same, || format!("create_finals_series_stage({:#x}) r={} fase={:#x} seis={:x?}", this.addr(), r, stage.addr(), six));
    r
}

/// Comparer for `create_grand_final_stage`.
///
/// The grand final comparer: only the original routine runs (it allocates the list, the table and the stage and draws
/// the round), with the plan made beforehand: the top two of the final series.
///
/// Afterwards, the stage in the second slot exists, with +0x3e = 2, +4 = the league's +4 and the two in its list
/// (+0xa7, 6 bytes; as a set).
///
/// [pt] O comparador da grande final: só a original (aloca a lista, a tabela e a fase e sorteia a rodada), com o plano feito
/// antes: os dois primeiros da série final. Depois, a fase da segunda vaga existe, com +0x3e = 2, +4 = o +4 da liga e
/// os dois na lista dela (+0xa7, 6 bytes; como conjunto).
pub(crate) unsafe extern "thiscall" fn compare_create_grand_final_stage(this: usize) {
    let this = League(this);
    let series = read_usize(this.stages());
    let (a, b) = (aus_nsl_table_row_club(League(series), 0), aus_nsl_table_row_club(League(series), 1));
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CREATE_GRAND_FINAL_STAGE)(this.addr());
    let stage = Cup(read_usize(this.stages() + 4));
    let same = !stage.is_null() && {
        let l = stage.team_table();
        let (x, y) = (read_usize(l), read_usize(l + 6));
        stage.team_count() == 2 && read_usize(stage.addr() + 4) == this.competition().addr() && ((x, y) == (a, b) || (x, y) == (b, a))
    };
    SCOREBOARD.record(same, || format!("create_grand_final_stage({:#x}) finalistas={:#x},{:#x} fase={:#x}", this.addr(), a, b, stage.addr()));
}

/// Comparer for `aus_nsl_set_up_rules`.
///
/// [pt] Comparador: lado a lado com a cópia em ensaio (as regras sem o calendário, que aloca), sobre o objeto da liga
/// (0xee) menos as saídas do calendário (+0x3a, +0xa9, +0xba; a 0x4111a0 não grava nada no objeto além delas); a
/// devolução.
pub(crate) unsafe extern "thiscall" fn compare_aus_nsl_set_up_rules(this: usize) -> u32 {
    let o = League(this);
    let reg = [(o.addr(), 0x3a), (o.addr() + league::LEGS, 0xa9 - 0x3c), (o.addr() + league::MATCH_RULES, 0xba - 0xab), (o.addr() + league::PROMOTED_COUNT, 0xee - 0xbe)];
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_AUS_NSL_SET_UP_RULES)(this), || {
        set_nsl_rules(this);
        1
    });
    SCOREBOARD.record(igual && r == n, || format!("aus_nsl_set_up_rules({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}

/// Comparer for `aus_nsl_update_staff_records_for_status`.
///
/// [pt] Comparador: só a original (os registros alocam) com a devolução prevista pelo ensaio.
pub(crate) unsafe extern "thiscall" fn compare_aus_nsl_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    let (n, _, _) = nsl_end_of_stage(this, club, status, stage, false);
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> u32>(IDX_AUS_NSL_UPDATE_STAFF_RECORDS_FOR_STATUS)(this, club, status, stage, a4, a5, a6);
    SCOREBOARD.record(r == n, || format!("aus_nsl_update_staff_records_for_status({:x}, {:x}, {}, {}) original={} rust={}", this, club, status as u8 as i8, stage as u8 as i8, r as i32, n as i32));
    r
}

/// Comparer for `aus_nsl_create_table`.
pub(crate) unsafe extern "thiscall" fn compare_aus_nsl_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_AUS_NSL_CREATE_TABLE);
    compare_japanese_table("aus_nsl_create_table", League(this), 0xd, KEY_CLUB_COMP_AUSTRALIAN_NATIONAL_SOCCER_LEAGUE, false, 0, || o(this))
}
