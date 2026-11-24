// ---------- sbG_4: the replies to a friendly tournament's invitations (friendly.cpp) ----------

/// Friendly::send_tournament_invitations (thiscall(tournament), ret 4) and Friendly::pick_replacement_invitee
/// (thiscall(tournament, place), ret 8), called by their addresses (both in the cx05_21 item).
pub(crate) const FN_FRIENDLY_SEND_INVITATIONS_AT: usize = 0x5c5310;
pub(crate) const FN_FRIENDLY_PICK_REPLACEMENT_AT: usize = 0x5c6350;

/// MiniCup::MiniCup (thiscall, ret 0x18) and MiniLeague::MiniLeague (thiscall, ret 0x2c), called by their addresses
/// (cx05_12 and cx05_13).
pub(crate) const FN_MINI_CUP_AT: usize = 0x765bf0;
pub(crate) const FN_MINI_LEAGUE_AT: usize = 0x766500;

/// The short-name tag every friendly tournament's competition record gets ("Fr", .data 0x9cd0f0).
pub(crate) const FRIENDLY_TOURNAMENT_TAG: usize = 0x9cd0f0;

/// Whether an invited club with a pending invitation answers now: a human club (`exe::human_manager_is_human_club`(club,
/// 0, 1, 0, 0)) only when it is the `club` asked about; an AI club when it is that club or with `force`.
pub(crate) unsafe fn friendly_club_answers_now(c: usize, force: u32, club: usize) -> bool {
    if exe::human_manager_is_human_club(c, 0, 1, 0, 0) != 0 { c == club } else { force != 0 || c == club }
}

/// What an invited club `club` answers to a tournament hosted by `host`: no if it is not free on the tournament's dates
/// (`exe::friendly_is_club_free_on_date`(this, start date, club, `tournament_slot::KIND`)) or either club has no nation.
/// Otherwise the pull of the trip: 1.65 within the same nation, 1.52625 abroad, times 0.85 for a different region
/// (`nation::ACTUAL_REGION`) and again for a different continent (`nation::CONTINENT`); when both clubs' stadiums have a
/// city, times (the invited club's `club::MIN_ATTENDANCE` / ((the distance in degrees × 0.55 + 1) × 60000) + 1). The club
/// says no only when the host's reputation (`club::REPUTATION`, signed) is below the invited club's divided by the pull
/// and the gap between them is above the pull × 600 (x87 compares: an unordered result counts as "below").
pub(crate) unsafe fn friendly_tournament_reply_accepted(this: usize, tournament: usize, host: usize, club: usize) -> bool {
    if exe::friendly_is_club_free_on_date(this, (tournament + tournament_slot::START_DATE) as *mut [u8; 8], club, read_u8(tournament + tournament_slot::KIND) as u32) == 0 {
        return false;
    }
    let (theirs, ours) = (read_usize(club + club::NATION), read_usize(host + club::NATION));
    if theirs == 0 || ours == 0 {
        return false;
    }
    let mut pull = if theirs == ours { read_f64(rdata::F64_1_65) } else { read_f64(0x96af18) }; // sbG_4: 1.52625 abroad
    if read_u8(theirs + nation::ACTUAL_REGION) != read_u8(ours + nation::ACTUAL_REGION) {
        pull *= read_f64(rdata::F64_0_85);
    }
    if read_usize(theirs + nation::CONTINENT) != read_usize(ours + nation::CONTINENT) {
        pull *= read_f64(rdata::F64_0_85);
    }
    let city = |c: usize| if Club(c).stadium().0 != 0 { Club(c).stadium().city() } else { 0 };
    let (far, near) = (city(club), city(host));
    if far != 0 && near != 0 {
        let d_lon = City(far).longitude() - City(near).longitude();
        let d_lat = City(far).latitude() - City(near).latitude();
        let distance = (d_lat * d_lat + d_lon * d_lon).sqrt();
        let one = read_f64(rdata::F64_1);
        pull *= read_i32(club + club::MIN_ATTENDANCE) as f64 / ((distance * read_f64(rdata::F64_0_55) + one) * read_f64(0x96af10)) + one; // 60000
    }
    let (theirs, ours) = (read_i16(club + club::REPUTATION) as i32, read_i16(host + club::REPUTATION) as i32);
    let refuses = !(ours as f64 >= theirs as f64 / pull) && !(pull * read_f64(rdata::F64_600) >= theirs.wrapping_sub(ours) as f64);
    !refuses
}

/// Processes the replies to a friendly tournament's invitations and, when every invited club has accepted, creates the
/// tournament's competition.
///
/// For each invited place k (1 up to `tournament_slot::TEAM_COUNT`, a signed byte) with a club and a pending invitation
/// (`tournament_slot::INVITE_PENDING` 1) that answers now (`friendly_club_answers_now`). The answer (`friendly_tournament_reply_accepted`): yes marks the place 3;
/// no empties it (club and mark 0) and, when the host is not human, a replacement is picked (Friendly::pick_replacement_invitee
/// 0x5c6350(tournament, k)) and the invitations go out again (Friendly::send_tournament_invitations 0x5c5310). Either
/// way the host's manager gets news 10 (`exe::new_newsitem`(10, 0)): the host's id, the club's id, the start day and
/// year (`tournament_slot::START_DATE`), 3 (yes) or 4 (no) and the slot index (`tournament_slot::SLOT_INDEX`).
///
/// With every place accepted and no competition yet (`tournament_slot::COMPETITION`), each place (the host's too) whose
/// club is there twice is emptied, and so is each one whose club is no longer free on the dates. If still all accepted:
/// the competition record (id = the Friendly's `friendly::FIRST_TOURNAMENT_COMP_ID` + the slot index) gets the name in
/// `competition::NAME` and `competition::SHORT_NAME`, the tag "Fr" at +0x53, the id, the host nation's continent, the
/// host's nation when every invited club shares it (otherwise none), the host's colours and reputation 0. Kind 1 builds
/// a MiniCup (operator new 0xbc; MiniCup::MiniCup(places, record, start date, year, sides, flags)); any other a MiniLeague
/// (0xf6; MiniLeague::MiniLeague(count, places, record, start date, the +0xd byte, 1, tie-breaks {+0xe byte, 1, 2, 3},
/// four zero promotion bytes, year, sides, flags)); sides 1 with a common nation, 2 with only a continent, 4 otherwise;
/// flags `tournament_slot::COMP_FLAGS_A` | `COMP_FLAGS_B`, with 2 added when A is 0x80. None: the error box of
/// friendly.cpp (0x9cd0a0) line 0xac0 (cup) or 0xaeb (league). The competition goes into `tournament_slot::COMPETITION`
/// and [`G_COMP_OBJECT_TABLE`]; a cup has its first round set up (`exe::cup_set_up_round`(cup, 0), even when null).
/// Original: `Friendly::process_tournament_replies` (0x5c5820, friendly.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn friendly_process_tournament_replies(this: usize, tournament: usize, force: u32, club: usize) {
    let t = tournament;
    let host = read_usize(t + tournament_slot::HOST);
    let host_nation = read_usize(host + club::NATION);
    let continent = if host_nation != 0 { read_usize(host_nation + nation::CONTINENT) } else { 0 };
    let place = |k: i8| t + tournament_slot::INVITED_CLUBS + k as usize * 4;
    let mark = |k: i8| t + tournament_slot::INVITE_PENDING + k as usize;
    let count = || read_i16(t + tournament_slot::TEAM_COUNT);
    let mut accepted = 0i8;
    let mut k = 1i8;
    while (k as i16) < count() {
        let c = read_usize(place(k));
        if c != 0 && read_u8(mark(k)) == 1 {
            if !friendly_club_answers_now(c, force, club) {
                k = k.wrapping_add(1);
                continue;
            }
            covered!("sbG_4 tournament reply");
            let code: u8 = if friendly_tournament_reply_accepted(this, t, host, c) {
                write_u8(mark(k), 3); // sbG_4: accepted
                3
            } else {
                write_usize(place(k), 0);
                write_u8(mark(k), 0);
                if exe::human_manager_is_human_club(host, 0, 1, 0, 0) == 0 {
                    let pick: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_FRIENDLY_PICK_REPLACEMENT_AT);
                    pick(this, t, k as u8 as u32);
                    let invite: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_FRIENDLY_SEND_INVITATIONS_AT);
                    invite(this, t);
                }
                4
            };
            let mut news = [0u32; 0x38];
            let n = news.as_mut_ptr() as usize;
            exe::new_newsitem(n, 0xa, 0);
            exe::newsitem_set_slot(n, 0, read_u32(host));
            exe::newsitem_set_slot(n, 1, read_u32(c));
            exe::newsitem_set_slot(n, 2, read_i16(t + tournament_slot::START_DATE) as i32 as u32);
            exe::newsitem_set_slot(n, 3, read_u16(t + tournament_slot::START_DATE + 2) as u32);
            exe::newsitem_set_slot(n, 4, code as u32);
            exe::newsitem_set_slot(n, 5, read_u8(t + tournament_slot::SLOT_INDEX) as i8 as u32);
            exe::news_send_to_club_manager(read_usize(G_NEWS), n, host);
        }
        if read_usize(place(k)) != 0 && read_u8(mark(k)) == 3 {
            accepted = accepted.wrapping_add(1);
        }
        k = k.wrapping_add(1);
    }
    let all_in = |accepted: i8| accepted as i32 == count() as i32 - 1 && read_usize(t + tournament_slot::COMPETITION) == 0;
    if all_in(accepted) && count() > 0 {
        let mut k = 0i8;
        while (k as i16) < count() {
            let n = count();
            if (0..n).any(|j| j as i8 != k && read_usize(place(k)) == read_usize(place(j as i8))) {
                write_usize(place(k), 0);
                write_u8(mark(k), 0);
                accepted = accepted.wrapping_sub(1);
            }
            let c = read_usize(place(k));
            if c != 0 && exe::friendly_is_club_free_on_date(this, (t + tournament_slot::START_DATE) as *mut [u8; 8], c, read_u8(t + tournament_slot::KIND) as u32) == 0 {
                write_usize(place(k), 0);
                write_u8(mark(k), 0);
                accepted = accepted.wrapping_sub(1);
            }
            k = k.wrapping_add(1);
        }
    }
    if all_in(accepted) {
        covered!("sbG_4 tournament created");
        create_friendly_tournament_comp(this, t, host, host_nation, continent);
    }
}

/// The competition of a friendly tournament whose invitations were all accepted (the second half of
/// `friendly_process_tournament_replies`): its record filled, the MiniCup or MiniLeague built and registered.
pub(crate) unsafe fn create_friendly_tournament_comp(this: usize, t: usize, host: usize, host_nation: usize, continent: usize) {
    let count = read_i16(t + tournament_slot::TEAM_COUNT);
    let mut k = 1i8;
    let mut common = host_nation;
    while (k as i16) < count {
        if read_usize(read_usize(t + tournament_slot::INVITED_CLUBS + k as usize * 4) + club::NATION) != host_nation {
            common = 0;
        }
        k = k.wrapping_add(1);
    }
    let id = (read_i16(this + friendly::FIRST_TOURNAMENT_COMP_ID) as i32).wrapping_add(read_u8(t + tournament_slot::SLOT_INDEX) as i8 as i32);
    let record = read_usize(G_CLUB_COMP_TABLE).wrapping_add((id as usize).wrapping_mul(0x6b));
    copy_c_string(record + competition::NAME, t + tournament_slot::NAME);
    copy_c_string(record + competition::SHORT_NAME, t + tournament_slot::NAME);
    copy_c_string(record + 0x53, FRIENDLY_TOURNAMENT_TAG); // the short tag
    write_i32(record, id);
    write_usize(record + competition::CONTINENT, continent);
    write_usize(record + competition::NATION, common);
    write_u32(record + competition::FOREGROUND_COLOUR, read_u32(host + club::FORE_COLOUR1));
    write_u32(record + competition::BACKGROUND_COLOUR, read_u32(host + club::BACK_COLOUR1));
    write_i16(record + competition::REPUTATION, 0);
    let a = read_u16(t + tournament_slot::COMP_FLAGS_A);
    let flags = (a | read_u16(t + tournament_slot::COMP_FLAGS_B)) as u32 | if a == 0x80 { 2 } else { 0 };
    let sides: u32 = if common != 0 { 1 } else if continent != 0 { 2 } else { 4 }; // sbG_4: the sides byte
    // simplification: the original pushes the sides byte, the year word and the flags word in dwords with stack and
    // register leftovers above them; the constructors read only the low byte or word
    let (year, start, places) = (read_u16(t + tournament_slot::START_DATE + 2) as u32, t + tournament_slot::START_DATE, t + tournament_slot::INVITED_CLUBS);
    let comp = if read_u8(t + tournament_slot::KIND) == 1 {
        let p = exe::crt_operator_new(0xbc);
        let r = if p != 0 {
            let build: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32) -> usize = core::mem::transmute(FN_MINI_CUP_AT);
            build(p, places, record, start, year, sides, flags)
        } else {
            0
        };
        write_usize(t + tournament_slot::COMPETITION, r);
        if r == 0 {
            show_error_at(0x9cd0a0, 0xac0);
        }
        r
    } else {
        let tiebreaks = [read_u8(t + 0xe), 1, 2, 3]; // +0xe: the first tie-break code
        let promotion = [0u8; 4];
        let p = exe::crt_operator_new(0xf6);
        let r = if p != 0 {
            let build: unsafe extern "thiscall" fn(usize, u32, usize, usize, usize, u32, u32, usize, usize, u32, u32, u32) -> usize = core::mem::transmute(FN_MINI_LEAGUE_AT);
            build(p, count as u16 as u32, places, record, start, read_u8(t + 0xd) as u32, 1, tiebreaks.as_ptr() as usize, promotion.as_ptr() as usize, year, sides, flags) // +0xd: the win points
        } else {
            0
        };
        write_usize(t + tournament_slot::COMPETITION, r);
        if r == 0 {
            show_error_at(0x9cd0a0, 0xaeb);
        }
        r
    };
    write_usize(read_usize(G_COMP_OBJECT_TABLE) + id as usize * 4, comp);
    if read_u8(t + tournament_slot::KIND) == 1 {
        exe::cup_set_up_round(comp, 0);
    }
}

/// Comparer for `friendly_process_tournament_replies`: it sends news, picks replacements and builds a competition, so
/// only the original runs; then the plan, with the answers computed before it (`friendly_tournament_reply_accepted` on
/// each pending place that answers): a yes leaves the club in its place marked 3 (or the place emptied by the final
/// check), a no does not.
pub(crate) unsafe extern "thiscall" fn compare_friendly_process_tournament_replies(this: usize, tournament: usize, force: u32, club: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I0%%);
    let t = tournament;
    let host = read_usize(t + tournament_slot::HOST);
    let n = read_i16(t + tournament_slot::TEAM_COUNT).clamp(0, 4) as usize;
    let plan: Vec<(usize, usize, bool)> = (1..n)
        .filter_map(|k| {
            let c = read_usize(t + tournament_slot::INVITED_CLUBS + k * 4);
            if c == 0 || read_u8(t + tournament_slot::INVITE_PENDING + k) != 1 || !friendly_club_answers_now(c, force, club) {
                return None;
            }
            Some((k, c, friendly_tournament_reply_accepted(this, t, host, c)))
        })
        .collect();
    o(this, tournament, force, club);
    let wrong: Vec<(usize, usize, bool)> = plan.iter().copied()
        .filter(|&(k, c, yes)| {
            let (now, marked) = (read_usize(t + tournament_slot::INVITED_CLUBS + k * 4), read_u8(t + tournament_slot::INVITE_PENDING + k) == 3);
            // a yes may still be emptied by the final check (the club twice, or no longer free)
            if yes { !(now == c && marked || now == 0) } else { now == c && marked }
        })
        .collect();
    SCOREBOARD.record(wrong.is_empty(), || format!("friendly_process_tournament_replies({:#x}): the answers differ (place, club, rust yes) {:x?}", t, wrong));
}
