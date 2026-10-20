// ---------- 196a leva: o confronto direto da liga e os termos oferecidos na transferência (league.cpp, transfer_manager.cpp) ----------

/// { 0x6834f0 } (cdecl, league.cpp): a ordem da minitabela do confronto direto (callback do qsort):
/// linha vazia empata; senão pontos (+0x19), e depois os critérios da liga ([0xb679fc]+0xc5, até 4;
/// 0 encerra): 1 saldo (+0x15 − +0x17), 2 gols pró (+0x15), 3 o byte +0x12, 5 a palavra +0x31; 4 e os
/// outros não decidem. Devolve b − a no primeiro critério que difere.
unsafe extern "C" fn ordem_do_confronto(a: usize, b: usize) -> i32 {
    if *(a as *const u32) == 0 || *(b as *const u32) == 0 {
        return 0;
    }
    let w = |p: usize, o: usize| core::ptr::read_unaligned((p + o) as *const i16) as i32;
    if w(a, 0x19) != w(b, 0x19) {
        return w(b, 0x19) - w(a, 0x19);
    }
    let criterios = *(0xb679fc as *const usize) + 0xc5;
    for k in 0..4 {
        let (x, y) = match *((criterios + k) as *const i8) {
            0 => return 0,
            1 => (w(a, 0x15) - w(a, 0x17), w(b, 0x15) - w(b, 0x17)),
            2 => (w(a, 0x15), w(b, 0x15)),
            3 => (*((a + 0x12) as *const i8) as i32, *((b + 0x12) as *const i8) as i32),
            5 => (w(a, 0x31), w(b, 0x31)),
            _ => continue,
        };
        if x != y {
            return y - x;
        }
    }
    0
}

/// O registro de 14 bytes do confronto da linha `linha` contra o adversário `adversario` (a linha da
/// tabela tem 59 bytes; em +5 o ponteiro dos confrontos).
unsafe fn registro_do_confronto(tabela: usize, linha: i32, adversario: i32) -> usize {
    let confrontos = core::ptr::read_unaligned((tabela as isize + linha as isize * 59 + 5) as *const usize);
    (confrontos as isize + adversario as isize * 14) as usize
}

/// { 0x6869a0 } (thiscall, ret 0x14, league.cpp): soma um jogo ao confronto direto: jogos (+4), vitórias
/// (+5, com os pontos por vitória da liga +0xc2), empates (+6, pontos por empate +0xc3) ou derrotas (+7),
/// gols pró (+8) e contra (+0xa); os pontos em +0xc. Linha ou adversário negativo: nada.
unsafe extern "thiscall" fn atualizar_confronto_direto(liga: usize, tabela: usize, linha: i32, adversario: i32, pro: i8, contra: i8) {
    if linha < 0 || adversario < 0 {
        return;
    }
    let e = registro_do_confronto(tabela, linha, adversario);
    let byte = |o: usize| (e + o) as *mut u8;
    let palavra = |o: usize| (e + o) as *mut i16;
    let somar = |o: usize, v: i16| core::ptr::write_unaligned(palavra(o), core::ptr::read_unaligned(palavra(o)).wrapping_add(v));
    *byte(4) = (*byte(4)).wrapping_add(1);
    if pro > contra {
        *byte(5) = (*byte(5)).wrapping_add(1);
        somar(0xc, *((liga + 0xc2) as *const i8) as i16);
    } else if pro == contra {
        *byte(6) = (*byte(6)).wrapping_add(1);
        somar(0xc, *((liga + 0xc3) as *const i8) as i16);
    } else {
        *byte(7) = (*byte(7)).wrapping_add(1);
    }
    somar(8, pro as i16);
    somar(0xa, contra as i16);
}

/// { 0x8c1f80 } (thiscall, ret 4, transfer_manager.cpp): marca que a oferta `k` (fora de empréstimo,
/// +0x1f ≠ 1) teve os termos oferecidos ao jogador: o estado +0x2c vira 8 se era 6, senão 7. Índice
/// fora da lista ([+4] ofertas): a original (caixa de erro).
unsafe extern "thiscall" fn marcar_termos_oferecidos(gerente: usize, k: i32) {
    if k < 0 || k >= *((gerente + 4) as *const i32) {
        return orig::<unsafe extern "thiscall" fn(usize, i32)>(462)(gerente, k);
    }
    let oferta = *((*(gerente as *const usize) as isize + k as isize * 4) as *const usize);
    if oferta != 0 && *((oferta + 0x1f) as *const u8) != 1 {
        let estado = (oferta + 0x2c) as *mut u8;
        *estado = (*estado == 6) as u8 + 7;
    }
}

unsafe extern "C" fn comparar_ordem_do_confronto(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(460)(a, b);
    let n = ordem_do_confronto(a, b);
    PLACAR.registrar(r == n, || format!("ordem_do_confronto original={} rust={}", r, n));
    r
}

unsafe extern "thiscall" fn comparar_atualizar_confronto_direto(liga: usize, tabela: usize, linha: i32, adversario: i32, pro: i8, contra: i8) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, i32, i32, i8, i8)>(461);
    if linha < 0 || adversario < 0 {
        return o(liga, tabela, linha, adversario, pro, contra);
    }
    let e = registro_do_confronto(tabela, linha, adversario);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(e, 14)], || o(liga, tabela, linha, adversario, pro, contra),
        || atualizar_confronto_direto(liga, tabela, linha, adversario, pro, contra));
    PLACAR.registrar(igual, || format!("atualizar_confronto_direto({}, {}, {}x{}) {:x?} x {:x?}", linha, adversario, pro, contra, mo, mr));
}

unsafe extern "thiscall" fn comparar_marcar_termos_oferecidos(gerente: usize, k: i32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32)>(462);
    if k < 0 || k >= *((gerente + 4) as *const i32) {
        return o(gerente, k);
    }
    let oferta = *((*(gerente as *const usize) as isize + k as isize * 4) as *const usize);
    if oferta == 0 {
        return o(gerente, k);
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(oferta + 0x2c, 1)], || o(gerente, k), || marcar_termos_oferecidos(gerente, k));
    PLACAR.registrar(igual, || format!("marcar_termos_oferecidos({}) {:x?} x {:x?}", k, mo, mr));
}
