// ---------- batch %%LEVA%%: the Coppa Italia: the predicted positions, the clubs' records and the news of a club's fate (ita_cup.cpp) ----------

/// The predicted positions of the Italian cups (0x640210 and 0x63e580, the same body; only the source file, the box
/// lines, the knockout stage and the band width change). Rule: the entrants by reputation; from the final backwards,
/// the clubs of each knockout round share the place of the losers of that round (the winner 1 and the runner-up 2,
/// then 3 for the semi-finalists, 5 for the quarter-finalists...), and the rest go in bands of `band`.
///
/// Without statistics (+8): the box `lines.0` and return. A copy of the entrant list (+0xee, +0xf2 clubs, a dword)
/// (`exe::crt_malloc`(4 × n); out of memory: the box `lines.1` and return), sorted by reputation (`exe::crt_qsort`(v, n,
/// 4, 0x4b07d0): 4 pushes; add esp -> cdecl); the round table of the knockout stage (virtual
/// `comp_vtable::CREATE_SCHEDULE`(this, `stage`, &rounds, &extra, 0): push 0; push &extra; push &rounds; push stage;
/// mov ecx -> thiscall, ret 0x10). Two byte counters start at 1: `next` (the next place) and `place` (the one being
/// given). From the last round down to round 0, with the word +0x1a of the round record (`cup_round::SIZE` bytes: the
/// round's ties): in the last round first, that many clubs get `place` (still 1: the winner) and `next` goes up by one
/// each; then `place` = `next` and that many clubs get it (`next` up by one each). The clubs taken in order from the
/// sorted copy, with a word index and no bound check (as the original). Then `place` = `next` and each remaining club
/// (index below the dword +0xf2) gets `place`; `next` goes up by one, and when `next` equals `place` + `band` (in 32
/// bits: a band past 255 never closes), `place` = `next`. Each place goes through Comp_Stats::set_club_final_position
/// (`exe::comp_stats_set_club_final_position`(stats, club, place): push place; push club; mov ecx, [+8] -> thiscall, ret
/// 8; the original pushes the place in a dword with stack garbage above the byte, which the routine does not read: the
/// copy passes the clean byte). The round table and the copy are freed (`exe::crt_free`).
unsafe fn italian_cup_predicted_positions(this: usize, file: usize, lines: (u32, u32), stage: u32, band: u32, label: &'static str) {
    let stats = read_usize(this + 8);
    if stats == 0 {
        return show_error_at(file, lines.0);
    }
    let v = exe::crt_malloc(read_u32(this + 0xf2) << 2);
    if v == 0 {
        return show_error_at(file, lines.1);
    }
    if read_i32(this + 0xf2) > 0 {
        let mut i = 0i16;
        loop {
            write_usize(v + 4 * i as usize, read_usize(read_usize(this + 0xee) + 4 * i as usize));
            i = i.wrapping_add(1);
            if i as i32 >= read_i32(this + 0xf2) {
                break;
            }
        }
    }
    exe::crt_qsort(v, read_u32(this + 0xf2), 4, 0x4b07d0);
    let create_schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut extra) = (0i16, 0i16);
    let table = create_schedule(this, stage, &mut rounds, &mut extra, 0);
    let (mut next, mut place) = (1u8, 1u8);
    let mut idx = 0i16;
    let give = |place: u8, idx: &mut i16, next: &mut u8| {
        exe::comp_stats_set_club_final_position(stats, read_usize(v + 4 * *idx as usize), place as u32);
        *idx = idx.wrapping_add(1);
        *next = next.wrapping_add(1);
    };
    let last = rounds.wrapping_sub(1);
    if last >= 0 {
        covered!(label);
        let mut r = last as i32;
        while r >= 0 {
            let ties = table.wrapping_add((r as usize).wrapping_mul(cup_round::SIZE)) + 0x1a;
            if r == rounds as i32 - 1 {
                let mut k = 0i16;
                while k < read_i16(ties) {
                    give(place, &mut idx, &mut next); // cqx7: the final's winner
                    k += 1;
                }
            }
            place = next;
            let mut k = 0i16;
            while k < read_i16(ties) {
                give(place, &mut idx, &mut next);
                k += 1;
            }
            r -= 1;
        }
    }
    place = next;
    while (idx as i32) < read_i32(this + 0xf2) {
        exe::comp_stats_set_club_final_position(stats, read_usize(v + 4 * idx as usize), place as u32);
        next = next.wrapping_add(1);
        if next as u32 == place as u32 + band {
            place = next;
        }
        idx = idx.wrapping_add(1);
    }
    exe::crt_free(table);
    exe::crt_free(v);
}

/// The predicted position (`comp_stats_club::FINAL_POSITION`) of each entrant (+0xee, +0xf2) in the cup's statistics
/// (+8), 0 without entry.
unsafe fn italian_cup_positions(this: usize) -> Vec<u8> {
    let n = read_i32(this + 0xf2).clamp(0, 0x200) as usize;
    (0..n).map(|i| {
        let c = read_usize(read_usize(this + 0xee) + 4 * i);
        let e = if c != 0 { exe::comp_stats_find_club_entry(read_usize(this + 8), c) } else { 0 };
        if e != 0 { read_u8(e + comp_stats_club::FINAL_POSITION) } else { 0 }
    }).collect()
}

/// The comparer of the Italian cups' predicted positions, side by side over the same entries: the original first
/// (Comp_Stats::set_club_final_position adds the entry of a club that has none), then the copy, which finds the entries
/// already there and only rewrites `comp_stats_club::FINAL_POSITION` (the same list, the same qsort: the same order;
/// the round table made again and freed); the position of each entrant after the copy must equal the original's.
/// Without statistics or list: only the original.
unsafe fn compare_italian_cup_predicted(name: &str, this: usize, copy: unsafe extern "thiscall" fn(usize), o: impl FnOnce()) {
    if read_usize(this + 8) == 0 || read_usize(this + 0xee) == 0 || read_i32(this + 0xf2) <= 0 {
        return o();
    }
    o();
    let original = italian_cup_positions(this);
    copy(this);
    let after = italian_cup_positions(this);
    SCOREBOARD.record(after == original, || format!("{}({:#x}): copy {:?} original {:?}", name, this, after, original));
}

/// The predicted positions of the Coppa Italia (at creation): the 48 entrants by reputation; the knockout of stage 7
/// gives 1 and 2 to the final's pair, then a shared place to the losers of each round; the rest in bands of 8
/// (`italian_cup_predicted_positions`; the boxes at lines 0x438 and 0x440).
/// Original: `ItaCup::set_predicted_positions` (0x640210, ita_cup.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn ita_cup_set_predicted_positions(this: usize) {
    italian_cup_predicted_positions(this, 0x9ea23c, (0x438, 0x440), 7, 8, "cqx_7 coppa italia predicted") // cqx7: the Coppa Italia bands of 8
}

/// Comparer for `ita_cup_set_predicted_positions` (`compare_italian_cup_predicted`, side by side).
unsafe extern "thiscall" fn compare_ita_cup_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    compare_italian_cup_predicted("ita_cup_set_predicted_positions", this, ita_cup_set_predicted_positions, || o(this));
}

/// The body of the two Italian cups' staff records (0x63ff00 and 0x63e270; only the knockout stage `limit` and the
/// group qualification round change), `update_staff_records_for_status(club, outcome, stage, _, fixture, final)` (the
/// virtual +0x44); returns 0. Stage (signed byte) below `limit`, the groups: outcome 6, the qualification
/// (`european_cup_add_qualified_record`: stage 0, round `group_round`, weight 0x28); 0xfe, the failure
/// (`european_cup_add_failed_to_qualify_record`: 0x41c, 0x28). Stage `limit`, the knockout, weight =
/// `scale_by_round`(0x32, 0x64, the fixture's `round_number()`, 5): 0xff, the elimination
/// (`european_cup_add_knocked_out_record`: the fixture's `round_prefix()` and `round_name()`, the weight); 0, the
/// qualification with `round_prefix()` and the name word (+0x6f) of the fixture's round in the round table of the
/// knockout stage ([`cup::STAGES`][`limit`] + `cup::ROUND_TABLE`; the high 16 bits the original pushes are the stage
/// pointer's); 1, the winner (Staff_Records 0x86a800); 2, the runner-up (0x86b0b0). Anything else: nothing. The
/// staff records are [`G_STAFF_RECORDS`] (thiscall: 0x86bdd0 ret 0x14, 0x86c1d0 ret 0x10, 0x86c000 ret 0x14,
/// 0x86a800 and 0x86b0b0 ret 0xc); words pushed with garbage in the high bits are passed clean (the callees read the
/// word).
unsafe fn italian_cup_staff_records(this: usize, club: usize, outcome: u32, stage: u32, game: usize, final_: u32, limit: i8, group_round: u32, label: &'static str) -> u32 {
    let game = Fixture(game);
    let (d, f) = (outcome as u8, stage as u8 as i8);
    let comp = read_usize(this + 4);
    if f < limit {
        if d == 6 {
            covered!(label);
            european_cup_add_qualified_record(club, comp, 0, group_round, 0x28);
        } else if d == 0xfe {
            european_cup_add_failed_to_qualify_record(club, comp, 0x41c, 0x28);
        }
    } else if f == limit {
        match d {
            0xff => {
                let weight = scale_by_round(0x32, 0x64, game.round_number(), 5);
                european_cup_add_knocked_out_record(club, comp, game.round_prefix() as u16 as u32, game.round_name() as u16 as u32, weight);
            }
            0 => {
                let weight = scale_by_round(0x32, 0x64, game.round_number(), 5);
                let knockout = read_usize(read_usize(this + cup::STAGES) + limit as usize * 4);
                let r = game.round_number() as isize;
                let name = read_u16(read_usize(knockout + cup::ROUND_TABLE).wrapping_add((r * cup_round::SIZE as isize) as usize) + 0x6f) as u32;
                european_cup_add_qualified_record(club, comp, game.round_prefix() as u16 as u32, (knockout as u32 & 0xffff_0000) | name, weight);
            }
            1 => cup_record_final_result(0x86a800, club, game, final_),
            2 => cup_record_final_result(0x86b0b0, club, game, final_),
            _ => {}
        }
    }
    0
}

/// Rule: in the Coppa Italia, the clubs that get through the groups (stages below 7) get the qualified record (round
/// 0xa, weight 0x28) and those that do not the failed record; in the knockout (stage 7) elimination and qualification
/// weigh by round (50 to 100 over the 5 rounds), and the winner and runner-up get the final records
/// (`italian_cup_staff_records`).
/// Original: `ItaCup::update_staff_records_for_status` (0x63ff00, ita_cup.cpp, thiscall, ret 0x18).
unsafe extern "thiscall" fn ita_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, _a4: u32, game: usize, final_: u32) -> u32 {
    italian_cup_staff_records(this, club, outcome, stage, game, final_, 7, 0xa, "cqx_7 coppa italia group qualified")
}

// simplification: the records go to the history (and the reputation and morale): they cannot run twice. Only the
// original; the plan: return 0.
unsafe extern "thiscall" fn compare_ita_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, a4: u32, game: usize, final_: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(%%I1%%)(this, club, outcome, stage, a4, game, final_);
    SCOREBOARD.record(r == 0, || format!("ita_cup_update_staff_records_for_status({:#x}, {}, {}) returned {}", club, outcome as u8, stage as u8, r));
    r
}

/// Rule: the Coppa Italia news of a club's fate: in the groups (stages below 7) qualification (outcome 6) and
/// elimination (0xfe) go through the generic cup news; a club going through with no news object gets the text
/// "through to first round" with a link to the club; in the knockout (stage 7) the generic knockout news.
///
/// `ItaCup::build_fate_news_text(club_id, outcome, stage, a4, a5, a6, a7, a8, news, text)` (10 dwords, ret 0x28).
/// Stage (signed byte) below 7: outcome 6 with `news`: cdecl 0x4b4590(club_id, a4, the competition +4, 6, news, text)
/// (6 pushes; add esp, 0x18); outcome 6 without `news`: the club (`club_at`(club_id)) gives the text
/// (`exe::langlib_sprintf`(0xde1f64, 0x9ea270 "{}<%s - Team Name(e.g.Ajax)>{} through to first round", its byte +0x52
/// twice, its name +0x38): 5 pushes; add esp, 0x14 -> cdecl) set in `text` (Text::set_string 0x4ae660(text, 0xde1f64),
/// thiscall, ret 4) with the link (Text::add_link 0x4ae8a0(text, name, 0x7d5, club), thiscall, ret 0xc); outcome 0xfe:
/// 0x4b4590(club_id, a4, competition, −2, news, text); other outcomes nothing. Stage 7: with `news`, cdecl
/// 0x4b0b80(club_id, a5, a8, outcome, a6, text); without, cdecl 0x4b4420(club_id, a5, outcome, text). Stages above 7:
/// nothing.
/// Original: `ItaCup::build_fate_news_text` (0x63f130, ita_cup.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn ita_cup_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, _a7: u32, a8: u32, news: u32, text: usize) {
    let (d, f) = (outcome as u8, stage as u8 as i8);
    let cup_news: unsafe extern "C" fn(u32, u32, usize, i32, u32, usize) = core::mem::transmute(0x4b4590usize);
    let comp = read_usize(this + 4);
    if f < 7 {
        if d == 6 {
            if news != 0 {
                return cup_news(club_id, a4, comp, 6, news, text);
            }
            covered!("cqx_7 coppa italia through to first round");
            let club = read_usize(G_CLUB_TABLE).wrapping_add((club_id as usize).wrapping_mul(club::SIZE));
            let b = read_u8(club + 0x52) as i8 as i32;
            let sprintf: unsafe extern "C" fn(usize, usize, i32, i32, usize) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
            sprintf(0xde1f64, 0x9ea270, b, b, club + 0x38);
            let set_string: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TEXT_SET_STRING);
            set_string(text, 0xde1f64);
            let add_link: unsafe extern "thiscall" fn(usize, usize, u32, usize) = core::mem::transmute(FN_TEXT_ADD_LINK);
            add_link(text, club + 0x38, 0x7d5, club); // cqx7: the club link
        } else if d == 0xfe {
            cup_news(club_id, a4, comp, -2, news, text);
        }
    } else if f == 7 {
        if news != 0 {
            let knockout_news: unsafe extern "C" fn(u32, u32, u32, u32, u32, usize) = core::mem::transmute(0x4b0b80usize);
            knockout_news(club_id, a5, a8, outcome, a6, text);
        } else {
            let knockout_text: unsafe extern "C" fn(u32, u32, u32, usize) = core::mem::transmute(0x4b4420usize);
            knockout_text(club_id, a5, outcome, text);
        }
    }
}

/// Comparer for `ita_cup_build_fate_news_text`. On the paths that only write the text (a club going through the groups
/// without a news object: the "through to first round" text and its link) side by side with `compare_text_writer`: the
/// original into the caller's Text, the copy into a local one. On the paths that pass the news object to the generic
/// cup news (0x4b4590, 0x4b0b80, 0x4b4420) only the original (they may also write the news object, which takes its
/// text once).
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn compare_ita_cup_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize)>(%%I2%%);
    if (stage as u8 as i8) < 7 && outcome as u8 == 6 && news == 0 && text != 0 {
        return compare_text_writer(text, || o(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text),
            |t| ita_cup_build_fate_news_text(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, t),
            || format!("ita_cup_build_fate_news_text(club {}, outcome {}, stage {})", club_id, outcome as u8, stage as u8 as i8));
    }
    o(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text)
}
