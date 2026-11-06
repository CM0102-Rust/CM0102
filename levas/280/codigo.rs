// ---------- 280a leva: o calendário da primeira divisão e da Premiership inglesas: as rodadas com o Natal conforme o dia da semana e os play-offs de acesso (eng_first.cpp, eng_prm.cpp) ----------


/// Uma rodada da tabela de 0x41 bytes: a data (dia, mês 0..11, anos a somar ao ano da liga +0x40, dia da semana para
/// onde a data é levada), o código do byte +4 ([inferência] o turno) e as faixas de TV em ordem (dia da semana, turno,
/// quantos jogos; a lista termina em −1).
type Cqp5Rodada = (i8, i8, i8, i8, i8, &'static [(i8, i8, i8)]);

/// As rodadas `primeira`.. da tabela `t`, na ordem da original: para cada uma, a data (0x68a160: push 0; push ano;
/// push código; push semana; push anos; push mês; push dia; push i; push t; add esp -> cdecl: o valor 0) e as
/// faixas (0x68a1c0: push 0; push c; push b; push a; push j; push i; push t; add esp -> cdecl, valor 0).
unsafe fn cqp5_rodadas(t: usize, ano: u32, primeira: i32, tabela: &[Cqp5Rodada]) {
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

unsafe fn cqp5_natal(t: usize, ano: u32, n: &Cqp5Natal, arquivo: usize) {
    let construir: unsafe extern "thiscall" fn(*mut [u16; 4], u32, u32, u32, u32) -> usize = core::mem::transmute(0x549f70usize);
    cqp5_rodadas(t, ano, 0, n.comeco);
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
    cqp5_rodadas(t, ano, n.comeco.len() as i32, meio);
    cqp5_rodadas(t, ano, (n.comeco.len() + meio.len()) as i32, n.fim);
}

/// Lado a lado em todas as fases: a original com as saídas de quem chama, a cópia com saídas próprias, a partir do
/// mesmo [0xb67a34]; as saídas e os bytes gravados das duas tabelas têm de ser iguais (na fase −1 as rodadas de liga,
/// `cqp5_gravados`; nas outras as de copa, `cql10_gravados`); a tabela da cópia é solta (0x9452ca, cdecl).
unsafe fn cqp5_comparar(nome: &str, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32,
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
unsafe extern "thiscall" fn calendario_da_primeira_inglesa(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
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
                aviso_em(0x9c4374, 0xae);
                return 0;
            }
            crate::comparacao::ramo("cqp_5 calendario da primeira inglesa");
            let ano = core::ptr::read_unaligned((this + 0x40) as *const u16) as u32;
            cqp5_natal(t, ano, &CQP5_PRIMEIRA_NATAL, 0x9c4374);
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
                aviso_em(0x9c4374, 0x1ed);
                return 0;
            }
            crate::comparacao::ramo("cqp_5 play-offs da primeira inglesa");
            let ano = core::ptr::read_unaligned((this + 0x40) as *const i16) as i32 as u32;
            cql10_rodada(t, 0, &CQP5_PRIMEIRA_PLAYOFF_0, ano);
            cql10_rodada(t, 1, &CQP5_PRIMEIRA_PLAYOFF_1, ano);
            t
        }
        _ => 0,
    }
}

unsafe extern "thiscall" fn comparar_calendario_da_primeira_inglesa(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1923);
    cqp5_comparar("calendario_da_primeira_inglesa", fase, rodadas, extra, liga, || o(this, fase, rodadas, extra, liga), |r, e, l| calendario_da_primeira_inglesa(this, fase, r, e, l))
}

/// { 0x573660 } (thiscall, ret 0x10, eng_prm.cpp, 5.019 bytes, método +0x3c): `EngPrm::create_schedule(fase, &rodadas,
/// &extra, &é_liga)`: só a fase −1, 38 rodadas (`CQP5_PREMIERSHIP_NATAL`); as outras devolvem 0. Fonte 0x9c43b0.
unsafe extern "thiscall" fn calendario_da_premiership(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
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
                aviso_em(0x9c43b0, 0xa7);
                return 0;
            }
            crate::comparacao::ramo("cqp_5 calendario da premiership");
            let ano = core::ptr::read_unaligned((this + 0x40) as *const u16) as u32;
            cqp5_natal(t, ano, &CQP5_PREMIERSHIP_NATAL, 0x9c43b0);
            t
        }
        _ => 0,
    }
}

unsafe extern "thiscall" fn comparar_calendario_da_premiership(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1924);
    cqp5_comparar("calendario_da_premiership", fase, rodadas, extra, liga, || o(this, fase, rodadas, extra, liga), |r, e, l| calendario_da_premiership(this, fase, r, e, l))
}

// ---------- 280a leva: o calendário da Primera espanhola: as 38 rodadas, uma tabela para o ano par e outra para o ímpar, com a rodada do Natal conforme o dia da semana (spa_first.cpp) ----------


/// Uma rodada da tabela de 0x41 bytes: a data (dia, mês 0..11, anos a somar ao ano da liga +0x40, dia da semana para
/// onde a data é levada), o código do byte +4 ([inferência] o turno) e as faixas de TV em ordem (dia da semana, turno,
/// quantos jogos; a lista termina em −1).
type Cqp6Rodada = (i8, i8, i8, i8, i8, &'static [(i8, i8, i8)]);

/// As rodadas `primeira`.. da tabela `t`, na ordem da original: para cada uma, a data (0x68a160: push 0; push ano;
/// push código; push semana; push anos; push mês; push dia; push i; push t; add esp -> cdecl: o valor 0) e as
/// faixas (0x68a1c0: push 0; push c; push b; push a; push j; push i; push t; add esp -> cdecl, valor 0).
unsafe fn cqp6_rodadas(t: usize, ano: u32, primeira: i32, tabela: &[Cqp6Rodada]) {
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
    let construir: unsafe extern "thiscall" fn(*mut [u16; 4], u32, u32, u32, u32) -> usize = core::mem::transmute(0x549f70usize);
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

// ---------- 280a leva: a virada da temporada e a fase seguinte dos estaduais: Baiano, Centro-Oeste e Gaúcho (bra_reg_bahia.cpp, bra_reg_central.cpp, bra_reg_gaucho.cpp) ----------

unsafe fn cqp7_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// A virada da temporada de um estadual (`start_new_season`, slot 3, +0x08; o mesmo molde nos nove estaduais de pontos
/// corridos com mata-mata: só mudam o fonte, as linhas das caixas e a rotina da tabela, conferido instrução a instrução):
/// +0x4c = 0 e a tabela liberada (League::free_table 0x687970, thiscall(this, 0), ret 4); a lista de +0xba liberada
/// (free 0x9452ca) e zerada; com estatísticas (+8), Comp_Stats::archive_season_and_reset (0x4a1c50, thiscall(+8, 1),
/// ret 4); com a fase atual ≥ 0, as fases 0..=+0x30 que existem destruídas (a virtual 0, o destrutor com 1: libera);
/// o ano (+0x40, word) sobe um (`ano`; o contador `ramo`) e +0x30 = −1; então, cada um com a sua caixa se devolver 0: as regras da temporada
/// (virtual +0x8c, set_up_rules), a tabela com os clubes do estadual (`tabela`, thiscall, ret 0), os jogos
/// (League::create_fixtures 0x6835c0, thiscall, ret 0) e a ordem da tabela (League::sort_table 0x6827d0, thiscall(this,
/// 0), ret 4); por fim as posições previstas (virtual +0x5c) e a renovação dos árbitros do estadual
/// (OfficialsManager::renew_officials 0x79cee0, thiscall([0xb74340], +4), ret 4), cujo eax ela devolve.
unsafe fn cqp7_virada(this: usize, tabela: usize, arquivo: usize, linhas: [u32; 4], ano: i16, ramo: &'static str) -> u32 {
    let liberar_tabela: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x687970usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    core::ptr::write_unaligned((this + 0x4c) as *mut u32, 0);
    liberar_tabela(this, 0);
    let l = cqp7_d(this + 0xba);
    if l != 0 {
        liberar(l);
        core::ptr::write_unaligned((this + 0xba) as *mut usize, 0);
    }
    let estatisticas = cqp7_d(this + 8);
    if estatisticas != 0 {
        let arquivar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x4a1c50usize);
        arquivar(estatisticas, 1);
    }
    if (cqp7_d(this + 0x30) as i32) >= 0 {
        let mut i = 0i32;
        loop {
            let f = cqp7_d(cqp7_d(this + 0xc) + 4 * i as usize);
            if f != 0 {
                let destruir: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(cqp7_d(cqp7_d(f)));
                destruir(f, 1);
            }
            i += 1;
            if i > cqp7_d(this + 0x30) as i32 {
                break;
            }
        }
    }
    let a = core::ptr::read_unaligned((this + 0x40) as *const i16);
    core::ptr::write_unaligned((this + 0x40) as *mut i16, a.wrapping_add(ano));
    crate::comparacao::ramo(ramo);
    core::ptr::write_unaligned((this + 0x30) as *mut i32, -1);
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cqp7_d(cqp7_d(this) + 0x8c));
    if regras(this) == 0 {
        aviso_em(arquivo, linhas[0]);
        return 0;
    }
    let tabela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(tabela);
    if tabela(this) == 0 {
        aviso_em(arquivo, linhas[1]);
        return 0;
    }
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x6835c0usize);
    if jogos(this) == 0 {
        aviso_em(arquivo, linhas[2]);
        return 0;
    }
    let ordenar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x6827d0usize);
    if ordenar(this, 0) == 0 {
        aviso_em(arquivo, linhas[3]);
        return 0;
    }
    let previstas: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cqp7_d(cqp7_d(this) + 0x5c));
    previstas(this);
    let renovar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x79cee0usize);
    renovar(cqp7_d(0xb74340), cqp7_d(this + 4))
}

/// A fase seguinte de um estadual (`advance_stage`, slot 11, +0x28): na última fase (+0x30 ≥ +0x2c − 1, com sinal), a
/// caixa de erro e nada mais; senão +0x30 sobe um e, ao chegar a 0 (o fim dos pontos corridos), monta o mata-mata
/// (`mata_mata`, call direto com o ecx da liga -> thiscall, ret 0).
unsafe fn cqp7_fase_seguinte(this: usize, arquivo: usize, linha: u32, mata_mata: usize) {
    let fase = cqp7_d(this + 0x30) as i32;
    if fase >= (cqp7_d(this + 0x2c) as i32).wrapping_sub(1) {
        return aviso_em(arquivo, linha);
    }
    let fase = fase + 1;
    core::ptr::write_unaligned((this + 0x30) as *mut i32, fase);
    if fase == 0 {
        let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(mata_mata);
        f(this);
    }
}

/// O comparador da virada: só a original (ponytail: libera tabelas e fases, aloca as novas, sorteia os jogos e os
/// árbitros; não pode rodar duas vezes), conferindo o que só ela garante: o ano (+0x40) sobe um.
unsafe fn cqp7_comparar_virada(nome: &str, this: usize, original: impl FnOnce() -> u32) -> u32 {
    let antes = core::ptr::read_unaligned((this + 0x40) as *const i16);
    let r = original();
    let depois = core::ptr::read_unaligned((this + 0x40) as *const i16);
    PLACAR.registrar(depois == antes.wrapping_add(1), || format!("{}({:#x}) ano antes={} depois={}", nome, this, antes, depois));
    r
}

/// O comparador da fase seguinte: só a original (ponytail: ao chegar a 0 monta o mata-mata, que aloca e sorteia), com o
/// plano: na última fase, +0x30 igual; senão +0x30 + 1 (o mata-mata grava 0, o mesmo valor).
unsafe fn cqp7_comparar_fase_seguinte(nome: &str, this: usize, original: impl FnOnce()) {
    let antes = cqp7_d(this + 0x30) as i32;
    let ultima = antes >= (cqp7_d(this + 0x2c) as i32).wrapping_sub(1);
    original();
    let depois = cqp7_d(this + 0x30) as i32;
    PLACAR.registrar(depois == if ultima { antes } else { antes + 1 }, || format!("{}({:#x}) antes={} depois={}", nome, this, antes, depois));
}

/// { 0x42fe10 } (thiscall, ret 0, bra_reg_bahia.cpp): `BraRegBahia::start_new_season()`, a virada da temporada do Baiano (a tabela:
/// 0x42fd10; caixas nas linhas 0x1ca, 0x1d1, 0x1d7 e 0x1de).
unsafe extern "thiscall" fn cqp7_virada_do_baiano(this: usize) -> u32 {
    cqp7_virada(this, 0x42fd10, 0x98acac, [0x1ca, 0x1d1, 0x1d7, 0x1de], 1, "cqp_7 virada do baiano") // cqp7: o ano seguinte do Baiano
}

/// { 0x42ffc0 } (thiscall, ret 0, bra_reg_bahia.cpp): `BraRegBahia::advance_stage()`, a fase seguinte do Baiano: no fim dos pontos corridos,
/// o mata-mata de 4 (0x430050); a caixa da linha 0x1f8.
unsafe extern "thiscall" fn cqp7_fase_seguinte_do_baiano(this: usize) {
    cqp7_fase_seguinte(this, 0x98acac, 0x1f8, 0x430050);
}

/// { 0x431090 } (thiscall, ret 0, bra_reg_central.cpp): `BraRegCentral::start_new_season()`, a virada da temporada do Centro-Oeste (a tabela:
/// 0x430ef0; caixas nas linhas 0x1cb, 0x1d2, 0x1d8 e 0x1df).
unsafe extern "thiscall" fn cqp7_virada_do_centro_oeste(this: usize) -> u32 {
    cqp7_virada(this, 0x430ef0, 0x98ae10, [0x1cb, 0x1d2, 0x1d8, 0x1df], 1, "cqp_7 virada do centro_oeste") // cqp7: o ano seguinte do Centro-Oeste
}

/// { 0x431240 } (thiscall, ret 0, bra_reg_central.cpp): `BraRegCentral::advance_stage()`, a fase seguinte do Centro-Oeste: no fim dos pontos corridos,
/// o mata-mata de 8 (0x4312d0); a caixa da linha 0x1f9.
unsafe extern "thiscall" fn cqp7_fase_seguinte_do_centro_oeste(this: usize) {
    cqp7_fase_seguinte(this, 0x98ae10, 0x1f9, 0x4312d0);
}

/// { 0x432370 } (thiscall, ret 0, bra_reg_gaucho.cpp): `BraRegGaucho::start_new_season()`, a virada da temporada do Gaúcho (a tabela:
/// 0x4321d0; caixas nas linhas 0x1c6, 0x1cd, 0x1d3 e 0x1da).
unsafe extern "thiscall" fn cqp7_virada_do_gaucho(this: usize) -> u32 {
    cqp7_virada(this, 0x4321d0, 0x98ae50, [0x1c6, 0x1cd, 0x1d3, 0x1da], 1, "cqp_7 virada do gaucho") // cqp7: o ano seguinte do Gaúcho
}

/// { 0x432520 } (thiscall, ret 0, bra_reg_gaucho.cpp): `BraRegGaucho::advance_stage()`, a fase seguinte do Gaúcho: no fim dos pontos corridos,
/// o mata-mata de 4 (0x4325b0); a caixa da linha 0x1f4.
unsafe extern "thiscall" fn cqp7_fase_seguinte_do_gaucho(this: usize) {
    cqp7_fase_seguinte(this, 0x98ae50, 0x1f4, 0x4325b0);
}

unsafe extern "thiscall" fn comparar_cqp7_virada_do_baiano(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1926);
    cqp7_comparar_virada("cqp7_virada_do_baiano", this, || o(this))
}

unsafe extern "thiscall" fn comparar_cqp7_fase_seguinte_do_baiano(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1927);
    cqp7_comparar_fase_seguinte("cqp7_fase_seguinte_do_baiano", this, || o(this))
}

unsafe extern "thiscall" fn comparar_cqp7_virada_do_centro_oeste(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1928);
    cqp7_comparar_virada("cqp7_virada_do_centro_oeste", this, || o(this))
}

unsafe extern "thiscall" fn comparar_cqp7_fase_seguinte_do_centro_oeste(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1929);
    cqp7_comparar_fase_seguinte("cqp7_fase_seguinte_do_centro_oeste", this, || o(this))
}

unsafe extern "thiscall" fn comparar_cqp7_virada_do_gaucho(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1930);
    cqp7_comparar_virada("cqp7_virada_do_gaucho", this, || o(this))
}

unsafe extern "thiscall" fn comparar_cqp7_fase_seguinte_do_gaucho(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1931);
    cqp7_comparar_fase_seguinte("cqp7_fase_seguinte_do_gaucho", this, || o(this))
}

// ---------- 280a leva: a virada da temporada e a fase seguinte dos estaduais: Goiano, Norte e Nordeste (bra_reg_goias.cpp, bra_reg_north.cpp, bra_reg_northeast.cpp) ----------

unsafe fn cqp8_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// A virada da temporada de um estadual (`start_new_season`, slot 3, +0x08; o mesmo molde nos nove estaduais de pontos
/// corridos com mata-mata: só mudam o fonte, as linhas das caixas e a rotina da tabela, conferido instrução a instrução):
/// +0x4c = 0 e a tabela liberada (League::free_table 0x687970, thiscall(this, 0), ret 4); a lista de +0xba liberada
/// (free 0x9452ca) e zerada; com estatísticas (+8), Comp_Stats::archive_season_and_reset (0x4a1c50, thiscall(+8, 1),
/// ret 4); com a fase atual ≥ 0, as fases 0..=+0x30 que existem destruídas (a virtual 0, o destrutor com 1: libera);
/// o ano (+0x40, word) sobe um (`ano`; o contador `ramo`) e +0x30 = −1; então, cada um com a sua caixa se devolver 0: as regras da temporada
/// (virtual +0x8c, set_up_rules), a tabela com os clubes do estadual (`tabela`, thiscall, ret 0), os jogos
/// (League::create_fixtures 0x6835c0, thiscall, ret 0) e a ordem da tabela (League::sort_table 0x6827d0, thiscall(this,
/// 0), ret 4); por fim as posições previstas (virtual +0x5c) e a renovação dos árbitros do estadual
/// (OfficialsManager::renew_officials 0x79cee0, thiscall([0xb74340], +4), ret 4), cujo eax ela devolve.
unsafe fn cqp8_virada(this: usize, tabela: usize, arquivo: usize, linhas: [u32; 4], ano: i16, ramo: &'static str) -> u32 {
    let liberar_tabela: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x687970usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    core::ptr::write_unaligned((this + 0x4c) as *mut u32, 0);
    liberar_tabela(this, 0);
    let l = cqp8_d(this + 0xba);
    if l != 0 {
        liberar(l);
        core::ptr::write_unaligned((this + 0xba) as *mut usize, 0);
    }
    let estatisticas = cqp8_d(this + 8);
    if estatisticas != 0 {
        let arquivar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x4a1c50usize);
        arquivar(estatisticas, 1);
    }
    if (cqp8_d(this + 0x30) as i32) >= 0 {
        let mut i = 0i32;
        loop {
            let f = cqp8_d(cqp8_d(this + 0xc) + 4 * i as usize);
            if f != 0 {
                let destruir: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(cqp8_d(cqp8_d(f)));
                destruir(f, 1);
            }
            i += 1;
            if i > cqp8_d(this + 0x30) as i32 {
                break;
            }
        }
    }
    let a = core::ptr::read_unaligned((this + 0x40) as *const i16);
    core::ptr::write_unaligned((this + 0x40) as *mut i16, a.wrapping_add(ano));
    crate::comparacao::ramo(ramo);
    core::ptr::write_unaligned((this + 0x30) as *mut i32, -1);
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cqp8_d(cqp8_d(this) + 0x8c));
    if regras(this) == 0 {
        aviso_em(arquivo, linhas[0]);
        return 0;
    }
    let tabela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(tabela);
    if tabela(this) == 0 {
        aviso_em(arquivo, linhas[1]);
        return 0;
    }
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x6835c0usize);
    if jogos(this) == 0 {
        aviso_em(arquivo, linhas[2]);
        return 0;
    }
    let ordenar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x6827d0usize);
    if ordenar(this, 0) == 0 {
        aviso_em(arquivo, linhas[3]);
        return 0;
    }
    let previstas: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cqp8_d(cqp8_d(this) + 0x5c));
    previstas(this);
    let renovar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x79cee0usize);
    renovar(cqp8_d(0xb74340), cqp8_d(this + 4))
}

/// A fase seguinte de um estadual (`advance_stage`, slot 11, +0x28): na última fase (+0x30 ≥ +0x2c − 1, com sinal), a
/// caixa de erro e nada mais; senão +0x30 sobe um e, ao chegar a 0 (o fim dos pontos corridos), monta o mata-mata
/// (`mata_mata`, call direto com o ecx da liga -> thiscall, ret 0).
unsafe fn cqp8_fase_seguinte(this: usize, arquivo: usize, linha: u32, mata_mata: usize) {
    let fase = cqp8_d(this + 0x30) as i32;
    if fase >= (cqp8_d(this + 0x2c) as i32).wrapping_sub(1) {
        return aviso_em(arquivo, linha);
    }
    let fase = fase + 1;
    core::ptr::write_unaligned((this + 0x30) as *mut i32, fase);
    if fase == 0 {
        let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(mata_mata);
        f(this);
    }
}

/// O comparador da virada: só a original (ponytail: libera tabelas e fases, aloca as novas, sorteia os jogos e os
/// árbitros; não pode rodar duas vezes), conferindo o que só ela garante: o ano (+0x40) sobe um.
unsafe fn cqp8_comparar_virada(nome: &str, this: usize, original: impl FnOnce() -> u32) -> u32 {
    let antes = core::ptr::read_unaligned((this + 0x40) as *const i16);
    let r = original();
    let depois = core::ptr::read_unaligned((this + 0x40) as *const i16);
    PLACAR.registrar(depois == antes.wrapping_add(1), || format!("{}({:#x}) ano antes={} depois={}", nome, this, antes, depois));
    r
}

/// O comparador da fase seguinte: só a original (ponytail: ao chegar a 0 monta o mata-mata, que aloca e sorteia), com o
/// plano: na última fase, +0x30 igual; senão +0x30 + 1 (o mata-mata grava 0, o mesmo valor).
unsafe fn cqp8_comparar_fase_seguinte(nome: &str, this: usize, original: impl FnOnce()) {
    let antes = cqp8_d(this + 0x30) as i32;
    let ultima = antes >= (cqp8_d(this + 0x2c) as i32).wrapping_sub(1);
    original();
    let depois = cqp8_d(this + 0x30) as i32;
    PLACAR.registrar(depois == if ultima { antes } else { antes + 1 }, || format!("{}({:#x}) antes={} depois={}", nome, this, antes, depois));
}

/// { 0x4336f0 } (thiscall, ret 0, bra_reg_goias.cpp): `BraRegGoias::start_new_season()`, a virada da temporada do Goiano (a tabela:
/// 0x433540; caixas nas linhas 0x1c3, 0x1ca, 0x1d0 e 0x1d7).
unsafe extern "thiscall" fn cqp8_virada_do_goiano(this: usize) -> u32 {
    cqp8_virada(this, 0x433540, 0x98ae90, [0x1c3, 0x1ca, 0x1d0, 0x1d7], 1, "cqp_8 virada do goiano") // cqp8: o ano seguinte do Goiano
}

/// { 0x4338a0 } (thiscall, ret 0, bra_reg_goias.cpp): `BraRegGoias::advance_stage()`, a fase seguinte do Goiano: no fim dos pontos corridos,
/// o mata-mata de 8 (0x433930); a caixa da linha 0x1f1.
unsafe extern "thiscall" fn cqp8_fase_seguinte_do_goiano(this: usize) {
    cqp8_fase_seguinte(this, 0x98ae90, 0x1f1, 0x433930);
}

/// { 0x435e80 } (thiscall, ret 0, bra_reg_north.cpp): `BraRegNorth::start_new_season()`, a virada da temporada do Norte (a tabela:
/// 0x435d80; caixas nas linhas 0x1ca, 0x1d1, 0x1d7 e 0x1de).
unsafe extern "thiscall" fn cqp8_virada_do_norte(this: usize) -> u32 {
    cqp8_virada(this, 0x435d80, 0x98af14, [0x1ca, 0x1d1, 0x1d7, 0x1de], 1, "cqp_8 virada do norte") // cqp8: o ano seguinte do Norte
}

/// { 0x436030 } (thiscall, ret 0, bra_reg_north.cpp): `BraRegNorth::advance_stage()`, a fase seguinte do Norte: no fim dos pontos corridos,
/// o mata-mata de 4 (0x4360c0); a caixa da linha 0x1f8.
unsafe extern "thiscall" fn cqp8_fase_seguinte_do_norte(this: usize) {
    cqp8_fase_seguinte(this, 0x98af14, 0x1f8, 0x4360c0);
}

/// { 0x436e90 } (thiscall, ret 0, bra_reg_northeast.cpp): `BraRegNortheast::start_new_season()`, a virada da temporada do Nordeste (a tabela:
/// 0x436d90; caixas nas linhas 0x1bc, 0x1c3, 0x1c9 e 0x1d0).
unsafe extern "thiscall" fn cqp8_virada_do_nordeste(this: usize) -> u32 {
    cqp8_virada(this, 0x436d90, 0x98af54, [0x1bc, 0x1c3, 0x1c9, 0x1d0], 1, "cqp_8 virada do nordeste") // cqp8: o ano seguinte do Nordeste
}

/// { 0x437040 } (thiscall, ret 0, bra_reg_northeast.cpp): `BraRegNortheast::advance_stage()`, a fase seguinte do Nordeste: no fim dos pontos corridos,
/// o mata-mata de 4 (0x4370d0); a caixa da linha 0x1ea.
unsafe extern "thiscall" fn cqp8_fase_seguinte_do_nordeste(this: usize) {
    cqp8_fase_seguinte(this, 0x98af54, 0x1ea, 0x4370d0);
}

unsafe extern "thiscall" fn comparar_cqp8_virada_do_goiano(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1932);
    cqp8_comparar_virada("cqp8_virada_do_goiano", this, || o(this))
}

unsafe extern "thiscall" fn comparar_cqp8_fase_seguinte_do_goiano(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1933);
    cqp8_comparar_fase_seguinte("cqp8_fase_seguinte_do_goiano", this, || o(this))
}

unsafe extern "thiscall" fn comparar_cqp8_virada_do_norte(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1934);
    cqp8_comparar_virada("cqp8_virada_do_norte", this, || o(this))
}

unsafe extern "thiscall" fn comparar_cqp8_fase_seguinte_do_norte(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1935);
    cqp8_comparar_fase_seguinte("cqp8_fase_seguinte_do_norte", this, || o(this))
}

unsafe extern "thiscall" fn comparar_cqp8_virada_do_nordeste(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1936);
    cqp8_comparar_virada("cqp8_virada_do_nordeste", this, || o(this))
}

unsafe extern "thiscall" fn comparar_cqp8_fase_seguinte_do_nordeste(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1937);
    cqp8_comparar_fase_seguinte("cqp8_fase_seguinte_do_nordeste", this, || o(this))
}

// ---------- 280a leva: a virada da temporada e a fase seguinte dos estaduais: Pernambucano, Catarinense e Paulista (bra_reg_pern.cpp, bra_reg_santa.cpp, bra_reg_sp.cpp) ----------

unsafe fn cqp9_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// A virada da temporada de um estadual (`start_new_season`, slot 3, +0x08; o mesmo molde nos nove estaduais de pontos
/// corridos com mata-mata: só mudam o fonte, as linhas das caixas e a rotina da tabela, conferido instrução a instrução):
/// +0x4c = 0 e a tabela liberada (League::free_table 0x687970, thiscall(this, 0), ret 4); a lista de +0xba liberada
/// (free 0x9452ca) e zerada; com estatísticas (+8), Comp_Stats::archive_season_and_reset (0x4a1c50, thiscall(+8, 1),
/// ret 4); com a fase atual ≥ 0, as fases 0..=+0x30 que existem destruídas (a virtual 0, o destrutor com 1: libera);
/// o ano (+0x40, word) sobe um (`ano`; o contador `ramo`) e +0x30 = −1; então, cada um com a sua caixa se devolver 0: as regras da temporada
/// (virtual +0x8c, set_up_rules), a tabela com os clubes do estadual (`tabela`, thiscall, ret 0), os jogos
/// (League::create_fixtures 0x6835c0, thiscall, ret 0) e a ordem da tabela (League::sort_table 0x6827d0, thiscall(this,
/// 0), ret 4); por fim as posições previstas (virtual +0x5c) e a renovação dos árbitros do estadual
/// (OfficialsManager::renew_officials 0x79cee0, thiscall([0xb74340], +4), ret 4), cujo eax ela devolve.
unsafe fn cqp9_virada(this: usize, tabela: usize, arquivo: usize, linhas: [u32; 4], ano: i16, ramo: &'static str) -> u32 {
    let liberar_tabela: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x687970usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    core::ptr::write_unaligned((this + 0x4c) as *mut u32, 0);
    liberar_tabela(this, 0);
    let l = cqp9_d(this + 0xba);
    if l != 0 {
        liberar(l);
        core::ptr::write_unaligned((this + 0xba) as *mut usize, 0);
    }
    let estatisticas = cqp9_d(this + 8);
    if estatisticas != 0 {
        let arquivar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x4a1c50usize);
        arquivar(estatisticas, 1);
    }
    if (cqp9_d(this + 0x30) as i32) >= 0 {
        let mut i = 0i32;
        loop {
            let f = cqp9_d(cqp9_d(this + 0xc) + 4 * i as usize);
            if f != 0 {
                let destruir: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(cqp9_d(cqp9_d(f)));
                destruir(f, 1);
            }
            i += 1;
            if i > cqp9_d(this + 0x30) as i32 {
                break;
            }
        }
    }
    let a = core::ptr::read_unaligned((this + 0x40) as *const i16);
    core::ptr::write_unaligned((this + 0x40) as *mut i16, a.wrapping_add(ano));
    crate::comparacao::ramo(ramo);
    core::ptr::write_unaligned((this + 0x30) as *mut i32, -1);
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cqp9_d(cqp9_d(this) + 0x8c));
    if regras(this) == 0 {
        aviso_em(arquivo, linhas[0]);
        return 0;
    }
    let tabela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(tabela);
    if tabela(this) == 0 {
        aviso_em(arquivo, linhas[1]);
        return 0;
    }
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x6835c0usize);
    if jogos(this) == 0 {
        aviso_em(arquivo, linhas[2]);
        return 0;
    }
    let ordenar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x6827d0usize);
    if ordenar(this, 0) == 0 {
        aviso_em(arquivo, linhas[3]);
        return 0;
    }
    let previstas: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cqp9_d(cqp9_d(this) + 0x5c));
    previstas(this);
    let renovar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x79cee0usize);
    renovar(cqp9_d(0xb74340), cqp9_d(this + 4))
}

/// A fase seguinte de um estadual (`advance_stage`, slot 11, +0x28): na última fase (+0x30 ≥ +0x2c − 1, com sinal), a
/// caixa de erro e nada mais; senão +0x30 sobe um e, ao chegar a 0 (o fim dos pontos corridos), monta o mata-mata
/// (`mata_mata`, call direto com o ecx da liga -> thiscall, ret 0).
unsafe fn cqp9_fase_seguinte(this: usize, arquivo: usize, linha: u32, mata_mata: usize) {
    let fase = cqp9_d(this + 0x30) as i32;
    if fase >= (cqp9_d(this + 0x2c) as i32).wrapping_sub(1) {
        return aviso_em(arquivo, linha);
    }
    let fase = fase + 1;
    core::ptr::write_unaligned((this + 0x30) as *mut i32, fase);
    if fase == 0 {
        let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(mata_mata);
        f(this);
    }
}

/// O comparador da virada: só a original (ponytail: libera tabelas e fases, aloca as novas, sorteia os jogos e os
/// árbitros; não pode rodar duas vezes), conferindo o que só ela garante: o ano (+0x40) sobe um.
unsafe fn cqp9_comparar_virada(nome: &str, this: usize, original: impl FnOnce() -> u32) -> u32 {
    let antes = core::ptr::read_unaligned((this + 0x40) as *const i16);
    let r = original();
    let depois = core::ptr::read_unaligned((this + 0x40) as *const i16);
    PLACAR.registrar(depois == antes.wrapping_add(1), || format!("{}({:#x}) ano antes={} depois={}", nome, this, antes, depois));
    r
}

/// O comparador da fase seguinte: só a original (ponytail: ao chegar a 0 monta o mata-mata, que aloca e sorteia), com o
/// plano: na última fase, +0x30 igual; senão +0x30 + 1 (o mata-mata grava 0, o mesmo valor).
unsafe fn cqp9_comparar_fase_seguinte(nome: &str, this: usize, original: impl FnOnce()) {
    let antes = cqp9_d(this + 0x30) as i32;
    let ultima = antes >= (cqp9_d(this + 0x2c) as i32).wrapping_sub(1);
    original();
    let depois = cqp9_d(this + 0x30) as i32;
    PLACAR.registrar(depois == if ultima { antes } else { antes + 1 }, || format!("{}({:#x}) antes={} depois={}", nome, this, antes, depois));
}

/// { 0x4394d0 } (thiscall, ret 0, bra_reg_pern.cpp): `BraRegPern::start_new_season()`, a virada da temporada do Pernambucano (a tabela:
/// 0x439330; caixas nas linhas 0x1ca, 0x1d1, 0x1d7 e 0x1de).
unsafe extern "thiscall" fn cqp9_virada_do_pernambucano(this: usize) -> u32 {
    cqp9_virada(this, 0x439330, 0x98afd8, [0x1ca, 0x1d1, 0x1d7, 0x1de], 1, "cqp_9 virada do pernambucano") // cqp9: o ano seguinte do Pernambucano
}

/// { 0x439680 } (thiscall, ret 0, bra_reg_pern.cpp): `BraRegPern::advance_stage()`, a fase seguinte do Pernambucano: no fim dos pontos corridos,
/// o mata-mata de 4 (0x439710); a caixa da linha 0x1f8.
unsafe extern "thiscall" fn cqp9_fase_seguinte_do_pernambucano(this: usize) {
    cqp9_fase_seguinte(this, 0x98afd8, 0x1f8, 0x439710);
}

/// { 0x43bd10 } (thiscall, ret 0, bra_reg_santa.cpp): `BraRegSanta::start_new_season()`, a virada da temporada do Catarinense (a tabela:
/// 0x43bb70; caixas nas linhas 0x1ca, 0x1d1, 0x1d7 e 0x1de).
unsafe extern "thiscall" fn cqp9_virada_do_catarinense(this: usize) -> u32 {
    cqp9_virada(this, 0x43bb70, 0x98b050, [0x1ca, 0x1d1, 0x1d7, 0x1de], 1, "cqp_9 virada do catarinense") // cqp9: o ano seguinte do Catarinense
}

/// { 0x43bec0 } (thiscall, ret 0, bra_reg_santa.cpp): `BraRegSanta::advance_stage()`, a fase seguinte do Catarinense: no fim dos pontos corridos,
/// o mata-mata de 4 (0x43bf50); a caixa da linha 0x1f8.
unsafe extern "thiscall" fn cqp9_fase_seguinte_do_catarinense(this: usize) {
    cqp9_fase_seguinte(this, 0x98b050, 0x1f8, 0x43bf50);
}

/// { 0x43cd40 } (thiscall, ret 0, bra_reg_sp.cpp): `BraRegSp::start_new_season()`, a virada da temporada do Paulista (a tabela:
/// 0x43cc40; caixas nas linhas 0x1bc, 0x1c3, 0x1c9 e 0x1d0).
unsafe extern "thiscall" fn cqp9_virada_do_paulista(this: usize) -> u32 {
    cqp9_virada(this, 0x43cc40, 0x98b090, [0x1bc, 0x1c3, 0x1c9, 0x1d0], 1, "cqp_9 virada do paulista") // cqp9: o ano seguinte do Paulista
}

/// { 0x43cef0 } (thiscall, ret 0, bra_reg_sp.cpp): `BraRegSp::advance_stage()`, a fase seguinte do Paulista: no fim dos pontos corridos,
/// o mata-mata de 4 (0x43cf80); a caixa da linha 0x1ea.
unsafe extern "thiscall" fn cqp9_fase_seguinte_do_paulista(this: usize) {
    cqp9_fase_seguinte(this, 0x98b090, 0x1ea, 0x43cf80);
}

unsafe extern "thiscall" fn comparar_cqp9_virada_do_pernambucano(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1938);
    cqp9_comparar_virada("cqp9_virada_do_pernambucano", this, || o(this))
}

unsafe extern "thiscall" fn comparar_cqp9_fase_seguinte_do_pernambucano(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1939);
    cqp9_comparar_fase_seguinte("cqp9_fase_seguinte_do_pernambucano", this, || o(this))
}

unsafe extern "thiscall" fn comparar_cqp9_virada_do_catarinense(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1940);
    cqp9_comparar_virada("cqp9_virada_do_catarinense", this, || o(this))
}

unsafe extern "thiscall" fn comparar_cqp9_fase_seguinte_do_catarinense(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1941);
    cqp9_comparar_fase_seguinte("cqp9_fase_seguinte_do_catarinense", this, || o(this))
}

unsafe extern "thiscall" fn comparar_cqp9_virada_do_paulista(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1942);
    cqp9_comparar_virada("cqp9_virada_do_paulista", this, || o(this))
}

unsafe extern "thiscall" fn comparar_cqp9_fase_seguinte_do_paulista(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1943);
    cqp9_comparar_fase_seguinte("cqp9_fase_seguinte_do_paulista", this, || o(this))
}

// ---------- 280a leva: o calendário da Segunda espanhola: as 42 rodadas, uma tabela para o ano par e outra para o ímpar (spa_second.cpp) ----------


/// Uma rodada da tabela de 0x41 bytes: a data (dia, mês 0..11, anos a somar ao ano da liga +0x40, dia da semana para
/// onde a data é levada), o código do byte +4 ([inferência] o turno) e as faixas de TV em ordem (dia da semana, turno,
/// quantos jogos; a lista termina em −1).
type Cqq4Rodada = (i8, i8, i8, i8, i8, &'static [(i8, i8, i8)]);

/// As rodadas `primeira`.. da tabela `t`, na ordem da original: para cada uma, a data (0x68a160: push 0; push ano;
/// push código; push semana; push anos; push mês; push dia; push i; push t; add esp -> cdecl: o valor 0) e as
/// faixas (0x68a1c0: push 0; push c; push b; push a; push j; push i; push t; add esp -> cdecl, valor 0).
unsafe fn cqq4_rodadas(t: usize, ano: u32, primeira: i32, tabela: &[Cqq4Rodada]) {
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
unsafe fn cqq4_gravados(t: usize, guia: usize, n: usize) -> Vec<u8> {
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
struct Cqq4Natal {
    comeco: &'static [Cqq4Rodada],
    referencia: usize,
    ajuste: i32,
    casos: [&'static [Cqq4Rodada]; 8],
    fim: &'static [Cqq4Rodada],
    linha: u32,
}

unsafe fn cqq4_natal(t: usize, ano: u32, n: &Cqq4Natal, arquivo: usize) {
    let construir: unsafe extern "thiscall" fn(*mut [u16; 4], u32, u32, u32, u32) -> usize = core::mem::transmute(0x549f70usize);
    cqq4_rodadas(t, ano, 0, n.comeco);
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
    cqq4_rodadas(t, ano, n.comeco.len() as i32, meio);
    cqq4_rodadas(t, ano, (n.comeco.len() + meio.len()) as i32, n.fim);
}

/// Lado a lado em todas as fases: a original com as saídas de quem chama, a cópia com saídas próprias, a partir do
/// mesmo [0xb67a34]; as saídas e os bytes gravados das duas tabelas têm de ser iguais (na fase −1 as rodadas de liga,
/// `cqq4_gravados`; nas outras as de copa, `cql10_gravados`); a tabela da cópia é solta (0x9452ca, cdecl).
unsafe fn cqq4_comparar(nome: &str, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32,
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
            cqq4_gravados(a, a, n) == cqq4_gravados(b, a, n)
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

/// a Segunda espanhola: as rodadas antes do Natal (ano par)
const CQQ4_SEGUNDA_PAR_COMECO: [Cqq4Rodada; 17] = [
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
const CQQ4_SEGUNDA_PAR_NATAL_0: [Cqq4Rodada; 1] = [
    (26, 11, 0, -1, 2, &[(-1, -1, -1)]), // 17: 26/dez
];

/// as rodadas do Natal quando a rodada 16 (a de referência) cai em 18/dez (ano par)
const CQQ4_SEGUNDA_PAR_NATAL_1: [Cqq4Rodada; 1] = [
    (26, 11, 0, -1, 2, &[(-1, -1, -1)]), // 17: 26/dez
];

/// as rodadas do Natal quando a rodada 16 (a de referência) cai em 19/dez (ano par)
const CQQ4_SEGUNDA_PAR_NATAL_2: [Cqq4Rodada; 1] = [
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 17: 26/dez
];

/// as rodadas do Natal quando a rodada 16 (a de referência) cai em 20/dez (ano par)
const CQQ4_SEGUNDA_PAR_NATAL_3: [Cqq4Rodada; 1] = [
    (27, 11, 0, -1, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 17: 27/dez
];

/// as rodadas do Natal quando a rodada 16 (a de referência) cai em 21/dez (ano par)
const CQQ4_SEGUNDA_PAR_NATAL_4: [Cqq4Rodada; 1] = [
    (28, 11, 0, -1, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 17: 28/dez
];

/// as rodadas do Natal quando a rodada 16 (a de referência) cai em 22/dez (ano par)
const CQQ4_SEGUNDA_PAR_NATAL_5: [Cqq4Rodada; 1] = [
    (29, 11, 0, -1, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 17: 29/dez
];

/// as rodadas do Natal quando a rodada 16 (a de referência) cai em 23/dez (ano par)
const CQQ4_SEGUNDA_PAR_NATAL_6: [Cqq4Rodada; 1] = [
    (30, 11, 0, -1, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 17: 30/dez
];

/// as rodadas do Natal no caso padrão (a rodada 16 fora de 17 a 23/dez: a caixa) (ano par)
const CQQ4_SEGUNDA_PAR_NATAL_PADRAO: [Cqq4Rodada; 1] = [
    (27, 11, 0, 6, 1, &[(5, 1, 1), (5, 1, 3), (5, 1, 3), (-1, -1, -1)]), // 17: 27/dez
];

/// a Segunda espanhola: as rodadas depois do Natal (ano par)
const CQQ4_SEGUNDA_PAR_FIM: [Cqq4Rodada; 24] = [
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
const CQQ4_SEGUNDA_IMPAR_TODAS: [Cqq4Rodada; 42] = [
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

const CQQ4_SEGUNDA_PAR_NATAL: Cqq4Natal = Cqq4Natal { comeco: &CQQ4_SEGUNDA_PAR_COMECO, referencia: 16, ajuste: 17, casos: [&CQQ4_SEGUNDA_PAR_NATAL_0, &CQQ4_SEGUNDA_PAR_NATAL_1, &CQQ4_SEGUNDA_PAR_NATAL_2, &CQQ4_SEGUNDA_PAR_NATAL_3, &CQQ4_SEGUNDA_PAR_NATAL_4, &CQQ4_SEGUNDA_PAR_NATAL_5, &CQQ4_SEGUNDA_PAR_NATAL_6, &CQQ4_SEGUNDA_PAR_NATAL_PADRAO], fim: &CQQ4_SEGUNDA_PAR_FIM, linha: 0x146 };

/// { 0x853c90 } (thiscall, ret 0x10, spa_second.cpp, 7.827 bytes, método +0x3c): `SpaSecond::create_schedule(fase,
/// &rodadas, &extra, &é_liga)`: só a fase −1, 42 rodadas, a tabela pela paridade do ano +0x40 (no ano par com a rodada
/// do Natal; no ímpar sem); as outras devolvem 0. Fonte 0xa829bc.
unsafe extern "thiscall" fn calendario_da_segunda_espanhola(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
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
                aviso_em(0xa828bc, 0xa9);
                return 0;
            }
            crate::comparacao::ramo("cqq_4 calendario da segunda espanhola");
            let ano = core::ptr::read_unaligned((this + 0x40) as *const u16) as u32;
            if (ano as i16) % 2 != 0 {
                cqq4_rodadas(t, ano, 0, &CQQ4_SEGUNDA_IMPAR_TODAS);
            } else {
                cqq4_natal(t, ano, &CQQ4_SEGUNDA_PAR_NATAL, 0xa828bc);
            }
            t
        }
        _ => 0,
    }
}

unsafe extern "thiscall" fn comparar_calendario_da_segunda_espanhola(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1944);
    cqq4_comparar("calendario_da_segunda_espanhola", fase, rodadas, extra, liga, || o(this, fase, rodadas, extra, liga), |r, e, l| calendario_da_segunda_espanhola(this, fase, r, e, l))
}

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

// ---------- 280a leva: o construtor da primeira divisão francesa: as duas competições-marcador, os clubes da CFA e da divisão de baixo levados à divisão genérica e as duas ligas de fundo montadas e soltas (fra_first.cpp) ----------

unsafe fn cqq6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqq6_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// O registro de competição de id [g] (vetor [0xae23d0], 0x6b bytes; a original faz id × 3 × 9 × 4 − id).
unsafe fn cqq6_registro(g: usize) -> usize {
    cqq6_d(0xae23d0).wrapping_add(cqq6_d(g).wrapping_mul(0x6b))
}

/// Uma competição-marcador (operator new 0x944cf1 de 0xa3: push; add esp, 4 -> cdecl; sem memória: 0): Comp::Comp
/// (0x48c580: mov ecx -> thiscall, ret 0), a vtable `vtabela`, +0x42 = 1, o registro de id [g] em +4 e sem disciplina
/// (+0x50 = 0xff).
unsafe fn cqq6_marcador(vtabela: usize, g: usize) -> usize {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let comp: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x48c580usize);
    let c = novo(0xa3);
    if c == 0 {
        return 0;
    }
    comp(c);
    cqq6_pd(c, vtabela);
    *((c + 0x42) as *mut u8) = 1;
    cqq6_pd(c + 4, cqq6_registro(g));
    *((c + 0x50) as *mut u8) = 0xff;
    c
}

/// { 0x5bb160 } (thiscall, ret 8, fra_first.cpp, 988 bytes, quadro SEH só para os operator new): `FraFirst::FraFirst(ano,
/// registro)`: a base (League::League 0x682200: mov ecx -> thiscall, ret 0), a vtable 0x96aa24, o ano (+0x40, palavra),
/// o registro (+4) e a disciplina 0xc (+0x50); em +0xee e +0xf2 as competições-marcador da CFA (vtable 0x96a994, id
/// [0x9cf5d8]) e da divisão de baixo (0x96a904, id [0x9cf548]). Tenta carregar do arquivo temporário
/// (League::load_from_temp_file 0x687b10: push 1; mov ecx -> thiscall, ret 4): carregou, pronto. Senão, a liga nova:
/// cada clube (os [0xae235c] − 2 × [0xae2350] primeiros de [0xae23b4], 0x245 bytes; a conta refeita a cada volta) cuja
/// competição (+0x57) é a CFA passa à divisão genérica (registro de id [0x9cf764]) com a CFA como competição de reserva
/// (+0x60); o da divisão de baixo, à genérica sem reserva (+0x60 = 0). Depois fase 0xff (+0x44), fase atual −1 (+0x30),
/// sem fases (+0x2c, +0xc); as regras (0x5bc410), a tabela (0x5bc2b0), os jogos (League::create_fixtures 0x6835c0), a
/// ordem (League::sort_table 0x6827d0: push 0 -> thiscall, ret 4), as estatísticas (operator new de 0x5ce e
/// Comp_Stats::Comp_Stats 0x49ee70: push liga; mov ecx, novo -> thiscall, ret 4) em +8 e as posições previstas
/// (0x68a850: mov ecx -> thiscall, ret 0); cada passo que falha mostra a caixa da sua linha (0x65, 0x6c, 0x73, 0x7c,
/// 0x83) e encerra. Por fim, as duas ligas de fundo: a CFA (operator new de 0xee; FraCfa::FraCfa 0x5b91c0: push ano;
/// mov ecx -> thiscall, ret 4) e a divisão de baixo (0x5bd600, o mesmo), que deixam os seus jogos no calendário do jogo,
/// e são destruídas (a virtual 0 com 1: push 1; mov ecx -> thiscall, ret 4). Devolve this.
unsafe extern "thiscall" fn construir_primeira_francesa(this: usize, ano: u32, registro: usize) -> usize {
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x682200usize);
    let carregar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x687b10usize);
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x5bc410usize);
    let tabela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x5bc2b0usize);
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x6835c0usize);
    let ordenar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x6827d0usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let estatisticas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x49ee70usize);
    let previstas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68a850usize);
    let cfa: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(0x5b91c0usize);
    let baixo: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(0x5bd600usize);
    base(this);
    cqq6_pd(this, 0x96aa24);
    core::ptr::write_unaligned((this + 0x40) as *mut u16, ano as u16);
    cqq6_pd(this + 4, registro);
    *((this + 0x50) as *mut u8) = 0xc; // cqq6: a disciplina da primeira francesa
    let m = cqq6_marcador(0x96a994, 0x9cf5d8);
    cqq6_pd(this + 0xee, m);
    let m = cqq6_marcador(0x96a904, 0x9cf548);
    cqq6_pd(this + 0xf2, m);
    if carregar(this, 1) != 0 {
        return this;
    }
    crate::comparacao::ramo("cqq_6 primeira francesa nova");
    let n = || (cqq6_d(0xae235c) as i32).wrapping_sub((cqq6_d(0xae2350) as i32).wrapping_mul(2));
    if n() > 0 {
        let mut p = cqq6_d(0xae23b4) + 0x57;
        let mut i: i32 = 0;
        loop {
            let comp = cqq6_d(p);
            if comp != 0 {
                if cqq6_d(comp) == cqq6_d(0x9cf5d8) {
                    cqq6_pd(p + 9, comp); // cqq6: a CFA vira a competição de reserva
                    cqq6_pd(p, cqq6_registro(0x9cf764));
                } else if cqq6_d(comp) == cqq6_d(0x9cf548) {
                    cqq6_pd(p + 9, 0); // cqq6: a divisão de baixo fica sem reserva
                    cqq6_pd(p, cqq6_registro(0x9cf764));
                }
            }
            i += 1;
            p += 0x245;
            if i >= n() {
                break;
            }
        }
    }
    *((this + 0x44) as *mut u8) = 0xff;
    cqq6_pd(this + 0x30, 0xffff_ffff);
    cqq6_pd(this + 0x2c, 0);
    cqq6_pd(this + 0xc, 0);
    if regras(this) == 0 {
        aviso_em(0x9ccb5c, 0x65);
        return this;
    }
    if tabela(this) == 0 {
        aviso_em(0x9ccb5c, 0x6c);
        return this;
    }
    if jogos(this) == 0 {
        aviso_em(0x9ccb5c, 0x73);
        return this;
    }
    if ordenar(this, 0) == 0 {
        aviso_em(0x9ccb5c, 0x7c);
        return this;
    }
    let e = novo(0x5ce);
    let s = if e != 0 { estatisticas(e, this) } else { 0 };
    cqq6_pd(this + 8, s);
    if s == 0 {
        aviso_em(0x9ccb5c, 0x83);
        return this;
    }
    previstas(this);
    let ano = core::ptr::read_unaligned((this + 0x40) as *const u16) as u32;
    let a = novo(0xee);
    let ligas_cfa = if a != 0 { cfa(a, ano) } else { 0 };
    let b = novo(0xee);
    let ligas_baixo = if b != 0 { baixo(b, ano) } else { 0 };
    for l in [ligas_cfa, ligas_baixo] {
        if l != 0 {
            let destruir: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(cqq6_d(cqq6_d(l)));
            destruir(l, 1);
        }
    }
    this
}

/// Sorteia (as ligas de fundo), aloca e mexe nos clubes: só a original, conferindo a devolução, a vtable, o ano, o
/// registro, a disciplina e os dois marcadores (vtable e registro) e, na liga nova, os campos do começo e as
/// estatísticas.
unsafe extern "thiscall" fn comparar_construir_primeira_francesa(this: usize, ano: u32, registro: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(1946);
    let r = o(this, ano, registro);
    let marcador = |m: usize, vt: usize, g: usize| m == 0 || (cqq6_d(m) == vt && cqq6_d(m + 4) == cqq6_registro(g) && *((m + 0x50) as *const u8) == 0xff);
    let mut ok = r == this && cqq6_d(this) == 0x96aa24 && core::ptr::read_unaligned((this + 0x40) as *const u16) == ano as u16
        && cqq6_d(this + 4) == registro && *((this + 0x50) as *const u8) == 0xc
        && marcador(cqq6_d(this + 0xee), 0x96a994, 0x9cf5d8) && marcador(cqq6_d(this + 0xf2), 0x96a904, 0x9cf548);
    if *((this + 0x44) as *const u8) == 0xff && cqq6_d(this + 8) != 0 {
        ok = ok && cqq6_d(this + 0x2c) == 0 && cqq6_d(this + 0xc) == 0;
    }
    PLACAR.registrar(ok, || format!("construir_primeira_francesa({:#x}): devolveu {:#x}, vtable {:#x}, +0xee = {:#x}, +0xf2 = {:#x}, +8 = {:#x}", this, r, cqq6_d(this), cqq6_d(this + 0xee), cqq6_d(this + 0xf2), cqq6_d(this + 8)));
    r
}

// ---------- 280a leva: os play-offs das Séries A e B: a fase seguinte e o mata-mata dos 4 primeiros (bra_nat_first.cpp, bra_nat_second.cpp) ----------

unsafe fn cqq7_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqq7_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O clube da linha `l` da tabela (+0xb1, linhas de 0x3b bytes, +0 o clube).
unsafe fn cqq7_clube(liga: usize, l: usize) -> usize {
    cqq7_d(cqq7_d(liga + 0xb1) + l * 0x3b)
}

/// O play-off de uma Série (o mesmo molde na A e na B, conferido instrução a instrução: só mudam o fonte e as linhas
/// das caixas): a lista dos 4 (malloc 0x944e46 de 0x10; sem memória: a caixa `linha_sem_lista` e volta) com os clubes
/// das `linhas` da tabela, nesta ordem (as chaves: 0 × 1, 2 × 3); a virtual +0x3c (create_round_table: thiscall(this,
/// fase 0, &rodadas, &extra, 0), ret 0x10) monta a tabela de rodadas; a fase nova (operator new 0x944cf1 de 0xb2;
/// Cup_Stage::Cup_Stage 0x522e00, thiscall, ret 0x3c: a liga, 4, a lista, as rodadas, +4, a tabela, o ano +0x40, 0, 1,
/// o extra, 0x14, 0, 0, 0 e nenhuma semente: a ordem não é por sementes) vai para a primeira vaga das fases ([+0xc]) e a
/// tabela de rodadas é liberada (free 0x9452ca). Sem fase: a caixa `linha_sem_fase`, a lista liberada e volta. Com ela:
/// Cup::set_up_round (0x51c800, thiscall(fase, 0), ret 4), a lista liberada e +0x30 = 0. O quadro SEH da original só
/// cobre o construtor (sem destrutor).
unsafe fn cqq7_play_off(this: usize, linhas: &[usize; 4], arquivo: usize, linha_sem_lista: u32, linha_sem_fase: u32, ramo: &'static str) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let lista = alocar(0x10);
    if lista == 0 {
        return aviso_em(arquivo, linha_sem_lista);
    }
    for (k, &l) in linhas.iter().enumerate() {
        core::ptr::write_unaligned((lista + 4 * k) as *mut usize, cqq7_clube(this, l));
    }
    let tabela: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(cqq7_d(cqq7_d(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = tabela(this, 0, &mut rodadas, &mut extra, 0);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let p = novo(0xb2);
    // ponytail: a original empurra as rodadas, o extra e o ano em dwords com lixo nos 16 bits altos (as saídas da
    // virtual são words na pilha; o ano vai no dx); o construtor só lê as palavras.
    let fase = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(0x522e00usize);
        construir(p, this, 4, lista, rodadas as u16 as u32, cqq7_d(this + 4), t, cqq7_w(this + 0x40) as u16 as u32, 0, 1, extra as u16 as u32, 0x14, 0, 0, 0, core::ptr::null())
    } else {
        0
    };
    core::ptr::write_unaligned(cqq7_d(this + 0xc) as *mut usize, fase);
    liberar(t);
    let fase = cqq7_d(cqq7_d(this + 0xc));
    if fase == 0 {
        aviso_em(arquivo, linha_sem_fase);
        liberar(lista);
        return;
    }
    crate::comparacao::ramo(ramo);
    let montar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x51c800usize);
    montar(fase, 0);
    liberar(lista);
    core::ptr::write_unaligned((this + 0x30) as *mut u32, 0);
}

/// A fase seguinte de uma Série (`advance_stage`, slot 11, +0x28): na última fase (+0x30 ≥ +0x2c − 1, com sinal), a
/// caixa de erro e nada mais; senão +0x30 sobe um e, ao chegar a 0 (o fim dos pontos corridos), monta o play-off
/// (`play_off`, call direto com o ecx da liga -> thiscall, ret 0).
unsafe fn cqq7_fase_seguinte(this: usize, arquivo: usize, linha: u32, play_off: usize) {
    let fase = cqq7_d(this + 0x30) as i32;
    if fase >= (cqq7_d(this + 0x2c) as i32).wrapping_sub(1) {
        return aviso_em(arquivo, linha);
    }
    let fase = fase + 1;
    core::ptr::write_unaligned((this + 0x30) as *mut i32, fase);
    if fase == 0 {
        let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(play_off);
        f(this);
    }
}

/// { 0x42a950 } (thiscall, ret 0, bra_nat_first.cpp): `BraNatFirst::advance_stage()`, a fase seguinte da Série A: no fim
/// dos pontos corridos, o play-off (0x42a9e0); a caixa da linha 0x264.
unsafe extern "thiscall" fn cqq7_fase_seguinte_da_serie_a(this: usize) {
    cqq7_fase_seguinte(this, 0x98aa30, 0x264, 0x42a9e0);
}

/// { 0x42a9e0 } (thiscall, ret 0, bra_nat_first.cpp): `BraNatFirst::create_play_off_stage()`, o play-off da Série A:
/// 1º × 4º, 2º × 3º (linhas 0, 3, 1, 2); caixas nas linhas 0x286 (sem lista) e 0x29d (sem fase).
unsafe extern "thiscall" fn cqq7_play_off_da_serie_a(this: usize) {
    cqq7_play_off(this, &[0, 3, 1, 2], 0x98aa30, 0x286, 0x29d, "cqq_7 serie a"); // cqq7: 1º × 4º, 2º × 3º Série A
}

/// { 0x42cdd0 } (thiscall, ret 0, bra_nat_second.cpp): `BraNatSecond::advance_stage()`, a fase seguinte da Série B: no fim
/// dos pontos corridos, o play-off (0x42ce60); a caixa da linha 0x23d.
unsafe extern "thiscall" fn cqq7_fase_seguinte_da_serie_b(this: usize) {
    cqq7_fase_seguinte(this, 0x98ab64, 0x23d, 0x42ce60);
}

/// { 0x42ce60 } (thiscall, ret 0, bra_nat_second.cpp): `BraNatSecond::create_play_off_stage()`, o play-off da Série B:
/// 1º × 4º, 2º × 3º (linhas 0, 3, 1, 2); caixas nas linhas 0x25f (sem lista) e 0x276 (sem fase).
unsafe extern "thiscall" fn cqq7_play_off_da_serie_b(this: usize) {
    cqq7_play_off(this, &[0, 3, 1, 2], 0x98ab64, 0x25f, 0x276, "cqq_7 serie b"); // cqq7: 1º × 4º, 2º × 3º Série B
}

/// O comparador da fase seguinte: só a original (ponytail: ao chegar a 0 monta o play-off, que aloca e sorteia), com o
/// plano: na última fase, +0x30 igual; senão +0x30 + 1 (o play-off grava 0, o mesmo valor).
unsafe fn cqq7_comparar_fase_seguinte(nome: &str, this: usize, original: impl FnOnce()) {
    let antes = cqq7_d(this + 0x30) as i32;
    let ultima = antes >= (cqq7_d(this + 0x2c) as i32).wrapping_sub(1);
    original();
    let depois = cqq7_d(this + 0x30) as i32;
    PLACAR.registrar(depois == if ultima { antes } else { antes + 1 }, || format!("{}({:#x}) antes={} depois={}", nome, this, antes, depois));
}

/// O comparador do play-off: só a original (aloca a lista, a tabela e a fase e sorteia a rodada), com o plano feito
/// antes: os clubes das linhas. Depois, a fase da primeira vaga tem de existir, com +0x3e = 4, +4 = o +4 da liga e os 4
/// primeiros clubes da lista dela (+0xa7, 6 bytes) iguais aos do plano (como conjunto: o sorteio da rodada pode
/// reordenar), e +0x30 = 0.
unsafe fn cqq7_comparar_play_off(nome: &str, this: usize, linhas: &[usize; 4], original: impl FnOnce()) {
    let mut plano: Vec<usize> = linhas.iter().map(|&l| cqq7_clube(this, l)).collect();
    original();
    let fase = cqq7_d(cqq7_d(this + 0xc));
    if fase == 0 {
        PLACAR.registrar(false, || format!("{}({:#x}) sem fase", nome, this));
        return;
    }
    let lista = cqq7_d(fase + 0xa7);
    let mut achou: Vec<usize> = (0..4).map(|k| cqq7_d(lista + 6 * k)).collect();
    let (pl, ac) = (plano.clone(), achou.clone());
    plano.sort_unstable();
    achou.sort_unstable();
    let igual = plano == achou && cqq7_w(fase + 0x3e) == 4 && cqq7_d(fase + 4) == cqq7_d(this + 4) && cqq7_d(this + 0x30) == 0;
    PLACAR.registrar(igual, || format!("{}({:#x}) plano={:x?} fase={:#x} lista={:x?} +0x3e={}", nome, this, pl, fase, ac, cqq7_w(fase + 0x3e)));
}

unsafe extern "thiscall" fn comparar_cqq7_fase_seguinte_da_serie_a(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1947);
    cqq7_comparar_fase_seguinte("cqq7_fase_seguinte_da_serie_a", this, || o(this))
}

unsafe extern "thiscall" fn comparar_cqq7_play_off_da_serie_a(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1948);
    cqq7_comparar_play_off("cqq7_play_off_da_serie_a", this, &[0, 3, 1, 2], || o(this))
}

unsafe extern "thiscall" fn comparar_cqq7_fase_seguinte_da_serie_b(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1949);
    cqq7_comparar_fase_seguinte("cqq7_fase_seguinte_da_serie_b", this, || o(this))
}

unsafe extern "thiscall" fn comparar_cqq7_play_off_da_serie_b(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1950);
    cqq7_comparar_play_off("cqq7_play_off_da_serie_b", this, &[0, 3, 1, 2], || o(this))
}

// ---------- 280a leva: a virada do futebol brasileiro: a Série A, o acesso e o rebaixamento em cadeia e a Série B (bra_nat_first.cpp, bra_nat_second.cpp) ----------

unsafe fn cqq8_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// A competição de id [`global`] (a tabela das competições [0xadadfc]).
unsafe fn cqq8_comp(global: usize) -> usize {
    cqq8_d(cqq8_d(0xadadfc) + 4 * cqq8_d(global))
}

/// A virtual `deslocamento` de `c`, thiscall sem argumentos.
unsafe fn cqq8_virtual(c: usize, deslocamento: usize) -> u32 {
    let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cqq8_d(cqq8_d(c) + deslocamento));
    f(c)
}

/// O corpo comum da virada de uma Série (o mesmo da virada dos estaduais, cqp_7..9): a tabela liberada
/// (League::free_table 0x687970, thiscall(this, 0), ret 4); a lista de +0xba liberada (free 0x9452ca) e zerada; com
/// estatísticas (+8), Comp_Stats::archive_season_and_reset (0x4a1c50, thiscall(+8, 1), ret 4); com a fase atual ≥ 0,
/// as fases 0..=+0x30 que existem destruídas (a virtual 0 com 1); o ano (+0x40, word) sobe `ano` e +0x30 = −1; então,
/// cada um com a sua caixa se devolver 0: as regras (virtual +0x8c), a tabela (`tabela`, thiscall, ret 0), os jogos
/// (League::create_fixtures 0x6835c0, thiscall, ret 0) e a ordem da tabela (League::sort_table 0x6827d0, thiscall(this,
/// 0), ret 4); por fim as posições previstas (virtual +0x5c). Devolve falso depois de uma caixa.
unsafe fn cqq8_refazer(this: usize, tabela: usize, arquivo: usize, linhas: [u32; 4], ano: i16) -> bool {
    let liberar_tabela: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x687970usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    liberar_tabela(this, 0);
    let l = cqq8_d(this + 0xba);
    if l != 0 {
        liberar(l);
        core::ptr::write_unaligned((this + 0xba) as *mut usize, 0);
    }
    let estatisticas = cqq8_d(this + 8);
    if estatisticas != 0 {
        let arquivar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x4a1c50usize);
        arquivar(estatisticas, 1);
    }
    if (cqq8_d(this + 0x30) as i32) >= 0 {
        let mut i = 0i32;
        loop {
            let f = cqq8_d(cqq8_d(this + 0xc) + 4 * i as usize);
            if f != 0 {
                let destruir: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(cqq8_d(cqq8_d(f)));
                destruir(f, 1);
            }
            i += 1;
            if i > cqq8_d(this + 0x30) as i32 {
                break;
            }
        }
    }
    let a = core::ptr::read_unaligned((this + 0x40) as *const i16);
    core::ptr::write_unaligned((this + 0x40) as *mut i16, a.wrapping_add(ano));
    core::ptr::write_unaligned((this + 0x30) as *mut i32, -1);
    if cqq8_virtual(this, 0x8c) == 0 {
        aviso_em(arquivo, linhas[0]);
        return false;
    }
    let tabela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(tabela);
    if tabela(this) == 0 {
        aviso_em(arquivo, linhas[1]);
        return false;
    }
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x6835c0usize);
    if jogos(this) == 0 {
        aviso_em(arquivo, linhas[2]);
        return false;
    }
    let ordenar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x6827d0usize);
    if ordenar(this, 0) == 0 {
        aviso_em(arquivo, linhas[3]);
        return false;
    }
    cqq8_virtual(this, 0x5c);
    true
}

/// O fim comum: as expectativas da diretoria de cada clube (League::set_club_season_expectations 0x68aa80, thiscall,
/// ret 0) e a renovação dos árbitros (OfficialsManager::renew_officials 0x79cee0, thiscall([0xb74340], +4), ret 4),
/// cujo eax a virada devolve.
unsafe fn cqq8_expectativas_e_arbitros(this: usize) -> u32 {
    let expectativas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68aa80usize);
    expectativas(this);
    let renovar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x79cee0usize);
    renovar(cqq8_d(0xb74340), cqq8_d(this + 4)) // cqq8: os árbitros
}

/// As viradas das outras catorze competições do Brasil, na ordem da original: a Série B, a Série C e os doze estaduais
/// (Baiano, Centro-Oeste, Gaúcho, Goiano, Mineiro, Norte, Nordeste, Paranaense, Pernambucano, Carioca, Catarinense e
/// Paulista: os globais de id 0x9cf9e0, 0x9cf9ac, 0x9cf9c0, 0x9cf998, 0x9cf9c8, 0x9cf9b4, 0x9cf9d4, 0x9cf994, 0x9cf9a4,
/// 0x9cf9d0, 0x9cf9bc, 0x9cf9dc, 0x9cf9b0, 0x9cf9e4).
const CQQ8_VIRADAS: [usize; 14] = [0x9cf9e0, 0x9cf9ac, 0x9cf9c0, 0x9cf998, 0x9cf9c8, 0x9cf9b4, 0x9cf9d4, 0x9cf994, 0x9cf9a4, 0x9cf9d0, 0x9cf9bc, 0x9cf9dc, 0x9cf9b0, 0x9cf9e4];

/// { 0x42a640 } (thiscall, ret 0, bra_nat_first.cpp): `BraNatFirst::start_new_season()`, a virada do futebol brasileiro
/// (a Série A comanda): +0x4c = 0; o acesso e o rebaixamento em cadeia (a virtual +0xb0, 0x42ae90, thiscall(this, 1),
/// ret 4); os inscritos da Copa do Brasil (0x42b760), a entrada de 16 clubes na Série C (0x42b520) e as trocas dos
/// estaduais com as divisões de baixo (0x42b130), as três com mov ecx, esi e sem push (thiscall, ret 0; o mapa as diz
/// cdecl(void): o ecx sobra, sem efeito); o nível da divisão dos clubes da Série A e da Série B limitado
/// (League::limit_club_division_tier 0x68a980, thiscall(liga, 1, 3, −3, 1), ret 0x10); a Série A refeita (o corpo comum,
/// a tabela 0x42a480, caixas nas linhas 0x223, 0x22a, 0x230 e 0x237); as viradas das outras catorze (a virtual +8);
/// as expectativas e os árbitros.
unsafe extern "thiscall" fn cqq8_virada_da_serie_a(this: usize) -> u32 {
    core::ptr::write_unaligned((this + 0x4c) as *mut u32, 0);
    let cadeia: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(cqq8_d(cqq8_d(this) + 0xb0));
    cadeia(this, 1);
    for f in [0x42b760usize, 0x42b520, 0x42b130] {
        let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(f);
        f(this);
    }
    let nivel: unsafe extern "thiscall" fn(usize, u32, u32, i32, u32) = core::mem::transmute(0x68a980usize);
    nivel(this, 1, 3, -3, 1);
    nivel(cqq8_comp(0x9cf9e0), 1, 3, -3, 1);
    if !cqq8_refazer(this, 0x42a480, 0x98aa30, [0x223, 0x22a, 0x230, 0x237], 1) {
        return 0;
    }
    for (k, g) in CQQ8_VIRADAS.iter().enumerate() {
        if k == 13 {
            crate::comparacao::ramo("cqq_8 virada do paulista");
        }
        cqq8_virtual(cqq8_comp(*g), 8); // cqq8: a virada de cada uma
    }
    cqq8_expectativas_e_arbitros(this)
}

/// { 0x42ae90 } (thiscall, ret 4, bra_nat_first.cpp): `BraNatFirst::process_promotion_relegation_chain(int)` (slot 45,
/// +0xb0; o argumento não é lido): as posições finais (a virtual +0xa4, record_final_positions, thiscall, ret 0) da
/// Série A, da Série B, da Série C e das sete fases da Série C ([C+0xc][0..7]); depois o acesso e o rebaixamento
/// (league_promote_and_relegate 0x689c80, thiscall(this, de cima, de baixo, 1, 1, −1, −1), ret 0x18: a original põe
/// o ecx da Série A e a rotina o guarda) entre A e B, entre B e C e entre a B e cada uma das sete fases da C. Devolve o
/// eax da última.
unsafe extern "thiscall" fn cqq8_acesso_e_rebaixamento(this: usize, _pares: u32) -> u32 {
    let (a, b, c) = (0x9cf9ec, 0x9cf9e0, 0x9cf9ac);
    cqq8_virtual(cqq8_comp(a), 0xa4);
    cqq8_virtual(cqq8_comp(b), 0xa4);
    cqq8_virtual(cqq8_comp(c), 0xa4);
    for k in 0..7 {
        cqq8_virtual(cqq8_d(cqq8_d(cqq8_comp(c) + 0xc) + 4 * k), 0xa4);
    }
    let trocar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, i32, i32) -> u32 = core::mem::transmute(0x689c80usize);
    crate::comparacao::ramo("cqq_8 acesso e rebaixamento");
    trocar(this, cqq8_comp(a), cqq8_comp(b), 1, 1, -1, -1); // cqq8: A e B
    let mut r = trocar(this, cqq8_comp(b), cqq8_comp(c), 1, 1, -1, -1); // cqq8: B e C
    for k in 0..7 {
        r = trocar(this, cqq8_comp(b), cqq8_d(cqq8_d(cqq8_comp(c) + 0xc) + 4 * k), 1, 1, -1, -1);
    }
    r
}

/// { 0x42cc10 } (thiscall, ret 0, bra_nat_second.cpp): `BraNatSecond::start_new_season()`, a virada da Série B (da
/// virada da Série A): +0x4c = 0, o corpo comum (a tabela 0x42ca70, caixas nas linhas 0x20c, 0x213, 0x219 e 0x220), as
/// expectativas e os árbitros.
unsafe extern "thiscall" fn cqq8_virada_da_serie_b(this: usize) -> u32 {
    core::ptr::write_unaligned((this + 0x4c) as *mut u32, 0);
    if !cqq8_refazer(this, 0x42ca70, 0x98ab64, [0x20c, 0x213, 0x219, 0x220], 1) {
        return 0;
    }
    crate::comparacao::ramo("cqq_8 virada da serie b");
    cqq8_expectativas_e_arbitros(this)
}

/// Comparador das viradas: só a original (ponytail: libera e aloca tabelas e fases, sorteia jogos e árbitros e roda as
/// viradas das outras competições; não pode rodar duas vezes), conferindo o que só ela garante: o ano sobe um.
unsafe fn cqq8_comparar_virada(nome: &str, this: usize, original: impl FnOnce() -> u32) -> u32 {
    let antes = core::ptr::read_unaligned((this + 0x40) as *const i16);
    let r = original();
    let depois = core::ptr::read_unaligned((this + 0x40) as *const i16);
    PLACAR.registrar(depois == antes.wrapping_add(1), || format!("{}({:#x}) ano antes={} depois={}", nome, this, antes, depois));
    r
}

unsafe extern "thiscall" fn comparar_cqq8_virada_da_serie_a(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1951);
    cqq8_comparar_virada("cqq8_virada_da_serie_a", this, || o(this))
}

/// Comparador do acesso e rebaixamento: só a original (ponytail: move clubes entre as listas das ligas, que realocam).
/// A cópia roda no modo Rust (contador "cqq_8 acesso e rebaixamento") e é provada pelas mutações e pelas carreiras.
unsafe extern "thiscall" fn comparar_cqq8_acesso_e_rebaixamento(this: usize, pares: u32) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize, u32) -> u32>(1952)(this, pares)
}

unsafe extern "thiscall" fn comparar_cqq8_virada_da_serie_b(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1953);
    cqq8_comparar_virada("cqq8_virada_da_serie_b", this, || o(this))
}

// ---------- 280a leva: a J1: a fase seguinte, a final entre os campeões dos dois turnos e a virada da temporada (jap_j1.cpp) ----------

unsafe fn cqq9_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqq9_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O campeão de um turno: o clube da primeira linha da tabela (+0xb1) da fase `k` ([+0xc][k], um League_Stage).
unsafe fn cqq9_campeao_do_turno(this: usize, k: usize) -> usize {
    cqq9_d(cqq9_d(cqq9_d(cqq9_d(this + 0xc) + 4 * k) + 0xb1))
}

/// { 0x6639c0 } (thiscall, ret 0, jap_j1.cpp): `JapJ1::advance_stage()`, a fase seguinte da J1 (dois turnos, as fases 0
/// e 1, e a final, a fase 2). Na última fase (+0x30 ≥ +0x2c − 1, com sinal): a caixa da linha 0x223 e nada mais. Senão
/// +0x30 sobe um e, ao chegar a 2, monta a final (0x663a50, call direto com o ecx da liga).
unsafe extern "thiscall" fn cqq9_fase_seguinte_da_j1(this: usize) {
    let fase = cqq9_d(this + 0x30) as i32;
    if fase >= (cqq9_d(this + 0x2c) as i32).wrapping_sub(1) {
        return aviso_em(0x9eabc8, 0x223);
    }
    let fase = fase + 1;
    core::ptr::write_unaligned((this + 0x30) as *mut i32, fase);
    if fase == 2 {
        let final_: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x663a50usize);
        final_(this);
    }
}

/// { 0x663a50 } (thiscall, ret 0, jap_j1.cpp): `JapJ1::create_championship_final_stage()`, a final da J1 entre os
/// campeões dos dois turnos (o clube da primeira linha da tabela de cada um), sempre (sem conferir se é o mesmo clube):
/// a lista de 2 (malloc 0x944e46 de 8; sem memória: a caixa da linha 0x245 e volta); a virtual +0x3c
/// (create_round_table: thiscall(this, fase 2, &rodadas, &extra, 0), ret 0x10); a fase nova (operator new 0x944cf1 de
/// 0xb2; Cup_Stage::Cup_Stage 0x522e00, thiscall, ret 0x3c: a liga, 2, a lista, as rodadas, +4, a tabela, o ano +0x40,
/// 2, 1, o extra, 0x14, 0, 0, 0 e nenhuma semente) na terceira vaga das fases ([+0xc]+8); a lista e a tabela liberadas
/// (free 0x9452ca, nessa ordem); Cup::set_up_round (0x51c800, thiscall(fase, 0), ret 4) chamada com a fase da terceira
/// vaga mesmo nula (como na original); sem fase, a caixa da linha 0x25c; com ela, +0x30 = 2. O quadro SEH da original
/// só cobre o construtor (sem destrutor).
unsafe extern "thiscall" fn cqq9_final_da_j1(this: usize) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let lista = alocar(8);
    if lista == 0 {
        return aviso_em(0x9eabc8, 0x245);
    }
    core::ptr::write_unaligned(lista as *mut usize, cqq9_campeao_do_turno(this, 0)); // cqq9: o campeão do primeiro turno
    core::ptr::write_unaligned((lista + 4) as *mut usize, cqq9_campeao_do_turno(this, 1)); // cqq9: o campeão do segundo turno
    let tabela: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(cqq9_d(cqq9_d(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = tabela(this, 2, &mut rodadas, &mut extra, 0);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let p = novo(0xb2);
    // ponytail: a original empurra as rodadas, o extra e o ano em dwords com lixo nos 16 bits altos (as saídas da
    // virtual são words na pilha; o ano vai no dx); o construtor só lê as palavras.
    let fase = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(0x522e00usize);
        construir(p, this, 2, lista, rodadas as u16 as u32, cqq9_d(this + 4), t, cqq9_w(this + 0x40) as u16 as u32, 2, 1, extra as u16 as u32, 0x14, 0, 0, 0, core::ptr::null())
    } else {
        0
    };
    core::ptr::write_unaligned((cqq9_d(this + 0xc) + 8) as *mut usize, fase);
    liberar(lista);
    liberar(t);
    let montar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x51c800usize);
    montar(cqq9_d(cqq9_d(this + 0xc) + 8), 0);
    if cqq9_d(cqq9_d(this + 0xc) + 8) == 0 {
        return aviso_em(0x9eabc8, 0x25c);
    }
    crate::comparacao::ramo("cqq_9 final");
    core::ptr::write_unaligned((this + 0x30) as *mut u32, 2);
}

/// { 0x6636d0 } (thiscall, ret 0, jap_j1.cpp): `JapJ1::start_new_season()`, a virada do futebol japonês (a J1 comanda):
/// +0x4c = 0; o acesso e o rebaixamento (a virtual +0xb0, thiscall(this, 1), ret 4); a tabela liberada
/// (League::free_table 0x687970, thiscall(this, 0), ret 4); a lista de +0xba liberada e zerada; com estatísticas (+8),
/// Comp_Stats::archive_season_and_reset (0x4a1c50, thiscall(+8, 1), ret 4); com a fase atual ≥ 0, as fases 0..=+0x30
/// que existem destruídas (a virtual 0 com 1); o vetor das fases liberado e um novo de 3 (malloc 0x944e46 de 0xc,
/// +0x2c = 3; sem memória: a caixa da linha 0x1b4, a limpeza 0x5cc7d0, 0x5a8f60 e exit(−1) 0x945934, todas cdecl); o
/// ano (+0x40) sobe um e +0x30 = −1; então, cada um com a sua caixa se devolver 0: as regras (virtual +0x8c, 0x1be), a
/// tabela (0x6634e0, 0x1c5), os jogos (League::create_fixtures 0x6835c0, 0x1cb) e a ordem (League::sort_table 0x6827d0,
/// thiscall(this, 0), 0x1d2); depois a virada da J2 (a virtual +8 da competição de id [0x9cf768]), as posições previstas
/// (virtual +0x5c), as expectativas das diretorias (League::set_club_season_expectations 0x68aa80) e a renovação dos
/// árbitros (OfficialsManager::renew_officials 0x79cee0, thiscall([0xb74340], +4), ret 4), cujo eax ela devolve.
unsafe extern "thiscall" fn cqq9_virada_da_j1(this: usize) -> u32 {
    let virtual_ = |c: usize, desl: usize| -> u32 {
        let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cqq9_d(cqq9_d(c) + desl));
        f(c)
    };
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    core::ptr::write_unaligned((this + 0x4c) as *mut u32, 0);
    let cadeia: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(cqq9_d(cqq9_d(this) + 0xb0));
    cadeia(this, 1);
    let liberar_tabela: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x687970usize);
    liberar_tabela(this, 0);
    let l = cqq9_d(this + 0xba);
    if l != 0 {
        liberar(l);
        core::ptr::write_unaligned((this + 0xba) as *mut usize, 0);
    }
    let estatisticas = cqq9_d(this + 8);
    if estatisticas != 0 {
        let arquivar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x4a1c50usize);
        arquivar(estatisticas, 1);
    }
    if (cqq9_d(this + 0x30) as i32) >= 0 {
        let mut i = 0i32;
        loop {
            let f = cqq9_d(cqq9_d(this + 0xc) + 4 * i as usize);
            if f != 0 {
                let destruir: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(cqq9_d(cqq9_d(f)));
                destruir(f, 1);
            }
            i += 1;
            if i > cqq9_d(this + 0x30) as i32 {
                break;
            }
        }
    }
    liberar(cqq9_d(this + 0xc));
    core::ptr::write_unaligned((this + 0x2c) as *mut u32, 3);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let v = alocar(0xc);
    core::ptr::write_unaligned((this + 0xc) as *mut usize, v);
    if v == 0 {
        aviso_em(0x9eabc8, 0x1b4);
        for f in [0x5cc7d0usize, 0x5a8f60] {
            let f: unsafe extern "C" fn() = core::mem::transmute(f);
            f();
        }
        let sair: unsafe extern "C" fn(i32) = core::mem::transmute(0x945934usize);
        sair(-1);
    }
    let a = core::ptr::read_unaligned((this + 0x40) as *const i16);
    core::ptr::write_unaligned((this + 0x40) as *mut i16, a.wrapping_add(1)); // cqq9: o ano seguinte da J1
    core::ptr::write_unaligned((this + 0x30) as *mut i32, -1);
    crate::comparacao::ramo("cqq_9 virada da j1");
    let erro = if virtual_(this, 0x8c) == 0 {
        0x1be
    } else if core::mem::transmute::<usize, unsafe extern "thiscall" fn(usize) -> u32>(0x6634e0usize)(this) == 0 {
        0x1c5
    } else if core::mem::transmute::<usize, unsafe extern "thiscall" fn(usize) -> u32>(0x6835c0usize)(this) == 0 {
        0x1cb
    } else if core::mem::transmute::<usize, unsafe extern "thiscall" fn(usize, usize) -> u32>(0x6827d0usize)(this, 0) == 0 {
        0x1d2
    } else {
        0
    };
    if erro != 0 {
        aviso_em(0x9eabc8, erro);
        return 0;
    }
    virtual_(cqq9_d(cqq9_d(0xadadfc) + 4 * cqq9_d(0x9cf768)), 8);
    virtual_(this, 0x5c);
    let expectativas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68aa80usize);
    expectativas(this);
    let renovar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x79cee0usize);
    renovar(cqq9_d(0xb74340), cqq9_d(this + 4))
}

/// O comparador da fase seguinte: só a original (ponytail: ao chegar a 2 monta a final, que aloca e sorteia), com o
/// plano: na última fase, +0x30 igual; senão +0x30 + 1 (a final grava 2, o mesmo valor).
unsafe extern "thiscall" fn comparar_cqq9_fase_seguinte_da_j1(this: usize) {
    let antes = cqq9_d(this + 0x30) as i32;
    let ultima = antes >= (cqq9_d(this + 0x2c) as i32).wrapping_sub(1);
    orig::<unsafe extern "thiscall" fn(usize)>(1954)(this);
    let depois = cqq9_d(this + 0x30) as i32;
    PLACAR.registrar(depois == if ultima { antes } else { antes + 1 }, || format!("cqq9_fase_seguinte_da_j1({:#x}) antes={} depois={}", this, antes, depois));
}

/// O comparador da final: só a original (aloca a lista, a tabela e a fase e sorteia a rodada), com o plano feito antes:
/// os campeões dos turnos. Depois, a fase da terceira vaga existe, com +0x3e = 2, +4 = o +4 da liga e os dois campeões
/// na lista dela (+0xa7, 6 bytes; como conjunto), e +0x30 = 2.
unsafe extern "thiscall" fn comparar_cqq9_final_da_j1(this: usize) {
    let (a, b) = (cqq9_campeao_do_turno(this, 0), cqq9_campeao_do_turno(this, 1));
    orig::<unsafe extern "thiscall" fn(usize)>(1955)(this);
    let fase = cqq9_d(cqq9_d(this + 0xc) + 8);
    let igual = fase != 0 && {
        let l = cqq9_d(fase + 0xa7);
        let (x, y) = (cqq9_d(l), cqq9_d(l + 6));
        cqq9_w(fase + 0x3e) == 2 && cqq9_d(fase + 4) == cqq9_d(this + 4) && ((x, y) == (a, b) || (x, y) == (b, a)) && cqq9_d(this + 0x30) == 2
    };
    PLACAR.registrar(igual, || format!("cqq9_final_da_j1({:#x}) campeoes={:#x},{:#x} fase={:#x} +0x30={}", this, a, b, fase, cqq9_d(this + 0x30)));
}

/// O comparador da virada: só a original (ponytail: libera e aloca tabelas e fases, sorteia jogos e árbitros e vira a
/// J2; não pode rodar duas vezes), conferindo o que só ela garante: o ano sobe um e +0x2c = 3.
unsafe extern "thiscall" fn comparar_cqq9_virada_da_j1(this: usize) -> u32 {
    let antes = cqq9_w(this + 0x40);
    let r = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1956)(this);
    let depois = cqq9_w(this + 0x40);
    PLACAR.registrar(depois == antes.wrapping_add(1) && cqq9_d(this + 0x2c) == 3, || format!("cqq9_virada_da_j1({:#x}) ano antes={} depois={} +0x2c={}", this, antes, depois, cqq9_d(this + 0x2c)));
    r
}

// ---------- 280a leva: as sedes dos torneios: a tabela padrão, a notícia das candidaturas e a carga do HostCountry.tmp (host_country.cpp) ----------

unsafe fn cqo3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqo3_pd(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v)
}

unsafe fn cqo3_w(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

unsafe fn cqo3_pw(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, v)
}

/// As datas de referência das decisões (Date::set 0x54a130: dia do mês, mês a partir de 0, ano 1997, sem dia da
/// semana): o dia do ano delas vai ao +0x1f das entradas do laço seguinte.
const CQO3_DATAS: [[u32; 4]; 7] = [
    [0xa, 0x6, 0x7cd, 0xffffffff], // cqo3: 10 de julho, a Copa do Mundo
    [0x14, 0x6, 0x7cd, 0xffffffff],
    [0x14, 0x6, 0x7cd, 0xffffffff],
    [0x14, 0x6, 0x7cd, 0xffffffff],
    [0x14, 0x6, 0x7cd, 0xffffffff],
    [0x14, 0x6, 0x7cd, 0xffffffff],
    [0x14, 0x8, 0x7cd, 0xffffffff],
];

/// Os laços das entradas de uma competição: (início na tabela, entradas, a competição [global], primeiro ano, passo
/// dos anos, intervalo +0x21). Na ordem: a Copa do Mundo ([0x9cf79c], 1998..2026 de 4 em 4), a Eurocopa ([0x9cf7a4],
/// 2000..2028), a Copa Africana ([0x9cf7ac], 2000..2014 de 2 em 2), as Olimpíadas ([0x9cf7a0], 2000..2028), a Copa
/// América ([0x9cf7a8], 1999..2009 de 2 em 2), [0x9cf95c] (1999..2004, ano a ano) e a Copa da Ásia ([0x9cf790],
/// 2000..2016) [inferência: os nomes pelos endereços da comunidade, docs/ENDERECOS_COMUNIDADE.md].
const CQO3_LACOS: [(u16, u8, u32, u16, u16, u8); 7] = [
    (0x0, 8, 0x9cf79c, 1998, 4, 0x20),
    (0x110, 8, 0x9cf7a4, 2000, 4, 0x20), // cqo3: a Eurocopa
    (0x220, 8, 0x9cf7ac, 2000, 2, 0x10), // cqo3: a Copa Africana
    (0x330, 8, 0x9cf7a0, 2000, 4, 0x20),
    (0x462, 6, 0x9cf7a8, 1999, 2, 0xc), // cqo3: a Copa América
    (0x5fa, 6, 0x9cf95c, 1999, 1, 0x6), // cqo3: as edições anuais
    (0x6c6, 5, 0x9cf790, 2000, 4, 0x14),
];

/// As gravações da tabela padrão, na ordem da original (geradas da desmontagem de 0x5f999a..0x5fa640): (deslocamento,
/// tipo, valor); tipo 0 = dword do global `valor` (uma nação ou competição), 1 = dword `valor` (−1 = sem sede), 2 =
/// byte (o +0x1e de cada entrada: 0/1 a candidatura escolhida, −1 votação aberta, −2 fechada, −3 a sede padrão
/// fixa), 3 = word 0 (ano 0 e dia 0 das entradas sem edição), 4 = a data `CQO3_DATAS[valor]`, 5 = o laço
/// `CQO3_LACOS[valor]`.
const CQO3_TABELA: [(u16, u8, u32); 281] = [
    (0x0, 4, 0x0), (0x0, 5, 0x0), (0x6, 0, 0x9cf300), (0xa, 1, 0xffffffff), (0xe, 0, 0x9cf488), (0x12, 1, 0xffffffff),
    (0x16, 0, 0x9cf434), (0x1a, 1, 0xffffffff), (0x1e, 2, 0x0), (0x28, 0, 0x9cf484), (0x2c, 0, 0x9cf36c), (0x30, 0, 0x9cf2e4),
    (0x34, 1, 0xffffffff), (0x38, 0, 0x9cf360), (0x3c, 1, 0xffffffff), (0x40, 2, 0x0), (0x4a, 0, 0x9cf314), (0x4e, 1, 0xffffffff),
    (0x52, 0, 0x9cf2e4), (0x56, 1, 0xffffffff), (0x5a, 0, 0x9cf218), (0x5e, 1, 0xffffffff), (0x62, 2, 0x0), (0x6c, 0, 0x9cf260),
    (0x70, 1, 0xffffffff), (0x74, 0, 0x9cf218), (0x78, 1, 0xffffffff), (0x7c, 0, 0x9cf480), (0x80, 1, 0xffffffff), (0x84, 2, 0xfe),
    (0x8e, 0, 0x9cf4a8), (0x92, 0, 0x9cf408), (0x96, 0, 0x9cf488), (0x9a, 1, 0xffffffff), (0x9e, 0, 0x9cf4ac), (0xa2, 1, 0xffffffff),
    (0xa6, 2, 0xfe), (0xb0, 0, 0x9cf3d0), (0xb4, 1, 0xffffffff), (0xb8, 0, 0x9cf4f4), (0xbc, 1, 0xffffffff), (0xc0, 0, 0x9cf280),
    (0xc4, 1, 0xffffffff), (0xc8, 2, 0xfe), (0xd2, 0, 0x9cf360), (0xd6, 1, 0xffffffff), (0xda, 0, 0x9cf314), (0xde, 1, 0xffffffff),
    (0xe2, 0, 0x9cf2e4), (0xe6, 1, 0xffffffff), (0xea, 2, 0xfe), (0xf4, 0, 0x9cf260), (0xf8, 1, 0xffffffff), (0xfc, 0, 0x9cf218),
    (0x100, 1, 0xffffffff), (0x104, 0, 0x9cf4f8), (0x108, 1, 0xffffffff), (0x10c, 2, 0xfe), (0x0, 4, 0x1), (0x0, 5, 0x1),
    (0x116, 0, 0x9cf338), (0x11a, 0, 0x9cf244), (0x11e, 0, 0x9cf360), (0x122, 1, 0xffffffff), (0x126, 0, 0x9cf408), (0x12a, 1, 0xffffffff),
    (0x12e, 2, 0x0), (0x138, 0, 0x9cf344), (0x13c, 0, 0x9cf228), (0x140, 0, 0x9cf434), (0x144, 1, 0xffffffff), (0x148, 0, 0x9cf488),
    (0x14c, 1, 0xffffffff), (0x150, 2, 0x1), (0x15a, 0, 0x9cf45c), (0x15e, 0, 0x9cf510), (0x162, 0, 0x9cf4a8), (0x166, 1, 0xffffffff),
    (0x16a, 0, 0x9cf4ac), (0x16e, 1, 0xffffffff), (0x172, 2, 0xfe), (0x17c, 0, 0x9cf300), (0x180, 1, 0xffffffff), (0x184, 0, 0x9cf408),
    (0x188, 1, 0xffffffff), (0x18c, 0, 0x9cf2c0), (0x190, 1, 0xffffffff), (0x194, 2, 0xfe), (0x19e, 0, 0x9cf2dc), (0x1a2, 1, 0xffffffff),
    (0x1a6, 0, 0x9cf4ac), (0x1aa, 1, 0xffffffff), (0x1ae, 0, 0x9cf51c), (0x1b2, 1, 0xffffffff), (0x1b6, 2, 0xfe), (0x1c0, 0, 0x9cf4ac),
    (0x1c4, 1, 0xffffffff), (0x1c8, 0, 0x9cf244), (0x1cc, 1, 0xffffffff), (0x1d0, 0, 0x9cf338), (0x1d4, 1, 0xffffffff), (0x1d8, 2, 0xfe),
    (0x1e2, 0, 0x9cf360), (0x1e6, 1, 0xffffffff), (0x1ea, 0, 0x9cf314), (0x1ee, 1, 0xffffffff), (0x1f2, 0, 0x9cf2e4), (0x1f6, 1, 0xffffffff),
    (0x1fa, 2, 0xfe), (0x204, 0, 0x9cf31c), (0x208, 1, 0xffffffff), (0x20c, 0, 0x9cf488), (0x210, 1, 0xffffffff), (0x214, 0, 0x9cf300),
    (0x218, 1, 0xffffffff), (0x21c, 2, 0xfe), (0x0, 4, 0x2), (0x0, 5, 0x2), (0x226, 0, 0x9cf318), (0x22a, 0, 0x9cf400),
    (0x23e, 2, 0xfd), (0x248, 0, 0x9cf2d8), (0x24c, 1, 0xffffffff), (0x260, 2, 0xfd), (0x26a, 0, 0x9cf49c), (0x26e, 1, 0xffffffff),
    (0x282, 2, 0xfd), (0x28c, 0, 0x9cf4d8), (0x290, 1, 0xffffffff), (0x2a4, 2, 0xfd), (0x2ae, 0, 0x9cf27c), (0x2b2, 1, 0xffffffff),
    (0x2c6, 2, 0xfd), (0x2d0, 0, 0x9cf400), (0x2d4, 1, 0xffffffff), (0x2e8, 2, 0xfd), (0x2f2, 0, 0x9cf3d8), (0x2f6, 1, 0xffffffff),
    (0x30a, 2, 0xfd), (0x314, 0, 0x9cf208), (0x318, 1, 0xffffffff), (0x32c, 2, 0xfd), (0x0, 4, 0x3), (0x0, 5, 0x3),
    (0x336, 0, 0x9cf224), (0x33a, 1, 0xffffffff), (0x34e, 2, 0xfd), (0x358, 0, 0x9cf31c), (0x35c, 1, 0xffffffff), (0x370, 2, 0xfd),
    (0x37a, 0, 0x9cf314), (0x37e, 1, 0xffffffff), (0x392, 2, 0xfd), (0x39c, 0, 0x9cf360), (0x3a0, 1, 0xffffffff), (0x3b4, 2, 0xfd),
    (0x3be, 0, 0x9cf36c), (0x3c2, 1, 0xffffffff), (0x3d6, 2, 0xfd), (0x3e0, 0, 0x9cf3d0), (0x3e4, 1, 0xffffffff), (0x3f8, 2, 0xfd),
    (0x402, 0, 0x9cf280), (0x406, 1, 0xffffffff), (0x41a, 2, 0xfd), (0x424, 0, 0x9cf488), (0x428, 1, 0xffffffff), (0x43c, 2, 0xfd),
    (0x440, 0, 0x9cf6e4), (0x444, 3, 0x0), (0x45f, 3, 0x0), (0x461, 2, 0x0), (0x446, 0, 0x9cf36c), (0x44a, 1, 0xffffffff),
    (0x45e, 2, 0xfd), (0x0, 4, 0x4), (0x0, 5, 0x4), (0x468, 0, 0x9cf420), (0x46c, 1, 0xffffffff), (0x480, 2, 0xfd),
    (0x48a, 0, 0x9cf29c), (0x48e, 1, 0xffffffff), (0x4a2, 2, 0xfd), (0x4ac, 0, 0x9cf428), (0x4b0, 1, 0xffffffff), (0x4c4, 2, 0xfd),
    (0x4ce, 0, 0x9cf504), (0x4d2, 1, 0xffffffff), (0x4e6, 2, 0xfd), (0x4f0, 0, 0x9cf218), (0x4f4, 1, 0xffffffff), (0x508, 2, 0xfd),
    (0x512, 0, 0x9cf260), (0x516, 1, 0xffffffff), (0x52a, 2, 0xfd), (0x52e, 0, 0x9cf6e8), (0x532, 3, 0x0), (0x54d, 3, 0x0),
    (0x54f, 2, 0x0), (0x534, 0, 0x9cf45c), (0x538, 1, 0xffffffff), (0x54c, 2, 0xfd), (0x550, 0, 0x9cf6f0), (0x554, 3, 0x0),
    (0x56f, 3, 0x0), (0x571, 2, 0x0), (0x556, 0, 0x9cf2c0), (0x55a, 1, 0xffffffff), (0x56e, 2, 0xfd), (0x572, 0, 0x9cf6ec),
    (0x576, 3, 0x0), (0x591, 3, 0x0), (0x593, 2, 0x0), (0x578, 0, 0x9cf300), (0x57c, 1, 0xffffffff), (0x590, 2, 0xfd),
    (0x594, 0, 0x9cf7bc), (0x598, 3, 0x0), (0x5b3, 3, 0x0), (0x5b5, 2, 0x0), (0x59a, 0, 0x9cf488), (0x59e, 1, 0xffffffff),
    (0x5b2, 2, 0xfd), (0x5b6, 0, 0x9cf7cc), (0x5ba, 3, 0x0), (0x5d5, 3, 0x0), (0x5d7, 2, 0x0), (0x5bc, 0, 0x9cf224),
    (0x5c0, 1, 0xffffffff), (0x5d4, 2, 0xfd), (0x5d8, 0, 0x9cf76c), (0x5dc, 3, 0x0), (0x5f7, 3, 0x0), (0x5f9, 2, 0x0),
    (0x5de, 0, 0x9cf224), (0x5e2, 0, 0x9cf3f4), (0x5f6, 2, 0xfd), (0x0, 4, 0x5), (0x0, 5, 0x5), (0x600, 0, 0x9cf420),
    (0x604, 1, 0xffffffff), (0x618, 2, 0xfd), (0x622, 0, 0x9cf4f8), (0x626, 1, 0xffffffff), (0x63a, 2, 0xfd), (0x644, 0, 0x9cf218),
    (0x648, 1, 0xffffffff), (0x65c, 2, 0xfd), (0x666, 0, 0x9cf3d0), (0x66a, 1, 0xffffffff), (0x67e, 2, 0xfd), (0x688, 0, 0x9cf4f4),
    (0x68c, 1, 0xffffffff), (0x6a0, 2, 0xfd), (0x6aa, 0, 0x9cf260), (0x6ae, 1, 0xffffffff), (0x6c2, 2, 0xfd), (0x0, 4, 0x6),
    (0x0, 5, 0x6), (0x6cc, 0, 0x9cf390), (0x6d0, 1, 0xffffffff), (0x6e4, 2, 0xfd), (0x6ee, 0, 0x9cf46c), (0x6f2, 1, 0xffffffff),
    (0x706, 2, 0xfd), (0x710, 0, 0x9cf36c), (0x714, 1, 0xffffffff), (0x728, 2, 0xfd), (0x732, 0, 0x9cf298), (0x736, 1, 0xffffffff),
    (0x74a, 2, 0xfd), (0x754, 0, 0x9cf484), (0x758, 1, 0xffffffff), (0x76c, 2, 0xfd), (0x770, 0, 0x9cf964), (0x774, 3, 0x0),
    (0x78f, 3, 0x0), (0x791, 2, 0x0), (0x776, 0, 0x9cf484), (0x77a, 1, 0xffffffff), (0x78e, 2, 0xfd),
];

/// { 0x5f9970 } (thiscall, ret 0, host_country.cpp, `HostCountry::init_default_table`; pelo construtor 0x5f97c0, sem o
/// HostCountry.tmp; quadro SEH só pela Data local): a tabela padrão das sedes (57 entradas de 0x22 bytes em [this],
/// conta em +4: +0 a competição, +4 o ano, +6/+0xa, +0xe/+0x12, +0x16/+0x1a as três candidaturas (duas nações cada),
/// +0x1e a escolhida, +0x1f o dia da decisão, +0x21 o intervalo entre edições), pelas gravações de `CQO3_TABELA`;
/// depois o dia da decisão de cada entrada anda um sorteio de −35 a +34 (0x90cfc0(70)); a entrada que não é padrão
/// fixa (−3) e tem segunda ou terceira candidatura completa leva a caixa (linha 0x226) e perde as duas.
unsafe extern "thiscall" fn tabela_das_sedes(this: usize) {
    let definir: unsafe extern "thiscall" fn(*mut crate::data::Data, u32, u32, u32, u32) -> usize = core::mem::transmute(0x54a130usize);
    let mut d = crate::data::Data { dia: 40, ano: 1972, bissexto: 1 };
    for &(off, t, v) in CQO3_TABELA.iter() {
        let e = cqo3_d(this) + off as usize;
        match t {
            0 => cqo3_pd(e, cqo3_d(v as usize) as u32),
            1 => cqo3_pd(e, v),
            2 => *(e as *mut u8) = v as u8,
            3 => cqo3_pw(e, v as u16),
            4 => {
                let a = CQO3_DATAS[v as usize];
                definir(&mut d, a[0], a[1], a[2], a[3]);
            }
            _ => {
                let (ini, n, g, ano, passo, intervalo) = CQO3_LACOS[v as usize];
                let mut e = cqo3_d(this) + ini as usize;
                let mut y = ano;
                for _ in 0..n {
                    cqo3_pw(e + 4, y);
                    cqo3_pd(e, cqo3_d(g as usize) as u32);
                    cqo3_pw(e + 0x1f, d.dia as u16);
                    *((e + 0x21) as *mut u8) = intervalo;
                    e += 0x22;
                    y = y.wrapping_add(passo);
                }
            }
        }
    }
    let sortear: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let n = cqo3_w(this + 4) as i16;
    let mut k: i16 = 0;
    while k < n {
        let o = (k as i32 * 0x22) as usize;
        let p = cqo3_d(this) + o + 0x1f;
        crate::comparacao::ramo("cqo_3 sorteio do dia");
        let r = sortear(0x46); // cqo3: o sorteio do dia da decisão
        cqo3_pw(p, cqo3_w(p).wrapping_add((r - 0x23) as u16));
        let e = cqo3_d(this) + o;
        if *((e + 0x1e) as *const u8) != 0xfd && (cqo3_d(e + 0x12) as u32 != u32::MAX || cqo3_d(e + 0x1a) as u32 != u32::MAX) {
            aviso_em(0x9cf0d8, 0x226);
            cqo3_pd(cqo3_d(this) + o + 0x12, u32::MAX);
            cqo3_pd(cqo3_d(this) + o + 0x1a, u32::MAX);
        }
        k += 1;
    }
}

/// Lado a lado (a cópia sem contar os sorteios) pela tabela (conta +4 × 0x22 bytes), pelo estado do sorteador e por
/// [0xb67a34].
unsafe extern "thiscall" fn comparar_tabela_das_sedes(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1957);
    let n = cqo3_w(this + 4) as i16;
    if cqo3_d(this) == 0 || n <= 0 {
        return o(this);
    }
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(cqo3_d(this), n as usize * 0x22), (0xb67a34, 4)]);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this), || crate::aleatorio::sem_contar(|| tabela_das_sedes(this)));
    PLACAR.registrar(igual, || format!("tabela_das_sedes: {:x?} x {:x?}", mo, mr).chars().take(4000).collect());
}

/// { 0x5faec0 } (thiscall, ret 4, host_country.cpp, `HostCountry::post_host_news(lista)`; pelo calendário das sedes
/// 0x5fa850; quadro SEH só pelas notícias locais, sem destrutor): para cada entrada da lista (cursor reposto, 0x68d1b0),
/// fora a candidatura escolhida (ou a padrão fixa) da edição de daqui a 8 anos ([0xae2c92] + 8), as Olimpíadas
/// ([0x9cf7a0]) e a Copa do Mundo de 2006 na Alemanha ([0x9cf79c], 0x7d6, [0x9cf314]): com a votação aberta (−1) a
/// notícia 4 (as candidaturas: competição, ano e as seis nações); senão a notícia 5 (a escolhida: competição, ano e as
/// duas sedes dela, a primeira na padrão fixa), montadas por 0x77bbe0/0x785790 e entregues aos técnicos do continente
/// da competição (tabela [0xae23d0], 0x6b bytes, +0x59; 0x786640) ou a todos (0x786780).
unsafe extern "thiscall" fn noticia_das_sedes(_this: usize, lista: usize) {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let dado: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x785790usize);
    let continente: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x786640usize);
    let todos: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x786780usize);
    crate::comparacao::ramo("cqo_3 noticia das sedes");
    cqo3_pd(lista + 0xa, cqo3_d(lista + 2) as u32);
    let mut it = proximo(lista);
    while it != 0 {
        let e = cqo3_d(it);
        let k = *((e + 0x1e) as *const i8);
        let pular = ((k > 0 || k == -3) && cqo3_w(0xae2c92) as u32 == (cqo3_w(e + 4) as u32).wrapping_sub(8))
            || cqo3_d(e) == cqo3_d(0x9cf7a0)
            || (cqo3_d(e) == cqo3_d(0x9cf79c) && cqo3_w(e + 4) == 0x7d6 && cqo3_d(e + 6) == cqo3_d(0x9cf314));
        if !pular {
            // a notícia local (0xe8 bytes, como nas outras cópias)
            let mut n = [0u8; 0xe8];
            let p = n.as_mut_ptr() as usize;
            if k == -1 {
                construir(p, 4, 0);
                dado(p, 0, cqo3_d(cqo3_d(it)) as u32);
                dado(p, 1, cqo3_w(cqo3_d(it) + 4) as u32);
                for (i, off) in [6usize, 0xa, 0xe, 0x12, 0x16, 0x1a].iter().enumerate() {
                    dado(p, 2 + i as u32, cqo3_d(cqo3_d(it) + off) as u32);
                }
            } else {
                construir(p, 5, 0);
                let mut b = *((cqo3_d(it) + 0x1e) as *const i8);
                if b == -3 {
                    b = 0;
                }
                dado(p, 0, cqo3_d(cqo3_d(it)) as u32);
                dado(p, 1, cqo3_w(cqo3_d(it) + 4) as u32);
                let o = (b as i32 * 8) as usize;
                dado(p, 2, cqo3_d(cqo3_d(it).wrapping_add(o) + 6) as u32);
                dado(p, 3, cqo3_d(cqo3_d(it).wrapping_add(o) + 0xa) as u32);
            }
            let id = cqo3_d(cqo3_d(it));
            let c = cqo3_d(cqo3_d(0xae23d0).wrapping_add(id.wrapping_mul(0x6b)) + 0x59);
            if c != 0 {
                continente(cqo3_d(0xb7431c), p, c);
            } else {
                todos(cqo3_d(0xb7431c), p);
            }
        }
        it = proximo(lista);
    }
}

// ponytail: entrega notícias (aloca); só a original. A cópia leva o ramo.
unsafe extern "thiscall" fn comparar_noticia_das_sedes(this: usize, lista: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1958)(this, lista)
}

/// { 0x5fb160 } (thiscall, ret 0, host_country.cpp, `HostCountry::load_temp_file`; pelo construtor 0x5f97c0): lê o
/// HostCountry.tmp da pasta CM3_TEMP (0x90bde0, fopen 0x945b00 "rb"): a conta (word, +4) e as entradas de 0x22 bytes
/// num bloco novo ([this], 0x944e46); falhando a leitura, a caixa (linhas 0x448/0x452/0x45c), fecha (0x945a09) e, com
/// o bloco, o solta (0x9452ca). Devolve 1 com tudo lido, 0 sem o arquivo ou com erro.
unsafe extern "thiscall" fn carregar_sedes(this: usize) -> u32 {
    let caminho: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x90bde0usize);
    let abrir: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x945b00usize);
    let ler: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x945bfeusize);
    let fechar: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x945a09usize);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let f = abrir(caminho(0x987da0, 0x9cf10c), 0x98f928);
    if f == 0 {
        crate::comparacao::ramo("cqo_3 sem o arquivo");
        return 0;
    }
    if ler(this + 4, 2, 1, f) != 1 {
        aviso_em(0x9cf0d8, 0x448);
        fechar(f);
        return 0;
    }
    let n = cqo3_w(this + 4) as i16 as i32;
    let b = alocar((n * 0x22) as u32);
    cqo3_pd(this, b as u32);
    if b == 0 {
        aviso_em(0x9cf0d8, 0x452);
        fechar(f);
        return 0;
    }
    let n = cqo3_w(this + 4) as i16 as i32 as u32;
    if ler(b, 0x22, n, f) != cqo3_w(this + 4) as i16 as i32 as u32 {
        aviso_em(0x9cf0d8, 0x45c);
        fechar(f);
        soltar(cqo3_d(this));
        return 0;
    }
    fechar(f);
    1
}

// ponytail: abre o arquivo e aloca; só a original. A cópia leva os ramos.
unsafe extern "thiscall" fn comparar_carregar_sedes(this: usize) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize) -> u32>(1959)(this)
}
