// ---------- %%LEVA%%a leva: a copa e as fases: a árvore dos confrontos da copa, a primeira fase do grupo e a fase atual (Cup.cpp, comp.cpp) ----------

unsafe fn cqd7_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqd7_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqd7_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqd7_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

unsafe fn cqd7_pw(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, v)
}

unsafe fn cqd7_pb(a: usize, v: u8) {
    *(a as *mut u8) = v
}

/// O confronto `p` (0x35 bytes) da rodada `r` da copa (vetores em [+0xad]).
unsafe fn cqd7_confronto(copa: usize, r: i32, p: i32) -> usize {
    cqd7_d(cqd7_d(copa + 0xad) + (r as isize * 4) as usize).wrapping_add((p as isize * 0x35) as usize)
}

/// A rodada `r` da copa (0x68 bytes na tabela [+0xa3]).
unsafe fn cqd7_rodada(copa: usize, r: i32) -> usize {
    cqd7_d(copa + 0xa3).wrapping_add((r as isize * 0x68) as usize)
}

/// { 0x518790 } (thiscall, ret 0, Cup.cpp): `Cup::map_fixture_tree()`, da criação das copas (22 chamadas até
/// 2002-06-01). Os vetores dos confrontos de cada rodada (0x944e46: 4 por rodada e 0x35 por confronto; sem memória, a
/// caixa "Memory Allocation Error" da linha 0x8b ou 0x9e e o jogo para; rodada sem confrontos, a caixa da linha 0x95 e
/// -1); a marca +5 de cada time (+0xa7, 6 bytes) e +0xab/+0xac zeradas. Com clubes (+0x45): cada confronto vazio (os
/// times e as vagas −1, +8 = 0, +0x14..+0x31 = 0xff, +0x32..+0x34 = 0). Nas rodadas até a penúltima (a antepenúltima com
/// a última do tipo 0x8c): a ordem dos confrontos (embaralhada aos pares sorteados se a rodada seguinte tem menos da
/// metade); cada um ganha a vaga na rodada seguinte: pelo par (rodada +0xd bit 0x200: p/2) ou sorteada (com o bit 0x400,
/// a primeira vaga toda livre a partir dela), o lado pela moeda (0x90cfc0(2); ocupada, a vaga seguinte e outra moeda);
/// sem vaga livre, uma vaga sorteada duas rodadas adiante (sem ela, o jogo para: linhas 0x175 e 0x1a0). O confronto
/// guarda a vaga (+6), o lado (+8: 1 mandante) e a rodada de destino (+0x12); a vaga guarda o confronto (+0xa/+0xc) e a
/// rodada de origem (+0xe/+0x10). Depois os times de cada rodada (+0x1c, a partir de +0x1e) entram nos confrontos dela:
/// o primeiro pelo par ou sorteado (0x400: o primeiro todo livre), o lado pela moeda, e numa vaga ocupada o time da
/// nação +0x24 vira mandante (sem vaga, a caixa da linha 0x1c2 ou 0x222 e -1). Sem confrontos na primeira rodada, +0xb1
/// = 1. Por fim o número do primeiro confronto de cada rodada (+0x5b), de trás para a frente. Devolve 1.
unsafe extern "thiscall" fn copa_arvore_dos_confrontos(this: usize) -> i32 {
    let reservar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let t = reservar((cqd7_w(this + 0x3c) as i32 as u32) << 2);
    cqd7_pd(this + 0xad, t);
    if t == 0 {
        cpq11_sem_memoria(0x8b, 0x9b5600);
    }
    let mut i = 0i32;
    while i < cqd7_w(this + 0x3c) as i32 {
        let n = cqd7_w(cqd7_rodada(this, i) + 0x1a);
        if n < 1 {
            aviso_em(0x9b55ac, 0x95);
            return -1;
        }
        let v = reservar((n as i32 * 0x35) as u32);
        cqd7_pd(cqd7_d(this + 0xad) + i as usize * 4, v);
        if cqd7_d(cqd7_d(this + 0xad) + i as usize * 4) == 0 {
            cpq11_sem_memoria(0x9e, 0x9b55d8);
        }
        i += 1;
    }
    if cqd7_d(this + 0xa7) != 0 {
        let mut k = 0i32;
        while k < cqd7_w(this + 0x3e) as i32 {
            cqd7_pb(cqd7_d(this + 0xa7) + k as usize * 6 + 5, 0);
            k += 1;
        }
    }
    cqd7_pb(this + 0xab, 0);
    cqd7_pb(this + 0xac, 0);
    if cqd7_d(this + 0x45) == 0 {
        return 1;
    }
    let mut i = 0i32;
    while i < cqd7_w(this + 0x3c) as i32 {
        let mut j = 0i32;
        while j < cqd7_w(cqd7_rodada(this, i) + 0x1a) as i32 {
            let c = cqd7_confronto(this, i, j);
            for k in [0usize, 2, 6, 0xa, 0xc, 0xe, 0x10, 0x12] {
                cqd7_pw(c + k, 0xffff);
            }
            cqd7_pw(c + 8, 0);
            for k in 0x14..=0x31usize {
                cqd7_pb(c + k, 0xff);
            }
            cqd7_pb(c + 0x32, 0);
            cqd7_pb(c + 0x33, 0);
            cqd7_pb(c + 0x34, 0);
            j += 1;
        }
        i += 1;
    }
    let nr = cqd7_w(this + 0x3c);
    let m = (if cqd7_w(cqd7_rodada(this, nr as i32).wrapping_sub(0x61)) == 0x8c { nr.wrapping_sub(1) } else { nr }) as i32;
    let mut r = 0i32;
    while r < m - 1 {
        let n = cqd7_w(cqd7_rodada(this, r) + 0x1a);
        let ordem = if n != 0 { reservar(n as i32 as u32) } else { 0 };
        let mut k = 0i32;
        while k < cqd7_w(cqd7_rodada(this, r) + 0x1a) as i32 {
            cqd7_pb(ordem + k as usize, k as u8);
            k += 1;
        }
        let nn = cqd7_w(cqd7_rodada(this, r) + 0x1a) as i32;
        let prox = cqd7_w(cqd7_rodada(this, r) + 0x82) as i32;
        if prox < nn / 2 && nn > 0 {
            let (mut q, mut conta) = (0i32, nn);
            loop {
                let a = sorteio(conta) as usize;
                let b = sorteio(cqd7_w(cqd7_rodada(this, r) + 0x1a) as i32) as usize;
                let x = cqd7_b(ordem + a);
                cqd7_pb(ordem + a, cqd7_b(ordem + b));
                cqd7_pb(ordem + b, x);
                q += 1;
                conta = cqd7_w(cqd7_rodada(this, r) + 0x1a) as i32;
                if q >= conta {
                    break;
                }
            }
        }
        let mut k = 0i32;
        while k < cqd7_w(cqd7_rodada(this, r) + 0x1a) as i32 {
            let rr = cqd7_rodada(this, r);
            let p = cqd7_b(ordem + k as usize) as i8 as i32;
            let mut s: i32;
            let mut inicio: i32;
            if cqd7_w(rr + 0x75) as u16 & 0x200 != 0 {
                s = p / 2; // cqd7: a vaga pelo par
                inicio = s;
            } else {
                s = sorteio(cqd7_w(rr + 0x82) as i32);
                inicio = s;
                if cqd7_w(rr + 0x75) as u16 & 0x400 != 0 {
                    loop {
                        let v = cqd7_confronto(this, r + 1, s as i16 as i32);
                        if cqd7_w(v + 0xa) == -1 && cqd7_w(v + 0xc) == -1 {
                            inicio = s;
                            break;
                        }
                        s += 1;
                        if s as i16 >= cqd7_w(rr + 0x82) {
                            s = 0;
                        }
                        if s as i16 == inicio as i16 {
                            break;
                        }
                    }
                }
            }
            let (mut s, inicio) = (s, inicio); // cqd7: a vaga inicial na rodada seguinte
            let c = cqd7_confronto(this, r, p);
            loop {
                let moeda = sorteio(2);
                let v = cqd7_confronto(this, r + 1, s as i16 as i32);
                let primeiro_a = moeda != 0; // cqd7: a moeda do lado na rodada seguinte
                let lado = if primeiro_a {
                    if cqd7_w(v + 0xa) == -1 { Some(true) } else if cqd7_w(v + 0xc) == -1 { Some(false) } else { None }
                } else if cqd7_w(v + 0xc) == -1 {
                    Some(false)
                } else if cqd7_w(v + 0xa) == -1 {
                    Some(true)
                } else {
                    None
                };
                if let Some(a) = lado {
                    crate::comparacao::ramo("cqd_7 vaga na rodada seguinte");
                    cqd7_pw(v + if a { 0xa } else { 0xc }, p as u16);
                    cqd7_pw(v + if a { 0xe } else { 0x10 }, r as u16);
                    cqd7_pw(c + 6, s as u16);
                    cqd7_pw(c + 8, a as u16);
                    cqd7_pw(c + 0x12, (r + 1) as u16);
                    break;
                }
                if (s as i16 as i32) < cqd7_w(cqd7_rodada(this, r) + 0x82) as i32 - 1 {
                    s += 1;
                } else {
                    s = 0;
                }
                if inicio as i16 == s as i16 {
                    // sem vaga livre: uma vaga sorteada duas rodadas adiante
                    if r >= m - 2 {
                        erro_em(0x9b55ac, 0x175);
                    }
                    let mut t = sorteio(cqd7_w(cqd7_rodada(this, r) + 0xea) as i32);
                    let ini = t;
                    loop {
                        let v = cqd7_confronto(this, r + 2, t as i16 as i32);
                        let a = if cqd7_w(v + 0xa) == -1 {
                            true
                        } else if cqd7_w(v + 0xc) == -1 {
                            false
                        } else {
                            if (t as i16 as i32) < cqd7_w(cqd7_rodada(this, r) + 0xea) as i32 - 1 {
                                t += 1;
                            } else {
                                t = 0;
                            }
                            if t as i16 == ini as i16 {
                                erro_em(0x9b55ac, 0x1a0);
                            }
                            continue;
                        };
                        cqd7_pw(v + if a { 0xa } else { 0xc }, p as u16);
                        cqd7_pw(v + if a { 0xe } else { 0x10 }, r as u16);
                        cqd7_pw(c + 6, t as u16);
                        cqd7_pw(c + 8, a as u16);
                        break;
                    }
                    cqd7_pw(c + 0x12, (r + 2) as u16);
                    break;
                }
            }
            k += 1;
        }
        if ordem != 0 {
            soltar(ordem);
        }
        r += 1;
    }
    if cqd7_w(cqd7_rodada(this, 0) + 0x1a) == 0 {
        cqd7_pb(this + 0xb1, 1);
        return 1;
    }
    // os times de cada rodada nos confrontos dela; `inicio` segue de um time para o outro, como na original
    let mut inicio: i32 = -1;
    let mut r = 0i32;
    while r < m {
        if cqd7_w(cqd7_rodada(this, r) + 0x1c) > 0 {
            let mut q = 0i32;
            loop {
                let rr = cqd7_rodada(this, r);
                let mut di: i32;
                if cqd7_w(rr + 0xd) as u16 & 0x200 != 0 {
                    di = 0;
                    if cqd7_w(rr + 0x1a) <= 0 {
                        aviso_em(0x9b55ac, 0x1c2);
                        return -1;
                    }
                } else {
                    di = sorteio(cqd7_w(rr + 0x1a) as i32);
                    inicio = di;
                    let rr = cqd7_rodada(this, r);
                    if cqd7_w(rr + 0xd) as u16 & 0x400 != 0 {
                        loop {
                            let c = cqd7_confronto(this, r, di as i16 as i32);
                            if cqd7_w(c) == -1 && cqd7_w(c + 2) == -1 {
                                inicio = di;
                                break;
                            }
                            di += 1;
                            if di as i16 >= cqd7_w(rr + 0x1a) {
                                di = 0;
                            }
                            if di as i16 == inicio as i16 {
                                break;
                            }
                        }
                    }
                }
                loop {
                    let moeda = sorteio(2);
                    let x = di as i16 as i32;
                    let c = cqd7_confronto(this, r, x);
                    let livre2 = || cqd7_w(c + 2) == -1 && cqd7_w(c + 0xc) == -1;
                    let primeiro_0 = moeda != 0; // cqd7: a moeda do lado na primeira rodada
                    let v0 = cqd7_w(c) as u16;
                    let livre0 = v0 == 0xffff && cqd7_w(c + 0xa) == -1;
                    let lado = if primeiro_0 {
                        if livre0 { Some(0usize) } else if livre2() { Some(2) } else { None }
                    } else if livre2() {
                        Some(2)
                    } else if livre0 {
                        Some(0)
                    } else {
                        None
                    };
                    if let Some(o) = lado {
                        crate::comparacao::ramo("cqd_7 time na primeira rodada");
                        cqd7_pw(c + o, (cqd7_w(cqd7_rodada(this, r) + 0x1e) as u16).wrapping_add(q as u16));
                        break;
                    }
                    // a vaga ocupada: o time da nação +0x24 no lado de fora vira mandante
                    if cqd7_d(this + 0x24) as u32 != u32::MAX && v0 != 0xffff {
                        let fora = cqd7_w(c + 2);
                        if fora != -1 {
                            let clube = cqd7_d(cqd7_d(this + 0xa7) + (fora as isize * 6) as usize);
                            if clube != 0 {
                                let nacao = cqd7_d(clube + 0x69);
                                if nacao != 0 && cqd7_d(nacao) == cqd7_d(this + 0x24) {
                                    cqd7_pw(c, fora as u16);
                                    cqd7_pw(c + 2, v0);
                                }
                            }
                        }
                    }
                    if x < cqd7_w(cqd7_rodada(this, r) + 0x1a) as i32 - 1 {
                        di += 1;
                    } else {
                        di = 0;
                    }
                    if inicio as i16 == di as i16 {
                        aviso_em(0x9b55ac, 0x222);
                        return -1;
                    }
                }
                q += 1;
                if q >= cqd7_w(cqd7_rodada(this, r) + 0x1c) as i32 {
                    break;
                }
            }
        }
        r += 1;
    }
    let nr = cqd7_w(this + 0x3c);
    if nr > 0 && cqd7_w(cqd7_rodada(this, nr as i32).wrapping_sub(0x61)) == 0x8c {
        let mut k = nr.wrapping_sub(3) as i32;
        let mut n: u8 = 5;
        while k >= 0 {
            cqd7_pb(cqd7_rodada(this, k) + 0x5b, n);
            n = n.wrapping_add(cqd7_b(cqd7_rodada(this, k) + 0x1a));
            k -= 1;
        }
    } else {
        let mut n = cqd7_b(cqd7_rodada(this, nr as i32).wrapping_sub(0x50)).wrapping_add(1);
        let mut k = nr.wrapping_sub(2) as i32;
        while k >= 0 {
            cqd7_pb(cqd7_rodada(this, k) + 0x5b, n);
            n = n.wrapping_add(cqd7_b(cqd7_rodada(this, k) + 0x1a));
            k -= 1;
        }
    }
    1
}

/// O comparador da árvore dos confrontos: só a original (ponytail: aloca os vetores dos confrontos e sorteia; não
/// roda duas vezes). Confere o que ela garante quando devolve 1 com clubes: cada confronto das rodadas mapeadas aponta
/// (+6 vaga, +8 lado, +0x12 rodada uma ou duas adiante) uma vaga que aponta de volta para ele (+0xa/+0xe mandante,
/// +0xc/+0x10 visitante).
unsafe extern "thiscall" fn comparar_copa_arvore_dos_confrontos(this: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> i32>(%%I0%%);
    let r = o(this);
    if r != 1 || cqd7_d(this + 0x45) == 0 {
        return r;
    }
    let nr = cqd7_w(this + 0x3c);
    let m = (if cqd7_w(cqd7_rodada(this, nr as i32).wrapping_sub(0x61)) == 0x8c { nr.wrapping_sub(1) } else { nr }) as i32;
    let mut erro: Option<(i32, i32)> = None;
    'fora: for rd in 0..(m - 1).max(0) {
        for p in 0..cqd7_w(cqd7_rodada(this, rd) + 0x1a) as i32 {
            let c = cqd7_confronto(this, rd, p);
            let destino = cqd7_w(c + 0x12) as i32;
            let a = cqd7_w(c + 8) == 1;
            let ok = (destino == rd + 1 || destino == rd + 2) && {
                let v = cqd7_confronto(this, destino, cqd7_w(c + 6) as i32);
                cqd7_w(v + if a { 0xa } else { 0xc }) as i32 == p && cqd7_w(v + if a { 0xe } else { 0x10 }) as i32 == rd
            };
            if !ok {
                erro = Some((rd, p));
                break 'fora;
            }
        }
    }
    PLACAR.registrar(erro.is_none(), || format!("copa_arvore_dos_confrontos: o confronto (rodada, índice) {:?} sem a vaga de volta", erro));
    r
}

/// A fase `k` da competição como a original a lê no meio das rotinas: −1 a própria, 0..+0x30 em [+0xc]; fora disso a
/// caixa da linha 0x36e de comp.cpp e 0.
unsafe fn cqd7_fase(comp: usize, k: i32) -> usize {
    if k < -1 || k > cqd7_d(comp + 0x30) as i32 {
        aviso_em(0x99b80c, 0x36e);
        return 0;
    }
    if k == -1 { comp } else { cqd7_d(cqd7_d(comp + 0xc) + k as usize * 4) }
}

/// { 0x48d2b0 } (thiscall, ret 4, comp.cpp): `Comp::get_first_stage_of_group(k)`, das telas das competições
/// (0x4947a0): da fase `k` para trás, enquanto as fases têm o mesmo grupo (0x4b01d0 do id +0x3a: push id; add esp, 4 ->
/// cdecl) e o índice é maior que −1, o índice da primeira delas (o próprio k sem grupo).
unsafe extern "thiscall" fn competicao_primeira_fase_do_grupo(this: usize, k: i32) -> i32 {
    let grupo: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x4b01d0usize);
    let mut e = k;
    let g = grupo(cqd7_w(cqd7_fase(this, e) + 0x3a) as i32);
    if g > -1 && e > -1 {
        loop {
            let f = cqd7_fase(this, e - 1);
            if grupo(cqd7_w(f + 0x3a) as i32) != g {
                break;
            }
            e -= 1;
            if e <= -1 {
                break;
            }
        }
    }
    e
}

/// O comparador da primeira fase do grupo: com o índice válido (−1..+0x30), as duas (só leem) e o mesmo índice; fora
/// dele só a original (a caixa).
unsafe extern "thiscall" fn comparar_competicao_primeira_fase_do_grupo(this: usize, k: i32) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32) -> i32>(%%I1%%);
    if k < -1 || k > cqd7_d(this + 0x30) as i32 {
        return o(this, k);
    }
    let r = o(this, k);
    let n = competicao_primeira_fase_do_grupo(this, k);
    PLACAR.registrar(r == n, || format!("competicao_primeira_fase_do_grupo({}): original={} rust={}", k, r, n));
    r
}

/// { 0x48cec0 } (thiscall, ret 0, comp.cpp): `Comp::get_current_stage()`, das telas das competições (0x48f8e0): a
/// primeira fase (−1 a própria, até +0x30) que não acabou (método 0x10 com (0, 1) = 0); se é uma copa (+0x43 = 1) que
/// não começou (+0xab = 0), volta ao começo do grupo dela pela fase anterior (0x4b01d0 e 0x48d200: push k; mov ecx,
/// this -> thiscall, ret 4). Todas acabadas: o começo do grupo da última. Fase inválida no fim: a caixa da linha 0x36e
/// e a própria competição; fase nula: a própria.
unsafe extern "thiscall" fn competicao_fase_atual(this: usize) -> usize {
    let grupo: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x4b01d0usize);
    let fase: unsafe extern "thiscall" fn(usize, i32) -> usize = core::mem::transmute(0x48d200usize);
    let voltar = |s0: i32| -> i32 {
        let mut s = s0;
        let g = grupo(cqd7_w(cqd7_fase(this, s) + 0x3a) as i32);
        if g > -1 && s > -1 {
            loop {
                let f = fase(this, s - 1);
                if grupo(cqd7_w(f + 0x3a) as i32) != g {
                    break;
                }
                s -= 1;
                if s <= -1 {
                    break;
                }
            }
        }
        s
    };
    let fim = |s: i32| -> usize {
        if s < -1 || s > cqd7_d(this + 0x30) as i32 {
            aviso_em(0x99b80c, 0x36e);
            return this;
        }
        let f = if s == -1 { this } else { cqd7_d(cqd7_d(this + 0xc) + s as usize * 4) };
        if f != 0 { f } else { this }
    };
    let mut e: i32 = -1;
    if cqd7_d(this + 0x30) as i32 >= -1 {
        loop {
            let f = cqd7_fase(this, e);
            let acabou: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(cqd7_d(cqd7_d(f) + 0x10));
            if acabou(f, 0, 1) as u8 == 0 {
                if f == 0 {
                    break;
                }
                if cqd7_b(f + 0x43) == 1 && cqd7_b(f + 0xab) == 0 && e > -1 {
                    return fim(voltar(e - 1));
                }
                return f;
            }
            e += 1;
            if e > cqd7_d(this + 0x30) as i32 {
                break;
            }
        }
    }
    fim(voltar(cqd7_d(this + 0x30) as i32))
}

/// O comparador da fase atual: só a original (ponytail: o método 0x10 das fases pode refazer a tabela; não roda duas
/// vezes); a cópia se prova no modo Rust pelas telas.
unsafe extern "thiscall" fn comparar_competicao_fase_atual(this: usize) -> usize {
    orig::<unsafe extern "thiscall" fn(usize) -> usize>(%%I2%%)(this)
}
