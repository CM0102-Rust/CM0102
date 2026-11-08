use super::*;

// The comparers of the routines in ../match_eng.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../match_eng.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `restart_play` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_restart_play(m: usize) {
    use std::sync::atomic::Ordering::Relaxed;
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_RESTART_PLAY);
    if is_recording_top_level() {
        return o(m);
    }
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(m, MOTOR), (0xb67a34, 4), (G_CONTRACT_LOOKUP_RESULT, 4)]);
    let t = *((m + 0x4792) as *const usize);
    for clube in [*((t + 0x1c) as *const usize), *((t + 0x20) as *const usize)] {
        reg.extend(linhas_do_clube(clube));
    }
    let write = RECORD_RESTART.swap(true, Relaxed);
    let a7 = GRAVAR_A7.swap(true, Relaxed);
    let (_, _, igual, sequencia, k) = comparar_com_fita(reg, || o(m), |chamar| {
        restart_play_with(m, &mut |rotina, this, args| {
            let mask = match rotina {
                0x6d6f70 => [0xffffff, u32::MAX, 0xff, 0xff, 0xff],
                0x6e1090 => [u32::MAX, 0, 0, 0, 0],
                _ => [0, 0, 0, 0, 0],
            };
            chamar(rotina, this, args, mask)
        })
    });
    GRAVAR_A7.store(a7, Relaxed);
    RECORD_RESTART.store(write, Relaxed);
    PLACAR.registrar(igual && sequencia, || format!("restart_play(modo {}) {} chamadas: {}", *((m + 0x8eb2) as *const u8), k, tape_detail()));
}

/// Comparer for `engine_loop` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_engine_loop(m: usize, n: u32) -> u32 {
    // simplification: orquestrador sem conta propria; a fita dele fotografaria o motor a cada decisao de
    // jogador. Validado pela carreira (Rust x original); as rotinas que ele chama seguem comparadas.
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_ENGINE_LOOP)(m, n)
}

/// Comparer for `build_engine` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_build_engine(m: usize) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_BUILD_ENGINE)(m)
}

/// Comparer for `prepare_match` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_prepare_match(m: usize, jogo: usize, rapida: u32, commentator: usize, ao_vivo: u32) -> u8 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize, u32) -> u8>(IDX_PREPARE_MATCH)(m, jogo, rapida, commentator, ao_vivo)
}

/// Comparer for `destroy_engine` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_destroy_engine(m: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_DESTROY_ENGINE)(m)
}

/// Comparer for `com_fita` (comparison mode: see the header of motor.rs).
pub(crate) unsafe fn comparar_com_fita<R>(reg: Vec<(usize, usize)>, original: impl FnOnce() -> R, rust: impl FnOnce(&mut dyn FnMut(usize, usize, [u32; 5], [u32; 5]) -> u32) -> R) -> (R, R, bool, bool, usize) {
    // a fita de fora (comparacao aninhada dentro de uma unidade gravada) volta no fim
    let externa = tape_lock().take();
    *tape_lock() = Some((reg.clone(), Vec::new()));
    // contexto proprio: a rotina comparada pode estar dentro de uma chamada gravavel em andamento (a
    // 0x6f5b80 roda dentro da 0x6f09f0 no jogo normal); as chamadas dela contam a partir de zero
    let level = RECORDABLE_DEPTH.swap(0, std::sync::atomic::Ordering::Relaxed);
    TAPE_DETAIL.lock().unwrap_or_else(|e| e.into_inner()).clear();
    *REQUESTED_BUFFERS.lock().unwrap_or_else(|e| e.into_inner()) = None;
    let gravadas = core::cell::RefCell::new(Vec::new());
    let sequencia = core::cell::Cell::new(true);
    let rastros = core::cell::RefCell::new((Vec::new(), Vec::new()));
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(
        &reg,
        || {
            let (r, ro) = crate::aleatorio::rastrear(original);
            rastros.borrow_mut().0 = ro;
            *gravadas.borrow_mut() = tape_lock().take().map_or(Vec::new(), |f| f.1);
            r
        },
        || {
            let g = gravadas.borrow();
            let mut prox = g.iter();
            // (rotina, this, argumentos, mascara dos argumentos que a rotina chamada le)
            let (n, rr) = crate::aleatorio::rastrear(|| crate::aleatorio::sem_contar(|| {
                rust(&mut |rotina, this, a, mascara| match prox.next() {
                    Some((gr, gt, ga, resposta, antes, depois, vetores)) if *gr == rotina && *gt == this && (0..5).all(|i| ga[i] & mascara[i] == a[i] & mascara[i]) => {
                        let now = crate::comparacao::Memoria::ler(&reg);
                        if now != *antes && sequencia.get() {
                            let difs: Vec<String> = antes.0.iter().zip(now.0.iter()).filter(|(x, y)| x != y).map(|(x, y)| format!("{:#x}: {:x?} x {:x?}", x.0, x.1, y.1)).collect();
                            *TAPE_DETAIL.lock().unwrap_or_else(|e| e.into_inner()) = format!("estado antes de {:#x} difere: {}", rotina, difs.join("; ").chars().take(300).collect::<String>());
                            sequencia.set(false);
                        }
                        depois.escrever();
                        if !vetores.0.is_empty() || !vetores.1.is_empty() {
                            match REQUESTED_BUFFERS.lock().unwrap_or_else(|e| e.into_inner()).take() {
                                Some((ent, sai)) => {
                                    if read_buffers(&ent) != vetores.0 && sequencia.get() {
                                        *TAPE_DETAIL.lock().unwrap_or_else(|e| e.into_inner()) = format!("entradas de {:#x} diferem", rotina);
                                        sequencia.set(false);
                                    }
                                    write_buffers(&sai, &vetores.1);
                                }
                                None => {
                                    if sequencia.get() {
                                        *TAPE_DETAIL.lock().unwrap_or_else(|e| e.into_inner()) = format!("vetores de {:#x} nao pedidos", rotina);
                                    }
                                    sequencia.set(false);
                                }
                            }
                        }
                        *resposta
                    }
                    outra => {
                        if sequencia.get() {
                            *TAPE_DETAIL.lock().unwrap_or_else(|e| e.into_inner()) =
                                format!("gravada {:x?} pedida ({:#x}, {:#x}, {:x?})", outra.map(|c| (c.0, c.1, c.2, c.3)), rotina, this, a);
                        }
                        sequencia.set(false);
                        0
                    }
                })
            }));
            rastros.borrow_mut().1 = rr;
            if let Some(c) = prox.next() {
                if sequencia.get() {
                    *TAPE_DETAIL.lock().unwrap_or_else(|e| e.into_inner()) = format!("sobrou a gravada {:x?}", (c.0, c.1, c.2, c.3));
                }
                sequencia.set(false);
            }
            n
        },
    );
    // os sorteios (pedido, resultado) tambem tem de bater: `sortear(5)` no lugar de `sortear(6)` com o
    // mesmo resultado nao muda a memoria nem o estado do gerador, mas e outra rotina (131a leva)
    if sequencia.get() && rastros.borrow().0 != rastros.borrow().1 {
        *TAPE_DETAIL.lock().unwrap_or_else(|e| e.into_inner()) = "sorteios diferentes".to_string();
        sequencia.set(false);
    }
    if !igual {
        let difs: Vec<String> = mo.0.iter().zip(mr.0.iter()).filter(|(a, b)| a != b).map(|(a, b)| format!("{:#x}: {:x?} x {:x?}", a.0, a.1, b.1)).collect();
        let mut d = TAPE_DETAIL.lock().unwrap_or_else(|e| e.into_inner());
        if sequencia.get() {
            d.clear();
        }
        d.push_str(&format!(" memoria {}", difs.join("; ").chars().take(300).collect::<String>()));
    }
    if !igual || !sequencia.get() {
        let (ro, rr) = &*rastros.borrow();
        // a partir de pouco antes do primeiro sorteio diferente
        let i = ro.iter().zip(rr.iter()).take_while(|(a, b)| a == b).count().saturating_sub(3);
        TAPE_DETAIL.lock().unwrap_or_else(|e| e.into_inner()).push_str(&format!(" | sorteios ({} x {}, desde o {}) original {:?} rust {:?}", ro.len(), rr.len(), i, &ro[i..ro.len().min(i + 12)], &rr[i..rr.len().min(i + 12)]));
    }
    RECORDABLE_DEPTH.store(level, std::sync::atomic::Ordering::Relaxed);
    *tape_lock() = externa;
    let k = gravadas.borrow().len();
    (r, n, igual, sequencia.get(), k)
}

/// Comparer for `play` (comparison mode: see the header of motor.rs).
pub(crate) unsafe fn comparar_play(p: usize, a1: u32, a2: u32, a3: u32, a4: u32, rodar: &dyn Fn() -> u32) -> u8 {
    use std::sync::atomic::Ordering::Relaxed;
    let m = *((p + 0x19e) as *const usize);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(m, MOTOR), (0xb67a34, 4), (G_CONTRACT_LOOKUP_RESULT, 4)]);
    let t = *((m + 0x4792) as *const usize);
    for clube in [*((t + 0x1c) as *const usize), *((t + 0x20) as *const usize)] {
        reg.extend(linhas_do_clube(clube));
    }
    let write = RECORD_716.swap(true, Relaxed);
    let a7 = GRAVAR_A7.swap(true, Relaxed);
    let (r, rr, igual, sequencia, k) = comparar_com_fita(reg, || rodar() as u8, |chamar| {
        play_with(p, a1 as i16, f32::from_bits(a2), a3 as i8, a4 as u8, &mut |rotina, this, args| {
            let mask = match rotina {
                0x6d6f70 => [0xffffff, u32::MAX, 0xff, 0xff, 0xff],
                0x6cd770 => [0xff, 0xff, 0xffff, 0xff, u32::MAX],
                0x897760 => [u32::MAX, 0xff, u32::MAX, 0, 0],
                0x6e1aa0 | 0x6f0630 => [0, 0, 0, 0, 0],
                _ => [0xff, 0, 0, 0, 0],
            };
            chamar(rotina, this, args, mask)
        })
    });
    GRAVAR_A7.store(a7, Relaxed);
    RECORD_716.store(write, Relaxed);
    PLACAR.registrar(igual && sequencia && r == rr, || format!("play(forca {}, estilo {}, forte {}) original={} rust={} {} chamadas: {}", a1 as i16, a3 as i8, a4 as u8, r, rr, k, tape_detail()));
    r
}

/// Comparer for `match_wrap_up` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_match_wrap_up(m: usize) {
    use std::sync::atomic::Ordering::Relaxed;
    let o = original_fn::<unsafe extern "C" fn(usize)>(IDX_MATCH_WRAP_UP);
    if is_recording_top_level() {
        return o(m);
    }
    let reg = wrap_up_regions(m);
    let write = RECORD_WRAP_UP.swap(true, Relaxed);
    let (_, _, igual, sequencia, k) = comparar_com_fita(reg, || {
        zero_stack_below();
        o(m)
    }, |chamar| {
        match_wrap_up_with(m, &mut |rotina, this, args| {
            let mask = match rotina {
                0x68d0d0 => [0, u32::MAX, 0, 0, 0],
                _ => [u32::MAX; 5],
            };
            chamar(rotina, this, args, mask)
        })
    });
    RECORD_WRAP_UP.store(write, Relaxed);
    PLACAR.registrar(igual && sequencia, || format!("match_wrap_up {} chamadas: {}", k, tape_detail()));
}

/// Comparer for `injure` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_injure(m: usize, kind: u32, chance: u32, p: usize, o: usize) {
    if is_recording() {
        // unidade da fita de fora; por dentro, a propria comparacao (fita aninhada)
        call_recording(FN_CM3_MATCH_ENGINE_INJURE_PLAYER, m, [kind, chance, p as u32, o as u32, 0], || {
            compare_injure_with_tape(m, kind, chance, p, o);
            0
        });
        return;
    }
    compare_injure_with_tape(m, kind, chance, p, o);
}

/// Comparer for `day_match_order` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_day_match_order(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_DAY_MATCH_ORDER)(a, b);
    let n = day_match_order(a, b);
    PLACAR.registrar(r == n, || format!("day_match_order({:#x}, {:#x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `action_order` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_action_order(a: *const usize, b: *const usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(*const usize, *const usize) -> i32>(IDX_ACTION_ORDER)(a, b);
    let n = action_order(a, b);
    PLACAR.registrar(r == n, || format!("action_order({:#x}, {:#x}) original={} rust={}", *a, *b, r, n));
    r
}

/// Comparer for `context_commentary` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_context_commentary(m: usize, side: u32, evento: u32, out: *mut i32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, *mut i32) -> u32>(IDX_CONTEXT_COMMENTARY);
    if is_recording_top_level() {
        return o(m, side, evento, out);
    }
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    if !out.is_null() {
        reg.push((out as usize, 4));
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(m, side, evento, out), || crate::aleatorio::sem_contar(|| context_commentary(m, side, evento, out)));
    PLACAR.registrar(igual && r == n, || format!("context_commentary({}, {:#x}) original={} rust={} {:x?} x {:x?}", side as u8, evento as u16, r, n, mo, mr));
    probe_context_commentary(m, o);
    r
}

/// Comparer for `referee_grade` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_referee_grade(m: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_REFEREE_GRADE);
    if is_recording_top_level() {
        return o(m);
    }
    // o sorteio e a ficha do arbitro
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    let referee = *((m + 0xf5b6) as *const usize);
    if referee != 0 && *(G_OFFICIALS_MANAGER as *const usize) != 0 {
        reg.extend(official_record_region(*(G_OFFICIALS_MANAGER as *const usize), *(referee as *const i32)));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(m), || referee_grade(m));
    PLACAR.registrar(igual, || format!("referee_grade {:x?} x {:x?}", mo, mr));
}

/// Comparer for `quick_simulation` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_quick_simulation(m: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_QUICK_SIMULATION);
    if is_recording_top_level() {
        return o(m);
    }
    // o motor inteiro, o sorteio, o erro, 0xae1080, a linha da tatica dos dois clubes (a expulsao
    // pode trocar o capitao) e o registro da partida (placares e agregado)
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(m, MOTOR), (0xb67a34, 4), (G_CONTRACT_LOOKUP_RESULT, 4)]);
    let t = *((m + 0x4792) as *const usize);
    reg.push((t, 0x50));
    for clube in [*((t + 0x1c) as *const usize), *((t + 0x20) as *const usize)] {
        reg.extend(linhas_do_clube(clube));
    }
    let write = RECORD_716.swap(true, std::sync::atomic::Ordering::Relaxed);
    let (_, _, igual, sequencia, k) = comparar_com_fita(reg, || {
        zero_stack_below();
        o(m)
    }, |chamar| {
        quick_simulation_with(m, &mut |rotina, this, args| {
            let mask = match rotina {
                0x6d6f70 => [0xffff, u32::MAX, 0xff, 0xff, 0xff],
                0x6d0980 => [0xff, 0xff, u32::MAX, u32::MAX, 0],
                0x897760 => [u32::MAX, 0xff, u32::MAX, 0, 0],
                0x62ed70 => [u32::MAX, 0, 0, 0, 0],
                0x6e1aa0 => [0, 0, 0, 0, 0],
                _ => [0xff, 0, 0, 0, 0],
            };
            chamar(rotina, this, args, mask)
        })
    });
    RECORD_716.store(write, std::sync::atomic::Ordering::Relaxed);
    PLACAR.registrar(igual && sequencia, || format!("quick_simulation {} chamadas: {}", k, tape_detail()));
}

/// Comparer for `record_move` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_record_move(this: usize, a: u16, b: u32, c: u32, d: u8, e: u8, f2: u8, g: u8, h: u8, i: u8) {
    let f = original_fn::<unsafe extern "thiscall" fn(usize, u16, u32, u32, u8, u8, u8, u8, u8, u8)>(IDX_RECORD_MOVE);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this + 0x475a, 16)], || f(this, a, b, c, d, e, f2, g, h, i), || record_move(this, a, b, c, d, e, f2, g, h, i));
    PLACAR.registrar(igual, || format!("record_move original={:x?} rust={:x?}", mo, mr));
}

/// Comparer for `check_move` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_check_move(this: usize) -> u8 {
    let f = original_fn::<unsafe extern "thiscall" fn(usize) -> u8>(IDX_CHECK_MOVE);
    let mut reg = vec![(this + 0x475a, 16)];
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || f(this), || crate::aleatorio::sem_contar(|| check_move(this)));
    PLACAR.registrar(igual && r == n, || format!("check_move original={} {:x?} rust={} {:x?}", r, mo, n, mr));
    r
}

/// Comparer for `apply_factor` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_apply_factor(this: usize, f: f32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, f32)>(IDX_APPLY_FACTOR);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this + 0x1a2, 4)], || o(this, f), || apply_factor(this, f));
    PLACAR.registrar(igual, || format!("apply_factor({}) original={:x?} rust={:x?}", f, mo, mr));
}

/// Comparer for `engine_punish_move` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_engine_punish_move(this: usize, j: usize, other: usize, kind: *mut u8, x: i8, sev: i8, flag: u8, weight: *mut f32, expulsar: u8) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, *mut u8, i8, i8, u8, *mut f32, u8)>(IDX_ENGINE_PUNISH_MOVE);
    let mut reg = regioes_do_cartao(this, j, kind, weight);
    reg.push((other + 0x15, 1));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, j, other, kind, x, sev, flag, weight, expulsar), || crate::aleatorio::sem_contar(|| engine_punish_move(this, j, other, kind, x, sev, flag, weight, expulsar)));
    PLACAR.registrar(igual, || format!("engine_punish_move original={:x?} rust={:x?}", mo, mr));
}

/// Comparer for `engine_punish_foul` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_engine_punish_foul(this: usize, j: usize, kind: *mut u8, x: i8, sev: i8, flag: u8, weight: *mut f32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut u8, i8, i8, u8, *mut f32)>(IDX_ENGINE_PUNISH_FOUL);
    let reg = regioes_do_cartao(this, j, kind, weight);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, j, kind, x, sev, flag, weight), || crate::aleatorio::sem_contar(|| engine_punish_foul(this, j, kind, x, sev, flag, weight)));
    PLACAR.registrar(igual, || format!("engine_punish_foul original={:x?} rust={:x?}", mo, mr));
}

/// Comparer for `choose_square` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_choose_square(this: usize, j: usize, sx: *mut i8, sy: *mut i8, flags: u32) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut i8, *mut i8, u32) -> u8>(IDX_CHOOSE_SQUARE);
    let mut reg = vec![(sx as usize, 1), (sy as usize, 1)];
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, j, sx, sy, flags), || crate::aleatorio::sem_contar(|| choose_square(this, j, sx, sy, flags)));
    PLACAR.registrar(igual && r == n, || format!("choose_square(flags {:#x}) original={} {:x?} rust={} {:x?}", flags, r, mo, n, mr));
    r
}

/// Comparer for `team_strength` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_team_strength(this: usize, l: i8) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8)>(IDX_TEAM_STRENGTH);
    let mut reg = vec![(this + 0x1d8, 0x20), (0xb67a34, 4)];
    for k in 0..20 {
        let p = side_player_slot(this, l, k);
        reg.extend([(p + 0x35, 2), (p + 0x4d, 4)]);
    }
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, l), || crate::aleatorio::sem_contar(|| team_strength(this, l)));
    PLACAR.registrar(igual, || format!("team_strength({}) original={:x?} rust={:x?}", l, mo, mr));
}

/// O pontape inicial mexe em todo o motor (jogadores, grade, bola): compara o objeto inteiro.
pub(crate) unsafe extern "thiscall" fn comparar_kick_off(this: usize) {
    // dentro da comparacao do reinicio (0x6c54a0) a bola parada e uma unidade da fita
    if is_recording_restart() {
        let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_KICK_OFF);
        call_recording(FN_CM3_MATCH_ENGINE_SET_UP_KICK_OFF, this, [0; 5], || {
            o(this);
            0
        });
        return;
    }
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_KICK_OFF);
    let mut reg = vec![(this, MOTOR)];
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this), || crate::aleatorio::sem_contar(|| kick_off(this)));
    PLACAR.registrar(igual, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|r| (0..a[r].1.len()).filter(move |&i| a[r].1[i] != b[r].1[i]).map(move |i| a[r].0 + i)).take(16).collect();
        format!("kick_off() diferencas em {:x?}", dif)
    });
}

/// Comparer for `roll_ball` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_roll_ball(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ROLL_BALL);
    let extra = [(this + 0x8ea0, 0x40), (this + 0x4780, 0x10), (this + 0xf570, 0x20), (this + 0x1a0, 0x10)];
    let (igual, k) = comparar_com_comentarios_em(&extra, || o(this), |c| roll_loose_ball_with(this, c));
    PLACAR.registrar(igual, || format!("roll_ball ({} comentarios)", k));
}

/// Comparer for `fine_positions` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_fine_positions(this: usize, out: *mut u8) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *mut u8)>(IDX_FINE_POSITIONS);
    let mut reg = vec![(out as usize, 2 + 6 * 22), (0xb67a34, 4)];
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, out), || crate::aleatorio::sem_contar(|| fine_positions(this, out)));
    PLACAR.registrar(igual, || format!("fine_positions original={:x?} rust={:x?}", mo, mr));
}

/// Comparer for `next_list_item` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_next_list_item(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_NEXT_LIST_ITEM);
    // o jogo chama bilhoes de vezes (carga de carreira com todas as ligas): o cursor de 4 bytes fica
    // numa variavel, sem as copias alocadas do `lado_a_lado` (que tomavam ~90% do tempo)
    let cursor = (this + 0xa) as *mut u32;
    let before = cursor.read_unaligned();
    let r = o(this);
    let after = cursor.read_unaligned();
    cursor.write_unaligned(before);
    let v = next_list_item(this);
    let after_r = cursor.read_unaligned();
    cursor.write_unaligned(after);
    PLACAR.registrar(after == after_r && r == v, || format!("next_list_item original={:#x} cursor {:#x} rust={:#x} cursor {:#x}", r, after, v, after_r));
    r
}

/// Comparer for `group_nation` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "stdcall" fn comparar_group_nation(nacao: usize) -> u32 {
    let r = original_fn::<unsafe extern "stdcall" fn(usize) -> u32>(IDX_GROUP_NATION)(nacao);
    let n = group_nation(nacao);
    PLACAR.registrar(r == n, || format!("group_nation({:#x}) original={} rust={}", nacao, r, n));
    r
}

/// Comparer for `side_strength` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_side_strength(this: usize, side: i8) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8)>(IDX_SIDE_STRENGTH);
    let l = side as isize;
    let mut reg: Vec<(usize, usize)> = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push(((this as isize + 0x1ce + 2 * l) as usize, 2));
    reg.push(((this as isize + 0xaf + l) as usize, 1));
    reg.push((side_player_slot(this, side, 0), 20 * 0x1be));
    reg.push((G_CONTRACT_LOOKUP_RESULT, 4));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, side), || crate::aleatorio::sem_contar(|| side_strength(this, side)));
    PLACAR.registrar(igual, || format!("side_strength({}) original={:x?} rust={:x?}", side, mo, mr));
}

/// Comparer for `intercept` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_intercept(this: usize, x: i8, y: i8, a3: u32, goal: i8, a5: u32, a6: u32) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8, i8, u32, i8, u32, u32) -> u8>(IDX_INTERCEPT);
    if is_recording_top_level() {
        return o(this, x, y, a3, goal, a5, a6);
    }
    // o que as jogadas mudam e a copia Rust le: a grade inteira, os bytes de todos os jogadores, os
    // campos da bola no motor e o sorteio
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(this + 0x215e, 9 * 12 * 90), (this + 0x8ea0, 0x20), (this + 0x4780, 0x10), (this + 0xf570, 0x44)]);
    for l in 0..2i8 {
        for k in 0..20 {
            let j = side_player_slot(this, l, k);
            reg.extend([(j + 0x12, 1), (j + 0x19, 1), (j + 0x2b, 1), (j + 0x198, 2)]);
        }
    }
    let (r, n, igual, sequencia, k) = comparar_com_fita(reg, || o(this, x, y, a3, goal, a5, a6), |chamar| {
        let mut play = |p: usize, a: [u32; 4]| chamar(FN_MATCH_PLAYER_TRY_BLOCK_SHOT, p, [a[0], a[1], a[2], a[3], 0], ARG_MASK_70DFD0) as u8;
        find_interception_with(this, x, y, a3, goal, a5, a6, &mut |lado, grupo, v, a3, a5, a6| try_side_block_with(this, lado, grupo, v, a3, 1, a5, a6, &mut play))
    });
    PLACAR.registrar(igual && sequencia && r == n, || format!("intercept({}, {} gol {}) original={} rust={} ({} jogadas)", x, y, goal, r, n, k));
    r
}

/// Comparer for `apply_substitutions` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_apply_substitutions(m: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_APPLY_SUBSTITUTIONS);
    if is_recording_top_level() {
        return o(m);
    }
    // o motor inteiro (jogadores e comissoes), o sorteio, o erro, 0xae1080 e a linha da tatica dos dois
    // clubes (o capitao em +0x52, que a substituicao troca)
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(m, MOTOR), (0xb67a34, 4), (G_CONTRACT_LOOKUP_RESULT, 4)]);
    let t = *((m + 0x4792) as *const usize);
    for clube in [*((t + 0x1c) as *const usize), *((t + 0x20) as *const usize)] {
        reg.extend(linhas_do_clube(clube));
    }
    let (_, _, igual, sequencia, k) = comparar_com_fita(reg, || o(m), |chamar| {
        apply_substitutions_with(m, &mut |rotina, this, args| {
            let mask = match rotina {
                0x6d6f70 => [0xffff, u32::MAX, 0xff, 0xff, 0xff],
                0x897760 => [u32::MAX, 0xff, u32::MAX, 0, 0],
                0x6e1aa0 => [0, 0, 0, 0, 0],
                _ => [0xff, 0, 0, 0, 0],
            };
            chamar(rotina, this, args, mask)
        })
    });
    PLACAR.registrar(igual && sequencia, || format!("apply_substitutions() {} chamadas: {}", k, tape_detail()));
}

/// Comparer for `punishments` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_punishments(m: usize, first: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_PUNISHMENTS);
    if is_recording_top_level() {
        return o(m, first);
    }
    // o motor inteiro, o sorteio, o erro, 0xae1080 e a linha da tatica dos dois clubes (o capitao)
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(m, MOTOR), (0xb67a34, 4), (G_CONTRACT_LOOKUP_RESULT, 4)]);
    let t = *((m + 0x4792) as *const usize);
    for clube in [*((t + 0x1c) as *const usize), *((t + 0x20) as *const usize)] {
        reg.extend(linhas_do_clube(clube));
    }
    RECORD_716.store(true, std::sync::atomic::Ordering::Relaxed);
    let (_, _, igual, sequencia, k) = comparar_com_fita(reg, || o(m, first), |chamar| {
        apply_pending_punishments_with(m, first, &mut |rotina, this, args| {
            let mask = match rotina {
                0x6d6f70 => [0xffff, u32::MAX, 0xff, 0xff, 0xff],
                0x897760 => [u32::MAX, 0xff, u32::MAX, 0, 0],
                0x6e1aa0 | 0x716630 => [0, 0, 0, 0, 0],
                _ => [0xff, 0, 0, 0, 0],
            };
            chamar(rotina, this, args, mask)
        })
    });
    RECORD_716.store(false, std::sync::atomic::Ordering::Relaxed);
    PLACAR.registrar(igual && sequencia, || format!("punishments() {} chamadas: {}", k, tape_detail()));
}

/// Comparer for `penalties` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_penalties(m: usize, mode: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_PENALTIES);
    if is_recording_top_level() {
        return o(m, mode);
    }
    // o motor inteiro, o sorteio, o erro, o narrador (+0x8f7a aponta para fora) e o placar da partida
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    let jogo = *((m + 0x4792) as *const usize);
    reg.extend([(m, MOTOR), (0xb67a34, 4), (*((m + 0x8f7a) as *const usize), 1), (jogo + 0x49, 2)]);
    let (_, _, igual, sequencia, k) = comparar_com_fita(reg, || o(m, mode), |chamar| {
        penalty_shootout_step_with(m, mode, &mut |rotina, this, args| {
            let mask = match rotina {
                0x6d6f70 => [0xffff, u32::MAX, 0xff, 0xff, 0xff],
                0x702050 => [0xff, u32::MAX, 0xffff, 0xff, 0],
                0x6f46b0 => [0xff, 0xff, 0, 0, 0],
                0x6f0630 | 0x714d40 => [0, 0, 0, 0, 0],
                _ => [0xff, 0, 0, 0, 0],
            };
            chamar(rotina, this, args, mask)
        })
    });
    PLACAR.registrar(igual && sequencia, || format!("penalties(estado {}) {} chamadas: {}", byte(m, 0x8eb3), k, tape_detail()));
}

/// Comparer for `apply_foul` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_apply_foul(m: usize) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u8>(IDX_APPLY_FOUL);
    if is_recording_top_level() {
        return o(m);
    }
    // o motor inteiro, o sorteio, o erro, 0xae1080 e a linha da tatica dos dois clubes (a expulsao
    // pode trocar o capitao)
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(m, MOTOR), (0xb67a34, 4), (G_CONTRACT_LOOKUP_RESULT, 4)]);
    let t = *((m + 0x4792) as *const usize);
    for clube in [*((t + 0x1c) as *const usize), *((t + 0x20) as *const usize)] {
        reg.extend(linhas_do_clube(clube));
    }
    let kind = byte(m, 0x475a);
    RECORD_716.store(true, std::sync::atomic::Ordering::Relaxed);
    let (r, rr, igual, sequencia, k) = comparar_com_fita(reg, || o(m), |chamar| {
        apply_pending_foul_with(m, &mut |rotina, this, args| {
            let mask = match rotina {
                0x6d6f70 => [0xffff, u32::MAX, 0xff, 0xff, 0xff],
                0x897760 => [u32::MAX, 0xff, u32::MAX, 0, 0],
                0x6e1aa0 => [0, 0, 0, 0, 0],
                _ => [0xff, 0, 0, 0, 0],
            };
            chamar(rotina, this, args, mask)
        })
    });
    RECORD_716.store(false, std::sync::atomic::Ordering::Relaxed);
    PLACAR.registrar(igual && sequencia && r == rr, || format!("apply_foul(tipo {}) original={} rust={} {} chamadas: {}", kind, r, rr, k, tape_detail()));
    r
}

/// Comparer for `contest` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_contest(m: usize, x: u32, y: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> usize>(IDX_CONTEST);
    if is_recording_top_level() {
        return o(m, x, y);
    }
    ADVANTAGE_UNANSWERED.store(false, std::sync::atomic::Ordering::Relaxed);
    // o motor inteiro, o sorteio, o erro, 0xae1080 e a linha da tatica dos dois clubes (a expulsao
    // pode trocar o capitao)
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(m, MOTOR), (0xb67a34, 4), (G_CONTRACT_LOOKUP_RESULT, 4)]);
    let t = *((m + 0x4792) as *const usize);
    for clube in [*((t + 0x1c) as *const usize), *((t + 0x20) as *const usize)] {
        reg.extend(linhas_do_clube(clube));
    }
    let write = RECORD_716.swap(true, std::sync::atomic::Ordering::Relaxed);
    let (r, rr, igual, sequencia, k) = comparar_com_fita(reg, || o(m, x, y), |chamar| {
        tackle_contest_with(m, x as i8, y as i8, &mut |rotina, this, args| {
            let mask = match rotina {
                0x6d6f70 => [0xffff, u32::MAX, 0xff, 0xff, 0xff],
                0x6cd770 => [0xff, 0xff, 0xffff, 0xff, u32::MAX],
                0x62ed70 => [u32::MAX, 0, 0, 0, 0],
                0x6d0980 => [0xff, 0xff, u32::MAX, u32::MAX, 0],
                0x897760 => [u32::MAX, 0xff, u32::MAX, 0, 0],
                0x6e1aa0 | 0x716c90 | 0x6f0630 | 0x7103e0 => [0, 0, 0, 0, 0],
                _ => [0xff, 0, 0, 0, 0],
            };
            chamar(rotina, this, args, mask)
        })
    });
    RECORD_716.store(write, std::sync::atomic::Ordering::Relaxed);
    let certo = igual && sequencia && r == rr;
    if !certo && ADVANTAGE_UNANSWERED.swap(false, std::sync::atomic::Ordering::Relaxed) {
        // a original leu lixo da pilha como vantagem (ver `tackle_contest_with`): conta a parte
        let n = CONTEST_STACK_GARBAGE.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
        if n <= 20 || n.is_power_of_two() {
            linha(&format!("motor: LIXO_DE_PILHA contest({}, {}) vantagem sem resposta do juiz ({} ate agora): {}", x as i8, y as i8, n, tape_detail()));
        }
        PLACAR.registrar(true, String::new);
        return r;
    }
    PLACAR.registrar(certo, || format!("contest({}, {}) original={:#x} rust={:#x} {} chamadas: {}", x as i8, y as i8, r, rr, k, tape_detail()));
    r
}

/// Comparer for `player_left` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_player_left(m: usize, j: usize, ja_away: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_PLAYER_LEFT);
    if is_recording_top_level() {
        return o(m, j, ja_away);
    }
    // o motor inteiro (a grade, os jogadores, a comissao +0x9032), o sorteio e o que a forca do lado
    // e o erro tocam
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(m, MOTOR), (0xb67a34, 4), (G_CONTRACT_LOOKUP_RESULT, 4)]);
    // o capitao guardado na linha da tatica (+0x52) dos dois clubes: a substituicao (0x6e1090) o troca
    // antes de a comparada conferir se quem saiu era o capitao
    let taticas = *(G_TACTICS as *const usize);
    let row: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
    let t = *((m + 0x4792) as *const usize);
    for clube in [*((t + 0x1c) as *const usize), *((t + 0x20) as *const usize)] {
        for humano in 0..2u32 {
            let l = if clube == 0 { -1 } else { row(clube, humano) };
            if l != -1 {
                reg.push(((*((taticas + 0x601) as *const usize)).wrapping_add((l as usize).wrapping_mul(0x91)) + 0x52, 4));
            }
        }
    }
    let (_, _, igual, sequencia, k) = comparar_com_fita(reg, || o(m, j, ja_away), |chamar| {
        player_left_pitch_with(m, j, ja_away as u8, &mut |rotina, this, args| {
            chamar(rotina, this, args, if rotina == 0x897760 { [u32::MAX, 0xff, u32::MAX, 0, 0] } else { [0xff, 0, 0, 0, 0] })
        })
    });
    PLACAR.registrar(igual && sequencia, || format!("player_left(ja_fora {}) {} chamadas: {}", ja_away as u8, k, tape_detail()));
}

/// Comparer for `tackle_injury` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_tackle_injury(m: usize, a: usize, kind: u32, b: usize, kind_b: u32, direto: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, u32)>(IDX_TACKLE_INJURY);
    if is_recording_top_level() {
        return o(m, a, kind, b, kind_b, direto);
    }
    // o sorteio, os dois jogadores, os registros de lesao deles e o que a 0x6d0980 mexe no motor
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(m + 0x8ea0, 0x20), (m + 0xf582, 4), (m + 0xf639, 2)]);
    let table = *(*(G_INJURY as *const usize) as *const usize);
    for p in [a, b] {
        if p != 0 {
            reg.push((p, 0x1be));
            let person = *((p + 0x69) as *const usize);
            if person != 0 && *(person as *const i32) >= 0 && *(person as *const i32) < *(G_STAFF_COUNT as *const i32) {
                reg.push((table + *(person as *const i32) as usize * 31, 31));
            }
        }
    }
    let (_, _, igual, sequencia, k) = comparar_com_fita(reg, || o(m, a, kind, b, kind_b, direto), |chamar| {
        tackle_injury_with(m, a, kind as u16 as i16, b, kind_b as u16 as i16, direto as u8, &mut |rotina, this, args| {
            chamar(rotina, this, args, if rotina == 0x62ed70 { [u32::MAX, 0, 0, 0, 0] } else { [0xff, 0xff, u32::MAX, u32::MAX, 0] })
        })
    });
    PLACAR.registrar(igual && sequencia, || format!("tackle_injury(lances {}, {} direto {}) {} chamadas: {}", kind as i16, kind_b as i16, direto as u8, k, tape_detail()));
}

/// Comparer for `kit` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_kit(clube: usize, other: usize, qualquer: u32) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize, u32) -> u8>(IDX_KIT)(clube, other, qualquer);
    let n = kit(clube, other, qualquer);
    PLACAR.registrar(r == n, || format!("kit(qualquer {}) original={} rust={}", qualquer as u8, r, n));
    r
}

/// Comparer for `develop_player` (comparison mode: see the header of motor.rs).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "C" fn comparar_develop_player(jm: usize, person: usize, clube: usize, rep: usize, comp: usize, rating: f32, average_b: u32, strong: u32) {
    type F = unsafe extern "C" fn(usize, usize, usize, usize, usize, f32, u32, u32);
    let o = original_fn::<F>(IDX_DEVELOP_PLAYER);
    if is_recording_wrap_up_calls() {
        call_recording(FN_UPDATE_PLAYER_REPUTATION_AFTER_MATCH, jm, [person as u32, clube as u32, rep as u32, rating.to_bits(), average_b & 0xff | (strong & 0xff) << 8], || {
            o(jm, person, clube, rep, comp, rating, average_b, strong);
            0
        });
        return;
    }
    let st = *((person + staff::PLAYER) as *const usize);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(st + 5, 0xa), (st + 0x45, 1)]);
    if jm != 0 {
        reg.extend([(jm + 0x17, 1), (jm + 0x47, 6)]);
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(jm, person, clube, rep, comp, rating, average_b, strong), || {
        crate::aleatorio::sem_contar(|| develop_player(jm, person, clube, rep, comp, rating, average_b, strong))
    });
    PLACAR.registrar(igual, || format!("develop_player(nota {} jogo {}) original={:x?} rust={:x?}", rating, jm != 0, mo, mr));
}

/// Comparer for `opposition_challenge` (comparison mode: see the header of motor.rs).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn comparar_opposition_challenge(m: usize, atacante: usize, side: i8, kind: i8, a: *mut u8, b: *mut u8, rating: *mut i16, lance: *mut i8, arriscado: *mut i8, free: u8) -> usize {
    type F = unsafe extern "thiscall" fn(usize, usize, i8, i8, *mut u8, *mut u8, *mut i16, *mut i8, *mut i8, u8) -> usize;
    let o = original_fn::<F>(IDX_OPPOSITION_CHALLENGE);
    // as saidas, o sorteio e o folego (+0x4d) dos 40 jogadores (quem vence cansa)
    let mut reg = [(0usize, 0usize); 48];
    reg[0] = crate::aleatorio::REGIOES_DO_ESTADO[0];
    reg[1] = crate::aleatorio::REGIOES_DO_ESTADO[1];
    reg[2] = (rating as usize, 2);
    reg[3] = (lance as usize, 1);
    reg[4] = (arriscado as usize, 1);
    reg[5] = (a as usize, if a.is_null() { 0 } else { 1 });
    reg[6] = (b as usize, if b.is_null() { 0 } else { 1 });
    for l in 0..2i8 {
        for k in 0..20 {
            reg[7 + l as usize * 20 + k] = (side_player_slot(m, l, k) + 0x4d, 4);
        }
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg[..47], || o(m, atacante, side, kind, a, b, rating, lance, arriscado, free), || {
        crate::aleatorio::sem_contar(|| opposition_challenge(m, atacante, side, kind, a, b, rating, lance, arriscado, free))
    });
    PLACAR.registrar(igual && r == n, || format!("opposition_challenge(tipo {:#x}) original={:#x} rust={:#x} {:x?} x {:x?}", kind, r, n, mo, mr));
    r
}

/// Comparer for `pressure` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_pressure(m: usize, j: usize, out: *mut i16, list: usize) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut i16, usize) -> u8>(IDX_PRESSURE);
    if list == 0 {
        let reg = [(out as usize, 2)];
        let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(m, j, out, 0), || pressure(m, j, out, 0));
        PLACAR.registrar(igual && r == n, || format!("pressure() original={} {:x?} rust={} {:x?}", r, mo, n, mr));
        return r;
    }
    if is_recording_top_level() {
        return o(m, j, out, list);
    }
    let (r, n, igual, sequencia, k) = comparar_com_fita(vec![(out as usize, 2)], || o(m, j, out, list), |chamar| {
        pressure_on_player_with(m, j, out, list, &mut |l, p| {
            chamar(FN_LLIST_ADD_TAIL, taped_pressure_list_addr(l, 0x1be), [p as u32, 0x1be, 0, 0, 0], [u32::MAX; 5]);
        })
    });
    PLACAR.registrar(igual && sequencia && r == n, || format!("pressure(lista) original={} rust={} {} copias: {}", r, n, k, tape_detail()));
    r
}

/// Comparer for `period_completed` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_period_completed(m: usize) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u8>(IDX_PERIOD_COMPLETED);
    if is_recording_top_level() {
        return o(m);
    }
    let t = *((m + 0x4792) as *const usize);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(m + 0x1a0, 0x30), (m + 0x215e, 0x8f00 - 0x215e), (m + 0xf56e, 0xd8), (t + 0x38, 0x18)]);
    let (r, n, igual, sequencia, k) = comparar_com_fita(reg, || o(m), |chamar| {
        period_completed_with(m, &mut |rotina, this, args| {
            chamar(rotina, this, args, match rotina {
                0x6d6f70 => [0xffff, u32::MAX, 0xff, 0xff, 0xff],
                0x6be120 => [u32::MAX, 0, 0, 0, 0],
                _ => [0xffff, 0, 0, 0, 0],
            })
        })
    });
    PLACAR.registrar(igual && sequencia && r == n, || format!("period_completed(periodo {:#x}) original={} rust={} {} chamadas: {}", *((m + 0x8ed4) as *const i16), r, n, k, tape_detail()));
    r
}

/// Comparer for `deflect_ball` (comparison mode: see the header of motor.rs).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn comparar_deflect_ball(this: usize, x: i8, y: i8, espalhar: i8, side: i8, inside: u8, cansa: u8, chutador: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8, i8, i8, i8, u8, u8, usize)>(IDX_DEFLECT_BALL);
    if is_recording_top_level() {
        call_recording(FN_CM3_MATCH_ENGINE_MOVE_BALL_TO_CELL, this, [x as u8 as u32, y as u8 as u32, espalhar as u8 as u32 | (side as u8 as u32) << 8, inside as u32, chutador as u32], || {
            o(this, x, y, espalhar, side, inside, cansa, chutador);
            0
        });
        return;
    }
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(this + 0x1a0, 0x30), (this + 0x215e, 0x8f00 - 0x215e), (this + 0xf56e, 0xa0)]);
    let (_, _, igual, sequencia, k) = comparar_com_fita(reg, || o(this, x, y, espalhar, side, inside, cansa, chutador), |chamar| {
        deflect_ball_with(this, x, y, espalhar, side, inside, cansa, chutador, &mut |rotina, alvo, args| {
            chamar(rotina, alvo, args, if rotina == 0x6cd490 { [0xff, 0, 0, 0, 0] } else { [u32::MAX, 0, 0, 0, 0] })
        })
    });
    PLACAR.registrar(igual && sequencia, || format!("deflect_ball({}, {} espalhar {} lado {}) {} chamadas: {}", x, y, espalhar, side, k, tape_detail()));
}

/// Comparer for `line_up_set_piece` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_line_up_set_piece(this: usize) {
    // dentro da comparacao do reinicio (0x6c54a0) a bola parada e uma unidade da fita
    if is_recording_restart() {
        let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_LINE_UP_SET_PIECE);
        call_recording(FN_CM3_MATCH_ENGINE_SET_UP_FREE_KICK, this, [0; 5], || {
            o(this);
            0
        });
        return;
    }
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_LINE_UP_SET_PIECE);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(this + 0x1a0, 0x30), (this + 0x215e, 0x8f00 - 0x215e), (this + 0xf56e, 0x90)]);
    let side_b = byte(this, 0x8eae);
    let (_, _, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || o(this), || crate::aleatorio::sem_contar(|| line_up_set_piece(this)));
    PLACAR.registrar(igual, || format!("line_up_set_piece(lado {})", side_b));
}

/// Comparer for `line_up_kickoff` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_line_up_kickoff(this: usize) {
    // dentro da comparacao do reinicio (0x6c54a0) a bola parada e uma unidade da fita
    if is_recording_restart() {
        let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_LINE_UP_KICKOFF);
        call_recording(FN_CM3_MATCH_ENGINE_SET_UP_GOAL_KICK, this, [0; 5], || {
            o(this);
            0
        });
        return;
    }
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_LINE_UP_KICKOFF);
    // tudo o que a arrumacao (e o que ela chama) grava: o motor de 0x1a0, a grade, os 40 jogadores, a
    // bola e as posses (0x215e..0x8f00) e os ponteiros de 0xf56e
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(this + 0x1a0, 0x30), (this + 0x215e, 0x8f00 - 0x215e), (this + 0xf56e, 0x48)]);
    let (_, _, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || o(this), || crate::aleatorio::sem_contar(|| line_up_kickoff(this)));
    PLACAR.registrar(igual, || format!("line_up_kickoff(lado {})", byte(this, 0x8eae)));
}

/// Comparer for `look_for_play` (comparison mode: see the header of motor.rs).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn comparar_look_for_play(this: usize, side: i8, group: usize, a3: u32, a4: u32, mode: u8, a6: u32, a7: u32) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8, usize, u32, u32, u8, u32, u32) -> u8>(IDX_LOOK_FOR_PLAY);
    if is_recording_top_level() {
        return o(this, side, group, a3, a4, mode, a6, a7);
    }
    // o que a jogada muda e a copia Rust le: a lista e a quantidade do lado, os bytes de cada
    // jogador, o jogador marcado no motor e o sorteio
    let base = (group as isize + side as isize) as usize;
    let list = (group as isize + side as isize * 44) as usize;
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(this + 0xf5ae, 4), (base + 0x58, 1), (list, 44)]);
    for k in 0..byte(base, 0x58).clamp(0, 11) as usize {
        let j = *((list + 4 * k) as *const usize);
        reg.extend([(j + 0x12, 1), (j + 0x19, 1), (j + 0x2b, 1)]);
    }
    let (r, n, igual, sequencia, k) = comparar_com_fita(
        reg,
        || o(this, side, group, a3, a4, mode, a6, a7),
        |chamar| try_side_block_with(this, side, group, a3, a4, mode, a6, a7, &mut |p, a| chamar(FN_MATCH_PLAYER_TRY_BLOCK_SHOT, p, [a[0], a[1], a[2], a[3], 0], ARG_MASK_70DFD0) as u8),
    );
    PLACAR.registrar(igual && sequencia && r == n, || format!("look_for_play({} modo {}) original={} rust={} ({} jogadas)", side, mode, r, n, k));
    r
}

/// Comparer for `choose_teammate` (comparison mode: see the header of motor.rs).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn comparar_choose_teammate(
    m: usize, bx: i8, by: i8, kind: *mut i8, moveu: *mut u8, x: *mut i8, y: *mut i8, best: *mut i32, request: i8,
) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8, i8, *mut i8, *mut u8, *mut i8, *mut i8, *mut i32, i8) -> usize>(IDX_CHOOSE_TEAMMATE);
    let mut regions = vec![(kind as usize, 1), (moveu as usize, 1), (x as usize, 1), (y as usize, 1), (best as usize, 4), (m + 0x8eb2, 1)];
    regions.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado(
        &regions,
        || o(m, bx, by, kind, moveu, x, y, best, request),
        || crate::aleatorio::sem_contar(|| choose_teammate(m, bx, by, kind, moveu, x, y, best, request)),
    );
    PLACAR.registrar(igual && r == v, || format!("choose_teammate(pedido {:#x}) original={:x} {:x?} rust={:x} {:x?}", request, r, mo, v, mr));
    r
}

/// Comparer for `player_order` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_player_order(m: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_PLAYER_ORDER);
    let mut regions = vec![(m + 0x8eb8, 1), (m + 0x8f7e, 4 * 40), (G_ACTION_ORDER_MATCH_ENGINE, 4)];
    regions.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regions, || o(m), || crate::aleatorio::sem_contar(|| player_order(m)));
    PLACAR.registrar(igual, || format!("player_order original={:x?} rust={:x?}", mo, mr));
}

/// Comparer for `penalty_order` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_penalty_order(m: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_PENALTY_ORDER);
    let mut regions = vec![(m, MOTOR)];
    regions.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regions, || o(m), || crate::aleatorio::sem_contar(|| penalty_order(m)));
    PLACAR.registrar(igual, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("penalty_order diferencas em {:x?}", dif)
    });
}

/// Comparer for `who_to_mark` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_who_to_mark(m: usize, j: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_WHO_TO_MARK);
    let (r, v, igual, _, _) = crate::comparacao::lado_a_lado(
        &crate::aleatorio::REGIOES_DO_ESTADO,
        || o(m, j),
        || crate::aleatorio::sem_contar(|| who_to_mark(m, j)),
    );
    PLACAR.registrar(igual && r == v, || format!("who_to_mark original={:x} rust={:x}", r, v));
    r
}

/// Comparer for `take_off_pitch` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_take_off_pitch(this: usize, j: usize, mode: i8) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i8)>(IDX_TAKE_OFF_PITCH);
    let reg = [(this, MOTOR), (0xb67a34, 4)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, j, mode), || take_off_pitch(this, j, mode));
    PLACAR.registrar(igual, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|r| (0..a[r].1.len()).filter(move |&i| a[r].1[i] != b[r].1[i]).map(move |i| a[r].0 + i)).take(16).collect();
        format!("take_off_pitch({}) diferencas em {:x?}", mode, dif)
    });
}

/// Comparer for `individual_marking` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_individual_marking(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_INDIVIDUAL_MARKING);
    let reg = [(this, MOTOR)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this), || individual_marking(this));
    PLACAR.registrar(igual, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|r| (0..a[r].1.len()).filter(move |&i| a[r].1[i] != b[r].1[i]).map(move |i| a[r].0 + i)).take(16).collect();
        format!("individual_marking diferencas em {:x?}", dif)
    });
}

/// Comparer for `new_reputation` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_new_reputation(this: usize, side: i8, value: f32, alt: u8, bonus: u8) {
    let o = original_fn::<unsafe extern "C" fn(usize, i8, f32, u8, u8)>(IDX_NEW_REPUTATION);
    if is_recording_wrap_up_calls() {
        call_recording(FN_MATCH_ENG_COMPUTE_CLUB_REPUTATION, this, [side as u8 as u32, value.to_bits(), alt as u32 | (bonus as u32) << 8, 0, 0], || {
            o(this, side, value, alt, bonus);
            0
        });
        return;
    }
    let reg = [(this, 0x200)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, side, value, alt, bonus), || new_reputation(this, side, value, alt, bonus));
    PLACAR.registrar(igual, || {
        let (a, b) = (&mo.0[0].1, &mr.0[0].1);
        format!("new_reputation lado {} valor {} alt {} bonus {}: {:?} x {:?}", side, value, alt, bonus, &a[0x1d6..0x1da], &b[0x1d6..0x1da])
    });
}

/// Comparer for `player_reputation` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_player_reputation(reg: usize, meu: usize, other: usize, partida: usize, value: f32, alt: u8, bonus: u8) {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize, usize, f32, u8, u8)>(IDX_PLAYER_REPUTATION);
    if is_recording_wrap_up_calls() {
        call_recording(FN_UPDATE_MANAGER_REPUTATION_AFTER_MATCH, reg, [meu as u32, other as u32, partida as u32, value.to_bits(), alt as u32 | (bonus as u32) << 8], || {
            o(reg, meu, other, partida, value, alt, bonus);
            0
        });
        return;
    }
    let regions = [(reg, 6)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regions, || o(reg, meu, other, partida, value, alt, bonus), || player_reputation(reg, meu, other, partida, value, alt, bonus));
    PLACAR.registrar(igual, || format!("player_reputation valor {} alt {} bonus {}: {:?} x {:?}", value, alt, bonus, mo.0[0].1, mr.0[0].1));
}

/// Comparer for `choose_taker` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_choose_taker(this: usize, side: i8, kind: i8, excluidos: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8, i8, usize) -> usize>(IDX_CHOOSE_TAKER);
    let mut regions = vec![(this, MOTOR)];
    regions.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(
        &regions,
        || o(this, side, kind, excluidos),
        || crate::aleatorio::sem_contar(|| choose_taker(this, side, kind, excluidos)),
    );
    PLACAR.registrar(igual && r == n, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("choose_taker(lado {}, tipo {}) original={:x} rust={:x}, diferencas em {:x?}", side, kind, r, n, dif)
    });
    r
}

/// Comparer for `throw_in` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_throw_in(this: usize) {
    // dentro da comparacao do reinicio (0x6c54a0) a bola parada e uma unidade da fita
    if is_recording_restart() {
        let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_THROW_IN);
        call_recording(FN_CM3_MATCH_ENGINE_SET_UP_THROW_IN, this, [0; 5], || {
            o(this);
            0
        });
        return;
    }
    comparar_reinicio("throw_in", this, original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_THROW_IN), throw_in);
}

/// Comparer for `corner` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_corner(this: usize) {
    // dentro da comparacao do reinicio (0x6c54a0) a bola parada e uma unidade da fita
    if is_recording_restart() {
        let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CORNER);
        call_recording(FN_CM3_MATCH_ENGINE_SET_UP_CORNER_KICK, this, [0; 5], || {
            o(this);
            0
        });
        return;
    }
    comparar_reinicio("corner", this, original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CORNER), corner);
}

/// Comparer for `penalty` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_penalty(this: usize) {
    // dentro da comparacao do reinicio (0x6c54a0) a bola parada e uma unidade da fita
    if is_recording_restart() {
        let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_PENALTY);
        call_recording(FN_CM3_MATCH_ENGINE_SET_UP_PENALTY, this, [0; 5], || {
            o(this);
            0
        });
        return;
    }
    comparar_reinicio("penalty", this, original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_PENALTY), penalty);
}
