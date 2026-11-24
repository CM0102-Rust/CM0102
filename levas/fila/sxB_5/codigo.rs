// --- sxB_5: the news texts of the board's reply to a request and to an ultimatum (manager_manager.cpp) ---

/// The board's reply to request kind `request` (1 the stadium, 2 the training facilities, 3 transfer funds, 4 the
/// wage budget, 5 time to rebuild) by the reply code `reply`; `None` for kinds 6 and above (the placeholder texts).
/// Transfer funds: reply 2 refused, then "once the finances are corrected" (0xa109f8) when the value (slot 3) is
/// above 7250 and the flag (slot 4) is set, else "not prepared to invest more this season" (0xa10944) when the squad
/// investment (`board_record::SQUAD_INVESTMENT` of the club's board `record`, a double) is above 7500000, else "cannot afford" (0xa108c8); the
/// replies 4, 5, 7 and above 8 "cannot afford" with the flag, else "adequate resources" (0xa1066c).
unsafe fn board_request_reply_template(request: i8, reply: u8, value: i16, flag: u8, record: usize) -> Option<usize> {
    let funds_default = if flag != 0 { 0xa108c8 } else { 0xa1066c };
    Some(match request {
        1 => match reply {
            0 => 0xa1128c, // "... plans afoot to expand the stadium."
            1 => 0xa111dc,
            2 => 0xa11150,
            3 => 0xa1107c,
            6 => 0xa10fdc,
            7 => 0xa1135c, // "... the club doesn't own the stadium ..."
            _ => 0xa10f50,
        },
        2 => match reply {
            0 => 0xa10ed4, // "... investing in expanding the club's training facilities."
            1 => 0xa10e14,
            2 => 0xa10d74,
            3 => 0xa10c90,
            6 => 0xa10bfc,
            _ => 0xa10b74,
        },
        3 => match reply {
            0 => 0xa10af8, // "... extra transfer funds available to you immediately."
            1 | 3 => 0xa107fc,
            2 if value > 0x1c52 && flag != 0 => 0xa109f8,
            2 if read_f64(record + board_record::SQUAD_INVESTMENT) > read_f64(0x96d290) => 0xa10944, // 7500000.0
            2 => 0xa108c8,
            6 => 0xa1077c,
            8 => 0xa106e4,
            _ => funds_default,
        },
        4 => match reply {
            0 => 0xa105cc, // "... change the club's wage structure slightly ..."
            1 | 3 => 0xa104f8,
            2 => 0xa10430,
            6 => 0xa103a0,
            _ => 0xa102f8,
        },
        5 => match reply as i8 {
            0 => 0xa10024, // "... will give you time to turn the club around."
            4 => 0xa100bc,
            6 => 0xa10178,
            _ => 0xa10234,
        },
        _ => return None,
    })
}

/// Writes the headline or body of the board's reply to a request by the manager (news item (6, 0x1786)).
///
/// `MANAGER_MANAGER::news_board_request_reply_text(news, body, out)`: slot 0 the club, 1 the request kind, 2 the
/// reply, 3 a value (a signed word), 4 a flag. Headline: "Board reaction to request ..." by the kind (1 to expand the
/// stadium, 2 for youth investment, 3 for transfer funds, 4 for a higher wage budget, 5 for time to rebuild the squad,
/// 6 for player bonus payments; other kinds "for larger half time oranges", copied with strcpy). Body: reply 0xc
/// "dismissed, will inform you" (0xa114a4), 10 "dismissed, manage the team" (0xa113ec), else by the kind
/// (`board_request_reply_template`, with the club's long gender and name); kind 6 the developers' placeholder
/// (0xa0ff88) and other kinds another (0xa0fee4), both through `FN_CRT_SPRINTF` with the long name. The text goes into
/// the shared buffer (0xde1f64) and the club's long name links to the club (0x7d5).
/// Original: `MANAGER_MANAGER::news_board_request_reply_text` (0x6a9470, manager_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn news_board_request_reply_text(this: usize, news: usize, body: u32, out: usize) {
    let buf = NEWS_TEXT_BUFFER;
    let club_id = exe::newsitem_get_slot(news, 0) as i32;
    let club = club_at(club_id as isize);
    let long = club.addr() + club::NAME;
    if body == 0 {
        let request = exe::newsitem_get_slot(news, 1) as i8;
        covered!("sxB_5 board request headline");
        let tpl = match request {
            1 => 0xa0feb8, // "Board reaction to request to expand stadium"
            2 => 0xa0fe88, // "... for youth investment"
            3 => 0xa0fe58, // "... for transfer funds"
            4 => 0xa0fe24, // "... for higher wage budget"
            5 => 0xa0fdf0, // "... for time to rebuild squad"
            6 => 0xa0fdbc, // "... for player bonus payments"
            _ => 0,
        };
        if tpl != 0 {
            story_lang!(buf, tpl);
        } else {
            news_strcpy(buf, 0xa0fd84); // "... for larger half time oranges"
        }
    } else {
        let request = exe::newsitem_get_slot(news, 1) as i8;
        let reply = exe::newsitem_get_slot(news, 2) as u8;
        let value = exe::newsitem_get_slot(news, 3) as i16;
        let flag = exe::newsitem_get_slot(news, 4) as u8;
        covered!("sxB_5 board request body");
        let lg = club.gender_name() as i8 as i32;
        let record = read_usize(this) + club_id as usize * 0x49; // the club's board record
        let tpl = match reply {
            0xc => Some(0xa114a4), // "... dismissed your request but ... will inform you immediately."
            10 => Some(0xa113ec), // "... your job is to manage the team ..."
            _ => board_request_reply_template(request, reply, value, flag, record),
        };
        match tpl {
            Some(t) => story_lang!(buf, t, lg, lg, long),
            None => story_crt!(buf, if request == 6 { 0xa0ff88 } else { 0xa0fee4 }, long), // the placeholders
        }
    }
    exe::text_set_string(out, buf as *const u8);
    exe::text_add_link(out, long as *const u8, 0x7d5, club);
}

/// Comparer for `news_board_request_reply_text` (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_news_board_request_reply_text(this: usize, news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I0%%);
    compare_text_writer(out, || o(this, news, body, out), |t| news_board_request_reply_text(this, news, body, t),
        || format!("news_board_request_reply_text(body {}, request {}, reply {})", body, exe::newsitem_get_slot(news, 1) as i8, exe::newsitem_get_slot(news, 2) as u8));
}

/// Writes the headline or body of the board's reply to a manager's ultimatum (news item (6, 0x178f)): a private
/// meeting or a terse fax, or the press rumour of the ultimatum.
///
/// `news_ultimatum_response_text(news, body, out)`: slots 0..3 the manager (id and names), 4 the club, 5 the demand (1
/// the stadium, 2 the training facilities, 3 transfer funds, other the wage budget), 6 the kind (1 the press rumour),
/// 7 the board's answer, 9 and 10 the supporters' mood for the stadium and the training facilities; slot 8 is read and
/// dropped. The manager's name (`exe::database_format_staff_name`, 0x1f4 bytes, style 1). Headline: the rumour "<club>
/// fans respond to speculation" (0xa12ef8, the short gender and name); otherwise "Board refuse to ..." for an answer of
/// 2, 3 or 4, else "Board agree to ..." (the stadium, the training facilities, else transfer funds). Body: the rumour
/// by the answer (2 he will leave if refused: 0xa13ad0, 0xa13c54, 0xa13e44, 0xa13ffc; else the supporters' view:
/// the stadium 0xa131f8 or 0xa13050 with the stadium's name by slot 9, the training facilities 0xa13398 or 0xa13520 by
/// slot 10, 0xa13738, 0xa13924), with the club's long name and the manager's name twice; otherwise the board: 4 too
/// inexperienced (0xa14e5c), 3 overlooked this time (0xa1499c, 0xa14af0, 0xa14c18, 0xa14d30), 2 clear your desk
/// (0xa1450c, 0xa14664, 0xa14764, 0xa14848), 1 a terse fax agreeing (0xa141c4, 0xa14280, 0xa14354, 0xa14438); any
/// other answer leaves the shared buffer as it was. Links (body): the manager (0x7da) when his names match and the
/// club's long name (0x7d5).
/// Original: `news_ultimatum_response_text` (0x6adb70, manager_manager.cpp, stdcall, ret 0xc). Defect of the
/// original: an unknown answer in the body sets the text of whatever the shared buffer held.
pub(crate) unsafe extern "stdcall" fn news_ultimatum_response_text(news: usize, body: u32, out: usize) {
    let buf = NEWS_TEXT_BUFFER;
    let s: [u32; 11] = core::array::from_fn(|k| exe::newsitem_get_slot(news, k as u32));
    let manager = staff_at(s[0] as i32 as isize);
    let club = club_at(s[4] as i32 as isize);
    let (demand, kind, answer) = (s[5] as i32, s[6] as i32, s[7] as i32);
    let name = [0u8; 0x1f4];
    let nm = name.as_ptr() as usize;
    exe::database_format_staff_name(manager, nm, 0x1f4, 1);
    // the four demands: stadium, training facilities, transfer funds, wages
    let by_demand = |t: [usize; 4]| match demand {
        1 => t[0],
        2 => t[1],
        3 => t[2],
        _ => t[3],
    };
    if body == 0 {
        covered!("sxB_5 ultimatum headline");
        if kind == 1 {
            let g = club.short_gender_name() as i8 as i32;
            story_lang!(buf, 0xa12ef8, g, g, club.addr() + club::SHORT_NAME); // "... fans respond to speculation"
        } else if matches!(answer, 2 | 3 | 4) {
            story_lang!(buf, by_demand([0xa12f38, 0xa12f5c, 0xa12f8c, 0xa12f8c])); // "Board refuse to ..."
        } else {
            story_lang!(buf, by_demand([0xa12fc4, 0xa12fe8, 0xa13018, 0xa13018])); // "Board agree to ..."
        }
        exe::text_set_string(out, buf as *const u8);
        return;
    }
    covered!("sxB_5 ultimatum body");
    let lg = club.gender_name() as i8 as i32;
    let long = club.addr() + club::NAME;
    if kind == 1 {
        if answer == 2 {
            story_lang!(buf, by_demand([0xa13ad0, 0xa13c54, 0xa13e44, 0xa13ffc]), lg, lg, 0x64, 0x64, long, nm, nm);
        } else if demand == 1 && s[9] == 0 {
            let stadium = club.stadium();
            let sg = stadium.gender_name() as i8 as i32;
            story_lang!(buf, 0xa13050, lg, lg, 0x64, 0x64, sg, sg, long, nm, nm, stadium.addr() + stadium::NAME);
        } else {
            let training = if s[10] != 0 { 0xa13398 } else { 0xa13520 };
            story_lang!(buf, by_demand([0xa131f8, training, 0xa13738, 0xa13924]), lg, lg, 0x64, 0x64, long, nm, nm);
        }
    } else {
        let tpl = match answer {
            4 => Some(0xa14e5c), // "... such demands are inappropriate from such an inexperienced manager ..."
            3 => Some(by_demand([0xa1499c, 0xa14af0, 0xa14c18, 0xa14d30])), // "... willing to overlook your outburst ..."
            2 => Some(by_demand([0xa1450c, 0xa14664, 0xa14764, 0xa14848])), // "... expect your desk to be cleared ..."
            1 => Some(by_demand([0xa141c4, 0xa14280, 0xa14354, 0xa14438])), // "A terse fax ..."
            _ => None, // the shared buffer as it was
        };
        if let Some(t) = tpl {
            story_lang!(buf, t, lg, lg, long);
        }
    }
    exe::text_set_string(out, buf as *const u8);
    if exe::database_staff_matches_names(manager, s[1], s[2], s[3], 0usize) != 0 {
        exe::text_add_link(out, name.as_ptr(), 0x7da, manager);
    }
    exe::text_add_link(out, long as *const u8, 0x7d5, club);
}

/// Comparer for `news_ultimatum_response_text` (`compare_text_writer`).
pub(crate) unsafe extern "stdcall" fn compare_news_ultimatum_response_text(news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, u32, usize)>(%%I1%%);
    compare_text_writer(out, || o(news, body, out), |t| news_ultimatum_response_text(news, body, t),
        || format!("news_ultimatum_response_text(body {}, demand {}, answer {})", body, exe::newsitem_get_slot(news, 5) as i32, exe::newsitem_get_slot(news, 7) as i32));
}
