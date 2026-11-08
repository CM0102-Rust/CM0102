use super::*;

// The comparers of the routines in ../injury.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../injury.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `register_injury` (comparison mode: see the header of motor.rs).
///
/// [pt] A 0x62be00 so e chamada pela 0x6d0980: a comparacao e a dela (a lista entra nas regioes).
pub(crate) unsafe extern "thiscall" fn comparar_register_injury(this: usize, sheet: usize, kind: *mut u8, chance: *mut u8) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut u8, *mut u8)>(IDX_REGISTER_INJURY)(this, sheet, kind, chance)
}

/// Comparer for `player_highlight` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_player_highlight(p: usize, code: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_PLAYER_HIGHLIGHT);
    if is_recording_top_level() {
        return o(p, code);
    }
    let m = *((p + match_player::ENGINE) as *const usize);
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.extend([(m, MOTOR), (0xb67a34, 4)]);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(p, code), || crate::rng::without_counting(|| player_highlight(p, code)));
    SCOREBOARD.record(igual && r == n, || format!("player_highlight({:#x}) original={} rust={} {:x?} x {:x?}", code as u16, r, n, mo, mr));
    r
}

/// Comparer for `post_recovery_news` (comparison mode: see the header of motor.rs), as a dry run (model
/// RARE_FINANCE_EVENT_DRY_RUN).
///
/// The copy runs first without delivering the news item and stopping before the absconding, then the original routine
/// from the same state; compares the player record (0x6e), the player sheet (0x46), the contract (0x4f), the form
/// record (31), byte +0xc of the 79-byte record, the club manager's record and the random generator state.
///
/// With the absconding in the copy (the original puts him on the list, draws at random and sends a news item): only the
/// original.
///
/// [pt] Comparador em ensaio (modelo RARE_FINANCE_EVENT_DRY_RUN): a cópia roda primeiro sem a entrega da notícia e parando antes da fuga,
/// depois a original a partir do mesmo estado; compara o registro do jogador (0x6e), a ficha (0x46), o contrato (0x4f),
/// o registro de forma (31), o byte +0xc do registro de 79 bytes, o registro do técnico do clube e o estado do sorteio.
/// Com a fuga na cópia (a original põe na lista, sorteia e manda notícia): só a original.
pub(crate) unsafe extern "thiscall" fn comparar_post_recovery_news(this: usize, p: usize, estagio: u32, kind: u32, force: u32, days: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(IDX_POST_RECOVERY_NEWS);
    if p == 0 {
        return o(this, p, estagio, kind, force, days);
    }
    let mut reg: Vec<(usize, usize)> = Vec::from(crate::rng::STATE_REGIONS);
    reg.push((p, 0x6e));
    if read_u32_as_usize(p + staff::PLAYER) != 0 {
        reg.push((read_u32_as_usize(p + staff::PLAYER), 0x46));
    }
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let before_1080 = read_u32_as_usize(G_CONTRACT_LOOKUP_RESULT);
    let c = contract_of(read_u32_as_usize(G_CONTRACT_MANAGER), p);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = before_1080;
    if c != 0 {
        reg.push((c, 0x4f));
    }
    reg.push((G_CONTRACT_LOOKUP_RESULT, 4));
    reg.push((physical_record_of_player(this, p), 31));
    reg.push((read_u32_as_usize(G_STAFF_STATE_TABLE).wrapping_add((*(p as *const i32) as isize * 0x4f) as usize), 0x4f));
    let club = read_u32_as_usize(p + staff::CLUB_JOB);
    if club != 0 && read_u32_as_usize(club + club::MANAGER) != 0 {
        reg.push((read_u32_as_usize(club + club::MANAGER), 0x6e));
    }
    let before = crate::comparison::MemorySnapshot::read(&reg);
    RECOVERY_NEWS_DRY_RUN = true;
    RECOVERY_NEWS_WOULD_GO_AWOL = false;
    crate::rng::pause_trace(|| crate::rng::without_counting(|| post_recovery_news(this, p, estagio, kind, force, days)));
    RECOVERY_NEWS_DRY_RUN = false;
    let fugiu = RECOVERY_NEWS_WOULD_GO_AWOL;
    let mr = crate::comparison::MemorySnapshot::read(&reg);
    before.write_back();
    o(this, p, estagio, kind, force, days);
    if fugiu {
        return;
    }
    let mo = crate::comparison::MemorySnapshot::read(&reg);
    SCOREBOARD.record(mo == mr, || format!("post_recovery_news(pessoa {:x}, estagio {}, tipo {:x}) {:x?} x {:x?}", p, estagio as u8, kind as u8, mo.0, mr.0));
}

/// Comparer for `injury_get_category_name` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado no texto (o tamanho, até 0x100) e na devolução; o tamanho curto (a caixa): só a original.
pub(crate) unsafe extern "stdcall" fn comparar_injury_get_category_name(category: u32, text: usize, size: i32) -> u32 {
    let o = original_fn::<unsafe extern "stdcall" fn(u32, usize, i32) -> u32>(IDX_INJURY_GET_CATEGORY_NAME);
    if size < 0x14 || text == 0 {
        return o(category, text, size);
    }
    let (r, n, igual, _, _) = crate::comparison::side_by_side(&[(text, (size as usize).min(0x100))], || o(category, text, size), || injury_get_category_name(category, text, size));
    SCOREBOARD.record(igual && r == n, || format!("injury_get_category_name({}) original={} rust={}", category as u8 as i8, r, n));
    r
}

/// Comparer for `add_injury` (0x62d250; comparison mode: see the header of motor.rs).
///
/// Only the original runs (it allocates, delivers the news item, schedules retirement and calls original routines that
/// draw random numbers along the way, such as training 0x8b1750 and relapse 0x62e500, which may injure again: it cannot
/// run twice). It checks what the injury guarantees afterwards: invalid arguments or recurrence only give −1; otherwise
/// −1 only with the person still injured (+0xe ≠ −1) or with no array; a valid slot with the person still in it (+0xe =
/// slot, with no relapse having changed it): the type in the slot and at +0x1d, +0x1c = 0, +0x2c = +0x28, +0x2e =
/// +0x2a, and +0x2a = the return to training from duration +0x28 (the copy's x87).
///
/// [pt] Comparador da 0x62d250: só a original (aloca, entrega a notícia, agenda aposentadoria e chama originais que sorteiam
/// no meio, como o treino 0x8b1750 e a recaída 0x62e500, que pode lesionar de novo: não pode rodar duas vezes). Confere
/// o que a lesão garante depois: argumentos inválidos ou só recorrência dão −1; −1 de resto só com a pessoa ainda
/// lesionada (+0xe ≠ −1) ou sem vetor; vaga válida com a pessoa ainda nela (+0xe = vaga, sem recaída que a tenha
/// trocado): o tipo na vaga e em +0x1d, +0x1c = 0, +0x2c = +0x28, +0x2e = +0x2a e +0x2a = a volta ao treino pela
/// duração +0x28 (o x87 da cópia).
pub(crate) unsafe extern "thiscall" fn comparar_add_injury(this: usize, kind_arg: u32, retroativa: u32, p: usize, game: usize, surto: u32, news: u32, rec_arg: u32, so_recorrencia: u32) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, usize, usize, u32, u32, u32, u32) -> i32>(IDX_ADD_INJURY);
    let r = o(this, kind_arg, retroativa, p, game, surto, news, rec_arg, so_recorrencia);
    let kind = kind_arg as u8 as i8;
    let valida = p != 0 && (read_usize(p) as i32) < read_usize(G_STAFF_COUNT) as i32 && read_usize(p + staff::PLAYER) != 0 && (0..0x52).contains(&kind);
    let ok = if !valida || so_recorrencia != 0 {
        r == -1
    } else {
        let reg = physical_record(this, read_usize(p));
        let current = read_usize(reg + 0xe) as i32;
        if r == -1 {
            current != -1 || read_usize(this + 4) == 0
        } else if r < 0 || current != r {
            r >= 0 // replaced later (the relapse injured him again): nothing to check
        } else {
            let l = injury_entry(this, r as usize);
            // the relapse (0x62e500) may injure the person themselves again with 0x4d/0x4e/0x4f: the swap frees this slot and
            // the new injury reoccupies it (the first free one), with its type; checks the new one
            let t = read_u8(l) as i8;
            let kind = if t != kind && (0x4d..=0x4f).contains(&t) { t } else { kind };
            read_u8(l) as i8 == kind
                && read_u8(reg + 0x1d) as i8 == kind
                && read_u8(reg + 0x1c) == 0
                && read_i16(l + 0x2c) == read_i16(l + 0x28)
                && read_i16(l + 0x2e) == read_i16(l + 0x2a)
                && read_i16(l + 0x2a) == return_to_training_days(kind, read_i16(l + 0x28))
        }
    };
    SCOREBOARD.record(ok, || format!("add_injury(tipo {:#x}, retroativa {}, pessoa {:#x}, recorrencia {}, so {}) = {}", kind_arg, retroativa, p, rec_arg as u8 as i8, so_recorrencia, r));
    r
}

/// Comparer for `injury_post_injury_news` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x62dca0: só a original (entrega a notícia, que não pode sair duas vezes; a cópia leva o ramo no modo
/// Rust). simplification: sem plano a conferir além do que a original entrega.
pub(crate) unsafe extern "thiscall" fn comparar_injury_post_injury_news(this: usize, p: usize, l: usize, arg: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(IDX_INJURY_POST_INJURY_NEWS)(this, p, l, arg)
}

/// Comparer for `injury_daily_update` (comparison mode: see the header of motor.rs): side-by-side comparer of the
/// training phase.
///
/// The copy runs as a dry run (without freeing the list, without the new injury and without the final routines) and the
/// original routine from the same state (the Injury header, the injury table and the random draw); the original is
/// checked when it reaches the return-from-injury news (`0x62e010`, detoured in the same item), before the final
/// routines. With a new injury (it allocates and sends news) or with everyone's fitness (day 180 of the first season):
/// only the original routine.
///
/// [pt] Comparador lado a lado da fase do treino: a cópia em ensaio (sem soltar a lista, sem a lesão nova e sem as rotinas
/// do fim) e a original a partir do mesmo estado (o cabeçalho do Injury, a tabela de lesões e o sorteio); a original é
/// conferida quando chega às notícias da volta (0x62e010, desviada no mesmo item), antes das rotinas do fim. Com lesão
/// nova (aloca e manda notícia) ou com o físico de todos (dia 180 da primeira temporada): só a original.
pub(crate) unsafe extern "thiscall" fn comparar_injury_daily_update(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_INJURY_DAILY_UPDATE);
    let mut reg = vec![(this, 0x2c)];
    let n = read_i32(this + 8);
    if n > 0 && read_usize(this + 4) != 0 {
        reg.push((read_usize(this + 4), n as usize * 0x30));
    }
    reg.extend(crate::rng::STATE_REGIONS);
    let before = crate::comparison::MemorySnapshot::read(&reg);
    INJURY_DAILY_DRY_RUN = true;
    INJURY_DAILY_STOPPED = false;
    crate::rng::pause_trace(|| crate::rng::without_counting(|| injury_daily_update(this)));
    INJURY_DAILY_DRY_RUN = false;
    let mr = crate::comparison::MemorySnapshot::read(&reg);
    before.write_back();
    if INJURY_DAILY_STOPPED {
        crate::comparison::branch("cpw_9 lesao nova so na original");
        return o(this);
    }
    *core::ptr::addr_of_mut!(INJURY_DAILY_PENDING_REGIONS) = Some((reg, mr));
    o(this);
    // without 0x62e010 hooked (disabled), the original did not go through the checkpoint: it stays without comparison
    *core::ptr::addr_of_mut!(INJURY_DAILY_PENDING_REGIONS) = None;
}

/// Comparer for `post_due_recovery_news` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_post_due_recovery_news(this: usize) {
    if let Some((reg, mr)) = (*core::ptr::addr_of_mut!(INJURY_DAILY_PENDING_REGIONS)).take() {
        let mo = crate::comparison::MemorySnapshot::read(&reg);
        SCOREBOARD.record(mo == mr, || format!("injury_daily_update({:x}) {:x?} x {:x?}", this, mo, mr));
    }
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_POST_DUE_RECOVERY_NEWS)(this)
}

/// Comparer for `apply_match_effects` (comparison mode: see the header of motor.rs).
///
/// Side by side on the random generator's state, on the cursor of the knocks list and on the recovery (+0xa, 4 bytes,
/// of the 31-byte record at [this]) of each person in the slots.
///
/// A match with a knock that reaches the injury type: only the original routine (the new injury allocates and sends the
/// news item; the severity of the first knock may be stack residue).
///
/// [pt] Comparador: lado a lado no estado do sorteio, no cursor da lista das batidas e na recuperação (+0xa, 4 bytes, do
/// registro de 31 bytes em [this]) de cada pessoa das vagas. Partida com batida que chega ao tipo da lesão: só a original (a lesão nova aloca e manda a
/// notícia; a gravidade da primeira batida pode ser sobra de pilha).
pub(crate) unsafe extern "thiscall" fn comparar_apply_match_effects(this: usize, game: usize, partida: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_APPLY_MATCH_EFFECTS);
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    let mut batida = false;
    for lado in 0..2usize {
        for k in 0..0x14usize {
            let slot = partida + 0x1c0 + 0x625 * lado + 0x49 * k;
            let Some((id, pessoa)) = staff_of_match_slot(slot) else { continue };
            if pessoa == 0 {
                return o(this, game, partida);
            }
            reg.push(((read_i32(this) as u32).wrapping_add((id as u32).wrapping_mul(31)).wrapping_add(0xa) as usize, 4));
            if *((slot - 0x11) as *const u8) != 0 && read_i32(this + injury_fields::MATCH_HIT_LIST) != 0 && has_injury_hit(this, id, pessoa) {
                batida = true;
            }
        }
    }
    if batida {
        crate::comparison::branch("apply_match_effects com batida");
        return o(this, game, partida);
    }
    if read_i32(this + injury_fields::MATCH_HIT_LIST) != 0 {
        reg.push((read_i32(this + injury_fields::MATCH_HIT_LIST) as usize + 0xa, 4)); // the cursor of the kicks list
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, game, partida), || crate::rng::without_counting(|| apply_match_effects(this, game, partida)));
    SCOREBOARD.record(igual, || format!("apply_match_effects({:x}, {:x}) {:x?} x {:x?}", game, partida, mo, mr));
}

/// Comparer for `find_match_injury` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado no cursor da lista e na devolução.
pub(crate) unsafe extern "thiscall" fn comparar_find_match_injury(this: usize, person: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_FIND_MATCH_INJURY);
    let list = read_i32(this + injury_fields::MATCH_HIT_LIST) as usize;
    let reg: &[(usize, usize)] = if list == 0 { &[] } else { &[(list + 0xa, 4)] };
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(reg, || o(this, person), || find_match_injury(this, person));
    SCOREBOARD.record(igual && r == n, || format!("find_match_injury({:x}) original={:x} {:x?} rust={:x} {:x?}", person, r, mo, n, mr));
    r
}

/// Comparer for `is_player_on_holiday` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a devolução (só o al).
pub(crate) unsafe extern "thiscall" fn comparar_is_player_on_holiday(this: usize, person: usize) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_IS_PLAYER_ON_HOLIDAY);
    let r = o(this, person);
    let n = is_player_on_holiday(this, person);
    SCOREBOARD.record(r == n, || format!("is_player_on_holiday({:x}) original={} rust={}", person, r, n));
    r
}

/// Comparer for `end_player_injury` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado na entrada (tipo) e no +0xe do registro quando nada sai da rotina (férias e sem notícia);
/// com o histórico (arquivo) ou a notícia: só a original, conferindo o plano (tipo 0xff e +0xe −1). Sem pessoa: a
/// original (caixa).
pub(crate) unsafe extern "thiscall" fn comparar_end_player_injury(this: usize, person: usize, news: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_END_PLAYER_INJURY);
    if person == 0 || read_i32(person) >= read_i32(G_STAFF_COUNT) {
        return o(this, person, news);
    }
    let r = physical_record_of_id(this, read_i32(person));
    let i = read_i32(r + 0xe);
    if i == -1 {
        return o(this, person, news);
    }
    let e = (read_i32(this + 4) as u32).wrapping_add((i as u32).wrapping_mul(0x30)) as usize;
    let human: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let club = read_i32(person + staff::CLUB_JOB) as usize;
    let away = *(e as *const u8) != 0x51 || (news != 0 && club != 0 && human(club, 0, 1, 0, 0) != 0);
    if away {
        o(this, person, news);
        let ok = *(e as *const u8) == 0xff && read_i32(physical_record_of_id(this, read_i32(person)) + 0xe) == -1;
        SCOREBOARD.record(ok, || format!("end_player_injury({:x}, {}) plano: tipo {:x} e +0xe {}", person, news, *(e as *const u8), read_i32(physical_record_of_id(this, read_i32(person)) + 0xe)));
        return;
    }
    let reg = [(e, 1), (r + 0xe, 4)];
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(this, person, news), || end_player_injury(this, person, news));
    SCOREBOARD.record(igual, || format!("end_player_injury({:x}, {}) {:x?} x {:x?}", person, news, mo, mr));
}

/// Comparer for `get_player_injury_status` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original (com a forma do dia) e o estado relido pela regra (injury_status, sem rodar a 0x62c680 de
/// novo: a original já a rodou).
pub(crate) unsafe extern "thiscall" fn comparar_get_player_injury_status(this: usize, person: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_GET_PLAYER_INJURY_STATUS)(this, person);
    if person != 0 && (read_u32_as_usize(person) as i32) < read_u32_as_usize(G_STAFF_COUNT) as i32 {
        let n = injury_status(this, person);
        SCOREBOARD.record(r == n, || format!("get_player_injury_status(pessoa {:x}) original={} rust={}", person, r, n));
    }
    r
}

/// Comparer for `add_illness_outbreak` (comparison mode: see the header of motor.rs).
///
/// The original routine draws at random, allocates and reorders: the plan first (the draws and the selection, with the
/// random generator's state saved and restored); then the original runs.
///
/// It checks the same random generator state and, if the outbreak was added, the count +1 and the outbreak (date, type,
/// club, match) in the vector; otherwise the count unchanged.
///
/// [pt] Comparador: a original sorteia, aloca e reordena: o plano antes (os sorteios e a seleção, com o estado do sorteio
/// guardado e devolvido); a original roda. Confere o mesmo estado do sorteio e, se o surto entrou, a conta +1 e o surto
/// (data, tipo, clube, partida) no vetor; senão a conta igual.
pub(crate) unsafe extern "thiscall" fn comparar_add_illness_outbreak(this: usize, data: usize, kind: u32, club_i: i32, partida: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, i32, usize)>(IDX_ADD_ILLNESS_OUTBREAK);
    let state = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS);
    let enters = crate::rng::pause_trace(|| crate::rng::without_counting(|| outbreak_takes_hold(kind as u8, club_i)));
    let after_of_plan = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS);
    state.write_back();
    let n0 = read_i32(this + injury_fields::OUTBREAK_COUNT);
    let mut expected = [0u8; OUTBREAK_SIZE];
    expected[..8].copy_from_slice(&*(data as *const [u8; 8]));
    expected[8] = kind as u8;
    expected[9..13].copy_from_slice(&club_i.to_le_bytes());
    let id = if partida != 0 { read_i32(partida) } else { -1 };
    expected[13..17].copy_from_slice(&id.to_le_bytes());
    o(this, data, kind, club_i, partida);
    let n = read_i32(this + injury_fields::OUTBREAK_COUNT);
    let same_draw = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS) == after_of_plan;
    let v = read_usize(this + injury_fields::OUTBREAKS);
    let ok = if enters && v != 0 {
        n == n0 + 1 && (0..n.max(0) as usize).any(|k| *((v + k * OUTBREAK_SIZE) as *const [u8; OUTBREAK_SIZE]) == expected)
    } else {
        n == n0
    };
    SCOREBOARD.record(ok && same_draw, || {
        format!("add_illness_outbreak(tipo {}, clube {}) plano entra={}, conta {} -> {}, mesmo sorteio={}", kind as u8, club_i, enters, n0, n, same_draw)
    });
}

/// Comparer for `spread_illness_outbreaks` (comparison mode: see the header of motor.rs).
///
/// The original routine draws at random and makes people ill (0x62d250, which may add outbreaks): only it runs, and the
/// plan made beforehand is checked: how many outbreaks were more than five days old (the ones that leave) and,
/// afterwards, no outbreak older than five days and the count at least the previous one minus those that leave.
///
/// [pt] Comparador: a original sorteia e adoece (0x62d250, que pode acrescentar surtos): roda só ela e confere o plano
/// feito antes: quantos surtos tinham mais de cinco dias (os que saem) e, depois, nenhum surto com mais de cinco dias e
/// a conta pelo menos a de antes menos os que saem.
pub(crate) unsafe extern "thiscall" fn comparar_spread_illness_outbreaks(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_SPREAD_ILLNESS_OUTBREAKS);
    let cinco = five_days_ago();
    let n0 = read_i32(this + injury_fields::OUTBREAK_COUNT);
    let hoje4 = read_usize(G_CURRENT_DATE) as u32;
    let mut saem = 0;
    for k in 0..n0.max(0) as usize {
        let e = read_usize(this + injury_fields::OUTBREAKS) + k * OUTBREAK_SIZE;
        let (ano_e, ano_h) = (read_i16(e + 2) as u16, (hoje4 >> 16) as u16);
        if ano_h < ano_e || (ano_h == ano_e && (hoje4 as u16 as i16) < read_i16(e)) {
            break;
        }
        if is_after_outbreak(&cinco, e) {
            saem += 1;
        }
    }
    o(this);
    let n = read_i32(this + injury_fields::OUTBREAK_COUNT);
    let old = (0..n.max(0) as usize).find(|&k| is_after_outbreak(&cinco, read_usize(this + injury_fields::OUTBREAKS) + k * OUTBREAK_SIZE));
    SCOREBOARD.record(old.is_none() && n >= n0 - saem, || {
        format!("spread_illness_outbreaks: conta {} -> {} (saem {}), surto velho {:?}", n0, n, saem, old)
    });
}

/// Comparer for `injury_player_in_close_season` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (datas, a seleção e a temporada só calculam).
pub(crate) unsafe extern "stdcall" fn comparar_injury_player_in_close_season(person: usize) -> u32 {
    let r = original_fn::<unsafe extern "stdcall" fn(usize) -> u32>(IDX_INJURY_PLAYER_IN_CLOSE_SEASON)(person);
    let n = injury_player_in_close_season(person);
    SCOREBOARD.record(r == n, || format!("injury_player_in_close_season({:x}) original={} rust={}", person, r, n));
    r
}

/// Comparer for `init_squad_fitness` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelos registros de lesão (31 bytes × [0xae2364]), pelos clubes (as vagas) e pelo clube e
/// cargo (+0x39..+0x3d) de quem não é jogador nas vagas.
pub(crate) unsafe extern "thiscall" fn comparar_init_squad_fitness(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_INIT_SQUAD_FITNESS);
    let base = read_usize(G_CLUB_TABLE);
    let mut regions = vec![(read_usize(this), (read_i32(G_STAFF_COUNT).max(0) as usize) * 31), (base, (read_i32(G_CLUB_COUNT).max(0) as usize) * club::SIZE)];
    for i in 0..injury_club_count_without_nations().max(0) as usize {
        for s in 0..50usize {
            let p = read_usize(base + i * club::SIZE + 0xd7 + 4 * s);
            if p != 0 && read_i32(p) < read_i32(G_STAFF_COUNT) && read_usize(p + staff::PLAYER) == 0 {
                regions.push((p + staff::CLUB_JOB, 5));
            }
        }
    }
    let (_, _, igual, _, _) = crate::comparison::side_by_side(&regions, || o(this), || init_squad_fitness(this));
    SCOREBOARD.record(igual, || format!("init_squad_fitness: memória difere ({} regiões)", regions.len()));
}

/// Comparer for `init_records` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original aloca, sorteia e lesiona (0x62d250): roda só ela e confere o que a 0x62d250 não muda: o
/// vetor alocado e o +0x1e = 50 de todos os registros.
pub(crate) unsafe extern "thiscall" fn comparar_init_records(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_INIT_RECORDS)(this);
    let v = read_usize(this);
    if v == 0 {
        return;
    }
    let errado = (0..read_i32(G_STAFF_COUNT).max(0) as usize).find(|&k| read_u8(v + 31 * k + 0x1e) != 0x32);
    SCOREBOARD.record(errado.is_none(), || format!("init_records: registro {:?} sem o +0x1e = 50", errado));
}

/// Comparer for `roll_recurring_injury` (comparison mode: see the header of motor.rs): `0x62d250` (treat) cannot run
/// twice.
///
/// The plan first, with the random-draw state saved (without counting or tracing) and the record marks restored; then
/// the original routine runs. It checks the marks of the record and of the chosen one and, outside the treat path, that
/// the random-draw state equals the plan's (the same random draws).
///
/// [pt] Comparador: a 0x62d250 (tratar) não pode rodar duas vezes: o plano antes, com o estado do sorteio guardado (sem
/// contar nem rastrear) e as marcas do registro devolvidas; a original roda. Confere as marcas do registro e do
/// escolhido e, fora do tratar, o estado do sorteio igual ao do plano (os mesmos sorteios).
pub(crate) unsafe extern "thiscall" fn comparar_roll_recurring_injury(this: usize, record: usize, injury: *const u8) {
    let state = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS);
    let marks = (read_u8(record + 0x12), read_u8(record + 0x13));
    let plan = crate::rng::pause_trace(|| crate::rng::without_counting(|| plan_recurring_injury(this, record, injury)));
    let after_of_plan = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS);
    state.write_back();
    *((record + 0x12) as *mut u8) = marks.0;
    *((record + 0x13) as *mut u8) = marks.1;
    let kind = *injury;
    original_fn::<unsafe extern "thiscall" fn(usize, usize, *const u8)>(IDX_ROLL_RECURRING_INJURY)(this, record, injury);
    let now = (read_u8(record + 0x12), read_u8(record + 0x13));
    let same_draw = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS) == after_of_plan;
    let ok = match plan {
        RecurringInjuryOutcome::Nada => now == marks && same_draw,
        RecurringInjuryOutcome::Marcada => now == (kind, 0) && same_draw,
        RecurringInjuryOutcome::Limpar(r) => {
            (read_u8(r + 0x12), read_u8(r + 0x13)) == (0xff, 0) && (r == record || now == (kind, 0)) && same_draw
        }
        RecurringInjuryOutcome::Tratar(_) => true, // a 0x62d250 sorteia e grava o resto
    };
    SCOREBOARD.record(ok, || {
        format!("roll_recurring_injury(tipo {}) plano {:x?}, marcas {:?} -> {:?}, mesmo sorteio={}", kind, plan, marks, now, same_draw)
    });
}

/// Comparer for `reset_player_history` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelo começo e o último deslocamento do histórico (8 bytes; a 0x58f110 só grava o segundo).
pub(crate) unsafe extern "thiscall" fn comparar_reset_player_history(this: usize, person: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_RESET_PLAYER_HISTORY);
    if person == 0 {
        return o(this, person);
    }
    let r = injury_history_record(this, read_usize(person));
    let (a, b, igual, mo, mr) = crate::comparison::side_by_side_light(&[(r + 0x14, 8)], || o(this, person), || reset_player_history(this, person));
    SCOREBOARD.record(igual && a == b, || format!("reset_player_history({}) {:x?} x {:x?}", read_usize(person), mo, mr));
    a
}

/// Comparer for `injury_reset_player` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a 0x68d1f0 libera o item tirado e a 0x62e700 grava o histórico no arquivo: o plano antes (os itens
/// que ficam, o byte do registro de 79 bytes, os bytes do registro que não mudam); a original roda; confere.
pub(crate) unsafe extern "thiscall" fn comparar_injury_reset_player(this: usize, id: u32) -> usize {
    let l = read_usize(this + injury_fields::MATCH_HIT_LIST);
    let expected: Vec<usize> = if l != 0 { list_items_capped(l).into_iter().filter(|&e| read_usize(e) as u32 != id).collect() } else { Vec::new() };
    let r = injury_history_record(this, id as usize);
    let g = read_usize(G_STAFF_STATE_TABLE).wrapping_add((id as usize).wrapping_mul(79)) + 0xc;
    let (byte, nivel, b1c) = (read_u8(g) & 0xfe, read_u8(r + 0x13), read_u8(r + 0x1c));
    let ret = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(IDX_INJURY_RESET_PLAYER)(this, id);
    let items = if l != 0 { list_items_capped(l) } else { Vec::new() };
    let w = |o: usize| core::ptr::read_unaligned((r + o) as *const i16);
    let dd = |o: usize| read_usize(r + o) as i32;
    let ok = items == expected && ret == g && read_u8(g) == byte
        && read_usize(r) == read_usize(G_CURRENT_DATE) && read_usize(r + 4) == read_usize(G_CURRENT_DATE_LEAP)
        && w(8) == 10000 && w(0xa) == 10000 && w(0xc) == 0 && dd(0xe) == -1 && read_u8(r + 0x12) == 0xff
        && read_u8(r + 0x13) == nivel && dd(0x14) == -1 && dd(0x18) == -1 && read_u8(r + 0x1c) == b1c && read_u8(r + 0x1d) == 0xff;
    SCOREBOARD.record(ok, || {
        let v: Vec<u8> = (0..31).map(|o| read_u8(r + o)).collect();
        format!("injury_reset_player({}) itens {} (esperava {}), byte {:#x} (esperava {:#x}), registro {:02x?}", id, items.len(), expected.len(), read_u8(g), byte, v)
    });
    ret
}

/// Comparer for `recover_injury` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_recover_injury(t: usize, p: *const i32, mark: i16, ritmo: i16) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *const i32, i16, i16)>(IDX_RECOVER_INJURY);
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    if !p.is_null() && *p < *(G_STAFF_COUNT as *const i32) {
        reg.push(((*(t as *const usize)).wrapping_add((*p as usize).wrapping_mul(31)).wrapping_add(0xa), 4));
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(t, p, mark, ritmo), || crate::rng::without_counting(|| recover_injury(t, p, mark, ritmo)));
    SCOREBOARD.record(igual, || format!("recover_injury({:#x}, {:p}, {}, {}) {:x?} x {:x?}", t, p, mark, ritmo, mo, mr));
}

/// Comparer for `get_condition` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (a 0x62c400 só lê).
pub(crate) unsafe extern "thiscall" fn comparar_get_condition(this: usize, player: usize, em_porcento: u32) -> u16 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u16>(IDX_GET_CONDITION)(this, player, em_porcento);
    let n = get_condition(this, player, em_porcento);
    SCOREBOARD.record(r == n, || format!("get_condition({:x}, {}) original={} rust={}", player, em_porcento, r, n));
    r
}

/// Comparer for `set_training_load` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_set_training_load(injuries: usize, id: *const i32, value: u8) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *const i32, u8)>(IDX_SET_TRAINING_LOAD);
    let a = training_load_byte(injuries, id) as usize;
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(a, 1)], || o(injuries, id, value), || set_training_load(injuries, id, value));
    SCOREBOARD.record(igual, || format!("set_training_load({}, {}) {:x?} x {:x?}", *id, value, mo, mr));
}

/// Comparer for `update_player_fitness` (comparison mode: see the header of motor.rs), side by side.
///
/// Compared: the person's 31-byte record, byte +0xa of the 79-byte record, the statics and the club's saved data
/// (0xb63d08..0xb63d34), and the random draw. The copy runs as a dry run, without the end of the injury (0x62e700):
/// when it reaches that point, only the original stands (it already ran first). The first time (statics still to be
/// created, with atexit) and with no record (null person, id outside the table, no player sheet): only the original
/// runs.
///
/// [pt] Comparador lado a lado: o registro de 31 bytes da pessoa, o byte +0xa do registro de 79 bytes, as estáticas e o
/// guardado do clube (0xb63d08..0xb63d34) e o sorteio. A cópia roda em ensaio, sem o fim da lesão (0x62e700): quando
/// ela chega lá, fica a original sozinha (já rodou primeiro). Na primeira vez (estáticas por criar, com atexit) e sem
/// registro (pessoa nula, id fora da tabela, sem ficha): só a original.
pub(crate) unsafe extern "thiscall" fn comparar_update_player_fitness(this: usize, person: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_UPDATE_PLAYER_FITNESS);
    if *(G_FITNESS_UPDATE_STATIC_INIT_FLAGS as *const u8) & 7 != 7 || person == 0 || read_i32(person) < 0 || read_i32(person) >= read_i32(G_STAFF_COUNT) || read_usize(person + staff::PLAYER) == 0 {
        return o(this, person); // simplification: the statics and the first-time atexit cannot run twice
    }
    let id = read_i32(person) as usize;
    let rec = read_usize(this).wrapping_add(id.wrapping_mul(31));
    let general = read_usize(G_STAFF_STATE_TABLE).wrapping_add(id.wrapping_mul(0x4f));
    let e = crate::rng::STATE_REGIONS;
    let reg = [(rec, 31), (general + 0xa, 1), (G_FITNESS_UPDATE_CACHE_DATE, 0x2c), e[0], e[1]];
    END_INJURY_REACHED = false;
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(this, person), || {
        END_INJURY_DRY_RUN = true;
        crate::rng::without_counting(|| update_player_fitness(this, person));
        END_INJURY_DRY_RUN = false;
    });
    if END_INJURY_REACHED {
        crate::comparison::branch("cpw_8 fim da lesao so na original");
        return;
    }
    SCOREBOARD.record(igual, || format!("update_player_fitness(pessoa {:x}) {:x?} x {:x?}", person, mo, mr));
}

/// Comparer for `daily_wear` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_daily_wear(t: usize, id: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32)>(IDX_DAILY_WEAR);
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    if id < *(G_STAFF_COUNT as *const i32) {
        reg.push(((*(t as *const usize)).wrapping_add((id as usize).wrapping_mul(31)).wrapping_add(0xc), 2));
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(t, id), || crate::rng::without_counting(|| daily_wear(t, id)));
    SCOREBOARD.record(igual, || format!("daily_wear({:#x}, {}) {:x?} x {:x?}", t, id, mo, mr));
}

/// Comparer for `choose_injury_type` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_choose_injury_type(injuries: usize, category: i8, severity: i8, player: usize) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8, i8, usize) -> u8>(IDX_CHOOSE_INJURY_TYPE);
    // simplification: a null player opens the error box of 0x62ed70; does not run twice
    if player == 0 {
        return o(injuries, category, severity, player);
    }
    // simplification: 0x62ed70 (first effect of 0x62d140, before any random roll of its own) goes through the
    // 0x62c680, which rolls only on the first query of the day and caches it; calling it here first fills the
    // cache in the same order as the original, and both sides of lado_a_lado find the cache full
    let current: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY);
    current(injuries, player);
    let (r, n, igual, _, _) = crate::comparison::side_by_side(&crate::rng::STATE_REGIONS,
        || o(injuries, category, severity, player), || crate::rng::without_counting(|| choose_injury_type(injuries, category, severity, player)));
    SCOREBOARD.record(igual && r == n, || format!("choose_injury_type({}, {}) original={} rust={} sorteio igual={}", category, severity, r, n, igual));
    r
}

/// Comparer for `injury_compare_entry_dates` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_injury_compare_entry_dates(key: usize, item: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_INJURY_COMPARE_ENTRY_DATES)(key, item);
    let n = injury_compare_entry_dates(key, item);
    SCOREBOARD.record(r == n, || format!("injury_compare_entry_dates(chave {:x}, item {:x}) original={} rust={}", key, item, r, n));
    r
}

/// Comparador: o al devolvido.
pub(crate) unsafe extern "thiscall" fn comparar_get_recurring_injury_type(this: usize, person: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_GET_RECURRING_INJURY_TYPE)(this, person);
    let n = get_recurring_injury_type(this, person);
    SCOREBOARD.record(r == n, || format!("get_recurring_injury_type(pessoa {:x}) original={:x} rust={:x}", person, r, n));
    r
}

/// Comparer for `injury_severity` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_injury_severity(injuries: usize, kind: i8) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, i8) -> u8>(IDX_INJURY_SEVERITY)(injuries, kind);
    let n = injury_severity(injuries, kind);
    SCOREBOARD.record(r == n, || format!("injury_severity({}) original={} rust={}", kind, r, n));
    r
}

/// Comparer for `injury_grade` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_injury_grade(injuries: usize, person: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_INJURY_GRADE)(injuries, person);
    let n = injury_grade(injuries, person);
    SCOREBOARD.record(r == n, || format!("injury_grade() original={} rust={}", r, n));
    r
}
