use super::*;

// ---------- 279a leva: o calendário da segunda e da terceira divisões inglesas: as 46 rodadas com o Natal conforme o dia da semana e os play-offs de acesso (eng_second.cpp, eng_third.cpp) ----------


/// Uma rodada da tabela de 0x41 bytes: a data (dia, mês 0..11, anos a somar ao ano da liga +0x40, dia da semana para
/// onde a data é levada), o código do byte +4 ([inferência] o turno) e as faixas de TV em ordem (dia da semana, turno,
/// quantos jogos; a lista termina em −1).
pub(crate) type Cqp4Rodada = (i8, i8, i8, i8, i8, &'static [(i8, i8, i8)]);

/// Sets dates and match slots of the rounds from `primeira` in table t (League round set_date / set_match_slot).
///
/// [pt] As rodadas `primeira`.. da tabela `t`, na ordem da original: para cada uma, a data (0x68a160: push 0; push ano;
/// push código; push semana; push anos; push mês; push dia; push i; push t; add esp -> cdecl: o valor 0) e as
/// faixas (0x68a1c0: push 0; push c; push b; push a; push j; push i; push t; add esp -> cdecl, valor 0).
pub(crate) unsafe fn eng_second_set_league_rounds(t: usize, ano: u32, primeira: i32, tabela: &[Cqp4Rodada]) {
    let data: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, i32, u32, u32) -> usize = core::mem::transmute(FN_LEAGUE_ROUND_SET_DATE);
    let band: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, u32) = core::mem::transmute(FN_LEAGUE_ROUND_SET_MATCH_SLOT);
    for (k, &(dia, mes, anos, semana, codigo, faixas)) in tabela.iter().enumerate() {
        let i = primeira + k as i32;
        data(t, i, dia as i32, mes as i32, anos as i32, semana as i32, codigo as i32, ano, 0);
        for (j, &(a, b, c)) in faixas.iter().enumerate() {
            band(t, i, j as i32, a as i32, b as i32, c as i32, 0);
        }
    }
}


/// A fase principal com as rodadas do Natal: as do começo em sequência; depois a data de 1º de dezembro do ano
/// (Date::Date 0x549f70: push −1; push ano; push 0xb; push 1; lea ecx, local -> thiscall, ret 0x10) e o caso pelo dia
/// da rodada de referência (+0, palavra; uma rodada de `casos` levada ao dia da semana): k = (dia da rodada − dia da
/// data + 1, em 16 bits) − `ajuste`; de 0 a 6 escolhe as rodadas do Natal; fora disso a caixa da linha `linha` e as do
/// caso padrão (o oitavo); depois as do fim.
pub(crate) struct Cqp4Natal {
    pub(crate) comeco: &'static [Cqp4Rodada],
    pub(crate) referencia: usize,
    pub(crate) ajuste: i32,
    pub(crate) casos: [&'static [Cqp4Rodada]; 8],
    pub(crate) fim: &'static [Cqp4Rodada],
    pub(crate) linha: u32,
}

/// Sets the rounds around Christmas: start rounds, the middle block chosen by the weekday of the reference date, and
/// end rounds.
pub(crate) unsafe fn eng_second_set_christmas_rounds(t: usize, ano: u32, n: &Cqp4Natal, arquivo: usize) {
    let construir: unsafe extern "thiscall" fn(*mut [u16; 4], u32, u32, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE_549F70);
    eng_second_set_league_rounds(t, ano, 0, n.comeco);
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
    eng_second_set_league_rounds(t, ano, n.comeco.len() as i32, meio);
    eng_second_set_league_rounds(t, ano, (n.comeco.len() + meio.len()) as i32, n.fim);
}

/// Side-by-side comparer of a round table: outputs and written bytes must match; frees the copy's table.
///
/// [pt] Lado a lado em todas as fases: a original com as saídas de quem chama, a cópia com saídas próprias, a partir do
/// mesmo [0xb67a34]; as saídas e os bytes gravados das duas tabelas têm de ser iguais (na fase −1 as rodadas de liga,
/// `written_league_round_bytes`; nas outras as de copa, `eng_cc_cup_written_round_bytes`); a tabela da cópia é solta (0x9452ca, cdecl).
pub(crate) unsafe fn eng_second_compare_round_table(nome: &str, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32,
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

/// a segunda e a terceira divisões inglesas (as mesmas datas): as rodadas antes do Natal
pub(crate) const CQP4_SEGUNDA_COMECO: [Cqp4Rodada; 23] = [
    (12, 7, 0, 5, 1, &[(-1, -1, -1)]), // 0: 12/ago
    (19, 7, 0, 5, 1, &[(-1, -1, -1)]), // 1: 19/ago
    (26, 7, 0, 5, 1, &[(-1, -1, -1)]), // 2: 26/ago
    (28, 7, 0, 0, 2, &[(-1, -1, -1)]), // 3: 28/ago
    (2, 8, 0, 5, 1, &[(-1, -1, -1)]), // 4: 2/set
    (9, 8, 0, 5, 1, &[(-1, -1, -1)]), // 5: 9/set
    (13, 8, 0, 2, 2, &[(-1, -1, -1)]), // 6: 13/set
    (16, 8, 0, 5, 1, &[(-1, -1, -1)]), // 7: 16/set
    (23, 8, 0, 5, 1, &[(-1, -1, -1)]), // 8: 23/set
    (30, 8, 0, 5, 1, &[(-1, -1, -1)]), // 9: 30/set
    (7, 9, 0, 5, 1, &[(-1, -1, -1)]), // 10: 7/out
    (14, 9, 0, 5, 1, &[(-1, -1, -1)]), // 11: 14/out
    (17, 9, 0, 1, 2, &[(-1, -1, -1)]), // 12: 17/out
    (21, 9, 0, 5, 1, &[(-1, -1, -1)]), // 13: 21/out
    (24, 9, 0, 1, 2, &[(-1, -1, -1)]), // 14: 24/out
    (28, 9, 0, 5, 1, &[(-1, -1, -1)]), // 15: 28/out
    (4, 10, 0, 5, 1, &[(-1, -1, -1)]), // 16: 4/nov
    (11, 10, 0, 5, 1, &[(-1, -1, -1)]), // 17: 11/nov
    (18, 10, 0, 5, 1, &[(-1, -1, -1)]), // 18: 18/nov
    (25, 10, 0, 5, 1, &[(-1, -1, -1)]), // 19: 25/nov
    (2, 11, 0, 5, 1, &[(-1, -1, -1)]), // 20: 2/dez
    (9, 11, 0, 5, 1, &[(-1, -1, -1)]), // 21: 9/dez
    (16, 11, 0, 5, 1, &[(-1, -1, -1)]), // 22: 16/dez
];

/// as rodadas do Natal quando a rodada 22 (a de referência) cai em 13/dez
pub(crate) const CQP4_SEGUNDA_NATAL_0: [Cqp4Rodada; 4] = [
    (20, 11, 0, -1, 1, &[(-1, -1, -1)]), // 23: 20/dez
    (27, 11, 0, -1, 1, &[(-1, -1, -1)]), // 24: 27/dez
    (30, 11, 0, -1, 2, &[(-1, -1, -1)]), // 25: 30/dez
    (3, 0, 1, -1, 1, &[(-1, -1, -1)]), // 26: 3/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 22 (a de referência) cai em 14/dez
pub(crate) const CQP4_SEGUNDA_NATAL_1: [Cqp4Rodada; 4] = [
    (21, 11, 0, -1, 1, &[(-1, -1, -1)]), // 23: 21/dez
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 24: 26/dez
    (28, 11, 0, -1, 1, &[(-1, -1, -1)]), // 25: 28/dez
    (1, 0, 1, -1, 1, &[(-1, -1, -1)]), // 26: 1/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 22 (a de referência) cai em 15/dez
pub(crate) const CQP4_SEGUNDA_NATAL_2: [Cqp4Rodada; 4] = [
    (22, 11, 0, -1, 1, &[(-1, -1, -1)]), // 23: 22/dez
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 24: 26/dez
    (29, 11, 0, -1, 1, &[(-1, -1, -1)]), // 25: 29/dez
    (1, 0, 1, -1, 1, &[(-1, -1, -1)]), // 26: 1/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 22 (a de referência) cai em 16/dez
pub(crate) const CQP4_SEGUNDA_NATAL_3: [Cqp4Rodada; 4] = [
    (23, 11, 0, -1, 1, &[(-1, -1, -1)]), // 23: 23/dez
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 24: 26/dez
    (30, 11, 0, -1, 1, &[(-1, -1, -1)]), // 25: 30/dez
    (1, 0, 1, -1, 1, &[(-1, -1, -1)]), // 26: 1/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 22 (a de referência) cai em 17/dez
pub(crate) const CQP4_SEGUNDA_NATAL_4: [Cqp4Rodada; 4] = [
    (21, 11, 0, -1, 2, &[(-1, -1, -1)]), // 23: 21/dez
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 24: 26/dez
    (31, 11, 0, -1, 1, &[(-1, -1, -1)]), // 25: 31/dez
    (4, 0, 1, -1, 2, &[(-1, -1, -1)]), // 26: 4/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 22 (a de referência) cai em 18/dez
pub(crate) const CQP4_SEGUNDA_NATAL_5: [Cqp4Rodada; 4] = [
    (22, 11, 0, -1, 2, &[(-1, -1, -1)]), // 23: 22/dez
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 24: 26/dez
    (28, 11, 0, -1, 2, &[(-1, -1, -1)]), // 25: 28/dez
    (1, 0, 1, -1, 1, &[(-1, -1, -1)]), // 26: 1/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 22 (a de referência) cai em 19/dez
pub(crate) const CQP4_SEGUNDA_NATAL_6: [Cqp4Rodada; 4] = [
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 23: 26/dez
    (28, 11, 0, -1, 2, &[(-1, -1, -1)]), // 24: 28/dez
    (2, 0, 1, -1, 1, &[(-1, -1, -1)]), // 25: 2/jan (+1 ano)
    (5, 0, 1, -1, 2, &[(-1, -1, -1)]), // 26: 5/jan (+1 ano)
];

/// as rodadas do Natal no caso padrão (a rodada 22 fora de 13 a 19/dez: a caixa)
pub(crate) const CQP4_SEGUNDA_NATAL_PADRAO: [Cqp4Rodada; 4] = [
    (26, 11, 0, 5, 1, &[(-1, -1, -1)]), // 23: 26/dez
    (28, 11, 0, 0, 2, &[(-1, -1, -1)]), // 24: 28/dez
    (2, 0, 1, -1, 1, &[(-1, -1, -1)]), // 25: 2/jan (+1 ano)
    (5, 0, 1, -1, 2, &[(-1, -1, -1)]), // 26: 5/jan (+1 ano)
];

/// a segunda e a terceira divisões inglesas (as mesmas datas): as rodadas depois do Natal
pub(crate) const CQP4_SEGUNDA_FIM: [Cqp4Rodada; 19] = [
    (13, 0, 1, 5, 1, &[(-1, -1, -1)]), // 27: 13/jan (+1 ano)
    (20, 0, 1, 5, 1, &[(-1, -1, -1)]), // 28: 20/jan (+1 ano)
    (3, 1, 1, 5, 1, &[(-1, -1, -1)]), // 29: 3/fev (+1 ano)
    (10, 1, 1, 5, 1, &[(-1, -1, -1)]), // 30: 10/fev (+1 ano)
    (17, 1, 1, 5, 1, &[(-1, -1, -1)]), // 31: 17/fev (+1 ano)
    (20, 1, 1, 1, 2, &[(-1, -1, -1)]), // 32: 20/fev (+1 ano)
    (24, 1, 1, 5, 1, &[(-1, -1, -1)]), // 33: 24/fev (+1 ano)
    (3, 2, 1, 5, 1, &[(-1, -1, -1)]), // 34: 3/mar (+1 ano)
    (7, 2, 1, 2, 2, &[(-1, -1, -1)]), // 35: 7/mar (+1 ano)
    (10, 2, 1, 5, 1, &[(-1, -1, -1)]), // 36: 10/mar (+1 ano)
    (17, 2, 1, 1, 2, &[(-1, -1, -1)]), // 37: 17/mar (+1 ano)
    (24, 2, 1, 5, 1, &[(-1, -1, -1)]), // 38: 24/mar (+1 ano)
    (31, 2, 1, 5, 1, &[(-1, -1, -1)]), // 39: 31/mar (+1 ano)
    (7, 3, 1, 5, 1, &[(-1, -1, -1)]), // 40: 7/abr (+1 ano)
    (14, 3, 1, 5, 1, &[(-1, -1, -1)]), // 41: 14/abr (+1 ano)
    (16, 3, 1, 0, 2, &[(-1, -1, -1)]), // 42: 16/abr (+1 ano)
    (21, 3, 1, 5, 1, &[(-1, -1, -1)]), // 43: 21/abr (+1 ano)
    (28, 3, 1, 5, 1, &[(-1, -1, -1)]), // 44: 28/abr (+1 ano)
    (6, 4, 1, 6, 1, &[(-1, -1, -1)]), // 45: 6/mai (+1 ano)
];

/// as rodadas do Natal quando a rodada 22 (a de referência) cai em 13/dez
pub(crate) const CQP4_TERCEIRA_NATAL_0: [Cqp4Rodada; 4] = [
    (20, 11, 0, -1, 1, &[(-1, -1, -1)]), // 23: 20/dez
    (27, 11, 0, -1, 1, &[(-1, -1, -1)]), // 24: 27/dez
    (30, 11, 0, -1, 2, &[(-1, -1, -1)]), // 25: 30/dez
    (3, 0, 1, -1, 1, &[(-1, -1, -1)]), // 26: 3/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 22 (a de referência) cai em 14/dez
pub(crate) const CQP4_TERCEIRA_NATAL_1: [Cqp4Rodada; 4] = [
    (21, 11, 0, -1, 1, &[(-1, -1, -1)]), // 23: 21/dez
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 24: 26/dez
    (28, 11, 0, -1, 1, &[(-1, -1, -1)]), // 25: 28/dez
    (1, 0, 1, -1, 1, &[(-1, -1, -1)]), // 26: 1/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 22 (a de referência) cai em 15/dez
pub(crate) const CQP4_TERCEIRA_NATAL_2: [Cqp4Rodada; 4] = [
    (22, 11, 0, -1, 1, &[(-1, -1, -1)]), // 23: 22/dez
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 24: 26/dez
    (29, 11, 0, -1, 1, &[(-1, -1, -1)]), // 25: 29/dez
    (1, 0, 1, -1, 1, &[(-1, -1, -1)]), // 26: 1/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 22 (a de referência) cai em 16/dez
pub(crate) const CQP4_TERCEIRA_NATAL_3: [Cqp4Rodada; 4] = [
    (23, 11, 0, -1, 1, &[(-1, -1, -1)]), // 23: 23/dez
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 24: 26/dez
    (30, 11, 0, -1, 1, &[(-1, -1, -1)]), // 25: 30/dez
    (1, 0, 1, -1, 1, &[(-1, -1, -1)]), // 26: 1/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 22 (a de referência) cai em 17/dez
pub(crate) const CQP4_TERCEIRA_NATAL_4: [Cqp4Rodada; 4] = [
    (21, 11, 0, -1, 2, &[(-1, -1, -1)]), // 23: 21/dez
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 24: 26/dez
    (31, 11, 0, -1, 1, &[(-1, -1, -1)]), // 25: 31/dez
    (4, 0, 1, -1, 2, &[(-1, -1, -1)]), // 26: 4/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 22 (a de referência) cai em 18/dez
pub(crate) const CQP4_TERCEIRA_NATAL_5: [Cqp4Rodada; 4] = [
    (22, 11, 0, -1, 2, &[(-1, -1, -1)]), // 23: 22/dez
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 24: 26/dez
    (28, 11, 0, -1, 2, &[(-1, -1, -1)]), // 25: 28/dez
    (1, 0, 1, -1, 1, &[(-1, -1, -1)]), // 26: 1/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 22 (a de referência) cai em 19/dez
pub(crate) const CQP4_TERCEIRA_NATAL_6: [Cqp4Rodada; 4] = [
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 23: 26/dez
    (28, 11, 0, -1, 2, &[(-1, -1, -1)]), // 24: 28/dez
    (2, 0, 1, -1, 1, &[(-1, -1, -1)]), // 25: 2/jan (+1 ano)
    (5, 0, 1, -1, 2, &[(-1, -1, -1)]), // 26: 5/jan (+1 ano)
];

/// as rodadas do Natal no caso padrão (a rodada 22 fora de 13 a 19/dez: a caixa)
pub(crate) const CQP4_TERCEIRA_NATAL_PADRAO: [Cqp4Rodada; 4] = [
    (26, 11, 0, 5, 1, &[(-1, -1, -1)]), // 23: 26/dez
    (28, 11, 0, 0, 2, &[(-1, -1, -1)]), // 24: 28/dez
    (2, 0, 1, -1, 1, &[(-1, -1, -1)]), // 25: 2/jan (+1 ano)
    (5, 0, 1, -1, 2, &[(-1, -1, -1)]), // 26: 5/jan (+1 ano)
];

pub(crate) const CQP4_SEGUNDA_NATAL: Cqp4Natal = Cqp4Natal { comeco: &CQP4_SEGUNDA_COMECO, referencia: 22, ajuste: 13, casos: [&CQP4_SEGUNDA_NATAL_0, &CQP4_SEGUNDA_NATAL_1, &CQP4_SEGUNDA_NATAL_2, &CQP4_SEGUNDA_NATAL_3, &CQP4_SEGUNDA_NATAL_4, &CQP4_SEGUNDA_NATAL_5, &CQP4_SEGUNDA_NATAL_6, &CQP4_SEGUNDA_NATAL_PADRAO], fim: &CQP4_SEGUNDA_FIM, linha: 0x15c };
pub(crate) const CQP4_SEGUNDA_PLAYOFF_0: Cql10R = cql10r((7, 4, 1, 0), (0xc, 4, 1, 5, 1, 0), 0x82, (0, 0x204, 3), 5, (4, 2, 4, 0), (0, 2, 4), (0, 0, 0));
pub(crate) const CQP4_SEGUNDA_PLAYOFF_1: Cql10R = cql10r((0x11, 4, 1, 3), (0x1b, 4, 1, 6, 1, 4), 0x96, (0, 3, 0), 5, (2, 1, 0, 0), (0, 1, 0), (0, 0, 0));
pub(crate) const CQP4_TERCEIRA_NATAL: Cqp4Natal = Cqp4Natal { comeco: &CQP4_SEGUNDA_COMECO, referencia: 22, ajuste: 13, casos: [&CQP4_TERCEIRA_NATAL_0, &CQP4_TERCEIRA_NATAL_1, &CQP4_TERCEIRA_NATAL_2, &CQP4_TERCEIRA_NATAL_3, &CQP4_TERCEIRA_NATAL_4, &CQP4_TERCEIRA_NATAL_5, &CQP4_TERCEIRA_NATAL_6, &CQP4_TERCEIRA_NATAL_PADRAO], fim: &CQP4_SEGUNDA_FIM, linha: 0x161 };
pub(crate) const CQP4_TERCEIRA_PLAYOFF_0: Cql10R = cql10r((7, 4, 1, 0), (0xc, 4, 1, 5, 1, 0), 0x82, (0, 0x204, 3), 5, (4, 2, 4, 0), (0, 2, 4), (0, 0, 0));
pub(crate) const CQP4_TERCEIRA_PLAYOFF_1: Cql10R = cql10r((0x11, 4, 1, 3), (0x1a, 4, 1, 5, 1, 4), 0x96, (0, 3, 0), 5, (2, 1, 0, 0), (0, 1, 0), (0, 0, 0));

/// the Division Two schedule: 46 rounds (24 clubs), from 12 August to 6 May, nearly all on the same weekday (5) and
/// some midweek; at Christmas, four fixed-date rounds (20 December to 5 January) chosen by the day the 16 December
/// round falls on. The promotion play-offs (stage 0): two-legged semi-finals (4 clubs, draw on 7 May, first leg on 12,
/// 4 days apart) and a one-off final on 27 May [inference: venue 4 is the neutral ground]
/// Original: `EngSecond::create_schedule` (0x5757a0, eng_second.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x5757a0 } (thiscall, ret 0x10, eng_second.cpp, 3.692 bytes, método +0x3c): `EngSecond::create_schedule(fase,
/// &rodadas, &extra, &é_liga)`: a fase −1, 46 rodadas (`CQP4_SEGUNDA_NATAL`); a fase 0, os play-offs (semifinais e
/// final); as outras devolvem 0. Fonte 0x9c43e8.
pub(crate) unsafe extern "thiscall" fn eng_second_create_schedule(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    match fase as u8 {
        0xff => {
            if !liga.is_null() {
                *liga = 1;
            }
            *rodadas = 46;
            *extra = 0;
            let t = alocar(0xbae);
            if t == 0 {
                show_error_at(0x9c43e8, 0xae);
                return 0;
            }
            crate::comparacao::ramo("cqp_4 calendario da segunda inglesa");
            let ano = core::ptr::read_unaligned((this + 0x40) as *const u16) as u32;
            eng_second_set_christmas_rounds(t, ano, &CQP4_SEGUNDA_NATAL, 0x9c43e8);
            t
        }
        0 => {
            if !liga.is_null() {
                *liga = 0;
            }
            *rodadas = 2;
            *extra = 0xa0;
            let t = alocar(0xd0);
            if t == 0 {
                show_error_at(0x9c43e8, 0x1b5);
                return 0;
            }
            crate::comparacao::ramo("cqp_4 play-offs da segunda inglesa");
            let ano = core::ptr::read_unaligned((this + 0x40) as *const i16) as i32 as u32;
            eng_cc_cup_write_cup_round(t, 0, &CQP4_SEGUNDA_PLAYOFF_0, ano);
            eng_cc_cup_write_cup_round(t, 1, &CQP4_SEGUNDA_PLAYOFF_1, ano);
            t
        }
        _ => 0,
    }
}

pub(crate) unsafe extern "thiscall" fn comparar_eng_second_create_schedule(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_ENG_SECOND_CREATE_SCHEDULE);
    eng_second_compare_round_table("eng_second_create_schedule", fase, rodadas, extra, liga, || o(this, fase, rodadas, extra, liga), |r, e, l| eng_second_create_schedule(this, fase, r, e, l))
}

/// the Division Three schedule: the same 46 rounds as Division Two. The promotion play-offs (stage 0): two-legged
/// semi-finals (4 clubs, draw on 7 May, first leg on 12, 4 days apart) and a one-off final on 26 May [inference: venue
/// 4 is the neutral ground]
/// Original: `EngThird::create_schedule` (0x5770e0, eng_third.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x5770e0 } (thiscall, ret 0x10, eng_third.cpp, 3.692 bytes, método +0x3c): `EngThird::create_schedule`: o mesmo
/// código da segunda (as mesmas rodadas), com a data da final dos play-offs um dia antes. Fonte 0x9c4424.
pub(crate) unsafe extern "thiscall" fn eng_second_create_schedule_5770e0(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    match fase as u8 {
        0xff => {
            if !liga.is_null() {
                *liga = 1;
            }
            *rodadas = 46;
            *extra = 0;
            let t = alocar(0xbae);
            if t == 0 {
                show_error_at(0x9c4424, 0xb3);
                return 0;
            }
            crate::comparacao::ramo("cqp_4 calendario da terceira inglesa");
            let ano = core::ptr::read_unaligned((this + 0x40) as *const u16) as u32;
            eng_second_set_christmas_rounds(t, ano, &CQP4_TERCEIRA_NATAL, 0x9c4424);
            t
        }
        0 => {
            if !liga.is_null() {
                *liga = 0;
            }
            *rodadas = 2;
            *extra = 0xa0;
            let t = alocar(0xd0);
            if t == 0 {
                show_error_at(0x9c4424, 0x1ba);
                return 0;
            }
            crate::comparacao::ramo("cqp_4 play-offs da terceira inglesa");
            let ano = core::ptr::read_unaligned((this + 0x40) as *const i16) as i32 as u32;
            eng_cc_cup_write_cup_round(t, 0, &CQP4_TERCEIRA_PLAYOFF_0, ano);
            eng_cc_cup_write_cup_round(t, 1, &CQP4_TERCEIRA_PLAYOFF_1, ano);
            t
        }
        _ => 0,
    }
}

pub(crate) unsafe extern "thiscall" fn comparar_eng_second_create_schedule_5770e0(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_ENG_SECOND_CREATE_SCHEDULE_5770E0);
    eng_second_compare_round_table("eng_second_create_schedule_5770e0", fase, rodadas, extra, liga, || o(this, fase, rodadas, extra, liga), |r, e, l| eng_second_create_schedule_5770e0(this, fase, r, e, l))
}

// ---------- 262a leva: as divisões inglesas no fim da temporada: os registros pela situação na tabela e no mata-mata, e a situação do clube na First e na Third (eng_second.cpp, eng_first.cpp, eng_third.cpp) ----------



/// The Staff_Records object (G_STAFF_RECORDS).
///
/// [pt] O registro de staff ([0xdd7f00]).
pub(crate) unsafe fn eng_second_staff_records() -> usize {
    read_usize(G_STAFF_RECORDS)
}

/// in the league (stage 0xff): title (5) 0x868c50, promotion (0) 0x869480 with weight 0x64, play-off place (1) 0x86bdd0
/// (0xa0, 0x82, 0x1e), relegation (3) 0x86a1c0; in the play-offs, on the club row: won (1) promotion with 0x32, +0x37 =
/// 0 and removed = 1; went through (0) 0x86bdd0 (0xa0, 0x96, 0xf); out (0xff, 2) +0x37 = 0xfe; returns 0
/// Original: `EngSecond::update_staff_records_for_status` (0x576c50, eng_second.cpp, thiscall, ret 0x18).
///
/// [pt] { 0x576c50 } (thiscall, ret 0x18, eng_second.cpp): `EngSecond::update_staff_records_for_status(clube, situação,
/// fase, &saiu, _, _)`, a virtual +0x44 da First, da Second e da Third (anunciada pela 0x686670). Devolve 0.
/// Na liga (fase 0xff), pela situação: 5, o título (Staff_Records::add_league_champions_record 0x868c50,
/// thiscall(registros, clube, comp +4), ret 8); 0, o acesso (0x869480, thiscall(registros, clube, comp, 0x64), ret
/// 0xc); 1, a vaga no mata-mata (0x86bdd0, thiscall(registros, clube, comp, 0xa0, 0x82, 0x1e), ret 0x14); 3, o
/// rebaixamento (0x86a1c0, thiscall(registros, clube, comp), ret 8). No mata-mata (outra fase), na linha do clube da
/// tabela (+0xb1, 0x3b bytes, +0x3e linhas): 1 (venceu a final), o acesso com peso 0x32, +0x37 = 0 e saiu = 1; 0
/// (passou da rodada), a classificação (0xa0, 0x96, 0xf) e segue procurando; 0xff ou 2 (caiu), +0x37 = 0xfe.
pub(crate) unsafe extern "thiscall" fn eng_second_update_staff_records_for_status(this: usize, clube: usize, situacao: u32, fase: u32, saiu: usize, _a: u32, _b: u32) -> u32 {
    let s = situacao as u8;
    let comp = read_usize(this + 4);
    if fase as u8 == 0xff {
        match s {
            5 => {
                let titulo: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_STAFF_RECORDS_ADD_LEAGUE_CHAMPIONS_RECORD);
                titulo(eng_second_staff_records(), clube, comp);
            }
            0 => {
                crate::comparacao::ramo("cpz_2 acesso na tabela");
                let acesso: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_STAFF_RECORDS_ADD_PROMOTION_RECORD);
                acesso(eng_second_staff_records(), clube, comp, 0x64); // cpz2: o acesso pela tabela pesa 0x64
            }
            1 => {
                crate::comparacao::ramo("cpz_2 vaga no mata-mata");
                let vaga: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(FN_STAFF_RECORDS_ADD_QUALIFIED_FOR_ROUND_RECORD);
                vaga(eng_second_staff_records(), clube, comp, 0xa0, 0x82, 0x1e); // cpz2: a vaga no mata-mata pesa 0x1e
            }
            3 => {
                crate::comparacao::ramo("cpz_2 rebaixamento na tabela");
                let cair: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_STAFF_RECORDS_ADD_RELEGATION_RECORD);
                cair(eng_second_staff_records(), clube, comp); // cpz2: o rebaixamento pela tabela
            }
            _ => {}
        }
        return 0;
    }
    let mut i: i32 = 0;
    while i < *((this + 0x3e) as *const i16) as i32 {
        let linha = read_usize(this + 0xb1) + (i as usize) * 0x3b;
        if read_usize(linha) == clube {
            match s {
                1 => {
                    crate::comparacao::ramo("cpz_2 venceu o mata-mata");
                    let acesso: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_STAFF_RECORDS_ADD_PROMOTION_RECORD);
                    acesso(eng_second_staff_records(), clube, comp, 0x32);
                    *((read_usize(this + 0xb1) + (i as usize) * 0x3b + 0x37) as *mut u8) = 0;
                    *(saiu as *mut u8) = 1;
                    return 0;
                }
                0 => {
                    let vaga: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(FN_STAFF_RECORDS_ADD_QUALIFIED_FOR_ROUND_RECORD);
                    vaga(eng_second_staff_records(), clube, comp, 0xa0, 0x96, 0xf);
                }
                0xff | 2 => {
                    *((read_usize(this + 0xb1) + (i as usize) * 0x3b + 0x37) as *mut u8) = 0xfe;
                    return 0;
                }
                _ => {}
            }
        }
        i += 1;
    }
    0
}

/// Comparador da 0x576c50: os registros gravam o histórico no arquivo, sorteiam o moral e podem renovar contratos
/// (não rodam duas vezes): só a original. O plano, feito antes, no mata-mata: a primeira linha do clube decide o que a
/// rotina grava (+0x37 = 0 e saiu = 1 com 1; +0x37 = 0xfe com 0xff ou 2; nada com 0, que segue procurando outra linha
/// do mesmo clube); depois a linha e `saiu` têm de estar assim. Na liga a rotina não grava nada: a devolução.
pub(crate) unsafe extern "thiscall" fn comparar_eng_second_update_staff_records_for_status(this: usize, clube: usize, situacao: u32, fase: u32, saiu: usize, a: u32, b: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32, u32) -> u32>(IDX_ENG_SECOND_UPDATE_STAFF_RECORDS_FOR_STATUS);
    let s = situacao as u8;
    let mut plano: Option<(usize, u8, bool)> = None;
    if fase as u8 != 0xff {
        let tab = read_usize(this + 0xb1);
        let n = *((this + 0x3e) as *const i16) as i32;
        let mut i: i32 = 0;
        while i < n {
            let linha = tab + (i as usize) * 0x3b;
            if read_usize(linha) == clube {
                match s {
                    1 => {
                        plano = Some((linha, 0, true));
                        break;
                    }
                    0xff | 2 => {
                        plano = Some((linha, 0xfe, false));
                        break;
                    }
                    _ => {}
                }
            }
            i += 1;
        }
    }
    let r = o(this, clube, situacao, fase, saiu, a, b);
    let igual = r == 0
        && match plano {
            Some((linha, v, marca)) => read_u8(linha + 0x37) == v && (!marca || read_u8(saiu) == 1),
            None => true,
        };
    PLACAR.registrar(igual, || format!("eng_second_update_staff_records_for_status({:#x}, {:#x}, {}, {}) original={} plano={:x?}", this, clube, s, fase as u8, r, plano));
    r
}

/// Comp_Stats::find_club_entry (0x4a4850) for a club in this competition.
///
/// [pt] A entrada do clube nas estatísticas da competição (Comp_Stats::find_club_entry 0x4a4850: thiscall(+8, clube), ret 4;
/// põe a tabela na ordem dos clubes se precisa e procura).
pub(crate) unsafe fn eng_second_comp_stats_club_entry(this: usize, clube: usize) -> usize {
    let achar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_FIND_CLUB_ENTRY);
    achar(read_usize(this + 8), clube)
}

/// Writes the club's stage, best and worst (+0x73..+0x75) in the competition stats, remapping play-off positions.
///
/// [pt] A situação do clube nas estatísticas (+0x73 a fase, +0x74 a melhor, +0x75 a pior), comum à First e à Third: na liga
/// (fase 0xff) a melhor e a pior de `baixo` a `alto` (as vagas do mata-mata) viram `baixo`, e com a pior nessa faixa a
/// pior vira `alto`; no mata-mata (fase 0), 1, 2 e 3 viram `baixo`, `baixo` + 1 e `baixo` + 2 (a melhor, a pior e a
/// fase); noutra fase, como vieram.
pub(crate) unsafe fn set_club_stage_status(this: usize, clube: usize, fase: u8, atual: u8, melhor: u8, pior: u8, baixo: u8, alto: u8) {
    let e = eng_second_comp_stats_club_entry(this, clube);
    let (mut m, mut p, mut a) = (melhor, pior, atual);
    if fase == 0xff {
        if m >= baixo && m <= alto {
            m = baixo;
        }
        if p >= baixo && p <= alto {
            p = alto;
        }
    } else if fase == 0 {
        let mapa = |v: u8| match v {
            1 => baixo,
            2 => baixo + 1,
            3 => baixo + 2,
            _ => v,
        };
        m = mapa(m);
        p = mapa(p);
        a = mapa(a);
    }
    *((e + 0x73) as *mut u8) = a;
    *((e + 0x74) as *mut u8) = m;
    *((e + 0x75) as *mut u8) = p;
}


/// like the First one, with places 4 to 7 (4 and 7; in the play-offs, 4, 5 and 6)
/// Original: `EngThird::set_club_progress` (0x578660, eng_third.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x578660 } (thiscall, ret 0x14, eng_third.cpp): `EngThird::set_club_progress(clube, fase, atual, melhor, pior)`,
/// a virtual +0x68 da Third: as vagas do mata-mata são 4 a 7.
pub(crate) unsafe extern "thiscall" fn eng_second_set_club_progress(this: usize, clube: usize, fase: u32, atual: u32, melhor: u32, pior: u32) {
    set_club_stage_status(this, clube, fase as u8, atual as u8, melhor as u8, pior as u8, 4, 7)
}

/// Comparer of the stage status: side by side on the entry's 3 bytes; only the original without an entry.
///
/// [pt] O comparador das duas: a entrada é achada antes (a original e a cópia acham a mesma; a ordem dos clubes já fica
/// pronta) e as duas rodam lado a lado nos 3 bytes dela. Sem entrada (a original gravaria no nulo): só a original.
pub(crate) unsafe fn compare_club_stage_status(nome: &str, this: usize, clube: usize, original: impl FnOnce(), copia: impl FnOnce()) {
    let e = eng_second_comp_stats_club_entry(this, clube);
    if e == 0 {
        return original();
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(e + 0x73, 3)], original, copia);
    PLACAR.registrar(igual, || format!("{}({:#x}, {:#x}) original={:x?} rust={:x?}", nome, this, clube, mo, mr));
}


pub(crate) unsafe extern "thiscall" fn comparar_eng_second_set_club_progress(this: usize, clube: usize, fase: u32, atual: u32, melhor: u32, pior: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(IDX_ENG_SECOND_SET_CLUB_PROGRESS);
    compare_club_stage_status("eng_second_set_club_progress", this, clube, || o(this, clube, fase, atual, melhor, pior), || eng_second_set_club_progress(this, clube, fase, atual, melhor, pior))
}

/// like the First one (line 0x294; the play-offs 0x576a70)
/// Original: `EngSecond::advance_stage` (0x5769e0, eng_second.cpp, thiscall, ret 0).
///
/// [pt] { 0x5769e0 } (thiscall, ret 0, eng_second.cpp): `EngSecond::advance_stage()` (a virtual +0x28): fonte 0x9c43e8,
/// linha 0x294; o mata-mata 0x576a70.
pub(crate) unsafe extern "thiscall" fn eng_second_advance_stage(this: usize) {
    advance_league_stage(this, 0x9c43e8, 0x294, 0x576a70, "cpz_3 mata-mata da second")
}

pub(crate) unsafe extern "thiscall" fn comparar_eng_second_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ENG_SECOND_ADVANCE_STAGE);
    compare_stage_advance("eng_second_advance_stage", this, || o(this), || eng_second_advance_stage(this))
}

/// like the First one (6th, 3rd, 5th and 4th), with +0x3a = the extra from virtual +0x3c (lines 0x2b6 and 0x2cc)
/// Original: `EngSecond::create_play_off_stage` (0x576a70, eng_second.cpp, thiscall, ret 0).
///
/// [pt] { 0x576a70 } (thiscall, ret 0, eng_second.cpp): `EngSecond::create_play_off_stage()` (de 0x5769e0). As chaves da
/// First (linhas 5, 2, 4 e 3); +0x3a da fase = o extra da virtual +0x3c. Fonte 0x9c43e8, linhas 0x2b6 e 0x2cc.
pub(crate) unsafe extern "thiscall" fn eng_second_create_play_off_stage(this: usize) {
    create_playoff_stage(this, [5, 2, 4, 3], 0x9c43e8, 0x2b6, 0x2cc, None, "cpz_1 mata-mata da second"); // cpz1: 6º × 3º e 5º × 4º da Second
}

pub(crate) unsafe extern "thiscall" fn comparar_eng_second_create_play_off_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ENG_SECOND_CREATE_PLAY_OFF_STAGE);
    compare_playoff_stage("eng_second_create_play_off_stage", this, [5, 2, 4, 3], || o(this))
}

/// the Second Division: discipline 9, no minimum ground, the league and the play-off
/// Original: `EngSecond::EngSecond` (0x5754a0, eng_second.cpp, thiscall, ret 8).
///
/// [pt] { 0x5754a0 } (thiscall, ret 8, eng_second.cpp): `EngSecond::EngSecond(ano, registro)`. A Second Division: 1 fase além
/// da liga (o play-off), sem estádio mínimo.
pub(crate) unsafe extern "thiscall" fn construct_eng_second(this: usize, ano: u32, registro: usize) -> usize {
    construct_english_division(this, ano, registro, &CQL6_SECOND)
}

pub(crate) unsafe extern "thiscall" fn comparar_construct_eng_second(this: usize, ano: u32, registro: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_ENG_SECOND);
    compare_english_constructor("construct_eng_second", this, ano, registro, &CQL6_SECOND, || o(this, ano, registro))
}

/// the Second (24): 2 go up directly, 3rd to 6th to the play-offs, the bottom 4 go down to the Third
/// Original: `EngSecond::set_up_rules` (0x576780, eng_second.cpp, thiscall, ret 0).
///
/// [pt] { 0x576780 } (thiscall, ret 0, eng_second.cpp): `EngSecond::set_up_rules()`. A Second: como a First, mas 4
/// caem (+0xc1); a First acima ([0x9cf5c0]) e a Third abaixo ([0x9cf5c8]).
pub(crate) unsafe extern "thiscall" fn eng_second_set_up_rules(this: usize) -> u32 {
    crate::comparacao::ramo("cqi_4 regras da second");
    eng_prm_apply_league_rules(this, &eng_prm_second_division_rules())
}

pub(crate) unsafe extern "thiscall" fn comparar_eng_second_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ENG_SECOND_SET_UP_RULES);
    eng_prm_compare_league_rules("eng_second_set_up_rules", this, eng_prm_second_division_rules(), || o(this), || eng_second_set_up_rules(this))
}

/// the Second has 24 clubs; prize fund of 20,000 per place; TV pays 2,500,000 to each club (300,000 up to 1999)
/// Original: `EngSecond::create_table` (0x576630, eng_second.cpp, thiscall, ret 0).
///
/// [pt] { 0x576630 } (thiscall, ret 0, eng_second.cpp): `EngSecond::create_table()`: 24 clubes ([0x9cf5c4]), cota 20.000
/// e a TV: 2.500.000 depois de 1999 (300.000 antes). Fonte 0x9c43e8, linha 0x1f5.
pub(crate) unsafe extern "thiscall" fn eng_second_create_table(this: usize) -> u32 {
    eng_prm_fill_league_table(this, 0x18, 0x9c43e8, 0x1f5, KEY_CLUB_COMP_ENGLISH_SECOND_DIVISION, 0x4e20, || eng_prm_pay_tv_rights(this, if is_after_1999() { 0x2625a0 } else { 0x493e0 }, 0))
}

pub(crate) unsafe extern "thiscall" fn comparar_eng_second_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ENG_SECOND_CREATE_TABLE);
    eng_prm_compare_league_table("eng_second_create_table", this, 0x18, KEY_CLUB_COMP_ENGLISH_SECOND_DIVISION, 0x4e20, || o(this))
}
