// ---------- batch %%LEVA%%: the year ratings filters of the award screens and the human manager's menu bar record (award_screens.cpp, Menubar.cpp) ----------

/// The award year ratings filter of the world ratings: a player counts when his current reputation is over 2000.
///
/// `award_rating_filter_reputation_over_2000(person, fixture)`: 1 when the person, his player sheet
/// (`staff::PLAYER`) and the sheet's `player_sheet::CURRENT_REPUTATION` (signed word) > 2000; else 0. The fixture is
/// not read.
/// Original: `award_rating_filter_reputation_over_2000` (0x414b20, award_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn award_rating_filter_reputation_over_2000(person: usize, _fixture: usize) -> u32 {
    if person == 0 {
        return 0;
    }
    covered!("tel39_3 year ratings filter");
    let sheet = read_usize(person + staff::PLAYER);
    // tel39_3: the reputation threshold
    (sheet != 0 && read_i16(sheet + player_sheet::CURRENT_REPUTATION) > 2000) as u32
}

/// Whether the nation record `nation` (a pointer, may be null) is the fixed nation of `key` (the key holds its id).
unsafe fn award_nation_is(nation: usize, key: usize) -> bool {
    nation != 0 && read_u32(nation) == read_u32(key)
}

/// The award year ratings filter of the second ratings: a player counts when he, or the fixture's competition,
/// belongs to Denmark, Belgium, France, Holland or Sweden.
///
/// `award_rating_filter_selected_nations(person, fixture)`, in the original's order: the person's `staff::NATION` is
/// Denmark; the fixture's `fixture::COMPETITION`'s `competition::NATION` is Belgium; the person's nation or the
/// competition's nation is France; the competition's nation is Holland (a Dutch player in a foreign competition does
/// not count, unlike France and Sweden); the person's or the competition's nation is Sweden (each nation by its fixed
/// key, the id compared). A null person or fixture, competition or nation skips that test. 1 when one holds, else 0.
/// Original: `award_rating_filter_selected_nations` (0x414b40, award_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn award_rating_filter_selected_nations(person: usize, fixture: usize) -> u32 {
    let own = |key: usize| person != 0 && award_nation_is(read_usize(person + staff::NATION), key);
    let comp = |key: usize| {
        if fixture == 0 {
            return false;
        }
        let c = read_usize(fixture + fixture::COMPETITION);
        c != 0 && award_nation_is(read_usize(c + competition::NATION), key)
    };
    // tel39_3: Denmark by the person, Belgium by the competition, France by either, Holland by the competition,
    // Sweden by either
    (own(KEY_NATION_DENMARK) || comp(KEY_NATION_BELGIUM) || own(KEY_NATION_FRANCE) || comp(KEY_NATION_FRANCE)
        || comp(KEY_NATION_HOLLAND) || own(KEY_NATION_SWEDEN) || comp(KEY_NATION_SWEDEN)) as u32
}

/// The menu bar record of a human manager: the one of the current slot's manager when `manager` is null; null when
/// there is none or his id is not among the last 16 staff records.
///
/// `get_human_manager_record(manager)`: a null manager is replaced by the slot's manager ([G_SCREEN_RECORDS_MANAGER +
/// slot × 0x300]); null stays null. k = id − G_STAFF_COUNT + 16; 0 ≤ k < 16 gives record k, else null (`MENUBAR_RECORDS`, 0xf6 bytes each).
/// Original: `get_human_manager_record` (0x765480, Menubar.cpp, cdecl).
pub(crate) unsafe extern "C" fn get_human_manager_record(manager: usize) -> usize {
    let mut m = manager;
    if m == 0 {
        m = read_usize((G_SCREEN_RECORDS_MANAGER as isize + read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize * 0x300) as usize);
        if m == 0 {
            return 0;
        }
    }
    covered!("tel39_3 human record");
    // tel39_3: the position among the last 16 staff records
    let k = read_i32(m).wrapping_sub(read_i32(G_STAFF_COUNT)).wrapping_add(16);
    if !(0..16).contains(&k) {
        return 0;
    }
    MENUBAR_RECORDS + k as usize * MENUBAR_RECORD_SIZE
}

/// Comparer for `award_rating_filter_reputation_over_2000`: the answer side by side (nothing is written).
pub(crate) unsafe extern "C" fn compare_award_rating_filter_reputation_over_2000(person: usize, fixture: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(IDX_AWARD_RATING_FILTER_REPUTATION_OVER_2000)(person, fixture);
    let n = award_rating_filter_reputation_over_2000(person, fixture);
    SCOREBOARD.record(r == n, || format!("award_rating_filter_reputation_over_2000({:#x}) original={} rust={}", person, r, n));
    r
}

/// Comparer for `award_rating_filter_selected_nations`: the answer side by side (nothing is written).
pub(crate) unsafe extern "C" fn compare_award_rating_filter_selected_nations(person: usize, fixture: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(IDX_AWARD_RATING_FILTER_SELECTED_NATIONS)(person, fixture);
    let n = award_rating_filter_selected_nations(person, fixture);
    SCOREBOARD.record(r == n, || format!("award_rating_filter_selected_nations({:#x}, {:#x}) original={} rust={}", person, fixture, r, n));
    r
}

/// Comparer for `get_human_manager_record`: the record side by side (nothing is written).
pub(crate) unsafe extern "C" fn compare_get_human_manager_record(manager: usize) -> usize {
    let r = original_fn::<unsafe extern "C" fn(usize) -> usize>(IDX_GET_HUMAN_MANAGER_RECORD)(manager);
    let n = get_human_manager_record(manager);
    SCOREBOARD.record(r == n, || format!("get_human_manager_record({:#x}) original={:#x} rust={:#x}", manager, r, n));
    r
}
