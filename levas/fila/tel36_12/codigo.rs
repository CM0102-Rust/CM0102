// ---------- batch %%LEVA%%: Setup.cpp's static dates and the division club count fix (Setup.cpp) ----------

/// Rule: Setup.cpp's start date is a static Date built empty at program start.
///
/// `setup_static_init_start_date()`: `Date::Date` on 0xdd6ec8 and its destructor (`setup_static_destroy_start_date`,
/// empty) registered with `atexit`.
/// Original: `setup_static_init_start_date` (0x81b830, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn setup_static_init_start_date() {
    exe::date_date_54a110(0xdd6ec8usize);
    exe::crt_atexit(0x81b850usize);
}

/// The start date's destructor: nothing.
/// Original: `setup_static_destroy_start_date` (0x81b850, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn setup_static_destroy_start_date() {}

/// Rule: Setup.cpp's saved date is a static Date built empty at program start.
///
/// `setup_static_init_saved_date()`: `Date::Date` on 0xdd70a0 and its empty destructor registered with `atexit`.
/// Original: `setup_static_init_saved_date` (0x81b860, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn setup_static_init_saved_date() {
    exe::date_date_54a110(0xdd70a0usize);
    exe::crt_atexit(0x81b880usize);
}

/// The saved date's destructor: nothing.
/// Original: `setup_static_destroy_saved_date` (0x81b880, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn setup_static_destroy_saved_date() {}

/// Rule: when a new game loads Greece, each Greek division must have exactly its number of clubs (14): clubs past the
/// number go to the lower division placeholder, and missing places are filled with clubs of the nation that have no
/// division, play in the placeholder or Premier divisions, or play in a division of lower reputation.
///
/// `fix_division_club_count(comp_id, expected)`: the clubs in table order, stopping at the first national team
/// (`database_is_national_team`). First pass: a club whose `club::DIVISION` is the competition counts while the count
/// is below `expected`; past it the club's division becomes `KEY_CLUB_COMP_A_LOWER_DIVISION`'s competition and the
/// text "DB_ERROR: Found 1 more team in db for <name>" is written (C `sprintf`) into the screens' text buffer. With
/// fewer than `expected`: "DB_ERROR: Not enough teams in database for <name> (Found n, Expected m)". Second pass, while
/// the count is below `expected` (stopping at a national team): a club of the competition's nation (`club::NATION` ==
/// `competition::NATION`) whose division is none, the lower placeholder, the Premier placeholder
/// (`KEY_CLUB_COMP_A_PREMIER_DIVISION`) or one of lower `competition::REPUTATION` moves to the competition and counts.
/// The texts are never shown (only written to the buffer).
/// Original: `fix_division_club_count` (0x839920, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn fix_division_club_count(comp_id: i32, expected: u32) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let expected = expected as i16;
    let comps = || read_usize(G_CLUB_COMP_TABLE);
    let comp_at = |id: u32| comps().wrapping_add(id.wrapping_mul(TEL36_CLUB_COMP_SIZE) as usize);
    let comp = comp_at(comp_id as u32);
    let mut found = 0i16;
    let mut club = read_usize(G_CLUB_TABLE);
    for _ in 0..read_i32(G_CLUB_COUNT) {
        if exe::database_is_national_team(club) != 0 {
            break;
        }
        let division = read_usize(club + club::DIVISION);
        if division != 0 && read_i32(division) == comp_id {
            if found < expected {
                found += 1;
            } else {
                covered!("tel36_12 one more club in the division");
                write_usize(club + club::DIVISION, comp_at(read_u32(KEY_CLUB_COMP_A_LOWER_DIVISION)));
                format(JOB_TEXT, 0xa82578, comp_at(comp_id as u32) + competition::NAME);
            }
        }
        club += club::SIZE;
    }
    if found < expected {
        covered!("tel36_12 not enough clubs in the division");
        format(JOB_TEXT, 0xa82530, comp + competition::NAME, found as i32, expected as i32);
    }
    let mut club = read_usize(G_CLUB_TABLE);
    for _ in 0..read_i32(G_CLUB_COUNT) {
        if found >= expected || exe::database_is_national_team(club) != 0 {
            return;
        }
        let nation = read_u32(club + club::NATION);
        if nation != 0 && nation == read_u32(comp_at(comp_id as u32) + competition::NATION) {
            let division = read_usize(club + club::DIVISION);
            let takes = division == 0 || {
                let id = read_u32(division);
                id == read_u32(KEY_CLUB_COMP_A_LOWER_DIVISION) || id == read_u32(KEY_CLUB_COMP_A_PREMIER_DIVISION)
                    || read_i16(division + competition::REPUTATION) < read_i16(comp_at(comp_id as u32) + competition::REPUTATION)
            };
            if takes {
                found += 1;
                write_usize(club + club::DIVISION, comp_at(comp_id as u32));
            }
        }
        club += club::SIZE;
    }
}

/// Comparer for `setup_static_init_start_date`.
pub(crate) unsafe extern "C" fn compare_setup_static_init_start_date() {
    // simplification: the atexit registration does not run twice
    original_fn::<unsafe extern "C" fn()>(%%I0%%)()
}

/// Comparer for `setup_static_destroy_start_date`.
pub(crate) unsafe extern "C" fn compare_setup_static_destroy_start_date() {
    original_fn::<unsafe extern "C" fn()>(%%I1%%)()
}

/// Comparer for `setup_static_init_saved_date`.
pub(crate) unsafe extern "C" fn compare_setup_static_init_saved_date() {
    // simplification: the atexit registration does not run twice
    original_fn::<unsafe extern "C" fn()>(%%I2%%)()
}

/// Comparer for `setup_static_destroy_saved_date`.
pub(crate) unsafe extern "C" fn compare_setup_static_destroy_saved_date() {
    original_fn::<unsafe extern "C" fn()>(%%I3%%)()
}

/// Comparer for `fix_division_club_count`: the club table and the text buffer side by side.
pub(crate) unsafe extern "C" fn compare_fix_division_club_count(comp_id: i32, expected: u32) {
    let o = original_fn::<unsafe extern "C" fn(i32, u32)>(%%I4%%);
    let reg = [(read_usize(G_CLUB_TABLE), read_i32(G_CLUB_COUNT).max(0) as usize * club::SIZE), (JOB_TEXT, 0x100)];
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&reg, || o(comp_id, expected), || fix_division_club_count(comp_id, expected));
    SCOREBOARD.record(same, || format!("fix_division_club_count({}, {}): differences {:?}", comp_id, expected as i16, first_differences(&mo, &mr)));
}
