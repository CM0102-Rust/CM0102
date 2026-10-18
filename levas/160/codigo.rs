// ---------- centesima sexagesima leva: estatisticas do jogador (player_stats.cpp) e zagueiros na formacao ----------

/// { 0x7c2150 } (stdcall, ret 0xc, player_stats.cpp): a estatística `qual` do registro `r` (jogos
/// em +0, bytes +2..+0xd, palavras +0xe..+0x1a, +0x1c inteiro), como a original a deixa no x87:
/// 1..12 os bytes; 13 e 16 as porcentagens (+0x12/+0x10 e +0x1a/+0x18, vezes 100.0f); 14, 15, 17 e
/// 21 as médias por jogo; 18 a média de +0xe com menos de 20 jogos completada por `peso` (vezes
/// 0.05f); 20 a nota (2×+9 − +0xa + jogos, completada por 1.5 com menos de 20 jogos; 1.5f sem
/// jogos); 22 o inteiro. Sem divisor: 0.0. `r` nulo, 19 ou fora de 1..22: a original (caixa de erro).
unsafe extern "stdcall" fn estatistica_do_jogador(r: *const u8, qual: i8, peso: f32) -> f64 {
    if r.is_null() || !(1..=22).contains(&qual) || qual == 19 {
        return orig::<unsafe extern "stdcall" fn(*const u8, i8, f32) -> f64>(352)(r, qual, peso);
    }
    let b = |o: usize| *r.add(o) as f64;
    let w = |o: usize| core::ptr::read_unaligned(r.add(o) as *const i16) as f64;
    let jogos = *r;
    let cem = f32_em(0x968960) as f64;
    let vinte_avos = f32_em(0x96d61c) as f64;
    match qual {
        1 => b(0),
        2..=4 => b(qual as usize),
        5..=12 => b(qual as usize + 1),
        13 | 16 => {
            let (n, d) = if qual == 13 { (0x12, 0x10) } else { (0x1a, 0x18) };
            if core::ptr::read_unaligned(r.add(d) as *const i16) > 0 {
                w(n) / w(d) * cem
            } else {
                0.0
            }
        }
        14 | 15 | 17 | 21 => {
            let o = match qual { 14 => 0x14, 15 => 0x16, 17 => 0xe, _ => 0x18 };
            if jogos != 0 { w(o) / jogos as f64 } else { 0.0 }
        }
        18 => {
            if jogos < 20 {
                ((20 - jogos as i32) as f64 * peso as f64 + w(0xe)) * vinte_avos
            } else {
                w(0xe) / jogos as f64
            }
        }
        20 => {
            if jogos == 0 {
                return f32_em(0x96b004) as f64;
            }
            let nota = (*r.add(9) as i32 * 2 - *r.add(10) as i32 + jogos as i32) as f64;
            if jogos < 20 {
                (nota + (20 - jogos as i32) as f64 * f64_em(0x968940)) * vinte_avos
            } else {
                nota / jogos as f64
            }
        }
        _ => core::ptr::read_unaligned(r.add(0x1c) as *const i32) as f64,
    }
}

/// { 0x7c1950 } (thiscall, ret 4, player_stats.cpp): a média das cinco notas de 4 bits do registro
/// `*indice` (0x27 bytes cada, a partir de [this+0x30]): os dois meios de +0x24 e de +0x25 e o meio
/// baixo de +0x26, cada um ×4 (zero conta 27), somados em 16 bits e divididos por 4 (truncando).
/// `indice` nulo: a original (caixa de erro).
unsafe extern "thiscall" fn media_das_notas(this: usize, indice: *const i32) -> i32 {
    if indice.is_null() {
        return orig::<unsafe extern "thiscall" fn(usize, *const i32) -> i32>(353)(this, indice);
    }
    let reg = (*indice).wrapping_mul(0x27).wrapping_add(*((this + 0x30) as *const i32)) as usize;
    let nota = |n: u8| if n == 0 { 0x1b } else { n as u16 * 4 };
    let (a, c, e) = (*((reg + 0x24) as *const u8), *((reg + 0x25) as *const u8), *((reg + 0x26) as *const u8));
    let soma = nota(a & 0xf)
        .wrapping_add(nota(a >> 4))
        .wrapping_add(nota(c & 0xf))
        .wrapping_add(nota(c >> 4))
        .wrapping_add(nota(e & 0xf));
    soma as i16 as i32 / 4
}

/// { 0x5b8dc0 } (thiscall, formation.cpp): como a 0x5b8960, para os zagueiros: devolve 0 se alguma
/// das 11 vagas com lado aberto (0x880) é de zagueiro (4, na casa +0x12d ou +0x14e) ou de volante
/// (8 na +0x12d; na +0x14e pela 0x5b2ae0); senão 1. Devolve só `al`.
unsafe extern "thiscall" fn zagueiros_sem_conflito(this: usize) -> u8 {
    let casa: unsafe extern "thiscall" fn(usize, u32, u32) -> u16 = core::mem::transmute(0x5b2ae0usize);
    let w = |o: usize| core::ptr::read_unaligned((this + o) as *const u16);
    for i in 0..=10u32 {
        let a = w(0x12d + 2 * i as usize);
        let b = w(0x14e + 2 * i as usize);
        if a & 0x880 != 0 && a & 4 != 0 {
            return 0;
        }
        if b & 0x880 != 0 && b & 4 != 0 {
            return 0;
        }
        if a & 0x880 != 0 && a & 8 != 0 {
            return 0;
        }
        if b & 0x880 != 0 && casa(this, i, 1) & 8 != 0 {
            return 0;
        }
    }
    1
}

unsafe extern "stdcall" fn comparar_estatistica_do_jogador(r: *const u8, qual: i8, peso: f32) -> f64 {
    let o = orig::<unsafe extern "stdcall" fn(*const u8, i8, f32) -> f64>(352)(r, qual, peso);
    let n = estatistica_do_jogador(r, qual, peso);
    PLACAR.registrar(o.to_bits() == n.to_bits(), || format!("estatistica_do_jogador({:p}, {}, {}) original={} rust={}", r, qual, peso, o, n));
    o
}

comparar_valor!(comparar_media_das_notas, media_das_notas, 353, (indice: *const i32) -> i32);
comparar_valor!(comparar_zagueiros_sem_conflito, zagueiros_sem_conflito, 354, () -> u8);
