use super::*;

/// Builds the general.dat summary: the active nations (two groups), the human managers with club and nation, the game
/// date, the version, the play time and time(). `build_save_game_summary(summary)` (cdecl): the summary that
/// `save_game_data` writes into `general.dat` (right after the version):
///
/// - +0: the nations of the league table (`G_LEAGUE_TABLE`, 0x23 of 0x48 bytes) with bit 2 set in `league_selected()`,
/// the first through sprintf with its own name as the format (`FN_CRT_SPRINTF`), the others through the translated ",
/// <%s>" (`FN_LANGLIB_SPRINTF`); +1000 (0x3e8): the same with bit 1;
///
/// - 16 managers, from the last of the person table (`G_STAFF_TABLE`, `staff::SIZE` bytes) backwards: a human one
/// (`classification()` = 1) gives the name at +0x7e4 + 0x78 × i (`database_format_staff_name`, 200) and at +0x820 +
/// 0x78 × i "club / nation" (`club_job()` and `national_job()`, name at `club::SHORT_NAME` / `nation::SHORT_NAME` of
/// each), only one of them, or nothing; a non-human one only zeroes the name (the club stays as it was in the caller's
/// buffer);
///
/// - `save_summary::GAME_DATE` the game date (`exe::date_operator_3d` of `G_CURRENT_DATE`), +0x7d0 the version
/// (`exe::utils_get_version_string`), +0xf6c the play time (`G_GAME_PLAY_TIME_TOTAL` += time() −
/// `G_GAME_PLAY_TIME_LAST_STAMP`; `G_GAME_PLAY_TIME_LAST_STAMP` = time()) and +0xf70 the time().
/// Original: `build_save_game_summary` (0x591f80, file_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn build_save_game_summary(summary: *mut u8) {
    let format: unsafe extern "C" fn(*mut u8, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let translate: unsafe extern "C" fn(*mut u8, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let mut nation = [0u8; 100];
    *summary = 0;
    *summary.add(0x3e8) = 0;
    for (bit, destino) in [(2u8, summary), (1u8, summary.add(0x3e8))] {
        let mut first = true;
        for i in 0..0x23usize {
            let n = Nation(*((G_LEAGUE_TABLE + i * 0x48) as *const usize));
            if n.is_null() || n.league_selected() & bit == 0 {
                continue;
            }
            if first {
                format(nation.as_mut_ptr(), n.addr() + nation::SHORT_NAME);
                first = false;
            } else {
                translate(nation.as_mut_ptr(), 0x9c8134, n.addr() + nation::SHORT_NAME);
            }
            append_text(destino, nation.as_ptr());
        }
        if bit == 1 && first {
            *summary.add(0x3e8) = 0;
        }
    }
    let how_many = *(G_STAFF_COUNT as *const i32);
    let table = *(G_STAFF_TABLE as *const usize);
    for i in 0..0x10i32 {
        let club_txt = summary.add(0x820 + 0x78 * i as usize);
        let p = Staff(table.wrapping_add((how_many - i - 1).wrapping_mul((staff::SIZE as i32)) as usize));
        if p.classification() != 1 {
            *club_txt.sub(0x3c) = 0;
            continue;
        }
        exe::database_format_staff_name(p, (club_txt.sub(0x3c)) as *mut u8 as usize, 200, 1);
        let club = p.club_job();
        let nation_of = p.national_job();
        if !club.is_null() && !nation_of.is_null() {
            translate(club_txt, 0x9c80f4, club.addr() + club::SHORT_NAME, nation_of.addr() + nation::SHORT_NAME);
        } else if !club.is_null() {
            copy_text(club_txt, (club.addr() + club::SHORT_NAME) as *const u8);
        } else if !nation_of.is_null() {
            copy_text(club_txt, (nation_of.addr() + nation::SHORT_NAME) as *const u8);
        } else {
            *club_txt = 0;
        }
    }
    let mut copy = [0u8; 8];
    exe::date_operator_3d(summary as usize + save_summary::GAME_DATE, (copy.as_mut_ptr()) as usize, G_CURRENT_DATE);
    copy_text(summary.add(0x7d0), exe::utils_get_version_string());
    let mut now = 0i32;
    let t = exe::at_946ebb(&mut now);
    let total = (*(G_GAME_PLAY_TIME_TOTAL as *const i32)).wrapping_add(t.wrapping_sub(*(G_GAME_PLAY_TIME_LAST_STAMP as *const i32)));
    *(G_GAME_PLAY_TIME_TOTAL as *mut i32) = total;
    core::ptr::write_unaligned(summary.add(0xf6c) as *mut i32, total);
    *(G_GAME_PLAY_TIME_LAST_STAMP as *mut i32) = t;
    let t2 = exe::at_946ebb(core::ptr::null_mut());
    core::ptr::write_unaligned(summary.add(0xf70) as *mut i32, t2);
}
