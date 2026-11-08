// ---------- %%LEVA%%a leva: a copa: o sorteio pela semente, o sorteio que separa as nações e o começo da temporada da copa (Cup.cpp) ----------

unsafe fn cqb11_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// Tira os times dos confrontos da rodada `r` para o vetor de 7 bytes do sorteio (o time +0, o vínculo +2/+4, o
/// nível +6), como nas 0x51d2d0 e 0x51d9f0: o mandante (+0, com +0xe/+0xa) e o visitante (+2, com +0x10/+0xc); com
/// time, a entrada leva o nível do time (+4 do time; a semente), `com_time(vaga)` roda com a vaga ainda intacta e a
/// vaga vira −2 (livre); sem time (−1), a entrada fica −1 com 0xff no nível (o resto da entrada fica como estava).
/// Devolve quantas entradas (a conta de 32 bits, usada em 16).
unsafe fn cqb11_tirar_times(copa: usize, r: i32, vetor: usize, mut com_time: impl FnMut(usize)) -> i32 {
    let mut n: i32 = 0;
    let mut k: i32 = 0;
    while (k as i16) < cpq11_w(cpq11_rodada(copa, r) + 0x1a) {
        for (lado, ligado, jogo) in [(0usize, 0xeusize, 0xausize), (2, 0x10, 0xc)] {
            let t = cpq11_confronto(copa, r, k as i16 as i32);
            let e = cpq11_em(vetor, n as i16 as i32, 7);
            n = n.wrapping_add(1);
            let time = cpq11_w(t + lado);
            if time != -1 {
                cpq11_por_w(e, time as u16);
                cpq11_por_w(e + 2, cpq11_w(t + ligado) as u16);
                cpq11_por_w(e + 4, cpq11_w(t + jogo) as u16);
                *((e + 6) as *mut u8) = cqb11_b(cpq11_time(copa, time) + 4); // a semente do time
                com_time(t + lado);
                cpq11_por_w(t + lado, 0xfffe);
            } else {
                cpq11_por_w(e, 0xffff);
                *((e + 6) as *mut u8) = 0xff;
            }
        }
        k += 1;
    }
    n
}

/// A entrada `e` do vetor vai ao confronto `cur` da rodada `r`: no mandante (+0, +0xe, +0xa) ou no visitante (+2,
/// +0x10, +0xc); com vínculo (+2 da entrada ≠ −1), o confronto ligado na rodada seguinte ganha a rodada (+0x12), o
/// confronto sorteado (+6) e o lado (+8: 1 mandante, 0 visitante).
unsafe fn cqb11_colocar(copa: usize, r: i32, rodada: u32, e: usize, cur: i32, mandante: bool) {
    let t = cpq11_confronto(copa, r, cur as i16 as i32);
    let (lado, ligado, jogo) = if mandante { (0usize, 0xeusize, 0xausize) } else { (2, 0x10, 0xc) };
    cpq11_por_w(t + lado, cpq11_w(e) as u16);
    cpq11_por_w(t + ligado, cpq11_w(e + 2) as u16);
    cpq11_por_w(t + jogo, cpq11_w(e + 4) as u16);
    let seguinte = cpq11_w(e + 2);
    if seguinte != -1 {
        let v = cpq11_confronto(copa, seguinte as i32, cpq11_w(e + 4) as i32);
        cpq11_por_w(v + 0x12, rodada as u16);
        cpq11_por_w(v + 6, cur as u16);
        cpq11_por_w(v + 8, mandante as u16);
    }
}

/// Os times `a` e `b` (índices em [copa + 0xa7]) têm clube e são da mesma nação (+0x53).
unsafe fn cqb11_mesma_nacao(copa: usize, a: i16, b: i16) -> bool {
    let c1 = cpq11_us(cpq11_time(copa, a));
    if c1 == 0 {
        return false;
    }
    let c2 = cpq11_us(cpq11_time(copa, b));
    c2 != 0 && cpq11_d(c2 + 0x53) == cpq11_d(c1 + 0x53)
}

/// O passo da volta pelos confrontos: o próximo (em círculo, 16 bits); de volta ao sorteado, a primeira volta vira a
/// segunda (`segunda`), na segunda cai a regra (`regra`: a nação, ou o "sem a mesma nação" da 0x51d9f0) e, sem ela,
/// devolve false (a caixa).
unsafe fn cqb11_passo(copa: usize, r: i32, cur: &mut i32, inicio: i16, segunda: &mut bool, regra: &mut bool) -> bool {
    *cur = cur.wrapping_add(1);
    if (*cur as i16) >= cpq11_w(cpq11_rodada(copa, r) + 0x1a) {
        *cur = 0;
    }
    if *cur as i16 == inicio {
        if !*segunda {
            *segunda = true;
        } else if *regra {
            *regra = false;
        } else {
            return false;
        }
    }
    true
}

/// { 0x51d2d0 } (thiscall, ret 4, Cup.cpp): `Cup::sort_round_teams(rodada)`: o sorteio pela semente (regra 0x400 da
/// rodada). 1) Tira os times dos confrontos para o vetor de 7 bytes (malloc 0x944e46 de 7 × vagas +0x18; sem memória,
/// a caixa da linha 0xbf8 e o jogo para), cada um com a semente (+4 do time). 2) Ordena pela semente, a maior primeiro
/// (qsort 0x945333 com 0x51ebc0). 3) Cada entrada com time (até a primeira sem, nível 0xff) vai a um confronto
/// sorteado (0x90cfc0 dos confrontos), avançando em círculo: na primeira volta, só confrontos com as duas vagas sem
/// time (< 0): com as duas livres (−2), o lado sorteado (0x90cfc0(2) ≠ 0: mandante); só o mandante livre: mandante;
/// só o visitante: visitante. Da segunda volta em diante (a marca não volta a zero), qualquer vaga livre, mas com a
/// regra 4 da rodada (+0xb) o mandante livre não serve quando o visitante do confronto é da mesma nação, e o
/// visitante livre não serve quando o mandante é; dando a volta inteira de novo, cai a regra das nações (também para
/// as entradas seguintes) e, sem ela, a caixa da linha 0xcff e sai. Libera o vetor (0x9452ca).
unsafe extern "thiscall" fn sort_round_teams(copa: usize, rodada: u32) {
    let r = rodada as i16 as i32;
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let ordenar: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(0x945333usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    // os quatro: push n; add esp, 4 (malloc e sorteio) / push p; add esp, 4 (free) / 4 pushes; add esp, 0x10 (qsort) -> cdecl
    let vetor = alocar((cpq11_w(cpq11_rodada(copa, r) + 0x18) as i32).wrapping_mul(7) as u32);
    if vetor == 0 {
        cpq11_sem_memoria(0xbf8, 0x9b56dc); // "CUP::sort_round_teams() : Cannot allocate memory"
    }
    let mut nacao = cqb11_b(cpq11_rodada(copa, r) + 0xb) & 4 != 0; // a regra 4 (+0xb) da rodada
    let n = cqb11_tirar_times(copa, r, vetor, |_| {}) as i16;
    crate::comparacao::ramo("cqb_11 ordena pela semente");
    ordenar(vetor, n as i32, 7, 0x51ebc0); // cqb11: pela semente, a maior primeiro
    let mut segunda = false;
    let mut j: i16 = 0;
    while j < n {
        let e = cpq11_em(vetor, j as i32, 7);
        if cqb11_b(e + 6) == 0xff {
            break;
        }
        let s = sorteio(cpq11_w(cpq11_rodada(copa, r) + 0x1a) as i32);
        let inicio = s as i16;
        let mut cur = s;
        let mandante = loop {
            let t = cpq11_confronto(copa, r, cur as i16 as i32);
            let (casa, fora) = (cpq11_w(t), cpq11_w(t + 2));
            let mut lado: Option<bool> = None;
            if segunda {
                let w0 = cpq11_w(e);
                let livre = casa == -2 && !(nacao && w0 >= 0 && fora >= 0 && cqb11_mesma_nacao(copa, w0, fora));
                if fora == -2 {
                    if livre && sorteio(2) != 0 {
                        lado = Some(true);
                    } else if livre || !(nacao && w0 >= 0 && casa >= 0 && cqb11_mesma_nacao(copa, w0, casa)) {
                        lado = Some(false);
                    }
                } else if livre {
                    lado = Some(true);
                }
            } else if casa < 0 && fora < 0 {
                if fora == -2 {
                    if casa == -2 && sorteio(2) != 0 {
                        crate::comparacao::ramo("cqb_11 semente lado sorteado");
                        lado = Some(true); // cqb11: o lado sorteado
                    } else {
                        lado = Some(false);
                    }
                } else if casa == -2 {
                    lado = Some(true);
                }
            }
            if let Some(m) = lado {
                break m;
            }
            if !cqb11_passo(copa, r, &mut cur, inicio, &mut segunda, &mut nacao) {
                aviso_em(0x9b55ac, 0xcff);
                liberar(vetor);
                return;
            }
        };
        cqb11_colocar(copa, r, rodada, e, cur, mandante);
        j = j.wrapping_add(1);
    }
    liberar(vetor);
}

/// Comparador: sorteia (0x90cfc0) e só usa vetores temporários: lado a lado pelo estado do sorteador e pelos
/// confrontos de todas as rodadas (os vínculos vão às rodadas seguintes), a cópia em sem_contar.
unsafe extern "thiscall" fn comparar_sort_round_teams(copa: usize, rodada: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32)>(%%I0%%);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend(cpq11_confrontos(copa));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(copa, rodada),
        || crate::aleatorio::sem_contar(|| sort_round_teams(copa, rodada)));
    PLACAR.registrar(igual, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("sort_round_teams(rodada {}) diferencas em {:x?}", rodada as i16, dif)
    });
}

/// { 0x51d9f0 } (thiscall, ret 4, Cup.cpp): `Cup::check_round_teams_for_nation(rodada)`: o sorteio que separa as nações
/// (regra 4 da rodada sem a 0x400). Conta os times de cada nação num vetor zerado de [0xae2350] bytes (0x945501;
/// sem memória, a caixa da linha 0xd27 e o jogo para) enquanto tira os times dos confrontos (malloc de 7 × vagas, a
/// caixa da linha 0xd34), pelo +0 da nação (+0x53) do clube; o nível de cada entrada com time vira a conta da nação
/// dele (sem nação, fica a semente); libera as contas e ordena, a maior primeiro (as nações com mais times saem
/// antes). Cada entrada (até a primeira com 0xff) vai a um confronto sorteado, em círculo: na primeira volta, só
/// confrontos sem time nas duas vagas (com as duas livres, o lado sorteado; só uma livre, ela); da segunda em diante
/// (a marca fica), qualquer vaga livre, menos (enquanto valer a regra, que volta a valer a cada entrada) um confronto
/// com um time da mesma nação do clube da entrada; dando a volta inteira de novo, cai a regra e, sem ela, a caixa da
/// linha 0xe5c e sai. Libera o vetor.
unsafe extern "thiscall" fn check_round_teams_for_nation(copa: usize, rodada: u32) {
    let r = rodada as i16 as i32;
    let zerado: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(0x945501usize);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let ordenar: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(0x945333usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    // calloc: push 1; push [0xae2350]; add esp, 8 -> cdecl (n, 1); free e qsort: add esp, 0x14 cobre os dois -> cdecl
    let contas = zerado(cpq11_d(0xae2350), 1);
    if contas == 0 {
        cpq11_sem_memoria(0xd27, 0x9b5710); // "CUP::check_round_teams_for_nation() : cannot malloc array"
    }
    let vetor = alocar((cpq11_w(cpq11_rodada(copa, r) + 0x18) as i32).wrapping_mul(7) as u32);
    if vetor == 0 {
        cpq11_sem_memoria(0xd34, 0x9b56dc);
    }
    let n = cqb11_tirar_times(copa, r, vetor, |vaga| {
        let clube = cpq11_us(cpq11_time(copa, cpq11_w(vaga)));
        if clube != 0 {
            let nacao = cpq11_us(clube + 0x53);
            if nacao != 0 {
                let c = cpq11_us(nacao).wrapping_add(contas);
                *(c as *mut u8) = cqb11_b(c).wrapping_add(1);
            }
        }
    }) as i16;
    if n > 0 {
        let mut e = vetor;
        for _ in 0..n as i32 {
            if cpq11_w(e) != -1 {
                // ponytail: sem conferir o clube nulo, como a original (que leria o +0x53 dele)
                let nacao = cpq11_us(cpq11_us(cpq11_time(copa, cpq11_w(e))) + 0x53);
                if nacao != 0 {
                    crate::comparacao::ramo("cqb_11 conta da nacao");
                    *((e + 6) as *mut u8) = cqb11_b(cpq11_us(nacao).wrapping_add(contas)); // cqb11: o nível vira a conta da nação
                }
            }
            e += 7;
        }
    }
    liberar(contas);
    ordenar(vetor, n as i32, 7, 0x51ebc0);
    let mut segunda = false;
    let mut j: i16 = 0;
    while j < n {
        let e = cpq11_em(vetor, j as i32, 7);
        let mut regra = true; // volta a valer a cada entrada
        let clube = if cpq11_w(e) != -1 { cpq11_us(cpq11_time(copa, cpq11_w(e))) } else { 0 };
        if cqb11_b(e + 6) == 0xff {
            break;
        }
        let s = sorteio(cpq11_w(cpq11_rodada(copa, r) + 0x1a) as i32);
        let inicio = s as i16;
        let mut cur = s;
        let mandante = loop {
            let t = cpq11_confronto(copa, r, cur as i16 as i32);
            let mut lado: Option<bool> = None;
            if segunda {
                let mut livre = false;
                let mut m = false;
                let casa = cpq11_w(t);
                let c0 = if casa == -2 {
                    m = true;
                    livre = true;
                    0
                } else if casa == -1 {
                    0
                } else {
                    cpq11_us(cpq11_time(copa, casa))
                };
                let fora = cpq11_w(t + 2);
                let c1 = if fora == -2 {
                    if !(livre && sorteio(2) != 0) {
                        m = false;
                        livre = true;
                    }
                    0
                } else if fora == -1 {
                    0
                } else {
                    cpq11_us(cpq11_time(copa, fora))
                };
                let mesma = clube != 0 && regra
                    && ((c0 != 0 && cpq11_d(clube + 0x53) == cpq11_d(c0 + 0x53)) || (c1 != 0 && cpq11_d(clube + 0x53) == cpq11_d(c1 + 0x53)));
                if !mesma && livre {
                    lado = Some(m);
                }
            } else {
                let (casa, fora) = (cpq11_w(t), cpq11_w(t + 2));
                if casa < 0 && fora < 0 {
                    if fora == -2 {
                        lado = Some(casa == -2 && sorteio(2) != 0);
                    } else if casa == -2 {
                        lado = Some(true);
                    }
                }
            }
            if let Some(m) = lado {
                break m;
            }
            if !cqb11_passo(copa, r, &mut cur, inicio, &mut segunda, &mut regra) {
                aviso_em(0x9b55ac, 0xe5c);
                liberar(vetor);
                return;
            }
        };
        cqb11_colocar(copa, r, rodada, e, cur, mandante); // cqb11: a entrada vai ao confronto sorteado
        j = j.wrapping_add(1);
    }
    liberar(vetor);
}

/// Comparador: como o da 0x51d2d0 (sorteia; as contas e o vetor são temporários).
unsafe extern "thiscall" fn comparar_check_round_teams_for_nation(copa: usize, rodada: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32)>(%%I1%%);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend(cpq11_confrontos(copa));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(copa, rodada),
        || crate::aleatorio::sem_contar(|| check_round_teams_for_nation(copa, rodada)));
    PLACAR.registrar(igual, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("check_round_teams_for_nation(rodada {}) diferencas em {:x?}", rodada as i16, dif)
    });
}

/// { 0x518780 } (thiscall, ret 0, Cup.cpp): `Cup::start_new_season()` da classe base: só zera o byte +0xb1 da copa
/// (as copas derivadas liberam as tabelas delas).
unsafe extern "thiscall" fn cup_start_new_season(copa: usize) {
    *((copa + 0xb1) as *mut u8) = 0;
}

/// Comparador: lado a lado no byte +0xb1.
unsafe extern "thiscall" fn comparar_cup_start_new_season(copa: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    let reg = [(copa + 0xb1, 1)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(copa), || cup_start_new_season(copa));
    PLACAR.registrar(igual, || format!("cup_start_new_season({:#x}) {:x?} x {:x?}", copa, mo, mr));
}
