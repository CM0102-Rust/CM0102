// ---------- %%LEVA%%a leva: o Troféu da Liga inglesa (LDV Vans Trophy): os 56 clubes das duas chaves (Norte e Sul), a fase das chaves e a fase seguinte (eng_auto_cup.cpp) ----------

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
unsafe fn eng_auto_cup_is_in_comp(c: usize, g: usize) -> bool {
    let l = cqs5_d(c + 0x57);
    l != 0 && cqs5_d(l) == cqs5_d(g)
}

/// Põe o clube `c` na vaga `k` da lista (+0xb2, relida a cada vez) e conta.
unsafe fn push_section_club(this: usize, k: &mut i32, c: usize) {
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
unsafe fn eng_auto_cup_select_section_clubs(this: usize) -> u32 {
    cqs5_pd(this + 0xb6, 0x38);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let l = alocar(0xe0);
    cqs5_pd(this + 0xb2, l);
    if l == 0 {
        show_error_at(0x9c4180, 0x24c);
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
            if eng_auto_cup_is_in_comp(c, 0x9cf5c4) || eng_auto_cup_is_in_comp(c, 0x9cf5c8) {
                push_section_club(this, &mut k, c);
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
            if eng_auto_cup_is_in_comp(c, 0x9cf69c) && antes != 0 && cqs5_d(antes) == cqs5_d(0x9cf5c8) {
                crate::comparison::branch("cqs_5 relegated to the conference");
                push_section_club(this, &mut k, c); // cqs5: o rebaixado entra
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
                        if antes != 0 && cqs5_d(antes) == cqs5_d(0x9cf69c) && eng_auto_cup_is_in_comp(c, 0x9cf69c) {
                            let v = *((c + 0x5f) as *const i8);
                            if ((cat == 6 && v <= cat) || v == cat) && c != 0 && c != achado {
                                push_section_club(this, &mut k, c);
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
                if eng_auto_cup_is_in_comp(c, 0x9cf69c) && achado != c {
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
                    if nac != 0 && cqs5_d(nac) == cqs5_d(0x9cf2e4) && !eng_auto_cup_is_in_comp(c, 0x9cf764) {
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
            push_section_club(this, &mut k, local[a as u16 as usize]);
            a = a.wrapping_add(1);
        }
    }
    if k != cqs5_d(this + 0xb6) as i32 {
        return 0;
    }
    crate::comparison::branch("cqs_5 league trophy sections");
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    ordenar(cqs5_d(this + 0xb2), cqs5_d(this + 0xb6) as u32, 4, 0x53fd40); // cqs5: Norte e Sul pela latitude
    let metade = (cqs5_d(this + 0xb6) as i32) / 2;
    ordenar(cqs5_d(this + 0xb2), metade as u32, 4, 0x4b0810);
    let metade = (cqs5_d(this + 0xb6) as i32) / 2;
    ordenar(cqs5_d(this + 0xb2) + metade as usize * 4, metade as u32, 4, 0x4b0810);
    1
}

unsafe extern "thiscall" fn select_section_clubs(this: usize) -> u32 {
    eng_auto_cup_select_section_clubs(this)
}

/// Lado a lado a partir do mesmo estado (o sorteador, +0xb6, [0xb67a34]); a original deixa a lista dela em +0xb2, a cópia
/// faz a sua (sem contar os sorteios), comparada nas +0xb6 vagas e liberada. Quando a original não acha o rebaixado (o
/// caso em que ela lê a dword sem valor da pilha), só a original.
unsafe extern "thiscall" fn compare_select_section_clubs(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    let base = cqs5_d(0xae23b4);
    let tem_rebaixado = (0..cqs5_d(0xae235c) as i32).any(|i| {
        let c = base + i as usize * 0x245;
        let antes = cqs5_d(c + 0x5b);
        eng_auto_cup_is_in_comp(c, 0x9cf69c) && antes != 0 && cqs5_d(antes) == cqs5_d(0x9cf5c8)
    });
    if !tem_rebaixado {
        return o(this);
    }
    let antes = cqs5_d(this + 0xb2);
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.extend([(this + 0xb6, 4usize), (0xb67a34, 4)]);
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let (a, b, igual, _, _) = crate::comparison::side_by_side(&reg, || {
        let r = o(this);
        lo.set(cqs5_d(this + 0xb2));
        cqs5_pd(this + 0xb2, antes);
        r
    }, || {
        let r = crate::rng::without_counting(|| eng_auto_cup_select_section_clubs(this));
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
    SCOREBOARD.record(igual && a == b && lista, || format!("select_section_clubs({:#x}) devolveu {} x {} memória {} lista {}", this, a, b, igual, lista));
    a
}

/// { 0x56bab0 } (thiscall, ret 0, eng_auto_cup.cpp; quadro SEH só para o new, sem destrutor): `EngAutoCup::
/// create_section_stage()`: a fase das chaves (Norte e Sul) com os +0x3e clubes da lista +0xb2, na ordem: um vetor
/// (0x944e46 de 4 × +0x3e; sem memória, a caixa da linha 0x2e9) com eles; as rodadas pelo método +0x3c (push 0; push
/// &extra; push &rodadas; push 0; mov ecx -> thiscall, ret 0x10); a fase (0x944cf1(0xb2); Cup_Stage::Cup_Stage 0x522e00,
/// thiscall, ret 0x3c: a copa, +0x3e (word), o vetor, as rodadas (word), +4, a tabela, o ano +0x40 (word), 0, 1, o extra
/// (word), +0x51 (byte), 0, 0, 0, sem sementes; os words e bytes vão com lixo em cima, que o construtor não lê) em
/// [+0xc][0]; o vetor e a tabela liberados (0x9452ca). Sem fase, a caixa da linha 0x300; com ela, +0x30 = 0.
unsafe extern "thiscall" fn create_section_stage(this: usize) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let times = alocar(((cqs5_w(this + 0x3e) as i32) << 2) as u32);
    if times == 0 {
        return show_error_at(0x9c4180, 0x2e9);
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
        return show_error_at(0x9c4180, 0x300);
    }
    crate::comparison::branch("cqs_5 trophy sections stage");
    cqs5_pd(this + 0x30, 0);
}

/// Só a original (aloca a fase), conferindo o plano: a fase em [+0xc][0] e +0x30 = 0.
unsafe extern "thiscall" fn compare_create_section_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    o(this);
    let f = cqs5_d(cqs5_d(this + 0xc));
    SCOREBOARD.record(f != 0 && cqs5_d(this + 0x30) == 0, || format!("create_section_stage({:#x}) fase {:#x} +0x30 {}", this, f, cqs5_d(this + 0x30) as i32));
}

/// { 0x56bc50 } (thiscall, ret 0, eng_auto_cup.cpp; a virtual +0x28): `EngAutoCup::advance_stage()`: na última fase
/// (+0x30 ≥ +0x2c − 1, com sinal), a caixa da linha 0x314 e nada mais; senão +0x30 sobe um e, ao chegar a 1 (o fim das
/// chaves), monta a final (EngAutoCup::create_final_stage 0x56bce0, call direto com o ecx da copa -> thiscall, ret 0).
unsafe extern "thiscall" fn eng_auto_cup_advance_stage(this: usize) {
    let fase = cqs5_d(this + 0x30) as i32;
    if fase >= (cqs5_d(this + 0x2c) as i32).wrapping_sub(1) {
        return show_error_at(0x9c4180, 0x314);
    }
    let fase = fase + 1;
    cqs5_pd(this + 0x30, fase as usize);
    if fase == 1 {
        crate::comparison::branch("cqs_5 trophy final");
        let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x56bce0usize);
        f(this);
    }
}

/// Só a original (a final aloca e sorteia), conferindo o plano: +0x30 sobe um fora da última fase e fica na última.
unsafe extern "thiscall" fn compare_eng_auto_cup_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    let antes = cqs5_d(this + 0x30) as i32;
    let ultima = antes >= (cqs5_d(this + 0x2c) as i32).wrapping_sub(1);
    o(this);
    let depois = cqs5_d(this + 0x30) as i32;
    SCOREBOARD.record(depois == if ultima { antes } else { antes + 1 }, || format!("eng_auto_cup_advance_stage({:#x}) fase {} -> {}", this, antes, depois));
}
