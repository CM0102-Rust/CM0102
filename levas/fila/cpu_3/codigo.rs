// ---------- %%LEVA%%a leva: as estatísticas do jogador: a estatística de uma categoria e as notas combinadas, a ordem dos idiomas e a ordem da história por clube (player_stats.cpp) ----------

unsafe fn cpu3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpu3_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// { 0x7c1e30 } (thiscall, ret 0x10, player_stats.cpp): `PlayerStats::get_stat(jogador, categoria, qual, padrão)`, como
/// a original a deixa no x87. Sem jogador: a caixa "Error" (linha 0x659) e 0.0; id (+0) fora da tabela ([0xae236c]):
/// 0.0. Com `qual` ≠ 0x13 e a categoria (byte com sinal) até 6: o registro da categoria (0x7c3920) dá a estatística
/// (0x7c2150 com `qual` e `padrão`); sem registro, `padrão` para 0x12 e 0x14 e 0.0 para as outras. Senão as notas
/// combinadas pelas categorias 2+3 (jogos n e total t de +0xe: n < 20, ((20 − n)·5 + t)·0.05f; senão t/n; guardado em
/// f32), 4 (n < 8, ((8 − n)·3 + t)·0.125; senão t/n; f32) e 5 (n < 5, ((5 − n) + t)·0.2f; senão t/n; no x87): categoria
/// 100, (c5·1.5 + c4·2.5 + c23·6)·0.1; 101, (c5·3 + c4·5 + 2·c23)·0.1; 102, (c5·5 + c4·4 + c23)·0.1; as outras 0.0. Sem
/// o registro 4 ou 5 a original lê locais sem valor (a cópia usa zero; o comparador deixa esses casos com a original).
/// Pela escolha da escalação 0x6e2f20, pelo MATCH_PLAYER::setup 0x6eaea0, pela reputação do clube 0x545e90, pela força
/// do jogador 0x546ab0, pelo dia da comissão 0x5ce2d0 e por outras. Quente: sem contador.
unsafe extern "thiscall" fn get_stat(this: usize, jogador: usize, categoria: u32, qual: u32, padrao: f32) -> f64 {
    if jogador == 0 {
        show_error_at(0xa78310, 0x659);
        return 0.0;
    }
    if cpu3_d(jogador) as i32 >= cpu3_d(0xae236c) as i32 {
        return 0.0;
    }
    let registro: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x7c3920usize);
    if qual as u8 != 0x13 && categoria as u8 as i8 <= 6 {
        let r = registro(this, jogador, categoria);
        if r != 0 {
            let valor: unsafe extern "stdcall" fn(usize, u32, u32) -> f64 = core::mem::transmute(0x7c2150usize);
            return valor(r, qual, padrao.to_bits()); // cpu3: o valor do registro
        }
        if qual as u8 == 0x12 || qual as u8 == 0x14 {
            return padrao as f64; // cpu3: sem registro, o padrao
        }
        return 0.0; // cpu3: sem registro, zero
    }
    let (mut n, mut t) = (0u8, 0i16);
    let r2 = registro(this, jogador, 2);
    if r2 != 0 {
        n = *(r2 as *const u8);
        t = cpu3_w(r2 + 0xe);
    }
    let r3 = registro(this, jogador, 3);
    if r3 != 0 {
        n = n.wrapping_add(*(r3 as *const u8));
        t = t.wrapping_add(cpu3_w(r3 + 0xe));
    }
    let c23 = if n < 0x14 {
        (((0x14 - n as i32) as f64 * 5.0 + t as f64) * (0.05f32 as f64)) as f32
    } else {
        (t as f64 / n as f64) as f32
    };
    // ponytail: sem o registro 4 a original lê a vaga do byte n com os 3 bytes de cima sem valor; a cópia usa zero
    let mut c4 = f32::from_bits(n as u32);
    let r4 = registro(this, jogador, 4);
    if r4 != 0 {
        let k = *(r4 as *const u8);
        let v = cpu3_w(r4 + 0xe) as f64;
        c4 = if k < 8 { (((8 - k as i32) as f64 * 3.0 + v) * 0.125) as f32 } else { (v / k as f64) as f32 };
    }
    let mut c5 = 0.0f64; // ponytail: sem o registro 5, a original carrega uma vaga sem valor; a cópia usa zero
    let r5 = registro(this, jogador, 5);
    if r5 != 0 {
        let k = *(r5 as *const u8);
        let v = cpu3_w(r5 + 0xe) as f64;
        c5 = if k < 5 { ((5 - k as i32) as f64 + v) * (0.2f32 as f64) } else { v / k as f64 };
    }
    let (c23, c4) = (c23 as f64, c4 as f64);
    match categoria as u8 as i8 as i32 {
        100 => ((c5 * 1.5 + c4 * 2.5) + c23 * 6.0) * 0.1,
        101 => ((c5 * 3.0 + c4 * 5.0) + (c23 + c23)) * 0.1,
        102 => ((c5 * 5.0 + c4 * 4.0) + c23) * 0.1,
        _ => 0.0,
    }
}

unsafe extern "thiscall" fn compare_get_stat(this: usize, jogador: usize, categoria: u32, qual: u32, padrao: f32) -> f64 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, f32) -> f64>(%%I0%%);
    if jogador == 0 || (cpu3_d(jogador) as i32) >= cpu3_d(0xae236c) as i32 {
        return o(this, jogador, categoria, qual, padrao);
    }
    let c = categoria as u8 as i8;
    let simples = qual as u8 != 0x13 && c <= 6;
    // ponytail: categoria fora de 1..6 cai na caixa da 0x7c3920; nas combinadas sem o registro 4 ou 5, locais sem valor
    let registro: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x7c3920usize);
    if (simples && !(1..=6).contains(&c))
        || (!simples && (100..=102).contains(&(c as i32)) && (registro(this, jogador, 4) == 0 || registro(this, jogador, 5) == 0))
    {
        return o(this, jogador, categoria, qual, padrao);
    }
    let r = o(this, jogador, categoria, qual, padrao);
    let n = get_stat(this, jogador, categoria, qual, padrao);
    SCOREBOARD.record(r.to_bits() == n.to_bits(), || {
        format!("get_stat(jogador {}, categoria {}, qual {}, padrão {}) original={} rust={}", cpu3_d(jogador) as i32, c, qual as u8, padrao, r, n)
    });
    r
}

/// { 0x7c6d40 } (cdecl, player_stats.cpp): `player_stats_compare_language_entries(a, b)` (o qsort e o bsearch dos
/// idiomas, registros de 6 bytes {id, idioma, fluência}): a nulo, −1; b nulo, 1; ids (+0) diferentes, a diferença;
/// idioma 0xff em qualquer um, 0; senão a diferença dos idiomas (+4, com sinal). Pela fluência no idioma 0x7c6830 e pelos
/// idiomas acrescentados, tirados e melhorados.
unsafe extern "C" fn player_stats_compare_language_entries(a: usize, b: usize) -> i32 {
    if a == 0 {
        return -1;
    }
    if b == 0 {
        return 1;
    }
    let (ia, ib) = (cpu3_d(a) as i32, cpu3_d(b) as i32);
    if ia != ib {
        return ia.wrapping_sub(ib);
    }
    let (la, lb) = (*((a + 4) as *const u8), *((b + 4) as *const u8));
    if la == 0xff || lb == 0xff {
        return 0;
    }
    la as i8 as i32 - lb as i8 as i32
}

unsafe extern "C" fn compare_player_stats_compare_language_entries(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I1%%)(a, b);
    let n = player_stats_compare_language_entries(a, b);
    SCOREBOARD.record(r == n, || format!("player_stats_compare_language_entries({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// { 0x7c6d90 } (cdecl, player_stats.cpp): `player_stats_compare_history_by_club(a, b)` (o qsort da história na carga
/// das estatísticas 0x7bfd50): a nula, 1; b nula, −1; pelo dword +0xa e depois pelo +4, sem sinal: maior 1, menor −1,
/// iguais 0.
unsafe extern "C" fn player_stats_compare_history_by_club(a: usize, b: usize) -> i32 {
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let (ua, ub) = (cpu3_d(a + 0xa) as u32, cpu3_d(b + 0xa) as u32);
    if ua != ub {
        return if ua > ub { 1 } else { -1 };
    }
    let (va, vb) = (cpu3_d(a + 4) as u32, cpu3_d(b + 4) as u32);
    if va > vb {
        1
    } else if va < vb {
        -1
    } else {
        0
    }
}

unsafe extern "C" fn compare_player_stats_compare_history_by_club(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I2%%)(a, b);
    let n = player_stats_compare_history_by_club(a, b);
    SCOREBOARD.record(r == n, || format!("player_stats_compare_history_by_club({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}
