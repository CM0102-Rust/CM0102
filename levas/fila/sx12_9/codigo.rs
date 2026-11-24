// ---------- batch %%LEVA%%: the season labels and the extra division names of the key nations (key_nation.cpp; sx12_9) ----------

/// Writes "{year}/{(year + 1) mod 10}" for a season spanning two years, else "{year}"; a year 0 means the season of
/// today, the year before when today is before the row's season end day (`key_nation_record::SEASON_END_DAY`).
unsafe fn write_season_label(text: usize, year: u16, row: Option<KeyNationRecord>) {
    match row.filter(|r| r.split_season_flag() != 0) {
        Some(r) => {
            let mut y = year;
            if y == 0 {
                y = read_u16(G_CURRENT_DATE_YEAR);
                if (read_i16(G_CURRENT_DATE) as i32) < r.season_end_day() as i32 {
                    y = y.wrapping_sub(1); // still the season that started last year
                }
            }
            covered!("sx12_9 two-year season");
            story_crt!(text, 0x9eb024, y as u32, (y as u32 + 1) as i32 % 10); // sx12_9: "%d/%d"
        }
        None => {
            let y = if year == 0 { read_u16(G_CURRENT_DATE_YEAR) } else { year };
            story_crt!(text, 0x98806c, y as u32); // "%d"
        }
    }
}

/// Writes the season label for a club: two years ("2001/2") when its key nation row (`exe::key_nation_index_of_club`)
/// spans two years (`key_nation_record::SPLIT_SEASON_FLAG`), else the year; returns 1.
///
/// `key_nation_format_club_season(year, club, text, size)`: a null text or a size under 10 shows the "Error" box
/// (key_nation.cpp line 0x30b) and returns 0. A null club or a national team (`exe::database_is_national_team`): the
/// plain year. Year 0: the season of today (`write_season_label`).
/// Original: `key_nation_format_club_season` (0x669630, key_nation.cpp, cdecl).
pub(crate) unsafe extern "C" fn key_nation_format_club_season(year: u32, club: usize, text: usize, size: i32) -> u32 {
    if text == 0 || size < 0xa {
        show_error_at(0x9eaff4, 0x30b); // "...\key_nation.cpp"
        return 0;
    }
    let row = if club != 0 && exe::database_is_national_team(club) == 0 {
        Some(KeyNationRecord(G_LEAGUE_TABLE.wrapping_add((exe::key_nation_index_of_club(club) as i8 as isize * 72) as usize)))
    } else {
        None
    };
    write_season_label(text, year as u16, row);
    1
}

/// Writes the season label for a competition: two years when its key nation row (`exe::key_nation_index_of_comp`, not
/// the world rows 0 and 1) spans two years, else the year; returns 1.
///
/// `key_nation_format_comp_season(year, comp, text, size)`: a null text or a size under 10 shows the "Error" box
/// (key_nation.cpp line 0x350) and returns 0. A null competition: the plain year. Year 0: the season of today.
/// Original: `key_nation_format_comp_season` (0x6697c0, key_nation.cpp, cdecl).
pub(crate) unsafe extern "C" fn key_nation_format_comp_season(year: u32, comp: usize, text: usize, size: i32) -> u32 {
    if text == 0 || size < 0xa {
        show_error_at(0x9eaff4, 0x350);
        return 0;
    }
    let row = if comp != 0 {
        match exe::key_nation_index_of_comp(comp) as i8 {
            0 | 1 => None, // the world rows
            k => Some(KeyNationRecord(G_LEAGUE_TABLE.wrapping_add((k as isize * 72) as usize))),
        }
    } else {
        None
    };
    write_season_label(text, year as u16, row);
    1
}

/// Writes the name of a nation's extra division (the one below its key leagues) and returns 1, or returns 0 when it
/// has none.
///
/// `key_nation_extra_division_name(nation, text, size)`: a null nation: 0. A text under 20 bytes: the "Error" box
/// (key_nation.cpp line 0x4a1) and 0. By the nation's id: England (`KEY_NATION_ENGLAND`) "Conference Division"
/// (with the English Conference's name, the string at 0xad9c64), Germany "Regional Divisions", Italy "Serie C2 A, B,
/// C", Portugal and Spain "Second Division B", Sweden "Second Division" (each through the game's text formatter, only
/// with a text); any other nation: 0.
/// Original: `key_nation_extra_division_name` (0x669f50, key_nation.cpp, cdecl).
pub(crate) unsafe extern "C" fn key_nation_extra_division_name(nation: usize, text: usize, size: i32) -> u32 {
    if nation == 0 {
        return 0;
    }
    if text != 0 && size < 0x14 {
        show_error_at(0x9eaff4, 0x4a1);
        return 0;
    }
    let id = read_u32(nation);
    let name = if id == read_u32(KEY_NATION_ENGLAND) {
        0x9eb078 // "Conference Division<%s - COMMENT - English Conference>"
    } else if id == read_u32(KEY_NATION_GERMANY) {
        0x9eb064 // "Regional Divisions"
    } else if id == read_u32(KEY_NATION_ITALY) {
        0x9eb050 // "Serie C2 A, B, C"
    } else if id == read_u32(KEY_NATION_PORTUGAL) || id == read_u32(KEY_NATION_SPAIN) {
        0x9eb03c // sx12_9: "Second Division B"
    } else if id == read_u32(KEY_NATION_SWEDEN) {
        0x9eb02c // "Second Division"
    } else {
        return 0;
    };
    if text != 0 {
        covered!("sx12_9 extra division name");
        story_lang!(text, name, 0xad9c64); // the English Conference's name, read only by the first template
    }
    1
}

/// Runs a label writer side by side: the original into the caller's text, the copy into a local one; the strings and
/// the returns compared. `boxed` (the "Error" box) or no text: only the original (and the return, when there is no
/// text).
unsafe fn compare_label_writer(name: &str, text: usize, boxed: bool, original: impl FnOnce() -> u32, copy: impl FnOnce(usize) -> u32) -> u32 {
    if boxed {
        return original();
    }
    let r = original();
    let mut local = [0u8; 0x100];
    let n = copy(if text == 0 { 0 } else { local.as_mut_ptr() as usize });
    let a: &[u8] = if text == 0 { &[] } else { core::ffi::CStr::from_ptr(text as *const core::ffi::c_char).to_bytes() };
    let b: &[u8] = if text == 0 { &[] } else { core::ffi::CStr::from_ptr(local.as_ptr() as *const core::ffi::c_char).to_bytes() };
    SCOREBOARD.record(r == n && a == b, || format!("{} original {} {:?} rust {} {:?}", name, r, String::from_utf8_lossy(a), n, String::from_utf8_lossy(b)));
    r
}

/// Comparer for `key_nation_format_club_season` (also only the original when `key_nation_index_of_club` would show its
/// box).
pub(crate) unsafe extern "C" fn compare_key_nation_format_club_season(year: u32, club: usize, text: usize, size: i32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(u32, usize, usize, i32) -> u32>(%%I0%%);
    let boxed = text == 0 || size < 0xa || (club != 0 && hits_key_nation_error(Club(club))); // the "Error" boxes
    compare_label_writer("key_nation_format_club_season", text, boxed, || o(year, club, text, size),
        |t| key_nation_format_club_season(year, club, t, size))
}

/// Comparer for `key_nation_format_comp_season`.
pub(crate) unsafe extern "C" fn compare_key_nation_format_comp_season(year: u32, comp: usize, text: usize, size: i32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(u32, usize, usize, i32) -> u32>(%%I1%%);
    compare_label_writer("key_nation_format_comp_season", text, text == 0 || size < 0xa, || o(year, comp, text, size),
        |t| key_nation_format_comp_season(year, comp, t, size))
}

/// Comparer for `key_nation_extra_division_name`.
pub(crate) unsafe extern "C" fn compare_key_nation_extra_division_name(nation: usize, text: usize, size: i32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, i32) -> u32>(%%I2%%);
    compare_label_writer("key_nation_extra_division_name", text, nation != 0 && text != 0 && size < 0x14, || o(nation, text, size),
        |t| key_nation_extra_division_name(nation, t, size))
}
