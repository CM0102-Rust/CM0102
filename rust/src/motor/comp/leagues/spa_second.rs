use super::*;

// ---------- 280a leva: o calendário da Segunda espanhola: as 42 rodadas, uma tabela para o ano par e outra para o ímpar (spa_second.cpp) ----------


/// Uma rodada da tabela de 0x41 bytes: a data (dia, mês 0..11, anos a somar ao ano da liga +0x40, dia da semana para
/// onde a data é levada), o código do byte +4 ([inferência] o turno) e as faixas de TV em ordem (dia da semana, turno,
/// quantos jogos; a lista termina em −1).
pub(crate) type Cqq4Rodada = (i8, i8, i8, i8, i8, &'static [(i8, i8, i8)]);

/// Writes rounds starting at `primeira` into round table t: each round's date (0x68a160) and match slots (0x68a1c0),
/// from a static table.
///
/// [pt] As rodadas `primeira`.. da tabela `t`, na ordem da original: para cada uma, a data (0x68a160: push 0; push ano;
/// push código; push semana; push anos; push mês; push dia; push i; push t; add esp -> cdecl: o valor 0) e as
/// faixas (0x68a1c0: push 0; push c; push b; push a; push j; push i; push t; add esp -> cdecl, valor 0).
pub(crate) unsafe fn spa_second_set_league_rounds(t: usize, ano: u32, primeira: i32, tabela: &[Cqq4Rodada]) {
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
pub(crate) struct Cqq4Natal {
    pub(crate) comeco: &'static [Cqq4Rodada],
    pub(crate) referencia: usize,
    pub(crate) ajuste: i32,
    pub(crate) casos: [&'static [Cqq4Rodada]; 8],
    pub(crate) fim: &'static [Cqq4Rodada],
    pub(crate) linha: u32,
}

/// Writes the opening rounds, then picks the middle block by how many days a reference round falls after 1 December
/// (error box if out of range), then the closing rounds [inference: Christmas break placement].
pub(crate) unsafe fn set_rounds_with_christmas(t: usize, ano: u32, n: &Cqq4Natal, arquivo: usize) {
    let construir: unsafe extern "thiscall" fn(*mut [u16; 4], u32, u32, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE_549F70);
    spa_second_set_league_rounds(t, ano, 0, n.comeco);
    let mut d = [0u16; 4];
    construir(&mut d, 1, 0xb, ano, 0xffff_ffff);
    let dia = core::ptr::read_unaligned((t + n.referencia * 0x41) as *const u16);
    let k = (dia.wrapping_sub(d[0]).wrapping_add(1) as i16 as i32).wrapping_sub(n.ajuste);
    let caso = if k as u32 > 6 {
        show_error_at(arquivo, n.linha);
        7
    } else {
        k as usize
    };
    let meio = n.casos[caso];
    spa_second_set_league_rounds(t, ano, n.comeco.len() as i32, meio);
    spa_second_set_league_rounds(t, ano, (n.comeco.len() + meio.len()) as i32, n.fim);
}

/// Side-by-side comparer of create_schedule for every stage: outputs and written bytes (league rounds for -1, cup
/// rounds otherwise) must match; frees the port's table.
///
/// [pt] Lado a lado em todas as fases: a original com as saídas de quem chama, a cópia com saídas próprias, a partir do
/// mesmo [0xb67a34]; as saídas e os bytes gravados das duas tabelas têm de ser iguais (na fase −1 as rodadas de liga,
/// `written_league_round_bytes`; nas outras as de copa, `eng_cc_cup_written_round_bytes`); a tabela da cópia é solta (0x9452ca, cdecl).
pub(crate) unsafe fn spa_second_compare_schedule_all_stages(nome: &str, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32,
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
            written_league_round_bytes(a, a, n) == written_league_round_bytes(b, a, n)
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

/// a Segunda espanhola: as rodadas antes do Natal (ano par)
pub(crate) const CQQ4_SEGUNDA_PAR_COMECO: [Cqq4Rodada; 17] = [
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
pub(crate) const CQQ4_SEGUNDA_PAR_NATAL_0: [Cqq4Rodada; 1] = [
    (26, 11, 0, -1, 2, &[(-1, -1, -1)]), // 17: 26/dez
];

/// as rodadas do Natal quando a rodada 16 (a de referência) cai em 18/dez (ano par)
pub(crate) const CQQ4_SEGUNDA_PAR_NATAL_1: [Cqq4Rodada; 1] = [
    (26, 11, 0, -1, 2, &[(-1, -1, -1)]), // 17: 26/dez
];

/// as rodadas do Natal quando a rodada 16 (a de referência) cai em 19/dez (ano par)
pub(crate) const CQQ4_SEGUNDA_PAR_NATAL_2: [Cqq4Rodada; 1] = [
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 17: 26/dez
];

/// as rodadas do Natal quando a rodada 16 (a de referência) cai em 20/dez (ano par)
pub(crate) const CQQ4_SEGUNDA_PAR_NATAL_3: [Cqq4Rodada; 1] = [
    (27, 11, 0, -1, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 17: 27/dez
];

/// as rodadas do Natal quando a rodada 16 (a de referência) cai em 21/dez (ano par)
pub(crate) const CQQ4_SEGUNDA_PAR_NATAL_4: [Cqq4Rodada; 1] = [
    (28, 11, 0, -1, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 17: 28/dez
];

/// as rodadas do Natal quando a rodada 16 (a de referência) cai em 22/dez (ano par)
pub(crate) const CQQ4_SEGUNDA_PAR_NATAL_5: [Cqq4Rodada; 1] = [
    (29, 11, 0, -1, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 17: 29/dez
];

/// as rodadas do Natal quando a rodada 16 (a de referência) cai em 23/dez (ano par)
pub(crate) const CQQ4_SEGUNDA_PAR_NATAL_6: [Cqq4Rodada; 1] = [
    (30, 11, 0, -1, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 17: 30/dez
];

/// as rodadas do Natal no caso padrão (a rodada 16 fora de 17 a 23/dez: a caixa) (ano par)
pub(crate) const CQQ4_SEGUNDA_PAR_NATAL_PADRAO: [Cqq4Rodada; 1] = [
    (27, 11, 0, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 17: 27/dez
];

/// a Segunda espanhola: as rodadas depois do Natal (ano par)
pub(crate) const CQQ4_SEGUNDA_PAR_FIM: [Cqq4Rodada; 24] = [
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

/// a Segunda espanhola (impar)
pub(crate) const CQQ4_SEGUNDA_IMPAR_TODAS: [Cqq4Rodada; 42] = [
    (22, 7, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 0: 22/ago
    (29, 7, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 1: 29/ago
    (5, 8, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 2: 5/set
    (12, 8, 0, 6, 1, &[(-1, -1, -1)]), // 3: 12/set
    (19, 8, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 4: 19/set
    (26, 8, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 5: 26/set
    (3, 9, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 6: 3/out
    (10, 9, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 7: 10/out
    (13, 9, 0, 2, 2, &[(-1, -1, -1)]), // 8: 13/out
    (17, 9, 0, 6, 1, &[(-1, -1, -1)]), // 9: 17/out
    (24, 9, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 10: 24/out
    (31, 9, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 11: 31/out
    (7, 10, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 12: 7/nov
    (14, 10, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 13: 14/nov
    (21, 10, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 14: 21/nov
    (28, 10, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 15: 28/nov
    (5, 11, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 16: 5/dez
    (12, 11, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 17: 12/dez
    (19, 11, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 18: 19/dez
    (5, 0, 1, 2, 2, &[(-1, -1, -1)]), // 19: 5/jan (+1 ano)
    (9, 0, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 20: 9/jan (+1 ano)
    (16, 0, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 21: 16/jan (+1 ano)
    (23, 0, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 22: 23/jan (+1 ano)
    (30, 0, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 23: 30/jan (+1 ano)
    (6, 1, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 24: 6/fev (+1 ano)
    (13, 1, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 25: 13/fev (+1 ano)
    (20, 1, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 26: 20/fev (+1 ano)
    (27, 1, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 27: 27/fev (+1 ano)
    (5, 2, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 28: 5/mar (+1 ano)
    (12, 2, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 29: 12/mar (+1 ano)
    (19, 2, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 30: 19/mar (+1 ano)
    (26, 2, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 31: 26/mar (+1 ano)
    (2, 3, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 32: 2/abr (+1 ano)
    (9, 3, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 33: 9/abr (+1 ano)
    (16, 3, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 34: 16/abr (+1 ano)
    (23, 3, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 35: 23/abr (+1 ano)
    (30, 3, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 36: 30/abr (+1 ano)
    (7, 3, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 37: 7/abr (+1 ano)
    (14, 4, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 38: 14/mai (+1 ano)
    (21, 4, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 39: 21/mai (+1 ano)
    (28, 4, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 40: 28/mai (+1 ano)
    (4, 5, 1, 6, 1, &[(-1, -1, -1)]), // 41: 4/jun (+1 ano)
];

pub(crate) const CQQ4_SEGUNDA_PAR_NATAL: Cqq4Natal = Cqq4Natal { comeco: &CQQ4_SEGUNDA_PAR_COMECO, referencia: 16, ajuste: 17, casos: [&CQQ4_SEGUNDA_PAR_NATAL_0, &CQQ4_SEGUNDA_PAR_NATAL_1, &CQQ4_SEGUNDA_PAR_NATAL_2, &CQQ4_SEGUNDA_PAR_NATAL_3, &CQQ4_SEGUNDA_PAR_NATAL_4, &CQQ4_SEGUNDA_PAR_NATAL_5, &CQQ4_SEGUNDA_PAR_NATAL_6, &CQQ4_SEGUNDA_PAR_NATAL_PADRAO], fim: &CQQ4_SEGUNDA_PAR_FIM, linha: 0x146 };

/// the Segunda schedule: 42 rounds (22 clubs), nearly all on Sunday (day 6); in odd years (2001/02) a fixed table from
/// 22 August to 4 June, with one TV slot; in even years, from 30 August to 6 June, with three TV slots and one
/// fixed-date Christmas round chosen by the day the mid-December round falls on
/// Original: `SpaSecond::create_schedule` (0x853c90, spa_second.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x853c90 } (thiscall, ret 0x10, spa_second.cpp, 7.827 bytes, método +0x3c): `SpaSecond::create_schedule(fase,
/// &rodadas, &extra, &é_liga)`: só a fase −1, 42 rodadas, a tabela pela paridade do ano +0x40 (no ano par com a rodada
/// do Natal; no ímpar sem); as outras devolvem 0. Fonte 0xa829bc.
pub(crate) unsafe extern "thiscall" fn spa_second_create_schedule(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
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
                show_error_at(0xa828bc, 0xa9);
                return 0;
            }
            crate::comparacao::ramo("cqq_4 calendario da segunda espanhola");
            let ano = core::ptr::read_unaligned((this + 0x40) as *const u16) as u32;
            if (ano as i16) % 2 != 0 {
                spa_second_set_league_rounds(t, ano, 0, &CQQ4_SEGUNDA_IMPAR_TODAS);
            } else {
                set_rounds_with_christmas(t, ano, &CQQ4_SEGUNDA_PAR_NATAL, 0xa828bc);
            }
            t
        }
        _ => 0,
    }
}

pub(crate) unsafe extern "thiscall" fn comparar_spa_second_create_schedule(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_SPA_SECOND_CREATE_SCHEDULE);
    spa_second_compare_schedule_all_stages("spa_second_create_schedule", fase, rodadas, extra, liga, || o(this, fase, rodadas, extra, liga), |r, e, l| spa_second_create_schedule(this, fase, r, e, l))
}

/// the Spanish Segunda: discipline rule 0x1a (the Spanish one), only the league
/// Original: `SpaSecond::SpaSecond` (0x8539a0, spa_second.cpp, thiscall, ret 8).
///
/// [pt] { 0x8539a0 } (thiscall, ret 8, spa_second.cpp): `SpaSecond::SpaSecond(ano, registro)`: a Segunda espanhola,
/// disciplina 0x1a, só a liga.
pub(crate) unsafe extern "thiscall" fn construct_spa_second(this: usize, ano: u32, registro: usize) -> usize {
    construct_division_league(this, ano, registro, &CQM5_SEGUNDA)
}

pub(crate) unsafe extern "thiscall" fn comparar_construct_spa_second(this: usize, ano: u32, registro: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_SPA_SECOND);
    compare_division_league_ctor("construct_spa_second", this, ano, registro, &CQM5_SEGUNDA, || o(this, ano, registro))
}

/// La Segunda (22): the top 3 go up directly, the bottom 4 go down to Segunda B when it is loaded [inference from
/// nation bit 4]; B teams cannot go up (marked by the table, 0x855b40)
/// Original: `SpaSecond::set_up_rules` (0x855cc0, spa_second.cpp, thiscall, ret 0).
///
/// [pt] { 0x855cc0 } (thiscall, ret 0, spa_second.cpp): `SpaSecond::set_up_rules()`. A Segunda: as comuns, 2 turnos, 3
/// pontos, o 4º desempate 3, 3 sobem direto (+0xbe), nenhum play-off de acesso, 4 caem (+0xc1), a Primera acima
/// ([0x9cf624]) e abaixo a Segunda B ([0x9cf7b8]) quando a nação tem as divisões de baixo carregadas (bit 4 de +0x11c
/// da nação [inferência]), senão −1; +0x52 = 2.
pub(crate) unsafe extern "thiscall" fn spa_second_set_up_rules(this: usize) -> u32 {
    crate::comparacao::ramo("cqk_4 regras da segunda");
    spa_first_apply_league_rules(this, &segunda_rules(this))
}

pub(crate) unsafe extern "thiscall" fn comparar_spa_second_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SPA_SECOND_SET_UP_RULES);
    spa_first_compare_league_rules("spa_second_set_up_rules", this, segunda_rules(this), || o(this), || spa_second_set_up_rules(this))
}

/// the Segunda has 22 clubs (the competition's, in database order); the B team of a Primera club is marked as not
/// promotable (fate −4); the prize fund starts from 50,000
/// Original: `SpaSecond::create_table` (0x855b40, spa_second.cpp, thiscall, ret 0).
///
/// [pt] { 0x855b40 } (thiscall, ret 0, spa_second.cpp): `SpaSecond::create_table()`: a Segunda tem 22 clubes (+0x3e),
/// a tabela (malloc 0x944e46 de 0x512: push; add esp, 4 -> cdecl; sem memória, a caixa da linha 0x298 e 0) recebe, na
/// ordem do banco ([0xae23b4], [0xae235c] clubes de 0x245 bytes), os clubes da competição de id [0x9cf628] até encher
/// (League::init_table_row 0x687430: push 0; push 0; push clube; push linha (a palavra empurrada tem o byte da linha e o
/// lixo da pilha em cima; a 0x687430 só lê o byte); mov ecx, this -> thiscall, ret 0x10); o time B de um clube da
/// Primera ([0x9cf624]) leva o destino −4 (+0x37 = 0xfc: não pode subir). Cheia: o fundo de prêmios (+0x28 =
/// League::calc_prize_fund 0x682f70: push 50000; mov ecx -> thiscall, ret 4), +0xe1 = 1 e 1; senão 0.
pub(crate) unsafe extern "thiscall" fn spa_second_create_table(this: usize) -> u32 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let iniciar: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(FN_LEAGUE_INIT_TABLE_ROW);
    let premios: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_LEAGUE_CALC_PRIZE_FUND);
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 0x16);
    let t = alocar(0x512);
    write_u32_from_usize(this + 0xb1, t);
    if t == 0 {
        show_error_at(0xa828bc, 0x298);
        return 0;
    }
    let mut k: u8 = 0;
    let n = read_u32_as_usize(G_CLUB_COUNT) as i32;
    let base = read_u32_as_usize(G_CLUB_TABLE);
    let mut i: i16 = 0;
    while (i as i32) < n {
        let c = base + (i as u16 as usize) * 0x245; // cqm6: o clube i do banco
        if spa_lower_is_in_comp(c, KEY_CLUB_COMP_SPANISH_SECOND_DIVISION) {
            iniciar(this, k as u32, c, 0, 0);
            if is_b_team_of_comp_club(c, KEY_CLUB_COMP_SPANISH_FIRST_DIVISION) {
                crate::comparacao::ramo("cqm_6 time b na segunda");
                *((read_u32_as_usize(this + 0xb1) + (k as i8 as isize as usize).wrapping_mul(0x3b) + 0x37) as *mut u8) = 0xfc;
            }
            k = k.wrapping_add(1);
            if k as i8 as i16 == read_i16(this + 0x3e) {
                break;
            }
        }
        i = i.wrapping_add(1);
    }
    crate::comparacao::ramo("cqm_6 tabela da segunda");
    if k as i8 as i16 != read_i16(this + 0x3e) {
        return 0;
    }
    let f = premios(this, 0xc350);
    write_u32_from_usize(this + 0x28, f as usize);
    *((this + 0xe1) as *mut u8) = 1;
    1
}

/// Aloca: só a original. O plano, feito antes: os clubes da Segunda na ordem do banco (até 22) e a marca dos times B.
/// Depois: a devolução, +0x3e e as linhas (o clube e o destino −4 dos marcados).
pub(crate) unsafe extern "thiscall" fn comparar_spa_second_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SPA_SECOND_CREATE_TABLE);
    let n = read_u32_as_usize(G_CLUB_COUNT) as i32;
    let base = read_u32_as_usize(G_CLUB_TABLE);
    let mut plano: Vec<(usize, bool)> = Vec::new();
    for i in 0..n.max(0) as usize {
        let c = base + i * 0x245;
        if spa_lower_is_in_comp(c, KEY_CLUB_COMP_SPANISH_SECOND_DIVISION) {
            plano.push((c, is_b_team_of_comp_club(c, KEY_CLUB_COMP_SPANISH_FIRST_DIVISION)));
            if plano.len() == 0x16 {
                break;
            }
        }
    }
    let r = o(this);
    let t = read_u32_as_usize(this + 0xb1);
    let mut ok = r == (plano.len() == 0x16) as u32 && read_i16(this + 0x3e) == 0x16 && t != 0;
    if ok {
        for (k, &(c, b)) in plano.iter().enumerate() {
            let l = t + k * 0x3b;
            ok = ok && read_u32_as_usize(l) == c && (!b || *((l + 0x37) as *const u8) == 0xfc);
        }
    }
    PLACAR.registrar(ok, || format!("spa_second_create_table({:#x}): devolveu {}, plano {:x?}", this, r, plano));
    r
}
