use super::*;

/// Tells whether club `clube` takes part in competition `this` (returns 1/0). National teams from another continent
/// than the competition's (+4 -> +0x59) and clubs of nations from another continent are rejected, a club of the host
/// nation whose division (+0x57) is the competition itself is accepted; otherwise it searches the competition and its
/// stages (+0xc, count +0x30): the participant list of the latest group stage (type 2 at +0x43, table at +0xb1,
/// 0x3b-byte rows) or of the current stage (+0xa7, 6-byte entries), with the entry count at +0x3e.
pub(crate) unsafe extern "thiscall" fn club_in_competition(this: usize, club: usize) -> u8 {
    let this = League(this);
    let base = this.competition().addr();
    let d = |p: usize, o: usize| read_usize(p + o);
    let limit = read_i32(G_CLUB_COUNT).wrapping_sub(read_i32(G_NATION_COUNT));
    let nation = d(club, 0x53);
    if read_i32(club) >= limit {
        let continent = d(base, 0x59);
        if continent != 0 && (nation == 0 || d(nation, 0x71) != continent) {
            return 0;
        }
    } else if nation != 0 {
        if nation == d(base, 0x5d) && d(club, 0x57) != 0 && d(club, 0x57) == base {
            return 1;
        }
        if d(nation, 0x71) != d(base, 0x59) {
            return 0;
        }
    }
    let n = this.current_stage() as i32;
    let (mut grupo, mut outra) = (0usize, 0usize);
    let mut i = -1i32;
    while i <= n {
        let stage = League(if i == -1 { this.addr() } else { d(d(this.addr(), league::STAGES), 4 * i as usize) });
        if (stage.comp_type() as i8) == 2 {
            grupo = stage.addr();
        } else {
            outra = stage.addr();
        }
        let (f, tabela, step_towards) = if grupo != 0 { (grupo, 0xb1, 0x3b) } else { (outra, 0xa7, 6) };
        let how_many = read_i16(f + league::CLUB_COUNT);
        let list = d(f, tabela);
        let mut k = 0i16;
        while k < how_many {
            if d(list, step_towards * k as usize) == club {
                return 1;
            }
            k += 1;
        }
        i += 1;
    }
    0
}

/// Returns 1 if the club's id matches one of the nine fixed international-tournament keys (World Cup, Olympic
/// tournament, European Championship, Copa America, African Cup of Nations, Asian Cup, Oceania Nations Cup, Asian Games
/// tournament, Confederations Cup), else 0; a null club gives 0.
pub(crate) unsafe extern "stdcall" fn special_club(club: usize) -> u32 {
    if club == 0 {
        return 0;
    }
    let id = read_u32(club);
    [KEY_CLUB_COMP_FIFA_WORLD_CUP, KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_OLYMPIC_GAMES, KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP, KEY_CLUB_COMP_COPA_AMERICA, KEY_CLUB_COMP_AFRICAN_CUP_OF_NATIONS, KEY_CLUB_COMP_ASIAN_CUP, KEY_CLUB_COMP_OCEANIA_NATIONS_CUP, KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_ASIAN_GAMES, KEY_CLUB_COMP_CONFEDERATIONS_CUP]
        .iter()
        .any(|&a| read_u32(a) == id) as u32
}

/// Calls the league's vtable slot 0xa4 with no arguments [inference: finalizes the season standings and marks
/// promoted/relegated clubs, +0x37].
pub(crate) unsafe fn finalize_league_table(league: League) {
    let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(league.addr()) + league_vtable::RECORD_FINAL_POSITIONS));
    f(league.addr())
}

// simplification: closes seasons and moves clubs between leagues (contracts, managers, news): does not run twice; only the
// original. The copy takes the branch.

/// Sends the prize money news to the club's human manager, only when the prize is at least 50 × the club reputation:
/// the competition, the amount, the position, the round, the kind and, with the fixture, the opponent and whether the
/// club won.
///
/// `Comp::send_prize_money_news(club, amount (i64), position, round, fixture, kind)`, ret 0x1c; SEH frame only for the
/// local news item. To the club's human manager (`club_has_human_manager`), with an amount of at least 50 × the
/// reputation (`.reputation()`), news 0x1a (`exe::new_newsitem`): 0 the competition (`[this+4]`), 1 the amount, 2 the
/// club, 3 the position, 4 the round, 7 the kind; with the fixture, 5 the opponent and 6 whether the club won
/// (`.home_result_mark()`/`.away_result_mark()` the penalty shoot-out winner [inference], otherwise
/// `.home_aggregate()`/`.away_aggregate()`, otherwise `.home_goals()`/`.away_goals()` the score); without it, −1 and 0.
/// Delivered to him (`exe::news_send_to_club_manager`).
/// Original: `Comp::send_prize_money_news` (0x48e3e0, comp.cpp, thiscall, ret 0x1c).
pub(crate) unsafe extern "thiscall" fn send_prize_money_news(this: usize, club: usize, lo: u32, hi: i32, position: u32, round: u32, fixture: usize, kind: u32) {
    let fixture = Fixture(fixture);
    let club = Club(club);
    if !club_has_human_manager(club.addr()) {
        return;
    }
    let minimum = club.reputation() as i64 * 50;
    let value = ((hi as i64) << 32) | lo as i64;
    if value < minimum {
        return;
    }
    covered!("cqp_1 prize news");
    let mut n = [0u8; 0xe8];
    let p = n.as_mut_ptr() as usize;
    exe::new_newsitem(p, 0x1a, 0);
    exe::newsitem_set_slot(p, (0) as u32 as i8, read_usize(read_usize(this + 4)) as u32);
    exe::newsitem_set_slot(p, (1) as u32 as i8, lo);
    exe::newsitem_set_slot(p, (2) as u32 as i8, read_usize(club.addr()) as u32);
    exe::newsitem_set_slot(p, (3) as u32 as i8, position as u8 as i8 as i32 as u32);
    exe::newsitem_set_slot(p, (4) as u32 as i8, round as u16 as i16 as i32 as u32);
    exe::newsitem_set_slot(p, (7) as u32 as i8, kind as u8 as i8 as i32 as u32);
    let _b = |a: usize| read_i8(a);
    let won = if !fixture.is_null() {
        if fixture.home_club() == club {
            exe::newsitem_set_slot(p, (5) as u32 as i8, read_usize(fixture.away_club().addr()) as u32);
            fixture.home_result_mark() != result_mark::NONE || fixture.home_aggregate() > fixture.away_aggregate() || fixture.home_goals() > fixture.away_goals()
        } else {
            exe::newsitem_set_slot(p, (5) as u32 as i8, read_usize(fixture.home_club().addr()) as u32);
            fixture.away_result_mark() != result_mark::NONE || fixture.home_aggregate() < fixture.away_aggregate() || fixture.home_goals() < fixture.away_goals()
        }
    } else {
        exe::newsitem_set_slot(p, (5) as u32 as i8, u32::MAX);
        false
    };
    exe::newsitem_set_slot(p, (6) as u32 as i8, won as u32);
    exe::news_send_to_club_manager(read_usize(G_NEWS), p, club);
}

// simplification: delivers the news (allocates); original only. The copy takes the branch.

/// Sends the TV rights news to the club's human manager: the club, the competition, the amount and the year.
///
/// `Comp::send_tv_rights_news(club, value)`, ret 8; SEH frame only for the local news item. To the club's human manager
/// (`club_has_human_manager`), news 0x1f (`exe::new_newsitem`): 0 the club, 1 the competition (`.competition()`), 2 the
/// amount, 3 the competition year (`.year()`), delivered to him (`exe::news_send_to_club_manager`).
/// Original: `Comp::send_tv_rights_news` (0x48ea40, comp.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn send_tv_rights_news(this: usize, club: usize, value: u32) {
    let this = League(this);
    if !club_has_human_manager(club) {
        return;
    }
    covered!("cqp_1 tv news");
    let mut n = [0u8; 0xe8];
    let p = n.as_mut_ptr() as usize;
    exe::new_newsitem(p, 0x1f, 0);
    exe::newsitem_set_slot(p, (0) as u32 as i8, read_usize(club) as u32);
    exe::newsitem_set_slot(p, (1) as u32 as i8, read_usize(this.competition().addr()) as u32);
    exe::newsitem_set_slot(p, (2) as u32 as i8, value);
    exe::newsitem_set_slot(p, (3) as u32 as i8, this.year() as i32 as u32);
    exe::news_send_to_club_manager(read_usize(G_NEWS), p, club);
}

// simplification: delivers the news (allocates); original only. The copy takes the branch.

/// In the top competition, tells whether it and every competition down the chain are complete.
///
/// Ret 0, method +0xc of the competitions: `Comp::all_stages_complete()`: only in the top competition
/// (`.upper_division_id()` ≤ −1), it and each one in the chain below (`league::LOWER_DIVISION_ID`, the index into
/// `[G_COMP_OBJECT_TABLE]`) must be complete (`comp_vtable::CHECK_COMPLETE` with 1, 1: push 1; push 1; mov ecx ->
/// thiscall, ret 8); returns 1 or 0 (in al).
/// Original: `Comp::all_stages_complete` (0x48ce10, comp.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn all_stages_complete(this: usize) -> u8 {
    let this = League(this);
    if ((this.upper_division_id() as usize) as i32) > -1 {
        return 0;
    }
    let completa = |c: usize| -> u8 {
        let f: unsafe extern "thiscall" fn(usize, u32, u32) -> u8 = core::mem::transmute(read_usize(read_usize(c) + comp_vtable::CHECK_COMPLETE));
        f(c, 1, 1)
    };
    if completa(this.addr()) == 0 {
        return 0;
    }
    let mut i = this.lower_division_id() as i32;
    while i >= 0 {
        let c = read_usize(read_usize(G_COMP_OBJECT_TABLE) + i as usize * 4);
        if completa(c) == 0 {
            return 0;
        }
        covered!("cqh_6 lower league complete");
        i = read_usize(c + league::LOWER_DIVISION_ID) as i32; // cqh6: the lower league
    }
    1
}

/// Builds the vector of the distinct clubs of all stages (up to 200) and returns the count.
///
/// Ret 8, method +0x58 of the competitions: `Comp::collect_club_list(&list, _)`: a vector of 200 pointers
/// (`exe::crt_malloc`(0x320); out of memory: the box at line 0x681, a null list and 0) with the distinct clubs of all
/// stages (−1 the competition itself up to `.current_stage()`; the others in `.stages()`): in a league
/// (`league::COMP_TYPE` = 2) those of table `league::TABLE` (rows of `league_table_row::SIZE` bytes), otherwise those
/// of `cup::TEAM_TABLE` (6 bytes), skipping null and repeated ones, up to 200; stores the vector in `list` and returns
/// the count (word, in ax).
/// Original: `Comp::collect_club_list` (0x48e1c0, comp.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn comp_collect_club_list(this: usize, list: *mut usize, _x: u32) -> u16 {
    let this = League(this);
    let v = exe::crt_malloc(0x320);
    if v == 0 {
        show_error_at(0x99b80c, 0x681);
        *list = 0;
        return 0;
    }
    let mut n = 0i16;
    let mut s = -1i8;
    while s as i32 <= this.current_stage() as i32 {
        let f = League(if s == -1 { this.addr() } else { read_usize(this.stages() + (s as i32 * 4) as usize) });
        let league = f.comp_type() == 2;
        let mut i = 0i16;
        while i < f.club_count() {
            let club = if league { read_usize(f.table() + i as usize * league_table_row::SIZE) } else { read_usize(read_usize(f.addr() + cup::TEAM_TABLE) + i as usize * 6) };
            if club != 0 && !(0..n).any(|k| read_usize(v + k as usize * 4) == club) && n < 0xc8 { // cqh6: up to 200 clubs
                covered!("cqh_6 club in the list");
                write_u32(v + n as usize * 4, club as u32); // cqh6: the club in the list
                n += 1;
            }
            i += 1;
        }
        s = s.wrapping_add(1);
    }
    *list = v;
    n as u16
}

/// Finds the stage id, the status and the round the club reached, searching from the last stage back.
///
/// Ret 0x10: `Comp::find_club_progress(club, &stage_id, &round, &status)`: over the stages from the last
/// (`.current_stage()`) to the first (−1 the competition itself; the others in `.stages()`), skipping those with id
/// (`league::STAGE_ID`) 0x3fc: in a league (`league::COMP_TYPE` = 2), the club's table row (`exe::league_get_table_row`
/// push i; mov ecx, stage -> thiscall, ret 4): stage_id = the id (`league::STAGE_ID`) and status =
/// `league_table_row::STATUS` of the row, and returns; in a cup, the last team of table `cup::TEAM_TABLE` (6 bytes)
/// that is the club gives stage_id = the id (`cup::STAGE_LABEL`) and status = +5, and the round is the first round
/// (from `cup::CURRENT_ROUND` − 1 backwards, `cup::ROUND_TABLE` with `cup_round::SIZE` (0x68) bytes, the ties of
/// `cup::TIE_TABLES` with 0x35 bytes) with a tie in which the team plays (+0 or +2): round = +7 of the round, and
/// returns.
/// Original: `Comp::find_club_progress` (0x48cc70, comp.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn find_club_progress(this: usize, club: usize, stage_id: *mut i16, round: *mut i16, status: *mut u8) {
    let this = League(this);
    let mut f = this.current_stage() as i32;
    while f >= -1 {
        let s = League(if f > -1 { read_usize(this.stages() + f as usize * 4) } else { this.addr() });
        if s.stage_id() != 0x3fc {
            if s.comp_type() == 2 {
                let mut i = 0i8;
                while (i as i16) < s.club_count() {
                    if read_usize(exe::league_get_table_row(s, i as i32)) == club {
                        *stage_id = s.stage_id();
                        *status = read_u8(exe::league_get_table_row(s, i as i32) + league_table_row::STATUS);
                        return;
                    }
                    i = i.wrapping_add(1);
                }
            } else {
                let mut team: i16 = -1;
                let mut k = 0i16;
                while k < read_i16(s.addr() + cup::TEAM_COUNT) {
                    let t = read_usize(s.addr() + cup::TEAM_TABLE) + k as usize * 6;
                    if read_usize(t) == club {
                        *stage_id = read_i16(s.addr() + cup::STAGE_LABEL);
                        covered!("cqh_6 cup progress");
                        *status = read_u8(t + 5); // cqh6: the team’s situation
                        team = k;
                    }
                    k += 1;
                }
                if team != -1 {
                    let mut r = (read_u8(s.addr() + cup::CURRENT_ROUND) as i8 as i16).wrapping_sub(1);
                    while r >= 0 {
                        let n = read_i16(read_usize(s.addr() + cup::ROUND_TABLE) + r as usize * cup_round::SIZE + cup_round::TIES);
                        let confrontos = read_usize(read_usize(s.addr() + cup::TIE_TABLES) + r as usize * 4);
                        let mut j = 0i16;
                        while j < n {
                            let c = confrontos.wrapping_add((j as i32 * 0x35) as usize);
                            if read_i16(c) == team || read_i16(c + 2) == team {
                                *round = read_i16(read_usize(s.addr() + cup::ROUND_TABLE) + r as usize * 0x68 + 7);
                                return;
                            }
                            j += 1;
                        }
                        r -= 1;
                    }
                }
            }
        }
        f -= 1;
    }
}

/// Address of tie p (0x35 bytes) of cup round r (per-round arrays at [+0xad]).
pub(crate) unsafe fn cup_tie(cup: Cup, r: i32, p: i32) -> usize {
    read_usize(cup.tie_tables() + (r as isize * 4) as usize).wrapping_add((p as isize * 0x35) as usize)
}

/// Address of cup round r (0x68 bytes in table [+0xa3]).
pub(crate) unsafe fn comp_cup_round(cup: Cup, r: i32) -> usize {
    cup.round_table().wrapping_add((r as isize * 0x68) as usize)
}

/// Stage k of the competition as the original reads it: -1 itself, 0..+0x30 from [+0xc]; out of range shows the
/// comp.cpp box and returns 0.
pub(crate) unsafe fn comp_stage(comp: League, k: i32) -> usize {
    if k < -1 || k > comp.current_stage() as i32 {
        show_error_at(0x99b80c, 0x36e);
        return 0;
    }
    if k == -1 { comp.addr() } else { read_usize(comp.stages() + k as usize * 4) }
}

/// From the given stage backwards, returns the index of the first stage of the same group.
///
/// Ret 4: `Comp::get_first_stage_of_group(k)`, used by the competition screens (0x4947a0): from stage `k` backwards,
/// while the stages have the same group (`exe::comp_util_get_parent_stage_id` of the id `league::STAGE_ID`: push id;
/// add esp, 4 -> cdecl) and the index is greater than −1, the index of the first of them (k itself when it has no
/// group).
/// Original: `Comp::get_first_stage_of_group` (0x48d2b0, comp.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn get_first_stage_of_group(this: usize, k: i32) -> i32 {
    let mut e = k;
    let g = exe::comp_util_get_parent_stage_id(read_i16(comp_stage(League(this), e) + league::STAGE_ID) as i32);
    if g > -1 && e > -1 {
        loop {
            let f = League(comp_stage(League(this), e - 1));
            if exe::comp_util_get_parent_stage_id(f.stage_id() as i32) != g {
                break;
            }
            e -= 1;
            if e <= -1 {
                break;
            }
        }
    }
    e
}

/// Returns the first stage not finished (for a cup not yet started, the start of its group); with all finished, the
/// start of the last one's group.
///
/// Ret 0: `Comp::get_current_stage()`, used by the competition screens (0x48f8e0): the first stage (−1 the competition
/// itself, up to `.current_stage()`) that has not finished (`comp_vtable::CHECK_COMPLETE` with (0, 1) = 0); if it is a
/// cup (`league::COMP_TYPE` = 1) that has not started (`cup::CURRENT_ROUND` = 0), goes back to the start of its group
/// through the previous stage (`exe::comp_util_get_parent_stage_id` and `exe::comp_get_stage`: push k; mov ecx, this ->
/// thiscall, ret 4). All finished: the start of the last one's group. Invalid stage at the end: the box at line 0x36e
/// and the competition itself; null stage: the competition itself.
/// Original: `Comp::get_current_stage` (0x48cec0, comp.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn get_current_stage(this: usize) -> usize {
    let this = League(this);
    let go_back = |s0: i32| -> i32 {
        let mut s = s0;
        let g = exe::comp_util_get_parent_stage_id(read_i16(comp_stage(this, s) + league::STAGE_ID) as i32);
        if g > -1 && s > -1 {
            loop {
                let f = exe::comp_get_stage(this, s - 1);
                if exe::comp_util_get_parent_stage_id(read_i16(f + league::STAGE_ID) as i32) != g {
                    break;
                }
                s -= 1;
                if s <= -1 {
                    break;
                }
            }
        }
        s
    };
    let end = |s: i32| -> usize {
        if s < -1 || s > this.current_stage() as i32 {
            show_error_at(0x99b80c, 0x36e);
            return this.addr();
        }
        let f = if s == -1 { this.addr() } else { read_usize(this.stages() + s as usize * 4) };
        if f != 0 { f } else { this.addr() }
    };
    let mut e: i32 = -1;
    if this.current_stage() as i32 >= -1 {
        loop {
            let f = comp_stage(this, e);
            let acabou: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(read_usize(read_usize(f) + comp_vtable::CHECK_COMPLETE));
            if acabou(f, 0, 1) as u8 == 0 {
                if f == 0 {
                    break;
                }
                if read_u8(f + league::COMP_TYPE) == 1 && read_u8(f + cup::CURRENT_ROUND) == 0 && e > -1 {
                    return end(go_back(e - 1));
                }
                return f;
            }
            e += 1;
            if e > this.current_stage() as i32 {
                break;
            }
        }
    }
    end(go_back(this.current_stage() as i32))
}

/// Address of entry k (72 bytes) of the 35 key nations table at 0xb63d60 (+0 nation, +0x14 active index).
pub(crate) fn key_nation_entry(k: i8) -> usize {
    G_LEAGUE_TABLE + k as i32 as usize * 72
}

/// Index k of the nation in the key nations table (active nation entry), or None.
pub(crate) unsafe fn find_key_nation_index(nation: usize) -> Option<i8> {
    let mut k: i8 = 0;
    while k < 0x23 {
        let e = Nation(read_usize(key_nation_entry(k)));
        if !e.is_null() && e.league_selected() != 0 && e.addr() == nation {
            return Some(k);
        }
        k += 1;
    }
    None
}

/// the active index of the club's nation among the active key nations; otherwise 0 for a national team and 3 for a club
/// (the type used by the fixture search and the date clash check)
/// Original: `comp_active_key_nation_of_club` (0x48dee0, comp.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_active_key_nation_of_club(club: usize) -> u8 {
    let club = Club(club);
    let nation = club.nation().addr();
    if nation != 0 {
        if let Some(k) = find_key_nation_index(nation) {
            covered!("cqb_12 club key nation");
            return read_u8(key_nation_entry(k) + key_nation_record::ACTIVE_INDEX); // cqb12: the active index of the club’s nation
        }
    }
    // 0x53b900: push club; add esp, 4 -> cdecl
    if exe::database_is_national_team(club) != 0 {
        return 0;
    }
    covered!("cqb_12 club without key nation");
    3 // cqb12: club outside the key nations
}

/// the active index of the stadium city's nation among the active key nations, or 3
/// Original: `comp_active_key_nation_of_stadium` (0x48df40, comp.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_active_key_nation_of_stadium(stadium: usize) -> u8 {
    let stadium = Stadium(stadium);
    let u = |a: usize| read_usize(a);
    if stadium.is_null() || stadium.city() == 0 || u(stadium.city() + city::NATION) == 0 {
        return 3;
    }
    match find_key_nation_index(u(stadium.city() + city::NATION)) {
        Some(k) => {
            covered!("cqb_12 stadium key nation");
            read_u8(key_nation_entry(k) + key_nation_record::ACTIVE_INDEX) // cqb12: the active index of the stadium’s nation
        }
        None => 3,
    }
}

/// Index of the key nations entry whose active index (+0x14) is ativo, if any.
pub(crate) unsafe fn key_nation_by_active_index(active: i8) -> Option<i8> {
    (0..0x23i8).find(|&k| read_i8(key_nation_entry(k) + key_nation_record::ACTIVE_INDEX) == active)
}

/// the first key nation table entry with that active index; none, the error box and 3
/// Original: `comp_key_nation_entry_from_active_index` (0x48dcd0, comp.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_key_nation_entry_from_active_index(active: i8) -> i32 {
    match key_nation_by_active_index(active) {
        Some(k) => k as i32,
        None => {
            show_error_at(0x99b80c, 0x543); // "E:\dev\CM3\cm3 00-01\cm3\code\comp\comp.cpp"
            3
        }
    }
}

/// Club c's national call-up record in [this + 0x20] (6 bytes: entries, capacity, count), if the club counts (in range,
/// marked nation, not the excluded one).
pub(crate) unsafe fn club_callup_record(this: usize, c: Club) -> Option<usize> {
    let n = (read_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_usize(G_NATION_COUNT) as i32).wrapping_mul(2));
    let id = read_usize(c.addr()) as i32;
    if id >= n || id < 0 {
        return None;
    }
    let nation = c.nation();
    if nation.is_null() || nation.league_selected() == 0 || read_usize(nation.addr()) == read_usize(KEY_NATION_BRAZIL) {
        return None;
    }
    Some(read_usize(this + national_teams_fields::CLUB_CALL_UP_RECORDS).wrapping_add(6 * id as usize))
}

/// Adds `conta` call-ups for club `c` on date `d` (day at +0, year at +2).
///
/// If there is an entry with the same date, the count is added to it; otherwise a new entry is created (when the array
/// is full, 3 more slots are added via `exe::crt_realloc`) with the date and the count, and the array is re-sorted by date (qsort
/// `exe::crt_qsort` with `compare_call_up_records_by_date`). Out of memory: the error box (line 0x21c8) and false (the routine stops).
pub(crate) unsafe fn add_club_callups(this: usize, c: usize, count: u8, d: *mut u8) -> bool {
    let rec = match club_callup_record(this, Club(c)) {
        Some(r) => r,
        None => return true,
    };
    let how_many = *((rec + 5) as *const i8);
    let (dia, ano) = (core::ptr::read_unaligned(d as *const u16), core::ptr::read_unaligned((d as usize + 2) as *const u16));
    let mut k: i8 = 0;
    while k < how_many {
        let e = read_usize(rec).wrapping_add(9 * k as i32 as usize);
        if core::ptr::read_unaligned((e + 2) as *const u16) == ano && core::ptr::read_unaligned(e as *const u16) == dia {
            covered!("cpw_13 sum at the date");
            *((e + 8) as *mut u8) = (*((e + 8) as *const u8)).wrapping_add(count); // cpw13: adds to the date
            return true;
        }
        k += 1;
    }
    if how_many == *((rec + 4) as *const i8) {
        let cap = (*((rec + 4) as *const u8)).wrapping_add(3);
        *((rec + 4) as *mut u8) = cap;
        let v = exe::crt_realloc(read_usize(rec), (cap as i8 as i32 * 9) as u32);
        core::ptr::write_unaligned(rec as *mut usize, v);
        if v == 0 {
            show_error_at(0xa5b2a4, 0x21c8);
            return false;
        }
    }
    let q = *((rec + 5) as *const i8) as i32;
    let e = read_usize(rec).wrapping_add(9 * q as usize);
    let mut t = [0u8; 8];
    exe::date_operator_3d(e, (t.as_mut_ptr()) as usize, (d) as usize);
    covered!("cpw_13 new record");
    *((e + 8) as *mut u8) = count; // cpw13: the count of the new record
    let q = (*((rec + 5) as *const i8)).wrapping_add(1);
    *((rec + 5) as *mut i8) = q;
    exe::crt_qsort(read_usize(rec), q as i32 as u32, 9, 0x778e90);
    true
}

/// True when some club that `record_club_call_ups` may add to has its call-up array full (capacity = count), so it may reallocate.
pub(crate) unsafe fn callup_array_full(this: usize, team: Club) -> bool {
    (0..0x32usize).any(|i| {
        let s = Staff(team.squad(i));
        if s.is_null() || read_usize(s.addr()) as i32 >= read_usize(G_STAFF_COUNT) as i32 || s.club_job().is_null() {
            return false;
        }
        match club_callup_record(this, s.club_job()) {
            Some(rec) => read_i8(rec + 4) == read_i8(rec + 5),
            None => false,
        }
    })
}

/// forwards the match importance query to the fixture's stage (`fixture::STAGE_INDEX`; virtual
/// `comp_vtable::GET_MATCH_IMPORTANCE`)
/// Original: `Comp::get_fixture_match_importance` (0x48e190, comp.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn get_fixture_match_importance(this: usize, p: usize, a: u32, b: u32) -> u32 {
    let p = Fixture(p);
    let this = League(this);
    let stage = p.stage_index();
    let c = if stage == 0xff { this.addr() } else { read_usize(this.stages().wrapping_add(4 * (stage as i8 as i32) as usize)) };
    let f: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(read_usize(read_usize(c) + comp_vtable::GET_MATCH_IMPORTANCE));
    f(c, p.addr(), a, b)
}

/// returns stage k of the competition (-1 = itself); out of range, the error box and 0
/// Original: `Comp::get_stage` (0x48d200, comp.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn get_stage(this: usize, k: i32) -> usize {
    let this = League(this);
    if k < -1 || k > this.current_stage() as i32 {
        return original_fn::<unsafe extern "thiscall" fn(usize, i32) -> usize>(IDX_GET_STAGE)(this.addr(), k);
    }
    if k == -1 {
        return this.addr();
    }
    read_usize(this.stages().wrapping_add(4 * k as usize))
}

/// Returns the active key of the key nation that owns the fixture: by the competition's nation, by the nations of the
/// clubs in a friendly, 0 for the World Cup, 1 for national side tournaments.
///
/// `comp_active_key_nation_of_fixture(comp, fixture)`, the key of the FIXTURE_MANAGER match lists (add_fixture
/// `add_fixture`, find_fixtures `find_fixtures`, ...), of the quick simulation (`exe::match_day_can_quick_simulate`)
/// and of the FIFA ranking weight (`exe::fifa_rankings_record_match_result`). Friendly
/// (`exe::friendly_is_friendly_competition` over `[G_FRIENDLY]`) or reserves (`[KEY_CLUB_COMP_RESERVE]`) with the given
/// fixture and its competition being the friendly one (`[G_FRIENDLY]`+4) or the reserves one: with either side a
/// national side (`exe::database_is_national_team` of the clubs `.home_club()`/`.away_club()`), 1; different nations
/// (`.nation()`) or outside the selection (`.league_selected()`), 3; the nation in the table of the 35 key nations
/// (`G_LEAGUE_TABLE`, 72 bytes each, selected in `.league_selected()`), byte +0x14 of the row; otherwise 0 with the
/// home side a national side and 3 without. Otherwise, a nation's competition (`.nation()`): byte +0x14 of its row, or
/// 3. Without a nation: World Cup and qualifiers (`exe::key_nation_is_world_cup_comp`), 0; international tournament
/// (`exe::key_nation_is_international_comp`), 1; otherwise the key nation index (`exe::key_nation_index_of_comp`).
/// TapeCall on every fixture searched: no counter.
/// Original: `comp_active_key_nation_of_fixture` (0x48dd70, comp.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_active_key_nation_of_fixture(comp: usize, fixture: usize) -> u8 {
    let fixture = Fixture(fixture);
    let comp = Competition(comp);
    let row = |nation: usize| (0..0x23usize).find(|&k| {
        let n = Nation(read_usize(G_LEAGUE_TABLE + 72 * k));
        !n.is_null() && n.league_selected() != 0 && n.addr() == nation
    });
    let nation = comp.nation();
    let reservas = read_usize(KEY_CLUB_COMP_RESERVE);
    if (exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), comp) != 0 || read_usize(comp.addr()) == reservas) && !fixture.is_null() {
        let pc = fixture.competition();
        if pc == Competition(read_usize(read_usize(G_FRIENDLY) + 4)) || read_usize(pc.addr()) == reservas {
            if exe::database_is_national_team(fixture.home_club()) != 0 || exe::database_is_national_team(fixture.away_club()) != 0 {
                return 1;
            }
            let home = fixture.home_club();
            let a = home.nation();
            if a != fixture.away_club().nation() || a.league_selected() == 0 {
                return 3;
            }
            if let Some(k) = row(a.addr()) {
                return read_u8(0xb63d74 + 72 * k); // cpt3: the friendly’s nation
            }
            return if exe::database_is_national_team(home) != 0 { 0 } else { 3 };
        }
    }
    if !comp.nation().is_null() {
        return match row(nation.addr()) {
            Some(k) => read_u8(0xb63d74 + 72 * k), // cpt3: the competition’s key nation
            None => 3,
        };
    }
    if exe::key_nation_is_world_cup_comp(comp) != 0 {
        return 0; // cpt3: Copa do Mundo e eliminatorias
    }
    if exe::key_nation_is_international_comp(comp) != 0 {
        return 1; // cpt3: torneio internacional
    }
    exe::key_nation_index_of_comp(comp)
}

/// Tells whether a round's fixtures are known: the stage is a league, or the round is before the current one, or it is
/// the current last round with code 0x8c.
///
/// Ret 8: `Comp::are_round_fixtures_known(stage, round)`: the stage (−1 = the competition itself; above
/// `.current_stage()`: 0; below −1: the box at line 0x36e, as in the original) is a league (`.comp_type()` =
/// `comp_type::LEAGUE`): 1. Otherwise 1 if the round is before the current one (`.current_round()`, byte) or is the
/// current one, the last (`.round_count()` − 1) and with code 0x8c (+7 of its 0x68-byte record in `.round_table()`);
/// otherwise 0.
/// Original: `Comp::are_round_fixtures_known` (0x48d410, comp.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn are_round_fixtures_known(this: usize, stage: i32, round: i32) -> u32 {
    let this = League(this);
    if stage > this.current_stage() as i32 {
        return 0;
    }
    if stage < -1 {
        return original_fn::<unsafe extern "thiscall" fn(usize, i32, i32) -> u32>(IDX_ARE_ROUND_FIXTURES_KNOWN)(this.addr(), stage, round);
    }
    let c = Cup(if stage == -1 { this.addr() } else { read_usize(this.stages().wrapping_add(4 * stage as usize)) });
    if c.comp_type() == comp_type::LEAGUE {
        return 1;
    }
    let current = (c.current_round() as i8) as i32;
    if round < current {
        return 1;
    }
    if round != current {
        return 0;
    }
    let n = c.round_count() as i32;
    if current != n - 1 {
        return 0;
    }
    let code = read_i16(c.round_table().wrapping_add((0x68 * (n - 1)) as usize) + 7);
    (code == 0x8c) as u32
}

/// Returns 1 when the match round (`fixture::ROUND_NAME`) ends in 0 or is 1000 or above, else 0.
///
/// Ret 8: `Comp::counts_for_fixture_clash(game, other)` (virtual +0x60 of the competitions, used by the date clash
/// check `collect_day_clashes`): 1 if the units digit (`exe::comp_util_get_round_variant`) of the round
/// (`fixture::ROUND_NAME`, signed) is 0 (a round of 1000 or more also gives 0), otherwise 0. `other` is not read.
/// Original: `Comp::counts_for_fixture_clash` (0x48e360, comp.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn counts_for_fixture_clash(_this: usize, game: usize, _other: usize) -> u32 {
    let game = Fixture(game);
    (exe::comp_util_get_round_variant(game.round_name() as i32) == 0) as u32
}

