// ---------- sx18_28: the reserve competition: whether two clubs can field reserve sides, today's unplayable reserve fixtures and the B and Under 21 sub competitions (record_utils.cpp) ----------

/// A Turkish competition key the reserve rules require of a Turkish home club's division (no name in `keys.rs` yet).
const KEY_RESERVE_TURKISH_DIVISION: usize = 0x9cf8d4;

/// The competition record (0x6b bytes in `G_CLUB_COMP_TABLE`) of the competition whose id the key holds.
unsafe fn reserve_comp_record(key: usize) -> usize {
    read_usize(G_CLUB_COMP_TABLE).wrapping_add((read_i32(key) as usize).wrapping_mul(0x6b))
}

/// One club's squad for a reserve match, over the 50 `club::SQUAD` slots: the empty slots; and, among the players at
/// the club (`staff::CLUB_JOB` = the club) and not injured (`Injury::get_player_injury_status`), the listed ones (id
/// below `G_STAFF_COUNT`), the young ones (age at `when` up to 21, counted only when `min_young` is not −1), the
/// available ones (bit 0 of `staff::PLAYING_SQUAD` clear) and those of them listed.
struct ReserveSquad {
    empty: i8,
    listed: i8,
    young: i8,
    available: i8,
    available_listed: i8,
}

unsafe fn reserve_squad(club: usize, min_young: i8, when: *const u16) -> ReserveSquad {
    let mut s = ReserveSquad { empty: 0, listed: 0, young: 0, available: 0, available_listed: 0 };
    for k in 0..50 {
        let p = read_usize(club + club::SQUAD + 4 * k);
        if p == 0 {
            s.empty += 1;
            continue;
        }
        if read_usize(p + staff::CLUB_JOB) != club || exe::injury_get_player_injury_status(read_usize(G_INJURY), p) != 0 {
            continue;
        }
        let listed = read_i32(p) < read_i32(G_STAFF_COUNT);
        if listed {
            s.listed += 1;
        }
        if min_young != -1 && exe::database_get_staff_age(p, when) <= 0x15 {
            s.young += 1;
        }
        if read_u8(p + staff::PLAYING_SQUAD) & 1 == 0 {
            if listed {
                s.available_listed += 1;
            }
            s.available += 1;
        }
    }
    s
}

/// Whether a club can field its reserve side: with fewer than 11 available players, no more than 36 listed and enough
/// empty slots to make up 14; a human club needs 11 available and one of them listed; and the young players must reach
/// `min_young`.
unsafe fn reserve_side_ok(club: usize, s: &ReserveSquad, min_young: i8) -> bool {
    if s.available < 0xb && (s.listed > 0x24 || (s.empty as i32) < 0xe - s.available as i32) {
        return false;
    }
    if exe::human_manager_is_human_club(club, 1, 1, 0, 0) != 0 && (s.available < 0xb || s.available_listed < 1) { // sx18_28: a human club's reserve side
        return false;
    }
    s.young >= min_young
}

/// Whether both clubs of a fixture can field a reserve side (`reserve_squad`, `reserve_side_ok`).
///
/// With `check_date`: not for a fixture with a round name (`fixture::ROUND_NAME` ≠ 0), nor when the home club is
/// Turkish (`club::NATION`) and its division is not the Turkish one, nor for the round prefixes 0x418, 0x419 and 0xa0,
/// nor on 26 December of the fixture's year. A home club in the Scottish Premier Division or Turkish needs 15 young
/// players (21 or under at the fixture's date); the others none (−1). Both clubs must have a reserve squad
/// (`database_club_has_reserve_squad`); then the home club's side and the away club's are checked in turn.
/// Original: `ReserveComp::can_clubs_field_reserve_sides` (0x7e6ba0, record_utils.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn can_clubs_field_reserve_sides(_this: usize, fixture: usize, check_date: u32) -> u32 {
    let f = Fixture(fixture);
    let nation_is_turkey = |club: usize| read_i32(read_usize(club + club::NATION)) == read_i32(KEY_NATION_TURKEY);
    if check_date != 0 {
        let home = f.home_club().addr();
        if read_u16(fixture + fixture::ROUND_NAME) != 0 {
            return 0;
        }
        if nation_is_turkey(home) {
            let division = read_usize(home + club::DIVISION);
            if division == 0 || read_i32(division) != read_i32(KEY_RESERVE_TURKISH_DIVISION) {
                return 0;
            }
        }
        if matches!(read_u16(fixture + fixture::ROUND_PREFIX), 0x418 | 0x419 | 0xa0) {
            return 0;
        }
        let (mut boxing_day, mut played) = ([0u16; 4], [0u8; 8]);
        exe::date_date_549f70(&mut boxing_day, 0x1a, 0xb, f.year() as u32, u32::MAX);
        let p = exe::new_date(played.as_mut_ptr() as usize, f.day() as u32, f.year() as u32);
        if boxing_day[1] == read_u16(p + 2) && boxing_day[0] == read_u16(p) { // sx18_28: no reserve match on 26 December
            return 0;
        }
    }
    let (mut min_young, mut when) = (-1i8, [0u16; 4]);
    exe::date_date_54a110(when.as_mut_ptr() as usize);
    let home = f.home_club().addr();
    if home != 0 {
        let division = read_usize(home + club::DIVISION);
        if division != 0 && (read_i32(division) == read_i32(KEY_CLUB_COMP_SCOTTISH_PREMIER_DIVISION) || nation_is_turkey(home)) {
            covered!("sx18_28 reserve sides with young players");
            min_young = 0xf;
            let mut t = [0u8; 8];
            let p = exe::new_date(t.as_mut_ptr() as usize, f.day() as u32, f.year() as u32);
            copy_record_date(when.as_mut_ptr() as usize, p);
        }
    }
    let away = f.away_club().addr();
    if exe::database_club_has_reserve_squad(home) == 0 || exe::database_club_has_reserve_squad(away) == 0 {
        return 0;
    }
    covered!("sx18_28 reserve sides counted");
    for club in [home, away] {
        let s = reserve_squad(club, min_young, when.as_ptr());
        if !reserve_side_ok(club, &s, min_young) {
            return 0;
        }
    }
    1
}

/// Removes today's reserve fixtures that cannot be played: for each active key nation's fixture list of today
/// (`fix_man_get_fixture_list_for_date` with today, the day slice and the index), each fixture of the reserve
/// competition or of the French CFA or lower division whose clubs cannot field reserve sides
/// (`can_clubs_field_reserve_sides` without the date checks) is copied to a local list; then each one is removed
/// (`FIXTURE_MANAGER::remove_fixture`).
/// Original: `ReserveComp::cancel_unplayable_reserve_fixtures` (0x7e6a70, record_utils.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn cancel_unplayable_reserve_fixtures(this: usize) {
    for fixture in unplayable_reserve_fixtures(this) {
        covered!("sx18_28 reserve fixture cancelled");
        exe::fixture_manager_remove_fixture(read_usize(G_FIXTURE_MANAGER), fixture.as_ptr() as usize);
    }
}

/// Today's reserve fixtures whose clubs cannot field reserve sides, copied (`fixture::SIZE` bytes each), in order.
unsafe fn unplayable_reserve_fixtures(this: usize) -> Vec<[u8; fixture::SIZE]> {
    let mut out = Vec::new();
    let mut k = 0i8;
    while k < read_i8(G_KEY_NATION_ACTIVE_COUNT) {
        let list = exe::fix_man_get_fixture_list_for_date(read_usize(G_FIXTURE_MANAGER), G_CURRENT_DATE, read_u8(G_CURRENT_DAY_SLICE) as u32, k as u8 as u32);
        if list != 0 {
            write_u32(list + llist::CURSOR, read_u32(list + llist::HEAD));
            loop {
                let fixture = exe::llist_get_next(list);
                if fixture == 0 {
                    break;
                }
                let comp = read_usize(fixture + fixture::COMPETITION);
                if comp == 0 {
                    continue;
                }
                let id = read_i32(comp);
                let reserve = [KEY_CLUB_COMP_RESERVE, KEY_CLUB_COMP_FRENCH_CFA, KEY_CLUB_COMP_FRENCH_LOWER_DIVISION].iter().any(|&key| id == read_i32(key));
                if reserve && can_clubs_field_reserve_sides(this, fixture, 0) == 0 {
                    let mut copy = [0u8; fixture::SIZE];
                    core::ptr::copy_nonoverlapping(fixture as *const u8, copy.as_mut_ptr(), fixture::SIZE);
                    out.push(copy);
                }
            }
        }
        k += 1;
    }
    out
}

/// Registers the reserve competition and builds its two sub competitions: the reserve competition object goes to its
/// slot of `G_COMP_OBJECT_TABLE` (`KEY_CLUB_COMP_RESERVE`); a B international competition (0xa3 bytes, `Comp::Comp`,
/// vtable 0x96f074, its record `KEY_CLUB_COMP_B_INTERNATIONAL`) goes to +0xa3 and to its slot, and an Under 21
/// international one (vtable 0x96efe4, `KEY_CLUB_COMP_UNDER_21_INTERNATIONAL`) to +0xa7 and to its slot (0 when out of
/// memory). Then, with the French first division built, the French CFA's slot gets its object's +0xee and the French
/// lower division's its +0xf2; with the Spanish first division built, the Spanish lower division's slot gets its +0xee.
/// Original: `ReserveComp::create_sub_comps` (0x7e61c0, record_utils.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn reserve_comp_create_sub_comps(this: usize) {
    let slot = |key: usize| read_usize(G_COMP_OBJECT_TABLE).wrapping_add((read_i32(key) as usize).wrapping_mul(4));
    write_usize(slot(KEY_CLUB_COMP_RESERVE), this);
    for (at, vtable, key) in [(0xa3usize, 0x96f074u32, KEY_CLUB_COMP_B_INTERNATIONAL), (0xa7, 0x96efe4, KEY_CLUB_COMP_UNDER_21_INTERNATIONAL)] {
        let p = exe::crt_operator_new(0xa3);
        let comp = if p != 0 {
            exe::new_comp(p);
            write_u32(p, vtable);
            write_usize(p + 4, reserve_comp_record(key)); // the competition's record
            p
        } else {
            0
        };
        write_usize(this + at, comp); // the B (+0xa3) and Under 21 (+0xa7) international competitions
        write_usize(slot(key), comp);
    }
    covered!("sx18_28 reserve sub competitions");
    let french = read_usize(slot(KEY_CLUB_COMP_FRENCH_FIRST_DIVISION));
    if french != 0 {
        write_usize(slot(KEY_CLUB_COMP_FRENCH_CFA), read_usize(french + 0xee)); // the French first division's CFA object
        write_usize(slot(KEY_CLUB_COMP_FRENCH_LOWER_DIVISION), read_usize(read_usize(slot(KEY_CLUB_COMP_FRENCH_FIRST_DIVISION)) + 0xf2));
    }
    let spanish = read_usize(slot(KEY_CLUB_COMP_SPANISH_FIRST_DIVISION));
    if spanish != 0 {
        write_usize(slot(KEY_CLUB_COMP_SPANISH_LOWER_DIVISION), read_usize(spanish + 0xee)); // the Spanish first division's lower object
    }
}

compare_value!(compare_can_clubs_field_reserve_sides, can_clubs_field_reserve_sides, %%I0%%, (fixture: usize, check_date: u32) -> u32);

/// Comparer for `cancel_unplayable_reserve_fixtures`: the fixtures cannot be removed twice; the copy lists the
/// fixtures it would remove first (their first 0x14 bytes: competition, stadium, statistics slot and clubs), then only
/// the original runs; none of them may be left among today's fixtures.
pub(crate) unsafe extern "thiscall" fn compare_cancel_unplayable_reserve_fixtures(this: usize) {
    let expected: Vec<[u8; 0x14]> = unplayable_reserve_fixtures(this).iter().map(|f| f[..0x14].try_into().unwrap()).collect();
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this);
    let mut left = 0;
    let mut k = 0i8;
    while k < read_i8(G_KEY_NATION_ACTIVE_COUNT) {
        let list = exe::fix_man_get_fixture_list_for_date(read_usize(G_FIXTURE_MANAGER), G_CURRENT_DATE, read_u8(G_CURRENT_DAY_SLICE) as u32, k as u8 as u32);
        if list != 0 {
            write_u32(list + llist::CURSOR, read_u32(list + llist::HEAD));
            loop {
                let fixture = exe::llist_get_next(list);
                if fixture == 0 {
                    break;
                }
                if expected.iter().any(|e| core::slice::from_raw_parts(fixture as *const u8, 0x14) == &e[..]) {
                    left += 1;
                }
            }
        }
        k += 1;
    }
    SCOREBOARD.record(left == 0, || format!("cancel_unplayable_reserve_fixtures: {} of {} unplayable fixtures still there", left, expected.len()));
}

/// Comparer for `reserve_comp_create_sub_comps`: it allocates the sub competitions (it cannot run twice); only the
/// original runs and the table slots it leaves are checked.
pub(crate) unsafe extern "thiscall" fn compare_reserve_comp_create_sub_comps(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this);
    let slot = |key: usize| read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((read_i32(key) as usize).wrapping_mul(4)));
    let (b, u21) = (read_usize(this + 0xa3), read_usize(this + 0xa7));
    let mut ok = slot(KEY_CLUB_COMP_RESERVE) == this && slot(KEY_CLUB_COMP_B_INTERNATIONAL) == b && slot(KEY_CLUB_COMP_UNDER_21_INTERNATIONAL) == u21
        && (b == 0 || (read_u32(b) == 0x96f074 && read_usize(b + 4) == reserve_comp_record(KEY_CLUB_COMP_B_INTERNATIONAL)))
        && (u21 == 0 || (read_u32(u21) == 0x96efe4 && read_usize(u21 + 4) == reserve_comp_record(KEY_CLUB_COMP_UNDER_21_INTERNATIONAL)));
    let french = slot(KEY_CLUB_COMP_FRENCH_FIRST_DIVISION);
    if french != 0 {
        ok &= slot(KEY_CLUB_COMP_FRENCH_CFA) == read_usize(french + 0xee) && slot(KEY_CLUB_COMP_FRENCH_LOWER_DIVISION) == read_usize(french + 0xf2);
    }
    let spanish = slot(KEY_CLUB_COMP_SPANISH_FIRST_DIVISION);
    if spanish != 0 {
        ok &= slot(KEY_CLUB_COMP_SPANISH_LOWER_DIVISION) == read_usize(spanish + 0xee);
    }
    SCOREBOARD.record(ok, || format!("reserve_comp_create_sub_comps({:#x}): B {:#x}, Under 21 {:#x}", this, b, u21));
}
