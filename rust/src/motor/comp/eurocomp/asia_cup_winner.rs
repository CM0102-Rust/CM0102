use super::*;

/// The group each African Cup quarter-final slot comes from (−1 = group A, the cup's own table; 0..2 = the stages of
/// groups B to D) and the position: the keys are the pairs of the list.
pub(crate) const AFRICAN_NATIONS_QUARTER_FINAL_SLOTS: [(i32, i32); 8] = [
    (-1, 0), (0, 1), // cqt13: 1º A × 2º B
    (1, 0), (2, 1), // cqt13: 1º C × 2º D
    (1, 1), (2, 0), // cqt13: 2º C × 1º D
    (0, 0), (-1, 1), // cqt13: 1º B × 2º A
];

/// Club at position p of group g (or the cup itself when g < 0) via League::get_table_row.
pub(crate) unsafe fn group_table_club(this: League, g: i32, p: i32) -> usize {
    let group = if g < 0 { this.addr() } else { read_usize(this.stages() + g as usize * 4) };
    read_usize(exe::league_get_table_row(group, p))
}

/// `AfricanNations::create_knockout_stage()` (`0x402090`, thiscall, ret 0, `african_nations.cpp`): builds the
/// quarter-finals of the African Nations Cup.
///
/// SEH frame only for the stage's `new`, with no destructor on the normal path; called by the next-stage routine
/// `0x401e10` on reaching stage 3.
///
/// The list of the 8 (`0x944e46(0x20)`: push; add esp, 4 -> cdecl; out of memory, the box of line `0x311` and return)
/// by the pairings of AFRICAN_NATIONS_QUARTER_FINAL_SLOTS, in call order; the round table via virtual `+0x3c` (push 0;
/// push &id; push &rodadas; push 3; mov ecx, this -> thiscall, ret 0x10); the stage (`0x944cf1(0xb2)` push; add esp, 4
/// -> cdecl; Cup_Stage::Cup_Stage `0x522e00`, thiscall, ret 0x3c: the cup, 8, the list, the rounds, `+4`, the table,
/// the year `+0x40`, 3, 4, the id, 0x14, 0, 0, 0 and no seeds; the original routine pushes the rounds, the id and the
/// year with garbage in the high 16 bits, which the constructor does not read) into [`+0xc`][3]; the list and the table
/// are freed (`0x9452ca`).
///
/// Without the stage: the box of line `0x32c` and return. With it: the round is set up (`0x51c800` Cup::set_up_round:
/// push 0; mov ecx, stage -> thiscall, ret 4) and `+0x30` = 3.
pub(crate) unsafe extern "thiscall" fn asia_cup_winner_create_knockout_stage(this: usize) {
    let this = League(this);
    let list = exe::crt_malloc(0x20);
    if list == 0 {
        return show_error_at(0x9870f8, 0x311);
    }
    covered!("cqt_13 african cup quarterfinals");
    for (k, &(g, p)) in AFRICAN_NATIONS_QUARTER_FINAL_SLOTS.iter().enumerate() {
        write_usize(list + k * 4, group_table_club(this, g, p));
    }
    let rounds_of_cup: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rodadas, mut id) = (0i16, 0i16);
    let table = rounds_of_cup(this.addr(), 3, &mut rodadas, &mut id, 0);
    let p = exe::crt_operator_new(0xb2);
    let stage = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this.addr(), 8, list, rodadas as u16 as u32, this.competition().addr(), table, this.year() as u16 as u32, 3, 4, id as u16 as u32, 0x14, 0, 0, 0, core::ptr::null())
    } else {
        0
    };
    write_usize(this.stages() + 0xc, stage);
    exe::crt_free(list);
    exe::crt_free(table);
    let f = read_usize(this.stages() + 0xc);
    if f == 0 {
        return show_error_at(0x9870f8, 0x32c);
    }
    exe::cup_set_up_round(f, 0);
    this.set_current_stage(3);
}

// simplification: allocates the stage and sets up the round: does not run twice. Original only; the plan: the stage at [+0xc][3] with 8 teams,
// the cup’s +4 and the 8 cross-paired ones (read earlier through the tables, +0xb1 of each group) in its list (+0xa7, 6 bytes; as
// conjunto) e +0x30 = 3.

/// AsiaCupWinner::advance_stage(): moves the Asian Cup Winners' Cup to its next stage (+0x30); on reaching stage 1
/// (after both zones' knockouts) it creates the finals. Shows the original error box if already at the last stage.
pub(crate) unsafe extern "thiscall" fn asia_cup_winner_advance_stage(this: usize) {
    let this = Cup(this);
    let f = this.current_stage() as i32;
    if f >= (this.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(0x987b60, 0x328);
    }
    let f = f + 1;
    this.set_current_stage(f as usize);
    if f == 1 {
        covered!("cqt_13 asian cup winners cup finals");
        exe::asiacupwinner_create_finals_stage(this);
    }
}

// simplification: the finals allocate and draw: they do not run twice. Original only; the plan: +0x30 goes up one (outside the last
// stage, which shows the box and changes nothing).

/// The 4 finalists of the Asian Cup Winners' Cup: the surviving entrants of the two zones' knockouts, with the 2nd and
/// 3rd swapped.
///
/// The survivors (+5 = 1 in the 6-byte table) of the first zone's knockout (the cup's table, +0xa7) and then those of
/// the second (table +0xa7 of stage [+0xc][0]), up to 4 in total, both searches going up to the cup's +0x3e (signed
/// word). The 2nd and 3rd swap places (the pairings: the brackets are the pairs of the list).
///
/// Slots left over stay 0 (the original routine leaves them with whatever malloc returned).
pub(crate) unsafe fn asian_cup_winners_finalists(this: Cup) -> [usize; 4] {
    let mut v = [0usize; 4];
    let mut n = 0i16;
    let teams = this.team_count();
    for tabela in [this.team_table(), read_usize(read_usize(this.stages()) + cup::TEAM_TABLE)] {
        let mut i = 0i16;
        while i < teams && n < 4 {
            let e = tabela + i as usize * 6;
            if read_u8(e + 5) == 1 {
                v[n as usize] = read_usize(e);
                n += 1;
            }
            i += 1;
        }
    }
    v.swap(1, 2); // cqt13: crosses the middle pair
    v
}

/// `AsiaCupWinner::create_finals_stage()` (`exe::asiacupwinner_create_finals_stage`, thiscall, ret 0, asia_cup_winner.cpp): builds the 4-team finals
/// stage of the Asian Cup Winners' Cup from its finalists, gets the round table from the cup's virtual +0x3c,
/// constructs the Cup_Stage in stage slot 1 (cup +0xc) and draws its round 0.
///
/// The SEH frame is only for the stage's new: no destructor on the normal path.
///
/// The list of 4 (`exe::crt_malloc`(0x10): push; add esp, 4 -> cdecl; out of memory, the error box for line 0x34b and return)
/// from asian_cup_winners_finalists; the round table via virtual +0x3c (push 0; push &id; push &rodadas; push 1; mov
/// ecx, this -> thiscall, ret 0x10); the stage (`exe::crt_operator_new`(0xb2) push; add esp, 4 -> cdecl; Cup_Stage::Cup_Stage
/// `construct_cup_stage`, thiscall, ret 0x3c: the cup, 4, the list, the rounds, +4, the table, the year +0x40, 1, 2, the id, 0x14,
/// 0, 0, 0 and no seed; the original pushes the rounds, the id and the year with garbage in the high 16 bits, which the
/// constructor does not read) stored in [+0xc][1]; the table is freed (`exe::crt_free`).
///
/// No stage: the error box for line 0x375 and return (the list is not freed, as in the original). With it: the draw of
/// round 0 (`draw_round` Cup::draw_round: push 0; mov ecx, stage -> thiscall, ret 4) and the list is freed.
pub(crate) unsafe extern "thiscall" fn create_finals_stage(this: usize) {
    let this = Cup(this);
    let list = exe::crt_malloc(0x10);
    if list == 0 {
        return show_error_at(0x987b60, 0x34b);
    }
    covered!("cqt_13 asian cup winners cup finalists");
    // simplification: with fewer than 4 alive the original leaves in the slots what malloc gave; the copy writes 0 (in this career there are 4)
    for (k, &c) in asian_cup_winners_finalists(this).iter().enumerate() {
        write_usize(list + k * 4, c);
    }
    let rounds_of_cup: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rodadas, mut id) = (0i16, 0i16);
    let table = rounds_of_cup(this.addr(), 1, &mut rodadas, &mut id, 0);
    let p = exe::crt_operator_new(0xb2);
    let stage = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this.addr(), 4, list, rodadas as u16 as u32, read_usize(this.addr() + 4), table, this.year() as u16 as u32, 1, 2, id as u16 as u32, 0x14, 0, 0, 0, core::ptr::null())
    } else {
        0
    };
    write_usize(this.stages() + 4, stage);
    exe::crt_free(table);
    let f = read_usize(this.stages() + 4);
    if f == 0 {
        return show_error_at(0x987b60, 0x375);
    }
    exe::cup_draw_round(f, 0);
    exe::crt_free(list);
}

// simplification: allocates the stage and draws the round: it does not run twice. Original only; the plan: the stage in [+0xc][1] with 4
// teams, the cup’s +4 and the 4 finalists (read earlier) in its list (+0xa7, 6 bytes; as a set).

/// rule: the Asian Cup Winners' Cup has, in each zone (stages −1 and 0, ids 0x480 and 0x481), three two-legged
/// qualifying rounds: 1st on 23/8, 2nd on 20/9, quarter-finals on 24 March of the following year (21 days between legs
/// in the 1st, 7 after); the final stage (stage 1) in May: semi-final on 24/5, third place and final on 26/5, single
/// matches; category 4 in the qualifiers and 6 after
/// Original: `AsiaCupWinner::create_round_table` (0x40cf90, asia_cup_winner.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn asia_cup_winner_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_round_table(Cup(this), asian_cup_winners_stage(stage), rounds, extra, league, 0x987b60)
}

/// rule: the Asian Cup Winners' Cup has 16 teams, the qualified clubs (`asia_club_champ_fields::QUALIFIED_CLUBS`) in
/// the order they were picked, all with class and status 0
/// Original: `AsiaCupWinner::create_team_entries` (0x40d4d0, asia_cup_winner.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn create_team_entries(this: usize) -> i32 {
    build_team_table(Cup(this), 0x987b60, 0x185, "cqq_12 times da recopa asiatica", |i| i) // cqq12: the Asian Cup Winners’ Cup teams
}

/// Rule: the Asian Cup Winners' Cup records: in the zone knockouts (stage 0), elimination and qualification weigh by
/// round (1 to 80 over the 3 rounds) and the zone winner gets the record of round 0x82 (weight 0x5a); in the final
/// (stage 1) the winner, the runner-up and the third get the final records.
///
/// `AsiaCupWinner::update_staff_records_for_status(club, outcome, stage, _, fixture, final)` (the virtual +0x44).
/// Stage (signed byte) below 1, weight = `scale_by_round`(1, 0x50, the fixture's `round_number()`, 3): outcome 0xff or
/// 2, the elimination (`european_cup_add_knocked_out_record`: `round_prefix()`, `round_name()`, the weight); 0, the
/// qualification (`european_cup_add_qualified_record`: `round_prefix()`, the name word +0x6f of the fixture's round in
/// the cup's round table `cup::ROUND_TABLE`, the weight); 1, the qualification (0, 0x82, 0x5a); returns 0. Stage 1:
/// outcome 1 the winner (Staff_Records 0x86a800), 2 the runner-up (0x86b0b0), 3 the third (0x86b710), returns 0;
/// otherwise −1. Above 1: 0. (The high 16 bits of the words the original pushes come from small products of the round:
/// zero.)
/// Original: `AsiaCupWinner::update_staff_records_for_status` (0x40e260, asia_cup_winner.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn asia_cup_winner_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, _a4: u32, game: usize, final_: u32) -> u32 {
    let g = Fixture(game);
    let (d, f) = (outcome as u8, stage as u8 as i8);
    let comp = read_usize(this + 4);
    if f < 1 {
        match d {
            0xff | 2 => {
                let weight = scale_by_round(1, 0x50, g.round_number(), 3);
                european_cup_add_knocked_out_record(club, comp, g.round_prefix() as u16 as u32, g.round_name() as u16 as u32, weight);
            }
            0 => {
                covered!("cqx_10 cup winners cup zone qualified");
                let weight = scale_by_round(1, 0x50, g.round_number(), 3);
                let name = read_u16(Cup(this).round_table().wrapping_add((g.round_number() as isize * cup_round::SIZE as isize) as usize) + 0x6f) as u32;
                european_cup_add_qualified_record(club, comp, g.round_prefix() as u16 as u32, name, weight);
            }
            1 => european_cup_add_qualified_record(club, comp, 0, 0x82, 0x5a),
            _ => {}
        }
        0
    } else if f == 1 {
        match d {
            1 => cup_record_final_result(0x86a800, club, g, final_),
            2 => cup_record_final_result(0x86b0b0, club, g, final_),
            3 => cup_record_final_result(0x86b710, club, g, final_),
            _ => return u32::MAX,
        }
        0
    } else {
        0
    }
}

pub(crate) unsafe extern "thiscall" fn compare_asia_cup_winner_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, a4: u32, game: usize, final_: u32) -> u32 {
    let (d, f) = (outcome as u8, stage as u8 as i8);
    let plan = if f == 1 && !(1..=3).contains(&d) { u32::MAX } else { 0 };
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_ASIA_CUP_WINNER_UPDATE_STAFF_RECORDS_FOR_STATUS)(this, club, outcome, stage, a4, game, final_);
    SCOREBOARD.record(r == plan, || format!("asia_cup_winner_update_staff_records_for_status({:#x}, {}, {}) returned {:#x}, plan {:#x}", club, d, f, r, plan));
    r
}

/// The Asian Cup Winners' Cup of the game's year, at career creation: 2 stages and a 4,000,000 prize fund, built like
/// the Asian Club Championship (`build_asian_club_cup` with `ASIAN_CUP_WINNERS_CUP_PARTS`). The original's SEH frame
/// only matters for exceptions.
/// Original: `AsiaCupWinner::AsiaCupWinner` (0x40cbe0, asia_cup_winner.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_asia_cup_winner(this: usize, year: u32, record: usize) -> usize {
    covered!("cqw_13 asian cup winners cup");
    let year = year as i16; // cqw13: the Asian Cup Winners' Cup of the game's year
    build_asian_club_cup(Cup(this), year, record, &ASIAN_CUP_WINNERS_CUP_PARTS)
}

/// Comparer for `construct_asia_cup_winner`.
pub(crate) unsafe extern "thiscall" fn compare_construct_asia_cup_winner(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_ASIA_CUP_WINNER);
    compare_asian_club_cup_ctor("construct_asia_cup_winner", this, year, record, 0x967574, || o(this, year, record))
}

/// The 32 Asian Cup Winners' Cup qualifiers: +0xb6 = 32 and the 32-place list +0xb2 (`exe::crt_calloc`(0x20, 4), cdecl;
/// out of memory, the error box of line 0x241 and 0). The latest winner of the Cup Winners' Cup itself
/// (`exe::comp_util_get_latest_placed_club`([+4], 0), cdecl), if it does not play the Champions League, goes to place
/// 0 if its nation is in the east zone, otherwise to 16. In a local list of up to 1,200 clubs: the Emperor's Cup winner
/// ([`KEY_CLUB_COMP_JAPANESE_EMPEROR_S_CUP`]; the runner-up if the winner is already in places 0/16 or plays the
/// Champions League), if Japan has its `nation::LEAGUE_SELECTED` byte, and the Korean cup's
/// ([`KEY_CLUB_COMP_KOREAN_FA_CUP`]) in the same way; and every Asian club ([`KEY_CONTINENT_ASIA`]) outside places 0
/// and 16. The list is sorted and shuffled (`exe::comp_util_sort_and_shuffle_top_clubs`(list, n − 1, 40), cdecl). In
/// its order, each club whose nation has not had a place yet and that does not play the Champions League takes the
/// first free place of its zone (0..15 east, 16..31 west), while the zone's last place is free; the nation is marked
/// even when the zone is full. Returns 1. (Stack frame 0x1598.)
/// Original: `AsiaCupWinner::select_qualified_clubs` (0x40d830, asia_cup_winner.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn asia_cup_winner_select_qualified_clubs(this: usize) -> u32 {
    write_u32(this + 0xb6, 0x20);
    let east: [u32; 23] = core::array::from_fn(|i| read_usize(ASIAN_CUP_WINNERS_EAST_ZONE_KEYS[i]) as u32);
    let west: [u32; 19] = core::array::from_fn(|i| read_usize(ASIAN_CUP_WINNERS_WEST_ZONE_KEYS[i]) as u32);
    let list = exe::crt_calloc(0x20, 4);
    write_usize(this + 0xb2, list);
    if list == 0 {
        show_error_at(0x987b60, 0x241);
        return 0;
    }
    let holder = exe::comp_util_get_latest_placed_club(read_usize(this + 4), 0);
    if !asia_cup_winner_in_champions_league(holder) {
        if asia_cup_winner_zone_index(holder, &east) > -1 {
            write_usize(list, holder);
        } else {
            write_usize(list + 0x40, holder);
        }
    }
    let mut clubs = [0usize; 0x4b0];
    let mut n = 0usize;
    if let Some(c) = asia_cup_winner_national_cup_club(this, KEY_NATION_JAPAN, KEY_CLUB_COMP_JAPANESE_EMPEROR_S_CUP) {
        covered!("cqu_11 emperors cup club");
        clubs[0] = c;
        n = 1;
    }
    if let Some(c) = asia_cup_winner_national_cup_club(this, KEY_NATION_SOUTH_KOREA, KEY_CLUB_COMP_KOREAN_FA_CUP) {
        clubs[n] = c;
        n += 1;
    }
    let total = (read_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_usize(G_NATION_COUNT) as i32).wrapping_mul(2));
    let asia = read_usize(KEY_CONTINENT_ASIA);
    let mut c = read_usize(G_CLUB_TABLE);
    for _ in 0..total.max(0) {
        let nation = Nation(read_usize(c + club::NATION));
        if !nation.is_null() {
            let continent = nation.continent();
            if continent != 0 && read_usize(continent) == asia && (n as i16) < 0x4b0 && c != read_usize(list) && c != read_usize(list + 0x40) {
                clubs[n] = c;
                n += 1;
            }
        }
        c += club::SIZE;
    }
    covered!("cqu_11 club draw");
    exe::comp_util_sort_and_shuffle_top_clubs(clubs.as_mut_ptr() as usize, (n as u32).wrapping_sub(1), 0x28); // cqu11: 40 picks in the shuffle
    let (mut east_marks, mut west_marks) = ([0u8; 23], [0u8; 19]);
    for &club in clubs.iter().take((n as i16).max(0) as usize) {
        let i = asia_cup_winner_zone_index(club, &east);
        let (mark, first) = if i > -1 {
            (&mut east_marks[i as usize], 0usize)
        } else {
            let j = asia_cup_winner_zone_index(club, &west);
            if j <= -1 {
                continue;
            }
            (&mut west_marks[j as usize], 16usize)
        };
        if *mark != 0 || asia_cup_winner_in_champions_league(club) {
            continue;
        }
        *mark = 1;
        if read_usize(list + (first + 15) * 4) != 0 {
            continue;
        }
        if let Some(k) = (first..first + 16).find(|&k| read_usize(list + k * 4) == 0) {
            write_usize(list + k * 4, club);
        }
    }
    1
}

pub(crate) unsafe extern "thiscall" fn compare_asia_cup_winner_select_qualified_clubs(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ASIA_CUP_WINNER_SELECT_QUALIFIED_CLUBS);
    let r = o(this);
    if r == 1 {
        let list = read_usize(this + 0xb2);
        let mut nations: Vec<i32> = Vec::new();
        let mut error = 0usize;
        for k in 0..32usize {
            let c = if list != 0 { read_usize(list + k * 4) } else { 0 };
            if c == 0 {
                continue;
            }
            if asia_cup_winner_in_champions_league(c) {
                error = 100 + k;
            }
            if k == 0 || k == 16 {
                continue;
            }
            let nation = asia_cup_winner_club_nation_id(c);
            let zone = if k < 16 { &ASIAN_CUP_WINNERS_EAST_ZONE_KEYS[..] } else { &ASIAN_CUP_WINNERS_WEST_ZONE_KEYS[..] };
            if !zone.iter().any(|&g| read_usize(g) as i32 == nation) || nations.contains(&nation) {
                error = k;
            }
            nations.push(nation);
        }
        let ok = list != 0 && read_usize(this + 0xb6) == 0x20 && error == 0;
        SCOREBOARD.record(ok, || format!("asia_cup_winner_select_qualified_clubs({:#x}): list {:#x} +0xb6 {} wrong place {}", this, list, read_usize(this + 0xb6), error));
    }
    r
}

/// Stage 0 of the Asian Cup Winners' Cup, a knockout of the 16 west zone qualifiers (boxes at lines 0x2fd and 0x314).
/// (The original's SEH frame is only for the stage's new: no destructor on the normal path.)
/// Original: `AsiaCupWinner::create_zone_stage` (0x40ddf0, asia_cup_winner.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn asia_cup_winner_create_zone_stage(this: usize) {
    asia_cup_winner_zone_knockout_stage(this, 0x987b60, (0x2fd, 0x314), "cqu_11 west zone team")
}

pub(crate) unsafe extern "thiscall" fn compare_asia_cup_winner_create_zone_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ASIA_CUP_WINNER_CREATE_ZONE_STAGE);
    asia_cup_winner_check_zone_stage("asia_cup_winner_create_zone_stage", this, || o(this))
}

/// The predicted positions of the two Asian Cup Winners' Cup zones (boxes at lines 0x3db and 0x3e3).
/// Original: `AsiaCupWinner::rank_teams` (0x40e3b0, asia_cup_winner.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn asia_cup_winner_rank_teams(this: usize) {
    asia_cup_winner_predicted_positions(this, 0x987b60, (0x3db, 0x3e3))
}

pub(crate) unsafe extern "thiscall" fn compare_asia_cup_winner_rank_teams(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ASIA_CUP_WINNER_RANK_TEAMS);
    asia_cup_winner_check_positions("asia_cup_winner_rank_teams", this, || o(this))
}

/// The index of the club's nation in a list of nation ids (0x40cba0(club, list, n): 3 pushes; add esp, 0xc -> cdecl;
/// returns the index byte or −1).
pub(crate) unsafe fn asia_cup_winner_zone_index(club: usize, list: &[u32]) -> i8 {
    exe::asia_cup_winner_find_club_nation_slot(club, list.as_ptr(), list.len() as u32) as i8
}

/// Does the club already play the Asian Club Championship (0x40e7b0(club): push; add esp, 4 -> cdecl): in that cup's
/// team table or in that of its stage 0.
pub(crate) unsafe fn asia_cup_winner_in_champions_league(club: usize) -> bool {
    exe::asia_cup_winner_club_in_club_championship(club) != 0
}

/// The winner of the cup `cup_key` of the nation `nation_key` (when the nation's `nation::LEAGUE_SELECTED` byte is set,
/// in its [`G_NATION_TABLE`] record): the winner, or the runner-up if the winner is already in place 0 or 16 or plays
/// the Champions League; the runner-up in those conditions (or null) does not enter.
pub(crate) unsafe fn asia_cup_winner_national_cup_club(this: usize, nation_key: usize, cup_key: usize) -> Option<usize> {
    let n = read_usize(nation_key);
    if read_u8(read_usize(G_NATION_TABLE).wrapping_add(n.wrapping_mul(nation::SIZE)).wrapping_add(nation::LEAGUE_SELECTED)) == 0 {
        return None;
    }
    let cup_record = comp_record_of_key(cup_key);
    let list = read_usize(this + 0xb2);
    let c = exe::comp_util_get_latest_placed_club(cup_record, 0); // cqu11: the cup winner
    if c != read_usize(list) && c != read_usize(list + 0x40) && !asia_cup_winner_in_champions_league(c) {
        return if c != 0 { Some(c) } else { None };
    }
    let c = exe::comp_util_get_latest_placed_club(cup_record, 1);
    if c == read_usize(list) || c == read_usize(list + 0x40) || asia_cup_winner_in_champions_league(c) || c == 0 {
        return None;
    }
    Some(c)
}

/// The Asian Cup Winners' Cup: vtable 0x967574, 2 stages; AsiaCupWinner::select_qualified_clubs (0x40d830),
/// create_team_entries (0x40d4d0), create_zone_stage (0x40ddf0) and rank_teams (0x40e3b0); error boxes of source
/// 0x987b60 at lines 0x4e, 0x55, 0x64 and 0x74.
pub(crate) const ASIAN_CUP_WINNERS_CUP_PARTS: AsianClubCupParts = AsianClubCupParts { vtable: 0x967574, stages: 2, file: 0x987b60, select_qualified: 0x40d830, build_team_table: 0x40d4d0, create_stage: 0x40ddf0, predicted: 0x40e3b0, rows: [0x4e, 0x55, 0x64, 0x74] };

/// The Cup Winners' Cup east zone (23 nations, in the original's order; Pakistan comes after Laos).
pub(crate) const ASIAN_CUP_WINNERS_EAST_ZONE_KEYS: [usize; 23] = [KEY_NATION_BANGLADESH, KEY_NATION_BRUNEI_DARUSSALAM, KEY_NATION_CHINA, KEY_NATION_HONG_KONG, KEY_NATION_INDIA,
    KEY_NATION_INDONESIA, KEY_NATION_JAPAN, KEY_NATION_KAMPUCHEA, KEY_NATION_LAOS, KEY_NATION_PAKISTAN, KEY_NATION_MACAU, KEY_NATION_MALAYSIA, KEY_NATION_MALDIVES,
    KEY_NATION_MYANMAR, KEY_NATION_NEPAL, KEY_NATION_NORTH_KOREA, KEY_NATION_SINGAPORE, KEY_NATION_SOUTH_KOREA, KEY_NATION_SRI_LANKA, KEY_NATION_TAIWAN,
    KEY_NATION_THAILAND, KEY_NATION_THE_PHILIPPINES, KEY_NATION_VIETNAM];
