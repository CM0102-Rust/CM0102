// ---------- rx11b_2: the reserve match result news: who gets it and its text (record_utils.cpp) ----------

/// The news type of a reserve match result.
const RESERVE_RESULT_NEWS: u32 = 0x25;

/// The manager who gets one side's reserve result (`reserve_result_news`), or None.
///
/// A side with its side mark set (`fixture::HOME_SIDE_MARK`/`AWAY_SIDE_MARK`, `marked`) needs a linked team
/// (`club_get_linked_b_team` with the output) whose output is 0. The manager: the club's own when it has a human manager
/// (`news_club_has_human_manager`), else the human manager of its linked team (`club_get_linked_b_team` without the
/// output); none: None. A marked side is skipped when that manager controls the reserve team
/// (`HUMAN_MANAGER::controls_reserve_team`).
unsafe fn reserve_result_manager(club: Club, marked: bool) -> Option<Staff> {
    if marked {
        let mut linked = 0u32;
        if exe::club_get_linked_b_team(club, &mut linked, 1) == 0 || linked != 0 {
            return None;
        }
    }
    let manager = if exe::news_club_has_human_manager(read_usize(G_NEWS), club) != 0 {
        club.manager()
    } else {
        let linked = exe::club_get_linked_b_team(club, core::ptr::null_mut(), 1);
        if linked == 0 || exe::news_club_has_human_manager(read_usize(G_NEWS), linked) == 0 {
            return None;
        }
        Club(linked).manager()
    };
    if marked && exe::human_manager_controls_reserve_team(read_usize(G_HUMAN_MANAGER), manager) != 0 {
        return None;
    }
    Some(manager)
}

/// The reserve result news items of a fixture, handed to `send` (the item, the manager's club) home side first.
///
/// For each side (`reserve_result_manager`): a news item of type `RESERVE_RESULT_NEWS` with the side's match record
/// (`fill_match_record_from_fixture`, `write_match_record_to_news`), then three slots: the side's mark clear, the other
/// side's mark clear and the stadium id (`fixture::STADIUM`, −1 none).
unsafe fn reserve_result_news(game: Fixture, send: &mut dyn FnMut(usize, Club)) {
    let sides = [(game.home_club(), game.home_side_mark(), game.away_side_mark() as u8),
                 (game.away_club(), game.away_side_mark() as u8, game.home_side_mark())];
    for (club, own, other) in sides {
        let Some(manager) = reserve_result_manager(club, own != 0) else { continue };
        covered!("rx11b_2 reserve result news");
        let mut item = [0u32; 0x38]; // the local news item (0xe0 bytes)
        let news = item.as_mut_ptr() as usize;
        exe::new_newsitem(news, RESERVE_RESULT_NEWS, 0);
        let mut record = [0u8; 0x20];
        exe::fill_match_record_from_fixture(game, club, record.as_mut_ptr() as usize);
        let mut k = 0u8;
        exe::write_match_record_to_news(record.as_ptr() as usize, news, &mut k);
        exe::newsitem_set_slot(news, k as i8, (own == 0) as u32);
        exe::newsitem_set_slot(news, k.wrapping_add(1) as i8, (other == 0) as u32); // rx11b_2: the other side's mark
        let stadium = game.stadium();
        exe::newsitem_set_slot(news, k.wrapping_add(2) as i8, if stadium != 0 { read_u32(stadium) } else { u32::MAX });
        send(news, manager.club_job());
    }
}

/// Sends the result of a reserve match to the human managers of the two clubs (or of their linked teams):
/// `reserve_result_news` and `News::send_to_club_manager` to the manager's club. National teams (the home club,
/// `database_is_national_team`) get nothing. Returns 1.
/// Original: `ReserveComp::add_result` (0x7e6f70, record_utils.cpp, thiscall, ret 8; slot 0x4 of the ReserveComp
/// vtable 0x96ef54; the second argument is not read).
pub(crate) unsafe extern "thiscall" fn reserve_comp_add_result(_this: usize, game: usize, _unused: u32) -> u32 {
    let game = Fixture(game);
    if exe::database_is_national_team(game.home_club()) != 0 {
        return 1;
    }
    reserve_result_news(game, &mut |news, club| {
        exe::news_send_to_club_manager(read_usize(G_NEWS), news, club);
    });
    1
}

/// The reserve match result's text: the headline ("<club> win", "draw" or "lose") or the body ("<club> have beaten
/// <opponent> <score> at <stadium>.", "have drawn <score> with", "have lost <score> to"; without "at <stadium>" when the
/// stadium slot is −1).
///
/// The news item: the match record (`read_match_record_from_news`, `BanMatch`), then the club's mark-clear slot (its
/// name as the first team when set: `club_get_national_team_name`, the last two arguments 1, 1 in the headline and 1, 0
/// in the body), the opponent's and the stadium id (`G_STADIUM_TABLE`, 0x4e bytes, its name at +4 and gender at +0x37).
/// The score is the larger goal count first (0x9c116c). The links: the score when the match statistics still match
/// (`Match_Stats::fixture_entry_matches`, 0x7d3), the club by its built name and the opponent by its short name (0x7d6
/// for a first team, 0x7d5 otherwise). Genders sign-extended, twice each, in the original's order.
/// Original: `reserve_result_news_text` (0x7e7280, record_utils.cpp, thiscall, ret 0xc; this is the reserve
/// competition, not read).
pub(crate) unsafe extern "thiscall" fn reserve_result_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let fmt = news_text_format();
    let slot = |k: u8| exe::newsitem_get_slot(news, k as u32);
    let mut k = 0u8;
    let mut m = BanMatch([0u8; 0x20]);
    m.0[..8].copy_from_slice(&BAN_DEFAULT_DATE);
    exe::read_match_record_from_news(news, &mut k, m.0.as_mut_ptr());
    let club = Club(m.club());
    let (gf, ga) = m.goals();
    let link = |first_team: u32| if first_team != 0 { 0x7d6 } else { 0x7d5 };
    let mut name = [0u8; 0x12c];
    let np = name.as_mut_ptr() as usize;
    let cg = club.short_gender_name() as i8 as i32;
    if full == 0 {
        covered!("rx11b_2 reserve result headline");
        let own = slot(k);
        exe::club_get_national_team_name(club, own as usize, np, 0x12c, 1, 1);
        let template = if gf > ga { 0xa7a49c } else if gf == ga { 0xa7a46c } else { 0xa7a43c }; // "<club> win/draw/lose"
        fmt(NEWS_TEXT_BUFFER, template, cg, cg, np);
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        exe::text_add_link(out, name.as_ptr(), link(own), club);
        return;
    }
    covered!("rx11b_2 reserve result body");
    let (own, other, stadium) = (slot(k), slot(k.wrapping_add(1)), slot(k.wrapping_add(2)));
    let opponent = Club(m.opponent());
    let mut opponent_name = [0u8; 0x12c];
    let op = opponent_name.as_mut_ptr() as usize;
    exe::club_get_national_team_name(club, own as usize, np, 0x12c, 1, 0);
    exe::club_get_national_team_name(opponent, other as usize, op, 0x12c, 1, 0);
    let og = opponent.short_gender_name() as i8 as i32;
    let mut score = [0u8; 0xc];
    let sp = score.as_mut_ptr() as usize;
    fmt(sp, 0x9c116c, gf.max(ga) as i32, gf.min(ga) as i32); // "<more>-<fewer>"
    if stadium != u32::MAX {
        let st = read_usize(G_STADIUM_TABLE).wrapping_add((stadium as usize).wrapping_mul(0x4e));
        let (sg, sn) = (read_i8(st + 0x37) as i32, st + 4); // the stadium's gender and name
        if gf > ga {
            fmt(NEWS_TEXT_BUFFER, 0xa7a7b0, cg, cg, og, og, sg, sg, np, op, sp, sn); // "have beaten .. at"
        } else if gf == ga {
            fmt(NEWS_TEXT_BUFFER, 0xa7a704, cg, cg, og, og, sg, sg, np, sp, op, sn); // "have drawn .. with .. at"
        } else {
            fmt(NEWS_TEXT_BUFFER, 0xa7a65c, cg, cg, og, og, sg, sg, np, sp, op, sn); // "have lost .. to .. at"
        }
    } else if gf > ga {
        fmt(NEWS_TEXT_BUFFER, 0xa7a5d8, cg, cg, og, og, np, op, sp); // "have beaten"
    } else {
        let template = if gf == ga { 0xa7a550 } else { 0xa7a4cc }; // "have drawn .. with", "have lost .. to"
        fmt(NEWS_TEXT_BUFFER, template, cg, cg, og, og, np, sp, op);
    }
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    if exe::match_stats_fixture_entry_matches(read_usize(G_MATCH_STATS), m.fixture() as i32, read_i32(m.at(0x11)),
        read_i32(m.at(0x15)), read_i32(m.at(0)), read_i32(m.at(2))) != 0 {
        exe::text_add_link(out, score.as_ptr(), 0x7d3, m.fixture() as usize);
    }
    exe::text_add_link(out, name.as_ptr(), link(own), club);
    exe::text_add_link(out, (opponent.addr() + club::SHORT_NAME) as *const u8, link(other), opponent);
}

/// Comparer for `reserve_comp_add_result`: the news goes to the managers' inboxes, so only the original runs. The plan
/// first: the items the copy would send (the manager's club and the item's type and slots). After the original, each
/// planned item for a human manager (one of the last 16 people) must be among the newest of his inbox ring (News +8:
/// start, next slot; 100 items of 0xdf bytes), with the same type and the same written slots; and the return is 1.
pub(crate) unsafe extern "thiscall" fn compare_reserve_comp_add_result(this: usize, game: usize, unused: u32) -> u32 {
    let mut planned: Vec<(usize, Vec<u8>)> = Vec::new();
    if exe::database_is_national_team(Fixture(game).home_club()) == 0 {
        reserve_result_news(Fixture(game), &mut |news, club| {
            // the type (+0) and the slots the routine writes (the match record and three more, at most 20 slots)
            let bytes = core::slice::from_raw_parts(news as *const u8, news_item::DATA + 4 * 20).to_vec();
            planned.push((club.addr(), bytes));
        });
    }
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I0%%)(this, game, unused);
    let news_obj = read_usize(G_NEWS);
    let mut missing = Vec::new();
    let mut taken: Vec<usize> = Vec::new();
    for (club, bytes) in &planned {
        let manager = Club(*club).manager();
        if manager.is_null() {
            continue;
        }
        let idx = read_i32(manager.addr()).wrapping_sub(read_i32(G_STAFF_COUNT)).wrapping_add(0x10);
        if idx < 0 {
            continue; // not a human: nothing delivered
        }
        let e = read_usize(news_obj + news_fields::HUMAN_INBOXES) + idx as usize * 12;
        let (start, next) = (read_i32(e), read_i32(e + 4));
        let found = (1..=planned.len() as i32).any(|back| {
            let mut s = next - back;
            if s < start {
                s += 0x64;
            }
            let item = read_usize(news_obj).wrapping_add((s as usize).wrapping_mul(NEWS_NEWS_RECORD_SIZE));
            let stored = core::slice::from_raw_parts(item as *const u8, bytes.len());
            if !taken.contains(&item) && stored[..4] == bytes[..4] && stored[news_item::DATA..] == bytes[news_item::DATA..] {
                taken.push(item);
                true
            } else {
                false
            }
        });
        if !found {
            missing.push(*club);
        }
    }
    SCOREBOARD.record(r == 1 && missing.is_empty(), || format!("reserve_comp_add_result(fixture {:#x}): return {}, planned items not delivered to the clubs {:x?}",
        game, r, missing));
    r
}

/// Comparer for `reserve_result_news_text`: the original into the caller's Text, the copy into a local one
/// (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_reserve_result_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I1%%);
    compare_text_writer(out, || o(this, news, full, out), |t| reserve_result_news_text(this, news, full, t),
        || format!("reserve_result_news_text(news {:#x}, full {})", news, full));
}
