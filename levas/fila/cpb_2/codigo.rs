// ---------- %%LEVA%%a leva: a copa: o progresso do time e a copa terminada (Cup.cpp) ----------
// Usa a escuta do item cpb_1 (cpb1_*), que vem antes na fila.

/// { 0x521b30 } (thiscall ret 8, Cup.cpp): `Cup::record_team_progress(time, rodada)`: guarda até onde o time foi pela
/// posição 0x68 da vtable da copa (set_club_progress: clube, etapa +0x44, três códigos). Com a última rodada do tipo
/// 0x8c (disputa de 3º): na antepenúltima, o campeão (+5 = 1) dá 1,1,1 e o vice (2) 2,2,2; na última (a de 3º), o 3º
/// dá 3,3,3 e o 4º 4,4,4; nas outras, sem desfecho (0) o código da rodada seguinte (+0x5b dela; antes da antepenúltima:
/// 4) com 1 no meio, com desfecho o da rodada (+0x5b). Sem ela: na última, o campeão 1,1,1 e o vice o número de
/// confrontos +1; nas outras, sem desfecho os confrontos da seguinte +1 (antes da última) ou o código dela, com 1 no
/// meio; com desfecho o código da rodada. Chama pelo ponteiro da vtable, como a original; os bytes vão sem os bits
/// altos que a original deixa nos registradores (as 70 versões de set_club_progress leem bytes).
unsafe extern "thiscall" fn copa_progresso_do_time(copa: usize, time: u32, rodada: u32) {
    let n = cpb1_i16(copa + 0x3c) as i32;
    let rodadas = cpb1_us(copa + 0xa3);
    let r = rodada as i16 as i32;
    let e = cpb1_em(cpb1_us(copa + 0xa7), time as i16 as i32, 6);
    let d = cpb1_u8(e + 5);
    let codigo = |o: usize| cpb1_u8(cpb1_em(rodadas, r, 0x68) + o);
    let (x, y, z) = if n > 0 && cpb1_u16(cpb1_em(rodadas, n, 0x68) - 0x61) == 0x8c {
        if r == n - 2 {
            match d { 1 => (1, 1, 1), 2 => (2, 2, 2), _ => return }
        } else if r == n - 1 {
            match d { 3 => (3, 3, 3), 4 => (4, 4, 4), _ => return }
        } else if d == 0 {
            if r + 1 == n - 2 { (4, 1, 4) } else { let v = codigo(0xc3); (v, 1, v) }
        } else {
            let v = codigo(0x5b);
            (v, v, v)
        }
    } else if r == n - 1 {
        match d {
            1 => (1, 1, 1),
            2 => { let v = codigo(0x1a).wrapping_add(1); (v, v, v) }
            _ => return,
        }
    } else if d == 0 {
        let v = if r + 1 == n - 1 { codigo(0x82).wrapping_add(1) } else { codigo(0xc3) }; // a rodada seguinte
        (v, 1, v)
    } else {
        let v = codigo(0x5b);
        (v, v, v)
    };
    let clube = cpb1_u32(e);
    let etapa = cpb1_u8(copa + 0x44) as u32;
    let f = cpb1_us(cpb1_us(copa) + 0x68);
    let a = [clube, etapa, x as u32, y as u32, z as u32];
    cpb1_chamar(f, copa, &a, &[!0, 0xff, 0xff, 0xff, 0xff], || {
        core::mem::transmute::<usize, unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> u32>(f)(copa, a[0], a[1], a[2], a[3], a[4])
    });
}

unsafe extern "thiscall" fn comparar_copa_progresso_do_time(copa: usize, time: u32, rodada: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32)>(%%I0%%);
    let (_, ok, msg) = cpb1_comparar(&[], &[(copa, 0x68, cpb1_v5 as *const () as usize)], &[], || o(copa, time, rodada), || copa_progresso_do_time(copa, time, rodada));
    PLACAR.registrar(ok, || format!("copa_progresso_do_time(time {}, rodada {}) {}", time as i16, rodada as i16, msg));
}

/// { 0x521e00 } (thiscall ret 8, Cup.cpp): `Cup::check_complete(etapas, _)`: 0 se a copa não acabou (+0xb1); pedindo
/// as etapas (1) numa copa de etapas (+0x44 = 0xff), 0 se uma das etapas (+0xc, de 0 até +0x30) não terminou (posição
/// 0x10 da vtable dela, com 0 e 1: a mesma pergunta) ou se a etapa atual (+0x30) não chegou à última (+0x2c − 1);
/// senão 1. Devolve só `al`.
unsafe extern "thiscall" fn copa_terminou(copa: usize, etapas: u32, _a2: u32) -> u8 {
    if cpb1_u8(copa + 0xb1) == 0 {
        return 0;
    }
    if etapas == 1 && cpb1_u8(copa + 0x44) == 0xff {
        let mut k: i32 = 0;
        if cpb1_u32(copa + 0x30) as i32 >= 0 {
            loop {
                let etapa = cpb1_us(cpb1_em(cpb1_us(copa + 0xc), k, 4));
                let f = cpb1_us(cpb1_us(etapa) + 0x10);
                let fim = cpb1_chamar(f, etapa, &[0, 1], &[!0, !0], || {
                    core::mem::transmute::<usize, unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(f)(etapa, 0, 1)
                });
                if fim as u8 == 0 {
                    return 0; // uma etapa não terminou
                }
                k += 1;
                if k > cpb1_u32(copa + 0x30) as i32 {
                    break;
                }
            }
        }
        if (cpb1_u32(copa + 0x30) as i32) < (cpb1_u32(copa + 0x2c) as i32).wrapping_sub(1) {
            return 0;
        }
    }
    1
}

unsafe extern "thiscall" fn comparar_copa_terminou(copa: usize, etapas: u32, a2: u32) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32) -> u8>(%%I1%%);
    // as etapas escutadas (a pergunta delas pode ter efeito: League::check_complete paga os prêmios ao terminar)
    let mut vtabs = Vec::new();
    if cpb1_u8(copa + 0xb1) != 0 && etapas == 1 && cpb1_u8(copa + 0x44) == 0xff {
        for k in 0..=(cpb1_u32(copa + 0x30) as i32) {
            vtabs.push((cpb1_us(cpb1_em(cpb1_us(copa + 0xc), k, 4)), 0x10, cpb1_v2 as *const () as usize));
        }
    }
    let (r, ok, msg) = cpb1_comparar(&[], &vtabs, &[], || o(copa, etapas, a2), || copa_terminou(copa, etapas, a2));
    PLACAR.registrar(ok, || format!("copa_terminou(etapas {}) {}", etapas, msg));
    r
}
