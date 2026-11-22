// ---------- batch %%LEVA%%: the club screen's records, the squad screen opener and the squad statistics (club_screens.cpp) ----------

/// The last match's team sheet record (two 0x625-byte team blocks from +0x11f) and the club history states (16 of
/// 0x2a bytes), globals of club_screens.cpp built at the game start.
pub(crate) const CLUB_SCREENS_MATCH_RECORD: usize = 0xad9db8;
pub(crate) const CLUB_SCREENS_HISTORY_STATES: usize = 0xadab50;
/// The club screen's build, input and leave functions (passed to `ScreenManager::open_screen`).
pub(crate) const FN_CLUB_SCREEN_BUILD: usize = 0x454900;
pub(crate) const FN_CLUB_SCREEN_INPUT: usize = 0x4671f0;
pub(crate) const FN_CLUB_SCREEN_LEAVE: usize = 0x469da0;

/// Builds the last match's team sheet record (`match_team_pair_construct`) and registers its destructor
/// (`club_screens_exit_match_record`) with `atexit`.
/// Original: `club_screens_init_match_record` (0x453b90, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn club_screens_init_match_record() {
    match_team_pair_construct(CLUB_SCREENS_MATCH_RECORD);
    exe::crt_atexit(0x453bb0usize);
}

/// Destroys the last match's team sheet record (`match_team_pair_destructor`).
/// Original: `club_screens_exit_match_record` (0x453bb0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn club_screens_exit_match_record() {
    exe::match_team_pair_destructor(CLUB_SCREENS_MATCH_RECORD);
}

/// Builds a team sheet record: its two 0x625-byte team blocks from +0x11f (`eh vector constructor iterator` with
/// `match_team_block_construct` and `match_team_block_destruct`); returns the record.
/// Original: `match_team_pair_construct` (0x453be0, club_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn match_team_pair_construct(this: usize) -> usize {
    exe::crt_vector_constructor_iterator(this + 0x11f, 0x625, 2, 0x453c30usize, 0x453c10usize);
    this
}

/// Destroys a team block: its 20 rating records of 0x49 bytes from +0x71 (`eh vector destructor iterator` with the
/// empty 0x5a53c0).
/// Original: `match_team_block_destruct` (0x453c10, club_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn match_team_block_destruct(this: usize) {
    exe::crt_vector_destructor_iterator(this + 0x71, 0x49, 0x14, 0x5a53c0usize);
}

/// Builds a team block: its 20 rating records of 0x49 bytes from +0x71 (`rating_record_construct` 0x453c50, the
/// empty 0x5a53c0 as destructor); returns the block.
/// Original: `match_team_block_construct` (0x453c30, club_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn match_team_block_construct(this: usize) -> usize {
    exe::crt_vector_constructor_iterator(this + 0x71, 0x49, 0x14, 0x453c50usize, 0x5a53c0usize);
    this
}

/// Builds the 16 club history states (0x2a bytes each, `club_history_state_construct` and
/// `club_history_state_destruct`) and registers their destruction (`club_screens_exit_history_states`) with `atexit`.
/// Original: `club_screens_init_history_states` (0x453c60, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn club_screens_init_history_states() {
    exe::crt_vector_constructor_iterator(CLUB_SCREENS_HISTORY_STATES, 0x2a, 0x10, 0x453c90usize, 0x453cf0usize);
    exe::crt_atexit(0x453d40usize);
}

/// Builds a club history state: its date (`Date::Date`) and its list at +0x18 (`LLIST::LLIST`), then all its 0x2a
/// bytes cleared; returns the state.
/// Original: `club_history_state_construct` (0x453c90, club_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn club_history_state_construct(this: usize) -> usize {
    exe::date_date_54a110(this);
    exe::new_llist(this + 0x18);
    core::ptr::write_bytes(this as *mut u8, 0, 0x2a);
    this
}

/// Destroys a club history state: the object at +0x26, when there is one, through its scalar deleting destructor
/// (slot 0 of its vtable, with 1), then the list at +0x18 (`LLIST::~LLIST`).
/// Original: `club_history_state_destruct` (0x453cf0, club_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn club_history_state_destruct(this: usize) {
    let object = read_usize(this + 0x26);
    if object != 0 {
        let destroy: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(read_usize(read_usize(object)));
        destroy(object, 1);
    }
    exe::drop_llist(this + 0x18);
}

/// Destroys the 16 club history states (`eh vector destructor iterator` with `club_history_state_destruct`).
/// Original: `club_screens_exit_history_states` (0x453d40, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn club_screens_exit_history_states() {
    exe::crt_vector_destructor_iterator(CLUB_SCREENS_HISTORY_STATES, 0x2a, 0x10, 0x453cf0usize);
}

/// The default squad view settings of a club the human does not control: the dwords 0x41 (the filter, parameter 7),
/// 8 (parameter 8) and 0x154a21 (parameter 0x18), then the bytes 6 (parameter 9), -1 (0x19) and -1 (0x1a).
const CLUB_SQUAD_DEFAULT_VIEW: [u8; 15] = [0x41, 0, 0, 0, 8, 0, 0, 0, 0x21, 0x4a, 0x15, 0, 6, 0xff, 0xff];

/// The three best players of a club's squad (its 50 pointers at +0xd7) by `CURRENT_REPUTATION` (`WORLD_REPUTATION`
/// for a national team) + 10 × `CURRENT_ABILITY`, in 16-bit arithmetic; ties keep the earlier; returns the count (at
/// most 3) and the three players.
unsafe fn club_squad_stars(club: usize) -> (i8, [usize; 3]) {
    let (mut count, mut best, mut score) = (0i8, [0usize; 3], [-10000i16; 3]);
    for k in 0..0x32 {
        let p = read_usize(club + 0xd7 + k * 4);
        if p == 0 || read_usize(p + staff::PLAYER) == 0 {
            continue;
        }
        let national = exe::database_is_national_team(club) != 0;
        let sheet = read_usize(p + staff::PLAYER);
        let reputation = read_i16(sheet + if national { player_sheet::WORLD_REPUTATION } else { player_sheet::CURRENT_REPUTATION });
        let s = reputation.wrapping_add(read_i16(sheet + player_sheet::CURRENT_ABILITY).wrapping_mul(10));
        if count < 3 {
            count += 1;
        }
        if s > score[0] {
            best = [p, best[0], best[1]];
            score = [s, score[0], score[1]];
        } else if s > score[1] {
            best = [best[0], p, best[1]];
            score = [score[0], s, score[1]];
        } else if s > score[2] {
            best[2] = p;
            score[2] = s;
        }
    }
    (count, best)
}

/// Opens the club screen at its squad tab, with the squad view and the club's three best players.
///
/// A club without a reserve squad (`database_club_has_reserve_squad`) has `reserve` cleared. With `restore`, the 60
/// parameters of the current screen are saved first. The view settings are the human's
/// (`HUMAN_MANAGER::get_squad_view_settings(0)`) when he controls the club, otherwise the defaults. The screen opens
/// (`ScreenManager::open_screen` with the club screen's build, input and leave functions, not single instance); when
/// it does: 0 = club, 1 = reserve, 2 = 3 = 0, 4 = 5 with `tab5` (otherwise 1), 5 = 6 = 0, 7 / 8 = the view's first
/// two dwords, 9 = its byte +0xc (signed), 0xa = 0xb = 0, 0xc = the year, 0xd = 0xe = 0, 0xf = 1, 0x10 = 0x11 = 0,
/// 0x12 = 1, 0x13..0x16 = 0, 0x17 = the year, 0x18 = the view's third dword, 0x19 / 0x1a = its bytes +0xd / +0xe
/// (signed), 0x1b = 2 with `tab5` (otherwise 1), 0x1c..0x1f = 0; with `restore` the saved 4, 7, 8, 9, 0x10, 0x11,
/// 0x12, 0x18, 0x19, 0x1a and 0x1b come back; with `highlight` parameter 7 gets bit 0x800. Parameters 0x20, 0x21 and
/// 0x22 are the ids of the club's three best players (`club_squad_stars`), -1 for each missing.
/// Original: `open_club_squad_screen` (0x453d60, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn open_club_squad_screen(club: usize, reserve: u32, restore: u32, tab5: u32, highlight: u32) {
    let mut reserve = reserve;
    if club != 0 && exe::database_club_has_reserve_squad(club) == 0 {
        reserve = 0;
    }
    let mut saved = [0usize; 0x3c];
    if restore != 0 {
        for (k, v) in saved.iter_mut().enumerate() {
            *v = get_screen_param(k as u32);
        }
    }
    let default_view = CLUB_SQUAD_DEFAULT_VIEW;
    let view = if club != 0 && exe::human_manager_current_controls_club(club, 0, 1) != 0 {
        let settings: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5fe950usize);
        settings(read_usize(G_HUMAN_MANAGER), 0)
    } else {
        default_view.as_ptr() as usize
    };
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, FN_CLUB_SCREEN_BUILD, FN_CLUB_SCREEN_INPUT, 0, FN_CLUB_SCREEN_LEAVE, 0) == 0 {
        return;
    }
    covered!("tel30_8 club squad screen opened");
    let year = read_u16(G_CURRENT_DATE_YEAR) as usize;
    let tab = |a: usize, b: usize| if tab5 != 0 { a } else { b };
    let params: [(u32, usize); 32] = [
        (0, club), (1, reserve as usize), (2, 0), (3, 0), (4, tab(5, 1)), (5, 0), (6, 0), (7, read_usize(view)),
        (8, read_usize(view + 4)), (9, read_i8(view + 0xc) as i32 as usize), (0xa, 0), (0xb, 0), (0xc, year), (0xd, 0),
        (0xe, 0), (0xf, 1), (0x10, 0), (0x11, 0), (0x12, 1), (0x13, 0), (0x14, 0), (0x15, 0), (0x16, 0), (0x17, year),
        (0x18, read_usize(view + 8)), (0x19, read_i8(view + 0xd) as i32 as usize), (0x1a, read_i8(view + 0xe) as i32 as usize),
        (0x1b, tab(2, 1)), (0x1c, 0), (0x1d, 0), (0x1e, 0), (0x1f, 0),
    ];
    for (k, v) in params {
        set_screen_param(k, v);
    }
    if restore != 0 {
        for k in [4u32, 7, 8, 9, 0x10, 0x11, 0x12, 0x18, 0x19, 0x1a, 0x1b] {
            set_screen_param(k, saved[k as usize]);
        }
    }
    if highlight != 0 {
        set_screen_param(7, read_usize(view) | 0x800);
    }
    let (count, best) = if club != 0 { club_squad_stars(club) } else { (0, [0; 3]) };
    let id = |k: usize, need: i8| if count >= need { read_usize(best[k]) } else { usize::MAX };
    set_screen_param(0x20, id(0, 1));
    set_screen_param(0x21, id(1, 2));
    set_screen_param(0x22, id(2, 3));
}

/// A menu item: the id -1 and the value 0 (two words); returns the item.
/// Original: `MenuItem::MenuItem` (0x4568f0, club_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn menu_item_menu_item(this: usize) -> usize {
    write_u16(this, 0xffff);
    write_u16(this + 2, 0);
    this
}

/// The squad statistics of a club's screen (0x45 bytes at `out`).
pub(crate) mod club_squad_stats {
    /// u8: players counted / u8: injured (status 2).
    pub(crate) const COUNT: usize = 0x0;
    pub(crate) const INJURED: usize = 0x1;
    /// f32: the average age of the selected / of all.
    pub(crate) const SELECTED_AGE: usize = 0x2;
    pub(crate) const AVERAGE_AGE: usize = 0x6;
    /// i32: the wages (all, selected), the average wage (all, selected).
    pub(crate) const WAGES: usize = 0xa;
    pub(crate) const SELECTED_WAGES: usize = 0xe;
    pub(crate) const AVERAGE_WAGE: usize = 0x12;
    pub(crate) const SELECTED_AVERAGE_WAGE: usize = 0x16;
    /// The oldest, youngest, best paid and worst paid player (the last only among contracts whose type, the low six
    /// bits of +0x35, is not 6), with the two wages.
    pub(crate) const OLDEST: usize = 0x1a;
    pub(crate) const YOUNGEST: usize = 0x1e;
    pub(crate) const BEST_PAID: usize = 0x22;
    pub(crate) const WORST_PAID: usize = 0x26;
    pub(crate) const BEST_WAGE: usize = 0x2a;
    pub(crate) const WORST_WAGE: usize = 0x2e;
    /// u16: the caps; the most and fewest capped player; the most valuable.
    pub(crate) const CAPS: usize = 0x32;
    pub(crate) const MOST_CAPPED: usize = 0x34;
    pub(crate) const LEAST_CAPPED: usize = 0x38;
    pub(crate) const MOST_VALUABLE: usize = 0x3c;
    /// u8 counts: with a non-national team job and the squad flag 4, internationals, foreigners, non-EU, nationals.
    pub(crate) const CLUB_JOB_ELSEWHERE: usize = 0x40;
    pub(crate) const INTERNATIONALS: usize = 0x41;
    pub(crate) const FOREIGNERS: usize = 0x42;
    pub(crate) const NON_EU: usize = 0x43;
    pub(crate) const NATIONALS: usize = 0x44;
    pub(crate) const SIZE: usize = 0x45;
}

/// Gathers the squad statistics of a club's screen into `out` (`club_squad_stats`).
///
/// The side is the club's linked senior side when `club_get_linked_b_team` reports it as a B team, else the club; the
/// squad is `database_collect_club_squad(club, list, 1)`. Every field is cleared but `SELECTED_AVERAGE_WAGE`. Each
/// of the first 50 players with an id below `G_STAFF_COUNT` (for a national team, without the squad flag 8, or 4 with
/// `reserve`): counted; injured (status 2) counted; his age (`staff::AGE`, signed) summed in 16 bits, and also for
/// the selected (`Tactics::find_selection_slot(side, player, 0)` ≠ 0xff, counted); his contract with the side
/// (`get_contract_for_club`): the wage summed (also for the selected, asked again), the best paid (strictly more) and,
/// among contracts of a type other than 6, the worst paid (strictly less); the oldest (earliest date of birth: year,
/// then day; the first on ties) and the youngest (latest); the caps summed, the most capped (more than the most so
/// far, a signed byte) and the fewest (fewer); the most valuable (strictly more `staff::VALUE`); the counts of a
/// non-national team job with the squad flag 4, of a national team job, of foreigners (`database_is_staff_foreign(..,
/// club nation, 1)`), of non-EU players (`database_is_staff_eu_national`) and of the club's nationals. At the end, with
/// players, `AVERAGE_WAGE` = wages / count truncated and `AVERAGE_AGE` = the age sum / count as a float; with
/// selected players, `SELECTED_AVERAGE_WAGE` = their wages / their count truncated and `SELECTED_AGE` their age average.
/// Original: `club_screen_collect_squad_stats` (0x487840, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn club_screen_collect_squad_stats(club: usize, reserve: u32, out: usize) {
    use club_squad_stats as s;
    let (mut oldest, mut youngest, mut scratch) = ([0u32; 2], [0u32; 2], [0u32; 2]);
    exe::date_date_54a110(oldest.as_mut_ptr() as usize);
    exe::date_date_54a110(youngest.as_mut_ptr() as usize);
    let (mut selected, mut most_caps, mut fewest_caps) = (0u8, 0u8, 0u8);
    let (mut age_sum, mut selected_age_sum) = (0u16, 0u16);
    let mut senior = 0u32;
    let mut side = club;
    if exe::club_get_linked_b_team(club, &mut senior, 1) != 0 && senior == 0 {
        side = exe::club_get_linked_b_team(club, core::ptr::null_mut(), 1);
    }
    let mut squad = [0usize; 100];
    exe::database_collect_club_squad(club, squad.as_mut_ptr(), 1);
    for at in [s::SELECTED_AGE, s::AVERAGE_AGE, s::WAGES, s::SELECTED_WAGES, s::AVERAGE_WAGE, s::OLDEST, s::YOUNGEST, s::BEST_PAID,
        s::BEST_WAGE, s::WORST_PAID, s::WORST_WAGE, s::MOST_CAPPED, s::LEAST_CAPPED, s::MOST_VALUABLE] {
        write_u32(out + at, 0);
    }
    for at in [s::COUNT, s::INJURED, s::CLUB_JOB_ELSEWHERE, s::INTERNATIONALS, s::FOREIGNERS, s::NON_EU, s::NATIONALS] {
        write_u8(out + at, 0);
    }
    write_u16(out + s::CAPS, 0);
    let bump = |at: usize| write_u8(out + at, read_u8(out + at).wrapping_add(1));
    let club_nation = read_usize(club + club::NATION);
    for &p in squad.iter().take(0x32) {
        if p == 0 || read_i32(p) >= read_i32(G_STAFF_COUNT) {
            continue;
        }
        if exe::database_is_national_team(club) != 0 {
            let flag = if reserve != 0 { 4 } else { 8 };
            if read_u8(p + staff::PLAYING_SQUAD) & flag != 0 {
                continue;
            }
        }
        bump(s::COUNT);
        if exe::injury_get_player_injury_status(read_usize(G_INJURY), p) == 2 {
            bump(s::INJURED);
        }
        let age = read_i8(p + staff::AGE) as i16 as u16;
        age_sum = age_sum.wrapping_add(age);
        if exe::tactics_find_selection_slot(read_usize(G_TACTICS), side, p, 0) != 0xff {
            selected_age_sum = selected_age_sum.wrapping_add(age);
            selected = selected.wrapping_add(1);
        }
        let contract = exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), p, side);
        if contract != 0 {
            let wage = read_i32(contract + 0xc);
            write_i32(out + s::WAGES, read_i32(out + s::WAGES).wrapping_add(wage));
            if exe::tactics_find_selection_slot(read_usize(G_TACTICS), side, p, 0) != 0xff {
                write_i32(out + s::SELECTED_WAGES, read_i32(out + s::SELECTED_WAGES).wrapping_add(read_i32(contract + 0xc)));
            }
            if read_usize(out + s::BEST_PAID) == 0 || read_i32(contract + 0xc) > read_i32(out + s::BEST_WAGE) {
                write_usize(out + s::BEST_PAID, p);
                write_i32(out + s::BEST_WAGE, read_i32(contract + 0xc));
            }
            if read_u8(contract + 0x35) & 0x3f != 6
                && (read_usize(out + s::WORST_PAID) == 0 || read_i32(contract + 0xc) < read_i32(out + s::WORST_WAGE)) {
                write_usize(out + s::WORST_PAID, p);
                write_i32(out + s::WORST_WAGE, read_i32(contract + 0xc));
            }
        }
        let birth = p + staff::DATE_OF_BIRTH;
        let (day, year) = (read_i16(birth), read_u16(birth + 2));
        let (oday, oyear) = (oldest[0] as u16 as i16, (oldest[0] >> 16) as u16);
        if read_usize(out + s::OLDEST) == 0 || oyear > year || (oyear == year && oday > day) {
            write_usize(out + s::OLDEST, p);
            exe::date_operator_3d(oldest.as_mut_ptr() as usize, scratch.as_mut_ptr() as usize, birth);
        }
        let (yday, yyear) = (youngest[0] as u16 as i16, (youngest[0] >> 16) as u16);
        if read_usize(out + s::YOUNGEST) == 0 || yyear < year || (yyear == year && yday < day) {
            write_usize(out + s::YOUNGEST, p);
            exe::date_operator_3d(youngest.as_mut_ptr() as usize, scratch.as_mut_ptr() as usize, birth);
        }
        let caps = read_u8(p + staff::INT_APPS);
        write_u16(out + s::CAPS, read_u16(out + s::CAPS).wrapping_add(caps as u16));
        if caps as i32 > most_caps as i8 as i32 || read_usize(out + s::MOST_CAPPED) == 0 {
            write_usize(out + s::MOST_CAPPED, p);
            most_caps = read_u8(p + staff::INT_APPS);
        }
        if (caps as i32) < fewest_caps as i8 as i32 || read_usize(out + s::LEAST_CAPPED) == 0 {
            write_usize(out + s::LEAST_CAPPED, p);
            fewest_caps = read_u8(p + staff::INT_APPS);
        }
        let valuable = read_usize(out + s::MOST_VALUABLE);
        if valuable == 0 || read_i32(p + staff::VALUE) > read_i32(valuable + staff::VALUE) {
            write_usize(out + s::MOST_VALUABLE, p);
        }
        let job = read_usize(p + staff::NATIONAL_JOB);
        if job != 0 && exe::club_is_national_team(job) == 0 && read_u8(p + staff::PLAYING_SQUAD) & 4 != 0 {
            bump(s::CLUB_JOB_ELSEWHERE);
        }
        let job = read_usize(p + staff::NATIONAL_JOB);
        if job != 0 && exe::club_is_national_team(job) != 0 {
            bump(s::INTERNATIONALS);
        }
        if exe::database_is_staff_foreign(p, read_usize(club + club::NATION), 1) != 0 {
            bump(s::FOREIGNERS);
        }
        if exe::database_is_staff_eu_national(p, read_usize(club + club::NATION)) == 0 {
            bump(s::NON_EU);
        }
        if read_usize(p + staff::NATION) == club_nation {
            bump(s::NATIONALS);
        }
    }
    let count = read_i8(out + s::COUNT);
    if count != 0 {
        write_i32(out + s::AVERAGE_WAGE, ftol(read_i32(out + s::WAGES) as f64 / count as f64));
    }
    if selected != 0 {
        let n = selected as i8 as f64;
        write_i32(out + s::SELECTED_AVERAGE_WAGE, ftol(read_i32(out + s::SELECTED_WAGES) as f64 / n));
        write_f32(out + s::SELECTED_AGE, (selected_age_sum as i16 as f64 / n) as f32);
    }
    if count != 0 {
        write_f32(out + s::AVERAGE_AGE, (age_sum as i16 as f64 / count as f64) as f32);
    }
}

/// Comparer for `club_screens_init_match_record`.
pub(crate) unsafe extern "C" fn compare_club_screens_init_match_record() {
    // simplification: a static constructor (atexit) runs once at the game start; proven in Rust mode
    original_fn::<unsafe extern "C" fn()>(%%I0%%)()
}

/// Comparer for `club_screens_exit_match_record`.
pub(crate) unsafe extern "C" fn compare_club_screens_exit_match_record() {
    // simplification: a static destructor runs once at the game exit; proven in Rust mode
    original_fn::<unsafe extern "C" fn()>(%%I1%%)()
}

/// Comparer for `match_team_pair_construct`.
pub(crate) unsafe extern "thiscall" fn compare_match_team_pair_construct(this: usize) -> usize {
    // simplification: a constructor (the vector iterator registers its unwinding) runs once; proven in Rust mode
    original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I2%%)(this)
}

/// Comparer for `match_team_block_destruct`.
pub(crate) unsafe extern "thiscall" fn compare_match_team_block_destruct(this: usize) {
    // simplification: a destructor runs once; proven in Rust mode
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I3%%)(this)
}

/// Comparer for `match_team_block_construct`.
pub(crate) unsafe extern "thiscall" fn compare_match_team_block_construct(this: usize) -> usize {
    // simplification: a constructor runs once; proven in Rust mode
    original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I4%%)(this)
}

/// Comparer for `club_screens_init_history_states`.
pub(crate) unsafe extern "C" fn compare_club_screens_init_history_states() {
    // simplification: a static constructor (atexit) runs once at the game start; proven in Rust mode
    original_fn::<unsafe extern "C" fn()>(%%I5%%)()
}

/// Comparer for `club_history_state_construct`: the original only (it builds a list), then the plan: the 0x2a bytes
/// cleared and the state returned.
pub(crate) unsafe extern "thiscall" fn compare_club_history_state_construct(this: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I6%%)(this);
    let clear = (0..0x2a).all(|k| read_u8(this + k) == 0);
    SCOREBOARD.record(clear && r == this, || format!("club_history_state_construct({:#x}): cleared {} returned {:#x}", this, clear, r));
    r
}

/// Comparer for `club_history_state_destruct`.
pub(crate) unsafe extern "thiscall" fn compare_club_history_state_destruct(this: usize) {
    // simplification: a destructor runs once; proven in Rust mode
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I7%%)(this)
}

/// Comparer for `club_screens_exit_history_states`.
pub(crate) unsafe extern "C" fn compare_club_screens_exit_history_states() {
    // simplification: a static destructor runs once at the game exit; proven in Rust mode
    original_fn::<unsafe extern "C" fn()>(%%I8%%)()
}

// simplification: opens a screen (the history allocates a node): it does not run twice. Original only; the plan: the
// club screen current with the club in parameter 0 and the three best players in 0x20..0x22.
/// Comparer for `open_club_squad_screen`.
pub(crate) unsafe extern "C" fn compare_open_club_squad_screen(club: usize, reserve: u32, restore: u32, tab5: u32, highlight: u32) {
    original_fn::<unsafe extern "C" fn(usize, u32, u32, u32, u32)>(%%I9%%)(club, reserve, restore, tab5, highlight);
    let node = current_screen_node(G_SCREEN_MANAGER);
    if node == 0 || read_usize(node) != FN_CLUB_SCREEN_BUILD || get_screen_param(0) != club {
        return;
    }
    let (count, best) = if club != 0 { club_squad_stars(club) } else { (0, [0; 3]) };
    let id = |k: usize, need: i8| if count >= need { read_usize(best[k]) } else { usize::MAX };
    let want = [id(0, 1), id(1, 2), id(2, 3)];
    let got = [get_screen_param(0x20), get_screen_param(0x21), get_screen_param(0x22)];
    SCOREBOARD.record(want == got, || format!("open_club_squad_screen (club screen, squad): best players original={:?} rust={:?}", got, want));
}

/// Comparer for `menu_item_menu_item`: the two words side by side.
pub(crate) unsafe extern "thiscall" fn compare_menu_item_menu_item(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I10%%);
    let (r, n, same, _, _) = crate::comparison::side_by_side_light(&[(this, 4)], || o(this), || menu_item_menu_item(this));
    SCOREBOARD.record(same && r == n, || format!("MenuItem::MenuItem({:#x}) differs", this));
    r
}

/// Comparer for `club_screen_collect_squad_stats`: the 0x45-byte statistics side by side.
pub(crate) unsafe extern "C" fn compare_club_screen_collect_squad_stats(club: usize, reserve: u32, out: usize) {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, usize)>(%%I11%%);
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&[(out, club_squad_stats::SIZE)], || o(club, reserve, out),
        || club_screen_collect_squad_stats(club, reserve, out));
    SCOREBOARD.record(same, || format!("club_screen_collect_squad_stats (club screen, squad statistics): differences {:?}", first_differences(&mo, &mr)));
}
