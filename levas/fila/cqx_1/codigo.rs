// ---------- batch %%LEVA%%: the Asian Club Championship: the 32 qualifiers of the two zones, the west zone stage and the predicted positions (asia_club_champ.cpp) ----------

/// The Asian Club Championship east zone (23 nations, in the original's order; Pakistan comes last, unlike the Cup
/// Winners' Cup list).
const ASIAN_CLUB_CHAMP_EAST_ZONE_KEYS: [usize; 23] = [KEY_NATION_BANGLADESH, KEY_NATION_BRUNEI_DARUSSALAM, KEY_NATION_CHINA, KEY_NATION_HONG_KONG, KEY_NATION_INDIA,
    KEY_NATION_INDONESIA, KEY_NATION_JAPAN, KEY_NATION_KAMPUCHEA, KEY_NATION_LAOS, KEY_NATION_MACAU, KEY_NATION_MALAYSIA, KEY_NATION_MALDIVES, KEY_NATION_MYANMAR,
    KEY_NATION_NEPAL, KEY_NATION_NORTH_KOREA, KEY_NATION_SINGAPORE, KEY_NATION_SOUTH_KOREA, KEY_NATION_SRI_LANKA, KEY_NATION_TAIWAN, KEY_NATION_THAILAND,
    KEY_NATION_THE_PHILIPPINES, KEY_NATION_VIETNAM, KEY_NATION_PAKISTAN];

/// The index of nation id `n` in a zone (the first equal), as the original's signed-byte loop over the local list.
fn asian_club_champ_zone_slot(zone: &[u32], n: i32) -> Option<usize> {
    zone.iter().position(|&g| g as i32 == n)
}

/// The champion of the league `league_key` (or its runner-up, if the champion is already in place 0 or 16), when the
/// nation `nation_key` has its `nation::LEAGUE_SELECTED` byte set in its [`G_NATION_TABLE`] record; the club is taken
/// as it comes (no null check).
unsafe fn asian_club_champ_league_club(this: usize, nation_key: usize, league_key: usize) -> Option<usize> {
    let n = read_usize(nation_key);
    if read_u8(read_usize(G_NATION_TABLE).wrapping_add(n.wrapping_mul(nation::SIZE)).wrapping_add(nation::LEAGUE_SELECTED)) == 0 {
        return None;
    }
    let league = comp_record_of_key(league_key);
    let list = read_usize(this + 0xb2);
    let c = exe::comp_util_get_latest_placed_club(league, 0); // cqx1: the league champion
    if c == read_usize(list) || c == read_usize(list + 0x40) {
        return Some(exe::comp_util_get_latest_placed_club(league, 1));
    }
    Some(c)
}

/// The 32 Asian Club Championship qualifiers. Rule: two zones of 16 (east: the Far East and South Asia; west: the
/// Middle East and Central Asia); the last winner takes place 0 or 16 by its nation's zone; the J1 champion (or the
/// runner-up, if the champion is already in) and the Korean league champion join the pool when those leagues are
/// selected; then the Asian clubs, sorted by reputation and shuffled, give one place per nation in its zone while
/// places remain.
///
/// `AsiaClubChamp::select_qualified_clubs()`: +0xb6 = 32 and the 32-place list +0xb2 (`exe::crt_calloc`(0x20, 4):
/// push 4; push 0x20; add esp, 8 -> cdecl; out of memory, the error box of line 0x289 and 0). The latest winner of the
/// cup itself (`exe::comp_util_get_latest_placed_club`([+4], 0), cdecl; its nation id is −1 without club or nation)
/// goes to place 0 if its nation is in the east zone (`ASIAN_CLUB_CHAMP_EAST_ZONE_KEYS`), otherwise to 16. In a local
/// list of up to 1,200 clubs: the J1 club (`asian_club_champ_league_club` with [`KEY_NATION_JAPAN`] and
/// [`KEY_CLUB_COMP_JAPANESE_J_LEAGUE_1`]), then the Korean league's ([`KEY_NATION_SOUTH_KOREA`],
/// [`KEY_CLUB_COMP_KOREAN_LEAGUE`]); and every club of the club table (`G_CLUB_COUNT` − 2 × `G_NATION_COUNT` of them)
/// whose nation's continent is [`KEY_CONTINENT_ASIA`], outside places 0 and 16, while the count (a signed word) is below
/// 0x4b0. The list is sorted and shuffled (`exe::comp_util_sort_and_shuffle_top_clubs`(list, n − 1, 40), cdecl). In its
/// order, each club whose nation has not had a place yet takes the first free place of its zone (0..15 east
/// (`ASIAN_CLUB_CHAMP_EAST_ZONE_KEYS`), 16..31 west (`ASIAN_CUP_WINNERS_WEST_ZONE_KEYS`, the same 19 nations)), while
/// the zone's last place (15 or 31) is free; the nation is marked even when the zone is full; a club of no zone is
/// skipped. Returns 1. (Stack frame 0x1594.)
/// Original: `AsiaClubChamp::select_qualified_clubs` (0x40b950, asia_club_champ.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn asia_club_champ_select_qualified_clubs(this: usize) -> u32 {
    write_u32(this + 0xb6, 0x20);
    let east: [u32; 23] = core::array::from_fn(|i| read_usize(ASIAN_CLUB_CHAMP_EAST_ZONE_KEYS[i]) as u32);
    let west: [u32; 19] = core::array::from_fn(|i| read_usize(ASIAN_CUP_WINNERS_WEST_ZONE_KEYS[i]) as u32);
    let list = exe::crt_calloc(0x20, 4);
    write_usize(this + 0xb2, list);
    if list == 0 {
        show_error_at(0x987a40, 0x289);
        return 0;
    }
    let holder = exe::comp_util_get_latest_placed_club(read_usize(this + 4), 0);
    if asian_club_champ_zone_slot(&east, asia_cup_winner_club_nation_id(holder)).is_some() {
        write_usize(list, holder);
    } else {
        write_usize(list + 0x40, holder);
    }
    let mut clubs = [0usize; 0x4b0];
    let mut n = 0usize;
    if let Some(c) = asian_club_champ_league_club(this, KEY_NATION_JAPAN, KEY_CLUB_COMP_JAPANESE_J_LEAGUE_1) {
        covered!("cqx_1 j1 club");
        clubs[0] = c;
        n = 1;
    }
    if let Some(c) = asian_club_champ_league_club(this, KEY_NATION_SOUTH_KOREA, KEY_CLUB_COMP_KOREAN_LEAGUE) {
        clubs[n] = c;
        n += 1;
    }
    let total = (read_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_usize(G_NATION_COUNT) as i32).wrapping_mul(2));
    let asia = read_usize(KEY_CONTINENT_ASIA);
    let mut c = read_usize(G_CLUB_TABLE);
    for _ in 0..total.max(0) {
        let nation = Nation(read_usize(c + club::NATION));
        if !nation.is_null() {
            let continent = nation.continent();
            if continent != 0 && read_usize(continent) == asia && (n as i16) < 0x4b0 && c != read_usize(list) && c != read_usize(list + 0x40) {
                clubs[n] = c;
                n += 1;
            }
        }
        c += club::SIZE;
    }
    covered!("cqx_1 club draw");
    exe::comp_util_sort_and_shuffle_top_clubs(clubs.as_mut_ptr() as usize, (n as u32).wrapping_sub(1), 0x28); // cqx1: 40 picks in the shuffle
    let (mut east_marks, mut west_marks) = ([0u8; 23], [0u8; 19]);
    for &club in clubs.iter().take((n as i16).max(0) as usize) {
        let nation = asia_cup_winner_club_nation_id(club);
        let (mark, first) = match asian_club_champ_zone_slot(&east, nation) {
            Some(i) => (&mut east_marks[i], 0usize),
            None => match asian_club_champ_zone_slot(&west, nation) {
                Some(j) => (&mut west_marks[j], 16usize),
                None => continue,
            },
        };
        if *mark != 0 {
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

// simplification: the list is allocated and the shuffle draws: it cannot run twice. Only the original; the plan:
// returns 1, +0xb6 = 32, the list at +0xb2 and, outside places 0 and 16 (the holder's), each club in its nation's zone
// (east in places 0..15, west in 16..31) and no nation repeated.
unsafe extern "thiscall" fn compare_asia_club_champ_select_qualified_clubs(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    let r = o(this);
    if r == 1 {
        let east: [u32; 23] = core::array::from_fn(|i| read_usize(ASIAN_CLUB_CHAMP_EAST_ZONE_KEYS[i]) as u32);
        let west: [u32; 19] = core::array::from_fn(|i| read_usize(ASIAN_CUP_WINNERS_WEST_ZONE_KEYS[i]) as u32);
        let list = read_usize(this + 0xb2);
        let mut nations: Vec<i32> = Vec::new();
        let mut wrong = 0usize;
        for k in 0..32usize {
            let c = if list != 0 { read_usize(list + k * 4) } else { 0 };
            if c == 0 || k == 0 || k == 16 {
                continue;
            }
            let nation = asia_cup_winner_club_nation_id(c);
            let zone: &[u32] = if k < 16 { &east } else { &west };
            if asian_club_champ_zone_slot(zone, nation).is_none() || nations.contains(&nation) {
                wrong = k;
            }
            nations.push(nation);
        }
        let ok = list != 0 && read_usize(this + 0xb6) == 0x20 && wrong == 0;
        SCOREBOARD.record(ok, || format!("asia_club_champ_select_qualified_clubs({:#x}): list {:#x} +0xb6 {} wrong place {}", this, list, read_usize(this + 0xb6), wrong));
    }
    r
}

/// Stage 0 of the Asian Club Championship: a knockout of the 16 west zone qualifiers (places 16 to 31 of the list
/// +0xb2), on the cup's round table; the same body as the Cup Winners' Cup's (`asia_cup_winner_zone_knockout_stage`),
/// with the error boxes at lines 0x32a (no memory for the teams) and 0x341 (no stage). (The original's SEH frame is only
/// for the stage's new: no destructor on the normal path.)
/// Original: `AsiaClubChamp::create_second_zone_cup_stage` (0x40bec0, asia_club_champ.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn asia_club_champ_create_second_zone_cup_stage(this: usize) {
    asia_cup_winner_zone_knockout_stage(this, 0x987a40, (0x32a, 0x341), "cqx_1 west zone team"); // cqx1: the west zone knockout
}

/// Comparer for `asia_club_champ_create_second_zone_cup_stage`: only the original (it allocates the stage), with the
/// plan of the Cup Winners' Cup's (`asia_cup_winner_check_zone_stage`).
unsafe extern "thiscall" fn compare_asia_club_champ_create_second_zone_cup_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    asia_cup_winner_check_zone_stage("asia_club_champ_create_second_zone_cup_stage", this, || o(this))
}

/// The predicted positions of the two Asian Club Championship zones: in each zone, by reputation, 1, 3, 5, 7, four 9ths
/// and eight 17ths; the same body as the Cup Winners' Cup's (`asia_cup_winner_predicted_positions`), with the error
/// boxes at lines 0x45a (no statistics) and 0x462 (no memory).
/// Original: `AsiaClubChamp::rank_teams` (0x40c720, asia_club_champ.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn asia_club_champ_rank_teams(this: usize) {
    covered!("cqx_1 predicted positions");
    asia_cup_winner_predicted_positions(this, 0x987a40, (0x45a, 0x462)); // cqx1: the predicted positions of the two zones
}

/// The clubs of the two halves (the cup's team table and stage 0's), when both tables are there with 16 clubs.
unsafe fn asian_club_champ_ranked_clubs(this: usize) -> Option<Vec<usize>> {
    let stages = read_usize(this + cup::STAGES);
    if read_i16(this + cup::TEAM_COUNT) != 16 || read_usize(this + 8) == 0 || stages == 0 || read_usize(stages) == 0 || read_usize(this + cup::TEAM_TABLE) == 0
        || read_usize(read_usize(stages) + cup::TEAM_TABLE) == 0 {
        return None;
    }
    let mut v: Vec<usize> = (0..16).map(|k| read_usize(read_usize(this + cup::TEAM_TABLE) + k * 6)).collect();
    v.extend((0..16).map(|k| read_usize(read_usize(read_usize(stages) + cup::TEAM_TABLE) + k * 6)));
    if v.contains(&0) { None } else { Some(v) }
}

/// The predicted position (`comp_stats_club::FINAL_POSITION`) of each club in the cup's statistics, 0 without entry.
unsafe fn asian_club_champ_positions(this: usize, clubs: &[usize]) -> Vec<u8> {
    clubs.iter().map(|&c| {
        let e = exe::comp_stats_find_club_entry(read_usize(this + 8), c);
        if e != 0 { read_u8(e + comp_stats_club::FINAL_POSITION) } else { 0 }
    }).collect()
}

/// Comparer for `asia_club_champ_rank_teams`, side by side over the same entries: the original first (it adds each
/// club's entry to the statistics, Comp_Stats::set_club_final_position), with the plan of the Cup Winners' Cup's
/// (`asia_cup_winner_check_positions`: by reputation 1, 3, 5, 7, four 9ths and eight 17ths in each half); then the
/// copy, which finds the entries already there and only rewrites `comp_stats_club::FINAL_POSITION` (the sort of the
/// same array, the same qsort: the same order); the 32 positions after the copy must equal the original's.
unsafe extern "thiscall" fn compare_asia_club_champ_rank_teams(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    let clubs = asian_club_champ_ranked_clubs(this);
    asia_cup_winner_check_positions("asia_club_champ_rank_teams", this, || o(this));
    if let Some(clubs) = clubs {
        let original = asian_club_champ_positions(this, &clubs);
        asia_club_champ_rank_teams(this);
        let copy = asian_club_champ_positions(this, &clubs);
        SCOREBOARD.record(copy == original, || format!("asia_club_champ_rank_teams({:#x}): copy {:?} original {:?}", this, copy, original));
    }
}
