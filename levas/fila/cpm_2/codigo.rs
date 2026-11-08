// ---------- %%LEVA%%a leva: a tabela da liga: iniciar a linha do clube, o vetor dos clubes e as posições finais (league.cpp) ----------

unsafe fn cpm2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpm2_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

unsafe fn cpm2_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpm2_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// Linhas da tabela: 0x3b bytes (+0 clube, +4 posição, +5 confrontos diretos de 14 bytes, +9 histórico das posições,
/// +0x11/+0x1f/+0x2d os totais, em casa e fora, +0x37 situação, +0x38 marca, +0x39, +0x3a clubes).
const CPM2_LINHA: usize = 0x3b;

/// A regra 4 (confrontos diretos) entre as 4 regras de desempate (+0xc5, até a primeira 0).
unsafe fn uses_head_to_head_rule(this: usize) -> bool {
    for k in 0..4 {
        match cpm2_b(this + 0xc5 + k) {
            0 => return false,
            4 => return true,
            _ => {}
        }
    }
    false
}

/// { 0x687430 } (thiscall, ret 0x10, league.cpp): `League::init_table_row(i, clube, marca, linhas)`: põe o clube na
/// linha `i` (char) da tabela `linhas` (nula: a da liga, +0xb1): o clube com situação +0x64 = 3 passa a 2; o total de
/// rodadas +0xa7 ainda −1 vira (clubes +0x3e − 1) × turnos +0x3c (mais de 50 clubes: caixa e segue); a linha recebe o
/// clube, a posição `i`, os três blocos de totais zerados, a marca (+0x38), situação 0xff, 1 em +0x39 e os clubes
/// (+0x3a); com turnos, o histórico das posições (+9, +0xa7 bytes; sem memória, caixa e fim); com a regra 4, os
/// confrontos diretos (+5, 14 bytes por adversário, zerados). Na última linha com confrontos, cada linha recebe os
/// adversários na ordem da tabela.
unsafe extern "thiscall" fn init_table_row(this: usize, i: u32, clube: usize, marca: u32, linhas: usize) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    if clube != 0 && cpm2_b(clube + 0x64) == 3 {
        *((clube + 0x64) as *mut u8) = 2;
    }
    if cpm2_w(this + 0xa7) == -1 && cpm2_w(this + 0x3e) > 0 {
        if cpm2_w(this + 0x3e) > 0x32 {
            show_error_at(0x9f15f4, 0xfe9); // mais de 50 clubes: a caixa, e segue
        }
        let total = (cpm2_w(this + 0x3e) - 1).wrapping_mul(cpm2_w(this + 0x3c));
        core::ptr::write_unaligned((this + 0xa7) as *mut i16, total);
    }
    let tab = if linhas == 0 { cpm2_d(this + 0xb1) } else { linhas };
    let idx = i as u8 as i8 as isize;
    let l = tab.wrapping_add((idx * CPM2_LINHA as isize) as usize);
    cpm2_por(l, clube);
    *((l + 4) as *mut u8) = i as u8;
    for k in 0..3 {
        let e = l + 0x11 + 14 * k;
        cpm2_por(e - 4, 0);
        core::ptr::write_bytes(e as *mut u8, 0, 4);
        core::ptr::write_bytes((e + 4) as *mut u8, 0, 6);
    }
    *((l + 0x38) as *mut u8) = marca as u8;
    *((l + 0x37) as *mut u8) = 0xff;
    *((l + 0x39) as *mut u8) = 1;
    *((l + 0x3a) as *mut u8) = cpm2_b(this + 0x3e);
    cpm2_por(l + 5, 0);
    if cpm2_w(this + 0x3c) > 0 {
        let p = alocar(cpm2_w(this + 0xa7) as i32 as u32);
        cpm2_por(l + 9, p);
        if p == 0 {
            show_error_at(0x9f15f4, 0x100c);
            return;
        }
    } else {
        cpm2_por(l + 9, 0);
    }
    if uses_head_to_head_rule(this) {
        let n = cpm2_w(this + 0x3e) as i32;
        cpm2_por(l + 5, alocar((n * 14 - 14) as u32));
        // os confrontos zerados (+4..+0xd de cada um; a original grava o +4 duas vezes, sem conferir a memória)
        let mut k = 0i32;
        while k < cpm2_w(this + 0x3e) as i32 - 1 {
            let e = cpm2_d(l + 5) + 14 * k as usize;
            core::ptr::write_bytes((e + 4) as *mut u8, 0, 10);
            k += 1;
        }
    }
    let n = cpm2_w(this + 0x3e) as isize;
    if idx == n - 1 && cpm2_d(l + 5) != 0 {
        crate::comparacao::ramo("cpm_2 confrontos diretos");
        let mut a = 0isize;
        while a < cpm2_w(this + 0x3e) as isize {
            let mut k = 0usize;
            let mut j = 0isize;
            while j < cpm2_w(this + 0x3e) as isize {
                if a != j {
                    let p = cpm2_d(tab + a as usize * CPM2_LINHA + 5);
                    cpm2_por(p + 14 * k, cpm2_d(tab + j as usize * CPM2_LINHA)); // o clube adversário
                    k += 1;
                }
                j += 1;
            }
            a += 1;
        }
    }
}

/// { 0x687a20 } (thiscall, ret 8, league.cpp): `League::get_club_array(saida, n)`: um vetor novo (malloc 0x944e46) com
/// os clubes das linhas da tabela (+0xb1) em `*saida` e a conta (+0x3e) em `*n`; sem memória, caixa e `*n` = 0.
unsafe extern "thiscall" fn league_get_club_array(this: usize, saida: *mut usize, n: *mut i16) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let p = alocar((cpm2_w(this + 0x3e) as i32 * 4) as u32);
    *saida = p;
    if p == 0 {
        show_error_at(0x9f15f4, 0x10cc);
        *n = 0;
        return;
    }
    crate::comparacao::ramo("cpm_2 vetor dos clubes");
    let mut i = 0i16;
    while i < cpm2_w(this + 0x3e) {
        cpm2_por(p + 4 * i as usize, cpm2_d(cpm2_d(this + 0xb1) + i as usize * CPM2_LINHA));
        i += 1;
    }
    *n = cpm2_w(this + 0x3e); // a conta
}

/// { 0x68aa30 } (thiscall, league.cpp): `League::record_final_positions()`: cada clube da tabela (+0xb1, na ordem) que
/// ainda está na divisão da liga (+0x57 = +4 dela) guarda a divisão como a da última temporada (+0x5b) e a posição
/// (linha + 1) em +0x5f.
unsafe extern "thiscall" fn record_final_positions(this: usize) {
    let mut i = 0i16;
    while i < cpm2_w(this + 0x3e) {
        let divisao = cpm2_d(this + 4);
        let c = cpm2_d(cpm2_d(this + 0xb1) + i as usize * CPM2_LINHA);
        if cpm2_d(c + 0x57) == divisao {
            crate::comparacao::ramo("cpm_2 posicao final");
            cpm2_por(c + 0x5b, divisao);
            *((c + 0x5f) as *mut u8) = (i as u8).wrapping_add(1); // a posição final
        }
        i += 1;
    }
}

/// Comparador: a original aloca (histórico e confrontos): roda só ela e confere a linha com o plano feito antes (o
/// total de rodadas, a situação do clube, os campos, os blocos zerados, os vetores presentes e, na última linha, os
/// adversários de cada linha).
unsafe extern "thiscall" fn comparar_init_table_row(this: usize, i: u32, clube: usize, marca: u32, linhas: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize, u32, usize)>(%%I0%%);
    let a7 = cpm2_w(this + 0xa7);
    let n = cpm2_w(this + 0x3e);
    let a7_esperado = if a7 == -1 && n > 0 { (n - 1).wrapping_mul(cpm2_w(this + 0x3c)) } else { a7 };
    let situacao = if clube != 0 { cpm2_b(clube + 0x64) } else { 0 };
    let situacao_esperada = if situacao == 3 { 2 } else { situacao };
    let tab = if linhas == 0 { cpm2_d(this + 0xb1) } else { linhas };
    let confrontos = uses_head_to_head_rule(this);
    o(this, i, clube, marca, linhas);
    let idx = i as u8 as i8 as isize;
    let l = tab.wrapping_add((idx * CPM2_LINHA as isize) as usize);
    let mut ok = cpm2_d(l) == clube && cpm2_b(l + 4) == i as u8 && cpm2_b(l + 0x37) == 0xff && cpm2_b(l + 0x38) == marca as u8
        && cpm2_b(l + 0x39) == 1 && cpm2_b(l + 0x3a) == n as u8 && cpm2_w(this + 0xa7) == a7_esperado
        && (clube == 0 || cpm2_b(clube + 0x64) == situacao_esperada)
        && (cpm2_d(l + 9) != 0) == (cpm2_w(this + 0x3c) > 0) && (cpm2_d(l + 5) != 0) == confrontos;
    for k in 0..3 {
        let e = l + 0x11 + 14 * k;
        ok &= (e - 4..e + 10).all(|a| cpm2_b(a) == 0);
    }
    let ultima = idx == n as isize - 1 && cpm2_d(l + 5) != 0;
    if cpm2_d(l + 5) != 0 && !ultima {
        let p = cpm2_d(l + 5);
        ok &= (0..(n as usize).saturating_sub(1)).all(|k| (p + 14 * k + 4..p + 14 * k + 14).all(|a| cpm2_b(a) == 0));
    }
    if ultima {
        for a in 0..n as usize {
            let p = cpm2_d(tab + a * CPM2_LINHA + 5);
            let outros = (0..n as usize).filter(|&j| j != a);
            ok &= outros.enumerate().all(|(k, j)| cpm2_d(p + 14 * k) == cpm2_d(tab + j * CPM2_LINHA));
        }
    }
    PLACAR.registrar(ok, || format!("init_table_row(linha {}, clube {:x}, marca {}) clubes {}, rodadas {} (esperava {}), confrontos {} ({:x}), histórico {:x}",
        idx, clube, marca as u8, n, cpm2_w(this + 0xa7), a7_esperado, confrontos, cpm2_d(l + 5), cpm2_d(l + 9)));
}

/// Comparador: a original aloca o vetor: roda só ela e confere a conta e os clubes, na ordem da tabela.
unsafe extern "thiscall" fn comparar_league_get_club_array(this: usize, saida: *mut usize, n: *mut i16) {
    original_fn::<unsafe extern "thiscall" fn(usize, *mut usize, *mut i16)>(%%I1%%)(this, saida, n);
    let p = *saida;
    if p == 0 {
        return; // sem memória: a caixa já saiu
    }
    let conta = cpm2_w(this + 0x3e);
    let ok = *n == conta && (0..conta.max(0) as usize).all(|i| cpm2_d(p + 4 * i) == cpm2_d(cpm2_d(this + 0xb1) + i * CPM2_LINHA));
    PLACAR.registrar(ok, || format!("league_get_club_array conta {} (tabela {})", *n, conta));
}

/// Comparador: lado a lado pela divisão da última temporada e pela posição (+0x5b..+0x5f) de cada clube da tabela.
unsafe extern "thiscall" fn comparar_record_final_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    let reg: Vec<(usize, usize)> = (0..cpm2_w(this + 0x3e).max(0) as usize)
        .map(|i| (cpm2_d(cpm2_d(this + 0xb1) + i * CPM2_LINHA) + 0x5b, 5))
        .collect();
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this), || record_final_positions(this));
    PLACAR.registrar(igual, || format!("record_final_positions {:x?} x {:x?}", mo, mr));
}
