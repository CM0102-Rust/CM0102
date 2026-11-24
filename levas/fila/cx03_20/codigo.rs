// ---------- cx03_20: the squads of the MLS All-Star Game (usa_mls_all_stars.cpp) ----------

/// The 20 position masks of an All-Star squad, in the order `match_man_assign_players_to_positions` fills them.
pub(crate) const ALL_STAR_SQUAD_POSITIONS: [u16; 20] = [1, 1, 1, 0x804, 0x84, 4, 4, 0x204, 0x204, 0x204, 0x810, 0x90, 0x10, 0x10, 0x210, 0x210, 0x40, 0x40, 0x40, 0x40];

/// The players one All-Star selection can call up: for each of the first four rows of the tables, two passes; the
/// first over `first`'s row, the second over the same row of `second` when that club's All-Star conference
/// (`usa_mls_get_club_all_star_conference`) is `conference`; each player of the club's squad (`club.squad`, 50 slots)
/// with a player record, not out injured (`exe::injury_get_player_injury_status` ≠ 2), free of international duty on
/// `date` (`exe::nationalteams_get_international_duty_club`(…, player, date, 0, 0, 0) = 0) and still at that club
/// (`staff.club_job()`) joins `list`. A club met in both passes is collected twice, as in the original. Returns the count
/// (a signed word in the original).
unsafe fn collect_all_star_candidates(list: &mut [usize], first: League, second: League, conference: u16, date: &[u16; 4]) -> i16 {
    let mut n: i16 = 0;
    for row in 0..4usize {
        for pass in 0..2 {
            let table = if pass == 0 { first } else { second };
            let club = read_usize(table.table() + row * league_table_row::SIZE);
            if pass == 1 && usa_mls_get_club_all_star_conference(club) != conference {
                continue;
            }
            for slot in 0..50usize {
                let person = Staff(Club(club).squad(slot));
                if person.is_null() || person.player().is_null() {
                    continue;
                }
                if exe::injury_get_player_injury_status(read_usize(G_INJURY), person) == 2 {
                    continue;
                }
                if exe::nationalteams_get_international_duty_club(read_usize(G_NATIONAL_TEAMS), person, date.as_ptr() as usize, 0usize, 0, 0) != 0 {
                    continue;
                }
                if person.club_job().addr() == club {
                    list[n as usize] = person.addr();
                    n = n.wrapping_add(1);
                }
            }
        }
    }
    n
}

/// Fills one All-Star club's squad: the club (`club_at` of the id kept in `key`) loses its whole squad (the 50 slots
/// cleared); the candidates are sorted (`exe::crt_qsort`, 4-byte entries, `qsort_comparator_squad_by_player_0xb`
/// 0x5402f0); at most 127 are used, and fewer than 30 show the box of `row` (the squad is still filled); then the squad
/// is filled from them (`exe::match_man_assign_players_to_positions`(list, n, the 20 `ALL_STAR_SQUAD_POSITIONS`, 20, 0,
/// the club's squad, 2, then eight zeros), cdecl).
unsafe fn fill_all_star_squad(list: &mut [usize], n: i16, key: usize, row: u32) {
    let club = club_at(read_usize(key) as isize);
    for slot in 0..50usize {
        write_usize(club.addr() + 0xd7 + slot * 4, 0); // the squad slots
    }
    exe::crt_qsort(list.as_mut_ptr() as usize, n as i32 as u32, 4, 0x5402f0usize);
    let mut n = n;
    if n > 0x7f {
        n = 0x7f;
    } else if n < 0x1e {
        show_error_at(0xaa20a0, row);
    }
    exe::match_man_assign_players_to_positions(list.as_mut_ptr() as usize, n as i32 as u32, ALL_STAR_SQUAD_POSITIONS.as_ptr() as usize, 0x14, 0,
        club.addr() + 0xd7, 2, 0usize, 0, 0, 0, 0, 0, 0, 0);
}

/// rule: the two MLS All-Star selections are called up from the clubs in the first four places: the East from the
/// MLS's own table and from the first conference table's clubs of the East conference, the West from the second
/// conference table and from the first conference table's clubs of the West conference; each selection takes the fit
/// players free of international duty on the day of the game, sorted, at most 127 of them, and picks a squad of 20 by
/// position (`ALL_STAR_SQUAD_POSITIONS`).
///
/// The game day: the first round's date (`cup::ROUND_TABLE`: the day +3 and the year offset +5 on `cup::YEAR`;
/// Date::Date `exe::new_date`, thiscall, ret 8). The MLS (`G_COMP_OBJECT_TABLE`[`KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE`])
/// and its stages 0 and 1 (`cup::STAGES`). The East (`collect_all_star_candidates` over the MLS table and stage 0, the
/// conference 0x413; `fill_all_star_squad` into `G_KEY_CLUB_MLS_ALL_STARS_EAST`, the box of line 0x1dc), then the West
/// (over stage 1 and stage 0, 0x414; into `G_KEY_CLUB_MLS_ALL_STARS_WEST`, line 0x221). The original keeps both lists of
/// 300 on the stack one after the other: the copy's are larger, which only matters past 300 candidates.
/// Original: `UsaMlsAllStars::select_squads` (0x90a980, usa_mls_all_stars.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn usa_mls_all_stars_select_squads(this: usize) {
    let cup = Cup(this);
    let mls = League(read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_usize(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE) * 4));
    let first_stage = League(read_usize(mls.stages()));
    let second_stage = League(read_usize(mls.stages() + 4));
    let round = cup.round_table();
    let mut date = [0u16; 4];
    exe::new_date(date.as_mut_ptr() as usize, read_u16(round + cup_round::DAY) as u32,
        (read_u8(round + cup_round::YEAR_OFFSET) as i8 as i16).wrapping_add(cup.year()) as u16 as u32);
    covered!("cx03_20 all-star squads");
    let mut east = [0usize; 400];
    let n = collect_all_star_candidates(&mut east, mls, first_stage, 0x413, &date);
    fill_all_star_squad(&mut east, n, G_KEY_CLUB_MLS_ALL_STARS_EAST, 0x1dc);
    let mut west = [0usize; 400];
    let n = collect_all_star_candidates(&mut west, second_stage, first_stage, 0x414, &date);
    fill_all_star_squad(&mut west, n, G_KEY_CLUB_MLS_ALL_STARS_WEST, 0x221);
}

/// Comparer for `usa_mls_all_stars_select_squads`: side by side over the two All-Star clubs' squads (the 50 slots each);
/// the injury and duty checks and the position picker only read.
pub(crate) unsafe extern "thiscall" fn compare_usa_mls_all_stars_select_squads(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    let east = club_at(read_usize(G_KEY_CLUB_MLS_ALL_STARS_EAST) as isize).addr() + 0xd7;
    let west = club_at(read_usize(G_KEY_CLUB_MLS_ALL_STARS_WEST) as isize).addr() + 0xd7;
    let (_, _, same, mo, mc) = crate::comparison::side_by_side(&[(east, 200), (west, 200), (0xb67a34, 4)], || o(this), || usa_mls_all_stars_select_squads(this));
    SCOREBOARD.record(same, || format!("usa_mls_all_stars_select_squads({:#x}): original {:x?} copy {:x?}", this, mo, mc));
}
