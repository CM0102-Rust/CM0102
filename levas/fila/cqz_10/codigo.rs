// ---------- cqz_10: the European Championship finals: the constructor, the team list and the day (euro_champ.cpp) ----------

/// The European Championship edition: from `start`, up one year at a time while the remainder of (year − 1992) by 4 is
/// not 0 (signed, as the original's `and 0x80000003` with its sign adjustment).
fn euro_champ_edition(start: i16) -> i16 {
    let mut year = start;
    while (year as i32 - 0x7c8) % 4 != 0 {
        year = year.wrapping_add(1);
    }
    year
}

/// The European Championship finals at career creation.
///
/// League::League (`exe::new_league`, mov ecx -> thiscall, ret 0); the record (`league::COMPETITION`), vtable 0x96a020,
/// `club_sides` 4, no upper or lower division (−1), `league::ENTRANTS_ARE_QUALIFIERS` 1, discipline rule 0, the edition
/// (`euro_champ_edition` of the year after the game's: in a 2001 game, 2004), `league_flags` 0x28 and importance 0xf.
/// If the saved state loads (`exe::league_load_from_temp_file(this, 1)`: push 1; mov ecx -> thiscall, ret 4;
/// nonzero), returns this.
///
/// Otherwise: the main stage (0xff), `current_stage` −1, 4 stages and the stage vector (`exe::crt_malloc(0x10)`: push;
/// add esp -> cdecl, unchecked); the nation list (EuroChamp::set_up_team_list 0x57bd80, thiscall, ret 0; its return
/// value is not read); the rules (EuroChamp::set_up_rules 0x57b630; 0: the error box of line 0x5f of euro_champ.cpp and
/// returns this); the statistics (operator new `exe::crt_operator_new(0x5ce)` and Comp_Stats::Comp_Stats
/// `exe::new_comp_stats(new, this)`: push this; mov ecx, new -> thiscall, ret 4) into `comp_stats`, null: the box of
/// line 0x66 (and it goes on). Then, when the edition is less than a year after the game's year (signed words), the
/// entrant count (`league::ENTRANT_COUNT`) = 16, else `league::ACTIVE` = 0 (with the edition in the same year the
/// active flag is left as it was); `league::CLUB_COUNT` = 0 and `league::TABLE` = null. Returns this.
/// Original: `EuroChamp::EuroChamp` (0x57a890, euro_champ.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_euro_champ(this: usize, year: u32, record: usize) -> usize {
    let league = League(this);
    exe::new_league(league);
    write_u32(this + league::COMPETITION, record as u32);
    write_u32(this, 0x96a020);
    league.set_club_sides(4);
    write_u32(this + league::UPPER_DIVISION_ID, 0xffff_ffff);
    write_u32(this + league::LOWER_DIVISION_ID, 0xffff_ffff);
    write_u8(this + league::ENTRANTS_ARE_QUALIFIERS, 1);
    league.set_discipline_rule(0);
    let edition = euro_champ_edition((year as u16).wrapping_add(1) as i16); // cqz10: the next European Championship
    league.set_year(edition);
    league.set_league_flags(0x28);
    league.set_importance(0xf);
    if exe::league_load_from_temp_file(league, 1) != 0 {
        return this;
    }
    covered!("cqz_10 euro finals");
    league.set_stage_index(stage_index::MAIN);
    write_u32(this + league::CURRENT_STAGE, 0xffff_ffff);
    write_u32(this + league::STAGE_COUNT, 4);
    write_u32(this + league::STAGES, exe::crt_malloc(0x10) as u32);
    let team_list: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x57bd80usize);
    team_list(this);
    let rules: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x57b630usize);
    if rules(this) == 0 {
        show_error_at(0x9c4638, 0x5f);
        return this;
    }
    let p = exe::crt_operator_new(0x5ce);
    let e = if p != 0 { exe::new_comp_stats(p, league) } else { 0 };
    write_u32(this + league::COMP_STATS, e as u32);
    if e == 0 {
        show_error_at(0x9c4638, 0x66);
    }
    if (league.year() as i32) - (year as u16 as i16 as i32) < 1 { // cqz10: the finals of this year already have their 16
        write_u16(this + league::ENTRANT_COUNT, 0x10);
    } else {
        write_u32(this + league::ACTIVE, 0);
    }
    write_u16(this + league::CLUB_COUNT, 0);
    write_u32(this + league::TABLE, 0);
    this
}

/// Comparer for `construct_euro_champ`: only the original runs (it allocates the stages, the entrant book and the
/// statistics), then the plan (`compare_nations_tournament_ctor`: this, vtable, record, edition year and importance).
pub(crate) unsafe extern "thiscall" fn compare_construct_euro_champ(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    compare_nations_tournament_ctor("construct_euro_champ", League(this), record, 0x96a020, euro_champ_edition((year as u16).wrapping_add(1) as i16), || o(this, year, record))
}

/// The nations of the default European Championship field after the hosts, in entrant-book slots 2 to 15 (the keys of
/// their nations): Italy, Finland, Spain, Scotland, Romania, Germany, France, Wales, England, Denmark, Austria, Croatia,
/// Bulgaria and Norway.
const EURO_CHAMP_DEFAULT_FIELD: [usize; 14] = [KEY_NATION_ITALY, KEY_NATION_FINLAND, KEY_NATION_SPAIN, KEY_NATION_SCOTLAND, KEY_NATION_ROMANIA,
    KEY_NATION_GERMANY, KEY_NATION_FRANCE, KEY_NATION_WALES, KEY_NATION_ENGLAND, KEY_NATION_DENMARK, KEY_NATION_AUSTRIA, KEY_NATION_CROATIA,
    KEY_NATION_BULGARIA, KEY_NATION_NORWAY];

/// The national team of the nation with id `id` (`exe::database_get_national_team` on `G_NATION_TABLE` + `nation::SIZE`
/// × id: push; add esp, 4 -> cdecl).
unsafe fn national_team_of_id(id: usize) -> usize {
    exe::database_get_national_team(read_usize(G_NATION_TABLE).wrapping_add(id.wrapping_mul(nation::SIZE)))
}

/// rule: the European Championship field before qualifying: sixteen places, the hosts first (marked as qualified) and
/// then a fixed list of fourteen nations (Italy, Finland, Spain, Scotland, Romania, Germany, France, Wales, England,
/// Denmark, Austria, Croatia, Bulgaria and Norway) in slots 2 to 15
///
/// `league::ENTRANT_COUNT` = 0, `league::ENTRANT_CAPACITY` = 16 and the entrant book (`exe::crt_calloc(16, 6)`: 2
/// pushes; add esp -> cdecl) into `league::ENTRANT_BOOK`; without it, the error box of line 0x32d and 0. The hosts of
/// the finals year (HostCountry::find_hosts on [`G_HOST_COUNTRY`]: 5 pushes; mov ecx -> thiscall, ret 0x14; the year
/// pushed with garbage in the high 16 bits, read as a word): with one or more, the entry at the count gets status 1
/// (+4) and the first host's national team and the count goes up; with two or more, the same for the second; with none,
/// the box of line 0x345 (and it goes on). Then slots 2 to 15 (+0xc to +0x5a, 6 bytes each) get the national teams of
/// `EURO_CHAMP_DEFAULT_FIELD` (status left 0, count unchanged). Returns 1.
/// Original: `EuroChamp::set_up_team_list` (0x57bd80, euro_champ.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn euro_champ_set_up_team_list(this: usize) -> u32 {
    let league = League(this);
    write_u16(this + league::ENTRANT_COUNT, 0);
    write_u16(this + league::ENTRANT_CAPACITY, 0x10);
    let book = exe::crt_calloc(0x10, 6);
    write_u32(this + league::ENTRANT_BOOK, book as u32);
    if book == 0 {
        show_error_at(0x9c4638, 0x32d);
        return 0;
    }
    let mut hosts = [-1i32; 2];
    let (a, b) = hosts.split_at_mut(1);
    let n = exe::hostcountry_find_hosts(read_usize(G_HOST_COUNTRY), read_usize(KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP), league.year() as u16 as u32, &mut a[0], &mut b[0], 1) as i8;
    if n > 0 {
        covered!("cqz_10 euro hosts");
        for &h in hosts.iter().take(if n > 1 { 2 } else { 1 }) {
            let slot = read_usize(this + league::ENTRANT_BOOK) + read_i16(this + league::ENTRANT_COUNT) as isize as usize * 6;
            write_u8(slot + 4, 1);
            let team = national_team_of_id(h as usize);
            let slot = read_usize(this + league::ENTRANT_BOOK) + read_i16(this + league::ENTRANT_COUNT) as isize as usize * 6;
            write_usize(slot, team);
            write_u16(this + league::ENTRANT_COUNT, read_u16(this + league::ENTRANT_COUNT).wrapping_add(1));
        }
    } else {
        show_error_at(0x9c4638, 0x345);
    }
    covered!("cqz_10 euro field");
    for (k, &key) in EURO_CHAMP_DEFAULT_FIELD.iter().enumerate() {
        let team = national_team_of_id(read_usize(key));
        write_usize(read_usize(this + league::ENTRANT_BOOK) + (k + 2) * 6, team); // cqz10: the default field in slots 2 to 15
    }
    1
}

// simplification: the entrant book is allocated: it cannot run twice. Only the original; the plan: return value 1 with the book,
// 16 places, the hosts counted, and slots 2 to 15 with the national teams of the default field.
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_set_up_team_list(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    let r = o(this);
    let book = read_usize(this + league::ENTRANT_BOOK);
    let ok = r == 0 || (book != 0 && read_u16(this + league::ENTRANT_CAPACITY) == 0x10 && read_u16(this + league::ENTRANT_COUNT) <= 2
        && EURO_CHAMP_DEFAULT_FIELD.iter().enumerate().all(|(k, &key)| read_usize(book + (k + 2) * 6) == national_team_of_id(read_usize(key))));
    SCOREBOARD.record(ok, || format!("euro_champ_set_up_team_list({:#x}) returned {}, {} entrants", this, r, read_u16(this + league::ENTRANT_COUNT)));
    r
}

/// rule: the European Championship day: on the seeding date (10 January of the finals year) the nations are seeded
/// into pots, and on the draw date (10 February) the groups are drawn, if the table does not exist yet; then the league
/// day
///
/// Without `full`, only the league day (`League::update_for_date`(this, today, 0): 2 pushes; mov ecx -> thiscall, ret
/// 8). With it: when today's day (signed word) equals `EURO_CHAMP_SEEDING_DAY` (unsigned word) and today's year
/// (unsigned word) equals `EURO_CHAMP_SEEDING_YEAR_OFFSET` (unsigned) + `.year()` (signed), the seeding
/// (EuroChamp::seed_teams 0x57c710, thiscall, ret 0). Then the draw date (0x549ef0 on `.next_draw_day()` and
/// `.next_draw_year_offset()` + `.year()`): when today is that day (the two words) and `league::TABLE` is null, the
/// draw (EuroChamp::run_group_draw 0x57c1d0, thiscall, ret 0). Then the league day with `full`, whose return value (al)
/// it passes on.
/// Original: `EuroChamp::update_for_date` (0x57c620, euro_champ.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn euro_champ_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let league = League(this);
    // the full eax is passed on (the exe:: wrapper reads only the byte)
    let league_day: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(FN_LEAGUE_UPDATE_FOR_DATE);
    if full == 0 {
        return league_day(this, today, 0) as u32;
    }
    covered!("cqz_10 euro day");
    let seeding_day = read_u16(this + 0x13e) as i32 == read_i16(today) as i32;
    let seeding_year = (read_u16(this + 0x140) as i32).wrapping_add(league.year() as i32) == read_u16(today + 2) as i32;
    if seeding_day && seeding_year { // cqz10: the seeding day
        let seed: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x57c710usize);
        seed(this);
    }
    let (day, year) = oceania_nations_draw_date(league);
    if read_u16(today + 2) == year && read_u16(today) == day && league.table() == 0 {
        let draw: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x57c1d0usize);
        draw(this);
    }
    league_day(this, today, full) as u32
}

// simplification: the seeding, the draw and the league day cannot run twice. Only the original; the plan on the draw day (the
// copy's condition): the group table built.
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I2%%);
    let league = League(this);
    let draws = full != 0 && league.table() == 0 && {
        let (day, year) = oceania_nations_draw_date(league);
        read_u16(today) == day && read_u16(today + 2) == year
    };
    let r = o(this, today, full);
    if draws {
        SCOREBOARD.record(league.table() != 0, || format!("euro_champ_update_for_date({:#x}): the draw did not build the table", this));
    }
    r
}
