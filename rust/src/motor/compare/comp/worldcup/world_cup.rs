use super::*;

// The comparers of the routines in ../comp/worldcup/world_cup.rs.
// [pt] Os comparadores das rotinas de ../comp/worldcup/world_cup.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `draw_groups` (comparison mode: see the header of motor.rs): side by side.
///
/// The original routine and the copy (without counting the random draws) start from the same random-draw state, slots
/// (`+0xee`, 0xa0 bytes), `+0x36`, [`0xb67a34`] and, in the 2002 World Cup, the nations table ([`+0x14`], 32 × 6); the
/// two allocated lists must be equal byte by read_i8(the copy's is freed + (`0x9452ca`)).
///
/// [pt] Lado a lado: a original e a cópia (sem contar os sorteios) a partir do mesmo estado do sorteador, das vagas
/// (+0xee, 0xa0 bytes), de +0x36, de [0xb67a34] e, na Copa de 2002, da tabela das nações ([+0x14], 32 × 6); as duas
/// listas alocadas têm de ser iguais byte a read_i8(a da cópia é liberada + 0x9452ca).
pub(crate) unsafe extern "thiscall" fn compare_draw_groups(this: usize) -> usize {
    let this = League(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_DRAW_GROUPS);
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.extend_from_slice(&[(this.addr() + 0xee, 0xa0), (this.addr() + league::ENTRANT_COUNT, 2), (G_MOUSE_LEFT_BUTTON_DOWN, 4)]);
    if this.year() == 0x7d2 && this.entrant_book() != 0 {
        reg.push((this.entrant_book(), 0xc0));
    }
    let (a, b, igual, _, _) = crate::comparison::side_by_side(&reg, || o(this.addr()), || crate::rng::without_counting(|| draw_groups(this.addr())));
    let same = if a != 0 && b != 0 {
        core::slice::from_raw_parts(a as *const u8, 0xa0) == core::slice::from_raw_parts(b as *const u8, 0xa0)
    } else {
        a == 0 && b == 0
    };
    if b != 0 {
        exe::crt_free(b);
    }
    SCOREBOARD.record(igual && same, || format!("draw_groups: memory equal={} list equal={}", igual, same));
    a
}

/// Comparer for `run_group_draw`.
///
/// [pt] Só a original (o sorteio, os jogos, as notícias e as previsões não rodam duas vezes), conferindo o que ela garante:
/// terminada (+0x45 = 1), as sete fases dos grupos 2..8 existem e a fase atual (+0x30) é a última delas (6).
pub(crate) unsafe extern "thiscall" fn compare_run_group_draw(this: usize) {
    let this = League(this);
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_RUN_GROUP_DRAW)(this.addr());
    if this.active() == 1 {
        let stages = this.stages();
        let ok = stages != 0 && (0..7).all(|g| read_usize(stages + g * 4) != 0) && this.current_stage() == 6;
        SCOREBOARD.record(ok, || format!("run_group_draw: stages of the groups ou +0x30 ({}) away of the expected", this.current_stage() as i32));
    }
}

/// Comparer for `world_cup_update_for_date`.
///
/// [pt] Só a original (a semeadura, o sorteio e o dia da liga não rodam duas vezes), conferindo o que ela garante: no dia do
/// sorteio (a condição da cópia), a Copa sai com a tabela do grupo 1 (+0xb1) e +0x45 = 1.
pub(crate) unsafe extern "thiscall" fn compare_world_cup_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let this = League(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_WORLD_CUP_UPDATE_FOR_DATE);
    let year = read_u16(today + 2) as i32;
    let day = read_u16(today);
    let draws = this.next_draw_day() as u16 == day && this.next_draw_year_offset() as i32 + this.year() as i32 == year && full != 0 && this.table() == 0;
    let r = o(this.addr(), today, full);
    if draws {
        SCOREBOARD.record(this.table() != 0 && this.active() == 1, || format!("world_cup_update_for_date: day of the draw without a table (+0xb1 = {:#x}, +0x45 = {})", this.table(), this.active()));
    }
    r
}

/// Comparer for `world_cup_create_group_table`.
///
/// [pt] Só a original (aloca a tabela e manda a notícia dos grupos), conferindo o plano feito antes: devolve 1, +0x3e = 4 e
/// as linhas da tabela (0x3b bytes, +0 o clube) com os clubes do grupo 1 da lista, na ordem.
pub(crate) unsafe extern "thiscall" fn compare_world_cup_create_group_table(this: usize, list: usize) -> u32 {
    let this = League(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_WORLD_CUP_CREATE_GROUP_TABLE);
    let plan = group_clubs(list, 1);
    let r = o(this.addr(), list);
    if r == 1 {
        let t = this.table();
        let ok = this.club_count() == 4 && plan.iter().enumerate().all(|(k, &c)| read_usize(t + k * 0x3b) == c);
        SCOREBOARD.record(ok, || format!("world_cup_create_group_table: as rows not has the clubs of the group 1 ({:x?})", plan));
    }
    r
}

/// Comparer for `world_cup_create_group_stage`.
///
/// Only the original runs (it allocates the stage and sends the news item), checking the plan: it returns 1 with the
/// stage in [+0xc][g], +0x30 = g, the stage with the World Cup's team count (+0x3e) and year and, in the stage table
/// (+0xb1, rows of 0x3b bytes), the clubs of group g+2 of the list (in any order: in the 02/11 comparison the stage
/// rows did not come out in list order [inference: the stage sorts or draws its own table]).
///
/// [pt] Só a original (aloca a fase e manda a notícia), conferindo o plano: devolve 1 com a fase em [+0xc][g], +0x30 = g, a
/// fase com os times (+0x3e) da Copa, o ano dela e, na tabela da fase (+0xb1, linhas de 0x3b bytes), os clubes do
/// grupo g+2 da lista (em qualquer ordem: na comparação de 02/11 as linhas da fase não
/// saíram na ordem da lista [inferência: a fase ordena ou sorteia a tabela dela]).
pub(crate) unsafe extern "thiscall" fn compare_world_cup_create_group_stage(this: usize, g: i32, list: usize) -> u32 {
    let this = League(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, usize) -> u32>(IDX_WORLD_CUP_CREATE_GROUP_STAGE);
    let plan = group_clubs(list, g + 2);
    let r = o(this.addr(), g, list);
    if r == 1 {
        let f = League(read_usize(this.stages() + g as usize * 4));
        let t = f.table();
        // the rounds (3) in the date count +0xa9 and the id (0x420 + g) at +0x3a: the order of the constructor’s arguments
        let ok = !f.is_null() && this.current_stage() as i32 == g && f.club_count() == this.club_count() && f.year() == this.year()
            && f.round_count() == 3 && f.stage_id() as i32 == 0x420 + g
            && (t == 0 || {
                // the stage rows do not follow the list order: checks the set of clubs
                let mut a: Vec<usize> = (0..plan.len()).map(|k| read_usize(t + k * 0x3b)).collect();
                let mut b = plan.clone();
                a.sort_unstable();
                b.sort_unstable();
                a == b
            });
        SCOREBOARD.record(ok, || format!("world_cup_create_group_stage({}): a stage not bate with o plan ({:x?})", g, plan));
    }
    r
}

/// Comparer for `world_cup_set_predicted_positions`.
///
/// Only the original routine runs (it adds records to the stats and notifies the manager handler), checking the plan
/// made beforehand: the vector sorted as it sorts (the same qsort, on a copy) and, afterwards, the predicted position
/// (+0x72 of the record found by `exe::comp_stats_find_club_entry`) of each of the first 32.
///
/// [pt] Só a original (acrescenta registros à estatística e avisa o gerenciador dos técnicos), conferindo o plano feito
/// antes: o vetor ordenado como ela ordena (a mesma qsort, numa cópia) e, depois, a posição prevista (+0x72 do registro
/// achado por 0x4a4850) de cada um dos 32 primeiros.
pub(crate) unsafe extern "thiscall" fn compare_world_cup_set_predicted_positions(this: usize) {
    let this = League(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_WORLD_CUP_SET_PREDICTED_POSITIONS);
    if this.comp_stats() == 0 || this.entrant_count() < 0x20 || this.entrant_book() == 0 {
        return o(this.addr());
    }
    let n = this.entrant_count() as usize;
    let mut v: Vec<usize> = (0..n).map(|i| read_usize(this.entrant_book() + i * 6)).collect();
    exe::crt_qsort((v.as_mut_ptr()) as usize, n as u32, 4, 0x4b07d0);
    o(this.addr());
    const FAIXAS: [u8; 32] = [1, 2, 3, 4, 5, 5, 5, 5, 9, 9, 9, 9, 9, 9, 9, 9, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x19, 0x19, 0x19, 0x19, 0x19, 0x19, 0x19, 0x19];
    let wrong = (0..32).find(|&k| {
        let e = CompStatsClub(if v[k] != 0 { exe::comp_stats_find_club_entry(this.comp_stats(), v[k]) } else { 0 });
        e.is_null() || e.danger_position() != FAIXAS[k]
    });
    SCOREBOARD.record(wrong.is_none(), || format!("world_cup_set_predicted_positions: a position prevista of the slot {:?} not bate", wrong));
}

/// Comparer for `world_cup_create_schedule`.
///
/// Side by side in the group stages: the original routine with the caller's outputs and the copy with its own outputs,
/// from the same [0xb67a34]; the outputs and the bytes written to the two allocated tables must be equal (the copy's is
/// freed).
///
/// The knockout: only the original.
///
/// [pt] Lado a lado nas fases de grupos: a original com as saídas de quem chama e a cópia com saídas próprias, a partir do
/// mesmo [0xb67a34]; as saídas e os bytes gravados das duas tabelas alocadas têm de ser iguais (a da cópia é liberada).
/// O mata-mata: só a original.
pub(crate) unsafe extern "thiscall" fn compare_world_cup_create_schedule(this: usize, stage: u32, rounds: *mut i16, id: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_WORLD_CUP_CREATE_SCHEDULE);
    if stage as u8 as i8 >= 7 {
        return o(this, stage, rounds, id, league);
    }
    let (mut r2, mut i2, mut l2) = (0i16, 0i16, 0i32);
    let reg = [(0xb67a34usize, 4usize)];
    let (a, b, igual, _, _) = crate::comparison::side_by_side(
        &reg,
        || o(this, stage, rounds, id, league),
        || world_cup_create_schedule(this, stage, &mut r2, &mut i2, if league.is_null() { core::ptr::null_mut() } else { &mut l2 }),
    );
    let outs = *rounds == r2 && *id == i2 && (league.is_null() || *league == l2);
    let table = if a != 0 && b != 0 { written_group_round_bytes(a) == written_group_round_bytes(b) } else { a == 0 && b == 0 };
    if b != 0 {
        exe::crt_free(b);
    }
    SCOREBOARD.record(igual && outs && table, || format!("world_cup_create_schedule({}): outs {} table {}", stage as u8 as i8, outs, table));
    a
}

/// Comparer for `add_qualified_team`.
///
/// [pt] Lado a lado (+0x36 e a vaga da tabela); lista cheia (a caixa): só a original.
pub(crate) unsafe extern "thiscall" fn compare_add_qualified_team(this: usize, club: usize) {
    let this = League(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_ADD_QUALIFIED_TEAM);
    let n = this.entrant_count();
    if n >= this.entrant_capacity() || this.entrant_book() == 0 {
        return o(this.addr(), club);
    }
    let reg = [(this.addr() + league::ENTRANT_COUNT, 2usize), (this.entrant_book().wrapping_add((n as i32 * 6) as usize), 6usize)];
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this.addr(), club), || add_qualified_team(this.addr(), club));
    SCOREBOARD.record(igual, || format!("add_qualified_team: {:x?} x {:x?}", mo, mr));
}

/// Comparer for `seed_teams`.
///
/// [pt] Na Copa de 2002 (volta sem gravar nada) lado a lado em +0x4b; nas outras, só a original (caixas, e as saídas da
/// 0x5fa730 que a original pode deixar sem gravar).
pub(crate) unsafe extern "thiscall" fn compare_seed_teams(this: usize) {
    let this = League(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_SEED_TEAMS);
    if this.year() != 0x7d2 {
        return o(this.addr());
    }
    let reg = [(this.addr() + league::ENTRANT_COUNT, 0x16usize), (this.addr() + 0xee, 0xa0usize)];
    let (_, _, igual, _, _) = crate::comparison::side_by_side(&reg, || o(this.addr()), || seed_teams(this.addr()));
    SCOREBOARD.record(igual, || "seed_teams: a Copa de 2002 mudou".to_string());
}
