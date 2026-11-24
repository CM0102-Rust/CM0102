// ---------- sbA_7: a year of the fixture calendar: building it, destroying it and saving it (fix_man.cpp) ----------

/// The source file named in the fixture calendar's error boxes.
const FIX_MAN_CPP: usize = 0x9cc208;

/// Builds a year block of the fixture calendar.
///
/// The other year's block (`fixture_manager::OTHER_YEAR`) is the one given; without it the main block checks the CD
/// first (`find_game_cd`; the no-CD patch of the running exe is honoured). No moved news queue yet; the main flag
/// (`fixture_manager::IS_PRIMARY`); the main block loads the club dates (`fix_man_load_club_dates`) and, with a game
/// data version of at least 254000 either way (`G_GAME_DATA_VERSION`), the leg checks
/// (`fix_man_load_fixture_leg_checks`). The year and whether it is a leap year (`date_is_leap_year`). For each of the
/// 366 days, the three slices' arrays of one list per active key nation (`G_KEY_NATION_ACTIVE_COUNT`, malloc of 4 per
/// nation; failing: the error box 0xaf and the block stays half built) and the lists (`operator new` of 0xe and
/// `LLIST::LLIST`; failing: the box 0xba). Then the year's fixtures are loaded (`FIXTURE_MANAGER::load_fixtures`).
/// Returns `this`.
/// Original: `FIXTURE_MANAGER::FIXTURE_MANAGER` (0x5a9cf0, fix_man.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn new_fixture_manager(this: usize, year: u32, other: usize, is_main: u8) -> usize {
    if other == 0 && is_main != 0 {
        find_game_cd(0x5a9d65, 0x5a9da7);
    }
    write_usize(this + fixture_manager::OTHER_YEAR, other);
    write_usize(this + fixture_manager::MOVED_NEWS_QUEUE, 0);
    write_u8(this + fixture_manager::IS_PRIMARY, is_main);
    if is_main != 0 {
        fix_man_load_club_dates();
        if read_i32(G_GAME_DATA_VERSION).wrapping_abs() >= 254000 {
            fix_man_load_fixture_leg_checks();
        }
    }
    write_u16(this + fixture_manager::YEAR, year as u16);
    write_u8(this + fixture_manager::LEAP_YEAR, (exe::date_is_leap_year(year) != 0) as u8);
    let nations = || read_i8(G_KEY_NATION_ACTIVE_COUNT) as i32;
    for day in 0..0x16e {
        let slices = this + fixture_manager::DAY_LISTS + day * 0xc;
        for j in 0..3 {
            write_usize(slices + j * 4, exe::crt_malloc((nations() * 4) as u32));
        }
        if (0..3).any(|j| read_usize(slices + j * 4) == 0) {
            show_error_at(FIX_MAN_CPP, 0xaf);
            return this;
        }
        let mut k: i16 = 0;
        while (k as i32) < nations() {
            for j in 0..3 {
                let mem = exe::crt_operator_new(0xe);
                let list = if mem != 0 { exe::new_llist(mem) } else { 0 };
                write_usize(read_usize(slices + j * 4) + k as usize * 4, list);
            }
            if (0..3).any(|j| read_usize(read_usize(slices + j * 4) + k as usize * 4) == 0) {
                show_error_at(FIX_MAN_CPP, 0xba);
                return this;
            }
            k += 1;
        }
    }
    exe::fixture_manager_load_fixtures(this);
    this
}

/// Destroys a year block of the fixture calendar: the moved news queue (`LLIST::~LLIST` and delete), the year saved
/// (`FIXTURE_MANAGER::save_fixtures` with 0; failing: the error box 0xe3), every day's lists destroyed and their slots
/// cleared, the three arrays freed; the main block also destroys the other year's block and frees the club dates table
/// (`G_CLUB_FIXTURE_YEAR_TABLE`, then 0).
/// Original: `FIXTURE_MANAGER::~FIXTURE_MANAGER` (0x5aa0d0, fix_man.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_fixture_manager(this: usize) {
    let queue = read_usize(this + fixture_manager::MOVED_NEWS_QUEUE);
    if queue != 0 {
        exe::drop_llist(queue);
        exe::crt_operator_delete(queue);
    }
    if save_fixtures(this, 0) == 0 {
        show_error_at(FIX_MAN_CPP, 0xe3);
    }
    for day in 0..0x16e {
        let slices = this + fixture_manager::DAY_LISTS + day * 0xc;
        let mut k: i16 = 0;
        while (k as i32) < read_i8(G_KEY_NATION_ACTIVE_COUNT) as i32 {
            for j in 0..3 {
                let slot = read_usize(slices + j * 4) + k as usize * 4;
                let list = read_usize(slot);
                if list != 0 {
                    exe::drop_llist(list);
                    exe::crt_operator_delete(list);
                    write_usize(slot, 0);
                }
            }
            k += 1;
        }
        for j in 0..3 {
            exe::crt_free(read_usize(slices + j * 4));
        }
    }
    if read_u8(this + fixture_manager::IS_PRIMARY) != 0 {
        let other = read_usize(this + fixture_manager::OTHER_YEAR);
        if other != 0 {
            drop_fixture_manager(other);
            exe::crt_operator_delete(other);
            write_usize(this + fixture_manager::OTHER_YEAR, 0);
        }
        exe::crt_free(read_usize(G_CLUB_FIXTURE_YEAR_TABLE));
        write_usize(G_CLUB_FIXTURE_YEAR_TABLE, 0);
    }
}

/// Saves the year's fixtures into CM3_TEMP\fixtures_<year>.tmp (the name kept at `fixture_manager::FILE_NAME`): the
/// count, then every fixture (0x4f bytes) of every day, key nation and slice, and the count again at the start. Not
/// opened: the error box 0x69f and 0; a failed write of the count or the seek: the boxes 0x6ab, 0x6d2 and 0x6d8, the file
/// closed and 0 (the fixtures' writes are not checked). With no fixture the file is removed. With `save_all` the main
/// block also saves the other year (failing: 0), the club dates and the leg checks. Returns 1.
/// Defect of the original: the empty file is removed by its bare name (`remove("fixtures_<year>.tmp")`), not in
/// CM3_TEMP, so it stays.
/// Original: `FIXTURE_MANAGER::save_fixtures` (0x5ac060, fix_man.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn save_fixtures(this: usize, save_all: u32) -> u32 {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let name = this + fixture_manager::FILE_NAME;
    format(name, 0x9cc238, read_u16(this + fixture_manager::YEAR) as u32); // "fixtures_%d.tmp"
    let f = open_temp_file(name, 0x9a7f28); // "wb"
    if f == 0 {
        show_error_at(FIX_MAN_CPP, 0x69f);
        return 0;
    }
    let mut count: i32 = 0;
    let write_count = |count: i32| exe::crt_fwrite(&count as *const i32 as usize, 4, 1, f) == 1;
    let fail = |row: u32| {
        show_error_at(FIX_MAN_CPP, row);
        exe::crt_fclose(f);
        0
    };
    if !write_count(count) {
        return fail(0x6ab);
    }
    for day in 0..0x16e {
        let slices = this + fixture_manager::DAY_LISTS + day * 0xc;
        let mut k = 0;
        while k < read_i8(G_KEY_NATION_ACTIVE_COUNT) as i32 {
            for j in 0..3 {
                let list = read_usize(read_usize(slices + j * 4) + k as usize * 4);
                let mut fixture = list_first(list);
                while fixture != 0 {
                    exe::crt_fwrite(fixture, 0x4f, 1, f);
                    count += 1;
                    fixture = exe::llist_get_next(list);
                }
            }
            k += 1;
        }
    }
    if exe::crt_fseek(f, 0, 0) != 0 {
        return fail(0x6d2);
    }
    if !write_count(count) {
        return fail(0x6d8);
    }
    exe::crt_fclose(f);
    if count == 0 {
        covered!("sbA_7 empty fixtures file");
        exe::crt_remove(name as *const u8);
    }
    if save_all != 0 && read_u8(this + fixture_manager::IS_PRIMARY) != 0 {
        let other = read_usize(this + fixture_manager::OTHER_YEAR);
        if other != 0 && save_fixtures(other, 1) == 0 {
            return 0;
        }
        fix_man_save_club_dates();
        fix_man_save_fixture_leg_checks();
    }
    1
}

/// The whole of a CM3_TEMP file (empty when it does not open).
unsafe fn read_temp_file(name: usize) -> Vec<u8> {
    let f = open_temp_file(name, 0x98f928); // "rb"
    if f == 0 {
        return Vec::new();
    }
    exe::crt_fseek(f, 0, 2);
    let size = exe::crt_ftell(f).max(0) as usize;
    exe::crt_fseek(f, 0, 0);
    let mut v = vec![0u8; size];
    if size != 0 {
        exe::crt_fread(v.as_mut_ptr() as usize, size as u32, 1, f);
    }
    exe::crt_fclose(f);
    v
}

/// Comparer for `new_fixture_manager`: only the original runs (it allocates the lists and loads the year); it checks
/// the year, the leap year flag, the main flag, the other year and the first day's lists.
pub(crate) unsafe extern "thiscall" fn compare_new_fixture_manager(this: usize, year: u32, other: usize, is_main: u8) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize, u8) -> usize>(%%I0%%)(this, year, other, is_main);
    let ok = r == this && read_u16(this + fixture_manager::YEAR) == year as u16
        && read_u8(this + fixture_manager::LEAP_YEAR) == (exe::date_is_leap_year(year) != 0) as u8
        && read_u8(this + fixture_manager::IS_PRIMARY) == is_main && read_usize(this + fixture_manager::OTHER_YEAR) == other
        && (0..3).all(|j| read_usize(this + j * 4) != 0);
    SCOREBOARD.record(ok, || format!("new_fixture_manager(year {}, other {:#x}, main {}): the block is not as planned", year as u16, other, is_main));
    r
}

/// Comparer for `drop_fixture_manager`: only the original runs (it frees the block); a main block must leave the club
/// dates table freed (0) and no other year.
pub(crate) unsafe extern "thiscall" fn compare_drop_fixture_manager(this: usize) {
    let main = read_u8(this + fixture_manager::IS_PRIMARY) != 0;
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this);
    if main {
        SCOREBOARD.record(read_usize(G_CLUB_FIXTURE_YEAR_TABLE) == 0 && read_usize(this + fixture_manager::OTHER_YEAR) == 0,
            || "drop_fixture_manager: the main block kept the club dates or the other year".to_string());
    }
}

/// Comparer for `save_fixtures`: both write the same file (the second write replaces the first); the year's file
/// after each and the results must be the same.
pub(crate) unsafe extern "thiscall" fn compare_save_fixtures(this: usize, save_all: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I2%%)(this, save_all);
    let name = this + fixture_manager::FILE_NAME;
    let original = read_temp_file(name);
    let n = save_fixtures(this, save_all);
    let copy = read_temp_file(name);
    SCOREBOARD.record(r == n && original == copy, || format!("save_fixtures(year {}, all {}): original {} ({} bytes) rust {} ({} bytes)",
        read_u16(this + fixture_manager::YEAR), save_all, r, original.len(), n, copy.len()));
    r
}
