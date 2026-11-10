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
pub(crate) type ItaSerALeagueRound = (i8, i8, i8, i8, i8, &'static [(i8, i8, i8)]);

/// Writes rounds `primeira`.. into round table `t`, in the original routine's order.
///
/// For each round: the date (`0x68a160`: push 0; push year; push code; push week; push years; push month; push day;
/// push i; push t; add esp -> cdecl: value 0) and the slots (`0x68a1c0`: push 0; push c; push b; push a; push j; push
/// i; push t; add esp -> cdecl, value 0).
///
/// [pt] As rodadas `primeira`.. da tabela `t`, na ordem da original: para cada uma, a data (0x68a160: push 0; push ano;
/// push código; push semana; push anos; push mês; push dia; push i; push t; add esp -> cdecl: o valor 0) e as
/// faixas (0x68a1c0: push 0; push c; push b; push a; push j; push i; push t; add esp -> cdecl, valor 0).
pub(crate) unsafe fn ita_ser_a_set_league_rounds(t: usize, year: u32, first: i32, table: &[ItaSerALeagueRound]) {
    for (k, &(dia, mes, anos, semana, codigo, faixas)) in table.iter().enumerate() {
        let i = first + k as i32;
        exe::league_round_set_date(t, i, dia as i32, mes as i32, anos as i32, semana as i32, codigo as i32, year, 0);
        for (j, &(a, b, c)) in faixas.iter().enumerate() {
            exe::league_round_set_match_slot(t, i, j as i32, a as i32, b as i32, c as i32, 0);
        }
    }
}

/// O que muda entre as ligas de fase única.
pub(crate) struct ItaSerASingleStageLeague {
    pub(crate) rounds: i16,
    pub(crate) file: usize,
    pub(crate) row: u32,
    pub(crate) table: &'static [ItaSerALeagueRound],
    pub(crate) branch: &'static str,
}

/// The common create_schedule body (thiscall(league, stage, &rounds, &extra, &é_liga), ret 0x10; no saved state): only
/// the main stage (−1; the others return 0 without writing anything).
///
/// é_liga (if requested) = 1, the rounds, extra = 0, the table of rounds × 0x41 bytes (malloc `exe::crt_malloc`: push; add esp,
/// 4 -> cdecl; out of memory: the box and 0) with the league's dates and slots. Returns the table.
///
/// [pt] O corpo comum (thiscall(liga, fase, &rodadas, &extra, &é_liga), ret 0x10; sem estado salvo): só a fase principal
/// (−1; as outras devolvem 0 sem gravar nada): é_liga (se pedido) = 1, as rodadas, extra = 0, a tabela de rodadas × 0x41
/// bytes (malloc 0x944e46: push; add esp, 4 -> cdecl; sem memória: a caixa e 0) com as datas e as faixas da liga.
/// Devolve a tabela.
pub(crate) unsafe fn build_league_schedule(this: League, stage: u32, rodadas: *mut i16, extra: *mut i16, league: *mut i32, p: &ItaSerASingleStageLeague) -> usize {
    if stage as u8 != 0xff {
        return 0;
    }
    if !league.is_null() {
        *league = 1;
    }
    *rodadas = p.rounds;
    *extra = 0;
    let t = exe::crt_malloc(p.rounds as u32 * 0x41);
    if t == 0 {
        show_error_at(p.file, p.row);
        return 0;
    }
    crate::comparison::branch(p.branch);
    let ano = (this.year() as u16) as u32;
    ita_ser_a_set_league_rounds(t, ano, 0, p.table);
    t
}

/// a Série A, 34 rodadas de 28/ago a 12/mai; quase todas com duas faixas de TV (dias 6 e 5, turno 2, 2 jogos cada)
pub(crate) const SERIE_A_ROUNDS: [ItaSerALeagueRound; 34] = [
    (28, 7, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 0: 28/ago
    (9, 8, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 1: 9/set
    (16, 8, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 2: 16/set
    (23, 8, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 3: 23/set
    (30, 8, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 4: 30/set
    (10, 9, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 5: 10/out
    (14, 9, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 6: 14/out
    (21, 9, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 7: 21/out
    (28, 9, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 8: 28/out
    (4, 10, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 9: 4/nov
    (18, 10, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 10: 18/nov
    (25, 10, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 11: 25/nov
    (2, 11, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 12: 2/dez
    (9, 11, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 13: 9/dez
    (16, 11, 0, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 14: 16/dez
    (6, 0, 1, 2, 2, &[(-1, -1, -1)]), // 15: 6/jan (+1 ano)
    (13, 0, 1, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 16: 13/jan (+1 ano)
    (20, 0, 1, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 17: 20/jan (+1 ano)
    (27, 0, 1, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 18: 27/jan (+1 ano)
    (3, 1, 1, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 19: 3/fev (+1 ano)
    (10, 1, 1, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 20: 10/fev (+1 ano)
    (17, 1, 1, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 21: 17/fev (+1 ano)
    (24, 1, 1, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 22: 24/fev (+1 ano)
    (3, 2, 1, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 23: 3/mar (+1 ano)
    (10, 2, 1, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 24: 10/mar (+1 ano)
    (17, 2, 1, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 25: 17/mar (+1 ano)
    (24, 2, 1, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 26: 24/mar (+1 ano)
    (30, 2, 1, 5, 1, &[(-1, -1, -1)]), // 27: 30/mar (+1 ano)
    (7, 3, 1, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 28: 7/abr (+1 ano)
    (14, 3, 1, 6, 1, &[(6, 2, 2), (5, 2, 2), (-1, -1, -1)]), // 29: 14/abr (+1 ano)
    (21, 3, 1, 6, 1, &[(-1, -1, -1)]), // 30: 21/abr (+1 ano)
    (28, 3, 1, 6, 1, &[(-1, -1, -1)]), // 31: 28/abr (+1 ano)
    (5, 4, 1, 6, 1, &[(-1, -1, -1)]), // 32: 5/mai (+1 ano)
    (12, 4, 1, 6, 1, &[(-1, -1, -1)]), // 33: 12/mai (+1 ano)
];

/// a Série B, 38 rodadas de 29/ago a 14/mai; quase todas com duas faixas de TV (dias 4 e 0, turno 2, 2 jogos cada)
pub(crate) const SERIE_B_ROUNDS: [ItaSerALeagueRound; 38] = [
    (29, 7, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 0: 29/ago
    (5, 8, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 1: 5/set
    (12, 8, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 2: 12/set
    (19, 8, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 3: 19/set
    (26, 8, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 4: 26/set
    (3, 9, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 5: 3/out
    (10, 9, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 6: 10/out
    (17, 9, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 7: 17/out
    (24, 9, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 8: 24/out
    (31, 9, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 9: 31/out
    (7, 10, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 10: 7/nov
    (14, 10, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 11: 14/nov
    (21, 10, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 12: 21/nov
    (28, 10, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 13: 28/nov
    (5, 11, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 14: 5/dez
    (12, 11, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 15: 12/dez
    (19, 11, 0, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 16: 19/dez
    (6, 0, 1, 3, 2, &[(-1, -1, -1)]), // 17: 6/jan (+1 ano)
    (9, 0, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 18: 9/jan (+1 ano)
    (16, 0, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 19: 16/jan (+1 ano)
    (19, 0, 1, 6, 1, &[(-1, -1, -1)]), // 20: 19/jan (+1 ano)
    (23, 0, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 21: 23/jan (+1 ano)
    (30, 0, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 22: 30/jan (+1 ano)
    (6, 1, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 23: 6/fev (+1 ano)
    (13, 1, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 24: 13/fev (+1 ano)
    (20, 1, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 25: 20/fev (+1 ano)
    (27, 1, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 26: 27/fev (+1 ano)
    (5, 2, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 27: 5/mar (+1 ano)
    (12, 2, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 28: 12/mar (+1 ano)
    (19, 2, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 29: 19/mar (+1 ano)
    (26, 2, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 30: 26/mar (+1 ano)
    (2, 3, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 31: 2/abr (+1 ano)
    (9, 3, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 32: 9/abr (+1 ano)
    (16, 3, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 33: 16/abr (+1 ano)
    (23, 3, 1, 6, 1, &[(4, 2, 2), (0, 2, 2), (-1, -1, -1)]), // 34: 23/abr (+1 ano)
    (30, 3, 1, 6, 1, &[(-1, -1, -1)]), // 35: 30/abr (+1 ano)
    (7, 4, 1, 6, 1, &[(-1, -1, -1)]), // 36: 7/mai (+1 ano)
    (14, 4, 1, 6, 1, &[(-1, -1, -1)]), // 37: 14/mai (+1 ano)
];

/// The French first division: the rounds in order, each marked `true` if it is only used with 38 rounds (20 clubs; with
/// 34, the `false` ones, in sequence). The index in the comment is the one of the 38-round season.
///
/// [pt] A primeira divisão francesa: as rodadas em ordem, cada uma marcada `true` se só entra com 38 rodadas (20 clubes; com
/// 34, as de `false`, em sequência). O índice no comentário é o da temporada de 38.
pub(crate) const FRENCH_FIRST_DIVISION_ROUNDS: [(bool, ItaSerALeagueRound); 38] = [
    (false, (31, 6, 0, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)])), // 0: 31/jul
    (false, (6, 7, 0, 4, 2, &[(-1, -1, -1)])), // 1: 6/ago
    (false, (14, 7, 0, 5, 2, &[(4, 1, 1), (4, 2, 1), (-1, -1, -1)])), // 2: 14/ago
    (false, (20, 7, 0, 4, 2, &[(-1, -1, -1)])), // 3: 20/ago
    (false, (28, 7, 0, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)])), // 4: 28/ago
    (false, (10, 8, 0, 4, 2, &[(-1, -1, -1)])), // 5: 10/set
    (false, (18, 8, 0, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)])), // 6: 18/set
    (false, (25, 8, 0, 4, 2, &[(-1, -1, -1)])), // 7: 25/set
    (false, (2, 9, 0, 5, 2, &[(4, 1, 1), (4, 2, 1), (-1, -1, -1)])), // 8: 2/out
    (false, (12, 9, 0, 1, 2, &[(-1, -1, -1)])), // 9: 12/out
    (false, (15, 9, 0, 4, 2, &[(-1, -1, -1)])), // 10: 15/out
    (false, (23, 9, 0, 5, 2, &[(4, 1, 1), (4, 2, 1), (-1, -1, -1)])), // 11: 23/out
    (false, (30, 9, 0, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)])), // 12: 30/out
    (false, (6, 10, 0, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)])), // 13: 6/nov
    (false, (10, 10, 0, 2, 2, &[(1, 2, 1), (1, 2, 1), (-1, -1, -1)])), // 14: 10/nov
    (false, (19, 10, 0, 4, 2, &[(-1, -1, -1)])), // 15: 19/nov
    (false, (27, 10, 0, 5, 2, &[(4, 1, 1), (4, 2, 1), (-1, -1, -1)])), // 16: 27/nov
    (false, (3, 11, 0, 4, 2, &[(-1, -1, -1)])), // 17: 3/dez
    (false, (11, 11, 0, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)])), // 18: 11/dez
    (false, (15, 11, 0, 2, 2, &[(1, 2, 1), (1, 2, 1), (-1, -1, -1)])), // 19: 15/dez
    (false, (18, 11, 0, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)])), // 20: 18/dez
    (true, (8, 0, 1, 5, 2, &[(4, 2, 1), (4, 2, 1), (-1, -1, -1)])), // 21: 8/jan (+1 ano)
    (false, (15, 0, 1, 5, 2, &[(4, 2, 1), (4, 2, 1), (-1, -1, -1)])), // 22: 15/jan (+1 ano)
    (true, (22, 0, 1, 5, 2, &[(4, 2, 1), (4, 2, 1), (-1, -1, -1)])), // 23: 22/jan (+1 ano)
    (false, (26, 0, 1, 2, 2, &[(1, 2, 1), (1, 2, 1), (-1, -1, -1)])), // 24: 26/jan (+1 ano)
    (false, (2, 1, 1, 2, 2, &[(1, 2, 1), (1, 2, 1), (-1, -1, -1)])), // 25: 2/fev (+1 ano)
    (false, (5, 1, 1, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)])), // 26: 5/fev (+1 ano)
    (false, (16, 1, 1, 2, 2, &[(1, 2, 1), (1, 2, 1), (-1, -1, -1)])), // 27: 16/fev (+1 ano)
    (false, (27, 1, 1, 4, 2, &[(-1, -1, -1)])), // 28: 27/fev (+1 ano)
    (true, (4, 2, 1, 5, 2, &[(4, 2, 1), (4, 2, 1), (-1, -1, -1)])), // 29: 4/mar (+1 ano)
    (false, (11, 2, 1, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)])), // 30: 11/mar (+1 ano)
    (true, (18, 2, 1, 5, 2, &[(4, 2, 1), (4, 2, 1), (-1, -1, -1)])), // 31: 18/mar (+1 ano)
    (false, (25, 2, 1, 5, 2, &[(4, 1, 1), (4, 2, 1), (-1, -1, -1)])), // 32: 25/mar (+1 ano)
    (false, (8, 3, 1, 5, 2, &[(4, 1, 1), (4, 2, 1), (-1, -1, -1)])), // 33: 8/abr (+1 ano)
    (false, (15, 3, 1, 5, 2, &[(6, 1, 1), (6, 2, 1), (-1, -1, -1)])), // 34: 15/abr (+1 ano)
    (false, (29, 3, 1, 5, 2, &[(6, 2, 1), (6, 2, 1), (-1, -1, -1)])), // 35: 29/abr (+1 ano)
    (false, (13, 4, 1, 5, 2, &[(-1, -1, -1)])), // 36: 13/mai (+1 ano)
    (false, (20, 4, 1, 5, 2, &[(-1, -1, -1)])), // 37: 20/mai (+1 ano)
];

pub(crate) const SERIE_A_LEAGUE: ItaSerASingleStageLeague = ItaSerASingleStageLeague { rounds: 0x22, file: 0x9ea2a8, row: 0xb0, table: &SERIE_A_ROUNDS, branch: "cqo_6 calendario da serie a" };
pub(crate) const SERIE_B_LEAGUE: ItaSerASingleStageLeague = ItaSerASingleStageLeague { rounds: 0x26, file: 0x9ea384, row: 0xa8, table: &SERIE_B_ROUNDS, branch: "cqo_6 calendario da serie b" };

/// The play-off match of the Italian leagues (stage 0, one 0x68-byte cup round, `eng_cc_cup_write_cup_round`; the year
/// +0x40 signed).
///
/// The draw and the match in May of the following year (Serie A: draw on 13 May, match on 19 May; Serie B: 15 May and
/// 21 May), name 0xa0, no draw (0x200), tie-break 3, category 5, two legs (+0x21 = 2) with 3 days between the matches,
/// no prizes; the teams (2n teams, n matches, 2n new entrants) from the league's byte +0xee (n, signed; arithmetic in
/// 16 bits as in the original) [inference: the requested play-off ties].
///
/// [pt] O jogo-desempate das ligas italianas (fase 0, uma rodada de copa de 0x68 bytes, `eng_cc_cup_write_cup_round`; o ano +0x40 com
/// sinal): o sorteio e o jogo em maio do ano seguinte (a Série A: sorteio em 13/mai, jogo em 19/mai; a Série B: 15/mai e
/// 21/mai), nome 0xa0, sem sorteio (0x200), desempate do confronto 3, categoria 5, ida e volta (+0x21 = 2) com 3 dias
/// entre os jogos, sem prêmios; os times (2n, n jogos, 2n novos) pelo byte +0xee da liga (n, com sinal; as contas em 16
/// bits como a original) [inferência: os confrontos de desempate pedidos].
pub(crate) fn italian_playoff_round(sorteio: (u32, u32, u32, u32), game: (u32, u32, u32, u32, u32, u32), n: i8) -> EngCcCupCupRound {
    let n = n as i16;
    let two = n.wrapping_shl(1) as u16;
    cql10r(sorteio, game, 0xa0, (0, 0x200, 3), 5, (two, n as u16, two, 0), (0, 2, 3), (0, 0, 0))
}

/// O que muda entre a Série A e a Série B.
pub(crate) struct ItalianLeagueWithTiebreak {
    pub(crate) league: &'static ItaSerASingleStageLeague,
    pub(crate) draw: (u32, u32, u32, u32),
    pub(crate) game: (u32, u32, u32, u32, u32, u32),
    pub(crate) row_tiebreak: u32,
}

/// The shared create_schedule body of the two Italian leagues (thiscall(league, stage, &rounds, &extra, &é_liga), ret
/// 0x10; no saved state).
///
/// Stage −1 is the main one (`build_league_schedule`); stage 0, the play-off match: é_liga (if requested) = 0, 1 round,
/// extra 0, the table of 0x68 bytes (malloc `exe::crt_malloc`; out of memory: the box and 0); the others return 0 without
/// writing anything. Returns the table.
///
/// [pt] O corpo comum das duas (thiscall(liga, fase, &rodadas, &extra, &é_liga), ret 0x10; sem estado salvo): a fase −1 é a
/// principal (`build_league_schedule`); a 0, o jogo-desempate: é_liga (se pedido) = 0, 1 rodada, extra 0, a tabela de 0x68
/// bytes (malloc 0x944e46; sem memória: a caixa e 0); as outras devolvem 0 sem gravar nada. Devolve a tabela.
pub(crate) unsafe fn italian_schedule(this: League, stage: u32, rounds: *mut i16, extra: *mut i16, liga: *mut i32, p: &ItalianLeagueWithTiebreak) -> usize {
    match stage as u8 {
        0xff => build_league_schedule(this, stage, rounds, extra, liga, p.league),
        0 => {
            if !liga.is_null() {
                *liga = 0;
            }
            *rounds = 1;
            *extra = 0;
            let t = exe::crt_malloc(0x68);
            if t == 0 {
                show_error_at(p.league.file, p.row_tiebreak);
                return 0;
            }
            crate::comparison::branch("cqo_6 italian tiebreak match");
            let ano = this.year() as i32 as u32;
            eng_cc_cup_write_cup_round(t, 0, &italian_playoff_round(p.draw, p.game, read_i8(this.addr() + 0xee)), ano);
            t
        }
        _ => 0,
    }
}

pub(crate) const SERIE_A_WITH_PLAYOFF: ItalianLeagueWithTiebreak = ItalianLeagueWithTiebreak { league: &SERIE_A_LEAGUE, draw: (0xd, 4, 1, 0), game: (0x13, 4, 1, 6, 1, 0), row_tiebreak: 0x226 };
pub(crate) const SERIE_B_WITH_TIEBREAK: ItalianLeagueWithTiebreak = ItalianLeagueWithTiebreak { league: &SERIE_B_LEAGUE, draw: (0xf, 4, 1, 0), game: (0x15, 4, 1, 6, 1, 0), row_tiebreak: 0x170 };

/// the Serie A schedule: 34 rounds (18 clubs), from 28 August to 12 May of the next year, all on the same weekday (6)
/// except two (6 January, 30 March); in nearly all, two matches go to each of two TV slots (days 6 and 5, slot 2)
/// [inference: the televised weekend evening matches]. Stage 0, the play-off match (spareggio) [inference: clubs level
/// on points for a place]: two legs 3 days apart, draw on 13 and match on 19 May, ties in fixed order, as many ties as
/// the byte +0xee
/// Original: `ItaSerA::create_schedule` (0x6409b0, ita_ser_a.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x6409b0 } (thiscall, ret 0x10, ita_ser_a.cpp, 2.901 bytes, método +0x3c): `ItaSerA::create_schedule(fase,
/// &rodadas, &extra, &é_liga)`: a Série A, 34 rodadas (`SERIE_A_ROUNDS`), e o jogo-desempate. Fonte 0x9ea2a8, linhas 0xb0
/// e 0x226.
pub(crate) unsafe extern "thiscall" fn ita_ser_a_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    italian_schedule(League(this), stage, rounds, extra, league, &SERIE_A_WITH_PLAYOFF)
}

/// Row i of the league table [+0xb1] (0x3b bytes each).
///
/// [pt] A linha `i` da tabela [+0xb1] (0x3b bytes).
pub(crate) unsafe fn ita_ser_a_table_row(league: League, i: usize) -> usize {
    league.table() + i * 0x3b
}

/// The Serie A zones by position (0 = first): 1 champion, 2 the other Champions League places, 3 UEFA Cup places, 4 the
/// next two [inference: Intertoto], 5 relegation (+0xc1), 0 the rest.
///
/// 2 covers the nation's `cl` places (UefaSeeding::get_nation_places `exe::uefaseeding_get_nation_places`: push &uefa; push &cl; push
/// [[liga+4]+0x5d] (the competition's nation); mov ecx, [0xde1f58] -> thiscall, ret 0xc, which writes 1 to both before
/// searching); 3 the `uefa` UEFA Cup places, right after; 4 the next two [inference: Intertoto]; 5 the last `+0xc1` of
/// 18 (relegation); 0 the rest.
///
/// The original routine builds the 18 zones in a local record (16 + 2 bytes) and reads two beyond it with a table of
/// more than 18 clubs; the copy uses 64 bytes with zeros after the 18 (Serie A has 18).
///
/// [pt] As faixas da Série A, por posição (0 = o primeiro): 1 o campeão; 2 as outras vagas da Liga dos Campeões (as `cl`
/// vagas da nação, UefaSeeding::get_nation_places 0x905c60: push &uefa; push &cl; push [[liga+4]+0x5d] (a nação da
/// competição); mov ecx, [0xde1f58] -> thiscall, ret 0xc, que grava 1 nas duas antes de procurar); 3 as `uefa` vagas da
/// Copa da UEFA, logo depois; 4 as duas seguintes [inferência: a Intertoto]; 5 as `+0xc1` últimas de 18 (o
/// rebaixamento); 0 o resto. A original monta as 18 faixas num registro local (16 + 2 bytes) e lê duas além com uma
/// tabela de mais de 18 clubes; a cópia usa 64 bytes com zero depois das 18 (a Série A tem 18).
pub(crate) unsafe fn serie_a_zones(league: League) -> [u8; 64] {
    let mut z = [0u8; 64];
    z[0] = 1;
    let (mut cl, mut uefa) = (0i8, 0i8);
    exe::uefaseeding_get_nation_places(read_usize(G_UEFA_SEEDING), league.competition().nation().addr(), &mut cl, &mut uefa);
    // simplification: the nation’s counts stay from 1 to 16 in the game; the copy ignores the bands that would fall outside the record.
    let cl = cl as i32;
    let uefa = uefa as i32;
    if cl > 1 {
        for k in 1..cl {
            if (k as usize) < 64 {
                z[k as usize] = 2;
            }
        }
    }
    if uefa > 0 {
        for k in 0..uefa {
            if ((cl + k) as usize) < 64 {
                z[(cl + k) as usize] = 3;
            }
        }
    }
    let i = uefa + cl;
    if i >= 0 && (i as usize) + 1 < 64 {
        z[i as usize] = 4; // cql4: the two positions after the European slots
        z[i as usize + 1] = 4;
    }
    let relegated = (league.relegated_count() as i8) as i32;
    if relegated > 0 {
        for k in 0..relegated {
            if 17 - k >= 0 {
                z[(17 - k) as usize] = 5;
            }
        }
    }
    z
}

/// Per-position zones of Serie B: 1 champion, 2 the 3 promotion places, 3 the last 4 of 20 (relegation), 0 the rest.
///
/// [pt] As faixas da Série B: 1 o campeão; 2 as 3 vagas seguintes de acesso; 3 as 4 últimas de 20 (o rebaixamento); 0 o
/// resto (registro local de 20 bytes na original).
pub(crate) fn serie_b_zones() -> [u8; 64] {
    let mut z = [0u8; 64];
    z[0] = 1;
    z[1] = 2;
    z[2] = 2;
    z[3] = 2; // cql4: the Serie B promotion slots
    for k in 16..20 {
        z[k] = 3;
    }
    z
}

/// True if two adjacent table rows have equal points (+0x19) but different zones.
///
/// [pt] Há duas linhas vizinhas com os mesmos pontos (+0x19) em faixas diferentes? (a conferência do comparador)
pub(crate) unsafe fn tie_on_zone_boundary(league: League, z: &[u8; 64]) -> bool {
    let n = league.club_count() as i32 - 1;
    (0..n.max(0) as usize).any(|i| {
        let l = ita_ser_a_table_row(league, i);
        read_i16(l + 0x19) == read_i16(l + 0x3b + 0x19) && z[i.min(63)] != z[(i + 1).min(63)]
    })
}

/// The Serie A day; at season end, two clubs level on points in different bands are not split by goal difference: the
/// first such pair from the top is ordered by head-to-head and goes to an extra match (the next stage); with no tie on
/// a boundary, the season closes.
///
/// Convention: thiscall, ret 8, `ita_ser_a.cpp`, slot `+0x14`. `ItaSerA::update_for_date(hoje, completo)` (today,
/// complete). First the league day (League::update_for_date `0x6847c0`: push complete; push today; mov ecx, this ->
/// thiscall, ret 8), whose return value it passes on.
///
/// Then, at the end of the season (`+0xdd`), in the main stage (`+0x2c` = 0, no stages created), with `completo` and
/// with `+0xe6` [inference: the table closed]: it clears `+0x4c` and looks, from top to bottom, for the first pair of
/// neighbouring clubs with the same points in different bands (serie_a_zones). If found: it orders the tied clubs by
/// head-to-head (`0x6471a0`: push points; mov ecx, this -> thiscall, ret 4), creates the array of 1 stage (`+0x2c` = 1,
/// `+0xc` = malloc `0x944e46` of 4: push 4; add esp, 4 -> cdecl) and calls virtual `+0x28` (the next stage: the
/// tie-break match). With no tie on a boundary: `+0xdd` = 0, `+0xe6` = 0 and virtual `+0x94`
/// (League::update_positions_and_status).
///
/// The rule: in Italy, a tie on points between clubs in different bands (the title; Italy's Champions League places;
/// the UEFA Cup places; the next two [inference: the Intertoto]; relegation, the last `+0xc1` of 18) is not decided by
/// goal difference: it goes to an extra match.
/// Original: `ItaSerA::update_for_date` (0x642880, ita_ser_a.cpp, thiscall, ret 8).
///
/// [pt] { 0x642880 } (thiscall, ret 8, ita_ser_a.cpp, slot +0x14): `ItaSerA::update_for_date(hoje, completo)`. Primeiro o
/// dia de liga (League::update_for_date 0x6847c0: push completo; push hoje; mov ecx, this -> thiscall, ret 8), cuja
/// devolução ela repassa. Depois, no fim da temporada (+0xdd), na fase principal (+0x2c = 0, sem fases criadas), com
/// `completo` e com +0xe6 [inferência: a tabela fechada]: zera +0x4c e procura, de cima para baixo, o primeiro par de
/// clubes vizinhos com os mesmos pontos em faixas diferentes (serie_a_zones). Achando: ordena os empatados
/// pelo confronto direto (0x6471a0: push pontos; mov ecx, this -> thiscall, ret 4), cria o vetor de 1 fase (+0x2c =
/// 1, +0xc = malloc 0x944e46 de 4: push 4; add esp, 4 -> cdecl) e chama a virtual +0x28 (a fase seguinte: o jogo de
/// desempate). Sem empate na fronteira: +0xdd = 0, +0xe6 = 0 e a virtual +0x94 (League::update_positions_and_status).
/// A regra: na Itália, empate em pontos entre clubes de faixas diferentes (título, Liga dos Campeões, Copa da UEFA,
/// Intertoto [inferência], rebaixamento) não se decide pelo saldo: vai a um jogo extra.
pub(crate) unsafe extern "thiscall" fn ita_ser_a_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let this = League(this);
    let r = (exe::league_update_for_date(this, today, full) as u32);
    // simplification: the original returns the league day byte with the upper bits of eax left over from the last call; the
    // the competitions loop only reads the byte.
    if this.complete() == 0 || this.stage_count() != 0 || full == 0 || this.tiebreak_matches() == 0 {
        return r;
    }
    let z = serie_a_zones(this);
    let n = this.club_count() as i32 - 1;
    this.set_season_closed(0);
    crate::comparison::branch("cql_4 serie a end");
    let mut i: i16 = 0;
    if n > 0 {
        loop {
            let l = ita_ser_a_table_row(this, i as usize);
            let points = read_i16(l + 0x19);
            if points == read_i16(l + 0x54) && z[(i as usize).min(63)] != z[(i as usize + 1).min(63)] { // cql4: the Serie A pair at the boundary
                exe::ita_league_sort_clubs_level_on_points(this, points as u16 as u32);
                this.set_stage_count(1);
                let v = exe::crt_malloc(4);
                this.set_stages(v);
                let proxima: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(this.addr()) + 0x28));
                proxima(this.addr());
                return r;
            }
            i = i.wrapping_add(1);
            if i as i32 >= n {
                break;
            }
        }
    }
    this.set_complete(0);
    this.set_tiebreak_matches(0);
    let status: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(this.addr()) + 0x94));
    status(this.addr());
    r
}

/// The comparer of the two routines: the league day and the play-off stage cannot run twice, so only the original
/// routine runs and its guarantees are checked.
///
/// In the main stage and with `completo`: if the play-off stage was created (`+0x2c` changed), there is a pair of
/// neighbouring rows in the table tied across different zones; and the original routine never leaves the season end
/// pending (`+0xdd` and `+0xe6` set and `+0x2c` = 0), because it either creates the stage or zeroes them.
///
/// [pt] O comparador das duas: o dia de liga e a fase do desempate não rodam duas vezes: só a original, conferindo o que
/// ela garante. Na fase principal e com `completo`: se a fase do desempate foi criada (+0x2c mudou), há na tabela um
/// par vizinho empatado em faixas diferentes; e a original nunca sai com o fim da temporada pendente (+0xdd e +0xe6
/// ligados e +0x2c = 0), porque ou cria a fase ou os zera.
pub(crate) unsafe fn compare_league_day(name: &str, this: League, full: u32, faixas: impl Fn() -> [u8; 64], original: impl FnOnce() -> u32) -> u32 {
    let stages = this.stage_count();
    let r = original();
    if full != 0 && stages == 0 {
        if this.stage_count() != 0 {
            let z = faixas();
            SCOREBOARD.record(tie_on_zone_boundary(this, &z) && this.stages() != 0, || format!("{}({:#x}): stage of the tiebreak without draw in the boundary (+0xc = {:#x})", name, this.addr(), this.stages()));
        } else {
            SCOREBOARD.record(!(this.complete() != 0 && this.tiebreak_matches() != 0), || format!("{}({:#x}): o end of the season ficou pendente (+0xdd e +0xe6 ligados)", name, this.addr()));
        }
    }
    r
}

/// Updates the staff records by status in Serie A: in the league, the base records; in the tie-break match, the winner
/// (status 1 [inference: the club won]) is placed above the other in the table (the rows swap if it was below) and,
/// with no play-off pending, the season closes. Returns −1 in the tie-break case.
///
/// Convention: thiscall, ret `0x18`, `ita_ser_a.cpp`, slot `+0x44`. `ItaSerA::update_staff_records_for_status(clube,
/// situação, fase, a4, jogo, a6)` (club, status, stage, a4, match, a6). In the league (stage `0xff`): the base version
/// (League::update_staff_records_for_status `0x686940`: push a6; push match; push a4; push -1; push status; push club;
/// mov ecx, this -> thiscall, ret `0x18`), whose return value it passes on.
///
/// In the tie-break match (any other stage), only with status 1: `+0xad` = 0, sorts the table (League::sort_table
/// `0x6827d0`: push 0; mov ecx -> thiscall, ret 4) and finds the club's row: if the club above is one of the two in the
/// match (`+0x1c`, `+0x20`), the two rows swap places; otherwise, if the club below is one of them, they stay. In both
/// cases the two rows get their position (`+4`) and an open destination (`+0x37` = `0xff`). Then, if no row has
/// destination 1 (the play-off above), it closes the season: `+0xe6` = 0, `+0xdd` = 0, virtual `+0x94`, virtual `+0x10`
/// (0, 0) and `+0xdd` = 1.
///
/// The rule: the winner of the tie-break match finishes ahead of the other.
/// Original: `ItaSerA::update_staff_records_for_status` (0x641dc0, ita_ser_a.cpp, thiscall, ret 0x18).
///
/// [pt] { 0x641dc0 } (thiscall, ret 0x18, ita_ser_a.cpp, slot +0x44): `ItaSerA::update_staff_records_for_status(clube,
/// situação, fase, a4, jogo, a6)`. Na liga (fase 0xff): a da base (League::update_staff_records_for_status 0x686940:
/// push a6; push jogo; push a4; push -1; push situação; push clube; mov ecx, this -> thiscall, ret 0x18), cuja devolução
/// repassa. No jogo de desempate (outra fase), só com a situação 1 [inferência: o clube venceu]: +0xad = 0, ordena a
/// tabela (League::sort_table 0x6827d0: push 0; mov ecx -> thiscall, ret 4) e acha a linha do clube: se o de cima é
/// um dos dois do jogo (+0x1c, +0x20), as duas linhas trocam de lugar; senão, se o de baixo é um deles, ficam. Nos dois
/// casos as duas linhas ganham a posição (+4) e o destino em aberto (+0x37 = 0xff). Depois, se nenhuma linha tem o
/// destino 1 (o play-off de cima), fecha a temporada: +0xe6 = 0, +0xdd = 0, a virtual +0x94, a virtual +0x10 (0, 0) e
/// +0xdd = 1. Devolve −1. A regra: quem vence o jogo de desempate fica à frente do outro.
pub(crate) unsafe extern "thiscall" fn ita_ser_a_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, a4: u32, game: usize, a6: u32) -> u32 {
    let this = League(this);
    if stage as u8 == 0xff {
        return exe::league_update_staff_records_for_status(this, club, status, 0xffff_ffff, a4, game, a6);
    }
    if status as u8 != 1 {
        return 0xffff_ffff;
    }
    crate::comparison::branch("cql_4 tiebreak result");
    this.set_table_column(0);
    exe::league_sort_table(this, (0) as u32 as usize);
    let t = this.table();
    let (a, b) = (read_usize(game + 0x1c), read_usize(game + 0x20));
    let mut i: i16 = 0;
    while i < this.club_count() {
        let l = t + i as usize * 0x3b;
        if read_usize(l) == club {
            if i > 0 {
                let above = read_usize(l - 0x3b);
                if above == a || above == b {
                    // the club was below the other: they swap
                    let mut tmp = [0u8; 0x3b];
                    core::ptr::copy_nonoverlapping(l as *const u8, tmp.as_mut_ptr(), 0x3b);
                    core::ptr::copy_nonoverlapping((l - 0x3b) as *const u8, l as *mut u8, 0x3b);
                    let l2 = this.table() + i as usize * 0x3b;
                    core::ptr::copy_nonoverlapping(tmp.as_ptr(), (l2 - 0x3b) as *mut u8, 0x3b);
                    let l3 = this.table() + i as usize * 0x3b;
                    write_u8(l3 + 4, i as u8);
                    write_u8(l3 - 0x3b + 4, (i as u8).wrapping_sub(1));
                    write_u8(l3 + 0x37, 0xff);
                    write_u8(l3 - 0x3b + 0x37, 0xff);
                    break;
                }
            }
            if i < this.club_count() {
                // simplification: the original reads the club of the next row even on the last one (4 bytes past the table), as here.
                let below = read_usize(l + 0x3b);
                if below == a || below == b {
                    write_u8(t + i as usize * 0x3b + 4, i as u8);
                    let t2 = this.table();
                    write_u8(t2 + i as usize * 0x3b + 0x3f, (i as u8).wrapping_add(1));
                    write_u8(this.table() + i as usize * 0x3b + 0x37, 0xff);
                    write_u8(this.table() + i as usize * 0x3b + 0x72, 0xff);
                    break;
                }
            }
        }
        i += 1;
    }
    let n = this.club_count();
    let mut k: i16 = 0;
    while k < n {
        if read_u8(ita_ser_a_table_row(this, k as usize) + 0x37) == 1 {
            return 0xffff_ffff;
        }
        k += 1;
    }
    this.set_tiebreak_matches(0);
    this.set_complete(0);
    let status_f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(this.addr()) + 0x94));
    status_f(this.addr());
    let end: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(read_usize(read_usize(this.addr()) + 0x10));
    end(this.addr(), 0, 0);
    this.set_complete(1);
    0xffff_ffff
}

/// What changes from one Italian division to another in the constructor.
///
/// [pt] O que muda de uma divisão italiana para outra no construtor.
pub(crate) struct ItalianDivision {
    pub(crate) vtabela: usize,
    /// a Série A e a Série B gravam +0xee = 0 (o campo próprio da classe [inferência: o desempate em curso]) e +0x4c = 1
    pub(crate) series: bool,
    /// fases além da liga (+0x2c): 0 na A e na B; 2 na C1 (o play-off e o play-out; o vetor [+0xc] = malloc de 8)
    pub(crate) stages: u32,
    pub(crate) rules: usize,
    pub(crate) table: usize,
    /// The real 2001/02 calendar by club names (set_up_fixed_fixtures; mov ecx, this -> thiscall, ret 0).
    ///
    /// [pt] o calendário real de 2001/02 pelo nome dos clubes (set_up_fixed_fixtures; mov ecx, this -> thiscall, ret 0)
    pub(crate) fixed: usize,
    pub(crate) file: usize,
    /// as caixas: regras, tabela, jogos, ordem, estatísticas
    pub(crate) rows: [u32; 5],
}

/// The body of the four Italian division constructors (thiscall(this, year, record), ret 8, SEH frame only for the
/// operator new); returns this.
///
/// The base (League::League `exe::new_league`: mov ecx -> thiscall, ret 0), the record (+4), the vtable, the year (+0x40) and
/// discipline rule 0x11 (+0x50, the Italian leagues' one); it tries to load the state (League::load_from_temp_file
/// `exe::league_load_from_temp_file`: push 1; mov ecx -> thiscall, ret 4). If it loaded: done.
///
/// Otherwise, the new league: in A and B, +0xee = 0 and +0x4c = 1; stage 0xff (+0x44), current stage −1 (+0x30), +0x2c
/// stages and the array [+0xc] (malloc `exe::crt_malloc` of 4 × stages: push; add esp, 4 -> cdecl; 0 with no stages); the rules
/// and the table (error box and stop if they fail); the real 2001/02 calendar (unchecked); the fixtures
/// (League::create_fixtures `create_fixtures`), the sort (League::sort_table `sort_table`: push 0 -> thiscall, ret 4), the stats
/// (operator new `exe::crt_operator_new` of 0x5ce: push; add esp, 4 -> cdecl; Comp_Stats::Comp_Stats `exe::new_comp_stats`: push league; mov ecx,
/// new -> thiscall, ret 4) at +8 and the predicted positions (`league_set_predicted_positions`: mov ecx -> thiscall, ret 0).
///
/// [pt] O corpo dos quatro construtores (thiscall(this, ano, registro), ret 8, quadro SEH só para o operator new): a base
/// (League::League 0x682200: mov ecx -> thiscall, ret 0), o registro (+4), a vtable, o ano (+0x40) e a regra de
/// disciplina 0x11 (+0x50, a das ligas italianas); tenta carregar o estado (League::load_from_temp_file 0x687b10: push
/// 1; mov ecx -> thiscall, ret 4). Carregou: pronto. Senão, a liga nova: na A e na B, +0xee = 0 e +0x4c = 1; fase 0xff
/// (+0x44), fase atual −1 (+0x30), +0x2c fases e o vetor [+0xc] (malloc 0x944e46 de 4 × fases: push; add esp, 4 ->
/// cdecl; 0 sem fases); as regras e a tabela (caixa e fim se falharem); o calendário real de 2001/02 (sem conferir);
/// os jogos (League::create_fixtures 0x6835c0), a ordem (League::sort_table 0x6827d0: push 0 -> thiscall, ret 4), as
/// estatísticas (operator new 0x944cf1 de 0x5ce: push; add esp, 4 -> cdecl; Comp_Stats::Comp_Stats 0x49ee70: push
/// liga; mov ecx, novo -> thiscall, ret 4) em +8 e as posições previstas (0x68a850: mov ecx -> thiscall, ret 0).
/// Devolve this.
pub(crate) unsafe fn construct_italian_division(this: League, year: u32, registro: usize, p: &ItalianDivision) -> usize {
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(p.rules);
    let tabela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(p.table);
    let fixos: unsafe extern "thiscall" fn(usize) = core::mem::transmute(p.fixed);
    exe::new_league(this);
    write_usize(this.addr() + league::COMPETITION, registro);
    write_usize(this.addr(), p.vtabela);
    this.set_year((year as u16) as i16);
    this.set_discipline_rule(0x11); // cqm4: the discipline rule of the Italian leagues
    if exe::league_load_from_temp_file(this, 1) != 0 {
        return this.addr();
    }
    crate::comparison::branch("cqm_4 new italian division");
    if p.series {
        write_u8(this.addr() + 0xee, 0);
        this.set_season_closed(1); // cqm4: the history mark of A and B
    }
    this.set_stage_index(0xff);
    this.set_current_stage(0xffff_ffff);
    this.set_stage_count(p.stages as usize);
    if p.stages != 0 {
        let v = exe::crt_malloc(p.stages * 4);
        this.set_stages(v);
    } else {
        this.set_stages(0);
    }
    if regras(this.addr()) == 0 {
        show_error_at(p.file, p.rows[0]);
        return this.addr();
    }
    if tabela(this.addr()) == 0 {
        show_error_at(p.file, p.rows[1]);
        return this.addr();
    }
    fixos(this.addr()); // cqm4: the real 2001/02 calendar
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(p.file, p.rows[2]);
        return this.addr();
    }
    if exe::league_sort_table(this, (0) as u32 as usize) == 0 {
        show_error_at(p.file, p.rows[3]);
        return this.addr();
    }
    let e = exe::crt_operator_new(0x5ce);
    let s = if e != 0 { exe::new_comp_stats(e, this) } else { 0 };
    this.set_comp_stats(s);
    if s == 0 {
        show_error_at(p.file, p.rows[4]);
        return this.addr();
    }
    exe::league_set_predicted_positions(this); // cqm4: the expected positions
    this.addr()
}

pub(crate) const SERIE_A: ItalianDivision = ItalianDivision {
    vtabela: 0x96c3cc,
    series: true,
    stages: 0,
    rules: 0x641640,
    table: 0x641510,
    fixed: 0x642a70,
    file: 0x9ea2a8,
    rows: [0x53, 0x5a, 0x64, 0x6d, 0x74],
};
pub(crate) const SERIE_B_DIVISION: ItalianDivision = ItalianDivision {
    vtabela: 0x96c480,
    series: true,
    stages: 0,
    rules: 0x6468a0,
    table: 0x646780,
    fixed: 0x647230,
    file: 0x9ea384,
    rows: [0x4b, 0x52, 0x5c, 0x65, 0x6c],
};
pub(crate) const SERIE_C1A_DIVISION: ItalianDivision = ItalianDivision {
    vtabela: 0x96c534,
    series: false,
    stages: 2, // cqm4: the C1/A play-off and play-out
    rules: 0x64b770,
    table: 0x64b670,
    fixed: 0x64be90,
    file: 0x9ea474,
    rows: [0x49, 0x50, 0x5a, 0x63, 0x6a],
};
pub(crate) const SERIE_C1B_DIVISION: ItalianDivision = ItalianDivision {
    vtabela: 0x96c5e8,
    series: false,
    stages: 2,
    rules: 0x64f920,
    table: 0x64f820,
    fixed: 0x650100,
    file: 0x9ea558,
    rows: [0x4a, 0x51, 0x5b, 0x64, 0x6b],
};

/// Serie A: discipline rule 0x11 (the Italian leagues one), only the league (the tie-break is born at season end); with
/// no saved state, the rules, the table, the real 2001/02 fixture list by club name, the fixtures, the order, the stats
/// and the predicted positions
/// Original: `ItaSerA::ItaSerA` (0x6406d0, ita_ser_a.cpp, thiscall, ret 8).
///
/// [pt] { 0x6406d0 } (thiscall, ret 8, ita_ser_a.cpp): `ItaSerA::ItaSerA(ano, registro)`: a Série A, só a liga (o jogo de
/// desempate nasce no fim, pela 0x642880).
pub(crate) unsafe extern "thiscall" fn construct_ita_ser_a(this: usize, year: u32, record: usize) -> usize {
    construct_italian_division(League(this), year, record, &SERIE_A)
}

/// Comparer of the four constructors: runs only the original and checks return value, vtable, year, record, discipline
/// and, for a new league, stages and Serie A/B fields.
///
/// [pt] O comparador dos quatro: alocam: só a original, conferindo o que ela grava: a devolução, a vtable, o ano, o
/// registro e a disciplina; na liga nova (+0x44 = 0xff, com as estatísticas), +0x2c, o vetor das fases e, na A e na
/// B, +0x4c = 1 e +0xee = 0.
pub(crate) unsafe fn compare_italian_constructor(name: &str, this: League, year: u32, record: usize, p: &ItalianDivision, original: impl FnOnce() -> usize) -> usize {
    let r = original();
    let mut ok = r == this.addr() && read_usize(this.addr()) == p.vtabela && (this.year() as u16) == year as u16
        && this.competition().addr() == record && this.discipline_rule() == 0x11;
    if this.stage_index() == 0xff && this.comp_stats() != 0 {
        ok = ok && this.stage_count() == p.stages as usize && (this.stages() != 0) == (p.stages != 0);
        if p.series {
            ok = ok && *((this.addr() + 0xee) as *const u8) == 0;
        }
    }
    SCOREBOARD.record(ok, || format!("{}({:#x}): returned {:#x}, vtable {:#x}, +0x2c = {}, +0xc = {:#x}, +8 = {:#x}", name, this.addr(), r, read_usize(this.addr()), this.stage_count(), this.stages(), this.comp_stats()));
    r
}

/// Number of league matches: (clubs +0x3e / 2) x rounds +0xa9.
///
/// [pt] Quantos jogos a liga tem: (clubes +0x3e / 2, com sinal, para zero) × rodadas +0xa9.
pub(crate) unsafe fn league_match_count(league: League) -> i32 {
    let clubs = league.club_count() as i32;
    let rounds = league.round_count() as i32;
    (clubs / 2).wrapping_mul(rounds)
}

/// The body of set_up_fixed_fixtures for both leagues (thiscall, ret 0, SEH frame only for the vector constructor):
/// allocates the fixed-fixture vector (+0xea), fills each fixture from the table by club names, and destroys it (+0xea
/// = 0) if any club was not found.
///
/// The vector of fixed fixtures (operator new `exe::crt_operator_new` of n × 16 + 4: push; add esp, 4 -> cdecl; the count n in the
/// first dword and each fixture through constructor 0x645800, with destructor 0x5a53c0 for the exception: `eh vector
/// constructor iterator` `exe::crt_vector_constructor_iterator`, push 0x5a53c0; push 0x645800; push n; push 0x10; push vector -> stdcall, ret 0x14)
/// in +0xea; out of memory, the box for line `linha` of source file `arquivo` and end.
///
/// Then each fixture of `tabela`, in order, in its place in the vector (fixed_fixture_set_by_club_names `exe::fixed_fixture_set_by_club_names`: push
/// the 6th (away, via the map); push the 5th (home); push year; push month (0 = January); push day; push league; mov
/// ecx, [+0xea] + 16 × k -> thiscall, ret 0x18: the date and the clubs found by name, −1 in +8/+0xc if not found).
///
/// Finally, if any of the n fixtures was left with a club not found (+8 or +0xc = −1), the vector is destroyed (`eh
/// vector destructor iterator` `exe::crt_vector_destructor_iterator`: push 0x5a53c0; push count; push 0x10; push vector -> stdcall, ret 0x10;
/// operator delete `exe::crt_operator_delete` of the block: push; add esp, 4 -> cdecl) and +0xea = 0: the league goes back to the
/// randomly drawn calendar.
///
/// [pt] O corpo das duas (set_up_fixed_fixtures; thiscall, ret 0, quadro SEH só para o construtor de vetor): o vetor dos
/// jogos fixos (operator new 0x944cf1 de n × 16 + 4: push; add esp, 4 -> cdecl; a conta n no primeiro dword e cada jogo
/// pelo construtor 0x645800, com o destrutor 0x5a53c0 para a exceção: `eh vector constructor iterator` 0x9463af, push
/// 0x5a53c0; push 0x645800; push n; push 0x10; push vetor -> stdcall, ret 0x14) em +0xea; sem memória, a caixa da
/// linha `linha` do fonte `arquivo` e fim. Depois cada jogo da `tabela`, na ordem, no seu lugar do vetor
/// (fixed_fixture_set_by_club_names 0x68aff0: push o 6º (fora, pelo mapa); push o 5º (casa); push ano; push mês (0 =
/// janeiro); push dia; push liga; mov ecx, [+0xea] + 16 × k -> thiscall, ret 0x18: a data e os clubes achados pelo
/// nome, −1 em +8/+0xc se não acha). Por fim, se algum dos n jogos ficou com um clube sem achar (+8 ou +0xc = −1), o
/// vetor é destruído (`eh vector destructor iterator` 0x9462bb: push 0x5a53c0; push conta; push 0x10; push vetor ->
/// stdcall, ret 0x10; operator delete 0x944c94 do bloco: push; add esp, 4 -> cdecl) e +0xea = 0: a liga volta ao
/// calendário sorteado.
pub(crate) unsafe fn set_up_fixed_fixtures(this: League, table: &[(u8, u8, u16, u32, u32)], file: usize, row: u32) {
    let n = league_match_count(this);
    let block = exe::crt_operator_new((n as u32).wrapping_mul(16).wrapping_add(4));
    let array = if block != 0 {
        write_usize(block, n as usize);
        exe::crt_vector_constructor_iterator(block + 4, 0x10, n as u32, 0x645800, 0x5a53c0);
        block + 4
    } else {
        0
    };
    this.set_fixed_fixtures(array);
    if array == 0 {
        show_error_at(file, row);
        return;
    }
    for (k, &(dia, mes, ano, casa, fora)) in table.iter().enumerate() {
        exe::fixed_fixture_set_by_club_names(this.fixed_fixtures() + k * 0x10, this, dia as u32, mes as u32, ano as u32, casa, fora); // cqn5: each fixed match
    }
    crate::comparison::branch("cqn_5 fixed calendar");
    let n = league_match_count(this);
    if n <= 0 {
        return;
    }
    let v = this.fixed_fixtures();
    for i in 0..n as usize {
        let f = v + i * 0x10;
        if read_usize(f + 8) == 0xffff_ffff || read_usize(f + 0xc) == 0xffff_ffff { // cqn5: a club not found drops the list
            if v != 0 {
                exe::crt_vector_destructor_iterator(v, 0x10, read_usize(v - 4) as u32, 0x5a53c0);
                exe::crt_operator_delete(v - 4);
            }
            this.set_fixed_fixtures(0);
            return;
        }
    }
}

/// the 2001/02 Serie A does not draw its fixtures: the 306 matches (34 rounds of 9) are fixed, with date and both clubs
/// by name; if any name is missing from the database, the whole list is dropped and the league falls back to the drawn
/// schedule
/// Original: `ItaSerA::create_fixed_fixtures` (0x642a70, ita_ser_a.cpp, thiscall, ret 0).
///
/// [pt] { 0x642a70 } (thiscall, ret 0, ita_ser_a.cpp, 11.650 bytes): `ItaSerA::create_fixed_fixtures()`: os 306 jogos da
/// Série A 2001/02 (34 rodadas de 9, com o dia, o mês (0 = janeiro) e o ano que a original empurra). Fonte 0x9ea2a8, linha 0x60f.
pub(crate) unsafe extern "thiscall" fn create_fixed_fixtures(this: usize) {
    set_up_fixed_fixtures(League(this), &SERIE_A_FIXED_FIXTURES, 0x9ea2a8, 0x60f)
}

/// The fixed matches (day, month with 0 = January, year, the 5th argument (home, by the map), the 6th (away)), with the
/// names in the exe's `.rdata`.
///
/// In the original routine's order (extracted from the disassembly and checked: the vector positions go in steps of
/// 16).
///
/// [pt] Os jogos fixos (dia, mês com 0 = janeiro, ano, o 5º argumento (casa, pelo mapa), o 6º (fora)), os nomes no .rdata do
/// exe; na ordem da original (extraídos da desmontagem e conferidos: os lugares do vetor vão de 16 em 16).
pub(crate) const SERIE_A_FIXED_FIXTURES: [(u8, u8, u16, u32, u32); 306] = [
    (28, 7, 2001, 0x9ea370, 0x9ea378), // Bologna × Atalanta
    (28, 7, 2001, 0x9ea360, 0x9ea368), // Brescia × Milan
    (28, 7, 2001, 0x9ea34c, 0x9ea358), // Fiorentina × Chievo
    (28, 7, 2001, 0x9ea33c, 0x9ea344), // Inter × Perugia
    (28, 7, 2001, 0x9ea328, 0x9ea334), // Juventus × Venezia
    (28, 7, 2001, 0x9ea314, 0x9ea31c), // Lazio × Piacenza
    (28, 7, 2001, 0x9ea304, 0x9ea30c), // Lecce × Parma
    (28, 7, 2001, 0x9ea2f4, 0x9ea2fc), // Udinese × Torino
    (28, 7, 2001, 0x9ea2e4, 0x9ea2ec), // Verona × Roma
    (9, 8, 2001, 0x9ea378, 0x9ea328), // Atalanta × Juventus
    (9, 8, 2001, 0x9ea358, 0x9ea370), // Chievo × Bologna
    (9, 8, 2001, 0x9ea368, 0x9ea34c), // Milan × Fiorentina
    (9, 8, 2001, 0x9ea30c, 0x9ea33c), // Parma × Inter
    (9, 8, 2001, 0x9ea344, 0x9ea314), // Perugia × Lazio
    (9, 8, 2001, 0x9ea31c, 0x9ea304), // Piacenza × Lecce
    (9, 8, 2001, 0x9ea2ec, 0x9ea2f4), // Roma × Udinese
    (9, 8, 2001, 0x9ea2fc, 0x9ea360), // Torino × Brescia
    (9, 8, 2001, 0x9ea334, 0x9ea2e4), // Venezia × Verona
    (16, 8, 2001, 0x9ea370, 0x9ea30c), // Bologna × Parma
    (16, 8, 2001, 0x9ea360, 0x9ea304), // Brescia × Lecce
    (16, 8, 2001, 0x9ea34c, 0x9ea378), // Fiorentina × Atalanta
    (16, 8, 2001, 0x9ea33c, 0x9ea334), // Inter × Venezia
    (16, 8, 2001, 0x9ea328, 0x9ea358), // Juventus × Chievo
    (16, 8, 2001, 0x9ea314, 0x9ea2fc), // Lazio × Torino
    (16, 8, 2001, 0x9ea31c, 0x9ea2ec), // Piacenza × Roma
    (16, 8, 2001, 0x9ea2f4, 0x9ea368), // Udinese × Milan
    (16, 8, 2001, 0x9ea2e4, 0x9ea344), // Verona × Perugia
    (23, 8, 2001, 0x9ea378, 0x9ea2e4), // Atalanta × Verona
    (23, 8, 2001, 0x9ea358, 0x9ea31c), // Chievo × Piacenza
    (23, 8, 2001, 0x9ea304, 0x9ea328), // Lecce × Juventus
    (23, 8, 2001, 0x9ea368, 0x9ea314), // Milan × Lazio
    (23, 8, 2001, 0x9ea30c, 0x9ea360), // Parma × Brescia
    (23, 8, 2001, 0x9ea344, 0x9ea2f4), // Perugia × Udinese
    (23, 8, 2001, 0x9ea2ec, 0x9ea34c), // Roma × Fiorentina
    (23, 8, 2001, 0x9ea2fc, 0x9ea33c), // Torino × Inter
    (23, 8, 2001, 0x9ea334, 0x9ea370), // Venezia × Bologna
    (30, 8, 2001, 0x9ea360, 0x9ea378), // Brescia × Atalanta
    (30, 8, 2001, 0x9ea34c, 0x9ea334), // Fiorentina × Venezia
    (30, 8, 2001, 0x9ea33c, 0x9ea370), // Inter × Bologna
    (30, 8, 2001, 0x9ea328, 0x9ea2ec), // Juventus × Roma
    (30, 8, 2001, 0x9ea314, 0x9ea30c), // Lazio × Parma
    (30, 8, 2001, 0x9ea344, 0x9ea368), // Perugia × Milan
    (30, 8, 2001, 0x9ea31c, 0x9ea2fc), // Piacenza × Torino
    (30, 8, 2001, 0x9ea2f4, 0x9ea358), // Udinese × Chievo
    (30, 8, 2001, 0x9ea2e4, 0x9ea304), // Verona × Lecce
    (10, 9, 2001, 0x9ea378, 0x9ea368), // Atalanta × Milan
    (10, 9, 2001, 0x9ea370, 0x9ea2f4), // Bologna × Udinese
    (10, 9, 2001, 0x9ea358, 0x9ea314), // Chievo × Lazio
    (10, 9, 2001, 0x9ea34c, 0x9ea328), // Fiorentina × Juventus
    (10, 9, 2001, 0x9ea33c, 0x9ea2e4), // Inter × Verona
    (10, 9, 2001, 0x9ea304, 0x9ea344), // Lecce × Perugia
    (10, 9, 2001, 0x9ea2ec, 0x9ea360), // Roma × Brescia
    (10, 9, 2001, 0x9ea2fc, 0x9ea30c), // Torino × Parma
    (10, 9, 2001, 0x9ea334, 0x9ea31c), // Venezia × Piacenza
    (14, 9, 2001, 0x9ea360, 0x9ea358), // Brescia × Chievo
    (14, 9, 2001, 0x9ea328, 0x9ea2fc), // Juventus × Torino
    (14, 9, 2001, 0x9ea314, 0x9ea378), // Lazio × Atalanta
    (14, 9, 2001, 0x9ea304, 0x9ea34c), // Lecce × Fiorentina
    (14, 9, 2001, 0x9ea368, 0x9ea334), // Milan × Venezia
    (14, 9, 2001, 0x9ea30c, 0x9ea31c), // Parma × Piacenza
    (14, 9, 2001, 0x9ea344, 0x9ea2ec), // Perugia × Roma
    (14, 9, 2001, 0x9ea2f4, 0x9ea33c), // Udinese × Inter
    (14, 9, 2001, 0x9ea2e4, 0x9ea370), // Verona × Bologna
    (21, 9, 2001, 0x9ea378, 0x9ea2f4), // Atalanta × Udinese
    (21, 9, 2001, 0x9ea370, 0x9ea328), // Bologna × Juventus
    (21, 9, 2001, 0x9ea358, 0x9ea30c), // Chievo × Parma
    (21, 9, 2001, 0x9ea34c, 0x9ea2e4), // Fiorentina × Verona
    (21, 9, 2001, 0x9ea33c, 0x9ea368), // Inter × Milan
    (21, 9, 2001, 0x9ea31c, 0x9ea360), // Piacenza × Brescia
    (21, 9, 2001, 0x9ea2ec, 0x9ea304), // Roma × Lecce
    (21, 9, 2001, 0x9ea2fc, 0x9ea344), // Torino × Perugia
    (21, 9, 2001, 0x9ea334, 0x9ea314), // Venezia × Lazio
    (28, 9, 2001, 0x9ea360, 0x9ea334), // Brescia × Venezia
    (28, 9, 2001, 0x9ea358, 0x9ea2fc), // Chievo × Torino
    (28, 9, 2001, 0x9ea328, 0x9ea33c), // Juventus × Inter
    (28, 9, 2001, 0x9ea304, 0x9ea378), // Lecce × Atalanta
    (28, 9, 2001, 0x9ea368, 0x9ea370), // Milan × Bologna
    (28, 9, 2001, 0x9ea30c, 0x9ea2e4), // Parma × Verona
    (28, 9, 2001, 0x9ea344, 0x9ea31c), // Perugia × Piacenza
    (28, 9, 2001, 0x9ea2ec, 0x9ea314), // Roma × Lazio
    (28, 9, 2001, 0x9ea2f4, 0x9ea34c), // Udinese × Fiorentina
    (4, 10, 2001, 0x9ea378, 0x9ea2ec), // Atalanta × Roma
    (4, 10, 2001, 0x9ea370, 0x9ea34c), // Bologna × Fiorentina
    (4, 10, 2001, 0x9ea33c, 0x9ea304), // Inter × Lecce
    (4, 10, 2001, 0x9ea314, 0x9ea360), // Lazio × Brescia
    (4, 10, 2001, 0x9ea30c, 0x9ea344), // Parma × Perugia
    (4, 10, 2001, 0x9ea31c, 0x9ea2f4), // Piacenza × Udinese
    (4, 10, 2001, 0x9ea2fc, 0x9ea368), // Torino × Milan
    (4, 10, 2001, 0x9ea334, 0x9ea358), // Venezia × Chievo
    (4, 10, 2001, 0x9ea2e4, 0x9ea328), // Verona × Juventus
    (18, 10, 2001, 0x9ea378, 0x9ea334), // Atalanta × Venezia
    (18, 10, 2001, 0x9ea34c, 0x9ea2fc), // Fiorentina × Torino
    (18, 10, 2001, 0x9ea328, 0x9ea30c), // Juventus × Parma
    (18, 10, 2001, 0x9ea304, 0x9ea370), // Lecce × Bologna
    (18, 10, 2001, 0x9ea368, 0x9ea31c), // Milan × Piacenza
    (18, 10, 2001, 0x9ea344, 0x9ea360), // Perugia × Brescia
    (18, 10, 2001, 0x9ea2ec, 0x9ea33c), // Roma × Inter
    (18, 10, 2001, 0x9ea2f4, 0x9ea314), // Udinese × Lazio
    (18, 10, 2001, 0x9ea2e4, 0x9ea358), // Verona × Chievo
    (25, 10, 2001, 0x9ea370, 0x9ea2ec), // Bologna × Roma
    (25, 10, 2001, 0x9ea360, 0x9ea2f4), // Brescia × Udinese
    (25, 10, 2001, 0x9ea358, 0x9ea344), // Chievo × Perugia
    (25, 10, 2001, 0x9ea33c, 0x9ea34c), // Inter × Fiorentina
    (25, 10, 2001, 0x9ea314, 0x9ea328), // Lazio × Juventus
    (25, 10, 2001, 0x9ea30c, 0x9ea368), // Parma × Milan
    (25, 10, 2001, 0x9ea31c, 0x9ea378), // Piacenza × Atalanta
    (25, 10, 2001, 0x9ea2fc, 0x9ea2e4), // Torino × Verona
    (25, 10, 2001, 0x9ea334, 0x9ea304), // Venezia × Lecce
    (2, 11, 2001, 0x9ea378, 0x9ea33c), // Atalanta × Inter
    (2, 11, 2001, 0x9ea370, 0x9ea2fc), // Bologna × Torino
    (2, 11, 2001, 0x9ea34c, 0x9ea31c), // Fiorentina × Piacenza
    (2, 11, 2001, 0x9ea328, 0x9ea344), // Juventus × Perugia
    (2, 11, 2001, 0x9ea304, 0x9ea314), // Lecce × Lazio
    (2, 11, 2001, 0x9ea368, 0x9ea358), // Milan × Chievo
    (2, 11, 2001, 0x9ea2ec, 0x9ea334), // Roma × Venezia
    (2, 11, 2001, 0x9ea2f4, 0x9ea30c), // Udinese × Parma
    (2, 11, 2001, 0x9ea2e4, 0x9ea360), // Verona × Brescia
    (9, 11, 2001, 0x9ea360, 0x9ea33c), // Brescia × Inter
    (9, 11, 2001, 0x9ea358, 0x9ea304), // Chievo × Lecce
    (9, 11, 2001, 0x9ea314, 0x9ea34c), // Lazio × Fiorentina
    (9, 11, 2001, 0x9ea368, 0x9ea328), // Milan × Juventus
    (9, 11, 2001, 0x9ea30c, 0x9ea2ec), // Parma × Roma
    (9, 11, 2001, 0x9ea344, 0x9ea334), // Perugia × Venezia
    (9, 11, 2001, 0x9ea31c, 0x9ea370), // Piacenza × Bologna
    (9, 11, 2001, 0x9ea2fc, 0x9ea378), // Torino × Atalanta
    (9, 11, 2001, 0x9ea2f4, 0x9ea2e4), // Udinese × Verona
    (16, 11, 2001, 0x9ea378, 0x9ea30c), // Atalanta × Parma
    (16, 11, 2001, 0x9ea370, 0x9ea344), // Bologna × Perugia
    (16, 11, 2001, 0x9ea34c, 0x9ea360), // Fiorentina × Brescia
    (16, 11, 2001, 0x9ea33c, 0x9ea358), // Inter × Chievo
    (16, 11, 2001, 0x9ea328, 0x9ea31c), // Juventus × Piacenza
    (16, 11, 2001, 0x9ea304, 0x9ea2fc), // Lecce × Torino
    (16, 11, 2001, 0x9ea2ec, 0x9ea368), // Roma × Milan
    (16, 11, 2001, 0x9ea334, 0x9ea2f4), // Venezia × Udinese
    (16, 11, 2001, 0x9ea2e4, 0x9ea314), // Verona × Lazio
    (6, 0, 2002, 0x9ea360, 0x9ea328), // Brescia × Juventus
    (6, 0, 2002, 0x9ea358, 0x9ea2ec), // Chievo × Roma
    (6, 0, 2002, 0x9ea314, 0x9ea370), // Lazio × Bologna
    (6, 0, 2002, 0x9ea368, 0x9ea2e4), // Milan × Verona
    (6, 0, 2002, 0x9ea30c, 0x9ea34c), // Parma × Fiorentina
    (6, 0, 2002, 0x9ea344, 0x9ea378), // Perugia × Atalanta
    (6, 0, 2002, 0x9ea31c, 0x9ea33c), // Piacenza × Inter
    (6, 0, 2002, 0x9ea2fc, 0x9ea334), // Torino × Venezia
    (6, 0, 2002, 0x9ea2f4, 0x9ea304), // Udinese × Lecce
    (13, 0, 2002, 0x9ea378, 0x9ea358), // Atalanta × Chievo
    (13, 0, 2002, 0x9ea370, 0x9ea360), // Bologna × Brescia
    (13, 0, 2002, 0x9ea34c, 0x9ea344), // Fiorentina × Perugia
    (13, 0, 2002, 0x9ea33c, 0x9ea314), // Inter × Lazio
    (13, 0, 2002, 0x9ea328, 0x9ea2f4), // Juventus × Udinese
    (13, 0, 2002, 0x9ea304, 0x9ea368), // Lecce × Milan
    (13, 0, 2002, 0x9ea2ec, 0x9ea2fc), // Roma × Torino
    (13, 0, 2002, 0x9ea334, 0x9ea30c), // Venezia × Parma
    (13, 0, 2002, 0x9ea2e4, 0x9ea31c), // Verona × Piacenza
    (20, 0, 2002, 0x9ea378, 0x9ea370), // Atalanta × Bologna
    (20, 0, 2002, 0x9ea368, 0x9ea360), // Milan × Brescia
    (20, 0, 2002, 0x9ea358, 0x9ea34c), // Chievo × Fiorentina
    (20, 0, 2002, 0x9ea344, 0x9ea33c), // Perugia × Inter
    (20, 0, 2002, 0x9ea334, 0x9ea328), // Venezia × Juventus
    (20, 0, 2002, 0x9ea31c, 0x9ea314), // Piacenza × Lazio
    (20, 0, 2002, 0x9ea30c, 0x9ea304), // Parma × Lecce
    (20, 0, 2002, 0x9ea2fc, 0x9ea2f4), // Torino × Udinese
    (20, 0, 2002, 0x9ea2ec, 0x9ea2e4), // Roma × Verona
    (27, 0, 2002, 0x9ea328, 0x9ea378), // Juventus × Atalanta
    (27, 0, 2002, 0x9ea370, 0x9ea358), // Bologna × Chievo
    (27, 0, 2002, 0x9ea34c, 0x9ea368), // Fiorentina × Milan
    (27, 0, 2002, 0x9ea33c, 0x9ea30c), // Inter × Parma
    (27, 0, 2002, 0x9ea314, 0x9ea344), // Lazio × Perugia
    (27, 0, 2002, 0x9ea304, 0x9ea31c), // Lecce × Piacenza
    (27, 0, 2002, 0x9ea2f4, 0x9ea2ec), // Udinese × Roma
    (27, 0, 2002, 0x9ea360, 0x9ea2fc), // Brescia × Torino
    (27, 0, 2002, 0x9ea2e4, 0x9ea334), // Verona × Venezia
    (3, 1, 2002, 0x9ea30c, 0x9ea370), // Parma × Bologna
    (3, 1, 2002, 0x9ea304, 0x9ea360), // Lecce × Brescia
    (3, 1, 2002, 0x9ea378, 0x9ea34c), // Atalanta × Fiorentina
    (3, 1, 2002, 0x9ea334, 0x9ea33c), // Venezia × Inter
    (3, 1, 2002, 0x9ea358, 0x9ea328), // Chievo × Juventus
    (3, 1, 2002, 0x9ea2fc, 0x9ea314), // Torino × Lazio
    (3, 1, 2002, 0x9ea2ec, 0x9ea31c), // Roma × Piacenza
    (3, 1, 2002, 0x9ea368, 0x9ea2f4), // Milan × Udinese
    (3, 1, 2002, 0x9ea344, 0x9ea2e4), // Perugia × Verona
    (10, 1, 2002, 0x9ea2e4, 0x9ea378), // Verona × Atalanta
    (10, 1, 2002, 0x9ea31c, 0x9ea358), // Piacenza × Chievo
    (10, 1, 2002, 0x9ea328, 0x9ea304), // Juventus × Lecce
    (10, 1, 2002, 0x9ea314, 0x9ea368), // Lazio × Milan
    (10, 1, 2002, 0x9ea360, 0x9ea30c), // Brescia × Parma
    (10, 1, 2002, 0x9ea2f4, 0x9ea344), // Udinese × Perugia
    (10, 1, 2002, 0x9ea34c, 0x9ea2ec), // Fiorentina × Roma
    (10, 1, 2002, 0x9ea33c, 0x9ea2fc), // Inter × Torino
    (10, 1, 2002, 0x9ea370, 0x9ea334), // Bologna × Venezia
    (17, 1, 2002, 0x9ea378, 0x9ea360), // Atalanta × Brescia
    (17, 1, 2002, 0x9ea334, 0x9ea34c), // Venezia × Fiorentina
    (17, 1, 2002, 0x9ea370, 0x9ea33c), // Bologna × Inter
    (17, 1, 2002, 0x9ea2ec, 0x9ea328), // Roma × Juventus
    (17, 1, 2002, 0x9ea30c, 0x9ea314), // Parma × Lazio
    (17, 1, 2002, 0x9ea368, 0x9ea344), // Milan × Perugia
    (17, 1, 2002, 0x9ea2fc, 0x9ea31c), // Torino × Piacenza
    (17, 1, 2002, 0x9ea358, 0x9ea2f4), // Chievo × Udinese
    (17, 1, 2002, 0x9ea304, 0x9ea2e4), // Lecce × Verona
    (24, 1, 2002, 0x9ea368, 0x9ea378), // Milan × Atalanta
    (24, 1, 2002, 0x9ea2f4, 0x9ea370), // Udinese × Bologna
    (24, 1, 2002, 0x9ea314, 0x9ea358), // Lazio × Chievo
    (24, 1, 2002, 0x9ea328, 0x9ea34c), // Juventus × Fiorentina
    (24, 1, 2002, 0x9ea2e4, 0x9ea33c), // Verona × Inter
    (24, 1, 2002, 0x9ea344, 0x9ea304), // Perugia × Lecce
    (24, 1, 2002, 0x9ea360, 0x9ea2ec), // Brescia × Roma
    (24, 1, 2002, 0x9ea30c, 0x9ea2fc), // Parma × Torino
    (24, 1, 2002, 0x9ea31c, 0x9ea334), // Piacenza × Venezia
    (3, 2, 2002, 0x9ea358, 0x9ea360), // Chievo × Brescia
    (3, 2, 2002, 0x9ea2fc, 0x9ea328), // Torino × Juventus
    (3, 2, 2002, 0x9ea378, 0x9ea314), // Atalanta × Lazio
    (3, 2, 2002, 0x9ea34c, 0x9ea304), // Fiorentina × Lecce
    (3, 2, 2002, 0x9ea334, 0x9ea368), // Venezia × Milan
    (3, 2, 2002, 0x9ea31c, 0x9ea30c), // Piacenza × Parma
    (3, 2, 2002, 0x9ea2ec, 0x9ea344), // Roma × Perugia
    (3, 2, 2002, 0x9ea33c, 0x9ea2f4), // Inter × Udinese
    (3, 2, 2002, 0x9ea370, 0x9ea2e4), // Bologna × Verona
    (10, 2, 2002, 0x9ea2f4, 0x9ea378), // Udinese × Atalanta
    (10, 2, 2002, 0x9ea328, 0x9ea370), // Juventus × Bologna
    (10, 2, 2002, 0x9ea30c, 0x9ea358), // Parma × Chievo
    (10, 2, 2002, 0x9ea2e4, 0x9ea34c), // Verona × Fiorentina
    (10, 2, 2002, 0x9ea368, 0x9ea33c), // Milan × Inter
    (10, 2, 2002, 0x9ea360, 0x9ea31c), // Brescia × Piacenza
    (10, 2, 2002, 0x9ea304, 0x9ea2ec), // Lecce × Roma
    (10, 2, 2002, 0x9ea344, 0x9ea2fc), // Perugia × Torino
    (10, 2, 2002, 0x9ea314, 0x9ea334), // Lazio × Venezia
    (17, 2, 2002, 0x9ea334, 0x9ea360), // Venezia × Brescia
    (17, 2, 2002, 0x9ea2fc, 0x9ea358), // Torino × Chievo
    (17, 2, 2002, 0x9ea33c, 0x9ea328), // Inter × Juventus
    (17, 2, 2002, 0x9ea378, 0x9ea304), // Atalanta × Lecce
    (17, 2, 2002, 0x9ea370, 0x9ea368), // Bologna × Milan
    (17, 2, 2002, 0x9ea2e4, 0x9ea30c), // Verona × Parma
    (17, 2, 2002, 0x9ea31c, 0x9ea344), // Piacenza × Perugia
    (17, 2, 2002, 0x9ea314, 0x9ea2ec), // Lazio × Roma
    (17, 2, 2002, 0x9ea34c, 0x9ea2f4), // Fiorentina × Udinese
    (24, 2, 2002, 0x9ea2ec, 0x9ea378), // Roma × Atalanta
    (24, 2, 2002, 0x9ea34c, 0x9ea370), // Fiorentina × Bologna
    (24, 2, 2002, 0x9ea304, 0x9ea33c), // Lecce × Inter
    (24, 2, 2002, 0x9ea360, 0x9ea314), // Brescia × Lazio
    (24, 2, 2002, 0x9ea344, 0x9ea30c), // Perugia × Parma
    (24, 2, 2002, 0x9ea2f4, 0x9ea31c), // Udinese × Piacenza
    (24, 2, 2002, 0x9ea368, 0x9ea2fc), // Milan × Torino
    (24, 2, 2002, 0x9ea358, 0x9ea334), // Chievo × Venezia
    (24, 2, 2002, 0x9ea328, 0x9ea2e4), // Juventus × Verona
    (30, 2, 2002, 0x9ea334, 0x9ea378), // Venezia × Atalanta
    (30, 2, 2002, 0x9ea2fc, 0x9ea34c), // Torino × Fiorentina
    (30, 2, 2002, 0x9ea30c, 0x9ea328), // Parma × Juventus
    (30, 2, 2002, 0x9ea370, 0x9ea304), // Bologna × Lecce
    (30, 2, 2002, 0x9ea31c, 0x9ea368), // Piacenza × Milan
    (30, 2, 2002, 0x9ea360, 0x9ea344), // Brescia × Perugia
    (30, 2, 2002, 0x9ea33c, 0x9ea2ec), // Inter × Roma
    (30, 2, 2002, 0x9ea314, 0x9ea2f4), // Lazio × Udinese
    (30, 2, 2002, 0x9ea358, 0x9ea2e4), // Chievo × Verona
    (7, 3, 2002, 0x9ea2ec, 0x9ea370), // Roma × Bologna
    (7, 3, 2002, 0x9ea2f4, 0x9ea360), // Udinese × Brescia
    (7, 3, 2002, 0x9ea344, 0x9ea358), // Perugia × Chievo
    (7, 3, 2002, 0x9ea34c, 0x9ea33c), // Fiorentina × Inter
    (7, 3, 2002, 0x9ea328, 0x9ea314), // Juventus × Lazio
    (7, 3, 2002, 0x9ea368, 0x9ea30c), // Milan × Parma
    (7, 3, 2002, 0x9ea378, 0x9ea31c), // Atalanta × Piacenza
    (7, 3, 2002, 0x9ea2e4, 0x9ea2fc), // Verona × Torino
    (7, 3, 2002, 0x9ea304, 0x9ea334), // Lecce × Venezia
    (14, 3, 2002, 0x9ea33c, 0x9ea378), // Inter × Atalanta
    (14, 3, 2002, 0x9ea2fc, 0x9ea370), // Torino × Bologna
    (14, 3, 2002, 0x9ea31c, 0x9ea34c), // Piacenza × Fiorentina
    (14, 3, 2002, 0x9ea344, 0x9ea328), // Perugia × Juventus
    (14, 3, 2002, 0x9ea314, 0x9ea304), // Lazio × Lecce
    (14, 3, 2002, 0x9ea358, 0x9ea368), // Chievo × Milan
    (14, 3, 2002, 0x9ea334, 0x9ea2ec), // Venezia × Roma
    (14, 3, 2002, 0x9ea30c, 0x9ea2f4), // Parma × Udinese
    (14, 3, 2002, 0x9ea360, 0x9ea2e4), // Brescia × Verona
    (21, 3, 2002, 0x9ea33c, 0x9ea360), // Inter × Brescia
    (21, 3, 2002, 0x9ea304, 0x9ea358), // Lecce × Chievo
    (21, 3, 2002, 0x9ea34c, 0x9ea314), // Fiorentina × Lazio
    (21, 3, 2002, 0x9ea328, 0x9ea368), // Juventus × Milan
    (21, 3, 2002, 0x9ea2ec, 0x9ea30c), // Roma × Parma
    (21, 3, 2002, 0x9ea334, 0x9ea344), // Venezia × Perugia
    (21, 3, 2002, 0x9ea370, 0x9ea31c), // Bologna × Piacenza
    (21, 3, 2002, 0x9ea378, 0x9ea2fc), // Atalanta × Torino
    (21, 3, 2002, 0x9ea2e4, 0x9ea2f4), // Verona × Udinese
    (28, 3, 2002, 0x9ea30c, 0x9ea378), // Parma × Atalanta
    (28, 3, 2002, 0x9ea344, 0x9ea370), // Perugia × Bologna
    (28, 3, 2002, 0x9ea360, 0x9ea34c), // Brescia × Fiorentina
    (28, 3, 2002, 0x9ea358, 0x9ea33c), // Chievo × Inter
    (28, 3, 2002, 0x9ea31c, 0x9ea328), // Piacenza × Juventus
    (28, 3, 2002, 0x9ea2fc, 0x9ea304), // Torino × Lecce
    (28, 3, 2002, 0x9ea368, 0x9ea2ec), // Milan × Roma
    (28, 3, 2002, 0x9ea2f4, 0x9ea334), // Udinese × Venezia
    (28, 3, 2002, 0x9ea314, 0x9ea2e4), // Lazio × Verona
    (5, 4, 2002, 0x9ea328, 0x9ea360), // Juventus × Brescia
    (5, 4, 2002, 0x9ea2ec, 0x9ea358), // Roma × Chievo
    (5, 4, 2002, 0x9ea370, 0x9ea314), // Bologna × Lazio
    (5, 4, 2002, 0x9ea2e4, 0x9ea368), // Verona × Milan
    (5, 4, 2002, 0x9ea34c, 0x9ea30c), // Fiorentina × Parma
    (5, 4, 2002, 0x9ea378, 0x9ea344), // Atalanta × Perugia
    (5, 4, 2002, 0x9ea33c, 0x9ea31c), // Inter × Piacenza
    (5, 4, 2002, 0x9ea334, 0x9ea2fc), // Venezia × Torino
    (5, 4, 2002, 0x9ea304, 0x9ea2f4), // Lecce × Udinese
    (12, 4, 2002, 0x9ea358, 0x9ea378), // Chievo × Atalanta
    (12, 4, 2002, 0x9ea360, 0x9ea370), // Brescia × Bologna
    (12, 4, 2002, 0x9ea344, 0x9ea34c), // Perugia × Fiorentina
    (12, 4, 2002, 0x9ea314, 0x9ea33c), // Lazio × Inter
    (12, 4, 2002, 0x9ea2f4, 0x9ea328), // Udinese × Juventus
    (12, 4, 2002, 0x9ea368, 0x9ea304), // Milan × Lecce
    (12, 4, 2002, 0x9ea2fc, 0x9ea2ec), // Torino × Roma
    (12, 4, 2002, 0x9ea30c, 0x9ea334), // Parma × Venezia
    (12, 4, 2002, 0x9ea31c, 0x9ea2e4), // Piacenza × Verona
];

pub(crate) const SERIE_B_FIXTURES: [(u8, u8, u16, u32, u32); 380] = [
    (26, 7, 2001, 0x9ea464, 0x9ea46c), // Ancona × Reggina
    (26, 7, 2001, 0x9ea450, 0x9ea45c), // Cagliari × Messina
    (26, 7, 2001, 0x9ea440, 0x9ea448), // Como × Crotone
    (26, 7, 2001, 0x9ea42c, 0x9ea434), // Cosenza × Cittadella
    (26, 7, 2001, 0x9ea41c, 0x9ea424), // Empoli × Palermo
    (26, 7, 2001, 0x9ea40c, 0x9ea414), // Genoa × Napoli
    (26, 7, 2001, 0x9ea3fc, 0x9ea404), // Modena × Bari
    (26, 7, 2001, 0x9ea3e4, 0x9ea3f0), // Salernitana × Sampdoria
    (26, 7, 2001, 0x9ea3d4, 0x9ea3dc), // Ternana × Siena
    (26, 7, 2001, 0x9ea3c0, 0x9ea3c8), // Vicenza × Pistoiese
    (2, 8, 2001, 0x9ea404, 0x9ea41c), // Bari × Empoli
    (2, 8, 2001, 0x9ea434, 0x9ea3e4), // Cittadella × Salernitana
    (2, 8, 2001, 0x9ea448, 0x9ea3fc), // Crotone × Modena
    (2, 8, 2001, 0x9ea45c, 0x9ea3d4), // Messina × Ternana
    (2, 8, 2001, 0x9ea414, 0x9ea464), // Napoli × Ancona
    (2, 8, 2001, 0x9ea424, 0x9ea450), // Palermo × Cagliari
    (2, 8, 2001, 0x9ea3c8, 0x9ea40c), // Pistoiese × Genoa
    (2, 8, 2001, 0x9ea46c, 0x9ea440), // Reggina × Como
    (2, 8, 2001, 0x9ea3f0, 0x9ea42c), // Sampdoria × Cosenza
    (2, 8, 2001, 0x9ea3dc, 0x9ea3c0), // Siena × Vicenza
    (9, 8, 2001, 0x9ea464, 0x9ea3dc), // Ancona × Siena
    (9, 8, 2001, 0x9ea450, 0x9ea404), // Cagliari × Bari
    (9, 8, 2001, 0x9ea42c, 0x9ea3d4), // Cosenza × Ternana
    (9, 8, 2001, 0x9ea41c, 0x9ea45c), // Empoli × Messina
    (9, 8, 2001, 0x9ea40c, 0x9ea440), // Genoa × Como
    (9, 8, 2001, 0x9ea3fc, 0x9ea414), // Modena × Napoli
    (9, 8, 2001, 0x9ea424, 0x9ea3c8), // Palermo × Pistoiese
    (9, 8, 2001, 0x9ea46c, 0x9ea3f0), // Reggina × Sampdoria
    (9, 8, 2001, 0x9ea3e4, 0x9ea448), // Salernitana × Crotone
    (9, 8, 2001, 0x9ea3c0, 0x9ea434), // Vicenza × Cittadella
    (16, 8, 2001, 0x9ea404, 0x9ea424), // Bari × Palermo
    (16, 8, 2001, 0x9ea434, 0x9ea46c), // Cittadella × Reggina
    (16, 8, 2001, 0x9ea440, 0x9ea3e4), // Como × Salernitana
    (16, 8, 2001, 0x9ea448, 0x9ea450), // Crotone × Cagliari
    (16, 8, 2001, 0x9ea45c, 0x9ea464), // Messina × Ancona
    (16, 8, 2001, 0x9ea414, 0x9ea3c0), // Napoli × Vicenza
    (16, 8, 2001, 0x9ea3c8, 0x9ea42c), // Pistoiese × Cosenza
    (16, 8, 2001, 0x9ea3f0, 0x9ea41c), // Sampdoria × Empoli
    (16, 8, 2001, 0x9ea3dc, 0x9ea40c), // Siena × Genoa
    (16, 8, 2001, 0x9ea3d4, 0x9ea3fc), // Ternana × Modena
    (23, 8, 2001, 0x9ea464, 0x9ea448), // Ancona × Crotone
    (23, 8, 2001, 0x9ea450, 0x9ea3f0), // Cagliari × Sampdoria
    (23, 8, 2001, 0x9ea42c, 0x9ea440), // Cosenza × Como
    (23, 8, 2001, 0x9ea41c, 0x9ea414), // Empoli × Napoli
    (23, 8, 2001, 0x9ea40c, 0x9ea45c), // Genoa × Messina
    (23, 8, 2001, 0x9ea3fc, 0x9ea3dc), // Modena × Siena
    (23, 8, 2001, 0x9ea424, 0x9ea434), // Palermo × Cittadella
    (23, 8, 2001, 0x9ea46c, 0x9ea404), // Reggina × Bari
    (23, 8, 2001, 0x9ea3e4, 0x9ea3c8), // Salernitana × Pistoiese
    (23, 8, 2001, 0x9ea3c0, 0x9ea3d4), // Vicenza × Ternana
    (30, 8, 2001, 0x9ea404, 0x9ea448), // Bari × Crotone
    (30, 8, 2001, 0x9ea434, 0x9ea41c), // Cittadella × Empoli
    (30, 8, 2001, 0x9ea440, 0x9ea464), // Como × Ancona
    (30, 8, 2001, 0x9ea42c, 0x9ea46c), // Cosenza × Reggina
    (30, 8, 2001, 0x9ea45c, 0x9ea3c0), // Messina × Vicenza
    (30, 8, 2001, 0x9ea414, 0x9ea450), // Napoli × Cagliari
    (30, 8, 2001, 0x9ea3c8, 0x9ea3fc), // Pistoiese × Modena
    (30, 8, 2001, 0x9ea3f0, 0x9ea424), // Sampdoria × Palermo
    (30, 8, 2001, 0x9ea3dc, 0x9ea3e4), // Siena × Salernitana
    (30, 8, 2001, 0x9ea3d4, 0x9ea40c), // Ternana × Genoa
    (7, 9, 2001, 0x9ea464, 0x9ea40c), // Ancona × Genoa
    (7, 9, 2001, 0x9ea450, 0x9ea3dc), // Cagliari × Siena
    (7, 9, 2001, 0x9ea448, 0x9ea434), // Crotone × Cittadella
    (7, 9, 2001, 0x9ea41c, 0x9ea3c8), // Empoli × Pistoiese
    (7, 9, 2001, 0x9ea3fc, 0x9ea440), // Modena × Como
    (7, 9, 2001, 0x9ea46c, 0x9ea424), // Reggina × Palermo
    (7, 9, 2001, 0x9ea3e4, 0x9ea45c), // Salernitana × Messina
    (7, 9, 2001, 0x9ea3f0, 0x9ea404), // Sampdoria × Bari
    (7, 9, 2001, 0x9ea3d4, 0x9ea414), // Ternana × Napoli
    (7, 9, 2001, 0x9ea3c0, 0x9ea42c), // Vicenza × Cosenza
    (10, 9, 2001, 0x9ea464, 0x9ea3f0), // Ancona × Sampdoria
    (10, 9, 2001, 0x9ea440, 0x9ea3d4), // Como × Ternana
    (10, 9, 2001, 0x9ea42c, 0x9ea3fc), // Cosenza × Modena
    (10, 9, 2001, 0x9ea41c, 0x9ea450), // Empoli × Cagliari
    (10, 9, 2001, 0x9ea40c, 0x9ea434), // Genoa × Cittadella
    (10, 9, 2001, 0x9ea45c, 0x9ea3dc), // Messina × Siena
    (10, 9, 2001, 0x9ea414, 0x9ea3e4), // Napoli × Salernitana
    (10, 9, 2001, 0x9ea424, 0x9ea448), // Palermo × Crotone
    (10, 9, 2001, 0x9ea3c8, 0x9ea46c), // Pistoiese × Reggina
    (10, 9, 2001, 0x9ea3c0, 0x9ea404), // Vicenza × Bari
    (14, 9, 2001, 0x9ea404, 0x9ea40c), // Bari × Genoa
    (14, 9, 2001, 0x9ea434, 0x9ea414), // Cittadella × Napoli
    (14, 9, 2001, 0x9ea448, 0x9ea3c0), // Crotone × Vicenza
    (14, 9, 2001, 0x9ea45c, 0x9ea424), // Messina × Palermo
    (14, 9, 2001, 0x9ea3fc, 0x9ea450), // Modena × Cagliari
    (14, 9, 2001, 0x9ea46c, 0x9ea41c), // Reggina × Empoli
    (14, 9, 2001, 0x9ea3e4, 0x9ea42c), // Salernitana × Cosenza
    (14, 9, 2001, 0x9ea3f0, 0x9ea3c8), // Sampdoria × Pistoiese
    (14, 9, 2001, 0x9ea3dc, 0x9ea440), // Siena × Como
    (14, 9, 2001, 0x9ea3d4, 0x9ea464), // Ternana × Ancona
    (28, 9, 2001, 0x9ea464, 0x9ea434), // Ancona × Cittadella
    (28, 9, 2001, 0x9ea450, 0x9ea46c), // Cagliari × Reggina
    (28, 9, 2001, 0x9ea440, 0x9ea45c), // Como × Messina
    (28, 9, 2001, 0x9ea42c, 0x9ea3dc), // Cosenza × Siena
    (28, 9, 2001, 0x9ea41c, 0x9ea448), // Empoli × Crotone
    (28, 9, 2001, 0x9ea40c, 0x9ea3e4), // Genoa × Salernitana
    (28, 9, 2001, 0x9ea414, 0x9ea3f0), // Napoli × Sampdoria
    (28, 9, 2001, 0x9ea424, 0x9ea3d4), // Palermo × Ternana
    (28, 9, 2001, 0x9ea3c8, 0x9ea404), // Pistoiese × Bari
    (28, 9, 2001, 0x9ea3c0, 0x9ea3fc), // Vicenza × Modena
    (4, 10, 2001, 0x9ea404, 0x9ea414), // Bari × Napoli
    (4, 10, 2001, 0x9ea434, 0x9ea450), // Cittadella × Cagliari
    (4, 10, 2001, 0x9ea440, 0x9ea3c0), // Como × Vicenza
    (4, 10, 2001, 0x9ea448, 0x9ea46c), // Crotone × Reggina
    (4, 10, 2001, 0x9ea40c, 0x9ea3f0), // Genoa × Sampdoria
    (4, 10, 2001, 0x9ea45c, 0x9ea42c), // Messina × Cosenza
    (4, 10, 2001, 0x9ea3fc, 0x9ea41c), // Modena × Empoli
    (4, 10, 2001, 0x9ea3e4, 0x9ea464), // Salernitana × Ancona
    (4, 10, 2001, 0x9ea3dc, 0x9ea424), // Siena × Palermo
    (4, 10, 2001, 0x9ea3d4, 0x9ea3c8), // Ternana × Pistoiese
    (11, 10, 2001, 0x9ea404, 0x9ea3dc), // Bari × Siena
    (11, 10, 2001, 0x9ea450, 0x9ea3d4), // Cagliari × Ternana
    (11, 10, 2001, 0x9ea42c, 0x9ea40c), // Cosenza × Genoa
    (11, 10, 2001, 0x9ea41c, 0x9ea3e4), // Empoli × Salernitana
    (11, 10, 2001, 0x9ea414, 0x9ea448), // Napoli × Crotone
    (11, 10, 2001, 0x9ea424, 0x9ea3fc), // Palermo × Modena
    (11, 10, 2001, 0x9ea3c8, 0x9ea440), // Pistoiese × Como
    (11, 10, 2001, 0x9ea46c, 0x9ea45c), // Reggina × Messina
    (11, 10, 2001, 0x9ea3f0, 0x9ea434), // Sampdoria × Cittadella
    (11, 10, 2001, 0x9ea3c0, 0x9ea464), // Vicenza × Ancona
    (18, 10, 2001, 0x9ea464, 0x9ea42c), // Ancona × Cosenza
    (18, 10, 2001, 0x9ea434, 0x9ea404), // Cittadella × Bari
    (18, 10, 2001, 0x9ea440, 0x9ea424), // Como × Palermo
    (18, 10, 2001, 0x9ea448, 0x9ea3f0), // Crotone × Sampdoria
    (18, 10, 2001, 0x9ea40c, 0x9ea3c0), // Genoa × Vicenza
    (18, 10, 2001, 0x9ea45c, 0x9ea414), // Messina × Napoli
    (18, 10, 2001, 0x9ea3fc, 0x9ea46c), // Modena × Reggina
    (18, 10, 2001, 0x9ea3e4, 0x9ea450), // Salernitana × Cagliari
    (18, 10, 2001, 0x9ea3dc, 0x9ea3c8), // Siena × Pistoiese
    (18, 10, 2001, 0x9ea3d4, 0x9ea41c), // Ternana × Empoli
    (25, 10, 2001, 0x9ea404, 0x9ea440), // Bari × Como
    (25, 10, 2001, 0x9ea450, 0x9ea464), // Cagliari × Ancona
    (25, 10, 2001, 0x9ea434, 0x9ea3d4), // Cittadella × Ternana
    (25, 10, 2001, 0x9ea448, 0x9ea42c), // Crotone × Cosenza
    (25, 10, 2001, 0x9ea41c, 0x9ea3dc), // Empoli × Siena
    (25, 10, 2001, 0x9ea424, 0x9ea40c), // Palermo × Genoa
    (25, 10, 2001, 0x9ea3c8, 0x9ea45c), // Pistoiese × Messina
    (25, 10, 2001, 0x9ea46c, 0x9ea414), // Reggina × Napoli
    (25, 10, 2001, 0x9ea3e4, 0x9ea3c0), // Salernitana × Vicenza
    (25, 10, 2001, 0x9ea3f0, 0x9ea3fc), // Sampdoria × Modena
    (2, 11, 2001, 0x9ea464, 0x9ea3c8), // Ancona × Pistoiese
    (2, 11, 2001, 0x9ea440, 0x9ea450), // Como × Cagliari
    (2, 11, 2001, 0x9ea42c, 0x9ea404), // Cosenza × Bari
    (2, 11, 2001, 0x9ea40c, 0x9ea448), // Genoa × Crotone
    (2, 11, 2001, 0x9ea45c, 0x9ea3f0), // Messina × Sampdoria
    (2, 11, 2001, 0x9ea3fc, 0x9ea434), // Modena × Cittadella
    (2, 11, 2001, 0x9ea414, 0x9ea424), // Napoli × Palermo
    (2, 11, 2001, 0x9ea3dc, 0x9ea46c), // Siena × Reggina
    (2, 11, 2001, 0x9ea3d4, 0x9ea3e4), // Ternana × Salernitana
    (2, 11, 2001, 0x9ea3c0, 0x9ea41c), // Vicenza × Empoli
    (9, 11, 2001, 0x9ea404, 0x9ea464), // Bari × Ancona
    (9, 11, 2001, 0x9ea450, 0x9ea3c8), // Cagliari × Pistoiese
    (9, 11, 2001, 0x9ea434, 0x9ea45c), // Cittadella × Messina
    (9, 11, 2001, 0x9ea448, 0x9ea3dc), // Crotone × Siena
    (9, 11, 2001, 0x9ea41c, 0x9ea440), // Empoli × Como
    (9, 11, 2001, 0x9ea3fc, 0x9ea40c), // Modena × Genoa
    (9, 11, 2001, 0x9ea414, 0x9ea42c), // Napoli × Cosenza
    (9, 11, 2001, 0x9ea424, 0x9ea3c0), // Palermo × Vicenza
    (9, 11, 2001, 0x9ea46c, 0x9ea3e4), // Reggina × Salernitana
    (9, 11, 2001, 0x9ea3f0, 0x9ea3d4), // Sampdoria × Ternana
    (16, 11, 2001, 0x9ea464, 0x9ea3fc), // Ancona × Modena
    (16, 11, 2001, 0x9ea440, 0x9ea3f0), // Como × Sampdoria
    (16, 11, 2001, 0x9ea42c, 0x9ea450), // Cosenza × Cagliari
    (16, 11, 2001, 0x9ea40c, 0x9ea41c), // Genoa × Empoli
    (16, 11, 2001, 0x9ea45c, 0x9ea404), // Messina × Bari
    (16, 11, 2001, 0x9ea3c8, 0x9ea414), // Pistoiese × Napoli
    (16, 11, 2001, 0x9ea3e4, 0x9ea424), // Salernitana × Palermo
    (16, 11, 2001, 0x9ea3dc, 0x9ea434), // Siena × Cittadella
    (16, 11, 2001, 0x9ea3d4, 0x9ea448), // Ternana × Crotone
    (16, 11, 2001, 0x9ea3c0, 0x9ea46c), // Vicenza × Reggina
    (23, 11, 2001, 0x9ea404, 0x9ea3e4), // Bari × Salernitana
    (23, 11, 2001, 0x9ea450, 0x9ea40c), // Cagliari × Genoa
    (23, 11, 2001, 0x9ea434, 0x9ea440), // Cittadella × Como
    (23, 11, 2001, 0x9ea448, 0x9ea3c8), // Crotone × Pistoiese
    (23, 11, 2001, 0x9ea41c, 0x9ea464), // Empoli × Ancona
    (23, 11, 2001, 0x9ea3fc, 0x9ea45c), // Modena × Messina
    (23, 11, 2001, 0x9ea414, 0x9ea3dc), // Napoli × Siena
    (23, 11, 2001, 0x9ea424, 0x9ea42c), // Palermo × Cosenza
    (23, 11, 2001, 0x9ea46c, 0x9ea3d4), // Reggina × Ternana
    (23, 11, 2001, 0x9ea3f0, 0x9ea3c0), // Sampdoria × Vicenza
    (6, 0, 2002, 0x9ea464, 0x9ea424), // Ancona × Palermo
    (6, 0, 2002, 0x9ea440, 0x9ea414), // Como × Napoli
    (6, 0, 2002, 0x9ea42c, 0x9ea41c), // Cosenza × Empoli
    (6, 0, 2002, 0x9ea40c, 0x9ea46c), // Genoa × Reggina
    (6, 0, 2002, 0x9ea45c, 0x9ea448), // Messina × Crotone
    (6, 0, 2002, 0x9ea3c8, 0x9ea434), // Pistoiese × Cittadella
    (6, 0, 2002, 0x9ea3e4, 0x9ea3fc), // Salernitana × Modena
    (6, 0, 2002, 0x9ea3dc, 0x9ea3f0), // Siena × Sampdoria
    (6, 0, 2002, 0x9ea3d4, 0x9ea404), // Ternana × Bari
    (6, 0, 2002, 0x9ea3c0, 0x9ea450), // Vicenza × Cagliari
    (13, 0, 2002, 0x9ea46c, 0x9ea464), // Reggina × Ancona
    (13, 0, 2002, 0x9ea45c, 0x9ea450), // Messina × Cagliari
    (13, 0, 2002, 0x9ea448, 0x9ea440), // Crotone × Como
    (13, 0, 2002, 0x9ea434, 0x9ea42c), // Cittadella × Cosenza
    (13, 0, 2002, 0x9ea424, 0x9ea41c), // Palermo × Empoli
    (13, 0, 2002, 0x9ea414, 0x9ea40c), // Napoli × Genoa
    (13, 0, 2002, 0x9ea404, 0x9ea3fc), // Bari × Modena
    (13, 0, 2002, 0x9ea3f0, 0x9ea3e4), // Sampdoria × Salernitana
    (13, 0, 2002, 0x9ea3dc, 0x9ea3d4), // Siena × Ternana
    (13, 0, 2002, 0x9ea3c8, 0x9ea3c0), // Pistoiese × Vicenza
    (20, 0, 2002, 0x9ea41c, 0x9ea404), // Empoli × Bari
    (20, 0, 2002, 0x9ea3e4, 0x9ea434), // Salernitana × Cittadella
    (20, 0, 2002, 0x9ea3fc, 0x9ea448), // Modena × Crotone
    (20, 0, 2002, 0x9ea3d4, 0x9ea45c), // Ternana × Messina
    (20, 0, 2002, 0x9ea464, 0x9ea414), // Ancona × Napoli
    (20, 0, 2002, 0x9ea450, 0x9ea424), // Cagliari × Palermo
    (20, 0, 2002, 0x9ea40c, 0x9ea3c8), // Genoa × Pistoiese
    (20, 0, 2002, 0x9ea440, 0x9ea46c), // Como × Reggina
    (20, 0, 2002, 0x9ea42c, 0x9ea3f0), // Cosenza × Sampdoria
    (20, 0, 2002, 0x9ea3c0, 0x9ea3dc), // Vicenza × Siena
    (3, 1, 2002, 0x9ea3dc, 0x9ea464), // Siena × Ancona
    (3, 1, 2002, 0x9ea404, 0x9ea450), // Bari × Cagliari
    (3, 1, 2002, 0x9ea3d4, 0x9ea42c), // Ternana × Cosenza
    (3, 1, 2002, 0x9ea45c, 0x9ea41c), // Messina × Empoli
    (3, 1, 2002, 0x9ea440, 0x9ea40c), // Como × Genoa
    (3, 1, 2002, 0x9ea414, 0x9ea3fc), // Napoli × Modena
    (3, 1, 2002, 0x9ea3c8, 0x9ea424), // Pistoiese × Palermo
    (3, 1, 2002, 0x9ea3f0, 0x9ea46c), // Sampdoria × Reggina
    (3, 1, 2002, 0x9ea448, 0x9ea3e4), // Crotone × Salernitana
    (3, 1, 2002, 0x9ea434, 0x9ea3c0), // Cittadella × Vicenza
    (10, 1, 2002, 0x9ea424, 0x9ea404), // Palermo × Bari
    (10, 1, 2002, 0x9ea46c, 0x9ea434), // Reggina × Cittadella
    (10, 1, 2002, 0x9ea3e4, 0x9ea440), // Salernitana × Como
    (10, 1, 2002, 0x9ea450, 0x9ea448), // Cagliari × Crotone
    (10, 1, 2002, 0x9ea464, 0x9ea45c), // Ancona × Messina
    (10, 1, 2002, 0x9ea3c0, 0x9ea414), // Vicenza × Napoli
    (10, 1, 2002, 0x9ea42c, 0x9ea3c8), // Cosenza × Pistoiese
    (10, 1, 2002, 0x9ea41c, 0x9ea3f0), // Empoli × Sampdoria
    (10, 1, 2002, 0x9ea40c, 0x9ea3dc), // Genoa × Siena
    (10, 1, 2002, 0x9ea3fc, 0x9ea3d4), // Modena × Ternana
    (17, 1, 2002, 0x9ea448, 0x9ea464), // Crotone × Ancona
    (17, 1, 2002, 0x9ea3f0, 0x9ea450), // Sampdoria × Cagliari
    (17, 1, 2002, 0x9ea440, 0x9ea42c), // Como × Cosenza
    (17, 1, 2002, 0x9ea414, 0x9ea41c), // Napoli × Empoli
    (17, 1, 2002, 0x9ea45c, 0x9ea40c), // Messina × Genoa
    (17, 1, 2002, 0x9ea3dc, 0x9ea3fc), // Siena × Modena
    (17, 1, 2002, 0x9ea434, 0x9ea424), // Cittadella × Palermo
    (17, 1, 2002, 0x9ea404, 0x9ea46c), // Bari × Reggina
    (17, 1, 2002, 0x9ea3c8, 0x9ea3e4), // Pistoiese × Salernitana
    (17, 1, 2002, 0x9ea3d4, 0x9ea3c0), // Ternana × Vicenza
    (24, 1, 2002, 0x9ea448, 0x9ea404), // Crotone × Bari
    (24, 1, 2002, 0x9ea41c, 0x9ea434), // Empoli × Cittadella
    (24, 1, 2002, 0x9ea464, 0x9ea440), // Ancona × Como
    (24, 1, 2002, 0x9ea46c, 0x9ea42c), // Reggina × Cosenza
    (24, 1, 2002, 0x9ea3c0, 0x9ea45c), // Vicenza × Messina
    (24, 1, 2002, 0x9ea450, 0x9ea414), // Cagliari × Napoli
    (24, 1, 2002, 0x9ea3fc, 0x9ea3c8), // Modena × Pistoiese
    (24, 1, 2002, 0x9ea424, 0x9ea3f0), // Palermo × Sampdoria
    (24, 1, 2002, 0x9ea3e4, 0x9ea3dc), // Salernitana × Siena
    (24, 1, 2002, 0x9ea40c, 0x9ea3d4), // Genoa × Ternana
    (3, 2, 2002, 0x9ea40c, 0x9ea464), // Genoa × Ancona
    (3, 2, 2002, 0x9ea3dc, 0x9ea450), // Siena × Cagliari
    (3, 2, 2002, 0x9ea434, 0x9ea448), // Cittadella × Crotone
    (3, 2, 2002, 0x9ea3c8, 0x9ea41c), // Pistoiese × Empoli
    (3, 2, 2002, 0x9ea440, 0x9ea3fc), // Como × Modena
    (3, 2, 2002, 0x9ea424, 0x9ea46c), // Palermo × Reggina
    (3, 2, 2002, 0x9ea45c, 0x9ea3e4), // Messina × Salernitana
    (3, 2, 2002, 0x9ea404, 0x9ea3f0), // Bari × Sampdoria
    (3, 2, 2002, 0x9ea414, 0x9ea3d4), // Napoli × Ternana
    (3, 2, 2002, 0x9ea42c, 0x9ea3c0), // Cosenza × Vicenza
    (10, 2, 2002, 0x9ea3f0, 0x9ea464), // Sampdoria × Ancona
    (10, 2, 2002, 0x9ea3d4, 0x9ea440), // Ternana × Como
    (10, 2, 2002, 0x9ea3fc, 0x9ea42c), // Modena × Cosenza
    (10, 2, 2002, 0x9ea450, 0x9ea41c), // Cagliari × Empoli
    (10, 2, 2002, 0x9ea434, 0x9ea40c), // Cittadella × Genoa
    (10, 2, 2002, 0x9ea3dc, 0x9ea45c), // Siena × Messina
    (10, 2, 2002, 0x9ea3e4, 0x9ea414), // Salernitana × Napoli
    (10, 2, 2002, 0x9ea448, 0x9ea424), // Crotone × Palermo
    (10, 2, 2002, 0x9ea46c, 0x9ea3c8), // Reggina × Pistoiese
    (10, 2, 2002, 0x9ea404, 0x9ea3c0), // Bari × Vicenza
    (17, 2, 2002, 0x9ea40c, 0x9ea404), // Genoa × Bari
    (17, 2, 2002, 0x9ea414, 0x9ea434), // Napoli × Cittadella
    (17, 2, 2002, 0x9ea3c0, 0x9ea448), // Vicenza × Crotone
    (17, 2, 2002, 0x9ea424, 0x9ea45c), // Palermo × Messina
    (17, 2, 2002, 0x9ea450, 0x9ea3fc), // Cagliari × Modena
    (17, 2, 2002, 0x9ea41c, 0x9ea46c), // Empoli × Reggina
    (17, 2, 2002, 0x9ea42c, 0x9ea3e4), // Cosenza × Salernitana
    (17, 2, 2002, 0x9ea3c8, 0x9ea3f0), // Pistoiese × Sampdoria
    (17, 2, 2002, 0x9ea440, 0x9ea3dc), // Como × Siena
    (17, 2, 2002, 0x9ea464, 0x9ea3d4), // Ancona × Ternana
    (30, 2, 2002, 0x9ea434, 0x9ea464), // Cittadella × Ancona
    (30, 2, 2002, 0x9ea46c, 0x9ea450), // Reggina × Cagliari
    (30, 2, 2002, 0x9ea45c, 0x9ea440), // Messina × Como
    (30, 2, 2002, 0x9ea3dc, 0x9ea42c), // Siena × Cosenza
    (30, 2, 2002, 0x9ea448, 0x9ea41c), // Crotone × Empoli
    (30, 2, 2002, 0x9ea3e4, 0x9ea40c), // Salernitana × Genoa
    (30, 2, 2002, 0x9ea3f0, 0x9ea414), // Sampdoria × Napoli
    (30, 2, 2002, 0x9ea3d4, 0x9ea424), // Ternana × Palermo
    (30, 2, 2002, 0x9ea404, 0x9ea3c8), // Bari × Pistoiese
    (30, 2, 2002, 0x9ea3fc, 0x9ea3c0), // Modena × Vicenza
    (7, 3, 2002, 0x9ea414, 0x9ea404), // Napoli × Bari
    (7, 3, 2002, 0x9ea450, 0x9ea434), // Cagliari × Cittadella
    (7, 3, 2002, 0x9ea3c0, 0x9ea440), // Vicenza × Como
    (7, 3, 2002, 0x9ea46c, 0x9ea448), // Reggina × Crotone
    (7, 3, 2002, 0x9ea3f0, 0x9ea40c), // Sampdoria × Genoa
    (7, 3, 2002, 0x9ea42c, 0x9ea45c), // Cosenza × Messina
    (7, 3, 2002, 0x9ea41c, 0x9ea3fc), // Empoli × Modena
    (7, 3, 2002, 0x9ea464, 0x9ea3e4), // Ancona × Salernitana
    (7, 3, 2002, 0x9ea424, 0x9ea3dc), // Palermo × Siena
    (7, 3, 2002, 0x9ea3c8, 0x9ea3d4), // Pistoiese × Ternana
    (14, 3, 2002, 0x9ea3dc, 0x9ea404), // Siena × Bari
    (14, 3, 2002, 0x9ea3d4, 0x9ea450), // Ternana × Cagliari
    (14, 3, 2002, 0x9ea40c, 0x9ea42c), // Genoa × Cosenza
    (14, 3, 2002, 0x9ea3e4, 0x9ea41c), // Salernitana × Empoli
    (14, 3, 2002, 0x9ea448, 0x9ea414), // Crotone × Napoli
    (14, 3, 2002, 0x9ea3fc, 0x9ea424), // Modena × Palermo
    (14, 3, 2002, 0x9ea440, 0x9ea3c8), // Como × Pistoiese
    (14, 3, 2002, 0x9ea45c, 0x9ea46c), // Messina × Reggina
    (14, 3, 2002, 0x9ea434, 0x9ea3f0), // Cittadella × Sampdoria
    (14, 3, 2002, 0x9ea464, 0x9ea3c0), // Ancona × Vicenza
    (21, 3, 2002, 0x9ea42c, 0x9ea464), // Cosenza × Ancona
    (21, 3, 2002, 0x9ea404, 0x9ea434), // Bari × Cittadella
    (21, 3, 2002, 0x9ea424, 0x9ea440), // Palermo × Como
    (21, 3, 2002, 0x9ea3f0, 0x9ea448), // Sampdoria × Crotone
    (21, 3, 2002, 0x9ea3c0, 0x9ea40c), // Vicenza × Genoa
    (21, 3, 2002, 0x9ea414, 0x9ea45c), // Napoli × Messina
    (21, 3, 2002, 0x9ea46c, 0x9ea3fc), // Reggina × Modena
    (21, 3, 2002, 0x9ea450, 0x9ea3e4), // Cagliari × Salernitana
    (21, 3, 2002, 0x9ea3c8, 0x9ea3dc), // Pistoiese × Siena
    (21, 3, 2002, 0x9ea41c, 0x9ea3d4), // Empoli × Ternana
    (28, 3, 2002, 0x9ea440, 0x9ea404), // Como × Bari
    (28, 3, 2002, 0x9ea464, 0x9ea450), // Ancona × Cagliari
    (28, 3, 2002, 0x9ea3d4, 0x9ea434), // Ternana × Cittadella
    (28, 3, 2002, 0x9ea42c, 0x9ea448), // Cosenza × Crotone
    (28, 3, 2002, 0x9ea3dc, 0x9ea41c), // Siena × Empoli
    (28, 3, 2002, 0x9ea40c, 0x9ea424), // Genoa × Palermo
    (28, 3, 2002, 0x9ea45c, 0x9ea3c8), // Messina × Pistoiese
    (28, 3, 2002, 0x9ea414, 0x9ea46c), // Napoli × Reggina
    (28, 3, 2002, 0x9ea3c0, 0x9ea3e4), // Vicenza × Salernitana
    (28, 3, 2002, 0x9ea3fc, 0x9ea3f0), // Modena × Sampdoria
    (5, 4, 2002, 0x9ea3c8, 0x9ea464), // Pistoiese × Ancona
    (5, 4, 2002, 0x9ea450, 0x9ea440), // Cagliari × Como
    (5, 4, 2002, 0x9ea404, 0x9ea42c), // Bari × Cosenza
    (5, 4, 2002, 0x9ea448, 0x9ea40c), // Crotone × Genoa
    (5, 4, 2002, 0x9ea3f0, 0x9ea45c), // Sampdoria × Messina
    (5, 4, 2002, 0x9ea434, 0x9ea3fc), // Cittadella × Modena
    (5, 4, 2002, 0x9ea424, 0x9ea414), // Palermo × Napoli
    (5, 4, 2002, 0x9ea46c, 0x9ea3dc), // Reggina × Siena
    (5, 4, 2002, 0x9ea3e4, 0x9ea3d4), // Salernitana × Ternana
    (5, 4, 2002, 0x9ea41c, 0x9ea3c0), // Empoli × Vicenza
    (12, 4, 2002, 0x9ea464, 0x9ea404), // Ancona × Bari
    (12, 4, 2002, 0x9ea3c8, 0x9ea450), // Pistoiese × Cagliari
    (12, 4, 2002, 0x9ea45c, 0x9ea434), // Messina × Cittadella
    (12, 4, 2002, 0x9ea3dc, 0x9ea448), // Siena × Crotone
    (12, 4, 2002, 0x9ea440, 0x9ea41c), // Como × Empoli
    (12, 4, 2002, 0x9ea40c, 0x9ea3fc), // Genoa × Modena
    (12, 4, 2002, 0x9ea42c, 0x9ea414), // Cosenza × Napoli
    (12, 4, 2002, 0x9ea3c0, 0x9ea424), // Vicenza × Palermo
    (12, 4, 2002, 0x9ea3e4, 0x9ea46c), // Salernitana × Reggina
    (12, 4, 2002, 0x9ea3d4, 0x9ea3f0), // Ternana × Sampdoria
    (19, 4, 2002, 0x9ea3fc, 0x9ea464), // Modena × Ancona
    (19, 4, 2002, 0x9ea3f0, 0x9ea440), // Sampdoria × Como
    (19, 4, 2002, 0x9ea450, 0x9ea42c), // Cagliari × Cosenza
    (19, 4, 2002, 0x9ea41c, 0x9ea40c), // Empoli × Genoa
    (19, 4, 2002, 0x9ea404, 0x9ea45c), // Bari × Messina
    (19, 4, 2002, 0x9ea414, 0x9ea3c8), // Napoli × Pistoiese
    (19, 4, 2002, 0x9ea424, 0x9ea3e4), // Palermo × Salernitana
    (19, 4, 2002, 0x9ea434, 0x9ea3dc), // Cittadella × Siena
    (19, 4, 2002, 0x9ea448, 0x9ea3d4), // Crotone × Ternana
    (19, 4, 2002, 0x9ea46c, 0x9ea3c0), // Reggina × Vicenza
    (26, 4, 2002, 0x9ea3e4, 0x9ea404), // Salernitana × Bari
    (26, 4, 2002, 0x9ea40c, 0x9ea450), // Genoa × Cagliari
    (26, 4, 2002, 0x9ea440, 0x9ea434), // Como × Cittadella
    (26, 4, 2002, 0x9ea3c8, 0x9ea448), // Pistoiese × Crotone
    (26, 4, 2002, 0x9ea464, 0x9ea41c), // Ancona × Empoli
    (26, 4, 2002, 0x9ea45c, 0x9ea3fc), // Messina × Modena
    (26, 4, 2002, 0x9ea3dc, 0x9ea414), // Siena × Napoli
    (26, 4, 2002, 0x9ea42c, 0x9ea424), // Cosenza × Palermo
    (26, 4, 2002, 0x9ea3d4, 0x9ea46c), // Ternana × Reggina
    (26, 4, 2002, 0x9ea3c0, 0x9ea3f0), // Vicenza × Sampdoria
    (2, 5, 2002, 0x9ea424, 0x9ea464), // Palermo × Ancona
    (2, 5, 2002, 0x9ea414, 0x9ea440), // Napoli × Como
    (2, 5, 2002, 0x9ea41c, 0x9ea42c), // Empoli × Cosenza
    (2, 5, 2002, 0x9ea46c, 0x9ea40c), // Reggina × Genoa
    (2, 5, 2002, 0x9ea448, 0x9ea45c), // Crotone × Messina
    (2, 5, 2002, 0x9ea434, 0x9ea3c8), // Cittadella × Pistoiese
    (2, 5, 2002, 0x9ea3fc, 0x9ea3e4), // Modena × Salernitana
    (2, 5, 2002, 0x9ea3f0, 0x9ea3dc), // Sampdoria × Siena
    (2, 5, 2002, 0x9ea404, 0x9ea3d4), // Bari × Ternana
    (2, 5, 2002, 0x9ea450, 0x9ea3c0), // Cagliari × Vicenza
];

/// In the rehearsal (the comparer) the copy writes the rules and does not call virtual +0x3c (the round table, which
/// allocates).
///
/// [pt] No ensaio (o comparador) a cópia grava as regras e não chama a virtual +0x3c (a tabela de rodadas, que aloca).
pub(crate) static mut ITA_SER_A_RULES_DRY_RUN: bool = false;

/// Writes the rule fields (offset, size, value) into the league and builds the main-stage round table via virtual +0x3c
/// into +0xba (skipped in dry run); returns 1.
///
/// [pt] Grava as regras na liga (deslocamento, tamanho 1/2/4, valor), na ordem da lista, e monta a tabela de rodadas da
/// fase principal: a virtual +0x3c (create_schedule: thiscall(liga, −1, &+0xa9, &+0x3a, 0), ret 0x10) com o
/// resultado em +0xba. Devolve 1.
pub(crate) unsafe fn ita_ser_a_apply_league_rules(this: League, written: &[(usize, u8, u32)]) -> u32 {
    for &(d, t, v) in written {
        match t {
            1 => *((this.addr() + d) as *mut u8) = v as u8,
            2 => write_u16(this.addr() + d, v as u16),
            _ => write_u32(this.addr() + d, v),
        }
    }
    if !ITA_SER_A_RULES_DRY_RUN {
        let rounds: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + 0x3c));
        let t = rounds(this.addr(), 0xffff_ffff, this.addr() + league::ROUND_COUNT, this.addr() + league::STAGE_ID, 0);
        this.set_round_dates(t);
    }
    1
}

/// The rules common to the four Italian divisions.
///
/// 1 point for a draw (`+0xc3`), category 2 for the matches (`+0xc4`), `+0xab` and `+0xdb` = 0 (the words copied into
/// the matches), national clubs (`+0x42` = 1), the 1st tie-breaker 1 (`+0xc5`) and the 2nd 2 (`+0xc6`), the fixed round
/// order in the grid (bit 0x40 of `+0xd9`, with the low byte left as is), 7 substitutes on the bench (`+0x49`) and 3
/// substitutions (`+0x4a`).
///
/// [pt] As regras comuns às quatro: 1 ponto pelo empate (+0xc3), categoria 2 nas partidas (+0xc4), +0xab e +0xdb = 0 (as
/// palavras copiadas nas partidas), clubes nacionais (+0x42 = 1), o 1º desempate 1 (+0xc5) e o 2º 2 (+0xc6), a ordem
/// fixa das rodadas na grade (bit 0x40 de +0xd9, com o byte de baixo como está), 7 reservas (+0x49) e 3 substituições
/// (+0x4a).
pub(crate) unsafe fn italian_common_rules(this: League) -> Vec<(usize, u8, u32)> {
    let d9 = (this.league_flags() as u8) | 0x40;
    vec![(0xc3, 1, 1), (0xc4, 1, 2), (0xab, 2, 0), (0xdb, 2, 0), (0x42, 1, 1), (0xc5, 1, 1), (0xc6, 1, 2), (0xd9, 1, d9 as u32), (0x49, 1, 7), (0x4a, 1, 3)]
}

/// Serie A (18 clubs): 2 rounds, 3 points for a win and 1 for a draw, nobody goes up, the bottom 4 go down to B; 7
/// substitutes and 3 substitutions; tie-breaks 1, 2 and 4 [codes; inference: goal difference, goals for, head to head];
/// a points tie across a place border goes to a tie-break match (+0xe6); fixed round order
/// Original: `ItaSerA::set_up_rules` (0x641640, ita_ser_a.cpp, thiscall, ret 0).
///
/// [pt] { 0x641640 } (thiscall, ret 0, ita_ser_a.cpp): `ItaSerA::set_up_rules()`. A Série A: as comuns, 2 turnos, 3
/// pontos, os desempates 3º = 4 e 4º = 0 (+0xc7, +0xc8), ninguém sobe, nenhum play-off, 4 caem direto (+0xc1),
/// nenhuma liga acima (+0x1c = −1), a Série B abaixo (+0x20 = [0x9cf574]), +0x52 = 2 [sem nome] e +0xe6 = 1 (o
/// empate em pontos na fronteira de uma vaga não se decide pelos critérios: vai ao jogo-desempate, ver 0x642880).
pub(crate) unsafe extern "thiscall" fn ita_ser_a_set_up_rules(this: usize) -> u32 {
    crate::comparison::branch("cqi_6 serie a rules");
    ita_ser_a_apply_league_rules(League(this), &serie_a_rules(this))
}

/// Rule writes for Serie A (common rules plus rounds, win points, 4 relegated, lower division Serie B).
pub(crate) unsafe fn serie_a_rules(this: usize) -> Vec<(usize, u8, u32)> {
    let mut v = italian_common_rules(League(this));
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3), (0xc7, 1, 4), (0xc8, 1, 0), (0xbe, 1, 0), (0xbf, 1, 0), (0xc0, 1, 0)]);
    v.push((0xc1, 1, 4)); // cqi6: a Série A rebaixa 4
    v.extend_from_slice(&[(0x1c, 4, 0xffff_ffff), (0x52, 1, 2), (0x20, 4, read_usize(KEY_CLUB_COMP_ITALIAN_SERIE_B) as u32), (0xe6, 4, 1)]);
    v
}

/// Rule writes for Serie B (common rules plus rounds, promotion/relegation and links to Serie A and C1).
pub(crate) unsafe fn serie_b_rules(this: usize) -> Vec<(usize, u8, u32)> {
    let mut v = italian_common_rules(League(this));
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3)]); // cqi6: a Série B joga 2 turnos
    v.extend_from_slice(&[(0xc7, 1, 4), (0xc8, 1, 0), (0xbe, 1, 4), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 4), (0x1c, 4, read_usize(KEY_CLUB_COMP_ITALIAN_SERIE_A) as u32), (0x20, 4, read_usize(KEY_CLUB_COMP_ITALIAN_SERIE_C1_A) as u32), (0x52, 1, 2), (0xe6, 4, 1)]);
    v
}

/// Rule writes for Serie C1/A.
pub(crate) unsafe fn serie_c1a_rules(this: usize) -> Vec<(usize, u8, u32)> {
    let mut v = italian_common_rules(League(this));
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3)]); // cqi6: the C1/A gives 3 points per win
    v.extend_from_slice(&[(0xc7, 1, 0), (0xbe, 1, 1), (0xbf, 1, 4), (0xc0, 1, 4), (0xc1, 1, 1), (0x1c, 4, read_usize(KEY_CLUB_COMP_ITALIAN_SERIE_B) as u32), (0x20, 4, read_usize(KEY_CLUB_COMP_ITALIAN_SERIE_C1_B) as u32)]);
    v
}

/// Rule writes for Serie C1/B (lower division C2/A only when that league is selected).
pub(crate) unsafe fn serie_c1b_rules(this: usize) -> Vec<(usize, u8, u32)> {
    let mut v = italian_common_rules(League(this));
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3), (0xc7, 1, 0), (0xbe, 1, 1), (0xbf, 1, 4), (0xc0, 1, 4), (0xc1, 1, 1), (0x1c, 4, read_usize(KEY_CLUB_COMP_ITALIAN_SERIE_C1_A) as u32)]);
    let nation = Nation(read_usize(read_usize(this + 4) + 0x5d));
    let below = if nation.league_selected() & 4 != 0 { read_usize(KEY_CLUB_COMP_ITALIAN_SERIE_C2_A) as u32 } else { 0xffff_ffff };
    v.push((0x20, 4, below));
    v
}

/// Dry-run comparer: runs the original, restores +0x1c..+0xec, runs the port without the virtual call and compares
/// every written field and the return value; keeps the original's state.
///
/// [pt] O comparador das quatro (ensaio, como o da cqi_4): a original roda inteira; a faixa +0x1c..+0xec volta e a cópia
/// grava as regras sem a virtual +0x3c; os campos que a cópia grava saem iguais aos da original, e a devolução 1; no
/// fim fica a faixa da original.
pub(crate) unsafe fn ita_ser_a_compare_league_rules(name: &str, this: League, written: Vec<(usize, u8, u32)>, original: impl FnOnce() -> u32, copy: impl FnOnce() -> u32) -> u32 {
    let band = [(this.addr() + league::UPPER_DIVISION_ID, 0xd0)];
    let before = crate::comparison::MemorySnapshot::read(&band);
    let r = original();
    let mo = crate::comparison::MemorySnapshot::read(&band);
    before.write_back();
    ITA_SER_A_RULES_DRY_RUN = true;
    let rr = crate::rng::pause_trace(copy);
    ITA_SER_A_RULES_DRY_RUN = false;
    let mr = crate::comparison::MemorySnapshot::read(&band);
    mo.write_back();
    let field = |m: &crate::comparison::MemorySnapshot, d: usize, t: u8| -> u32 {
        let b = &m.0[0].1[d - 0x1c..];
        match t {
            1 => b[0] as u32,
            2 => u16::from_le_bytes([b[0], b[1]]) as u32,
            _ => u32::from_le_bytes([b[0], b[1], b[2], b[3]]),
        }
    };
    let difs: Vec<(usize, u32, u32)> = written.iter().filter(|e| field(&mo, e.0, e.1) != field(&mr, e.0, e.1)).map(|e| (e.0, field(&mo, e.0, e.1), field(&mr, e.0, e.1))).collect();
    SCOREBOARD.record(difs.is_empty() && r == rr, || format!("{}({:#x}) original={} rust={} (offset, original, rust)={:x?}", name, this.addr(), r, rr, difs));
    r
}

/// Shared table body: sets the club count, allocates the table and fills the rows with the division's clubs in database
/// order; returns whether it filled up.
///
/// +0x3e = `n` clubs; the table (malloc `exe::crt_malloc` of n × 0x3b; when out of memory: the error box of source file
/// `arquivo` at line `linha` and None); the clubs of [0xae23b4] (0x245 bytes, [0xae235c] of them) whose competition
/// (+0x57, the id at +0) is that of global `g`, in database order, each in the next row (League::init_table_row
/// `init_table_row`: thiscall(league, row, club, 0, 0), ret 0x10), until full.
///
/// Returns whether it filled up.
///
/// [pt] O corpo comum: +0x3e = `n` clubes; a tabela (malloc 0x944e46 de n × 0x3b; sem memória: a caixa do fonte
/// `arquivo` na linha `linha` e None); os clubes de [0xae23b4] (0x245 bytes, [0xae235c] deles) cuja competição
/// (+0x57, o id no +0) é a do global `g`, na ordem do banco, cada um na linha seguinte (League::init_table_row
/// 0x687430: thiscall(liga, linha, clube, 0, 0), ret 0x10), até encher. Devolve se encheu.
pub(crate) unsafe fn ita_ser_a_fill_league_table(this: League, n: i16, file: usize, row: u32, g: usize) -> Option<bool> {
    this.set_club_count(n);
    let t = exe::crt_malloc((n as i32 as u32).wrapping_mul(0x3b));
    this.set_table(t);
    if t == 0 {
        show_error_at(file, row);
        return None;
    }
    let mut k: u8 = 0;
    let mut i: i16 = 0;
    let mut club = read_usize(G_CLUB_TABLE);
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        let comp = read_usize(club + club::DIVISION);
        if comp != 0 && read_usize(comp) == read_usize(g) {
            let row = k; // cqj4: row k receives the k-th club of the division
            // simplification: the original pushes the row in eax with the high bits of the competition id; 0x687430 only reads
            // o byte.
            exe::league_init_table_row(this, row as u32, club, 0, 0); // cqj4: the rows without a mark
            k = k.wrapping_add(1);
            if k as i8 as i16 == this.club_count() {
                break;
            }
        }
        i = i.wrapping_add(1);
        club = club.wrapping_add(club::SIZE);
    }
    Some(k as i8 as i16 == this.club_count())
}

/// Sets the prize fund (+0x28 = calc_prize_fund(cota)) and +0xe1 = 1 (paid by table at league end).
///
/// [pt] O fundo de prêmios (+0x28 = League::calc_prize_fund 0x682f70: thiscall(liga, cota), ret 4) e +0xe1 = 1 (o fundo
/// é pago pela tabela no fim da liga, 0x682e10).
pub(crate) unsafe fn set_prize_fund(this: League, quota: i32) {
    let f = exe::league_calc_prize_fund(this, quota);
    this.set_prize_fund(f);
    this.set_prize_mode(1);
}

/// Creates the Serie A table: 18 clubs; the prize fund starts at 250,000 per place (paid by table position when the
/// league ends); TV pays 5,000,000 to each club.
///
/// `ItaSerA::create_table()`: 18 clubs (`[0x9cf570]`); once full, the fund by the 250,000 share and the TV money of
/// 5,000,000 to each club (League::pay_tv_rights `0x683010`: thiscall(league, value, 0), ret 8); returns 1 (0 if it
/// could not fill). Source `0x9ea2a8`, line `0x253`.
/// Original: `ItaSerA::create_table` (0x641510, ita_ser_a.cpp, thiscall, ret 0).
///
/// [pt] { 0x641510 } (thiscall, ret 0, ita_ser_a.cpp): `ItaSerA::create_table()`: 18 clubes ([0x9cf570]); cheia, o fundo
/// pela cota de 250.000 e a TV de 5.000.000 a cada clube (League::pay_tv_rights 0x683010: thiscall(liga, valor, 0),
/// ret 8); devolve 1 (0 sem encher). Fonte 0x9ea2a8, linha 0x253.
pub(crate) unsafe extern "thiscall" fn ita_ser_a_create_table(this: usize) -> u32 {
    match ita_ser_a_fill_league_table(League(this), 0x12, 0x9ea2a8, 0x253, KEY_CLUB_COMP_ITALIAN_SERIE_A) {
        Some(true) => {}
        _ => return 0,
    }
    crate::comparison::branch("cqj_4 serie a fixtures");
    set_prize_fund(League(this), 0x3d090); // cqj4: the Serie A quota
    exe::league_pay_tv_rights(this, 0x4c4b40, 0); // cqj4: the Serie A TV money
    1
}

/// The comparer of the four table routines: the original routine allocates the table (and Serie A's also pays the TV
/// money, with news), so only it runs.
///
/// The plan, made beforehand: the division's clubs in database order (up to `n`). Afterwards it checks: `+0x3e` = `n`,
/// the return value = 1 if the plan was full, the rows holding the plan's clubs in order and, with `cota` and a full
/// table, `+0xe1` = 1 and `+0x28` = the fund for that share.
///
/// [pt] O comparador das quatro: a original aloca a tabela (e a da A paga a TV, com notícia): só ela. O plano, feito
/// antes: os clubes da divisão na ordem do banco (até `n`). Depois: +0x3e = `n`, a devolução = 1 se o plano encheu,
/// as linhas com os clubes do plano na ordem e, com `cota`, cheia, +0xe1 = 1 e +0x28 = o fundo da cota.
pub(crate) unsafe fn ita_ser_a_compare_league_table(name: &str, this: League, n: i16, g: usize, quota: Option<i32>, original: impl FnOnce() -> u32) -> u32 {
    let mut plan: Vec<usize> = Vec::new();
    let mut i: i32 = 0;
    while i < read_usize(G_CLUB_COUNT) as i32 && (plan.len() as i16) < n {
        let club = Club(read_usize(G_CLUB_TABLE) + (i as usize) * club::SIZE);
        let comp = club.division().addr();
        if comp != 0 && read_usize(comp) == read_usize(g) {
            plan.push(club.addr());
        }
        i += 1;
    }
    let r = original();
    let t = this.table();
    let mut same = r == (plan.len() as i16 == n) as u32 && this.club_count() == n && t != 0;
    let mut found: Vec<usize> = Vec::new();
    if same {
        found = (0..plan.len()).map(|k| read_usize(t + k * 0x3b)).collect();
        same = found == plan;
        if let (1, Some(c)) = (r, quota) {
            same = same && this.prize_mode() == 1 && this.prize_fund() == exe::league_calc_prize_fund(this, c);
        }
    }
    SCOREBOARD.record(same, || format!("{}({:#x}) original={} plan={:x?} table={:x?}", name, this.addr(), r, plan, found));
    r
}

