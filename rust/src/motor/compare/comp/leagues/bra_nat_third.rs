use super::*;

// The comparers of the routines in ../comp/leagues/bra_nat_third.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/leagues/bra_nat_third.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `bra_nat_third_create_final_stage` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador da final: só a original (aloca a tabela e a fase e sorteia a rodada), com o plano feito antes: os
/// campeões dos grupos da terceira fase. Depois, a fase da vaga 10 existe, com +0x3e = 2, os dois na lista dela (como
/// conjunto) e +0x30 = 10.
pub(crate) unsafe extern "thiscall" fn compare_bra_nat_third_create_final_stage(this: usize) {
    let (a, b) = (stage_leader(this, 8), stage_leader(this, 9));
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_NAT_THIRD_CREATE_FINAL_STAGE)(this);
    let stage = read_usize(read_usize(this + 0xc) + 0x28);
    let same = stage != 0 && {
        let l = read_usize(stage + 0xa7);
        let (x, y) = (read_usize(l), read_usize(l + 6));
        read_i16(stage + 0x3e) == 2 && read_usize(this + 0x30) == 10 && ((x, y) == (a, b) || (x, y) == (b, a))
    };
    SCOREBOARD.record(same, || format!("bra_nat_third_create_final_stage({:#x}) campeoes={:#x},{:#x} fase={:#x}", this, a, b, stage));
}

/// Comparer for `bra_nat_third_start_new_season` (comparison mode: see the header of motor.rs).
///
/// Only the original runs (simplification: it frees tables and stages, allocates the new ones, draws fixtures,
/// officials and groups; it cannot run twice), checking what only it guarantees: the year + 1, the seven groups in
/// slots 0 to 6 and the list of 56 zeroed.
///
/// [pt] O comparador da virada: só a original (simplification: libera tabelas e fases, aloca as novas, sorteia jogos, árbitros e
/// grupos; não pode rodar duas vezes), conferindo o que só ela garante: o ano + 1, os sete grupos nas vagas 0 a 6 e a
/// lista dos 56 zerada.
pub(crate) unsafe extern "thiscall" fn compare_bra_nat_third_start_new_season(this: usize) -> u32 {
    let before = read_i16(this + 0x40);
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_NAT_THIRD_START_NEW_SEASON)(this);
    let after = read_i16(this + 0x40);
    let same = after == before.wrapping_add(1) && read_usize(this + 0xee) == 0 && (0..7).all(|g| read_usize(read_usize(this + 0xc) + 4 * g) != 0);
    SCOREBOARD.record(same, || format!("bra_nat_third_start_new_season({:#x}) ano antes={} depois={} +0xee={:#x}", this, before, after, read_usize(this + 0xee)));
    r
}

/// Comparer for `bra_nat_third_set_predicted_positions` (comparison mode: see the header of motor.rs).
///
/// The predicted positions comparer: only the original routine runs (0x4a2540 adds the record of a club that has none;
/// it cannot run twice), with the plan made beforehand: the list of the 56 sorted by the same qsort on a copy.
///
/// Afterwards, the league's list in the same order and the position (+0x72 of the record found by 0x4a4850) of each
/// club.
///
/// [pt] O comparador das posições previstas: só a original (a 0x4a2540 acrescenta o registro de um clube que não tem; não
/// pode rodar duas vezes), com o plano feito antes: a lista dos 56 ordenada pela mesma qsort numa cópia; depois, a
/// lista da liga na mesma ordem e a posição (+0x72 do registro achado por 0x4a4850) de cada clube.
pub(crate) unsafe extern "thiscall" fn compare_bra_nat_third_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_NAT_THIRD_SET_PREDICTED_POSITIONS);
    if read_usize(this + 8) == 0 || read_usize(this + 0xee) == 0 {
        return o(this);
    }
    let sort: unsafe extern "C" fn(*mut usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let achar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_FIND_CLUB_ENTRY);
    let mut v = [0usize; 0x38];
    for (k, c) in v.iter_mut().enumerate() {
        *c = read_usize(read_usize(this + 0xee) + 4 * k);
    }
    sort(v.as_mut_ptr(), 0x38, 4, 0x4b07d0);
    o(this);
    let errado = (0..0x38).find(|&k| {
        let e = if v[k] != 0 { achar(read_usize(this + 8), v[k]) } else { 0 };
        read_usize(read_usize(this + 0xee) + 4 * k) != v[k] || e == 0 || *((e + 0x72) as *const u8) != place_for_index(k)
    });
    SCOREBOARD.record(errado.is_none(), || format!("bra_nat_third_set_predicted_positions({:#x}): a vaga {:?} não bate", this, errado));
}

/// Comparer for `bra_nat_third_create_group_stage` (comparison mode: see the header of motor.rs): the comparer of one
/// group; only the original routine runs (it allocates the vector, the round table and the stage), with the plan made
/// beforehand.
///
/// The plan: the seven clubs of slots 7(g + 1).. of the list of 56. Afterwards: the stage in slot g with `+0x3e` = 7,
/// the seven in its table (as a set) and `+0x30` = g.
///
/// [pt] O comparador de um grupo: só a original (aloca o vetor, a tabela das rodadas e a fase), com o plano feito antes: os
/// sete clubes das vagas 7(g + 1).. da lista dos 56. Depois, a fase na vaga g com +0x3e = 7, os sete na tabela dela
/// (como conjunto) e +0x30 = g.
pub(crate) unsafe extern "thiscall" fn compare_bra_nat_third_create_group_stage(this: usize, g: u32) {
    let k = g as u8 as i8 as i32;
    let mut plan = [0usize; 7];
    for (j, c) in plan.iter_mut().enumerate() {
        *c = read_usize(read_usize(this + 0xee) + 4 * (7 * (k + 1) as usize + j));
    }
    original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_BRA_NAT_THIRD_CREATE_GROUP_STAGE)(this, g);
    let stage = read_usize(read_usize(this + 0xc) + 4 * k as usize);
    let same = stage != 0 && read_i16(stage + 0x3e) == 7 && read_usize(this + 0x30) as i32 == k && {
        let mut nela = [0usize; 7];
        for (j, c) in nela.iter_mut().enumerate() {
            *c = read_usize(read_usize(stage + 0xb1) + j * 0x3b);
        }
        let mut a = plan;
        a.sort_unstable();
        nela.sort_unstable();
        a == nela
    };
    SCOREBOARD.record(same, || format!("bra_nat_third_create_group_stage({:#x}, {}) fase={:#x} plano={:x?}", this, k, stage, plan));
}

/// Comparer for `create_second_round_stage` (comparison mode: see the header of motor.rs).
///
/// The second-round comparer: only the original routine runs (it allocates the list, the table and the stage and draws
/// the round), with the plan made beforehand by the same selection on a copy of the records.
///
/// The stage in slot 7 exists, with +0x3e = 16 and the sixteen in slot order in its list (+0xa7, 6 bytes), and +0x30 =
/// 7.
///
/// [pt] O comparador da segunda fase: só a original (aloca a lista, a tabela e a fase e sorteia a rodada), com o plano feito
/// antes pela mesma escolha numa cópia dos registros: a fase da vaga 7 existe, com +0x3e = 16 e os dezesseis na ordem
/// das vagas na lista dela (+0xa7, 6 bytes), e +0x30 = 7.
pub(crate) unsafe extern "thiscall" fn compare_create_second_round_stage(this: usize) {
    let mut r = [(0usize, 0i16); 16];
    for (k, g) in (-1i32..7).enumerate() {
        let t = read_usize(if g == -1 { this } else { read_usize(read_usize(this + 0xc) + 4 * g as usize) } + 0xb1);
        r[2 * k] = (read_usize(t), read_i16(t + 0x19));
        r[2 * k + 1] = (read_usize(t + 0x3b), read_i16(t + 0x54));
    }
    let mut plan = [0usize; 16];
    for k in 0..16 {
        let mut best: Option<usize> = None;
        for (i, e) in r.iter().enumerate() {
            if best.map_or(true, |m| e.1 > r[m].1) && e.0 != 0 {
                best = Some(i);
            }
        }
        if let Some(m) = best {
            plan[SECOND_ROUND_SEEDS[k] as usize] = r[m].0;
            r[m].0 = 0;
        }
    }
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CREATE_SECOND_ROUND_STAGE)(this);
    let stage = read_usize(read_usize(this + 0xc) + 0x1c);
    let same = stage != 0 && read_i16(stage + 0x3e) == 0x10 && read_usize(this + 0x30) == 7 && {
        let mut nela = [0usize; 16];
        for (j, c) in nela.iter_mut().enumerate() {
            *c = read_usize(read_usize(stage + 0xa7) + 6 * j);
        }
        let (mut a, mut b) = (plan, nela);
        a.sort_unstable();
        b.sort_unstable();
        a == b
    };
    SCOREBOARD.record(same, || format!("create_second_round_stage({:#x}) fase={:#x} plano={:x?}", this, stage, plan));
}

/// Comparer for `create_third_round_groups` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador da terceira fase: só a original (embaralha: sorteia; aloca as tabelas e as fases), com o plano: os
/// vencedores da segunda fase; depois, as fases 8 e 9 com 4 clubes cada, os oito juntos (como conjunto) e +0x30 = 9.
pub(crate) unsafe extern "thiscall" fn compare_create_third_round_groups(this: usize) {
    let second = read_usize(read_usize(this + 0xc) + 0x1c);
    let mut plan = [0usize; 8];
    let mut n = 0usize;
    let mut i = 0i32;
    while i < read_i16(second + 0x3e) as i32 && n < 8 {
        let e = read_usize(second + 0xa7) + 6 * i as usize;
        if read_u8(e + 5) == 1 {
            plan[n] = read_usize(e);
            n += 1;
        }
        i += 1;
    }
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CREATE_THIRD_ROUND_GROUPS)(this);
    let (a, b) = (read_usize(read_usize(this + 0xc) + 0x20), read_usize(read_usize(this + 0xc) + 0x24));
    let same = a != 0 && b != 0 && read_i16(a + 0x3e) == 4 && read_i16(b + 0x3e) == 4 && read_usize(this + 0x30) == 9 && {
        let mut nela = [0usize; 8];
        for j in 0..4 {
            nela[j] = read_usize(read_usize(a + 0xb1) + j * 0x3b);
            nela[4 + j] = read_usize(read_usize(b + 0xb1) + j * 0x3b);
        }
        let mut p = plan;
        p.sort_unstable();
        nela.sort_unstable();
        p == nela
    };
    SCOREBOARD.record(same, || format!("create_third_round_groups({:#x}) n={} plano={:x?}", this, n, plan));
}

/// Comparer for `bra_nat_third_create_table` (the Série C; comparison mode: see the header of motor.rs).
///
/// The original frees, allocates and draws: only it runs. The plan, made beforehand: the clubs whose division is
/// [this+4], in database order, up to 56. Afterwards: return value 1, +0x3e = 7, the list (+0xee) with the plan's clubs
/// in some order (the random draw) and table rows 0..6 with the first 7 of the list.
///
/// [pt] O comparador da Série C: a original libera e aloca e sorteia: só ela. O plano, feito antes: os clubes cuja divisão é
/// [this+4], na ordem do banco, até 56. Depois: a devolução 1, +0x3e = 7, a lista (+0xee) com os clubes do plano em
/// alguma ordem (o sorteio) e as linhas 0..6 da tabela com os 7 primeiros da lista.
pub(crate) unsafe extern "thiscall" fn compare_bra_nat_third_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_NAT_THIRD_CREATE_TABLE);
    let mut plan = Vec::new();
    let mut i: i32 = 0;
    while i < read_usize(G_CLUB_COUNT) as i32 && plan.len() < 0x38 {
        let club = read_usize(G_CLUB_TABLE) + (i as usize) * club::SIZE;
        if read_usize(club + club::DIVISION) == read_usize(this + 4) {
            plan.push(club);
        }
        i += 1;
    }
    let r = o(this);
    let (lista, t) = (read_usize(this + 0xee), read_usize(this + 0xb1));
    let mut same = r == 1 && read_i16(this + 0x3e) == 7 && lista != 0 && t != 0 && plan.len() >= 7;
    let mut drawn: Vec<usize> = Vec::new();
    if same {
        drawn = (0..plan.len()).map(|k| read_usize(lista + k * 4)).collect();
        let rows_ok = (0..7usize).all(|j| read_usize(t + j * 0x3b) == drawn[j]);
        let (mut a, mut b) = (drawn.clone(), plan.clone());
        a.sort_unstable();
        b.sort_unstable();
        same = rows_ok && a == b;
    }
    SCOREBOARD.record(same, || format!("bra_nat_third_create_table({:#x}) original={} plano={:x?} lista={:x?}", this, r, plan, drawn));
    r
}

/// Comparer for `bra_nat_third_set_up_rules` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: como o da Série A.
pub(crate) unsafe extern "thiscall" fn compare_bra_nat_third_set_up_rules(this: usize) -> u32 {
    let reg = league_regions_without_schedule(this);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_NAT_THIRD_SET_UP_RULES)(this), || {
        set_serie_c_rules(this);
        1
    });
    SCOREBOARD.record(igual && r == n, || format!("bra_nat_third_set_up_rules({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}

/// Comparer for `bra_nat_third_update_staff_records_for_status` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: como o da Série A.
pub(crate) unsafe extern "thiscall" fn compare_bra_nat_third_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    let (n, l, dest) = serie_c_staff_records(this, club, status, stage, false);
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> u32>(IDX_BRA_NAT_THIRD_UPDATE_STAFF_RECORDS_FOR_STATUS)(this, club, status, stage, a4, a5, a6);
    let gravado = if l == 0 { dest } else { *((l + 0x37) as *const u8) };
    SCOREBOARD.record(r == n && gravado == dest, || format!("bra_nat_third_update_staff_records_for_status({:x}, {:x}, {}, {}) original={} rust={} destino {} x {}", this, club, status as u8 as i8, stage as u8 as i8, r as i32, n as i32, gravado as i8, dest as i8));
    r
}

/// Comparador: como o do Mineiro.
pub(crate) unsafe extern "thiscall" fn compare_bra_nat_third_set_club_progress(this: usize, club: usize, stage: u32, a: u32, b: u32, c: u32) -> usize {
    let e = bra_reg_minas_gerais_comp_stats_club_entry(this, club);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&[(e + 0x73, 3)], || original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> usize>(IDX_BRA_NAT_THIRD_SET_CLUB_PROGRESS)(this, club, stage, a, b, c), || bra_nat_third_set_club_progress(this, club, stage, a, b, c));
    SCOREBOARD.record(igual && r == n, || format!("bra_nat_third_set_club_progress({:x}, {:x}, {}, {}, {}, {}) original={:x} rust={:x} {:?} {:?}", this, club, stage as u8 as i8, a as u8, b as u8, c as u8, r, n, mo, mr));
    r
}
