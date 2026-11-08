use super::*;


/// Uma rodada de copa (registro de 0x68 bytes, docs/.md §2.4): o sorteio (dia, mês com 0 =
/// janeiro, anos a somar ao ano da copa +0x40, dia da semana ou −1) pela 0x521e60; o jogo (o mesmo, mais o turno em +6
/// e o mando em +0x13) pela 0x521eb0; depois os campos: +7 o nome da rodada, +9 o índice, +0xb os bits do sorteio, +0xd
/// e +0xf os desempates (jogo único e ida e volta), +0x17 a categoria copiada em cada jogo, +0x18/+0x1a/+0x1c/+0x1e
/// times, jogos, times novos e inscritos que já entraram, +0x20/+0x21/+0x22 replay, jogos por confronto e dias entre
/// eles, +0x5c/+0x60/+0x64 os prêmios (chegar, vencer, perder).
#[derive(Clone, Copy)]
pub(crate) struct FraSuperCupRound {
    pub(crate) sorteio: (u32, u32, u32, u32),
    pub(crate) jogo: (u32, u32, u32, u32, u32, u32),
    pub(crate) codigo: u16,
    pub(crate) bits: (u16, u16, u16),
    pub(crate) categoria: u8,
    pub(crate) times: (u16, u16, u16, u16),
    pub(crate) confronto: (u8, u8, u8),
    pub(crate) premios: (u32, u32, u32),
}

/// A linha da tabela: (sorteio), (jogo), nome, (bits do sorteio, desempate, desempate de ida e volta), categoria,
/// (times, jogos, novos, inscritos), (replay, jogos por confronto, dias entre eles), (prêmios).
pub(crate) const fn cql12r(sorteio: (u32, u32, u32, u32), jogo: (u32, u32, u32, u32, u32, u32), codigo: u16, bits: (u16, u16, u16), categoria: u8,
    times: (u16, u16, u16, u16), confronto: (u8, u8, u8), premios: (u32, u32, u32)) -> FraSuperCupRound {
    FraSuperCupRound { sorteio, jogo, codigo, bits, categoria, times, confronto, premios }
}

/// Writes round i of a cup round table: draw and match dates and the round fields.
///
/// [pt] Grava a rodada `i` da tabela `t` (ano-base `ano`, a palavra +0x40 da copa com sinal): as duas datas (0x521e60 e
/// 0x521eb0, cdecl, 7 e 9 argumentos; add esp, 0x1c / 0x24) e os campos.
pub(crate) unsafe fn fra_super_write_cup_round(t: usize, i: u32, r: &FraSuperCupRound, year: u32) {
    let sorteio: unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_CUP_SET_ROUND_DRAW_DATE);
    let jogo: unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_CUP_SET_ROUND_MATCH_DATE);
    sorteio(t, i, r.sorteio.0, r.sorteio.1, r.sorteio.2, r.sorteio.3, year);
    jogo(t, i, r.jogo.0, r.jogo.1, r.jogo.2, r.jogo.3, r.jogo.4, year, r.jogo.5);
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

/// Common body of create_round_table for single-stage cups at stage −1, with an adjustment applied to the first round
/// (the French Super Cup venue draw); 0 at other stages.
///
/// [pt] O corpo comum da virtual +0x3c (`create_round_table(fase, &rodadas, &extra, &saida)`, thiscall, ret 0x10) das copas
/// de uma fase só: fora da fase −1 (o byte), devolve 0 sem gravar nada. Na −1: saida (se pedida) = 0, rodadas = o
/// tamanho da tabela, extra = 0; a tabela (0x944e46 de 0x68 × rodadas; sem memória, a caixa `linha` de `arquivo` e
/// devolve 0) com cada rodada; devolve a tabela. `ajuste` mexe na primeira rodada depois da alocação (o mando sorteado da
/// Supercopa francesa).
pub(crate) unsafe fn fra_super_create_single_stage_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32, table: &[FraSuperCupRound], file: usize, row: u32, name: &'static str,
    adjust: impl FnOnce(&mut FraSuperCupRound)) -> usize {
    if stage as u8 != 0xff {
        return 0;
    }
    if !out.is_null() {
        *out = 0;
    }
    *rounds = table.len() as i16;
    *extra = 0;
    let alloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let t = alloc(table.len() as u32 * 0x68);
    if t == 0 {
        show_error_at(file, row);
        return 0;
    }
    crate::comparacao::ramo(name);
    let mut t0 = table[0];
    adjust(&mut t0);
    let year = core::ptr::read_unaligned((this + 0x40) as *const i16) as i32 as u32;
    for (i, r) in table.iter().enumerate() {
        fra_super_write_cup_round(t, i as u32, if i == 0 { &t0 } else { r }, year);
    }
    t
}


/// Side-by-side comparer for create_round_table: same outputs and same written bytes in both round tables; frees the
/// copy's table.
///
/// [pt] Lado a lado: a original com as saídas de quem chama, a cópia com saídas próprias que começam com os mesmos valores, a
/// partir do mesmo [0xb67a34]; as saídas e os bytes gravados das duas tabelas têm de ser iguais (a da cópia é liberada).
pub(crate) unsafe fn fra_super_compare_round_table(name: &str, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32,
    o: impl FnOnce() -> usize, c: impl FnOnce(*mut i16, *mut i16, *mut i32) -> usize) -> usize {
    if stage as u8 != 0xff {
        // simplification: outside stage −1 neither of the two writes anything; the outputs may not even exist
        let a = o();
        PLACAR.registrar(a == 0, || format!("{}(fase {}) devolveu {:#x}", name, stage as u8 as i8, a));
        return a;
    }
    let (mut r2, mut e2) = (*rounds, *extra);
    let mut s2 = if out.is_null() { 0 } else { *out };
    let reg = [(0xb67a34usize, 4usize)];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, o, || c(&mut r2, &mut e2, if out.is_null() { core::ptr::null_mut() } else { &mut s2 }));
    let outs = *rounds == r2 && *extra == e2 && (out.is_null() || *out == s2);
    let table = if a != 0 && b != 0 { eng_cc_cup_written_round_bytes(a, *rounds) == eng_cc_cup_written_round_bytes(b, r2) } else { a == 0 && b == 0 };
    if b != 0 {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
        soltar(b);
    }
    PLACAR.registrar(igual && outs && table, || format!("{}(fase {}): saídas {} ({} x {}) tabela {}", name, stage as u8 as i8, outs, *rounds, r2, table));
    a
}

/// A Supercopa da França: um jogo em julho; o prêmio de 60 mil por chegar. O mando vem à parte (`french_super_cup_venue`).
pub(crate) const FRENCH_SUPER_CUP_ROUNDS: [FraSuperCupRound; 1] = [
    cql12r((5, 6, 0, 5), (0x18, 6, 0, 5, 2, 0x400), 0, (0, 1, 0), 6, (2, 1, 2, 0), (0, 1, 0), (0xea60, 0, 0)), // cql12: o jogo da Supercopa da França
];

/// Venue flag of the French Super Cup: 0x400 in 2001, otherwise one of six flags drawn at random (FN_UTILS_RANDOM(6)).
///
/// [pt] O mando da Supercopa da França: 0x400 em 2001 (+0x40 = 0x7d1); nos outros anos, um dos seis (0x200, 0x400, 0x800,
/// 0x1000, 0x2000, 0x4000) pela 0x90cfc0 (push 6; add esp, 4 -> cdecl, o sorteio de 0 a 5).
pub(crate) unsafe fn french_super_cup_venue(this: usize) -> u32 {
    if core::ptr::read_unaligned((this + 0x40) as *const i16) == 0x7d1 {
        return 0x400;
    }
    let sortear: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    const MANDOS: [i16; 6] = [0x200, 0x400, 0x800, 0x1000, 0x2000, 0x4000];
    MANDOS[sortear(6) as usize] as i32 as u32
}

/// rule: the French Super Cup (Trophée des Champions) is a single match between 2 clubs, drawn on 5/7 and played on
/// 24/7 (moved to weekday 5), decided on penalties [inference: tiebreak 1]; in 2001 at venue 0x400, in other years at
/// one of six drawn venues (0x200 to 0x4000: [inference] the country's largest grounds); 60 thousand to each club for
/// taking part; category 6
/// Original: `FraSuper::create_round_table` (0x5bf5d0, fra_super.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x5bf5d0 } (thiscall, ret 0x10, fra_super.cpp; a virtual +0x3c da Supercopa da França): `FraSuper::
/// create_round_table(fase, &rodadas, &extra, &saida)`: na fase −1, a rodada única com o mando de
/// `french_super_cup_venue` (a original sorteia depois da data do sorteio, que não sorteia: a cópia sorteia logo antes);
/// a caixa da linha 0x97 sem memória.
pub(crate) unsafe extern "thiscall" fn fra_super_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    fra_super_create_single_stage_round_table(this, stage, rounds, extra, out, &FRENCH_SUPER_CUP_ROUNDS, 0x9ccc48, 0x97, "cql_12 tabela da supercopa da franca",
        |r| r.jogo.5 = french_super_cup_venue(this))
}

// simplification: outside 2001 the original rolls home advantage (0x90cfc0): only it runs in that case.

/// A Supercopa da Espanha: ida e volta (7 dias entre os jogos) em agosto, sem sorteio (+0xd = 0x200), desempate de ida e
/// volta 1.
pub(crate) const SPANISH_SUPER_CUP: [FraSuperCupRound; 1] = [
    cql12r((5, 6, 0, 5), (0x14, 7, 0, 6, 1, 0), 0, (0, 0x200, 1), 6, (2, 1, 2, 0), (0, 2, 7), (0, 0, 0)), // cql12: a ida da Supercopa da Espanha
];



/// A Supercopa do Japão: um jogo em março no estádio neutro, 150 mil por vencer.
pub(crate) const JAPAN_SUPER_CUP_ROUNDS: [FraSuperCupRound; 1] = [
    cql12r((1, 1, 0, 5), (4, 2, 0, 5, 1, 4), 0, (0, 1, 0), 6, (2, 1, 2, 0), (0, 1, 0), (0, 0x249f0, 0)), // cql12: o jogo da Supercopa do Japão
];

/// rule: the Japanese Super Cup (Xerox Super Cup) is a single match between 2 clubs, drawn on 1/2 and played on 4/3
/// (moved to weekday 5) at a neutral ground [inference: venue 4], decided on penalties [inference: tiebreak 1]; 150
/// thousand to the winner; category 6
/// Original: `JapSuper::create_round_table` (0x666020, jap_super.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x666020 } (thiscall, ret 0x10, jap_super.cpp; a virtual +0x3c da Supercopa do Japão): `JapSuper::
/// create_round_table(fase, &rodadas, &extra, &saida)`: na fase −1, a rodada única; a caixa da linha 0x98 sem memória.
pub(crate) unsafe extern "thiscall" fn fra_super_create_round_table_666020(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    fra_super_create_single_stage_round_table(this, stage, rounds, extra, out, &JAPAN_SUPER_CUP_ROUNDS, 0x9eaf4c, 0x98, "cql_12 tabela da supercopa do japao", |_| {})
}


/// rule: the French Super Cup is between the First Division champion and the Coupe de France winner; if the same club
/// won both, the First Division runner-up takes its place; if one is missing, the error box
/// Original: `FraSuper::select_teams` (0x5bf770, fra_super.cpp, thiscall, ret 0).
///
/// [pt] regra: a Supercopa da França é entre o campeão da Primeira e o da Copa da França; se o mesmo clube ganhou as duas,
/// entra o vice da Primeira; sem algum deles, a caixa de erro
pub(crate) unsafe extern "thiscall" fn fra_super_select_teams(this: usize) -> i32 {
    french_super_cup_select_teams(this).0
}

