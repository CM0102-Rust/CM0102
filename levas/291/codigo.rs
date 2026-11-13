// ---------- batch 291: the Asian Club Championship: the 32 qualifiers of the two zones, the west zone stage and the predicted positions (asia_club_champ.cpp) ----------

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
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ASIA_CLUB_CHAMP_SELECT_QUALIFIED_CLUBS);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ASIA_CLUB_CHAMP_CREATE_SECOND_ZONE_CUP_STAGE);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ASIA_CLUB_CHAMP_RANK_TEAMS);
    let clubs = asian_club_champ_ranked_clubs(this);
    asia_cup_winner_check_positions("asia_club_champ_rank_teams", this, || o(this));
    if let Some(clubs) = clubs {
        let original = asian_club_champ_positions(this, &clubs);
        asia_club_champ_rank_teams(this);
        let copy = asian_club_champ_positions(this, &clubs);
        SCOREBOARD.record(copy == original, || format!("asia_club_champ_rank_teams({:#x}): copy {:?} original {:?}", this, copy, original));
    }
}

// ---------- batch 291: the FIFA Club World Championship: the round tables, the rules and the day (world_club_champ.cpp) ----------

/// The World Club Championship: the three rounds of each group (29 July, 2 and 5 August), code 1, value 1, with match
/// 2 empty.
const WORLD_CLUB_CHAMP_GROUP_ROUNDS: [GoldcupScheduleOp; 6] = [
    GoldcupScheduleOp::D(0, 0x1d, 6, 0, 6, 1, 1), GoldcupScheduleOp::L(0, 2, -1, -1, -1, 0), GoldcupScheduleOp::D(1, 2, 7, 0, 3, 1, 1),
    GoldcupScheduleOp::L(1, 2, -1, -1, -1, 0), GoldcupScheduleOp::D(2, 5, 7, 0, 6, 1, 1), GoldcupScheduleOp::L(2, 2, -1, -1, -1, 0),
];
/// The World Club Championship: the matches of the group of stage −1.
const WORLD_CLUB_CHAMP_GROUP_A_MATCHES: [GoldcupScheduleOp; 6] = [
    GoldcupScheduleOp::L(0, 0, 5, 1, 3, 4), GoldcupScheduleOp::L(0, 1, 6, 2, 3, 0x40), GoldcupScheduleOp::L(1, 0, 2, 1, 3, 4),
    GoldcupScheduleOp::L(1, 1, 2, 2, 3, 0x40), GoldcupScheduleOp::L(2, 0, 5, 1, 3, 4), GoldcupScheduleOp::L(2, 1, 5, 2, 3, 0x40),
];
/// The World Club Championship: the matches of the group of stage 0.
const WORLD_CLUB_CHAMP_GROUP_B_MATCHES: [GoldcupScheduleOp; 6] = [
    GoldcupScheduleOp::L(0, 0, 6, 1, 3, 4), GoldcupScheduleOp::L(0, 1, 0, 2, 3, 4), GoldcupScheduleOp::L(1, 0, 3, 1, 3, 0x40),
    GoldcupScheduleOp::L(1, 1, 3, 2, 3, 4), GoldcupScheduleOp::L(2, 0, 6, 1, 3, 4), GoldcupScheduleOp::L(2, 1, 6, 2, 3, 0x40),
];
/// The World Club Championship: the matches of the group of stage 1.
const WORLD_CLUB_CHAMP_GROUP_C_MATCHES: [GoldcupScheduleOp; 6] = [
    GoldcupScheduleOp::L(0, 0, 1, 1, 3, 0x40), GoldcupScheduleOp::L(0, 1, 1, 2, 3, 4), GoldcupScheduleOp::L(1, 0, 4, 1, 3, 0x40),
    GoldcupScheduleOp::L(1, 1, 4, 2, 3, 4), GoldcupScheduleOp::L(2, 0, 0, 1, 3, 4), GoldcupScheduleOp::L(2, 1, 0, 2, 3, 0x40),
];
/// The World Club Championship knockout (stage 2), single matches: the semi-finals drawn on 7 August and played on 9
/// August (tie-break 0x83, category 0x17), the final drawn on 10 August and played on 12 August (prizes 6,000,000 and
/// 5,000,000) and the third-place match on 12 August (prizes 4,000,000 and 3,000,000).
const WORLD_CLUB_CHAMP_KNOCKOUT: [GoldcupScheduleOp; 54] = [
    GoldcupScheduleOp::S(0, 7, 7, 0, 1), GoldcupScheduleOp::J(0, 9, 7, 0, 3, 1, 0x40), GoldcupScheduleOp::W(0x7, 0x82),
    GoldcupScheduleOp::W(0x9, 0), GoldcupScheduleOp::W(0xb, 0), GoldcupScheduleOp::W(0xd, 0x83), GoldcupScheduleOp::B(0x17, 0x17),
    GoldcupScheduleOp::W(0x18, 4), GoldcupScheduleOp::W(0x1a, 2), GoldcupScheduleOp::W(0x1c, 4), GoldcupScheduleOp::W(0xf, 0),
    GoldcupScheduleOp::W(0x1e, 0), GoldcupScheduleOp::B(0x20, 0), GoldcupScheduleOp::B(0x21, 1), GoldcupScheduleOp::B(0x22, 0),
    GoldcupScheduleOp::X(0x5c, 0), GoldcupScheduleOp::X(0x60, 0), GoldcupScheduleOp::X(0x64, 0),
    GoldcupScheduleOp::S(1, 0xa, 7, 0, 4), GoldcupScheduleOp::J(1, 0xc, 7, 0, 6, 2, 4), GoldcupScheduleOp::W(0x6f, 0x96),
    GoldcupScheduleOp::W(0x71, 1), GoldcupScheduleOp::W(0x73, 0), GoldcupScheduleOp::W(0x75, 0x83), GoldcupScheduleOp::B(0x7f, 0x17),
    GoldcupScheduleOp::W(0x80, 2), GoldcupScheduleOp::W(0x82, 1), GoldcupScheduleOp::W(0x84, 0), GoldcupScheduleOp::W(0x77, 0),
    GoldcupScheduleOp::W(0x86, 0), GoldcupScheduleOp::B(0x88, 0), GoldcupScheduleOp::B(0x89, 1), GoldcupScheduleOp::B(0x8a, 0),
    GoldcupScheduleOp::X(0xc4, 0), GoldcupScheduleOp::X(0xc8, 0x5b8d80), GoldcupScheduleOp::X(0xcc, 0x4c4b40), // cqx2: the final prizes
    GoldcupScheduleOp::S(2, 0xa, 7, 0, 4), GoldcupScheduleOp::J(2, 0xc, 7, 0, 6, 1, 0x40), GoldcupScheduleOp::W(0xd9, 2),
    GoldcupScheduleOp::W(0xe8, 2), GoldcupScheduleOp::W(0xd7, 0x8c), GoldcupScheduleOp::W(0xdb, 0), GoldcupScheduleOp::W(0xdd, 0x83),
    GoldcupScheduleOp::B(0xe7, 0x17), GoldcupScheduleOp::W(0xea, 1), GoldcupScheduleOp::W(0xec, 0), GoldcupScheduleOp::W(0xdf, 0),
    GoldcupScheduleOp::W(0xee, 0), GoldcupScheduleOp::B(0xf0, 0), GoldcupScheduleOp::B(0xf1, 1), GoldcupScheduleOp::B(0xf2, 0),
    GoldcupScheduleOp::X(0x12c, 0), GoldcupScheduleOp::X(0x130, 0x3d0900), GoldcupScheduleOp::X(0x134, 0x2dc6c0),
];

/// The World Club Championship stage description by the signed stage byte (the three groups below 2, the knockout at
/// 2, nothing above).
fn world_club_champ_phase(stage: u32) -> Option<GoldcupStagePlan> {
    let f = stage as u8 as i8;
    if f < 2 {
        let games: &'static [GoldcupScheduleOp] = match f {
            -1 => &WORLD_CLUB_CHAMP_GROUP_A_MATCHES,
            0 => &WORLD_CLUB_CHAMP_GROUP_B_MATCHES,
            1 => &WORLD_CLUB_CHAMP_GROUP_C_MATCHES,
            _ => &[],
        };
        Some(GoldcupStagePlan { league: 1, rounds: 3, extra: (f as i16).wrapping_add(0x3e9), extra_check: false, size: 0xc3, row: 0x99,
            ops: &WORLD_CLUB_CHAMP_GROUP_ROUNDS, ops2: games, name: "cqx_2 world club championship groups" })
    } else if f == 2 {
        Some(GoldcupStagePlan { league: 0, rounds: 3, extra: 0, extra_check: false, size: 0x138, row: 0xd3,
            ops: &WORLD_CLUB_CHAMP_KNOCKOUT, ops2: &[], name: "cqx_2 world club championship knockout" })
    } else {
        None
    }
}

/// Rule: the World Club Championship has three groups of three rounds, single round robin, on 29 July, 2 and 5 August
/// (stages −1, 0 and 1, ids 0x3e8 to 0x3ea), each round with the group's two fixed matches; then the knockout (stage 2)
/// in single matches: semi-finals on 9 August, final and third place on 12 August; category 0x17 (club world cups);
/// prizes 6 and 5 million in the final, 4 and 3 million for the third place.
///
/// `WorldClubChamp::create_schedule(stage, &rounds, &extra, &is_league)` (virtual `comp_vtable::CREATE_SCHEDULE`). Stage
/// below 2 (signed byte): is_league = 1 (if asked), 3 rounds, extra = stage + 0x3e9; the table (`exe::crt_malloc`(0xc3);
/// out of memory, the error box at line 0x99 and 0) with the three round dates (`exe::league_round_set_date`, 9
/// arguments, cdecl; the year +0x40 as a word) and match 2 of each round empty (`exe::league_round_set_match_slot`, 7
/// arguments, cdecl), then the group's six matches. Stage 2: is_league = 0, 3 rounds, extra = 0; the table (0x138; the
/// box at line 0xd3) with the draw dates (`exe::cup_set_round_draw_date`, 7 arguments) and match dates
/// (`exe::cup_set_round_match_date`, 9 arguments; the year signed), and the fields of each round record (0x68 bytes).
/// Above 2: 0 with nothing written. (`build_phase_table`, the engine of the national-team tournaments.)
/// Original: `WorldClubChamp::create_schedule` (0x929390, world_club_champ.cpp, thiscall, ret 0x10).
unsafe extern "thiscall" fn world_club_champ_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    build_phase_table(League(this), world_club_champ_phase(stage), rounds, extra, league, 0xad5764)
}

/// Comparer for `world_club_champ_create_schedule`: side by side (`compare_phase_table`).
unsafe extern "thiscall" fn compare_world_club_champ_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_WORLD_CLUB_CHAMP_CREATE_SCHEDULE);
    compare_phase_table("world_club_champ_create_schedule", stage, world_club_champ_phase(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| world_club_champ_create_schedule(this, stage, r, e, l))
}

/// The World Club Championship rule fields: 3 points a win and 1 a draw, tie-breaks 1 and 2 [inference: goal
/// difference, goals for], two qualifiers per group, category 0x17, 7 substitutes and 3 changes, one meeting
/// (`league::LEGS` = 1), one promoted (the group winner) and the rule bits 0x28.
const WORLD_CLUB_CHAMP_RULE_FIELDS: [(usize, u8, u32); 15] = [(0xc2, 1, 3), (0x4a, 1, 3), (0x43, 1, 2), (0xc6, 1, 2), (0x3c, 2, 1), (0xc3, 1, 1), (0xc4, 1, 0x17),
    (0xc5, 1, 1), (0xc7, 1, 4), (0xbe, 1, 1), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 0), (0x49, 1, 7), (0xd9, 2, 0x28)];

/// Rule: the World Club Championship groups are single round robin, 3 points a win and 1 a draw, category 0x17; the
/// draw is on 5 February of the competition year (moved to weekday 5), the competition year being the first even year
/// from 2003 on (the constructor 0x9291a2), so it never comes before June 2002.
///
/// `WorldClubChamp::set_up_rules()` (virtual `comp_vtable::SET_UP_RULES`): the rule fields
/// (`WORLD_CLUB_CHAMP_RULE_FIELDS`, `goldcup_write_rule_fields`), the round table of the first group (virtual
/// `comp_vtable::CREATE_SCHEDULE`(−1, &`league::ROUND_COUNT`, &`league::STAGE_ID`, 0): push 0; push; push; push −1; mov
/// ecx -> thiscall, ret 0x10) in `league::ROUND_DATES`, then the draw date (`competition_date_word`(5, 1, 0, 5):
/// Date::Date 0x549f70, thiscall, ret 0x10) in `league::NEXT_DRAW_DAY` and `league::NEXT_DRAW_YEAR_OFFSET` = 0
/// (unlike the Copa América's, the table comes before the date). Returns 1.
/// Original: `WorldClubChamp::set_up_rules` (0x929bb0, world_club_champ.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn world_club_champ_set_up_rules(this: usize) -> u32 {
    covered!("cqx_2 world club championship rules");
    goldcup_write_rule_fields(this, &WORLD_CLUB_CHAMP_RULE_FIELDS, 3);
    let l = League(this);
    let table: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    l.set_round_dates(table(this, 0xffff_ffff, this + league::ROUND_COUNT, this + league::STAGE_ID, 0));
    let day = 5; // cqx2: the draw day
    l.set_next_draw_day(competition_date_word(l, day, 1, 0, 5) as i16);
    l.set_next_draw_year_offset(0);
    1
}

/// Comparer for `world_club_champ_set_up_rules`, side by side over the same object: the original first, with the plan
/// (`check_set_up_rules`: the fields, the date and return 1); then the copy, whose rule fields, date and round table
/// (the bytes the first group's operations write) must equal the original's; the copy's table is freed and the
/// original's put back in `league::ROUND_DATES`.
unsafe extern "thiscall" fn compare_world_club_champ_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WORLD_CLUB_CHAMP_SET_UP_RULES);
    let r = check_set_up_rules("world_club_champ_set_up_rules", this, &WORLD_CLUB_CHAMP_RULE_FIELDS, (5, 1, 0, 5), 0, || o(this));
    let l = League(this);
    let t = l.round_dates();
    if r != 1 || t == 0 {
        return r;
    }
    let before: Vec<u8> = (0x3au32..0xdb).filter(|&k| !(0xba..0xbe).contains(&k)).map(|k| read_u8(this + k as usize)).collect();
    let r2 = world_club_champ_set_up_rules(this);
    let t2 = l.round_dates();
    let after: Vec<u8> = (0x3au32..0xdb).filter(|&k| !(0xba..0xbe).contains(&k)).map(|k| read_u8(this + k as usize)).collect();
    let tables = t2 != 0 && phase_table_written_bytes(&WORLD_CLUB_CHAMP_GROUP_ROUNDS, &WORLD_CLUB_CHAMP_GROUP_A_MATCHES, t) == phase_table_written_bytes(&WORLD_CLUB_CHAMP_GROUP_ROUNDS, &WORLD_CLUB_CHAMP_GROUP_A_MATCHES, t2);
    if t2 != 0 && t2 != t {
        exe::crt_free(t2);
    }
    l.set_round_dates(t);
    SCOREBOARD.record(r2 == r && before == after && tables, || format!("world_club_champ_set_up_rules({:#x}): copy returned {}, fields equal {}, table equal {}", this, r2, before == after, tables));
    r
}

/// Rule: the World Club Championship's day: before the draw date nothing (not even the league day); on the draw day (5
/// February of the competition year, from 2003 on) the entrants, the predicted positions, the first group's table, the
/// fixtures, the order of the table and the other groups' stages are made once; then the league day.
///
/// `WorldClubChamp::update_for_date(today, full)` (virtual +0x14; the SEH frame only covers the local dates, no
/// destructor). The draw date is that of Date::Date 0x549ef0(`league::NEXT_DRAW_DAY`, `league::NEXT_DRAW_YEAR_OFFSET` +
/// `league::YEAR`) (lea ecx, local; push year; push day -> thiscall, ret 8: the day of the year and the year, 8 bytes).
/// Before it (today's year lower, unsigned; or the same year and today's day lower, signed) it returns 0. On the draw
/// day, if the draw is not done (`league::ACTIVE` = 0) and WITHOUT `full`: the entrants (WorldClubChamp 0x92a8b0, mov
/// ecx -> thiscall, ret 0), the predicted positions (virtual `comp_vtable::SET_PREDICTED_POSITIONS`, thiscall, ret 0),
/// the first group's table (WorldClubChamp 0x929aa0, thiscall, ret 0; 0: the error box at line 0x3c7), the fixtures
/// (`exe::league_create_fixtures`; 0: the box at line 0x3ce), the sorted table (`exe::league_sort_table`(this, 0); 0:
/// the box at line 0x3d5) (an error returns 0 without the league day), the other groups' stages (WorldClubChamp
/// 0x929e60, thiscall, ret 0), the drawn list (+0xee) freed (`exe::crt_free`) and zeroed, +0xf2 = 0 and
/// `league::ACTIVE` = 1. Then the league day (`exe::league_update_for_date`(this, today, full): push full; push today;
/// mov ecx -> thiscall, ret 8), whose return value it passes on. (In the test career the draw date is in 2003: only
/// the early return runs.)
/// Original: `WorldClubChamp::update_for_date` (0x92a6b0, world_club_champ.cpp, thiscall, ret 8).
unsafe extern "thiscall" fn world_club_champ_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let l = League(this);
    let day = l.next_draw_day() as u16;
    let year = (l.next_draw_year_offset() as u16).wrapping_add(l.year() as u16);
    let day_of_today = read_u16(today);
    let year_of_today = read_u16(today + 2);
    // simplification: on the early return and on errors the original returns only al = 0 (the rest of eax is leftover);
    // the caller reads the byte
    if year_of_today < year || (year_of_today == year && (day_of_today as i16) < day as i16) {
        covered!("cqx_2 before the draw");
        return 0; // cqx2: nothing before the draw date
    }
    if year_of_today == year && day_of_today == day && l.active() == 0 && full == 0 {
        covered!("cqx_2 world club championship draw");
        let routine = |a: usize| -> unsafe extern "thiscall" fn(usize) -> i32 { core::mem::transmute(a) };
        routine(0x92a8b0)(this); // WorldClubChamp: the entrants
        let predicted: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::SET_PREDICTED_POSITIONS));
        predicted(this);
        if routine(0x929aa0)(this) == 0 {
            show_error_at(0xad5764, 0x3c7);
            return 0;
        }
        if exe::league_create_fixtures(l) == 0 {
            show_error_at(0xad5764, 0x3ce);
            return 0;
        }
        if exe::league_sort_table(l, 0) == 0 {
            show_error_at(0xad5764, 0x3d5);
            return 0;
        }
        routine(0x929e60)(this); // WorldClubChamp: the other groups' stages
        exe::crt_free(read_usize(this + 0xee));
        write_usize(this + 0xee, 0);
        write_usize(this + 0xf2, 0);
        l.set_active(1);
    }
    exe::league_update_for_date(l, today, full) as u32
}

/// Comparer for `world_club_champ_update_for_date`. Before the draw date (the plan's condition) the routine only reads:
/// side by side, the original and then the copy, and the returned bytes must be equal. From the date on, only the
/// original (the draw allocates and draws; the league day plays), with the plan on the draw day: `league::ACTIVE` = 1,
/// the list +0xee and +0xf2 zeroed and the table made.
unsafe extern "thiscall" fn compare_world_club_champ_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_WORLD_CLUB_CHAMP_UPDATE_FOR_DATE);
    let l = League(this);
    let day = l.next_draw_day() as u16;
    let year = (l.next_draw_year_offset() as u16).wrapping_add(l.year() as u16);
    let (dt, yt) = (read_u16(today), read_u16(today + 2));
    if yt < year || (yt == year && (dt as i16) < day as i16) {
        let r = o(this, today, full);
        let c = world_club_champ_update_for_date(this, today, full);
        SCOREBOARD.record(r as u8 == c as u8, || format!("world_club_champ_update_for_date({:#x}): before the draw original {} copy {}", this, r as u8, c as u8));
        return r;
    }
    let draws = yt == year && dt == day && l.active() == 0 && full == 0;
    let r = o(this, today, full);
    if draws {
        let ok = l.active() == 1 && read_usize(this + 0xee) == 0 && read_usize(this + 0xf2) == 0 && l.table() != 0;
        SCOREBOARD.record(ok, || format!("world_club_champ_update_for_date({:#x}): the draw (+0x45 {}, +0xee {:#x})", this, l.active(), read_usize(this + 0xee)));
    }
    r
}

// ---------- batch 291: the predicted positions of the state leagues: Bahia state league, Centre-West league, Rio Grande do Sul state league (bra_reg_bahia.cpp, bra_reg_central.cpp, bra_reg_gaucho.cpp) ----------

/// One Brazilian state league's predicted positions: the source file of the error boxes, their two lines (no
/// statistics, no memory), the fixed places of the first clubs in the order (the knockout) and the counter.
struct StateLeaguePredicted {
    file: usize,
    lines: [u32; 2],
    places: &'static [u32],
    label: &'static str,
}

/// The predicted positions of a Brazilian state league (the same mould of 336 to 432 bytes in the twelve, checked
/// instruction by instruction: only the source file, the box lines and the fixed knockout places change; the
/// quarter-final ones (8 places) run the tail with an index and a position, the others with the position only: the
/// same clubs and places).
///
/// `set_predicted_positions()` (virtual `comp_vtable::SET_PREDICTED_POSITIONS`, from the constructor and the season
/// turnover). Without statistics (`league::COMP_STATS`): the box of the first line and return. The clubs of the
/// `league::CLUB_COUNT` rows of the table (`league::TABLE`, `league_table_row::SIZE` bytes each) go to an array
/// (`exe::crt_malloc`(4 × n): push; add esp, 4 -> cdecl; out of memory: the box of the second line and return),
/// sorted by strength (`exe::comp_util_sort_clubs_by_strength`(array, n): push n; push array; add esp, 8 -> cdecl, the
/// strongest first); the first ones get the fixed places without looking at how many clubs there are, and the next
/// ones, while there is a club (k up to n), place k (Comp_Stats::set_club_final_position
/// `exe::comp_stats_set_club_final_position`(stats, club, place): push place; push club; mov ecx, stats -> thiscall,
/// ret 8); the array is freed (`exe::crt_free`).
unsafe fn state_league_predicted_positions(this: usize, s: &StateLeaguePredicted) {
    let l = League(this);
    if l.comp_stats() == 0 {
        return show_error_at(s.file, s.lines[0]);
    }
    let v = exe::crt_malloc(((l.club_count() as i32) << 2) as u32);
    if v == 0 {
        return show_error_at(s.file, s.lines[1]);
    }
    let mut i = 0i16;
    while i < l.club_count() {
        write_usize(v + 4 * i as usize, read_usize(l.table() + league_table_row::SIZE * i as usize));
        i += 1;
    }
    exe::comp_util_sort_clubs_by_strength(v, l.club_count() as i32); // cqx3: the order by strength
    for (k, &p) in s.places.iter().enumerate() {
        exe::comp_stats_set_club_final_position(l.comp_stats(), read_usize(v + 4 * k), p);
    }
    covered!(s.label);
    let mut k = s.places.len() as i32 + 1;
    while k <= l.club_count() as i32 {
        exe::comp_stats_set_club_final_position(l.comp_stats(), read_usize(v + 4 * (k - 1) as usize), k as u32);
        k += 1;
    }
    exe::crt_free(v);
}

/// The predicted position (`comp_stats_club::FINAL_POSITION`) of each club in the statistics `stats`, 0 without
/// entry (Comp_Stats::find_club_entry `exe::comp_stats_find_club_entry`).
unsafe fn state_league_positions(stats: usize, clubs: &[usize]) -> Vec<u8> {
    clubs.iter().map(|&c| {
        let e = if c != 0 { exe::comp_stats_find_club_entry(stats, c) } else { 0 };
        if e != 0 { read_u8(e + comp_stats_club::FINAL_POSITION) } else { 0 }
    }).collect()
}

/// The comparer of the state leagues' predicted positions, side by side over the same entries: the original first
/// (Comp_Stats::set_club_final_position adds the entry of a club that has none), then the copy, which finds the entries
/// already there and only rewrites `comp_stats_club::FINAL_POSITION` (the same table, the same sort by strength: the
/// same order, ties included); the position of each club of the table after the copy must equal the original's. With
/// fewer clubs than fixed places (the original reads past the array), without statistics or table: only the original.
unsafe fn compare_state_league_predicted(name: &str, this: usize, s: &StateLeaguePredicted, o: impl FnOnce()) {
    let l = League(this);
    let n = l.club_count() as i32;
    if l.comp_stats() == 0 || l.table() == 0 || n < s.places.len() as i32 {
        return o();
    }
    let clubs: Vec<usize> = (0..n as usize).map(|i| read_usize(l.table() + league_table_row::SIZE * i)).collect();
    o();
    let original = state_league_positions(l.comp_stats(), &clubs);
    state_league_predicted_positions(this, s);
    let copy = state_league_positions(l.comp_stats(), &clubs);
    SCOREBOARD.record(copy == original, || format!("{}({:#x}): copy {:?} original {:?}", name, this, copy, original));
}

/// The Bahia state league: the fixed places 1, 2, 3, 3 (the semi-finals).
const BAHIA_PREDICTED: StateLeaguePredicted = StateLeaguePredicted { file: 0x98acac, lines: [0x307, 0x30f], places: &[1, 2, 3, 3], label: "cqx_3 bahia predicted" }; // cqx3: the bahia places

/// The predicted positions of the Bahia state league (at creation and at the season turnover): the clubs sorted by strength; the
/// first ones get the knockout places, 1, 2, 3, 3, and the others their place in the order (`state_league_predicted_positions`
/// with `BAHIA_PREDICTED`); it is the board and press expectation for each club [inference]. The error boxes at
/// lines 0x307 (no statistics) and 0x30f (no memory).
/// Original: `BraRegBahia::set_predicted_positions` (0x430340, bra_reg_bahia.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn bra_reg_bahia_set_predicted_positions(this: usize) {
    state_league_predicted_positions(this, &BAHIA_PREDICTED)
}

/// Comparer for `bra_reg_bahia_set_predicted_positions` (`compare_state_league_predicted`, side by side).
unsafe extern "thiscall" fn compare_bra_reg_bahia_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_REG_BAHIA_SET_PREDICTED_POSITIONS);
    compare_state_league_predicted("bra_reg_bahia_set_predicted_positions", this, &BAHIA_PREDICTED, || o(this));
}

/// The Centre-West league: the fixed places 1, 2, 3, 3, 5, 5, 5, 5 (the quarter-finals).
const CENTRAL_PREDICTED: StateLeaguePredicted = StateLeaguePredicted { file: 0x98ae10, lines: [0x30c, 0x314], places: &[1, 2, 3, 3, 5, 5, 5, 5], label: "cqx_3 centre-west predicted" }; // cqx3: the centre-west places

/// The predicted positions of the Centre-West league (at creation and at the season turnover): the clubs sorted by strength; the
/// first ones get the knockout places, 1, 2, 3, 3, 5, 5, 5, 5, and the others their place in the order (`state_league_predicted_positions`
/// with `CENTRAL_PREDICTED`); it is the board and press expectation for each club [inference]. The error boxes at
/// lines 0x30c (no statistics) and 0x314 (no memory).
/// Original: `BraRegCentral::set_predicted_positions` (0x431610, bra_reg_central.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn bra_reg_central_set_predicted_positions(this: usize) {
    state_league_predicted_positions(this, &CENTRAL_PREDICTED)
}

/// Comparer for `bra_reg_central_set_predicted_positions` (`compare_state_league_predicted`, side by side).
unsafe extern "thiscall" fn compare_bra_reg_central_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_REG_CENTRAL_SET_PREDICTED_POSITIONS);
    compare_state_league_predicted("bra_reg_central_set_predicted_positions", this, &CENTRAL_PREDICTED, || o(this));
}

/// The Rio Grande do Sul state league: the fixed places 1, 2, 3, 3 (the semi-finals).
const GAUCHO_PREDICTED: StateLeaguePredicted = StateLeaguePredicted { file: 0x98ae50, lines: [0x303, 0x30b], places: &[1, 2, 3, 3], label: "cqx_3 rio grande do sul predicted" }; // cqx3: the rio grande do sul places

/// The predicted positions of the Rio Grande do Sul state league (at creation and at the season turnover): the clubs sorted by strength; the
/// first ones get the knockout places, 1, 2, 3, 3, and the others their place in the order (`state_league_predicted_positions`
/// with `GAUCHO_PREDICTED`); it is the board and press expectation for each club [inference]. The error boxes at
/// lines 0x303 (no statistics) and 0x30b (no memory).
/// Original: `BraRegGaucho::set_predicted_positions` (0x432980, bra_reg_gaucho.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn bra_reg_gaucho_set_predicted_positions(this: usize) {
    state_league_predicted_positions(this, &GAUCHO_PREDICTED)
}

/// Comparer for `bra_reg_gaucho_set_predicted_positions` (`compare_state_league_predicted`, side by side).
unsafe extern "thiscall" fn compare_bra_reg_gaucho_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_REG_GAUCHO_SET_PREDICTED_POSITIONS);
    compare_state_league_predicted("bra_reg_gaucho_set_predicted_positions", this, &GAUCHO_PREDICTED, || o(this));
}

// ---------- batch 291: the predicted positions of the state leagues: Goiás state league, Minas Gerais state league, North league (bra_reg_goias.cpp, bra_reg_minas_gerais.cpp, bra_reg_north.cpp) ----------

/// The Goiás state league: the fixed places 1, 2, 3, 3, 5, 5, 5, 5 (the quarter-finals).
const GOIAS_PREDICTED: StateLeaguePredicted = StateLeaguePredicted { file: 0x98ae90, lines: [0x304, 0x30c], places: &[1, 2, 3, 3, 5, 5, 5, 5], label: "cqx_4 goiás predicted" }; // cqx4: the goiás places

/// The predicted positions of the Goiás state league (at creation and at the season turnover): the clubs sorted by strength; the
/// first ones get the knockout places, 1, 2, 3, 3, 5, 5, 5, 5, and the others their place in the order (`state_league_predicted_positions`
/// with `GOIAS_PREDICTED`); it is the board and press expectation for each club [inference]. The error boxes at
/// lines 0x304 (no statistics) and 0x30c (no memory).
/// Original: `BraRegGoias::set_predicted_positions` (0x433b40, bra_reg_goias.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn bra_reg_goias_set_predicted_positions(this: usize) {
    state_league_predicted_positions(this, &GOIAS_PREDICTED)
}

/// Comparer for `bra_reg_goias_set_predicted_positions` (`compare_state_league_predicted`, side by side).
unsafe extern "thiscall" fn compare_bra_reg_goias_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_REG_GOIAS_SET_PREDICTED_POSITIONS);
    compare_state_league_predicted("bra_reg_goias_set_predicted_positions", this, &GOIAS_PREDICTED, || o(this));
}

/// The Minas Gerais state league: the fixed places 1, 2 (the final between the top two).
const MINAS_GERAIS_PREDICTED: StateLeaguePredicted = StateLeaguePredicted { file: 0x98aed0, lines: [0x33f, 0x347], places: &[1, 2], label: "cqx_4 minas gerais predicted" }; // cqx4: the minas gerais places

/// The predicted positions of the Minas Gerais state league (at creation and at the season turnover): the clubs sorted by strength; the
/// first ones get the knockout places, 1, 2, and the others their place in the order (`state_league_predicted_positions`
/// with `MINAS_GERAIS_PREDICTED`); it is the board and press expectation for each club [inference]. The error boxes at
/// lines 0x33f (no statistics) and 0x347 (no memory).
/// Original: `BraRegMinasGerais::set_predicted_positions` (0x434f80, bra_reg_minas_gerais.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn bra_reg_minas_gerais_set_predicted_positions(this: usize) {
    state_league_predicted_positions(this, &MINAS_GERAIS_PREDICTED)
}

/// Comparer for `bra_reg_minas_gerais_set_predicted_positions` (`compare_state_league_predicted`, side by side).
unsafe extern "thiscall" fn compare_bra_reg_minas_gerais_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_REG_MINAS_GERAIS_SET_PREDICTED_POSITIONS);
    compare_state_league_predicted("bra_reg_minas_gerais_set_predicted_positions", this, &MINAS_GERAIS_PREDICTED, || o(this));
}

/// The North league: the fixed places 1, 2, 3, 3 (the semi-finals).
const NORTH_PREDICTED: StateLeaguePredicted = StateLeaguePredicted { file: 0x98af14, lines: [0x307, 0x30f], places: &[1, 2, 3, 3], label: "cqx_4 north predicted" }; // cqx4: the north places

/// The predicted positions of the North league (at creation and at the season turnover): the clubs sorted by strength; the
/// first ones get the knockout places, 1, 2, 3, 3, and the others their place in the order (`state_league_predicted_positions`
/// with `NORTH_PREDICTED`); it is the board and press expectation for each club [inference]. The error boxes at
/// lines 0x307 (no statistics) and 0x30f (no memory).
/// Original: `BraRegNorth::set_predicted_positions` (0x436280, bra_reg_north.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn bra_reg_north_set_predicted_positions(this: usize) {
    state_league_predicted_positions(this, &NORTH_PREDICTED)
}

/// Comparer for `bra_reg_north_set_predicted_positions` (`compare_state_league_predicted`, side by side).
unsafe extern "thiscall" fn compare_bra_reg_north_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_REG_NORTH_SET_PREDICTED_POSITIONS);
    compare_state_league_predicted("bra_reg_north_set_predicted_positions", this, &NORTH_PREDICTED, || o(this));
}

// ---------- batch 291: the predicted positions of the state leagues: Northeast league, Paraná state league, Pernambuco state league (bra_reg_northeast.cpp, bra_reg_parana.cpp, bra_reg_pern.cpp) ----------

/// The Northeast league: the fixed places 1, 2, 3, 3 (the semi-finals).
const NORTHEAST_PREDICTED: StateLeaguePredicted = StateLeaguePredicted { file: 0x98af54, lines: [0x2f9, 0x301], places: &[1, 2, 3, 3], label: "cqx_5 northeast predicted" }; // cqx5: the northeast places

/// The predicted positions of the Northeast league (at creation and at the season turnover): the clubs sorted by strength; the
/// first ones get the knockout places, 1, 2, 3, 3, and the others their place in the order (`state_league_predicted_positions`
/// with `NORTHEAST_PREDICTED`); it is the board and press expectation for each club [inference]. The error boxes at
/// lines 0x2f9 (no statistics) and 0x301 (no memory).
/// Original: `BraRegNortheast::set_predicted_positions` (0x437290, bra_reg_northeast.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn bra_reg_northeast_set_predicted_positions(this: usize) {
    state_league_predicted_positions(this, &NORTHEAST_PREDICTED)
}

/// Comparer for `bra_reg_northeast_set_predicted_positions` (`compare_state_league_predicted`, side by side).
unsafe extern "thiscall" fn compare_bra_reg_northeast_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_REG_NORTHEAST_SET_PREDICTED_POSITIONS);
    compare_state_league_predicted("bra_reg_northeast_set_predicted_positions", this, &NORTHEAST_PREDICTED, || o(this));
}

/// The Paraná state league: the fixed places 1, 2 (the final between the top two).
const PARANA_PREDICTED: StateLeaguePredicted = StateLeaguePredicted { file: 0x98af98, lines: [0x33f, 0x347], places: &[1, 2], label: "cqx_5 paraná predicted" }; // cqx5: the paraná places

/// The predicted positions of the Paraná state league (at creation and at the season turnover): the clubs sorted by strength; the
/// first ones get the knockout places, 1, 2, and the others their place in the order (`state_league_predicted_positions`
/// with `PARANA_PREDICTED`); it is the board and press expectation for each club [inference]. The error boxes at
/// lines 0x33f (no statistics) and 0x347 (no memory).
/// Original: `BraRegParana::set_predicted_positions` (0x438590, bra_reg_parana.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn bra_reg_parana_set_predicted_positions(this: usize) {
    state_league_predicted_positions(this, &PARANA_PREDICTED)
}

/// Comparer for `bra_reg_parana_set_predicted_positions` (`compare_state_league_predicted`, side by side).
unsafe extern "thiscall" fn compare_bra_reg_parana_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_REG_PARANA_SET_PREDICTED_POSITIONS);
    compare_state_league_predicted("bra_reg_parana_set_predicted_positions", this, &PARANA_PREDICTED, || o(this));
}

/// The Pernambuco state league: the fixed places 1, 2, 3, 3 (the semi-finals).
const PERNAMBUCO_PREDICTED: StateLeaguePredicted = StateLeaguePredicted { file: 0x98afd8, lines: [0x307, 0x30f], places: &[1, 2, 3, 3], label: "cqx_5 pernambuco predicted" }; // cqx5: the pernambuco places

/// The predicted positions of the Pernambuco state league (at creation and at the season turnover): the clubs sorted by strength; the
/// first ones get the knockout places, 1, 2, 3, 3, and the others their place in the order (`state_league_predicted_positions`
/// with `PERNAMBUCO_PREDICTED`); it is the board and press expectation for each club [inference]. The error boxes at
/// lines 0x307 (no statistics) and 0x30f (no memory).
/// Original: `BraRegPern::set_predicted_positions` (0x4398d0, bra_reg_pern.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn bra_reg_pern_set_predicted_positions(this: usize) {
    state_league_predicted_positions(this, &PERNAMBUCO_PREDICTED)
}

/// Comparer for `bra_reg_pern_set_predicted_positions` (`compare_state_league_predicted`, side by side).
unsafe extern "thiscall" fn compare_bra_reg_pern_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_REG_PERN_SET_PREDICTED_POSITIONS);
    compare_state_league_predicted("bra_reg_pern_set_predicted_positions", this, &PERNAMBUCO_PREDICTED, || o(this));
}

// ---------- batch 291: the predicted positions of the state leagues: Rio state league, Santa Catarina state league, São Paulo state league (bra_reg_rio.cpp, bra_reg_santa.cpp, bra_reg_sp.cpp) ----------

/// The Rio state league: the fixed places 1, 2 (the final between the top two).
const RIO_PREDICTED: StateLeaguePredicted = StateLeaguePredicted { file: 0x98b014, lines: [0x33f, 0x347], places: &[1, 2], label: "cqx_6 rio predicted" }; // cqx6: the rio places

/// The predicted positions of the Rio state league (at creation and at the season turnover): the clubs sorted by strength; the
/// first ones get the knockout places, 1, 2, and the others their place in the order (`state_league_predicted_positions`
/// with `RIO_PREDICTED`); it is the board and press expectation for each club [inference]. The error boxes at
/// lines 0x33f (no statistics) and 0x347 (no memory).
/// Original: `BraRegRio::set_predicted_positions` (0x43add0, bra_reg_rio.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn bra_reg_rio_set_predicted_positions(this: usize) {
    state_league_predicted_positions(this, &RIO_PREDICTED)
}

/// Comparer for `bra_reg_rio_set_predicted_positions` (`compare_state_league_predicted`, side by side).
unsafe extern "thiscall" fn compare_bra_reg_rio_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_REG_RIO_SET_PREDICTED_POSITIONS);
    compare_state_league_predicted("bra_reg_rio_set_predicted_positions", this, &RIO_PREDICTED, || o(this));
}

/// The Santa Catarina state league: the fixed places 1, 2, 3, 3 (the semi-finals).
const SANTA_CATARINA_PREDICTED: StateLeaguePredicted = StateLeaguePredicted { file: 0x98b050, lines: [0x307, 0x30f], places: &[1, 2, 3, 3], label: "cqx_6 santa catarina predicted" }; // cqx6: the santa catarina places

/// The predicted positions of the Santa Catarina state league (at creation and at the season turnover): the clubs sorted by strength; the
/// first ones get the knockout places, 1, 2, 3, 3, and the others their place in the order (`state_league_predicted_positions`
/// with `SANTA_CATARINA_PREDICTED`); it is the board and press expectation for each club [inference]. The error boxes at
/// lines 0x307 (no statistics) and 0x30f (no memory).
/// Original: `BraRegSanta::set_predicted_positions` (0x43c110, bra_reg_santa.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn bra_reg_santa_set_predicted_positions(this: usize) {
    state_league_predicted_positions(this, &SANTA_CATARINA_PREDICTED)
}

/// Comparer for `bra_reg_santa_set_predicted_positions` (`compare_state_league_predicted`, side by side).
unsafe extern "thiscall" fn compare_bra_reg_santa_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_REG_SANTA_SET_PREDICTED_POSITIONS);
    compare_state_league_predicted("bra_reg_santa_set_predicted_positions", this, &SANTA_CATARINA_PREDICTED, || o(this));
}

/// The São Paulo state league: the fixed places 1, 2, 3, 3 (the semi-finals).
const SAO_PAULO_PREDICTED: StateLeaguePredicted = StateLeaguePredicted { file: 0x98b090, lines: [0x2f9, 0x301], places: &[1, 2, 3, 3], label: "cqx_6 são paulo predicted" }; // cqx6: the são paulo places

/// The predicted positions of the São Paulo state league (at creation and at the season turnover): the clubs sorted by strength; the
/// first ones get the knockout places, 1, 2, 3, 3, and the others their place in the order (`state_league_predicted_positions`
/// with `SAO_PAULO_PREDICTED`); it is the board and press expectation for each club [inference]. The error boxes at
/// lines 0x2f9 (no statistics) and 0x301 (no memory).
/// Original: `BraRegSp::set_predicted_positions` (0x43d140, bra_reg_sp.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn bra_reg_sp_set_predicted_positions(this: usize) {
    state_league_predicted_positions(this, &SAO_PAULO_PREDICTED)
}

/// Comparer for `bra_reg_sp_set_predicted_positions` (`compare_state_league_predicted`, side by side).
unsafe extern "thiscall" fn compare_bra_reg_sp_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_REG_SP_SET_PREDICTED_POSITIONS);
    compare_state_league_predicted("bra_reg_sp_set_predicted_positions", this, &SAO_PAULO_PREDICTED, || o(this));
}

// ---------- batch 291: the Coppa Italia: the predicted positions, the clubs' records and the news of a club's fate (ita_cup.cpp) ----------

/// The predicted positions of the Italian cups (0x640210 and 0x63e580, the same body; only the source file, the box
/// lines, the knockout stage and the band width change). Rule: the entrants by reputation; from the final backwards,
/// the clubs of each knockout round share the place of the losers of that round (the winner 1 and the runner-up 2,
/// then 3 for the semi-finalists, 5 for the quarter-finalists...), and the rest go in bands of `band`.
///
/// Without statistics (+8): the box `lines.0` and return. A copy of the entrant list (+0xee, +0xf2 clubs, a dword)
/// (`exe::crt_malloc`(4 × n); out of memory: the box `lines.1` and return), sorted by reputation (`exe::crt_qsort`(v, n,
/// 4, 0x4b07d0): 4 pushes; add esp -> cdecl); the round table of the knockout stage (virtual
/// `comp_vtable::CREATE_SCHEDULE`(this, `stage`, &rounds, &extra, 0): push 0; push &extra; push &rounds; push stage;
/// mov ecx -> thiscall, ret 0x10). Two byte counters start at 1: `next` (the next place) and `place` (the one being
/// given). From the last round down to round 0, with the word +0x1a of the round record (`cup_round::SIZE` bytes: the
/// round's ties): in the last round first, that many clubs get `place` (still 1: the winner) and `next` goes up by one
/// each; then `place` = `next` and that many clubs get it (`next` up by one each). The clubs taken in order from the
/// sorted copy, with a word index and no bound check (as the original). Then `place` = `next` and each remaining club
/// (index below the dword +0xf2) gets `place`; `next` goes up by one, and when `next` equals `place` + `band` (in 32
/// bits: a band past 255 never closes), `place` = `next`. Each place goes through Comp_Stats::set_club_final_position
/// (`exe::comp_stats_set_club_final_position`(stats, club, place): push place; push club; mov ecx, [+8] -> thiscall, ret
/// 8; the original pushes the place in a dword with stack garbage above the byte, which the routine does not read: the
/// copy passes the clean byte). The round table and the copy are freed (`exe::crt_free`).
unsafe fn italian_cup_predicted_positions(this: usize, file: usize, lines: (u32, u32), stage: u32, band: u32, label: &'static str) {
    let stats = read_usize(this + 8);
    if stats == 0 {
        return show_error_at(file, lines.0);
    }
    let v = exe::crt_malloc(read_u32(this + 0xf2) << 2);
    if v == 0 {
        return show_error_at(file, lines.1);
    }
    if read_i32(this + 0xf2) > 0 {
        let mut i = 0i16;
        loop {
            write_usize(v + 4 * i as usize, read_usize(read_usize(this + 0xee) + 4 * i as usize));
            i = i.wrapping_add(1);
            if i as i32 >= read_i32(this + 0xf2) {
                break;
            }
        }
    }
    exe::crt_qsort(v, read_u32(this + 0xf2), 4, 0x4b07d0);
    let create_schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut extra) = (0i16, 0i16);
    let table = create_schedule(this, stage, &mut rounds, &mut extra, 0);
    let (mut next, mut place) = (1u8, 1u8);
    let mut idx = 0i16;
    let give = |place: u8, idx: &mut i16, next: &mut u8| {
        exe::comp_stats_set_club_final_position(stats, read_usize(v + 4 * *idx as usize), place as u32);
        *idx = idx.wrapping_add(1);
        *next = next.wrapping_add(1);
    };
    let last = rounds.wrapping_sub(1);
    if last >= 0 {
        covered!(label);
        let mut r = last as i32;
        while r >= 0 {
            let ties = table.wrapping_add((r as usize).wrapping_mul(cup_round::SIZE)) + 0x1a;
            if r == rounds as i32 - 1 {
                let mut k = 0i16;
                while k < read_i16(ties) {
                    give(place, &mut idx, &mut next); // cqx7: the final's winner
                    k += 1;
                }
            }
            place = next;
            let mut k = 0i16;
            while k < read_i16(ties) {
                give(place, &mut idx, &mut next);
                k += 1;
            }
            r -= 1;
        }
    }
    place = next;
    while (idx as i32) < read_i32(this + 0xf2) {
        exe::comp_stats_set_club_final_position(stats, read_usize(v + 4 * idx as usize), place as u32);
        next = next.wrapping_add(1);
        if next as u32 == place as u32 + band {
            place = next;
        }
        idx = idx.wrapping_add(1);
    }
    exe::crt_free(table);
    exe::crt_free(v);
}

/// The predicted position (`comp_stats_club::FINAL_POSITION`) of each entrant (+0xee, +0xf2) in the cup's statistics
/// (+8), 0 without entry.
unsafe fn italian_cup_positions(this: usize) -> Vec<u8> {
    let n = read_i32(this + 0xf2).clamp(0, 0x200) as usize;
    (0..n).map(|i| {
        let c = read_usize(read_usize(this + 0xee) + 4 * i);
        let e = if c != 0 { exe::comp_stats_find_club_entry(read_usize(this + 8), c) } else { 0 };
        if e != 0 { read_u8(e + comp_stats_club::FINAL_POSITION) } else { 0 }
    }).collect()
}

/// The comparer of the Italian cups' predicted positions, side by side over the same entries: the original first
/// (Comp_Stats::set_club_final_position adds the entry of a club that has none), then the copy, which finds the entries
/// already there and only rewrites `comp_stats_club::FINAL_POSITION` (the same list, the same qsort: the same order;
/// the round table made again and freed); the position of each entrant after the copy must equal the original's.
/// Without statistics or list: only the original.
unsafe fn compare_italian_cup_predicted(name: &str, this: usize, copy: unsafe extern "thiscall" fn(usize), o: impl FnOnce()) {
    if read_usize(this + 8) == 0 || read_usize(this + 0xee) == 0 || read_i32(this + 0xf2) <= 0 {
        return o();
    }
    o();
    let original = italian_cup_positions(this);
    copy(this);
    let after = italian_cup_positions(this);
    SCOREBOARD.record(after == original, || format!("{}({:#x}): copy {:?} original {:?}", name, this, after, original));
}

/// The predicted positions of the Coppa Italia (at creation): the 48 entrants by reputation; the knockout of stage 7
/// gives 1 and 2 to the final's pair, then a shared place to the losers of each round; the rest in bands of 8
/// (`italian_cup_predicted_positions`; the boxes at lines 0x438 and 0x440).
/// Original: `ItaCup::set_predicted_positions` (0x640210, ita_cup.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn ita_cup_set_predicted_positions(this: usize) {
    italian_cup_predicted_positions(this, 0x9ea23c, (0x438, 0x440), 7, 8, "cqx_7 coppa italia predicted") // cqx7: the Coppa Italia bands of 8
}

/// Comparer for `ita_cup_set_predicted_positions` (`compare_italian_cup_predicted`, side by side).
unsafe extern "thiscall" fn compare_ita_cup_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ITA_CUP_SET_PREDICTED_POSITIONS);
    compare_italian_cup_predicted("ita_cup_set_predicted_positions", this, ita_cup_set_predicted_positions, || o(this));
}

/// The body of the two Italian cups' staff records (0x63ff00 and 0x63e270; only the knockout stage `limit` and the
/// group qualification round change), `update_staff_records_for_status(club, outcome, stage, _, fixture, final)` (the
/// virtual +0x44); returns 0. Stage (signed byte) below `limit`, the groups: outcome 6, the qualification
/// (`european_cup_add_qualified_record`: stage 0, round `group_round`, weight 0x28); 0xfe, the failure
/// (`european_cup_add_failed_to_qualify_record`: 0x41c, 0x28). Stage `limit`, the knockout, weight =
/// `scale_by_round`(0x32, 0x64, the fixture's `round_number()`, 5): 0xff, the elimination
/// (`european_cup_add_knocked_out_record`: the fixture's `round_prefix()` and `round_name()`, the weight); 0, the
/// qualification with `round_prefix()` and the name word (+0x6f) of the fixture's round in the round table of the
/// knockout stage ([`cup::STAGES`][`limit`] + `cup::ROUND_TABLE`; the high 16 bits the original pushes are the stage
/// pointer's); 1, the winner (Staff_Records 0x86a800); 2, the runner-up (0x86b0b0). Anything else: nothing. The
/// staff records are [`G_STAFF_RECORDS`] (thiscall: 0x86bdd0 ret 0x14, 0x86c1d0 ret 0x10, 0x86c000 ret 0x14,
/// 0x86a800 and 0x86b0b0 ret 0xc); words pushed with garbage in the high bits are passed clean (the callees read the
/// word).
unsafe fn italian_cup_staff_records(this: usize, club: usize, outcome: u32, stage: u32, game: usize, final_: u32, limit: i8, group_round: u32, label: &'static str) -> u32 {
    let game = Fixture(game);
    let (d, f) = (outcome as u8, stage as u8 as i8);
    let comp = read_usize(this + 4);
    if f < limit {
        if d == 6 {
            covered!(label);
            european_cup_add_qualified_record(club, comp, 0, group_round, 0x28);
        } else if d == 0xfe {
            european_cup_add_failed_to_qualify_record(club, comp, 0x41c, 0x28);
        }
    } else if f == limit {
        match d {
            0xff => {
                let weight = scale_by_round(0x32, 0x64, game.round_number(), 5);
                european_cup_add_knocked_out_record(club, comp, game.round_prefix() as u16 as u32, game.round_name() as u16 as u32, weight);
            }
            0 => {
                let weight = scale_by_round(0x32, 0x64, game.round_number(), 5);
                let knockout = read_usize(read_usize(this + cup::STAGES) + limit as usize * 4);
                let r = game.round_number() as isize;
                let name = read_u16(read_usize(knockout + cup::ROUND_TABLE).wrapping_add((r * cup_round::SIZE as isize) as usize) + 0x6f) as u32;
                european_cup_add_qualified_record(club, comp, game.round_prefix() as u16 as u32, (knockout as u32 & 0xffff_0000) | name, weight);
            }
            1 => cup_record_final_result(0x86a800, club, game, final_),
            2 => cup_record_final_result(0x86b0b0, club, game, final_),
            _ => {}
        }
    }
    0
}

/// Rule: in the Coppa Italia, the clubs that get through the groups (stages below 7) get the qualified record (round
/// 0xa, weight 0x28) and those that do not the failed record; in the knockout (stage 7) elimination and qualification
/// weigh by round (50 to 100 over the 5 rounds), and the winner and runner-up get the final records
/// (`italian_cup_staff_records`).
/// Original: `ItaCup::update_staff_records_for_status` (0x63ff00, ita_cup.cpp, thiscall, ret 0x18).
unsafe extern "thiscall" fn ita_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, _a4: u32, game: usize, final_: u32) -> u32 {
    italian_cup_staff_records(this, club, outcome, stage, game, final_, 7, 0xa, "cqx_7 coppa italia group qualified")
}

// simplification: the records go to the history (and the reputation and morale): they cannot run twice. Only the
// original; the plan: return 0.
unsafe extern "thiscall" fn compare_ita_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, a4: u32, game: usize, final_: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_ITA_CUP_UPDATE_STAFF_RECORDS_FOR_STATUS)(this, club, outcome, stage, a4, game, final_);
    SCOREBOARD.record(r == 0, || format!("ita_cup_update_staff_records_for_status({:#x}, {}, {}) returned {}", club, outcome as u8, stage as u8, r));
    r
}

/// Rule: the Coppa Italia news of a club's fate: in the groups (stages below 7) qualification (outcome 6) and
/// elimination (0xfe) go through the generic cup news; a club going through with no news object gets the text
/// "through to first round" with a link to the club; in the knockout (stage 7) the generic knockout news.
///
/// `ItaCup::build_fate_news_text(club_id, outcome, stage, a4, a5, a6, a7, a8, news, text)` (10 dwords, ret 0x28).
/// Stage (signed byte) below 7: outcome 6 with `news`: cdecl 0x4b4590(club_id, a4, the competition +4, 6, news, text)
/// (6 pushes; add esp, 0x18); outcome 6 without `news`: the club (`club_at`(club_id)) gives the text
/// (`exe::langlib_sprintf`(0xde1f64, 0x9ea270 "{}<%s - Team Name(e.g.Ajax)>{} through to first round", its byte +0x52
/// twice, its name +0x38): 5 pushes; add esp, 0x14 -> cdecl) set in `text` (Text::set_string 0x4ae660(text, 0xde1f64),
/// thiscall, ret 4) with the link (Text::add_link 0x4ae8a0(text, name, 0x7d5, club), thiscall, ret 0xc); outcome 0xfe:
/// 0x4b4590(club_id, a4, competition, −2, news, text); other outcomes nothing. Stage 7: with `news`, cdecl
/// 0x4b0b80(club_id, a5, a8, outcome, a6, text); without, cdecl 0x4b4420(club_id, a5, outcome, text). Stages above 7:
/// nothing.
/// Original: `ItaCup::build_fate_news_text` (0x63f130, ita_cup.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn ita_cup_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, _a7: u32, a8: u32, news: u32, text: usize) {
    let (d, f) = (outcome as u8, stage as u8 as i8);
    let cup_news: unsafe extern "C" fn(u32, u32, usize, i32, u32, usize) = core::mem::transmute(0x4b4590usize);
    let comp = read_usize(this + 4);
    if f < 7 {
        if d == 6 {
            if news != 0 {
                return cup_news(club_id, a4, comp, 6, news, text);
            }
            covered!("cqx_7 coppa italia through to first round");
            let club = read_usize(G_CLUB_TABLE).wrapping_add((club_id as usize).wrapping_mul(club::SIZE));
            let b = read_u8(club + 0x52) as i8 as i32;
            let sprintf: unsafe extern "C" fn(usize, usize, i32, i32, usize) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
            sprintf(0xde1f64, 0x9ea270, b, b, club + 0x38);
            let set_string: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TEXT_SET_STRING);
            set_string(text, 0xde1f64);
            let add_link: unsafe extern "thiscall" fn(usize, usize, u32, usize) = core::mem::transmute(FN_TEXT_ADD_LINK);
            add_link(text, club + 0x38, 0x7d5, club); // cqx7: the club link
        } else if d == 0xfe {
            cup_news(club_id, a4, comp, -2, news, text);
        }
    } else if f == 7 {
        if news != 0 {
            let knockout_news: unsafe extern "C" fn(u32, u32, u32, u32, u32, usize) = core::mem::transmute(0x4b0b80usize);
            knockout_news(club_id, a5, a8, outcome, a6, text);
        } else {
            let knockout_text: unsafe extern "C" fn(u32, u32, u32, usize) = core::mem::transmute(0x4b4420usize);
            knockout_text(club_id, a5, outcome, text);
        }
    }
}

/// Comparer for `ita_cup_build_fate_news_text`. On the paths that only write the text (a club going through the groups
/// without a news object: the "through to first round" text and its link) side by side with `compare_text_writer`: the
/// original into the caller's Text, the copy into a local one. On the paths that pass the news object to the generic
/// cup news (0x4b4590, 0x4b0b80, 0x4b4420) only the original (they may also write the news object, which takes its
/// text once).
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn compare_ita_cup_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize)>(IDX_ITA_CUP_BUILD_FATE_NEWS_TEXT);
    if (stage as u8 as i8) < 7 && outcome as u8 == 6 && news == 0 && text != 0 {
        return compare_text_writer(text, || o(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text),
            |t| ita_cup_build_fate_news_text(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, t),
            || format!("ita_cup_build_fate_news_text(club {}, outcome {}, stage {})", club_id, outcome as u8, stage as u8 as i8));
    }
    o(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text)
}

// ---------- batch 291: the Serie C cup: the predicted positions and the clubs' records (ita_c_cup.cpp) ----------

/// The predicted positions of the Serie C cup (at creation): the 90 entrants by reputation; the knockout of stage 0xf
/// gives 1 and 2 to the final's pair, then a shared place to the losers of each round; the rest in bands of 16
/// (`italian_cup_predicted_positions`, cqx_7; the boxes at lines 0x439 and 0x441).
/// Original: `ItaCCup::set_predicted_positions` (0x63e580, ita_c_cup.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn ita_c_cup_set_predicted_positions(this: usize) {
    italian_cup_predicted_positions(this, 0x9ea204, (0x439, 0x441), 0xf, 0x10, "cqx_8 serie c cup predicted") // cqx8: the Serie C cup bands of 16
}

/// Comparer for `ita_c_cup_set_predicted_positions` (`compare_italian_cup_predicted`, side by side).
unsafe extern "thiscall" fn compare_ita_c_cup_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ITA_C_CUP_SET_PREDICTED_POSITIONS);
    compare_italian_cup_predicted("ita_c_cup_set_predicted_positions", this, ita_c_cup_set_predicted_positions, || o(this));
}

/// Rule: in the Serie C cup, as in the Coppa Italia, with the groups below stage 0xf and the group qualification at
/// round 0x14 (weight 0x28) (`italian_cup_staff_records`, cqx_7).
/// Original: `ItaCCup::update_staff_records_for_status` (0x63e270, ita_c_cup.cpp, thiscall, ret 0x18).
unsafe extern "thiscall" fn ita_c_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, _a4: u32, game: usize, final_: u32) -> u32 {
    italian_cup_staff_records(this, club, outcome, stage, game, final_, 0xf, 0x14, "cqx_8 serie c cup group qualified")
}

// simplification: the records go to the history (and the reputation and morale): they cannot run twice. Only the
// original; the plan: return 0.
unsafe extern "thiscall" fn compare_ita_c_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, a4: u32, game: usize, final_: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_ITA_C_CUP_UPDATE_STAFF_RECORDS_FOR_STATUS)(this, club, outcome, stage, a4, game, final_);
    SCOREBOARD.record(r == 0, || format!("ita_c_cup_update_staff_records_for_status({:#x}, {}, {}) returned {}", club, outcome as u8, stage as u8, r));
    r
}

// ---------- batch 291: the season turnover of the Asian Super Cup and of the CONCACAF Champions' Cup; the Asian Club Championship records (asia_super_cup.cpp, con_champ.cpp, asia_club_champ.cpp) ----------

/// The first half of a cup's season turnover (the same block in AsiaSuperCup and ConChamp): `cup::SEASON_CLOSED` = 0;
/// the team table (`cup::TEAM_TABLE`) and the round table (`cup::ROUND_TABLE`) freed (`exe::crt_free`: push; add esp, 4
/// -> cdecl) and zeroed; with the tie tables (`cup::TIE_TABLES`), each of the `cup::ROUND_COUNT` rounds' (a signed
/// word) freed and zeroed, then the array freed and zeroed; the season archived in the statistics (+8, when there:
/// Comp_Stats::archive_season_and_reset `exe::comp_stats_archive_season_and_reset`(stats, 1), thiscall, ret 4); the
/// year (`cup::YEAR`) up by one; `cup::CURRENT_ROUND` and `cup::RUN_BY_STAGES` = 0.
unsafe fn cup_turnover_free_season(cup: Cup) {
    cup.set_season_closed(0);
    if cup.team_table() != 0 {
        exe::crt_free(cup.team_table());
        cup.set_team_table(0);
    }
    if cup.round_table() != 0 {
        exe::crt_free(cup.round_table());
        cup.set_round_table(0);
    }
    if cup.tie_tables() != 0 {
        let mut i = 0i32;
        while i < cup.round_count() as i32 {
            let p = read_usize(cup.tie_tables() + i as usize * 4);
            if p != 0 {
                exe::crt_free(p);
                write_usize(cup.tie_tables() + i as usize * 4, 0);
            }
            i += 1;
        }
        exe::crt_free(cup.tie_tables());
        cup.set_tie_tables(0);
    }
    let stats = read_usize(cup.addr() + 8);
    if stats != 0 {
        exe::comp_stats_archive_season_and_reset(stats, 1);
    }
    cup.set_year(cup.year().wrapping_add(1)); // cqx9: the next season's cup
    cup.set_current_round(0);
    cup.set_run_by_stages(0);
}

/// A cup virtual (thiscall, ret 0) at slot offset `o`; its signed return.
unsafe fn cup_turnover_virtual(cup: Cup, o: usize) -> i32 {
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(read_usize(read_usize(cup.addr()) + o));
    f(cup.addr())
}

/// Rule: the Asian Super Cup turns over only in June (month 5, 0-based): the finished cup is cleared, its statistics
/// archived and the year moves on; then the two entrants, the rules, the tie tree, the first round and the predicted
/// positions are made again.
///
/// `AsiaSuperCup::start_new_season()` (virtual +8). The month of today (Date::get_month `exe::date_get_month`
/// ([`G_CURRENT_DATE`]), thiscall, ret 0: 0 to 11, or −1) other than 5: nothing (it returns the month). Otherwise the
/// first half (`cup_turnover_free_season`); the entrants (AsiaSuperCup 0x410a50: mov ecx -> thiscall, ret 0; negative:
/// the error box at line 0x11f and return), the rules (virtual `comp_vtable::SET_UP_RULES`; negative: the box at line
/// 0x126), the tie tree (virtual `cup_vtable::MAP_FIXTURE_TREE`; negative: the box at line 0x12d), the first round
/// (Cup::set_up_round `exe::cup_set_up_round`(this, 0): push 0; mov ecx -> thiscall, ret 4) and the predicted positions
/// (virtual `comp_vtable::SET_PREDICTED_POSITIONS`), whose return it passes on.
/// Original: `AsiaSuperCup::start_new_season` (0x410c50, asia_super_cup.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn asia_super_cup_start_new_season(this: usize) -> u32 {
    let cup = Cup(this);
    let month = exe::date_get_month(G_CURRENT_DATE);
    // simplification: the original returns the whole eax of get_month (the month, or −1); the caller does not read it
    if month != 5 { // cqx9: the Asian Super Cup turns over in June
        return month as i8 as i32 as u32;
    }
    covered!("cqx_9 asian super cup turnover");
    cup_turnover_free_season(cup);
    let entrants: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x410a50usize);
    // simplification: in the boxes the original returns the box's eax; the caller does not read it
    if entrants(this) < 0 {
        show_error_at(0x987be0, 0x11f);
        return 0;
    }
    if cup_turnover_virtual(cup, comp_vtable::SET_UP_RULES) < 0 {
        show_error_at(0x987be0, 0x126);
        return 0;
    }
    if cup_turnover_virtual(cup, cup_vtable::MAP_FIXTURE_TREE) < 0 {
        show_error_at(0x987be0, 0x12d);
        return 0;
    }
    exe::cup_set_up_round(cup, 0); // cqx9: the first round of the new Super Cup
    cup_turnover_virtual(cup, comp_vtable::SET_PREDICTED_POSITIONS) as u32
}

// simplification: the turnover frees the tables, archives the statistics and draws: it cannot run twice. Only the
// original; the plan: in June the year goes up by one (`compare_cup_new_season`), in the other months nothing changes.
unsafe extern "thiscall" fn compare_asia_super_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ASIA_SUPER_CUP_START_NEW_SEASON);
    if exe::date_get_month(G_CURRENT_DATE) != 5 {
        let year = Cup(this).year();
        let r = o(this);
        SCOREBOARD.record(Cup(this).year() == year, || format!("asia_super_cup_start_new_season({:#x}) outside June: year {} -> {}", this, year, Cup(this).year()));
        return r;
    }
    compare_cup_new_season("asia_super_cup_start_new_season", Cup(this), || o(this))
}

/// Rule: the CONCACAF Champions' Cup turnover: the finished cup is cleared, its statistics archived and the year moves
/// on; an empty team table is made for the entrants (chosen later) and the rules and the tie tree are made again.
///
/// `ConChamp::start_new_season()` (virtual +8): the first half (`cup_turnover_free_season`); the team table
/// (`exe::crt_calloc`(`cup::TEAM_COUNT`, 6): push 6; push n; add esp, 8 -> cdecl) in `cup::TEAM_TABLE`; without it, the
/// error box at line 0x184 and return; the rules (virtual `comp_vtable::SET_UP_RULES`; negative: the box at line
/// 0x18b) and the tie tree (virtual `cup_vtable::MAP_FIXTURE_TREE`; negative: the box at line 0x192).
/// Original: `ConChamp::start_new_season` (0x4c1440, con_champ.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn con_champ_start_new_season(this: usize) {
    let cup = Cup(this);
    covered!("cqx_9 concacaf champions cup turnover");
    cup_turnover_free_season(cup);
    let t = exe::crt_calloc(cup.team_count() as i32 as u32, 6);
    cup.set_team_table(t);
    if t == 0 {
        return show_error_at(0x9a7d30, 0x184);
    }
    if cup_turnover_virtual(cup, comp_vtable::SET_UP_RULES) < 0 {
        return show_error_at(0x9a7d30, 0x18b);
    }
    if cup_turnover_virtual(cup, cup_vtable::MAP_FIXTURE_TREE) < 0 { // cqx9: the CONCACAF tie tree
        show_error_at(0x9a7d30, 0x192);
    }
}

// simplification: the turnover frees the tables and archives the statistics: it cannot run twice. Only the original;
// the plan: the year goes up by one and the team table is there.
unsafe extern "thiscall" fn compare_con_champ_start_new_season(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CON_CHAMP_START_NEW_SEASON);
    let year = Cup(this).year();
    o(this);
    SCOREBOARD.record(Cup(this).year() == year.wrapping_add(1) && Cup(this).team_table() != 0, || format!("con_champ_start_new_season({:#x}): year {} -> {}, team table {:#x}", this, year, Cup(this).year(), Cup(this).team_table()));
}

/// Rule: the Asian Club Championship records: in the zone knockouts (stage 0), elimination and qualification weigh by
/// round (1 to 80 over the 2 rounds) and the zone winner gets the record of reaching the finals (round 0x78, weight
/// 0x5a); in the stages 1 and 2 the qualified club gets the record of round 0x82 (weight 0x5a); in the finals (stage 3)
/// the winner, the runner-up and the third get the final records.
///
/// `AsiaClubChamp::update_staff_records_for_status(club, outcome, stage, _, fixture, final)` (the virtual +0x44).
/// Stage (signed byte) below 1, weight = `scale_by_round`(1, 0x50, the fixture's `round_number()`, 2): outcome 0xff or
/// 2, the elimination (`european_cup_add_knocked_out_record`: `round_prefix()`, `round_name()`, the weight); 0, the
/// qualification (`european_cup_add_qualified_record`: `round_prefix()`, the name word +0x6f of the fixture's round in
/// the cup's own round table `cup::ROUND_TABLE`, the weight); 1, the qualification (0, 0x78, 0x5a); returns 0. Stages 1
/// and 2: outcome 6, the qualification (0, 0x82, 0x5a), returns 0; otherwise −1. Stage 3: outcome 1 the winner
/// (Staff_Records 0x86a800), 2 the runner-up (0x86b0b0), 3 the third (0x86b710), returns 0; otherwise −1. Above 3: 0.
/// (The high 16 bits of the words the original pushes come from small products of the round: zero.)
/// Original: `AsiaClubChamp::update_staff_records_for_status` (0x40c5b0, asia_club_champ.cpp, thiscall, ret 0x18).
unsafe extern "thiscall" fn asia_club_champ_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, _a4: u32, game: usize, final_: u32) -> u32 {
    let g = Fixture(game);
    let (d, f) = (outcome as u8, stage as u8 as i8);
    let comp = read_usize(this + 4);
    if f < 1 {
        match d {
            0xff | 2 => {
                let weight = scale_by_round(1, 0x50, g.round_number(), 2);
                european_cup_add_knocked_out_record(club, comp, g.round_prefix() as u16 as u32, g.round_name() as u16 as u32, weight);
            }
            0 => {
                covered!("cqx_9 asian club championship zone qualified");
                let weight = scale_by_round(1, 0x50, g.round_number(), 2);
                let name = read_u16(Cup(this).round_table().wrapping_add((g.round_number() as isize * cup_round::SIZE as isize) as usize) + 0x6f) as u32;
                european_cup_add_qualified_record(club, comp, g.round_prefix() as u16 as u32, name, weight);
            }
            1 => european_cup_add_qualified_record(club, comp, 0, 0x78, 0x5a),
            _ => {}
        }
        0
    } else if f < 3 {
        if d == 6 {
            european_cup_add_qualified_record(club, comp, 0, 0x82, 0x5a);
            0
        } else {
            u32::MAX
        }
    } else if f == 3 {
        match d {
            1 => cup_record_final_result(0x86a800, club, g, final_),
            2 => cup_record_final_result(0x86b0b0, club, g, final_),
            3 => cup_record_final_result(0x86b710, club, g, final_),
            _ => return u32::MAX,
        }
        0
    } else {
        0
    }
}

// simplification: the records go to the history (and the reputation and morale): they cannot run twice. Only the
// original; the plan: the return by the stage and outcome (0, or −1 where the copy gives −1).
unsafe extern "thiscall" fn compare_asia_club_champ_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, a4: u32, game: usize, final_: u32) -> u32 {
    let (d, f) = (outcome as u8, stage as u8 as i8);
    let plan = if (1..3).contains(&f) && d != 6 || f == 3 && !(1..=3).contains(&d) { u32::MAX } else { 0 };
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_ASIA_CLUB_CHAMP_UPDATE_STAFF_RECORDS_FOR_STATUS)(this, club, outcome, stage, a4, game, final_);
    SCOREBOARD.record(r == plan, || format!("asia_club_champ_update_staff_records_for_status({:#x}, {}, {}) returned {:#x}, plan {:#x}", club, d, f, r, plan));
    r
}

// ---------- batch 291: the Paulista's points for a result; the records of the Asian Cup Winners' Cup and of the Merconorte (bra_reg_sp.cpp, asia_cup_winner.cpp, con_merc_cup.cpp) ----------

/// Rule: the São Paulo state league gives the league points for a result (the win and draw points of its rules),
/// passing the query to the League base.
///
/// `BraRegSp::points_for_result(club, fixture)` (the virtual of the points query): League::points_for_result
/// (0x68aad0, thiscall(this, club, fixture), ret 8; this goes through in ecx untouched), whose return it passes on.
/// Original: `BraRegSp::points_for_result` (0x43c4a0, bra_reg_sp.cpp, thiscall, ret 8).
unsafe extern "thiscall" fn bra_reg_sp_points_for_result(this: usize, club: usize, fixture: usize) -> u32 {
    covered!("cqx_10 paulista points");
    let league_points: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(0x68aad0usize);
    league_points(this, club, fixture) as u32 // cqx10: the League's points
}

/// Comparer for `bra_reg_sp_points_for_result`: side by side (the query only reads); the returned byte (the caller
/// reads al) must be equal.
unsafe extern "thiscall" fn compare_bra_reg_sp_points_for_result(this: usize, club: usize, fixture: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_BRA_REG_SP_POINTS_FOR_RESULT)(this, club, fixture);
    let c = bra_reg_sp_points_for_result(this, club, fixture);
    SCOREBOARD.record(r as u8 == c as u8, || format!("bra_reg_sp_points_for_result({:#x}, {:#x}) original {} copy {}", club, fixture, r as u8, c as u8));
    r
}

/// Rule: the Asian Cup Winners' Cup records: in the zone knockouts (stage 0), elimination and qualification weigh by
/// round (1 to 80 over the 3 rounds) and the zone winner gets the record of round 0x82 (weight 0x5a); in the final
/// (stage 1) the winner, the runner-up and the third get the final records.
///
/// `AsiaCupWinner::update_staff_records_for_status(club, outcome, stage, _, fixture, final)` (the virtual +0x44).
/// Stage (signed byte) below 1, weight = `scale_by_round`(1, 0x50, the fixture's `round_number()`, 3): outcome 0xff or
/// 2, the elimination (`european_cup_add_knocked_out_record`: `round_prefix()`, `round_name()`, the weight); 0, the
/// qualification (`european_cup_add_qualified_record`: `round_prefix()`, the name word +0x6f of the fixture's round in
/// the cup's round table `cup::ROUND_TABLE`, the weight); 1, the qualification (0, 0x82, 0x5a); returns 0. Stage 1:
/// outcome 1 the winner (Staff_Records 0x86a800), 2 the runner-up (0x86b0b0), 3 the third (0x86b710), returns 0;
/// otherwise −1. Above 1: 0. (The high 16 bits of the words the original pushes come from small products of the round:
/// zero.)
/// Original: `AsiaCupWinner::update_staff_records_for_status` (0x40e260, asia_cup_winner.cpp, thiscall, ret 0x18).
unsafe extern "thiscall" fn asia_cup_winner_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, _a4: u32, game: usize, final_: u32) -> u32 {
    let g = Fixture(game);
    let (d, f) = (outcome as u8, stage as u8 as i8);
    let comp = read_usize(this + 4);
    if f < 1 {
        match d {
            0xff | 2 => {
                let weight = scale_by_round(1, 0x50, g.round_number(), 3);
                european_cup_add_knocked_out_record(club, comp, g.round_prefix() as u16 as u32, g.round_name() as u16 as u32, weight);
            }
            0 => {
                covered!("cqx_10 cup winners cup zone qualified");
                let weight = scale_by_round(1, 0x50, g.round_number(), 3);
                let name = read_u16(Cup(this).round_table().wrapping_add((g.round_number() as isize * cup_round::SIZE as isize) as usize) + 0x6f) as u32;
                european_cup_add_qualified_record(club, comp, g.round_prefix() as u16 as u32, name, weight);
            }
            1 => european_cup_add_qualified_record(club, comp, 0, 0x82, 0x5a),
            _ => {}
        }
        0
    } else if f == 1 {
        match d {
            1 => cup_record_final_result(0x86a800, club, g, final_),
            2 => cup_record_final_result(0x86b0b0, club, g, final_),
            3 => cup_record_final_result(0x86b710, club, g, final_),
            _ => return u32::MAX,
        }
        0
    } else {
        0
    }
}

// simplification: the records go to the history: they cannot run twice. Only the original; the plan: the return (−1
// in the final for an outcome other than 1 to 3, otherwise 0).
unsafe extern "thiscall" fn compare_asia_cup_winner_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, a4: u32, game: usize, final_: u32) -> u32 {
    let (d, f) = (outcome as u8, stage as u8 as i8);
    let plan = if f == 1 && !(1..=3).contains(&d) { u32::MAX } else { 0 };
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_ASIA_CUP_WINNER_UPDATE_STAFF_RECORDS_FOR_STATUS)(this, club, outcome, stage, a4, game, final_);
    SCOREBOARD.record(r == plan, || format!("asia_cup_winner_update_staff_records_for_status({:#x}, {}, {}) returned {:#x}, plan {:#x}", club, d, f, r, plan));
    r
}

/// Rule: the Merconorte records: in the groups (stages below 3) the qualified club gets the record of round 0x82 and
/// the club going out the failed record (weight 0x28 both); in the knockout (stage 3) elimination and qualification
/// weigh by round (50 to 100 over the 2 rounds) and the winner and runner-up get the final records.
///
/// `ConMercCup::update_staff_records_for_status(club, outcome, stage, _, fixture, final)` (the virtual +0x44); returns
/// 0. Stage (signed byte) below 3: outcome 6, the qualification (`european_cup_add_qualified_record`: 0, 0x82, 0x28);
/// 0xfe, the failure (`european_cup_add_failed_to_qualify_record`: 0x41c, 0x28). Stage 3, weight =
/// `scale_by_round`(0x32, 0x64, the fixture's `round_number()`, 2): 0xff, the elimination
/// (`european_cup_add_knocked_out_record`: `round_prefix()`, `round_name()`, the weight); 0, the qualification with
/// `round_prefix()` and the name word +0x6f of the fixture's round in the round table of stage 3 ([`cup::STAGES`][3] +
/// `cup::ROUND_TABLE`; the high 16 bits the original pushes are the stage pointer's); 1, the winner (0x86a800); 2, the
/// runner-up (0x86b0b0). Anything else, and above stage 3: nothing.
/// Original: `ConMercCup::update_staff_records_for_status` (0x4c2940, con_merc_cup.cpp, thiscall, ret 0x18).
unsafe extern "thiscall" fn con_merc_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, _a4: u32, game: usize, final_: u32) -> u32 {
    let g = Fixture(game);
    let (d, f) = (outcome as u8, stage as u8 as i8);
    let comp = read_usize(this + 4);
    if f < 3 {
        if d == 6 {
            covered!("cqx_10 merconorte group qualified");
            european_cup_add_qualified_record(club, comp, 0, 0x82, 0x28);
        } else if d == 0xfe {
            european_cup_add_failed_to_qualify_record(club, comp, 0x41c, 0x28);
        }
    } else if f == 3 {
        match d {
            0xff => {
                let weight = scale_by_round(0x32, 0x64, g.round_number(), 2);
                european_cup_add_knocked_out_record(club, comp, g.round_prefix() as u16 as u32, g.round_name() as u16 as u32, weight);
            }
            0 => {
                let weight = scale_by_round(0x32, 0x64, g.round_number(), 2);
                let knockout = read_usize(read_usize(this + cup::STAGES) + 0xc);
                let name = read_u16(read_usize(knockout + cup::ROUND_TABLE).wrapping_add((g.round_number() as isize * cup_round::SIZE as isize) as usize) + 0x6f) as u32;
                european_cup_add_qualified_record(club, comp, g.round_prefix() as u16 as u32, (knockout as u32 & 0xffff_0000) | name, weight);
            }
            1 => cup_record_final_result(0x86a800, club, g, final_),
            2 => cup_record_final_result(0x86b0b0, club, g, final_),
            _ => {}
        }
    }
    0
}

// simplification: the records go to the history: they cannot run twice. Only the original; the plan: return 0.
unsafe extern "thiscall" fn compare_con_merc_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, a4: u32, game: usize, final_: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_CON_MERC_CUP_UPDATE_STAFF_RECORDS_FOR_STATUS)(this, club, outcome, stage, a4, game, final_);
    SCOREBOARD.record(r == 0, || format!("con_merc_cup_update_staff_records_for_status({:#x}, {}, {}) returned {}", club, outcome as u8, stage as u8, r));
    r
}

// ---------- batch 291: the Intertoto match (the UEFA coefficient points); the records of the LDV Vans Trophy and of the Oceania Club Championship (intertoto_cup.cpp, eng_auto_cup.cpp, oceania_club_champ.cpp) ----------

/// The comparer's dry run of `intertoto_cup_process_fixture`: the copy gives the coefficient points without the cup
/// base (Cup::process_fixture, which plays out the whole match and cannot run twice).
static INTERTOTO_FIXTURE_DRY_RUN: AtomicBool = AtomicBool::new(false);

/// Rule: each Intertoto match gives UEFA coefficient points to the clubs' nations: 2 to the winner's nation, 1 to each
/// on a draw (by the score after extra time when there was extra time); then the match goes on as in any cup.
///
/// `IntertotoCup::process_fixture(fixture, a2)` (virtual). The score: `home_goals_extra_time()` and
/// `away_goals_extra_time()` when the first is not 0xff, otherwise `home_goals()` and `away_goals()` (signed bytes).
/// Home ahead: 2 points to the home club's nation (UefaSeeding::add_nation_points `add_uefa_coefficient`([club +
/// `club::NATION`], 2): push 2; push nation; mov ecx, [`G_UEFA_SEEDING`] -> thiscall, ret 8); away ahead: 2 to the away
/// club's; a draw: 1 to the home club's, then 1 to the away club's. Then the cup base (`exe::cup_process_fixture`(this,
/// fixture, a2): push a2; push fixture; mov ecx, this -> thiscall, ret 8).
/// Original: `IntertotoCup::process_fixture` (0x633290, intertoto_cup.cpp, thiscall, ret 8).
unsafe extern "thiscall" fn intertoto_cup_process_fixture(this: usize, game: usize, arg: u32) {
    let g = Fixture(game);
    let (ga, gb) = if (g.home_goals_extra_time() as u8) != 0xff {
        ((g.home_goals_extra_time() as u8) as i8, (g.away_goals_extra_time() as u8) as i8)
    } else {
        ((g.home_goals() as u8) as i8, (g.away_goals() as u8) as i8)
    };
    covered!("cqx_11 intertoto coefficient");
    if ga > gb {
        add_uefa_coefficient(g.home_club().nation().addr(), 2); // cqx11: the win's 2 points
    } else if ga < gb {
        add_uefa_coefficient(g.away_club().nation().addr(), 2);
    } else {
        add_uefa_coefficient(g.home_club().nation().addr(), 1);
        add_uefa_coefficient(g.away_club().nation().addr(), 1);
    }
    if INTERTOTO_FIXTURE_DRY_RUN.load(Relaxed) {
        return;
    }
    exe::cup_process_fixture(this, game, arg); // cqx11: the cup base plays the match out
}

/// Comparer for `intertoto_cup_process_fixture` (the `compare_process_fixture` mould of the Champions League): the copy
/// first in a dry run (only the coefficient points, without counting draws), the two nations' coefficient words
/// (`nation_coefficient_word`) read and put back; then the original; the words after the original must equal the
/// copy's. When the stage advanced in the base (its draws add the qualified clubs' coefficient): no comparison. Without
/// a club or a nation: only the original.
unsafe extern "thiscall" fn compare_intertoto_cup_process_fixture(this: usize, game: usize, arg: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_INTERTOTO_CUP_PROCESS_FIXTURE);
    let g = Fixture(game);
    let (a, b) = if !g.is_null() { (g.home_club(), g.away_club()) } else { (Club(0), Club(0)) };
    if a.is_null() || b.is_null() {
        return o(this, game, arg);
    }
    let (ca, cb) = (nation_coefficient_word(a), nation_coefficient_word(b));
    if ca == 0 || cb == 0 {
        return o(this, game, arg);
    }
    let check = [(ca, 2), (cb, 2)];
    let before = crate::comparison::MemorySnapshot::read(&check);
    let stage = Cup(this).current_stage() as i32;
    INTERTOTO_FIXTURE_DRY_RUN.store(true, Relaxed);
    crate::rng::pause_trace(|| crate::rng::without_counting(|| intertoto_cup_process_fixture(this, game, arg)));
    INTERTOTO_FIXTURE_DRY_RUN.store(false, Relaxed);
    let copy = crate::comparison::MemorySnapshot::read(&check);
    before.write_back();
    o(this, game, arg);
    if Cup(this).current_stage() as i32 != stage {
        return; // simplification: the stage advanced and its draw added coefficient: no comparison
    }
    let original = crate::comparison::MemorySnapshot::read(&check);
    SCOREBOARD.record(original == copy, || format!("intertoto_cup_process_fixture(game {:x}) original={:x?} copy={:x?}", game, original, copy));
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
unsafe extern "thiscall" fn eng_auto_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, _a4: u32, game: usize, final_: u32) -> u32 {
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

// simplification: the records go to the history: they cannot run twice. Only the original; the plan: return 0.
unsafe extern "thiscall" fn compare_eng_auto_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, a4: u32, game: usize, final_: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_ENG_AUTO_CUP_UPDATE_STAFF_RECORDS_FOR_STATUS)(this, club, outcome, stage, a4, game, final_);
    SCOREBOARD.record(r == 0, || format!("eng_auto_cup_update_staff_records_for_status({:#x}, {}, {}) returned {}", club, outcome as u8, stage as u8, r));
    r
}

/// Rule: the Oceania Club Championship records: in the groups (stage below 1) the qualified club gets the record of
/// round 0x82 and the club going out the failed record (weight 0x19 both); in the final (stage 1) the winner and
/// runner-up get the final records.
///
/// `OceaniaClubChamp::update_staff_records_for_status(club, outcome, stage, _, fixture, final)` (the virtual +0x44);
/// returns 0. Stage (signed byte) below 1: outcome 6, the qualification (`european_cup_add_qualified_record`: 0, 0x82,
/// 0x19); 0xfe, the failure (`european_cup_add_failed_to_qualify_record`: 0x41c, 0x19). Stage 1: outcome 1 the winner
/// (Staff_Records 0x86a800), 2 the runner-up (0x86b0b0). Anything else: nothing.
/// Original: `OceaniaClubChamp::update_staff_records_for_status` (0x797a90, oceania_club_champ.cpp, thiscall, ret 0x18).
unsafe extern "thiscall" fn oceania_club_champ_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, _a4: u32, game: usize, final_: u32) -> u32 {
    let (d, f) = (outcome as u8, stage as u8 as i8);
    let comp = read_usize(this + 4);
    if f < 1 {
        if d == 6 {
            covered!("cqx_11 oceania group qualified");
            european_cup_add_qualified_record(club, comp, 0, 0x82, 0x19);
        } else if d == 0xfe {
            european_cup_add_failed_to_qualify_record(club, comp, 0x41c, 0x19);
        }
    } else if f == 1 {
        match d {
            1 => cup_record_final_result(0x86a800, club, Fixture(game), final_),
            2 => cup_record_final_result(0x86b0b0, club, Fixture(game), final_),
            _ => {}
        }
    }
    0
}

// simplification: the records go to the history: they cannot run twice. Only the original; the plan: return 0.
unsafe extern "thiscall" fn compare_oceania_club_champ_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, a4: u32, game: usize, final_: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_OCEANIA_CLUB_CHAMP_UPDATE_STAFF_RECORDS_FOR_STATUS)(this, club, outcome, stage, a4, game, final_);
    SCOREBOARD.record(r == 0, || format!("oceania_club_champ_update_staff_records_for_status({:#x}, {}, {}) returned {}", club, outcome as u8, stage as u8, r));
    r
}

// ---------- batch 291: the UEFA Cup club list; the domestic cup semi-final losers in the UEFA seeding; the Mercosur records (uefa_cup.cpp, uefa_seeding.cpp, conmebol_merc.cpp) ----------

/// Rule: the UEFA Cup's club list: of one round (kind 1: round count − 8, kind 2: − 7, kind 3: − 4) the two clubs of
/// each tie, in tie order; otherwise (or with no such round) every entrant of the team table.
///
/// `UefaCup::collect_club_list(&out, kind)` (virtual +0x58). The list (`exe::crt_malloc`(4 × `cup::TEAM_COUNT`): push;
/// add esp, 4 -> cdecl; out of memory: the error box at line 0x2d6, `out` = 0 and 0). Kind (byte) 1, 2 or 3: the round
/// r = `cup::ROUND_COUNT` − 8, 7 or 4 (words); if r > −1, for each of the round's ties (the word +0x1a of its record in
/// `cup::ROUND_TABLE`, `cup_round::SIZE` bytes; the tie k at [`cup::TIE_TABLES`][r] + 0x35 k) the home team (word +0)
/// and then the away team (word +2), when not −1, as clubs of the team table (`cup::TEAM_TABLE`, 6-byte rows), with no
/// bound check (as the original). Otherwise every non-null club of the team table (`cup::TEAM_COUNT`, re-read each
/// turn), while the count is below the table size. `out` = the list; returns the count in the low word (the high word
/// is `out`'s, as the original's eax).
/// Original: `UefaCup::collect_club_list` (0x903120, uefa_cup.cpp, thiscall, ret 8).
unsafe extern "thiscall" fn uefa_cup_collect_club_list(this: usize, out: usize, kind: u32) -> u32 {
    let cup = Cup(this);
    let n = cup.team_count();
    let v = exe::crt_malloc(((n as i32) << 2) as u32);
    if v == 0 {
        show_error_at(0xaa19e0, 0x2d6);
        write_usize(out, 0);
        return 0; // simplification: the original returns the box's eax with ax = 0; the caller reads the word
    }
    let mut count = 0i16;
    let round = match kind as u8 {
        1 => Some(cup.round_count().wrapping_sub(8)),
        2 => Some(cup.round_count().wrapping_sub(7)),
        3 => Some(cup.round_count().wrapping_sub(4)),
        _ => None,
    };
    let push = |c: usize, count: &mut i16| {
        write_usize(v + 4 * *count as usize, c);
        *count = count.wrapping_add(1);
    };
    match round {
        Some(r) if r > -1 => {
            covered!("cqx_12 uefa cup round clubs");
            let r = r as usize;
            let mut k = 0i16;
            while k < read_i16(cup.round_table().wrapping_add(r * cup_round::SIZE) + 0x1a) {
                let tie = read_usize(cup.tie_tables() + 4 * r).wrapping_add(0x35 * k as usize);
                let home = read_i16(tie);
                if home != -1 {
                    push(read_usize(cup.team_table().wrapping_add(6 * home as isize as usize)), &mut count);
                }
                let away = read_i16(tie + 2);
                if away != -1 { // cqx12: the away club of the tie
                    push(read_usize(cup.team_table().wrapping_add(6 * away as isize as usize)), &mut count);
                }
                k += 1;
            }
        }
        _ => {
            let mut i = 0i16;
            while i < cup.team_count() {
                let c = read_usize(cup.team_table() + 6 * i as usize);
                if c != 0 && count < n {
                    push(c, &mut count);
                }
                i += 1;
            }
        }
    }
    write_usize(out, v);
    (out as u32 & 0xffff_0000) | count as u16 as u32
}

/// Comparer for `uefa_cup_collect_club_list`: side by side (it only allocates its own list): the original with the
/// caller's `out`, the copy with its own; the counts (the low word) and the clubs of the two lists must be equal; the
/// copy's list is freed.
unsafe extern "thiscall" fn compare_uefa_cup_collect_club_list(this: usize, out: usize, kind: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_UEFA_CUP_COLLECT_CLUB_LIST)(this, out, kind);
    let mut mine = 0usize;
    let c = uefa_cup_collect_club_list(this, &mut mine as *mut usize as usize, kind);
    let theirs = read_usize(out);
    let n = (r as u16 as i16).max(0) as usize;
    let same = r as u16 == c as u16 && (theirs == 0) == (mine == 0) && (theirs == 0 || (0..n).all(|k| read_usize(theirs + 4 * k) == read_usize(mine + 4 * k)));
    if mine != 0 {
        exe::crt_free(mine);
    }
    SCOREBOARD.record(same, || format!("uefa_cup_collect_club_list({:#x}, {}): original {} copy {}", this, kind as u8, r as u16, c as u16));
    r
}

/// Rule: the two losing semi-finalists of a European nation's domestic cup are kept in the UEFA seeding (in the cup's
/// slot, or the first free of 50), for the UEFA Cup places of the next season; a cup of another continent is left out.
///
/// `UefaSeeding::record_cup_semi_final_losers(comp_id, club_a, club_b)`. The competition (`comp_record_of_key`'s
/// arithmetic: [`G_CLUB_COMP_TABLE`] + 0x6b × id) must have a continent (`competition::CONTINENT`) and it must be
/// Europe ([`KEY_CONTINENT_EUROPE`]); otherwise nothing. Among the 50 slots of 12 bytes at +0x440 (the cup id, then the
/// two clubs' dwords), the one with this id, or else the first with −1; none: the error box at line 0x6b2. The slot
/// gets the id, `club_a` (+4) and `club_b` (+8).
/// Original: `UefaSeeding::record_cup_semi_final_losers` (0x905790, uefa_seeding.cpp, thiscall, ret 0xc).
unsafe extern "thiscall" fn uefa_seeding_record_cup_semi_final_losers(this: usize, comp_id: u32, club_a: u32, club_b: u32) {
    let comp = read_usize(G_CLUB_COMP_TABLE).wrapping_add((comp_id as usize).wrapping_mul(0x6b));
    let continent = read_usize(comp + competition::CONTINENT);
    if continent == 0 || read_u32(continent) != read_u32(KEY_CONTINENT_EUROPE) {
        return;
    }
    let mut slot: i8 = -1;
    for k in 0..0x32i8 {
        let e = read_u32(this + 0x440 + 12 * k as usize);
        if e == comp_id {
            slot = k;
            break;
        }
        if slot == -1 && e == u32::MAX {
            slot = k;
        }
    }
    if slot == -1 {
        return show_error_at(0xaa1a1c, 0x6b2);
    }
    covered!("cqx_12 semi-final losers");
    let e = this + 0x440 + 12 * slot as usize;
    write_u32(e, comp_id);
    write_u32(e + 4, club_a); // cqx12: the first semi-final loser
    write_u32(e + 8, club_b);
}

/// Comparer for `uefa_seeding_record_cup_semi_final_losers`, side by side over the same object: the original first,
/// then the copy, which finds the cup's slot the original filled and writes it again; the 50 slots after the copy must
/// equal the original's.
unsafe extern "thiscall" fn compare_uefa_seeding_record_cup_semi_final_losers(this: usize, comp_id: u32, club_a: u32, club_b: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32)>(IDX_UEFA_SEEDING_RECORD_CUP_SEMI_FINAL_LOSERS)(this, comp_id, club_a, club_b);
    let original = crate::comparison::MemorySnapshot::read(&[(this + 0x440, 0x258)]);
    uefa_seeding_record_cup_semi_final_losers(this, comp_id, club_a, club_b);
    let copy = crate::comparison::MemorySnapshot::read(&[(this + 0x440, 0x258)]);
    SCOREBOARD.record(copy == original, || format!("uefa_seeding_record_cup_semi_final_losers({}, {:#x}, {:#x}): the slots differ", comp_id, club_a, club_b));
}

/// Rule: the Mercosur records: in the groups (stages below 4) the qualified club gets the record of round 0x78 (weight
/// 0x28); in the knockout (stage 4) elimination and qualification weigh by round (50 to 100 over the 2 rounds) and the
/// winner and runner-up get the final records.
///
/// `ConmebolMerc::update_staff_records_for_status(club, outcome, stage, _, fixture, final)` (the virtual +0x44). Stage
/// (signed byte) below 4: outcome 6, the qualification (`european_cup_add_qualified_record`: 0, 0x78, 0x28), returns 0;
/// otherwise −1. Stage 4, weight = `scale_by_round`(0x32, 0x64, the fixture's `round_number()`, 2): 0xff, the
/// elimination (`european_cup_add_knocked_out_record`: `round_prefix()`, `round_name()`, the weight); 0, the
/// qualification with `round_prefix()` and the name word +0x6f of the fixture's round in the round table of stage 4
/// ([`cup::STAGES`][4] + `cup::ROUND_TABLE`; the high 16 bits come from a small product of the round: zero); 1, the
/// winner (0x86a800); 2, the runner-up (0x86b0b0); returns 0. Above 4: 0.
/// Original: `ConmebolMerc::update_staff_records_for_status` (0x4c5960, conmebol_merc.cpp, thiscall, ret 0x18).
unsafe extern "thiscall" fn conmebol_merc_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, _a4: u32, game: usize, final_: u32) -> u32 {
    let g = Fixture(game);
    let (d, f) = (outcome as u8, stage as u8 as i8);
    let comp = read_usize(this + 4);
    if f < 4 {
        if d != 6 {
            return u32::MAX;
        }
        covered!("cqx_12 mercosur group qualified");
        european_cup_add_qualified_record(club, comp, 0, 0x78, 0x28);
    } else if f == 4 {
        match d {
            0xff => {
                let weight = scale_by_round(0x32, 0x64, g.round_number(), 2);
                european_cup_add_knocked_out_record(club, comp, g.round_prefix() as u16 as u32, g.round_name() as u16 as u32, weight);
            }
            0 => {
                let weight = scale_by_round(0x32, 0x64, g.round_number(), 2);
                let knockout = read_usize(read_usize(this + cup::STAGES) + 0x10);
                let name = read_u16(read_usize(knockout + cup::ROUND_TABLE).wrapping_add((g.round_number() as isize * cup_round::SIZE as isize) as usize) + 0x6f) as u32;
                european_cup_add_qualified_record(club, comp, g.round_prefix() as u16 as u32, name, weight);
            }
            1 => cup_record_final_result(0x86a800, club, g, final_),
            2 => cup_record_final_result(0x86b0b0, club, g, final_),
            _ => {}
        }
    }
    0
}

// simplification: the records go to the history: they cannot run twice. Only the original; the plan: the return (−1
// in the groups for an outcome other than 6, otherwise 0).
unsafe extern "thiscall" fn compare_conmebol_merc_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, a4: u32, game: usize, final_: u32) -> u32 {
    let (d, f) = (outcome as u8, stage as u8 as i8);
    let plan = if f < 4 && d != 6 { u32::MAX } else { 0 };
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_CONMEBOL_MERC_UPDATE_STAFF_RECORDS_FOR_STATUS)(this, club, outcome, stage, a4, game, final_);
    SCOREBOARD.record(r == plan, || format!("conmebol_merc_update_staff_records_for_status({:#x}, {}, {}) returned {:#x}, plan {:#x}", club, d, f, r, plan));
    r
}

// ---------- batch 291: the mini league table of a set of clubs (sub_league.cpp) ----------

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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize, u32, u32, u32, u32, i32, u32, u32, usize, u32, u32) -> usize>(IDX_CONSTRUCT_SUB_LEAGUE);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, usize, u32, u32, u32, u32, usize)>(IDX_EUROPEAN_CUP_BUILD_FATE_NEWS_TEXT);
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
