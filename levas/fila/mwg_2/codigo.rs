// ---------- batch %%LEVA%%: virtual players: counting a squad and topping it up (virtual_staff.cpp) ----------

/// Exe addresses the squad top-up of the virtual staff uses that have no name in exe_fns.rs.
pub(crate) mod virtual_squad_fns {
    pub(crate) const GET_PLAYER_POSITION_GROUP: usize = 0x90e020; // VIRTUAL_STAFF::get_player_position_group
    pub(crate) const COUNT_AVAILABLE_PLAYERS: usize = 0x90d8f0; // VIRTUAL_STAFF::count_available_players
    pub(crate) const CREATE_VIRTUAL_PLAYER: usize = 0x90e0a0; // VIRTUAL_STAFF::create_virtual_player
    pub(crate) const GET_MAX_SQUAD_SIZE: usize = 0x7754c0; // NationalTeams::get_max_squad_size
    pub(crate) const GET_MIN_SQUAD_SIZE: usize = 0x775520; // NationalTeams::get_min_squad_size
    /// The fewest players of each of the 8 position groups (`get_player_position_group`) a side needs: a club or
    /// main national squad at +0 (4, 4, 4, 8, 4, 8, 4, 8 from the constructor), a B national squad at +8 (6, 4, 4, 12,
    /// 4, 8, 4, 8) (bytes of the VIRTUAL_STAFF object).
    pub(crate) const MIN_BY_GROUP: usize = 0x0;
    /// The fewest players of each position group before a club outside the selected leagues tops up its eligible
    /// count past 20 (2, 2, 2, 4, 2, 4, 2, 4; bytes of the VIRTUAL_STAFF object).
    pub(crate) const MIN_BY_GROUP_LOWER_CLUB: usize = 0x10;
    /// The most virtual players one top-up creates (the size of an id block).
    pub(crate) const BLOCK_SIZE: u8 = 25;
    /// The player record size (70 bytes, `G_PLAYER_DATA_TABLE`).
    pub(crate) const PLAYER_SIZE: usize = 0x46;
}

/// `create_virtual_player` through the exe (the detour runs its comparer in comparison mode).
unsafe fn virtual_create(this: usize, club: Club, group: u8, staff_id: u32, player_id: u32, youngsters: u32, reserve: u32, squad_number: u8) {
    let f: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32, u32) = core::mem::transmute(virtual_squad_fns::CREATE_VIRTUAL_PLAYER);
    f(this, club.addr(), group as u32, staff_id, player_id, youngsters, reserve, squad_number as u32);
}

/// Counts the players of a club's squad who can play a match, by position group, and drops its leftover virtual
/// players.
///
/// `VIRTUAL_STAFF::count_available_players(club, pass, fixture, &counts, &available, &real, &eligible)`; `pass` is not
/// read. A B national squad is a national team row (`exe::database_is_national_team`) that is not a senior one
/// (`exe::club_is_national_team`). A zeroed 8-byte table (`exe::crt_calloc`) gets one count per position group. Over
/// the 50 squad slots: a virtual player (staff id at or above the staff count) leaves the team selection
/// (`exe::tactics_remove_player_from_selection` with 1, then 0) and the slot; every other person counts as real; one
/// away on international duty today (`exe::nationalteams_get_international_duty_club`), or at a club who is not on
/// its staff (`.club_job()`), stops there; the eligible count takes those marked for the squad (`.playing_squad()` bit
/// 4 in a B squad, bit 1 elsewhere); the badly injured (`exe::injury_get_player_injury_status` 2) and, with a fixture
/// not marked -2, the banned for its competition (`exe::discipline_is_player_banned`) stop; the rest are available
/// and add one to their position group (`get_player_position_group`, none for 0xff). The counts go out through the
/// non-null pointers; without a table pointer the table is freed (the check of 30 available and enough goalkeepers
/// before it ends the same way both ways). Returns 1.
/// Original: `VIRTUAL_STAFF::count_available_players` (0x90d8f0, virtual_staff.cpp, thiscall, ret 0x1c).
pub(crate) unsafe extern "thiscall" fn count_available_players(this: usize, club: usize, _pass: u32, fixture: usize, out_counts: *mut usize, out_available: *mut u8, out_real: *mut u8, out_eligible: *mut u8) -> u32 {
    let c = Club(club);
    let b_squad = exe::database_is_national_team(c) != 0 && exe::club_is_national_team(c) == 0;
    let counts = exe::crt_calloc(8, 1);
    let group: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(virtual_squad_fns::GET_PLAYER_POSITION_GROUP);
    let (mut available, mut real, mut eligible) = (0u8, 0u8, 0u8);
    for i in 0..50 {
        let s = c.squad(i);
        if s == 0 {
            continue;
        }
        if read_i32(s) >= read_i32(G_STAFF_COUNT) {
            exe::tactics_remove_player_from_selection(read_usize(G_TACTICS), c, s, 1, 1);
            exe::tactics_remove_player_from_selection(read_usize(G_TACTICS), c, s, 0, 1);
            c.set_squad(i, 0);
            continue;
        }
        real = real.wrapping_add(1);
        if exe::nationalteams_get_international_duty_club(read_usize(G_NATIONAL_TEAMS), s, G_CURRENT_DATE, c, 0, 0) != 0 {
            continue;
        }
        let person = Staff(s);
        if exe::database_is_national_team(c) == 0 && person.club_job() != c {
            continue;
        }
        let mark = person.playing_squad();
        if (b_squad && mark & 4 != 0) || (!b_squad && mark & 1 != 0) {
            eligible = eligible.wrapping_add(1);
        }
        if exe::injury_get_player_injury_status(read_usize(G_INJURY), s) == 2 {
            covered!("mwg_2 injured player");
            continue; // mwg2: the injured are not available
        }
        if fixture != 0 && read_i32(fixture + fixture::COMP_ID) != -2 && exe::discipline_is_player_banned(read_usize(G_DISCIPLINE), s, read_usize(fixture + fixture::COMPETITION), 1, 0) != 0 {
            covered!("mwg_2 banned player");
            continue;
        }
        let g = group(this, s);
        if g != 0xff {
            let k = counts.wrapping_add(g as i8 as isize as usize);
            write_u8(k, read_u8(k).wrapping_add(1));
        }
        available = available.wrapping_add(1);
    }
    if !out_counts.is_null() {
        *out_counts = counts;
    }
    if !out_available.is_null() {
        *out_available = available;
    }
    if !out_real.is_null() {
        *out_real = real;
    }
    if !out_eligible.is_null() {
        *out_eligible = eligible;
    }
    if out_counts.is_null() {
        exe::crt_free(counts);
    }
    1
}

/// Comparer for `count_available_players`.
///
/// The original runs first (it drops the virtual players from the squad and the selection) with its own outputs; the
/// copy then counts the same squad (the real players are untouched) with its own: the three counts and the 8 group
/// counts must match. The caller gets the original's outputs, and its table is freed when the caller asked for none.
pub(crate) unsafe extern "thiscall" fn compare_count_available_players(this: usize, club: usize, pass: u32, fixture: usize, out_counts: *mut usize, out_available: *mut u8, out_real: *mut u8, out_eligible: *mut u8) -> u32 {
    type F = unsafe extern "thiscall" fn(usize, usize, u32, usize, *mut usize, *mut u8, *mut u8, *mut u8) -> u32;
    let (mut ot, mut oa, mut or, mut oe) = (0usize, 0u8, 0u8, 0u8);
    let r = original_fn::<F>(%%I0%%)(this, club, pass, fixture, &mut ot, &mut oa, &mut or, &mut oe);
    let (mut nt, mut na, mut nr, mut ne) = (0usize, 0u8, 0u8, 0u8);
    let n = count_available_players(this, club, pass, fixture, &mut nt, &mut na, &mut nr, &mut ne);
    let table = |t: usize| (0..8).map(|k| read_u8(t + k)).collect::<Vec<u8>>();
    let (to, tn) = (table(ot), table(nt));
    SCOREBOARD.record(r == n && oa == na && or == nr && oe == ne && to == tn, || format!("count_available_players(club {:x}): original {} available {} real {} eligible {} groups {:?}; rust {} available {} real {} eligible {} groups {:?}",
        club, r, oa, or, oe, to, n, na, nr, ne, tn));
    exe::crt_free(nt);
    if !out_counts.is_null() {
        *out_counts = ot;
    } else {
        exe::crt_free(ot);
    }
    if !out_available.is_null() {
        *out_available = oa;
    }
    if !out_real.is_null() {
        *out_real = or;
    }
    if !out_eligible.is_null() {
        *out_eligible = oe;
    }
    r
}

/// Creates virtual players for the position groups where a club (or national squad) is short, until it has enough.
///
/// `VIRTUAL_STAFF::top_up_club_squad(club, staff_id, player_id, fixture, pass)`. First the seed to put back at the
/// end (`exe::utils_random(30000)` + 1). A club without a nation (`.nation()`) or one of the MLS All Stars sides
/// (`G_KEY_CLUB_MLS_ALL_STARS_EAST`/`WEST`) is left alone. The squad is counted (`count_available_players`: the group
/// table, available, real and eligible). The created players are youngsters (`create_virtual_player`: born 17 years before, weaker)
/// at a senior national team, at a club whose nation's league is selected (`nation::LEAGUE_SELECTED`) and whose
/// division has a competition object (`G_COMP_OBJECT_TABLE`), and at a Japanese club whose division is none of the
/// J.League 1 and 2, the Football, University and Administrative Division leagues; a B national squad uses the second row of group minimums
/// (`MIN_BY_GROUP` + 8). The target: a national squad managed by a human (`exe::human_manager_is_human_club` with 1,
/// 1) is left alone; otherwise the main and B squad maxima (`NationalTeams::get_max_squad_size` with 0 and 1) add up
/// to the target, a B squad without a minimum (`get_min_squad_size` with 1 equal to 0) drops the B part; at a senior
/// national team whose nation's team is human, a human who does not manage all national teams
/// (`exe::human_manager_controls_all_national_teams`) leaves it alone, and with no main squad minimum, or more
/// available than it, a nation with more than 500 staff (`.number_staff()`) gets no virtual players (target 0). A
/// club's target is 50, and a computer club needs 20 eligible players before the lower minimums. The reserve flag
/// starts as `exe::database_club_has_reserve_squad`. While the real players are fewer than the target and fewer than
/// 25 were created, each round over the 8 groups (stopping at 50 real players or 25 created): a group under its
/// minimum (`MIN_BY_GROUP`) sets the reserve flag after group 0 when more than 20 are available; the generator is
/// seeded (`exe::crt_srand`) with (players already created for the group + 10000) × (club id + 1) × (group + 1000), so
/// a club always gets the same virtual players; a national squad creates a main squad player (`create_virtual_player`,
/// not reserve, one more eligible) while the main maximum is above the eligible, else at a B squad with a B maximum
/// above available − eligible a reserve one (and ends everything when available − eligible passes 20); a club creates
/// a normal player (one more eligible), except when its division is a selected league or it already has its group
/// minimum (`MIN_BY_GROUP_LOWER_CLUB`) and 20 eligible, and it has the computer's 20 eligible: then a reserve player
/// when the reserve flag is set, or none with 20 eligible. Each step moves the staff and player ids, the group count,
/// available and created on (the squad numbers run from 51); a round without a short group ends it. At the end the
/// group table is freed and the generator seeded again with the seed of the start.
/// Original: `VIRTUAL_STAFF::top_up_club_squad` (0x90dae0, virtual_staff.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn top_up_club_squad(this: usize, club: usize, staff_id: u32, player_id: u32, fixture: usize, pass: u32) {
    use virtual_squad_fns as vs;
    let c = Club(club);
    let seed = exe::utils_random(30000).wrapping_add(1);
    if c.nation().is_null() {
        return;
    }
    if read_u32(club) == read_u32(G_KEY_CLUB_MLS_ALL_STARS_WEST) || read_u32(club) == read_u32(G_KEY_CLUB_MLS_ALL_STARS_EAST) {
        return;
    }
    let count: unsafe extern "thiscall" fn(usize, usize, u32, usize, *mut usize, *mut u8, *mut u8, *mut u8) -> u32 = core::mem::transmute(vs::COUNT_AVAILABLE_PLAYERS);
    let (mut counts, mut available, mut real, mut eligible) = (0usize, 0u8, 0u8, 0u8);
    if count(this, club, pass, fixture, &mut counts, &mut available, &mut real, &mut eligible) == 0 {
        if counts != 0 {
            exe::crt_free(counts);
        }
        return;
    }
    let squad_size: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(vs::GET_MAX_SQUAD_SIZE);
    let min_squad: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(vs::GET_MIN_SQUAD_SIZE);
    let national_teams = read_usize(G_NATIONAL_TEAMS);
    let (mut b_squad, mut youngsters) = (0usize, 0u32);
    if exe::database_is_national_team(c) != 0 {
        if exe::club_is_national_team(c) != 0 {
            youngsters = 1;
        } else {
            b_squad = 1;
        }
    } else {
        let n = c.nation();
        let d = c.division();
        let comp_object = |d: Competition| read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_u32(d.addr()) as usize * 4) != 0;
        let japanese_lower = |d: Competition| {
            let id = read_u32(d.addr());
            ![KEY_CLUB_COMP_JAPANESE_ADMINISTRATIVE_DIVISION, KEY_CLUB_COMP_JAPANESE_UNIVERSITY_LEAGUE, KEY_CLUB_COMP_JAPANESE_FOOTBALL_LEAGUE, KEY_CLUB_COMP_JAPANESE_J_LEAGUE_2, KEY_CLUB_COMP_JAPANESE_J_LEAGUE_1]
                .iter()
                .any(|&k| id == read_u32(k))
        };
        if n.league_selected() != 0 && !d.is_null() && comp_object(d) {
            covered!("mwg_2 selected league club");
            youngsters = 1; // mwg2: a selected league's club gets young virtual players
        } else if read_u32(n.addr()) == read_u32(KEY_NATION_JAPAN) && !d.is_null() && japanese_lower(d) {
            covered!("mwg_2 japanese club outside the named leagues");
            youngsters = 1;
        }
    }
    let (mut max_main, mut max_b, mut min_eligible) = (0xffu8, 0xffu8, 0xffu8);
    let mut target: u8;
    if exe::database_is_national_team(c) != 0 {
        if exe::human_manager_is_human_club(c, 1, 1, 0, 0) != 0 {
            return;
        }
        max_main = squad_size(national_teams, club, 0);
        max_b = squad_size(national_teams, club, 1);
        target = max_main.wrapping_add(max_b);
        if exe::club_is_national_team(c) == 0 {
            if min_squad(national_teams, club, 1) == 0 {
                max_b = 0;
                target = max_main;
            }
        } else {
            let team = exe::database_get_national_team(c.nation());
            if exe::human_manager_is_human_club(team, 0, 1, 0, 0) != 0 {
                let mut all = 0u32;
                if exe::human_manager_controls_all_national_teams(read_usize(G_HUMAN_MANAGER), 0usize, &mut all) != 0 || all == 0 {
                    return;
                }
                if pass == 0 && exe::club_is_national_team(c) == 0 {
                    return;
                }
            }
            let min = min_squad(national_teams, club, 0);
            if (min == 0 || available as i8 > min as i8) && c.nation().number_staff() > 500 {
                covered!("mwg_2 nation with enough staff");
                max_main = 0;
                max_b = 0;
                target = 0;
            }
        }
    } else {
        target = 50;
        if exe::human_manager_is_human_club(c, 1, 1, 0, 0) == 0 {
            min_eligible = 20;
        }
    }
    let mut reserve = exe::database_club_has_reserve_squad(c);
    if (real as i8) < (target as i8) {
        let mut created = [0u8; 8];
        let (mut staff_id, mut player_id) = (staff_id, player_id);
        let (mut squad_number, mut made) = (0x33u8, 0u8);
        'rounds: while (made as i8) < vs::BLOCK_SIZE as i8 {
            let mut progress = false;
            for g in 0..8u8 {
                if real as i8 >= 50 || made as i8 >= vs::BLOCK_SIZE as i8 {
                    break;
                }
                let have = read_i8(counts + g as usize);
                if have >= read_i8(this + vs::MIN_BY_GROUP + b_squad * 8 + g as usize) {
                    continue;
                }
                covered!("mwg_2 group below its minimum");
                if g != 0 && available as i8 > 20 && reserve == 0 {
                    reserve = 1;
                }
                let before = created[g as usize];
                let seed_g = (before as i8 as i32 + 10000).wrapping_mul(read_i32(club).wrapping_add(1)).wrapping_mul(g as i32 + 1000);
                exe::crt_srand(seed_g as u32); // mwg2: the seed of the club's virtual players
                if exe::database_is_national_team(c) != 0 {
                    if (max_main as i8) > (eligible as i8) {
                        virtual_create(this, c, g, staff_id, player_id, youngsters, 0, squad_number);
                        squad_number = squad_number.wrapping_add(1);
                        eligible = eligible.wrapping_add(1);
                    } else if exe::club_is_national_team(c) == 0 && max_b != 0 && (max_b as i8 as i32) > (available as i8 as i32 - eligible as i8 as i32) {
                        covered!("mwg_2 national b squad player");
                        virtual_create(this, c, g, staff_id, player_id, youngsters, 1, squad_number);
                        squad_number = squad_number.wrapping_add(1);
                        if available as i8 as i32 - eligible as i8 as i32 > 20 {
                            break 'rounds;
                        }
                    }
                } else {
                    let d = c.division();
                    let lower = d.is_null() || read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_u32(d.addr()) as usize * 4) == 0;
                    let normal = if lower && (have < read_i8(this + vs::MIN_BY_GROUP_LOWER_CLUB + g as usize) || (eligible as i8) < 20) {
                        true
                    } else if min_eligible != 0xff && (available as i8) < (min_eligible as i8) {
                        true
                    } else if reserve != 0 {
                        covered!("mwg_2 reserve player");
                        virtual_create(this, c, g, staff_id, player_id, youngsters, reserve, squad_number);
                        squad_number = squad_number.wrapping_add(1);
                        false
                    } else {
                        (eligible as i8) < 20
                    };
                    if normal {
                        virtual_create(this, c, g, staff_id, player_id, youngsters, 0, squad_number);
                        squad_number = squad_number.wrapping_add(1);
                        eligible = eligible.wrapping_add(1);
                    }
                }
                progress = true;
                write_u8(counts + g as usize, (have as u8).wrapping_add(1));
                staff_id = staff_id.wrapping_add(1);
                player_id = player_id.wrapping_add(1);
                available = available.wrapping_add(1);
                made = made.wrapping_add(1); // mwg2: one more created
                created[g as usize] = before.wrapping_add(1);
            }
            if !progress || (real as i8) >= (target as i8) {
                break;
            }
        }
    }
    exe::crt_free(counts);
    exe::crt_srand(seed as u32);
}

/// Comparer for `top_up_club_squad`.
///
/// Side by side from the same state: the generator (`crate::rng::STATE_REGIONS`, with the CRT seed), the club's 50
/// squad slots and the 25 staff and player records of the id block. The created players (`create_virtual_player`)
/// run their own comparer in the original's pass.
pub(crate) unsafe extern "thiscall" fn compare_top_up_club_squad(this: usize, club: usize, staff_id: u32, player_id: u32, fixture: usize, pass: u32) {
    let mut regions = crate::rng::STATE_REGIONS.to_vec();
    regions.push((club + club::SQUAD, 50 * 4));
    regions.push((read_usize(G_STAFF_TABLE) + staff_id as usize * staff::SIZE, virtual_squad_fns::BLOCK_SIZE as usize * staff::SIZE));
    regions.push((read_usize(G_PLAYER_DATA_TABLE) + player_id as usize * virtual_squad_fns::PLAYER_SIZE, virtual_squad_fns::BLOCK_SIZE as usize * virtual_squad_fns::PLAYER_SIZE));
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32)>(%%I1%%);
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, club, staff_id, player_id, fixture, pass), || {
        crate::rng::without_counting(|| top_up_club_squad(this, club, staff_id, player_id, fixture, pass))
    });
    SCOREBOARD.record(equal, || format!("top_up_club_squad(club {:x}, staff {}, player {}, pass {}): original {:x?} rust {:x?}", club, staff_id, player_id, pass, mo, mr));
}
