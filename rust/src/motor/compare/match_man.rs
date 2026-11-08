use super::*;

// The comparers of the routines in ../match_man.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../match_man.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `select_automatically` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_select_automatically(t: usize, outro: usize, modo: u32) -> i8 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> i8>(IDX_SELECT_AUTOMATICALLY)(t, outro, modo)
}

/// Comparer for `apply_squad_rule` (comparison mode: see the header of motor.rs).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "C" fn comparar_apply_squad_rule(
    clube: usize, tecnico: usize, regra: usize, modo: u32, nt: u32, nb: u32, nr: u32, starters: usize, banco: usize, reserva: usize,
    saida: usize, codigos: usize, curtos: usize, marcas: usize, bytes: usize, lado: usize,
) {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize, u32, u32, u32, u32, usize, usize, usize, usize, usize, usize, usize, usize, usize)>(IDX_APPLY_SQUAD_RULE);
    if is_recording_top_level() {
        return o(clube, tecnico, regra, modo, nt, nb, nr, starters, banco, reserva, saida, codigos, curtos, marcas, bytes, lado);
    }
    let n = |v: u32| (v as u8 as i8).max(0) as usize;
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((saida, 4 * (n(nt) + n(nb))));
    if banco != 0 {
        reg.push((banco, 4 * n(nb)));
    }
    if reserva != 0 {
        reg.push((reserva, 4 * n(nr)));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(
        &reg,
        || o(clube, tecnico, regra, modo, nt, nb, nr, starters, banco, reserva, saida, codigos, curtos, marcas, bytes, lado),
        || crate::aleatorio::sem_contar(|| apply_squad_rule(clube, tecnico, regra, modo, nt, nb, nr, starters, banco, reserva, saida, codigos, curtos, marcas, bytes, lado)),
    );
    PLACAR.registrar(igual, || format!("apply_squad_rule(regra {:x?}) {:x?} x {:x?}", core::slice::from_raw_parts(regra as *const u8, 6), mo, mr));
}

/// Comparer for `off_match_score` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_off_match_score(p: usize, pos: u32, flags: u32, clube: usize, limit: i32, o: usize) -> i32 {
    let f = original_fn::<unsafe extern "C" fn(usize, u32, u32, usize, i32, usize) -> i32>(IDX_OFF_MATCH_SCORE);
    if is_recording_top_level() {
        return f(p, pos, flags, clube, limit, o);
    }
    // simplification: sem as regiões do sorteador. A nota não sorteia: o único sorteio vem da 0x62ed70, que
    // atualiza o físico do jogador (0x62c680) só na primeira consulta e guarda; a cópia, depois da
    // original, não o refaz. Devolver o sorteador entre as duas acusava essa diferença de cache.
    let reg = vec![(G_CONTRACT_LOOKUP_RESULT, 4usize)];
    let rastros = core::cell::RefCell::new((Vec::new(), Vec::new()));
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(
        &reg,
        || {
            let (r, v) = crate::aleatorio::rastrear(|| f(p, pos, flags, clube, limit, o));
            rastros.borrow_mut().0 = v;
            r
        },
        || {
            let (r, v) = crate::aleatorio::rastrear(|| crate::aleatorio::sem_contar(|| off_match_score(p, pos, flags, clube, limit, o)));
            rastros.borrow_mut().1 = v;
            r
        },
    );
    PLACAR.registrar(igual && r == n, || format!("off_match_score({:#x}, pos {:#x}, flags {:#x}, limit {}) original={} rust={} sorteios {:?} x {:?} {:x?} x {:x?}", p, pos as u16, flags, limit, r, n, rastros.borrow().0, rastros.borrow().1, mo, mr));
    r
}

/// Comparer for `lineup_slot_score` (comparison mode: see the header of motor.rs).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "C" fn comparar_lineup_slot_score(
    p: usize, f: usize, reg: usize, pos: u32, flags: u32, clube: usize, lado: usize, tecnico: usize, limit: i32, marca: u32, marca_alta: u32,
    bytes: u32, tatica: u32, pos2: u32, ferias: usize,
) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize, u32, u32, usize, usize, usize, i32, u32, u32, u32, u32, u32, usize) -> i32>(IDX_LINEUP_SLOT_SCORE);
    if is_recording_top_level() {
        return o(p, f, reg, pos, flags, clube, lado, tecnico, limit, marca, marca_alta, bytes, tatica, pos2, ferias);
    }
    // sem as regiões do sorteador, pelo mesmo motivo da nota fora da partida (0x62ed70 com cache)
    let regioes = vec![(G_CONTRACT_LOOKUP_RESULT, 4usize)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(
        &regioes,
        || o(p, f, reg, pos, flags, clube, lado, tecnico, limit, marca, marca_alta, bytes, tatica, pos2, ferias),
        || crate::aleatorio::sem_contar(|| lineup_slot_score(p, f, reg, pos, flags, clube, lado, tecnico, limit, marca, marca_alta, bytes, tatica, pos2, ferias)),
    );
    PLACAR.registrar(igual && r == n, || format!("lineup_slot_score({:#x}, pos {:#x}/{:#x}, flags {:#x}, marca {:#x}) original={} rust={} {:x?} x {:x?}", p, pos as u16, pos2 as u16, flags, marca, r, n, mo, mr));
    r
}

/// Comparer for `select_side` (comparison mode: see the header of motor.rs).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn comparar_select_side(t: usize, m: usize, tecnico: usize, linhas: usize, jogadores: usize, lado: u32, clube: usize, outro: usize, jogo: usize, vagas: *mut i8) -> i8 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32, usize, usize, usize, *mut i8) -> i8>(IDX_SELECT_SIDE)(t, m, tecnico, linhas, jogadores, lado, clube, outro, jogo, vagas)
}

/// Comparer for `perceived_ability` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_perceived_ability(o: usize, p: usize, c: usize, tipo: i8, rep: i16) -> i16 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize, usize, i8, i16) -> i16>(IDX_PERCEIVED_ABILITY)(o, p, c, tipo, rep);
    let n = perceived_ability(o, p, c, tipo, rep);
    PLACAR.registrar(r == n, || format!("perceived_ability({:#x}, {:#x}, {}, {}) original={} rust={}", o, p, tipo, rep, r, n));
    r
}

/// Comparer for `pick` (comparison mode: see the header of motor.rs).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "C" fn comparar_pick(
    a1: usize, a2: u32, a3: usize, a4: u32, a5: u32, a6: usize, a7: u32, a8: usize, a9: usize, a10: u32, a11: u32, a12: usize, a13: usize, a14: u32, a15: usize,
) -> u8 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, usize, u32, u32, usize, u32, usize, usize, u32, u32, usize, usize, u32, usize) -> u8>(IDX_PICK);
    let chamar = || o(a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13, a14, a15);
    let n1 = (a2 as u8 as i8).max(0) as usize;
    let n = ((a4 as u8 as i8 as i32 + a5 as u8 as i8 as i32).max(0)) as usize;
    if is_recording_top_level() {
        // dentro de uma comparacao com fita: unidade com vetores (a escala le e escreve na pilha de quem chama)
        return call_recording_buffers(
            0x6e2000,
            0,
            [0, a2, a7, a8 as u32, a9 as u32],
            &[(a1, 4 * n1), (a3, 2 * n), (a12, 8 * n), (a13, n), (a15, 2 * n)],
            &[(a6, 4 * n)],
            || chamar() as u32,
        ) as u8;
    }
    // Sonda (so na comparacao): a 0x6dbc50, no modo 2, nunca escreve o curto da 1a vaga extra e passa
    // o lixo da pilha a escala (retorno 0x6dd383). Conta quantas vezes ele nao e zero e se zera-lo muda
    // a escala (a original duas vezes: com o lixo e com 0).
    if crate::desvios::RETORNO == 0x6dd383 && a5 as u8 != 0 && a15 != 0 {
        use std::sync::atomic::{AtomicU32, Ordering::Relaxed};
        static CHAMADAS: AtomicU32 = AtomicU32::new(0);
        static NAO_ZERO: AtomicU32 = AtomicU32::new(0);
        static MUDA: AtomicU32 = AtomicU32::new(0);
        let campo = (a15 + 2 * (a4 as u8 as usize)) as *mut u16;
        let lixo = *campo;
        let k = CHAMADAS.fetch_add(1, Relaxed) + 1;
        if lixo != 0 {
            NAO_ZERO.fetch_add(1, Relaxed);
            let (r1, r2, igual, _, _) = crate::comparacao::lado_a_lado(&[(a6, 4 * n)], chamar, || {
                *campo = 0;
                let r = chamar();
                *campo = lixo;
                r
            });
            if !igual || r1 != r2 {
                MUDA.fetch_add(1, Relaxed);
            }
        }
        if lixo != 0 || k % 500 == 0 {
            crate::registro::linha(&format!(
                "sonda_lixo: valor {:#06x}; {} escalas da 0x6dbc50, {} com lixo, {} mudam se zerado",
                lixo, k, NAO_ZERO.load(Relaxed), MUDA.load(Relaxed)
            ));
        }
    }
    // a saida e as datas estaticas que a nota (0x6e2f20) toca
    let reg = [(a6, 4 * n), (G_DAYS_SINCE_SCRATCH_DATE, 8), (0xae2418, 1), (G_SUB_DAYS_RESULT_DATE, 8), (0xae249e, 1)];
    let (r, rr, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, chamar, || pick(a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13, a14, a15));
    PLACAR.registrar(igual && r == rr, || format!("pick(n {} vagas {}+{} modo {:#x}) original={} rust={} {:x?} x {:x?}", a2 as u8 as i8, a4 as u8 as i8, a5 as u8 as i8, a7, r, rr, mo, mr));
    r
}

/// Comparer for `stance` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_stance(c: usize) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u8>(IDX_STANCE);
    let m = *((c + 0x12) as *const usize);
    // os ativos (motor+4/+5, 0x6cbbb0), o fator dos dois lados, as datas (0x54cdb0) e o ponteiro da
    // tabela que a busca na liga (0x4a4850) ordena
    let reg = [(m + 4, 2), (m + 0x476a, 8), (G_DAYS_SINCE_SCRATCH_DATE, 8), (0xae2418, 1), (G_COMP_STATS_SORT_CONTEXT, 4)];
    let (r, rr, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(c), || stance(c));
    PLACAR.registrar(igual && r == rr, || format!("stance(lado {}) original={} rust={} {:x?} x {:x?}", byte(c, 0x18), r, rr, mo, mr));
    r
}

/// Comparer for `choose_formation` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_choose_formation(c: usize, fichas: usize, a2: u32, a3: u32, a4: u32) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) -> u8>(IDX_CHOOSE_FORMATION);
    let n = (a2 as u8 as i8).max(0) as usize;
    if is_recording_top_level() {
        // dentro de uma comparacao com fita: unidade com vetores (as fichas estao na pilha de quem chama)
        return call_recording_buffers(0x6e0230, c, [0, a2, a3, a4, 0], &[(fichas, 4 * n)], &[], || o(c, fichas, a2, a3, a4) as u32) as u8;
    }
    // o sorteio, o motor, a copia da tatica (+0x2b), as linhas do clube no banco de taticas (a formacao
    // gravada e o que as instrucoes mexem), as datas da nota (0x6e2f20) e o ponteiro da busca na liga
    let m = *((c + 0x12) as *const usize);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(m, MOTOR), (*((c + 0x2b) as *const usize), 0x18e3), (0xb67a34, 4), (G_DAYS_SINCE_SCRATCH_DATE, 8), (0xae2418, 1), (G_SUB_DAYS_RESULT_DATE, 8), (0xae249e, 1), (G_COMP_STATS_SORT_CONTEXT, 4)]);
    let taticas = *(G_TACTICS as *const usize);
    let linha: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
    let clube = *((c + 0x2f) as *const usize);
    for humano in 0..2u32 {
        let l = if clube == 0 { -1 } else { linha(clube, humano) };
        if l != -1 {
            reg.push(((*((taticas + 0x601) as *const usize)).wrapping_add((l as usize).wrapping_mul(0x91)), 0x91));
        }
    }
    let (r, rr, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(c, fichas, a2, a3, a4), || choose_formation(c, fichas, a2, a3, a4));
    PLACAR.registrar(igual && r == rr, || {
        // o primeiro byte diferente (as regioes sao grandes)
        let dif = mo.0.iter().zip(mr.0.iter()).find_map(|(a, b)| {
            a.1.iter().zip(b.1.iter()).position(|(x, y)| x != y).map(|k| format!("{:#x}+{:#x}: {:x?} x {:x?}", a.0, k, &a.1[k..(k + 8).min(a.1.len())], &b.1[k..(k + 8).min(b.1.len())]))
        });
        format!("choose_formation(n {} modo {:#x} {}) original={} rust={} {}", a2 as u8 as i8, a3, a4 as u8, r, rr, dif.unwrap_or_default())
    });
    r
}

/// Comparer for `instructions` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_instructions(c: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_INSTRUCTIONS);
    // o sorteio, o motor (a marcacao individual no bloco tatico), a tatica copiada (+0x2b) e as linhas
    // do clube no banco de taticas
    let m = *((c + 0x12) as *const usize);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(m, MOTOR), (*((c + 0x2b) as *const usize), 0x18e3), (0xb67a34, 4)]);
    let taticas = *(G_TACTICS as *const usize);
    let linha: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
    let clube = *((c + 0x2f) as *const usize);
    for humano in 0..2u32 {
        let l = if clube == 0 { -1 } else { linha(clube, humano) };
        if l != -1 {
            reg.push(((*((taticas + 0x601) as *const usize)).wrapping_add((l as usize).wrapping_mul(0x91)), 0x91));
        }
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(c), || instructions(c));
    PLACAR.registrar(igual, || {
        let dif = mo.0.iter().zip(mr.0.iter()).find_map(|(a, b)| {
            a.1.iter().zip(b.1.iter()).position(|(x, y)| x != y).map(|k| format!("{:#x}+{:#x}: {:x?} x {:x?}", a.0, k, &a.1[k..(k + 8).min(a.1.len())], &b.1[k..(k + 8).min(b.1.len())]))
        });
        format!("instructions(lado {}) {}", byte(c, 0x18), dif.unwrap_or_default())
    });
}

/// Comparer for `put_in_place` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_put_in_place(c: usize, pos: u32, pessoa: usize, extra: u32) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize, u32) -> u8>(IDX_PUT_IN_PLACE);
    if is_recording_top_level() {
        // dentro dos titulares (ou da comissao): unidade da fita
        return call_recording(FN_MATCH_MAN_PLACE_PLAYER_IN_TEAM, c, [pos, pessoa as u32, extra, 0, 0], || o(c, pos, pessoa, extra) as u32) as u8;
    }
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(*((c + 0x12) as *const usize), MOTOR), (0xb67a34, 4)]);
    let (r, rr, igual, sequencia, k) = comparar_com_fita(reg, || o(c, pos, pessoa, extra), |chamar| {
        place_in_slot_with(c, pos, pessoa, extra, &mut |rotina, this, args| chamar(rotina, this, args, [u32::MAX, u32::MAX, 0xff, 0xff, 0xff]))
    });
    PLACAR.registrar(igual && sequencia && r == rr, || format!("put_in_place({}, {:#x}) original={} rust={} {} chamadas: {}", pos as u8 as i8, pessoa, r, rr, k, tape_detail()));
    r
}

/// Comparer for `order_by_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_order_by_table(a: *const usize, b: *const usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(*const usize, *const usize) -> i32>(IDX_ORDER_BY_TABLE)(a, b);
    let n = order_by_table(a, b);
    PLACAR.registrar(r == n, || format!("order_by_table({:#x}, {:#x}) original={} rust={}", *a, *b, r, n));
    r
}

/// Comparer for `order_by_condition` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_order_by_condition(a: *const usize, b: *const usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(*const usize, *const usize) -> i32>(IDX_ORDER_BY_CONDITION)(a, b);
    let n = order_by_condition(a, b);
    PLACAR.registrar(r == n, || format!("order_by_condition({:#x}, {:#x}) original={} rust={}", *a, *b, r, n));
    r
}

/// Comparer for `starters` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_starters(c: usize) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u8>(IDX_STARTERS);
    if is_recording_top_level() {
        // dentro da comissao do inicio da partida: unidade da fita
        return call_recording(FN_MATCH_MAN_PLACE_SELECTED_PLAYERS, c, [0, 0, 0, 0, 0], || o(c) as u32) as u8;
    }
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(*((c + 0x12) as *const usize), MOTOR), (0xb67a34, 4)]);
    let (r, rr, igual, sequencia, k) = comparar_com_fita(reg, || o(c), |chamar| {
        starters_with(c, &mut |rotina, this, args| chamar(rotina, this, args, [0xff, u32::MAX, 0xff, 0, 0]))
    });
    PLACAR.registrar(igual && sequencia && r == rr, || format!("starters(lado {}) original={} rust={} {} chamadas: {}", byte(c, 0x18), r, rr, k, tape_detail()));
    r
}

/// Comparer for `store_tactic` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_store_tactic(c: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_STORE_TACTIC);
    let mut reg = linhas_do_clube(*((c + 0x2f) as *const usize));
    reg.push((0xb67a34, 4));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(c), || store_tactic(c));
    PLACAR.registrar(igual, || format!("store_tactic(humano {}) {}", byte(c, 0x19), primeira_diferenca(&mo, &mr)));
}

/// Comparer for `substitute` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_substitute(c: usize, goleiro: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_SUBSTITUTE);
    if is_recording_top_level() {
        call_recording(FN_MATCH_MAN_MAKE_SUBSTITUTIONS, c, [goleiro, 0, 0, 0, 0], || {
            o(c, goleiro);
            0
        });
        return;
    }
    let m = *((c + 0x12) as *const usize);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(m, MOTOR), (0xb67a34, 4)]);
    let (_, _, igual, sequencia, k) = comparar_com_fita(reg, || o(c, goleiro), |chamar| {
        substitute_with(c, goleiro as u8, &mut |rotina, this, args| {
            let mascara = match rotina {
                0x6d6f70 => [0xffff, u32::MAX, 0xff, 0xff, 0xff],
                0x897760 => [u32::MAX, 0xff, u32::MAX, 0, 0],
                0x6e1aa0 => [0, 0, 0, 0, 0],
                _ => [0, 0xff, u32::MAX, u32::MAX, u32::MAX],
            };
            chamar(rotina, this, args, mascara)
        })
    });
    PLACAR.registrar(igual && sequencia, || format!("substitute(goleiro {}) {} chamadas: {}", goleiro as u8, k, tape_detail()));
}

/// Comparer for `redo_team` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_redo_team(c: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_REDO_TEAM);
    if is_recording_top_level() {
        call_recording(FN_MATCH_MAN_REASSIGN_POSITIONS, c, [0, 0, 0, 0, 0], || {
            o(c);
            0
        });
        return;
    }
    let m = *((c + 0x12) as *const usize);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(m, MOTOR), (0xb67a34, 4)]);
    let (_, _, igual, sequencia, k) = comparar_com_fita(reg, || o(c), |chamar| {
        rebuild_team_with(c, &mut |rotina, this, args| {
            let mascara = match rotina {
                0x6d6f70 => [0xffff, u32::MAX, 0xff, 0xff, 0xff],
                0x6e0230 => [0, 0xff, u32::MAX, u32::MAX, 0],
                _ => [0, 0xff, u32::MAX, u32::MAX, u32::MAX],
            };
            chamar(rotina, this, args, mascara)
        })
    });
    PLACAR.registrar(igual && sequencia, || format!("redo_team() {} chamadas: {}", k, tape_detail()));
}

/// Comparer for `best_in_list` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_best_in_list(this: usize, lista: *const usize, n: i8) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *const usize, i8) -> usize>(IDX_BEST_IN_LIST);
    let reg = [crate::aleatorio::REGIOES_DO_ESTADO[0], crate::aleatorio::REGIOES_DO_ESTADO[1], (G_DAYS_SINCE_SCRATCH_DATE, 8), (0xae2418, 1)];
    let (r, rr, igual, _, _) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, lista, n), || best_in_list(this, lista, n));
    PLACAR.registrar(igual && r == rr, || format!("best_in_list({}) original={:#x} rust={:#x}", n, r, rr));
    r
}

/// Comparer for `best_rated` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_best_rated(this: usize) -> i8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i8>(IDX_BEST_RATED);
    let reg = [crate::aleatorio::REGIOES_DO_ESTADO[0], crate::aleatorio::REGIOES_DO_ESTADO[1], (G_DAYS_SINCE_SCRATCH_DATE, 8), (0xae2418, 1)];
    let (r, n, igual, _, _) = crate::comparacao::lado_a_lado_leve(&reg, || o(this), || best_rated(this));
    PLACAR.registrar(igual && r == n, || format!("best_rated() original={} rust={}", r, n));
    r
}

/// Comparer for `rate_player` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_rate_player(pessoa: usize, jogador: usize, clube: usize) -> i16 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize) -> i16>(IDX_RATE_PLAYER);
    // a 0x54cdb0 mexe na data estatica dela: a comparacao restaura
    let reg = [crate::aleatorio::REGIOES_DO_ESTADO[0], crate::aleatorio::REGIOES_DO_ESTADO[1], (G_DAYS_SINCE_SCRATCH_DATE, 8), (0xae2418, 1)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(pessoa, jogador, clube), || rate_player(pessoa, jogador, clube));
    PLACAR.registrar(igual && r == n, || format!("rate_player() original={} rust={} {:x?} x {:x?}", r, n, mo, mr));
    r
}
