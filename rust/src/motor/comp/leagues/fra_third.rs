use super::*;

/// Uma rodada da tabela de 0x41 bytes: a data (dia, mês 0..11, anos a somar ao ano da liga +0x40, dia da semana para
/// onde a data é levada), o código do byte +4 ([inferência] o turno) e as faixas de TV em ordem (dia da semana, turno,
/// quantos jogos; a lista termina em −1).
pub(crate) type Cqo4Rodada = (i8, i8, i8, i8, i8, &'static [(i8, i8, i8)]);

/// Writes league rounds from `first` in table t: date (0x68a160) and match slots (0x68a1c0) of each round.
///
/// [pt] As rodadas `primeira`.. da tabela `t`, na ordem da original: para cada uma, a data (0x68a160: push 0; push ano;
/// push código; push semana; push anos; push mês; push dia; push i; push t; add esp -> cdecl: o valor 0) e as
/// faixas (0x68a1c0: push 0; push c; push b; push a; push j; push i; push t; add esp -> cdecl, valor 0).
pub(crate) unsafe fn fra_third_write_league_rounds(t: usize, ano: u32, primeira: i32, tabela: &[Cqo4Rodada]) {
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

/// Collects the bytes of an n-round league table the game reads: date, +0x3d..+0x40 and match slots up to the
/// terminator in the guide table.
///
/// [pt] Os bytes que a tabela de `n` rodadas tem gravados e que o jogo lê: de cada rodada, a data (+0..+4) e +0x3d..+0x40,
/// e as faixas até a que termina a lista (a de dia −1 na tabela `guia`, a da original); o resto da memória alocada não
/// é escrito.
pub(crate) unsafe fn written_league_round_bytes(t: usize, guia: usize, n: usize) -> Vec<u8> {
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
pub(crate) struct Cqo4Liga {
    pub(crate) rodadas: i16,
    pub(crate) arquivo: usize,
    pub(crate) linha: u32,
    pub(crate) tabela: &'static [Cqo4Rodada],
    pub(crate) ramo: &'static str,
}

/// Common create_schedule body: at stage −1 sets outputs, allocates rounds × 0x41 bytes and writes the league's dates
/// and slots; returns the table (0 at other stages).
///
/// [pt] O corpo comum (thiscall(liga, fase, &rodadas, &extra, &é_liga), ret 0x10; sem estado salvo): só a fase principal
/// (−1; as outras devolvem 0 sem gravar nada): é_liga (se pedido) = 1, as rodadas, extra = 0, a tabela de rodadas × 0x41
/// bytes (malloc 0x944e46: push; add esp, 4 -> cdecl; sem memória: a caixa e 0) com as datas e as faixas da liga.
/// Devolve a tabela.
pub(crate) unsafe fn create_league_schedule(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32, p: &Cqo4Liga) -> usize {
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
        show_error_at(p.arquivo, p.linha);
        return 0;
    }
    crate::comparacao::ramo(p.ramo);
    let ano = core::ptr::read_unaligned((this + 0x40) as *const u16) as u32;
    fra_third_write_league_rounds(t, ano, 0, p.tabela);
    t
}

/// Side-by-side comparer for create_schedule: same outputs and same written round bytes; frees the copy's table.
///
/// [pt] Lado a lado: a original com as saídas de quem chama, a cópia com saídas próprias, a partir do mesmo [0xb67a34]; as
/// saídas e os bytes gravados das duas tabelas têm de ser iguais (a da cópia é liberada: 0x9452ca, cdecl).
pub(crate) unsafe fn compare_league_schedule(nome: &str, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32,
                       original: impl FnOnce() -> usize, copia: impl FnOnce(*mut i16, *mut i16, *mut i32) -> usize) -> usize {
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let (mut r2, mut e2, mut l2) = (0i16, 0i16, 0i32);
    let reg = [(0xb67a34usize, 4usize)];
    let pl2 = if liga.is_null() { core::ptr::null_mut() } else { &mut l2 as *mut i32 };
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, original, || copia(&mut r2, &mut e2, pl2));
    let saidas = fase as u8 != 0xff || (*rodadas == r2 && *extra == e2 && (liga.is_null() || *liga == l2));
    let tabela = if a != 0 && b != 0 {
        let n = *rodadas as usize;
        written_league_round_bytes(a, a, n) == written_league_round_bytes(b, a, n)
    } else {
        a == 0 && b == 0
    };
    if b != 0 {
        soltar(b);
    }
    PLACAR.registrar(igual && saidas && tabela, || format!("{}(fase {}): saídas {} tabela {}", nome, fase as u8 as i8, saidas, tabela));
    a
}

/// a terceira divisão (o National), 38 rodadas de 7/ago a 20/mai, sem faixas de TV
pub(crate) const CQO4_TERCEIRA: [Cqo4Rodada; 38] = [
    (7, 7, 0, 5, 2, &[(-1, -1, -1)]), // 0: 7/ago
    (13, 7, 0, 4, 2, &[(-1, -1, -1)]), // 1: 13/ago
    (17, 7, 0, 1, 2, &[(-1, -1, -1)]), // 2: 17/ago
    (21, 7, 0, 5, 2, &[(-1, -1, -1)]), // 3: 21/ago
    (28, 7, 0, 5, 2, &[(-1, -1, -1)]), // 4: 28/ago
    (3, 8, 0, 4, 2, &[(-1, -1, -1)]), // 5: 3/set
    (7, 8, 0, 1, 2, &[(-1, -1, -1)]), // 6: 7/set
    (11, 8, 0, 5, 2, &[(-1, -1, -1)]), // 7: 11/set
    (18, 8, 0, 5, 2, &[(-1, -1, -1)]), // 8: 18/set
    (25, 8, 0, 5, 2, &[(-1, -1, -1)]), // 9: 25/set
    (2, 9, 0, 5, 2, &[(-1, -1, -1)]), // 10: 2/out
    (8, 9, 0, 4, 2, &[(-1, -1, -1)]), // 11: 8/out
    (12, 9, 0, 1, 2, &[(-1, -1, -1)]), // 12: 12/out
    (16, 9, 0, 5, 2, &[(-1, -1, -1)]), // 13: 16/out
    (30, 9, 0, 5, 2, &[(-1, -1, -1)]), // 14: 30/out
    (12, 10, 0, 4, 2, &[(-1, -1, -1)]), // 15: 12/nov
    (20, 10, 0, 5, 2, &[(-1, -1, -1)]), // 16: 20/nov
    (1, 11, 0, 2, 2, &[(-1, -1, -1)]), // 17: 1/dez
    (4, 11, 0, 5, 2, &[(-1, -1, -1)]), // 18: 4/dez
    (11, 11, 0, 5, 2, &[(-1, -1, -1)]), // 19: 11/dez
    (7, 0, 1, 4, 2, &[(-1, -1, -1)]), // 20: 7/jan (+1 ano)
    (15, 0, 1, 5, 2, &[(-1, -1, -1)]), // 21: 15/jan (+1 ano)
    (28, 0, 1, 4, 2, &[(-1, -1, -1)]), // 22: 28/jan (+1 ano)
    (5, 1, 1, 5, 2, &[(-1, -1, -1)]), // 23: 5/fev (+1 ano)
    (18, 1, 1, 4, 2, &[(-1, -1, -1)]), // 24: 18/fev (+1 ano)
    (26, 1, 1, 5, 2, &[(-1, -1, -1)]), // 25: 26/fev (+1 ano)
    (3, 2, 1, 4, 2, &[(-1, -1, -1)]), // 26: 3/mar (+1 ano)
    (11, 2, 1, 5, 2, &[(-1, -1, -1)]), // 27: 11/mar (+1 ano)
    (18, 2, 1, 5, 2, &[(-1, -1, -1)]), // 28: 18/mar (+1 ano)
    (25, 2, 1, 5, 2, &[(-1, -1, -1)]), // 29: 25/mar (+1 ano)
    (31, 2, 1, 4, 2, &[(-1, -1, -1)]), // 30: 31/mar (+1 ano)
    (8, 3, 1, 5, 2, &[(-1, -1, -1)]), // 31: 8/abr (+1 ano)
    (15, 3, 1, 5, 2, &[(-1, -1, -1)]), // 32: 15/abr (+1 ano)
    (21, 3, 1, 4, 2, &[(-1, -1, -1)]), // 33: 21/abr (+1 ano)
    (29, 3, 1, 5, 2, &[(-1, -1, -1)]), // 34: 29/abr (+1 ano)
    (5, 4, 1, 4, 2, &[(-1, -1, -1)]), // 35: 5/mai (+1 ano)
    (13, 4, 1, 5, 2, &[(-1, -1, -1)]), // 36: 13/mai (+1 ano)
    (20, 4, 1, 5, 2, &[(-1, -1, -1)]), // 37: 20/mai (+1 ano)
];

/// a segunda divisão, 38 rodadas de 31/jul a 20/mai; 18 rodadas com uma faixa de TV (dia 5, turno 1, 1 jogo)
pub(crate) const CQO4_SEGUNDA: [Cqo4Rodada; 38] = [
    (31, 6, 0, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 0: 31/jul
    (7, 7, 0, 5, 2, &[(-1, -1, -1)]), // 1: 7/ago
    (13, 7, 0, 4, 2, &[(-1, -1, -1)]), // 2: 13/ago
    (18, 7, 0, 1, 2, &[(-1, -1, -1)]), // 3: 18/ago
    (21, 7, 0, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 4: 21/ago
    (27, 7, 0, 4, 2, &[(-1, -1, -1)]), // 5: 27/ago
    (3, 8, 0, 4, 2, &[(-1, -1, -1)]), // 6: 3/set
    (7, 8, 0, 1, 2, &[(-1, -1, -1)]), // 7: 7/set
    (11, 8, 0, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 8: 11/set
    (18, 8, 0, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 9: 18/set
    (25, 8, 0, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 10: 25/set
    (1, 9, 0, 4, 2, &[(-1, -1, -1)]), // 11: 1/out
    (8, 9, 0, 4, 2, &[(-1, -1, -1)]), // 12: 8/out
    (16, 9, 0, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 13: 16/out
    (23, 9, 0, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 14: 23/out
    (30, 9, 0, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 15: 30/out
    (6, 10, 0, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 16: 6/nov
    (9, 10, 0, 1, 2, &[(-1, -1, -1)]), // 17: 9/nov
    (12, 10, 0, 4, 2, &[(-1, -1, -1)]), // 18: 12/nov
    (20, 10, 0, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 19: 20/nov
    (4, 11, 0, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 20: 4/dez
    (11, 11, 0, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 21: 11/dez
    (15, 0, 1, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 22: 15/jan (+1 ano)
    (26, 0, 1, 2, 2, &[(-1, -1, -1)]), // 23: 26/jan (+1 ano)
    (2, 1, 1, 2, 2, &[(-1, -1, -1)]), // 24: 2/fev (+1 ano)
    (5, 1, 1, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 25: 5/fev (+1 ano)
    (16, 1, 1, 2, 2, &[(-1, -1, -1)]), // 26: 16/fev (+1 ano)
    (26, 1, 1, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 27: 26/fev (+1 ano)
    (11, 2, 1, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 28: 11/mar (+1 ano)
    (24, 2, 1, 4, 2, &[(-1, -1, -1)]), // 29: 24/mar (+1 ano)
    (28, 2, 1, 1, 2, &[(-1, -1, -1)]), // 30: 28/mar (+1 ano)
    (8, 3, 1, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 31: 8/abr (+1 ano)
    (15, 3, 1, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 32: 15/abr (+1 ano)
    (21, 3, 1, 4, 2, &[(-1, -1, -1)]), // 33: 21/abr (+1 ano)
    (28, 3, 1, 4, 2, &[(-1, -1, -1)]), // 34: 28/abr (+1 ano)
    (5, 4, 1, 4, 2, &[(-1, -1, -1)]), // 35: 5/mai (+1 ano)
    (13, 4, 1, 5, 2, &[(-1, -1, -1)]), // 36: 13/mai (+1 ano)
    (20, 4, 1, 5, 2, &[(-1, -1, -1)]), // 37: 20/mai (+1 ano)
];

/// a divisão de baixo (de fundo), a mesma tabela da segunda, com a faixa também nas duas últimas rodadas
pub(crate) const CQO4_BAIXO: [Cqo4Rodada; 38] = [
    (31, 6, 0, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 0: 31/jul
    (7, 7, 0, 5, 2, &[(-1, -1, -1)]), // 1: 7/ago
    (13, 7, 0, 4, 2, &[(-1, -1, -1)]), // 2: 13/ago
    (18, 7, 0, 1, 2, &[(-1, -1, -1)]), // 3: 18/ago
    (21, 7, 0, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 4: 21/ago
    (27, 7, 0, 4, 2, &[(-1, -1, -1)]), // 5: 27/ago
    (3, 8, 0, 4, 2, &[(-1, -1, -1)]), // 6: 3/set
    (7, 8, 0, 1, 2, &[(-1, -1, -1)]), // 7: 7/set
    (11, 8, 0, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 8: 11/set
    (18, 8, 0, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 9: 18/set
    (25, 8, 0, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 10: 25/set
    (1, 9, 0, 4, 2, &[(-1, -1, -1)]), // 11: 1/out
    (8, 9, 0, 4, 2, &[(-1, -1, -1)]), // 12: 8/out
    (16, 9, 0, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 13: 16/out
    (23, 9, 0, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 14: 23/out
    (30, 9, 0, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 15: 30/out
    (6, 10, 0, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 16: 6/nov
    (9, 10, 0, 1, 2, &[(-1, -1, -1)]), // 17: 9/nov
    (12, 10, 0, 4, 2, &[(-1, -1, -1)]), // 18: 12/nov
    (20, 10, 0, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 19: 20/nov
    (4, 11, 0, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 20: 4/dez
    (11, 11, 0, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 21: 11/dez
    (15, 0, 1, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 22: 15/jan (+1 ano)
    (26, 0, 1, 2, 2, &[(-1, -1, -1)]), // 23: 26/jan (+1 ano)
    (2, 1, 1, 2, 2, &[(-1, -1, -1)]), // 24: 2/fev (+1 ano)
    (5, 1, 1, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 25: 5/fev (+1 ano)
    (16, 1, 1, 2, 2, &[(-1, -1, -1)]), // 26: 16/fev (+1 ano)
    (26, 1, 1, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 27: 26/fev (+1 ano)
    (11, 2, 1, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 28: 11/mar (+1 ano)
    (24, 2, 1, 4, 2, &[(-1, -1, -1)]), // 29: 24/mar (+1 ano)
    (28, 2, 1, 1, 2, &[(-1, -1, -1)]), // 30: 28/mar (+1 ano)
    (8, 3, 1, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 31: 8/abr (+1 ano)
    (15, 3, 1, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 32: 15/abr (+1 ano)
    (21, 3, 1, 4, 2, &[(-1, -1, -1)]), // 33: 21/abr (+1 ano)
    (28, 3, 1, 4, 2, &[(-1, -1, -1)]), // 34: 28/abr (+1 ano)
    (5, 4, 1, 4, 2, &[(-1, -1, -1)]), // 35: 5/mai (+1 ano)
    (13, 4, 1, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 36: 13/mai (+1 ano)
    (20, 4, 1, 5, 2, &[(5, 1, 1), (-1, -1, -1)]), // 37: 20/mai (+1 ano)
];

pub(crate) const CQO4_LIGA_TERCEIRA: Cqo4Liga = Cqo4Liga { rodadas: 0x26, arquivo: 0x9ccc80, linha: 0xa6, tabela: &CQO4_TERCEIRA, ramo: "cqo_4 calendario da terceira francesa" };
pub(crate) const CQO4_LIGA_SEGUNDA: Cqo4Liga = Cqo4Liga { rodadas: 0x26, arquivo: 0x9ccc0c, linha: 0xa7, tabela: &CQO4_SEGUNDA, ramo: "cqo_4 calendario da segunda francesa" };
pub(crate) const CQO4_LIGA_BAIXO: Cqo4Liga = Cqo4Liga { rodadas: 0x26, arquivo: 0x9ccbd0, linha: 0xa9, tabela: &CQO4_BAIXO, ramo: "cqo_4 calendario da divisao de baixo francesa" };

/// the National schedule: 38 rounds (home and away with 20 clubs), from 7 August to 20 May of the next year, mostly at
/// weekends and some midweek; round 21 already on 7 January; no TV slots (no match moves day); main stage only
/// Original: `FraThird::create_schedule` (0x5bfe30, fra_third.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x5bfe30 } (thiscall, ret 0x10, fra_third.cpp, 1.948 bytes, método +0x3c): `FraThird::create_schedule(fase, &rodadas, &extra,
/// &é_liga)`: o National, 38 rodadas (`CQO4_TERCEIRA`). Fonte 0x9ccc80, linha 0xa6.
pub(crate) unsafe extern "thiscall" fn fra_third_create_schedule(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    create_league_schedule(this, fase, rodadas, extra, liga, &CQO4_LIGA_TERCEIRA)
}

/// Comparer for `fra_third_create_schedule` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_fra_third_create_schedule(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_FRA_THIRD_CREATE_SCHEDULE);
    compare_league_schedule("fra_third_create_schedule", fase, rodadas, extra, liga, || o(this, fase, rodadas, extra, liga), |r, e, l| fra_third_create_schedule(this, fase, r, e, l))
}



/// the lower division (background) schedule: the same 38 dates as the second division, with the TV slot also in the
/// last two rounds
/// Original: `FraLower::create_schedule` (0x5bd920, fra_lower.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x5bd920 } (thiscall, ret 0x10, fra_lower.cpp, 2.311 bytes, método +0x3c): `FraLower::create_schedule`: a divisão de
/// baixo, 38 rodadas (`CQO4_BAIXO`). Fonte 0x9ccbd0, linha 0xa9.
pub(crate) unsafe extern "thiscall" fn fra_third_create_schedule_5bd920(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    create_league_schedule(this, fase, rodadas, extra, liga, &CQO4_LIGA_BAIXO)
}

/// Comparer for `fra_third_create_schedule_5bd920` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_fra_third_create_schedule_5bd920(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_FRA_THIRD_CREATE_SCHEDULE_5BD920);
    compare_league_schedule("fra_third_create_schedule_5bd920", fase, rodadas, extra, liga, || o(this, fase, rodadas, extra, liga), |r, e, l| fra_third_create_schedule_5bd920(this, fase, r, e, l))
}

/// the National: 4 go up in 2001 and 3 afterwards, 4 go down (no league below in the game); 3 substitutes and 3
/// substitutions
/// Original: `FraThird::set_up_rules` (0x5c0700, fra_third.cpp, thiscall, ret 0).
///
/// [pt] { 0x5c0700 } (thiscall, ret 0, fra_third.cpp): `FraThird::set_up_rules()`. O National: as comuns, 2 turnos, 3
/// pontos; sobem 4 em 2001 e 3 depois (+0xbe), caem 4 (+0xc1); a segunda acima ([0x9cf5d0]), nenhuma abaixo (+0x20
/// = −1), +0xa7 (os jogos da temporada) = −1 (recalculado pela 0x687430) e 3 reservas (+0x49).
pub(crate) unsafe extern "thiscall" fn fra_third_set_up_rules(this: usize) -> u32 {
    crate::comparacao::ramo("cqj_5 regras do national");
    fra_first_apply_league_rules(this, &national_division_rules(this))
}

/// Comparer for `fra_third_set_up_rules` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_fra_third_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_FRA_THIRD_SET_UP_RULES);
    fra_first_compare_league_rules("fra_third_set_up_rules", this, national_division_rules(this), || o(this), || fra_third_set_up_rules(this))
}

/// the National has 20 clubs; prize fund of 20,000 per place; TV pays 200,000 to each club
/// Original: `FraThird::create_table` (0x5c05d0, fra_third.cpp, thiscall, ret 0).
///
/// [pt] { 0x5c05d0 } (thiscall, ret 0, fra_third.cpp): `FraThird::create_table()`: 20 clubes ([0x9cf5d4]); cheia, o fundo
/// pela cota de 20.000 e a TV de 200.000 a cada clube; 1. Fonte 0x9ccc80, linha 0x15d.
pub(crate) unsafe extern "thiscall" fn fra_third_create_table(this: usize) -> u32 {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 0x14);
    if fra_first_fill_league_table(this, 0x9ccc80, 0x15d, KEY_CLUB_COMP_FRENCH_NATIONAL) != Some(true) {
        return 0;
    }
    set_prize_fund(this, 0x4e20);
    eng_prm_pay_tv_rights(this, 0x30d40, 0);
    1
}

/// Comparer for `fra_third_create_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_fra_third_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_FRA_THIRD_CREATE_TABLE);
    fra_first_compare_league_table("fra_third_create_table", this, 0x14, KEY_CLUB_COMP_FRENCH_NATIONAL, Some(0x4e20), || o(this))
}

/// the French third division (the National): discipline 0xc, only the league
/// Original: `FraThird::FraThird` (0x5bfb40, fra_third.cpp, thiscall, ret 8).
///
/// [pt] { 0x5bfb40 } (thiscall, ret 8, fra_third.cpp): `FraThird::FraThird(ano, registro)`: a terceira divisão francesa (o
/// National), disciplina 0xc, só a liga.
pub(crate) unsafe extern "thiscall" fn construct_fra_third(this: usize, ano: u32, registro: usize) -> usize {
    construct_division_league(this, ano, registro, &CQM5_FRANCA_3)
}

/// Comparer for `construct_fra_third` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_construct_fra_third(this: usize, ano: u32, registro: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_FRA_THIRD);
    compare_division_league_ctor("construct_fra_third", this, ano, registro, &CQM5_FRANCA_3, || o(this, ano, registro))
}
