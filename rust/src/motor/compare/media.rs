use super::*;

// The comparers of the routines in ../media.rs.
// [pt] Os comparadores das rotinas de ../media.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `process_transfer_interest`.
pub(crate) unsafe extern "thiscall" fn compare_process_transfer_interest(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_PROCESS_TRANSFER_INTEREST)(this)
}

/// Comparer for `news_create_not_for_sale_story`.
pub(crate) unsafe extern "C" fn compare_news_create_not_for_sale_story(player: usize, club: usize, chairman: usize, other: usize) {
    original_fn::<unsafe extern "C" fn(usize, usize, usize, usize)>(IDX_NEWS_CREATE_NOT_FOR_SALE_STORY)(player, club, chairman, other)
}

/// Comparer for `queue_trophy_praise_story`.
pub(crate) unsafe extern "thiscall" fn compare_queue_trophy_praise_story(this: usize, club: usize, comp: usize, mark: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(IDX_QUEUE_TROPHY_PRAISE_STORY)(this, club, comp, mark)
}

/// Comparer for `apply_squad_reaction_to_player_story`.
///
/// Through the tape of the post-match gates (compare_pos): the disaffections (`exe::staff_add_disliked_staff`), the mark (`set_flags`) and the
/// complaints (`exe::staff_contract_apply_event_flags`) run once, in the original, and the copy checks the call and receives the state afterwards.
/// Regions: the random generator, [0xae1080], the six persons of the club (+0x9b, `club_add_disliked_staff`), the player's contract and,
/// for each person of the array, the preferences (+0x1c..+0x33), +0x58..+0x5d and the club contract (+0 to +0x4f).
///
/// [pt] Comparador: pela fita das portas do pós-jogo (compare_pos): as desafeições (0x547af0), a marca (0x867560) e as
/// queixas (0x867700) rodam uma vez, na original, e a cópia confere a chamada e recebe o estado de depois. Regiões: o
/// sorteador, [0xae1080], as seis pessoas do clube (+0x9b, a 0x547940), o contrato do jogador e, de cada pessoa do
/// vetor, as preferências (+0x1c..+0x33), o +0x58..+0x5d e o contrato no clube (+0 a +0x4f).
pub(crate) unsafe extern "thiscall" fn compare_apply_squad_reaction_to_player_story(this: usize, topic: u32, player: usize, club: usize, ids: usize, n: u32) {
    let club = Club(club);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize, usize, usize, u32)>(IDX_APPLY_SQUAD_REACTION_TO_PLAYER_STORY);
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.push((G_CONTRACT_LOOKUP_RESULT, 4));
    if !club.is_null() && player != 0 {
        reg.push((club.addr() + club::FAV_STAFF1, 24));
        reg.extend(contract_region(player, club));
        for &p in club_staff_vector(club, &mut |r, t, a| dispatch_post_match_call(r, t, a)).iter().take(57) {
            if p == 0 {
                continue;
            }
            let pref = read_usize(p + staff::STAFF_PREFERENCES);
            if pref != 0 {
                reg.push((pref + preferences::STAFF_FAVOURITE_STAFF1, 24));
            }
            reg.push((p + staff::DETERMINATION, 6));
            reg.extend(contract_region(p, club));
        }
    }
    compare_pos("apply_squad_reaction_to_player_story", reg, || o(this, topic, player, club.addr(), ids, n),
        |chamar| squad_reaction_to_story_with(topic, Staff(player), club, ids, n, chamar));
}

/// Comparer for `warn_squad_of_position_rival`.
///
/// [pt] Comparador: pela fita das portas do pós-jogo (a história 0x72c1f0 e a marca 0x867560 rodam só na original). Regiões:
/// o sorteador, [0xae1080] e, de cada um dos 50 do elenco, o +0x58..+0x5d e o contrato no clube (+0 a +0x4f).
pub(crate) unsafe extern "thiscall" fn compare_warn_squad_of_position_rival(this: usize, player: usize, club: usize, a: u32, b: u32) {
    let club = Club(club);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(IDX_WARN_SQUAD_OF_POSITION_RIVAL);
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.push((G_CONTRACT_LOOKUP_RESULT, 4));
    if !club.is_null() {
        for i in 0..50 {
            let p = Staff(club.squad(i));
            if !p.is_null() {
                reg.push((p.addr() + staff::DETERMINATION, 6));
                reg.extend(contract_region(p.addr(), club));
            }
        }
    }
    compare_pos("warn_squad_of_position_rival", reg, || o(this, player, club.addr(), a, b), |chamar| warn_position_rivals_with(Staff(player), club, chamar));
}

/// Comparer for `media_format_response_option_label`.
///
/// [pt] Comparador: lado a lado nos primeiros bytes da saída (até 0x50, o maior rótulo tem 61).
pub(crate) unsafe extern "stdcall" fn compare_media_format_response_option_label(kind: u32, option: u32, out: usize, size: u32) {
    let o = original_fn::<unsafe extern "stdcall" fn(u32, u32, usize, u32)>(IDX_MEDIA_FORMAT_RESPONSE_OPTION_LABEL);
    let reg = if out != 0 && (size as i16) >= 0xf { vec![(out, (size as u16 as usize).min(0x50))] } else { Vec::new() };
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(kind, option, out, size), || media_format_response_option_label(kind, option, out, size));
    SCOREBOARD.record(igual, || format!("media_format_response_option_label({}, {}) {:x?} x {:x?}", kind as i16, option as i8, mo, mr));
}

/// Comparer for `process_transfer_squad_reactions` (comparison mode: see the header of motor.rs): a dry-run comparer.
///
/// The copy runs without news or transfer request (it stops when it reaches one) and the original routine from the same
/// state: the random draw, [`0xae1080`] (the player sheet of the last contract), the static date of `0x54cdb0`, the new
/// club's supporters row in [[`0xb67a44`]] and, for each member of the two clubs' squads (and the person of the media
/// story), the contract with the club (0x50 bytes) and the opinions row in [`0xae2d0c`] (0x4f bytes). When the copy
/// stops: only the original routine.
///
/// [pt] Comparador em ensaio: a cópia sem notícia nem pedido de transferência (para quando chega a um) e a original a partir
/// do mesmo estado: o sorteio, [0xae1080] (a ficha do último contrato), a data estática da 0x54cdb0, a linha da torcida
/// do novo clube em [[0xb67a44]] e, de cada um dos elencos dos dois clubes (e da pessoa da história da mídia), o contrato
/// com o clube (0x50 bytes) e a linha de opiniões em [0xae2d0c] (0x4f bytes). Quando a cópia para: só a original.
pub(crate) unsafe extern "thiscall" fn compare_process_transfer_squad_reactions(this: usize, p: usize, new: usize, old: usize, flag: u32) {
    let p = Staff(p);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32)>(IDX_PROCESS_TRANSFER_SQUAD_REACTIONS);
    if p.is_null() || new == 0 || p.player().is_null() || ((p.age() as i8) as i32) < 0x13 || current_reputation(p) < 0x109a {
        return o(this, p.addr(), new, old, flag);
    }
    let sheet = read_usize(G_CONTRACT_LOOKUP_RESULT);
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.push((G_CONTRACT_LOOKUP_RESULT, 4));
    reg.push((G_DAYS_SINCE_SCRATCH_DATE, 8));
    if read_usize(G_MANAGER_MANAGER) != 0 {
        reg.push((read_usize(read_usize(G_MANAGER_MANAGER)).wrapping_add((read_i32(new) as u32).wrapping_mul(0x49) as usize), 0x49));
    }
    let mut join = |m: usize, club: usize| {
        if m == 0 {
            return;
        }
        let c = media_contract_for_club(m, Club(club));
        if c != 0 {
            reg.push((c, 0x50));
        }
        reg.push((read_usize(G_STAFF_STATE_TABLE).wrapping_add((read_i32(m) as u32).wrapping_mul(0x4f) as usize), 0x4f));
    };
    for club in [new, old] {
        if club != 0 {
            for j in 0..50usize {
                join(read_usize(club + club::SQUAD + 4 * j), club);
            }
        }
    }
    if read_i32(this + media_fields::STORY_LOG_COUNT) > 0 && read_i32(read_usize(this) + 8) != -1 {
        join(media_staff_by_id(read_i32(read_usize(this) + 8)), new);
    }
    write_usize(G_CONTRACT_LOOKUP_RESULT, sheet); // the contracts read above do not count
    let before = crate::comparison::MemorySnapshot::read(&reg);
    MEDIA_REACTION_DRY_RUN = true;
    MEDIA_REACTION_STOPPED = false;
    crate::rng::pause_trace(|| crate::rng::without_counting(|| process_transfer_squad_reactions(this, p.addr(), new, old, flag)));
    MEDIA_REACTION_DRY_RUN = false;
    let mr = crate::comparison::MemorySnapshot::read(&reg);
    before.write_back();
    if MEDIA_REACTION_STOPPED {
        crate::comparison::branch("cqa_13 reactions only in the original");
        return o(this, p.addr(), new, old, flag);
    }
    o(this, p.addr(), new, old, flag);
    let mo = crate::comparison::MemorySnapshot::read(&reg);
    SCOREBOARD.record(mo == mr, || {
        let dif: Vec<usize> = mo.0.iter().zip(mr.0.iter()).flat_map(|(a, b)| (0..a.1.len()).filter(move |&i| a.1[i] != b.1[i]).map(move |i| a.0 + i)).take(16).collect();
        format!("process_transfer_squad_reactions(jogador {:#x}, novo {:#x}, velho {:#x}) diferencas em {:x?}", p.addr(), new, old, dif)
    });
}

/// Comparer for `queue_signing_celebration_story`.
pub(crate) unsafe extern "thiscall" fn compare_queue_signing_celebration_story(this: usize, p: usize, club: usize, kind: u32, a: u32, porta_voz: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, usize)>(IDX_QUEUE_SIGNING_CELEBRATION_STORY)(this, p, club, kind, a, porta_voz)
}

/// Comparer for `queue_sale_unrest_story`.
pub(crate) unsafe extern "thiscall" fn compare_queue_sale_unrest_story(this: usize, old: usize, new: usize, p: usize, comp: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize)>(IDX_QUEUE_SALE_UNREST_STORY)(this, old, new, p, comp)
}

/// Comparer for `media_pick_outlet_type_for_club`.
pub(crate) unsafe extern "C" fn compare_media_pick_outlet_type_for_club(club: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_MEDIA_PICK_OUTLET_TYPE_FOR_CLUB);
    let (r, n, igual, _, _) = crate::comparison::side_by_side_light(&crate::rng::STATE_REGIONS, || o(club), || crate::rng::without_counting(|| media_pick_outlet_type_for_club(club)));
    SCOREBOARD.record(igual && r == n, || format!("media_pick_outlet_type_for_club({:x}) original={} rust={} sorteio igual={}", club, r, n, igual));
    r
}

/// Comparer for `media_pick_outlet_type_for_person`.
pub(crate) unsafe extern "C" fn compare_media_pick_outlet_type_for_person(person: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_MEDIA_PICK_OUTLET_TYPE_FOR_PERSON);
    let (r, n, igual, _, _) = crate::comparison::side_by_side_light(&crate::rng::STATE_REGIONS, || o(person), || crate::rng::without_counting(|| media_pick_outlet_type_for_person(person)));
    SCOREBOARD.record(igual && r == n, || format!("media_pick_outlet_type_for_person({:x}) original={} rust={} sorteio igual={}", person, r, n, igual));
    r
}

/// Comparer for `adjust_supporter_confidence`.
///
/// [pt] Comparador: a original sorteia e entrega a notícia (não pode rodar duas vezes): roda só ela e confere o plano feito
/// antes: a confiança da torcida nova (a variação é pura).
pub(crate) unsafe extern "thiscall" fn compare_adjust_supporter_confidence(this: usize, club: usize, sujeito: usize, topic: u32, target: u32, variant: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32)>(IDX_ADJUST_SUPPORTER_CONFIDENCE);
    if club == 0 {
        return o(this, club, sujeito, topic, target, variant);
    }
    let p = manager_manager_row(Club(club)) + 8;
    let before = read_i16(p);
    let v = fan_confidence_change(Club(club), target);
    let expected = before.wrapping_add(v).clamp(1, 10000);
    o(this, club, sujeito, topic, target, variant);
    let after = read_i16(manager_manager_row(Club(club)) + 8);
    SCOREBOARD.record(after == expected, || format!("adjust_supporter_confidence(clube {:x}, alvo {}) {} + {} = {} x original {}", club, target as i16, before, v, expected, after));
}

/// Comparer for `adjust_board_confidence`.
///
/// [pt] Comparador: como o da torcida, pela confiança da diretoria nova.
pub(crate) unsafe extern "thiscall" fn compare_adjust_board_confidence(this: usize, club: usize, sujeito: usize, topic: u32, target: u32, variant: u32, sexto: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32)>(IDX_ADJUST_BOARD_CONFIDENCE);
    if club == 0 {
        return o(this, club, sujeito, topic, target, variant, sexto);
    }
    let p = manager_manager_row(Club(club)) + 6;
    let before = read_i16(p);
    let v = board_confidence_change(Club(club), target);
    let expected = before.wrapping_add(v).clamp(1, 10000);
    o(this, club, sujeito, topic, target, variant, sexto);
    let after = read_i16(manager_manager_row(Club(club)) + 6);
    SCOREBOARD.record(after == expected, || format!("adjust_board_confidence(clube {:x}, alvo {}) {} + {} = {} x original {}", club, target as i32, before, v, expected, after));
}

/// Comparer for `queue_player_gratitude_news`.
///
/// [pt] Comparador: a gratidão sorteia, sobe +0x44 da ficha e entrega a notícia: só a original (a prova é a carreira e o
/// modo Rust).
pub(crate) unsafe extern "thiscall" fn compare_queue_player_gratitude_news(this: usize, player: usize, club: usize, kind: u32, variant: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(IDX_QUEUE_PLAYER_GRATITUDE_NEWS)(this, player, club, kind, variant)
}

/// Comparer for `post_player_backing_request_news`.
///
/// [pt] Comparador: a notícia vai à caixa do técnico (não pode rodar duas vezes): só a original.
pub(crate) unsafe extern "thiscall" fn compare_post_player_backing_request_news(this: usize, player: usize, club: usize, kind: u32, quarto: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(IDX_POST_PLAYER_BACKING_REQUEST_NEWS)(this, player, club, kind, quarto)
}

/// Comparer for `post_player_criticism_reaction_news`.
///
/// [pt] Comparador: a notícia vai às caixas dos técnicos (não pode rodar duas vezes): só a original.
pub(crate) unsafe extern "thiscall" fn compare_post_player_criticism_reaction_news(this: usize, player: usize, club: usize, topic: u32, reply: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(IDX_POST_PLAYER_CRITICISM_REACTION_NEWS)(this, player, club, topic, reply)
}

/// Comparer for `rate_player_form_for_story`.
///
/// [pt] Comparador: a nota é pura (as estatísticas, as datas e o contrato só leem; nada sorteia): compara a devolução bit a
/// bit (a original e a cópia na mesma precisão dupla do x87).
pub(crate) unsafe extern "thiscall" fn compare_rate_player_form_for_story(this: usize, player: usize, club: usize, contract: usize, kind: u32, mode: u32, weight: f32) -> f64 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, f32) -> f64>(IDX_RATE_PLAYER_FORM_FOR_STORY)(this, player, club, contract, kind, mode, weight);
    let n = rate_player_form_for_story(this, player, club, contract, kind, mode, weight);
    SCOREBOARD.record(o.to_bits() == n.to_bits() || (o.is_nan() && n.is_nan()),
        || format!("rate_player_form_for_story(tipo {}, modo {}, peso {}) {} x {}", kind as u16 as i16, mode as u8, weight, o, n));
    o
}

/// Comparer for `write_financial_trouble_news_text`.
///
/// [pt] Comparador: o texto vai para o objeto de saída (0x4ae660/0x4ae8a0 acrescentam e alocam): roda só a original (a cópia
/// prova no modo Rust, pelo ramo).
pub(crate) unsafe extern "thiscall" fn compare_write_financial_trouble_news_text(this: usize, news: usize, body: u32, out: usize) {
    // simplification: the text object cannot receive the text twice; only the original runs here
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(IDX_WRITE_FINANCIAL_TROUBLE_NEWS_TEXT)(this, news, body, out);
}

/// Comparer for `log_story`.
///
/// [pt] Comparador: a original pode alocar e realocar (não roda duas vezes): o registro esperado antes; confere a conta e
/// que o registro esperado está na lista (a ordem é a do qsort).
pub(crate) unsafe extern "thiscall" fn compare_log_story(this: usize, a: usize, b: usize, kind: u32, subtype: u32, reply: u32) {
    let n = read_usize(this + media_fields::STORY_LOG_COUNT);
    let expected = build_news_log_record(a, b, kind, subtype, reply);
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32)>(IDX_LOG_STORY)(this, a, b, kind, subtype, reply);
    let m = read_usize(this + media_fields::STORY_LOG_COUNT);
    let found = (0..m).any(|k| {
        core::slice::from_raw_parts(read_usize(this).wrapping_add(k * 0x17) as *const u8, 0x17) == &expected[..]
    });
    SCOREBOARD.record(m == n.wrapping_add(1) && found, || {
        format!("log_story(tipo {:#x}, subtipo {}) conta {} -> {}, esperado {:x?} achou {}", kind, subtype as u8 as i8, n, m, expected, found)
    });
}

/// Comparer for `expire_story_log`.
///
/// [pt] Comparador: lado a lado pelo registro (ponteiro, capacidade e conta; as histórias) e o estado do sorteio (a cópia
/// sem contar os sorteios).
pub(crate) unsafe extern "thiscall" fn compare_expire_story_log(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_EXPIRE_STORY_LOG);
    if read_usize(this) == 0 {
        return o(this);
    }
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.push((this, 0xc));
    reg.push((read_usize(this), (read_usize(this + media_fields::STORY_LOG_COUNT) as i32).max(0) as usize * 0x17));
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this), || crate::rng::without_counting(|| expire_story_log(this)));
    // mo.0[2] and mr.0[2]: pointer, capacity and count after each side
    SCOREBOARD.record(igual, || format!("expire_story_log {:x?} x {:x?}", mo.0[2].1, mr.0[2].1));
}

/// Comparer for `has_logged_story`.
///
/// [pt] Comparador: pelo valor (a busca e a data padrão da chave só leem).
pub(crate) unsafe extern "thiscall" fn compare_has_logged_story(this: usize, a: usize, b: usize, kind: u32, subtype: u32, desde: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, usize) -> u32>(IDX_HAS_LOGGED_STORY)(this, a, b, kind, subtype, desde);
    let n = has_logged_story(this, a, b, kind, subtype, desde);
    SCOREBOARD.record(r == n, || format!("has_logged_story(tipo {:#x}, subtipo {}, desde {}) original={} rust={}", kind, subtype as u16 as i16, desde != 0, r, n));
    r
}

/// Comparer for `add_transfer_interest`.
///
/// [pt] Comparador: a original pode alocar e realocar (não roda duas vezes): o plano antes (já está? o registro novo);
/// confere a conta e que o registro novo está no vetor (a ordem é a do qsort).
pub(crate) unsafe extern "thiscall" fn compare_add_transfer_interest(this: usize, player: usize, club: usize, value: i32, avaliacao: f64) {
    let n = read_usize(this + media_fields::INTEREST_COUNT);
    let ja_esta = has_interest(this, &interest_key(player, club));
    let expected = if ja_esta { None } else { Some(build_interest_record(player, club, value, avaliacao)) };
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, i32, f64)>(IDX_ADD_TRANSFER_INTEREST)(this, player, club, value, avaliacao);
    let m = read_usize(this + media_fields::INTEREST_COUNT);
    let ok = match &expected {
        None => m == n,
        Some(e) => m == n.wrapping_add(1) && (0..m).any(|k| {
            core::slice::from_raw_parts(read_usize(this + media_fields::TRANSFER_INTERESTS).wrapping_add(k * 0x19) as *const u8, 0x19) == &e[..]
        }),
    };
    SCOREBOARD.record(ok, || format!("add_transfer_interest(valor {}, avaliacao {}) conta {} -> {}, esperado {:x?}", value, avaliacao, n, m, expected));
}

/// Comparer for `compare_id_pair_keys`.
///
/// [pt] Comparador: pelo valor (só lê).
pub(crate) unsafe extern "C" fn compare_compare_id_pair_keys(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_COMPARE_ID_PAIR_KEYS)(a, b);
    let n = compare_id_pair_keys(a, b);
    SCOREBOARD.record(r == n, || format!("compare_id_pair_keys({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `compare_story_log_records`.
///
/// [pt] Comparador: pelo valor (só lê).
pub(crate) unsafe extern "C" fn compare_compare_story_log_records(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_COMPARE_STORY_LOG_RECORDS)(a, b);
    let n = compare_story_log_records(a, b);
    SCOREBOARD.record(r == n, || format!("compare_story_log_records({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `remove_transfer_interest`.
pub(crate) unsafe extern "thiscall" fn compare_remove_transfer_interest(this: usize, person: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_REMOVE_TRANSFER_INTEREST);
    let n = read_i32(this + media_fields::INTEREST_COUNT).max(0) as usize;
    let reg = [(this + media_fields::TRANSFER_INTERESTS, 0xc), (read_u32(this + media_fields::TRANSFER_INTERESTS) as usize, n * 25)];
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, person), || remove_transfer_interest(this, person));
    SCOREBOARD.record(igual, || format!("remove_transfer_interest({:x}) {:x?} x {:x?}", person, mo, mr));
}

/// Comparer for `get_finable_story_kind`.
pub(crate) unsafe extern "thiscall" fn compare_get_finable_story_kind(average: usize, player: usize) -> u16 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u16>(IDX_GET_FINABLE_STORY_KIND)(average, player);
    let n = get_finable_story_kind(average, player);
    SCOREBOARD.record(r == n, || format!("get_finable_story_kind original={} rust={}", r, n));
    r
}

/// Comparer for `story_with_response_two`.
pub(crate) unsafe extern "thiscall" fn compare_story_with_response_two(average: usize, id: *const i32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, *const i32) -> u32>(IDX_STORY_WITH_RESPONSE_TWO)(average, id);
    let n = story_with_response_two(average, id);
    SCOREBOARD.record(r == n, || format!("story_with_response_two({}) original={} rust={}", *id, r, n));
    r
}

/// Comparer for `has_story`.
pub(crate) unsafe extern "thiscall" fn compare_has_story(average: usize, id: *const i32) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, *const i32) -> u8>(IDX_HAS_STORY)(average, id);
    let n = has_story(average, id);
    SCOREBOARD.record(r == n, || format!("has_story({}) original={} rust={}", *id, r, n));
    r
}

/// Comparer for `media_queue_player_apology_news`.
///
/// [pt] Comparador: a notícia sorteia e vai para a caixa do técnico: só a original (a prova é a carreira e o modo Rust).
pub(crate) unsafe extern "stdcall" fn compare_media_queue_player_apology_news(player: usize, club: usize, historia: i16) {
    original_fn::<unsafe extern "stdcall" fn(usize, usize, i16)>(IDX_MEDIA_QUEUE_PLAYER_APOLOGY_NEWS)(player, club, historia)
}

/// Comparer for `pick_former_international_pundit`.
pub(crate) unsafe extern "thiscall" fn compare_pick_former_international_pundit(this: usize, club: usize, subject: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> usize>(IDX_PICK_FORMER_INTERNATIONAL_PUNDIT);
    if subject == 0 && club == 0 {
        return o(this, club, subject);
    }
    // the commentator comes only from the random roll
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&crate::rng::STATE_REGIONS, || o(this, club, subject),
        || crate::rng::without_counting(|| pick_former_international_pundit(this, club, subject)));
    SCOREBOARD.record(igual && r == n, || format!("pick_former_international_pundit {:#x} x {:#x} {:x?} x {:x?}", r, n, mo, mr));
    r
}

/// Comparer for `pick_ai_manager_response`.
///
/// [pt] Comparador: a resposta sorteia e pode pôr o jogador entre os rejeitados do técnico (0x547af0): lado a lado no
/// estado do gerador e no bloco das preferências do técnico (+0x65, +0x1c..+0x34), a cópia sem contar os sorteios.
pub(crate) unsafe extern "thiscall" fn compare_pick_ai_manager_response(this: usize, manager: usize, player: usize, kind: u32) -> u32 {
    let manager = Staff(manager);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32>(IDX_PICK_AI_MANAGER_RESPONSE);
    let mut regions = crate::rng::STATE_REGIONS.to_vec();
    if !manager.is_null() && manager.staff_preferences() != 0 {
        regions.push((manager.staff_preferences() + 0x1c, 0x18));
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(
        &regions,
        || o(this, manager.addr(), player, kind),
        || crate::rng::without_counting(|| pick_ai_manager_response(this, manager.addr(), player, kind)),
    );
    SCOREBOARD.record(igual && r == n, || format!("pick_ai_manager_response(tipo {:#x}) original={} rust={} {:x?} x {:x?}", kind as i16, r, n, mo.0, mr.0));
    r
}

/// Comparer for `post_financial_trouble_news`.
///
/// [pt] Comparador: a notícia sorteia (0x90cfc0) e vai para as caixas dos técnicos: só a original (a prova é a carreira e o
/// modo Rust).
pub(crate) unsafe extern "thiscall" fn compare_post_financial_trouble_news(this: usize, club: usize) {
    crate::comparison::branch("post_financial_trouble_news");
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_POST_FINANCIAL_TROUBLE_NEWS)(this, club)
}

/// Comparer for `pick_rival_manager_id`.
///
/// [pt] Comparador: sorteia (0x90cfc0): lado a lado pelo estado do sorteio, a cópia em sem_contar (as escritas são só nas
/// locais da pilha de cada uma).
pub(crate) unsafe extern "thiscall" fn compare_pick_rival_manager_id(this: usize, club: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_PICK_RIVAL_MANAGER_ID);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&crate::rng::STATE_REGIONS, || o(this, club),
        || crate::rng::without_counting(|| pick_rival_manager_id(this, club)));
    SCOREBOARD.record(igual && r == n, || format!("pick_rival_manager_id({:x}) original={:#x} rust={:#x} {:x?} x {:x?}", club, r, n, mo, mr));
    r
}

/// Comparer for `queue_manager_fa_complaint_story`.
pub(crate) unsafe extern "thiscall" fn compare_queue_manager_fa_complaint_story(this: usize, club: usize, person: usize, kind: u32, a: u32, b: u32, c: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32)>(IDX_QUEUE_MANAGER_FA_COMPLAINT_STORY)(this, club, person, kind, a, b, c)
}
