// ---------- %%LEVA%%a leva: o calendário da primeira divisão e da Premiership inglesas: as rodadas com o Natal conforme o dia da semana e os play-offs de acesso (eng_first.cpp, eng_prm.cpp) ----------


/// Uma rodada da tabela de 0x41 bytes: a data (dia, mês 0..11, anos a somar ao ano da liga +0x40, dia da semana para
/// onde a data é levada), o código do byte +4 ([inferência] o turno) e as faixas de TV em ordem (dia da semana, turno,
/// quantos jogos; a lista termina em −1).
type Cqp5Rodada = (i8, i8, i8, i8, i8, &'static [(i8, i8, i8)]);

/// As rodadas `primeira`.. da tabela `t`, na ordem da original: para cada uma, a data (0x68a160: push 0; push ano;
/// push código; push semana; push anos; push mês; push dia; push i; push t; add esp -> cdecl: o valor 0) e as
/// faixas (0x68a1c0: push 0; push c; push b; push a; push j; push i; push t; add esp -> cdecl, valor 0).
unsafe fn eng_first_set_league_rounds(t: usize, ano: u32, primeira: i32, tabela: &[Cqp5Rodada]) {
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
unsafe fn cqp5_gravados(t: usize, guia: usize, n: usize) -> Vec<u8> {
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
struct Cqp5Natal {
    comeco: &'static [Cqp5Rodada],
    referencia: usize,
    ajuste: i32,
    casos: [&'static [Cqp5Rodada]; 8],
    fim: &'static [Cqp5Rodada],
    linha: u32,
}

unsafe fn eng_first_set_christmas_rounds(t: usize, ano: u32, n: &Cqp5Natal, arquivo: usize) {
    let construir: unsafe extern "thiscall" fn(*mut [u16; 4], u32, u32, u32, u32) -> usize = core::mem::transmute(0x549f70usize);
    eng_first_set_league_rounds(t, ano, 0, n.comeco);
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
    eng_first_set_league_rounds(t, ano, n.comeco.len() as i32, meio);
    eng_first_set_league_rounds(t, ano, (n.comeco.len() + meio.len()) as i32, n.fim);
}

/// Lado a lado em todas as fases: a original com as saídas de quem chama, a cópia com saídas próprias, a partir do
/// mesmo [0xb67a34]; as saídas e os bytes gravados das duas tabelas têm de ser iguais (na fase −1 as rodadas de liga,
/// `cqp5_gravados`; nas outras as de copa, `eng_cc_cup_written_round_bytes`); a tabela da cópia é solta (0x9452ca, cdecl).
unsafe fn eng_first_compare_round_table(nome: &str, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32,
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
            cqp5_gravados(a, a, n) == cqp5_gravados(b, a, n)
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

/// a primeira divisão inglesa: as rodadas antes do Natal
const CQP5_PRIMEIRA_COMECO: [Cqp5Rodada; 23] = [
    (12, 7, 0, 5, 1, &[(6, 1, 2), (-1, -1, -1)]), // 0: 12/ago
    (19, 7, 0, 5, 1, &[(6, 1, 2), (-1, -1, -1)]), // 1: 19/ago
    (26, 7, 0, 5, 1, &[(4, 2, 2), (6, 1, 2), (-1, -1, -1)]), // 2: 26/ago
    (28, 7, 0, 0, 2, &[(-1, -1, -1)]), // 3: 28/ago
    (2, 8, 0, 5, 1, &[(4, 2, 2), (-1, -1, -1)]), // 4: 2/set
    (9, 8, 0, 5, 1, &[(4, 2, 2), (6, 1, 2), (-1, -1, -1)]), // 5: 9/set
    (13, 8, 0, 2, 2, &[(-1, -1, -1)]), // 6: 13/set
    (16, 8, 0, 5, 1, &[(4, 2, 2), (6, 1, 2), (-1, -1, -1)]), // 7: 16/set
    (23, 8, 0, 5, 1, &[(4, 2, 2), (6, 1, 2), (-1, -1, -1)]), // 8: 23/set
    (30, 8, 0, 5, 1, &[(4, 2, 2), (6, 1, 2), (-1, -1, -1)]), // 9: 30/set
    (7, 9, 0, 5, 1, &[(4, 2, 2), (6, 1, 2), (-1, -1, -1)]), // 10: 7/out
    (14, 9, 0, 5, 1, &[(6, 1, 2), (-1, -1, -1)]), // 11: 14/out
    (17, 9, 0, 1, 2, &[(2, 2, 2), (2, 2, 2), (-1, -1, -1)]), // 12: 17/out
    (21, 9, 0, 5, 1, &[(4, 2, 2), (-1, -1, -1)]), // 13: 21/out
    (24, 9, 0, 1, 2, &[(2, 2, 2), (2, 2, 2), (-1, -1, -1)]), // 14: 24/out
    (28, 9, 0, 5, 1, &[(4, 2, 2), (6, 1, 2), (-1, -1, -1)]), // 15: 28/out
    (4, 10, 0, 5, 1, &[(6, 1, 2), (-1, -1, -1)]), // 16: 4/nov
    (11, 10, 0, 5, 1, &[(4, 2, 2), (6, 1, 2), (-1, -1, -1)]), // 17: 11/nov
    (18, 10, 0, 5, 1, &[(4, 2, 2), (-1, -1, -1)]), // 18: 18/nov
    (25, 10, 0, 5, 1, &[(4, 2, 2), (-1, -1, -1)]), // 19: 25/nov
    (2, 11, 0, 5, 1, &[(6, 1, 2), (-1, -1, -1)]), // 20: 2/dez
    (9, 11, 0, 5, 1, &[(4, 2, 2), (6, 1, 2), (-1, -1, -1)]), // 21: 9/dez
    (16, 11, 0, 5, 1, &[(6, 1, 2), (-1, -1, -1)]), // 22: 16/dez
];

/// as rodadas do Natal quando a rodada 22 (a de referência) cai em 13/dez
const CQP5_PRIMEIRA_NATAL_0: [Cqp5Rodada; 4] = [
    (20, 11, 0, -1, 1, &[(-1, -1, -1)]), // 23: 20/dez
    (27, 11, 0, -1, 1, &[(-1, -1, -1)]), // 24: 27/dez
    (30, 11, 0, -1, 2, &[(-1, -1, -1)]), // 25: 30/dez
    (3, 0, 1, -1, 1, &[(-1, -1, -1)]), // 26: 3/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 22 (a de referência) cai em 14/dez
const CQP5_PRIMEIRA_NATAL_1: [Cqp5Rodada; 4] = [
    (21, 11, 0, -1, 1, &[(-1, -1, -1)]), // 23: 21/dez
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 24: 26/dez
    (28, 11, 0, -1, 1, &[(-1, -1, -1)]), // 25: 28/dez
    (1, 0, 1, -1, 1, &[(-1, -1, -1)]), // 26: 1/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 22 (a de referência) cai em 15/dez
const CQP5_PRIMEIRA_NATAL_2: [Cqp5Rodada; 4] = [
    (22, 11, 0, -1, 1, &[(-1, -1, -1)]), // 23: 22/dez
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 24: 26/dez
    (29, 11, 0, -1, 1, &[(-1, -1, -1)]), // 25: 29/dez
    (1, 0, 1, -1, 1, &[(-1, -1, -1)]), // 26: 1/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 22 (a de referência) cai em 16/dez
const CQP5_PRIMEIRA_NATAL_3: [Cqp5Rodada; 4] = [
    (23, 11, 0, -1, 1, &[(-1, -1, -1)]), // 23: 23/dez
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 24: 26/dez
    (30, 11, 0, -1, 1, &[(-1, -1, -1)]), // 25: 30/dez
    (1, 0, 1, -1, 1, &[(-1, -1, -1)]), // 26: 1/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 22 (a de referência) cai em 17/dez
const CQP5_PRIMEIRA_NATAL_4: [Cqp5Rodada; 4] = [
    (21, 11, 0, -1, 2, &[(-1, -1, -1)]), // 23: 21/dez
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 24: 26/dez
    (31, 11, 0, -1, 1, &[(-1, -1, -1)]), // 25: 31/dez
    (4, 0, 1, -1, 2, &[(-1, -1, -1)]), // 26: 4/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 22 (a de referência) cai em 18/dez
const CQP5_PRIMEIRA_NATAL_5: [Cqp5Rodada; 4] = [
    (22, 11, 0, -1, 2, &[(-1, -1, -1)]), // 23: 22/dez
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 24: 26/dez
    (28, 11, 0, -1, 2, &[(-1, -1, -1)]), // 25: 28/dez
    (1, 0, 1, -1, 1, &[(-1, -1, -1)]), // 26: 1/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 22 (a de referência) cai em 19/dez
const CQP5_PRIMEIRA_NATAL_6: [Cqp5Rodada; 4] = [
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 23: 26/dez
    (28, 11, 0, -1, 2, &[(-1, -1, -1)]), // 24: 28/dez
    (2, 0, 1, -1, 1, &[(-1, -1, -1)]), // 25: 2/jan (+1 ano)
    (5, 0, 1, -1, 2, &[(-1, -1, -1)]), // 26: 5/jan (+1 ano)
];

/// as rodadas do Natal no caso padrão (a rodada 22 fora de 13 a 19/dez: a caixa)
const CQP5_PRIMEIRA_NATAL_PADRAO: [Cqp5Rodada; 4] = [
    (26, 11, 0, 5, 1, &[(-1, -1, -1)]), // 23: 26/dez
    (28, 11, 0, 0, 2, &[(-1, -1, -1)]), // 24: 28/dez
    (2, 0, 1, -1, 1, &[(-1, -1, -1)]), // 25: 2/jan (+1 ano)
    (5, 0, 1, -1, 2, &[(-1, -1, -1)]), // 26: 5/jan (+1 ano)
];

/// a primeira divisão inglesa: as rodadas depois do Natal
const CQP5_PRIMEIRA_FIM: [Cqp5Rodada; 19] = [
    (13, 0, 1, 5, 1, &[(4, 2, 2), (6, 1, 2), (-1, -1, -1)]), // 27: 13/jan (+1 ano)
    (20, 0, 1, 5, 1, &[(4, 2, 2), (6, 1, 2), (-1, -1, -1)]), // 28: 20/jan (+1 ano)
    (3, 1, 1, 5, 1, &[(4, 2, 2), (6, 1, 2), (-1, -1, -1)]), // 29: 3/fev (+1 ano)
    (10, 1, 1, 5, 1, &[(4, 2, 2), (6, 1, 2), (-1, -1, -1)]), // 30: 10/fev (+1 ano)
    (17, 1, 1, 5, 1, &[(4, 2, 2), (6, 1, 2), (-1, -1, -1)]), // 31: 17/fev (+1 ano)
    (20, 1, 1, 1, 2, &[(2, 2, 2), (-1, -1, -1)]), // 32: 20/fev (+1 ano)
    (24, 1, 1, 5, 1, &[(4, 2, 2), (6, 1, 2), (-1, -1, -1)]), // 33: 24/fev (+1 ano)
    (3, 2, 1, 5, 1, &[(4, 2, 2), (6, 1, 2), (-1, -1, -1)]), // 34: 3/mar (+1 ano)
    (7, 2, 1, 2, 2, &[(-1, -1, -1)]), // 35: 7/mar (+1 ano)
    (10, 2, 1, 5, 1, &[(-1, -1, -1)]), // 36: 10/mar (+1 ano)
    (17, 2, 1, 1, 2, &[(2, 2, 2), (2, 2, 2), (-1, -1, -1)]), // 37: 17/mar (+1 ano)
    (24, 2, 1, 5, 1, &[(4, 2, 2), (-1, -1, -1)]), // 38: 24/mar (+1 ano)
    (31, 2, 1, 5, 1, &[(4, 2, 2), (-1, -1, -1)]), // 39: 31/mar (+1 ano)
    (7, 3, 1, 5, 1, &[(4, 2, 2), (6, 1, 2), (-1, -1, -1)]), // 40: 7/abr (+1 ano)
    (14, 3, 1, 5, 1, &[(4, 2, 2), (-1, -1, -1)]), // 41: 14/abr (+1 ano)
    (16, 3, 1, 0, 2, &[(-1, -1, -1)]), // 42: 16/abr (+1 ano)
    (21, 3, 1, 5, 1, &[(4, 2, 2), (-1, -1, -1)]), // 43: 21/abr (+1 ano)
    (28, 3, 1, 5, 1, &[(4, 2, 2), (-1, -1, -1)]), // 44: 28/abr (+1 ano)
    (6, 4, 1, 6, 1, &[(-1, -1, -1)]), // 45: 6/mai (+1 ano)
];

/// a Premiership: as rodadas antes do Natal
const CQP5_PREMIERSHIP_COMECO: [Cqp5Rodada; 18] = [
    (19, 7, 0, 5, 1, &[(-1, -1, -1)]), // 0: 19/ago
    (23, 7, 0, 2, 2, &[(1, 2, 2), (-1, -1, -1)]), // 1: 23/ago
    (26, 7, 0, 5, 1, &[(6, 1, 2), (6, 1, 2), (0, 2, 2), (-1, -1, -1)]), // 2: 26/ago
    (6, 8, 0, 2, 2, &[(1, 2, 2), (-1, -1, -1)]), // 3: 6/set
    (9, 8, 0, 5, 1, &[(6, 1, 2), (6, 1, 2), (0, 2, 2), (-1, -1, -1)]), // 4: 9/set
    (16, 8, 0, 5, 1, &[(6, 1, 2), (6, 1, 2), (0, 2, 2), (-1, -1, -1)]), // 5: 16/set
    (23, 8, 0, 5, 1, &[(6, 1, 2), (6, 1, 2), (0, 2, 2), (-1, -1, -1)]), // 6: 23/set
    (30, 8, 0, 5, 1, &[(6, 1, 2), (-1, -1, -1)]), // 7: 30/set
    (13, 9, 0, 5, 1, &[(6, 1, 2), (6, 1, 2), (0, 2, 2), (-1, -1, -1)]), // 8: 13/out
    (21, 9, 0, 5, 1, &[(6, 1, 2), (6, 1, 2), (0, 2, 2), (-1, -1, -1)]), // 9: 21/out
    (28, 9, 0, 5, 1, &[(6, 1, 2), (6, 1, 2), (0, 2, 2), (-1, -1, -1)]), // 10: 28/out
    (4, 10, 0, 5, 1, &[(6, 1, 2), (6, 1, 2), (0, 2, 2), (-1, -1, -1)]), // 11: 4/nov
    (11, 10, 0, 5, 1, &[(6, 1, 2), (6, 1, 2), (0, 2, 2), (-1, -1, -1)]), // 12: 11/nov
    (18, 10, 0, 5, 1, &[(6, 1, 2), (6, 1, 2), (0, 2, 2), (-1, -1, -1)]), // 13: 18/nov
    (25, 10, 0, 5, 1, &[(6, 1, 2), (6, 1, 2), (0, 2, 2), (-1, -1, -1)]), // 14: 25/nov
    (2, 11, 0, 5, 1, &[(6, 1, 2), (6, 1, 2), (0, 2, 2), (-1, -1, -1)]), // 15: 2/dez
    (9, 11, 0, 5, 1, &[(6, 1, 2), (6, 1, 2), (0, 2, 2), (-1, -1, -1)]), // 16: 9/dez
    (16, 11, 0, 5, 1, &[(6, 1, 2), (6, 1, 2), (0, 2, 2), (-1, -1, -1)]), // 17: 16/dez
];

/// as rodadas do Natal quando a rodada 17 (a de referência) cai em 13/dez
const CQP5_PREMIERSHIP_NATAL_0: [Cqp5Rodada; 4] = [
    (20, 11, 0, -1, 1, &[(-1, -1, -1)]), // 18: 20/dez
    (27, 11, 0, -1, 1, &[(-1, -1, -1)]), // 19: 27/dez
    (30, 11, 0, -1, 2, &[(-1, -1, -1)]), // 20: 30/dez
    (3, 0, 1, -1, 1, &[(-1, -1, -1)]), // 21: 3/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 17 (a de referência) cai em 14/dez
const CQP5_PREMIERSHIP_NATAL_1: [Cqp5Rodada; 4] = [
    (21, 11, 0, -1, 1, &[(-1, -1, -1)]), // 18: 21/dez
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 19: 26/dez
    (28, 11, 0, -1, 1, &[(-1, -1, -1)]), // 20: 28/dez
    (1, 0, 1, -1, 1, &[(-1, -1, -1)]), // 21: 1/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 17 (a de referência) cai em 15/dez
const CQP5_PREMIERSHIP_NATAL_2: [Cqp5Rodada; 4] = [
    (22, 11, 0, -1, 1, &[(-1, -1, -1)]), // 18: 22/dez
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 19: 26/dez
    (29, 11, 0, -1, 1, &[(-1, -1, -1)]), // 20: 29/dez
    (1, 0, 1, -1, 1, &[(-1, -1, -1)]), // 21: 1/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 17 (a de referência) cai em 16/dez
const CQP5_PREMIERSHIP_NATAL_3: [Cqp5Rodada; 4] = [
    (23, 11, 0, -1, 1, &[(-1, -1, -1)]), // 18: 23/dez
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 19: 26/dez
    (30, 11, 0, -1, 1, &[(-1, -1, -1)]), // 20: 30/dez
    (1, 0, 1, -1, 1, &[(-1, -1, -1)]), // 21: 1/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 17 (a de referência) cai em 17/dez
const CQP5_PREMIERSHIP_NATAL_4: [Cqp5Rodada; 4] = [
    (21, 11, 0, -1, 2, &[(-1, -1, -1)]), // 18: 21/dez
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 19: 26/dez
    (31, 11, 0, -1, 1, &[(-1, -1, -1)]), // 20: 31/dez
    (4, 0, 1, -1, 2, &[(-1, -1, -1)]), // 21: 4/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 17 (a de referência) cai em 18/dez
const CQP5_PREMIERSHIP_NATAL_5: [Cqp5Rodada; 4] = [
    (22, 11, 0, -1, 2, &[(-1, -1, -1)]), // 18: 22/dez
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 19: 26/dez
    (28, 11, 0, -1, 2, &[(-1, -1, -1)]), // 20: 28/dez
    (1, 0, 1, -1, 1, &[(-1, -1, -1)]), // 21: 1/jan (+1 ano)
];

/// as rodadas do Natal quando a rodada 17 (a de referência) cai em 19/dez
const CQP5_PREMIERSHIP_NATAL_6: [Cqp5Rodada; 4] = [
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 18: 26/dez
    (28, 11, 0, -1, 2, &[(-1, -1, -1)]), // 19: 28/dez
    (2, 0, 1, -1, 1, &[(-1, -1, -1)]), // 20: 2/jan (+1 ano)
    (5, 0, 1, -1, 2, &[(-1, -1, -1)]), // 21: 5/jan (+1 ano)
];

/// as rodadas do Natal no caso padrão (a rodada 17 fora de 13 a 19/dez: a caixa)
const CQP5_PREMIERSHIP_NATAL_PADRAO: [Cqp5Rodada; 4] = [
    (26, 11, 0, 5, 1, &[(-1, -1, -1)]), // 18: 26/dez
    (28, 11, 0, 0, 2, &[(-1, -1, -1)]), // 19: 28/dez
    (2, 0, 1, -1, 1, &[(-1, -1, -1)]), // 20: 2/jan (+1 ano)
    (5, 0, 1, -1, 2, &[(-1, -1, -1)]), // 21: 5/jan (+1 ano)
];

/// a Premiership: as rodadas depois do Natal
const CQP5_PREMIERSHIP_FIM: [Cqp5Rodada; 16] = [
    (13, 0, 1, 5, 1, &[(6, 1, 2), (6, 1, 2), (0, 2, 2), (-1, -1, -1)]), // 22: 13/jan (+1 ano)
    (20, 0, 1, 5, 1, &[(6, 1, 2), (6, 1, 2), (0, 2, 2), (-1, -1, -1)]), // 23: 20/jan (+1 ano)
    (31, 0, 1, 5, 1, &[(6, 1, 2), (6, 1, 2), (0, 2, 2), (-1, -1, -1)]), // 24: 31/jan (+1 ano)
    (3, 1, 1, 5, 1, &[(6, 1, 2), (6, 1, 2), (0, 2, 2), (-1, -1, -1)]), // 25: 3/fev (+1 ano)
    (10, 1, 1, 5, 1, &[(6, 1, 2), (6, 1, 2), (0, 2, 2), (-1, -1, -1)]), // 26: 10/fev (+1 ano)
    (24, 1, 1, 5, 1, &[(6, 1, 2), (6, 1, 2), (0, 2, 2), (-1, -1, -1)]), // 27: 24/fev (+1 ano)
    (3, 2, 1, 5, 1, &[(6, 1, 2), (6, 1, 2), (0, 2, 2), (-1, -1, -1)]), // 28: 3/mar (+1 ano)
    (17, 2, 1, 5, 1, &[(6, 1, 2), (6, 1, 2), (0, 2, 2), (-1, -1, -1)]), // 29: 17/mar (+1 ano)
    (31, 2, 1, 5, 1, &[(6, 1, 2), (6, 1, 2), (0, 2, 2), (-1, -1, -1)]), // 30: 31/mar (+1 ano)
    (7, 3, 1, 5, 1, &[(6, 1, 2), (6, 1, 2), (0, 2, 2), (-1, -1, -1)]), // 31: 7/abr (+1 ano)
    (14, 3, 1, 5, 1, &[(-1, -1, -1)]), // 32: 14/abr (+1 ano)
    (18, 3, 1, 2, 2, &[(-1, -1, -1)]), // 33: 18/abr (+1 ano)
    (21, 3, 1, 5, 1, &[(6, 1, 2), (6, 1, 2), (0, 2, 2), (-1, -1, -1)]), // 34: 21/abr (+1 ano)
    (28, 3, 1, 5, 1, &[(6, 1, 2), (6, 1, 2), (0, 2, 2), (-1, -1, -1)]), // 35: 28/abr (+1 ano)
    (5, 4, 1, 5, 1, &[(6, 1, 2), (6, 1, 2), (0, 2, 2), (-1, -1, -1)]), // 36: 5/mai (+1 ano)
    (20, 4, 1, 6, 1, &[(-1, -1, -1)]), // 37: 20/mai (+1 ano)
];

const CQP5_PRIMEIRA_NATAL: Cqp5Natal = Cqp5Natal { comeco: &CQP5_PRIMEIRA_COMECO, referencia: 22, ajuste: 13, casos: [&CQP5_PRIMEIRA_NATAL_0, &CQP5_PRIMEIRA_NATAL_1, &CQP5_PRIMEIRA_NATAL_2, &CQP5_PRIMEIRA_NATAL_3, &CQP5_PRIMEIRA_NATAL_4, &CQP5_PRIMEIRA_NATAL_5, &CQP5_PRIMEIRA_NATAL_6, &CQP5_PRIMEIRA_NATAL_PADRAO], fim: &CQP5_PRIMEIRA_FIM, linha: 0x17c };
const CQP5_PRIMEIRA_PLAYOFF_0: Cql10R = cql10r((7, 4, 1, 0), (0xc, 4, 1, 5, 1, 0), 0x82, (0, 0x204, 3), 5, (4, 2, 4, 0), (0, 2, 4), (0, 0, 0));
const CQP5_PRIMEIRA_PLAYOFF_1: Cql10R = cql10r((0x11, 4, 1, 3), (0x1c, 4, 1, 0, 2, 4), 0x96, (0, 3, 0), 5, (2, 1, 0, 0), (0, 1, 0), (0, 0, 0));
const CQP5_PREMIERSHIP_NATAL: Cqp5Natal = Cqp5Natal { comeco: &CQP5_PREMIERSHIP_COMECO, referencia: 17, ajuste: 13, casos: [&CQP5_PREMIERSHIP_NATAL_0, &CQP5_PREMIERSHIP_NATAL_1, &CQP5_PREMIERSHIP_NATAL_2, &CQP5_PREMIERSHIP_NATAL_3, &CQP5_PREMIERSHIP_NATAL_4, &CQP5_PREMIERSHIP_NATAL_5, &CQP5_PREMIERSHIP_NATAL_6, &CQP5_PREMIERSHIP_NATAL_PADRAO], fim: &CQP5_PREMIERSHIP_FIM, linha: 0x173 };

/// { 0x571980 } (thiscall, ret 0x10, eng_first.cpp, 4.688 bytes, método +0x3c): `EngFirst::create_schedule(fase,
/// &rodadas, &extra, &é_liga)`: a fase −1, 46 rodadas (`CQP5_PRIMEIRA_NATAL`, as datas da segunda com faixas de TV);
/// a fase 0, os play-offs; as outras devolvem 0. Fonte 0x9c4374.
unsafe extern "thiscall" fn eng_first_create_schedule(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    match fase as u8 {
        0xff => {
            if !liga.is_null() {
                *liga = 1;
            }
            *rodadas = 46;
            *extra = 0;
            let t = alocar(0xbae);
            if t == 0 {
                show_error_at(0x9c4374, 0xae);
                return 0;
            }
            crate::comparacao::ramo("cqp_5 calendario da primeira inglesa");
            let ano = core::ptr::read_unaligned((this + 0x40) as *const u16) as u32;
            eng_first_set_christmas_rounds(t, ano, &CQP5_PRIMEIRA_NATAL, 0x9c4374);
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
                show_error_at(0x9c4374, 0x1ed);
                return 0;
            }
            crate::comparacao::ramo("cqp_5 play-offs da primeira inglesa");
            let ano = core::ptr::read_unaligned((this + 0x40) as *const i16) as i32 as u32;
            eng_cc_cup_write_cup_round(t, 0, &CQP5_PRIMEIRA_PLAYOFF_0, ano);
            eng_cc_cup_write_cup_round(t, 1, &CQP5_PRIMEIRA_PLAYOFF_1, ano);
            t
        }
        _ => 0,
    }
}

unsafe extern "thiscall" fn comparar_eng_first_create_schedule(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    eng_first_compare_round_table("eng_first_create_schedule", fase, rodadas, extra, liga, || o(this, fase, rodadas, extra, liga), |r, e, l| eng_first_create_schedule(this, fase, r, e, l))
}

/// { 0x573660 } (thiscall, ret 0x10, eng_prm.cpp, 5.019 bytes, método +0x3c): `EngPrm::create_schedule(fase, &rodadas,
/// &extra, &é_liga)`: só a fase −1, 38 rodadas (`CQP5_PREMIERSHIP_NATAL`); as outras devolvem 0. Fonte 0x9c43b0.
unsafe extern "thiscall" fn eng_prm_create_schedule(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    match fase as u8 {
        0xff => {
            if !liga.is_null() {
                *liga = 1;
            }
            *rodadas = 38;
            *extra = 0;
            let t = alocar(0x9a6);
            if t == 0 {
                show_error_at(0x9c43b0, 0xa7);
                return 0;
            }
            crate::comparacao::ramo("cqp_5 calendario da premiership");
            let ano = core::ptr::read_unaligned((this + 0x40) as *const u16) as u32;
            eng_first_set_christmas_rounds(t, ano, &CQP5_PREMIERSHIP_NATAL, 0x9c43b0);
            t
        }
        _ => 0,
    }
}

unsafe extern "thiscall" fn comparar_eng_prm_create_schedule(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I1%%);
    eng_first_compare_round_table("eng_prm_create_schedule", fase, rodadas, extra, liga, || o(this, fase, rodadas, extra, liga), |r, e, l| eng_prm_create_schedule(this, fase, r, e, l))
}
