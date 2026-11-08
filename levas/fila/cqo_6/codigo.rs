// ---------- %%LEVA%%a leva: o calendário das primeiras divisões da Itália e da França e da Série B: as rodadas da temporada, o jogo-desempate italiano e as 34 ou 38 rodadas da francesa (ita_ser_a.cpp, ita_ser_b.cpp, fra_first.cpp) ----------

/// Uma rodada da tabela de 0x41 bytes: a data (dia, mês 0..11, anos a somar ao ano da liga +0x40, dia da semana para
/// onde a data é levada), o código do byte +4 ([inferência] o turno) e as faixas de TV em ordem (dia da semana, turno,
/// quantos jogos; a lista termina em −1).
type Cqo6Rodada = (i8, i8, i8, i8, i8, &'static [(i8, i8, i8)]);

/// As rodadas `primeira`.. da tabela `t`, na ordem da original: para cada uma, a data (0x68a160: push 0; push ano;
/// push código; push semana; push anos; push mês; push dia; push i; push t; add esp -> cdecl: o valor 0) e as
/// faixas (0x68a1c0: push 0; push c; push b; push a; push j; push i; push t; add esp -> cdecl, valor 0).
unsafe fn ita_ser_a_set_league_rounds(t: usize, ano: u32, primeira: i32, tabela: &[Cqo6Rodada]) {
    let data: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, i32, u32, u32) -> usize = core::mem::transmute(0x68a160usize);
    let faixa: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, u32) = core::mem::transmute(0x68a1c0usize);
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
unsafe fn cqo6_gravados(t: usize, guia: usize, n: usize) -> Vec<u8> {
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
struct Cqo6Liga {
    rodadas: i16,
    arquivo: usize,
    linha: u32,
    tabela: &'static [Cqo6Rodada],
    ramo: &'static str,
}

/// O corpo comum (thiscall(liga, fase, &rodadas, &extra, &é_liga), ret 0x10; sem estado salvo): só a fase principal
/// (−1; as outras devolvem 0 sem gravar nada): é_liga (se pedido) = 1, as rodadas, extra = 0, a tabela de rodadas × 0x41
/// bytes (malloc 0x944e46: push; add esp, 4 -> cdecl; sem memória: a caixa e 0) com as datas e as faixas da liga.
/// Devolve a tabela.
unsafe fn build_league_schedule(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32, p: &Cqo6Liga) -> usize {
    if fase as u8 != 0xff {
        return 0;
    }
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    if !liga.is_null() {
        *liga = 1;
    }
    *rodadas = p.rodadas;
    *extra = 0;
    let t = alocar(p.rodadas as u32 * 0x41);
    if t == 0 {
        show_error_at(p.arquivo, p.linha);
        return 0;
    }
    crate::comparacao::ramo(p.ramo);
    let ano = core::ptr::read_unaligned((this + 0x40) as *const u16) as u32;
    ita_ser_a_set_league_rounds(t, ano, 0, p.tabela);
    t
}

/// Lado a lado: a original com as saídas de quem chama, a cópia com saídas próprias, a partir do mesmo [0xb67a34]; as
/// saídas e os bytes gravados das duas tabelas têm de ser iguais (a da cópia é liberada: 0x9452ca, cdecl).
unsafe fn ita_ser_a_compare_schedule(nome: &str, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32,
                       original: impl FnOnce() -> usize, copia: impl FnOnce(*mut i16, *mut i16, *mut i32) -> usize) -> usize {
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
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
/// `cqo6_gravados`; nas outras as de copa, `eng_cc_cup_written_round_bytes`); a tabela da cópia é solta (0x9452ca, cdecl).
unsafe fn ita_ser_a_compare_schedule_all_stages(nome: &str, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32,
                             original: impl FnOnce() -> usize, copia: impl FnOnce(*mut i16, *mut i16, *mut i32) -> usize) -> usize {
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
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
            eng_cc_cup_written_round_bytes(a, *rodadas) == eng_cc_cup_written_round_bytes(b, *rodadas)
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
const CQO6_SERIE_A: [Cqo6Rodada; 34] = [
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
const CQO6_SERIE_B: [Cqo6Rodada; 38] = [
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
const CQO6_PRIMEIRA_FRANCESA: [(bool, Cqo6Rodada); 38] = [
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

const CQO6_LIGA_SERIE_A: Cqo6Liga = Cqo6Liga { rodadas: 0x22, arquivo: 0x9ea2a8, linha: 0xb0, tabela: &CQO6_SERIE_A, ramo: "cqo_6 calendario da serie a" };
const CQO6_LIGA_SERIE_B: Cqo6Liga = Cqo6Liga { rodadas: 0x26, arquivo: 0x9ea384, linha: 0xa8, tabela: &CQO6_SERIE_B, ramo: "cqo_6 calendario da serie b" };

/// O jogo-desempate das ligas italianas (fase 0, uma rodada de copa de 0x68 bytes, `eng_cc_cup_write_cup_round`; o ano +0x40 com
/// sinal): o sorteio e o jogo em maio do ano seguinte (a Série A: sorteio em 13/mai, jogo em 19/mai; a Série B: 15/mai e
/// 21/mai), nome 0xa0, sem sorteio (0x200), desempate do confronto 3, categoria 5, ida e volta (+0x21 = 2) com 3 dias
/// entre os jogos, sem prêmios; os times (2n, n jogos, 2n novos) pelo byte +0xee da liga (n, com sinal; as contas em 16
/// bits como a original) [inferência: os confrontos de desempate pedidos].
fn italian_playoff_round(sorteio: (u32, u32, u32, u32), jogo: (u32, u32, u32, u32, u32, u32), n: i8) -> Cql10R {
    let n = n as i16;
    let dois = n.wrapping_shl(1) as u16;
    cql10r(sorteio, jogo, 0xa0, (0, 0x200, 3), 5, (dois, n as u16, dois, 0), (0, 2, 3), (0, 0, 0))
}

/// O que muda entre a Série A e a Série B.
struct Cqo6Italia {
    liga: &'static Cqo6Liga,
    sorteio: (u32, u32, u32, u32),
    jogo: (u32, u32, u32, u32, u32, u32),
    linha_desempate: u32,
}

/// O corpo comum das duas (thiscall(liga, fase, &rodadas, &extra, &é_liga), ret 0x10; sem estado salvo): a fase −1 é a
/// principal (`build_league_schedule`); a 0, o jogo-desempate: é_liga (se pedido) = 0, 1 rodada, extra 0, a tabela de 0x68
/// bytes (malloc 0x944e46; sem memória: a caixa e 0); as outras devolvem 0 sem gravar nada. Devolve a tabela.
unsafe fn italian_schedule(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32, p: &Cqo6Italia) -> usize {
    match fase as u8 {
        0xff => build_league_schedule(this, fase, rodadas, extra, liga, p.liga),
        0 => {
            let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
            if !liga.is_null() {
                *liga = 0;
            }
            *rodadas = 1;
            *extra = 0;
            let t = alocar(0x68);
            if t == 0 {
                show_error_at(p.liga.arquivo, p.linha_desempate);
                return 0;
            }
            crate::comparacao::ramo("cqo_6 jogo-desempate italiano");
            let ano = core::ptr::read_unaligned((this + 0x40) as *const i16) as i32 as u32;
            eng_cc_cup_write_cup_round(t, 0, &italian_playoff_round(p.sorteio, p.jogo, *((this + 0xee) as *const i8)), ano);
            t
        }
        _ => 0,
    }
}

const CQO6_SERIE_A_E_DESEMPATE: Cqo6Italia = Cqo6Italia { liga: &CQO6_LIGA_SERIE_A, sorteio: (0xd, 4, 1, 0), jogo: (0x13, 4, 1, 6, 1, 0), linha_desempate: 0x226 };
const CQO6_SERIE_B_E_DESEMPATE: Cqo6Italia = Cqo6Italia { liga: &CQO6_LIGA_SERIE_B, sorteio: (0xf, 4, 1, 0), jogo: (0x15, 4, 1, 6, 1, 0), linha_desempate: 0x170 };

/// { 0x6409b0 } (thiscall, ret 0x10, ita_ser_a.cpp, 2.901 bytes, método +0x3c): `ItaSerA::create_schedule(fase,
/// &rodadas, &extra, &é_liga)`: a Série A, 34 rodadas (`CQO6_SERIE_A`), e o jogo-desempate. Fonte 0x9ea2a8, linhas 0xb0
/// e 0x226.
unsafe extern "thiscall" fn ita_ser_a_create_schedule(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    italian_schedule(this, fase, rodadas, extra, liga, &CQO6_SERIE_A_E_DESEMPATE)
}

unsafe extern "thiscall" fn comparar_ita_ser_a_create_schedule(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    ita_ser_a_compare_schedule_all_stages("ita_ser_a_create_schedule", fase, rodadas, extra, liga, || o(this, fase, rodadas, extra, liga), |r, e, l| ita_ser_a_create_schedule(this, fase, r, e, l))
}

/// { 0x645af0 } (thiscall, ret 0x10, ita_ser_b.cpp, 3.215 bytes, método +0x3c): `ItaSerB::create_schedule`: a Série B,
/// 38 rodadas (`CQO6_SERIE_B`), e o jogo-desempate. Fonte 0x9ea384, linhas 0xa8 e 0x170.
unsafe extern "thiscall" fn ita_ser_a_create_schedule_645af0(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    italian_schedule(this, fase, rodadas, extra, liga, &CQO6_SERIE_B_E_DESEMPATE)
}

unsafe extern "thiscall" fn comparar_ita_ser_a_create_schedule_645af0(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I1%%);
    ita_ser_a_compare_schedule_all_stages("ita_ser_a_create_schedule_645af0", fase, rodadas, extra, liga, || o(this, fase, rodadas, extra, liga), |r, e, l| ita_ser_a_create_schedule_645af0(this, fase, r, e, l))
}

/// { 0x5bb690 } (thiscall, ret 0x10, fra_first.cpp, 3.090 bytes, método +0x3c): `FraFirst::create_schedule(fase,
/// &rodadas, &extra, &é_liga)`: só a fase −1 (as outras devolvem 0). Em 2001 (o ano +0x40 = 0x7d1) 34 rodadas (18
/// clubes); nos outros anos 38 (20 clubes). é_liga (se pedido) = 1, as rodadas, extra 0, a tabela de rodadas × 0x41
/// bytes (sem memória: a caixa da linha 0xd1 e 0); as rodadas de `CQO6_PRIMEIRA_FRANCESA` em sequência (as marcadas só
/// entram com 38); no fim, se a conta não bate com as rodadas, a caixa da linha 0x1b2 (e devolve a tabela assim mesmo).
/// Fonte 0x9ccb5c.
unsafe extern "thiscall" fn fra_first_create_schedule(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    if fase as u8 != 0xff {
        return 0;
    }
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let ano = core::ptr::read_unaligned((this + 0x40) as *const u16);
    let n: i16 = if ano == 0x7d1 { 0x22 } else { 0x26 }; // cqo6: 34 rodadas em 2001, 38 depois
    if !liga.is_null() {
        *liga = 1;
    }
    *rodadas = n;
    *extra = 0;
    let t = alocar((n as i32 * 0x41) as u32);
    if t == 0 {
        show_error_at(0x9ccb5c, 0xd1);
        return 0;
    }
    crate::comparacao::ramo("cqo_6 calendario da primeira francesa");
    let mut i: i16 = 0;
    for (so_38, r) in CQO6_PRIMEIRA_FRANCESA.iter() {
        if *so_38 && n != 0x26 {
            continue;
        }
        ita_ser_a_set_league_rounds(t, ano as u32, i as i32, core::slice::from_ref(r));
        i += 1;
    }
    if i != n {
        show_error_at(0x9ccb5c, 0x1b2);
    }
    t
}

unsafe extern "thiscall" fn comparar_fra_first_create_schedule(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I2%%);
    ita_ser_a_compare_schedule("fra_first_create_schedule", fase, rodadas, extra, liga, || o(this, fase, rodadas, extra, liga), |r, e, l| fra_first_create_schedule(this, fase, r, e, l))
}
