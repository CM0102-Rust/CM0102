// ---------- %%LEVA%%a leva: a tabela da liga: ordenar, a minitabela do confronto direto, liberar e conferir o fim (league.cpp) ----------

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
unsafe extern "thiscall" fn sort_table(liga: usize, linhas: usize) -> u32 {
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
                    let decidir = cpa1_d(a) == 0 || cpa1_d(b) == 0 || is_sort_tie(a, b, col);
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
unsafe fn is_sort_tie(a: usize, b: usize, col: usize) -> bool {
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
unsafe fn sort_regions(liga: usize, linhas: usize) -> Vec<(usize, usize)> {
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

unsafe extern "thiscall" fn comparar_sort_table(liga: usize, linhas: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%);
    let reg = sort_regions(liga, linhas);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(liga, linhas), || sort_table(liga, linhas));
    PLACAR.registrar(igual && r == n, || format!("sort_table({:x}, {:x}) {}/{} {:x?} x {:x?}", liga, linhas, r, n, mo, mr));
    r
}

/// { 0x682b30 } (thiscall, league.cpp): `League::build_h2h_table(linhas)` (sem nome no mapa da comunidade): uma
/// minitabela (calloc 0x945501 de +0x3e linhas) com o clube de cada linha e, para cada outro clube com os mesmos
/// pontos (+0x19), a soma do confronto direto contra ele (entrada de +5: bytes +4..+7 em +0x11..+0x14, palavras
/// +8/+0xa/+0xc em +0x15/+0x17/+0x19); ordena com 0x6834f0 e grava em +4 de cada linha a posição, que começa no
/// número de clubes e desce a cada par da minitabela que a 0x6834f0 separa. Solta a minitabela (0x9452ca). Sem
/// memória: a original (a caixa do league.cpp). Chamada pela 0x6827d0 na regra 4.
unsafe extern "thiscall" fn build_h2h_table(liga: usize, linhas: usize) {
    let n = || cpa1_w(liga + 0x3e);
    let calloc: unsafe extern "C" fn(i32, u32) -> usize = core::mem::transmute(0x945501usize);
    let t = calloc(n() as i32, CPA1_LINHA as u32);
    if t == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%)(liga, linhas); // caixa: só a original
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

unsafe extern "thiscall" fn comparar_build_h2h_table(liga: usize, linhas: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%);
    if linhas == 0 {
        return o(liga, linhas);
    }
    let reg = [(linhas, CPA1_LINHA * cpa1_w(liga + 0x3e).max(0) as usize)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(liga, linhas), || build_h2h_table(liga, linhas));
    PLACAR.registrar(igual, || format!("build_h2h_table({:x}, {:x}) {:x?} x {:x?}", liga, linhas, mo, mr));
}

/// { 0x687970 } (thiscall, league.cpp): `League::free_table(tabela)`: `tabela` nula = a da liga (+0xb1). Com clubes
/// (+0x3e ≥ 1) e tabela: solta (0x9452ca) os vetores +5 de cada linha (se o da primeira não é nulo), depois os +9
/// (idem) e a tabela; sendo a da liga, zera +0x3e e +0xb1. Sem SEH, sem sorteio.
unsafe extern "thiscall" fn free_table(liga: usize, tabela: usize) {
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
unsafe extern "thiscall" fn comparar_free_table(liga: usize, tabela: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%)(liga, tabela)
}

/// { 0x689ad0 } (thiscall, league.cpp): `League::check_complete(fases, avisar)`: 0 sem tabela, encerrada (+0xb5) ou
/// sem +0x45. Fora da fase única (+0x44 ≠ 0xff) já completa (+0xdd): 1. Não completa: acerta o total de rodadas
/// +0xa7 (−1) e, se algum clube ainda não jogou todas (+0x11 < +0xa7), 0; senão, nas divisões de liga (0x4b0700)
/// com +0xa9 > 8 e sem +0xe6, os registros de fim de liga (0x86a070), e sem +0xe6 os prêmios (0x682e10). Com
/// `fases` = 1 na fase única: cada fase de +0xc (0..=+0x30) tem de estar completa (virtual +0x10 com 0, 1) e
/// +0x30 ≥ +0x2c − 1, senão 0. Marca +0xdd (antes, com `avisar`, a virtual +0x94) e devolve 1. Sem SEH, sem sorteio.
unsafe extern "thiscall" fn league_check_complete(liga: usize, fases: u32, avisar: u32) -> u8 {
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
unsafe fn is_complete_check_read_only(liga: usize, fases: u32) -> bool {
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

unsafe extern "thiscall" fn comparar_league_check_complete(liga: usize, fases: u32, avisar: u32) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u8>(%%I3%%);
    if !is_complete_check_read_only(liga, fases) {
        return o(liga, fases, avisar); // fim de liga (registros, prêmios, fases, aviso): só a original
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(liga + 0xa7, 2)], || o(liga, fases, avisar), || league_check_complete(liga, fases, avisar));
    PLACAR.registrar(igual && r == n, || format!("league_check_complete({:x}, {}, {}) {}/{} {:x?} x {:x?}", liga, fases, avisar, r, n, mo, mr));
    r
}
