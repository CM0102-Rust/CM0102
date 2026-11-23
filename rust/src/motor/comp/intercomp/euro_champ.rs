use super::*;

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
/// value is not read); the exe::eurochamp_set_up_rules(EuroChamp::set_up_rules 0x57b630; 0: the error box of line 0x5f of euro_champ.cpp and
/// returns this); the statistics (operator new `exe::crt_operator_new(0x5ce)` and Comp_Stats::Comp_Stats
/// `exe::new_comp_stats(new, this)`: push this; mov ecx, new -> thiscall, ret 4) into `comp_stats`, null: the box of
/// line 0x66 (and it goes on). Then, when the edition is less than a year after the game's year (signed words), the
/// entrant count (`league::ENTRANT_COUNT`) = 16, else `league::ACTIVE` = 0 (with the edition in the same year the
/// active flag is left as it was); `league::CLUB_COUNT` = 0 and `league::TABLE` = null. Returns this.
/// Original: `EuroChamp::EuroChamp` (0x57a890, euro_champ.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_euro_champ(this: usize, year: u32, record: usize) -> usize {
    let this = League(this);
    let league = this;
    exe::new_league(league);
    write_u32(this.addr() + league::COMPETITION, record as u32);
    write_u32(this.addr(), 0x96a020);
    league.set_club_sides(4);
    this.set_upper_division_id(-1);
    this.set_lower_division_id((0xffff_ffff) as usize);
    this.set_entrants_are_qualifiers(1);
    league.set_discipline_rule(0);
    let edition = euro_champ_edition((year as u16).wrapping_add(1) as i16); // cqz10: the next European Championship
    league.set_year(edition);
    league.set_league_flags(0x28);
    league.set_importance(0xf);
    if exe::league_load_from_temp_file(league, 1) != 0 {
        return this.addr();
    }
    covered!("cqz_10 euro finals");
    league.set_stage_index(stage_index::MAIN);
    this.set_current_stage((0xffff_ffff) as usize);
    this.set_stage_count((4) as usize);
    this.set_stages((exe::crt_malloc(0x10) as u32) as usize);
    exe::eurochamp_set_up_team_list(this);
    if exe::eurochamp_set_up_rules(this) == 0 {
        show_error_at(0x9c4638, 0x5f);
        return this.addr();
    }
    let p = exe::crt_operator_new(0x5ce);
    let e = if p != 0 { exe::new_comp_stats(p, league) } else { 0 };
    this.set_comp_stats((e as u32) as usize);
    if e == 0 {
        show_error_at(0x9c4638, 0x66);
    }
    if (league.year() as i32) - (year as u16 as i16 as i32) < 1 { // cqz10: the finals of this year already have their 16
        this.set_entrant_count(0x10u16 as i16);
    } else {
        this.set_active((0) as usize);
    }
    this.set_club_count(0u16 as i16);
    this.set_table((0) as usize);
    this.addr()
}

/// Comparer for `construct_euro_champ`: only the original runs (it allocates the stages, the entrant book and the
/// statistics), then the plan (`compare_nations_tournament_ctor`: this, vtable, record, edition year and importance).
pub(crate) unsafe extern "thiscall" fn compare_construct_euro_champ(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_EURO_CHAMP);
    compare_nations_tournament_ctor("construct_euro_champ", League(this), record, 0x96a020, euro_champ_edition((year as u16).wrapping_add(1) as i16), || o(this, year, record))
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
    let this = League(this);
    let league = this;
    this.set_entrant_count(0u16 as i16);
    this.set_entrant_capacity(0x10u16 as i16);
    let book = exe::crt_calloc(0x10, 6);
    this.set_entrant_book((book as u32) as usize);
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
            let slot = this.entrant_book() + this.entrant_count() as isize as usize * 6;
            write_u8(slot + 4, 1);
            let team = national_team_of_id(h as usize);
            let slot = this.entrant_book() + this.entrant_count() as isize as usize * 6;
            write_usize(slot, team);
            this.set_entrant_count(((this.entrant_count() as u16).wrapping_add(1)) as i16);
        }
    } else {
        show_error_at(0x9c4638, 0x345);
    }
    covered!("cqz_10 euro field");
    for (k, &key) in EURO_CHAMP_DEFAULT_FIELD.iter().enumerate() {
        let team = national_team_of_id(read_usize(key));
        write_usize(this.entrant_book() + (k + 2) * 6, team); // cqz10: the default field in slots 2 to 15
    }
    1
}

pub(crate) unsafe extern "thiscall" fn compare_euro_champ_set_up_team_list(this: usize) -> u32 {
    let this = League(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_EURO_CHAMP_SET_UP_TEAM_LIST);
    let r = o(this.addr());
    let book = this.entrant_book();
    let ok = r == 0 || (book != 0 && (this.entrant_capacity() as u16) == 0x10 && (this.entrant_count() as u16) <= 2
        && EURO_CHAMP_DEFAULT_FIELD.iter().enumerate().all(|(k, &key)| read_usize(book + (k + 2) * 6) == national_team_of_id(read_usize(key))));
    SCOREBOARD.record(ok, || format!("euro_champ_set_up_team_list({:#x}) returned {}, {} entrants", this.addr(), r, (this.entrant_count() as u16)));
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
    if full == 0 {
        return exe::league_update_for_date(this, today, 0) as u32;
    }
    covered!("cqz_10 euro day");
    let seeding_day = read_u16(this + 0x13e) as i32 == read_i16(today) as i32;
    let seeding_year = (read_u16(this + 0x140) as i32).wrapping_add(league.year() as i32) == read_u16(today + 2) as i32;
    if seeding_day && seeding_year { // cqz10: the seeding day
        exe::eurochamp_seed_teams(this);
    }
    let (day, year) = oceania_nations_draw_date(league);
    if read_u16(today + 2) == year && read_u16(today) == day && league.table() == 0 {
        let draw: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_EUROCHAMP_RUN_GROUP_DRAW);
        draw(this);
    }
    exe::league_update_for_date(this, today, full) as u32
}

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

/// rule: the European Championship groups are single round robin, 3/1 points, tiebreak codes 4, 1 and 2, two qualifiers
/// per group, national teams, category 10, 7 substitutes and 3 changes; the seeding date is 10 January and the draw
/// date 10 February of the finals year (weekday 5).
///
/// Writes `EURO_CHAMP_RULE_FIELDS` (the win at `+0xc2`; the stage id `+0x3a` = 0x3e8 before the round table rewrites
/// it), Date::Date(0xa, 0, year, 5) (0x549f70: lea ecx, local; 4 pushes -> thiscall, ret 0x10; the year with garbage
/// in the high 16 bits, read as a word) into `EURO_CHAMP_SEEDING_DAY` and 0 into `EURO_CHAMP_SEEDING_YEAR_OFFSET`, then
/// Date::Date(0xa, 1, year, 5) into `.next_draw_day()`, `.next_draw_year_offset()` 0 and the stage −1 round table
/// (virtual +0x3c: push 0; push &+0x3a; push &+0xa9; push −1 -> thiscall, ret 0x10) into `.round_dates()`. Returns 1.
/// Original: `EuroChamp::set_up_rules` (0x57b630, euro_champ.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn euro_champ_set_up_rules(this: usize) -> u32 {
    covered!("cqz_2 euro rules");
    let win = 3; // cqz2: the win in the European Championship groups
    goldcup_write_rule_fields(this, &EURO_CHAMP_RULE_FIELDS, win);
    let seeding = 0; // cqz2: the seeding month of the European Championship (January)
    let word = competition_date_word(League(this), 0xa, seeding, 0, 5);
    write_u16(this + EURO_CHAMP_SEEDING_DAY, word);
    write_u16(this + EURO_CHAMP_SEEDING_YEAR_OFFSET, 0);
    let word = competition_date_word(League(this), 0xa, 1, 0, 5);
    finish_rules_setup(League(this), word, 0)
}

/// Comparer for `euro_champ_set_up_rules`: original only (virtual +0x3c allocates the round table), then the fields, both
/// dates and the return value.
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_EURO_CHAMP_SET_UP_RULES);
    let r = check_set_up_rules("euro_champ_set_up_rules", this, &EURO_CHAMP_RULE_FIELDS.iter().copied().filter(|f| f.0 != 0x3a).collect::<Vec<_>>(), (0xa, 1, 0, 5), 0, || o(this));
    let word = competition_date_word(League(this), 0xa, 0, 0, 5);
    let (day, offset) = (read_u16(this + EURO_CHAMP_SEEDING_DAY), read_u16(this + EURO_CHAMP_SEEDING_YEAR_OFFSET));
    SCOREBOARD.record(day == word && offset == 0, || format!("euro_champ_set_up_rules({:#x}): seeding date {:#x} expected {:#x}, offset {}", this, day, word, offset));
    r
}

/// rule: the European Championship has four groups (stages −1 to 2, ids 0x3e8 to 0x3eb) of three rounds on 9, 15 and
/// 19 June; the knockout (stage 3) in single matches: quarter-finals on 22/6, semi-finals on 26/6 and final on 30/6
/// (no third place); stages from 4 on have no table.
///
/// Built from the original with the round-table machine of the Copa América (`GoldcupScheduleOp`, `build_phase_table`):
/// each operation is a call of the original (league_round_set_date 0x68a160, league_round_set_match_slot 0x68a1c0,
/// cup_set_round_draw_date 0x521e60, cup_set_round_match_date 0x521eb0, cup_set_round_match_day 0x521f10: pushes; add esp
/// -> cdecl) or a byte/word/dword written into the table, in the original's order (months from 0 = January); the outputs
/// (`rounds`, `extra` always written, `league` only when not null), the size of the table and the error-box line when
/// out of memory are per stage (signed byte), every stage −128..127 checked against the original by emulation.
/// Original: `EuroChamp::create_schedule` (0x57ab90, euro_champ.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn euro_champ_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    build_phase_table(League(this), euro_champ_phase(stage), rounds, extra, league, 0x9c4638)
}

/// Comparer for `euro_champ_create_schedule`: side by side (`compare_phase_table`: the outputs and the bytes written to the two tables).
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_EURO_CHAMP_CREATE_SCHEDULE);
    compare_phase_table("euro_champ_create_schedule", stage, euro_champ_phase(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| euro_champ_create_schedule(this, stage, r, e, l))
}

/// The European Championship edition: from `start`, up one year at a time while the remainder of (year − 1992) by 4 is
/// not 0 (signed, as the original's `and 0x80000003` with its sign adjustment).
pub(crate) fn euro_champ_edition(start: i16) -> i16 {
    let mut year = start;
    while (year as i32 - 0x7c8) % 4 != 0 {
        year = year.wrapping_add(1);
    }
    year
}

/// The national team of the nation with id `id` (`exe::database_get_national_team` on `G_NATION_TABLE` + `nation::SIZE`
/// × id: push; add esp, 4 -> cdecl).
pub(crate) unsafe fn national_team_of_id(id: usize) -> usize {
    exe::database_get_national_team(read_usize(G_NATION_TABLE).wrapping_add(id.wrapping_mul(nation::SIZE)))
}

/// The round table of stage `stage` (signed byte) of cqz_7 euro round table, or none.
pub(crate) fn euro_champ_phase(stage: u32) -> Option<GoldcupStagePlan> {
    let f = stage as u8 as i8;
    match f {
        -128..=-2 => Some(GoldcupStagePlan { league: 1, rounds: 3, extra: (f as i16).wrapping_add(0x3e9), extra_check: false, size: 0xc3, row: 0xb4,
            ops: &EURO_CHAMP_OPS_1, ops2: &[], name: "cqz_7 euro round table" }),
        -1 => Some(GoldcupStagePlan { league: 1, rounds: 3, extra: 0x3e8, extra_check: false, size: 0xc3, row: 0xb4,
            ops: &EURO_CHAMP_OPS_2, ops2: &[], name: "cqz_7 euro round table" }),
        0 => Some(GoldcupStagePlan { league: 1, rounds: 3, extra: 0x3e9, extra_check: false, size: 0xc3, row: 0xb4,
            ops: &EURO_CHAMP_OPS_3, ops2: &[], name: "cqz_7 euro round table" }),
        1 => Some(GoldcupStagePlan { league: 1, rounds: 3, extra: 0x3ea, extra_check: false, size: 0xc3, row: 0xb4,
            ops: &EURO_CHAMP_OPS_4, ops2: &[], name: "cqz_7 euro round table" }),
        2 => Some(GoldcupStagePlan { league: 1, rounds: 3, extra: 0x3eb, extra_check: false, size: 0xc3, row: 0xb4,
            ops: &EURO_CHAMP_OPS_5, ops2: &[], name: "cqz_7 euro round table" }),
        3 => Some(GoldcupStagePlan { league: 0, rounds: 3, extra: 0, extra_check: false, size: 0x138, row: 0xf6,
            ops: &EURO_CHAMP_OPS_6, ops2: &[], name: "cqz_7 euro round table" }),
        4..=127 => None,
    }
}

/// The nations of the default European Championship field after the hosts, in entrant-book slots 2 to 15 (the keys of
/// their nations): Italy, Finland, Spain, Scotland, Romania, Germany, France, Wales, England, Denmark, Austria, Croatia,
/// Bulgaria and Norway.
pub(crate) const EURO_CHAMP_DEFAULT_FIELD: [usize; 14] = [KEY_NATION_ITALY, KEY_NATION_FINLAND, KEY_NATION_SPAIN, KEY_NATION_SCOTLAND, KEY_NATION_ROMANIA,
    KEY_NATION_GERMANY, KEY_NATION_FRANCE, KEY_NATION_WALES, KEY_NATION_ENGLAND, KEY_NATION_DENMARK, KEY_NATION_AUSTRIA, KEY_NATION_CROATIA,
    KEY_NATION_BULGARIA, KEY_NATION_NORWAY];

/// Day of year of the European Championship seeding date (the first word of the date), read by its daily update.
pub(crate) const EURO_CHAMP_SEEDING_DAY: usize = 0x13e;

/// Year of the European Championship seeding date relative to the finals year (+0x40).
pub(crate) const EURO_CHAMP_SEEDING_YEAR_OFFSET: usize = 0x140;

/// The rule fields of the European Championship finals (offset, width, value), in the original's write order: national
/// teams, 2 qualifiers per group, single round robin, 3/1 points, category 10, tiebreak codes 4, 1 and 2, the stage id
/// 0x3e8 and 7 substitutes.
pub(crate) const EURO_CHAMP_RULE_FIELDS: [(usize, u8, u32); 15] = [(0x43, 1, 2), (0xc7, 1, 2), (0xbe, 1, 2), (0xc2, 1, 3), (0x4a, 1, 3), (0x3c, 2, 1), (0xc3, 1, 1),
    (0xc4, 1, 0xa), (0xc5, 1, 4), (0xc6, 1, 1), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 0), (0x3a, 2, 0x3e8), (0x49, 1, 7)];

/// cqz_7 euro round table, the operations of round table 1 (from the original, in order).
pub(crate) const EURO_CHAMP_OPS_1: [GoldcupScheduleOp; 6] = [
    Cqp12D(0, 9, 5, 0, 6, 1, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 0xf, 5, 0, 5, 1, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 0x13, 5, 0, 2, 1, 1), Cqp12L(2, 2, -1, -1, -1, 0),
];

/// cqz_7 euro round table, the operations of round table 2 (from the original, in order).
pub(crate) const EURO_CHAMP_OPS_2: [GoldcupScheduleOp; 12] = [
    Cqp12D(0, 9, 5, 0, 6, 1, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 0xf, 5, 0, 5, 1, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 0x13, 5, 0, 2, 1, 1), Cqp12L(2, 2, -1, -1, -1, 0),
    Cqp12L(0, 0, 5, 2, 3, 4), Cqp12L(0, 1, 0, 2, 3, 0x80), Cqp12L(1, 0, 3, 2, 3, 4), Cqp12L(1, 1, 5, 1, 3, 0x80),
    Cqp12L(2, 0, 1, 1, 3, 4), Cqp12L(2, 1, 1, 1, 3, 0x80),
];

/// cqz_7 euro round table, the operations of round table 3 (from the original, in order).
pub(crate) const EURO_CHAMP_OPS_3: [GoldcupScheduleOp; 12] = [
    Cqp12D(0, 9, 5, 0, 6, 1, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 0xf, 5, 0, 5, 1, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 0x13, 5, 0, 2, 1, 1), Cqp12L(2, 2, -1, -1, -1, 0),
    Cqp12L(0, 0, 6, 1, 3, 0x40), Cqp12L(0, 1, 0, 1, 3, 0x1000), Cqp12L(1, 0, 3, 1, 3, 0x40),
    Cqp12L(1, 1, 5, 2, 3, 0x1000), Cqp12L(2, 0, 1, 2, 3, 0x1000), Cqp12L(2, 1, 1, 2, 3, 0x40),
];

/// cqz_7 euro round table, the operations of round table 4 (from the original, in order).
pub(crate) const EURO_CHAMP_OPS_4: [GoldcupScheduleOp; 12] = [
    Cqp12D(0, 9, 5, 0, 6, 1, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 0xf, 5, 0, 5, 1, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 0x13, 5, 0, 2, 1, 1), Cqp12L(2, 2, -1, -1, -1, 0),
    Cqp12L(0, 0, 6, 2, 3, 0x100), Cqp12L(0, 1, 1, 1, 3, 0x400), Cqp12L(1, 0, 4, 1, 3, 0x100),
    Cqp12L(1, 1, 6, 1, 3, 0x400), Cqp12L(2, 0, 2, 2, 3, 0x400), Cqp12L(2, 1, 2, 2, 3, 0x100),
];

/// cqz_7 euro round table, the operations of round table 5 (from the original, in order).
pub(crate) const EURO_CHAMP_OPS_5: [GoldcupScheduleOp; 12] = [
    Cqp12D(0, 9, 5, 0, 6, 1, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 0xf, 5, 0, 5, 1, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 0x13, 5, 0, 2, 1, 1), Cqp12L(2, 2, -1, -1, -1, 0),
    Cqp12L(0, 0, 6, 1, 3, 0x200), Cqp12L(0, 1, 1, 2, 3, 0x800), Cqp12L(1, 0, 4, 2, 3, 0x200),
    Cqp12L(1, 1, 6, 2, 3, 0x800), Cqp12L(2, 0, 2, 1, 3, 0x200), Cqp12L(2, 1, 2, 1, 3, 0x800),
];

/// cqz_7 euro round table, the operations of round table 6 (from the original, in order).
pub(crate) const EURO_CHAMP_OPS_6: [GoldcupScheduleOp; 62] = [
    Cqp12S(0, 0x14, 5, 0, 3), Cqp12J(0, 0x16, 5, 0, 5, 1, 1), Cqp12E(0, 0, 5, 1, 3, 0x40), Cqp12E(0, 1, 5, 2, 3, 4),
    Cqp12E(0, 2, 6, 2, 3, 0x80), Cqp12E(0, 3, 6, 1, 3, 0x100), Cqp12E(0, 4, -1, -1, -1, 0), Cqp12W(0x7, 0x78),
    Cqp12W(0x9, 0x0), Cqp12W(0xb, 0x0), Cqp12W(0xd, 0x383), Cqp12B(0x17, 0xa), Cqp12W(0x18, 0x8), Cqp12W(0x1a, 0x4),
    Cqp12W(0x1c, 0x8), Cqp12W(0xf, 0x0), Cqp12W(0x1e, 0x0), Cqp12B(0x20, 0x0), Cqp12B(0x21, 0x1), Cqp12B(0x22, 0x0),
    Cqp12X(0x5c, 0x0), Cqp12X(0x60, 0x0), Cqp12X(0x64, 0x0), Cqp12S(1, 0x18, 5, 0, 0), Cqp12J(1, 0x1a, 5, 0, 2, 1, 1),
    Cqp12E(1, 0, 2, 2, 3, 0x40), Cqp12E(1, 1, 2, 1, 3, 0x80), Cqp12E(1, 2, -1, -1, -1, 0), Cqp12W(0x6f, 0x82),
    Cqp12W(0x71, 0x1), Cqp12W(0x73, 0x0), Cqp12W(0x75, 0x383), Cqp12B(0x7f, 0xa), Cqp12W(0x80, 0x4),
    Cqp12W(0x82, 0x2), Cqp12W(0x84, 0x0), Cqp12W(0x77, 0x0), Cqp12W(0x86, 0x0), Cqp12B(0x88, 0x0), Cqp12B(0x89, 0x1),
    Cqp12B(0x8a, 0x0), Cqp12X(0xc4, 0x0), Cqp12X(0xc8, 0x0), Cqp12X(0xcc, 0x0), Cqp12S(2, 0x1b, 5, 0, 3),
    Cqp12J(2, 0x1e, 5, 0, 6, 1, 4), Cqp12W(0xd7, 0x96), Cqp12W(0xd9, 0x2), Cqp12W(0xdb, 0x0), Cqp12W(0xdd, 0x283),
    Cqp12B(0xe7, 0xa), Cqp12W(0xe8, 0x2), Cqp12W(0xea, 0x1), Cqp12W(0xec, 0x0), Cqp12W(0xdf, 0x0), Cqp12W(0xee, 0x0),
    Cqp12B(0xf0, 0x0), Cqp12B(0xf1, 0x1), Cqp12B(0xf2, 0x0), Cqp12X(0x12c, 0x0), Cqp12X(0x130, 0x0),
    Cqp12X(0x134, 0x0),
];
