// ---------- %%LEVA%%a leva: o valor do jogador para o clube (a nota pela vaga da lista de observação e o preço que o clube pagaria), a entrada do jogador na busca do clube observado e o prêmio por jogos na seleção achado (shortlist_manager.cpp, scout_manager.cpp, transfer_manager.cpp) ----------

unsafe fn cqa7_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqa7_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqa7_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// A constante double do exe em `a` (lida do próprio exe para ser a mesma, bit a bit, que o x87 da original carrega).
unsafe fn cqa7_k(a: usize) -> f64 {
    core::ptr::read_unaligned(a as *const f64)
}

/// O fim sem valor da 0x840d80 (0x841f62): fora da lista de empréstimos da vaga (bit 0 de +0x24), 0; com ela e a pessoa
/// listada para empréstimo (0x544590 com 0), a nota vira max(nota + 5000, nota × 1,2) (_ftol) e 0.
unsafe fn loan_listed_value_end(p: usize, vaga: usize, saida: *mut i32) -> i32 {
    if cqa7_b(vaga + 0x24) & 1 == 0 {
        return 0;
    }
    let emprestimo: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x544590usize);
    if emprestimo(p, 0) != 0 {
        let r = *saida;
        let a = r.wrapping_add(5000) as f64;
        let q = r as f64 * cqa7_k(0x968978);
        *saida = match_pl_ftol(if a > q { a } else { q });
    }
    0
}

/// O ajuste pela habilidade vista dos jogadores jovens (as vagas de tipo 1, 2 e 3 em +0x24 >> 1): com menos de 27 anos
/// (+0x18 da pessoa) e nota acima de 20000, a habilidade atual vista pelo técnico do clube (0x6e86d0 com 1 e nota/4)
/// × 220 / nota, limitada a `frac` + 0,6 e puxada para 1 aos 24, 25 e 26 anos; com 1,05 × mínimo abaixo da nota ou a
/// vaga de nota alta (`alta`), acima de 1 ela ainda vai pela reputação do clube na tabela [this+0xdc] (9 bytes por
/// clube, palavra +5) sobre a reputação do clube (+0x80, pelo menos 1000), entre 1 e 1,2. Acima de 1: `frac` × ela, e o
/// valor base e a nota × (ela + 1) / 2.
unsafe fn young_player_adjustment(this: usize, clube: usize, p: usize, saida: *mut i32, minimo: i32, alta: bool, idade: i8, r: i32, frac: &mut f64, e: &mut u32) {
    if idade >= 0x1b || r <= 20000 {
        return;
    }
    let vista: unsafe extern "C" fn(usize, usize, usize, u32, u32) -> i16 = core::mem::transmute(0x6e86d0usize);
    let sv = vista(cqa7_d(clube + 0xcf), p, clube, 1, (r / 4) as u32) as i32;
    let o = *saida as f64;
    let mut d1 = sv as f64 * cqa7_k(0x968f48) / o;
    let teto = *frac + cqa7_k(0x968e48);
    if !(d1 < teto) {
        d1 = teto;
    }
    match idade {
        0x18 => d1 = d1 * cqa7_k(0x968e40) + cqa7_k(0x968e80),
        0x19 => d1 = (d1 + cqa7_k(0x967890)) * cqa7_k(0x968968),
        0x1a => d1 = d1 * cqa7_k(0x968e80) + cqa7_k(0x968e40),
        _ => {}
    }
    if minimo as f64 * cqa7_k(0x9689b8) < o || alta {
        if !(d1 > cqa7_k(0x967890)) {
            return;
        }
        let a = cqa7_w(clube + 0x80);
        let ra = if a < 1000 { 1000 } else { a as i32 };
        let nr = cqa7_w(cqa7_d(this + 0xdc).wrapping_add(cqa7_d(clube).wrapping_mul(9)) + 5) as i32;
        let mut x = nr as f64 / ra as f64;
        if !(cqa7_k(0x967890) <= x) {
            x = cqa7_k(0x967890);
        } else if cqa7_k(0x968978) < x {
            x = cqa7_k(0x968978);
        }
        d1 = d1 * x;
    }
    if d1 > cqa7_k(0x967890) {
        crate::comparison::branch("cqa_7 jovem valorizado");
        *frac = d1 * *frac;
        let h = (d1 + cqa7_k(0x967890)) * cqa7_k(0x968968);
        *e = match_pl_ftol(*e as f64 * h) as u32;
        *saida = match_pl_ftol(o * h);
    }
}

/// { 0x840d80 } (thiscall, ret 0x18, shortlist_manager.cpp, 4,6 KB de x87; o mapa dá 1332 bytes):
/// `Shortlist_Manager::value_player_for_club(clube, pessoa, vaga, saida, mínimo, teto)`, das listas de observação e das
/// ofertas do computador (0x83f3a0, 0x842e10, 0x843800, 0x8c0810, 0x8c8090, 0x8c8860, 0x8e5d60). O observador é o
/// técnico do clube (+0xcf; sem a ficha +0x69, uma sorteada, 0x90cfc0 sobre [0xae2368], em [0xae23c0] + 0x44 × i). As
/// marcas da nota pelo tipo da vaga (+0x24 & 0x1e: 4 e 6 com 0x5882, ou 0x815882 para o próprio jogador do clube
/// humano, e o escopo da reputação 0x5408c0; 8 com 0x5982 e o escopo; senão 0x5902); a vaga da lista de empréstimos (bit
/// 0) muda o divisor do mínimo para 16 ou 6 (0x544590). Clube humano (0x600a50 com 1, 1): o técnico dele (0x53ca90) e,
/// com o jogador na lista do olheiro (0x7f7490), a nota (0x6e6340 com −1 e o substituto 0x600e90) vira max(nota +
/// 15000, nota × 1,5); senão a nota com o mínimo² × 0,000025 / divisor. Computador: com a marca 0x100 (ou 0x80) o melhor
/// observador entre o técnico e os 7 olheiros (+0x1b3) pelo conhecimento (0x544410 com 6 ou 5) menos 5 ou 10 pelo
/// escopo; a nota com o mínimo e esse observador, × 1,05 para o listado para transferência de outro clube (0x544520).
/// Nota negativa ou 10000 abaixo do mínimo, ou vaga da lista de empréstimos: o fim sem valor (`loan_listed_value_end`). A névoa
/// (+0xb do registro de 79 bytes em [0xae2d0c], 4 bits) limita a nota a nota + 5000 por (1 + n²/100) × nota. Pela
/// posição (0x83bd00) o teto do preço e a fração; o valor base (alto com nota ≥ 35000, +0xb da vaga ≥ nota/8 e +1 ≥
/// 0x80); o ajuste pelo tipo da vaga (1, 2, 3: os jovens, `young_player_adjustment`; 3, 4, 5, 6: frações); a idade acima do divisor
/// (28, 32 sem posição, 19 na vaga 5) e o estrangeiro (0x53d980); o preço pela consistência (+0x2d da ficha +0x61) e o
/// teto pelo tipo da vaga, pela reputação, pela ficha (+0xd) e pela idade; acima de 1,5 milhão, a habilidade potencial
/// vista (0x6e86d0 com 2) abaixo de 145 puxa para 1,5 milhão; o limite da divisão do clube (+0x57, +0x69 < 16); a nação
/// do clube (+0x53) [0x9cf434] ou [0x9cf300]; com `teto`, no máximo +7 da vaga (pelo menos 1). Devolve o preço (_ftol).
unsafe extern "thiscall" fn value_player_for_club(this: usize, clube: usize, p: usize, vaga: usize, saida: *mut i32, minimo: i32, teto: u32) -> i32 {
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let escopo: unsafe extern "C" fn(usize, usize) -> u8 = core::mem::transmute(0x5408c0usize);
    let conhecer: unsafe extern "C" fn(usize, i8) -> i32 = core::mem::transmute(0x544410usize);
    let emprestimo: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x544590usize);
    let nota: unsafe extern "C" fn(usize, u32, u32, usize, i32, usize) -> i32 = core::mem::transmute(0x6e6340usize);
    let mut obs = cqa7_d(clube + 0xcf);
    let t7 = cqa7_d(vaga + 7) as i32;
    let teto_i = if t7 > 1 { t7 } else { 1 };
    let mut div: i8 = 4;
    if obs != 0 && cqa7_d(obs + 0x69) == 0 {
        let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
        let i = sorteio(cqa7_d(0xae2368) as i32);
        core::ptr::write_unaligned((obs + 0x69) as *mut usize, cqa7_d(0xae23c0).wrapping_add((i as usize).wrapping_mul(0x44)));
    }
    // ponytail: a original empurra a palavra da posição com a palavra alta de ecx (lixo); quem recebe lê só a palavra
    let pos = cqa7_w(vaga + 1) as u16 as u32;
    let tipo = cqa7_b(vaga + 0x24) & 0x1e;
    let flags: u32 = if tipo == 4 || tipo == 6 {
        let mut f = 0x5882;
        if cqa7_d(vaga + 0xf) == cqa7_d(p) && cqa7_d(p + 0x39) == clube && humano(clube, 1, 1, 0, 0) != 0 {
            f = 0x815882;
        }
        match escopo(p, cqa7_d(clube + 0xcf)) as i8 {
            0 => f | 0x40,
            1 => f | 0x50,
            2 => f | 0x10,
            3 => f | 0x30,
            4 => f | 0x20,
            _ => f,
        }
    } else if tipo == 8 {
        match escopo(p, cqa7_d(clube + 0xcf)) as i8 {
            0 => 0x59c2,
            1 => 0x59d2,
            2 => 0x5992,
            3 => 0x59b2,
            4 => 0x59a2,
            _ => 0x5982,
        }
    } else {
        0x5902
    };
    if cqa7_b(vaga + 0x24) & 1 != 0 {
        div = if emprestimo(p, 0) != 0 { 0x10 } else { 6 };
    }
    // o mínimo² × 0,000025 / divisor (_ftol)
    let corte = |m: i32, div: i8| {
        let q = m as f64;
        match_pl_ftol(q * cqa7_k(0x96d240) * q / div as f64)
    };
    if humano(clube, 1, 1, 0, 0) != 0 {
        let tecnico: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53ca90usize);
        let in_list: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7f7490usize);
        let substituto: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x600e90usize);
        let m = tecnico(clube);
        if in_list(cqa7_d(0xb74448), p, m) != 0 {
            let s = substituto(clube);
            let r = nota(p, pos, flags, clube, -1, s);
            *saida = r;
            let a = r.wrapping_add(15000) as f64;
            let q = r as f64 * cqa7_k(0x968940);
            *saida = match_pl_ftol(if a > q { a } else { q });
        } else {
            let s = substituto(clube);
            let v = corte(minimo, div);
            *saida = nota(p, pos, flags, clube, v, s);
        }
    } else {
        if flags & 0x100 != 0 || flags & 0x80 != 0 {
            let nivel: i8 = if flags & 0x100 != 0 { 6 } else { 5 };
            let mut melhor: i8 = 0;
            if flags & 0x100 != 0 {
                if obs != 0 && cqa7_d(obs + 0x69) != 0 {
                    melhor = conhecer(cqa7_d(obs + 0x69), 6) as i8;
                    match escopo(p, obs) as i8 {
                        3 => melhor = melhor.wrapping_sub(5),
                        4 => melhor = melhor.wrapping_sub(10),
                        _ => {}
                    }
                }
            } else if obs != 0 {
                // a marca 0x80: o técnico entra com +5 (−5 e +0 pelo escopo 4 e 3)
                melhor = (conhecer(cqa7_d(obs + 0x69), 5) as i8).wrapping_add(5);
                match escopo(p, obs) as i8 {
                    3 => melhor = melhor.wrapping_sub(5),
                    4 => melhor = melhor.wrapping_sub(10),
                    _ => {}
                }
            }
            for k in 0..7 {
                let s = cqa7_d(clube + 0x1b3 + 4 * k);
                if s == 0 || cqa7_d(s + 0x69) == 0 {
                    continue;
                }
                let mut v = conhecer(cqa7_d(s + 0x69), nivel) as i8;
                match escopo(p, s) as i8 {
                    3 => {
                        if nivel == 6 && cqa7_w(clube + 0x80) < 0xdac {
                            continue;
                        }
                        v = v.wrapping_sub(5);
                    }
                    4 => {
                        if nivel == 6 && cqa7_w(clube + 0x80) < 0xdac {
                            continue;
                        }
                        v = v.wrapping_sub(10);
                    }
                    _ => {}
                }
                if v > melhor {
                    melhor = v;
                    obs = s;
                }
            }
        }
        let v = if minimo > -1 { corte(minimo, div) } else { -1 };
        *saida = nota(p, pos, flags, clube, v, obs);
        let transferencia: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x544520usize);
        if clube != cqa7_d(p + 0x39) && transferencia(p, 0) != 0 {
            crate::comparison::branch("cqa_7 listado para transferencia");
            *saida = match_pl_ftol(*saida as f64 * cqa7_k(0x9689b8)); // cqa7: o listado de outro clube × 1,05
        }
    }
    let r = *saida;
    if r < 0 || r < minimo.wrapping_sub(10000) || cqa7_b(vaga + 0x24) & 1 != 0 {
        return loan_listed_value_end(p, vaga, saida);
    }
    let nevoa = cqa7_b(cqa7_d(0xae2d0c).wrapping_add(cqa7_d(p).wrapping_mul(0x4f)) + 0xb) & 0xf;
    if nevoa != 0 {
        let a = r.wrapping_add(5000) as f64;
        let w = (game_pow(nevoa as f64, cqa7_k(0x968950)) * cqa7_k(0x967898) + cqa7_k(0x967890)) * r as f64;
        *saida = match_pl_ftol(if a < w { a } else { w });
    }
    div = 0x1c;
    let posicao: unsafe extern "thiscall" fn(usize, u32) -> u8 = core::mem::transmute(0x83bd00usize);
    let c = posicao(this, pos) as i8 as i32;
    let (teto_pos, mut frac) = match c {
        1..=4 | 17 => (8000000.0, 0.6),
        5..=7 | 19 => (9000000.0, 0.65),
        8..=10 | 18 => (10000000.0, 0.75),
        11..=13 | 21 => (11000000.0, 0.8),
        14 | 20 => (13000000.0, 1.0),
        _ => {
            div = 0x20;
            (6000000.0, 0.575)
        }
    };
    let r = *saida;
    let bv = cqa7_d(vaga + 0xb) as i32;
    let alta = r >= 35000 && bv >= r / 8 && cqa7_w(vaga + 1) >= 0x80;
    let mut e = if alta {
        let v = r as f64;
        match_pl_ftol(((v * cqa7_k(0x96d238) - cqa7_k(0x967890)) * v + bv as f64 * cqa7_k(0x9689e0)) * cqa7_k(0x96f918)) as u32
    } else {
        crate::comparison::branch("cqa_7 valor base");
        match_pl_ftol((bv as f64 * cqa7_k(0x9689e0) + r as f64 * cqa7_k(0x969040)) * cqa7_k(0x96f918)) as u32 // cqa7: o valor base
    };
    if e < 1 {
        e = 1;
    }
    let idade = cqa7_b(p + 0x18) as i8;
    match (cqa7_b(vaga + 0x24) >> 1) & 0xf {
        1 | 2 => young_player_adjustment(this, clube, p, saida, minimo, alta, idade, r, &mut frac, &mut e),
        3 => {
            let k = if cqa7_w(vaga + 1) < 0x80 { cqa7_k(0x96a670) } else { cqa7_k(0x96d718) };
            e = match_pl_ftol(e as f64 * k) as u32;
            young_player_adjustment(this, clube, p, saida, minimo, alta, idade, r, &mut frac, &mut e);
        }
        4 => e = match_pl_ftol(e as f64 * cqa7_k(0x968968)) as u32,
        5 => {
            e = match_pl_ftol(e as f64 * cqa7_k(0x968a80)) as u32;
            div = div.wrapping_sub(9);
        }
        6 => e = match_pl_ftol(e as f64 * cqa7_k(0x968918)) as u32,
        _ => {}
    }
    let tipo = cqa7_b(vaga + 0x24) & 0x1e;
    if idade > div {
        let d = idade as i32 - div as i32;
        let dd = d as f64;
        let ficha = cqa7_d(p + 0x61);
        let f = cqa7_k(0x967890) / (dd * cqa7_k(0x968e80) + cqa7_k(0x9689b0)); // cqa7: a idade acima do divisor
        let ts = cqa7_w(ficha + 0xd);
        let g = if ts > 0x1770 { ts as i32 as f64 * cqa7_k(0x96a680) - cqa7_k(0x968940) } else { 0.0 };
        let estrangeira: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x53d980usize);
        let y = if estrangeira(p, cqa7_d(clube + 0x53), 1) != 0 {
            f / (dd * cqa7_k(0x968f20) + cqa7_k(0x9689d8))
        } else if idade as i32 > div as i32 + 2 && (tipo == 8 || tipo == 6) && cqa7_w(ficha + 0xb) < cqa7_w(clube + 0x80) {
            (cqa7_k(0x967890) - (d - 2) as f64 * cqa7_k(0x96d438)) * f
        } else {
            f
        };
        crate::comparison::branch("cqa_7 idade");
        let um = cqa7_k(0x967890);
        if tipo == 0xa {
            *saida = match_pl_ftol(((um - g) * y + g) * *saida as f64);
        }
        e = match_pl_ftol(((um - g) * y + g) * e as f64) as u32;
    }
    let ficha = cqa7_d(p + 0x61);
    let cons = cqa7_b(ficha + 0x2d) as i8 as f64;
    let c = match_pl_ftol((cqa7_k(0x967890) - game_pow(cons, cqa7_k(0x968950)) * cqa7_k(0x968f40)) * e as f64);
    let mut kk = (c.wrapping_mul(c) as u32) as f64 * teto_pos * cqa7_k(0x968eb8);
    let ra = cqa7_w(clube + 0x80);
    if tipo == 0xc {
        if ra > 5000 {
            let lim = (ra as i32 - 5000) as f64 * cqa7_k(0x968ea8) + cqa7_k(0x968880);
            if !(kk <= lim) {
                kk = (ra as i32 - 100000) as f64 + cqa7_k(0x968880);
            }
        } else {
            let lim = (ra as i32 * 10) as f64;
            if !(kk <= lim) {
                kk = ra as f64 * cqa7_k(0x9688b0);
            }
        }
    } else if tipo == 0xa {
        let lim = if ra > 5000 {
            let z = (ra as i32 - 5000) as f64 * cqa7_k(0x968918) + cqa7_k(0x968878);
            z * z
        } else {
            let z = ra as f64;
            z * z * cqa7_k(0x96f910)
        };
        if !(kk <= lim) {
            kk = lim;
        }
    }
    let ts = cqa7_w(ficha + 0xd);
    if ts <= 5000 {
        let lim = frac * cqa7_k(0x96f908);
        if lim < kk {
            kk = lim;
        }
    } else if ts <= 8000 {
        let lim = ts as f64 * frac * cqa7_k(0x96f900) + cqa7_k(0x96f8f8);
        if lim < kk {
            kk = lim;
        }
    } else if ts < 9000 {
        let lim = (ts as f64 * cqa7_k(0x9690b8) - cqa7_k(0x96f8f0)) * frac;
        if lim < kk {
            kk = lim;
        }
        frac = frac * cqa7_k(0x968940);
    } else {
        kk = cqa7_k(0x96d568);
        frac = frac * cqa7_k(0x968e18);
    }
    let pela_idade = match idade {
        0x10 => Some(0x96f8d8),
        0x11 => Some(0x968e90),
        0x12 | 0x1f => Some(0x96d850),
        0x13 | 0x1d => Some(0x96f8e0),
        0x1c => Some(0x96a630),
        0x1e => Some(0x96f908),
        0x20 => Some(0x9688c8),
        0x21 => Some(0x96a650),
        0x22 => Some(0x9688d8),
        0x23 => Some(0x96f8e8),
        0x24..=0x28 => Some(0x968870),
        _ => None,
    };
    if let Some(a) = pela_idade {
        let t = frac * cqa7_k(a);
        if t < kk {
            kk = t;
        }
    }
    let base = cqa7_k(0x96f8d0);
    let mut x = kk;
    if kk > base {
        let vista: unsafe extern "C" fn(usize, usize, usize, u32, u32) -> i16 = core::mem::transmute(0x6e86d0usize);
        let n2 = vista(cqa7_d(clube + 0xcf), p, clube, 2, (*saida / 4) as u32) as i32 as f64;
        if n2 < cqa7_k(0x96f8c8) {
            let mut a = n2 * cqa7_k(0x967878) - cqa7_k(0x96f8c0);
            if !(cqa7_k(0x9689a0) <= a) {
                a = cqa7_k(0x9689a0);
            }
            x = (kk - base) * a + base;
        }
    }
    let bc = cqa7_d(clube + 0x57);
    if bc != 0 && cqa7_w(bc + 0x69) < 0x10 {
        let b69 = cqa7_w(bc + 0x69) as i32;
        let ri = cqa7_w(clube + 0x80) as i32 / 500;
        let m = if ri > b69 { ri } else { b69 };
        let mut lim = m.wrapping_mul(670000);
        if lim <= 1000000 {
            lim = 1000000;
        }
        let ts = cqa7_w(ficha + 0xd) as i32;
        if ts > 8000 {
            lim = lim.wrapping_add((ts - 8000) * 10000);
        }
        let l = lim as f64;
        if !(x <= l) {
            let m2 = if cqa7_k(0x967890) < frac { cqa7_k(0x967890) } else { frac };
            x = match_pl_ftol(l * m2) as f64;
        }
    }
    // a nação do clube (+0x53 do clube: a original usa edi, que aqui já é o clube)
    let nacao = cqa7_d(cqa7_d(clube + 0x53));
    if nacao == cqa7_d(0x9cf434) {
        if x < cqa7_k(0x968e90) {
            x = x * (x * cqa7_k(0x968e88) * cqa7_k(0x968e48) + cqa7_k(0x968e80));
        } else {
            x = x * cqa7_k(0x9689d8);
        }
    } else if nacao == cqa7_d(0x9cf300) {
        x = match_pl_ftol(x * cqa7_k(0x9689b0)) as f64;
    }
    if teto != 0 {
        let c = teto_i as f64;
        if c < x {
            x = c;
        }
        x = match_pl_ftol(x) as f64;
    }
    match_pl_ftol(x)
}

/// Comparador da 0x840d80: lado a lado na nota de saída e na devolução (o resto ela só lê). O técnico do clube sem a
/// ficha +0x69 (a original sorteia uma): só a original.
unsafe extern "thiscall" fn compare_value_player_for_club(this: usize, clube: usize, p: usize, vaga: usize, saida: *mut i32, minimo: i32, teto: u32) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut i32, i32, u32) -> i32>(%%I0%%);
    let obs = cqa7_d(clube + 0xcf);
    if obs != 0 && cqa7_d(obs + 0x69) == 0 {
        return o(this, clube, p, vaga, saida, minimo, teto);
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&[(saida as usize, 4)], || o(this, clube, p, vaga, saida, minimo, teto), || value_player_for_club(this, clube, p, vaga, saida, minimo, teto));
    SCOREBOARD.record(igual && r == n, || {
        format!("value_player_for_club(clube {:x}, pessoa {:x}, vaga {:x}, mínimo {}, teto {}) original={} rust={} {:x?} x {:x?}", clube, p, vaga, minimo, teto, r, n, mo, mr)
    });
    r
}

/// { 0x7f8610 } (thiscall, ret 4, scout_manager.cpp): `SCOUT_MANAGER::find_club_search_entry(pessoa)`, das telas da
/// pessoa e da busca (0x808540, 0x813210, 0x870650, 0x87a510): sem o humano da vez ou sem o clube da pessoa (+0x39), 0.
/// A vaga do humano (byte +0 − byte de [0xae2364] + 16, 0..15; fora, a caixa da linha 0x4b5 e 0). A primeira das 10
/// buscas (0x7f6cd0) com o clube observado (+0x190) igual ao da pessoa e o registro dele (+0x170) dá a vaga da pessoa
/// nele (0x7bd700 sem reservar); nenhuma, 0.
unsafe extern "thiscall" fn find_club_search_entry(this: usize, p: usize) -> usize {
    let m = scout_manager_current_human_manager();
    if m == 0 || cqa7_d(p + 0x39) == 0 {
        return 0;
    }
    let vaga = cqa7_b(m).wrapping_sub(cqa7_b(0xae2364)).wrapping_add(0x10) as i8;
    if !(0..=15).contains(&vaga) {
        show_error_at(0xa7b094, 0x4b5);
        return 0;
    }
    let busca: unsafe extern "thiscall" fn(usize, u32, usize) -> usize = core::mem::transmute(0x7f6cd0usize);
    for i in 0..10u32 {
        let l = busca(this, i, m);
        // ponytail: a original lê +0x190 sem conferir a busca (cairia com 0); aqui a busca nula não conta
        if l == 0 || cqa7_d(l + 0x190) != cqa7_d(cqa7_d(p + 0x39)) || cqa7_d(busca(this, i, m) + 0x170) == 0 {
            continue;
        }
        let vaga_do_jogador: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x7bd700usize);
        return vaga_do_jogador(cqa7_d(busca(this, i, m) + 0x170), p, 0);
    }
    0
}

/// Comparador da 0x7f8610: pela devolução (só lê).
unsafe extern "thiscall" fn compare_find_club_search_entry(this: usize, p: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I1%%);
    let r = o(this, p);
    let n = find_club_search_entry(this, p);
    SCOREBOARD.record(r == n, || format!("find_club_search_entry(pessoa {:x}) original={:x} rust={:x}", p, r, n));
    r
}

/// { 0x8d0f00 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::find_international_appearance_money(pessoa)`,
/// das telas da pessoa (0x884ea2): sem a marca 4 da pessoa ([this+0x86e] + id × 8), 0. A busca binária (0x946464:
/// cdecl(chave, vetor [this+0x898], conta [this+0x8a0], 0xd, 0x8b4e00)) pela chave com o id da pessoa em +0, o único
/// campo que a comparação lê; achada, o registro; senão a marca 4 sai e 0.
unsafe extern "thiscall" fn find_international_appearance_money(this: usize, p: usize) -> usize {
    let m = (cqa7_d(this + 0x86e).wrapping_add(cqa7_d(p).wrapping_mul(8))) as *mut u8;
    if *m & 4 == 0 {
        return 0;
    }
    let chave = [cqa7_d(p) as u32, 0, 0, 0];
    let bsearch: unsafe extern "C" fn(*const u32, usize, usize, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let r = bsearch(chave.as_ptr(), cqa7_d(this + 0x898), cqa7_d(this + 0x8a0), 0xd, 0x8b4e00);
    if r != 0 {
        return r;
    }
    *m = (*m).wrapping_sub(4);
    0
}

/// Comparador da 0x8d0f00: lado a lado na marca da pessoa e na devolução.
unsafe extern "thiscall" fn compare_find_international_appearance_money(this: usize, p: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I2%%);
    let m = cqa7_d(this + 0x86e).wrapping_add(cqa7_d(p).wrapping_mul(8));
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&[(m, 1)], || o(this, p), || find_international_appearance_money(this, p));
    SCOREBOARD.record(igual && r == n, || format!("find_international_appearance_money(pessoa {:x}) original={:x} rust={:x} {:x?} x {:x?}", p, r, n, mo, mr));
    r
}
