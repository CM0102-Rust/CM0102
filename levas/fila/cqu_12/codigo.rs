// ---------- %%LEVA%%a leva: the Oceania World Cup qualifying: the start day, the first group table and the second group stage (wc_oceania_league.cpp) ----------

/// The qualifying's start date (`exe::new_date` on a local date: push year; push day; lea ecx -> thiscall, ret 8): the
/// day `league::NEXT_DRAW_DAY` and the year `league::NEXT_DRAW_YEAR_OFFSET` + `league::YEAR` (words), returned as (day,
/// year).
unsafe fn wc_oceania_league_start_date(this: usize) -> (u16, u16) {
    // the date is 8 bytes: the day, the year and the leap-year dword at +4, which the routine also writes
    let mut date = [0u32; 2];
    let year = (read_i16(this + league::NEXT_DRAW_YEAR_OFFSET) as u16).wrapping_add(read_i16(this + league::YEAR) as u16);
    exe::new_date(date.as_mut_ptr() as usize, read_i16(this + league::NEXT_DRAW_DAY) as u16 as u32, year as u32);
    (date[0] as u16, (date[0] >> 16) as u16)
}

/// The Oceania qualifying day (`comp_vtable::UPDATE_FOR_DATE`; the original's SEH frame is only for the local dates,
/// with no destructor): `WcOceaniaLeague::update_for_date(today, full)`. Before the start (today's year lower,
/// unsigned; or the same year and the day lower, signed) it returns 0 without the league day. On the start day or on
/// any day of 2001 (a game starting in 2001 sets up the qualifying already under way), with `league::ACTIVE` = 0 and
/// WITH `full`: the Oceania national teams are released from the continent's competitions (0x775070([`G_NATIONAL_TEAMS`],
/// the continent [`G_CONTINENT_TABLE`] + 0xc6 × [`KEY_CONTINENT_OCEANIA`], 0): push 0; push; mov ecx -> thiscall, ret 8);
/// each national team of the first group's table (`league::TABLE`, `league_table_row::SIZE`-byte rows;
/// `league::CLUB_COUNT` rows) and each of the +0xf3 of the list +0xef (the non-null ones) enters the qualifying
/// (`exe::nationalteams_set_current_competition`([`G_NATIONAL_TEAMS`], its `club::NATION`, `league::COMPETITION`),
/// thiscall, ret 8); the fixtures (`exe::league_create_fixtures`; 0: the error box of line 0x3a4 and 0), the sorted
/// table (`exe::league_sort_table`(this, 0); 0: the error box of line 0x3ab and 0), the second group stage
/// (WcOceaniaLeague::create_second_group_stage 0x91f1a0, thiscall, ret 0) and `league::ACTIVE` = 1. Then the league
/// day (League::update_for_date `FN_LEAGUE_UPDATE_FOR_DATE`, thiscall(this, today, full), ret 8), whose return value it
/// passes on.
/// Original: `WcOceaniaLeague::update_for_date` (0x91f3d0, wc_oceania_league.cpp, thiscall, ret 8).
unsafe extern "thiscall" fn wc_oceania_league_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let release_continent: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x775070usize);
    let create_second_group_stage: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x91f1a0usize);
    // the full eax is passed on, so the u8 wrapper `exe::league_update_for_date` is not used
    let league_update_for_date: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_LEAGUE_UPDATE_FOR_DATE);
    let (day, year) = wc_oceania_league_start_date(this);
    let today_day = read_u16(today);
    let today_year = read_u16(today + 2);
    // ponytail: on the early returns before the start and on the errors the original returns only al (0); the caller reads the byte
    if today_year < year || (today_year == year && (today_day as i16) < day as i16) {
        return 0;
    }
    let set_up_day = (today_year == year && today_day == day) || today_year == 0x7d1; // cqu12: the 2001 game sets it up at once
    if set_up_day && read_usize(this + league::ACTIVE) == 0 && full != 0 {
        covered!("cqu_12 qualifying set up");
        let national_teams = read_usize(G_NATIONAL_TEAMS);
        release_continent(national_teams, read_usize(G_CONTINENT_TABLE).wrapping_add(read_usize(KEY_CONTINENT_OCEANIA).wrapping_mul(0xc6)), 0);
        let mut i = 0i16;
        while i < read_i16(this + league::CLUB_COUNT) {
            let team = read_usize(read_usize(this + league::TABLE) + i as usize * league_table_row::SIZE);
            exe::nationalteams_set_current_competition(national_teams, read_usize(team + club::NATION), read_usize(this + league::COMPETITION));
            i += 1;
        }
        let mut k = 0i16;
        while (k as i32) < read_usize(this + 0xf3) as i32 {
            let e = read_usize(read_usize(this + 0xef) + k as usize * 4);
            if e != 0 {
                exe::nationalteams_set_current_competition(national_teams, read_usize(e + club::NATION), read_usize(this + league::COMPETITION));
            }
            k = k.wrapping_add(1);
        }
        if exe::league_create_fixtures(this) == 0 {
            show_error_at(0xad4afc, 0x3a4);
            return 0;
        }
        if exe::league_sort_table(this, 0usize) == 0 {
            show_error_at(0xad4afc, 0x3ab);
            return 0;
        }
        create_second_group_stage(this);
        write_u32(this + league::ACTIVE, 1);
    }
    league_update_for_date(this, today, full)
}

// ponytail: the competition entries, the fixtures, the stage and the league day cannot run twice. Only the original;
// the plan on the set-up day (the copy's condition): `league::ACTIVE` = 1, the second group stage in
// [`league::STAGES`][0] and the current stage (`league::CURRENT_STAGE`) = 0.
unsafe extern "thiscall" fn compare_wc_oceania_league_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I0%%);
    let sets_up = read_usize(this + league::ACTIVE) == 0 && full != 0 && {
        let (day, year) = wc_oceania_league_start_date(this);
        let (td, ty) = (read_u16(today), read_u16(today + 2));
        !(ty < year || (ty == year && (td as i16) < day as i16)) && ((ty == year && td == day) || ty == 0x7d1)
    };
    let r = o(this, today, full);
    if sets_up {
        let stages = read_usize(this + league::STAGES);
        let ok = read_usize(this + league::ACTIVE) == 1 && stages != 0 && read_usize(stages) != 0 && read_usize(this + league::CURRENT_STAGE) == 0;
        SCOREBOARD.record(ok, || format!("wc_oceania_league_update_for_date({:#x}): the set-up did not build the stage (+0x45 {}, +0x30 {})", this, read_usize(this + league::ACTIVE), read_usize(this + league::CURRENT_STAGE) as i32));
    }
    r
}

/// The first Oceania qualifying group has 5 national teams (`league::CLUB_COUNT`), the table `league::TABLE`
/// (`exe::crt_malloc`(0x127), cdecl: 5 rows of `league_table_row::SIZE` bytes; out of memory, the error box of line
/// 0x24b and 0), each row i with the national team of the even places of the list +0xef (2i)
/// (`exe::league_init_table_row`(this, i, team, 0, 0), thiscall, ret 0x10), the group draw news
/// (`exe::league_send_group_draw_news`) and returns 1.
/// Original: `WcOceaniaLeague::init_group_table` (0x91ec00, wc_oceania_league.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn wc_oceania_league_init_group_table(this: usize) -> u32 {
    write_i16(this + league::CLUB_COUNT, 5); // cqu12: 5 national teams in the first group
    let t = exe::crt_malloc(0x127);
    write_usize(this + league::TABLE, t);
    if t == 0 {
        show_error_at(0xad4afc, 0x24b);
        return 0;
    }
    let mut i = 0i8;
    while (i as i16) < read_i16(this + league::CLUB_COUNT) {
        covered!("cqu_12 first group row");
        // ponytail: the index goes in a dword whose 3 high bytes are stack garbage; the routine reads only the byte (char)
        exe::league_init_table_row(this, i as u8 as u32, read_usize(read_usize(this + 0xef) + i as isize as usize * 8), 0, 0);
        i = i.wrapping_add(1);
    }
    exe::league_send_group_draw_news(this);
    1
}

// ponytail: the table is allocated (and each row allocates its history) and the news goes to the managers: it cannot
// run twice. Only the original; the plan: returns 1, `league::CLUB_COUNT` = 5 and in row i the national team 2i of
// the list with position i (`league_table_row::POSITION`).
unsafe extern "thiscall" fn compare_wc_oceania_league_init_group_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    let list = read_usize(this + 0xef);
    if list == 0 {
        return o(this);
    }
    let expected: [usize; 5] = core::array::from_fn(|i| read_usize(list + i * 8));
    let r = o(this);
    let t = read_usize(this + league::TABLE);
    let mut read = [(0usize, 0u8); 5];
    if t != 0 {
        for (i, l) in read.iter_mut().enumerate() {
            *l = (read_usize(t + i * league_table_row::SIZE), read_u8(t + i * league_table_row::SIZE + league_table_row::POSITION));
        }
    }
    let same = r == 1 && read_i16(this + league::CLUB_COUNT) == 5 && t != 0 && (0..5).all(|i| read[i] == (expected[i], i as u8));
    SCOREBOARD.record(same, || format!("wc_oceania_league_init_group_table({:#x}) returned {} rows {:x?} expected {:x?}", this, r, read, expected));
    r
}

/// The second Oceania qualifying group has 4 national teams, or 5 when `wc_oceania_league_fields::EXEMPT_COUNT` is
/// not 1; the teams in an array (`exe::crt_malloc`(4 × n), cdecl; out of memory, the error box of line 0x355 and 0)
/// with the national teams of the odd places of the list +0xef (2i + 1); the round table through
/// `comp_vtable::CREATE_SCHEDULE` (push 0; push &id; push &rounds; push 0; mov ecx, this -> thiscall, ret 0x10); the
/// stage (`exe::crt_operator_new`(0xee), cdecl; the League_Stage constructor `FN_LEAGUE_STAGE_LEAGUE_STAGE`, thiscall,
/// ret 0x58, 22 arguments: the qualifying, n, the array, 1, `league::COMPETITION`, the round table, the rounds (word),
/// `league::WIN_POINTS`, `league::DRAW_POINTS`, `league::MATCH_CATEGORY`, &`league::TIEBREAKS`, a local 4-byte array
/// {1, 0, 0, 0} (the places [inference]), `league::YEAR` (word), 0, the id (word), `league::IMPORTANCE`, 4, 0,
/// `league::LEAGUE_FLAGS` (word), −1, 0, 2; the original pushes the words and bytes with garbage in the high bytes,
/// which the constructor does not read: the copy passes clean values) in [`league::STAGES`][0]; the array and the
/// round table freed (`exe::crt_free`). Without the stage: the error box of line 0x36b and 0. With it:
/// `league::CURRENT_STAGE` = 0, the group news (`exe::league_send_group_draw_news` on the stage) and 1. (The original's
/// SEH frame is only for the stage's new: no destructor on the normal path.)
/// Original: `WcOceaniaLeague::create_second_group_stage` (0x91f1a0, wc_oceania_league.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn wc_oceania_league_create_second_group_stage(this: usize) -> u32 {
    #[allow(clippy::type_complexity)]
    let construct_stage: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let places: [u8; 4] = [1, 0, 0, 0];
    let n = 4 + (read_u8(this + wc_oceania_league_fields::EXEMPT_COUNT) != 1) as usize;
    let teams = exe::crt_malloc((n * 4) as u32);
    if teams == 0 {
        show_error_at(0xad4afc, 0x355);
        return 0;
    }
    for i in 0..n {
        covered!("cqu_12 second group team");
        write_usize(teams + i * 4, read_usize(read_usize(this + 0xef) + i * 8 + 4)); // cqu12: the national team in place i of the second group
    }
    let create_schedule: unsafe extern "thiscall" fn(usize, i32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut id) = (0i16, 0i16);
    let table = create_schedule(this, 0, &mut rounds, &mut id, 0);
    let p = exe::crt_operator_new(0xee);
    let stage = if p != 0 {
        let b = |d: usize| read_u8(this + d) as u32;
        construct_stage(
            p,
            this,
            n as u32,
            teams,
            1,
            read_usize(this + league::COMPETITION),
            table,
            rounds as u16 as u32,
            b(league::WIN_POINTS),
            b(league::DRAW_POINTS),
            b(league::MATCH_CATEGORY),
            this + league::TIEBREAKS,
            places.as_ptr() as usize,
            read_i16(this + league::YEAR) as u16 as u32,
            0,
            id as u16 as u32,
            b(league::IMPORTANCE),
            4,
            0,
            read_i16(this + league::LEAGUE_FLAGS) as u16 as u32,
            -1,
            0,
            2,
        )
    } else {
        0
    };
    write_usize(read_usize(this + league::STAGES), stage);
    exe::crt_free(teams);
    exe::crt_free(table);
    let f = read_usize(read_usize(this + league::STAGES));
    if f == 0 {
        show_error_at(0xad4afc, 0x36b);
        return 0;
    }
    write_u32(this + league::CURRENT_STAGE, 0);
    exe::league_send_group_draw_news(f);
    1
}

// ponytail: the stage is allocated and the news goes to the managers: it cannot run twice. Only the original; the
// plan: returns 1 with the stage in [`league::STAGES`][0], `league::CURRENT_STAGE` = 0, the stage with n teams and the
// qualifying's year and, in its table (`league::TABLE`, `league_table_row::SIZE`-byte rows), the national teams of the
// odd places of the list (in any order: the constructor sorts the table).
unsafe extern "thiscall" fn compare_wc_oceania_league_create_second_group_stage(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    let list = read_usize(this + 0xef);
    if list == 0 {
        return o(this);
    }
    let n = 4 + (read_u8(this + wc_oceania_league_fields::EXEMPT_COUNT) != 1) as usize;
    let mut plan: Vec<usize> = (0..n).map(|i| read_usize(list + i * 8 + 4)).collect();
    let r = o(this);
    if r == 1 {
        let f = read_usize(read_usize(this + league::STAGES));
        let t = if f != 0 { read_usize(f + league::TABLE) } else { 0 };
        let mut a: Vec<usize> = if t != 0 { (0..n).map(|k| read_usize(t + k * league_table_row::SIZE)).collect() } else { Vec::new() };
        a.sort_unstable();
        plan.sort_unstable();
        let ok = f != 0 && read_usize(this + league::CURRENT_STAGE) == 0 && read_i16(f + league::CLUB_COUNT) as usize == n && read_i16(f + league::YEAR) == read_i16(this + league::YEAR) && a == plan;
        SCOREBOARD.record(ok, || format!("wc_oceania_league_create_second_group_stage: the stage does not match the plan ({:x?} read {:x?})", plan, a));
    }
    r
}
