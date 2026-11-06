// ---------- %%LEVA%%a leva: o calendário da divisão de baixo espanhola (a Segunda B): as 42 rodadas, uma tabela para o ano par e outra para o ímpar, com a rodada do Natal (spa_lower.cpp) ----------


/// Uma rodada da tabela de 0x41 bytes: a data (dia, mês 0..11, anos a somar ao ano da liga +0x40, dia da semana para
/// onde a data é levada), o código do byte +4 ([inferência] o turno) e as faixas de TV em ordem (dia da semana, turno,
/// quantos jogos; a lista termina em −1).
type Cqq5Rodada = (i8, i8, i8, i8, i8, &'static [(i8, i8, i8)]);

/// As rodadas `primeira`.. da tabela `t`, na ordem da original: para cada uma, a data (0x68a160: push 0; push ano;
/// push código; push semana; push anos; push mês; push dia; push i; push t; add esp -> cdecl: o valor 0) e as
/// faixas (0x68a1c0: push 0; push c; push b; push a; push j; push i; push t; add esp -> cdecl, valor 0).
unsafe fn cqq5_rodadas(t: usize, ano: u32, primeira: i32, tabela: &[Cqq5Rodada]) {
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
unsafe fn cqq5_gravados(t: usize, guia: usize, n: usize) -> Vec<u8> {
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
struct Cqq5Natal {
    comeco: &'static [Cqq5Rodada],
    referencia: usize,
    ajuste: i32,
    casos: [&'static [Cqq5Rodada]; 8],
    fim: &'static [Cqq5Rodada],
    linha: u32,
}

unsafe fn cqq5_natal(t: usize, ano: u32, n: &Cqq5Natal, arquivo: usize) {
    let construir: unsafe extern "thiscall" fn(*mut [u16; 4], u32, u32, u32, u32) -> usize = core::mem::transmute(0x549f70usize);
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
/// `cqq5_gravados`; nas outras as de copa, `cql10_gravados`); a tabela da cópia é solta (0x9452ca, cdecl).
unsafe fn cqq5_comparar(nome: &str, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32,
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
            cqq5_gravados(a, a, n) == cqq5_gravados(b, a, n)
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
const CQQ5_BAIXO_PAR_COMECO: [Cqq5Rodada; 17] = [
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
const CQQ5_BAIXO_PAR_NATAL_0: [Cqq5Rodada; 1] = [
    (26, 11, 0, -1, 2, &[(-1, -1, -1)]), // 17: 26/dez
];

/// as rodadas do Natal quando a rodada 16 (a de referência) cai em 18/dez (ano par)
const CQQ5_BAIXO_PAR_NATAL_1: [Cqq5Rodada; 1] = [
    (26, 11, 0, -1, 2, &[(-1, -1, -1)]), // 17: 26/dez
];

/// as rodadas do Natal quando a rodada 16 (a de referência) cai em 19/dez (ano par)
const CQQ5_BAIXO_PAR_NATAL_2: [Cqq5Rodada; 1] = [
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 17: 26/dez
];

/// as rodadas do Natal quando a rodada 16 (a de referência) cai em 20/dez (ano par)
const CQQ5_BAIXO_PAR_NATAL_3: [Cqq5Rodada; 1] = [
    (27, 11, 0, -1, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 17: 27/dez
];

/// as rodadas do Natal quando a rodada 16 (a de referência) cai em 21/dez (ano par)
const CQQ5_BAIXO_PAR_NATAL_4: [Cqq5Rodada; 1] = [
    (28, 11, 0, -1, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 17: 28/dez
];

/// as rodadas do Natal quando a rodada 16 (a de referência) cai em 22/dez (ano par)
const CQQ5_BAIXO_PAR_NATAL_5: [Cqq5Rodada; 1] = [
    (29, 11, 0, -1, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 17: 29/dez
];

/// as rodadas do Natal quando a rodada 16 (a de referência) cai em 23/dez (ano par)
const CQQ5_BAIXO_PAR_NATAL_6: [Cqq5Rodada; 1] = [
    (30, 11, 0, -1, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 17: 30/dez
];

/// as rodadas do Natal no caso padrão (a rodada 16 fora de 17 a 23/dez: a caixa) (ano par)
const CQQ5_BAIXO_PAR_NATAL_PADRAO: [Cqq5Rodada; 1] = [
    (27, 11, 0, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 17: 27/dez
];

/// a divisão de baixo espanhola: as rodadas depois do Natal (ano par)
const CQQ5_BAIXO_PAR_FIM: [Cqq5Rodada; 24] = [
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
const CQQ5_BAIXO_IMPAR_COMECO: [Cqq5Rodada; 20] = [
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
const CQQ5_BAIXO_IMPAR_NATAL_0: [Cqq5Rodada; 1] = [
    (26, 11, 0, -1, 2, &[(-1, -1, -1)]), // 20: 26/dez
];

/// as rodadas do Natal quando a rodada 19 (a de referência) cai em 18/dez (ano impar)
const CQQ5_BAIXO_IMPAR_NATAL_1: [Cqq5Rodada; 1] = [
    (26, 11, 0, -1, 2, &[(-1, -1, -1)]), // 20: 26/dez
];

/// as rodadas do Natal quando a rodada 19 (a de referência) cai em 19/dez (ano impar)
const CQQ5_BAIXO_IMPAR_NATAL_2: [Cqq5Rodada; 1] = [
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 20: 26/dez
];

/// as rodadas do Natal quando a rodada 19 (a de referência) cai em 20/dez (ano impar)
const CQQ5_BAIXO_IMPAR_NATAL_3: [Cqq5Rodada; 1] = [
    (27, 11, 0, -1, 1, &[(5, 1, 1), (-1, -1, -1)]), // 20: 27/dez
];

/// as rodadas do Natal quando a rodada 19 (a de referência) cai em 21/dez (ano impar)
const CQQ5_BAIXO_IMPAR_NATAL_4: [Cqq5Rodada; 1] = [
    (28, 11, 0, -1, 1, &[(5, 1, 1), (-1, -1, -1)]), // 20: 28/dez
];

/// as rodadas do Natal quando a rodada 19 (a de referência) cai em 22/dez (ano impar)
const CQQ5_BAIXO_IMPAR_NATAL_5: [Cqq5Rodada; 1] = [
    (29, 11, 0, -1, 1, &[(5, 1, 1), (-1, -1, -1)]), // 20: 29/dez
];

/// as rodadas do Natal quando a rodada 19 (a de referência) cai em 23/dez (ano impar)
const CQQ5_BAIXO_IMPAR_NATAL_6: [Cqq5Rodada; 1] = [
    (30, 11, 0, -1, 1, &[(5, 1, 1), (-1, -1, -1)]), // 20: 30/dez
];

/// as rodadas do Natal no caso padrão (a rodada 19 fora de 17 a 23/dez: a caixa) (ano impar)
const CQQ5_BAIXO_IMPAR_NATAL_PADRAO: [Cqq5Rodada; 1] = [
    (27, 11, 0, 6, 1, &[(5, 1, 1), (-1, -1, -1)]), // 20: 27/dez
];

/// a divisão de baixo espanhola: as rodadas depois do Natal (ano impar)
const CQQ5_BAIXO_IMPAR_FIM: [Cqq5Rodada; 21] = [
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

const CQQ5_BAIXO_PAR_NATAL: Cqq5Natal = Cqq5Natal { comeco: &CQQ5_BAIXO_PAR_COMECO, referencia: 16, ajuste: 17, casos: [&CQQ5_BAIXO_PAR_NATAL_0, &CQQ5_BAIXO_PAR_NATAL_1, &CQQ5_BAIXO_PAR_NATAL_2, &CQQ5_BAIXO_PAR_NATAL_3, &CQQ5_BAIXO_PAR_NATAL_4, &CQQ5_BAIXO_PAR_NATAL_5, &CQQ5_BAIXO_PAR_NATAL_6, &CQQ5_BAIXO_PAR_NATAL_PADRAO], fim: &CQQ5_BAIXO_PAR_FIM, linha: 0x128 };
const CQQ5_BAIXO_IMPAR_NATAL: Cqq5Natal = Cqq5Natal { comeco: &CQQ5_BAIXO_IMPAR_COMECO, referencia: 19, ajuste: 17, casos: [&CQQ5_BAIXO_IMPAR_NATAL_0, &CQQ5_BAIXO_IMPAR_NATAL_1, &CQQ5_BAIXO_IMPAR_NATAL_2, &CQQ5_BAIXO_IMPAR_NATAL_3, &CQQ5_BAIXO_IMPAR_NATAL_4, &CQQ5_BAIXO_IMPAR_NATAL_5, &CQQ5_BAIXO_IMPAR_NATAL_6, &CQQ5_BAIXO_IMPAR_NATAL_PADRAO], fim: &CQQ5_BAIXO_IMPAR_FIM, linha: 0x237 };

/// { 0x851260 } (thiscall, ret 0x10, spa_lower.cpp, 8.495 bytes, método +0x3c): `SpaLower::create_schedule(fase,
/// &rodadas, &extra, &é_liga)`: só a fase −1, 42 rodadas, a tabela pela paridade do ano +0x40, cada uma com a rodada
/// do Natal; as outras devolvem 0. Fonte 0xa82980.
unsafe extern "thiscall" fn calendario_da_divisao_de_baixo_espanhola(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
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

unsafe extern "thiscall" fn comparar_calendario_da_divisao_de_baixo_espanhola(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    cqq5_comparar("calendario_da_divisao_de_baixo_espanhola", fase, rodadas, extra, liga, || o(this, fase, rodadas, extra, liga), |r, e, l| calendario_da_divisao_de_baixo_espanhola(this, fase, r, e, l))
}
