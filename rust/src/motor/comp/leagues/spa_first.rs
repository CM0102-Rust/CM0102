// ---------- 280a leva: o calendário da Primera espanhola: as 38 rodadas, uma tabela para o ano par e outra para o ímpar, com a rodada do Natal conforme o dia da semana (spa_first.cpp) ----------


/// Uma rodada da tabela de 0x41 bytes: a data (dia, mês 0..11, anos a somar ao ano da liga +0x40, dia da semana para
/// onde a data é levada), o código do byte +4 ([inferência] o turno) e as faixas de TV em ordem (dia da semana, turno,
/// quantos jogos; a lista termina em −1).
type Cqp6Rodada = (i8, i8, i8, i8, i8, &'static [(i8, i8, i8)]);

/// As rodadas `primeira`.. da tabela `t`, na ordem da original: para cada uma, a data (0x68a160: push 0; push ano;
/// push código; push semana; push anos; push mês; push dia; push i; push t; add esp -> cdecl: o valor 0) e as
/// faixas (0x68a1c0: push 0; push c; push b; push a; push j; push i; push t; add esp -> cdecl, valor 0).
unsafe fn cqp6_rodadas(t: usize, ano: u32, primeira: i32, tabela: &[Cqp6Rodada]) {
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
unsafe fn cqp6_gravados(t: usize, guia: usize, n: usize) -> Vec<u8> {
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

/// A fase principal com as rodadas do Natal: as do começo em sequência; depois a data de 1º de dezembro do ano
/// (Date::Date 0x549f70: push −1; push ano; push 0xb; push 1; lea ecx, local -> thiscall, ret 0x10) e o caso pelo dia
/// da rodada de referência (+0, palavra; uma rodada de `casos` levada ao dia da semana): k = (dia da rodada − dia da
/// data + 1, em 16 bits) − `ajuste`; de 0 a 6 escolhe as rodadas do Natal; fora disso a caixa da linha `linha` e as do
/// caso padrão (o oitavo); depois as do fim.
struct Cqp6Natal {
    comeco: &'static [Cqp6Rodada],
    referencia: usize,
    ajuste: i32,
    casos: [&'static [Cqp6Rodada]; 8],
    fim: &'static [Cqp6Rodada],
    linha: u32,
}

unsafe fn cqp6_natal(t: usize, ano: u32, n: &Cqp6Natal, arquivo: usize) {
    let construir: unsafe extern "thiscall" fn(*mut [u16; 4], u32, u32, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE_549F70);
    cqp6_rodadas(t, ano, 0, n.comeco);
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
    cqp6_rodadas(t, ano, n.comeco.len() as i32, meio);
    cqp6_rodadas(t, ano, (n.comeco.len() + meio.len()) as i32, n.fim);
}

/// Lado a lado em todas as fases: a original com as saídas de quem chama, a cópia com saídas próprias, a partir do
/// mesmo [0xb67a34]; as saídas e os bytes gravados das duas tabelas têm de ser iguais (na fase −1 as rodadas de liga,
/// `cqp6_gravados`; nas outras as de copa, `cql10_gravados`); a tabela da cópia é solta (0x9452ca, cdecl).
unsafe fn cqp6_comparar(nome: &str, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32,
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
            cqp6_gravados(a, a, n) == cqp6_gravados(b, a, n)
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

/// a Primera espanhola: as rodadas antes do Natal (ano par)
const CQP6_PRIMERA_PAR_COMECO: [Cqp6Rodada; 15] = [
    (30, 7, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 0: 30/ago
    (13, 8, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 1: 13/set
    (20, 8, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 2: 20/set
    (27, 8, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 3: 27/set
    (4, 9, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 4: 4/out
    (18, 9, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 5: 18/out
    (25, 9, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 6: 25/out
    (1, 10, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 7: 1/nov
    (8, 10, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 8: 8/nov
    (15, 10, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 9: 15/nov
    (22, 10, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 10: 22/nov
    (29, 10, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 11: 29/nov
    (6, 11, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 12: 6/dez
    (13, 11, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 13: 13/dez
    (20, 11, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 14: 20/dez
];

/// as rodadas do Natal quando a rodada 14 (a de referência) cai em 17/dez (ano par)
const CQP6_PRIMERA_PAR_NATAL_0: [Cqp6Rodada; 1] = [
    (26, 11, 0, -1, 2, &[(-1, -1, -1)]), // 15: 26/dez
];

/// as rodadas do Natal quando a rodada 14 (a de referência) cai em 18/dez (ano par)
const CQP6_PRIMERA_PAR_NATAL_1: [Cqp6Rodada; 1] = [
    (26, 11, 0, -1, 2, &[(-1, -1, -1)]), // 15: 26/dez
];

/// as rodadas do Natal quando a rodada 14 (a de referência) cai em 19/dez (ano par)
const CQP6_PRIMERA_PAR_NATAL_2: [Cqp6Rodada; 1] = [
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 15: 26/dez
];

/// as rodadas do Natal quando a rodada 14 (a de referência) cai em 20/dez (ano par)
const CQP6_PRIMERA_PAR_NATAL_3: [Cqp6Rodada; 1] = [
    (27, 11, 0, -1, 1, &[(5, 1, 1), (-1, -1, -1)]), // 15: 27/dez
];

/// as rodadas do Natal quando a rodada 14 (a de referência) cai em 21/dez (ano par)
const CQP6_PRIMERA_PAR_NATAL_4: [Cqp6Rodada; 1] = [
    (28, 11, 0, -1, 1, &[(5, 1, 1), (-1, -1, -1)]), // 15: 28/dez
];

/// as rodadas do Natal quando a rodada 14 (a de referência) cai em 22/dez (ano par)
const CQP6_PRIMERA_PAR_NATAL_5: [Cqp6Rodada; 1] = [
    (29, 11, 0, -1, 1, &[(5, 1, 1), (-1, -1, -1)]), // 15: 29/dez
];

/// as rodadas do Natal quando a rodada 14 (a de referência) cai em 23/dez (ano par)
const CQP6_PRIMERA_PAR_NATAL_6: [Cqp6Rodada; 1] = [
    (30, 11, 0, -1, 1, &[(5, 1, 1), (-1, -1, -1)]), // 15: 30/dez
];

/// as rodadas do Natal no caso padrão (a rodada 14 fora de 17 a 23/dez: a caixa) (ano par)
const CQP6_PRIMERA_PAR_NATAL_PADRAO: [Cqp6Rodada; 1] = [
    (27, 11, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 15: 27/dez
];

/// a Primera espanhola: as rodadas depois do Natal (ano par)
const CQP6_PRIMERA_PAR_FIM: [Cqp6Rodada; 22] = [
    (3, 0, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 16: 3/jan (+1 ano)
    (10, 0, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 17: 10/jan (+1 ano)
    (17, 0, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 18: 17/jan (+1 ano)
    (24, 0, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 19: 24/jan (+1 ano)
    (31, 0, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 20: 31/jan (+1 ano)
    (7, 1, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 21: 7/fev (+1 ano)
    (14, 1, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 22: 14/fev (+1 ano)
    (21, 1, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 23: 21/fev (+1 ano)
    (28, 1, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 24: 28/fev (+1 ano)
    (7, 2, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 25: 7/mar (+1 ano)
    (14, 2, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 26: 14/mar (+1 ano)
    (21, 2, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 27: 21/mar (+1 ano)
    (4, 3, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 28: 4/abr (+1 ano)
    (11, 3, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 29: 11/abr (+1 ano)
    (18, 3, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 30: 18/abr (+1 ano)
    (25, 3, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 31: 25/abr (+1 ano)
    (2, 4, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 32: 2/mai (+1 ano)
    (9, 4, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 33: 9/mai (+1 ano)
    (16, 4, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 34: 16/mai (+1 ano)
    (23, 4, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 35: 23/mai (+1 ano)
    (30, 4, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 36: 30/mai (+1 ano)
    (6, 5, 1, 6, 1, &[(-1, -1, -1)]), // 37: 6/jun (+1 ano)
];

/// a Primera espanhola: as rodadas antes do Natal (ano impar)
const CQP6_PRIMERA_IMPAR_COMECO: [Cqp6Rodada; 16] = [
    (22, 7, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 0: 22/ago
    (29, 7, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 1: 29/ago
    (12, 8, 0, 6, 2, &[(5, 1, 1), (-1, -1, -1)]), // 2: 12/set
    (19, 8, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 3: 19/set
    (26, 8, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 4: 26/set
    (3, 9, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 5: 3/out
    (17, 9, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 6: 17/out
    (20, 9, 0, 2, 2, &[(-1, -1, -1)]), // 7: 20/out
    (24, 9, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 8: 24/out
    (31, 9, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 9: 31/out
    (7, 10, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 10: 7/nov
    (21, 10, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 11: 21/nov
    (28, 10, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 12: 28/nov
    (5, 11, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 13: 5/dez
    (12, 11, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 14: 12/dez
    (19, 11, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 15: 19/dez
];

/// as rodadas do Natal quando a rodada 15 (a de referência) cai em 16/dez (ano impar)
const CQP6_PRIMERA_IMPAR_NATAL_0: [Cqp6Rodada; 1] = [
    (23, 11, 0, -1, 1, &[(-1, -1, -1)]), // 16: 23/dez
];

/// as rodadas do Natal quando a rodada 15 (a de referência) cai em 17/dez (ano impar)
const CQP6_PRIMERA_IMPAR_NATAL_1: [Cqp6Rodada; 1] = [
    (20, 11, 0, -1, 2, &[(-1, -1, -1)]), // 16: 20/dez
];

/// as rodadas do Natal quando a rodada 15 (a de referência) cai em 18/dez (ano impar)
const CQP6_PRIMERA_IMPAR_NATAL_2: [Cqp6Rodada; 1] = [
    (21, 11, 0, -1, 2, &[(-1, -1, -1)]), // 16: 21/dez
];

/// as rodadas do Natal quando a rodada 15 (a de referência) cai em 19/dez (ano impar)
const CQP6_PRIMERA_IMPAR_NATAL_3: [Cqp6Rodada; 1] = [
    (22, 11, 0, -1, 1, &[(-1, -1, -1)]), // 16: 22/dez
];

/// as rodadas do Natal quando a rodada 15 (a de referência) cai em 20/dez (ano impar)
const CQP6_PRIMERA_IMPAR_NATAL_4: [Cqp6Rodada; 1] = [
    (3, 0, 1, -1, 1, &[(-1, -1, -1)]), // 16: 3/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 15 (a de referência) cai em 21/dez (ano impar)
const CQP6_PRIMERA_IMPAR_NATAL_5: [Cqp6Rodada; 1] = [
    (28, 11, 0, -1, 1, &[(-1, -1, -1)]), // 16: 28/dez
];

/// as rodadas do Natal quando a rodada 15 (a de referência) cai em 22/dez (ano impar)
const CQP6_PRIMERA_IMPAR_NATAL_6: [Cqp6Rodada; 1] = [
    (29, 11, 0, -1, 1, &[(5, 1, 1), (-1, -1, -1)]), // 16: 29/dez
];

/// as rodadas do Natal no caso padrão (a rodada 15 fora de 16 a 22/dez: a caixa) (ano impar)
const CQP6_PRIMERA_IMPAR_NATAL_PADRAO: [Cqp6Rodada; 1] = [
    (27, 11, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 16: 27/dez
];

/// a Primera espanhola: as rodadas depois do Natal (ano impar)
const CQP6_PRIMERA_IMPAR_FIM: [Cqp6Rodada; 21] = [
    (5, 0, 1, 2, 2, &[(-1, -1, -1)]), // 17: 5/jan (+1 ano)
    (9, 0, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 18: 9/jan (+1 ano)
    (16, 0, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 19: 16/jan (+1 ano)
    (23, 0, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 20: 23/jan (+1 ano)
    (30, 0, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 21: 30/jan (+1 ano)
    (6, 1, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 22: 6/fev (+1 ano)
    (13, 1, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 23: 13/fev (+1 ano)
    (20, 1, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 24: 20/fev (+1 ano)
    (27, 1, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 25: 27/fev (+1 ano)
    (5, 2, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 26: 5/mar (+1 ano)
    (12, 2, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 27: 12/mar (+1 ano)
    (19, 2, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 28: 19/mar (+1 ano)
    (26, 2, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 29: 26/mar (+1 ano)
    (2, 3, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 30: 2/abr (+1 ano)
    (9, 3, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 31: 9/abr (+1 ano)
    (16, 3, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 32: 16/abr (+1 ano)
    (23, 3, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 33: 23/abr (+1 ano)
    (30, 3, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 34: 30/abr (+1 ano)
    (7, 4, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 35: 7/mai (+1 ano)
    (14, 4, 1, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 36: 14/mai (+1 ano)
    (21, 4, 1, 6, 1, &[(-1, -1, -1)]), // 37: 21/mai (+1 ano)
];

const CQP6_PRIMERA_PAR_NATAL: Cqp6Natal = Cqp6Natal { comeco: &CQP6_PRIMERA_PAR_COMECO, referencia: 14, ajuste: 17, casos: [&CQP6_PRIMERA_PAR_NATAL_0, &CQP6_PRIMERA_PAR_NATAL_1, &CQP6_PRIMERA_PAR_NATAL_2, &CQP6_PRIMERA_PAR_NATAL_3, &CQP6_PRIMERA_PAR_NATAL_4, &CQP6_PRIMERA_PAR_NATAL_5, &CQP6_PRIMERA_PAR_NATAL_6, &CQP6_PRIMERA_PAR_NATAL_PADRAO], fim: &CQP6_PRIMERA_PAR_FIM, linha: 0x11b };
const CQP6_PRIMERA_IMPAR_NATAL: Cqp6Natal = Cqp6Natal { comeco: &CQP6_PRIMERA_IMPAR_COMECO, referencia: 15, ajuste: 16, casos: [&CQP6_PRIMERA_IMPAR_NATAL_0, &CQP6_PRIMERA_IMPAR_NATAL_1, &CQP6_PRIMERA_IMPAR_NATAL_2, &CQP6_PRIMERA_IMPAR_NATAL_3, &CQP6_PRIMERA_IMPAR_NATAL_4, &CQP6_PRIMERA_IMPAR_NATAL_5, &CQP6_PRIMERA_IMPAR_NATAL_6, &CQP6_PRIMERA_IMPAR_NATAL_PADRAO], fim: &CQP6_PRIMERA_IMPAR_FIM, linha: 0x1e1 };

/// { 0x84e450 } (thiscall, ret 0x10, spa_first.cpp, 6.107 bytes, método +0x3c): `SpaFirst::create_schedule(fase,
/// &rodadas, &extra, &é_liga)`: só a fase −1, 38 rodadas, a tabela pela paridade do ano +0x40 (o resto da divisão com
/// sinal: `movsx; and 0x80000001; jns`), cada uma com a rodada do Natal; as outras devolvem 0. Fonte 0xa82944.
unsafe extern "thiscall" fn calendario_da_primera(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    match fase as u8 {
        0xff => {
            if !liga.is_null() {
                *liga = 1;
            }
            *rodadas = 38;
            *extra = 0;
            let t = alocar(0x9a6);
            if t == 0 {
                aviso_em(0xa82844, 0xb0);
                return 0;
            }
            crate::comparacao::ramo("cqp_6 calendario da primera");
            let ano = core::ptr::read_unaligned((this + 0x40) as *const u16) as u32;
            if (ano as i16) % 2 != 0 {
                cqp6_natal(t, ano, &CQP6_PRIMERA_IMPAR_NATAL, 0xa82844);
            } else {
                cqp6_natal(t, ano, &CQP6_PRIMERA_PAR_NATAL, 0xa82844);
            }
            t
        }
        _ => 0,
    }
}

unsafe extern "thiscall" fn comparar_calendario_da_primera(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1925);
    cqp6_comparar("calendario_da_primera", fase, rodadas, extra, liga, || o(this, fase, rodadas, extra, liga), |r, e, l| calendario_da_primera(this, fase, r, e, l))
}

// ---------- 275a leva: as regras das divisões espanholas (Primera, Segunda e a divisão de baixo): turnos, pontos, vagas, o confronto direto primeiro nos desempates e a tabela de rodadas (spa_first.cpp, spa_second.cpp, spa_lower.cpp) ----------


/// No ensaio (o comparador) a cópia grava as regras e não chama a virtual +0x3c (a tabela de rodadas, que aloca).
static mut CQK4_ENSAIO: bool = false;

/// Grava as regras na liga (deslocamento, tamanho 1/2/4, valor), na ordem da lista, e monta a tabela de rodadas da
/// fase principal: a virtual +0x3c (create_schedule: thiscall(liga, −1, &+0xa9, &+0x3a, 0), ret 0x10) com o
/// resultado em +0xba. Devolve 1.
unsafe fn cqk4_regras(this: usize, escritas: &[(usize, u8, u32)]) -> u32 {
    for &(d, t, v) in escritas {
        match t {
            1 => *((this + d) as *mut u8) = v as u8,
            2 => core::ptr::write_unaligned((this + d) as *mut u16, v as u16),
            _ => core::ptr::write_unaligned((this + d) as *mut u32, v),
        }
    }
    if !CQK4_ENSAIO {
        let rodadas: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + 0x3c));
        let t = rodadas(this, 0xffff_ffff, this + 0xa9, this + 0x3a, 0);
        core::ptr::write_unaligned((this + 0xba) as *mut usize, t);
    }
    1
}

/// As regras comuns às três: 1 ponto pelo empate (+0xc3), categoria 2 nas partidas (+0xc4), clubes nacionais (+0x42
/// = 1), os desempates 4, 1 e 2 (+0xc5..+0xc7: o confronto direto primeiro [inferência pelo código 4], depois o
/// saldo e os gols pró), nenhum play-off de permanência (+0xc0 = 0), os bits de regra +0xd9 = 0x43 (campeão, vagas de
/// cima de acesso e a ordem fixa das rodadas), 7 reservas (+0x49) e 3 substituições (+0x4a).
fn cqk4_comuns() -> Vec<(usize, u8, u32)> {
    vec![(0xc3, 1, 1), (0xc4, 1, 2), (0x42, 1, 1), (0xc5, 1, 4), (0xc6, 1, 1), (0xc7, 1, 2), (0xc0, 1, 0), (0xd9, 2, 0x43), (0x49, 1, 7), (0x4a, 1, 3)]
}

/// { 0x84fdf0 } (thiscall, ret 0, spa_first.cpp): `SpaFirst::set_up_rules()`. A Primera: +0xa9 = 38 (as rodadas;
/// a virtual reescreve), as comuns, 2 turnos, 3 pontos, o 4º desempate 3 (+0xc8), +0xab e +0xdb = 0, ninguém sobe
/// (+0xbe = +0xbf = 0), 3 caem (+0xc1), nenhuma liga acima (+0x1c = −1), a Segunda abaixo (+0x20 = [0x9cf628]) e
/// +0x52 = 2 [sem nome].
unsafe extern "thiscall" fn regras_da_primera(this: usize) -> u32 {
    crate::comparacao::ramo("cqk_4 regras da primera");
    core::ptr::write_unaligned((this + 0xa9) as *mut u16, 0x26);
    cqk4_regras(this, &cqk4_primera())
}

unsafe fn cqk4_primera() -> Vec<(usize, u8, u32)> {
    let mut v = cqk4_comuns();
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3), (0xc8, 1, 3), (0xab, 2, 0), (0xdb, 2, 0), (0xbe, 1, 0), (0xbf, 1, 0)]);
    v.push((0xc1, 1, 3)); // cqk4: a Primera rebaixa 3
    v.extend_from_slice(&[(0x1c, 4, 0xffff_ffff), (0x20, 4, read_usize(KEY_CLUB_COMP_SPANISH_SECOND_DIVISION) as u32), (0x52, 1, 2)]);
    v
}

/// { 0x855cc0 } (thiscall, ret 0, spa_second.cpp): `SpaSecond::set_up_rules()`. A Segunda: as comuns, 2 turnos, 3
/// pontos, o 4º desempate 3, 3 sobem direto (+0xbe), nenhum play-off de acesso, 4 caem (+0xc1), a Primera acima
/// ([0x9cf624]) e abaixo a Segunda B ([0x9cf7b8]) quando a nação tem as divisões de baixo carregadas (bit 4 de +0x11c
/// da nação [inferência]), senão −1; +0x52 = 2.
unsafe extern "thiscall" fn regras_da_segunda(this: usize) -> u32 {
    crate::comparacao::ramo("cqk_4 regras da segunda");
    cqk4_regras(this, &cqk4_segunda(this))
}

unsafe fn cqk4_segunda(this: usize) -> Vec<(usize, u8, u32)> {
    let mut v = cqk4_comuns();
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3)]); // cqk4: a Segunda joga 2 turnos
    v.extend_from_slice(&[(0xc8, 1, 3), (0xbe, 1, 3), (0xbf, 1, 0), (0xc1, 1, 4), (0x1c, 4, read_usize(KEY_CLUB_COMP_SPANISH_FIRST_DIVISION) as u32)]);
    let nacao = read_usize(read_usize(this + 4) + 0x5d);
    let abaixo = if *((nacao + 0x11c) as *const u8) & 4 != 0 { read_usize(KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B) as u32 } else { 0xffff_ffff };
    v.extend_from_slice(&[(0x20, 4, abaixo), (0x52, 1, 2)]);
    v
}

/// { 0x853790 } (thiscall, ret 0, spa_lower.cpp): `SpaLower::set_up_rules()`. A divisão de baixo (os times B e os
/// melhores das divisões menores): as comuns, 2 turnos, 3 pontos, 2 sobem direto (+0xbe) e 2 vão ao play-off de
/// acesso (+0xbf), 4 caem (+0xc1), a Primera como liga de cima ([0x9cf624], assim no original) e nenhuma abaixo.
unsafe extern "thiscall" fn regras_da_divisao_de_baixo(this: usize) -> u32 {
    crate::comparacao::ramo("cqk_4 regras da divisao de baixo");
    cqk4_regras(this, &cqk4_baixo())
}

unsafe fn cqk4_baixo() -> Vec<(usize, u8, u32)> {
    let mut v = cqk4_comuns();
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3)]); // cqk4: a divisão de baixo dá 3 pontos pela vitória
    v.extend_from_slice(&[(0xbe, 1, 2), (0xbf, 1, 2), (0xc1, 1, 4), (0x1c, 4, read_usize(KEY_CLUB_COMP_SPANISH_FIRST_DIVISION) as u32), (0x20, 4, 0xffff_ffff)]);
    v
}

/// O comparador das três (ensaio, como o da cqi_4): a original roda inteira; a faixa +0x1c..+0xec volta e a cópia
/// grava as regras sem a virtual +0x3c; os campos da lista saem iguais aos da original (o +0xa9 da Primera, que a
/// virtual reescreve, fica de fora), e a devolução 1; no fim fica a faixa da original.
unsafe fn cqk4_comparar(nome: &str, this: usize, escritas: Vec<(usize, u8, u32)>, original: impl FnOnce() -> u32, copia: impl FnOnce() -> u32) -> u32 {
    let faixa = [(this + 0x1c, 0xd0)];
    let antes = crate::comparacao::Memoria::ler(&faixa);
    let r = original();
    let mo = crate::comparacao::Memoria::ler(&faixa);
    antes.escrever();
    CQK4_ENSAIO = true;
    let rr = crate::aleatorio::pausar_rastro(copia);
    CQK4_ENSAIO = false;
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

unsafe extern "thiscall" fn comparar_regras_da_primera(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1762);
    cqk4_comparar("regras_da_primera", this, cqk4_primera(), || o(this), || regras_da_primera(this))
}

unsafe extern "thiscall" fn comparar_regras_da_segunda(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1763);
    cqk4_comparar("regras_da_segunda", this, cqk4_segunda(this), || o(this), || regras_da_segunda(this))
}

unsafe extern "thiscall" fn comparar_regras_da_divisao_de_baixo(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1764);
    cqk4_comparar("regras_da_divisao_de_baixo", this, cqk4_baixo(), || o(this), || regras_da_divisao_de_baixo(this))
}

// ---------- 275a leva: a Primera espanhola (a tabela, a TV, a divisão de baixo genérica e a situação nas estatísticas) e os registros da Série C2/A italiana pela situação na tabela, no play-off e no play-out (spa_first.cpp, ita_ser_c2a.cpp) ----------



/// O registro de competição (0x6b bytes em [0xae23d0]) do id guardado no global `g`.
unsafe fn cqk5_registro(g: usize) -> usize {
    read_usize(0xae23d0).wrapping_add((read_usize(g) as i32 as usize).wrapping_mul(0x6b))
}

/// { 0x84fc70 } (thiscall, ret 0, spa_first.cpp): `SpaFirst::create_table()`. +0x3e = 20 e a tabela (malloc 0x944e46
/// de 0x49c; sem memória: a caixa do fonte 0xa82844, linha 0x250, e 0). Antes de encher, os clubes da divisão de
/// baixo espanhola ([0x9cf6b0]) passam à divisão de baixo genérica (+0x57 = o registro de [0x9cf764]). Depois os
/// clubes da Primera ([0x9cf624]) na ordem do banco, cada um na linha seguinte (League::init_table_row 0x687430:
/// thiscall(liga, linha, clube, 0, 0), ret 0x10), até encher (sem encher: 0). Cheia: o fundo de prêmios (+0x28 =
/// League::calc_prize_fund 0x682f70 pela cota de 250.000), +0xe1 = 1, a TV de 5.000.000 a cada clube (0x683010:
/// thiscall(liga, valor, 0), ret 8) e 1.
unsafe extern "thiscall" fn tabela_da_primera(this: usize) -> u32 {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 0x14);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let t = alocar(0x49c);
    core::ptr::write_unaligned((this + 0xb1) as *mut usize, t);
    if t == 0 {
        aviso_em(0xa82844, 0x250);
        return 0;
    }
    // a divisão de baixo espanhola vira a genérica
    let mut i: i16 = 0;
    let mut campo = read_usize(0xae23b4) + 0x57;
    while (i as i32) < read_usize(0xae235c) as i32 {
        let comp = read_usize(campo);
        if comp != 0 && read_usize(comp) == read_usize(KEY_CLUB_COMP_SPANISH_LOWER_DIVISION) {
            core::ptr::write_unaligned(campo as *mut usize, cqk5_registro(KEY_CLUB_COMP_A_LOWER_DIVISION));
        }
        i = i.wrapping_add(1);
        campo = campo.wrapping_add(0x245);
    }
    let iniciar: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(FN_LEAGUE_INIT_TABLE_ROW);
    let mut k: u8 = 0;
    let mut i: i16 = 0;
    let mut clube = read_usize(0xae23b4);
    while (i as i32) < read_usize(0xae235c) as i32 {
        let comp = read_usize(clube + 0x57);
        if comp != 0 && read_usize(comp) == read_usize(KEY_CLUB_COMP_SPANISH_FIRST_DIVISION) {
            let linha = k; // cqk5: a linha k recebe o k-ésimo clube da Primera
            // ponytail: a original empurra a linha no eax com os bits de cima do id da competição; a 0x687430 só lê
            // o byte.
            iniciar(this, linha as u32, clube, 0, 0);
            k = k.wrapping_add(1);
            if k as i8 as i16 == read_i16(this + 0x3e) {
                break;
            }
        }
        i = i.wrapping_add(1);
        clube = clube.wrapping_add(0x245);
    }
    if k as i8 as i16 != read_i16(this + 0x3e) {
        return 0;
    }
    crate::comparacao::ramo("cqk_5 tabela da primera");
    let premio: unsafe extern "thiscall" fn(usize, i32) -> i32 = core::mem::transmute(FN_LEAGUE_CALC_PRIZE_FUND);
    let f = premio(this, 0x3d090); // cqk5: a cota da Primera
    core::ptr::write_unaligned((this + 0x28) as *mut i32, f);
    *((this + 0xe1) as *mut u8) = 1; // cqk5: o prêmio pela tabela no fim da liga
    let pagar: unsafe extern "thiscall" fn(usize, i32, usize) = core::mem::transmute(FN_LEAGUE_PAY_TV_RIGHTS);
    pagar(this, 0x4c4b40, 0); // cqk5: a TV da Primera
    1
}

/// Comparador da 0x84fc70: a original aloca a tabela, muda a divisão dos clubes e paga a TV (com notícia): só ela. O
/// plano, feito antes: os clubes da divisão de baixo espanhola (que passam à genérica) e os da Primera na ordem do
/// banco (até 20). Depois: os primeiros com +0x57 = o registro de [0x9cf764]; +0x3e = 20, a devolução, as linhas com
/// os clubes da Primera na ordem e, cheia, +0xe1 = 1 e +0x28 = o fundo da cota.
unsafe extern "thiscall" fn comparar_tabela_da_primera(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1765);
    let (mut baixo, mut plano): (Vec<usize>, Vec<usize>) = (Vec::new(), Vec::new());
    for i in 0..(read_usize(0xae235c) as i32).max(0) as usize {
        let clube = read_usize(0xae23b4) + i * 0x245;
        let comp = read_usize(clube + 0x57);
        if comp != 0 && read_usize(comp) == read_usize(KEY_CLUB_COMP_SPANISH_LOWER_DIVISION) {
            baixo.push(clube);
        } else if comp != 0 && read_usize(comp) == read_usize(KEY_CLUB_COMP_SPANISH_FIRST_DIVISION) && plano.len() < 0x14 {
            plano.push(clube);
        }
    }
    let r = o(this);
    let t = read_usize(this + 0xb1);
    let generica = cqk5_registro(KEY_CLUB_COMP_A_LOWER_DIVISION);
    let mut igual = r == (plano.len() == 0x14) as u32 && read_i16(this + 0x3e) == 0x14 && t != 0 && baixo.iter().all(|&c| read_usize(c + 0x57) == generica);
    let mut achou: Vec<usize> = Vec::new();
    if igual {
        achou = (0..plano.len()).map(|k| read_usize(t + k * 0x3b)).collect();
        igual = achou == plano;
        if r == 1 {
            let premio: unsafe extern "thiscall" fn(usize, i32) -> i32 = core::mem::transmute(FN_LEAGUE_CALC_PRIZE_FUND);
            igual = igual && *((this + 0xe1) as *const u8) == 1 && core::ptr::read_unaligned((this + 0x28) as *const i32) == premio(this, 0x3d090);
        }
    }
    PLACAR.registrar(igual, || format!("tabela_da_primera({:#x}) original={} baixo={} plano={:x?} tabela={:x?}", this, r, baixo.len(), plano, achou));
    r
}

/// O registro de staff ([0xdd7f00]).
unsafe fn cqk5_registros() -> usize {
    read_usize(0xdd7f00)
}

/// { 0x654180 } (thiscall, ret 0x18, ita_ser_c2a.cpp): `ItaSerC2a::update_staff_records_for_status(clube, situação,
/// fase, _, jogo, _)`, a virtual +0x44 (anunciada pela 0x686670). Devolve 0. Na liga (fase 0xff), pela situação: 5,
/// o título (Staff_Records::add_league_champions_record 0x868c50: thiscall(registros, clube, comp +4), ret 8); 0, o
/// acesso (0x869480: thiscall(registros, clube, comp, 0x64), ret 0xc); 1, a vaga no play-off (0x86bdd0:
/// thiscall(registros, clube, comp, 0xa0, 0x82, 0x28), ret 0x14); 3, o rebaixamento (0x86a1c0: thiscall(registros,
/// clube, comp), ret 8). No play-off de acesso (fase 0), na primeira linha do clube com situação 1, 2 ou 0xff: 1
/// (venceu), o acesso com peso 0x32 e +0x37 = 0; 2 ou 0xff (caiu), a eliminação (0x86c000: thiscall(registros, clube,
/// comp, fase do jogo +0x32, rodada +0x30, peso), ret 0x14; o peso = comp_util_scale_by_round 0x4bf850: cdecl(0x1e,
/// 0x32, rodada do jogo +0x34, 2)) e +0x37 = 0xfe. No play-out (fase 1): 1 (salvou-se), +0x37 = 0xfe; 2 ou 0xff,
/// o rebaixamento e +0x37 = 3.
unsafe extern "thiscall" fn registros_pela_situacao_na_c2a(this: usize, clube: usize, situacao: u32, fase: u32, _a4: u32, jogo: usize, _a6: u32) -> u32 {
    let s = situacao as u8;
    let comp = read_usize(this + 4);
    let f = fase as u8;
    if f == 0xff {
        match s {
            5 => {
                let titulo: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_STAFF_RECORDS_ADD_LEAGUE_CHAMPIONS_RECORD);
                titulo(cqk5_registros(), clube, comp);
            }
            0 => {
                let acesso: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_STAFF_RECORDS_ADD_PROMOTION_RECORD);
                acesso(cqk5_registros(), clube, comp, 0x64);
            }
            1 => {
                crate::comparacao::ramo("cqk_5 vaga no play-off da c2a");
                let vaga: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(FN_STAFF_RECORDS_ADD_QUALIFIED_FOR_ROUND_RECORD);
                vaga(cqk5_registros(), clube, comp, 0xa0, 0x82, 0x28);
            }
            3 => {
                crate::comparacao::ramo("cqk_5 rebaixamento na c2a");
                let cair: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_STAFF_RECORDS_ADD_RELEGATION_RECORD);
                cair(cqk5_registros(), clube, comp); // cqk5: o rebaixamento pela tabela da C2/A
            }
            _ => {}
        }
        return 0;
    }
    if f > 1 {
        return 0;
    }
    let n = read_i16(this + 0x3e) as i32;
    let mut i: i32 = 0;
    while i < n {
        let linha = read_usize(this + 0xb1) + (i as usize) * 0x3b;
        if read_usize(linha) == clube {
            if f == 0 {
                match s {
                    1 => {
                        let acesso: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_STAFF_RECORDS_ADD_PROMOTION_RECORD);
                        acesso(cqk5_registros(), clube, comp, 0x32);
                        *((read_usize(this + 0xb1) + (i as usize) * 0x3b + 0x37) as *mut u8) = 0;
                        return 0;
                    }
                    2 | 0xff => {
                        let escala: unsafe extern "C" fn(u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_COMP_UTIL_SCALE_BY_ROUND);
                        let peso = escala(0x1e, 0x32, read_i16(jogo + 0x34) as u16 as u32, 2);
                        // ponytail: a original empurra a fase (+0x32) e a rodada (+0x30) do jogo como palavras em
                        // registradores com lixo nos 16 bits de cima; a 0x86c000 só lê as palavras.
                        let fora: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(FN_STAFF_RECORDS_ADD_KNOCKED_OUT_RECORD);
                        fora(cqk5_registros(), clube, comp, read_i16(jogo + 0x32) as u16 as u32, read_i16(jogo + 0x30) as u16 as u32, peso);
                        *((read_usize(this + 0xb1) + (i as usize) * 0x3b + 0x37) as *mut u8) = 0xfe;
                        return 0;
                    }
                    _ => {}
                }
            } else {
                match s {
                    1 => {
                        *((read_usize(this + 0xb1) + (i as usize) * 0x3b + 0x37) as *mut u8) = 0xfe;
                        return 0;
                    }
                    2 | 0xff => {
                        let cair: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_STAFF_RECORDS_ADD_RELEGATION_RECORD);
                        cair(cqk5_registros(), clube, comp);
                        *((read_usize(this + 0xb1) + (i as usize) * 0x3b + 0x37) as *mut u8) = 3;
                        return 0;
                    }
                    _ => {}
                }
            }
        }
        i += 1;
    }
    0
}

/// Comparador da 0x654180: os registros vão ao histórico das pessoas e mexem nas reputações e no moral (não rodam duas
/// vezes): só a original. O plano, feito antes, nas fases 0 e 1: a primeira linha do clube com situação 1, 2 ou 0xff
/// decide o +0x37 que a rotina grava (fase 0: 0 com 1, 0xfe com 2/0xff; fase 1: 0xfe com 1, 3 com 2/0xff); depois a
/// linha tem de estar assim. Na liga ela só registra: a devolução 0.
unsafe extern "thiscall" fn comparar_registros_pela_situacao_na_c2a(this: usize, clube: usize, situacao: u32, fase: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(1766);
    let (s, f) = (situacao as u8, fase as u8);
    let mut plano: Option<(usize, u8)> = None;
    if f <= 1 {
        let n = read_i16(this + 0x3e) as i32;
        for i in 0..n.max(0) as usize {
            let l = read_usize(this + 0xb1) + i * 0x3b;
            if read_usize(l) == clube && (s == 1 || s == 2 || s == 0xff) {
                plano = Some((l, match (f, s) {
                    (0, 1) => 0,
                    (0, _) => 0xfe,
                    (_, 1) => 0xfe,
                    _ => 3,
                }));
                break;
            }
        }
    }
    let r = o(this, clube, situacao, fase, a4, jogo, a6);
    let igual = r == 0 && plano.map_or(true, |(l, v)| *((l + 0x37) as *const u8) == v);
    PLACAR.registrar(igual, || format!("registros_pela_situacao_na_c2a({:#x}, {:#x}, {}, {}) original={} plano={:x?}", this, clube, s, f, r, plano));
    r
}

/// { 0x850880 } (thiscall, ret 0x14, spa_first.cpp): `SpaFirst::set_club_progress(clube, fase, atual, melhor, pior)`,
/// a virtual +0x68 (da 0x684cd0 a cada linha da tabela): na entrada do clube nas estatísticas (Comp_Stats::
/// find_club_entry 0x4a4850: thiscall(+8, clube), ret 4) grava a posição atual (+0x73), a melhor possível (+0x74;
/// 17 e 18 viram 16) e a pior (+0x75; 17 e 18 viram 19): a faixa de 17 a 18 da Primera não conta como faixa própria
/// [inferência: as posições de 17 a 18 juntam-se à vizinha de cima na melhor e à de baixo na pior].
unsafe extern "thiscall" fn situacao_na_primera(this: usize, clube: usize, _fase: u32, atual: u32, melhor: u32, pior: u32) {
    let achar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_FIND_CLUB_ENTRY);
    let e = achar(read_usize(this + 8), clube);
    let m = match melhor as u8 {
        0x11 | 0x12 => 0x10,
        v => v,
    };
    let p = match pior as u8 {
        0x11 | 0x12 => 0x13,
        v => v,
    };
    *((e + 0x73) as *mut u8) = atual as u8;
    *((e + 0x74) as *mut u8) = m;
    *((e + 0x75) as *mut u8) = p;
}

/// Comparador da 0x850880: a entrada é achada antes (a original e a cópia acham a mesma) e as duas rodam lado a lado
/// nos 3 bytes dela. Sem entrada (a original gravaria no nulo): só a original.
unsafe extern "thiscall" fn comparar_situacao_na_primera(this: usize, clube: usize, fase: u32, atual: u32, melhor: u32, pior: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(1767);
    let achar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_FIND_CLUB_ENTRY);
    let e = achar(read_usize(this + 8), clube);
    if e == 0 {
        return o(this, clube, fase, atual, melhor, pior);
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(e + 0x73, 3)], || o(this, clube, fase, atual, melhor, pior), || situacao_na_primera(this, clube, fase, atual, melhor, pior));
    PLACAR.registrar(igual, || format!("situacao_na_primera({:#x}, {:#x}) original={:x?} rust={:x?}", this, clube, mo, mr));
}

