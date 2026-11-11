// ---------- %%LEVA%%a leva: a Copa das Nações da Oceania: o dia do sorteio, o sorteio das seis seleções e a tabela do primeiro grupo (oceania_nations.cpp) ----------

/// The draw date (`exe::new_date` on a local date: push year; push day; lea ecx -> thiscall, ret 8): the day
/// `league::NEXT_DRAW_DAY` and the year `league::NEXT_DRAW_YEAR_OFFSET` + `league::YEAR` (words), returned as (day,
/// year).
unsafe fn oceania_nations_draw_date(this: League) -> (u16, u16) {
    // the date is 8 bytes: the day, the year and the leap-year dword at +4, which 0x549ef0 also writes
    let mut date = [0u32; 2];
    let year = (this.next_draw_year_offset() as u16).wrapping_add(this.year() as u16);
    exe::new_date(date.as_mut_ptr() as usize, this.next_draw_day() as u16 as u32, year as u32);
    (date[0] as u16, (date[0] >> 16) as u16)
}

/// The day of the Oceania Nations Cup (the virtual `comp_vtable::UPDATE_FOR_DATE`; SEH frame only for the local dates,
/// no destructor). Without `full`, only the league day (`League::update_for_date`(this, today, 0): 2 pushes; mov ecx ->
/// thiscall, ret 8). Before the draw (today's year lower, unsigned; or the same year and the day lower, signed) it
/// returns 0 without the league day. On the draw day (`oceania_nations_draw_date`), with `league::ACTIVE` = 0: the six
/// nations (`oceania_nations_select_and_draw_teams`, 0x799e30, thiscall, ret 0), the predicted positions (the virtual
/// `comp_vtable::SET_PREDICTED_POSITIONS`, thiscall, ret 0), the first group table (`oceania_nations_create_group_table`,
/// 0x7995a0, thiscall, ret 0; 0: the box for line 0x405 and 0), the fixtures (`exe::league_create_fixtures`; 0: the box
/// for line 0x40b and 0), the sorted table (`exe::league_sort_table`(this, 0): push 0; mov ecx -> thiscall, ret 4; 0:
/// the box for line 0x412 and 0), the second group stage (0x799a60(this, 0): push 0; mov ecx -> thiscall, ret 4), the
/// entrant list +0xee freed (`exe::crt_free`) and zeroed, its count +0xf2 (word) = 0 and `league::ACTIVE` = 1. Then the
/// league day, whose return value it passes on.
/// Original: `OceaniaNations::update_for_date` (0x79a100, oceania_nations.cpp, thiscall, ret 8).
unsafe extern "thiscall" fn oceania_nations_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let league = League(this);
    let select_and_draw: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x799e30usize);
    let first_group: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x7995a0usize);
    let second_group: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x799a60usize);
    // the full eax is passed on (the exe:: wrapper reads only the byte)
    let league_day: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_LEAGUE_UPDATE_FOR_DATE);
    if full == 0 {
        return league_day(this, today, 0);
    }
    let (day, year) = oceania_nations_draw_date(league);
    let today_day = read_u16(today);
    let today_year = read_u16(today + 2);
    // simplification: on the returns before the draw and on the errors the original returns only al (0); the caller reads the byte
    if today_year < year || (today_year == year && (today_day as i16) < day as i16) {
        return 0;
    }
    if today_year == year && today_day == day && league.active() == 0 {
        covered!("cqv_10 draw day");
        select_and_draw(this);
        let predicted: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::SET_PREDICTED_POSITIONS));
        predicted(this);
        if first_group(this) == 0 {
            show_error_at(0xa6a0ec, 0x405);
            return 0;
        }
        if exe::league_create_fixtures(league) == 0 {
            show_error_at(0xa6a0ec, 0x40b);
            return 0;
        }
        if exe::league_sort_table(league, 0) == 0 {
            show_error_at(0xa6a0ec, 0x412);
            return 0;
        }
        second_group(this, 0);
        exe::crt_free(read_usize(this + 0xee));
        write_u32(this + 0xee, 0);
        write_i16(this + 0xf2, 0);
        write_u32(this + league::ACTIVE, 1);
    }
    league_day(this, today, full)
}

// simplification: the draw, the tables, the fixtures and the league day cannot run twice. Only the original; the plan on
// the draw day (the copy's condition): `league::ACTIVE` = 1, the list +0xee and +0xf2 zeroed, the first group table and
// the second group stage created.
unsafe extern "thiscall" fn compare_oceania_nations_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I0%%);
    let league = League(this);
    let draws = full != 0 && league.active() == 0 && {
        let (day, year) = oceania_nations_draw_date(league);
        read_u16(today) == day && read_u16(today + 2) == year
    };
    let r = o(this, today, full);
    if draws {
        let stages = league.stages();
        let ok = league.active() == 1 && read_usize(this + 0xee) == 0 && read_i16(this + 0xf2) == 0 && league.table() != 0 && stages != 0 && read_usize(stages) != 0;
        SCOREBOARD.record(ok, || format!("oceania_nations_update_for_date({:#x}): the draw did not build the groups (+0x45 {})", this, league.active()));
    }
    r
}

/// The record of the nation of key `key` (global): `G_NATION_TABLE` + `nation::SIZE` x [key].
unsafe fn oceania_nations_nation_record(key: usize) -> usize {
    read_usize(G_NATION_TABLE).wrapping_add(read_usize(key).wrapping_mul(nation::SIZE))
}

/// The six nations of the Oceania Nations Cup and their draw. +0xf2 = 6 and the list +0xee of 6 national teams
/// (`exe::crt_malloc`(0x18), cdecl; out of memory, the box for line 0x39e and 0). Of the Oceania nations (continent
/// `KEY_CONTINENT_OCEANIA`, `nation::CONTINENT`) other than Australia (`KEY_NATION_AUSTRALIA`) and New Zealand
/// (`KEY_NATION_NEW_ZEALAND`): up to 10 Melanesian (`exe::database_is_melanesian_nation`) and up to 10 Polynesian
/// (`exe::database_is_polynesian_nation`; a Melanesian one beyond the 10 falls through to the Polynesian test), each by
/// its main national team (`exe::database_get_national_team`). A draw of 2 (`exe::utils_random`(2), cdecl) puts
/// Australia in slot 0 and New Zealand in slot 3 (with 1) or the other way round; the Melanesian and the Polynesian
/// teams are sorted and shuffled (`exe::comp_util_sort_and_shuffle_top_clubs`(list, n, 2): 3 pushes -> cdecl); another
/// draw puts the first two Melanesian teams in slots 1 and 4 (or 4 and 1) and another the first two Polynesian ones in
/// slots 2 and 5 (or 5 and 2). Returns 1.
/// Original: `OceaniaNations::select_and_draw_teams` (0x799e30, oceania_nations.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn oceania_nations_select_and_draw_teams(this: usize) -> u32 {
    write_i16(this + 0xf2, 6);
    let list = exe::crt_malloc(0x18);
    write_usize(this + 0xee, list);
    if list == 0 {
        show_error_at(0xa6a0ec, 0x39e);
        return 0;
    }
    let (mut mel, mut pol) = ([0usize; 10], [0usize; 10]);
    let (mut nm, mut np) = (0i8, 0i8);
    let oceania = read_usize(KEY_CONTINENT_OCEANIA);
    let mut i = 0i16;
    while (i as i32) < read_usize(G_NATION_COUNT) as i32 {
        let rec = read_usize(G_NATION_TABLE).wrapping_add((i as isize as usize).wrapping_mul(nation::SIZE));
        let cont = Nation(rec).continent();
        let k = i as i32;
        if cont != 0 && read_usize(cont) == oceania && k != read_usize(KEY_NATION_AUSTRALIA) as i32 && k != read_usize(KEY_NATION_NEW_ZEALAND) as i32 {
            if exe::database_is_melanesian_nation(rec) != 0 && nm < 10 {
                mel[nm as usize] = exe::database_get_national_team(rec);
                nm += 1;
            } else if exe::database_is_polynesian_nation(rec) != 0 && np < 10 {
                // cqv10: up to 10 Polynesian teams
                pol[np as usize] = exe::database_get_national_team(rec);
                np += 1;
            }
        }
        i = i.wrapping_add(1);
    }
    covered!("cqv_10 oceania cup draw");
    let (australia, new_zealand) = (oceania_nations_nation_record(KEY_NATION_AUSTRALIA), oceania_nations_nation_record(KEY_NATION_NEW_ZEALAND));
    if exe::utils_random(2) != 0 { // cqv10: the draw of Australia and New Zealand
        write_usize(read_usize(this + 0xee), exe::database_get_national_team(australia));
        write_usize(read_usize(this + 0xee) + 0xc, exe::database_get_national_team(new_zealand));
    } else {
        write_usize(read_usize(this + 0xee) + 0xc, exe::database_get_national_team(australia));
        write_usize(read_usize(this + 0xee), exe::database_get_national_team(new_zealand));
    }
    exe::comp_util_sort_and_shuffle_top_clubs(mel.as_mut_ptr() as usize, nm as i16 as u16 as u32, 2); // cqv10: the Melanesian teams with 2 picks
    exe::comp_util_sort_and_shuffle_top_clubs(pol.as_mut_ptr() as usize, np as i16 as u16 as u32, 2);
    let slot = |k: usize| read_usize(this + 0xee) + k * 4;
    if exe::utils_random(2) != 0 {
        write_usize(slot(1), mel[0]);
        write_usize(slot(4), mel[1]);
    } else {
        write_usize(slot(4), mel[0]);
        write_usize(slot(1), mel[1]);
    }
    if exe::utils_random(2) != 0 {
        write_usize(slot(2), pol[0]);
        write_usize(slot(5), pol[1]);
    } else {
        write_usize(slot(5), pol[0]);
        write_usize(slot(2), pol[1]);
    }
    1
}

// simplification: the list is allocated and the draws count: it cannot run twice. Only the original; the plan: returns
// 1, +0xf2 = 6 and, in the list, Australia and New Zealand in slots 0 and 3 (in either order).
unsafe extern "thiscall" fn compare_oceania_nations_select_and_draw_teams(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    let r = o(this);
    if r == 1 {
        let list = read_usize(this + 0xee);
        let mut a = [read_usize(list), read_usize(list + 0xc)];
        let mut b = [exe::database_get_national_team(oceania_nations_nation_record(KEY_NATION_AUSTRALIA)), exe::database_get_national_team(oceania_nations_nation_record(KEY_NATION_NEW_ZEALAND))];
        a.sort_unstable();
        b.sort_unstable();
        SCOREBOARD.record(read_i16(this + 0xf2) == 6 && a == b, || format!("oceania_nations_select_and_draw_teams({:#x}): slots 0 and 3 {:x?} expected {:x?}", this, a, b));
    }
    r
}

/// The first group of the Oceania Nations Cup has 3 national teams (`league::CLUB_COUNT`); the table
/// `league::TABLE` (`exe::crt_malloc`(0xb1): 3 rows of `league_table_row::SIZE` bytes; out of memory, the box for line
/// 0x246 and 0), each row i with national team i of the list +0xee (`exe::league_init_table_row`: push 0; push 0; push
/// team; push i; mov ecx -> thiscall, ret 0x10), the group draw news (`exe::league_send_group_draw_news`) and returns 1.
/// The body of the Oceania qualifier's 0x91ec00 with 3 rows and the list in 4-byte steps.
/// Original: `OceaniaNations::create_group_table` (0x7995a0, oceania_nations.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn oceania_nations_create_group_table(this: usize) -> u32 {
    let league = League(this);
    league.set_club_count(3);
    let t = exe::crt_malloc(0xb1);
    league.set_table(t);
    if t == 0 {
        show_error_at(0xa6a0ec, 0x246);
        return 0;
    }
    let mut i = 0i8;
    while (i as i16) < league.club_count() {
        // simplification: the index goes in a dword whose 3 high bytes are stack leftovers; 0x687430 reads only the byte (char)
        exe::league_init_table_row(league, i as u8 as u32, read_usize(read_usize(this + 0xee) + i as isize as usize * 4), 0, 0);
        i = i.wrapping_add(1);
    }
    exe::league_send_group_draw_news(league);
    1
}

// simplification: the table is allocated and the news goes to the managers: it cannot run twice. Only the original; the
// plan: returns 1, `league::CLUB_COUNT` = 3 and in row i national team i of the list with position i (`league_table_row::POSITION`).
unsafe extern "thiscall" fn compare_oceania_nations_create_group_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    let list = read_usize(this + 0xee);
    if list == 0 {
        return o(this);
    }
    let expected: [usize; 3] = core::array::from_fn(|i| read_usize(list + i * 4));
    let r = o(this);
    let league = League(this);
    let t = league.table();
    let mut read = [(0usize, 0u8); 3];
    if t != 0 {
        for (i, l) in read.iter_mut().enumerate() {
            *l = (read_usize(t + i * league_table_row::SIZE), read_u8(t + i * league_table_row::SIZE + league_table_row::POSITION));
        }
    }
    let same = r == 1 && league.club_count() == 3 && t != 0 && (0..3).all(|i| read[i] == (expected[i], i as u8));
    SCOREBOARD.record(same, || format!("oceania_nations_create_group_table({:#x}) returned {} rows {:x?} expected {:x?}", this, r, read, expected));
    r
}
