
// ---------- centesima quadragesima quinta leva: o laco do motor (0x6b98c0) ----------

// Chamadas do laco pelo endereco: cada uma passa pelo proprio desvio (copia Rust ou comparador).
unsafe fn em0(a: usize, this: usize) -> u32 {
    core::mem::transmute::<usize, unsafe extern "thiscall" fn(usize) -> u32>(a)(this)
}
unsafe fn em1(a: usize, this: usize, x: u32) -> u32 {
    core::mem::transmute::<usize, unsafe extern "thiscall" fn(usize, u32) -> u32>(a)(this, x)
}
unsafe fn em2(a: usize, this: usize, x: u32, y: u32) -> u32 {
    core::mem::transmute::<usize, unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(a)(this, x, y)
}
unsafe fn em3(a: usize, this: usize, x: u32, y: u32, z: u32) -> u32 {
    core::mem::transmute::<usize, unsafe extern "thiscall" fn(usize, u32, u32, u32) -> u32>(a)(this, x, y, z)
}
unsafe fn em4(a: usize, this: usize, x: u32, y: u32, z: u32, w: u32) -> u32 {
    core::mem::transmute::<usize, unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32>(a)(this, x, y, z, w)
}

/// Fim da partida pelo laco: o fechamento (0x6d1cc0) e, para cada clube que um humano dirige
/// (0x600a50) sem o registro +0x9a05 do tecnico (0x5fd480), as instrucoes guardadas voltam (0x896fe0).
unsafe fn encerrar_clubes(m: usize) {
    let u = |q: usize, o: usize| *((q + o) as *const usize);
    let f: unsafe extern "C" fn(usize) = core::mem::transmute(0x6d1cc0usize);
    f(m);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    for (o, marca) in [(0x1c, 0x40), (0x20, 0x41)] {
        let t = u(m, 0x4792);
        if byte(t, marca) == 0 && humano(u(t, o), 1, 1, 0, 0) != 0 && em1(0x5fd480, u(0xb63c98, 0), u(u(t, o), 0xcf) as u32) == 0 {
            em4(0x896fe0, u(0xde1d08, 0), u(t, o) as u32, 1, 0, 0);
        }
    }
}

/// Os mais perto da bola de cada lado (0x6c30e0, so dentro da area quando a bola esta nela) e se o
/// guarda da posicao 1 e o mais proximo dela (0x6ce110 + 0x6c3010).
unsafe fn perto_da_bola(m: usize) {
    let (bx, by) = (byte(m, 0x8ea7), byte(m, 0x8ea8));
    let a0 = (2..=6).contains(&bx) && (0..=1).contains(&by);
    *((m + 0xf586) as *mut usize) = em4(0x6c30e0, m, 0, bx as u8 as u32, by as u8 as u32, !a0 as u32) as usize;
    let (bx, by) = (byte(m, 0x8ea7), byte(m, 0x8ea8));
    let a1 = (2..=6).contains(&bx) && (10..=11).contains(&by);
    *((m + 0xf58a) as *mut usize) = em4(0x6c30e0, m, 1, bx as u8 as u32, by as u8 as u32, !a1 as u32) as usize;
    for (lado, campo) in [(0u32, 0x8eac), (1, 0x8ead)] {
        let (bx, by) = (byte(m, 0x8ea7), byte(m, 0x8ea8));
        let g = em1(0x6ce110, m, lado);
        *((m + campo) as *mut u8) = em3(0x6c3010, m, g, bx as u8 as u32, by as u8 as u32) as u8;
    }
}

/// { 0x6b98c0 } (ret 4, match_eng.cpp): o laco do motor, `n` instantes da partida. Partida ja
/// decidida (+0): simulacao rapida (0x6beeb0), destaque (0x6d0f80), fechamento, instrucoes dos
/// humanos de volta e nota do arbitro (0x6d4a20); devolve 2. Senao limpa as fichas e os ponteiros
/// de quem saiu e, a cada instante: punicoes pendentes (0x6d1210), disputa de penaltis (0x6be710;
/// no fim, 2), o pontape inicial narrado (0x212f/0x1f74), a substituicao sorteada, o lance marcado
/// (0x6bac40/0x6bba10), o reinicio (0x6c54a0) e o cobrador da bola parada (0x6ceb90). No relogio
/// (+0x8ed0): fim por abandono (8), o periodo (0x6bd930: 4 nos intervalos; fim da partida com o
/// placar agregado, 0x896290 e 2 ou 4). Bola rolando (0x6bbe40) ou parada somam acrescimos; senao
/// a posse (0x6cf750), o tempo de cada jogador (0x6f4aa0), as linhas (0x6c3aa0, 0x6cd660, 0x6bd760),
/// os mais perto da bola, a pressao nas areas (0x6bd6b0), as marcacoes (0x7160f0/0x6bc030) e a
/// decisao de cada jogador ativo (0x7103e0), com o chute pendente (0x6f6c00) e a disputa (0x6c9cd0).
/// Devolve 1 ao fim dos `n` instantes.
#[allow(clippy::cognitive_complexity)]
unsafe extern "thiscall" fn laco_do_motor(m: usize, n_in: u32) -> u32 {
    let u = |o: usize| *((m + o) as *const usize);
    let b = |o: usize| (m + o) as *mut u8;
    let w = |o: usize| (m + o) as *mut i16;
    let narrar = |codigo: u32, a4: u32| {
        let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32) = core::mem::transmute(0x6d6f70usize);
        f(u(0x8f7a), codigo, m as u32, u32::MAX, a4, u32::MAX, u32::MAX, 0);
    };
    let acrescimos = || {
        let r = sortear(10) as i8;
        *w(0x8ed0) = (*w(0x8ed0)).wrapping_add(r as i16);
        let v = sortear(ftol(r as f64 * f64_em(0x9689b0)));
        *b(0x8eb7) = (*b(0x8eb7)).wrapping_add(v as u8);
    };
    let mut n = n_in as u8 as i8;
    if byte(m, 0) != 0 {
        em0(0x6beeb0, m);
        em0(0x6d0f80, m);
        encerrar_clubes(m);
        em0(0x6d4a20, m);
        return 2;
    }
    for lado in 0..2 {
        for k in 0..20 {
            let f = m + 0x4796 + (lado * 20 + k) * 0x1be;
            if byte(f, 0x19) >= 0 && *((f + 0x19e) as *const u32) == 0 {
                *((f + 0x19) as *mut u8) = 0xfb;
            }
        }
    }
    for o in [0xf576, 0xf57a, 0xf57e, 0xf5ae, 0x91de, 0xf586, 0xf58a, 0xf5a6, 0xf5aa, 0xf58e, 0xf592] {
        let q = u(o);
        if q != 0 && *((q + 0x19e) as *const usize) == 0 {
            *((m + o) as *mut usize) = 0;
        }
    }
    'instante: while n != 0 {
        if byte(m, 0x8eb4) == 1 {
            em1(0x6d1210, m, 0);
            n = n.wrapping_sub(1);
            continue;
        }
        match byte(m, 0x8eb3) {
            1..=3 => {
                em1(0x6be710, m, 4);
                if byte(m, 0x8eb3) == 0 {
                    crate::comparacao::ramo("laco penaltis decididos");
                    encerrar_clubes(m);
                    em1(0x6ce460, m, *w(0x8ed4) as u16 as u32);
                    em0(0x6d4a20, m);
                    return 2;
                }
                n = n.wrapping_sub(1);
                continue;
            }
            4..=6 => {
                em1(0x6be710, m, 3);
                if byte(m, 0x8eb3) == 0 {
                    crate::comparacao::ramo("laco penaltis decididos sem resumo");
                    encerrar_clubes(m);
                    em0(0x6d4a20, m);
                    return 2;
                }
                n = n.wrapping_sub(1);
                continue;
            }
            _ => {}
        }
        if byte(m, 0x1ca) == 0 && byte(m, 0x8eb2) == 8 && byte(m, 0x1a8) == 0 {
            narrar(if *w(0x8ed0) > 1 { 0x1f74 } else { 0x212f }, byte(m, 0x8eae) as u8 as u32);
            *b(0x1ca) = 1;
            continue;
        }
        if byte(m, 0x8eb2) != 0 && byte(m, 0x8eb4) == 0 && byte(m, 0x1a8) == 0 {
            // a substituicao sorteada: um jogador em campo ate 20 tentativas (0x7164b0 libera os de fora)
            let mut tentativas = 0i8;
            let ficha = |a: i8, c: i8| m + 0x4796 + ((a as i32 * 20 + c as i32) * 0x1be) as usize;
            let (a, c) = loop {
                let a = sortear(2) as i8;
                let c = sortear(0x14) as i8;
                tentativas += 1;
                if byte(ficha(a, c), 0x19) >= 0 {
                    break (a, c);
                }
                if tentativas >= 0x14 {
                    break (a, c);
                }
                if em2(0x7164b0, ficha(a, c), 1, 0) as u8 != 0 {
                    break (a, c);
                }
            };
            if tentativas < 0x14 && byte(ficha(a, c), 0x19) >= 0 {
                em1(0x6d1210, m, ficha(a, c) as u32);
                if byte(m, 0x8eb4) != 0 {
                    continue;
                }
            }
        }
        if byte(m, 0x8eb4) == 2 {
            *b(0x8eb4) = 0;
        }
        // o lance marcado
        let q = u(0x4761);
        if byte(m, 0x475a) != 0 && q != 0 {
            let cl = byte(q, 0x27);
            if byte(m, 0x8eb2) != 0 {
                let d = u(0xf582);
                let (by, ly) = (byte(m, 0x8ea8), byte(m, 0x475c));
                if d != 0 && byte(d, 0x27) as i32 == (cl == 0) as i32 && ((by > ly && cl == 1) || (by < ly && cl == 0)) {
                    crate::comparacao::ramo("laco lance descartado");
                    em0(0x6bba10, m);
                } else if sortear(5) == 0 && em0(0x6bac40, m) as u8 != 0 {
                    continue;
                }
            } else if sortear(7) == 0 {
                let v = if byte(m, 0x475a) != 0 && u(0x4761) != 0 { (byte(u(0x4761), 0x27) == 0) as u32 } else { 0xff };
                narrar(0x20ef, v);
                if em0(0x6bac40, m) as u8 != 0 {
                    continue;
                }
            }
        }
        if byte(m, 0x1a8) == 0 {
            if byte(m, 0x8eb2) != 0 {
                em0(0x6c54a0, m);
            }
            let d = u(0xf582);
            if byte(m, 0x1a8) == 0 && byte(m, 0x8eb2) != 0 && (d == 0 || byte(d, 0x19) < 0) {
                crate::comparacao::ramo("laco cobrador da bola parada");
                let c = em3(0x6ceb90, m, byte(m, 0x8eae) as u8 as u32, byte(m, 0x8eb2) as u8 as u32, 0) as usize;
                *((m + 0xf582) as *mut usize) = c;
                em2(0x6f46b0, c, byte(m, 0x8ea7) as u8 as u32, byte(m, 0x8ea8) as u8 as u32);
                *((u(0xf582) + 0x19c) as *mut i16) = 200;
                em0(0x6f0630, u(0xf582));
                let d = if byte(m, 0x8eae) != 0 { 1 } else { 0xa };
                em4(0x6f4920, u(0xf582), byte(m, 0x8ea7) as u8 as u32, byte(m, 0x8ea8) as u8 as u32, 4, d);
            }
        }
        *w(0x8ed2) = (*w(0x8ed2)).wrapping_add(1);
        if *w(0x8ed2) >= 1 || byte(m, 0x1a8) != 0 {
            n = n.wrapping_sub(1);
            let limite = byte(m, 0xf638);
            if limite != -1 && (limite as i16).wrapping_mul(11) < *w(0x8ed0) {
                crate::comparacao::ramo("laco abandono");
                *b(0xf5bd) = 0xfd;
                *b(0xf5f3) = 0xfd;
                *((u(0x4792) + 0x43) as *mut u8) = 0xfd;
                *b(0x8eb2) = 0;
                narrar(0x217b, 0);
                return 8;
            }
            if byte(m, 0x8ebb) != 0 {
                em1(0x6ce460, m, 0x3de);
            }
            *w(0x8ed0) = (*w(0x8ed0)).wrapping_add(1);
            *w(0x8ed2) = 0;
            match em0(0x6bd930, m) as i8 {
                0 => {
                    for base in [0x4796, 0x6a6e] {
                        for k in 0..20 {
                            let f = m + base + k * 0x1be;
                            if byte(f, 0x19) >= 0 {
                                em0(0x6f55c0, f);
                            }
                        }
                    }
                }
                1..=3 => return 4,
                4 => {
                    // fim da partida
                    em0(0x6d0f80, m);
                    let t = u(0x4792);
                    let tb = |o: usize| (t + o) as *mut u8;
                    let ag = byte(t, 0x4b);
                    if ag != -1 {
                        if byte(t, 0x47) != -1 {
                            *tb(0x4b) = (*tb(0x47)).wrapping_add(ag as u8);
                            *tb(0x4c) = (*tb(0x4c)).wrapping_add(*tb(0x48));
                        } else {
                            *tb(0x4b) = (*tb(0x43)).wrapping_add(ag as u8);
                            *tb(0x4c) = (*tb(0x4c)).wrapping_add(*tb(0x44));
                        }
                    }
                    if *(t as *const i32) != -2 {
                        em1(0x896290, *(0xde1d08 as *const usize), t as u32);
                    }
                    let p = byte(m, 0x8eb3);
                    if p != 0 {
                        crate::comparacao::ramo("laco fim para os penaltis");
                        *w(0x8ed0) = (*w(0x8ed0) as i32 / 30 * 30) as i16;
                        narrar(if p == 1 { 0x2003 } else { 0x2002 }, 0);
                        return 4;
                    }
                    encerrar_clubes(m);
                    em0(0x6d4a20, m);
                    return 2;
                }
                r => {
                    // ponytail: o original para o jogo com "Error" (match_eng.cpp, linha 0x776); a 0x6bd930 so devolve 0..=4
                    linha(&format!("motor: laco_do_motor: periodo devolveu {}", r));
                    std::process::abort();
                }
            }
        }
        // bola rolando ou parada: so acrescimos
        if byte(m, 0x8eb2) == 0 && {
            em0(0x6bbe40, m);
            byte(m, 0x8eb2) != 0
        } {
            acrescimos();
            continue;
        }
        if u(0xf5aa) != u(0xf582) && u(0xf582) != 0 {
            *((m + 0xf5aa) as *mut usize) = 0;
            *((m + 0xf5a6) as *mut usize) = 0;
        }
        em2(0x6cf750, m, byte(m, 0x8eae) as u8 as u32, byte(m, 0x8ebb) as u8 as u32);
        for base in [0x4796, 0x6a6e] {
            for k in 0..20 {
                let f = m + base + k * 0x1be;
                if byte(f, 0x19) >= 0 {
                    em0(0x6f4aa0, f);
                }
            }
        }
        em0(0x6c3aa0, m);
        em0(0x6cd660, m);
        em0(0x6bd760, m);
        *((m + 0x91de) as *mut usize) = 0;
        perto_da_bola(m);
        *b(2) = 0xff;
        *b(3) = 0xff;
        let (mut vx, mut vy) = (byte(m, 0x8ea7), byte(m, 0x8ea8));
        if *((m + 0x9766) as *const u32) & 0x1000 != 0 {
            em1(0x6bd6b0, m, 0);
        }
        if *((m + 0xb049) as *const u32) & 0x1000 != 0 {
            em1(0x6bd6b0, m, 1);
        }
        let lista = |i: i32| *((m + 0x8f7e + i as usize * 4) as *const usize);
        let mut i = 0i8;
        while i < byte(m, 0x8eb8) {
            let q = lista(i as i32);
            *((q + 0x18) as *mut i8) = i;
            *((q + 0x101) as *mut u8) = 0;
            *((q + 0x1aa) as *mut usize) = 0;
            i += 1;
        }
        // as marcacoes
        let mut i = 0i8;
        while i < byte(m, 0x8eb8) {
            let q = lista(i as i32);
            i += 1;
            let lado = byte(q, 0x27);
            if em1(0x5b57c0, m + 0x91e2 + lado as isize as usize * 0x18e3, byte(q, 0x19) as u8 as u32) & 1 != 0 {
                continue;
            }
            let mut alvo = em0(0x7160f0, q) as usize;
            if alvo == 0 {
                if em0(0x717410, q) as u16 == 1 || q == u(0xf582) {
                    continue;
                }
                let tab = || em0(0x717410, q);
                let by = byte(m, 0x8ea8);
                let marcar = if *((m + 0x9766 + lado as isize as usize * 0x18e3) as *const u32) & 0x4000 != 0 {
                    true
                } else if byte(m, 0x8eae) == lado && if lado == 0 { by >= 3 } else { by <= 8 } {
                    false
                } else if (tab() & 4 != 0 || tab() & 2 != 0) && em0(0x714d00, q) as u8 != 0 {
                    true
                } else {
                    let fundo = if tab() & 8 == 0 && tab() & 0x10 == 0 {
                        false
                    } else if lado == 0 {
                        by < 3
                    } else {
                        by > 8
                    };
                    fundo || na_area_atacada((lado != 1) as i8, byte(m, 0x8ea7), by)
                };
                if !marcar {
                    continue;
                }
                alvo = em1(0x6bc030, m, q as u32) as usize;
                if alvo == 0 {
                    continue;
                }
            }
            *((q + 0x1aa) as *mut usize) = alvo;
            let k = byte(alvo, 0x101);
            if k < 3 {
                *((alvo + 0x1ae + k as usize * 4) as *mut usize) = q;
                *((alvo + 0x101) as *mut u8) = (k + 1) as u8;
            }
        }
        // a decisao de cada jogador ativo
        let mut cansou = false;
        let mut i = 0i32;
        while i < byte(m, 0x8eb8) as i32 {
            let q = lista(i);
            if *((q + 0x1aa) as *const usize) == 0 || byte(m, 0x8eae) == byte(q, 0x27) {
                if !cansou || byte(q, 0x104) as i32 > sortear(10) {
                    em0(0x7103e0, q);
                }
                let q = lista(i);
                let acao = *((q + 0x198) as *const i16);
                if acao == 0x11 || acao == 0x12 {
                    let v = f32_em(q + if acao == 0x11 { 0x71 } else { 0xc1 }) as f64;
                    if sortear(ftol(f32_em(0x96d2c0) as f64 - v * f32_em(0x968960) as f64)) == 0 {
                        cansou = true;
                    }
                }
                if byte(m, 0x8eb2) != 0 || byte(m, 0x1a8) != 0 {
                    acrescimos();
                    if byte(m, 0x8eb5) != 0 {
                        em0(0x6d0b60, m);
                    }
                    continue 'instante;
                }
                // o chute pendente e a disputa marcada
                let mut feito = false;
                if byte(m, 0x1c5) != 0 {
                    *b(0x1c5) = (*b(0x1c5)).wrapping_sub(1);
                    if byte(m, 0x1c5) == 0 && byte(m, 0x8ea9) == 4 && u(0x477e) == 0 {
                        crate::comparacao::ramo("laco disputa marcada");
                        em2(0x6c9cd0, m, byte(m, 0x1c6) as u8 as u32, byte(m, 0x1c7) as u8 as u32);
                        feito = true;
                    }
                }
                let c = u(0x477e);
                if !feito && byte(m, 0x1c5) == 0 && c != 0 {
                    let chutar = |c: usize| em3(0x6f6c00, c, *w(0x8ed6) as u16 as u32, byte(m, 0x1c8) as u8 as u32, byte(m, 0x1c9) as u8 as u32);
                    if byte(c, 0x102) == byte(m, 0x8ea7) && byte(c, 0x103) == byte(m, 0x8ea8) {
                        chutar(c);
                    } else if c == u(0xf57a) && byte(c, 0x2b) == 0 && byte(c, 0x105) != 0 && byte(c, 0x107) != 0 {
                        crate::comparacao::ramo("laco batedor se aproxima");
                        *b(0x1c5) = 1;
                        em1(0x713fc0, c, 1);
                        *b(0x1c5) = 0;
                        let c2 = u(0x477e);
                        if c2 == c && byte(c2, 0x102) == byte(m, 0x8ea7) && byte(c2, 0x103) == byte(m, 0x8ea8) {
                            chutar(c2);
                        }
                    }
                }
                if byte(m, 0x8eb2) != 0 || byte(m, 0x1a8) != 0 {
                    acrescimos();
                    continue 'instante;
                }
                if u(0x91de) != 0 {
                    continue 'instante;
                }
                let q = lista(i);
                let acao = *((q + 0x198) as *const i16);
                if (acao == 0x104 || acao == 0x76) && i < 0x16 {
                    crate::comparacao::ramo("laco entra na lista");
                    let k = byte(m, 0x8eb8);
                    *((m + 0x8f7e + k as isize as usize * 4) as *mut usize) = q;
                    *b(0x8eb8) = (k + 1) as u8;
                }
                em0(0x6c3aa0, m);
                if vx != byte(m, 0x8ea7) || vy != byte(m, 0x8ea8) {
                    perto_da_bola(m);
                    vx = byte(m, 0x8ea7);
                    vy = byte(m, 0x8ea8);
                }
            }
            i += 1;
        }
    }
    1
}

unsafe extern "thiscall" fn comparar_laco_do_motor(m: usize, n: u32) -> u32 {
    // ponytail: orquestrador sem conta propria; a fita dele fotografaria o motor a cada decisao de
    // jogador. Validado pela carreira (Rust x original); as rotinas que ele chama seguem comparadas.
    orig::<unsafe extern "thiscall" fn(usize, u32) -> u32>(320)(m, n)
}
