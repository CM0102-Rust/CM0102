// ---------- 283a leva: a Série C: a final, a virada da temporada e as posições previstas (bra_nat_third.cpp) ----------

unsafe fn cqs4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqs4_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O clube da primeira linha da tabela (+0xb1) da fase `k` da liga (k = −1: a própria liga).
unsafe fn cqs4_primeiro(this: usize, k: i32) -> usize {
    let t = if k == -1 { this } else { cqs4_d(cqs4_d(this + 0xc) + 4 * k as usize) };
    cqs4_d(cqs4_d(t + 0xb1))
}

/// { 0x42e980 } (thiscall, ret 0, bra_nat_third.cpp; quadro SEH só para o new): `BraNatThird::create_final_stage()`, a
/// final da Série C, do fim da terceira fase: os campeões dos dois grupos da terceira fase (o primeiro da tabela da
/// fase 8 e o da fase 9, nessa ordem) numa lista local de 2; a tabela das rodadas pela virtual +0x3c
/// (create_round_table: thiscall(this, 10, &rodadas, &extra, 0), ret 0x10); a fase (operator new 0x944cf1(0xb2) e o
/// construtor Cup_Stage 0x522e00, thiscall, ret 0x3c: a liga, 2, a lista, as rodadas, +4, a tabela, o ano, 10, 1, o
/// extra, 0x14, 0, 0, 0, sementes nulas; words limpas) em [+0xc][10]. Sem fase: a caixa da linha 0x392 e volta (sem
/// liberar a tabela). Com ela: a tabela liberada (free 0x9452ca) e +0x30 = 10. [inferência] A final decide o campeão
/// da Série C, em ida e volta.
unsafe extern "thiscall" fn cqs4_final_da_serie_c(this: usize) {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    #[allow(clippy::type_complexity)]
    let construir: unsafe extern "thiscall" fn(usize, usize, u32, *const usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, usize) -> usize =
        core::mem::transmute(0x522e00usize);
    let mut lista = [0usize; 2];
    lista[0] = cqs4_primeiro(this, 8); // cqs4: o campeão do grupo A
    lista[1] = cqs4_primeiro(this, 9);
    let tabela: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(cqs4_d(cqs4_d(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = tabela(this, 10, &mut rodadas, &mut extra, 0);
    let p = novo(0xb2);
    let fase = if p != 0 {
        construir(p, this, 2, lista.as_ptr(), rodadas as u16 as u32, cqs4_d(this + 4), t, cqs4_w(this + 0x40) as u16 as u32, 10, 1,
                  extra as u16 as u32, 0x14, 0, 0, 0, 0)
    } else {
        0
    };
    core::ptr::write_unaligned((cqs4_d(this + 0xc) + 0x28) as *mut usize, fase);
    if cqs4_d(cqs4_d(this + 0xc) + 0x28) == 0 {
        return aviso_em(0x98aba4, 0x392);
    }
    crate::comparacao::ramo("cqs_4 final");
    soltar(t);
    core::ptr::write_unaligned((this + 0x30) as *mut u32, 10);
}

/// { 0x42def0 } (thiscall, ret 0, bra_nat_third.cpp): `BraNatThird::start_new_season()`, a virada da Série C (chamada
/// pela virada da Série A): +0x4c = 0 e a tabela liberada (League::free_table 0x687970, thiscall(this, 0), ret 4); o
/// calendário (+0xba) liberado (free 0x9452ca) e zerado; com estatísticas (+8), Comp_Stats::archive_season_and_reset
/// (0x4a1c50, thiscall(+8, 1), ret 4); com a fase atual ≥ 0, as fases 0..=+0x30 que existem destruídas (a virtual 0,
/// o destrutor com 1); +0x30 = −1; o vetor das fases liberado; o ano (+0x40, word) sobe um; um vetor novo de +0x2c
/// fases (malloc de 4 × +0x2c, sem zerar; com +0x2c ≤ 0, nulo); então, cada um com a sua caixa (linhas 0x202, 0x209,
/// 0x20f e 0x216) se devolver 0: as regras (virtual +0x8c), a tabela e a lista dos 56 (0x42dc90, thiscall, ret 0), os
/// jogos (League::create_fixtures 0x6835c0) e a ordem da tabela (League::sort_table 0x6827d0, thiscall(this, 0), ret
/// 4); por fim as expectativas das diretorias (League::set_club_season_expectations 0x68aa80, thiscall, ret 0), a
/// renovação dos árbitros (OfficialsManager::renew_officials 0x79cee0, thiscall([0xb74340], +4), ret 4), os sete
/// grupos da primeira fase (0x42ee80(g), thiscall, ret 4, g de 0 a 6), as posições previstas (virtual +0x5c) e a lista
/// dos 56 (+0xee) liberada (se houver) e zerada. Com caixa, a cópia devolve 0 (a original devolve o eax da caixa, que
/// a virada do Brasil não lê); no fim, o eax do free ou o +0xee (a cópia devolve 0: ninguém lê).
unsafe extern "thiscall" fn cqs4_virada_da_serie_c(this: usize) -> u32 {
    let liberar_tabela: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x687970usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    core::ptr::write_unaligned((this + 0x4c) as *mut u32, 0);
    liberar_tabela(this, 0);
    let l = cqs4_d(this + 0xba);
    if l != 0 {
        liberar(l);
        core::ptr::write_unaligned((this + 0xba) as *mut usize, 0);
    }
    let estatisticas = cqs4_d(this + 8);
    if estatisticas != 0 {
        let arquivar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x4a1c50usize);
        arquivar(estatisticas, 1);
    }
    if (cqs4_d(this + 0x30) as i32) >= 0 {
        let mut i = 0i32;
        loop {
            let f = cqs4_d(cqs4_d(this + 0xc) + 4 * i as usize);
            if f != 0 {
                let destruir: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(cqs4_d(cqs4_d(f)));
                destruir(f, 1);
            }
            i += 1;
            if i > cqs4_d(this + 0x30) as i32 {
                break;
            }
        }
    }
    core::ptr::write_unaligned((this + 0x30) as *mut i32, -1);
    liberar(cqs4_d(this + 0xc));
    let n = cqs4_d(this + 0x2c) as i32;
    let a = cqs4_w(this + 0x40);
    core::ptr::write_unaligned((this + 0x40) as *mut i16, a.wrapping_add(1));
    crate::comparacao::ramo("cqs_4 virada da serie c");
    let v = if n > 0 { alocar((n * 4) as u32) } else { 0 };
    core::ptr::write_unaligned((this + 0xc) as *mut usize, v);
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cqs4_d(cqs4_d(this) + 0x8c));
    if regras(this) == 0 {
        aviso_em(0x98aba4, 0x202);
        return 0;
    }
    let tabela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x42dc90usize);
    if tabela(this) == 0 {
        aviso_em(0x98aba4, 0x209);
        return 0;
    }
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x6835c0usize);
    if jogos(this) == 0 {
        aviso_em(0x98aba4, 0x20f);
        return 0;
    }
    let ordenar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x6827d0usize);
    if ordenar(this, 0) == 0 {
        aviso_em(0x98aba4, 0x216);
        return 0;
    }
    let expectativas: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x68aa80usize);
    expectativas(this);
    let renovar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x79cee0usize);
    renovar(cqs4_d(0xb74340), cqs4_d(this + 4)); // cqs4: a renovação dos árbitros da Série C
    let grupo: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x42ee80usize);
    for g in 0..7 {
        grupo(this, g);
    }
    let previstas: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cqs4_d(cqs4_d(this) + 0x5c));
    previstas(this);
    let l = cqs4_d(this + 0xee);
    if l != 0 {
        liberar(l);
    }
    core::ptr::write_unaligned((this + 0xee) as *mut usize, 0);
    0
}

/// As posições previstas da Série C, na ordem da lista dos 56 depois de ordenada: 1, 2, 3, 3, 5, 5, 7, 7, oito vezes 9
/// (as faixas do mata-mata) e, para os outros quarenta, a faixa de oito em oito: 16, 24, 32, 40 e 48.
const CQS4_LUGARES: [u8; 16] = [1, 2, 3, 3, 5, 5, 7, 7, 9, 9, 9, 9, 9, 9, 9, 9]; // cqs4: as posições previstas do mata-mata

fn cqs4_lugar(k: usize) -> u8 {
    if k < 16 { CQS4_LUGARES[k] } else { (16 + 8 * ((k - 16) / 8)) as u8 }
}

/// { 0x42f0a0 } (thiscall, ret 0, bra_nat_third.cpp): `BraNatThird::set_predicted_positions()`, as posições previstas da
/// Série C (do construtor e da virada, depois dos grupos). Sem estatísticas (+8): a caixa da linha 0x4a2 e volta. A
/// lista dos 56 (+0xee) é ordenada no lugar pela reputação (qsort 0x945333 com 0x4b07d0, a maior primeiro, nulos no
/// fim) e cada clube, na ordem, ganha a posição prevista de `cqs4_lugar` (Comp_Stats::set_club_final_position 0x4a2540,
/// thiscall(+8, clube, posição), ret 8; a original empurra a faixa dos quarenta num dword com o byte da faixa e lixo
/// da pilha em cima; a 0x4a2540 só lê o byte: a cópia passa o byte limpo).
unsafe extern "thiscall" fn cqs4_posicoes_previstas_da_serie_c(this: usize) {
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let posicao: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x4a2540usize);
    if cqs4_d(this + 8) == 0 {
        return aviso_em(0x98aba4, 0x4a2);
    }
    ordenar(cqs4_d(this + 0xee), 0x38, 4, 0x4b07d0); // cqs4: a lista dos 56 pela reputação
    for k in 0..0x38usize {
        posicao(cqs4_d(this + 8), cqs4_d(cqs4_d(this + 0xee) + 4 * k), cqs4_lugar(k) as u32);
    }
    crate::comparacao::ramo("cqs_4 posicoes previstas");
}

/// O comparador da final: só a original (aloca a tabela e a fase e sorteia a rodada), com o plano feito antes: os
/// campeões dos grupos da terceira fase. Depois, a fase da vaga 10 existe, com +0x3e = 2, os dois na lista dela (como
/// conjunto) e +0x30 = 10.
unsafe extern "thiscall" fn comparar_cqs4_final_da_serie_c(this: usize) {
    let (a, b) = (cqs4_primeiro(this, 8), cqs4_primeiro(this, 9));
    orig::<unsafe extern "thiscall" fn(usize)>(2027)(this);
    let fase = cqs4_d(cqs4_d(this + 0xc) + 0x28);
    let igual = fase != 0 && {
        let l = cqs4_d(fase + 0xa7);
        let (x, y) = (cqs4_d(l), cqs4_d(l + 6));
        cqs4_w(fase + 0x3e) == 2 && cqs4_d(this + 0x30) == 10 && ((x, y) == (a, b) || (x, y) == (b, a))
    };
    PLACAR.registrar(igual, || format!("cqs4_final_da_serie_c({:#x}) campeoes={:#x},{:#x} fase={:#x}", this, a, b, fase));
}

/// O comparador da virada: só a original (ponytail: libera tabelas e fases, aloca as novas, sorteia jogos, árbitros e
/// grupos; não pode rodar duas vezes), conferindo o que só ela garante: o ano + 1, os sete grupos nas vagas 0 a 6 e a
/// lista dos 56 zerada.
unsafe extern "thiscall" fn comparar_cqs4_virada_da_serie_c(this: usize) -> u32 {
    let antes = cqs4_w(this + 0x40);
    let r = orig::<unsafe extern "thiscall" fn(usize) -> u32>(2028)(this);
    let depois = cqs4_w(this + 0x40);
    let igual = depois == antes.wrapping_add(1) && cqs4_d(this + 0xee) == 0 && (0..7).all(|g| cqs4_d(cqs4_d(this + 0xc) + 4 * g) != 0);
    PLACAR.registrar(igual, || format!("cqs4_virada_da_serie_c({:#x}) ano antes={} depois={} +0xee={:#x}", this, antes, depois, cqs4_d(this + 0xee)));
    r
}

/// O comparador das posições previstas: só a original (a 0x4a2540 acrescenta o registro de um clube que não tem; não
/// pode rodar duas vezes), com o plano feito antes: a lista dos 56 ordenada pela mesma qsort numa cópia; depois, a
/// lista da liga na mesma ordem e a posição (+0x72 do registro achado por 0x4a4850) de cada clube.
unsafe extern "thiscall" fn comparar_cqs4_posicoes_previstas_da_serie_c(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(2029);
    if cqs4_d(this + 8) == 0 || cqs4_d(this + 0xee) == 0 {
        return o(this);
    }
    let ordenar: unsafe extern "C" fn(*mut usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let achar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
    let mut v = [0usize; 0x38];
    for (k, c) in v.iter_mut().enumerate() {
        *c = cqs4_d(cqs4_d(this + 0xee) + 4 * k);
    }
    ordenar(v.as_mut_ptr(), 0x38, 4, 0x4b07d0);
    o(this);
    let errado = (0..0x38).find(|&k| {
        let e = if v[k] != 0 { achar(cqs4_d(this + 8), v[k]) } else { 0 };
        cqs4_d(cqs4_d(this + 0xee) + 4 * k) != v[k] || e == 0 || *((e + 0x72) as *const u8) != cqs4_lugar(k)
    });
    PLACAR.registrar(errado.is_none(), || format!("cqs4_posicoes_previstas_da_serie_c({:#x}): a vaga {:?} não bate", this, errado));
}

// ---------- 283a leva: o Troféu da Liga inglesa (LDV Vans Trophy): os 56 clubes das duas chaves (Norte e Sul), a fase das chaves e a fase seguinte (eng_auto_cup.cpp) ----------

unsafe fn cqs5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqs5_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqs5_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// O clube `c` está na competição de id [`g`] (+0x57 a divisão, ponteiro para o registro; +0 o id)?
unsafe fn cqs5_na(c: usize, g: usize) -> bool {
    let l = cqs5_d(c + 0x57);
    l != 0 && cqs5_d(l) == cqs5_d(g)
}

/// Põe o clube `c` na vaga `k` da lista (+0xb2, relida a cada vez) e conta.
unsafe fn cqs5_por(this: usize, k: &mut i32, c: usize) {
    *k += 1;
    cqs5_pd(cqs5_d(this + 0xb2) + (*k as usize - 1) * 4, c);
}

/// { 0x56b730 } (thiscall, ret 0, eng_auto_cup.cpp; da virada, n = 1 até 2002-06-01): `EngAutoCup::select_section_clubs()`:
/// os 56 clubes do Troféu da Liga (+0xb6 = 0x38) numa lista (0x944e46 de 0xe0 em +0xb2; sem memória, a caixa da linha
/// 0x24c e 0), em quatro passos, cada um até a lista encher (conta == +0xb6):
/// 1) pela ordem da tabela dos clubes ([0xae23b4], [0xae235c] clubes), os das divisões [0x9cf5c4] e [0x9cf5c8] (a segunda
///    e a terceira divisões inglesas [inferência]);
/// 2) o primeiro clube da divisão [0x9cf69c] (a Conference [inferência]) que veio da [0x9cf5c8] (+0x5b, a divisão da
///    temporada anterior [inferência]): o rebaixado; ele é lembrado (`achado`) para não entrar de novo;
/// 3) se a nação da copa ([+4] +0x5d) tem o bit 4 em +0x11c, os clubes da [0x9cf69c] que já estavam nela (+0x5b), pela
///    categoria +0x5f (com sinal): primeiro os de 6 ou menos, depois 7, 8... até encher; senão, até 30 clubes da
///    [0x9cf69c] (fora o achado) e, faltando, os clubes ingleses (+0x53 = [0x9cf2e4]) fora da divisão genérica [0x9cf764]
///    (entre os [0xae235c] − 2 × [0xae2350] primeiros), até 30, ordenados pela reputação com os 3 × faltam primeiros
///    embaralhados (comp_util_sort_and_shuffle_top_clubs 0x4afbd0: push faltam; push quantos; push lista; add esp, 0xc
///    -> cdecl; sorteia), e deles os primeiros que faltam.
/// Cheia, a lista é ordenada pela latitude da cidade do estádio (qsort 0x945333 com 0x53fd40) e cada metade (a do Norte e
/// a do Sul [inferência]) pela reputação, da menor para a maior (qsort com 0x4b0810); devolve 1. Sem encher, 0.
unsafe fn cqs5_selecao(this: usize) -> u32 {
    cqs5_pd(this + 0xb6, 0x38);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let l = alocar(0xe0);
    cqs5_pd(this + 0xb2, l);
    if l == 0 {
        aviso_em(0x9c4180, 0x24c);
        return 0;
    }
    let base = cqs5_d(0xae23b4);
    let n = cqs5_d(0xae235c) as i32;
    let cheia = |k: i32| k == cqs5_d(this + 0xb6) as i32;
    let mut k = 0i32;
    // 1) a segunda e a terceira divisões
    let mut i: i16 = 0;
    if n > 0 {
        loop {
            let c = base + i as u16 as usize * 0x245;
            if cqs5_na(c, 0x9cf5c4) || cqs5_na(c, 0x9cf5c8) {
                cqs5_por(this, &mut k, c);
            }
            if cheia(k) {
                break;
            }
            i = i.wrapping_add(1);
            if i as i32 >= n {
                break;
            }
        }
    }
    // 2) o rebaixado da terceira divisão para a Conference
    // ponytail: sem rebaixado a original compara depois com uma dword da pilha sem valor; a cópia usa 0 (nenhum clube)
    let mut achado = 0usize;
    let mut i: i16 = 0;
    if n > 0 {
        loop {
            let c = base + i as u16 as usize * 0x245;
            let antes = cqs5_d(c + 0x5b);
            if cqs5_na(c, 0x9cf69c) && antes != 0 && cqs5_d(antes) == cqs5_d(0x9cf5c8) {
                crate::comparacao::ramo("cqs_5 rebaixado para a conference");
                cqs5_por(this, &mut k, c); // cqs5: o rebaixado entra
                achado = c;
                break;
            }
            if cheia(k) {
                break;
            }
            i = i.wrapping_add(1);
            if i as i32 >= n {
                break;
            }
        }
    }
    let nacao = cqs5_d(cqs5_d(this + 4) + 0x5d);
    if *((nacao + 0x11c) as *const u8) & 4 != 0 {
        // 3a) a Conference pela categoria
        let mut cat: i8 = 6;
        if k < cqs5_d(this + 0xb6) as i32 {
            loop {
                let mut i: i16 = 0;
                if n > 0 {
                    loop {
                        if k >= cqs5_d(this + 0xb6) as i32 {
                            break;
                        }
                        let c = base + i as u16 as usize * 0x245;
                        let antes = cqs5_d(c + 0x5b);
                        if antes != 0 && cqs5_d(antes) == cqs5_d(0x9cf69c) && cqs5_na(c, 0x9cf69c) {
                            let v = *((c + 0x5f) as *const i8);
                            if ((cat == 6 && v <= cat) || v == cat) && c != 0 && c != achado {
                                cqs5_por(this, &mut k, c);
                            }
                        }
                        i = i.wrapping_add(1);
                        if i as i32 >= n {
                            break;
                        }
                    }
                }
                cat = cat.wrapping_add(1);
                if k >= cqs5_d(this + 0xb6) as i32 {
                    break;
                }
            }
        }
    } else {
        // 3b) até 30 da Conference e, faltando, ingleses fora da divisão genérica; embaralhados
        let mut local = [0usize; 0x1e];
        let mut m: i8 = 0;
        let mut i: i16 = 0;
        if n > 0 {
            loop {
                let c = base + i as u16 as usize * 0x245;
                if cqs5_na(c, 0x9cf69c) && achado != c {
                    local[m as u8 as usize] = c;
                    m = m.wrapping_add(1);
                    if m == 0x1e {
                        break;
                    }
                }
                i = i.wrapping_add(1);
                if i as i32 >= n {
                    break;
                }
            }
        }
        if (m as i32) < cqs5_d(this + 0xb6) as i32 - k {
            let lim = n.wrapping_sub((cqs5_d(0xae2350) as i32).wrapping_mul(2));
            let mut j: i16 = 0;
            if lim > 0 {
                loop {
                    let c = base + j as u16 as usize * 0x245;
                    let nac = cqs5_d(c + 0x53);
                    if nac != 0 && cqs5_d(nac) == cqs5_d(0x9cf2e4) && !cqs5_na(c, 0x9cf764) {
                        local[m as u8 as usize] = c;
                        m = m.wrapping_add(1);
                        if m == 0x1e {
                            break;
                        }
                    }
                    j = j.wrapping_add(1);
                    if j as i32 >= lim {
                        break;
                    }
                }
            }
        }
        let embaralhar: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(0x4afbd0usize);
        // ponytail: a original empurra as duas palavras em dwords com lixo nos 16 bits altos; a 0x4afbd0 lê as palavras
        embaralhar(local.as_mut_ptr() as usize, m as i16 as u16 as u32, (cqs5_w(this + 0xb6).wrapping_sub(k as i16)) as u16 as u32);
        let mut a: i16 = 0;
        while k < cqs5_d(this + 0xb6) as i32 {
            cqs5_por(this, &mut k, local[a as u16 as usize]);
            a = a.wrapping_add(1);
        }
    }
    if k != cqs5_d(this + 0xb6) as i32 {
        return 0;
    }
    crate::comparacao::ramo("cqs_5 chaves do trofeu da liga");
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    ordenar(cqs5_d(this + 0xb2), cqs5_d(this + 0xb6) as u32, 4, 0x53fd40); // cqs5: Norte e Sul pela latitude
    let metade = (cqs5_d(this + 0xb6) as i32) / 2;
    ordenar(cqs5_d(this + 0xb2), metade as u32, 4, 0x4b0810);
    let metade = (cqs5_d(this + 0xb6) as i32) / 2;
    ordenar(cqs5_d(this + 0xb2) + metade as usize * 4, metade as u32, 4, 0x4b0810);
    1
}

unsafe extern "thiscall" fn cqs5_trofeu_clubes(this: usize) -> u32 {
    cqs5_selecao(this)
}

/// Lado a lado a partir do mesmo estado (o sorteador, +0xb6, [0xb67a34]); a original deixa a lista dela em +0xb2, a cópia
/// faz a sua (sem contar os sorteios), comparada nas +0xb6 vagas e liberada. Quando a original não acha o rebaixado (o
/// caso em que ela lê a dword sem valor da pilha), só a original.
unsafe extern "thiscall" fn comparar_cqs5_trofeu_clubes(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(2030);
    let base = cqs5_d(0xae23b4);
    let tem_rebaixado = (0..cqs5_d(0xae235c) as i32).any(|i| {
        let c = base + i as usize * 0x245;
        let antes = cqs5_d(c + 0x5b);
        cqs5_na(c, 0x9cf69c) && antes != 0 && cqs5_d(antes) == cqs5_d(0x9cf5c8)
    });
    if !tem_rebaixado {
        return o(this);
    }
    let antes = cqs5_d(this + 0xb2);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(this + 0xb6, 4usize), (0xb67a34, 4)]);
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || {
        let r = o(this);
        lo.set(cqs5_d(this + 0xb2));
        cqs5_pd(this + 0xb2, antes);
        r
    }, || {
        let r = crate::aleatorio::sem_contar(|| cqs5_selecao(this));
        lr.set(cqs5_d(this + 0xb2));
        cqs5_pd(this + 0xb2, lo.get());
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let k = 4 * cqs5_d(this + 0xb6).min(0x38);
    let lista = if lo != 0 && lr != 0 && a == 1 {
        core::slice::from_raw_parts(lo as *const u8, k) == core::slice::from_raw_parts(lr as *const u8, k)
    } else {
        true
    };
    if lr != 0 && lr != lo {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(lr);
    }
    PLACAR.registrar(igual && a == b && lista, || format!("cqs5_trofeu_clubes({:#x}) devolveu {} x {} memória {} lista {}", this, a, b, igual, lista));
    a
}

/// { 0x56bab0 } (thiscall, ret 0, eng_auto_cup.cpp; quadro SEH só para o new, sem destrutor): `EngAutoCup::
/// create_section_stage()`: a fase das chaves (Norte e Sul) com os +0x3e clubes da lista +0xb2, na ordem: um vetor
/// (0x944e46 de 4 × +0x3e; sem memória, a caixa da linha 0x2e9) com eles; as rodadas pelo método +0x3c (push 0; push
/// &extra; push &rodadas; push 0; mov ecx -> thiscall, ret 0x10); a fase (0x944cf1(0xb2); Cup_Stage::Cup_Stage 0x522e00,
/// thiscall, ret 0x3c: a copa, +0x3e (word), o vetor, as rodadas (word), +4, a tabela, o ano +0x40 (word), 0, 1, o extra
/// (word), +0x51 (byte), 0, 0, 0, sem sementes; os words e bytes vão com lixo em cima, que o construtor não lê) em
/// [+0xc][0]; o vetor e a tabela liberados (0x9452ca). Sem fase, a caixa da linha 0x300; com ela, +0x30 = 0.
unsafe extern "thiscall" fn cqs5_trofeu_fase_das_chaves(this: usize) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let times = alocar(((cqs5_w(this + 0x3e) as i32) << 2) as u32);
    if times == 0 {
        return aviso_em(0x9c4180, 0x2e9);
    }
    let mut i = 0i32;
    while i < cqs5_w(this + 0x3e) as i32 {
        cqs5_pd(times + i as usize * 4, cqs5_d(cqs5_d(this + 0xb2) + i as usize * 4)); // cqs5: a ordem das chaves
        i += 1;
    }
    let rodadas_da_copa: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(cqs5_d(cqs5_d(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let tabela = rodadas_da_copa(this, 0, &mut rodadas, &mut extra, 0);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let p = novo(0xb2);
    let f = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(0x522e00usize);
        construir(p, this, cqs5_w(this + 0x3e) as u16 as u32, times, rodadas as u16 as u32, cqs5_d(this + 4), tabela, cqs5_w(this + 0x40) as u16 as u32, 0, 1,
            extra as u16 as u32, *((this + 0x51) as *const u8) as u32, 0, 0, 0, core::ptr::null())
    } else {
        0
    };
    cqs5_pd(cqs5_d(this + 0xc), f);
    soltar(times);
    soltar(tabela);
    if cqs5_d(cqs5_d(this + 0xc)) == 0 {
        return aviso_em(0x9c4180, 0x300);
    }
    crate::comparacao::ramo("cqs_5 fase das chaves do trofeu");
    cqs5_pd(this + 0x30, 0);
}

/// Só a original (aloca a fase), conferindo o plano: a fase em [+0xc][0] e +0x30 = 0.
unsafe extern "thiscall" fn comparar_cqs5_trofeu_fase_das_chaves(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(2031);
    o(this);
    let f = cqs5_d(cqs5_d(this + 0xc));
    PLACAR.registrar(f != 0 && cqs5_d(this + 0x30) == 0, || format!("cqs5_trofeu_fase_das_chaves({:#x}) fase {:#x} +0x30 {}", this, f, cqs5_d(this + 0x30) as i32));
}

/// { 0x56bc50 } (thiscall, ret 0, eng_auto_cup.cpp; a virtual +0x28): `EngAutoCup::advance_stage()`: na última fase
/// (+0x30 ≥ +0x2c − 1, com sinal), a caixa da linha 0x314 e nada mais; senão +0x30 sobe um e, ao chegar a 1 (o fim das
/// chaves), monta a final (EngAutoCup::create_final_stage 0x56bce0, call direto com o ecx da copa -> thiscall, ret 0).
unsafe extern "thiscall" fn cqs5_trofeu_fase_seguinte(this: usize) {
    let fase = cqs5_d(this + 0x30) as i32;
    if fase >= (cqs5_d(this + 0x2c) as i32).wrapping_sub(1) {
        return aviso_em(0x9c4180, 0x314);
    }
    let fase = fase + 1;
    cqs5_pd(this + 0x30, fase as usize);
    if fase == 1 {
        crate::comparacao::ramo("cqs_5 final do trofeu");
        let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x56bce0usize);
        f(this);
    }
}

/// Só a original (a final aloca e sorteia), conferindo o plano: +0x30 sobe um fora da última fase e fica na última.
unsafe extern "thiscall" fn comparar_cqs5_trofeu_fase_seguinte(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(2032);
    let antes = cqs5_d(this + 0x30) as i32;
    let ultima = antes >= (cqs5_d(this + 0x2c) as i32).wrapping_sub(1);
    o(this);
    let depois = cqs5_d(this + 0x30) as i32;
    PLACAR.registrar(depois == if ultima { antes } else { antes + 1 }, || format!("cqs5_trofeu_fase_seguinte({:#x}) fase {} -> {}", this, antes, depois));
}

// ---------- 283a leva: o Troféu da Liga inglesa (LDV Vans Trophy): a final entre os campeões das duas chaves e a classificação final da copa nas estatísticas (eng_auto_cup.cpp) ----------

unsafe fn cqs6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqs6_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqs6_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// O primeiro clube da lista de inscritos `l` (entradas de 6 bytes: +0 o clube, +5 a situação) com a situação `s`, entre
/// as `n` primeiras (n = +0x3e da copa, word com sinal); sem ele, None.
unsafe fn cqs6_vivo(l: usize, n: i32, s: u8) -> Option<usize> {
    (0..n.max(0) as usize).find(|&i| *((l + i * 6 + 5) as *const u8) == s).map(|i| cqs6_d(l + i * 6))
}

/// { 0x56bce0 } (thiscall, ret 0, eng_auto_cup.cpp; da fase seguinte, quadro SEH só para o new, sem destrutor):
/// `EngAutoCup::create_final_stage()`: a final do Troféu: um vetor de 2 (0x944e46 de 8; sem memória, a caixa da linha
/// 0x337) com o primeiro clube de situação 1 (o que segue vivo [inferência]) da lista da copa (+0xa7) e o primeiro de
/// situação 1 da lista da fase das chaves ([+0xc][0] +0xa7), ambos entre os +0x3e primeiros (da copa); as rodadas pelo
/// método +0x3c (push 0; push &extra; push &rodadas; push 1; mov ecx -> thiscall, ret 0x10); a fase (0x944cf1(0xb2);
/// Cup_Stage::Cup_Stage 0x522e00, thiscall, ret 0x3c: a copa, 2, o vetor, as rodadas (word), +4, a tabela, o ano +0x40
/// (word), 1, 1, o extra (word), 0x14, 0, 0, 0, sem sementes) em [+0xc][1]; a tabela das rodadas liberada (0x9452ca). Sem
/// fase, a caixa da linha 0x35a (o vetor fica, como na original). Com ela, o sorteio da rodada (Cup::draw_round 0x51c410:
/// push 0; mov ecx, fase -> thiscall, ret 4) e o vetor liberado.
unsafe extern "thiscall" fn cqs6_trofeu_final(this: usize) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let l = alocar(8);
    if l == 0 {
        return aviso_em(0x9c4180, 0x337);
    }
    if let Some(c) = cqs6_vivo(cqs6_d(this + 0xa7), cqs6_w(this + 0x3e) as i32, 1) { // cqs6: o finalista da copa
        cqs6_pd(l, c);
    }
    if let Some(c) = cqs6_vivo(cqs6_d(cqs6_d(cqs6_d(this + 0xc)) + 0xa7), cqs6_w(this + 0x3e) as i32, 1) { // cqs6: o finalista das chaves
        cqs6_pd(l + 4, c);
    }
    let rodadas_da_copa: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(cqs6_d(cqs6_d(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let tabela = rodadas_da_copa(this, 1, &mut rodadas, &mut extra, 0);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let p = novo(0xb2);
    let f = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(0x522e00usize);
        construir(p, this, 2, l, rodadas as u16 as u32, cqs6_d(this + 4), tabela, cqs6_w(this + 0x40) as u16 as u32, 1, 1, // cqs6: o ano da final
             extra as u16 as u32, 0x14, 0, 0, 0, core::ptr::null())
    } else {
        0
    };
    cqs6_pd(cqs6_d(this + 0xc) + 4, f);
    soltar(tabela);
    let f = cqs6_d(cqs6_d(this + 0xc) + 4);
    if f == 0 {
        return aviso_em(0x9c4180, 0x35a);
    }
    crate::comparacao::ramo("cqs_6 final do trofeu");
    let sortear: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x51c410usize);
    sortear(f, 0); // cqs6: o sorteio da final
    soltar(l);
}

/// Só a original (aloca a fase e sorteia), conferindo o plano: a fase em [+0xc][1] (com o vetor dos dois finalistas
/// achados antes, quando há fase).
unsafe extern "thiscall" fn comparar_cqs6_trofeu_final(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(2033);
    o(this);
    let f = cqs6_d(cqs6_d(this + 0xc) + 4);
    PLACAR.registrar(f != 0, || format!("cqs6_trofeu_final({:#x}) sem a fase da final", this));
}

/// { 0x56c010 } (thiscall, ret 0, eng_auto_cup.cpp; a virtual da classificação final, n = 1 até 2002-06-01): `EngAutoCup::
/// rank_teams()`: a posição final de cada clube nas estatísticas da copa (+8, Comp_Stats; sem elas, a caixa da linha 0x3bb).
/// Um vetor de 2 × +0x3e ponteiros (0x944e46; sem memória, a caixa da linha 0x3c3): na primeira metade os clubes da lista
/// da copa (+0xa7, entradas de 6 bytes), na segunda os da fase das chaves ([+0xc][0] +0xa7); cada metade ordenada pela
/// reputação (+0x80, word), da maior para a menor (qsort 0x945333 com 0x4b07d0). Os dois primeiros (o de maior reputação
/// de cada metade): o de reputação maior (com sinal, estritamente) fica em 1º e o outro em 2º (Comp_Stats::
/// set_club_final_position 0x4a2540: push posição; push clube; mov ecx, +8 -> thiscall, ret 8). Depois, para cada metade,
/// da última rodada (+0x3c − 1) para a primeira, tantos clubes (os seguintes da metade, a partir do 2º) quantos a rodada
/// tem em +0x1a da tabela das rodadas (+0xa3, 0x68 bytes por rodada), com as posições 3, 5, 7... (a original empurra o
/// dword do byte da posição com lixo da pilha em cima; a 0x4a2540 só lê o byte). O vetor é liberado.
unsafe extern "thiscall" fn cqs6_trofeu_classificacao(this: usize) {
    if cqs6_d(this + 8) == 0 {
        return aviso_em(0x9c4180, 0x3bb);
    }
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let v = alocar(((cqs6_w(this + 0x3e) as i32) << 3) as u32);
    if v == 0 {
        return aviso_em(0x9c4180, 0x3c3);
    }
    let mut d: i16 = 0;
    while d < cqs6_w(this + 0x3e) {
        let e = d as i32 as usize * 6;
        cqs6_pd(v + d as i32 as usize * 4, cqs6_d(cqs6_d(this + 0xa7) + e));
        let n = cqs6_w(this + 0x3e) as i32;
        cqs6_pd(v + (n + d as i32) as usize * 4, cqs6_d(cqs6_d(cqs6_d(cqs6_d(this + 0xc)) + 0xa7) + e));
        d = d.wrapping_add(1);
    }
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let n = cqs6_w(this + 0x3e) as i32;
    ordenar(v, n as u32, 4, 0x4b07d0);
    let n = cqs6_w(this + 0x3e) as i32;
    ordenar(v + n as usize * 4, n as u32, 4, 0x4b07d0);
    let posicao: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x4a2540usize);
    let a = cqs6_d(v);
    let b = cqs6_d(v + cqs6_w(this + 0x3e) as i32 as usize * 4);
    crate::comparacao::ramo("cqs_6 classificacao do trofeu");
    if cqs6_w(a + 0x80) > cqs6_w(b + 0x80) {
        posicao(cqs6_d(this + 8), a, 1);
        posicao(cqs6_d(this + 8), cqs6_d(v + cqs6_w(this + 0x3e) as i32 as usize * 4), 2);
    } else {
        posicao(cqs6_d(this + 8), a, 2);
        posicao(cqs6_d(this + 8), cqs6_d(v + cqs6_w(this + 0x3e) as i32 as usize * 4), 1);
    }
    for lado in 0..2u8 {
        let mut k: i16 = if lado == 0 { 1 } else { cqs6_w(this + 0x3e).wrapping_add(1) };
        let mut pos: u8 = 3;
        let ultima = cqs6_w(this + 0x3c).wrapping_sub(1);
        if ultima < 0 {
            continue;
        }
        let mut r = ultima as i32;
        while r >= 0 {
            let p = pos;
            let mut j: i16 = 0;
            while j < cqs6_w(cqs6_d(this + 0xa3) + r as usize * 0x68 + 0x1a) {
                let c = cqs6_d(v + k as i32 as usize * 4);
                k = k.wrapping_add(1);
                posicao(cqs6_d(this + 8), c, p as u32);
                pos = pos.wrapping_add(2);
                j = j.wrapping_add(1);
            }
            r -= 1;
        }
    }
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    soltar(v);
}

// ponytail: grava nas estatísticas da copa (Comp_Stats, que acha ou acrescenta a entrada do clube: aloca); só a original
unsafe extern "thiscall" fn comparar_cqs6_trofeu_classificacao(this: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(2034)(this)
}

// ---------- 283a leva: a fase de mata-mata das copas (Cup_Stage): o construtor (a tabela das rodadas copiada, os times com as sementes e a árvore dos confrontos) e o destrutor (cup_stage.cpp) ----------

unsafe fn cqs7_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqs7_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqs7_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

unsafe fn cqs7_pw(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, v)
}

unsafe fn cqs7_pb(a: usize, v: u8) {
    *(a as *mut u8) = v
}

/// { 0x522e00 } (thiscall, ret 0x3c, cup_stage.cpp; quadro SEH sem destrutor no caminho normal): `Cup_Stage::Cup_Stage(pai,
/// n, times, rodadas, a5, modelo, ano, fase, b2, id, b3, arvore, por_ordem, a14, sementes)`: a base (Cup::Cup 0x518640,
/// mov ecx -> thiscall, ret 0); +4 = a5; +0x40 = o ano (word); +0x44 = a fase, +0x42 = b2 (bytes); +0x30 = +0x2c = −1;
/// +0x51 = b3; a vtable 0x969468; +0x43 = 1; +0xb1 = +0xab = +0xac = 0; +0x3c = as rodadas e +0x3a = o id (words);
/// +0x49 e +0x4a copiados do pai; +0x28 = a14. A tabela das rodadas (0x944e46 de rodadas × 0x68 em +0xa3; sem memória, a
/// caixa da linha 0x48) copiada do modelo; +0x3e = n e a lista dos times (calloc 0x945501(n, 6) em +0xa7; sem memória, a
/// caixa da linha 0x53): cada time com a situação (+5) 0 e a semente (+4): a do vetor `sementes`, ou, sem ele, a posição
/// na lista se `por_ordem`, ou 0. Por fim a árvore dos confrontos: com `arvore`, Cup_Stage::map_fixture_tree (0x5241e0),
/// senão Cup::map_fixture_tree (0x518790), as duas thiscall, ret 0 (sorteiam); negativa, a caixa da linha 0x62 ou 0x6a.
/// Devolve this.
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn cqs7_fase_de_copa(this: usize, pai: usize, n: u32, times: usize, rodadas: u32, a5: u32, modelo: usize, ano: u32, fase: u32, b2: u32,
    id: u32, b3: u32, arvore: u32, por_ordem: u32, a14: u32, sementes: usize) -> usize {
    const F: usize = 0x9b5944;
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x518640usize);
    base(this);
    cqs7_pd(this + 4, a5 as usize);
    cqs7_pw(this + 0x40, ano as u16); // cqs7: o ano da fase
    cqs7_pb(this + 0x42, b2 as u8);
    cqs7_pb(this + 0x44, fase as u8);
    cqs7_pd(this + 0x30, usize::MAX);
    cqs7_pd(this + 0x2c, usize::MAX);
    cqs7_pb(this + 0x51, b3 as u8);
    cqs7_pd(this, 0x969468);
    cqs7_pb(this + 0x43, 1);
    cqs7_pb(this + 0xb1, 0);
    cqs7_pb(this + 0xab, 0);
    cqs7_pb(this + 0xac, 0);
    cqs7_pw(this + 0x3c, rodadas as u16);
    cqs7_pw(this + 0x3a, id as u16);
    cqs7_pb(this + 0x49, *((pai + 0x49) as *const u8));
    cqs7_pb(this + 0x4a, *((pai + 0x4a) as *const u8));
    cqs7_pd(this + 0x28, a14 as usize);
    let tamanho = (rodadas as u16 as i16 as i32 * 0x68) as u32;
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let t = alocar(tamanho);
    cqs7_pd(this + 0xa3, t);
    if t == 0 {
        aviso_em(F, 0x48);
        return this;
    }
    core::ptr::copy_nonoverlapping(modelo as *const u8, t as *mut u8, tamanho as usize);
    cqs7_pw(this + 0x3e, n as u16);
    let zerada: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(0x945501usize);
    let l = zerada(n as u16 as i16 as i32 as u32, 6);
    cqs7_pd(this + 0xa7, l);
    if l == 0 {
        aviso_em(F, 0x53);
        return this;
    }
    crate::comparacao::ramo("cqs_7 fase de copa");
    let m = cqs7_w(this + 0x3e) as i32;
    let mut i = 0i32;
    while i < m {
        let e = i as usize * 6;
        cqs7_pd(cqs7_d(this + 0xa7) + e, cqs7_d(times + i as usize * 4)); // cqs7: o time i
        cqs7_pb(cqs7_d(this + 0xa7) + e + 5, 0);
        let s = if sementes != 0 {
            *((sementes + i as usize) as *const u8) // cqs7: a semente do vetor
        } else if por_ordem != 0 {
            i as u8
        } else {
            0
        };
        cqs7_pb(cqs7_d(this + 0xa7) + e + 4, s);
        i += 1;
    }
    let (mapa, linha) = if arvore != 0 { (0x5241e0usize, 0x62) } else { (0x518790usize, 0x6a) };
    let mapa: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(mapa);
    if mapa(this) < 0 {
        aviso_em(F, linha);
    }
    this
}

/// Só a original (a base, as alocações e a árvore, que sorteia: não roda duas vezes), conferindo o plano feito antes: devolve
/// this, +0x3e = n, +0x3c = as rodadas, +0x40 = o ano, as duas tabelas montadas e, na lista, os mesmos clubes.
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn comparar_cqs7_fase_de_copa(this: usize, pai: usize, n: u32, times: usize, rodadas: u32, a5: u32, modelo: usize, ano: u32, fase: u32, b2: u32,
    id: u32, b3: u32, arvore: u32, por_ordem: u32, a14: u32, sementes: usize) -> usize {
    #[allow(clippy::type_complexity)]
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, u32, usize, u32, u32, u32, u32, u32, u32, u32, u32, usize) -> usize>(2035);
    let k = (n as u16 as i16).max(0) as usize;
    let plano: Vec<(usize, u8)> = (0..k).map(|i| {
        let s = if sementes != 0 { *((sementes + i) as *const u8) } else if por_ordem != 0 { i as u8 } else { 0 };
        (cqs7_d(times + i * 4), s)
    }).collect();
    let tamanho = (rodadas as u16 as i16 as i32 * 0x68).max(0) as usize;
    let modelo_antes: Vec<u8> = core::slice::from_raw_parts(modelo as *const u8, tamanho).to_vec();
    let r = o(this, pai, n, times, rodadas, a5, modelo, ano, fase, b2, id, b3, arvore, por_ordem, a14, sementes);
    let l = cqs7_d(this + 0xa7);
    let t = cqs7_d(this + 0xa3);
    // a árvore (chamada no fim) reescreve partes da tabela das rodadas e da lista (na comparação de 06/11 as duas
    // diferiam do que o construtor grava): confere os campos fixos, a tabela montada e o conjunto dos clubes da lista
    let _ = &modelo_antes;
    let ok = r == this && cqs7_w(this + 0x3e) as u16 == n as u16 && cqs7_w(this + 0x3c) as u16 == rodadas as u16 && cqs7_w(this + 0x40) as u16 == ano as u16
        && t != 0 && l != 0 && {
            let mut a: Vec<usize> = (0..plano.len()).map(|i| cqs7_d(l + i * 6)).collect();
            let mut b: Vec<usize> = plano.iter().map(|x| x.0).collect();
            a.sort_unstable();
            b.sort_unstable();
            a == b
        };
    PLACAR.registrar(ok, || format!("cqs7_fase_de_copa({:#x}) n={} rodadas={} ano={}", this, n as u16 as i16, rodadas as u16 as i16, ano as u16));
    r
}

/// { 0x523300 } (thiscall, ret 0, cup_stage.cpp; o destrutor, chamado pelo destrutor virtual): `Cup_Stage::~Cup_Stage()`:
/// a vtable 0x969468; a lista dos times (+0xa7), a tabela das rodadas (+0xa3) e, com os confrontos (+0xad), cada vetor de
/// confrontos das +0x3c rodadas e o próprio vetor liberados (0x9452ca, cdecl; os ponteiros ficam); por fim a base
/// (Cup::~Cup 0x518690, mov ecx -> thiscall, ret 0), cujo eax ela devolve.
unsafe extern "thiscall" fn cqs7_fim_da_fase_de_copa(this: usize) -> usize {
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    cqs7_pd(this, 0x969468);
    if cqs7_d(this + 0xa7) != 0 {
        soltar(cqs7_d(this + 0xa7));
    }
    if cqs7_d(this + 0xa3) != 0 {
        soltar(cqs7_d(this + 0xa3));
    }
    if cqs7_d(this + 0xad) != 0 {
        let mut i = 0i32;
        while i < cqs7_w(this + 0x3c) as i32 {
            let p = cqs7_d(cqs7_d(this + 0xad) + i as usize * 4);
            if p != 0 {
                soltar(p);
            }
            i += 1;
        }
        soltar(cqs7_d(this + 0xad));
    }
    crate::comparacao::ramo("cqs_7 fim da fase de copa");
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x518690usize);
    base(this)
}

// ponytail: libera as tabelas e destrói a base; não pode rodar duas vezes: só a original
unsafe extern "thiscall" fn comparar_cqs7_fim_da_fase_de_copa(this: usize) -> usize {
    orig::<unsafe extern "thiscall" fn(usize) -> usize>(2036)(this)
}

// ---------- 283a leva: os mata-matas da Libertadores e da Merconorte: os classificados dos grupos e a fase seguinte (conmebol_liber.cpp, con_merc_cup.cpp) ----------

unsafe fn cqs10_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqs10_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqs10_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// Os classificados por grupo (+0xbe, byte com sinal).
unsafe fn cqs10_vagas(this: usize) -> i32 {
    *((this + 0xbe) as *const i8) as i32
}

/// O clube da linha i da tabela do grupo 1 (a da própria copa: +0xb1, linhas de 0x3b bytes).
unsafe fn cqs10_do_grupo_um(this: usize, i: usize) -> usize {
    cqs10_d(cqs10_d(this + 0xb1) + i * 0x3b) // cqs10: a linha do grupo 1
}

/// O clube da posição j da fase f ([+0xc][f], um League_Stage): a linha pela 0x6826d0 (League::get_table_row: push j;
/// mov ecx, fase -> thiscall, ret 4) e o clube no +0 dela.
unsafe fn cqs10_da_fase(this: usize, f: usize, j: i32) -> usize {
    let linha: unsafe extern "thiscall" fn(usize, i32) -> usize = core::mem::transmute(0x6826d0usize);
    let fase = cqs10_d(cqs10_d(this + 0xc) + f * 4);
    cqs10_d(linha(fase, j)) // cqs10: a posição j da fase
}

/// O mata-mata depois dos grupos (o mesmo molde na Libertadores e na Merconorte, conferido instrução a instrução: mudam
/// o tamanho da lista, as fases de grupo, a fase do mata-mata, o fonte e as linhas das caixas). A lista (0x944e46(4 ×
/// times): push; add esp, 4 -> cdecl; sem memória, a caixa `linha_sem_lista` e volta) com os +0xbe primeiros do grupo 1
/// (na ordem das linhas) e, de cada fase de grupo 0..`fases` − 1, os +0xbe primeiros (posições 0, 1...); a tabela das
/// rodadas pela virtual +0x3c (push 0; push &id; push &rodadas; push `mata`; mov ecx, this -> thiscall, ret 0x10); a
/// fase (0x944cf1(0xb2) push; add esp, 4 -> cdecl; Cup_Stage::Cup_Stage 0x522e00, thiscall, ret 0x3c: a copa, `times`,
/// a lista, as rodadas, +4, a tabela, o ano +0x40, `mata`, 2, o id, 0x14, 0, 0, 0 e nenhuma semente; a original empurra
/// as rodadas, o id e o ano com lixo nos 16 bits altos, que o construtor não lê) em [+0xc][`mata`]; a lista e a tabela
/// liberadas (0x9452ca). Sem fase: a caixa `linha_sem_fase`. Com ela: o sorteio da rodada 0 (0x51c410 Cup::draw_round:
/// push 0; mov ecx, fase -> thiscall, ret 4). O quadro SEH da original só cobre o construtor (sem destrutor).
unsafe fn cqs10_mata_mata(this: usize, times: u32, fases: usize, mata: u32, arquivo: usize, linha_sem_lista: u32, linha_sem_fase: u32, ramo: &'static str) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let sortear: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x51c410usize);
    let lista = alocar(times * 4);
    if lista == 0 {
        return aviso_em(arquivo, linha_sem_lista);
    }
    let mut n = 0usize;
    let mut i = 0i32;
    while i < cqs10_vagas(this) {
        cqs10_pd(lista + n * 4, cqs10_do_grupo_um(this, i as usize));
        n += 1;
        i += 1;
    }
    for f in 0..fases {
        let mut j = 0i32;
        while j < cqs10_vagas(this) {
            crate::comparacao::ramo(ramo);
            cqs10_pd(lista + n * 4, cqs10_da_fase(this, f, j)); // cqs10: os classificados de cada fase
            n += 1;
            j += 1;
        }
    }
    let rodadas_da_copa: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(cqs10_d(cqs10_d(this) + 0x3c));
    let (mut rodadas, mut id) = (0i16, 0i16);
    let tabela = rodadas_da_copa(this, mata, &mut rodadas, &mut id, 0);
    let p = novo(0xb2);
    let fase = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(0x522e00usize);
        construir(p, this, times, lista, rodadas as u16 as u32, cqs10_d(this + 4), tabela, cqs10_w(this + 0x40) as u16 as u32, mata, 2, id as u16 as u32, 0x14, 0, 0, 0, core::ptr::null())
    } else {
        0
    };
    cqs10_pd(cqs10_d(this + 0xc) + mata as usize * 4, fase);
    soltar(lista);
    soltar(tabela);
    let f = cqs10_d(cqs10_d(this + 0xc) + mata as usize * 4);
    if f == 0 {
        return aviso_em(arquivo, linha_sem_fase);
    }
    sortear(f, 0);
}

/// O plano do mata-mata (o comparador roda só a original: a lista, a tabela e a fase são alocadas e a rodada é
/// sorteada): os classificados lidos antes (o grupo 1 nas linhas, as fases pelas linhas da tabela +0xb1 delas, como a
/// 0x6826d0); depois a fase em [+0xc][`mata`] com +0x3e = `times`, o +4 da copa e os clubes da lista dela (+0xa7, 6
/// bytes) iguais aos do plano como conjunto (o sorteio pode reordenar).
unsafe fn cqs10_comparar_mata_mata(nome: &str, this: usize, times: usize, fases: usize, mata: usize, original: impl FnOnce()) {
    let q = cqs10_vagas(this).max(0) as usize;
    let mut plano: Vec<usize> = (0..q).map(|i| cqs10_d(cqs10_d(this + 0xb1) + i * 0x3b)).collect();
    for f in 0..fases {
        let fase = cqs10_d(cqs10_d(this + 0xc) + f * 4);
        for j in 0..q {
            plano.push(cqs10_d(cqs10_d(fase + 0xb1) + j * 0x3b));
        }
    }
    original();
    let fase = cqs10_d(cqs10_d(this + 0xc) + mata * 4);
    if fase == 0 {
        PLACAR.registrar(false, || format!("{}({:#x}) sem fase", nome, this));
        return;
    }
    let lista = cqs10_d(fase + 0xa7);
    let mut achou: Vec<usize> = (0..times).map(|k| cqs10_d(lista + 6 * k)).collect();
    let (pl, ac) = (plano.clone(), achou.clone());
    plano.sort_unstable();
    achou.sort_unstable();
    let igual = plano == achou && cqs10_w(fase + 0x3e) as usize == times && cqs10_d(fase + 4) == cqs10_d(this + 4);
    PLACAR.registrar(igual, || format!("{}({:#x}) plano={:x?} fase={:#x} lista={:x?} +0x3e={}", nome, this, pl, fase, ac, cqs10_w(fase + 0x3e)));
}

/// { 0x4c3d10 } (thiscall, ret 0, conmebol_liber.cpp): `ConmebolLiber::create_knockout_stage()`, o mata-mata da
/// Libertadores: os 16 (os 2 primeiros do grupo 1 e das 7 fases de grupo) na fase 7; caixas nas linhas 0x24c (sem
/// lista) e 0x26a (sem fase).
unsafe extern "thiscall" fn cqs10_libertadores_mata_mata(this: usize) {
    cqs10_mata_mata(this, 0x10, 7, 7, 0x9a7de4, 0x24c, 0x26a, "cqs_10 classificados da libertadores");
}

unsafe extern "thiscall" fn comparar_cqs10_libertadores_mata_mata(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(2037);
    cqs10_comparar_mata_mata("cqs10_libertadores_mata_mata", this, 0x10, 7, 7, || o(this))
}

/// { 0x4c2330 } (thiscall, ret 0, con_merc_cup.cpp): `ConMercCup::create_semi_final_stage()`, as semifinais da
/// Merconorte: os 4 (o primeiro do grupo 1 e das 3 fases de grupo) na fase 3; caixas nas linhas 0x21d (sem lista) e
/// 0x23b (sem fase).
unsafe extern "thiscall" fn cqs10_merconorte_semifinais(this: usize) {
    cqs10_mata_mata(this, 4, 3, 3, 0x9a7d6c, 0x21d, 0x23b, "cqs_10 classificados da merconorte");
}

unsafe extern "thiscall" fn comparar_cqs10_merconorte_semifinais(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(2038);
    cqs10_comparar_mata_mata("cqs10_merconorte_semifinais", this, 4, 3, 3, || o(this))
}

/// { 0x4c20b0 } (thiscall, ret 0, con_merc_cup.cpp; a virtual +0x28): `ConMercCup::advance_stage()`, a fase seguinte da
/// Merconorte: na última fase (+0x30 ≥ +0x2c − 1, com sinal), a caixa da linha 0x1c7; senão +0x30 sobe e, chegando a 3
/// (depois dos grupos), as semifinais (0x4c2330: mov ecx, this -> thiscall, ret 0).
unsafe extern "thiscall" fn cqs10_merconorte_fase_seguinte(this: usize) {
    let f = cqs10_d(this + 0x30) as i32;
    if f >= (cqs10_d(this + 0x2c) as i32).wrapping_sub(1) {
        return aviso_em(0x9a7d6c, 0x1c7);
    }
    let f = f + 1;
    cqs10_pd(this + 0x30, f as usize);
    if f == 3 { // cqs10: as semifinais da Merconorte depois dos grupos
        crate::comparacao::ramo("cqs_10 semifinais da merconorte");
        let semifinais: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x4c2330usize);
        semifinais(this);
    }
}

// ponytail: as semifinais (0x4c2330) alocam e sorteiam: não rodam duas vezes. Só a original; o plano: +0x30 sobe um
// (fora da última fase, que dá a caixa e não muda nada).
unsafe extern "thiscall" fn comparar_cqs10_merconorte_fase_seguinte(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(2039);
    let f = cqs10_d(this + 0x30) as i32;
    let ultima = f >= (cqs10_d(this + 0x2c) as i32).wrapping_sub(1);
    o(this);
    let depois = cqs10_d(this + 0x30) as i32;
    PLACAR.registrar(depois == if ultima { f } else { f + 1 }, || format!("cqs10_merconorte_fase_seguinte({:#x}) fase {} -> {}", this, f, depois));
}

// ---------- 283a leva: os convidados da Merconorte e a fase seguinte da Mercosul e da Copa América (con_merc_cup.cpp, conmebol_merc.cpp, goldcup.cpp) ----------

unsafe fn cqs11_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqs11_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// Os clubes de uma nação para a Merconorte: até 30, na ordem da tabela de clubes.
struct Cqs11Nacao {
    clubes: [usize; 30],
    n: u8,
}

impl Cqs11Nacao {
    fn novo() -> Self {
        Cqs11Nacao { clubes: [0; 30], n: 0 }
    }

    fn por(&mut self, c: usize) {
        if self.n < 0x1e {
            self.clubes[self.n as usize] = c;
            self.n += 1;
        }
    }
}

/// { 0x4c2560 } (thiscall, ret 0, con_merc_cup.cpp): `ConMercCup::select_invited_clubs()`: +0xf2 = 16 e a lista +0xee
/// (0x944e46(0x40): push; add esp, 4 -> cdecl; sem memória, a caixa da linha 0x262 e devolve 0). Os clubes da tabela
/// ([0xae23b4], 0x245 bytes, [0xae235c] deles; a primeira seleção, 0x53b900 database_is_national_team: push clube; add
/// esp, 4 -> cdecl, encerra a busca) com nação (+0x53) vão, pelo id da nação (+0 dela), para as listas do México
/// ([0x9cf3d0]), da Colômbia ([0x9cf29c]), do Peru ([0x9cf428]) e da Bolívia ([0x9cf254]), até 30 em cada; os de outra
/// nação cuja liga (+0x57) é a [0x9cf590] (a MLS) vão para a lista da liga americana, até 12. Cada lista é ordenada pela
/// reputação e tem a faixa de cima embaralhada (0x4afbd0 comp_util_sort_and_shuffle_top_clubs: push quantos; push a
/// conta (word); push lista; add esp, 0xc -> cdecl; sorteia): México 4, Colômbia 5, Peru 3, Bolívia 2, a liga americana 2.
/// A lista: nas vagas 2..5 os 4 primeiros do México, 6..10 os 5 da Colômbia, 11..13 os 3 do Peru, 14..15 os 2 da
/// Bolívia; nas vagas 0 e 1, sem a competição da liga americana ([[0xadadfc] + 4 × [0x9cf728]] = 0) os 2 primeiros da
/// lista dela; com ela, o campeão do último registro da competição [0x9cf590] (0xae23d0 + 0x6b × id) e o da competição
/// +4 dela (0x4afd80 comp_util_get_latest_placed_club: push lugar; push competição; add esp -> cdecl), e, se forem o
/// mesmo clube, o vice da primeira na vaga 0. A lista embaralhada (0x4afb40(lista, 16): push; push; add esp, 8 -> cdecl;
/// 3 × 16 trocas sorteadas). Devolve 1.
/// A original guarda as listas na pilha sem zerá-las: com menos clubes que as vagas, ela lê sobra; a cópia lê zero
/// [inferência: nesta carreira as quatro nações e a MLS têm clubes que bastam; o comparador confere a lista].
unsafe fn cqs11_convidados(this: usize) -> u32 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let ordenar: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(0x4afbd0usize);
    let colocado: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x4afd80usize);
    let embaralhar: unsafe extern "C" fn(usize, u32) = core::mem::transmute(0x4afb40usize);
    cqs11_pd(this + 0xf2, 0x10);
    let americana = cqs11_d(cqs11_d(0xadadfc) + cqs11_d(0x9cf728) * 4);
    let lista = alocar(0x40);
    cqs11_pd(this + 0xee, lista);
    if lista == 0 {
        aviso_em(0x9a7d6c, 0x262);
        return 0;
    }
    let (mut mexico, mut colombia, mut peru, mut bolivia) = (Cqs11Nacao::novo(), Cqs11Nacao::novo(), Cqs11Nacao::novo(), Cqs11Nacao::novo());
    let mut mls = [0usize; 12];
    let mut n_mls = 0i8;
    let mut i = 0i32;
    let mut c = cqs11_d(0xae23b4);
    while i < cqs11_d(0xae235c) as i32 {
        if selecao(c) != 0 {
            break;
        }
        let nacao = cqs11_d(c + 0x53);
        if nacao != 0 {
            let id = cqs11_d(nacao);
            if id == cqs11_d(0x9cf3d0) {
                mexico.por(c);
            } else if id == cqs11_d(0x9cf29c) {
                colombia.por(c);
            } else if id == cqs11_d(0x9cf428) {
                peru.por(c);
            } else if id == cqs11_d(0x9cf254) {
                bolivia.por(c);
            } else {
                let liga = cqs11_d(c + 0x57);
                if liga != 0 && cqs11_d(liga) == cqs11_d(0x9cf590) && n_mls < 0xc {
                    mls[n_mls as usize] = c;
                    n_mls += 1;
                }
            }
        }
        i += 1;
        c += 0x245;
    }
    crate::comparacao::ramo("cqs_11 convidados da merconorte");
    ordenar(mexico.clubes.as_mut_ptr() as usize, mexico.n as u32, 4); // cqs11: os 4 do México
    ordenar(colombia.clubes.as_mut_ptr() as usize, colombia.n as u32, 5);
    ordenar(peru.clubes.as_mut_ptr() as usize, peru.n as u32, 3);
    ordenar(bolivia.clubes.as_mut_ptr() as usize, bolivia.n as u32, 2);
    ordenar(mls.as_mut_ptr() as usize, n_mls as i16 as u16 as u32, 2);
    let vaga = |k: usize, v: usize| cqs11_pd(cqs11_d(this + 0xee) + k * 4, v);
    vaga(2, mexico.clubes[0]);
    vaga(3, mexico.clubes[1]);
    vaga(4, mexico.clubes[2]);
    vaga(5, mexico.clubes[3]);
    for k in 0..5 {
        vaga(6 + k, colombia.clubes[k]);
    }
    for k in 0..3 {
        vaga(11 + k, peru.clubes[k]);
    }
    vaga(14, bolivia.clubes[0]);
    vaga(15, bolivia.clubes[1]);
    if americana == 0 {
        vaga(0, mls[0]);
        vaga(1, mls[1]);
    } else {
        let registro = cqs11_d(0xae23d0) + cqs11_d(0x9cf590) * 0x6b;
        crate::comparacao::ramo("cqs_11 campeao da liga americana");
        vaga(0, colocado(registro, 0)); // cqs11: o campeão da liga americana
        vaga(1, colocado(cqs11_d(americana + 4), 0));
        let l = cqs11_d(this + 0xee);
        if cqs11_d(l) == cqs11_d(l + 4) {
            vaga(0, colocado(registro, 1));
        }
    }
    embaralhar(cqs11_d(this + 0xee), 0x10);
    1
}

unsafe extern "thiscall" fn cqs11_merconorte_convidados(this: usize) -> u32 {
    cqs11_convidados(this)
}

/// Lado a lado: o +0xf2, o [0xb67a34] e o estado do sorteador a partir do mesmo estado (a cópia sorteia sem contar); a
/// original deixa a lista dela em +0xee, a cópia faz a sua, comparada nas 16 vagas e liberada. Os sorteios das listas
/// das nações ficam na pilha de cada uma: só a lista final e o sorteador contam.
unsafe extern "thiscall" fn comparar_cqs11_merconorte_convidados(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(2040);
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let a_ = crate::aleatorio::REGIOES_DO_ESTADO;
    let reg = [(this + 0xf2, 4usize), (0xb67a34, 4), a_[0], a_[1]];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || {
        let r = o(this);
        lo.set(cqs11_d(this + 0xee));
        r
    }, || {
        let r = crate::aleatorio::sem_contar(|| cqs11_convidados(this));
        lr.set(cqs11_d(this + 0xee));
        cqs11_pd(this + 0xee, lo.get());
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let lista = if lo != 0 && lr != 0 {
        core::slice::from_raw_parts(lo as *const u8, 0x40) == core::slice::from_raw_parts(lr as *const u8, 0x40)
    } else {
        lo == 0 && lr == 0
    };
    if lr != 0 && lr != lo {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(lr);
    }
    PLACAR.registrar(igual && a == b && lista, || format!("cqs11_merconorte_convidados({:#x}) devolveu {} x {} memória {} lista {}", this, a, b, igual, lista));
    a
}

/// A fase seguinte de um torneio com grupos e mata-mata (`advance_stage`, a virtual +0x28): na última fase (+0x30 ≥
/// +0x2c − 1, com sinal), a caixa da linha e volta; senão +0x30 sobe e, chegando a `mata`, o mata-mata (call direto com
/// o ecx da copa -> thiscall, ret 0).
unsafe fn cqs11_fase_seguinte(this: usize, arquivo: usize, linha: u32, mata: i32, rotina: usize, ramo: &'static str) {
    let f = cqs11_d(this + 0x30) as i32;
    if f >= (cqs11_d(this + 0x2c) as i32).wrapping_sub(1) {
        return aviso_em(arquivo, linha);
    }
    let f = f + 1;
    cqs11_pd(this + 0x30, f as usize);
    if f == mata {
        crate::comparacao::ramo(ramo);
        let m: unsafe extern "thiscall" fn(usize) = core::mem::transmute(rotina);
        m(this);
    }
}

// ponytail: o mata-mata aloca e sorteia: não roda duas vezes. Só a original; o plano: +0x30 sobe um (fora da última
// fase, que dá a caixa e não muda nada).
unsafe fn cqs11_comparar_fase_seguinte(nome: &str, this: usize, original: impl FnOnce()) {
    let f = cqs11_d(this + 0x30) as i32;
    let ultima = f >= (cqs11_d(this + 0x2c) as i32).wrapping_sub(1);
    original();
    let depois = cqs11_d(this + 0x30) as i32;
    PLACAR.registrar(depois == if ultima { f } else { f + 1 }, || format!("{}({:#x}) fase {} -> {}", nome, this, f, depois));
}

/// { 0x4c5140 } (thiscall, ret 0, conmebol_merc.cpp; a virtual +0x28): `ConmebolMerc::advance_stage()`, a fase seguinte
/// da Mercosul: a caixa da linha 0x1d9 na última; chegando a 4 (depois dos 4 grupos-fase), as quartas (0x4c53d0).
unsafe extern "thiscall" fn cqs11_mercosul_fase_seguinte(this: usize) {
    cqs11_fase_seguinte(this, 0x9a7e5c, 0x1d9, 4, 0x4c53d0, "cqs_11 quartas da mercosul");
}

unsafe extern "thiscall" fn comparar_cqs11_mercosul_fase_seguinte(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(2041);
    cqs11_comparar_fase_seguinte("cqs11_mercosul_fase_seguinte", this, || o(this))
}

/// { 0x5e1620 } (thiscall, ret 0, goldcup.cpp; a virtual +0x28): `GoldCup::advance_stage()`, a fase seguinte da Copa
/// América: a caixa da linha 0x29d na última; chegando a 2 (depois dos 2 grupos-fase), o mata-mata (0x5e18a0).
unsafe extern "thiscall" fn cqs11_copa_america_fase_seguinte(this: usize) {
    cqs11_fase_seguinte(this, 0x9cd640, 0x29d, 2, 0x5e18a0, "cqs_11 mata-mata da copa america");
}

unsafe extern "thiscall" fn comparar_cqs11_copa_america_fase_seguinte(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(2042);
    cqs11_comparar_fase_seguinte("cqs11_copa_america_fase_seguinte", this, || o(this))
}

// ---------- 283a leva: a Copa América: as 12 seleções, as posições previstas e a temporada nova (goldcup.cpp) ----------

unsafe fn cqs12_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqs12_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqs12_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// A seleção (o clube da seleção principal) da nação de id [`global`]: 0x53b3d0 database_get_national_team (push nação;
/// add esp -> cdecl) com a nação da tabela [0xae23a8] (0x122 bytes cada).
unsafe fn cqs12_selecao(id: usize) -> usize {
    let selecao: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    selecao(cqs12_d(0xae23a8) + id * 0x122)
}

/// As 12 da Copa América de 2001, na ordem da lista (os grupos intercalados: A nas vagas 0, 3, 6 e 9; B nas 1, 4, 7 e
/// 10; C nas 2, 5, 8 e 11): os globais dos ids das nações.
const CQS12_DE_2001: [usize; 12] = [
    0x9cf29c, 0x9cf260, 0x9cf2a8, 0x9cf294, // cqs12: as vagas 0 a 3 (a Colômbia na 0)
    0x9cf3d0, 0x9cf33c, 0x9cf2d4, 0x9cf428, // cqs12: as vagas 4 a 7 (o México na 4, o Peru na 7)
    0x9cf4f4, 0x9cf504, 0x9cf420, 0x9cf254, // cqs12: as vagas 8 a 11 (a Bolívia na 11)
];

/// { 0x5e1cd0 } (thiscall, ret 0, goldcup.cpp): `GoldCup::set_up_team_list()`: +0xf2 = 12 e a lista +0xee (0x944e46(0x30):
/// push; add esp, 4 -> cdecl; sem memória, a caixa da linha 0x37b e devolve 0). Em 2001 (+0x40 = 0x7d1): as 12 fixas
/// (CQS12_DE_2001, cada uma pela 0x53b3d0) e devolve 1, sem ordenar. Nos outros anos: as sedes da Copa América
/// ([0x9cf7a8]) no ano (0x5fa730 HostCountry::find_hosts em [0xb5e134]: push 1; push &segunda; push &primeira; push ano;
/// push competição -> thiscall, ret 0x14; as duas começam em −1; devolve quantas, byte com sinal) na frente; depois,
/// pela ordem das nações ([0xae23a8], [0xae2350] delas, i como palavra) e enquanto a lista não enche (+0xf2), as do
/// continente [0x9cfa1c] (+0x71, o +0 dele) e as convidadas: em 1999 (0x7cf) a [0x9cf36c], nos outros anos a [0x9cf280];
/// e sempre o México ([0x9cf3d0]); cada uma entra se não é a nação de uma sede (+0x53 da seleção). Sem 12: a caixa da
/// linha 0x3bc e devolve 0; com 12, a lista ordenada pela reputação (qsort 0x945333 com 0x4b07d0: 4 pushes; add esp ->
/// cdecl) e devolve 1.
unsafe fn cqs12_selecoes(this: usize) -> u32 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let selecao: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    cqs12_pd(this + 0xf2, 0xc);
    let lista = alocar(0x30);
    cqs12_pd(this + 0xee, lista);
    if lista == 0 {
        aviso_em(0x9cd640, 0x37b);
        return 0;
    }
    let ano = cqs12_w(this + 0x40) as u16;
    if ano == 0x7d1 {
        crate::comparacao::ramo("cqs_12 selecoes de 2001");
        for (k, &g) in CQS12_DE_2001.iter().enumerate() {
            cqs12_pd(cqs12_d(this + 0xee) + k * 4, cqs12_selecao(cqs12_d(g)));
        }
        return 1;
    }
    let sedes: unsafe extern "thiscall" fn(usize, usize, u32, *mut i32, *mut i32, u32) -> u32 = core::mem::transmute(0x5fa730usize);
    let mut ids = [-1i32; 2];
    let (a, b) = ids.split_at_mut(1);
    // ponytail: a original empurra o eax com o ano nos 16 bits baixos e sobra nos altos; a 0x5fa730 só lê o word
    let quantas = sedes(cqs12_d(0xb5e134), cqs12_d(0x9cf7a8), ano as u32, a.as_mut_ptr(), b.as_mut_ptr(), 1) as u8 as i8;
    let mut k = 0i16;
    while k < quantas as i16 {
        cqs12_pd(cqs12_d(this + 0xee) + k as usize * 4, cqs12_selecao(ids[k as usize] as usize));
        k += 1;
    }
    let mut i = 0i16;
    let mut nacao = cqs12_d(0xae23a8);
    while (i as i32) < cqs12_d(0xae2350) as i32 && (k as i32) < cqs12_d(this + 0xf2) as i32 {
        let continente = cqs12_d(nacao + 0x71);
        let id = cqs12_d(nacao);
        let entra = (continente != 0 && cqs12_d(continente) == cqs12_d(0x9cfa1c))
            || (ano == 0x7cf && id == cqs12_d(0x9cf36c))
            || (ano != 0x7cf && id == cqs12_d(0x9cf280))
            || id == cqs12_d(0x9cf3d0);
        if entra {
            let l = cqs12_d(this + 0xee);
            let sede = (0..quantas.max(0) as usize).any(|j| cqs12_d(cqs12_d(l + j * 4) + 0x53) == nacao);
            if !sede {
                cqs12_pd(cqs12_d(this + 0xee) + k as usize * 4, selecao(nacao));
                k += 1;
            }
        }
        i = i.wrapping_add(1);
        nacao += 0x122;
    }
    if k as i32 != cqs12_d(this + 0xf2) as i32 {
        aviso_em(0x9cd640, 0x3bc);
        return 0;
    }
    ordenar(cqs12_d(this + 0xee), k as i32 as u32, 4, 0x4b07d0);
    1
}

unsafe extern "thiscall" fn cqs12_copa_america_selecoes(this: usize) -> u32 {
    cqs12_selecoes(this)
}

/// Lado a lado: o +0xf2 e o [0xb67a34] a partir do mesmo estado (nada sorteia); a original deixa a lista dela em +0xee, a
/// cópia faz a sua, comparada nas 12 vagas e liberada.
unsafe extern "thiscall" fn comparar_cqs12_copa_america_selecoes(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(2043);
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let reg = [(this + 0xf2, 4usize), (0xb67a34, 4)];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || {
        let r = o(this);
        lo.set(cqs12_d(this + 0xee));
        r
    }, || {
        let r = cqs12_selecoes(this);
        lr.set(cqs12_d(this + 0xee));
        cqs12_pd(this + 0xee, lo.get());
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let lista = if lo != 0 && lr != 0 {
        a == 0 || core::slice::from_raw_parts(lo as *const u8, 0x30) == core::slice::from_raw_parts(lr as *const u8, 0x30)
    } else {
        lo == 0 && lr == 0
    };
    if lr != 0 && lr != lo {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(lr);
    }
    PLACAR.registrar(igual && a == b && lista, || format!("cqs12_copa_america_selecoes({:#x}) devolveu {} x {} memória {} lista {}", this, a, b, igual, lista));
    a
}

/// As posições previstas da Copa América, pela ordem de reputação das 12.
const CQS12_PREVISTAS: [u32; 12] = [1, 2, 3, 3, 5, 5, 5, 5, 9, 0xa, 0xa, 0xa];

/// { 0x5e2340 } (thiscall, ret 0, goldcup.cpp; a virtual +0x5c): `GoldCup::set_predicted_positions()`: sem estatística
/// (+8), a caixa da linha 0x419 e volta; uma cópia da lista (0x944e46(+0xf2 × 4); sem memória, a caixa da linha 0x421 e
/// volta), ordenada pela reputação (qsort 0x945333 com 0x4b07d0, +0xf2 itens), e a posição prevista de cada uma na
/// estatística (0x4a2540 Comp_Stats::set_club_final_position: push posição; push clube; mov ecx, [+8] -> thiscall,
/// ret 8): 1, 2, 3, 3, quatro 5, 9 e três 10 (as 12 primeiras da cópia); a cópia liberada (0x9452ca).
unsafe extern "thiscall" fn cqs12_copa_america_previstas(this: usize) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let posicao: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x4a2540usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    if cqs12_d(this + 8) == 0 {
        return aviso_em(0x9cd640, 0x419);
    }
    let v = alocar((cqs12_d(this + 0xf2) << 2) as u32);
    if v == 0 {
        return aviso_em(0x9cd640, 0x421);
    }
    let n = cqs12_d(this + 0xf2) << 2;
    core::ptr::copy_nonoverlapping(cqs12_d(this + 0xee) as *const u8, v as *mut u8, n);
    ordenar(v, cqs12_d(this + 0xf2) as u32, 4, 0x4b07d0);
    crate::comparacao::ramo("cqs_12 previstas da copa america");
    for (k, &p) in CQS12_PREVISTAS.iter().enumerate() {
        posicao(cqs12_d(this + 8), cqs12_d(v + k * 4), p);
    }
    soltar(v);
}

// ponytail: a estatística acha ou cria a entrada de cada seleção (aloca): não roda duas vezes. Só a original; o plano:
// a lista ordenada pela reputação antes, e depois a posição (+0x72 da entrada, 0x4a4850 Comp_Stats::find_club_entry) de
// cada uma das 12 igual à da faixa.
unsafe extern "thiscall" fn comparar_cqs12_copa_america_previstas(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(2044);
    if cqs12_d(this + 8) == 0 || cqs12_d(this + 0xee) == 0 || cqs12_d(this + 0xf2) != 0xc {
        return o(this);
    }
    let ordenar: unsafe extern "C" fn(*mut usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let achar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
    let mut v: Vec<usize> = (0..12).map(|i| cqs12_d(cqs12_d(this + 0xee) + i * 4)).collect();
    ordenar(v.as_mut_ptr(), 12, 4, 0x4b07d0);
    o(this);
    let errado = (0..12).find(|&k| {
        let e = if v[k] != 0 { achar(cqs12_d(this + 8), v[k]) } else { 0 };
        e == 0 || *((e + 0x72) as *const u8) as u32 != CQS12_PREVISTAS[k]
    });
    PLACAR.registrar(errado.is_none(), || format!("cqs12_copa_america_previstas: a posição prevista da vaga {:?} não bate", errado));
}

/// { 0x5e1530 } (thiscall, ret 0, goldcup.cpp; a virtual +8): `GoldCup::start_new_season()`, a Copa América seguinte:
/// +0x4c = 0, a tabela da copa liberada (0x687970 League::free_table: push 0; mov ecx, this -> thiscall, ret 4), a
/// tabela das rodadas +0xba liberada (0x9452ca) e zerada, a temporada arquivada na estatística (0x4a1c50
/// Comp_Stats::archive_season_and_reset: push 1; mov ecx, [+8] -> thiscall, ret 4), as fases 0..+0x30 destruídas (a
/// virtual 0 de cada uma com 1: o destrutor que libera), o ano +0x40 sobe 2, +0x45 = 0, +0x30 = −1 e as regras de novo
/// (a virtual +0x8c, thiscall, ret 0; 0: a caixa da linha 0x28b).
unsafe extern "thiscall" fn cqs12_copa_america_temporada_nova(this: usize) {
    let soltar_tabela: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x687970usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let arquivar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x4a1c50usize);
    cqs12_pd(this + 0x4c, 0);
    soltar_tabela(this, 0);
    if cqs12_d(this + 0xba) != 0 {
        soltar(cqs12_d(this + 0xba));
        cqs12_pd(this + 0xba, 0);
    }
    if cqs12_d(this + 8) != 0 {
        arquivar(cqs12_d(this + 8), 1);
    }
    if (cqs12_d(this + 0x30) as i32) >= 0 {
        let mut i = 0i32;
        loop {
            let f = cqs12_d(cqs12_d(this + 0xc) + i as usize * 4);
            if f != 0 {
                let destruir: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(cqs12_d(cqs12_d(f)));
                destruir(f, 1);
            }
            i += 1;
            if i > cqs12_d(this + 0x30) as i32 {
                break;
            }
        }
    }
    crate::comparacao::ramo("cqs_12 temporada nova da copa america");
    core::ptr::write_unaligned((this + 0x40) as *mut i16, cqs12_w(this + 0x40).wrapping_add(2)); // cqs12: a Copa América a cada 2 anos
    cqs12_pd(this + 0x45, 0);
    cqs12_pd(this + 0x30, usize::MAX);
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cqs12_d(cqs12_d(this) + 0x8c));
    if regras(this) == 0 {
        aviso_em(0x9cd640, 0x28b);
    }
}

// ponytail: libera as tabelas e destrói as fases: não roda duas vezes. Só a original; o plano: o ano sobe 2, +0x45 = 0,
// +0x30 = −1, +0xba = 0 e +0x4c = 0 (as regras gravam a tabela das rodadas de novo em +0xba: conferida só que existe
// quando havia antes).
unsafe extern "thiscall" fn comparar_cqs12_copa_america_temporada_nova(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(2045);
    let ano = cqs12_w(this + 0x40);
    o(this);
    let ok = cqs12_w(this + 0x40) == ano.wrapping_add(2) && cqs12_d(this + 0x45) == 0 && cqs12_d(this + 0x30) as i32 == -1 && cqs12_d(this + 0x4c) == 0;
    PLACAR.registrar(ok, || format!("cqs12_copa_america_temporada_nova({:#x}): ano {} -> {}, +0x45 {}, +0x30 {}", this, ano, cqs12_w(this + 0x40), cqs12_d(this + 0x45), cqs12_d(this + 0x30) as i32));
}

// ---------- 283a leva: os construtores dos estaduais: Baiano, Centro-Oeste e Gaúcho (as regras, a tabela, os jogos, as estatísticas e as posições previstas na criação) (bra_reg_bahia.cpp, bra_reg_central.cpp, bra_reg_gaucho.cpp) ----------

unsafe fn cqt1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// Um estadual: a vtable, as fases além da liga (+0x2c: 1 nos de pontos corridos com mata-mata, 3 nos de dois turnos e
/// final; o vetor [+0xc] = malloc de 4 × fases), as regras (thiscall, ret 0), a tabela (thiscall, ret 0), as posições
/// previstas (thiscall, ret 0), o fonte e as linhas das caixas (regras, tabela, jogos, ordem, estatísticas).
struct Cqt1Estadual {
    vtabela: usize,
    fases: u32,
    regras: usize,
    tabela: usize,
    previstas: usize,
    arquivo: usize,
    linhas: [u32; 5],
}

/// O construtor de um estadual (o mesmo molde de 544 bytes nos doze, conferido instrução a instrução: só mudam a
/// vtable, as fases, as três rotinas próprias, o fonte e as linhas). League::League (0x682200, thiscall, ret 0); +4 = o
/// registro da competição; a vtable; o ano (+0x40, word) = `ano`; a regra de disciplina (+0x50) = 7. Se o estado
/// salvo carrega (League::load_from_temp_file 0x687b10, thiscall(this, 1), ret 4; devolve ≠ 0), devolve this. Senão
/// (a carreira nova): +0x51 = 0xf (a importância), +0x44 = 0xff, +0x30 = −1, +0x2c = as fases e o vetor delas (malloc
/// 0x944e46, sem conferir); então, cada um com a sua caixa (e devolvendo this) se devolver 0: as regras, a tabela, os
/// jogos (League::create_fixtures 0x6835c0, thiscall, ret 0) e a ordem (League::sort_table 0x6827d0, thiscall(this,
/// 0), ret 4); as estatísticas (operator new 0x944cf1 de 0x5ce e Comp_Stats::Comp_Stats 0x49ee70, thiscall(novo,
/// this), ret 4; o quadro SEH da original cobre só esse new) em +8 (sem elas: a caixa); por fim as posições previstas.
#[allow(clippy::too_many_arguments)]
unsafe fn cqt1_construir(this: usize, ano: u32, registro: usize, e: &Cqt1Estadual, disciplina: u8, previstas_sim: bool, ramo: &'static str) -> usize {
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x682200usize);
    let carregar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x687b10usize);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(e.regras);
    let tabela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(e.tabela);
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x6835c0usize);
    let ordenar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x6827d0usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let estatisticas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x49ee70usize);
    let previstas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(e.previstas);
    base(this);
    core::ptr::write_unaligned((this + 4) as *mut usize, registro);
    core::ptr::write_unaligned(this as *mut usize, e.vtabela);
    core::ptr::write_unaligned((this + 0x40) as *mut u16, ano as u16);
    *((this + 0x50) as *mut u8) = disciplina;
    if carregar(this, 1) != 0 {
        return this;
    }
    crate::comparacao::ramo(ramo);
    *((this + 0x51) as *mut u8) = 0xf;
    *((this + 0x44) as *mut u8) = 0xff;
    core::ptr::write_unaligned((this + 0x30) as *mut i32, -1);
    core::ptr::write_unaligned((this + 0x2c) as *mut u32, e.fases);
    let v = alocar(e.fases * 4);
    core::ptr::write_unaligned((this + 0xc) as *mut usize, v);
    if regras(this) == 0 {
        aviso_em(e.arquivo, e.linhas[0]);
        return this;
    }
    if tabela(this) == 0 {
        aviso_em(e.arquivo, e.linhas[1]);
        return this;
    }
    if jogos(this) == 0 {
        aviso_em(e.arquivo, e.linhas[2]);
        return this;
    }
    if ordenar(this, 0) == 0 {
        aviso_em(e.arquivo, e.linhas[3]);
        return this;
    }
    let n = novo(0x5ce);
    let s = if n != 0 { estatisticas(n, this) } else { 0 };
    core::ptr::write_unaligned((this + 8) as *mut usize, s);
    if s == 0 {
        aviso_em(e.arquivo, e.linhas[4]);
        return this;
    }
    if previstas_sim {
        previstas(this);
    }
    this
}

/// O comparador dos três: alocam e sorteiam os jogos: só a original, conferindo o que ela grava: a devolução, a
/// vtable, o ano, o registro e a disciplina; na liga nova (+0x44 = 0xff, com as estatísticas), +0x2c, o vetor das
/// fases e a importância +0x51 = 0xf.
unsafe fn cqt1_comparar(nome: &str, this: usize, ano: u32, registro: usize, e: &Cqt1Estadual, original: impl FnOnce() -> usize) -> usize {
    let r = original();
    let mut ok = r == this && cqt1_d(this) == e.vtabela && core::ptr::read_unaligned((this + 0x40) as *const u16) == ano as u16
        && cqt1_d(this + 4) == registro && *((this + 0x50) as *const u8) == 7;
    if *((this + 0x44) as *const u8) == 0xff && cqt1_d(this + 8) != 0 {
        ok = ok && cqt1_d(this + 0x2c) == e.fases as usize && cqt1_d(this + 0xc) != 0 && *((this + 0x51) as *const u8) == 0xf;
    }
    PLACAR.registrar(ok, || format!("{}({:#x}): devolveu {:#x}, vtable {:#x}, +0x2c = {}, +0xc = {:#x}, +8 = {:#x}", nome, this, r, cqt1_d(this), cqt1_d(this + 0x2c), cqt1_d(this + 0xc), cqt1_d(this + 8)));
    r
}

const CQT1_BAIANO: Cqt1Estadual = Cqt1Estadual {
    vtabela: 0x967fb4,
    fases: 1,
    regras: 0x43bc70,
    tabela: 0x42fd10,
    previstas: 0x430340,
    arquivo: 0x98acac,
    linhas: [0x4d, 0x54, 0x5b, 0x64, 0x6b],
};

const CQT1_CENTRO_OESTE: Cqt1Estadual = Cqt1Estadual {
    vtabela: 0x968068,
    fases: 1,
    regras: 0x430ff0,
    tabela: 0x430ef0,
    previstas: 0x431610,
    arquivo: 0x98ae10,
    linhas: [0x4c, 0x53, 0x59, 0x60, 0x67],
};

const CQT1_GAUCHO: Cqt1Estadual = Cqt1Estadual {
    vtabela: 0x96811c,
    fases: 1,
    regras: 0x4322d0,
    tabela: 0x4321d0,
    previstas: 0x432980,
    arquivo: 0x98ae50,
    linhas: [0x4d, 0x54, 0x5b, 0x64, 0x6b],
};

/// { 0x42f2d0 } (thiscall, ret 8, bra_reg_bahia.cpp): `BraRegBahia::BraRegBahia(ano, registro)`: o Baiano (uma fase além da liga).
unsafe extern "thiscall" fn cqt1_construir_baiano(this: usize, ano: u32, registro: usize) -> usize {
    cqt1_construir(this, ano, registro, &CQT1_BAIANO, 7, true, "cqt_1 baiano novo") // cqt1: a disciplina e as posições previstas do Baiano
}

/// { 0x4304b0 } (thiscall, ret 8, bra_reg_central.cpp): `BraRegCentral::BraRegCentral(ano, registro)`: o Centro-Oeste (uma fase além da liga).
unsafe extern "thiscall" fn cqt1_construir_centro_oeste(this: usize, ano: u32, registro: usize) -> usize {
    cqt1_construir(this, ano, registro, &CQT1_CENTRO_OESTE, 7, true, "cqt_1 centro_oeste novo") // cqt1: a disciplina e as posições previstas do Centro-Oeste
}

/// { 0x4317c0 } (thiscall, ret 8, bra_reg_gaucho.cpp): `BraRegGaucho::BraRegGaucho(ano, registro)`: o Gaúcho (uma fase além da liga).
unsafe extern "thiscall" fn cqt1_construir_gaucho(this: usize, ano: u32, registro: usize) -> usize {
    cqt1_construir(this, ano, registro, &CQT1_GAUCHO, 7, true, "cqt_1 gaucho novo") // cqt1: a disciplina e as posições previstas do Gaúcho
}

unsafe extern "thiscall" fn comparar_cqt1_construir_baiano(this: usize, ano: u32, registro: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(2046);
    cqt1_comparar("cqt1_construir_baiano", this, ano, registro, &CQT1_BAIANO, || o(this, ano, registro))
}

unsafe extern "thiscall" fn comparar_cqt1_construir_centro_oeste(this: usize, ano: u32, registro: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(2047);
    cqt1_comparar("cqt1_construir_centro_oeste", this, ano, registro, &CQT1_CENTRO_OESTE, || o(this, ano, registro))
}

unsafe extern "thiscall" fn comparar_cqt1_construir_gaucho(this: usize, ano: u32, registro: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(2048);
    cqt1_comparar("cqt1_construir_gaucho", this, ano, registro, &CQT1_GAUCHO, || o(this, ano, registro))
}

// ---------- 283a leva: os construtores dos estaduais: Goiano, Mineiro e Norte (as regras, a tabela, os jogos, as estatísticas e as posições previstas na criação) (bra_reg_goias.cpp, bra_reg_minas_gerais.cpp, bra_reg_north.cpp) ----------

unsafe fn cqt2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// Um estadual: a vtable, as fases além da liga (+0x2c: 1 nos de pontos corridos com mata-mata, 3 nos de dois turnos e
/// final; o vetor [+0xc] = malloc de 4 × fases), as regras (thiscall, ret 0), a tabela (thiscall, ret 0), as posições
/// previstas (thiscall, ret 0), o fonte e as linhas das caixas (regras, tabela, jogos, ordem, estatísticas).
struct Cqt2Estadual {
    vtabela: usize,
    fases: u32,
    regras: usize,
    tabela: usize,
    previstas: usize,
    arquivo: usize,
    linhas: [u32; 5],
}

/// O construtor de um estadual (o mesmo molde de 544 bytes nos doze, conferido instrução a instrução: só mudam a
/// vtable, as fases, as três rotinas próprias, o fonte e as linhas). League::League (0x682200, thiscall, ret 0); +4 = o
/// registro da competição; a vtable; o ano (+0x40, word) = `ano`; a regra de disciplina (+0x50) = 7. Se o estado
/// salvo carrega (League::load_from_temp_file 0x687b10, thiscall(this, 1), ret 4; devolve ≠ 0), devolve this. Senão
/// (a carreira nova): +0x51 = 0xf (a importância), +0x44 = 0xff, +0x30 = −1, +0x2c = as fases e o vetor delas (malloc
/// 0x944e46, sem conferir); então, cada um com a sua caixa (e devolvendo this) se devolver 0: as regras, a tabela, os
/// jogos (League::create_fixtures 0x6835c0, thiscall, ret 0) e a ordem (League::sort_table 0x6827d0, thiscall(this,
/// 0), ret 4); as estatísticas (operator new 0x944cf1 de 0x5ce e Comp_Stats::Comp_Stats 0x49ee70, thiscall(novo,
/// this), ret 4; o quadro SEH da original cobre só esse new) em +8 (sem elas: a caixa); por fim as posições previstas.
#[allow(clippy::too_many_arguments)]
unsafe fn cqt2_construir(this: usize, ano: u32, registro: usize, e: &Cqt2Estadual, disciplina: u8, previstas_sim: bool, ramo: &'static str) -> usize {
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x682200usize);
    let carregar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x687b10usize);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(e.regras);
    let tabela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(e.tabela);
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x6835c0usize);
    let ordenar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x6827d0usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let estatisticas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x49ee70usize);
    let previstas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(e.previstas);
    base(this);
    core::ptr::write_unaligned((this + 4) as *mut usize, registro);
    core::ptr::write_unaligned(this as *mut usize, e.vtabela);
    core::ptr::write_unaligned((this + 0x40) as *mut u16, ano as u16);
    *((this + 0x50) as *mut u8) = disciplina;
    if carregar(this, 1) != 0 {
        return this;
    }
    crate::comparacao::ramo(ramo);
    *((this + 0x51) as *mut u8) = 0xf;
    *((this + 0x44) as *mut u8) = 0xff;
    core::ptr::write_unaligned((this + 0x30) as *mut i32, -1);
    core::ptr::write_unaligned((this + 0x2c) as *mut u32, e.fases);
    let v = alocar(e.fases * 4);
    core::ptr::write_unaligned((this + 0xc) as *mut usize, v);
    if regras(this) == 0 {
        aviso_em(e.arquivo, e.linhas[0]);
        return this;
    }
    if tabela(this) == 0 {
        aviso_em(e.arquivo, e.linhas[1]);
        return this;
    }
    if jogos(this) == 0 {
        aviso_em(e.arquivo, e.linhas[2]);
        return this;
    }
    if ordenar(this, 0) == 0 {
        aviso_em(e.arquivo, e.linhas[3]);
        return this;
    }
    let n = novo(0x5ce);
    let s = if n != 0 { estatisticas(n, this) } else { 0 };
    core::ptr::write_unaligned((this + 8) as *mut usize, s);
    if s == 0 {
        aviso_em(e.arquivo, e.linhas[4]);
        return this;
    }
    if previstas_sim {
        previstas(this);
    }
    this
}

/// O comparador dos três: alocam e sorteiam os jogos: só a original, conferindo o que ela grava: a devolução, a
/// vtable, o ano, o registro e a disciplina; na liga nova (+0x44 = 0xff, com as estatísticas), +0x2c, o vetor das
/// fases e a importância +0x51 = 0xf.
unsafe fn cqt2_comparar(nome: &str, this: usize, ano: u32, registro: usize, e: &Cqt2Estadual, original: impl FnOnce() -> usize) -> usize {
    let r = original();
    let mut ok = r == this && cqt2_d(this) == e.vtabela && core::ptr::read_unaligned((this + 0x40) as *const u16) == ano as u16
        && cqt2_d(this + 4) == registro && *((this + 0x50) as *const u8) == 7;
    if *((this + 0x44) as *const u8) == 0xff && cqt2_d(this + 8) != 0 {
        ok = ok && cqt2_d(this + 0x2c) == e.fases as usize && cqt2_d(this + 0xc) != 0 && *((this + 0x51) as *const u8) == 0xf;
    }
    PLACAR.registrar(ok, || format!("{}({:#x}): devolveu {:#x}, vtable {:#x}, +0x2c = {}, +0xc = {:#x}, +8 = {:#x}", nome, this, r, cqt2_d(this), cqt2_d(this + 0x2c), cqt2_d(this + 0xc), cqt2_d(this + 8)));
    r
}

const CQT2_GOIANO: Cqt2Estadual = Cqt2Estadual {
    vtabela: 0x9681d0,
    fases: 1,
    regras: 0x433640,
    tabela: 0x433540,
    previstas: 0x433b40,
    arquivo: 0x98ae90,
    linhas: [0x4c, 0x53, 0x59, 0x60, 0x67],
};

const CQT2_MINEIRO: Cqt2Estadual = Cqt2Estadual {
    vtabela: 0x968284,
    fases: 3,
    regras: 0x43a600,
    tabela: 0x434810,
    previstas: 0x434f80,
    arquivo: 0x98aed0,
    linhas: [0x4e, 0x55, 0x5b, 0x62, 0x69],
};

const CQT2_NORTE: Cqt2Estadual = Cqt2Estadual {
    vtabela: 0x968338,
    fases: 1,
    regras: 0x439430,
    tabela: 0x435d80,
    previstas: 0x436280,
    arquivo: 0x98af14,
    linhas: [0x4d, 0x54, 0x5b, 0x64, 0x6b],
};

/// { 0x432b50 } (thiscall, ret 8, bra_reg_goias.cpp): `BraRegGoias::BraRegGoias(ano, registro)`: o Goiano (uma fase além da liga).
unsafe extern "thiscall" fn cqt2_construir_goiano(this: usize, ano: u32, registro: usize) -> usize {
    cqt2_construir(this, ano, registro, &CQT2_GOIANO, 7, true, "cqt_2 goiano novo") // cqt2: a disciplina e as posições previstas do Goiano
}

/// { 0x433d50 } (thiscall, ret 8, bra_reg_minas_gerais.cpp): `BraRegMinasGerais::BraRegMinasGerais(ano, registro)`: o Mineiro (três fases além da liga).
unsafe extern "thiscall" fn cqt2_construir_mineiro(this: usize, ano: u32, registro: usize) -> usize {
    cqt2_construir(this, ano, registro, &CQT2_MINEIRO, 7, true, "cqt_2 mineiro novo") // cqt2: a disciplina e as posições previstas do Mineiro
}

/// { 0x435340 } (thiscall, ret 8, bra_reg_north.cpp): `BraRegNorth::BraRegNorth(ano, registro)`: o Norte (uma fase além da liga).
unsafe extern "thiscall" fn cqt2_construir_norte(this: usize, ano: u32, registro: usize) -> usize {
    cqt2_construir(this, ano, registro, &CQT2_NORTE, 7, true, "cqt_2 norte novo") // cqt2: a disciplina e as posições previstas do Norte
}

unsafe extern "thiscall" fn comparar_cqt2_construir_goiano(this: usize, ano: u32, registro: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(2049);
    cqt2_comparar("cqt2_construir_goiano", this, ano, registro, &CQT2_GOIANO, || o(this, ano, registro))
}

unsafe extern "thiscall" fn comparar_cqt2_construir_mineiro(this: usize, ano: u32, registro: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(2050);
    cqt2_comparar("cqt2_construir_mineiro", this, ano, registro, &CQT2_MINEIRO, || o(this, ano, registro))
}

unsafe extern "thiscall" fn comparar_cqt2_construir_norte(this: usize, ano: u32, registro: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(2051);
    cqt2_comparar("cqt2_construir_norte", this, ano, registro, &CQT2_NORTE, || o(this, ano, registro))
}

// ---------- 283a leva: os construtores dos estaduais: Nordeste, Paranaense e Pernambucano (as regras, a tabela, os jogos, as estatísticas e as posições previstas na criação) (bra_reg_northeast.cpp, bra_reg_parana.cpp, bra_reg_pern.cpp) ----------

unsafe fn cqt3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// Um estadual: a vtable, as fases além da liga (+0x2c: 1 nos de pontos corridos com mata-mata, 3 nos de dois turnos e
/// final; o vetor [+0xc] = malloc de 4 × fases), as regras (thiscall, ret 0), a tabela (thiscall, ret 0), as posições
/// previstas (thiscall, ret 0), o fonte e as linhas das caixas (regras, tabela, jogos, ordem, estatísticas).
struct Cqt3Estadual {
    vtabela: usize,
    fases: u32,
    regras: usize,
    tabela: usize,
    previstas: usize,
    arquivo: usize,
    linhas: [u32; 5],
}

/// O construtor de um estadual (o mesmo molde de 544 bytes nos doze, conferido instrução a instrução: só mudam a
/// vtable, as fases, as três rotinas próprias, o fonte e as linhas). League::League (0x682200, thiscall, ret 0); +4 = o
/// registro da competição; a vtable; o ano (+0x40, word) = `ano`; a regra de disciplina (+0x50) = 7. Se o estado
/// salvo carrega (League::load_from_temp_file 0x687b10, thiscall(this, 1), ret 4; devolve ≠ 0), devolve this. Senão
/// (a carreira nova): +0x51 = 0xf (a importância), +0x44 = 0xff, +0x30 = −1, +0x2c = as fases e o vetor delas (malloc
/// 0x944e46, sem conferir); então, cada um com a sua caixa (e devolvendo this) se devolver 0: as regras, a tabela, os
/// jogos (League::create_fixtures 0x6835c0, thiscall, ret 0) e a ordem (League::sort_table 0x6827d0, thiscall(this,
/// 0), ret 4); as estatísticas (operator new 0x944cf1 de 0x5ce e Comp_Stats::Comp_Stats 0x49ee70, thiscall(novo,
/// this), ret 4; o quadro SEH da original cobre só esse new) em +8 (sem elas: a caixa); por fim as posições previstas.
#[allow(clippy::too_many_arguments)]
unsafe fn cqt3_construir(this: usize, ano: u32, registro: usize, e: &Cqt3Estadual, disciplina: u8, previstas_sim: bool, ramo: &'static str) -> usize {
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x682200usize);
    let carregar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x687b10usize);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(e.regras);
    let tabela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(e.tabela);
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x6835c0usize);
    let ordenar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x6827d0usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let estatisticas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x49ee70usize);
    let previstas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(e.previstas);
    base(this);
    core::ptr::write_unaligned((this + 4) as *mut usize, registro);
    core::ptr::write_unaligned(this as *mut usize, e.vtabela);
    core::ptr::write_unaligned((this + 0x40) as *mut u16, ano as u16);
    *((this + 0x50) as *mut u8) = disciplina;
    if carregar(this, 1) != 0 {
        return this;
    }
    crate::comparacao::ramo(ramo);
    *((this + 0x51) as *mut u8) = 0xf;
    *((this + 0x44) as *mut u8) = 0xff;
    core::ptr::write_unaligned((this + 0x30) as *mut i32, -1);
    core::ptr::write_unaligned((this + 0x2c) as *mut u32, e.fases);
    let v = alocar(e.fases * 4);
    core::ptr::write_unaligned((this + 0xc) as *mut usize, v);
    if regras(this) == 0 {
        aviso_em(e.arquivo, e.linhas[0]);
        return this;
    }
    if tabela(this) == 0 {
        aviso_em(e.arquivo, e.linhas[1]);
        return this;
    }
    if jogos(this) == 0 {
        aviso_em(e.arquivo, e.linhas[2]);
        return this;
    }
    if ordenar(this, 0) == 0 {
        aviso_em(e.arquivo, e.linhas[3]);
        return this;
    }
    let n = novo(0x5ce);
    let s = if n != 0 { estatisticas(n, this) } else { 0 };
    core::ptr::write_unaligned((this + 8) as *mut usize, s);
    if s == 0 {
        aviso_em(e.arquivo, e.linhas[4]);
        return this;
    }
    if previstas_sim {
        previstas(this);
    }
    this
}

/// O comparador dos três: alocam e sorteiam os jogos: só a original, conferindo o que ela grava: a devolução, a
/// vtable, o ano, o registro e a disciplina; na liga nova (+0x44 = 0xff, com as estatísticas), +0x2c, o vetor das
/// fases e a importância +0x51 = 0xf.
unsafe fn cqt3_comparar(nome: &str, this: usize, ano: u32, registro: usize, e: &Cqt3Estadual, original: impl FnOnce() -> usize) -> usize {
    let r = original();
    let mut ok = r == this && cqt3_d(this) == e.vtabela && core::ptr::read_unaligned((this + 0x40) as *const u16) == ano as u16
        && cqt3_d(this + 4) == registro && *((this + 0x50) as *const u8) == 7;
    if *((this + 0x44) as *const u8) == 0xff && cqt3_d(this + 8) != 0 {
        ok = ok && cqt3_d(this + 0x2c) == e.fases as usize && cqt3_d(this + 0xc) != 0 && *((this + 0x51) as *const u8) == 0xf;
    }
    PLACAR.registrar(ok, || format!("{}({:#x}): devolveu {:#x}, vtable {:#x}, +0x2c = {}, +0xc = {:#x}, +8 = {:#x}", nome, this, r, cqt3_d(this), cqt3_d(this + 0x2c), cqt3_d(this + 0xc), cqt3_d(this + 8)));
    r
}

const CQT3_NORDESTE: Cqt3Estadual = Cqt3Estadual {
    vtabela: 0x9683ec,
    fases: 1,
    regras: 0x4322d0,
    tabela: 0x436d90,
    previstas: 0x437290,
    arquivo: 0x98af54,
    linhas: [0x4d, 0x54, 0x5b, 0x64, 0x6b],
};

const CQT3_PARANAENSE: Cqt3Estadual = Cqt3Estadual {
    vtabela: 0x9684a0,
    fases: 3,
    regras: 0x437fc0,
    tabela: 0x437ec0,
    previstas: 0x438590,
    arquivo: 0x98af98,
    linhas: [0x4e, 0x55, 0x5b, 0x62, 0x69],
};

const CQT3_PERNAMBUCANO: Cqt3Estadual = Cqt3Estadual {
    vtabela: 0x968554,
    fases: 1,
    regras: 0x439430,
    tabela: 0x439330,
    previstas: 0x4398d0,
    arquivo: 0x98afd8,
    linhas: [0x4d, 0x54, 0x5b, 0x64, 0x6b],
};

/// { 0x4363f0 } (thiscall, ret 8, bra_reg_northeast.cpp): `BraRegNortheast::BraRegNortheast(ano, registro)`: o Nordeste (uma fase além da liga).
unsafe extern "thiscall" fn cqt3_construir_nordeste(this: usize, ano: u32, registro: usize) -> usize {
    cqt3_construir(this, ano, registro, &CQT3_NORDESTE, 7, true, "cqt_3 nordeste novo") // cqt3: a disciplina e as posições previstas do Nordeste
}

/// { 0x437400 } (thiscall, ret 8, bra_reg_parana.cpp): `BraRegParana::BraRegParana(ano, registro)`: o Paranaense (três fases além da liga).
unsafe extern "thiscall" fn cqt3_construir_paranaense(this: usize, ano: u32, registro: usize) -> usize {
    cqt3_construir(this, ano, registro, &CQT3_PARANAENSE, 7, true, "cqt_3 paranaense novo") // cqt3: a disciplina e as posições previstas do Paranaense
}

/// { 0x4388f0 } (thiscall, ret 8, bra_reg_pern.cpp): `BraRegPern::BraRegPern(ano, registro)`: o Pernambucano (uma fase além da liga).
unsafe extern "thiscall" fn cqt3_construir_pernambucano(this: usize, ano: u32, registro: usize) -> usize {
    cqt3_construir(this, ano, registro, &CQT3_PERNAMBUCANO, 7, true, "cqt_3 pernambucano novo") // cqt3: a disciplina e as posições previstas do Pernambucano
}

unsafe extern "thiscall" fn comparar_cqt3_construir_nordeste(this: usize, ano: u32, registro: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(2052);
    cqt3_comparar("cqt3_construir_nordeste", this, ano, registro, &CQT3_NORDESTE, || o(this, ano, registro))
}

unsafe extern "thiscall" fn comparar_cqt3_construir_paranaense(this: usize, ano: u32, registro: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(2053);
    cqt3_comparar("cqt3_construir_paranaense", this, ano, registro, &CQT3_PARANAENSE, || o(this, ano, registro))
}

unsafe extern "thiscall" fn comparar_cqt3_construir_pernambucano(this: usize, ano: u32, registro: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(2054);
    cqt3_comparar("cqt3_construir_pernambucano", this, ano, registro, &CQT3_PERNAMBUCANO, || o(this, ano, registro))
}

// ---------- 283a leva: os construtores dos estaduais: Carioca, Catarinense e Paulista (as regras, a tabela, os jogos, as estatísticas e as posições previstas na criação) e a média de público dos clubes no fim da temporada (bra_reg_rio.cpp, bra_reg_santa.cpp, bra_reg_sp.cpp, game.cpp) ----------

unsafe fn cqt4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// Um estadual: a vtable, as fases além da liga (+0x2c: 1 nos de pontos corridos com mata-mata, 3 nos de dois turnos e
/// final; o vetor [+0xc] = malloc de 4 × fases), as regras (thiscall, ret 0), a tabela (thiscall, ret 0), as posições
/// previstas (thiscall, ret 0), o fonte e as linhas das caixas (regras, tabela, jogos, ordem, estatísticas).
struct Cqt4Estadual {
    vtabela: usize,
    fases: u32,
    regras: usize,
    tabela: usize,
    previstas: usize,
    arquivo: usize,
    linhas: [u32; 5],
}

/// O construtor de um estadual (o mesmo molde de 544 bytes nos doze, conferido instrução a instrução: só mudam a
/// vtable, as fases, as três rotinas próprias, o fonte e as linhas). League::League (0x682200, thiscall, ret 0); +4 = o
/// registro da competição; a vtable; o ano (+0x40, word) = `ano`; a regra de disciplina (+0x50) = 7. Se o estado
/// salvo carrega (League::load_from_temp_file 0x687b10, thiscall(this, 1), ret 4; devolve ≠ 0), devolve this. Senão
/// (a carreira nova): +0x51 = 0xf (a importância), +0x44 = 0xff, +0x30 = −1, +0x2c = as fases e o vetor delas (malloc
/// 0x944e46, sem conferir); então, cada um com a sua caixa (e devolvendo this) se devolver 0: as regras, a tabela, os
/// jogos (League::create_fixtures 0x6835c0, thiscall, ret 0) e a ordem (League::sort_table 0x6827d0, thiscall(this,
/// 0), ret 4); as estatísticas (operator new 0x944cf1 de 0x5ce e Comp_Stats::Comp_Stats 0x49ee70, thiscall(novo,
/// this), ret 4; o quadro SEH da original cobre só esse new) em +8 (sem elas: a caixa); por fim as posições previstas.
#[allow(clippy::too_many_arguments)]
unsafe fn cqt4_construir(this: usize, ano: u32, registro: usize, e: &Cqt4Estadual, disciplina: u8, previstas_sim: bool, ramo: &'static str) -> usize {
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x682200usize);
    let carregar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x687b10usize);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(e.regras);
    let tabela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(e.tabela);
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x6835c0usize);
    let ordenar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x6827d0usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let estatisticas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x49ee70usize);
    let previstas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(e.previstas);
    base(this);
    core::ptr::write_unaligned((this + 4) as *mut usize, registro);
    core::ptr::write_unaligned(this as *mut usize, e.vtabela);
    core::ptr::write_unaligned((this + 0x40) as *mut u16, ano as u16);
    *((this + 0x50) as *mut u8) = disciplina;
    if carregar(this, 1) != 0 {
        return this;
    }
    crate::comparacao::ramo(ramo);
    *((this + 0x51) as *mut u8) = 0xf;
    *((this + 0x44) as *mut u8) = 0xff;
    core::ptr::write_unaligned((this + 0x30) as *mut i32, -1);
    core::ptr::write_unaligned((this + 0x2c) as *mut u32, e.fases);
    let v = alocar(e.fases * 4);
    core::ptr::write_unaligned((this + 0xc) as *mut usize, v);
    if regras(this) == 0 {
        aviso_em(e.arquivo, e.linhas[0]);
        return this;
    }
    if tabela(this) == 0 {
        aviso_em(e.arquivo, e.linhas[1]);
        return this;
    }
    if jogos(this) == 0 {
        aviso_em(e.arquivo, e.linhas[2]);
        return this;
    }
    if ordenar(this, 0) == 0 {
        aviso_em(e.arquivo, e.linhas[3]);
        return this;
    }
    let n = novo(0x5ce);
    let s = if n != 0 { estatisticas(n, this) } else { 0 };
    core::ptr::write_unaligned((this + 8) as *mut usize, s);
    if s == 0 {
        aviso_em(e.arquivo, e.linhas[4]);
        return this;
    }
    if previstas_sim {
        previstas(this);
    }
    this
}

/// O comparador dos três: alocam e sorteiam os jogos: só a original, conferindo o que ela grava: a devolução, a
/// vtable, o ano, o registro e a disciplina; na liga nova (+0x44 = 0xff, com as estatísticas), +0x2c, o vetor das
/// fases e a importância +0x51 = 0xf.
unsafe fn cqt4_comparar(nome: &str, this: usize, ano: u32, registro: usize, e: &Cqt4Estadual, original: impl FnOnce() -> usize) -> usize {
    let r = original();
    let mut ok = r == this && cqt4_d(this) == e.vtabela && core::ptr::read_unaligned((this + 0x40) as *const u16) == ano as u16
        && cqt4_d(this + 4) == registro && *((this + 0x50) as *const u8) == 7;
    if *((this + 0x44) as *const u8) == 0xff && cqt4_d(this + 8) != 0 {
        ok = ok && cqt4_d(this + 0x2c) == e.fases as usize && cqt4_d(this + 0xc) != 0 && *((this + 0x51) as *const u8) == 0xf;
    }
    PLACAR.registrar(ok, || format!("{}({:#x}): devolveu {:#x}, vtable {:#x}, +0x2c = {}, +0xc = {:#x}, +8 = {:#x}", nome, this, r, cqt4_d(this), cqt4_d(this + 0x2c), cqt4_d(this + 0xc), cqt4_d(this + 8)));
    r
}

const CQT4_CARIOCA: Cqt4Estadual = Cqt4Estadual {
    vtabela: 0x968608,
    fases: 3,
    regras: 0x43a600,
    tabela: 0x43a500,
    previstas: 0x43add0,
    arquivo: 0x98b014,
    linhas: [0x4e, 0x55, 0x5b, 0x62, 0x69],
};

const CQT4_CATARINENSE: Cqt4Estadual = Cqt4Estadual {
    vtabela: 0x9686bc,
    fases: 1,
    regras: 0x43bc70,
    tabela: 0x43bb70,
    previstas: 0x43c110,
    arquivo: 0x98b050,
    linhas: [0x4d, 0x54, 0x5b, 0x64, 0x6b],
};

const CQT4_PAULISTA: Cqt4Estadual = Cqt4Estadual {
    vtabela: 0x968770,
    fases: 1,
    regras: 0x4322d0,
    tabela: 0x43cc40,
    previstas: 0x43d140,
    arquivo: 0x98b090,
    linhas: [0x4d, 0x54, 0x5b, 0x64, 0x6b],
};

/// { 0x439a40 } (thiscall, ret 8, bra_reg_rio.cpp): `BraRegRio::BraRegRio(ano, registro)`: o Carioca (três fases além da liga).
unsafe extern "thiscall" fn cqt4_construir_carioca(this: usize, ano: u32, registro: usize) -> usize {
    cqt4_construir(this, ano, registro, &CQT4_CARIOCA, 7, true, "cqt_4 carioca novo") // cqt4: a disciplina e as posições previstas do Carioca
}

/// { 0x43b130 } (thiscall, ret 8, bra_reg_santa.cpp): `BraRegSanta::BraRegSanta(ano, registro)`: o Catarinense (uma fase além da liga).
unsafe extern "thiscall" fn cqt4_construir_catarinense(this: usize, ano: u32, registro: usize) -> usize {
    cqt4_construir(this, ano, registro, &CQT4_CATARINENSE, 7, true, "cqt_4 catarinense novo") // cqt4: a disciplina e as posições previstas do Catarinense
}

/// { 0x43c280 } (thiscall, ret 8, bra_reg_sp.cpp): `BraRegSp::BraRegSp(ano, registro)`: o Paulista (uma fase além da liga).
unsafe extern "thiscall" fn cqt4_construir_paulista(this: usize, ano: u32, registro: usize) -> usize {
    cqt4_construir(this, ano, registro, &CQT4_PAULISTA, 7, true, "cqt_4 paulista novo") // cqt4: a disciplina e as posições previstas do Paulista
}

unsafe extern "thiscall" fn comparar_cqt4_construir_carioca(this: usize, ano: u32, registro: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(2055);
    cqt4_comparar("cqt4_construir_carioca", this, ano, registro, &CQT4_CARIOCA, || o(this, ano, registro))
}

unsafe extern "thiscall" fn comparar_cqt4_construir_catarinense(this: usize, ano: u32, registro: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(2056);
    cqt4_comparar("cqt4_construir_catarinense", this, ano, registro, &CQT4_CATARINENSE, || o(this, ano, registro))
}

unsafe extern "thiscall" fn comparar_cqt4_construir_paulista(this: usize, ano: u32, registro: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(2057);
    cqt4_comparar("cqt4_construir_paulista", this, ano, registro, &CQT4_PAULISTA, || o(this, ano, registro))
}

unsafe fn cqt4_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqt4_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqt4_pi(a: usize, v: i32) {
    core::ptr::write_unaligned(a as *mut i32, v)
}

/// A reputação da competição de uma liga (+0x69 do registro +4, word).
unsafe fn cqt4_rep(liga: usize) -> i32 {
    cqt4_w(cqt4_d(liga + 4) + 0x69) as i32
}

/// O piso que sobe com a reputação `r` (dos ramos que puxam o público para cima): com r < 5, 250 × r × f; senão
/// 750 × (r − 4) × f + 1000 (a original compara com a parte inteira + 1000 e grava a conta com 1000 somado antes do
/// truncamento). Abaixo dele, o público vai a ele e o recorde (+0x7b) sobe até ele.
unsafe fn cqt4_piso_pela_reputacao(c: usize, r: i32, f: f64) {
    let lim = if r < 5 {
        let lim = ftol((r.wrapping_mul(250)) as f64 * f);
        if cqt4_i(c + 0x73) >= lim {
            return;
        }
        lim
    } else {
        let x = (r.wrapping_sub(4).wrapping_mul(750)) as f64 * f;
        if cqt4_i(c + 0x73) >= ftol(x).wrapping_add(1000) {
            return;
        }
        ftol(x + 1000.0)
    };
    cqt4_pi(c + 0x73, lim);
    if lim > cqt4_i(c + 0x7b) {
        cqt4_pi(c + 0x7b, lim);
    }
}

/// O teto `lim` do público: acima dele, o público cai a ele e o mínimo (+0x77) desce até ele.
unsafe fn cqt4_teto(c: usize, lim: i32) {
    if cqt4_i(c + 0x73) > lim {
        cqt4_pi(c + 0x73, lim);
        if lim < cqt4_i(c + 0x77) {
            cqt4_pi(c + 0x77, lim);
        }
    }
}

/// { 0x5d6280 } (cdecl, game.cpp, sem nome no mapa): `atualizar_media_de_publico(liga)`, do laço das temporadas
/// (0x5d5ae0, em 0x5d5cfb: push liga; call; add esp, 4), no fim da temporada de cada liga da nação (a de cima e as de
/// baixo, pela +0x20). A média de público do clube (+0x73, Attendance) anda com o resultado da temporada, entre o
/// mínimo (+0x77) e o recorde (+0x7b).
/// O peso `f` vem da nação da competição (o primeiro dword de [+4]+0x5d): 0,1 em [0x9cf510], [0x9cf2dc], [0x9cf3e4],
/// [0x9cf224]; 0,25 em [0x9cf2c0], [0x9cf2fc], [0x9cf408], [0x9cf4a8], [0x9cf4f8]; 0,5 em [0x9cf45c]; 1 nas outras.
/// `r` = a reputação da competição (+0x69 do registro, word). A tabela final vem das estatísticas (Comp_Stats::
/// get_sorted_team_table 0x4a4b20, thiscall(+8, 1, &n, 1000), ret 0xc: registros de 0x76 bytes, +0 o id do clube,
/// +0x72 a posição prevista, +0x73 a final, +5 os jogos em casa [inferência], +6 o público somado [inferência]);
/// para cada clube (os ramos na ordem da original, com as mesmas chamadas às virtuais +0x98 "tem acesso" e +0x9c "tem
/// rebaixamento"; d = prevista − final + 1, word):
/// - campeão sem acesso: + (r + max(d, 0)) × 200 × f (dono do estádio, +0x6d) ou × 100 × f;
/// - 2º (ou posição 0) sem acesso: + (r + max(d, 0)) × 125 × f ou × 65 × f;
/// - subiu (posição ≤ +0xbe): pela liga de cima ([0xadadfc][+0x1c]; sem ela, nada): v = +0x80 do clube (a reputação,
///   word) × max(r de cima − r, 1) × f × 0,1, no mínimo 100; + v + sorteio(v) (dono) ou + v; e o piso pela reputação
///   da liga de cima;
/// - play-off de acesso (+0xbf ≠ 0 e posição ≤ +0xbf): com d − 1 > 0: + sorteio(r(d − 1) × 75 × f) + r(d − 1) × 75 × f
///   (dono) ou + sorteio(r(d − 1) × 25 × f) + r(d − 1) × 10 × f; senão + sorteio(r × 50 × f ou r × 25 × f); com
///   jogos em casa, a mistura com o público real: (público somado / jogos + 4 × média) × f × 0,2;
/// - caiu (posição > +0x3e − +0xc1): pela liga de baixo (sem ela, nada): v = +0x80 × min(r de baixo − r, −1) × f × 0,2,
///   no máximo −100 (sem rebaixamento na de baixo, +0x20 = −1: v = 2v − 250 − sorteio(750)); + v + sorteio(v); e o
///   teto de 2000 (r de baixo ≤ 7) ou 3000 vezes a r de baixo, × f;
/// - os outros: + sorteio(r × d × 5 × f), × 10 para o dono com d > 0 ou com +0x80 acima de 500 × r; a mistura com o
///   público real (sem o peso f); o piso pela reputação; e o teto de 2000 r (r ≤ 7), 3000 r (r ≤ 12) ou 5000 r.
/// No fim de cada um: no mínimo 500, e dentro de [+0x77, +0x7b]. A tabela é liberada (free 0x9452ca).
unsafe extern "C" fn cqt4_atualizar_media_de_publico(liga: usize) {
    let tabela_c = cqt4_d(0xadadfc);
    let cima = if cqt4_i(liga + 0x1c) == -1 { 0 } else { cqt4_d(tabela_c + 4 * cqt4_d(liga + 0x1c)) };
    let baixo = if cqt4_i(liga + 0x20) == -1 { 0 } else { cqt4_d(tabela_c + 4 * cqt4_d(liga + 0x20)) };
    let mut f = 1.0f64;
    let reg = cqt4_d(liga + 4);
    if reg != 0 && cqt4_d(reg + 0x5d) != 0 {
        let n = cqt4_d(cqt4_d(reg + 0x5d));
        let em = |gs: &[usize]| gs.iter().any(|&g| n == cqt4_d(g));
        if em(&[0x9cf510, 0x9cf2dc, 0x9cf3e4, 0x9cf224]) {
            f = f64::from_bits(0x3fb9_9999_9999_999a);
        } else if em(&[0x9cf2c0, 0x9cf2fc, 0x9cf408, 0x9cf4a8, 0x9cf4f8]) {
            f = 0.25;
        } else if n == cqt4_d(0x9cf45c) {
            f = 0.5;
        }
    }
    let ordenada: unsafe extern "thiscall" fn(usize, u32, *mut i32, u32) -> usize = core::mem::transmute(0x4a4b20usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let acesso = || {
        let v: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cqt4_d(cqt4_d(liga) + 0x98));
        v(liga)
    };
    let rebaixamento = || {
        let v: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cqt4_d(cqt4_d(liga) + 0x9c));
        v(liga)
    };
    let mut n = 0i32;
    let t = ordenada(cqt4_d(liga + 8), 1, &mut n, 1000);
    if t == 0 {
        return;
    }
    let mut i = 0i32;
    while i < n {
        let e = t + 0x76 * i as usize;
        let c = cqt4_d(0xae23b4) + 0x245 * cqt4_d(e);
        let pos = *((e + 0x73) as *const u8);
        let prevista = *((e + 0x72) as *const u8);
        let dono = *((c + 0x6d) as *const u8) != 0;
        let somar = |v: i32| cqt4_pi(c + 0x73, cqt4_i(c + 0x73).wrapping_add(v));
        let d = (prevista as i16).wrapping_sub(pos as i16).wrapping_add(1);
        if pos == 1 && acesso() == 0 {
            let k = cqt4_rep(liga) + d.max(0) as i32;
            let val = if dono { k.wrapping_mul(200) } else { k.wrapping_mul(100) };
            somar(ftol(val as f64 * f));
        } else if acesso() == 0 && pos < 3 {
            let k = cqt4_rep(liga) + d.max(0) as i32;
            let val = if dono { k.wrapping_mul(125) } else { k.wrapping_mul(65) };
            somar(ftol(val as f64 * f));
        } else if acesso() != 0 && pos as i32 <= *((liga + 0xbe) as *const i8) as i32 {
            if cima != 0 {
                let mut a = cqt4_w(cqt4_d(cima + 4) + 0x69).wrapping_sub(cqt4_w(cqt4_d(liga + 4) + 0x69));
                if a < 1 {
                    a = 1;
                }
                let v = ftol(cqt4_w(c + 0x80) as f64 * a as f64 * f * 0.1).max(100);
                if dono {
                    let r = sorteio(v);
                    somar(r.wrapping_add(v));
                } else {
                    somar(v);
                }
                cqt4_piso_pela_reputacao(c, cqt4_rep(cima), f);
                crate::comparacao::ramo("cqt_4 publico de quem subiu");
            }
        } else if *((liga + 0xbf) as *const i8) != 0 && pos as i32 <= *((liga + 0xbf) as *const i8) as i32 {
            let a = (prevista as i16).wrapping_sub(pos as i16);
            if a > 0 {
                let s = a as i32;
                if dono {
                    let v = ftol(cqt4_rep(liga).wrapping_mul(s).wrapping_mul(75) as f64 * f);
                    let r = sorteio(v);
                    let w = ftol(cqt4_rep(liga).wrapping_mul(s).wrapping_mul(75) as f64 * f);
                    somar(r.wrapping_add(w));
                } else {
                    let v = ftol(cqt4_rep(liga).wrapping_mul(s).wrapping_mul(25) as f64 * f);
                    let r = sorteio(v);
                    let w = ftol(cqt4_rep(liga).wrapping_mul(s).wrapping_mul(10) as f64 * f);
                    somar(r.wrapping_add(w));
                }
            } else {
                let r = cqt4_rep(liga);
                let val = if dono { r.wrapping_mul(50) } else { r.wrapping_mul(25) };
                let v = ftol(val as f64 * f);
                somar(sorteio(v));
            }
            let jogos = *((e + 5) as *const u8);
            if jogos != 0 {
                let val = (cqt4_i(e + 6) / jogos as i32).wrapping_add(cqt4_i(c + 0x73).wrapping_mul(4));
                cqt4_pi(c + 0x73, ftol(val as f64 * f * 0.2));
            }
        } else if rebaixamento() != 0
            && pos as i32 > cqt4_w(liga + 0x3e) as i32 - *((liga + 0xc1) as *const i8) as i32
        {
            if baixo != 0 {
                let mut a = cqt4_w(cqt4_d(baixo + 4) + 0x69).wrapping_sub(cqt4_w(cqt4_d(liga + 4) + 0x69));
                if a > -1 {
                    a = -1;
                }
                let mut v = ftol(cqt4_w(c + 0x80) as f64 * a as f64 * f * 0.2).min(-100);
                if cqt4_i(baixo + 0x20) == -1 {
                    let r = sorteio(0x2ee);
                    v = v.wrapping_mul(2).wrapping_sub(0xfa).wrapping_sub(r);
                }
                let r = sorteio(v);
                somar(r.wrapping_add(v));
                let rb = cqt4_rep(baixo);
                let lim = if rb > 7 { ftol(rb.wrapping_mul(3000) as f64 * f) } else { ftol(rb.wrapping_mul(2000) as f64 * f) };
                cqt4_teto(c, lim);
                crate::comparacao::ramo("cqt_4 publico de quem caiu");
            }
        } else {
            let r16 = cqt4_w(cqt4_d(liga + 4) + 0x69);
            let r = r16 as i32;
            let val = if d <= 0 && cqt4_w(c + 0x80) <= r16.wrapping_mul(500) {
                r.wrapping_mul(d as i32).wrapping_mul(5)
            } else if dono {
                r.wrapping_mul(d as i32).wrapping_mul(10)
            } else {
                r.wrapping_mul(d as i32).wrapping_mul(5)
            };
            let v = ftol(val as f64 * f);
            somar(sorteio(v));
            let jogos = *((e + 5) as *const u8);
            if jogos != 0 {
                let val = (cqt4_i(e + 6) / jogos as i32).wrapping_add(cqt4_i(c + 0x73).wrapping_mul(4));
                cqt4_pi(c + 0x73, ftol(val as f64 * 0.2)); // cqt4: a mistura com o público real
            }
            crate::comparacao::ramo("cqt_4 publico do meio da tabela");
            cqt4_piso_pela_reputacao(c, cqt4_rep(liga), f);
            let r = cqt4_rep(liga);
            let teto = if r > 12 { r.wrapping_mul(5000) } else if r > 7 { r.wrapping_mul(3000) } else { r.wrapping_mul(2000) };
            cqt4_teto(c, teto);
        }
        if cqt4_i(c + 0x73) < 500 {
            cqt4_pi(c + 0x73, 500);
        }
        let a = cqt4_i(c + 0x73);
        if a < cqt4_i(c + 0x77) {
            cqt4_pi(c + 0x73, cqt4_i(c + 0x77));
        } else if a > cqt4_i(c + 0x7b) {
            cqt4_pi(c + 0x73, cqt4_i(c + 0x7b));
        }
        i += 1;
    }
    soltar(t);
}

/// O comparador da média de público: lado a lado (a cópia sem contar os sorteios, com o estado do sorteador
/// restaurado entre as duas) sobre a tabela dos clubes inteira (onde ficam +0x73, +0x77 e +0x7b) e o estado do
/// sorteador; a tabela final (0x4a4b20) é alocada e liberada nas duas.
unsafe extern "C" fn comparar_cqt4_atualizar_media_de_publico(liga: usize) {
    let o = orig::<unsafe extern "C" fn(usize)>(2058);
    let mut regioes = vec![(cqt4_d(0xae23b4), cqt4_d(0xae235c) * 0x245)];
    regioes.extend_from_slice(&crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, _, _) = crate::comparacao::lado_a_lado(&regioes, || o(liga), || crate::aleatorio::sem_contar(|| cqt4_atualizar_media_de_publico(liga)));
    PLACAR.registrar(igual, || format!("cqt4_atualizar_media_de_publico({:#x}): o público dos clubes não bate", liga));
}

// ---------- 283a leva: o Campeonato de Clubes da Oceania: o dia do sorteio e as tabelas dos grupos A e B (oceania_club_champ.cpp) ----------

unsafe fn cqt10_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqt10_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqt10_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// O clube k dos 8 escolhidos (+0xee, ponteiros: o grupo A nas vagas 0..3, o B nas 4..7), a que as duas tabelas leem.
unsafe fn cqt10_escolhido(this: usize, k: usize) -> usize {
    cqt10_d(cqt10_d(this + 0xee) + k * 4) // cqt10: o escolhido k
}

/// { 0x797b40 } (thiscall, ret 8, oceania_club_champ.cpp; a virtual +0x14; quadro SEH só das datas locais, sem
/// destrutor): `OceaniaClubChamp::update_for_date(hoje, completo)`, o dia do Campeonato de Clubes da Oceania. A data do
/// sorteio é a de 0x549ef0(+0xb6, +0xb8 + +0x40) (lea ecx, local; push ano; push dia -> thiscall, ret 8, pura). Antes
/// dela (o ano de hoje menor, sem sinal; ou o mesmo ano e o dia de hoje menor, com sinal) a rotina volta 0 sem o dia de
/// liga. No dia do sorteio, fora do sorteio feito (+0x45 = 0) e SEM `completo`: os 8 clubes (0x798220: mov ecx ->
/// thiscall, ret 0), as posições previstas (a virtual +0x5c, thiscall, ret 0), a tabela do grupo A (0x7978c0, thiscall,
/// ret 0; 0: a caixa da linha 0x21d), os jogos (0x6835c0 League::create_fixtures, thiscall, ret 0; 0: a caixa da linha
/// 0x224), a tabela ordenada (0x6827d0 push 0; mov ecx -> thiscall, ret 4; 0: a caixa da linha 0x22b) (um erro volta 1,
/// sem o dia de liga), a fase do grupo B (0x797e80, thiscall, ret 0), a lista +0xee liberada (0x9452ca: push; add esp,
/// 4 -> cdecl) e zerada, +0xf2 = 0 e +0x45 = 1. Depois o dia de liga (0x6847c0: push completo; push hoje; mov ecx, this
/// -> thiscall, ret 8), cuja devolução ela repassa.
unsafe extern "thiscall" fn cqt10_oceania_dia(this: usize, hoje: usize, completo: u32) -> u32 {
    let clubes: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x798220usize);
    let grupo_a: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x7978c0usize);
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x6835c0usize);
    let ordenar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x6827d0usize);
    let grupo_b: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x797e80usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let liga: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x6847c0usize);
    let dia = cqt10_w(this + 0xb6) as u16;
    let ano = (cqt10_w(this + 0xb8) as u16).wrapping_add(cqt10_w(this + 0x40) as u16);
    let dia_de_hoje = core::ptr::read_unaligned(hoje as *const u16);
    let ano_de_hoje = core::ptr::read_unaligned((hoje + 2) as *const u16);
    // ponytail: nas voltas antes do sorteio e nos erros a original devolve só o al (0 ou 1; o resto do eax é sobra das
    // chamadas de antes); quem chama lê o byte
    if ano_de_hoje < ano || (ano_de_hoje == ano && (dia_de_hoje as i16) < dia as i16) {
        return 0;
    }
    if ano_de_hoje == ano && dia_de_hoje == dia && cqt10_d(this + 0x45) == 0 && completo == 0 {
        crate::comparacao::ramo("cqt_10 sorteio da oceania");
        clubes(this);
        let previstas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(cqt10_d(cqt10_d(this) + 0x5c));
        previstas(this);
        if grupo_a(this) == 0 {
            aviso_em(0xa6a068, 0x21d);
            return 1;
        }
        if jogos(this) == 0 {
            aviso_em(0xa6a068, 0x224);
            return 1;
        }
        if ordenar(this, 0) == 0 {
            aviso_em(0xa6a068, 0x22b);
            return 1;
        }
        grupo_b(this);
        soltar(cqt10_d(this + 0xee));
        cqt10_pd(this + 0xee, 0);
        cqt10_pd(this + 0xf2, 0);
        cqt10_pd(this + 0x45, 1);
    }
    liga(this, hoje, completo)
}

// ponytail: os clubes, as previsões, as tabelas, os jogos e o dia de liga não rodam duas vezes. Só a original; o plano
// no dia do sorteio (a condição da cópia): +0x45 = 1, a lista +0xee e +0xf2 zeradas, a tabela do grupo A e a fase do
// grupo B criadas e a fase atual (+0x30) = 0.
unsafe extern "thiscall" fn comparar_cqt10_oceania_dia(this: usize, hoje: usize, completo: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(2059);
    let sorteia = cqt10_d(this + 0x45) == 0
        && completo == 0
        && (cqt10_w(this + 0xb8) as u16).wrapping_add(cqt10_w(this + 0x40) as u16) == core::ptr::read_unaligned((hoje + 2) as *const u16)
        && cqt10_w(this + 0xb6) as u16 == core::ptr::read_unaligned(hoje as *const u16);
    let r = o(this, hoje, completo);
    if sorteia {
        let fases = cqt10_d(this + 0xc);
        let ok = cqt10_d(this + 0x45) == 1
            && cqt10_d(this + 0xee) == 0
            && cqt10_d(this + 0xf2) == 0
            && cqt10_d(this + 0xb1) != 0
            && fases != 0
            && cqt10_d(fases) != 0
            && cqt10_d(this + 0x30) == 0;
        PLACAR.registrar(ok, || format!("cqt10_oceania_dia({:#x}): o sorteio não montou os grupos (+0x45 {}, +0x30 {})", this, cqt10_d(this + 0x45), cqt10_d(this + 0x30) as i32));
    }
    r
}

/// { 0x7978c0 } (thiscall, ret 0, oceania_club_champ.cpp): `OceaniaClubChamp::create_group_a_table()`: 4 times (+0x3e), a
/// tabela +0xb1 (0x944e46(0xec): push 0xec; add esp, 4 -> cdecl; 4 linhas de 0x3b bytes; sem memória, a caixa da linha
/// 0x182 e devolve 0), cada linha i com o escolhido i (+0xee) (0x687430 League::init_table_row: push 0; push 0; push
/// clube; push i; mov ecx, this -> thiscall, ret 0x10), a notícia do sorteio do grupo (0x684230: mov ecx, this ->
/// thiscall, ret 0) e devolve 1. O mesmo corpo da 0x4019c0 da Copa Africana (só o arquivo e a linha da caixa mudam).
unsafe extern "thiscall" fn cqt10_oceania_grupo_a(this: usize) -> u32 {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let linha: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(0x687430usize);
    let noticia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x684230usize);
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 4);
    let t = novo(0xec);
    cqt10_pd(this + 0xb1, t);
    if t == 0 {
        aviso_em(0xa6a068, 0x182);
        return 0;
    }
    let mut i = 0i8;
    while (i as i16) < cqt10_w(this + 0x3e) {
        crate::comparacao::ramo("cqt_10 linha do grupo a");
        // ponytail: o índice vai num dword cujos 3 bytes altos são sobra de pilha; a 0x687430 lê só o byte (char)
        linha(this, i as u8 as u32, cqt10_escolhido(this, i as usize), 0, 0); // cqt10: a linha i do grupo A
        i = i.wrapping_add(1);
    }
    noticia(this);
    1
}

// ponytail: a tabela é alocada (e cada linha aloca o histórico) e a notícia vai aos técnicos: não roda duas vezes. Só a
// original; o plano: devolve 1, 4 times e na linha i (0x3b bytes) o escolhido i com a posição i (+4).
unsafe extern "thiscall" fn comparar_cqt10_oceania_grupo_a(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(2060);
    let lista = cqt10_d(this + 0xee);
    if lista == 0 {
        return o(this);
    }
    let previstos: [usize; 4] = core::array::from_fn(|i| cqt10_d(lista + i * 4));
    let r = o(this);
    let t = cqt10_d(this + 0xb1);
    let mut lidos = [(0usize, 0u8); 4];
    if t != 0 {
        for (i, l) in lidos.iter_mut().enumerate() {
            *l = (cqt10_d(t + i * 0x3b), *((t + i * 0x3b + 4) as *const u8));
        }
    }
    let igual = r == 1 && cqt10_w(this + 0x3e) == 4 && t != 0 && (0..4).all(|i| lidos[i] == (previstos[i], i as u8));
    PLACAR.registrar(igual, || format!("cqt10_oceania_grupo_a({:#x}) devolveu {} linhas {:x?} previstas {:x?}", this, r, lidos, previstos));
    r
}

/// { 0x797e80 } (thiscall, ret 0, oceania_club_champ.cpp, quadro SEH só para o new da fase: sem destrutor no caminho
/// normal): `OceaniaClubChamp::create_group_b_stage()`: os times num vetor de 4 ponteiros (0x944e46(0x10): push; add esp,
/// 4 -> cdecl; sem memória, a caixa da linha 0x296 e 0): os escolhidos 4 a 7 (+0xee); a tabela das rodadas pela virtual
/// +0x3c (push 0; push &id; push &rodadas; push 0; mov ecx, this -> thiscall, ret 0x10); a fase (0x944cf1(0xee) push; add
/// esp, 4 -> cdecl; o construtor League_Stage 0x68b100, thiscall, ret 0x58, 22 argumentos: a copa, 4, o vetor, 1, +4, a
/// tabela das rodadas, as rodadas (word), +0xc2, +0xc3, +0xc4, &+0xc5, &+0xbe, o ano +0x40 (word), 0, o id (word),
/// +0x51, 2, 0, +0xd9 (word), −1, 0, 2; a original empurra os words e bytes com lixo nos bytes altos, que o construtor
/// não lê: a cópia passa os valores limpos) em [+0xc][0]; o vetor e a tabela das rodadas liberados (0x9452ca). Sem fase:
/// a caixa da linha 0x2ad e 0. Com ela: +0x30 = 0, a notícia do grupo (0x684230 na fase) e 1.
unsafe extern "thiscall" fn cqt10_oceania_grupo_b(this: usize) -> u32 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let noticia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x684230usize);
    #[allow(clippy::type_complexity)]
    let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(0x68b100usize);
    let times = alocar(0x10);
    if times == 0 {
        aviso_em(0xa6a068, 0x296);
        return 0;
    }
    for n in 0..4usize {
        crate::comparacao::ramo("cqt_10 time do grupo b");
        cqt10_pd(times + n * 4, cqt10_escolhido(this, 4 + n)); // cqt10: o time na vaga do grupo B
    }
    let rodadas_da_copa: unsafe extern "thiscall" fn(usize, i32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(cqt10_d(cqt10_d(this) + 0x3c));
    let (mut rodadas, mut id) = (0i16, 0i16);
    let tabela = rodadas_da_copa(this, 0, &mut rodadas, &mut id, 0);
    let p = novo(0xee);
    let fase = if p != 0 {
        let b = |d: usize| *((this + d) as *const u8) as u32;
        construir(
            p,
            this,
            4,
            times,
            1,
            cqt10_d(this + 4),
            tabela,
            rodadas as u16 as u32,
            b(0xc2),
            b(0xc3),
            b(0xc4),
            this + 0xc5,
            this + 0xbe,
            cqt10_w(this + 0x40) as u16 as u32,
            0,
            id as u16 as u32,
            b(0x51),
            2,
            0,
            cqt10_w(this + 0xd9) as u16 as u32,
            -1,
            0,
            2,
        )
    } else {
        0
    };
    cqt10_pd(cqt10_d(this + 0xc), fase);
    soltar(times);
    soltar(tabela);
    let f = cqt10_d(cqt10_d(this + 0xc));
    if f == 0 {
        aviso_em(0xa6a068, 0x2ad);
        return 0;
    }
    cqt10_pd(this + 0x30, 0);
    noticia(f);
    1
}

// ponytail: a fase é alocada e a notícia vai aos técnicos: não roda duas vezes. Só a original; o plano: devolve 1 com a
// fase em [+0xc][0], +0x30 = 0, a fase com 4 times e o ano da copa e, na tabela dela (+0xb1, linhas de 0x3b bytes), os
// escolhidos 4 a 7 (em qualquer ordem: o construtor ordena a tabela).
unsafe extern "thiscall" fn comparar_cqt10_oceania_grupo_b(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(2061);
    let lista = cqt10_d(this + 0xee);
    if lista == 0 {
        return o(this);
    }
    let mut plano: Vec<usize> = (4..8).map(|k| cqt10_d(lista + k * 4)).collect();
    let r = o(this);
    if r == 1 {
        let f = cqt10_d(cqt10_d(this + 0xc));
        let t = if f != 0 { cqt10_d(f + 0xb1) } else { 0 };
        let mut a: Vec<usize> = if t != 0 { (0..4).map(|k| cqt10_d(t + k * 0x3b)).collect() } else { Vec::new() };
        a.sort_unstable();
        plano.sort_unstable();
        let ok = f != 0 && cqt10_d(this + 0x30) == 0 && cqt10_w(f + 0x3e) == 4 && cqt10_w(f + 0x40) == cqt10_w(this + 0x40) && a == plano;
        PLACAR.registrar(ok, || format!("cqt10_oceania_grupo_b: a fase não bate com o plano ({:x?} lidos {:x?})", plano, a));
    }
    r
}

// ---------- 283a leva: o Campeonato de Clubes da Oceania: os 8 clubes, a fase seguinte e as semifinais (oceania_club_champ.cpp) ----------

unsafe fn cqt11_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqt11_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqt11_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// Os clubes de uma origem para a Oceania: até 20, na ordem da tabela de clubes.
struct Cqt11Lista {
    clubes: [usize; 20],
    n: i16,
}

impl Cqt11Lista {
    fn novo() -> Self {
        Cqt11Lista { clubes: [0; 20], n: 0 }
    }

    /// Entra se cabe (a conta é palavra com sinal, < 20).
    fn cabe(&self) -> bool {
        self.n < 0x14
    }

    fn por(&mut self, c: usize) {
        self.clubes[self.n as usize] = c;
        self.n += 1;
    }

    /// Ordena pela reputação e embaralha a faixa de cima (0x4afbd0 comp_util_sort_and_shuffle_top_clubs: push quantos;
    /// push a conta; push lista; add esp -> cdecl; sorteia).
    unsafe fn ordenar(&mut self, quantos: u32) {
        let f: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(0x4afbd0usize);
        f(self.clubes.as_mut_ptr() as usize, self.n as u16 as u32, quantos);
    }

    /// O primeiro da lista e o primeiro depois dele de outra nação (+0x53), como a original: percorre as `n` vagas; vaga
    /// vazia não conta como primeiro.
    unsafe fn dois_de_nacoes_diferentes(&self) -> (usize, usize) {
        let (mut a, mut b) = (0usize, 0usize);
        for &c in self.clubes.iter().take(self.n.max(0) as usize) {
            if a == 0 {
                a = c;
            } else if b == 0 && cqt11_d(c + 0x53) != cqt11_d(a + 0x53) {
                b = c;
            }
        }
        (a, b)
    }
}

/// O par de um pote vai para os grupos A e B: com o sorteio de 2 (0x90cfc0(2): push 2; add esp -> cdecl) diferente de
/// zero, o primeiro na vaga `k` (grupo A) e o segundo na `k` + 4 (grupo B); com zero, ao contrário.
unsafe fn cqt11_par(this: usize, k: usize, primeiro: usize, segundo: usize, sorteio: i32) {
    let l = cqt11_d(this + 0xee);
    if sorteio != 0 {
        cqt11_pd(l + k * 4, primeiro);
        cqt11_pd(cqt11_d(this + 0xee) + (k + 4) * 4, segundo);
    } else {
        cqt11_pd(l + (k + 4) * 4, primeiro);
        cqt11_pd(cqt11_d(this + 0xee) + k * 4, segundo);
    }
}

/// { 0x798220 } (thiscall, ret 0, oceania_club_champ.cpp): `OceaniaClubChamp::select_qualified_clubs()`: +0xf2 = 8 e a
/// lista +0xee (0x944e46(0x20): push; add esp, 4 -> cdecl; sem memória, a caixa da linha 0x307 e devolve 0). O campeão
/// do último registro da liga australiana (0x4afd80 comp_util_get_latest_placed_club: push 0; push competição
/// 0xae23d0 + 0x6b × [0x9cf89c]; add esp, 8 -> cdecl). Os clubes ([0xae23b4], 0x245 bytes, [0xae235c] − 2 × [0xae2350]
/// deles) de nação da Oceania ([0x9cfa18]: +0x71 da nação, o +0 dele) vão, pelo id da nação, para as listas da Nova
/// Zelândia ([0x9cf3f4]), da Austrália ([0x9cf224]), do Taiti ([0x9cf4b4]) e de Fiji ([0x9cf2f8]), até 20 em cada (a
/// conta é palavra com sinal); os outros (ou com a lista cheia) para a melanésia (0x53d900 database_is_melanesian_nation:
/// push nação; add esp -> cdecl; cheia, o clube fica de fora) ou, senão, a polinésia (0x53d940 com a nação de novo), até
/// 20. Sem campeão, o melhor
/// australiano (a lista com a faixa de 1 sorteada) no lugar. Os pares, cada um sorteado entre os grupos A (vagas 0..3)
/// e B (4..7) por 0x90cfc0(2): o campeão e o melhor neozelandês (vagas 0/4); o melhor de Fiji e o do Taiti (1/5); os dois
/// primeiros melanésios de nações diferentes (2/6, a faixa de 2); os dois polinésios (3/7). Devolve 1.
/// A original guarda as listas na pilha sem zerá-las; com menos clubes que o lido ela lê sobra; a cópia lê zero
/// [inferência: nesta carreira as listas têm clubes que bastam; o comparador confere a lista].
unsafe fn cqt11_clubes(this: usize) -> u32 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let colocado: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x4afd80usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let melanesia: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53d900usize);
    let polinesia: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53d940usize);
    cqt11_pd(this + 0xf2, 8);
    let lista = alocar(0x20);
    cqt11_pd(this + 0xee, lista);
    if lista == 0 {
        aviso_em(0xa6a068, 0x307);
        return 0;
    }
    let mut campeao = colocado(cqt11_d(0xae23d0) + cqt11_d(0x9cf89c) * 0x6b, 0);
    let (mut nz, mut aus, mut tai, mut fij, mut mel, mut pol) =
        (Cqt11Lista::novo(), Cqt11Lista::novo(), Cqt11Lista::novo(), Cqt11Lista::novo(), Cqt11Lista::novo(), Cqt11Lista::novo());
    let mut i = 0i32;
    let mut c = cqt11_d(0xae23b4);
    while i < (cqt11_d(0xae235c) as i32).wrapping_sub((cqt11_d(0xae2350) as i32).wrapping_mul(2)) {
        let nacao = cqt11_d(c + 0x53);
        if nacao != 0 && cqt11_d(nacao + 0x71) != 0 && cqt11_d(cqt11_d(nacao + 0x71)) == cqt11_d(0x9cfa18) {
            let id = cqt11_d(nacao);
            if id == cqt11_d(0x9cf3f4) && nz.cabe() {
                nz.por(c);
            } else if id == cqt11_d(0x9cf224) && aus.cabe() {
                aus.por(c);
            } else if id == cqt11_d(0x9cf4b4) && tai.cabe() {
                tai.por(c);
            } else if id == cqt11_d(0x9cf2f8) && fij.cabe() {
                fij.por(c);
            } else if melanesia(nacao) != 0 {
                if mel.cabe() {
                    mel.por(c);
                }
            } else if polinesia(cqt11_d(c + 0x53)) != 0 && pol.cabe() {
                pol.por(c);
            }
        }
        i += 1;
        c += 0x245;
    }
    crate::comparacao::ramo("cqt_11 clubes da oceania");
    if campeao == 0 {
        aus.ordenar(1);
        campeao = aus.clubes[0];
    }
    nz.ordenar(1);
    cqt11_par(this, 0, campeao, nz.clubes[0], sorteio(2)); // cqt11: o campeão australiano e o neozelandês
    fij.ordenar(1);
    tai.ordenar(1);
    cqt11_par(this, 1, fij.clubes[0], tai.clubes[0], sorteio(2)); // cqt11: Fiji e Taiti
    mel.ordenar(2);
    let (a, b) = mel.dois_de_nacoes_diferentes();
    cqt11_par(this, 2, a, b, sorteio(2)); // cqt11: os dois melanésios
    pol.ordenar(2);
    let (a, b) = pol.dois_de_nacoes_diferentes();
    cqt11_par(this, 3, a, b, sorteio(2));
    1
}

unsafe extern "thiscall" fn cqt11_oceania_clubes(this: usize) -> u32 {
    cqt11_clubes(this)
}

/// Lado a lado: o +0xf2, o [0xb67a34] e o estado do sorteador a partir do mesmo estado (a cópia sorteia sem contar); a
/// original deixa a lista dela em +0xee, a cópia faz a sua, comparada nas 8 vagas e liberada.
unsafe extern "thiscall" fn comparar_cqt11_oceania_clubes(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(2062);
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let a_ = crate::aleatorio::REGIOES_DO_ESTADO;
    let reg = [(this + 0xf2, 4usize), (0xb67a34, 4), a_[0], a_[1]];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || {
        let r = o(this);
        lo.set(cqt11_d(this + 0xee));
        r
    }, || {
        let r = crate::aleatorio::sem_contar(|| cqt11_clubes(this));
        lr.set(cqt11_d(this + 0xee));
        cqt11_pd(this + 0xee, lo.get());
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let lista = if lo != 0 && lr != 0 {
        core::slice::from_raw_parts(lo as *const u8, 0x20) == core::slice::from_raw_parts(lr as *const u8, 0x20)
    } else {
        lo == 0 && lr == 0
    };
    if lr != 0 && lr != lo {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(lr);
    }
    PLACAR.registrar(igual && a == b && lista, || format!("cqt11_oceania_clubes({:#x}) devolveu {} x {} memória {} lista {}", this, a, b, igual, lista));
    a
}

/// { 0x797df0 } (thiscall, ret 0, oceania_club_champ.cpp; a virtual +0x28): `OceaniaClubChamp::advance_stage()`: na
/// última fase (+0x30 ≥ +0x2c − 1, com sinal), a caixa da linha 0x274; senão +0x30 sobe e, chegando a 1 (depois dos
/// grupos), as semifinais (0x798050: mov ecx, this -> thiscall, ret 0).
unsafe extern "thiscall" fn cqt11_oceania_fase_seguinte(this: usize) {
    let f = cqt11_d(this + 0x30) as i32;
    if f >= (cqt11_d(this + 0x2c) as i32).wrapping_sub(1) {
        return aviso_em(0xa6a068, 0x274);
    }
    let f = f + 1;
    cqt11_pd(this + 0x30, f as usize);
    if f == 1 {
        crate::comparacao::ramo("cqt_11 semifinais da oceania");
        let semifinais: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x798050usize);
        semifinais(this);
    }
}

// ponytail: as semifinais alocam e sorteiam: não rodam duas vezes. Só a original; o plano: +0x30 sobe um (fora da
// última fase, que dá a caixa e não muda nada).
unsafe extern "thiscall" fn comparar_cqt11_oceania_fase_seguinte(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(2063);
    let f = cqt11_d(this + 0x30) as i32;
    let ultima = f >= (cqt11_d(this + 0x2c) as i32).wrapping_sub(1);
    o(this);
    let depois = cqt11_d(this + 0x30) as i32;
    PLACAR.registrar(depois == if ultima { f } else { f + 1 }, || format!("cqt11_oceania_fase_seguinte({:#x}) fase {} -> {}", this, f, depois));
}

/// Os 4 das semifinais, na ordem da lista: o 1º do grupo A (a tabela da copa, +0xb1), o 2º do B (a tabela da fase
/// [+0xc][0]), o 2º do A e o 1º do B (os cruzamentos: as chaves são os pares da lista).
unsafe fn cqt11_semifinalistas(this: usize) -> [usize; 4] {
    let a = |l: usize| cqt11_d(cqt11_d(this + 0xb1) + l * 0x3b);
    let b = |l: usize| cqt11_d(cqt11_d(cqt11_d(cqt11_d(this + 0xc)) + 0xb1) + l * 0x3b);
    [a(0), b(1), a(1), b(0)] // cqt11: 1º A × 2º B, 2º A × 1º B
}

/// { 0x798050 } (thiscall, ret 0, oceania_club_champ.cpp, quadro SEH só para o new da fase: sem destrutor no caminho
/// normal): `OceaniaClubChamp::create_knockout_stage()`, as semifinais: a lista dos 4 (0x944e46(0x10): push; add esp, 4
/// -> cdecl; sem memória, a caixa da linha 0x2cb e volta), os cruzamentos de cqt11_semifinalistas; a tabela das rodadas
/// pela virtual +0x3c (push 0; push &id; push &rodadas; push 1; mov ecx, this -> thiscall, ret 0x10); a fase (0x944cf1(0xb2)
/// push; add esp, 4 -> cdecl; Cup_Stage::Cup_Stage 0x522e00, thiscall, ret 0x3c: a copa, 4, a lista, as rodadas, +4, a
/// tabela, o ano +0x40, 1, 2, o id, 0x14, 0, 0, 0 e nenhuma semente; a original empurra as rodadas, o id e o ano com lixo
/// nos 16 bits altos, que o construtor não lê) em [+0xc][1]; a tabela e a lista liberadas (0x9452ca). Sem fase: a caixa
/// da linha 0x2e3 e volta. Com ela: a rodada montada (0x51c800 Cup::set_up_round: push 0; mov ecx, fase -> thiscall,
/// ret 4).
unsafe extern "thiscall" fn cqt11_oceania_semifinais(this: usize) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let montar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x51c800usize);
    let lista = alocar(0x10);
    if lista == 0 {
        return aviso_em(0xa6a068, 0x2cb);
    }
    crate::comparacao::ramo("cqt_11 cruzamentos da oceania");
    for (k, &c) in cqt11_semifinalistas(this).iter().enumerate() {
        cqt11_pd(lista + k * 4, c);
    }
    let rodadas_da_copa: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(cqt11_d(cqt11_d(this) + 0x3c));
    let (mut rodadas, mut id) = (0i16, 0i16);
    let tabela = rodadas_da_copa(this, 1, &mut rodadas, &mut id, 0);
    let p = novo(0xb2);
    let fase = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(0x522e00usize);
        construir(p, this, 4, lista, rodadas as u16 as u32, cqt11_d(this + 4), tabela, cqt11_w(this + 0x40) as u16 as u32, 1, 2, id as u16 as u32, 0x14, 0, 0, 0, core::ptr::null())
    } else {
        0
    };
    cqt11_pd(cqt11_d(this + 0xc) + 4, fase);
    soltar(tabela);
    soltar(lista);
    let f = cqt11_d(cqt11_d(this + 0xc) + 4);
    if f == 0 {
        return aviso_em(0xa6a068, 0x2e3);
    }
    montar(f, 0);
}

// ponytail: aloca a fase e monta a rodada: não roda duas vezes. Só a original; o plano: a fase em [+0xc][1] com 4 times,
// o +4 da copa e os 4 cruzados (lidos antes) na lista dela (+0xa7, 6 bytes; como conjunto: a rodada pode reordenar).
unsafe extern "thiscall" fn comparar_cqt11_oceania_semifinais(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(2064);
    if cqt11_d(this + 0xb1) == 0 || cqt11_d(this + 0xc) == 0 || cqt11_d(cqt11_d(this + 0xc)) == 0 {
        return o(this);
    }
    let mut plano = cqt11_semifinalistas(this).to_vec();
    o(this);
    let fase = cqt11_d(cqt11_d(this + 0xc) + 4);
    if fase == 0 {
        PLACAR.registrar(false, || format!("cqt11_oceania_semifinais({:#x}) sem fase", this));
        return;
    }
    let lista = cqt11_d(fase + 0xa7);
    let mut achou: Vec<usize> = (0..4).map(|k| cqt11_d(lista + 6 * k)).collect();
    let (pl, ac) = (plano.clone(), achou.clone());
    plano.sort_unstable();
    achou.sort_unstable();
    let igual = plano == achou && cqt11_w(fase + 0x3e) == 4 && cqt11_d(fase + 4) == cqt11_d(this + 4);
    PLACAR.registrar(igual, || format!("cqt11_oceania_semifinais({:#x}) plano={:x?} lista={:x?}", this, pl, ac));
}
