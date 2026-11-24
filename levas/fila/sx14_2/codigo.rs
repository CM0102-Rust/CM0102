// ---------- batch %%LEVA%%: the news text of the board's reaction to a match (manager_manager.cpp) ----------

/// The headline of the board's match reaction (news 0x138c): the derby headlines (marks 0x40) have no club; the
/// others quote the opponent, except the fury one, which quotes the club itself.
unsafe fn board_reaction_headline(marks: u32, club: Club, other: Club) {
    if marks & 0x40 != 0 {
        let template = if marks & 8 != 0 {
            0xa0bd0c // "Board delighted at derby showing"
        } else if marks & 0x20 != 0 {
            0xa0bcec // "Board happy at derby display"
        } else if marks & 1 != 0 {
            0xa0bccc // "Board furious at derby display"
        } else if marks & 2 != 0 {
            0xa0bca8 // "Board unhappy with derby display"
        } else {
            0xa0bc88 // "Board reaction to derby match"
        };
        story_lang!(NEWS_TEXT_BUFFER, template);
        return;
    }
    let (template, quoted) = if marks & 8 != 0 {
        (0xa0bc48, other) // "Board delighted after <club> game"
    } else if marks & 0x20 != 0 {
        (0xa0bc0c, other) // "<club> match pleases board"
    } else if marks & 1 != 0 {
        (0xa0bbcc, club) // "Board fury at <club> performance"
    } else if marks & 2 != 0 && marks & 0x1000 != 0 {
        (0xa0bb8c, other) // "Board unhappy after <club> defeat"
    } else {
        (0xa0bb50, other) // "Board reaction to <club> game"
    };
    let g = quoted.short_gender_name() as i8 as i32;
    story_lang!(NEWS_TEXT_BUFFER, template, g, g, quoted.addr() + club::SHORT_NAME);
}

/// Writes the headline or the body of the board's reaction to a match (news 0x138c, posted by
/// `post_board_match_reaction_news`).
///
/// `news_board_match_reaction_text(news, body, out)`. The headline (`board_reaction_headline`) from slots 0 (the
/// marks), 1 (the club) and 2 (the opponent), with the links to the two clubs' short names (0x7d5). The body reads
/// the 18 slots in order (the marks, the clubs, the score, the aggregate, the fixture's stats slot, the three named
/// players, the variant, the defeat, the run, the target, the current and best positions and the league flag) and
/// the match record behind them (`exe::read_match_record_from_news`, a `RecordMatch`); the score "<higher>-<lower>"
/// (0xa0bd30) and the verdict (`FN_MANAGER_MANAGER_BUILD_BOARD_MATCH_REACTION_TEXT`, with the record's competition)
/// into the game's text buffer, which goes into `out` (`exe::text_set_string`); the links: the two clubs' names
/// (0x7d5), the competition's short name (0x7d0), each named player still set (0x7db, the name by
/// `exe::database_format_staff_name`, mode 1) and the score (0x7d3) when the fixture is still in the match stats
/// (`RecordMatch::link_score`).
/// Original: `MANAGER_MANAGER::news_board_match_reaction_text` (0x6a6310, manager_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn news_board_match_reaction_text(this: usize, news: usize, body: u32, out: usize) {
    if body == 0 {
        covered!("sx14_2 headline");
        let (marks, club, other) = (exe::newsitem_get_slot(news, 0), exe::newsitem_get_slot(news, 1), exe::newsitem_get_slot(news, 2));
        let (club, other) = (club_at(club as i32 as isize), club_at(other as i32 as isize));
        board_reaction_headline(marks, club, other);
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        exe::text_add_link(out, (club.addr() + club::SHORT_NAME) as *const u8, 0x7d5, club);
        exe::text_add_link(out, (other.addr() + club::SHORT_NAME) as *const u8, 0x7d5, other);
        return;
    }
    covered!("sx14_2 body");
    let mut k: u8 = 0;
    let mut slots = [0u32; 18];
    for v in slots.iter_mut() {
        *v = exe::newsitem_get_slot(news, k as i8 as u32);
        k += 1;
    }
    let mut record = [0u8; 0x20];
    let rec = record.as_mut_ptr() as usize;
    exe::date_date_54a110(rec);
    exe::read_match_record_from_news(news, &mut k, record.as_mut_ptr());
    let record = RecordMatch(rec);
    let [marks, club, other, goals_a, goals_b, aggregate_for, aggregate_against, _stats_slot, id1, id2, id3, variant, loss, run, target, current, best, in_league] = slots;
    let (club, other) = (club_at(club as i32 as isize), club_at(other as i32 as isize));
    let (goals_a, goals_b) = (goals_a as i32, goals_b as i32);
    let mut score = [0u8; 100];
    story_lang!(score.as_mut_ptr(), 0xa0bd30, goals_a.max(goals_b), goals_a.min(goals_b)); // "<goals>-<opp goals>"
    let ids = [id1 as i32, id2 as i32, id3 as i32];
    let verdict: unsafe extern "thiscall" fn(usize, usize, usize, usize, i32, usize, i32, i32, usize, u32, u32, u32, u32, u32, u32, u32) -> u32 =
        core::mem::transmute(FN_MANAGER_MANAGER_BUILD_BOARD_MATCH_REACTION_TEXT);
    verdict(this, club.addr(), other.addr(), record.comp(), marks as i32, score.as_ptr() as usize, aggregate_for as i32, aggregate_against as i32,
            ids.as_ptr() as usize, variant, loss, run, target, current, best, in_league);
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    exe::text_add_link(out, club.name_addr() as *const u8, 0x7d5, club);
    exe::text_add_link(out, other.name_addr() as *const u8, 0x7d5, other);
    let comp = record.comp();
    exe::text_add_link(out, (comp + competition::SHORT_NAME) as *const u8, 0x7d0, comp);
    let mut name = [0u8; 200];
    for id in ids {
        if id != -1 {
            let p = staff_at(id as isize);
            exe::database_format_staff_name(p, name.as_mut_ptr() as usize, 200, 1);
            exe::text_add_link(out, name.as_ptr(), 0x7db, p);
        }
    }
    record.link_score(out, &score);
}

/// Comparer for `news_board_match_reaction_text`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_news_board_match_reaction_text(this: usize, news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I0%%);
    compare_text_writer(out, || o(this, news, body, out), |t| news_board_match_reaction_text(this, news, body, t),
                        || format!("news_board_match_reaction_text(news {:#x}, body {})", news, body));
}
