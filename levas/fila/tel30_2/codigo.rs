// ---------- batch %%LEVA%%: the background pictures (2): the setters by record and by section (background.cpp) ----------

/// The setters' common body: on a screen that takes a picture (`background_screen_open`; otherwise 0), with the
/// pictures off ":" goes on and 1 returns. A record's own finder comes first; with `exact` its miss returns 0; then
/// the fallbacks in order (each a finder and the related record, skipped when the record is null); a null record with
/// `exact` returns 0. Then a random picture of the record's own section (`own(0, second)`), and failing that
/// `background_set_random_picture`; 1 is returned. Every picture found goes on through `background_set_file`.
unsafe fn background_set_for(record: usize, second: u32, exact: u32, own: unsafe extern "C" fn(usize, u32) -> usize,
    fallbacks: impl FnOnce(&mut dyn FnMut(unsafe extern "C" fn(usize, u32) -> usize, usize) -> bool) -> bool) -> u32 {
    if !background_screen_open() {
        return 0;
    }
    if read_u32(BACKGROUND_PICTURES_ON) == 0 {
        background_set(BACKGROUND_NONE);
        return 1;
    }
    if record == 0 {
        if exact != 0 {
            return 0;
        }
    } else {
        let name = own(record, second);
        if name != 0 {
            background_set_file(name);
            return 1;
        }
        if exact != 0 {
            return 0;
        }
        let mut attempt = |find: unsafe extern "C" fn(usize, u32) -> usize, related: usize| {
            if related == 0 {
                return false;
            }
            let name = find(related, second);
            if name != 0 {
                background_set_file(name);
                return true;
            }
            false
        };
        if fallbacks(&mut attempt) {
            return 1;
        }
    }
    let name = own(0, second);
    if name != 0 {
        background_set_file(name);
        return 1;
    }
    background_set_random_picture();
    1
}

/// Sets a nation's picture (`background_set_for`): its own subsection, then its national team's club subsection
/// (`database_get_national_team`), then its national stadium (`nation::NATIONAL_STADIUM`).
/// Original: `background_set_nation_picture` (0x41c100, background.cpp, cdecl).
pub(crate) unsafe extern "C" fn background_set_nation_picture(record: usize, second: u32, exact: u32) -> u32 {
    background_set_for(record, second, exact, background_find_nation_picture, |attempt| {
        attempt(background_find_club_picture, exe::database_get_national_team(record))
            || attempt(background_find_stadium_picture, read_usize(record + nation::NATIONAL_STADIUM))
    })
}

/// Sets a club's picture (`background_set_for`): its own subsection, then its stadium (`club::STADIUM`), its second
/// stadium (+0x6e), its division (`club::DIVISION`, a competition) and its nation (`club::NATION`).
/// Original: `background_set_club_picture` (0x41c250, background.cpp, cdecl).
pub(crate) unsafe extern "C" fn background_set_club_picture(record: usize, second: u32, exact: u32) -> u32 {
    background_set_for(record, second, exact, background_find_club_picture, |attempt| {
        attempt(background_find_stadium_picture, read_usize(record + club::STADIUM))
            || attempt(background_find_stadium_picture, read_usize(record + 0x6e))
            || attempt(background_find_comp_picture, read_usize(record + club::DIVISION))
            || attempt(background_find_nation_picture, read_usize(record + club::NATION))
    })
}

/// Sets a club competition's picture (`background_set_for`): its own subsection, then its latest winner's club
/// (`comp_util_get_latest_placed_club(comp, 0)`) and its nation (`competition::NATION`).
/// Original: `background_set_comp_picture` (0x41c410, background.cpp, cdecl).
pub(crate) unsafe extern "C" fn background_set_comp_picture(record: usize, second: u32, exact: u32) -> u32 {
    background_set_for(record, second, exact, background_find_comp_picture, |attempt| {
        attempt(background_find_club_picture, exe::comp_util_get_latest_placed_club(record, 0))
            || attempt(background_find_nation_picture, read_usize(record + competition::NATION))
    })
}

/// Sets an award's picture (`background_set_for`): its own subsection, then its nation (+0x57 of the staff
/// competition).
/// Original: `background_set_award_picture` (0x41c560, background.cpp, cdecl).
pub(crate) unsafe extern "C" fn background_set_award_picture(record: usize, second: u32, exact: u32) -> u32 {
    background_set_for(record, second, exact, background_find_award_picture, |attempt| {
        attempt(background_find_nation_picture, read_usize(record + 0x57))
    })
}

/// Sets a stadium's picture (`background_set_for`): its own subsection, then its city's nation (`stadium::CITY`,
/// then `city::NATION`, both non-null).
/// Original: `background_set_stadium_picture` (0x41c670, background.cpp, cdecl).
pub(crate) unsafe extern "C" fn background_set_stadium_picture(record: usize, second: u32, exact: u32) -> u32 {
    background_set_for(record, second, exact, background_find_stadium_picture, |attempt| {
        let city = read_usize(record + stadium::CITY);
        city != 0 && attempt(background_find_nation_picture, read_usize(city + city::NATION))
    })
}

/// Sets a staff member's picture (`background_set_for`): his own subsection, then his club (`staff::CLUB_JOB`) and
/// his nation (`staff::NATION`).
/// Original: `background_set_staff_picture` (0x41c790, background.cpp, cdecl).
pub(crate) unsafe extern "C" fn background_set_staff_picture(record: usize, second: u32, exact: u32) -> u32 {
    background_set_for(record, second, exact, background_find_staff_picture, |attempt| {
        attempt(background_find_club_picture, read_usize(record + staff::CLUB_JOB))
            || attempt(background_find_nation_picture, read_usize(record + staff::NATION))
    })
}

/// Sets a random picture of section `k` (training, boardroom...): on a screen that takes a picture (otherwise 0),
/// with the pictures off ":" and 1. With the section's array: a random subsection (`utils_random(count)`; -1 skips)
/// and, when it has a first list, a random picture of it; a non-null one goes on and 1 returns. Otherwise `exact`
/// returns 0, and without it `background_set_random_picture` and 1.
/// Original: `background_set_section_picture` (0x41c8e0, background.cpp, cdecl).
pub(crate) unsafe extern "C" fn background_set_section_picture(k: i32, exact: u32) -> u32 {
    if !background_screen_open() {
        return 0;
    }
    if read_u32(BACKGROUND_PICTURES_ON) == 0 {
        background_set(BACKGROUND_NONE);
        return 1;
    }
    let at = (BACKGROUND_SECTIONS as i32).wrapping_add(k.wrapping_mul(BACKGROUND_SECTION_SIZE as i32)) as u32 as usize;
    if read_usize(at + 6) != 0 {
        let s = exe::utils_random(read_i16(at + 4) as i32) as i16;
        if s != -1 {
            let sub = (read_usize(at + 6) as isize + s as isize * 16) as usize;
            if read_usize(sub + background_sub::LIST) != 0 {
                let p = exe::utils_random(read_i16(sub + background_sub::COUNT) as i32) as i16;
                let list = read_usize((read_usize(at + 6) as isize + s as isize * 16) as usize + background_sub::LIST);
                let file = read_usize((list as isize + p as isize * 4) as usize);
                if file != 0 {
                    covered!("tel30_2 section picture");
                    background_set_file(file);
                    return 1;
                }
            }
        }
    }
    if exact != 0 {
        return 0;
    }
    background_set_random_picture();
    1
}

/// Comparer for `background_set_nation_picture`: side by side over the generator and the screen's picture.
pub(crate) unsafe extern "C" fn compare_background_set_nation_picture(record: usize, second: u32, exact: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, u32) -> u32>(%%I0%%);
    background_compare("set_nation_picture", || o(record, second, exact), || background_set_nation_picture(record, second, exact))
}

/// Comparer for `background_set_club_picture`: side by side over the generator and the screen's picture.
pub(crate) unsafe extern "C" fn compare_background_set_club_picture(record: usize, second: u32, exact: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, u32) -> u32>(%%I1%%);
    background_compare("set_club_picture", || o(record, second, exact), || background_set_club_picture(record, second, exact))
}

/// Comparer for `background_set_comp_picture`: side by side over the generator and the screen's picture.
pub(crate) unsafe extern "C" fn compare_background_set_comp_picture(record: usize, second: u32, exact: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, u32) -> u32>(%%I2%%);
    background_compare("set_comp_picture", || o(record, second, exact), || background_set_comp_picture(record, second, exact))
}

/// Comparer for `background_set_award_picture`: side by side over the generator and the screen's picture.
pub(crate) unsafe extern "C" fn compare_background_set_award_picture(record: usize, second: u32, exact: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, u32) -> u32>(%%I3%%);
    background_compare("set_award_picture", || o(record, second, exact), || background_set_award_picture(record, second, exact))
}

/// Comparer for `background_set_stadium_picture`: side by side over the generator and the screen's picture.
pub(crate) unsafe extern "C" fn compare_background_set_stadium_picture(record: usize, second: u32, exact: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, u32) -> u32>(%%I4%%);
    background_compare("set_stadium_picture", || o(record, second, exact), || background_set_stadium_picture(record, second, exact))
}

/// Comparer for `background_set_staff_picture`: side by side over the generator and the screen's picture.
pub(crate) unsafe extern "C" fn compare_background_set_staff_picture(record: usize, second: u32, exact: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, u32) -> u32>(%%I5%%);
    background_compare("set_staff_picture", || o(record, second, exact), || background_set_staff_picture(record, second, exact))
}

/// Comparer for `background_set_section_picture`: side by side over the generator and the screen's picture.
pub(crate) unsafe extern "C" fn compare_background_set_section_picture(k: i32, exact: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(i32, u32) -> u32>(%%I6%%);
    background_compare("set_section_picture", || o(k, exact), || background_set_section_picture(k, exact))
}
