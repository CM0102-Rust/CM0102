// ---------- rx11b_3: the physio report: the assessment, the news item and its text (physio.cpp) ----------

/// The physio report's news type.
const PHYSIO_REPORT_NEWS: u32 = 0x1392;

/// Flags of the physio report (the word in news slot 8), set by `physio_assess_player_fitness`.
const PHYSIO_RECURRING_INJURY: u16 = 0x1; // carrying a recurring injury
const PHYSIO_PRONE_STRONG: u16 = 0x2; // a fine athlete prone to injuries (injury proneness above 17)
const PHYSIO_PRONE: u16 = 0x4; // somewhat prone to injuries
const PHYSIO_RARELY_INJURED: u16 = 0x8; // will rarely succumb to injury (injury proneness below 4)
const PHYSIO_NO_CONTACT: u16 = 0x10; // not enough contact with the player to judge
const PHYSIO_STAMINA_TROUBLE: u16 = 0x20; // difficulty in sustaining his fitness (natural fitness above 15)
const PHYSIO_HEALTHY: u16 = 0x40; // no long term concerns (natural fitness below 6)
const PHYSIO_OTHER_CLUB: u16 = 0x80; // the player is at another club
const PHYSIO_TREATMENT: u16 = 0x100; // a treatment is recommended for the recurring injury
const PHYSIO_FAITH_HEALER: u16 = 0x200; // a faith healer rather than a specialist or surgery

const FN_PHYSIO_GET_AGE_STAGE: usize = 0x7a25c0;
const FN_PHYSIO_GET_DAYS_TO_JUDGE: usize = 0x7a2900;

/// Assesses a player for a physio: the report flags (`PHYSIO_*`) and the recurring injury type (0xff none).
///
/// Nothing without the player's sheet (`player()`) or the physio's (`non_player()`). The judgement: (ids of both summed)
/// mod 120 − 60, plus ten times the physio's attribute 10 (`staff_compute_non_player_attribute`). Above (player id
/// mod 40) − injury proneness + 75 with a recurring injury level (`Injury::get_recurring_injury_level`), or with a level
/// above 1 and a world reputation above 4750: the recurring injury (`Injury::get_recurring_injury_type`,
/// `PHYSIO_RECURRING_INJURY`); then, with a level above 1 whose sum with the injury proneness passes 15, a treatment
/// (`PHYSIO_TREATMENT`; a faith healer when the physio id is a multiple of 146), and the player's staff state flag
/// (`staff_state::EXTRA_FLAGS` bit 0) is set. A player at another club than the physio's (his contract's club too,
/// `CONTRACT_MANAGER::get_contract`, which the original leaves in `G_CONTRACT_LOOKUP_RESULT`) is judged with
/// min(0.4 x, x − 75) (`PHYSIO_OTHER_CLUB`); at the same club, when the days to judge (`physio_get_days_to_judge`) pass
/// his days at the club, with min(x / 2, x − 50) (`PHYSIO_NO_CONTACT`). Then the proneness: below 4, rarely injured when
/// the judgement passes (50 − proneness) x 2 or the world reputation 4250; above 17, prone (strongly) when it passes
/// 2 x proneness + 80 or the reputation 4250; else prone. The natural fitness: above 15, trouble sustaining fitness when
/// the judgement passes (50 − fitness) x 2, or with a reputation above 4750 and a fitness above 17; below 6, healthy when
/// it passes 2 x fitness + 80, or with a reputation above 4750 and a fitness below 4.
/// Defect of the original (`DEFEITOS_DO_ORIGINAL`): a query that writes `G_CONTRACT_LOOKUP_RESULT` and the staff state
/// flag as side effects; the copy keeps both.
/// Original: `physio_assess_player_fitness` (0x7a2650, physio.cpp, thiscall, ret 0x10; this is the physio object
/// 0xb74344).
pub(crate) unsafe extern "thiscall" fn physio_assess_player_fitness(this: usize, physio: usize, player: usize, flags: *mut u16, recurring: *mut u8) {
    let (physio, player) = (Staff(physio), Staff(player));
    *flags = 0;
    *recurring = 0xff;
    if player.player().is_null() || physio.non_player().is_null() {
        return;
    }
    let injury = read_usize(G_INJURY);
    let attribute = exe::staff_compute_non_player_attribute(physio, physio.non_player(), 10) as i8 as i32;
    let mut judgement = read_i32(physio.addr()).wrapping_add(read_i32(player.addr())) % 0x78 - 0x3c + attribute * 10;
    let sheet = player.player();
    let mut recurring_found = false;
    if judgement > read_i32(player.addr()) % 0x28 - sheet.injury_proneness() as i32 + 0x4b
        && exe::injury_get_recurring_injury_level(injury, player) != 0 {
        recurring_found = true;
    } else if exe::injury_get_recurring_injury_level(injury, player) as i8 > 1 && sheet.world_reputation() > 0x128e {
        recurring_found = true;
    }
    if recurring_found {
        *recurring = exe::injury_get_recurring_injury_type(injury, player);
        *flags |= PHYSIO_RECURRING_INJURY;
    }
    if *flags & PHYSIO_RECURRING_INJURY != 0 && *flags & PHYSIO_NO_CONTACT == 0 {
        covered!("rx11b_3 recurring injury reported");
        let level = exe::injury_get_recurring_injury_level(injury, player) as i8;
        if level > 1 && sheet.injury_proneness() as i32 + level as i32 > 0xf {
            *flags |= PHYSIO_TREATMENT;
            if read_i32(physio.addr()) % 0x92 == 0 {
                *flags |= PHYSIO_FAITH_HEALER;
            }
        }
        let row = read_usize(G_STAFF_STATE_TABLE).wrapping_add((read_i32(player.addr()) as usize).wrapping_mul(staff_state::SIZE));
        write_u8(row + staff_state::EXTRA_FLAGS, read_u8(row + staff_state::EXTRA_FLAGS) | 1);
    }
    let days = exe::date_days_since(G_CURRENT_DATE, player.date_joined_club() as u32, player.date_joined_club_leap_year() as u32);
    let mut same_club = player.club_job().addr() == physio.club_job().addr();
    if !same_club {
        let contract = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), player);
        write_usize(G_CONTRACT_LOOKUP_RESULT, contract); // the original's side effect
        let club = if contract != 0 { club_at(read_i32(contract + 4) as isize).addr() } else { 0 }; // the contract's club id
        same_club = club == physio.club_job().addr();
    }
    if !same_club {
        covered!("rx11b_3 player at another club");
        *flags |= PHYSIO_OTHER_CLUB;
        let (a, b) = (judgement as f64 * read_f64(rdata::F64_0_4), judgement.wrapping_sub(0x4b) as f64);
        judgement = (if a < b { a } else { b }) as i32;
    } else {
        let days_to_judge: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(FN_PHYSIO_GET_DAYS_TO_JUDGE);
        if days_to_judge(this, physio.addr(), player.addr()) > days {
            *flags |= PHYSIO_NO_CONTACT;
            judgement = (judgement / 2).min(judgement - 0x32);
        }
    }
    let proneness = sheet.injury_proneness() as i32;
    let reputation = sheet.world_reputation();
    if proneness < 4 {
        *flags |= if judgement > (0x32 - proneness) * 2 || reputation > 0x109a { PHYSIO_RARELY_INJURED } else { PHYSIO_PRONE };
    } else if proneness > 0x11 {
        *flags |= if judgement > proneness * 2 + 0x50 || reputation > 0x109a { PHYSIO_PRONE_STRONG } else { PHYSIO_PRONE };
    } else {
        *flags |= PHYSIO_PRONE;
    }
    let fitness = sheet.natural_fitness() as i32;
    if fitness > 0xf {
        if judgement > (0x32 - fitness) * 2 || (reputation > 0x128e && fitness > 0x11) {
            *flags |= PHYSIO_STAMINA_TROUBLE; // rx11b_3: the natural fitness above 15
        }
    } else if fitness < 6 && (judgement > fitness * 2 + 0x50 || (reputation > 0x128e && fitness < 4)) {
        *flags |= PHYSIO_HEALTHY;
    }
}

/// Files a physio's report on a player as news (`PHYSIO_REPORT_NEWS`) to the physio's club manager.
///
/// Nothing without both, or when either has no club (`club_job()`). The familiarity: 1 after more than 1000 days at
/// the club with a loyalty above 10, else 4 when the days at the club reach 5 x (21 − adaptability) + 90, else 2. The
/// slots: 0..3 the physio (`set_news_person_slots`), 4..7 the player, 8 the flags and 9 the recurring injury type
/// (`physio_assess_player_fitness`), 10 the age stage (`physio_get_age_stage`), 11 the familiarity, 12 and 13 the two
/// trait words (`player_search_assess_player_traits`), 14 and 15 the current injury's type and duration
/// (`Injury::get_player_injury`, +0 and +0x2c; −1 and −1 without one). Delivered by `News::send_to_club_manager` to the
/// physio's club.
/// Original: `physio_file_report_news` (0x7a1a90, physio.cpp, thiscall, ret 8; this is the physio object 0xb74344).
pub(crate) unsafe extern "thiscall" fn physio_file_report_news(this: usize, physio: usize, player: usize) {
    physio_report_news(this, physio, player, &mut |news, club| {
        exe::news_send_to_club_manager(read_usize(G_NEWS), news, club);
    });
}

/// The physio report news item of `physio_file_report_news`, handed to `send` with the physio's club.
unsafe fn physio_report_news(this: usize, physio: usize, player: usize, send: &mut dyn FnMut(usize, Club)) {
    let (physio, player) = (Staff(physio), Staff(player));
    if physio.is_null() || physio.club_job().is_null() || player.is_null() || player.club_job().is_null() {
        return;
    }
    let days = exe::date_days_since(G_CURRENT_DATE, player.date_joined_club() as u32, player.date_joined_club_leap_year() as u32);
    let familiarity: i8 = if days > 1000 && player.loyalty() > 10 {
        1
    } else if days >= (0x15 - player.adaptability() as i32) * 5 + 0x5a {
        4
    } else {
        2
    };
    let age_stage: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_PHYSIO_GET_AGE_STAGE);
    let stage = age_stage(this, player.addr()) as i8;
    let (mut trait1, mut trait2) = (0i16, 0i16);
    exe::player_search_assess_player_traits(player, physio, &mut trait1, &mut trait2);
    let (mut flags, mut recurring) = (0u16, 0u8);
    physio_assess_player_fitness(this, physio.addr(), player.addr(), &mut flags, &mut recurring);
    covered!("rx11b_3 physio report filed");
    let mut item = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let news = item.as_mut_ptr() as usize;
    exe::new_newsitem(news, PHYSIO_REPORT_NEWS, 0);
    set_news_person_slots(news, 0, physio);
    set_news_person_slots(news, 4, player);
    set_news_slot(news, 8, flags as i16 as u32);
    set_news_slot(news, 9, recurring as i8 as u32);
    set_news_slot(news, 0xa, stage as u32); // rx11b_3: the age stage
    set_news_slot(news, 0xb, familiarity as u32);
    set_news_slot(news, 0xc, trait1 as u32);
    set_news_slot(news, 0xd, trait2 as u32);
    let injury = exe::injury_get_player_injury(read_usize(G_INJURY), player);
    if injury != 0 {
        set_news_slot(news, 0xe, read_i8(injury) as u32); // the injury type
        set_news_slot(news, 0xf, read_i16(injury + 0x2c) as u32); // its days left [inference]
    } else {
        set_news_slot(news, 0xe, u32::MAX);
        set_news_slot(news, 0xf, u32::MAX);
    }
    send(news, physio.club_job());
}

/// The injury types the physio sends to a specialist rather than to surgery (`physio_write_report_news_text`).
const SPECIALIST_INJURIES: [u32; 8] = [0xb, 0x11, 0x17, 0x1e, 0x23, 0x29, 0x3b, 0x4c];

/// One injury sentence of the physio report: the name of injury `kind` (`injury_get_type_name`, 100 bytes) into `name`
/// and its gender (`injury_get_type_name_gender`), asked twice as the original does.
unsafe fn physio_injury(kind: u32, name: &mut [u8; 0x64]) -> (i32, i32) {
    let injury = read_usize(G_INJURY);
    exe::injury_get_type_name(injury, kind, name.as_mut_ptr() as usize, 0x64, 1);
    let first = exe::injury_get_type_name_gender(injury, kind) as i8 as i32;
    let second = exe::injury_get_type_name_gender(injury, kind) as i8 as i32;
    (first, second)
}

/// Writes the headline or the body of a physio report news item (`physio_file_report_news`'s slots).
///
/// The headline, by (player id + physio id) mod 3 (signed): 0 "Physio <physio> reports on <player>", 1 "<player>
/// medical report", otherwise "<physio> gives <player> verdict" (names in form 5).
///
/// The body (names in form 1) joins up to three paragraphs (`%s\n\n%s`, `%s\n\n%s\n\n%s`, by the CRT sprintf):
/// - the fitness verdict by the flags: exactly `PHYSIO_NO_CONTACT`, not enough contact; `PHYSIO_PRONE_STRONG` (with
///   `PHYSIO_STAMINA_TROUBLE` or `PHYSIO_HEALTHY` worded apart), `PHYSIO_RARELY_INJURED`, `PHYSIO_RECURRING_INJURY` long
///   term concerns, otherwise no (long term) concerns; with no flags at all, the player's attitude from the trait words'
///   bit 0x20 (professional, else lax) as a further paragraph;
/// - the injuries: the current one (type in slot 14, `format_duration_text` of slot 15) with or without the recurring one
///   (slot 9) when the same, the recurring one in addition, or alone by the flags ("This is because", "also believes",
///   "However");
/// - with `PHYSIO_TREATMENT` and a recurring injury, the treatment: a faith healer (`PHYSIO_FAITH_HEALER`), a specialist
///   (`SPECIALIST_INJURIES`) or surgery; then the attitude paragraph is not written.
/// The links (0x7d7): both names of the physio and of the player when their name ids still match
/// (`database_staff_matches_names`).
/// Original: `physio_write_report_news_text` (0x7a1cd0, physio.cpp, thiscall, ret 0xc; this is not read).
pub(crate) unsafe extern "thiscall" fn physio_write_report_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let fmt = news_text_format();
    let crt_format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let s: [u32; 16] = core::array::from_fn(|k| exe::newsitem_get_slot(news, k as u32));
    let (physio_id, player_id, flags, recurring) = (s[0], s[4], s[8], s[9]);
    let mut physio1 = [0u8; 0xc8];
    let mut physio5 = [0u8; 0xc8];
    let mut player1 = [0u8; 0xc8];
    let mut player5 = [0u8; 0xc8];
    exe::database_format_name_from_ids(s[1], s[2], s[3], physio1.as_mut_ptr(), 0xc8, 1, physio_id);
    exe::database_format_name_from_ids(s[1], s[2], s[3], physio5.as_mut_ptr(), 0xc8, 5, physio_id);
    exe::database_format_name_from_ids(s[5], s[6], s[7], player1.as_mut_ptr(), 0xc8, 1, player_id);
    exe::database_format_name_from_ids(s[5], s[6], s[7], player5.as_mut_ptr(), 0xc8, 5, player_id);
    let (ph1, ph5, pl1, pl5) = (physio1.as_ptr() as usize, physio5.as_ptr() as usize, player1.as_ptr() as usize, player5.as_ptr() as usize);
    if full == 0 {
        match (player_id as i32).wrapping_add(physio_id as i32) % 3 {
            0 => fmt(NEWS_TEXT_BUFFER, 0xa6a400, 0x64, 0x64, ph5, pl5), // "Physio <physio> reports on <player>"
            1 => fmt(NEWS_TEXT_BUFFER, 0xa6a458, 0x64, pl5), // "<player> medical report"
            _ => fmt(NEWS_TEXT_BUFFER, 0xa6a48c, 0x64, 0x64, ph5, pl5), // "<physio> gives <player> verdict"
        };
    } else {
        covered!("rx11b_3 physio report body");
        let mut verdict = [0u8; 0x3e8];
        let mut injuries = [0u8; 0x3e8];
        let mut attitude = [0u8; 0x3e8];
        let (vp, ip, ap) = (verdict.as_mut_ptr() as usize, injuries.as_mut_ptr() as usize, attitude.as_mut_ptr() as usize);
        let current = s[14];
        let flag = |f: u16| flags & f as u32 != 0;
        if flags == PHYSIO_NO_CONTACT as u32 {
            fmt(vp, 0xa6b128, ph1, pl1); // "not had enough contact"
        } else {
            let template = if flag(PHYSIO_PRONE_STRONG) {
                if flag(PHYSIO_STAMINA_TROUBLE) { 0xa6b060 } else if flag(PHYSIO_HEALTHY) { 0xa6af90 } else { 0xa6aee8 }
            } else if flag(PHYSIO_RARELY_INJURED) {
                0xa6ae44
            } else if flag(PHYSIO_RECURRING_INJURY) {
                0xa6adac // "long term concerns"
            } else if current != u32::MAX {
                0xa6ad18 // "no long term concerns"
            } else {
                0xa6ac90 // "no concerns"
            };
            fmt(vp, template, 0x64, 0x64, ph1, pl1);
        }
        let mut has_attitude = false;
        if flags == 0 {
            if s[12] as u8 & 0x20 != 0 {
                fmt(ap, 0xa6ac04, 0x64, pl5); // "professional approach to his health"
                has_attitude = true;
            } else if s[13] as u8 & 0x20 != 0 {
                fmt(ap, 0xa6ab9c, 0x64, pl5); // "a very lax approach"
                has_attitude = true;
            }
        }
        let mut has_injuries = false;
        let (mut current_name, mut recurring_name, mut period) = ([0u8; 0x64], [0u8; 0x64], [0u8; 0x64]);
        let (cn, rn, pp) = (current_name.as_mut_ptr() as usize, recurring_name.as_mut_ptr() as usize, period.as_mut_ptr() as usize);
        let recurring_known = recurring as i32 > -1;
        if current != u32::MAX {
            has_injuries = true;
            exe::injury_get_type_name(read_usize(G_INJURY), current, cn, 0x64, 1);
            exe::format_duration_text(s[15] as i32, pp, 0x64, 1);
            if recurring_known {
                exe::injury_get_type_name(read_usize(G_INJURY), recurring, rn, 0x64, 1);
            }
            if flag(PHYSIO_RECURRING_INJURY) && recurring == current {
                let (g1, g2) = physio_injury(current, &mut current_name);
                fmt(ip, 0xa6aa94, 0x64, g2, g1, pl5, cn, pp, ph5); // the current injury, likely to recur
            } else if flag(PHYSIO_RECURRING_INJURY) && recurring_known {
                let (g1, g2) = physio_injury(recurring, &mut recurring_name);
                fmt(ip, 0xa6a9b0, 0x64, 0x64, g2, g1, ph1, pl1, rn); // "In addition to his current injury"
            } else {
                let (g1, g2) = physio_injury(current, &mut current_name);
                fmt(ip, 0xa6a90c, 0x64, g2, g1, pl5, cn, pp); // the current injury
            }
        } else if recurring_known {
            has_injuries = true;
            let template = if flag(PHYSIO_RECURRING_INJURY) { 0xa6a840 } else if flag(PHYSIO_PRONE_STRONG) { 0xa6a778 } else { 0xa6a6b0 };
            let (g1, g2) = physio_injury(recurring, &mut recurring_name);
            fmt(ip, template, 0x64, 0x64, g2, g1, ph1, pl1, rn); // "This is because" / "also believes" / "However"
        }
        let (vp, ip, ap) = (verdict.as_ptr() as usize, injuries.as_ptr() as usize, attitude.as_ptr() as usize);
        if flag(PHYSIO_TREATMENT) && recurring_known {
            covered!("rx11b_3 physio treatment");
            let mut treatment = [0u8; 0x400]; // the original has 0xc8 bytes before its form-5 physio name
            let tp = treatment.as_mut_ptr() as usize;
            let template = if flag(PHYSIO_FAITH_HEALER) {
                0xa6a66c // "a faith healer"
            } else if SPECIALIST_INJURIES.contains(&recurring) {
                0xa6a5a8 // "a specialist"
            } else {
                0xa6a4ec // "surgery"
            };
            fmt(tp, template);
            if !has_injuries {
                crt_format(NEWS_TEXT_BUFFER, 0x991948, vp, tp); // "%s\n\n%s"
            } else if verdict[0] != 0 {
                crt_format(NEWS_TEXT_BUFFER, 0xa6a4e0, vp, ip, tp); // "%s\n\n%s\n\n%s"
            } else {
                crt_format(NEWS_TEXT_BUFFER, 0x991948, ip, tp);
            }
        } else if has_injuries {
            if has_attitude {
                crt_format(NEWS_TEXT_BUFFER, 0xa6a4e0, vp, ap, ip);
            } else {
                crt_format(NEWS_TEXT_BUFFER, 0x991948, vp, ip);
            }
        } else if has_attitude {
            crt_format(NEWS_TEXT_BUFFER, 0x991948, vp, ap);
        } else {
            crt_format(NEWS_TEXT_BUFFER, 0x991944, vp); // "%s"
        }
    }
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    let physio = staff_at(physio_id as i32 as isize);
    if exe::database_staff_matches_names(physio, s[1], s[2], s[3], 0usize) != 0 {
        exe::text_add_link(out, physio1.as_ptr(), 0x7d7, physio);
        exe::text_add_link(out, physio5.as_ptr(), 0x7d7, physio);
    }
    let player = staff_at(player_id as i32 as isize);
    if exe::database_staff_matches_names(player, s[5], s[6], s[7], 0usize) != 0 {
        exe::text_add_link(out, player1.as_ptr(), 0x7d7, player);
        exe::text_add_link(out, player5.as_ptr(), 0x7d7, player);
    }
}

/// Comparer for `physio_assess_player_fitness`: side by side over the two outputs, `G_CONTRACT_LOOKUP_RESULT` and the
/// player's staff state flags.
pub(crate) unsafe extern "thiscall" fn compare_physio_assess_player_fitness(this: usize, physio: usize, player: usize, flags: *mut u16, recurring: *mut u8) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, *mut u16, *mut u8)>(%%I0%%);
    let row = if player != 0 {
        read_usize(G_STAFF_STATE_TABLE).wrapping_add((read_i32(player) as usize).wrapping_mul(staff_state::SIZE)) + staff_state::EXTRA_FLAGS
    } else {
        G_CONTRACT_LOOKUP_RESULT
    };
    let regions = [(flags as usize, 2), (recurring as usize, 1), (G_CONTRACT_LOOKUP_RESULT, 4), (row, 1)];
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, physio, player, flags, recurring),
        || physio_assess_player_fitness(this, physio, player, flags, recurring));
    SCOREBOARD.record(same, || format!("physio_assess_player_fitness(physio {:#x}, player {:#x}) {:x?} x {:x?}", physio, player, mo, mr));
}

/// Comparer for `physio_file_report_news`: the news goes to the inbox, so only the original runs. The plan first: the
/// item the copy would send; after the original it must be the newest of the physio's club manager's inbox (when he is
/// human), with the same type and the same 16 slots.
pub(crate) unsafe extern "thiscall" fn compare_physio_file_report_news(this: usize, physio: usize, player: usize) {
    let mut planned: Option<(Club, Vec<u8>)> = None;
    physio_report_news(this, physio, player, &mut |news, club| {
        planned = Some((club, core::slice::from_raw_parts(news as *const u8, news_item::DATA + 4 * 16).to_vec()));
    });
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I1%%)(this, physio, player);
    let Some((club, bytes)) = planned else { return SCOREBOARD.record(true, String::new) };
    let manager = club.manager();
    let idx = if manager.is_null() { -1 } else { read_i32(manager.addr()).wrapping_sub(read_i32(G_STAFF_COUNT)).wrapping_add(0x10) };
    if idx < 0 {
        return SCOREBOARD.record(true, String::new); // not a human: nothing delivered
    }
    let news_obj = read_usize(G_NEWS);
    let e = read_usize(news_obj + news_fields::HUMAN_INBOXES) + idx as usize * 12;
    let mut last = read_i32(e + 4) - 1;
    if last < read_i32(e) {
        last += 0x64;
    }
    let item = read_usize(news_obj).wrapping_add((last as usize).wrapping_mul(NEWS_NEWS_RECORD_SIZE));
    let stored = core::slice::from_raw_parts(item as *const u8, bytes.len());
    SCOREBOARD.record(stored[..4] == bytes[..4] && stored[news_item::DATA..] == bytes[news_item::DATA..],
        || format!("physio_file_report_news(physio {:#x}, player {:#x}): stored {:x?} planned {:x?}", physio, player, &stored[news_item::DATA..], &bytes[news_item::DATA..]));
}

/// Comparer for `physio_write_report_news_text`: the original into the caller's Text, the copy into a local one
/// (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_physio_write_report_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I2%%);
    compare_text_writer(out, || o(this, news, full, out), |t| physio_write_report_news_text(this, news, full, t),
        || format!("physio_write_report_news_text(news {:#x}, full {})", news, full));
}
