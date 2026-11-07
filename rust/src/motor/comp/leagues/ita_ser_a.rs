use super::*;

// ---------- 279a leva: o calendário das primeiras divisões da Itália e da França e da Série B: as rodadas da temporada, o jogo-desempate italiano e as 34 ou 38 rodadas da francesa (ita_ser_a.cpp, ita_ser_b.cpp, fra_first.cpp) ----------

/// Uma rodada da tabela de 0x41 bytes: a data (dia, mês 0..11, anos a somar ao ano da liga +0x40, dia da semana para
/// onde a data é levada), o código do byte +4 ([inferência] o turno) e as faixas de TV em ordem (dia da semana, turno,
/// quantos jogos; a lista termina em −1).
pub(crate) type Cqo6Rodada = (i8, i8, i8, i8, i8, &'static [(i8, i8, i8)]);

/// As rodadas `primeira`.. da tabela `t`, na ordem da original: para cada uma, a data (0x68a160: push 0; push ano;
/// push código; push semana; push anos; push mês; push dia; push i; push t; add esp -> cdecl: o valor 0) e as
/// faixas (0x68a1c0: push 0; push c; push b; push a; push j; push i; push t; add esp -> cdecl, valor 0).
pub(crate) unsafe fn cqo6_rodadas(t: usize, ano: u32, primeira: i32, tabela: &[Cqo6Rodada]) {
    let data: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, i32, u32, u32) -> usize = core::mem::transmute(FN_LEAGUE_ROUND_SET_DATE);
    let faixa: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, u32) = core::mem::transmute(FN_LEAGUE_ROUND_SET_MATCH_SLOT);
    for (k, &(dia, mes, anos, semana, codigo, faixas)) in tabela.iter().enumerate() {
        let i = primeira + k as i32;
        data(t, i, dia as i32, mes as i32, anos as i32, semana as i32, codigo as i32, ano, 0);
        for (j, &(a, b, c)) in faixas.iter().enumerate() {
            faixa(t, i, j as i32, a as i32, b as i32, c as i32, 0);
        }
    }
}

/// Os bytes que a tabela de `n` rodadas tem gravados e que o jogo lê: de cada rodada, a data (+0..+4) e +0x3d..+0x40,
/// e as faixas até a que termina a lista (a de dia −1 na tabela `guia`, a da original); o resto da memória alocada não
/// é escrito.
pub(crate) unsafe fn cqo6_gravados(t: usize, guia: usize, n: usize) -> Vec<u8> {
    let mut v = Vec::new();
    for r in 0..n {
        let e = t + r * 0x41;
        v.extend_from_slice(core::slice::from_raw_parts(e as *const u8, 5));
        v.extend_from_slice(core::slice::from_raw_parts((e + 0x3d) as *const u8, 4));
        for j in 0..8usize {
            v.extend_from_slice(core::slice::from_raw_parts((e + 5 + j * 7) as *const u8, 7));
            if *((guia + r * 0x41 + 5 + j * 7 + 4) as *const i8) == -1 {
                break;
            }
        }
    }
    v
}

/// O que muda entre as ligas de fase única.
pub(crate) struct Cqo6Liga {
    pub(crate) rodadas: i16,
    pub(crate) arquivo: usize,
    pub(crate) linha: u32,
    pub(crate) tabela: &'static [Cqo6Rodada],
    pub(crate) ramo: &'static str,
}

/// O corpo comum (thiscall(liga, fase, &rodadas, &extra, &é_liga), ret 0x10; sem estado salvo): só a fase principal
/// (−1; as outras devolvem 0 sem gravar nada): é_liga (se pedido) = 1, as rodadas, extra = 0, a tabela de rodadas × 0x41
/// bytes (malloc 0x944e46: push; add esp, 4 -> cdecl; sem memória: a caixa e 0) com as datas e as faixas da liga.
/// Devolve a tabela.
pub(crate) unsafe fn cqo6_calendario(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32, p: &Cqo6Liga) -> usize {
    if fase as u8 != 0xff {
        return 0;
    }
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    if !liga.is_null() {
        *liga = 1;
    }
    *rodadas = p.rodadas;
    *extra = 0;
    let t = alocar(p.rodadas as u32 * 0x41);
    if t == 0 {
        aviso_em(p.arquivo, p.linha);
        return 0;
    }
    crate::comparacao::ramo(p.ramo);
    let ano = core::ptr::read_unaligned((this + 0x40) as *const u16) as u32;
    cqo6_rodadas(t, ano, 0, p.tabela);
    t
}

/// Lado a lado: a original com as saídas de quem chama, a cópia com saídas próprias, a partir do mesmo [0xb67a34]; as
/// saídas e os bytes gravados das duas tabelas têm de ser iguais (a da cópia é liberada: 0x9452ca, cdecl).
pub(crate) unsafe fn cqo6_comparar(nome: &str, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32,
                       original: impl FnOnce() -> usize, copia: impl FnOnce(*mut i16, *mut i16, *mut i32) -> usize) -> usize {
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let (mut r2, mut e2, mut l2) = (0i16, 0i16, 0i32);
    let reg = [(0xb67a34usize, 4usize)];
    let pl2 = if liga.is_null() { core::ptr::null_mut() } else { &mut l2 as *mut i32 };
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, original, || copia(&mut r2, &mut e2, pl2));
    let saidas = fase as u8 != 0xff || (*rodadas == r2 && *extra == e2 && (liga.is_null() || *liga == l2));
    let tabela = if a != 0 && b != 0 {
        let n = *rodadas as usize;
        cqo6_gravados(a, a, n) == cqo6_gravados(b, a, n)
    } else {
        a == 0 && b == 0
    };
    if b != 0 {
        soltar(b);
    }
    PLACAR.registrar(igual && saidas && tabela, || format!("{}(fase {}): saídas {} tabela {}", nome, fase as u8 as i8, saidas, tabela));
    a
}

/// Lado a lado em todas as fases: a original com as saídas de quem chama, a cópia com saídas próprias, a partir do
/// mesmo [0xb67a34]; as saídas e os bytes gravados das duas tabelas têm de ser iguais (na fase −1 as rodadas de liga,
/// `cqo6_gravados`; nas outras as de copa, `cql10_gravados`); a tabela da cópia é solta (0x9452ca, cdecl).
pub(crate) unsafe fn cqo6_comparar_fases(nome: &str, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32,
                             original: impl FnOnce() -> usize, copia: impl FnOnce(*mut i16, *mut i16, *mut i32) -> usize) -> usize {
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let (mut r2, mut e2, mut l2) = (0i16, 0i16, 0i32);
    let reg = [(0xb67a34usize, 4usize)];
    let pl2 = if liga.is_null() { core::ptr::null_mut() } else { &mut l2 as *mut i32 };
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, original, || copia(&mut r2, &mut e2, pl2));
    let gravou = a != 0 || b != 0;
    let saidas = !gravou || (*rodadas == r2 && *extra == e2 && (liga.is_null() || *liga == l2));
    let tabela = if a != 0 && b != 0 {
        if fase as u8 == 0xff {
            let n = *rodadas as usize;
            cqo6_gravados(a, a, n) == cqo6_gravados(b, a, n)
        } else {
            cql10_gravados(a, *rodadas) == cql10_gravados(b, *rodadas)
        }
    } else {
        a == 0 && b == 0
    };
    if b != 0 {
        soltar(b);
    }
    PLACAR.registrar(igual && saidas && tabela, || format!("{}(fase {}): saídas {} tabela {}", nome, fase as u8 as i8, saidas, tabela));
    a
}

/// a Série A, 34 rodadas de 28/ago a 12/mai; quase todas com duas faixas de TV (dias 6 e 5, turno 2, 2 jogos cada)
pub(crate) const CQO6_SERIE_A: [Cqo6Rodada; 34] = [
    (28, 7, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 0: 28/ago
    (9, 8, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 1: 9/set
    (16, 8, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 2: 16/set
    (23, 8, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 3: 23/set
    (30, 8, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 4: 30/set
    (10, 9, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 5: 10/out
    (14, 9, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 6: 14/out
    (21, 9, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 7: 21/out
    (28, 9, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 8: 28/out
    (4, 10, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 9: 4/nov
    (18, 10, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 10: 18/nov
    (25, 10, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 11: 25/nov
    (2, 11, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 12: 2/dez
    (9, 11, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 13: 9/dez
    (16, 11, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 14: 16/dez
    (6, 0, 1, 2, 2, &[(-1, -1, -1)]), // 15: 6/jan (+1 ano)
    (13, 0, 1, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 16: 13/jan (+1 ano)
    (20, 0, 1, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 17: 20/jan (+1 ano)
    (27, 0, 1, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 18: 27/jan (+1 ano)
    (3, 1, 1, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 19: 3/fev (+1 ano)
    (10, 1, 1, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 20: 10/fev (+1 ano)
    (17, 1, 1, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 21: 17/fev (+1 ano)
    (24, 1, 1, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 22: 24/fev (+1 ano)
    (3, 2, 1, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 23: 3/mar (+1 ano)
    (10, 2, 1, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 24: 10/mar (+1 ano)
    (17, 2, 1, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 25: 17/mar (+1 ano)
    (24, 2, 1, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 26: 24/mar (+1 ano)
    (30, 2, 1, 5, 1, &[(-1, -1, -1)]), // 27: 30/mar (+1 ano)
    (7, 3, 1, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 28: 7/abr (+1 ano)
    (14, 3, 1, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 29: 14/abr (+1 ano)
    (21, 3, 1, 6, 1, &[(-1, -1, -1)]), // 30: 21/abr (+1 ano)
    (28, 3, 1, 6, 1, &[(-1, -1, -1)]), // 31: 28/abr (+1 ano)
    (5, 4, 1, 6, 1, &[(-1, -1, -1)]), // 32: 5/mai (+1 ano)
    (12, 4, 1, 6, 1, &[(-1, -1, -1)]), // 33: 12/mai (+1 ano)
];

/// a Série B, 38 rodadas de 29/ago a 14/mai; quase todas com duas faixas de TV (dias 4 e 0, turno 2, 2 jogos cada)
pub(crate) const CQO6_SERIE_B: [Cqo6Rodada; 38] = [
    (29, 7, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 0: 29/ago
    (5, 8, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 1: 5/set
    (12, 8, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 2: 12/set
    (19, 8, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 3: 19/set
    (26, 8, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 4: 26/set
    (3, 9, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 5: 3/out
    (10, 9, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 6: 10/out
    (17, 9, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 7: 17/out
    (24, 9, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 8: 24/out
    (31, 9, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 9: 31/out
    (7, 10, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 10: 7/nov
    (14, 10, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 11: 14/nov
    (21, 10, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 12: 21/nov
    (28, 10, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 13: 28/nov
    (5, 11, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 14: 5/dez
    (12, 11, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 15: 12/dez
    (19, 11, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 16: 19/dez
    (6, 0, 1, 3, 2, &[(-1, -1, -1)]), // 17: 6/jan (+1 ano)
    (9, 0, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 18: 9/jan (+1 ano)
    (16, 0, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 19: 16/jan (+1 ano)
    (19, 0, 1, 6, 1, &[(-1, -1, -1)]), // 20: 19/jan (+1 ano)
    (23, 0, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 21: 23/jan (+1 ano)
    (30, 0, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 22: 30/jan (+1 ano)
    (6, 1, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 23: 6/fev (+1 ano)
    (13, 1, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 24: 13/fev (+1 ano)
    (20, 1, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 25: 20/fev (+1 ano)
    (27, 1, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 26: 27/fev (+1 ano)
    (5, 2, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 27: 5/mar (+1 ano)
    (12, 2, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 28: 12/mar (+1 ano)
    (19, 2, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 29: 19/mar (+1 ano)
    (26, 2, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 30: 26/mar (+1 ano)
    (2, 3, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 31: 2/abr (+1 ano)
    (9, 3, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 32: 9/abr (+1 ano)
    (16, 3, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 33: 16/abr (+1 ano)
    (23, 3, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 34: 23/abr (+1 ano)
    (30, 3, 1, 6, 1, &[(-1, -1, -1)]), // 35: 30/abr (+1 ano)
    (7, 4, 1, 6, 1, &[(-1, -1, -1)]), // 36: 7/mai (+1 ano)
    (14, 4, 1, 6, 1, &[(-1, -1, -1)]), // 37: 14/mai (+1 ano)
];

/// A primeira divisão francesa: as rodadas em ordem, cada uma marcada `true` se só entra com 38 rodadas (20 clubes; com
/// 34, as de `false`, em sequência). O índice no comentário é o da temporada de 38.
pub(crate) const CQO6_PRIMEIRA_FRANCESA: [(bool, Cqo6Rodada); 38] = [
    (false, (31, 6, 0, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)])), // 0: 31/jul
    (false, (6, 7, 0, 4, 2, &[(-1, -1, -1)])), // 1: 6/ago
    (false, (14, 7, 0, 5, 2, &[(4, 1, 1), (4, 2, 1), (-1, -1, -1)])), // 2: 14/ago
    (false, (20, 7, 0, 4, 2, &[(-1, -1, -1)])), // 3: 20/ago
    (false, (28, 7, 0, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)])), // 4: 28/ago
    (false, (10, 8, 0, 4, 2, &[(-1, -1, -1)])), // 5: 10/set
    (false, (18, 8, 0, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)])), // 6: 18/set
    (false, (25, 8, 0, 4, 2, &[(-1, -1, -1)])), // 7: 25/set
    (false, (2, 9, 0, 5, 2, &[(4, 1, 1), (4, 2, 1), (-1, -1, -1)])), // 8: 2/out
    (false, (12, 9, 0, 1, 2, &[(-1, -1, -1)])), // 9: 12/out
    (false, (15, 9, 0, 4, 2, &[(-1, -1, -1)])), // 10: 15/out
    (false, (23, 9, 0, 5, 2, &[(4, 1, 1), (4, 2, 1), (-1, -1, -1)])), // 11: 23/out
    (false, (30, 9, 0, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)])), // 12: 30/out
    (false, (6, 10, 0, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)])), // 13: 6/nov
    (false, (10, 10, 0, 2, 2, &[(1, 2, 1), (1, 2, 1), (-1, -1, -1)])), // 14: 10/nov
    (false, (19, 10, 0, 4, 2, &[(-1, -1, -1)])), // 15: 19/nov
    (false, (27, 10, 0, 5, 2, &[(4, 1, 1), (4, 2, 1), (-1, -1, -1)])), // 16: 27/nov
    (false, (3, 11, 0, 4, 2, &[(-1, -1, -1)])), // 17: 3/dez
    (false, (11, 11, 0, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)])), // 18: 11/dez
    (false, (15, 11, 0, 2, 2, &[(1, 2, 1), (1, 2, 1), (-1, -1, -1)])), // 19: 15/dez
    (false, (18, 11, 0, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)])), // 20: 18/dez
    (true, (8, 0, 1, 5, 2, &[(4, 2, 1), (4, 2, 1), (-1, -1, -1)])), // 21: 8/jan (+1 ano)
    (false, (15, 0, 1, 5, 2, &[(4, 2, 1), (4, 2, 1), (-1, -1, -1)])), // 22: 15/jan (+1 ano)
    (true, (22, 0, 1, 5, 2, &[(4, 2, 1), (4, 2, 1), (-1, -1, -1)])), // 23: 22/jan (+1 ano)
    (false, (26, 0, 1, 2, 2, &[(1, 2, 1), (1, 2, 1), (-1, -1, -1)])), // 24: 26/jan (+1 ano)
    (false, (2, 1, 1, 2, 2, &[(1, 2, 1), (1, 2, 1), (-1, -1, -1)])), // 25: 2/fev (+1 ano)
    (false, (5, 1, 1, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)])), // 26: 5/fev (+1 ano)
    (false, (16, 1, 1, 2, 2, &[(1, 2, 1), (1, 2, 1), (-1, -1, -1)])), // 27: 16/fev (+1 ano)
    (false, (27, 1, 1, 4, 2, &[(-1, -1, -1)])), // 28: 27/fev (+1 ano)
    (true, (4, 2, 1, 5, 2, &[(4, 2, 1), (4, 2, 1), (-1, -1, -1)])), // 29: 4/mar (+1 ano)
    (false, (11, 2, 1, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)])), // 30: 11/mar (+1 ano)
    (true, (18, 2, 1, 5, 2, &[(4, 2, 1), (4, 2, 1), (-1, -1, -1)])), // 31: 18/mar (+1 ano)
    (false, (25, 2, 1, 5, 2, &[(4, 1, 1), (4, 2, 1), (-1, -1, -1)])), // 32: 25/mar (+1 ano)
    (false, (8, 3, 1, 5, 2, &[(4, 1, 1), (4, 2, 1), (-1, -1, -1)])), // 33: 8/abr (+1 ano)
    (false, (15, 3, 1, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)])), // 34: 15/abr (+1 ano)
    (false, (29, 3, 1, 5, 2, &[(6, 2, 1), (6, 2, 1), (-1, -1, -1)])), // 35: 29/abr (+1 ano)
    (false, (13, 4, 1, 5, 2, &[(-1, -1, -1)])), // 36: 13/mai (+1 ano)
    (false, (20, 4, 1, 5, 2, &[(-1, -1, -1)])), // 37: 20/mai (+1 ano)
];

pub(crate) const CQO6_LIGA_SERIE_A: Cqo6Liga = Cqo6Liga { rodadas: 0x22, arquivo: 0x9ea2a8, linha: 0xb0, tabela: &CQO6_SERIE_A, ramo: "cqo_6 calendario da serie a" };
pub(crate) const CQO6_LIGA_SERIE_B: Cqo6Liga = Cqo6Liga { rodadas: 0x26, arquivo: 0x9ea384, linha: 0xa8, tabela: &CQO6_SERIE_B, ramo: "cqo_6 calendario da serie b" };

/// O jogo-desempate das ligas italianas (fase 0, uma rodada de copa de 0x68 bytes, `cql10_rodada`; o ano +0x40 com
/// sinal): o sorteio e o jogo em maio do ano seguinte (a Série A: sorteio em 13/mai, jogo em 19/mai; a Série B: 15/mai e
/// 21/mai), nome 0xa0, sem sorteio (0x200), desempate do confronto 3, categoria 5, ida e volta (+0x21 = 2) com 3 dias
/// entre os jogos, sem prêmios; os times (2n, n jogos, 2n novos) pelo byte +0xee da liga (n, com sinal; as contas em 16
/// bits como a original) [inferência: os confrontos de desempate pedidos].
pub(crate) fn cqo6_desempate(sorteio: (u32, u32, u32, u32), jogo: (u32, u32, u32, u32, u32, u32), n: i8) -> Cql10R {
    let n = n as i16;
    let dois = n.wrapping_shl(1) as u16;
    cql10r(sorteio, jogo, 0xa0, (0, 0x200, 3), 5, (dois, n as u16, dois, 0), (0, 2, 3), (0, 0, 0))
}

/// O que muda entre a Série A e a Série B.
pub(crate) struct Cqo6Italia {
    pub(crate) liga: &'static Cqo6Liga,
    pub(crate) sorteio: (u32, u32, u32, u32),
    pub(crate) jogo: (u32, u32, u32, u32, u32, u32),
    pub(crate) linha_desempate: u32,
}

/// O corpo comum das duas (thiscall(liga, fase, &rodadas, &extra, &é_liga), ret 0x10; sem estado salvo): a fase −1 é a
/// principal (`cqo6_calendario`); a 0, o jogo-desempate: é_liga (se pedido) = 0, 1 rodada, extra 0, a tabela de 0x68
/// bytes (malloc 0x944e46; sem memória: a caixa e 0); as outras devolvem 0 sem gravar nada. Devolve a tabela.
pub(crate) unsafe fn cqo6_italia(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32, p: &Cqo6Italia) -> usize {
    match fase as u8 {
        0xff => cqo6_calendario(this, fase, rodadas, extra, liga, p.liga),
        0 => {
            let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
            if !liga.is_null() {
                *liga = 0;
            }
            *rodadas = 1;
            *extra = 0;
            let t = alocar(0x68);
            if t == 0 {
                aviso_em(p.liga.arquivo, p.linha_desempate);
                return 0;
            }
            crate::comparacao::ramo("cqo_6 jogo-desempate italiano");
            let ano = core::ptr::read_unaligned((this + 0x40) as *const i16) as i32 as u32;
            cql10_rodada(t, 0, &cqo6_desempate(p.sorteio, p.jogo, *((this + 0xee) as *const i8)), ano);
            t
        }
        _ => 0,
    }
}

pub(crate) const CQO6_SERIE_A_E_DESEMPATE: Cqo6Italia = Cqo6Italia { liga: &CQO6_LIGA_SERIE_A, sorteio: (0xd, 4, 1, 0), jogo: (0x13, 4, 1, 6, 1, 0), linha_desempate: 0x226 };
pub(crate) const CQO6_SERIE_B_E_DESEMPATE: Cqo6Italia = Cqo6Italia { liga: &CQO6_LIGA_SERIE_B, sorteio: (0xf, 4, 1, 0), jogo: (0x15, 4, 1, 6, 1, 0), linha_desempate: 0x170 };

/// the Serie A schedule: 34 rounds (18 clubs), from 28 August to 12 May of the next year, all on the same weekday (6)
/// except two (6 January, 30 March); in nearly all, two matches go to each of two TV slots (days 6 and 5, slot 2)
/// [inference: the televised weekend evening matches]. Stage 0, the play-off match (spareggio) [inference: clubs level
/// on points for a place]: two legs 3 days apart, draw on 13 and match on 19 May, ties in fixed order, as many ties as
/// the byte +0xee
/// Original: `ItaSerA::create_schedule` (0x6409b0, ita_ser_a.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x6409b0 } (thiscall, ret 0x10, ita_ser_a.cpp, 2.901 bytes, método +0x3c): `ItaSerA::create_schedule(fase,
/// &rodadas, &extra, &é_liga)`: a Série A, 34 rodadas (`CQO6_SERIE_A`), e o jogo-desempate. Fonte 0x9ea2a8, linhas 0xb0
/// e 0x226.
pub(crate) unsafe extern "thiscall" fn calendario_da_serie_a(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    cqo6_italia(this, fase, rodadas, extra, liga, &CQO6_SERIE_A_E_DESEMPATE)
}

pub(crate) unsafe extern "thiscall" fn comparar_calendario_da_serie_a(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1896);
    cqo6_comparar_fases("calendario_da_serie_a", fase, rodadas, extra, liga, || o(this, fase, rodadas, extra, liga), |r, e, l| calendario_da_serie_a(this, fase, r, e, l))
}

/// the Serie B schedule: 38 rounds (20 clubs), from 29 August to 14 May, on the same weekday (6) except one (6
/// January); in nearly all, two matches in each of two TV slots (days 4 and 0, slot 2). The play-off match as in the
/// Serie A, on 15 and 21 May
/// Original: `ItaSerB::create_schedule` (0x645af0, ita_ser_b.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x645af0 } (thiscall, ret 0x10, ita_ser_b.cpp, 3.215 bytes, método +0x3c): `ItaSerB::create_schedule`: a Série B,
/// 38 rodadas (`CQO6_SERIE_B`), e o jogo-desempate. Fonte 0x9ea384, linhas 0xa8 e 0x170.
pub(crate) unsafe extern "thiscall" fn calendario_da_serie_b(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    cqo6_italia(this, fase, rodadas, extra, liga, &CQO6_SERIE_B_E_DESEMPATE)
}

pub(crate) unsafe extern "thiscall" fn comparar_calendario_da_serie_b(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1897);
    cqo6_comparar_fases("calendario_da_serie_b", fase, rodadas, extra, liga, || o(this, fase, rodadas, extra, liga), |r, e, l| calendario_da_serie_b(this, fase, r, e, l))
}

/// the French first division schedule: in 2001/02, 34 rounds (18 clubs) from 31 July to 20 May; from 2002/03, 38 rounds
/// (20 clubs), with four extra dates in January and March (8 and 22 January, 4 and 18 March); nearly all with two
/// matches in TV slots (days 6, 4 or 1, slots 1 and 2); main stage only
/// Original: `FraFirst::create_schedule` (0x5bb690, fra_first.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x5bb690 } (thiscall, ret 0x10, fra_first.cpp, 3.090 bytes, método +0x3c): `FraFirst::create_schedule(fase,
/// &rodadas, &extra, &é_liga)`: só a fase −1 (as outras devolvem 0). Em 2001 (o ano +0x40 = 0x7d1) 34 rodadas (18
/// clubes); nos outros anos 38 (20 clubes). é_liga (se pedido) = 1, as rodadas, extra 0, a tabela de rodadas × 0x41
/// bytes (sem memória: a caixa da linha 0xd1 e 0); as rodadas de `CQO6_PRIMEIRA_FRANCESA` em sequência (as marcadas só
/// entram com 38); no fim, se a conta não bate com as rodadas, a caixa da linha 0x1b2 (e devolve a tabela assim mesmo).
/// Fonte 0x9ccb5c.
pub(crate) unsafe extern "thiscall" fn calendario_da_primeira_francesa(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    if fase as u8 != 0xff {
        return 0;
    }
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let ano = core::ptr::read_unaligned((this + 0x40) as *const u16);
    let n: i16 = if ano == 0x7d1 { 0x22 } else { 0x26 }; // cqo6: 34 rodadas em 2001, 38 depois
    if !liga.is_null() {
        *liga = 1;
    }
    *rodadas = n;
    *extra = 0;
    let t = alocar((n as i32 * 0x41) as u32);
    if t == 0 {
        aviso_em(0x9ccb5c, 0xd1);
        return 0;
    }
    crate::comparacao::ramo("cqo_6 calendario da primeira francesa");
    let mut i: i16 = 0;
    for (so_38, r) in CQO6_PRIMEIRA_FRANCESA.iter() {
        if *so_38 && n != 0x26 {
            continue;
        }
        cqo6_rodadas(t, ano as u32, i as i32, core::slice::from_ref(r));
        i += 1;
    }
    if i != n {
        aviso_em(0x9ccb5c, 0x1b2);
    }
    t
}

pub(crate) unsafe extern "thiscall" fn comparar_calendario_da_primeira_francesa(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1898);
    cqo6_comparar("calendario_da_primeira_francesa", fase, rodadas, extra, liga, || o(this, fase, rodadas, extra, liga), |r, e, l| calendario_da_primeira_francesa(this, fase, r, e, l))
}

// ---------- 278a leva: o desempate da Série A e da Série B italianas no fim da temporada (o "spareggio": empate em pontos na fronteira de duas faixas), a ordem dos empatados pelo confronto direto e o resultado do jogo de desempate (ita_ser_a.cpp, ita_ser_b.cpp) ----------





/// A linha `i` da tabela [+0xb1] (0x3b bytes).
pub(crate) unsafe fn cql4_linha(liga: usize, i: usize) -> usize {
    read_u32_as_usize(liga + 0xb1) + i * 0x3b
}

/// As faixas da Série A, por posição (0 = o primeiro): 1 o campeão; 2 as outras vagas da Liga dos Campeões (as `cl`
/// vagas da nação, UefaSeeding::get_nation_places 0x905c60: push &uefa; push &cl; push [[liga+4]+0x5d] (a nação da
/// competição); mov ecx, [0xde1f58] -> thiscall, ret 0xc, que grava 1 nas duas antes de procurar); 3 as `uefa` vagas da
/// Copa da UEFA, logo depois; 4 as duas seguintes [inferência: a Intertoto]; 5 as `+0xc1` últimas de 18 (o
/// rebaixamento); 0 o resto. A original monta as 18 faixas num registro local (16 + 2 bytes) e lê duas além com uma
/// tabela de mais de 18 clubes; a cópia usa 64 bytes com zero depois das 18 (a Série A tem 18).
pub(crate) unsafe fn cql4_faixas_da_serie_a(liga: usize) -> [u8; 64] {
    let vagas: unsafe extern "thiscall" fn(usize, usize, *mut i8, *mut i8) = core::mem::transmute(FN_UEFASEEDING_GET_NATION_PLACES);
    let mut z = [0u8; 64];
    z[0] = 1;
    let (mut cl, mut uefa) = (0i8, 0i8);
    vagas(read_u32_as_usize(0xde1f58), read_u32_as_usize(read_u32_as_usize(liga + 4) + 0x5d), &mut cl, &mut uefa);
    // ponytail: as contas da nação ficam de 1 a 16 no jogo; a cópia ignora as faixas que cairiam fora do registro.
    let cl = cl as i32;
    let uefa = uefa as i32;
    if cl > 1 {
        for k in 1..cl {
            if (k as usize) < 64 {
                z[k as usize] = 2;
            }
        }
    }
    if uefa > 0 {
        for k in 0..uefa {
            if ((cl + k) as usize) < 64 {
                z[(cl + k) as usize] = 3;
            }
        }
    }
    let i = uefa + cl;
    if i >= 0 && (i as usize) + 1 < 64 {
        z[i as usize] = 4; // cql4: as duas posições depois das vagas europeias
        z[i as usize + 1] = 4;
    }
    let rebaixados = *((liga + 0xc1) as *const i8) as i32;
    if rebaixados > 0 {
        for k in 0..rebaixados {
            if 17 - k >= 0 {
                z[(17 - k) as usize] = 5;
            }
        }
    }
    z
}

/// As faixas da Série B: 1 o campeão; 2 as 3 vagas seguintes de acesso; 3 as 4 últimas de 20 (o rebaixamento); 0 o
/// resto (registro local de 20 bytes na original).
pub(crate) fn cql4_faixas_da_serie_b() -> [u8; 64] {
    let mut z = [0u8; 64];
    z[0] = 1;
    z[1] = 2;
    z[2] = 2;
    z[3] = 2; // cql4: as vagas de acesso da Série B
    for k in 16..20 {
        z[k] = 3;
    }
    z
}

/// Há duas linhas vizinhas com os mesmos pontos (+0x19) em faixas diferentes? (a conferência do comparador)
pub(crate) unsafe fn cql4_empate_na_fronteira(liga: usize, z: &[u8; 64]) -> bool {
    let n = read_i16(liga + 0x3e) as i32 - 1;
    (0..n.max(0) as usize).any(|i| {
        let l = cql4_linha(liga, i);
        read_i16(l + 0x19) == read_i16(l + 0x3b + 0x19) && z[i.min(63)] != z[(i + 1).min(63)]
    })
}

/// the Serie A day; at season end, two clubs level on points in different bands (the title; Italy's Champions League
/// places; the UEFA Cup places; the next two [inference: the Intertoto]; the relegated, the last +0xc1 of 18) are not
/// split by goal difference: the first such pair from the top is ordered by head-to-head and goes to an extra match
/// (the next stage); with no tie on a boundary, the season closes
/// Original: `ItaSerA::update_for_date` (0x642880, ita_ser_a.cpp, thiscall, ret 8).
///
/// [pt] { 0x642880 } (thiscall, ret 8, ita_ser_a.cpp, slot +0x14): `ItaSerA::update_for_date(hoje, completo)`. Primeiro o
/// dia de liga (League::update_for_date 0x6847c0: push completo; push hoje; mov ecx, this -> thiscall, ret 8), cuja
/// devolução ela repassa. Depois, no fim da temporada (+0xdd), na fase principal (+0x2c = 0, sem fases criadas), com
/// `completo` e com +0xe6 [inferência: a tabela fechada]: zera +0x4c e procura, de cima para baixo, o primeiro par de
/// clubes vizinhos com os mesmos pontos em faixas diferentes (cql4_faixas_da_serie_a). Achando: ordena os empatados
/// pelo confronto direto (0x6471a0: push pontos; mov ecx, this -> thiscall, ret 4), cria o vetor de 1 fase (+0x2c =
/// 1, +0xc = malloc 0x944e46 de 4: push 4; add esp, 4 -> cdecl) e chama a virtual +0x28 (a fase seguinte: o jogo de
/// desempate). Sem empate na fronteira: +0xdd = 0, +0xe6 = 0 e a virtual +0x94 (League::update_positions_and_status).
/// A regra: na Itália, empate em pontos entre clubes de faixas diferentes (título, Liga dos Campeões, Copa da UEFA,
/// Intertoto [inferência], rebaixamento) não se decide pelo saldo: vai a um jogo extra.
pub(crate) unsafe extern "thiscall" fn desempate_da_serie_a(this: usize, hoje: usize, completo: u32) -> u32 {
    let dia: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_LEAGUE_UPDATE_FOR_DATE);
    let ordenar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_ITA_LEAGUE_SORT_CLUBS_LEVEL_ON_POINTS);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let r = dia(this, hoje, completo);
    // ponytail: a original devolve o byte do dia de liga com os bits de cima do eax que sobram da última chamada; o
    // laço das competições só lê o byte.
    if read_u32_as_usize(this + 0xdd) == 0 || read_u32_as_usize(this + 0x2c) != 0 || completo == 0 || read_u32_as_usize(this + 0xe6) == 0 {
        return r;
    }
    let z = cql4_faixas_da_serie_a(this);
    let n = read_i16(this + 0x3e) as i32 - 1;
    write_u32_from_usize(this + 0x4c, 0);
    crate::comparacao::ramo("cql_4 fim da serie a");
    let mut i: i16 = 0;
    if n > 0 {
        loop {
            let l = cql4_linha(this, i as usize);
            let pontos = read_i16(l + 0x19);
            if pontos == read_i16(l + 0x54) && z[(i as usize).min(63)] != z[(i as usize + 1).min(63)] { // cql4: o par da Série A na fronteira
                ordenar(this, pontos as u16 as u32);
                write_u32_from_usize(this + 0x2c, 1);
                let v = alocar(4);
                write_u32_from_usize(this + 0xc, v);
                let proxima: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(this) + 0x28));
                proxima(this);
                return r;
            }
            i = i.wrapping_add(1);
            if i as i32 >= n {
                break;
            }
        }
    }
    write_u32_from_usize(this + 0xdd, 0);
    write_u32_from_usize(this + 0xe6, 0);
    let situacao: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(this) + 0x94));
    situacao(this);
    r
}

/// the Serie B day; the fixed bands are the champion, 3 more promotion places (4 go up) and the 4 relegated of 20; each
/// tied pair on a boundary is ordered by head-to-head and, if any, the tie-break match
/// Original: `ItaSerB::update_for_date` (0x647050, ita_ser_b.cpp, thiscall, ret 8).
///
/// [pt] { 0x647050 } (thiscall, ret 8, ita_ser_b.cpp, slot +0x14): `ItaSerB::update_for_date(hoje, completo)`. Como a da
/// Série A, com as faixas fixas da Série B (o campeão, mais 3 de acesso, os 4 rebaixados de 20) e sem parar no
/// primeiro par: cada par vizinho empatado na fronteira é ordenado pelo confronto direto (0x6471a0: push edi, os
/// pontos na palavra de baixo e o resto do `completo` nos bits de cima, que a 0x6471a0 não lê; mov ecx -> thiscall,
/// ret 4); havendo algum, a fase do desempate (+0x2c = 1, +0xc = malloc de 4, virtual +0x28). Sem nenhum: +0xdd = 0,
/// +0xe6 = 0 e a virtual +0x94.
pub(crate) unsafe extern "thiscall" fn desempate_da_serie_b(this: usize, hoje: usize, completo: u32) -> u32 {
    let dia: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_LEAGUE_UPDATE_FOR_DATE);
    let ordenar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_ITA_LEAGUE_SORT_CLUBS_LEVEL_ON_POINTS);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let r = dia(this, hoje, completo);
    if read_u32_as_usize(this + 0xdd) == 0 || read_u32_as_usize(this + 0x2c) != 0 || completo == 0 || read_u32_as_usize(this + 0xe6) == 0 {
        return r;
    }
    let z = cql4_faixas_da_serie_b();
    write_u32_from_usize(this + 0x4c, 0);
    crate::comparacao::ramo("cql_4 fim da serie b");
    let mut achou = false;
    let mut i: i16 = 0;
    while (i as i32) < read_i16(this + 0x3e) as i32 - 1 {
        let l = cql4_linha(this, i as usize);
        let pontos = read_i16(l + 0x19);
        if pontos == read_i16(l + 0x54) && z[(i as usize).min(63)] != z[(i as usize + 1).min(63)] { // cql4: o par da Série B na fronteira
            achou = true;
            ordenar(this, pontos as u16 as u32);
        }
        i = i.wrapping_add(1);
    }
    if achou {
        write_u32_from_usize(this + 0x2c, 1);
        let v = alocar(4);
        write_u32_from_usize(this + 0xc, v);
        let proxima: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(this) + 0x28));
        proxima(this);
        return r;
    }
    write_u32_from_usize(this + 0xdd, 0);
    write_u32_from_usize(this + 0xe6, 0);
    let situacao: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(this) + 0x94));
    situacao(this);
    r
}

/// O comparador das duas: o dia de liga e a fase do desempate não rodam duas vezes: só a original, conferindo o que
/// ela garante. Na fase principal e com `completo`: se a fase do desempate foi criada (+0x2c mudou), há na tabela um
/// par vizinho empatado em faixas diferentes; e a original nunca sai com o fim da temporada pendente (+0xdd e +0xe6
/// ligados e +0x2c = 0), porque ou cria a fase ou os zera.
pub(crate) unsafe fn cql4_comparar(nome: &str, this: usize, completo: u32, faixas: impl Fn() -> [u8; 64], original: impl FnOnce() -> u32) -> u32 {
    let fases = read_u32_as_usize(this + 0x2c);
    let r = original();
    if completo != 0 && fases == 0 {
        if read_u32_as_usize(this + 0x2c) != 0 {
            let z = faixas();
            PLACAR.registrar(cql4_empate_na_fronteira(this, &z) && read_u32_as_usize(this + 0xc) != 0, || format!("{}({:#x}): fase do desempate sem empate na fronteira (+0xc = {:#x})", nome, this, read_u32_as_usize(this + 0xc)));
        } else {
            PLACAR.registrar(!(read_u32_as_usize(this + 0xdd) != 0 && read_u32_as_usize(this + 0xe6) != 0), || format!("{}({:#x}): o fim da temporada ficou pendente (+0xdd e +0xe6 ligados)", nome, this));
        }
    }
    r
}

pub(crate) unsafe extern "thiscall" fn comparar_desempate_da_serie_a(this: usize, hoje: usize, completo: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(1852);
    cql4_comparar("desempate_da_serie_a", this, completo, || cql4_faixas_da_serie_a(this), || o(this, hoje, completo))
}

pub(crate) unsafe extern "thiscall" fn comparar_desempate_da_serie_b(this: usize, hoje: usize, completo: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(1853);
    cql4_comparar("desempate_da_serie_b", this, completo, cql4_faixas_da_serie_b, || o(this, hoje, completo))
}

/// clubs level on points are ordered by the head-to-head among them (the mini-table 0x682b30) and the table positions
/// are renumbered
/// Original: `ita_league_sort_clubs_level_on_points` (0x6471a0, ita_ser_b.cpp, thiscall, ret 4).
///
/// [pt] { 0x6471a0 } (thiscall, ret 4, ita_ser_b.cpp; chamada também pela da Série A): `ita_league_sort_clubs_level_on_points
/// (pontos)`. Monta o confronto direto em +4 de cada linha (League::build_h2h_table 0x682b30: push [+0xb1]; mov ecx,
/// this -> thiscall, ret 4), acha o bloco das linhas com esses pontos (a primeira e quantas; a tabela já está ordenada
/// pelos pontos), ordena o bloco pela comparação do confronto (qsort 0x945333 com 0x683410, linhas de 0x3b: push;
/// add esp, 0x10 -> cdecl; sem linha, o qsort recebe 0 e 0) e renumera a posição (+4) de todas as linhas.
pub(crate) unsafe extern "thiscall" fn ordem_dos_empatados_na_italia(this: usize, pontos: u32) {
    let confronto: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_LEAGUE_RANK_BY_HEAD_TO_HEAD);
    let qsort: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    confronto(this, read_u32_as_usize(this + 0xb1));
    let n = read_i16(this + 0x3e);
    let mut primeira = 0usize;
    let mut quantas: u32 = 0;
    if n > 0 {
        let mut l = read_u32_as_usize(this + 0xb1);
        for _ in 0..n as i32 {
            if read_i16(l + 0x19) == pontos as u16 as i16 {
                if primeira == 0 {
                    primeira = l;
                }
                quantas = quantas.wrapping_add(1);
            }
            l += 0x3b;
        }
    }
    qsort(primeira, quantas as u16 as i16 as i32, 0x3b, 0x683410); // cql4: o bloco dos empatados
    let mut i: i16 = 0;
    while i < read_i16(this + 0x3e) {
        *((cql4_linha(this, i as usize) + 4) as *mut u8) = i as u8;
        i += 1;
    }
}

/// Lado a lado na tabela inteira (a 0x682b30 grava +4 de cada linha e solta a sua minitabela; o qsort só mexe no
/// bloco).
pub(crate) unsafe extern "thiscall" fn comparar_ordem_dos_empatados_na_italia(this: usize, pontos: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32)>(1854);
    let t = read_u32_as_usize(this + 0xb1);
    let n = read_i16(this + 0x3e).max(0) as usize;
    if t == 0 || n == 0 {
        return o(this, pontos);
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(t, n * 0x3b)], || o(this, pontos), || ordem_dos_empatados_na_italia(this, pontos));
    PLACAR.registrar(igual, || format!("ordem_dos_empatados_na_italia({:#x}, {}) original={:x?} rust={:x?}", this, pontos as u16 as i16, mo, mr));
}

/// in the league, the base records; in the tie-break match, the winner (status 1 [inference]) is placed above the other
/// in the table (rows swap if it was below) and, with no play-off pending, the season closes
/// Original: `ItaSerA::update_staff_records_for_status` (0x641dc0, ita_ser_a.cpp, thiscall, ret 0x18).
///
/// [pt] { 0x641dc0 } (thiscall, ret 0x18, ita_ser_a.cpp, slot +0x44): `ItaSerA::update_staff_records_for_status(clube,
/// situação, fase, a4, jogo, a6)`. Na liga (fase 0xff): a da base (League::update_staff_records_for_status 0x686940:
/// push a6; push jogo; push a4; push -1; push situação; push clube; mov ecx, this -> thiscall, ret 0x18), cuja devolução
/// repassa. No jogo de desempate (outra fase), só com a situação 1 [inferência: o clube venceu]: +0xad = 0, ordena a
/// tabela (League::sort_table 0x6827d0: push 0; mov ecx -> thiscall, ret 4) e acha a linha do clube: se o de cima é
/// um dos dois do jogo (+0x1c, +0x20), as duas linhas trocam de lugar; senão, se o de baixo é um deles, ficam. Nos dois
/// casos as duas linhas ganham a posição (+4) e o destino em aberto (+0x37 = 0xff). Depois, se nenhuma linha tem o
/// destino 1 (o play-off de cima), fecha a temporada: +0xe6 = 0, +0xdd = 0, a virtual +0x94, a virtual +0x10 (0, 0) e
/// +0xdd = 1. Devolve −1. A regra: quem vence o jogo de desempate fica à frente do outro.
pub(crate) unsafe extern "thiscall" fn resultado_do_desempate_da_serie_a(this: usize, clube: usize, situacao: u32, fase: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    if fase as u8 == 0xff {
        let base: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32 = core::mem::transmute(FN_LEAGUE_UPDATE_STAFF_RECORDS_FOR_STATUS);
        return base(this, clube, situacao, 0xffff_ffff, a4, jogo, a6);
    }
    if situacao as u8 != 1 {
        return 0xffff_ffff;
    }
    crate::comparacao::ramo("cql_4 resultado do desempate");
    let ordenar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_LEAGUE_SORT_TABLE);
    write_u32_from_usize(this + 0xad, 0);
    ordenar(this, 0);
    let t = read_u32_as_usize(this + 0xb1);
    let (a, b) = (read_u32_as_usize(jogo + 0x1c), read_u32_as_usize(jogo + 0x20));
    let mut i: i16 = 0;
    while i < read_i16(this + 0x3e) {
        let l = t + i as usize * 0x3b;
        if read_u32_as_usize(l) == clube {
            if i > 0 {
                let acima = read_u32_as_usize(l - 0x3b);
                if acima == a || acima == b {
                    // o clube estava abaixo do outro: trocam
                    let mut tmp = [0u8; 0x3b];
                    core::ptr::copy_nonoverlapping(l as *const u8, tmp.as_mut_ptr(), 0x3b);
                    core::ptr::copy_nonoverlapping((l - 0x3b) as *const u8, l as *mut u8, 0x3b);
                    let l2 = read_u32_as_usize(this + 0xb1) + i as usize * 0x3b;
                    core::ptr::copy_nonoverlapping(tmp.as_ptr(), (l2 - 0x3b) as *mut u8, 0x3b);
                    let l3 = read_u32_as_usize(this + 0xb1) + i as usize * 0x3b;
                    *((l3 + 4) as *mut u8) = i as u8;
                    *((l3 - 0x3b + 4) as *mut u8) = (i as u8).wrapping_sub(1);
                    *((l3 + 0x37) as *mut u8) = 0xff;
                    *((l3 - 0x3b + 0x37) as *mut u8) = 0xff;
                    break;
                }
            }
            if i < read_i16(this + 0x3e) {
                // ponytail: a original lê o clube da linha seguinte mesmo na última (4 bytes além da tabela), como aqui.
                let abaixo = read_u32_as_usize(l + 0x3b);
                if abaixo == a || abaixo == b {
                    *((t + i as usize * 0x3b + 4) as *mut u8) = i as u8;
                    let t2 = read_u32_as_usize(this + 0xb1);
                    *((t2 + i as usize * 0x3b + 0x3f) as *mut u8) = (i as u8).wrapping_add(1);
                    *((read_u32_as_usize(this + 0xb1) + i as usize * 0x3b + 0x37) as *mut u8) = 0xff;
                    *((read_u32_as_usize(this + 0xb1) + i as usize * 0x3b + 0x72) as *mut u8) = 0xff;
                    break;
                }
            }
        }
        i += 1;
    }
    let n = read_i16(this + 0x3e);
    let mut k: i16 = 0;
    while k < n {
        if read_u8(cql4_linha(this, k as usize) + 0x37) == 1 {
            return 0xffff_ffff;
        }
        k += 1;
    }
    write_u32_from_usize(this + 0xe6, 0);
    write_u32_from_usize(this + 0xdd, 0);
    let situacao_f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(this) + 0x94));
    situacao_f(this);
    let fim: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(this) + 0x10));
    fim(this, 0, 0);
    write_u32_from_usize(this + 0xdd, 1);
    0xffff_ffff
}

/// Na liga e no desempate com a situação 1 há chamadas que não rodam duas vezes (os registros da base, a ordenação, as
/// virtuais): só a original. Nas outras situações a cópia só devolve −1: compara a devolução.
pub(crate) unsafe extern "thiscall" fn comparar_resultado_do_desempate_da_serie_a(this: usize, clube: usize, situacao: u32, fase: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(1855);
    let r = o(this, clube, situacao, fase, a4, jogo, a6);
    if fase as u8 != 0xff && situacao as u8 != 1 {
        PLACAR.registrar(r == 0xffff_ffff, || format!("resultado_do_desempate_da_serie_a: devolveu {:#x}", r));
    }
    r
}

// ---------- 278a leva: os construtores da Série A, da Série B e dos dois grupos da Série C1 italianas (as regras, a tabela, o calendário real de 2001/02, os jogos, as estatísticas e as posições previstas na criação) (ita_ser_a.cpp, ita_ser_b.cpp, ita_ser_c1a.cpp, ita_ser_c1b.cpp) ----------



/// O que muda de uma divisão italiana para outra no construtor.
pub(crate) struct Cqm4Divisao {
    pub(crate) vtabela: usize,
    /// a Série A e a Série B gravam +0xee = 0 (o campo próprio da classe [inferência: o desempate em curso]) e +0x4c = 1
    pub(crate) serie: bool,
    /// fases além da liga (+0x2c): 0 na A e na B; 2 na C1 (o play-off e o play-out; o vetor [+0xc] = malloc de 8)
    pub(crate) fases: u32,
    pub(crate) regras: usize,
    pub(crate) tabela: usize,
    /// o calendário real de 2001/02 pelo nome dos clubes (set_up_fixed_fixtures; mov ecx, this -> thiscall, ret 0)
    pub(crate) fixos: usize,
    pub(crate) arquivo: usize,
    /// as caixas: regras, tabela, jogos, ordem, estatísticas
    pub(crate) linhas: [u32; 5],
}

/// O corpo dos quatro construtores (thiscall(this, ano, registro), ret 8, quadro SEH só para o operator new): a base
/// (League::League 0x682200: mov ecx -> thiscall, ret 0), o registro (+4), a vtable, o ano (+0x40) e a regra de
/// disciplina 0x11 (+0x50, a das ligas italianas); tenta carregar o estado (League::load_from_temp_file 0x687b10: push
/// 1; mov ecx -> thiscall, ret 4). Carregou: pronto. Senão, a liga nova: na A e na B, +0xee = 0 e +0x4c = 1; fase 0xff
/// (+0x44), fase atual −1 (+0x30), +0x2c fases e o vetor [+0xc] (malloc 0x944e46 de 4 × fases: push; add esp, 4 ->
/// cdecl; 0 sem fases); as regras e a tabela (caixa e fim se falharem); o calendário real de 2001/02 (sem conferir);
/// os jogos (League::create_fixtures 0x6835c0), a ordem (League::sort_table 0x6827d0: push 0 -> thiscall, ret 4), as
/// estatísticas (operator new 0x944cf1 de 0x5ce: push; add esp, 4 -> cdecl; Comp_Stats::Comp_Stats 0x49ee70: push
/// liga; mov ecx, novo -> thiscall, ret 4) em +8 e as posições previstas (0x68a850: mov ecx -> thiscall, ret 0).
/// Devolve this.
pub(crate) unsafe fn cqm4_construir(this: usize, ano: u32, registro: usize, p: &Cqm4Divisao) -> usize {
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LEAGUE_LEAGUE);
    let carregar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_LEAGUE_LOAD_FROM_TEMP_FILE);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(p.regras);
    let tabela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(p.tabela);
    let fixos: unsafe extern "thiscall" fn(usize) = core::mem::transmute(p.fixos);
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_LEAGUE_CREATE_FIXTURES);
    let ordenar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_LEAGUE_SORT_TABLE);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let estatisticas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_COMP_STATS);
    let previstas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LEAGUE_SET_PREDICTED_POSITIONS);
    base(this);
    write_u32_from_usize(this + 4, registro);
    write_u32_from_usize(this, p.vtabela);
    core::ptr::write_unaligned((this + 0x40) as *mut u16, ano as u16);
    *((this + 0x50) as *mut u8) = 0x11; // cqm4: a regra de disciplina das ligas italianas
    if carregar(this, 1) != 0 {
        return this;
    }
    crate::comparacao::ramo("cqm_4 divisao italiana nova");
    if p.serie {
        *((this + 0xee) as *mut u8) = 0;
        write_u32_from_usize(this + 0x4c, 1); // cqm4: a marca do histórico da A e da B
    }
    *((this + 0x44) as *mut u8) = 0xff;
    write_u32_from_usize(this + 0x30, 0xffff_ffff);
    write_u32_from_usize(this + 0x2c, p.fases as usize);
    if p.fases != 0 {
        let v = alocar(p.fases * 4);
        write_u32_from_usize(this + 0xc, v);
    } else {
        write_u32_from_usize(this + 0xc, 0);
    }
    if regras(this) == 0 {
        aviso_em(p.arquivo, p.linhas[0]);
        return this;
    }
    if tabela(this) == 0 {
        aviso_em(p.arquivo, p.linhas[1]);
        return this;
    }
    fixos(this); // cqm4: o calendário real de 2001/02
    if jogos(this) == 0 {
        aviso_em(p.arquivo, p.linhas[2]);
        return this;
    }
    if ordenar(this, 0) == 0 {
        aviso_em(p.arquivo, p.linhas[3]);
        return this;
    }
    let e = novo(0x5ce);
    let s = if e != 0 { estatisticas(e, this) } else { 0 };
    write_u32_from_usize(this + 8, s);
    if s == 0 {
        aviso_em(p.arquivo, p.linhas[4]);
        return this;
    }
    previstas(this); // cqm4: as posições previstas
    this
}

pub(crate) const CQM4_SERIE_A: Cqm4Divisao = Cqm4Divisao {
    vtabela: 0x96c3cc,
    serie: true,
    fases: 0,
    regras: 0x641640,
    tabela: 0x641510,
    fixos: 0x642a70,
    arquivo: 0x9ea2a8,
    linhas: [0x53, 0x5a, 0x64, 0x6d, 0x74],
};
pub(crate) const CQM4_SERIE_B: Cqm4Divisao = Cqm4Divisao {
    vtabela: 0x96c480,
    serie: true,
    fases: 0,
    regras: 0x6468a0,
    tabela: 0x646780,
    fixos: 0x647230,
    arquivo: 0x9ea384,
    linhas: [0x4b, 0x52, 0x5c, 0x65, 0x6c],
};
pub(crate) const CQM4_C1A: Cqm4Divisao = Cqm4Divisao {
    vtabela: 0x96c534,
    serie: false,
    fases: 2, // cqm4: o play-off e o play-out da C1/A
    regras: 0x64b770,
    tabela: 0x64b670,
    fixos: 0x64be90,
    arquivo: 0x9ea474,
    linhas: [0x49, 0x50, 0x5a, 0x63, 0x6a],
};
pub(crate) const CQM4_C1B: Cqm4Divisao = Cqm4Divisao {
    vtabela: 0x96c5e8,
    serie: false,
    fases: 2,
    regras: 0x64f920,
    tabela: 0x64f820,
    fixos: 0x650100,
    arquivo: 0x9ea558,
    linhas: [0x4a, 0x51, 0x5b, 0x64, 0x6b],
};

/// Serie A: discipline rule 0x11 (the Italian leagues one), only the league (the tie-break is born at season end); with
/// no saved state, the rules, the table, the real 2001/02 fixture list by club name, the fixtures, the order, the stats
/// and the predicted positions
/// Original: `ItaSerA::ItaSerA` (0x6406d0, ita_ser_a.cpp, thiscall, ret 8).
///
/// [pt] { 0x6406d0 } (thiscall, ret 8, ita_ser_a.cpp): `ItaSerA::ItaSerA(ano, registro)`: a Série A, só a liga (o jogo de
/// desempate nasce no fim, pela 0x642880).
pub(crate) unsafe extern "thiscall" fn construir_serie_a(this: usize, ano: u32, registro: usize) -> usize {
    cqm4_construir(this, ano, registro, &CQM4_SERIE_A)
}

/// Serie B: discipline 0x11, only the league, the real 2001/02 fixture list
/// Original: `ItaSerB::ItaSerB` (0x645810, ita_ser_b.cpp, thiscall, ret 8).
///
/// [pt] { 0x645810 } (thiscall, ret 8, ita_ser_b.cpp): `ItaSerB::ItaSerB(ano, registro)`: a Série B, só a liga.
pub(crate) unsafe extern "thiscall" fn construir_serie_b(this: usize, ano: u32, registro: usize) -> usize {
    cqm4_construir(this, ano, registro, &CQM4_SERIE_B)
}

/// C1/A: discipline 0x11, the league and 2 stages (the promotion play-off and the play-out), the real 2001/02 fixture
/// list
/// Original: `ItaSerC1a::ItaSerC1a` (0x64aa70, ita_ser_c1a.cpp, thiscall, ret 8).
///
/// [pt] { 0x64aa70 } (thiscall, ret 8, ita_ser_c1a.cpp): `ItaSerC1a::ItaSerC1a(ano, registro)`: a C1/A, a liga e 2 fases
/// (o play-off de acesso e o play-out).
pub(crate) unsafe extern "thiscall" fn construir_c1a(this: usize, ano: u32, registro: usize) -> usize {
    cqm4_construir(this, ano, registro, &CQM4_C1A)
}

/// C1/B, as C1/A
/// Original: `ItaSerC1b::ItaSerC1b` (0x64ec20, ita_ser_c1b.cpp, thiscall, ret 8).
///
/// [pt] { 0x64ec20 } (thiscall, ret 8, ita_ser_c1b.cpp): `ItaSerC1b::ItaSerC1b(ano, registro)`: a C1/B, como a C1/A.
pub(crate) unsafe extern "thiscall" fn construir_c1b(this: usize, ano: u32, registro: usize) -> usize {
    cqm4_construir(this, ano, registro, &CQM4_C1B)
}

/// O comparador dos quatro: alocam: só a original, conferindo o que ela grava: a devolução, a vtable, o ano, o
/// registro e a disciplina; na liga nova (+0x44 = 0xff, com as estatísticas), +0x2c, o vetor das fases e, na A e na
/// B, +0x4c = 1 e +0xee = 0.
pub(crate) unsafe fn cqm4_comparar(nome: &str, this: usize, ano: u32, registro: usize, p: &Cqm4Divisao, original: impl FnOnce() -> usize) -> usize {
    let r = original();
    let mut ok = r == this && read_u32_as_usize(this) == p.vtabela && core::ptr::read_unaligned((this + 0x40) as *const u16) == ano as u16
        && read_u32_as_usize(this + 4) == registro && *((this + 0x50) as *const u8) == 0x11;
    if *((this + 0x44) as *const u8) == 0xff && read_u32_as_usize(this + 8) != 0 {
        ok = ok && read_u32_as_usize(this + 0x2c) == p.fases as usize && (read_u32_as_usize(this + 0xc) != 0) == (p.fases != 0);
        if p.serie {
            ok = ok && *((this + 0xee) as *const u8) == 0;
        }
    }
    PLACAR.registrar(ok, || format!("{}({:#x}): devolveu {:#x}, vtable {:#x}, +0x2c = {}, +0xc = {:#x}, +8 = {:#x}", nome, this, r, read_u32_as_usize(this), read_u32_as_usize(this + 0x2c), read_u32_as_usize(this + 0xc), read_u32_as_usize(this + 8)));
    r
}

pub(crate) unsafe extern "thiscall" fn comparar_construir_serie_a(this: usize, ano: u32, registro: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(1865);
    cqm4_comparar("construir_serie_a", this, ano, registro, &CQM4_SERIE_A, || o(this, ano, registro))
}

pub(crate) unsafe extern "thiscall" fn comparar_construir_serie_b(this: usize, ano: u32, registro: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(1866);
    cqm4_comparar("construir_serie_b", this, ano, registro, &CQM4_SERIE_B, || o(this, ano, registro))
}

pub(crate) unsafe extern "thiscall" fn comparar_construir_c1a(this: usize, ano: u32, registro: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(1867);
    cqm4_comparar("construir_c1a", this, ano, registro, &CQM4_C1A, || o(this, ano, registro))
}

pub(crate) unsafe extern "thiscall" fn comparar_construir_c1b(this: usize, ano: u32, registro: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(1868);
    cqm4_comparar("construir_c1b", this, ano, registro, &CQM4_C1B, || o(this, ano, registro))
}

// ---------- 278a leva: o calendário real de 2001/02 da Série A e da Série B italianas (os jogos fixos pelo nome dos clubes) (ita_ser_a.cpp, ita_ser_b.cpp) ----------



/// Quantos jogos a liga tem: (clubes +0x3e / 2, com sinal, para zero) × rodadas +0xa9.
pub(crate) unsafe fn cqn5_jogos(liga: usize) -> i32 {
    let clubes = core::ptr::read_unaligned((liga + 0x3e) as *const i16) as i32;
    let rodadas = core::ptr::read_unaligned((liga + 0xa9) as *const i16) as i32;
    (clubes / 2).wrapping_mul(rodadas)
}

/// O corpo das duas (set_up_fixed_fixtures; thiscall, ret 0, quadro SEH só para o construtor de vetor): o vetor dos
/// jogos fixos (operator new 0x944cf1 de n × 16 + 4: push; add esp, 4 -> cdecl; a conta n no primeiro dword e cada jogo
/// pelo construtor 0x645800, com o destrutor 0x5a53c0 para a exceção: `eh vector constructor iterator` 0x9463af, push
/// 0x5a53c0; push 0x645800; push n; push 0x10; push vetor -> stdcall, ret 0x14) em +0xea; sem memória, a caixa da
/// linha `linha` do fonte `arquivo` e fim. Depois cada jogo da `tabela`, na ordem, no seu lugar do vetor
/// (fixed_fixture_set_by_club_names 0x68aff0: push o 6º (fora, pelo mapa); push o 5º (casa); push ano; push mês (0 =
/// janeiro); push dia; push liga; mov ecx, [+0xea] + 16 × k -> thiscall, ret 0x18: a data e os clubes achados pelo
/// nome, −1 em +8/+0xc se não acha). Por fim, se algum dos n jogos ficou com um clube sem achar (+8 ou +0xc = −1), o
/// vetor é destruído (`eh vector destructor iterator` 0x9462bb: push 0x5a53c0; push conta; push 0x10; push vetor ->
/// stdcall, ret 0x10; operator delete 0x944c94 do bloco: push; add esp, 4 -> cdecl) e +0xea = 0: a liga volta ao
/// calendário sorteado.
pub(crate) unsafe fn cqn5_fixos(this: usize, tabela: &[(u8, u8, u16, u32, u32)], arquivo: usize, linha: u32) {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let construir: unsafe extern "stdcall" fn(usize, u32, u32, usize, usize) = core::mem::transmute(FN_CRT_VECTOR_CONSTRUCTOR_ITERATOR);
    let destruir: unsafe extern "stdcall" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_VECTOR_DESTRUCTOR_ITERATOR);
    let apagar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
    let jogo: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) = core::mem::transmute(FN_FIXED_FIXTURE_SET_BY_CLUB_NAMES);
    let n = cqn5_jogos(this);
    let bloco = novo((n as u32).wrapping_mul(16).wrapping_add(4));
    let vetor = if bloco != 0 {
        write_u32_from_usize(bloco, n as usize);
        construir(bloco + 4, 0x10, n as u32, 0x645800, 0x5a53c0);
        bloco + 4
    } else {
        0
    };
    write_u32_from_usize(this + 0xea, vetor);
    if vetor == 0 {
        aviso_em(arquivo, linha);
        return;
    }
    for (k, &(dia, mes, ano, casa, fora)) in tabela.iter().enumerate() {
        jogo(read_u32_as_usize(this + 0xea) + k * 0x10, this, dia as u32, mes as u32, ano as u32, casa, fora); // cqn5: cada jogo fixo
    }
    crate::comparacao::ramo("cqn_5 calendario fixo");
    let n = cqn5_jogos(this);
    if n <= 0 {
        return;
    }
    let v = read_u32_as_usize(this + 0xea);
    for i in 0..n as usize {
        let f = v + i * 0x10;
        if read_u32_as_usize(f + 8) == 0xffff_ffff || read_u32_as_usize(f + 0xc) == 0xffff_ffff { // cqn5: um clube sem achar derruba a lista
            if v != 0 {
                destruir(v, 0x10, read_u32_as_usize(v - 4) as u32, 0x5a53c0);
                apagar(v - 4);
            }
            write_u32_from_usize(this + 0xea, 0);
            return;
        }
    }
}

/// the 2001/02 Serie A does not draw its fixtures: the 306 matches (34 rounds of 9) are fixed, with date and both clubs
/// by name; if any name is missing from the database, the whole list is dropped and the league falls back to the drawn
/// schedule
/// Original: `ItaSerA::create_fixed_fixtures` (0x642a70, ita_ser_a.cpp, thiscall, ret 0).
///
/// [pt] { 0x642a70 } (thiscall, ret 0, ita_ser_a.cpp, 11.650 bytes): `ItaSerA::create_fixed_fixtures()`: os 306 jogos da
/// Série A 2001/02 (34 rodadas de 9, com o dia, o mês (0 = janeiro) e o ano que a original empurra). Fonte 0x9ea2a8, linha 0x60f.
pub(crate) unsafe extern "thiscall" fn calendario_fixo_da_serie_a(this: usize) {
    cqn5_fixos(this, &CQN5_SERIE_A, 0x9ea2a8, 0x60f)
}

/// the 2001/02 Serie B: the 380 matches (38 rounds of 10) fixed, with the same rule
/// Original: `ItaSerB::set_up_fixed_fixtures` (0x647230, ita_ser_b.cpp, thiscall, ret 0).
///
/// [pt] { 0x647230 } (thiscall, ret 0, ita_ser_b.cpp, 14.388 bytes): `ItaSerB::set_up_fixed_fixtures()`: os 380 jogos da
/// Série B 2001/02 (38 rodadas de 10). Fonte 0x9ea384, linha 0x39a.
pub(crate) unsafe extern "thiscall" fn calendario_fixo_da_serie_b(this: usize) {
    cqn5_fixos(this, &CQN5_SERIE_B, 0x9ea384, 0x39a)
}

/// O comparador das duas: alocam: só a original, conferindo o que ela garante: ou a lista ficou (+0xea com a conta n e
/// cada jogo com os dois clubes achados), ou foi derrubada (+0xea = 0).
pub(crate) unsafe fn cqn5_comparar(nome: &str, this: usize, original: impl FnOnce()) {
    original();
    let v = read_u32_as_usize(this + 0xea);
    let n = cqn5_jogos(this);
    let ok = v == 0 || (read_u32_as_usize(v - 4) as i32 == n && (0..n.max(0) as usize).all(|i| read_u32_as_usize(v + i * 0x10 + 8) != 0xffff_ffff && read_u32_as_usize(v + i * 0x10 + 0xc) != 0xffff_ffff));
    PLACAR.registrar(ok, || format!("{}({:#x}): a lista dos jogos fixos (+0xea = {:#x}) fora do esperado", nome, this, v));
}

pub(crate) unsafe extern "thiscall" fn comparar_calendario_fixo_da_serie_a(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1881);
    cqn5_comparar("calendario_fixo_da_serie_a", this, || o(this))
}

pub(crate) unsafe extern "thiscall" fn comparar_calendario_fixo_da_serie_b(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1882);
    cqn5_comparar("calendario_fixo_da_serie_b", this, || o(this))
}

/// Os jogos fixos (dia, mês com 0 = janeiro, ano, o 5º argumento (casa, pelo mapa), o 6º (fora)), os nomes no .rdata do
/// exe; na ordem da original (extraídos da desmontagem e conferidos: os lugares do vetor vão de 16 em 16).
pub(crate) const CQN5_SERIE_A: [(u8, u8, u16, u32, u32); 306] = [
    (28, 7, 2001, 0x9ea370, 0x9ea378), // Bologna × Atalanta
    (28, 7, 2001, 0x9ea360, 0x9ea368), // Brescia × Milan
    (28, 7, 2001, 0x9ea34c, 0x9ea358), // Fiorentina × Chievo
    (28, 7, 2001, 0x9ea33c, 0x9ea344), // Inter × Perugia
    (28, 7, 2001, 0x9ea328, 0x9ea334), // Juventus × Venezia
    (28, 7, 2001, 0x9ea314, 0x9ea31c), // Lazio × Piacenza
    (28, 7, 2001, 0x9ea304, 0x9ea30c), // Lecce × Parma
    (28, 7, 2001, 0x9ea2f4, 0x9ea2fc), // Udinese × Torino
    (28, 7, 2001, 0x9ea2e4, 0x9ea2ec), // Verona × Roma
    (9, 8, 2001, 0x9ea378, 0x9ea328), // Atalanta × Juventus
    (9, 8, 2001, 0x9ea358, 0x9ea370), // Chievo × Bologna
    (9, 8, 2001, 0x9ea368, 0x9ea34c), // Milan × Fiorentina
    (9, 8, 2001, 0x9ea30c, 0x9ea33c), // Parma × Inter
    (9, 8, 2001, 0x9ea344, 0x9ea314), // Perugia × Lazio
    (9, 8, 2001, 0x9ea31c, 0x9ea304), // Piacenza × Lecce
    (9, 8, 2001, 0x9ea2ec, 0x9ea2f4), // Roma × Udinese
    (9, 8, 2001, 0x9ea2fc, 0x9ea360), // Torino × Brescia
    (9, 8, 2001, 0x9ea334, 0x9ea2e4), // Venezia × Verona
    (16, 8, 2001, 0x9ea370, 0x9ea30c), // Bologna × Parma
    (16, 8, 2001, 0x9ea360, 0x9ea304), // Brescia × Lecce
    (16, 8, 2001, 0x9ea34c, 0x9ea378), // Fiorentina × Atalanta
    (16, 8, 2001, 0x9ea33c, 0x9ea334), // Inter × Venezia
    (16, 8, 2001, 0x9ea328, 0x9ea358), // Juventus × Chievo
    (16, 8, 2001, 0x9ea314, 0x9ea2fc), // Lazio × Torino
    (16, 8, 2001, 0x9ea31c, 0x9ea2ec), // Piacenza × Roma
    (16, 8, 2001, 0x9ea2f4, 0x9ea368), // Udinese × Milan
    (16, 8, 2001, 0x9ea2e4, 0x9ea344), // Verona × Perugia
    (23, 8, 2001, 0x9ea378, 0x9ea2e4), // Atalanta × Verona
    (23, 8, 2001, 0x9ea358, 0x9ea31c), // Chievo × Piacenza
    (23, 8, 2001, 0x9ea304, 0x9ea328), // Lecce × Juventus
    (23, 8, 2001, 0x9ea368, 0x9ea314), // Milan × Lazio
    (23, 8, 2001, 0x9ea30c, 0x9ea360), // Parma × Brescia
    (23, 8, 2001, 0x9ea344, 0x9ea2f4), // Perugia × Udinese
    (23, 8, 2001, 0x9ea2ec, 0x9ea34c), // Roma × Fiorentina
    (23, 8, 2001, 0x9ea2fc, 0x9ea33c), // Torino × Inter
    (23, 8, 2001, 0x9ea334, 0x9ea370), // Venezia × Bologna
    (30, 8, 2001, 0x9ea360, 0x9ea378), // Brescia × Atalanta
    (30, 8, 2001, 0x9ea34c, 0x9ea334), // Fiorentina × Venezia
    (30, 8, 2001, 0x9ea33c, 0x9ea370), // Inter × Bologna
    (30, 8, 2001, 0x9ea328, 0x9ea2ec), // Juventus × Roma
    (30, 8, 2001, 0x9ea314, 0x9ea30c), // Lazio × Parma
    (30, 8, 2001, 0x9ea344, 0x9ea368), // Perugia × Milan
    (30, 8, 2001, 0x9ea31c, 0x9ea2fc), // Piacenza × Torino
    (30, 8, 2001, 0x9ea2f4, 0x9ea358), // Udinese × Chievo
    (30, 8, 2001, 0x9ea2e4, 0x9ea304), // Verona × Lecce
    (10, 9, 2001, 0x9ea378, 0x9ea368), // Atalanta × Milan
    (10, 9, 2001, 0x9ea370, 0x9ea2f4), // Bologna × Udinese
    (10, 9, 2001, 0x9ea358, 0x9ea314), // Chievo × Lazio
    (10, 9, 2001, 0x9ea34c, 0x9ea328), // Fiorentina × Juventus
    (10, 9, 2001, 0x9ea33c, 0x9ea2e4), // Inter × Verona
    (10, 9, 2001, 0x9ea304, 0x9ea344), // Lecce × Perugia
    (10, 9, 2001, 0x9ea2ec, 0x9ea360), // Roma × Brescia
    (10, 9, 2001, 0x9ea2fc, 0x9ea30c), // Torino × Parma
    (10, 9, 2001, 0x9ea334, 0x9ea31c), // Venezia × Piacenza
    (14, 9, 2001, 0x9ea360, 0x9ea358), // Brescia × Chievo
    (14, 9, 2001, 0x9ea328, 0x9ea2fc), // Juventus × Torino
    (14, 9, 2001, 0x9ea314, 0x9ea378), // Lazio × Atalanta
    (14, 9, 2001, 0x9ea304, 0x9ea34c), // Lecce × Fiorentina
    (14, 9, 2001, 0x9ea368, 0x9ea334), // Milan × Venezia
    (14, 9, 2001, 0x9ea30c, 0x9ea31c), // Parma × Piacenza
    (14, 9, 2001, 0x9ea344, 0x9ea2ec), // Perugia × Roma
    (14, 9, 2001, 0x9ea2f4, 0x9ea33c), // Udinese × Inter
    (14, 9, 2001, 0x9ea2e4, 0x9ea370), // Verona × Bologna
    (21, 9, 2001, 0x9ea378, 0x9ea2f4), // Atalanta × Udinese
    (21, 9, 2001, 0x9ea370, 0x9ea328), // Bologna × Juventus
    (21, 9, 2001, 0x9ea358, 0x9ea30c), // Chievo × Parma
    (21, 9, 2001, 0x9ea34c, 0x9ea2e4), // Fiorentina × Verona
    (21, 9, 2001, 0x9ea33c, 0x9ea368), // Inter × Milan
    (21, 9, 2001, 0x9ea31c, 0x9ea360), // Piacenza × Brescia
    (21, 9, 2001, 0x9ea2ec, 0x9ea304), // Roma × Lecce
    (21, 9, 2001, 0x9ea2fc, 0x9ea344), // Torino × Perugia
    (21, 9, 2001, 0x9ea334, 0x9ea314), // Venezia × Lazio
    (28, 9, 2001, 0x9ea360, 0x9ea334), // Brescia × Venezia
    (28, 9, 2001, 0x9ea358, 0x9ea2fc), // Chievo × Torino
    (28, 9, 2001, 0x9ea328, 0x9ea33c), // Juventus × Inter
    (28, 9, 2001, 0x9ea304, 0x9ea378), // Lecce × Atalanta
    (28, 9, 2001, 0x9ea368, 0x9ea370), // Milan × Bologna
    (28, 9, 2001, 0x9ea30c, 0x9ea2e4), // Parma × Verona
    (28, 9, 2001, 0x9ea344, 0x9ea31c), // Perugia × Piacenza
    (28, 9, 2001, 0x9ea2ec, 0x9ea314), // Roma × Lazio
    (28, 9, 2001, 0x9ea2f4, 0x9ea34c), // Udinese × Fiorentina
    (4, 10, 2001, 0x9ea378, 0x9ea2ec), // Atalanta × Roma
    (4, 10, 2001, 0x9ea370, 0x9ea34c), // Bologna × Fiorentina
    (4, 10, 2001, 0x9ea33c, 0x9ea304), // Inter × Lecce
    (4, 10, 2001, 0x9ea314, 0x9ea360), // Lazio × Brescia
    (4, 10, 2001, 0x9ea30c, 0x9ea344), // Parma × Perugia
    (4, 10, 2001, 0x9ea31c, 0x9ea2f4), // Piacenza × Udinese
    (4, 10, 2001, 0x9ea2fc, 0x9ea368), // Torino × Milan
    (4, 10, 2001, 0x9ea334, 0x9ea358), // Venezia × Chievo
    (4, 10, 2001, 0x9ea2e4, 0x9ea328), // Verona × Juventus
    (18, 10, 2001, 0x9ea378, 0x9ea334), // Atalanta × Venezia
    (18, 10, 2001, 0x9ea34c, 0x9ea2fc), // Fiorentina × Torino
    (18, 10, 2001, 0x9ea328, 0x9ea30c), // Juventus × Parma
    (18, 10, 2001, 0x9ea304, 0x9ea370), // Lecce × Bologna
    (18, 10, 2001, 0x9ea368, 0x9ea31c), // Milan × Piacenza
    (18, 10, 2001, 0x9ea344, 0x9ea360), // Perugia × Brescia
    (18, 10, 2001, 0x9ea2ec, 0x9ea33c), // Roma × Inter
    (18, 10, 2001, 0x9ea2f4, 0x9ea314), // Udinese × Lazio
    (18, 10, 2001, 0x9ea2e4, 0x9ea358), // Verona × Chievo
    (25, 10, 2001, 0x9ea370, 0x9ea2ec), // Bologna × Roma
    (25, 10, 2001, 0x9ea360, 0x9ea2f4), // Brescia × Udinese
    (25, 10, 2001, 0x9ea358, 0x9ea344), // Chievo × Perugia
    (25, 10, 2001, 0x9ea33c, 0x9ea34c), // Inter × Fiorentina
    (25, 10, 2001, 0x9ea314, 0x9ea328), // Lazio × Juventus
    (25, 10, 2001, 0x9ea30c, 0x9ea368), // Parma × Milan
    (25, 10, 2001, 0x9ea31c, 0x9ea378), // Piacenza × Atalanta
    (25, 10, 2001, 0x9ea2fc, 0x9ea2e4), // Torino × Verona
    (25, 10, 2001, 0x9ea334, 0x9ea304), // Venezia × Lecce
    (2, 11, 2001, 0x9ea378, 0x9ea33c), // Atalanta × Inter
    (2, 11, 2001, 0x9ea370, 0x9ea2fc), // Bologna × Torino
    (2, 11, 2001, 0x9ea34c, 0x9ea31c), // Fiorentina × Piacenza
    (2, 11, 2001, 0x9ea328, 0x9ea344), // Juventus × Perugia
    (2, 11, 2001, 0x9ea304, 0x9ea314), // Lecce × Lazio
    (2, 11, 2001, 0x9ea368, 0x9ea358), // Milan × Chievo
    (2, 11, 2001, 0x9ea2ec, 0x9ea334), // Roma × Venezia
    (2, 11, 2001, 0x9ea2f4, 0x9ea30c), // Udinese × Parma
    (2, 11, 2001, 0x9ea2e4, 0x9ea360), // Verona × Brescia
    (9, 11, 2001, 0x9ea360, 0x9ea33c), // Brescia × Inter
    (9, 11, 2001, 0x9ea358, 0x9ea304), // Chievo × Lecce
    (9, 11, 2001, 0x9ea314, 0x9ea34c), // Lazio × Fiorentina
    (9, 11, 2001, 0x9ea368, 0x9ea328), // Milan × Juventus
    (9, 11, 2001, 0x9ea30c, 0x9ea2ec), // Parma × Roma
    (9, 11, 2001, 0x9ea344, 0x9ea334), // Perugia × Venezia
    (9, 11, 2001, 0x9ea31c, 0x9ea370), // Piacenza × Bologna
    (9, 11, 2001, 0x9ea2fc, 0x9ea378), // Torino × Atalanta
    (9, 11, 2001, 0x9ea2f4, 0x9ea2e4), // Udinese × Verona
    (16, 11, 2001, 0x9ea378, 0x9ea30c), // Atalanta × Parma
    (16, 11, 2001, 0x9ea370, 0x9ea344), // Bologna × Perugia
    (16, 11, 2001, 0x9ea34c, 0x9ea360), // Fiorentina × Brescia
    (16, 11, 2001, 0x9ea33c, 0x9ea358), // Inter × Chievo
    (16, 11, 2001, 0x9ea328, 0x9ea31c), // Juventus × Piacenza
    (16, 11, 2001, 0x9ea304, 0x9ea2fc), // Lecce × Torino
    (16, 11, 2001, 0x9ea2ec, 0x9ea368), // Roma × Milan
    (16, 11, 2001, 0x9ea334, 0x9ea2f4), // Venezia × Udinese
    (16, 11, 2001, 0x9ea2e4, 0x9ea314), // Verona × Lazio
    (6, 0, 2002, 0x9ea360, 0x9ea328), // Brescia × Juventus
    (6, 0, 2002, 0x9ea358, 0x9ea2ec), // Chievo × Roma
    (6, 0, 2002, 0x9ea314, 0x9ea370), // Lazio × Bologna
    (6, 0, 2002, 0x9ea368, 0x9ea2e4), // Milan × Verona
    (6, 0, 2002, 0x9ea30c, 0x9ea34c), // Parma × Fiorentina
    (6, 0, 2002, 0x9ea344, 0x9ea378), // Perugia × Atalanta
    (6, 0, 2002, 0x9ea31c, 0x9ea33c), // Piacenza × Inter
    (6, 0, 2002, 0x9ea2fc, 0x9ea334), // Torino × Venezia
    (6, 0, 2002, 0x9ea2f4, 0x9ea304), // Udinese × Lecce
    (13, 0, 2002, 0x9ea378, 0x9ea358), // Atalanta × Chievo
    (13, 0, 2002, 0x9ea370, 0x9ea360), // Bologna × Brescia
    (13, 0, 2002, 0x9ea34c, 0x9ea344), // Fiorentina × Perugia
    (13, 0, 2002, 0x9ea33c, 0x9ea314), // Inter × Lazio
    (13, 0, 2002, 0x9ea328, 0x9ea2f4), // Juventus × Udinese
    (13, 0, 2002, 0x9ea304, 0x9ea368), // Lecce × Milan
    (13, 0, 2002, 0x9ea2ec, 0x9ea2fc), // Roma × Torino
    (13, 0, 2002, 0x9ea334, 0x9ea30c), // Venezia × Parma
    (13, 0, 2002, 0x9ea2e4, 0x9ea31c), // Verona × Piacenza
    (20, 0, 2002, 0x9ea378, 0x9ea370), // Atalanta × Bologna
    (20, 0, 2002, 0x9ea368, 0x9ea360), // Milan × Brescia
    (20, 0, 2002, 0x9ea358, 0x9ea34c), // Chievo × Fiorentina
    (20, 0, 2002, 0x9ea344, 0x9ea33c), // Perugia × Inter
    (20, 0, 2002, 0x9ea334, 0x9ea328), // Venezia × Juventus
    (20, 0, 2002, 0x9ea31c, 0x9ea314), // Piacenza × Lazio
    (20, 0, 2002, 0x9ea30c, 0x9ea304), // Parma × Lecce
    (20, 0, 2002, 0x9ea2fc, 0x9ea2f4), // Torino × Udinese
    (20, 0, 2002, 0x9ea2ec, 0x9ea2e4), // Roma × Verona
    (27, 0, 2002, 0x9ea328, 0x9ea378), // Juventus × Atalanta
    (27, 0, 2002, 0x9ea370, 0x9ea358), // Bologna × Chievo
    (27, 0, 2002, 0x9ea34c, 0x9ea368), // Fiorentina × Milan
    (27, 0, 2002, 0x9ea33c, 0x9ea30c), // Inter × Parma
    (27, 0, 2002, 0x9ea314, 0x9ea344), // Lazio × Perugia
    (27, 0, 2002, 0x9ea304, 0x9ea31c), // Lecce × Piacenza
    (27, 0, 2002, 0x9ea2f4, 0x9ea2ec), // Udinese × Roma
    (27, 0, 2002, 0x9ea360, 0x9ea2fc), // Brescia × Torino
    (27, 0, 2002, 0x9ea2e4, 0x9ea334), // Verona × Venezia
    (3, 1, 2002, 0x9ea30c, 0x9ea370), // Parma × Bologna
    (3, 1, 2002, 0x9ea304, 0x9ea360), // Lecce × Brescia
    (3, 1, 2002, 0x9ea378, 0x9ea34c), // Atalanta × Fiorentina
    (3, 1, 2002, 0x9ea334, 0x9ea33c), // Venezia × Inter
    (3, 1, 2002, 0x9ea358, 0x9ea328), // Chievo × Juventus
    (3, 1, 2002, 0x9ea2fc, 0x9ea314), // Torino × Lazio
    (3, 1, 2002, 0x9ea2ec, 0x9ea31c), // Roma × Piacenza
    (3, 1, 2002, 0x9ea368, 0x9ea2f4), // Milan × Udinese
    (3, 1, 2002, 0x9ea344, 0x9ea2e4), // Perugia × Verona
    (10, 1, 2002, 0x9ea2e4, 0x9ea378), // Verona × Atalanta
    (10, 1, 2002, 0x9ea31c, 0x9ea358), // Piacenza × Chievo
    (10, 1, 2002, 0x9ea328, 0x9ea304), // Juventus × Lecce
    (10, 1, 2002, 0x9ea314, 0x9ea368), // Lazio × Milan
    (10, 1, 2002, 0x9ea360, 0x9ea30c), // Brescia × Parma
    (10, 1, 2002, 0x9ea2f4, 0x9ea344), // Udinese × Perugia
    (10, 1, 2002, 0x9ea34c, 0x9ea2ec), // Fiorentina × Roma
    (10, 1, 2002, 0x9ea33c, 0x9ea2fc), // Inter × Torino
    (10, 1, 2002, 0x9ea370, 0x9ea334), // Bologna × Venezia
    (17, 1, 2002, 0x9ea378, 0x9ea360), // Atalanta × Brescia
    (17, 1, 2002, 0x9ea334, 0x9ea34c), // Venezia × Fiorentina
    (17, 1, 2002, 0x9ea370, 0x9ea33c), // Bologna × Inter
    (17, 1, 2002, 0x9ea2ec, 0x9ea328), // Roma × Juventus
    (17, 1, 2002, 0x9ea30c, 0x9ea314), // Parma × Lazio
    (17, 1, 2002, 0x9ea368, 0x9ea344), // Milan × Perugia
    (17, 1, 2002, 0x9ea2fc, 0x9ea31c), // Torino × Piacenza
    (17, 1, 2002, 0x9ea358, 0x9ea2f4), // Chievo × Udinese
    (17, 1, 2002, 0x9ea304, 0x9ea2e4), // Lecce × Verona
    (24, 1, 2002, 0x9ea368, 0x9ea378), // Milan × Atalanta
    (24, 1, 2002, 0x9ea2f4, 0x9ea370), // Udinese × Bologna
    (24, 1, 2002, 0x9ea314, 0x9ea358), // Lazio × Chievo
    (24, 1, 2002, 0x9ea328, 0x9ea34c), // Juventus × Fiorentina
    (24, 1, 2002, 0x9ea2e4, 0x9ea33c), // Verona × Inter
    (24, 1, 2002, 0x9ea344, 0x9ea304), // Perugia × Lecce
    (24, 1, 2002, 0x9ea360, 0x9ea2ec), // Brescia × Roma
    (24, 1, 2002, 0x9ea30c, 0x9ea2fc), // Parma × Torino
    (24, 1, 2002, 0x9ea31c, 0x9ea334), // Piacenza × Venezia
    (3, 2, 2002, 0x9ea358, 0x9ea360), // Chievo × Brescia
    (3, 2, 2002, 0x9ea2fc, 0x9ea328), // Torino × Juventus
    (3, 2, 2002, 0x9ea378, 0x9ea314), // Atalanta × Lazio
    (3, 2, 2002, 0x9ea34c, 0x9ea304), // Fiorentina × Lecce
    (3, 2, 2002, 0x9ea334, 0x9ea368), // Venezia × Milan
    (3, 2, 2002, 0x9ea31c, 0x9ea30c), // Piacenza × Parma
    (3, 2, 2002, 0x9ea2ec, 0x9ea344), // Roma × Perugia
    (3, 2, 2002, 0x9ea33c, 0x9ea2f4), // Inter × Udinese
    (3, 2, 2002, 0x9ea370, 0x9ea2e4), // Bologna × Verona
    (10, 2, 2002, 0x9ea2f4, 0x9ea378), // Udinese × Atalanta
    (10, 2, 2002, 0x9ea328, 0x9ea370), // Juventus × Bologna
    (10, 2, 2002, 0x9ea30c, 0x9ea358), // Parma × Chievo
    (10, 2, 2002, 0x9ea2e4, 0x9ea34c), // Verona × Fiorentina
    (10, 2, 2002, 0x9ea368, 0x9ea33c), // Milan × Inter
    (10, 2, 2002, 0x9ea360, 0x9ea31c), // Brescia × Piacenza
    (10, 2, 2002, 0x9ea304, 0x9ea2ec), // Lecce × Roma
    (10, 2, 2002, 0x9ea344, 0x9ea2fc), // Perugia × Torino
    (10, 2, 2002, 0x9ea314, 0x9ea334), // Lazio × Venezia
    (17, 2, 2002, 0x9ea334, 0x9ea360), // Venezia × Brescia
    (17, 2, 2002, 0x9ea2fc, 0x9ea358), // Torino × Chievo
    (17, 2, 2002, 0x9ea33c, 0x9ea328), // Inter × Juventus
    (17, 2, 2002, 0x9ea378, 0x9ea304), // Atalanta × Lecce
    (17, 2, 2002, 0x9ea370, 0x9ea368), // Bologna × Milan
    (17, 2, 2002, 0x9ea2e4, 0x9ea30c), // Verona × Parma
    (17, 2, 2002, 0x9ea31c, 0x9ea344), // Piacenza × Perugia
    (17, 2, 2002, 0x9ea314, 0x9ea2ec), // Lazio × Roma
    (17, 2, 2002, 0x9ea34c, 0x9ea2f4), // Fiorentina × Udinese
    (24, 2, 2002, 0x9ea2ec, 0x9ea378), // Roma × Atalanta
    (24, 2, 2002, 0x9ea34c, 0x9ea370), // Fiorentina × Bologna
    (24, 2, 2002, 0x9ea304, 0x9ea33c), // Lecce × Inter
    (24, 2, 2002, 0x9ea360, 0x9ea314), // Brescia × Lazio
    (24, 2, 2002, 0x9ea344, 0x9ea30c), // Perugia × Parma
    (24, 2, 2002, 0x9ea2f4, 0x9ea31c), // Udinese × Piacenza
    (24, 2, 2002, 0x9ea368, 0x9ea2fc), // Milan × Torino
    (24, 2, 2002, 0x9ea358, 0x9ea334), // Chievo × Venezia
    (24, 2, 2002, 0x9ea328, 0x9ea2e4), // Juventus × Verona
    (30, 2, 2002, 0x9ea334, 0x9ea378), // Venezia × Atalanta
    (30, 2, 2002, 0x9ea2fc, 0x9ea34c), // Torino × Fiorentina
    (30, 2, 2002, 0x9ea30c, 0x9ea328), // Parma × Juventus
    (30, 2, 2002, 0x9ea370, 0x9ea304), // Bologna × Lecce
    (30, 2, 2002, 0x9ea31c, 0x9ea368), // Piacenza × Milan
    (30, 2, 2002, 0x9ea360, 0x9ea344), // Brescia × Perugia
    (30, 2, 2002, 0x9ea33c, 0x9ea2ec), // Inter × Roma
    (30, 2, 2002, 0x9ea314, 0x9ea2f4), // Lazio × Udinese
    (30, 2, 2002, 0x9ea358, 0x9ea2e4), // Chievo × Verona
    (7, 3, 2002, 0x9ea2ec, 0x9ea370), // Roma × Bologna
    (7, 3, 2002, 0x9ea2f4, 0x9ea360), // Udinese × Brescia
    (7, 3, 2002, 0x9ea344, 0x9ea358), // Perugia × Chievo
    (7, 3, 2002, 0x9ea34c, 0x9ea33c), // Fiorentina × Inter
    (7, 3, 2002, 0x9ea328, 0x9ea314), // Juventus × Lazio
    (7, 3, 2002, 0x9ea368, 0x9ea30c), // Milan × Parma
    (7, 3, 2002, 0x9ea378, 0x9ea31c), // Atalanta × Piacenza
    (7, 3, 2002, 0x9ea2e4, 0x9ea2fc), // Verona × Torino
    (7, 3, 2002, 0x9ea304, 0x9ea334), // Lecce × Venezia
    (14, 3, 2002, 0x9ea33c, 0x9ea378), // Inter × Atalanta
    (14, 3, 2002, 0x9ea2fc, 0x9ea370), // Torino × Bologna
    (14, 3, 2002, 0x9ea31c, 0x9ea34c), // Piacenza × Fiorentina
    (14, 3, 2002, 0x9ea344, 0x9ea328), // Perugia × Juventus
    (14, 3, 2002, 0x9ea314, 0x9ea304), // Lazio × Lecce
    (14, 3, 2002, 0x9ea358, 0x9ea368), // Chievo × Milan
    (14, 3, 2002, 0x9ea334, 0x9ea2ec), // Venezia × Roma
    (14, 3, 2002, 0x9ea30c, 0x9ea2f4), // Parma × Udinese
    (14, 3, 2002, 0x9ea360, 0x9ea2e4), // Brescia × Verona
    (21, 3, 2002, 0x9ea33c, 0x9ea360), // Inter × Brescia
    (21, 3, 2002, 0x9ea304, 0x9ea358), // Lecce × Chievo
    (21, 3, 2002, 0x9ea34c, 0x9ea314), // Fiorentina × Lazio
    (21, 3, 2002, 0x9ea328, 0x9ea368), // Juventus × Milan
    (21, 3, 2002, 0x9ea2ec, 0x9ea30c), // Roma × Parma
    (21, 3, 2002, 0x9ea334, 0x9ea344), // Venezia × Perugia
    (21, 3, 2002, 0x9ea370, 0x9ea31c), // Bologna × Piacenza
    (21, 3, 2002, 0x9ea378, 0x9ea2fc), // Atalanta × Torino
    (21, 3, 2002, 0x9ea2e4, 0x9ea2f4), // Verona × Udinese
    (28, 3, 2002, 0x9ea30c, 0x9ea378), // Parma × Atalanta
    (28, 3, 2002, 0x9ea344, 0x9ea370), // Perugia × Bologna
    (28, 3, 2002, 0x9ea360, 0x9ea34c), // Brescia × Fiorentina
    (28, 3, 2002, 0x9ea358, 0x9ea33c), // Chievo × Inter
    (28, 3, 2002, 0x9ea31c, 0x9ea328), // Piacenza × Juventus
    (28, 3, 2002, 0x9ea2fc, 0x9ea304), // Torino × Lecce
    (28, 3, 2002, 0x9ea368, 0x9ea2ec), // Milan × Roma
    (28, 3, 2002, 0x9ea2f4, 0x9ea334), // Udinese × Venezia
    (28, 3, 2002, 0x9ea314, 0x9ea2e4), // Lazio × Verona
    (5, 4, 2002, 0x9ea328, 0x9ea360), // Juventus × Brescia
    (5, 4, 2002, 0x9ea2ec, 0x9ea358), // Roma × Chievo
    (5, 4, 2002, 0x9ea370, 0x9ea314), // Bologna × Lazio
    (5, 4, 2002, 0x9ea2e4, 0x9ea368), // Verona × Milan
    (5, 4, 2002, 0x9ea34c, 0x9ea30c), // Fiorentina × Parma
    (5, 4, 2002, 0x9ea378, 0x9ea344), // Atalanta × Perugia
    (5, 4, 2002, 0x9ea33c, 0x9ea31c), // Inter × Piacenza
    (5, 4, 2002, 0x9ea334, 0x9ea2fc), // Venezia × Torino
    (5, 4, 2002, 0x9ea304, 0x9ea2f4), // Lecce × Udinese
    (12, 4, 2002, 0x9ea358, 0x9ea378), // Chievo × Atalanta
    (12, 4, 2002, 0x9ea360, 0x9ea370), // Brescia × Bologna
    (12, 4, 2002, 0x9ea344, 0x9ea34c), // Perugia × Fiorentina
    (12, 4, 2002, 0x9ea314, 0x9ea33c), // Lazio × Inter
    (12, 4, 2002, 0x9ea2f4, 0x9ea328), // Udinese × Juventus
    (12, 4, 2002, 0x9ea368, 0x9ea304), // Milan × Lecce
    (12, 4, 2002, 0x9ea2fc, 0x9ea2ec), // Torino × Roma
    (12, 4, 2002, 0x9ea30c, 0x9ea334), // Parma × Venezia
    (12, 4, 2002, 0x9ea31c, 0x9ea2e4), // Piacenza × Verona
];

pub(crate) const CQN5_SERIE_B: [(u8, u8, u16, u32, u32); 380] = [
    (26, 7, 2001, 0x9ea464, 0x9ea46c), // Ancona × Reggina
    (26, 7, 2001, 0x9ea450, 0x9ea45c), // Cagliari × Messina
    (26, 7, 2001, 0x9ea440, 0x9ea448), // Como × Crotone
    (26, 7, 2001, 0x9ea42c, 0x9ea434), // Cosenza × Cittadella
    (26, 7, 2001, 0x9ea41c, 0x9ea424), // Empoli × Palermo
    (26, 7, 2001, 0x9ea40c, 0x9ea414), // Genoa × Napoli
    (26, 7, 2001, 0x9ea3fc, 0x9ea404), // Modena × Bari
    (26, 7, 2001, 0x9ea3e4, 0x9ea3f0), // Salernitana × Sampdoria
    (26, 7, 2001, 0x9ea3d4, 0x9ea3dc), // Ternana × Siena
    (26, 7, 2001, 0x9ea3c0, 0x9ea3c8), // Vicenza × Pistoiese
    (2, 8, 2001, 0x9ea404, 0x9ea41c), // Bari × Empoli
    (2, 8, 2001, 0x9ea434, 0x9ea3e4), // Cittadella × Salernitana
    (2, 8, 2001, 0x9ea448, 0x9ea3fc), // Crotone × Modena
    (2, 8, 2001, 0x9ea45c, 0x9ea3d4), // Messina × Ternana
    (2, 8, 2001, 0x9ea414, 0x9ea464), // Napoli × Ancona
    (2, 8, 2001, 0x9ea424, 0x9ea450), // Palermo × Cagliari
    (2, 8, 2001, 0x9ea3c8, 0x9ea40c), // Pistoiese × Genoa
    (2, 8, 2001, 0x9ea46c, 0x9ea440), // Reggina × Como
    (2, 8, 2001, 0x9ea3f0, 0x9ea42c), // Sampdoria × Cosenza
    (2, 8, 2001, 0x9ea3dc, 0x9ea3c0), // Siena × Vicenza
    (9, 8, 2001, 0x9ea464, 0x9ea3dc), // Ancona × Siena
    (9, 8, 2001, 0x9ea450, 0x9ea404), // Cagliari × Bari
    (9, 8, 2001, 0x9ea42c, 0x9ea3d4), // Cosenza × Ternana
    (9, 8, 2001, 0x9ea41c, 0x9ea45c), // Empoli × Messina
    (9, 8, 2001, 0x9ea40c, 0x9ea440), // Genoa × Como
    (9, 8, 2001, 0x9ea3fc, 0x9ea414), // Modena × Napoli
    (9, 8, 2001, 0x9ea424, 0x9ea3c8), // Palermo × Pistoiese
    (9, 8, 2001, 0x9ea46c, 0x9ea3f0), // Reggina × Sampdoria
    (9, 8, 2001, 0x9ea3e4, 0x9ea448), // Salernitana × Crotone
    (9, 8, 2001, 0x9ea3c0, 0x9ea434), // Vicenza × Cittadella
    (16, 8, 2001, 0x9ea404, 0x9ea424), // Bari × Palermo
    (16, 8, 2001, 0x9ea434, 0x9ea46c), // Cittadella × Reggina
    (16, 8, 2001, 0x9ea440, 0x9ea3e4), // Como × Salernitana
    (16, 8, 2001, 0x9ea448, 0x9ea450), // Crotone × Cagliari
    (16, 8, 2001, 0x9ea45c, 0x9ea464), // Messina × Ancona
    (16, 8, 2001, 0x9ea414, 0x9ea3c0), // Napoli × Vicenza
    (16, 8, 2001, 0x9ea3c8, 0x9ea42c), // Pistoiese × Cosenza
    (16, 8, 2001, 0x9ea3f0, 0x9ea41c), // Sampdoria × Empoli
    (16, 8, 2001, 0x9ea3dc, 0x9ea40c), // Siena × Genoa
    (16, 8, 2001, 0x9ea3d4, 0x9ea3fc), // Ternana × Modena
    (23, 8, 2001, 0x9ea464, 0x9ea448), // Ancona × Crotone
    (23, 8, 2001, 0x9ea450, 0x9ea3f0), // Cagliari × Sampdoria
    (23, 8, 2001, 0x9ea42c, 0x9ea440), // Cosenza × Como
    (23, 8, 2001, 0x9ea41c, 0x9ea414), // Empoli × Napoli
    (23, 8, 2001, 0x9ea40c, 0x9ea45c), // Genoa × Messina
    (23, 8, 2001, 0x9ea3fc, 0x9ea3dc), // Modena × Siena
    (23, 8, 2001, 0x9ea424, 0x9ea434), // Palermo × Cittadella
    (23, 8, 2001, 0x9ea46c, 0x9ea404), // Reggina × Bari
    (23, 8, 2001, 0x9ea3e4, 0x9ea3c8), // Salernitana × Pistoiese
    (23, 8, 2001, 0x9ea3c0, 0x9ea3d4), // Vicenza × Ternana
    (30, 8, 2001, 0x9ea404, 0x9ea448), // Bari × Crotone
    (30, 8, 2001, 0x9ea434, 0x9ea41c), // Cittadella × Empoli
    (30, 8, 2001, 0x9ea440, 0x9ea464), // Como × Ancona
    (30, 8, 2001, 0x9ea42c, 0x9ea46c), // Cosenza × Reggina
    (30, 8, 2001, 0x9ea45c, 0x9ea3c0), // Messina × Vicenza
    (30, 8, 2001, 0x9ea414, 0x9ea450), // Napoli × Cagliari
    (30, 8, 2001, 0x9ea3c8, 0x9ea3fc), // Pistoiese × Modena
    (30, 8, 2001, 0x9ea3f0, 0x9ea424), // Sampdoria × Palermo
    (30, 8, 2001, 0x9ea3dc, 0x9ea3e4), // Siena × Salernitana
    (30, 8, 2001, 0x9ea3d4, 0x9ea40c), // Ternana × Genoa
    (7, 9, 2001, 0x9ea464, 0x9ea40c), // Ancona × Genoa
    (7, 9, 2001, 0x9ea450, 0x9ea3dc), // Cagliari × Siena
    (7, 9, 2001, 0x9ea448, 0x9ea434), // Crotone × Cittadella
    (7, 9, 2001, 0x9ea41c, 0x9ea3c8), // Empoli × Pistoiese
    (7, 9, 2001, 0x9ea3fc, 0x9ea440), // Modena × Como
    (7, 9, 2001, 0x9ea46c, 0x9ea424), // Reggina × Palermo
    (7, 9, 2001, 0x9ea3e4, 0x9ea45c), // Salernitana × Messina
    (7, 9, 2001, 0x9ea3f0, 0x9ea404), // Sampdoria × Bari
    (7, 9, 2001, 0x9ea3d4, 0x9ea414), // Ternana × Napoli
    (7, 9, 2001, 0x9ea3c0, 0x9ea42c), // Vicenza × Cosenza
    (10, 9, 2001, 0x9ea464, 0x9ea3f0), // Ancona × Sampdoria
    (10, 9, 2001, 0x9ea440, 0x9ea3d4), // Como × Ternana
    (10, 9, 2001, 0x9ea42c, 0x9ea3fc), // Cosenza × Modena
    (10, 9, 2001, 0x9ea41c, 0x9ea450), // Empoli × Cagliari
    (10, 9, 2001, 0x9ea40c, 0x9ea434), // Genoa × Cittadella
    (10, 9, 2001, 0x9ea45c, 0x9ea3dc), // Messina × Siena
    (10, 9, 2001, 0x9ea414, 0x9ea3e4), // Napoli × Salernitana
    (10, 9, 2001, 0x9ea424, 0x9ea448), // Palermo × Crotone
    (10, 9, 2001, 0x9ea3c8, 0x9ea46c), // Pistoiese × Reggina
    (10, 9, 2001, 0x9ea3c0, 0x9ea404), // Vicenza × Bari
    (14, 9, 2001, 0x9ea404, 0x9ea40c), // Bari × Genoa
    (14, 9, 2001, 0x9ea434, 0x9ea414), // Cittadella × Napoli
    (14, 9, 2001, 0x9ea448, 0x9ea3c0), // Crotone × Vicenza
    (14, 9, 2001, 0x9ea45c, 0x9ea424), // Messina × Palermo
    (14, 9, 2001, 0x9ea3fc, 0x9ea450), // Modena × Cagliari
    (14, 9, 2001, 0x9ea46c, 0x9ea41c), // Reggina × Empoli
    (14, 9, 2001, 0x9ea3e4, 0x9ea42c), // Salernitana × Cosenza
    (14, 9, 2001, 0x9ea3f0, 0x9ea3c8), // Sampdoria × Pistoiese
    (14, 9, 2001, 0x9ea3dc, 0x9ea440), // Siena × Como
    (14, 9, 2001, 0x9ea3d4, 0x9ea464), // Ternana × Ancona
    (28, 9, 2001, 0x9ea464, 0x9ea434), // Ancona × Cittadella
    (28, 9, 2001, 0x9ea450, 0x9ea46c), // Cagliari × Reggina
    (28, 9, 2001, 0x9ea440, 0x9ea45c), // Como × Messina
    (28, 9, 2001, 0x9ea42c, 0x9ea3dc), // Cosenza × Siena
    (28, 9, 2001, 0x9ea41c, 0x9ea448), // Empoli × Crotone
    (28, 9, 2001, 0x9ea40c, 0x9ea3e4), // Genoa × Salernitana
    (28, 9, 2001, 0x9ea414, 0x9ea3f0), // Napoli × Sampdoria
    (28, 9, 2001, 0x9ea424, 0x9ea3d4), // Palermo × Ternana
    (28, 9, 2001, 0x9ea3c8, 0x9ea404), // Pistoiese × Bari
    (28, 9, 2001, 0x9ea3c0, 0x9ea3fc), // Vicenza × Modena
    (4, 10, 2001, 0x9ea404, 0x9ea414), // Bari × Napoli
    (4, 10, 2001, 0x9ea434, 0x9ea450), // Cittadella × Cagliari
    (4, 10, 2001, 0x9ea440, 0x9ea3c0), // Como × Vicenza
    (4, 10, 2001, 0x9ea448, 0x9ea46c), // Crotone × Reggina
    (4, 10, 2001, 0x9ea40c, 0x9ea3f0), // Genoa × Sampdoria
    (4, 10, 2001, 0x9ea45c, 0x9ea42c), // Messina × Cosenza
    (4, 10, 2001, 0x9ea3fc, 0x9ea41c), // Modena × Empoli
    (4, 10, 2001, 0x9ea3e4, 0x9ea464), // Salernitana × Ancona
    (4, 10, 2001, 0x9ea3dc, 0x9ea424), // Siena × Palermo
    (4, 10, 2001, 0x9ea3d4, 0x9ea3c8), // Ternana × Pistoiese
    (11, 10, 2001, 0x9ea404, 0x9ea3dc), // Bari × Siena
    (11, 10, 2001, 0x9ea450, 0x9ea3d4), // Cagliari × Ternana
    (11, 10, 2001, 0x9ea42c, 0x9ea40c), // Cosenza × Genoa
    (11, 10, 2001, 0x9ea41c, 0x9ea3e4), // Empoli × Salernitana
    (11, 10, 2001, 0x9ea414, 0x9ea448), // Napoli × Crotone
    (11, 10, 2001, 0x9ea424, 0x9ea3fc), // Palermo × Modena
    (11, 10, 2001, 0x9ea3c8, 0x9ea440), // Pistoiese × Como
    (11, 10, 2001, 0x9ea46c, 0x9ea45c), // Reggina × Messina
    (11, 10, 2001, 0x9ea3f0, 0x9ea434), // Sampdoria × Cittadella
    (11, 10, 2001, 0x9ea3c0, 0x9ea464), // Vicenza × Ancona
    (18, 10, 2001, 0x9ea464, 0x9ea42c), // Ancona × Cosenza
    (18, 10, 2001, 0x9ea434, 0x9ea404), // Cittadella × Bari
    (18, 10, 2001, 0x9ea440, 0x9ea424), // Como × Palermo
    (18, 10, 2001, 0x9ea448, 0x9ea3f0), // Crotone × Sampdoria
    (18, 10, 2001, 0x9ea40c, 0x9ea3c0), // Genoa × Vicenza
    (18, 10, 2001, 0x9ea45c, 0x9ea414), // Messina × Napoli
    (18, 10, 2001, 0x9ea3fc, 0x9ea46c), // Modena × Reggina
    (18, 10, 2001, 0x9ea3e4, 0x9ea450), // Salernitana × Cagliari
    (18, 10, 2001, 0x9ea3dc, 0x9ea3c8), // Siena × Pistoiese
    (18, 10, 2001, 0x9ea3d4, 0x9ea41c), // Ternana × Empoli
    (25, 10, 2001, 0x9ea404, 0x9ea440), // Bari × Como
    (25, 10, 2001, 0x9ea450, 0x9ea464), // Cagliari × Ancona
    (25, 10, 2001, 0x9ea434, 0x9ea3d4), // Cittadella × Ternana
    (25, 10, 2001, 0x9ea448, 0x9ea42c), // Crotone × Cosenza
    (25, 10, 2001, 0x9ea41c, 0x9ea3dc), // Empoli × Siena
    (25, 10, 2001, 0x9ea424, 0x9ea40c), // Palermo × Genoa
    (25, 10, 2001, 0x9ea3c8, 0x9ea45c), // Pistoiese × Messina
    (25, 10, 2001, 0x9ea46c, 0x9ea414), // Reggina × Napoli
    (25, 10, 2001, 0x9ea3e4, 0x9ea3c0), // Salernitana × Vicenza
    (25, 10, 2001, 0x9ea3f0, 0x9ea3fc), // Sampdoria × Modena
    (2, 11, 2001, 0x9ea464, 0x9ea3c8), // Ancona × Pistoiese
    (2, 11, 2001, 0x9ea440, 0x9ea450), // Como × Cagliari
    (2, 11, 2001, 0x9ea42c, 0x9ea404), // Cosenza × Bari
    (2, 11, 2001, 0x9ea40c, 0x9ea448), // Genoa × Crotone
    (2, 11, 2001, 0x9ea45c, 0x9ea3f0), // Messina × Sampdoria
    (2, 11, 2001, 0x9ea3fc, 0x9ea434), // Modena × Cittadella
    (2, 11, 2001, 0x9ea414, 0x9ea424), // Napoli × Palermo
    (2, 11, 2001, 0x9ea3dc, 0x9ea46c), // Siena × Reggina
    (2, 11, 2001, 0x9ea3d4, 0x9ea3e4), // Ternana × Salernitana
    (2, 11, 2001, 0x9ea3c0, 0x9ea41c), // Vicenza × Empoli
    (9, 11, 2001, 0x9ea404, 0x9ea464), // Bari × Ancona
    (9, 11, 2001, 0x9ea450, 0x9ea3c8), // Cagliari × Pistoiese
    (9, 11, 2001, 0x9ea434, 0x9ea45c), // Cittadella × Messina
    (9, 11, 2001, 0x9ea448, 0x9ea3dc), // Crotone × Siena
    (9, 11, 2001, 0x9ea41c, 0x9ea440), // Empoli × Como
    (9, 11, 2001, 0x9ea3fc, 0x9ea40c), // Modena × Genoa
    (9, 11, 2001, 0x9ea414, 0x9ea42c), // Napoli × Cosenza
    (9, 11, 2001, 0x9ea424, 0x9ea3c0), // Palermo × Vicenza
    (9, 11, 2001, 0x9ea46c, 0x9ea3e4), // Reggina × Salernitana
    (9, 11, 2001, 0x9ea3f0, 0x9ea3d4), // Sampdoria × Ternana
    (16, 11, 2001, 0x9ea464, 0x9ea3fc), // Ancona × Modena
    (16, 11, 2001, 0x9ea440, 0x9ea3f0), // Como × Sampdoria
    (16, 11, 2001, 0x9ea42c, 0x9ea450), // Cosenza × Cagliari
    (16, 11, 2001, 0x9ea40c, 0x9ea41c), // Genoa × Empoli
    (16, 11, 2001, 0x9ea45c, 0x9ea404), // Messina × Bari
    (16, 11, 2001, 0x9ea3c8, 0x9ea414), // Pistoiese × Napoli
    (16, 11, 2001, 0x9ea3e4, 0x9ea424), // Salernitana × Palermo
    (16, 11, 2001, 0x9ea3dc, 0x9ea434), // Siena × Cittadella
    (16, 11, 2001, 0x9ea3d4, 0x9ea448), // Ternana × Crotone
    (16, 11, 2001, 0x9ea3c0, 0x9ea46c), // Vicenza × Reggina
    (23, 11, 2001, 0x9ea404, 0x9ea3e4), // Bari × Salernitana
    (23, 11, 2001, 0x9ea450, 0x9ea40c), // Cagliari × Genoa
    (23, 11, 2001, 0x9ea434, 0x9ea440), // Cittadella × Como
    (23, 11, 2001, 0x9ea448, 0x9ea3c8), // Crotone × Pistoiese
    (23, 11, 2001, 0x9ea41c, 0x9ea464), // Empoli × Ancona
    (23, 11, 2001, 0x9ea3fc, 0x9ea45c), // Modena × Messina
    (23, 11, 2001, 0x9ea414, 0x9ea3dc), // Napoli × Siena
    (23, 11, 2001, 0x9ea424, 0x9ea42c), // Palermo × Cosenza
    (23, 11, 2001, 0x9ea46c, 0x9ea3d4), // Reggina × Ternana
    (23, 11, 2001, 0x9ea3f0, 0x9ea3c0), // Sampdoria × Vicenza
    (6, 0, 2002, 0x9ea464, 0x9ea424), // Ancona × Palermo
    (6, 0, 2002, 0x9ea440, 0x9ea414), // Como × Napoli
    (6, 0, 2002, 0x9ea42c, 0x9ea41c), // Cosenza × Empoli
    (6, 0, 2002, 0x9ea40c, 0x9ea46c), // Genoa × Reggina
    (6, 0, 2002, 0x9ea45c, 0x9ea448), // Messina × Crotone
    (6, 0, 2002, 0x9ea3c8, 0x9ea434), // Pistoiese × Cittadella
    (6, 0, 2002, 0x9ea3e4, 0x9ea3fc), // Salernitana × Modena
    (6, 0, 2002, 0x9ea3dc, 0x9ea3f0), // Siena × Sampdoria
    (6, 0, 2002, 0x9ea3d4, 0x9ea404), // Ternana × Bari
    (6, 0, 2002, 0x9ea3c0, 0x9ea450), // Vicenza × Cagliari
    (13, 0, 2002, 0x9ea46c, 0x9ea464), // Reggina × Ancona
    (13, 0, 2002, 0x9ea45c, 0x9ea450), // Messina × Cagliari
    (13, 0, 2002, 0x9ea448, 0x9ea440), // Crotone × Como
    (13, 0, 2002, 0x9ea434, 0x9ea42c), // Cittadella × Cosenza
    (13, 0, 2002, 0x9ea424, 0x9ea41c), // Palermo × Empoli
    (13, 0, 2002, 0x9ea414, 0x9ea40c), // Napoli × Genoa
    (13, 0, 2002, 0x9ea404, 0x9ea3fc), // Bari × Modena
    (13, 0, 2002, 0x9ea3f0, 0x9ea3e4), // Sampdoria × Salernitana
    (13, 0, 2002, 0x9ea3dc, 0x9ea3d4), // Siena × Ternana
    (13, 0, 2002, 0x9ea3c8, 0x9ea3c0), // Pistoiese × Vicenza
    (20, 0, 2002, 0x9ea41c, 0x9ea404), // Empoli × Bari
    (20, 0, 2002, 0x9ea3e4, 0x9ea434), // Salernitana × Cittadella
    (20, 0, 2002, 0x9ea3fc, 0x9ea448), // Modena × Crotone
    (20, 0, 2002, 0x9ea3d4, 0x9ea45c), // Ternana × Messina
    (20, 0, 2002, 0x9ea464, 0x9ea414), // Ancona × Napoli
    (20, 0, 2002, 0x9ea450, 0x9ea424), // Cagliari × Palermo
    (20, 0, 2002, 0x9ea40c, 0x9ea3c8), // Genoa × Pistoiese
    (20, 0, 2002, 0x9ea440, 0x9ea46c), // Como × Reggina
    (20, 0, 2002, 0x9ea42c, 0x9ea3f0), // Cosenza × Sampdoria
    (20, 0, 2002, 0x9ea3c0, 0x9ea3dc), // Vicenza × Siena
    (3, 1, 2002, 0x9ea3dc, 0x9ea464), // Siena × Ancona
    (3, 1, 2002, 0x9ea404, 0x9ea450), // Bari × Cagliari
    (3, 1, 2002, 0x9ea3d4, 0x9ea42c), // Ternana × Cosenza
    (3, 1, 2002, 0x9ea45c, 0x9ea41c), // Messina × Empoli
    (3, 1, 2002, 0x9ea440, 0x9ea40c), // Como × Genoa
    (3, 1, 2002, 0x9ea414, 0x9ea3fc), // Napoli × Modena
    (3, 1, 2002, 0x9ea3c8, 0x9ea424), // Pistoiese × Palermo
    (3, 1, 2002, 0x9ea3f0, 0x9ea46c), // Sampdoria × Reggina
    (3, 1, 2002, 0x9ea448, 0x9ea3e4), // Crotone × Salernitana
    (3, 1, 2002, 0x9ea434, 0x9ea3c0), // Cittadella × Vicenza
    (10, 1, 2002, 0x9ea424, 0x9ea404), // Palermo × Bari
    (10, 1, 2002, 0x9ea46c, 0x9ea434), // Reggina × Cittadella
    (10, 1, 2002, 0x9ea3e4, 0x9ea440), // Salernitana × Como
    (10, 1, 2002, 0x9ea450, 0x9ea448), // Cagliari × Crotone
    (10, 1, 2002, 0x9ea464, 0x9ea45c), // Ancona × Messina
    (10, 1, 2002, 0x9ea3c0, 0x9ea414), // Vicenza × Napoli
    (10, 1, 2002, 0x9ea42c, 0x9ea3c8), // Cosenza × Pistoiese
    (10, 1, 2002, 0x9ea41c, 0x9ea3f0), // Empoli × Sampdoria
    (10, 1, 2002, 0x9ea40c, 0x9ea3dc), // Genoa × Siena
    (10, 1, 2002, 0x9ea3fc, 0x9ea3d4), // Modena × Ternana
    (17, 1, 2002, 0x9ea448, 0x9ea464), // Crotone × Ancona
    (17, 1, 2002, 0x9ea3f0, 0x9ea450), // Sampdoria × Cagliari
    (17, 1, 2002, 0x9ea440, 0x9ea42c), // Como × Cosenza
    (17, 1, 2002, 0x9ea414, 0x9ea41c), // Napoli × Empoli
    (17, 1, 2002, 0x9ea45c, 0x9ea40c), // Messina × Genoa
    (17, 1, 2002, 0x9ea3dc, 0x9ea3fc), // Siena × Modena
    (17, 1, 2002, 0x9ea434, 0x9ea424), // Cittadella × Palermo
    (17, 1, 2002, 0x9ea404, 0x9ea46c), // Bari × Reggina
    (17, 1, 2002, 0x9ea3c8, 0x9ea3e4), // Pistoiese × Salernitana
    (17, 1, 2002, 0x9ea3d4, 0x9ea3c0), // Ternana × Vicenza
    (24, 1, 2002, 0x9ea448, 0x9ea404), // Crotone × Bari
    (24, 1, 2002, 0x9ea41c, 0x9ea434), // Empoli × Cittadella
    (24, 1, 2002, 0x9ea464, 0x9ea440), // Ancona × Como
    (24, 1, 2002, 0x9ea46c, 0x9ea42c), // Reggina × Cosenza
    (24, 1, 2002, 0x9ea3c0, 0x9ea45c), // Vicenza × Messina
    (24, 1, 2002, 0x9ea450, 0x9ea414), // Cagliari × Napoli
    (24, 1, 2002, 0x9ea3fc, 0x9ea3c8), // Modena × Pistoiese
    (24, 1, 2002, 0x9ea424, 0x9ea3f0), // Palermo × Sampdoria
    (24, 1, 2002, 0x9ea3e4, 0x9ea3dc), // Salernitana × Siena
    (24, 1, 2002, 0x9ea40c, 0x9ea3d4), // Genoa × Ternana
    (3, 2, 2002, 0x9ea40c, 0x9ea464), // Genoa × Ancona
    (3, 2, 2002, 0x9ea3dc, 0x9ea450), // Siena × Cagliari
    (3, 2, 2002, 0x9ea434, 0x9ea448), // Cittadella × Crotone
    (3, 2, 2002, 0x9ea3c8, 0x9ea41c), // Pistoiese × Empoli
    (3, 2, 2002, 0x9ea440, 0x9ea3fc), // Como × Modena
    (3, 2, 2002, 0x9ea424, 0x9ea46c), // Palermo × Reggina
    (3, 2, 2002, 0x9ea45c, 0x9ea3e4), // Messina × Salernitana
    (3, 2, 2002, 0x9ea404, 0x9ea3f0), // Bari × Sampdoria
    (3, 2, 2002, 0x9ea414, 0x9ea3d4), // Napoli × Ternana
    (3, 2, 2002, 0x9ea42c, 0x9ea3c0), // Cosenza × Vicenza
    (10, 2, 2002, 0x9ea3f0, 0x9ea464), // Sampdoria × Ancona
    (10, 2, 2002, 0x9ea3d4, 0x9ea440), // Ternana × Como
    (10, 2, 2002, 0x9ea3fc, 0x9ea42c), // Modena × Cosenza
    (10, 2, 2002, 0x9ea450, 0x9ea41c), // Cagliari × Empoli
    (10, 2, 2002, 0x9ea434, 0x9ea40c), // Cittadella × Genoa
    (10, 2, 2002, 0x9ea3dc, 0x9ea45c), // Siena × Messina
    (10, 2, 2002, 0x9ea3e4, 0x9ea414), // Salernitana × Napoli
    (10, 2, 2002, 0x9ea448, 0x9ea424), // Crotone × Palermo
    (10, 2, 2002, 0x9ea46c, 0x9ea3c8), // Reggina × Pistoiese
    (10, 2, 2002, 0x9ea404, 0x9ea3c0), // Bari × Vicenza
    (17, 2, 2002, 0x9ea40c, 0x9ea404), // Genoa × Bari
    (17, 2, 2002, 0x9ea414, 0x9ea434), // Napoli × Cittadella
    (17, 2, 2002, 0x9ea3c0, 0x9ea448), // Vicenza × Crotone
    (17, 2, 2002, 0x9ea424, 0x9ea45c), // Palermo × Messina
    (17, 2, 2002, 0x9ea450, 0x9ea3fc), // Cagliari × Modena
    (17, 2, 2002, 0x9ea41c, 0x9ea46c), // Empoli × Reggina
    (17, 2, 2002, 0x9ea42c, 0x9ea3e4), // Cosenza × Salernitana
    (17, 2, 2002, 0x9ea3c8, 0x9ea3f0), // Pistoiese × Sampdoria
    (17, 2, 2002, 0x9ea440, 0x9ea3dc), // Como × Siena
    (17, 2, 2002, 0x9ea464, 0x9ea3d4), // Ancona × Ternana
    (30, 2, 2002, 0x9ea434, 0x9ea464), // Cittadella × Ancona
    (30, 2, 2002, 0x9ea46c, 0x9ea450), // Reggina × Cagliari
    (30, 2, 2002, 0x9ea45c, 0x9ea440), // Messina × Como
    (30, 2, 2002, 0x9ea3dc, 0x9ea42c), // Siena × Cosenza
    (30, 2, 2002, 0x9ea448, 0x9ea41c), // Crotone × Empoli
    (30, 2, 2002, 0x9ea3e4, 0x9ea40c), // Salernitana × Genoa
    (30, 2, 2002, 0x9ea3f0, 0x9ea414), // Sampdoria × Napoli
    (30, 2, 2002, 0x9ea3d4, 0x9ea424), // Ternana × Palermo
    (30, 2, 2002, 0x9ea404, 0x9ea3c8), // Bari × Pistoiese
    (30, 2, 2002, 0x9ea3fc, 0x9ea3c0), // Modena × Vicenza
    (7, 3, 2002, 0x9ea414, 0x9ea404), // Napoli × Bari
    (7, 3, 2002, 0x9ea450, 0x9ea434), // Cagliari × Cittadella
    (7, 3, 2002, 0x9ea3c0, 0x9ea440), // Vicenza × Como
    (7, 3, 2002, 0x9ea46c, 0x9ea448), // Reggina × Crotone
    (7, 3, 2002, 0x9ea3f0, 0x9ea40c), // Sampdoria × Genoa
    (7, 3, 2002, 0x9ea42c, 0x9ea45c), // Cosenza × Messina
    (7, 3, 2002, 0x9ea41c, 0x9ea3fc), // Empoli × Modena
    (7, 3, 2002, 0x9ea464, 0x9ea3e4), // Ancona × Salernitana
    (7, 3, 2002, 0x9ea424, 0x9ea3dc), // Palermo × Siena
    (7, 3, 2002, 0x9ea3c8, 0x9ea3d4), // Pistoiese × Ternana
    (14, 3, 2002, 0x9ea3dc, 0x9ea404), // Siena × Bari
    (14, 3, 2002, 0x9ea3d4, 0x9ea450), // Ternana × Cagliari
    (14, 3, 2002, 0x9ea40c, 0x9ea42c), // Genoa × Cosenza
    (14, 3, 2002, 0x9ea3e4, 0x9ea41c), // Salernitana × Empoli
    (14, 3, 2002, 0x9ea448, 0x9ea414), // Crotone × Napoli
    (14, 3, 2002, 0x9ea3fc, 0x9ea424), // Modena × Palermo
    (14, 3, 2002, 0x9ea440, 0x9ea3c8), // Como × Pistoiese
    (14, 3, 2002, 0x9ea45c, 0x9ea46c), // Messina × Reggina
    (14, 3, 2002, 0x9ea434, 0x9ea3f0), // Cittadella × Sampdoria
    (14, 3, 2002, 0x9ea464, 0x9ea3c0), // Ancona × Vicenza
    (21, 3, 2002, 0x9ea42c, 0x9ea464), // Cosenza × Ancona
    (21, 3, 2002, 0x9ea404, 0x9ea434), // Bari × Cittadella
    (21, 3, 2002, 0x9ea424, 0x9ea440), // Palermo × Como
    (21, 3, 2002, 0x9ea3f0, 0x9ea448), // Sampdoria × Crotone
    (21, 3, 2002, 0x9ea3c0, 0x9ea40c), // Vicenza × Genoa
    (21, 3, 2002, 0x9ea414, 0x9ea45c), // Napoli × Messina
    (21, 3, 2002, 0x9ea46c, 0x9ea3fc), // Reggina × Modena
    (21, 3, 2002, 0x9ea450, 0x9ea3e4), // Cagliari × Salernitana
    (21, 3, 2002, 0x9ea3c8, 0x9ea3dc), // Pistoiese × Siena
    (21, 3, 2002, 0x9ea41c, 0x9ea3d4), // Empoli × Ternana
    (28, 3, 2002, 0x9ea440, 0x9ea404), // Como × Bari
    (28, 3, 2002, 0x9ea464, 0x9ea450), // Ancona × Cagliari
    (28, 3, 2002, 0x9ea3d4, 0x9ea434), // Ternana × Cittadella
    (28, 3, 2002, 0x9ea42c, 0x9ea448), // Cosenza × Crotone
    (28, 3, 2002, 0x9ea3dc, 0x9ea41c), // Siena × Empoli
    (28, 3, 2002, 0x9ea40c, 0x9ea424), // Genoa × Palermo
    (28, 3, 2002, 0x9ea45c, 0x9ea3c8), // Messina × Pistoiese
    (28, 3, 2002, 0x9ea414, 0x9ea46c), // Napoli × Reggina
    (28, 3, 2002, 0x9ea3c0, 0x9ea3e4), // Vicenza × Salernitana
    (28, 3, 2002, 0x9ea3fc, 0x9ea3f0), // Modena × Sampdoria
    (5, 4, 2002, 0x9ea3c8, 0x9ea464), // Pistoiese × Ancona
    (5, 4, 2002, 0x9ea450, 0x9ea440), // Cagliari × Como
    (5, 4, 2002, 0x9ea404, 0x9ea42c), // Bari × Cosenza
    (5, 4, 2002, 0x9ea448, 0x9ea40c), // Crotone × Genoa
    (5, 4, 2002, 0x9ea3f0, 0x9ea45c), // Sampdoria × Messina
    (5, 4, 2002, 0x9ea434, 0x9ea3fc), // Cittadella × Modena
    (5, 4, 2002, 0x9ea424, 0x9ea414), // Palermo × Napoli
    (5, 4, 2002, 0x9ea46c, 0x9ea3dc), // Reggina × Siena
    (5, 4, 2002, 0x9ea3e4, 0x9ea3d4), // Salernitana × Ternana
    (5, 4, 2002, 0x9ea41c, 0x9ea3c0), // Empoli × Vicenza
    (12, 4, 2002, 0x9ea464, 0x9ea404), // Ancona × Bari
    (12, 4, 2002, 0x9ea3c8, 0x9ea450), // Pistoiese × Cagliari
    (12, 4, 2002, 0x9ea45c, 0x9ea434), // Messina × Cittadella
    (12, 4, 2002, 0x9ea3dc, 0x9ea448), // Siena × Crotone
    (12, 4, 2002, 0x9ea440, 0x9ea41c), // Como × Empoli
    (12, 4, 2002, 0x9ea40c, 0x9ea3fc), // Genoa × Modena
    (12, 4, 2002, 0x9ea42c, 0x9ea414), // Cosenza × Napoli
    (12, 4, 2002, 0x9ea3c0, 0x9ea424), // Vicenza × Palermo
    (12, 4, 2002, 0x9ea3e4, 0x9ea46c), // Salernitana × Reggina
    (12, 4, 2002, 0x9ea3d4, 0x9ea3f0), // Ternana × Sampdoria
    (19, 4, 2002, 0x9ea3fc, 0x9ea464), // Modena × Ancona
    (19, 4, 2002, 0x9ea3f0, 0x9ea440), // Sampdoria × Como
    (19, 4, 2002, 0x9ea450, 0x9ea42c), // Cagliari × Cosenza
    (19, 4, 2002, 0x9ea41c, 0x9ea40c), // Empoli × Genoa
    (19, 4, 2002, 0x9ea404, 0x9ea45c), // Bari × Messina
    (19, 4, 2002, 0x9ea414, 0x9ea3c8), // Napoli × Pistoiese
    (19, 4, 2002, 0x9ea424, 0x9ea3e4), // Palermo × Salernitana
    (19, 4, 2002, 0x9ea434, 0x9ea3dc), // Cittadella × Siena
    (19, 4, 2002, 0x9ea448, 0x9ea3d4), // Crotone × Ternana
    (19, 4, 2002, 0x9ea46c, 0x9ea3c0), // Reggina × Vicenza
    (26, 4, 2002, 0x9ea3e4, 0x9ea404), // Salernitana × Bari
    (26, 4, 2002, 0x9ea40c, 0x9ea450), // Genoa × Cagliari
    (26, 4, 2002, 0x9ea440, 0x9ea434), // Como × Cittadella
    (26, 4, 2002, 0x9ea3c8, 0x9ea448), // Pistoiese × Crotone
    (26, 4, 2002, 0x9ea464, 0x9ea41c), // Ancona × Empoli
    (26, 4, 2002, 0x9ea45c, 0x9ea3fc), // Messina × Modena
    (26, 4, 2002, 0x9ea3dc, 0x9ea414), // Siena × Napoli
    (26, 4, 2002, 0x9ea42c, 0x9ea424), // Cosenza × Palermo
    (26, 4, 2002, 0x9ea3d4, 0x9ea46c), // Ternana × Reggina
    (26, 4, 2002, 0x9ea3c0, 0x9ea3f0), // Vicenza × Sampdoria
    (2, 5, 2002, 0x9ea424, 0x9ea464), // Palermo × Ancona
    (2, 5, 2002, 0x9ea414, 0x9ea440), // Napoli × Como
    (2, 5, 2002, 0x9ea41c, 0x9ea42c), // Empoli × Cosenza
    (2, 5, 2002, 0x9ea46c, 0x9ea40c), // Reggina × Genoa
    (2, 5, 2002, 0x9ea448, 0x9ea45c), // Crotone × Messina
    (2, 5, 2002, 0x9ea434, 0x9ea3c8), // Cittadella × Pistoiese
    (2, 5, 2002, 0x9ea3fc, 0x9ea3e4), // Modena × Salernitana
    (2, 5, 2002, 0x9ea3f0, 0x9ea3dc), // Sampdoria × Siena
    (2, 5, 2002, 0x9ea404, 0x9ea3d4), // Bari × Ternana
    (2, 5, 2002, 0x9ea450, 0x9ea3c0), // Cagliari × Vicenza
];


// ---------- 275a leva: as regras das divisões italianas (Série A, B, C1/A e C1/B): turnos, pontos, vagas, desempates, o empate que vai ao jogo-desempate e a tabela de rodadas (ita_ser_a.cpp, ita_ser_b.cpp, ita_ser_c1a.cpp, ita_ser_c1b.cpp) ----------


/// No ensaio (o comparador) a cópia grava as regras e não chama a virtual +0x3c (a tabela de rodadas, que aloca).
pub(crate) static mut CQI6_ENSAIO: bool = false;

/// Grava as regras na liga (deslocamento, tamanho 1/2/4, valor), na ordem da lista, e monta a tabela de rodadas da
/// fase principal: a virtual +0x3c (create_schedule: thiscall(liga, −1, &+0xa9, &+0x3a, 0), ret 0x10) com o
/// resultado em +0xba. Devolve 1.
pub(crate) unsafe fn cqi6_regras(this: usize, escritas: &[(usize, u8, u32)]) -> u32 {
    for &(d, t, v) in escritas {
        match t {
            1 => *((this + d) as *mut u8) = v as u8,
            2 => core::ptr::write_unaligned((this + d) as *mut u16, v as u16),
            _ => core::ptr::write_unaligned((this + d) as *mut u32, v),
        }
    }
    if !CQI6_ENSAIO {
        let rodadas: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + 0x3c));
        let t = rodadas(this, 0xffff_ffff, this + 0xa9, this + 0x3a, 0);
        core::ptr::write_unaligned((this + 0xba) as *mut usize, t);
    }
    1
}

/// As regras comuns às quatro: 1 ponto pelo empate (+0xc3), categoria 2 nas partidas (+0xc4), +0xab e +0xdb = 0 (as
/// palavras copiadas nas partidas), clubes nacionais (+0x42 = 1), o 1º desempate 1 (+0xc5) e o 2º 2 (+0xc6), a ordem
/// fixa das rodadas na grade (bit 0x40 de +0xd9, com o byte de baixo como está), 7 reservas (+0x49) e 3 substituições
/// (+0x4a).
pub(crate) unsafe fn cqi6_comuns(this: usize) -> Vec<(usize, u8, u32)> {
    let d9 = *((this + 0xd9) as *const u8) | 0x40;
    vec![(0xc3, 1, 1), (0xc4, 1, 2), (0xab, 2, 0), (0xdb, 2, 0), (0x42, 1, 1), (0xc5, 1, 1), (0xc6, 1, 2), (0xd9, 1, d9 as u32), (0x49, 1, 7), (0x4a, 1, 3)]
}

/// Serie A (18 clubs): 2 rounds, 3 points for a win and 1 for a draw, nobody goes up, the bottom 4 go down to B; 7
/// substitutes and 3 substitutions; tie-breaks 1, 2 and 4 [codes; inference: goal difference, goals for, head to head];
/// a points tie across a place border goes to a tie-break match (+0xe6); fixed round order
/// Original: `ItaSerA::set_up_rules` (0x641640, ita_ser_a.cpp, thiscall, ret 0).
///
/// [pt] { 0x641640 } (thiscall, ret 0, ita_ser_a.cpp): `ItaSerA::set_up_rules()`. A Série A: as comuns, 2 turnos, 3
/// pontos, os desempates 3º = 4 e 4º = 0 (+0xc7, +0xc8), ninguém sobe, nenhum play-off, 4 caem direto (+0xc1),
/// nenhuma liga acima (+0x1c = −1), a Série B abaixo (+0x20 = [0x9cf574]), +0x52 = 2 [sem nome] e +0xe6 = 1 (o
/// empate em pontos na fronteira de uma vaga não se decide pelos critérios: vai ao jogo-desempate, ver 0x642880).
pub(crate) unsafe extern "thiscall" fn regras_da_serie_a(this: usize) -> u32 {
    crate::comparacao::ramo("cqi_6 regras da serie a");
    cqi6_regras(this, &cqi6_serie_a(this))
}

pub(crate) unsafe fn cqi6_serie_a(this: usize) -> Vec<(usize, u8, u32)> {
    let mut v = cqi6_comuns(this);
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3), (0xc7, 1, 4), (0xc8, 1, 0), (0xbe, 1, 0), (0xbf, 1, 0), (0xc0, 1, 0)]);
    v.push((0xc1, 1, 4)); // cqi6: a Série A rebaixa 4
    v.extend_from_slice(&[(0x1c, 4, 0xffff_ffff), (0x52, 1, 2), (0x20, 4, read_usize(KEY_CLUB_COMP_ITALIAN_SERIE_B) as u32), (0xe6, 4, 1)]);
    v
}

/// Serie B (20): the top 4 go up to A, the bottom 4 go down to C1; the rest as in A (tie-break match too)
/// Original: `ItaSerB::set_up_rules` (0x6468a0, ita_ser_b.cpp, thiscall, ret 0).
///
/// [pt] { 0x6468a0 } (thiscall, ret 0, ita_ser_b.cpp): `ItaSerB::set_up_rules()`. A Série B: como a A, mas 4 sobem
/// direto (+0xbe) e 4 caem (+0xc1); a A acima ([0x9cf570]) e a C1/A abaixo ([0x9cf578]).
pub(crate) unsafe extern "thiscall" fn regras_da_serie_b(this: usize) -> u32 {
    crate::comparacao::ramo("cqi_6 regras da serie b");
    cqi6_regras(this, &cqi6_serie_b(this))
}

pub(crate) unsafe fn cqi6_serie_b(this: usize) -> Vec<(usize, u8, u32)> {
    let mut v = cqi6_comuns(this);
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3)]); // cqi6: a Série B joga 2 turnos
    v.extend_from_slice(&[(0xc7, 1, 4), (0xc8, 1, 0), (0xbe, 1, 4), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 4), (0x1c, 4, read_usize(KEY_CLUB_COMP_ITALIAN_SERIE_A) as u32), (0x20, 4, read_usize(KEY_CLUB_COMP_ITALIAN_SERIE_C1_A) as u32), (0x52, 1, 2), (0xe6, 4, 1)]);
    v
}

/// Serie C1/A (18): the champion goes up to B, 2nd to 5th go to the promotion play-off, 14th to 17th to the relegation
/// play-out and the last goes down; tie-breaks 1 and 2; the league below stored is C1/B (as in the original)
/// Original: `ItaSerC1a::set_up_rules` (0x64b770, ita_ser_c1a.cpp, thiscall, ret 0).
///
/// [pt] { 0x64b770 } (thiscall, ret 0, ita_ser_c1a.cpp): `ItaSerC1a::set_up_rules()`. A C1/A: as comuns, 2 turnos, 3
/// pontos, os desempates 3º = 0 (+0xc7), o campeão sobe direto (+0xbe = 1), do 2º ao 5º vão ao play-off de acesso
/// (+0xbf = 4), do 14º ao 17º ao play-out de permanência (+0xc0 = 4), o último cai (+0xc1 = 1), a Série B acima
/// ([0x9cf574]) e abaixo o id de [0x9cf57c] (o da C1/B, o outro grupo: assim no original).
pub(crate) unsafe extern "thiscall" fn regras_da_c1a(this: usize) -> u32 {
    crate::comparacao::ramo("cqi_6 regras da c1a");
    cqi6_regras(this, &cqi6_c1a(this))
}

pub(crate) unsafe fn cqi6_c1a(this: usize) -> Vec<(usize, u8, u32)> {
    let mut v = cqi6_comuns(this);
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3)]); // cqi6: a C1/A dá 3 pontos pela vitória
    v.extend_from_slice(&[(0xc7, 1, 0), (0xbe, 1, 1), (0xbf, 1, 4), (0xc0, 1, 4), (0xc1, 1, 1), (0x1c, 4, read_usize(KEY_CLUB_COMP_ITALIAN_SERIE_B) as u32), (0x20, 4, read_usize(KEY_CLUB_COMP_ITALIAN_SERIE_C1_B) as u32)]);
    v
}

/// Serie C1/B (18): as C1/A; below it C2/A when the nation has its lower divisions loaded [inference from nation bit 4]
/// Original: `ItaSerC1b::set_up_rules` (0x64f920, ita_ser_c1b.cpp, thiscall, ret 0).
///
/// [pt] { 0x64f920 } (thiscall, ret 0, ita_ser_c1b.cpp): `ItaSerC1b::set_up_rules()`. A C1/B: como a C1/A; acima o id de
/// [0x9cf578] e abaixo a C2/A ([0x9cf580]) quando a nação tem as divisões de baixo carregadas (bit 4 de +0x11c da
/// nação [inferência]), senão −1.
pub(crate) unsafe extern "thiscall" fn regras_da_c1b(this: usize) -> u32 {
    crate::comparacao::ramo("cqi_6 regras da c1b");
    cqi6_regras(this, &cqi6_c1b(this))
}

pub(crate) unsafe fn cqi6_c1b(this: usize) -> Vec<(usize, u8, u32)> {
    let mut v = cqi6_comuns(this);
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3), (0xc7, 1, 0), (0xbe, 1, 1), (0xbf, 1, 4), (0xc0, 1, 4), (0xc1, 1, 1), (0x1c, 4, read_usize(KEY_CLUB_COMP_ITALIAN_SERIE_C1_A) as u32)]);
    let nacao = read_usize(read_usize(this + 4) + 0x5d);
    let abaixo = if *((nacao + 0x11c) as *const u8) & 4 != 0 { read_usize(KEY_CLUB_COMP_ITALIAN_SERIE_C2_A) as u32 } else { 0xffff_ffff };
    v.push((0x20, 4, abaixo));
    v
}

/// O comparador das quatro (ensaio, como o da cqi_4): a original roda inteira; a faixa +0x1c..+0xec volta e a cópia
/// grava as regras sem a virtual +0x3c; os campos que a cópia grava saem iguais aos da original, e a devolução 1; no
/// fim fica a faixa da original.
pub(crate) unsafe fn cqi6_comparar(nome: &str, this: usize, escritas: Vec<(usize, u8, u32)>, original: impl FnOnce() -> u32, copia: impl FnOnce() -> u32) -> u32 {
    let faixa = [(this + 0x1c, 0xd0)];
    let antes = crate::comparacao::Memoria::ler(&faixa);
    let r = original();
    let mo = crate::comparacao::Memoria::ler(&faixa);
    antes.escrever();
    CQI6_ENSAIO = true;
    let rr = crate::aleatorio::pausar_rastro(copia);
    CQI6_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&faixa);
    mo.escrever();
    let campo = |m: &crate::comparacao::Memoria, d: usize, t: u8| -> u32 {
        let b = &m.0[0].1[d - 0x1c..];
        match t {
            1 => b[0] as u32,
            2 => u16::from_le_bytes([b[0], b[1]]) as u32,
            _ => u32::from_le_bytes([b[0], b[1], b[2], b[3]]),
        }
    };
    let difs: Vec<(usize, u32, u32)> = escritas.iter().filter(|e| campo(&mo, e.0, e.1) != campo(&mr, e.0, e.1)).map(|e| (e.0, campo(&mo, e.0, e.1), campo(&mr, e.0, e.1))).collect();
    PLACAR.registrar(difs.is_empty() && r == rr, || format!("{}({:#x}) original={} rust={} (deslocamento, original, rust)={:x?}", nome, this, r, rr, difs));
    r
}

pub(crate) unsafe extern "thiscall" fn comparar_regras_da_serie_a(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1748);
    cqi6_comparar("regras_da_serie_a", this, cqi6_serie_a(this), || o(this), || regras_da_serie_a(this))
}

pub(crate) unsafe extern "thiscall" fn comparar_regras_da_serie_b(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1749);
    cqi6_comparar("regras_da_serie_b", this, cqi6_serie_b(this), || o(this), || regras_da_serie_b(this))
}

pub(crate) unsafe extern "thiscall" fn comparar_regras_da_c1a(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1750);
    cqi6_comparar("regras_da_c1a", this, cqi6_c1a(this), || o(this), || regras_da_c1a(this))
}

pub(crate) unsafe extern "thiscall" fn comparar_regras_da_c1b(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1751);
    cqi6_comparar("regras_da_c1b", this, cqi6_c1b(this), || o(this), || regras_da_c1b(this))
}

// ---------- 275a leva: a tabela das divisões italianas (Série A, B, C1/A e C1/B): os clubes de cada divisão, o fundo de prêmios e o dinheiro da TV (ita_ser_a.cpp, ita_ser_b.cpp, ita_ser_c1a.cpp, ita_ser_c1b.cpp) ----------



/// O corpo comum: +0x3e = `n` clubes; a tabela (malloc 0x944e46 de n × 0x3b; sem memória: a caixa do fonte
/// `arquivo` na linha `linha` e None); os clubes de [0xae23b4] (0x245 bytes, [0xae235c] deles) cuja competição
/// (+0x57, o id no +0) é a do global `g`, na ordem do banco, cada um na linha seguinte (League::init_table_row
/// 0x687430: thiscall(liga, linha, clube, 0, 0), ret 0x10), até encher. Devolve se encheu.
pub(crate) unsafe fn cqj4_encher(this: usize, n: i16, arquivo: usize, linha: u32, g: usize) -> Option<bool> {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, n);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let t = alocar((n as i32 as u32).wrapping_mul(0x3b));
    core::ptr::write_unaligned((this + 0xb1) as *mut usize, t);
    if t == 0 {
        aviso_em(arquivo, linha);
        return None;
    }
    let iniciar: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(FN_LEAGUE_INIT_TABLE_ROW);
    let mut k: u8 = 0;
    let mut i: i16 = 0;
    let mut clube = read_usize(0xae23b4);
    while (i as i32) < read_usize(0xae235c) as i32 {
        let comp = read_usize(clube + 0x57);
        if comp != 0 && read_usize(comp) == read_usize(g) {
            let linha = k; // cqj4: a linha k recebe o k-ésimo clube da divisão
            // ponytail: a original empurra a linha no eax com os bits de cima do id da competição; a 0x687430 só lê
            // o byte.
            iniciar(this, linha as u32, clube, 0, 0); // cqj4: as linhas sem marca
            k = k.wrapping_add(1);
            if k as i8 as i16 == read_i16(this + 0x3e) {
                break;
            }
        }
        i = i.wrapping_add(1);
        clube = clube.wrapping_add(0x245);
    }
    Some(k as i8 as i16 == read_i16(this + 0x3e))
}

/// O fundo de prêmios (+0x28 = League::calc_prize_fund 0x682f70: thiscall(liga, cota), ret 4) e +0xe1 = 1 (o fundo
/// é pago pela tabela no fim da liga, 0x682e10).
pub(crate) unsafe fn cqj4_fundo(this: usize, cota: i32) {
    let premio: unsafe extern "thiscall" fn(usize, i32) -> i32 = core::mem::transmute(FN_LEAGUE_CALC_PRIZE_FUND);
    let f = premio(this, cota);
    core::ptr::write_unaligned((this + 0x28) as *mut i32, f);
    *((this + 0xe1) as *mut u8) = 1;
}

/// Serie A has 18 clubs; the prize fund starts at 250,000 per place (paid by table position when the league ends); TV
/// pays 5,000,000 to each club
/// Original: `ItaSerA::create_table` (0x641510, ita_ser_a.cpp, thiscall, ret 0).
///
/// [pt] { 0x641510 } (thiscall, ret 0, ita_ser_a.cpp): `ItaSerA::create_table()`: 18 clubes ([0x9cf570]); cheia, o fundo
/// pela cota de 250.000 e a TV de 5.000.000 a cada clube (League::pay_tv_rights 0x683010: thiscall(liga, valor, 0),
/// ret 8); devolve 1 (0 sem encher). Fonte 0x9ea2a8, linha 0x253.
pub(crate) unsafe extern "thiscall" fn tabela_da_serie_a(this: usize) -> u32 {
    match cqj4_encher(this, 0x12, 0x9ea2a8, 0x253, KEY_CLUB_COMP_ITALIAN_SERIE_A) {
        Some(true) => {}
        _ => return 0,
    }
    crate::comparacao::ramo("cqj_4 tabela da serie a");
    cqj4_fundo(this, 0x3d090); // cqj4: a cota da Série A
    let pagar: unsafe extern "thiscall" fn(usize, i32, usize) = core::mem::transmute(FN_LEAGUE_PAY_TV_RIGHTS);
    pagar(this, 0x4c4b40, 0); // cqj4: a TV da Série A
    1
}

/// Serie B has 20 clubs; prize fund of 70,000 per place; no TV money
/// Original: `ItaSerB::create_table` (0x646780, ita_ser_b.cpp, thiscall, ret 0).
///
/// [pt] { 0x646780 } (thiscall, ret 0, ita_ser_b.cpp): `ItaSerB::create_table()`: 20 clubes ([0x9cf574]); cheia, o fundo
/// pela cota de 70.000 (sem TV) e 1. Fonte 0x9ea384, linha 0x19d.
pub(crate) unsafe extern "thiscall" fn tabela_da_serie_b(this: usize) -> u32 {
    match cqj4_encher(this, 0x14, 0x9ea384, 0x19d, KEY_CLUB_COMP_ITALIAN_SERIE_B) {
        Some(true) => {}
        _ => return 0,
    }
    cqj4_fundo(this, 0x11170);
    1
}

/// Serie C1/A has 18 clubs; no prize fund and no TV money
/// Original: `ItaSerC1a::create_table` (0x64b670, ita_ser_c1a.cpp, thiscall, ret 0).
///
/// [pt] { 0x64b670 } (thiscall, ret 0, ita_ser_c1a.cpp): `ItaSerC1a::create_table()`: 18 clubes ([0x9cf578]), sem fundo
/// nem TV; devolve se encheu. Fonte 0x9ea474, linha 0x1bd.
pub(crate) unsafe extern "thiscall" fn tabela_da_c1a(this: usize) -> u32 {
    cqj4_encher(this, 0x12, 0x9ea474, 0x1bd, KEY_CLUB_COMP_ITALIAN_SERIE_C1_A).unwrap_or(false) as u32
}

/// Serie C1/B has 18 clubs; no prize fund and no TV money
/// Original: `ItaSerC1b::create_table` (0x64f820, ita_ser_c1b.cpp, thiscall, ret 0).
///
/// [pt] { 0x64f820 } (thiscall, ret 0, ita_ser_c1b.cpp): `ItaSerC1b::create_table()`: 18 clubes ([0x9cf57c]); como a
/// C1/A. Fonte 0x9ea558, linha 0x1be.
pub(crate) unsafe extern "thiscall" fn tabela_da_c1b(this: usize) -> u32 {
    cqj4_encher(this, 0x12, 0x9ea558, 0x1be, KEY_CLUB_COMP_ITALIAN_SERIE_C1_B).unwrap_or(false) as u32
}

/// O comparador das quatro: a original aloca a tabela (e a da A paga a TV, com notícia): só ela. O plano, feito
/// antes: os clubes da divisão na ordem do banco (até `n`). Depois: +0x3e = `n`, a devolução = 1 se o plano encheu,
/// as linhas com os clubes do plano na ordem e, com `cota`, cheia, +0xe1 = 1 e +0x28 = o fundo da cota.
pub(crate) unsafe fn cqj4_comparar(nome: &str, this: usize, n: i16, g: usize, cota: Option<i32>, original: impl FnOnce() -> u32) -> u32 {
    let mut plano: Vec<usize> = Vec::new();
    let mut i: i32 = 0;
    while i < read_usize(0xae235c) as i32 && (plano.len() as i16) < n {
        let clube = read_usize(0xae23b4) + (i as usize) * 0x245;
        let comp = read_usize(clube + 0x57);
        if comp != 0 && read_usize(comp) == read_usize(g) {
            plano.push(clube);
        }
        i += 1;
    }
    let r = original();
    let t = read_usize(this + 0xb1);
    let mut igual = r == (plano.len() as i16 == n) as u32 && read_i16(this + 0x3e) == n && t != 0;
    let mut achou: Vec<usize> = Vec::new();
    if igual {
        achou = (0..plano.len()).map(|k| read_usize(t + k * 0x3b)).collect();
        igual = achou == plano;
        if let (1, Some(c)) = (r, cota) {
            let premio: unsafe extern "thiscall" fn(usize, i32) -> i32 = core::mem::transmute(FN_LEAGUE_CALC_PRIZE_FUND);
            igual = igual && *((this + 0xe1) as *const u8) == 1 && core::ptr::read_unaligned((this + 0x28) as *const i32) == premio(this, c);
        }
    }
    PLACAR.registrar(igual, || format!("{}({:#x}) original={} plano={:x?} tabela={:x?}", nome, this, r, plano, achou));
    r
}

pub(crate) unsafe extern "thiscall" fn comparar_tabela_da_serie_a(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1752);
    cqj4_comparar("tabela_da_serie_a", this, 0x12, KEY_CLUB_COMP_ITALIAN_SERIE_A, Some(0x3d090), || o(this))
}

pub(crate) unsafe extern "thiscall" fn comparar_tabela_da_serie_b(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1753);
    cqj4_comparar("tabela_da_serie_b", this, 0x14, KEY_CLUB_COMP_ITALIAN_SERIE_B, Some(0x11170), || o(this))
}

pub(crate) unsafe extern "thiscall" fn comparar_tabela_da_c1a(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1754);
    cqj4_comparar("tabela_da_c1a", this, 0x12, KEY_CLUB_COMP_ITALIAN_SERIE_C1_A, None, || o(this))
}

pub(crate) unsafe extern "thiscall" fn comparar_tabela_da_c1b(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1755);
    cqj4_comparar("tabela_da_c1b", this, 0x12, KEY_CLUB_COMP_ITALIAN_SERIE_C1_B, None, || o(this))
}

