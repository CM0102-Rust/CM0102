use super::*;

// The comparers of the routines in ../discipline.rs.
// [pt] Os comparadores das rotinas de ../discipline.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `serve_international_bans`.
///
/// The count (0x556e90) writes the history to the file when the suspension ends and cannot run twice: the plan first
/// (the nation's persons and, for each one, what 0x556e90 will do, serve_ban_match_plan), then the original routine
/// runs.
///
/// It checks, person by person, the new counter or the released suspension (free: +0 = 0xff, +2 = −1).
///
/// [pt] Comparador: a contagem (0x556e90) grava o histórico no arquivo quando a suspensão termina e não pode rodar duas
/// vezes: o plano antes (as pessoas da nação e, de cada uma, o que a 0x556e90 fará, serve_ban_match_plan), a original roda;
/// confere, pessoa a pessoa, o contador novo ou a suspensão solta (livre: +0 = 0xff, +2 = −1).
pub(crate) unsafe extern "thiscall" fn compare_serve_international_bans(this: usize, team: usize) {
    let (ini, fim) = nation_index_range(this, team);
    let mut plan = Vec::new();
    let mut e = ini;
    while ini != 0 && e < fim {
        let id = read_usize(e) as u32;
        let j = serve_ban_match_plan(this, id, 0, 1);
        let before = if j.suspensao != 0 { *((j.suspensao + 1) as *const u8) } else { 0 };
        plan.push((id, j, before));
        e += 8;
    }
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_SERVE_INTERNATIONAL_BANS)(this, team);
    let mut error = None;
    for (id, j, before) in &plan {
        if j.suspensao == 0 {
            continue;
        }
        let s = j.suspensao;
        let (jogos, contador, dias) = (*(s as *const u8), *((s + ban_record::MATCHES_SERVED) as *const u8), read_i16(s + ban_record::DAYS));
        let ok = if j.cumprida { jogos == 0xff && dias == -1 } else { contador == j.counter.unwrap_or(*before) };
        if !ok && error.is_none() {
            error = Some(format!("pessoa {} contador {} (esperava {:?}, antes {}), cumprida={} suspensão {:#x}/{}", id, contador, j.counter, before, j.cumprida, jogos, dias));
        }
    }
    SCOREBOARD.record(error.is_none(), || format!("serve_international_bans(time {:#x}, {} pessoas): {}", team, plan.len(), error.clone().unwrap_or_default()));
}

/// Comparer for `apply_appeal_result`.
///
/// [pt] Comparador: o órgão tira as suspensões pendentes (0x7e91d0, lista que solta nós) e a 0x556d00 pode gravar o
/// histórico: só a original, com o plano antes; confere a suspensão solta (+0 = 0xff, +2 = −1) ou os jogos e os dias
/// novos.
pub(crate) unsafe extern "thiscall" fn compare_apply_appeal_result(this: usize, id: u32, rule: u32, reduces: u32, minus_games: u32, minus_days: u32, new: u32, days_fixed: u32, x: u32) {
    let r = appeal_plan(this, id, rule, reduces, minus_games, minus_days, new, days_fixed);
    original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32)>(IDX_APPLY_APPEAL_RESULT)(this, id, rule, reduces, minus_games, minus_days, new, days_fixed, x);
    if r.suspensao == 0 {
        return;
    }
    let s = r.suspensao;
    let (jogos, dias) = (*(s as *const u8), read_i16(s + ban_record::DAYS));
    let ok = if r.solta { jogos == 0xff && dias == -1 } else { jogos == r.games && dias == r.days };
    SCOREBOARD.record(ok, || format!("apply_appeal_result({}, regra {}) jogos {:#x} dias {} (esperava {:#x}/{}, solta={})", id, rule as i8, jogos, dias, r.games, r.days, r.solta));
}

/// Comparer for `get_slot_text`.
///
/// [pt] Comparador: sem regra na vaga (ou pessoa fora da tabela), a devolução (0, sem gravar nada); com regra, o texto do órgão
/// (0x7e88d0) pode contar a suspensão por dias (0x556e90, que grava o histórico no arquivo): só a original.
pub(crate) unsafe extern "thiscall" fn compare_get_slot_text(this: usize, person: usize, slot: u32, out: usize, size: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize, u32) -> u32>(IDX_GET_SLOT_TEXT);
    if person == 0 || (read_usize(person) as i32) < read_usize(G_STAFF_COUNT) as i32 && ban_slot_rule(this, person, slot) != 0xff {
        // simplification: the governing body’s text does not run twice; the copy only passes it on
        return o(this, person, slot, out, size);
    }
    let r = o(this, person, slot, out, size);
    let n = get_slot_text(this, person, slot, out, size);
    SCOREBOARD.record(r == n, || format!("get_slot_text(pessoa {:#x}, vaga {}) original={} rust={}", person, slot as i8, r, n));
    r
}

/// Comparer for `add_ban` (comparison mode: see the header of motor.rs): `0x556940` reallocates the vectors and links
/// the bodies (it cannot run twice).
///
/// The plan first, then the original routine runs; it checks the old ban extended (matches, days, bit 0 of both), or
/// the new one in the slot (matches, days, the 4 rules), the count, the rule entries pointing to the slot and, with the
/// fourth rule, the nations index with one more.
///
/// [pt] Comparador: a 0x556940 realoca os vetores e liga os órgãos (não pode rodar duas vezes): o plano antes, a original
/// roda; confere a velha estendida (jogos, dias, bit 0 das duas), ou a nova na vaga (jogos, dias, as 4 regras), a
/// conta, as entradas das regras apontando para a vaga e, com a quarta regra, o índice das nações com um a mais.
pub(crate) unsafe extern "thiscall" fn compare_add_ban(this: usize, id: u32, ban: usize) {
    let plan = add_ban_plan(this, id, ban);
    let index = read_i16(this + discipline_fields::NATION_INDEX_COUNT);
    let mut rules = [0u8; 4];
    core::ptr::copy_nonoverlapping((ban + ban_record::RULES) as *const u8, rules.as_mut_ptr(), 4);
    original_fn::<unsafe extern "thiscall" fn(usize, u32, usize)>(IDX_ADD_BAN)(this, id, ban);
    let (ok, detalhe) = match plan {
        SuspensionPlan::Caixa => return,
        SuspensionPlan::Estender { s, games: jogos, days: dias } => {
            let ok = *(s as *const u8) == jogos && read_i16(s + ban_record::DAYS) == dias && *((s + ban_record::FLAGS) as *const u8) & 1 == 1
                && *((ban + ban_record::FLAGS) as *const u8) & 1 == 1;
            (ok, format!("estender: jogos {} (esperava {}), dias {} (esperava {})", *(s as *const u8), jogos, read_i16(s + ban_record::DAYS), dias))
        }
        SuspensionPlan::Nova { slot: vaga, count: conta } => {
            let s = read_usize(this + discipline_fields::BANS).wrapping_add((vaga as i32 as usize).wrapping_mul(20));
            let mut ok = read_i16(this + discipline_fields::BAN_COUNT) == conta && *(s as *const u8) == *(ban as *const u8) && read_i16(s + ban_record::DAYS) == read_i16(ban + ban_record::DAYS);
            for k in 0..4usize {
                ok &= *((s + ban_record::RULES + k) as *const u8) == rules[k];
                if rules[k] != 0xff {
                    let e = ruling_body_entry(ruling_body_of_rule(this, rules[k]), id);
                    ok &= e != 0 && read_i16(e + ruling_body_entry::BAN_INDEX) == vaga;
                }
            }
            if rules[3] != 0xff {
                ok &= read_i16(this + discipline_fields::NATION_INDEX_COUNT) == index.wrapping_add(1);
            }
            (ok, format!("nova: vaga {} conta {} (esperava {}), índice {} (antes {})", vaga, read_i16(this + discipline_fields::BAN_COUNT), conta, read_i16(this + discipline_fields::NATION_INDEX_COUNT), index))
        }
    };
    SCOREBOARD.record(ok, || format!("add_ban({}) {}", id, detalhe));
}

/// Comparer for `clear_bans_invalid_for_club`.
///
/// 0x556270 releases bans (nation index, governing bodies): the plan beforehand (the slots the club does not cover and
/// the entries), then the original runs; it checks the freed slots (0xff, −1, entry −1) and that the others are intact.
/// An entry repeated in two slots is not checked for the second one.
///
/// [pt] Comparador: a 0x556270 solta suspensões (índice das nações, órgãos): o plano antes (as vagas que o clube não cobre
/// e as entradas), a original roda; confere as vagas liberadas (0xff, −1, a entrada −1) e as outras intactas. Uma
/// entrada repetida em duas vagas não entra na conferência da segunda.
pub(crate) unsafe extern "thiscall" fn compare_clear_bans_invalid_for_club(this: usize, person: usize, nada: u32, club: usize) {
    let id = read_usize(person) as u32;
    let r = discipline_record(this, id);
    let mut before = [0u8; 29];
    core::ptr::copy_nonoverlapping(r as *const u8, before.as_mut_ptr(), 29);
    let mut plan: [Option<usize>; 4] = [None; 4];
    let mut duvida = [false; 4];
    for k in 0..4usize {
        let rule = before[k * 5];
        if rule == 0xff {
            continue;
        }
        let body = ruling_body_of_rule(this, rule);
        if !ruling_body_covers_club(body, club) {
            let e = ruling_body_entry(body, id);
            if e != 0 {
                duvida[k] = plan[..k].iter().any(|x| *x == Some(e));
                plan[k] = Some(e);
            }
        }
    }
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(IDX_CLEAR_BANS_INVALID_FOR_CLUB)(this, person, nada, club);
    let mut after = [0u8; 29];
    core::ptr::copy_nonoverlapping(r as *const u8, after.as_mut_ptr(), 29);
    let mut ok = after[20..] == before[20..];
    for k in 0..4usize {
        if duvida[k] {
            continue;
        }
        let v = &after[k * 5..k * 5 + 5];
        ok &= match plan[k] {
            Some(e) => v == [0xff; 5] && read_usize(e) as u32 == 0xffff_ffff,
            None => *v == before[k * 5..k * 5 + 5],
        };
    }
    SCOREBOARD.record(ok, || format!("clear_bans_invalid_for_club({}) plano {:x?} antes {:02x?} depois {:02x?}", id, plan, before, after));
}

/// Comparer for `restore_held_bans`.
///
/// 0x556570 removes and frees items from the list (lesson 1: the expected result is built BEFORE; no item is read
/// afterwards) and releases suspensions: the plan first, then the original routine runs.
///
/// It checks [0xae1080], the restored slots (the 5 stored bytes), the released ones (0xff, −1, entry −1), the count of
/// held ones and that the other slots are intact.
///
/// [pt] Comparador: a 0x556570 tira e libera itens da lista (lição 1: o esperado é montado ANTES; nenhum item é lido depois)
/// e solta suspensões: o plano antes, a original roda; confere [0xae1080], as vagas devolvidas (os 5 bytes guardados), as
/// soltas (0xff, −1, a entrada −1), a conta de guardadas e as outras vagas intactas.
pub(crate) unsafe extern "thiscall" fn compare_restore_held_bans(this: usize, person: usize) {
    let id = read_usize(person) as u32;
    let r = discipline_record(this, id);
    let p = return_bans_plan(this, person);
    let mut before = [0u8; 29];
    core::ptr::copy_nonoverlapping(r as *const u8, before.as_mut_ptr(), 29);
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_RESTORE_HELD_BANS)(this, person);
    let mut after = [0u8; 29];
    core::ptr::copy_nonoverlapping(r as *const u8, after.as_mut_ptr(), 29);
    let mut expected = before;
    for (k, b) in &p.guardadas {
        expected[k * 5..k * 5 + 5].copy_from_slice(b);
    }
    let mut ok = read_usize(G_CONTRACT_LOOKUP_RESULT) == p.contract;
    for (k, e) in &p.soltas {
        expected[k * 5..k * 5 + 5].copy_from_slice(&[0xff; 5]);
        ok &= read_usize(*e) as u32 == 0xffff_ffff;
    }
    expected[0x1c] = p.rest;
    ok &= after == expected;
    SCOREBOARD.record(ok, || format!("restore_held_bans({}) clube {:x} esperado {:02x?} depois {:02x?}", id, p.club, expected, after));
}

/// Comparer for `update_ban_progress`.
///
/// [pt] Comparador: a 0x58e340 grava o histórico no arquivo (não pode rodar duas vezes): o plano antes, a original roda;
/// confere o contador, ou a suspensão solta (livre e a entrada do órgão sem índice), ou nada mudado.
pub(crate) unsafe extern "thiscall" fn compare_update_ban_progress(this: usize, id: u32, rule: u32, count: u32) {
    let j = serve_ban_match_plan(this, id, rule, count);
    let before = if j.suspensao != 0 { *((j.suspensao + 1) as *const u8) } else { 0 };
    original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32)>(IDX_UPDATE_BAN_PROGRESS)(this, id, rule, count);
    if j.suspensao == 0 {
        return;
    }
    let s = j.suspensao;
    let (jogos, contador, dias) = (*(s as *const u8), *((s + ban_record::MATCHES_SERVED) as *const u8), read_i16(s + ban_record::DAYS));
    let ok = if j.cumprida {
        jogos == 0xff && dias == -1 && read_i16(j.entry + ruling_body_entry::BAN_INDEX) == -1
    } else {
        contador == j.counter.unwrap_or(before)
    };
    SCOREBOARD.record(ok, || {
        format!("update_ban_progress({}, {}, {}) contador {} (esperava {:?}, antes {}), cumprida={} suspensão {:#x}/{} entrada {}",
            id, rule as u8, count, contador, j.counter, before, j.cumprida, jogos, dias, read_i16(j.entry + ruling_body_entry::BAN_INDEX))
    });
}

/// Comparer for `release_ban`.
///
/// [pt] Comparador: lado a lado pela suspensão, o índice das nações (conta e itens) e as entradas da pessoa nos órgãos das
/// regras (achadas antes, sem criar). Quarta regra sem o item no índice: só a original (caixa).
pub(crate) unsafe extern "thiscall" fn compare_release_ban(this: usize, id: u32, index: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> usize>(IDX_RELEASE_BAN);
    let s = read_usize(this + discipline_fields::BANS).wrapping_add((index as u16 as i16 as i32 as usize).wrapping_mul(20));
    if *((s + ban_record::NATION_RULE) as *const u8) != 0xff && find_in_nation_index(this, id) == 0 {
        return o(this, id, index);
    }
    let mut reg = vec![(s, 20), (this + discipline_fields::NATION_INDEX_CAPACITY, 4)];
    let n = read_i16(this + discipline_fields::NATION_INDEX_COUNT);
    if n > 0 {
        reg.push((read_usize(this + discipline_fields::NATION_INDEX), n as usize * 8));
    }
    for k in 0..4usize {
        let rule = *((s + ban_record::RULES + k) as *const u8);
        if rule != 0xff {
            let e = ruling_body_entry(ruling_body_for_rule(this, rule), id);
            if e != 0 {
                reg.push((e, 14));
            }
        }
    }
    let (r, m, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, id, index), || release_ban(this, id, index));
    SCOREBOARD.record(igual && r == m, || format!("release_ban({}, {}) {:x?} x {:x?}", id, index as u16 as i16, mo, mr));
    r
}

/// Comparer for `clear_ban_history`.
///
/// [pt] Comparador: lado a lado pelo começo e o último deslocamento do histórico (8 bytes; a 0x58f110 só grava o segundo).
pub(crate) unsafe extern "thiscall" fn compare_clear_ban_history(this: usize, person: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_CLEAR_BAN_HISTORY);
    if person == 0 || read_usize(person + staff::PLAYER) == 0 {
        return o(this, person);
    }
    let r = person_discipline_record(this, read_usize(person));
    let (a, b, igual, mo, mr) = crate::comparison::side_by_side_light(&[(r + discipline_record::HISTORY_START, 8)], || o(this, person), || clear_ban_history(this, person));
    SCOREBOARD.record(igual && a == b, || format!("clear_ban_history({}) {:x?} x {:x?}", read_usize(person), mo, mr));
    a
}

/// Comparer for `discipline_operator`.
pub(crate) unsafe extern "thiscall" fn compare_discipline_operator(this: usize, source: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_DISCIPLINE_OPERATOR);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(this, 0x14)], || o(this, source), || discipline_operator(this, source));
    SCOREBOARD.record(igual && r == n, || format!("discipline_operator {:x?} x {:x?}", mo, mr));
    r
}

/// Comparer for `discipline_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_discipline_new_season(this: usize, id: *const i32) {
    original_fn::<unsafe extern "thiscall" fn(usize, *const i32)>(IDX_DISCIPLINE_NEW_SEASON)(this, id)
}

/// Comparer for `discipline_daily_update`.
pub(crate) unsafe extern "thiscall" fn compare_discipline_daily_update(this: usize, today: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_DISCIPLINE_DAILY_UPDATE)(this, today)
}

/// Comparer for `discipline_set_slot`.
pub(crate) unsafe extern "thiscall" fn compare_discipline_set_slot(this: usize, id: i32, slot: i8, kind: u8, value: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, i8, u8, u32)>(IDX_DISCIPLINE_SET_SLOT);
    let a = (*((this + discipline_fields::PERSON_RECORDS) as *const usize) as isize + id as isize * 29 + slot as isize * 5) as usize;
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(a, 5)], || o(this, id, slot, kind, value), || discipline_set_slot(this, id, slot, kind, value));
    SCOREBOARD.record(igual, || format!("discipline_set_slot({}, {}) {:x?} x {:x?}", id, slot, mo, mr));
}

/// Comparer for `is_player_suspended`.
///
/// [pt] Comparador: lado a lado pelas 20 vagas (+0x1e0) do clube do jogador (a 0x7e85b0 e a 0x600a50 só leem).
pub(crate) unsafe extern "thiscall" fn compare_is_player_suspended(this: usize, player: usize, comp: *const i32, extra: u32, partida: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *const i32, u32, usize) -> u32>(IDX_IS_PLAYER_SUSPENDED);
    let club = if player != 0 { read_usize(player + staff::CLUB_JOB) } else { 0 };
    let regions: &[(usize, usize)] = if club != 0 { &[(club + club::TEAM_SELECTED, 80)] } else { &[] };
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(regions, || o(this, player, comp, extra, partida), || is_player_suspended(this, player, comp, extra, partida));
    SCOREBOARD.record(igual && r == n, || format!("is_player_suspended({:x}, {}) original={} rust={} {:x?} x {:x?}", player, *comp, r, n, mo, mr));
    r
}

/// Comparer for `drop_held_bans`.
///
/// [pt] Comparador: o plano da cópia (os itens do id que saem até o contador zerar) antes; a original tira; confere os
/// itens que ficaram e o contador.
pub(crate) unsafe extern "thiscall" fn compare_drop_held_bans(this: usize, person: *const u32) {
    let l = this + discipline_fields::HELD_BANS;
    let n = stored_suspension_counter(this, *person as usize);
    let before = list_items_capped(l);
    let mut left = *n;
    let mut expected = Vec::new();
    for &e in &before {
        if left > 0 && read_usize(e) as u32 == *person {
            left -= 1;
        } else {
            expected.push(e);
        }
    }
    original_fn::<unsafe extern "thiscall" fn(usize, *const u32)>(IDX_DROP_HELD_BANS)(this, person);
    let after = list_items_capped(l);
    SCOREBOARD.record(after == expected && *n == left, || format!("drop_held_bans({}) itens {} -> {} (esperava {}), contador {} (esperava {})", *person, before.len(), after.len(), expected.len(), *n, left));
}

/// Comparer for `hold_bans_for_loan`.
///
/// [pt] Comparador: o plano da cópia (as vagas que não valem no clube do empréstimo) antes; a original guarda; confere o
/// registro do jogador: as vagas do plano em −1, as outras iguais e o contador somado.
pub(crate) unsafe extern "thiscall" fn compare_hold_bans_for_loan(this: usize, player: usize) {
    let sheet = exe::contract_manager_get_loan_contract(read_usize(G_CONTRACT_MANAGER), (player as *const i32) as usize);
    if sheet == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_HOLD_BANS_FOR_LOAN)(this, player);
    }
    let (reg, vagas) = suspensions_to_store(this, sheet, player);
    let mut expected = core::slice::from_raw_parts(reg as *const u8, 29).to_vec();
    for &k in &vagas {
        expected[5 * k as usize..5 * k as usize + 5].fill(0xff);
    }
    expected[0x1c] = expected[0x1c].wrapping_add(vagas.len() as u8);
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_HOLD_BANS_FOR_LOAN)(this, player);
    let after = core::slice::from_raw_parts(reg as *const u8, 29).to_vec();
    SCOREBOARD.record(after == expected, || format!("hold_bans_for_loan({:x}) vagas {:?} {:x?} x {:x?}", player, vagas, after, expected));
}

/// Comparer for `is_one_booking_from_ban`.
pub(crate) unsafe extern "thiscall" fn compare_is_one_booking_from_ban(this: usize, person: usize, competition: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_IS_ONE_BOOKING_FROM_BAN)(this, person, competition);
    let n = is_one_booking_from_ban(this, person, competition);
    SCOREBOARD.record(r == n, || format!("is_one_booking_from_ban(pessoa {:x}, competicao {:x}) original={:x} rust={:x}", person, competition, r, n));
    r
}

/// Comparer for `process_match`.
///
/// [pt] Comparador: o tribunal (0x7e7ff0) dá as suspensões: só a original.
pub(crate) unsafe extern "thiscall" fn compare_process_match(this: usize, partida: usize, result: u32) -> u32 {
    crate::comparison::branch("process_match");
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_PROCESS_MATCH)(this, partida, result)
}

/// Comparer for `discipline_remove_player`.
///
/// [pt] Comparador: a 0x556830 solta suspensões e tira (e libera) itens da lista: o plano antes (as entradas das vagas), a
/// original roda; confere o registro (0xff, conta 0) e as entradas livres (+0 = −1).
pub(crate) unsafe extern "thiscall" fn compare_discipline_remove_player(this: usize, id: u32) {
    let r = read_usize(this + discipline_fields::PERSON_RECORDS).wrapping_add((id as usize).wrapping_mul(29));
    let mut entries = Vec::new();
    for k in 0..4usize {
        let rule = *((r + k * 5) as *const u8);
        if rule != 0xff {
            let e = ruling_body_entry(ruling_body_of_rule(this, rule), id);
            if e != 0 {
                entries.push(e);
            }
        }
    }
    original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_DISCIPLINE_REMOVE_PLAYER)(this, id);
    let mut after = [0u8; 29];
    core::ptr::copy_nonoverlapping(r as *const u8, after.as_mut_ptr(), 29);
    let ok = after[..0x1c].iter().all(|&b| b == 0xff) && after[0x1c] == 0
        && entries.iter().all(|&e| read_usize(e) as u32 == 0xffff_ffff);
    SCOREBOARD.record(ok, || format!("discipline_remove_player({}) registro {:02x?} entradas {:x?}", id, after, entries));
}

/// Comparer for `find_held_ban`.
pub(crate) unsafe extern "thiscall" fn compare_find_held_ban(discipline: usize, id: i32, slot: i8) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, i8) -> usize>(IDX_FIND_HELD_BAN);
    // the list keeps the cursor: both start from the same one and must leave it the same
    let (ro, rr, igual, mo, mr) = crate::comparison::side_by_side(&[(discipline + discipline_fields::HELD_BANS, 0x12)], || o(discipline, id, slot), || find_held_ban(discipline, id, slot));
    SCOREBOARD.record(igual && ro == rr, || format!("find_held_ban({}, {}) original={:#x} rust={:#x} {:x?} x {:x?}", id, slot, ro, rr, mo, mr));
    ro
}
