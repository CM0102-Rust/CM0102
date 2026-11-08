// ---------- %%LEVA%%a leva: o calendário das copas da França e da Espanha: as rodadas da Copa da França, da Copa da Liga francesa e da Copa do Rei (fra_cup.cpp, fra_lge_cup.cpp, spa_cup.cpp) ----------


/// Uma rodada de copa (registro de 0x68 bytes, docs/.md §2.4): o sorteio (dia, mês com 0 =
/// janeiro, anos a somar ao ano da copa +0x40, dia da semana ou −1) pela 0x521e60; o jogo (o mesmo, mais o turno em +6
/// e o mando em +0x13) pela 0x521eb0; depois os campos: +7 o nome da rodada, +9 o índice, +0xb os bits do sorteio, +0xd
/// e +0xf os desempates (jogo único e ida e volta), +0x17 a categoria copiada em cada jogo, +0x18/+0x1a/+0x1c/+0x1e
/// times, jogos, times novos e inscritos que já entraram, +0x20/+0x21/+0x22 replay, jogos por confronto e dias entre
/// eles, +0x5c/+0x60/+0x64 os prêmios (chegar, vencer, perder).
#[derive(Clone, Copy)]
struct FraCupCupRound {
    sorteio: (u32, u32, u32, u32),
    jogo: (u32, u32, u32, u32, u32, u32),
    codigo: u16,
    bits: (u16, u16, u16),
    categoria: u8,
    times: (u16, u16, u16, u16),
    confronto: (u8, u8, u8),
    premios: (u32, u32, u32),
}

/// A linha da tabela: (sorteio), (jogo), nome, (bits do sorteio, desempate, desempate de ida e volta), categoria,
/// (times, jogos, novos, inscritos), (replay, jogos por confronto, dias entre eles), (prêmios).
const fn cql11r(sorteio: (u32, u32, u32, u32), jogo: (u32, u32, u32, u32, u32, u32), codigo: u16, bits: (u16, u16, u16), categoria: u8,
    times: (u16, u16, u16, u16), confronto: (u8, u8, u8), premios: (u32, u32, u32)) -> FraCupCupRound {
    FraCupCupRound { sorteio, jogo, codigo, bits, categoria, times, confronto, premios }
}

/// Grava a rodada `i` da tabela `t` (ano-base `ano`, a palavra +0x40 da copa com sinal): as duas datas (0x521e60 e
/// 0x521eb0, cdecl, 7 e 9 argumentos; add esp, 0x1c / 0x24) e os campos.
unsafe fn fra_cup_write_cup_round(t: usize, i: u32, r: &FraCupCupRound, ano: u32) {
    let sorteio: unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(0x521e60usize);
    let jogo: unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(0x521eb0usize);
    sorteio(t, i, r.sorteio.0, r.sorteio.1, r.sorteio.2, r.sorteio.3, ano);
    jogo(t, i, r.jogo.0, r.jogo.1, r.jogo.2, r.jogo.3, r.jogo.4, ano, r.jogo.5);
    let e = t + i as usize * 0x68;
    let w = |o: usize, v: u16| core::ptr::write_unaligned((e + o) as *mut u16, v);
    let d = |o: usize, v: u32| core::ptr::write_unaligned((e + o) as *mut u32, v);
    w(7, r.codigo);
    w(9, i as u16);
    w(0xb, r.bits.0);
    w(0xd, r.bits.1);
    w(0xf, r.bits.2);
    *((e + 0x17) as *mut u8) = r.categoria;
    w(0x18, r.times.0);
    w(0x1a, r.times.1);
    w(0x1c, r.times.2);
    w(0x1e, r.times.3);
    *((e + 0x20) as *mut u8) = r.confronto.0;
    *((e + 0x21) as *mut u8) = r.confronto.1;
    *((e + 0x22) as *mut u8) = r.confronto.2;
    d(0x5c, r.premios.0);
    d(0x60, r.premios.1);
    d(0x64, r.premios.2);
}

/// O corpo comum da virtual +0x3c (`create_round_table(fase, &rodadas, &extra, &saida)`, thiscall, ret 0x10) das copas
/// de uma fase só: fora da fase −1 (o byte), devolve 0 sem gravar nada. Na −1: saida (se pedida) = 0, rodadas = o
/// tamanho da tabela, extra = 0; a tabela (0x944e46 de 0x68 × rodadas; sem memória, a caixa `linha` de `arquivo` e
/// devolve 0) com cada rodada; devolve a tabela.
unsafe fn single_stage_round_table(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32, tabela: &[FraCupCupRound], arquivo: usize, linha: u32, nome: &'static str) -> usize {
    if fase as u8 != 0xff {
        return 0;
    }
    if !saida.is_null() {
        *saida = 0;
    }
    *rodadas = tabela.len() as i16;
    *extra = 0;
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let t = alocar(tabela.len() as u32 * 0x68);
    if t == 0 {
        show_error_at(arquivo, linha);
        return 0;
    }
    crate::comparacao::ramo(nome);
    let ano = core::ptr::read_unaligned((this + 0x40) as *const i16) as i32 as u32;
    for (i, r) in tabela.iter().enumerate() {
        fra_cup_write_cup_round(t, i as u32, r, ano);
    }
    t
}

/// Os bytes que a original grava em cada rodada (o resto da memória alocada fica como veio): +0..+0x10 (as datas e os
/// campos até o desempate de ida e volta), +0x13..+0x22 (o mando até os dias entre os jogos) e +0x5c..+0x67 (os prêmios).
unsafe fn cql11_gravados(t: usize, n: i16) -> Vec<u8> {
    let mut v = Vec::new();
    for r in 0..n.max(0) as usize {
        let e = t + r * 0x68;
        for &(a, b) in &[(0usize, 0x11usize), (0x13, 0x23), (0x5c, 0x68)] {
            v.extend_from_slice(core::slice::from_raw_parts((e + a) as *const u8, b - a));
        }
    }
    v
}

/// Lado a lado: a original com as saídas de quem chama, a cópia com saídas próprias que começam com os mesmos valores, a
/// partir do mesmo [0xb67a34]; as saídas e os bytes gravados das duas tabelas têm de ser iguais (a da cópia é liberada).
unsafe fn compare_single_stage_round_table(nome: &str, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32,
    o: impl FnOnce() -> usize, c: impl FnOnce(*mut i16, *mut i16, *mut i32) -> usize) -> usize {
    if fase as u8 != 0xff {
        // ponytail: fora da fase −1 nenhuma das duas grava nada; as saídas podem nem existir
        let a = o();
        PLACAR.registrar(a == 0, || format!("{}(fase {}) devolveu {:#x}", nome, fase as u8 as i8, a));
        return a;
    }
    let (mut r2, mut e2) = (*rodadas, *extra);
    let mut s2 = if saida.is_null() { 0 } else { *saida };
    let reg = [(0xb67a34usize, 4usize)];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, o, || c(&mut r2, &mut e2, if saida.is_null() { core::ptr::null_mut() } else { &mut s2 }));
    let saidas = *rodadas == r2 && *extra == e2 && (saida.is_null() || *saida == s2);
    let tabela = if a != 0 && b != 0 { cql11_gravados(a, *rodadas) == cql11_gravados(b, r2) } else { a == 0 && b == 0 };
    if b != 0 {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(b);
    }
    PLACAR.registrar(igual && saidas && tabela, || format!("{}(fase {}): saídas {} ({} x {}) tabela {}", nome, fase as u8 as i8, saidas, *rodadas, r2, tabela));
    a
}

/// A Copa da França (FraCup): nove rodadas de outubro a maio do ano seguinte, a partir da 6ª rodada nacional (código
/// 0x3c); o clube da divisão mais baixa joga em casa (+0xb = 1) da 7ª rodada à semifinal.
const FRENCH_CUP_ROUNDS: [FraCupCupRound; 9] = [
    cql11r((0xa, 9, 0, 0xffff_ffff), (6, 0xa, 0, 5, 2, 0), 0x3c, (0, 3, 0), 4, (0x28, 0x14, 0x28, 0), (0, 1, 0), (0, 0, 0)),
    cql11r((0xc, 0xa, 0, 4), (0x1b, 0xa, 0, 5, 2, 0), 0x46, (1, 3, 0), 4, (0xb8, 0x5c, 0xa4, 0x28), (0, 1, 0), (0, 0, 0)), // cql11: a 7ª rodada da Copa da França
    cql11r((3, 0xb, 0, 4), (0x12, 0xb, 0, 5, 2, 0), 0x50, (1, 3, 0), 4, (0x5c, 0x2e, 0, 0xcc), (0, 1, 0), (0, 0, 0)),
    cql11r((6, 0, 1, 3), (0x16, 0, 1, 5, 2, 0), 0x5a, (1, 3, 0), 4, (0x40, 0x20, 0x12, 0xcc), (0, 1, 0), (0, 0, 0)),
    cql11r((0x1b, 0, 1, 3), (0xc, 1, 1, 5, 2, 0), 0x64, (1, 3, 0), 4, (0x20, 0x10, 0, 0), (0, 1, 0), (0, 0, 0)),
    cql11r((0x14, 1, 1, 6), (4, 2, 1, 5, 2, 0), 0x6e, (1, 3, 0), 4, (0x10, 8, 0, 0), (0, 1, 0), (0, 0, 0x186a0)),
    cql11r((5, 2, 1, 6), (0x13, 2, 1, 6, 2, 0), 0x78, (1, 3, 0), 6, (8, 4, 0, 0), (0, 1, 0), (0, 0, 0x30d40)),
    cql11r((0x14, 2, 1, 0), (0xc, 3, 1, 2, 2, 0), 0x82, (1, 3, 0), 6, (4, 2, 0, 0), (0, 1, 0), (0, 0, 0x61a80)),
    cql11r((0xd, 3, 1, 3), (6, 4, 1, 5, 2, 4), 0x96, (0, 3, 0), 6, (2, 1, 0, 0), (0, 1, 0), (0, 0xf4240, 0xc3500)),
];

/// { 0x5ba3f0 } (thiscall, ret 0x10, fra_cup.cpp; a virtual +0x3c da Copa da França): `FraCup::create_round_table(fase,
/// &rodadas, &extra, &saida)`: na fase −1, as 9 rodadas; a caixa da linha 0xa6 sem memória.
unsafe extern "thiscall" fn fra_cup_create_round_table(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    single_stage_round_table(this, fase, rodadas, extra, saida, &FRENCH_CUP_ROUNDS, 0x9ccb28, 0xa6, "cql_11 tabela da copa da franca")
}

unsafe extern "thiscall" fn comparar_fra_cup_create_round_table(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    compare_single_stage_round_table("fra_cup_create_round_table", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| fra_cup_create_round_table(this, fase, r, e, s))
}

/// A Copa da Liga francesa (FraLgeCup): sete rodadas de setembro a abril do ano seguinte.
const FRENCH_LEAGUE_CUP_ROUNDS: [FraCupCupRound; 7] = [
    cql11r((1, 8, 0, 3), (5, 9, 0, 1, 2, 0), 0xe6, (0, 3, 0), 4, (0x10, 8, 0x10, 0), (0, 1, 0), (0, 0x493e0, 0)),
    cql11r((0xf, 9, 0, 4), (0x10, 0xa, 0, 1, 2, 0), 0xa, (0, 3, 0), 4, (0x1c, 0xe, 0x14, 0x10), (0, 1, 0), (0, 0x493e0, 0)), // cql11: a 1ª rodada da Copa da Liga francesa
    cql11r((0x1a, 0xa, 0, 4), (8, 0, 1, 5, 2, 0), 0x14, (0, 3, 0), 4, (0x20, 0x10, 0x12, 0x24), (0, 1, 0), (0, 0x493e0, 0)),
    cql11r((0xe, 0, 1, 4), (0x1d, 0, 1, 5, 2, 0), 0x1e, (0, 3, 0), 4, (0x10, 8, 0, 0x24), (0, 1, 0), (0, 0x493e0, 0)),
    cql11r((3, 1, 1, 4), (0x13, 1, 1, 5, 2, 0), 0x78, (0, 3, 0), 4, (8, 4, 0, 0), (0, 1, 0), (0, 0x493e0, 0)),
    cql11r((4, 2, 1, 4), (1, 3, 1, 5, 2, 0), 0x82, (0, 3, 0), 6, (4, 2, 0, 0), (0, 1, 0), (0, 0x493e0, 0)),
    cql11r((2, 3, 1, 6), (0x16, 3, 1, 5, 2, 4), 0x96, (0, 3, 0), 6, (2, 1, 0, 0), (0, 1, 0), (0, 0x16e360, 0)),
];

/// { 0x5bcc40 } (thiscall, ret 0x10, fra_lge_cup.cpp; a virtual +0x3c da Copa da Liga francesa): `FraLgeCup::
/// create_round_table(fase, &rodadas, &extra, &saida)`: na fase −1, as 7 rodadas; a caixa da linha 0xa2 sem memória.
unsafe extern "thiscall" fn fra_cup_create_round_table_5bcc40(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    single_stage_round_table(this, fase, rodadas, extra, saida, &FRENCH_LEAGUE_CUP_ROUNDS, 0x9ccb98, 0xa2, "cql_11 tabela da copa da liga francesa")
}

unsafe extern "thiscall" fn comparar_fra_cup_create_round_table_5bcc40(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I1%%);
    compare_single_stage_round_table("fra_cup_create_round_table_5bcc40", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| fra_cup_create_round_table_5bcc40(this, fase, r, e, s))
}

/// A Copa do Rei (SpaCup): sete rodadas de agosto a maio do ano seguinte; o clube da divisão mais baixa joga em casa e
/// os clubes da mesma nação se separam (+0xb = 3) até a semifinal; da 3ª rodada à semifinal, ida e volta.
const COPA_DEL_REY_ROUNDS: [FraCupCupRound; 7] = [
    cql11r((0x12, 7, 0, 4), (0x1e, 7, 0, 2, 2, 0), 0xe6, (3, 3, 0), 4, (0x26, 0x13, 0x26, 0), (0, 1, 7), (0, 0xffff_ffff, 0)),
    cql11r((8, 8, 0, 4), (0xd, 0xb, 0, 2, 2, 0), 0xa, (3, 3, 0), 4, (0x40, 0x20, 0x2d, 0x26), (0, 1, 0), (0, 0xffff_ffff, 0)),
    cql11r((0xf, 0xb, 0, 4), (3, 0, 1, 2, 2, 0), 0x14, (3, 3, 0), 4, (0x20, 0x10, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)), // cql11: a 2ª rodada da Copa do Rei
    cql11r((5, 0, 1, 4), (0xa, 0, 1, 2, 2, 0), 0x1e, (3, 0, 3), 4, (0x10, 8, 0, 0), (0, 2, 7), (0, 0xffff_ffff, 0)),
    cql11r((0x13, 0, 1, 4), (0x1f, 0, 1, 2, 2, 0), 0x78, (3, 0, 3), 6, (8, 4, 0, 0), (0, 2, 7), (0, 0xffff_ffff, 0)),
    cql11r((0x17, 1, 1, 4), (0xb, 3, 1, 2, 2, 0), 0x82, (3, 0, 3), 6, (4, 2, 0, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)),
    cql11r((0x1a, 3, 1, 3), (0x1a, 4, 1, 5, 1, 4), 0x96, (0, 3, 0), 6, (2, 1, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
];

/// { 0x84d220 } (thiscall, ret 0x10, spa_cup.cpp; a virtual +0x3c da Copa do Rei): `SpaCup::create_round_table(fase,
/// &rodadas, &extra, &saida)`: na fase −1, as 7 rodadas; a caixa da linha 0xa8 sem memória.
unsafe extern "thiscall" fn spa_cup_create_round_table(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    single_stage_round_table(this, fase, rodadas, extra, saida, &COPA_DEL_REY_ROUNDS, 0xa82810, 0xa8, "cql_11 tabela da copa do rei")
}

unsafe extern "thiscall" fn comparar_spa_cup_create_round_table(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I2%%);
    compare_single_stage_round_table("spa_cup_create_round_table", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| spa_cup_create_round_table(this, fase, r, e, s))
}