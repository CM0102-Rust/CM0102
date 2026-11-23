use super::*;

/// True if the club's division (+0x57) has the competition id in global g.
pub(crate) unsafe fn eng_auto_cup_is_in_comp(c: usize, g: usize) -> bool {
    let c = Club(c);
    let l = c.division().addr();
    l != 0 && read_usize(l) == read_usize(g)
}

/// Puts club c in the next slot of the list at +0xb2 and increments the count.
pub(crate) unsafe fn push_section_club(this: usize, k: &mut i32, c: usize) {
    *k += 1;
    write_usize(read_usize(this + eng_auto_cup::SELECTED_CLUBS) + (*k as usize - 1) * 4, c);
}

/// `EngAutoCup::select_section_clubs()` (`select_section_clubs`, thiscall, ret 0, eng_auto_cup.cpp; from the season rollover, n = 1
/// up to 2002-06-01): puts the 56 League Trophy clubs (+0xb6 = 0x38) in a list (`exe::crt_malloc` of 0xe0 at +0xb2; out of
/// memory, the box for line 0x24c and 0), in four steps, each one until the list is full (count == +0xb6).
///
/// 1) In club table order ([0xae23b4], [0xae235c] clubs), those of divisions [0x9cf5c4] and [0x9cf5c8] (the English
/// second and third divisions [inference]).
///
/// 2) The first club of division [0x9cf69c] (the Conference [inference]) that came from [0x9cf5c8] (+0x5b, the previous
/// season's division [inference]): the relegated club; it is remembered (`achado`) so it does not enter again.
///
/// 3) If the cup's nation ([+4] +0x5d) has bit 4 in +0x11c, the clubs of [0x9cf69c] that were already in it (+0x5b), by
/// category +0x5f (signed): first those of 6 or less, then 7, 8... until full; otherwise, up to 30 clubs of [0x9cf69c]
/// (except the remembered one) and, if short, the English clubs (+0x53 = [0x9cf2e4]) outside the generic division
/// [0x9cf764] (among the first [0xae235c] − 2 × [0xae2350]), up to 30, sorted by reputation with the first 3 × missing
/// shuffled (comp_util_sort_and_shuffle_top_clubs `comp_util_sort_and_shuffle_top_clubs`: push missing; push how many; push list; add esp, 0xc ->
/// cdecl; uses random draws), and of those the first ones still missing.
///
/// Once full, the list is sorted by the latitude of the stadium's city (qsort `exe::crt_qsort` with `database_compare_clubs_by_latitude`) and each half
/// (the North and the South [inference]) by reputation, lowest to highest (qsort with 0x4b0810); returns 1. If it does
/// not fill, 0.
pub(crate) unsafe fn eng_auto_cup_select_section_clubs(this: usize) -> u32 {
    write_usize(this + eng_auto_cup::SELECTED_COUNT, 0x38);
    let l = exe::crt_malloc(0xe0);
    write_usize(this + eng_auto_cup::SELECTED_CLUBS, l);
    if l == 0 {
        show_error_at(0x9c4180, 0x24c);
        return 0;
    }
    let base = read_usize(G_CLUB_TABLE);
    let n = read_usize(G_CLUB_COUNT) as i32;
    let cheia = |k: i32| k == read_usize(this + eng_auto_cup::SELECTED_COUNT) as i32;
    let mut k = 0i32;
    // 1) the second and third divisions
    let mut i: i16 = 0;
    if n > 0 {
        loop {
            let c = base + i as u16 as usize * club::SIZE;
            if eng_auto_cup_is_in_comp(c, KEY_CLUB_COMP_ENGLISH_SECOND_DIVISION) || eng_auto_cup_is_in_comp(c, KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION) {
                push_section_club(this, &mut k, c);
            }
            if cheia(k) {
                break;
            }
            i = i.wrapping_add(1);
            if i as i32 >= n {
                break;
            }
        }
    }
    // 2) the club relegated from the third division to the Conference
    // simplification: without a relegated club the original later compares with a stack dword without a value; the copy uses 0 (no club)
    let mut found = 0usize;
    let mut i: i16 = 0;
    if n > 0 {
        loop {
            let c = base + i as u16 as usize * club::SIZE;
            let before = read_usize(c + club::LAST_DIVISION);
            if eng_auto_cup_is_in_comp(c, KEY_CLUB_COMP_ENGLISH_CONFERENCE) && before != 0 && read_usize(before) == read_usize(KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION) {
                covered!("cqs_5 relegated to the conference");
                push_section_club(this, &mut k, c); // cqs5: o rebaixado entra
                found = c;
                break;
            }
            if cheia(k) {
                break;
            }
            i = i.wrapping_add(1);
            if i as i32 >= n {
                break;
            }
        }
    }
    let nacao = Nation(read_usize(read_usize(this + 4) + competition::NATION));
    if nacao.league_selected() & league_selected::LOWER_DIVISIONS_LOADED != 0 {
        // 3a) the Conference by category
        let mut cat: i8 = 6;
        if k < read_usize(this + eng_auto_cup::SELECTED_COUNT) as i32 {
            loop {
                let mut i: i16 = 0;
                if n > 0 {
                    loop {
                        if k >= read_usize(this + eng_auto_cup::SELECTED_COUNT) as i32 {
                            break;
                        }
                        let c = base + i as u16 as usize * club::SIZE;
                        let before = read_usize(c + club::LAST_DIVISION);
                        if before != 0 && read_usize(before) == read_usize(KEY_CLUB_COMP_ENGLISH_CONFERENCE) && eng_auto_cup_is_in_comp(c, KEY_CLUB_COMP_ENGLISH_CONFERENCE) {
                            let v = read_i8(c + club::LAST_POSITION);
                            if ((cat == 6 && v <= cat) || v == cat) && c != 0 && c != found {
                                push_section_club(this, &mut k, c);
                            }
                        }
                        i = i.wrapping_add(1);
                        if i as i32 >= n {
                            break;
                        }
                    }
                }
                cat = cat.wrapping_add(1);
                if k >= read_usize(this + eng_auto_cup::SELECTED_COUNT) as i32 {
                    break;
                }
            }
        }
    } else {
        // 3b) up to 30 from the Conference and, if short, English clubs outside the generic division; shuffled
        let mut local = [0usize; 0x1e];
        let mut m: i8 = 0;
        let mut i: i16 = 0;
        if n > 0 {
            loop {
                let c = base + i as u16 as usize * club::SIZE;
                if eng_auto_cup_is_in_comp(c, KEY_CLUB_COMP_ENGLISH_CONFERENCE) && found != c {
                    local[m as u8 as usize] = c;
                    m = m.wrapping_add(1);
                    if m == 0x1e {
                        break;
                    }
                }
                i = i.wrapping_add(1);
                if i as i32 >= n {
                    break;
                }
            }
        }
        if (m as i32) < read_usize(this + eng_auto_cup::SELECTED_COUNT) as i32 - k {
            let lim = n.wrapping_sub((read_usize(G_NATION_COUNT) as i32).wrapping_mul(2));
            let mut j: i16 = 0;
            if lim > 0 {
                loop {
                    let c = base + j as u16 as usize * club::SIZE;
                    let nation = read_usize(c + club::NATION);
                    if nation != 0 && read_usize(nation) == read_usize(KEY_NATION_ENGLAND) && !eng_auto_cup_is_in_comp(c, KEY_CLUB_COMP_A_LOWER_DIVISION) {
                        local[m as u8 as usize] = c;
                        m = m.wrapping_add(1);
                        if m == 0x1e {
                            break;
                        }
                    }
                    j = j.wrapping_add(1);
                    if j as i32 >= lim {
                        break;
                    }
                }
            }
        }
        // simplification: the original pushes the two words in dwords with garbage in the high 16 bits; 0x4afbd0 reads the words
        exe::comp_util_sort_and_shuffle_top_clubs(local.as_mut_ptr() as usize, m as i16 as u16 as u32, (read_i16(this + eng_auto_cup::SELECTED_COUNT).wrapping_sub(k as i16)) as u16 as u32);
        let mut a: i16 = 0;
        while k < read_usize(this + eng_auto_cup::SELECTED_COUNT) as i32 {
            push_section_club(this, &mut k, local[a as u16 as usize]);
            a = a.wrapping_add(1);
        }
    }
    if k != read_usize(this + eng_auto_cup::SELECTED_COUNT) as i32 {
        return 0;
    }
    covered!("cqs_5 league trophy sections");
    exe::crt_qsort(read_usize(this + eng_auto_cup::SELECTED_CLUBS), read_usize(this + eng_auto_cup::SELECTED_COUNT) as u32, 4, 0x53fd40); // cqs5: North and South by latitude
    let half = (read_usize(this + eng_auto_cup::SELECTED_COUNT) as i32) / 2;
    exe::crt_qsort(read_usize(this + eng_auto_cup::SELECTED_CLUBS), half as u32, 4, 0x4b0810);
    let half = (read_usize(this + eng_auto_cup::SELECTED_COUNT) as i32) / 2;
    exe::crt_qsort(read_usize(this + eng_auto_cup::SELECTED_CLUBS) + half as usize * 4, half as u32, 4, 0x4b0810);
    1
}

/// rule: the League Trophy (LDV Vans) has 56 clubs: all of the second and third divisions, the club relegated from the
/// third to the Conference and, to complete it, Conference clubs (with the nation rule on, those already in it by
/// category, 6 or less first; otherwise up to 30 Conference clubs or other English clubs outside the generic division,
/// sorted by reputation with the best shuffled); the list is split into North and South by the latitude of the stadium
/// city and each half goes from lowest reputation to highest
/// Original: `EngAutoCup::select_section_clubs` (0x56b730, eng_auto_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn select_section_clubs(this: usize) -> u32 {
    eng_auto_cup_select_section_clubs(this)
}

/// Creates the Trophy section stage: a knockout (Cup_Stage) with the 56 clubs in list order (North, then South), using
/// the rounds of stage 0 of the cup table, no seeding.
///
/// `EngAutoCup::create_section_stage()`; SEH frame only for the new, no destructor. The section stage (North and South)
/// with the `.team_count()` clubs of list `eng_auto_cup::SELECTED_CLUBS`, in order: a vector (`exe::crt_malloc` of 4 ×
/// `.team_count()`; out of memory, the box for line 0x2e9) holding them; the rounds through method
/// `comp_vtable::CREATE_SCHEDULE` (push 0; push &extra; push &rounds; push 0; mov ecx -> thiscall, ret 0x10); the stage
/// (`exe::crt_operator_new`(0xb2); Cup_Stage::Cup_Stage `FN_CUP_STAGE_CUP_STAGE`, thiscall, ret 0x3c: the cup,
/// `.team_count()` (word), the vector, the rounds (word), +4, the table, the year `.year()` (word), 0, 1, the extra
/// (word), `.importance_bias()` (byte), 0, 0, 0, no seeds; the words and bytes are pushed with garbage in the upper
/// bits, which the constructor does not read) into `.stages()`[0]; the vector and the table are freed
/// (`exe::crt_free`). Without a stage, the box for line 0x300; with it, `.set_current_stage(0)`.
/// Original: `EngAutoCup::create_section_stage` (0x56bab0, eng_auto_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn create_section_stage(this: usize) {
    let this = Cup(this);
    let teams = exe::crt_malloc(((this.team_count() as i32) << 2) as u32);
    if teams == 0 {
        return show_error_at(0x9c4180, 0x2e9);
    }
    let mut i = 0i32;
    while i < this.team_count() as i32 {
        write_usize(teams + i as usize * 4, read_usize(read_usize(this.addr() + eng_auto_cup::SELECTED_CLUBS) + i as usize * 4)); // cqs5: the order of the brackets
        i += 1;
    }
    let rounds_of_cup: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let table = rounds_of_cup(this.addr(), 0, &mut rodadas, &mut extra, 0);
    let p = exe::crt_operator_new(0xb2);
    let f = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this.addr(), this.team_count() as u16 as u32, teams, rodadas as u16 as u32, read_usize(this.addr() + 4), table, this.year() as u16 as u32, 0, 1,
            extra as u16 as u32, this.importance_bias() as u32, 0, 0, 0, core::ptr::null())
    } else {
        0
    };
    write_usize(this.stages(), f);
    exe::crt_free(teams);
    exe::crt_free(table);
    if read_usize(this.stages()) == 0 {
        return show_error_at(0x9c4180, 0x300);
    }
    covered!("cqs_5 trophy sections stage");
    this.set_current_stage(0);
}

/// Moves the Trophy on by one stage; when the sections end (stage 1) it builds the final between the North and South
/// winners; at the last stage, the error box.
///
/// `EngAutoCup::advance_stage()`, virtual +0x28. At the last stage (`.current_stage()` ≥ `.stage_count()` − 1, signed),
/// the box for line 0x314 and nothing else; otherwise `.current_stage()` goes up by one and, on reaching 1 (the end of
/// the sections), it builds the final (EngAutoCup::create_final_stage `exe::engautocup_create_final_stage`, direct call
/// with the cup's ecx -> thiscall, ret 0).
/// Original: `EngAutoCup::advance_stage` (0x56bc50, eng_auto_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn eng_auto_cup_advance_stage(this: usize) {
    let this = Cup(this);
    let stage = this.current_stage() as i32;
    if stage >= (this.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(0x9c4180, 0x314);
    }
    let stage = stage + 1;
    this.set_current_stage(stage as usize);
    if stage == 1 {
        covered!("cqs_5 trophy final");
        exe::engautocup_create_final_stage(this);
    }
}

/// First club among the first n 6-byte entrants whose status (+5) is s, or None.
pub(crate) unsafe fn first_entrant_with_status(l: usize, n: i32, s: u8) -> Option<usize> {
    (0..n.max(0) as usize).find(|&i| read_u8(l + i * 6 + 5) == s).map(|i| read_usize(l + i * 6))
}

/// Builds the Trophy final when the sections end: a single tie (2-team Cup_Stage, stage 1) between the club still alive
/// in the cup list and the one still alive in the section stage, and draws the round at once (home side).
///
/// `EngAutoCup::create_final_stage()`; from the next stage, SEH frame only for the new, no destructor. The Trophy
/// final: a vector of 2 (`exe::crt_malloc` of 8; out of memory, the box for line 0x337) with the first club of status 1
/// (the one still alive [inference]) in the cup list (`.team_table()`) and the first of status 1 in the section stage
/// list (`.stages()`[0] + `cup::TEAM_TABLE`), both among the first `.team_count()` (of the cup); the rounds through
/// method `comp_vtable::CREATE_SCHEDULE` (push 0; push &extra; push &rounds; push 1; mov ecx -> thiscall, ret 0x10);
/// the stage (`exe::crt_operator_new`(0xb2); Cup_Stage::Cup_Stage `FN_CUP_STAGE_CUP_STAGE`, thiscall, ret 0x3c: the
/// cup, 2, the vector, the rounds (word), +4, the table, the year `.year()` (word), 1, 1, the extra (word), 0x14, 0, 0,
/// 0, no seeds) into `.stages()`[1]; the round table is freed (`exe::crt_free`). Without a stage, the box for line
/// 0x35a (the vector is left allocated, as in the original). With it, the round draw (Cup::draw_round
/// `exe::cup_draw_round`: push 0; mov ecx, stage -> thiscall, ret 4) and the vector is freed.
/// Original: `EngAutoCup::create_final_stage` (0x56bce0, eng_auto_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn eng_auto_cup_create_final_stage(this: usize) {
    let this = Cup(this);
    let l = exe::crt_malloc(8);
    if l == 0 {
        return show_error_at(0x9c4180, 0x337);
    }
    if let Some(c) = first_entrant_with_status(this.team_table(), this.team_count() as i32, 1) { // cqs6: the cup finalist
        write_usize(l, c);
    }
    if let Some(c) = first_entrant_with_status(read_usize(read_usize(this.stages()) + cup::TEAM_TABLE), this.team_count() as i32, 1) { // cqs6: the brackets finalist
        write_usize(l + 4, c);
    }
    let rounds_of_cup: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let table = rounds_of_cup(this.addr(), 1, &mut rodadas, &mut extra, 0);
    let p = exe::crt_operator_new(0xb2);
    let f = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this.addr(), 2, l, rodadas as u16 as u32, read_usize(this.addr() + 4), table, this.year() as u16 as u32, 1, 1, // cqs6: the year of the final
             extra as u16 as u32, 0x14, 0, 0, 0, core::ptr::null())
    } else {
        0
    };
    write_usize(this.stages() + 4, f);
    exe::crt_free(table);
    let f = read_usize(this.stages() + 4);
    if f == 0 {
        return show_error_at(0x9c4180, 0x35a);
    }
    covered!("cqs_6 trophy final");
    exe::cup_draw_round(f, 0); // cqs6: the draw of the final
    exe::crt_free(l);
}

/// Writes the Trophy final ranking into the cup statistics: the highest-reputation club of each half takes 1st and 2nd;
/// the others share one position per round, from the last round to the first.
///
/// `EngAutoCup::rank_teams()`, the virtual for the final ranking (n = 1 up to 2002-06-01). The final position of each
/// club in the cup statistics (+8, Comp_Stats; without them, the box for line 0x3bb). A vector of 2 × `.team_count()`
/// pointers (`exe::crt_malloc`; out of memory, the box for line 0x3c3): in the first half the clubs of the cup list
/// (`.team_table()`, 6-byte entries), in the second those of the section stage (`.stages()`[0] + `cup::TEAM_TABLE`);
/// each half sorted by reputation (`club::REPUTATION`, word), highest first (qsort `exe::crt_qsort` with 0x4b07d0). The
/// first two (the highest-reputation club of each half): the one with the higher reputation (signed, strictly) is 1st
/// and the other 2nd (Comp_Stats::set_club_final_position `exe::comp_stats_set_club_final_position`: push position;
/// push club; mov ecx, +8 -> thiscall, ret 8). Then, for each half, from the last round (`.round_count()` − 1) to the
/// first, as many clubs (the next ones of the half, from the 2nd on) as the round has in `cup_round::TIES` of the round
/// table (`.round_table()`, `cup_round::SIZE` bytes per round), with positions 3, 5, 7... (3, then 3 + 2 × those
/// already placed). The original pushes the dword of the position byte with stack garbage in the upper bits;
/// `exe::comp_stats_set_club_final_position` only reads the byte. The vector is freed.
/// Original: `EngAutoCup::rank_teams` (0x56c010, eng_auto_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn eng_auto_cup_rank_teams(this: usize) {
    let this = Cup(this);
    if read_usize(this.addr() + 8) == 0 {
        return show_error_at(0x9c4180, 0x3bb);
    }
    let v = exe::crt_malloc(((this.team_count() as i32) << 3) as u32);
    if v == 0 {
        return show_error_at(0x9c4180, 0x3c3);
    }
    let mut d: i16 = 0;
    while d < this.team_count() {
        let e = d as i32 as usize * 6;
        write_usize(v + d as i32 as usize * 4, read_usize(this.team_table() + e));
        let n = this.team_count() as i32;
        write_usize(v + (n + d as i32) as usize * 4, read_usize(read_usize(read_usize(this.stages()) + cup::TEAM_TABLE) + e));
        d = d.wrapping_add(1);
    }
    let n = this.team_count() as i32;
    exe::crt_qsort(v, n as u32, 4, 0x4b07d0);
    let n = this.team_count() as i32;
    exe::crt_qsort(v + n as usize * 4, n as u32, 4, 0x4b07d0);
    let a = Club(read_usize(v));
    let b = Club(read_usize(v + this.team_count() as i32 as usize * 4));
    covered!("cqs_6 trophy standings");
    if a.reputation() > b.reputation() {
        exe::comp_stats_set_club_final_position(read_usize(this.addr() + 8), a, 1);
        exe::comp_stats_set_club_final_position(read_usize(this.addr() + 8), read_usize(v + this.team_count() as i32 as usize * 4), 2);
    } else {
        exe::comp_stats_set_club_final_position(read_usize(this.addr() + 8), a, 2);
        exe::comp_stats_set_club_final_position(read_usize(this.addr() + 8), read_usize(v + this.team_count() as i32 as usize * 4), 1);
    }
    for lado in 0..2u8 {
        let mut k: i16 = if lado == 0 { 1 } else { this.team_count().wrapping_add(1) };
        let mut pos: u8 = 3;
        let last = this.round_count().wrapping_sub(1);
        if last < 0 {
            continue;
        }
        let mut r = last as i32;
        while r >= 0 {
            let p = pos;
            let mut j: i16 = 0;
            while j < read_i16(this.round_table() + r as usize * cup_round::SIZE + cup_round::TIES) {
                let c = read_usize(v + k as i32 as usize * 4);
                k = k.wrapping_add(1);
                exe::comp_stats_set_club_final_position(read_usize(this.addr() + 8), c, p as u32);
                pos = pos.wrapping_add(2);
                j = j.wrapping_add(1);
            }
            r -= 1;
        }
    }
    exe::crt_free(v);
}

// simplification: writes to the cup statistics (Comp_Stats, which finds or appends the club’s entry: allocates); original only

/// rule: the League Trophy (Auto Windscreens) has two 28-club sections; the second takes the last 28 of the 56 chosen
/// clubs (the array at +0xb2, already sorted by latitude and reputation: the southern section [inference]), in order,
/// class and status 0
/// Original: `EngAutoCup::collect_entrants` (0x56b3c0, eng_auto_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn eng_auto_cup_collect_entrants(this: usize) -> i32 {
    collect_auto_cup_entrants(Cup(this)).0
}

/// rule: the LDV Vans Trophy has two sections (stages −1 and 0, North and South [inference], ids 0x41d and 0x41e) of
/// five single-match rounds: 1st on 10/12 (24 teams), 2nd on 14/1 (16 teams, 4 new), quarter-finals on 4/2, semi-final
/// on 18/2 and the section final over two legs from 4/3 (7 days); the final (stage 1) drawn on 12/3 and played on 20/4
/// of the following year with venue 4 (neutral ground [inference]); category 4 in the sections and 6 in the finals; no
/// per-round prizes
/// Original: `EngAutoCup::create_round_table` (0x56ade0, eng_auto_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn eng_auto_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_round_table(Cup(this), ldv_trophy_stage(stage), rounds, extra, league, 0x9c4180)
}

/// The season turnover of the Auto Windscreens Shield (the Football League Trophy): last season's tables, selected
/// clubs and section stages dropped, the season archived, the year plus one, then the 56 clubs selected again, the
/// entrants, the round table, the tie tree and the first section stage rebuilt; the selected list is freed again and
/// the predicted positions close it.
///
/// `cup::SEASON_CLOSED` = 0; the team table (`cup::TEAM_TABLE`), the round table (`cup::ROUND_TABLE`) and the selected
/// clubs (`eng_auto_cup::SELECTED_CLUBS`) freed (`exe::crt_free`) and zeroed; when there are tie tables
/// (`cup::TIE_TABLES`), each round's (`cup::ROUND_COUNT` rounds, a signed word) freed and zeroed and the array freed and
/// zeroed; with a current stage (`cup::CURRENT_STAGE`) ≥ 0, the stages 0..=current destroyed (virtual 0 with 1) and
/// then `cup::CURRENT_STAGE` = −1; with statistics, Comp_Stats::archive_season_and_reset
/// (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1), ret 4); the year up by one and
/// `cup::CURRENT_ROUND` = 0.
///
/// Then, each with its own box (eng_auto_cup.cpp): the 56 section clubs (EngAutoCup::select_section_clubs 0x56b730,
/// thiscall, ret 0; box 0x1ee when it returns 0); `cup::RUN_BY_STAGES` = 0; the entrants (EngAutoCup::collect_entrants
/// 0x56b3c0, box 0x1f8 when negative); the round table (virtual `comp_vtable::SET_UP_RULES`, box 0x1ff when negative)
/// and the tie tree (virtual `cup_vtable::MAP_FIXTURE_TREE`, box 0x206 when negative). Then the first section stage
/// (EngAutoCup::create_section_stage 0x56bab0, thiscall, ret 0), the selected clubs freed and zeroed again, and the
/// predicted positions (virtual `comp_vtable::SET_PREDICTED_POSITIONS`), whose eax it returns. After a box the copy
/// returns 0 (the original returns the box's eax; the turnover's caller does not read it).
/// Original: `EngAutoCup::start_new_season` (0x56b4b0, eng_auto_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn eng_auto_cup_start_new_season(this: usize) -> u32 {
    let this = Cup(this);
    this.set_season_closed(0);
    for o in [cup::TEAM_TABLE, cup::ROUND_TABLE, eng_auto_cup::SELECTED_CLUBS] {
        let l = read_usize(this.addr() + o);
        if l != 0 {
            exe::crt_free(l);
            write_usize(this.addr() + o, 0);
        }
    }
    if this.tie_tables() != 0 {
        let mut i = 0i32;
        while i < this.round_count() as i32 {
            let p = read_usize(this.tie_tables() + i as usize * 4);
            if p != 0 {
                exe::crt_free(p);
                write_usize(this.tie_tables() + i as usize * 4, 0);
            }
            i += 1;
        }
        exe::crt_free(this.tie_tables());
        this.set_tie_tables(0);
    }
    if read_i32(this.addr() + cup::CURRENT_STAGE) >= 0 {
        let mut i = 0i32;
        loop {
            let f = read_usize(read_usize(this.addr() + cup::STAGES) + 4 * i as usize);
            if f != 0 {
                let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(f)));
                destroy(f, 1);
            }
            i += 1;
            if i > read_i32(this.addr() + cup::CURRENT_STAGE) {
                break;
            }
        }
    }
    write_i32(this.addr() + cup::CURRENT_STAGE, -1);
    let stats = read_usize(this.addr() + 8);
    if stats != 0 {
        exe::comp_stats_archive_season_and_reset(stats, 1);
    }
    let a = this.year();
    this.set_year(a.wrapping_add(1)); // mwf13: the year of the Auto Windscreens Shield
    this.set_current_round(0);
    covered!("mwf_13 auto windscreens rollover");
    if (exe::engautocup_select_section_clubs(this) as u32) == 0 {
        show_error_at(0x9c4180, 0x1ee);
        return 0;
    }
    this.set_run_by_stages(0);
    let entrants: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_ENGAUTOCUP_COLLECT_ENTRANTS);
    if entrants(this.addr()) < 0 {
        show_error_at(0x9c4180, 0x1f8);
        return 0;
    }
    if call_cup_virtual(this, comp_vtable::SET_UP_RULES) < 0 {
        show_error_at(0x9c4180, 0x1ff);
        return 0;
    }
    if call_cup_virtual(this, cup_vtable::MAP_FIXTURE_TREE) < 0 {
        show_error_at(0x9c4180, 0x206);
        return 0;
    }
    exe::engautocup_create_section_stage(this);
    let l = read_usize(this.addr() + eng_auto_cup::SELECTED_CLUBS);
    if l != 0 {
        exe::crt_free(l);
        write_usize(this.addr() + eng_auto_cup::SELECTED_CLUBS, 0);
    }
    call_cup_virtual(this, comp_vtable::SET_PREDICTED_POSITIONS) as u32
}

/// Comparer for `eng_auto_cup_start_new_season`: only the original runs (it frees and rebuilds the cup); the year must
/// go up by one.
pub(crate) unsafe extern "thiscall" fn compare_eng_auto_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ENG_AUTO_CUP_START_NEW_SEASON);
    compare_cup_new_season("eng_auto_cup_start_new_season", Cup(this), || o(this))
}

/// The Auto Windscreens Shield rounds: a prize fund of 400,000 and the round table.
///
/// `cup::PRIZE_FUND` = 400,000 (0x61a80); the round table of stage −1 (virtual `comp_vtable::CREATE_SCHEDULE`: push 0;
/// push &`cup::STAGE_LABEL`; push &`cup::ROUND_COUNT`; push −1 -> thiscall, ret 0x10) into `cup::ROUND_TABLE`. The
/// substitutes and changes are not set here. Returns 1.
/// Original: `EngAutoCup::set_up_rounds` (0x56b700, eng_auto_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn eng_auto_cup_set_up_rounds(this: usize) -> i32 {
    let this = Cup(this);
    covered!("mwf_13 auto windscreens rounds");
    this.set_prize_fund(0x61a80); // mwf13: the Auto Windscreens Shield prize fund
    let table: unsafe extern "thiscall" fn(usize, u32, usize, usize, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let t = table(this.addr(), 0xffff_ffff, this.addr() + cup::ROUND_COUNT, this.addr() + cup::STAGE_LABEL, 0);
    this.set_round_table(t);
    1
}

/// Comparer for `eng_auto_cup_set_up_rounds`: side by side (as the Copa do Brasil's): the prize fund, the round count
/// and the stage label, and the bytes the round table writes (`eng_cc_cup_written_round_bytes`); the copy's table is
/// freed and the original's kept.
pub(crate) unsafe extern "thiscall" fn compare_eng_auto_cup_set_up_rounds(this: usize) -> i32 {
    let this = Cup(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_ENG_AUTO_CUP_SET_UP_ROUNDS);
    let to = core::cell::Cell::new(0usize);
    let tr = core::cell::Cell::new(0usize);
    let before = this.round_table();
    let reg = [(this.addr() + cup::PRIZE_FUND, 4usize), (this.addr() + cup::STAGE_LABEL, 4), (G_MOUSE_LEFT_BUTTON_DOWN, 4)];
    let (a, b, same, _, _) = crate::comparison::side_by_side(&reg, || {
        let r = o(this.addr());
        to.set(this.round_table());
        this.set_round_table(before);
        r
    }, || {
        let r = eng_auto_cup_set_up_rounds(this.addr());
        tr.set(this.round_table());
        this.set_round_table(to.get());
        r
    });
    let (to, tr) = (to.get(), tr.get());
    let n = this.round_count();
    let table = if to != 0 && tr != 0 { eng_cc_cup_written_round_bytes(to, n) == eng_cc_cup_written_round_bytes(tr, n) } else { to == 0 && tr == 0 };
    if tr != 0 && tr != to {
        exe::crt_free(tr);
    }
    SCOREBOARD.record(same && a == b && table, || format!("eng_auto_cup_set_up_rounds({:#x}) returned {} x {} memory {} table {}", this.addr(), a, b, same, table));
    a
}

/// Rule: in the LDV Vans Trophy, in the sections (stages up to 0) elimination and qualification weigh by round (1 to 80
/// over the 5 rounds) and the section winner gets the record of reaching the final (round 0x96, weight 0x5a); in the
/// final (stage 1) the winner and runner-up get the final records.
///
/// `EngAutoCup::update_staff_records_for_status(club, outcome, stage, _, fixture, final)` (the virtual +0x44); returns
/// 0. Stage (signed byte) up to 0, weight = `scale_by_round`(1, 0x50, the fixture's `round_number()`, 5): outcome 0xff
/// or 2, the elimination (`european_cup_add_knocked_out_record`: `round_prefix()`, `round_name()`, the weight); 0, the
/// qualification (`european_cup_add_qualified_record`: `round_prefix()`, the name word +0x6f of the fixture's round in
/// the cup's round table `cup::ROUND_TABLE`, the weight; the high 16 bits the original pushes come from small products
/// of the round: zero); 1, the qualification for the final (0, 0x96, 0x5a). Stage 1: outcome 1 the winner
/// (Staff_Records 0x86a800), 2 the runner-up (0x86b0b0). Anything else: nothing.
/// Original: `EngAutoCup::update_staff_records_for_status` (0x56bee0, eng_auto_cup.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn eng_auto_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, _a4: u32, game: usize, final_: u32) -> u32 {
    let g = Fixture(game);
    let (d, f) = (outcome as u8, stage as u8 as i8);
    let comp = read_usize(this + 4);
    if f <= 0 {
        match d {
            0xff | 2 => {
                covered!("cqx_11 ldv trophy knocked out");
                let weight = scale_by_round(1, 0x50, g.round_number(), 5);
                european_cup_add_knocked_out_record(club, comp, g.round_prefix() as u16 as u32, g.round_name() as u16 as u32, weight);
            }
            0 => {
                let weight = scale_by_round(1, 0x50, g.round_number(), 5);
                let name = read_u16(Cup(this).round_table().wrapping_add((g.round_number() as isize * cup_round::SIZE as isize) as usize) + 0x6f) as u32;
                european_cup_add_qualified_record(club, comp, g.round_prefix() as u16 as u32, name, weight);
            }
            1 => european_cup_add_qualified_record(club, comp, 0, 0x96, 0x5a),
            _ => {}
        }
    } else if f == 1 {
        match d {
            1 => cup_record_final_result(0x86a800, club, g, final_),
            2 => cup_record_final_result(0x86b0b0, club, g, final_),
            _ => {}
        }
    }
    0
}

pub(crate) unsafe extern "thiscall" fn compare_eng_auto_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, a4: u32, game: usize, final_: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_ENG_AUTO_CUP_UPDATE_STAFF_RECORDS_FOR_STATUS)(this, club, outcome, stage, a4, game, final_);
    SCOREBOARD.record(r == 0, || format!("eng_auto_cup_update_staff_records_for_status({:#x}, {}, {}) returned {}", club, outcome as u8, stage as u8, r));
    r
}

/// The English League Trophy constructor: see `eng_auto_cup_constructor_body`.
/// Original: `EngAutoCup::EngAutoCup` (0x56aa30, eng_auto_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_eng_auto_cup(this: usize, year: u32, owner: usize) -> usize {
    eng_auto_cup_constructor_body(this, year, owner) // cqv9: the League Trophy
}

/// Only the original (it allocates, draws and reads the save file), checking the plan
/// (`eng_auto_european_check_constructor_plan`).
pub(crate) unsafe extern "thiscall" fn compare_construct_eng_auto_cup(this: usize, year: u32, owner: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_ENG_AUTO_CUP)(this, year, owner);
    let error = eng_auto_european_check_constructor_plan(this, r, year, owner, 0x969894, &LEAGUE_TROPHY_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_eng_auto_cup({:#x}, {}, {:#x}): {}", this, year as u16, owner, error.clone().unwrap_or_default()));
    r
}

/// The plan the comparer checks after the original: the return value (this), the vtable, the competition record
/// (`league::COMPETITION`) and the year (`cup::YEAR`); on a new cup (`cup::STAGE_INDEX` = `stage_index::MAIN`), each
/// fixed field the constructor writes (offset, width, value) and the rounds (`cup::ROUND_TABLE`, when `rounds`) and the
/// statistics created.
pub(crate) unsafe fn eng_auto_european_check_constructor_plan(this: usize, r: usize, year: u32, owner: usize, vt: usize, fields: &[(usize, u8, u32)], rounds: bool) -> Option<String> {
    let this = League(this);
    if r != this.addr() || read_usize(this.addr()) != vt || this.competition().addr() != owner || read_u16(this.addr() + cup::YEAR) != year as u16 {
        return Some(format!("returned {:#x}, vtable {:#x}, +4 {:#x}, year {}", r, read_usize(this.addr()), this.competition().addr(), read_u16(this.addr() + cup::YEAR)));
    }
    if read_u8(this.addr() + cup::STAGE_INDEX) != stage_index::MAIN || league_will_load_saved_state(owner) {
        return None;
    }
    for &(o, t, v) in fields {
        let x = match t {
            1 => read_u8(this.addr() + o) as u32,
            2 => read_u16(this.addr() + o) as u32,
            _ => read_u32(this.addr() + o),
        };
        if x != v {
            return Some(format!("+{:#x} = {:#x}, the constructor writes {:#x}", o, x, v));
        }
    }
    if (rounds && read_usize(this.addr() + cup::ROUND_TABLE) == 0) || this.comp_stats() == 0 {
        return Some(format!("rounds {:#x}, statistics {:#x}", read_usize(this.addr() + cup::ROUND_TABLE), this.comp_stats()));
    }
    None
}

/// The constructor of the English League Trophy, on the Cup base (`exe::new_cup`, thiscall, ret 0): the fixed fields
/// (the competition record in `league::COMPETITION`, the vtable 0x969894, the year in `cup::YEAR`,
/// `cup::CURRENT_ROUND` = 0, `cup::STAGE_INDEX` = `stage_index::MAIN`, `cup::CURRENT_STAGE` = -1, `cup::STAGE_COUNT` =
/// 2, `cup::COMP_TYPE` = `comp_type::CUP`, +0x1c and +0x20 = -1, `eng_auto_cup::SELECTED_CLUBS` = 0, discipline rule
/// `ENGLAND` (9), 5 substitutes named, 3 substitutions allowed); if the saved state loads (Cup::load_from_disk 0x51fc00:
/// push 1; mov ecx, this -> thiscall, ret 4; returns != 0), returns this. On a new cup, in order: the stage array in
/// `cup::STAGES` (`exe::crt_malloc` of 4 x `cup::STAGE_COUNT`, or 0 with no stages); `cup::IMPORTANCE_BIAS` = 0xf and
/// `cup::RUN_BY_STAGES` = 0; the 56 section clubs (`select_section_clubs`, 0x56b730, thiscall, ret 0; the box for line
/// 0x52 if it returns 0); the entrants (`eng_auto_cup_collect_entrants`, 0x56b3c0, thiscall, ret 0; the box for line
/// 0x59 if it returns 0); the prize fund (`cup::PRIZE_FUND` = 400,000); the rounds (`eng_auto_european_create_rounds`);
/// the statistics in `league::COMP_STATS` (the box for line 0x68 without them); the section stage
/// (`create_section_stage`, 0x56bab0, thiscall, ret 0, unchecked); the list `eng_auto_cup::SELECTED_CLUBS` freed
/// (`exe::crt_free`) and zeroed; the tie tree (`map_fixture_tree`, 0x518790, thiscall, ret 0; the box for line 0x78 if
/// it returns 0); the final ranking (`eng_auto_cup_rank_teams`, 0x56c010, thiscall, ret 0, unchecked). Every failing
/// step shows its box (`show_error_at`) and returns this. Returns this. Rule: prize fund of 400,000; 5 substitutes on the
/// bench and 3 substitutions; discipline rule 9 [from the competitions reference reading, section 2.1].
/// Original: `EngAutoCup::EngAutoCup` (0x56aa30, eng_auto_cup.cpp, thiscall, ret 8).
pub(crate) unsafe fn eng_auto_cup_constructor_body(this: usize, year: u32, owner: usize) -> usize {
    let this = League(this);
    let cup = Cup(this.addr());
    exe::new_cup(cup);
    write_usize(this.addr() + league::COMPETITION, owner);
    write_usize(this.addr(), 0x969894);
    write_u16(this.addr() + cup::YEAR, year as u16);
    cup.set_current_round(0x0);
    cup.set_stage_index(stage_index::MAIN);
    write_u32(this.addr() + cup::CURRENT_STAGE, 0xffffffff);
    write_u32(this.addr() + cup::STAGE_COUNT, 0x2);
    cup.set_comp_type(comp_type::CUP);
    this.set_upper_division_id(0xffffffffu32 as i32);
    this.set_lower_division_id((0xffffffff) as usize);
    write_u32(this.addr() + eng_auto_cup::SELECTED_CLUBS, 0x0);
    cup.set_discipline_index(discipline_rule::ENGLAND);
    cup.set_substitutes_named(0x5);
    cup.set_substitutions_allowed(0x3);
    if exe::cup_load_from_disk(this, 1) != 0 {
        return this.addr();
    }
    covered!("cqv_9 league trophy new");
    let stages = cup.stage_count() as i32;
    cup.set_stages(if stages > 0 { exe::crt_malloc((stages << 2) as u32) } else { 0 });
    cup.set_importance_bias(0xf);
    cup.set_run_by_stages(0x0);
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_ENGAUTOCUP_SELECT_SECTION_CLUBS);
    let r = f(this.addr());
    if r == 0 {
        show_error_at(0x9c4180, 0x52);
        return this.addr();
    }
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_ENGAUTOCUP_COLLECT_ENTRANTS);
    let r = f(this.addr());
    if r == 0 {
        show_error_at(0x9c4180, 0x59);
        return this.addr();
    }
    cup.set_prize_fund(0x61a80);
    eng_auto_european_create_rounds(cup);
    if !eng_auto_european_create_comp_stats(this.addr()) {
        show_error_at(0x9c4180, 0x68);
        return this.addr();
    }
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_ENGAUTOCUP_CREATE_SECTION_STAGE);
    f(this.addr());
    let l = read_usize(this.addr() + eng_auto_cup::SELECTED_CLUBS);
    if l != 0 {
        exe::crt_free(l);
        write_u32(this.addr() + eng_auto_cup::SELECTED_CLUBS, 0);
    }
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_CUP_MAP_FIXTURE_TREE);
    let r = f(this.addr());
    if r == 0 {
        show_error_at(0x9c4180, 0x78);
        return this.addr();
    }
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_ENGAUTOCUP_RANK_TEAMS);
    f(this.addr());
    this.addr()
}

/// The cup's rounds through its virtual `comp_vtable::CREATE_SCHEDULE` (push 0; push &`cup::STAGE_LABEL`; push
/// &`cup::ROUND_COUNT`; push -1; mov ecx, this -> thiscall, ret 0x10): the round table, stored in `cup::ROUND_TABLE`.
pub(crate) unsafe fn eng_auto_european_create_rounds(cup: Cup) {
    let f: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(cup.addr()) + comp_vtable::CREATE_SCHEDULE));
    let t = f(cup.addr(), 0xffff_ffff, cup.addr() + cup::ROUND_COUNT, cup.addr() + cup::STAGE_LABEL, 0);
    cup.set_round_table(t);
}

/// The statistics (`exe::crt_operator_new` of 0x5ce, cdecl; `exe::new_comp_stats`(new, this), thiscall, ret 4) into
/// `league::COMP_STATS` (the Comp base field); the original's SEH frame covers only that `new`. Returns whether they
/// were created.
pub(crate) unsafe fn eng_auto_european_create_comp_stats(this: usize) -> bool {
    let this = League(this);
    let n = exe::crt_operator_new(0x5ce);
    let s = if n != 0 { exe::new_comp_stats(n, this) } else { 0 };
    this.set_comp_stats(s);
    s != 0
}

/// The fixed fields the League Trophy constructor writes on a new cup (offset, width, value).
pub(crate) const LEAGUE_TROPHY_FIELDS: [(usize, u8, u32); 13] = [(0x1c, 4, 0xffffffff), (0x20, 4, 0xffffffff), (cup::PRIZE_FUND, 4, 0x61a80), (cup::STAGE_COUNT, 4, 0x2), (cup::COMP_TYPE, 1, 0x1), (cup::STAGE_INDEX, 1, 0xff), (cup::SUBSTITUTES_NAMED, 1, 0x5), (cup::SUBSTITUTIONS_ALLOWED, 1, 0x3), (cup::DISCIPLINE_INDEX, 1, 0x9), (cup::IMPORTANCE_BIAS, 1, 0xf), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0), (eng_auto_cup::SELECTED_CLUBS, 4, 0x0)];
