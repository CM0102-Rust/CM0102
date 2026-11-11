// ---------- %%LEVA%%a leva: the inter-continental cups: the two clubs of the Intercontinental Cup and of the Interamerican Cup; the Merconorte predicted positions (world_club_cup.cpp, inter_amer_cup.cpp, con_merc_cup.cpp) ----------

/// Is the club's continent ([[`club::NATION`] + `nation::CONTINENT`]] (dword)) the one of the key `continent`?
unsafe fn intercontinental_club_in_continent(c: usize, continent: usize) -> bool {
    let n = read_usize(c + club::NATION);
    if n == 0 {
        return false;
    }
    let k = read_usize(n + nation::CONTINENT);
    k != 0 && read_usize(k) == read_usize(continent)
}

/// One place of an inter-continental cup. If the continent (key `continent`; the record [`G_CONTINENT_TABLE`] + 0xc6 ×
/// key) has a loaded league (0x6699d0(continent): push; add esp, 4 -> cdecl) or the cup's year (`cup::YEAR`) is 2001:
/// the latest club at place `place` of the competition of key `comp` (`exe::comp_util_get_latest_placed_club`(record,
/// place), cdecl; the winner is place 0), returning (club, true, false). Otherwise, the clubs (up to the first national
/// team, `exe::database_is_national_team`, cdecl) of the continent with a reputation (`club::REPUTATION`, word) above
/// `min_reputation`, in a local list; with any, the list sorted and shuffled with 1 pick
/// (`exe::comp_util_sort_and_shuffle_top_clubs`(list, n, 1), cdecl) and the first one, (club, false, false); with none,
/// the error box of line `line` and the competition's winner, which the original ALWAYS writes in place 1 (+6), even
/// when looking for place 0 (a defect: place 0 keeps whatever the allocated memory had); returns (club, false, true).
#[allow(clippy::too_many_arguments)]
unsafe fn intercontinental_cup_slot(this: usize, continent: usize, min_reputation: i16, comp: usize, place: u32, file: usize, line: u32) -> (usize, bool, bool) {
    let league_loaded: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x6699d0usize);
    let record = read_usize(G_CONTINENT_TABLE).wrapping_add(read_usize(continent).wrapping_mul(0xc6));
    if league_loaded(record) != 0 || read_i16(this + cup::YEAR) == 0x7d1 {
        return (exe::comp_util_get_latest_placed_club(comp_record_of_key(comp), place), true, false);
    }
    let mut list: Vec<usize> = Vec::new();
    let mut c = read_usize(G_CLUB_TABLE);
    for _ in 0..(read_usize(G_CLUB_COUNT) as i32).max(0) {
        if exe::database_is_national_team(c) != 0 {
            break;
        }
        if read_i16(c + club::REPUTATION) > min_reputation && intercontinental_club_in_continent(c, continent) {
            list.push(c);
        }
        c += club::SIZE;
    }
    if !list.is_empty() {
        exe::comp_util_sort_and_shuffle_top_clubs(list.as_mut_ptr() as usize, list.len() as u32, 1);
        return (list[0], false, false);
    }
    show_error_at(file, line);
    (exe::comp_util_get_latest_placed_club(comp_record_of_key(comp), 0), false, true)
}

/// The start of both cups: `cup::TEAM_COUNT` = 2 and the team table `cup::TEAM_TABLE` of 2 six-byte teams
/// (`exe::crt_malloc`(0xc), cdecl; out of memory, the error box of line 0xc6 and −1).
unsafe fn intercontinental_cup_team_table(this: usize, file: usize) -> Option<usize> {
    write_i16(this + cup::TEAM_COUNT, 2);
    let t = exe::crt_malloc(0xc);
    write_usize(this + cup::TEAM_TABLE, t);
    if t == 0 {
        show_error_at(file, 0xc6);
        return None;
    }
    Some(t)
}

/// The end of both cups: bytes +4, +5, +0xa and +0xb of the team table zeroed; returns 1.
unsafe fn intercontinental_cup_finish(this: usize) -> i32 {
    for k in [4usize, 5, 0xa, 0xb] {
        write_u8(read_usize(this + cup::TEAM_TABLE) + k, 0);
    }
    1
}

/// The two Intercontinental Cup clubs. Place 0 is Europe's: the Champions League winner
/// ([`KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE`]), or a European club with reputation above 5,000 drawn when Europe
/// has no loaded league outside 2001 (the error box of line 0xea with none; a null winner: the error box of line 0xd1
/// and 0). Place 1 is South America's: the Libertadores winner ([`KEY_CLUB_COMP_SOUTH_AMERICAN_COPA_LIBERTADORES`]) or
/// the draw in the same way (boxes at lines 0x12a and 0xff); when it is the winner and the winner is South American,
/// the Libertadores runner-up takes its place if it is South American too. Returns 1 (−1 out of memory). (Stack frame
/// 0x11a4.)
/// Original: `WorldClubCup::pick_qualifiers` (0x92b8b0, world_club_cup.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn world_club_cup_pick_qualifiers(this: usize) -> i32 {
    let Some(t) = intercontinental_cup_team_table(this, 0xad5848) else {
        return -1;
    };
    let (europe, by_winner, fallback) = intercontinental_cup_slot(this, KEY_CONTINENT_EUROPE, 0x1388, KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE, 0, 0xad5848, 0xea); // cqu13: the European champion
    write_usize(t + if fallback { 6 } else { 0 }, europe);
    if by_winner {
        covered!("cqu_13 european club");
        if europe == 0 {
            show_error_at(0xad5848, 0xd1);
            return 0;
        }
    }
    let (south, by_winner, _) = intercontinental_cup_slot(this, KEY_CONTINENT_SOUTH_AMERICA, 0x1388, KEY_CLUB_COMP_SOUTH_AMERICAN_COPA_LIBERTADORES, 0, 0xad5848, 0x12a);
    write_usize(read_usize(this + cup::TEAM_TABLE) + 6, south);
    if by_winner {
        if south == 0 {
            show_error_at(0xad5848, 0xff);
            return 0;
        }
        if intercontinental_club_in_continent(south, KEY_CONTINENT_SOUTH_AMERICA) {
            let runner_up = exe::comp_util_get_latest_placed_club(comp_record_of_key(KEY_CLUB_COMP_SOUTH_AMERICAN_COPA_LIBERTADORES), 1);
            if intercontinental_club_in_continent(runner_up, KEY_CONTINENT_SOUTH_AMERICA) {
                write_usize(read_usize(this + cup::TEAM_TABLE) + 6, runner_up);
            }
        }
    }
    intercontinental_cup_finish(this)
}

// ponytail: the draw (in Europe or South America without a league) and the allocated table cannot run twice. Only the
// original; the plan: returns 1, `cup::TEAM_COUNT` = 2, neither club null, the first from one continent and the second
// from the other, the status bytes zeroed.
unsafe fn intercontinental_cup_check_clubs(name: &str, this: usize, continents: (usize, usize), o: impl FnOnce() -> i32) -> i32 {
    let r = o();
    if r == 1 {
        let t = read_usize(this + cup::TEAM_TABLE);
        let (a, b) = (read_usize(t), read_usize(t + 6));
        let bytes = [4usize, 5, 0xa, 0xb].iter().all(|&k| read_u8(t + k) == 0);
        let ok = read_i16(this + cup::TEAM_COUNT) == 2 && a != 0 && b != 0 && intercontinental_club_in_continent(a, continents.0) && intercontinental_club_in_continent(b, continents.1) && bytes;
        SCOREBOARD.record(ok, || format!("{}({:#x}): clubs {:#x} {:#x} status zeroed {}", name, this, a, b, bytes));
    }
    r
}

unsafe extern "thiscall" fn compare_world_club_cup_pick_qualifiers(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I0%%);
    intercontinental_cup_check_clubs("world_club_cup_pick_qualifiers", this, (KEY_CONTINENT_EUROPE, KEY_CONTINENT_SOUTH_AMERICA), || o(this))
}

/// The two Interamerican Cup clubs. Place 0 is North America's: the CONCACAF Champions Cup winner
/// ([`KEY_CLUB_COMP_CONCACAF_CHAMPIONS_CUP`]), or a North American club with reputation above 1,000 drawn when it has
/// no loaded league outside 2001 (boxes at lines 0xea and 0xd1); place 1 is South America's: the Libertadores winner
/// ([`KEY_CLUB_COMP_SOUTH_AMERICAN_COPA_LIBERTADORES`]) or the draw above 5,000 (boxes at lines 0x118 and 0xff).
/// Returns 1 (0 with a null winner; −1 out of memory). (Stack frame 0x11a4.)
/// Original: `InterAmerCup::select_teams` (0x632480, inter_amer_cup.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn inter_amer_cup_select_teams(this: usize) -> i32 {
    let Some(t) = intercontinental_cup_team_table(this, 0x9e9d94) else {
        return -1;
    };
    let (north, by_winner, fallback) = intercontinental_cup_slot(this, KEY_CONTINENT_NORTH_AMERICA, 0x3e8, KEY_CLUB_COMP_CONCACAF_CHAMPIONS_CUP, 0, 0x9e9d94, 0xea); // cqu13: the CONCACAF champion
    write_usize(t + if fallback { 6 } else { 0 }, north);
    if by_winner {
        covered!("cqu_13 north american club");
        if north == 0 {
            show_error_at(0x9e9d94, 0xd1);
            return 0;
        }
    }
    let (south, by_winner, _) = intercontinental_cup_slot(this, KEY_CONTINENT_SOUTH_AMERICA, 0x1388, KEY_CLUB_COMP_SOUTH_AMERICAN_COPA_LIBERTADORES, 0, 0x9e9d94, 0x118); // cqu13: the Libertadores champion in the Interamerican Cup
    write_usize(read_usize(this + cup::TEAM_TABLE) + 6, south);
    if by_winner {
        covered!("cqu_13 interamerican south american club");
        if south == 0 {
            show_error_at(0x9e9d94, 0xff);
            return 0;
        }
    }
    intercontinental_cup_finish(this)
}

unsafe extern "thiscall" fn compare_inter_amer_cup_select_teams(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I1%%);
    intercontinental_cup_check_clubs("inter_amer_cup_select_teams", this, (KEY_CONTINENT_NORTH_AMERICA, KEY_CONTINENT_SOUTH_AMERICA), || o(this))
}

/// The calls of the CONMEBOL cups' predicted positions (0x4c2a90, 0x4c4210 and 0x4c5a90, the same body), in order: the
/// invited list (+0xee, +0xf2 clubs) already sorted; the rounds of the table `table` (`cup_round::SIZE`-byte records;
/// the word `cup_round::TIES` of each, those leaving in it), from the last to the first: the last gives position 1 to
/// those leaving in it and, like all of them, the running position (which goes up with each club) to as many again;
/// those remaining go in blocks of `group` with the position of the block's start. Returns (club, position).
unsafe fn con_merc_cup_predicted_plan(this: usize, table: usize, rounds: i16, group: u32) -> Vec<(usize, u8)> {
    let mut v = Vec::new();
    let list = read_usize(this + 0xee);
    let mut i: i16 = 0;
    let mut bl: u8 = 1;
    let mut current: u8 = 1;
    let mut r = rounds as i32 - 1;
    while r >= 0 {
        let p = table + r as usize * cup_round::SIZE + cup_round::TIES;
        if r == rounds as i32 - 1 {
            let mut j: i16 = 0;
            while j < read_i16(p) {
                v.push((read_usize(list + i as isize as usize * 4), current));
                i = i.wrapping_add(1);
                bl = bl.wrapping_add(1);
                j += 1;
            }
        }
        current = bl;
        let mut j: i16 = 0;
        while j < read_i16(p) {
            v.push((read_usize(list + i as isize as usize * 4), current));
            i = i.wrapping_add(1);
            bl = bl.wrapping_add(1);
            j += 1;
        }
        r -= 1;
    }
    let mut pos = bl;
    while (i as i32) < read_usize(this + 0xf2) as i32 {
        v.push((read_usize(list + i as isize as usize * 4), pos));
        bl = bl.wrapping_add(1);
        if bl as u32 == pos as u32 + group {
            pos = bl;
        }
        i = i.wrapping_add(1);
    }
    v
}

/// The predicted positions body: without statistics (+8), the error box `line`; the list +0xee of +0xf2 invited clubs
/// sorted by reputation (`exe::crt_qsort`(list, n, 4, 0x4b07d0), cdecl); the round table of stage `stage` through
/// `comp_vtable::CREATE_SCHEDULE` (push 0; push &extra; push &rounds; push stage; mov ecx, this -> thiscall, ret
/// 0x10); each club of the plan with its position (`exe::comp_stats_set_club_final_position`(stats, club, position),
/// thiscall, ret 8; the original pushes the byte with garbage in the high bytes, the routine reads the byte); the table
/// freed (`exe::crt_free`).
unsafe fn con_merc_cup_predicted_positions(this: usize, stage: u32, group: u32, file: usize, line: u32) {
    if read_usize(this + 8) == 0 {
        return show_error_at(file, line);
    }
    exe::crt_qsort(read_usize(this + 0xee), read_usize(this + 0xf2) as u32, 4, 0x4b07d0usize);
    let create_schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut extra) = (0i16, 0i16);
    let table = create_schedule(this, stage, &mut rounds, &mut extra, 0);
    for (c, p) in con_merc_cup_predicted_plan(this, table, rounds, group) {
        exe::comp_stats_set_club_final_position(read_usize(this + 8), c, p as u32);
    }
    exe::crt_free(table);
}

/// The Merconorte predicted positions, by the rounds of stage 3 (the knockout) and in blocks of 4 for those staying in
/// the groups (the error box at line 0x32e).
/// Original: `ConMercCup::set_predicted_positions` (0x4c2a90, con_merc_cup.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn con_merc_cup_set_predicted_positions(this: usize) {
    con_merc_cup_predicted_positions(this, 3, 4, 0x9a7d6c, 0x32e)
}

// ponytail: Comp_Stats::set_club_final_position adds the club's entry (it reallocates): it cannot run twice. Only the
// original; the plan, with the list already sorted by it: each invited club with an entry and the positions
// (`comp_stats_club::FINAL_POSITION`, `exe::comp_stats_find_club_entry`) rising in list order, the first 1. (Asking
// for the stage's round table again moves the cup's calendar: the 07/11 comparison showed the Cup.cpp line 1278 box;
// the plan calls nothing but the lookup.)
unsafe fn con_merc_cup_check_predicted(name: &str, this: usize, _stage: u32, _group: u32, o: impl FnOnce()) {
    o();
    let stats = read_usize(this + 8);
    let list = read_usize(this + 0xee);
    let n = read_usize(this + 0xf2) as i32;
    if stats == 0 || list == 0 || !(1..=256).contains(&n) {
        return;
    }
    let mut read: Vec<i32> = Vec::new();
    for k in 0..n as usize {
        let c = read_usize(list + k * 4);
        let e = if c != 0 { exe::comp_stats_find_club_entry(stats, c) } else { 0 };
        read.push(if e != 0 { read_u8(e + comp_stats_club::FINAL_POSITION) as i32 } else { -1 });
    }
    let ok = read[0] == 1 && read.iter().all(|&x| x >= 1) && read.windows(2).all(|w| w[0] <= w[1]);
    SCOREBOARD.record(ok, || format!("{}({:#x}): positions {:?}", name, this, read));
}

unsafe extern "thiscall" fn compare_con_merc_cup_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    con_merc_cup_check_predicted("con_merc_cup_set_predicted_positions", this, 3, 4, || o(this))
}
