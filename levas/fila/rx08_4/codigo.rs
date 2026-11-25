// ---------- rx08_4: the European World Cup qualifying: the staff records of a nation's fate, its news text and the group draw (wc_europe_league.cpp) ----------

/// The European qualifying's play-off stage index (+0x101, signed byte; −1 before the format is set), after the last
/// group stage (`WC_EUROPE_LAST_GROUP`, +0xff) and the runners-up stage (+0x100).
pub(crate) const WC_EUROPE_PLAY_OFF_STAGE: usize = 0x101;

/// The World Cup's record (`KEY_CLUB_COMP_FIFA_WORLD_CUP` in [`G_CLUB_COMP_TABLE`], 0x6b bytes each) and its
/// competition object ([`G_COMP_OBJECT_TABLE`]).
unsafe fn world_cup_record_and_object() -> (usize, usize) {
    let key = read_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP);
    (read_usize(G_CLUB_COMP_TABLE).wrapping_add(key.wrapping_mul(0x6b)), read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(key.wrapping_mul(4))))
}

/// A nation through to the World Cup from the European qualifying: the World Cup qualification record (weight
/// `weight`), its place in the World Cup (`exe::worldcup_add_qualified_team`) and in the national teams' list of
/// qualified nations (NationalTeams::add_qualified_nation on [`G_NATIONAL_TEAMS`]).
unsafe fn wc_europe_league_qualify_nation(club: usize, weight: u32) {
    let (record, world_cup) = world_cup_record_and_object();
    european_cup_add_qualified_record(club, record, 0, 0, weight);
    exe::worldcup_add_qualified_team(world_cup, club); // rx08_4: the nation enters the World Cup
    let add: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_NATIONALTEAMS_ADD_QUALIFIED_NATION);
    add(read_usize(G_NATIONAL_TEAMS), club, record);
}

/// rule: in the European qualifying a group winner (status 7) gets the World Cup qualification record (weight 100),
/// enters the World Cup and the list of qualified nations, and its status becomes 4; in the play-off the winner (1) gets
/// the same with weight 50 and the loser (2) the knocked-out record (weight 50) and ends its campaign; the other group
/// fates leave no record
///
/// `this.update_staff_records_for_status(club, outcome, stage, out, game, a6)` (virtual +0x44). In the groups (the play-off
/// stage `WC_EUROPE_PLAY_OFF_STAGE` not set, or the stage byte below it, signed): outcome 7, the World Cup
/// (`wc_europe_league_qualify_nation` with 0x64) and `*out` = 4 when there is `out`, −1; outcomes 1 and 0xfe, −1 without
/// any record; any other, 0. In the play-off stage itself: outcome 1, the World Cup with 0x32 (`*out` untouched), −1;
/// outcome 2, `european_cup_add_knocked_out_record`(club, `league::COMPETITION`, the fixture's `fixture::ROUND_PREFIX`,
/// its `fixture::ROUND_NAME`, 0x32) and the national team's campaign ended (`clear_national_team_competition`), 0; any
/// other, 0. Past the play-off stage, 0.
/// Original: `WcEuropeLeague::update_staff_records_for_status` (0x918980, wc_europe_league.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, out: usize, game: usize, _a6: u32) -> u32 {
    let play_off = read_u8(this + WC_EUROPE_PLAY_OFF_STAGE) as i8;
    let stage = stage as u8 as i8;
    if play_off == -1 || stage < play_off {
        return match outcome as u8 {
            7 => {
                covered!("rx08_4 european group winner to the world cup");
                wc_europe_league_qualify_nation(club, 0x64);
                if out != 0 {
                    write_u8(out, 4);
                }
                u32::MAX
            }
            1 | 0xfe => u32::MAX,
            _ => 0,
        };
    }
    if stage != play_off {
        return 0;
    }
    match outcome as u8 {
        1 => {
            covered!("rx08_4 european play-off winner");
            wc_europe_league_qualify_nation(club, 0x32);
            u32::MAX
        }
        2 => {
            covered!("rx08_4 european play-off loser");
            // simplification: the original pushes the two words with the high halves of the registers; the callee reads words
            european_cup_add_knocked_out_record(club, League(this).competition().addr(), read_u16(game + fixture::ROUND_PREFIX) as u32, read_u16(game + fixture::ROUND_NAME) as u32, 0x32);
            clear_national_team_competition(Club(club)); // rx08_4: the play-off loser ends its campaign
            0
        }
        _ => 0,
    }
}

/// Comparer for `wc_europe_league_update_staff_records_for_status`: the records, the World Cup place and the end of the
/// campaign do not run twice, so only the original runs; the plan: the return, `*out` = 4 for a group winner with
/// `out`, and the World Cup's entrant count one more (when it had room) for a nation through.
pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, out: usize, game: usize, a6: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, usize, u32) -> u32>(%%I0%%);
    let play_off = read_u8(this + WC_EUROPE_PLAY_OFF_STAGE) as i8;
    let s = stage as u8 as i8;
    let in_groups = play_off == -1 || s < play_off;
    let through = (in_groups && outcome as u8 == 7) || (!in_groups && s == play_off && outcome as u8 == 1);
    let expected = if through || (in_groups && matches!(outcome as u8, 1 | 0xfe)) { u32::MAX } else { 0 };
    let world_cup = League(world_cup_record_and_object().1);
    if through && world_cup.is_null() {
        return o(this, club, outcome, stage, out, game, a6);
    }
    let n = if through { world_cup.entrant_count() } else { 0 };
    let room = through && n < world_cup.entrant_capacity();
    let r = o(this, club, outcome, stage, out, game, a6);
    let out_ok = !(in_groups && outcome as u8 == 7 && out != 0) || read_u8(out) == 4;
    let list_ok = !through || world_cup.entrant_count() == if room { n + 1 } else { n };
    SCOREBOARD.record(r == expected && out_ok && list_ok, || format!("wc_europe_league_update_staff_records_for_status({:#x}, {}, stage {}) returned {:#x} (expected {:#x}), list {}", club, outcome as u8, s, r, expected, n));
    r
}

/// Prints one news sentence of a nation and a competition into the global buffer 0xde1f64 (`FN_LANGLIB_SPRINTF`: the
/// template, the club's gender twice, the competition's gender twice, the club's short name and the competition's
/// `name`), sets it as the text and links the club (0x7d5) and the competition `linked` (its short name, 0x7d0).
unsafe fn wc_europe_league_fate_sentence(text: usize, template: usize, club: Club, comp: usize, name: usize, linked: usize) {
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let club_gender = club.short_gender_name() as i8 as i32;
    let comp_gender = read_u8(comp + competition::SHORT_GENDER_NAME) as i8 as i32;
    let club_name = club.addr() + club::SHORT_NAME;
    sprintf(0xde1f64, template, club_gender, club_gender, comp_gender, comp_gender, club_name, name);
    exe::text_set_string(text, 0xde1f64 as *const u8);
    exe::text_add_link(text, club_name as *const u8, 0x7d5, club);
    exe::text_add_link(text, (linked + competition::SHORT_NAME) as *const u8, 0x7d0, linked);
}

/// rule: the news text of a nation's fate in the European qualifying: in the groups the winner "qualify for" the World
/// Cup (the full sentence is the shared group one) and the eliminated get the shared group sentence; the best runner-up
/// "are the best second placed team" (or "one of the three best", with one exempt nation) of the qualifying, "qualify
/// for" the World Cup in the short sentence; in the play-off the shared cup tie sentence, or in short "qualify for" the
/// World Cup for the winner and "out of" the European Championship for the loser
///
/// The arguments as `world_cup_build_fate_news_text` (the club index `club_index` in [`G_CLUB_TABLE`], the fate byte
/// `fate`, `stage`, the stage id `a3` (a signed word), the fixture `a4`, `a5`, `a6`, `a7`, `full_sentence` and the Text
/// `text`).
/// - Stage ids 1000 to 1009 (the groups): fate 7 with the full sentence, the shared group sentence
///   (`exe::comp_util_write_group_qualify_news`(club_index, a3, the competition, 7, full_sentence, text)); fate 7
///   without it, 0x9c470c with the World Cup's short name; fate 0xfe, the group sentence with −2; any other, nothing.
/// - Stage id 1020 (the runners-up) and fate 7: with the full sentence, 0x9c4800 when the exempt count is 1 (one of the
///   three best runners-up) or 0x9c4764 otherwise (the best), with the competition's long name (`competition::NAME`), the
///   club linked and the competition's long name linked (0x7d0); without it, 0xad4ab0 with the World Cup.
/// - Otherwise only with a fixture `a4` whose `fixture::ROUND_NAME` is 0xa4 (the play-off): with the full sentence, the
///   shared cup tie sentence (`exe::comp_util_write_cup_tie_result_news`(club_index, a4, a7, fate, a5, text)); without
///   it, fate 1 0xad4ab0 with the World Cup, any other fate 0x9c46b8 "out of" with the European Championship's short
///   name (`KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP`), still linked to the World Cup.
/// The short sentences go through `wc_europe_league_fate_sentence` (the club's link first, then the World Cup's).
/// Original: `WcEuropeLeague::build_fate_news_text` (0x918b00, wc_europe_league.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn wc_europe_league_build_fate_news_text(this: usize, club_index: u32, fate: u32, stage: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32, full_sentence: u32, text: usize) {
    let _ = (stage, a6);
    let league = League(this);
    let comp = league.competition().addr();
    let club = club_at(club_index as i32 as isize);
    let (world_cup, _) = world_cup_record_and_object();
    let to_world_cup = |template: usize| wc_europe_league_fate_sentence(text, template, club, world_cup, world_cup + competition::SHORT_NAME, world_cup);
    let stage_id = a3 as u16 as i16;
    // simplification: the original leaves al with the fate byte or a callee's eax; the callers do not read it
    if (0x3e8..=0x3f1).contains(&stage_id) {
        match fate as u8 {
            7 if full_sentence != 0 => {
                covered!("rx08_4 european group winner, full sentence");
                exe::comp_util_write_group_qualify_news(club_index, a3, comp, 7, full_sentence, text);
            }
            7 => {
                covered!("rx08_4 european group winner");
                to_world_cup(0x9c470c);
            }
            0xfe => {
                covered!("rx08_4 european group out");
                exe::comp_util_write_group_qualify_news(club_index, a3, comp, -2, full_sentence, text); // rx08_4: the group exit sentence
            }
            _ => {}
        }
        return;
    }
    if stage_id == 0x3fc && fate as u8 == 7 {
        if full_sentence == 0 {
            to_world_cup(0xad4ab0);
            return;
        }
        covered!("rx08_4 european best runner-up");
        let template = if read_u8(this + WC_EUROPE_EXEMPT) == 1 { 0x9c4800 } else { 0x9c4764 }; // rx08_4: one exempt nation, three best runners-up
        let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
        let club_gender = club.short_gender_name() as i8 as i32;
        let comp_gender = read_u8(comp + competition::SHORT_GENDER_NAME) as i8 as i32;
        sprintf(0xde1f64, template, club_gender, club_gender, comp_gender, comp_gender, club.addr() + club::SHORT_NAME, comp + competition::NAME);
        exe::text_set_string(text, 0xde1f64 as *const u8);
        exe::text_add_link(text, (club.addr() + club::SHORT_NAME) as *const u8, 0x7d5, club);
        exe::text_add_link(text, (comp + competition::NAME) as *const u8, 0x7d0, comp);
        return;
    }
    if a4 == 0 || read_u16(a4 as usize + fixture::ROUND_NAME) != 0xa4 {
        return;
    }
    covered!("rx08_4 european play-off fate");
    if full_sentence != 0 {
        exe::comp_util_write_cup_tie_result_news(club_index, a4, a7, fate, a5, text);
    } else if fate as u8 == 1 {
        to_world_cup(0xad4ab0);
    } else {
        let euro = read_usize(G_CLUB_COMP_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP).wrapping_mul(0x6b));
        wc_europe_league_fate_sentence(text, 0x9c46b8, club, euro, euro + competition::SHORT_NAME, world_cup); // rx08_4: the loser "out of" the European Championship
    }
}

/// Comparer for `wc_europe_league_build_fate_news_text`: side by side into two Texts (`compare_text_writer`).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_build_fate_news_text(this: usize, club_index: u32, fate: u32, stage: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32, full_sentence: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize)>(%%I1%%);
    compare_text_writer(text, || o(this, club_index, fate, stage, a3, a4, a5, a6, a7, full_sentence, text),
        |t| wc_europe_league_build_fate_news_text(this, club_index, fate, stage, a3, a4, a5, a6, a7, full_sentence, t),
        || format!("wc_europe_league_build_fate_news_text(club {}, fate {}, stage id {}, full {})", club_index, fate as u8, a3 as u16 as i16, full_sentence));
}

/// Deals the European qualifying's entry pool (+0xee, already sorted by strength; its count +0xf2, a dword) into the
/// group slots `slots` (six per group): pot by pot (at most six), the group order shuffled (20 swaps of two drawn
/// places, `exe::utils_random`(groups) twice each), then each group in that order takes the next nation while it has
/// room (its size from WcEuropeLeague::get_group_size of the group index less one, a signed word). Returns how many
/// nations each group got.
unsafe fn wc_europe_league_deal_groups(this: usize, slots: usize, groups: i8) -> [i8; 10] {
    let size: unsafe extern "thiscall" fn(usize, u32) -> i16 = core::mem::transmute(FN_WC_EUROPE_LEAGUE_GET_GROUP_SIZE);
    let mut filled = [0i8; 10];
    let mut order: [i8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut next = 0i8;
    let pool_left = |next: i8| (next as i32) < read_i32(this + 0xf2);
    for _pot in 0..6 {
        if !pool_left(next) {
            break;
        }
        for _ in 0..20 {
            let a = exe::utils_random(groups as i32) as i8 as usize;
            let b = exe::utils_random(groups as i32) as i8 as usize;
            order.swap(a, b);
        }
        let mut k = 0i16;
        while k < groups as i16 && pool_left(next) {
            let g = order[k as usize];
            if g < groups {
                let had = filled[g as usize];
                if (had as i16) < size(this, g.wrapping_sub(1) as u8 as u32) { // rx08_4: the group's size by its index less one
                    let nation = read_usize(read_usize(this + 0xee) + next as usize * 4);
                    write_usize(slots + (g as usize * 6 + had as usize) * 4, nation);
                    next += 1;
                    filled[g as usize] = had + 1;
                }
            }
            k += 1;
        }
    }
    filled
}

/// rule: the European qualifying draw: the nations, strongest first, go into the groups pot by pot; in each pot the
/// groups are shuffled and each one with room takes the next nation, so every pot gives one nation to each group
///
/// The group count is the last group stage (`WC_EUROPE_LAST_GROUP`, signed byte) + 2 (the section's own group and the
/// group stages); a slot table of six per group (`exe::crt_malloc` of the signed byte groups × 6, times 4) in +0xf7
/// (without memory, the error box of line 0x27b and nothing else), dealt by `wc_europe_league_deal_groups`. The unused
/// slots keep what `malloc` left.
/// Original: `WcEuropeLeague::draw_groups` (0x918ec0, wc_europe_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_draw_groups(this: usize) {
    let groups = (read_u8(this + WC_EUROPE_LAST_GROUP) as i8).wrapping_add(2);
    let slots = exe::crt_malloc(((groups.wrapping_mul(6) as i32) << 2) as u32);
    write_usize(this + 0xf7, slots);
    if slots == 0 {
        show_error_at(wc_cup_turnover_consts::WC_EUROPE_LEAGUE_CPP, 0x27b);
        return;
    }
    covered!("rx08_4 european group draw");
    wc_europe_league_deal_groups(this, slots, groups);
}

/// Comparer for `wc_europe_league_draw_groups`: the original draws into its own slot table; the generator goes back to
/// where it was and the copy deals the same draws into a table of its own (its draws not counted); the dealt slots of
/// every group must match. The generator is left as the original left it.
pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_draw_groups(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    let rng_before = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS);
    o(this);
    let slots = read_usize(this + 0xf7);
    if slots == 0 {
        return;
    }
    let rng_after = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS);
    rng_before.write_back();
    let groups = (read_u8(this + WC_EUROPE_LAST_GROUP) as i8).wrapping_add(2);
    let mut mine = [0usize; 60];
    let filled = crate::rng::pause_trace(|| crate::rng::without_counting(|| wc_europe_league_deal_groups(this, mine.as_mut_ptr() as usize, groups)));
    rng_after.write_back();
    let wrong = (0..groups.clamp(0, 10) as usize).flat_map(|g| (0..filled[g].clamp(0, 6) as usize).map(move |k| g * 6 + k))
        .find(|&s| read_usize(slots + s * 4) != mine[s]);
    SCOREBOARD.record(wrong.is_none(), || format!("wc_europe_league_draw_groups({:#x}): slot {:?} original {:#x} rust {:#x}", this, wrong, wrong.map_or(0, |s| read_usize(slots + s * 4)), wrong.map_or(0, |s| mine[s])));
}
