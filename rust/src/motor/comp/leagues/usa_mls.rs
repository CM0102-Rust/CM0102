use super::*;

/// Row i of a league/stage table (+0xb1, 0x3b bytes): club and points (+0x19).
pub(crate) unsafe fn table_row_club_points(t: League, i: usize) -> (usize, i16) {
    let l = LeagueTableRow(t.table() + i * league_table_row::SIZE);
    (read_usize(l.addr()), l.points())
}

/// Writes the 6-read_i8(club + points) record k into a byte vector.
pub(crate) fn put_club_points_record(r: &mut [u8], k: usize, (c, p): (usize, i16)) {
    r[6 * k..6 * k + 4].copy_from_slice(&(c as u32).to_le_bytes());
    r[6 * k + 4..6 * k + 6].copy_from_slice(&p.to_le_bytes());
}

/// Club of the 6-byte record k of a byte vector.
pub(crate) fn club_of_record(r: &[u8], k: usize) -> usize {
    u32::from_le_bytes([r[6 * k], r[6 * k + 1], r[6 * k + 2], r[6 * k + 3]]) as usize
}

/// The MLS has three divisions of four (the league and stages 0 and 1, played together in the regular season), the
/// play-off (stage 2) and the final (stage 3): the next stage goes from stage 1 to the play-off and from the play-off
/// to the final, building each; past the last stage it is an error (the box)
/// Original: `UsaMls::advance_stage` (0x909400, usa_mls.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn usa_mls_advance_stage(this: usize) {
    let this = League(this);
    let stage = this.current_stage() as i32;
    if stage >= (this.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(0xaa1fb4, 0x55b);
    }
    if stage == 1 {
        this.set_current_stage((2) as usize);
    } else if stage == 2 {
        this.set_current_stage((3) as usize);
    }
    match this.current_stage() {
        2 => {
            let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_USAMLS_CREATE_PLAYOFF_STAGE);
            f(this.addr());
        }
        3 => {
            let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_USAMLS_CREATE_FINAL_STAGE);
            f(this.addr());
        }
        _ => {}
    }
}

/// Creates the MLS play-off with eight clubs: the three division winners, ordered by points, and the five best of the
/// rest; the winners go to the final. `UsaMls::create_playoff_stage()`, the MLS play-off, from the next stage; SEH
/// frame only for the new. 1) A list of 8 (malloc `exe::crt_malloc` of 0x20; out of memory: the box for line 0x58d and
/// it returns). 2) The three division winners (the first of the league table, of stage 0 and of stage 1), in 6-byte
/// records (the club and the points, +0x19 of the row), sorted by points (qsort `exe::crt_qsort` with 0x90a130: more
/// points first), go to slots 1, 3 and 5 of the list. 3) The others: the second, third and fourth of each table (in the
/// order: league, stage 0, stage 1, row by row), nine records sorted by points in the same way (ties with no rule of
/// their own); the five best go to slots 7, 6, 4, 2 and 0 (the best in 7). The play-off pairs: the best winner against
/// the fifth best of the rest, the second winner against the fourth, the third winner against the third, and the best
/// of the rest against the second [inference: the pairs follow the list]. 4) The round table through virtual
/// `comp_vtable::CREATE_SCHEDULE` (thiscall(this, 2, &rounds, &extra, 0), ret 0x10); the stage (operator new
/// `exe::crt_operator_new`(0xb2) and the Cup_Stage constructor `FN_CUP_STAGE_CUP_STAGE`, thiscall, ret 0x3c: the
/// league, 8, the list, the rounds, `.competition()`, the table, the year `.year()`, 2, 1, the extra, 0x12, 1, 0, 0 and
/// null seeds; the port passes clean words) into `.stages()`[2] and the table freed (free `exe::crt_free`). Without a
/// stage: the box for line 0x5c4. With it: Cup::set_up_round (`exe::cup_set_up_round`, thiscall(stage, 0), ret 4). The
/// list is not freed (the stage keeps it; the original leaves it so).
/// Original: `UsaMls::create_playoff_stage` (0x9094c0, usa_mls.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn usa_mls_create_playoff_stage(this: usize) {
    let this = League(this);
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, usize) -> usize =
        core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
    let (a, b) = (read_usize(this.stages()), read_usize(this.stages() + 4));
    let list = exe::crt_malloc(0x20);
    if list == 0 {
        return show_error_at(0xaa1fb4, 0x58d);
    }
    let poe = |k: usize, c: usize| write_usize(list + 4 * k, c);
    let mut r = [0u8; 54];
    put_club_points_record(&mut r, 0, table_row_club_points(this, 0));
    put_club_points_record(&mut r, 1, table_row_club_points(League(a), 0));
    put_club_points_record(&mut r, 2, table_row_club_points(League(b), 0));
    exe::crt_qsort((r.as_mut_ptr()) as usize, 3, 6, 0x90a130);
    poe(1, club_of_record(&r, 0)); // cqr4: the best division champion
    poe(3, club_of_record(&r, 1));
    poe(5, club_of_record(&r, 2));
    let mut k = 0usize;
    for j in 1..4 {
        for t in [this.addr(), a, b] {
            put_club_points_record(&mut r, k, table_row_club_points(League(t), j));
            k += 1;
        }
    }
    exe::crt_qsort((r.as_mut_ptr()) as usize, 9, 6, 0x90a130);
    poe(7, club_of_record(&r, 0));
    poe(6, club_of_record(&r, 1));
    poe(4, club_of_record(&r, 2));
    poe(2, club_of_record(&r, 3));
    poe(0, club_of_record(&r, 4)); // cqr4: the fifth best of the others
    let table: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = table(this.addr(), 2, &mut rodadas, &mut extra, 0);
    let p = exe::crt_operator_new(0xb2);
    let stage = if p != 0 {
        construct(p, this.addr(), 8, list, rodadas as u16 as u32, this.competition().addr(), t, this.year() as u16 as u32, 2, 1,
                  extra as u16 as u32, 0x12, 1, 0, 0, 0)
    } else {
        0
    };
    write_usize(this.stages() + 8, stage);
    exe::crt_free(t);
    let stage = read_usize(this.stages() + 8);
    if stage == 0 {
        return show_error_at(0xaa1fb4, 0x5c4);
    }
    covered!("cqr_4 play-off");
    exe::cup_set_up_round(stage, 0);
}

/// Creates the MLS Cup final: the two play-off winners in a two-club cup stage [inference: a single game]; the winner
/// is the MLS champion. `UsaMls::create_final_stage()`, the MLS Cup final, from the next stage; SEH frame only for the
/// new. The first two play-off teams (`.stages()`[2], the 6-byte records of `.team_table()`, in order) with the winner
/// mark (+5 = 1) in a list of 2 (malloc `exe::crt_malloc` of 8; out of memory: the box for line 0x5e5 and it returns);
/// the round table through virtual `comp_vtable::CREATE_SCHEDULE` (thiscall(this, 3, &rounds, &extra, 0), ret 0x10);
/// the stage (operator new `exe::crt_operator_new`(0xb2) and the Cup_Stage constructor `FN_CUP_STAGE_CUP_STAGE`: the
/// league, 2, the list, the rounds, `.competition()`, the table, the year, 3, 1, the extra, 0x14, 0, 0, 0 and null
/// seeds) into `.stages()`[3] and the table freed. Without a stage: the box for line 0x5fe and it returns (without
/// freeing the list). With it: Cup::set_up_round (`exe::cup_set_up_round`, thiscall(stage, 0)) and the list freed (free
/// `exe::crt_free`).
/// Original: `UsaMls::create_final_stage` (0x9097d0, usa_mls.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn usa_mls_create_final_stage(this: usize) {
    let this = League(this);
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, usize) -> usize =
        core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
    let po = Cup(read_usize(this.stages() + 8));
    let list = exe::crt_malloc(8);
    if list == 0 {
        return show_error_at(0xaa1fb4, 0x5e5);
    }
    let mut n = 0i8;
    let mut i = 0i32;
    while i < po.team_count() as i32 && n < 2 {
        let e = po.team_table() + 6 * i as usize;
        if read_u8(e + 5) == 1 {
            write_usize(list + 4 * n as usize, read_usize(e)); // cqr4: os finalistas
            n += 1;
        }
        i += 1;
    }
    let table: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = table(this.addr(), 3, &mut rodadas, &mut extra, 0);
    let p = exe::crt_operator_new(0xb2);
    let stage = if p != 0 {
        construct(p, this.addr(), 2, list, rodadas as u16 as u32, this.competition().addr(), t, this.year() as u16 as u32, 3, 1,
                  extra as u16 as u32, 0x14, 0, 0, 0, 0)
    } else {
        0
    };
    write_usize(this.stages() + 0xc, stage);
    exe::crt_free(t);
    let stage = read_usize(this.stages() + 0xc);
    if stage == 0 {
        return show_error_at(0xaa1fb4, 0x5fe);
    }
    covered!("cqr_4 final");
    exe::cup_set_up_round(stage, 0);
    exe::crt_free(list);
}

/// True if club c is in this league's competition and its MLS division code matches.
pub(crate) unsafe fn is_in_mls_division(this: League, c: Club, code: i16) -> bool {
    c.division() == this.competition() && (exe::usa_mls_get_club_division(c) as u32) as u16 as i16 == code
}

/// Creates the MLS table (the first division, code 0x413): the four clubs of the competition with that division code,
/// in database order; succeeds if it found all four. `UsaMls::create_table()`: `.club_count()` = 4; the table (malloc
/// `exe::crt_malloc` of 0xec, 4 rows of 0x3b; out of memory: the box for line 0x2fe and 0) at `.table()`; in database
/// order (`G_CLUB_TABLE`, `G_CLUB_COUNT` clubs of `club::SIZE` bytes, i16 counter), while rows are missing, each club
/// of the competition with code 0x413 goes to the next row (League::init_table_row `exe::league_init_table_row`: push
/// 0; push 0; push club; push eax with the row in the low byte and the code's garbage above, which it does not read;
/// mov ecx -> thiscall, ret 0x10). Returns 1 if it filled the 4 rows.
/// Original: `UsaMls::create_table` (0x908560, usa_mls.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn usa_mls_create_table(this: usize) -> u32 {
    let this = League(this);
    this.set_club_count(4);
    let t = exe::crt_malloc(0xec);
    this.set_table(t);
    if t == 0 {
        show_error_at(0xaa1fb4, 0x2fe);
        return 0;
    }
    let mut n: i16 = 0;
    let base = read_usize(G_CLUB_TABLE);
    let mut i: i16 = 0;
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 && n < this.club_count() {
        let c = base + i as u16 as usize * club::SIZE;
        if is_in_mls_division(this, Club(c), 0x413) {
            exe::league_init_table_row(this, n as u8 as u32, c, 0, 0); // cqs1: the next row of the first division
            n += 1;
        }
        i = i.wrapping_add(1);
    }
    covered!("cqs_1 mls fixtures");
    (n == this.club_count()) as u32
}

/// Creates the stage of one of the other MLS divisions (`create_second_division_stage` and
/// `create_third_division_stage`): club vector, round table, League_Stage, stored in slot k with the division code.
///
/// Same template in both, checked instruction by instruction: only the division code, slot k, the error boxes' source
/// file and the lines change.
///
/// 1) A vector of +0x3e clubs (malloc `exe::crt_malloc` of 4 × +0x3e; when out of memory: the error box for line `linhas[0]` and
/// return) with the clubs of the competition with `codigo`, in database order (i32 counter), while there are slots
/// left.
///
/// 2) The round table from virtual +0x3c (create_round_table: thiscall(this, −3, &rodadas, &extra, 0), ret 0x10).
///
/// 3) The stage: operator new `exe::crt_operator_new`(0xee) and the League_Stage constructor 0x68b100 (thiscall, ret 0x58, 22
/// arguments, in the original routine's order: the league, +0x3e (word), the vector, +0x3c (word, the rounds), +4, the
/// round table, the rounds (word), the points +0xc2 (`pv`) and +0xc3, the category +0xc4, &+0xc5, &+0xbe (the league's
/// places), the year +0x40 (word), k, the extra (word), +0x51, 1, +0xab (word), +0xd9 (word), +0xa7 (word), 0, 2; the
/// copy passes clean values). When out of memory: 0.
///
/// 4) [+0xc][k] = the stage; the vector and the round table freed (free `exe::crt_free`, in that order). Without a stage: the
/// error box for line `linhas[1]`. With one: +0x30 = `atual` (the original writes k) and the stage's +0x3a = the
/// division code.
pub(crate) unsafe fn create_division_stage(this: League, code: i16, k: u32, current: u32, pv: usize, rows: [u32; 2], ramo: &'static str) {
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let v = exe::crt_malloc(((this.club_count() as i32) << 2) as u32);
    if v == 0 {
        return show_error_at(0xaa1fb4, rows[0]);
    }
    let mut n: i16 = 0;
    let base = read_usize(G_CLUB_TABLE);
    let mut i = 0i32;
    while i < read_usize(G_CLUB_COUNT) as i32 && n < this.club_count() {
        let c = base + i as usize * club::SIZE;
        if is_in_mls_division(this, Club(c), code) {
            write_usize(v + 4 * n as usize, c);
            n += 1;
        }
        i += 1;
    }
    let table: unsafe extern "thiscall" fn(usize, i32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = table(this.addr(), -3, &mut rodadas, &mut extra, 0);
    let p = exe::crt_operator_new(0xee);
    let stage = if p != 0 {
        construct(p, this.addr(), this.club_count() as u16 as u32, v, this.legs() as u16 as u32, this.competition().addr(), t, rodadas as u16 as u32,
                  read_u8(this.addr() + pv) as u32, this.draw_points() as u32, this.match_category() as u32, this.addr() + league::TIEBREAKS, this.addr() + league::PROMOTED_COUNT,
                  this.year() as u16 as u32, k, extra as u16 as u32, this.importance() as u32, 1, this.match_rules() as u16 as u32,
                  this.league_flags() as u16 as u32, this.games_per_club() as u16 as u32, 0, 2)
    } else {
        0
    };
    write_usize(this.stages() + 4 * k as usize, stage);
    exe::crt_free(v);
    exe::crt_free(t);
    let stage = read_usize(this.stages() + 4 * k as usize);
    if stage == 0 {
        return show_error_at(0xaa1fb4, rows[1]);
    }
    this.set_current_stage((current) as usize);
    covered!(ramo);
    write_i16(stage + league::STAGE_ID, code);
}

/// The second MLS division (code 0x47b): a round-robin stage with its four clubs (in database order), the same rounds,
/// points, tie-breakers and places as the league, as stage 0; the three division winners and the five best of the rest
/// go to the play-off
/// Original: `UsaMls::create_second_division_stage` (0x908fc0, usa_mls.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn create_second_division_stage(this: usize) {
    create_division_stage(League(this), 0x47b, 0, 0, 0xc2, [0x4e6, 0x506], "cqs_1 segunda divisao") // cqs1: the points per win in the second
}

/// The third MLS division (code 0x414): the same template as the second, as stage 1; the current stage becomes 1, from
/// which the next stage goes to the play-off
/// Original: `UsaMls::create_third_division_stage` (0x9091e0, usa_mls.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn create_third_division_stage(this: usize) {
    create_division_stage(League(this), 0x414, 1, 1, 0xc2, [0x524, 0x544], "cqs_1 terceira divisao") // cqs1: the current stage after the third
}

/// The comparer of an MLS division stage: runs only the original (it allocates the array, the round table and the
/// stage), checked against the plan made beforehand.
///
/// The plan: the clubs with the code, in database order. Afterwards, the stage in slot k with the league's +0x3e, the
/// clubs in its table (as a set), +0x3a = the code and +0x30 = k.
pub(crate) unsafe fn compare_division_stage(name: &str, this: League, code: i16, k: usize, original: impl FnOnce()) {
    let mut plan = [0usize; 8];
    let mut n = 0usize;
    let base = read_usize(G_CLUB_TABLE);
    let mut i = 0i32;
    while i < read_usize(G_CLUB_COUNT) as i32 && (n as i16) < this.club_count() && n < 8 {
        let c = base + i as usize * club::SIZE;
        if is_in_mls_division(this, Club(c), code) {
            plan[n] = c;
            n += 1;
        }
        i += 1;
    }
    original();
    let stage = League(read_usize(this.stages() + 4 * k));
    let same = !stage.is_null() && stage.stage_id() == code && this.current_stage() == k && stage.club_count() == this.club_count() && {
        let mut in_it = [0usize; 8];
        for (j, c) in in_it.iter_mut().enumerate().take(n) {
            *c = read_usize(stage.table() + j * league_table_row::SIZE);
        }
        let (mut a, mut b) = (plan, in_it);
        a[..n].sort_unstable();
        b[..n].sort_unstable();
        a[..n] == b[..n]
    };
    SCOREBOARD.record(same, || format!("{}({:#x}) stage={:#x} n={} plan={:x?}", name, this.addr(), stage.addr(), n, plan));
}

/// `UsaMls::start_new_season()` (`0x908e00`, thiscall, ret 0, `usa_mls.cpp`): the MLS season rollover.
///
/// `+0x4c` = 0 and the table freed (League::free_table `0x687970`, thiscall(this, 0), ret 4); the schedule (`+0xba`)
/// freed (free `0x9452ca`) and zeroed; with the current stage ≥ 0, the existing stages 0..=`+0x30` destroyed (virtual
/// 0, the destructor with 1); `+0x30` = −1; with statistics (`+8`), Comp_Stats::archive_season_and_reset (`0x4a1c50`,
/// thiscall(`+8`, 1), ret 4); the year (`+0x40`, word) goes up by `ano` (the counter "cqs_2 mls rollover").
///
/// Then, each with its own box (lines `0x4af`, `0x4b6`, `0x4bc` and `0x4c3`) if it returns 0: the rules (virtual
/// `+0x8c`), the first division's table (`0x908560`), the season's fixtures (UsaMls::create_season_fixtures `0x9086e0`,
/// which also builds the stages of the other two divisions) and the table order (League::sort_table `0x6827d0`,
/// thiscall(this, 0), ret 4); finally the predicted positions (virtual `+0x5c`), the boards' expectations
/// (League::set_club_season_expectations `0x68aa80`, thiscall, ret 0) and the renewal of the referees
/// (OfficialsManager::renew_officials `0x79cee0`, thiscall([`0xb74340`], `+4`), ret 4), whose eax it returns.
///
/// After a box, the copy returns 0 (the original routine returns the box's eax; the rollover is called by the rollover
/// loop, which does not read it). The stage vector (`+0xc`) stays: the divisions and the play-off rewrite the slots.
pub(crate) unsafe fn mls_start_new_season(this: League, year: i16) -> u32 {
    write_u32(this.addr() + league::SEASON_CLOSED, 0);
    exe::league_free_table(this, 0);
    let l = this.round_dates();
    if l != 0 {
        exe::crt_free(l);
        this.set_round_dates(0);
    }
    if (this.current_stage() as i32) >= 0 {
        let mut i = 0i32;
        loop {
            let f = read_usize(this.stages() + 4 * i as usize);
            if f != 0 {
                let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(f)));
                destroy(f, 1);
            }
            i += 1;
            if i > this.current_stage() as i32 {
                break;
            }
        }
    }
    write_i32(this.addr() + league::CURRENT_STAGE, -1);
    let stats = this.comp_stats();
    if stats != 0 {
        exe::comp_stats_archive_season_and_reset(stats, 1);
    }
    let a = this.year();
    this.set_year(a.wrapping_add(year));
    covered!("cqs_2 mls rollover");
    let rules: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::SET_UP_RULES));
    if rules(this.addr()) == 0 {
        show_error_at(0xaa1fb4, 0x4af);
        return 0;
    }
    if exe::usamls_create_table(this) == 0 {
        show_error_at(0xaa1fb4, 0x4b6);
        return 0;
    }
    if exe::usamls_create_season_fixtures(this) == 0 {
        show_error_at(0xaa1fb4, 0x4bc);
        return 0;
    }
    if exe::league_sort_table(this, 0) == 0 {
        show_error_at(0xaa1fb4, 0x4c3);
        return 0;
    }
    let predicted: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::SET_PREDICTED_POSITIONS));
    predicted(this.addr());
    exe::league_set_club_season_expectations(this);
    exe::officialsmanager_renew_officials(read_usize(G_OFFICIALS_MANAGER), this.competition().addr()) as u32  // cqs2: the renewal of the MLS referees
}

/// The MLS turnover: clears the table, the schedule and the stages (the other two divisions, the play-off and the
/// final), archives the season in the statistics, moves to the next year and rebuilds the rules, the first division,
/// the season fixtures (with the other two division stages) and the order; then the predicted positions, the board
/// expectations and the referee renewal
/// Original: `UsaMls::start_new_season` (0x908e00, usa_mls.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn usa_mls_start_new_season(this: usize) -> u32 {
    mls_start_new_season(League(this), 1) // cqs2: the next year of the MLS
}

/// Sets the MLS predicted positions: in each division the four clubs are ordered by reputation and get positions 1, 4,
/// 7 and 10, one scale shared by the three divisions. `UsaMls::set_predicted_positions()`. Without statistics
/// (`.comp_stats()`): the box for line 0x675 and it returns. A vector of 3 × `.club_count()` clubs (malloc
/// `exe::crt_malloc`; out of memory: the box for line 0x67d and it returns): those of the league table, those of stage
/// 0 and those of stage 1 (the three divisions, row by row), each block of `.club_count()` sorted by reputation (qsort
/// `exe::crt_qsort` with 0x4b07d0, highest first, nulls at the end). The first twelve of the vector get, in each
/// division, positions 1, 4, 7 and 10 (Comp_Stats::set_club_final_position `exe::comp_stats_set_club_final_position`,
/// thiscall(`.comp_stats()`, club, position), ret 8; the order of the twelve calls is the vector's); the vector is
/// freed (free `exe::crt_free`). [inference] The three divisions are mixed in the prediction: each division's favourite
/// is predicted 1st, the second 4th, the third 7th and the last 10th.
/// Original: `UsaMls::set_predicted_positions` (0x909bb0, usa_mls.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn usa_mls_set_predicted_positions(this: usize) {
    let this = League(this);
    let n3 = this.club_count().wrapping_mul(3);
    if this.comp_stats() == 0 {
        return show_error_at(0xaa1fb4, 0x675);
    }
    let v = exe::crt_malloc(((n3 as i32) << 2) as u32);
    if v == 0 {
        return show_error_at(0xaa1fb4, 0x67d);
    }
    let mut i: i16 = 0;
    while i < this.club_count() {
        let n = this.club_count() as i32;
        let k = i as usize;
        write_usize(v + 4 * k, aus_nsl_table_row_club(this, k));
        write_usize(v + 4 * (k + n as usize), aus_nsl_table_row_club(League(read_usize(this.stages())), k));
        write_usize(v + 4 * (k + 2 * n as usize), aus_nsl_table_row_club(League(read_usize(this.stages() + 4)), k));
        i += 1;
    }
    let n = this.club_count() as i32;
    exe::crt_qsort(v, n as u32, 4, 0x4b07d0);
    let n = this.club_count() as i32;
    exe::crt_qsort(v + 4 * n as usize, n as u32, 4, 0x4b07d0);
    let n = this.club_count() as i32;
    exe::crt_qsort(v + 8 * n as usize, n as u32, 4, 0x4b07d0);
    let places: [u32; 12] = [1, 4, 7, 10, 1, 4, 7, 10, 1, 4, 7, 10]; // cqs2: the predicted positions of the three divisions
    for (k, p) in places.iter().enumerate() {
        exe::comp_stats_set_club_final_position(this.comp_stats(), read_usize(v + 4 * k), *p);
    }
    covered!("cqs_2 expected positions");
    exe::crt_free(v);
}

/// The list of the twelve MLS clubs: the four of the first division, then the four of the second and the four of the
/// third, in table order [inference: used by the MLS All-Star game]
/// Original: `UsaMls::get_club_array` (0x909a40, usa_mls.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn usa_mls_get_club_array(this: usize, list: *mut usize, n: *mut i16) {
    let this = League(this);
    *n = 0xc;
    let v = exe::crt_malloc(0x30);
    *list = v;
    if v == 0 {
        show_error_at(0xaa1fb4, 0x64c);
        *n = 0;
        return;
    }
    let mut i: i16 = 0;
    while i < this.club_count() {
        write_usize(*list + 4 * i as usize, aus_nsl_table_row_club(this, i as usize));
        i += 1;
    }
    let mut i: i16 = 0;
    while i < this.club_count() {
        let k = this.club_count() as i32 + i as i32;
        write_usize(*list + 4 * k as usize, aus_nsl_table_row_club(League(read_usize(this.stages())), i as usize));
        i += 1;
    }
    let mut i: i16 = 0;
    while i < this.club_count() {
        let k = 2 * this.club_count() as i32 + i as i32;
        write_usize(*list + 4 * k as usize, aus_nsl_table_row_club(League(read_usize(this.stages() + 4)), i as usize));
        i += 1;
    }
    covered!("cqs_2 mls clubs");
}

/// Staff_Records::add_failed_to_qualify_record (`add_failed_to_qualify_record`) for a club.
pub(crate) unsafe fn usa_mls_add_failed_to_qualify_record(club: usize, comp: usize, stage: u32, weight: u32) {
    exe::staff_records_add_failed_to_qualify_record(read_usize(G_STAFF_RECORDS), club, comp, stage, weight);
}

/// comp_util_scale_by_round: weight 50..70 by the match round.
pub(crate) unsafe fn weight_by_round(game: usize) -> u32 {
    let game = Fixture(game);
    exe::comp_util_scale_by_round(0x32, 0x46, game.round_number() as u16 as u32, 2) as u32 
}

/// The MLS stage-end outcome (`usa_mls_update_staff_records_for_status`): qualified, failed or knocked-out records by situation and phase; dry run
/// without effects when `fazer` is false.
pub(crate) unsafe fn mls_stage_outcome(this: usize, club: usize, status: u32, stage: u32, game: usize, fazer: bool) -> u32 {
    let game = Fixture(game);
    let (st, fa) = (status as u8, stage as u8 as i8);
    let comp = read_usize(this + 4);
    if fa < 2 {
        // the season (-1) and the first two play-off rounds
        if st == 6 {
            if fazer {
                covered!("cqk_9 mls qualified");
                bra_nat_first_add_qualified_record(club, comp, 0xa0, 0x78, 0x28); // cqk9: the MLS qualifier
            }
            return 0;
        }
        if st != 0xfe {
            return 0;
        }
        if fazer {
            let f = if fa == -1 { 0x413 } else if fa != 0 { 0x414 } else { 0x47b };
            usa_mls_add_failed_to_qualify_record(club, comp, f, 0x28);
        }
        return u32::MAX;
    }
    if fa == 2 {
        // a final
        if st == 1 {
            if fazer {
                bra_nat_first_add_qualified_record(club, comp, 0x417, 0x96, 0x50);
            }
        } else if st == 0 {
            if fazer {
                // simplification: the original pushes whole eax/ecx/edx (garbage above the words: the caller’s, the half
                // from above this); 0x4bf850 and 0x86bdd0 only read the words: the words go.
                let weight = weight_by_round(game.addr());
                bra_nat_first_add_qualified_record(club, comp, game.round_prefix() as u16 as u32, game.round_name() as u16 as u32, weight);
            }
        } else if st == 0xff || st == 2 {
            if fazer {
                let weight = weight_by_round(game.addr());
                exe::staff_records_add_knocked_out_record(read_usize(G_STAFF_RECORDS), club, comp, game.round_prefix() as u16 as u32, game.round_name() as u16 as u32, weight);
            }
        }
        return 0;
    }
    if st == 1 && fazer {
        let f: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_STAFF_RECORDS_ADD_LEAGUE_CHAMPIONS_RECORD);
        f(read_usize(G_STAFF_RECORDS), club, comp);
    }
    0
}

/// MLS: in the season and the first two play-off rounds, those who go through get the qualification record (stage 0xa0,
/// round 0x78, weight 0x28) and those left out the failed-to-qualify one; in the final, the winner gets the final
/// record (weight 0x50) and the others the round one, weighted 50 to 70; then the champion gets the champion record
/// Original: `UsaMls::update_staff_records_for_status` (0x908200, usa_mls.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn usa_mls_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, a4: u32, game: usize, a6: u32) -> u32 {
    let _ = (a4, a6);
    mls_stage_outcome(this, club, status, stage, game, true)
}

/// MLS result: each game goes to the overall table and, in the regular season, also to the two conference tables (East
/// and West [inference]), without counting the statistics twice. `UsaMls::add_result(game, stats)` (slot 2): the match
/// goes to the overall table (`League::add_result` `exe::league_add_result`) with the match's mark
/// `fixture::STAGE_INDEX` set to -1 when below 2; in stage 1 (`.current_stage()`, the regular season), it also goes to
/// the tables of the two conferences (stages 0 and 1, through their slot 2 `comp_vtable::ADD_RESULT`, with mark 0 and 1
/// and without the statistics block `.comp_stats()`, which is saved and restored). Returns 1.
/// Original: `UsaMls::add_result` (0x9099d0, usa_mls.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn usa_mls_add_result(this: usize, game: usize, stats: u32) -> u32 {
    let game = Fixture(game);
    let this = League(this);
    if (game.stage_index() as i8) < 2 {
        game.set_stage_index(0xff);
    }
    exe::league_add_result(this, game, stats);
    if this.current_stage() != 1 {
        return 1;
    }
    let stored = this.comp_stats();
    this.set_comp_stats((0) as usize);
    game.set_stage_index(0);
    let estagio = |k: usize| -> (usize, unsafe extern "thiscall" fn(usize, usize, u32) -> u32) {
        let e = read_usize(this.stages() + 4 * k);
        (e, core::mem::transmute(read_usize(read_usize(e) + comp_vtable::ADD_RESULT)))
    };
    let (e0, f0) = estagio(0);
    f0(e0, game.addr(), stats);
    game.set_stage_index(1);
    let (e1, f1) = estagio(1);
    covered!("cqk_9 mls second conference");
    f1(e1, game.addr(), stats); // cqk9: the match in the second conference table
    this.set_comp_stats((stored as u32) as usize);
    1
}

/// Returns the club's conference in the MLS All-Star game: East (0x413) or West (0x414); clubs outside the MLS, none.
/// `usa_mls_get_club_all_star_conference(club)` (cdecl): the club's conference in the All-Star game: 0x413 (East) for
/// New England, MetroStars, Tampa Bay, D.C. United, Columbus and Miami (the keys 0x9d0390, 0x9d0394, 0x9d0388,
/// 0x9d0384, 0x9d03a4, 0x9d0380), 0x414 (West) for Colorado, Dallas, Kansas City, Los Angeles, San Jose and Chicago
/// (0x9d03a8, 0x9d03a0, 0x9d039c, 0x9d0398, 0x9d038c, 0x9d037c); 0 for a club whose league (`.division()`) is not the
/// MLS (key `KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE`); an MLS club outside the twelve: the "Error" box (as in the
/// original).
/// Original: `usa_mls_get_club_all_star_conference` (0x90a000, usa_mls.cpp, cdecl).
pub(crate) unsafe extern "C" fn usa_mls_get_club_all_star_conference(club: usize) -> u16 {
    let club = Club(club);
    let nation = club.division().addr();
    if nation == 0 || read_usize(nation) != read_usize(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE) {
        return 0;
    }
    let id = read_usize(club.addr());
    const LESTE: [usize; 6] = [0x9d0390, 0x9d0394, 0x9d0388, 0x9d0384, 0x9d03a4, 0x9d0380];
    const OESTE: [usize; 6] = [0x9d03a8, 0x9d03a0, 0x9d039c, 0x9d0398, 0x9d038c, 0x9d037c];
    if LESTE.iter().any(|&k| read_usize(k) == id) {
        covered!("cqk_9 conference 0x413");
        return 0x413; // cqk9: the first conference
    }
    if OESTE.iter().any(|&k| read_usize(k) == id) {
        return 0x414;
    }
    original_fn::<unsafe extern "C" fn(usize) -> u16>(IDX_USA_MLS_GET_CLUB_ALL_STAR_CONFERENCE)(club.addr())
}

/// MLS: 32 games per team (+0xa7; four round robins in +0x3c); win 3, draw 1; tie-break by goal difference, goals
/// scored and head-to-head; 1 top place in the overall table (rule word 0x48: 0x8 next stage, the play-offs; 0x40 fixed
/// round order); nobody goes up or down; the word +0xab = 0xc2 copied to each match. No schedule here: the games are
/// built by hand (`exe::usamls_create_season_fixtures`)
/// Original: `UsaMls::set_up_rules` (0x908660, usa_mls.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn usa_mls_set_up_rules(this: usize) -> u32 {
    set_mls_rules(this);
    covered!("cqj_8 mls rules");
    1
}

/// MLS: in the regular season and play-off stages 0 and 1, the current position 1, 2, 3, 4 becomes 1, 3, 7, 10, the
/// lowest 1 to 3 becomes 1 and 4 becomes 10, the highest 1 to 3 becomes 7 and 4 becomes 10 (positions on the
/// whole-league scale [inference]); in stage 2, each code becomes 2 × code − 1; after that, as they come
/// Original: `UsaMls::set_club_progress` (0x909dd0, usa_mls.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn usa_mls_set_club_progress(this: usize, club: usize, stage: u32, a: u32, b: u32, c: u32) -> usize {
    let e = bra_reg_minas_gerais_comp_stats_club_entry(this, club);
    let (fa, a, b, c) = (stage as u8 as i8, a as u8, b as u8, c as u8);
    if fa < 2 {
        let (b, c) = (mls_progress_code(b, 1), mls_progress_code(c, 7));
        let a = match a {
            1 => 1,
            2 => {
                covered!("cqk_8 mls second");
                3 // cqk8: the MLS code 2
            }
            3 => 7,
            4 => 10,
            _ => a,
        };
        write_progress_codes(e, a, b, c);
    } else if fa == 2 {
        write_progress_codes(e, a.wrapping_mul(2).wrapping_sub(1), b.wrapping_mul(2).wrapping_sub(1), c.wrapping_mul(2).wrapping_sub(1));
    } else {
        write_progress_codes(e, a, b, c);
    }
    e
}

/// The MLS season fixtures: the first division's own round robin, the other two divisions as stages, then the
/// cross-division matches (every club meets clubs of the other two divisions), and the round table of the league's own
/// stage again. Returns 1.
///
/// In order: the league's round table for stage −3 (virtual `comp_vtable::CREATE_SCHEDULE`: push 0; push
/// &`league::STAGE_ID`; push &`league::ROUND_COUNT`; push −3 -> thiscall, ret 0x10) into `round_dates()`, the fixtures
/// of the first division (League::create_fixtures `create_fixtures`, thiscall, ret 0) and that table freed (free
/// `exe::crt_free`, the pointer left as it is); the second and third division stages
/// (UsaMls::create_second_division_stage and UsaMls::create_third_division_stage, thiscall, ret 0); the two stages read
/// (`stages()`[0] and [1]); the cross-division table, stage −2 (the same call), into `round_dates()`.
///
/// The match record (0x4f bytes on the stack): +0 the competition id, `fixture::COMPETITION`, `fixture::SEASON_YEAR`
/// (`year()`), `fixture::ROUND_NAME` 0, `fixture::ROUND_PREFIX` (`stage_id()`), `fixture::CATEGORY`
/// (`match_category()`, then per round), `fixture::ROUND_DRAW_RULES` (`round_draw_rules()`), `fixture::TIE_RULES`
/// (`match_rules()`), `fixture::SQUAD_RULES` low nibble `substitutes_named()` and next nibble `substitutions_allowed()`
/// (bits 8..10 cleared), `fixture::STAGE_INDEX` (`stage_index()`), +0x43..+0x4c 0xff, `fixture::MATCH_STATS_SLOT` −1,
/// the result marks 0, `fixture::STAGE` 0xff, both side marks 1, and `fixture::NEXT_ROUND_DAY` /
/// `fixture::NEXT_ROUND_YEAR` the last date of the cross-division table (Date::Date `exe::new_date` of its day and its
/// year + `year()`).
///
/// For each round k of the table: `fixture::YEAR`, `fixture::DAY` and `fixture::TIME_SLICE` of entry k,
/// `fixture::CATEGORY` 2 (5 in the last three rounds), `fixture::ROUND_NUMBER` k. A row round (`is_mls_row_round`; the
/// t-th, t from 0) plays row i (0..3) of one division against row i of another (`MLS_ROW_ROUND_PAIRS`[t % 3]), home and
/// away swapped when t is odd. An ordinary round (the s-th, s from 0) plays six matches (`MLS_ORDINARY_ROUND_PAIRS`):
/// with m = s % 4 and the base row b = 0 (s < 4 or 8 ≤ s < 12) or 2, match j moves b on (b + 1 from j = 1; at j = 4,
/// the second division's last row + 1), all mod 4; the first table present takes row b, the second row b + m (between
/// the second and third divisions, from j = 4: b + 2 with m even, otherwise b + 1 at j = 5 and b + 3 at j = 4), mod 4;
/// home and away swapped when s is odd (`add_mls_cross_division_fixture`).
///
/// Finally the cross-division table freed and the league's table for stage −1 (the same call) into `round_dates()`,
/// and `stage_id()` = 0x413.
///
/// Simplification: the original's record keeps leftovers of its stack in the bytes it does not write (+4..+7, +0x18,
/// +0x2e, +0x2f, +0x39, +0x3c and the bits 11..15 of `fixture::SQUAD_RULES`, read and written back by the and/or); here
/// they are 0, a value the game produces (as in `create_fixtures`).
/// Original: `UsaMls::create_season_fixtures` (0x9086e0, usa_mls.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn usa_mls_create_season_fixtures(this: usize) -> u32 {
    let this = League(this);
    let schedule: unsafe extern "thiscall" fn(usize, i32, usize, usize, u32) -> usize = core::mem::transmute(vtable_method(this, comp_vtable::CREATE_SCHEDULE));
    let rounds_at = this.addr() + league::ROUND_COUNT;
    let stage_id_at = this.addr() + league::STAGE_ID;
    this.set_round_dates(schedule(this.addr(), -3, rounds_at, stage_id_at, 0));
    create_fixtures(this.addr());
    exe::crt_free(this.round_dates());
    create_second_division_stage(this.addr());
    create_third_division_stage(this.addr());
    let second = League(read_usize(this.stages()));
    let third = League(read_usize(this.stages() + 4));
    this.set_round_dates(schedule(this.addr(), -2, rounds_at, stage_id_at, 0));
    let mut reg = [0u8; 0x60];
    let r = Fixture(reg.as_mut_ptr() as usize);
    let c = this.competition().addr();
    r.set_squad_rules(((this.substitutes_named() & 0xf) as u16 | (((this.substitutions_allowed() & 0xf) as u16) << 4)) & 0xf8ff);
    write_usize(r.addr(), read_usize(c));
    r.set_competition(Competition(c));
    r.set_round_name(0);
    r.set_season_year(this.year() as u16);
    r.set_round_prefix(this.stage_id());
    r.set_category(this.match_category());
    r.set_round_draw_rules(this.round_draw_rules() as u16);
    r.set_tie_rules(this.match_rules() as u16);
    r.set_stage_index(this.stage_index());
    for k in 0x43..=0x4cusize {
        write_u8(r.addr() + k, 0xff);
    }
    r.set_match_stats_slot(usize::MAX);
    r.set_home_result_mark(0);
    r.set_away_result_mark(0);
    r.set_stage(0xff);
    r.set_home_side_mark(1);
    r.set_away_side_mark(1);
    let last = this.round_dates().wrapping_add((this.round_count() as isize * 0x41) as usize);
    let mut d = [0u8; 8];
    // the original pushes the words with leftovers in the high bytes; Date::Date only reads the words
    exe::new_date(d.as_mut_ptr() as usize, read_u16(last - 0x41) as u32, read_i16(last - 0x3f).wrapping_add(this.year()) as u16 as u32);
    r.set_next_round_day(read_u16(d.as_ptr() as usize));
    r.set_next_round_year(read_u16(d.as_ptr() as usize + 2));
    let tables = [this.table(), second.table(), third.table()];
    let (mut ordinary, mut row_rounds) = (0i16, 0i16);
    let mut second_last_row: u8 = 0; // esp+0x17: written (j = 1) before it is read (j = 4) in every ordinary round
    let mut k: i16 = 0;
    while k < this.round_count() {
        let e = this.round_dates() + k as usize * 0x41;
        r.set_year(read_i16(e + league_round_date::YEAR_OFFSET).wrapping_add(this.year()) as u16);
        r.set_day(read_u16(e));
        r.set_time_slice(read_u8(e + league_round_date::TIME_SLICE));
        r.set_category(if (k as i32) < this.round_count() as i32 - 3 { 2 } else { 5 });
        r.set_round_number(k);
        let venue = e + league_round_date::VENUE_VALUE;
        if is_mls_row_round(k) {
            covered!("cqy_2 mls row round");
            let pairs = &MLS_ROW_ROUND_PAIRS[(row_rounds % 3) as usize];
            let swap = row_rounds % 2 != 0;
            for (i, p) in pairs.iter().enumerate() {
                let rows = [if p[0] { i as u8 } else { 0xff }, if p[1] { i as u8 } else { 0xff }, if p[2] { i as u8 } else { 0xff }];
                add_mls_cross_division_fixture(this, r, tables, rows, swap, venue);
            }
            row_rounds += 1;
        } else {
            covered!("cqy_2 mls ordinary round");
            let m = (ordinary % 4) as i8;
            let mut b: i8 = if ordinary < 4 || (8..12).contains(&ordinary) { 0 } else { 2 };
            let swap = ordinary % 2 != 0;
            for (j, p) in MLS_ORDINARY_ROUND_PAIRS.iter().enumerate() {
                if j == 4 {
                    b = (second_last_row as i8).wrapping_add(1); // cqy2: the second against the third from its last row
                } else if j != 0 {
                    b = b.wrapping_add(1);
                }
                b %= 4;
                let mut rows = [0xffu8; 3];
                let mut next = b;
                if p[0] {
                    rows[0] = b as u8;
                    next = b.wrapping_add(m) % 4;
                }
                if p[1] {
                    rows[1] = next as u8;
                    let shift = if j > 3 {
                        if m % 2 == 0 { 2 } else if j != 4 { 1 } else { 3 }
                    } else {
                        m
                    };
                    next = b.wrapping_add(shift) % 4;
                    second_last_row = rows[1];
                }
                if p[2] {
                    rows[2] = next as u8;
                }
                add_mls_cross_division_fixture(this, r, tables, rows, swap, venue);
            }
            ordinary += 1;
        }
        k += 1;
    }
    exe::crt_free(this.round_dates());
    this.set_round_dates(schedule(this.addr(), -1, rounds_at, stage_id_at, 0));
    this.set_stage_id(0x413);
    1
}

/// Comparer for `usa_mls_create_season_fixtures`: only the original runs (it creates the stages and puts the matches
/// into the calendar: it cannot run twice); afterwards the plan: it returns 1, `stage_id()` = 0x413, and the round table
/// of stage −1 and the two division stages exist.
pub(crate) unsafe extern "thiscall" fn compare_usa_mls_create_season_fixtures(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_USA_MLS_CREATE_SEASON_FIXTURES)(this);
    let l = League(this);
    let ok = r == 1 && l.stage_id() == 0x413 && l.round_dates() != 0 && l.stages() != 0 && read_usize(l.stages()) != 0 && read_usize(l.stages() + 4) != 0;
    SCOREBOARD.record(ok, || format!("usa_mls_create_season_fixtures({:#x}): returned {}, stage id {:#x}, table {:#x}", this, r, l.stage_id(), l.round_dates()));
    r
}

/// MLS at career creation: the year and the database competition, discipline rule 0x1e (`discipline_rule::USA`) and
/// importance 10 (before loading the saved state) and, with no saved state, four stages besides the league (the
/// conferences and the knockout), the rules written inline (`MLS_INLINE_RULES`: 3 points for a win and 1 for a draw,
/// the tie-breakers and the match category), the table of the three divisions, the season fixtures
/// (UsaMls::create_season_fixtures instead of League::create_fixtures), the order, the statistics and the predicted
/// positions (`bra_nat_third_construct_league` with `USA_MLS_LEAGUE`).
/// Original: `UsaMls::UsaMls` (0x906f80, usa_mls.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_usa_mls(this: usize, year: u32, record: usize) -> usize {
    bra_nat_third_construct_league(League(this), year, record, &USA_MLS_LEAGUE, discipline_rule::USA, "cqu_2 usa_mls new") // cqu2: MLS
}

/// Comparer for `construct_usa_mls`.
pub(crate) unsafe extern "thiscall" fn compare_construct_usa_mls(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_USA_MLS);
    bra_nat_third_compare_league_ctor("construct_usa_mls", League(this), year, record, &USA_MLS_LEAGUE, discipline_rule::USA, || o(this, year, record))
}

/// Whether cross-division round k is a row round (rounds 2, 5, 8, 12, 15 and 18: each row against the same row of
/// another division).
pub(crate) fn is_mls_row_round(k: i16) -> bool {
    k == 2 || k == 5 || k == 8 || k == 12 || k == 15 || k == 18 // cqy2: the row rounds
}

/// Puts one cross-division match into the fixture record and the calendar.
///
/// `rows` are the row indices of the three tables (`[league, stage 0, stage 1]`, 0xff = that division does not play);
/// `tables` their table pointers (`league::TABLE`, rows of 0x3b bytes, +0 the club). The home side is the first present
/// club and the away side the next one; a null club pointer in the home place lets the next club take the home side,
/// as in the original. With `swap`, they change places. The ids (`fixture::HOME_CLUB_ID`, `fixture::AWAY_CLUB_ID`) are
/// the clubs' +0, or −1 with none. Then the stadium (Stadium::set_fixture_venue `exe::stadium_set_fixture_venue`: push
/// the league's `fixed_stadium_id()`; push &match; push the round's `league_round_date::VENUE_VALUE`; mov ecx,
/// [0xdd7ef4] -> thiscall, ret 0xc) and the calendar (FIXTURE_MANAGER::add_fixture `exe::fixture_manager_add_fixture`:
/// push 1; push &match; mov ecx, [0xae2a58] -> thiscall, ret 8).
pub(crate) unsafe fn add_mls_cross_division_fixture(this: League, r: Fixture, tables: [usize; 3], rows: [u8; 3], swap: bool, venue: usize) {
    let (mut home, mut away) = (0usize, 0usize);
    for d in 0..3 {
        if rows[d] == 0xff {
            continue;
        }
        let c = read_usize(tables[d] + (rows[d] as i8 as isize * 0x3b) as usize);
        if home != 0 {
            away = c;
        } else {
            home = c;
        }
    }
    if swap {
        core::mem::swap(&mut home, &mut away); // cqy2: home and away by the parity of the round
    }
    r.set_home_club(Club(home));
    r.set_away_club(Club(away));
    r.set_home_club_id(if home != 0 { read_usize(home) } else { usize::MAX });
    r.set_away_club_id(if away != 0 { read_usize(away) } else { usize::MAX });
    exe::stadium_set_fixture_venue(read_usize(G_STADIUM), read_usize(venue), r, this.fixed_stadium_id());
    exe::fixture_manager_add_fixture(read_usize(G_FIXTURE_MANAGER), r, 1);
}

/// The six matches of an ordinary cross-division round, in the original's order (the pointer array at esp+0xc0): two of
/// the first against the second division, two of the first against the third, two of the second against the third.
pub(crate) const MLS_ORDINARY_ROUND_PAIRS: [[bool; 3]; 6] = [MLS_PAIR_FIRST_SECOND, MLS_PAIR_FIRST_SECOND, MLS_PAIR_FIRST_THIRD,
    MLS_PAIR_FIRST_THIRD, MLS_PAIR_SECOND_THIRD, MLS_PAIR_SECOND_THIRD];

/// The four matches of a row round, by the count of row rounds so far mod 3 (the arrays at esp+0x90, +0xa0 and +0xb0):
/// row i of one division against row i of the other.
pub(crate) const MLS_ROW_ROUND_PAIRS: [[[bool; 3]; 4]; 3] = [
    [MLS_PAIR_FIRST_SECOND, MLS_PAIR_FIRST_SECOND, MLS_PAIR_SECOND_THIRD, MLS_PAIR_SECOND_THIRD],
    [MLS_PAIR_FIRST_THIRD, MLS_PAIR_FIRST_THIRD, MLS_PAIR_FIRST_SECOND, MLS_PAIR_FIRST_SECOND],
    [MLS_PAIR_SECOND_THIRD, MLS_PAIR_SECOND_THIRD, MLS_PAIR_FIRST_THIRD, MLS_PAIR_FIRST_THIRD],
];

pub(crate) const USA_MLS_LEAGUE: BraNatThirdLeagueVariant = BraNatThirdLeagueVariant {
    vtable: 0x970844,
    importance: Some((0xa, true)),
    stages: 4,
    writes: &MLS_INLINE_RULES,
    rules: None,
    table: FN_USAMLS_CREATE_TABLE,
    fixtures: FN_USAMLS_CREATE_SEASON_FIXTURES,
    predicted: 0x909bb0,
    serie_c: false,
    file: 0xaa1fb4,
    rows: &[0x59, 0x5f, 0x66, 0x6d],
};

/// The MLS division pairs of a cross-division match, as the original's three-character strings ("00_" 0xaa2094: the
/// league's own table and stage 0; "0_0" 0xaa2098: the league and stage 1; "_00" 0xaa209c: stages 0 and 1): which of
/// the three tables (`[league, stage 0, stage 1]`) take part.
pub(crate) const MLS_PAIR_FIRST_SECOND: [bool; 3] = [true, true, false];

pub(crate) const MLS_PAIR_FIRST_THIRD: [bool; 3] = [true, false, true];

pub(crate) const MLS_PAIR_SECOND_THIRD: [bool; 3] = [false, true, true];

/// The MLS rules, written inline in the original's order (offset, width, value).
pub(crate) const MLS_INLINE_RULES: [(usize, u8, u32); 18] = [
    (league::WIN_POINTS, 1, 3), // cqu2: the MLS points for a win
    (league::TIEBREAKS + 2, 1, 3),
    (league::DRAW_POINTS, 1, 1),
    (league::MATCH_RULES, 2, 0xc2),
    (league::MATCH_CATEGORY, 1, 2),
    (league::CLUB_SIDES, 1, 1),
    (league::TIEBREAKS, 1, 1),
    (league::TIEBREAKS + 1, 1, 2),
    (league::TIEBREAKS + 3, 1, 0),
    (league::PROMOTED_COUNT, 1, 1),
    (league::PROMOTION_PLAYOFF_COUNT, 1, 0),
    (league::RELEGATION_PLAYOFF_COUNT, 1, 0),
    (league::RELEGATED_COUNT, 1, 0),
    (league::UPPER_DIVISION_ID, 4, 0xffff_ffff),
    (league::LOWER_DIVISION_ID, 4, 0xffff_ffff),
    (league::LEAGUE_FLAGS, 2, 0x48),
    (league::LEGS, 2, 4),
    (league::GAMES_PER_CLUB, 2, 0x20),
];
