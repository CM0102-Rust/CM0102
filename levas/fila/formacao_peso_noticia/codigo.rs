// ---------- %%LEVA%%a leva: a formacao da tabela tatica, o peso das competicoes do jogador e o dado da noticia ----------

/// { 0x896570 } (thiscall, ret 8, tactics.cpp): o registro (0x18e3 bytes) da formação `f`: as
/// [+0x5fd] primeiras em [+0x5f9], as seguintes (até [+0x5f5] mais, só sem `humano`) em [+0x5f1].
/// Formação negativa ou fora: a original (caixa de erro).
unsafe extern "thiscall" fn formation_record(t: usize, f: i16, humano: u32) -> usize {
    let primeiras = core::ptr::read_unaligned((t + 0x5fd) as *const i32);
    let limite = if humano != 0 { primeiras } else { core::ptr::read_unaligned((t + 0x5f5) as *const i32).wrapping_add(primeiras) };
    let f = f as i32;
    if f < 0 || f >= limite {
        return orig::<unsafe extern "thiscall" fn(usize, i16, u32) -> usize>(%%I0%%)(t, f as i16, humano);
    }
    if f < primeiras {
        core::ptr::read_unaligned((t + 0x5f9) as *const usize).wrapping_add(f as usize * 0x18e3)
    } else {
        core::ptr::read_unaligned((t + 0x5f1) as *const usize).wrapping_add((f - primeiras) as usize * 0x18e3)
    }
}

/// { 0x7c6930 } (stdcall, ret 8, player_stats.cpp): o peso da competição `cod` para o jogador `p`
/// pelas duas competições do clube (+0x1a e +0x1e): nas quatro competições especiais ([0x9cf4a8]
/// com 0x16/0x20, [0x9cf408] com 0x1e/0x20, [0x9cf2c0] com 0x16/0x1e, [0x9cf244] com 7) e nos códigos
/// +0x78/+0x79 da competição: 500/1000/500/300 pela primeira, 300/500/300/100 pela segunda; sem
/// competição, 0. Devolve só `ax`.
unsafe extern "stdcall" fn competition_weight(p: usize, cod: i8) -> u16 {
    let k = |a: usize| *(a as *const i32);
    // como na original: a primeira constante igual ao id decide (sem o código certo, vai aos códigos)
    let especial = |id: i32| -> u8 {
        if id == k(0x9cf4a8) {
            (cod == 0x16 || cod == 0x20) as u8
        } else if id == k(0x9cf408) {
            (cod == 0x1e || cod == 0x20) as u8
        } else if id == k(0x9cf2c0) {
            (cod == 0x16 || cod == 0x1e) as u8
        } else if id == k(0x9cf244) && cod == 7 {
            2
        } else {
            0
        }
    };
    let c1 = core::ptr::read_unaligned((p + 0x1a) as *const usize);
    if c1 == 0 {
        return 0;
    }
    match especial(k(c1)) {
        1 => return 500,
        2 => return 1000,
        _ => {}
    }
    if *((c1 + 0x78) as *const i8) == cod {
        return 500;
    }
    if *((c1 + 0x79) as *const i8) == cod {
        return 300;
    }
    let c2 = core::ptr::read_unaligned((p + 0x1e) as *const usize);
    if c2 == 0 {
        return 0;
    }
    match especial(k(c2)) {
        1 => return 300,
        2 => return 500,
        _ => {}
    }
    if *((c2 + 0x78) as *const i8) == cod {
        return 300;
    }
    if *((c2 + 0x79) as *const i8) == cod { 100 } else { 0 }
}

/// { 0x785790 } (thiscall, ret 8, news.cpp): grava `valor` no dado `k` 0..49 da notícia (+5, 4 bytes
/// cada). Devolve 1. `k` fora: a original (caixa de erro).
unsafe extern "thiscall" fn set_news_field(t: usize, k: i8, valor: u32) -> u32 {
    if !(0..0x32).contains(&k) {
        return orig::<unsafe extern "thiscall" fn(usize, i8, u32) -> u32>(%%I2%%)(t, k, valor);
    }
    core::ptr::write_unaligned((t + 5 + 4 * k as usize) as *mut u32, valor);
    1
}

comparar_valor!(comparar_formation_record, formation_record, %%I0%%, (f: i16, humano: u32) -> usize);

unsafe extern "stdcall" fn comparar_competition_weight(p: usize, cod: i8) -> u16 {
    let r = orig::<unsafe extern "stdcall" fn(usize, i8) -> u16>(%%I1%%)(p, cod);
    let n = competition_weight(p, cod);
    PLACAR.registrar(r == n, || format!("competition_weight({:#x}, {}) original={} rust={}", p, cod, r, n));
    r
}

unsafe extern "thiscall" fn comparar_set_news_field(t: usize, k: i8, valor: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, i8, u32) -> u32>(%%I2%%);
    let reg = if (0..0x32).contains(&k) { vec![(t + 5 + 4 * k as usize, 4)] } else { vec![] };
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(t, k, valor), || set_news_field(t, k, valor));
    PLACAR.registrar(igual && r == v, || format!("set_news_field({:#x}, {}, {:#x}) original={} {:x?} rust={} {:x?}", t, k, valor, r, mo, v, mr));
    r
}
