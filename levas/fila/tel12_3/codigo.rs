// ---------- batch %%LEVA%%: the Setup screens, part 7: appointing the human manager, the default league and creating the game (Setup.cpp) ----------

// (uses setup_warning, setup_fatal_error, check_game_cd and copy_text of setup.rs and the Setup names of tel11_1)
const FN_MANAGER_MANAGER_APPOINT_MANAGER_TO_CLUB: usize = 0x69ebc0; // thiscall(MANAGER_MANAGER, club, manager) ret 8
const FN_FRIENDLY_REMOVE_CLUB_FROM_AI_PLANS: usize = 0x5c64e0; // thiscall(Friendly, club) ret 4
const FN_DATABASE_TRIM_SENIOR_AND_B_TEAM_SQUADS: usize = 0x541e60; // cdecl()
const FN_VERIFY_BACKGROUND_CONFIG: usize = 0x41bc20; // cdecl()
const FN_VERIFY_HISTORY_FILES: usize = 0x5f2840; // cdecl()
const FN_VALIDATE_EURO_CFG: usize = 0x905d40; // cdecl()
const FN_DATABASE_BUILD_NATION_NAME_INDEX: usize = 0x539750; // cdecl()
const FN_DATABASE_PRUNE_COMP_HISTORY_TABLE: usize = 0x539ae0; // cdecl()
const FN_DATABASE_TRIM_STAFF_TABLES: usize = 0x524d70; // cdecl(database_size)
const FN_DATABASE_TRIM_CLUB_TABLE: usize = 0x5267c0; // cdecl(database_size)
const FN_FIX_DIVISION_CLUB_COUNT: usize = 0x839920; // cdecl(comp_id, wanted)
const FN_DATABASE_MERGE_B_TEAM_STAFF: usize = 0x544970; // cdecl()
const FN_SETUP_SET_GAME_START_DATE: usize = 0x830520; // cdecl() -> 1 or 0 (ignored)
const FN_DATABASE_PLAYER_INITIALISATION: usize = 0x535a80; // cdecl(game_date)
const FN_DATABASE_APPLY_FAKE_PLAYERS: usize = 0x532e00; // cdecl()
const FN_KEY_NATION_NUMBER_ACTIVE_ENTRIES: usize = 0x669140; // cdecl() -> 1
const FN_KEY_NATION_CREATE_ALL_COMPS: usize = 0x669180; // cdecl() -> int
const FN_BUILD_COMP_OBJECT_TABLE: usize = 0x8386f0; // cdecl() -> 1 or 0
const FN_RESERVECOMP_CREATE_SUB_COMPS: usize = 0x7e61c0; // thiscall(ReserveComp)
const FN_CONTRACT_MANAGER_INITIALISE_ALL_STAFF_CONTRACTS: usize = 0x4c7560; // thiscall(CONTRACT_MANAGER)
const FN_FINANCETABLE_REFRESH_ALL_WAGE_BILLS: usize = 0x59cf50; // thiscall(FinanceTable)
const FN_SQUAD_MANAGER_INIT_NEW_GAME: usize = 0x859dc0; // thiscall(SQUAD_MANAGER)
const FN_GAME_RUN_TO_START_DATE: usize = 0x5cc850; // cdecl()
const G_DATABASE_SIZE_OPTION: usize = 0xdd7114; // the database size of the new game (Config options & 7), byte
const MANAGER_MANAGER_CLUB_RECORDS: usize = 0x0; // the MANAGER_MANAGER's table of 0x49-byte club records [inference: indexed by club id]

/// Appoints the human `manager` to `club` (returns 1; 0 with a null argument, after the Setup.cpp "Error" box 0x1a4d).
///
/// The club's manager leaves (`MANAGER_MANAGER::remove_club_manager`, departure 3, reason 0, keeping the ratings), or,
/// without one, the manager of its linked B team (`club_get_linked_b_team`(club, 0, 1)) when it has one. For a national
/// side (`database_is_national_team`) that is not a club's national team (`club_is_national_team`), the manager of the
/// nation's under-21 side (`database_get_national_u21_team` of `club::NATION`) leaves too, unless a human runs it
/// (`human_manager_is_human_club`(u21, 1, 1, 0, 0)). The manager's reputations come from the club's (`club::REPUTATION`):
/// current and home reputation = reputation / 10 + 3000, world reputation = reputation × 0.4 + 750 (truncated);
/// `staff::LOYALTY` = 20. The club leaves the friendlies' AI plans (`Friendly::remove_club_from_ai_plans`) and the
/// manager is appointed (`MANAGER_MANAGER::appoint_manager_to_club`); outside a running game (`G_INTERACTIVE_MODE_FLAG`
/// = 0) bit 0 of byte 2 of the club's 0x49-byte record in the MANAGER_MANAGER table is cleared.
/// Original: `appoint_human_manager_to_club` (0x826f20, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn appoint_human_manager_to_club(manager: usize, club: usize) -> u32 {
    covered!("tel12_3 appoint human manager");
    let (manager, club) = (Staff(manager), Club(club));
    let remove: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) = core::mem::transmute(FN_MANAGER_MANAGER_REMOVE_CLUB_MANAGER);
    let appoint: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_MANAGER_MANAGER_APPOINT_MANAGER_TO_CLUB);
    let friendly: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_FRIENDLY_REMOVE_CLUB_FROM_AI_PLANS);
    if manager.0 == 0 || club.0 == 0 {
        setup_warning(0x1a4d);
        return 0;
    }
    if club.manager().0 != 0 {
        remove(read_usize(G_MANAGER_MANAGER), club.0, 3, 0, 1);
    } else {
        let b = exe::club_get_linked_b_team(club, core::ptr::null_mut(), 1);
        if b != 0 && Club(b).manager().0 != 0 {
            remove(read_usize(G_MANAGER_MANAGER), b, 3, 0, 1);
        }
    }
    if exe::database_is_national_team(club) != 0 && exe::club_is_national_team(club) == 0 {
        let u21 = exe::database_get_national_u21_team(read_usize(club.0 + club::NATION));
        if Club(u21).manager().0 != 0 && exe::human_manager_is_human_club(u21, 1, 1, 0, 0) == 0 {
            remove(read_usize(G_MANAGER_MANAGER), u21, 3, 0, 1);
        }
    }
    let reputation = club.reputation() as i32;
    let np = manager.non_player();
    np.set_current_reputation((reputation / 10 + 0xbb8) as i16);
    np.set_home_reputation((reputation / 10 + 0xbb8) as i16);
    let world = ftol(reputation as f64 * read_f64(rdata::F64_0_4) + read_f64(rdata::F64_750));
    manager.non_player().set_world_reputation(world as i16);
    manager.set_loyalty(0x14);
    friendly(read_usize(G_FRIENDLY), club.0);
    appoint(read_usize(G_MANAGER_MANAGER), club.0, manager.0);
    if read_u32(G_INTERACTIVE_MODE_FLAG) == 0 {
        let id = read_u32(club.0) as usize;
        let flags = read_usize(read_usize(G_MANAGER_MANAGER) + MANAGER_MANAGER_CLUB_RECORDS) + id * 0x49 + 2;
        write_u8(flags, read_u8(flags) & 0xfe);
    }
    1
}

/// Picks the default league by the game's language: with a database (`G_NATION_TABLE`; without one, the Setup.cpp
/// "Error" box 0x1d88 and 0), every nation's `nation::LEAGUE_SELECTED` is zeroed, then the key nation of the language
/// (`langlib_get_language`: 0 and 1 England, 2 France, 3 Germany, 4 Italy, 5 Spain, 6 Denmark, 7 Holland, 8 Norway, 9
/// Portugal, 10 Sweden, 11 Japan, 12 South Korea) gets 2 (selected, in the foreground), `G_NATIONS_TO_LOAD_COUNT` =
/// `G_SELECTED_NATION_COUNT` = 1 and becomes `G_MAIN_NATION`; returns 1. Any other language: the counters and the main
/// nation stay zeroed, the "Error" box 0x1e09 and 0.
/// Original: `select_default_league_by_language` (0x827d50, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn select_default_league_by_language() -> u32 {
    covered!("tel12_3 default league");
    if read_usize(G_NATION_TABLE) == 0 {
        setup_warning(0x1d88);
        return 0;
    }
    for k in 0..read_i32(G_NATION_COUNT).max(0) as usize {
        write_u8(read_usize(G_NATION_TABLE) + k * nation::SIZE + nation::LEAGUE_SELECTED, 0);
    }
    write_u32(G_NATIONS_TO_LOAD_COUNT, 0);
    write_u32(G_SELECTED_NATION_COUNT, 0);
    write_u32(G_MAIN_NATION, 0);
    let key = match exe::langlib_get_language() {
        0 | 1 => KEY_NATION_ENGLAND,
        2 => KEY_NATION_FRANCE,
        3 => KEY_NATION_GERMANY,
        4 => KEY_NATION_ITALY,
        5 => KEY_NATION_SPAIN,
        6 => KEY_NATION_DENMARK,
        7 => KEY_NATION_HOLLAND,
        8 => KEY_NATION_NORWAY,
        9 => KEY_NATION_PORTUGAL,
        10 => KEY_NATION_SWEDEN,
        11 => KEY_NATION_JAPAN,
        12 => KEY_NATION_SOUTH_KOREA,
        _ => {
            setup_warning(0x1e09);
            return 0;
        }
    };
    let selected = |k: usize| read_usize(G_NATION_TABLE) + read_u32(k) as usize * nation::SIZE;
    write_u8(selected(key) + nation::LEAGUE_SELECTED, 2);
    write_u32(G_NATIONS_TO_LOAD_COUNT, 1);
    write_u32(G_SELECTED_NATION_COUNT, 1);
    write_usize(G_MAIN_NATION, selected(key));
    1
}

/// How a subsystem's constructor is called by `create_game` (thiscall, `this` = the `operator new`).
#[derive(Clone, Copy)]
enum CreateArg {
    /// (this, 0)
    Zero,
    /// (this)
    Alone,
    /// (this, 0, matches stored): Match_Stats
    Stats,
    /// (this, game year, 0, 1): FIXTURE_MANAGER (the original pushes a dword with stack leftover above the year; the
    /// constructor only reads the word)
    Fixture,
    /// (this, the names language byte): PunditList
    Language,
}

/// One step of `create_game` after the start: `Routine(f, row)` a cdecl() that fails at Setup.cpp `row` when it returns
/// 0; `Method(f, global)` a thiscall on the subsystem in `global`; `Object(size, constructor, global, row, arg)`; the club
/// pass (`Clubs`).
#[derive(Clone, Copy)]
enum CreateStep {
    Routine(usize, u32),
    Method(usize, usize),
    Object(u32, usize, usize, u32, CreateArg),
    Clubs,
}

/// The steps of `create_game` from 2 on, in the original's order, with the progress step of each (0x2c in all).
const CREATE_STEPS: [(u32, CreateStep); 0x2d] = [
    (2, CreateStep::Routine(FN_KEY_NATION_NUMBER_ACTIVE_ENTRIES, 0x1f6d)),
    (3, CreateStep::Object(0x609, 0x893dc0, G_TACTICS, 0x1f82, CreateArg::Zero)),
    (4, CreateStep::Object(0x10, 0x5fb430, G_HUMAN_MANAGER, 0x1f97, CreateArg::Zero)),
    (5, CreateStep::Object(0xe8, 0x7f6760, G_SCOUT_MANAGER, 0x1fac, CreateArg::Zero)),
    (6, CreateStep::Object(0x14, 0x785b20, G_NEWS, 0x1fc1, CreateArg::Zero)),
    (7, CreateStep::Object(0x3507, 0x5c1f80, G_FRIENDLY, 0x1fd6, CreateArg::Alone)),
    (8, CreateStep::Object(0x698, 0x903360, G_UEFA_SEEDING, 0x1feb, CreateArg::Alone)),
    (9, CreateStep::Object(0x75, 0x4c5fe0, G_CONMEBOL_SEEDING, 0x2000, CreateArg::Alone)),
    (0xa, CreateStep::Object(0x18, 0x868390, G_STAFF_RECORDS, 0x2015, CreateArg::Zero)),
    (0xb, CreateStep::Object(0x20, 0x729000, G_MATCH_STATS, 0x2037, CreateArg::Stats)),
    (0xc, CreateStep::Object(0x30, 0x444520, G_CLUB_RECORDS, 0x204c, CreateArg::Zero)),
    (0xd, CreateStep::Object(0x48, 0x7bfd50, G_PLAYER_STATS, 0x2061, CreateArg::Zero)),
    (0xe, CreateStep::Object(0x10, 0x442ab0, G_CLUB_HISTORY, 0x2076, CreateArg::Zero)),
    (0xf, CreateStep::Object(6, 0x5f97c0, G_HOST_COUNTRY, 0x208b, CreateArg::Alone)),
    (0x10, CreateStep::Object(0x2a, 0x5554a0, G_DISCIPLINE, 0x20a0, CreateArg::Zero)),
    (0x11, CreateStep::Object(0xe0, 0x413180, G_AWARD_MANAGER, 0x20b5, CreateArg::Zero)),
    (0x12, CreateStep::Object(0xc, 0x5a4fc0, G_FINE, 0x20c9, CreateArg::Zero)),
    (0x13, CreateStep::Object(0x10, 0x418440, G_AWOL, 0x20dd, CreateArg::Zero)),
    (0x14, CreateStep::Object(8, 0x5697e0, G_DISPUTE, 0x20f1, CreateArg::Zero)),
    (0x15, CreateStep::Object(0x1070, 0x794ad0, G_NOTES, 0x2104, CreateArg::Zero)),
    (0x16, CreateStep::Object(0x2c, 0x62a680, G_INJURY, 0x2119, CreateArg::Zero)),
    (0x17, CreateStep::Object(0x3a, 0x76a4e0, G_NATIONAL_TEAMS, 0x212e, CreateArg::Zero)),
    (0x18, CreateStep::Object(0x6a, 0x90d140, G_VIRTUAL_STAFF, 0x2143, CreateArg::Alone)),
    (0x19, CreateStep::Object(0x19, 0x85b670, G_STADIUM, 0x2158, CreateArg::Alone)),
    (0x1a, CreateStep::Object(0x1184, 0x5a9cf0, G_FIXTURE_MANAGER, 0x216d, CreateArg::Fixture)),
    (0x1b, CreateStep::Object(0xab, 0x7e5fc0, G_RESERVE_COMP, 0x2182, CreateArg::Zero)),
    (0x1b, CreateStep::Routine(FN_KEY_NATION_CREATE_ALL_COMPS, 0x2193)),
    (0x1c, CreateStep::Routine(FN_BUILD_COMP_OBJECT_TABLE, 0x21a7)),
    (0x1d, CreateStep::Object(2, 0x859aa0, G_SQUAD_MANAGER, 0x21bc, CreateArg::Zero)),
    (0x1e, CreateStep::Method(FN_RESERVECOMP_CREATE_SUB_COMPS, G_RESERVE_COMP)),
    (0x1f, CreateStep::Object(4, 0x59a370, G_FINANCE_TABLE, 0x21e1, CreateArg::Zero)),
    (0x20, CreateStep::Object(0x3c, 0x4c7040, G_CONTRACT_MANAGER, 0x21f6, CreateArg::Zero)),
    (0x21, CreateStep::Method(FN_FINANCETABLE_REFRESH_ALL_WAGE_BILLS, G_FINANCE_TABLE)),
    (0x22, CreateStep::Object(0x94a, 0x8bba30, G_TRANSFER_MANAGER, 0x221e, CreateArg::Zero)),
    (0x23, CreateStep::Clubs),
    (0x24, CreateStep::Object(0x60, 0x7a5380, G_PLAYER_REGEN, 0x2243, CreateArg::Zero)),
    (0x25, CreateStep::Object(0x1c, 0x68dbf0, G_MANAGER_MANAGER, 0x2258, CreateArg::Zero)),
    (0x26, CreateStep::Object(0x18, 0x72b1d0, G_MEDIA, 0x226d, CreateArg::Zero)),
    (0x27, CreateStep::Object(0x40, 0x5af630, G_FOG_OF_WAR, 0x2282, CreateArg::Zero)),
    (0x28, CreateStep::Object(0x10, 0x79b580, G_OFFICIALS_MANAGER, 0x2297, CreateArg::Zero)),
    (0x29, CreateStep::Method(FN_SQUAD_MANAGER_INIT_NEW_GAME, G_SQUAD_MANAGER)),
    (0x2a, CreateStep::Object(8, 0x84b940, G_SIMULATED_STATS, 0x22bc, CreateArg::Zero)),
    (0x2b, CreateStep::Object(0xc, 0x9226c0, G_WEATHER, 0x22d1, CreateArg::Alone)),
    (0x2c, CreateStep::Object(0x688f, 0x8b0ec0, G_TRAINING_MANAGER, 0x22e6, CreateArg::Zero)),
    (0x2c, CreateStep::Object(0xa, 0x7da190, G_PUNDIT_LIST, 0x22f8, CreateArg::Language)),
];

/// The CD search at the start of `create_game`: each CD drive (`GetLogicalDriveStringsA`, `GetDriveTypeA` = 5) is looked
/// at for "<root>cm0102.exe" (`file_find_first`) until one has it, with the "Insert CD" box in between. A no-CD exe
/// patched in this routine (as `cm0102-1080`: the drive type's `jne` at 0x8281ae made `nop`, the found test at 0x8281f0
/// made `xor`) is honoured by reading those bytes.
unsafe fn create_game_find_cd() {
    let drives: unsafe extern "system" fn(u32, *mut u8) -> u32 = core::mem::transmute(read_usize(0x9670f8));
    let kind: unsafe extern "system" fn(usize) -> u32 = core::mem::transmute(read_usize(0x967100));
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let any_drive = (0x8281ae as *const u8).read() == 0x90;
    let always_found = (0x8281f0 as *const u8).read() != 0x85;
    loop {
        // simplification: the buffer starts zeroed (as check_game_cd)
        let mut list = [0u8; 0x100];
        if drives(200, list.as_mut_ptr()) != 0 && list[0] != 0 {
            let mut i = 0usize;
            loop {
                let root = list.as_ptr() as usize + i;
                let t = kind(root);
                if any_drive || t == 5 {
                    let (mut name, mut found) = ([0u8; 0x104], [0u8; 0x114]);
                    format(name.as_mut_ptr() as usize, 0x987e00, root, 0x63u32, 0x6du32, 0x30u32, 0x31u32, 0x30u32, 0x32u32, 0x2eu32, 0x65u32, 0x78u32, 0x65u32);
                    if exe::file_find_first(name.as_ptr() as usize, found.as_mut_ptr() as usize, 0) == 0 || always_found {
                        return;
                    }
                    exe::file_find_close(found.as_mut_ptr() as usize);
                }
                // simplification: the original steps with a signed byte (only past 32 drives would it differ)
                i += 4;
                if list[i] == 0 {
                    break;
                }
            }
        }
        let (mut title, mut message) = ([0u8; 0x100], [0u8; 0x100]);
        text(title.as_mut_ptr() as usize, 0x987df4); // "Insert CD"
        text(message.as_mut_ptr() as usize, 0x987dc4); // "Please insert the CM01/02 CD and click on OK"
        exe::gui_show_message_box(title.as_ptr() as usize, message.as_ptr() as usize, 1);
    }
}

/// The runtime record of each person for a new game (`G_STAFF_STATE_TABLE`, `staff_state::SIZE` × `G_STAFF_COUNT`).
///
/// For person `s`: the two ids −1, byte +8 = 0, `staff_state::STATUS_FLAGS` keeps only bit 7. A club join date after the
/// game's start date (`G_GAME_START_DATE`; unsigned year, signed day) becomes that date (`Date::operator=`). With a
/// player record (`staff::PLAYER`) and a club job, and fewer days at the club (`Date::days_since` from the game date)
/// than 415 − 10 × adaptability (`staff::ADAPTABILITY`), the settling counter `staff_state::INJURY_COUNTER` [inference:
/// raised by injuries; here a settling-in value] = (26, or 21 from 215 days) − a − random(a), a = adaptability × 0.5
/// (truncated); a person without a nation gets England's; below 275 days a foreigner
/// (`database_is_staff_foreign`(s, nation, 1)) adds random(5) + 5; it is capped by the current ability
/// (`player_sheet::CURRENT_ABILITY`) × 0.15 (unsigned byte against the signed cap); otherwise 0. A nation join date
/// (`staff::DATE_JOINED_NATION`) after the start date becomes it. `staff_state::B_INTERNATIONAL_PLAYED` = 0, bit 1 of
/// `staff_state::EXTRA_FLAGS` cleared, the five former clubs (−1, weight 0) and the opinion links (−1) emptied.
unsafe fn create_game_staff_states(states: usize) {
    let start = G_GAME_START_DATE;
    for i in 0..read_i32(G_STAFF_COUNT).max(0) as usize {
        let st = states + i * staff_state::SIZE;
        let s = read_usize(G_STAFF_TABLE) + i * staff::SIZE;
        write_u32(st, 0xffffffff);
        write_u32(st + 4, 0xffffffff);
        write_u8(st + 8, 0);
        write_u8(st + staff_state::STATUS_FLAGS, read_u8(st + staff_state::STATUS_FLAGS) & 0x80);
        let after_start = |date: usize| {
            let (year, day) = (read_u16(start + 2), read_i16(start));
            year < read_u16(date + 2) || (year == read_u16(date + 2) && day < read_i16(date))
        };
        let joined = s + staff::DATE_JOINED_CLUB;
        if after_start(joined) {
            let mut tmp = [0u8; 8];
            exe::date_operator_3d(joined, tmp.as_mut_ptr() as usize, start);
        }
        let days = exe::date_days_since(G_CURRENT_DATE, read_u32(joined), read_u32(joined + 4));
        let adaptability = read_i8(s + staff::ADAPTABILITY) as i32;
        let player = read_usize(s + staff::PLAYER);
        if player == 0 || read_usize(s + staff::CLUB_JOB) == 0 || days >= 0x19f - 10 * adaptability {
            write_u8(st + staff_state::INJURY_COUNTER, 0);
        } else {
            let a = ftol(adaptability as f64 * read_f64(rdata::F64_0_5)) as u8;
            let r = exe::utils_random(a as i8 as i32) as u8;
            let base: u8 = if days < 0xd7 { 0x1a } else { 0x15 };
            write_u8(st + staff_state::INJURY_COUNTER, base.wrapping_sub(a).wrapping_sub(r));
            if read_usize(s + staff::NATION) == 0 {
                write_usize(s + staff::NATION, read_usize(G_NATION_TABLE) + read_u32(KEY_NATION_ENGLAND) as usize * nation::SIZE);
            }
            if days < 0x113 && exe::database_is_staff_foreign(s, read_usize(s + staff::NATION), 1) != 0 {
                let extra = (exe::utils_random(5) as u8).wrapping_add(5);
                write_u8(st + staff_state::INJURY_COUNTER, read_u8(st + staff_state::INJURY_COUNTER).wrapping_add(extra));
            }
            let ability = read_i16(read_usize(s + staff::PLAYER) + player_sheet::CURRENT_ABILITY) as i32;
            let cap = ftol(ability as f64 * read_f64(rdata::F64_0_15)) as i8;
            if read_u8(st + staff_state::INJURY_COUNTER) as i32 > cap as i32 {
                write_u8(st + staff_state::INJURY_COUNTER, cap as u8);
            }
        }
        let nation_date = s + staff::DATE_JOINED_NATION;
        if after_start(nation_date) {
            let mut tmp = [0u8; 8];
            exe::date_operator_3d(nation_date, tmp.as_mut_ptr() as usize, start);
        }
        write_u8(st + staff_state::B_INTERNATIONAL_PLAYED, 0);
        write_u8(st + staff_state::EXTRA_FLAGS, read_u8(st + staff_state::EXTRA_FLAGS) & 0xfd);
        for j in 0..5 {
            write_u32(st + staff_state::FORMER_CLUB_ID + 5 * j, 0xffffffff);
            write_u8(st + staff_state::FORMER_CLUB_WEIGHT + 5 * j, 0);
        }
        for j in 0..2 {
            for c in [staff_state::OPINION_LINK_STAFF_ID, 0x2b, 0x2f, 0x33, staff_state::OPINION_LINK_FLAG] {
                write_u32(st + c + 0x14 * j, 0xffffffff);
            }
        }
    }
}

/// The club pass of `create_game` (step 0x23), for each club: `club::OWN_STADIUM` = 0 for Brighton and Wimbledon (the
/// first 6 letters of `club::NAME`), 1 for the others; a reputation (`club::REPUTATION`) ≤ 0 becomes random(4000) + 1 as
/// a signed byte; attendance ≤ 0 becomes reputation² / 1000 for a national side (`database_is_national_team`), / 2000
/// otherwise, at least 250; minimum attendance ≤ 0: reputation² / 3350 (at least 750) or / 10000 (at least 75); maximum
/// attendance ≤ 0: reputation² / 500 (at least 3250) or / 1000 (at least 12500), plus random(7500); a training value
/// (`club::TRAINING`) outside 1..20 becomes reputation / 500 within 1..20.
unsafe fn create_game_clubs() {
    for i in 0..read_i32(G_CLUB_COUNT).max(0) {
        let club = club_at(i as isize);
        let name = club.0 + club::NAME;
        let shares = exe::crt_strncmp(name, 0xa813a0, 6) == 0 || exe::crt_strncmp(name, 0xa81394, 6) == 0; // "Brighton", "Wimbledon"
        club.set_own_stadium(if shares { 0 } else { 1 });
        if club.reputation() <= 0 {
            let r = (exe::utils_random(0xfa0) as u8).wrapping_add(1);
            club.set_reputation(r as i8 as i16);
        }
        if club.attendance() <= 0 {
            let national = exe::database_is_national_team(club) != 0;
            let rep = club.reputation() as i32;
            let square = rep.wrapping_mul(rep);
            let a = if national { square / 1000 } else { square / 2000 };
            club.set_attendance(a.max(0xfa));
        }
        if club.min_attendance() <= 0 {
            let national = exe::database_is_national_team(club) != 0;
            let rep = club.reputation() as i32;
            let square = rep.wrapping_mul(rep);
            let a = if national { (square / 0xd16).max(0x2ee) } else { (square / 10000).max(0x4b) };
            club.set_min_attendance(a);
        }
        if club.max_attendance() <= 0 {
            let national = exe::database_is_national_team(club) != 0;
            let rep = club.reputation() as i32;
            let square = rep.wrapping_mul(rep);
            let a = if national { (square / 500).max(0xcb2) } else { (square / 1000).max(0x30d4) };
            club.set_max_attendance(exe::utils_random(0x1d4c).wrapping_add(a));
        }
        let training = club.training();
        if training < 1 || training > 0x14 {
            let t = (club.reputation() as i32 / 500) as i8;
            club.set_training(t.clamp(1, 0x14));
        }
    }
}

/// Creates a new game (`quick_start` is not read: the original's argument is ignored); returns 1 when it is made.
///
/// Below 25 MB of free disk (`os_get_free_disk_space_mb`) `G_GAME_IN_PROGRESS_FLAG` = 1, below 50 MB of free memory
/// (with `G_MEMORY_CHECK_ENABLED`) `G_GAME_SESSION_STARTED_FLAG` = 1, and 0. The game CD (`create_game_find_cd`); without
/// a database, it is loaded from CM3_DATA (`load_database_from_files`(path, 1); a failure: the Setup.cpp "Error" box
/// 0x1e3d and 0) and the Config applied (`Config::apply_and_save`). Then: `database_trim_senior_and_b_team_squads`; the
/// command line checks "-verifypics" (`verify_background_config`), "-verifyhist" (`verify_history_files`),
/// "-verifyplcfg" (`player_setup_cfg_load`(0x203b, 1)) and "-verifyeurocfg" (`validate_euro_cfg`);
/// `database_build_nation_name_index`, `database_prune_comp_history_table`; the database size (Config options & 7) into
/// `G_DATABASE_SIZE_OPTION` trims the people and the clubs (`database_trim_staff_tables`, `database_trim_club_table`);
/// with Greece selected, its two divisions get 14 clubs (`fix_division_club_count`); `database_merge_b_team_staff`; the
/// club reputation order (`G_CLUB_REPUTATION_ORDER`, 4 × `G_CLUB_COUNT`; fatal 0x1ea0) sorted
/// (`database_sort_clubs_by_reputation`(1)); the start date (`setup_set_game_start_date`) and `G_GAME_START_DATE` moved
/// back 4 weeks (`Date::sub_weeks`); the people's runtime records (`create_game_staff_states`; fatal 0x1eaf) and the
/// clubs' (`G_CLUB_STATE_TABLE`, 6 × `G_CLUB_COUNT`: −1, 0, 0; fatal 0x1f0c); `G_GAME_PLAY_TIME_LAST_STAMP` = time(),
/// `G_GAME_PLAY_TIME_TOTAL` = 0; each club's `club::CONTINENTAL_COMP_ID` = −1 and its squad members without a club job
/// get it; `database_player_initialisation` at the game date and, with the German setup or without real players,
/// `database_apply_fake_players`. Then "Initialising game data" with the progress bar (0x2c steps): step 1 clears the
/// `*.tmp` files, and before each later step the disk and memory are checked again (25 MB each; before Match_Stats the
/// disk must hold its matches: 5000 with option 0x400, else 20000 with 0x800, else 10000, × 3450 >> 20 MB, 25 when 0):
/// the subsystems of `CREATE_STEPS` (each `operator new` and its constructor into its global; a null global gives its
/// "Error" box and 0, FOG_OF_WAR checking the Media global as in the original; CONTRACT_MANAGER then
/// `initialise_all_staff_contracts`), `key_nation_number_active_entries`, `key_nation_create_all_comps`,
/// `build_comp_object_table`, `ReserveComp::create_sub_comps`, `FinanceTable::refresh_all_wage_bills`, the club pass
/// (`create_game_clubs`) and `SQUAD_MANAGER::init_new_game`. Finally `game_run_to_start_date`,
/// `G_GAME_DATA_LOADED_FLAG` = 1, `G_GAME_HAS_SAVE_FILE_FLAG` = 0 and, without `auto_setup.cfg`, the "CD Not Required"
/// search (`check_game_cd`(false)). The original's two-pass loop (the first only sets the total 0x2c) runs the body once.
/// Original: `create_game` (0x828110, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn create_game(_quick_start: u32) -> u32 {
    covered!("tel12_3 create game");
    let call = |f: usize| {
        let f: unsafe extern "C" fn() = core::mem::transmute(f);
        f()
    };
    let size_call = |f: usize, size: u32| {
        let f: unsafe extern "C" fn(u32) = core::mem::transmute(f);
        f(size)
    };
    let text: unsafe extern "C" fn(*mut u8, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let player_setup: unsafe extern "C" fn(u32, u32) = core::mem::transmute(FN_PLAYER_SETUP_CFG_LOAD);
    let fix_division: unsafe extern "C" fn(u32, u32) = core::mem::transmute(FN_FIX_DIVISION_CLUB_COUNT);
    let initialise: unsafe extern "C" fn(usize) = core::mem::transmute(FN_DATABASE_PLAYER_INITIALISATION);
    let apply: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_CONFIG_APPLY_AND_SAVE);
    let no_memory = |minimum: i32| read_u32(G_MEMORY_CHECK_ENABLED) != 0 && exe::os_get_free_memory_mb() < minimum;
    if exe::os_get_free_disk_space_mb(0) < 0x19 {
        write_u32(G_GAME_IN_PROGRESS_FLAG, 1);
        return 0;
    }
    if no_memory(0x32) {
        write_u32(G_GAME_SESSION_STARTED_FLAG, 1);
        return 0;
    }
    create_game_find_cd();
    if read_usize(G_NATION_TABLE) == 0 {
        let mut data = [0u8; 0x104];
        copy_text(data.as_mut_ptr(), exe::utils_build_file_path(0x9b5348, 0xad9c64) as *const u8); // "CM3_DATA"
        if exe::load_database_from_files(data.as_ptr(), 1) == 0 {
            setup_warning(0x1e3d);
            return 0;
        }
        apply(read_usize(G_CONFIG));
    }
    call(FN_DATABASE_TRIM_SENIOR_AND_B_TEAM_SQUADS);
    if exe::crt_strstr(G_COMMAND_LINE_TEXT, 0x9899d4) != 0 {
        call(FN_VERIFY_BACKGROUND_CONFIG); // "-verifypics"
    }
    if exe::crt_strstr(G_COMMAND_LINE_TEXT, 0xa813d4) != 0 {
        call(FN_VERIFY_HISTORY_FILES); // "-verifyhist"
    }
    if exe::crt_strstr(G_COMMAND_LINE_TEXT, 0xa813c4) != 0 {
        player_setup(0x203b, 1); // "-verifyplcfg"
    }
    if exe::crt_strstr(G_COMMAND_LINE_TEXT, 0xa813b4) != 0 {
        call(FN_VALIDATE_EURO_CFG); // "-verifyeurocfg"
    }
    call(FN_DATABASE_BUILD_NATION_NAME_INDEX);
    call(FN_DATABASE_PRUNE_COMP_HISTORY_TABLE);
    let size = (read_u32(read_usize(G_CONFIG) + CONFIG_OPTIONS) & 7) as u8;
    write_u8(G_DATABASE_SIZE_OPTION, size);
    size_call(FN_DATABASE_TRIM_STAFF_TABLES, size as i8 as i32 as u32);
    size_call(FN_DATABASE_TRIM_CLUB_TABLE, read_u8(G_DATABASE_SIZE_OPTION) as u32);
    let greece = read_usize(G_NATION_TABLE) + read_u32(KEY_NATION_GREECE) as usize * nation::SIZE;
    if read_u8(greece + nation::LEAGUE_SELECTED) != 0 {
        fix_division(read_u32(KEY_CLUB_COMP_GREEK_PREMIER_DIVISION), 0xe);
        fix_division(read_u32(KEY_CLUB_COMP_GREEK_SECOND_DIVISION), 0xe);
    }
    call(FN_DATABASE_MERGE_B_TEAM_STAFF);
    let order = exe::crt_malloc(read_u32(G_CLUB_COUNT).wrapping_mul(4));
    write_usize(G_CLUB_REPUTATION_ORDER, order);
    if order == 0 {
        setup_fatal_error(0x1ea0);
    }
    exe::database_sort_clubs_by_reputation(1);
    call(FN_SETUP_SET_GAME_START_DATE);
    let (mut earlier, mut tmp) = ([0u8; 8], [0u8; 8]);
    let r = exe::date_sub_weeks(G_GAME_START_DATE, &mut earlier, 4);
    exe::date_operator_3d(G_GAME_START_DATE, tmp.as_mut_ptr() as usize, r);
    write_u32(G_STAFF_STATE_COUNT, read_u32(G_STAFF_COUNT));
    let states = exe::crt_malloc(read_u32(G_STAFF_COUNT).wrapping_mul(staff_state::SIZE as u32));
    write_usize(G_STAFF_STATE_TABLE, states);
    if states == 0 {
        setup_fatal_error(0x1eaf);
    }
    create_game_staff_states(states);
    write_u32(G_CLUB_STATE_COUNT, read_u32(G_CLUB_COUNT));
    let clubs = exe::crt_malloc(read_u32(G_CLUB_COUNT).wrapping_mul(6));
    write_usize(G_CLUB_STATE_TABLE, clubs);
    if clubs == 0 {
        setup_fatal_error(0x1f0c);
    }
    for i in 0..read_i32(G_CLUB_COUNT).max(0) as usize {
        let c = clubs + 6 * i;
        write_u32(c, 0xffffffff);
        write_u8(c + 4, 0);
        write_u8(c + 5, 0);
    }
    let mut now = 0i32;
    write_i32(G_GAME_PLAY_TIME_LAST_STAMP, exe::at_946ebb(&mut now));
    write_u32(G_GAME_PLAY_TIME_TOTAL, 0);
    for i in 0..read_i32(G_CLUB_COUNT).max(0) as usize {
        let club = read_usize(G_CLUB_TABLE) + i * club::SIZE;
        write_u32(club + club::CONTINENTAL_COMP_ID, 0xffffffff);
        for k in 0..0x32 {
            let p = read_usize(read_usize(G_CLUB_TABLE) + i * club::SIZE + club::SQUAD + 4 * k);
            if p != 0 && read_usize(p + staff::CLUB_JOB) == 0 {
                write_usize(p + staff::CLUB_JOB, read_usize(G_CLUB_TABLE) + i * club::SIZE);
            }
        }
    }
    initialise(G_CURRENT_DATE);
    if read_u8(G_PLAYER_SETUP_GERMAN_FLAG) != 0 || read_u8(G_USE_REAL_PLAYERS) == 0 {
        call(FN_DATABASE_APPLY_FAKE_PLAYERS);
    }
    // the progress bar: "Initialising game data", 0x2c steps
    let mut label = [0u8; 0xc8];
    text(label.as_mut_ptr(), 0x9cc2ac);
    let progress = |k: u32| exe::screenmanager_send_network_status_message(G_SCREEN_MANAGER, label.as_ptr() as usize, k as i32, 0x2c, -1);
    // disk and memory between the steps: sets the flag and returns true
    let short_of = |minimum_disk: i32| {
        if exe::os_get_free_disk_space_mb(0) < minimum_disk {
            write_u32(G_GAME_IN_PROGRESS_FLAG, 1);
            return true;
        }
        if no_memory(0x19) {
            write_u32(G_GAME_SESSION_STARTED_FLAG, 1);
            return true;
        }
        false
    };
    progress(1);
    exe::utils_clear_temp_folder(0xa813ac); // "*.tmp"
    let options = read_u32(read_usize(G_CONFIG) + CONFIG_OPTIONS);
    let matches: u32 = if options & 0x400 != 0 { 5000 } else if options & 0x800 != 0 { 20000 } else { 10000 };
    for &(k, step) in CREATE_STEPS.iter() {
        let disk = if k == 0xb {
            match matches.wrapping_mul(0xd7a) >> 20 {
                0 => 0x19,
                mb => mb as i32,
            }
        } else {
            0x19
        };
        if short_of(disk) {
            return 0;
        }
        progress(k);
        match step {
            CreateStep::Routine(f, row) => {
                let f: unsafe extern "C" fn() -> i32 = core::mem::transmute(f);
                if f() == 0 {
                    setup_warning(row);
                    return 0;
                }
            }
            CreateStep::Method(f, global) => {
                let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(f);
                f(read_usize(global));
            }
            CreateStep::Object(size, constructor, global, row, arg) => {
                let p = exe::crt_operator_new(size);
                let o = if p == 0 {
                    0
                } else {
                    match arg {
                        CreateArg::Zero => {
                            let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(constructor);
                            f(p, 0)
                        }
                        CreateArg::Alone => {
                            let f: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(constructor);
                            f(p)
                        }
                        CreateArg::Stats => {
                            let f: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(constructor);
                            f(p, 0, matches)
                        }
                        CreateArg::Fixture => {
                            let f: unsafe extern "thiscall" fn(usize, u32, usize, u32) -> usize = core::mem::transmute(constructor);
                            f(p, read_u16(G_CURRENT_DATE_YEAR) as u32, 0, 1)
                        }
                        CreateArg::Language => {
                            let f: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(constructor);
                            f(p, read_u8(G_DATABASE_NAMES_LANGUAGE) as u32)
                        }
                    }
                };
                write_usize(global, o);
                let checked = if global == G_FOG_OF_WAR { read_usize(G_MEDIA) } else { o };
                if checked == 0 {
                    setup_warning(row);
                    return 0;
                }
                if global == G_CONTRACT_MANAGER {
                    let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_CONTRACT_MANAGER_INITIALISE_ALL_STAFF_CONTRACTS);
                    f(o);
                }
            }
            CreateStep::Clubs => create_game_clubs(),
        }
    }
    call(FN_GAME_RUN_TO_START_DATE);
    write_u32(G_GAME_DATA_LOADED_FLAG, 1);
    write_u32(G_GAME_HAS_SAVE_FILE_FLAG, 0);
    if exe::utils_file_exists(0xa80340) == 0 {
        check_game_cd(false); // "auto_setup.cfg"
    }
    1
}

/// Comparer of `appoint_human_manager_to_club`: only the original runs (managers leave and are appointed, with their
/// news and the friendlies' plans; it cannot run twice). The copy is proved in Rust mode: the automatic career runs it
/// once, when its human manager is put on a club (`create_game_from_setup_file`).
pub(crate) unsafe extern "C" fn compare_appoint_human_manager_to_club(manager: usize, club: usize) -> u32 {
    original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(%%I0%%)(manager, club)
}

/// Comparer of `select_default_league_by_language`: side by side over the nations' league flags, the three league
/// counters and the main nation (the "Error" boxes only open without a database or with an unknown language: original
/// only then).
pub(crate) unsafe extern "C" fn compare_select_default_league_by_language() -> u32 {
    let o = original_fn::<unsafe extern "C" fn() -> u32>(%%I1%%);
    let nations = read_usize(G_NATION_TABLE);
    let count = read_i32(G_NATION_COUNT);
    if nations == 0 || count <= 0 || exe::langlib_get_language() > 12 {
        return o();
    }
    let regions = [(nations, count as usize * nation::SIZE), (G_NATIONS_TO_LOAD_COUNT, 12)];
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(), || select_default_league_by_language());
    SCOREBOARD.record(equal && r == n, || format!("select_default_league_by_language: original={} rust={} differences at {:?}", r, n, first_differences(&mo, &mr)));
    r
}

/// Comparer of `create_game`: only the original runs (it builds the whole game: it cannot run twice). The copy is proved
/// in Rust mode: the automatic career creates its game through it (`create_game_from_setup_file`).
pub(crate) unsafe extern "C" fn compare_create_game(quick_start: u32) -> u32 {
    original_fn::<unsafe extern "C" fn(u32) -> u32>(%%I2%%)(quick_start)
}
