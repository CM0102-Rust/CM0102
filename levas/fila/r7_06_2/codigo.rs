// ---------- r7_06_2: the European 2002 World Cup qualifying campaign (wc_europe_league.cpp) ----------

/// One fixed 2002 European match: (stage, day, month from 0, year, home nation key, home goals, away nation key, away
/// goals); stage −1 the section's own group; goals −1: not played.
pub(crate) type Fixed2002Match = (i8, u8, u8, u16, usize, i8, usize, i8);

/// WcEuropeLeague::add_fixed_result (thiscall, ret 0x20; hooked), called by the campaign load.
pub(crate) const FN_WC_EUROPE_LEAGUE_ADD_FIXED_RESULT: usize = 0x91db60;

/// rule: the European qualifying of a game whose year has reached the campaign year loads the real 2002 campaign: the
/// nine real groups (the section's own and eight group stages, six nations in the first five and five in the last
/// four), then the 230 matches of September 2000 to October 2001 with the scores known when the game was made
///
/// One exempt nation (`WC_EUROPE_EXEMPT` 1). The entry pool (+0xee, freed with `exe::crt_free` when set) made again for
/// 50 national teams (+0xf2 the count: 0x32 before the allocation, 0 after it; `exe::crt_malloc`(0xc8); none: the error
/// box of line 0x8ea and return). For each group of `WC_EUROPE_2002_GROUPS` in order, each nation (its id read from the
/// key) gives its national team (`national_team_of_id`) to the group's list, and asked again, to the pool (+0xee
/// [+0xf2], then +0xf2 + 1). The first group is the section's own table: `club_count` the group's size, the table
/// (`exe::crt_malloc` of `league_table_row::SIZE` × the size; none: the box of line 0x906 and return) and its rows
/// (`exe::league_init_table_row`(row, team, 0, 0)); each other group k (1..=8) is the group stage k − 1
/// (`exe::wceuropeleague_create_group_stage`(k − 1, list, size)). Then the runners-up stage
/// (`exe::wceuropeleague_create_runners_up_stage`), the predicted positions (`comp_vtable::SET_PREDICTED_POSITIONS`),
/// the pool freed and cleared (+0xee and +0xf2 0), the section active (`set_active(1)`), and each match of
/// `WC_EUROPE_2002_RESULTS` in order through WcEuropeLeague::add_fixed_result (the nation ids read from the keys at each
/// call; goals −1 pushed as −1). The original passes the row (`init_table_row`) and the size (`create_group_stage`)
/// with stack leftovers in the high bytes, which they do not read: the copy passes clean values.
/// Only the constructor calls it, when the game's year is not before the campaign year, never before 2004: with the
/// game starting in 2001 it never runs.
/// Defects of the original: dead code in the game as it is (the 2004 floor of the campaign year); with a start year
/// patched to one after a campaign year (e.g. 2005), the 2000-2001 groups and matches replace the drawn groups of a
/// later campaign, and the 23 matches without a score stay in the calendar, in the past, never played; the table's
/// error box (line 0x906) returns without freeing the pool.
/// Original: `WcEuropeLeague::load_2002_campaign` (0x91b760, wc_europe_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_load_2002_campaign(this: usize) {
    let league = League(this);
    let cpp = wc_cup_turnover_consts::WC_EUROPE_LEAGUE_CPP;
    write_u8(this + WC_EUROPE_EXEMPT, 1);
    if read_usize(this + 0xee) != 0 {
        exe::crt_free(read_usize(this + 0xee)); // the entry pool
    }
    write_u32(this + 0xf2, 0x32); // its count
    let pool = exe::crt_malloc(0xc8);
    write_usize(this + 0xee, pool);
    if pool == 0 {
        show_error_at(cpp, 0x8ea);
        return;
    }
    write_u32(this + 0xf2, 0);
    covered!("r7_06_2 european 2002 campaign");
    let mut teams = [0usize; 6];
    for (group, nations) in WC_EUROPE_2002_GROUPS.iter().enumerate() {
        for (k, &key) in nations.iter().enumerate() {
            let id = read_usize(key);
            teams[k] = national_team_of_id(id);
            let count = read_u32(this + 0xf2);
            write_usize(read_usize(this + 0xee) + count as usize * 4, national_team_of_id(id)); // the original asks again
            write_u32(this + 0xf2, count + 1);
        }
        let size = nations.len();
        if group == 0 {
            league.set_club_count(size as i16); // r7_06_2: the own group's size
            let table = exe::crt_malloc((size * league_table_row::SIZE) as u32);
            league.set_table(table);
            if table == 0 {
                show_error_at(cpp, 0x906);
                return;
            }
            let mut row = 0usize;
            while (row as i16) < league.club_count() {
                exe::league_init_table_row(this, row as u32, teams[row], 0, 0);
                row += 1;
            }
        } else {
            exe::wceuropeleague_create_group_stage(this, group as i32 - 1, teams.as_ptr() as usize, size as u32); // r7_06_2: group stage k - 1
        }
    }
    exe::wceuropeleague_create_runners_up_stage(this);
    let predicted: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(vtable_method(league, comp_vtable::SET_PREDICTED_POSITIONS));
    predicted(this);
    exe::crt_free(read_usize(this + 0xee));
    write_u32(this + 0xee, 0);
    write_u32(this + 0xf2, 0);
    league.set_active(1);
    let add: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32) = core::mem::transmute(FN_WC_EUROPE_LEAGUE_ADD_FIXED_RESULT);
    for &(stage, day, month, year, home, home_goals, away, away_goals) in WC_EUROPE_2002_RESULTS.iter() {
        add(this, stage as i32 as u32, day as u32, month as u32, year as u32, read_u32(home), home_goals as i32 as u32, read_u32(away), away_goals as i32 as u32);
    }
}

/// Comparer for `wc_europe_league_load_2002_campaign`: it allocates and builds the stages and the calendar (it cannot run
/// twice), so only the original runs; then the plan: the section active, the pool freed and cleared, one exempt nation
/// and six nations in the own table. It never runs in the game as it is; the copy was checked by emulation.
pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_load_2002_campaign(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this);
    let league = League(this);
    let ok = league.active() == 1 && read_usize(this + 0xee) == 0 && read_u32(this + 0xf2) == 0 && read_u8(this + WC_EUROPE_EXEMPT) == 1 && league.club_count() == 6;
    SCOREBOARD.record(ok, || format!("wc_europe_league_load_2002_campaign({:#x}): active {}, pool {:#x}/{}, clubs {}", this, league.active(), read_usize(this + 0xee),
        read_u32(this + 0xf2), league.club_count()));
}

/// The 230 European matches of the 2002 campaign (2 September 2000 to 7 October 2001), in the original's order:
/// (stage: −1 the section's own group, 0..=7 the other eight; day; month from 0; year; home nation key; its goals;
/// away nation key; its goals). Goals −1: the last round (6 and 7 October 2001), not played yet when the data was made.
pub(crate) const WC_EUROPE_2002_RESULTS: [Fixed2002Match; 230] = [
    (-1, 2, 8, 2000, KEY_NATION_SWITZERLAND, 0, KEY_NATION_RUSSIA, 1), // r7_06_2: Switzerland-Russia
    (-1, 2, 8, 2000, KEY_NATION_FAROE_ISLANDS, 2, KEY_NATION_SLOVENIA, 2),
    (-1, 2, 8, 2000, KEY_NATION_LUXEMBOURG, 0, KEY_NATION_YUGOSLAVIA, 2),
    (-1, 7, 9, 2000, KEY_NATION_LUXEMBOURG, 1, KEY_NATION_SLOVENIA, 2),
    (-1, 7, 9, 2000, KEY_NATION_SWITZERLAND, 5, KEY_NATION_FAROE_ISLANDS, 1),
    (-1, 11, 9, 2000, KEY_NATION_RUSSIA, 3, KEY_NATION_LUXEMBOURG, 0),
    (-1, 11, 9, 2000, KEY_NATION_SLOVENIA, 2, KEY_NATION_SWITZERLAND, 2),
    (-1, 24, 2, 2001, KEY_NATION_YUGOSLAVIA, 1, KEY_NATION_SWITZERLAND, 1),
    (-1, 24, 2, 2001, KEY_NATION_RUSSIA, 1, KEY_NATION_SLOVENIA, 1),
    (-1, 24, 2, 2001, KEY_NATION_LUXEMBOURG, 0, KEY_NATION_FAROE_ISLANDS, 2),
    (-1, 28, 2, 2001, KEY_NATION_RUSSIA, 1, KEY_NATION_FAROE_ISLANDS, 0),
    (-1, 28, 2, 2001, KEY_NATION_SWITZERLAND, 5, KEY_NATION_LUXEMBOURG, 0),
    (-1, 28, 2, 2001, KEY_NATION_SLOVENIA, 1, KEY_NATION_YUGOSLAVIA, 1),
    (-1, 25, 3, 2001, KEY_NATION_YUGOSLAVIA, 0, KEY_NATION_RUSSIA, 1),
    (-1, 2, 5, 2001, KEY_NATION_RUSSIA, 1, KEY_NATION_YUGOSLAVIA, 1),
    (-1, 2, 5, 2001, KEY_NATION_FAROE_ISLANDS, 0, KEY_NATION_SWITZERLAND, 1),
    (-1, 2, 5, 2001, KEY_NATION_SLOVENIA, 2, KEY_NATION_LUXEMBOURG, 0),
    (-1, 6, 5, 2001, KEY_NATION_SWITZERLAND, 0, KEY_NATION_SLOVENIA, 1),
    (-1, 6, 5, 2001, KEY_NATION_LUXEMBOURG, 1, KEY_NATION_RUSSIA, 2),
    (-1, 6, 5, 2001, KEY_NATION_FAROE_ISLANDS, 0, KEY_NATION_YUGOSLAVIA, 6),
    (-1, 15, 7, 2001, KEY_NATION_YUGOSLAVIA, 2, KEY_NATION_FAROE_ISLANDS, 0),
    (-1, 1, 8, 2001, KEY_NATION_SWITZERLAND, 1, KEY_NATION_YUGOSLAVIA, 2),
    (-1, 1, 8, 2001, KEY_NATION_SLOVENIA, 2, KEY_NATION_RUSSIA, 1),
    (-1, 1, 8, 2001, KEY_NATION_FAROE_ISLANDS, 1, KEY_NATION_LUXEMBOURG, 0),
    (-1, 5, 8, 2001, KEY_NATION_YUGOSLAVIA, 1, KEY_NATION_SLOVENIA, 1),
    (-1, 5, 8, 2001, KEY_NATION_LUXEMBOURG, 0, KEY_NATION_SWITZERLAND, 3),
    (-1, 5, 8, 2001, KEY_NATION_FAROE_ISLANDS, 0, KEY_NATION_RUSSIA, 3),
    (-1, 6, 9, 2001, KEY_NATION_YUGOSLAVIA, -1, KEY_NATION_LUXEMBOURG, -1),
    (-1, 6, 9, 2001, KEY_NATION_RUSSIA, -1, KEY_NATION_SWITZERLAND, -1),
    (-1, 6, 9, 2001, KEY_NATION_SLOVENIA, -1, KEY_NATION_FAROE_ISLANDS, -1),
    (0, 16, 7, 2000, KEY_NATION_ESTONIA, 1, KEY_NATION_ANDORRA, 0),
    (0, 2, 8, 2000, KEY_NATION_ANDORRA, 2, KEY_NATION_CYPRUS, 3),
    (0, 2, 8, 2000, KEY_NATION_HOLLAND, 2, KEY_NATION_REPUBLIC_OF_IRELAND, 2),
    (0, 3, 8, 2000, KEY_NATION_ESTONIA, 1, KEY_NATION_PORTUGAL, 3),
    (0, 7, 9, 2000, KEY_NATION_PORTUGAL, 1, KEY_NATION_REPUBLIC_OF_IRELAND, 1),
    (0, 7, 9, 2000, KEY_NATION_ANDORRA, 1, KEY_NATION_ESTONIA, 2),
    (0, 7, 9, 2000, KEY_NATION_CYPRUS, 0, KEY_NATION_HOLLAND, 4),
    (0, 11, 9, 2000, KEY_NATION_REPUBLIC_OF_IRELAND, 2, KEY_NATION_ESTONIA, 0),
    (0, 11, 9, 2000, KEY_NATION_HOLLAND, 0, KEY_NATION_PORTUGAL, 2),
    (0, 15, 10, 2000, KEY_NATION_CYPRUS, 5, KEY_NATION_ANDORRA, 0),
    (0, 14, 1, 2001, KEY_NATION_PORTUGAL, 3, KEY_NATION_ANDORRA, 0),
    (0, 24, 2, 2001, KEY_NATION_ANDORRA, 0, KEY_NATION_HOLLAND, 5),
    (0, 24, 2, 2001, KEY_NATION_CYPRUS, 0, KEY_NATION_REPUBLIC_OF_IRELAND, 4),
    (0, 28, 2, 2001, KEY_NATION_PORTUGAL, 2, KEY_NATION_HOLLAND, 2),
    (0, 28, 2, 2001, KEY_NATION_CYPRUS, 2, KEY_NATION_ESTONIA, 2),
    (0, 28, 2, 2001, KEY_NATION_ANDORRA, 0, KEY_NATION_REPUBLIC_OF_IRELAND, 3),
    (0, 25, 3, 2001, KEY_NATION_HOLLAND, 4, KEY_NATION_CYPRUS, 0),
    (0, 25, 3, 2001, KEY_NATION_REPUBLIC_OF_IRELAND, 3, KEY_NATION_ANDORRA, 1),
    (0, 2, 5, 2001, KEY_NATION_REPUBLIC_OF_IRELAND, 1, KEY_NATION_PORTUGAL, 1),
    (0, 2, 5, 2001, KEY_NATION_ESTONIA, 2, KEY_NATION_HOLLAND, 4),
    (0, 6, 5, 2001, KEY_NATION_ESTONIA, 0, KEY_NATION_REPUBLIC_OF_IRELAND, 2),
    (0, 6, 5, 2001, KEY_NATION_PORTUGAL, 6, KEY_NATION_CYPRUS, 0),
    (0, 15, 7, 2001, KEY_NATION_ESTONIA, 2, KEY_NATION_CYPRUS, 2),
    (0, 1, 8, 2001, KEY_NATION_ANDORRA, 1, KEY_NATION_PORTUGAL, 7),
    (0, 1, 8, 2001, KEY_NATION_REPUBLIC_OF_IRELAND, 1, KEY_NATION_HOLLAND, 0),
    (0, 5, 8, 2001, KEY_NATION_HOLLAND, 5, KEY_NATION_ESTONIA, 0),
    (0, 5, 8, 2001, KEY_NATION_CYPRUS, 1, KEY_NATION_PORTUGAL, 3),
    (0, 6, 9, 2001, KEY_NATION_HOLLAND, -1, KEY_NATION_ANDORRA, -1),
    (0, 6, 9, 2001, KEY_NATION_PORTUGAL, -1, KEY_NATION_ESTONIA, -1),
    (0, 6, 9, 2001, KEY_NATION_REPUBLIC_OF_IRELAND, -1, KEY_NATION_CYPRUS, -1),
    (1, 2, 8, 2000, KEY_NATION_ICELAND, 1, KEY_NATION_DENMARK, 2),
    (1, 2, 8, 2000, KEY_NATION_BULGARIA, 0, KEY_NATION_CZECH_REPUBLIC, 1),
    (1, 2, 8, 2000, KEY_NATION_NORTHERN_IRELAND, 1, KEY_NATION_MALTA, 0),
    (1, 7, 9, 2000, KEY_NATION_BULGARIA, 3, KEY_NATION_MALTA, 0),
    (1, 7, 9, 2000, KEY_NATION_NORTHERN_IRELAND, 1, KEY_NATION_DENMARK, 1),
    (1, 7, 9, 2000, KEY_NATION_CZECH_REPUBLIC, 4, KEY_NATION_ICELAND, 0),
    (1, 11, 9, 2000, KEY_NATION_ICELAND, 1, KEY_NATION_NORTHERN_IRELAND, 0),
    (1, 11, 9, 2000, KEY_NATION_DENMARK, 1, KEY_NATION_BULGARIA, 1),
    (1, 11, 9, 2000, KEY_NATION_MALTA, 0, KEY_NATION_CZECH_REPUBLIC, 0),
    (1, 24, 2, 2001, KEY_NATION_NORTHERN_IRELAND, 0, KEY_NATION_CZECH_REPUBLIC, 1),
    (1, 25, 2, 2001, KEY_NATION_BULGARIA, 2, KEY_NATION_ICELAND, 1),
    (1, 25, 2, 2001, KEY_NATION_MALTA, 0, KEY_NATION_DENMARK, 5),
    (1, 28, 2, 2001, KEY_NATION_CZECH_REPUBLIC, 0, KEY_NATION_DENMARK, 0),
    (1, 28, 2, 2001, KEY_NATION_BULGARIA, 4, KEY_NATION_NORTHERN_IRELAND, 3),
    (1, 25, 3, 2001, KEY_NATION_MALTA, 1, KEY_NATION_ICELAND, 4),
    (1, 2, 5, 2001, KEY_NATION_ICELAND, 3, KEY_NATION_MALTA, 0),
    (1, 2, 5, 2001, KEY_NATION_NORTHERN_IRELAND, 0, KEY_NATION_BULGARIA, 1),
    (1, 2, 5, 2001, KEY_NATION_DENMARK, 2, KEY_NATION_CZECH_REPUBLIC, 1),
    (1, 6, 5, 2001, KEY_NATION_ICELAND, 1, KEY_NATION_BULGARIA, 1),
    (1, 6, 5, 2001, KEY_NATION_DENMARK, 2, KEY_NATION_MALTA, 1),
    (1, 6, 5, 2001, KEY_NATION_CZECH_REPUBLIC, 3, KEY_NATION_NORTHERN_IRELAND, 1),
    (1, 1, 8, 2001, KEY_NATION_ICELAND, 3, KEY_NATION_CZECH_REPUBLIC, 1),
    (1, 1, 8, 2001, KEY_NATION_MALTA, 0, KEY_NATION_BULGARIA, 2),
    (1, 1, 8, 2001, KEY_NATION_DENMARK, 1, KEY_NATION_NORTHERN_IRELAND, 1),
    (1, 5, 8, 2001, KEY_NATION_CZECH_REPUBLIC, 3, KEY_NATION_MALTA, 2),
    (1, 5, 8, 2001, KEY_NATION_BULGARIA, 0, KEY_NATION_DENMARK, 2),
    (1, 5, 8, 2001, KEY_NATION_NORTHERN_IRELAND, 3, KEY_NATION_ICELAND, 0),
    (1, 6, 9, 2001, KEY_NATION_CZECH_REPUBLIC, -1, KEY_NATION_BULGARIA, -1),
    (1, 6, 9, 2001, KEY_NATION_DENMARK, -1, KEY_NATION_ICELAND, -1),
    (1, 6, 9, 2001, KEY_NATION_MALTA, -1, KEY_NATION_NORTHERN_IRELAND, -1),
    (2, 2, 8, 2000, KEY_NATION_TURKEY, 2, KEY_NATION_MOLDOVA, 0),
    (2, 2, 8, 2000, KEY_NATION_AZERBAIJAN, 0, KEY_NATION_SWEDEN, 1),
    (2, 2, 8, 2000, KEY_NATION_SLOVAKIA, 2, KEY_NATION_FYR_OF_MACEDONIA, 0),
    (2, 7, 9, 2000, KEY_NATION_FYR_OF_MACEDONIA, 3, KEY_NATION_AZERBAIJAN, 0),
    (2, 7, 9, 2000, KEY_NATION_SWEDEN, 1, KEY_NATION_TURKEY, 1),
    (2, 7, 9, 2000, KEY_NATION_MOLDOVA, 0, KEY_NATION_SLOVAKIA, 1),
    (2, 11, 9, 2000, KEY_NATION_SLOVAKIA, 0, KEY_NATION_SWEDEN, 0),
    (2, 11, 9, 2000, KEY_NATION_AZERBAIJAN, 0, KEY_NATION_TURKEY, 1),
    (2, 11, 9, 2000, KEY_NATION_MOLDOVA, 0, KEY_NATION_FYR_OF_MACEDONIA, 0),
    (2, 24, 2, 2001, KEY_NATION_SWEDEN, 1, KEY_NATION_FYR_OF_MACEDONIA, 0),
    (2, 24, 2, 2001, KEY_NATION_TURKEY, 1, KEY_NATION_SLOVAKIA, 1),
    (2, 24, 2, 2001, KEY_NATION_AZERBAIJAN, 0, KEY_NATION_MOLDOVA, 0),
    (2, 28, 2, 2001, KEY_NATION_MOLDOVA, 0, KEY_NATION_SWEDEN, 2),
    (2, 28, 2, 2001, KEY_NATION_SLOVAKIA, 3, KEY_NATION_AZERBAIJAN, 1),
    (2, 28, 2, 2001, KEY_NATION_FYR_OF_MACEDONIA, 1, KEY_NATION_TURKEY, 2),
    (2, 2, 5, 2001, KEY_NATION_TURKEY, 3, KEY_NATION_AZERBAIJAN, 0),
    (2, 2, 5, 2001, KEY_NATION_FYR_OF_MACEDONIA, 2, KEY_NATION_MOLDOVA, 2),
    (2, 2, 5, 2001, KEY_NATION_SWEDEN, 2, KEY_NATION_SLOVAKIA, 0),
    (2, 6, 5, 2001, KEY_NATION_TURKEY, 3, KEY_NATION_FYR_OF_MACEDONIA, 3),
    (2, 6, 5, 2001, KEY_NATION_SWEDEN, 6, KEY_NATION_MOLDOVA, 0),
    (2, 6, 5, 2001, KEY_NATION_AZERBAIJAN, 2, KEY_NATION_SLOVAKIA, 0),
    (2, 1, 8, 2001, KEY_NATION_FYR_OF_MACEDONIA, 1, KEY_NATION_SWEDEN, 2),
    (2, 1, 8, 2001, KEY_NATION_MOLDOVA, 2, KEY_NATION_AZERBAIJAN, 0),
    (2, 1, 8, 2001, KEY_NATION_SLOVAKIA, 0, KEY_NATION_TURKEY, 1),
    (2, 5, 8, 2001, KEY_NATION_TURKEY, 1, KEY_NATION_SWEDEN, 2),
    (2, 5, 8, 2001, KEY_NATION_SLOVAKIA, 4, KEY_NATION_MOLDOVA, 2),
    (2, 5, 8, 2001, KEY_NATION_AZERBAIJAN, 1, KEY_NATION_FYR_OF_MACEDONIA, 1),
    (2, 7, 9, 2001, KEY_NATION_FYR_OF_MACEDONIA, -1, KEY_NATION_SLOVAKIA, -1),
    (2, 7, 9, 2001, KEY_NATION_MOLDOVA, -1, KEY_NATION_TURKEY, -1),
    (2, 7, 9, 2001, KEY_NATION_SWEDEN, -1, KEY_NATION_AZERBAIJAN, -1),
    (3, 2, 8, 2000, KEY_NATION_BELARUS, 2, KEY_NATION_WALES, 1),
    (3, 2, 8, 2000, KEY_NATION_NORWAY, 0, KEY_NATION_ARMENIA, 0),
    (3, 2, 8, 2000, KEY_NATION_UKRAINE, 1, KEY_NATION_POLAND, 3),
    (3, 7, 9, 2000, KEY_NATION_WALES, 1, KEY_NATION_NORWAY, 1),
    (3, 7, 9, 2000, KEY_NATION_ARMENIA, 2, KEY_NATION_UKRAINE, 3),
    (3, 7, 9, 2000, KEY_NATION_POLAND, 3, KEY_NATION_BELARUS, 1),
    (3, 11, 9, 2000, KEY_NATION_POLAND, 0, KEY_NATION_WALES, 0),
    (3, 11, 9, 2000, KEY_NATION_NORWAY, 0, KEY_NATION_UKRAINE, 1),
    (3, 11, 9, 2000, KEY_NATION_BELARUS, 2, KEY_NATION_ARMENIA, 1),
    (3, 24, 2, 2001, KEY_NATION_UKRAINE, 0, KEY_NATION_BELARUS, 0),
    (3, 24, 2, 2001, KEY_NATION_NORWAY, 2, KEY_NATION_POLAND, 3),
    (3, 24, 2, 2001, KEY_NATION_ARMENIA, 2, KEY_NATION_WALES, 2),
    (3, 28, 2, 2001, KEY_NATION_POLAND, 4, KEY_NATION_ARMENIA, 0),
    (3, 28, 2, 2001, KEY_NATION_WALES, 1, KEY_NATION_UKRAINE, 1),
    (3, 28, 2, 2001, KEY_NATION_BELARUS, 2, KEY_NATION_NORWAY, 1),
    (3, 2, 5, 2001, KEY_NATION_ARMENIA, 0, KEY_NATION_BELARUS, 0),
    (3, 2, 5, 2001, KEY_NATION_WALES, 1, KEY_NATION_POLAND, 2),
    (3, 2, 5, 2001, KEY_NATION_UKRAINE, 0, KEY_NATION_NORWAY, 0),
    (3, 6, 5, 2001, KEY_NATION_NORWAY, 1, KEY_NATION_BELARUS, 1),
    (3, 6, 5, 2001, KEY_NATION_UKRAINE, 1, KEY_NATION_WALES, 1),
    (3, 6, 5, 2001, KEY_NATION_ARMENIA, 1, KEY_NATION_POLAND, 1),
    (3, 1, 8, 2001, KEY_NATION_BELARUS, 0, KEY_NATION_UKRAINE, 2),
    (3, 1, 8, 2001, KEY_NATION_WALES, 0, KEY_NATION_ARMENIA, 0),
    (3, 1, 8, 2001, KEY_NATION_POLAND, 0, KEY_NATION_NORWAY, 0),
    (3, 5, 8, 2001, KEY_NATION_UKRAINE, 3, KEY_NATION_ARMENIA, 0),
    (3, 5, 8, 2001, KEY_NATION_BELARUS, 4, KEY_NATION_POLAND, 1),
    (3, 5, 8, 2001, KEY_NATION_NORWAY, 3, KEY_NATION_WALES, 2),
    (3, 6, 9, 2001, KEY_NATION_WALES, -1, KEY_NATION_BELARUS, -1),
    (3, 6, 9, 2001, KEY_NATION_ARMENIA, -1, KEY_NATION_NORWAY, -1),
    (3, 6, 9, 2001, KEY_NATION_POLAND, -1, KEY_NATION_UKRAINE, -1),
    (4, 2, 8, 2000, KEY_NATION_LATVIA, 0, KEY_NATION_SCOTLAND, 1),
    (4, 2, 8, 2000, KEY_NATION_BELGIUM, 0, KEY_NATION_CROATIA, 0),
    (4, 7, 9, 2000, KEY_NATION_LATVIA, 0, KEY_NATION_BELGIUM, 4),
    (4, 7, 9, 2000, KEY_NATION_SAN_MARINO, 0, KEY_NATION_SCOTLAND, 2),
    (4, 11, 9, 2000, KEY_NATION_CROATIA, 1, KEY_NATION_SCOTLAND, 1),
    (4, 15, 10, 2000, KEY_NATION_SAN_MARINO, 0, KEY_NATION_LATVIA, 1),
    (4, 14, 1, 2001, KEY_NATION_BELGIUM, 10, KEY_NATION_SAN_MARINO, 1),
    (4, 24, 2, 2001, KEY_NATION_CROATIA, 4, KEY_NATION_LATVIA, 1),
    (4, 24, 2, 2001, KEY_NATION_SCOTLAND, 2, KEY_NATION_BELGIUM, 2),
    (4, 28, 2, 2001, KEY_NATION_SCOTLAND, 4, KEY_NATION_SAN_MARINO, 0),
    (4, 25, 3, 2001, KEY_NATION_LATVIA, 1, KEY_NATION_SAN_MARINO, 1),
    (4, 2, 5, 2001, KEY_NATION_CROATIA, 4, KEY_NATION_SAN_MARINO, 0),
    (4, 2, 5, 2001, KEY_NATION_BELGIUM, 3, KEY_NATION_LATVIA, 1),
    (4, 6, 5, 2001, KEY_NATION_LATVIA, 0, KEY_NATION_CROATIA, 1),
    (4, 6, 5, 2001, KEY_NATION_SAN_MARINO, 1, KEY_NATION_BELGIUM, 4),
    (4, 1, 8, 2001, KEY_NATION_SCOTLAND, 0, KEY_NATION_CROATIA, 0),
    (4, 5, 8, 2001, KEY_NATION_SAN_MARINO, 0, KEY_NATION_CROATIA, 4),
    (4, 5, 8, 2001, KEY_NATION_BELGIUM, 2, KEY_NATION_SCOTLAND, 0),
    (4, 6, 9, 2001, KEY_NATION_CROATIA, -1, KEY_NATION_BELGIUM, -1),
    (4, 6, 9, 2001, KEY_NATION_SCOTLAND, -1, KEY_NATION_LATVIA, -1),
    (5, 2, 8, 2000, KEY_NATION_BOSNIA_HERZEGOVINA, 1, KEY_NATION_SPAIN, 2),
    (5, 3, 8, 2000, KEY_NATION_ISRAEL, 2, KEY_NATION_LIECHTENSTEIN, 0),
    (5, 7, 9, 2000, KEY_NATION_LIECHTENSTEIN, 0, KEY_NATION_AUSTRIA, 1),
    (5, 7, 9, 2000, KEY_NATION_SPAIN, 2, KEY_NATION_ISRAEL, 0),
    (5, 11, 9, 2000, KEY_NATION_ISRAEL, 3, KEY_NATION_BOSNIA_HERZEGOVINA, 1),
    (5, 11, 9, 2000, KEY_NATION_AUSTRIA, 1, KEY_NATION_SPAIN, 1),
    (5, 24, 2, 2001, KEY_NATION_BOSNIA_HERZEGOVINA, 1, KEY_NATION_AUSTRIA, 1),
    (5, 24, 2, 2001, KEY_NATION_SPAIN, 5, KEY_NATION_LIECHTENSTEIN, 0),
    (5, 28, 2, 2001, KEY_NATION_LIECHTENSTEIN, 0, KEY_NATION_BOSNIA_HERZEGOVINA, 3),
    (5, 28, 2, 2001, KEY_NATION_AUSTRIA, 2, KEY_NATION_ISRAEL, 1),
    (5, 25, 3, 2001, KEY_NATION_AUSTRIA, 2, KEY_NATION_LIECHTENSTEIN, 0),
    (5, 2, 5, 2001, KEY_NATION_SPAIN, 4, KEY_NATION_BOSNIA_HERZEGOVINA, 1),
    (5, 2, 5, 2001, KEY_NATION_LIECHTENSTEIN, 0, KEY_NATION_ISRAEL, 3),
    (5, 6, 5, 2001, KEY_NATION_ISRAEL, 1, KEY_NATION_SPAIN, 1),
    (5, 1, 8, 2001, KEY_NATION_BOSNIA_HERZEGOVINA, 0, KEY_NATION_ISRAEL, 0),
    (5, 1, 8, 2001, KEY_NATION_SPAIN, 4, KEY_NATION_AUSTRIA, 0),
    (5, 5, 8, 2001, KEY_NATION_LIECHTENSTEIN, 0, KEY_NATION_SPAIN, 2),
    (5, 5, 8, 2001, KEY_NATION_AUSTRIA, 2, KEY_NATION_BOSNIA_HERZEGOVINA, 0),
    (5, 7, 9, 2001, KEY_NATION_ISRAEL, -1, KEY_NATION_AUSTRIA, -1),
    (5, 7, 9, 2001, KEY_NATION_BOSNIA_HERZEGOVINA, -1, KEY_NATION_LIECHTENSTEIN, -1),
    (6, 2, 8, 2000, KEY_NATION_ROMANIA, 1, KEY_NATION_LITHUANIA, 0),
    (6, 2, 8, 2000, KEY_NATION_HUNGARY, 2, KEY_NATION_ITALY, 2),
    (6, 7, 9, 2000, KEY_NATION_LITHUANIA, 0, KEY_NATION_GEORGIA, 4),
    (6, 7, 9, 2000, KEY_NATION_ITALY, 3, KEY_NATION_ROMANIA, 0),
    (6, 11, 9, 2000, KEY_NATION_LITHUANIA, 1, KEY_NATION_HUNGARY, 6),
    (6, 11, 9, 2000, KEY_NATION_ITALY, 2, KEY_NATION_GEORGIA, 0),
    (6, 24, 2, 2001, KEY_NATION_HUNGARY, 1, KEY_NATION_LITHUANIA, 1),
    (6, 24, 2, 2001, KEY_NATION_ROMANIA, 0, KEY_NATION_ITALY, 2),
    (6, 28, 2, 2001, KEY_NATION_GEORGIA, 0, KEY_NATION_ROMANIA, 2),
    (6, 28, 2, 2001, KEY_NATION_ITALY, 4, KEY_NATION_LITHUANIA, 0),
    (6, 2, 5, 2001, KEY_NATION_ROMANIA, 2, KEY_NATION_HUNGARY, 0),
    (6, 2, 5, 2001, KEY_NATION_GEORGIA, 1, KEY_NATION_ITALY, 2),
    (6, 6, 5, 2001, KEY_NATION_LITHUANIA, 1, KEY_NATION_ROMANIA, 2),
    (6, 6, 5, 2001, KEY_NATION_HUNGARY, 4, KEY_NATION_GEORGIA, 1),
    (6, 1, 8, 2001, KEY_NATION_GEORGIA, 3, KEY_NATION_HUNGARY, 1),
    (6, 1, 8, 2001, KEY_NATION_LITHUANIA, 0, KEY_NATION_ITALY, 0),
    (6, 5, 8, 2001, KEY_NATION_GEORGIA, 2, KEY_NATION_LITHUANIA, 0),
    (6, 5, 8, 2001, KEY_NATION_HUNGARY, 0, KEY_NATION_ROMANIA, 2),
    (6, 6, 9, 2001, KEY_NATION_ITALY, -1, KEY_NATION_HUNGARY, -1),
    (6, 6, 9, 2001, KEY_NATION_ROMANIA, -1, KEY_NATION_GEORGIA, -1),
    (7, 2, 8, 2000, KEY_NATION_FINLAND, 2, KEY_NATION_ALBANIA, 1),
    (7, 2, 8, 2000, KEY_NATION_GERMANY, 2, KEY_NATION_GREECE, 0),
    (7, 7, 9, 2000, KEY_NATION_GREECE, 1, KEY_NATION_FINLAND, 0),
    (7, 7, 9, 2000, KEY_NATION_ENGLAND, 0, KEY_NATION_GERMANY, 1),
    (7, 11, 9, 2000, KEY_NATION_ALBANIA, 2, KEY_NATION_GREECE, 0),
    (7, 11, 9, 2000, KEY_NATION_FINLAND, 0, KEY_NATION_ENGLAND, 0),
    (7, 24, 2, 2001, KEY_NATION_ENGLAND, 2, KEY_NATION_FINLAND, 1),
    (7, 24, 2, 2001, KEY_NATION_GERMANY, 2, KEY_NATION_ALBANIA, 1),
    (7, 28, 2, 2001, KEY_NATION_ALBANIA, 1, KEY_NATION_ENGLAND, 3),
    (7, 28, 2, 2001, KEY_NATION_GREECE, 2, KEY_NATION_GERMANY, 4),
    (7, 2, 5, 2001, KEY_NATION_GREECE, 1, KEY_NATION_ALBANIA, 0),
    (7, 2, 5, 2001, KEY_NATION_FINLAND, 2, KEY_NATION_GERMANY, 2),
    (7, 6, 5, 2001, KEY_NATION_GREECE, 0, KEY_NATION_ENGLAND, 2),
    (7, 6, 5, 2001, KEY_NATION_ALBANIA, 0, KEY_NATION_GERMANY, 2),
    (7, 1, 8, 2001, KEY_NATION_GERMANY, 1, KEY_NATION_ENGLAND, 5),
    (7, 1, 8, 2001, KEY_NATION_ALBANIA, 0, KEY_NATION_FINLAND, 2),
    (7, 5, 8, 2001, KEY_NATION_ENGLAND, 2, KEY_NATION_ALBANIA, 0),
    (7, 5, 8, 2001, KEY_NATION_FINLAND, 5, KEY_NATION_GREECE, 1),
    (7, 6, 9, 2001, KEY_NATION_GERMANY, -1, KEY_NATION_FINLAND, -1),
    (7, 6, 9, 2001, KEY_NATION_ENGLAND, -1, KEY_NATION_GREECE, -1), // r7_06_2: the last match
];

/// The nine groups of the 2002 European section (the order of the original's local table): the section's own group
/// first, then the eight group stages; the last four groups have five nations.
pub(crate) const WC_EUROPE_2002_GROUPS: [&[usize]; 9] = [
    &[KEY_NATION_SWITZERLAND, KEY_NATION_RUSSIA, KEY_NATION_FAROE_ISLANDS, KEY_NATION_SLOVENIA, KEY_NATION_LUXEMBOURG, KEY_NATION_YUGOSLAVIA],
    &[KEY_NATION_ANDORRA, KEY_NATION_CYPRUS, KEY_NATION_HOLLAND, KEY_NATION_REPUBLIC_OF_IRELAND, KEY_NATION_ESTONIA, KEY_NATION_PORTUGAL],
    &[KEY_NATION_ICELAND, KEY_NATION_DENMARK, KEY_NATION_BULGARIA, KEY_NATION_CZECH_REPUBLIC, KEY_NATION_NORTHERN_IRELAND, KEY_NATION_MALTA],
    &[KEY_NATION_TURKEY, KEY_NATION_MOLDOVA, KEY_NATION_AZERBAIJAN, KEY_NATION_SWEDEN, KEY_NATION_SLOVAKIA, KEY_NATION_FYR_OF_MACEDONIA],
    &[KEY_NATION_BELARUS, KEY_NATION_WALES, KEY_NATION_NORWAY, KEY_NATION_ARMENIA, KEY_NATION_UKRAINE, KEY_NATION_POLAND],
    &[KEY_NATION_LATVIA, KEY_NATION_SCOTLAND, KEY_NATION_BELGIUM, KEY_NATION_CROATIA, KEY_NATION_SAN_MARINO],
    &[KEY_NATION_BOSNIA_HERZEGOVINA, KEY_NATION_SPAIN, KEY_NATION_ISRAEL, KEY_NATION_LIECHTENSTEIN, KEY_NATION_AUSTRIA],
    &[KEY_NATION_ITALY, KEY_NATION_HUNGARY, KEY_NATION_ROMANIA, KEY_NATION_GEORGIA, KEY_NATION_LITHUANIA],
    &[KEY_NATION_FINLAND, KEY_NATION_ALBANIA, KEY_NATION_GERMANY, KEY_NATION_GREECE, KEY_NATION_ENGLAND],
];
