use super::*;

/// Describes one round of the 0x41-byte table.
///
/// It holds the date (day, month 0..11, years to add to the league year +0x40, the weekday the date is moved to), the
/// code of byte +4 ([inference] the half of the season) and the TV slots in order (weekday, half, how many matches; the
/// list ends with −1).
///
/// [pt] Uma rodada da tabela de 0x41 bytes: a data (dia, mês 0..11, anos a somar ao ano da liga +0x40, dia da semana para
/// onde a data é levada), o código do byte +4 ([inferência] o turno) e as faixas de TV em ordem (dia da semana, turno,
/// quantos jogos; a lista termina em −1).
pub(crate) type FraThirdLeagueRound = (i8, i8, i8, i8, i8, &'static [(i8, i8, i8)]);

/// Sets the dates and match slots of rounds `primeira`.. of table `t`, in the original routine's order.
///
/// For each round, the date (0x68a160: push 0; push year; push code; push week; push years; push month; push day; push
/// i; push t; add esp -> cdecl: value 0) and the slots (0x68a1c0: push 0; push c; push b; push a; push j; push i; push
/// t; add esp -> cdecl, value 0).
///
/// [pt] As rodadas `primeira`.. da tabela `t`, na ordem da original: para cada uma, a data (0x68a160: push 0; push ano;
/// push código; push semana; push anos; push mês; push dia; push i; push t; add esp -> cdecl: o valor 0) e as
/// faixas (0x68a1c0: push 0; push c; push b; push a; push j; push i; push t; add esp -> cdecl, valor 0).
pub(crate) unsafe fn fra_third_write_league_rounds(t: usize, year: u32, first: i32, table: &[FraThirdLeagueRound]) {
    let data: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, i32, u32, u32) -> usize = core::mem::transmute(FN_LEAGUE_ROUND_SET_DATE);
    for (k, &(dia, mes, anos, semana, codigo, faixas)) in table.iter().enumerate() {
        let i = first + k as i32;
        data(t, i, dia as i32, mes as i32, anos as i32, semana as i32, codigo as i32, year, 0);
        for (j, &(a, b, c)) in faixas.iter().enumerate() {
            exe::league_round_set_match_slot(t, i, j as i32, a as i32, b as i32, c as i32, 0);
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
pub(crate) struct FraThirdSingleStageLeague {
    pub(crate) rounds: i16,
    pub(crate) file: usize,
    pub(crate) row: u32,
    pub(crate) table: &'static [FraThirdLeagueRound],
    pub(crate) branch: &'static str,
}

/// The common create_schedule body (thiscall(league, stage, &rounds, &extra, &é_liga), ret 0x10; no saved state): only
/// the main stage (−1; the others return 0 without writing anything).
///
/// é_liga (if requested) = 1, the rounds, extra = 0, the table of rounds × 0x41 bytes (malloc 0x944e46: push; add esp,
/// 4 -> cdecl; out of memory: the box and 0) with the league's dates and slots. Returns the table.
///
/// [pt] O corpo comum (thiscall(liga, fase, &rodadas, &extra, &é_liga), ret 0x10; sem estado salvo): só a fase principal
/// (−1; as outras devolvem 0 sem gravar nada): é_liga (se pedido) = 1, as rodadas, extra = 0, a tabela de rodadas × 0x41
/// bytes (malloc 0x944e46: push; add esp, 4 -> cdecl; sem memória: a caixa e 0) com as datas e as faixas da liga.
/// Devolve a tabela.
pub(crate) unsafe fn create_league_schedule(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32, p: &FraThirdSingleStageLeague) -> usize {
    if fase as u8 != 0xff {
        return 0;
    }
    if !liga.is_null() {
        *liga = 1;
    }
    *rodadas = p.rounds;
    *extra = 0;
    let t = exe::crt_malloc(p.rounds as u32 * 0x41);
    if t == 0 {
        show_error_at(p.file, p.row);
        return 0;
    }
    crate::comparison::branch(p.branch);
    let ano = core::ptr::read_unaligned((this + league::YEAR) as *const u16) as u32;
    fra_third_write_league_rounds(t, ano, 0, p.table);
    t
}

/// a terceira divisão (o National), 38 rodadas de 7/ago a 20/mai, sem faixas de TV
pub(crate) const FRENCH_THIRD_DIVISION_ROUNDS: [FraThirdLeagueRound; 38] = [
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

/// The second division: 38 rounds from 31 July to 20 May; 18 rounds with one TV slot (day 5, half 1, 1 match).
///
/// [pt] a segunda divisão, 38 rodadas de 31/jul a 20/mai; 18 rodadas com uma faixa de TV (dia 5, turno 1, 1 jogo)
pub(crate) const SECOND_DIVISION_ROUNDS: [FraThirdLeagueRound; 38] = [
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

/// The bottom division: the same table as the second one, with the TV slot also in the last two rounds.
///
/// [pt] a divisão de baixo (de fundo), a mesma tabela da segunda, com a faixa também nas duas últimas rodadas
pub(crate) const LOWER_DIVISION_ROUNDS: [FraThirdLeagueRound; 38] = [
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

pub(crate) const FRENCH_THIRD_LEAGUE: FraThirdSingleStageLeague = FraThirdSingleStageLeague { rounds: 0x26, file: 0x9ccc80, row: 0xa6, table: &FRENCH_THIRD_DIVISION_ROUNDS, branch: "cqo_4 calendario da terceira francesa" };
pub(crate) const FRENCH_SECOND_DIVISION_LEAGUE: FraThirdSingleStageLeague = FraThirdSingleStageLeague { rounds: 0x26, file: 0x9ccc0c, row: 0xa7, table: &SECOND_DIVISION_ROUNDS, branch: "cqo_4 calendario da segunda francesa" };
pub(crate) const LOWER_DIVISION_LEAGUE: FraThirdSingleStageLeague = FraThirdSingleStageLeague { rounds: 0x26, file: 0x9ccbd0, row: 0xa9, table: &LOWER_DIVISION_ROUNDS, branch: "cqo_4 calendario da divisao de baixo francesa" };

/// the National schedule: 38 rounds (home and away with 20 clubs), from 7 August to 20 May of the next year, mostly at
/// weekends and some midweek; round 21 already on 7 January; no TV slots (no match moves day); main stage only
/// Original: `FraThird::create_schedule` (0x5bfe30, fra_third.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x5bfe30 } (thiscall, ret 0x10, fra_third.cpp, 1.948 bytes, método +0x3c): `FraThird::create_schedule(fase, &rodadas, &extra,
/// &é_liga)`: o National, 38 rodadas (`FRENCH_THIRD_DIVISION_ROUNDS`). Fonte 0x9ccc80, linha 0xa6.
pub(crate) unsafe extern "thiscall" fn fra_third_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_league_schedule(this, stage, rounds, extra, league, &FRENCH_THIRD_LEAGUE)
}

/// Sets up the rules of the French National: 4 go up in 2001 and 3 afterwards, 4 go down (no league below in the game);
/// 3 substitutes and 3 substitutions.
///
/// `FraThird::set_up_rules()`: the common rules, 2 rounds, 3 points; 4 promoted in 2001 and 3 afterwards (`+0xbe`), 4
/// relegated (`+0xc1`); the second division above (`[0x9cf5d0]`), none below (`+0x20` = −1), `+0xa7` (the season's
/// matches) = −1 (recomputed by `0x687430`) and 3 substitutes (`+0x49`).
/// Original: `FraThird::set_up_rules` (0x5c0700, fra_third.cpp, thiscall, ret 0).
///
/// [pt] { 0x5c0700 } (thiscall, ret 0, fra_third.cpp): `FraThird::set_up_rules()`. O National: as comuns, 2 turnos, 3
/// pontos; sobem 4 em 2001 e 3 depois (+0xbe), caem 4 (+0xc1); a segunda acima ([0x9cf5d0]), nenhuma abaixo (+0x20
/// = −1), +0xa7 (os jogos da temporada) = −1 (recalculado pela 0x687430) e 3 reservas (+0x49).
pub(crate) unsafe extern "thiscall" fn fra_third_set_up_rules(this: usize) -> u32 {
    crate::comparison::branch("cqj_5 regras do national");
    fra_first_apply_league_rules(this, &national_division_rules(this))
}

/// the National has 20 clubs; prize fund of 20,000 per place; TV pays 200,000 to each club
/// Original: `FraThird::create_table` (0x5c05d0, fra_third.cpp, thiscall, ret 0).
///
/// [pt] { 0x5c05d0 } (thiscall, ret 0, fra_third.cpp): `FraThird::create_table()`: 20 clubes ([0x9cf5d4]); cheia, o fundo
/// pela cota de 20.000 e a TV de 200.000 a cada clube; 1. Fonte 0x9ccc80, linha 0x15d.
pub(crate) unsafe extern "thiscall" fn fra_third_create_table(this: usize) -> u32 {
    core::ptr::write_unaligned((this + league::CLUB_COUNT) as *mut i16, 0x14);
    if fra_first_fill_league_table(this, 0x9ccc80, 0x15d, KEY_CLUB_COMP_FRENCH_NATIONAL) != Some(true) {
        return 0;
    }
    set_prize_fund(this, 0x4e20);
    eng_prm_pay_tv_rights(this, 0x30d40, 0);
    1
}

/// the French third division (the National): discipline 0xc, only the league
/// Original: `FraThird::FraThird` (0x5bfb40, fra_third.cpp, thiscall, ret 8).
///
/// [pt] { 0x5bfb40 } (thiscall, ret 8, fra_third.cpp): `FraThird::FraThird(ano, registro)`: a terceira divisão francesa (o
/// National), disciplina 0xc, só a liga.
pub(crate) unsafe extern "thiscall" fn construct_fra_third(this: usize, year: u32, record: usize) -> usize {
    construct_division_league(this, year, record, &FRENCH_THIRD_DIVISION)
}

