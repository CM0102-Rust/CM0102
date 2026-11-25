// ---------- rxg_3: the Club World Championship: one entrant and the knockout stage (world_club_champ.cpp) ----------

/// world_club_champ.cpp (the error boxes).
const WORLD_CLUB_CHAMP_SOURCE: usize = 0xad5764;

/// The league table's records (`G_LEAGUE_TABLE`, 0x48 bytes each, up to 0xb64738): +0 the nation, +4 the continent,
/// +0xc the division count, +0x10 the division array (+4 of each the division's competition), +0x1e the cup.
const LEAGUE_TABLE_RECORD: usize = 0x48;
const LEAGUE_TABLE_END: usize = 0xb64738;

/// The competition record of a club competition id (`G_CLUB_COMP_TABLE`, 0x6b bytes each).
unsafe fn club_comp_record(id: i32) -> usize {
    read_usize(G_CLUB_COMP_TABLE).wrapping_add((id as usize).wrapping_mul(0x6b))
}

/// Whether `club` is among the first `count` entries of `list` (a signed word count, as the original's loops).
unsafe fn world_club_champ_listed(list: usize, count: i16, club: usize) -> bool {
    (0..count.max(0) as usize).any(|k| read_usize(list + 4 * k) == club)
}

/// The latest placed club of `comp` (`exe::comp_util_get_latest_placed_club`) at `place`, moving to the next place
/// when it is already in the list, for each of `places` in turn.
unsafe fn world_club_champ_placed_club(comp: usize, list: usize, count: i16, places: &[u32]) -> usize {
    let mut club = exe::comp_util_get_latest_placed_club(comp, places[0]);
    for &place in &places[1..] {
        if world_club_champ_listed(list, count, club) {
            club = exe::comp_util_get_latest_placed_club(comp, place);
        }
    }
    club
}

/// A random entrant: the clubs (of the first club count − 2 × nation count) that `belongs` accepts with reputation at
/// least 5000, then 3500, 2000... (1500 lower each pass) until there are 6 (at most `cap`; the continent's search
/// stops below -2000, the nation's does not), shuffled at the top (`exe::comp_util_sort_and_shuffle_top_clubs`, the
/// first 4 or fewer); the first not yet in the list. None: the "Error" box at `line` and a random club of all.
unsafe fn world_club_champ_random_entrant(list: usize, count: i16, belongs: &dyn Fn(Club) -> bool, cap: i16, floor: bool, line: u32) -> usize {
    let mut candidates: Vec<usize> = Vec::with_capacity(cap as usize);
    let clubs = read_i32(G_CLUB_COUNT).wrapping_sub(2 * read_i32(G_NATION_COUNT));
    let mut threshold: i16 = 5000;
    loop {
        if floor && threshold <= -2000 {
            break;
        }
        candidates.clear();
        for k in 0..clubs.max(0) as isize {
            let club = club_at(k);
            if belongs(club) && club.reputation() >= threshold {
                candidates.push(club.addr());
                if candidates.len() >= cap as usize {
                    break;
                }
            }
        }
        threshold = threshold.wrapping_sub(1500); // rxg_3: the next reputation band
        if candidates.len() >= 6 {
            break;
        }
    }
    let n = candidates.len() as u32;
    exe::comp_util_sort_and_shuffle_top_clubs(candidates.as_mut_ptr() as usize, n, n.min(4));
    if let Some(&club) = candidates.iter().find(|&&c| !world_club_champ_listed(list, count, c)) {
        return club;
    }
    show_error_at(WORLD_CLUB_CHAMP_SOURCE, line);
    club_at(exe::utils_random(clubs) as isize).addr()
}

/// Rule: a continent's Club World Championship entrant is the winner of its club championship (the runner-up when the
/// winner is already in), when the continent has a selected nation or it is 2001; otherwise a random club of the
/// continent with high reputation. The host's entrant is its first league division's champion (then the second and the
/// third placed when already in), when that league is selected or it is 2001; otherwise a random club of the nation.
///
/// `WorldClubChamp::pick_entrant(continent, nation, list, count, club comp)`. With a continent (not -1): the
/// competition is the cup of the league table record (`G_LEAGUE_TABLE`) whose continent is that one and whose nation
/// is 0; none or no cup, Asia's Super Cup, Oceania's Club Championship or the CONCACAF Champions' Cup for those three;
/// a `club comp` other than -1 replaces it. With a competition, and the continent has a selected nation
/// (`exe::key_nation_continent_has_selected_nation`) or the year (+0x40) is 2001: its latest winner
/// (`exe::comp_util_get_latest_placed_club` place 0, place 1 when already listed); a club (not 0) not in the list is
/// the answer. Otherwise `world_club_champ_random_entrant` of the continent's clubs (at most 2000, the error line
/// 0x4fc). With continent -1, the nation's league table record: its first division that is a league division
/// (`exe::comp_util_is_league_division`); with it, and the division's nation selected (+0x11c) or 2001: places 0, 1
/// and 2 in turn; the club not in the list is the answer (a 0 club too). Otherwise the random entrant of the nation's
/// clubs (at most 1000, the error line 0x556).
///
/// Defects of the original: the nation's random search has no reputation floor, so a host nation with fewer than 6
/// clubs makes it loop forever (the 16-bit threshold wraps around); the host's champion is not checked for 0. The copy
/// does the same.
/// Original: `WorldClubChamp::pick_entrant` (0x92ae60, world_club_champ.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn world_club_champ_pick_entrant(this: usize, continent: i32, nation: i32, list: usize, count: u32, club_comp: i32) -> usize {
    let count = count as i16;
    if continent != -1 {
        covered!("rxg_3 continental entrant");
        let mut comp = 0usize;
        let mut record = G_LEAGUE_TABLE;
        while record < LEAGUE_TABLE_END {
            let c = read_usize(record + 4); // the record's continent
            if c != 0 && read_i32(c) == continent && read_usize(record) == 0 {
                comp = read_usize(record + 0x1e); // the continent's club championship
                break;
            }
            record += LEAGUE_TABLE_RECORD;
        }
        if comp == 0 {
            for (key, cup) in [(KEY_CONTINENT_ASIA, KEY_CLUB_COMP_ASIAN_SUPER_CUP), (KEY_CONTINENT_OCEANIA, KEY_CLUB_COMP_OCEANIA_CLUB_CHAMPIONSHIP), (KEY_CONTINENT_NORTH_AMERICA, KEY_CLUB_COMP_CONCACAF_CHAMPIONS_CUP)] {
                if continent == read_i32(key) {
                    comp = club_comp_record(read_i32(cup));
                    break;
                }
            }
        }
        if club_comp != -1 {
            comp = club_comp_record(club_comp);
        }
        if comp != 0 {
            let selected = exe::key_nation_continent_has_selected_nation(read_usize(G_CONTINENT_TABLE).wrapping_add((continent as usize).wrapping_mul(0xc6))) != 0;
            if selected || read_u16(this + 0x40) == 2001 { // +0x40: the season year
                let mut club = exe::comp_util_get_latest_placed_club(comp, 0);
                if club != 0 {
                    if world_club_champ_listed(list, count, club) {
                        club = exe::comp_util_get_latest_placed_club(comp, 1); // the runner-up
                    }
                    if !world_club_champ_listed(list, count, club) {
                        return club;
                    }
                }
            }
        }
        let belongs = |club: Club| {
            let n = club.nation();
            !n.is_null() && n.continent() != 0 && read_i32(n.continent()) == continent
        };
        return world_club_champ_random_entrant(list, count, &belongs, 2000, true, 0x4fc);
    }
    covered!("rxg_3 host entrant");
    let mut record = G_LEAGUE_TABLE;
    while record < LEAGUE_TABLE_END {
        let n = read_usize(record);
        if n != 0 && read_i32(n) == nation {
            break;
        }
        record += LEAGUE_TABLE_RECORD;
    }
    if record < LEAGUE_TABLE_END {
        let divisions = read_i32(record + 0xc);
        let mut k: i16 = 0;
        while (k as i32) < divisions {
            let entry = read_usize(read_usize(record + 0x10) + 4 * k as usize);
            if exe::comp_util_is_league_division(read_i32(read_usize(entry + 4))) != 0 {
                let comp = read_usize(read_usize(read_usize(record + 0x10) + 4 * k as usize) + 4);
                if comp != 0 && (Nation(read_usize(comp + competition::NATION)).league_selected() != 0 || read_u16(this + 0x40) == 2001) {
                    let club = world_club_champ_placed_club(comp, list, count, &[0, 1, 2]);
                    if !world_club_champ_listed(list, count, club) {
                        return club;
                    }
                }
                break;
            }
            k += 1;
        }
    }
    let belongs = |club: Club| !club.nation().is_null() && read_i32(club.nation().addr()) == nation;
    world_club_champ_random_entrant(list, count, &belongs, 1000, false, 0x556)
}

/// Comparer for `world_club_champ_pick_entrant`: the original only, with the plan: the answer is a club of the table.
pub(crate) unsafe extern "thiscall" fn compare_world_club_champ_pick_entrant(this: usize, continent: i32, nation: i32, list: usize, count: u32, club_comp: i32) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, i32, i32, usize, u32, i32) -> usize>(%%I0%%)(this, continent, nation, list, count, club_comp);
    let first = read_usize(G_CLUB_TABLE);
    let ok = r >= first && (r - first) % club::SIZE == 0 && (r - first) / club::SIZE < read_u32(G_CLUB_COUNT) as usize;
    SCOREBOARD.record(ok, || format!("world_club_champ_pick_entrant({}, {}, count {}): {:#x} is not a club", continent, nation, count, r));
    r
}

/// The prize of a club knocked out of the Club World Championship: 2,500,000 to its finances (`exe::
/// financetable_get_club_finance` on `G_FINANCE_TABLE`, asked twice as the original does; none, nothing).
unsafe fn world_club_champ_pay_knockout_prize(club: usize) {
    if exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club) != 0 {
        exe::finance_add_prize_money(exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club), 2_500_000);
    }
}

/// Rule: the three group winners and the best runner-up (the three runners-up ranked in a table of their own) go to a
/// four-club knockout stage; every other club gets 2,500,000 as it goes out.
///
/// `WorldClubChamp::create_knockout_stage()`: the list of 4 (`exe::crt_malloc`(0x10); none: the "Error" box at line
/// 0x354 and nothing else) with the winners of the championship's own table and of its two group stages; a scratch
/// League_Stage of 3 (`FN_LEAGUE_STAGE_LEAGUE_STAGE` with this, 3, no list, the competition, the win and draw points,
/// the tie-breaks, the promoted count, the year, stage -2, flags 2) whose table rows take the three runners-up's rows
/// (0x3b bytes, the dwords +0x40 and +0x44 cleared), sorted (`exe::league_sort_table`); its first is the fourth
/// entrant and the scratch stage is deleted (its vtable's first slot with 1). Then the cup's rounds
/// (`comp_vtable::CREATE_SCHEDULE` with 2) and a Cup_Stage of 4 (`FN_CUP_STAGE_CUP_STAGE`: the list, the rounds, the
/// competition, the table, the year, 2, 2, the id, 0x14) into stage 2; the rounds and the list freed. No stage: the
/// box at line 0x387. Otherwise the first round set up (`exe::cup_set_up_round`), `league::CURRENT_STAGE` = 2 and the
/// prize (`world_club_champ_pay_knockout_prize`) for the third and the fourth of the three groups and for the
/// runners-up other than the fourth entrant.
///
/// Defects of the original: the fourth entrant is read from the list after it is freed (the copy reads it before);
/// a failed `new` of the scratch stage writes through a null pointer.
/// Original: `WorldClubChamp::create_knockout_stage` (0x92a140, world_club_champ.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn world_club_champ_create_knockout_stage(this: usize) {
    let l = League(this);
    let list = exe::crt_malloc(0x10);
    if list == 0 {
        show_error_at(WORLD_CLUB_CHAMP_SOURCE, 0x354);
        return;
    }
    covered!("rxg_3 club world championship knockout stage");
    // the three group tables: the championship's own and its stages 0 and 1
    let tables = [l.table(), read_usize(read_usize(l.stages()) + league::TABLE), read_usize(read_usize(l.stages() + 4) + league::TABLE)];
    for (k, &t) in tables.iter().enumerate() {
        write_usize(list + 4 * k, read_usize(t)); // the group winner
    }
    let e = exe::crt_operator_new(0xee);
    let runners_up = if e != 0 {
        let ctor: LeagueStageCtorFn = core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
        ctor(e, this, 3, 0, 0, l.competition().addr(), 0, 0, l.win_points() as u32, l.draw_points() as u32, 0,
            this + league::TIEBREAKS, this + league::PROMOTED_COUNT, l.year() as u16 as u32, -2i32 as u32, 0, 0, 1, 0, 2, -1, 0, 2)
    } else {
        0
    };
    for (k, &t) in tables.iter().enumerate() {
        let row = read_usize(runners_up + league::TABLE) + k * league_table_row::SIZE;
        for b in 0..league_table_row::SIZE {
            write_u8(row + b, read_u8(t + league_table_row::SIZE + b)); // the group's second row
        }
        write_u32(row + 0x44, 0);
        write_u32(row + 0x40, 0);
    }
    exe::league_sort_table(runners_up, 0usize);
    let best_second = read_usize(read_usize(runners_up + league::TABLE));
    write_usize(list + 0xc, best_second);
    if runners_up != 0 {
        let delete: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(read_usize(read_usize(runners_up)));
        delete(runners_up, 1);
    }
    let schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut id) = (0i16, 0i16);
    let table = schedule(this, 2, &mut rounds, &mut id, 0);
    let p = exe::crt_operator_new(0xb2);
    let stage = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this, 4, list, rounds as u16 as u32, l.competition().addr(), table, l.year() as u16 as u32, 2, 2, id as u16 as u32, 0x14, 0, 0, 0, core::ptr::null())
    } else {
        0
    };
    write_usize(l.stages() + 8, stage);
    exe::crt_free(table);
    exe::crt_free(list);
    if read_usize(l.stages() + 8) == 0 {
        show_error_at(WORLD_CLUB_CHAMP_SOURCE, 0x387);
        return;
    }
    exe::cup_set_up_round(read_usize(l.stages() + 8), 0);
    write_u32(this + league::CURRENT_STAGE, 2);
    let tables = [l.table(), read_usize(read_usize(l.stages()) + league::TABLE), read_usize(read_usize(l.stages() + 4) + league::TABLE)];
    for &t in &tables {
        world_club_champ_pay_knockout_prize(read_usize(t + 2 * league_table_row::SIZE)); // third
        world_club_champ_pay_knockout_prize(read_usize(t + 3 * league_table_row::SIZE)); // fourth
    }
    for &t in &tables {
        let second = read_usize(t + league_table_row::SIZE);
        if best_second != second {
            world_club_champ_pay_knockout_prize(second); // rxg_3: a runner-up that goes out
        }
    }
}

/// Comparer for `world_club_champ_create_knockout_stage`: it allocates, builds the stage and pays: the original only,
/// with the plan: with a stage 2, the current stage is 2.
pub(crate) unsafe extern "thiscall" fn compare_world_club_champ_create_knockout_stage(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this);
    let stage = read_usize(League(this).stages() + 8);
    SCOREBOARD.record(stage == 0 || read_i32(this + league::CURRENT_STAGE) == 2, || format!("world_club_champ_create_knockout_stage({:#x}): stage {:#x}, current {}", this, stage, read_i32(this + league::CURRENT_STAGE)));
}
