// ---------- %%LEVA%%a leva: the Asian Cup Winners' Cup: the 32 qualifiers of the two zones, the west zone stage and the predicted positions (asia_cup_winner.cpp) ----------

/// The nation id of the club ([[`club::NATION`]], dword); without a club or a nation, −1.
unsafe fn asia_cup_winner_club_nation_id(c: usize) -> i32 {
    if c == 0 || read_usize(c + club::NATION) == 0 {
        return -1;
    }
    read_usize(read_usize(c + club::NATION)) as i32
}

/// The Cup Winners' Cup east zone (23 nations, in the original's order; Pakistan comes after Laos).
const ASIAN_CUP_WINNERS_EAST_ZONE_KEYS: [usize; 23] = [KEY_NATION_BANGLADESH, KEY_NATION_BRUNEI_DARUSSALAM, KEY_NATION_CHINA, KEY_NATION_HONG_KONG, KEY_NATION_INDIA,
    KEY_NATION_INDONESIA, KEY_NATION_JAPAN, KEY_NATION_KAMPUCHEA, KEY_NATION_LAOS, KEY_NATION_PAKISTAN, KEY_NATION_MACAU, KEY_NATION_MALAYSIA, KEY_NATION_MALDIVES,
    KEY_NATION_MYANMAR, KEY_NATION_NEPAL, KEY_NATION_NORTH_KOREA, KEY_NATION_SINGAPORE, KEY_NATION_SOUTH_KOREA, KEY_NATION_SRI_LANKA, KEY_NATION_TAIWAN,
    KEY_NATION_THAILAND, KEY_NATION_THE_PHILIPPINES, KEY_NATION_VIETNAM];

/// The west zone (19 nations, as in the Asian Club Championship).
const ASIAN_CUP_WINNERS_WEST_ZONE_KEYS: [usize; 19] = [KEY_NATION_AFGHANISTAN, KEY_NATION_BAHRAIN, KEY_NATION_IRAN, KEY_NATION_IRAQ, KEY_NATION_JORDAN,
    KEY_NATION_KAZAKHSTAN, KEY_NATION_KUWAIT, KEY_NATION_KYRGYZSTAN, KEY_NATION_LEBANON, KEY_NATION_OMAN, KEY_NATION_PALESTINE, KEY_NATION_QATAR,
    KEY_NATION_SAUDI_ARABIA, KEY_NATION_SYRIA, KEY_NATION_TAJIKISTAN, KEY_NATION_TURKMENISTAN, KEY_NATION_UNITED_ARAB_EMIRATES, KEY_NATION_UZBEKISTAN,
    KEY_NATION_YEMEN];

/// The index of the club's nation in a list of nation ids (0x40cba0(club, list, n): 3 pushes; add esp, 0xc -> cdecl;
/// returns the index byte or −1).
unsafe fn asia_cup_winner_zone_index(club: usize, list: &[u32]) -> i8 {
    let f: unsafe extern "C" fn(usize, *const u32, u32) -> u32 = core::mem::transmute(0x40cba0usize);
    f(club, list.as_ptr(), list.len() as u32) as u8 as i8
}

/// Does the club already play the Asian Club Championship (0x40e7b0(club): push; add esp, 4 -> cdecl): in that cup's
/// team table or in that of its stage 0.
unsafe fn asia_cup_winner_in_champions_league(club: usize) -> bool {
    let f: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x40e7b0usize);
    f(club) != 0
}

/// The winner of the cup `cup_key` of the nation `nation_key` (when the nation's `nation::LEAGUE_SELECTED` byte is set,
/// in its [`G_NATION_TABLE`] record): the winner, or the runner-up if the winner is already in place 0 or 16 or plays
/// the Champions League; the runner-up in those conditions (or null) does not enter.
unsafe fn asia_cup_winner_national_cup_club(this: usize, nation_key: usize, cup_key: usize) -> Option<usize> {
    let n = read_usize(nation_key);
    if read_u8(read_usize(G_NATION_TABLE).wrapping_add(n.wrapping_mul(nation::SIZE)).wrapping_add(nation::LEAGUE_SELECTED)) == 0 {
        return None;
    }
    let cup_record = comp_record_of_key(cup_key);
    let list = read_usize(this + 0xb2);
    let c = exe::comp_util_get_latest_placed_club(cup_record, 0); // cqu11: the cup winner
    if c != read_usize(list) && c != read_usize(list + 0x40) && !asia_cup_winner_in_champions_league(c) {
        return if c != 0 { Some(c) } else { None };
    }
    let c = exe::comp_util_get_latest_placed_club(cup_record, 1);
    if c == read_usize(list) || c == read_usize(list + 0x40) || asia_cup_winner_in_champions_league(c) || c == 0 {
        return None;
    }
    Some(c)
}

/// The 32 Asian Cup Winners' Cup qualifiers: +0xb6 = 32 and the 32-place list +0xb2 (`exe::crt_calloc`(0x20, 4), cdecl;
/// out of memory, the error box of line 0x241 and 0). The latest winner of the Cup Winners' Cup itself
/// (`exe::comp_util_get_latest_placed_club`([+4], 0), cdecl), if it does not play the Champions League, goes to place
/// 0 if its nation is in the east zone, otherwise to 16. In a local list of up to 1,200 clubs: the Emperor's Cup winner
/// ([`KEY_CLUB_COMP_JAPANESE_EMPEROR_S_CUP`]; the runner-up if the winner is already in places 0/16 or plays the
/// Champions League), if Japan has its `nation::LEAGUE_SELECTED` byte, and the Korean cup's
/// ([`KEY_CLUB_COMP_KOREAN_FA_CUP`]) in the same way; and every Asian club ([`KEY_CONTINENT_ASIA`]) outside places 0
/// and 16. The list is sorted and shuffled (`exe::comp_util_sort_and_shuffle_top_clubs`(list, n − 1, 40), cdecl). In
/// its order, each club whose nation has not had a place yet and that does not play the Champions League takes the
/// first free place of its zone (0..15 east, 16..31 west), while the zone's last place is free; the nation is marked
/// even when the zone is full. Returns 1. (Stack frame 0x1598.)
/// Original: `AsiaCupWinner::select_qualified_clubs` (0x40d830, asia_cup_winner.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn asia_cup_winner_select_qualified_clubs(this: usize) -> u32 {
    write_u32(this + 0xb6, 0x20);
    let east: [u32; 23] = core::array::from_fn(|i| read_usize(ASIAN_CUP_WINNERS_EAST_ZONE_KEYS[i]) as u32);
    let west: [u32; 19] = core::array::from_fn(|i| read_usize(ASIAN_CUP_WINNERS_WEST_ZONE_KEYS[i]) as u32);
    let list = exe::crt_calloc(0x20, 4);
    write_usize(this + 0xb2, list);
    if list == 0 {
        show_error_at(0x987b60, 0x241);
        return 0;
    }
    let holder = exe::comp_util_get_latest_placed_club(read_usize(this + 4), 0);
    if !asia_cup_winner_in_champions_league(holder) {
        if asia_cup_winner_zone_index(holder, &east) > -1 {
            write_usize(list, holder);
        } else {
            write_usize(list + 0x40, holder);
        }
    }
    let mut clubs = [0usize; 0x4b0];
    let mut n = 0usize;
    if let Some(c) = asia_cup_winner_national_cup_club(this, KEY_NATION_JAPAN, KEY_CLUB_COMP_JAPANESE_EMPEROR_S_CUP) {
        covered!("cqu_11 emperors cup club");
        clubs[0] = c;
        n = 1;
    }
    if let Some(c) = asia_cup_winner_national_cup_club(this, KEY_NATION_SOUTH_KOREA, KEY_CLUB_COMP_KOREAN_FA_CUP) {
        clubs[n] = c;
        n += 1;
    }
    let total = (read_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_usize(G_NATION_COUNT) as i32).wrapping_mul(2));
    let asia = read_usize(KEY_CONTINENT_ASIA);
    let mut c = read_usize(G_CLUB_TABLE);
    for _ in 0..total.max(0) {
        let nation = read_usize(c + club::NATION);
        if nation != 0 {
            let continent = read_usize(nation + nation::CONTINENT);
            if continent != 0 && read_usize(continent) == asia && (n as i16) < 0x4b0 && c != read_usize(list) && c != read_usize(list + 0x40) {
                clubs[n] = c;
                n += 1;
            }
        }
        c += club::SIZE;
    }
    covered!("cqu_11 club draw");
    exe::comp_util_sort_and_shuffle_top_clubs(clubs.as_mut_ptr() as usize, (n as u32).wrapping_sub(1), 0x28); // cqu11: 40 picks in the shuffle
    let (mut east_marks, mut west_marks) = ([0u8; 23], [0u8; 19]);
    for &club in clubs.iter().take((n as i16).max(0) as usize) {
        let i = asia_cup_winner_zone_index(club, &east);
        let (mark, first) = if i > -1 {
            (&mut east_marks[i as usize], 0usize)
        } else {
            let j = asia_cup_winner_zone_index(club, &west);
            if j <= -1 {
                continue;
            }
            (&mut west_marks[j as usize], 16usize)
        };
        if *mark != 0 || asia_cup_winner_in_champions_league(club) {
            continue;
        }
        *mark = 1;
        if read_usize(list + (first + 15) * 4) != 0 {
            continue;
        }
        if let Some(k) = (first..first + 16).find(|&k| read_usize(list + k * 4) == 0) {
            write_usize(list + k * 4, club);
        }
    }
    1
}

// ponytail: the list is allocated and the shuffle draws: it cannot run twice. Only the original; the plan: returns 1,
// +0xb6 = 32, the list at +0xb2, none of its clubs in the Asian Club Championship and, outside places 0 and 16 (the Cup
// Winners' Cup holder's), each club in its nation's zone and no nation repeated.
unsafe extern "thiscall" fn compare_asia_cup_winner_select_qualified_clubs(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    let r = o(this);
    if r == 1 {
        let list = read_usize(this + 0xb2);
        let mut nations: Vec<i32> = Vec::new();
        let mut error = 0usize;
        for k in 0..32usize {
            let c = if list != 0 { read_usize(list + k * 4) } else { 0 };
            if c == 0 {
                continue;
            }
            if asia_cup_winner_in_champions_league(c) {
                error = 100 + k;
            }
            if k == 0 || k == 16 {
                continue;
            }
            let nation = asia_cup_winner_club_nation_id(c);
            let zone = if k < 16 { &ASIAN_CUP_WINNERS_EAST_ZONE_KEYS[..] } else { &ASIAN_CUP_WINNERS_WEST_ZONE_KEYS[..] };
            if !zone.iter().any(|&g| read_usize(g) as i32 == nation) || nations.contains(&nation) {
                error = k;
            }
            nations.push(nation);
        }
        let ok = list != 0 && read_usize(this + 0xb6) == 0x20 && error == 0;
        SCOREBOARD.record(ok, || format!("asia_cup_winner_select_qualified_clubs({:#x}): list {:#x} +0xb6 {} wrong place {}", this, list, read_usize(this + 0xb6), error));
    }
    r
}

/// The knockout stage of one zone of the Asian cups (0x40bec0 and 0x40ddf0, the same body; only the source file and
/// the box lines change): the `cup::TEAM_COUNT` teams (`exe::crt_malloc`(4 × n), cdecl; out of memory, the box
/// `lines.0`) are qualifiers 16.. of the list +0xb2; the round table through `comp_vtable::CREATE_SCHEDULE` (push 0;
/// push &extra; push &rounds; push 0; mov ecx, this -> thiscall, ret 0x10); the stage (`exe::crt_operator_new`(0xb2)
/// and the Cup_Stage constructor `FN_CUP_STAGE_CUP_STAGE`, thiscall, ret 0x3c, 15 arguments: the cup, n, the teams,
/// the rounds, +4, the table, the year `cup::YEAR`, 0, 2, the extra, `cup::IMPORTANCE_BIAS`, 0, 0, 0, 0; the original
/// pushes the words and the byte with garbage in the high bytes, which the constructor does not read: the copy passes
/// clean values) in [`cup::STAGES`][0]; the teams and the table freed (`exe::crt_free`). Without the stage, the box
/// `lines.1`; with it, `cup::CURRENT_STAGE` = 0.
unsafe fn asia_cup_winner_zone_knockout_stage(this: usize, file: usize, lines: (u32, u32), label: &'static str) {
    #[allow(clippy::type_complexity)]
    let construct_stage: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize =
        core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
    let n = read_i16(this + cup::TEAM_COUNT);
    let teams = exe::crt_malloc((n as i32 as u32).wrapping_mul(4));
    if teams == 0 {
        return show_error_at(file, lines.0);
    }
    for i in 0..n.max(0) as usize {
        covered!(label);
        write_usize(teams + i * 4, read_usize(read_usize(this + 0xb2) + 0x40 + i * 4)); // cqu11: team i of the west zone
    }
    let create_schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut extra) = (0i16, 0i16);
    let table = create_schedule(this, 0, &mut rounds, &mut extra, 0);
    let p = exe::crt_operator_new(0xb2);
    let stage = if p != 0 {
        construct_stage(p, this, n as u16 as u32, teams, rounds as u16 as u32, read_usize(this + 4), table, read_i16(this + cup::YEAR) as u16 as u32, 0, 2,
                        extra as u16 as u32, read_u8(this + cup::IMPORTANCE_BIAS) as u32, 0, 0, 0, 0)
    } else {
        0
    };
    write_usize(read_usize(this + cup::STAGES), stage);
    exe::crt_free(teams);
    exe::crt_free(table);
    if read_usize(read_usize(this + cup::STAGES)) == 0 {
        return show_error_at(file, lines.1);
    }
    write_u32(this + cup::CURRENT_STAGE, 0);
}

/// Stage 0 of the Asian Cup Winners' Cup, a knockout of the 16 west zone qualifiers (boxes at lines 0x2fd and 0x314).
/// (The original's SEH frame is only for the stage's new: no destructor on the normal path.)
/// Original: `AsiaCupWinner::create_zone_stage` (0x40ddf0, asia_cup_winner.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn asia_cup_winner_create_zone_stage(this: usize) {
    asia_cup_winner_zone_knockout_stage(this, 0x987b60, (0x2fd, 0x314), "cqu_11 west zone team")
}

// ponytail: the stage is allocated: it cannot run twice. Only the original; the plan: the stage in [`cup::STAGES`][0],
// `cup::CURRENT_STAGE` = 0 and, in its team table (`cup::TEAM_TABLE`, 6-byte rows), qualifiers 16.. of the list (in
// any order).
unsafe fn asia_cup_winner_check_zone_stage(name: &str, this: usize, o: impl FnOnce()) {
    let list = read_usize(this + 0xb2);
    let n = read_i16(this + cup::TEAM_COUNT).max(0) as usize;
    if list == 0 || n > 16 {
        return o();
    }
    let mut plan: Vec<usize> = (0..n).map(|i| read_usize(list + 0x40 + i * 4)).collect();
    o();
    let f = read_usize(read_usize(this + cup::STAGES));
    if f == 0 {
        return;
    }
    let t = read_usize(f + cup::TEAM_TABLE);
    let mut read: Vec<usize> = if t != 0 { (0..n).map(|i| read_usize(t + i * 6)).collect() } else { Vec::new() };
    read.sort_unstable();
    plan.sort_unstable();
    SCOREBOARD.record(read_usize(this + cup::CURRENT_STAGE) == 0 && read == plan, || format!("{}({:#x}): teams {:x?} plan {:x?} +0x30 {}", name, this, read, plan, read_usize(this + cup::CURRENT_STAGE) as i32));
}

unsafe extern "thiscall" fn compare_asia_cup_winner_create_zone_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    asia_cup_winner_check_zone_stage("asia_cup_winner_create_zone_stage", this, || o(this))
}

/// The predicted positions of the Asian cups (0x40c720 and 0x40e3b0, the same body): without statistics (+8), the box
/// `lines.0`; an array of 2n clubs (`exe::crt_malloc`(8 × `cup::TEAM_COUNT`); out of memory, the box `lines.1`): the n
/// of the cup's team table (`cup::TEAM_TABLE`, 6-byte rows) and the n of stage 0's team table; each half sorted by
/// reputation (`exe::crt_qsort`(v, n, 4, 0x4b07d0), cdecl); the 32 positions in order
/// (`exe::comp_stats_set_club_final_position`(stats, club, position), thiscall, ret 8): in each half 1, 3, 5, 7, 9, 9,
/// 9, 9 and eight 17s; the array freed.
unsafe fn asia_cup_winner_predicted_positions(this: usize, file: usize, lines: (u32, u32)) {
    if read_usize(this + 8) == 0 {
        return show_error_at(file, lines.0);
    }
    let v = exe::crt_malloc((read_i16(this + cup::TEAM_COUNT) as i32 as u32).wrapping_mul(8));
    if v == 0 {
        return show_error_at(file, lines.1);
    }
    let mut i = 0i16;
    while i < read_i16(this + cup::TEAM_COUNT) {
        let k = i as usize;
        write_usize(v + k * 4, read_usize(read_usize(this + cup::TEAM_TABLE) + k * 6));
        let n = read_i16(this + cup::TEAM_COUNT) as isize as usize;
        write_usize(v + (n + k) * 4, read_usize(read_usize(read_usize(read_usize(this + cup::STAGES)) + cup::TEAM_TABLE) + k * 6));
        i += 1;
    }
    let n = read_i16(this + cup::TEAM_COUNT) as i32;
    exe::crt_qsort(v, n as u32, 4, 0x4b07d0usize);
    let n = read_i16(this + cup::TEAM_COUNT) as i32;
    exe::crt_qsort(v + (n as usize).wrapping_mul(4), n as u32, 4, 0x4b07d0usize);
    for (k, &p) in ASIAN_CUP_WINNERS_PREDICTED_POSITIONS.iter().enumerate() {
        exe::comp_stats_set_club_final_position(read_usize(this + 8), read_usize(v + k * 4), p);
    }
    exe::crt_free(v);
}

/// The 32 predicted positions, in array order (the two halves).
const ASIAN_CUP_WINNERS_PREDICTED_POSITIONS: [u32; 32] = [1, 3, 5, 7, 9, 9, 9, 9, 17, 17, 17, 17, 17, 17, 17, 17, 1, 3, 5, 7, 9, 9, 9, 9, 17, 17, 17, 17, 17, 17, 17, 17];

/// The predicted positions of the two Asian Cup Winners' Cup zones (boxes at lines 0x3db and 0x3e3).
/// Original: `AsiaCupWinner::rank_teams` (0x40e3b0, asia_cup_winner.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn asia_cup_winner_rank_teams(this: usize) {
    asia_cup_winner_predicted_positions(this, 0x987b60, (0x3db, 0x3e3))
}

// ponytail: Comp_Stats::set_club_final_position adds the club's entry (it reallocates): it cannot run twice. Only the
// original; the plan: in each half (the cup's team table and stage 0's, 16 clubs each), the written positions
// (`comp_stats_club::FINAL_POSITION`, `exe::comp_stats_find_club_entry`) by reputation (`club::REPUTATION`) in
// descending order make 1, 3, 5, 7, 9, 9, 9, 9 and eight 17s; reputation ties may swap (qsort is not stable).
unsafe fn asia_cup_winner_check_positions(name: &str, this: usize, o: impl FnOnce()) {
    let n = read_i16(this + cup::TEAM_COUNT);
    let stats = read_usize(this + 8);
    let stages = read_usize(this + cup::STAGES);
    if n != 16 || stats == 0 || stages == 0 || read_usize(stages) == 0 || read_usize(this + cup::TEAM_TABLE) == 0 || read_usize(read_usize(stages) + cup::TEAM_TABLE) == 0 {
        return o();
    }
    let halves: [Vec<usize>; 2] = [
        (0..16).map(|k| read_usize(read_usize(this + cup::TEAM_TABLE) + k * 6)).collect(),
        (0..16).map(|k| read_usize(read_usize(read_usize(stages) + cup::TEAM_TABLE) + k * 6)).collect(),
    ];
    if halves.iter().flatten().any(|&c| c == 0) {
        return o();
    }
    o();
    let mut error = None;
    for (m, half) in halves.iter().enumerate() {
        let mut v: Vec<(i16, u8)> = half
            .iter()
            .map(|&c| {
                let e = exe::comp_stats_find_club_entry(stats, c);
                (read_i16(c + club::REPUTATION), if e != 0 { read_u8(e + comp_stats_club::FINAL_POSITION) } else { 0 })
            })
            .collect();
        v.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
        let positions: Vec<u8> = v.iter().map(|x| x.1).collect();
        if positions.iter().zip(ASIAN_CUP_WINNERS_PREDICTED_POSITIONS.iter()).any(|(&a, &b)| a as u32 != b) {
            error = Some((m, positions));
        }
    }
    SCOREBOARD.record(error.is_none(), || format!("{}({:#x}): positions by reputation {:?}", name, this, error));
}

unsafe extern "thiscall" fn compare_asia_cup_winner_rank_teams(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    asia_cup_winner_check_positions("asia_cup_winner_rank_teams", this, || o(this))
}
