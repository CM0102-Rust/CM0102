use super::*;

// The comparers of the routines in ../comp/eurocomp/european_cup.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/eurocomp/european_cup.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `draw_phase1_groups` (comparison mode: see the header of motor.rs).
///
/// [pt] Só a original (sorteia, paga prêmios, aloca as fases e manda notícias), conferindo o plano feito antes: com 32
/// clubes, as 8 fases em [+0xc], +0x30 = 7 e os clubes das tabelas das fases (+0xb1, linhas de 0x3b bytes, 4 por
/// fase) são exatamente os 32 do plano.
pub(crate) unsafe extern "thiscall" fn comparar_draw_phase1_groups(this: usize) {
    let plan = group_draw_clubs(this);
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_DRAW_PHASE1_GROUPS)(this);
    if plan.len() != 32 || read_u32_as_usize(read_u32_as_usize(G_COMP_OBJECT_TABLE) + read_u32_as_usize(KEY_CLUB_COMP_UEFA_CUP) * 4) == 0 {
        return;
    }
    let stages = read_u32_as_usize(this + 0xc);
    let mut vistos = Vec::with_capacity(32);
    let mut ok = read_u32_as_usize(this + 0x30) == 7;
    for g in 0..8usize {
        let f = read_u32_as_usize(stages + g * 4);
        if f == 0 || read_u32_as_usize(f + 0xb1) == 0 {
            ok = false;
            break;
        }
        for r in 0..4usize {
            vistos.push(read_u32_as_usize(read_u32_as_usize(f + 0xb1) + r * 0x3b));
        }
    }
    vistos.sort_unstable();
    ok = ok && vistos == plan;
    PLACAR.record(ok, || format!("draw_phase1_groups: as fases não têm os 32 clubes do plano (+0x30 = {})", read_u32_as_usize(this + 0x30) as i32));
}

/// Comparer for `european_cup_compare_clubs_by_seed` (comparison mode: see the header of motor.rs).
///
/// [pt] Pura: a devolução.
pub(crate) unsafe extern "C" fn comparar_european_cup_compare_clubs_by_seed(a: *const usize, b: *const usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(*const usize, *const usize) -> i32>(IDX_EUROPEAN_CUP_COMPARE_CLUBS_BY_SEED)(a, b);
    let n = european_cup_compare_clubs_by_seed(a, b);
    PLACAR.record(r == n, || format!("european_cup_compare_clubs_by_seed: original={} rust={}", r, n));
    r
}

/// Comparer for `european_cup_rank_teams` (comparison mode: see the header of motor.rs).
///
/// [pt] Só a original (acrescenta registros à estatística), conferindo o plano feito antes (o vetor ordenado numa cópia e a
/// tabela do mata-mata pedida ao método +0x3c e liberada): a posição (+0x72 do registro achado por 0x4a4850) de cada
/// clube do plano.
pub(crate) unsafe extern "thiscall" fn comparar_european_cup_rank_teams(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_EUROPEAN_CUP_RANK_TEAMS);
    if read_u32_as_usize(this + 8) == 0 || read_u32_as_usize(this + 0x14) == 0 || read_i16(this + 0x36) <= 0 {
        return o(this);
    }
    let sort: unsafe extern "C" fn(*mut u32, i32) = core::mem::transmute(FN_COMP_UTIL_SORT_CLUBS_BY_STRENGTH);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let achar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_FIND_CLUB_ENTRY);
    let n = read_i16(this + 0x36) as usize;
    let mut v: Vec<u32> = (0..n).map(|i| read_u32_as_usize(read_u32_as_usize(this + 0x14) + i * 6) as u32).collect();
    sort(v.as_mut_ptr(), n as i32);
    let rounds_of_cup: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(this) + 0x3c));
    let (mut nm, mut id) = (0i16, 0i16);
    let knockout = rounds_of_cup(this, 0xc, &mut nm, &mut id, 0);
    let plan = if knockout != 0 { planned_final_positions(this, knockout, nm).0 } else { Vec::new() };
    soltar(knockout);
    o(this);
    if plan.iter().any(|&(k, _)| k as usize >= n) {
        return; // the original reads past the vector: no check
    }
    let errado = plan.iter().find(|&&(k, p)| {
        let c = v[k as usize] as usize;
        c != 0 && {
            let e = achar(read_u32_as_usize(this + 8), c);
            e == 0 || *((e + 0x72) as *const u8) != p
        }
    });
    PLACAR.record(errado.is_none(), || format!("european_cup_rank_teams: a posição {:?} não bate", errado));
}

/// Comparer for `draw_phase2_groups` (comparison mode: see the header of motor.rs).
///
/// [pt] Só a original (sorteia, paga prêmios, aloca as fases e manda notícias; e se chama de novo), conferindo o plano: na
/// chamada de fora (tentativa 0), as fases 8..11 existem, +0x30 = 11 e os clubes das tabelas delas (+0xb1, linhas de
/// 0x3b bytes) são os 16 classificados.
pub(crate) unsafe extern "thiscall" fn comparar_draw_phase2_groups(this: usize, attempt: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_DRAW_PHASE2_GROUPS);
    if attempt as u8 != 0 || read_u32_as_usize(read_u32_as_usize(G_COMP_OBJECT_TABLE) + read_u32_as_usize(KEY_CLUB_COMP_UEFA_CUP) * 4) == 0 {
        return o(this, attempt);
    }
    let plan = second_stage_clubs(this);
    o(this, attempt);
    let mut vistos = Vec::with_capacity(16);
    let mut ok = read_u32_as_usize(this + 0x30) == 11;
    for h in 8..12usize {
        let f = read_u32_as_usize(read_u32_as_usize(this + 0xc) + h * 4);
        if f == 0 || read_u32_as_usize(f + 0xb1) == 0 {
            ok = false;
            break;
        }
        for r in 0..4usize {
            vistos.push(read_u32_as_usize(read_u32_as_usize(f + 0xb1) + r * 0x3b));
        }
    }
    vistos.sort_unstable();
    ok = ok && vistos == plan;
    PLACAR.record(ok, || format!("draw_phase2_groups: as fases não têm os 16 classificados (+0x30 = {})", read_u32_as_usize(this + 0x30) as i32));
}

/// Comparer for `european_cup_compare_phase2_draw_entries` (comparison mode: see the header of motor.rs).
///
/// [pt] Pura: a devolução.
pub(crate) unsafe extern "C" fn comparar_european_cup_compare_phase2_draw_entries(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_EUROPEAN_CUP_COMPARE_PHASE2_DRAW_ENTRIES)(a, b);
    let n = european_cup_compare_phase2_draw_entries(a, b);
    PLACAR.record(r == n, || format!("european_cup_compare_phase2_draw_entries: original={} rust={}", r, n));
    r
}

/// Comparer for `european_cup_update_staff_records_for_status` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_european_cup_update_staff_records_for_status(this: usize, clube: usize, outcome: u32, stage: u32, a: u32, jogo: usize, end: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_EUROPEAN_CUP_UPDATE_STAFF_RECORDS_FOR_STATUS);
    let (d, f) = (outcome as u8, stage as u8);
    // the original’s bands compare the stage byte as signed (0x80 to 0xfe count as stage 1)
    let sai = if f == 0xff {
        d == 0xff || d == 2
    } else if (f as i8) < 0xc {
        d == 0xfe
    } else if f == 0xc {
        d == 1 || d == 2 || d == 0xff
    } else {
        false
    };
    let before = if clube != 0 { core::ptr::read_unaligned((clube + club::EURO_FLAG) as *const i32) } else { 0 };
    let r = o(this, clube, outcome, stage, a, jogo, end);
    let after = if clube != 0 { core::ptr::read_unaligned((clube + club::EURO_FLAG) as *const i32) } else { 0 };
    let same = r == 0 && (clube == 0 || after == if sai { -1 } else { before });
    PLACAR.record(same, || format!("european_cup_update_staff_records_for_status({:#x}, {:#x}, {}, {}) original={} +0x1db {} -> {} sai={}", this, clube, d, f, r, before, after, sai));
    r
}

/// Comparer for `draw_quarter_finals` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_draw_quarter_finals(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_DRAW_QUARTER_FINALS);
    let mut vivos: Vec<usize> = Vec::new();
    let mut away: Vec<usize> = Vec::new();
    for g in 0..4i8 {
        vivos.push(second_stage_group_club(this, g, 0));
        vivos.push(second_stage_group_club(this, g, 1));
        away.push(second_stage_group_club(this, g, 2));
        away.push(second_stage_group_club(this, g, 3));
    }
    o(this);
    let stage = read_usize(read_usize(this + 0xc) + 0x30);
    if stage == 0 {
        PLACAR.record(false, || format!("draw_quarter_finals({:#x}) sem fase", this));
        return;
    }
    let list = read_usize(stage + 0xa7);
    let mut found: Vec<usize> = (0..8).map(|k| read_usize(list + 6 * k)).collect();
    let mut expected = vivos.clone();
    found.sort_unstable();
    expected.sort_unstable();
    let saiu = away.iter().all(|&c| core::ptr::read_unaligned((c + 0x1db) as *const i32) == -1);
    let same = found == expected && saiu && read_i16(stage + 0x3e) == 8 && core::ptr::read_unaligned((this + 0x30) as *const i32) == 0xc;
    PLACAR.record(same, || format!("draw_quarter_finals({:#x}) esperado={:x?} lista={:x?} saiu={} +0x30={}", this, expected, found, saiu, core::ptr::read_unaligned((this + 0x30) as *const i32)));
}

/// Comparer for `european_cup_collect_club_list` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x586660: a original aloca a lista (só ela roda; quem chama libera). O plano, feito antes numa lista
/// local pelos mesmos inscritos: a conta (palavra de baixo da devolução) e os clubes da lista dela iguais. Nos tipos
/// que vão para a 0x48e1c0 e sem lista: a devolução e a saída como a original deixou.
pub(crate) unsafe extern "thiscall" fn comparar_european_cup_collect_club_list(this: usize, out: usize, kind: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_EUROPEAN_CUP_COLLECT_CLUB_LIST);
    let t = kind as u8;
    let sem_7 = match t {
        1 => Some(true),
        2 => Some(false),
        3 if core::ptr::read_unaligned((this + 0x30) as *const i32) == 0xb => Some(false),
        _ => None,
    };
    let mut plan = [0usize; 71];
    let n = match sem_7 {
        Some(s) if read_i16(this + 0x38) <= 71 => active_entrants(this, s, plan.as_mut_ptr()),
        _ => return o(this, out, kind),
    };
    let r = o(this, out, kind);
    let v = read_usize(out);
    if v == 0 {
        PLACAR.record(r as u16 == 0, || format!("european_cup_collect_club_list({:#x}, {}) sem lista original={:#x}", this, t, r));
        return r;
    }
    let list = core::slice::from_raw_parts(v as *const usize, (n.max(0) as usize).min(71));
    let same = r as u16 as i16 == n && list == &plan[..list.len()];
    PLACAR.record(same, || format!("european_cup_collect_club_list({:#x}, {}) original={:#x} rust={} lista={:x?} plano={:x?}", this, t, r, n, list, &plan[..list.len()]));
    r
}

/// Comparer for `european_cup_collect_entrants` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x584fa0: a original aloca (só ela roda); o plano confere depois tudo o que ela gravou: a lista nova
/// copiada e repreenchida pela cópia (fill_champions_league_entrants só lê os clubes, que a original não muda) tem de sair igual,
/// +0x36, +0x38 e a devolução.
pub(crate) unsafe extern "thiscall" fn comparar_european_cup_collect_entrants(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_EUROPEAN_CUP_COLLECT_ENTRANTS)(this);
    let t = read_usize(this + 0x14);
    if t == 0 {
        PLACAR.record(r == 0, || format!("european_cup_collect_entrants({:x}) sem lista original={}", this, r));
        return r;
    }
    let mut v = core::slice::from_raw_parts(t as *const u8, 0x47 * 6).to_vec();
    let total = fill_champions_league_entrants(v.as_mut_ptr() as usize);
    let n = if total == 0x47 { 1 } else { 0 };
    let same = r == n
        && v[..] == *core::slice::from_raw_parts(t as *const u8, 0x47 * 6)
        && read_i16(this + 0x36) == if n == 1 { 0x47 } else { 0 }
        && read_i16(this + 0x38) == 0x47;
    PLACAR.record(same, || format!("european_cup_collect_entrants({:x}) original={} +0x36={} rust={} total={}", this, r, read_i16(this + 0x36), n, total));
    r
}

/// Comparer for `build_qualifying_entry_table` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x584990: a original aloca (só ela roda); depois a tabela dela, copiada e remontada pela cópia,
/// tem de sair igual, com +0x3e = 55 e a devolução.
pub(crate) unsafe extern "thiscall" fn comparar_build_qualifying_entry_table(this: usize) -> i32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_BUILD_QUALIFYING_ENTRY_TABLE)(this);
    let t = read_usize(this + 0xa7);
    if t == 0 || read_i16(this + 0x3e) != 0x37 {
        PLACAR.record(r == -1 && t == 0, || format!("build_qualifying_entry_table({:x}) original={} tabela={:x} +0x3e={}", this, r, t, read_i16(this + 0x3e)));
        return r;
    }
    let mut v = core::slice::from_raw_parts(t as *const u8, 0x14a).to_vec();
    fill_qualifying_table(this, v.as_mut_ptr() as usize);
    let same = r == 1 && v[..] == *core::slice::from_raw_parts(t as *const u8, 0x14a);
    PLACAR.record(same, || format!("build_qualifying_entry_table({:x}) original={} tabela={:x?}", this, r, &v[..]));
    r
}

/// Comparer for `european_cup_advance_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_european_cup_advance_stage(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_EUROPEAN_CUP_ADVANCE_STAGE)(this)
}

/// Comparer for `process_fixture` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x583b10, em ensaio: a cópia sem a notícia e sem a base, e a original inteira, a partir do mesmo
/// estado (as palavras do coeficiente das duas nações e os registros de finanças dos dois clubes, restaurados depois
/// do ensaio). Confere as palavras do coeficiente, que a base não toca, salvo pelo sorteio das fases (0x585290,
/// 0x585ab0: o coeficiente dos classificados), que só roda quando a fase (+0x30) anda: aí, sem comparação. As
/// finanças não entram (a base paga o prêmio da rodada da copa nos mesmos campos, 0x51a2fe); o prêmio segue o ramo do
/// placar que os pontos do coeficiente já conferem. Sem clube, nação, seleção ou finanças: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_process_fixture(this: usize, jogo: usize, arg: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_PROCESS_FIXTURE);
    let (a, b) = if jogo != 0 { (read_usize(jogo + 0x1c), read_usize(jogo + 0x20)) } else { (0, 0) };
    if a == 0 || b == 0 {
        return o(this, jogo, arg); // simplification: without a club, original only
    }
    let (ca, cb) = (nation_coefficient_word(a), nation_coefficient_word(b));
    let (fa, fb) = (european_cup_club_finance(a), european_cup_club_finance(b));
    if ca == 0 || cb == 0 || fa == 0 || fb == 0 {
        return o(this, jogo, arg); // simplification: without nation, national team or finances, original only (crashes the same way)
    }
    let check = [(ca, 2), (cb, 2)];
    let before = crate::comparison::MemorySnapshot::read(&[(ca, 2), (cb, 2), (fa, 0x167), (fb, 0x167)]);
    let stage = read_i32(this + 0x30);
    EUROPEAN_CUP_FIXTURE_DRY_RUN = true;
    crate::rng::pause_trace(|| crate::rng::without_counting(|| process_fixture(this, jogo, arg)));
    EUROPEAN_CUP_FIXTURE_DRY_RUN = false;
    let mr = crate::comparison::MemorySnapshot::read(&check);
    before.write_back();
    o(this, jogo, arg);
    if read_i32(this + 0x30) != stage {
        return; // simplification: the stage advanced in the database and the draw added coefficient: no comparison
    }
    let mo = crate::comparison::MemorySnapshot::read(&check);
    PLACAR.record(mo == mr, || format!("process_fixture(jogo {:x}) original={:x?} rust={:x?}", jogo, mo, mr));
}

/// Comparer for `european_cup_set_club_progress` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x586c00: lado a lado nos três bytes do registro do clube (+0x73..+0x75); sem registro (a original
/// cai), só a original.
pub(crate) unsafe extern "thiscall" fn comparar_european_cup_set_club_progress(this: usize, clube: usize, stage: u32, p1: u32, p2: u32, p3: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(IDX_EUROPEAN_CUP_SET_CLUB_PROGRESS);
    let search: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_FIND_CLUB_ENTRY);
    let e = search(read_usize(this + 8), clube);
    if e == 0 {
        return o(this, clube, stage, p1, p2, p3); // simplification: without a record, original only
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(
        &[(e + 0x73, 3)],
        || o(this, clube, stage, p1, p2, p3),
        || european_cup_set_club_progress(this, clube, stage, p1, p2, p3),
    );
    PLACAR.record(igual, || format!("european_cup_set_club_progress(fase {} {} {} {}) original={:x?} rust={:x?}", stage as u8 as i8, p1 as u8, p2 as u8, p3 as u8, mo, mr));
}

/// Comparer for `european_cup_create_round_table` (comparison mode: see the header of motor.rs).
///
/// [pt] Lado a lado nas fases de grupos: a original com as saídas de quem chama, a cópia com saídas próprias, a partir do
/// mesmo [0xb67a34]; as saídas e os bytes gravados das duas tabelas têm de ser iguais (a da cópia é liberada). As
/// preliminares e o mata-mata: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_european_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, id: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_EUROPEAN_CUP_CREATE_ROUND_TABLE);
    let f = stage as u8 as i8;
    if f == -1 || f >= 0xc {
        return o(this, stage, rounds, id, league);
    }
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let (mut r2, mut i2, mut l2) = (0i16, 0i16, 0i32);
    let reg = [(0xb67a34usize, 4usize)];
    let (a, b, igual, _, _) = crate::comparison::side_by_side(
        &reg,
        || o(this, stage, rounds, id, league),
        || european_cup_create_round_table(this, stage, &mut r2, &mut i2, if league.is_null() { core::ptr::null_mut() } else { &mut l2 }),
    );
    let outs = *rounds == r2 && *id == i2 && (league.is_null() || *league == l2);
    let table = if a != 0 && b != 0 { league_written_table_bytes(a) == league_written_table_bytes(b) } else { a == 0 && b == 0 };
    if b != 0 {
        soltar(b);
    }
    PLACAR.record(igual && outs && table, || format!("european_cup_create_round_table({}): saídas {} tabela {}", f, outs, table));
    a
}
