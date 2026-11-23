use super::*;

/// Competition record id in the database (G_CLUB_COMP_TABLE + 0x6b × id).
pub(crate) unsafe fn club_comp_record(id: usize) -> usize {
    read_usize(G_CLUB_COMP_TABLE).wrapping_add(id.wrapping_mul(0x6b))
}

/// The four lists of `con_champ_select_qualified_clubs` (Mexican, Central American, Caribbean and MLS clubs) with their counts.
///
/// Built in the order of the club database ([0xae23b4], 0x245 bytes, [0xae235c] clubs; it stops at the first one that
/// is a national team, `exe::database_is_national_team`).
pub(crate) struct ConcacafClubLists {
    pub(crate) mex: [usize; 200],
    pub(crate) centro: [usize; 200],
    pub(crate) caribe: [usize; 200],
    pub(crate) mls: [usize; 12],
    pub(crate) n: [u32; 4],
}

/// The four club lists of `con_champ_select_qualified_clubs` in database order (Mexican, Central American, Caribbean, MLS) with their counts,
/// stopping at the first national team.
pub(crate) unsafe fn concacaf_club_lists() -> ConcacafClubLists {
    // simplification: the four lists start zeroed in the copy (in the original, stack leftover that is only read when a club is missing;
    // the comparer sends this case to the original alone)
    let mut l = ConcacafClubLists { mex: [0; 200], centro: [0; 200], caribe: [0; 200], mls: [0; 12], n: [0; 4] };
    let (mut a, mut b, mut c, mut m) = (0u8, 0u8, 0u8, 0i8);
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i32;
    while i < read_usize(G_CLUB_COUNT) as i32 {
        if exe::database_is_national_team(club) != 0 {
            break;
        }
        let nation = Nation(read_usize(club + club::NATION));
        if !nation.is_null() {
            let id = read_usize(nation.addr());
            if id == read_usize(KEY_NATION_MEXICO) {
                if a < 0xc8 {
                    l.mex[a as usize] = club;
                    a += 1;
                }
            } else {
                let div = read_usize(club + club::DIVISION);
                if div != 0 && read_usize(div) == read_usize(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE) {
                    if m < 0xc {
                        l.mls[m as usize] = club;
                        m += 1;
                    }
                } else if id != read_usize(KEY_NATION_UNITED_STATES) && nation.region() == 7 {
                    if b < 0xc8 {
                        l.centro[b as usize] = club;
                        b += 1;
                    }
                } else if nation.region() == 6 && c < 0xc8 {
                    l.caribe[c as usize] = club;
                    c += 1;
                }
            }
        }
        i += 1;
        club = club.wrapping_add(club::SIZE);
    }
    l.n = [a as u32, b as u32, c as u32, m as u8 as u32];
    l
}

/// rule: eight clubs. Two from Mexico (drawn among the 6 with the highest reputation), three from Central America
/// (nation region 7, other than nation [`KEY_NATION_UNITED_STATES`]; among the top 9), one from the Caribbean (region
/// 6; among the top 3) and two from the United States: with competition [`KEY_CLUB_COMP_US_OPEN_CUP`] running
/// [inference: the US Open Cup], the latest MLS champion and its champion (if the same club, the place goes to the MLS
/// runner-up); without it, two MLS clubs among the top 3 by reputation. Up to 200 clubs per region and 12 from the MLS
/// are considered
/// Original: `ConChamp::select_qualified_clubs` (0x4c11a0, con_champ.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn con_champ_select_qualified_clubs(this: usize) -> u32 {
    let this = Cup(this);
    this.set_team_count(8);
    let other = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_US_OPEN_CUP).wrapping_mul(4)));
    let mut l = concacaf_club_lists();
    exe::comp_util_sort_and_shuffle_top_clubs(l.mex.as_mut_ptr() as usize, l.n[0], 2);
    exe::comp_util_sort_and_shuffle_top_clubs(l.centro.as_mut_ptr() as usize, l.n[1], 3); // cqi11: três centro-americanos
    exe::comp_util_sort_and_shuffle_top_clubs(l.caribe.as_mut_ptr() as usize, l.n[2], 1);
    exe::comp_util_sort_and_shuffle_top_clubs(l.mls.as_mut_ptr() as usize, l.n[3], 1);
    let t = |k: usize| this.team_table().wrapping_add(k * 6);
    let pd = |a: usize, v: usize| write_u32(a, v as u32);
    covered!("cqi_11 qualified");
    pd(t(2), l.mex[0]);
    pd(t(3), l.mex[1]); // cqi11: the Mexican runner-up
    pd(t(4), l.centro[0]);
    pd(t(5), l.centro[1]);
    pd(t(6), l.centro[2]);
    pd(t(7), l.caribe[0]);
    if other == 0 {
        pd(t(0), l.mls[0]);
        pd(t(1), l.mls[1]);
    } else {
        covered!("cqi_11 mls champion");
        let mls = club_comp_record(read_usize(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE));
        pd(t(0), exe::comp_util_get_latest_placed_club(mls, 0));
        pd(t(1), exe::comp_util_get_latest_placed_club(read_usize(other + 4), 0));
        if read_usize(t(0)) == read_usize(t(1)) { // cqi11: the champion of both gives the slot to the MLS runner-up
            pd(t(0), exe::comp_util_get_latest_placed_club(mls, 1));
        }
    }
    let mut i = 0i32;
    while i < this.team_count() as i32 {
        write_u8(this.team_table() + i as usize * 6 + 4, 0);
        write_u8(this.team_table() + i as usize * 6 + 5, 0);
        i += 1;
    }
    1
}

/// Daily update of the CONCACAF Champions' Cup. Rule: on the first day of the cup season (empty team table), picks the
/// qualified clubs and the predicted positions; every day, the cup update (draws and matches).
///
/// Ret 8; virtual +0x14 of the cup: `ConChamp::update_for_date(today, stage)`. With the team table empty (the club of
/// team 0 of `.team_table()` = 0) and stage ≠ 0: the qualified clubs (`exe::conchamp_select_qualified_clubs`: mov ecx,
/// this -> thiscall, ret 0; negative: the box at line 0x1ba and returns 0) and the predicted positions
/// (`comp_vtable::SET_PREDICTED_POSITIONS`). Then the cup update (`exe::cup_update_for_date`: push stage; push today;
/// mov ecx, this -> thiscall, ret 8), whose return value is returned.
/// Original: `ConChamp::update_for_date` (0x4c1620, con_champ.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn con_champ_update_for_date(this: usize, today: usize, stage: u32) -> u32 {
    let this = Cup(this);
    if read_usize(this.team_table()) == 0 && stage != 0 {
        if exe::conchamp_select_qualified_clubs(this) < 0 {
            // simplification: 0x4c11a0 always returns 1; the error case stays with the box and the original’s 0
            show_error_at(0x9a7d30, 0x1ba);
            return 0;
        }
        covered!("cqi_11 national team day");
        let prever: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::SET_PREDICTED_POSITIONS));
        prever(this.addr());
    }
    exe::cup_update_for_date(this, today, stage)
}

// simplification: the cup update (0x51f890) plays and draws: does not run twice. Only the original runs; the plan: with the
// empty table and stage ≠ 0, the cup comes out with 8 teams and team 0 filled (0x4c11a0 has its own comparer); otherwise +0x3e
// does not change.

/// Sets up the rounds of the CONCACAF Champions' Cup. Rule: the cup prize fund is 2,000,000; the round table
/// (three-round knockout) comes from `comp_vtable::CREATE_SCHEDULE`.
///
/// Ret 0; virtual +0x8c: `ConChamp::set_up_rounds()`; returns 1. The prize fund (`.set_prize_fund()`) of 2,000,000 and
/// the round table (`.set_round_table()`) from `comp_vtable::CREATE_SCHEDULE` (push 0; push &`cup::STAGE_LABEL`; push
/// &`cup::ROUND_COUNT`; push −1; mov ecx, this -> thiscall, ret 0x10).
/// Original: `ConChamp::set_up_rounds` (0x4c15f0, con_champ.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn con_champ_set_up_rounds(this: usize) -> u32 {
    let this = Cup(this);
    this.set_prize_fund((0x1e8480) as i32);
    let table: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let t = table(this.addr(), 0xffff_ffff, this.addr() + cup::ROUND_COUNT, this.addr() + cup::STAGE_LABEL, 0);
    this.set_round_table((t as u32) as usize);
    1
}

// simplification: virtual +0x3c allocates the rounds table: does not run twice. Original only; the plan checks the bottom of
// prizes (+0x28 = 2,000,000) and return value 1.

/// Creates the round table of the CONCACAF Champions' Cup. Rule: 8 teams in single matches: quarter-finals drawn on 1/8
/// and played on 28/8, semi-final on 1/9 and final on 4/9; category 8, tie-break 3; no per-round prize (−1).
///
/// Ret 0x10; virtual +0x3c: `ConChamp::create_round_table(stage, &rounds, &extra, &out)`: at stage −1, out (if
/// requested) = 0, rounds = 3, extra = 0; the table (`exe::crt_malloc` of 0x138; out of memory: the box at line 0x97
/// and 0) with the 3 rounds and, in the first two, day 0 of the empty tie (`exe::cup_set_round_match_day` with −1, −1,
/// −1 and home 0; the original calls it between the match and the fields, which do not overlap). Outside −1, 0.
/// Original: `ConChamp::create_round_table` (0x4c0f00, con_champ.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn con_champ_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let this = Cup(this);
    if stage as u8 != 0xff {
        return 0;
    }
    if !out.is_null() {
        *out = 0;
    }
    *rounds = 3;
    *extra = 0;
    let t = exe::crt_malloc(0x138);
    if t == 0 {
        show_error_at(0x9a7d30, 0x97);
        return 0;
    }
    covered!("cqp_13 concacaf fixtures");
    let year = this.year() as i32 as u32;
    for (i, r) in CONCACAF_CHAMPIONS_CUP_ROUNDS.iter().enumerate() {
        eng_cc_cup_write_cup_round(t, i as u32, r, year);
        if i < 2 {
            exe::cup_set_round_match_day(t, i as i32, 0, -1, -1, -1, 0);
        }
    }
    t
}

/// Rule: the CONCACAF Champions' Cup turnover: the finished cup is cleared, its statistics archived and the year moves
/// on; an empty team table is made for the entrants (chosen later) and the rules and the tie tree are made again.
///
/// `ConChamp::start_new_season()` (virtual +8): the first half (`cup_turnover_free_season`); the team table
/// (`exe::crt_calloc`(`cup::TEAM_COUNT`, 6): push 6; push n; add esp, 8 -> cdecl) in `cup::TEAM_TABLE`; without it, the
/// error box at line 0x184 and return; the rules (virtual `comp_vtable::SET_UP_RULES`; negative: the box at line
/// 0x18b) and the tie tree (virtual `cup_vtable::MAP_FIXTURE_TREE`; negative: the box at line 0x192).
/// Original: `ConChamp::start_new_season` (0x4c1440, con_champ.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn con_champ_start_new_season(this: usize) {
    let cup = Cup(this);
    covered!("cqx_9 concacaf champions cup turnover");
    cup_turnover_free_season(cup);
    let t = exe::crt_calloc(cup.team_count() as i32 as u32, 6);
    cup.set_team_table(t);
    if t == 0 {
        return show_error_at(0x9a7d30, 0x184);
    }
    if cup_turnover_virtual(cup, comp_vtable::SET_UP_RULES) < 0 {
        return show_error_at(0x9a7d30, 0x18b);
    }
    if cup_turnover_virtual(cup, cup_vtable::MAP_FIXTURE_TREE) < 0 { // cqx9: the CONCACAF tie tree
        show_error_at(0x9a7d30, 0x192);
    }
}

pub(crate) unsafe extern "thiscall" fn compare_con_champ_start_new_season(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CON_CHAMP_START_NEW_SEASON);
    let year = Cup(this).year();
    o(this);
    SCOREBOARD.record(Cup(this).year() == year.wrapping_add(1) && Cup(this).team_table() != 0, || format!("con_champ_start_new_season({:#x}): year {} -> {}, team table {:#x}", this, year, Cup(this).year(), Cup(this).team_table()));
}

/// The CONCACAF Champions Cup of the game's year, at career creation: 8 clubs that come in later (the table starts
/// empty) and a 2,000,000 prize fund.
///
/// Cup::Cup (`exe::new_cup`, mov ecx -> thiscall, ret 0); the competition record at +4, the vtable 0x968aa0, the year
/// (`set_year`), the common fields (`intercontinental_cup_common_fields`), `club_sides` 2, `discipline_index` 0xff and
/// `run_by_stages` 0. If the saved state loads (`intercontinental_cup_loads`), returns this. Otherwise 8 teams
/// (`team_count`) in a zeroed team table of 6 bytes each (calloc `exe::crt_calloc(8, 6)`: push 6; push 8; add esp, 8 ->
/// cdecl; out of memory, the error box of line 0x41, source 0x9a7d30, and this), the prize fund 2,000,000 and the common
/// end (`finish_intercontinental_cup`). Returns this. The original's SEH frame only matters for exceptions.
/// Original: `ConChamp::ConChamp` (0x4c0c40, con_champ.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_con_champ(this: usize, year: u32, record: usize) -> usize {
    let this = Cup(this);
    exe::new_cup(this);
    covered!("cqw_12 concacaf champions cup");
    write_u32(this.addr() + 4, record as u32);
    write_u32(this.addr(), 0x968aa0);
    this.set_year(year as i16); // cqw12: the CONCACAF cup of the game's year
    intercontinental_cup_common_fields(this);
    this.set_club_sides(2);
    this.set_discipline_index(0xff);
    this.set_run_by_stages(0);
    if intercontinental_cup_loads(this) {
        return this.addr();
    }
    this.set_team_count(8);
    let t = exe::crt_calloc(8, 6);
    write_u32(this.addr() + cup::TEAM_TABLE, t as u32);
    if t == 0 {
        show_error_at(0x9a7d30, 0x41);
        return this.addr();
    }
    this.set_prize_fund(0x1e8480);
    finish_intercontinental_cup(this, 0x9a7d30);
    this.addr()
}

/// Comparer for `construct_con_champ`.
pub(crate) unsafe extern "thiscall" fn compare_construct_con_champ(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_CON_CHAMP);
    compare_intercontinental_cup_ctor("construct_con_champ", this, year, record, 0x968aa0, || o(this, year, record))
}

/// The fields every inter-continental knockout cup sets: `current_round` 0, `stage_index` 0xff (the main
/// competition), `stage_count` 0 and `current_stage` −1.
pub(crate) unsafe fn intercontinental_cup_common_fields(this: Cup) {
    this.set_current_round(0);
    this.set_stage_index(stage_index::MAIN);
    write_u32(this.addr() + cup::STAGE_COUNT, 0);
    write_u32(this.addr() + cup::CURRENT_STAGE, 0xffff_ffff);
}

/// The common end of a new inter-continental cup; returns whether it got to the end with the statistics.
///
/// The round table (virtual `comp_vtable::CREATE_SCHEDULE`(−1, &`cup::ROUND_COUNT`, &`cup::STAGE_LABEL`, 0): push 0;
/// push; push; push −1; mov ecx -> thiscall, ret 0x10) in `round_table()`; the tie tree (Cup::map_fixture_tree
/// 0x518790, thiscall, ret 0; negative: the error box of line 0x4f of source `file` and false); the statistics
/// (operator new `exe::crt_operator_new` of 0x5ce and Comp_Stats::Comp_Stats `exe::new_comp_stats(new, this)`: push
/// this; mov ecx -> thiscall, ret 4) at +8 (null: the box of line 0x56 and false).
pub(crate) unsafe fn finish_intercontinental_cup(this: Cup, file: usize) -> bool {
    let create_schedule: unsafe extern "thiscall" fn(usize, i32, usize, usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    write_u32(this.addr() + cup::ROUND_TABLE, create_schedule(this.addr(), -1, this.addr() + cup::ROUND_COUNT, this.addr() + cup::STAGE_LABEL, 0) as u32);
    let map_fixture_tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_CUP_MAP_FIXTURE_TREE); // Cup::map_fixture_tree
    if map_fixture_tree(this.addr()) < 0 {
        show_error_at(file, 0x4f);
        return false;
    }
    let p = exe::crt_operator_new(0x5ce);
    let e = if p != 0 { exe::new_comp_stats(p, this) } else { 0 };
    write_u32(this.addr() + 8, e as u32);
    if e == 0 {
        show_error_at(file, 0x56);
        return false;
    }
    true
}

/// Whether the saved state of the cup loads (Cup::load_from_disk 0x51fc00(this, 1): push 1; mov ecx -> thiscall, ret 4;
/// nonzero).
pub(crate) unsafe fn intercontinental_cup_loads(this: Cup) -> bool {
    let load_from_disk: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_CUP_LOAD_FROM_DISK); // Cup::load_from_disk
    load_from_disk(this.addr(), 1) != 0
}

/// Shared comparer of the inter-continental cup constructors. They allocate the table, the tree and the statistics,
/// so they cannot run twice: only the original runs, then the plan is checked: it returns this, the vtable, the record
/// at +4 and the year (`cup::YEAR`).
pub(crate) unsafe fn compare_intercontinental_cup_ctor(name: &str, this: usize, year: u32, record: usize, vtable: usize, o: impl FnOnce() -> usize) -> usize {
    let r = o();
    let read = Cup(this).year();
    let ok = r == this && read_usize(this) == vtable && read_usize(this + 4) == record && built_year_ok(read as u16, year as u16);
    SCOREBOARD.record(ok, || format!("{}({:#x}): year {} requested {}, vtable {:#x}", name, this, read, year as i16, read_usize(this)));
    r
}
