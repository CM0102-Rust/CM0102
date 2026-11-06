// ---------- 276a leva: o calendário das copas inglesas: as rodadas da Copa da Liga, da Copa da Inglaterra e da Supercopa (eng_cc_cup.cpp, eng_fa_cup.cpp, eng_charity.cpp) ----------

/// Uma rodada de copa (registro de 0x68 bytes, docs/.md §2.4): o sorteio (dia, mês com 0 =
/// janeiro, anos a somar ao ano da copa +0x40, dia da semana ou −1) pela 0x521e60; o jogo (o mesmo, mais o turno em +6
/// e o mando em +0x13) pela 0x521eb0; depois os campos: +7 o nome da rodada, +9 o índice, +0xb os bits do sorteio, +0xd
/// e +0xf os desempates (jogo único e ida e volta), +0x17 a categoria copiada em cada jogo, +0x18/+0x1a/+0x1c/+0x1e
/// times, jogos, times novos e inscritos que já entraram, +0x20/+0x21/+0x22 replay, jogos por confronto e dias entre
/// eles, +0x5c/+0x60/+0x64 os prêmios (chegar, vencer, perder).
#[derive(Clone, Copy)]
struct Cql10R {
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
const fn cql10r(sorteio: (u32, u32, u32, u32), jogo: (u32, u32, u32, u32, u32, u32), codigo: u16, bits: (u16, u16, u16), categoria: u8,
    times: (u16, u16, u16, u16), confronto: (u8, u8, u8), premios: (u32, u32, u32)) -> Cql10R {
    Cql10R { sorteio, jogo, codigo, bits, categoria, times, confronto, premios }
}

/// Grava a rodada `i` da tabela `t` (ano-base `ano`, a palavra +0x40 da copa com sinal): as duas datas (0x521e60 e
/// 0x521eb0, cdecl, 7 e 9 argumentos; add esp, 0x1c / 0x24) e os campos.
unsafe fn cql10_rodada(t: usize, i: u32, r: &Cql10R, ano: u32) {
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
unsafe fn cql10_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32, tabela: &[Cql10R], arquivo: usize, linha: u32, nome: &'static str) -> usize {
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
        aviso_em(arquivo, linha);
        return 0;
    }
    crate::comparacao::ramo(nome);
    let ano = core::ptr::read_unaligned((this + 0x40) as *const i16) as i32 as u32;
    for (i, r) in tabela.iter().enumerate() {
        cql10_rodada(t, i as u32, r, ano);
    }
    t
}

/// Os bytes que a original grava em cada rodada (o resto da memória alocada fica como veio): +0..+0x10 (as datas e os
/// campos até o desempate de ida e volta), +0x13..+0x22 (o mando até os dias entre os jogos) e +0x5c..+0x67 (os prêmios).
unsafe fn cql10_gravados(t: usize, n: i16) -> Vec<u8> {
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
unsafe fn cql10_comparar(nome: &str, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32,
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
    let tabela = if a != 0 && b != 0 { cql10_gravados(a, *rodadas) == cql10_gravados(b, r2) } else { a == 0 && b == 0 };
    if b != 0 {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(b);
    }
    PLACAR.registrar(igual && saidas && tabela, || format!("{}(fase {}): saídas {} ({} x {}) tabela {}", nome, fase as u8 as i8, saidas, *rodadas, r2, tabela));
    a
}

/// A Copa da Liga inglesa (EngCcCup): sete rodadas de agosto a março do ano seguinte; as três primeiras dependem de
/// +0xb2 (os clubes nas copas europeias, que entram mais tarde) e vêm à parte (`cql10_copa_da_liga_rodadas`).
const CQL10_COPA_DA_LIGA: [Cql10R; 7] = [
    cql10r((0x17, 6, 0, 5), (0x16, 7, 0, 1, 2, 0), 0xa, (0, 0x403, 0), 4, (0, 0, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
    cql10r((0x17, 7, 0, 2), (0x14, 8, 0, 2, 2, 0), 0x14, (0, 0x403, 0), 4, (0, 0, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)), // cql10: a 2ª rodada da Copa da Liga
    cql10r((0x1c, 8, 0, 3), (0x1c, 9, 0, 2, 2, 0), 0x1e, (0, 3, 0), 4, (0x20, 0x10, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
    cql10r((0x1d, 9, 0, 3), (0xb, 0xa, 0, 2, 2, 0), 0x28, (0, 0x203, 0), 4, (0x10, 8, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
    cql10r((0xc, 0xa, 0, 3), (2, 0xb, 0, 2, 2, 0), 0x78, (0, 0x203, 0), 6, (8, 4, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
    cql10r((3, 0xb, 0, 3), (0x1b, 0, 1, 2, 2, 0), 0x82, (0, 0x4200, 0x4003), 6, (4, 2, 0, 0), (0, 2, 0x15), (0, 0xffff_ffff, 0)),
    cql10r((0x12, 1, 1, 3), (1, 3, 1, 6, 1, 4), 0x96, (0, 3, 0), 6, (2, 1, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
];

/// A tabela da Copa da Liga com `b` = +0xb2 (byte com sinal; as contas em 16 bits como a original): 1ª rodada 2b + 0x38
/// times, b + 0x1c jogos, 2b + 0x38 novos; 2ª rodada (0x20 − b) × 2 times, 0x20 − b jogos, (0xc − b) × 3 novos e 2b +
/// 0x38 inscritos; 3ª rodada b novos e 0x5c − b inscritos.
fn cql10_copa_da_liga_rodadas(b: i16) -> [Cql10R; 7] {
    let mut t = CQL10_COPA_DA_LIGA;
    let x = |v: i16| v as u16;
    t[0].times = (x(b.wrapping_mul(2).wrapping_add(0x38)), x(b.wrapping_add(0x1c)), x(b.wrapping_mul(2).wrapping_add(0x38)), 0);
    t[1].times = (x(0x20i16.wrapping_sub(b).wrapping_mul(2)), x(0x20i16.wrapping_sub(b)), x(0xci16.wrapping_sub(b).wrapping_mul(3)), x(b.wrapping_mul(2).wrapping_add(0x38)));
    t[2].times.2 = x(b);
    t[2].times.3 = x(0x5ci16.wrapping_sub(b));
    t
}

/// { 0x56c570 } (thiscall, ret 0x10, eng_cc_cup.cpp; a virtual +0x3c da Copa da Liga): `EngCcCup::create_round_table(
/// fase, &rodadas, &extra, &saida)`: na fase −1, as 7 rodadas (`cql10_copa_da_liga_rodadas` com o +0xb2); a caixa da
/// linha 0xa2 sem memória.
unsafe extern "thiscall" fn cql10_copa_da_liga_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let t = cql10_copa_da_liga_rodadas(*((this + 0xb2) as *const i8) as i16);
    cql10_tabela(this, fase, rodadas, extra, saida, &t, 0x9c4224, 0xa2, "cql_10 tabela da copa da liga")
}

unsafe extern "thiscall" fn comparar_cql10_copa_da_liga_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1772);
    cql10_comparar("cql10_copa_da_liga_tabela", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| cql10_copa_da_liga_tabela(this, fase, r, e, s))
}

/// A Copa da Inglaterra (EngFaCup): nove rodadas de outubro a maio do ano seguinte, com replay (+0x20 = 1) até as
/// quartas; as rodadas 2 a 5 dependem de +0xb3 (isenções) e +0xb4 (a escolha da entrada na 5ª rodada) e do +0x3e (os
/// inscritos) e vêm à parte (`cql10_copa_da_inglaterra_rodadas`). A original grava ainda +0x13 = 0 nas rodadas 0 a 6,
/// que a data do jogo já deixou com o mando 0.
const CQL10_COPA_DA_INGLATERRA: [Cql10R; 9] = [
    cql10r((9, 9, 0, 5), (0x10, 9, 0, 5, 1, 0), 0xdc, (0, 0, 3), 4, (0x38, 0x1c, 0x38, 0), (1, 0, 0xa), (0, 0x2710, 0)),
    cql10r((0x11, 9, 0, 6), (0x12, 0xa, 0, 5, 1, 0), 0xa, (0, 0, 3), 4, (0x50, 0x28, 0x34, 0x38), (1, 1, 0xa), (0, 0x4e20, 0)),
    cql10r((0x13, 0xa, 0, 6), (9, 0xb, 0, 5, 1, 0), 0x14, (0, 0, 3), 4, (0, 0, 0, 0), (1, 1, 0xa), (0, 0x4e20, 0)), // cql10: a 2ª rodada da Copa da Inglaterra
    cql10r((0xa, 0xb, 0, 6), (6, 0, 1, 5, 1, 0), 0x1e, (0, 0, 3), 4, (0, 0, 0, 0), (1, 1, 0xa), (0, 0x7530, 0)),
    cql10r((7, 0, 1, 6), (0x1b, 0, 1, 5, 1, 0), 0x28, (0, 0, 3), 4, (0, 0, 0, 0), (1, 1, 0xa), (0, 0xc350, 0)),
    cql10r((0x1c, 0, 1, 6), (0x11, 1, 1, 5, 1, 0), 0x32, (0, 0, 3), 4, (0x10, 8, 0, 0), (1, 1, 0xa), (0, 0x124f8, 0)),
    cql10r((0x12, 1, 1, 6), (0xa, 2, 1, 5, 1, 0), 0x78, (0, 0, 3), 6, (8, 4, 0, 0), (1, 1, 0xa), (0, 0x249f0, 0)),
    cql10r((0xb, 2, 1, 6), (8, 3, 1, 6, 1, 0x2a), 0x82, (0, 3, 0), 6, (4, 2, 0, 0), (0, 1, 0xa), (0, 0x61a80, 0)),
    cql10r((9, 3, 1, 0), (0xc, 4, 1, 5, 1, 4), 0x96, (0, 3, 0), 6, (2, 1, 0, 0), (0, 1, 0), (0, 0x1e8480, 0xf4240)),
];

/// A tabela da Copa da Inglaterra com `b` = +0xb3 (byte com sinal), `entrada` = +0xb4 e `inscritos` = +0x3e (contas
/// em 16 bits): com +0xb4 = 1, a 1ª rodada propriamente dita tem 0x28 times, 0x13 jogos e nenhum novo, e a 2ª 0x40
/// times, 0x20 jogos, 0x2b novos e 0x6c inscritos; senão, 6b + 0x28 times, 3b + 0x14 jogos, 6b novos e 0x6c inscritos
/// na primeira, (0x10 − b) × 4 times, (0x10 − b) × 2 jogos, 0x2c − 7b novos e (b + 0x12) × 6 inscritos na segunda. A 3ª
/// tem (0x10 − b) × 2 times e 0x10 − b jogos; a 4ª, b novos e +0x3e − b inscritos.
fn cql10_copa_da_inglaterra_rodadas(b: i16, entrada: u8, inscritos: i16) -> [Cql10R; 9] {
    let mut t = CQL10_COPA_DA_INGLATERRA;
    let x = |v: i16| v as u16;
    if entrada == 1 {
        t[2].times = (0x28, 0x13, 0, 0);
        t[3].times = (0x40, 0x20, 0x2b, 0x6c);
    } else {
        t[2].times = (x(b.wrapping_mul(6).wrapping_add(0x28)), x(b.wrapping_mul(3).wrapping_add(0x14)), x(b.wrapping_mul(6)), 0x6c);
        t[3].times = (x(0x10i16.wrapping_sub(b).wrapping_mul(4)), x(0x10i16.wrapping_sub(b).wrapping_mul(2)), x(0x2ci16.wrapping_sub(b.wrapping_mul(7))),
            x(b.wrapping_add(0x12).wrapping_mul(6)));
    }
    t[4].times.0 = x(0x10i16.wrapping_sub(b).wrapping_mul(2));
    t[4].times.1 = x(0x10i16.wrapping_sub(b));
    t[5].times.2 = x(b);
    t[5].times.3 = x(inscritos.wrapping_sub(b));
    t
}

/// { 0x56f320 } (thiscall, ret 0x10, eng_fa_cup.cpp; a virtual +0x3c da Copa da Inglaterra): `EngFaCup::
/// create_round_table(fase, &rodadas, &extra, &saida)`: na fase −1, as 9 rodadas (`cql10_copa_da_inglaterra_rodadas`);
/// a caixa da linha 0xb6 sem memória.
unsafe extern "thiscall" fn cql10_copa_da_inglaterra_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let t = cql10_copa_da_inglaterra_rodadas(*((this + 0xb3) as *const i8) as i16, *((this + 0xb4) as *const u8), core::ptr::read_unaligned((this + 0x3e) as *const i16));
    cql10_tabela(this, fase, rodadas, extra, saida, &t, 0x9c42cc, 0xb6, "cql_10 tabela da copa da inglaterra")
}

unsafe extern "thiscall" fn comparar_cql10_copa_da_inglaterra_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1773);
    cql10_comparar("cql10_copa_da_inglaterra_tabela", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| cql10_copa_da_inglaterra_tabela(this, fase, r, e, s))
}

/// A Supercopa inglesa (Charity Shield): uma rodada, um jogo em agosto.
const CQL10_SUPERCOPA: [Cql10R; 1] = [
    cql10r((5, 6, 0, 5), (0xd, 7, 0, 6, 1, 4), 0, (0, 1, 0), 6, (2, 1, 2, 0), (0, 1, 0), (0, 0, 0)), // cql10: o jogo da Supercopa inglesa
];

/// { 0x56d640 } (thiscall, ret 0x10, eng_charity.cpp; a virtual +0x3c da Supercopa): `EngCharity::create_round_table(
/// fase, &rodadas, &extra, &saida)`: na fase −1, a rodada única; a caixa da linha 0x9c sem memória.
unsafe extern "thiscall" fn cql10_supercopa_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    cql10_tabela(this, fase, rodadas, extra, saida, &CQL10_SUPERCOPA, 0x9c425c, 0x9c, "cql_10 tabela da supercopa inglesa")
}

unsafe extern "thiscall" fn comparar_cql10_supercopa_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1774);
    cql10_comparar("cql10_supercopa_tabela", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| cql10_supercopa_tabela(this, fase, r, e, s))
}

// ---------- 276a leva: o calendário das copas da França e da Espanha: as rodadas da Copa da França, da Copa da Liga francesa e da Copa do Rei (fra_cup.cpp, fra_lge_cup.cpp, spa_cup.cpp) ----------


/// Uma rodada de copa (registro de 0x68 bytes, docs/.md §2.4): o sorteio (dia, mês com 0 =
/// janeiro, anos a somar ao ano da copa +0x40, dia da semana ou −1) pela 0x521e60; o jogo (o mesmo, mais o turno em +6
/// e o mando em +0x13) pela 0x521eb0; depois os campos: +7 o nome da rodada, +9 o índice, +0xb os bits do sorteio, +0xd
/// e +0xf os desempates (jogo único e ida e volta), +0x17 a categoria copiada em cada jogo, +0x18/+0x1a/+0x1c/+0x1e
/// times, jogos, times novos e inscritos que já entraram, +0x20/+0x21/+0x22 replay, jogos por confronto e dias entre
/// eles, +0x5c/+0x60/+0x64 os prêmios (chegar, vencer, perder).
#[derive(Clone, Copy)]
struct Cql11R {
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
    times: (u16, u16, u16, u16), confronto: (u8, u8, u8), premios: (u32, u32, u32)) -> Cql11R {
    Cql11R { sorteio, jogo, codigo, bits, categoria, times, confronto, premios }
}

/// Grava a rodada `i` da tabela `t` (ano-base `ano`, a palavra +0x40 da copa com sinal): as duas datas (0x521e60 e
/// 0x521eb0, cdecl, 7 e 9 argumentos; add esp, 0x1c / 0x24) e os campos.
unsafe fn cql11_rodada(t: usize, i: u32, r: &Cql11R, ano: u32) {
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
unsafe fn cql11_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32, tabela: &[Cql11R], arquivo: usize, linha: u32, nome: &'static str) -> usize {
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
        aviso_em(arquivo, linha);
        return 0;
    }
    crate::comparacao::ramo(nome);
    let ano = core::ptr::read_unaligned((this + 0x40) as *const i16) as i32 as u32;
    for (i, r) in tabela.iter().enumerate() {
        cql11_rodada(t, i as u32, r, ano);
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
unsafe fn cql11_comparar(nome: &str, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32,
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
const CQL11_COPA_DA_FRANCA: [Cql11R; 9] = [
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
unsafe extern "thiscall" fn cql11_copa_da_franca_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    cql11_tabela(this, fase, rodadas, extra, saida, &CQL11_COPA_DA_FRANCA, 0x9ccb28, 0xa6, "cql_11 tabela da copa da franca")
}

unsafe extern "thiscall" fn comparar_cql11_copa_da_franca_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1775);
    cql11_comparar("cql11_copa_da_franca_tabela", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| cql11_copa_da_franca_tabela(this, fase, r, e, s))
}

/// A Copa da Liga francesa (FraLgeCup): sete rodadas de setembro a abril do ano seguinte.
const CQL11_COPA_DA_LIGA_FRANCESA: [Cql11R; 7] = [
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
unsafe extern "thiscall" fn cql11_copa_da_liga_francesa_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    cql11_tabela(this, fase, rodadas, extra, saida, &CQL11_COPA_DA_LIGA_FRANCESA, 0x9ccb98, 0xa2, "cql_11 tabela da copa da liga francesa")
}

unsafe extern "thiscall" fn comparar_cql11_copa_da_liga_francesa_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1776);
    cql11_comparar("cql11_copa_da_liga_francesa_tabela", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| cql11_copa_da_liga_francesa_tabela(this, fase, r, e, s))
}

/// A Copa do Rei (SpaCup): sete rodadas de agosto a maio do ano seguinte; o clube da divisão mais baixa joga em casa e
/// os clubes da mesma nação se separam (+0xb = 3) até a semifinal; da 3ª rodada à semifinal, ida e volta.
const CQL11_COPA_DO_REI: [Cql11R; 7] = [
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
unsafe extern "thiscall" fn cql11_copa_do_rei_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    cql11_tabela(this, fase, rodadas, extra, saida, &CQL11_COPA_DO_REI, 0xa82810, 0xa8, "cql_11 tabela da copa do rei")
}

unsafe extern "thiscall" fn comparar_cql11_copa_do_rei_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1777);
    cql11_comparar("cql11_copa_do_rei_tabela", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| cql11_copa_do_rei_tabela(this, fase, r, e, s))
}
// ---------- 276a leva: o calendário das supercopas: o jogo da Supercopa da França, da Espanha e do Japão (fra_super.cpp, spa_super.cpp, jap_super.cpp) ----------


/// Uma rodada de copa (registro de 0x68 bytes, docs/.md §2.4): o sorteio (dia, mês com 0 =
/// janeiro, anos a somar ao ano da copa +0x40, dia da semana ou −1) pela 0x521e60; o jogo (o mesmo, mais o turno em +6
/// e o mando em +0x13) pela 0x521eb0; depois os campos: +7 o nome da rodada, +9 o índice, +0xb os bits do sorteio, +0xd
/// e +0xf os desempates (jogo único e ida e volta), +0x17 a categoria copiada em cada jogo, +0x18/+0x1a/+0x1c/+0x1e
/// times, jogos, times novos e inscritos que já entraram, +0x20/+0x21/+0x22 replay, jogos por confronto e dias entre
/// eles, +0x5c/+0x60/+0x64 os prêmios (chegar, vencer, perder).
#[derive(Clone, Copy)]
struct Cql12R {
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
const fn cql12r(sorteio: (u32, u32, u32, u32), jogo: (u32, u32, u32, u32, u32, u32), codigo: u16, bits: (u16, u16, u16), categoria: u8,
    times: (u16, u16, u16, u16), confronto: (u8, u8, u8), premios: (u32, u32, u32)) -> Cql12R {
    Cql12R { sorteio, jogo, codigo, bits, categoria, times, confronto, premios }
}

/// Grava a rodada `i` da tabela `t` (ano-base `ano`, a palavra +0x40 da copa com sinal): as duas datas (0x521e60 e
/// 0x521eb0, cdecl, 7 e 9 argumentos; add esp, 0x1c / 0x24) e os campos.
unsafe fn cql12_rodada(t: usize, i: u32, r: &Cql12R, ano: u32) {
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
/// devolve 0) com cada rodada; devolve a tabela. `ajuste` mexe na primeira rodada depois da alocação (o mando sorteado da
/// Supercopa francesa).
unsafe fn cql12_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32, tabela: &[Cql12R], arquivo: usize, linha: u32, nome: &'static str,
    ajuste: impl FnOnce(&mut Cql12R)) -> usize {
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
        aviso_em(arquivo, linha);
        return 0;
    }
    crate::comparacao::ramo(nome);
    let mut t0 = tabela[0];
    ajuste(&mut t0);
    let ano = core::ptr::read_unaligned((this + 0x40) as *const i16) as i32 as u32;
    for (i, r) in tabela.iter().enumerate() {
        cql12_rodada(t, i as u32, if i == 0 { &t0 } else { r }, ano);
    }
    t
}

/// Os bytes que a original grava em cada rodada (o resto da memória alocada fica como veio): +0..+0x10 (as datas e os
/// campos até o desempate de ida e volta), +0x13..+0x22 (o mando até os dias entre os jogos) e +0x5c..+0x67 (os prêmios).
unsafe fn cql12_gravados(t: usize, n: i16) -> Vec<u8> {
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
unsafe fn cql12_comparar(nome: &str, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32,
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
    let tabela = if a != 0 && b != 0 { cql12_gravados(a, *rodadas) == cql12_gravados(b, r2) } else { a == 0 && b == 0 };
    if b != 0 {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(b);
    }
    PLACAR.registrar(igual && saidas && tabela, || format!("{}(fase {}): saídas {} ({} x {}) tabela {}", nome, fase as u8 as i8, saidas, *rodadas, r2, tabela));
    a
}

/// A Supercopa da França: um jogo em julho; o prêmio de 60 mil por chegar. O mando vem à parte (`cql12_mando_da_franca`).
const CQL12_SUPERCOPA_DA_FRANCA: [Cql12R; 1] = [
    cql12r((5, 6, 0, 5), (0x18, 6, 0, 5, 2, 0x400), 0, (0, 1, 0), 6, (2, 1, 2, 0), (0, 1, 0), (0xea60, 0, 0)), // cql12: o jogo da Supercopa da França
];

/// O mando da Supercopa da França: 0x400 em 2001 (+0x40 = 0x7d1); nos outros anos, um dos seis (0x200, 0x400, 0x800,
/// 0x1000, 0x2000, 0x4000) pela 0x90cfc0 (push 6; add esp, 4 -> cdecl, o sorteio de 0 a 5).
unsafe fn cql12_mando_da_franca(this: usize) -> u32 {
    if core::ptr::read_unaligned((this + 0x40) as *const i16) == 0x7d1 {
        return 0x400;
    }
    let sortear: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(0x90cfc0usize);
    const MANDOS: [i16; 6] = [0x200, 0x400, 0x800, 0x1000, 0x2000, 0x4000];
    MANDOS[sortear(6) as usize] as i32 as u32
}

/// { 0x5bf5d0 } (thiscall, ret 0x10, fra_super.cpp; a virtual +0x3c da Supercopa da França): `FraSuper::
/// create_round_table(fase, &rodadas, &extra, &saida)`: na fase −1, a rodada única com o mando de
/// `cql12_mando_da_franca` (a original sorteia depois da data do sorteio, que não sorteia: a cópia sorteia logo antes);
/// a caixa da linha 0x97 sem memória.
unsafe extern "thiscall" fn cql12_supercopa_da_franca_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    cql12_tabela(this, fase, rodadas, extra, saida, &CQL12_SUPERCOPA_DA_FRANCA, 0x9ccc48, 0x97, "cql_12 tabela da supercopa da franca",
        |r| r.jogo.5 = cql12_mando_da_franca(this))
}

// ponytail: fora de 2001 a original sorteia o mando (0x90cfc0): só ela roda nesse caso.
unsafe extern "thiscall" fn comparar_cql12_supercopa_da_franca_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1778);
    if core::ptr::read_unaligned((this + 0x40) as *const i16) != 0x7d1 {
        return o(this, fase, rodadas, extra, saida);
    }
    cql12_comparar("cql12_supercopa_da_franca_tabela", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| cql12_supercopa_da_franca_tabela(this, fase, r, e, s))
}

/// A Supercopa da Espanha: ida e volta (7 dias entre os jogos) em agosto, sem sorteio (+0xd = 0x200), desempate de ida e
/// volta 1.
const CQL12_SUPERCOPA_DA_ESPANHA: [Cql12R; 1] = [
    cql12r((5, 6, 0, 5), (0x14, 7, 0, 6, 1, 0), 0, (0, 0x200, 1), 6, (2, 1, 2, 0), (0, 2, 7), (0, 0, 0)), // cql12: a ida da Supercopa da Espanha
];

/// { 0x858960 } (thiscall, ret 0x10, spa_super.cpp; a virtual +0x3c da Supercopa da Espanha): `SpaSuper::
/// create_round_table(fase, &rodadas, &extra, &saida)`: na fase −1, a rodada única; a caixa da linha 0x99 sem memória.
unsafe extern "thiscall" fn cql12_supercopa_da_espanha_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    cql12_tabela(this, fase, rodadas, extra, saida, &CQL12_SUPERCOPA_DA_ESPANHA, 0xa82a58, 0x99, "cql_12 tabela da supercopa da espanha", |_| {})
}

unsafe extern "thiscall" fn comparar_cql12_supercopa_da_espanha_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1779);
    cql12_comparar("cql12_supercopa_da_espanha_tabela", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| cql12_supercopa_da_espanha_tabela(this, fase, r, e, s))
}

/// A Supercopa do Japão: um jogo em março no estádio neutro, 150 mil por vencer.
const CQL12_SUPERCOPA_DO_JAPAO: [Cql12R; 1] = [
    cql12r((1, 1, 0, 5), (4, 2, 0, 5, 1, 4), 0, (0, 1, 0), 6, (2, 1, 2, 0), (0, 1, 0), (0, 0x249f0, 0)), // cql12: o jogo da Supercopa do Japão
];

/// { 0x666020 } (thiscall, ret 0x10, jap_super.cpp; a virtual +0x3c da Supercopa do Japão): `JapSuper::
/// create_round_table(fase, &rodadas, &extra, &saida)`: na fase −1, a rodada única; a caixa da linha 0x98 sem memória.
unsafe extern "thiscall" fn cql12_supercopa_do_japao_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    cql12_tabela(this, fase, rodadas, extra, saida, &CQL12_SUPERCOPA_DO_JAPAO, 0x9eaf4c, 0x98, "cql_12 tabela da supercopa do japao", |_| {})
}

unsafe extern "thiscall" fn comparar_cql12_supercopa_do_japao_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1780);
    cql12_comparar("cql12_supercopa_do_japao_tabela", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| cql12_supercopa_do_japao_tabela(this, fase, r, e, s))
}
// ---------- 276a leva: os inscritos das copas inglesas: a Copa da Liga, a segunda chave do Troféu da Liga e os dois clubes da Supercopa (eng_cc_cup.cpp, eng_auto_cup.cpp, eng_charity.cpp) ----------

unsafe fn cqm10_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqm10_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O clube `i` da tabela dos clubes ([0xae23b4], registros de 0x245 bytes).
unsafe fn cqm10_clube(i: i16) -> usize {
    cqm10_d(0xae23b4) + i as isize as usize * 0x245
}

/// O id da competição da divisão do clube (+0x57, ponteiro para o registro da competição; +0 o id), ou None sem divisão.
unsafe fn cqm10_divisao(c: usize) -> Option<usize> {
    let d = cqm10_d(c + 0x57);
    if d == 0 { None } else { Some(cqm10_d(d)) }
}

/// Grava a inscrição `k` da lista (+0xa7, 6 bytes: o clube, +4 a classe, +5 a situação).
unsafe fn cqm10_inscrever(this: usize, k: i32, clube: usize, classe: u8) {
    let e = cqm10_d(this + 0xa7) + k as usize * 6;
    core::ptr::write_unaligned(e as *mut u32, clube as u32);
    *((e + 4) as *mut u8) = classe;
    *((e + 5) as *mut u8) = 0;
}

/// Uma passada pelos clubes da divisão `id` ([id], o id da competição) fora das copas europeias (+0x1db = −1), até a
/// lista encher (+0x3e); a classe de cada um vem de `classe(clube)`.
unsafe fn cqm10_passada(this: usize, n: &mut i32, id: usize, classe: impl Fn(usize) -> u8) {
    let mut i: i16 = 0;
    while (i as i32) < cqm10_d(0xae235c) as i32 {
        let c = cqm10_clube(i);
        if let Some(d) = cqm10_divisao(c) {
            if *n != cqm10_w(this + 0x3e) as i32 && d == cqm10_d(id) && cqm10_d(c + 0x1db) as i32 == -1 {
                cqm10_inscrever(this, *n, c, classe(c));
                *n += 1;
                if *n == cqm10_w(this + 0x3e) as i32 {
                    return;
                }
            }
        }
        i = i.wrapping_add(1);
    }
}

/// A classe de quem veio da divisão `de` (+0x5b, a competição da temporada anterior; +0x5f a posição nela, byte): `base`
/// menos a posição.
unsafe fn cqm10_veio(c: usize, de: usize) -> bool {
    let p = cqm10_d(c + 0x5b);
    p != 0 && cqm10_d(p) == cqm10_d(de)
}

/// { 0x56ccc0 } (thiscall, ret 0, eng_cc_cup.cpp): `EngCcCup::collect_entrants()`: +0x3e = 92 e +0xb2 = 0; a lista
/// (0x944e46 de 0x228 em +0xa7; sem memória, a caixa da linha 0x18f e −1). Fora das copas europeias, na ordem: a Third
/// ([0x9cf5c8]) e a Second ([0x9cf5c4]) com classe 0; a First ([0x9cf5c0]) com 0x46 − posição se veio da Premier
/// ([0x9cf5bc]), 0x28 − posição se já era da First, 0 senão; a Premier com 0x78 − posição se já era da Premier, 0x64 −
/// posição se veio da First, 0 senão. Depois os clubes das quatro divisões nas copas europeias (+0x1db ≠ −1), classe
/// 0x78, cada um somando 1 ao +0xb2. A lista vai ao qsort (0x945333 com 0x5221c0: a classe menor primeiro, no empate a
/// reputação) e a classe vira 1 (Premier), 2 (First) ou 3 (os outros). Lista cheia: +0xb2 no máximo 12 (byte com sinal) e
/// 1; senão 0. Devolve também quantos entraram (para o comparador).
unsafe fn cqm10_copa_da_liga(this: usize) -> (i32, i32) {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 0x5c);
    *((this + 0xb2) as *mut u8) = 0;
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let l = alocar(0x228);
    core::ptr::write_unaligned((this + 0xa7) as *mut u32, l as u32);
    if l == 0 {
        aviso_em(0x9c4224, 0x18f);
        return (-1, 0);
    }
    let mut n = 0i32;
    cqm10_passada(this, &mut n, 0x9cf5c8, |_| 0);
    cqm10_passada(this, &mut n, 0x9cf5c4, |_| 0);
    cqm10_passada(this, &mut n, 0x9cf5c0, |c| {
        let pos = *((c + 0x5f) as *const u8);
        if cqm10_veio(c, 0x9cf5bc) {
            crate::comparacao::ramo("cqm_10 caiu da premier");
            0x46u8.wrapping_sub(pos) // cqm10: quem caiu da Premier
        } else if cqm10_veio(c, 0x9cf5c0) {
            0x28u8.wrapping_sub(pos)
        } else {
            0
        }
    });
    cqm10_passada(this, &mut n, 0x9cf5bc, |c| {
        let pos = *((c + 0x5f) as *const u8);
        if cqm10_veio(c, 0x9cf5bc) {
            0x78u8.wrapping_sub(pos)
        } else if cqm10_veio(c, 0x9cf5c0) {
            0x64u8.wrapping_sub(pos)
        } else {
            0
        }
    });
    let mut i: i16 = 0;
    while (i as i32) < cqm10_d(0xae235c) as i32 {
        let c = cqm10_clube(i);
        if let Some(d) = cqm10_divisao(c) {
            if n != cqm10_w(this + 0x3e) as i32
                && (d == cqm10_d(0x9cf5bc) || d == cqm10_d(0x9cf5c0) || d == cqm10_d(0x9cf5c4) || d == cqm10_d(0x9cf5c8))
                && cqm10_d(c + 0x1db) as i32 != -1
            {
                crate::comparacao::ramo("cqm_10 clube nas copas europeias");
                *((this + 0xb2) as *mut u8) = (*((this + 0xb2) as *const u8)).wrapping_add(1);
                cqm10_inscrever(this, n, c, 0x78);
                n += 1;
                if n == cqm10_w(this + 0x3e) as i32 {
                    break;
                }
            }
        }
        i = i.wrapping_add(1);
    }
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    ordenar(cqm10_d(this + 0xa7), n as u32, 6, 0x5221c0);
    crate::comparacao::ramo("cqm_10 inscritos da copa da liga");
    let mut k: i16 = 0;
    while (k as i32) < n {
        let e = cqm10_d(this + 0xa7) + k as usize * 6;
        let c = cqm10_d(e);
        let d = if c != 0 { cqm10_divisao(c) } else { None };
        *((e + 4) as *mut u8) = match d {
            Some(x) if x == cqm10_d(0x9cf5bc) => 1, // cqm10: a classe da Premier
            Some(x) if x == cqm10_d(0x9cf5c0) => 2,
            _ => 3,
        };
        k = k.wrapping_add(1);
    }
    if n != cqm10_w(this + 0x3e) as i32 {
        return (0, n);
    }
    if *((this + 0xb2) as *const i8) > 0xc {
        *((this + 0xb2) as *mut u8) = 0xc;
    }
    (1, n)
}

unsafe extern "thiscall" fn cqm10_copa_da_liga_inscritos(this: usize) -> i32 {
    cqm10_copa_da_liga(this).0
}

/// Lado a lado para as rotinas que alocam a lista +0xa7: as regiões `reg` (com o [0xb67a34]) a partir do mesmo estado; a
/// original deixa a lista dela em +0xa7, a cópia faz a sua, que é comparada nos `bytes(n)` primeiros bytes e liberada (o
/// +0xa7 volta à da original).
unsafe fn cqm10_comparar(nome: &str, this: usize, reg: &[(usize, usize)], o: impl FnOnce() -> i32, c: impl FnOnce() -> (i32, i32),
    bytes: impl Fn(i32, i32) -> usize) -> i32 {
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let nr = core::cell::Cell::new(0i32);
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(reg, || {
        let r = o();
        lo.set(cqm10_d(this + 0xa7));
        r
    }, || {
        let (r, n) = c();
        lr.set(cqm10_d(this + 0xa7));
        nr.set(n);
        core::ptr::write_unaligned((this + 0xa7) as *mut u32, lo.get() as u32);
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let k = bytes(a, nr.get());
    let lista = if lo != 0 && lr != 0 {
        core::slice::from_raw_parts(lo as *const u8, k) == core::slice::from_raw_parts(lr as *const u8, k)
    } else {
        lo == 0 && lr == 0
    };
    if lr != 0 && lr != lo {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(lr);
    }
    PLACAR.registrar(igual && a == b && lista, || format!("{}({:#x}) devolveu {} x {} memória {} lista {} ({} bytes)", nome, this, a, b, igual, lista, k));
    a
}

unsafe extern "thiscall" fn comparar_cqm10_copa_da_liga_inscritos(this: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> i32>(1781);
    cqm10_comparar("cqm10_copa_da_liga_inscritos", this, &[(this + 0x3e, 2), (this + 0xb2, 1), (0xb67a34, 4)], || o(this),
        || cqm10_copa_da_liga(this), |_, n| n.max(0) as usize * 6)
}

/// { 0x56b3c0 } (thiscall, ret 0, eng_auto_cup.cpp): `EngAutoCup::collect_entrants()`: +0x3e = 28; a lista (0x944e46 de
/// 0xa8 em +0xa7; sem memória, a caixa da linha 0x19f e −1) com a segunda metade dos 56 clubes escolhidos (o vetor de
/// ponteiros em +0xb2, a partir do 28º: +0x70), classe e situação 0. Devolve 1.
unsafe fn cqm10_trofeu(this: usize) -> (i32, i32) {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 0x1c);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let l = alocar(0xa8);
    core::ptr::write_unaligned((this + 0xa7) as *mut u32, l as u32);
    if l == 0 {
        aviso_em(0x9c4180, 0x19f);
        return (-1, 0);
    }
    crate::comparacao::ramo("cqm_10 segunda chave do trofeu da liga");
    let mut k: i16 = 0;
    if cqm10_w(this + 0x3e) > 0 {
        loop {
            let c = cqm10_d(cqm10_d(this + 0xb2) + 0x70 + k as usize * 4); // cqm10: o k-ésimo da segunda metade
            cqm10_inscrever(this, k as i32, c, 0);
            k = k.wrapping_add(1);
            if k >= cqm10_w(this + 0x3e) {
                break;
            }
        }
    }
    (1, k as i32)
}

unsafe extern "thiscall" fn cqm10_trofeu_inscritos(this: usize) -> i32 {
    cqm10_trofeu(this).0
}

unsafe extern "thiscall" fn comparar_cqm10_trofeu_inscritos(this: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> i32>(1782);
    cqm10_comparar("cqm10_trofeu_inscritos", this, &[(this + 0x3e, 2), (0xb67a34, 4)], || o(this), || cqm10_trofeu(this), |_, n| n.max(0) as usize * 6)
}

/// O clube que terminou em `lugar` (0 o campeão, 1 o vice) na última temporada da competição de id [id] (a competição do
/// banco: [0xae23d0] + 0x6b × id; 0x4afd80: push lugar; push competição; add esp, 8 -> cdecl).
unsafe fn cqm10_colocado(id: usize, lugar: u32) -> usize {
    let colocado: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x4afd80usize);
    colocado(cqm10_d(0xae23d0).wrapping_add(cqm10_d(id).wrapping_mul(0x6b)), lugar)
}

/// { 0x56d780 } (thiscall, ret 0, eng_charity.cpp): `EngCharity::select_teams()`: +0x3e = 2; a lista (0x944e46 de 0xc em
/// +0xa7; sem memória, a caixa da linha 0xc5 e −1). O campeão da Premier ([0x9cf5bc]; sem ele, a caixa 0xce e 0) e o da
/// Copa da Inglaterra ([0x9cf74c]; sem ele, 0xd7 e 0); se é o mesmo clube, o segundo vira o vice da Premier (sem ele,
/// 0xe3 e 0). Classe e situação 0 nos dois; devolve 1.
unsafe fn cqm10_supercopa(this: usize) -> (i32, i32) {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 2);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let l = alocar(0xc);
    core::ptr::write_unaligned((this + 0xa7) as *mut u32, l as u32);
    if l == 0 {
        aviso_em(0x9c425c, 0xc5);
        return (-1, 0);
    }
    let a = cqm10_colocado(0x9cf5bc, 0); // cqm10: o campeão inglês
    core::ptr::write_unaligned(l as *mut u32, a as u32);
    if a == 0 {
        aviso_em(0x9c425c, 0xce);
        return (0, 0);
    }
    let b = cqm10_colocado(0x9cf74c, 0);
    core::ptr::write_unaligned((l + 6) as *mut u32, b as u32);
    if b == 0 {
        aviso_em(0x9c425c, 0xd7);
        return (0, 0);
    }
    if a == b {
        crate::comparacao::ramo("cqm_10 dobradinha inglesa");
        let b = cqm10_colocado(0x9cf5bc, 1);
        core::ptr::write_unaligned((l + 6) as *mut u32, b as u32);
        if b == 0 {
            aviso_em(0x9c425c, 0xe3);
            return (0, 0);
        }
    }
    crate::comparacao::ramo("cqm_10 clubes da supercopa inglesa");
    *((l + 4) as *mut u8) = 0;
    *((l + 5) as *mut u8) = 0;
    *((l + 0xa) as *mut u8) = 0;
    *((l + 0xb) as *mut u8) = 0;
    (1, 2)
}

unsafe extern "thiscall" fn cqm10_supercopa_clubes(this: usize) -> i32 {
    cqm10_supercopa(this).0
}

unsafe extern "thiscall" fn comparar_cqm10_supercopa_clubes(this: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> i32>(1783);
    cqm10_comparar("cqm10_supercopa_clubes", this, &[(this + 0x3e, 2), (0xb67a34, 4)], || o(this), || cqm10_supercopa(this),
        |r, _| if r == 1 { 12 } else { 0 })
}

// ---------- 276a leva: os inscritos das copas francesas: a Copa da França, a Copa da Liga francesa e os dois clubes da Supercopa (fra_cup.cpp, fra_lge_cup.cpp, fra_super.cpp) ----------

unsafe fn cqm11_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqm11_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O clube `i` da tabela dos clubes ([0xae23b4], registros de 0x245 bytes).
unsafe fn cqm11_clube(i: i16) -> usize {
    cqm11_d(0xae23b4) + i as isize as usize * 0x245
}

/// O id da competição apontada por `c + o` (+0 do registro), ou None sem ela.
unsafe fn cqm11_comp(c: usize, o: usize) -> Option<usize> {
    let d = cqm11_d(c + o);
    if d == 0 { None } else { Some(cqm11_d(d)) }
}

/// Grava a inscrição `n` da lista (+0xa7, 6 bytes: o clube, +4 classe 0, +5 situação 0) e conta.
unsafe fn cqm11_inscrever(this: usize, n: &mut i32, clube: usize) {
    let e = cqm11_d(this + 0xa7) + *n as usize * 6;
    core::ptr::write_unaligned(e as *mut u32, clube as u32);
    *((e + 4) as *mut u8) = 0;
    *((e + 5) as *mut u8) = 0;
    *n += 1;
}

/// Uma passada pelos clubes (os `ate` primeiros, índice em 16 bits) que passam em `entra(clube, n)`; com `para`, a
/// passada acaba quando a lista enche (+0x3e).
unsafe fn cqm11_passada(this: usize, n: &mut i32, ate: impl Fn() -> i32, para: bool, entra: impl Fn(usize, i32) -> bool) {
    let mut i: i16 = 0;
    while (i as i32) < ate() {
        let c = cqm11_clube(i);
        if entra(c, *n) {
            cqm11_inscrever(this, n, c);
            if para && *n == cqm11_w(this + 0x3e) as i32 {
                return;
            }
        }
        i = i.wrapping_add(1);
    }
}

/// Os clubes da divisão [id] enquanto cabem (n < +0x3e), até encher.
unsafe fn cqm11_divisao(this: usize, n: &mut i32, id: usize) {
    cqm11_passada(this, n, || cqm11_d(0xae235c) as i32, true, |c, n| cqm11_comp(c, 0x57) == Some(cqm11_d(id)) && n < cqm11_w(this + 0x3e) as i32);
}

/// O limite dos amadores da Copa da França: 0xb8 em 2001 (+0x40 = 0x7d1), 0xb6 nos outros anos.
unsafe fn cqm11_limite_franca(this: usize) -> i32 {
    if cqm11_w(this + 0x40) == 0x7d1 { 0xb8 } else { 0xb6 }
}

/// { 0x5baa50 } (thiscall, ret 0, fra_cup.cpp): `FraCup::collect_entrants()`: +0x3e = 222; a lista (0x944e46 de 0x534 em
/// +0xa7; sem memória, a caixa da linha 0x16f e −1). Na ordem: todos os da divisão [0x9cf5d4] (sem conferir o espaço); os
/// da divisão [0x9cf764] ligados (+0x60) à [0x9cf5d8]; os franceses (+0x53 = [0x9cf300]) da [0x9cf764] sem ligação; os
/// franceses sem divisão que não são seleções (0x53b900), entre os clubes antes das 2 × [0xae2350] seleções do fim — estas
/// três até `cqm11_limite_franca`; a Segunda ([0x9cf5d0]) e a Primeira ([0x9cf5cc]) enquanto cabem. Devolve 1 com a lista
/// cheia, senão 0 (e quantos entraram, para o comparador).
unsafe fn cqm11_copa_da_franca(this: usize) -> (i32, i32) {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 0xde);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let l = alocar(0x534);
    core::ptr::write_unaligned((this + 0xa7) as *mut u32, l as u32);
    if l == 0 {
        aviso_em(0x9ccb28, 0x16f);
        return (-1, 0);
    }
    let mut n = 0i32;
    let todos = || cqm11_d(0xae235c) as i32;
    cqm11_passada(this, &mut n, todos, false, |c, _| cqm11_comp(c, 0x57) == Some(cqm11_d(0x9cf5d4)));
    cqm11_passada(this, &mut n, todos, true, |c, n| {
        cqm11_comp(c, 0x57) == Some(cqm11_d(0x9cf764)) && cqm11_comp(c, 0x60) == Some(cqm11_d(0x9cf5d8)) && n < cqm11_limite_franca(this)
    });
    cqm11_passada(this, &mut n, todos, true, |c, n| {
        cqm11_comp(c, 0x53) == Some(cqm11_d(0x9cf300)) && cqm11_comp(c, 0x57) == Some(cqm11_d(0x9cf764)) && cqm11_d(c + 0x60) == 0
            && n < cqm11_limite_franca(this)
    });
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    cqm11_passada(this, &mut n, || cqm11_d(0xae235c) as i32 - 2 * cqm11_d(0xae2350) as i32, true, |c, n| {
        cqm11_d(c + 0x53) != 0 && selecao(c) == 0 && cqm11_d(c + 0x57) == 0 && cqm11_comp(c, 0x53) == Some(cqm11_d(0x9cf300))
            && n < cqm11_limite_franca(this)
    });
    crate::comparacao::ramo("cqm_11 inscritos da copa da franca");
    cqm11_divisao(this, &mut n, 0x9cf5d0); // cqm11: a Segunda francesa na Copa da França
    cqm11_divisao(this, &mut n, 0x9cf5cc); // cqm11: a Primeira francesa na Copa da França
    ((n == cqm11_w(this + 0x3e) as i32) as i32, n)
}

unsafe extern "thiscall" fn cqm11_copa_da_franca_inscritos(this: usize) -> i32 {
    cqm11_copa_da_franca(this).0
}

/// Lado a lado para as rotinas que alocam a lista +0xa7 (como `cqm10_comparar`): as regiões `reg` (com o [0xb67a34]) a
/// partir do mesmo estado; a lista da cópia é comparada nos `bytes(devolução, n)` primeiros bytes e liberada.
unsafe fn cqm11_comparar(nome: &str, this: usize, reg: &[(usize, usize)], o: impl FnOnce() -> i32, c: impl FnOnce() -> (i32, i32),
    bytes: impl Fn(i32, i32) -> usize) -> i32 {
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let nr = core::cell::Cell::new(0i32);
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(reg, || {
        let r = o();
        lo.set(cqm11_d(this + 0xa7));
        r
    }, || {
        let (r, n) = c();
        lr.set(cqm11_d(this + 0xa7));
        nr.set(n);
        core::ptr::write_unaligned((this + 0xa7) as *mut u32, lo.get() as u32);
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let k = bytes(a, nr.get());
    let lista = if lo != 0 && lr != 0 {
        core::slice::from_raw_parts(lo as *const u8, k) == core::slice::from_raw_parts(lr as *const u8, k)
    } else {
        lo == 0 && lr == 0
    };
    if lr != 0 && lr != lo {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(lr);
    }
    PLACAR.registrar(igual && a == b && lista, || format!("{}({:#x}) devolveu {} x {} memória {} lista {} ({} bytes)", nome, this, a, b, igual, lista, k));
    a
}

unsafe extern "thiscall" fn comparar_cqm11_copa_da_franca_inscritos(this: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> i32>(1784);
    cqm11_comparar("cqm11_copa_da_franca_inscritos", this, &[(this + 0x3e, 2), (0xb67a34, 4)], || o(this), || cqm11_copa_da_franca(this),
        |_, n| n.clamp(0, 0xde) as usize * 6)
}

/// { 0x5bd160 } (thiscall, ret 0, fra_lge_cup.cpp): `FraLgeCup::collect_entrants()`: +0x3e = 54; a lista (0x944e46 de
/// 0x144 em +0xa7; sem memória, a caixa da linha 0x146 e −1). Na ordem: os da divisão [0x9cf5d4] que não vieram (+0x5b,
/// com competição) da [0x9cf5d8], até 16 em 2001 (+0x40 = 0x7d1) e 14 nos outros anos; os da [0x9cf5d4] que vieram da
/// [0x9cf5d8] enquanto a lista tem menos de 16; a Segunda ([0x9cf5d0]) e a Primeira ([0x9cf5cc]) enquanto cabem.
/// Devolve 1 com a lista cheia, senão 0.
unsafe fn cqm11_copa_da_liga(this: usize) -> (i32, i32) {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 0x36);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let l = alocar(0x144);
    core::ptr::write_unaligned((this + 0xa7) as *mut u32, l as u32);
    if l == 0 {
        aviso_em(0x9ccb98, 0x146);
        return (-1, 0);
    }
    let mut n = 0i32;
    let todos = || cqm11_d(0xae235c) as i32;
    let limite = if cqm11_w(this + 0x40) == 0x7d1 { 0x10 } else { 0xe };
    cqm11_passada(this, &mut n, todos, true, |c, n| {
        cqm11_comp(c, 0x57) == Some(cqm11_d(0x9cf5d4)) && cqm11_d(c + 0x5b) != 0 && cqm11_comp(c, 0x5b) != Some(cqm11_d(0x9cf5d8)) && n < limite
    });
    // a original testa o tamanho antes de cada clube: a passada para quando a lista chega a 16
    let mut i: i16 = 0;
    while (i as i32) < cqm11_d(0xae235c) as i32 && n * 6 < 0x60 {
        let c = cqm11_clube(i);
        if cqm11_comp(c, 0x57) == Some(cqm11_d(0x9cf5d4)) && cqm11_comp(c, 0x5b) == Some(cqm11_d(0x9cf5d8)) {
            cqm11_inscrever(this, &mut n, c);
            if n == cqm11_w(this + 0x3e) as i32 {
                break;
            }
        }
        i = i.wrapping_add(1);
    }
    crate::comparacao::ramo("cqm_11 inscritos da copa da liga francesa");
    cqm11_divisao(this, &mut n, 0x9cf5d0); // cqm11: a Segunda francesa na Copa da Liga
    cqm11_divisao(this, &mut n, 0x9cf5cc); // cqm11: a Primeira francesa na Copa da Liga
    ((n == cqm11_w(this + 0x3e) as i32) as i32, n)
}

unsafe extern "thiscall" fn cqm11_copa_da_liga_inscritos(this: usize) -> i32 {
    cqm11_copa_da_liga(this).0
}

unsafe extern "thiscall" fn comparar_cqm11_copa_da_liga_inscritos(this: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> i32>(1785);
    cqm11_comparar("cqm11_copa_da_liga_inscritos", this, &[(this + 0x3e, 2), (0xb67a34, 4)], || o(this), || cqm11_copa_da_liga(this),
        |_, n| n.clamp(0, 0x36) as usize * 6)
}

/// O clube que terminou em `lugar` (0 o campeão, 1 o vice) na última temporada da competição de id [id] (0x4afd80: push
/// lugar; push competição do banco [0xae23d0] + 0x6b × id; add esp, 8 -> cdecl).
unsafe fn cqm11_colocado(id: usize, lugar: u32) -> usize {
    let colocado: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x4afd80usize);
    colocado(cqm11_d(0xae23d0).wrapping_add(cqm11_d(id).wrapping_mul(0x6b)), lugar)
}

/// { 0x5bf770 } (thiscall, ret 0, fra_super.cpp): `FraSuper::select_teams()`: +0x3e = 2; a lista (0x944e46 de 0xc em
/// +0xa7; sem memória, a caixa da linha 0xc4 e −1). O campeão da Primeira ([0x9cf5cc]; sem ele, a caixa 0xcd e 0) e o
/// da Copa da França ([0x9cf70c]; sem ele, 0xd6 e 0); se é o mesmo clube, o segundo vira o vice da Primeira (sem ele, 0xe2
/// e 0). Classe e situação 0 nos dois; devolve 1.
unsafe fn cqm11_supercopa(this: usize) -> (i32, i32) {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 2);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let l = alocar(0xc);
    core::ptr::write_unaligned((this + 0xa7) as *mut u32, l as u32);
    if l == 0 {
        aviso_em(0x9ccc48, 0xc4);
        return (-1, 0);
    }
    let a = cqm11_colocado(0x9cf5cc, 0); // cqm11: o campeão francês
    core::ptr::write_unaligned(l as *mut u32, a as u32);
    if a == 0 {
        aviso_em(0x9ccc48, 0xcd);
        return (0, 0);
    }
    let b = cqm11_colocado(0x9cf70c, 0);
    core::ptr::write_unaligned((l + 6) as *mut u32, b as u32);
    if b == 0 {
        aviso_em(0x9ccc48, 0xd6);
        return (0, 0);
    }
    if a == b {
        crate::comparacao::ramo("cqm_11 dobradinha francesa");
        let b = cqm11_colocado(0x9cf5cc, 1);
        core::ptr::write_unaligned((l + 6) as *mut u32, b as u32);
        if b == 0 {
            aviso_em(0x9ccc48, 0xe2);
            return (0, 0);
        }
    }
    crate::comparacao::ramo("cqm_11 clubes da supercopa francesa");
    *((l + 4) as *mut u8) = 0;
    *((l + 5) as *mut u8) = 0;
    *((l + 0xa) as *mut u8) = 0;
    *((l + 0xb) as *mut u8) = 0;
    (1, 2)
}

unsafe extern "thiscall" fn cqm11_supercopa_clubes(this: usize) -> i32 {
    cqm11_supercopa(this).0
}

unsafe extern "thiscall" fn comparar_cqm11_supercopa_clubes(this: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> i32>(1786);
    cqm11_comparar("cqm11_supercopa_clubes", this, &[(this + 0x3e, 2), (0xb67a34, 4)], || o(this), || cqm11_supercopa(this),
        |r, _| if r == 1 { 12 } else { 0 })
}

// ---------- 276a leva: as supercopas e o Jogo das Estrelas: os clubes da Supercopa da Espanha e do Japão e o calendário do Jogo das Estrelas da MLS (spa_super.cpp, jap_super.cpp, usa_mls_all_stars.cpp) ----------

unsafe fn cqm12_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

/// O clube que terminou em `lugar` (0 o campeão, 1 o vice) na última temporada da competição de id [id] (0x4afd80: push
/// lugar; push competição do banco [0xae23d0] + 0x6b × id; add esp, 8 -> cdecl).
unsafe fn cqm12_colocado(id: usize, lugar: u32) -> usize {
    let colocado: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x4afd80usize);
    colocado(cqm12_d(0xae23d0).wrapping_add(cqm12_d(id).wrapping_mul(0x6b)), lugar)
}

/// O começo comum: +0x3e = 2 e a lista de dois (0x944e46 de 0xc em +0xa7); sem memória, a caixa `linha` e None.
unsafe fn cqm12_lista(this: usize, arquivo: usize, linha: u32) -> Option<usize> {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 2);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let l = alocar(0xc);
    core::ptr::write_unaligned((this + 0xa7) as *mut u32, l as u32);
    if l == 0 {
        aviso_em(arquivo, linha);
        return None;
    }
    Some(l)
}

/// O fim comum: classe e situação 0 nos dois; devolve 1.
unsafe fn cqm12_fim(l: usize) -> (i32, i32) {
    *((l + 4) as *mut u8) = 0;
    *((l + 5) as *mut u8) = 0;
    *((l + 0xa) as *mut u8) = 0;
    *((l + 0xb) as *mut u8) = 0;
    (1, 2)
}

/// Grava o clube `c` na vaga `v` da lista; sem clube, a caixa `linha` (fonte `arquivo`) e falso.
unsafe fn cqm12_vaga(l: usize, v: usize, c: usize, arquivo: usize, linha: u32) -> bool {
    core::ptr::write_unaligned((l + 6 * v) as *mut u32, c as u32);
    if c == 0 {
        aviso_em(arquivo, linha);
        return false;
    }
    true
}

/// { 0x858aa0 } (thiscall, ret 0, spa_super.cpp): `SpaSuper::select_teams()`: a lista (sem memória, a caixa da linha
/// 0xc2 e −1). O campeão da Copa do Rei ([0x9cf740]; sem ele, a caixa 0xcb e 0) e o da Liga ([0x9cf624]; sem ele, 0xd4
/// e 0); se é o mesmo clube, o primeiro vira o vice da Copa do Rei (sem ele, 0xe0 e 0). Devolve 1.
unsafe fn cqm12_espanha(this: usize) -> (i32, i32) {
    const F: usize = 0xa82a58;
    let l = match cqm12_lista(this, F, 0xc2) {
        Some(l) => l,
        None => return (-1, 0),
    };
    let a = cqm12_colocado(0x9cf740, 0); // cqm12: o campeão da Copa do Rei
    if !cqm12_vaga(l, 0, a, F, 0xcb) {
        return (0, 0);
    }
    let b = cqm12_colocado(0x9cf624, 0);
    if !cqm12_vaga(l, 1, b, F, 0xd4) {
        return (0, 0);
    }
    if a == b {
        crate::comparacao::ramo("cqm_12 dobradinha espanhola");
        if !cqm12_vaga(l, 0, cqm12_colocado(0x9cf740, 1), F, 0xe0) {
            return (0, 0);
        }
    }
    crate::comparacao::ramo("cqm_12 clubes da supercopa da espanha");
    cqm12_fim(l)
}

unsafe extern "thiscall" fn cqm12_supercopa_da_espanha_clubes(this: usize) -> i32 {
    cqm12_espanha(this).0
}

/// Lado a lado para as rotinas que alocam a lista +0xa7 (como `cqm10_comparar`): o +0x3e e o [0xb67a34] a partir do
/// mesmo estado; a lista da cópia é comparada nos 12 bytes (só com a devolução 1) e liberada.
unsafe fn cqm12_comparar(nome: &str, this: usize, o: impl FnOnce() -> i32, c: impl FnOnce() -> (i32, i32)) -> i32 {
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let reg = [(this + 0x3e, 2usize), (0xb67a34, 4)];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || {
        let r = o();
        lo.set(cqm12_d(this + 0xa7));
        r
    }, || {
        let (r, _) = c();
        lr.set(cqm12_d(this + 0xa7));
        core::ptr::write_unaligned((this + 0xa7) as *mut u32, lo.get() as u32);
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let k = if a == 1 { 12 } else { 0 };
    let lista = if lo != 0 && lr != 0 {
        core::slice::from_raw_parts(lo as *const u8, k) == core::slice::from_raw_parts(lr as *const u8, k)
    } else {
        lo == 0 && lr == 0
    };
    if lr != 0 && lr != lo {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(lr);
    }
    PLACAR.registrar(igual && a == b && lista, || format!("{}({:#x}) devolveu {} x {} memória {} lista {}", nome, this, a, b, igual, lista));
    a
}

unsafe extern "thiscall" fn comparar_cqm12_supercopa_da_espanha_clubes(this: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> i32>(1787);
    cqm12_comparar("cqm12_supercopa_da_espanha_clubes", this, || o(this), || cqm12_espanha(this))
}

/// { 0x666160 } (thiscall, ret 0, jap_super.cpp): `JapSuper::select_teams()`: a lista (sem memória, a caixa da linha
/// 0xc1 e −1). O campeão da J1 ([0x9cf65c]; sem ele, 0xca e 0) — sem nação (+0x53 = 0), o vice (sem ele, 0xd6 e 0); o da
/// Copa do Imperador ([0x9cf660]; sem ele, 0xe0 e 0) — sem nação, o vice (sem ele, 0xec e 0). Se é o mesmo clube, o
/// segundo vira o vice da Copa do Imperador (sem ele, 0xf9 e 0) e, se ainda é o mesmo, o primeiro vira o vice da J1
/// (sem ele, 0x105 e 0). Devolve 1.
unsafe fn cqm12_japao(this: usize) -> (i32, i32) {
    const F: usize = 0x9eaf4c;
    let l = match cqm12_lista(this, F, 0xc1) {
        Some(l) => l,
        None => return (-1, 0),
    };
    let a = cqm12_colocado(0x9cf65c, 0); // cqm12: o campeão da J1
    if !cqm12_vaga(l, 0, a, F, 0xca) {
        return (0, 0);
    }
    if cqm12_d(a + 0x53) == 0 && !cqm12_vaga(l, 0, cqm12_colocado(0x9cf65c, 1), F, 0xd6) {
        return (0, 0);
    }
    let b = cqm12_colocado(0x9cf660, 0);
    if !cqm12_vaga(l, 1, b, F, 0xe0) {
        return (0, 0);
    }
    if cqm12_d(b + 0x53) == 0 && !cqm12_vaga(l, 1, cqm12_colocado(0x9cf660, 1), F, 0xec) {
        return (0, 0);
    }
    if cqm12_d(l) == cqm12_d(l + 6) {
        crate::comparacao::ramo("cqm_12 dobradinha japonesa");
        if !cqm12_vaga(l, 1, cqm12_colocado(0x9cf660, 1), F, 0xf9) {
            return (0, 0);
        }
        if cqm12_d(l) == cqm12_d(l + 6) && !cqm12_vaga(l, 0, cqm12_colocado(0x9cf65c, 1), F, 0x105) {
            return (0, 0);
        }
    }
    crate::comparacao::ramo("cqm_12 clubes da supercopa do japao");
    cqm12_fim(l)
}

unsafe extern "thiscall" fn cqm12_supercopa_do_japao_clubes(this: usize) -> i32 {
    cqm12_japao(this).0
}

unsafe extern "thiscall" fn comparar_cqm12_supercopa_do_japao_clubes(this: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> i32>(1788);
    cqm12_comparar("cqm12_supercopa_do_japao_clubes", this, || o(this), || cqm12_japao(this))
}

/// { 0x90a400 } (thiscall, ret 0x10, usa_mls_all_stars.cpp; a virtual +0x3c do Jogo das Estrelas): `UsaMlsAllStars::
/// create_round_table(fase, &rodadas, &extra, &saida)`: fora da fase −1 (o byte), 0. Na −1: saida (se pedida) = 0,
/// rodadas = 1, extra = 0; a tabela (0x944e46 de 0x68; sem memória, a caixa da linha 0x9a e 0) com a rodada única: o
/// sorteio em 15 do mês 5 (junho) levado ao dia da semana 5, o jogo em 2 do mês 7 (agosto) levado ao dia 6, turno 1,
/// mando 0 (0x521e60 e 0x521eb0, cdecl, com o ano +0x40); nome 0, +0xd = 0x20, categoria 8, 2 times, 1 jogo, 2 novos,
/// 1 jogo por confronto, sem prêmio.
unsafe extern "thiscall" fn cqm12_jogo_das_estrelas_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    if fase as u8 != 0xff {
        return 0;
    }
    if !saida.is_null() {
        *saida = 0;
    }
    *rodadas = 1;
    *extra = 0;
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let t = alocar(0x68);
    if t == 0 {
        aviso_em(0xaa20a0, 0x9a);
        return 0;
    }
    crate::comparacao::ramo("cqm_12 tabela do jogo das estrelas");
    let ano = core::ptr::read_unaligned((this + 0x40) as *const i16) as i32 as u32;
    let sorteio: unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(0x521e60usize);
    let jogo: unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(0x521eb0usize);
    sorteio(t, 0, 0xf, 5, 0, 5, ano);
    jogo(t, 0, 2, 7, 0, 6, 1, ano, 0); // cqm12: o Jogo das Estrelas em 2 de agosto
    let w = |o: usize, v: u16| core::ptr::write_unaligned((t + o) as *mut u16, v);
    let d = |o: usize, v: u32| core::ptr::write_unaligned((t + o) as *mut u32, v);
    w(0x18, 2);
    w(0x1c, 2);
    w(7, 0);
    w(9, 0);
    w(0xb, 0);
    w(0xd, 0x20);
    *((t + 0x17) as *mut u8) = 8;
    w(0x1a, 1);
    w(0xf, 0);
    w(0x1e, 0);
    *((t + 0x20) as *mut u8) = 0;
    *((t + 0x21) as *mut u8) = 1;
    *((t + 0x22) as *mut u8) = 0;
    d(0x5c, 0);
    d(0x60, 0);
    d(0x64, 0);
    t
}

/// Os bytes que a original grava na rodada: +0..+0x10, +0x13..+0x22 e +0x5c..+0x67.
unsafe fn cqm12_gravados(t: usize) -> Vec<u8> {
    let mut v = Vec::new();
    for &(a, b) in &[(0usize, 0x11usize), (0x13, 0x23), (0x5c, 0x68)] {
        v.extend_from_slice(core::slice::from_raw_parts((t + a) as *const u8, b - a));
    }
    v
}

/// Lado a lado: a original com as saídas de quem chama, a cópia com saídas próprias que começam iguais, a partir do
/// mesmo [0xb67a34]; as saídas e os bytes gravados das duas tabelas têm de ser iguais (a da cópia é liberada). Fora da
/// fase −1, só a original (nenhuma grava nada).
unsafe extern "thiscall" fn comparar_cqm12_jogo_das_estrelas_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1789);
    if fase as u8 != 0xff {
        let a = o(this, fase, rodadas, extra, saida);
        PLACAR.registrar(a == 0, || format!("cqm12_jogo_das_estrelas_tabela(fase {}) devolveu {:#x}", fase as u8 as i8, a));
        return a;
    }
    let (mut r2, mut e2) = (*rodadas, *extra);
    let mut s2 = if saida.is_null() { 0 } else { *saida };
    let reg = [(0xb67a34usize, 4usize)];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || o(this, fase, rodadas, extra, saida),
        || cqm12_jogo_das_estrelas_tabela(this, fase, &mut r2, &mut e2, if saida.is_null() { core::ptr::null_mut() } else { &mut s2 }));
    let saidas = *rodadas == r2 && *extra == e2 && (saida.is_null() || *saida == s2);
    let tabela = if a != 0 && b != 0 { cqm12_gravados(a) == cqm12_gravados(b) } else { a == 0 && b == 0 };
    if b != 0 {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(b);
    }
    PLACAR.registrar(igual && saidas && tabela, || format!("cqm12_jogo_das_estrelas_tabela: saídas {} tabela {}", saidas, tabela));
    a
}

// ---------- 276a leva: o dia das copas: a virada da Copa da Liga inglesa, as inscrições da Copa da Inglaterra e a convocação do Jogo das Estrelas (eng_cc_cup.cpp, eng_fa_cup.cpp, usa_mls_all_stars.cpp) ----------

unsafe fn cqn10_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqn10_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// A primeira dword de Date(dia, mês, ano, dia da semana) (0x549f70: push semana; push ano; push mês; push dia; lea ecx,
/// local -> thiscall, ret 0x10): a palavra de baixo é o dia do ano, a de cima o ano.
unsafe fn cqn10_data(dia: u32, mes: u32, ano: u32, semana: u32) -> u32 {
    let construir: unsafe extern "thiscall" fn(*mut [u32; 2], u32, u32, u32, u32) -> usize = core::mem::transmute(0x549f70usize);
    let mut d = [0u32; 2];
    construir(&mut d, dia, mes, ano, semana);
    d[0]
}

/// A data `d` (dword de `cqn10_data`) é `hoje` (o Date de quem chama: +0 o dia do ano, +2 o ano)? A original compara o
/// ano e depois o dia.
unsafe fn cqn10_e_hoje(d: u32, hoje: usize) -> bool {
    (d >> 16) as u16 == cqn10_w(hoje + 2) as u16 && d as u16 == cqn10_w(hoje) as u16
}

/// Libera as tabelas da temporada antes de refazer: a lista dos inscritos (+0xa7), a das rodadas (+0xa3) e os confrontos
/// por rodada (+0xad, um vetor por rodada, +0x3c rodadas), cada um se existe (0x9452ca, cdecl). Os ponteiros ficam como
/// estão (as rotinas seguintes os refazem).
unsafe fn cqn10_liberar(this: usize) {
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    if cqn10_d(this + 0xa7) != 0 {
        soltar(cqn10_d(this + 0xa7));
    }
    if cqn10_d(this + 0xa3) != 0 {
        soltar(cqn10_d(this + 0xa3));
    }
    if cqn10_d(this + 0xad) != 0 {
        let mut i = 0i32;
        while i < cqn10_w(this + 0x3c) as i32 {
            let p = cqn10_d(cqn10_d(this + 0xad) + i as usize * 4);
            if p != 0 {
                soltar(p);
            }
            i += 1;
        }
        soltar(cqn10_d(this + 0xad));
    }
}

/// A virtual `o` da copa (thiscall, sem argumentos; mov ecx, copa; call [vtable + o]).
unsafe fn cqn10_virtual(this: usize, o: usize) -> i32 {
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(cqn10_d(cqn10_d(this) + o));
    f(this)
}

/// A atualização da copa (0x51f890: push fase; push hoje; mov ecx, copa -> thiscall, ret 8), cuja devolução sai.
unsafe fn cqn10_base(this: usize, hoje: usize, fase: u32) -> u32 {
    let base: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x51f890usize);
    base(this, hoje, fase)
}

/// { 0x56caf0 } (thiscall, ret 8, eng_cc_cup.cpp; a virtual +0x14 da Copa da Liga): `EngCcCup::update_for_date(hoje,
/// fase)`. Uma vez (+0x45 = 0), quando o ano do jogo ([0xde1f58] +0x439, palavra sem sinal) chega ao da copa (+0x40, com
/// sinal) e a fase não é 0: libera as tabelas (`cqn10_liberar`), +0x45 = 1, os inscritos (0x56ccc0, thiscall), as
/// rodadas (virtual +0x8c), a árvore dos confrontos (virtual +0x94) e a ordem prevista (virtual +0x5c); uma devolução
/// negativa dá a caixa (linhas 0x161, 0x168, 0x16f) e 0. Sempre, no fim, a atualização da copa.
unsafe extern "thiscall" fn cqn10_copa_da_liga_dia(this: usize, hoje: usize, fase: u32) -> u32 {
    if cqn10_d(this + 0x45) == 0 && cqn10_w(cqn10_d(0xde1f58) + 0x439) as u16 as i32 == cqn10_w(this + 0x40) as i32 && fase != 0 {
        cqn10_liberar(this);
        crate::comparacao::ramo("cqn_10 virada da copa da liga");
        core::ptr::write_unaligned((this + 0x45) as *mut u32, 1); // cqn10: a Copa da Liga refeita uma vez
        let inscritos: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x56ccc0usize);
        // ponytail: nas caixas a original zera só o al (o resto do eax vem da caixa); quem chama lê o byte
        if inscritos(this) < 0 {
            aviso_em(0x9c4224, 0x161);
            return 0;
        }
        if cqn10_virtual(this, 0x8c) < 0 {
            aviso_em(0x9c4224, 0x168);
            return 0;
        }
        if cqn10_virtual(this, 0x94) < 0 {
            aviso_em(0x9c4224, 0x16f);
            return 0;
        }
        cqn10_virtual(this, 0x5c);
    }
    cqn10_base(this, hoje, fase)
}

// ponytail: a virada libera e refaz as tabelas e a atualização da copa joga e sorteia: não rodam duas vezes. Só a
// original; o plano: +0x45 vai a 1 quando a condição vale, senão fica.
unsafe extern "thiscall" fn comparar_cqn10_copa_da_liga_dia(this: usize, hoje: usize, fase: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(1790);
    let antes = cqn10_d(this + 0x45);
    let vira = antes == 0 && cqn10_w(cqn10_d(0xde1f58) + 0x439) as u16 as i32 == cqn10_w(this + 0x40) as i32 && fase != 0;
    let r = o(this, hoje, fase);
    let depois = cqn10_d(this + 0x45);
    PLACAR.registrar(depois == if vira { 1 } else { antes }, || format!("cqn10_copa_da_liga_dia({:#x}) vira {} +0x45 {} -> {}", this, vira, antes, depois));
    r
}

/// { 0x570550 } (thiscall, ret 8, eng_fa_cup.cpp; a virtual +0x14 da Copa da Inglaterra; quadro SEH pela notícia do
/// caminho de 1999, sem destrutor no caminho normal): `EngFaCup::update_for_date(hoje, fase)`. Com +0x45 = 0 e a fase
/// diferente de 0: em Date(1, 9, 1999) (o mês 9 é outubro) com +0xb2 = 0, a pergunta ao técnico humano sobre a entrada na
/// 5ª rodada (só em 1999: a original pelo trampolim); em Date(3, 9, ano de hoje) (3 de outubro), libera as tabelas, +0x45
/// = 1, os inscritos (0x56fab0, thiscall), as rodadas (virtual +0x8c), a árvore (virtual +0x94) — uma devolução negativa
/// dá a caixa (linhas 0x3d6, 0x3dd, 0x3e4) e 1 —, a ordem prevista (virtual +0x5c) e o sorteio da 1ª rodada (0x51c800:
/// push 0; mov ecx, copa -> thiscall, ret 4). Sempre, no fim, a atualização da copa.
unsafe extern "thiscall" fn cqn10_copa_da_inglaterra_dia(this: usize, hoje: usize, fase: u32) -> u32 {
    if cqn10_d(this + 0x45) == 0 && fase != 0 {
        if cqn10_e_hoje(cqn10_data(1, 9, 0x7cf, 0xffff_ffff), hoje) && *((this + 0xb2) as *const u8) == 0 {
            // ponytail: a pergunta de 1999 monta e entrega uma notícia (só nesse dia, fora de qualquer carreira nova)
            return orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(1791)(this, hoje, fase);
        }
        // ponytail: o ano vai no ax com o resto do eax nos bytes altos; a 0x549f70 o lê como palavra
        if cqn10_e_hoje(cqn10_data(3, 9, cqn10_w(hoje + 2) as u16 as u32, 0xffff_ffff), hoje) { // cqn10: as inscrições da Copa da Inglaterra em 3/10
            cqn10_liberar(this);
            crate::comparacao::ramo("cqn_10 inscricoes da copa da inglaterra");
            core::ptr::write_unaligned((this + 0x45) as *mut u32, 1);
            let inscritos: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x56fab0usize);
            if inscritos(this) < 0 {
                aviso_em(0x9c42cc, 0x3d6);
                return 1;
            }
            if cqn10_virtual(this, 0x8c) < 0 {
                aviso_em(0x9c42cc, 0x3dd);
                return 1;
            }
            if cqn10_virtual(this, 0x94) < 0 {
                aviso_em(0x9c42cc, 0x3e4);
                return 1;
            }
            cqn10_virtual(this, 0x5c);
            let sortear: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x51c800usize);
            sortear(this, 0);
        }
    }
    cqn10_base(this, hoje, fase)
}

// ponytail: como a da Copa da Liga, só a original; o plano: +0x45 vai a 1 em 3 de outubro (com +0x45 = 0 e a fase), fora
// do caminho de 1999.
unsafe extern "thiscall" fn comparar_cqn10_copa_da_inglaterra_dia(this: usize, hoje: usize, fase: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(1791);
    let antes = cqn10_d(this + 0x45);
    let vira = antes == 0 && fase != 0 && cqn10_e_hoje(cqn10_data(3, 9, cqn10_w(hoje + 2) as u16 as u32, 0xffff_ffff), hoje)
        && !(cqn10_e_hoje(cqn10_data(1, 9, 0x7cf, 0xffff_ffff), hoje) && *((this + 0xb2) as *const u8) == 0);
    let r = o(this, hoje, fase);
    let depois = cqn10_d(this + 0x45);
    PLACAR.registrar(!vira || depois == 1, || format!("cqn10_copa_da_inglaterra_dia({:#x}) vira {} +0x45 {} -> {}", this, vira, antes, depois));
    r
}

/// { 0x90a900 } (thiscall, ret 8, usa_mls_all_stars.cpp; a virtual +0x14 do Jogo das Estrelas; quadro SEH pela Date
/// local, sem destrutor): `UsaMlsAllStars::update_for_date(hoje, fase)`: com a fase diferente de 0, no dia de Date(26, 6,
/// ano de hoje, dia da semana 6) (o sábado de 26 de julho em diante [inferência]), a convocação das duas seleções
/// (0x90a980, mov ecx -> thiscall, ret 0); depois a atualização da copa.
unsafe extern "thiscall" fn cqn10_jogo_das_estrelas_dia(this: usize, hoje: usize, fase: u32) -> u32 {
    // ponytail: o ano vai num registrador com lixo nos 16 bits altos; a 0x549f70 o lê como palavra
    let ano = cqn10_w(hoje + 2) as u16 as u32;
    let d = cqn10_data(0x1a, 6, ano, 6); // cqn10: a convocação do Jogo das Estrelas
    if fase != 0 && cqn10_e_hoje(d, hoje) {
        crate::comparacao::ramo("cqn_10 convocacao do jogo das estrelas");
        let convocar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x90a980usize);
        convocar(this);
    }
    cqn10_base(this, hoje, fase)
}

// ponytail: a convocação mexe nos elencos e a atualização da copa joga: não rodam duas vezes. Só a original roda.
unsafe extern "thiscall" fn comparar_cqn10_jogo_das_estrelas_dia(this: usize, hoje: usize, fase: u32) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(1792)(this, hoje, fase)
}

// ---------- 276a leva: o calendário das copas do Japão e dos EUA: as rodadas da Copa da Liga japonesa, da Copa do Imperador e da Copa dos EUA (jap_j_cup.cpp, jap_emp_cup.cpp, usa_open_cup.cpp) ----------


/// Uma rodada de copa (registro de 0x68 bytes, docs/.md §2.4): o sorteio (dia, mês com 0 =
/// janeiro, anos a somar ao ano da copa +0x40, dia da semana ou −1) pela 0x521e60; o jogo (o mesmo, mais o turno em +6
/// e o mando em +0x13) pela 0x521eb0; depois os campos: +7 o nome da rodada, +9 o índice, +0xb os bits do sorteio, +0xd
/// e +0xf os desempates (jogo único e ida e volta), +0x17 a categoria copiada em cada jogo, +0x18/+0x1a/+0x1c/+0x1e
/// times, jogos, times novos e inscritos que já entraram, +0x20/+0x21/+0x22 replay, jogos por confronto e dias entre
/// eles, +0x5c/+0x60/+0x64 os prêmios (chegar, vencer, perder).
#[derive(Clone, Copy)]
struct Cqn11R {
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
const fn cqn11r(sorteio: (u32, u32, u32, u32), jogo: (u32, u32, u32, u32, u32, u32), codigo: u16, bits: (u16, u16, u16), categoria: u8,
    times: (u16, u16, u16, u16), confronto: (u8, u8, u8), premios: (u32, u32, u32)) -> Cqn11R {
    Cqn11R { sorteio, jogo, codigo, bits, categoria, times, confronto, premios }
}

/// Grava a rodada `i` da tabela `t` (ano-base `ano`, a palavra +0x40 da copa com sinal): as duas datas (0x521e60 e
/// 0x521eb0, cdecl, 7 e 9 argumentos; add esp, 0x1c / 0x24) e os campos.
unsafe fn cqn11_rodada(t: usize, i: u32, r: &Cqn11R, ano: u32) {
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
unsafe fn cqn11_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32, tabela: &[Cqn11R], arquivo: usize, linha: u32, nome: &'static str) -> usize {
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
        aviso_em(arquivo, linha);
        return 0;
    }
    crate::comparacao::ramo(nome);
    let ano = core::ptr::read_unaligned((this + 0x40) as *const i16) as i32 as u32;
    for (i, r) in tabela.iter().enumerate() {
        cqn11_rodada(t, i as u32, r, ano);
    }
    t
}

/// Os bytes que a original grava em cada rodada (o resto da memória alocada fica como veio): +0..+0x10 (as datas e os
/// campos até o desempate de ida e volta), +0x13..+0x22 (o mando até os dias entre os jogos) e +0x5c..+0x67 (os prêmios).
unsafe fn cqn11_gravados(t: usize, n: i16) -> Vec<u8> {
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
unsafe fn cqn11_comparar(nome: &str, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32,
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
    let tabela = if a != 0 && b != 0 { cqn11_gravados(a, *rodadas) == cqn11_gravados(b, r2) } else { a == 0 && b == 0 };
    if b != 0 {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(b);
    }
    PLACAR.registrar(igual && saidas && tabela, || format!("{}(fase {}): saídas {} ({} x {}) tabela {}", nome, fase as u8 as i8, saidas, *rodadas, r2, tabela));
    a
}

/// A Copa da Liga japonesa (J League Cup): cinco rodadas de fevereiro a novembro, as quatro primeiras em ida e volta (7
/// dias entre os jogos, desempate de ida e volta 0x83).
const CQN11_COPA_DA_LIGA_JAPONESA: [Cqn11R; 5] = [
    cqn11r((0xa, 1, 0, 0xffff_ffff), (0xc, 3, 0, 2, 2, 0), 0xa, (0, 0, 0x83), 4, (0x18, 0xc, 0x18, 0), (0, 2, 7), (0, 0xffff_ffff, 0)), // cqn11: a 1ª rodada da Copa da Liga japonesa
    cqn11r((0x14, 3, 0, 3), (5, 6, 0, 2, 2, 0), 0x14, (0, 0, 0x83), 4, (0x10, 8, 4, 0x18), (0, 2, 7), (0, 0xffff_ffff, 0)),
    cqn11r((0xd, 6, 0, 3), (0x1e, 7, 0, 2, 2, 0), 0x78, (0, 0, 0x83), 4, (8, 4, 0, 0), (0, 2, 7), (0, 0xffff_ffff, 0)),
    cqn11r((7, 8, 0, 3), (0xb, 9, 0, 2, 2, 0), 0x82, (0, 0, 0x83), 6, (4, 2, 0, 0), (0, 2, 7), (0, 0xffff_ffff, 0)),
    cqn11r((0x13, 9, 0, 3), (4, 0xa, 0, 5, 1, 4), 0x96, (0, 3, 0), 6, (2, 1, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
];

/// { 0x665640 } (thiscall, ret 0x10, jap_j_cup.cpp; a virtual +0x3c da Copa da Liga japonesa): `JapJCup::
/// create_round_table(fase, &rodadas, &extra, &saida)`: na fase −1, as 5 rodadas; a caixa da linha 0x9e sem memória.
unsafe extern "thiscall" fn cqn11_copa_da_liga_japonesa_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    cqn11_tabela(this, fase, rodadas, extra, saida, &CQN11_COPA_DA_LIGA_JAPONESA, 0x9eaf14, 0x9e, "cqn_11 tabela da copa da liga japonesa")
}

unsafe extern "thiscall" fn comparar_cqn11_copa_da_liga_japonesa_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1793);
    cqn11_comparar("cqn11_copa_da_liga_japonesa_tabela", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| cqn11_copa_da_liga_japonesa_tabela(this, fase, r, e, s))
}

/// A Copa do Imperador: sete rodadas em jogo único de novembro ao começo de janeiro do ano seguinte, sorteio por sementes
/// com grupos separados (+0xd = 0x83).
const CQN11_COPA_DO_IMPERADOR: [Cqn11R; 7] = [
    cqn11r((1, 0xa, 0, 6), (0x1d, 0xa, 0, 6, 1, 0), 0xa, (0, 0x83, 0), 4, (0x40, 0x20, 0x40, 0), (0, 1, 0), (0, 0xffff_ffff, 0)), // cqn11: a 1ª rodada da Copa do Imperador
    cqn11r((0x1e, 0xa, 0, 0), (6, 0xb, 0, 6, 1, 0), 0x14, (0, 0x83, 0), 4, (0x20, 0x10, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
    cqn11r((7, 0xb, 0, 0), (0xd, 0xb, 0, 6, 1, 0), 0x1e, (0, 0x83, 0), 4, (0x20, 0x10, 0x10, 0x40), (0, 1, 0), (0, 0xffff_ffff, 0)),
    cqn11r((0xe, 0xb, 0, 0), (0x14, 0xb, 0, 6, 1, 0), 0x28, (0, 0x83, 0), 4, (0x10, 8, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
    cqn11r((0x15, 0xb, 0, 0), (0x17, 0xb, 0, 2, 2, 0), 0x78, (0, 0x83, 0), 6, (8, 4, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
    cqn11r((0x18, 0xb, 0, 3), (0x1b, 0xb, 0, 6, 1, 0), 0x82, (0, 0x83, 0), 6, (4, 2, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
    cqn11r((0x1c, 0xb, 0, 0), (2, 0, 1, 5, 1, 4), 0x96, (0, 0x83, 0), 6, (2, 1, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
];

/// { 0x661ff0 } (thiscall, ret 0x10, jap_emp_cup.cpp; a virtual +0x3c da Copa do Imperador): `JapEmpCup::
/// create_round_table(fase, &rodadas, &extra, &saida)`: na fase −1, as 7 rodadas; a caixa da linha 0xa0 sem memória.
unsafe extern "thiscall" fn cqn11_copa_do_imperador_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    cqn11_tabela(this, fase, rodadas, extra, saida, &CQN11_COPA_DO_IMPERADOR, 0x9eab90, 0xa0, "cqn_11 tabela da copa do imperador")
}

unsafe extern "thiscall" fn comparar_cqn11_copa_do_imperador_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1794);
    cqn11_comparar("cqn11_copa_do_imperador_tabela", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| cqn11_copa_do_imperador_tabela(this, fase, r, e, s))
}

/// A Copa dos EUA (US Open Cup): cinco rodadas em jogo único de junho a outubro; na 1ª rodada o sorteio leva também o
/// bit 0x400 (+0xd = 0x483).
const CQN11_COPA_DOS_EUA: [Cqn11R; 5] = [
    cqn11r((1, 5, 0, 3), (0xe, 5, 0, 2, 2, 0), 0x14, (0, 0x483, 0), 4, (0x20, 0x10, 0x20, 0), (0, 0, 0), (0, 0xffff_ffff, 0)),
    cqn11r((0xf, 5, 0, 3), (0x19, 5, 0, 6, 1, 0), 0x1e, (0, 0x83, 0), 4, (0x10, 8, 0, 0), (0, 0, 0), (0, 0xffff_ffff, 0)),
    cqn11r((0x1a, 5, 0, 0), (9, 7, 0, 2, 2, 0), 0x78, (0, 0x83, 0), 6, (8, 4, 0, 0), (0, 0, 0), (0, 0xffff_ffff, 0)),
    cqn11r((0xa, 7, 0, 3), (0x17, 7, 0, 2, 2, 0), 0x82, (0, 0x83, 0), 6, (4, 2, 0, 0), (0, 0, 0), (0, 0xffff_ffff, 0)), // cqn11: a semifinal da Copa dos EUA
    cqn11r((0x18, 7, 0, 3), (0x16, 9, 0, 6, 1, 4), 0x96, (0, 0x83, 0), 6, (2, 1, 0, 0), (0, 0, 0), (0, 0xffff_ffff, 0)),
];

/// { 0x90b170 } (thiscall, ret 0x10, usa_open_cup.cpp; a virtual +0x3c da Copa dos EUA): `UsaOpenCup::
/// create_round_table(fase, &rodadas, &extra, &saida)`: na fase −1, as 5 rodadas; a caixa da linha 0x9c sem memória.
unsafe extern "thiscall" fn cqn11_copa_dos_eua_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    cqn11_tabela(this, fase, rodadas, extra, saida, &CQN11_COPA_DOS_EUA, 0xaa20e0, 0x9c, "cqn_11 tabela da copa dos eua")
}

unsafe extern "thiscall" fn comparar_cqn11_copa_dos_eua_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1795);
    cqn11_comparar("cqn11_copa_dos_eua_tabela", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| cqn11_copa_dos_eua_tabela(this, fase, r, e, s))
}
// ---------- 276a leva: os inscritos das copas do Japão e dos EUA: a Copa da Liga japonesa, a Copa dos EUA e a Copa do Imperador (jap_j_cup.cpp, usa_open_cup.cpp, jap_emp_cup.cpp) ----------

unsafe fn cqn12_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqn12_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O clube `i` da tabela dos clubes ([0xae23b4], registros de 0x245 bytes).
unsafe fn cqn12_clube(i: i32) -> usize {
    cqn12_d(0xae23b4).wrapping_add((i as isize * 0x245) as usize)
}

/// O id da competição apontada por `c + o` (+0 do registro), ou None sem ela.
unsafe fn cqn12_comp(c: usize, o: usize) -> Option<usize> {
    let d = cqn12_d(c + o);
    if d == 0 { None } else { Some(cqn12_d(d)) }
}

/// Grava a inscrição `k` da lista (+0xa7, 6 bytes: o clube, +4 a classe, +5 a situação 0).
unsafe fn cqn12_inscrever(this: usize, k: i32, clube: usize, classe: u8) {
    let e = cqn12_d(this + 0xa7) + k as usize * 6;
    core::ptr::write_unaligned(e as *mut u32, clube as u32);
    *((e + 4) as *mut u8) = classe;
    *((e + 5) as *mut u8) = 0;
}

/// O sorteio de 0 a n − 1 (0x90cfc0: push n; add esp, 4 -> cdecl).
unsafe fn cqn12_sortear(n: i32) -> i32 {
    let sortear: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    sortear(n)
}

/// Ordena `n` clubes do vetor pela reputação e embaralha os melhores (0x4afbd0: push escolhas; push n; push vetor; add
/// esp, 0xc -> cdecl; já em Rust).
unsafe fn cqn12_embaralhar(v: *mut usize, n: i32, escolhas: u32) {
    let f: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(0x4afbd0usize);
    // ponytail: a original empurra a conta com lixo nos 16 bits altos; a 0x4afbd0 lê a palavra
    f(v as usize, n as i16 as u16 as u32, escolhas);
}

/// O qsort de `n` inscritos a partir de `base` (0x945333 com 0x522190: push 0x522190; push 6; push n; push base; add
/// esp, 0x10 -> cdecl; 0x522190 já em Rust).
unsafe fn cqn12_ordenar(base: usize, n: u32) {
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    ordenar(base, n, 6, 0x522190);
}

/// { 0x665a00 } (thiscall, ret 0, jap_j_cup.cpp): `JapJCup::collect_entrants()`: +0x3e = 28; a lista (0x944e46 de 0xa8 em
/// +0xa7; sem memória, a caixa da linha 0x11a e −1). Os clubes da J2 ([0x9cf768]) enquanto cabem, classe 0; os da J1
/// ([0x9cf65c]) enquanto cabem, cada um com a classe sorteada de 0 a 19 (0x90cfc0(20), depois de gravar o clube); o
/// qsort (0x522190) das 16 inscrições a partir da 11ª (+0x42). Devolve 1 com a lista cheia, senão 0.
unsafe fn cqn12_copa_da_liga(this: usize) -> (i32, i32) {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 0x1c);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let l = alocar(0xa8);
    core::ptr::write_unaligned((this + 0xa7) as *mut u32, l as u32);
    if l == 0 {
        aviso_em(0x9eaf14, 0x11a);
        return (-1, 0);
    }
    let mut n = 0i32;
    let mut i: i16 = 0;
    while (i as i32) < cqn12_d(0xae235c) as i32 {
        let c = cqn12_clube(i as i32);
        if cqn12_comp(c, 0x57) == Some(cqn12_d(0x9cf768)) && n < cqn12_w(this + 0x3e) as i32 {
            cqn12_inscrever(this, n, c, 0);
            n += 1;
            if n == cqn12_w(this + 0x3e) as i32 {
                break;
            }
        }
        i = i.wrapping_add(1);
    }
    let mut i: i16 = 0;
    while (i as i32) < cqn12_d(0xae235c) as i32 {
        let c = cqn12_clube(i as i32);
        if cqn12_comp(c, 0x57) == Some(cqn12_d(0x9cf65c)) && n < cqn12_w(this + 0x3e) as i32 {
            let e = cqn12_d(this + 0xa7) + n as usize * 6;
            core::ptr::write_unaligned(e as *mut u32, c as u32);
            crate::comparacao::ramo("cqn_12 semente da j1");
            let s = cqn12_sortear(0x14); // cqn12: a classe sorteada dos clubes da J1
            *((e + 4) as *mut u8) = s as u8;
            *((e + 5) as *mut u8) = 0;
            n += 1;
            if n == cqn12_w(this + 0x3e) as i32 {
                break;
            }
        }
        i = i.wrapping_add(1);
    }
    cqn12_ordenar(cqn12_d(this + 0xa7) + 0x42, 0x10);
    ((n == cqn12_w(this + 0x3e) as i32) as i32, n)
}

unsafe extern "thiscall" fn cqn12_copa_da_liga_inscritos(this: usize) -> i32 {
    cqn12_copa_da_liga(this).0
}

/// Lado a lado para as rotinas que alocam a lista +0xa7 e sorteiam: o +0x3e, o [0xb67a34] e o estado do sorteador a
/// partir do mesmo estado (a cópia sorteia sem contar); a original deixa a lista dela em +0xa7, a cópia faz a sua,
/// comparada nos inscritos que ela gravou e liberada.
unsafe fn cqn12_comparar(nome: &str, this: usize, o: impl FnOnce() -> i32, c: impl FnOnce() -> (i32, i32)) -> i32 {
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let nr = core::cell::Cell::new(0i32);
    let a_ = crate::aleatorio::REGIOES_DO_ESTADO;
    let reg = [(this + 0x3e, 2usize), (0xb67a34, 4), a_[0], a_[1]];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || {
        let r = o();
        lo.set(cqn12_d(this + 0xa7));
        r
    }, || {
        let (r, n) = crate::aleatorio::sem_contar(c);
        lr.set(cqn12_d(this + 0xa7));
        nr.set(n);
        core::ptr::write_unaligned((this + 0xa7) as *mut u32, lo.get() as u32);
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let k = nr.get().max(0) as usize * 6;
    let lista = if lo != 0 && lr != 0 {
        core::slice::from_raw_parts(lo as *const u8, k) == core::slice::from_raw_parts(lr as *const u8, k)
    } else {
        lo == 0 && lr == 0
    };
    if lr != 0 && lr != lo {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(lr);
    }
    PLACAR.registrar(igual && a == b && lista, || format!("{}({:#x}) devolveu {} x {} memória {} lista {} ({} bytes)", nome, this, a, b, igual, lista, k));
    a
}

unsafe extern "thiscall" fn comparar_cqn12_copa_da_liga_inscritos(this: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> i32>(1796);
    cqn12_comparar("cqn12_copa_da_liga_inscritos", this, || o(this), || cqn12_copa_da_liga(this))
}

/// { 0x90b530 } (thiscall, ret 0, usa_open_cup.cpp): `UsaOpenCup::collect_entrants()`: +0x3e = 32. Os clubes cuja
/// divisão (+0x57, o ponteiro) é a competição do banco de [0x9cf594] ou a de [0x9cf598] vão para dois vetores locais de
/// até 50 (a passada para quando os dois enchem); cada vetor é ordenado e embaralhado (0x4afbd0, 10 escolhas). A lista
/// (0x945501, o calloc de +0x3e × 6, em +0xa7; sem memória, a caixa da linha 0x138 e −1) leva os 10 primeiros de cada
/// vetor com a classe 2 e depois todos os clubes da MLS ([0x9cf590]) com a classe 1 (sem conferir o espaço). Devolve 1.
unsafe fn cqn12_copa_dos_eua(this: usize) -> (i32, i32) {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 0x20);
    let comp = |id: usize| cqn12_d(0xae23d0).wrapping_add(cqn12_d(id).wrapping_mul(0x6b));
    let (a, b) = (comp(0x9cf594), comp(0x9cf598));
    // ponytail: os vetores locais da original começam com o que estava na pilha; com menos de 10 clubes numa das
    // ligas ela copiaria essa sobra (e a 0x4afbd0 já daria a caixa): a cópia começa com zero
    let mut va = [0usize; 50];
    let mut vb = [0usize; 50];
    let (mut na, mut nb) = (0i8, 0i8);
    let mut i = 0i32;
    while i < cqn12_d(0xae235c) as i32 {
        if na == 0x32 && nb == 0x32 {
            break;
        }
        let c = cqn12_clube(i);
        let d = cqn12_d(c + 0x57);
        if d == a && na < 0x32 {
            va[na as usize] = c;
            na += 1;
        } else if d == b && nb < 0x32 {
            vb[nb as usize] = c;
            nb += 1;
        }
        i += 1;
    }
    cqn12_embaralhar(va.as_mut_ptr(), na as i32, 10);
    cqn12_embaralhar(vb.as_mut_ptr(), nb as i32, 10);
    let calloc: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(0x945501usize);
    let l = calloc(cqn12_w(this + 0x3e) as i32 as u32, 6);
    core::ptr::write_unaligned((this + 0xa7) as *mut u32, l as u32);
    if l == 0 {
        aviso_em(0xaa20e0, 0x138);
        return (-1, 0);
    }
    let mut k = 0i8;
    for &c in va.iter().take(10).chain(vb.iter().take(10)) {
        cqn12_inscrever(this, k as i32, c, 2); // cqn12: os 10 de cada liga de baixo
        k = k.wrapping_add(1);
    }
    crate::comparacao::ramo("cqn_12 inscritos da copa dos eua");
    let mut i = 0i32;
    while i < cqn12_d(0xae235c) as i32 {
        let c = cqn12_clube(i);
        if cqn12_comp(c, 0x57) == Some(cqn12_d(0x9cf590)) {
            cqn12_inscrever(this, k as i32, c, 1);
            k = k.wrapping_add(1);
        }
        i += 1;
    }
    (1, k as i32)
}

unsafe extern "thiscall" fn cqn12_copa_dos_eua_inscritos(this: usize) -> i32 {
    cqn12_copa_dos_eua(this).0
}

unsafe extern "thiscall" fn comparar_cqn12_copa_dos_eua_inscritos(this: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> i32>(1797);
    cqn12_comparar("cqn12_copa_dos_eua_inscritos", this, || o(this), || cqn12_copa_dos_eua(this))
}

/// { 0x6624f0 } (thiscall, ret 0, jap_emp_cup.cpp): `JapEmpCup::collect_entrants()`: +0x3e = 80; a lista (0x944e46 de
/// 0x1e0 em +0xa7; sem memória, a caixa da linha 0x147 e −1). Dos clubes japoneses (+0x53 = [0x9cf36c]) com divisão, em
/// vetores locais: os da [0x9cf678] (até 100), os da [0x9cf674] (até 30), os da [0x9cf670] (até 30) e os de outras
/// divisões que não a J2 ([0x9cf768]) nem a J1 ([0x9cf65c]) (até 6); sem nenhum destes, o último da [0x9cf678] sai de lá
/// para cá. Com menos de 47, 2, 3 ou 1 em cada um, a caixa da linha 0x178 e 0. Cada vetor é ordenado e embaralhado
/// (0x4afbd0 com 47, 2, 3 e 1 escolhas); a lista leva os 47, os 2 e os 3 primeiros (o último vetor não entra), classe 0;
/// depois os da J2 enquanto cabem (classe 0) e os da J1 enquanto cabem, com a classe = a posição da temporada anterior
/// (+0x5f), até encher; o qsort (0x522190) das 16 últimas inscrições. Devolve 1 com a lista cheia, senão 0.
unsafe fn cqn12_copa_do_imperador(this: usize) -> (i32, i32) {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 0x50);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let l = alocar(0x1e0);
    core::ptr::write_unaligned((this + 0xa7) as *mut u32, l as u32);
    if l == 0 {
        aviso_em(0x9eab90, 0x147);
        return (-1, 0);
    }
    let mut va = [0usize; 100];
    let mut vb = [0usize; 30];
    let mut vc = [0usize; 30];
    let mut vd = [0usize; 6];
    let (mut na, mut nb, mut nc, mut nd) = (0i8, 0i8, 0i8, 0i8);
    if cqn12_d(0xae235c) as i32 > 0 {
        let mut i: i16 = 0;
        while (i as i32) < cqn12_d(0xae235c) as i32 {
            let c = cqn12_clube(i as i32);
            if cqn12_d(c + 0x57) != 0 && cqn12_comp(c, 0x53) == Some(cqn12_d(0x9cf36c)) {
                let d = cqn12_d(cqn12_d(c + 0x57));
                if d == cqn12_d(0x9cf678) {
                    if na < 0x64 {
                        va[na as usize] = c;
                        na += 1;
                    }
                } else if d == cqn12_d(0x9cf674) {
                    if nb < 0x1e {
                        vb[nb as usize] = c;
                        nb += 1;
                    }
                } else if d == cqn12_d(0x9cf670) {
                    if nc < 0x1e {
                        vc[nc as usize] = c;
                        nc += 1;
                    }
                } else if d != cqn12_d(0x9cf768) && d != cqn12_d(0x9cf65c) && nd < 6 {
                    vd[nd as usize] = c;
                    nd += 1;
                }
            }
            i = i.wrapping_add(1);
        }
        if nd == 0 {
            na = na.wrapping_sub(1);
            nd = 1;
            vd[0] = if na >= 0 { va[na as usize] } else { 0 };
        }
    } else {
        // ponytail: sem clubes a original lê o vetor fora do lugar (índice −1); a cópia usa zero e cai na caixa abaixo
        na = na.wrapping_sub(1);
        nd = 1;
        vd[0] = 0;
    }
    if na < 0x2f || nb < 2 || nc < 3 || nd < 1 {
        aviso_em(0x9eab90, 0x178);
        return (0, 0);
    }
    cqn12_embaralhar(va.as_mut_ptr(), na as i32, 0x2f);
    cqn12_embaralhar(vb.as_mut_ptr(), nb as i32, 2);
    cqn12_embaralhar(vc.as_mut_ptr(), nc as i32, 3);
    cqn12_embaralhar(vd.as_mut_ptr(), nd as i32, 1);
    crate::comparacao::ramo("cqn_12 inscritos da copa do imperador");
    let mut n = 0i32;
    for &c in va.iter().take(0x2f).chain(vb.iter().take(2)).chain(vc.iter().take(3)) {
        cqn12_inscrever(this, n, c, 0); // cqn12: os sorteados das divisões de baixo
        n += 1;
    }
    let mut i: i16 = 0;
    while (i as i32) < cqn12_d(0xae235c) as i32 {
        let c = cqn12_clube(i as i32);
        if cqn12_comp(c, 0x57) == Some(cqn12_d(0x9cf768)) && n < cqn12_w(this + 0x3e) as i32 {
            cqn12_inscrever(this, n, c, 0);
            n += 1;
        }
        i = i.wrapping_add(1);
    }
    let mut i: i16 = 0;
    while (i as i32) < cqn12_d(0xae235c) as i32 {
        let c = cqn12_clube(i as i32);
        if cqn12_comp(c, 0x57) == Some(cqn12_d(0x9cf65c)) && n < cqn12_w(this + 0x3e) as i32 {
            crate::comparacao::ramo("cqn_12 classe da j1 na copa do imperador");
            cqn12_inscrever(this, n, c, *((c + 0x5f) as *const u8)); // cqn12: a classe dos clubes da J1 na Copa do Imperador
            n += 1;
            if n == cqn12_w(this + 0x3e) as i32 {
                break;
            }
        }
        i = i.wrapping_add(1);
    }
    cqn12_ordenar(cqn12_d(this + 0xa7).wrapping_add(((n * 3 - 0x30) * 2) as isize as usize), 0x10);
    ((n == cqn12_w(this + 0x3e) as i32) as i32, n)
}

unsafe extern "thiscall" fn cqn12_copa_do_imperador_inscritos(this: usize) -> i32 {
    cqn12_copa_do_imperador(this).0
}

unsafe extern "thiscall" fn comparar_cqn12_copa_do_imperador_inscritos(this: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> i32>(1798);
    cqn12_comparar("cqn12_copa_do_imperador_inscritos", this, || o(this), || cqn12_copa_do_imperador(this))
}

// ---------- 276a leva: os números de camisa: o número de um jogador no clube, se o clube usa números e gravar o número (squad_manager.cpp) ----------

unsafe fn cql1_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

/// O contrato `k` (não nulo) vale no clube `c`: o clube do contrato (+4, id) é o próprio `c` ([c] = id), ou é o clube
/// que a 0x540a50(c, 0, 1) devolve (cdecl de 3, conferida: push 1; push 0; push clube; add esp, 0xc) [inferência: o
/// clube principal de um time B]. O id do contrato é relido depois da chamada, como na original.
unsafe fn cql1_vale_no_clube(k: usize, c: usize) -> bool {
    let parceiro: unsafe extern "C" fn(usize, u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    if cql1_d(k + 4) == cql1_d(c) {
        return true;
    }
    let p = parceiro(c, 0, 1);
    let id = cql1_d(k + 4) as usize;
    (cql1_d(0xae23b4) as usize).wrapping_add(id.wrapping_mul(0x245)) == p
}

/// { 0x85a760 } (thiscall, ret 8, squad_manager.cpp, `SQUAD_MANAGER::get_squad_number`; o this não é lido): o número
/// de camisa (char, negativo nos clubes sem técnico humano) da pessoa `p` no clube `c`. Sem um dos dois, 0. Pessoa fora
/// da tabela das pessoas com contrato (id ≥ [0xae2364]): o byte +4 da ficha de jogador (+0x61). Senão o contrato
/// (0x4cf5a0, thiscall em [0xae1050], push pessoa), se a pessoa é jogador (0x53b620 com o +0x3d, cdecl de 1) e o
/// contrato vale no clube: o +0x3a do contrato; senão o contrato de empréstimo (0x4cf6d0, thiscall em [0xae1050], push
/// pessoa), se vale no clube: o +0x3a dele; senão 0.
unsafe extern "thiscall" fn numero_de_camisa(_this: usize, p: usize, c: usize) -> i8 {
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let emprestimo: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf6d0usize);
    let jogador: unsafe extern "C" fn(i8) -> u32 = core::mem::transmute(0x53b620usize);
    if p == 0 || c == 0 {
        return 0;
    }
    if cql1_d(p) as i32 >= cql1_d(0xae2364) as i32 {
        return *((cql1_d(p + 0x61) as usize + 4) as *const i8);
    }
    let k = contrato(cql1_d(0xae1050) as usize, p);
    if k != 0 && jogador(*((p + 0x3d) as *const i8)) != 0 && cql1_vale_no_clube(k, c) {
        crate::comparacao::ramo("cql_1 numero do contrato");
        return *((k + 0x3a) as *const i8); // cql1: o número do contrato
    }
    let k = emprestimo(cql1_d(0xae1050) as usize, p);
    if k != 0 && cql1_vale_no_clube(k, c) {
        return *((k + 0x3a) as *const i8);
    }
    0
}

/// A devolução das duas (a rotina só lê; as chamadas de dentro são buscas).
unsafe extern "thiscall" fn comparar_numero_de_camisa(this: usize, p: usize, c: usize) -> i8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> i8>(1799);
    let r = o(this, p, c);
    let n = numero_de_camisa(this, p, c);
    PLACAR.registrar(r == n, || format!("numero_de_camisa(pessoa {:x}, clube {:x}): original {} rust {}", p, c, r, n));
    r
}

/// { 0x85add0 } (thiscall, ret 8, squad_manager.cpp, `SQUAD_MANAGER::club_uses_squad_numbers`; o this não é lido):
/// 1 se o clube usa números de camisa: a competição da divisão do clube (o id em [[c+0x57]], o objeto na tabela
/// [0xadadfc]) existe e tem o bit 2 do +0x52 e, com `outra` (Comp*), a outra competição (a da tabela pelo id [outra])
/// também tem o bit. Sem divisão ou sem a competição, 0; sem clube, a caixa da linha 0x4b0 e 0.
unsafe extern "thiscall" fn clube_usa_numeros(_this: usize, c: usize, outra: usize) -> u32 {
    if c == 0 {
        aviso_em(0xa82b88, 0x4b0);
        return 0;
    }
    let div = cql1_d(c + 0x57) as usize;
    if div == 0 {
        return 0;
    }
    let tabela = cql1_d(0xadadfc) as usize;
    let comp = cql1_d(tabela + cql1_d(div) as usize * 4) as usize;
    if comp == 0 {
        return 0;
    }
    if *((comp + 0x52) as *const u8) & 2 == 0 {
        return 0;
    }
    if outra != 0 {
        let comp2 = cql1_d(tabela + cql1_d(outra) as usize * 4) as usize;
        if *((comp2 + 0x52) as *const u8) & 2 == 0 {
            return 0;
        }
    }
    crate::comparacao::ramo("cql_1 usa numeros");
    1 // cql1: usa números
}

/// A devolução das duas (a rotina só lê); sem clube, só a original (a caixa não roda duas vezes).
unsafe extern "thiscall" fn comparar_clube_usa_numeros(this: usize, c: usize, outra: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(1800);
    if c == 0 {
        return o(this, c, outra);
    }
    let r = o(this, c, outra);
    let n = clube_usa_numeros(this, c, outra);
    PLACAR.registrar(r == n, || format!("clube_usa_numeros(clube {:x}, outra {:x}): original {} rust {}", c, outra, r, n));
    r
}

/// { 0x85a850 } (thiscall, ret 0xc, squad_manager.cpp, `SQUAD_MANAGER::set_squad_number`; o this não é lido): grava o
/// número `n` (char de −50 a 50; fora disso a caixa da linha 0x332) no +0x3a do contrato da pessoa `p` (0x4cf5a0) se ele
/// vale no clube `c`, senão no do contrato de empréstimo (0x4cf6d0) se ele vale no clube. Sem pessoa ou sem clube, nada.
unsafe extern "thiscall" fn gravar_numero_de_camisa(_this: usize, p: usize, c: usize, n: u32) {
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let emprestimo: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf6d0usize);
    if p == 0 || c == 0 {
        return;
    }
    let n = n as u8 as i8;
    if !(-50..=50).contains(&n) {
        aviso_em(0xa82b88, 0x332);
        return;
    }
    let k = contrato(cql1_d(0xae1050) as usize, p);
    if k != 0 && cql1_vale_no_clube(k, c) {
        crate::comparacao::ramo("cql_1 grava no contrato");
        *((k + 0x3a) as *mut i8) = n; // cql1: grava no contrato
        return;
    }
    let k = emprestimo(cql1_d(0xae1050) as usize, p);
    if k != 0 && cql1_vale_no_clube(k, c) {
        *((k + 0x3a) as *mut i8) = n;
    }
}

/// Lado a lado: o +0x3a dos dois contratos da pessoa (achados antes; as buscas só leem). Número fora da faixa (a caixa),
/// sem pessoa ou sem clube: só a original.
unsafe extern "thiscall" fn comparar_gravar_numero_de_camisa(this: usize, p: usize, c: usize, n: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(1801);
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let emprestimo: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf6d0usize);
    if p == 0 || c == 0 || !(-50..=50).contains(&(n as u8 as i8)) {
        return o(this, p, c, n);
    }
    let mut reg = Vec::new();
    for k in [contrato(cql1_d(0xae1050) as usize, p), emprestimo(cql1_d(0xae1050) as usize, p)] {
        if k != 0 {
            reg.push((k + 0x3a, 1));
        }
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, p, c, n), || gravar_numero_de_camisa(this, p, c, n));
    PLACAR.registrar(igual, || format!("gravar_numero_de_camisa(pessoa {:x}, clube {:x}, {}): original {:?} rust {:?}", p, c, n as u8 as i8, mo.0, mr.0));
}

// ---------- 276a leva: os números de camisa: dar número a quem chega e numerar o elenco de um clube (squad_manager.cpp) ----------

unsafe fn cql2_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

/// A pessoa `p` está na tabela das pessoas com contrato (id < [0xae2364], com sinal).
unsafe fn cql2_com_contrato(p: usize) -> bool {
    (cql2_d(p) as i32) < cql2_d(0xae2364) as i32
}

/// Os números preferidos (índice = número − 1) pela posição da ficha de jogador `f` (as notas de posição de 1 a 20,
/// "boa" = 15 ou mais): goleiro (+0xf) o 1; zagueiro ou líbero (+0x11, +0x10) o 2 se joga pela direita (+0x17), o 3 se
/// pela esquerda (+0x18), senão o 4, o 5 ou o 6; volante, meia ou meia-atacante (+0x12, +0x13, +0x14) o 6, o 7 ou o 8;
/// atacante (+0x15) o 8, o 9, o 10 ou o 11. Termina em 0xff. Nenhuma das posições boas: `None` (a original lê então a
/// lista de 4 bytes sem valor na pilha, ver `comparar_dar_numero_de_camisa`).
unsafe fn cql2_preferidos(f: usize) -> Option<[u8; 5]> {
    let b = |o: usize| *((f + o) as *const i8) >= 15;
    if b(0xf) {
        return Some([0, 0xff, 0xff, 0xff, 0xff]);
    }
    if b(0x11) || b(0x10) {
        if b(0x17) {
            return Some([1, 0xff, 0xff, 0xff, 0xff]);
        }
        if b(0x18) {
            return Some([2, 0xff, 0xff, 0xff, 0xff]);
        }
        return Some([3, 4, 5, 0xff, 0xff]);
    }
    if b(0x13) || b(0x12) || b(0x14) {
        return Some([5, 6, 7, 0xff, 0xff]);
    }
    if b(0x15) {
        return Some([7, 8, 9, 10, 0xff]);
    }
    None
}

/// { 0x85a470 } (thiscall, ret 8, squad_manager.cpp, `SQUAD_MANAGER::allocate_squad_number`): dá um número de camisa
/// à pessoa `p` que chega ao clube `c`. Marca os números ocupados pelo elenco (0x541f90(c, lista, 0), cdecl de 3; a
/// lista ordenada pela idade com o qsort 0x945333 e a 0x540130; quem está fora da tabela das pessoas com contrato ou sem
/// ficha de jogador não conta) com o número de cada um (0x85a760, em módulo). Se `p` está na tabela e ainda não tem
/// número: o primeiro número livre entre os preferidos da posição (`cql2_preferidos`); senão o primeiro livre de 12 a
/// 50; e, se ainda sem número, o maior livre de 11 para baixo. O número vai positivo para o clube de técnico humano
/// (0x600a50(c, 0, 1, 0, 0), cdecl de 5) e negativo para os outros (0x85a850). Sem a pessoa, nada.
unsafe extern "thiscall" fn dar_numero_de_camisa(this: usize, c: usize, p: usize) {
    if p == 0 {
        return;
    }
    let elenco: unsafe extern "C" fn(usize, *mut usize, u32) -> u32 = core::mem::transmute(0x541f90usize);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let numero: unsafe extern "thiscall" fn(usize, usize, usize) -> i8 = core::mem::transmute(0x85a760usize);
    let gravar: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x85a850usize);
    let mut usado = [0u8; 50];
    let mut lista = [0usize; 50];
    elenco(c, lista.as_mut_ptr(), 0);
    ordenar(lista.as_mut_ptr() as usize, 0x32, 4, 0x540130);
    for q in lista.iter_mut() {
        let x = *q;
        if x == 0 {
            continue;
        }
        if cql2_d(x + 0x61) == 0 {
            *q = 0;
            continue;
        }
        if cql2_com_contrato(x) {
            let n = (numero(this, x, c) as i32).abs() as u8;
            if n != 0 {
                usado[(n as i8 as isize - 1) as usize] = 1;
            }
        }
    }
    if !cql2_com_contrato(p) || numero(this, p, c) != 0 {
        return;
    }
    let pref = cql2_preferidos(cql2_d(p + 0x61) as usize).unwrap_or([0xff; 5]);
    // o número com o sinal do clube: positivo com técnico humano, negativo sem
    let sinal = |k: u8| -> u32 {
        if humano(c, 0, 1, 0, 0) != 0 {
            k.wrapping_add(1) as u32
        } else {
            0xffu8.wrapping_sub(k) as u32
        }
    };
    let mut b = 0usize;
    while pref[b] != 0xff {
        let k = pref[b];
        if usado[k as i8 as usize] == 0 {
            gravar(this, p, c, sinal(k));
            usado[k as usize] = 1;
            crate::comparacao::ramo("cql_2 numero da posicao");
            return; // cql2: o número da posição
        }
        b += 1;
        if b >= 5 {
            break;
        }
    }
    if let Some(k) = (11u8..50).find(|&k| usado[k as usize] == 0) {
        gravar(this, p, c, sinal(k));
        usado[k as usize] = 1;
    }
    if numero(this, p, c) != 0 {
        return;
    }
    if let Some(k) = (0u8..=10).rev().find(|&k| usado[k as usize] == 0) {
        gravar(this, p, c, sinal(k));
    }
}

/// Lado a lado: o +0x3a dos dois contratos da pessoa (a única que a rotina numera) e a marca da caixa. Quando nenhuma
/// posição da ficha é boa, a original lê a lista de preferidos de 4 bytes sem valor na pilha (sobra de quem chamou):
/// a cópia usa a lista vazia (um valor que o jogo produz quando a sobra é 0xff) e o comparador deixa só a original.
unsafe extern "thiscall" fn comparar_dar_numero_de_camisa(this: usize, c: usize, p: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1802);
    if p == 0 || cql2_d(p + 0x61) == 0 || cql2_preferidos(cql2_d(p + 0x61) as usize).is_none() {
        return o(this, c, p);
    }
    let reg = cql2_regioes(&[p]);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, c, p), || dar_numero_de_camisa(this, c, p));
    PLACAR.registrar(igual, || format!("dar_numero_de_camisa(clube {:x}, pessoa {:x}): original {:?} rust {:?}", c, p, mo.0, mr.0));
}

/// O +0x3a dos contratos (o do clube e o de empréstimo) das pessoas `ps` e a marca da caixa [0xb67a34].
unsafe fn cql2_regioes(ps: &[usize]) -> Vec<(usize, usize)> {
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let emprestimo: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf6d0usize);
    let mut reg = vec![(0xb67a34usize, 4usize)];
    for &p in ps {
        if p == 0 {
            continue;
        }
        for k in [contrato(cql2_d(0xae1050) as usize, p), emprestimo(cql2_d(0xae1050) as usize, p)] {
            if k != 0 && !reg.contains(&(k + 0x3a, 1)) {
                reg.push((k + 0x3a, 1));
            }
        }
    }
    reg
}

/// As vagas da escalação de referência da 0x859fe0 para a 0x6e2000: os 11 titulares e os 9 reservas, cada um com as
/// posições que servem (máscaras: 1 goleiro, 4/0x10/0x40 os setores, 0x80..0x800 as faixas [inferência]).
const CQL2_VAGAS: [u16; 20] = [
    0x1, 0x804, 0x84, 0x210, 0x204, 0x204, 0x810, 0x210, 0x240, 0x240, 0x90, // cql2: os titulares
    0x10, 0x1, 0x4, 0x40, 0x40, 0x10, 0x4, 0x40, 0x4, // os reservas
];

/// { 0x859fe0 } (thiscall, ret 8, squad_manager.cpp, `SQUAD_MANAGER::assign_club_squad_numbers`): numera o elenco do
/// clube `c`. Junta os jogadores do elenco (+0xd7, 50 vagas; só os da tabela das pessoas com contrato) e os do clube
/// ligado (0x540a50(c, 0, 1), cdecl de 3; até 50 no total); escala os 11 titulares e os 9 reservas de referência
/// (0x6e2000, cdecl de 15, com `CQL2_VAGAS` e os modos 0x5840) e põe os outros depois, nas vagas livres; ordena os 30
/// últimos pela idade (qsort 0x945333 com a 0x540130) e junta a lista. Depois, na ordem: do 22º em diante, quem não tem
/// contrato com o clube (0x4cf5f0) ou tem contrato do tipo 6 ou 7 (+0x4f, nibble alto) [inferência: juvenil e amador]
/// fica sem número; quem tem número (0x85a760) o guarda se ele está livre e, sem `manter`, se é de titular (até 11)
/// entre os 11 primeiros ou de reserva (12 em diante) depois deles (senão fica sem número); com `manter`, guarda o número
/// com o sinal trocado; quem não tem número ganha o da sua ordem (1º o 1...) se livre. Os 40 primeiros sem número que
/// têm contrato com o clube (fora os tipos 6 e 7) ganham um pela 0x85a470. No fim, cada número guardado é regravado:
/// positivo (o número) com técnico humano (0x600a50(c, 0, 1, 0, 0)), senão −1, −2... na ordem dos números.
unsafe extern "thiscall" fn numerar_elenco(this: usize, c: usize, manter: u32) {
    let parceiro: unsafe extern "C" fn(usize, u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    let escalar: Escalar = core::mem::transmute(0x6e2000usize);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let do_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let numero: unsafe extern "thiscall" fn(usize, usize, usize) -> i8 = core::mem::transmute(0x85a760usize);
    let gravar: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x85a850usize);
    let dar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x85a470usize);
    let mut guardado = [0usize; 50]; // por número − 1
    let mut ordem = [0usize; 50];
    let mut todos = [0usize; 50];
    let mut n = 0usize;
    for k in 0..50 {
        let x = cql2_d(c + 0xd7 + 4 * k) as usize;
        if x != 0 && cql2_com_contrato(x) {
            todos[n] = x;
            n += 1;
        }
    }
    let l = parceiro(c, 0, 1);
    if l != 0 {
        for k in 0..50 {
            if n >= 50 {
                break;
            }
            let x = cql2_d(l + 0xd7 + 4 * k) as usize;
            if x != 0 && cql2_com_contrato(x) {
                todos[n] = x;
                n += 1;
            }
        }
    }
    let vagas = CQL2_VAGAS;
    crate::comparacao::ramo("cql_2 escala");
    escalar(todos.as_ptr() as usize, n as u32, vagas.as_ptr() as usize, 0xb, 9, ordem.as_mut_ptr() as usize, 0x5840, c, 0, 0, 0, 0, 0, 0, 0);
    let mut livre = 0usize;
    for &x in todos[..n].iter() {
        if ordem[..20].contains(&x) {
            continue;
        }
        while livre < 50 && ordem[livre] != 0 {
            livre += 1;
        }
        if livre < 50 {
            ordem[livre] = x;
        }
    }
    ordenar(ordem[20..].as_mut_ptr() as usize, 0x1e, 4, 0x540130);
    let mut a = 0usize;
    for b in 0..50 {
        let x = ordem[b];
        if x == 0 || b <= a {
            continue;
        }
        while a < b && ordem[a] != 0 {
            a += 1;
        }
        if a < b {
            ordem[a] = x;
            a += 1;
            ordem[b] = 0;
        }
    }
    // `vez` = −1, −2... (char), a ordem de cada um; `k` = 1, 2... a mesma ordem em positivo
    for i in 0..50usize {
        let vez = (-1 - i as i32) as i8;
        let x = ordem[i];
        if x == 0 {
            continue;
        }
        if vez < -21 {
            let ct = do_clube(cql2_d(0xae1050) as usize, x, c);
            let t = if ct != 0 { *((ct + 0x4f) as *const u8) & 0xf0 } else { 0 };
            if ct == 0 || t == 0x60 || t == 0x70 {
                gravar(this, x, c, 0);
                continue;
            }
        }
        let nb = (numero(this, x, c) as i32).abs() as u8 as i8;
        if nb == 0 {
            if guardado[i] == 0 {
                guardado[i] = x;
                gravar(this, x, c, vez as u8 as u32);
            }
            continue;
        }
        let vaga = (nb as isize - 1) as usize;
        if guardado[vaga] != 0 {
            gravar(this, x, c, 0);
            continue;
        }
        if manter != 0 {
            gravar(this, x, c, nb.wrapping_neg() as u8 as u32);
        } else if (nb < 12 && vez < -11) || (nb >= 12 && vez > -12) {
            crate::comparacao::ramo("cql_2 perde o numero");
            gravar(this, x, c, 0); // cql2: o número fora do lugar
            continue;
        }
        if nb > 0 {
            guardado[vaga] = x;
        }
    }
    for b in 0..40usize {
        let x = ordem[b];
        if x == 0 {
            continue;
        }
        let ct = do_clube(cql2_d(0xae1050) as usize, x, c);
        if ct == 0 {
            continue;
        }
        let t = *((ct + 0x4f) as *const u8) & 0xf0;
        if t == 0x60 || t == 0x70 {
            continue;
        }
        if numero(this, x, c) == 0 {
            dar(this, c, x);
        }
    }
    if humano(c, 0, 1, 0, 0) != 0 {
        for (k, &x) in guardado.iter().enumerate() {
            if x != 0 {
                gravar(this, x, c, k as u32 + 1);
            }
        }
    } else {
        let mut v = -1i8;
        for &x in guardado.iter() {
            if x != 0 {
                crate::comparacao::ramo("cql_2 regrava negativo");
                gravar(this, x, c, v as u8 as u32); // cql2: regrava negativo
            }
            v = v.wrapping_sub(1);
        }
    }
}

/// Lado a lado: o +0x3a dos contratos de quem está no elenco do clube e no do clube ligado (as únicas pessoas que a
/// rotina e a 0x85a470 numeram) e a marca da caixa.
unsafe extern "thiscall" fn comparar_numerar_elenco(this: usize, c: usize, manter: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1803);
    let parceiro: unsafe extern "C" fn(usize, u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    let mut ps: Vec<usize> = (0..50).map(|k| cql2_d(c + 0xd7 + 4 * k) as usize).collect();
    let l = parceiro(c, 0, 1);
    if l != 0 {
        ps.extend((0..50).map(|k| cql2_d(l + 0xd7 + 4 * k) as usize));
    }
    let reg = cql2_regioes(&ps);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, c, manter), || numerar_elenco(this, c, manter));
    PLACAR.registrar(igual, || {
        let d: Vec<String> = mo.0.iter().zip(mr.0.iter()).filter(|(a, b)| a.1 != b.1).take(12).map(|(a, b)| format!("{:x}: {:?}/{:?}", a.0, a.1, b.1)).collect();
        format!("numerar_elenco(clube {:x}, manter {}): {}", c, manter, d.join(", "))
    });
}

// ---------- 276a leva: os números de camisa: numerar as ligas no começo do jogo e a inscrição nas copas europeias (squad_manager.cpp) ----------

unsafe fn cql3_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

/// As ligas que a 0x859dc0 da original mandou numerar (gravadas pelo comparador da 0x859e20 enquanto o da 0x859dc0
/// roda a original), para conferir o plano da cópia.
static mut CQL3_PLANO: Option<Vec<usize>> = None;

/// As ligas que a 0x859dc0 numera: as competições da tabela [0xadadfc] (conta [0xadadf8], relida a cada volta) que
/// são liga de divisão (0x4b0700 com o id [[+4]], cdecl de 1) e usam números (bit 2 do +0x52).
unsafe fn cql3_ligas_a_numerar(mut cada: impl FnMut(usize)) {
    let divisao: unsafe extern "C" fn(i32) -> u32 = core::mem::transmute(0x4b0700usize);
    let mut i = 0i32;
    while i < cql3_d(0xadadf8) as i32 {
        let c = cql3_d(cql3_d(0xadadfc) as usize + 4 * i as usize) as usize;
        if c != 0 && divisao(cql3_d(cql3_d(c + 4) as usize) as i32) != 0 && *((c + 0x52) as *const u8) & 2 != 0 {
            crate::comparacao::ramo("cql_3 numera a liga");
            cada(c); // cql3: numera a liga
        }
        i += 1;
    }
}

/// { 0x859dc0 } (thiscall, ret 0, squad_manager.cpp, `SQUAD_MANAGER::init_new_game`): zera o ano das inscrições (+0,
/// word) e numera (0x859e20(this, liga, 1, 0)) cada liga de divisão que usa números (`cql3_ligas_a_numerar`).
unsafe extern "thiscall" fn numerar_no_comeco(this: usize) {
    let numerar: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(0x859e20usize);
    *(this as *mut u16) = 0;
    cql3_ligas_a_numerar(|c| numerar(this, c, 1, 0));
}

/// Só a original (ela numera as ligas pela 0x859e20, que manda notícia ao técnico humano: não roda duas vezes); o plano
/// da cópia (as ligas, achadas antes, sem numerar) tem de ser o que a original fez (gravado pelo comparador da
/// 0x859e20) e o +0 zerado.
unsafe extern "thiscall" fn comparar_numerar_no_comeco(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1804);
    let mut plano = Vec::new();
    cql3_ligas_a_numerar(|c| plano.push(c));
    CQL3_PLANO = Some(Vec::new());
    o(this);
    let feito = CQL3_PLANO.take().unwrap_or_default();
    let zerado = *(this as *const u16) == 0;
    PLACAR.registrar(feito == plano && zerado, || format!("numerar_no_comeco: plano {:x?} original {:x?} ano zerado {}", plano, feito, zerado));
}

/// { 0x859e20 } (thiscall, ret 0xc, squad_manager.cpp, `SQUAD_MANAGER::assign_league_squad_numbers`): numera os clubes
/// da liga `liga` (a lista do método +0x58 da competição, thiscall(liga, &lista, 0), devolve a conta em ax; a lista é
/// solta no fim pela 0x9452ca). Em 1998 vale sempre `inicial`. Pula o time B (0x540a50(clube, &principal, 1) com o
/// clube ligado e principal = 0). Clube de técnico humano (0x7875e0, com ecx = [0xb7431c]) sem `automatico`: os números
/// negativos do elenco (0x541f90(clube, lista, 0)) viram positivos e o técnico recebe o pedido da lista de números
/// (0x85aec0, original). Senão: com `inicial`, cada jogador do elenco (da tabela das pessoas com contrato e com ficha)
/// recebe o número do banco (o +4 da ficha, entre 0 e 50); e o elenco é numerado (0x859fe0(clube, 1)).
unsafe extern "thiscall" fn numerar_liga(this: usize, liga: usize, mut inicial: u32, automatico: u32) {
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    let elenco: unsafe extern "C" fn(usize, *mut usize, u32) -> u32 = core::mem::transmute(0x541f90usize);
    let humano: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7875e0usize);
    let pedido: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x85aec0usize);
    let numero: unsafe extern "thiscall" fn(usize, usize, usize) -> i8 = core::mem::transmute(0x85a760usize);
    let gravar: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x85a850usize);
    let numerar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x859fe0usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    if *(0xae2c92 as *const u16) == 0x7ce {
        inicial = 1;
    }
    let clubes: unsafe extern "thiscall" fn(usize, *mut usize, u32) -> u16 = core::mem::transmute(cql3_d(cql3_d(liga) as usize + 0x58) as usize);
    let mut lista = 0usize;
    let n = clubes(liga, &mut lista, 0) as i16 as i32;
    for i in 0..n.max(0) as usize {
        let c = cql3_d(lista + 4 * i) as usize;
        let mut principal = 0u32;
        if parceiro(c, &mut principal, 1) != 0 && principal == 0 {
            continue;
        }
        let mut sq = [0usize; 50];
        elenco(c, sq.as_mut_ptr(), 0);
        if humano(cql3_d(0xb7431c) as usize, c) != 0 && automatico == 0 {
            for &p in sq.iter() {
                if p != 0 && numero(this, p, c) < 0 {
                    gravar(this, p, c, (numero(this, p, c) as u8).wrapping_neg() as u32);
                }
            }
            pedido(this, c);
            continue;
        }
        if inicial != 0 {
            for &p in sq.iter() {
                if p == 0 || cql3_d(p) as i32 >= cql3_d(0xae2364) as i32 {
                    continue;
                }
                let f = cql3_d(p + 0x61) as usize;
                if f == 0 {
                    continue;
                }
                let b = *((f + 4) as *const i8);
                let k = if b < 0 { 0 } else if b > 0x32 { 0x32 } else { b as i32 };
                crate::comparacao::ramo("cql_3 numero do banco");
                gravar(this, p, c, k as u32); // cql3: o número do banco
            }
        }
        numerar(this, c, 1);
    }
    if lista != 0 {
        soltar(lista);
    }
}

/// Lado a lado: o +0x3a dos contratos (o do clube e o de empréstimo) de quem está no elenco de cada clube da liga e
/// no do clube ligado, e a marca da caixa. Com algum clube de técnico humano sem `automatico` (a notícia do pedido não
/// roda duas vezes), só a original. A lista dos clubes vem do mesmo método +0x58 (solta depois).
unsafe extern "thiscall" fn comparar_numerar_liga(this: usize, liga: usize, inicial: u32, automatico: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32)>(1805);
    if let Some(v) = (*core::ptr::addr_of_mut!(CQL3_PLANO)).as_mut() {
        v.push(liga);
    }
    let parceiro: unsafe extern "C" fn(usize, u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    let humano: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7875e0usize);
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let emprestimo: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf6d0usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let clubes: unsafe extern "thiscall" fn(usize, *mut usize, u32) -> u16 = core::mem::transmute(cql3_d(cql3_d(liga) as usize + 0x58) as usize);
    let mut lista = 0usize;
    let n = clubes(liga, &mut lista, 0) as i16 as i32;
    let cs: Vec<usize> = (0..n.max(0) as usize).map(|i| cql3_d(lista + 4 * i) as usize).collect();
    if lista != 0 {
        soltar(lista);
    }
    if automatico == 0 && cs.iter().any(|&c| humano(cql3_d(0xb7431c) as usize, c) != 0) {
        return o(this, liga, inicial, automatico);
    }
    let mut reg = vec![(0xb67a34usize, 4usize)];
    for &c in &cs {
        let l = parceiro(c, 0, 1);
        for q in [c, l] {
            if q == 0 {
                continue;
            }
            for k in 0..50 {
                let p = cql3_d(q + 0xd7 + 4 * k) as usize;
                if p == 0 {
                    continue;
                }
                for ct in [contrato(cql3_d(0xae1050) as usize, p), emprestimo(cql3_d(0xae1050) as usize, p)] {
                    if ct != 0 && !reg.contains(&(ct + 0x3a, 1)) {
                        reg.push((ct + 0x3a, 1));
                    }
                }
            }
        }
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, liga, inicial, automatico), || numerar_liga(this, liga, inicial, automatico));
    PLACAR.registrar(igual, || {
        let d: Vec<String> = mo.0.iter().zip(mr.0.iter()).filter(|(a, b)| a.1 != b.1).take(12).map(|(a, b)| format!("{:x}: {:?}/{:?}", a.0, a.1, b.1)).collect();
        format!("numerar_liga(liga {:x}, {} {}): {}", liga, inicial, automatico, d.join(", "))
    });
}

/// As vagas da escalação de referência da 0x85aae0: os 11 titulares e até 14 reservas (as da 0x859fe0, `CQL2_VAGAS`,
/// até a 19ª; a 20ª aqui é de goleiro).
const CQL3_VAGAS: [u16; 25] = [
    0x1, 0x804, 0x84, 0x210, 0x204, 0x204, 0x810, 0x210, 0x240, 0x240, 0x90, // os titulares
    0x10, 0x1, 0x4, 0x40, 0x40, 0x10, 0x4, 0x40, 0x1, 0x4, 0x40, 0x40, 0x10, 0x4, // os reservas
];

/// { 0x85aae0 } (thiscall, ret 0xc, squad_manager.cpp, `SQUAD_MANAGER::select_euro_registered_players`; o this não é
/// lido): a inscrição do clube `c` nas copas europeias na etapa `etapa` (1 a primeira; `marca` [inferência: a lista A
/// ou B]), pela marca de inscrição da pessoa (+0x6d: 1 ou 2 inscrito, 4 e 8 outras marcas). Do elenco (0x541f90(c,
/// lista, 0)), os candidatos são os da tabela das pessoas com contrato, sem contrato do tipo 5 (+0x35 & 0x3f; o
/// contrato achado fica em [0xae1080]), sem contrato de empréstimo e sem as marcas 8 e 0x10 (+0x6d & 0x18); fora da
/// etapa 1, a marca 1 cai para 0 e a 4 vira 2, e os 2 são contados. A escalação de referência (0x6e2000 com
/// `CQL3_VAGAS`, os 11 titulares e 14 reservas, ou 9 na etapa 1 sem `marca`) escolhe os melhores. Na etapa 3, até 2
/// inscritos (2) que ficaram fora da escalação perdem a inscrição (vão a 4 com `marca`, senão 0) e a conta baixa (no
/// mínimo 21). Depois cada escalado: na etapa 1, é inscrito (1 com `marca`, senão 2) e contado; nas outras, quem ainda
/// não é 2 nem tem a marca 8 é inscrito do mesmo jeito enquanto a conta for menor que as vagas da escalação. Devolve a
/// conta dos inscritos (char).
unsafe extern "thiscall" fn inscrever_na_europa(_this: usize, c: usize, etapa: u32, marca: u32) -> u8 {
    let elenco: unsafe extern "C" fn(usize, *mut usize, u32) -> u32 = core::mem::transmute(0x541f90usize);
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let emprestimo: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf6d0usize);
    let escalar: Escalar = core::mem::transmute(0x6e2000usize);
    let etapa = etapa as u8;
    let mut sq = [0usize; 50];
    elenco(c, sq.as_mut_ptr(), 0);
    let reservas: u8 = if etapa == 1 { if marca != 0 { 0xe } else { 9 } } else { 0xe };
    let mut cand = [0usize; 50];
    let mut n = 0u8;
    let mut conta = 0i8;
    let mut tirados = 0i8;
    let m6d = |p: usize| (p + 0x6d) as *mut u8;
    for &p in sq.iter() {
        if p == 0 || cql3_d(p) as i32 >= cql3_d(0xae2364) as i32 {
            continue;
        }
        let k = contrato(cql3_d(0xae1050) as usize, p);
        *(0xae1080 as *mut u32) = k as u32;
        if k != 0 && *((k + 0x35) as *const u8) & 0x3f == 5 {
            continue;
        }
        if emprestimo(cql3_d(0xae1050) as usize, p) != 0 || *m6d(p) & 0x18 != 0 {
            continue;
        }
        cand[n as i8 as usize] = p;
        n = n.wrapping_add(1);
        let a = *m6d(p);
        if a == 0 || etapa == 1 {
            continue;
        }
        if a == 1 {
            *m6d(p) = 0;
        } else if a == 4 {
            *m6d(p) = 2;
        }
        if *m6d(p) == 2 {
            conta = conta.wrapping_add(1);
        }
    }
    let vagas = CQL3_VAGAS;
    let mut esc = [0usize; 50];
    escalar(cand.as_ptr() as usize, n as u32, vagas.as_ptr() as usize, 0xb, reservas as u32, esc.as_mut_ptr() as usize, 0x5840, c, 0, 0, 0, 0, 0, 0, 0);
    if etapa == 3 {
        let mut d = 0i8;
        while d < n as i8 && tirados < 2 {
            let p = cand[d as usize];
            if p != 0 && *m6d(p) == 2 && !esc[..0x19].contains(&p) {
                *m6d(p) = if marca != 0 { 4 } else { 0 };
                tirados += 1;
                conta = conta.wrapping_sub(1);
                if conta <= 0x15 {
                    conta = 0x15;
                }
            }
            d += 1;
        }
    }
    let total = reservas as i8 as i32 + 0xb;
    let mut b = 0i32;
    while b < total {
        let p = esc[b as usize];
        if p != 0 {
            let v = if marca != 0 { 1 } else { 2 };
            if etapa == 1 {
                crate::comparacao::ramo("cql_3 inscreve na etapa 1");
                *m6d(p) = v; // cql3: inscreve
                conta = conta.wrapping_add(1);
            } else {
                let a = *m6d(p);
                if a != 2 && a & 8 == 0 && (conta as i32) < total {
                    crate::comparacao::ramo("cql_3 inscreve depois");
                    *m6d(p) = v; // cql3: inscreve depois
                    conta = conta.wrapping_add(1);
                }
            }
        }
        b = (b as i8).wrapping_add(1) as i32;
    }
    conta as u8
}

/// Lado a lado: a marca de inscrição (+0x6d) de quem está no elenco (a mesma 0x541f90), o contrato achado [0xae1080]
/// e a devolução.
unsafe extern "thiscall" fn comparar_inscrever_na_europa(this: usize, c: usize, etapa: u32, marca: u32) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u8>(1806);
    let elenco: unsafe extern "C" fn(usize, *mut usize, u32) -> u32 = core::mem::transmute(0x541f90usize);
    let mut sq = [0usize; 50];
    elenco(c, sq.as_mut_ptr(), 0);
    let mut reg = vec![(0xae1080usize, 4usize)];
    for &p in sq.iter() {
        if p != 0 && !reg.contains(&(p + 0x6d, 1)) {
            reg.push((p + 0x6d, 1));
        }
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, c, etapa, marca), || inscrever_na_europa(this, c, etapa, marca));
    PLACAR.registrar(igual && r == n, || {
        let d: Vec<String> = mo.0.iter().zip(mr.0.iter()).filter(|(a, b)| a.1 != b.1).take(12).map(|(a, b)| format!("{:x}: {:?}/{:?}", a.0, a.1, b.1)).collect();
        format!("inscrever_na_europa(clube {:x}, etapa {}, marca {}): original {} rust {} {}", c, etapa as u8, marca, r, n, d.join(", "))
    });
    r
}

// ---------- 276a leva: as regras de transferência da Holanda, da Croácia e da Irlanda: as janelas de transferência (croatia_rules.cpp, holland_rules.cpp, ireland_rules.cpp) ----------

unsafe fn cqm1_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

/// As janelas de transferência: +8 = o número de janelas; a tabela (`t`, datas de 6 bytes: +0 a regra, +1 o número da
/// janela, +2 o dia da semana para o qual a data anda (0 domingo ... 6 sábado, 0xff nenhum) [inferência], +3 o dia, +4 o
/// mês (0 = janeiro), +5 1 no início da janela e 0 no fim; duas datas por janela) alocada em +4 (0x944e46; sem memória,
/// a caixa da linha `linha` do fonte `arquivo`).
unsafe fn cqm1_janelas(this: usize, n: u8, t: &[[u8; 6]], arquivo: usize, linha: u32) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    *((this + 8) as *mut u8) = n;
    let j = alocar((t.len() * 6) as u32);
    core::ptr::write_unaligned((this + 4) as *mut u32, j as u32);
    if j == 0 {
        aviso_em(arquivo, linha);
        return;
    }
    for (k, r) in t.iter().enumerate() {
        core::ptr::copy_nonoverlapping(r.as_ptr(), (j + k * 6) as *mut u8, 6);
    }
}

/// O construtor das regras de uma nação: a base (0x8e84a0, original, thiscall com os 5 argumentos de quem chama), a
/// vtable da nação e as janelas.
unsafe fn cqm1_construir(this: usize, a: [u32; 5], vtable: u32, n: u8, t: &[[u8; 6]], arquivo: usize, linha: u32) {
    let base: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(0x8e84a0usize);
    base(this, a[0], a[1], a[2], a[3], a[4]);
    core::ptr::write_unaligned(this as *mut u32, vtable);
    cqm1_janelas(this, n, t, arquivo, linha);
}

/// Lado a lado à mão: a original no objeto de quem chama; a cópia num objeto de rascunho com o mesmo conteúdo de antes
/// (os 0x19 bytes das regras); iguais os campos (fora o ponteiro +4) e as janelas alocadas (a do rascunho é solta).
unsafe fn cqm1_comparar(nome: &str, this: usize, o: impl FnOnce(), c: impl FnOnce(usize)) {
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let mut s = [0u8; 0x20];
    core::ptr::copy_nonoverlapping(this as *const u8, s.as_mut_ptr(), 0x19);
    o();
    let r = s.as_mut_ptr() as usize;
    c(r);
    let a = core::slice::from_raw_parts(this as *const u8, 0x19);
    let campos = a[..4] == s[..4] && a[8..] == s[8..0x19];
    let (ja, jb) = (cqm1_d(this + 4) as usize, cqm1_d(r + 4) as usize);
    let n = s[8] as usize * 12;
    let janelas = if ja != 0 && jb != 0 {
        core::slice::from_raw_parts(ja as *const u8, n) == core::slice::from_raw_parts(jb as *const u8, n)
    } else {
        ja == jb
    };
    if jb != 0 {
        soltar(jb);
    }
    PLACAR.registrar(campos && janelas, || format!("{}: campos {} janelas {}", nome, campos, janelas));
}

/// As janelas da Holanda: uma janela, de 8 de junho (andando para o sábado [inferência]) a 4 de abril (andando para a sexta-feira [inferência]).
const CQM1_HOLANDA: [[u8; 6]; 2] = [
    [0x0c, 0x00, 0x06, 0x08, 0x05, 0x01], // cqm1: a Holanda
    [0x0c, 0x00, 0x05, 0x04, 0x03, 0x00],
];

/// { 0x5f9690 } (thiscall, ret 0x14, holland_rules.cpp, `HOLLAND_RULES::HOLLAND_RULES`; quadro SEH só do `new`, sem destrutor no caminho normal): as regras de transferência da Holanda (regra 12): a base (0x8e84a0), a vtable 0x96ba88, 1 janela(s) de `CQM1_HOLANDA`; o +0x13 = 50 (o padrão da base é 50) [inferência: o limite de estrangeiros]; o +0x14 = 50 (o padrão é 30). Devolve o objeto.
unsafe extern "thiscall" fn regras_holanda(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    crate::comparacao::ramo("cqm_1 holanda");
    cqm1_construir(this, [a1, a2, a3, a4, a5], 0x96ba88, 1, &CQM1_HOLANDA, 0x9cf04c, 0x52);
    *((this + 0x13) as *mut u8) = 50;
    *((this + 0x14) as *mut u8) = 50;
    this
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `cqm1_comparar`).
unsafe extern "thiscall" fn comparar_regras_holanda(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(1807);
    let mut r = 0usize;
    cqm1_comparar("regras_holanda", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        regras_holanda(s, a1, a2, a3, a4, a5);
    });
    r
}

/// As janelas da Croácia: uma janela, de 1º de junho a 30 de março.
const CQM1_CROACIA: [[u8; 6]; 2] = [
    [0x05, 0x00, 0xff, 0x01, 0x05, 0x01], // cqm1: a Croácia
    [0x05, 0x00, 0xff, 0x1e, 0x02, 0x00],
];

/// { 0x518500 } (thiscall, ret 0x14, croatia_rules.cpp, `CROATIA_RULES::CROATIA_RULES`; quadro SEH só do `new`, sem destrutor no caminho normal): as regras de transferência da Croácia (regra 5): a base (0x8e84a0), a vtable 0x969394, 1 janela(s) de `CQM1_CROACIA`. Devolve o objeto.
unsafe extern "thiscall" fn regras_croacia(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    crate::comparacao::ramo("cqm_1 croacia");
    cqm1_construir(this, [a1, a2, a3, a4, a5], 0x969394, 1, &CQM1_CROACIA, 0x9b5570, 0x4e);
    this
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `cqm1_comparar`).
unsafe extern "thiscall" fn comparar_regras_croacia(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(1808);
    let mut r = 0usize;
    cqm1_comparar("regras_croacia", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        regras_croacia(s, a1, a2, a3, a4, a5);
    });
    r
}

/// As janelas da Irlanda: uma janela, de 10 de maio a 31 de janeiro.
const CQM1_IRLANDA: [[u8; 6]; 2] = [
    [0x0d, 0x00, 0xff, 0x0a, 0x04, 0x01], // cqm1: a Irlanda
    [0x0d, 0x00, 0xff, 0x1f, 0x00, 0x00],
];

/// { 0x63c210 } (thiscall, ret 0x14, ireland_rules.cpp, `IRELAND_RULES::IRELAND_RULES`; quadro SEH só do `new`, sem destrutor no caminho normal): as regras de transferência da Irlanda (regra 13): a base (0x8e84a0), a vtable 0x96c190, 1 janela(s) de `CQM1_IRLANDA`. Devolve o objeto.
unsafe extern "thiscall" fn regras_irlanda(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    crate::comparacao::ramo("cqm_1 irlanda");
    cqm1_construir(this, [a1, a2, a3, a4, a5], 0x96c190, 1, &CQM1_IRLANDA, 0x9ea12c, 0x4e);
    this
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `cqm1_comparar`).
unsafe extern "thiscall" fn comparar_regras_irlanda(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(1809);
    let mut r = 0usize;
    cqm1_comparar("regras_irlanda", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        regras_irlanda(s, a1, a2, a3, a4, a5);
    });
    r
}
