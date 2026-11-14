// ---------- cqz_9: the Asian Cup honours, the Oceania Nations Cup predictions and the Oceania qualifying stages (asia_nations.cpp, oceania_nations.cpp, wc_oceania_league.cpp) ----------

/// rule: the season honours of the Asian Cup (winner, runner-up, third) are those of its knockout, the third stage
///
/// A tail call: the stage `.stages()`[2] (the knockout) does its own `comp_vtable::RECORD_SEASON_HONOURS` (thiscall,
/// ret 0).
/// Original: `AsiaNations::record_season_honours` (0x40f590, asia_nations.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn asia_nations_record_season_honours(this: usize) {
    let this = League(this);
    covered!("cqz_9 asian cup honours");
    let stage = read_usize(this.stages() + 8); // cqz9: the knockout is the third stage
    let h: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(stage) + comp_vtable::RECORD_SEASON_HONOURS));
    h(stage);
}

// simplification: the honours go to the history: they cannot run twice, and the routine writes nothing beyond the jump. Only the
// original.
pub(crate) unsafe extern "thiscall" fn compare_asia_nations_record_season_honours(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ASIA_NATIONS_RECORD_SEASON_HONOURS)(this)
}

/// The predicted final places of the six Oceania Nations Cup nations, in reputation order.
const OCEANIA_NATIONS_PREDICTED_PLACES: [u32; 6] = [1, 2, 3, 3, 5, 5];

/// rule: the Oceania Nations Cup predictions: the six drawn nations by reputation get the places 1, 2, 3, 3, 5 and 5
///
/// The virtual `comp_vtable::SET_PREDICTED_POSITIONS`. Without statistics (`league::COMP_STATS`), the error box of line
/// 0x436 and return; a copy of the drawn list +0xee with the count +0xf2 (signed word) × 4 bytes (`exe::crt_malloc`:
/// push; add esp -> cdecl; out of memory, the box of line 0x43e and return), sorted by reputation (qsort
/// `exe::crt_qsort`(copy, count, 4, 0x4b07d0): 4 pushes -> cdecl), and the first six get their predicted place in the
/// statistics (`exe::comp_stats_set_club_final_position`: push place; push team; mov ecx, statistics -> thiscall, ret
/// 8), whatever the count; the copy freed (`exe::crt_free`).
/// Original: `OceaniaNations::set_predicted_positions` (0x79a320, oceania_nations.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn oceania_nations_set_predicted_positions(this: usize) {
    if read_usize(this + league::COMP_STATS) == 0 {
        return show_error_at(0xa6a0ec, 0x436);
    }
    let n = read_i16(this + 0xf2) as i32;
    let v = exe::crt_malloc((n << 2) as u32);
    if v == 0 {
        return show_error_at(0xa6a0ec, 0x43e);
    }
    core::ptr::copy_nonoverlapping(read_usize(this + 0xee) as *const u8, v as *mut u8, ((n << 2) as u32) as usize);
    exe::crt_qsort(v, n as u32, 4, 0x4b07d0);
    covered!("cqz_9 oceania nations predictions");
    for (k, &p) in OCEANIA_NATIONS_PREDICTED_PLACES.iter().enumerate() {
        exe::comp_stats_set_club_final_position(read_usize(this + league::COMP_STATS), read_usize(v + k * 4), p); // cqz9: the place by reputation
    }
    exe::crt_free(v);
}

// simplification: the statistics find or create each nation's entry (allocates): it cannot run twice. Only the original; the plan:
// the list sorted by reputation first, then the place (`CompStatsClub::final_position`) of each of the six equal to the
// band's.
pub(crate) unsafe extern "thiscall" fn compare_oceania_nations_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_OCEANIA_NATIONS_SET_PREDICTED_POSITIONS);
    if read_usize(this + league::COMP_STATS) == 0 || read_usize(this + 0xee) == 0 || read_i16(this + 0xf2) != 6 {
        return o(this);
    }
    let mut v: Vec<usize> = (0..6).map(|i| read_usize(read_usize(this + 0xee) + i * 4)).collect();
    exe::crt_qsort((v.as_mut_ptr()) as usize, 6, 4, 0x4b07d0);
    o(this);
    let wrong = (0..6).find(|&k| {
        let e = CompStatsClub(if v[k] != 0 { exe::comp_stats_find_club_entry(read_usize(this + league::COMP_STATS), v[k]) } else { 0 });
        e.is_null() || e.final_position() as u32 != OCEANIA_NATIONS_PREDICTED_PLACES[k]
    });
    SCOREBOARD.record(wrong.is_none(), || format!("oceania_nations_set_predicted_positions: the predicted place of slot {:?} differs", wrong));
}

/// rule: the Oceania World Cup qualifying moves on stage by stage: from the groups to the tie of the group winners (stage
/// 1), and from it to the inter-continental play-off (stage 2) only once the South American qualifying is complete
/// (the play-off opponent comes from it); past the last stage it is an error
///
/// On stage 1 (`league::CURRENT_STAGE` = 1): the South American section (the competition object of
/// `KEY_CLUB_COMP_WORLD_CUP_SOUTH_AMERICAN_QUALIFYING_SECT` in [`G_COMP_OBJECT_TABLE`]) must be complete (its
/// `comp_vtable::CHECK_COMPLETE` with 1, 1: push 1; push 1; mov ecx -> thiscall, ret 8; al 0 returns). At or past the
/// last stage (`stage_count` − 1, signed): the error box of line 0x329. Otherwise the stage goes up one: to 1, the tie of
/// the group winners (WcOceaniaLeague::create_final_stage 0x91f640, thiscall, ret 0); to 2, the play-off
/// (WcOceaniaLeague::create_playoff_stage 0x91f800, thiscall, ret 0).
/// Original: `WcOceaniaLeague::advance_stage` (0x91f0d0, wc_oceania_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_oceania_league_advance_stage(this: usize) {
    let league = League(this);
    if read_u32(this + league::CURRENT_STAGE) == 1 {
        let south = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_WORLD_CUP_SOUTH_AMERICAN_QUALIFYING_SECT).wrapping_mul(4)));
        let complete: unsafe extern "thiscall" fn(usize, u32, u32) -> u8 = core::mem::transmute(read_usize(read_usize(south) + comp_vtable::CHECK_COMPLETE));
        if complete(south, 1, 1) == 0 { // cqz9: the play-off waits for the South American qualifying
            return;
        }
    }
    let stage = read_u32(this + league::CURRENT_STAGE) as i32;
    if stage >= (league.stage_count() as i32).wrapping_sub(1) {
        show_error_at(0xad4afc, 0x329);
        return;
    }
    let stage = stage + 1;
    write_u32(this + league::CURRENT_STAGE, stage as u32);
    covered!("cqz_9 oceania qualifying next stage");
    match stage {
        1 => {
            let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x91f640usize);
            f(this);
        }
        2 => {
            let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x91f800usize);
            f(this);
        }
        _ => {}
    }
}

// simplification: the next stage is allocated and drawn: it cannot run twice. Only the original; the plan: the stage went up one
// (when not at the last one and, from stage 1, with the South American qualifying complete), or stayed.
pub(crate) unsafe extern "thiscall" fn compare_wc_oceania_league_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_WC_OCEANIA_LEAGUE_ADVANCE_STAGE);
    let before = read_u32(this + league::CURRENT_STAGE) as i32;
    o(this);
    let after = read_u32(this + league::CURRENT_STAGE) as i32;
    SCOREBOARD.record(after == before || after == before + 1, || format!("wc_oceania_league_advance_stage({:#x}): stage {} -> {}", this, before, after));
}

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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_EURO_CHAMP);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_EURO_CHAMP_SET_UP_TEAM_LIST);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_EURO_CHAMP_UPDATE_FOR_DATE);
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

// ---------- cqz_11: the World Cup finals: the constructor and the nation list (world_cup.cpp) ----------

/// The World Cup edition: from `start`, up one year at a time while the remainder of (year − 1994) by 4 is not 0
/// (signed, as the original's `and 0x80000003` with its sign adjustment).
fn world_cup_edition(start: i16) -> i16 {
    let mut year = start;
    while (year as i32 - 0x7ca) % 4 != 0 {
        year = year.wrapping_add(1);
    }
    year
}

/// The World Cup finals at career creation.
///
/// League::League (`exe::new_league`, mov ecx -> thiscall, ret 0); the record (`league::COMPETITION`), vtable 0x9712f0,
/// `club_sides` 4, no upper or lower division (−1), `league::ENTRANTS_ARE_QUALIFIERS` 1, discipline rule 0, the edition
/// (`world_cup_edition` of the year after the game's: in a 2001 game, 2002), importance 0xf; the entrant book of 32
/// places (`league::ENTRANT_COUNT` 0, `league::ENTRANT_CAPACITY` 0x20, `exe::crt_calloc(0x20, 6)`: 2 pushes; add esp ->
/// cdecl) into `league::ENTRANT_BOOK`; without it, the error box of line 0x58 of world_cup.cpp and returns this. If the
/// saved state loads (`exe::league_load_from_temp_file(this, 1)`), returns this.
///
/// Otherwise: `league_flags` 0x28, the main stage, `current_stage` −1, 8 stages and the stage vector
/// (`exe::crt_malloc(0x20)`, unchecked); the nation list (WorldCup::set_up_team_list 0x92da40, thiscall, ret 0; return
/// value not read); the rules (WorldCup::set_up_rules 0x92d200; 0: the box of line 0x72 and returns this); the
/// statistics (operator new `exe::crt_operator_new(0x5ce)` and Comp_Stats::Comp_Stats `exe::new_comp_stats(new,
/// this)`), null: the box of line 0x7a (and it goes on). Then, when the edition is the game's year (words), the entrant
/// count = 32, else `league::ACTIVE` = 0; `league::TABLE` = null and `league::CLUB_COUNT` = 0. Returns this.
/// Original: `WorldCup::WorldCup` (0x92bf50, world_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_world_cup(this: usize, year: u32, record: usize) -> usize {
    let league = League(this);
    exe::new_league(league);
    write_u32(this + league::COMPETITION, record as u32);
    write_u32(this, 0x9712f0);
    league.set_club_sides(4);
    write_u32(this + league::UPPER_DIVISION_ID, 0xffff_ffff);
    write_u32(this + league::LOWER_DIVISION_ID, 0xffff_ffff);
    write_u8(this + league::ENTRANTS_ARE_QUALIFIERS, 1);
    league.set_discipline_rule(0);
    let edition = world_cup_edition((year as u16).wrapping_add(1) as i16); // cqz11: the next World Cup
    league.set_year(edition);
    league.set_importance(0xf);
    write_u16(this + league::ENTRANT_COUNT, 0);
    write_u16(this + league::ENTRANT_CAPACITY, 0x20);
    let book = exe::crt_calloc(0x20, 6);
    write_u32(this + league::ENTRANT_BOOK, book as u32);
    if book == 0 {
        show_error_at(0xad5888, 0x58);
        return this;
    }
    if exe::league_load_from_temp_file(league, 1) != 0 {
        return this;
    }
    covered!("cqz_11 world cup");
    league.set_league_flags(0x28);
    league.set_stage_index(stage_index::MAIN);
    write_u32(this + league::CURRENT_STAGE, 0xffff_ffff);
    write_u32(this + league::STAGE_COUNT, 8);
    write_u32(this + league::STAGES, exe::crt_malloc(0x20) as u32);
    let team_list: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x92da40usize);
    team_list(this);
    let rules: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x92d200usize);
    if rules(this) == 0 {
        show_error_at(0xad5888, 0x72);
        return this;
    }
    let p = exe::crt_operator_new(0x5ce);
    let e = if p != 0 { exe::new_comp_stats(p, league) } else { 0 };
    write_u32(this + league::COMP_STATS, e as u32);
    if e == 0 {
        show_error_at(0xad5888, 0x7a);
    }
    if league.year() as u16 == year as u16 { // cqz11: the World Cup of the game's own year has its 32
        write_u16(this + league::ENTRANT_COUNT, 0x20);
    } else {
        write_u32(this + league::ACTIVE, 0);
    }
    write_u32(this + league::TABLE, 0);
    write_u16(this + league::CLUB_COUNT, 0);
    this
}

/// Comparer for `construct_world_cup`: only the original runs (it allocates the book, the stages and the statistics),
/// then the plan (`compare_nations_tournament_ctor`: this, vtable, record, edition year and importance).
pub(crate) unsafe extern "thiscall" fn compare_construct_world_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_WORLD_CUP);
    compare_nations_tournament_ctor("construct_world_cup", League(this), record, 0x9712f0, world_cup_edition((year as u16).wrapping_add(1) as i16), || o(this, year, record))
}

/// The default World Cup field after the holder and the hosts, in entrant-book slots 3 to 31 (the keys of their
/// nations), written without status nor count.
const WORLD_CUP_DEFAULT_FIELD: [usize; 29] = [KEY_NATION_ITALY, KEY_NATION_SPAIN, KEY_NATION_ARGENTINA, KEY_NATION_ROMANIA, KEY_NATION_HOLLAND,
    KEY_NATION_NIGERIA, KEY_NATION_SOUTH_AFRICA, KEY_NATION_CAMEROON, KEY_NATION_TUNISIA, KEY_NATION_MOROCCO, KEY_NATION_MEXICO, KEY_NATION_JAMAICA,
    KEY_NATION_UNITED_STATES, KEY_NATION_ENGLAND, KEY_NATION_SCOTLAND, KEY_NATION_DENMARK, KEY_NATION_BULGARIA, KEY_NATION_YUGOSLAVIA,
    KEY_NATION_NORWAY, KEY_NATION_BELGIUM, KEY_NATION_CROATIA, KEY_NATION_AUSTRIA, KEY_NATION_COLOMBIA, KEY_NATION_PARAGUAY, KEY_NATION_CHILE,
    KEY_NATION_SOUTH_KOREA, KEY_NATION_JAPAN, KEY_NATION_SAUDI_ARABIA, KEY_NATION_IRAN];

/// The 2002 World Cup qualifiers from the CONCACAF and Africa, entered as qualified (status 6) after the holder and the
/// hosts: Costa Rica, two of the United States, Honduras and Mexico (drawn), then Cameroon, Nigeria, Senegal, Tunisia
/// and South Africa.
const WORLD_CUP_2002_AFRICA: [usize; 5] = [KEY_NATION_CAMEROON, KEY_NATION_NIGERIA, KEY_NATION_SENEGAL, KEY_NATION_TUNISIA, KEY_NATION_SOUTH_AFRICA];

/// The national team of the nation of key `key` (`exe::database_get_national_team` on `G_NATION_TABLE` +
/// `nation::SIZE` × [key]: push; add esp, 4 -> cdecl).
unsafe fn world_cup_team_of_key(key: usize) -> usize {
    exe::database_get_national_team(read_usize(G_NATION_TABLE).wrapping_add(read_usize(key).wrapping_mul(nation::SIZE)))
}

/// The entry `k` (6 bytes: +0 team, +4 status) of the World Cup entrant book.
unsafe fn world_cup_book_entry(this: usize, k: i16) -> usize {
    read_usize(this + league::ENTRANT_BOOK) + (k as isize * 6) as usize
}

/// Adds the national team of key `key` as qualified (status 6) at the entrant count: its team first, then, when the
/// count reached the capacity (signed words), the error box of line 0x456 and nothing; otherwise the status, the team
/// and the count up one.
unsafe fn world_cup_add_2002_qualifier(this: usize, key: usize) {
    let team = world_cup_team_of_key(key);
    let n = read_i16(this + league::ENTRANT_COUNT);
    if n >= read_i16(this + league::ENTRANT_CAPACITY) {
        show_error_at(0xad5888, 0x456);
        return;
    }
    write_u8(world_cup_book_entry(this, n) + 4, 6);
    write_usize(world_cup_book_entry(this, read_i16(this + league::ENTRANT_COUNT)), team);
    write_u16(this + league::ENTRANT_COUNT, read_u16(this + league::ENTRANT_COUNT).wrapping_add(1));
}

/// rule: the World Cup field: the holder first (status 2), then the hosts of the finals year (status 1; a host that is
/// also the holder does not count twice), then a fixed list of 29 nations in slots 3 to 31; for the 2002 finals the
/// CONCACAF and African qualifiers are entered as qualified (status 6) after the holder and the hosts: Costa Rica, two
/// of the United States, Honduras and Mexico (one in three United States and Honduras, else half and half United
/// States and Mexico or Honduras and Mexico), Cameroon, Nigeria, Senegal, Tunisia and South Africa
///
/// `league::ENTRANT_COUNT` = 0; entry 0's status = 2 and its team the last World Cup winner
/// (`exe::comp_util_get_latest_placed_club`(World Cup competition record, 0): 2 pushes; add esp -> cdecl); count 1. The
/// hosts of the finals year (HostCountry::find_hosts on [`G_HOST_COUNTRY`]: 5 pushes; mov ecx -> thiscall, ret 0x14;
/// the year with garbage in the high 16 bits, read as a word): with one or more, the entry at the count gets status 1 and the first host's national team, count up
/// one, and down one again when entry 1's team is entry 0's; with two or more, the same for the second host against
/// entry 2; with exactly one, entry 2's team is China's (no status, no count); with none, entries 1 and 2 get Iceland
/// and Hungary and the count is 3. Then slots 3 to 31 (+0x12 to +0xba) get `WORLD_CUP_DEFAULT_FIELD`. With the year
/// 2002, the 2002 qualifiers (`world_cup_add_2002_qualifier`, at the count, overwriting the default field): Costa Rica;
/// a draw of 3 (`exe::utils_random(3)`: push; add esp -> cdecl; the whole eax tested) giving 0, the United States and
/// Honduras, else a draw of 2: 0, the United States and Mexico, else Honduras and Mexico; then `WORLD_CUP_2002_AFRICA`.
/// Returns 1.
/// Original: `WorldCup::set_up_team_list` (0x92da40, world_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn world_cup_set_up_team_list(this: usize) -> u32 {
    let league = League(this);
    write_u16(this + league::ENTRANT_COUNT, 0);
    write_u8(read_usize(this + league::ENTRANT_BOOK) + 4, 2);
    let world_cup = read_usize(G_CLUB_COMP_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP).wrapping_mul(0x6b));
    let holder = exe::comp_util_get_latest_placed_club(world_cup, 0);
    write_usize(world_cup_book_entry(this, read_i16(this + league::ENTRANT_COUNT)), holder);
    write_u16(this + league::ENTRANT_COUNT, read_u16(this + league::ENTRANT_COUNT).wrapping_add(1));
    let (mut second, mut first) = (0i32, 0i32);
    let n = exe::hostcountry_find_hosts(read_usize(G_HOST_COUNTRY), read_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP), league.year() as u16 as u32, &mut first, &mut second, 1) as i8;
    if n > 0 {
        covered!("cqz_11 world cup hosts");
        for (k, host) in [(1usize, first), (2usize, second)].into_iter().take(if n > 1 { 2 } else { 1 }) {
            write_u8(world_cup_book_entry(this, read_i16(this + league::ENTRANT_COUNT)) + 4, 1);
            let team = exe::database_get_national_team(read_usize(G_NATION_TABLE).wrapping_add((host as usize).wrapping_mul(nation::SIZE)));
            write_usize(world_cup_book_entry(this, read_i16(this + league::ENTRANT_COUNT)), team);
            write_u16(this + league::ENTRANT_COUNT, read_u16(this + league::ENTRANT_COUNT).wrapping_add(1));
            let book = read_usize(this + league::ENTRANT_BOOK);
            if read_usize(book) == read_usize(book + k * 6) { // cqz11: a host that is the holder counts once
                write_u16(this + league::ENTRANT_COUNT, read_u16(this + league::ENTRANT_COUNT).wrapping_sub(1));
            }
        }
        if n == 1 {
            write_usize(read_usize(this + league::ENTRANT_BOOK) + 0xc, world_cup_team_of_key(KEY_NATION_CHINA));
        }
    } else {
        write_usize(read_usize(this + league::ENTRANT_BOOK) + 6, world_cup_team_of_key(KEY_NATION_ICELAND));
        write_usize(read_usize(this + league::ENTRANT_BOOK) + 0xc, world_cup_team_of_key(KEY_NATION_HUNGARY));
        write_u16(this + league::ENTRANT_COUNT, 3);
    }
    covered!("cqz_11 world cup field");
    for (k, &key) in WORLD_CUP_DEFAULT_FIELD.iter().enumerate() {
        write_usize(read_usize(this + league::ENTRANT_BOOK) + (k + 3) * 6, world_cup_team_of_key(key));
    }
    if league.year() != 0x7d2 {
        return 1;
    }
    covered!("cqz_11 world cup 2002 qualifiers");
    world_cup_add_2002_qualifier(this, KEY_NATION_COSTA_RICA);
    let pair = if exe::utils_random(3) == 0 { // cqz11: one in three, the United States and Honduras
        [KEY_NATION_UNITED_STATES, KEY_NATION_HONDURAS]
    } else if exe::utils_random(2) == 0 {
        [KEY_NATION_UNITED_STATES, KEY_NATION_MEXICO]
    } else {
        [KEY_NATION_HONDURAS, KEY_NATION_MEXICO]
    };
    for key in pair.into_iter().chain(WORLD_CUP_2002_AFRICA) {
        world_cup_add_2002_qualifier(this, key);
    }
    1
}

// simplification: the draw of the CONCACAF pair uses the game's random draws: it cannot run twice. Only the original; the plan:
// return value 1, the holder in entry 0 with status 2, the count within the capacity and, outside 2002, slots 3 to 31
// with the default field.
pub(crate) unsafe extern "thiscall" fn compare_world_cup_set_up_team_list(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WORLD_CUP_SET_UP_TEAM_LIST);
    let r = o(this);
    let book = read_usize(this + league::ENTRANT_BOOK);
    let year2002 = League(this).year() == 0x7d2;
    let ok = r == 1 && read_u8(book + 4) == 2 && read_i16(this + league::ENTRANT_COUNT) <= read_i16(this + league::ENTRANT_CAPACITY)
        && (year2002 || WORLD_CUP_DEFAULT_FIELD.iter().enumerate().all(|(k, &key)| read_usize(book + (k + 3) * 6) == world_cup_team_of_key(key)));
    SCOREBOARD.record(ok, || format!("world_cup_set_up_team_list({:#x}) returned {}, {} entrants", this, r, read_i16(this + league::ENTRANT_COUNT)));
    r
}

// ---------- cqz_12: the day of the South American and Asian World Cup qualifying and the Asian entrant list (wc_south_american_league.cpp, wc_asia_league.cpp) ----------

/// The day of a World Cup qualifying section, before its draw: (true, al 0) when today is before the draw date (today's
/// year lower, unsigned; or the same year and the day lower, signed), and whether today is the draw day.
unsafe fn qualifying_day_before_draw(this: League, today: usize) -> (bool, bool) {
    let (day, year) = oceania_nations_draw_date(this);
    let (today_day, today_year) = (read_u16(today), read_u16(today + 2));
    let before = today_year < year || (today_year == year && (today_day as i16) < day as i16);
    (before, today_year == year && today_day == day)
}

/// rule: the South American qualifying starts on its start date: with the full day update and the league not yet
/// started, the fixtures are made, the table sorted, the predictions set and every South American national team
/// leaves its continental competitions to play this one; before the start nothing runs, not even the league day
///
/// The draw date (`oceania_nations_draw_date`): before it returns 0 (al) without the league day. On it, with
/// `league::ACTIVE` 0 and `full`: the fixtures (`exe::league_create_fixtures`; 0: the error box of line 0x27b and 0),
/// the sorted table (`exe::league_sort_table(this, 0)`; 0: the box of line 0x282 and 0), `league::ACTIVE` = 1, the
/// predicted positions (the virtual `comp_vtable::SET_PREDICTED_POSITIONS`), the South American national teams released
/// from their continental competitions (NationalTeams::release_competitions_of_continent 0x775070 on
/// [`G_NATIONAL_TEAMS`] with the South American continent record — `G_CONTINENT_TABLE` + 0xc6 × the key — and 0: 2
/// pushes; mov ecx -> thiscall, ret 8) and each nation of the table rows (the club of row k, 0x3b bytes from
/// `league::TABLE`, its `club::NATION`; k word signed below `league::CLUB_COUNT`) entered in this competition
/// (`exe::nationalteams_set_current_competition`(national teams, nation, the record `league::COMPETITION`): 2 pushes;
/// mov ecx -> thiscall, ret 8). Then the league day with `full` (`League::update_for_date`), whose al it passes on.
/// Original: `WcSouthAmericanLeague::update_for_date` (0x921160, wc_south_american_league.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn wc_south_american_league_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let league = League(this);
    // the full eax is passed on (the exe:: wrapper reads only the byte)
    let league_day: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(FN_LEAGUE_UPDATE_FOR_DATE);
    covered!("cqz_12 south american day");
    let (before, draw_day) = qualifying_day_before_draw(league, today);
    // simplification: on the returns before the start and on the errors the original returns only al (0); the caller reads the byte
    if before {
        return 0;
    }
    if draw_day && league.active() == 0 && full != 0 {
        covered!("cqz_12 south american start");
        if exe::league_create_fixtures(league) == 0 {
            show_error_at(0xad4ce0, 0x27b);
            return 0;
        }
        if exe::league_sort_table(league, 0) == 0 {
            show_error_at(0xad4ce0, 0x282);
            return 0;
        }
        league.set_active(1);
        let predicted: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::SET_PREDICTED_POSITIONS));
        predicted(this);
        let release: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x775070usize);
        let continent = read_usize(G_CONTINENT_TABLE).wrapping_add(read_usize(KEY_CONTINENT_SOUTH_AMERICA).wrapping_mul(0xc6));
        release(read_usize(G_NATIONAL_TEAMS), continent, 0); // cqz12: the South American teams leave their continental competitions
        let mut k = 0i16;
        while k < read_i16(this + league::CLUB_COUNT) {
            let club = Club(read_usize(read_usize(this + league::TABLE) + (k as isize * 0x3b) as usize));
            exe::nationalteams_set_current_competition(read_usize(G_NATIONAL_TEAMS), club.nation(), read_usize(this + league::COMPETITION));
            k = k.wrapping_add(1);
        }
    }
    league_day(this, today, full) as u32
}

// simplification: the fixtures, the national teams' competitions and the league day cannot run twice. Only the original; the plan
// on the start day (the copy's condition): `league::ACTIVE` = 1.
pub(crate) unsafe extern "thiscall" fn compare_wc_south_american_league_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_WC_SOUTH_AMERICAN_LEAGUE_UPDATE_FOR_DATE);
    let league = League(this);
    let starts = full != 0 && league.active() == 0 && qualifying_day_before_draw(league, today).1;
    let r = o(this, today, full);
    if starts {
        SCOREBOARD.record(league.active() == 1, || format!("wc_south_american_league_update_for_date({:#x}): the start did not activate it", this));
    }
    r
}

/// rule: on the draw date of the Asian qualifying, with the full day update and the league not yet started, the first
/// round groups are drawn, the first group gets its table and fixtures and the nine other groups their stages; before
/// the draw nothing runs, not even the league day
///
/// The draw date (`oceania_nations_draw_date`): before it returns 0 (al) without the league day. On it, with
/// `league::ACTIVE` 0 and `full`: the draw (WcAsiaLeague::draw_first_round_groups 0x912bd0, thiscall, ret 0), the first
/// group table (WcAsiaLeague::init_first_group_table 0x912ad0; 0: the error box of line 0x40c and 0), the fixtures
/// (`exe::league_create_fixtures`; 0: the box of line 0x412 and 0), the sorted table (`exe::league_sort_table(this,
/// 0)`; 0: the box of line 0x419 and 0), the groups 0 to 8 (WcAsiaLeague::create_first_round_group 0x913540: push k;
/// mov ecx -> thiscall, ret 4), the predicted positions (the virtual `comp_vtable::SET_PREDICTED_POSITIONS`), the drawn
/// list +0xf7 freed (`exe::crt_free`: push; add esp -> cdecl) and zeroed and `league::ACTIVE` = 1. Then the league day
/// with `full`, whose al it passes on.
/// Original: `WcAsiaLeague::update_for_date` (0x913140, wc_asia_league.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn wc_asia_league_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let league = League(this);
    let league_day: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(FN_LEAGUE_UPDATE_FOR_DATE);
    covered!("cqz_12 asian day");
    let (before, draw_day) = qualifying_day_before_draw(league, today);
    // simplification: on the returns before the draw and on the errors the original returns only al (0); the caller reads the byte
    if before {
        return 0;
    }
    if draw_day && league.active() == 0 && full != 0 {
        covered!("cqz_12 asian draw");
        let draw: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x912bd0usize);
        let first_group: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x912ad0usize);
        let group: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x913540usize);
        draw(this);
        if first_group(this) == 0 {
            show_error_at(0xad4680, 0x40c);
            return 0;
        }
        if exe::league_create_fixtures(league) == 0 {
            show_error_at(0xad4680, 0x412);
            return 0;
        }
        if exe::league_sort_table(league, 0) == 0 {
            show_error_at(0xad4680, 0x419);
            return 0;
        }
        for k in 0..9 {
            group(this, k);
        }
        let predicted: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::SET_PREDICTED_POSITIONS));
        predicted(this);
        exe::crt_free(read_usize(this + 0xf7));
        write_u32(this + 0xf7, 0);
        league.set_active(1);
    }
    league_day(this, today, full) as u32
}

// simplification: the draw, the tables and the league day cannot run twice. Only the original; the plan on the draw day (the
// copy's condition): `league::ACTIVE` = 1, the list +0xf7 zeroed and the first group table built.
pub(crate) unsafe extern "thiscall" fn compare_wc_asia_league_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_WC_ASIA_LEAGUE_UPDATE_FOR_DATE);
    let league = League(this);
    let draws = full != 0 && league.active() == 0 && qualifying_day_before_draw(league, today).1;
    let r = o(this, today, full);
    if draws {
        let ok = league.active() == 1 && read_usize(this + 0xf7) == 0 && league.table() != 0;
        SCOREBOARD.record(ok, || format!("wc_asia_league_update_for_date({:#x}): the draw did not build the groups", this));
    }
    r
}

/// rule: the Asian World Cup qualifying entrants: 44 places minus the Asian nations exempt (the hosts of the next World
/// Cup and the last winner when Asian), filled with the other Asian national teams in database order, sorted by
/// reputation; at most 40 go on
///
/// +0xf3 (the entrant count, dword) = 0x2c, the exempt count +0xee = 0, `league::GAMES_PER_CLUB` = −1. Without
/// [`G_HOST_COUNTRY`]: the error box of line 0x5e1 and no hosts; otherwise the hosts of the World Cup two years later
/// (HostCountry::find_hosts: 5 pushes; mov ecx -> thiscall, ret 0x14; the year with garbage in the high 16 bits, read
/// as a word); for each (k signed byte below the count) up to the first −1, an Asian one (`KEY_CONTINENT_ASIA`) adds one
/// to +0xee. The last World Cup winner (`exe::comp_util_get_latest_placed_club`(World Cup record, 0)): its nation, with
/// a continent, Asian and neither host, adds one. +0xf3 = 0x2c − exempt (signed byte) and the list +0xef
/// (`exe::crt_malloc` of 4 × count: cdecl; out of memory, the box of line 0x610 and 0). Over the nations (i word
/// signed), each Asian one while the list (dword) is below the count: its national team (`exe::database_get_national_team`),
/// when the nation is neither host and the team is not the winner, goes into the list. Short of the count it returns 0
/// (unsorted); full, the list sorted by reputation (qsort `exe::crt_qsort`(list, n, 4, 0x4b07d0)), the count capped
/// at 40 (as a signed word) and 1.
/// Original: `WcAsiaLeague::build_entrant_list` (0x913f60, wc_asia_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_asia_league_build_entrant_list(this: usize) -> u32 {
    let league = League(this);
    let exempt = this + 0xee;
    write_u32(this + 0xf3, 0x2c);
    write_u8(exempt, 0);
    league.set_games_per_club(-1);
    let mut hosts = [-1i32; 2];
    if read_usize(G_HOST_COUNTRY) == 0 {
        show_error_at(0xad4680, 0x5e1);
    } else {
        let (a, b) = hosts.split_at_mut(1);
        let n = exe::hostcountry_find_hosts(read_usize(G_HOST_COUNTRY), read_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP), (league.year() as u16).wrapping_add(2) as u32, &mut a[0], &mut b[0], 1) as i8;
        let mut k = 0i8;
        // simplification: the original reads the host slot k of its two locals for any k below the count; find_hosts gives at most two
        while k < n && (k as usize) < hosts.len() {
            let id = hosts[k as usize];
            if id == -1 {
                break;
            }
            let c = Nation(read_usize(G_NATION_TABLE).wrapping_add((id as usize).wrapping_mul(nation::SIZE))).continent();
            if c != 0 && read_usize(c) == read_usize(KEY_CONTINENT_ASIA) {
                write_u8(exempt, read_u8(exempt).wrapping_add(1)); // cqz12: an Asian host is exempt
            }
            k += 1;
        }
    }
    let world_cup = read_usize(G_CLUB_COMP_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP).wrapping_mul(0x6b));
    let winner = exe::comp_util_get_latest_placed_club(world_cup, 0);
    if winner != 0 {
        let nation = Club(winner).nation();
        if !nation.is_null() {
            let c = nation.continent();
            let id = read_usize(nation.addr()) as i32;
            if c != 0 && read_usize(c) == read_usize(KEY_CONTINENT_ASIA) && id != hosts[0] && id != hosts[1] {
                write_u8(exempt, read_u8(exempt).wrapping_add(1));
            }
        }
    }
    let n = 0x2c - read_u8(exempt) as i8 as i32; // cqz12: 44 places minus the exempt
    write_u32(this + 0xf3, n as u32);
    let list = exe::crt_malloc((n as u32).wrapping_mul(4));
    write_u32(this + 0xef, list as u32);
    if list == 0 {
        show_error_at(0xad4680, 0x610);
        return 0;
    }
    covered!("cqz_12 asian entrants");
    let mut count = 0i32;
    let mut i = 0i16;
    while (i as i32) < read_usize(G_NATION_COUNT) as i32 {
        let nation = Nation(read_usize(G_NATION_TABLE).wrapping_add((i as isize as usize).wrapping_mul(nation::SIZE)));
        let c = nation.continent();
        if c != 0 && read_usize(c) == read_usize(KEY_CONTINENT_ASIA) && count < read_u32(this + 0xf3) as i32 {
            let team = exe::database_get_national_team(nation);
            let id = read_usize(nation.addr()) as i32;
            if id != hosts[0] && id != hosts[1] && team != winner {
                count += 1;
                write_usize(read_usize(this + 0xef) + (count as usize - 1) * 4, team);
            }
        }
        i = i.wrapping_add(1);
    }
    if count != read_u32(this + 0xf3) as i32 {
        return 0;
    }
    exe::crt_qsort(read_usize(this + 0xef), count as u32, 4, 0x4b07d0);
    let capped = (read_u32(this + 0xf3) as i32).min(0x28) as i16; // cqz12: at most 40 go on
    write_u32(this + 0xf3, capped as i32 as u32);
    1
}

// simplification: the list is allocated and kept: it cannot run twice. Only the original; the plan: return value 1 with the count
// at most 40 and every team of the list from Asia, or 0.
pub(crate) unsafe extern "thiscall" fn compare_wc_asia_league_build_entrant_list(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WC_ASIA_LEAGUE_BUILD_ENTRANT_LIST);
    let r = o(this);
    let n = read_u32(this + 0xf3) as i32;
    let list = read_usize(this + 0xef);
    let ok = r == 0 || (n <= 0x28 && list != 0 && (0..n.max(0) as usize).all(|k| {
        let nation = Club(read_usize(list + k * 4)).nation();
        let c = if nation.is_null() { 0 } else { nation.continent() };
        c != 0 && read_usize(c) == read_usize(KEY_CONTINENT_ASIA)
    }));
    SCOREBOARD.record(ok, || format!("wc_asia_league_build_entrant_list({:#x}) returned {} with {} entrants", this, r, n));
    r
}

// ---------- cqz_13: the European World Cup qualifying: the entry pool and the day (wc_europe_league.cpp) ----------

/// The European qualifying's exempt count (+0xf6, byte: the European hosts and holder) and its group format: the last
/// group-stage index, the runners-up stage and the play-off stage (+0xff, +0x100, +0x101), read by the round table.
const WC_EUROPE_EXEMPT: usize = 0xf6;
const WC_EUROPE_LAST_GROUP: usize = 0xff;

/// rule: the European World Cup qualifying entry pool: fifty places minus the European nations exempt (the hosts of the
/// next World Cup and the last winner when European), filled with the other European national teams in database order
/// and sorted by reputation; the exempt count sets the format: none, nine groups (stages up to 8) then the runners-up
/// (9) and the play-offs (10); one or two, eight groups (7, 8, 9); more, seven groups (6, 7, 8)
///
/// +0xf2 (the pool count, dword) = 0x32, `WC_EUROPE_EXEMPT` = 0, `league::GAMES_PER_CLUB` = −1. Without
/// [`G_HOST_COUNTRY`]: the error box of line 0x4c1 and no hosts; otherwise the hosts of the World Cup two years later
/// (HostCountry::find_hosts: 5 pushes; mov ecx -> thiscall, ret 0x14; the year with garbage in the high 16 bits, read as
/// a word); for each (k signed byte below the count) up to the first −1, a European one (`KEY_CONTINENT_EUROPE`) adds
/// one. The last World Cup winner (`exe::comp_util_get_latest_placed_club`(World Cup record, 0)): its nation, with a
/// continent, European and neither host, adds one. +0xf2 = 0x32 − exempt (signed byte); the old list +0xee freed
/// (`exe::crt_free`) when there is one, and a new one of fifty slots (`exe::crt_malloc(0xc8)`; out of memory, the box of
/// line 0x4f4 and 0). Over the nations (i word signed), each European one while the list is below fifty (not below the
/// count): its national team, when the nation is neither host and the team is not the winner, goes into the list. Below
/// the count (signed) it returns 0; otherwise the list sorted by reputation (qsort `exe::crt_qsort`(list, n, 4,
/// 0x4b07d0)), the format set and 1.
/// Original: `WcEuropeLeague::build_entry_pool` (0x919e30, wc_europe_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_build_entry_pool(this: usize) -> u32 {
    let league = League(this);
    let exempt = this + WC_EUROPE_EXEMPT;
    write_u32(this + 0xf2, 0x32);
    write_u8(exempt, 0);
    league.set_games_per_club(-1);
    let mut hosts = [-1i32; 2];
    if read_usize(G_HOST_COUNTRY) == 0 {
        show_error_at(0xad4a6c, 0x4c1);
    } else {
        let (a, b) = hosts.split_at_mut(1);
        let n = exe::hostcountry_find_hosts(read_usize(G_HOST_COUNTRY), read_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP), (league.year() as u16).wrapping_add(2) as u32, &mut a[0], &mut b[0], 1) as i8;
        let mut k = 0i8;
        // simplification: the original reads the host slot k of its two locals for any k below the count; find_hosts gives at most two
        while k < n && (k as usize) < hosts.len() {
            let id = hosts[k as usize];
            if id == -1 {
                break;
            }
            let c = Nation(read_usize(G_NATION_TABLE).wrapping_add((id as usize).wrapping_mul(nation::SIZE))).continent();
            if c != 0 && read_usize(c) == read_usize(KEY_CONTINENT_EUROPE) {
                write_u8(exempt, read_u8(exempt).wrapping_add(1));
            }
            k += 1;
        }
    }
    let world_cup = read_usize(G_CLUB_COMP_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP).wrapping_mul(0x6b));
    let winner = exe::comp_util_get_latest_placed_club(world_cup, 0);
    if winner != 0 {
        let nation = Club(winner).nation();
        if !nation.is_null() {
            let c = nation.continent();
            let id = read_usize(nation.addr()) as i32;
            if c != 0 && read_usize(c) == read_usize(KEY_CONTINENT_EUROPE) && id != hosts[0] && id != hosts[1] {
                write_u8(exempt, read_u8(exempt).wrapping_add(1));
            }
        }
    }
    write_u32(this + 0xf2, (0x32 - read_u8(exempt) as i8 as i32) as u32);
    if read_usize(this + 0xee) != 0 {
        exe::crt_free(read_usize(this + 0xee));
    }
    let list = exe::crt_malloc(0xc8);
    write_u32(this + 0xee, list as u32);
    if list == 0 {
        show_error_at(0xad4a6c, 0x4f4);
        return 0;
    }
    covered!("cqz_13 european pool");
    let mut count = 0i32;
    let mut i = 0i16;
    while (i as i32) < read_usize(G_NATION_COUNT) as i32 {
        let nation = Nation(read_usize(G_NATION_TABLE).wrapping_add((i as isize as usize).wrapping_mul(nation::SIZE)));
        let c = nation.continent();
        if c != 0 && read_usize(c) == read_usize(KEY_CONTINENT_EUROPE) && count < 0x32 {
            let team = exe::database_get_national_team(nation);
            let id = read_usize(nation.addr()) as i32;
            if id != hosts[0] && id != hosts[1] && team != winner {
                count += 1;
                write_usize(read_usize(this + 0xee) + (count as usize - 1) * 4, team);
            }
        }
        i = i.wrapping_add(1);
    }
    if count < read_u32(this + 0xf2) as i32 {
        return 0;
    }
    exe::crt_qsort(read_usize(this + 0xee), count as u32, 4, 0x4b07d0);
    let format: [u8; 3] = match read_u8(exempt) {
        0 => [8, 9, 0xa], // cqz13: nine groups without exempt nations
        1 | 2 => [7, 8, 9],
        _ => [6, 7, 8],
    };
    for (k, v) in format.into_iter().enumerate() {
        write_u8(this + WC_EUROPE_LAST_GROUP + k, v);
    }
    1
}

// simplification: the pool is allocated and kept: it cannot run twice. Only the original; the plan: return value 0, or 1 with
// the count at least the places, every team of the list from Europe and the format of the exempt count.
pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_build_entry_pool(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WC_EUROPE_LEAGUE_BUILD_ENTRY_POOL);
    let r = o(this);
    let n = read_u32(this + 0xf2) as i32;
    let list = read_usize(this + 0xee);
    let format = match read_u8(this + WC_EUROPE_EXEMPT) {
        0 => [8u8, 9, 0xa],
        1 | 2 => [7, 8, 9],
        _ => [6, 7, 8],
    };
    let ok = r == 0 || (list != 0 && (0..3).all(|k| read_u8(this + WC_EUROPE_LAST_GROUP + k) == format[k]) && (0..n.max(0) as usize).all(|k| {
        let nation = Club(read_usize(list + k * 4)).nation();
        let c = if nation.is_null() { 0 } else { nation.continent() };
        c != 0 && read_usize(c) == read_usize(KEY_CONTINENT_EUROPE)
    }));
    SCOREBOARD.record(ok, || format!("wc_europe_league_build_entry_pool({:#x}) returned {} with {} places", this, r, n));
    r
}

/// rule: on the draw date of the European qualifying, with the full day update and the league not yet started, the
/// groups are drawn, this group gets its table and fixtures, the other groups and the runners-up stage are built; on a
/// day with results the runners-up table is refreshed; before the draw nothing runs, not even the league day
///
/// The draw date (`oceania_nations_draw_date`): before it returns 0 (al) without the league day. On it, with
/// `league::ACTIVE` 0 and `full`: the draw (WcEuropeLeague::draw_groups 0x918ec0, thiscall, ret 0), this group's table
/// (WcEuropeLeague::init_own_group_table 0x9190d0; 0: the error box of line 0x5c7 and 0), the fixtures
/// (`exe::league_create_fixtures`; 0: the box of line 0x5cd and 0), the sorted table (`exe::league_sort_table(this,
/// 0)`; 0: the box of line 0x5d4 and 0); the groups 0 to n − 2 (WcEuropeLeague::create_group_stage 0x9196d0: push 6;
/// push 0; push g; mov ecx -> thiscall, ret 0xc), n by the exempt count (0: 10; 1 or 2: 9; more: 8); the drawn list
/// +0xf7 freed (`exe::crt_free`) and zeroed; the runners-up stage (WcEuropeLeague::create_runners_up_stage 0x91a140);
/// the predicted positions (the virtual `comp_vtable::SET_PREDICTED_POSITIONS`); the pool +0xee freed and zeroed, its
/// count +0xf2 (dword) zeroed and `league::ACTIVE` = 1. On any other day (the draw day too when active or without
/// `full`): when the game's date ([`G_CURRENT_DATE`], the two words) is `league::LAST_RESULT_DATE`, the runners-up table
/// (WcEuropeLeague::fill_runners_up_table 0x91a240). Then the league day with `full` (`League::update_for_date`), whose
/// al it passes on.
/// Original: `WcEuropeLeague::update_for_date` (0x91a500, wc_europe_league.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let league = League(this);
    let league_day: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(FN_LEAGUE_UPDATE_FOR_DATE);
    covered!("cqz_13 european day");
    let (day, year) = oceania_nations_draw_date(league);
    let (today_day, today_year) = (read_u16(today), read_u16(today + 2));
    // simplification: on the returns before the draw and on the errors the original returns only al (0); the caller reads the byte
    if today_year < year || (today_year == year && (today_day as i16) < day as i16) {
        return 0;
    }
    if today_year == year && today_day == day && league.active() == 0 && full != 0 {
        covered!("cqz_13 european draw");
        let draw: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x918ec0usize);
        let own_group: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x9190d0usize);
        let group: unsafe extern "thiscall" fn(usize, i32, usize, u32) -> u32 = core::mem::transmute(0x9196d0usize);
        let runners_up: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x91a140usize);
        draw(this);
        if own_group(this) == 0 {
            show_error_at(0xad4a6c, 0x5c7);
            return 0;
        }
        if exe::league_create_fixtures(league) == 0 {
            show_error_at(0xad4a6c, 0x5cd);
            return 0;
        }
        if exe::league_sort_table(league, 0) == 0 {
            show_error_at(0xad4a6c, 0x5d4);
            return 0;
        }
        let groups: i32 = match read_u8(this + WC_EUROPE_EXEMPT) {
            0 => 0xa,
            1 | 2 => 9,
            _ => 8,
        };
        for g in 0..groups - 1 {
            group(this, g, 0, 6);
        }
        exe::crt_free(read_usize(this + 0xf7));
        write_u32(this + 0xf7, 0);
        runners_up(this);
        let predicted: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::SET_PREDICTED_POSITIONS));
        predicted(this);
        exe::crt_free(read_usize(this + 0xee));
        write_u32(this + 0xee, 0);
        write_u32(this + 0xf2, 0);
        league.set_active(1);
    } else if read_u16(G_CURRENT_DATE + 2) == read_u16(this + league::LAST_RESULT_DATE + 2) && read_u16(G_CURRENT_DATE) == read_u16(this + league::LAST_RESULT_DATE) {
        let fill: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x91a240usize);
        fill(this);
    }
    league_day(this, today, full) as u32
}

// simplification: the draw, the stages, the runners-up table and the league day cannot run twice. Only the original; the plan on
// the draw day (the copy's condition): `league::ACTIVE` = 1 and the pool, its count and the list +0xf7 zeroed.
pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_WC_EUROPE_LEAGUE_UPDATE_FOR_DATE);
    let league = League(this);
    let draws = full != 0 && league.active() == 0 && {
        let (day, year) = oceania_nations_draw_date(league);
        read_u16(today) == day && read_u16(today + 2) == year
    };
    let r = o(this, today, full);
    if draws {
        let ok = league.active() == 1 && read_usize(this + 0xee) == 0 && read_u32(this + 0xf2) == 0 && read_usize(this + 0xf7) == 0;
        SCOREBOARD.record(ok, || format!("wc_europe_league_update_for_date({:#x}): the draw did not build the groups", this));
    }
    r
}

// ---------- cqz_14: the news text of a nation's Copa América fate (goldcup.cpp) ----------

/// The Copa América fate templates of the knockout (stage 2), by fate byte: 1 win, 2 runners-up, 3 third place, 4
/// fourth place, 0xff knocked out; any other, "through to next round" (0x987198).
fn copa_america_fate_template(fate: u8) -> usize {
    match fate {
        1 => 0x98732c, // cqz14: "win"
        2 => 0x9872c8,
        3 => 0x987264,
        4 => 0x987200,
        0xff => 0x98713c,
        _ => 0x987198,
    }
}

/// rule: the news text of a nation's Copa América fate: in the group stages (stage below 2) only the quarter-final
/// qualification (fate 6) has a text, "<team> qualify for quarter finals"; in the knockout (stage 2) the win, the
/// runners-up place, the third and fourth places, the elimination ("knocked out of") and any other fate ("through to
/// next round in") of the competition; from stage 3 on there is no text
///
/// The arguments: the club index `club_index` in [`G_CLUB_TABLE`] (`club::SIZE` each), the fate byte `fate`, the stage
/// byte `stage` (signed), four values passed on (`a3`..`a7`), `full_sentence` and the Text `text`.
///
/// Stage below 2: fate other than 6 writes nothing. With `full_sentence`, the shared qualification sentence 0x4b4590
/// (club_index, a3, the competition record `league::COMPETITION`, 6, full_sentence, text: 6 pushes; add esp, 0x18 ->
/// cdecl). Otherwise `langlib_sprintf` (`FN_LANGLIB_SPRINTF`, cdecl, variadic) into the shared buffer 0xde1f64 of
/// template 0x98737c with the club's gender (`club::SHORT_GENDER_NAME`, signed byte, twice) and short name
/// (`club::SHORT_NAME`), set as the text (`exe::text_set_string`: push; mov ecx, text -> thiscall, ret 4) with the
/// club's link (`exe::text_add_link`(text, short name, 0x7d5, club): 3 pushes -> thiscall, ret 0xc).
///
/// Stage 2: with `full_sentence`, the shared knockout sentence 0x4b0b80 (club_index, a4, a7, fate, a5, text: cdecl).
/// Otherwise the template of `copa_america_fate_template` with the club's gender twice, the competition's gender
/// (`competition::SHORT_GENDER_NAME`) twice, the club's and the competition's short names, set as the text, with the
/// club's link (0x7d5) and the competition's (`competition::SHORT_NAME`, 0x7d0, the record).
/// Original: `GoldCup::build_fate_news_text` (0x5e0f50, goldcup.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn goldcup_build_fate_news_text(this: usize, club_index: u32, fate: u32, stage: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32, full_sentence: u32, text: usize) {
    let _ = a6;
    let comp = read_usize(this + league::COMPETITION);
    let club = read_usize(G_CLUB_TABLE).wrapping_add((club_index as usize).wrapping_mul(club::SIZE));
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let club_gender = read_u8(club + club::SHORT_GENDER_NAME) as i8 as i32;
    let s = stage as u8 as i8;
    // simplification: on the paths without text the original leaves al with the stage or the fate byte; the callers do not read it
    if s < 2 {
        if fate as u8 != 6 {
            return;
        }
        covered!("cqz_14 copa america quarter finals");
        if full_sentence != 0 {
            let sentence: unsafe extern "C" fn(u32, u32, usize, u32, u32, usize) = core::mem::transmute(0x4b4590usize);
            sentence(club_index, a3, comp, 6, full_sentence, text);
            return;
        }
        sprintf(0xde1f64, 0x98737c, club_gender, club_gender, club + club::SHORT_NAME);
        exe::text_set_string(text, 0xde1f64 as *const u8);
        exe::text_add_link(text, (club + club::SHORT_NAME) as *const u8, 0x7d5, club);
        return;
    }
    if s != 2 {
        return;
    }
    covered!("cqz_14 copa america knockout fate");
    if full_sentence != 0 {
        let sentence: unsafe extern "C" fn(u32, u32, u32, u32, u32, usize) = core::mem::transmute(0x4b0b80usize);
        sentence(club_index, a4, a7, fate, a5, text);
        return;
    }
    let comp_gender = read_u8(comp + competition::SHORT_GENDER_NAME) as i8 as i32;
    sprintf(0xde1f64, copa_america_fate_template(fate as u8), club_gender, club_gender, comp_gender, comp_gender, club + club::SHORT_NAME, comp + competition::SHORT_NAME);
    exe::text_set_string(text, 0xde1f64 as *const u8);
    exe::text_add_link(text, (club + club::SHORT_NAME) as *const u8, 0x7d5, club);
    exe::text_add_link(text, (comp + competition::SHORT_NAME) as *const u8, 0x7d0, comp);
}

/// Comparer for `goldcup_build_fate_news_text`: side by side into two Texts (`compare_text_writer`).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_goldcup_build_fate_news_text(this: usize, club_index: u32, fate: u32, stage: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32, full_sentence: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize)>(IDX_GOLDCUP_BUILD_FATE_NEWS_TEXT);
    compare_text_writer(text, || o(this, club_index, fate, stage, a3, a4, a5, a6, a7, full_sentence, text),
        |t| goldcup_build_fate_news_text(this, club_index, fate, stage, a3, a4, a5, a6, a7, full_sentence, t),
        || format!("goldcup_build_fate_news_text(club {}, fate {}, stage {}, full {})", club_index, fate as u8, stage as u8 as i8, full_sentence));
}
