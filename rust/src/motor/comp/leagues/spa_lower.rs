use super::*;

// ---------- 280a leva: o calendário da divisão de baixo espanhola (a Segunda B): as 42 rodadas, uma tabela para o ano par e outra para o ímpar, com a rodada do Natal (spa_lower.cpp) ----------


/// Uma rodada da tabela de 0x41 bytes: a data (dia, mês 0..11, anos a somar ao ano da liga +0x40, dia da semana para
/// onde a data é levada), o código do byte +4 ([inferência] o turno) e as faixas de TV em ordem (dia da semana, turno,
/// quantos jogos; a lista termina em −1).
pub(crate) type Cqq5Rodada = (i8, i8, i8, i8, i8, &'static [(i8, i8, i8)]);

/// As rodadas `primeira`.. da tabela `t`, na ordem da original: para cada uma, a data (0x68a160: push 0; push ano;
/// push código; push semana; push anos; push mês; push dia; push i; push t; add esp -> cdecl: o valor 0) e as
/// faixas (0x68a1c0: push 0; push c; push b; push a; push j; push i; push t; add esp -> cdecl, valor 0).
pub(crate) unsafe fn cqq5_rodadas(t: usize, ano: u32, primeira: i32, tabela: &[Cqq5Rodada]) {
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


/// A fase principal com as rodadas do Natal: as do começo em sequência; depois a data de 1º de dezembro do ano
/// (Date::Date 0x549f70: push −1; push ano; push 0xb; push 1; lea ecx, local -> thiscall, ret 0x10) e o caso pelo dia
/// da rodada de referência (+0, palavra; uma rodada de `casos` levada ao dia da semana): k = (dia da rodada − dia da
/// data + 1, em 16 bits) − `ajuste`; de 0 a 6 escolhe as rodadas do Natal; fora disso a caixa da linha `linha` e as do
/// caso padrão (o oitavo); depois as do fim.
pub(crate) struct Cqq5Natal {
    pub(crate) comeco: &'static [Cqq5Rodada],
    pub(crate) referencia: usize,
    pub(crate) ajuste: i32,
    pub(crate) casos: [&'static [Cqq5Rodada]; 8],
    pub(crate) fim: &'static [Cqq5Rodada],
    pub(crate) linha: u32,
}

pub(crate) unsafe fn cqq5_natal(t: usize, ano: u32, n: &Cqq5Natal, arquivo: usize) {
    let construir: unsafe extern "thiscall" fn(*mut [u16; 4], u32, u32, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE_549F70);
    cqq5_rodadas(t, ano, 0, n.comeco);
    let mut d = [0u16; 4];
    construir(&mut d, 1, 0xb, ano, 0xffff_ffff);
    let dia = core::ptr::read_unaligned((t + n.referencia * 0x41) as *const u16);
    let k = (dia.wrapping_sub(d[0]).wrapping_add(1) as i16 as i32).wrapping_sub(n.ajuste);
    let caso = if k as u32 > 6 {
        aviso_em(arquivo, n.linha);
        7
    } else {
        k as usize
    };
    let meio = n.casos[caso];
    cqq5_rodadas(t, ano, n.comeco.len() as i32, meio);
    cqq5_rodadas(t, ano, (n.comeco.len() + meio.len()) as i32, n.fim);
}

/// Lado a lado em todas as fases: a original com as saídas de quem chama, a cópia com saídas próprias, a partir do
/// mesmo [0xb67a34]; as saídas e os bytes gravados das duas tabelas têm de ser iguais (na fase −1 as rodadas de liga,
/// `cqo4_gravados`; nas outras as de copa, `cql10_gravados`); a tabela da cópia é solta (0x9452ca, cdecl).
pub(crate) unsafe fn cqq5_comparar(nome: &str, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32,
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
            cqo4_gravados(a, a, n) == cqo4_gravados(b, a, n)
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

/// a divisão de baixo espanhola: as rodadas antes do Natal (ano par)
pub(crate) const CQQ5_BAIXO_PAR_COMECO: [Cqq5Rodada; 17] = [
    (30, 7, 0, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 0: 30/ago
    (6, 8, 0, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 1: 6/set
    (13, 8, 0, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 2: 13/set
    (20, 8, 0, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 3: 20/set
    (27, 8, 0, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 4: 27/set
    (4, 9, 0, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 5: 4/out
    (11, 9, 0, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 6: 11/out
    (18, 9, 0, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 7: 18/out
    (25, 9, 0, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 8: 25/out
    (1, 10, 0, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 9: 1/nov
    (8, 10, 0, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 10: 8/nov
    (15, 10, 0, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 11: 15/nov
    (22, 10, 0, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 12: 22/nov
    (29, 10, 0, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 13: 29/nov
    (6, 11, 0, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 14: 6/dez
    (13, 11, 0, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 15: 13/dez
    (20, 11, 0, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 16: 20/dez
];

/// as rodadas do Natal quando a rodada 16 (a de referência) cai em 17/dez (ano par)
pub(crate) const CQQ5_BAIXO_PAR_NATAL_0: [Cqq5Rodada; 1] = [
    (26, 11, 0, -1, 2, &[(-1, -1, -1)]), // 17: 26/dez
];

/// as rodadas do Natal quando a rodada 16 (a de referência) cai em 18/dez (ano par)
pub(crate) const CQQ5_BAIXO_PAR_NATAL_1: [Cqq5Rodada; 1] = [
    (26, 11, 0, -1, 2, &[(-1, -1, -1)]), // 17: 26/dez
];

/// as rodadas do Natal quando a rodada 16 (a de referência) cai em 19/dez (ano par)
pub(crate) const CQQ5_BAIXO_PAR_NATAL_2: [Cqq5Rodada; 1] = [
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 17: 26/dez
];

/// as rodadas do Natal quando a rodada 16 (a de referência) cai em 20/dez (ano par)
pub(crate) const CQQ5_BAIXO_PAR_NATAL_3: [Cqq5Rodada; 1] = [
    (27, 11, 0, -1, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 17: 27/dez
];

/// as rodadas do Natal quando a rodada 16 (a de referência) cai em 21/dez (ano par)
pub(crate) const CQQ5_BAIXO_PAR_NATAL_4: [Cqq5Rodada; 1] = [
    (28, 11, 0, -1, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 17: 28/dez
];

/// as rodadas do Natal quando a rodada 16 (a de referência) cai em 22/dez (ano par)
pub(crate) const CQQ5_BAIXO_PAR_NATAL_5: [Cqq5Rodada; 1] = [
    (29, 11, 0, -1, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 17: 29/dez
];

/// as rodadas do Natal quando a rodada 16 (a de referência) cai em 23/dez (ano par)
pub(crate) const CQQ5_BAIXO_PAR_NATAL_6: [Cqq5Rodada; 1] = [
    (30, 11, 0, -1, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 17: 30/dez
];

/// as rodadas do Natal no caso padrão (a rodada 16 fora de 17 a 23/dez: a caixa) (ano par)
pub(crate) const CQQ5_BAIXO_PAR_NATAL_PADRAO: [Cqq5Rodada; 1] = [
    (27, 11, 0, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 17: 27/dez
];

/// a divisão de baixo espanhola: as rodadas depois do Natal (ano par)
pub(crate) const CQQ5_BAIXO_PAR_FIM: [Cqq5Rodada; 24] = [
    (3, 0, 1, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 18: 3/jan (+1 ano)
    (10, 0, 1, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 19: 10/jan (+1 ano)
    (17, 0, 1, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 20: 17/jan (+1 ano)
    (24, 0, 1, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 21: 24/jan (+1 ano)
    (31, 0, 1, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 22: 31/jan (+1 ano)
    (7, 1, 1, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 23: 7/fev (+1 ano)
    (14, 1, 1, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 24: 14/fev (+1 ano)
    (21, 1, 1, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 25: 21/fev (+1 ano)
    (28, 1, 1, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 26: 28/fev (+1 ano)
    (7, 2, 1, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 27: 7/mar (+1 ano)
    (14, 2, 1, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 28: 14/mar (+1 ano)
    (21, 2, 1, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 29: 21/mar (+1 ano)
    (28, 2, 1, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 30: 28/mar (+1 ano)
    (4, 3, 1, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 31: 4/abr (+1 ano)
    (11, 3, 1, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 32: 11/abr (+1 ano)
    (11, 3, 1, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 33: 11/abr (+1 ano)
    (18, 3, 1, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 34: 18/abr (+1 ano)
    (25, 3, 1, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 35: 25/abr (+1 ano)
    (2, 4, 1, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 36: 2/mai (+1 ano)
    (9, 4, 1, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 37: 9/mai (+1 ano)
    (16, 4, 1, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 38: 16/mai (+1 ano)
    (23, 4, 1, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 39: 23/mai (+1 ano)
    (30, 4, 1, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 40: 30/mai (+1 ano)
    (6, 5, 1, 6, 1, &[(-1, -1, -1)]), // 41: 6/jun (+1 ano)
];

/// a divisão de baixo espanhola: as rodadas antes do Natal (ano impar)
pub(crate) const CQQ5_BAIXO_IMPAR_COMECO: [Cqq5Rodada; 20] = [
    (16, 7, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 0: 16/ago
    (23, 7, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 1: 23/ago
    (26, 7, 0, 2, 2, &[(-1, -1, -1)]), // 2: 26/ago
    (30, 7, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 3: 30/ago
    (2, 8, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 4: 2/set
    (13, 8, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 5: 13/set
    (20, 8, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 6: 20/set
    (27, 8, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 7: 27/set
    (4, 9, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 8: 4/out
    (11, 9, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 9: 11/out
    (18, 9, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 10: 18/out
    (25, 9, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 11: 25/out
    (1, 10, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 12: 1/nov
    (8, 10, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 13: 8/nov
    (15, 10, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 14: 15/nov
    (22, 10, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 15: 22/nov
    (29, 10, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 16: 29/nov
    (6, 11, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 17: 6/dez
    (13, 11, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 18: 13/dez
    (20, 11, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 19: 20/dez
];

/// as rodadas do Natal quando a rodada 19 (a de referência) cai em 17/dez (ano impar)
pub(crate) const CQQ5_BAIXO_IMPAR_NATAL_0: [Cqq5Rodada; 1] = [
    (26, 11, 0, -1, 2, &[(-1, -1, -1)]), // 20: 26/dez
];

/// as rodadas do Natal quando a rodada 19 (a de referência) cai em 18/dez (ano impar)
pub(crate) const CQQ5_BAIXO_IMPAR_NATAL_1: [Cqq5Rodada; 1] = [
    (26, 11, 0, -1, 2, &[(-1, -1, -1)]), // 20: 26/dez
];

/// as rodadas do Natal quando a rodada 19 (a de referência) cai em 19/dez (ano impar)
pub(crate) const CQQ5_BAIXO_IMPAR_NATAL_2: [Cqq5Rodada; 1] = [
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 20: 26/dez
];

/// as rodadas do Natal quando a rodada 19 (a de referência) cai em 20/dez (ano impar)
pub(crate) const CQQ5_BAIXO_IMPAR_NATAL_3: [Cqq5Rodada; 1] = [
    (27, 11, 0, -1, 1, &[(5, 1, 1), (-1, -1, -1)]), // 20: 27/dez
];

/// as rodadas do Natal quando a rodada 19 (a de referência) cai em 21/dez (ano impar)
pub(crate) const CQQ5_BAIXO_IMPAR_NATAL_4: [Cqq5Rodada; 1] = [
    (28, 11, 0, -1, 1, &[(5, 1, 1), (-1, -1, -1)]), // 20: 28/dez
];

/// as rodadas do Natal quando a rodada 19 (a de referência) cai em 22/dez (ano impar)
pub(crate) const CQQ5_BAIXO_IMPAR_NATAL_5: [Cqq5Rodada; 1] = [
    (29, 11, 0, -1, 1, &[(5, 1, 1), (-1, -1, -1)]), // 20: 29/dez
];

/// as rodadas do Natal quando a rodada 19 (a de referência) cai em 23/dez (ano impar)
pub(crate) const CQQ5_BAIXO_IMPAR_NATAL_6: [Cqq5Rodada; 1] = [
    (30, 11, 0, -1, 1, &[(5, 1, 1), (-1, -1, -1)]), // 20: 30/dez
];

/// as rodadas do Natal no caso padrão (a rodada 19 fora de 17 a 23/dez: a caixa) (ano impar)
pub(crate) const CQQ5_BAIXO_IMPAR_NATAL_PADRAO: [Cqq5Rodada; 1] = [
    (27, 11, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 20: 27/dez
];

/// a divisão de baixo espanhola: as rodadas depois do Natal (ano impar)
pub(crate) const CQQ5_BAIXO_IMPAR_FIM: [Cqq5Rodada; 21] = [
    (3, 0, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 21: 3/jan (+1 ano)
    (10, 0, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 22: 10/jan (+1 ano)
    (17, 0, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 23: 17/jan (+1 ano)
    (24, 0, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 24: 24/jan (+1 ano)
    (31, 0, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 25: 31/jan (+1 ano)
    (7, 1, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 26: 7/fev (+1 ano)
    (14, 1, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 27: 14/fev (+1 ano)
    (21, 1, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 28: 21/fev (+1 ano)
    (28, 1, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 29: 28/fev (+1 ano)
    (7, 2, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 30: 7/mar (+1 ano)
    (14, 2, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 31: 14/mar (+1 ano)
    (21, 2, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 32: 21/mar (+1 ano)
    (28, 2, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 33: 28/mar (+1 ano)
    (4, 3, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 34: 4/abr (+1 ano)
    (11, 3, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 35: 11/abr (+1 ano)
    (11, 3, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 36: 11/abr (+1 ano)
    (18, 3, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 37: 18/abr (+1 ano)
    (25, 3, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 38: 25/abr (+1 ano)
    (2, 4, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 39: 2/mai (+1 ano)
    (9, 4, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 40: 9/mai (+1 ano)
    (16, 4, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 41: 16/mai (+1 ano)
];

pub(crate) const CQQ5_BAIXO_PAR_NATAL: Cqq5Natal = Cqq5Natal { comeco: &CQQ5_BAIXO_PAR_COMECO, referencia: 16, ajuste: 17, casos: [&CQQ5_BAIXO_PAR_NATAL_0, &CQQ5_BAIXO_PAR_NATAL_1, &CQQ5_BAIXO_PAR_NATAL_2, &CQQ5_BAIXO_PAR_NATAL_3, &CQQ5_BAIXO_PAR_NATAL_4, &CQQ5_BAIXO_PAR_NATAL_5, &CQQ5_BAIXO_PAR_NATAL_6, &CQQ5_BAIXO_PAR_NATAL_PADRAO], fim: &CQQ5_BAIXO_PAR_FIM, linha: 0x128 };
pub(crate) const CQQ5_BAIXO_IMPAR_NATAL: Cqq5Natal = Cqq5Natal { comeco: &CQQ5_BAIXO_IMPAR_COMECO, referencia: 19, ajuste: 17, casos: [&CQQ5_BAIXO_IMPAR_NATAL_0, &CQQ5_BAIXO_IMPAR_NATAL_1, &CQQ5_BAIXO_IMPAR_NATAL_2, &CQQ5_BAIXO_IMPAR_NATAL_3, &CQQ5_BAIXO_IMPAR_NATAL_4, &CQQ5_BAIXO_IMPAR_NATAL_5, &CQQ5_BAIXO_IMPAR_NATAL_6, &CQQ5_BAIXO_IMPAR_NATAL_PADRAO], fim: &CQQ5_BAIXO_IMPAR_FIM, linha: 0x237 };

/// the Spanish lower division schedule (background): 42 rounds, nearly all on Sunday (day 6); in odd years (2001/02)
/// from 16 August to 16 May, in even years from 30 August to 6 June; in each, one fixed-date Christmas round chosen by
/// the day the mid-December round falls on
/// Original: `SpaLower::create_schedule` (0x851260, spa_lower.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x851260 } (thiscall, ret 0x10, spa_lower.cpp, 8.495 bytes, método +0x3c): `SpaLower::create_schedule(fase,
/// &rodadas, &extra, &é_liga)`: só a fase −1, 42 rodadas, a tabela pela paridade do ano +0x40, cada uma com a rodada
/// do Natal; as outras devolvem 0. Fonte 0xa82980.
pub(crate) unsafe extern "thiscall" fn spa_lower_create_schedule(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    match fase as u8 {
        0xff => {
            if !liga.is_null() {
                *liga = 1;
            }
            *rodadas = 42;
            *extra = 0;
            let t = alocar(0xaaa);
            if t == 0 {
                aviso_em(0xa82880, 0x8b);
                return 0;
            }
            crate::comparacao::ramo("cqq_5 calendario da divisao de baixo espanhola");
            let ano = core::ptr::read_unaligned((this + 0x40) as *const u16) as u32;
            if (ano as i16) % 2 != 0 {
                cqq5_natal(t, ano, &CQQ5_BAIXO_IMPAR_NATAL, 0xa82880);
            } else {
                cqq5_natal(t, ano, &CQQ5_BAIXO_PAR_NATAL, 0xa82880);
            }
            t
        }
        _ => 0,
    }
}

pub(crate) unsafe extern "thiscall" fn comparar_spa_lower_create_schedule(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_SPA_LOWER_CREATE_SCHEDULE);
    cqq5_comparar("spa_lower_create_schedule", fase, rodadas, extra, liga, || o(this, fase, rodadas, extra, liga), |r, e, l| spa_lower_create_schedule(this, fase, r, e, l))
}

// ---------- 278a leva: a divisão de baixo espanhola (o construtor e a tabela com os times B e os melhores da Segunda B) e a tabela da Segunda espanhola (os times B que não sobem) (spa_lower.cpp, spa_second.cpp) ----------




/// O registro de competição de id [g] (vetor [0xae23d0], 0x6b bytes).
pub(crate) unsafe fn cqm6_registro(g: usize) -> usize {
    read_u32_as_usize(G_CLUB_COMP_TABLE).wrapping_add(read_u32_as_usize(g).wrapping_mul(0x6b))
}

/// A competição do clube (+0x57, o id no +0) é a de id [g]?
pub(crate) unsafe fn cqm6_na(c: usize, g: usize) -> bool {
    let comp = read_u32_as_usize(c + 0x57);
    comp != 0 && read_u32_as_usize(comp) == read_u32_as_usize(g)
}

/// the Spanish lower division is a background league with no discipline (0xff), in the competition of id [0x9cf6b0];
/// with no saved state, the rules, the table and the fixtures (no stats nor predicted positions)
/// Original: `SpaLower::SpaLower` (0x851020, spa_lower.cpp, thiscall, ret 4).
///
/// [pt] { 0x851020 } (thiscall, ret 4, spa_lower.cpp, quadro SEH só no caminho de exceção): `SpaLower::SpaLower(ano)`, a
/// divisão de baixo espanhola (criada pela Primera quando a nação não tem o bit 4 de +0x11c): a base (League::League
/// 0x682200: mov ecx -> thiscall, ret 0), a vtable 0x96fb1c, o ano (+0x40), a regra de disciplina 0xff (nenhuma) e o
/// registro da competição de id [0x9cf6b0] (+4). Tenta carregar o estado (0x687b10: push 1 -> thiscall, ret 4);
/// senão, só a liga (+0x44 = 0xff, +0x30 = −1, +0x2c = 0), as regras (0x853790), a tabela (0x8533d0) e os jogos
/// (League::create_fixtures 0x6835c0), todos mov ecx -> thiscall, ret 0; cada falha, a caixa da sua linha (0x3f,
/// 0x46, 0x4d do fonte 0xa82880) e fim. Devolve this.
pub(crate) unsafe extern "thiscall" fn construct_spa_lower(this: usize, ano: u32) -> usize {
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LEAGUE_LEAGUE);
    let carregar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_LEAGUE_LOAD_FROM_TEMP_FILE);
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_SPALOWER_SET_UP_RULES);
    let tabela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_SPALOWER_CREATE_TABLE);
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_LEAGUE_CREATE_FIXTURES);
    base(this);
    write_u32_from_usize(this, 0x96fb1c);
    core::ptr::write_unaligned((this + 0x40) as *mut u16, ano as u16);
    *((this + 0x50) as *mut u8) = 0xff;
    write_u32_from_usize(this + 4, cqm6_registro(KEY_CLUB_COMP_SPANISH_LOWER_DIVISION));
    if carregar(this, 1) != 0 {
        return this;
    }
    crate::comparacao::ramo("cqm_6 divisao de baixo nova");
    *((this + 0x44) as *mut u8) = 0xff;
    write_u32_from_usize(this + 0x30, 0xffff_ffff);
    write_u32_from_usize(this + 0x2c, 0);
    if regras(this) == 0 {
        aviso_em(0xa82880, 0x3f);
        return this;
    }
    if tabela(this) == 0 {
        aviso_em(0xa82880, 0x46);
        return this;
    }
    if jogos(this) == 0 {
        aviso_em(0xa82880, 0x4d);
    }
    this
}

/// Aloca e sorteia (pela tabela): só a original, conferindo a devolução, a vtable, o ano, a disciplina e o registro.
pub(crate) unsafe extern "thiscall" fn comparar_construct_spa_lower(this: usize, ano: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32) -> usize>(IDX_CONSTRUCT_SPA_LOWER);
    let r = o(this, ano);
    let ok = r == this && read_u32_as_usize(this) == 0x96fb1c && core::ptr::read_unaligned((this + 0x40) as *const u16) == ano as u16
        && *((this + 0x50) as *const u8) == 0xff && read_u32_as_usize(this + 4) == cqm6_registro(KEY_CLUB_COMP_SPANISH_LOWER_DIVISION);
    PLACAR.registrar(ok, || format!("construct_spa_lower({:#x}): devolveu {:#x}, vtable {:#x}, +4 = {:#x}", this, r, read_u32_as_usize(this), read_u32_as_usize(this + 4)));
    r
}

/// O clube de cima do clube `c` (club_get_linked_b_team 0x540a50: push 1; push &eh_a; push c; add esp, 0xc -> cdecl),
/// se `c` é o time B dele (eh_a = 0) e ele está na competição de id [g].
pub(crate) unsafe fn cqm6_time_b_de(c: usize, g: usize) -> bool {
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let mut eh_a: u32 = 0;
    let p = parceiro(c, &mut eh_a, 1);
    p != 0 && eh_a == 0 && cqm6_na(p, g)
}



/// Os candidatos da divisão de baixo espanhola, do registro local de 100 da original: os clubes (todos, na ordem do
/// banco, até 100) das competições de id [0x9cf62c], [0x9cf630], [0x9cf634], [0x9cf638] [inferência: os 4 grupos da
/// Segunda B] ou, se espanhóis, da de id [0x9cf764] [inferência: a divisão genérica de baixo], que não são time B nem
/// time A de ninguém (club_get_linked_b_team devolve 0).
pub(crate) unsafe fn cqm6_candidatos() -> Vec<usize> {
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let mut v = Vec::new();
    let n = read_u32_as_usize(G_CLUB_COUNT) as i32;
    let base = read_u32_as_usize(G_CLUB_TABLE);
    let mut i: i16 = 0;
    while (i as i32) < n && v.len() < 100 {
        let c = base + (i as u16 as usize) * 0x245;
        let comp = read_u32_as_usize(c + 0x57);
        let nacao = read_u32_as_usize(c + 0x53);
        if comp != 0 && nacao != 0 {
            let id = read_u32_as_usize(comp);
            let grupo = [KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B1, KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B2, KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B3, KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B4].iter().any(|&g| id == read_u32_as_usize(g));
            if grupo || (read_u32_as_usize(nacao) == read_u32_as_usize(KEY_NATION_SPAIN) && id == read_u32_as_usize(KEY_CLUB_COMP_A_LOWER_DIVISION)) {
                let mut eh_a: u32 = 0;
                if parceiro(c, &mut eh_a, 1) == 0 {
                    v.push(c);
                }
            }
        }
        i = i.wrapping_add(1);
    }
    v
}

/// Os times B espanhóis (fora da Primera e da Segunda) cujo clube de cima está na competição de id [g], na ordem do
/// banco, até 22 linhas (`k` já usadas). Os últimos `2 × [0xae2350]` clubes do banco ficam de fora [inferência: os
/// clubes criados depois, sem divisão].
pub(crate) unsafe fn cqm6_times_b(g: usize, k: &mut u8, linhas: i16, mut por: impl FnMut(usize, bool)) {
    let n = (read_u32_as_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_u32_as_usize(G_NATION_COUNT) as i32).wrapping_mul(2));
    let base = read_u32_as_usize(G_CLUB_TABLE);
    let mut i: i16 = 0;
    while (i as i32) < n {
        let c = base + (i as u16 as usize) * 0x245;
        let comp = read_u32_as_usize(c + 0x57);
        let nacao = read_u32_as_usize(c + 0x53);
        if comp != 0 && nacao != 0 && read_u32_as_usize(nacao) == read_u32_as_usize(KEY_NATION_SPAIN) && read_u32_as_usize(comp) != read_u32_as_usize(KEY_CLUB_COMP_SPANISH_FIRST_DIVISION) && read_u32_as_usize(comp) != read_u32_as_usize(KEY_CLUB_COMP_SPANISH_SECOND_DIVISION) {
            let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
            let mut eh_a: u32 = 0;
            let p = parceiro(c, &mut eh_a, 1);
            let entra = p != 0 && eh_a == 0 && read_u32_as_usize(p + 0x57) != 0 && (*k as i8 as i16) < linhas && read_u32_as_usize(read_u32_as_usize(p + 0x57)) == read_u32_as_usize(g);
            if entra {
                *k = k.wrapping_add(1);
            }
            por(c, entra);
        }
        i = i.wrapping_add(1);
    }
}

/// the lower division has 22 clubs: first the Spanish B teams whose parent club is in the Primera, then those of the
/// Segunda (database order); the rest come from the Segunda B clubs [inference: competitions [0x9cf62c..0x9cf638]] and
/// the Spanish generic division that are not B teams, ordered by reputation with the best shuffled; each club placed
/// keeps the competition at +0x60, but the Segunda B-team pass zeroes +0x60 of every Spanish side not entering it,
/// including the Primera ones already placed [observed in the comparison]
/// Original: `SpaLower::create_table` (0x8533d0, spa_lower.cpp, thiscall, ret 0).
///
/// [pt] { 0x8533d0 } (thiscall, ret 0, spa_lower.cpp, 947 bytes): `SpaLower::create_table()`: a divisão de baixo tem 22
/// clubes (+0x3e), a tabela (malloc de 0x512; sem memória, a caixa da linha 0x2ac e 0). Primeiro, os clubes que estão
/// nela (a competição de id [0x9cf6b0]) passam à de id [0x9cf764] [inferência: a divisão genérica]. Depois enche, na
/// ordem do banco: os times B espanhóis cujo clube de cima está na Primera, depois os da Segunda (cada um com +0x60 =
/// o registro desta competição; os outros times espanhóis de fora das duas, com +0x60 = 0); o resto com os
/// candidatos (cqm6_candidatos), ordenados pela reputação e com os 3 × faltam primeiros embaralhados
/// (comp_util_sort_and_shuffle_top_clubs 0x4afbd0: push faltam; push quantos; push lista; add esp, 0xc -> cdecl; sorteia),
/// cada um com +0x60 = o registro. As linhas: League::init_table_row 0x687430 (push 0; push 0; push clube; push eax
/// (al = a linha, os bits de cima são lixo de outra conta; a 0x687430 só lê o byte); mov ecx, this -> thiscall, ret
/// 0x10). Devolve se encheu.
pub(crate) unsafe extern "thiscall" fn spa_lower_create_table(this: usize) -> u32 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let iniciar: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(FN_LEAGUE_INIT_TABLE_ROW);
    let embaralhar: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(FN_COMP_UTIL_SORT_AND_SHUFFLE_TOP_CLUBS);
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 0x16);
    let t = alocar(0x512);
    write_u32_from_usize(this + 0xb1, t);
    if t == 0 {
        aviso_em(0xa82880, 0x2ac);
        return 0;
    }
    // os clubes desta divisão passam à genérica
    let n = (read_u32_as_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_u32_as_usize(G_NATION_COUNT) as i32).wrapping_mul(2));
    let base = read_u32_as_usize(G_CLUB_TABLE);
    let mut i: i16 = 0;
    while (i as i32) < n {
        let c = base + (i as u16 as usize) * 0x245;
        if cqm6_na(c, KEY_CLUB_COMP_SPANISH_LOWER_DIVISION) {
            write_u32_from_usize(c + 0x57, cqm6_registro(KEY_CLUB_COMP_A_LOWER_DIVISION));
        }
        i = i.wrapping_add(1);
    }
    let mut k: u8 = 0;
    for g in [KEY_CLUB_COMP_SPANISH_FIRST_DIVISION, KEY_CLUB_COMP_SPANISH_SECOND_DIVISION] { // cqm6: os times B da Primera e da Segunda
        let linhas = read_i16(this + 0x3e);
        // a conta das linhas para a condição anda junto com `k`
        let mut kk = k;
        cqm6_times_b(g, &mut kk, linhas, |c, entra| {
            if entra {
                iniciar(this, k as u32, c, 0, 0);
                k = k.wrapping_add(1);
                write_u32_from_usize(c + 0x60, read_u32_as_usize(this + 4));
            } else {
                write_u32_from_usize(c + 0x60, 0);
            }
        });
    }
    crate::comparacao::ramo("cqm_6 tabela da divisao de baixo");
    let mut lista = cqm6_candidatos();
    let quantos = lista.len() as u16 as u32;
    let faltam = (read_i16(this + 0x3e) as u16).wrapping_sub(k as i8 as i16 as u16) as u32; // cqm6: quantos faltam
    lista.resize(100, 0);
    embaralhar(lista.as_mut_ptr() as usize, quantos, faltam);
    let mut j: i16 = 0;
    while j < quantos as u16 as i16 && (k as i8 as i16) < read_i16(this + 0x3e) {
        let c = lista[j as usize];
        iniciar(this, k as u32, c, 0, 0);
        k = k.wrapping_add(1);
        write_u32_from_usize(c + 0x60, read_u32_as_usize(this + 4));
        j += 1;
    }
    (k as i8 as i16 == read_i16(this + 0x3e)) as u32
}

/// Aloca e sorteia: só a original. O plano, feito antes: os times B (da Primera e da Segunda) das primeiras linhas.
/// Depois: +0x3e e o clube das primeiras linhas. O +0x60 não entra: a segunda passada (a dos times B da Segunda) zera o
/// +0x60 de todo time espanhol que não entra nela, inclusive os da Primera que a primeira acabou de pôr na tabela
/// (comportamento da original, que a cópia repete; achado pela comparação de 8 ligas, 05/11/2026).
pub(crate) unsafe extern "thiscall" fn comparar_spa_lower_create_table(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SPA_LOWER_CREATE_TABLE);
    let mut plano: Vec<usize> = Vec::new();
    let mut k: u8 = 0;
    for g in [KEY_CLUB_COMP_SPANISH_FIRST_DIVISION, KEY_CLUB_COMP_SPANISH_SECOND_DIVISION] {
        cqm6_times_b(g, &mut k, 0x16, |c, entra| {
            if entra {
                plano.push(c);
            }
        });
    }
    let r = o(this);
    let t = read_u32_as_usize(this + 0xb1);
    let mut ok = read_i16(this + 0x3e) == 0x16 && t != 0;
    if ok {
        for (i, &c) in plano.iter().enumerate() {
            ok = ok && read_u32_as_usize(t + i * 0x3b) == c;
        }
    }
    PLACAR.registrar(ok, || format!("spa_lower_create_table({:#x}): devolveu {}, plano {:x?}, tabela {:x?}", this, r, plano,
        (0..plano.len()).map(|i| if t != 0 { read_u32_as_usize(t + i * 0x3b) } else { 0 }).collect::<Vec<_>>()));
    r
}

/// the lower division (22: B teams and the best minor clubs by reputation): 2 go up directly and 2 go to the promotion
/// play-off, 4 go down; the league above stored is La Primera (as in the original)
/// Original: `SpaLower::set_up_rules` (0x853790, spa_lower.cpp, thiscall, ret 0).
///
/// [pt] { 0x853790 } (thiscall, ret 0, spa_lower.cpp): `SpaLower::set_up_rules()`. A divisão de baixo (os times B e os
/// melhores das divisões menores): as comuns, 2 turnos, 3 pontos, 2 sobem direto (+0xbe) e 2 vão ao play-off de
/// acesso (+0xbf), 4 caem (+0xc1), a Primera como liga de cima ([0x9cf624], assim no original) e nenhuma abaixo.
pub(crate) unsafe extern "thiscall" fn spa_lower_set_up_rules(this: usize) -> u32 {
    crate::comparacao::ramo("cqk_4 regras da divisao de baixo");
    cqk4_regras(this, &cqk4_baixo())
}

pub(crate) unsafe extern "thiscall" fn comparar_spa_lower_set_up_rules(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SPA_LOWER_SET_UP_RULES);
    cqk4_comparar("spa_lower_set_up_rules", this, cqk4_baixo(), || o(this), || spa_lower_set_up_rules(this))
}
