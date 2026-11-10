use super::*;

// The comparers of the routines in ../comp/leagues/usa_mls.rs.
// [pt] Os comparadores das rotinas de ../comp/leagues/usa_mls.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `usa_mls_advance_stage`.
pub(crate) unsafe extern "thiscall" fn compare_usa_mls_advance_stage(this: usize) {
    let this = League(this);
    let before = this.current_stage() as i32;
    let last = before >= (this.stage_count() as i32).wrapping_sub(1);
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_USA_MLS_ADVANCE_STAGE)(this.addr());
    let after = this.current_stage() as i32;
    let expected = if last { before } else if before == 1 || before == 2 { before + 1 } else { before };
    SCOREBOARD.record(after == expected, || format!("usa_mls_advance_stage({:#x}) before={} after={}", this.addr(), before, after));
}

/// Comparer for `usa_mls_create_playoff_stage`.
///
/// The play-off comparer: only the original routine runs (it allocates the list, the table and the stage and draws the
/// round), with the plan made beforehand by the copy of the selection (the same records and the same qsort): the stage
/// in the third slot exists, with +0x3e = 8, +4 = the league's +4 and the eight in its list (+0xa7, 6 bytes; as a set).
///
/// [pt] O comparador do play-off: só a original (aloca a lista, a tabela e a fase e sorteia a rodada), com o plano feito
/// antes pela cópia da escolha (os mesmos registros e o mesmo qsort): a fase da terceira vaga existe, com +0x3e = 8, +4
/// = o +4 da liga e os oito na lista dela (+0xa7, 6 bytes; como conjunto).
pub(crate) unsafe extern "thiscall" fn compare_usa_mls_create_playoff_stage(this: usize) {
    let this = League(this);
    let (a, b) = (read_usize(this.stages()), read_usize(this.stages() + 4));
    let mut plan = [0usize; 8];
    let mut r = [0u8; 54];
    put_club_points_record(&mut r, 0, table_row_club_points(this, 0));
    put_club_points_record(&mut r, 1, table_row_club_points(League(a), 0));
    put_club_points_record(&mut r, 2, table_row_club_points(League(b), 0));
    exe::crt_qsort((r.as_mut_ptr()) as usize, 3, 6, 0x90a130);
    for k in 0..3 {
        plan[k] = club_of_record(&r, k);
    }
    let mut k = 0usize;
    for j in 1..4 {
        for t in [this.addr(), a, b] {
            put_club_points_record(&mut r, k, table_row_club_points(League(t), j));
            k += 1;
        }
    }
    exe::crt_qsort((r.as_mut_ptr()) as usize, 9, 6, 0x90a130);
    for k in 0..5 {
        plan[3 + k] = club_of_record(&r, k);
    }
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_USA_MLS_CREATE_PLAYOFF_STAGE)(this.addr());
    let stage = Cup(read_usize(this.stages() + 8));
    let same = !stage.is_null() && stage.team_count() == 8 && read_usize(stage.addr() + 4) == this.competition().addr() && {
        let mut in_it = [0usize; 8];
        for (i, c) in in_it.iter_mut().enumerate() {
            *c = read_usize(stage.team_table() + 6 * i);
        }
        let mut p = plan;
        p.sort_unstable();
        in_it.sort_unstable();
        p == in_it
    };
    SCOREBOARD.record(same, || format!("usa_mls_create_playoff_stage({:#x}) stage={:#x} plan={:x?}", this.addr(), stage.addr(), plan));
}

/// Comparer for `usa_mls_create_final_stage` (comparison mode: see the header of motor.rs): the comparer of the final;
/// only the original routine runs (it allocates the list, the table and the stage and draws the round), with the plan
/// made beforehand.
///
/// The plan: the first two winners of the play-off. Afterwards: the stage in the fourth slot exists, with `+0x3e` = 2,
/// `+4` = the league's `+4` and the two in its list (as a set).
///
/// [pt] O comparador da final: só a original (aloca a lista, a tabela e a fase e sorteia a rodada), com o plano feito antes:
/// os dois primeiros vencedores do play-off. Depois, a fase da quarta vaga existe, com +0x3e = 2, +4 = o +4 da liga e
/// os dois na lista dela (como conjunto).
pub(crate) unsafe extern "thiscall" fn compare_usa_mls_create_final_stage(this: usize) {
    let this = League(this);
    let po = Cup(read_usize(this.stages() + 8));
    let mut v = [0usize; 2];
    let mut n = 0usize;
    let mut i = 0i32;
    while i < po.team_count() as i32 && n < 2 {
        let e = po.team_table() + 6 * i as usize;
        if read_u8(e + 5) == 1 {
            v[n] = read_usize(e);
            n += 1;
        }
        i += 1;
    }
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_USA_MLS_CREATE_FINAL_STAGE)(this.addr());
    let stage = Cup(read_usize(this.stages() + 0xc));
    // simplification: with fewer than two winners the original reads uninitialized slots from the list; then only the stage is checked.
    let same = !stage.is_null() && n < 2 || !stage.is_null() && {
        let l = stage.team_table();
        let (x, y) = (read_usize(l), read_usize(l + 6));
        stage.team_count() == 2 && read_usize(stage.addr() + 4) == this.competition().addr() && ((x, y) == (v[0], v[1]) || (x, y) == (v[1], v[0]))
    };
    SCOREBOARD.record(same, || format!("usa_mls_create_final_stage({:#x}) finalistas={:x?} n={} stage={:#x}", this.addr(), v, n, stage.addr()));
}

/// Comparer for `usa_mls_create_table`.
///
/// [pt] O comparador da tabela: só a original (simplification: aloca a tabela e as listas das linhas), com o plano feito antes:
/// os clubes da competição com o código 0x413, na ordem do banco, até 4. Depois, a devolução, +0x3e = 4 e as linhas.
pub(crate) unsafe extern "thiscall" fn compare_usa_mls_create_table(this: usize) -> u32 {
    let this = League(this);
    let mut plan = [0usize; 4];
    let mut n = 0usize;
    let base = read_usize(G_CLUB_TABLE);
    let mut i: i16 = 0;
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 && n < 4 {
        let c = base + i as u16 as usize * club::SIZE;
        if is_in_mls_division(this, Club(c), 0x413) {
            plan[n] = c;
            n += 1;
        }
        i = i.wrapping_add(1);
    }
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_USA_MLS_CREATE_TABLE)(this.addr());
    let t = this.table();
    let same = r == (n == 4) as u32 && this.club_count() == 4 && t != 0 && (0..n).all(|k| read_usize(t + k * 0x3b) == plan[k]);
    SCOREBOARD.record(same, || format!("usa_mls_create_table({:#x}) r={} n={} plan={:x?}", this.addr(), r, n, plan));
    r
}

/// Comparer for `create_second_division_stage`.
pub(crate) unsafe extern "thiscall" fn compare_create_second_division_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CREATE_SECOND_DIVISION_STAGE);
    compare_division_stage("create_second_division_stage", League(this), 0x47b, 0, || o(this))
}

/// Comparer for `create_third_division_stage`.
pub(crate) unsafe extern "thiscall" fn compare_create_third_division_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CREATE_THIRD_DIVISION_STAGE);
    compare_division_stage("create_third_division_stage", League(this), 0x414, 1, || o(this))
}

/// Comparer for `usa_mls_start_new_season`.
///
/// The season rollover comparer: only the original routine runs (simplification: it frees tables and stages, allocates
/// the new ones, draws the fixtures and the referees; it cannot run twice), checking what only it guarantees: the year
/// + 1 and the three divisions built (+0x3e = 4 and slots 0 and 1 of the stages).
///
/// [pt] O comparador da virada: só a original (simplification: libera tabelas e fases, aloca as novas, sorteia os jogos e os
/// árbitros; não pode rodar duas vezes), conferindo o que só ela garante: o ano + 1 e as três divisões montadas
/// (+0x3e = 4 e as vagas 0 e 1 das fases).
pub(crate) unsafe extern "thiscall" fn compare_usa_mls_start_new_season(this: usize) -> u32 {
    let this = League(this);
    let before = this.year();
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_USA_MLS_START_NEW_SEASON)(this.addr());
    let after = this.year();
    let same = after == before.wrapping_add(1) && this.club_count() == 4 && read_usize(this.stages()) != 0 && read_usize(this.stages() + 4) != 0;
    SCOREBOARD.record(same, || format!("usa_mls_start_new_season({:#x}) year before={} after={} +0x3e={}", this.addr(), before, after, this.club_count()));
    r
}

/// Comparer for `usa_mls_set_predicted_positions`.
///
/// The predicted positions comparer: only the original routine runs (simplification: `set_club_final_position` adds the record of a club
/// that has none; it cannot run twice), with the plan made beforehand: the twelve clubs of the three tables, each block
/// sorted by the same qsort on a copy; afterwards, the position (+0x72 of the record found by `exe::comp_stats_find_club_entry`) of each one.
///
/// [pt] O comparador das posições previstas: só a original (simplification: a 0x4a2540 acrescenta o registro de um clube que não
/// tem; não pode rodar duas vezes), com o plano feito antes: os doze clubes das três tabelas, cada bloco ordenado pela
/// mesma qsort numa cópia; depois, a posição (+0x72 do registro achado por 0x4a4850) de cada um.
pub(crate) unsafe extern "thiscall" fn compare_usa_mls_set_predicted_positions(this: usize) {
    let this = League(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_USA_MLS_SET_PREDICTED_POSITIONS);
    let n = this.club_count() as usize;
    if this.comp_stats() == 0 || n != 4 {
        return o(this.addr());
    }
    let mut v = [0usize; 12];
    for k in 0..4 {
        v[k] = aus_nsl_table_row_club(this, k);
        v[4 + k] = aus_nsl_table_row_club(League(read_usize(this.stages())), k);
        v[8 + k] = aus_nsl_table_row_club(League(read_usize(this.stages() + 4)), k);
    }
    for b in 0..3 {
        exe::crt_qsort((v[4 * b..].as_mut_ptr()) as usize, 4, 4, 0x4b07d0);
    }
    o(this.addr());
    const LUGARES: [u8; 12] = [1, 4, 7, 10, 1, 4, 7, 10, 1, 4, 7, 10];
    let wrong = (0..12).find(|&k| {
        let e = if v[k] != 0 { exe::comp_stats_find_club_entry(this.comp_stats(), v[k]) } else { 0 };
        e == 0 || read_u8(e + comp_stats_club::DANGER_POSITION) != LUGARES[k]
    });
    SCOREBOARD.record(wrong.is_none(), || format!("usa_mls_set_predicted_positions({:#x}): a position of the slot {:?} not bate ({:x?})", this.addr(), wrong, v));
}

/// Comparer for `usa_mls_get_club_array`.
///
/// [pt] O comparador da lista: só a original (aloca a lista, que fica com quem chamou), com o plano: n = 12 e os doze clubes
/// das três tabelas na ordem.
pub(crate) unsafe extern "thiscall" fn compare_usa_mls_get_club_array(this: usize, list: *mut usize, n: *mut i16) {
    let this = League(this);
    original_fn::<unsafe extern "thiscall" fn(usize, *mut usize, *mut i16)>(IDX_USA_MLS_GET_CLUB_ARRAY)(this.addr(), list, n);
    let m = this.club_count() as usize;
    let same = *n == 0xc && *list != 0 && (0..m).all(|i| {
        read_usize(*list + 4 * i) == aus_nsl_table_row_club(this, i)
            && read_usize(*list + 4 * (m + i)) == aus_nsl_table_row_club(League(read_usize(this.stages())), i)
            && read_usize(*list + 4 * (2 * m + i)) == aus_nsl_table_row_club(League(read_usize(this.stages() + 4)), i)
    });
    SCOREBOARD.record(same, || format!("usa_mls_get_club_array({:#x}) n={}", this.addr(), *n));
}

/// Comparer for `usa_mls_update_staff_records_for_status`.
///
/// [pt] Comparador: só a original (os registros alocam e vão para o histórico do técnico; não rodam duas vezes) com a
/// devolução prevista pelo ensaio.
pub(crate) unsafe extern "thiscall" fn compare_usa_mls_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, a4: u32, game: usize, a6: u32) -> u32 {
    let n = mls_stage_outcome(this, club, status, stage, game, false);
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_USA_MLS_UPDATE_STAFF_RECORDS_FOR_STATUS)(this, club, status, stage, a4, game, a6);
    SCOREBOARD.record(r == n, || format!("usa_mls_update_staff_records_for_status({:x}, {:x}, {}, {}) original={} rust={}", this, club, status as u8 as i8, stage as u8 as i8, r as i32, n as i32));
    r
}

/// Comparer for `usa_mls_add_result`.
///
/// [pt] Comparador: só a original (simplification: o resultado entra nas tabelas; não roda duas vezes), com o plano: a devolução
/// 1, a marca +0x42 do jogo (1 no estágio 1; senão -1 se estava abaixo de 2) e o bloco +8 devolvido.
pub(crate) unsafe extern "thiscall" fn compare_usa_mls_add_result(this: usize, game: usize, stats: u32) -> u32 {
    let game = Fixture(game);
    let this = League(this);
    let mark = game.stage_index() as i8;
    let block = this.comp_stats();
    let expected = if this.current_stage() == 1 { 1 } else if mark < 2 { -1 } else { mark };
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_USA_MLS_ADD_RESULT)(this.addr(), game.addr(), stats);
    let (m, b) = ((game.stage_index() as i8), this.comp_stats());
    SCOREBOARD.record(r == 1 && m == expected && b == block, || format!("usa_mls_add_result({:x}, {:x}) returned {} mark {} x {} block {:x} x {:x}", this.addr(), game.addr(), r, m, expected, b, block));
    r
}

/// Comparer for `usa_mls_get_club_all_star_conference`.
///
/// [pt] Comparador: a devolução (só lê; o caso de erro fica com a original sozinha).
pub(crate) unsafe extern "C" fn compare_usa_mls_get_club_all_star_conference(club: usize) -> u16 {
    let club = Club(club);
    let r = original_fn::<unsafe extern "C" fn(usize) -> u16>(IDX_USA_MLS_GET_CLUB_ALL_STAR_CONFERENCE)(club.addr());
    let nation = club.division().addr();
    if nation != 0 && read_usize(nation) == read_usize(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE) && r == 0 {
        return r; // the error case (the box has already been shown in the original)
    }
    let n = usa_mls_get_club_all_star_conference(club.addr());
    SCOREBOARD.record(r == n, || format!("usa_mls_get_club_all_star_conference({:x}) original={:x} rust={:x}", club.addr(), r, n));
    r
}

/// Comparer for `usa_mls_set_up_rules`.
///
/// [pt] Comparador: lado a lado sobre o objeto da liga (0xee; só grava nele); a devolução.
pub(crate) unsafe extern "thiscall" fn compare_usa_mls_set_up_rules(this: usize) -> u32 {
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&[(this, 0xee)], || original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_USA_MLS_SET_UP_RULES)(this), || usa_mls_set_up_rules(this));
    SCOREBOARD.record(igual && r == n, || format!("usa_mls_set_up_rules({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}

/// Comparador: como o do Mineiro.
pub(crate) unsafe extern "thiscall" fn compare_usa_mls_set_club_progress(this: usize, club: usize, stage: u32, a: u32, b: u32, c: u32) -> usize {
    let e = bra_reg_minas_gerais_comp_stats_club_entry(this, club);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&[(e + comp_stats_club::CURRENT_POSITION, 3)], || original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> usize>(IDX_USA_MLS_SET_CLUB_PROGRESS)(this, club, stage, a, b, c), || usa_mls_set_club_progress(this, club, stage, a, b, c));
    SCOREBOARD.record(igual && r == n, || format!("usa_mls_set_club_progress({:x}, {:x}, {}, {}, {}, {}) original={:x} rust={:x} {:?} {:?}", this, club, stage as u8 as i8, a as u8, b as u8, c as u8, r, n, mo, mr));
    r
}
