use super::*;

/// Club record i in the club table (0x245 bytes each).
pub(crate) unsafe fn fix_man_club_by_index(i: usize) -> Club {
    Club(read_usize(G_CLUB_TABLE).wrapping_add(i).wrapping_add(i.wrapping_mul(0x244)))
}

/// True if the club has a human manager (`exe::human_manager_is_human_club` with 1,1,0,0).
pub(crate) unsafe fn fix_man_is_human_club(club: usize) -> bool {
    exe::human_manager_is_human_club(club, 1, 1, 0, 0) != 0
}

/// Daily fixture manager update: reserve fixtures, queued fixture news, removal of AI-only friendlies and cup round
/// labels.
///
/// `FIXTURE_MANAGER::daily_update()`, from the day loop (0x5cc850); SEH frame with no destructor on the normal path
/// (the local dates). By today's weekday (`exe::date_get_weekday` on `G_CURRENT_DATE`):
///
/// 1. days 2 and 3: the reserve fixtures for 3 days ahead (`exe::date_add_days` +
/// `exe::reservecomp_schedule_reserve_fixtures`: push &date; mov ecx, [`G_RESERVE_COMP`] -> thiscall, ret 4) and, on
/// day 3, also for tomorrow;
///
/// 2. the fixture news queue (`fixture_manager::MOVED_NEWS_QUEUE`, a list): each news item goes to the manager of the
/// club in slot 0 if slot 0xe asks for it and to the manager of the club in slot 1 if slot 0xf asks for it
/// (`exe::newsitem_get_slot`: push k; mov ecx, news -> thiscall, ret 4; `exe::news_send_to_club_manager`: push club;
/// push news; mov ecx, [`G_NEWS`] -> thiscall, ret 8); the list is destroyed (`exe::drop_llist` +
/// `exe::crt_operator_delete`) and `fixture_manager::MOVED_NEWS_QUEUE` becomes 0;
///
/// 3. days 5 and 6, if yesterday is in the same year: in yesterday's lists (the 3 slices, the key nations up to
/// `G_KEY_NATION_ACTIVE_COUNT`), from back to front, the friendlies (`KEY_CLUB_COMP_RESERVE`, `fixture::CATEGORY` = 1)
/// between two clubs without a human manager leave the list (`exe::llist_get_prev` + `exe::llist_remove_node`); the
/// first fixture that is not a friendly of that type ends the list;
///
/// 4. the leg check list (`G_FIXTURE_LEGS_LIST`): each entry whose cup (or its stage +4) has `cup::COMP_TYPE` = 1
/// labels the following fixtures of the round (`exe::cup_label_round_extra_fixtures`: push
/// `fixture_leg_check::AWAY_CLUB_ID`; push +7; push round; ecx = the cup -> thiscall, ret 0xc); the list is emptied
/// (`exe::llist_clear`).
/// Original: `FIXTURE_MANAGER::daily_update` (0x5aaf00, fix_man.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fix_man_daily_update(this: usize) {
    let (mut a, mut b, mut c) = ([0u8; 8], [0u8; 8], [0u8; 8]);
    let (pa, pb, pc) = (a.as_mut_ptr() as usize, b.as_mut_ptr() as usize, c.as_mut_ptr() as usize);
    let day = exe::date_get_weekday(G_CURRENT_DATE) as i16;
    if day == 2 || day == 3 {
        exe::date_add_days(G_CURRENT_DATE, pa, 3);
        exe::reservecomp_schedule_reserve_fixtures(read_usize(G_RESERVE_COMP), pa);
        if day == 3 {
            exe::date_add_days(G_CURRENT_DATE, pb, 1);
            covered!("cqd_6 tomorrow reserves");
            exe::reservecomp_schedule_reserve_fixtures(read_usize(G_RESERVE_COMP), pb); // cqd6: tomorrow’s reservations
        }
    }
    let l = read_usize(this + fixture_manager::MOVED_NEWS_QUEUE);
    if l != 0 {
        write_usize(l + llist::CURSOR, read_usize(l + 2));
        let mut n = exe::llist_get_next(l);
        while n != 0 {
            let ca = fix_man_club_by_index(exe::newsitem_get_slot(n, 0) as usize);
            let cb = fix_man_club_by_index(exe::newsitem_get_slot(n, 1) as usize);
            if exe::newsitem_get_slot(n, 0xe) != 0 {
                exe::news_send_to_club_manager(read_usize(G_NEWS), n, ca);
            }
            if exe::newsitem_get_slot(n, 0xf) != 0 {
                exe::news_send_to_club_manager(read_usize(G_NEWS), n, cb);
            }
            n = exe::llist_get_next(read_usize(this + fixture_manager::MOVED_NEWS_QUEUE));
        }
        let l = read_usize(this + fixture_manager::MOVED_NEWS_QUEUE);
        if l != 0 {
            exe::drop_llist(l);
            exe::crt_operator_delete(l);
        }
        write_usize(this + fixture_manager::MOVED_NEWS_QUEUE, 0);
    }
    if day == 5 || day == 6 {
        let t = exe::date_sub_days(G_CURRENT_DATE, pa, 1);
        if read_i16(t + 2) == read_i16(G_CURRENT_DATE_YEAR) {
            let ontem = read_i16(exe::date_sub_days(G_CURRENT_DATE, pc, 1)) as u16 as usize;
            for fatia in 0..3usize {
                let lists = read_usize(this + (ontem * 3 + fatia) * 4);
                let mut k: i8 = 0;
                while k < read_u8(G_KEY_NATION_ACTIVE_COUNT) as i8 {
                    let list = read_usize(lists + k as isize as usize * 4);
                    let mut in_the = read_usize(list + 6);
                    write_usize(list + llist::CURSOR, in_the);
                    while in_the != 0 {
                        let f = read_usize(in_the);
                        if read_usize(f) != read_usize(KEY_CLUB_COMP_RESERVE) || read_u8(f + fixture::CATEGORY) != 1 {
                            break;
                        }
                        if !fix_man_is_human_club(read_usize(f + fixture::HOME_CLUB)) && !fix_man_is_human_club(read_usize(f + fixture::AWAY_CLUB)) {
                            exe::llist_get_prev(list);
                            covered!("cqd_6 friendly discarded");
                            exe::llist_remove_node(list, in_the); // cqd6: yesterday’s friendly leaves the list
                        } else {
                            exe::llist_get_prev(list);
                        }
                        in_the = read_usize(list + llist::CURSOR);
                    }
                    k = k.wrapping_add(1);
                }
            }
        }
    }
    if read_i16(G_FIXTURE_LEGS_LIST) != 0 {
        write_usize(G_FIXTURE_LEGS_LIST_CURSOR, read_usize(G_FIXTURE_LEGS_LIST_HEAD));
        let mut e = exe::llist_get_next(G_FIXTURE_LEGS_LIST);
        while e != 0 {
            let i = read_usize(e) as i32;
            let table = read_usize(G_COMP_OBJECT_TABLE);
            if i >= 0 && table != 0 {
                let mut cp = read_usize(table + i as usize * 4);
                if cp != 0 {
                    let stage = read_u8(e + 4) as i8;
                    if stage as i32 <= read_usize(cp + cup::CURRENT_STAGE) as i32 {
                        if stage != -1 {
                            cp = read_usize(read_usize(cp + cup::STAGES) + stage as isize as usize * 4);
                        }
                        if read_u8(cp + cup::COMP_TYPE) == 1 {
                            // simplification: the original pushes eax with the round word and the high bytes of the pointer of the
                            // entry; 0x522c70 only reads the word
                            exe::cup_label_round_extra_fixtures(cp, read_i16(e + 5) as u16 as u32, read_usize(e + 7) as u32, read_usize(e + fixture_leg_check::AWAY_CLUB_ID) as u32);
                        }
                    }
                }
            }
            e = exe::llist_get_next(G_FIXTURE_LEGS_LIST);
        }
        exe::llist_clear(G_FIXTURE_LEGS_LIST);
    }
}

/// Returns the fixture list for a year, day, slice and key nation, switching the other year block to the requested year
/// or creating it.
///
/// `FIXTURE_MANAGER::get_fixture_list(year, day, slice, kind)`; SEH frame with no destructor on the normal path. The
/// list of fixtures of the year, day (word), slice (byte) and key nation (signed byte) in the chain of year blocks. By
/// the block's year (`fixture_manager::YEAR`): if the other block (`fixture_manager::OTHER_YEAR`) is not of that year,
/// it is switched to the year (`exe::fixture_manager_set_year`: push year; ecx = the other -> thiscall, ret 4); without
/// the other block, a new block (`exe::crt_operator_new`, 0x1184 bytes; `exe::new_fixture_manager`: push 0; push this;
/// push year; ecx = the new one -> thiscall, ret 0xc) goes into `fixture_manager::OTHER_YEAR`; out of memory, the box
/// for line 0x681 and 0. The list: [[block + (day·3 + slice)·4] + kind·4].
/// Original: `FIXTURE_MANAGER::get_fixture_list` (0x5abf10, fix_man.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn get_fixture_list(this: usize, year: u32, day: u32, slice: u32, kind: u32) -> usize {
    let mut b = this;
    while read_i16(b + fixture_manager::YEAR) != year as i16 {
        let o = read_usize(b + fixture_manager::OTHER_YEAR);
        if o == 0 {
            let m = exe::crt_operator_new(0x1184);
            let r = if m == 0 { 0 } else { exe::new_fixture_manager(m, year, b, 0) };
            write_usize(b + fixture_manager::OTHER_YEAR, r);
            if r == 0 {
                show_error_at(0x9cc208, 0x681);
                return 0;
            }
        } else if read_i16(o + fixture_manager::YEAR) != year as i16 {
            exe::fixture_manager_set_year(o, year);
        }
        b = read_usize(b + fixture_manager::OTHER_YEAR);
    }
    let i = (day & 0xffff) as usize * 3 + (slice & 0xff) as usize;
    read_usize(read_usize(b + i * 4) + (kind as u8 as i8) as isize as usize * 4)
}

/// Reads fixtures_<year>.tmp: each 0x4f-byte fixture gets its club, competition and stadium pointers and goes into the
/// list of its day, slice and key nation.
///
/// `FIXTURE_MANAGER::load_fixtures()`. The name `fixtures_<year>.tmp` at `fixture_manager::FILE_NAME`
/// (`FN_CRT_SPRINTF`) and, with a year (`fixture_manager::YEAR`, unsigned) from 1980 on and the file in CM3_TEMP
/// (`exe::utils_build_file_path`: push name; push "CM3_TEMP"; add esp, 8 -> cdecl; `exe::crt_fopen` fopen with "rb"),
/// the count (dword) and the 0x4f-byte fixtures (`exe::crt_fread` fread; on failure, the box for line 0x730 or 0x73e
/// and the file is closed): the club ids (`.home_club_id()`, `.away_club_id()` → `.set_home_club()`,
/// `.set_away_club()`), the competition id (+0 → `.set_competition()`, 0x6b bytes in `G_CLUB_COMP_TABLE`) and the
/// stadium id (`.stadium_id()` → `.set_stadium()`, 0x4e bytes in `G_STADIUM_TABLE`, 0 outside `G_STADIUM_COUNT`) become
/// pointers (id -1: the game stops, lines 0x74b, 0x751, 0x758), and the fixture goes (`exe::llist_add_tail`: push 0x4f;
/// push &fixture; ecx = the list -> thiscall, ret 8; on failure, the game stops, 0x785) into the list of the fixture's
/// day (`.day()`), slice (`.time_slice()`) and key nation (`exe::comp_active_key_nation_of_fixture`: push &fixture;
/// push competition; add esp, 8 -> cdecl). At the end, fclose (`exe::crt_fclose`).
/// Original: `FIXTURE_MANAGER::load_fixtures` (0x5ac3b0, fix_man.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn load_fixtures(this: usize) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let name = this + fixture_manager::FILE_NAME;
    let year = read_i16(this + fixture_manager::YEAR) as u16;
    format(name, 0x9cc238, year as u32);
    if year < 0x7bc {
        return;
    }
    let f = exe::crt_fopen(exe::utils_build_file_path(0x987da0, name), 0x98f928);
    if f == 0 {
        return;
    }
    let mut count = 0i32;
    if exe::crt_fread(&mut count as *mut i32 as usize, 4, 1, f) != 1 {
        show_error_at(0x9cc208, 0x730);
        exe::crt_fclose(f);
        return;
    }
    let mut game = [0u8; 0x4f];
    let j = Fixture(game.as_mut_ptr() as usize);
    let mut i = 0i32;
    while i < count {
        if exe::crt_fread(j, 0x4f, 1, f) != 1 {
            show_error_at(0x9cc208, 0x73e);
            exe::crt_fclose(f);
            return;
        }
        let a = j.home_club_id();
        if a as u32 == u32::MAX {
            fatal_error_at(0x9cc208, 0x74b);
        }
        j.set_home_club(fix_man_club_by_index(a));
        let b = j.away_club_id();
        if b as u32 == u32::MAX {
            fatal_error_at(0x9cc208, 0x751);
        }
        j.set_away_club(fix_man_club_by_index(b));
        let c = read_usize(j.addr());
        if c as u32 == u32::MAX {
            fatal_error_at(0x9cc208, 0x758);
        }
        let comp = read_usize(G_CLUB_COMP_TABLE).wrapping_add(c.wrapping_mul(0x6b));
        j.set_competition(Competition(comp));
        let e = j.stadium_id() as i32;
        let stadium = if e != -1 && e < read_usize(G_STADIUM_COUNT) as i32 {
            read_usize(G_STADIUM_TABLE).wrapping_add((e as usize).wrapping_mul(0x4e))
        } else {
            0
        };
        j.set_stadium(stadium);
        let k = exe::comp_active_key_nation_of_fixture(comp, j) as i8;
        let day = (j.day() as i16) as u16 as usize; // cqd6: the day of the loaded match
        let list = read_usize(read_usize(this + (day * 3 + j.time_slice() as usize) * 4) + k as isize as usize * 4);
        covered!("cqd_6 match loaded");
        if (exe::llist_add_tail(list, j, 0x4f) as u32) == 0 {
            fatal_error_at(0x9cc208, 0x785);
        }
        i += 1;
    }
    exe::crt_fclose(f);
}

/// The rehearsal of the comparer for one day's conflicts. The copy does not add anything (0x68d0d0 allocates the node
/// and the copy of the match) and keeps, in order, the matches it would add.
pub(crate) static DAY_CLASHES_DRY_RUN: AtomicBool = AtomicBool::new(false);
pub(crate) static mut DAY_CLASHES_APPENDED: Vec<usize> = Vec::new();

/// Appends a copy of the 0x4f-byte fixture to the list (0x68d0d0); in dry-run mode only records it.
pub(crate) unsafe fn append_fixture(list: usize, game: usize) {
    if DAY_CLASHES_DRY_RUN.load(Relaxed) {
        DAY_CLASHES_APPENDED.push(game);
        return;
    }
    exe::llist_add_tail(list, game, 0x4f);
}

/// Data pointers of a LLIST's nodes in order (head +2, node next +6, data +0).
pub(crate) unsafe fn fix_man_list_items(list: usize) -> Vec<usize> {
    let mut v = Vec::new();
    if list == 0 {
        return v;
    }
    let mut in_the = read_usize(list + 2);
    while in_the != 0 {
        v.push(read_usize(in_the));
        in_the = read_usize(in_the + 6);
    }
    v
}

/// Concatenated 0x4f bytes of each fixture in a sequence.
pub(crate) unsafe fn fixtures_bytes(games: &[usize]) -> Vec<u8> {
    let mut v = Vec::new();
    for &j in games {
        v.extend_from_slice(core::slice::from_raw_parts(j as *const u8, 0x4f));
    }
    v
}

/// Builds a date from (day, month, year) via `exe::date_date_549f70` and returns (day of year, year).
pub(crate) unsafe fn day_of_year(day: u32, month: u32, year: u32) -> (u16, u16) {
    let f: unsafe extern "thiscall" fn(*mut [u8; 8], u32, u32, u32, u32) -> *mut [u8; 8] = core::mem::transmute(FN_DATE_DATE_549F70);
    let mut t = [0u8; 8];
    let r = f(&mut t, day, month, year, 0xffff_ffff);
    (read_u16(r as usize), read_u16(r as usize + 2))
}

/// True if the club's division (+0x57) has the given competition id.
pub(crate) unsafe fn is_in_league(club: Club, id: i32) -> bool {
    let l = club.division().addr();
    l != 0 && read_i32(l) == id
}

/// Calls the competition's clash-rule method (vtable +0x60) for fixtures a and b; 0 without the table or the
/// competition.
pub(crate) unsafe fn comp_clash_rule(id_of: usize, a: usize, b: usize) -> u32 {
    let table = read_usize(G_COMP_OBJECT_TABLE);
    if table == 0 {
        return 0;
    }
    let c = read_usize(table.wrapping_add((read_i32(id_of) as usize).wrapping_mul(4)));
    if c == 0 {
        return 0;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(read_usize(read_usize(c) + comp_vtable::COUNTS_FOR_FIXTURE_CLASH));
    f(c, a, b)
}

/// If checking played and the fixture was played (+0x43 != 0xff) sets severity 0x16; otherwise appends it and raises
/// severity to its type (+0x3a).
pub(crate) unsafe fn add_clash_by_type(grav: *mut i32, list: usize, x: Fixture, check_played: bool) {
    if check_played && (x.home_goals() as u8) != 0xff {
        *grav = 0x16;
        return;
    }
    append_fixture(list, x.addr());
    let t = x.category() as i8 as i32;
    if t > *grav {
        *grav = t;
    }
}

/// If severity is still 0 sets it to 0x14 or 1 (common tail of the same-club/same-stadium branches).
pub(crate) unsafe fn default_clash_severity(grav: *mut i32, twenty: bool) {
    if *grav == 0 {
        *grav = if twenty { 0x14 } else { 1 };
    }
}

/// Collects the clashes of a fixture on its day plus an offset: forbidden dates, fixtures of the same club or stadium,
/// national teams and big clubs, and fixtures already played.
///
/// `FIXTURE_MANAGER::collect_day_clashes(game, grav, list, nations, n, data, desloc)`; SEH frame only for the dates.
/// Used for a fixture's clashes (`find_fixture_clashes`) on day `data` + `desloc` (`exe::date_add_days`: push 16-bit n;
/// push &local; ecx = date -> thiscall, ret 8). A cup (`.category()` = 1) only on the day itself. On the day itself,
/// the forbidden dates give 19 and return: 25 December (except in the competitions of the nation of
/// `KEY_NATION_JAPAN`), Sunday in those of the nation of `KEY_NATION_NORTHERN_IRELAND` (`exe::date_get_weekday`: ecx =
/// date -> thiscall), 10 to 20 April in those of `KEY_NATION_GREECE` (the dates through `exe::date_date_549f70`: 4
/// pushes -> thiscall); 17 May with a club of nation `KEY_NATION_NORWAY` (also with an offset, without returning, if it
/// is the day itself). Then, for each nation in `nations` (except the 0xff ones) and each of the 3 slices, the day list
/// (`exe::fixture_manager_get_fixture_list`: ecx = manager; 4 pushes -> thiscall, ret 0x10) with the cursor at the
/// start: each fixture in it (`exe::llist_get_next`) that shares a club and slice
/// (`fixture::HOME_SIDE_MARK`/`fixture::AWAY_SIDE_MARK`) goes in by type; a club in another slice goes in with 0x14 or
/// 1 by `fixture::TIME_SLICE` and by type 1; the same stadium (`fixture::STADIUM`) outside the competition of
/// `KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION`, likewise; the national teams of the clubs' nation
/// (`exe::database_get_national_team`: push; add esp, 4 -> cdecl) and the clubs with `.reputation()` above 5000 by the
/// competition rule (+0x60, `comp_clash_rule`); fixtures of the competition of `KEY_CLUB_COMP_AMERICAN_MLS_ALL_STARS`
/// with a club of the league of `KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE` give 19; a fixture already played
/// (`fixture::HOME_GOALS`) gives 0x16. Missing club: the box and the game ends (as in the original).
/// Original: `FIXTURE_MANAGER::collect_day_clashes` (0x5adba0, fix_man.cpp, thiscall, ret 0x1c).
pub(crate) unsafe extern "thiscall" fn collect_day_clashes(this: usize, game: usize, grav: *mut i32, list: usize, nations: usize, n: u32, data: usize, desloc: u32) {
    let game = Fixture(game);
    // 0x53b900 (push; add esp, 4 -> cdecl), 0x53b3d0 (push; add esp, 4 -> cdecl)
    // 0x76d940: ecx = [0xb74318]; push comp -> thiscall, ret 4 (the Rust copy ignores ecx)
    let e_sel = exe::database_is_national_team(game.home_club());
    let n1 = game.home_club().nation().addr();
    // simplification: without a nation the original leaves the national team uninitialized; every club has a nation, so 0 does not happen in the game
    let s1 = if n1 != 0 { exe::database_get_national_team(n1) } else { 0 };
    let n2 = game.away_club().nation().addr();
    let s2 = if n2 != 0 { exe::database_get_national_team(n2) } else { 0 };
    let flag = desloc as u8 as i8;
    let mut day = [0u8; 8];
    exe::date_add_days(data, (&mut day) as *mut [u8; 8] as usize, flag as i16 as u16 as u32);
    let d = day.as_ptr() as usize;
    let year = || read_usize(d + 2) as u32; // the year dword as the original pushes it (0x549f70 and 0x5abf10 read the word)
    let same = |t: (u16, u16)| t.1 == read_u16(d + 2) && t.0 == read_u16(d);
    let dezenove = || {
        if *grav < 0x13 {
            *grav = 0x13;
        }
    };
    let kind = game.category();
    if kind == 1 && flag != 0 {
        return;
    }
    if flag == 0 {
        let comp = game.competition();
        let nc = if !comp.is_null() { comp.nation() } else { Nation(0) };
        let and_of_the = |g: usize| !nc.is_null() && read_i32(nc.addr()) == read_i32(g);
        if same(day_of_year(0x19, 0xb, year())) && !and_of_the(KEY_NATION_JAPAN) {
            covered!("cqc_3 Christmas");
            return dezenove();
        }
        if and_of_the(KEY_NATION_NORTHERN_IRELAND) && exe::date_get_weekday((&day) as *const [u8; 8] as usize) as i16 == 6 {
            return dezenove();
        }
        if and_of_the(KEY_NATION_GREECE) {
            let t = day_of_year(10, 3, year());
            if t.1 < read_u16(d + 2) || (t.1 == read_u16(d + 2) && (t.0 as i16) < read_u16(d) as i16) {
                let t = day_of_year(0x14, 3, year());
                if t.1 > read_u16(d + 2) || (t.1 == read_u16(d + 2) && (t.0 as i16) > read_u16(d) as i16) {
                    return dezenove();
                }
            }
        }
    }
    let nation = read_usize(G_NATION_TABLE).wrapping_add((read_i32(KEY_NATION_NORWAY) as usize).wrapping_mul(nation::SIZE));
    if game.home_club().nation().addr() == nation || game.away_club().nation().addr() == nation {
        if same(day_of_year(0x11, 4, year())) && flag == 0 {
            return dezenove();
        }
    }
    let mut k: i8 = 0;
    while k < n as u8 as i8 {
        let p = nations.wrapping_add(k as isize as usize);
        if read_u8(p) != 0xff {
            let mut slice: i8 = 0;
            while slice <= 2 {
                let l = exe::fixture_manager_get_fixture_list(this, year(), read_usize(d) as u32, slice as u8 as u32, read_u8(p) as u32);
                if l != 0 {
                    write_usize(l + llist::CURSOR, read_usize(l + 2));
                    let mut x = exe::llist_get_next(l);
                    while x != 0 {
                        let c1 = Club(read_usize(x + fixture::HOME_CLUB));
                        let h = game.home_club();
                        let a = game.away_club();
                        if c1.is_null() || h.is_null() || a.is_null() {
                            // the box (line 0xcc8) and the end of the game: the original
                            return original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut i32, usize, usize, u32, usize, u32)>(IDX_COLLECT_DAY_CLASHES)(this, game.addr(), grav, list, nations, n, data, desloc);
                        }
                        let c2 = Club(read_usize(x + fixture::AWAY_CLUB));
                        let (sx0, sx1, s0, s1b) = (read_u8(x + fixture::HOME_SIDE_MARK), read_u8(x + fixture::AWAY_SIDE_MARK), game.home_side_mark(), (game.away_side_mark() as u8));
                        if (c1 == h && sx0 == s0) || (c1 == a && sx0 == s1b) || (c2 == h && sx1 == s0) || (c2 == a && sx1 == s1b) {
                            covered!("cqc_3 same club in the same pot");
                            add_clash_by_type(grav, list, Fixture(x), true); // cqc3: the same club in the same slice
                        } else if (flag == 0 && c1 == h && sx0 != s0) || (c1 == a && sx0 != s1b) || (c2 == h && sx1 != s0) || (c2 == a && sx1 != s1b) {
                            if read_u8(x + fixture::HOME_GOALS) != 0xff {
                                *grav = 0x16;
                            } else {
                                append_fixture(list, x);
                                if game.time_slice() == read_u8(x + fixture::TIME_SLICE) || read_u8(x + fixture::CATEGORY) != 1 {
                                    if *grav == 0 {
                                        covered!("cqc_3 same club in another pot");
                                    }
                                    default_clash_severity(grav, true); // cqc3: the same club in another pot
                                } else if game.category() != match_category::FRIENDLY {
                                    default_clash_severity(grav, false);
                                }
                            }
                        } else if flag == 0 && read_usize(x + fixture::STADIUM) != 0 && game.stadium() == read_usize(x + fixture::STADIUM) && read_i32(x) != read_i32(KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION) && read_i32(game.addr()) != read_i32(KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION) {
                            if read_u8(x + fixture::HOME_GOALS) != 0xff {
                                *grav = 0x16;
                            } else {
                                append_fixture(list, x);
                                let t = read_u8(x + fixture::CATEGORY);
                                if game.time_slice() == read_u8(x + fixture::TIME_SLICE) {
                                    if t != 1 {
                                        default_clash_severity(grav, true);
                                    } else if *grav < 1 {
                                        *grav = 1;
                                    }
                                } else if t != 1 {
                                    default_clash_severity(grav, true);
                                } else if game.category() != match_category::FRIENDLY {
                                    default_clash_severity(grav, false);
                                }
                            }
                        } else {
                            let abs_flag = (flag as i32).abs();
                            let national_teams = (e_sel == 0
                                && exe::national_teams_is_finals_comp(read_usize(G_NATIONAL_TEAMS), read_usize(x + fixture::COMPETITION)) == 0
                                && abs_flag < 2
                                && (read_u8(p) == 0 || read_u8(p) == 1)
                                && (game.category() as i8) < 5
                                && c1.addr() == s1)
                                || c1.addr() == s2
                                || c2.addr() == s1
                                || c2.addr() == s2;
                            if national_teams {
                                if ((h.reputation() as u16) as i16 > 0x1388 || (a.reputation() as u16) as i16 > 0x1388) && comp_clash_rule(game.addr(), game.addr(), x) != 0 {
                                    add_clash_by_type(grav, list, Fixture(x), true);
                                }
                            } else if read_i32(x) == read_i32(KEY_CLUB_COMP_AMERICAN_MLS_ALL_STARS) && (is_in_league(h, read_i32(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE)) || is_in_league(a, read_i32(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE))) {
                                *grav = 0x13;
                            } else if read_i32(game.addr()) == read_i32(KEY_CLUB_COMP_AMERICAN_MLS_ALL_STARS) && (is_in_league(c1, read_i32(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE)) || is_in_league(c2, read_i32(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE))) {
                                add_clash_by_type(grav, list, Fixture(x), false);
                            } else if e_sel != 0
                                && exe::national_teams_is_finals_comp(read_usize(G_NATIONAL_TEAMS), game.competition()) == 0
                                && abs_flag < 2
                                && read_u8(p) != 0
                                && read_u8(p) != 1
                            {
                                let (nx1, nx2) = (c1.nation().addr(), c2.nation().addr());
                                if (nx1 == n1 || nx2 == n1 || nx1 == n2 || nx2 == n2)
                                    && ((c1.reputation() as u16) as i16 > 0x1388 || (c2.reputation() as u16) as i16 > 0x1388)
                                    && (read_u8(x + fixture::CATEGORY) as i8) < 5
                                    && comp_clash_rule(x, x, game.addr()) != 0
                                {
                                    add_clash_by_type(grav, list, Fixture(x), true);
                                }
                            }
                        }
                        x = exe::llist_get_next(l);
                    }
                }
                slice += 1;
            }
        }
        k += 1;
    }
}

/// Finds a fixture's clashes: builds the nations to check, creates the clash list and collects the clashes of the
/// surrounding days; returns the severity and the list, or 0 with no clash.
///
/// `FIXTURE_MANAGER::find_fixture_clashes(game, out, two_days)`, used by the calendar (`add_fixture`,
/// `find_new_fixture_date`, `resolve_same_day_clash`, `move_nearby_club_fixtures`); SEH frame for the new list. Without
/// a fixture or without an output: the box (as in the original). It zeroes the output; the nations to check, without
/// repeats: 0 to 6, the fixture's key nation (`exe::comp_active_key_nation_of_fixture`: 2 pushes; add esp, 8 -> cdecl),
/// those of the two clubs' leagues, the stadium's (`exe::comp_active_key_nation_of_stadium`: push; add esp, 4 ->
/// cdecl), the partners (`exe::fix_man_get_partner_key_nation_index`: push; add esp, 4 -> cdecl) and, with a home
/// national team (`exe::database_is_national_team`), those of the two national teams
/// (`exe::comp_active_key_nation_of_club`: push; add esp, 4 -> cdecl). Key 3 or 5 outside the competition of
/// `KEY_CLUB_COMP_FIFA_CLUB_WORLD_CHAMPIONSHIP` turns off the two days before. The new list (new 0xe:
/// `exe::crt_operator_new`, push; add esp, 4 -> cdecl; `exe::new_llist`: ecx = block -> thiscall; out of memory, the
/// box and 0) and the clashes of each day (`exe::fixture_manager_collect_day_clashes`: ecx = this; 7 pushes ->
/// thiscall, ret 0x1c) from the fixture's date (`exe::new_date`: push `.year()`; push `.day()`; ecx = local ->
/// thiscall): below `.category()` 0x17, the two days before (with the flag) and the day before; the day itself and the
/// day after; again the two days before with the flag. With no clash: the list is destroyed and freed
/// (`exe::drop_llist`: ecx = list -> thiscall; `exe::crt_operator_delete`: push; add esp, 4 -> cdecl), 0x15 in the
/// output if the national team forbids it (`exe::nationalteams_can_schedule_club_fixture`: ecx = [`G_NATIONAL_TEAMS`];
/// push fixture -> thiscall, ret 4) and 0. Otherwise the severity in the output and the list.
/// Original: `FIXTURE_MANAGER::find_fixture_clashes` (0x5ad780, fix_man.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn find_fixture_clashes(this: usize, game: usize, out: *mut i32, two_days: u32) -> usize {
    let game = Fixture(game);
    if game.is_null() || out.is_null() {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut i32, u32) -> usize>(IDX_FIND_FIXTURE_CLASHES)(this, game.addr(), out, two_days); // a caixa
    }
    let mut two = two_days;
    *out = 0;
    let mut a = [0xffu8; 16];
    for k in 0..7 {
        a[k] = k as u8;
    }
    let key = exe::comp_active_key_nation_of_fixture(game.competition(), game) as i8;
    a[7] = key as u8;
    let mut n: usize = 8;
    let home = game.home_club();
    if !home.is_null() && home.division().addr() != 0 {
        a[8] = exe::comp_active_key_nation_of_fixture(home.division().addr(), 0);
        n = 9;
    }
    let away = game.away_club();
    if !away.is_null() && away.division().addr() != 0 {
        a[n] = exe::comp_active_key_nation_of_fixture(away.division().addr(), 0);
        n += 1;
    }
    if game.stadium() != 0 {
        a[n] = exe::comp_active_key_nation_of_stadium(game.stadium());
        n += 1;
    }
    a[n] = exe::fix_man_get_partner_key_nation_index(game.home_club());
    n += 1;
    a[n] = exe::fix_man_get_partner_key_nation_index(game.away_club());
    n += 1;
    if read_i32(game.addr()) != read_i32(KEY_CLUB_COMP_FIFA_CLUB_WORLD_CHAMPIONSHIP) && (key == 3 || key == 5) {
        two = 0;
    }
    if exe::database_is_national_team(game.home_club()) != 0 {
        a[n] = exe::comp_active_key_nation_of_club(game.home_club());
        n += 1;
        a[n] = exe::comp_active_key_nation_of_club(game.away_club());
        n += 1;
    }
    for i in 0..n.saturating_sub(1) {
        for j in i + 1..n {
            if (a[i] as i8) > -1 && a[i] == a[j] {
                a[j] = 0xff;
            }
        }
    }
    let block = exe::crt_operator_new(0xe);
    let list = if block != 0 { exe::new_llist(block) } else { 0 };
    if list == 0 {
        show_error_at(0x9cc208, 0xc1d); // "E:\dev\CM3\cm3 00-01\cm3\code\comp\fix_man.cpp"
        return 0;
    }
    let mut data = [0u8; 8];
    exe::new_date((&mut data) as *mut [u8; 8] as usize, game.day() as u32, game.year() as u32);
    let mut g: i32 = 0;
    let (pa, pd) = (a.as_ptr() as usize, data.as_ptr() as usize);
    if (game.category() as i8) < 0x17 {
        if two != 0 {
            exe::fixture_manager_collect_day_clashes(this, game, &mut g, list, pa, n as u32, pd, -2i32 as u32);
        }
        exe::fixture_manager_collect_day_clashes(this, game, &mut g, list, pa, n as u32, pd, -1i32 as u32);
    }
    exe::fixture_manager_collect_day_clashes(this, game, &mut g, list, pa, n as u32, pd, 0);
    covered!("cqc_3 next day");
    exe::fixture_manager_collect_day_clashes(this, game, &mut g, list, pa, n as u32, pd, 1); // cqc3: o dia seguinte
    if two != 0 {
        exe::fixture_manager_collect_day_clashes(this, game, &mut g, list, pa, n as u32, pd, -2i32 as u32);
    }
    if g == 0 {
        exe::drop_llist(list);
        exe::crt_operator_delete(list);
        if exe::nationalteams_can_schedule_club_fixture(read_usize(G_NATIONAL_TEAMS), game) == 0 {
            *out = 0x15;
        }
        return 0;
    }
    *out = g;
    list
}

/// Clears the leg check list at `G_FIXTURE_LEGS_LIST` and refills it with the 15-byte records of
/// check_fixture_legs.tmp.
///
/// `fix_man_load_fixture_leg_checks()` (cdecl), from the construction of the main calendar (`exe::new_fixture_manager`,
/// on load and on game creation): empties the leg check list `G_FIXTURE_LEGS_LIST` (`exe::llist_clear`: ecx = list ->
/// thiscall) and rebuilds it with the 15-byte records of check_fixture_legs.tmp in the temporary folder
/// (`exe::utils_build_file_path`: 2 pushes; add esp, 8 -> cdecl; fopen `exe::crt_fopen` "rb", fread `exe::crt_fread`,
/// fclose `exe::crt_fclose`: cdecl), using the 16-bit count at the start; a short record: the box (line 0x10b9) and
/// whatever was left in the record goes in (`exe::llist_add_tail`).
/// Original: `fix_man_load_fixture_leg_checks` (0x5af390, fix_man.cpp, cdecl).
pub(crate) unsafe extern "C" fn fix_man_load_fixture_leg_checks() {
    exe::llist_clear(G_FIXTURE_LEGS_LIST);
    let f = exe::crt_fopen(exe::utils_build_file_path(0x987da0, 0x9cc258), 0x98f928); // "CM3_TEMP", "check_fixture_legs.tmp", "rb"
    if f == 0 {
        return;
    }
    // simplification: when reading the count fails the original uses stack garbage; here 0 (the game’s own file)
    let mut count = [0u8; 2];
    exe::crt_fread((count.as_mut_ptr()) as usize, 2, 1, f);
    let count = i16::from_le_bytes(count);
    let mut reg = [0u8; 15];
    let mut k: i16 = 0;
    while k < count {
        if exe::crt_fread((reg.as_mut_ptr()) as usize, 0xf, 1, f) != 1 {
            show_error_at(0x9cc208, 0x10b9);
        }
        exe::llist_add_tail(G_FIXTURE_LEGS_LIST, (reg.as_ptr()) as *const u8 as usize, 0xf);
        k = k.wrapping_add(1);
    }
    exe::crt_fclose(f);
}

pub(crate) type RawDate = [u8; 8];

/// True if date a is after date b (year +2 unsigned, then day signed).
pub(crate) unsafe fn fix_man_date_is_after(a: usize, b: usize) -> bool {
    let (ya, yb) = (read_u16(a + 2), read_u16(b + 2));
    ya > yb || (ya == yb && (read_u16(a) as i16) > (read_u16(b) as i16))
}

/// The date operations (thiscall; the +n ones return their static date or `ret`).
///
/// `exe::date_date_54a110` the default date, 0x549ef0 the date (day, year) (push year; push day), `exe::date_operator_3d` the assignment (push
/// source; push ret), `exe::date_add_days` + days, 0x54c8e0 − days, `exe::date_add_weeks` + weeks (push n; push ret), `exe::date_get_nearest_weekday` the target
/// weekday (push dest; push target), `exe::date_get_weekday` the weekday (no pushes).
pub(crate) unsafe fn date_default(d: &mut RawDate) {
    exe::date_date_54a110((d) as *mut RawDate as usize);
}
/// Builds a date from (day, year) (Date::Date).
pub(crate) unsafe fn date_from_day_year(d: &mut RawDate, day: u16, year: u16) -> usize {
    exe::new_date((d) as *mut RawDate as usize, day as u32, year as u32)
}
/// Assigns date `fonte` to d (Date operator=).
pub(crate) unsafe fn date_assign(d: &mut RawDate, font: usize) {
    let mut ret = [0u8; 8];
    exe::date_operator_3d((d) as *mut RawDate as usize, (&mut ret) as *mut RawDate as usize, font);
}
/// Calls a date arithmetic routine (add/sub days or weeks) with a local return buffer and returns a pointer to the
/// result.
pub(crate) unsafe fn date_op(routine: usize, d: usize, n: u32) -> usize {
    let f: unsafe extern "thiscall" fn(usize, *mut RawDate, u32) -> usize = core::mem::transmute(routine);
    let mut ret = [0u8; 8];
    // simplification: the original passes a ret on its stack; all three write into it or into the static and return the pointer,
    // which is read right after (before the next call), as in the original
    let r = f(d, &mut ret, n);
    if r == ret.as_ptr() as usize {
        DATE_OP_RESULT = ret;
        return core::ptr::addr_of!(DATE_OP_RESULT) as usize;
    }
    r
}
pub(crate) static mut DATE_OP_RESULT: RawDate = [0; 8];
/// Weekday of a date (`exe::date_get_weekday`).
pub(crate) unsafe fn date_weekday(d: usize) -> i32 {
    exe::date_get_weekday(d)
}
/// Writes into dest the nearest date with the target weekday (`exe::date_get_nearest_weekday`).
pub(crate) unsafe fn nearest_weekday(d: usize, target: u32, dest: &mut RawDate) {
    exe::date_get_nearest_weekday(d, target, (dest) as *mut RawDate as usize);
}

/// Time slice for a competition on a day (FIXTURE_MANAGER get_time_slice `get_time_slice`).
pub(crate) unsafe fn fixture_time_slice(this: usize, comp: usize, day: u32) -> u8 {
    exe::fixture_manager_get_time_slice(this, comp, day)
}

/// Clashes of a fixture (`find_fixture_clashes`): returns (clash list or 0, severity).
pub(crate) unsafe fn fix_man_find_fixture_clashes(this: usize, game: usize, two: u32) -> (usize, i32) {
    let mut g: i32 = 0;
    let l = exe::fixture_manager_find_fixture_clashes(this, game, &mut g, two);
    (l, g)
}

/// Destroys and deletes a LLIST.
pub(crate) unsafe fn free_list(l: usize) {
    exe::drop_llist(l);
    exe::crt_operator_delete(l);
}

/// Finds a new date for a fixture: within a window by fixture type, it tries weekdays in a fixed order and keeps the
/// first date with no (or tolerable) clash; otherwise 2 days later.
///
/// `FIXTURE_MANAGER::find_new_fixture_date(game)`, used by calendar entry (`add_fixture`), by same-day clashes
/// (`resolve_same_day_clash`) and by rescheduling (`move_nearby_club_fixtures`); SEH frame only for the dates. Without
/// a fixture: the box (as in the original). If the units digit of the round (`.round_name()`,
/// `exe::comp_util_get_round_variant`: push; add esp, 4 -> cdecl) is 3 or 4, the fixture's 15-byte record goes into the
/// leg check list `G_FIXTURE_LEGS_LIST` (`exe::llist_add_tail`). The window by type (`.category()`): 2, up to the date
/// at `.next_round_year()`/`.next_round_day()` (without it, the box and 2 weeks), starting 4 weeks later (if the end is
/// more than 20 weeks away) or 3 days later; 4 and 6, from 3 days to 2 weeks; 5, from 0 to 1 week; 7, 8 and 10, from 3
/// days before to 1 week; the others, none. In two passes (the second checking the two days before), through the
/// weekdays in the order 2, 1, 5, 6, 0, 3, 4, every 4 days of the window: the target day (`nearest_weekday`) from 2
/// days after today and inside the window becomes the date (`.set_year()`, `.set_day()`) and the time slot
/// (`.set_time_slice()`, `fixture_time_slice`); with no clash (`fix_man_find_fixture_clashes`), or with severity 1 or
/// below the type, it stays. Not found: 2 days after the fixture's date (or after today, if it has already passed).
/// Original: `FIXTURE_MANAGER::find_new_fixture_date` (0x5ab510, fix_man.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn find_new_fixture_date(this: usize, game: usize) {
    let game = Fixture(game);
    if game.is_null() {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_FIND_NEW_FIXTURE_DATE)(this, game.addr()); // a caixa
    }
    let (mut s, mut e, mut j, mut h, mut k) = ([0u8; 8], [0u8; 8], [0u8; 8], [0u8; 8], [0u8; 8]);
    date_default(&mut s);
    date_default(&mut e);
    date_default(&mut j);
    date_default(&mut h);
    date_default(&mut k);
    let days: [i16; 9] = [2, 1, -1, 5, 6, -1, 0, 3, 4]; // cqc4: the order of the weekdays
    let mut tmp = [0u8; 8];
    let r = date_from_day_year(&mut tmp, game.day(), game.year());
    date_assign(&mut j, r);
    let round = (game.round_name() as u16) as i16 as i32;
    if exe::comp_util_get_round_variant(round) == 3 || exe::comp_util_get_round_variant(round) == 4 {
        let mut reg = [0u8; 15];
        reg[0..4].copy_from_slice(&core::slice::from_raw_parts(game.addr() as *const u8, 4));
        reg[4] = game.stage_index();
        reg[5..7].copy_from_slice(&(game.round_number() as u16).to_le_bytes());
        reg[7..11].copy_from_slice(&core::slice::from_raw_parts((game.addr() + fixture::HOME_CLUB_ID) as *const u8, 4));
        reg[11..15].copy_from_slice(&core::slice::from_raw_parts((game.addr() + fixture::AWAY_CLUB_ID) as *const u8, 4));
        exe::llist_add_tail(G_FIXTURE_LEGS_LIST, (reg.as_ptr()) as *const u8 as usize, 0xf);
    }
    let pj = j.as_ptr() as usize;
    match (game.category() as i8 as i32).wrapping_sub(2) {
        0 => {
            if game.next_round_year() != 0 {
                let mut t = [0u8; 8];
                let r = date_from_day_year(&mut t, game.next_round_day(), game.next_round_year());
                date_assign(&mut e, r);
            } else {
                show_error_at(0x9cc208, 0x4f9); // "E:\dev\CM3\cm3 00-01\cm3\code\comp\fix_man.cpp"
                let r = date_op(0x54c770, pj, 2);
                date_assign(&mut e, r);
            }
            let end = e;
            let r = date_op(0x54c770, pj, 0x14);
            let r = if fix_man_date_is_after(end.as_ptr() as usize, r) { date_op(0x54c770, pj, 4) } else { date_op(0x54c5b0, pj, 3) };
            date_assign(&mut s, r);
        }
        2 | 4 => {
            let r = date_op(0x54c770, pj, 2);
            date_assign(&mut e, r);
            let r = date_op(0x54c5b0, pj, 3);
            date_assign(&mut s, r);
        }
        3 => {
            let r = date_op(0x54c770, pj, 1);
            date_assign(&mut e, r);
            date_assign(&mut s, pj);
        }
        5 | 6 | 8 => {
            let r = date_op(0x54c770, pj, 1);
            date_assign(&mut e, r);
            let r = date_op(0x54c8e0, pj, 3);
            date_assign(&mut s, r);
        }
        _ => {}
    }
    let (ps, pe, ph, pk) = (s.as_ptr() as usize, e.as_ptr() as usize, h.as_ptr() as usize, k.as_ptr() as usize);
    for passada in 0..2u8 {
        for i in 0..3usize {
            for jj in 0..3usize {
                let target = days[i * 3 + jj];
                if target == -1 {
                    continue;
                }
                date_assign(&mut k, ps);
                while fix_man_date_is_after(pe, pk) {
                    nearest_weekday(pk, target as u16 as u32, &mut h);
                    let t = date_op(0x54c5b0, G_CURRENT_DATE, 2);
                    let t = [read_usize(t), read_usize(t + 4)];
                    let pt = t.as_ptr() as usize;
                    if !fix_man_date_is_after(pt, ph) && !fix_man_date_is_after(ph, pe) {
                        covered!("cqc_4 new date");
                        game.set_year(read_u16(ph + 2));
                        game.set_day(read_u16(ph));
                        game.set_time_slice(fixture_time_slice(this, game.competition().addr(), target as u16 as u32));
                        let (l, g) = fix_man_find_fixture_clashes(this, game.addr(), (passada == 1) as u32);
                        if l == 0 {
                            if g == 0 {
                                return;
                            }
                        } else {
                            free_list(l);
                        }
                        if g == 1 || (game.category() as i8 as i32) > g {
                            return;
                        }
                    }
                    let r = date_op(0x54c5b0, pk, 4); // cqc4: the search step
                    date_assign(&mut k, r);
                }
            }
        }
    }
    if fix_man_date_is_after(G_CURRENT_DATE, pj) {
        date_assign(&mut j, G_CURRENT_DATE);
    }
    let r = date_op(0x54c5b0, pj, 2);
    date_assign(&mut j, r);
    game.set_day(read_u16(pj));
    game.set_year(read_u16(pj + 2));
}

/// Adds a fixture to the calendar: fixes the stage, reschedules late fixtures, moves weekend league fixtures to the
/// home club's preferred day, resolves clashes, handles national team fixtures and neighbours, and puts the fixture in
/// the day list or in the other year's block.
///
/// `FIXTURE_MANAGER::add_fixture(game, clear)`, used by the cups, leagues, friendlies and rescheduling (20 callers);
/// SEH frame for the dates and the new manager. `clear` zeroes the count of chained reschedulings (word
/// `G_ADD_FIXTURE_RETRY_COUNT`). It fixes the stage (`exe::fix_man_record_club_fixture_years`: push; add esp, 4 ->
/// cdecl), `.set_referee(−1)`; a fixture whose date has already passed and is not played: new date
/// (`exe::fixture_manager_find_new_fixture_date`). A league fixture of the home club's league (`.division()` or
/// `.reserve_division()`) with no round (`.round_name()`, `.round_prefix()`) and no stage (`.stage()`), once (bit 0x800
/// of `.squad_rules()`): on Saturday or Sunday, it moves to the home club's preferred day (`.match_day()`: 4, 5 or 6)
/// if there is no clash there. With fewer than 10 chained reschedulings, the clashes (`fix_man_find_fixture_clashes`):
/// severity 0x14, the same-day clash (`exe::fixture_manager_resolve_same_day_clash`) and 1; a cup or type 9 refused by
/// the competition (`comp_vtable::ACCEPTS_FIXTURE`): 0; severity at least the type: 0 if the fixture's national team
/// has a call-up (`exe::comp_is_reserve_or_minor_comp` on [`G_RESERVE_COMP`]), otherwise a new date and it goes in
/// again; otherwise it goes in (`exe::fixture_manager_insert_fixture`) and the clashing fixtures leave: the friendlies
/// (`FN_FRIENDLY_IS_FRIENDLY_COMPETITION`/`exe::friendly_on_friendly_fixture_cancelled` on [`G_FRIENDLY`]), type 9
/// (`exe::fixture_manager_remove_fixture`), the others rescheduled (`exe::fixture_manager_remove_fixture`,
/// `exe::fixture_manager_find_new_fixture_date`, `exe::fixture_manager_add_fixture`) with the news
/// (`FN_FIXTURE_MANAGER_QUEUE_FIXTURE_MOVED_NEWS` on [`G_FIXTURE_MANAGER`]). In the block's year
/// (`fixture_manager::YEAR`): the key nation (`exe::comp_active_key_nation_of_fixture`), the order
/// (`exe::fixture_manager_add_away_trip_illness_risk`), the national team fixtures
/// (`exe::reservecomp_add_b_or_under21_fixture` on [`G_RESERVE_COMP`], `exe::nationalteams_select_squads_for_fixture`
/// on [`G_NATIONAL_TEAMS`]), the neighbours (`exe::fixture_manager_move_nearby_club_fixtures`, key 3 or 5); if it did
/// not go in, into the list of the day, slice and key (`exe::llist_add_tail`). Outside the year, into the other year's
/// block (`fixture_manager::OTHER_YEAR`: `exe::fixture_manager_set_year` for the year, or a new one of 0x1184 bytes,
/// `exe::new_fixture_manager`; out of memory, the box and 0).
/// Original: `FIXTURE_MANAGER::add_fixture` (0x5aa680, fix_man.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn add_fixture(this: usize, game: usize, clear: u32) -> u32 {
    let game = Fixture(game);
    let count = G_ADD_FIXTURE_RETRY_COUNT as *mut i16;
    let mut entrou = false;
    if clear != 0 {
        *count = 0;
    }
    exe::fix_man_record_club_fixture_years(game);
    game.set_referee(0xffffu16 as i16);
    let today = [read_usize(G_CURRENT_DATE), read_usize(G_CURRENT_DATE_LEAP)];
    let mut dj = [0u8; 8];
    let r = date_from_day_year(&mut dj, game.day(), game.year());
    if !fix_man_date_is_after(r, today.as_ptr() as usize) && (game.home_goals() as u8) == 0xff {
        exe::fixture_manager_find_new_fixture_date(this, game);
    }
    let marks = game.squad_rules();
    let home = game.home_club();
    let comp = game.competition().addr();
    if marks & 0x800 == 0
        && (comp == home.division().addr() || comp == home.reserve_division().addr())
        && (game.round_prefix() as u16) == 0
        && (game.round_name() as u16) == 0
        && game.stage() == 0xff
    {
        game.set_squad_rules(marks | 0x800);
        let preferred = home.match_day();
        if preferred != 0xff && (preferred == 5 || preferred == 6 || preferred == 4) {
            let mut d = [0u8; 8];
            date_from_day_year(&mut d, game.day(), game.year());
            let pd = d.as_ptr() as usize;
            let day = date_weekday(pd) as u8;
            if preferred != day && (day == 5 || day == 6) {
                let mut t = [0u8; 8];
                date_default(&mut t);
                covered!("cqc_4 home side preferred day");
                nearest_weekday(pd, preferred as i8 as i16 as u16 as u32, &mut t); // cqc4: the home side’s preferred day
                let pt = t.as_ptr() as usize;
                game.set_year(read_u16(pt + 2));
                game.set_day(read_u16(pt));
                game.set_time_slice(fixture_time_slice(this, game.competition().addr(), date_weekday(pt) as u32));
                let (l, g) = fix_man_find_fixture_clashes(this, game.addr(), 0);
                if l != 0 || g != 0 {
                    game.set_day(read_u16(pd));
                    game.set_year(read_u16(pd + 2));
                    game.set_time_slice(fixture_time_slice(this, game.competition().addr(), date_weekday(pd) as u32));
                    if l != 0 {
                        free_list(l);
                    }
                }
            }
        }
    }
    if *count < 10 {
        let (l, g) = fix_man_find_fixture_clashes(this, game.addr(), 0);
        if l != 0 || g != 0 {
            *count = (*count).wrapping_add(1);
            if g == 0x14 {
                covered!("cqc_4 same day");
                exe::fixture_manager_resolve_same_day_clash(this, game, l); // cqc4: the same-day conflict
                if l != 0 {
                    free_list(l);
                }
                return 1;
            }
            let kind = game.category();
            if (kind == 1 || kind == 9) && game.competition().addr() != 0 && read_usize(G_COMP_OBJECT_TABLE) != 0 {
                let c = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((read_usize(game.competition().addr()) as usize).wrapping_mul(4)));
                if c != 0 {
                    let accepted: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(read_usize(read_usize(c) + comp_vtable::ACCEPTS_FIXTURE));
                    if accepted(c, game.addr()) == 0 {
                        if l != 0 {
                            free_list(l);
                        }
                        return 0;
                    }
                }
            }
            // 0x7e6f20: mov ecx, [0xb74434]; push comp -> thiscall, ret 4
            if g >= game.category() as i8 as i32 {
                if l != 0 {
                    free_list(l);
                }
                if exe::comp_is_reserve_or_minor_comp(read_usize(G_RESERVE_COMP), game.competition().addr()) != 0 {
                    return 0;
                }
                exe::fixture_manager_find_new_fixture_date(this, game);
                return (exe::fixture_manager_add_fixture(this, game, 0) != 0) as u32;
            }
            exe::fixture_manager_insert_fixture(this, game);
            entrou = true;
            if l != 0 {
                let friendly: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FRIENDLY_IS_FRIENDLY_COMPETITION);
                let news: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_FIXTURE_MANAGER_QUEUE_FIXTURE_MOVED_NEWS);
                write_usize(l + llist::CURSOR, read_usize(l + 2));
                let mut x = exe::llist_get_next(l);
                while x != 0 {
                    if friendly(read_usize(G_FRIENDLY), read_usize(x + fixture::COMPETITION)) != 0 {
                        exe::friendly_on_friendly_fixture_cancelled(read_usize(G_FRIENDLY), x, 0);
                    } else if read_u8(x + fixture::CATEGORY) == 9 {
                        exe::fixture_manager_remove_fixture(this, x);
                    } else {
                        if exe::comp_is_reserve_or_minor_comp(read_usize(G_RESERVE_COMP), game.competition().addr()) != 0 {
                            return 0; // as in the original, without freeing the list
                        }
                        let mut copy = [0u8; 0x4f];
                        core::ptr::copy_nonoverlapping(x as *const u8, copy.as_mut_ptr(), 0x4f);
                        let pc = copy.as_mut_ptr() as usize;
                        if exe::fixture_manager_remove_fixture(this, x) != 0 {
                            exe::fixture_manager_find_new_fixture_date(this, pc);
                            exe::fixture_manager_add_fixture(this, pc, 0);
                        }
                        news(read_usize(G_FIXTURE_MANAGER), pc, game.addr(), 0);
                    }
                    x = exe::llist_get_next(l);
                }
                free_list(l);
            }
        }
    }
    let year = game.year();
    let mut key: i8 = 0;
    if year == read_u16(this + fixture_manager::YEAR) {
        key = exe::comp_active_key_nation_of_fixture(game.competition().addr(), game) as i8;
        exe::fixture_manager_add_away_trip_illness_risk(this, game);
        if exe::database_is_national_team(game.home_club().addr()) != 0 && exe::database_is_national_team(game.away_club().addr()) != 0 && (game.home_goals() as u8) == 0xff {
            let c = game.competition().addr();
            let id = read_usize(c) as i32;
            if id != read_usize(KEY_CLUB_COMP_B_INTERNATIONAL) as i32 && id != read_usize(KEY_CLUB_COMP_UNDER_21_INTERNATIONAL) as i32 && game.flags() & 3 == 0 {
                let friendly: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FRIENDLY_IS_FRIENDLY_COMPETITION);
                if exe::national_teams_is_finals_comp(read_usize(G_NATIONAL_TEAMS), c) == 0 || friendly(read_usize(G_FRIENDLY), game.competition().addr()) != 0 {
                    exe::reservecomp_add_b_or_under21_fixture(read_usize(G_RESERVE_COMP), game);
                }
            }
            exe::nationalteams_select_squads_for_fixture(read_usize(G_NATIONAL_TEAMS), game);
        }
        if key == 3 || key == 5 {
            exe::fixture_manager_move_nearby_club_fixtures(this, game);
        }
    }
    if entrou {
        return 1;
    }
    let year = game.year();
    if year == read_u16(this + fixture_manager::YEAR) {
        let day = game.day() as usize;
        let slice = game.time_slice() as usize;
        let row = read_usize(this + (day * 3 + slice) * 4);
        let list = read_usize(row.wrapping_add((key as isize as usize).wrapping_mul(4)));
        return ((exe::llist_add_tail(list, game, 0x4f) as u32) != 0) as u32;
    }
    let mut other = read_usize(this + fixture_manager::OTHER_YEAR);
    if other != 0 {
        if read_u16(other + fixture_manager::YEAR) != year {
            exe::fixture_manager_set_year(other, year as u32);
        }
    } else {
        let b = exe::crt_operator_new(0x1184);
        other = if b != 0 { exe::new_fixture_manager(b, year as u32, this, 0) } else { 0 };
        write_usize(this + fixture_manager::OTHER_YEAR, other);
        if other == 0 {
            show_error_at(0x9cc208, 0x2e6);
            return 0;
        }
    }
    exe::fixture_manager_add_fixture(read_usize(this + fixture_manager::OTHER_YEAR), game, 0);
    1
}

/// Switches a fixture block to another year: saves the year's fixtures, moves the other block to the neighbouring year
/// if needed, destroys the 366 days' lists, switches the year and leap flag, creates new lists and loads the new year's
/// fixtures.
///
/// `FIXTURE_MANAGER::set_year(year)`, used by calendar entry (`add_fixture`, the other year's block), by saving and
/// removing fixtures and by the friendlies; SEH frame for the new blocks. If the same year (word,
/// `fixture_manager::YEAR`): returns 1. Otherwise: it writes the year's fixtures (`exe::fixture_manager_save_fixtures`:
/// ecx = block; push 0 -> thiscall; on failure, the box and 0); if the other block (`fixture_manager::OTHER_YEAR`) is
/// in the requested year, it moves to the year neighbouring this one's (`exe::fixture_manager_set_year` on it); it
/// destroys and frees the lists of the 366 days × 3 slices × key nations (`G_KEY_NATION_ACTIVE_COUNT`, signed byte;
/// `exe::drop_llist`, `exe::crt_operator_delete`); the other block in the requested year, if this is the main one
/// (`fixture_manager::IS_PRIMARY`), is destroyed and freed (`exe::drop_fixture_manager`: ecx = block -> thiscall;
/// `exe::crt_operator_delete`); the new year, leap flag at `fixture_manager::LEAP_YEAR` (`exe::date_is_leap_year`: push
/// year; add esp, 4 -> cdecl), the new lists (new 0xe: `exe::crt_operator_new`, push; add esp, 4 -> cdecl;
/// `exe::new_llist`: ecx = block -> thiscall; out of memory, the box and 0) and the loading of the year's fixtures
/// (`exe::fixture_manager_load_fixtures`: ecx = block -> thiscall). Returns 1.
/// Original: `FIXTURE_MANAGER::set_year` (0x5aa2d0, fix_man.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn set_year(this: usize, year: u32) -> u32 {
    let a = year as u16;
    if a == read_u16(this + fixture_manager::YEAR) {
        return 1;
    }
    if exe::fixture_manager_save_fixtures(this, 0) == 0 {
        show_error_at(0x9cc208, 0x126);
        return 0;
    }
    let other = read_usize(this + fixture_manager::OTHER_YEAR);
    if other != 0 {
        let oa = read_u16(other + fixture_manager::YEAR);
        if oa == a {
            let vizinho = if read_u16(this + fixture_manager::YEAR) as u32 == oa as u32 + 1 { oa.wrapping_sub(1) } else { oa.wrapping_add(1) };
            exe::fixture_manager_set_year(other, vizinho as u32);
        }
    }
    let n = || read_u8(G_KEY_NATION_ACTIVE_COUNT) as i8;
    for dia in 0..0x16eusize {
        if n() > 0 {
            let base = this + 8 + dia * 0xc;
            let mut k: i16 = 0;
            loop {
                for p in [base - 8, base - 4, base] {
                    let slot = read_usize(p).wrapping_add((k as i32 as usize).wrapping_mul(4));
                    let l = read_usize(slot);
                    if l != 0 {
                        free_list(l);
                        write_usize(read_usize(p).wrapping_add((k as i32 as usize).wrapping_mul(4)), 0);
                    }
                }
                k = k.wrapping_add(1);
                if k >= n() as i16 {
                    break;
                }
            }
        }
    }
    let other = read_usize(this + fixture_manager::OTHER_YEAR);
    if other != 0 && read_u16(other + fixture_manager::YEAR) == a && read_u8(this + fixture_manager::IS_PRIMARY) != 0 {
        exe::drop_fixture_manager(other);
        exe::crt_operator_delete(other);
        write_usize(this + fixture_manager::OTHER_YEAR, 0);
    }
    write_u16(this + fixture_manager::YEAR, a);
    write_u8(this + fixture_manager::LEAP_YEAR, (exe::date_is_leap_year(year) != 0) as u8);
    covered!("cqc_4 calendar new year");
    for dia in 0..0x16eusize {
        if n() > 0 {
            let e = this + dia * 0xc;
            let mut k: i16 = 0;
            loop {
                let desloc = (k as i32 as usize).wrapping_mul(4);
                for j in 0..3usize {
                    let b = exe::crt_operator_new(0xe);
                    let l = if b != 0 { exe::new_llist(b) } else { 0 };
                    write_usize(read_usize(e + j * 4).wrapping_add(desloc), l);
                }
                if (0..3usize).any(|j| read_usize(read_usize(e + j * 4).wrapping_add(desloc)) == 0) {
                    show_error_at(0x9cc208, 0x170);
                    return 0;
                }
                k = k.wrapping_add(1);
                if k >= n() as i16 {
                    break;
                }
            }
        }
    }
    exe::fixture_manager_load_fixtures(this);
    1
}

/// Opens a file by name in the CM3_TEMP folder with fopen and the given mode.
pub(crate) unsafe fn open_temp_file(name: usize, mode: usize) -> usize {
    exe::crt_fopen(exe::utils_build_file_path(0x987da0, name), mode)
}

/// Writes the club dates table `G_CLUB_FIXTURE_YEAR_TABLE`, one word per club ([0xae235c]), into
/// CM3_TEMP\club_dates.tmp.
///
/// `fix_man_save_club_dates()` (cdecl), from the calendar save (FIXTURE_MANAGER::save
/// `exe::fixture_manager_save_fixtures`, 0x5ac394): without the club dates table (`G_CLUB_FIXTURE_YEAR_TABLE`), the
/// "Error" box for line 0x1011 (fix_man.cpp). It writes into CM3_TEMP\club_dates.tmp ("wb") one word per club
/// ([0xae235c] of them, fwrite `exe::crt_fwrite`); a short write: the box for line 0x101b; then it closes the file. If
/// the file does not open: nothing.
/// Original: `fix_man_save_club_dates` (0x5af040, fix_man.cpp, cdecl).
pub(crate) unsafe extern "C" fn fix_man_save_club_dates() {
    if read_usize(G_CLUB_FIXTURE_YEAR_TABLE) == 0 {
        return show_error_at(0x9cc208, 0x1011);
    }
    let f = open_temp_file(0x9cc248, 0x9a7f28);
    if f == 0 {
        return;
    }
    covered!("cpy_7 save club dates");
    if exe::crt_fwrite(read_usize(G_CLUB_FIXTURE_YEAR_TABLE), 2, club_count(), f) != club_count() { // cpy7: the club dates
        show_error_at(0x9cc208, 0x101b);
    }
    uefa_seeding_crt_fclose(f);
}

/// Allocates the club dates table `G_CLUB_FIXTURE_YEAR_TABLE` (one word per club) and reads it from
/// CM3_TEMP\club_dates.tmp, or zeroes it without the file.
///
/// `fix_man_load_club_dates()` (cdecl), from the calendar constructor (`exe::new_fixture_manager`, 0x5a9e38: on a new
/// career and on loading the save). With the table already allocated (`G_CLUB_FIXTURE_YEAR_TABLE`), the "Error" box for
/// line 0xfe4. It allocates one word per club (malloc `exe::crt_malloc` of 2 × [0xae235c]; out of memory, the box for
/// line 0xfec); reads from CM3_TEMP\club_dates.tmp ("rb", fread `exe::crt_fread`; a short read: the box for line 0xff6)
/// and closes it; without the file (a new career), it zeroes the table.
/// Original: `fix_man_load_club_dates` (0x5aee90, fix_man.cpp, cdecl).
pub(crate) unsafe extern "C" fn fix_man_load_club_dates() {
    if read_usize(G_CLUB_FIXTURE_YEAR_TABLE) != 0 {
        return show_error_at(0x9cc208, 0xfe4);
    }
    let t = exe::crt_malloc(club_count().wrapping_mul(2));
    write_usize(G_CLUB_FIXTURE_YEAR_TABLE, t);
    if t == 0 {
        return show_error_at(0x9cc208, 0xfec);
    }
    let f = open_temp_file(0x9cc248, 0x98f928);
    if f == 0 {
        covered!("cpy_7 club dates zeroed");
        core::ptr::write_bytes(read_usize(G_CLUB_FIXTURE_YEAR_TABLE) as *mut u8, 0, club_count().wrapping_mul(2) as usize);
        return;
    }
    covered!("cpy_7 club dates read");
    if exe::crt_fread(read_usize(G_CLUB_FIXTURE_YEAR_TABLE), 2, club_count(), f) != club_count() { // cpy7: the reading of the dates
        show_error_at(0x9cc208, 0xff6);
    }
    uefa_seeding_crt_fclose(f);
}

/// Writes the leg check list `G_FIXTURE_LEGS_LIST` into CM3_TEMP\check_fixture_legs.tmp: the count (2 bytes) and the
/// 15-byte items.
///
/// `fix_man_save_fixture_leg_checks()` (cdecl), from the calendar save (0x5ac399): with the leg check list
/// (`G_FIXTURE_LEGS_LIST`, LLIST) empty (the count word zero), nothing. It writes into CM3_TEMP\check_fixture_legs.tmp
/// ("wb") the count (2 bytes) and, from the start of the list (cursor `G_FIXTURE_LEGS_LIST_CURSOR` ← head
/// `G_FIXTURE_LEGS_LIST_HEAD`), each 15-byte item (`exe::llist_get_next`; a short write: the "Error" box for line
/// 0x10e0, and it carries on); then it closes the file. If the file does not open: nothing.
/// Original: `fix_man_save_fixture_leg_checks` (0x5af4a0, fix_man.cpp, cdecl).
pub(crate) unsafe extern "C" fn fix_man_save_fixture_leg_checks() {
    let count = read_usize(G_FIXTURE_LEGS_LIST);
    if count as u16 == 0 {
        return;
    }
    let f = open_temp_file(0x9cc258, 0x9a7f28);
    if f == 0 {
        return;
    }
    covered!("cpy_7 save leg check");
    exe::crt_fwrite(&count as *const usize as usize, 2, 1, f);
    write_usize(G_FIXTURE_LEGS_LIST_CURSOR, read_usize(G_FIXTURE_LEGS_LIST_HEAD));
    loop {
        let item = exe::llist_get_next(G_FIXTURE_LEGS_LIST);
        if item == 0 {
            break;
        }
        if exe::crt_fwrite(item, 0xf, 1, f) != 1 {
            show_error_at(0x9cc208, 0x10e0);
        }
    }
    uefa_seeding_crt_fclose(f);
}

/// Rehearsal mode of `resolve_same_day_clash` (the comparer). The copy stops at the decision, before touching the calendar.
pub(crate) static SAME_DAY_CLASH_DRY_RUN: AtomicBool = AtomicBool::new(false);

/// Resolves a new fixture falling on the same day as another: the one with higher priority stays and the other is moved
/// to a new date, with the rescheduling news.
///
/// `FIXTURE_MANAGER::resolve_same_day_clash(game, conflitos)`; SEH frame with no destructor on the normal path. The new
/// fixture `game` falls on the same day as another, the first of the `conflitos` list (cursor `llist::CURSOR` goes back
/// to the start +2 and `exe::llist_get_next` reads it); without either of them, the box for line 0xde6 and it returns
/// 1. It copies both (0x4f bytes) and finds the target weekday from the date of the list's fixture (`fixture::DAY`,
/// `fixture::YEAR`; `FN_DATE_DATE`, `exe::date_operator_3d`, `exe::date_get_weekday`): days 0 and 5 of
/// `exe::date_get_weekday` go to 6, 1 and 3 to 2, 2 to 1, 4 and 6 to 5 (`exe::date_get_nearest_weekday` gives the
/// date). Each one's priority is its competition's (`competition::REPUTATION` of `fixture::COMPETITION`), +0x15 (21)
/// with a national team at home (`fixture::HOME_CLUB`, `exe::database_is_national_team`); the list's fixture leaves the
/// day when the new one's priority is higher (or equal, with the new one's home club reputation `club::REPUTATION`
/// higher) and it has not been played yet (`.home_goals()` = 0xff): it is removed from the calendar
/// (`exe::fixture_manager_remove_fixture`) and the new one goes in (`exe::fixture_manager_add_fixture`); otherwise the
/// new one leaves. The one that leaves gets the new date; with a clash there
/// (`exe::fixture_manager_find_fixture_clashes`, the list destroyed and freed: `exe::drop_llist`,
/// `exe::crt_operator_delete`), a fixed fixture (`fixture::CATEGORY` = 1) returns 0, otherwise
/// `exe::fixture_manager_find_new_fixture_date` finds another date; it goes into the calendar
/// (`exe::fixture_manager_add_fixture`) with the rescheduling news (`exe::fixture_manager_queue_fixture_moved_news`,
/// caused by the other one). Returns 1.
/// Original: `FIXTURE_MANAGER::resolve_same_day_clash` (0x5ae380, fix_man.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn resolve_same_day_clash(this: usize, game: usize, conflitos: usize) -> u32 {
    // the dates: ecx = the date, no pushes -> thiscall (0x54a110, 0x54a3f0); 0x549ef0: push [+0x28]; push [+0x2a];
    // 0x417c10: push ret; push &tmp; 0x54a2d0: push &b; push alvo -> thiscall
    let new: unsafe extern "thiscall" fn(*mut [u8; 8], u32, u32) -> *mut [u8; 8] = core::mem::transmute(FN_DATE_DATE);
    // 0x68d1b0: ecx = the list, no pushes -> thiscall; 0x53b900: push clube; add esp, 4 -> cdecl
    let (mut a, mut b) = ([0u8; 8], [0u8; 8]);
    exe::date_date_54a110((&mut a) as *mut [u8; 8] as usize);
    exe::date_date_54a110((&mut b) as *mut [u8; 8] as usize);
    let mut severity: u32 = 0; // [esp+0x10]: the uncased target of the table and the severity from 0x5ad780
    write_usize(conflitos + llist::CURSOR, read_usize(conflitos + 2));
    let other = exe::llist_get_next(conflitos);
    if game == 0 || other == 0 {
        if !SAME_DAY_CLASH_DRY_RUN.load(Relaxed) {
            show_error_at(0x9cc208, 0xde6); // "E:\dev\CM3\cm3 00-01\cm3\code\comp\fix_man.cpp"
        }
        return 1;
    }
    let (mut novo, mut velho) = ([0u8; 0x50], [0u8; 0x50]);
    core::ptr::copy_nonoverlapping(game as *const u8, novo.as_mut_ptr(), 0x4f);
    core::ptr::copy_nonoverlapping(other as *const u8, velho.as_mut_ptr(), 0x4f);
    let (mut tmp, mut tmp2) = ([0u8; 8], [0u8; 8]);
    // simplification: the high bytes of the two 16-bit pushes do not count (0x549ef0 reads the word)
    let r = new(&mut tmp, read_i16(other + fixture::DAY) as u16 as u32, read_i16(other + fixture::YEAR) as u16 as u32);
    exe::date_operator_3d((&mut a) as *mut [u8; 8] as usize, (&mut tmp2) as *mut [u8; 8] as usize, (r) as *mut [u8; 8] as usize);
    let target = match exe::date_get_weekday((&mut a) as *mut [u8; 8] as usize) as i16 {
        0 | 5 => 6,
        1 | 3 => 2,
        2 => 1,
        4 | 6 => 5,
        _ => severity,
    };
    covered!("cqb_13 target day");
    exe::date_get_nearest_weekday((&mut a) as *mut [u8; 8] as usize, target, (&mut b) as *mut [u8; 8] as usize); // cqb13: the target weekday
    let n = novo.as_mut_ptr() as usize;
    let v = Fixture(velho.as_mut_ptr() as usize);
    let mut pn = read_i16(read_usize(n + fixture::COMPETITION) + competition::REPUTATION);
    let mut pv = v.competition().reputation();
    if exe::database_is_national_team(read_usize(n + fixture::HOME_CLUB)) != 0 {
        pn = pn.wrapping_add(0x15);
    }
    if exe::database_is_national_team(v.home_club()) != 0 {
        pv = pv.wrapping_add(0x15);
    }
    let new_wins = pn > pv // cqb13: the higher priority stays
        || (pn == pv && read_i16(read_usize(n + fixture::HOME_CLUB) + club::REPUTATION) > v.home_club().reputation());
    if new_wins {
        covered!("cqb_13 new one wins");
    }
    let sai_old = new_wins && (v.home_goals() as u8) == 0xff; // cqb13: the one in the list not yet played
    if SAME_DAY_CLASH_DRY_RUN.load(Relaxed) {
        return 1;
    }
    // the calendar ones: ecx = this; push the match (0x5abc30, 0x5ab510; ret 4), push 0; push the match (0x5aa680; ret 8),
    // push 0; push &gravidade; push the match (0x5ad780; ret 0xc), push 0; push causa; push the match (0x5aac70; ret 0xc);
    // 0x68d040: ecx = the list; 0x944c94: push list; add esp, 4 -> cdecl
    let (sai, causa) = if sai_old {
        covered!("cqb_13 match leaves the list");
        exe::fixture_manager_remove_fixture(this, other);
        exe::fixture_manager_add_fixture(this, game, 0);
        (v.addr(), game)
    } else {
        (n, other)
    };
    write_u16(sai + fixture::DAY, u16::from_le_bytes([b[0], b[1]]));
    write_u16(sai + fixture::YEAR, u16::from_le_bytes([b[2], b[3]]));
    let list = exe::fixture_manager_find_fixture_clashes(this, sai, (&mut severity) as *mut u32 as *mut i32, 0);
    if list != 0 {
        exe::drop_llist(list);
        exe::crt_operator_delete(list);
        if read_u8(sai + fixture::CATEGORY) == 1 {
            return 0;
        }
        exe::fixture_manager_find_new_fixture_date(this, sai);
    }
    exe::fixture_manager_add_fixture(this, sai, 0);
    exe::fixture_manager_queue_fixture_moved_news(this, sai, causa, 0);
    1
}

/// Returns the club's first year with fixtures: 2001, or 2002 by bit 0 of its word in `G_CLUB_FIXTURE_YEAR_TABLE`.
///
/// `FIXTURE_MANAGER::get_club_first_fixture_year(&id)`; this is not read. The first year with fixtures of club `[id]`:
/// 2001, or 2002 with bit 0 of its word in `G_CLUB_FIXTURE_YEAR_TABLE`. Returns `ax` (the screen caller compares 16
/// bits).
/// Original: `FIXTURE_MANAGER::get_club_first_fixture_year` (0x5aee40, fix_man.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn get_club_first_fixture_year(_this: usize, id: usize) -> u16 {
    let b = *((read_usize(G_CLUB_FIXTURE_YEAR_TABLE) as isize).wrapping_add((*(id as *const i32) as isize).wrapping_mul(2)) as *const u8);
    0x7d1 + (b & 1) as u16
}

/// Returns the club's last year with fixtures: its word in `G_CLUB_FIXTURE_YEAR_TABLE` without bit 0, or the current
/// year.
///
/// `FIXTURE_MANAGER::get_club_last_fixture_year(&id)`; this is not read. The club's last year with fixtures: its word
/// in `G_CLUB_FIXTURE_YEAR_TABLE` without bit 0 (halved); with no year, the current year `G_CURRENT_DATE_YEAR`. Returns
/// `ax` (the screen caller keeps 16 bits).
/// Original: `FIXTURE_MANAGER::get_club_last_fixture_year` (0x5aee60, fix_man.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn get_club_last_fixture_year(_this: usize, id: usize) -> u16 {
    let w = core::ptr::read_unaligned((read_usize(G_CLUB_FIXTURE_YEAR_TABLE) as isize).wrapping_add((*(id as *const i32) as isize).wrapping_mul(2)) as *const u16);
    if w & 0xfffe != 0 { w >> 1 } else { *(G_CURRENT_DATE_YEAR as *const u16) }
}

compare_value!(compare_get_club_first_fixture_year, get_club_first_fixture_year, IDX_GET_CLUB_FIRST_FIXTURE_YEAR, (id: usize) -> u16);
compare_value!(compare_get_club_last_fixture_year, get_club_last_fixture_year, IDX_GET_CLUB_LAST_FIXTURE_YEAR, (id: usize) -> u16);

/// LLIST constructor (`exe::new_llist`).
pub(crate) unsafe fn list_init(l: usize) {
    exe::new_llist(l);
}

/// LLIST destructor (`exe::drop_llist`).
pub(crate) unsafe fn list_destroy(l: usize) {
    exe::drop_llist(l);
}

/// Empties a LLIST (`manager_manager_clear`).
pub(crate) unsafe fn list_clear(l: usize) {
    exe::llist_clear(l);
}

/// Sets the LLIST cursor (+0xa) to the head (+2).
pub(crate) unsafe fn list_rewind(l: usize) {
    write_u32(l + llist::CURSOR, read_u32(l + 2));
}

/// True if the competition is a friendly (`exe::friendly_is_friendly_competition` on G_FRIENDLY).
pub(crate) unsafe fn fix_man_is_friendly_comp(comp: u32) -> bool {
    exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), (comp) as usize) != 0
}

/// Builds a date from (day, year) at d (0x549ef0) and returns it.
pub(crate) unsafe fn date_from_day_year_ptr(d: *mut u8, day: u32, year: u32) -> usize {
    exe::new_date(d as usize, day, year)
}

/// Date minus n days into `saida` (0x54c8e0).
pub(crate) unsafe fn date_sub_days(d: usize, out: *mut u8, n: u32) -> usize {
    exe::date_sub_days(d, out as usize, n)
}

/// Date plus n days into `saida` (`exe::date_add_days`).
pub(crate) unsafe fn fix_man_date_add_days(d: usize, out: *mut u8, n: u32) -> usize {
    exe::date_add_days(d, out as usize, n)
}

/// Assigns date `origem` to `destino` (`exe::date_operator_3d`).
pub(crate) unsafe fn date_assign_ptr(target: *mut u8, source: usize) {
    let mut temporary = [0u8; 8];
    exe::date_operator_3d(target as usize, temporary.as_mut_ptr() as usize, source);
}

/// Finds the club's fixtures between two dates into a list (FIXTURE_MANAGER find_fixtures `find_fixtures`, any comp/type, side
/// 1).
pub(crate) unsafe fn find_club_fixtures_between(this: usize, a: *const u8, b: *const u8, club: u32, list: usize) -> u32 {
    let (a, b) = (a as usize, b as usize);
    exe::fixture_manager_find_fixtures(this, read_u32(a), read_u32(a + 2), read_u32(b), read_u32(b + 2), club, list, u32::MAX, 0xffff_fffe, 1) as u32 
}

/// Removes a fixture from the calendar (`remove_fixture`).
pub(crate) unsafe fn fix_man_remove_fixture(this: usize, game: usize) -> u32 {
    exe::fixture_manager_remove_fixture(this, game)
}

/// Adds a fixture to the calendar (`add_fixture`).
pub(crate) unsafe fn fix_man_add_fixture(this: usize, game: usize, clear: u32) {
    exe::fixture_manager_add_fixture(this, game, clear);
}

/// Queues the "fixture moved" news item (`queue_fixture_moved_news`) for the fixture, cause and old date.
pub(crate) unsafe fn fix_man_queue_fixture_moved_news(this: usize, game: usize, causa: Fixture, old: usize) {
    exe::fixture_manager_queue_fixture_moved_news(this, game, causa, (old) as u32);
}

/// True if the fixture's new date clashes with a fixture other than the original f1 (via `find_fixture_clashes`); frees the clash
/// list.
pub(crate) unsafe fn new_date_has_clash(this: usize, f2: *mut u8, f1: *const u8) -> bool {
    let mut severity = 0i32;
    let l = exe::fixture_manager_find_fixture_clashes(this, f2 as usize, &mut severity, 0);
    if l == 0 {
        return false;
    }
    list_rewind(l);
    let mut other = false;
    loop {
        let q = contract_manager_list_next(l);
        if q == 0 {
            break;
        }
        if core::slice::from_raw_parts(f1, 0x4f) != core::slice::from_raw_parts(q as *const u8, 0x4f) {
            other = true;
            break;
        }
    }
    list_destroy(l);
    exe::crt_operator_delete(l);
    other
}

/// Copies the first fixture of the list into f1 and f2 (0x4f bytes).
pub(crate) unsafe fn copy_first_fixture(l: usize, f1: &mut [u8; 0x4f], f2: &mut [u8; 0x4f]) {
    list_rewind(l);
    let p = contract_manager_list_next(l);
    core::ptr::copy_nonoverlapping(p as *const u8, f1.as_mut_ptr(), 0x4f);
    *f2 = *f1;
}

/// For a non-friendly fixture, moves the clubs' fixtures two days before one day earlier and those two days after one
/// day later, if the new date is free, with the news.
///
/// `FIXTURE_MANAGER::move_nearby_club_fixtures(game)`, from adding the fixture (`add_fixture`); SEH for the local list
/// and the dates. A fixture of a friendly competition (`.competition()`, `fix_man_is_friendly_comp`): nothing. For both
/// clubs (+0x1c, +0x20) of a nation (`.nation()`) with the flag `.league_selected()`: the club's first fixture two days
/// before (`find_club_fixtures_between`, side 1), not a friendly and with +0x43 = 0xff, moves to one day before it; the
/// first fixture two days after, not a friendly, moves to one day after it. Only if the new date has no other fixture
/// (`new_date_has_clash`): it removes (`fix_man_remove_fixture`), adds (`fix_man_add_fixture`, 0) and posts the
/// rescheduled fixture news (`fix_man_queue_fixture_moved_news`, with the fixture as the cause).
/// Original: `FIXTURE_MANAGER::move_nearby_club_fixtures` (0x5ae9d0, fix_man.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn move_nearby_club_fixtures(this: usize, game: usize) {
    let game = Fixture(game);
    let mut d = [0u8; 8];
    exe::date_date_54a110(d.as_mut_ptr() as usize);
    let mut l = [0u8; 0xe];
    let lp = l.as_mut_ptr() as usize;
    list_init(lp);
    if fix_man_is_friendly_comp(game.competition().addr() as u32) {
        list_destroy(lp);
        return;
    }
    let mut f1 = [0u8; 0x4f];
    let mut f2 = [0u8; 0x4f];
    for k in 0..2 {
        let club = Club(read_usize(game.addr() + if k == 0 { 0x1c } else { 0x20 }));
        let nation = club.nation();
        if nation.is_null() || nation.league_selected() == 0 {
            continue;
        }
        // dois dias antes
        let mut t = [0u8; 8];
        let mut u = [0u8; 8];
        let r = date_from_day_year_ptr(t.as_mut_ptr(), game.day() as u32, game.year() as u32);
        let r = date_sub_days(r, u.as_mut_ptr(), 2);
        date_assign_ptr(d.as_mut_ptr(), r);
        list_clear(lp);
        if find_club_fixtures_between(this, d.as_ptr(), d.as_ptr(), read_u32(club.addr()), lp) as u16 != 0 {
            copy_first_fixture(lp, &mut f1, &mut f2);
            if !fix_man_is_friendly_comp(read_u32(f1.as_ptr() as usize + fixture::COMPETITION)) {
                covered!("cpz_8 match two days before");
                if f2[0x43] == 0xff { // cpz8: the match from two days before without the mark
                    let mut v = [0u8; 8];
                    let r = date_sub_days(d.as_ptr() as usize, v.as_mut_ptr(), 1); // cpz8: um dia antes
                    date_assign_ptr(d.as_mut_ptr(), r);
                    f2[0x2a..0x2c].copy_from_slice(&d[0..2]);
                    f2[0x28..0x2a].copy_from_slice(&d[2..4]);
                    if !new_date_has_clash(this, f2.as_mut_ptr(), f1.as_ptr()) {
                        covered!("cpz_8 match brought forward");
                        fix_man_remove_fixture(this, f1.as_ptr() as usize);
                        fix_man_add_fixture(this, f2.as_mut_ptr() as usize, 0);
                        fix_man_queue_fixture_moved_news(this, f2.as_mut_ptr() as usize, game, 0);
                    }
                }
            }
        }
        // dois dias depois
        let mut t = [0u8; 8];
        let mut u = [0u8; 8];
        let r = date_from_day_year_ptr(t.as_mut_ptr(), game.day() as u32, game.year() as u32);
        let r = fix_man_date_add_days(r, u.as_mut_ptr(), 2);
        date_assign_ptr(d.as_mut_ptr(), r);
        list_clear(lp);
        if find_club_fixtures_between(this, d.as_ptr(), d.as_ptr(), read_u32(club.addr()), lp) as u16 == 0 {
            continue;
        }
        copy_first_fixture(lp, &mut f1, &mut f2);
        if fix_man_is_friendly_comp(read_u32(f1.as_ptr() as usize + fixture::COMPETITION)) {
            continue;
        }
        let mut v = [0u8; 8];
        let r = fix_man_date_add_days(d.as_ptr() as usize, v.as_mut_ptr(), 1); // cpz8: um dia depois
        date_assign_ptr(d.as_mut_ptr(), r);
        f2[0x2a..0x2c].copy_from_slice(&d[0..2]);
        f2[0x28..0x2a].copy_from_slice(&d[2..4]);
        if !new_date_has_clash(this, f2.as_mut_ptr(), f1.as_ptr()) {
            covered!("cpz_8 match postponed");
            fix_man_remove_fixture(this, f1.as_ptr() as usize);
            fix_man_add_fixture(this, f2.as_mut_ptr() as usize, 0);
            fix_man_queue_fixture_moved_news(this, f2.as_mut_ptr() as usize, game, 0);
        }
    }
    list_destroy(lp);
}

// simplification: moves matches in the calendar (removes, merges, allocates and frees lists, news): original only; the copy carries the branches.

/// Reschedules the club's fixtures from one day before to one day after a national team date: each removed fixture gets
/// a new date, is added back and posts the news with the old date.
///
/// `FIXTURE_MANAGER::reschedule_club_fixtures_near_date(club, data)`, from the national team call-ups (0x777940: a club
/// with more than two players called up); SEH for the local list. The club's fixtures (the id in `*club`) from one day
/// before to one day after the date (`find_club_fixtures_between`, side 1); each one that leaves the calendar
/// (`fix_man_remove_fixture`) gets a new date (`exe::fixture_manager_find_new_fixture_date`), goes back
/// (`fix_man_add_fixture`, 1) and posts the news with the old date (`fix_man_queue_fixture_moved_news`, no cause).
/// Original: `FIXTURE_MANAGER::reschedule_club_fixtures_near_date` (0x5abb10, fix_man.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn reschedule_club_fixtures_near_date(this: usize, club: usize, data: usize) {
    let mut l = [0u8; 0xe];
    let lp = l.as_mut_ptr() as usize;
    list_init(lp);
    let mut a = [0u8; 8];
    let mut b = [0u8; 8];
    date_sub_days(data, a.as_mut_ptr(), 1);
    fix_man_date_add_days(data, b.as_mut_ptr(), 1);
    find_club_fixtures_between(this, a.as_ptr(), b.as_ptr(), read_u32(club), lp);
    list_rewind(lp);
    loop {
        let p = contract_manager_list_next(lp);
        if p == 0 {
            break;
        }
        let mut old = [0u8; 8];
        date_from_day_year_ptr(old.as_mut_ptr(), read_u16(p + fixture::DAY) as u32, read_u16(p + fixture::YEAR) as u32);
        if fix_man_remove_fixture(this, p) != 0 {
            covered!("cpz_8 match rescheduled by national team");
            exe::fixture_manager_find_new_fixture_date(this, p);
            fix_man_add_fixture(this, p, 1);
            fix_man_queue_fixture_moved_news(this, p, Fixture(0), old.as_ptr() as usize);
        }
    }
    list_destroy(lp);
}

// simplification: moves matches in the calendar and gives the news (allocates): original only; the copy takes the branch.

/// The rehearsal of the news item. The copy builds the new news item here instead of putting it in the queue (and does
/// not create the queue).
pub(crate) static FIXTURE_MOVED_NEWS_DRY_RUN: AtomicBool = AtomicBool::new(false);
pub(crate) static mut FIXTURE_MOVED_NEWS_BUILT: [u8; 0xe0] = [0; 0xe0];
pub(crate) static FIXTURE_MOVED_NEWS_WAS_BUILT: AtomicBool = AtomicBool::new(false);

/// Data slot k of a news item (NEWSITEM get_slot).
pub(crate) unsafe fn news_slot(n: usize, k: u32) -> u32 {
    exe::newsitem_get_slot(n, k)
}

/// News item already in the list for the same fixture (slots 0,1,2,0xe,0xf vs fixture +0xc,+0x10,+0,+0x40,+0x41), or 0.
pub(crate) unsafe fn find_queued_fixture_news(l: usize, f: usize) -> usize {
    list_rewind(l);
    loop {
        let it = contract_manager_list_next(l);
        if it == 0 {
            return 0;
        }
        if news_slot(it, 0) == read_u32(f + fixture::HOME_CLUB_ID)
            && news_slot(it, 1) == read_u32(f + fixture::AWAY_CLUB_ID)
            && news_slot(it, 2) == read_u32(f)
            && news_slot(it, 0xe) as i32 == read_i8_as_i32(f + fixture::HOME_SIDE_MARK)
            && news_slot(it, 0xf) as i32 == read_i8_as_i32(f + fixture::AWAY_SIDE_MARK)
        {
            return it;
        }
    }
}

/// Queues the "fixture moved" news: updates the new date of an existing news item for the same fixture, or queues news
/// 1 with the fixture and the cause or the old date.
///
/// `FIXTURE_MANAGER::queue_fixture_moved_news(f, causa, old)`, from the reschedulings (`add_fixture`,
/// `reschedule_club_fixtures_near_date`, `resolve_same_day_clash`, `move_nearby_club_fixtures`); SEH for the local
/// news. The queue at `fixture_manager::MOVED_NEWS_QUEUE` (created the first time: `exe::crt_operator_new` and
/// `exe::new_llist`): if there is already news for the same fixture, it only changes the new date (data 5 and 6);
/// otherwise news 1: the fixture (`fixture::HOME_CLUB_ID`, `fixture::AWAY_CLUB_ID`, +0, `fixture::ROUND_PREFIX`,
/// `fixture::ROUND_NAME`, the new date `.day()`/`.year()`, `.home_side_mark()`, `.away_side_mark()`) and the cause (the
/// same fields of it in data 7 to 0xd) or, with no cause, −1 and the old date (data 0xc and 0xd); it goes into the
/// queue (`exe::llist_add_tail`, 0xdf).
/// Original: `FIXTURE_MANAGER::queue_fixture_moved_news` (0x5aac70, fix_man.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn queue_fixture_moved_news(this: usize, f: usize, causa: usize, old: usize) {
    let f = Fixture(f);
    if read_usize(this + fixture_manager::MOVED_NEWS_QUEUE) == 0 && !FIXTURE_MOVED_NEWS_DRY_RUN.load(Relaxed) {
        let m = exe::crt_operator_new(0xe);
        if m != 0 {
            list_init(m);
        }
        write_usize(this + fixture_manager::MOVED_NEWS_QUEUE, m);
    }
    let l = read_usize(this + fixture_manager::MOVED_NEWS_QUEUE);
    if l != 0 {
        let it = find_queued_fixture_news(l, f.addr());
        if it != 0 {
            exe::newsitem_set_slot(it, 5, f.day() as u32);
            exe::newsitem_set_slot(it, 6, f.year() as u32);
            return;
        }
    }
    covered!("cpz_8 rescheduled match news");
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    exe::new_newsitem(nt, 1, 0);
    let fields = |nt: usize, k: i8, j: usize| {
        exe::newsitem_set_slot(nt, k, read_u32(j + fixture::HOME_CLUB_ID));
        exe::newsitem_set_slot(nt, k + 1, read_u32(j + fixture::AWAY_CLUB_ID));
        exe::newsitem_set_slot(nt, k + 2, read_u32(j));
        exe::newsitem_set_slot(nt, k + 3, read_i16_as_i32(j + fixture::ROUND_PREFIX) as u32);
        exe::newsitem_set_slot(nt, k + 4, read_i16_as_i32(j + fixture::ROUND_NAME) as u32);
        exe::newsitem_set_slot(nt, k + 5, read_u16(j + fixture::DAY) as u32);
    };
    fields(nt, 0, f.addr());
    exe::newsitem_set_slot(nt, 6, f.year() as u32);
    exe::newsitem_set_slot(nt, 0xe, ((f.home_side_mark() as i8) as i32) as u32);
    exe::newsitem_set_slot(nt, 0xf, (f.away_side_mark() as i32) as u32);
    if causa != 0 {
        fields(nt, 7, causa);
        exe::newsitem_set_slot(nt, 0xd, read_u16(causa + fixture::YEAR) as u32);
    } else {
        exe::newsitem_set_slot(nt, 7, u32::MAX);
        exe::newsitem_set_slot(nt, 0xc, read_i16_as_i32(old) as u32);
        exe::newsitem_set_slot(nt, 0xd, read_u16(old + 2) as u32);
    }
    if FIXTURE_MOVED_NEWS_DRY_RUN.load(Relaxed) {
        core::ptr::copy_nonoverlapping(nt as *const u8, core::ptr::addr_of_mut!(FIXTURE_MOVED_NEWS_BUILT) as *mut u8, 0xe0);
        FIXTURE_MOVED_NEWS_WAS_BUILT.store(true, Relaxed);
        return;
    }
    exe::llist_add_tail(read_usize(this + fixture_manager::MOVED_NEWS_QUEUE), nt, 0xdf);
}

/// The calendar block for the year of match `f` (+0x28, word), walking the calendar chain from `fm`.
///
/// This is the loop shared by `find_stored_fixture`, `insert_fixture` and `remove_fixture`: the next block (+0x112c) of another year is moved to the
/// requested year (`set_year` set_year); with no next block, a new block (`exe::crt_operator_new`, 0x1184 bytes; `exe::new_fixture_manager` with the year,
/// the previous block and 0) joins the chain.
///
/// Returns 0 if memory ran out (the chain keeps 0 at +0x112c, as in the original routine).
pub(crate) unsafe fn calendar_year_block(fm: usize, f: Fixture) -> usize {
    let mut b = fm;
    loop {
        let year = f.year();
        if year == read_u16(b + fixture_manager::YEAR) {
            return b;
        }
        let next = read_usize(b + fixture_manager::OTHER_YEAR);
        if next != 0 {
            if read_u16(next + fixture_manager::YEAR) != year {
                // simplification: the original pushes eax with the year in the low word and register leftovers on top; the
                // 0x5aa2d0 compares the word (the path only runs at the calendar year turn)
                exe::fixture_manager_set_year(next, year as u32);
            }
        } else {
            let p = exe::crt_operator_new((0x1184) as u32);
            // the original’s ecx was the null +0x112c: the year is passed clean
            let r = if p != 0 { exe::new_fixture_manager(p, year as u32, b, 0) } else { 0 };
            write_u32(b + fixture_manager::OTHER_YEAR, r as u32);
            if r == 0 {
                return 0;
            }
        }
        b = read_usize(b + fixture_manager::OTHER_YEAR);
    }
}

/// Fixture list in block b for the fixture's day (+0x2a), time slice (+0x3f) and key nation.
pub(crate) unsafe fn fixture_list_in_block(b: usize, f: Fixture) -> usize {
    let n = (exe::comp_active_key_nation_of_fixture(f.competition(), f) as i8) as isize;
    let k = f.day() as usize * 3 + f.time_slice() as usize;
    read_usize((read_usize(b + k * 4) as isize + n * 4) as usize)
}

/// Returns the stored copy in the calendar whose 0x4f bytes match the fixture's, or 0 if none.
///
/// `FIXTURE_MANAGER::find_stored_fixture(fixture)`: the copy stored in the calendar whose 0x4f bytes equal the
/// fixture's, in the list of its day, slice and key nation (the year block through the chain; the list cursor goes back
/// to the start and walks). Null fixture or none equal: 0; out of memory for the year block: the box (line 0x7cf) and
/// 0.
/// Original: `FIXTURE_MANAGER::find_stored_fixture` (0x5ac7a0, fix_man.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn find_stored_fixture(this: usize, f: usize) -> usize {
    if f == 0 {
        return 0;
    }
    let b = calendar_year_block(this, Fixture(f));
    if b == 0 {
        show_error_at(0x9cc208, 0x7cf);
        return 0;
    }
    let l = fixture_list_in_block(b, Fixture(f));
    write_u32(l + llist::CURSOR, read_usize(l + 2) as u32);
    let target = core::slice::from_raw_parts(f as *const u8, 0x4f);
    let mut item = list_next_item(l);
    while item != 0 {
        if core::slice::from_raw_parts(item as *const u8, 0x4f) == target {
            covered!("cps_11 found the stored match");
            return item; // cps11: the saved copy
        }
        item = list_next_item(l);
    }
    0
}

/// Finds (or creates) the fixture's year block and appends a copy of the fixture to its day, slice and key-nation list.
///
/// `FIXTURE_MANAGER::insert_fixture(f)`: appends a copy of the fixture's 0x4f bytes (`exe::llist_add_tail`) to the list
/// of its day, slice and key nation in the year block (through the chain). Out of memory for the block: the box (line
/// 0x3ff); a failed copy: the box (line 0x3eb).
/// Original: `FIXTURE_MANAGER::insert_fixture` (0x5ab230, fix_man.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn insert_fixture(this: usize, f: usize) {
    let b = calendar_year_block(this, Fixture(f));
    if b == 0 {
        return show_error_at(0x9cc208, 0x3ff);
    }
    let l = fixture_list_in_block(b, Fixture(f));
    if exe::llist_add_tail(l, f, 0x4f) == 0 { // cps11: guardar
        return show_error_at(0x9cc208, 0x3eb);
    }
    covered!("cps_11 stored match");
}

/// Removes the first item with the same bytes from the fixture's day, slice and key-nation list; returns 1 if removed.
///
/// `FIXTURE_MANAGER::remove_fixture(f)`: removes from the list of the fixture's day, slice and key nation (in the year
/// block, through the chain) the first item with the same bytes (`exe::llist_remove`). Returns 1 if removed, 0 if not;
/// out of memory for the block: the box (line 0x5ef) and 0.
/// Original: `FIXTURE_MANAGER::remove_fixture` (0x5abc30, fix_man.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn remove_fixture(this: usize, f: usize) -> u32 {
    let b = calendar_year_block(this, Fixture(f));
    if b == 0 {
        show_error_at(0x9cc208, 0x5ef);
        return 0;
    }
    let r = exe::llist_remove(fixture_list_in_block(b, Fixture(f)), f); // cps11: tirar
    if r != 0 {
        covered!("cps_11 match removed");
    }
    (r != 0) as u32
}

/// Fixture list of the fixture in the calendar (year block, then list); 0 on out of memory.
pub(crate) unsafe fn find_fixture_list(fm: usize, f: usize) -> usize {
    let b = calendar_year_block(fm, Fixture(f));
    if b == 0 { 0 } else { fixture_list_in_block(b, Fixture(f)) }
}

/// Fixture list for (year, day, slice, key nation) in calendar fm (`get_fixture_list`, creates the year block if missing).
pub(crate) unsafe fn fix_man_get_fixture_list(fm: usize, year: u32, day: u32, slice: u32, nation: u32) -> usize {
    exe::fixture_manager_get_fixture_list(fm, year, day, slice, nation)
}

/// Rewinds the LLIST cursor and returns the first item.
pub(crate) unsafe fn list_first(l: usize) -> usize {
    write_u32(l + llist::CURSOR, read_u32(l + 2));
    list_next_item(l)
}

/// Appends a copy of the 0x4f-byte fixture to the list (0x68d0d0); 0 on failure.
pub(crate) unsafe fn list_append_fixture(l: usize, f: usize) -> usize {
    exe::llist_add_tail(l, f, 0x4f)
}

/// Active key nation of competition `comp` (`comp_active_key_nation_of_fixture` on its club-comp record, no fixture).
pub(crate) unsafe fn comp_key_nation(comp: u32) -> u8 {
    (exe::comp_active_key_nation_of_fixture((read_u32(G_CLUB_COMP_TABLE).wrapping_add(comp.wrapping_mul(0x6b))) as usize, 0) as u32) as u8
}

/// Fixture passes the `find_fixtures` filters: type (+0x42), club home/away with its side (+0x40/+0x41) and competition (+0);
/// wildcards allowed.
pub(crate) unsafe fn fixture_matches_filter(f: Fixture, club: u32, comp: u32, kind: u8, side: u8) -> bool {
    if kind != 0xfe && kind != f.stage_index() {
        return false;
    }
    if club != u32::MAX {
        let home = f.home_club_id() as u32;
        if club != home && club != (f.away_club_id() as u32) {
            return false;
        }
        if side != 0xff {
            let o_dele = if club == home { f.home_side_mark() } else { f.away_side_mark() as u8  };
            if o_dele != side {
                return false;
            }
        }
    }
    comp == u32::MAX || comp == read_u32(f.addr())
}

/// Counts (and copies into the list, if given) the calendar fixtures between two dates in the key-nation lists of the
/// club or competition, with type, club side and competition filters.
///
/// `FIXTURE_MANAGER::find_fixtures(dia0, ano0, dia1, ano1, club, list, comp, kind, side)`: counts (and, with `list`,
/// copies into it) the calendar fixtures between (dia0, ano0) and (dia1, ano1) inclusive, day by day (366 per year), in
/// the three slices of each day and in the key-nation lists: the club's (index into [0xae23b4], 0x245 bytes; a national
/// team `exe::database_is_national_team`: nations 0 and 1; otherwise 2..7, the one from
/// `exe::comp_active_key_nation_of_club` and the partner `exe::fix_man_get_partner_key_nation_index`, without repeats)
/// or only the competition's (`comp_key_nation`). Filters: kind (0xfe any), club side (0xff any) and competition (−1
/// any). Returns the count (word); a failed copy shows the box (line 0x93b) and stops there. Hot (288 thousand calls up
/// to 2002-06-01): no counter.
/// Original: `FIXTURE_MANAGER::find_fixtures` (0x5acbe0, fix_man.cpp, thiscall, ret 0x24).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn find_fixtures(this: usize, dia0: u32, ano0: u32, dia1: u32, ano1: u32, club: u32,
                                                      list: usize, comp: u32, kind: u32, side: u32) -> u32 {
    let (kind, side) = (kind as u8, side as u8);
    let mut nations = [0xffu8; 8];
    let mut n = 0usize;
    if club != u32::MAX {
        let reg = club_at(club as isize).addr();
        if exe::database_is_national_team(reg) != 0 {
            nations[0] = 0;
            nations[1] = 1;
            n = 2;
        } else {
            nations[..6].copy_from_slice(&[2, 3, 4, 5, 6, 7]);
            nations[6] = (exe::comp_active_key_nation_of_club(reg) as u32) as u8;
            nations[7] = (exe::fix_man_get_partner_key_nation_index(reg) as u32) as u8;
            n = 8;
            for i in 0..7 {
                if (i + 1..8).any(|j| nations[j] == nations[i]) {
                    nations[i] = 0xff; // repetida adiante
                }
            }
        }
    } else if (comp as i32) > -1 {
        nations[0] = comp_key_nation(comp);
        n = 1;
    }
    let (mut dia, mut ano, mut total) = (dia0, ano0, 0u32);
    loop {
        if dia as u16 >= 0x16e {
            ano = ano.wrapping_add(1); // the next year, from day 0
            dia = 0;
            continue;
        }
        if ano as u16 > ano1 as u16 || (ano as u16 == ano1 as u16 && dia as u16 > dia1 as u16) {
            return total & 0xffff; // past the last day
        }
        for fatia in 0..3u32 {
            for &nacao in &nations[..n] {
                if nacao == 0xff {
                    continue;
                }
                let l = fix_man_get_fixture_list(this, ano, dia, fatia, nacao as u32);
                if l == 0 {
                    continue;
                }
                let mut f = list_first(l);
                while f != 0 {
                    if fixture_matches_filter(Fixture(f), club, comp, kind, side) {
                        if list != 0 && list_append_fixture(list, f) == 0 {
                            show_error_at(0x9cc208, 0x93b);
                            return total & 0xffff;
                        }
                        total = total.wrapping_add(1);
                    }
                    f = list_next_item(l);
                }
            }
        }
        dia = dia.wrapping_add(1);
    }
}

/// Copies into the list the fixtures of one day and slice for the requested competition, round and type, and returns
/// how many.
///
/// `FIXTURE_MANAGER::collect_comp_fixtures_on_day(year, day, slice, comp, round, kind, list)` (from the leagues' TV
/// postponement and from moving cup rounds to the fixed dates): copies into `list` the fixtures of the day and slice,
/// in the list of the competition's key nation (`comp_key_nation`; −1: nation 0), of the competition (+0; −1 any), of
/// the round (`fixture::ROUND_NUMBER`, word; −1 any) and of the kind (`fixture::STAGE_INDEX`; 0xfe any). Returns how
/// many (word); a failed copy shows the box (line 0x634) and stops there.
/// Original: `FIXTURE_MANAGER::collect_comp_fixtures_on_day` (0x5abd90, fix_man.cpp, thiscall, ret 0x1c).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn collect_comp_fixtures_on_day(this: usize, year: u32, day: u32, slice: u32, comp: u32, round: u32,
                                                       kind: u32, list: usize) -> u32 {
    let kind = kind as u8;
    // simplification: the original pushes the nation local with high bytes of stack leftovers; 0x5abf10 only reads the byte
    let nation = if comp != u32::MAX { comp_key_nation(comp) } else { 0 };
    let mut total = 0u32;
    let l = fix_man_get_fixture_list(this, year, day, slice, nation as u32);
    if l == 0 {
        return 0;
    }
    let mut f = list_first(l);
    while f != 0 {
        let mut ok = true;
        if comp != u32::MAX && comp != read_u32(f) {
            ok = false;
        }
        if round != u32::MAX && round as i32 != read_i16(f + fixture::ROUND_NUMBER) as i32 {
            ok = false; // another round
        }
        if (kind == 0xfe || read_u8(f + fixture::STAGE_INDEX) == kind) && ok {
            if list_append_fixture(list, f) == 0 {
                show_error_at(0x9cc208, 0x634);
                return total & 0xffff;
            }
            covered!("cpr_12 round match");
            total = total.wrapping_add(1);
        }
        f = list_next_item(l);
    }
    total & 0xffff
}

/// Tells whether the competition has a fixture on the date in any of the three calendar slices.
///
/// `fix_man_comp_has_fixture_on_date(data, comp)` (stdcall, ret 8; the `ecx` = [`G_FIXTURE_MANAGER`] set by the caller
/// is not used), from the European Championship qualifiers (0x57fd60): 1 if the competition has a fixture (+0 = comp)
/// on the date in any of the three slices of the calendar [`G_FIXTURE_MANAGER`], in its key nation's list
/// (`comp_key_nation`). Competition −1 or below: 0; null date: the box (line 0xe89) and 0.
/// Original: `fix_man_comp_has_fixture_on_date` (0x5ae6d0, fix_man.cpp, stdcall, ret 8).
pub(crate) unsafe extern "stdcall" fn fix_man_comp_has_fixture_on_date(data: usize, comp: u32) -> u8 {
    if (comp as i32) <= -1 {
        return 0;
    }
    let nation = comp_key_nation(comp);
    if data == 0 {
        show_error_at(0x9cc208, 0xe89);
        return 0;
    }
    let fm = read_u32(G_FIXTURE_MANAGER) as usize;
    // simplification: the original pushes the year and the day (words) with leftover high bytes in the registers; 0x5abf10
    // compares the year as a word and masks the day (the year dword only follows if the year block is missing, and the two years of the
    // career are in the chain since the game’s creation)
    let (ano, dia) = (read_u16(data + 2) as u32, read_u16(data) as u32);
    for fatia in 0..3u32 {
        let l = fix_man_get_fixture_list(fm, ano, dia, fatia, nation as u32);
        if l == 0 {
            continue;
        }
        let mut f = list_first(l);
        while f != 0 {
            if read_u32(f) == comp {
                return 1;
            }
            f = list_next_item(l);
        }
    }
    0
}

/// True if the items the original appended to `lista` (after the old tail) equal those of the temporary list `nova`, in
/// order, 0x4f bytes each.
pub(crate) unsafe fn appended_items_match(list: usize, count_before: u16, tail_before: u32, new: usize) -> bool {
    let acrescentados = read_u16(list).wrapping_sub(count_before);
    if acrescentados != read_u16(new) {
        return false;
    }
    let mut a = if count_before == 0 { read_u32(list + 2) } else { read_u32(tail_before as usize + 6) } as usize;
    let mut b = read_u32(new + 2) as usize;
    for _ in 0..acrescentados {
        if a == 0 || b == 0 {
            return false;
        }
        let (da, db) = (read_u32(a) as usize, read_u32(b) as usize);
        if core::slice::from_raw_parts(da as *const u8, 0x4f) != core::slice::from_raw_parts(db as *const u8, 0x4f) {
            return false;
        }
        a = read_u32(a + 6) as usize;
        b = read_u32(b + 6) as usize;
    }
    true
}

/// Runs the port with a temporary LLIST and checks its items against those the original appended to the real list;
/// returns (result, equal).
pub(crate) unsafe fn run_with_temp_list(list: usize, count_before: u16, tail_before: u32, copy: impl FnOnce(usize) -> u32) -> (u32, bool) {
    let mut new = [0u8; 0x10];
    let p = new.as_mut_ptr() as usize;
    exe::new_llist(p);
    let n = copy(p);
    let equal = appended_items_match(list, count_before, tail_before, p);
    exe::drop_llist(p);
    (n, equal)
}

/// Fixture list for (year, day, slice, group) via `get_fixture_list` with the cursor rewound; 0 if null.
pub(crate) unsafe fn get_fixture_list_rewound(this: usize, year: u32, day: u32, slice: u32, group: u32) -> usize {
    let l = exe::fixture_manager_get_fixture_list(this, year, day, slice, group);
    if l != 0 {
        write_u32(l + llist::CURSOR, read_u32(l + 2));
    }
    l
}

/// Next item of a LLIST (`next_list_item`) or 0.
pub(crate) unsafe fn list_next_item(l: usize) -> usize {
    exe::llist_get_next(l)
}

/// Fixture passes find_next_fixture's filter: competition (+0), stage (+0x34) and both clubs (home +0xc or away +0x10),
/// each -1 = any.
pub(crate) unsafe fn fixture_matches_next_filter(f: Fixture, comp: i32, stage: i32, clube_a: i32, club_b: i32) -> bool {
    let club = |c: i32| c == -1 || c == (f.home_club_id() as i32) || c == (f.away_club_id() as i32);
    (comp == -1 || comp == f.comp_id()) // the competition
        && (stage == -1 || stage == f.round_number() as i32)
        && club(clube_a)
        && club(club_b)
}

/// Finds the next (or previous) fixture from a year/day/slice cursor, inclusive, matching the competition, stage and
/// two club filters; writes the cursor back and returns the fixture, or 0.
///
/// `FIXTURE_MANAGER::find_next_fixture(pano, pdia, pfatia, adiante, comp, stage, club, club_b, ...)`: the fixture after
/// (or before, `adiante` 0) the cursor (year, day, slice), the cursor itself included, that passes the filter
/// (`fixture_matches_next_filter`: competition +0, stage +0x34 and the two clubs +0xc/+0x10, each -1 = any). The group
/// of lists comes from `exe::comp_active_key_nation_of_fixture` (the competition of table `G_CLUB_COMP_TABLE` + 107 ×
/// comp, 0). Forward: slices up to 2, days up to 365 (0x16e exclusive), following years from day 0 (the slice only goes
/// back to 0 when the day turns); backward: slices down to 0, then 2, days down to 0, previous years from day 0x16d.
/// Five years from the cursor's year (forward: signed less than unsigned; backward: signed), 0. Found: it writes the
/// cursor (day, year and the slice as a byte) and returns the fixture. The last 3 arguments are not read. The original
/// passes `get_fixture_list` the year, day and slice with the high half of locals that were never written (garbage),
/// but it and what it calls (`set_year`, `exe::new_fixture_manager`, `exe::date_is_leap_year`) only use the word/byte:
/// the port passes zero. Used in the cup draws (Cup.cpp 0x519d90, `exe::cup_process_fixture`,
/// `exe::cup_label_round_extra_fixtures`). No SEH, no box (`get_fixture_list`'s only when out of memory).
/// Original: `FIXTURE_MANAGER::find_next_fixture` (0x5ac920, fix_man.cpp, thiscall, ret 0x2c).
pub(crate) unsafe extern "thiscall" fn find_next_fixture(this: usize, pano: *mut u16, pdia: *mut u16, pfatia: *mut u8, adiante: u32,
    comp: i32, stage: i32, club: i32, club_b: i32, _a9: u32, _a10: u32, _a11: u32) -> usize {
    let table = read_usize(G_CLUB_COMP_TABLE);
    let group = (exe::comp_active_key_nation_of_fixture(table.wrapping_add((comp as usize).wrapping_mul(107)), 0) as u32) as u8 as u32;
    let (mut year, mut day, mut slice) = (*pano, *pdia, *pfatia as u16);
    let find = |year: u16, day: u16, slice: u16| -> usize {
        let l = get_fixture_list_rewound(this, year as u32, day as u32, slice as u32, group);
        if l == 0 {
            return 0;
        }
        loop {
            let f = list_next_item(l);
            if f == 0 || fixture_matches_next_filter(Fixture(f), comp, stage, club, club_b) {
                return f;
            }
        }
    };
    let f = 'busca: {
        if adiante != 0 {
            loop {
                while (day as i16) < 0x16e {
                    while (slice as i16) < 3 {
                        let f = find(year, day, slice);
                        if f != 0 {
                            break 'busca f;
                        }
                        slice = slice.wrapping_add(1);
                    }
                    day = day.wrapping_add(1);
                    slice = 0; // o dia seguinte
                }
                year = year.wrapping_add(1);
                day = 0;
                if (year as i16 as i32) - (*pano as i32) >= 5 {
                    return 0;
                }
            }
        } else {
            loop {
                while (day as i16) >= 0 {
                    while (slice as i16) >= 0 {
                        let f = find(year, day, slice);
                        if f != 0 {
                            break 'busca f;
                        }
                        slice = slice.wrapping_sub(1);
                    }
                    slice = 2; // the previous day
                    day = day.wrapping_sub(1);
                }
                year = year.wrapping_sub(1);
                day = 0x16d;
                if (*pano as i16 as i32) - (year as i16 as i32) >= 5 {
                    return 0;
                }
            }
        }
    };
    *pdia = day;
    *pano = year;
    *pfatia = slice as u8;
    f
}

/// Returns 1 when a fixture in the day lists uses the stadium, in the given slice or in slices 0 to 2 (slice -1).
///
/// `FIXTURE_MANAGER::is_stadium_in_use(stadium, day, year, slice)`: 1 if any fixture (`fixture::STADIUM` = the stadium,
/// not null) in the day's lists uses the stadium. The groups: 0 to 7 and the stadium's
/// (`exe::comp_active_key_nation_of_stadium`, byte), the latter skipped if it repeats one of the eight. Slice -1
/// (byte): slices 0 to 2, otherwise only that one. The slice goes to `get_fixture_list_rewound` in the low byte of the
/// year argument (the original keeps the loop there: the 3 high bytes are the year's). Null stadium: 0. Used when
/// scheduling friendlies (friendly.cpp `add_friendly_match`, 0x5c75a0) and in
/// `exe::reservecomp_schedule_reserve_fixtures`, `exe::reservecomp_add_b_or_under21_fixture`. No SEH, no locals without
/// a value.
/// Original: `FIXTURE_MANAGER::is_stadium_in_use` (0x5ae870, fix_man.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn is_stadium_in_use(this: usize, stadium: usize, day: u32, year: u32, slice: u32) -> u32 {
    if stadium == 0 {
        return 0;
    }
    let mut groups = [0u8, 1, 2, 3, 4, 5, 6, 7, (exe::comp_active_key_nation_of_stadium(stadium) as u32) as u8];
    for i in 0..8 {
        for j in i + 1..9 {
            if groups[i] as i8 >= 0 && groups[i] == groups[j] {
                groups[j] = 0xff;
            }
        }
    }
    let (ini, fim) = if slice as u8 == 0xff { (0i8, 2i8) } else { (slice as i8, slice as i8) };
    for &g in &groups {
        if g == 0xff {
            continue;
        }
        let mut s = ini;
        while s <= fim {
            let l = get_fixture_list_rewound(this, year, day, (year & 0xffff_ff00) | s as u8 as u32, g as u32);
            if l != 0 {
                loop {
                    let f = list_next_item(l);
                    if f == 0 {
                        break;
                    }
                    let e = read_usize(f + fixture::STADIUM);
                    if e != 0 && e == stadium { // the match stadium
                        return 1;
                    }
                }
            }
            s = s.wrapping_add(1);
        }
    }
    0
}

compare_value!(compare_is_stadium_in_use, is_stadium_in_use, IDX_IS_STADIUM_IN_USE, (estadio: usize, dia: u32, ano: u32, fatia: u32) -> u32);

/// Calls the original routine (ret 4) with eax = year and ecx = this, as its only caller does.
pub(crate) unsafe fn call_original_with_year(o: usize, this: usize, year: u32) -> u32 {
    let r: u32;
    core::arch::asm!(
        "push {a}",
        "call {f}",
        a = in(reg) year,
        f = in(reg) o,
        inout("eax") year => r,
        inout("ecx") this => _,
        clobber_abi("C"),
    );
    r
}

/// Returns the fixture's time slice (1 weekend, 2 otherwise) by the competition's nation and the weekday.
///
/// `this` is not read. The time slice of a fixture of competition `comp` on day `day` (word): the Netherlands, Ireland,
/// Belgium and France (the id of `.nation()` in `KEY_NATION_HOLLAND`, `KEY_NATION_REPUBLIC_OF_IRELAND`,
/// `KEY_NATION_BELGIUM`, `KEY_NATION_FRANCE`) only have 1 on day 6; the others, 1 on days 5 and 6; otherwise 2. Returns
/// only `al`.
/// Original: `FIXTURE_MANAGER::get_time_slice` (0x5af2f0, fix_man.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn get_time_slice(_this: usize, comp: usize, day: u32) -> u8 {
    let comp = Competition(comp);
    let day = day as i16;
    if !comp.is_null() {
        let nation = comp.nation();
        if !nation.is_null() {
            let id = read_u32(nation.addr());
            if [KEY_NATION_HOLLAND, KEY_NATION_REPUBLIC_OF_IRELAND, KEY_NATION_BELGIUM, KEY_NATION_FRANCE].iter().any(|&g| id == read_u32(g)) {
                return if day != 6 { 2 } else { 1 };
            }
        }
    }
    if day == 6 || day == 5 { 1 } else { 2 }
}

/// With the stadium in a third-world country, gives the visiting clubs the illness risk on the day before the fixture.
///
/// `FIXTURE_MANAGER::add_away_trip_illness_risk(f)`; `this` is not used. With the fixture's stadium (`.stadium()`), its
/// city (`stadium::CITY`) and the city's nation (`city::NATION`) in a third-world country (`.state_of_development()` =
/// 3), each visiting club from another nation (`.nation()`) — the home club (`.home_club()`, id `.home_club_id()`) and
/// the away club (`.away_club()`, id `.away_club_id()`) — gets the illness risk on the day before the fixture.
/// Original: `FIXTURE_MANAGER::add_away_trip_illness_risk` (0x5ab3c0, fix_man.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn add_away_trip_illness_risk(_this: usize, f: usize) {
    let f = Fixture(f);
    let stadium = Stadium(f.stadium());
    if stadium.is_null() {
        return;
    }
    let city = stadium.city();
    if city == 0 {
        return;
    }
    let nation = Nation(read_usize(city + city::NATION));
    if nation.is_null() {
        return;
    }
    if nation.state_of_development() != 3 { // cps13: só o terceiro mundo
        return;
    }
    let home = f.home_club();
    if !home.is_null() && home.nation() != nation {
        covered!("cps_13 home side illness");
        // the original pushes the year in the club’s eax and the day in the nation’s ecx (high halves of the pointers)
        add_illness_risk_before_match(f, f.home_club_id(), read_usize(read_usize(f.stadium() + stadium::CITY) + city::NATION), nation.addr() as u32, home.addr() as u32);
    }
    let away = f.away_club();
    if !away.is_null() && away.nation().addr() != read_usize(read_usize(f.stadium() + stadium::CITY) + city::NATION) {
        covered!("cps_13 away side illness");
        // the year in the stadium’s ecx and the day in the city’s edx
        let (e, c) = (f.stadium(), read_usize(f.stadium() + stadium::CITY));
        add_illness_risk_before_match(f, f.away_club_id(), read_usize(c + city::NATION), c as u32, e as u32);
    }
}

/// For an English or Welsh club, returns the active key-nation index of the other nation; otherwise 0xff.
///
/// `fix_man_get_partner_key_nation_index(club)` (cdecl), from the fixture searches `find_fixtures`, 0x5acee0 and
/// `find_fixture_clashes`: for a club from England (`KEY_NATION_ENGLAND`) or Wales (`KEY_NATION_WALES`) (`.nation()`,
/// id +0), the index of the other nation's active key nation (the 35 records of 72 bytes at `G_LEAGUE_TABLE`: +0 the
/// nation, active at `.league_selected()`, +0x14 the index); otherwise 0xff.
/// Original: `fix_man_get_partner_key_nation_index` (0x5af5b0, fix_man.cpp, cdecl).
pub(crate) unsafe extern "C" fn fix_man_get_partner_key_nation_index(club: usize) -> u8 {
    let club = Club(club);
    let nation = (club.nation().addr() as u32) as usize;
    if nation == 0 {
        return 0xff;
    }
    let (id, inglaterra, gales) = (read_u32(nation), read_u32(KEY_NATION_ENGLAND), read_u32(KEY_NATION_WALES));
    let other = if id == inglaterra {
        gales
    } else if id == gales {
        inglaterra
    } else {
        return 0xff;
    };
    let target = read_u32(G_NATION_TABLE).wrapping_add(other.wrapping_mul(0x122)) as usize;
    for i in 0..0x23usize {
        let p = Nation(read_u32(G_LEAGUE_TABLE + i * 72) as usize);
        if !p.is_null() && p.league_selected() != 0 && p.addr() == target {
            return read_u8(0xb63d74 + i * 72);
        }
    }
    0xff
}

/// Returns the fixture list (`exe::fixture_manager_get_fixture_list` of the manager [`G_FIXTURE_MANAGER`]) for the year
/// (+2) and day (+0) of the date, in the given slice and group.
///
/// `get_fixture_list_for_date(data, slice, group)` (stdcall, ret 0xc): the fixture list from
/// `exe::fixture_manager_get_fixture_list` of the manager [`G_FIXTURE_MANAGER`] for the year (+2) and day (+0) of the
/// date. The year argument carries in its high half that of the date pointer (`mov ax, [eax+2]`), which is reproduced;
/// the day argument carries the caller's ecx high half, which `get_fixture_list` discards (`and 0xffff`): the port
/// passes zero. Used on match days (set_up_the_day `set_up_the_day`, Rust, each list of the day), in the friendlies
/// (0x5cbf3e), in the human manager (0x5fe83a), in the media (0x7776a8) and in 0x7e6498, 0x7e6ac9.
/// Original: `FIXTURE_MANAGER::get_fixture_list_for_date` (0x5ae6a0, fix_man.cpp, stdcall, ret 0xc).
pub(crate) unsafe extern "stdcall" fn get_fixture_list_for_date(data: usize, slice: u32, group: u32) -> usize {
    let year = (data as u32 & 0xffff_0000) | read_u16(data + 2) as u32;
    let day = read_u16(data) as u32;
    exe::fixture_manager_get_fixture_list(read_usize(G_FIXTURE_MANAGER), year, day, slice, group) // the match manager
}

/// Marks the fixture's year in both clubs' entries of the club dates table (year × 2 + bit, 2001 rule).
///
/// (cdecl) Marks the year (`.year()`, word) of fixture `game` in the entries of the two clubs (+0xc and +0x10) of the
/// table `G_CLUB_FIXTURE_YEAR_TABLE` (words: year × 2 + bit 0). Empty entry: bit 0 is set, or the entry is zeroed in
/// 2001, and the year goes in; otherwise in 2001 bit 0 is cleared and the year goes up if larger. Null table: as in the
/// original (error box).
/// Original: `mark_calendar_year` (0x5af180, fix_man.cpp, cdecl).
pub(crate) unsafe extern "C" fn mark_calendar_year(game: usize) {
    let game = Fixture(game);
    let table = read_usize(G_CLUB_FIXTURE_YEAR_TABLE);
    if table == 0 {
        return original_fn::<unsafe extern "C" fn(usize)>(IDX_MARK_CALENDAR_YEAR)(game.addr());
    }
    let year = game.year();
    for o in [0xc, 0x10] {
        let e = table.wrapping_add((read_i32(game.addr() + o) as usize).wrapping_mul(2)) as *mut u16;
        let v = core::ptr::read_unaligned(e);
        if v & 0xfffe == 0 {
            let w = if year == 0x7d1 { 0 } else { v | 1 };
            core::ptr::write_unaligned(e, w & 1 | year << 1);
        } else {
            if v & 1 != 0 && year == 0x7d1 {
                core::ptr::write_unaligned(e, v & 0xfffe);
            }
            let v = core::ptr::read_unaligned(e);
            if year > v >> 1 {
                core::ptr::write_unaligned(e, year.wrapping_mul(2) | v & 1);
            }
        }
    }
}

