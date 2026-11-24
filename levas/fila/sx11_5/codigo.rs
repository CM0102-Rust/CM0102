// ---------- batch %%LEVA%%: the program start, the human turn and the match day step (game.cpp) ----------

/// Exe routines and globals of the program start and the human turn (cm0102.exe 3.9.68).
#[allow(dead_code)]
pub(crate) mod game_start_fns {
    pub(crate) const CONFIG_CONSTRUCT: usize = 0x5d72c0; // Config::Config thiscall(this) -> this
    pub(crate) const HALL_OF_FAME_CONSTRUCT: usize = 0x5ef0b0; // HallOfFame::HallOfFame thiscall(this) -> this
    pub(crate) const HALL_OF_FAME_SIZE: u32 = 0x52080;
    pub(crate) const TEXT_EMPTY: usize = 0xad9c64; // ""
    pub(crate) const TEXT_NOMEMCHECK: usize = 0x9cd238; // "nomemcheck"
    /// Set to 1 once the settings and the hall of fame exist; cleared by `game_fatal_cleanup` [inference: the
    /// program-initialised flag].
    pub(crate) const G_PROGRAM_INITIALISED: usize = 0xb67a30;
    /// Nonzero for a session without human managers that still runs the job information screen [inference: the
    /// network spectator].
    pub(crate) const G_TURN_WITHOUT_HUMANS: usize = 0xb67a0c;
    /// A word that must be 0 for a human's urgent news to open the news screen [inference].
    pub(crate) const G_NEWS_SCREEN_BLOCKED: usize = 0xdd6bec;
}

/// Copies the command line into `dst` and lowercases it there (`exe::at_9532a6`, the C runtime `_strlwr`); a null
/// command line counts as the empty text.
unsafe fn store_command_line(dst: usize, command_line: usize) {
    copy_c_string(dst, if command_line == 0 { game_start_fns::TEXT_EMPTY } else { command_line });
    exe::at_9532a6(dst);
}

/// Whether a stored command line switches the memory check off: a non-empty line without "nomemcheck" (see the defect
/// in `game_init_from_command_line`).
unsafe fn command_line_switches_memory_check_off(line: usize) -> bool {
    read_u8(line) != 0 && exe::crt_strstr(line, game_start_fns::TEXT_NOMEMCHECK) == 0
}

/// The program start: the command line, lowercased, in `config_fns::G_COMMAND_LINE` (`store_command_line`); the
/// live settings (`operator new` 0x31b, `Config::Config`) in `G_CONFIG` and the hall of fame (`operator new` 0x52080,
/// `HallOfFame::HallOfFame`) in `G_HALL_OF_FAME`, 0 when an allocation fails; the program-initialised flag; and the
/// memory check (`G_MEMORY_CHECK_ENABLED`, on by default) switched off by `command_line_switches_memory_check_off`.
/// Returns 1.
///
/// Defects of the original: the "nomemcheck" test is inverted (the switch keeps the check on, any other non-empty
/// command line, such as "-windowed", turns it off); the command line is copied without a bound into a 100-byte buffer
/// (0xae2c98..0xae2cfb; a longer one overwrites the game phase byte 0xae2cfc and what follows).
/// Original: `game_init_from_command_line` (0x5cc6d0, game.cpp, cdecl).
pub(crate) unsafe extern "C" fn game_init_from_command_line(command_line: usize) -> u32 {
    store_command_line(config_fns::G_COMMAND_LINE, command_line);
    let p = exe::crt_operator_new(config_offsets::SIZE as u32);
    let config = if p != 0 {
        let c: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(game_start_fns::CONFIG_CONSTRUCT);
        c(p)
    } else {
        0
    };
    write_usize(G_CONFIG, config);
    let p = exe::crt_operator_new(game_start_fns::HALL_OF_FAME_SIZE);
    let hall = if p != 0 {
        let c: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(game_start_fns::HALL_OF_FAME_CONSTRUCT);
        c(p)
    } else {
        0
    };
    write_usize(G_HALL_OF_FAME, hall);
    write_u32(game_start_fns::G_PROGRAM_INITIALISED, 1);
    if command_line_switches_memory_check_off(config_fns::G_COMMAND_LINE) {
        write_u32(G_MEMORY_CHECK_ENABLED, 0);
    }
    1
}

/// The human manager in the k-th of the last 16 rows of the person table (k = 0 the last), as the original walks them.
unsafe fn human_turn_person(k: usize) -> Staff {
    staff_at(read_usize(G_STAFF_COUNT) as isize - k as isize - 1)
}

/// Opens a human's screen for the turn: the news screen (`exe::news_screens_open_news_screen`(person, 0)) when
/// `news` holds, otherwise his club screen (`exe::open_manager_club_screen`(person, 0, 0, 0, 0)).
unsafe fn human_turn_open(person: Staff, news: bool) {
    if news {
        exe::news_screens_open_news_screen(person, 0);
    } else {
        exe::open_manager_club_screen(person, 0, 0, 0, 0);
    }
}

/// `game_run_human_turn(next)`: the screens of the human managers' turn, from the game's day loop. Returns 1 while the
/// turn goes on (the screen loop `exe::screenmanager_monitor` ran), 0 when there is no turn.
///
/// - No human manager (`exe::count_human_managers(0)`) but `G_TURN_WITHOUT_HUMANS`: the job information screen and the
///   screen loop, 1.
/// - The game date differs from `*next` and the disk and memory have 25 MB free (`exe::os_get_free_disk_space_mb(0)`,
///   `exe::os_get_free_memory_mb()`): each human (classification byte set, among the last 16 rows) with urgent news
///   (`exe::news_has_urgent_items`(news, person, 1)), not on holiday (`exe::human_manager_is_on_holiday`) and
///   `G_NEWS_SCREEN_BLOCKED` 0 gets his news screen (`exe::news_screens_open_news_screen`(person, 1)). None: 0. Some:
///   the other humans get the news screen when they have neither a club nor a national job, otherwise their club screen;
///   `*next` becomes the game date plus a week (`exe::date_add_weeks`) and the screen loop runs, 1.
/// - Otherwise (the same date, or short of disk or memory): every human gets a screen (`human_turn_open`: the club
///   screen when he has a job and no urgent news (flag 0), otherwise the news screen); with no human at all, the manager
///   status screen (`setup_open_manager_status_screen`); `*next` the game date plus a week and the screen loop, 1.
/// Original: `game_run_human_turn` (0x5cde40, game.cpp, cdecl).
pub(crate) unsafe extern "C" fn game_run_human_turn(next: usize) -> u32 {
    if exe::count_human_managers(0) == 0 && read_u32(game_start_fns::G_TURN_WITHOUT_HUMANS) != 0 {
        let screen: unsafe extern "C" fn() = core::mem::transmute(0x6b0a10usize); // open_job_information_screen
        screen();
        exe::screenmanager_monitor(G_SCREEN_MANAGER);
        return 1;
    }
    let today: [u16; 4] = core::array::from_fn(|k| read_u16(G_CURRENT_DATE + 2 * k));
    let new_day = today[1] != read_u16(next + 2) || today[0] != read_u16(next);
    if new_day && exe::os_get_free_disk_space_mb(0) >= 0x19 && exe::os_get_free_memory_mb() >= 0x19 {
        let mut urgent = [false; 16];
        for (k, u) in urgent.iter_mut().enumerate() {
            let p = human_turn_person(k);
            if p.classification() != 0 && exe::news_has_urgent_items(read_usize(G_NEWS), p, 1) != 0
                && exe::human_manager_is_on_holiday(read_usize(G_HUMAN_MANAGER), p) == 0 && read_u16(game_start_fns::G_NEWS_SCREEN_BLOCKED) == 0 {
                exe::news_screens_open_news_screen(p, 1);
                *u = true;
            }
        }
        if !urgent.contains(&true) {
            return 0;
        }
        covered!("sx11_5 urgent news turn");
        for (k, &u) in urgent.iter().enumerate() {
            let p = human_turn_person(k);
            if !u && p.classification() != 0 {
                human_turn_open(p, p.club_job().is_null() && p.national_job().is_null());
            }
        }
    } else {
        let mut any = false;
        for k in 0..16 {
            let p = human_turn_person(k);
            if p.classification() == 0 {
                continue;
            }
            any = true;
            let jobless = p.club_job().is_null() && p.national_job().is_null();
            human_turn_open(p, jobless || exe::news_has_urgent_items(read_usize(G_NEWS), p, 0) != 0);
        }
        if !any {
            let screen: unsafe extern "C" fn() = core::mem::transmute(0x81fea0usize); // setup_open_manager_status_screen
            screen();
        }
    }
    let (mut week, mut tmp) = ([0u8; 8], [0u8; 8]);
    let d = exe::date_add_weeks(G_CURRENT_DATE, &mut week, 1);
    exe::date_operator_3d(next, tmp.as_mut_ptr() as usize, d);
    exe::screenmanager_monitor(G_SCREEN_MANAGER);
    1
}

/// `game_match_day_step_callback()`: the display's idle callback the day loop installs (0xdd6956): with a match day
/// (`G_MATCH_DAY`) it moves the day's background matches one step (`exe::match_day_step_live_match`(day, 0)).
/// Original: `game_match_day_step_callback` (0x5d61b0, game.cpp, cdecl).
pub(crate) unsafe extern "C" fn game_match_day_step_callback() {
    let day = read_usize(G_MATCH_DAY);
    if day != 0 {
        exe::match_day_step_live_match(day, 0usize);
    }
}

/// Comparer for `game_init_from_command_line`: it builds the settings (display, sound, files) and the hall of fame, so
/// only the original runs; then the plan: the stored command line is the argument lowercased (the copy's
/// `store_command_line` into a local), the settings and the hall of fame exist, the program is initialised and the
/// memory check follows `command_line_switches_memory_check_off`.
pub(crate) unsafe extern "C" fn compare_game_init_from_command_line(command_line: usize) -> u32 {
    let check_before = read_u32(G_MEMORY_CHECK_ENABLED);
    let mut plan = [0u8; 0x1000];
    store_command_line(plan.as_mut_ptr() as usize, command_line);
    let off = command_line_switches_memory_check_off(plan.as_ptr() as usize);
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I0%%)(command_line);
    let line = c_string_bytes(config_fns::G_COMMAND_LINE);
    let check = if off { 0 } else { check_before };
    let ok = r == 1 && line == c_string_bytes(plan.as_ptr() as usize) && read_usize(G_CONFIG) != 0 && read_usize(G_HALL_OF_FAME) != 0
        && read_u32(game_start_fns::G_PROGRAM_INITIALISED) == 1 && read_u32(G_MEMORY_CHECK_ENABLED) == check;
    SCOREBOARD.record(ok, || format!("game_init_from_command_line({:?}): stored {:?}, config {:#x}, hall of fame {:#x}, memory check {} (plan {})",
        String::from_utf8_lossy(&c_string_bytes(plan.as_ptr() as usize)), String::from_utf8_lossy(&line), read_usize(G_CONFIG),
        read_usize(G_HALL_OF_FAME), read_u32(G_MEMORY_CHECK_ENABLED), check));
    r
}

/// Comparer for `game_run_human_turn`: it opens screens and runs the screen loop, so only the original runs; a turn
/// (1) that moved `*next` moved it to the game date plus a week.
pub(crate) unsafe extern "C" fn compare_game_run_human_turn(next: usize) -> u32 {
    let before = read_u32(next) as u64 | (read_u32(next + 4) as u64) << 32;
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I1%%)(next);
    let after = read_u32(next) as u64 | (read_u32(next + 4) as u64) << 32;
    if r as u8 == 1 && after != before {
        let mut week = [0u8; 8];
        let d = exe::date_add_weeks(G_CURRENT_DATE, &mut week, 1);
        let ok = read_u16(next) == read_u16(d) && read_u16(next + 2) == read_u16(d + 2);
        SCOREBOARD.record(ok, || format!("game_run_human_turn: next turn {}/{}, plan {}/{}", read_i16(next), read_u16(next + 2), read_i16(d), read_u16(d + 2)));
    }
    r
}

/// Comparer for `game_match_day_step_callback`: it moves matches, so only the original runs.
pub(crate) unsafe extern "C" fn compare_game_match_day_step_callback() {
    original_fn::<unsafe extern "C" fn()>(%%I2%%)()
}
