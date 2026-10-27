// ---------- 241a leva: a tabela da liga: ordenar, a minitabela do confronto direto, liberar e conferir o fim (league.cpp) ----------

unsafe fn cpa1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpa1_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpa1_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// Linhas da tabela: 0x3b bytes; +0 clube, +4 posição, +5 confrontos diretos (entradas de 14 bytes), +9 histórico
/// das posições (um byte por rodada), +0x11 jogos e, a cada 14 bytes por coluna (+0xad), os pontos em +0x19.
const CPA1_LINHA: usize = 0x3b;

/// { 0x6827d0 } (thiscall, league.cpp): `League::sort_table(linhas)`: ordena a tabela (`linhas` nula: a da liga,
/// +0xb1). Antes acerta o total de rodadas +0xa7 (−1: (+0x3e − 1) × +0x3c, com clubes e +0x45). A regra 6 (+0xc5,
/// sem coluna +0xad) só ordena (qsort 0x945333 com 0x683210); a regra 4 com confrontos monta a minitabela
/// (0x682b30) e, sendo a primeira regra, ordena com 0x683410 e passa bolha nas linhas da liga (+0xb1) enquanto
/// trocar: empate em pontos (ou na comparação própria +0xa3) decidido pela 0x683210; senão qsort com 0x683210.
/// Por fim (+0x44 ≠ 0xfe, sem coluna) grava a posição em +4 e no histórico +9 (rodada +0x11 até +0xa7). Guarda a
/// liga em [0xb679fc] e a comparação própria em [0xb67a00]. Devolve 1. Sem SEH, sem sorteio.
unsafe extern "thiscall" fn ordenar_tabela_da_liga(liga: usize, linhas: usize) -> u32 {
    let n = || cpa1_w(liga + 0x3e);
    let linhas = if linhas == 0 { cpa1_d(liga + 0xb1) } else { linhas };
    if cpa1_w(liga + 0xa7) == -1 && n() > 0 && cpa1_d(liga + 0x45) != 0 {
        core::ptr::write_unaligned((liga + 0xa7) as *mut i16, (n() - 1).wrapping_mul(cpa1_w(liga + 0x3c)));
    }
    if linhas == 0 {
        return 1;
    }
    let qsort: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(0x945333usize);
    let ordem: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(0x683210usize);
    *(0xb679fc as *mut usize) = liga;
    let coluna = cpa1_d(liga + 0xad);
    let mut so_ordenar = false;
    let mut confronto = false;
    for k in 0..4usize {
        let regra = cpa1_b(liga + 0xc5 + k);
        if coluna == 0 && regra == 6 {
            so_ordenar = true;
            break;
        }
        if regra == 0 {
            break;
        }
        if regra == 4 && n() != 0 && cpa1_d(linhas + 5) != 0 {
            confronto = k == 0;
            let mini: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x682b30usize);
            mini(liga, linhas);
            break;
        }
    }
    if so_ordenar {
        qsort(linhas, n() as i32, CPA1_LINHA as u32, 0x683210);
    } else {
        *(0xb67a00 as *mut usize) = cpa1_d(liga + 0xa3);
        if confronto {
            qsort(linhas, n() as i32, CPA1_LINHA as u32, 0x683410);
            loop {
                let mut trocou = false;
                if n() as i32 - 1 <= 0 {
                    break;
                }
                let mut i: i8 = 0;
                loop {
                    let (oa, ob) = (CPA1_LINHA * i as i32 as usize, CPA1_LINHA * (i as i32 + 1) as usize);
                    let (a, b) = (cpa1_d(liga + 0xb1) + oa, cpa1_d(liga + 0xb1) + ob);
                    let col = cpa1_d(*(0xb679fc as *const usize) + 0xad);
                    let decidir = cpa1_d(a) == 0 || cpa1_d(b) == 0 || cpa1_empate(a, b, col);
                    if decidir && ordem(cpa1_d(liga + 0xb1) + oa, cpa1_d(liga + 0xb1) + ob) > 0 {
                        let t = cpa1_d(liga + 0xb1);
                        core::ptr::swap_nonoverlapping((t + oa) as *mut u8, (t + ob) as *mut u8, CPA1_LINHA); // a troca das linhas
                        trocou = true;
                    }
                    i = i.wrapping_add(1);
                    if i as i32 >= n() as i32 - 1 {
                        break;
                    }
                }
                if !trocou {
                    break;
                }
            }
        } else {
            qsort(linhas, n() as i32, CPA1_LINHA as u32, 0x683210);
        }
    }
    if cpa1_b(liga + 0x44) != 0xfe && cpa1_d(liga + 0xad) == 0 && n() > 0 {
        let mut c: i8 = 0;
        loop {
            let r = linhas + CPA1_LINHA * c as i32 as usize;
            *((r + 4) as *mut i8) = c; // a posição
            let hist = cpa1_d(r + 9);
            let rodada = cpa1_b(r + 0x11) as i8;
            if hist != 0 && rodada > 0 && rodada as i16 <= cpa1_w(liga + 0xa7) {
                *((hist + rodada as usize - 1) as *mut i8) = c;
            }
            c = c.wrapping_add(1);
            if c as i16 >= n() {
                break;
            }
        }
    }
    1
}

/// A bolha da 0x6827d0 decide o par (a, b) pela 0x683210? Só quando empatam: na comparação própria [0xb67a00]
/// (chamada de novo quando dá diferente de 0) ou, sem ela, nos pontos (+0x19 da coluna) e, com os dois sem jogos
/// (+0x11), no id dos clubes; com jogos, no confronto direto (gols +5/+7 e +8/+0xa da entrada do clube b).
unsafe fn cpa1_empate(a: usize, b: usize, col: usize) -> bool {
    let propria = *(0xb67a00 as *const usize);
    if propria != 0 {
        let f: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(propria);
        if f(a, b) != 0 {
            let f: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(*(0xb67a00 as *const usize));
            return f(a, b) == 0;
        }
    }
    let o = col.wrapping_mul(14);
    let (pa, pb) = (cpa1_w(a + o + 0x19), cpa1_w(b + o + 0x19));
    if pa != pb {
        return false;
    }
    if cpa1_b(a + o + 0x11) == 0 && cpa1_b(b + o + 0x11) == 0 {
        let (ca, cb) = (cpa1_d(a), cpa1_d(b));
        if ca == 0 || cb == 0 {
            return true;
        }
        return *(ca as *const i32) == *(cb as *const i32);
    }
    let m = cpa1_w(*(0xb679fc as *const usize) + 0x3e) as i32 - 1;
    if m <= 0 {
        return true;
    }
    let (h, cb) = (cpa1_d(a + 5), cpa1_d(b));
    let mut k: i16 = 0;
    loop {
        let e = h.wrapping_add(14 * k as i32 as usize);
        if cpa1_d(e) == cb {
            if e == 0 {
                return true;
            }
            return cpa1_b(e + 5) == cpa1_b(e + 7) && cpa1_w(e + 8) == cpa1_w(e + 0xa);
        }
        k = k.wrapping_add(1);
        if k as i32 >= m {
            return true;
        }
    }
}

/// As regiões da ordenação: as linhas pedidas e as da liga, o total de rodadas, as duas globais e o histórico de
/// posições de cada linha (o byte gravado vai até a rodada +0x11, um byte com sinal: no máximo 127).
unsafe fn cpa1_regioes_da_ordem(liga: usize, linhas: usize) -> Vec<(usize, usize)> {
    let n = cpa1_w(liga + 0x3e).max(0) as usize;
    let linhas = if linhas == 0 { cpa1_d(liga + 0xb1) } else { linhas };
    let mut reg = vec![(liga + 0xa7, 2), (0xb679fc, 4), (0xb67a00, 4)];
    if linhas == 0 {
        return reg;
    }
    reg.push((linhas, CPA1_LINHA * n));
    let propria = cpa1_d(liga + 0xb1);
    if propria != 0 && propria != linhas {
        reg.push((propria, CPA1_LINHA * n));
    }
    let mut total = cpa1_w(liga + 0xa7);
    if total == -1 && n > 0 && cpa1_d(liga + 0x45) != 0 {
        total = (n as i16 - 1).wrapping_mul(cpa1_w(liga + 0x3c));
    }
    let tam = total.clamp(0, 127) as usize;
    if tam > 0 {
        for i in 0..n {
            let hist = cpa1_d(linhas + CPA1_LINHA * i + 9);
            if hist != 0 {
                reg.push((hist, tam));
            }
        }
    }
    reg
}

unsafe extern "thiscall" fn comparar_ordenar_tabela_da_liga(liga: usize, linhas: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(773);
    let reg = cpa1_regioes_da_ordem(liga, linhas);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(liga, linhas), || ordenar_tabela_da_liga(liga, linhas));
    PLACAR.registrar(igual && r == n, || format!("ordenar_tabela_da_liga({:x}, {:x}) {}/{} {:x?} x {:x?}", liga, linhas, r, n, mo, mr));
    r
}

/// { 0x682b30 } (thiscall, league.cpp): `League::build_h2h_table(linhas)` (sem nome no mapa da comunidade): uma
/// minitabela (calloc 0x945501 de +0x3e linhas) com o clube de cada linha e, para cada outro clube com os mesmos
/// pontos (+0x19), a soma do confronto direto contra ele (entrada de +5: bytes +4..+7 em +0x11..+0x14, palavras
/// +8/+0xa/+0xc em +0x15/+0x17/+0x19); ordena com 0x6834f0 e grava em +4 de cada linha a posição, que começa no
/// número de clubes e desce a cada par da minitabela que a 0x6834f0 separa. Solta a minitabela (0x9452ca). Sem
/// memória: a original (a caixa do league.cpp). Chamada pela 0x6827d0 na regra 4.
unsafe extern "thiscall" fn mini_tabela_do_confronto(liga: usize, linhas: usize) {
    let n = || cpa1_w(liga + 0x3e);
    let calloc: unsafe extern "C" fn(i32, u32) -> usize = core::mem::transmute(0x945501usize);
    let t = calloc(n() as i32, CPA1_LINHA as u32);
    if t == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize)>(774)(liga, linhas); // caixa: só a original
    }
    let linha = |base: usize, i: i16| base + CPA1_LINHA * i as i32 as usize;
    let mut i: i16 = 0;
    if n() > 0 {
        loop {
            let (ri, mi) = (linha(linhas, i), linha(t, i));
            *(mi as *mut usize) = cpa1_d(ri);
            let mut j: i16 = 0;
            if n() > 0 {
                loop {
                    let rj = linha(linhas, j);
                    if i != j && cpa1_w(ri + 0x19) == cpa1_w(rj + 0x19) {
                        let m = n() as i32 - 1;
                        let (h, cj) = (cpa1_d(ri + 5), cpa1_d(rj));
                        let mut k: i16 = 0;
                        while (k as i32) < m {
                            let e = h + 14 * k as i32 as usize;
                            if cpa1_d(e) == cj {
                                for (de, para) in [(4, 0x11), (5, 0x12), (6, 0x13), (7, 0x14)] {
                                    *((mi + para) as *mut u8) = cpa1_b(mi + para).wrapping_add(cpa1_b(e + de)); // o confronto somado
                                }
                                for (de, para) in [(8, 0x15), (0xa, 0x17), (0xc, 0x19)] {
                                    let v = cpa1_w(mi + para).wrapping_add(cpa1_w(e + de));
                                    core::ptr::write_unaligned((mi + para) as *mut i16, v);
                                }
                                break;
                            }
                            k = k.wrapping_add(1);
                        }
                    }
                    j = j.wrapping_add(1);
                    if j >= n() {
                        break;
                    }
                }
            }
            i = i.wrapping_add(1);
            if i >= n() {
                break;
            }
        }
    }
    let qsort: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(0x945333usize);
    qsort(t, n() as i32, CPA1_LINHA as u32, 0x6834f0);
    let separa: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(0x6834f0usize);
    let mut pos = n() as u8;
    let mut a: i16 = 0;
    if n() > 0 {
        loop {
            let clube = cpa1_d(linha(t, a));
            let mut c: i16 = 0;
            while c < n() {
                if clube == cpa1_d(linha(linhas, c)) {
                    *((linha(linhas, c) + 4) as *mut u8) = pos;
                    break;
                }
                c = c.wrapping_add(1);
            }
            if (a as i32 + 1) < n() as i32 && separa(linha(t, a), linha(t, a + 1)) != 0 {
                pos = pos.wrapping_sub(1); // a posição desce
            }
            a = a.wrapping_add(1);
            if a >= n() {
                break;
            }
        }
    }
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    free(t);
}

unsafe extern "thiscall" fn comparar_mini_tabela_do_confronto(liga: usize, linhas: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(774);
    if linhas == 0 {
        return o(liga, linhas);
    }
    let reg = [(linhas, CPA1_LINHA * cpa1_w(liga + 0x3e).max(0) as usize)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(liga, linhas), || mini_tabela_do_confronto(liga, linhas));
    PLACAR.registrar(igual, || format!("mini_tabela_do_confronto({:x}, {:x}) {:x?} x {:x?}", liga, linhas, mo, mr));
}

/// { 0x687970 } (thiscall, league.cpp): `League::free_table(tabela)`: `tabela` nula = a da liga (+0xb1). Com clubes
/// (+0x3e ≥ 1) e tabela: solta (0x9452ca) os vetores +5 de cada linha (se o da primeira não é nulo), depois os +9
/// (idem) e a tabela; sendo a da liga, zera +0x3e e +0xb1. Sem SEH, sem sorteio.
unsafe extern "thiscall" fn liberar_tabela_da_liga(liga: usize, tabela: usize) {
    let (t, propria) = if tabela == 0 { (cpa1_d(liga + 0xb1), true) } else { (tabela, false) };
    if cpa1_w(liga + 0x3e) < 1 || t == 0 {
        return;
    }
    crate::comparacao::ramo("liga liberar tabela");
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    for campo in [5usize, 9] {
        if cpa1_d(t + campo) != 0 {
            let mut i = 0usize;
            while (i as i32) < cpa1_w(liga + 0x3e) as i32 {
                let p = cpa1_d(t + campo + CPA1_LINHA * i);
                if p != 0 {
                    free(p);
                }
                i += 1;
            }
        }
    }
    free(t);
    if propria { // a da liga: zera a contagem e a tabela
        core::ptr::write_unaligned((liga + 0x3e) as *mut i16, 0);
        core::ptr::write_unaligned((liga + 0xb1) as *mut usize, 0);
    }
}

/// Comparador: liberar duas vezes soltaria os mesmos blocos duas vezes.
// ponytail: só a original (como as liberações da 232ª); a prova é a carreira e o ramo da cópia no modo Rust.
unsafe extern "thiscall" fn comparar_liberar_tabela_da_liga(liga: usize, tabela: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize)>(775)(liga, tabela)
}

/// { 0x689ad0 } (thiscall, league.cpp): `League::check_complete(fases, avisar)`: 0 sem tabela, encerrada (+0xb5) ou
/// sem +0x45. Fora da fase única (+0x44 ≠ 0xff) já completa (+0xdd): 1. Não completa: acerta o total de rodadas
/// +0xa7 (−1) e, se algum clube ainda não jogou todas (+0x11 < +0xa7), 0; senão, nas divisões de liga (0x4b0700)
/// com +0xa9 > 8 e sem +0xe6, os registros de fim de liga (0x86a070), e sem +0xe6 os prêmios (0x682e10). Com
/// `fases` = 1 na fase única: cada fase de +0xc (0..=+0x30) tem de estar completa (virtual +0x10 com 0, 1) e
/// +0x30 ≥ +0x2c − 1, senão 0. Marca +0xdd (antes, com `avisar`, a virtual +0x94) e devolve 1. Sem SEH, sem sorteio.
unsafe extern "thiscall" fn liga_completa(liga: usize, fases: u32, avisar: u32) -> u8 {
    let tab = cpa1_d(liga + 0xb1);
    if tab == 0 || cpa1_b(liga + 0xb5) != 0 || cpa1_d(liga + 0x45) == 0 {
        return 0;
    }
    let unica = cpa1_b(liga + 0x44) == 0xff;
    if cpa1_d(liga + 0xdd) == 0 {
        if cpa1_w(liga + 0xa7) == -1 && cpa1_w(liga + 0x3e) > 0 {
            core::ptr::write_unaligned((liga + 0xa7) as *mut i16, (cpa1_w(liga + 0x3e) - 1).wrapping_mul(cpa1_w(liga + 0x3c)));
        }
        let rodadas = cpa1_w(liga + 0xa7);
        for i in 0..cpa1_w(liga + 0x3e).max(0) as usize {
            if ((cpa1_b(tab + CPA1_LINHA * i + 0x11) as i8) as i16) < rodadas { // ainda falta jogar
                return 0;
            }
        }
        crate::comparacao::ramo("liga completa: fim da liga");
        let divisao: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(0x4b0700usize);
        let id = *(cpa1_d(liga + 4) as *const u32);
        let mut premios = true;
        if divisao(id) != 0 && cpa1_w(liga + 0xa9) > 8 {
            if cpa1_d(liga + 0xe6) != 0 {
                premios = false;
            } else {
                let registros: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x86a070usize);
                registros(*(0xdd7f00 as *const usize), liga);
            }
        }
        if premios && cpa1_d(liga + 0xe6) == 0 {
            let pagar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x682e10usize);
            pagar(liga);
        }
    } else if !unica {
        return 1;
    }
    if fases == 1 && cpa1_b(liga + 0x44) == 0xff {
        crate::comparacao::ramo("liga completa: fases");
        let mut k = 0usize;
        if cpa1_d(liga + 0x30) as i32 >= 0 {
            loop {
                let fase = cpa1_d(cpa1_d(liga + 0xc) + 4 * k);
                let completa: unsafe extern "thiscall" fn(usize, u32, u32) -> u8 = core::mem::transmute(cpa1_d(cpa1_d(fase) + 0x10));
                if completa(fase, 0, 1) == 0 {
                    return 0;
                }
                k += 1;
                if k as i32 > cpa1_d(liga + 0x30) as i32 {
                    break;
                }
            }
        }
        if (cpa1_d(liga + 0x30) as i32) < (cpa1_d(liga + 0x2c) as i32).wrapping_sub(1) {
            return 0;
        }
    }
    if cpa1_d(liga + 0xdd) == 0 {
        if avisar != 0 {
            let fim: unsafe extern "thiscall" fn(usize) = core::mem::transmute(cpa1_d(cpa1_d(liga) + 0x94));
            fim(liga);
        }
        core::ptr::write_unaligned((liga + 0xdd) as *mut u32, 1);
    }
    1
}

/// O caminho da 0x689ad0 que só lê (e acerta +0xa7)? Escrito à parte da cópia: sem tabela, encerrada ou sem +0x45;
/// completa fora da fase única, ou na única sem pedir as fases; ou algum clube sem todos os jogos.
unsafe fn cpa1_completa_so_le(liga: usize, fases: u32) -> bool {
    let tab = cpa1_d(liga + 0xb1);
    if tab == 0 || cpa1_b(liga + 0xb5) != 0 || cpa1_d(liga + 0x45) == 0 {
        return true;
    }
    if cpa1_d(liga + 0xdd) != 0 {
        return cpa1_b(liga + 0x44) != 0xff || fases != 1;
    }
    let n = cpa1_w(liga + 0x3e);
    let mut total = cpa1_w(liga + 0xa7);
    if total == -1 && n > 0 {
        total = (n - 1).wrapping_mul(cpa1_w(liga + 0x3c));
    }
    (0..n.max(0) as usize).any(|i| ((cpa1_b(tab + 0x3b * i + 0x11) as i8) as i16) < total)
}

unsafe extern "thiscall" fn comparar_liga_completa(liga: usize, fases: u32, avisar: u32) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32) -> u8>(776);
    if !cpa1_completa_so_le(liga, fases) {
        return o(liga, fases, avisar); // fim de liga (registros, prêmios, fases, aviso): só a original
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(liga + 0xa7, 2)], || o(liga, fases, avisar), || liga_completa(liga, fases, avisar));
    PLACAR.registrar(igual && r == n, || format!("liga_completa({:x}, {}, {}) {}/{} {:x?} x {:x?}", liga, fases, avisar, r, n, mo, mr));
    r
}

// ---------- 241a leva: o fim de temporada da liga: os direitos de TV, as honras da temporada e o registro no histórico de competições (league.cpp, comp_util.cpp) ----------

unsafe fn cpa2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpa2_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O pagamento da 0x683010 (`noticia` falso: sem a notícia da 0x48ea40, para o comparador).
unsafe fn cpa2_pagar_tv(liga: usize, valor: i32, filtro: usize, noticia: bool) {
    let financas = *(0xae2a38 as *const usize);
    if financas == 0 || cpa2_w(liga + 0x3e) <= 0 {
        return;
    }
    crate::comparacao::ramo("liga direitos de tv");
    let financas_do_clube: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let somar_tv: unsafe extern "thiscall" fn(usize, i32) = core::mem::transmute(0x5987c0usize);
    let mut i: i16 = 0;
    loop {
        let linha = cpa2_d(liga + 0xb1) + 0x3b * i as i32 as usize;
        let clube = cpa2_d(linha);
        if filtro == 0 || cpa2_d(clube + 0x5b) == filtro {
            somar_tv(financas_do_clube(*(0xae2a38 as *const usize), clube), valor); // o dinheiro da TV
            if noticia {
                let avisar: unsafe extern "thiscall" fn(usize, usize, i32) = core::mem::transmute(0x48ea40usize);
                avisar(liga, cpa2_d(cpa2_d(liga + 0xb1) + 0x3b * i as i32 as usize), valor);
            }
        }
        i = i.wrapping_add(1);
        if i >= cpa2_w(liga + 0x3e) {
            break;
        }
    }
}

/// { 0x683010 } (thiscall, league.cpp): `League::pay_tv_rights(valor, filtro)`: com a tabela de finanças
/// [0xae2a38], a cada clube da tabela (+0xb1; com `filtro`, só os de +0x5b igual a ele) soma `valor` à receita de TV
/// (0x5a0590 → 0x5987c0) e manda a notícia (Comp::send_tv_rights_news 0x48ea40, só para clube de técnico humano).
/// Sem SEH, sem sorteio, sem locais.
unsafe extern "thiscall" fn pagar_direitos_de_tv(liga: usize, valor: i32, filtro: usize) {
    cpa2_pagar_tv(liga, valor, filtro, true)
}

/// Comparador: as finanças (0x167 bytes) de cada clube da tabela; a cópia roda sem a notícia (a da original fica, uma
/// vez só). Clube nulo na tabela (a caixa da 0x5a0590): só a original.
unsafe extern "thiscall" fn comparar_pagar_direitos_de_tv(liga: usize, valor: i32, filtro: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32, usize)>(777);
    let financas = *(0xae2a38 as *const usize);
    let n = cpa2_w(liga + 0x3e).max(0) as usize;
    let clubes: Vec<usize> = (0..n).map(|i| cpa2_d(cpa2_d(liga + 0xb1) + 0x3b * i)).collect();
    if financas == 0 || clubes.contains(&0) {
        return o(liga, valor, filtro);
    }
    let financas_do_clube: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let reg: Vec<(usize, usize)> = clubes.iter().map(|&c| (financas_do_clube(financas, c), 0x167)).collect();
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(liga, valor, filtro), || cpa2_pagar_tv(liga, valor, filtro, false));
    PLACAR.registrar(igual, || format!("pagar_direitos_de_tv({:x}, {}, {:x}) {:x?} x {:x?}", liga, valor, filtro, mo, mr));
}

/// { 0x684640 } (thiscall, league.cpp): `League::record_season_honours()`: campeão, vice e (com mais de 2 clubes)
/// terceiro da tabela (+0xb1). Se a divisão do campeão (+0x57) é esta competição (+4), zera nos clubes ([0xae23b4],
/// 0x245 bytes, [0xae235c] − [0xae2350]) o +0x5b que aponta para ela. Se a virtual +0x88 da liga da competição
/// ([0xadadfc][id]) pede, o destaque é a seleção (0x53b3d0) do país-sede (HostCountry::find_hosts 0x5fa730 com o ano
/// +0x40): com duas sedes, a de maior +0x8e (empate: a primeira). Grava no histórico (0x4afce0) e devolve o que ela
/// devolve. Sem SEH, sem sorteio; as duas locais das sedes só são lidas quando a 0x5fa730 as grava.
unsafe extern "thiscall" fn registrar_honras_da_temporada(liga: usize) -> usize {
    crate::comparacao::ramo("liga honras da temporada");
    let tab = cpa2_d(liga + 0xb1);
    let vice = cpa2_d(tab + 0x3b);
    let campeao = cpa2_d(tab);
    let terceiro = if cpa2_w(liga + 0x3e) > 2 { cpa2_d(tab + 0x76) } else { 0 };
    if cpa2_d(campeao + 0x57) == cpa2_d(liga + 4) {
        let quantos = || (*(0xae235c as *const i32)).wrapping_sub(*(0xae2350 as *const i32));
        let mut k = 0i32;
        while k < quantos() {
            let p = (*(0xae23b4 as *const usize)).wrapping_add(0x245 * k as usize + 0x5b);
            if cpa2_d(p) == cpa2_d(liga + 4) {
                core::ptr::write_unaligned(p as *mut usize, 0); // a divisão anterior
            }
            k += 1;
        }
    }
    let comp = cpa2_d(liga + 4);
    let alvo = *((*(0xadadfc as *const usize)).wrapping_add(4 * *(comp as *const u32) as usize) as *const usize);
    let pede: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cpa2_d(cpa2_d(alvo) + 0x88));
    let mut destaque = 0usize;
    if pede(alvo) != 0 {
        let sedes: unsafe extern "thiscall" fn(usize, u32, u32, *mut i32, *mut i32, u32) -> u8 = core::mem::transmute(0x5fa730usize);
        let selecao: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
        let (mut a, mut b) = (0i32, 0i32);
        let q = sedes(*(0xb5e134 as *const usize), *(cpa2_d(liga + 4) as *const u32), cpa2_w(liga + 0x40) as u16 as u32, &mut a, &mut b, 0);
        let nacao = |k: i32| (*(0xae23a8 as *const usize)).wrapping_add(k.wrapping_mul(0x122) as usize);
        if q == 2 {
            destaque = if cpa2_w(nacao(a) + 0x8e) < cpa2_w(nacao(b) + 0x8e) { selecao(nacao(b)) } else { selecao(nacao(a)) };
        } else if q != 0 {
            destaque = selecao(nacao(a));
        }
    }
    let registrar: unsafe extern "C" fn(usize, usize, usize, usize, usize) -> usize = core::mem::transmute(0x4afce0usize);
    registrar(liga, campeao, vice, terceiro, destaque) // campeão, vice, terceiro
}

/// As regiões do histórico de competições: contagem, ponteiro e as linhas (26 bytes).
unsafe fn cpa2_regioes_do_historico() -> Vec<(usize, usize)> {
    let n = (*(0xae237c as *const i32)).max(0) as usize;
    vec![(0xae237c, 4), (0xae23d4, 4), (*(0xae23d4 as *const usize), 26 * n)]
}

/// Comparador: os clubes (o +0x5b) e o histórico de competições. Sem linha livre no histórico (a 0x4afce0 faria
/// realloc): só a original.
unsafe extern "thiscall" fn comparar_registrar_honras_da_temporada(liga: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> usize>(778);
    if cpa2_linha_livre() == 0 {
        return o(liga);
    }
    let mut reg = cpa2_regioes_do_historico();
    let quantos = (*(0xae235c as *const i32)).wrapping_sub(*(0xae2350 as *const i32)).max(0) as usize;
    reg.push((*(0xae23b4 as *const usize), 0x245 * quantos));
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(liga), || registrar_honras_da_temporada(liga));
    PLACAR.registrar(igual && r == n, || format!("registrar_honras_da_temporada({:x}) {:x}/{:x} {:x?} x {:x?}", liga, r, n, mo, mr));
    r
}

/// A linha livre do histórico de competições ([0xae23d4], [0xae237c] linhas de 26 bytes): a seguinte à última com
/// +4 (competição) preenchido; nenhuma preenchida, ou a última preenchida: 0 (falta crescer).
unsafe fn cpa2_linha_livre() -> usize {
    let n = *(0xae237c as *const i32);
    let t = *(0xae23d4 as *const usize);
    let mut i = n - 1;
    while i >= 0 {
        if cpa2_d(t.wrapping_add(26 * i as usize + 4)) != 0 {
            break;
        }
        i -= 1;
    }
    if i < 0 || i >= n - 1 {
        return 0;
    }
    t.wrapping_add(26 * (i + 1) as usize)
}

/// { 0x4afce0 } (cdecl, comp_util.cpp): `comp_util_add_history_record(liga, primeiro, segundo, terceiro, extra)`:
/// a linha livre do histórico de competições (sem ela, cresce 20 linhas pela 0x527f60 e usa a primeira nova; sem
/// memória, 0) recebe a competição (+4 da liga), o ano [0xae2c92] (+8), e os quatro valores em +0xa, +0xe, +0x12,
/// +0x16. Devolve a linha. Sem SEH, sem sorteio, sem locais.
unsafe extern "C" fn acrescentar_ao_historico(liga: usize, a: u32, b: u32, c: u32, d: u32) -> usize {
    let mut r = cpa2_linha_livre();
    if r == 0 {
        crate::comparacao::ramo("histórico de competições crescer");
        let crescer: unsafe extern "C" fn(i32) -> u32 = core::mem::transmute(0x527f60usize);
        if crescer(0x14) == 0 {
            return 0;
        }
        let n = *(0xae237c as *const i32);
        r = (*(0xae23d4 as *const usize)).wrapping_add(26usize.wrapping_mul(n.wrapping_sub(20) as usize));
    }
    core::ptr::write_unaligned((r + 4) as *mut usize, cpa2_d(liga + 4));
    core::ptr::write_unaligned((r + 8) as *mut u16, *(0xae2c92 as *const u16)); // o ano
    for (k, v) in [a, b, c, d].into_iter().enumerate() {
        core::ptr::write_unaligned((r + 0xa + 4 * k) as *mut u32, v);
    }
    r
}

unsafe extern "C" fn comparar_acrescentar_ao_historico(liga: usize, a: u32, b: u32, c: u32, d: u32) -> usize {
    let o = orig::<unsafe extern "C" fn(usize, u32, u32, u32, u32) -> usize>(779);
    if cpa2_linha_livre() == 0 {
        return o(liga, a, b, c, d); // cresce (realloc): só a original
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&cpa2_regioes_do_historico(), || o(liga, a, b, c, d), || acrescentar_ao_historico(liga, a, b, c, d));
    PLACAR.registrar(igual && r == n, || format!("acrescentar_ao_historico({:x}, {:x}, {:x}, {:x}, {:x}) {:x}/{:x} {:x?} x {:x?}", liga, a, b, c, d, r, n, mo, mr));
    r
}

// ---------- 241a leva: os auxiliares de competição: o tipo de confronto na liga, a força do clube e o colocado mais recente (comp_util.cpp) ----------

unsafe fn cpa3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpa3_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// A nota de um clube na estatística da liga (0x4b02e0): pesos 1 nos 4 atributos (+0x72..+0x75; +0x75 e +0x74 com
/// o peso [0x968968]) ou, com menos de 25 jogos (+4), proporcionais aos jogos; a média ponderada, na ordem do x87.
unsafe fn cpa3_nota(e: usize, b73: f64) -> f64 {
    let (mut p, mut k73, mut k72) = (f64_em(0x968968), 1.0f64, 1.0f64);
    let jogos = cpa3_b(e + 4);
    if jogos < 0x19 {
        let v = jogos as f64;
        p = v * f64_em(0x968a98);
        k73 = v * f64_em(0x968a90);
        k72 = f64_em(0x968938) - v * f64_em(0x968a88);
    }
    let s = cpa3_b(e + 0x75) as f64 * p + cpa3_b(e + 0x72) as f64 * k72 + b73 * k73 + cpa3_b(e + 0x74) as f64 * p;
    s / (p + p + k72 + k73)
}

/// { 0x4b02e0 } (cdecl, comp_util.cpp): `comp_util_get_league_clash_type(a, b)`: os dois clubes na mesma divisão
/// (+0x57, não reserva nem menor: 0x7e6f20), a liga dela ([0xadadfc][id]) com estatística (+8, 0x4a4850 dos dois),
/// sem fases (+0x30 = −1) e com +0x43 = 2, e a diferença de +0x73 até um quarto dos clubes (+0x3e). Com as notas
/// (cpa3_nota): 1 = briga pelo título (virtual +0x98 = 0, as duas abaixo de um terço dos clubes e cada uma não abaixo
/// do +0x74 do outro); 3 = briga na zona (+0x98 ≠ 0, as duas até 0,2 × clubes + +0xbe, ou com +0xbf até 0,2 × clubes
/// + +0xbe + +0xbf, e o mesmo cruzamento do +0x74); senão 0. Devolve só `al`. Sem SEH, sem sorteio; a virtual +0x98
/// é chamada até três vezes, como na original.
unsafe extern "C" fn tipo_de_confronto_na_liga(a: usize, b: usize) -> u8 {
    let comp = cpa3_d(a + 0x57);
    if comp == 0 || comp != cpa3_d(b + 0x57) {
        return 0;
    }
    let menor: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(0x7e6f20usize);
    if menor(comp) != 0 {
        return 0;
    }
    let liga = cpa3_d((*(0xadadfc as *const usize)).wrapping_add(4 * *(cpa3_d(a + 0x57) as *const u32) as usize));
    if liga == 0 || cpa3_d(liga + 8) == 0 {
        return 0;
    }
    let achar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
    let ea = achar(cpa3_d(liga + 8), a);
    let eb = achar(cpa3_d(liga + 8), b);
    if ea == 0 || eb == 0 || cpa3_d(liga + 0x30) as i32 != -1 || cpa3_b(liga + 0x43) != 2 {
        return 0;
    }
    let (a73, b73) = (cpa3_b(ea + 0x73) as i32, cpa3_b(eb + 0x73) as i32);
    let clubes = core::ptr::read_unaligned((liga + 0x3e) as *const i16) as i32;
    if (a73 - b73).abs() > ftol(clubes as f64 * f64_em(0x9689e0)) as i16 as i32 {
        return 0;
    }
    let (na, nb) = (cpa3_nota(ea, a73 as f64), cpa3_nota(eb, b73 as f64));
    let (a74, b74) = (cpa3_b(ea + 0x74) as f64, cpa3_b(eb + 0x74) as f64);
    let virtual98 = || {
        let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cpa3_d(cpa3_d(liga) + 0x98));
        f(liga)
    };
    // as comparações do x87: NaN conta como "menor" (C0) e "menor ou igual" (C0|C3)
    let cruzadas = !(a74 > nb) && !(b74 > na);
    if virtual98() == 0 {
        let t = clubes as f64 * f64_em(0x968a80);
        if !(na >= t) && !(nb >= t) && cruzadas {
            return 1; // briga pelo título
        }
    }
    let be = cpa3_b(liga + 0xbe) as i8 as f64;
    if virtual98() != 0 {
        let t = clubes as f64 * f64_em(0x968918) + be;
        if !(na > t) && !(nb > t) && cruzadas {
            return 3;
        }
    }
    if virtual98() == 0 {
        return 0;
    }
    let bf = cpa3_b(liga + 0xbf) as i8;
    if bf == 0 {
        return 0;
    }
    let t = clubes as f64 * f64_em(0x968918) + be + bf as f64;
    if !(na > t) && !(nb > t) && cruzadas {
        3
    } else {
        0
    }
}

// ponytail: ~4 milhões de chamadas na carreira: comparação só do valor, sem regiões (a rotina só lê; a 0x4a4850 pode
// reordenar a estatística, e a original já deixa ordenada para a cópia).
unsafe extern "C" fn comparar_tipo_de_confronto_na_liga(a: usize, b: usize) -> u8 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> u8>(780)(a, b);
    let n = tipo_de_confronto_na_liga(a, b);
    PLACAR.registrar(r == n, || format!("tipo_de_confronto_na_liga({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// { 0x4c0720 } (cdecl, comp_util.cpp): `comp_util_get_club_strength_score(clube)`: o dinheiro (sem tabela de
/// finanças [0xae2a38]: +0x65 / 400; com ela, o saldo de 64 bits / 300 pela 0x5a0590), mais 16 × a reputação (+0x80),
/// mais a nota fora da partida (0x6e6340, 0x840, limite −1) dos 15 escalados (0x6e2000, códigos 1, 0x804, 0x84,
/// 3 × 0x204, 0x810, 0x90, 3 × 0x210, 4 × 0x40) entre os jogadores (+0x61) do elenco (0x541f90). Clube nulo:
/// −10.000.000. Sem SEH, sem sorteio próprio. A original empurra o número de jogadores como dword com só o byte de
/// baixo gravado (o resto é sobra de pilha); a 0x6e2000 só lê o byte, e a cópia passa o byte estendido.
unsafe extern "C" fn forca_do_clube(clube: usize) -> i32 {
    if clube == 0 {
        return -10_000_000;
    }
    let financas = *(0xae2a38 as *const usize);
    let mut forca: i32 = 0;
    if financas == 0 {
        forca = core::ptr::read_unaligned((clube + 0x65) as *const i32) / 400;
    } else {
        let financas_do_clube: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
        let f = financas_do_clube(financas, clube);
        if f != 0 {
            forca = (core::ptr::read_unaligned(f as *const i64) / 300) as i32;
        }
    }
    forca = forca.wrapping_add((core::ptr::read_unaligned((clube + 0x80) as *const i16) as i32) << 4); // a reputação
    let mut elenco = [0usize; 50];
    let mut jogadores = [0usize; 50];
    let mut escalados = [0usize; 15];
    let codigos: [u16; 15] = [1, 0x804, 0x84, 0x204, 0x204, 0x204, 0x810, 0x90, 0x210, 0x210, 0x210, 0x40, 0x40, 0x40, 0x40];
    let reunir: unsafe extern "C" fn(usize, *mut usize, u32) -> u32 = core::mem::transmute(0x541f90usize);
    reunir(clube, elenco.as_mut_ptr(), 0);
    let mut k: i8 = 0;
    for &p in elenco.iter() {
        if p != 0 && cpa3_d(p + 0x61) != 0 {
            jogadores[k as usize] = p;
            k = k.wrapping_add(1);
        }
    }
    let escalar: Escalar = core::mem::transmute(0x6e2000usize);
    escalar(jogadores.as_ptr() as usize, k as u8 as u32, codigos.as_ptr() as usize, 0xf, 0, escalados.as_mut_ptr() as usize, 0x840, clube, 0, 0, 0, 0, 0, 0, 0);
    let nota: unsafe extern "C" fn(usize, u32, u32, usize, i32, usize) -> i32 = core::mem::transmute(0x6e6340usize);
    for (j, &p) in escalados.iter().enumerate() {
        if p != 0 {
            forca = forca.wrapping_add(nota(p, codigos[j] as u32, 0x840, clube, -1, 0));
        }
    }
    forca
}

// ponytail: 1.117 chamadas; só o valor. A escalação e as notas têm comparadores próprios, e o físico que a nota
// atualiza uma vez (0x62ed70, com sorteio) a original já deixa pronto para a cópia.
unsafe extern "C" fn comparar_forca_do_clube(clube: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize) -> i32>(781)(clube);
    let n = forca_do_clube(clube);
    PLACAR.registrar(r == n, || format!("forca_do_clube({:x}) original={} rust={}", clube, r, n));
    r
}

/// { 0x4afd80 } (cdecl, comp_util.cpp): `comp_util_get_latest_placed_club(comp, lugar)`: ordena o histórico de
/// competições (qsort 0x945333 com 0x540780), busca (bsearch 0x946464) a competição com ano −1 (qualquer), volta até a
/// primeira linha dela e devolve o clube do `lugar` (0 campeão +0xa, 1 vice +0xe, 2 terceiro +0x12); não achou: 0.
/// Outro lugar: a original (a caixa do comp_util.cpp). A chave na pilha da original só tem +4 e +8 gravados, e a
/// 0x540780 só lê esses dois; a cópia zera o resto.
unsafe extern "C" fn clube_colocado_mais_recente(comp: usize, lugar: u32) -> usize {
    let lugar = lugar as u8;
    if lugar > 2 {
        return orig::<unsafe extern "C" fn(usize, u32) -> usize>(782)(comp, lugar as u32); // caixa: só a original
    }
    let qsort: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(0x945333usize);
    let buscar: unsafe extern "C" fn(*const u8, usize, i32, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let comparar: unsafe extern "C" fn(*const u8, usize) -> i32 = core::mem::transmute(0x540780usize);
    qsort(*(0xae23d4 as *const usize), *(0xae237c as *const i32), 0x1a, 0x540780);
    let mut chave = [0u8; 0x1a];
    chave[4..8].copy_from_slice(&comp.to_le_bytes());
    chave[8..10].copy_from_slice(&0xffffu16.to_le_bytes()); // o ano: qualquer
    let mut p = buscar(chave.as_ptr(), *(0xae23d4 as *const usize), *(0xae237c as *const i32), 0x1a, 0x540780);
    if p == 0 {
        return 0;
    }
    let t = *(0xae23d4 as *const usize);
    while p > t {
        p -= 0x1a;
        if comparar(chave.as_ptr(), p) != 0 {
            p += 0x1a;
            break;
        }
    }
    cpa3_d(p + [0xa, 0xe, 0x12][lugar as usize]) // campeão, vice, terceiro
}

unsafe extern "C" fn comparar_clube_colocado_mais_recente(comp: usize, lugar: u32) -> usize {
    let o = orig::<unsafe extern "C" fn(usize, u32) -> usize>(782);
    if lugar as u8 > 2 {
        return o(comp, lugar);
    }
    let n = (*(0xae237c as *const i32)).max(0) as usize;
    let reg = [(*(0xae23d4 as *const usize), 0x1a * n)];
    let (r, x, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(comp, lugar), || clube_colocado_mais_recente(comp, lugar));
    PLACAR.registrar(igual && r == x, || format!("clube_colocado_mais_recente({:x}, {}) {:x}/{:x} {:x?} x {:x?}", comp, lugar as u8, r, x, mo, mr));
    r
}

// ---------- 241a leva: o amistoso marcado do clube e a lista de partidas da data (friendly.cpp, fix_man.cpp) ----------

/// { 0x5c88f0 } (thiscall, ret 8, friendly.cpp): `find_friendly_match(data, clube)`: o amistoso do clube na lista
/// de amistosos (+0x34c8; nula: 0), do começo (cursor +0xa = +2). Com `data`: os da mesma data (0x4a7f20 com os dois
/// dwords dela) e do clube (+0xc); o primeiro com +0x10 e +0x14 os dois não nulos sai na hora, os outros ficam
/// guardados (o último) e um segundo achado sai na hora; no fim, o guardado (ou 0). Sem `data`: o primeiro do clube
/// com +0x10 ou +0x14 nulo e +8 não nulo. A lista é relida do gerenciador a cada passo, como na original. Na marcação
/// e nas telas dos amistosos (0x5c6f32, 0x5c86de, 0x5c9139), na mídia (0x77ce23, 0x782d9e, 0x784314) e em 0x4741f9.
/// Sem SEH, sem caixa, sem locais.
unsafe extern "thiscall" fn amistoso_do_clube(this: usize, data: usize, clube: usize) -> usize {
    let u = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let mesma_data: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x4a7f20usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let l = u(this + 0x34c8);
    if l == 0 {
        return 0;
    }
    core::ptr::write_unaligned((l + 0xa) as *mut usize, u(l + 2));
    let mut guardado = 0;
    let mut a = proximo(l);
    while a != 0 {
        let do_clube = u(a + 0xc) == clube; // o clube do amistoso
        if data != 0 {
            if mesma_data(a, u(data) as u32, u(data + 4) as u32) != 0 && do_clube {
                if guardado != 0 || (u(a + 0x10) != 0 && u(a + 0x14) != 0) {
                    return a;
                }
                guardado = a;
            }
        } else if do_clube && (u(a + 0x10) == 0 || u(a + 0x14) == 0) && u(a + 8) != 0 {
            return a;
        }
        a = proximo(u(this + 0x34c8));
    }
    guardado
}

comparar_valor!(comparar_amistoso_do_clube, amistoso_do_clube, 783, (data: usize, clube: usize) -> usize);

/// { 0x5ae6a0 } (stdcall, ret 0xc, fix_man.cpp): `get_fixture_list_for_date(data, fatia, grupo)`: a lista de partidas
/// da 0x5abf10 do gerenciador [0xae2a58] para o ano (+2) e o dia (+0) da data. O argumento do ano leva na metade alta a
/// do ponteiro da data (`mov ax, [eax+2]`), reproduzida; o do dia leva a do ecx do chamador, que a 0x5abf10 descarta
/// (`and 0xffff`): a cópia passa zero. No dia de jogos (montar_o_dia 0x6b3c10, Rust, cada lista do dia), nos
/// amistosos (0x5cbf3e), no técnico humano (0x5fe83a), na mídia (0x7776a8) e em 0x7e6498, 0x7e6ac9.
unsafe extern "stdcall" fn lista_da_data(data: usize, fatia: u32, grupo: u32) -> usize {
    let lista: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> usize = core::mem::transmute(0x5abf10usize);
    let ano = (data as u32 & 0xffff_0000) | core::ptr::read_unaligned((data + 2) as *const u16) as u32;
    let dia = core::ptr::read_unaligned(data as *const u16) as u32;
    lista(*(0xae2a58 as *const usize), ano, dia, fatia, grupo) // o gerenciador de partidas
}

unsafe extern "stdcall" fn comparar_lista_da_data(data: usize, fatia: u32, grupo: u32) -> usize {
    let r = orig::<unsafe extern "stdcall" fn(usize, u32, u32) -> usize>(784)(data, fatia, grupo);
    let n = lista_da_data(data, fatia, grupo);
    PLACAR.registrar(r == n, || format!("lista_da_data({:x}, fatia {}, grupo {}) original={:x} rust={:x}", data, fatia, grupo, r, n));
    r
}

// ---------- 241a leva: a próxima partida pelo filtro, o estádio em uso no dia e a data do amistoso internacional (fix_man.cpp, friendly.cpp) ----------

/// A lista das partidas de (ano, dia, fatia, grupo) no gerenciador `this` (0x5abf10, thiscall ret 0x10, original:
/// carrega o ano se preciso), com o cursor voltado ao começo (+0xa = +2), como as duas originais fazem; nula: 0.
unsafe fn cpc1_lista(this: usize, ano: u32, dia: u32, fatia: u32, grupo: u32) -> usize {
    let lista: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> usize = core::mem::transmute(0x5abf10usize);
    let l = lista(this, ano, dia, fatia, grupo);
    if l != 0 {
        core::ptr::write_unaligned((l + 0xa) as *mut u32, core::ptr::read_unaligned((l + 2) as *const u32));
    }
    l
}

/// O próximo item da lista `l` (0x68d1b0, proximo_da_lista), ou 0.
unsafe fn cpc1_proximo(l: usize) -> usize {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    proximo(l)
}

/// A partida `f` passa pelo filtro da find_next_fixture? Cada filtro em -1 deixa passar: a competição (+0), a fase
/// (+0x34, curto com sinal) e os dois clubes (cada um o mandante +0xc ou o visitante +0x10).
unsafe fn cpc1_passa(f: usize, comp: i32, fase: i32, clube_a: i32, clube_b: i32) -> bool {
    let d = |o: usize| core::ptr::read_unaligned((f + o) as *const i32);
    let clube = |c: i32| c == -1 || c == d(0xc) || c == d(0x10);
    (comp == -1 || comp == d(0)) // a competição
        && (fase == -1 || fase == core::ptr::read_unaligned((f + 0x34) as *const i16) as i32)
        && clube(clube_a)
        && clube(clube_b)
}

/// { 0x5ac920 } (thiscall, ret 0x2c, fix_man.cpp): `FIXTURE_MANAGER::find_next_fixture(ano*, dia*, fatia*, adiante,
/// comp, fase, clube_a, clube_b, ...)`: a partida seguinte (ou anterior, `adiante` 0) ao cursor (ano, dia, fatia),
/// o próprio cursor incluído, que passa pelo filtro (cpc1_passa). O grupo das listas vem da 0x48dd70 (a competição
/// da tabela [0xae23d0] + 107 × comp, 0). Adiante: fatias até 2, dias até 365 (0x16e exclusivo), anos seguintes no
/// dia 0 (a fatia só volta a 0 na virada do dia); atrás: fatias até 0, depois 2, dias até 0, anos anteriores no dia
/// 0x16d. A cinco anos do ano do cursor (adiante: com sinal menos sem sinal; atrás: com sinal), 0. Achou: grava o
/// cursor (dia, ano e a fatia em byte) e devolve a partida. Os 3 últimos argumentos não são lidos. A original passa
/// à 0x5abf10 ano, dia e fatia com a metade alta de locais nunca escritas (lixo), mas ela e o que chama (0x5aa2d0,
/// 0x5a9cf0, 0x54cfe0) só usam a palavra/o byte: a cópia passa zero. Nos sorteios de copa (Cup.cpp 0x519d90,
/// 0x51a150, 0x522c70). Sem SEH, sem caixa (a da 0x5abf10 só sem memória).
unsafe extern "thiscall" fn proxima_partida(this: usize, pano: *mut u16, pdia: *mut u16, pfatia: *mut u8, adiante: u32,
    comp: i32, fase: i32, clube_a: i32, clube_b: i32, _a9: u32, _a10: u32, _a11: u32) -> usize {
    let grupo_de: unsafe extern "C" fn(usize, u32) -> u32 = core::mem::transmute(0x48dd70usize);
    let tabela = *(0xae23d0 as *const usize);
    let grupo = grupo_de(tabela.wrapping_add((comp as usize).wrapping_mul(107)), 0) as u8 as u32;
    let (mut ano, mut dia, mut fatia) = (*pano, *pdia, *pfatia as u16);
    let achar = |ano: u16, dia: u16, fatia: u16| -> usize {
        let l = cpc1_lista(this, ano as u32, dia as u32, fatia as u32, grupo);
        if l == 0 {
            return 0;
        }
        loop {
            let f = cpc1_proximo(l);
            if f == 0 || cpc1_passa(f, comp, fase, clube_a, clube_b) {
                return f;
            }
        }
    };
    let f = 'busca: {
        if adiante != 0 {
            loop {
                while (dia as i16) < 0x16e {
                    while (fatia as i16) < 3 {
                        let f = achar(ano, dia, fatia);
                        if f != 0 {
                            break 'busca f;
                        }
                        fatia = fatia.wrapping_add(1);
                    }
                    dia = dia.wrapping_add(1);
                    fatia = 0; // o dia seguinte
                }
                ano = ano.wrapping_add(1);
                dia = 0;
                if (ano as i16 as i32) - (*pano as i32) >= 5 {
                    return 0;
                }
            }
        } else {
            loop {
                while (dia as i16) >= 0 {
                    while (fatia as i16) >= 0 {
                        let f = achar(ano, dia, fatia);
                        if f != 0 {
                            break 'busca f;
                        }
                        fatia = fatia.wrapping_sub(1);
                    }
                    fatia = 2; // o dia anterior
                    dia = dia.wrapping_sub(1);
                }
                ano = ano.wrapping_sub(1);
                dia = 0x16d;
                if (*pano as i16 as i32) - (ano as i16 as i32) >= 5 {
                    return 0;
                }
            }
        }
    };
    *pdia = dia;
    *pano = ano;
    *pfatia = fatia as u8;
    f
}

unsafe extern "thiscall" fn comparar_proxima_partida(this: usize, pano: *mut u16, pdia: *mut u16, pfatia: *mut u8, adiante: u32,
    comp: i32, fase: i32, clube_a: i32, clube_b: i32, a9: u32, a10: u32, a11: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, *mut u16, *mut u16, *mut u8, u32, i32, i32, i32, i32, u32, u32, u32) -> usize>(785);
    let antes = (*pano, *pdia, *pfatia);
    let reg = [(pano as usize, 2), (pdia as usize, 2), (pfatia as usize, 1)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg,
        || o(this, pano, pdia, pfatia, adiante, comp, fase, clube_a, clube_b, a9, a10, a11),
        || proxima_partida(this, pano, pdia, pfatia, adiante, comp, fase, clube_a, clube_b, a9, a10, a11));
    PLACAR.registrar(igual && r == n, || format!("proxima_partida(cursor {:?}, adiante {}, comp {}, fase {}, clubes {:x} {:x}) original={:x} rust={:x} {:x?} x {:x?}",
        antes, adiante, comp, fase, clube_a, clube_b, r, n, mo, mr));
    r
}

/// { 0x5ae870 } (thiscall, ret 0x10, fix_man.cpp): `FIXTURE_MANAGER::is_stadium_in_use(estadio, dia, ano, fatia)`:
/// 1 se alguma partida (+0x18 = o estádio, não nulo) das listas do dia usa o estádio. Os grupos: 0 a 7 e o do estádio
/// (0x48df40, byte), este fora se repete um dos oito. Fatia -1 (byte): as fatias 0 a 2, senão só ela. A fatia vai à
/// 0x5abf10 no byte baixo do argumento do ano (a original guarda o laço lá: os 3 bytes altos são os do ano). Estádio
/// nulo: 0. Na marcação dos amistosos (friendly.cpp 0x5c6f00, 0x5c75a0) e em 0x7e6380, 0x7e6720. Sem SEH, sem locais
/// sem valor.
unsafe extern "thiscall" fn estadio_em_uso(this: usize, estadio: usize, dia: u32, ano: u32, fatia: u32) -> u32 {
    if estadio == 0 {
        return 0;
    }
    let grupo_do_estadio: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x48df40usize);
    let mut grupos = [0u8, 1, 2, 3, 4, 5, 6, 7, grupo_do_estadio(estadio) as u8];
    for i in 0..8 {
        for j in i + 1..9 {
            if grupos[i] as i8 >= 0 && grupos[i] == grupos[j] {
                grupos[j] = 0xff;
            }
        }
    }
    let (ini, fim) = if fatia as u8 == 0xff { (0i8, 2i8) } else { (fatia as i8, fatia as i8) };
    for &g in &grupos {
        if g == 0xff {
            continue;
        }
        let mut s = ini;
        while s <= fim {
            let l = cpc1_lista(this, ano, dia, (ano & 0xffff_ff00) | s as u8 as u32, g as u32);
            if l != 0 {
                loop {
                    let f = cpc1_proximo(l);
                    if f == 0 {
                        break;
                    }
                    let e = core::ptr::read_unaligned((f + 0x18) as *const usize);
                    if e != 0 && e == estadio { // o estádio da partida
                        return 1;
                    }
                }
            }
            s = s.wrapping_add(1);
        }
    }
    0
}

comparar_valor!(comparar_estadio_em_uso, estadio_em_uso, 786, (estadio: usize, dia: u32, ano: u32, fatia: u32) -> u32);

/// { 0x5cbdd0 } (thiscall, ret 4, friendly.cpp): `friendly_international_date_resolve(ano)`: a data (+3) da entrada
/// de data internacional `this` pela 0x54a130 (Date.cpp: dia, mês, ano, dia da semana) com o dia (+1) e o dia da
/// semana (+0) em byte com sinal e o mês (+2). A original monta os argumentos sobre o eax e o edx de entrada: o edx é o
/// ano (metade alta do dia), o eax é o ano também no único chamador (0x5cbc18, `push eax; call`; metade alta do mês e
/// do dia da semana). O eax devolvido é lixo: o endereço de pilha do temporário que a 0x417c10 copia no fim da 0x54a2d0
/// (muda com a profundidade da pilha), e o chamador o descarta (0x5cbc1d `mov eax, [esp+14h]`); só a data (+3..+0xb) conta.
unsafe extern "thiscall" fn data_do_amistoso_internacional(this: usize, ano: u32) -> u32 {
    let acertar: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x54a130usize);
    let b = |o: usize| *((this + o) as *const i8) as i16 as u16 as u32;
    let alto = ano & 0xffff_0000;
    let semana = alto | b(0);
    let mes = alto | (b(0) & 0xff00) | *((this + 2) as *const u8) as u32;
    acertar(this + 3, alto | b(1), mes, ano, semana) // a data com o ano dado
}

/// A original (ret 4) com eax = ano, como no único chamador.
unsafe fn cpc1_data_original(o: usize, this: usize, ano: u32) -> u32 {
    let r: u32;
    core::arch::asm!(
        "push {a}",
        "call {f}",
        a = in(reg) ano,
        f = in(reg) o,
        inout("eax") ano => r,
        inout("ecx") this => _,
        clobber_abi("C"),
    );
    r
}

unsafe extern "thiscall" fn comparar_data_do_amistoso_internacional(this: usize, ano: u32) -> u32 {
    let o = orig::<usize>(787);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 11)], || cpc1_data_original(o, this, ano),
        || data_do_amistoso_internacional(this, ano));
    PLACAR.registrar(igual /* o eax é lixo de pilha: só a memória */, || format!("data_do_amistoso_internacional({:x}, {:x}) original={:x} rust={:x} {:x?} x {:x?}",
        this, ano, r, n, mo, mr));
    r
}
