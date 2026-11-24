// ---------- sbG_1: filling a mini league from the fixtures its clubs played among themselves (sub_league.cpp) ----------

/// The size of the SubLeague object (operator new 0xee before SubLeague::SubLeague at 0x46f4db).
pub(crate) const SUB_LEAGUE_SIZE: usize = 0xee;

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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, i32, usize, usize, u32, u32, usize) -> u32>(%%I0%%);
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
