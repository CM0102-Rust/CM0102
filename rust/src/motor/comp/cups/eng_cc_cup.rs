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
unsafe fn cql10_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32, tabela: &[Cql10R], arquivo: usize, linha: u32, nome: &'static str) -> usize {
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
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
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

// ---------- 276a leva: os inscritos das copas inglesas: a Copa da Liga, a segunda chave do Troféu da Liga e os dois clubes da Supercopa (eng_cc_cup.cpp, eng_auto_cup.cpp, eng_charity.cpp) ----------



/// O clube `i` da tabela dos clubes ([0xae23b4], registros de 0x245 bytes).
unsafe fn cqm10_clube(i: i16) -> usize {
    read_u32_as_usize(0xae23b4) + i as isize as usize * 0x245
}

/// O id da competição da divisão do clube (+0x57, ponteiro para o registro da competição; +0 o id), ou None sem divisão.
unsafe fn cqm10_divisao(c: usize) -> Option<usize> {
    let d = read_u32_as_usize(c + 0x57);
    if d == 0 { None } else { Some(read_u32_as_usize(d)) }
}

/// Grava a inscrição `k` da lista (+0xa7, 6 bytes: o clube, +4 a classe, +5 a situação).
unsafe fn cqm10_inscrever(this: usize, k: i32, clube: usize, classe: u8) {
    let e = read_u32_as_usize(this + 0xa7) + k as usize * 6;
    core::ptr::write_unaligned(e as *mut u32, clube as u32);
    *((e + 4) as *mut u8) = classe;
    *((e + 5) as *mut u8) = 0;
}

/// Uma passada pelos clubes da divisão `id` ([id], o id da competição) fora das copas europeias (+0x1db = −1), até a
/// lista encher (+0x3e); a classe de cada um vem de `classe(clube)`.
unsafe fn cqm10_passada(this: usize, n: &mut i32, id: usize, classe: impl Fn(usize) -> u8) {
    let mut i: i16 = 0;
    while (i as i32) < read_u32_as_usize(0xae235c) as i32 {
        let c = cqm10_clube(i);
        if let Some(d) = cqm10_divisao(c) {
            if *n != read_i16(this + 0x3e) as i32 && d == read_u32_as_usize(id) && read_u32_as_usize(c + 0x1db) as i32 == -1 {
                cqm10_inscrever(this, *n, c, classe(c));
                *n += 1;
                if *n == read_i16(this + 0x3e) as i32 {
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
    let p = read_u32_as_usize(c + 0x5b);
    p != 0 && read_u32_as_usize(p) == read_u32_as_usize(de)
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
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let l = alocar(0x228);
    core::ptr::write_unaligned((this + 0xa7) as *mut u32, l as u32);
    if l == 0 {
        aviso_em(0x9c4224, 0x18f);
        return (-1, 0);
    }
    let mut n = 0i32;
    cqm10_passada(this, &mut n, KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION, |_| 0);
    cqm10_passada(this, &mut n, KEY_CLUB_COMP_ENGLISH_SECOND_DIVISION, |_| 0);
    cqm10_passada(this, &mut n, KEY_CLUB_COMP_ENGLISH_FIRST_DIVISION, |c| {
        let pos = *((c + 0x5f) as *const u8);
        if cqm10_veio(c, KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION) {
            crate::comparacao::ramo("cqm_10 caiu da premier");
            0x46u8.wrapping_sub(pos) // cqm10: quem caiu da Premier
        } else if cqm10_veio(c, KEY_CLUB_COMP_ENGLISH_FIRST_DIVISION) {
            0x28u8.wrapping_sub(pos)
        } else {
            0
        }
    });
    cqm10_passada(this, &mut n, KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION, |c| {
        let pos = *((c + 0x5f) as *const u8);
        if cqm10_veio(c, KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION) {
            0x78u8.wrapping_sub(pos)
        } else if cqm10_veio(c, KEY_CLUB_COMP_ENGLISH_FIRST_DIVISION) {
            0x64u8.wrapping_sub(pos)
        } else {
            0
        }
    });
    let mut i: i16 = 0;
    while (i as i32) < read_u32_as_usize(0xae235c) as i32 {
        let c = cqm10_clube(i);
        if let Some(d) = cqm10_divisao(c) {
            if n != read_i16(this + 0x3e) as i32
                && (d == read_u32_as_usize(KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION) || d == read_u32_as_usize(KEY_CLUB_COMP_ENGLISH_FIRST_DIVISION) || d == read_u32_as_usize(KEY_CLUB_COMP_ENGLISH_SECOND_DIVISION) || d == read_u32_as_usize(KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION))
                && read_u32_as_usize(c + 0x1db) as i32 != -1
            {
                crate::comparacao::ramo("cqm_10 clube nas copas europeias");
                *((this + 0xb2) as *mut u8) = (*((this + 0xb2) as *const u8)).wrapping_add(1);
                cqm10_inscrever(this, n, c, 0x78);
                n += 1;
                if n == read_i16(this + 0x3e) as i32 {
                    break;
                }
            }
        }
        i = i.wrapping_add(1);
    }
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    ordenar(read_u32_as_usize(this + 0xa7), n as u32, 6, 0x5221c0);
    crate::comparacao::ramo("cqm_10 inscritos da copa da liga");
    let mut k: i16 = 0;
    while (k as i32) < n {
        let e = read_u32_as_usize(this + 0xa7) + k as usize * 6;
        let c = read_u32_as_usize(e);
        let d = if c != 0 { cqm10_divisao(c) } else { None };
        *((e + 4) as *mut u8) = match d {
            Some(x) if x == read_u32_as_usize(KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION) => 1, // cqm10: a classe da Premier
            Some(x) if x == read_u32_as_usize(KEY_CLUB_COMP_ENGLISH_FIRST_DIVISION) => 2,
            _ => 3,
        };
        k = k.wrapping_add(1);
    }
    if n != read_i16(this + 0x3e) as i32 {
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
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let l = alocar(0xa8);
    core::ptr::write_unaligned((this + 0xa7) as *mut u32, l as u32);
    if l == 0 {
        aviso_em(0x9c4180, 0x19f);
        return (-1, 0);
    }
    crate::comparacao::ramo("cqm_10 segunda chave do trofeu da liga");
    let mut k: i16 = 0;
    if read_i16(this + 0x3e) > 0 {
        loop {
            let c = read_u32_as_usize(read_u32_as_usize(this + 0xb2) + 0x70 + k as usize * 4); // cqm10: o k-ésimo da segunda metade
            cqm10_inscrever(this, k as i32, c, 0);
            k = k.wrapping_add(1);
            if k >= read_i16(this + 0x3e) {
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
    let colocado: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_COMP_UTIL_GET_LATEST_PLACED_CLUB);
    colocado(read_u32_as_usize(0xae23d0).wrapping_add(read_u32_as_usize(id).wrapping_mul(0x6b)), lugar)
}

/// { 0x56d780 } (thiscall, ret 0, eng_charity.cpp): `EngCharity::select_teams()`: +0x3e = 2; a lista (0x944e46 de 0xc em
/// +0xa7; sem memória, a caixa da linha 0xc5 e −1). O campeão da Premier ([0x9cf5bc]; sem ele, a caixa 0xce e 0) e o da
/// Copa da Inglaterra ([0x9cf74c]; sem ele, 0xd7 e 0); se é o mesmo clube, o segundo vira o vice da Premier (sem ele,
/// 0xe3 e 0). Classe e situação 0 nos dois; devolve 1.
unsafe fn cqm10_supercopa(this: usize) -> (i32, i32) {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 2);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let l = alocar(0xc);
    core::ptr::write_unaligned((this + 0xa7) as *mut u32, l as u32);
    if l == 0 {
        aviso_em(0x9c425c, 0xc5);
        return (-1, 0);
    }
    let a = cqm10_colocado(KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION, 0); // cqm10: o campeão inglês
    core::ptr::write_unaligned(l as *mut u32, a as u32);
    if a == 0 {
        aviso_em(0x9c425c, 0xce);
        return (0, 0);
    }
    let b = cqm10_colocado(KEY_CLUB_COMP_ENGLISH_FA_CUP, 0);
    core::ptr::write_unaligned((l + 6) as *mut u32, b as u32);
    if b == 0 {
        aviso_em(0x9c425c, 0xd7);
        return (0, 0);
    }
    if a == b {
        crate::comparacao::ramo("cqm_10 dobradinha inglesa");
        let b = cqm10_colocado(KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION, 1);
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

// ---------- 276a leva: o dia das copas: a virada da Copa da Liga inglesa, as inscrições da Copa da Inglaterra e a convocação do Jogo das Estrelas (eng_cc_cup.cpp, eng_fa_cup.cpp, usa_mls_all_stars.cpp) ----------



/// A primeira dword de Date(dia, mês, ano, dia da semana) (0x549f70: push semana; push ano; push mês; push dia; lea ecx,
/// local -> thiscall, ret 0x10): a palavra de baixo é o dia do ano, a de cima o ano.
unsafe fn cqn10_data(dia: u32, mes: u32, ano: u32, semana: u32) -> u32 {
    let construir: unsafe extern "thiscall" fn(*mut [u32; 2], u32, u32, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE_549F70);
    let mut d = [0u32; 2];
    construir(&mut d, dia, mes, ano, semana);
    d[0]
}

/// A data `d` (dword de `cqn10_data`) é `hoje` (o Date de quem chama: +0 o dia do ano, +2 o ano)? A original compara o
/// ano e depois o dia.
unsafe fn cqn10_e_hoje(d: u32, hoje: usize) -> bool {
    (d >> 16) as u16 == read_i16(hoje + 2) as u16 && d as u16 == read_i16(hoje) as u16
}

/// Libera as tabelas da temporada antes de refazer: a lista dos inscritos (+0xa7), a das rodadas (+0xa3) e os confrontos
/// por rodada (+0xad, um vetor por rodada, +0x3c rodadas), cada um se existe (0x9452ca, cdecl). Os ponteiros ficam como
/// estão (as rotinas seguintes os refazem).
unsafe fn cqn10_liberar(this: usize) {
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    if read_u32_as_usize(this + 0xa7) != 0 {
        soltar(read_u32_as_usize(this + 0xa7));
    }
    if read_u32_as_usize(this + 0xa3) != 0 {
        soltar(read_u32_as_usize(this + 0xa3));
    }
    if read_u32_as_usize(this + 0xad) != 0 {
        let mut i = 0i32;
        while i < read_i16(this + 0x3c) as i32 {
            let p = read_u32_as_usize(read_u32_as_usize(this + 0xad) + i as usize * 4);
            if p != 0 {
                soltar(p);
            }
            i += 1;
        }
        soltar(read_u32_as_usize(this + 0xad));
    }
}

/// A virtual `o` da copa (thiscall, sem argumentos; mov ecx, copa; call [vtable + o]).
unsafe fn cqn10_virtual(this: usize, o: usize) -> i32 {
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(this) + o));
    f(this)
}

/// A atualização da copa (0x51f890: push fase; push hoje; mov ecx, copa -> thiscall, ret 8), cuja devolução sai.
unsafe fn cqn10_base(this: usize, hoje: usize, fase: u32) -> u32 {
    let base: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_CUP_UPDATE_FOR_DATE);
    base(this, hoje, fase)
}

/// { 0x56caf0 } (thiscall, ret 8, eng_cc_cup.cpp; a virtual +0x14 da Copa da Liga): `EngCcCup::update_for_date(hoje,
/// fase)`. Uma vez (+0x45 = 0), quando o ano do jogo ([0xde1f58] +0x439, palavra sem sinal) chega ao da copa (+0x40, com
/// sinal) e a fase não é 0: libera as tabelas (`cqn10_liberar`), +0x45 = 1, os inscritos (0x56ccc0, thiscall), as
/// rodadas (virtual +0x8c), a árvore dos confrontos (virtual +0x94) e a ordem prevista (virtual +0x5c); uma devolução
/// negativa dá a caixa (linhas 0x161, 0x168, 0x16f) e 0. Sempre, no fim, a atualização da copa.
unsafe extern "thiscall" fn cqn10_copa_da_liga_dia(this: usize, hoje: usize, fase: u32) -> u32 {
    if read_u32_as_usize(this + 0x45) == 0 && read_i16(read_u32_as_usize(0xde1f58) + 0x439) as u16 as i32 == read_i16(this + 0x40) as i32 && fase != 0 {
        cqn10_liberar(this);
        crate::comparacao::ramo("cqn_10 virada da copa da liga");
        core::ptr::write_unaligned((this + 0x45) as *mut u32, 1); // cqn10: a Copa da Liga refeita uma vez
        let inscritos: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_ENGCCCUP_COLLECT_ENTRANTS);
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
    let antes = read_u32_as_usize(this + 0x45);
    let vira = antes == 0 && read_i16(read_u32_as_usize(0xde1f58) + 0x439) as u16 as i32 == read_i16(this + 0x40) as i32 && fase != 0;
    let r = o(this, hoje, fase);
    let depois = read_u32_as_usize(this + 0x45);
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
    if read_u32_as_usize(this + 0x45) == 0 && fase != 0 {
        if cqn10_e_hoje(cqn10_data(1, 9, 0x7cf, 0xffff_ffff), hoje) && *((this + 0xb2) as *const u8) == 0 {
            // ponytail: a pergunta de 1999 monta e entrega uma notícia (só nesse dia, fora de qualquer carreira nova)
            return orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(1791)(this, hoje, fase);
        }
        // ponytail: o ano vai no ax com o resto do eax nos bytes altos; a 0x549f70 o lê como palavra
        if cqn10_e_hoje(cqn10_data(3, 9, read_i16(hoje + 2) as u16 as u32, 0xffff_ffff), hoje) { // cqn10: as inscrições da Copa da Inglaterra em 3/10
            cqn10_liberar(this);
            crate::comparacao::ramo("cqn_10 inscricoes da copa da inglaterra");
            core::ptr::write_unaligned((this + 0x45) as *mut u32, 1);
            let inscritos: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_ENGFACUP_COLLECT_ENTRANTS);
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
            let sortear: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_CUP_SET_UP_ROUND);
            sortear(this, 0);
        }
    }
    cqn10_base(this, hoje, fase)
}

// ponytail: como a da Copa da Liga, só a original; o plano: +0x45 vai a 1 em 3 de outubro (com +0x45 = 0 e a fase), fora
// do caminho de 1999.
unsafe extern "thiscall" fn comparar_cqn10_copa_da_inglaterra_dia(this: usize, hoje: usize, fase: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(1791);
    let antes = read_u32_as_usize(this + 0x45);
    let vira = antes == 0 && fase != 0 && cqn10_e_hoje(cqn10_data(3, 9, read_i16(hoje + 2) as u16 as u32, 0xffff_ffff), hoje)
        && !(cqn10_e_hoje(cqn10_data(1, 9, 0x7cf, 0xffff_ffff), hoje) && *((this + 0xb2) as *const u8) == 0);
    let r = o(this, hoje, fase);
    let depois = read_u32_as_usize(this + 0x45);
    PLACAR.registrar(!vira || depois == 1, || format!("cqn10_copa_da_inglaterra_dia({:#x}) vira {} +0x45 {} -> {}", this, vira, antes, depois));
    r
}

/// { 0x90a900 } (thiscall, ret 8, usa_mls_all_stars.cpp; a virtual +0x14 do Jogo das Estrelas; quadro SEH pela Date
/// local, sem destrutor): `UsaMlsAllStars::update_for_date(hoje, fase)`: com a fase diferente de 0, no dia de Date(26, 6,
/// ano de hoje, dia da semana 6) (o sábado de 26 de julho em diante [inferência]), a convocação das duas seleções
/// (0x90a980, mov ecx -> thiscall, ret 0); depois a atualização da copa.
unsafe extern "thiscall" fn cqn10_jogo_das_estrelas_dia(this: usize, hoje: usize, fase: u32) -> u32 {
    // ponytail: o ano vai num registrador com lixo nos 16 bits altos; a 0x549f70 o lê como palavra
    let ano = read_i16(hoje + 2) as u16 as u32;
    let d = cqn10_data(0x1a, 6, ano, 6); // cqn10: a convocação do Jogo das Estrelas
    if fase != 0 && cqn10_e_hoje(d, hoje) {
        crate::comparacao::ramo("cqn_10 convocacao do jogo das estrelas");
        let convocar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_USAMLSALLSTARS_SELECT_SQUADS);
        convocar(this);
    }
    cqn10_base(this, hoje, fase)
}

// ponytail: a convocação mexe nos elencos e a atualização da copa joga: não rodam duas vezes. Só a original roda.
unsafe extern "thiscall" fn comparar_cqn10_jogo_das_estrelas_dia(this: usize, hoje: usize, fase: u32) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(1792)(this, hoje, fase)
}

