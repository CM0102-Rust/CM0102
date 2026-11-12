// ---------- batch %%LEVA%%: the mini league table of a set of clubs (sub_league.cpp) ----------

/// Rule: a mini league of a given set of clubs (e.g. the runners-up of the qualifying groups compared among
/// themselves): a table of those clubs filled from the fixtures they played, with the given points per win and draw and
/// tie-breaks, then sorted.
///
/// `SubLeague::SubLeague(count, club_ids, a3, a4, a5, a6, comp_id, win_points, draw_points, tiebreaks, a11, a12)` (12
/// dwords, ret 0x30; the SEH frame only covers the League base). The League base (`FN_LEAGUE_LEAGUE` League::League,
/// thiscall, ret 0); `league::WIN_POINTS` = the byte `win_points`, `league::DRAW_POINTS` = the byte `draw_points`; the
/// vtable 0x96ff90; the four tie-break codes (`league::TIEBREAKS`) copied from `tiebreaks`; the dword
/// `league::PROMOTED_COUNT` (+0xbe..+0xc1) = 0, +0x2c = 0, `league::CURRENT_STAGE` = −1, +0x44 = 0xfe,
/// `league::LEGS` = 0, +0xc = 0, +8 = 0, +0xdd = 1; the competition +4 = the record of `comp_id` ([`G_CLUB_COMP_TABLE`]
/// + 0x6b × id) when 0 ≤ id < [0xae2378], otherwise null. `league::CLUB_COUNT` = `count` (word); the table
/// (`exe::crt_malloc`(`league_table_row::SIZE` × count); out of memory: the error box at line 0x56 and this); each row i
/// (a signed byte index) with the club of `club_ids`[i] (the club table [`G_CLUB_TABLE`] + `club::SIZE` × id;
/// League::init_table_row `exe::league_init_table_row`(this, i, club, 0, 0): push 0; push 0; push club; push i ->
/// thiscall, ret 0x10; the original pushes i in a dword with stack garbage above the byte, which the routine does not
/// read: the copy passes the clean byte); the fill from the fixtures (SubLeague 0x88c9f0(this, a3, a4, a5, a6, comp_id,
/// a11, a12): 7 pushes; mov ecx, this -> thiscall, ret 0x1c; 0: the box at line 0x63 and this); the table sorted
/// (`exe::league_sort_table`(this, 0); 0: the box at line 0x68). Returns this.
/// Original: `SubLeague::SubLeague` (0x88c6d0, sub_league.cpp, thiscall, ret 0x30).
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn construct_sub_league(this: usize, count: u32, club_ids: usize, a3: u32, a4: u32, a5: u32, a6: u32, comp_id: i32, win_points: u32,
    draw_points: u32, tiebreaks: usize, a11: u32, a12: u32) -> usize {
    let l = League(this);
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LEAGUE_LEAGUE);
    base(this);
    write_u8(this + league::WIN_POINTS, win_points as u8); // cqx14: the win points
    write_u8(this + league::DRAW_POINTS, draw_points as u8);
    write_u32(this, 0x96ff90);
    for k in 0..4 {
        write_u8(this + league::TIEBREAKS + k, read_u8(tiebreaks + k));
    }
    write_u32(this + league::PROMOTED_COUNT, 0);
    write_u32(this + 0x2c, 0);
    write_u32(this + league::CURRENT_STAGE, u32::MAX);
    write_u8(this + 0x44, 0xfe);
    write_u16(this + league::LEGS, 0);
    write_u32(this + 0xc, 0);
    write_u32(this + 8, 0);
    write_u32(this + 0xdd, 1);
    let comp = if comp_id >= 0 && comp_id < read_i32(0xae2378) { read_usize(G_CLUB_COMP_TABLE).wrapping_add((comp_id as usize).wrapping_mul(0x6b)) } else { 0 };
    write_usize(this + 4, comp);
    write_u16(this + league::CLUB_COUNT, count as u16);
    let t = exe::crt_malloc((l.club_count() as i32 as u32).wrapping_mul(league_table_row::SIZE as u32));
    l.set_table(t);
    if t == 0 {
        show_error_at(0xa8b464, 0x56);
        return this;
    }
    let n = l.club_count() as i32;
    let mut i = 0i8;
    while (i as i32) < n {
        covered!("cqx_14 sub league row");
        let club = read_usize(G_CLUB_TABLE).wrapping_add(read_usize(club_ids.wrapping_add((i as isize * 4) as usize)).wrapping_mul(club::SIZE));
        exe::league_init_table_row(l, i as u8 as u32, club, 0, 0);
        i = i.wrapping_add(1);
    }
    let fill: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, i32, u32, u32) -> u32 = core::mem::transmute(0x88c9f0usize);
    if fill(this, a3, a4, a5, a6, comp_id, a11, a12) == 0 {
        show_error_at(0xa8b464, 0x63);
        return this;
    }
    if exe::league_sort_table(l, 0) == 0 {
        show_error_at(0xa8b464, 0x68);
    }
    this
}

/// The bytes of a mini league to compare: the counts, points, tie-breaks and competition, and each row's club,
/// position and totals (without the row's head-to-head and history pointers, +5..+0xd).
unsafe fn sub_league_bytes(this: usize) -> Vec<u8> {
    let l = League(this);
    let mut v: Vec<u8> = [4usize, 5, 6, 7, league::WIN_POINTS, league::DRAW_POINTS, league::TIEBREAKS, league::TIEBREAKS + 1, league::TIEBREAKS + 2, league::TIEBREAKS + 3,
        league::CLUB_COUNT, league::CLUB_COUNT + 1].iter().map(|&o| read_u8(this + o)).collect();
    if l.table() != 0 {
        for i in 0..(l.club_count() as i32).clamp(0, 0x100) as usize {
            let r = l.table() + league_table_row::SIZE * i;
            v.extend((0..5).chain(0xd..league_table_row::SIZE).map(|k| read_u8(r + k)));
        }
    }
    v
}

/// Comparer for `construct_sub_league`: the original builds `this`; the copy builds a scratch object (`exe::crt_malloc`
/// of 0x200 zeroed bytes) from the same arguments; the counts, points, tie-breaks, competition and the table rows
/// (`sub_league_bytes`) must be equal; the copy's table is freed (League::free_table `exe::league_free_table`(copy, 0))
/// and then the scratch object.
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn compare_construct_sub_league(this: usize, count: u32, club_ids: usize, a3: u32, a4: u32, a5: u32, a6: u32, comp_id: i32, win_points: u32,
    draw_points: u32, tiebreaks: usize, a11: u32, a12: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize, u32, u32, u32, u32, i32, u32, u32, usize, u32, u32) -> usize>(%%I0%%);
    let r = o(this, count, club_ids, a3, a4, a5, a6, comp_id, win_points, draw_points, tiebreaks, a11, a12);
    let scratch = exe::crt_malloc(0x200);
    if scratch == 0 {
        return r;
    }
    core::ptr::write_bytes(scratch as *mut u8, 0, 0x200);
    construct_sub_league(scratch, count, club_ids, a3, a4, a5, a6, comp_id, win_points, draw_points, tiebreaks, a11, a12);
    let (a, b) = (sub_league_bytes(this), sub_league_bytes(scratch));
    if League(scratch).table() != 0 {
        exe::league_free_table(League(scratch), 0);
    }
    exe::crt_free(scratch);
    SCOREBOARD.record(r == this && a == b, || format!("construct_sub_league({:#x}, {} clubs): the copy's table differs", this, count as u16 as i16));
    r
}

/// The news text "{club} ... {competition}" of the European Cup fates: the text (`exe::langlib_sprintf`(0xde1f64,
/// `format`, the club's byte +0x52 twice, `comp_byte` twice, the club's name +0x38, `comp_name`): 8 pushes; add esp,
/// 0x20 -> cdecl) set in `text` (Text::set_string 0x4ae660(text, 0xde1f64), thiscall, ret 4) with the club link
/// (Text::add_link 0x4ae8a0(text, club name, 0x7d5, club), thiscall, ret 0xc) and the competition link (add_link(text,
/// `link_name`, 0x7d0, `link_data`)).
unsafe fn european_cup_fate_text(text: usize, club_id: u32, format: usize, comp_byte: u8, comp_name: usize, link_data: usize) {
    let club = read_usize(G_CLUB_TABLE).wrapping_add((club_id as usize).wrapping_mul(club::SIZE));
    let b = read_u8(club + 0x52) as i8 as i32;
    let k = comp_byte as i8 as i32;
    let sprintf: unsafe extern "C" fn(usize, usize, i32, i32, i32, i32, usize, usize) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    sprintf(0xde1f64, format, b, b, k, k, club + 0x38, comp_name);
    let set_string: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TEXT_SET_STRING);
    set_string(text, 0xde1f64);
    let add_link: unsafe extern "thiscall" fn(usize, usize, u32, usize) = core::mem::transmute(FN_TEXT_ADD_LINK);
    add_link(text, club + 0x38, 0x7d5, club); // cqx14: the club link
    add_link(text, comp_name, 0x7d0, link_data);
}

/// Rule: the Champions League news of a club's fate: in the qualifying rounds (stage −1), a club going through to
/// phase 1 gets its text and one dropping out goes to the UEFA Cup ("are entered for the UEFA Cup"), unless the round
/// is a base round 0xfa or 0x104 or there is a news object (then the generic news); in the first group phase (stages up
/// to 7) qualification uses the generic cup news and elimination sends the club to the UEFA Cup; in the second group
/// phase (stages 8 to 11) elimination is "knocked out of" the competition; the final stage (12) goes to the generic
/// news.
///
/// `EuropeanCup::build_fate_news_text(club_id, outcome, stage, a4, fixture, a6, a7, a8, news, text)` (10 dwords, ret
/// 0x28). The UEFA Cup record is [`G_CLUB_COMP_TABLE`] + 0x6b × [`KEY_CLUB_COMP_UEFA_CUP`]; the competition's own is +4.
/// Stage −1: if the base round id of the fixture's round word +0x30 (comp_util_get_base_round_id
/// `FN_COMP_UTIL_GET_BASE_ROUND_ID`, cdecl, called twice) is 0xfa or 0x104, or `news` is set: the generic news (Comp
/// 0x48c6d0(this, club_id, outcome, −1, a4, fixture, a6, 0, a8, news, text), thiscall, ret 0x28); otherwise outcome 1:
/// the text 0x9c4a1c with the competition's short name (+0x38, byte +0x52) and its link; other outcomes: the text
/// 0x9c49c0 ("are entered for the UEFA CUP") with the UEFA Cup's short name and link. Stages below 8 (signed): outcome
/// 6, cdecl 0x4b4590(club_id, a4, competition, 6, news, text); 0xfe with `news`: the text 0x9c4940 with the
/// competition's name (+4, byte +0x37) and its link; 0xfe without: the UEFA Cup text 0x9c49c0, whose competition link
/// carries the UEFA Cup's name but the competition's own record as data (as the original). Stages 8 to 11: outcome 6 as
/// above; 0xfe with `news`: the text 0x987ae0 with the competition's name (+4, byte +0x37); without: the text 0x987a80
/// ("knocked out of") with its short name (+0x38, byte +0x52). Stage 12: the generic news with 0xc for the stage.
/// Anything else: nothing (`european_cup_fate_text` writes the texts).
/// Original: `EuropeanCup::build_fate_news_text` (0x584550, european_cup.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn european_cup_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, fixture: usize, a6: u32, _a7: u32, a8: u32, news: u32, text: usize) {
    let (d, f) = (outcome as u8, stage as u8 as i8);
    let comp = read_usize(this + 4);
    let uefa = read_usize(G_CLUB_COMP_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_UEFA_CUP).wrapping_mul(0x6b));
    let generic = |s: i32| {
        let g: unsafe extern "thiscall" fn(usize, u32, u32, i32, u32, usize, u32, u32, u32, u32, usize) = core::mem::transmute(0x48c6d0usize);
        g(this, club_id, outcome, s, a4, fixture, a6, 0, a8, news, text);
    };
    let cup_news = || {
        let g: unsafe extern "C" fn(u32, u32, usize, i32, u32, usize) = core::mem::transmute(0x4b4590usize);
        g(club_id, a4, comp, 6, news, text);
    };
    if f == -1 {
        let base: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(FN_COMP_UTIL_GET_BASE_ROUND_ID);
        let w = read_u16(fixture + 0x30) as u32;
        if base(w) as u16 == 0xfa || base(w) as u16 == 0x104 || news != 0 {
            return generic(-1);
        }
        covered!("cqx_14 champions league qualifying text");
        if d == 1 {
            european_cup_fate_text(text, club_id, 0x9c4a1c, read_u8(comp + 0x52), comp + 0x38, comp);
        } else {
            european_cup_fate_text(text, club_id, 0x9c49c0, read_u8(uefa + 0x52), uefa + 0x38, uefa); // cqx14: dropped into the UEFA Cup
        }
    } else if f < 8 {
        if d == 6 {
            cup_news();
        } else if d == 0xfe {
            if news != 0 {
                european_cup_fate_text(text, club_id, 0x9c4940, read_u8(comp + 0x37), comp + 4, comp);
            } else {
                european_cup_fate_text(text, club_id, 0x9c49c0, read_u8(uefa + 0x52), uefa + 0x38, comp);
            }
        }
    } else if f < 0xc {
        if d == 6 {
            cup_news();
        } else if d == 0xfe {
            if news != 0 {
                european_cup_fate_text(text, club_id, 0x987ae0, read_u8(comp + 0x37), comp + 4, comp);
            } else {
                european_cup_fate_text(text, club_id, 0x987a80, read_u8(comp + 0x52), comp + 0x38, comp);
            }
        }
    } else if f == 0xc {
        generic(0xc);
    }
}

/// Comparer for `european_cup_build_fate_news_text`: on the paths that only write the text (`european_cup_fate_text`)
/// side by side with `compare_text_writer` (the original into the caller's Text, the copy into a local one); on the
/// paths of the generic news (0x48c6d0, 0x4b4590, which may also write the news object) only the original.
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn compare_european_cup_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, fixture: usize, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, usize, u32, u32, u32, u32, usize)>(%%I1%%);
    let (d, f) = (outcome as u8, stage as u8 as i8);
    let text_only = text != 0 && if f == -1 {
        let base: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(FN_COMP_UTIL_GET_BASE_ROUND_ID);
        let w = if fixture != 0 { read_u16(fixture + 0x30) as u32 } else { 0 };
        fixture != 0 && news == 0 && base(w) as u16 != 0xfa && base(w) as u16 != 0x104
    } else {
        f < 0xc && d == 0xfe
    };
    if !text_only {
        return o(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, news, text);
    }
    compare_text_writer(text, || o(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, news, text),
        |t| european_cup_build_fate_news_text(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, news, t),
        || format!("european_cup_build_fate_news_text(club {}, outcome {}, stage {})", club_id, d, f));
}
