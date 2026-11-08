// ---------- %%LEVA%%a leva: o resultado na tabela da liga e os pontos do resultado (league.cpp) ----------

unsafe fn cpe2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn increment_u8(a: usize) {
    *(a as *mut u8) = (*(a as *const u8)).wrapping_add(1);
}

unsafe fn league_add_i16(a: usize, v: i16) {
    core::ptr::write_unaligned(a as *mut i16, core::ptr::read_unaligned(a as *const i16).wrapping_add(v));
}

/// Linhas da tabela: 0x3b bytes; +0 clube, +5 confrontos (entradas de 14), +0x11..+0x1a os totais, +0x1f..+0x28 em
/// casa (jogos, vitórias, empates, derrotas, gols pró, contra, pontos), +0x2d..+0x36 fora.
const RESULT_TABLE_ROW_SIZE: usize = 0x3b;

/// { 0x685f60 } (thiscall, ret 0x18, league.cpp): `League::apply_result_to_table(partida, tabela, n, marcar,
/// pts_casa*, pts_fora*)`: acha as linhas do mandante (+0x1c) e do visitante (+0x20) entre as `n` (curto) da tabela
/// (para no primeiro clube que não é nenhum dos dois depois de achar ambos) e soma o jogo: jogos (+0x1f/+0x2d), gols
/// (+0x23/+0x25 e +0x31/+0x33: o placar de +0x47/+0x48 quando ≠ 0xff, senão o de +0x43/+0x44). O resultado pelo placar
/// +0x43/+0x44; empatado: pela prorrogação (+0x38 bit 2, +0x47/+0x48; `marcar` grava 3 em +0x3d/+0x3e) e pelos pênaltis
/// (bits 0x20/1, +0x49/+0x4a; vitória do mandante conta como empate dele; 5 com o 0x20, senão 4); empate puro com os
/// pontos de empate da liga (+0xc3). Os pontos de vitória pela virtual +0x90 (0x68aad0) da competição de tipo 2
/// ([0xadadfc], +0x43) ou da própria liga, lida a cada chamada. Refaz os totais das duas linhas e devolve os pontos
/// (+0x19) em `pts_casa`/`pts_fora` (0 sem a linha). Com confrontos na primeira linha (+5), acha o adversário em cada
/// uma e soma o confronto (0x6869a0). Sem SEH, sem sorteio, sem caixa, locais todas gravadas antes de lidas.
unsafe extern "thiscall" fn apply_result_to_table(liga: usize, p: usize, tab: usize, n: u32, marcar: u32, pts_casa: *mut i16, pts_fora: *mut i16) {
    let mut pontuador = liga;
    let id = cpe2_d(liga + 4);
    if id != 0 && cpe2_d(0xadadfc) != 0 {
        let comp = cpe2_d(cpe2_d(0xadadfc).wrapping_add(cpe2_d(id).wrapping_mul(4)));
        if *((comp + 0x43) as *const u8) == 2 {
            pontuador = comp;
        }
    }
    let n = n as i16;
    let linha = |k: i16| tab.wrapping_add(RESULT_TABLE_ROW_SIZE.wrapping_mul(k as i32 as usize));
    let (mut casa, mut fora): (i16, i16) = (-1, -1);
    let mut k: i16 = 0;
    if n > 0 {
        loop {
            let c = cpe2_d(linha(k));
            if c == cpe2_d(p + 0x1c) {
                casa = k;
            } else if c == cpe2_d(p + 0x20) {
                fora = k;
            } else if casa != -1 && fora != -1 {
                break;
            }
            k = k.wrapping_add(1);
            if k >= n {
                break;
            }
        }
    }
    let pb = |o: usize| *((p + o) as *const i8);
    let noventa = || *((p + 0x47) as *const u8) != 0xff;
    if casa > -1 {
        let r = linha(casa);
        increment_u8(r + 0x1f);
        let (pro, contra) = if noventa() { (pb(0x47), pb(0x48)) } else { (pb(0x43), pb(0x44)) };
        league_add_i16(r + 0x23, pro as i16);
        league_add_i16(r + 0x25, contra as i16);
    }
    if fora > -1 {
        let r = linha(fora);
        increment_u8(r + 0x2d);
        let (pro, contra) = if noventa() { (pb(0x48), pb(0x47)) } else { (pb(0x44), pb(0x43)) };
        league_add_i16(r + 0x31, pro as i16);
        league_add_i16(r + 0x33, contra as i16);
    }
    let pontos = |clube: usize| -> i16 {
        let f: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(cpe2_d(cpe2_d(pontuador) + 0x90));
        f(pontuador, clube, p) as u8 as i8 as i16
    };
    // vence o mandante: +0x20 e os pontos (+0x27) dele, derrota (+0x30) do visitante; perde: +0x22 e +0x2e com os pontos (+0x35)
    let vence_casa = |vitoria: usize| {
        if casa > -1 {
            let r = linha(casa);
            increment_u8(r + vitoria);
            league_add_i16(r + 0x27, pontos(cpe2_d(p + 0x1c))); // os pontos do mandante
        }
        if fora > -1 {
            increment_u8(linha(fora) + 0x30);
        }
    };
    let vence_fora = |vitoria: usize| {
        if casa > -1 {
            increment_u8(linha(casa) + 0x22);
        }
        if fora > -1 {
            let r = linha(fora);
            increment_u8(r + vitoria);
            league_add_i16(r + 0x35, pontos(cpe2_d(p + 0x20)));
        }
    };
    let (gc, gf) = (pb(0x43), pb(0x44));
    let flags = core::ptr::read_unaligned((p + 0x38) as *const u16);
    let penalties = if *((p + 0x38) as *const u8) & 0x20 != 0 { 5u8 } else { 4 };
    if gc > gf {
        vence_casa(0x20);
    } else if gc < gf {
        vence_fora(0x2e);
    } else if flags & 2 != 0 && pb(0x47) > pb(0x48) {
        vence_casa(0x20);
        if marcar != 0 {
            *((p + 0x3d) as *mut u8) = 3;
        }
    } else if flags & 2 != 0 && pb(0x47) < pb(0x48) {
        vence_fora(0x2e);
        if marcar != 0 {
            *((p + 0x3e) as *mut u8) = 3;
        }
    } else if flags & 0x21 != 0 && pb(0x49) > pb(0x4a) {
        vence_casa(0x21);
        if marcar != 0 {
            *((p + 0x3d) as *mut u8) = penalties;
        }
    } else if flags & 0x21 != 0 && pb(0x49) < pb(0x4a) {
        vence_fora(0x2f);
        if marcar != 0 {
            *((p + 0x3e) as *mut u8) = penalties;
        }
    } else if flags & 0x21 == 0 {
        let empate = *((liga + 0xc3) as *const i8) as i16; // o empate puro
        if casa > -1 {
            increment_u8(linha(casa) + 0x21);
            league_add_i16(linha(casa) + 0x27, empate);
        }
        if fora > -1 {
            increment_u8(linha(fora) + 0x2f);
            league_add_i16(linha(fora) + 0x35, empate);
        }
    }
    let totais = |r: usize| -> i16 {
        for (t, f, c) in [(0x11usize, 0x2d, 0x1f), (0x12, 0x2e, 0x20), (0x13, 0x2f, 0x21), (0x14, 0x30, 0x22)] {
            *((r + t) as *mut u8) = (*((r + f) as *const u8)).wrapping_add(*((r + c) as *const u8));
        }
        for (t, f, c) in [(0x15usize, 0x31, 0x23), (0x17, 0x33, 0x25), (0x19, 0x35, 0x27)] {
            let v = core::ptr::read_unaligned((r + f) as *const i16).wrapping_add(core::ptr::read_unaligned((r + c) as *const i16));
            core::ptr::write_unaligned((r + t) as *mut i16, v);
        }
        core::ptr::read_unaligned((r + 0x19) as *const i16)
    };
    for (k, pts) in [(casa, pts_casa), (fora, pts_fora)] {
        let v = if k > -1 { totais(linha(k)) } else { 0 };
        if !pts.is_null() {
            core::ptr::write_unaligned(pts, v);
        }
    }
    if cpe2_d(tab + 5) != 0 {
        let m = n as i32 - 1;
        let (mut ia, mut ib) = (-1i32, -1i32);
        let entrada = |linha_: i16, k: i16| cpe2_d(cpe2_d(linha(linha_) + 5).wrapping_add(14 * k as i32 as usize));
        let mut k: i16 = 0;
        if m > 0 {
            loop {
                if casa > -1 && entrada(casa, k) == cpe2_d(p + 0x20) {
                    ia = k as i32;
                    if ib > -1 {
                        break;
                    }
                }
                if fora > -1 && entrada(fora, k) == cpe2_d(p + 0x1c) {
                    ib = k as i32;
                    if ia > -1 {
                        break;
                    }
                }
                k = k.wrapping_add(1);
                if k as i32 >= m {
                    break;
                }
            }
        }
        let confronto: unsafe extern "thiscall" fn(usize, usize, i32, i32, i8, i8) = core::mem::transmute(0x6869a0usize);
        confronto(liga, tab, casa as i32, ia, gc, gf);
        confronto(liga, tab, fora as i32, ib, gf, gc);
    }
}

/// As regiões da 0x685f60: as `n` linhas, os confrontos de cada linha ((n − 1) entradas de 14, se a primeira os tem),
/// +0x3d/+0x3e da partida e os dois pontos devolvidos.
unsafe fn result_update_regions(p: usize, tab: usize, n: u32, pts_casa: *mut i16, pts_fora: *mut i16) -> Vec<(usize, usize)> {
    let n = (n as i16).max(0) as usize;
    let mut reg = vec![(p + 0x3d, 2)];
    if tab != 0 && n > 0 {
        reg.push((tab, RESULT_TABLE_ROW_SIZE * n));
        if cpe2_d(tab + 5) != 0 && n > 1 {
            for i in 0..n {
                let h = cpe2_d(tab + RESULT_TABLE_ROW_SIZE * i + 5);
                if h != 0 {
                    reg.push((h, 14 * (n - 1)));
                }
            }
        }
    }
    for pts in [pts_casa, pts_fora] {
        if !pts.is_null() {
            reg.push((pts as usize, 2));
        }
    }
    reg
}

unsafe extern "thiscall" fn comparar_apply_result_to_table(liga: usize, p: usize, tab: usize, n: u32, marcar: u32, pts_casa: *mut i16, pts_fora: *mut i16) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, *mut i16, *mut i16)>(%%I0%%);
    let reg = result_update_regions(p, tab, n, pts_casa, pts_fora);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(liga, p, tab, n, marcar, pts_casa, pts_fora),
        || apply_result_to_table(liga, p, tab, n, marcar, pts_casa, pts_fora));
    PLACAR.registrar(igual, || format!("apply_result_to_table({:x}, partida {:x}, {:x}, n {}, marcar {}) {:x?} x {:x?}",
        liga, p, tab, n as i16, marcar, mo, mr));
}

/// { 0x68aad0 } (thiscall, ret 8, league.cpp): `League::points_for_result(clube, partida)`: os pontos de vitória
/// (+0xc2) se o clube venceu (mandante +0x1c com +0x43 > +0x44, visitante +0x20 com menos), senão 0; empatado: pela
/// prorrogação (+0xab bit 2, +0x47/+0x48, os de vitória), pelos pênaltis (bits 0x21, +0x49/+0x4a: os de empate +0xc3
/// a quem venceu, 0 a quem perdeu) ou os de empate. Só o al é escrito: os 3 bytes altos do eax são os da partida.
unsafe extern "thiscall" fn points_for_result(liga: usize, clube: usize, p: usize) -> u32 {
    let pb = |o: usize| *((p + o) as *const i8);
    let (casa, fora) = (cpe2_d(p + 0x1c), cpe2_d(p + 0x20));
    let venceu = |a: i8, b: i8| (clube == casa && a > b) || (clube == fora && a < b);
    let ganho = |o: usize| *((liga + o) as *const u8);
    let flags = *((liga + 0xab) as *const u8);
    let v = if pb(0x43) != pb(0x44) {
        if venceu(pb(0x43), pb(0x44)) { ganho(0xc2) } else { 0 } // os pontos da vitória
    } else if flags & 2 != 0 && pb(0x47) != pb(0x48) {
        if venceu(pb(0x47), pb(0x48)) { ganho(0xc2) } else { 0 }
    } else if flags & 0x21 != 0 && pb(0x49) != pb(0x4a) {
        if venceu(pb(0x49), pb(0x4a)) { ganho(0xc3) } else { 0 }
    } else {
        ganho(0xc3)
    };
    (p as u32 & 0xffff_ff00) | v as u32
}

comparar_valor!(comparar_points_for_result, points_for_result, %%I1%%, (clube: usize, p: usize) -> u32);
