// ---------- 280a leva: o calendário da divisão de baixo espanhola (a Segunda B): as 42 rodadas, uma tabela para o ano par e outra para o ímpar, com a rodada do Natal (spa_lower.cpp) ----------


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
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1945);
    cqq5_comparar("calendario_da_divisao_de_baixo_espanhola", fase, rodadas, extra, liga, || o(this, fase, rodadas, extra, liga), |r, e, l| calendario_da_divisao_de_baixo_espanhola(this, fase, r, e, l))
}

// ---------- 278a leva: a divisão de baixo espanhola (o construtor e a tabela com os times B e os melhores da Segunda B) e a tabela da Segunda espanhola (os times B que não sobem) (spa_lower.cpp, spa_second.cpp) ----------




/// O registro de competição de id [g] (vetor [0xae23d0], 0x6b bytes).
unsafe fn cqm6_registro(g: usize) -> usize {
    read_u32_as_usize(0xae23d0).wrapping_add(read_u32_as_usize(g).wrapping_mul(0x6b))
}

/// A competição do clube (+0x57, o id no +0) é a de id [g]?
unsafe fn cqm6_na(c: usize, g: usize) -> bool {
    let comp = read_u32_as_usize(c + 0x57);
    comp != 0 && read_u32_as_usize(comp) == read_u32_as_usize(g)
}

/// { 0x851020 } (thiscall, ret 4, spa_lower.cpp, quadro SEH só no caminho de exceção): `SpaLower::SpaLower(ano)`, a
/// divisão de baixo espanhola (criada pela Primera quando a nação não tem o bit 4 de +0x11c): a base (League::League
/// 0x682200: mov ecx -> thiscall, ret 0), a vtable 0x96fb1c, o ano (+0x40), a regra de disciplina 0xff (nenhuma) e o
/// registro da competição de id [0x9cf6b0] (+4). Tenta carregar o estado (0x687b10: push 1 -> thiscall, ret 4);
/// senão, só a liga (+0x44 = 0xff, +0x30 = −1, +0x2c = 0), as regras (0x853790), a tabela (0x8533d0) e os jogos
/// (League::create_fixtures 0x6835c0), todos mov ecx -> thiscall, ret 0; cada falha, a caixa da sua linha (0x3f,
/// 0x46, 0x4d do fonte 0xa82880) e fim. Devolve this.
unsafe extern "thiscall" fn construir_divisao_de_baixo_espanhola(this: usize, ano: u32) -> usize {
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x682200usize);
    let carregar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x687b10usize);
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x853790usize);
    let tabela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x8533d0usize);
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x6835c0usize);
    base(this);
    write_u32_from_usize(this, 0x96fb1c);
    core::ptr::write_unaligned((this + 0x40) as *mut u16, ano as u16);
    *((this + 0x50) as *mut u8) = 0xff;
    write_u32_from_usize(this + 4, cqm6_registro(0x9cf6b0));
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
unsafe extern "thiscall" fn comparar_construir_divisao_de_baixo_espanhola(this: usize, ano: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32) -> usize>(1873);
    let r = o(this, ano);
    let ok = r == this && read_u32_as_usize(this) == 0x96fb1c && core::ptr::read_unaligned((this + 0x40) as *const u16) == ano as u16
        && *((this + 0x50) as *const u8) == 0xff && read_u32_as_usize(this + 4) == cqm6_registro(0x9cf6b0);
    PLACAR.registrar(ok, || format!("construir_divisao_de_baixo_espanhola({:#x}): devolveu {:#x}, vtable {:#x}, +4 = {:#x}", this, r, read_u32_as_usize(this), read_u32_as_usize(this + 4)));
    r
}

/// O clube de cima do clube `c` (club_get_linked_b_team 0x540a50: push 1; push &eh_a; push c; add esp, 0xc -> cdecl),
/// se `c` é o time B dele (eh_a = 0) e ele está na competição de id [g].
unsafe fn cqm6_time_b_de(c: usize, g: usize) -> bool {
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    let mut eh_a: u32 = 0;
    let p = parceiro(c, &mut eh_a, 1);
    p != 0 && eh_a == 0 && cqm6_na(p, g)
}

/// { 0x855b40 } (thiscall, ret 0, spa_second.cpp): `SpaSecond::create_table()`: a Segunda tem 22 clubes (+0x3e),
/// a tabela (malloc 0x944e46 de 0x512: push; add esp, 4 -> cdecl; sem memória, a caixa da linha 0x298 e 0) recebe, na
/// ordem do banco ([0xae23b4], [0xae235c] clubes de 0x245 bytes), os clubes da competição de id [0x9cf628] até encher
/// (League::init_table_row 0x687430: push 0; push 0; push clube; push linha (a palavra empurrada tem o byte da linha e o
/// lixo da pilha em cima; a 0x687430 só lê o byte); mov ecx, this -> thiscall, ret 0x10); o time B de um clube da
/// Primera ([0x9cf624]) leva o destino −4 (+0x37 = 0xfc: não pode subir). Cheia: o fundo de prêmios (+0x28 =
/// League::calc_prize_fund 0x682f70: push 50000; mov ecx -> thiscall, ret 4), +0xe1 = 1 e 1; senão 0.
unsafe extern "thiscall" fn tabela_da_segunda(this: usize) -> u32 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let iniciar: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(0x687430usize);
    let premios: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x682f70usize);
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 0x16);
    let t = alocar(0x512);
    write_u32_from_usize(this + 0xb1, t);
    if t == 0 {
        aviso_em(0xa828bc, 0x298);
        return 0;
    }
    let mut k: u8 = 0;
    let n = read_u32_as_usize(0xae235c) as i32;
    let base = read_u32_as_usize(0xae23b4);
    let mut i: i16 = 0;
    while (i as i32) < n {
        let c = base + (i as u16 as usize) * 0x245; // cqm6: o clube i do banco
        if cqm6_na(c, 0x9cf628) {
            iniciar(this, k as u32, c, 0, 0);
            if cqm6_time_b_de(c, 0x9cf624) {
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
unsafe extern "thiscall" fn comparar_tabela_da_segunda(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1874);
    let n = read_u32_as_usize(0xae235c) as i32;
    let base = read_u32_as_usize(0xae23b4);
    let mut plano: Vec<(usize, bool)> = Vec::new();
    for i in 0..n.max(0) as usize {
        let c = base + i * 0x245;
        if cqm6_na(c, 0x9cf628) {
            plano.push((c, cqm6_time_b_de(c, 0x9cf624)));
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
    PLACAR.registrar(ok, || format!("tabela_da_segunda({:#x}): devolveu {}, plano {:x?}", this, r, plano));
    r
}

/// Os candidatos da divisão de baixo espanhola, do registro local de 100 da original: os clubes (todos, na ordem do
/// banco, até 100) das competições de id [0x9cf62c], [0x9cf630], [0x9cf634], [0x9cf638] [inferência: os 4 grupos da
/// Segunda B] ou, se espanhóis, da de id [0x9cf764] [inferência: a divisão genérica de baixo], que não são time B nem
/// time A de ninguém (club_get_linked_b_team devolve 0).
unsafe fn cqm6_candidatos() -> Vec<usize> {
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    let mut v = Vec::new();
    let n = read_u32_as_usize(0xae235c) as i32;
    let base = read_u32_as_usize(0xae23b4);
    let mut i: i16 = 0;
    while (i as i32) < n && v.len() < 100 {
        let c = base + (i as u16 as usize) * 0x245;
        let comp = read_u32_as_usize(c + 0x57);
        let nacao = read_u32_as_usize(c + 0x53);
        if comp != 0 && nacao != 0 {
            let id = read_u32_as_usize(comp);
            let grupo = [0x9cf62c, 0x9cf630, 0x9cf634, 0x9cf638].iter().any(|&g| id == read_u32_as_usize(g));
            if grupo || (read_u32_as_usize(nacao) == read_u32_as_usize(0x9cf488) && id == read_u32_as_usize(0x9cf764)) {
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
unsafe fn cqm6_times_b(g: usize, k: &mut u8, linhas: i16, mut por: impl FnMut(usize, bool)) {
    let n = (read_u32_as_usize(0xae235c) as i32).wrapping_sub((read_u32_as_usize(0xae2350) as i32).wrapping_mul(2));
    let base = read_u32_as_usize(0xae23b4);
    let mut i: i16 = 0;
    while (i as i32) < n {
        let c = base + (i as u16 as usize) * 0x245;
        let comp = read_u32_as_usize(c + 0x57);
        let nacao = read_u32_as_usize(c + 0x53);
        if comp != 0 && nacao != 0 && read_u32_as_usize(nacao) == read_u32_as_usize(0x9cf488) && read_u32_as_usize(comp) != read_u32_as_usize(0x9cf624) && read_u32_as_usize(comp) != read_u32_as_usize(0x9cf628) {
            let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x540a50usize);
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

/// { 0x8533d0 } (thiscall, ret 0, spa_lower.cpp, 947 bytes): `SpaLower::create_table()`: a divisão de baixo tem 22
/// clubes (+0x3e), a tabela (malloc de 0x512; sem memória, a caixa da linha 0x2ac e 0). Primeiro, os clubes que estão
/// nela (a competição de id [0x9cf6b0]) passam à de id [0x9cf764] [inferência: a divisão genérica]. Depois enche, na
/// ordem do banco: os times B espanhóis cujo clube de cima está na Primera, depois os da Segunda (cada um com +0x60 =
/// o registro desta competição; os outros times espanhóis de fora das duas, com +0x60 = 0); o resto com os
/// candidatos (cqm6_candidatos), ordenados pela reputação e com os 3 × faltam primeiros embaralhados
/// (comp_util_sort_and_shuffle_top_clubs 0x4afbd0: push faltam; push quantos; push lista; add esp, 0xc -> cdecl; sorteia),
/// cada um com +0x60 = o registro. As linhas: League::init_table_row 0x687430 (push 0; push 0; push clube; push eax
/// (al = a linha, os bits de cima são lixo de outra conta; a 0x687430 só lê o byte); mov ecx, this -> thiscall, ret
/// 0x10). Devolve se encheu.
unsafe extern "thiscall" fn tabela_da_divisao_de_baixo_espanhola(this: usize) -> u32 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let iniciar: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(0x687430usize);
    let embaralhar: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(0x4afbd0usize);
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 0x16);
    let t = alocar(0x512);
    write_u32_from_usize(this + 0xb1, t);
    if t == 0 {
        aviso_em(0xa82880, 0x2ac);
        return 0;
    }
    // os clubes desta divisão passam à genérica
    let n = (read_u32_as_usize(0xae235c) as i32).wrapping_sub((read_u32_as_usize(0xae2350) as i32).wrapping_mul(2));
    let base = read_u32_as_usize(0xae23b4);
    let mut i: i16 = 0;
    while (i as i32) < n {
        let c = base + (i as u16 as usize) * 0x245;
        if cqm6_na(c, 0x9cf6b0) {
            write_u32_from_usize(c + 0x57, cqm6_registro(0x9cf764));
        }
        i = i.wrapping_add(1);
    }
    let mut k: u8 = 0;
    for g in [0x9cf624usize, 0x9cf628] { // cqm6: os times B da Primera e da Segunda
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
unsafe extern "thiscall" fn comparar_tabela_da_divisao_de_baixo_espanhola(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1875);
    let mut plano: Vec<usize> = Vec::new();
    let mut k: u8 = 0;
    for g in [0x9cf624usize, 0x9cf628] {
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
    PLACAR.registrar(ok, || format!("tabela_da_divisao_de_baixo_espanhola({:#x}): devolveu {}, plano {:x?}, tabela {:x?}", this, r, plano,
        (0..plano.len()).map(|i| if t != 0 { read_u32_as_usize(t + i * 0x3b) } else { 0 }).collect::<Vec<_>>()));
    r
}

