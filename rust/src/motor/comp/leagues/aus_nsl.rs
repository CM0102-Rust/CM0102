use super::*;

/// Club in row i of the table (+0xb1, 0x3b-byte rows) of league or stage t.
pub(crate) unsafe fn aus_nsl_table_row_club(t: League, i: usize) -> usize {
    read_usize(t.table() + i * league_table_row::SIZE)
}

/// The NSL has the regular season (round robin), the finals series (stage 0) and the grand final (stage 1): the next
/// stage moves the current stage on and builds the finals series at the end of the regular season and the grand final
/// at the end of the finals series; past the last stage it is an error (the box)
/// Original: `AusNsl::advance_stage` (0x411f70, aus_nsl.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn aus_nsl_advance_stage(this: usize) {
    let this = League(this);
    let stage = this.current_stage() as i32;
    if stage >= (this.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(0x987c20, 0x2f0);
    }
    let stage = stage + 1;
    this.set_current_stage((stage) as usize);
    if stage == 0 {
        let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_AUSNSL_CREATE_FINALS_SERIES_STAGE);
        f(this.addr());
    } else if stage == 1 {
        let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_AUSNSL_CREATE_GRAND_FINAL_STAGE);
        f(this.addr());
    }
}

/// The NSL finals series: the top six of the regular season play their own round-robin stage (with the league points
/// and tie-breakers) and its top two go to the grand final; sends the draw news.
///
/// `AusNsl::create_finals_series_stage()` (SEH frame only for the new), called at the end of the regular season. 1) The
/// top six of the league table (rows 0 to 5, in order) in a local vector of 6. 2) The round table through virtual +0x3c
/// (create_round_table: thiscall(this, 0, &rounds, &extra, 0), ret 0x10). 3) The stage: operator new `exe::crt_operator_new`(0xee) and
/// the League_Stage constructor 0x68b100 (thiscall, ret 0x58, 22 arguments, in the original's order: the league, 6, the
/// vector, 1, +4, the round table, the rounds (word), the points +0xc2 and +0xc3, the category +0xc4, &+0xc5,
/// &{2,0,0,0} (two top places: the two who go to the grand final), the year +0x40 (word), 0, the extra (word), +0x51,
/// 1, 0, +0xd9 (word), −1, 0, 2; the original pushes the words and bytes with garbage in the high bytes, which the
/// constructor does not read: the copy passes clean values). Out of memory: 0. 4) [+0xc][0] = the stage and the round
/// table freed (free `exe::crt_free`). No stage: the error box at line 0x328 and 0. With it: the draw news (`send_group_draw_news` on the
/// stage, thiscall, ret 0), +0x30 = 0 and 1.
///
/// [inference] The finals series is a short league of the six; its top two go to the grand final. The real NSL used a
/// double-chance knockout.
/// Original: `AusNsl::create_finals_series_stage` (0x412010, aus_nsl.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn create_finals_series_stage(this: usize) -> u32 {
    let this = League(this);
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, *const usize, u32, usize, usize, u32, u32, u32, u32, usize, *const u8, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let slots: [u8; 4] = [2, 0, 0, 0];
    let mut clubs = [0usize; 6];
    for (i, c) in clubs.iter_mut().enumerate() {
        *c = aus_nsl_table_row_club(this, i); // cqr3: the top six of the table
    }
    let table: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = table(this.addr(), 0, &mut rodadas, &mut extra, 0);
    let p = exe::crt_operator_new(0xee);
    let stage = if p != 0 {
        construct(p, this.addr(), 6, clubs.as_ptr(), 1, this.competition().addr(), t, rodadas as u16 as u32, this.win_points() as u32,
                  this.draw_points() as u32, this.match_category() as u32, this.addr() + league::TIEBREAKS, slots.as_ptr(), this.year() as u16 as u32, 0,
                  extra as u16 as u32, this.importance() as u32, 1, 0, this.league_flags() as u16 as u32, -1, 0, 2)
    } else {
        0
    };
    write_usize(this.stages(), stage);
    exe::crt_free(t);
    let stage = read_usize(this.stages());
    if stage == 0 {
        show_error_at(0x987c20, 0x328);
        return 0;
    }
    covered!("cqr_3 final series");
    exe::league_send_group_draw_news(stage);
    this.set_current_stage((0) as usize);
    1
}

/// The NSL grand final: the top two of the finals series in a two-club cup stage; the winner is the Australian
/// champion.
///
/// `AusNsl::create_grand_final_stage()` (SEH frame only for the new), called at the end of the finals series: the top
/// two of the finals series table ([+0xc][0], rows 0 and 1, in that order) in a list of 2 (malloc `exe::crt_malloc` of 8; out of
/// memory: the error box at line 0x348 and return); the round table through virtual +0x3c (thiscall(this, 1, &rounds,
/// &extra, 0), ret 0x10); the stage (operator new `exe::crt_operator_new`(0xb2) and the Cup_Stage constructor `construct_cup_stage`, thiscall, ret
/// 0x3c: the league, 2, the list, the rounds, +4, the table, the year +0x40, 1, 1, the extra, 0x14, 0, 0, 0 and null
/// seeds; the copy passes clean words) into [+0xc][1] and the table freed (free `exe::crt_free`). No stage: the error box at
/// line 0x35c and return (without freeing the list). With it: Cup::set_up_round (`set_up_round`, thiscall(stage, 0), ret 4)
/// and the list freed. The current stage (+0x30 = 1) was already written by the next stage.
///
/// [inference] The grand final is a single game, with the finals series winner as the home side.
/// Original: `AusNsl::create_grand_final_stage` (0x4121c0, aus_nsl.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn create_grand_final_stage(this: usize) {
    let this = League(this);
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, usize) -> usize =
        core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
    let list = exe::crt_malloc(8);
    if list == 0 {
        return show_error_at(0x987c20, 0x348);
    }
    let series = read_usize(this.stages());
    write_usize(list, aus_nsl_table_row_club(League(series), 0)); // cqr3: the first of the final series
    write_usize(list + 4, aus_nsl_table_row_club(League(series), 1)); // cqr3: the second of the final series
    let table: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = table(this.addr(), 1, &mut rodadas, &mut extra, 0);
    let p = exe::crt_operator_new(0xb2);
    let stage = if p != 0 {
        construct(p, this.addr(), 2, list, rodadas as u16 as u32, this.competition().addr(), t, this.year() as u16 as u32, 1, 1,
                  extra as u16 as u32, 0x14, 0, 0, 0, 0)
    } else {
        0
    };
    write_usize(this.stages() + 4, stage);
    exe::crt_free(t);
    let stage = read_usize(this.stages() + 4);
    if stage == 0 {
        return show_error_at(0x987c20, 0x35c);
    }
    covered!("cqr_3 grand final");
    exe::cup_set_up_round(stage, 0);
    exe::crt_free(list);
}

/// Writes the NSL rule fields (3 rounds, points, tiebreaks, 6 finalists...) before the schedule.
pub(crate) unsafe fn set_nsl_rules(o: usize) {
    write_u16_at(o, 0xab, 0);
    write_u8_at(o, 0xc8, 0);
    write_u8_at(o, 0xbf, 0);
    write_u8_at(o, 0xc0, 0);
    write_u8_at(o, 0xc1, 0);
    write_u8_at(o, 0xc4, 2);
    write_u8_at(o, 0xc6, 2);
    write_u8_at(o, 0x52, 2);
    write_u16_at(o, 0x3c, 3); // three shifts
    write_u8_at(o, 0xc2, 3); // cqj8: the NSL points per win
    write_u8_at(o, 0x4a, 3);
    write_u8_at(o, 0xc3, 1);
    write_u8_at(o, 0x42, 1);
    write_u8_at(o, 0xc5, 1); // tiebreakers: goal difference, goals for, away goals
    write_u8_at(o, 0xc7, 4);
    write_u8_at(o, 0xbe, 6); // the six of the final stage
    write_u32_at(o, 0x1c, 0xffff_ffff);
    write_u32_at(o, 0x20, 0xffff_ffff);
    write_u16_at(o, 0xd9, 0x28); // 0x8 fase seguinte, 0x20 eliminados marcados
    write_u8_at(o, 0x49, 4);
}

/// Writes the MLS rule fields (no schedule; 32 games, fixed round order...).
pub(crate) unsafe fn set_mls_rules(o: usize) {
    write_u8_at(o, 0xc4, 2);
    write_u8_at(o, 0xc6, 2);
    write_u8_at(o, 0xc2, 3);
    write_u8_at(o, 0xc8, 0);
    write_u8_at(o, 0xbf, 0);
    write_u8_at(o, 0xc0, 0);
    write_u8_at(o, 0xc1, 0);
    write_u8_at(o, 0xc3, 1); // cqj8: the MLS points per draw
    write_u16_at(o, 0xab, 0xc2);
    write_u8_at(o, 0x42, 1);
    write_u8_at(o, 0xc5, 1); // tiebreakers: goal difference, goals scored, head-to-head
    write_u8_at(o, 0xc7, 3);
    write_u8_at(o, 0xbe, 1);
    write_u32_at(o, 0x1c, 0xffff_ffff);
    write_u32_at(o, 0x20, 0xffff_ffff);
    write_u16_at(o, 0xd9, 0x48); // 0x8 next stage, 0x40 rounds in fixed order
    write_u16_at(o, 0x3c, 4);
    write_u16_at(o, 0xa7, 0x20); // 32 jogos por time
}

/// NSL: three round robins; win 3, draw 1; tie-break by goal difference, goals scored and away goals; the top 6 go to
/// the finals series (rule word 0x28: 0x8 next stage, no champion band, and 0x20, clubs without a chance marked
/// eliminated); nobody goes up or down; 4 bench substitutes and 3 substitutions; fixed squad numbers. Then the schedule
/// Original: `AusNsl::set_up_rules` (0x411ce0, aus_nsl.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn aus_nsl_set_up_rules(this: usize) -> u32 {
    let this = League(this);
    set_nsl_rules(this.addr());
    covered!("cqj_8 nsl rules");
    let vt = read_usize(this.addr());
    let f: unsafe extern "thiscall" fn(usize, i32, usize, usize, usize) -> usize = core::mem::transmute(read_usize(vt + 0x3c));
    let r = f(this.addr(), -1, this.addr() + league::ROUND_COUNT, this.addr() + league::STAGE_ID, 0);
    write_u32_at(this.addr(), 0xba, r as u32);
    1
}

/// Calls record_season_honours (vtable +0x30) on the first stage [[this+0xc]].
pub(crate) unsafe fn record_first_stage_honours(this: League) -> u32 {
    let e = read_usize(this.stages());
    let vt = read_usize(e);
    let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(vt + 0x30));
    f(e)
}

/// NSL: at the end of the regular season, the six clubs through to the finals series get the qualification record
/// (stage 0x435, weight 0x28); in the finals, those who go through get the record (round 0x96) and those who fall the
/// failed-to-qualify one; the champion (grand final) gets the champion record. No fate written to the table
/// Original: `AusNsl::update_staff_records_for_status` (0x411980, aus_nsl.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn aus_nsl_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    let _ = (a4, a5, a6);
    nsl_end_of_stage(this, club, status, stage, true).0
}

/// `AusNsl::create_table()`: the Australian NSL has 13 clubs (+0x57 = [0x9cf89c]), those of the division in database
/// order, until it is full (the counter in an int, compared as a word).
///
/// Not full: the error box at line 0x252 and 0; full: 1, with no prize fund nor TV money. Source 0x987c20 (the
/// out-of-memory box at line 0x241).
/// Original: `AusNsl::create_table` (0x411b70, aus_nsl.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn aus_nsl_create_table(this: usize) -> u32 {
    let this = League(this);
    if alloc_league_table(this, 0xd, 0x987c20, 0x241) == 0 {
        return 0;
    }
    let mut k: u32 = 0;
    let mut i: i16 = 0;
    let mut club = read_usize(G_CLUB_TABLE);
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        if (k as i16) >= this.club_count() {
            break;
        }
        if is_in_division(Club(club), KEY_CLUB_COMP_AUSTRALIAN_NATIONAL_SOCCER_LEAGUE) {
            jap_j1_init_table_row(this, k as u8, this.club_count(), 0, club); // cqm9: the NSL row
            k = k.wrapping_add(1);
        }
        i = i.wrapping_add(1);
        club = club.wrapping_add(club::SIZE);
    }
    if k as i16 != this.club_count() {
        show_error_at(0x987c20, 0x252);
        return 0;
    }
    covered!("cqm_9 nsl full");
    1
}

