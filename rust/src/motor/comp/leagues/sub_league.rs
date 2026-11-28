use super::*;

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
pub(crate) unsafe extern "thiscall" fn construct_sub_league(this: usize, count: u32, club_ids: usize, a3: u32, a4: u32, a5: u32, a6: u32, comp_id: i32, win_points: u32,
    draw_points: u32, tiebreaks: usize, a11: u32, a12: u32) -> usize {
    let this = League(this);
    let l = this;
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LEAGUE_LEAGUE);
    base(this.addr());
    this.set_win_points(win_points as u8); // cqx14: the win points
    this.set_draw_points(draw_points as u8);
    write_u32(this.addr(), 0x96ff90);
    for k in 0..4 {
        write_u8(this.addr() + league::TIEBREAKS + k, read_u8(tiebreaks + k));
    }
    write_u32(this.addr() + league::PROMOTED_COUNT, 0);
    this.set_stage_count((0) as usize);
    this.set_current_stage((u32::MAX) as usize);
    this.set_stage_index(0xfe);
    this.set_legs(0u16 as i16);
    this.set_stages((0) as usize);
    write_u32(this.addr() + 8, 0);
    this.set_complete((1) as usize);
    let comp = if comp_id >= 0 && comp_id < read_i32(0xae2378) { read_usize(G_CLUB_COMP_TABLE).wrapping_add((comp_id as usize).wrapping_mul(0x6b)) } else { 0 };
    write_usize(this.addr() + 4, comp);
    this.set_club_count((count as u16) as i16);
    let t = exe::crt_malloc((l.club_count() as i32 as u32).wrapping_mul(league_table_row::SIZE as u32));
    l.set_table(t);
    if t == 0 {
        show_error_at(0xa8b464, 0x56);
        return this.addr();
    }
    let n = l.club_count() as i32;
    let mut i = 0i8;
    while (i as i32) < n {
        covered!("cqx_14 sub league row");
        let club = read_usize(G_CLUB_TABLE).wrapping_add(read_usize(club_ids.wrapping_add((i as isize * 4) as usize)).wrapping_mul(club::SIZE));
        exe::league_init_table_row(l, i as u8 as u32, club, 0, 0);
        i = i.wrapping_add(1);
    }
    if exe::subleague_add_results_between_members(this, a3, a4, a5, a6, comp_id, a11, a12) == 0 {
        show_error_at(0xa8b464, 0x63);
        return this.addr();
    }
    if exe::league_sort_table(l, 0) == 0 {
        show_error_at(0xa8b464, 0x68);
    }
    this.addr()
}

/// Comparer for `construct_sub_league`: the original builds `this`; the copy builds a scratch object (`exe::crt_malloc`
/// of 0x200 zeroed bytes) from the same arguments; the counts, points, tie-breaks, competition and the table rows
/// (`sub_league_bytes`) must be equal; the copy's table is freed (League::free_table `exe::league_free_table`(copy, 0))
/// and then the scratch object.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_construct_sub_league(this: usize, count: u32, club_ids: usize, a3: u32, a4: u32, a5: u32, a6: u32, comp_id: i32, win_points: u32,
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

/// The bytes of a mini league to compare: the counts, points, tie-breaks and competition, and each row's club,
/// position and totals (without the row's head-to-head and history pointers, +5..+0xd).
pub(crate) unsafe fn sub_league_bytes(this: usize) -> Vec<u8> {
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

/// Fills a mini league's table from the played matches between its own clubs.
///
/// The fixtures come from `list` or, when it is null, from FIXTURE_MANAGER::scan_fixtures (`FN_FIXTURE_MANAGER_SCAN_FIXTURES`
/// on [`G_FIXTURE_MANAGER`], 18 arguments, into a local LLIST: from the day and year of `from`, forwards, competition
/// `comp_id`, rounds `round_min`..`round_max` (both −1 when `round_min` is −1, with the three unread arguments 0 instead of
/// −1), any clubs, at most `max`, played only (1), stage −2, either side, without strangers). The list cursor goes back to
/// its head; for each fixture, when `to` is given, one dated after it (Date::Date `exe::new_date`(day, year) of the
/// fixture; the year compared unsigned, the day signed) is skipped; an unplayed one (`fixture::HOME_GOALS` = 0xff) too.
/// Otherwise the table (`league_table_row::SIZE`-byte rows, `club_count()`) is searched for the home and the away club
/// (`fixture::HOME_CLUB`, `fixture::AWAY_CLUB`), stopping when both are found; with both, the match goes into the table
/// through virtual `comp_vtable::ADD_RESULT`(fixture, 0). The local list is destroyed; returns 1.
/// Original: `SubLeague::add_results_between_members` (0x88c9f0, sub_league.cpp, thiscall, ret 0x1c).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn sub_league_add_results_between_members(this: usize, round_min: i32, round_max: i32, from: usize, to: usize, comp_id: u32,
    max: u32, list: usize) -> u32 {
    let league = League(this);
    let mut own = [0u8; 0x10];
    let own = own.as_mut_ptr() as usize;
    exe::new_llist(own);
    let list = if list != 0 {
        list
    } else {
        covered!("sbG_1 sub league scan");
        // simplification: the original passes dwords with stack leftovers above the words; scan_fixtures only reads the words
        let (mut year, mut day, mut slice) = (read_i16(from + 2) as u16 as u32, read_i16(from) as u16 as u32, 0u8);
        let (round_min, round_max, unread) = if round_min == -1 { (-1, -1, 0) } else { (round_min, round_max, -1) };
        let scan: unsafe extern "thiscall" fn(usize, *mut u32, *mut u32, *mut u8, u32, u32, i32, i32, i32, i32, i32, i32, i32, u32, usize, u32, i32, i32, u32) -> u32 =
            core::mem::transmute(FN_FIXTURE_MANAGER_SCAN_FIXTURES);
        scan(read_usize(G_FIXTURE_MANAGER), &mut year, &mut day, &mut slice, 1, comp_id, round_min, round_max, -1, -1, unread, unread, unread, max, own, 1, -2, -1, 0);
        own
    };
    write_usize(list + llist::CURSOR, read_usize(list + llist::HEAD));
    let mut f = exe::llist_get_next(list);
    while f != 0 {
        let mut date = [0u8; 8];
        let after = to != 0 && {
            exe::new_date(date.as_mut_ptr() as usize, read_i16(f + fixture::DAY) as u16 as u32, read_i16(f + fixture::YEAR) as u16 as u32);
            let (day, year) = (i16::from_le_bytes([date[0], date[1]]), u16::from_le_bytes([date[2], date[3]]));
            year > read_u16(to + 2) || (year == read_u16(to + 2) && day > read_i16(to))
        };
        if !after && read_u8(f + fixture::HOME_GOALS) != 0xff {
            let (mut home, mut away) = (false, false);
            let mut i = 0i32;
            while i < league.club_count() as i32 {
                let club = read_usize(league.table() + i as usize * league_table_row::SIZE);
                home |= club == read_usize(f + fixture::HOME_CLUB);
                away |= club == read_usize(f + fixture::AWAY_CLUB);
                if home && away {
                    break;
                }
                i += 1;
            }
            if home && away { // sbG_1: both clubs in the table
                covered!("sbG_1 sub league result added");
                let add: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::ADD_RESULT));
                add(this, f, 0);
            }
        }
        f = exe::llist_get_next(list);
    }
    exe::drop_llist(own);
    1
}

/// Comparer for `sub_league_add_results_between_members`: side by side. Before the original fills `this`, a scratch
/// copy of the object (`SUB_LEAGUE_SIZE` bytes, `exe::crt_malloc`) gets its own copy of the table (League::copy_table
/// `exe::league_copy_table`, with new head-to-head and form buffers; the `league::GAMES_PER_CLUB` it may set is put
/// back on `this`); the copy fills the scratch object; the counts, points and rows (`sub_league_bytes`) and the return
/// must be equal. The scratch table (`exe::league_free_table`) and object are freed.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_sub_league_add_results_between_members(this: usize, round_min: i32, round_max: i32, from: usize, to: usize, comp_id: u32,
    max: u32, list: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, i32, usize, usize, u32, u32, usize) -> u32>(IDX_SUB_LEAGUE_ADD_RESULTS_BETWEEN_MEMBERS);
    let league = League(this);
    let scratch = if league.table() != 0 && league.club_count() > 0 { exe::crt_malloc(SUB_LEAGUE_SIZE as u32) } else { 0 };
    if scratch == 0 {
        return o(this, round_min, round_max, from, to, comp_id, max, list);
    }
    core::ptr::copy_nonoverlapping(this as *const u8, scratch as *mut u8, SUB_LEAGUE_SIZE);
    let games = league.games_per_club();
    League(scratch).set_table(exe::league_copy_table(this));
    league.set_games_per_club(games);
    let r = o(this, round_min, round_max, from, to, comp_id, max, list);
    let c = sub_league_add_results_between_members(scratch, round_min, round_max, from, to, comp_id, max, list);
    let (a, b) = (sub_league_bytes(this), sub_league_bytes(scratch));
    if League(scratch).table() != 0 {
        exe::league_free_table(League(scratch), 0);
    }
    exe::crt_free(scratch);
    SCOREBOARD.record(r == c && a == b, || format!("sub_league_add_results_between_members({:#x}, rounds {}..{}, comp {}, list {:#x}): original {} rust {}, the tables differ: {}",
        this, round_min, round_max, comp_id as i32, list, r, c, a != b));
    r
}

/// The size of the SubLeague object (operator new 0xee before SubLeague::SubLeague at 0x46f4db).
pub(crate) const SUB_LEAGUE_SIZE: usize = 0xee;
