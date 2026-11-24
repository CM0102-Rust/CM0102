// ---------- sx04b_1: the month award's winner news and its text (month_award.cpp) ----------

/// The news type of a month award's winner (category 2, written by `AWARD_MANAGER::write_award_news`).
const MONTH_AWARD_NEWS_TYPE: u32 = 0x1b;
/// The version mark the news item carries in slot 9; the text writer rewords nothing while it is there.
const MONTH_AWARD_NEWS_VERSION: u32 = 0x420e9;
/// A month award's staff competition record (`TStaffComp`, award_fields +5): +4 name, +0x37 its gender, +0x38 short
/// name, +0x52 its gender, +0x57 nation.
const STAFF_COMP_NATION: usize = 0x57;

/// Posts the news of the winner of one month slot of a month award.
///
/// The news item (type `MONTH_AWARD_NEWS_TYPE`, local) carries: 0 the award's nation id (`award_base::NATION_ID`)
/// and 1 its index in the nation's awards (`award_base::INDEX`), which `AWARD_MANAGER::write_award_news` uses to find
/// the award again; 2 the winner (`month_award_fields::SLOT_WINNER_ID`) and 3..5 the `names::ID` of his first, second and common names (read without
/// a null check, as the original does); 6 the club id (`SLOT_CLUB_ID`); 7 the month (signed byte) and 8 the year
/// (unsigned word) of the slot; 9 `MONTH_AWARD_NEWS_VERSION`; 10 a draw `utils_random(3)` that picks the text
/// template; 11 whether the winner has no player sheet; with a sheet, 12 his position mask
/// (`exe::formation_get_position_mask`) and 13 his strengths (`exe::staff_get_player_strength_flags` with the slot's
/// club); 14 whether he is a player-manager (`job_for_club()`). Then the receivers: with a transfer manager
/// (`G_TRANSFER_MANAGER`) and a player sheet, the clubs interested in the player
/// (`exe::transfer_manager_collect_player_news_recipients`, kind 0x48, up to 30). The item goes (`news_post`) to the
/// managers of the award's league (`award_base::CLUB_COMP`), or, without one, to those of the staff competition's
/// nation (`award_base::STAFF_COMP`, `STAFF_COMP_NATION`); then to each interested club not playing in the award's
/// league (`club::DIVISION`).
/// Defect of the original: with a player sheet and no club (slot club id −1) the strengths are read with the record
/// before the club table (`G_CLUB_TABLE` − `club::SIZE`); the copy does the same.
/// Original: `MONTH_AWARD::post_winner_news` (0x767330, month_award.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn month_award_post_winner_news(this: usize, slot: u32) {
    month_award_winner_news(this, slot, &mut |item, to| news_post(item, to));
}

/// The winner news of `month_award_post_winner_news`, its items going to `sink` (the game's `news_post`, or a
/// comparer's rehearsal).
pub(crate) unsafe fn month_award_winner_news(this: usize, slot: u32, sink: NewsSink) {
    let v = this.wrapping_add((slot as u8 as i8 as isize as usize).wrapping_mul(11));
    let winner = read_i32(v + month_award_fields::SLOT_WINNER_ID);
    let person = staff_at(winner as isize);
    covered!("sx04b_1 winner news");
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let item = local.as_mut_ptr() as usize;
    exe::new_newsitem(item, MONTH_AWARD_NEWS_TYPE, 0);
    exe::newsitem_set_slot(item, 0, read_u32(this + award_base::NATION_ID));
    exe::newsitem_set_slot(item, 1, read_u32(this + award_base::INDEX));
    exe::newsitem_set_slot(item, 2, winner as u32);
    exe::newsitem_set_slot(item, 3, read_u32(person.first_name() + names::ID));
    exe::newsitem_set_slot(item, 4, read_u32(person.second_name() + names::ID));
    exe::newsitem_set_slot(item, 5, read_u32(person.common_name() + names::ID));
    exe::newsitem_set_slot(item, 6, read_u32(v + month_award_fields::SLOT_CLUB_ID));
    exe::newsitem_set_slot(item, 7, read_i8_as_i32(v + month_award_fields::SLOT_MONTH) as u32);
    exe::newsitem_set_slot(item, 8, read_u16(v + month_award_fields::SLOT_YEAR) as u32);
    exe::newsitem_set_slot(item, 9, MONTH_AWARD_NEWS_VERSION);
    exe::newsitem_set_slot(item, 0xa, exe::utils_random(3) as u32); // sx04b_1: the template draw
    let sheet = person.player();
    exe::newsitem_set_slot(item, 0xb, sheet.is_null() as u32);
    if !sheet.is_null() {
        covered!("sx04b_1 winner with a player sheet");
        exe::newsitem_set_slot(item, 0xc, exe::formation_get_position_mask(sheet) as i16 as i32 as u32);
        let club = club_at(read_i32(v + month_award_fields::SLOT_CLUB_ID) as isize);
        exe::newsitem_set_slot(item, 0xd, exe::staff_get_player_strength_flags(person, club));
    }
    exe::newsitem_set_slot(item, 0xe, (person.job_for_club() == job_kind::PLAYER_MANAGER) as u32);
    let mut clubs = [0usize; 30];
    let mut n: i8 = 0;
    let tm = read_usize(G_TRANSFER_MANAGER);
    if tm != 0 && !person.player().is_null() {
        n = exe::transfer_manager_collect_player_news_recipients(tm, person, 0x48, clubs.as_mut_ptr(), 0x1e) as i8;
    }
    let league = read_usize(this + award_base::CLUB_COMP);
    let comp = read_usize(this + award_base::STAFF_COMP);
    if league != 0 {
        sink(item, NewsPost::Division(league));
    } else if comp != 0 {
        sink(item, NewsPost::Nation(read_usize(comp + STAFF_COMP_NATION)));
    }
    for &club in clubs.iter().take(n.max(0) as usize) {
        if read_usize(club + club::DIVISION) != read_usize(this + award_base::CLUB_COMP) { // sx04b_1: clubs of other leagues
            sink(item, NewsPost::Club(club));
        }
    }
}

/// Comparer for `month_award_post_winner_news`: the news is posted once, so the original runs and the copy is
/// rehearsed from the same random generator (`rehearse_news`: it fills the item and draws without posting); the
/// rehearsal's draws must be the start of the original's.
pub(crate) unsafe extern "thiscall" fn compare_month_award_post_winner_news(this: usize, slot: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(%%I0%%);
    let mut planned = Vec::new();
    let (same, from_original, from_copy) = rehearse_news(|| o(this, slot), || month_award_winner_news(this, slot, &mut |_, to| planned.push(to)));
    SCOREBOARD.record(same, || format!("month_award_post_winner_news(slot {}) draws original={:?} rust={:?}, rust posts {:?}", slot as u8, from_original, from_copy, planned));
}

/// Writes the headline (`body` 0) or the body of a month award's winner news into `out` (the slots are those of
/// `month_award_post_winner_news`).
///
/// The winner's name is formatted twice from the three name ids (`exe::database_format_name_from_ids`, 0xc8 bytes):
/// long (style 1) and short (style 5). When the winner has a player sheet and the item carries
/// `MONTH_AWARD_NEWS_VERSION`, his position name (`exe::write_position_name`) is written first, with its gender.
/// The headline: by (winner / (draw + 1)) % 6, "<player> takes award", "<player> named as <award>", "Award for
/// <player>", "<player> scoops prize", "<player> wins <month> award", or (5, or a negative remainder) "<award> award
/// decided" (the award by the short name of the staff competition, +5). The body: without a club "<player> has won the
/// <award> award for the month of <month>"; with a club, when the winner has a player sheet, the item carries the mark
/// and the draw is not 0, the player's description (`exe::write_player_description`, replaced by "player-boss" for a
/// player-manager) goes in "<club> <description> <player> is the winner of <month> <award> award" (draw 1) or "The
/// winner of <month> <award> award is <club> <description> <player>" (draw 2); otherwise "<club> <player> has won the
/// <award> award for <month>". The names, the genders and the month (`exe::date_get_month_name`) go in the order the
/// original pushes them. Then the links: the club (short name, 0x7d5), the winner's long and short names (0x7d7) when
/// his names still match the item's ids (`exe::database_staff_matches_names`), and the award's long and short names
/// (0x7d4). Returns the last link's result.
/// Defect of the original: the description's gender goes as a whole dword of which only the low byte was written (the
/// rest is stack leftover); the description reads the byte only.
/// Original: `MONTH_AWARD::write_news` (0x767580, month_award.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn month_award_write_news(this: usize, news: usize, out: usize, body: u32) -> i32 {
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let winner = slot(2) as i32;
    let ids = [slot(3), slot(4), slot(5)];
    let club_id = slot(6) as i32;
    let month = slot(7) as u8 as u32;
    let reworded = slot(9) != MONTH_AWARD_NEWS_VERSION;
    let draw = slot(0xa) as u8 as i8;
    let no_player = slot(0xb);
    let positions = slot(0xc);
    let strengths = slot(0xd);
    let player_boss = slot(0xe);
    let mut description = [0u8; 0xc8];
    let desc = description.as_mut_ptr() as usize;
    let mut gender: u8 = 0;
    if no_player == 0 && !reworded {
        gender = exe::write_position_name(positions, desc, 0xc8, 1);
    }
    let mut long_name = [0u8; 0xc8];
    let mut short_name = [0u8; 0xc8];
    exe::database_format_name_from_ids(ids[0], ids[1], ids[2], long_name.as_mut_ptr(), 0xc8, 1, winner as u32);
    exe::database_format_name_from_ids(ids[0], ids[1], ids[2], short_name.as_mut_ptr(), 0xc8, 5, winner as u32);
    let (long, short) = (long_name.as_ptr() as usize, short_name.as_ptr() as usize);
    let comp = read_usize(this + award_base::STAFF_COMP); // a TStaffComp: its names as in `competition`
    let fmt = news_text_format();
    let buf = NEWS_TEXT_BUFFER;
    if body != 0 {
        covered!("sx04b_1 body");
        let comp_gender = read_i8(comp + competition::GENDER_NAME) as i32 as u32;
        let comp_name = comp + competition::NAME;
        if club_id == -1 {
            let month_name = exe::date_get_month_name(month);
            // "<player> has won the <award> award for the month of <month>."
            fmt(buf, 0xa5ad8c, 0x64, comp_gender, comp_gender, 0x64, 0x64, long, comp_name, month_name);
        } else if no_player == 0 && draw != 0 && !reworded {
            covered!("sx04b_1 body with the description");
            let club = club_at(club_id as isize);
            gender = exe::write_player_description(winner, positions, strengths, desc, 0xc8, gender as u32, 1);
            if player_boss != 0 {
                fmt(desc, 0xa5b0f8); // "player-boss"
            }
            let desc_gender = gender as i8 as i32 as u32;
            let club_gender = read_i8(club.addr() + club::SHORT_GENDER_NAME) as i32 as u32;
            let club_name = club.addr() + club::SHORT_NAME;
            let month_name = exe::date_get_month_name(month);
            if draw == 1 { // sx04b_1: the club-first template
                // "<club> <description> <player> is the winner of <month> <award> award."
                fmt(buf, 0xa5b004, club_gender, club_gender, desc_gender, desc_gender, 0x64, 0x64, comp_gender, comp_gender, club_name, desc, long, month_name, comp_name);
            } else {
                // "The winner of <month> <award> award is <club> <description> <player>."
                fmt(buf, 0xa5af10, 0x64, 0x64, comp_gender, comp_gender, club_gender, club_gender, desc_gender, desc_gender, month_name, comp_name, club_name, desc, long);
            }
        } else {
            let club = club_at(club_id as isize);
            let club_gender = read_i8(club.addr() + club::SHORT_GENDER_NAME) as i32 as u32;
            let month_name = exe::date_get_month_name(month);
            // "<club> <player> has won the <award> award for <month>."
            fmt(buf, 0xa5ae44, club_gender, club_gender, 0x64, comp_gender, comp_gender, 0x64, 0x64, club.addr() + club::SHORT_NAME, long, comp_name, month_name);
        }
    } else {
        covered!("sx04b_1 headline");
        let award_gender = read_i8(comp + competition::SHORT_GENDER_NAME) as i32 as u32;
        let award = comp + competition::SHORT_NAME;
        match (winner / (draw as i32 + 1)) % 6 {
            0 => fmt(buf, 0xa5ad60, short), // "<player> takes award"
            1 => fmt(buf, 0xa5ad04, award_gender, award_gender, short, award), // "<player> named as <award>"
            2 => fmt(buf, 0xa5acd8, short), // "Award for <player>"
            3 => fmt(buf, 0xa5acac, short), // "<player> scoops prize"
            4 => fmt(buf, 0xa5ac5c, 0x64, 0x64, short, exe::date_get_month_name(month)), // "<player> wins <month> award"
            _ => fmt(buf, 0xa5ac18, award_gender, award_gender, award), // "<award> award decided"
        };
    }
    exe::text_set_string(out, buf as *const u8);
    if club_id != -1 {
        let club = club_at(club_id as isize);
        exe::text_add_link(out, (club.addr() + club::SHORT_NAME) as *const u8, 0x7d5, club);
    }
    let person = staff_at(winner as isize);
    if exe::database_staff_matches_names(person, ids[0], ids[1], ids[2], 0usize) != 0 {
        exe::text_add_link(out, long as *const u8, 0x7d7, person);
        exe::text_add_link(out, short as *const u8, 0x7d7, person);
    }
    exe::text_add_link(out, (comp + competition::NAME) as *const u8, 0x7d4, comp);
    exe::text_add_link(out, (comp + competition::SHORT_NAME) as *const u8, 0x7d4, comp)
}

/// Comparer for `month_award_write_news`: side by side into two Texts (`compare_text_writer`: the text, the links and
/// the random generator).
pub(crate) unsafe extern "thiscall" fn compare_month_award_write_news(this: usize, news: usize, out: usize, body: u32) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> i32>(%%I1%%);
    let mut r = 0;
    compare_text_writer(out, || r = o(this, news, out, body), |t| {
        month_award_write_news(this, news, t, body);
    }, || format!("month_award_write_news(winner {}, body {})", exe::newsitem_get_slot(news, 2) as i32, body));
    r
}
