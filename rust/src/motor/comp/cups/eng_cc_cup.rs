use super::*;

/// Writes one cup round (a 0x68-byte record, docs/.md §2.4).
///
/// The draw (day, month with 0 = January, years to add to the cup year +0x40, weekday or −1) goes through `cup_set_round_draw_date`; the
/// match (the same, plus the leg at +6 and the venue at +0x13) through `cup_set_round_match_date`.
///
/// Then the fields: +7 the round name, +9 the index, +0xb the draw bits, +0xd and +0xf the tie-breakers (single match
/// and two legs), +0x17 the category copied into each match, +0x18/+0x1a/+0x1c/+0x1e teams, matches, new teams and
/// entrants already in, +0x20/+0x21/+0x22 replay, matches per tie and days between them, +0x5c/+0x60/+0x64 the prize
/// money (reaching, winning, losing).
///
/// [pt] Uma rodada de copa (registro de 0x68 bytes, docs/.md §2.4): o sorteio (dia, mês com 0 =
/// janeiro, anos a somar ao ano da copa +0x40, dia da semana ou −1) pela 0x521e60; o jogo (o mesmo, mais o turno em +6
/// e o mando em +0x13) pela 0x521eb0; depois os campos: +7 o nome da rodada, +9 o índice, +0xb os bits do sorteio, +0xd
/// e +0xf os desempates (jogo único e ida e volta), +0x17 a categoria copiada em cada jogo, +0x18/+0x1a/+0x1c/+0x1e
/// times, jogos, times novos e inscritos que já entraram, +0x20/+0x21/+0x22 replay, jogos por confronto e dias entre
/// eles, +0x5c/+0x60/+0x64 os prêmios (chegar, vencer, perder).
#[derive(Clone, Copy)]
pub(crate) struct EngCcCupCupRound {
    pub(crate) draw: (u32, u32, u32, u32),
    pub(crate) game: (u32, u32, u32, u32, u32, u32),
    pub(crate) code: u16,
    pub(crate) bits: (u16, u16, u16),
    pub(crate) category: u8,
    pub(crate) teams: (u16, u16, u16, u16),
    pub(crate) tie: (u8, u8, u8),
    pub(crate) prizes: (u32, u32, u32),
}

/// A linha da tabela: (sorteio), (jogo), nome, (bits do sorteio, desempate, desempate de ida e volta), categoria,
/// (times, jogos, novos, inscritos), (replay, jogos por confronto, dias entre eles), (prêmios).
pub(crate) const fn cql10r(sorteio: (u32, u32, u32, u32), jogo: (u32, u32, u32, u32, u32, u32), codigo: u16, bits: (u16, u16, u16), categoria: u8,
    times: (u16, u16, u16, u16), confronto: (u8, u8, u8), premios: (u32, u32, u32)) -> EngCcCupCupRound {
    EngCcCupCupRound { draw: sorteio, game: jogo, code: codigo, bits, category: categoria, teams: times, tie: confronto, prizes: premios }
}

/// Writes round i of a cup round table: draw and match dates (`cup_set_round_draw_date`, `cup_set_round_match_date`) and the round fields (code, bits,
/// category, team counts, tie flags, prizes).
///
/// [pt] Grava a rodada `i` da tabela `t` (ano-base `ano`, a palavra +0x40 da copa com sinal): as duas datas (0x521e60 e
/// 0x521eb0, cdecl, 7 e 9 argumentos; add esp, 0x1c / 0x24) e os campos.
pub(crate) unsafe fn eng_cc_cup_write_cup_round(t: usize, i: u32, r: &EngCcCupCupRound, year: u32) {
    exe::cup_set_round_draw_date(t, i, r.draw.0, r.draw.1, r.draw.2, r.draw.3, year);
    exe::cup_set_round_match_date(t, i, r.game.0, r.game.1, r.game.2, r.game.3, r.game.4, year, r.game.5);
    let e = CupRound(t + i as usize * 0x68);
    let w = |o: usize, v: u16| write_u16(e.addr() + o, v);
    let d = |o: usize, v: u32| write_u32(e.addr() + o, v);
    w(7, r.code);
    w(9, i as u16);
    w(0xb, r.bits.0);
    w(0xd, r.bits.1);
    w(0xf, r.bits.2);
    e.set_match_category(r.category);
    w(0x18, r.teams.0);
    w(0x1a, r.teams.1);
    w(0x1c, r.teams.2);
    w(0x1e, r.teams.3);
    e.set_single_leg_flag(r.tie.0);
    e.set_legs(r.tie.1);
    e.set_leg_gap_days(r.tie.2);
    d(0x5c, r.prizes.0);
    d(0x60, r.prizes.1);
    d(0x64, r.prizes.2);
}

/// Common body of virtual `+0x3c` (`create_round_table(fase, &rodadas, &extra, &saida)`, thiscall, ret 0x10) for
/// single-stage cups.
///
/// Outside stage −1 (the byte), returns 0 without writing anything. At −1: `saida` (if requested) = 0, `rodadas` = the
/// table size, `extra` = 0; allocates the table (`0x944e46` of 0x68 × rounds; out of memory, the box `linha` of
/// `arquivo` and returns 0) and fills each round; returns the table.
///
/// [pt] O corpo comum da virtual +0x3c (`create_round_table(fase, &rodadas, &extra, &saida)`, thiscall, ret 0x10) das copas
/// de uma fase só: fora da fase −1 (o byte), devolve 0 sem gravar nada. Na −1: saida (se pedida) = 0, rodadas = o
/// tamanho da tabela, extra = 0; a tabela (0x944e46 de 0x68 × rodadas; sem memória, a caixa `linha` de `arquivo` e
/// devolve 0) com cada rodada; devolve a tabela.
pub(crate) unsafe fn eng_cc_cup_create_single_stage_round_table(this: Cup, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32, table: &[EngCcCupCupRound], file: usize, row: u32, name: &'static str) -> usize {
    if stage as u8 != 0xff {
        return 0;
    }
    if !out.is_null() {
        *out = 0;
    }
    *rounds = table.len() as i16;
    *extra = 0;
    let t = exe::crt_malloc(table.len() as u32 * 0x68);
    if t == 0 {
        show_error_at(file, row);
        return 0;
    }
    crate::comparison::branch(name);
    let year = this.year() as i32 as u32;
    for (i, r) in table.iter().enumerate() {
        eng_cc_cup_write_cup_round(t, i as u32, r, year);
    }
    t
}

/// Collects the bytes the original writes in each 0x68-byte round (+0..+0x10, +0x13..+0x22, +0x5c..+0x67).
///
/// [pt] Os bytes que a original grava em cada rodada (o resto da memória alocada fica como veio): +0..+0x10 (as datas e os
/// campos até o desempate de ida e volta), +0x13..+0x22 (o mando até os dias entre os jogos) e +0x5c..+0x67 (os prêmios).
pub(crate) unsafe fn eng_cc_cup_written_round_bytes(t: usize, n: i16) -> Vec<u8> {
    let mut v = Vec::new();
    for r in 0..n.max(0) as usize {
        let e = t + r * 0x68;
        for &(a, b) in &[(0usize, 0x11usize), (0x13, 0x23), (0x5c, 0x68)] {
            v.extend_from_slice(core::slice::from_raw_parts((e + a) as *const u8, b - a));
        }
    }
    v
}

/// A Copa da Liga inglesa (EngCcCup): sete rodadas de agosto a março do ano seguinte; as três primeiras dependem de
/// +0xb2 (os clubes nas copas europeias, que entram mais tarde) e vêm à parte (`league_cup_rounds`).
pub(crate) const LEAGUE_CUP_ROUNDS: [EngCcCupCupRound; 7] = [
    cql10r((0x17, 6, 0, 5), (0x16, 7, 0, 1, 2, 0), 0xa, (0, 0x403, 0), 4, (0, 0, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
    cql10r((0x17, 7, 0, 2), (0x14, 8, 0, 2, 2, 0), 0x14, (0, 0x403, 0), 4, (0, 0, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)), // cql10: the 2nd round of the League Cup
    cql10r((0x1c, 8, 0, 3), (0x1c, 9, 0, 2, 2, 0), 0x1e, (0, 3, 0), 4, (0x20, 0x10, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
    cql10r((0x1d, 9, 0, 3), (0xb, 0xa, 0, 2, 2, 0), 0x28, (0, 0x203, 0), 4, (0x10, 8, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
    cql10r((0xc, 0xa, 0, 3), (2, 0xb, 0, 2, 2, 0), 0x78, (0, 0x203, 0), 6, (8, 4, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
    cql10r((3, 0xb, 0, 3), (0x1b, 0, 1, 2, 2, 0), 0x82, (0, 0x4200, 0x4003), 6, (4, 2, 0, 0), (0, 2, 0x15), (0, 0xffff_ffff, 0)),
    cql10r((0x12, 1, 1, 3), (1, 3, 1, 6, 1, 4), 0x96, (0, 3, 0), 6, (2, 1, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
];

/// Builds the English League Cup round table, with team, match and entrant counts adjusted by `b` = +0xb2 (signed byte;
/// the number of European entrants; arithmetic done in 16 bits as in the original).
///
/// 1st round: 2b + 0x38 teams, b + 0x1c matches, 2b + 0x38 new entrants. 2nd round: (0x20 − b) × 2 teams, 0x20 − b
/// matches, (0xc − b) × 3 new entrants and 2b + 0x38 entrants carried over. 3rd round: b new entrants and 0x5c − b
/// entrants carried over.
///
/// [pt] A tabela da Copa da Liga com `b` = +0xb2 (byte com sinal; as contas em 16 bits como a original): 1ª rodada 2b + 0x38
/// times, b + 0x1c jogos, 2b + 0x38 novos; 2ª rodada (0x20 − b) × 2 times, 0x20 − b jogos, (0xc − b) × 3 novos e 2b +
/// 0x38 inscritos; 3ª rodada b novos e 0x5c − b inscritos.
pub(crate) fn league_cup_rounds(b: i16) -> [EngCcCupCupRound; 7] {
    let mut t = LEAGUE_CUP_ROUNDS;
    let x = |v: i16| v as u16;
    t[0].teams = (x(b.wrapping_mul(2).wrapping_add(0x38)), x(b.wrapping_add(0x1c)), x(b.wrapping_mul(2).wrapping_add(0x38)), 0);
    t[1].teams = (x(0x20i16.wrapping_sub(b).wrapping_mul(2)), x(0x20i16.wrapping_sub(b)), x(0xci16.wrapping_sub(b).wrapping_mul(3)), x(b.wrapping_mul(2).wrapping_add(0x38)));
    t[2].teams.2 = x(b);
    t[2].teams.3 = x(0x5ci16.wrapping_sub(b));
    t
}

/// rule: the English League Cup has 7 single-leg rounds, except the two-legged semi-final (21 days apart): 1st round
/// drawn on 23/7 and played on 22/8; 2nd draw 23/8, match 20/9; 3rd 28/9 and 28/10; 4th 29/10 and 11/11; quarter-finals
/// 12/11 and 2/12; semi-final 3/12 and 27/1 of the next year; final drawn 18/2, played on 1/4 at a neutral ground
/// [inference: venue 4], dates moved to each one's weekday; clubs in Europe (+0xb2) only enter in the 3rd round (1st
/// round 2b + 56 teams, 2nd (32 − b) × 2, (12 − b) × 3 new entrants; the b enter in the 3rd); category 4 up to the 4th
/// round and 6 after; no prize per round (−1)
/// Original: `EngCcCup::create_round_table` (0x56c570, eng_cc_cup.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x56c570 } (thiscall, ret 0x10, eng_cc_cup.cpp; a virtual +0x3c da Copa da Liga): `EngCcCup::create_round_table(
/// fase, &rodadas, &extra, &saida)`: na fase −1, as 7 rodadas (`league_cup_rounds` com o +0xb2); a caixa da
/// linha 0xa2 sem memória.
pub(crate) unsafe extern "thiscall" fn eng_cc_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let t = league_cup_rounds(read_i8(this + eng_cc_cup::EUROPEAN_ENTRANT_COUNT) as i16);
    eng_cc_cup_create_single_stage_round_table(Cup(this), stage, rounds, extra, out, &t, 0x9c4224, 0xa2, "cql_10 tabela da copa da liga")
}

/// The English FA Cup (EngFaCup): nine rounds from October to May of the following year, with replays (`+0x20` = 1) up
/// to the quarter-finals.
///
/// Rounds 2 to 5 depend on `+0xb3` (exemptions), `+0xb4` (the choice of entry in the 5th round) and `+0x3e` (the
/// entrants), and are built separately (`fa_cup_rounds`).
///
/// The original routine also writes `+0x13` = 0 in rounds 0 to 6, which the fixture date had already left with home
/// advantage 0.
///
/// [pt] A Copa da Inglaterra (EngFaCup): nove rodadas de outubro a maio do ano seguinte, com replay (+0x20 = 1) até as
/// quartas; as rodadas 2 a 5 dependem de +0xb3 (isenções) e +0xb4 (a escolha da entrada na 5ª rodada) e do +0x3e (os
/// inscritos) e vêm à parte (`fa_cup_rounds`). A original grava ainda +0x13 = 0 nas rodadas 0 a 6,
/// que a data do jogo já deixou com o mando 0.
pub(crate) const FA_CUP_ROUNDS: [EngCcCupCupRound; 9] = [
    cql10r((9, 9, 0, 5), (0x10, 9, 0, 5, 1, 0), 0xdc, (0, 0, 3), 4, (0x38, 0x1c, 0x38, 0), (1, 0, 0xa), (0, 0x2710, 0)),
    cql10r((0x11, 9, 0, 6), (0x12, 0xa, 0, 5, 1, 0), 0xa, (0, 0, 3), 4, (0x50, 0x28, 0x34, 0x38), (1, 1, 0xa), (0, 0x4e20, 0)),
    cql10r((0x13, 0xa, 0, 6), (9, 0xb, 0, 5, 1, 0), 0x14, (0, 0, 3), 4, (0, 0, 0, 0), (1, 1, 0xa), (0, 0x4e20, 0)), // cql10: the 2nd round of the FA Cup
    cql10r((0xa, 0xb, 0, 6), (6, 0, 1, 5, 1, 0), 0x1e, (0, 0, 3), 4, (0, 0, 0, 0), (1, 1, 0xa), (0, 0x7530, 0)),
    cql10r((7, 0, 1, 6), (0x1b, 0, 1, 5, 1, 0), 0x28, (0, 0, 3), 4, (0, 0, 0, 0), (1, 1, 0xa), (0, 0xc350, 0)),
    cql10r((0x1c, 0, 1, 6), (0x11, 1, 1, 5, 1, 0), 0x32, (0, 0, 3), 4, (0x10, 8, 0, 0), (1, 1, 0xa), (0, 0x124f8, 0)),
    cql10r((0x12, 1, 1, 6), (0xa, 2, 1, 5, 1, 0), 0x78, (0, 0, 3), 6, (8, 4, 0, 0), (1, 1, 0xa), (0, 0x249f0, 0)),
    cql10r((0xb, 2, 1, 6), (8, 3, 1, 6, 1, 0x2a), 0x82, (0, 3, 0), 6, (4, 2, 0, 0), (0, 1, 0xa), (0, 0x61a80, 0)),
    cql10r((9, 3, 1, 0), (0xc, 4, 1, 5, 1, 4), 0x96, (0, 3, 0), 6, (2, 1, 0, 0), (0, 1, 0), (0, 0x1e8480, 0xf4240)),
];

/// Builds the FA Cup round table, adjusting team, match and entrant counts by `b`, the entry mode and the entrants.
///
/// Here `b` = +0xb3 (signed byte), `entrada` = +0xb4 and `inscritos` = +0x3e (16-bit counts).
///
/// With +0xb4 = 1, the 1st round proper has 0x28 teams, 0x13 matches and no new teams, and the 2nd has 0x40 teams, 0x20
/// matches, 0x2b new teams and 0x6c entrants. Otherwise, the first has 6b + 0x28 teams, 3b + 0x14 matches, 6b new teams
/// and 0x6c entrants, and the second (0x10 − b) × 4 teams, (0x10 − b) × 2 matches, 0x2c − 7b new teams and (b + 0x12) ×
/// 6 entrants.
///
/// The 3rd has (0x10 − b) × 2 teams and 0x10 − b matches; the 4th, b new teams and +0x3e − b entrants.
///
/// [pt] A tabela da Copa da Inglaterra com `b` = +0xb3 (byte com sinal), `entrada` = +0xb4 e `inscritos` = +0x3e (contas
/// em 16 bits): com +0xb4 = 1, a 1ª rodada propriamente dita tem 0x28 times, 0x13 jogos e nenhum novo, e a 2ª 0x40
/// times, 0x20 jogos, 0x2b novos e 0x6c inscritos; senão, 6b + 0x28 times, 3b + 0x14 jogos, 6b novos e 0x6c inscritos
/// na primeira, (0x10 − b) × 4 times, (0x10 − b) × 2 jogos, 0x2c − 7b novos e (b + 0x12) × 6 inscritos na segunda. A 3ª
/// tem (0x10 − b) × 2 times e 0x10 − b jogos; a 4ª, b novos e +0x3e − b inscritos.
pub(crate) fn fa_cup_rounds(b: i16, entry: u8, entrants: i16) -> [EngCcCupCupRound; 9] {
    let mut t = FA_CUP_ROUNDS;
    let x = |v: i16| v as u16;
    if entry == 1 {
        t[2].teams = (0x28, 0x13, 0, 0);
        t[3].teams = (0x40, 0x20, 0x2b, 0x6c);
    } else {
        t[2].teams = (x(b.wrapping_mul(6).wrapping_add(0x28)), x(b.wrapping_mul(3).wrapping_add(0x14)), x(b.wrapping_mul(6)), 0x6c);
        t[3].teams = (x(0x10i16.wrapping_sub(b).wrapping_mul(4)), x(0x10i16.wrapping_sub(b).wrapping_mul(2)), x(0x2ci16.wrapping_sub(b.wrapping_mul(7))),
            x(b.wrapping_add(0x12).wrapping_mul(6)));
    }
    t[4].teams.0 = x(0x10i16.wrapping_sub(b).wrapping_mul(2));
    t[4].teams.1 = x(0x10i16.wrapping_sub(b));
    t[5].teams.2 = x(b);
    t[5].teams.3 = x(entrants.wrapping_sub(b));
    t
}

/// A Supercopa inglesa (Charity Shield): uma rodada, um jogo em agosto.
pub(crate) const CHARITY_SHIELD_ROUNDS: [EngCcCupCupRound; 1] = [
    cql10r((5, 6, 0, 5), (0xd, 7, 0, 6, 1, 4), 0, (0, 1, 0), 6, (2, 1, 2, 0), (0, 1, 0), (0, 0, 0)), // cql10: the English Super Cup match
];

/// Returns club i of the club table ([G_CLUB_TABLE] + i × 0x245).
///
/// [pt] O clube `i` da tabela dos clubes ([0xae23b4], registros de 0x245 bytes).
pub(crate) unsafe fn eng_cc_cup_club_by_index(i: i16) -> usize {
    read_usize(G_CLUB_TABLE) + i as isize as usize * club::SIZE
}

/// Returns the id of the club's division competition (club +0x57 → +0), or None without a division.
///
/// [pt] O id da competição da divisão do clube (+0x57, ponteiro para o registro da competição; +0 o id), ou None sem divisão.
pub(crate) unsafe fn division_comp_id(c: usize) -> Option<usize> {
    let d = read_usize(c + club::DIVISION);
    if d == 0 { None } else { Some(read_usize(d)) }
}

/// Writes entrant k of the cup entrant list (+0xa7, 6 bytes: club, class +4, status +5 = 0).
///
/// [pt] Grava a inscrição `k` da lista (+0xa7, 6 bytes: o clube, +4 a classe, +5 a situação).
pub(crate) unsafe fn eng_cc_cup_write_entrant(this: Cup, k: i32, club: usize, class: u8) {
    let e = this.team_table() + k as usize * 6;
    write_u32(e, club as u32);
    write_u8(e + 4, class);
    write_u8(e + 5, 0);
}

/// One pass over the clubs of division id outside European cups (+0x1db = −1), adding them with class(club) until the
/// list (+0x3e) is full.
///
/// [pt] Uma passada pelos clubes da divisão `id` ([id], o id da competição) fora das copas europeias (+0x1db = −1), até a
/// lista encher (+0x3e); a classe de cada um vem de `classe(clube)`.
pub(crate) unsafe fn eng_cc_cup_enter_division_clubs(this: Cup, n: &mut i32, id: usize, class: impl Fn(usize) -> u8) {
    let mut i: i16 = 0;
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        let c = eng_cc_cup_club_by_index(i);
        if let Some(d) = division_comp_id(c) {
            if *n != this.team_count() as i32 && d == read_usize(id) && read_usize(c + club::CONTINENTAL_COMP_ID) as i32 == -1 {
                eng_cc_cup_write_entrant(this, *n, c, class(c));
                *n += 1;
                if *n == this.team_count() as i32 {
                    return;
                }
            }
        }
        i = i.wrapping_add(1);
    }
}

/// True if the club's last-season competition (+0x5b) is division `de`.
///
/// [pt] A classe de quem veio da divisão `de` (+0x5b, a competição da temporada anterior; +0x5f a posição nela, byte): `base`
/// menos a posição.
pub(crate) unsafe fn came_from_division(c: usize, of: usize) -> bool {
    let p = read_usize(c + club::LAST_DIVISION);
    p != 0 && read_usize(p) == read_usize(of)
}

/// `EngCcCup::collect_entrants()` (`eng_cc_cup_collect_entrants`, thiscall, ret 0, eng_cc_cup.cpp): fills the League Cup entrant list from
/// the four English divisions with seeding classes, sorts it and sets +0xb2; returns the result and the count.
///
/// +0x3e = 92 and +0xb2 = 0; the list is allocated (`exe::crt_malloc` of 0x228 bytes at +0xa7; out of memory, the error box for
/// line 0x18f and −1).
///
/// Clubs not in the European cups, in this order: the Third ([0x9cf5c8]) and the Second ([0x9cf5c4]) with class 0; the
/// First ([0x9cf5c0]) with 0x46 − position if the club came down from the Premier ([0x9cf5bc]), 0x28 − position if it
/// was already in the First, 0 otherwise; the Premier with 0x78 − position if it was already in the Premier, 0x64 −
/// position if it came up from the First, 0 otherwise. Then the clubs of the four divisions that are in the European
/// cups (+0x1db ≠ −1), with class 0x78, each adding 1 to +0xb2.
///
/// The list goes to qsort (`exe::crt_qsort` with `cup_compare_entries_by_level_asc`: lower class first, ties broken by reputation) and the class then
/// becomes 1 (Premier), 2 (First) or 3 (the others). With the list full: +0xb2 capped at 12 (signed byte) and result 1;
/// otherwise 0. Also returns how many entered (for the comparer).
///
/// [pt] { 0x56ccc0 } (thiscall, ret 0, eng_cc_cup.cpp): `EngCcCup::collect_entrants()`: +0x3e = 92 e +0xb2 = 0; a lista
/// (0x944e46 de 0x228 em +0xa7; sem memória, a caixa da linha 0x18f e −1). Fora das copas europeias, na ordem: a Third
/// ([0x9cf5c8]) e a Second ([0x9cf5c4]) com classe 0; a First ([0x9cf5c0]) com 0x46 − posição se veio da Premier
/// ([0x9cf5bc]), 0x28 − posição se já era da First, 0 senão; a Premier com 0x78 − posição se já era da Premier, 0x64 −
/// posição se veio da First, 0 senão. Depois os clubes das quatro divisões nas copas europeias (+0x1db ≠ −1), classe
/// 0x78, cada um somando 1 ao +0xb2. A lista vai ao qsort (0x945333 com 0x5221c0: a classe menor primeiro, no empate a
/// reputação) e a classe vira 1 (Premier), 2 (First) ou 3 (os outros). Lista cheia: +0xb2 no máximo 12 (byte com sinal) e
/// 1; senão 0. Devolve também quantos entraram (para o comparador).
pub(crate) unsafe fn collect_league_cup_entrants(this: Cup) -> (i32, i32) {
    this.set_team_count(0x5c);
    write_u8(this.addr() + eng_cc_cup::EUROPEAN_ENTRANT_COUNT, 0);
    let l = exe::crt_malloc(0x228);
    this.set_team_table((l as u32) as usize);
    if l == 0 {
        show_error_at(0x9c4224, 0x18f);
        return (-1, 0);
    }
    let mut n = 0i32;
    eng_cc_cup_enter_division_clubs(this, &mut n, KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION, |_| 0);
    eng_cc_cup_enter_division_clubs(this, &mut n, KEY_CLUB_COMP_ENGLISH_SECOND_DIVISION, |_| 0);
    eng_cc_cup_enter_division_clubs(this, &mut n, KEY_CLUB_COMP_ENGLISH_FIRST_DIVISION, |c| {
        let pos = read_u8(c + club::LAST_POSITION);
        if came_from_division(c, KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION) {
            crate::comparison::branch("cqm_10 relegated from premier");
            0x46u8.wrapping_sub(pos) // cqm10: quem caiu da Premier
        } else if came_from_division(c, KEY_CLUB_COMP_ENGLISH_FIRST_DIVISION) {
            0x28u8.wrapping_sub(pos)
        } else {
            0
        }
    });
    eng_cc_cup_enter_division_clubs(this, &mut n, KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION, |c| {
        let pos = read_u8(c + club::LAST_POSITION);
        if came_from_division(c, KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION) {
            0x78u8.wrapping_sub(pos)
        } else if came_from_division(c, KEY_CLUB_COMP_ENGLISH_FIRST_DIVISION) {
            0x64u8.wrapping_sub(pos)
        } else {
            0
        }
    });
    let mut i: i16 = 0;
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        let c = eng_cc_cup_club_by_index(i);
        if let Some(d) = division_comp_id(c) {
            if n != this.team_count() as i32
                && (d == read_usize(KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION) || d == read_usize(KEY_CLUB_COMP_ENGLISH_FIRST_DIVISION) || d == read_usize(KEY_CLUB_COMP_ENGLISH_SECOND_DIVISION) || d == read_usize(KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION))
                && read_usize(c + club::CONTINENTAL_COMP_ID) as i32 != -1
            {
                crate::comparison::branch("cqm_10 club in european cups");
                write_u8(this.addr() + eng_cc_cup::EUROPEAN_ENTRANT_COUNT, read_u8(this.addr() + eng_cc_cup::EUROPEAN_ENTRANT_COUNT).wrapping_add(1));
                eng_cc_cup_write_entrant(this, n, c, 0x78);
                n += 1;
                if n == this.team_count() as i32 {
                    break;
                }
            }
        }
        i = i.wrapping_add(1);
    }
    exe::crt_qsort(this.team_table(), n as u32, 6, 0x5221c0);
    crate::comparison::branch("cqm_10 league cup entrants");
    let mut k: i16 = 0;
    while (k as i32) < n {
        let e = this.team_table() + k as usize * 6;
        let c = read_usize(e);
        let d = if c != 0 { division_comp_id(c) } else { None };
        write_u8(e + 4, match d {
            Some(x) if x == read_usize(KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION) => 1, // cqm10: the Premier class
            Some(x) if x == read_usize(KEY_CLUB_COMP_ENGLISH_FIRST_DIVISION) => 2,
            _ => 3,
        });
        k = k.wrapping_add(1);
    }
    if n != this.team_count() as i32 {
        return (0, n);
    }
    if read_i8(this.addr() + eng_cc_cup::EUROPEAN_ENTRANT_COUNT) > 0xc {
        write_u8(this.addr() + eng_cc_cup::EUROPEAN_ENTRANT_COUNT, 0xc);
    }
    (1, n)
}

/// rule: the League Cup takes the 92 clubs of the four English divisions, in the order Third, Second, First and Premier
/// (outside the European cups), then those in European cups (counted in +0xb2, at most 12: they enter in the 3rd
/// round); each one's class orders the list (qsort, smallest first, ties by smallest reputation): 0 in the Third and
/// Second, in the First 0x46 − position for clubs relegated from the Premier and 0x28 − position for the others, in the
/// Premier 0x78 − position for clubs that stayed and 0x64 − position for promoted ones, 0x78 for the European clubs;
/// after sorting the class becomes 1 (Premier), 2 (First) or 3
/// Original: `EngCcCup::collect_entrants` (0x56ccc0, eng_cc_cup.cpp, thiscall, ret 0).
///
/// [pt] regra: entram na Copa da Liga os 92 clubes das quatro divisões inglesas, na ordem Third, Second, First e Premier
/// (fora das copas europeias), e depois os das copas europeias (contados em +0xb2, no máximo 12: são os que entram na
/// 3ª rodada); a classe de cada um ordena a lista (qsort, a menor primeiro, no empate a menor reputação): 0 na Third e
/// na Second, na First 0x46 − posição para quem caiu da Premier e 0x28 − posição para quem ficou, na Premier 0x78 −
/// posição para quem ficou e 0x64 − posição para quem subiu, 0x78 nas copas europeias; depois da ordem a classe vira 1
/// (Premier), 2 (First) ou 3
pub(crate) unsafe extern "thiscall" fn eng_cc_cup_collect_entrants(this: usize) -> i32 {
    collect_league_cup_entrants(Cup(this)).0
}

/// Side-by-side comparer for routines that allocate the entrant list +0xa7: compares regions and the first bytes(n) of
/// both lists, frees the copy's list.
///
/// [pt] Lado a lado para as rotinas que alocam a lista +0xa7: as regiões `reg` (com o [0xb67a34]) a partir do mesmo estado; a
/// original deixa a lista dela em +0xa7, a cópia faz a sua, que é comparada nos `bytes(n)` primeiros bytes e liberada (o
/// +0xa7 volta à da original).
pub(crate) unsafe fn eng_cc_cup_compare_entrant_list(name: &str, this: Cup, reg: &[(usize, usize)], o: impl FnOnce() -> i32, c: impl FnOnce() -> (i32, i32),
    bytes: impl Fn(i32, i32) -> usize) -> i32 {
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let nr = core::cell::Cell::new(0i32);
    let (a, b, igual, _, _) = crate::comparison::side_by_side(reg, || {
        let r = o();
        lo.set(this.team_table());
        r
    }, || {
        let (r, n) = c();
        lr.set(this.team_table());
        nr.set(n);
        this.set_team_table((lo.get() as u32) as usize);
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let k = bytes(a, nr.get());
    let list = if lo != 0 && lr != 0 {
        core::slice::from_raw_parts(lo as *const u8, k) == core::slice::from_raw_parts(lr as *const u8, k)
    } else {
        lo == 0 && lr == 0
    };
    if lr != 0 && lr != lo {
        exe::crt_free(lr);
    }
    SCOREBOARD.record(igual && a == b && list, || format!("{}({:#x}) returned {} x {} memory {} list {} ({} bytes)", name, this.addr(), a, b, igual, list, k));
    a
}

/// `EngAutoCup::collect_entrants()` (`0x56b3c0`, thiscall, ret 0, `eng_auto_cup.cpp`): builds the entrant list with the
/// second half of the 56 chosen clubs.
///
/// Sets `+0x3e` = 28; allocates the list (`0x944e46` of 0xa8 into `+0xa7`; out of memory, the box of line `0x19f` and
/// −1) with the second half of the 56 chosen clubs (the pointer vector at `+0xb2`, from the 28th on: `+0x70`), class
/// and status 0. Returns 1.
///
/// [pt] { 0x56b3c0 } (thiscall, ret 0, eng_auto_cup.cpp): `EngAutoCup::collect_entrants()`: +0x3e = 28; a lista (0x944e46 de
/// 0xa8 em +0xa7; sem memória, a caixa da linha 0x19f e −1) com a segunda metade dos 56 clubes escolhidos (o vetor de
/// ponteiros em +0xb2, a partir do 28º: +0x70), classe e situação 0. Devolve 1.
pub(crate) unsafe fn collect_auto_cup_entrants(this: Cup) -> (i32, i32) {
    this.set_team_count(0x1c);
    let l = exe::crt_malloc(0xa8);
    this.set_team_table((l as u32) as usize);
    if l == 0 {
        show_error_at(0x9c4180, 0x19f);
        return (-1, 0);
    }
    crate::comparison::branch("cqm_10 league trophy second section");
    let mut k: i16 = 0;
    if this.team_count() > 0 {
        loop {
            let c = read_usize(read_usize(this.addr() + eng_auto_cup::SELECTED_CLUBS) + 0x70 + k as usize * 4); // cqm10: the k-th of the second half
            eng_cc_cup_write_entrant(this, k as i32, c, 0);
            k = k.wrapping_add(1);
            if k >= this.team_count() {
                break;
            }
        }
    }
    (1, k as i32)
}

/// Club that finished at place (0 champion, 1 runner-up) last season in competition id (`comp_util_get_latest_placed_club`).
///
/// [pt] O clube que terminou em `lugar` (0 o campeão, 1 o vice) na última temporada da competição de id [id] (a competição do
/// banco: [0xae23d0] + 0x6b × id; 0x4afd80: push lugar; push competição; add esp, 8 -> cdecl).
pub(crate) unsafe fn latest_placed_club(id: usize, place: u32) -> usize {
    exe::comp_util_get_latest_placed_club(read_usize(G_CLUB_COMP_TABLE).wrapping_add(read_usize(id).wrapping_mul(0x6b)), place)
}

/// `EngCharity::select_teams()` (`0x56d780`, thiscall, ret 0, `eng_charity.cpp`): picks the Premier League champion and
/// the FA Cup winner.
///
/// Sets `+0x3e` = 2; allocates the list (`0x944e46` of 0xc into `+0xa7`; out of memory, the box of line `0xc5` and −1).
/// The Premier champion ([`0x9cf5bc`]; without it, box `0xce` and 0) and the FA Cup winner ([`0x9cf74c`]; without it,
/// `0xd7` and 0); if both are the same club, the second becomes the Premier runner-up (without it, `0xe3` and 0). Class
/// and status 0 for both; returns 1.
///
/// [pt] { 0x56d780 } (thiscall, ret 0, eng_charity.cpp): `EngCharity::select_teams()`: +0x3e = 2; a lista (0x944e46 de 0xc em
/// +0xa7; sem memória, a caixa da linha 0xc5 e −1). O campeão da Premier ([0x9cf5bc]; sem ele, a caixa 0xce e 0) e o da
/// Copa da Inglaterra ([0x9cf74c]; sem ele, 0xd7 e 0); se é o mesmo clube, o segundo vira o vice da Premier (sem ele,
/// 0xe3 e 0). Classe e situação 0 nos dois; devolve 1.
pub(crate) unsafe fn select_charity_shield_teams(this: Cup) -> (i32, i32) {
    this.set_team_count(2);
    let l = exe::crt_malloc(0xc);
    this.set_team_table((l as u32) as usize);
    if l == 0 {
        show_error_at(0x9c425c, 0xc5);
        return (-1, 0);
    }
    let a = latest_placed_club(KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION, 0); // cqm10: the English champion
    write_u32(l, a as u32);
    if a == 0 {
        show_error_at(0x9c425c, 0xce);
        return (0, 0);
    }
    let b = latest_placed_club(KEY_CLUB_COMP_ENGLISH_FA_CUP, 0);
    write_u32(l + 6, b as u32);
    if b == 0 {
        show_error_at(0x9c425c, 0xd7);
        return (0, 0);
    }
    if a == b {
        crate::comparison::branch("cqm_10 english double");
        let b = latest_placed_club(KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION, 1);
        write_u32(l + 6, b as u32);
        if b == 0 {
            show_error_at(0x9c425c, 0xe3);
            return (0, 0);
        }
    }
    crate::comparison::branch("cqm_10 english super cup clubs");
    write_u8(l + 4, 0);
    write_u8(l + 5, 0);
    write_u8(l + 0xa, 0);
    write_u8(l + 0xb, 0);
    (1, 2)
}

/// First dword of Date(day, month, year, weekday) (`exe::date_date_549f70`): low word day of year, high word year.
///
/// [pt] A primeira dword de Date(dia, mês, ano, dia da semana) (0x549f70: push semana; push ano; push mês; push dia; lea ecx,
/// local -> thiscall, ret 0x10): a palavra de baixo é o dia do ano, a de cima o ano.
pub(crate) unsafe fn make_date_dword(day: u32, month: u32, year: u32, week: u32) -> u32 {
    let mut d = [0u32; 2];
    exe::date_date_549f70((&mut d) as *mut [u32; 2] as *mut [u16; 4], day, month, year, week);
    d[0]
}

/// True if the date dword equals the caller's today Date (year then day).
///
/// [pt] A data `d` (dword de `make_date_dword`) é `hoje` (o Date de quem chama: +0 o dia do ano, +2 o ano)? A original compara o
/// ano e depois o dia.
pub(crate) unsafe fn is_date_today(d: u32, today: usize) -> bool {
    (d >> 16) as u16 == read_i16(today + 2) as u16 && d as u16 == read_i16(today) as u16
}

/// Frees the season tables before rebuilding them.
///
/// It frees the entrant list (+0xa7), the rounds table (+0xa3) and the per-round ties (+0xad, one vector per round,
/// +0x3c rounds), each one only if it exists (`exe::crt_free`, cdecl). The pointers are left as they are (the following
/// routines rebuild them).
///
/// [pt] Libera as tabelas da temporada antes de refazer: a lista dos inscritos (+0xa7), a das rodadas (+0xa3) e os confrontos
/// por rodada (+0xad, um vetor por rodada, +0x3c rodadas), cada um se existe (0x9452ca, cdecl). Os ponteiros ficam como
/// estão (as rotinas seguintes os refazem).
pub(crate) unsafe fn free_season_tables(this: Cup) {
    if this.team_table() != 0 {
        exe::crt_free(this.team_table());
    }
    if this.round_table() != 0 {
        exe::crt_free(this.round_table());
    }
    if this.tie_tables() != 0 {
        let mut i = 0i32;
        while i < this.round_count() as i32 {
            let p = read_usize(this.tie_tables() + i as usize * 4);
            if p != 0 {
                exe::crt_free(p);
            }
            i += 1;
        }
        exe::crt_free(this.tie_tables());
    }
}

/// Calls the base cup update (FN_CUP_UPDATE_FOR_DATE, `cup_update_for_date`) with today and the stage, returning its result.
///
/// [pt] A atualização da copa (0x51f890: push fase; push hoje; mov ecx, copa -> thiscall, ret 8), cuja devolução sai.
pub(crate) unsafe fn eng_cc_cup_cup_update_for_date(this: Cup, today: usize, stage: u32) -> u32 {
    exe::cup_update_for_date(this, today, stage)
}

/// rule: the English League Cup is rebuilt once per season (+0x45), on the first day the game year reaches the cup
/// year: the old tables are freed and the 92 entrants, the 7 rounds, the tie tree and the predicted order are built;
/// then, every day, the cup update (draws and matches on the table dates)
/// Original: `EngCcCup::update_for_date` (0x56caf0, eng_cc_cup.cpp, thiscall, ret 8).
///
/// [pt] { 0x56caf0 } (thiscall, ret 8, eng_cc_cup.cpp; a virtual +0x14 da Copa da Liga): `hoje.update_for_date(/// fase)`. Uma vez (+0x45 = 0), quando o ano do jogo ([0xde1f58] +0x439, palavra sem sinal) chega ao da copa (+0x40, com
/// sinal) e a fase não é 0: libera as tabelas (`free_season_tables`), +0x45 = 1, os inscritos (0x56ccc0, thiscall), as
/// rodadas (virtual +0x8c), a árvore dos confrontos (virtual +0x94) e a ordem prevista (virtual +0x5c); uma devolução
/// negativa dá a caixa (linhas 0x161, 0x168, 0x16f) e 0. Sempre, no fim, a atualização da copa.
pub(crate) unsafe extern "thiscall" fn eng_cc_cup_update_for_date(this: usize, today: usize, stage: u32) -> u32 {
    let this = Cup(this);
    if this.season_set_up() == 0 && read_i16(read_usize(G_UEFA_SEEDING) + uefa_seeding::SEASON_YEAR) as u16 as i32 == this.year() as i32 && stage != 0 {
        free_season_tables(this);
        crate::comparison::branch("cqn_10 league cup rollover");
        this.set_season_set_up((1) as usize); // cqn10: the League Cup redone once
        // simplification: in the message boxes the original zeroes only al (the rest of eax comes from the box); the caller reads the byte
        if exe::engcccup_collect_entrants(this) < 0 {
            show_error_at(0x9c4224, 0x161);
            return 0;
        }
        if call_cup_virtual(this, 0x8c) < 0 {
            show_error_at(0x9c4224, 0x168);
            return 0;
        }
        if call_cup_virtual(this, 0x94) < 0 {
            show_error_at(0x9c4224, 0x16f);
            return 0;
        }
        call_cup_virtual(this, 0x5c);
    }
    eng_cc_cup_cup_update_for_date(this, today, stage)
}

// simplification: the season turn frees and rebuilds the tables and the cup update plays and draws: they do not run twice. Only the
// original; the plan: +0x45 goes to 1 when the condition holds, otherwise it stays.

// simplification: like the League Cup one, original only; the plan: +0x45 goes to 1 on 3 October (with +0x45 = 0 and the stage), outside
// of the 1999 path.

// simplification: the call-up touches the squads and the cup update plays: they do not run twice. Only the original runs.

