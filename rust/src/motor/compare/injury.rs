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
    let m = *((p + 0x19e) as *const usize);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(m, MOTOR), (0xb67a34, 4)]);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(p, code), || crate::aleatorio::sem_contar(|| player_highlight(p, code)));
    PLACAR.registrar(igual && r == n, || format!("player_highlight({:#x}) original={} rust={} {:x?} x {:x?}", code as u16, r, n, mo, mr));
    r
}

/// Comparer for `post_recovery_news` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador em ensaio (modelo RARE_FINANCE_EVENT_DRY_RUN): a cópia roda primeiro sem a entrega da notícia e parando antes da fuga,
/// depois a original a partir do mesmo estado; compara o registro do jogador (0x6e), a ficha (0x46), o contrato (0x4f),
/// o registro de forma (31), o byte +0xc do registro de 79 bytes, o registro do técnico do clube e o estado do sorteio.
/// Com a fuga na cópia (a original põe na lista, sorteia e manda notícia): só a original.
pub(crate) unsafe extern "thiscall" fn comparar_post_recovery_news(this: usize, p: usize, estagio: u32, kind: u32, forcar: u32, days: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(IDX_POST_RECOVERY_NEWS);
    if p == 0 {
        return o(this, p, estagio, kind, forcar, days);
    }
    let mut reg: Vec<(usize, usize)> = Vec::from(crate::aleatorio::REGIOES_DO_ESTADO);
    reg.push((p, 0x6e));
    if read_u32_as_usize(p + 0x61) != 0 {
        reg.push((read_u32_as_usize(p + 0x61), 0x46));
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
    let clube = read_u32_as_usize(p + 0x39);
    if clube != 0 && read_u32_as_usize(clube + club::MANAGER) != 0 {
        reg.push((read_u32_as_usize(clube + club::MANAGER), 0x6e));
    }
    let before = crate::comparacao::Memoria::ler(&reg);
    RECOVERY_NEWS_DRY_RUN = true;
    RECOVERY_NEWS_WOULD_GO_AWOL = false;
    crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| post_recovery_news(this, p, estagio, kind, forcar, days)));
    RECOVERY_NEWS_DRY_RUN = false;
    let fugiu = RECOVERY_NEWS_WOULD_GO_AWOL;
    let mr = crate::comparacao::Memoria::ler(&reg);
    before.escrever();
    o(this, p, estagio, kind, forcar, days);
    if fugiu {
        return;
    }
    let mo = crate::comparacao::Memoria::ler(&reg);
    PLACAR.registrar(mo == mr, || format!("post_recovery_news(pessoa {:x}, estagio {}, tipo {:x}) {:x?} x {:x?}", p, estagio as u8, kind as u8, mo.0, mr.0));
}

/// Comparer for `injury_get_category_name` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado no texto (o tamanho, até 0x100) e na devolução; o tamanho curto (a caixa): só a original.
pub(crate) unsafe extern "stdcall" fn comparar_injury_get_category_name(category: u32, text: usize, size: i32) -> u32 {
    let o = original_fn::<unsafe extern "stdcall" fn(u32, usize, i32) -> u32>(IDX_INJURY_GET_CATEGORY_NAME);
    if size < 0x14 || text == 0 {
        return o(category, text, size);
    }
    let (r, n, igual, _, _) = crate::comparacao::lado_a_lado(&[(text, (size as usize).min(0x100))], || o(category, text, size), || injury_get_category_name(category, text, size));
    PLACAR.registrar(igual && r == n, || format!("injury_get_category_name({}) original={} rust={}", category as u8 as i8, r, n));
    r
}

/// Comparer for `add_injury` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x62d250: só a original (aloca, entrega a notícia, agenda aposentadoria e chama originais que sorteiam
/// no meio, como o treino 0x8b1750 e a recaída 0x62e500, que pode lesionar de novo: não pode rodar duas vezes). Confere
/// o que a lesão garante depois: argumentos inválidos ou só recorrência dão −1; −1 de resto só com a pessoa ainda
/// lesionada (+0xe ≠ −1) ou sem vetor; vaga válida com a pessoa ainda nela (+0xe = vaga, sem recaída que a tenha
/// trocado): o tipo na vaga e em +0x1d, +0x1c = 0, +0x2c = +0x28, +0x2e = +0x2a e +0x2a = a volta ao treino pela
/// duração +0x28 (o x87 da cópia).
pub(crate) unsafe extern "thiscall" fn comparar_add_injury(this: usize, kind_arg: u32, retroativa: u32, p: usize, jogo: usize, surto: u32, noticia: u32, rec_arg: u32, so_recorrencia: u32) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, usize, usize, u32, u32, u32, u32) -> i32>(IDX_ADD_INJURY);
    let r = o(this, kind_arg, retroativa, p, jogo, surto, noticia, rec_arg, so_recorrencia);
    let kind = kind_arg as u8 as i8;
    let valida = p != 0 && (read_usize(p) as i32) < read_usize(G_STAFF_COUNT) as i32 && read_usize(p + 0x61) != 0 && (0..0x52).contains(&kind);
    let ok = if !valida || so_recorrencia != 0 {
        r == -1
    } else {
        let reg = physical_record(this, read_usize(p));
        let current = read_usize(reg + 0xe) as i32;
        if r == -1 {
            current != -1 || read_usize(this + 4) == 0
        } else if r < 0 || current != r {
            r >= 0 // trocada depois (a recaída lesionou de novo): nada a conferir
        } else {
            let l = injury_entry(this, r as usize);
            // a recaída (0x62e500) pode lesionar de novo a própria pessoa com 0x4d/0x4e/0x4f: a troca libera esta vaga e
            // a nova lesão a reocupa (a primeira livre), com o tipo dela; confere a nova
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
    PLACAR.registrar(ok, || format!("add_injury(tipo {:#x}, retroativa {}, pessoa {:#x}, recorrencia {}, so {}) = {}", kind_arg, retroativa, p, rec_arg as u8 as i8, so_recorrencia, r));
    r
}

/// Comparer for `injury_post_injury_news` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x62dca0: só a original (entrega a notícia, que não pode sair duas vezes; a cópia leva o ramo no modo
/// Rust). simplification: sem plano a conferir além do que a original entrega.
pub(crate) unsafe extern "thiscall" fn comparar_injury_post_injury_news(this: usize, p: usize, l: usize, arg: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(IDX_INJURY_POST_INJURY_NEWS)(this, p, l, arg)
}

/// Comparer for `injury_daily_update` (comparison mode: see the header of motor.rs).
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
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let before = crate::comparacao::Memoria::ler(&reg);
    INJURY_DAILY_DRY_RUN = true;
    INJURY_DAILY_STOPPED = false;
    crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| injury_daily_update(this)));
    INJURY_DAILY_DRY_RUN = false;
    let mr = crate::comparacao::Memoria::ler(&reg);
    before.escrever();
    if INJURY_DAILY_STOPPED {
        crate::comparacao::ramo("cpw_9 lesao nova so na original");
        return o(this);
    }
    *core::ptr::addr_of_mut!(INJURY_DAILY_PENDING_REGIONS) = Some((reg, mr));
    o(this);
    // sem a 0x62e010 desviada (desligada), a original não passou pelo ponto de conferência: fica sem comparação
    *core::ptr::addr_of_mut!(INJURY_DAILY_PENDING_REGIONS) = None;
}

/// Comparer for `post_due_recovery_news` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_post_due_recovery_news(this: usize) {
    if let Some((reg, mr)) = (*core::ptr::addr_of_mut!(INJURY_DAILY_PENDING_REGIONS)).take() {
        let mo = crate::comparacao::Memoria::ler(&reg);
        PLACAR.registrar(mo == mr, || format!("injury_daily_update({:x}) {:x?} x {:x?}", this, mo, mr));
    }
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_POST_DUE_RECOVERY_NEWS)(this)
}

/// Comparer for `apply_match_effects` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado no estado do sorteio, no cursor da lista das batidas e na recuperação (+0xa, 4 bytes, do
/// registro de 31 bytes em [this]) de cada pessoa das vagas. Partida com batida que chega ao tipo da lesão: só a original (a lesão nova aloca e manda a
/// notícia; a gravidade da primeira batida pode ser sobra de pilha).
pub(crate) unsafe extern "thiscall" fn comparar_apply_match_effects(this: usize, jogo: usize, partida: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_APPLY_MATCH_EFFECTS);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    let mut batida = false;
    for lado in 0..2usize {
        for k in 0..0x14usize {
            let slot = partida + 0x1c0 + 0x625 * lado + 0x49 * k;
            let Some((id, pessoa)) = staff_of_match_slot(slot) else { continue };
            if pessoa == 0 {
                return o(this, jogo, partida);
            }
            reg.push(((read_i32(this) as u32).wrapping_add((id as u32).wrapping_mul(31)).wrapping_add(0xa) as usize, 4));
            if *((slot - 0x11) as *const u8) != 0 && read_i32(this + 0x28) != 0 && has_injury_hit(this, id, pessoa) {
                batida = true;
            }
        }
    }
    if batida {
        crate::comparacao::ramo("apply_match_effects com batida");
        return o(this, jogo, partida);
    }
    if read_i32(this + 0x28) != 0 {
        reg.push((read_i32(this + 0x28) as usize + 0xa, 4)); // o cursor da lista das batidas
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, jogo, partida), || crate::aleatorio::sem_contar(|| apply_match_effects(this, jogo, partida)));
    PLACAR.registrar(igual, || format!("apply_match_effects({:x}, {:x}) {:x?} x {:x?}", jogo, partida, mo, mr));
}

/// Comparer for `find_match_injury` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado no cursor da lista e na devolução.
pub(crate) unsafe extern "thiscall" fn comparar_find_match_injury(this: usize, person: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_FIND_MATCH_INJURY);
    let list = read_i32(this + 0x28) as usize;
    let reg: &[(usize, usize)] = if list == 0 { &[] } else { &[(list + 0xa, 4)] };
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(reg, || o(this, person), || find_match_injury(this, person));
    PLACAR.registrar(igual && r == n, || format!("find_match_injury({:x}) original={:x} {:x?} rust={:x} {:x?}", person, r, mo, n, mr));
    r
}

/// Comparer for `is_player_on_holiday` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a devolução (só o al).
pub(crate) unsafe extern "thiscall" fn comparar_is_player_on_holiday(this: usize, person: usize) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_IS_PLAYER_ON_HOLIDAY);
    let r = o(this, person);
    let n = is_player_on_holiday(this, person);
    PLACAR.registrar(r == n, || format!("is_player_on_holiday({:x}) original={} rust={}", person, r, n));
    r
}

/// Comparer for `end_player_injury` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado na entrada (tipo) e no +0xe do registro quando nada sai da rotina (férias e sem notícia);
/// com o histórico (arquivo) ou a notícia: só a original, conferindo o plano (tipo 0xff e +0xe −1). Sem pessoa: a
/// original (caixa).
pub(crate) unsafe extern "thiscall" fn comparar_end_player_injury(this: usize, person: usize, noticia: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_END_PLAYER_INJURY);
    if person == 0 || read_i32(person) >= read_i32(G_STAFF_COUNT) {
        return o(this, person, noticia);
    }
    let r = physical_record_of_id(this, read_i32(person));
    let i = read_i32(r + 0xe);
    if i == -1 {
        return o(this, person, noticia);
    }
    let e = (read_i32(this + 4) as u32).wrapping_add((i as u32).wrapping_mul(0x30)) as usize;
    let human: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let clube = read_i32(person + staff::CLUB_JOB) as usize;
    let away = *(e as *const u8) != 0x51 || (noticia != 0 && clube != 0 && human(clube, 0, 1, 0, 0) != 0);
    if away {
        o(this, person, noticia);
        let ok = *(e as *const u8) == 0xff && read_i32(physical_record_of_id(this, read_i32(person)) + 0xe) == -1;
        PLACAR.registrar(ok, || format!("end_player_injury({:x}, {}) plano: tipo {:x} e +0xe {}", person, noticia, *(e as *const u8), read_i32(physical_record_of_id(this, read_i32(person)) + 0xe)));
        return;
    }
    let reg = [(e, 1), (r + 0xe, 4)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, person, noticia), || end_player_injury(this, person, noticia));
    PLACAR.registrar(igual, || format!("end_player_injury({:x}, {}) {:x?} x {:x?}", person, noticia, mo, mr));
}

/// Comparer for `get_player_injury_status` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original (com a forma do dia) e o estado relido pela regra (injury_status, sem rodar a 0x62c680 de
/// novo: a original já a rodou).
pub(crate) unsafe extern "thiscall" fn comparar_get_player_injury_status(this: usize, person: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_GET_PLAYER_INJURY_STATUS)(this, person);
    if person != 0 && (read_u32_as_usize(person) as i32) < read_u32_as_usize(G_STAFF_COUNT) as i32 {
        let n = injury_status(this, person);
        PLACAR.registrar(r == n, || format!("get_player_injury_status(pessoa {:x}) original={} rust={}", person, r, n));
    }
    r
}

/// Comparer for `add_illness_outbreak` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original sorteia, aloca e reordena: o plano antes (os sorteios e a seleção, com o estado do sorteio
/// guardado e devolvido); a original roda. Confere o mesmo estado do sorteio e, se o surto entrou, a conta +1 e o surto
/// (data, tipo, clube, partida) no vetor; senão a conta igual.
pub(crate) unsafe extern "thiscall" fn comparar_add_illness_outbreak(this: usize, data: usize, kind: u32, club_i: i32, partida: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, i32, usize)>(IDX_ADD_ILLNESS_OUTBREAK);
    let state = crate::comparacao::Memoria::ler(&crate::aleatorio::REGIOES_DO_ESTADO);
    let entra = crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| outbreak_takes_hold(kind as u8, club_i)));
    let after_of_plan = crate::comparacao::Memoria::ler(&crate::aleatorio::REGIOES_DO_ESTADO);
    state.escrever();
    let n0 = read_i32(this + 0x24);
    let mut expected = [0u8; OUTBREAK_SIZE];
    expected[..8].copy_from_slice(&*(data as *const [u8; 8]));
    expected[8] = kind as u8;
    expected[9..13].copy_from_slice(&club_i.to_le_bytes());
    let id = if partida != 0 { read_i32(partida) } else { -1 };
    expected[13..17].copy_from_slice(&id.to_le_bytes());
    o(this, data, kind, club_i, partida);
    let n = read_i32(this + 0x24);
    let same_draw = crate::comparacao::Memoria::ler(&crate::aleatorio::REGIOES_DO_ESTADO) == after_of_plan;
    let v = read_usize(this + 0x1c);
    let ok = if entra && v != 0 {
        n == n0 + 1 && (0..n.max(0) as usize).any(|k| *((v + k * OUTBREAK_SIZE) as *const [u8; OUTBREAK_SIZE]) == expected)
    } else {
        n == n0
    };
    PLACAR.registrar(ok && same_draw, || {
        format!("add_illness_outbreak(tipo {}, clube {}) plano entra={}, conta {} -> {}, mesmo sorteio={}", kind as u8, club_i, entra, n0, n, same_draw)
    });
}

/// Comparer for `spread_illness_outbreaks` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original sorteia e adoece (0x62d250, que pode acrescentar surtos): roda só ela e confere o plano
/// feito antes: quantos surtos tinham mais de cinco dias (os que saem) e, depois, nenhum surto com mais de cinco dias e
/// a conta pelo menos a de antes menos os que saem.
pub(crate) unsafe extern "thiscall" fn comparar_spread_illness_outbreaks(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_SPREAD_ILLNESS_OUTBREAKS);
    let cinco = five_days_ago();
    let n0 = read_i32(this + 0x24);
    let hoje4 = read_usize(G_CURRENT_DATE) as u32;
    let mut saem = 0;
    for k in 0..n0.max(0) as usize {
        let e = read_usize(this + 0x1c) + k * OUTBREAK_SIZE;
        let (ano_e, ano_h) = (read_i16(e + 2) as u16, (hoje4 >> 16) as u16);
        if ano_h < ano_e || (ano_h == ano_e && (hoje4 as u16 as i16) < read_i16(e)) {
            break;
        }
        if is_after_outbreak(&cinco, e) {
            saem += 1;
        }
    }
    o(this);
    let n = read_i32(this + 0x24);
    let velho = (0..n.max(0) as usize).find(|&k| is_after_outbreak(&cinco, read_usize(this + 0x1c) + k * OUTBREAK_SIZE));
    PLACAR.registrar(velho.is_none() && n >= n0 - saem, || {
        format!("spread_illness_outbreaks: conta {} -> {} (saem {}), surto velho {:?}", n0, n, saem, velho)
    });
}

/// Comparer for `injury_player_in_close_season` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (datas, a seleção e a temporada só calculam).
pub(crate) unsafe extern "stdcall" fn comparar_injury_player_in_close_season(person: usize) -> u32 {
    let r = original_fn::<unsafe extern "stdcall" fn(usize) -> u32>(IDX_INJURY_PLAYER_IN_CLOSE_SEASON)(person);
    let n = injury_player_in_close_season(person);
    PLACAR.registrar(r == n, || format!("injury_player_in_close_season({:x}) original={} rust={}", person, r, n));
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
            if p != 0 && read_i32(p) < read_i32(G_STAFF_COUNT) && read_usize(p + 0x61) == 0 {
                regions.push((p + 0x39, 5));
            }
        }
    }
    let (_, _, igual, _, _) = crate::comparacao::lado_a_lado(&regions, || o(this), || init_squad_fitness(this));
    PLACAR.registrar(igual, || format!("init_squad_fitness: memória difere ({} regiões)", regions.len()));
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
    PLACAR.registrar(errado.is_none(), || format!("init_records: registro {:?} sem o +0x1e = 50", errado));
}

/// Comparer for `roll_recurring_injury` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a 0x62d250 (tratar) não pode rodar duas vezes: o plano antes, com o estado do sorteio guardado (sem
/// contar nem rastrear) e as marcas do registro devolvidas; a original roda. Confere as marcas do registro e do
/// escolhido e, fora do tratar, o estado do sorteio igual ao do plano (os mesmos sorteios).
pub(crate) unsafe extern "thiscall" fn comparar_roll_recurring_injury(this: usize, record: usize, lesao: *const u8) {
    let state = crate::comparacao::Memoria::ler(&crate::aleatorio::REGIOES_DO_ESTADO);
    let marks = (read_u8(record + 0x12), read_u8(record + 0x13));
    let plan = crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| plan_recurring_injury(this, record, lesao)));
    let after_of_plan = crate::comparacao::Memoria::ler(&crate::aleatorio::REGIOES_DO_ESTADO);
    state.escrever();
    *((record + 0x12) as *mut u8) = marks.0;
    *((record + 0x13) as *mut u8) = marks.1;
    let kind = *lesao;
    original_fn::<unsafe extern "thiscall" fn(usize, usize, *const u8)>(IDX_ROLL_RECURRING_INJURY)(this, record, lesao);
    let now = (read_u8(record + 0x12), read_u8(record + 0x13));
    let same_draw = crate::comparacao::Memoria::ler(&crate::aleatorio::REGIOES_DO_ESTADO) == after_of_plan;
    let ok = match plan {
        RecurringInjuryOutcome::Nada => now == marks && same_draw,
        RecurringInjuryOutcome::Marcada => now == (kind, 0) && same_draw,
        RecurringInjuryOutcome::Limpar(r) => {
            (read_u8(r + 0x12), read_u8(r + 0x13)) == (0xff, 0) && (r == record || now == (kind, 0)) && same_draw
        }
        RecurringInjuryOutcome::Tratar(_) => true, // a 0x62d250 sorteia e grava o resto
    };
    PLACAR.registrar(ok, || {
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
    let (a, b, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(r + 0x14, 8)], || o(this, person), || reset_player_history(this, person));
    PLACAR.registrar(igual && a == b, || format!("reset_player_history({}) {:x?} x {:x?}", read_usize(person), mo, mr));
    a
}

/// Comparer for `injury_reset_player` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a 0x68d1f0 libera o item tirado e a 0x62e700 grava o histórico no arquivo: o plano antes (os itens
/// que ficam, o byte do registro de 79 bytes, os bytes do registro que não mudam); a original roda; confere.
pub(crate) unsafe extern "thiscall" fn comparar_injury_reset_player(this: usize, id: u32) -> usize {
    let l = read_usize(this + 0x28);
    let expected: Vec<usize> = if l != 0 { list_items_capped(l).into_iter().filter(|&e| read_usize(e) as u32 != id).collect() } else { Vec::new() };
    let r = injury_history_record(this, id as usize);
    let g = read_usize(G_STAFF_STATE_TABLE).wrapping_add((id as usize).wrapping_mul(79)) + 0xc;
    let (byte, nivel, b1c) = (read_u8(g) & 0xfe, read_u8(r + 0x13), read_u8(r + 0x1c));
    let ret = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(IDX_INJURY_RESET_PLAYER)(this, id);
    let itens = if l != 0 { list_items_capped(l) } else { Vec::new() };
    let w = |o: usize| core::ptr::read_unaligned((r + o) as *const i16);
    let dd = |o: usize| read_usize(r + o) as i32;
    let ok = itens == expected && ret == g && read_u8(g) == byte
        && read_usize(r) == read_usize(G_CURRENT_DATE) && read_usize(r + 4) == read_usize(G_CURRENT_DATE_LEAP)
        && w(8) == 10000 && w(0xa) == 10000 && w(0xc) == 0 && dd(0xe) == -1 && read_u8(r + 0x12) == 0xff
        && read_u8(r + 0x13) == nivel && dd(0x14) == -1 && dd(0x18) == -1 && read_u8(r + 0x1c) == b1c && read_u8(r + 0x1d) == 0xff;
    PLACAR.registrar(ok, || {
        let v: Vec<u8> = (0..31).map(|o| read_u8(r + o)).collect();
        format!("injury_reset_player({}) itens {} (esperava {}), byte {:#x} (esperava {:#x}), registro {:02x?}", id, itens.len(), expected.len(), read_u8(g), byte, v)
    });
    ret
}

/// Comparer for `recover_injury` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_recover_injury(t: usize, p: *const i32, mark: i16, ritmo: i16) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *const i32, i16, i16)>(IDX_RECOVER_INJURY);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    if !p.is_null() && *p < *(G_STAFF_COUNT as *const i32) {
        reg.push(((*(t as *const usize)).wrapping_add((*p as usize).wrapping_mul(31)).wrapping_add(0xa), 4));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(t, p, mark, ritmo), || crate::aleatorio::sem_contar(|| recover_injury(t, p, mark, ritmo)));
    PLACAR.registrar(igual, || format!("recover_injury({:#x}, {:p}, {}, {}) {:x?} x {:x?}", t, p, mark, ritmo, mo, mr));
}

/// Comparer for `get_condition` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (a 0x62c400 só lê).
pub(crate) unsafe extern "thiscall" fn comparar_get_condition(this: usize, player: usize, em_porcento: u32) -> u16 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u16>(IDX_GET_CONDITION)(this, player, em_porcento);
    let n = get_condition(this, player, em_porcento);
    PLACAR.registrar(r == n, || format!("get_condition({:x}, {}) original={} rust={}", player, em_porcento, r, n));
    r
}

/// Comparer for `set_training_load` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_set_training_load(lesoes: usize, id: *const i32, value: u8) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *const i32, u8)>(IDX_SET_TRAINING_LOAD);
    let a = training_load_byte(lesoes, id) as usize;
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(a, 1)], || o(lesoes, id, value), || set_training_load(lesoes, id, value));
    PLACAR.registrar(igual, || format!("set_training_load({}, {}) {:x?} x {:x?}", *id, value, mo, mr));
}

/// Comparer for `update_player_fitness` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador lado a lado: o registro de 31 bytes da pessoa, o byte +0xa do registro de 79 bytes, as estáticas e o
/// guardado do clube (0xb63d08..0xb63d34) e o sorteio. A cópia roda em ensaio, sem o fim da lesão (0x62e700): quando
/// ela chega lá, fica a original sozinha (já rodou primeiro). Na primeira vez (estáticas por criar, com atexit) e sem
/// registro (pessoa nula, id fora da tabela, sem ficha): só a original.
pub(crate) unsafe extern "thiscall" fn comparar_update_player_fitness(this: usize, person: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_UPDATE_PLAYER_FITNESS);
    if *(G_FITNESS_UPDATE_STATIC_INIT_FLAGS as *const u8) & 7 != 7 || person == 0 || read_i32(person) < 0 || read_i32(person) >= read_i32(G_STAFF_COUNT) || read_usize(person + staff::PLAYER) == 0 {
        return o(this, person); // simplification: as estáticas e o atexit da primeira vez não podem rodar duas vezes
    }
    let id = read_i32(person) as usize;
    let rec = read_usize(this).wrapping_add(id.wrapping_mul(31));
    let geral = read_usize(G_STAFF_STATE_TABLE).wrapping_add(id.wrapping_mul(0x4f));
    let e = crate::aleatorio::REGIOES_DO_ESTADO;
    let reg = [(rec, 31), (geral + 0xa, 1), (G_FITNESS_UPDATE_CACHE_DATE, 0x2c), e[0], e[1]];
    END_INJURY_REACHED = false;
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, person), || {
        END_INJURY_DRY_RUN = true;
        crate::aleatorio::sem_contar(|| update_player_fitness(this, person));
        END_INJURY_DRY_RUN = false;
    });
    if END_INJURY_REACHED {
        crate::comparacao::ramo("cpw_8 fim da lesao so na original");
        return;
    }
    PLACAR.registrar(igual, || format!("update_player_fitness(pessoa {:x}) {:x?} x {:x?}", person, mo, mr));
}

/// Comparer for `daily_wear` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_daily_wear(t: usize, id: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32)>(IDX_DAILY_WEAR);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    if id < *(G_STAFF_COUNT as *const i32) {
        reg.push(((*(t as *const usize)).wrapping_add((id as usize).wrapping_mul(31)).wrapping_add(0xc), 2));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(t, id), || crate::aleatorio::sem_contar(|| daily_wear(t, id)));
    PLACAR.registrar(igual, || format!("daily_wear({:#x}, {}) {:x?} x {:x?}", t, id, mo, mr));
}

/// Comparer for `choose_injury_type` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_choose_injury_type(lesoes: usize, category: i8, gravidade: i8, player: usize) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8, i8, usize) -> u8>(IDX_CHOOSE_INJURY_TYPE);
    // simplification: jogador nulo abre a caixa de erro do 0x62ed70; não roda duas vezes
    if player == 0 {
        return o(lesoes, category, gravidade, player);
    }
    // simplification: a 0x62ed70 (primeiro efeito da 0x62d140, antes de qualquer sorteio dela) passa pela
    // 0x62c680, que sorteia só na primeira consulta do dia e guarda em cache; chamá-la aqui antes enche o
    // cache na mesma ordem da original, e os dois lados do lado_a_lado acham o cache cheio
    let current: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY);
    current(lesoes, player);
    let (r, n, igual, _, _) = crate::comparacao::lado_a_lado(&crate::aleatorio::REGIOES_DO_ESTADO,
        || o(lesoes, category, gravidade, player), || crate::aleatorio::sem_contar(|| choose_injury_type(lesoes, category, gravidade, player)));
    PLACAR.registrar(igual && r == n, || format!("choose_injury_type({}, {}) original={} rust={} sorteio igual={}", category, gravidade, r, n, igual));
    r
}

/// Comparer for `injury_compare_entry_dates` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_injury_compare_entry_dates(key: usize, item: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_INJURY_COMPARE_ENTRY_DATES)(key, item);
    let n = injury_compare_entry_dates(key, item);
    PLACAR.registrar(r == n, || format!("injury_compare_entry_dates(chave {:x}, item {:x}) original={} rust={}", key, item, r, n));
    r
}

/// Comparador: o al devolvido.
pub(crate) unsafe extern "thiscall" fn comparar_get_recurring_injury_type(this: usize, person: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_GET_RECURRING_INJURY_TYPE)(this, person);
    let n = get_recurring_injury_type(this, person);
    PLACAR.registrar(r == n, || format!("get_recurring_injury_type(pessoa {:x}) original={:x} rust={:x}", person, r, n));
    r
}

/// Comparer for `injury_severity` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_injury_severity(lesoes: usize, kind: i8) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, i8) -> u8>(IDX_INJURY_SEVERITY)(lesoes, kind);
    let n = injury_severity(lesoes, kind);
    PLACAR.registrar(r == n, || format!("injury_severity({}) original={} rust={}", kind, r, n));
    r
}

/// Comparer for `injury_grade` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_injury_grade(lesoes: usize, person: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_INJURY_GRADE)(lesoes, person);
    let n = injury_grade(lesoes, person);
    PLACAR.registrar(r == n, || format!("injury_grade() original={} rust={}", r, n));
    r
}
