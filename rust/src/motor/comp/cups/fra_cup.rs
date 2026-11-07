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
    let sorteio: unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_CUP_SET_ROUND_DRAW_DATE);
    let jogo: unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_CUP_SET_ROUND_MATCH_DATE);
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
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
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
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
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

/// rule: the Coupe de France has 9 single-leg rounds, from the 6th national round (40 teams) to the final: 6th drawn
/// 10/10 (no fixed weekday), played 6/11; 7th 12/11 and 27/11 (164 clubs enter, 184 teams); 8th 3/12 and 18/12; last
/// 128 6/1 and 22/1 (18 enter); last 64 27/1 and 12/2; last 16 20/2 and 4/3; quarter-finals 5/3 and 19/3; semi-final
/// 20/3 and 12/4; final drawn 13/4, played 6/5 at a neutral ground [inference: venue 4]; from the 7th round to the
/// semi-final the lower-division club plays at home (+0xb = 1); the prize (+0x64) of 100 thousand in the last 16, 200
/// thousand in the quarter-finals, 400 thousand in the semi-final, and in the final 1 million (+0x60) and 800 thousand
/// (+0x64); category 4 up to the last 16 and 6 after
/// Original: `FraCup::create_round_table` (0x5ba3f0, fra_cup.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x5ba3f0 } (thiscall, ret 0x10, fra_cup.cpp; a virtual +0x3c da Copa da França): `FraCup::create_round_table(fase,
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

/// rule: the French League Cup has 7 single-leg rounds: preliminary (code 0xe6, 16 teams) drawn 1/9, played 5/10; 1st
/// round 15/10 and 16/11 (28 teams, 20 new); 2nd 26/11 and 8/1 (32 teams, 18 new); 3rd 14/1 and 29/1; quarter-finals
/// 3/2 and 19/2; semi-final 4/3 and 1/4; final 2/4 and 22/4 at a neutral ground [inference: venue 4]; 300 thousand for
/// winning each round and 1.5 million to the winner; category 4 up to the quarter-finals and 6 after
/// Original: `FraLgeCup::create_round_table` (0x5bcc40, fra_lge_cup.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x5bcc40 } (thiscall, ret 0x10, fra_lge_cup.cpp; a virtual +0x3c da Copa da Liga francesa): `FraLgeCup::
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

/// rule: the Copa del Rey has 7 rounds: preliminary (code 0xe6, 38 teams, 7 days between legs) drawn 18/8, played 30/8;
/// 1st round 8/9 and 13/12 (64 teams, 45 new); 2nd 15/12 and 3/1; 3rd (last 16) 5/1 and 10/1, quarter-finals 19/1 and
/// 31/1 and semi-final 23/2 and 11/4 over two legs (7, 7 and 14 days apart); final drawn 26/4, played 26/5 at a neutral
/// ground [inference: venue 4]; up to the semi-final the lower-division club plays at home and clubs of the same nation
/// are kept apart in the draw (+0xb = 3); no prize per round (−1); category 4 up to the last 16 and 6 after
/// Original: `SpaCup::create_round_table` (0x84d220, spa_cup.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x84d220 } (thiscall, ret 0x10, spa_cup.cpp; a virtual +0x3c da Copa do Rei): `SpaCup::create_round_table(fase,
/// &rodadas, &extra, &saida)`: na fase −1, as 7 rodadas; a caixa da linha 0xa8 sem memória.
unsafe extern "thiscall" fn cql11_copa_do_rei_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    cql11_tabela(this, fase, rodadas, extra, saida, &CQL11_COPA_DO_REI, 0xa82810, 0xa8, "cql_11 tabela da copa do rei")
}

unsafe extern "thiscall" fn comparar_cql11_copa_do_rei_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1777);
    cql11_comparar("cql11_copa_do_rei_tabela", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| cql11_copa_do_rei_tabela(this, fase, r, e, s))
}
// ---------- 276a leva: os inscritos das copas francesas: a Copa da França, a Copa da Liga francesa e os dois clubes da Supercopa (fra_cup.cpp, fra_lge_cup.cpp, fra_super.cpp) ----------



/// O clube `i` da tabela dos clubes ([0xae23b4], registros de 0x245 bytes).
unsafe fn cqm11_clube(i: i16) -> usize {
    read_u32_as_usize(0xae23b4) + i as isize as usize * 0x245
}

/// O id da competição apontada por `c + o` (+0 do registro), ou None sem ela.
unsafe fn cqm11_comp(c: usize, o: usize) -> Option<usize> {
    let d = read_u32_as_usize(c + o);
    if d == 0 { None } else { Some(read_u32_as_usize(d)) }
}

/// Grava a inscrição `n` da lista (+0xa7, 6 bytes: o clube, +4 classe 0, +5 situação 0) e conta.
unsafe fn cqm11_inscrever(this: usize, n: &mut i32, clube: usize) {
    let e = read_u32_as_usize(this + 0xa7) + *n as usize * 6;
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
            if para && *n == read_i16(this + 0x3e) as i32 {
                return;
            }
        }
        i = i.wrapping_add(1);
    }
}

/// Os clubes da divisão [id] enquanto cabem (n < +0x3e), até encher.
unsafe fn cqm11_divisao(this: usize, n: &mut i32, id: usize) {
    cqm11_passada(this, n, || read_u32_as_usize(0xae235c) as i32, true, |c, n| cqm11_comp(c, 0x57) == Some(read_u32_as_usize(id)) && n < read_i16(this + 0x3e) as i32);
}

/// O limite dos amadores da Copa da França: 0xb8 em 2001 (+0x40 = 0x7d1), 0xb6 nos outros anos.
unsafe fn cqm11_limite_franca(this: usize) -> i32 {
    if read_i16(this + 0x40) == 0x7d1 { 0xb8 } else { 0xb6 }
}

/// { 0x5baa50 } (thiscall, ret 0, fra_cup.cpp): `FraCup::collect_entrants()`: +0x3e = 222; a lista (0x944e46 de 0x534 em
/// +0xa7; sem memória, a caixa da linha 0x16f e −1). Na ordem: todos os da divisão [0x9cf5d4] (sem conferir o espaço); os
/// da divisão [0x9cf764] ligados (+0x60) à [0x9cf5d8]; os franceses (+0x53 = [0x9cf300]) da [0x9cf764] sem ligação; os
/// franceses sem divisão que não são seleções (0x53b900), entre os clubes antes das 2 × [0xae2350] seleções do fim — estas
/// três até `cqm11_limite_franca`; a Segunda ([0x9cf5d0]) e a Primeira ([0x9cf5cc]) enquanto cabem. Devolve 1 com a lista
/// cheia, senão 0 (e quantos entraram, para o comparador).
unsafe fn cqm11_copa_da_franca(this: usize) -> (i32, i32) {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 0xde);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let l = alocar(0x534);
    core::ptr::write_unaligned((this + 0xa7) as *mut u32, l as u32);
    if l == 0 {
        aviso_em(0x9ccb28, 0x16f);
        return (-1, 0);
    }
    let mut n = 0i32;
    let todos = || read_u32_as_usize(0xae235c) as i32;
    cqm11_passada(this, &mut n, todos, false, |c, _| cqm11_comp(c, 0x57) == Some(read_u32_as_usize(KEY_CLUB_COMP_FRENCH_NATIONAL)));
    cqm11_passada(this, &mut n, todos, true, |c, n| {
        cqm11_comp(c, 0x57) == Some(read_u32_as_usize(KEY_CLUB_COMP_A_LOWER_DIVISION)) && cqm11_comp(c, 0x60) == Some(read_u32_as_usize(KEY_CLUB_COMP_FRENCH_CFA)) && n < cqm11_limite_franca(this)
    });
    cqm11_passada(this, &mut n, todos, true, |c, n| {
        cqm11_comp(c, 0x53) == Some(read_u32_as_usize(KEY_NATION_FRANCE)) && cqm11_comp(c, 0x57) == Some(read_u32_as_usize(KEY_CLUB_COMP_A_LOWER_DIVISION)) && read_u32_as_usize(c + 0x60) == 0
            && n < cqm11_limite_franca(this)
    });
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    cqm11_passada(this, &mut n, || read_u32_as_usize(0xae235c) as i32 - 2 * read_u32_as_usize(0xae2350) as i32, true, |c, n| {
        read_u32_as_usize(c + 0x53) != 0 && selecao(c) == 0 && read_u32_as_usize(c + 0x57) == 0 && cqm11_comp(c, 0x53) == Some(read_u32_as_usize(KEY_NATION_FRANCE))
            && n < cqm11_limite_franca(this)
    });
    crate::comparacao::ramo("cqm_11 inscritos da copa da franca");
    cqm11_divisao(this, &mut n, KEY_CLUB_COMP_FRENCH_SECOND_DIVISION); // cqm11: a Segunda francesa na Copa da França
    cqm11_divisao(this, &mut n, KEY_CLUB_COMP_FRENCH_FIRST_DIVISION); // cqm11: a Primeira francesa na Copa da França
    ((n == read_i16(this + 0x3e) as i32) as i32, n)
}

/// rule: the Coupe de France takes 222 clubs, in this order (the order decides the entry round: the first 40 in the 6th
/// round, the next 164 in the 7th and the last 18, the First Division, in the last 128): all clubs of division
/// [0x9cf5d4] (the National [inference]); those of [0x9cf764] linked (+0x60) to [0x9cf5d8] (the CFA [inference]); the
/// French ones of [0x9cf764] without a link; the French clubs with no division that are not national teams — these
/// three groups up to 184 entrants in 2001 and 182 in other years; the Second and the First Division; with an
/// incomplete list it returns 0
/// Original: `FraCup::collect_entrants` (0x5baa50, fra_cup.cpp, thiscall, ret 0).
///
/// [pt] regra: entram na Copa da França 222 clubes, nesta ordem (a ordem decide a rodada de entrada: os 40 primeiros na 6ª
/// rodada, os 164 seguintes na 7ª e os 18 últimos, a Primeira, nos 64 avos): todos os da divisão [0x9cf5d4] (o National
/// [inferência]); os da [0x9cf764] ligados (+0x60) à [0x9cf5d8] (o CFA [inferência]); os franceses da [0x9cf764] sem
/// ligação; os franceses sem divisão que não são seleções — estes três grupos até 184 inscritos em 2001 e 182 nos
/// outros anos; a Segunda e a Primeira; com a lista incompleta, devolve 0
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
        lo.set(read_u32_as_usize(this + 0xa7));
        r
    }, || {
        let (r, n) = c();
        lr.set(read_u32_as_usize(this + 0xa7));
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
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
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
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let l = alocar(0x144);
    core::ptr::write_unaligned((this + 0xa7) as *mut u32, l as u32);
    if l == 0 {
        aviso_em(0x9ccb98, 0x146);
        return (-1, 0);
    }
    let mut n = 0i32;
    let todos = || read_u32_as_usize(0xae235c) as i32;
    let limite = if read_i16(this + 0x40) == 0x7d1 { 0x10 } else { 0xe };
    cqm11_passada(this, &mut n, todos, true, |c, n| {
        cqm11_comp(c, 0x57) == Some(read_u32_as_usize(KEY_CLUB_COMP_FRENCH_NATIONAL)) && read_u32_as_usize(c + 0x5b) != 0 && cqm11_comp(c, 0x5b) != Some(read_u32_as_usize(KEY_CLUB_COMP_FRENCH_CFA)) && n < limite
    });
    // a original testa o tamanho antes de cada clube: a passada para quando a lista chega a 16
    let mut i: i16 = 0;
    while (i as i32) < read_u32_as_usize(0xae235c) as i32 && n * 6 < 0x60 {
        let c = cqm11_clube(i);
        if cqm11_comp(c, 0x57) == Some(read_u32_as_usize(KEY_CLUB_COMP_FRENCH_NATIONAL)) && cqm11_comp(c, 0x5b) == Some(read_u32_as_usize(KEY_CLUB_COMP_FRENCH_CFA)) {
            cqm11_inscrever(this, &mut n, c);
            if n == read_i16(this + 0x3e) as i32 {
                break;
            }
        }
        i = i.wrapping_add(1);
    }
    crate::comparacao::ramo("cqm_11 inscritos da copa da liga francesa");
    cqm11_divisao(this, &mut n, KEY_CLUB_COMP_FRENCH_SECOND_DIVISION); // cqm11: a Segunda francesa na Copa da Liga
    cqm11_divisao(this, &mut n, KEY_CLUB_COMP_FRENCH_FIRST_DIVISION); // cqm11: a Primeira francesa na Copa da Liga
    ((n == read_i16(this + 0x3e) as i32) as i32, n)
}

/// rule: the French League Cup takes 54 clubs, in this order (the first 16 in the preliminary round, the next 20 in the
/// 1st round and the last 18 in the 2nd): those of division [0x9cf5d4] that did not come from [0x9cf5d8] (up to 16 in
/// 2001 and 14 in other years), topped up to 16 with those of [0x9cf5d4] that came from [0x9cf5d8] (the promoted ones
/// [inference]); the Second and the First Division; with an incomplete list it returns 0
/// Original: `FraLgeCup::collect_entrants` (0x5bd160, fra_lge_cup.cpp, thiscall, ret 0).
///
/// [pt] regra: entram na Copa da Liga francesa 54 clubes, nesta ordem (os 16 primeiros na preliminar, os 20 seguintes na 1ª
/// rodada e os 18 últimos na 2ª): os da divisão [0x9cf5d4] que não vieram da [0x9cf5d8] (até 16 em 2001 e 14 nos outros
/// anos), completados até 16 com os da [0x9cf5d4] que vieram da [0x9cf5d8] (os promovidos [inferência]); a Segunda e a
/// Primeira; com a lista incompleta, devolve 0
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
    let colocado: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_COMP_UTIL_GET_LATEST_PLACED_CLUB);
    colocado(read_u32_as_usize(0xae23d0).wrapping_add(read_u32_as_usize(id).wrapping_mul(0x6b)), lugar)
}

/// { 0x5bf770 } (thiscall, ret 0, fra_super.cpp): `FraSuper::select_teams()`: +0x3e = 2; a lista (0x944e46 de 0xc em
/// +0xa7; sem memória, a caixa da linha 0xc4 e −1). O campeão da Primeira ([0x9cf5cc]; sem ele, a caixa 0xcd e 0) e o
/// da Copa da França ([0x9cf70c]; sem ele, 0xd6 e 0); se é o mesmo clube, o segundo vira o vice da Primeira (sem ele, 0xe2
/// e 0). Classe e situação 0 nos dois; devolve 1.
unsafe fn cqm11_supercopa(this: usize) -> (i32, i32) {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 2);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let l = alocar(0xc);
    core::ptr::write_unaligned((this + 0xa7) as *mut u32, l as u32);
    if l == 0 {
        aviso_em(0x9ccc48, 0xc4);
        return (-1, 0);
    }
    let a = cqm11_colocado(KEY_CLUB_COMP_FRENCH_FIRST_DIVISION, 0); // cqm11: o campeão francês
    core::ptr::write_unaligned(l as *mut u32, a as u32);
    if a == 0 {
        aviso_em(0x9ccc48, 0xcd);
        return (0, 0);
    }
    let b = cqm11_colocado(KEY_CLUB_COMP_FRENCH_CUP, 0);
    core::ptr::write_unaligned((l + 6) as *mut u32, b as u32);
    if b == 0 {
        aviso_em(0x9ccc48, 0xd6);
        return (0, 0);
    }
    if a == b {
        crate::comparacao::ramo("cqm_11 dobradinha francesa");
        let b = cqm11_colocado(KEY_CLUB_COMP_FRENCH_FIRST_DIVISION, 1);
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

/// rule: the French Super Cup is between the First Division champion and the Coupe de France winner; if the same club
/// won both, the First Division runner-up takes its place; if one is missing, the error box
/// Original: `FraSuper::select_teams` (0x5bf770, fra_super.cpp, thiscall, ret 0).
///
/// [pt] regra: a Supercopa da França é entre o campeão da Primeira e o da Copa da França; se o mesmo clube ganhou as duas,
/// entra o vice da Primeira; sem algum deles, a caixa de erro
unsafe extern "thiscall" fn cqm11_supercopa_clubes(this: usize) -> i32 {
    cqm11_supercopa(this).0
}

unsafe extern "thiscall" fn comparar_cqm11_supercopa_clubes(this: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> i32>(1786);
    cqm11_comparar("cqm11_supercopa_clubes", this, &[(this + 0x3e, 2), (0xb67a34, 4)], || o(this), || cqm11_supercopa(this),
        |r, _| if r == 1 { 12 } else { 0 })
}

