// ---------- cx01_2: the competition news texts: a club's fate, the prize money and the TV rights (comp.cpp) ----------

/// Writes the news text of a club's fate in a competition into Text `text`.
///
/// `Comp::build_fate_news_text(club, outcome, _, _, fixture, next_stage, _, verdict, body, text)` (thiscall, ret 0x28;
/// the third, fourth and seventh arguments are not read). With a fixture (a cup tie) the text is the shared cup line:
/// with `body`, `comp_util_write_cup_tie_result_news(club, fixture, verdict, outcome, next_stage, text)`, otherwise
/// `comp_util_write_cup_win_or_defeat_news(club, fixture, outcome, text)`. Without one, by the `outcome` byte: 5 the
/// club wins the competition, 0 promotion, 1 reaches the play-offs, 2 drops into the play-offs, 3 relegated. The line
/// is printed into the global text buffer 0xde1f64 (`FN_LANGLIB_SPRINTF`) with the club's short name and gender (club
/// index `club`) and, in the body, the competition's long name and gender (in the headline of a title, its short name
/// and gender; the other headlines carry only the club); then `exe::text_set_string` and the links: the club (0x7d5)
/// and the competition name (0x7d0), except in the other headlines and, for the body of a title, when the competition
/// is a friendly one (`exe::friendly_is_friendly_competition`). Any other outcome writes "ERROR: No league fate yet for
/// <short name>" (crt sprintf) as the text.
/// Original: `Comp::build_fate_news_text` (0x48c6d0, comp.cpp, thiscall, ret 0x28).
pub(crate) unsafe extern "thiscall" fn comp_build_fate_news_text(this: usize, club: i32, outcome: u32, _a3: u32, _a4: u32, fixture: usize, next_stage: u32,
    _a7: u32, verdict: u32, body: u32, text: usize) {
    if fixture != 0 {
        if body != 0 {
            covered!("cx01_2 fate: cup tie result");
            exe::comp_util_write_cup_tie_result_news(club as u32, fixture as u32, verdict, outcome, next_stage, text);
        } else {
            covered!("cx01_2 fate: cup win or defeat");
            exe::comp_util_write_cup_win_or_defeat_news(club as u32, fixture as u32, outcome, text);
        }
        return;
    }
    let lang_sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let comp = League(this).competition();
    let c = club_at(club as isize);
    let cg = c.short_gender_name() as i8 as i32;
    let team = c.addr() + club::SHORT_NAME;
    let (long, kg) = (comp.name_addr(), comp.gender_name() as i8 as i32);
    // (body format, headline format)
    let formats = match outcome as u8 {
        5 => (0x98acec, 0x989da4), // wins the competition
        0 => (0x99b744, 0x99b710), // promotion
        1 => (0x99b698, 0x99b664), // reaches the play-offs
        2 => (0x99b5e8, 0x99b5b0), // drops into the play-offs
        3 => (0x99b540, 0x99b510), // relegated
        _ => {
            covered!("cx01_2 fate: no fate");
            let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
            sprintf(0xde1f64, 0x99b4ec, comp.short_name_addr()); // "ERROR: No league fate yet for %s"
            exe::text_set_string(text, 0xde1f64 as *const u8);
            return;
        }
    };
    if body != 0 {
        covered!("cx01_2 fate: body");
        lang_sprintf(0xde1f64, formats.0, cg, cg, kg, kg, team, long);
        exe::text_set_string(text, 0xde1f64 as *const u8);
        exe::text_add_link(text, team as *const u8, 0x7d5, c);
        if outcome as u8 == 5 && exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), comp) != 0 {
            return;
        }
        exe::text_add_link(text, long as *const u8, 0x7d0, comp);
    } else if outcome as u8 == 5 {
        covered!("cx01_2 fate: title headline");
        let (short, sg) = (comp.short_name_addr(), comp.short_gender_name() as i8 as i32);
        lang_sprintf(0xde1f64, formats.1, cg, cg, sg, sg, team, short);
        exe::text_set_string(text, 0xde1f64 as *const u8);
        exe::text_add_link(text, team as *const u8, 0x7d5, c);
        exe::text_add_link(text, short as *const u8, 0x7d0, comp);
    } else {
        covered!("cx01_2 fate: headline");
        lang_sprintf(0xde1f64, formats.1, cg, cg, team);
        exe::text_set_string(text, 0xde1f64 as *const u8);
        exe::text_add_link(text, team as *const u8, 0x7d5, c);
    }
}

/// An amount of money as the news show it: `Cash::get_display_amount` (flags 6) formatted by `cash_format_money` into a
/// 30-byte buffer.
unsafe fn news_money_text(amount: u32) -> [u8; 0x1e] {
    let mut money = [0u8; 0x1e];
    exe::cash_format_money(money.as_mut_ptr() as usize, 0x1e, exe::cash_get_display_amount(&amount, 6), 6);
    money
}

/// Writes the prize money news (type 0x1a) into Text `out`.
///
/// `Comp::write_prize_money_news_text(news, body, out)` (thiscall, ret 0xc). The news slots: 1 the amount, 2 the club
/// index, 3 the final position (byte), 4 the round, 5 the opponent's club index, 6 whether the club won (byte), 7 the
/// kind (byte). The amount is shown through `news_money_text`. The headline (`body` 0) is the club and the amount
/// (format 0x99b844), with the club link (0x7d5). The body: with a final position above 0, the club, the amount, the
/// position as an ordinal (`exe::utils_format_ordinal(position, −1)`) and the competition's long name (0x99bc18);
/// otherwise with a round, the club, the amount and the competition, stage and round label in form 1
/// (`exe::comp_util_format_comp_stage_round_label`, 300 bytes, gendered by `exe::comp_util_get_label_gender`, 0x99bb78);
/// otherwise kind 1 (0x99bad8) or 2 (0x99ba2c) the club, the amount and the competition's long name; otherwise, with
/// an opponent (index above −1), the club, the amount, the opponent and the competition (0x99b95c when the club won,
/// else 0x99b88c), and with none nothing at all. Each line goes to the global text buffer 0xde1f64
/// (`FN_LANGLIB_SPRINTF`) with the genders first, then `exe::text_set_string` and the links: the club (0x7d5), the
/// opponent (0x7d5) and the competition name or the label (0x7d0, the competition).
/// Original: `Comp::write_prize_money_news_text` (0x48e590, comp.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn comp_write_prize_money_news_text(this: usize, news: usize, body: u32, out: usize) {
    let lang_sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let comp = League(this).competition();
    let amount = exe::newsitem_get_slot(news, 1);
    let c = club_at(exe::newsitem_get_slot(news, 2) as i32 as isize);
    let cg = c.short_gender_name() as i8 as i32;
    let team = c.addr() + club::SHORT_NAME;
    if body == 0 {
        covered!("cx01_2 prize money headline");
        let money = news_money_text(amount);
        lang_sprintf(0xde1f64, 0x99b844, cg, cg, team, money.as_ptr() as usize);
        exe::text_set_string(out, 0xde1f64 as *const u8);
        exe::text_add_link(out, team as *const u8, 0x7d5, c);
        return;
    }
    let position = exe::newsitem_get_slot(news, 3) as i8;
    let round = exe::newsitem_get_slot(news, 4);
    let opponent = exe::newsitem_get_slot(news, 5) as i32;
    let won = exe::newsitem_get_slot(news, 6) as i8;
    let kind = exe::newsitem_get_slot(news, 7) as i8;
    let money = news_money_text(amount);
    let (long, kg) = (comp.name_addr(), comp.gender_name() as i8 as i32);
    if position > 0 {
        covered!("cx01_2 prize money for a position");
        let ordinal = exe::utils_format_ordinal(position as i32, -1);
        lang_sprintf(0xde1f64, 0x99bc18, cg, cg, kg, kg, team, money.as_ptr() as usize, ordinal, long);
    } else if round as u16 != 0 {
        covered!("cx01_2 prize money for a round");
        let mut label = [0u8; 0x12c];
        exe::comp_util_format_comp_stage_round_label(comp, 0, round, 1, label.as_mut_ptr(), 0x12c);
        let lg = exe::comp_util_get_label_gender(comp.addr() as u32, 0, round, 1) as i8 as i32;
        lang_sprintf(0xde1f64, 0x99bb78, cg, cg, lg, lg, team, money.as_ptr() as usize, label.as_ptr() as usize);
        exe::text_set_string(out, 0xde1f64 as *const u8);
        exe::text_add_link(out, team as *const u8, 0x7d5, c);
        exe::text_add_link(out, label.as_ptr(), 0x7d0, comp);
        return;
    } else if kind == 1 || kind == 2 {
        covered!("cx01_2 prize money for a title or runners up");
        lang_sprintf(0xde1f64, if kind == 1 { 0x99bad8 } else { 0x99ba2c }, cg, cg, kg, kg, team, money.as_ptr() as usize, long);
    } else if opponent > -1 {
        covered!("cx01_2 prize money for a match");
        let o = club_at(opponent as isize);
        let og = o.short_gender_name() as i8 as i32;
        let rival = o.addr() + club::SHORT_NAME;
        lang_sprintf(0xde1f64, if won != 0 { 0x99b95c } else { 0x99b88c }, cg, cg, og, og, kg, kg, team, money.as_ptr() as usize, rival, long);
        exe::text_set_string(out, 0xde1f64 as *const u8);
        exe::text_add_link(out, team as *const u8, 0x7d5, c);
        exe::text_add_link(out, rival as *const u8, 0x7d5, o);
        exe::text_add_link(out, long as *const u8, 0x7d0, comp);
        return;
    } else {
        return;
    }
    exe::text_set_string(out, 0xde1f64 as *const u8);
    exe::text_add_link(out, team as *const u8, 0x7d5, c);
    exe::text_add_link(out, long as *const u8, 0x7d0, comp);
}

/// Writes the TV rights news (type 0x1f) into Text `out`.
///
/// `Comp::write_tv_rights_news_text(news, body, out)` (thiscall, ret 0xc). The news slots: 0 the club index, 2 the
/// amount, 3 the season year. The amount is shown through `news_money_text`. The headline (`body` 0) is the club and
/// the amount (format 0x99bcd4), with the club link (0x7d5). The body is the club, the amount, the competition's short
/// name and the season (`exe::key_nation_format_club_season(year, club, buffer, 20)`), format 0x99bd1c, with the club
/// (0x7d5) and the competition's short name (0x7d0) links. The lines go to the global text buffer 0xde1f64
/// (`FN_LANGLIB_SPRINTF`) with the genders first, then `exe::text_set_string`.
/// Original: `Comp::write_tv_rights_news_text` (0x48eb00, comp.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn comp_write_tv_rights_news_text(this: usize, news: usize, body: u32, out: usize) {
    let lang_sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let c = club_at(exe::newsitem_get_slot(news, 0) as i32 as isize);
    let amount = exe::newsitem_get_slot(news, 2);
    let cg = c.short_gender_name() as i8 as i32;
    let team = c.addr() + club::SHORT_NAME;
    if body == 0 {
        covered!("cx01_2 tv rights headline");
        let money = news_money_text(amount);
        lang_sprintf(0xde1f64, 0x99bcd4, cg, cg, team, money.as_ptr() as usize);
        exe::text_set_string(out, 0xde1f64 as *const u8);
        exe::text_add_link(out, team as *const u8, 0x7d5, c);
        return;
    }
    covered!("cx01_2 tv rights body");
    let year = exe::newsitem_get_slot(news, 3);
    let money = news_money_text(amount);
    let mut season = [0u8; 0x14];
    exe::key_nation_format_club_season(year, c, season.as_mut_ptr() as usize, 0x14);
    let comp = League(this).competition();
    let (short, sg) = (comp.short_name_addr(), comp.short_gender_name() as i8 as i32);
    lang_sprintf(0xde1f64, 0x99bd1c, cg, cg, sg, sg, team, money.as_ptr() as usize, short, season.as_ptr() as usize);
    exe::text_set_string(out, 0xde1f64 as *const u8);
    exe::text_add_link(out, team as *const u8, 0x7d5, c);
    exe::text_add_link(out, short as *const u8, 0x7d0, comp);
}

/// Comparer for `comp_build_fate_news_text`: the original into the real Text and the copy into a local one
/// (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_comp_build_fate_news_text(this: usize, club: i32, outcome: u32, a3: u32, a4: u32, fixture: usize, next_stage: u32,
    a7: u32, verdict: u32, body: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, u32, u32, u32, usize, u32, u32, u32, u32, usize)>(%%I0%%);
    compare_text_writer(text, || o(this, club, outcome, a3, a4, fixture, next_stage, a7, verdict, body, text),
        |t| comp_build_fate_news_text(this, club, outcome, a3, a4, fixture, next_stage, a7, verdict, body, t),
        || format!("comp_build_fate_news_text({:#x}, club {}, outcome {}, fixture {:#x}, body {})", this, club, outcome as u8, fixture, body));
}

/// Comparer for `comp_write_prize_money_news_text` (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_comp_write_prize_money_news_text(this: usize, news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I1%%);
    compare_text_writer(out, || o(this, news, body, out), |t| comp_write_prize_money_news_text(this, news, body, t),
        || format!("comp_write_prize_money_news_text({:#x}, news {:#x}, body {})", this, news, body));
}

/// Comparer for `comp_write_tv_rights_news_text` (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_comp_write_tv_rights_news_text(this: usize, news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I2%%);
    compare_text_writer(out, || o(this, news, body, out), |t| comp_write_tv_rights_news_text(this, news, body, t),
        || format!("comp_write_tv_rights_news_text({:#x}, news {:#x}, body {})", this, news, body));
}
