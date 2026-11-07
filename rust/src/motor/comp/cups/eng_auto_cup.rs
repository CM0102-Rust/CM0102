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

