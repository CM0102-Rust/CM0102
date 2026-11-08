// ---------- %%LEVA%%a leva: a tabela da liga: as posições possíveis e a situação de cada clube (o título, o acesso, o mata-mata, o rebaixamento, a vaga europeia), e a fase seguinte da First e da Second (league.cpp, eng_first.cpp, eng_second.cpp) ----------

unsafe fn cpz3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpz3_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpz3_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpz3_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpz3_s(a: usize) -> i8 {
    *(a as *const i8)
}

/// No ensaio (o comparador da 0x684cd0) a cópia não chama as virtuais +0x40 (o anúncio da situação, que manda notícia
/// e grava os registros) e +0x68 (a situação nas estatísticas) nem as cláusulas de rebaixamento (0x4cd030).
static mut POSITIONS_STATUS_DRY_RUN: bool = false;

/// A linha `i` da tabela (+0xb1, 0x3b bytes).
unsafe fn league_table_row(liga: usize, i: i32) -> usize {
    cpz3_d(liga + 0xb1).wrapping_add((i as usize).wrapping_mul(0x3b))
}

/// A virtual +0x40 da liga (League::announce_team_status: thiscall(liga, clube, situação, fase, 0, 0), ret 0x14), com
/// o clube da linha `i`. `alto` = os bits de cima do registrador que a original empurra com a fase (+0x44) no byte de
/// baixo (o ponteiro da linha ou a devolução das cláusulas: reproduzidos).
unsafe fn league_announce_team_status(liga: usize, i: i32, situacao: u32, alto: u32) {
    if POSITIONS_STATUS_DRY_RUN {
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(cpz3_d(cpz3_d(liga) + 0x40));
    f(liga, cpz3_d(league_table_row(liga, i)), situacao, alto | cpz3_b(liga + 0x44) as u32, 0, 0);
}

/// Grava a situação `v` no +0x37 da linha `i` e a anuncia.
unsafe fn set_row_status(liga: usize, i: i32, v: u8, alto: u32) {
    *((league_table_row(liga, i) + 0x37) as *mut u8) = v;
    league_announce_team_status(liga, i, v as i8 as i32 as u32, alto);
}

/// O máximo de pontos da linha: (jogos da temporada +0xa7 − jogos feitos +0x11) × pontos da vitória (+0xc2) + pontos
/// (+0x19).
unsafe fn max_points(liga: usize, l: usize) -> i32 {
    ((cpz3_w(liga + 0xa7) as i32) - (cpz3_s(l + 0x11) as i32)).wrapping_mul(cpz3_s(liga + 0xc2) as i32).wrapping_add(cpz3_w(l + 0x19) as i32)
}

/// { 0x684cd0 } (thiscall, ret 0, league.cpp): `League::update_positions_and_status()`, a virtual +0x94 da liga (do
/// passe do dia 0x684946). +0xa7 (os jogos da temporada) = (+0x3e − 1) × +0x3c (16 bits) se ainda −1, com linhas e
/// +0x45. Com +0xdd nada mais. Senão ordena a tabela (League::sort_table 0x6827d0, thiscall(liga, 0), ret 4) e monta
/// o mapa das faixas (malloc 0x944e46 de +0x3e bytes em 0xff; sem memória: a caixa da linha 0x7c9): de cima, as
/// contas +0xbe (faixa 0, acesso) e +0xbf (faixa 1, mata-mata) nas linhas que não estão em 0xfe, 0xfc ou 3; de baixo,
/// +0xc1 (faixa 3, rebaixamento) e +0xc0 (faixa 2) nas que não estão em 0, 1 ou 5. Com +0xd9 bit 0, o primeiro da
/// tabela fora de 0xfe, 0xfc e 3. A melhor posição (+0x39) e a pior (+0x3a) de cada linha que ainda não estão
/// decididas, pelos máximos de pontos e pelos jogos feitos (com +0xe6 no empate dos que terminaram). Depois, linha a
/// linha: a pior = o primeiro + 1, o título (5); faixa 0 na pior, o acesso (0 com +0xd9 bit 1; 7 com bit 2; 6 com bit
/// 3) em quem está em 0xff ou 1; faixa 1, o mata-mata (1) em quem está em 0xff; faixa 3 na melhor, as cláusulas de
/// rebaixamento (CONTRACT_MANAGER::trigger_relegation_clauses 0x4cd030, thiscall([0xae1050], clube, +4), ret 8) e o
/// rebaixamento (3) em quem está em 0xff, 2 ou 0xfc; faixa 2 na melhor, 2 em quem está em 0xff ou 0xfe. Cada situação
/// nova é anunciada (+0x40). Com +0xd9 bit 0x20, quem fica entre as faixas sem faixa no meio e está em 0xff vira 0xfe
/// (anunciado com −2). Melhor = pior abaixo de 9, liga de cima (+0x44 = 0xff) que conta (0x4b0700, cdecl(id)), sem
/// +0x1c e clube sem vaga europeia ([0xae2d14] + 6 × id = −1) de nação europeia ([0x9cfa10]) fora de [0x9cf45c]: as
/// vagas da nação (UefaSeeding::get_nation_places 0x905c60, thiscall([0xde1f58], nação, &cl, &uefa), ret 0xc) dão a
/// Liga dos Campeões ([0x9cf6e8]) ou a Copa da UEFA ([0x9cf6f0]). Por fim a virtual +0x68 (a situação nas
/// estatísticas: clube, +0x44, posição, melhor, pior) e o free (0x9452ca) do mapa.
unsafe extern "thiscall" fn update_positions_and_status(this: usize) {
    let n0 = cpz3_w(this + 0x3e);
    if cpz3_w(this + 0xa7) == -1 && n0 > 0 && cpz3_d(this + 0x45) != 0 {
        *((this + 0xa7) as *mut i16) = (n0 - 1).wrapping_mul(cpz3_w(this + 0x3c));
    }
    if cpz3_d(this + 0xdd) != 0 {
        return;
    }
    let ordenar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x6827d0usize);
    ordenar(this, 0);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let mapa = alocar(cpz3_w(this + 0x3e) as i32 as u32);
    if mapa == 0 {
        return show_error_at(0x9f15f4, 0x7c9);
    }
    let n = cpz3_w(this + 0x3e);
    core::ptr::write_bytes(mapa as *mut u8, 0xff, n as i32 as usize);
    let m = |i: i32| (mapa as isize + i as isize) as *mut u8;
    let mut contas = [cpz3_b(this + 0xbe), cpz3_b(this + 0xbf), cpz3_b(this + 0xc0), cpz3_b(this + 0xc1)];
    let mut primeiro: i8 = -1;
    if cpz3_b(this + 0xd9) & 1 != 0 {
        let mut i: i16 = 0;
        while i < n {
            let s = cpz3_b(league_table_row(this, i as i32) + 0x37);
            if s != 0xfe && s != 0xfc && s != 3 {
                primeiro = i as i8;
                break;
            }
            i += 1;
        }
    }
    // de cima: as faixas 0 e 1
    let mut i: i32 = 0;
    if n > 0 {
        let mut k: i8 = 0;
        while k <= 1 {
            if contas[k as usize] != 0 {
                while (i as i16) < n {
                    let s = cpz3_b(league_table_row(this, i as i16 as i32) + 0x37);
                    if s != 0xfe && s != 0xfc && s != 3 {
                        *m(i as i16 as i32) = k as u8;
                        contas[k as usize] = contas[k as usize].wrapping_sub(1);
                    }
                    i += 1;
                    if contas[k as usize] == 0 {
                        break;
                    }
                }
            }
            k += 1;
            if (i as i16) >= n {
                break;
            }
        }
    }
    // de baixo: as faixas 3 e 2
    let mut i: i32 = (n as i32) - 1;
    let mut k: i8 = 3;
    while (i as i16) >= 0 && k >= 2 {
        if contas[k as usize] != 0 {
            while (i as i16) >= 0 {
                let s = cpz3_b(league_table_row(this, i as i16 as i32) + 0x37);
                if s != 0 && s != 1 && s != 5 {
                    *m(i as i16 as i32) = k as u8;
                    contas[k as usize] = contas[k as usize].wrapping_sub(1);
                }
                i -= 1;
                if contas[k as usize] == 0 {
                    break;
                }
            }
        }
        k -= 1;
    }
    // a melhor (+0x39) e a pior (+0x3a) posição possível
    let mut i: i32 = 0;
    while (i as i16) < cpz3_w(this + 0x3e) {
        let li = league_table_row(this, i as i16 as i32);
        if cpz3_b(li + 0x39) != cpz3_b(li + 0x3a) {
            let pos = (i as u8).wrapping_add(1);
            *((li + 0x39) as *mut u8) = pos;
            // os de cima (j < i): a melhor cai um para cada um que o máximo de i alcança
            let max_i = max_points(this, li);
            let mut j: i32 = 0;
            while (j as i16) < (i as i16) {
                let lj = league_table_row(this, j);
                let jogos = cpz3_w(this + 0xa7);
                let fim_i = (cpz3_s(li + 0x11) as i16) >= jogos;
                let fim_j = (cpz3_s(lj + 0x11) as i16) >= jogos;
                let pj = cpz3_w(lj + 0x19) as i32;
                let cai = if !fim_i && !fim_j {
                    max_i >= pj
                } else if !fim_j {
                    max_i == pj
                } else if !fim_i {
                    max_i >= pj
                } else {
                    max_i == pj && cpz3_d(this + 0xe6) != 0
                };
                if cai {
                    *((li + 0x39) as *mut u8) = cpz3_b(li + 0x39).wrapping_sub(1);
                }
                j += 1;
            }
            // os de baixo (j > i): a pior sobe um para cada um cujo máximo alcança i
            *((li + 0x3a) as *mut u8) = pos;
            let mut j: i32 = pos as i8 as i32;
            while (j as i16) < cpz3_w(this + 0x3e) {
                let lj = league_table_row(this, j as i16 as i32);
                let jogos = cpz3_w(this + 0xa7);
                let pj = cpz3_w(lj + 0x19) as i32;
                let max_j = max_points(this, lj);
                let fim_i = (cpz3_s(li + 0x11) as i16) >= jogos;
                let fim_j = (cpz3_s(lj + 0x11) as i16) >= jogos;
                let pi = cpz3_w(li + 0x19) as i32;
                let sobe = if !fim_i && !fim_j {
                    max_j >= pi
                } else if !fim_j {
                    max_j >= pi
                } else if !fim_i {
                    max_j == pi
                } else {
                    max_j == pj && cpz3_d(this + 0xe6) != 0
                };
                if sobe {
                    *((li + 0x3a) as *mut u8) = cpz3_b(li + 0x3a).wrapping_add(1);
                }
                j += 1;
            }
        }
        i += 1;
    }
    // a situação de cada linha
    let um_depois = (primeiro as i32) + 1;
    let mut i: i32 = 0;
    while (i as i16) < cpz3_w(this + 0x3e) {
        let ii = i as i16 as i32;
        let li = league_table_row(this, ii);
        let alto_linha = (li as u32) & !0xff;
        let pior = cpz3_s(li + 0x3a) as i32;
        let s = cpz3_b(li + 0x37);
        if pior == um_depois {
            if s != 3 && s != 0xfe && s != 0xfc && s != 5 {
                crate::comparacao::ramo("cpz_3 titulo");
                set_row_status(this, ii, 5, alto_linha); // cpz3: o título na pior posição = o primeiro + 1
            }
        } else {
            let f = *m(pior - 1);
            if f == 0 {
                if s == 0xff || s == 1 {
                    let d9 = cpz3_b(this + 0xd9);
                    if d9 & 2 != 0 {
                        if s != 0 {
                            set_row_status(this, ii, 0, alto_linha);
                        }
                    } else if d9 & 4 != 0 {
                        if s != 7 {
                            set_row_status(this, ii, 7, alto_linha);
                        }
                    } else if d9 & 8 != 0 {
                        if s != 6 {
                            set_row_status(this, ii, 6, alto_linha);
                        }
                    }
                }
            } else if f == 1 {
                if s == 0xff {
                    crate::comparacao::ramo("cpz_3 vaga no mata-mata");
                    set_row_status(this, ii, 1, alto_linha); // cpz3: a vaga no mata-mata (situação 1)
                }
            } else {
                let g = *m(cpz3_s(li + 0x39) as i32 - 1);
                if g == 3 {
                    let mut alto = alto_linha;
                    if !POSITIONS_STATUS_DRY_RUN {
                        crate::comparacao::ramo("cpz_3 clausulas de rebaixamento");
                        let clausulas: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x4cd030usize);
                        alto = clausulas(cpz3_d(0xae1050), cpz3_d(li), cpz3_d(this + 4)) & !0xff; // cpz3: as cláusulas de rebaixamento
                    }
                    let li = league_table_row(this, ii);
                    let s = cpz3_b(li + 0x37);
                    if s == 0xff || s == 2 || s == 0xfc {
                        set_row_status(this, ii, 3, alto);
                    }
                } else if g == 2 {
                    if s == 0xff || s == 0xfe {
                        set_row_status(this, ii, 2, alto_linha);
                    }
                }
            }
        }
        // entre as faixas, sem faixa no meio: 0xfe
        if cpz3_b(this + 0xd9) & 0x20 != 0 {
            let li = league_table_row(this, ii);
            let c0 = cpz3_s(this + 0xbe) as i32;
            let c2 = cpz3_s(this + 0xc0) as i32;
            let melhor = cpz3_s(li + 0x39);
            if (melhor as i32) > c0 + c2 {
                let c3 = cpz3_s(this + 0xc1) as i32;
                let pior = cpz3_s(li + 0x3a);
                if (pior as i32) <= (cpz3_w(this + 0x3e) as i32) - c3 - c2 {
                    let mut livre = true;
                    let mut a: i16 = (melhor as i16) - 1;
                    while a < pior as i16 {
                        if !livre {
                            break;
                        }
                        if *m(a as i32) != 0xff {
                            livre = false;
                        }
                        a += 1;
                    }
                    if livre && cpz3_b(li + 0x37) == 0xff {
                        *((li + 0x37) as *mut u8) = 0xfe;
                        league_announce_team_status(this, ii, 0xffff_fffe, (li as u32) & 0xffff_0000);
                    }
                }
            }
        }
        // a vaga europeia de quem já tem a posição decidida
        let li = league_table_row(this, ii);
        let melhor = cpz3_s(li + 0x39);
        if melhor == cpz3_s(li + 0x3a) && melhor < 9 && cpz3_b(this + 0x44) == 0xff {
            let conta: unsafe extern "C" fn(i32) -> u32 = core::mem::transmute(0x4b0700usize);
            if conta(cpz3_i(cpz3_d(this + 4))) != 0 && cpz3_i(this + 0x1c) == -1 {
                let vaga = cpz3_d(0xae2d14) + (cpz3_i(cpz3_d(li)) as usize).wrapping_mul(6);
                let reg = cpz3_d(this + 4);
                let nacao = cpz3_d(reg + 0x5d);
                if cpz3_i(vaga) == -1 && nacao != 0 && cpz3_i(nacao) != cpz3_i(0x9cf45c) {
                    let cont = cpz3_d(nacao + 0x71);
                    if cont != 0 && cpz3_i(cont) == cpz3_i(0x9cfa10) {
                        let (mut cl, mut uefa) = (0i8, 0i8);
                        let vagas: unsafe extern "thiscall" fn(usize, usize, *mut i8, *mut i8) = core::mem::transmute(0x905c60usize);
                        vagas(cpz3_d(0xde1f58), nacao, &mut cl, &mut uefa);
                        let li = league_table_row(this, ii);
                        let melhor = cpz3_s(li + 0x39) as i32;
                        let vaga = cpz3_d(0xae2d14) + (cpz3_i(cpz3_d(li)) as usize).wrapping_mul(6);
                        if melhor <= cl as i32 {
                            core::ptr::write_unaligned(vaga as *mut i32, cpz3_i(0x9cf6e8));
                        } else if melhor <= (cl as i32) + (uefa as i32) - 1 {
                            core::ptr::write_unaligned(vaga as *mut i32, cpz3_i(0x9cf6f0));
                        }
                    }
                }
            }
        }
        // a situação nas estatísticas (virtual +0x68)
        if !POSITIONS_STATUS_DRY_RUN {
            let li = league_table_row(this, ii);
            // ponytail: a original empurra a pior, a melhor, a posição e a fase em registradores com lixo nos bits de
            // cima; as +0x68 (0x573260, 0x578660, Comp::set_club_progress 0x48e380) só leem o byte.
            let f: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) = core::mem::transmute(cpz3_d(cpz3_d(this) + 0x68));
            f(this, cpz3_d(li), cpz3_b(this + 0x44) as u32, (i as u8).wrapping_add(1) as u32, cpz3_b(li + 0x39) as u32, cpz3_b(li + 0x3a) as u32);
        }
        i += 1;
    }
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    liberar(mapa);
}

/// As regiões que a 0x684cd0 grava: a tabela inteira (+0xb1, +0x3e linhas de 0x3b bytes), +0xa7 e a vaga europeia
/// ([0xae2d14] + 6 × id, 4 bytes) de cada clube da tabela.
unsafe fn table_status_regions(this: usize) -> Vec<(usize, usize)> {
    let n = cpz3_w(this + 0x3e).max(0) as usize;
    let mut v = vec![(this + 0xa7, 2)];
    let tab = cpz3_d(this + 0xb1);
    if n > 0 && tab != 0 {
        v.push((tab, n * 0x3b));
        let vagas = cpz3_d(0xae2d14);
        if vagas != 0 {
            for i in 0..n {
                let c = cpz3_d(tab + i * 0x3b);
                if c != 0 {
                    let a = vagas + (cpz3_i(c) as usize).wrapping_mul(6);
                    if !v.iter().any(|r| r.0 == a) {
                        v.push((a, 4));
                    }
                }
            }
        }
    }
    v
}

/// Comparador da 0x684cd0, em ensaio (modelo RARE_FINANCE_EVENT_DRY_RUN): a cópia roda primeiro sem as virtuais +0x40 e +0x68 nem
/// as cláusulas (que mandam notícia, gravam registros e sorteiam o moral); a memória volta e a original roda inteira.
/// As duas a partir do mesmo estado da tabela, de +0xa7 e das vagas europeias dos clubes da tabela, que têm de sair
/// iguais (as virtuais e as cláusulas não gravam nelas). Sem linhas ou com +0xdd: só a original.
unsafe extern "thiscall" fn comparar_update_positions_and_status(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    if cpz3_d(this + 0xdd) != 0 || cpz3_w(this + 0x3e) <= 0 || cpz3_d(this + 0xb1) == 0 {
        return o(this);
    }
    let regioes = table_status_regions(this);
    let antes = crate::comparacao::Memoria::ler(&regioes);
    POSITIONS_STATUS_DRY_RUN = true;
    crate::aleatorio::pausar_rastro(|| update_positions_and_status(this));
    POSITIONS_STATUS_DRY_RUN = false;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    o(this);
    let mo = crate::comparacao::Memoria::ler(&regioes);
    PLACAR.registrar(mo == mr, || format!("update_positions_and_status({:#x}) original={:x?} rust={:x?}", this, mo, mr));
}

/// A fase seguinte das ligas com mata-mata (o corpo de 0x572ff0 e 0x5769e0): na última fase (+0x30 ≥ +0x2c − 1, com
/// sinal) a caixa da linha `linha`; senão +0x30 + 1 e, se chegou a 0, o mata-mata (`mata_mata`, thiscall(this)).
unsafe fn advance_league_stage(this: usize, arquivo: usize, linha: u32, mata_mata: usize, nome: &'static str) {
    let f = cpz3_i(this + 0x30);
    if f >= cpz3_i(this + 0x2c).wrapping_sub(1) {
        return show_error_at(arquivo, linha);
    }
    let f = f.wrapping_add(1);
    core::ptr::write_unaligned((this + 0x30) as *mut i32, f);
    if f == 0 {
        crate::comparacao::ramo(nome);
        let criar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(mata_mata);
        criar(this);
    }
}

/// { 0x572ff0 } (thiscall, ret 0, eng_first.cpp): `EngFirst::advance_stage()` (a virtual +0x28): fonte 0x9c4374,
/// linha 0x2ce; o mata-mata 0x573080.
unsafe extern "thiscall" fn eng_first_advance_stage(this: usize) {
    advance_league_stage(this, 0x9c4374, 0x2ce, 0x573080, "cpz_3 mata-mata da first")
}

/// { 0x5769e0 } (thiscall, ret 0, eng_second.cpp): `EngSecond::advance_stage()` (a virtual +0x28): fonte 0x9c43e8,
/// linha 0x294; o mata-mata 0x576a70.
unsafe extern "thiscall" fn eng_second_advance_stage(this: usize) {
    advance_league_stage(this, 0x9c43e8, 0x294, 0x576a70, "cpz_3 mata-mata da second")
}

/// O comparador das duas: na fase que não chega a 0 (só +0x30 muda) a original e a cópia lado a lado em +0x30; na
/// última fase (a caixa) e na que chega a 0 (o mata-mata aloca a fase): só a original.
unsafe fn compare_stage_advance(nome: &str, this: usize, original: impl FnOnce(), copia: impl FnOnce()) {
    let f = cpz3_i(this + 0x30);
    if f >= cpz3_i(this + 0x2c).wrapping_sub(1) || f.wrapping_add(1) == 0 {
        return original();
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this + 0x30, 4)], original, copia);
    PLACAR.registrar(igual, || format!("{}({:#x}) original={:x?} rust={:x?}", nome, this, mo, mr));
}

unsafe extern "thiscall" fn comparar_eng_first_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    compare_stage_advance("eng_first_advance_stage", this, || o(this), || eng_first_advance_stage(this))
}

unsafe extern "thiscall" fn comparar_eng_second_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    compare_stage_advance("eng_second_advance_stage", this, || o(this), || eng_second_advance_stage(this))
}
