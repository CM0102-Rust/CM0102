// ---------- rx11b_1: the reserve competition's fixtures: the reserve matches of a date and the B or Under 21 match of a national team match (record_utils.cpp) ----------

/// A club's reserve stadium (`Club` +0x6e, ReserveStadium in docs/FORMATOS.md; no name in `fields.rs` yet).
const CLUB_RESERVE_STADIUM: usize = 0x6e;

/// The stadium a reserve or youth fixture is played at when it is free: the home club's reserve stadium
/// (`CLUB_RESERVE_STADIUM`) when it is set and `FIXTURE_MANAGER::is_stadium_in_use` says no for the day and year dwords
/// given (all three slices); 0 otherwise.
unsafe fn free_reserve_stadium(home: Club, day: u32, year: u32) -> usize {
    let stadium = read_usize(home.addr() + CLUB_RESERVE_STADIUM);
    if stadium != 0 && exe::fixture_manager_is_stadium_in_use(read_usize(G_FIXTURE_MANAGER), stadium, day, year, u32::MAX) == 0 {
        return read_usize(home.addr() + CLUB_RESERVE_STADIUM);
    }
    0
}

/// The date of the reserve matches for the league matches of `date` (8 bytes): two days later when `date` is weekday 4
/// (`Date::get_weekday`), two days earlier otherwise.
unsafe fn reserve_match_date(date: usize) -> [u8; 8] {
    let mut when = [0u8; 8];
    exe::date_date_54a110(when.as_mut_ptr() as usize);
    let mut t = [0u8; 8];
    let r = if exe::date_get_weekday(date) as u16 == 4 {
        exe::date_add_days(date, t.as_mut_ptr() as usize, 2)
    } else {
        exe::date_sub_days(date, t.as_mut_ptr() as usize, 2)
    };
    core::ptr::copy_nonoverlapping(r as *const u8, when.as_mut_ptr(), 8);
    when
}

/// The reserve fixture of league fixture `game` on `when`, or None when neither club side has a free ground.
///
/// A copy of the fixture: outside Turkish competitions (`competition().nation()` = `KEY_NATION_TURKEY`) the clubs swap
/// grounds (the ids follow); the reserve competition (`KEY_CLUB_COMP_RESERVE`, its record), the date `when`, category
/// 1, no stage, stage index or side marks, no round name, prefix or number, no statistics slot, the 0x100/0x200 bits of
/// the squad rules clear. The ground: the home club's reserve stadium when it is free on the day (`free_reserve_stadium`),
/// else its stadium (`club::STADIUM`) when free; neither: None. In a Belgian competition (`KEY_NATION_BELGIUM`) the squad
/// rules' low byte becomes 0x44 (four substitutes, four substitutions).
unsafe fn reserve_fixture_of(game: Fixture, when: &[u8; 8]) -> Option<[u8; fixture::SIZE]> {
    let mut r = [0u8; fixture::SIZE];
    core::ptr::copy_nonoverlapping(game.addr() as *const u8, r.as_mut_ptr(), fixture::SIZE);
    let f = Fixture(r.as_mut_ptr() as usize);
    // the competition's nation is read without a null check, as in the original
    if read_i32(game.competition().nation().addr()) != read_i32(KEY_NATION_TURKEY) { // rx11b_1: the reserves play at the other ground
        let (home, away) = (f.home_club(), f.away_club());
        f.set_home_club(away);
        f.set_away_club(home);
        f.set_home_club_id(read_usize(away.addr()));
        f.set_away_club_id(read_usize(home.addr()));
    }
    let (day, year) = (read_u32(when.as_ptr() as usize), read_u32(when.as_ptr() as usize + 2));
    f.set_squad_rules(f.squad_rules() & 0xfcff);
    f.set_comp_id(read_i32(KEY_CLUB_COMP_RESERVE));
    f.set_day(day as u16);
    f.set_year(year as u16);
    f.set_home_side_mark(0);
    f.set_away_side_mark(0);
    f.set_stage(0xff);
    f.set_category(1);
    f.set_stage_index(0xff);
    f.set_round_number(0);
    f.set_match_stats_slot(usize::MAX);
    f.set_competition(Competition(key_comp_record(KEY_CLUB_COMP_RESERVE)));
    f.set_round_prefix(0);
    f.set_round_name(0);
    let home = f.home_club();
    let mut stadium = free_reserve_stadium(home, day, year);
    if stadium == 0 {
        let main = home.stadium().addr();
        // the second check passes the fixture's own date dwords (+0x2a day, +0x28 year), the original's registers
        if main == 0 || exe::fixture_manager_is_stadium_in_use(read_usize(G_FIXTURE_MANAGER), main, read_u32(f.addr() + fixture::DAY),
            read_u32(f.addr() + fixture::YEAR), u32::MAX) != 0 {
            return None;
        }
        stadium = home.stadium().addr();
        if stadium == 0 {
            return None;
        }
    }
    f.set_stadium(stadium);
    f.set_stadium_id(read_usize(stadium));
    let nation = game.competition().nation();
    if !nation.is_null() && read_i32(nation.addr()) == read_i32(KEY_NATION_BELGIUM) {
        covered!("rx11b_1 Belgian reserve squad rules");
        f.set_squad_rules((f.squad_rules() & 0xff00) | 0x44);
    }
    Some(r)
}

/// The reserve fixtures of `date` (`reserve_fixture_of`), handed to `add` in the original's order: for the 35 key
/// nations of `G_LEAGUE_TABLE` (0x48 bytes each) with a nation whose league is chosen (`league_selected`) and that is not
/// France (`KEY_NATION_FRANCE`), the three slices of the day; each league fixture (`comp_util_is_league_division` of its
/// competition id) whose clubs can field reserve sides (`ReserveComp::can_clubs_field_reserve_sides` with the date
/// checks). The list of the date is walked on a copy of its header, as the original does (the list's own cursor is not
/// moved).
unsafe fn for_each_reserve_fixture(this: usize, date: usize, add: &mut dyn FnMut(&mut [u8; fixture::SIZE])) {
    let can_field: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_RESERVECOMP_CAN_CLUBS_FIELD_RESERVE_SIDES);
    let when = reserve_match_date(date);
    for k in 0..0x23usize {
        let entry = G_LEAGUE_TABLE + k * 0x48;
        let nation = Nation(read_usize(entry));
        if nation.is_null() || nation.league_selected() == 0 || read_i32(nation.addr()) == read_i32(KEY_NATION_FRANCE) {
            continue;
        }
        for slice in 0..3u32 {
            // the list header (count, head, tail, cursor: 14 bytes) copied, the cursor at the head
            let mut list = [0u8; 0xe];
            let l = exe::fix_man_get_fixture_list_for_date(read_usize(G_FIXTURE_MANAGER), date, slice, read_u8(entry + key_nation_record::ACTIVE_INDEX) as u32);
            if l != 0 {
                core::ptr::copy_nonoverlapping(l as *const u8, list.as_mut_ptr(), 0xe);
            }
            let lp = list.as_mut_ptr() as usize;
            write_u32(lp + llist::CURSOR, read_u32(lp + llist::HEAD));
            loop {
                let game = Fixture(exe::llist_get_next(lp));
                if game.is_null() {
                    break;
                }
                if exe::comp_util_is_league_division(read_i32(game.competition().addr())) == 0 || can_field(this, game.addr(), 1) == 0 {
                    continue;
                }
                if let Some(mut r) = reserve_fixture_of(game, &when) {
                    add(&mut r);
                }
            }
        }
    }
}

const FN_RESERVECOMP_CAN_CLUBS_FIELD_RESERVE_SIDES: usize = 0x7e6ba0;

/// For the league fixtures of a date, adds the reserve matches two days later (when the date is weekday 4) or two days
/// earlier: for the 35 key nations with a chosen league other than France, the three slices of the date, each league
/// fixture whose two clubs can field reserve sides becomes a reserve fixture (`for_each_reserve_fixture`,
/// `reserve_fixture_of`) added to the fixture manager (`FIXTURE_MANAGER::add_fixture`, clearing the retry count). The
/// reserve match is played at the other club's ground (not in Turkey), at its reserve stadium when free, else at its
/// stadium when free, else not at all. The original's three local lists (two never used) are left out.
/// Original: `ReserveComp::schedule_reserve_fixtures` (0x7e6380, record_utils.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn schedule_reserve_fixtures(this: usize, date: usize) {
    for_each_reserve_fixture(this, date, &mut |r| {
        covered!("rx11b_1 reserve fixture added");
        exe::fixture_manager_add_fixture(read_usize(G_FIXTURE_MANAGER), r.as_mut_ptr() as usize, 1);
    });
}

/// The linked fixture a national team fixture gets (`add_b_or_under21_fixture`): 0x100 a B international, 0x200 an
/// Under 21 one, 0 none.
///
/// Both clubs need a reputation of 2500. A B international when the fixture's competition is a friendly
/// (`Friendly::is_friendly_competition`), the day before it is in an even year and no later in its year than 20 May
/// (day of year of 20 May 1998), both nations have 70 staff (`nation::NUMBER_STAFF`) and both clubs a reserve squad
/// (`database_club_has_reserve_squad`). Otherwise an Under 21 one when the fixture has no round prefix or no round name
/// and, in a year divisible by four, it is not between 1 February and 1 August (days of year of 2000).
unsafe fn linked_fixture_kind(game: Fixture) -> u16 {
    let (home, away) = (game.home_club(), game.away_club());
    if (home.reputation() as i16) < 0x9c4 || (away.reputation() as i16) < 0x9c4 {
        return 0;
    }
    let mut d = [0u8; 8];
    let mut before = [0u8; 8];
    let p = exe::new_date(d.as_mut_ptr() as usize, game.day() as u32, game.year() as u32);
    exe::date_sub_days(p, before.as_mut_ptr() as usize, 1);
    let (day_before, year_before) = (read_i16(before.as_ptr() as usize), read_u16(before.as_ptr() as usize + 2));
    if exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), game.competition()) != 0 && year_before % 2 == 0 {
        let mut may20 = [0u16; 4];
        let m = exe::date_date_549f70(&mut may20, 0x14, 5, 0x7ce, u32::MAX);
        if day_before <= read_i16(m) && home.nation().number_staff() >= 0x46 && away.nation().number_staff() >= 0x46
            && exe::database_club_has_reserve_squad(home) != 0 && exe::database_club_has_reserve_squad(away) != 0 {
            return 0x100;
        }
    }
    if game.round_prefix() != 0 && game.round_name() != 0 {
        return 0;
    }
    if game.year() % 4 == 0 {
        let mut t = [0u16; 4];
        let feb1 = read_i16(exe::date_date_549f70(&mut t, 1, 2, 0x7d0, u32::MAX)) as i32;
        if game.day() as i32 > feb1 {
            let aug1 = read_i16(exe::date_date_549f70(&mut t, 1, 8, 0x7d0, u32::MAX)) as i32;
            if (game.day() as i32) < aug1 {
                return 0;
            }
        }
    }
    0x200
}

/// The fixture the manager keeps for `game` (`FIXTURE_MANAGER::find_stored_fixture`), or `game` itself.
unsafe fn stored_fixture(game: usize) -> Fixture {
    let stored = exe::fixture_manager_find_stored_fixture(read_usize(G_FIXTURE_MANAGER), game);
    Fixture(if stored != 0 { stored } else { game })
}

/// Adds a linked B or Under 21 international the day before a match between two national teams
/// (`linked_fixture_kind`).
///
/// A copy of the fixture on the day before, category 9, no stage, stage index, round name, prefix or number, the 0x400
/// bit of the squad rules clear. A B international (`KEY_CLUB_COMP_B_INTERNATIONAL`) keeps the two teams, clears the side
/// marks and sets the 0x100 bit (clearing 0x200) in its rules and in the stored senior fixture's (`stored_fixture`); an
/// Under 21 one (`KEY_CLUB_COMP_UNDER_21_INTERNATIONAL`) takes both nations' Under 21 teams
/// (`database_get_national_u21_team`, not checked for null, as in the original) and sets 0x200 (clearing 0x100). The
/// ground: the home team's reserve stadium when free (`free_reserve_stadium`), else `Stadium::set_fixture_venue` (flag
/// 0x20, the team's own country). With no stadium after that, the senior fixture loses both bits and nothing is added;
/// otherwise the fixture goes to the fixture manager (`FIXTURE_MANAGER::add_fixture`, clearing the retry count).
/// Original: `ReserveComp::add_b_or_under21_fixture` (0x7e6720, record_utils.cpp, thiscall, ret 4; this is not read).
pub(crate) unsafe extern "thiscall" fn add_b_or_under21_fixture(_this: usize, game: usize) {
    let game = Fixture(game);
    let kind = linked_fixture_kind(game);
    if kind == 0 {
        return;
    }
    let mut r = [0u8; fixture::SIZE];
    core::ptr::copy_nonoverlapping(game.addr() as *const u8, r.as_mut_ptr(), fixture::SIZE);
    let f = Fixture(r.as_mut_ptr() as usize);
    let (mut d, mut before) = ([0u8; 8], [0u8; 8]);
    let p = exe::new_date(d.as_mut_ptr() as usize, game.day() as u32, game.year() as u32);
    exe::date_sub_days(p, before.as_mut_ptr() as usize, 1);
    f.set_squad_rules(f.squad_rules() & 0xfbff);
    f.set_year(read_u16(before.as_ptr() as usize + 2));
    f.set_day(read_u16(before.as_ptr() as usize));
    f.set_stage(0xff);
    f.set_category(9);
    f.set_stage_index(0xff);
    f.set_round_number(0);
    f.set_round_prefix(0);
    f.set_round_name(0);
    let senior = stored_fixture(game.addr());
    let (key, clear) = if kind == 0x100 {
        covered!("rx11b_1 B international");
        f.set_home_side_mark(0);
        f.set_away_side_mark(0);
        (KEY_CLUB_COMP_B_INTERNATIONAL, 0x200u16)
    } else {
        covered!("rx11b_1 Under 21 international");
        let home = exe::database_get_national_u21_team(f.home_club().nation());
        let away = exe::database_get_national_u21_team(f.away_club().nation());
        f.set_home_club(Club(home));
        f.set_away_club(Club(away));
        f.set_home_club_id(read_usize(home));
        f.set_away_club_id(read_usize(away));
        (KEY_CLUB_COMP_UNDER_21_INTERNATIONAL, 0x100u16)
    };
    f.set_comp_id(read_i32(key));
    f.set_squad_rules((f.squad_rules() & !clear) | kind);
    f.set_competition(Competition(key_comp_record(key)));
    senior.set_squad_rules((senior.squad_rules() & !clear) | kind); // rx11b_1: the senior fixture's linked fixture bit
    let stadium = free_reserve_stadium(f.home_club(), read_u32(f.addr() + fixture::DAY), read_u32(f.addr() + fixture::YEAR));
    if stadium != 0 {
        f.set_stadium(stadium);
        f.set_stadium_id(read_usize(stadium));
    } else {
        exe::stadium_set_fixture_venue(read_usize(G_STADIUM), 0x20usize, f.addr(), usize::MAX);
    }
    if f.stadium() == 0 {
        senior.set_squad_rules(senior.squad_rules() & 0xfcff);
        return;
    }
    exe::fixture_manager_add_fixture(read_usize(G_FIXTURE_MANAGER), f.addr(), 1);
}

/// The (home club, away club) pairs of the reserve competition's fixtures on `when`, over every active key nation's
/// list and the three slices (a copy of each list's header).
unsafe fn reserve_pairs_on(when: &[u8; 8]) -> Vec<(usize, usize)> {
    let record = key_comp_record(KEY_CLUB_COMP_RESERVE);
    let mut v = Vec::new();
    for k in 0..read_i8(G_KEY_NATION_ACTIVE_COUNT).max(0) as u32 {
        for slice in 0..3u32 {
            let l = exe::fix_man_get_fixture_list_for_date(read_usize(G_FIXTURE_MANAGER), when.as_ptr() as usize, slice, k);
            if l == 0 {
                continue;
            }
            let mut list = [0u8; 0xe];
            core::ptr::copy_nonoverlapping(l as *const u8, list.as_mut_ptr(), 0xe);
            let lp = list.as_mut_ptr() as usize;
            write_u32(lp + llist::CURSOR, read_u32(lp + llist::HEAD));
            loop {
                let f = Fixture(exe::llist_get_next(lp));
                if f.is_null() {
                    break;
                }
                if f.competition().addr() == record {
                    v.push((f.home_club().addr(), f.away_club().addr()));
                }
            }
        }
    }
    v
}

/// Comparer for `schedule_reserve_fixtures`: the fixtures go into the calendar (and `add_fixture` may refuse or move
/// one), so only the original runs. The plan first: the reserve fixtures the copy would add (their pairs of clubs); then
/// every reserve fixture of the reserve date the original left that was not there before must be one of them, with the
/// planned clubs.
pub(crate) unsafe extern "thiscall" fn compare_schedule_reserve_fixtures(this: usize, date: usize) {
    let mut planned = Vec::new();
    for_each_reserve_fixture(this, date, &mut |r| {
        let f = Fixture(r.as_ptr() as usize);
        planned.push((f.home_club().addr(), f.away_club().addr()));
    });
    let when = reserve_match_date(date);
    let mut before = reserve_pairs_on(&when);
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%)(this, date);
    let mut unplanned = Vec::new();
    for pair in reserve_pairs_on(&when) {
        if let Some(k) = before.iter().position(|p| *p == pair) {
            before.swap_remove(k);
        } else if let Some(k) = planned.iter().position(|p| *p == pair) {
            planned.swap_remove(k);
        } else {
            unplanned.push(pair);
        }
    }
    SCOREBOARD.record(unplanned.is_empty(), || format!("schedule_reserve_fixtures(date {:x?}): reserve fixtures the copy did not plan {:x?} (still planned {:x?})",
        (read_u16(date), read_u16(date + 2)), unplanned, planned));
}

/// Comparer for `add_b_or_under21_fixture`: the linked fixture goes into the calendar, so only the original runs; the
/// plan is the kind (`linked_fixture_kind`): with none the stored senior fixture's squad rules stay as they were; with one
/// only its 0x300 bits may change, to the planned bit (or to none, when no ground was found).
pub(crate) unsafe extern "thiscall" fn compare_add_b_or_under21_fixture(this: usize, game: usize) {
    let kind = linked_fixture_kind(Fixture(game));
    let senior = stored_fixture(game);
    let before = senior.squad_rules();
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%)(this, game);
    let after = senior.squad_rules();
    let ok = if kind == 0 { after == before } else { after & !0x300 == before & !0x300 && (after & 0x300 == kind || after & 0x300 == 0) };
    SCOREBOARD.record(ok, || format!("add_b_or_under21_fixture(fixture {:#x}): planned kind {:#x}, senior rules {:#x} -> {:#x}", game, kind, before, after));
}
