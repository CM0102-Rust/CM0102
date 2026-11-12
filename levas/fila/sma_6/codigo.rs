// ---------- batch %%LEVA%%: the board's season expectations and confidence texts (manager_manager.cpp) ----------

/// Exe routines of the board texts without an `exe::` wrapper (still original, or Rust hooked and called through the
/// exe so the comparison runs their comparers).
pub(crate) mod board_text_fns {
    pub(crate) const BUILD_BOARD_EXPECTATION_TEXT: usize = 0x694440; // thiscall(this, club, flags, buf, size), ret 0x10
    pub(crate) const BUILD_BOARD_CONFIDENCE_TEXT: usize = 0x690d80; // thiscall(this, club, flags, buf, size), ret 0x10
    pub(crate) const BUILD_BOARD_NAMED_PLAYERS_TEXT: usize = 0x693850; // thiscall(this, buf, club, ids) -> al count
}

/// The expectation sentence of the board, by the flags: (title flag 1, the six format pairs of the other flags).
/// Each entry is the text without 0x800 and with 0x800; the three columns are flag 0x400, neither 0x400 nor 0x100,
/// and 0x100.
const EXPECTATION_TEXTS: [(u32, [[usize; 2]; 3]); 5] = [
    (4, [[0xa041e8, 0xa04268], [0xa03f98, 0xa03ffc], [0xa040a0, 0xa04124]]),
    (8, [[0xa03e58, 0xa03edc], [0xa03c18, 0xa03c7c], [0xa03d18, 0xa03d9c]]),
    (0x40, [[0xa03ab0, 0xa03b44], [0xa03828, 0xa038a0], [0xa03958, 0xa039e8]]),
    (0x10, [[0xa036d8, 0xa03760], [0xa03480, 0xa034ec], [0xa03598, 0xa0361c]]),
    (0x20, [[0xa0331c, 0xa033ac], [0xa030a4, 0xa03118], [0xa031cc, 0xa03258]]),
];

/// Writes the board's season expectation sentence for a club from the expectation flag bits into `buf`.
///
/// `build_board_expectation_text(club, flags, buf, size)` (the size is not used; `this` neither). Each sentence is a
/// language format (`FN_LANGLIB_SPRINTF`) with the club's gender twice (`club::GENDER_NAME`) and its name
/// (`club::NAME`). Flag 1 (the title): by 0x400 / neither 0x400 nor 0x100 / 0x100 and by 0x800, with a second version
/// when the club's nation (`.nation()`) has a continent (`.continent()`) other than Europe (`KEY_CONTINENT_EUROPE`).
/// Flag 2: with
/// 0x400 or 0x100 one text by 0x800; otherwise by 0x800 and by the virtual +0x98 of the division's competition object
/// (`G_COMP_OBJECT_TABLE`, the top-division test). Then the first of 4, 8, 0x40, 0x10 set (`EXPECTATION_TEXTS`);
/// without 0x20, the "improve the squad" text 0xa03038; with it, the 0x20 texts.
/// Original: `MANAGER_MANAGER::build_board_expectation_text` (0x694440, manager_manager.cpp, thiscall, ret 0x10; ecx
/// unused).
pub(crate) unsafe extern "thiscall" fn build_board_expectation_text(_this: usize, club: usize, flags: u32, buf: usize, _size: u32) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let c = Club(club);
    write_u8(buf, 0);
    let g = read_i8(club + club::GENDER_NAME) as i32;
    let say = |text: usize| {
        format(buf, text, g, g, club + club::NAME);
    };
    let f = |bit: u32| flags & bit != 0;
    let col = if f(0x400) { 0 } else if !f(0x100) { 1 } else { 2 };
    let late = f(0x800) as usize;
    if f(1) {
        covered!("sma_6 title expectation");
        let n = c.nation();
        let outside_europe = !n.is_null() && n.continent() != 0 && read_u32(n.continent()) != read_u32(KEY_CONTINENT_EUROPE);
        // (continental, other) by the column and 0x800
        let t: [[[usize; 2]; 2]; 3] = [[[0xa04a60, 0xa04ae8], [0xa04b80, 0xa04c4c]], [[0xa046a0, 0xa046f0], [0xa04744, 0xa047c0]],
            [[0xa04840, 0xa048a4], [0xa0490c, 0xa049b4]]];
        say(t[col][late][outside_europe as usize]); // sma6: the title sentence
        return;
    }
    if f(2) {
        if col != 1 {
            say(if late == 1 { 0xa04390 } else { 0xa0432c });
            return;
        }
        let d = c.division();
        let top = !d.is_null() && call_comp_virtual(read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_u32(d.addr()) as usize * 4), 0x98) != 0;
        say(match (late, top) {
            (0, true) => 0xa044b0,
            (0, false) => 0xa04438,
            (_, true) => 0xa045e4,
            _ => 0xa04528,
        });
        return;
    }
    for (bit, texts) in EXPECTATION_TEXTS {
        if bit == 0x20 && !f(0x20) {
            say(0xa03038);
            return;
        }
        if f(bit) {
            say(texts[col][late]);
            return;
        }
    }
}

/// Comparer for `build_board_expectation_text`: side by side over the buffer (the size the caller gives, at most 0x1388).
pub(crate) unsafe extern "thiscall" fn compare_build_board_expectation_text(this: usize, club: usize, flags: u32, buf: usize, size: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize, u32)>(%%I0%%);
    let n = (size as usize).clamp(1, 0x1388);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&[(buf, n)], || o(this, club, flags, buf, size), || build_board_expectation_text(this, club, flags, buf, size));
    SCOREBOARD.record(igual, || format!("build_board_expectation_text(flags {:#x}) {:?} x {:?}", flags,
        String::from_utf8_lossy(&mo.0[0].1[..0x80.min(n)]), String::from_utf8_lossy(&mr.0[0].1[..0x80.min(n)])));
}

/// Writes the headline or body of a board's season expectations.
///
/// `MANAGER_MANAGER::news_board_expectations_text(news, body, out)`: the news slots 0 (club id) and 1 (the flags,
/// `exe::newsitem_get_slot`). Body: the expectation sentence (`build_board_expectation_text`, through the exe).
/// Headline (`FN_LANGLIB_SPRINTF` of a fixed text): a national team (id above clubs − 2 × nations) "F.A. Announce
/// Expectations"; flag 0x400 "Board expect glory", 2 "want the title", 0x85 "hope for successful season", 0x30
/// "expecting difficult season", 0x308 "look forward to new season", otherwise "announce expectations". Then the text
/// into `out` (`exe::text_set_string`) and the club's link (`exe::text_add_link`, 0x7d5, name `club::NAME`), also on
/// the headline.
/// Original: `MANAGER_MANAGER::news_board_expectations_text` (0x6a4ae0, manager_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn news_board_expectations_text(this: usize, news: usize, body: u32, out: usize) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let mut text = vec![0u8; 0x1388];
    let t = text.as_mut_ptr() as usize;
    let id = exe::newsitem_get_slot(news, 0);
    let flags = exe::newsitem_get_slot(news, 1);
    let club = read_usize(G_CLUB_TABLE).wrapping_add((id as usize).wrapping_mul(club::SIZE));
    if body != 0 {
        covered!("sma_6 expectations body");
        let build: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32) = core::mem::transmute(board_text_fns::BUILD_BOARD_EXPECTATION_TEXT);
        build(this, club, flags, t, 0x1388);
    } else {
        let h = if id as i32 > read_i32(G_CLUB_COUNT) - 2 * read_i32(G_NATION_COUNT) {
            0xa0abcc
        } else if flags & 0x400 != 0 {
            0xa0abb8
        } else if flags & 2 != 0 {
            0xa0aba0
        } else if flags & 0x85 != 0 {
            0xa0ab7c
        } else if flags & 0x30 != 0 {
            0xa0ab58
        } else if flags & 0x308 != 0 {
            0xa0ab34
        } else {
            0xa0ab18
        };
        format(t, h); // sma6: the expectations headline
    }
    exe::text_set_string(out, t as *const u8);
    exe::text_add_link(out, (club + club::NAME) as *const u8, 0x7d5, club);
}

/// Comparer for `news_board_expectations_text` (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_news_board_expectations_text(this: usize, news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I1%%);
    compare_text_writer(out, || o(this, news, body, out), |t| news_board_expectations_text(this, news, body, t),
        || format!("news_board_expectations_text(body {})", body));
}

/// Writes the headline or body of the board's (or FA's) confidence update, with links to the players it names.
///
/// `MANAGER_MANAGER::news_board_confidence_update_text(news, body, out)`: slots 0 (club id) and 1 (the flags).
/// Headline: "Football Association Confidence Update" for a national team (`exe::database_is_national_team`), "Board
/// Confidence Update" otherwise. Body: the confidence sentence (0x690d80) and the named players' sentence (0x693850,
/// up to 15 ids, returns how many); with players, both joined by "%s\n\n%s", otherwise the first one. The text into
/// `out` (`exe::text_set_string`); each named player (id not −1) whose formatted name (`exe::database_format_staff_name`,
/// 0x96, style 1) is in the text (`exe::crt_strstr`) gets a link (0x7db); the club gets its link (0x7d5) when its name
/// is in the text.
/// Original: `MANAGER_MANAGER::news_board_confidence_update_text` (0x6a4d40, manager_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn news_board_confidence_update_text(this: usize, news: usize, body: u32, out: usize) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let mut ids = [-1i32; 15];
    let mut name = [0u8; 0x98];
    let mut text = vec![0u8; 0x1388];
    let mut first = vec![0u8; 0x1388];
    let mut players = vec![0u8; 0x5dc];
    let t = text.as_mut_ptr() as usize;
    let id = exe::newsitem_get_slot(news, 0);
    let flags = exe::newsitem_get_slot(news, 1);
    let club = read_usize(G_CLUB_TABLE).wrapping_add((id as usize).wrapping_mul(club::SIZE));
    let mut count: i32 = 0;
    if body == 0 {
        format(t, if exe::database_is_national_team(club) != 0 { 0xa0ac00 } else { 0xa0abe8 });
    } else {
        covered!("sma_6 confidence body");
        let conf: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32) = core::mem::transmute(board_text_fns::BUILD_BOARD_CONFIDENCE_TEXT);
        conf(this, club, flags, first.as_mut_ptr() as usize, 0x1388);
        let named: unsafe extern "thiscall" fn(usize, usize, usize, *mut i32) -> u8 = core::mem::transmute(board_text_fns::BUILD_BOARD_NAMED_PLAYERS_TEXT);
        count = named(this, players.as_mut_ptr() as usize, club, ids.as_mut_ptr()) as i8 as i32;
        if count != 0 {
            sprintf(t, 0x991948, first.as_ptr() as usize, players.as_ptr() as usize); // sma6: two sentences
        } else {
            sprintf(t, 0x991944, first.as_ptr() as usize);
        }
    }
    exe::text_set_string(out, t as *const u8);
    for &p in ids.iter().take(count.max(0) as usize) {
        if p == -1 {
            continue;
        }
        let person = read_usize(G_STAFF_TABLE).wrapping_add((p as usize).wrapping_mul(staff::SIZE));
        exe::database_format_staff_name(person, name.as_mut_ptr() as usize, 0x96, 1);
        if exe::crt_strstr(t, name.as_ptr() as usize) != 0 {
            exe::text_add_link(out, name.as_ptr(), 0x7db, person);
        }
    }
    if exe::crt_strstr(t, club + club::NAME) != 0 {
        exe::text_add_link(out, (club + club::NAME) as *const u8, 0x7d5, club);
    }
}

/// Comparer for `news_board_confidence_update_text` (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_news_board_confidence_update_text(this: usize, news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I2%%);
    compare_text_writer(out, || o(this, news, body, out), |t| news_board_confidence_update_text(this, news, body, t),
        || format!("news_board_confidence_update_text(body {})", body));
}
