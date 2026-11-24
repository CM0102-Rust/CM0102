// ---------- cx15_2: the UEFA seeding: a nation's season points, its European places and the registration deadlines ----------

/// Adds `points` (a signed byte) to the season points (`uefa_seeding_nation::SEASON_POINTS`, a word) of a nation's
/// seeding entry. The entry is found through the nation's national team (`exe::database_get_national_team`, cdecl):
/// its seeding rank (`club::EURO_SEEDING`, a signed byte, the position written by `roll_coefficients`) indexes the
/// 0x12-byte entries from `this`. Returns the address of the points word.
///
/// Called by the European cups for each club result (`exe::uefaseeding_add_nation_points` on `G_UEFA_SEEDING`).
/// Original: `UefaSeeding::add_nation_points` (0x9058b0, uefa_seeding.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn uefa_seeding_add_nation_points(this: usize, nation: usize, points: u32) -> usize {
    covered!("cx15_2 nation points");
    let team = Club(exe::database_get_national_team(nation));
    let at = seeding_nation_entry(this, team.euro_seeding() as i8 as i32) + uefa_seeding_nation::SEASON_POINTS;
    write_i16(at, read_i16(at).wrapping_add(points as u8 as i8 as i16));
    at
}

/// A nation's Champions League and UEFA Cup places: both 1 first (the UEFA Cup one written first), then every seeding
/// entry (the count, `uefa_seeding::NATION_COUNT`, a signed byte read again on each pass) whose nation (+4) is `nation`
/// overwrites them with its `uefa_seeding_nation::CHAMPIONS_LEAGUE_PLACES` and `uefa_seeding_nation::UEFA_CUP_PLACES`
/// (the last match wins; a nation outside the table keeps 1 and 1).
/// Original: `UefaSeeding::get_nation_places` (0x905c60, uefa_seeding.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn uefa_seeding_get_nation_places(this: usize, nation: usize, champions_league: *mut i8, uefa_cup: *mut i8) {
    *uefa_cup = 1;
    *champions_league = 1;
    let mut i = 0i32;
    while i < read_i8(this + uefa_seeding::NATION_COUNT) as i32 {
        let e = seeding_nation_entry(this, i);
        if read_usize(e + 4) == nation {
            covered!("cx15_2 nation places");
            *champions_league = read_i8(e + uefa_seeding_nation::CHAMPIONS_LEAGUE_PLACES);
            *uefa_cup = read_i8(e + uefa_seeding_nation::UEFA_CUP_PLACES);
        }
        i += 1;
    }
}

/// `SQUAD_MANAGER::process_euro_registration` (thiscall(step), ret 4; already Rust, called through the exe).
const FN_SQUAD_MANAGER_PROCESS_EURO_REGISTRATION: usize = 0x85a9b0;

/// The European registration step of today, if any: 1 on 4 July, 2 on 1 August, 3 on 2 January except in 2001. Each
/// date is built with `make_date_dword` (Date::Date) in the game's year (`G_CURRENT_DATE_YEAR`) with the weekday the
/// original passes (4, 4 and 1), all three before any test, and compared with today (`G_CURRENT_DATE`) by year and day.
unsafe fn registration_deadline_step() -> Option<u32> {
    let year = read_u16(G_CURRENT_DATE_YEAR) as u32;
    let qualifying = make_date_dword(4, 7, year, 4);
    let group_stage = make_date_dword(1, 8, year, 4);
    let knockout = make_date_dword(2, 1, year, 1);
    if is_date_today(qualifying, G_CURRENT_DATE) {
        Some(1)
    } else if is_date_today(group_stage, G_CURRENT_DATE) {
        Some(2)
    } else if is_date_today(knockout, G_CURRENT_DATE) && read_i16(G_CURRENT_DATE_YEAR) != 0x7d1 {
        Some(3)
    } else {
        None
    }
}

/// The European squad registration deadlines: on each of the three dates of `registration_deadline_step` the squad
/// manager (`G_SQUAD_MANAGER`) runs the registration of that step (`SQUAD_MANAGER::process_euro_registration`, which
/// picks the registered players of every club in the Champions League, the UEFA Cup and the Intertoto and sends the
/// human managers the registration news). Any other day, nothing. `this` is not read.
///
/// Called by the daily pass of the UEFA seeding (`update_season` with `diario`). SEH frame only for the three local
/// dates.
/// Original: `uefa_check_registration_deadlines` (0x9058e0, uefa_seeding.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn uefa_check_registration_deadlines(_this: usize) {
    if let Some(step) = registration_deadline_step() {
        covered!("cx15_2 registration deadline");
        let register: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_SQUAD_MANAGER_PROCESS_EURO_REGISTRATION);
        register(read_usize(G_SQUAD_MANAGER), step);
    }
}

/// Comparer for `uefa_seeding_add_nation_points`: side by side on the seeding table, and the returned address.
pub(crate) unsafe extern "thiscall" fn compare_uefa_seeding_add_nation_points(this: usize, nation: usize, points: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(%%I0%%);
    let reg = [(this, uefa_seeding::NATION_COUNT)];
    let (a, b, same, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, nation, points), || uefa_seeding_add_nation_points(this, nation, points));
    SCOREBOARD.record(same && a == b, || format!("uefa_seeding_add_nation_points(nation {:#x}, {}): {:#x}/{:#x}, differences at {:x?}", nation, points as u8 as i8, a, b,
        memory_diff_addresses(&mo, &mr)));
    a
}

/// Comparer for `uefa_seeding_get_nation_places`: the original writes the caller's bytes, the copy two locals.
pub(crate) unsafe extern "thiscall" fn compare_uefa_seeding_get_nation_places(this: usize, nation: usize, champions_league: *mut i8, uefa_cup: *mut i8) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut i8, *mut i8)>(%%I1%%)(this, nation, champions_league, uefa_cup);
    let (mut cl, mut uefa) = (0i8, 0i8);
    uefa_seeding_get_nation_places(this, nation, &mut cl, &mut uefa);
    SCOREBOARD.record(*champions_league == cl && *uefa_cup == uefa, || format!("uefa_seeding_get_nation_places(nation {:#x}): original {} {} rust {} {}", nation,
        *champions_league, *uefa_cup, cl, uefa));
}

/// Comparer for `uefa_check_registration_deadlines`: the registration sends news (it cannot run twice), so only the
/// original runs (the copy's rule is proved by the career mutations).
pub(crate) unsafe extern "thiscall" fn compare_uefa_check_registration_deadlines(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this)
}
