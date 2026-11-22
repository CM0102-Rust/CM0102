// ---------- batch %%LEVA%%: the club history: recording a finished competition's season (club_history.cpp) ----------

/// The club history store as `club_history_record_comp_season` uses it (`G_CLUB_HISTORY`): +0 the league history file
/// list, +4 the cup history file list, +8 the per-club records (16 bytes, the club id the index: +0/+4 the first and
/// last file offsets of the club's league seasons, +8/+0xc those of its cup runs).
pub(crate) mod club_history_season {
    pub(crate) const LEAGUE_LIST: usize = 0x0;
    pub(crate) const CUP_LIST: usize = 0x4;
    pub(crate) const RECORDS: usize = 0x8;
    pub(crate) const RECORD_SIZE: usize = 0x10;
    pub(crate) const LEAGUE_FIRST: usize = 0x0;
    pub(crate) const LEAGUE_LAST: usize = 0x4;
    pub(crate) const CUP_FIRST: usize = 0x8;
    pub(crate) const CUP_LAST: usize = 0xc;
    /// "E:\dev\CM3\cm3 00-01\cm3\code\club_history.cpp" (the error boxes).
    pub(crate) const CLUB_HISTORY_CPP: usize = 0x98b644;
}

/// The season of a finished competition in each club's history, with the file list append and the table sort given
/// from outside (the copy appends and sorts; the comparer's dry run records the appends and sorts).
///
/// See `club_history_record_comp_season`.
unsafe fn club_history_record_comp_season_with(this: usize, comp: usize, sort: &mut dyn FnMut(usize),
                                               add: &mut dyn FnMut(usize, usize, usize, &[u8])) -> u32 {
    use club_history_season as ch;
    use league as l;
    if comp == 0 {
        show_error_at(ch::CLUB_HISTORY_CPP, 0xab);
        return 0;
    }
    if read_u8(comp + l::STAGE_INDEX) != 0xff {
        show_error_at(ch::CLUB_HISTORY_CPP, 0xb3);
        return 0;
    }
    let competition = read_usize(comp + l::COMPETITION);
    if exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), competition) != 0 {
        return 0;
    }
    let comp_id = read_u32(competition);
    let year = read_u16(comp + l::YEAR);
    let records = read_usize(this + ch::RECORDS);
    let polish = read_u32(KEY_CLUB_COMP_POLISH_FIRST_DIVISION);
    if exe::comp_util_is_league_division(comp_id as i32) != 0 {
        // a league: the table of the league and of each following stage that is a league
        let mut stage = comp;
        loop {
            let recorded = if read_i16(stage + l::ROUND_COUNT) > 7 && comp_id != polish {
                true
            } else {
                comp_id == polish && read_u8(stage + l::STAGE_INDEX) == 1 // tel42_2: the Polish second stage
            };
            if recorded {
                covered!("tel42_2 league table into the history");
                sort(stage);
                let mut rec = [0u8; 0x15];
                rec[0..4].copy_from_slice(&comp_id.to_le_bytes());
                rec[4..6].copy_from_slice(&year.to_le_bytes());
                for k in 0..read_i16(stage + l::CLUB_COUNT).max(0) as usize {
                    let row = read_usize(stage + l::TABLE) + k * league_table_row::SIZE;
                    let club = records + read_u32(read_usize(row + league_table_row::CLUB)) as usize * ch::RECORD_SIZE;
                    rec[6] = read_u8(row + league_table_row::POSITION); // tel42_2: the final position
                    for (i, b) in rec[7..0x15].iter_mut().enumerate() {
                        *b = read_u8(row + 0xd + i);
                    }
                    if read_u32(read_usize(stage + l::COMPETITION)) == polish {
                        // the Polish second stage counts the 14 games of the first one
                        covered!("tel42_2 Polish games played");
                        rec[0xb] = rec[0xb].wrapping_add(0xe);
                    }
                    add(read_usize(this + ch::LEAGUE_LIST), club + ch::LEAGUE_FIRST, club + ch::LEAGUE_LAST, &rec);
                }
            }
            let index = read_i8(stage + l::STAGE_INDEX) as i32;
            if index >= read_i32(comp + l::CURRENT_STAGE) {
                return 1;
            }
            let next = exe::comp_get_stage(comp, index + 1);
            if read_u8(next + l::COMP_TYPE) != 2 {
                return 1;
            }
            stage = next;
        }
    }
    if read_u32(comp + cup::SEASON_SET_UP) == 0 {
        return 0;
    }
    let marks = exe::crt_calloc(read_u32(G_CLUB_COUNT), 4);
    if marks == 0 {
        show_error_at(ch::CLUB_HISTORY_CPP, 0xfd);
        return 0;
    }
    covered!("tel42_2 cup runs into the history");
    let mut rec = [0u8; 0xb];
    rec[0..4].copy_from_slice(&comp_id.to_le_bytes());
    rec[4..6].copy_from_slice(&year.to_le_bytes());
    // one record per club, from the last stage back to the competition itself (-1): a club keeps the furthest one
    let mut add_club = |club: usize, status: u8, label: u16, round: u16| {
        let id = read_u32(club) as usize;
        if read_u32(marks + id * 4) != 0 {
            return;
        }
        rec[6] = status;
        rec[7..9].copy_from_slice(&label.to_le_bytes());
        rec[9..0xb].copy_from_slice(&round.to_le_bytes());
        let r = records + id * ch::RECORD_SIZE;
        add(read_usize(this + ch::CUP_LIST), r + ch::CUP_FIRST, r + ch::CUP_LAST, &rec);
        write_u32(marks + id * 4, 1);
    };
    let mut s = read_i32(comp + cup::CURRENT_STAGE);
    while s >= -1 {
        let stage = exe::comp_get_stage(comp, s);
        if read_u8(stage + l::COMP_TYPE) == 2 {
            covered!("tel42_2 cup group stage");
            for k in 0..read_i16(stage + l::CLUB_COUNT).max(0) as usize {
                let row = read_usize(stage + l::TABLE) + k * league_table_row::SIZE;
                add_club(read_usize(row + league_table_row::CLUB), read_u8(row + league_table_row::STATUS),
                         read_u16(stage + l::STAGE_ID), 0xffff);
            }
        } else {
            let mut r = read_i16(stage + cup::ROUND_COUNT) as i32 - 1;
            while r >= 0 {
                let round = read_usize(stage + cup::ROUND_TABLE) + r as usize * cup_round::SIZE;
                let ties = read_usize(read_usize(stage + cup::TIE_TABLES) + r as usize * 4);
                for t in 0..read_i16(round + cup_round::TIES).max(0) as usize {
                    let tie = ties + t * 0x35;
                    for side in [cup_tie::HOME_TEAM, cup_tie::AWAY_TEAM] {
                        let team = read_i16(tie + side);
                        if team == -1 {
                            continue;
                        }
                        let entry = read_usize(stage + cup::TEAM_TABLE) + team as isize as usize * 6;
                        // tel42_2: the team's outcome and the round it reached
                        add_club(read_usize(entry), read_u8(entry + 5), read_u16(stage + cup::STAGE_LABEL),
                                 read_u16(round + cup_round::ROUND_CODE));
                    }
                }
                r -= 1;
            }
        }
        s -= 1;
    }
    exe::crt_free(marks);
    1
}

/// Adds the finished season of a competition to each club's history.
///
/// `Club_History::record_comp_season(comp)`: no competition: the club_history.cpp 0xab box and 0; a stage instead of
/// the competition itself (`league::STAGE_INDEX` ≠ 0xff): the 0xb3 box and 0; a friendly competition
/// (`Friendly::is_friendly_competition`): 0.
///
/// A league division (`comp_util_is_league_division` of the competition id): from the competition itself, each stage
/// whose table is recorded: with more than 7 rounds (`league::ROUND_COUNT`), except in the Polish first division, where
/// only its stage 1 is. The table is sorted (`League::sort_table`, 0) and each row gives a league record (21 bytes:
/// competition id, the competition's year, the row's position, then the 14 bytes of the totals from row +0xd: played,
/// won, drawn, lost, goals for and against, points; in the Polish first division played + 14) appended to the club's
/// league seasons (`FILE_LLIST::add_node` on the league list with the club's first and last offsets). Then the next
/// stage (`Comp::get_stage`, index + 1) while it exists (index below `league::CURRENT_STAGE`) and is a league
/// (`league::COMP_TYPE` = 2). Returns 1.
///
/// A cup: not set up for the season (`cup::SEASON_SET_UP` = 0): 0. A mark per club (`calloc`; none: the 0xfd box and
/// 0). From the current stage (`cup::CURRENT_STAGE`) back to -1 (the cup itself): in a league stage each table row's
/// club gets (competition id, year, the row's status `league_table_row::STATUS`, the stage id `league::STAGE_ID`,
/// -1); in a knockout stage the rounds from the last back, each tie's home and away team (index ≠ -1, the team table's
/// club) gets (id, year, the team's outcome +5, the stage label `cup::STAGE_LABEL`, the round code
/// `cup_round::ROUND_CODE`). Only the first record of each club (the furthest it got) goes to its cup runs (the cup
/// list, the club's +8/+0xc offsets). The marks freed; returns 1.
/// Original: `Club_History::record_comp_season` (0x442e80, club_history.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn club_history_record_comp_season(this: usize, comp: usize) -> u32 {
    club_history_record_comp_season_with(this, comp, &mut |stage| { exe::league_sort_table(stage, 0usize); },
        &mut |list, first, last, rec| { exe::file_llist_add_node(list, first, last, rec.as_ptr()); })
}

/// Comparer for `club_history_record_comp_season`: the copy plans, the original runs, the plan is checked in the files.
///
/// The records go into the history files (`FILE_LLIST::add_node` appends and moves the club's offsets): the routine
/// cannot run twice. The copy runs first as a dry run: it sorts the tables as the original does (`League::sort_table`
/// is a total order ending on the club id and stores the positions in that order: sorting again changes nothing) and
/// keeps each record it would append, with its club offsets; the original runs afterwards from the same state. Then,
/// for each club offset pair of the plan, the club's nodes are read back from the file (`FILE_LLIST::get_nodes`): the
/// newest ones must be the planned records, in order, and the returns equal. Without a competition or on a stage: only
/// the original (the error boxes).
pub(crate) unsafe extern "thiscall" fn compare_club_history_record_comp_season(this: usize, comp: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%);
    if comp == 0 || read_u8(comp + league::STAGE_INDEX) != 0xff {
        return o(this, comp);
    }
    // (list, first offset address, last offset address, record)
    let mut plan: Vec<(usize, usize, usize, Vec<u8>)> = Vec::new();
    let rr = club_history_record_comp_season_with(this, comp, &mut |stage| { exe::league_sort_table(stage, 0usize); },
        &mut |list, first, last, rec| plan.push((list, first, last, rec.to_vec())));
    let ro = o(this, comp);
    let mut wrong: Vec<String> = Vec::new();
    let mut done: Vec<usize> = Vec::new();
    for (list, first, last, _) in &plan {
        if done.contains(first) {
            continue;
        }
        done.push(*first);
        let wanted: Vec<&Vec<u8>> = plan.iter().filter(|p| p.1 == *first).map(|p| &p.3).collect();
        let nodes = exe::file_llist_get_nodes(*list, read_u32(*first), read_u32(*last));
        let mut got: Vec<Vec<u8>> = Vec::new();
        if nodes != 0 {
            write_usize(nodes + llist::CURSOR, read_usize(nodes + llist::TAIL));
            while got.len() < wanted.len() {
                let d = exe::llist_get_prev(nodes);
                if d == 0 {
                    break;
                }
                got.push((0..wanted[0].len()).map(|i| read_u8(d + i)).collect());
            }
            exe::llist_scalar_deleting_destructor(nodes, 1);
        }
        got.reverse();
        if got.len() != wanted.len() || got.iter().zip(wanted.iter()).any(|(g, w)| g != *w) {
            wrong.push(format!("club offsets {:#x}: planned {:02x?} in the file {:02x?}", first, wanted, got));
        }
    }
    SCOREBOARD.record(ro == rr && wrong.is_empty(), || format!("club_history_record_comp_season(comp {:#x}) original={} rust={} {} appends planned: {}",
        comp, ro, rr, plan.len(), wrong.join("; ")));
    ro
}
