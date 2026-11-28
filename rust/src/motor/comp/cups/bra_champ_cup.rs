use super::*;

/// Writes one group round (a 0x41-byte record, like the league rounds).
///
/// The date goes through `league_round_set_date` (day, month with 0 = January, weekday, code), with no years to add and value 0; and
/// match 0 is left empty (`store_round_match` with −1, −1, −1 and value 0).
pub(crate) type BraChampCupGroupRound = (u32, u32, u32, u32);

/// What virtual `+0x3c` builds for a stage.
///
/// Either nothing (returns 0 without writing), a group stage (the rounds, the extra value, the box line) or the
/// knockout (the rounds, the extra words (round, offset, value), the box line).
#[derive(Clone, Copy)]
pub(crate) enum BraChampCupStagePlan {
    Nada,
    Grupos(&'static [BraChampCupGroupRound], i16, u32, &'static str),
    Mata(&'static [EngCcCupCupRound], &'static [(usize, usize, u16)], u32, &'static str),
}

/// The virtual +0x3c (`create_schedule(fase, &rodadas, &extra, &é_liga)`, thiscall, ret 0x10) for the chosen stage `f`;
/// sets the outputs and returns the table.
///
/// Group stages: é_liga (if requested) = 1, rounds, extra; the table of 0x41 bytes per round (`exe::crt_malloc`; out of memory,
/// the box and 0) with the dates, the year +0x40 in 16 bits.
///
/// Knockout: é_liga = 0, rounds, extra = 0; the table of 0x68 bytes per round (out of memory, the box and 0), the
/// rounds written by `eng_cc_cup_write_cup_round` with the year +0x40 signed and the extra words.
pub(crate) unsafe fn build_stage_schedule(this: Cup, f: BraChampCupStagePlan, rounds: *mut i16, extra: *mut i16, league: *mut i32, file: usize) -> usize {
    match f {
        BraChampCupStagePlan::Nada => 0,
        BraChampCupStagePlan::Grupos(grupos, id, linha, nome) => {
            if !league.is_null() {
                *league = 1;
            }
            *rounds = grupos.len() as i16;
            *extra = id;
            let t = exe::crt_malloc(grupos.len() as u32 * 0x41);
            if t == 0 {
                show_error_at(file, linha);
                return 0;
            }
            covered!(nome);
            // the year goes with garbage in the high bytes in the original (the previous register with the word +0x40 over it); the
            // 0x68a160 only uses the low 16 bits: the copy passes the clean year
            let year = (this.year() as u16) as u32;
            for (r, &(dia, mes, semana, codigo)) in grupos.iter().enumerate() {
                exe::league_round_set_date(t, r as i32, dia as i32, mes as i32, 0, semana as i32, codigo as i32, year, 0);
                exe::league_round_set_match_slot(t, r as i32, 0, -1, -1, -1, 0);
            }
            t
        }
        BraChampCupStagePlan::Mata(mata, a_mais, linha, nome) => {
            if !league.is_null() {
                *league = 0;
            }
            *rounds = mata.len() as i16;
            *extra = 0;
            let t = exe::crt_malloc(mata.len() as u32 * 0x68);
            if t == 0 {
                show_error_at(file, linha);
                return 0;
            }
            covered!(nome);
            let year = this.year() as i32 as u32;
            for (i, r) in mata.iter().enumerate() {
                eng_cc_cup_write_cup_round(t, i as u32, r, year);
            }
            for &(r, o, v) in a_mais {
                write_u16(t + r * 0x68 + o, v);
            }
            t
        }
    }
}

/// Bytes the original writes in an n-round table for the stage: group rounds +0..+0xb and +0x3d..+0x40, knockout rounds
/// the cup bytes plus the extra words.
pub(crate) unsafe fn written_schedule_bytes(f: BraChampCupStagePlan, t: usize, n: i16) -> Vec<u8> {
    match f {
        BraChampCupStagePlan::Nada => Vec::new(),
        BraChampCupStagePlan::Grupos(..) => {
            let mut v = Vec::new();
            for r in 0..n.max(0) as usize {
                let e = t + r * 0x41;
                v.extend_from_slice(core::slice::from_raw_parts(e as *const u8, 0xc));
                v.extend_from_slice(core::slice::from_raw_parts((e + 0x3d) as *const u8, 4));
            }
            v
        }
        BraChampCupStagePlan::Mata(_, a_mais, ..) => {
            let mut v = eng_cc_cup_written_round_bytes(t, n);
            for &(r, o, _) in a_mais {
                v.extend_from_slice(core::slice::from_raw_parts((t + r * 0x68 + o) as *const u8, 2));
            }
            v
        }
    }
}

/// Side-by-side comparer of create_schedule.
///
/// The original routine runs with the caller's outputs, the copy with its own outputs that start with the same values,
/// both from the same [0xb67a34]; the outputs and the bytes written to the two tables must be equal; the copy's table
/// is freed (`exe::crt_free`).
///
/// In the stage without a table only the original runs (neither writes anything; the outputs may not even exist).
pub(crate) unsafe fn compare_stage_schedule(name: &str, stage: u32, f: BraChampCupStagePlan, rounds: *mut i16, extra: *mut i16, league: *mut i32,
    o: impl FnOnce() -> usize, c: impl FnOnce(*mut i16, *mut i16, *mut i32) -> usize) -> usize {
    if let BraChampCupStagePlan::Nada = f {
        let a = o();
        SCOREBOARD.record(a == 0, || format!("{}(stage {}) returned {:#x}", name, stage as u8 as i8, a));
        return a;
    }
    let (mut r2, mut e2) = (*rounds, *extra);
    let mut l2 = if league.is_null() { 0 } else { *league };
    let reg = [(0xb67a34usize, 4usize)];
    let (a, b, igual, _, _) = crate::comparison::side_by_side(&reg, o, || c(&mut r2, &mut e2, if league.is_null() { core::ptr::null_mut() } else { &mut l2 }));
    let outs = *rounds == r2 && *extra == e2 && (league.is_null() || *league == l2);
    let table = if a != 0 && b != 0 { written_schedule_bytes(f, a, *rounds) == written_schedule_bytes(f, b, r2) } else { a == 0 && b == 0 };
    if b != 0 {
        exe::crt_free(b);
    }
    SCOREBOARD.record(igual && outs && table, || format!("{}(stage {}): outs {} ({} x {}) table {}", name, stage as u8 as i8, outs, *rounds, r2, table));
    a
}

/// A Copa dos Campeões, a fase preliminar (fase −1): três rodadas às segundas [inferência: dia da semana 1], 19 e 26 de
/// junho e 3 de julho.
pub(crate) const CHAMPIONS_CUP_PRELIMINARY_ROUNDS: [BraChampCupGroupRound; 3] = [(0x13, 5, 1, 2), (0x1a, 5, 1, 2), (3, 6, 1, 2)]; // cqp10: the Champions Cup preliminary

/// A Copa dos Campeões, os grupos (fases 0 e 1): três rodadas, 9, 16 e 20 de julho.
pub(crate) const CHAMPIONS_CUP_GROUP_ROUNDS: [BraChampCupGroupRound; 3] = [(9, 6, 0, 2), (0x10, 6, 0, 2), (0x14, 6, 4, 2)];

/// A Copa dos Campeões, o mata-mata (fase 2 em diante): semifinal e final em ida e volta no fim de julho.
pub(crate) const CHAMPIONS_CUP_KNOCKOUT_ROUNDS: [EngCcCupCupRound; 2] = [
    cql10r((0x15, 6, 0, 5), (0x17, 6, 0, 0, 2, 0), 0x82, (0, 0x200, 1), 6, (4, 2, 4, 0), (0, 2, 2), (0, 0xffff_ffff, 0)),
    cql10r((0x1a, 6, 0, 3), (0x1b, 6, 0, 4, 2, 0), 0x96, (0, 0, 1), 6, (2, 1, 0, 0), (0, 2, 3), (0, 0xc3500, 0x61a80)),
];

/// Stage plan of the Copa dos Campeoes by signed stage: -1 preliminary, below 2 groups, 2 onward knockout.
pub(crate) fn champions_cup_stage(stage: u32) -> BraChampCupStagePlan {
    let f = stage as u8 as i8;
    if f == -1 {
        BraChampCupStagePlan::Grupos(&CHAMPIONS_CUP_PRELIMINARY_ROUNDS, 0xe6, 0xaa, "cqp_10 preliminar da copa dos campeoes")
    } else if f < 2 {
        BraChampCupStagePlan::Grupos(&CHAMPIONS_CUP_GROUP_ROUNDS, (f as i16).wrapping_add(0x3fd), 0xcd, "cqp_10 grupos da copa dos campeoes")
    } else {
        BraChampCupStagePlan::Mata(&CHAMPIONS_CUP_KNOCKOUT_ROUNDS, &[], 0xed, "cqp_10 mata-mata da copa dos campeoes")
    }
}

/// rule: the Copa dos Campeões has a preliminary stage (stage −1, id 0xe6) of three rounds on Mondays [inference:
/// weekday 1], 19 and 26 June and 3 July; then two groups (stages 0 and 1, id 0x3fd + stage) of three rounds, 9, 16 and
/// 20 July; the knockout (stage 2 on): semi-final drawn on 21/7 and played on 23/7, return 2 days later, and final
/// drawn on 26/7, match on 27/7, return 3 days later; category 6; two-leg tie-break 1; final prize 800 thousand to the
/// winner and 400 thousand to the runner-up
/// Original: `BraChampCup::create_schedule` (0x425c10, bra_champ_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn bra_champ_cup_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    build_stage_schedule(Cup(this), champions_cup_stage(stage), rounds, extra, league, 0x98a1e8)
}

/// A Merconorte, grupos (fases abaixo de 3): seis rodadas às terças de 15 em 15 dias, de 9 de setembro a 18 de novembro.
pub(crate) const MERCONORTE_GROUP_ROUNDS: [BraChampCupGroupRound; 6] = [(9, 8, 2, 2), (0x17, 8, 2, 2), (7, 9, 2, 2), (0x15, 9, 2, 2), (4, 0xa, 2, 2), (0x12, 0xa, 2, 2)]; // cqp10: the Merconorte groups

/// A Merconorte, o mata-mata (fase 3): semifinal em ida e volta (7 dias) e final em jogo único em dezembro.
pub(crate) const MERCONORTE_KNOCKOUT_ROUNDS: [EngCcCupCupRound; 2] = [
    cql10r((0x13, 0xa, 0, 3), (0x19, 0xa, 0, 2, 2, 0), 0x82, (0, 3, 0), 8, (4, 2, 4, 0), (0, 2, 7), (0, 0xffff_ffff, 0)),
    cql10r((3, 0xb, 0, 3), (9, 0xb, 0, 2, 2, 0), 0x96, (0, 3, 0), 8, (2, 1, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
];

/// Stage plan of the Merconorte: below 3 groups, 3 knockout, above nothing.
pub(crate) fn merconorte_stage(stage: u32) -> BraChampCupStagePlan {
    let f = stage as u8 as i8;
    if f < 3 {
        BraChampCupStagePlan::Grupos(&MERCONORTE_GROUP_ROUNDS, (f as i16).wrapping_add(0x3e9), 0x9b, "cqp_10 grupos da merconorte")
    } else if f == 3 {
        BraChampCupStagePlan::Mata(&MERCONORTE_KNOCKOUT_ROUNDS, &[], 0xc1, "cqp_10 mata-mata da merconorte")
    } else {
        BraChampCupStagePlan::Nada
    }
}

/// A Supercopa Europeia: jogo único em 24 de agosto (sorteio em 4 de julho) em campo neutro (mando 0x8000), sem prêmios.
pub(crate) const EUROPEAN_SUPER_CUP_ROUNDS: [EngCcCupCupRound; 1] = [
    cql10r((4, 6, 0, 5), (0x18, 7, 0, 4, 2, 0x8000), 0, (0, 0x83, 0), 8, (2, 1, 2, 0), (0, 1, 0), (0, 0, 0)), // cqp10: a Supercopa Europeia
];

/// A Supercopa Asiática: ida e volta (14 dias entre os jogos) a partir de 4 de agosto, sorteio em 4 de julho, sem prêmios.
pub(crate) const ASIAN_SUPER_CUP: [EngCcCupCupRound; 1] = [
    cql10r((4, 6, 0, 5), (4, 7, 0, 5, 2, 0), 0, (0, 0, 0x83), 6, (2, 1, 2, 0), (0, 2, 0xe), (0, 0, 0)), // cqp10: a Supercopa Asiática
];

/// rule: the Brazilian Champions' Cup group is single round robin, 3 points a win and 1 a draw; tiebreak by codes 3, 1,
/// 2 and 4 [inference: wins, goal difference, goals for, head-to-head]; two qualifiers; rule bits 8 (those who secure
/// the places go to the next stage); category 23 on each match (the champions' finals one); no prize fund
/// Original: `BraChampCup::set_up_rules` (0x426230, bra_champ_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_champ_cup_set_up_rules(this: usize) -> u32 {
    covered!("cqi_12 champions cup rules");
    set_cup_league_rules(League(this), &CHAMPIONS_CUP_RULES, 1)
}

/// Builds the Brazilian Champions Cup: League::League (`exe::new_league`), the competition record, the vtable, the
/// year (word), 3 substitutions allowed, 7 substitutes named, discipline rule 6 and importance 12. A saved state
/// (`exe::league_load_from_temp_file` with 1) ends it. Otherwise the league flags 0x28, the main stage (0xff), the
/// current stage −1, three stages and their vector (`exe::crt_malloc` of 12, unchecked), the entrants
/// (`bra_champ_cup_select_entrants`), the statistics (operator new 0x5ce and `exe::new_comp_stats`; none: the error box
/// of line 0x53) and the rules (`BraChampCup::set_up_rules`; 0: the box of line 0x5a); with them, the competition is
/// not active yet and has no table nor clubs. Returns the object.
/// Original: `BraChampCup::BraChampCup` (0x4259a0, bra_champ_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_bra_champ_cup(this: usize, year: u32, record: usize) -> usize {
    let cup = League(this);
    exe::new_league(this);
    write_usize(this + league::COMPETITION, record);
    write_usize(this, BRA_CHAMP_CUP_VTABLE);
    cup.set_year(year as u16 as i16);
    cup.set_substitutions_allowed(3);
    cup.set_substitutes_named(7);
    cup.set_discipline_rule(6);
    cup.set_importance(0xc);
    if exe::league_load_from_temp_file(this, 1) != 0 {
        return this;
    }
    covered!("cx01_10 new champions cup");
    cup.set_league_flags(0x28);
    cup.set_stage_index(stage_index::MAIN);
    write_i32(this + league::CURRENT_STAGE, -1);
    cup.set_stage_count(3);
    cup.set_stages(exe::crt_malloc(0xc));
    bra_champ_cup_select_entrants(this);
    let p = exe::crt_operator_new(0x5ce);
    let stats = if p != 0 { exe::new_comp_stats(p, this) } else { 0 };
    cup.set_comp_stats(stats);
    if stats == 0 {
        show_error_at(BRA_CHAMP_CUP_SOURCE, 0x53);
        return this;
    }
    let rules: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_BRA_CHAMP_CUP_SET_UP_RULES);
    if rules(this) == 0 {
        show_error_at(BRA_CHAMP_CUP_SOURCE, 0x5a);
        return this;
    }
    cup.set_active(0);
    cup.set_table(0);
    cup.set_club_count(0);
    this
}

/// Picks the nine entrants of the Brazilian Champions Cup into a new entrant book.
///
/// First, in table order and up to 200, the clubs whose state championship (`club::RESERVE_DIVISION`) is one of the
/// twelve of `BRA_CHAMP_CUP_STATE_PLACES`, sorted by reputation (`exe::crt_qsort` with
/// `compare_clubs_by_reputation_desc_nulls_last`). The book: `ENTRANTS_ARE_QUALIFIERS` = 0xff, capacity and count 9,
/// nine 6-byte entries (`exe::crt_calloc`; none: the error box of line 0x3d7). Every Brazilian club (`club::NATION`
/// id = Brazil) whose long name is one of `BRA_CHAMP_CUP_NAMED_CLUBS` takes that place. If a place is still empty,
/// each listed club in order takes its state's place when that place is empty.
/// Original: `BraChampCup::select_entrants` (0x426d80, bra_champ_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_champ_cup_select_entrants(this: usize) {
    let cup = League(this);
    let mut listed = [0usize; 0xc8];
    let mut n = 0usize;
    let mut i = 0i16;
    while (i as i32) < read_i32(G_CLUB_COUNT) && n < 0xc8 {
        let c = club_at(i as isize);
        let state = c.reserve_division();
        if !state.is_null() && bra_champ_cup_state_place(read_u32(state.addr())).is_some() {
            listed[n] = c.addr();
            n += 1;
        }
        i += 1;
    }
    exe::crt_qsort(listed.as_mut_ptr() as usize, n as u32, 4, FN_COMPARE_CLUBS_BY_REPUTATION_DESC_NULLS_LAST);
    cup.set_entrants_are_qualifiers(0xff);
    cup.set_entrant_capacity(9);
    cup.set_entrant_count(9);
    let book = exe::crt_calloc(6, 9);
    cup.set_entrant_book(book);
    if book == 0 {
        show_error_at(BRA_CHAMP_CUP_SOURCE, 0x3d7);
        return;
    }
    let brazil = read_u32(KEY_NATION_BRAZIL);
    let mut i = 0i16;
    while (i as i32) < read_i32(G_CLUB_COUNT) {
        let c = club_at(i as isize);
        if !c.nation().is_null() && read_u32(c.nation().addr()) == brazil {
            let name = c_string_bytes(c.name_addr());
            for &(named, place) in &BRA_CHAMP_CUP_NAMED_CLUBS {
                if name == c_string_bytes(named) {
                    covered!("cx01_10 named entrant");
                    write_usize(book + 6 * place, c.addr());
                }
            }
        }
        i += 1;
    }
    if (0..9).all(|k| read_usize(book + 6 * k) != 0) {
        return;
    }
    for &c in &listed[..n] {
        if let Some(place) = bra_champ_cup_state_place(read_u32(Club(c).reserve_division().addr())) {
            if read_usize(book + 6 * place) == 0 {
                covered!("cx01_10 state entrant");
                write_usize(book + 6 * place, c);
            }
        }
    }
}

/// Stores the predicted final positions of the nine entrants (`BRA_CHAMP_CUP_PREDICTED`, in entrant order) in the
/// competition statistics (`exe::comp_stats_set_club_final_position`). Without statistics the error box of line 0x319;
/// the clubs are first copied into a list of nine (`exe::crt_malloc`; none: the box of line 0x321), freed at the end.
/// Original: `BraChampCup::set_predicted_positions` (0x426ab0, bra_champ_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_champ_cup_set_predicted_positions(this: usize) {
    let cup = League(this);
    if cup.comp_stats() == 0 {
        show_error_at(BRA_CHAMP_CUP_SOURCE, 0x319);
        return;
    }
    let list = exe::crt_malloc(0x24);
    if list == 0 {
        show_error_at(BRA_CHAMP_CUP_SOURCE, 0x321);
        return;
    }
    covered!("cx01_10 predicted positions");
    for k in 0..9 {
        write_usize(list + 4 * k, read_usize(cup.entrant_book() + 6 * k));
    }
    for (k, &position) in BRA_CHAMP_CUP_PREDICTED.iter().enumerate() {
        exe::comp_stats_set_club_final_position(cup.comp_stats(), read_usize(list + 4 * k), position as u32);
    }
    exe::crt_free(list);
}

/// Comparer for `construct_bra_champ_cup`: only the original runs (it allocates the stages, the book and the
/// statistics); the return, the vtable, the year, the record and the bench and discipline values must be the ones the
/// copy writes.
pub(crate) unsafe extern "thiscall" fn compare_construct_bra_champ_cup(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_BRA_CHAMP_CUP)(this, year, record);
    let cup = League(this);
    let ok = r == this && read_usize(this) == BRA_CHAMP_CUP_VTABLE && cup.year() == year as u16 as i16 && cup.competition().addr() == record
        && cup.substitutions_allowed() == 3 && cup.substitutes_named() == 7 && cup.discipline_rule() == 6 && cup.importance() == 0xc;
    SCOREBOARD.record(ok, || format!("construct_bra_champ_cup({:#x}, {}, {:#x}): returned {:#x}, vtable {:#x}, year {}, record {:#x}, bench {}/{}, discipline {}, importance {}",
        this, year as u16, record, r, read_usize(this), cup.year(), cup.competition().addr(), cup.substitutes_named(), cup.substitutions_allowed(), cup.discipline_rule(), cup.importance()));
    r
}

/// Comparer for `bra_champ_cup_select_entrants`: the original builds its book, then the copy builds another; the nine
/// entries and the book fields must match. The copy's book is freed and the original's put back.
pub(crate) unsafe extern "thiscall" fn compare_bra_champ_cup_select_entrants(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_CHAMP_CUP_SELECT_ENTRANTS)(this);
    let cup = League(this);
    let book = cup.entrant_book();
    if book == 0 {
        return;
    }
    let fields = |c: League| (c.entrants_are_qualifiers(), c.entrant_capacity(), c.entrant_count());
    let original = (fields(cup), core::slice::from_raw_parts(book as *const u8, 6 * 9).to_vec());
    bra_champ_cup_select_entrants(this);
    let mine = cup.entrant_book();
    let copy = (fields(cup), if mine != 0 { core::slice::from_raw_parts(mine as *const u8, 6 * 9).to_vec() } else { Vec::new() });
    if mine != 0 {
        exe::crt_free(mine);
    }
    cup.set_entrant_book(book);
    SCOREBOARD.record(original == copy, || format!("bra_champ_cup_select_entrants({:#x}): original {:x?} rust {:x?}", this, original, copy));
}

/// Comparer for `bra_champ_cup_set_predicted_positions`: only the original runs (it may add the clubs to the
/// statistics); each entrant's final position must then be the copy's (`BRA_CHAMP_CUP_PREDICTED`).
pub(crate) unsafe extern "thiscall" fn compare_bra_champ_cup_set_predicted_positions(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_CHAMP_CUP_SET_PREDICTED_POSITIONS)(this);
    let cup = League(this);
    if cup.comp_stats() == 0 || cup.entrant_book() == 0 {
        return;
    }
    let got: Vec<u8> = (0..9).map(|k| {
        let e = exe::comp_stats_find_club_entry(cup.comp_stats(), read_usize(cup.entrant_book() + 6 * k));
        if e != 0 { CompStatsClub(e).final_position() } else { 0 }
    }).collect();
    SCOREBOARD.record(got == BRA_CHAMP_CUP_PREDICTED, || format!("bra_champ_cup_set_predicted_positions({:#x}): {:?}", this, got));
}

/// Builds the table of the three-club preliminary (the cup itself is the league): three clubs (`CLUB_COUNT`), the table
/// (`exe::crt_malloc` of 0xb1 bytes; none: the error box of line 0x195 and 0), the first three entrants in rows 0 to 2
/// (`exe::league_init_table_row` with 0, 0), each taken out of the entrant book as it goes in, and the group draw news
/// (`exe::league_send_group_draw_news`). Returns 1.
/// Original: `BraChampCup::create_preliminary_table` (0x426130, bra_champ_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_champ_cup_create_preliminary_table(this: usize) -> u32 {
    let cup = League(this);
    cup.set_club_count(3);
    let table = exe::crt_malloc(0xb1);
    cup.set_table(table);
    if table == 0 {
        show_error_at(BRA_CHAMP_CUP_SOURCE, 0x195);
        return 0;
    }
    covered!("cx01_11 preliminary table");
    let mut row = 0i8;
    while (row as i16) < cup.club_count() {
        let entry = cup.entrant_book() + row as isize as usize * 6;
        // the original passes the row as a dword whose high bytes are whatever was on the stack: only the byte is read
        exe::league_init_table_row(this, row as u8 as u32, read_usize(entry), 0, 0);
        write_usize(entry, 0); // the club leaves the entrant book
        row = row.wrapping_add(1);
    }
    exe::league_send_group_draw_news(this);
    1
}

/// Starts the preliminary: the predicted positions (`comp_vtable::SET_PREDICTED_POSITIONS`), then, each with its error
/// box when it answers 0, the preliminary table (`BraChampCup::create_preliminary_table`; line 0x491), the fixtures
/// (`exe::league_create_fixtures`; line 0x497) and the table order (`exe::league_sort_table` with 0; line 0x49e);
/// finally the competition is active.
/// Original: `BraChampCup::start_preliminary_stage` (0x427320, bra_champ_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_champ_cup_start_preliminary_stage(this: usize) {
    let cup = League(this);
    let predicted: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::SET_PREDICTED_POSITIONS));
    predicted(this);
    let table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_BRA_CHAMP_CUP_CREATE_PRELIMINARY_TABLE);
    if table(this) == 0 {
        return show_error_at(BRA_CHAMP_CUP_SOURCE, 0x491);
    }
    if exe::league_create_fixtures(this) == 0 {
        return show_error_at(BRA_CHAMP_CUP_SOURCE, 0x497);
    }
    if exe::league_sort_table(this, 0) == 0 {
        return show_error_at(BRA_CHAMP_CUP_SOURCE, 0x49e);
    }
    covered!("cx01_11 preliminary started");
    cup.set_active(1);
}

/// Moves the Champions Cup on by one stage: at the last stage (`CURRENT_STAGE` ≥ `STAGE_COUNT` − 1, signed) the error
/// box of line 0x219 and nothing else; otherwise the stage goes up by one and, at 0 (the end of the preliminary), the
/// groups are built (`BraChampCup::create_group_stages`); at 2 (the end of the groups), the semi-finals
/// (`BraChampCup::create_semi_final_stage`).
/// Original: `BraChampCup::advance_stage` (0x4263c0, bra_champ_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_champ_cup_advance_stage(this: usize) {
    let this = League(this);
    let cup = this;
    let stage = cup.current_stage() as i32;
    if stage >= (cup.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(BRA_CHAMP_CUP_SOURCE, 0x219);
    }
    let stage = stage + 1;
    this.set_current_stage((stage) as usize);
    let next = match stage {
        0 => FN_BRA_CHAMP_CUP_CREATE_GROUP_STAGES, // the groups after the preliminary
        2 => FN_BRA_CHAMP_CUP_CREATE_SEMI_FINAL_STAGE, // the semi-finals after the groups
        _ => return,
    };
    covered!("cx01_11 next stage built");
    let build: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(next);
    build(this.addr());
}

/// Comparer for `bra_champ_cup_create_preliminary_table`: only the original runs (the rows allocate and the news goes
/// to the humans); the plan made before must hold: 1 returned, three clubs, the first three entrants in the rows in
/// order and taken out of the book.
pub(crate) unsafe extern "thiscall" fn compare_bra_champ_cup_create_preliminary_table(this: usize) -> u32 {
    let cup = League(this);
    let plan: Vec<usize> = (0..3).map(|k| read_usize(cup.entrant_book() + k * 6)).collect();
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_CHAMP_CUP_CREATE_PRELIMINARY_TABLE)(this);
    let table = cup.table();
    let ok = r == 1 && cup.club_count() == 3 && table != 0
        && plan.iter().enumerate().all(|(k, &c)| read_usize(table + k * league_table_row::SIZE) == c && read_usize(cup.entrant_book() + k * 6) == 0);
    SCOREBOARD.record(ok, || format!("bra_champ_cup_create_preliminary_table({:#x}): returned {}, plan {:x?}", this, r, plan));
    r
}

/// Comparer for `bra_champ_cup_start_preliminary_stage`: only the original runs (the table, the fixtures and the order
/// allocate and draw); then the competition must be active with three clubs.
pub(crate) unsafe extern "thiscall" fn compare_bra_champ_cup_start_preliminary_stage(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_CHAMP_CUP_START_PRELIMINARY_STAGE)(this);
    let cup = League(this);
    SCOREBOARD.record(cup.active() == 1 && cup.club_count() == 3, || format!("bra_champ_cup_start_preliminary_stage({:#x}): active {} clubs {}", this, cup.active(), cup.club_count()));
}

/// Comparer for `bra_champ_cup_advance_stage`: only the original runs (the groups and the semi-finals allocate and
/// draw); the stage stays at the last one and otherwise goes up by one (the groups write their own stage afterwards:
/// there it only has to move on).
pub(crate) unsafe extern "thiscall" fn compare_bra_champ_cup_advance_stage(this: usize) {
    let cup = League(this);
    let before = cup.current_stage() as i32;
    let last = before >= (cup.stage_count() as i32).wrapping_sub(1);
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_CHAMP_CUP_ADVANCE_STAGE)(this);
    let after = cup.current_stage() as i32;
    let ok = if last { after == before } else if before + 1 == 0 { after > before } else { after == before + 1 };
    SCOREBOARD.record(ok, || format!("bra_champ_cup_advance_stage({:#x}): stage {} -> {}", this, before, after));
}

/// Builds the two four-club groups of the Brazilian Champions Cup (`bra_champ_cup_group_clubs`), each as a stage
/// (`bra_champ_cup_build_group`) followed by its group draw news (`exe::league_send_group_draw_news`); a group not
/// built shows the error box (group A line 0x252, group B line 0x268) and answers 0. Otherwise the current stage is 1
/// and it answers 1.
/// Original: `BraChampCup::create_group_stages` (0x426470, bra_champ_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_champ_cup_create_group_stages(this: usize) -> u32 {
    let this = League(this);
    let cup = this;
    let groups = bra_champ_cup_group_clubs(cup);
    for (g, row) in [(0u32, 0x252u32), (1, 0x268)] {
        let stage = bra_champ_cup_build_group(cup, g, &groups[g as usize]);
        if stage == 0 {
            show_error_at(BRA_CHAMP_CUP_SOURCE, row);
            return 0;
        }
        exe::league_send_group_draw_news(stage);
    }
    covered!("cx01_12 champions cup groups");
    this.set_current_stage((1) as usize);
    1
}

/// Builds the four-club knockout of the Champions Cup as stage 2: the semi-finalists (`bra_champ_cup_semi_finalists`)
/// in a list of four (`exe::crt_malloc`; none: the error box of line 0x286), then `create_knockout_stage` (the round
/// table of stage 2, the Cup_Stage with no seeds, the list and the table freed, the box of line 0x29d without a stage,
/// otherwise the first draw).
/// Original: `BraChampCup::create_semi_final_stage` (0x426720, bra_champ_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_champ_cup_create_semi_final_stage(this: usize) {
    let cup = League(this);
    let clubs = exe::crt_malloc(0x10);
    if clubs == 0 {
        return show_error_at(BRA_CHAMP_CUP_SOURCE, 0x286);
    }
    covered!("cx01_12 champions cup semi-finals");
    for (k, c) in bra_champ_cup_semi_finalists(cup).into_iter().enumerate() {
        write_usize(clubs + 4 * k, c);
    }
    create_knockout_stage(cup, 4, clubs, 2, core::ptr::null(), BRA_CHAMP_CUP_SOURCE, 0x29d);
}

/// Turns the Champions Cup over to a new season: not closed, the table freed (`exe::league_free_table` with 0), the
/// round dates freed and cleared, the statistics archived (`exe::comp_stats_archive_season_and_reset` with 1), every
/// stage up to the current one destroyed (virtual slot 0 with 1), the year up by one and the current stage −1; then
/// the rules (`comp_vtable::SET_UP_RULES`; 0: the error box of line 0x1fc and 0); with them, no table, no clubs and not
/// active, and it answers the rules' value.
/// Original: `BraChampCup::start_new_season` (0x4262c0, bra_champ_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_champ_cup_start_new_season(this: usize) -> u32 {
    let cup = League(this);
    cup.set_season_closed(0);
    exe::league_free_table(this, 0);
    if cup.round_dates() != 0 {
        exe::crt_free(cup.round_dates());
        cup.set_round_dates(0);
    }
    if cup.comp_stats() != 0 {
        exe::comp_stats_archive_season_and_reset(cup.comp_stats(), 1);
    }
    let mut k = 0i32;
    while k <= cup.current_stage() as i32 {
        let stage = read_usize(cup.stages() + 4 * k as usize);
        if stage != 0 {
            let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(stage)));
            destroy(stage, 1);
        }
        k += 1;
    }
    cup.set_year(cup.year().wrapping_add(1));
    write_i32(this + league::CURRENT_STAGE, -1);
    let rules: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::SET_UP_RULES));
    let r = rules(this);
    if r == 0 {
        // simplification: the original returns the box's eax; the season turnover does not read it
        show_error_at(BRA_CHAMP_CUP_SOURCE, 0x1fc);
        return 0;
    }
    covered!("cx01_12 champions cup new season");
    cup.set_table(0);
    cup.set_club_count(0);
    cup.set_active(0);
    r
}

/// Comparer for `bra_champ_cup_create_group_stages`: only the original runs (the stages allocate and the news goes
/// out); the groups the copy plans before must be the clubs of the two stages built, and the stage 1.
pub(crate) unsafe extern "thiscall" fn compare_bra_champ_cup_create_group_stages(this: usize) -> u32 {
    let cup = League(this);
    let plan = bra_champ_cup_group_clubs(cup).map(|mut g| { g.sort_unstable(); g.to_vec() });
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_CHAMP_CUP_CREATE_GROUP_STAGES)(this);
    let built: Vec<Vec<usize>> = (0..2).map(|g| { let s = read_usize(cup.stages() + 4 * g); if s != 0 { stage_clubs(s) } else { Vec::new() } }).collect();
    let ok = r != 1 || (cup.current_stage() as i32 == 1 && built[0] == plan[0] && built[1] == plan[1]);
    SCOREBOARD.record(ok, || format!("bra_champ_cup_create_group_stages({:#x}): returned {}, plan {:x?}, built {:x?}", this, r, plan, built));
    r
}

/// Comparer for `bra_champ_cup_create_semi_final_stage`: only the original runs (the stage allocates and draws); the
/// semi-finalists the copy plans before must be the clubs of the knockout stage built.
pub(crate) unsafe extern "thiscall" fn compare_bra_champ_cup_create_semi_final_stage(this: usize) {
    let cup = League(this);
    let mut plan = bra_champ_cup_semi_finalists(cup).to_vec();
    plan.sort_unstable();
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_CHAMP_CUP_CREATE_SEMI_FINAL_STAGE)(this);
    let stage = read_usize(cup.stages() + 8);
    let built = if stage != 0 { stage_clubs(stage) } else { Vec::new() };
    SCOREBOARD.record(stage == 0 || built == plan, || format!("bra_champ_cup_create_semi_final_stage({:#x}): plan {:x?}, built {:x?}", this, plan, built));
}

/// Comparer for `bra_champ_cup_start_new_season`: only the original runs (`compare_new_season`: the year up by one,
/// three stages and the current one −1).
pub(crate) unsafe extern "thiscall" fn compare_bra_champ_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_CHAMP_CUP_START_NEW_SEASON);
    compare_new_season("bra_champ_cup_start_new_season", League(this), || o(this))
}

/// Writes a club's staff records for its status in the Champions Cup and returns 0.
///
/// `BraChampCup::update_staff_records_for_status(club, outcome, stage, _, fixture, extra)` (thiscall, ret 0x18). In the
/// preliminary (stage 0): qualified (outcome 6) gets `add_qualified_for_round_record` (stage 0, round 0xa, weight 40),
/// out (0xfe) `add_failed_to_qualify_record` (0x41c, weight 40). At the other stages below 2 (signed): qualified gets
/// round 0x82 (weight 40), out gets 0xa (weight 40). At stage 2 (the knockout): out (0xff) the knocked-out record with
/// the fixture's round prefix and name, weighted 50 to 100 by its round number of 2 (`scale_by_round`); through (0) the
/// qualified record with the round prefix and the next round's code in the knockout stage's round table, the same
/// weight; the winner (1) the cup winners record and the club as the CONMEBOL seeding's Champions Cup winner
/// (`set_bra_champ_cup_winner`); the runner-up (2) the runners-up record. Stages above 2 write nothing.
/// Original: `BraChampCup::update_staff_records_for_status` (0x426900, bra_champ_cup.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn bra_champ_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, _a4: u32, fixture: usize, extra: u32) -> u32 {
    let cup = League(this);
    let comp = cup.competition().addr();
    let (outcome, stage) = (outcome as u8, stage as u8 as i8);
    if stage < 2 {
        let (round, out_stage) = if stage == 0 { (0xa, 0x41c) } else { (0x82, 0xa) };
        match outcome {
            6 => {
                covered!("cx01_13 records: qualified");
                european_cup_add_qualified_record(club, comp, 0, round, 0x28);
            }
            0xfe => {
                covered!("cx01_13 records: failed to qualify");
                european_cup_add_failed_to_qualify_record(club, comp, out_stage, 0x28);
            }
            _ => {}
        }
        return 0;
    }
    if stage != 2 {
        return 0;
    }
    let game = Fixture(fixture);
    match outcome {
        0xff => {
            let weight = scale_by_round(0x32, 0x64, game.round_number(), 2);
            // simplification: the round prefix and name go in registers with stale high halves; 0x86c000 reads the words
            european_cup_add_knocked_out_record(club, comp, game.round_prefix() as u16 as u32, game.round_name() as u16 as u32, weight);
        }
        0 => {
            let weight = scale_by_round(0x32, 0x64, game.round_number(), 2);
            let knockout = Cup(read_usize(cup.stages() + 8));
            let next = read_i16(knockout.round_table() + (game.round_number() as isize as usize).wrapping_mul(cup_round::SIZE) + cup_round::SIZE + cup_round::ROUND_CODE);
            european_cup_add_qualified_record(club, comp, game.round_prefix() as u16 as u32, next as u16 as u32, weight);
        }
        1 => {
            covered!("cx01_13 records: champions cup winner");
            african_nations_record_final_result(0x86a800, club, fixture, extra); // Staff_Records::add_cup_winners_record
            set_bra_champ_cup_winner(read_usize(G_CONMEBOL_SEEDING), club);
        }
        2 => african_nations_record_final_result(0x86b0b0, club, fixture, extra), // Staff_Records::add_cup_runners_up_record
        _ => {}
    }
    0
}

/// Stores a club's current, best and worst position codes in the competition statistics (the club's entry,
/// `exe::comp_stats_find_club_entry`, not checked), turning the raw stage codes into positions: in the preliminary
/// (stage 0xff) the current and best codes 1 and 2 become 1 and the worst 7, a 3 becomes 9; in the groups (stages
/// below 2, signed) the current and best codes 1 and 2 become 1 and the worst 3, a 3 becomes 5 and a 4 becomes 7; from
/// stage 2 on the codes are stored as they are.
/// Original: `BraChampCup::set_club_progress` (0x426c20, bra_champ_cup.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn bra_champ_cup_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let entry = CompStatsClub(exe::comp_stats_find_club_entry(League(this).comp_stats(), club));
    let stage = stage as u8;
    let (current, best, worst) = (current as u8, best as u8, worst as u8);
    let codes = if stage == 0xff {
        covered!("cx01_13 progress in the preliminary");
        (bra_champ_cup_position_code(true, current, 1), bra_champ_cup_position_code(true, best, 1), bra_champ_cup_position_code(true, worst, 7))
    } else if (stage as i8) < 2 {
        covered!("cx01_13 progress in the groups");
        (bra_champ_cup_position_code(false, current, 1), bra_champ_cup_position_code(false, best, 1), bra_champ_cup_position_code(false, worst, 3))
    } else {
        (current, best, worst)
    };
    entry.set_best_position(codes.1);
    entry.set_current_position(codes.0);
    entry.set_worst_position(codes.2);
}

/// Writes the news text of a club's fate in the Champions Cup into Text `text`.
///
/// Same arguments as `comp_build_fate_news_text` (club, outcome, stage, the stage's name id, fixture, next stage, _,
/// verdict, body, text; thiscall, ret 0x28). In the preliminary (stage 0xff) and the groups (other stages below 2):
/// through (outcome 6) the body is the group qualification line (`exe::comp_util_write_group_qualify_news(club, stage,
/// competition, 6, body, text)`) and the headline "through to group phase" (from the preliminary) or "through to semi
/// final" (from the groups) with the club, linked; out (0xfe) the group line with the stage name id instead of the
/// stage (`exe::comp_util_write_group_qualify_news(club, name, competition, −2, body, text)`); other outcomes nothing.
/// From stage 2 on, the shared cup lines: the tie result in the body (`exe::comp_util_write_cup_tie_result_news`), the
/// win or defeat in the headline (`exe::comp_util_write_cup_win_or_defeat_news`).
/// Defect of the original [inference]: the qualified line passes the stage index where the out line passes the stage's
/// name id.
/// Original: `BraChampCup::build_fate_news_text` (0x425fd0, bra_champ_cup.cpp, thiscall, ret 0x28).
pub(crate) unsafe extern "thiscall" fn bra_champ_cup_build_fate_news_text(this: usize, club: i32, outcome: u32, stage: u32, name: u32, fixture: usize, next_stage: u32,
    _a7: u32, verdict: u32, body: u32, text: usize) {
    let comp = League(this).competition();
    let s = stage as u8;
    if s == 0xff || (s as i8) < 2 {
        match outcome as u8 {
            6 if body != 0 => {
                covered!("cx01_13 fate: group qualified");
                exe::comp_util_write_group_qualify_news(club as u32, stage, comp, 6, body, text);
            }
            6 => {
                covered!("cx01_13 fate: through headline");
                let lang_sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
                let c = club_at(club as isize);
                let cg = c.short_gender_name() as i8 as i32;
                let team = c.addr() + club::SHORT_NAME;
                // "through to group phase" from the preliminary, "through to semi final" from the groups
                lang_sprintf(0xde1f64, if s == 0xff { 0x98a25c } else { 0x98a224 }, cg, cg, team);
                exe::text_set_string(text, 0xde1f64 as *const u8);
                exe::text_add_link(text, team as *const u8, 0x7d5, c);
            }
            0xfe => {
                covered!("cx01_13 fate: group out");
                exe::comp_util_write_group_qualify_news(club as u32, name, comp, -2, body, text);
            }
            _ => {}
        }
        return;
    }
    if body != 0 {
        exe::comp_util_write_cup_tie_result_news(club as u32, fixture as u32, verdict, outcome, next_stage, text);
    } else {
        exe::comp_util_write_cup_win_or_defeat_news(club as u32, fixture as u32, outcome, text);
    }
}

/// Comparer for `bra_champ_cup_update_staff_records_for_status`: the records go to the managers' history and the
/// winner to the CONMEBOL seeding (they cannot run twice); only the original runs and it must answer 0.
pub(crate) unsafe extern "thiscall" fn compare_bra_champ_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, a4: u32, fixture: usize, extra: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_BRA_CHAMP_CUP_UPDATE_STAFF_RECORDS_FOR_STATUS)(this, club, outcome, stage, a4, fixture, extra);
    SCOREBOARD.record(r == 0, || format!("bra_champ_cup_update_staff_records_for_status({:#x}, club {:#x}, {}, stage {}): returned {}", this, club, outcome as u8, stage as u8 as i8, r));
    r
}

/// Comparer for `bra_champ_cup_set_club_progress`: the three position bytes of the club's entry side by side.
pub(crate) unsafe extern "thiscall" fn compare_bra_champ_cup_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(IDX_BRA_CHAMP_CUP_SET_CLUB_PROGRESS);
    let stats = League(this).comp_stats();
    let entry = if stats != 0 { exe::comp_stats_find_club_entry(stats, club) } else { 0 };
    if entry == 0 {
        return o(this, club, stage, current, best, worst);
    }
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&[(entry + comp_stats_club::CURRENT_POSITION, 3)], || o(this, club, stage, current, best, worst),
        || bra_champ_cup_set_club_progress(this, club, stage, current, best, worst));
    SCOREBOARD.record(same, || format!("bra_champ_cup_set_club_progress({:#x}, club {:#x}, stage {}, {}, {}, {}): original {:x?} rust {:x?}", this, club, stage as u8 as i8,
        current as u8, best as u8, worst as u8, mo, mr));
}

/// Comparer for `bra_champ_cup_build_fate_news_text` (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_bra_champ_cup_build_fate_news_text(this: usize, club: i32, outcome: u32, stage: u32, name: u32, fixture: usize, next_stage: u32,
    a7: u32, verdict: u32, body: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, u32, u32, u32, usize, u32, u32, u32, u32, usize)>(IDX_BRA_CHAMP_CUP_BUILD_FATE_NEWS_TEXT);
    compare_text_writer(text, || o(this, club, outcome, stage, name, fixture, next_stage, a7, verdict, body, text),
        |t| bra_champ_cup_build_fate_news_text(this, club, outcome, stage, name, fixture, next_stage, a7, verdict, body, t),
        || format!("bra_champ_cup_build_fate_news_text({:#x}, club {}, outcome {}, stage {}, body {})", this, club, outcome as u8, stage as u8 as i8, body));
}

/// The place of a state championship id in `BRA_CHAMP_CUP_STATE_PLACES`.
pub(crate) unsafe fn bra_champ_cup_state_place(id: u32) -> Option<usize> {
    BRA_CHAMP_CUP_STATE_PLACES.iter().find(|&&(key, _)| read_u32(key) == id).map(|&(_, place)| place)
}

/// The four clubs of each Champions Cup group: the preliminary's winner with entrants 3, 4 and 5 (group A), the
/// preliminary's runner-up with entrants 6, 7 and 8 (group B).
pub(crate) unsafe fn bra_champ_cup_group_clubs(cup: League) -> [[usize; 4]; 2] {
    let (table, book) = (cup.table(), cup.entrant_book());
    let row = |k: usize| read_usize(table + k * league_table_row::SIZE);
    let entrant = |k: usize| read_usize(book + k * 6);
    [[row(0), entrant(3), entrant(4), entrant(5)], [row(1), entrant(6), entrant(7), entrant(8)]]
}

/// Builds group `g` of the Champions Cup as stage `g`: its round table (`comp_vtable::CREATE_SCHEDULE` for stage g),
/// the League_Stage (operator new 0xee; `FN_LEAGUE_STAGE_LEAGUE_STAGE` with the cup, 4 clubs, `clubs`, 1, the record,
/// the table, the rounds, the win and draw points, the match category, the tie-breaks, `BRA_CHAMP_CUP_GROUP_PLACES`,
/// the year, g, the stage label, the importance, 1, 0, the league flags, −1, 0, 2) into stage g and the round table
/// freed. Returns the stage (null when it was not built).
pub(crate) unsafe fn bra_champ_cup_build_group(cup: League, g: u32, clubs: &[usize; 4]) -> usize {
    let schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(cup.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut label) = (0i16, 0i16);
    let table = schedule(cup.addr(), g, &mut rounds, &mut label, 0);
    let p = exe::crt_operator_new(0xee);
    let mut stage = 0usize;
    if p != 0 {
        type LeagueStageCtor = unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize;
        let ctor: LeagueStageCtor = core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
        // simplification: the original pushes the rounds, the label, the year and the flags as dwords with stale high
        // bytes; League_Stage::League_Stage reads the words and bytes
        stage = ctor(p, cup.addr(), 4, clubs.as_ptr() as usize, 1, cup.competition().addr(), table, rounds as u16 as u32, cup.win_points() as u32,
            cup.draw_points() as u32, cup.match_category() as u32, cup.addr() + league::TIEBREAKS, BRA_CHAMP_CUP_GROUP_PLACES.as_ptr() as usize,
            cup.year() as u16 as u32, g, label as u16 as u32, cup.importance() as u32, 1, 0, cup.league_flags() as u16 as u32, 0xffff_ffff, 0, 2);
    }
    write_usize(cup.stages() + 4 * g as usize, stage);
    exe::crt_free(table);
    read_usize(cup.stages() + 4 * g as usize)
}

/// The four semi-finalists in draw order: group A's winner, group B's runner-up, group A's runner-up and group B's
/// winner (the first two rows of each group's table).
pub(crate) unsafe fn bra_champ_cup_semi_finalists(cup: League) -> [usize; 4] {
    let row = |g: usize, k: usize| read_usize(League(read_usize(cup.stages() + 4 * g)).table() + k * league_table_row::SIZE);
    [row(0, 0), row(1, 1), row(0, 1), row(1, 0)]
}

/// The clubs of a stage's table (league rows) or team table (cup), as a sorted list.
pub(crate) unsafe fn stage_clubs(stage: usize) -> Vec<usize> {
    let s = League(stage);
    let mut v: Vec<usize> = if s.comp_type() == 1 {
        (0..s.club_count().max(0) as usize).map(|k| read_usize(Cup(stage).team_table() + 6 * k)).collect()
    } else {
        (0..s.club_count().max(0) as usize).map(|k| read_usize(s.table() + k * league_table_row::SIZE)).collect()
    };
    v.sort_unstable();
    v
}

/// A club's position code in the Champions Cup statistics from a raw stage code: in the preliminary 1 and 2 give
/// `first_two`, 3 gives 9; in the groups 1 and 2 give `first_two`, 3 gives 5 and 4 gives 7; 0 and the other codes stay.
pub(crate) fn bra_champ_cup_position_code(preliminary: bool, code: u8, first_two: u8) -> u8 {
    match (preliminary, code) {
        (_, 1 | 2) => first_two,
        (true, 3) => 9,
        (false, 3) => 5,
        (false, 4) => 7,
        _ => code,
    }
}

/// The Champions Cup's vtable.
pub(crate) const BRA_CHAMP_CUP_VTABLE: usize = 0x967c44;

/// "E:\dev\CM3\cm3 00-01\cm3\code\comp\cups\bra_champ_cup.cpp", for the error boxes.
pub(crate) const BRA_CHAMP_CUP_SOURCE: usize = 0x98a1e8;

/// `BraChampCup::set_up_rules` (0x426230, bra_champ_cup.cpp, thiscall, ret 0; 0 on failure).
pub(crate) const FN_BRA_CHAMP_CUP_SET_UP_RULES: usize = 0x426230;

/// `compare_clubs_by_reputation_desc_nulls_last` (0x4b07d0, comp_util.cpp, cdecl): the qsort order by reputation.
pub(crate) const FN_COMPARE_CLUBS_BY_REPUTATION_DESC_NULLS_LAST: usize = 0x4b07d0;

/// The nine places of the Champions Cup and the long names of the clubs named for each; the name tests run in this
/// order for every Brazilian club, a later match overwriting the place (Vasco and Vila Nova both go to place 0).
pub(crate) const BRA_CHAMP_CUP_NAMED_CLUBS: [(usize, usize); 10] = [
    (0x98a398, 0), // "Associação Desportiva Vasco da Gama"
    (0x98a380, 0), // "Vila Nova Futebol Clube"
    (0x98a364, 1), // "Clube Náutico Capibaribe"
    (0x98a344, 2), // "São Raimundo Esporte Clube (AM)"
    (0x98a328, 3), // "América Futebol Clube (MG)"
    (0x98a304, 4), // "Grêmio Foot-Ball Porto-Alegrense"
    (0x98a2e4, 5), // "Clube de Regatas do Flamengo"
    (0x98a2c8, 6), // "Clube Atlético Paranaense"
    (0x98a2a8, 7), // "Sport Club Corinthians Paulista"
    (0x98a294, 8), // "Esporte Clube Bahia"
];

/// The Brazilian state championships whose strongest club fills an empty place, with that place.
pub(crate) const BRA_CHAMP_CUP_STATE_PLACES: [(usize, usize); 12] = [
    (KEY_CLUB_COMP_BRAZILIAN_BAHIA_STATE_CHAMPIONSHIP, 8),
    (KEY_CLUB_COMP_BRAZILIAN_CENTRAL_STATE_CHAMPIONSHIP, 0),
    (KEY_CLUB_COMP_BRAZILIAN_GAUCHO_STATE_CHAMPIONSHIP, 6),
    (KEY_CLUB_COMP_BRAZILIAN_GOIAS_STATE_CHAMPIONSHIP, 1),
    (KEY_CLUB_COMP_BRAZILIAN_MINAS_GERAIS_STATE_CHAMPIONSHI, 5),
    (KEY_CLUB_COMP_BRAZILIAN_NORTH_STATE_CHAMPIONSHIP, 0),
    (KEY_CLUB_COMP_BRAZILIAN_NORTHEAST_STATE_CHAMPIONSHIP, 0),
    (KEY_CLUB_COMP_BRAZILIAN_PARANA_STATE_CHAMPIONSHIP, 7),
    (KEY_CLUB_COMP_BRAZILIAN_PERNAMBUCO_STATE_CHAMPIONSHIP, 2),
    (KEY_CLUB_COMP_BRAZILIAN_RIO_DE_JANEIRO_STATE_CHAMPIONS, 3),
    (KEY_CLUB_COMP_BRAZILIAN_SANTA_CATARINA_STATE_CHAMPIONS, 1),
    (KEY_CLUB_COMP_BRAZILIAN_SAO_PAULO_STATE_CHAMP, 4),
];

/// The predicted final positions of the nine entrants, in entrant order.
pub(crate) const BRA_CHAMP_CUP_PREDICTED: [u8; 9] = [1, 2, 3, 3, 5, 5, 7, 7, 9];

/// `BraChampCup::create_group_stages` (0x426470, bra_champ_cup.cpp, thiscall, ret 0).
pub(crate) const FN_BRA_CHAMP_CUP_CREATE_GROUP_STAGES: usize = 0x426470;

/// `BraChampCup::create_semi_final_stage` (0x426720, bra_champ_cup.cpp, thiscall, ret 0).
pub(crate) const FN_BRA_CHAMP_CUP_CREATE_SEMI_FINAL_STAGE: usize = 0x426720;

/// `BraChampCup::create_preliminary_table` (0x426130, bra_champ_cup.cpp, thiscall, ret 0).
pub(crate) const FN_BRA_CHAMP_CUP_CREATE_PRELIMINARY_TABLE: usize = 0x426130;

/// The places of a Champions Cup group (the first two go through), passed to League_Stage::League_Stage.
pub(crate) const BRA_CHAMP_CUP_GROUP_PLACES: [u8; 4] = [2, 0, 0, 0];
