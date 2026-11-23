use super::*;

/// A Intertoto: cinco rodadas em ida e volta de junho a agosto (a 1ª com 40 times; 12 novos na 2ª, 8 na 3ª).
pub(crate) const INTERTOTO_ROUNDS: [EngCcCupCupRound; 5] = [
    cql10r((0x19, 5, 0, 4), (0x1a, 5, 0, 5, 1, 0), 0xa, (0, 4, 7), 7, (0x28, 0x14, 0x28, 0), (0, 2, 4), (0, 0, 0)), // cqp13: the 1st round of the Intertoto
    cql10r((1, 6, 0, 3), (3, 6, 0, 5, 1, 0), 0x14, (0, 0, 3), 7, (0x20, 0x10, 0xc, 0x28), (0, 2, 7), (0, 0, 0)),
    cql10r((0xb, 6, 0, 6), (0x11, 6, 0, 5, 1, 0), 0x1e, (0, 0, 3), 7, (0x18, 0xc, 8, 0x34), (0, 2, 7), (0, 0, 0)),
    cql10r((0x19, 6, 0, 6), (0x1c, 6, 0, 2, 2, 0), 0x82, (0, 0, 3), 8, (0xc, 6, 0, 0), (0, 2, 7), (0, 0, 0)),
    cql10r((5, 7, 0, 3), (0xa, 7, 0, 1, 2, 0), 0x96, (0, 0, 3), 8, (6, 3, 0, 0), (0, 2, 0xe), (0, 0, 0)),
];

/// rule: the Intertoto Cup has 5 two-legged rounds: 1st drawn on 25/6 and played on 26/6 (40 teams, return 4 days
/// later), 2nd on 3/7 (32 teams: 20 through and 12 new), 3rd on 17/7 (24: 16 and 8 new), semi-final on 28/7 and final
/// on 10/8 (6 teams: the 3 winners go to the UEFA Cup [inference]); category 7 in the first three and 8 after; no
/// prizes
/// Original: `IntertotoCup::create_round_table` (0x633300, intertoto_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn intertoto_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    eng_cc_cup_create_single_stage_round_table(Cup(this), stage, rounds, extra, out, &INTERTOTO_ROUNDS, 0x9e9e14, 0xe4, "cqp_13 tabela da intertoto")
}

/// A Copa Intercontinental: jogo único em 1º de dezembro (sorteio em 10 de outubro), mando 4 (Tóquio [inferência]).
pub(crate) const INTERCONTINENTAL_CUP_ROUNDS: [EngCcCupCupRound; 1] = [
    cql10r((0xa, 9, 0, 5), (1, 0xb, 0, 1, 2, 4), 0, (0, 3, 0), 6, (2, 1, 2, 0), (0, 1, 0), (0, 0, 0)), // cqp13: a Copa Intercontinental
];

/// A Copa Interamericana: ida e volta a partir de 26 de janeiro do ano seguinte (sorteio em 10 de outubro), 7 dias.
pub(crate) const INTERAMERICAN_CUP: [EngCcCupCupRound; 1] = [
    cql10r((0xa, 9, 0, 5), (0x1a, 0, 1, 2, 2, 0), 0, (0, 0, 3), 8, (2, 1, 2, 0), (0, 2, 7), (0, 0, 0)),
];

/// A Copa dos Campeões da CONCACAF: quartas (28/8), semifinal (1/9) e final (4/9) em jogo único, 8 times.
pub(crate) const CONCACAF_CHAMPIONS_CUP_ROUNDS: [EngCcCupCupRound; 3] = [
    cql10r((1, 7, 0, 5), (0x1c, 8, 0, 3, 2, 0), 0xa, (0, 3, 0), 8, (8, 4, 8, 0), (0, 0, 0), (0, 0xffff_ffff, 0)), // cqp13: the CONCACAF quarterfinals
    cql10r((0x1d, 8, 0, 4), (1, 9, 0, 6, 2, 0), 0x82, (0, 3, 0), 8, (4, 2, 0, 0), (0, 0, 0), (0, 0xffff_ffff, 0)),
    cql10r((2, 9, 0, 0), (4, 9, 0, 2, 2, 0), 0x96, (0, 3, 0), 8, (2, 1, 0, 0), (0, 0, 0), (0, 0xffff_ffff, 0)),
];

/// As rodadas fixas da Copa da UEFA (a preliminar e os campos que dependem de +0xb2 vêm à parte, em
/// `uefa_cup_rounds`): a 1ª rodada de qualificação em agosto, a 2ª no fim de agosto, a 1ª rodada em setembro, a 2ª
/// em novembro, a 3ª em dezembro, as oitavas em fevereiro, as quartas e a semifinal em março e abril e a final em
/// maio, todas em ida e volta menos a final.
pub(crate) const UEFA_CUP_ROUNDS: [EngCcCupCupRound; 8] = [
    cql10r((0x1c, 6, 0, 4), (0xa, 7, 0, 3, 2, 0), 0xdc, (4, 0, 3), 7, (0x52, 0x29, 0, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)), // cqq10: the 1st UEFA qualifying round
    cql10r((0x19, 7, 0, 4), (0xe, 8, 0, 3, 2, 0), 0xa, (4, 0x400, 3), 7, (0x60, 0x30, 0x37, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)),
    cql10r((0x1d, 8, 0, 4), (0x13, 9, 0, 3, 2, 0), 0x14, (4, 0, 3), 7, (0x30, 0x18, 0, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)),
    cql10r((3, 0xa, 0, 4), (0x17, 0xa, 0, 3, 2, 0), 0x1e, (0, 0, 3), 8, (0x20, 0x10, 8, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)),
    cql10r((8, 0xb, 0, 4), (1, 2, 1, 3, 2, 0), 0x28, (0, 0, 3), 8, (0x10, 8, 0, 0), (0, 2, 7), (0, 0xffff_ffff, 0)),
    cql10r((9, 2, 1, 4), (0xf, 2, 1, 3, 2, 0), 0x78, (0, 0, 3), 8, (8, 4, 0, 0), (0, 2, 7), (0, 0xffff_ffff, 0)),
    cql10r((0x17, 2, 1, 4), (5, 3, 1, 3, 2, 0), 0x82, (0, 0, 3), 8, (4, 2, 0, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)),
    cql10r((0x14, 3, 1, 4), (0x10, 4, 1, 2, 2, 4), 0x96, (0, 0x83, 0), 8, (2, 1, 0, 0), (0, 1, 0), (0, 0x16e360, 0)),
];

/// A preliminar (só com `b` = +0xb2 > 0): sorteio em 28/6, jogo em 6/7; b times, b/2 jogos, b novos.
pub(crate) const UEFA_CUP_PRELIMINARY_ROUND: EngCcCupCupRound =
    cql10r((0x1c, 5, 0, 2), (6, 6, 0, 3, 2, 0), 0xa0, (0, 0x200, 3), 7, (0, 0, 0, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0));

/// UEFA Cup round descriptions for b = +0xb2 (preliminary round when b > 0, then the qualifying and main rounds with
/// new/entered team counts) → (rounds, count).
pub(crate) fn uefa_cup_rounds(b: i8) -> ([EngCcCupCupRound; 9], usize) {
    let mut t = [UEFA_CUP_PRELIMINARY_ROUND; 9];
    let (b, m) = (b as i32, b as i32 / 2);
    let mut u = UEFA_CUP_ROUNDS;
    u[0].teams.2 = (0x52 - m) as u16;
    u[0].teams.3 = b as u16;
    u[1].teams.3 = (m + 0x52) as u16;
    u[3].teams.3 = (m + 0x89) as u16;
    let mut n = 0;
    if b > 0 {
        covered!("cqq_10 uefa preliminary");
        t[0].teams = (b as u16, m as u16, b as u16, 0);
        n = 1;
    }
    for r in u {
        t[n] = r;
        n += 1;
    }
    (t, n)
}

/// `IntertotoCup::collect_entrants()` (`intertoto_cup_collect_entrants`, thiscall, ret 0, intertoto_cup.cpp): the 60 clubs whose European cup
/// (+0x1db) is the Intertoto, put in the team list (+0xa7) with their coefficient as class and sorted.
///
/// +0x3e = 60; a local vector of 60 pointers (`exe::crt_malloc` of 0xf0; out of memory, the box for line 0x161 and 0) with the
/// clubs of the table ([0xae23b4], up to [0xae235c]) whose European cup (+0x1db) is the Intertoto ([0x9cf6f8]), up to
/// 60; if short, the box for line 0x172 and 0 (the vector stays allocated, as in the original).
///
/// The list (`exe::crt_malloc` of 6 × 60 at +0xa7; out of memory, the box 0x17c and −1) takes each club with class = its
/// coefficient (+0x1df) and status 0, goes to qsort (`cup_compare_entries_by_level_desc`) and the vector is freed. Returns 1 with the 60 (as the
/// result pair).
pub(crate) unsafe fn collect_intertoto_entrants(this: Cup) -> (i32, i32) {
    const F: usize = 0x9e9e14;
    this.set_team_count(0x3c);
    let v = exe::crt_malloc(0xf0);
    if v == 0 {
        show_error_at(F, 0x161);
        return (0, 0);
    }
    let mut k = 0i32;
    let mut i: i16 = 0;
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        if k >= this.team_count() as i32 {
            break;
        }
        let c = club_at(i as isize).addr();
        if read_usize(c + club::CONTINENTAL_COMP_ID) == read_usize(KEY_CLUB_COMP_INTER_TOTO_CUP) {
            write_u32(v + k as usize * 4, c as u32);
            k += 1;
        }
        i = i.wrapping_add(1);
    }
    let n = this.team_count() as i32;
    if k < n {
        show_error_at(F, 0x172);
        return (0, 0);
    }
    let l = exe::crt_malloc((n * 6) as u32);
    this.set_team_table((l as u32) as usize);
    if l == 0 {
        show_error_at(F, 0x17c);
        return (-1, 0);
    }
    covered!("cqq_10 intertoto entrants");
    let mut j: i16 = 0;
    while j < this.team_count() {
        let c = read_usize(v + j as usize * 4);
        let e = this.team_table() + j as usize * 6;
        write_u32(e, c as u32);
        write_u8(e + 4, read_u8(c + club::EURO_SEEDING)); // cqq10: the class by coefficient
        write_u8(e + 5, 0);
        j = j.wrapping_add(1);
    }
    exe::crt_qsort(this.team_table(), this.team_count() as i32 as u32, 6, 0x522190);
    exe::crt_free(v);
    ((k == this.team_count() as i32) as i32, this.team_count() as i32)
}

/// rule: the Intertoto Cup takes the 60 clubs the European rules put in it (European cup `club::CONTINENTAL_COMP_ID` =
/// the Intertoto, `KEY_CLUB_COMP_INTER_TOTO_CUP`), in club table order; each with class = its coefficient
/// (`club::EURO_SEEDING`), and the list is sorted by class (the seeds); with fewer than 60, the error box
/// Original: `IntertotoCup::collect_entrants` (0x6336c0, intertoto_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn intertoto_cup_collect_entrants(this: usize) -> i32 {
    collect_intertoto_entrants(Cup(this)).0
}

/// `IntertotoCup::collect_club_list(&saida, ativos)` (`0x633f70`, thiscall, ret 8, `intertoto_cup.cpp`; a virtual):
/// builds a pointer list of the clubs in list `+0xa7`.
///
/// Allocates the list (`0x944e46` of 4 × `+0x3e`; out of memory, the box of line `0x2fb`, `saida` = 0 and returns 0)
/// with the clubs of list `+0xa7` (non-null, up to `+0x3e`); with `ativos`, only those whose status (`+5`) is 0 or 1.
/// Stores the list in `saida` and returns the count (word).
pub(crate) unsafe fn collect_club_list(this: Cup, out: *mut usize, ativos: u32) -> u16 {
    let n = this.team_count();
    let l = exe::crt_malloc((n as i32 * 4) as u32);
    if l == 0 {
        show_error_at(0x9e9e14, 0x2fb);
        *out = 0;
        return 0;
    }
    covered!("cqq_10 intertoto list");
    let mut k: i16 = 0;
    let mut i: i16 = 0;
    while i < this.team_count() {
        let e = this.team_table() + i as usize * 6;
        let s = read_u8(e + 5);
        let c = read_usize(e);
        if (ativos as u8 == 0 || s == 1 || s == 0) && c != 0 && k < n { // cqq10: the clubs in the list
            write_u32(l + k as usize * 4, c as u32);
            k = k.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    *out = l;
    k as u16
}

/// rule: the Intertoto club list (for the caller): all entrants, or only those still in (status 0 or 1) when asked
/// Original: `IntertotoCup::collect_club_list` (0x633f70, intertoto_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn intertoto_cup_collect_club_list(this: usize, out: *mut usize, ativos: u32) -> u16 {
    collect_club_list(Cup(this), out, ativos)
}

/// The UEFA Cup competition object ([G_COMP_OBJECT_TABLE + 4 × KEY_CLUB_COMP_UEFA_CUP]), or 0.
pub(crate) unsafe fn uefa_cup_object() -> usize {
    read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_UEFA_CUP).wrapping_mul(4)))
}

/// The Intertoto winners (+5 = 1 in +0xa7) that fit the last three UEFA Cup slots, in the order `intertoto_cup_update_for_date` puts them →
/// (clubs, count).
pub(crate) unsafe fn intertoto_winners_for_uefa(this: Cup, uefa: Cup) -> ([usize; 3], usize) {
    let mut v = [0usize; 3];
    let mut k = 0usize;
    let mut slot = uefa.team_count().wrapping_sub(0x1b);
    let limit = slot.wrapping_add(3);
    let mut i = 0i16;
    if this.team_count() > 0 {
        loop {
            if slot >= limit {
                break;
            }
            let e = this.team_table().wrapping_add((i as isize * 6) as usize);
            if read_u8(e + 5) == 1 {
                v[k] = read_usize(e);
                k += 1;
                slot = slot.wrapping_add(1);
            }
            i = i.wrapping_add(1);
            if i >= this.team_count() {
                break;
            }
        }
    }
    (v, k)
}

/// Daily Intertoto update: once a season, when the Intertoto ends, its final winners (up to three, in August) enter the
/// UEFA Cup; every day, the cup update.
///
/// `today.update_for_date(stage)`, virtual +0x14 of the Intertoto. Once (`intertoto_cup::WINNERS_SENT_TO_UEFA` = 0),
/// when the cup has ended (virtual `comp_vtable::CHECK_COMPLETE` with 0, 1: push 1; push 0; mov ecx, this -> thiscall,
/// ret 8; the low byte): without the UEFA Cup (`uefa_cup_object`), the box for line 0x29e; with it, each Intertoto team
/// with +5 = 1 (the winners of the finals) goes, in order, into the places from `cup::TEAM_COUNT` − 27 on of the UEFA
/// Cup's table `cup::TEAM_TABLE` (the last three places; up to three: the club, +4 and +5 zeroed) and into its entry
/// book (`cup::ENTRANT_BOOK`, 6 bytes: the club and status 4 at +4 [inference: entry through the Intertoto];
/// `cup::ENTRANT_COUNT` goes up); then the UEFA predicted positions (virtual `comp_vtable::SET_PREDICTED_POSITIONS`:
/// mov ecx, uefa -> thiscall, ret 0), `intertoto_cup::WINNERS_SENT_TO_UEFA` = 1 and the predicted positions once more
/// (the original re-reads the UEFA Cup). Always, at the end, the cup update (`FN_CUP_UPDATE_FOR_DATE`: push stage; push
/// today; mov ecx, this -> thiscall, ret 8), whose return value is returned.
/// Original: `IntertotoCup::update_for_date` (0x633d40, intertoto_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn intertoto_cup_update_for_date(this: usize, today: usize, stage: u32) -> u32 {
    let this = Cup(this);
    if read_usize(this.addr() + intertoto_cup::WINNERS_SENT_TO_UEFA) == 0 {
        let acabou: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CHECK_COMPLETE));
        if acabou(this.addr(), 0, 1) as u8 != 0 {
            let uefa = uefa_cup_object();
            if uefa == 0 {
                show_error_at(0x9e9e14, 0x29e);
            } else {
                let mut slot = read_i16(uefa + cup::TEAM_COUNT).wrapping_sub(0x1b);
                let limit = slot.wrapping_add(3);
                let mut i = 0i16;
                if this.team_count() > 0 {
                    loop {
                        if slot >= limit {
                            break;
                        }
                        let e = this.team_table().wrapping_add((i as isize * 6) as usize);
                        if read_u8(e + 5) == 1 {
                            covered!("cqi_10 winner in uefa");
                            let s = read_usize(uefa + cup::TEAM_TABLE).wrapping_add((slot as isize * 6) as usize);
                            write_u32(s, read_usize(e) as u32);
                            write_u8(s + 4, 0);
                            write_u8(s + 5, 0);
                            let l = read_usize(uefa + cup::ENTRANT_BOOK).wrapping_add((read_i16(uefa + cup::ENTRANT_COUNT) as isize * 6) as usize);
                            write_u32(l, read_usize(s) as u32);
                            write_u8(l + 4, 4); // cqi10: the situation of the Intertoto winner in the UEFA book
                            write_i16(uefa + cup::ENTRANT_COUNT, read_i16(uefa + cup::ENTRANT_COUNT).wrapping_add(1));
                            slot = slot.wrapping_add(1);
                        }
                        i = i.wrapping_add(1);
                        if i >= this.team_count() {
                            break;
                        }
                    }
                }
                let prever: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(uefa) + comp_vtable::SET_PREDICTED_POSITIONS));
                prever(uefa);
                write_u32(this.addr() + intertoto_cup::WINNERS_SENT_TO_UEFA, 1);
                let uefa = uefa_cup_object();
                let prever: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(uefa) + comp_vtable::SET_PREDICTED_POSITIONS));
                prever(uefa);
                let base: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_CUP_UPDATE_FOR_DATE);
                return base(this.addr(), today, stage);
            }
            write_u32(this.addr() + intertoto_cup::WINNERS_SENT_TO_UEFA, 1);
        }
    }
    let base: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_CUP_UPDATE_FOR_DATE);
    base(this.addr(), today, stage)
}

// simplification: the cup update (0x51f890) plays and rolls: it does not run twice. Only the original runs; the plan, done
// before: if +0xb2 goes from 0 to 1 with the UEFA Cup, its book (+0x14) gets the winners (intertoto_winners_for_uefa), in
// order and with status 4, and +0x36 goes up as much; otherwise the book does not grow.

/// Records a club's Intertoto outcome in the staff records: the final winner qualifies for the UEFA Cup, a club going
/// through gets the next round record, a club knocked out leaves European competition.
///
/// `club.update_staff_records_for_status(outcome, _, _, game, _)`, virtual +0x44 of the Intertoto; returns 0. Weight =
/// `scale_by_round`(low, high, match round `fixture::ROUND_NUMBER`, rounds `.round_count()`) (cdecl, 4 pushes; add esp,
/// 0x10).
///
/// - 1 (won the final): the qualification record (`european_cup_add_qualified_record`) for the UEFA Cup (the
/// competition `G_CLUB_COMP_TABLE` + 0x6b × `KEY_CLUB_COMP_UEFA_CUP`), stage 0, round 0xa, weight(0x3c, 0x64), and the
/// club moves to the UEFA Cup (`.set_continental_comp_id(KEY_CLUB_COMP_UEFA_CUP)`).
///
/// - 0 (went through a round): if the next round exists (`fixture::ROUND_NUMBER` + 1 < `.round_count()`), the
/// qualification record in the Intertoto (+4) with the match's stage `fixture::ROUND_PREFIX`, the +7 of the next round
/// (`.round_table()`, 0x68 bytes) and the weight round × 99 / (rounds − 1) + 1 (only the low byte is incremented: inc
/// al).
///
/// - 0xff or 2 (knocked out, or lost the final): the elimination record (`european_cup_add_knocked_out_record`) in the
/// Intertoto, stage 0, round `fixture::ROUND_NAME` of the match, weight(0x14, 0x50), and the club leaves European
/// competition (`leave_euro_competition`: `club::CONTINENTAL_COMP_ID` = −1).
/// Original: `IntertotoCup::update_staff_records_for_status` (0x633c10, intertoto_cup.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn intertoto_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, a3: u32, a4: u32, game: usize, a6: u32) -> u32 {
    let game = Fixture(game);
    let this = Cup(this);
    let club = Club(club);
    match outcome as u8 {
        1 => {
            let weight = scale_by_round(0x3c, 0x64, game.round_number(), this.round_count() as u16 as u32);
            let uefa = read_usize(G_CLUB_COMP_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_UEFA_CUP).wrapping_mul(0x6b));
            european_cup_add_qualified_record(club.addr(), uefa, 0, 0xa, weight);
            covered!("cqi_10 intertoto winner");
            club.set_continental_comp_id(read_usize(KEY_CLUB_COMP_UEFA_CUP) as u32); // cqi10: o vencedor passa à UEFA
        }
        0 => {
            let r = game.round_number();
            let n = this.round_count();
            let next = r.wrapping_add(1);
            if next >= n {
                return 0;
            }
            let div = n as i32 - 1;
            if div == 0 {
                return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_INTERTOTO_CUP_UPDATE_STAFF_RECORDS_FOR_STATUS)(this.addr(), club.addr(), outcome, a3, a4, game.addr(), a6); // simplification: the original falls into the division
            }
            let q = (r as i32 * 99) / div;
            let weight = (q as u32 & !0xff) | (q as u8).wrapping_add(1) as u32; // inc al
            let round = read_u16(this.round_table().wrapping_add((next as isize * 0x68) as usize) + 7) as u32;
            // simplification: the stage goes in a register whose high 16 bits are those of the next round’s movsx (zero); clears
            european_cup_add_qualified_record(club.addr(), read_usize(this.addr() + 4), (game.round_prefix() as u16) as u32, round, weight);
        }
        0xff | 2 => {
            let weight = scale_by_round(0x14, 0x50, game.round_number(), this.round_count() as u16 as u32);
            // simplification: round +0x30 goes in a register with garbage in the high 16 bits; 0x86c000 only uses the word
            european_cup_add_knocked_out_record(club.addr(), read_usize(this.addr() + 4), 0, (game.round_name() as u16) as u32, weight);
            covered!("cqi_10 dropped into intertoto");
            leave_euro_competition(club); // cqi10: whoever is knocked out in the Intertoto leaves the European competition
        }
        _ => {}
    }
    0
}

// simplification: the records go to the people’s history (reputation, morale): they do not run twice. Only the original runs;
// the plan checks the club’s +0x1db: [0x9cf6f0] for the winner, −1 for those knocked out, untouched for the others.

// simplification: the base writes the records (history, reputation, morale) and may give the UEFA place to the champion: does not run
// times. Only the original runs; the plan checks the club’s +0x1db (−1 on outcomes 1, 2 and 0xff, untouched on the others).

/// The season turnover of the Intertoto Cup: last season's tables freed, the season archived, the year moved on by one,
/// the winners not yet sent to the UEFA Cup, the UEFA seeding's new season, the entrants collected, the round table and
/// the tie tree rebuilt, the predicted positions and the first round set up.
///
/// The first half of the cup turnover (`cup_turnover_free_season`: the team, round and tie tables freed and zeroed, the
/// season archived, the year up by one, `cup::CURRENT_ROUND` and `cup::RUN_BY_STAGES` = 0), then
/// `intertoto_cup::WINNERS_SENT_TO_UEFA` = 0 (the original writes it between the two bytes) and, with the UEFA seeding
/// object ([`G_UEFA_SEEDING`]), UefaSeeding::update_season(0) (`exe::uefaseeding_update_season`, thiscall, ret 4). Then, each
/// with its own box (intertoto_cup.cpp, lines 0x1e9, 0x1f0 and 0x1f7) when it returns a negative value: the entrants
/// (`exe::intertotocup_collect_entrants` IntertotoCup::collect_entrants, thiscall, ret 0), the round table (virtual
/// `comp_vtable::SET_UP_RULES`) and the tie tree (virtual `cup_vtable::MAP_FIXTURE_TREE`). Then the predicted positions
/// (virtual `comp_vtable::SET_PREDICTED_POSITIONS`) and the first round (`exe::cup_set_up_round` Cup::set_up_round(0),
/// push 0; mov ecx -> thiscall, ret 4), whose eax it returns. After a box the copy returns 0 (the original returns the
/// box's eax; the caller does not read it).
/// Original: `IntertotoCup::start_new_season` (0x633920, intertoto_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn intertoto_cup_start_new_season(this: usize) -> u32 {
    let this = Cup(this);
    cup_turnover_free_season(this);
    write_u32(this.addr() + intertoto_cup::WINNERS_SENT_TO_UEFA, 0); // mwg10: the winners not yet sent on
    let seeding = read_usize(G_UEFA_SEEDING);
    if seeding != 0 {
        exe::uefaseeding_update_season(seeding, 0);
    }
    covered!("mwg_10 intertoto rollover");
    if exe::intertotocup_collect_entrants(this) < 0 {
        show_error_at(INTERTOTO_CUP_CPP, 0x1e9);
        return 0;
    }
    if call_cup_virtual(this, comp_vtable::SET_UP_RULES) < 0 {
        show_error_at(INTERTOTO_CUP_CPP, 0x1f0);
        return 0;
    }
    if call_cup_virtual(this, cup_vtable::MAP_FIXTURE_TREE) < 0 {
        show_error_at(INTERTOTO_CUP_CPP, 0x1f7);
        return 0;
    }
    call_cup_virtual(this, comp_vtable::SET_PREDICTED_POSITIONS);
    exe::cup_set_up_round(this, 0) // mwg10: the first round set up
}

/// Comparer for `intertoto_cup_start_new_season`: only the original runs (it frees and rebuilds the cup); the year must
/// go up by one.
pub(crate) unsafe extern "thiscall" fn compare_intertoto_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_INTERTOTO_CUP_START_NEW_SEASON);
    compare_cup_new_season("intertoto_cup_start_new_season", Cup(this), || o(this))
}

/// Rule: each Intertoto match gives UEFA coefficient points to the clubs' nations: 2 to the winner's nation, 1 to each
/// on a draw (by the score after extra time when there was extra time); then the match goes on as in any cup.
///
/// `IntertotoCup::process_fixture(fixture, a2)` (virtual). The score: `home_goals_extra_time()` and
/// `away_goals_extra_time()` when the first is not 0xff, otherwise `home_goals()` and `away_goals()` (signed bytes).
/// Home ahead: 2 points to the home club's nation (UefaSeeding::add_nation_points `add_uefa_coefficient`([club +
/// `club::NATION`], 2): push 2; push nation; mov ecx, [`G_UEFA_SEEDING`] -> thiscall, ret 8); away ahead: 2 to the away
/// club's; a draw: 1 to the home club's, then 1 to the away club's. Then the cup base (`exe::cup_process_fixture`(this,
/// fixture, a2): push a2; push fixture; mov ecx, this -> thiscall, ret 8).
/// Original: `IntertotoCup::process_fixture` (0x633290, intertoto_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn intertoto_cup_process_fixture(this: usize, game: usize, arg: u32) {
    let g = Fixture(game);
    let (ga, gb) = if (g.home_goals_extra_time() as u8) != 0xff {
        ((g.home_goals_extra_time() as u8) as i8, (g.away_goals_extra_time() as u8) as i8)
    } else {
        ((g.home_goals() as u8) as i8, (g.away_goals() as u8) as i8)
    };
    covered!("cqx_11 intertoto coefficient");
    if ga > gb {
        add_uefa_coefficient(g.home_club().nation().addr(), 2); // cqx11: the win's 2 points
    } else if ga < gb {
        add_uefa_coefficient(g.away_club().nation().addr(), 2);
    } else {
        add_uefa_coefficient(g.home_club().nation().addr(), 1);
        add_uefa_coefficient(g.away_club().nation().addr(), 1);
    }
    if INTERTOTO_FIXTURE_DRY_RUN.load(Relaxed) {
        return;
    }
    exe::cup_process_fixture(this, game, arg); // cqx11: the cup base plays the match out
}

/// Comparer for `intertoto_cup_process_fixture` (the `compare_process_fixture` mould of the Champions League): the copy
/// first in a dry run (only the coefficient points, without counting draws), the two nations' coefficient words
/// (`nation_coefficient_word`) read and put back; then the original; the words after the original must equal the
/// copy's. When the stage advanced in the base (its draws add the qualified clubs' coefficient): no comparison. Without
/// a club or a nation: only the original.
pub(crate) unsafe extern "thiscall" fn compare_intertoto_cup_process_fixture(this: usize, game: usize, arg: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_INTERTOTO_CUP_PROCESS_FIXTURE);
    let g = Fixture(game);
    let (a, b) = if !g.is_null() { (g.home_club(), g.away_club()) } else { (Club(0), Club(0)) };
    if a.is_null() || b.is_null() {
        return o(this, game, arg);
    }
    let (ca, cb) = (nation_coefficient_word(a), nation_coefficient_word(b));
    if ca == 0 || cb == 0 {
        return o(this, game, arg);
    }
    let check = [(ca, 2), (cb, 2)];
    let before = crate::comparison::MemorySnapshot::read(&check);
    let stage = Cup(this).current_stage() as i32;
    INTERTOTO_FIXTURE_DRY_RUN.store(true, Relaxed);
    crate::rng::pause_trace(|| crate::rng::without_counting(|| intertoto_cup_process_fixture(this, game, arg)));
    INTERTOTO_FIXTURE_DRY_RUN.store(false, Relaxed);
    let copy = crate::comparison::MemorySnapshot::read(&check);
    before.write_back();
    o(this, game, arg);
    if Cup(this).current_stage() as i32 != stage {
        return; // simplification: the stage advanced and its draw added coefficient: no comparison
    }
    let original = crate::comparison::MemorySnapshot::read(&check);
    SCOREBOARD.record(original == copy, || format!("intertoto_cup_process_fixture(game {:x}) original={:x?} copy={:x?}", game, original, copy));
}

/// The Intertoto Cup constructor `IntertotoCup::IntertotoCup(year, record)`, run once at career creation. On the
/// Cup::Cup base (`exe::new_cup`): the year (`cup::YEAR`), the competition record at +4, the vtable (0x96bb8c),
/// `current_round()` = 0, `stage_index()` = 0xff, `current_stage()` = −1, `stage_count()` = 0, `club_sides()` = 2
/// (clubs of several nations), +0x52 = 3, 7 substitutes named, 3 substitutions allowed,
/// `intertoto_cup::WINNERS_SENT_TO_UEFA` = 0, discipline rule 1 and `run_by_stages()` = 0; no prize fund
/// (`cup::PRIZE_FUND` stays as the base left it). If the saved state loads (Cup::load_from_disk 0x51fc00,
/// thiscall(this, 1), ret 4; returns ≠ 0), it returns this. On a new cup, in order: the yearly European seeding
/// (UefaSeeding::update_season 0x9035a0 on [`G_UEFA_SEEDING`], if any; thiscall, ret 4); the rounds
/// (`comp_vtable::CREATE_SCHEDULE`, into `round_table()`); the 60 entrants (IntertotoCup::collect_entrants 0x6336c0,
/// thiscall, ret 0; the error box of line 0x60 if it returns 0); the tie tree (Cup::map_fixture_tree 0x518790,
/// thiscall, ret 0; the error box of line 0x67 if it returns 0); the statistics at +8 (the error box of line 0x6e
/// without them); the final ranking (Cup::rank_teams 0x5223a0, thiscall, ret 0, unchecked). Every failure returns this.
/// Original: `IntertotoCup::IntertotoCup` (0x632fb0, intertoto_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_intertoto_cup(this: usize, year: u32, record: usize) -> usize {
    construct_intertoto_cup_body(this, year, record) // cqu9: the Intertoto Cup
}

/// Runs only the original (it allocates, draws and reads the save file), checking the plan (`uefa_cup_ctor_plan_error`).
pub(crate) unsafe extern "thiscall" fn compare_construct_intertoto_cup(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_INTERTOTO_CUP)(this, year, record);
    let error = uefa_cup_ctor_plan_error(this, r, year, record, INTERTOTO_CUP_VTABLE, &INTERTOTO_CUP_CTOR_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_intertoto_cup({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// The body of the Intertoto Cup constructor (see `construct_intertoto_cup`).
pub(crate) unsafe fn construct_intertoto_cup_body(this: usize, year: u32, record: usize) -> usize {
    let cup = Cup(this);
    exe::new_cup(this);
    cup.set_year(year as i16);
    write_usize(this + 4, record);
    write_usize(this, INTERTOTO_CUP_VTABLE);
    cup.set_current_round(0);
    cup.set_stage_index(0xff);
    cup.set_current_stage(0xffff_ffff);
    cup.set_stage_count(0);
    cup.set_club_sides(2);
    write_u8(this + 0x52, 3);
    cup.set_substitutes_named(7);
    cup.set_substitutions_allowed(3);
    write_u32(this + intertoto_cup::WINNERS_SENT_TO_UEFA, 0);
    cup.set_discipline_index(1);
    cup.set_run_by_stages(0);
    if exe::cup_load_from_disk(this, 1) != 0 {
        return this;
    }
    covered!("cqu_9 intertoto cup new");
    // the yearly European seeding (UefaSeeding::update_season 0x9035a0: push 0; mov ecx, [G_UEFA_SEEDING] -> thiscall, ret 4)
    let seeding = read_usize(G_UEFA_SEEDING);
    if seeding != 0 {
        exe::uefaseeding_update_season(seeding, 0);
    }
    uefa_cup_ctor_create_rounds(this);
    if exe::intertotocup_collect_entrants(this) == 0 {
        show_error_at(0x9e9e14, 0x60);
        return this;
    }
    if exe::cup_map_fixture_tree(this) == 0 {
        show_error_at(0x9e9e14, 0x67);
        return this;
    }
    if !uefa_cup_ctor_create_stats(this) {
        show_error_at(0x9e9e14, 0x6e);
        return this;
    }
    exe::cup_rank_teams(this);
    this
}

/// "E:\dev\CM3\cm3 00-01\cm3\code\comp\eurocomp\intertoto_cup.cpp" (the error boxes).
pub(crate) const INTERTOTO_CUP_CPP: usize = 0x9e9e14;

/// The comparer's dry run of `intertoto_cup_process_fixture`: the copy gives the coefficient points without the cup
/// base (Cup::process_fixture, which plays out the whole match and cannot run twice).
pub(crate) static INTERTOTO_FIXTURE_DRY_RUN: AtomicBool = AtomicBool::new(false);

pub(crate) const INTERTOTO_CUP_VTABLE: usize = 0x96bb8c;

/// The fixed fields the Intertoto Cup constructor writes on a new cup (offset, width, value).
pub(crate) const INTERTOTO_CUP_CTOR_FIELDS: [(usize, u8, u32); 11] = [(cup::STAGE_COUNT, 4, 0x0), (cup::CURRENT_STAGE, 4, 0xffffffff), (cup::CLUB_SIDES, 1, 0x2), (cup::STAGE_INDEX, 1, 0xff), (cup::SUBSTITUTES_NAMED, 1, 0x7), (cup::SUBSTITUTIONS_ALLOWED, 1, 0x3), (cup::DISCIPLINE_INDEX, 1, 0x1), (0x52, 1, 0x3), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0), (intertoto_cup::WINNERS_SENT_TO_UEFA, 4, 0x0)];
