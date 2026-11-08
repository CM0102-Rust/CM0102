// ---------- %%LEVA%%a leva: o calendário das copas inglesas: as rodadas da Copa da Liga, da Copa da Inglaterra e da Supercopa (eng_cc_cup.cpp, eng_fa_cup.cpp, eng_charity.cpp) ----------

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
unsafe fn eng_cc_cup_write_cup_round(t: usize, i: u32, r: &Cql10R, ano: u32) {
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
unsafe fn eng_cc_cup_create_single_stage_round_table(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32, tabela: &[Cql10R], arquivo: usize, linha: u32, nome: &'static str) -> usize {
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
        eng_cc_cup_write_cup_round(t, i as u32, r, ano);
    }
    t
}

/// Os bytes que a original grava em cada rodada (o resto da memória alocada fica como veio): +0..+0x10 (as datas e os
/// campos até o desempate de ida e volta), +0x13..+0x22 (o mando até os dias entre os jogos) e +0x5c..+0x67 (os prêmios).
unsafe fn eng_cc_cup_written_round_bytes(t: usize, n: i16) -> Vec<u8> {
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
unsafe fn eng_cc_cup_compare_round_table(nome: &str, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32,
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
    let tabela = if a != 0 && b != 0 { eng_cc_cup_written_round_bytes(a, *rodadas) == eng_cc_cup_written_round_bytes(b, r2) } else { a == 0 && b == 0 };
    if b != 0 {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(b);
    }
    PLACAR.registrar(igual && saidas && tabela, || format!("{}(fase {}): saídas {} ({} x {}) tabela {}", nome, fase as u8 as i8, saidas, *rodadas, r2, tabela));
    a
}

/// A Copa da Liga inglesa (EngCcCup): sete rodadas de agosto a março do ano seguinte; as três primeiras dependem de
/// +0xb2 (os clubes nas copas europeias, que entram mais tarde) e vêm à parte (`league_cup_rounds`).
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
fn league_cup_rounds(b: i16) -> [Cql10R; 7] {
    let mut t = CQL10_COPA_DA_LIGA;
    let x = |v: i16| v as u16;
    t[0].times = (x(b.wrapping_mul(2).wrapping_add(0x38)), x(b.wrapping_add(0x1c)), x(b.wrapping_mul(2).wrapping_add(0x38)), 0);
    t[1].times = (x(0x20i16.wrapping_sub(b).wrapping_mul(2)), x(0x20i16.wrapping_sub(b)), x(0xci16.wrapping_sub(b).wrapping_mul(3)), x(b.wrapping_mul(2).wrapping_add(0x38)));
    t[2].times.2 = x(b);
    t[2].times.3 = x(0x5ci16.wrapping_sub(b));
    t
}

/// { 0x56c570 } (thiscall, ret 0x10, eng_cc_cup.cpp; a virtual +0x3c da Copa da Liga): `EngCcCup::create_round_table(
/// fase, &rodadas, &extra, &saida)`: na fase −1, as 7 rodadas (`league_cup_rounds` com o +0xb2); a caixa da
/// linha 0xa2 sem memória.
unsafe extern "thiscall" fn eng_cc_cup_create_round_table(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let t = league_cup_rounds(*((this + 0xb2) as *const i8) as i16);
    eng_cc_cup_create_single_stage_round_table(this, fase, rodadas, extra, saida, &t, 0x9c4224, 0xa2, "cql_10 tabela da copa da liga")
}

unsafe extern "thiscall" fn comparar_eng_cc_cup_create_round_table(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    eng_cc_cup_compare_round_table("eng_cc_cup_create_round_table", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| eng_cc_cup_create_round_table(this, fase, r, e, s))
}

/// A Copa da Inglaterra (EngFaCup): nove rodadas de outubro a maio do ano seguinte, com replay (+0x20 = 1) até as
/// quartas; as rodadas 2 a 5 dependem de +0xb3 (isenções) e +0xb4 (a escolha da entrada na 5ª rodada) e do +0x3e (os
/// inscritos) e vêm à parte (`fa_cup_rounds`). A original grava ainda +0x13 = 0 nas rodadas 0 a 6,
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
fn fa_cup_rounds(b: i16, entrada: u8, inscritos: i16) -> [Cql10R; 9] {
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
/// create_round_table(fase, &rodadas, &extra, &saida)`: na fase −1, as 9 rodadas (`fa_cup_rounds`);
/// a caixa da linha 0xb6 sem memória.
unsafe extern "thiscall" fn eng_cc_cup_create_round_table_56f320(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let t = fa_cup_rounds(*((this + 0xb3) as *const i8) as i16, *((this + 0xb4) as *const u8), core::ptr::read_unaligned((this + 0x3e) as *const i16));
    eng_cc_cup_create_single_stage_round_table(this, fase, rodadas, extra, saida, &t, 0x9c42cc, 0xb6, "cql_10 tabela da copa da inglaterra")
}

unsafe extern "thiscall" fn comparar_eng_cc_cup_create_round_table_56f320(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I1%%);
    eng_cc_cup_compare_round_table("eng_cc_cup_create_round_table_56f320", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| eng_cc_cup_create_round_table_56f320(this, fase, r, e, s))
}

/// A Supercopa inglesa (Charity Shield): uma rodada, um jogo em agosto.
const CQL10_SUPERCOPA: [Cql10R; 1] = [
    cql10r((5, 6, 0, 5), (0xd, 7, 0, 6, 1, 4), 0, (0, 1, 0), 6, (2, 1, 2, 0), (0, 1, 0), (0, 0, 0)), // cql10: o jogo da Supercopa inglesa
];

/// { 0x56d640 } (thiscall, ret 0x10, eng_charity.cpp; a virtual +0x3c da Supercopa): `EngCharity::create_round_table(
/// fase, &rodadas, &extra, &saida)`: na fase −1, a rodada única; a caixa da linha 0x9c sem memória.
unsafe extern "thiscall" fn eng_cc_cup_create_round_table_56d640(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    eng_cc_cup_create_single_stage_round_table(this, fase, rodadas, extra, saida, &CQL10_SUPERCOPA, 0x9c425c, 0x9c, "cql_10 tabela da supercopa inglesa")
}

unsafe extern "thiscall" fn comparar_eng_cc_cup_create_round_table_56d640(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I2%%);
    eng_cc_cup_compare_round_table("eng_cc_cup_create_round_table_56d640", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| eng_cc_cup_create_round_table_56d640(this, fase, r, e, s))
}
