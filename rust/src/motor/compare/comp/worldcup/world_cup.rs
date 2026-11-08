use super::*;

// The comparers of the routines in ../comp/worldcup/world_cup.rs.
// [pt] Os comparadores das rotinas de ../comp/worldcup/world_cup.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `draw_groups` (comparison mode: see the header of motor.rs): side by side.
///
/// The original routine and the copy (without counting the random draws) start from the same random-draw state, slots
/// (`+0xee`, 0xa0 bytes), `+0x36`, [`0xb67a34`] and, in the 2002 World Cup, the nations table ([`+0x14`], 32 × 6); the
/// two allocated lists must be equal byte by byte (the copy's is freed, `0x9452ca`).
///
/// [pt] Lado a lado: a original e a cópia (sem contar os sorteios) a partir do mesmo estado do sorteador, das vagas
/// (+0xee, 0xa0 bytes), de +0x36, de [0xb67a34] e, na Copa de 2002, da tabela das nações ([+0x14], 32 × 6); as duas
/// listas alocadas têm de ser iguais byte a byte (a da cópia é liberada, 0x9452ca).
pub(crate) unsafe extern "thiscall" fn compare_draw_groups(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_DRAW_GROUPS);
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.extend_from_slice(&[(this + 0xee, 0xa0), (this + league::ENTRANT_COUNT, 2), (0xb67a34, 4)]);
    if read_i16(this + league::YEAR) == 0x7d2 && read_u32_as_usize(this + league::ENTRANT_BOOK) != 0 {
        reg.push((read_u32_as_usize(this + league::ENTRANT_BOOK), 0xc0));
    }
    let (a, b, igual, _, _) = crate::comparison::side_by_side(&reg, || o(this), || crate::rng::without_counting(|| draw_groups(this)));
    let same = if a != 0 && b != 0 {
        core::slice::from_raw_parts(a as *const u8, 0xa0) == core::slice::from_raw_parts(b as *const u8, 0xa0)
    } else {
        a == 0 && b == 0
    };
    if b != 0 {
        exe::crt_free(b);
    }
    SCOREBOARD.record(igual && same, || format!("draw_groups: memória igual={} lista igual={}", igual, same));
    a
}

/// Comparer for `run_group_draw`.
///
/// [pt] Só a original (o sorteio, os jogos, as notícias e as previsões não rodam duas vezes), conferindo o que ela garante:
/// terminada (+0x45 = 1), as sete fases dos grupos 2..8 existem e a fase atual (+0x30) é a última delas (6).
pub(crate) unsafe extern "thiscall" fn compare_run_group_draw(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_RUN_GROUP_DRAW)(this);
    if read_u32_as_usize(this + league::ACTIVE) == 1 {
        let stages = read_u32_as_usize(this + league::STAGES);
        let ok = stages != 0 && (0..7).all(|g| read_u32_as_usize(stages + g * 4) != 0) && read_u32_as_usize(this + league::CURRENT_STAGE) == 6;
        SCOREBOARD.record(ok, || format!("run_group_draw: fases dos grupos ou +0x30 ({}) fora do esperado", read_u32_as_usize(this + league::CURRENT_STAGE) as i32));
    }
}

/// Comparer for `world_cup_update_for_date`.
///
/// [pt] Só a original (a semeadura, o sorteio e o dia da liga não rodam duas vezes), conferindo o que ela garante: no dia do
/// sorteio (a condição da cópia), a Copa sai com a tabela do grupo 1 (+0xb1) e +0x45 = 1.
pub(crate) unsafe extern "thiscall" fn compare_world_cup_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_WORLD_CUP_UPDATE_FOR_DATE);
    let year = core::ptr::read_unaligned((today + 2) as *const u16) as i32;
    let day = core::ptr::read_unaligned(today as *const u16);
    let draws = read_i16(this + league::NEXT_DRAW_DAY) as u16 == day && read_i16(this + league::NEXT_DRAW_YEAR_OFFSET) as i32 + read_i16(this + league::YEAR) as i32 == year && full != 0 && read_u32_as_usize(this + league::TABLE) == 0;
    let r = o(this, today, full);
    if draws {
        SCOREBOARD.record(read_u32_as_usize(this + league::TABLE) != 0 && read_u32_as_usize(this + league::ACTIVE) == 1, || format!("world_cup_update_for_date: dia do sorteio sem a tabela (+0xb1 = {:#x}, +0x45 = {})", read_u32_as_usize(this + league::TABLE), read_u32_as_usize(this + league::ACTIVE)));
    }
    r
}

/// Comparer for `world_cup_create_group_table`.
///
/// [pt] Só a original (aloca a tabela e manda a notícia dos grupos), conferindo o plano feito antes: devolve 1, +0x3e = 4 e
/// as linhas da tabela (0x3b bytes, +0 o clube) com os clubes do grupo 1 da lista, na ordem.
pub(crate) unsafe extern "thiscall" fn compare_world_cup_create_group_table(this: usize, list: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_WORLD_CUP_CREATE_GROUP_TABLE);
    let plan = group_clubs(list, 1);
    let r = o(this, list);
    if r == 1 {
        let t = read_u32_as_usize(this + league::TABLE);
        let ok = read_i16(this + league::CLUB_COUNT) == 4 && plan.iter().enumerate().all(|(k, &c)| read_u32_as_usize(t + k * 0x3b) == c);
        SCOREBOARD.record(ok, || format!("world_cup_create_group_table: as linhas não têm os clubes do grupo 1 ({:x?})", plan));
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, usize) -> u32>(IDX_WORLD_CUP_CREATE_GROUP_STAGE);
    let plan = group_clubs(list, g + 2);
    let r = o(this, g, list);
    if r == 1 {
        let f = read_u32_as_usize(read_u32_as_usize(this + league::STAGES) + g as usize * 4);
        let t = read_u32_as_usize(f + league::TABLE);
        // the rounds (3) in the date count +0xa9 and the id (0x420 + g) at +0x3a: the order of the constructor’s arguments
        let ok = f != 0 && read_u32_as_usize(this + league::CURRENT_STAGE) as i32 == g && read_i16(f + league::CLUB_COUNT) == read_i16(this + league::CLUB_COUNT) && read_i16(f + league::YEAR) == read_i16(this + league::YEAR)
            && read_i16(f + league::ROUND_COUNT) == 3 && read_i16(f + league::STAGE_ID) as i32 == 0x420 + g
            && (t == 0 || {
                // the stage rows do not follow the list order: checks the set of clubs
                let mut a: Vec<usize> = (0..plan.len()).map(|k| read_u32_as_usize(t + k * 0x3b)).collect();
                let mut b = plan.clone();
                a.sort_unstable();
                b.sort_unstable();
                a == b
            });
        SCOREBOARD.record(ok, || format!("world_cup_create_group_stage({}): a fase não bate com o plano ({:x?})", g, plan));
    }
    r
}

/// Comparer for `world_cup_set_predicted_positions`.
///
/// Only the original routine runs (it adds records to the stats and notifies the manager handler), checking the plan
/// made beforehand: the vector sorted as it sorts (the same qsort, on a copy) and, afterwards, the predicted position
/// (+0x72 of the record found by 0x4a4850) of each of the first 32.
///
/// [pt] Só a original (acrescenta registros à estatística e avisa o gerenciador dos técnicos), conferindo o plano feito
/// antes: o vetor ordenado como ela ordena (a mesma qsort, numa cópia) e, depois, a posição prevista (+0x72 do registro
/// achado por 0x4a4850) de cada um dos 32 primeiros.
pub(crate) unsafe extern "thiscall" fn compare_world_cup_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_WORLD_CUP_SET_PREDICTED_POSITIONS);
    if read_u32_as_usize(this + 8) == 0 || read_i16(this + league::ENTRANT_COUNT) < 0x20 || read_u32_as_usize(this + league::ENTRANT_BOOK) == 0 {
        return o(this);
    }
    let sort: unsafe extern "C" fn(*mut usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let n = read_i16(this + league::ENTRANT_COUNT) as usize;
    let mut v: Vec<usize> = (0..n).map(|i| read_u32_as_usize(read_u32_as_usize(this + league::ENTRANT_BOOK) + i * 6)).collect();
    sort(v.as_mut_ptr(), n as u32, 4, 0x4b07d0);
    o(this);
    const FAIXAS: [u8; 32] = [1, 2, 3, 4, 5, 5, 5, 5, 9, 9, 9, 9, 9, 9, 9, 9, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x19, 0x19, 0x19, 0x19, 0x19, 0x19, 0x19, 0x19];
    let errado = (0..32).find(|&k| {
        let e = if v[k] != 0 { exe::comp_stats_find_club_entry(read_u32_as_usize(this + 8), v[k]) } else { 0 };
        e == 0 || read_u8(e + comp_stats_club::DANGER_POSITION) != FAIXAS[k]
    });
    SCOREBOARD.record(errado.is_none(), || format!("world_cup_set_predicted_positions: a posição prevista da vaga {:?} não bate", errado));
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
    SCOREBOARD.record(igual && outs && table, || format!("world_cup_create_schedule({}): saídas {} tabela {}", stage as u8 as i8, outs, table));
    a
}

/// Comparer for `add_qualified_team`.
///
/// [pt] Lado a lado (+0x36 e a vaga da tabela); lista cheia (a caixa): só a original.
pub(crate) unsafe extern "thiscall" fn compare_add_qualified_team(this: usize, club: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_ADD_QUALIFIED_TEAM);
    let n = read_i16(this + league::ENTRANT_COUNT);
    if n >= read_i16(this + league::ENTRANT_CAPACITY) || read_u32_as_usize(this + league::ENTRANT_BOOK) == 0 {
        return o(this, club);
    }
    let reg = [(this + league::ENTRANT_COUNT, 2usize), (read_u32_as_usize(this + league::ENTRANT_BOOK).wrapping_add((n as i32 * 6) as usize), 6usize)];
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, club), || add_qualified_team(this, club));
    SCOREBOARD.record(igual, || format!("add_qualified_team: {:x?} x {:x?}", mo, mr));
}

/// Comparer for `seed_teams`.
///
/// [pt] Na Copa de 2002 (volta sem gravar nada) lado a lado em +0x4b; nas outras, só a original (caixas, e as saídas da
/// 0x5fa730 que a original pode deixar sem gravar).
pub(crate) unsafe extern "thiscall" fn compare_seed_teams(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_SEED_TEAMS);
    if read_i16(this + league::YEAR) != 0x7d2 {
        return o(this);
    }
    let reg = [(this + league::ENTRANT_COUNT, 0x16usize), (this + 0xee, 0xa0usize)];
    let (_, _, igual, _, _) = crate::comparison::side_by_side(&reg, || o(this), || seed_teams(this));
    SCOREBOARD.record(igual, || "seed_teams: a Copa de 2002 mudou".to_string());
}
