// ---------- batch %%LEVA%%: the Setup screens, part 8: the game start date, the competition object table, the network game and the nation filters (Setup.cpp) ----------

// (uses setup_warning of setup.rs and SETUP_TEXT, G_SETUP_SCREEN_TITLE, G_NETWORK_SETUP_HOST_FLAG, setup_format_title,
// setup_open_screen, setup_confirm_message and FN_SETUP_ADD_HUMAN_MANAGER of tel11_1, G_SCREEN_MANAGER_NETWORK of
// tel11_2, G_SCREEN_MANAGER_SLOT_RECORD_COUNTS of tel11_3)
const FN_FRIENDLY_REGISTER_COMP_OBJECTS: usize = 0x5ca030; // Friendly::register_comp_objects thiscall(Friendly)
const FN_NETWORK_HOST_OR_JOIN_GAME: usize = 0x77a9f0; // Network::host_or_join_game thiscall(network, join, name) ret 8 -> int
const FN_DISPLAY_ALLOC_GUIO_SNAPSHOT_POOL: usize = 0x55f1c0; // display_alloc_guio_snapshot_pool stdcall(slots) ret 4
const FN_OPEN_CLUB_SQUAD_SCREEN: usize = 0x453d60; // open_club_squad_screen(club, reserves, restore_params, alt_view, locked)
const FN_MATCH_DAY_DROP_VIEWER_AND_FINISH_MATCH: usize = 0x6b6970; // match_day_drop_viewer_and_finish_match()
const G_SCREEN_MANAGER_NETWORK_HOST_FLAG: usize = 0xb77c80; // g_screen_manager_network_host_flag: 1 when this machine hosts
const KEY_NATION_ENTRY_COUNT: usize = 35; // the rows of G_LEAGUE_TABLE
const KEY_NATION_RECORD_SIZE: usize = 0x48; // the size of a G_LEAGUE_TABLE row
const KEY_NATION_RECORD_NATION: usize = 0x0; // the nation of the entry (null: an international entry)
const KEY_NATION_RECORD_CONTINENT: usize = 0x4; // the continent of a continental club cups entry (key_nation_continent_has_selected_nation)
const KEY_NATION_RECORD_COMP_COUNT: usize = 0xc; // the number of competitions the entry created
const KEY_NATION_RECORD_COMPS: usize = 0x10; // the entry's array of competition objects (calloc)
const KEY_NATION_RECORD_SEASON_YEAR: usize = 0x1b; // 2 bytes: the season year the entry starts from
const KEY_NATION_RECORD_CUP_COMP: usize = 0x1e; // the competition record of the entry's cup [inference: only Ireland's and Northern Ireland's set it, to their FA cups]
const KEY_NATION_RECORD_START_DAY: usize = 0x32; // 2 bytes: Date::Date day of the season start (with START_MONTH, START_YEAR, START_WEEKDAY -> START_DATE)
const KEY_NATION_RECORD_START_YEAR: usize = 0x35; // 2 bytes: Date::Date year of the season start
const KEY_NATION_RECORD_START_WEEKDAY: usize = 0x37; // 2 bytes: Date::Date weekday of the season start
const KEY_NATION_RECORD_END_DAY: usize = 0x41; // 2 bytes: Date::Date day of the season end (-> END_DATE)
const KEY_NATION_RECORD_END_MONTH: usize = 0x43; // 1 byte: Date::Date month of the season end
const KEY_NATION_RECORD_END_YEAR: usize = 0x44; // 2 bytes: Date::Date year of the season end
const KEY_NATION_RECORD_END_WEEKDAY: usize = 0x46; // 2 bytes: Date::Date weekday of the season end
const HUMAN_MANAGER_COUNT_8: usize = 0x8; // a HUMAN_MANAGER count tested by the join hook [inference: humans already in the game]

/// Sets the new game's start date (`G_GAME_START_DATE`) from the key nation entries; returns 1, or 0 when no entry
/// fits (the date stays).
///
/// The 35 entries of `G_LEAGUE_TABLE` are walked in order, skipping those without a nation
/// (`KEY_NATION_RECORD_NATION`). The entry of the main nation (`G_MAIN_NATION`) gives its `key_nation_record::START_DATE`
/// (`Date::operator=`); then, for a European main nation (the id of its `nation::CONTINENT` is `KEY_CONTINENT_EUROPE`'s)
/// in a 2001 start, a start after Date::Date(1, 8, 2001, −1) (1 September 2001: the later year, unsigned, or the same
/// year and a later day, signed) is brought back to it. Without a main nation, the first entry whose nation has bit 1
/// of `nation::LEAGUE_SELECTED` gives its start date as it is.
/// Original: `setup_set_game_start_date` (0x830520, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn setup_set_game_start_date() -> u32 {
    covered!("tel13_1 game start date");
    let main = read_usize(G_MAIN_NATION);
    for k in 0..KEY_NATION_ENTRY_COUNT {
        let entry = G_LEAGUE_TABLE + k * KEY_NATION_RECORD_SIZE;
        let nation = read_usize(entry + KEY_NATION_RECORD_NATION);
        if nation == 0 {
            continue;
        }
        let mut tmp = [0u8; 8];
        if nation == main {
            exe::date_operator_3d(G_GAME_START_DATE, tmp.as_mut_ptr() as usize, entry + key_nation_record::START_DATE);
            if read_u32(Nation(nation).continent()) == read_u32(KEY_CONTINENT_EUROPE) && read_u16(G_GAME_START_DATE_YEAR) == 0x7d1 {
                let mut september = [0u16; 4];
                exe::date_date_549f70(&mut september, 1, 8, 0x7d1, 0xffff_ffff);
                let start_year = read_u16(G_GAME_START_DATE_YEAR);
                let later = september[1] < start_year || (september[1] == start_year && (september[0] as i16) < read_i16(G_GAME_START_DATE));
                if later {
                    let mut again = [0u16; 4];
                    let d = exe::date_date_549f70(&mut again, 1, 8, 0x7d1, 0xffff_ffff);
                    exe::date_operator_3d(G_GAME_START_DATE, tmp.as_mut_ptr() as usize, d);
                }
            }
            return 1;
        }
        if main == 0 && Nation(nation).league_selected() & 2 != 0 {
            exe::date_operator_3d(G_GAME_START_DATE, tmp.as_mut_ptr() as usize, entry + key_nation_record::START_DATE);
            return 1;
        }
    }
    0
}

/// Builds the table of competition objects by competition id; returns 1, or 0 when it cannot be allocated (the
/// Setup.cpp "Error" box 0x3896).
///
/// `G_COMP_OBJECT_COUNT` = `G_CLUB_COMP_COUNT` and `G_COMP_OBJECT_TABLE` = calloc(count × 4, 1). Each key nation entry
/// with a competition array (`KEY_NATION_RECORD_COMPS`) puts its non-null objects (up to `KEY_NATION_RECORD_COMP_COUNT`,
/// signed, read again each turn) at the id of their competition record (`league::COMPETITION`); then the friendlies
/// add theirs (`Friendly::register_comp_objects` on `G_FRIENDLY`).
/// Original: `build_comp_object_table` (0x8386f0, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn build_comp_object_table() -> u32 {
    covered!("tel13_1 comp object table");
    let count = read_u32(G_CLUB_COMP_COUNT);
    write_u32(G_COMP_OBJECT_COUNT, count);
    let table = exe::crt_calloc(count.wrapping_mul(4), 1);
    write_usize(G_COMP_OBJECT_TABLE, table);
    if table == 0 {
        setup_warning(0x3896);
        return 0;
    }
    for k in 0..KEY_NATION_ENTRY_COUNT {
        let entry = G_LEAGUE_TABLE + k * KEY_NATION_RECORD_SIZE;
        if read_usize(entry + KEY_NATION_RECORD_COMPS) == 0 {
            continue;
        }
        let mut i = 0i32;
        while i < read_i32(entry + KEY_NATION_RECORD_COMP_COUNT) {
            let comp = read_usize(read_usize(entry + KEY_NATION_RECORD_COMPS) + i as usize * 4);
            if comp != 0 {
                let id = read_u32(read_usize(comp + league::COMPETITION)) as usize;
                write_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(id.wrapping_mul(4)), comp);
            }
            i += 1;
        }
    }
    let register: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_FRIENDLY_REGISTER_COMP_OBJECTS);
    register(read_usize(G_FRIENDLY));
    1
}

/// Creates (`join` 0) or joins the network game `game_name`; returns 0 when it worked, −4 when not (0 with a null name,
/// after the Setup.cpp "Error" box 0x391a).
///
/// The status message "Creating network game ..." or "Joining network game ..." (langlib into `SETUP_TEXT`) goes out
/// (`ScreenManager::send_network_status_message`, 0, 0, −1); `G_SCREEN_MANAGER_NETWORK_HOST_FLAG` = 1 when creating;
/// the widget snapshot pool gets 16 slots when creating, 1 when joining (`display_alloc_guio_snapshot_pool`; the
/// original loads the remote Display into ecx, which the stdcall ignores). With a network object
/// (`G_SCREEN_MANAGER_NETWORK`) `Network::host_or_join_game`(join, name) nonzero means done. Otherwise a game in memory
/// (`G_COMP_OBJECT_TABLE`) is freed (`free_game_data`(0)) and the "Unable To Create" / "Unable To Join" box opens with
/// "The network game "<name>" could not be created." / "... could not be joined." (`setup_confirm_message`).
/// Original: `setup_create_or_join_network_game` (0x8388e0, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn setup_create_or_join_network_game(join: u32, game_name: usize) -> i32 {
    covered!("tel13_1 network game");
    if game_name == 0 {
        setup_warning(0x391a);
        return 0;
    }
    let text: unsafe extern "C" fn(usize, usize, ...) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
    text(SETUP_TEXT, if join == 0 { 0xa823ec } else { 0xa823d0 }); // "Creating network game ...", "Joining network game ..."
    exe::screenmanager_send_network_status_message(G_SCREEN_MANAGER, SETUP_TEXT, 0, 0, -1);
    write_u32(G_SCREEN_MANAGER_NETWORK_HOST_FLAG, (join == 0) as u32);
    let pool: unsafe extern "stdcall" fn(u32) = core::mem::transmute(FN_DISPLAY_ALLOC_GUIO_SNAPSHOT_POOL);
    pool(if join == 0 { 0x10 } else { 1 });
    let network = read_usize(G_SCREEN_MANAGER_NETWORK);
    if network != 0 {
        let host_or_join: unsafe extern "thiscall" fn(usize, u32, usize) -> i32 = core::mem::transmute(FN_NETWORK_HOST_OR_JOIN_GAME);
        if host_or_join(network, join, game_name) != 0 {
            return 0;
        }
    }
    if read_usize(G_COMP_OBJECT_TABLE) != 0 {
        let free: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(FN_FREE_GAME_DATA);
        free(0);
    }
    let mut heading = [0u8; 0x100];
    if join == 0 {
        setup_confirm_message(&mut heading, 0x9c7c98, 0xa8238c, game_name); // "Unable To Create"
    } else {
        setup_confirm_message(&mut heading, 0xa82338, 0xa82348, game_name); // "Unable To Join"
    }
    -4
}

/// The network hook for a player who joined (`player_id`); returns nothing.
///
/// With a positive `HUMAN_MANAGER_COUNT_8` or `human_manager_fields::HOLIDAY_COUNT` on `G_HUMAN_MANAGER`, Manager
/// Status opens (`setup_open_screen` with its five params zeroed) and that is all. In a running game
/// (`G_INTERACTIVE_MODE_FLAG`) the squad screen opens (`open_club_squad_screen`(0, 0, 0, 0, 0)) and a human manager is
/// added (`setup_add_human_manager`). Otherwise, when the current slot goes over the network
/// (`screen_manager::NETWORK_SLOT` of slot `G_SCREEN_MANAGER_CURRENT_SLOT`) and the player has exactly one record
/// (`G_SCREEN_MANAGER_SLOT_RECORD_COUNTS`), Manager Status opens; on the host (`G_NETWORK_SETUP_HOST_FLAG`) that is all,
/// on a client a human manager is added after it (whether it opened or not); in every other case a human manager is
/// added.
/// Original: `setup_on_network_player_joined` (0x838a80, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn setup_on_network_player_joined(player_id: i16) {
    covered!("tel13_1 player joined");
    let add: unsafe extern "C" fn() = core::mem::transmute(FN_SETUP_ADD_HUMAN_MANAGER);
    let humans = read_usize(G_HUMAN_MANAGER);
    if read_i32(humans + HUMAN_MANAGER_COUNT_8) > 0 || read_i32(humans + human_manager_fields::HOLIDAY_COUNT) > 0 {
        setup_open_screen(0x81ff10, 0x820950, 0, 5);
        return;
    }
    if read_u32(G_INTERACTIVE_MODE_FLAG) != 0 {
        let squad: unsafe extern "C" fn(usize, u32, u32, u32, u32) = core::mem::transmute(FN_OPEN_CLUB_SQUAD_SCREEN);
        squad(0, 0, 0, 0, 0);
        add();
        return;
    }
    let slot = read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize * 0x300;
    if read_u16(G_SCREEN_MANAGER + slot + screen_manager::NETWORK_SLOT) != 0
        && read_u16(G_SCREEN_MANAGER_SLOT_RECORD_COUNTS + player_id as isize as usize * 2) == 1
    {
        if read_u32(G_NETWORK_SETUP_HOST_FLAG) != 0 {
            setup_open_screen(0x81ff10, 0x820950, 0, 5);
            return;
        }
        setup_open_screen(0x81ff10, 0x820950, 0, 5);
    }
    add();
}

/// The network hook for a player who left; returns nothing.
///
/// The manager of the current slot (`G_SCREEN_RECORDS_MANAGER` of slot `G_SCREEN_MANAGER_CURRENT_SLOT`), when there is
/// one: with `staff::CLASSIFICATION` 0 or 5 it becomes 0; otherwise, with a `G_HUMAN_MANAGER` that is not on holiday
/// (`HUMAN_MANAGER::is_on_holiday`(0)), `HUMAN_MANAGER::set_on_holiday`(1, 0). Then the viewer leaves the match
/// (`match_day_drop_viewer_and_finish_match`, the original's tail jump).
/// Original: `setup_on_network_player_left` (0x838c00, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn setup_on_network_player_left() {
    covered!("tel13_1 player left");
    let slot = read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize * 0x300;
    let manager = Staff(read_usize(G_SCREEN_RECORDS_MANAGER + slot));
    if manager.0 != 0 {
        let classification = manager.classification();
        if classification == 0 || classification == 5 {
            manager.set_classification(0);
        } else {
            let humans = read_usize(G_HUMAN_MANAGER);
            if humans != 0 && exe::human_manager_is_on_holiday(humans, 0usize) == 0 {
                exe::human_manager_set_on_holiday(humans, 1, 0usize);
            }
        }
    }
    let finish: unsafe extern "C" fn() = core::mem::transmute(FN_MATCH_DAY_DROP_VIEWER_AND_FINISH_MATCH);
    finish();
}

/// Whether `nation` passes the region filter text `filter`: 1 or 0.
///
/// "ALL" anywhere: 1. ", EUROPE" or ",EUROPE" anywhere, or a filter starting with "EUROPE": the nation (its id against
/// the key's) is England, Scotland, Northern Ireland, the Republic of Ireland, Wales, France, Spain, Italy, Germany,
/// Holland, Belgium, Portugal, Turkey or Greece. Else "EASTERN_EUROPE": Russia, Croatia or Poland. Else "SCANDINAVIA":
/// Sweden, Norway, Finland or Denmark. Else "AMERICAS": Brazil, Argentina or the United States. Else "ASIA": Australia
/// or Japan. Anything else: 0.
/// Original: `nation_matches_region_filter` (0x839340, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn nation_matches_region_filter(nation: usize, filter: usize) -> u32 {
    covered!("tel13_1 region filter");
    let has = |text: usize| exe::crt_strstr(filter, text);
    let one_of = |keys: &[usize]| keys.iter().any(|&k| read_u32(nation) == read_u32(k)) as u32;
    if has(0xa82514) != 0 {
        return 1; // "ALL"
    }
    if has(0xa82508) != 0 || has(0xa82500) != 0 || has(0x9d6814) == filter {
        // ", EUROPE", ",EUROPE", "EUROPE"
        return one_of(&[
            KEY_NATION_ENGLAND, KEY_NATION_SCOTLAND, KEY_NATION_NORTHERN_IRELAND, KEY_NATION_REPUBLIC_OF_IRELAND, KEY_NATION_WALES,
            KEY_NATION_FRANCE, KEY_NATION_SPAIN, KEY_NATION_ITALY, KEY_NATION_GERMANY, KEY_NATION_HOLLAND, KEY_NATION_BELGIUM,
            KEY_NATION_PORTUGAL, KEY_NATION_TURKEY, KEY_NATION_GREECE,
        ]);
    }
    if has(0xa824f0) != 0 {
        return one_of(&[KEY_NATION_RUSSIA, KEY_NATION_CROATIA, KEY_NATION_POLAND]); // "EASTERN_EUROPE"
    }
    if has(0xa824e4) != 0 {
        return one_of(&[KEY_NATION_SWEDEN, KEY_NATION_NORWAY, KEY_NATION_FINLAND, KEY_NATION_DENMARK]); // "SCANDINAVIA"
    }
    if has(0xa824d8) != 0 {
        return one_of(&[KEY_NATION_BRAZIL, KEY_NATION_ARGENTINA, KEY_NATION_UNITED_STATES]); // "AMERICAS"
    }
    if has(0x9d681c) != 0 {
        return one_of(&[KEY_NATION_AUSTRALIA, KEY_NATION_JAPAN]); // "ASIA"
    }
    0
}

/// The qsort comparison of two nation pointers by nationality: a null first one sorts after (1), a null second one
/// before (−1); otherwise `utils_string_compare` of their `nation::NATIONALITY` texts (1, 0).
/// Original: `compare_nations_by_nationality` (0x839530, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_nations_by_nationality(a: usize, b: usize) -> i32 {
    covered!("tel13_1 nations by nationality");
    let (x, y) = (read_usize(a), read_usize(b));
    if x == 0 {
        return 1;
    }
    if y == 0 {
        return -1;
    }
    exe::utils_string_compare((x + nation::NATIONALITY) as *const u8, (y + nation::NATIONALITY) as *const u8, 1, 0)
}

/// The game title: "Championship Manager 2001/02" translated into `G_SETUP_SCREEN_TITLE` (`setup_format_title`),
/// which is returned.
/// Original: `get_game_title` (0x839570, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn get_game_title() -> usize {
    covered!("tel13_1 game title");
    setup_format_title();
    G_SETUP_SCREEN_TITLE
}

/// Comparer of `setup_set_game_start_date`: side by side over `G_GAME_START_DATE` and the result.
pub(crate) unsafe extern "C" fn compare_setup_set_game_start_date() -> u32 {
    let o = original_fn::<unsafe extern "C" fn() -> u32>(%%I0%%);
    let regions = [(G_GAME_START_DATE, 8)];
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(), || setup_set_game_start_date());
    SCOREBOARD.record(equal && r == n, || format!("setup_set_game_start_date: original={} rust={} differences at {:?}", r, n, first_differences(&mo, &mr)));
    r
}

/// Comparer of `build_comp_object_table`: only the original runs (it allocates the table); then the plan: the count is
/// `G_CLUB_COMP_COUNT` and every competition object of the key nation entries sits at its id (unless a later one, or a
/// friendly, has the same id).
pub(crate) unsafe extern "C" fn compare_build_comp_object_table() -> u32 {
    let r = original_fn::<unsafe extern "C" fn() -> u32>(%%I1%%)();
    let table = read_usize(G_COMP_OBJECT_TABLE);
    if r == 0 || table == 0 {
        return r;
    }
    let mut ok = read_u32(G_COMP_OBJECT_COUNT) == read_u32(G_CLUB_COMP_COUNT);
    let mut missing = 0usize;
    for k in 0..KEY_NATION_ENTRY_COUNT {
        let entry = G_LEAGUE_TABLE + k * KEY_NATION_RECORD_SIZE;
        let comps = read_usize(entry + KEY_NATION_RECORD_COMPS);
        if comps == 0 {
            continue;
        }
        for i in 0..read_i32(entry + KEY_NATION_RECORD_COMP_COUNT).max(0) as usize {
            let comp = read_usize(comps + i * 4);
            if comp != 0 && read_usize(table + read_u32(read_usize(comp + league::COMPETITION)) as usize * 4) == 0 {
                missing += 1;
            }
        }
    }
    ok &= missing == 0;
    SCOREBOARD.record(ok, || format!("build_comp_object_table: count {} of {}, {} objects missing", read_u32(G_COMP_OBJECT_COUNT), read_u32(G_CLUB_COMP_COUNT), missing));
    r
}

/// Comparer of `setup_create_or_join_network_game`: only the original runs (network, messages, boxes).
pub(crate) unsafe extern "C" fn compare_setup_create_or_join_network_game(join: u32, game_name: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(u32, usize) -> i32>(%%I2%%)(join, game_name)
}

/// Comparer of `setup_on_network_player_joined`: only the original runs (it opens screens and adds managers).
pub(crate) unsafe extern "C" fn compare_setup_on_network_player_joined(player_id: i16) {
    original_fn::<unsafe extern "C" fn(i16)>(%%I3%%)(player_id)
}

/// Comparer of `setup_on_network_player_left`: only the original runs (holidays and the match viewer).
pub(crate) unsafe extern "C" fn compare_setup_on_network_player_left() {
    original_fn::<unsafe extern "C" fn()>(%%I4%%)()
}

/// Comparer of `nation_matches_region_filter`: both run (it only reads) and the results must match.
pub(crate) unsafe extern "C" fn compare_nation_matches_region_filter(nation: usize, filter: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(%%I5%%)(nation, filter);
    if nation == 0 || filter == 0 {
        return r;
    }
    let n = nation_matches_region_filter(nation, filter);
    SCOREBOARD.record(r == n, || format!("nation_matches_region_filter(nation {:#x}, filter {:#x}): original={} rust={}", nation, filter, r, n));
    r
}

/// Comparer of `compare_nations_by_nationality`: both run (it only reads) and the results must match.
pub(crate) unsafe extern "C" fn compare_compare_nations_by_nationality(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I6%%)(a, b);
    let n = compare_nations_by_nationality(a, b);
    SCOREBOARD.record(r == n, || format!("compare_nations_by_nationality({:#x}, {:#x}): original={} rust={}", a, b, r, n));
    r
}

/// Comparer of `get_game_title`: side by side over the title buffer (up to `G_NETWORK_GAME_NAME`) and the result.
pub(crate) unsafe extern "C" fn compare_get_game_title() -> usize {
    let o = original_fn::<unsafe extern "C" fn() -> usize>(%%I7%%);
    let regions = [(G_SETUP_SCREEN_TITLE, 0x34)];
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(), || get_game_title());
    SCOREBOARD.record(equal && r == n, || format!("get_game_title: original={:#x} rust={:#x} differences at {:?}", r, n, first_differences(&mo, &mr)));
    r
}
