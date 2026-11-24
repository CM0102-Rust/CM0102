// ---------- sbH_2: the Finnish First Division promotion and relegation play-offs (fin_first.cpp) ----------

/// The latitude splitting the Finnish clubs into north and south (the double 61.08 at 0x96a4b8).
const FINNISH_NORTH_LATITUDE: usize = 0x96a4b8;
/// The ids of the four Kakkonen clubs promoted to the Ykkönen (four dwords at League +0xee of the Ykkönen; read by
/// FinPrm::refresh_first_division_clubs on the turnover).
const FIN_FIRST_PROMOTED_IDS: usize = 0xee;

/// The side of the Finnish split a club lies on, by the latitude of its stadium's city (`club::STADIUM`,
/// `stadium::CITY`, `city::LATITUDE`): `Some(true)` north (above `FINNISH_NORTH_LATITUDE`), `Some(false)` south (at or
/// below it, or not a number: the x87 compare of the original), `None` without a stadium or a city.
unsafe fn finnish_club_is_north(club: usize) -> Option<bool> {
    let stadium = Club(club).stadium();
    if stadium.0 == 0 || stadium.city() == 0 {
        return None;
    }
    Some(City(stadium.city()).latitude() > read_f64(FINNISH_NORTH_LATITUDE))
}

/// The Kakkonen clubs, at most 50 in club table order (indexes below `G_CLUB_COUNT` − 2 × `G_NATION_COUNT`): those with
/// a nation (`club::NATION`) and a division (`club::DIVISION`) whose id is that of one of the four Kakkonen groups
/// ([`KEY_CLUB_COMP_FINNISH_SECOND_DIVISION_NORTH`], `_SOUTH`, `_WEST`, `_EAST`).
unsafe fn kakkonen_clubs() -> ([usize; 50], usize) {
    let groups = [KEY_CLUB_COMP_FINNISH_SECOND_DIVISION_NORTH, KEY_CLUB_COMP_FINNISH_SECOND_DIVISION_SOUTH,
                  KEY_CLUB_COMP_FINNISH_SECOND_DIVISION_WEST, KEY_CLUB_COMP_FINNISH_SECOND_DIVISION_EAST];
    let (mut list, mut n) = ([0usize; 50], 0);
    for i in 0..read_i32(G_CLUB_COUNT) - 2 * read_i32(G_NATION_COUNT) {
        let club = club_at(i as isize);
        let division = club.division().addr();
        if club.nation().addr() != 0 && division != 0 && groups.iter().any(|&g| read_u32(division) == read_u32(g)) {
            list[n] = club.addr();
            n += 1;
            if n == list.len() {
                break;
            }
        }
    }
    (list, n)
}

/// Takes from `list` (its first `n` slots) the first club that `pick` accepts: the slot is emptied and the club
/// returned.
fn take_club(list: &mut [usize; 50], n: usize, pick: impl Fn(usize) -> bool) -> Option<usize> {
    let k = (0..n).find(|&k| list[k] != 0 && pick(list[k]))?;
    Some(core::mem::replace(&mut list[k], 0))
}

/// Picks the four Kakkonen clubs going up (their ids into `FIN_FIRST_PROMOTED_IDS`, in this order): up to two from the
/// north, then from the south up to two in all, then any up to four; each club taken leaves `list`.
unsafe fn pick_promoted_kakkonen_clubs(l: League, list: &mut [usize; 50], n: usize) {
    let mut done = 0;
    for (side, upto) in [(Some(true), 2), (Some(false), 2), (None, 4)] {
        while done < upto {
            let Some(club) = take_club(list, n, |c| side.is_none() || finnish_club_is_north(c) == side) else { break };
            write_u32(l.addr() + FIN_FIRST_PROMOTED_IDS + 4 * done, read_u32(club));
            done += 1;
        }
    }
}

/// The club of row `k` of a league's table.
unsafe fn table_club(l: League, k: usize) -> usize {
    LeagueTableRow(l.table() + k * league_table_row::SIZE).club() as usize
}

/// Rule: the Finnish First Division (Ykkönen) play-offs. Four Kakkonen clubs go up: two from the north and two from
/// the south (by latitude), among the strongest by reputation shuffled, completed by any. The promotion play-off pairs
/// the top two of the north group with the top two of the south group, crossed (first against second); the relegation
/// play-off pairs the eighth of each group with a remaining Kakkonen club from the north and one from the south.
///
/// `FinFirst::create_play_off_stages()`: `kakkonen_clubs`; fewer than 6 (or no index to look at): the fin_first.cpp box
/// of line 0x312. `exe::comp_util_sort_and_shuffle_top_clubs`(list, n, 6) (cdecl); `pick_promoted_kakkonen_clubs`. The
/// list of four (`exe::crt_malloc`(0x10); none: the box of line 0x33f): this league's (the north group) table rows 0
/// and 1 and stage 0's (the south group) rows 1 and 0, as [this 0, south 1, this 1, south 0]; stage 1
/// (`league_knockout_stage`(1, 4 clubs, flag 0)) into slot 1 (without it the box of line 0x352; the list and the round
/// table not freed); the table freed. The list again: this row 7, the first Kakkonen club left in the north, stage 0's
/// row 7, the first left in the south; an empty slot of the two takes the first club left, in order; stage 2 into slot
/// 2 (the box of line 0x393); the table freed, Cup::set_up_round (`exe::cup_set_up_round`(stage, 0)) on stages 1 and
/// 2, the list freed and `league::CURRENT_STAGE` = 2.
/// Original: `FinFirst::create_play_off_stages` (0x594080, fin_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fin_first_create_play_off_stages(this: usize) {
    let l = League(this);
    let (mut list, n) = kakkonen_clubs();
    if n < 6 {
        return show_error_at(FIN_FIRST_SOURCE, 0x312);
    }
    exe::comp_util_sort_and_shuffle_top_clubs(list.as_mut_ptr() as usize, n as u32, 6);
    pick_promoted_kakkonen_clubs(l, &mut list, n);
    let pair = exe::crt_malloc(0x10);
    if pair == 0 {
        return show_error_at(FIN_FIRST_SOURCE, 0x33f);
    }
    let south = League(read_usize(l.stages()));
    for (k, club) in [table_club(l, 0), table_club(south, 1), table_club(l, 1), table_club(south, 0)].into_iter().enumerate() {
        write_usize(pair + 4 * k, club);
    }
    let (promotion, table) = league_knockout_stage(l, 1, 4, pair, 0);
    write_usize(l.stages() + 4, promotion);
    if promotion == 0 {
        return show_error_at(FIN_FIRST_SOURCE, 0x352);
    }
    exe::crt_free(table);
    let mut clubs = [table_club(l, 7), 0, table_club(south, 7), 0];
    clubs[1] = take_club(&mut list, n, |c| finnish_club_is_north(c) == Some(true)).unwrap_or(0);
    clubs[3] = take_club(&mut list, n, |c| finnish_club_is_north(c) == Some(false)).unwrap_or(0);
    for slot in [1, 3] {
        if clubs[1] != 0 && clubs[3] != 0 {
            break;
        }
        if clubs[slot] == 0 {
            clubs[slot] = take_club(&mut list, n, |_| true).unwrap_or(0);
        }
    }
    for (k, &club) in clubs.iter().enumerate() {
        write_usize(pair + 4 * k, club);
    }
    let (relegation, table) = league_knockout_stage(l, 2, 4, pair, 0);
    write_usize(l.stages() + 8, relegation);
    if relegation == 0 {
        return show_error_at(FIN_FIRST_SOURCE, 0x393);
    }
    covered!("sbH_2 finnish first division play-offs");
    exe::crt_free(table);
    exe::cup_set_up_round(read_usize(l.stages() + 4), 0);
    exe::cup_set_up_round(read_usize(l.stages() + 8), 0);
    exe::crt_free(pair);
    l.set_current_stage(2);
}

/// Comparer for `fin_first_create_play_off_stages`: only the original (the Kakkonen clubs are shuffled by the game's
/// random numbers, and it builds the stages); afterwards stages 1 and 2 must exist, 2 be the current one, stage 1 hold
/// the four group clubs, stage 2 the two eighths, and the four promoted ids be four different Kakkonen clubs.
pub(crate) unsafe extern "thiscall" fn compare_fin_first_create_play_off_stages(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this);
    let l = League(this);
    let south = League(read_usize(l.stages()));
    let (s1, s2) = (read_usize(l.stages() + 4), read_usize(l.stages() + 8));
    let teams = |s: usize| -> Vec<usize> { (0..4).map(|k| read_usize(Cup(s).team_table() + 6 * k + cup_team_entry::CLUB)).collect() };
    let (mut plan1, mut got1) = (vec![table_club(l, 0), table_club(south, 1), table_club(l, 1), table_club(south, 0)], if s1 != 0 { teams(s1) } else { Vec::new() });
    plan1.sort();
    got1.sort();
    let got2 = if s2 != 0 { teams(s2) } else { Vec::new() };
    let eighths = got2.contains(&table_club(l, 7)) && got2.contains(&table_club(south, 7));
    let (list, n) = kakkonen_clubs();
    let mut ids: Vec<u32> = (0..4).map(|k| read_u32(this + FIN_FIRST_PROMOTED_IDS + 4 * k)).collect();
    let kakkonen = ids.iter().all(|&id| list[..n].iter().any(|&c| read_u32(c) == id));
    ids.sort();
    ids.dedup();
    SCOREBOARD.record(s1 != 0 && s2 != 0 && l.current_stage() as i32 == 2 && got1 == plan1 && eighths && kakkonen && ids.len() == 4,
        || format!("fin_first_create_play_off_stages({:#x}): stages {:#x} {:#x}, current {}, stage 1 {:x?} copy {:x?}, stage 2 {:x?}, promoted {:x?}", this, s1, s2, l.current_stage() as i32, got1, plan1, got2, ids));
}
