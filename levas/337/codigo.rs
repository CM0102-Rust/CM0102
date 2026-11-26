// ---------- gra_2c: the daily staff update (game.cpp): game_daily_staff_update, the whole routine ----------
//
// `game_daily_staff_update` (0x5ce2d0, game.cpp, cdecl, 30,544 bytes) runs every day over the staff table. Ported
// from the disassembly in named pieces, in the original's order (gra_2, gra_2b, gra_2c); every routine of the exe it
// calls goes through the exe's address (the hooked ones too), so the game passes through their hooks. Proven by the
// emulation of the whole routine against the original (levas/fila/gra_2c/check_routine.py).

/// `staff_pick_favoured_team_mate` (cdecl: person, club).
const FN_STAFF_PICK_FAVOURED_TEAM_MATE: usize = 0x548da0;
/// `Dispute::remove_dispute` (thiscall: person).
const FN_DISPUTE_REMOVE_DISPUTE: usize = 0x56a090;
/// `STAFF_CONTRACT::get_contract_satisfaction` (thiscall: person; 1 to 3).
const FN_STAFF_CONTRACT_GET_CONTRACT_SATISFACTION: usize = 0x866620;
/// `CONTRACT_MANAGER::post_wants_new_contract_news` (thiscall: contract).
const FN_CONTRACT_MANAGER_POST_WANTS_NEW_CONTRACT_NEWS: usize = 0x4d9f80;
/// `club_reputation_for_staff` (cdecl: club, person).
const FN_CLUB_REPUTATION_FOR_STAFF: usize = 0x545e90;
/// `staff_count_disliked_staff_at_club` (cdecl: person, club).
const FN_STAFF_COUNT_DISLIKED_STAFF_AT_CLUB: usize = 0x547c90;
/// `staff_count_favoured_staff_at_club` (cdecl: person, club).
const FN_STAFF_COUNT_FAVOURED_STAFF_AT_CLUB: usize = 0x547ce0;
/// `Media::queue_wants_first_team_story` (thiscall: player, reply round).
const FN_MEDIA_QUEUE_WANTS_FIRST_TEAM_STORY: usize = 0x75a640;
/// `TRAINING_MANAGER::has_fast_position_training` (thiscall: person).
const FN_TRAINING_MANAGER_HAS_FAST_POSITION_TRAINING: usize = 0x8b4db0;
/// `PlayerStats::improve_language` (thiscall: person, language, raise, sort).
const FN_PLAYERSTATS_IMPROVE_LANGUAGE: usize = 0x7c6ca0;

unsafe fn sd_pick_favoured_team_mate(p: usize, club: usize) -> usize {
    let f: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_STAFF_PICK_FAVOURED_TEAM_MATE);
    f(p, club)
}

unsafe fn sd_remove_dispute(p: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_DISPUTE_REMOVE_DISPUTE);
    f(read_usize(G_DISPUTE), p)
}

unsafe fn sd_contract_satisfaction(contract: usize, p: usize) -> u8 {
    let f: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_STAFF_CONTRACT_GET_CONTRACT_SATISFACTION);
    f(contract, p)
}

unsafe fn sd_post_wants_new_contract_news(contract: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_CONTRACT_MANAGER_POST_WANTS_NEW_CONTRACT_NEWS);
    f(read_usize(G_CONTRACT_MANAGER), contract)
}

unsafe fn sd_club_reputation_for_staff(club: usize, p: usize) -> u32 {
    let f: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_CLUB_REPUTATION_FOR_STAFF);
    f(club, p)
}

unsafe fn sd_count_disliked_at_club(p: usize, club: usize) -> u8 {
    let f: unsafe extern "C" fn(usize, usize) -> u8 = core::mem::transmute(FN_STAFF_COUNT_DISLIKED_STAFF_AT_CLUB);
    f(p, club)
}

unsafe fn sd_count_favoured_at_club(p: usize, club: usize) -> u8 {
    let f: unsafe extern "C" fn(usize, usize) -> u8 = core::mem::transmute(FN_STAFF_COUNT_FAVOURED_STAFF_AT_CLUB);
    f(p, club)
}

/// The contracted player's day (0x5ce4b6..0x5d352d), piece by piece in the original's order.
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5ce4b6..0x5d352d.
pub(crate) unsafe fn staff_day_player(d: &mut StaffDay, run: &mut StaffDayRun) {
    if staff_day_unrest_odds(d, run) {
        staff_day_grievance_unrest(d);
        staff_day_unrest_roll(d);
    }
    staff_day_disputes_and_morale(d);
    let contract = d.contract;
    if contract != 0 {
        staff_day_contract_moods(d);
        if read_u32(contract + contract::GRIEVANCES) & 0x400000 != 0 && d.club == 0 {
            exe::staff_contract_clear_grievances(contract, 0x400000);
            staff_day_part_b(d);
            return;
        }
    }
    if d.club == 0 {
        staff_day_part_b(d);
        return;
    }
    let injury = exe::injury_get_player_injury(read_usize(G_INJURY), d.p);
    if injury != 0 && read_i16(exe::injury_get_player_injury(read_usize(G_INJURY), d.p) + 0x2c) > 30 {
        staff_day_part_b(d);
        return;
    }
    if contract == 0 {
        return;
    }
    if !staff_day_grievance_fades(d) && read_usize(d.p + staff::PLAYER) != 0 && staff_day_youngster_praise(d).is_none() {
        if read_usize(d.p + staff::PLAYER) != 0 && staff_day_international_praise(d).is_none() {
            staff_day_club_stories(d);
        }
    }
    staff_day_value_demand(d);
    match staff_day_squad_status(d) {
        None => {
            exe::staff_contract_clear_grievances(contract, 0x10);
            exe::staff_contract_clear_grievances(contract, 2);
        }
        Some(true) => {}
        Some(false) => {
            if !staff_day_wants_move(d) {
                staff_day_contract_talks(d);
            }
        }
    }
    staff_day_part_b(d)
}

/// 0x5d2392: the settling part needs both a contract and a club (without a club the caller runs the unattached decline).
unsafe fn staff_day_part_b(d: &StaffDay) {
    if d.contract != 0 && d.club != 0 {
        staff_day_settling(d);
    }
}

/// The locals of one person's day in `game_daily_staff_update`, as the original keeps them in its frame.
#[derive(Clone, Copy)]
pub(crate) struct StaffDay {
    /// The person (`ebp`, [esp+0x6c]).
    p: usize,
    /// The person's id times the state record size ([esp+0x28]).
    state_off: usize,
    /// The player sheet ([esp+0x40]).
    pd: usize,
    /// The club the person plays for ([esp+0x1c]): the contract's club, or the club job for jobs 5 and 12.
    club: usize,
    /// The contract with that club ([esp+0x68], `esi`).
    contract: usize,
    /// The favoured team mate (`staff_pick_favoured_team_mate`, [esp+0x5c]).
    mate: usize,
    /// The club job differs from the club ([esp+0x27]).
    away_from_job: bool,
    /// The odds of the day's unrest roll ([esp+0x20]; only its low 16 bits are ever read).
    odds: i32,
    /// The club job's linked B team ([esp+0x7c]).
    b_team: u32,
}

/// The routine's locals that outlive one person.
pub(crate) struct StaffDayRun {
    /// [esp+0x70]: the favoured team mate's bonus, a short the original never initialises and carries from one person
    /// to the next (defect of the original: the first one reads stack residue; the copy starts it at 0).
    slot70: u32,
}

/// The person's state record byte +0xa: the day's unrest counter [inference: named injury counter in the fields].
unsafe fn sd_unrest(d: &StaffDay) -> usize {
    read_usize(G_STAFF_STATE_TABLE) + d.state_off + staff_state::INJURY_COUNTER
}

/// Adds `n` to the unrest counter (8 bits, wrapping).
unsafe fn sd_add_unrest(d: &StaffDay, n: i32) {
    let a = sd_unrest(d);
    write_u8(a, read_u8(a).wrapping_add(n as u8));
}

/// One grievance's roll: random(morale + 200) below random(`odds`) adds random(5) to the unrest.
unsafe fn sd_grievance_roll(d: &StaffDay, odds: i32) {
    let a = exe::utils_random(read_i8(d.contract + contract::MORALE) as i32 + 200);
    if a < exe::utils_random(odds) {
        sd_add_unrest(d, exe::utils_random(5));
    }
}

/// The club's state record (`G_CLUB_STATE_TABLE`, 6 bytes a club): byte `k`, signed.
unsafe fn sd_club_state(club: usize, k: usize) -> i8 {
    read_i8(read_usize(G_CLUB_STATE_TABLE) + read_u32(club) as usize * 6 + k)
}

/// The grievances word of the contract.
unsafe fn sd_grievances(d: &StaffDay) -> u32 {
    read_u32(d.contract + contract::GRIEVANCES)
}

/// The start of a player's day (0x5ce4b6..0x5ce9f9): the favoured team mate, the fatigue and form unrest, the
/// youngster's squad status, and the odds of the day's unrest roll. Returns false when the day's unrest part is skipped
/// (the jump to 0x5cf391).
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5ce4b6..0x5ce9f9.
unsafe fn staff_day_unrest_odds(d: &mut StaffDay, run: &mut StaffDayRun) -> bool {
    use exe::utils_random as random;
    let p = d.p;
    let id = read_i32(p);
    d.mate = sd_pick_favoured_team_mate(p, d.club);
    d.away_from_job = read_usize(p + staff::CLUB_JOB) != d.club;
    if read_u8(sd_unrest(d)) >= 50 {
        return false;
    }
    if random(200) == 0 {
        let limit = -1000 - random(5000);
        if (exe::injury_get_fatigue(read_usize(G_INJURY), id as u32) as i32) < limit {
            let unrest = sd_unrest(d);
            let fatigue = exe::injury_get_fatigue(read_usize(G_INJURY), read_u32(p)) as f64;
            let n = random(ftol(read_f64(rdata::F64_20) - fatigue * read_f64(rdata::F64_0_02)));
            write_u8(unrest, read_u8(unrest).wrapping_add(n as u8));
        }
    }
    let contract = d.contract;
    if contract != 0 {
        if sd_grievances(d) & 0x100000 != 0 && random(read_i8(p + staff::PRESSURE) as i32 + 5) == 0 {
            sd_add_unrest(d, random(5));
        }
        if d.club != 0 && read_i8(p + staff::AGE) > 23 {
            let status = read_u8(contract + contract::SQUAD_STATUS);
            if status & 0xf0 == 0x60 || status & 0xf0 == 0x50 {
                if exe::human_manager_is_human_club(d.club, 0, 1, 0, 0) != 0 {
                    write_u8(contract + contract::SQUAD_STATUS, status & 0xf);
                } else {
                    write_u8(contract + contract::SQUAD_STATUS, read_u8(contract + contract::SQUAD_STATUS) & 0xf | 0x40);
                }
            }
        }
    }
    if read_u8(sd_unrest(d)) < 15 && random(20) == 0 && random(read_i8(p + staff::PROFESSIONALISM) as i32) == 0 {
        let form = exe::playerstats_get_recent_form(read_usize(G_PLAYER_STATS), d.pd) as i32;
        if form > random(20) + 35 {
            sd_add_unrest(d, random(20));
        }
    }
    let limit = ftol(read_i16(d.pd + player_sheet::CURRENT_ABILITY) as f64 * read_f64(rdata::F64_0_15)) as i8 as i32;
    if read_u8(sd_unrest(d)) as i32 >= limit {
        return false;
    }
    // The odds: by the gap between ability and unrest against the professionalism.
    let prof = read_i8(p + staff::PROFESSIONALISM) as i32;
    let loyalty = read_i8(p + staff::LOYALTY) as i32;
    let determination = read_i8(p + staff::DETERMINATION) as i32;
    let pressure = read_i8(p + staff::PRESSURE) as i32;
    let unsettled = contract != 0 && exe::staff_is_transfer_listed_or_requested(p, 0usize) == 0 && sd_grievances(d) & 0x30321 != 0;
    let gap = (read_i16(d.pd + player_sheet::CURRENT_ABILITY) as i32).wrapping_sub(read_u8(sd_unrest(d)) as i32);
    // The 16-bit sums of the original keep the gap's (and the quotient's) high half in the dword.
    let high = |x: i32| (x as u32 & 0xffff_0000) as i32;
    let low = |b: i32| b as i16 as u16 as i32;
    d.odds = if unsettled {
        if gap < prof + 80 {
            ftol((loyalty + prof * 2 + 10 + id % 20) as f64 * read_f64(rdata::F64_2_5))
        } else if gap > prof + 125 {
            (high(gap) | low(loyalty)).wrapping_add(id % 15).wrapping_add(high(id % 15) | low(prof)).wrapping_add(5)
        } else {
            let x = (prof * 2) as f64 + loyalty as f64 + (id % 20) as f64 + read_f32(rdata::F32_5) as f64;
            ftol(x + x)
        }
    } else if gap < prof + 80 {
        ftol((determination + prof + pressure + id % 20 + 10) as f64 * read_f64(rdata::F64_2_5))
    } else if gap > prof + 140 {
        (high(gap) | low(pressure))
            .wrapping_add(low(determination))
            .wrapping_add(id % 15)
            .wrapping_add(high(id / 15) | low(prof))
            .wrapping_add(5)
    } else {
        let x = (determination + prof + pressure + id % 20 + 5) as f64;
        ftol(x + x)
    };
    // The favoured team mate: a short the original never resets between people (`slot70`).
    if d.mate != 0 {
        let mut v = run.slot70 as i32;
        if contract != 0 && sd_grievances(d) & 0x20020 != 0 && read_u32(d.mate + staff::NATION) == read_u32(p + staff::NATION) {
            v = v.wrapping_add(loyalty);
        } else {
            v = v.wrapping_add(ftol(loyalty as f64 * read_f64(rdata::F64_0_25)));
            if (v as i16) < 1 {
                v = 1;
            }
        }
        run.slot70 = v as u32;
        if read_i8(p + staff::AGE) > 23 || read_i16(d.pd + player_sheet::CURRENT_REPUTATION) > 0x109a {
            v = random(v as i16 as i32);
            run.slot70 = v as u32;
        }
        d.odds = d.odds.wrapping_add(v);
    }
    if contract != 0 {
        if read_i16(d.pd + player_sheet::WORLD_REPUTATION) < 0xea6 && read_i8(p + staff::AGE) < 19 {
            let age = read_i8(p + staff::AGE);
            let mut k = ftol(pressure as f64 * read_f64(rdata::F64_0_25) + read_f64(rdata::F64_3)) as i8;
            if age < 17 {
                k = k.wrapping_sub(3);
            } else if age == 17 {
                k = k.wrapping_sub(1);
            }
            let club = d.club;
            if club != 0 && read_usize(club + club::NATION) != 0 {
                let nation = read_usize(club + club::NATION);
                let x = (read_i8(nation + nation::LEAGUE_STANDARD) as f64 * read_f64(rdata::F64_0_2) + read_f64(rdata::F64_0_6))
                    * read_i16(club + club::REPUTATION) as f64;
                // fcom: a NaN falls to the last step
                if x > read_f32(rdata::F32_7750) as f64 {
                    k = k.wrapping_sub(2);
                } else if x > read_f32(0x96b03c) as f64 {
                    k = k.wrapping_sub(1); // 5750
                } else if !(x >= read_f32(0x96b038) as f64) {
                    k = k.wrapping_add(1); // 3750
                }
            }
            if k < 0 {
                k = 0;
            }
            if read_i8(contract + contract::MATCHES_LEFT_OUT) > k {
                let mut n = 7i8.wrapping_sub(k);
                if n < 2 {
                    n = 2;
                }
                sd_add_unrest(d, random(n as i32));
            }
        }
        if sd_grievances(d) & 0x40000 != 0 {
            let pr = pressure as i16;
            if (d.odds as i16) > 25 - pr {
                d.odds = d.odds.wrapping_add(pr as i32 - 25);
                if (d.odds as i16) < 1 {
                    d.odds = 1;
                }
            }
        }
    }
    if read_i8(d.pd + player_sheet::GOALKEEPER) > 14 {
        d.odds = d.odds.wrapping_mul(2).wrapping_add(10);
    } else if read_i8(d.pd + player_sheet::DEFENDER) > 14 {
        d.odds = ftol(d.odds as i16 as f64 * read_f64(rdata::F64_1_25) + read_f64(rdata::F64_5));
    }
    true
}

/// The grievances' unrest (0x5ce9f9..0x5cf106): one day in random(morale) for a player with grievances, each grievance
/// bit rolls random(contract morale + 200) against its own odds and adds random(5) to the unrest; two bits (0x20,
/// 0x20000) roll against (loyalty + 40) × 5 + morale when the favoured team mate shares the nation.
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5ce9f9..0x5cf106.
unsafe fn staff_day_grievance_unrest(d: &StaffDay) {
    use exe::utils_random as random;
    let contract = d.contract;
    if contract == 0 || sd_grievances(d) == 0 || random(read_i8(d.pd + player_sheet::PLAYER_MORALE) as i32) != 0 {
        return;
    }
    let mate_compatriot = |d: &StaffDay| d.mate != 0 && read_u32(d.mate + staff::NATION) == read_u32(d.p + staff::NATION);
    // the mate's roll: random((loyalty + 40) * 5 + morale) against random(100)
    let mate_roll = |d: &StaffDay| {
        let a = random((read_i8(d.p + staff::LOYALTY) as i32 + 40) * 5 + read_i8(d.contract + contract::MORALE) as i32);
        if a < random(100) {
            sd_add_unrest(d, random(5));
        }
    };
    if exe::staff_is_transfer_listed_or_requested(d.p, 0usize) == 0 && sd_grievances(d) & 1 != 0 {
        sd_grievance_roll(d, 20);
    }
    for (bit, odds) in [(2u32, 75), (8, 300)] {
        if sd_grievances(d) & bit != 0 {
            sd_grievance_roll(d, odds);
        }
    }
    if sd_grievances(d) & 0x20 != 0 {
        if mate_compatriot(d) {
            mate_roll(d);
        } else {
            sd_grievance_roll(d, 200);
        }
    }
    for (bit, odds) in [
        (0x40u32, 100),
        (0x80, 40),
        (0x100, 50),
        (0x200, 100),
        (0x400, 20),
        (0x800, 20),
        (0x1000, 20),
        (0x2000, 400),
        (0x4000, 100),
        (0x8000, 50),
        (0x10000, 10),
    ] {
        if sd_grievances(d) & bit != 0 {
            sd_grievance_roll(d, odds);
        }
    }
    if sd_grievances(d) & 0x20000 != 0 {
        if mate_compatriot(d) {
            mate_roll(d);
        } else {
            sd_grievance_roll(d, 100);
        }
    }
    for (bit, odds) in [(0x40000u32, 75), (0x80000, 75), (0x200000, 20), (0x400000, 20)] {
        if sd_grievances(d) & bit != 0 {
            sd_grievance_roll(d, odds);
        }
    }
    if exe::staff_contract_get_flags(contract) & 8 != 0 {
        sd_grievance_roll(d, 20);
    }
    if exe::staff_contract_get_flags(contract) & 0x20 != 0 {
        sd_grievance_roll(d, 10);
    }
    if sd_grievances(d) & 0x1000000 != 0 {
        sd_grievance_roll(d, 20);
    }
    if sd_grievances(d) & 0x800000 != 0 && exe::transfer_manager_get_future_transfer(read_usize(G_TRANSFER_MANAGER), d.p, 0) == 0 {
        sd_grievance_roll(d, 100);
    }
}

/// The game's average match rating: total / count of `G_PLAYER_STATS` (+0x10 / +0x18), 6.8 while the count is 0.
unsafe fn sd_average_rating() -> f64 {
    let stats = read_usize(G_PLAYER_STATS);
    let count = read_f64(stats + 0x18);
    // fcomp + test 0x40: equal (or NaN) takes 6.8
    if count == 0.0 || count.is_nan() {
        read_f64(rdata::F64_6_8)
    } else {
        read_f64(stats + 0x10) / count
    }
}

/// A player stat of the season (`PlayerStats::get_stat`, default 6.5) as the original stores it, a float.
unsafe fn sd_stat(p: usize, category: u32, which: u32) -> f32 {
    exe::playerstats_get_stat(read_usize(G_PLAYER_STATS), read_usize(p + staff::PLAYER), category, which, 6.5) as f32
}

/// The day's unrest roll of a club player (0x5cf106..0x5cf391): one time in random(odds), for the job 11 [inference:
/// a first team player], the unrest grows by random(8); a chairman losing patience may dislike him (rating draw over
/// his patience + 10 while the other rating is under 6.65, then one in patience - 2 × interference + 45); and one day
/// in 14 a human club's player with unrest above random(11), a rating above 1 and the other rating under the game's
/// average + unrest × 0.0133 gets a criticising training report from the assistant manager
/// (`news_post_training_report`), when the assistant judges well and is able enough.
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5cf106..0x5cf391.
unsafe fn staff_day_unrest_roll(d: &StaffDay) {
    use exe::utils_random as random;
    let p = d.p;
    if read_u8(p + staff::JOB_FOR_CLUB) != 11 || random(d.odds as i16 as i32) != 0 {
        return;
    }
    sd_add_unrest(d, random(8));
    let rating = sd_stat(p, 6, 1);
    let other = sd_stat(p, 6, 0x11);
    let club = d.club;
    if club == 0 {
        return;
    }
    let chairman = read_usize(club + club::CHAIRMAN);
    if chairman != 0 && read_usize(chairman + staff::NON_PLAYER) != 0 {
        let np = read_usize(chairman + staff::NON_PLAYER);
        let patience = read_i8(np + non_player::PATIENCE) as i16;
        if random(ftol(rating as f64)) > patience.wrapping_add(10) as i32
            && !(other as f64 >= read_f64(0x96b030))
        // under 6.65 (or NaN)
        {
            let n = patience.wrapping_sub((read_i8(np + non_player::INTERFERENCE) as i16).wrapping_mul(2)).wrapping_add(45);
            if random(n as i32) == 0 {
                covered!("gra_2b chairman dislikes an unsettled player");
                exe::staff_add_disliked_staff(read_usize(club + club::CHAIRMAN), p, 1);
            }
        }
    }
    if exe::human_manager_is_human_club(club, 0, 1, 0, 0) == 0
        || exe::injury_get_player_injury_status(read_usize(G_INJURY), p) == 2
        || club != read_usize(p + staff::CLUB_JOB)
        || (read_i16(G_CURRENT_DATE) as i32 % 14).wrapping_add(read_i32(p)) != 0
        || exe::awol_is_player_awol(read_usize(G_AWOL), p, 0) != 0
        || exe::staff_is_transfer_listed_or_requested(p, 0usize) != 0
    {
        return;
    }
    let unrest = read_u8(sd_unrest(d)) as i32;
    if unrest <= random(11) || !(rating > read_f32(rdata::F32_1)) {
        return;
    }
    let bar = unrest as f64 * read_f64(rdata::F64_0_01333333333) + sd_average_rating();
    if !(bar > other as f64) {
        return;
    }
    let contract = d.contract;
    if contract == 0 || read_i8(contract + contract::HAPPINESS) <= 0 {
        return;
    }
    let pd = read_usize(p + staff::PLAYER);
    if pd == 0 || read_i16(pd + player_sheet::CURRENT_REPUTATION) <= 0xabe {
        return;
    }
    let assistant = read_usize(club + club::ASSISTANT_MANAGER);
    if assistant == 0 || read_usize(assistant + staff::NON_PLAYER) == 0 {
        return;
    }
    let anp = read_usize(assistant + staff::NON_PLAYER);
    if exe::non_player_attribute_rating(anp, read_i8(anp + non_player::JUDGEMENT)) as i32 <= random(25) {
        return;
    }
    let r = random(400);
    if read_i16(read_usize(read_usize(club + club::ASSISTANT_MANAGER) + staff::NON_PLAYER) + non_player::CURRENT_ABILITY) as i32 <= r {
        return;
    }
    let schedule = exe::training_manager_get_player_schedule(read_usize(G_TRAINING_MANAGER), p);
    if schedule == 0 || read_u8(schedule + 0x15) == 0 {
        return;
    }
    covered!("gra_2b training report on an unsettled player");
    let f: unsafe extern "C" fn(usize, usize, usize, u32) = core::mem::transmute(FN_NEWS_POST_TRAINING_REPORT);
    f(p, read_usize(club + club::ASSISTANT_MANAGER), club, 0);
}

/// `news_post_training_report` (cdecl: player, coach, club, praise).
const FN_NEWS_POST_TRAINING_REPORT: usize = 0x588cd0;
/// `dispute_post_news` (stdcall in the map: staff, club, outcome, reason a, reason b; the original also sets ecx).
const FN_DISPUTE_POST_NEWS_GRA: usize = 0x56a3b0;
/// `Media::post_player_praise_story` (thiscall: player, club, topic, fixture record or null).
const FN_MEDIA_POST_PLAYER_PRAISE_STORY: usize = 0x744240;

/// The player's disputes, the fading unrest, the transfer status and the morale towards the club (0x5cf391..0x5cf57c).
/// A dispute ends without a club or with another club than the dispute's; a settled player at the club (no contract
/// flags, no grievances, happy) ends it with the news (`dispute_post_news`, outcome 3). The unrest of a fit player falls
/// by random(m / 5 + 2), m = random(morale + 5) when above random(40). A listed player at an AI club without grievances
/// who no longer wants to leave loses the transfer status bit 8 (unless his squad status is 0x70). One day in ten the
/// player's morale moves halfway to 10, or 10 + random(10 - the club's league position) when that is under 10, +3 when
/// the club favours him, -3 when it dislikes him (`staff_adjust_morale`).
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5cf391..0x5cf57c.
unsafe fn staff_day_disputes_and_morale(d: &StaffDay) {
    use exe::utils_random as random;
    let p = d.p;
    let club = d.club;
    let contract = d.contract;
    if read_usize(G_DISPUTE) != 0 && exe::dispute_is_in_dispute(read_usize(G_DISPUTE), p, 0) != 0 {
        if club == 0 {
            sd_remove_dispute(p);
        } else if read_i32(club) != exe::dispute_get_dispute_club_id(read_usize(G_DISPUTE), p) {
            sd_remove_dispute(p);
        } else if contract != 0
            && exe::staff_contract_get_flags(contract) == 0
            && read_u32(contract + contract::GRIEVANCES) == 0
            && read_i8(contract + contract::MORALE) > 0
        {
            covered!("gra_2b dispute settled");
            sd_remove_dispute(p);
            // stdcall in the map; the original also loads ecx with the dispute object, kept here
            let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(FN_DISPUTE_POST_NEWS_GRA);
            f(read_usize(G_DISPUTE), p, club, 3, 0, 0);
        }
    }
    if read_u8(sd_unrest(d)) != 0 && exe::injury_get_player_injury_status(read_usize(G_INJURY), p) != 2 {
        let m = random(read_i8(d.pd + player_sheet::PLAYER_MORALE) as i32 + 5) as i8 as i32;
        if m > random(40) {
            let mut fall = random(m / 5 + 2) as u8;
            let unrest = sd_unrest(d);
            let cur = read_u8(unrest);
            if fall > cur {
                fall = cur;
            }
            write_u8(unrest, cur - fall);
        }
    }
    if contract != 0
        && club != 0
        && read_u8(contract + contract::TRANSFER_STATUS) & 8 != 0
        && read_u32(contract + contract::GRIEVANCES) == 0
        && exe::staff_contract_wants_to_leave(contract) == 0
        && exe::human_manager_is_human_club(club, 0, 1, 0, 0) == 0
        && read_u8(contract + contract::SQUAD_STATUS) & 0xf0 != 0x70
    {
        write_u8(contract + contract::TRANSFER_STATUS, read_u8(contract + contract::TRANSFER_STATUS) & 0xf7);
    }
    if random(10) != 0 {
        return;
    }
    let mut target: u8 = 10;
    if club != 0 && read_usize(club + club::DIVISION) != 0 {
        let comp = read_usize(read_usize(G_COMP_OBJECT_TABLE) + 4 * read_u32(read_usize(club + club::DIVISION)) as usize);
        if comp != 0 && read_usize(comp + 8) != 0 {
            let entry = exe::comp_stats_find_club_entry(read_usize(comp + 8), club);
            if entry != 0 && read_u8(entry + 4) < 10 {
                target = (random(10 - read_u8(entry + 4) as i32) as u8).wrapping_add(10);
            }
        }
    }
    if exe::club_favours_staff(p, club) != 0 {
        target = target.wrapping_add(3);
    } else if exe::club_dislikes_staff(p, club) != 0 {
        target = target.wrapping_sub(3);
    }
    let morale = read_i8(d.pd + player_sheet::PLAYER_MORALE);
    if morale != target as i8 {
        let half = (morale as i32 + target as i8 as i32) / 2;
        // `sub al, cl`: only the low byte of the half becomes the step
        let step = (half as u32 & 0xffff_ff00) | (half as u8).wrapping_sub(morale as u8) as u32;
        exe::staff_adjust_morale(p, step, 1);
    }
}

/// The contract flags and grievances that come and go (0x5cf57c..0x5cfa3a): the ambition and professionalism set a
/// threshold of the club's league standing (club state byte +4); flag 0x10 (wants a bigger club [inference]) is set
/// after loyalty + 90 days at a club below it, for a young squad status (0x10..0x30) with loyalty + professionalism
/// under 20, one time in professionalism + 5, and cleared when the club rises above it; flag 0x1000 fades one day in ten;
/// flag 0x200 fades when the matches left out beat random(by pressure) + 1; grievances 0x40000000 and 0x20000000 fade
/// one day in 7 by the club's standing, the ambition and the pressure (at once without a club); grievance 0x80000000
/// fades by the pressure, or one day in ten after more than two matches left out; a settled player gains 5 morale
/// (professional at a club without trouble, or loyal at a club standing above 2); flag 4 [inference: wants to play in
/// Europe] comes and goes by the club state byte +5.
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5cf57c..0x5cfa3a.
unsafe fn staff_day_contract_moods(d: &StaffDay) {
    use exe::utils_random as random;
    let p = d.p;
    let club = d.club;
    let contract = d.contract;
    let id = read_i32(p);
    let day = read_i16(G_CURRENT_DATE) as i32;
    let ambition = read_i8(p + staff::AMBITION);
    let prof = read_i8(p + staff::PROFESSIONALISM);
    let mut bar: i8 = if ambition == 20 {
        -2
    } else if ambition > 15 {
        -3
    } else if ambition == 1 {
        -8
    } else {
        (ambition >= 6) as i8 - 5
    };
    if prof == 20 {
        bar -= 3;
    } else if prof > 15 {
        bar -= 2;
    } else if prof > 10 {
        bar -= 1;
    }
    if exe::staff_contract_get_flags(contract) & 0x10 != 0 {
        if club == 0 || sd_club_state(club, 4) > bar {
            exe::staff_contract_clear_flags(contract, 0x10);
        }
    } else if club != 0 {
        let loyalty = read_i8(p + staff::LOYALTY) as i32;
        let days = exe::date_days_since(G_CURRENT_DATE, read_u32(p + staff::DATE_JOINED_CLUB), read_u32(p + staff::DATE_JOINED_CLUB + 4));
        let status = read_u8(contract + contract::SQUAD_STATUS) & 0xf0;
        if days > loyalty + 90
            && sd_club_state(club, 4) < bar
            && (status == 0x10 || status == 0x20 || status == 0x30)
            && loyalty + (prof as i32) < 20
            && random(prof as i32 + 5) == 0
        {
            exe::staff_contract_set_flags(contract, 0x10);
        }
    }
    if exe::staff_contract_get_flags(contract) & 0x1000 != 0 && random(10) == 0 {
        exe::staff_contract_clear_flags(contract, 0x1000);
    }
    let pressure = read_i8(p + staff::PRESSURE);
    let calm: i8 = if pressure > 15 {
        0
    } else if pressure > 10 {
        1
    } else if pressure > 5 {
        2
    } else if pressure <= 1 {
        5
    } else {
        3
    };
    if exe::staff_contract_get_flags(contract) & 0x200 != 0 {
        let left_out = read_i8(contract + contract::MATCHES_LEFT_OUT);
        let r = random(calm as i32).wrapping_add(1);
        if left_out as i16 > r as i16 {
            exe::staff_contract_clear_flags(contract, 0x200);
        }
    }
    let pressure = pressure as i32;
    // grievances 0x40000000 and 0x20000000: the second rolls against the pressure, the first against pressure + 4
    for (bit, extra) in [(0x4000_0000u32, 4), (0x2000_0000, 0)] {
        if read_u32(contract + contract::GRIEVANCES) & bit == 0 {
            continue;
        }
        if club == 0 {
            exe::staff_contract_clear_grievances(contract, bit);
            continue;
        }
        if day.wrapping_add(id) % 7 != 0 {
            continue;
        }
        let standing = sd_club_state(club, 4);
        let fade = if standing > 2 {
            random(pressure + extra) != 0
        } else {
            let a = random((ambition as i16).wrapping_add(if standing > 0 { 5 } else { 25 }) as i32);
            random(pressure + extra) > a
        };
        if fade {
            exe::staff_contract_clear_grievances(contract, bit);
        }
    }
    if read_u32(contract + contract::GRIEVANCES) & 0x8000_0000 != 0 {
        let fade = if random(pressure) != 0 && random(15) == 0 {
            true
        } else {
            read_i8(contract + contract::MATCHES_LEFT_OUT) > 2
                && (day % 10).wrapping_add(id) == 0
                && read_i8(contract + contract::HAPPINESS) > 0
                && random(pressure) != 0
        };
        if fade {
            exe::staff_contract_clear_grievances(contract, 0x8000_0000);
        }
    }
    if club != 0 {
        if read_u32(contract + contract::GRIEVANCES) != 0 {
            staff_day_european_flag(d);
            return;
        }
        if exe::staff_is_transfer_listed_or_requested(p, 0usize) == 0 && sd_club_state(club, 5) == 0 && prof > 14 {
            staff_day_add_morale(contract, 5);
        }
    }
    if read_u32(contract + contract::GRIEVANCES) == 0 && club != 0 && sd_club_state(club, 4) > 2 && read_i8(p + staff::LOYALTY) > 10 {
        staff_day_add_morale(contract, 5);
    }
    staff_day_european_flag(d);
}

/// Flag 4 of the contract (0x5cf969..0x5cfa3a): cleared without a club, below state +5 of 3, or when random(pressure)
/// beats random(state + 1); set one time in (pressure step + state +5 + 1) for an ambitious and professional (both above
/// 10) player at a club whose state +5 is above step + 7.
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5cf969..0x5cfa3a.
unsafe fn staff_day_european_flag(d: &StaffDay) {
    use exe::utils_random as random;
    let p = d.p;
    let club = d.club;
    let contract = d.contract;
    if exe::staff_contract_get_flags(contract) & 4 != 0 {
        if club == 0 {
            exe::staff_contract_clear_flags(contract, 4);
            return;
        }
        let state = sd_club_state(club, 5);
        if state < 3 {
            exe::staff_contract_clear_flags(contract, 4);
            return;
        }
        let a = random(state as i32 + 1);
        if random(read_i8(p + staff::PRESSURE) as i32) > a {
            exe::staff_contract_clear_flags(contract, 4);
        }
        return;
    }
    if club == 0 {
        return;
    }
    let pressure = read_i8(p + staff::PRESSURE);
    let step: i8 = if pressure == 20 {
        5
    } else if pressure > 15 {
        3
    } else if pressure > 10 {
        2
    } else {
        (pressure > 5) as i8
    };
    if read_i8(p + staff::AMBITION) > 10
        && read_i8(p + staff::PROFESSIONALISM) > 10
        && sd_club_state(club, 5) as i32 > step as i32 + 7
        && random(pressure as i32 + step as i32 + 1) == 0
    {
        exe::staff_contract_set_flags(contract, 4);
    }
}

/// The grievances of a player under contract at a club, fit enough, that fade or start (0x5cfa97..0x5d0041): an
/// unsettled player (reputation above 5750 with grievances, or transfer status bit 1) joins the clubs' shortlists one
/// day in 20; 0x80000 fades when random(pressure) beats random(100); 0x1000000 (wants a bigger wage [inference]) ends
/// without finances, within 30 days of the contract, below world reputation 5000, below ambition + 100 of ability or
/// past 32, and starts for a disloyal ambitious star attacker one day in 7 when 90% of the club's wage budget beats his
/// wage; 0x100000, 0x200000, 0x2000000 (with -5 morale while it lasts), 0x4000000 and the contract flags 0x40, 0x100
/// and 0x80 (-5 morale for a hot temper) fade by the ambition; a first team player out of the side, unhappy, ambitious,
/// impatient and famous may go to the media (`Media::create_player_media_story`, kind 0x23); flag 0x20 fades by the
/// sportsmanship; 0x8000000 costs 5 morale and fades by the loyalty. Returns true when grievance 0x40000 was there
/// (the routine then skips the media stories: 0x5d0e23), after costing 5 morale to the calm and fading by the pressure.
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5cfa97..0x5d0041.
unsafe fn staff_day_grievance_fades(d: &StaffDay) -> bool {
    use exe::utils_random as random;
    let p = d.p;
    let pd = d.pd;
    let club = d.club;
    let contract = d.contract;
    let g = |c: usize| read_u32(c + contract::GRIEVANCES);
    let ambition = read_i8(p + staff::AMBITION) as i32;
    let state = read_usize(G_STAFF_STATE_TABLE) + read_u32(p) as usize * staff_state::SIZE;
    if read_u8(state + staff_state::STATUS_FLAGS) & 0xf == 0
        && ((read_i16(pd + player_sheet::CURRENT_REPUTATION) > 0x1676 && g(contract) != 0)
            || read_u8(contract + contract::TRANSFER_STATUS) & 1 != 0)
        && random(20) == 0
    {
        covered!("gra_2b unsettled player shortlisted");
        let listed = read_u8(contract + contract::TRANSFER_STATUS) & 1 != 0;
        let manager = read_usize(read_usize(G_TRANSFER_MANAGER) + 0x856); // the shortlist manager
        exe::shortlist_manager_add_player_to_club_shortlists(manager, p, listed as u32);
    }
    if g(contract) & 0x80000 != 0 {
        let a = random(read_i8(p + staff::PRESSURE) as i32);
        if a > random(100) {
            exe::staff_contract_clear_grievances(contract, 0x80000);
        }
    }
    let finance = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
    let grievances = g(contract);
    if grievances & 0x1000000 != 0 {
        let end = finance == 0
            || exe::date_days_since(G_CURRENT_DATE, read_u32(contract + contract::DATE_STARTED), read_u32(contract + contract::DATE_STARTED + 4)) < 30
            || read_i16(pd + player_sheet::WORLD_REPUTATION) < 5000
            || (read_i16(pd + player_sheet::CURRENT_ABILITY) as i32) < ambition + 100
            || read_i8(p + staff::AGE) > 32;
        if end {
            exe::staff_contract_clear_grievances(contract, 0x1000000);
        }
    } else if read_i8(p + staff::LOYALTY) < 15
        && ambition > 15
        && (read_i8(pd + player_sheet::ATTACKER) == 20 || read_i8(pd + player_sheet::ATTACKING_MIDFIELDER) == 20)
        && grievances & 1 == 0
        && exe::staff_is_transfer_listed_or_requested(p, club) == 0
        && read_i16(pd + player_sheet::CURRENT_ABILITY) as i32 > ambition + 105
        && read_i16(pd + player_sheet::WORLD_REPUTATION) > 0x1482
        && (read_i32(p) % 7).wrapping_add(read_i16(G_CURRENT_DATE) as i32) == 0
        && random(read_i8(p + staff::PROFESSIONALISM) as i32 + read_i8(p + staff::LOYALTY) as i32 + 10) == 0
        && ftol(read_i32(finance + 0x10) as f64 * read_f64(rdata::F64_0_9)) > read_i32(contract + contract::WAGE)
    {
        covered!("gra_2b star wants a bigger wage");
        exe::staff_contract_apply_event_flags(contract, 0x1000000, 0);
    }
    if g(contract) & 0x100000 != 0 {
        let fade = if read_i8(p + staff::AGE) < 24 {
            read_i8(contract + contract::UNHAPPY_FLAG) as i32 > 21 - read_i8(p + staff::PRESSURE) as i32 && {
                let a = random(ambition);
                a > random(20)
            }
        } else {
            let r = random(4);
            read_u8(contract + contract::MATCHES_LEFT_OUT) as i32 >= r && {
                let a = random(ambition);
                a > random(100)
            }
        };
        if fade {
            exe::staff_contract_clear_grievances(contract, 0x100000);
        }
    }
    if g(contract) & 0x200000 != 0 {
        let a = random(ambition);
        if a > random(50) {
            exe::staff_contract_clear_grievances(contract, 0x200000);
        }
    }
    if g(contract) & 0x2000000 != 0 {
        if random(ambition + 15) != 0 && read_i8(contract + contract::UNHAPPY_FLAG) <= 1 {
            staff_day_add_morale(contract, -5);
        } else {
            exe::staff_contract_clear_grievances(contract, 0x2000000);
            if exe::staff_contract_get_flags(contract) & 0x40 != 0 {
                exe::staff_contract_clear_flags(contract, 0x40);
            }
        }
    }
    if exe::staff_contract_get_flags(contract) & 0x40 != 0
        && (random(ambition + 5) == 0 || read_i8(contract + contract::UNHAPPY_FLAG) > 0)
    {
        exe::staff_contract_clear_flags(contract, 0x40);
    }
    if exe::staff_contract_get_flags(contract) & 0x100 != 0 && random(ambition + 5) == 0 {
        exe::staff_contract_clear_flags(contract, 0x100);
    }
    if exe::staff_contract_get_flags(contract) & 0x80 != 0 {
        if random(ambition + 5) == 0 {
            exe::staff_contract_clear_flags(contract, 0x80);
        } else if read_i8(p + staff::TEMPERAMENT) < 6 {
            staff_day_add_morale(contract, -5);
        }
    }
    if g(contract) & 0x4000000 != 0
        && ((read_i16(G_CURRENT_DATE) as i32).wrapping_add(read_i32(p)) % 10 == 0 || random(ambition) == 0)
    {
        exe::staff_contract_clear_grievances(contract, 0x4000000);
    }
    if read_u8(p + staff::JOB_FOR_CLUB) == 11
        && g(contract) & 0x10 != 0
        && exe::staff_is_transfer_listed_or_requested(p, club) == 0
        && !d.away_from_job
        && read_u8(contract + contract::MATCHES_LEFT_OUT) == 0
        && read_i8(contract + contract::UNHAPPY_FLAG) > 1
    {
        let happiness = read_i8(contract + contract::HAPPINESS) as i32;
        let r = random(100);
        if happiness < (10 - read_i8(p + staff::LOYALTY) as i32) * 2 - r
            && ambition > random(10)
            && (read_i8(p + staff::PROFESSIONALISM) as i32) < random(5) + 5
            && (read_i8(p + staff::TEMPERAMENT) as i32) < random(5) + 5
            && {
                let r = random(3000) + 1500;
                read_i16(read_usize(p + staff::PLAYER) + player_sheet::CURRENT_REPUTATION) as i32 > r
            }
            && read_i8(p + staff::AGE) > 19
            && (read_i16(G_CURRENT_DATE) as i32).wrapping_add(read_i32(p)).wrapping_add(8) % 14 == 0
            && random(read_i8(p + staff::PROFESSIONALISM) as i32) == 0
        {
            covered!("gra_2b player goes to the media");
            exe::media_create_player_media_story(read_usize(G_MEDIA), p, club, 0x23, 0, 0);
        }
    }
    if exe::staff_contract_get_flags(contract) & 0x20 != 0 {
        let a = random(read_i8(p + staff::SPORTSMANSHIP) as i32);
        if a >= random(50) {
            exe::staff_contract_clear_flags(contract, 0x20);
        }
    }
    if g(contract) & 0x8000000 != 0 {
        staff_day_add_morale(contract, -5);
        let a = random(read_i8(p + staff::LOYALTY) as i32);
        if a >= random(75) {
            exe::staff_contract_clear_grievances(contract, 0x8000000);
        }
    }
    if g(contract) & 0x40000 != 0 {
        if read_i8(p + staff::PRESSURE) < 6 {
            staff_day_add_morale(contract, -5);
        }
        let a = random(read_i8(p + staff::PRESSURE) as i32);
        if a >= random(50) {
            exe::staff_contract_clear_grievances(contract, 0x40000);
        }
        return true;
    }
    false
}

/// `PlayerStats::get_last_match_rating` (thiscall, the player sheet; the rating on x87, 6.5 without one).
const FN_PLAYERSTATS_GET_LAST_MATCH_RATING: usize = 0x7c1a60;
/// `news_post_player_thanks_manager` (cdecl: player, club).
const FN_NEWS_POST_PLAYER_THANKS_MANAGER: usize = 0x589300;

/// The player's last match rating (`PlayerStats::get_last_match_rating`).
unsafe fn sd_last_rating(p: usize) -> f32 {
    let f: unsafe extern "thiscall" fn(usize, usize) -> f32 = core::mem::transmute(FN_PLAYERSTATS_GET_LAST_MATCH_RATING);
    f(read_usize(G_PLAYER_STATS), read_usize(p + staff::PLAYER))
}

/// A media story praising the player (`Media::post_player_praise_story`, no fixture record).
unsafe fn sd_praise_story(p: usize, club: usize, topic: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, usize) = core::mem::transmute(FN_MEDIA_POST_PLAYER_PRAISE_STORY);
    f(read_usize(G_MEDIA), p, club, topic, 0);
}

/// The 8-byte date `G_CURRENT_DATE` + `days` (`Date::add_days`), as its day and year.
unsafe fn sd_date_in(days: u32) -> (i16, u16) {
    let mut out = [0u8; 8];
    let r = exe::date_add_days(G_CURRENT_DATE, out.as_mut_ptr() as usize, days);
    (read_i16(r), read_u16(r + 2))
}

/// The first scheduled date of a competition (`Comp::get_first_schedule_date`), as its day and year.
unsafe fn sd_first_date(comp: usize) -> (i16, u16) {
    let mut out = [0u8; 8];
    let r = exe::comp_get_first_schedule_date(comp, out.as_mut_ptr() as usize);
    (read_i16(r), read_u16(r + 2))
}

/// The youngster's praise (0x5d0041..0x5d02b5): a player under 22 without caps at a club in season with a manager,
/// fit, happy above random(40) - 20, at a club of reputation above 6000 + random(2000) or judged by the manager above
/// 130 + random(20) (`match_man_judge_player_ability`, rating (reputation / 2 + potential × 25)), one day in 14: with
/// under 100 games, a last rating of 7 or more, a season rating above 3 + random(5), form × 0.2 above the game's
/// average + 0.7 and the other rating above average + random(6) / 10, at a club standing above -2, gets a praise story
/// (topic 0x2b). Returns None when the youngster does not qualify (the routine goes on to the national praise).
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d0041..0x5d02b5.
unsafe fn staff_day_youngster_praise(d: &StaffDay) -> Option<()> {
    use exe::utils_random as random;
    let p = d.p;
    let club = d.club;
    let contract = d.contract;
    if club == 0
        || read_usize(club + club::MANAGER) == 0
        || d.away_from_job
        || read_u8(p + staff::INT_APPS) != 0
        || read_i8(p + staff::AGE) >= 22
        || exe::club_is_in_season(club, G_CURRENT_DATE) == 0
        || exe::injury_get_player_injury_status(read_usize(G_INJURY), p) != 0
    {
        return None;
    }
    let happiness = read_i8(contract + contract::HAPPINESS) as i32;
    if happiness <= random(40) - 20 {
        return None;
    }
    let r = random(2000);
    if read_i16(club + club::REPUTATION) as i32 <= r + 6000 {
        let pd = read_usize(p + staff::PLAYER);
        let half = read_i16(pd + player_sheet::CURRENT_REPUTATION) as i32 / 2;
        // the 16-bit product, added to the half in 32 bits; the callee reads the short
        let rating = half.wrapping_add((read_i16(pd + player_sheet::POTENTIAL_ABILITY)).wrapping_mul(25) as i32);
        let judged = exe::match_man_judge_player_ability(read_usize(club + club::MANAGER), p, club, 1, rating as i16) as i32;
        if judged <= random(20) + 130 {
            return None;
        }
    }
    let day = read_i16(G_CURRENT_DATE) as i32;
    if day.wrapping_add(read_i32(p)).wrapping_add(5) % 14 != 9 {
        return None;
    }
    let average = sd_average_rating() as f32;
    let apps = exe::player_stats_get_apps_for_club(read_usize(G_PLAYER_STATS), p, club) as i16 as f32;
    let season = sd_stat(p, 6, 1);
    let other = sd_stat(p, 6, 0x11);
    let form = (exe::playerstats_get_recent_form(read_usize(G_PLAYER_STATS), read_usize(p + staff::PLAYER)) as f64 * read_f64(rdata::F64_0_2)) as f32;
    let last = sd_last_rating(p);
    // the x87 tests: "under" holds for a NaN too, so each exit is the plain opposite
    if apps >= read_f32(rdata::F32_100) || !(last as f64 >= read_f64(rdata::F64_7)) {
        return Some(());
    }
    if (random(5) + 3) as f64 >= season as f64 {
        return Some(());
    }
    if (average as f64 + read_f64(rdata::F64_0_7)) >= form as f64 {
        return Some(());
    }
    let r = random(6);
    if (r as f64 * read_f64(rdata::F64_0_1) + average as f64) >= other as f64 {
        return Some(());
    }
    if sd_club_state(club, 4) <= -2 {
        return Some(());
    }
    covered!("gra_2b youngster praised");
    sd_praise_story(p, club, 0x2b);
    Some(())
}

/// The international's praise (0x5d02b5..0x5d0601): one day in 30 a happy international (happiness above random(40)),
/// fit, with fewer caps than 10 + random(10) or a world reputation under 5000 + random(2000), whose national team's
/// competition is a finals tournament (`NationalTeams::is_finals_comp`) starting between 10 and 100 days from today,
/// with a reputation above 5000 + random(3500), gets a praise story with the national team (topic 0x33) when his
/// season ratings beat 2 + random(4) and 5 + random(10) and both other ratings beat the average + 0.5 + random(6) / 10.
/// Returns None when he does not qualify (the club stories follow).
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d02b5..0x5d0601.
unsafe fn staff_day_international_praise(d: &StaffDay) -> Option<()> {
    use exe::utils_random as random;
    let p = d.p;
    if d.away_from_job || (read_i16(G_CURRENT_DATE) as i32).wrapping_add(read_i32(p)).wrapping_add(5) % 30 != 13 {
        return None;
    }
    let happiness = read_i8(d.contract + contract::HAPPINESS) as i32;
    if happiness <= random(40)
        || read_usize(p + staff::NATIONAL_JOB) == 0
        || exe::injury_get_player_injury_status(read_usize(G_INJURY), p) == 2
    {
        return None;
    }
    let r = random(10) + 10;
    if read_u8(p + staff::INT_APPS) as i32 >= r {
        let r = random(2000) + 5000;
        if read_i16(read_usize(p + staff::PLAYER) + player_sheet::WORLD_REPUTATION) as i32 >= r {
            return None;
        }
    }
    let teams = read_usize(G_NATIONAL_TEAMS);
    let comp = exe::nationalteams_get_current_competition(teams, read_usize(read_usize(p + staff::NATIONAL_JOB) + club::NATION));
    if exe::national_teams_is_finals_comp(read_usize(G_NATIONAL_TEAMS), comp) == 0 {
        return None;
    }
    let r = random(3500) + 5000;
    if read_i16(read_usize(p + staff::PLAYER) + player_sheet::CURRENT_REPUTATION) <= r as i16 {
        return None;
    }
    let comp_object = |comp: usize| read_usize(read_usize(G_COMP_OBJECT_TABLE) + 4 * read_u32(comp) as usize);
    // the finals start after 10 days...
    let (day, year) = sd_date_in(10);
    let (first_day, first_year) = sd_first_date(comp_object(comp));
    if !(year < first_year || (year == first_year && day < first_day)) {
        return None;
    }
    // ...and within 100
    let (day, year) = sd_date_in(100);
    let (first_day, first_year) = sd_first_date(comp_object(comp));
    if !(year > first_year || (year == first_year && day > first_day)) {
        return None;
    }
    let average = sd_average_rating() as f32;
    let season = sd_stat(p, 6, 1);
    let season5 = sd_stat(p, 5, 1);
    let other = sd_stat(p, 6, 0x11);
    let other5 = sd_stat(p, 5, 0x11);
    if (random(4) + 2) as f64 >= season5 as f64 || (random(10) + 5) as f64 >= season as f64 {
        return Some(());
    }
    for rating in [other, other5] {
        let r = random(6);
        if (r as f64 * read_f64(rdata::F64_0_1) + average as f64 + read_f64(rdata::F64_0_5)) >= rating as f64 {
            return Some(());
        }
    }
    covered!("gra_2b international praised");
    sd_praise_story(p, read_usize(p + staff::NATIONAL_JOB), 0x33);
    Some(())
}

/// The ratings the club stories read (0x5d06b7..0x5d0838), as the original keeps them (floats).
struct ClubStoryRatings {
    /// The game's average rating ([esp+0x20]).
    average: f32,
    /// The season rating (stat 6/1, [esp+0x2c]).
    season: f32,
    /// The other season rating plus the club's standing bonus ([esp+0x28]).
    other: f32,
    /// The recent form × 0.2 ([esp+0x58]).
    form: f32,
    /// Stat 6/2 over the season rating (at least 1), [esp+0x14].
    ratio: f32,
    /// The last match rating ([esp+0x44]).
    last: f32,
}

/// The club's stories about a player in the side (0x5d0601..0x5d0e27): one day in 5, for a happy (above 45) player at
/// his club in season with a manager, fit and left out more than once, when random(professionalism / 2) is 0 or the
/// media already have a story on him. A first team player with good ratings at a club standing below 0 whose form
/// trails the average may criticise the club (`staff_day_club_criticism`); otherwise a player in form (last rating 7 or
/// more) may thank the manager who favours him (`news_post_player_thanks_manager`) or be praised (topic 0x2b).
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d0601..0x5d0e27.
unsafe fn staff_day_club_stories(d: &StaffDay) {
    use exe::utils_random as random;
    let p = d.p;
    let club = d.club;
    let contract = d.contract;
    if read_usize(p + staff::PLAYER) == 0 || d.away_from_job {
        return;
    }
    if club == 0
        || read_usize(club + club::MANAGER) == 0
        || read_i8(contract + contract::HAPPINESS) <= 45
        || exe::injury_get_player_injury_status(read_usize(G_INJURY), p) == 2
        || read_i8(contract + contract::MATCHES_LEFT_OUT) <= 1
        || (read_i16(G_CURRENT_DATE) as i32).wrapping_add(read_i32(p)) % 5 != 3
        || exe::club_is_in_season(club, G_CURRENT_DATE) == 0
    {
        return;
    }
    if random(read_i8(p + staff::PROFESSIONALISM) as i32 / 2) != 0 && exe::media_has_story_for_player(read_usize(G_MEDIA), p) == 0 {
        return;
    }
    let average = sd_average_rating() as f32;
    let season = sd_stat(p, 6, 1);
    let other = sd_stat(p, 6, 0x11);
    let form = (exe::playerstats_get_recent_form(read_usize(G_PLAYER_STATS), read_usize(p + staff::PLAYER)) as f64 * read_f64(rdata::F64_0_2)) as f32;
    let floor = if season as f64 > read_f64(rdata::F64_1) { season as f64 } else { 1.0 };
    let ratio = (exe::playerstats_get_stat(read_usize(G_PLAYER_STATS), read_usize(p + staff::PLAYER), 6, 2, 6.5) / floor) as f32;
    let s62 = sd_stat(p, 6, 2);
    let s65 = sd_stat(p, 6, 5);
    let last = sd_last_rating(p);
    let standing = sd_club_state(club, 4);
    let mut bonus = standing as f64 * read_f64(rdata::F64_0_025);
    if bonus > read_f64(rdata::F64_0_2) {
        bonus = read_f32(0x96b028) as f64; // 0.2
    } else if !(bonus >= read_f64(rdata::F64_NEG_0_2)) {
        bonus = read_f32(0x96b024) as f64; // -0.2
    }
    let r = ClubStoryRatings { average, season, other: (bonus + other as f64) as f32, form, ratio, last };
    let avg = average as f64;
    if read_u8(p + staff::JOB_FOR_CLUB) == 11
        && season > read_f32(rdata::F32_1)
        && avg - read_f64(rdata::F64_0_35) > bonus + form as f64
        && !(s62 >= read_f32(rdata::F32_10))
        && !(s65 >= read_f32(rdata::F32_10))
        && !(s65 as f64 + s62 as f64 >= read_f32(rdata::F32_15) as f64)
        && standing < 0
    {
        staff_day_club_criticism(d, &r);
        return;
    }
    staff_day_club_praise(d, &r, bonus);
}

/// The criticism half of the club stories (0x5d08bb..0x5d0b42): a player whose ratings trail tells the media; with a
/// future transfer he goes (kind 0xc) when the average less random(5) / 10 beats his form; a recent signing (under 180
/// days less pressure) mostly (9 in 10) complains of the club (kind 2); with form and other rating apart from the
/// average by 0.3, an unsettled one (random(unrest + 20) above random(50)) says more (kind 0xd); else kind 0.
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d08bb..0x5d0b42.
unsafe fn staff_day_club_criticism(d: &StaffDay, r: &ClubStoryRatings) {
    use exe::utils_random as random;
    let p = d.p;
    let club = d.club;
    let avg = r.average as f64;
    let season = r.season as f64;
    let other = r.other as f64;
    let form = r.form as f64;
    let speaks = 'speaks: {
        let n = random(5);
        if n as f64 * read_f64(rdata::F64_0_1) + (avg - read_f64(rdata::F64_0_3)) > other && !((random(10) + 5) as f64 >= season) {
            break 'speaks true;
        }
        if avg + read_f64(rdata::F64_0_1) > other
            && avg - read_f64(rdata::F64_0_7) > form
            && !((random(10) + 5) as f64 >= season)
        {
            break 'speaks true;
        }
        let low = if !(form >= other) { form } else { other };
        let gap = (read_i16(d.pd + player_sheet::CURRENT_REPUTATION) as i32 - read_i16(club + club::REPUTATION) as i32) as f64;
        gap * read_f64(0x96b018) + avg + read_f64(rdata::F64_0_1) > low // 1/7000
            && avg + read_f64(rdata::F64_0_35) > other
            && !((random(15) + 5) as f64 >= season)
            && !(r.last as f64 > read_f64(rdata::F64_6))
    };
    if !speaks {
        return;
    }
    let story = |kind: u32| exe::media_create_player_media_story(read_usize(G_MEDIA), p, club, kind, 0, 0);
    if exe::transfer_manager_get_future_transfer(read_usize(G_TRANSFER_MANAGER), p, 0) != 0 {
        let n = random(5);
        if avg - n as f64 * read_f64(rdata::F64_0_1) > form {
            covered!("gra_2b leaving player criticises the club");
            story(0xc);
            return;
        }
    }
    let days = exe::date_days_since(G_CURRENT_DATE, read_u32(p + staff::DATE_JOINED_CLUB), read_u32(p + staff::DATE_JOINED_CLUB + 4));
    if days < 180 - read_i8(p + staff::PRESSURE) as i32 && random(10) != 0 {
        covered!("gra_2b new signing criticises the club");
        story(2);
        return;
    }
    if avg + read_f64(rdata::F64_0_3) > form && other > form + read_f64(rdata::F64_0_3) {
        let a = random(read_u8(sd_unrest(d)) as i32 + 20);
        if a > random(50) {
            covered!("gra_2b unsettled player criticises the club");
            story(0xd);
            return;
        }
    }
    covered!("gra_2b player criticises the club");
    story(0);
}

/// The praise half of the club stories (0x5d0b42..0x5d0e27): a player whose form beats the average + 0.35 less the
/// standing bonus, with a last rating of 7 or more. A season rating above 10 one time in 10 with the other rating or the
/// form well above the average thanks the manager when the manager favours him (one time in (25 - loyalty) × 4);
/// otherwise a chain of draws against the ratings may praise him (topic 0x2b).
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d0b42..0x5d0e27.
unsafe fn staff_day_club_praise(d: &StaffDay, r: &ClubStoryRatings, bonus: f64) {
    use exe::utils_random as random;
    let p = d.p;
    let club = d.club;
    let avg = r.average as f64;
    let season = r.season as f64;
    let other = r.other as f64;
    let form = r.form as f64;
    if avg + read_f64(rdata::F64_0_35) - bonus >= form || !(r.last as f64 >= read_f64(rdata::F64_7)) {
        return;
    }
    if r.season > read_f32(rdata::F32_10) && random(10) == 0 {
        if !(avg + read_f64(rdata::F64_0_4) >= other) || !(avg + read_f64(rdata::F64_1_1) >= form) {
            if exe::staff_is_favoured_by(read_usize(club + club::MANAGER), p) != 0
                && random((25 - read_i8(p + staff::LOYALTY) as i32) * 4) == 0
            {
                covered!("gra_2b player thanks the manager");
                let f: unsafe extern "C" fn(usize, usize) = core::mem::transmute(FN_NEWS_POST_PLAYER_THANKS_MANAGER);
                f(p, club);
            }
            return;
        }
    }
    let praise = 'praise: {
        if !((random(5) + 3) as f64 >= season) && !(avg + read_f64(rdata::F64_1_6) >= other) {
            break 'praise true;
        }
        let ratio = r.ratio as f64;
        let n = random(4);
        if !(n as f64 * read_f64(rdata::F64_0_1) + read_f64(rdata::F64_0_55) >= ratio)
            && !(avg + read_f64(rdata::F64_0_1) >= form)
            && !((random(20) + 4) as f64 >= season)
        {
            break 'praise true;
        }
        if !((random(20) + 10) as f64 >= season) {
            let n = random(10);
            if !(n as f64 * read_f64(rdata::F64_0_1) + avg + read_f64(rdata::F64_0_8) >= other) {
                break 'praise true;
            }
        }
        if !((random(20) + 10) as f64 >= season) {
            let both = (form + r.other as f64) * read_f64(rdata::F64_0_5);
            let n = random(10);
            if !(n as f64 * read_f64(rdata::F64_0_1) + avg + read_f64(rdata::F64_0_8) >= both) {
                let a = random(read_i16(club + club::REPUTATION) as i32 * 2);
                if random(read_i16(read_usize(p + staff::PLAYER) + player_sheet::CURRENT_REPUTATION) as i32) > a {
                    break 'praise true;
                }
            }
        }
        if (random(20) + 10) as f64 >= season {
            break 'praise false;
        }
        let n = random(16);
        if n as f64 * read_f64(rdata::F64_0_1) + avg + read_f64(rdata::F64_0_3) >= other {
            break 'praise false;
        }
        let k = random(2);
        let state = read_usize(G_STAFF_STATE_TABLE) + read_u32(p) as usize * staff_state::SIZE;
        (read_u8(state + staff_state::STATUS_FLAGS) & 0xf) as i32 > k
    };
    if praise {
        covered!("gra_2b player praised by the club's media");
        sd_praise_story(p, club, 0x2b);
    }
}

/// `CONTRACT_MANAGER::post_seeks_assurances_news` (thiscall: the contract).
const FN_CONTRACT_MANAGER_POST_SEEKS_ASSURANCES_NEWS: usize = 0x4d9840;

/// The club of the contract (`contract::CLUB`, an id into `G_CLUB_TABLE`).
unsafe fn sd_contract_club(contract: usize) -> usize {
    read_usize(G_CLUB_TABLE) + read_u32(contract + contract::CLUB) as usize * 0x245
}

/// Days from the date at `from` (8 bytes) to today, counted as the original's inline (365 a year, no leap days).
unsafe fn sd_days_since_inline(from: usize) -> i32 {
    (read_u16(G_CURRENT_DATE_YEAR) as i32 - read_u16(from + 2) as i32) * 365 - read_i16(from) as i32 + read_i16(G_CURRENT_DATE) as i32
}

/// Today against the contract's end (`Date::days_since` from the end date): days left.
unsafe fn sd_days_to_expiry(contract: usize) -> i32 {
    exe::date_days_since(contract + contract::CONTRACT_EXPIRES, read_u32(G_CURRENT_DATE), read_u32(G_CURRENT_DATE_LEAP))
}

/// `Date::days_since` from a stored date (two dwords at `at`) to today.
unsafe fn sd_days_since(at: usize) -> i32 {
    exe::date_days_since(G_CURRENT_DATE, read_u32(at), read_u32(at + 4))
}

/// The day's value and wage demand (0x5d0e27..0x5d10e5): a player who wants to leave, has morale under -25 or the
/// contract flag 8 gets a value (`CONTRACT_MANAGER::calc_player_value`) scaled by an unrest score (grievances,
/// ambition, loyalty, the club he likes or dislikes, youth) and the squad status at a big club; flag 8 (wants a better
/// contract [inference]) comes for a temperamental player valued above 250,000 and above the scaled value, and goes by
/// the professionalism.
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d0e27..0x5d10e5.
unsafe fn staff_day_value_demand(d: &StaffDay) {
    use exe::utils_random as random;
    let p = d.p;
    let club = d.club;
    let contract = d.contract;
    if exe::staff_contract_wants_to_leave(contract) == 0
        && read_i8(contract + contract::MORALE) >= -25
        && exe::staff_contract_get_flags(contract) & 8 == 0
    {
        return;
    }
    let human = club != 0 && exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0;
    let value = exe::contract_manager_calc_player_value(read_usize(G_CONTRACT_MANAGER), p, club, 0, -1, human as u32);
    let g = read_u32(contract + contract::GRIEVANCES);
    let mut score: i8 = if g != 0 { 5 } else { 0 };
    if g & 0x101 != 0 {
        score += 5;
    }
    let ambition = read_i8(p + staff::AMBITION);
    if ambition == 20 {
        score += 2;
    } else if ambition > 15 {
        score += 1;
    }
    let loyalty = read_i8(p + staff::LOYALTY);
    if loyalty == 1 {
        score += 3;
    } else if loyalty < 5 {
        score += 1;
    } else if loyalty == 20 {
        score -= 3;
    } else if loyalty > 15 {
        score -= 1;
    }
    if exe::staff_favours_club(p, club) != 0 {
        score -= 2;
    } else if exe::staff_dislikes_club(p, club) != 0 {
        score += 2;
    }
    let age = read_i8(p + staff::AGE);
    if age < 20 {
        score -= 4;
    } else if age < 23 {
        score -= 2;
    }
    let factor = read_f32(if score > 10 {
        rdata::F32_0_8999999762
    } else if score > 8 {
        rdata::F32_0_9499999881
    } else if score > 5 {
        rdata::F32_1
    } else if score > 3 {
        rdata::F32_1_100000024
    } else if score > 1 {
        rdata::F32_1_25
    } else if score == 0 {
        rdata::F32_1_75
    } else if score < -2 {
        rdata::F32_2_5
    } else if score < 0 {
        rdata::F32_2
    } else {
        rdata::F32_1_5
    }) as f64;
    let v = value as f64;
    let base = ftol(if value < 500_000 {
        v * read_f64(rdata::F64_4)
    } else if value < 2_500_000 {
        let x = v - read_f32(rdata::F32_500000) as f64;
        x + x + read_f64(rdata::F64_2000000)
    } else {
        (v - read_f32(rdata::F32_2500000) as f64) * read_f64(rdata::F64_1_25) + read_f64(rdata::F64_6000000)
    });
    let mut demand = ftol(factor * base as f64);
    if club != 0 && read_i16(club + club::REPUTATION) > 0x1676 && read_i8(contract + contract::HAPPINESS) > 0 {
        let status = read_u8(contract + contract::SQUAD_STATUS) >> 4;
        let human = exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0;
        let k = match status {
            1 => rdata::F64_1_2,
            2 if human => rdata::F64_1_15,
            2 => rdata::F64_1_2,
            3 if human => rdata::F64_1_1,
            3 => rdata::F64_1_15,
            _ if human => rdata::F64_1_05,
            _ => rdata::F64_1_1,
        };
        demand = ftol(demand as f64 * read_f64(k));
    }
    let worth = read_i32(p + staff::VALUE);
    if exe::staff_contract_get_flags(contract) & 8 != 0 {
        if (worth < 250_000 || worth < demand) && random(read_i8(p + staff::PROFESSIONALISM) as i32) != 0 {
            exe::staff_contract_clear_flags(contract, 8);
        }
    } else if worth > 250_000 && worth > demand && random(read_i8(p + staff::TEMPERAMENT) as i32 + 5) == 0 {
        covered!("gra_2b player wants a better contract");
        exe::staff_contract_set_flags(contract, 8);
    }
}

/// The squad status and assurances (0x5d10e5..0x5d1448). Grievance 0x400000 ends without a club or at a club with a
/// wealth band of 0 or more. Nothing more for a player leaving on a Bosman, retiring, or in the jobs 6, 8, 9 and 12. A
/// player out for more than 14 days loses grievance 0x10 (the rest of the day's contract part is skipped). A player
/// without squad status at a club with a manager, not listed nor queued for a free transfer, seeks assurances when the
/// manager has been there long enough: (club reputation / player reputation)^6 (at most 6) × 50 + 10 (at most 120) +
/// id % 33 days; he loses 20 morale, and the human manager gets the news. A first team player whose squad status is
/// 0x70 [inference: not needed], over 21 (a goalkeeper over 24 + id % 4), one day in 20, loses 50 morale and complains
/// (grievance 0x10); a famous, ambitious, impatient one may then dislike the club or the manager. Returns false when
/// the routine goes on with the contract talks (0x5d1448), true when the day's contract part is over (0x5d2392).
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d10e5..0x5d1448.
unsafe fn staff_day_squad_status(d: &StaffDay) -> Option<bool> {
    use exe::utils_random as random;
    let p = d.p;
    let club = d.club;
    let contract = d.contract;
    if read_u32(contract + contract::GRIEVANCES) & 0x400000 != 0 {
        let end = club == 0 || {
            let finance = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
            finance != 0 && exe::finance_get_wealth_band(finance) as i8 >= 0
        };
        if end {
            exe::staff_contract_clear_grievances(contract, 0x400000);
        }
    }
    if read_u8(contract + contract::LEAVING_ON_BOSMAN) != 0 || exe::player_regen_get_retirement_status(read_usize(G_PLAYER_REGEN), p) != 0 {
        return None;
    }
    let job = read_u8(p + staff::JOB_FOR_CLUB);
    if job == 12 || job == 8 || job == 6 || job == 9 {
        return None;
    }
    if read_usize(G_INJURY) != 0
        && exe::injury_get_player_injury_status(read_usize(G_INJURY), p) == 2
        && exe::injury_get_player_injury(read_usize(G_INJURY), p) != 0
        && read_i16(exe::injury_get_player_injury(read_usize(G_INJURY), p) + 0x2c) > 14
    {
        exe::staff_contract_clear_grievances(contract, 0x10);
        return Some(true);
    }
    if read_u8(contract + contract::SQUAD_STATUS) & 0xf0 == 0
        && read_usize(club + club::MANAGER) != 0
        && exe::transfer_manager_is_queued_for_free_transfer(read_usize(G_TRANSFER_MANAGER), p) == 0
        && exe::staff_is_transfer_listed_or_requested(p, club) == 0
    {
        let reputation = read_i16(d.pd + player_sheet::CURRENT_REPUTATION);
        let denominator = if reputation < 1 { 1 } else { reputation as i32 };
        let mut x = game_pow(read_i16(club + club::REPUTATION) as f64 / denominator as f64, read_f64(rdata::F64_6));
        if x >= read_f64(rdata::F64_6) {
            x = read_f64(rdata::F64_6);
        }
        x = x * read_f64(rdata::F64_50) + read_f64(rdata::F64_10);
        if x >= read_f64(rdata::F64_120) {
            x = read_f64(rdata::F64_120);
        }
        let days = ftol((read_i32(p) % 33) as f64 + x);
        let mut out = [0u8; 8];
        let date = exe::date_add_days(read_usize(club + club::MANAGER) + staff::DATE_JOINED_CLUB, out.as_mut_ptr() as usize, days as u32);
        let (day, year) = (read_i16(date), read_u16(date + 2));
        let later = year > read_u16(G_CURRENT_DATE_YEAR) || (year == read_u16(G_CURRENT_DATE_YEAR) && day > read_i16(G_CURRENT_DATE));
        if !later {
            covered!("gra_2b player seeks assurances");
            staff_day_add_morale(contract, -20);
            if exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0 {
                let f: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_CONTRACT_MANAGER_POST_SEEKS_ASSURANCES_NEWS);
                f(read_usize(G_CONTRACT_MANAGER), contract);
            }
        }
        return Some(true);
    }
    let g = read_u32(contract + contract::GRIEVANCES);
    if exe::staff_is_transfer_listed_or_requested(p, 0usize) != 0
        || job != 11
        || read_u8(contract + contract::SQUAD_STATUS) & 0xf0 != 0x70
        || g & 0x10 != 0
        || g & 4 != 0
        || random(20) != 0
        || read_usize(p + staff::PLAYER) == 0
    {
        return Some(false);
    }
    let age = read_i8(p + staff::AGE);
    if age <= 21 {
        return Some(false);
    }
    if read_i8(read_usize(p + staff::PLAYER) + player_sheet::GOALKEEPER) == 20 && age <= (read_i32(p) % 4 + 24) as i8 {
        return Some(false);
    }
    covered!("gra_2b unwanted player complains");
    staff_day_add_morale(contract, -50);
    exe::staff_contract_apply_event_flags(contract, 0x10, 0);
    let pd = read_usize(p + staff::PLAYER);
    if pd != 0
        && read_i16(pd + player_sheet::CURRENT_REPUTATION) > 0x1482
        && read_i8(p + staff::AMBITION) > 5
        && read_i8(p + staff::TEMPERAMENT) < 15
        && read_i8(p + staff::SPORTSMANSHIP) < 15
        && read_i8(p + staff::PROFESSIONALISM) < 10
        && random(4) != 0
    {
        sd_dislike_club_or_manager(d);
    }
    Some(true)
}

/// One time in ten the player dislikes the club, else its manager (`staff_add_disliked_club` / `_staff`, forced).
unsafe fn sd_dislike_club_or_manager(d: &StaffDay) {
    if exe::utils_random(10) == 0 {
        covered!("gra_2b player dislikes the club");
        exe::staff_add_disliked_club(d.p, d.club, 1);
    } else {
        covered!("gra_2b player dislikes the manager");
        exe::staff_add_disliked_staff(d.p, read_usize(d.club + club::MANAGER), 1);
    }
}

/// The days at the club counted for a new contract or a move (0x5d1495..0x5d1530): days since the contract started
/// (at most 650) plus, for a capped first team international at a club under reputation 5500, (caps + 20) × 10.
unsafe fn sd_contract_days(p: usize, club: usize, mut days: i32) -> i32 {
    if read_usize(p + staff::NATIONAL_JOB) != 0
        && read_u8(p + staff::INT_APPS) != 0
        && read_u8(p + staff::JOB_FOR_NATION) == 11
        && read_i16(club + club::REPUTATION) < 0x157c
    {
        days = days.wrapping_add((read_u8(p + staff::INT_APPS) as i32 + 20) * 10);
    }
    days
}

/// The player's pull: reputation + 2 × (days - 25 × (morale + 2 × loyalty + 22)).
unsafe fn sd_pull(d: &StaffDay, days: i32) -> i32 {
    let c = read_i8(d.pd + player_sheet::PLAYER_MORALE) as i32 + read_i8(d.p + staff::LOYALTY) as i32 * 2 + 22;
    read_i16(d.pd + player_sheet::CURRENT_REPUTATION) as i32 + 2 * (days - 25 * c)
}

/// Wanting a move and settling (0x5d1448..0x5d16e8): one day in 50 a player without grievance 1 whose morale is low
/// (contract morale + 10 × (player morale - 10) under 25, or a first team international) and who has been 120 days at
/// the club (`sd_contract_days`) gets grievance 1 (wants a move, -50 morale, grievance 2 cleared) when his pull (at most
/// 9000) beats both the club's reputation and the reputation he sees in it (`club_reputation_for_staff`), under 31. With
/// grievance 1, one day in 50 (or every day past 31) the grievance ends with +50 morale when the reputation he sees
/// beats the pull (less 150 a year past 31) + 200 + random(400). Returns true when the day's contract part is over.
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d1448..0x5d16e8.
unsafe fn staff_day_wants_move(d: &StaffDay) -> bool {
    use exe::utils_random as random;
    let p = d.p;
    let club = d.club;
    let contract = d.contract;
    if read_u32(contract + contract::GRIEVANCES) & 1 == 0 {
        let low = read_i8(contract + contract::MORALE) as i32 + (read_i8(d.pd + player_sheet::PLAYER_MORALE) as i32 - 10) * 10;
        let eligible = low < 25 || (read_usize(p + staff::NATIONAL_JOB) != 0 && read_u8(p + staff::JOB_FOR_NATION) == 11);
        if eligible && random(50) == 0 {
            let mut days = sd_days_since_inline(contract + contract::DATE_STARTED);
            if days < 0 {
                return true;
            }
            if days > 650 {
                days = sd_contract_days(p, club, 650);
            } else if days >= 120 {
                days = sd_contract_days(p, club, days);
            }
            if days < 250 {
                return true;
            }
            let mut pull = sd_pull(d, days);
            if pull > 9000 {
                pull = 9000;
            }
            if pull <= read_i16(club + club::REPUTATION) as i32
                || pull <= sd_club_reputation_for_staff(club, p) as i16 as i32
                || read_i8(p + staff::AGE) >= 31
            {
                return true;
            }
            covered!("gra_2b player wants a move");
            staff_day_add_morale(contract, -50);
            exe::staff_contract_apply_event_flags(contract, 1, 0);
            exe::staff_contract_clear_grievances(contract, 2);
            return true;
        }
        if read_u32(contract + contract::GRIEVANCES) & 1 == 0 {
            return false;
        }
    }
    let age = read_i8(p + staff::AGE);
    if random(50) != 0 && age <= 31 {
        return false;
    }
    let mut days = sd_days_since_inline(contract + contract::DATE_STARTED);
    if days < 0 {
        days = 0;
    } else if days > 650 {
        days = 650;
    }
    let mut pull = sd_pull(d, sd_contract_days(p, club, days));
    if age > 31 {
        pull += (31 - age as i32) * 150;
    }
    let seen = sd_club_reputation_for_staff(club, p) as i16 as i32;
    if random(400) + pull + 200 >= seen {
        return true;
    }
    covered!("gra_2b player settles");
    staff_day_add_morale(contract, 50);
    exe::staff_contract_clear_grievances(contract, 1);
    true
}

/// `date` (day, year) falls before today.
unsafe fn sd_before_today((day, year): (i16, u16)) -> bool {
    let today_year = read_u16(G_CURRENT_DATE_YEAR);
    year < today_year || (year == today_year && day < read_i16(G_CURRENT_DATE))
}

/// The date at `this` (8 bytes) + `days` (`Date::add_days`), as its day and year.
unsafe fn sd_add_days(this: usize, days: i32) -> (i16, u16) {
    let mut out = [0u8; 8];
    let r = exe::date_add_days(this, out.as_mut_ptr() as usize, days as u32);
    (read_i16(r), read_u16(r + 2))
}

/// `CONTRACT_MANAGER::offer_contract` of the AI club for a new contract (no club, no output, slot -1, factor 1).
unsafe fn sd_offer_new_contract(p: usize, mode: bool) -> u32 {
    exe::contract_manager_offer_contract(read_usize(G_CONTRACT_MANAGER), p, 0, 0, -1, 1.0, mode as u32)
}

/// The contract talks (0x5d16e8..0x5d2392): grievance 2 (wants a new contract) ends within 60 days of the contract;
/// past loyalty × 2 + 5 days into the game and loyalty × 20 + 250 days into the contract, a settled player (no
/// grievance 1, not listed, squad status not 0x70, fit for 60 days, no transfer pending) one day in 50 weighs his
/// contract (`STAFF_CONTRACT::get_contract_satisfaction`) against the club's shortlist entry of him: the AI club offers a
/// new contract (`CONTRACT_MANAGER::offer_contract`) by the entry's status, else he asks for one (grievance 2) and may
/// go public (`CONTRACT_MANAGER::post_wants_new_contract_news`, disliking the club or manager); otherwise a loan player
/// near his contract's end wants to stay (grievance 2), and a veteran away from home may want to go back (grievance
/// 0x100), else the dressing room relations move his morale (`staff_day_relations`).
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d16e8..0x5d2392.
unsafe fn staff_day_contract_talks(d: &StaffDay) {
    use exe::utils_random as random;
    let p = d.p;
    let contract = d.contract;
    if read_u32(contract + contract::GRIEVANCES) & 2 != 0 && sd_days_since(contract + contract::DATE_STARTED) < 60 {
        exe::staff_contract_clear_grievances(contract, 2);
        return;
    }
    let loyalty = read_i8(p + staff::LOYALTY) as i16;
    let talks = sd_before_today(sd_add_days(G_GAME_START_DATE, loyalty.wrapping_mul(2).wrapping_add(5) as i32))
        && sd_before_today(sd_add_days(contract + contract::DATE_STARTED, loyalty.wrapping_mul(20).wrapping_add(250) as i32))
        && read_u32(contract + contract::GRIEVANCES) & 1 == 0
        && exe::staff_is_transfer_listed_or_requested(p, 0usize) == 0
        && read_u8(contract + contract::SQUAD_STATUS) & 0xf0 != 0x70
        && random(50) == 0
        && !(read_usize(G_INJURY) != 0
            && exe::injury_get_player_injury(read_usize(G_INJURY), p) != 0
            && read_i16(exe::injury_get_player_injury(read_usize(G_INJURY), p) + 0x2c) > 60)
        && exe::transfer_manager_get_future_transfer(read_usize(G_TRANSFER_MANAGER), p, 0) == 0
        && exe::transfer_manager_is_queued_for_free_transfer(read_usize(G_TRANSFER_MANAGER), p) == 0;
    if talks {
        staff_day_new_contract(d);
    } else if !staff_day_contract_running_out(d) && !staff_day_going_home(d) {
        staff_day_relations(d);
    }
}

/// The satisfaction talk (0x5d188f..0x5d1eef), see `staff_day_contract_talks`.
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d188f..0x5d1eef.
unsafe fn staff_day_new_contract(d: &StaffDay) {
    use exe::utils_random as random;
    let p = d.p;
    let club = d.club;
    let contract = d.contract;
    let shortlists = read_usize(read_usize(G_TRANSFER_MANAGER) + 0x856);
    let entry = exe::shortlist_manager_find_club_shortlist_entry(shortlists, club, p, 0, 0);
    let satisfaction = sd_contract_satisfaction(contract, p) as i8;
    let g = |c: usize| read_u32(c + contract::GRIEVANCES);
    let loyalty = read_i8(p + staff::LOYALTY);
    // the entry's status (bits 1..4 of +0x24) and whether the entry is of this player (+0xf)
    let status = |entry: usize| (read_u8(entry + 0x24) >> 1) & 0xf;
    let own = |entry: usize| read_u32(entry + 0xf) == read_u32(p);
    let ask_publicly = |temper: i8| {
        let pd = d.pd;
        if read_i16(pd + player_sheet::CURRENT_REPUTATION) > 0x1482
            && read_i8(p + staff::AMBITION) > 5
            && read_i8(p + staff::TEMPERAMENT) < temper
            && read_i8(p + staff::SPORTSMANSHIP) < 15
            && read_i8(p + staff::PROFESSIONALISM) < 10
            && random(14) == 0
        {
            sd_dislike_club_or_manager(d);
        }
    };
    match satisfaction {
        3 => {
            if entry == 0 {
                return;
            }
            if g(contract) & 2 != 0 {
                // one day in 8, with grievance 2 alone and no offer pending, he says so
                let mark = (read_u32(G_CURRENT_DATE).wrapping_add(read_u32(p)) & 7) != 0;
                if mark || g(contract) != 2 || exe::contract_manager_find_contract_offer(read_usize(G_CONTRACT_MANAGER), read_u32(p), read_u32(contract + contract::CLUB)) != 0 {
                    return;
                }
                covered!("gra_2b player wants a new contract publicly");
                sd_post_wants_new_contract_news(contract);
                ask_publicly(10);
                return;
            }
            if sd_days_since(contract + contract::DATE_STARTED) < (loyalty as i32 + 25) * 10
                || read_i8(contract + contract::MORALE) >= -25
                || sd_days_to_expiry(contract) >= 565
            {
                return;
            }
            let ambition = read_i8(p + staff::AMBITION);
            if ambition <= 10 || loyalty >= 10 || read_i8(contract + contract::HAPPINESS) <= 75 {
                return;
            }
            let pull = ((ambition as i16).wrapping_sub((loyalty as i16).wrapping_mul(2)))
                .wrapping_mul(25)
                .wrapping_add((read_i8(p + staff::PRESSURE) as i16).wrapping_mul(10))
                .wrapping_add(read_i16(d.pd + player_sheet::WORLD_REPUTATION));
            if pull <= 0x1964 {
                return;
            }
            if exe::human_manager_is_human_club(sd_contract_club(contract), 0, 1, 0, 0) == 0 {
                let s = status(entry);
                let offered = match s {
                    1 => sd_offer_new_contract(p, own(entry)),
                    2 | 3 => sd_offer_new_contract(p, false),
                    _ => return,
                };
                if offered != 0 {
                    return;
                }
            }
            sd_ask_for_contract(d);
        }
        2 => {
            if g(contract) & 2 != 0 {
                staff_day_add_morale(contract, 10);
                exe::staff_contract_clear_grievances(contract, 2);
            }
        }
        1 => {
            if entry == 0 {
                return;
            }
            if g(contract) & 2 != 0 {
                staff_day_contract_reminder(d, entry);
                return;
            }
            if sd_days_since(contract + contract::DATE_STARTED) < (loyalty as i32 + 25) * 10 {
                return;
            }
            if sd_days_to_expiry(contract) > 180
                && exe::contract_manager_get_deserved_squad_status(read_usize(G_CONTRACT_MANAGER), p, contract) == 7
            {
                staff_day_add_morale(contract, -5);
                return;
            }
            let age = read_i8(p + staff::AGE);
            let veteran = if read_i8(read_usize(p + staff::PLAYER) + player_sheet::GOALKEEPER) == 20 { age > 34 } else { age > 31 };
            if veteran {
                let floor = (read_i8(p + staff::PROFESSIONALISM) as i8).wrapping_add(loyalty).wrapping_neg();
                if read_i8(contract + contract::MORALE) > floor && sd_days_to_expiry(contract) > (loyalty as i32 + 30) * 5 {
                    return;
                }
            }
            if age < 21
                && read_i8(contract + contract::HAPPINESS) < (loyalty as u8).wrapping_shl(1).wrapping_add(15) as i8
                && sd_days_to_expiry(contract) > (25 - loyalty as i32) * 10
            {
                return;
            }
            if exe::human_manager_is_human_club(sd_contract_club(contract), 0, 1, 0, 0) != 0 {
                if g(contract) == 0 {
                    covered!("gra_2b player asks a human club for a contract");
                    exe::staff_contract_apply_event_flags(contract, 2, 0);
                } else {
                    staff_day_add_morale(contract, -5);
                }
                return;
            }
            let s = status(entry);
            let offered = match s {
                1 => sd_offer_new_contract(p, own(entry)),
                2..=6 => sd_offer_new_contract(p, false),
                _ => return,
            };
            if offered == 0 {
                sd_ask_for_contract(d);
            }
        }
        _ => {}
    }
}

/// No offer came: without grievances he asks for a new contract (grievance 2), with some he loses 5 morale
/// (0x5d19e4, 0x5d1bc1).
unsafe fn sd_ask_for_contract(d: &StaffDay) {
    if read_u32(d.contract + contract::GRIEVANCES) != 0 {
        staff_day_add_morale(d.contract, -5);
    } else {
        covered!("gra_2b player asks for a new contract");
        exe::staff_contract_apply_event_flags(d.contract, 2, 0);
    }
}

/// A contented player who already asked (grievance 2, 0x5d1d46..0x5d1eef): one day in 45 the AI club offers again
/// (+15 morale when it does); one player in 8 with grievance 2 alone and no offer pending goes public
/// (`post_wants_new_contract_news`, may dislike the club or manager); he loses 10 morale.
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d1d46..0x5d1eef.
unsafe fn staff_day_contract_reminder(d: &StaffDay, entry: usize) {
    use exe::utils_random as random;
    let p = d.p;
    let contract = d.contract;
    if exe::human_manager_is_human_club(sd_contract_club(contract), 0, 1, 0, 0) == 0
        && (read_i16(G_CURRENT_DATE) as i32).wrapping_add(read_i32(p)) % 45 == 0
    {
        let s = (read_u8(entry + 0x24) >> 1) & 0xf;
        let offered = match s {
            1..=3 => sd_offer_new_contract(p, read_u32(entry + 0xf) == read_u32(p)),
            4..=6 => sd_offer_new_contract(p, false),
            _ => 0,
        };
        if offered != 0 {
            staff_day_add_morale(contract, 15);
        }
    }
    let id = read_u32(p);
    if id & 7 == 0
        && read_u32(contract + contract::GRIEVANCES) == 2
        && exe::contract_manager_find_contract_offer(read_usize(G_CONTRACT_MANAGER), id, read_u32(contract + contract::CLUB)) == 0
    {
        covered!("gra_2b player reminds the club publicly");
        sd_post_wants_new_contract_news(contract);
        if read_i16(d.pd + player_sheet::CURRENT_REPUTATION) > 0x1482
            && read_i8(p + staff::AMBITION) > 5
            && read_i8(p + staff::TEMPERAMENT) < 15
            && read_i8(p + staff::SPORTSMANSHIP) < 15
            && read_i8(p + staff::PROFESSIONALISM) < 10
            && random(14) == 0
        {
            sd_dislike_club_or_manager(d);
        }
    }
    staff_day_add_morale(contract, -10);
}

/// A loan player (contract type 1) without a loan contract, offers or a free transfer pending, whose contract ended
/// more days ago than random(100) (0x5d1eef..0x5d1fd4): wants a new contract (grievance 2) and loses that many days of
/// morale (8 bits). Returns false when the loan conditions do not hold.
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d1eef..0x5d1fd4.
unsafe fn staff_day_contract_running_out(d: &StaffDay) -> bool {
    let p = d.p;
    let contract = d.contract;
    if read_u8(contract + contract::CONTRACT_TYPE) & 0x3f != 1
        || exe::contract_manager_get_loan_contract(read_usize(G_CONTRACT_MANAGER), p) != 0
        || exe::transfer_manager_get_offers_for_player(read_usize(G_TRANSFER_MANAGER), p, 0, 0) != 0
        || exe::transfer_manager_get_exchange_offers_for_player(read_usize(G_TRANSFER_MANAGER), p, 0) != 0
        || exe::transfer_manager_is_queued_for_free_transfer(read_usize(G_TRANSFER_MANAGER), p) != 0
    {
        return false;
    }
    let days = sd_days_since_inline(contract + contract::CONTRACT_EXPIRES);
    if days <= exe::utils_random(100) {
        return true;
    }
    covered!("gra_2b loan player wants a contract");
    if read_u32(contract + contract::GRIEVANCES) & 2 == 0 {
        exe::staff_contract_apply_event_flags(contract, 2, 0);
    }
    staff_day_add_morale(contract, (days as u8).wrapping_neg() as i8 as i32);
    true
}

/// A veteran far from home wants to go back (0x5d1fd4..0x5d24e7): without grievance 0x100, loyalty under 17, past 25
/// (past 31 only abroad), not queued for a free transfer, with a world reputation above 4250 and reputations above
/// random draws over the club's, one day in 50: the days at the club (or in the contract, under 250 + random(150)),
/// raised for a foreigner near his contract's end (under 300 days) and lowered past 600 days, against a threshold of
/// loyalty, reputation and two seeded squares (`G_GAME_RANDOM_SEED`), raised when he favours the club, by the club's
/// standing and its European place [inference: club state +0 or club +0x1db set], lowered for a foreigner: above it
/// (and above 700) he loses 20 morale and gets grievance 0x100. Returns false when the conditions fail (the relations
/// follow).
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d1fd4..0x5d238e.
unsafe fn staff_day_going_home(d: &StaffDay) -> bool {
    use exe::utils_random as random;
    let p = d.p;
    let club = d.club;
    let contract = d.contract;
    if read_u32(contract + contract::GRIEVANCES) & 0x100 != 0 || read_i8(p + staff::LOYALTY) >= 17 {
        return false;
    }
    let age = read_i8(p + staff::AGE);
    if age <= 25 || (age >= 32 && read_u32(club + club::NATION) == read_u32(p + staff::NATION)) {
        return false;
    }
    if exe::transfer_manager_is_queued_for_free_transfer(read_usize(G_TRANSFER_MANAGER), p) != 0 {
        return false;
    }
    let pd = read_usize(p + staff::PLAYER);
    if read_i16(pd + player_sheet::WORLD_REPUTATION) <= 0x109a {
        return false;
    }
    let current = read_i16(pd + player_sheet::CURRENT_REPUTATION) as f64;
    let r = random(2000);
    if r as f64 + read_i16(club + club::REPUTATION) as f64 * read_f64(rdata::F64_0_5) >= current {
        return false;
    }
    let world = read_i16(read_usize(p + staff::PLAYER) + player_sheet::WORLD_REPUTATION) as f64;
    let r = random(1000);
    if r as f64 + read_i16(club + club::REPUTATION) as f64 * read_f64(rdata::F64_0_25) >= world {
        return false;
    }
    if random(50) != 0 {
        return false;
    }
    let mut days = sd_days_since_inline(p + staff::DATE_JOINED_CLUB);
    let in_contract = sd_days_since_inline(contract + contract::DATE_STARTED);
    if in_contract < random(150) + 250 {
        days = in_contract;
    }
    let left = -sd_days_since_inline(contract + contract::CONTRACT_EXPIRES);
    let foreign = exe::database_is_staff_foreign(p, read_usize(club + club::NATION), 1) != 0;
    if foreign && left < 300 {
        days = days + left + 400;
    } else if left > 600 {
        days += (600 - left) * 10;
    }
    let id = read_i32(p);
    let seeded = id.wrapping_add(read_i32(G_GAME_RANDOM_SEED));
    let a = game_pow((seeded % 31) as f64, read_f64(rdata::F64_2));
    let b = game_pow((seeded % 26) as f64, read_f64(rdata::F64_2));
    let loyalty = read_i8(p + staff::LOYALTY) as i32;
    let base = (150 * loyalty + 0x898) as f64 - read_i16(d.pd + player_sheet::CURRENT_REPUTATION) as f64 * read_f64(rdata::F64_0_1333333333);
    let mut bar = ftol(base + a - b);
    if exe::staff_favours_club(p, club) != 0 {
        bar = ftol(((id % 6) as f64 * read_f64(rdata::F64_0_5555555556) + read_f64(rdata::F64_1_25)) * bar as f64);
        if left > 300 {
            bar = ftol(((id % 7) as f64 * read_f64(rdata::F64_0_5555555556) + read_f64(rdata::F64_1_25)) * bar as f64);
        }
    }
    let state = read_usize(G_CLUB_STATE_TABLE) + read_u32(club) as usize * 6;
    let standing = read_i8(state + 4);
    if standing >= 0 {
        bar += (loyalty + 30) * (standing as i32 + 1);
    }
    if read_i32(state) != -1 || read_i32(club + 0x1db) != -1 {
        bar += (loyalty + 10) * 50;
    }
    if exe::database_is_staff_foreign(p, read_usize(club + club::NATION), 1) != 0 {
        let k = (id.wrapping_add(read_i32(club)) % 11) as f64;
        bar = ftol((read_f64(rdata::F64_0_75) - k * read_f64(rdata::F64_0_05)) * bar as f64);
    }
    if days > 700 && days >= bar {
        covered!("gra_2b veteran wants to go home");
        staff_day_add_morale(contract, -20);
        exe::staff_contract_apply_event_flags(contract, 0x100, 0);
    }
    true
}

/// The dressing room (0x5d24e7..0x5d28f8): one day in 50 × professionalism², his morale moves by the club he favours
/// (+random(100)) or dislikes (random(-100), grievance 0x40 under -50), by the manager who favours (+random(75)) or
/// dislikes him (random(-75), grievance 8 under -50), by the disliked staff at the club (random(-25 × count)) and the
/// favoured ones (random(25 × count)); his three disliked team mates [inference: preferences +0x28/+0x2c/+0x30] who
/// are listed at the club lift him by random(loyalty + 10). Without a reason left he drops grievances 0x40 and 8.
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d24e7..0x5d28f8.
unsafe fn staff_day_relations(d: &StaffDay) {
    use exe::utils_random as random;
    let p = d.p;
    let club = d.club;
    let contract = d.contract;
    if club == 0 || random(50) != 0 {
        return;
    }
    let prof = read_i8(p + staff::PROFESSIONALISM) as i32;
    if random(prof * prof) != 0 {
        return;
    }
    covered!("gra_2b dressing room relations");
    let add = |n: i32| staff_day_add_morale(contract, n as i8 as i32);
    let unhappy = |flag: u32| {
        if read_i8(contract + contract::MORALE) < -50 {
            exe::staff_contract_apply_event_flags(contract, flag, 0);
        }
    };
    let mut reason = false;
    if exe::staff_favours_club(p, club) != 0 {
        add(random(100));
    } else if exe::staff_dislikes_club(p, club) != 0 {
        add(random(-100));
        unhappy(0x40);
        reason = true;
    }
    if exe::staff_is_favoured_by(read_usize(club + club::MANAGER), p) != 0 {
        add(random(75));
    } else if exe::staff_is_disliked_by(read_usize(club + club::MANAGER), p) != 0 {
        add(random(-75));
        unhappy(8);
        reason = true;
    }
    let disliked = sd_count_disliked_at_club(p, club) as i8 as i32;
    if disliked != 0 {
        add(random(-25 * disliked));
        reason = true;
        let prefs = read_usize(p + staff::STAFF_PREFERENCES);
        if prefs != 0 {
            for slot in [0x28usize, 0x2c, 0x30] {
                let other = read_usize(prefs + slot);
                if other == 0 {
                    continue;
                }
                let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), other);
                write_usize(G_CONTRACT_LOOKUP_RESULT, c);
                let at = if c != 0 { sd_contract_club(c) } else { 0 };
                if at == club && exe::staff_is_transfer_listed_or_requested(read_usize(read_usize(p + staff::STAFF_PREFERENCES) + slot), club) != 0 {
                    add(random(read_i8(p + staff::LOYALTY) as i32 + 10));
                    if read_i8(contract + contract::MORALE) > 50 {
                        reason = false;
                    }
                }
            }
        }
        unhappy(0x40);
    }
    let favoured = sd_count_favoured_at_club(p, club) as i8 as i32;
    if favoured != 0 {
        add(random(25 * favoured));
    }
    if !reason {
        exe::staff_contract_clear_grievances(contract, 0x40);
        exe::staff_contract_clear_grievances(contract, 8);
    }
}

/// `AWOL::start_absence` (thiscall: player, two zeros).
const FN_AWOL_START_ABSENCE_GRA: usize = 0x419840;

/// A media story about the player at his club (`Media::create_player_media_story`, no extra arguments).
unsafe fn sd_media_story(d: &StaffDay, kind: u32) {
    exe::media_create_player_media_story(read_usize(G_MEDIA), d.p, d.club, kind, 0, 0);
}

/// A restless player goes to the media: with a story already running, random(professionalism) under random(6), else
/// one time in professionalism + `extra` (`extra` 5 for kind 1, 0 for kind 3).
unsafe fn sd_media_outburst(d: &StaffDay, kind: u32, extra: i32) {
    use exe::utils_random as random;
    let prof = read_i8(d.p + staff::PROFESSIONALISM) as i32;
    if exe::media_has_story_for_player(read_usize(G_MEDIA), d.p) != 0 {
        let a = random(prof);
        if a < random(6) {
            sd_media_story(d, kind);
            return;
        }
    }
    if random(prof + extra) == 0 {
        sd_media_story(d, kind);
    }
}

/// The club of a person's contract, unless it is of type 5 [inference: a trial] (0 without one); the lookup is kept in
/// `G_CONTRACT_LOOKUP_RESULT` as the original does.
unsafe fn sd_club_of(person: usize) -> usize {
    let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person);
    write_usize(G_CONTRACT_LOOKUP_RESULT, c);
    if c != 0 && read_u8(c + contract::CONTRACT_TYPE) & 0x3f != 5 {
        sd_contract_club(c)
    } else {
        0
    }
}

/// The people the player dislikes (`staff_get_disliked_staff`, up to 50) and whether one shares his club; the
/// first one found stops the search.
unsafe fn sd_disliked_team_mate(p: usize) -> (bool, u8) {
    let mut list = [0usize; 50];
    let mut count: u8 = 0;
    exe::staff_get_disliked_staff(p, list.as_mut_ptr() as usize, &mut count as *mut u8 as usize);
    let mut k: i8 = 0;
    while k < count as i8 {
        let other = list[k as usize];
        if other != 0 && sd_club_of(other) == sd_club_of(p) {
            return (true, count);
        }
        k += 1;
    }
    (false, count)
}

/// The rest of a contracted player's day (0x5d2392..0x5d352d): grievances 0x10 and 4 end once in the side (left out
/// at most once, without a linked B team); a dropped, unsettled, famous one may tell the media he wants first team
/// football (`Media::queue_wants_first_team_story`, contract flag 0x200); grievance 0x20000 (homesick [inference]) ends
/// in his home region, else may go to the media (kind 1); contract flag 1 and grievance 0x800 (a disliked team mate)
/// end when no disliked person shares his club, else cost morale; an ambitious first team player whose deserved
/// status is 1 to 3 and who has a transfer pending may talk (kind 3); grievances 0x200, 0x400, 0x80, 0x10000, 0x800000,
/// 0x4000 and 0x8000 fade; he learns the club's language (squad status nibble, by adaptability) or, abroad, grows
/// homesick by the days at the club against his character, losing morale (grievance 0x20000), and at last, with
/// grievance 0x20, may go absent (`AWOL::start_absence`).
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d2392..0x5d352d.
unsafe fn staff_day_settling(d: &StaffDay) {
    use exe::utils_random as random;
    let p = d.p;
    let club = d.club;
    let contract = d.contract;
    let g = |c: usize| read_u32(c + contract::GRIEVANCES);
    let left_out = |c: usize| read_i8(c + contract::MATCHES_LEFT_OUT);
    // the ambition and temperament tests of the original all give way to "left out more than once"
    for bit in [0x10u32, 4] {
        if g(contract) & bit != 0 && d.b_team == 0 && left_out(contract) > 1 {
            exe::staff_contract_clear_grievances(contract, bit);
        }
    }
    let pressure = read_i8(p + staff::PRESSURE) as i32;
    let prof = read_i8(p + staff::PROFESSIONALISM) as i32;
    let loyalty = read_i8(p + staff::LOYALTY) as i32;
    if g(contract) & 0x10 != 0
        && exe::staff_is_transfer_listed_or_requested(p, 0usize) == 0
        && exe::transfer_manager_get_future_transfer(read_usize(G_TRANSFER_MANAGER), p, 0) == 0
        && read_i8(contract + contract::MORALE) < -25
        && left_out(contract) < 2
        && pressure < 6
        && random(prof + 5) == 0
        && read_usize(p + staff::PLAYER) != 0
        && read_i16(read_usize(p + staff::PLAYER) + player_sheet::CURRENT_REPUTATION) > 0x109a
        && (read_i16(G_CURRENT_DATE) as i32 % 7).wrapping_add(read_i32(p)) == 0
    {
        covered!("gra_2b dropped player wants first team football");
        let f: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_MEDIA_QUEUE_WANTS_FIRST_TEAM_STORY);
        f(read_usize(G_MEDIA), p, 0);
        exe::staff_contract_set_flags(contract, 0x200);
    }
    if g(contract) & 0x20000 != 0 {
        let home = read_usize(p + staff::NATION);
        if read_u8(home + nation::REGION) == read_u8(read_usize(club + club::NATION) + nation::REGION) {
            exe::staff_contract_clear_grievances(contract, 0x20000);
        } else if read_u8(p + staff::JOB_FOR_CLUB) == 11
            && exe::transfer_manager_get_future_transfer(read_usize(G_TRANSFER_MANAGER), p, 0) == 0
            && !d.away_from_job
            && read_i16(read_usize(p + staff::PLAYER) + player_sheet::CURRENT_REPUTATION) > 0x109a
            && (read_i16(G_CURRENT_DATE) as i32).wrapping_add(read_i32(p)) % 12 == 4
            && prof < random(5) + 5
            && loyalty < random(5) + 5
        {
            covered!("gra_2b homesick player talks");
            sd_media_outburst(d, 1, 5);
        }
    }
    if exe::staff_contract_get_flags(contract) & 1 != 0 {
        let temper = (read_i8(p + staff::TEMPERAMENT) as u8).wrapping_sub(read_i8(p + staff::PROFESSIONALISM) as u8).wrapping_add(25) as i8;
        let (found, _) = sd_disliked_team_mate(p);
        if !found || random(temper as i32) == 0 {
            exe::staff_contract_clear_flags(contract, 1);
        } else {
            staff_day_add_morale(contract, -5);
        }
    }
    if g(contract) & 0x800 != 0 {
        let (found, _) = sd_disliked_team_mate(p);
        if found {
            staff_day_add_morale(contract, -5);
        } else {
            exe::staff_contract_clear_grievances(contract, 0x800);
        }
    }
    if read_u8(p + staff::JOB_FOR_CLUB) == 11
        && read_i8(p + staff::AGE) > 23
        && !d.away_from_job
        && read_usize(club + club::DIVISION) != 0
        && read_i8(contract + contract::UNHAPPY_FLAG) > 2
    {
        let ambition = read_i8(p + staff::AMBITION) as i32;
        let r = random(((65 - ambition) * 2 + read_i8(p + staff::TEMPERAMENT) as i32) * 25);
        let status = |s: u8| exe::contract_manager_get_deserved_squad_status(read_usize(G_CONTRACT_MANAGER), p, contract) == s;
        if read_i16(d.pd + player_sheet::CURRENT_REPUTATION) as i32 > r + 4000
            && exe::injury_get_player_injury_status(read_usize(G_INJURY), p) != 2
            && exe::discipline_is_player_banned(read_usize(G_DISCIPLINE), p, read_usize(club + club::DIVISION), 1, 0) == 0
            && exe::transfer_manager_get_future_transfer(read_usize(G_TRANSFER_MANAGER), p, 0) != 0
            && (status(1) || status(2) || status(3))
            && exe::staff_is_transfer_listed_or_requested(p, club) == 0
            && ambition > 10
            && (read_i16(G_CURRENT_DATE) as i32).wrapping_add(read_i32(p)) % 14 == 5
        {
            covered!("gra_2b leaving star talks");
            sd_media_outburst(d, 3, 0);
        }
    }
    for (bit, base) in [(0x200u32, 50), (0x400, 40), (0x80, 105)] {
        if g(contract) & bit != 0 && random(base - prof - loyalty) == 0 {
            exe::staff_contract_clear_grievances(contract, bit);
        }
    }
    if g(contract) & 0x10000 != 0 {
        staff_day_add_morale(contract, -5);
        // defect of the original: the second test, (grievances & 1) == 0x10000, can never hold
        if exe::staff_is_transfer_listed_or_requested(p, 0usize) != 0 {
            exe::staff_contract_clear_grievances(contract, 0x10000);
        }
    }
    if g(contract) & 0x800000 != 0 {
        staff_day_add_morale(contract, -5);
        let future = |p: usize| exe::transfer_manager_get_future_transfer(read_usize(G_TRANSFER_MANAGER), p, 0);
        if future(p) != 0 {
            exe::staff_contract_clear_grievances(contract, 0x800000);
        } else if read_i32(p) % 14 == 0 && random(read_i8(p + staff::TEMPERAMENT) as i32 + 5) == 0 {
            exe::staff_contract_clear_grievances(contract, 0x800000);
        } else if future(p) == 0 {
            exe::staff_contract_clear_grievances(contract, 0x800000);
            staff_day_add_morale(contract, 15);
        }
    }
    if g(contract) & 0x4000 != 0 && random(100 - prof - loyalty) == 0 {
        exe::staff_contract_clear_grievances(contract, 0x4000);
    }
    if g(contract) & 0x8000 != 0 {
        let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_TRAINING_MANAGER_HAS_FAST_POSITION_TRAINING);
        if f(read_usize(G_TRAINING_MANAGER), p) == 0 {
            exe::staff_contract_clear_grievances(contract, 0x8000);
        }
    }
    if read_u8(contract + contract::SQUAD_STATUS) & 0xf < 10 {
        let n = ftol(read_f64(rdata::F64_100) - read_i8(p + staff::ADAPTABILITY) as f64 * read_f64(rdata::F64_3_5));
        if random(n) == 0 {
            covered!("gra_2b player learns the language");
            sd_learn_language(p, club, contract);
            return;
        }
    }
    staff_day_homesick(d);
}

/// The squad status nibble (here the language level at the club) rises by one, at most 10; at 1 the player adds the
/// club nation's first language (`PlayerStats::add_language`), up to 10 he improves it (`improve_language`).
unsafe fn sd_learn_language(p: usize, club: usize, contract: usize) {
    let s = read_u8(contract + contract::SQUAD_STATUS);
    if s & 0xf < 10 {
        write_u8(contract + contract::SQUAD_STATUS, ((s.wrapping_add(1) ^ s) & 0xf) ^ s);
    }
    let level = read_u8(contract + contract::SQUAD_STATUS) & 0xf;
    let language = read_u8(read_usize(club + club::NATION) + nation::FIRST_LANGUAGE) as u32;
    if level == 1 {
        exe::playerstats_add_language(read_usize(G_PLAYER_STATS), p, language, 1, 1);
    } else if level <= 10 {
        let f: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) = core::mem::transmute(FN_PLAYERSTATS_IMPROVE_LANGUAGE);
        f(read_usize(G_PLAYER_STATS), p, language, 1, 1);
    }
}

/// Homesickness (0x5d304e..0x5d352d): a player abroad (neither nation is the club's) with adaptability under 10, one
/// day in 30: the days in the contract, halved for a star (world reputation above 7000) on his own continent or for a
/// player from outside Europe [inference: `KEY_CONTINENT_EUROPE`], × 0.8 with a compatriot favoured team mate. Outside
/// his home region, past (professionalism + loyalty + 16) × 50 + (adaptability - age) × 20 + id % 200 days, he loses 5
/// morale and becomes homesick (grievance 0x20000) unless already unsettled (0x20000 or 0x20). Otherwise, between 50
/// and 500 days, when squad status² × 15 + morale × 1.5 - days falls under -150: grievance 0x20 with -50 morale, or
/// with it already, -10 morale and, for a disloyal, famous, very unhappy player one day in 12 × pressure, an absence.
/// A settled squad status (4 or more) ends grievance 0x20.
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d304e..0x5d352d.
unsafe fn staff_day_homesick(d: &StaffDay) {
    use exe::utils_random as random;
    let p = d.p;
    let club = d.club;
    let contract = d.contract;
    let club_nation = read_usize(club + club::NATION);
    if club_nation == read_usize(p + staff::NATION)
        || club_nation == read_usize(p + staff::SECOND_NATION)
        || read_i8(p + staff::ADAPTABILITY) >= 10
        || random(30) != 0
    {
        return;
    }
    let mut days = sd_days_since_inline(contract + contract::DATE_STARTED);
    let home = read_usize(p + staff::NATION);
    let same_continent = read_i16(d.pd + player_sheet::WORLD_REPUTATION) > 0x1b58
        && read_usize(read_usize(club + club::NATION) + nation::CONTINENT) == read_usize(home + nation::CONTINENT);
    let continent = read_usize(home + nation::CONTINENT);
    if same_continent || (continent != 0 && read_u32(continent) != read_u32(KEY_CONTINENT_EUROPE)) {
        days /= 2;
    }
    if d.mate != 0 && read_usize(d.mate + staff::NATION) == home {
        days = ftol(days as f64 * read_f64(rdata::F64_0_8));
    }
    if read_u8(home + nation::REGION) != read_u8(read_usize(club + club::NATION) + nation::REGION) {
        let character = (read_i8(p + staff::PROFESSIONALISM) as i32 + read_i8(p + staff::LOYALTY) as i32 + 16) * 50
            + (read_i8(p + staff::ADAPTABILITY) as i32 - read_i8(p + staff::AGE) as i32) * 20
            + read_i32(p) % 200;
        if days > character {
            let g = read_u32(contract + contract::GRIEVANCES);
            if g & 0x20000 == 0 && g & 0x20 == 0 {
                covered!("gra_2b player homesick");
                staff_day_add_morale(contract, -5);
                exe::staff_contract_apply_event_flags(contract, 0x20000, 0);
            }
            return;
        }
    }
    if days > 50 && days < 500 {
        let s = (read_u8(contract + contract::SQUAD_STATUS) & 0xf) as i32;
        let morale = read_i8(contract + contract::MORALE);
        let mood = (s * s * 15) as f64 + morale as f64 * read_f64(rdata::F64_1_5) - days as f64;
        if !(mood >= read_f64(0x96afd0)) {
            // under -150
            if read_u32(contract + contract::GRIEVANCES) & 0x20 == 0 {
                covered!("gra_2b player unsettled abroad");
                staff_day_add_morale(contract, -50);
                exe::staff_contract_apply_event_flags(contract, 0x20, 0);
                return;
            }
            staff_day_add_morale(contract, -10);
            if exe::awol_is_player_awol(read_usize(G_AWOL), p, 0) != 0
                || exe::transfer_manager_get_future_transfer(read_usize(G_TRANSFER_MANAGER), p, 0) != 0
                || read_i8(p + staff::LOYALTY) >= 5
                || random(12) != 0
                || random(read_i8(p + staff::PRESSURE) as i32) != 0
                || read_usize(p + staff::PLAYER) == 0
                || read_i16(read_usize(p + staff::PLAYER) + player_sheet::CURRENT_REPUTATION) <= 0xcb2
                || read_i8(contract + contract::MORALE) >= -50
            {
                return;
            }
            covered!("gra_2b homesick player goes absent");
            let f: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(FN_AWOL_START_ABSENCE_GRA);
            f(read_usize(G_AWOL), p, 0, 0);
            return;
        }
    }
    if read_u8(contract + contract::SQUAD_STATUS) & 0xf >= 4
        && read_u32(contract + contract::GRIEVANCES) & 0x20 != 0
        && exe::awol_is_player_awol(read_usize(G_AWOL), p, 0) == 0
    {
        exe::staff_contract_clear_grievances(contract, 0x20);
    }
}

/// `Injury::get_match_condition` (0x62c510, thiscall: player, in percent).
const FN_INJURY_GET_MATCH_CONDITION_GRA: usize = 0x62c510;
/// `AWOL::report_poor_fitness` (0x41add0, thiscall: player, club).
const FN_AWOL_REPORT_POOR_FITNESS_GRA: usize = 0x41add0;
/// `staff_remove_favoured_club` (0x547f50, cdecl: person, club).
const FN_STAFF_REMOVE_FAVOURED_CLUB_GRA: usize = 0x547f50;
/// The game.cpp source path (the error box).
const GAME_CPP_PATH: usize = 0x9cd2a4;
/// 1 until the day's first development builds the two ability tables (`G_PLAYER_ABILITY_VALUES_SORTED`,
/// `G_PLAYER_DATA_PTRS_BY_ABILITY`); byte.
const G_ABILITY_TABLES_PENDING: usize = 0x9cd236;
/// qsort comparer of two 16-bit abilities (game.cpp, cdecl).
const FN_COMPARE_ABILITY_VALUES: usize = 0x5d6200;
/// qsort comparer of two player data records by ability, then potential (`compare_players_by_ability`, hooked).
const FN_COMPARE_PLAYERS_BY_ABILITY: usize = 0x5d6220;

/// Adds `delta` to the contract's morale (`contract::MORALE`, signed byte), the sum held to -100..100: the clamp the
/// daily staff update repeats inline some sixty times (the sum in 32 bits; inside the range the byte gets
/// morale + delta).
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), e.g. 0x5cf8ff..0x5cf920.
pub(crate) unsafe fn staff_day_add_morale(contract: usize, delta: i32) {
    let m = read_i8(contract + contract::MORALE);
    let sum = (m as i32).wrapping_add(delta);
    if sum > 100 {
        write_u8(contract + contract::MORALE, 100);
    } else if sum < -100 {
        write_u8(contract + contract::MORALE, 0x9c);
    } else {
        write_u8(contract + contract::MORALE, (m as u8).wrapping_add(delta as u8));
    }
}

/// The day of a person without a player sheet (0x5d3367..0x5d352d). Without a non-player sheet nothing changes
/// (an unattached person goes through `staff_day_unattached_decline`, which then has nothing to do). With a
/// contract whose squad status (low nibble of `contract::SQUAD_STATUS`) is under 10, one draw in ftol(100 -
/// adaptability × 3.5) raises that nibble by one (the high nibble kept) and the person learns the language of the club's
/// nation (`nation::FIRST_LANGUAGE`): the first step adds it (`PlayerStats::add_language`), the next ones up to 10
/// improve it (`improve_language`), fluency 1, the list sorted. Unattached (no club): `staff_day_unattached_decline`.
/// At a club, except in the club job 5 [inference: the job byte `staff::JOB_FOR_CLUB`, 5 left alone]: the current
/// reputation becomes ftol(((club reputation + 50 × current ability) / 2) × 0.05 + current reputation × 0.95) (the
/// half truncated and taken to 16 bits); under 40 years old the current ability moves by -random(3) + random(6) three
/// days in four, else -random(6) + random(6); from 40 only one day in 20, by -random(6) + random(6) (the other days
/// keep it and skip the normalisation); capped at the potential and the attributes normalised
/// (`staff_normalise_non_player_attributes`).
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d3367..0x5d352d.
pub(crate) unsafe fn staff_day_non_player(p: usize, club: usize, contract: usize) {
    let np = read_usize(p + staff::NON_PLAYER);
    if np == 0 {
        if club == 0 {
            staff_day_unattached_decline(p, 0);
        }
        return;
    }
    if contract != 0 && (read_u8(contract + contract::SQUAD_STATUS) & 0xf) < 10 {
        let n = ftol(read_f64(rdata::F64_100) - read_i8(p + staff::ADAPTABILITY) as f64 * read_f64(rdata::F64_3_5));
        if exe::utils_random(n) == 0 {
            covered!("gra_2 non-player learns a language");
            let s = read_u8(contract + contract::SQUAD_STATUS);
            if s & 0xf < 10 {
                write_u8(contract + contract::SQUAD_STATUS, ((s.wrapping_add(1) ^ s) & 0xf) ^ s);
            }
            let level = read_u8(contract + contract::SQUAD_STATUS) & 0xf;
            let language = read_u8(read_usize(club + club::NATION) + nation::FIRST_LANGUAGE) as u32;
            if level == 1 {
                exe::playerstats_add_language(read_usize(G_PLAYER_STATS), p, language, 1, 1);
            } else if level <= 10 {
                let f: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) = core::mem::transmute(FN_PLAYERSTATS_IMPROVE_LANGUAGE);
                f(read_usize(G_PLAYER_STATS), p, language, 1, 1);
            }
        }
    }
    if club == 0 {
        staff_day_unattached_decline(p, 0);
        return;
    }
    if read_u8(p + staff::JOB_FOR_CLUB) == 5 {
        return;
    }
    let half = ((read_i16(club + club::REPUTATION) as i32).wrapping_add(50 * read_i16(np + non_player::CURRENT_ABILITY) as i32) / 2) as i16;
    let current = read_i16(np + non_player::CURRENT_REPUTATION);
    let r = ftol(half as f64 * read_f64(rdata::F64_0_05) + current as f64 * read_f64(rdata::F64_0_95));
    write_u16(np + non_player::CURRENT_REPUTATION, r as u16);
    let ability = np + non_player::CURRENT_ABILITY;
    if read_i8(p + staff::AGE) < 40 {
        let down = if exe::utils_random(4) != 0 { exe::utils_random(3) } else { exe::utils_random(6) };
        let a = read_u16(ability).wrapping_sub(down as u16);
        write_u16(ability, a.wrapping_add(exe::utils_random(6) as u16));
    } else {
        if exe::utils_random(20) != 0 {
            return;
        }
        covered!("gra_2 non-player over 40 ability moves");
        let down = exe::utils_random(6);
        let a = read_u16(ability).wrapping_sub(down as u16);
        write_u16(ability, a.wrapping_add(exe::utils_random(6) as u16));
    }
    let potential = read_i16(np + non_player::POTENTIAL_ABILITY);
    if read_i16(ability) > potential {
        write_i16(ability, potential);
    }
    exe::staff_normalise_non_player_attributes(p);
}

/// An unattached person loses reputation (0x5d3539..0x5d377a; also the end of `staff_day_non_player`). The days
/// since joining (or leaving) the last club, (year - year joined) × 365 - day joined + today's day, against a grace of
/// 60 days plus 2 per cap (`staff::INT_APPS`): within it nothing. A player (`pd`, the player sheet) under 525 days loses
/// random(days - grace), halved (truncated, 16 bits) when the caps beat random(30); with a loss the current reputation
/// becomes the larger of 12.5 × current ability and current - loss, the home reputation the larger of 12.5 × current
/// ability and the new current - loss, the world reputation the larger of 5 × current ability and the new current -
/// loss. A non-player sheet of someone without a national job, between 361 and 524 days: loss ftol(random(days - 360)
/// × 0.25), drawn again as random(loss) when the nation's league is not loaded (`nation::LEAGUE_SELECTED`), halved when
/// the caps beat random(30); with a loss, half of it comes off the non-player current reputation (floor 35 × current
/// ability), the home reputation becomes the larger of 35 × current ability and the new current - half, the world
/// reputation the larger of 5 × current ability and the new current - half.
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d3539..0x5d377a.
pub(crate) unsafe fn staff_day_unattached_decline(p: usize, pd: usize) {
    let days = (read_u16(G_CURRENT_DATE_YEAR) as i32)
        .wrapping_sub(read_u16(p + staff::DATE_JOINED_CLUB_YEAR) as i32)
        .wrapping_mul(365)
        .wrapping_sub(read_i16(p + staff::DATE_JOINED_CLUB) as i32)
        .wrapping_add(read_i16(G_CURRENT_DATE) as i32);
    let grace = read_u8(p + staff::INT_APPS) as i32 * 2 + 60;
    if days <= grace {
        return;
    }
    if pd != 0 && days < 525 {
        let mut loss = exe::utils_random(days - grace);
        if read_u8(p + staff::INT_APPS) as i32 > exe::utils_random(30) {
            loss = (loss as i16 as i32) / 2;
        }
        if (loss as i16) > 0 {
            covered!("gra_2 unattached player loses reputation");
            let loss = loss as i16 as i32;
            // fcom: the floor 12.5 x ability stays only when above the value (ties and NaN take the value)
            let floored = |x: i32| {
                let floor = read_i16(pd + player_sheet::CURRENT_ABILITY) as f64 * read_f64(rdata::F64_12_5);
                ftol(if floor > x as f64 { floor } else { x as f64 })
            };
            let current = floored((read_i16(pd + player_sheet::CURRENT_REPUTATION) as i32).wrapping_sub(loss));
            write_u16(pd + player_sheet::CURRENT_REPUTATION, current as u16);
            let home = floored((current as i16 as i32).wrapping_sub(loss));
            write_u16(pd + player_sheet::HOME_REPUTATION, home as u16);
            let x = (read_i16(pd + player_sheet::CURRENT_REPUTATION) as i32).wrapping_sub(loss);
            let five = read_i16(pd + player_sheet::CURRENT_ABILITY) as i32 * 5;
            write_u16(pd + player_sheet::WORLD_REPUTATION, if five > x { five } else { x } as u16);
        }
    }
    let np = read_usize(p + staff::NON_PLAYER);
    if np == 0 || read_u32(p + staff::NATIONAL_JOB) != 0 || days <= 360 || days >= 525 {
        return;
    }
    let mut loss = ftol(exe::utils_random(days - 360) as f64 * read_f64(rdata::F64_0_25));
    if read_u8(read_usize(p + staff::NATION) + nation::LEAGUE_SELECTED) == 0 {
        loss = exe::utils_random(loss as i16 as i32);
    }
    if read_u8(p + staff::INT_APPS) as i32 > exe::utils_random(30) {
        loss = (loss as i16 as i32) / 2;
    }
    if (loss as i16) <= 0 {
        return;
    }
    covered!("gra_2 unattached non-player loses reputation");
    let half = (loss as i16 as i32) / 2;
    let at_least = |floor: i32, x: i32| if floor > x { floor } else { x } as u16;
    let ability = |np: usize| read_i16(np + non_player::CURRENT_ABILITY) as i32;
    let current = |np: usize| read_i16(np + non_player::CURRENT_REPUTATION) as i32;
    write_u16(np + non_player::CURRENT_REPUTATION, at_least(35 * ability(np), current(np).wrapping_sub(half)));
    write_u16(np + non_player::HOME_REPUTATION, at_least(35 * ability(np), current(np).wrapping_sub(half)));
    write_u16(np + non_player::WORLD_REPUTATION, at_least(5 * ability(np), current(np).wrapping_sub(half)));
}

/// A player on a club's training schedule may come out of training below par (0x5d3786..0x5d38f9). The player's
/// schedule (`TRAINING_MANAGER::get_player_schedule`) must exist with its byte +0x15 set [inference: training today],
/// at a club, without a non-player sheet and not injured (`Injury::get_player_injury_status` other than 2). The odds
/// are ftol(professionalism³ + 30) (the game's pow), plus random(morale) with a contract, at least 20; one draw in them
/// (random below 1): with the injury object, no current injury (`Injury::get_player_injury`) and not AWOL, the match
/// condition (`Injury::get_match_condition`) drops by random(40) × 100 / (random(natural fitness), at least 1)
/// (`Injury::set_match_condition`, 16 bits); a drop of at least 1000 + random(750) leaving the condition at 8000 or
/// less, when the player is not away on international duty
/// (`NationalTeams::get_international_duty_club` with today's date), is reported (`AWOL::report_poor_fitness`, with the
/// club job).
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d3786..0x5d38f9.
pub(crate) unsafe fn staff_day_training_condition(p: usize, club: usize, contract: usize) {
    let schedule = exe::training_manager_get_player_schedule(read_usize(G_TRAINING_MANAGER), p);
    if club == 0
        || schedule == 0
        || read_u8(schedule + 0x15) == 0
        || read_usize(p + staff::NON_PLAYER) != 0
        || exe::injury_get_player_injury_status(read_usize(G_INJURY), p) == 2
    {
        return;
    }
    let mut odds = ftol(game_pow(read_i8(p + staff::PROFESSIONALISM) as f64, read_f64(rdata::F64_3)) + read_f64(rdata::F64_30));
    if contract != 0 {
        odds = odds.wrapping_add(exe::utils_random(read_i8(contract + contract::MORALE) as i32));
    }
    if odds < 20 {
        odds = 20;
    }
    if exe::utils_random(odds) >= 1 {
        return;
    }
    let injury = read_usize(G_INJURY);
    if injury == 0 || exe::injury_get_player_injury(injury, p) != 0 || exe::awol_is_player_awol(read_usize(G_AWOL), p, 0) != 0 {
        return;
    }
    covered!("gra_2 tired after training");
    let pd = read_usize(p + staff::PLAYER);
    let fitness = exe::utils_random(read_i8(pd + player_sheet::NATURAL_FITNESS) as i32) as i8;
    let divisor = if fitness < 1 { 1 } else { fitness as i32 };
    let drop = exe::utils_random(40).wrapping_mul(100) / divisor;
    let get_condition: unsafe extern "thiscall" fn(usize, usize, u32) -> u16 = core::mem::transmute(FN_INJURY_GET_MATCH_CONDITION_GRA);
    let condition = get_condition(read_usize(G_INJURY), p, 0) as i16;
    let new = condition.wrapping_sub(drop as i16);
    exe::injury_set_match_condition(read_usize(G_INJURY), p, new, 0);
    let limit = exe::utils_random(750) + 1000;
    if (drop as i16 as i32) < limit || new > 8000 {
        return;
    }
    if exe::nationalteams_get_international_duty_club(read_usize(G_NATIONAL_TEAMS), p, G_CURRENT_DATE, club, 0, 0) != 0 {
        return;
    }
    covered!("gra_2 poor fitness reported");
    let report: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_AWOL_REPORT_POOR_FITNESS_GRA);
    report(read_usize(G_AWOL), p, read_usize(p + staff::CLUB_JOB));
}

/// The two tables of the day's ability redistribution, built by the day's first development (0x5d393e..0x5d3a0c):
/// while `G_ABILITY_TABLES_PENDING` is set, the current abilities of the whole player data table (+5 of each 0x46-byte
/// record) are copied to a new array (`crt_malloc`, 2 bytes each) and sorted (qsort, 16-bit compare 0x5d6200), and a
/// second array (4 bytes each) gets the address of each player data record in table order; the flag is cleared. A
/// failed allocation is the game.cpp error box at line 0x18d7 or 0x18e6 and the game ends (cleanup, exit(-1)). The
/// arrays are freed by whoever reads them [inference: not in this routine].
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d393e..0x5d3a0c.
pub(crate) unsafe fn staff_day_build_ability_tables() {
    if read_u8(G_ABILITY_TABLES_PENDING) == 0 {
        return;
    }
    let values = exe::crt_malloc((read_i32(G_PLAYER_DATA_COUNT) as u32).wrapping_mul(2));
    write_usize(G_PLAYER_ABILITY_VALUES_SORTED, values);
    if values == 0 {
        fatal_error_at(GAME_CPP_PATH, 0x18d7);
    }
    let table = read_usize(G_PLAYER_DATA_TABLE);
    let mut i = 0;
    while i < read_i32(G_PLAYER_DATA_COUNT) {
        write_u16(read_usize(G_PLAYER_ABILITY_VALUES_SORTED) + 2 * i as usize, read_u16(table + 0x46 * i as usize + 5));
        i += 1;
    }
    exe::crt_qsort(values, read_u32(G_PLAYER_DATA_COUNT), 2, FN_COMPARE_ABILITY_VALUES);
    let pointers = exe::crt_malloc(read_u32(G_PLAYER_DATA_COUNT).wrapping_mul(4));
    write_usize(G_PLAYER_DATA_PTRS_BY_ABILITY, pointers);
    if pointers == 0 {
        fatal_error_at(GAME_CPP_PATH, 0x18e6);
    }
    let mut i = 0;
    while i < read_i32(G_PLAYER_DATA_COUNT) {
        write_usize(read_usize(G_PLAYER_DATA_PTRS_BY_ABILITY) + 4 * i as usize, table + 0x46 * i as usize);
        i += 1;
    }
    write_u8(G_ABILITY_TABLES_PENDING, 0);
}

/// The age at which a player's development turns, by his best position (0x5d3a92..0x5d3af0): 31, or 28 with
/// attacker or attacking midfielder 15+, 29 with midfielder 15+, 30 with defensive midfielder 15+ (the first that
/// matches); then 33 for a goalkeeper 20, else 31 with sweeper 15+ or any defender rating. Defect of the original:
/// "defender != 0" (not "15+") sends nearly every outfield player back to 31, undoing the midfield and attack ages.
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d3a92..0x5d3af0.
pub(crate) unsafe fn staff_day_development_age(pd: usize) -> i8 {
    let mut age = 31;
    if read_i8(pd + player_sheet::ATTACKER) >= 15 || read_i8(pd + player_sheet::ATTACKING_MIDFIELDER) >= 15 {
        age = 28;
    } else if read_i8(pd + player_sheet::MIDFIELDER) >= 15 {
        age = 29;
    } else if read_i8(pd + player_sheet::DEFENSIVE_MIDFIELDER) >= 15 {
        age = 30;
    }
    if read_i8(pd + player_sheet::GOALKEEPER) == 20 {
        age = 33;
    } else if read_i8(pd + player_sheet::SWEEPER) >= 15 || read_i8(pd + player_sheet::DEFENDER) != 0 {
        age = 31; // defect of the original: any defender rating
    }
    age
}

/// One of a player's three development days a year, one time in ten: his personality drifts (0x5d3af0..0x5d3d09).
/// Ambition +1 under 25 (below 20, one in three) or -1 over 25 (above 1, one in three). Then, each one time in four,
/// half up and half down (random(2)), between 1 and 20: loyalty (at 20 going up, the club job becomes a favoured club,
/// `staff_add_favoured_club` with random(2); at 1 going down, it stops being one, `staff_remove_favoured_club`),
/// determination, pressure (up by 2 for a player over 22 with professionalism above 10, pressure under 10 and current
/// reputation above 6750; else up by 1 only with professionalism above 5; down by 2 from 3 for a player under 21 with
/// current reputation above 5250, else down by 1 only with professionalism under 15), professionalism (up by 2 from
/// under 10 past 28 years old, else 1; down by 2 from 3 for a player under 23 at a club job whose reputation is below
/// his current reputation (above 4250), else 1), temperament and sportsmanship. `age` is the age byte, `club` the
/// club job (`staff::CLUB_JOB`).
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d3af0..0x5d3d09.
pub(crate) unsafe fn staff_day_personality_drift(p: usize, age: i8, club: usize) {
    use exe::utils_random as random;
    if random(10) != 0 {
        return;
    }
    covered!("gra_2 personality drift");
    let ambition = p + staff::AMBITION;
    if age < 25 {
        if read_i8(ambition) < 20 && random(3) == 0 {
            write_u8(ambition, read_u8(ambition).wrapping_add(1));
        }
    } else if age > 25 && read_i8(ambition) > 1 && random(3) == 0 {
        write_u8(ambition, read_u8(ambition).wrapping_sub(1));
    }
    // loyalty
    if random(4) == 0 {
        let up = random(2) == 0;
        let v = read_i8(p + staff::LOYALTY);
        if up {
            if v < 20 {
                write_i8(p + staff::LOYALTY, v + 1);
            } else {
                let r = random(2);
                exe::staff_add_favoured_club(p, club, r as u32);
            }
        } else if v > 1 {
            write_i8(p + staff::LOYALTY, v - 1);
        } else {
            let remove: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_STAFF_REMOVE_FAVOURED_CLUB_GRA);
            remove(p, club);
        }
    }
    // determination
    if random(4) == 0 {
        let up = random(2) == 0;
        let v = read_i8(p + staff::DETERMINATION);
        if up {
            if v < 20 {
                write_i8(p + staff::DETERMINATION, v + 1);
            }
        } else if v > 1 {
            write_i8(p + staff::DETERMINATION, v - 1);
        }
    }
    // pressure
    if random(4) == 0 {
        let up = random(2) == 0;
        let v = read_i8(p + staff::PRESSURE);
        let pd = read_usize(p + staff::PLAYER);
        let professionalism = read_i8(p + staff::PROFESSIONALISM);
        if up {
            if v < 20 {
                if pd != 0 && age > 22 && professionalism > 10 && v < 10 && read_i16(pd + player_sheet::CURRENT_REPUTATION) > 0x1a5e {
                    write_i8(p + staff::PRESSURE, v + 2);
                } else if professionalism > 5 {
                    write_i8(p + staff::PRESSURE, v + 1);
                }
            }
        } else if v > 1 {
            if v > 2 && age < 21 && pd != 0 && read_i16(pd + player_sheet::CURRENT_REPUTATION) > 0x1482 {
                write_i8(p + staff::PRESSURE, v - 2);
            } else if professionalism < 15 {
                write_i8(p + staff::PRESSURE, v - 1);
            }
        }
    }
    // professionalism
    if random(4) == 0 {
        let up = random(2) == 0;
        let v = read_i8(p + staff::PROFESSIONALISM);
        if up {
            if v < 20 {
                write_i8(p + staff::PROFESSIONALISM, if v < 10 && age > 28 { v + 2 } else { v + 1 });
            }
        } else if v > 1 {
            let pd = read_usize(p + staff::PLAYER);
            let fall_two = club != 0 && pd != 0 && v > 2 && age < 23 && {
                let reputation = read_i16(pd + player_sheet::CURRENT_REPUTATION);
                reputation > 0x109a && read_i16(club + club::REPUTATION) < reputation
            };
            write_i8(p + staff::PROFESSIONALISM, if fall_two { v - 2 } else { v - 1 });
        }
    }
    // temperament, sportsmanship
    for field in [staff::TEMPERAMENT, staff::SPORTSMANSHIP] {
        if random(4) == 0 {
            let up = random(2) == 0;
            let v = read_i8(p + field);
            if up {
                if v < 20 {
                    write_i8(p + field, v + 1);
                }
            } else if v > 1 {
                write_i8(p + field, v - 1);
            }
        }
    }
}

/// A player may join the clubs' shortlists (0x5d578d..0x5d5864, every day for every player, after the development).
/// Interest: current reputation above 5000 + random(3000), or caps above random(10) with world reputation above
/// 3000; then, if the player's state byte has no low nibble set (`G_STAFF_STATE_TABLE` +0xb) [inference: not a
/// retired or virtual player] and he has grievances (`contract::GRIEVANCES`) or is listed or asked to leave
/// (`staff_is_transfer_listed_or_requested`), one time in 17 he is added. Otherwise random(current reputation) above
/// random(2,000,000) adds him, and so does one time in 20 at a club without a division (`club::DIVISION` 0). Added:
/// `SHORTLIST_MANAGER::add_player_to_club_shortlists` (the transfer manager's shortlist manager, the player, 1).
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d578d..0x5d5864.
pub(crate) unsafe fn staff_day_shortlist_check(p: usize, pd: usize, club: usize, contract: usize) {
    use exe::utils_random as random;
    let interest = read_i16(pd + player_sheet::CURRENT_REPUTATION) as i32 > random(3000) + 5000
        || (read_u8(p + staff::INT_APPS) as i32 > random(10) && read_i16(pd + player_sheet::WORLD_REPUTATION) > 3000);
    let add = 'decide: {
        if interest && read_u8(read_usize(G_STAFF_STATE_TABLE) + read_u32(p) as usize * 0x4f + 0xb) & 0xf == 0 {
            let unsettled = (contract != 0 && read_u32(contract + contract::GRIEVANCES) != 0) || exe::staff_is_transfer_listed_or_requested(p, 0usize) != 0;
            if unsettled && random(0x11) == 0 {
                covered!("gra_2 unsettled player shortlisted");
                break 'decide true;
            }
        }
        let a = random(read_i16(pd + player_sheet::CURRENT_REPUTATION) as i32);
        if a > random(2_000_000) {
            break 'decide true;
        }
        club != 0 && read_u32(club + club::DIVISION) == 0 && random(0x14) == 0
    };
    if add {
        covered!("gra_2 player shortlisted");
        let manager = transfer_manager_get_shortlist_manager(read_usize(G_TRANSFER_MANAGER));
        exe::shortlist_manager_add_player_to_club_shortlists(manager, p, 1);
    }
}

/// The end of the day, when some development moved an ability (0x5d587a..0x5d58e0): the player data records (the
/// array `G_PLAYER_DATA_PTRS_BY_ABILITY`) are sorted by current ability, then potential (qsort,
/// `compare_players_by_ability`), and the abilities sorted at the start of the day (`G_PLAYER_ABILITY_VALUES_SORTED`)
/// are handed back in that order, each capped at the record's potential: the spread of abilities stays the day's
/// [inference: what the original keeps by this].
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d587a..0x5d58e0.
pub(crate) unsafe fn staff_day_hand_back_abilities() {
    exe::crt_qsort(read_usize(G_PLAYER_DATA_PTRS_BY_ABILITY), read_u32(G_PLAYER_DATA_COUNT), 4, FN_COMPARE_PLAYERS_BY_ABILITY);
    let mut i = 0usize;
    while (i as i32) < read_i32(G_PLAYER_DATA_COUNT) {
        let record = read_usize(read_usize(G_PLAYER_DATA_PTRS_BY_ABILITY) + 4 * i);
        write_u16(record + player_sheet::CURRENT_ABILITY, read_u16(read_usize(G_PLAYER_ABILITY_VALUES_SORTED) + 2 * i));
        let potential = read_i16(record + player_sheet::POTENTIAL_ABILITY);
        if read_i16(record + player_sheet::CURRENT_ABILITY) > potential {
            write_i16(record + player_sheet::CURRENT_ABILITY, potential);
        }
        i += 1;
    }
}

// ---------- the development day (0x5d3d09..0x5d5771) ----------

/// The 13 growth steps of a development day (shorts, -15..14), indexed by two ratings held to 0..12.
const DEVELOPMENT_GROWTH_STEPS: usize = 0x9cd21c;
/// f32 -3.0: below it a falling player loses reputation at the fixed rates.
const F32_NEG_3_DEVELOPMENT: usize = 0x96afc0;
/// 7.5e-5: the potential squared, as part of the first rating.
const F64_POTENTIAL_SQUARED_SCALE: usize = 0x96af98;
/// 6.5: the base of the second rating.
const F64_6_5_DEVELOPMENT: usize = 0x96af90;
/// 1 / 26.6: a club's training facilities (or a manager's youngsters rating) to the training factor.
const F64_TRAINING_SCALE: usize = 0x96af80;
/// 1 / 13,300: a club's reputation to the training factor when it has no training facilities rating.
const F64_CLUB_REPUTATION_TRAINING_SCALE: usize = 0x96af78;
/// f32 1.31: the training factor at a club with a human manager (or none).
const F32_1_31_HUMAN_MANAGER: usize = 0x96af70;
/// 1 / 53.2: a coach's rating to the training factor.
const F64_COACH_SCALE: usize = 0x96af68;
/// 1 / 30.7: the division's reputation to the level factor.
const F64_DIVISION_LEVEL_SCALE: usize = 0x96af60;
/// 1 / 15,350: the club's reputation to the level factor when it has no division.
const F64_CLUB_LEVEL_SCALE: usize = 0x96af58;
/// -0.9: the share of the ability kept above a weak division's ceiling.
const F64_NEG_0_9_DEVELOPMENT: usize = 0x96af50;

/// `pow_double` (0x5d5a20, game.cpp, cdecl: base, exponent; the result in st0).
const FN_POW_DOUBLE_GRA: usize = 0x5d5a20;
/// `game_average` (0x5d5a60, game.cpp, thiscall: the player stats, second; the result in st0).
const FN_GAME_AVERAGE_GRA: usize = 0x5d5a60;
/// `clamp_char_sum` (0x5d61d0, game.cpp, cdecl: value, delta, minimum, maximum; bytes).
const FN_CLAMP_CHAR_SUM_GRA: usize = 0x5d61d0;
/// `player_search_get_club_target_rating` (0x7bca90, cdecl: club, base, forced).
const FN_PLAYER_SEARCH_GET_CLUB_TARGET_RATING_GRA: usize = 0x7bca90;
/// `training_manager_get_player_entry` (0x5d5ac0, game.cpp, thiscall: the player).
const FN_TRAINING_MANAGER_GET_PLAYER_ENTRY_GRA: usize = 0x5d5ac0;
/// `staff_fade_staff_relationships` (0x547f90, cdecl: person).
const FN_STAFF_FADE_STAFF_RELATIONSHIPS_GRA: usize = 0x547f90;

unsafe fn sd_pow_double(base: f64, exponent: f64) -> f64 {
    let f: unsafe extern "C" fn(f64, f64) -> f64 = core::mem::transmute(FN_POW_DOUBLE_GRA);
    f(base, exponent)
}

/// One attribute byte of the player sheet moved by `delta` and held to `min..max` (`clamp_char_sum`; only `al` is
/// written back, as the original does).
unsafe fn sd_clamp_attribute(pd: usize, field: usize, delta: i32, min: i32, max: i32) {
    let f: unsafe extern "C" fn(u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_CLAMP_CHAR_SUM_GRA);
    write_u8(pd + field, f(read_u8(pd + field) as u32, delta as u32, min as u32, max as u32) as u8);
}

/// The locals of one development day, as the original keeps them in its frame.
struct Development {
    /// The person (`ebp`) and his player sheet (`esi`).
    p: usize,
    pd: usize,
    /// The club job ([esp+0x30], `ebx`).
    club: usize,
    /// The age byte ([esp+0x3f]).
    age: i8,
    /// The age factor ([esp+0x20], a float): positive while the player still develops, negative when he declines.
    x: f32,
    /// The stamina mark ([esp+0x48], a byte): added to the stamina change.
    mark: i8,
    /// A long injury cut the growth ([esp+0x58]).
    injured: bool,
    /// The day's growth (the short [esp+0x28]); its final value also marks the day as changed.
    growth: i16,
    /// The physical attributes' step (the short [esp+0x5c]): the second growth step, at least -8.
    step: i16,
    /// The growth as it was when the ability moved ([esp+0x2c] from 0x5d4897).
    saved_growth: i32,
}

/// The age factor of a development day (0x5d3d09..0x5d3eba): x = turning age - age, × 0.25 when positive. Below 0
/// the player declines: when random(natural fitness) > 14, x becomes random(ftol(x × 10)) × 0.1; past 45 his three
/// reputations become 1; past 41 they are kept at 0.875, 0.875 and 0.75 (current, home, world); with x under -3 at
/// 0.925, 0.925 and 0.825; else at 0.975^-x, 0.975^-x and 0.95^-x (the game's pow).
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d3d09..0x5d3eba.
unsafe fn development_age_factor(d: &mut Development, turning_age: i8) {
    use exe::utils_random as random;
    let pd = d.pd;
    let mut factor = (turning_age as i32 - d.age as i32) as f64;
    d.x = factor as f32;
    if factor > 0.0 {
        factor *= read_f64(rdata::F64_0_25);
        d.x = factor as f32;
    }
    if !(factor < 0.0) {
        return;
    }
    if random(read_i8(pd + player_sheet::NATURAL_FITNESS) as i32) > 14 {
        let r = random(ftol(d.x as f64 * read_f32(rdata::F32_10) as f64));
        d.x = (r as f64 * read_f32(rdata::F32_0_1000000015) as f64) as f32;
    }
    let keep = |field: usize, rate: f64| write_u16(pd + field, ftol(read_i16(pd + field) as f64 * rate) as u16);
    if d.age > 45 {
        covered!("gra_2c reputations of an old player drop to 1");
        write_u16(pd + player_sheet::CURRENT_REPUTATION, 1);
        write_u16(pd + player_sheet::HOME_REPUTATION, 1);
        write_u16(pd + player_sheet::WORLD_REPUTATION, 1);
    } else if d.age > 41 {
        keep(player_sheet::CURRENT_REPUTATION, read_f64(rdata::F64_0_875));
        keep(player_sheet::HOME_REPUTATION, read_f64(rdata::F64_0_875));
        keep(player_sheet::WORLD_REPUTATION, read_f64(rdata::F64_0_75));
    } else if (d.x as f64) < read_f32(F32_NEG_3_DEVELOPMENT) as f64 {
        keep(player_sheet::CURRENT_REPUTATION, read_f64(rdata::F64_0_925));
        keep(player_sheet::HOME_REPUTATION, read_f64(rdata::F64_0_925));
        keep(player_sheet::WORLD_REPUTATION, read_f64(rdata::F64_0_825));
    } else {
        let years = (-d.x) as f64;
        let rate = game_pow(read_f64(rdata::F64_0_975), years);
        keep(player_sheet::CURRENT_REPUTATION, rate);
        keep(player_sheet::HOME_REPUTATION, rate);
        keep(player_sheet::WORLD_REPUTATION, game_pow(read_f64(rdata::F64_0_95), years));
    }
}

/// The season's form behind a development day (0x5d3eba..0x5d4070). With the stats (`PlayerStats::get_stat`, 6.5 by
/// default) a = (6, 1), b = (6, 0x11), c = (1, 0x11) and d = (1, 1): t = (x × 25 + 50) × 0.01 × d, held to 0..100, and
/// s = t + a. The rating is the game's average - 0.25 when s is 0; else n = t × c + b × a, and n / s from s = 20 up,
/// below it ((n / s) × s + (20 - s) × the average) × 0.05. A developing player (x > 0) adds a × x × 0.01; his stamina
/// mark becomes 1 when random(10) + 5 is under t × 0.5 + a, and grows by one when a is not 0, his stamina is under 10
/// and his professionalism is above 5 and above random(26). Returns the rating and s (a float).
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d3eba..0x5d4070.
unsafe fn development_form(d: &mut Development, average: f64) -> (f64, f32) {
    use exe::utils_random as random;
    let a = sd_stat(d.p, 6, 1);
    let b = sd_stat(d.p, 6, 0x11);
    let c = sd_stat(d.p, 1, 0x11);
    let last = exe::playerstats_get_stat(read_usize(G_PLAYER_STATS), d.pd, 1, 1, 6.5);
    let t = (d.x as f64 * read_f64(rdata::F64_25) + read_f64(rdata::F64_50)) * read_f64(rdata::F64_0_01) * last;
    // fcomp: above 100 (or NaN) 100, below 0 nothing
    let t = if t.is_nan() || t > read_f64(rdata::F64_100) {
        read_f64(rdata::F64_100)
    } else if t < read_f64(rdata::F64_0) {
        read_f64(rdata::F64_0)
    } else {
        t
    };
    let part = t as f32;
    let s = t + a as f64;
    let mut rating = if s == 0.0 || s.is_nan() {
        average - read_f64(rdata::F64_0_25)
    } else {
        let n = part as f64 * c as f64 + b as f64 * a as f64;
        let twenty = read_f32(rdata::F32_20) as f64;
        if s < twenty {
            (n / s * s + (twenty - s) * average) * read_f64(rdata::F64_0_05)
        } else {
            n / s
        }
    };
    if d.x > 0.0 {
        let developed = (a as f64 * d.x as f64 * read_f64(rdata::F64_0_01) + rating) as f32;
        let threshold = (part as f64 * read_f32(rdata::F32_0_5) as f64 + a as f64) as f32;
        if ((random(10) + 5) as f64) < threshold as f64 {
            d.mark = 1;
        }
        let professionalism = read_i8(d.p + staff::PROFESSIONALISM);
        if a != 0.0 && !a.is_nan() && read_i8(d.pd + player_sheet::STAMINA) < 10 && professionalism > 5 && professionalism as i32 > random(0x1a) {
            d.mark = d.mark.wrapping_add(1);
        }
        rating = developed as f64;
    }
    (rating, s as f32)
}

/// The day's growth steps (0x5d4070..0x5d41f3). Under 22 the current and home reputations grow by 2.5% when s is
/// above 5, else by 1%. With r1 and r2 two draws of random(10), u = r1 × 0.05 - r2 × 0.05 (floats), v = (rating -
/// the average) × 4.5 and, for a developing player, w = potential² × 7.5e-5: the growth is the step
/// (`DEVELOPMENT_GROWTH_STEPS`) at ftol(w + u + v + x + 7), the physical step the one at ftol(u + x + 6.5) but at least
/// -8, both indexes held to 0..12.
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d4070..0x5d41f3.
unsafe fn development_growth_steps(d: &mut Development, rating: f64, s: f32, average: f64) {
    use exe::utils_random as random;
    let pd = d.pd;
    if d.age < 22 {
        let rate = if s as f64 > read_f32(rdata::F32_5) as f64 { read_f64(rdata::F64_1_025) } else { read_f64(rdata::F64_1_01) };
        write_u16(pd + player_sheet::CURRENT_REPUTATION, ftol(read_i16(pd + player_sheet::CURRENT_REPUTATION) as f64 * rate) as u16);
        write_u16(pd + player_sheet::HOME_REPUTATION, ftol(read_i16(pd + player_sheet::HOME_REPUTATION) as f64 * rate) as u16);
    }
    let v = ((rating - average) * read_f64(rdata::F64_4_5)) as f32;
    let first = (random(10) as f64 * read_f64(rdata::F64_0_05)) as f32;
    let u = (first as f64 - random(10) as f64 * read_f64(rdata::F64_0_05)) as f32;
    let w = if d.x as f64 > read_f64(rdata::F64_0) {
        sd_pow_double(read_i16(pd + player_sheet::POTENTIAL_ABILITY) as f64, 2.0) * read_f64(F64_POTENTIAL_SQUARED_SCALE)
    } else {
        read_f32(rdata::F32_0) as f64
    };
    let growth_at = ftol(w + u as f64 + v as f64 + d.x as f64 + read_f64(rdata::F64_7));
    let step_at = ftol(u as f64 + d.x as f64 + read_f64(F64_6_5_DEVELOPMENT));
    let step = |at: i32| read_i16(DEVELOPMENT_GROWTH_STEPS + 2 * (at as i16).clamp(0, 12) as usize);
    d.growth = step(growth_at);
    d.step = step(step_at).max(-8);
}

/// Fatigue and injury against the growth (0x5d41f3..0x5d4384). With the fatigue (`Injury::get_fatigue`) under -500 -
/// random(1000) the stamina mark drops by random(3) past 28 years old, by random(2) past 24. Otherwise, with the
/// fatigue above 750 + random(750): a developing player whose fatigue is also above 1000 + random(200) loses
/// random(ftol(x × 25)) of growth, held at -8 under 19, -5 under 23, -3 under 30, else -20; and the mark drops by
/// random(3). A current injury whose days (+0x2c) give random(days) > 30 sets the growth to random(-5) and the injury
/// flag; one time in ten the mark drops by random(2).
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d41f3..0x5d4384.
unsafe fn development_fatigue_and_injury(d: &mut Development) {
    use exe::utils_random as random;
    let id = read_u32(d.p);
    let fatigue = || exe::injury_get_fatigue(read_usize(G_INJURY), id) as i32;
    let limit = -500 - random(1000);
    if fatigue() < limit {
        if d.age > 28 {
            d.mark = d.mark.wrapping_sub(random(3) as i8);
        } else if d.age > 24 {
            d.mark = d.mark.wrapping_sub(random(2) as i8);
        }
    } else {
        let tired = fatigue();
        if tired > random(750) + 750 {
            if d.x > 0.0 {
                let very_tired = fatigue();
                if very_tired > random(200) + 1000 {
                    covered!("gra_2c tired youngster loses growth");
                    let cut = random(ftol(d.x as f64 * read_f32(rdata::F32_25) as f64));
                    d.growth = d.growth.wrapping_sub(cut as i16);
                    let floor = if d.age < 19 {
                        -8
                    } else if d.age < 23 {
                        -5
                    } else if d.age < 30 {
                        -3
                    } else {
                        -20
                    };
                    if d.growth < floor {
                        d.growth = floor;
                    }
                }
            }
            d.mark = d.mark.wrapping_sub(random(3) as i8);
        }
    }
    let injury = read_usize(G_INJURY);
    if injury == 0 || exe::injury_get_player_injury(injury, d.p) == 0 {
        return;
    }
    let days = read_i16(exe::injury_get_player_injury(read_usize(G_INJURY), d.p) + 0x2c);
    if random(days as i32) > 30 {
        covered!("gra_2c injury cuts the growth");
        d.growth = random(-5) as i16;
        if random(10) == 0 {
            d.mark = d.mark.wrapping_sub(random(2) as i8);
        }
        d.injured = true;
    }
}

/// A coach's rating for one player (0x5d4484..0x5d4540): his coaching (goalkeeping coaching for a goalkeeper 20),
/// plus random(3) when his motivating is above 15; with the player's professionalism under 6, -1 - random(5) when the
/// coach's discipline is under 6, else -1 under 15; with the player's pressure under 6 and the coach's discipline above
/// 10, -1 - random(5) when his man handling is under 6, else -1 under 15 (ratings by `non_player_attribute_rating`).
/// Without a player sheet the rating is 5.
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d4484..0x5d4540.
unsafe fn development_coach_rating(p: usize, np: usize) -> i8 {
    use exe::utils_random as random;
    let pd = read_usize(p + staff::PLAYER);
    if pd == 0 {
        return 5;
    }
    let coaching = if read_u8(pd + player_sheet::GOALKEEPER) == 20 { non_player::COACHING_GKS } else { non_player::COACHING };
    let mut rating = exe::non_player_attribute_rating(np, read_i8(np + coaching));
    if exe::non_player_attribute_rating(np, read_i8(np + non_player::MOTIVATING)) > 15 {
        rating = rating.wrapping_add(random(3) as i8);
    }
    if read_i8(p + staff::PROFESSIONALISM) < 6 {
        let discipline = read_i8(np + non_player::DISCIPLINE);
        if discipline < 6 {
            rating = rating.wrapping_add((-1 - random(5)) as i8);
        } else if discipline < 15 {
            rating = rating.wrapping_sub(1);
        }
    }
    if read_i8(p + staff::PRESSURE) < 6 && read_i8(np + non_player::DISCIPLINE) > 10 {
        if exe::non_player_attribute_rating(np, read_i8(np + non_player::MAN_HANDLING)) < 6 {
            rating = rating.wrapping_add((-1 - random(5)) as i8);
        } else if exe::non_player_attribute_rating(np, read_i8(np + non_player::MAN_HANDLING)) < 15 {
            rating = rating.wrapping_sub(1);
        }
    }
    rating
}

/// The training behind a positive growth (0x5d4384..0x5d46b1). Without a club job the growth is ftol(growth × 0.2).
/// At a club: the factor starts at ((training facilities / 26.6, or reputation / 13,300 without them) + 0.75) × 0.5;
/// a computer manager with a non-player sheet multiplies it by youngsters / 26.6 + 0.75, else it is × 1.31. Then the
/// assistant and the five coaches (an empty coach slot repeats the last coach seen: defect of the original), each with
/// a non-player sheet, multiply it by w / 53.2 + 0.875, w being (coach rating + 2 × youngsters) × 0.33 under 20, (2 ×
/// coach rating + youngsters) × 0.33 under 23, else the coach rating. The level (division reputation / 30.7, or club
/// reputation / 15,350 without a division, + 0.5, at most 1) multiplies it too; one time in five it is 1; held to
/// 0.1..3. The growth becomes ftol(growth × factor), within 5 of the old one, and 2..25.
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d4384..0x5d46b1.
unsafe fn development_training(d: &mut Development) {
    use exe::utils_random as random;
    if d.growth <= 0 {
        return;
    }
    let club = d.club;
    if club == 0 {
        d.growth = ftol(d.growth as f64 * read_f64(rdata::F64_0_2)) as i16;
        return;
    }
    covered!("gra_2c training factor");
    let facilities = if read_u8(club + club::TRAINING) != 0 {
        read_i8(club + club::TRAINING) as f64 * read_f64(F64_TRAINING_SCALE)
    } else {
        read_i16(club + club::REPUTATION) as f64 * read_f64(F64_CLUB_REPUTATION_TRAINING_SCALE)
    };
    let mut factor = ((facilities + read_f64(rdata::F64_0_75)) * read_f32(rdata::F32_0_5) as f64) as f32;
    let manager = read_usize(club + club::MANAGER);
    if exe::human_manager_is_human_club(club, 1, 1, 0, 0) == 0 && manager != 0 && read_usize(manager + staff::NON_PLAYER) != 0 {
        let youngsters = read_i8(read_usize(manager + staff::NON_PLAYER) + non_player::YOUNGSTERS);
        factor = ((youngsters as f64 * read_f64(F64_TRAINING_SCALE) + read_f64(rdata::F64_0_75)) * factor as f64) as f32;
    } else {
        factor = (factor as f64 * read_f32(F32_1_31_HUMAN_MANAGER) as f64) as f32;
    }
    let mut coach = 0usize;
    for slot in -1i32..5 {
        if slot < 0 {
            coach = read_usize(club + club::ASSISTANT_MANAGER);
        } else {
            let next = read_usize(club + club::COACHES + 4 * slot as usize);
            if next != 0 {
                coach = next;
            }
        }
        if coach == 0 || read_usize(coach + staff::NON_PLAYER) == 0 {
            continue;
        }
        let np = read_usize(coach + staff::NON_PLAYER);
        let rating = development_coach_rating(d.p, np) as i32;
        let youngsters = read_i8(np + non_player::YOUNGSTERS) as i32;
        let w = if d.age < 20 {
            (rating + youngsters * 2) as f64 * read_f64(rdata::F64_0_33)
        } else if d.age < 23 {
            (youngsters + rating * 2) as f64 * read_f64(rdata::F64_0_33)
        } else {
            rating as f64
        };
        factor = ((w * read_f64(F64_COACH_SCALE) + read_f64(rdata::F64_0_875)) * factor as f64) as f32;
    }
    let division = read_usize(club + club::DIVISION);
    let level = if division != 0 {
        read_i16(division + competition::REPUTATION) as f64 * read_f64(F64_DIVISION_LEVEL_SCALE)
    } else {
        read_i16(club + club::REPUTATION) as f64 * read_f64(F64_CLUB_LEVEL_SCALE)
    } + read_f64(rdata::F64_0_5);
    let level = if level > read_f64(rdata::F64_1) { read_f32(rdata::F32_1) as f64 } else { level };
    let mut factor = (level * factor as f64) as f32;
    if random(5) == 0 {
        factor = 1.0;
    }
    let factor = if !(factor as f64 >= read_f64(rdata::F64_0_1)) {
        read_f64(rdata::F64_0_1)
    } else if factor as f64 > read_f64(rdata::F64_3) {
        read_f64(rdata::F64_3)
    } else {
        factor as f64
    };
    let growth = d.growth as i32;
    let mut trained = ftol(growth as f64 * factor) as i16 as i32;
    if growth - 5 > trained {
        trained = growth - 5;
    } else if growth + 5 < trained {
        trained = growth + 5;
    }
    d.growth = (trained as i16).clamp(2, 25);
}

/// The professionalism against the growth (0x5d46b1..0x5d4800), with k = id % 4 (C remainder): under 24 + k years old a
/// negative growth becomes random(growth) unless random(professionalism) is 0; else under 30 with professionalism above
/// 10 and random(professionalism) above random(200), the growth becomes at least random(20) + growth (low bytes added),
/// at most 25; else past 26 + k with professionalism under 6 and random(professionalism + 6) twice 0, the growth
/// becomes at most growth - random(20) (low bytes), at least -25.
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d46b1..0x5d4800.
unsafe fn development_professionalism(d: &mut Development) {
    use exe::utils_random as random;
    let k = read_i32(d.p) % 4;
    let age = d.age as i32;
    let professionalism = || read_i8(d.p + staff::PROFESSIONALISM) as i32;
    if age < 24 + k && d.growth < 0 && random(professionalism()) != 0 {
        d.growth = random(d.growth as i32) as i16;
        return;
    }
    if d.age < 30 && professionalism() > 10 && random(professionalism()) > random(200) {
        covered!("gra_2c professional player grows more");
        let sum = (random(20) as u8).wrapping_add(d.growth as u8) as i8;
        if d.growth as i32 <= (sum as i32).min(25) {
            d.growth = (sum as i16).min(25);
        }
        return;
    }
    if age > 26 + k && professionalism() < 6 && random(professionalism() + 6) == 0 && random(professionalism() + 6) == 0 {
        covered!("gra_2c unprofessional veteran grows less");
        let difference = (d.growth as u8).wrapping_sub(random(20) as u8) as i8;
        if d.growth as i32 >= (difference as i32).max(-25) {
            d.growth = (difference as i16).max(-25);
        }
    }
}

/// A tired young player's fall is limited (0x5d4800..0x5d488d): under 30, without the injury flag, at a club job, with
/// the fatigue above 1050 and a negative growth whose size passes 5% of the current ability, the growth becomes
/// ftol(-ability × 0.05), at most -1.
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d4800..0x5d488d.
unsafe fn development_fall_limit(d: &mut Development) {
    if d.age >= 30 || d.injured || d.club == 0 || read_usize(G_INJURY) == 0 {
        return;
    }
    if exe::injury_get_fatigue(read_usize(G_INJURY), read_u32(d.p)) <= 1050 || d.growth >= 0 {
        return;
    }
    let ability = read_i16(d.pd + player_sheet::CURRENT_ABILITY) as i32;
    if ability as f64 * read_f64(rdata::F64_0_05) < d.growth as f64 * read_f64(rdata::F64_NEG_1) {
        covered!("gra_2c tired player's fall limited");
        d.growth = (ftol(-ability as f64 * read_f64(rdata::F64_0_05)) as i16).min(-1);
    }
}

/// The current ability moves (0x5d488d..0x5d4996): it gains random(growth) (16 bits). At a club job in a division
/// whose reputation × 500 + the club's reputation is under 12,000, a rise is kept unless random(5) is 0; otherwise,
/// above the ceiling ftol((division reputation² × 25 + club reputation) × 0.01) (the game's pow) the ability becomes
/// ftol(ceiling × 0.1) - ftol(ability × -0.9) (16 bits), never below the ability of before. Returns the ability of
/// before.
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d488d..0x5d4996.
unsafe fn development_ability(d: &mut Development) -> i16 {
    use exe::utils_random as random;
    let ability = d.pd + player_sheet::CURRENT_ABILITY;
    d.saved_growth = d.growth as i32;
    let before = read_i16(ability);
    let rise = random(d.growth as i32);
    write_u16(ability, read_u16(ability).wrapping_add(rise as u16));
    let club = d.club;
    if club == 0 {
        return before;
    }
    let division = read_usize(club + club::DIVISION);
    if division == 0 {
        return before;
    }
    let division_reputation = read_i16(division + competition::REPUTATION) as i32;
    if (read_i16(club + club::REPUTATION) as i32).wrapping_add(division_reputation.wrapping_mul(500)) >= 12000
        || read_i16(ability) <= before
        || random(5) == 0
    {
        return before;
    }
    let ceiling = ftol(
        (game_pow(division_reputation as f64, read_f64(rdata::F64_2)) * read_f64(rdata::F64_25) + read_i16(club + club::REPUTATION) as f64)
            * read_f64(rdata::F64_0_01),
    ) as i16;
    if read_i16(ability) > ceiling {
        covered!("gra_2c weak division caps the rise");
        let kept = ftol(read_i16(ability) as f64 * read_f64(F64_NEG_0_9_DEVELOPMENT)) as i16;
        let capped = (ftol(ceiling as f64 * read_f64(rdata::F64_0_1)) as i16).wrapping_sub(kept);
        write_i16(ability, if capped < before { before } else { capped });
    }
    before
}

/// A big rise is praised (0x5d4996..0x5d4ad2): when the ability rose by more than random(20) to above 80 at a club job
/// run by a human manager (`news_club_has_human_manager`), with the unrest counter under 10, an assistant, the injury
/// object, no injury, not AWOL, and a first team contract (type & 0x3f = 2) with the happiness below 0 or the unhappy
/// count above 5: when the selection score (`match_man_score_player_for_selection`: player, -1, 0x41e88, club, -1,
/// assistant) passes 85% of the club's target rating (`player_search_get_club_target_rating`), the assistant's
/// training report praises him (`news_post_training_report`, 1).
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d4996..0x5d4ad2.
unsafe fn development_praise(d: &Development, before: i16) {
    let (p, club) = (d.p, d.club);
    let ability = read_i16(d.pd + player_sheet::CURRENT_ABILITY);
    if ability as i32 - before as i32 <= exe::utils_random(20) || ability <= 80 || club == 0 {
        return;
    }
    if exe::news_club_has_human_manager(read_usize(G_NEWS), club) == 0
        || read_u8(read_usize(G_STAFF_STATE_TABLE) + (read_u32(p) as usize).wrapping_mul(staff_state::SIZE) + staff_state::INJURY_COUNTER) >= 10
        || read_usize(club + club::ASSISTANT_MANAGER) == 0
        || read_usize(G_INJURY) == 0
        || exe::injury_get_player_injury(read_usize(G_INJURY), p) != 0
        || exe::awol_is_player_awol(read_usize(G_AWOL), p, 0) != 0
    {
        return;
    }
    let contract = exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), p, club);
    if contract == 0 || read_u8(contract + contract::CONTRACT_TYPE) & 0x3f != 2 {
        return;
    }
    if read_i8(contract + contract::HAPPINESS) >= 0 && read_i8(contract + contract::UNHAPPY_FLAG) <= 5 {
        return;
    }
    let score = exe::match_man_score_player_for_selection(p, u32::MAX, 0x41e88, club, -1, read_usize(club + club::ASSISTANT_MANAGER));
    let target_rating: unsafe extern "C" fn(usize, usize, usize) -> i32 = core::mem::transmute(FN_PLAYER_SEARCH_GET_CLUB_TARGET_RATING_GRA);
    let target = target_rating(club, 0, 0);
    if (target as f64 * read_f64(rdata::F64_0_85)) < score as f64 {
        covered!("gra_2c training report praises a big rise");
        let f: unsafe extern "C" fn(usize, usize, usize, u32) = core::mem::transmute(FN_NEWS_POST_TRAINING_REPORT);
        f(p, read_usize(club + club::ASSISTANT_MANAGER), club, 1);
    }
}

/// A declining player's physical attributes (0x5d4b24..0x5d4daf), with the step a and the divisor q = (natural
/// fitness × 0.4 + 4) × 50 × 0.01, at least 1: acceleration, agility, jumping, natural fitness, pace, stamina (plus the
/// stamina mark) and strength each move by ftol(random(a) / q), held to 1..20; reflexes by random(saved growth), held
/// to -125..125; vision between 1 and 99 becomes the larger of × 1.1 and + 5; consistency moves by random(2) when
/// random(professionalism) beats random(50); positioning under 100 grows like vision when the defender rating beats
/// random(5) + 15.
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d4b24..0x5d4daf.
unsafe fn development_decline_attributes(d: &Development) {
    use exe::utils_random as random;
    let pd = d.pd;
    let q = (read_i8(pd + player_sheet::NATURAL_FITNESS) as f64 * read_f64(rdata::F64_0_4) + read_f64(rdata::F64_4))
        * read_f64(rdata::F64_50)
        * read_f64(rdata::F64_0_01);
    let divisor = if q < read_f64(rdata::F64_1) { 1.0f32 } else { q as f32 };
    let step = d.step as i32;
    for field in [player_sheet::ACCELERATION, player_sheet::AGILITY, player_sheet::JUMPING, player_sheet::NATURAL_FITNESS, player_sheet::PLAYER_PACE] {
        let r = random(step);
        sd_clamp_attribute(pd, field, ftol(r as f64 / divisor as f64), 1, 20);
    }
    let r = random(step);
    sd_clamp_attribute(pd, player_sheet::STAMINA, ftol(r as f64 / divisor as f64 + d.mark as i32 as f64), 1, 20);
    let r = random(step);
    sd_clamp_attribute(pd, player_sheet::STRENGTH, ftol(r as f64 / divisor as f64), 1, 20);
    sd_clamp_attribute(pd, player_sheet::REFLEXES, random(d.saved_growth), -0x7d, 0x7d);
    sd_grow_ten_percent(pd, player_sheet::VISION, true);
    if random(read_i8(d.p + staff::PROFESSIONALISM) as i32) > random(50) {
        sd_clamp_attribute(pd, player_sheet::CONSISTENCY, random(2), 1, 20);
    }
    if read_i8(pd + player_sheet::DEFENDER) as i32 > random(5) + 15 {
        sd_grow_ten_percent(pd, player_sheet::POSITIONING, false);
    }
}

/// An attribute byte under 100 (and above 0 when `positive`) becomes ftol of the larger of × 1.1 and + 5.
unsafe fn sd_grow_ten_percent(pd: usize, field: usize, positive: bool) {
    let v = read_i8(pd + field);
    if v >= 100 || (positive && v <= 0) {
        return;
    }
    let tenth = v as f64 * read_f64(rdata::F64_1_1);
    let five = (v as i32 + 5) as f64;
    write_u8(pd + field, ftol(if tenth > five { tenth } else { five }) as u8);
}

/// A strongly developing player's mental attributes (0x5d4dc9..0x5d4f5e, x above 1), with the step a: consistency,
/// versatility and important matches move by random(3a) / 10 and leadership by random((random(2) + 2) × a) / 10, held
/// to 1..20. The stamina mark grows by one when the training entry's level (signed low nibble of byte 3,
/// `training_manager_get_player_entry`) beats random(4) + 3, and a negative mark is cleared at a club job whose training
/// facilities beat random(31). Stamina (only while under natural fitness + 5) and strength move by random(3a) / ((5 -
/// mark) × 2).
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d4dc9..0x5d4f5e.
unsafe fn development_strong_attributes(d: &mut Development) {
    use exe::utils_random as random;
    let pd = d.pd;
    let step = d.step as i32;
    let three = step * 3;
    sd_clamp_attribute(pd, player_sheet::CONSISTENCY, random(three) / 10, 1, 20);
    let lead = random((random(2) + 2).wrapping_mul(step));
    sd_clamp_attribute(pd, player_sheet::LEADERSHIP, lead / 10, 1, 20);
    sd_clamp_attribute(pd, player_sheet::VERSATILITY, random(three) / 10, 1, 20);
    sd_clamp_attribute(pd, player_sheet::IMPORTANT_MATCHES, random(three) / 10, 1, 20);
    let entry_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_TRAINING_MANAGER_GET_PLAYER_ENTRY_GRA);
    let entry = entry_of(read_usize(G_TRAINING_MANAGER), d.p);
    let level = ((read_u8(entry + 3) << 4) as i8 >> 4) as i32;
    if level > random(4) + 3 {
        d.mark = d.mark.wrapping_add(1);
    }
    if d.club != 0 && read_i8(d.club + club::TRAINING) as i32 > random(0x1f) && d.mark < 0 {
        d.mark = 0;
    }
    // (5 - mark) x 2 is never 0: the mark stays far below 5 here
    if (read_i8(pd + player_sheet::STAMINA) as i32) < read_i8(pd + player_sheet::NATURAL_FITNESS) as i32 + 5 {
        let r = random(three);
        sd_clamp_attribute(pd, player_sheet::STAMINA, r / ((5 - d.mark as i32) * 2), 1, 20);
    }
    let r = random(three);
    sd_clamp_attribute(pd, player_sheet::STRENGTH, r / ((5 - d.mark as i32) * 2), 1, 20);
}

/// A mildly developing player (0x5d4f63..0x5d50ab, x from 0 to 1). A negative stamina mark is cleared when the natural
/// fitness passes stamina + 2 or random(natural fitness) beats random(20); else it gains random(mark + 1). A positive
/// mark with stamina above 15 becomes random(mark). The stamina moves by the mark (1..20); above natural fitness + 5 it
/// loses random(2) when random(professionalism) is 0. With the step a: consistency and versatility move by random(2a)
/// / 10, leadership by random(2a) / (random(6) + 10), held to 1..20.
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d4f63..0x5d50ab.
unsafe fn development_mild_attributes(d: &mut Development) {
    use exe::utils_random as random;
    let pd = d.pd;
    let stamina = || read_i8(pd + player_sheet::STAMINA) as i32;
    let fitness = || read_i8(pd + player_sheet::NATURAL_FITNESS) as i32;
    if d.mark < 0 {
        if fitness() > stamina() + 2 || random(fitness()) > random(20) {
            d.mark = 0;
        } else {
            d.mark = d.mark.wrapping_add(random(d.mark as i32 + 1) as i8);
            if d.mark > 0 && stamina() > 15 {
                d.mark = random(d.mark as i32) as i8;
            }
        }
    } else if d.mark > 0 && stamina() > 15 {
        d.mark = random(d.mark as i32) as i8;
    }
    sd_clamp_attribute(pd, player_sheet::STAMINA, d.mark as i32, 1, 20);
    if stamina() > fitness() + 5 && random(read_i8(d.p + staff::PROFESSIONALISM) as i32) == 0 {
        write_u8(pd + player_sheet::STAMINA, read_u8(pd + player_sheet::STAMINA).wrapping_sub(random(2) as u8));
    }
    let two = (d.step as i32) * 2;
    sd_clamp_attribute(pd, player_sheet::CONSISTENCY, random(two) / 10, 1, 20);
    let r = random(two);
    let lead = r / (random(6) + 10);
    sd_clamp_attribute(pd, player_sheet::LEADERSHIP, lead, 1, 20);
    sd_clamp_attribute(pd, player_sheet::VERSATILITY, random(two) / 10, 1, 20);
}

/// Anticipation and decisions of a developing player (0x5d50ab..0x5d5115): each one moves by random(saved growth),
/// held to -125..125, when random(saved growth) beats random(100).
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d50ab..0x5d5115.
unsafe fn development_reading_of_the_game(d: &Development) {
    use exe::utils_random as random;
    for field in [player_sheet::ANTICIPATION, player_sheet::DECISIONS] {
        if random(d.saved_growth) > random(100) {
            sd_clamp_attribute(d.pd, field, random(d.saved_growth), -0x7d, 0x7d);
        }
    }
}

/// An attribute byte loses one when it is above `above` and random(3) is 0.
unsafe fn sd_maybe_lose_one(pd: usize, field: usize, above: i8) {
    if read_i8(pd + field) > above && exe::utils_random(3) == 0 {
        write_u8(pd + field, read_u8(pd + field).wrapping_sub(1));
    }
}

/// An attribute byte loses random(`n`) when it is above `high`, else one when it is above `low` and random(3) is 0.
unsafe fn sd_lose_some(pd: usize, field: usize, high: i8, n: i32, low: i8) {
    if read_i8(pd + field) > high {
        write_u8(pd + field, read_u8(pd + field).wrapping_sub(exe::utils_random(n) as u8));
    } else {
        sd_maybe_lose_one(pd, field, low);
    }
}

/// A forward (or attacking midfielder) rated 20 past `ability_over` loses random(`n`) of current ability when
/// random(ability) beats professionalism + stamina + `margin`.
unsafe fn sd_veteran_ability_loss(p: usize, pd: usize, ability_over: i16, margin: i32, n: i32) {
    let ability = read_i16(pd + player_sheet::CURRENT_ABILITY);
    if ability <= ability_over {
        return;
    }
    if exe::utils_random(ability as i32) > read_i8(p + staff::PROFESSIONALISM) as i32 + read_i8(pd + player_sheet::STAMINA) as i32 + margin {
        let loss = exe::utils_random(n);
        write_u16(pd + player_sheet::CURRENT_ABILITY, read_u16(pd + player_sheet::CURRENT_ABILITY).wrapping_sub(loss as u16));
    }
}

/// A 30-year-old's or older player's ageing (0x5d5115..0x5d5771), by the age byte.
/// Past 37: a forward rated 20 (attacking midfielder or attacker) over 50 ability loses random(20) when random(ability)
/// beats professionalism + stamina + 10, a defender, midfielder, sweeper or defensive midfielder rated 20 random(10)
/// against professionalism + stamina; jumping loses random(3) above 15, else one in three above 5 random(2);
/// acceleration and pace lose 1 + random(3) above 8 and then one in three above 4; agility one in three above 10,
/// natural fitness above 4; stamina and strength random(3) above 7, else one in three above 4.
/// 35 to 37: forwards rated 20 lose random(10) like the over-37 ones; jumping random(2) above 17, else one in three
/// random(2) above 10; acceleration random(3) above 15, else one in three above 5; pace random(3) above 15 and then one
/// in three above 5; agility one in three above 10, natural fitness above 8; stamina and strength random(3) above 15,
/// else one in three above 8 (strength above 10).
/// 33 and 34: a defender or defensive midfielder rated 20, or any goalkeeper rating, with professionalism above
/// random(11) + 10, work rate above random(15) + 6 and random(professionalism) not 0, gains random(2) of anticipation
/// and of positioning; forwards rated 20 over 100 ability lose random(6) when random(ability) beats professionalism +
/// stamina + 50; acceleration and pace lose random(2) above 15, else one in three above 10; agility one in three at
/// 20, natural fitness above 15, stamina above 18; strength at 20 loses one unless random(3) is 0 (two times in three,
/// against one in three everywhere else: [inference] a defect of the original).
/// 30 to 32: a defender, defensive midfielder, goalkeeper or sweeper rated 20 with professionalism above random(11) +
/// 10, work rate above random(10) + 6 and random(professionalism) not 0 gains random(2) of anticipation and of
/// positioning while they are under 15; an attacker, attacking midfielder or midfielder rated 20 with professionalism
/// above random(6) + 15, work rate above random(11) + 10 and random(professionalism) not 0 gains random(2) of
/// anticipation under 15; acceleration and pace lose one in three above 18.
/// The gains are `clamp_char_sum` -125..125.
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d5115..0x5d5771.
unsafe fn development_ageing(p: usize, pd: usize) {
    use exe::utils_random as random;
    let age = read_i8(p + staff::AGE);
    let rated_20 = |field: usize| read_u8(pd + field) == 20;
    let forward = || rated_20(player_sheet::ATTACKING_MIDFIELDER) || rated_20(player_sheet::ATTACKER);
    let gain = |field: usize| {
        let r = random(2);
        if r != 0 {
            sd_clamp_attribute(pd, field, r, -0x7d, 0x7d);
        }
    };
    if age < 30 {
        return;
    }
    if age > 37 {
        covered!("gra_2c ageing past 37");
        if forward() {
            sd_veteran_ability_loss(p, pd, 50, 10, 20);
        } else if rated_20(player_sheet::DEFENDER) || rated_20(player_sheet::MIDFIELDER) || rated_20(player_sheet::SWEEPER) || rated_20(player_sheet::DEFENSIVE_MIDFIELDER) {
            sd_veteran_ability_loss(p, pd, 50, 0, 10);
        }
        if read_i8(pd + player_sheet::JUMPING) > 15 {
            write_u8(pd + player_sheet::JUMPING, read_u8(pd + player_sheet::JUMPING).wrapping_sub(random(3) as u8));
        } else if read_i8(pd + player_sheet::JUMPING) > 5 && random(3) == 0 {
            write_u8(pd + player_sheet::JUMPING, read_u8(pd + player_sheet::JUMPING).wrapping_sub(random(2) as u8));
        }
        for field in [player_sheet::ACCELERATION, player_sheet::PLAYER_PACE] {
            if read_i8(pd + field) > 8 {
                write_u8(pd + field, read_u8(pd + field).wrapping_add((-1 - random(3)) as u8));
            }
            sd_maybe_lose_one(pd, field, 4);
        }
        sd_maybe_lose_one(pd, player_sheet::AGILITY, 10);
        sd_maybe_lose_one(pd, player_sheet::NATURAL_FITNESS, 4);
        sd_lose_some(pd, player_sheet::STAMINA, 7, 3, 4);
        sd_lose_some(pd, player_sheet::STRENGTH, 7, 3, 4);
    } else if age > 34 {
        if forward() {
            sd_veteran_ability_loss(p, pd, 50, 10, 10);
        }
        let jumping = read_i8(pd + player_sheet::JUMPING);
        if jumping > 17 || (jumping > 10 && random(3) == 0) {
            write_u8(pd + player_sheet::JUMPING, read_u8(pd + player_sheet::JUMPING).wrapping_sub(random(2) as u8));
        }
        sd_lose_some(pd, player_sheet::ACCELERATION, 15, 3, 5);
        if read_i8(pd + player_sheet::PLAYER_PACE) > 15 {
            write_u8(pd + player_sheet::PLAYER_PACE, read_u8(pd + player_sheet::PLAYER_PACE).wrapping_sub(random(3) as u8));
        }
        sd_maybe_lose_one(pd, player_sheet::PLAYER_PACE, 5);
        sd_maybe_lose_one(pd, player_sheet::AGILITY, 10);
        sd_maybe_lose_one(pd, player_sheet::NATURAL_FITNESS, 8);
        sd_lose_some(pd, player_sheet::STAMINA, 15, 3, 8);
        sd_lose_some(pd, player_sheet::STRENGTH, 15, 3, 10);
    } else if age > 32 {
        if (rated_20(player_sheet::DEFENDER) || rated_20(player_sheet::DEFENSIVE_MIDFIELDER) || read_u8(pd + player_sheet::GOALKEEPER) != 0)
            && read_i8(p + staff::PROFESSIONALISM) as i32 > random(11) + 10
            && read_i8(pd + player_sheet::WORK_RATE) as i32 > random(15) + 6
            && random(read_i8(p + staff::PROFESSIONALISM) as i32) != 0
        {
            gain(player_sheet::ANTICIPATION);
            gain(player_sheet::POSITIONING);
        }
        if forward() {
            sd_veteran_ability_loss(p, pd, 100, 50, 6);
        }
        sd_lose_some(pd, player_sheet::ACCELERATION, 15, 2, 10);
        sd_lose_some(pd, player_sheet::PLAYER_PACE, 15, 2, 10);
        if rated_20(player_sheet::AGILITY) && random(3) == 0 {
            write_u8(pd + player_sheet::AGILITY, read_u8(pd + player_sheet::AGILITY).wrapping_sub(1));
        }
        sd_maybe_lose_one(pd, player_sheet::NATURAL_FITNESS, 15);
        sd_maybe_lose_one(pd, player_sheet::STAMINA, 18);
        // defect of the original [inference]: strength at 20 loses one unless random(3) is 0
        if rated_20(player_sheet::STRENGTH) && random(3) != 0 {
            write_u8(pd + player_sheet::STRENGTH, read_u8(pd + player_sheet::STRENGTH).wrapping_sub(1));
        }
    } else {
        covered!("gra_2c ageing 30 to 32");
        let professionalism = || read_i8(p + staff::PROFESSIONALISM) as i32;
        let work_rate = || read_i8(pd + player_sheet::WORK_RATE) as i32;
        if rated_20(player_sheet::DEFENDER) || rated_20(player_sheet::DEFENSIVE_MIDFIELDER) || rated_20(player_sheet::GOALKEEPER) || rated_20(player_sheet::SWEEPER) {
            if professionalism() > random(11) + 10 && work_rate() > random(10) + 6 && random(professionalism()) != 0 {
                if read_i8(pd + player_sheet::ANTICIPATION) < 15 {
                    gain(player_sheet::ANTICIPATION);
                }
                if read_i8(pd + player_sheet::POSITIONING) < 15 {
                    gain(player_sheet::POSITIONING);
                }
            }
        } else if (rated_20(player_sheet::ATTACKER) || rated_20(player_sheet::ATTACKING_MIDFIELDER) || rated_20(player_sheet::MIDFIELDER))
            && professionalism() > random(6) + 15
            && work_rate() > random(11) + 10
            && random(professionalism()) != 0
            && read_i8(pd + player_sheet::ANTICIPATION) < 15
        {
            gain(player_sheet::ANTICIPATION);
        }
        sd_maybe_lose_one(pd, player_sheet::ACCELERATION, 18);
        sd_maybe_lose_one(pd, player_sheet::PLAYER_PACE, 18);
    }
}

/// One development day of a player (0x5d3d09..0x5d5771): the age factor, the form, the growth steps, fatigue and
/// injury, the training, the professionalism, the limited fall, the ability (held to the potential), the praise of a
/// big rise; the age factor x becomes random(ftol(x)) above -3; then the attributes of a declining (x < 0), strongly
/// developing (x > 1) or mildly developing player, anticipation and decisions for the developing ones, and the
/// ageing. Returns the day's growth (not 0: the day changed some ability).
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d3d09..0x5d5771.
unsafe fn development_day(p: usize, pd: usize, club: usize, age: i8, turning_age: i8, average: f64) -> i16 {
    covered!("gra_2c development day");
    let mut d = Development { p, pd, club, age, x: 0.0, mark: 0, injured: false, growth: 0, step: 0, saved_growth: 0 };
    development_age_factor(&mut d, turning_age);
    let (rating, s) = development_form(&mut d, average);
    development_growth_steps(&mut d, rating, s, average);
    development_fatigue_and_injury(&mut d);
    development_training(&mut d);
    development_professionalism(&mut d);
    development_fall_limit(&mut d);
    let before = development_ability(&mut d);
    development_praise(&d, before);
    let potential = read_i16(pd + player_sheet::POTENTIAL_ABILITY);
    if read_i16(pd + player_sheet::CURRENT_ABILITY) > potential {
        write_i16(pd + player_sheet::CURRENT_ABILITY, potential);
    }
    if d.x as f64 > read_f32(F32_NEG_3_DEVELOPMENT) as f64 {
        d.x = exe::utils_random(ftol(d.x as f64)) as f32;
    }
    if d.x < 0.0 {
        development_decline_attributes(&d);
    } else {
        if d.x as f64 > read_f64(rdata::F64_1) {
            development_strong_attributes(&mut d);
        } else {
            development_mild_attributes(&mut d);
        }
        development_reading_of_the_game(&d);
    }
    development_ageing(p, pd);
    d.growth
}

// ---------- the routine: the loop over the staff table ----------

/// `Injury::daily_wear` (0x62c2f0, thiscall: the person's id).
const FN_INJURY_DAILY_WEAR_GRA: usize = 0x62c2f0;

/// Today (`G_CURRENT_DATE`, 16 bits) is one of the person's three days of the year: id % 365 and the two 120 days
/// after it (each % 365, 16 bits).
unsafe fn sd_is_development_day(id: i32) -> bool {
    let first = (id % 365) as i16;
    let second = ((first as i32 + 120) % 365) as i16;
    let third = ((second as i32 + 120) % 365) as i16;
    let today = read_u32(G_CURRENT_DATE) as i16;
    today == first || today == second || today == third
}

/// One person's day (0x5ce33c..0x5d5864). Skipped without a nation or once retired (state bits 0x30 = 0x20). The club
/// is the club of the contract found (`game_get_contract_and_store`), or the club job for the jobs 5 and 12; the club
/// job's linked B team is looked up (`club_get_linked_b_team`, 1); a club without a nation counts as none, else the
/// contract is the one with that club (`contract_manager_get_contract_for_club`). Without a player sheet the
/// non-player day (`staff_day_non_player`). A player wears (`Injury::daily_wear`) and has his day
/// (`staff_day_player`) on one of his three days of the year, while his contract's club plays in a loaded division
/// (`G_COMP_OBJECT_TABLE`), with current reputation 4750 or more, or when (id % 7) + today is 0; then, without a
/// club, the unattached decline; the training condition; on his three days of the year the ability tables, the
/// game's average (`game_average`), his turning age, the personality drift, the development and the normalisation
/// (`staff_normalise_player_attributes`, `staff_fade_staff_relationships`); and every day the shortlist check. Returns
/// whether a development moved the growth.
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5ce33c..0x5d5864.
unsafe fn staff_day_person(p: usize, run: &mut StaffDayRun) -> bool {
    if read_usize(p + staff::NATION) == 0 {
        return false;
    }
    let id = read_i32(p);
    let state_off = (id as usize).wrapping_mul(staff_state::SIZE);
    if read_u8(read_usize(G_STAFF_STATE_TABLE) + state_off + staff_state::STATUS_FLAGS) & 0x30 == 0x20 {
        return false;
    }
    let pd = read_usize(p + staff::PLAYER);
    let found = game_get_contract_and_store(p);
    let mut club = if found != 0 { read_usize(G_CLUB_TABLE) + read_u32(found + contract::CLUB) as usize * club::SIZE } else { 0 };
    if club == 0 && matches!(read_u8(p + staff::JOB_FOR_CLUB), 5 | 12) {
        club = read_usize(p + staff::CLUB_JOB);
    }
    let mut b_team = 0u32;
    if read_usize(p + staff::CLUB_JOB) != 0 {
        exe::club_get_linked_b_team(read_usize(p + staff::CLUB_JOB), &mut b_team, 1);
    }
    let mut contract = 0;
    if club != 0 {
        if read_usize(club + club::NATION) != 0 {
            contract = exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), p, club);
        } else {
            club = 0;
        }
    }
    if pd == 0 {
        staff_day_non_player(p, club, contract);
        return false;
    }
    let wear: unsafe extern "thiscall" fn(usize, i32) = core::mem::transmute(FN_INJURY_DAILY_WEAR_GRA);
    wear(read_usize(G_INJURY), id);
    let division = if club != 0 { read_usize(club + club::DIVISION) } else { 0 };
    let in_play = contract != 0
        && division != 0
        && read_usize(G_COMP_OBJECT_TABLE) != 0
        && read_usize(read_usize(G_COMP_OBJECT_TABLE) + 4 * read_u32(division) as usize) != 0;
    if !sd_is_development_day(id)
        && !in_play
        && read_i16(pd + player_sheet::CURRENT_REPUTATION) < 0x128e
        && (id % 7).wrapping_add(read_i16(G_CURRENT_DATE) as i32) != 0
    {
        return false;
    }
    covered!("gra_2c player's day");
    let mut d = StaffDay { p, state_off, pd, club, contract, mate: 0, away_from_job: false, odds: 0, b_team };
    staff_day_player(&mut d, run);
    if club == 0 {
        staff_day_unattached_decline(p, pd);
    }
    staff_day_training_condition(p, club, contract);
    let mut changed = false;
    if sd_is_development_day(id) {
        staff_day_build_ability_tables();
        let average: unsafe extern "thiscall" fn(usize, u32) -> f64 = core::mem::transmute(FN_GAME_AVERAGE_GRA);
        let average = average(read_usize(G_PLAYER_STATS), 0);
        let sheet = read_usize(p + staff::PLAYER);
        if sheet == 0 {
            // dead: the player sheet was there at the start of the day
            show_error_at(GAME_CPP_PATH, 0x1901);
        } else {
            let club_job = read_usize(p + staff::CLUB_JOB);
            let age = read_i8(p + staff::AGE);
            let turning_age = staff_day_development_age(sheet);
            staff_day_personality_drift(p, age, club_job);
            let growth = development_day(p, sheet, club_job, age, turning_age, average);
            exe::staff_normalise_player_attributes(p);
            let fade: unsafe extern "C" fn(usize) = core::mem::transmute(FN_STAFF_FADE_STAFF_RELATIONSHIPS_GRA);
            fade(p);
            changed = growth != 0;
        }
    }
    staff_day_shortlist_check(p, pd, club, contract);
    changed
}

/// The daily update of every person: one day of a player's or a non-player's life (`staff_day_person`) for each
/// record of the staff table but the last (defect of the original: the end is base + (n - 1) × 0x6e with `<`, so the
/// last person never has the day); when a development moved some growth, the day's abilities are handed back in order
/// (`staff_day_hand_back_abilities`). The original builds an empty LLIST on its frame and destroys it at the end,
/// never using it: the copy has none. The favoured team mate's bonus starts at 0 (the original reads stack residue).
/// Defects of the original: the last person skipped; the uninitialised bonus; the gates `(id % 7) + day == 0` and the
/// like, which only open for id 0; an empty coach slot counted twice; in the ageing at 33 and 34, strength at 20 lost
/// two times in three.
/// Original: `game_daily_staff_update` (0x5ce2d0, game.cpp, cdecl).
pub(crate) unsafe extern "C" fn game_daily_staff_update() {
    let mut run = StaffDayRun { slot70: 0 };
    let mut changed = false;
    let table = read_usize(G_STAFF_TABLE);
    let end = table.wrapping_add((read_i32(G_STAFF_COUNT) as usize).wrapping_mul(staff::SIZE)).wrapping_sub(staff::SIZE);
    let mut p = table;
    while p < end {
        changed |= staff_day_person(p, &mut run);
        p += staff::SIZE;
    }
    if changed {
        staff_day_hand_back_abilities();
    }
}

/// Comparer for `game_daily_staff_update`: the original only (the day touches every person and posts news, offers and
/// shortlists through dozens of routines: it cannot run twice); the copy is proven by the emulation of the whole
/// routine (levas/fila/gra_2c/check_routine.py) and by the mutations through the career.
pub(crate) unsafe extern "C" fn compare_game_daily_staff_update() {
    original_fn::<unsafe extern "C" fn()>(IDX_GAME_DAILY_STAFF_UPDATE)()
}

// ---------- batch 337: the board's verdict on a match (manager_manager.cpp) ----------

/// The argument shapes of the board's match verdicts (the format call of each case of
/// `build_board_match_reaction_text`): who comes first and whether the score is quoted.
#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) enum BoardVerdictArgs {
    /// friendly: the opponent's gender twice, the board, the score and the opponent's name
    Friendly,
    /// the opponent and the competition, the board, the opponent's name and the competition's short name
    OpponentInComp,
    /// as `OpponentInComp`, with the score after the board
    OpponentInCompWithScore,
    /// the competition first: its gender, the opponent's, the board, the score, the competition's short name and the
    /// opponent's name
    CompThenOpponentWithScore,
}

/// The verdict template of a match by its marks: the friendly template (form `Friendly`) when the case asks whether
/// the competition is a friendly, the other template and its argument shape; `None` for the marks without a case of
/// their own (the 0x800 / 0x400 / plain defaults).
fn board_verdict_template(marks: i32) -> Option<(Option<usize>, usize, BoardVerdictArgs)> {
    use BoardVerdictArgs::*;
    Some(match marks {
        0x401 | 0x501 => (Some(0xa0f5f4), 0xa0f548, OpponentInComp),
        0x402 | 0x502 => (Some(0xa0f8ac), 0xa0f7f8, OpponentInComp),
        0x408 | 0x4a0 | 0x608 | 0x6a0 => (Some(0xa0eab4), 0xa0ea10, OpponentInComp),
        0x420 | 0x620 => (Some(0xa0f204), 0xa0f164, OpponentInComp),
        0x441 | 0x541 => (Some(0xa0f750), 0xa0f694, OpponentInComp),
        0x442 | 0x542 => (Some(0xa0fa34), 0xa0f958, OpponentInCompWithScore),
        0x444 | 0x544 => (Some(0xa0fbac), 0xa0fad8, OpponentInCompWithScore),
        0x448 | 0x648 => (Some(0xa0f350), 0xa0f2a0, OpponentInComp),
        0x450 | 0x650 => (Some(0xa0ee98), 0xa0ede4, OpponentInComp),
        0x460 | 0x660 => (Some(0xa0ec04), 0xa0eb54, OpponentInComp),
        0x481 | 0x581 => (Some(0xa0f49c), 0xa0f3f0, OpponentInComp),
        0x482 | 0x582 => (Some(0xa0f5f4), 0xa0ef3c, OpponentInComp), // the friendly text of 0x401
        0x484 | 0x584 => (Some(0xa0f0bc), 0xa0efec, OpponentInCompWithScore),
        0x488 | 0x688 => (Some(0xa0ed4c), 0xa0ecac, OpponentInComp),
        0x808 | 0xa08 => (None, 0xa0e6cc, CompThenOpponentWithScore),
        0x810 | 0xa10 => (None, 0xa0e168, CompThenOpponentWithScore),
        0x820 | 0xa20 => (None, 0xa0e350, CompThenOpponentWithScore),
        0x848 | 0xa48 => (Some(0xa0e62c), 0xa0e55c, OpponentInCompWithScore),
        0x850 | 0xa50 => (Some(0xa0e2c8), 0xa0e210, OpponentInCompWithScore),
        0x860 | 0xa60 => (Some(0xa0e4c8), 0xa0e404, OpponentInCompWithScore),
        0x888 | 0xa88 => (Some(0xa0e0d0), 0xa0e008, OpponentInCompWithScore),
        0x890 | 0xa90 => (Some(0xa0de38), 0xa0dd88, OpponentInCompWithScore),
        0x8a0 | 0xaa0 => (Some(0xa0df7c), 0xa0dec0, OpponentInCompWithScore),
        0x948 => (None, 0xa0e86c, OpponentInCompWithScore),
        0x988 => (None, 0xa0e790, OpponentInCompWithScore),
        0x1001 | 0x1101 => (Some(0xa0d030), 0xa0cf74, OpponentInCompWithScore),
        0x1002 | 0x1102 => (None, 0xa0d3cc, CompThenOpponentWithScore),
        0x1008 => (Some(0xa0dcd4), 0xa0dc08, OpponentInCompWithScore),
        0x1020 | 0x1120 => (Some(0xa0d85c), 0xa0d794, OpponentInCompWithScore),
        0x1041 | 0x1141 => (Some(0xa0d314), 0xa0d244, OpponentInCompWithScore),
        0x1042 | 0x1142 => (Some(0xa0d560), 0xa0d490, OpponentInCompWithScore),
        0x1044 | 0x1144 => (Some(0xa0d6f0), 0xa0d624, OpponentInCompWithScore),
        0x1048 | 0x1148 => (Some(0xa0db68), 0xa0da90, OpponentInCompWithScore),
        0x1060 | 0x1160 => (Some(0xa0d9dc), 0xa0d908, OpponentInCompWithScore),
        0x1081 | 0x1181 => (Some(0xa0d194), 0xa0d0d0, OpponentInCompWithScore),
        0x1082 | 0x1182 => (Some(0xa0ceb4), 0xa0cdec, OpponentInCompWithScore),
        0x1084 | 0x1184 => (Some(0xa0c9b8), 0xa0c8f4, OpponentInCompWithScore),
        0x1088 | 0x1188 => (Some(0xa0cd48), 0xa0cc70, OpponentInCompWithScore),
        0x10a0 | 0x11a0 => (Some(0xa0cb30), 0xa0ca60, OpponentInCompWithScore),
        0x1208 => (None, 0xa0cbd0, OpponentInComp),
        0x1220 => (None, 0xa0c854, OpponentInComp),
        0x1248 => (None, 0xa0c780, OpponentInComp),
        0x1260 => (None, 0xa0c610, OpponentInComp),
        0x1288 => (None, 0xa0c6b8, OpponentInComp),
        0x12a0 => (None, 0xa0c560, OpponentInComp),
        _ => return None,
    })
}

/// Formats one board verdict (`story_lang!`) into `out` with the argument shape of its case.
unsafe fn format_board_verdict(out: usize, template: usize, shape: BoardVerdictArgs, other: Club, comp: Competition, board: usize, score: usize) {
    let go = other.gender_name() as i32;
    let gc = comp.short_gender_name() as i8 as i32;
    let (name, short) = (other.name_addr(), comp.short_name_addr());
    match shape {
        BoardVerdictArgs::Friendly => story_lang!(out, template, go, go, board, score, name),
        BoardVerdictArgs::OpponentInComp => story_lang!(out, template, go, go, gc, gc, board, name, short),
        BoardVerdictArgs::OpponentInCompWithScore => story_lang!(out, template, go, go, gc, gc, board, score, name, short),
        BoardVerdictArgs::CompThenOpponentWithScore => story_lang!(out, template, gc, gc, go, go, board, score, short, name),
    }
}

/// The board's sentence on the league run after the verdict (0 when there is none): `run` 2 (a winning run) or −2 (a
/// losing run), the board's target, the club's current and best positions.
unsafe fn board_run_sentence(this: usize, club: Club, run: i8, target: i8, current: i8, best: i8) -> usize {
    let (target, current) = (target as i32, current as i32);
    let flags = read_u16(board_row(this, club) + board_record::EXPECTATION_FLAGS);
    // the first expectation flag (in this order) picks the sentence of the run of victories / of defeats
    let by_expectation = |sentences: [usize; 7]| {
        let bits = [1u16, 2, 4, 8, 0x40, 0x20];
        bits.iter().position(|&b| flags & b != 0).map_or(sentences[6], |k| sentences[k])
    };
    if run == 2 {
        if target < current {
            return 0;
        }
        if current > target + 4 {
            return 0xa0c514; // "Hopefully this run of results indicates a change in the club's fortunes."
        }
        if current > target - 4 {
            return 0xa0c4c4; // "We are very impressed with the way that the team is performing at the moment."
        }
        if current <= target {
            return 0;
        }
        return by_expectation([0xa0c474, 0xa0c42c, 0xa0c3e8, 0xa0c3a0, 0xa0c354, 0xa0c310, 0xa0c2c4]); // "This run of victories is keeping us on track to ..."
    }
    if run != -2 || target < best as i32 {
        return 0;
    }
    if current > target + 4 {
        return 0xa0c270; // "If this losing run isn't turned around soon we may have to look for a new manager."
    }
    if current < target - 4 && !club.division().is_null() {
        let league = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((read_i32(club.division().addr()) as usize).wrapping_mul(4)));
        if league != 0 {
            let clubs = read_i8(league + league::ENTRANT_COUNT) as i32; // a byte of the entrant count
            let relegation_zone = flags as i16 & expectation_flags::RELEGATION_BATTLE != 0
                && call_comp_virtual(league, league_vtable::HAS_RELEGATION) != 0
                && current > clubs - read_i8(league + league::RELEGATED_COUNT) as i32;
            return if relegation_zone {
                0xa0c22c // "We are in danger of getting relegated unless results improve soon."
            } else {
                0xa0c1d8 // "We are in danger of losing our good league position unless results improve soon."
            };
        }
    }
    if current <= target + 2 {
        return 0;
    }
    by_expectation([0xa0c174, 0xa0c118, 0xa0c0c0, 0xa0c068, 0xa0c008, 0xa0bfc0, 0xa0bf5c]) // "We are worried that this run of defeats ..."
}

/// Writes the board's (or the FA's) verdict on a match into the game's text buffer (`NEWS_TEXT_BUFFER`), by the
/// match marks, with a sentence on the league run or on the players it named; returns 1, or 0 when only the plain
/// default verdict was written.
///
/// `build_board_match_reaction_text(club, other, comp, marks, score, _, _, ids, variant, loss, run, target, current,
/// best, in_league)`, called after every match by `update_board_confidence_after_match` (the text is not used there:
/// only the return value) and by the news text `news_board_match_reaction_text`. The board: "The <club> board of
/// directors" (0xa0fc54, the club's `.gender_name()` and name), or for a national team (`exe::database_is_national_team`)
/// "The <nationality> Football Association" (0xa0fc8c, the adjective of the club's `.nation()` by
/// `exe::nation_get_nationality_adjective`, with 100 as the two genders). The verdict by `marks`
/// (`board_verdict_template`): the cases that ask whether the competition is a friendly
/// (`exe::friendly_is_friendly_competition` of `G_FRIENDLY`) take the friendly template; the others, their template
/// with the opponent's gender and name, the competition's `.short_gender_name()` and short name and the score.
/// Marks without a case write straight into the buffer with the score and return 0: 0xa0e94c with 0x800, 0xa0be98
/// with 0x400, else 0xa0bdd0.
///
/// The run sentence (`board_run_sentence`), only with `in_league`: a winning run (2) with the club at or above its
/// target by at most 3 places gives "We are very impressed..."; a losing run (−2) with
/// the target not above the best position: more than 4 places below the target "If this losing run isn't turned
/// around...", more than 4 above it in a division (`league::ENTRANT_COUNT` low byte, `league::RELEGATED_COUNT`,
/// `league_vtable::HAS_RELEGATION`) "danger of getting relegated" when the board expects a relegation battle
/// (`expectation_flags::RELEGATION_BATTLE` of `board_record::EXPECTATION_FLAGS`) and the club is in the relegation
/// places, else "danger of losing our good league position"; more than 2 places below the target, the "worried"
/// sentence of the first expectation flag (European places, title, promotion, play-offs, 0x40, relegation battle,
/// else the comfortable position). Without one and with marks 0x1800, the named players' sentence
/// (`build_board_player_performance_text`, with "won" = marks bit 0x800 in the low byte of `loss`). The buffer gets
/// "%s %s" (verdict and sentence) or "%s" (0x991948 / 0x991944, `story_crt!`).
///
/// Defects of the original: without a competition it writes 0xa0bd50 with the board string before building it
/// (uninitialised stack; the callers always pass one); the winning-run tests make "Hopefully this run..." and the
/// seven "This run of victories is keeping us on track..." sentences unreachable (they need the club both at or above
/// and below its target); the two aggregate arguments are never read.
/// Original: `MANAGER_MANAGER::build_board_match_reaction_text` (0x6a69c0, manager_manager.cpp, thiscall, ret 0x3c).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn build_board_match_reaction_text(this: usize, club: usize, other: usize, comp: usize, marks: i32, score: usize, _aggregate_for: i32,
                                                                       _aggregate_against: i32, ids: usize, variant: u32, loss: u32, run: u32, target: u32,
                                                                       current: u32, best: u32, in_league: u32) -> u32 {
    let (club, other, comp) = (Club(club), Club(other), Competition(comp));
    let mut board = [0u8; 200];
    let b = board.as_mut_ptr() as usize;
    if comp.is_null() {
        // simplification: the original quotes the board string before writing it (stack garbage); the copy an empty one
        story_lang!(NEWS_TEXT_BUFFER, 0xa0bd50, other.gender_name() as i32, other.gender_name() as i32, b, other.name_addr());
        return 0;
    }
    if exe::database_is_national_team(club) != 0 {
        let mut adjective = [0u8; 100];
        exe::nation_get_nationality_adjective(club.nation(), adjective.as_mut_ptr() as usize, 100, 9);
        story_lang!(b, 0xa0fc8c, 100, 100, adjective.as_ptr()); // "The <nationality> Football Association"
    } else {
        let g = club.gender_name() as i32;
        story_lang!(b, 0xa0fc54, g, g, club.name_addr()); // "The <club> board of directors"
    }
    let mut verdict = [0u8; 1500];
    let v = verdict.as_mut_ptr() as usize;
    match board_verdict_template(marks) {
        Some((friendly, template, shape)) => {
            covered!("sx14_1 verdict");
            match friendly {
                Some(t) if exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), comp) != 0 => {
                    covered!("sx14_1 friendly verdict");
                    format_board_verdict(v, t, BoardVerdictArgs::Friendly, other, comp, b, score)
                }
                _ => format_board_verdict(v, template, shape, other, comp, b, score),
            }
        }
        None => {
            covered!("sx14_1 default verdict");
            let template = if marks & 0x800 != 0 { 0xa0e94c } else if marks & 0x400 != 0 { 0xa0be98 } else { 0xa0bdd0 };
            format_board_verdict(NEWS_TEXT_BUFFER, template, BoardVerdictArgs::OpponentInCompWithScore, other, comp, b, score);
            return 0;
        }
    }
    let mut sentence = [0u8; 1500];
    let s = sentence.as_mut_ptr() as usize;
    if in_league as u8 != 0 {
        let t = board_run_sentence(this, club, run as i8, target as i8, current as i8, best as i8);
        if t != 0 {
            covered!("sx14_1 run sentence");
            story_lang!(s, t);
        }
    }
    if sentence[0] == 0 && marks & 0x1800 != 0 {
        covered!("sx14_1 named players");
        let won = (loss & 0xffff_ff00) | ((marks as u32 >> 11) & 1);
        build_board_player_performance_text(ids as *const i32, won, s, variant);
    }
    if sentence[0] != 0 {
        story_crt!(NEWS_TEXT_BUFFER, 0x991948, v, s); // "%s %s"
    } else {
        story_crt!(NEWS_TEXT_BUFFER, 0x991944, v); // "%s"
    }
    1
}

/// Comparer for `build_board_match_reaction_text`: the original and the copy side by side on the game's text buffer
/// (`NEWS_TEXT_BUFFER`, 2000 bytes) and the return value. Without a competition only the original runs (it quotes an
/// uninitialised string).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_build_board_match_reaction_text(this: usize, club: usize, other: usize, comp: usize, marks: i32, score: usize, aggregate_for: i32,
                                                                               aggregate_against: i32, ids: usize, variant: u32, loss: u32, run: u32, target: u32,
                                                                               current: u32, best: u32, in_league: u32) -> u32 {
    type F = unsafe extern "thiscall" fn(usize, usize, usize, usize, i32, usize, i32, i32, usize, u32, u32, u32, u32, u32, u32, u32) -> u32;
    let o = original_fn::<F>(IDX_BUILD_BOARD_MATCH_REACTION_TEXT);
    let original = || o(this, club, other, comp, marks, score, aggregate_for, aggregate_against, ids, variant, loss, run, target, current, best, in_league);
    if comp == 0 {
        return original();
    }
    let (ro, rr, same, mo, mr) = crate::comparison::side_by_side(&[(NEWS_TEXT_BUFFER, 2000)], original, || {
        build_board_match_reaction_text(this, club, other, comp, marks, score, aggregate_for, aggregate_against, ids, variant, loss, run, target, current, best, in_league)
    });
    let text = |m: &crate::comparison::MemorySnapshot| {
        let b = &m.0[0].1;
        String::from_utf8_lossy(&b[..b.iter().position(|&c| c == 0).unwrap_or(b.len())]).into_owned()
    };
    SCOREBOARD.record(same && ro == rr, || {
        format!("build_board_match_reaction_text(marks {:#x}, run {} target {} current {} best {} league {}) original {} {:?} rust {} {:?}",
                marks, run as i8, target as i8, current as i8, best as i8, in_league as u8, ro, text(&mo), rr, text(&mr))
    });
    ro
}

// ---------- batch 337: the news text of the board's reaction to a match (manager_manager.cpp) ----------

/// The headline of the board's match reaction (news 0x138c): the derby headlines (marks 0x40) have no club; the
/// others quote the opponent, except the fury one, which quotes the club itself.
unsafe fn board_reaction_headline(marks: u32, club: Club, other: Club) {
    if marks & 0x40 != 0 {
        let template = if marks & 8 != 0 {
            0xa0bd0c // "Board delighted at derby showing"
        } else if marks & 0x20 != 0 {
            0xa0bcec // "Board happy at derby display"
        } else if marks & 1 != 0 {
            0xa0bccc // "Board furious at derby display"
        } else if marks & 2 != 0 {
            0xa0bca8 // "Board unhappy with derby display"
        } else {
            0xa0bc88 // "Board reaction to derby match"
        };
        story_lang!(NEWS_TEXT_BUFFER, template);
        return;
    }
    let (template, quoted) = if marks & 8 != 0 {
        (0xa0bc48, other) // "Board delighted after <club> game"
    } else if marks & 0x20 != 0 {
        (0xa0bc0c, other) // "<club> match pleases board"
    } else if marks & 1 != 0 {
        (0xa0bbcc, club) // "Board fury at <club> performance"
    } else if marks & 2 != 0 && marks & 0x1000 != 0 {
        (0xa0bb8c, other) // "Board unhappy after <club> defeat"
    } else {
        (0xa0bb50, other) // "Board reaction to <club> game"
    };
    let g = quoted.short_gender_name() as i8 as i32;
    story_lang!(NEWS_TEXT_BUFFER, template, g, g, quoted.addr() + club::SHORT_NAME);
}

/// Writes the headline or the body of the board's reaction to a match (news 0x138c, posted by
/// `post_board_match_reaction_news`).
///
/// `news_board_match_reaction_text(news, body, out)`. The headline (`board_reaction_headline`) from slots 0 (the
/// marks), 1 (the club) and 2 (the opponent), with the links to the two clubs' short names (0x7d5). The body reads
/// the 18 slots in order (the marks, the clubs, the score, the aggregate, the fixture's stats slot, the three named
/// players, the variant, the defeat, the run, the target, the current and best positions and the league flag) and
/// the match record behind them (`exe::read_match_record_from_news`, a `RecordMatch`); the score "<higher>-<lower>"
/// (0xa0bd30) and the verdict (`FN_MANAGER_MANAGER_BUILD_BOARD_MATCH_REACTION_TEXT`, with the record's competition)
/// into the game's text buffer, which goes into `out` (`exe::text_set_string`); the links: the two clubs' names
/// (0x7d5), the competition's short name (0x7d0), each named player still set (0x7db, the name by
/// `exe::database_format_staff_name`, mode 1) and the score (0x7d3) when the fixture is still in the match stats
/// (`RecordMatch::link_score`).
/// Original: `MANAGER_MANAGER::news_board_match_reaction_text` (0x6a6310, manager_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn news_board_match_reaction_text(this: usize, news: usize, body: u32, out: usize) {
    if body == 0 {
        covered!("sx14_2 headline");
        let (marks, club, other) = (exe::newsitem_get_slot(news, 0), exe::newsitem_get_slot(news, 1), exe::newsitem_get_slot(news, 2));
        let (club, other) = (club_at(club as i32 as isize), club_at(other as i32 as isize));
        board_reaction_headline(marks, club, other);
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        exe::text_add_link(out, (club.addr() + club::SHORT_NAME) as *const u8, 0x7d5, club);
        exe::text_add_link(out, (other.addr() + club::SHORT_NAME) as *const u8, 0x7d5, other);
        return;
    }
    covered!("sx14_2 body");
    let mut k: u8 = 0;
    let mut slots = [0u32; 18];
    for v in slots.iter_mut() {
        *v = exe::newsitem_get_slot(news, k as i8 as u32);
        k += 1;
    }
    let mut record = [0u8; 0x20];
    let rec = record.as_mut_ptr() as usize;
    exe::date_date_54a110(rec);
    exe::read_match_record_from_news(news, &mut k, record.as_mut_ptr());
    let record = RecordMatch(rec);
    let [marks, club, other, goals_a, goals_b, aggregate_for, aggregate_against, _stats_slot, id1, id2, id3, variant, loss, run, target, current, best, in_league] = slots;
    let (club, other) = (club_at(club as i32 as isize), club_at(other as i32 as isize));
    let (goals_a, goals_b) = (goals_a as i32, goals_b as i32);
    let mut score = [0u8; 100];
    story_lang!(score.as_mut_ptr(), 0xa0bd30, goals_a.max(goals_b), goals_a.min(goals_b)); // "<goals>-<opp goals>"
    let ids = [id1 as i32, id2 as i32, id3 as i32];
    let verdict: unsafe extern "thiscall" fn(usize, usize, usize, usize, i32, usize, i32, i32, usize, u32, u32, u32, u32, u32, u32, u32) -> u32 =
        core::mem::transmute(FN_MANAGER_MANAGER_BUILD_BOARD_MATCH_REACTION_TEXT);
    verdict(this, club.addr(), other.addr(), record.comp(), marks as i32, score.as_ptr() as usize, aggregate_for as i32, aggregate_against as i32,
            ids.as_ptr() as usize, variant, loss, run, target, current, best, in_league);
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    exe::text_add_link(out, club.name_addr() as *const u8, 0x7d5, club);
    exe::text_add_link(out, other.name_addr() as *const u8, 0x7d5, other);
    let comp = record.comp();
    exe::text_add_link(out, (comp + competition::SHORT_NAME) as *const u8, 0x7d0, comp);
    let mut name = [0u8; 200];
    for id in ids {
        if id != -1 {
            let p = staff_at(id as isize);
            exe::database_format_staff_name(p, name.as_mut_ptr() as usize, 200, 1);
            exe::text_add_link(out, name.as_ptr(), 0x7db, p);
        }
    }
    record.link_score(out, &score);
}

/// Comparer for `news_board_match_reaction_text`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_news_board_match_reaction_text(this: usize, news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(IDX_NEWS_BOARD_MATCH_REACTION_TEXT);
    compare_text_writer(out, || o(this, news, body, out), |t| news_board_match_reaction_text(this, news, body, t),
                        || format!("news_board_match_reaction_text(news {:#x}, body {})", news, body));
}

// ---------- batch 337: the board's daily pass and the manager appointments (manager_manager.cpp) ----------

/// `MANAGER_MANAGER::post_job_offer_to_human_news` (0x6a4f80; thiscall(this, club, person), ret 8): the job offer to a
/// human manager.
const FN_POST_JOB_OFFER_TO_HUMAN_NEWS: usize = 0x6a4f80;

/// The vacancy item (`manager_manager_fields::VACANCY_LIST`, 0x26 bytes): the rumour flags (+0 posted, +1 done), the
/// linked candidate (+2), the chosen person (+6), the club (`job_vacancy::CLUB_ID`), the arrival date (+0x16 day,
/// +0x18 year) and the rumour date (+0x1e day, +0x20 year).
#[derive(Clone, Copy)]
struct VacancyItem(usize);

impl VacancyItem {
    unsafe fn person(self) -> Staff {
        staff_at(read_i32(self.0 + 6) as isize)
    }
    unsafe fn club(self) -> Club {
        club_at(read_i32(self.0 + job_vacancy::CLUB_ID) as isize)
    }
    /// True once today is past the date at `day` (+0 day, +2 year).
    unsafe fn date_passed(self, day: usize) -> bool {
        let (today, year) = (read_i16(G_CURRENT_DATE), read_u16(G_CURRENT_DATE + 2));
        let year_due = read_u16(self.0 + day + 2);
        year > year_due || (year == year_due && today > read_i16(self.0 + day))
    }
}

/// Sets the club's `board_record::JOB_APPLICATION_DATE` to "never" (23/07/1971).
unsafe fn clear_job_application_date(this: usize, club: Club) {
    let mut never = [0u16; 4];
    let d = exe::date_date_549f70(&mut never, 0x17, 7, 0x7b3, u32::MAX);
    manager_manager_copy_date(board_row(this, club) + board_record::JOB_APPLICATION_DATE, d);
}

/// True for the 16 human managers (the last ids of the staff table).
unsafe fn is_human_id(person: Staff) -> bool {
    read_i32(person.addr()) >= read_i32(G_STAFF_COUNT) - 0x10
}

/// Frees the person's staff job at `from` when he has a manager record (`remove_staff_from_club`).
unsafe fn remove_staff(this: usize, person: Staff, from: Club) {
    if !person.non_player().is_null() {
        remove_staff_from_club(this, person.addr(), from.addr());
    }
}

/// Takes the person out of the squad of `from` when he has a player record (`manager_manager_remove_player_from_squad`).
unsafe fn remove_player(this: usize, person: Staff, from: Club) {
    if !person.player().is_null() {
        manager_manager_remove_player_from_squad(this, person.addr(), from.addr());
    }
}

/// The 16 human managers who joined their club more than 180 days ago lose the club board's
/// `board_record_flags::NEW_MANAGER` bit (bit 0 of `board_record::FLAGS`).
unsafe fn settle_new_human_managers(this: usize) {
    let mut since = [0u16; 4];
    exe::date_sub_days(G_CURRENT_DATE, since.as_mut_ptr() as usize, 0xb4);
    for k in 0..0x10 {
        let person = staff_at((read_i32(G_STAFF_COUNT) - 0x10 + k) as isize);
        let club = person.club_job();
        if club.is_null() || person.classification() == 0 {
            continue;
        }
        let year = person.date_joined_club_year();
        if since[1] > year || (since[1] == year && (since[0] as i16) > read_i16(person.addr() + staff::DATE_JOINED_CLUB)) {
            let flags = board_row(this, club) + board_record::FLAGS;
            write_u8(flags, read_u8(flags) & 0xfe);
        }
    }
}

/// A human manager's vacancy whose arrival date came: without a classification the vacancy reopens; without a club or
/// out of work for 90 days the club offers him the job (`FN_POST_JOB_OFFER_TO_HUMAN_NEWS`) and the vacancy reopens;
/// otherwise the vacancy reopens too (the order of the list calls is the original's).
unsafe fn human_vacancy_reached(this: usize, list: usize, item: VacancyItem, person: Staff, club: Club) {
    if person.classification() == 0 {
        clear_job_application_date(this, club);
        exe::llist_remove(list, item.0);
        add_job_vacancy(this, club.addr(), person.addr());
    } else if person.club_job().is_null()
        || exe::date_days_since(G_CURRENT_DATE, person.date_joined_club() as u32, person.date_joined_club_leap_year() as u32) >= 0x5a
    {
        covered!("sx14_3 offer to the human");
        let offer: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_POST_JOB_OFFER_TO_HUMAN_NEWS);
        offer(this, club.addr(), person.addr());
        exe::llist_remove(list, item.0);
        add_job_vacancy(this, club.addr(), person.addr());
    } else {
        clear_job_application_date(this, club);
        add_job_vacancy(this, club.addr(), person.addr());
        exe::llist_remove(list, item.0);
    }
}

/// The club the appointed person leaves: his national team (for a national team, unless his job there is 11), else
/// his club; an AI manager going from a club to a national team with reputation above 6000 that is not a B team
/// (`exe::club_is_national_team`) leaves his club too. None otherwise.
unsafe fn previous_club(person: Staff, club: Club) -> Option<Club> {
    let national = exe::database_is_national_team(club) != 0;
    if national && !person.national_job().is_null() && person.job_for_nation() != 0xb {
        return Some(person.national_job());
    }
    if !national && !person.club_job().is_null() {
        return Some(person.club_job());
    }
    if !is_human_id(person) && national && exe::club_is_national_team(club) == 0 && club.reputation() > 0x1770 && !person.club_job().is_null() {
        return Some(person.club_job());
    }
    None
}

/// Moving to a club from his own club: frees his jobs there by his club job (`staff::JOB_FOR_CLUB`) and returns the
/// appointment kind (1 a player, 3 a manager or player-manager, 2 otherwise).
unsafe fn leave_club_for_club(this: usize, person: Staff, from: Club) -> u32 {
    match person.job_for_club() as i8 {
        5 => {
            remove_staff(this, person, from);
            3
        }
        6 | 8 => {
            remove_staff(this, person, from);
            2
        }
        0xb => {
            remove_player(this, person, from);
            1
        }
        0xc => {
            remove_player(this, person, from);
            remove_staff(this, person, from);
            3
        }
        0xf => {
            remove_player(this, person, from);
            remove_staff(this, person, from);
            2
        }
        _ => {
            if person.non_player().is_null() {
                person.set_job_for_club(0xb);
            } else {
                remove_staff(this, person, from);
            }
            if person.player().is_null() {
                person.set_job_for_club(if person.non_player().is_null() { 0 } else { 8 });
            } else {
                remove_player(this, person, from);
            }
            2
        }
    }
}

/// An AI person moving to a club from his national team: frees his jobs by his national job
/// (`staff::JOB_FOR_NATION`), although the default case writes his club job, and returns the appointment kind.
unsafe fn leave_national_team_for_club(this: usize, person: Staff, from: Club) -> u32 {
    match person.job_for_nation() as i8 {
        5 | 0xc => {
            remove_staff(this, person, from);
            3
        }
        6 | 8 | 0xf => {
            remove_staff(this, person, from);
            2
        }
        0xb => 1,
        _ => {
            if person.non_player().is_null() {
                person.set_job_for_club(0xb);
            } else {
                remove_staff(this, person, from);
            }
            if !person.player().is_null() {
                remove_player(this, person, from);
            } else {
                person.set_job_for_club(if person.non_player().is_null() { 0 } else { 8 });
            }
            2
        }
    }
}

/// Moving to a national team: frees his jobs at the club or national team he leaves by his national job and returns
/// the appointment kind; a job 15 without a player record shows the original's error box (line 0x6bb).
unsafe fn leave_for_national_team(this: usize, person: Staff, from: Club) -> u32 {
    match person.job_for_nation() as i8 {
        5 => {
            remove_staff(this, person, from);
            3
        }
        6 | 8 => {
            remove_staff(this, person, from);
            2
        }
        0xb => {
            remove_player(this, person, from);
            1
        }
        0xc => {
            remove_player(this, person, from);
            remove_staff(this, person, from);
            2
        }
        0xf => {
            if person.player().is_null() {
                show_error_at(0x9f18a4, 0x6bb);
            } else {
                remove_player(this, person, from);
            }
            remove_staff(this, person, from);
            2
        }
        _ => {
            if person.non_player().is_null() {
                person.set_job_for_nation(0xb);
            } else {
                remove_staff(this, person, from);
            }
            if !person.player().is_null() {
                remove_player(this, person, from);
            } else {
                person.set_job_for_club(if person.non_player().is_null() { 0 } else { 8 });
            }
            2
        }
    }
}

/// Posts the appointment and frees the person from the club he leaves (`leave_*`), ending his contract
/// (`exe::contract_manager_remove_contract`).
unsafe fn release_appointed_person(this: usize, person: Staff, previous: Option<Club>, club: Club) {
    let contracts = read_usize(G_CONTRACT_MANAGER);
    let appoint = |from: usize, kind: u32| post_manager_appointment_news(this, person.addr(), from, club.addr(), kind);
    if exe::database_is_national_team(club) == 0 {
        match previous {
            Some(from) if from == person.club_job() => {
                let kind = leave_club_for_club(this, person, from);
                appoint(from.addr(), kind);
                exe::contract_manager_remove_contract(contracts, person);
            }
            Some(from) if !is_human_id(person) && from == person.national_job() => {
                let kind = leave_national_team_for_club(this, person, from);
                appoint(from.addr(), kind);
                exe::contract_manager_remove_contract(contracts, person);
            }
            _ => appoint(0, 2),
        }
    } else {
        match previous {
            Some(from) if from == person.national_job() || from == person.club_job() => {
                covered!("sx14_3 to a national team");
                let kind = leave_for_national_team(this, person, from);
                appoint(from.addr(), kind);
                if exe::database_is_national_team(from) == 0 {
                    exe::contract_manager_remove_contract(contracts, person);
                }
            }
            _ => appoint(0, 2),
        }
    }
}

/// A player moving between two clubs goes into the past transfers (`exe::transfer_manager_add_past_transfer`): the
/// person, his three name ids (−1 without a name), the new and the old club, no fee, type 5 and today.
unsafe fn record_manager_move(person: Staff, previous: Club, club: Club) {
    let mut rec = [0u8; 0x25];
    let r = rec.as_mut_ptr() as usize;
    exe::date_date_54a110(r + 0x1d);
    write_i32(r, read_i32(person.addr()));
    for (k, name) in [person.first_name(), person.second_name(), person.common_name()].into_iter().enumerate() {
        write_i32(r + 4 + 4 * k, if name == 0 { -1 } else { read_i32(name + names::ID) });
    }
    write_i32(r + 0x10, read_i32(club.addr()));
    write_i32(r + 0x14, read_i32(previous.addr()));
    write_i32(r + 0x18, 0); // the fee
    write_u8(r + 0x1c, 5); // a non-player job
    manager_manager_copy_date(r + 0x1d, G_CURRENT_DATE);
    exe::transfer_manager_add_past_transfer(read_usize(G_TRANSFER_MANAGER), rec.as_mut_ptr());
}

/// The person takes the club: the joining date, the club's manager, and at a club the job (a player-manager, job 0xc,
/// with a free squad place, his reputation + 750 above the club's, fewer than 45 in the squad and outside Italy; else
/// manager, job 5) and the board's wealth band; at a national team the national job (5).
unsafe fn install_manager(this: usize, person: Staff, club: Club) {
    if exe::database_is_national_team(club) != 0 {
        manager_manager_copy_date(person.date_joined_nation_addr(), G_CURRENT_DATE);
    } else {
        manager_manager_copy_date(person.addr() + staff::DATE_JOINED_CLUB, G_CURRENT_DATE);
    }
    club.set_manager(person);
    let row = board_row(this, club);
    if exe::database_is_national_team(club) != 0 {
        person.set_national_job(club);
        write_u8(row + board_record::WEALTH_BAND, 0);
        person.set_job_for_nation(5);
        return;
    }
    person.set_club_job(club);
    let player = person.player();
    let nation = club.nation();
    let free_place = if !player.is_null()
        && (club.reputation() as i32) < player.current_reputation() as i32 + 0x2ee
        && (exe::database_count_club_squad(club, 0, 0) as i8) < 0x2d
        && !nation.is_null()
        && read_i32(nation.addr()) != read_i32(KEY_NATION_ITALY)
    {
        (0..0x32).find(|&k| club.squad(k) == 0)
    } else {
        None
    };
    match free_place {
        Some(k) => {
            covered!("sx14_3 player-manager");
            club.set_squad(k, person.addr());
            person.set_job_for_club(0xc);
            if exe::database_is_national_team(club) == 0 {
                let contracts = read_usize(G_CONTRACT_MANAGER);
                exe::contract_manager_remove_contract(contracts, person);
                exe::contract_manager_add_staff_contract(contracts, person, person.value() as u32);
                let squads = read_usize(G_SQUAD_MANAGER);
                exe::squad_manager_set_squad_number(squads, person, club, 0);
                if exe::squad_manager_club_uses_squad_numbers(squads, club, 0) != 0 {
                    exe::squad_manager_allocate_squad_number(squads, club, 0usize);
                }
            }
        }
        None => person.set_job_for_club(5),
    }
    let finance = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
    write_u8(row + board_record::WEALTH_BAND, if finance != 0 { exe::finance_get_wealth_band(finance) } else { 0 });
}

/// An AI person's vacancy whose arrival date came: without a manager record the vacancy reopens with him; on loan or
/// retiring (staff state 0x30) without both records it reopens without him; a club that already has a manager, or
/// a person under offer, just drops the item. Otherwise the appointment: the club he leaves
/// (`previous_club`; if he managed it and it can be managed, its season expectations and a vacancy there), the job
/// application date cleared, the release and the appointment news (`release_appointed_person`), the past transfer of a
/// player between two clubs (`record_manager_move`), the installation (`install_manager`), the item off the list and
/// the board reset (`reset_board_for_new_manager`).
unsafe fn appoint_from_vacancy(this: usize, list: usize, item: VacancyItem, person: Staff, club: Club) {
    if is_human_id(person) {
        human_vacancy_reached(this, list, item, person, club);
        return;
    }
    let contracts = read_usize(G_CONTRACT_MANAGER);
    let reopen = |left: usize| {
        clear_job_application_date(this, club);
        exe::llist_remove(list, item.0);
        add_job_vacancy(this, club.addr(), left);
    };
    if person.non_player().is_null() {
        clear_job_application_date(this, club);
        exe::llist_remove(list, item.0);
        add_job_vacancy(this, club.addr(), person.addr());
        return;
    }
    if exe::contract_manager_get_loan_contract(contracts, person) != 0
        || (staff_state_byte_b(read_i32(person.addr())) & 0x30 != 0 && (person.non_player().is_null() || person.player().is_null()))
    {
        reopen(0);
        return;
    }
    if exe::database_get_club_manager(club) != 0 || exe::transfer_manager_is_player_under_offer(read_usize(G_TRANSFER_MANAGER), person, 0) != 0 {
        clear_job_application_date(this, club);
        exe::llist_remove(list, item.0);
        return;
    }
    covered!("sx14_3 appointment");
    let previous = previous_club(person, club);
    if let Some(from) = previous {
        if from.manager() == person && exe::club_can_be_managed(from) != 0 {
            set_season_expectations(this, from.addr(), 1);
            add_job_vacancy(this, from.addr(), person.addr());
        }
    }
    clear_job_application_date(this, club);
    if person.non_player().is_null() {
        show_error_at(0x9f18a4, 0x5e2);
    }
    release_appointed_person(this, person, previous, club);
    if let Some(from) = previous {
        if !person.player().is_null() && exe::database_is_national_team(from) == 0 && !club.is_null() && exe::database_is_national_team(club) == 0 {
            record_manager_move(person, from, club);
        }
    }
    install_manager(this, person, club);
    // simplification: the original then syncs a human's scout lists and training schedules (0x7f7270, 0x8b2470), a
    // test that never holds here (the humans took `human_vacancy_reached`)
    exe::llist_remove(list, item.0);
    reset_board_for_new_manager(this, club.addr());
}

/// A vacancy still before its arrival date: once its rumour date passed, the rumour linking the candidate with the
/// club is posted once (`post_job_rumour_news`), while the club has no manager.
unsafe fn vacancy_rumour(this: usize, item: VacancyItem) {
    let e = item.0;
    if read_u8(e) == 0 || !item.date_passed(0x1e) || read_u8(e + 1) != 0 || read_i32(e + 2) == -1 {
        return;
    }
    write_u8(e + 1, 1);
    let club = item.club();
    if club.manager().is_null() {
        covered!("sx14_3 rumour");
        post_job_rumour_news(this, staff_at(read_i32(e + 2) as isize).addr(), club.addr());
    }
}

/// The board's daily pass over the clubs and the manager market.
///
/// `MANAGER_MANAGER::daily_update()`, every day (the game's day loop). When the month changes (`exe::date_get_month`
/// against the last month seen at 0xb67a40; the first day after loading only records it, flag 0x9f18a0), each club that
/// is not a national team, or a national team whose nation has +0x71 set, gets the board's confidence news
/// (`post_board_confidence_update_news`), then `fix_missing_jobs`. The human managers settle in after 180 days
/// (`settle_new_human_managers`); on Sundays (day of the year divisible by 7) `weekly_board_confidence_update`; the
/// pending departures (`process_pending_departures`). Then every vacancy of `manager_manager_fields::VACANCY_LIST`
/// from the head: past its arrival date, the appointment (`appoint_from_vacancy`); before it, the rumour
/// (`vacancy_rumour`). Finally the ban appeals (`process_ban_appeals`).
///
/// Defects of the original: a person moving to a club from his national team is released by his national job, but
/// the default case writes his club job (0xb, 0 or 8); the "manager record" check after the appointment can only show
/// its error box (line 0x5e2) if a call in between cleared the record.
/// Original: `MANAGER_MANAGER::daily_update` (0x68f9c0, manager_manager.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn daily_update(this: usize) {
    const FIRST_DAY_AFTER_LOAD: usize = 0x9f18a0;
    const LAST_MONTH: usize = 0xb67a40;
    let month = exe::date_get_month(G_CURRENT_DATE);
    if read_u8(FIRST_DAY_AFTER_LOAD) != 0 {
        write_u8(LAST_MONTH, month);
        write_u8(FIRST_DAY_AFTER_LOAD, 0);
    } else if month != read_u8(LAST_MONTH) {
        covered!("sx14_3 new month");
        write_u8(LAST_MONTH, exe::date_get_month(G_CURRENT_DATE));
        let mut k = 0;
        while k < read_i32(G_CLUB_COUNT) {
            let club = club_at(k as isize);
            let nation = club.nation();
            if exe::database_is_national_team(club) == 0 || (!nation.is_null() && read_u32(nation.addr() + 0x71) != 0) {
                post_board_confidence_update_news(this, club.addr());
            }
            k += 1;
        }
        fix_missing_jobs(this);
    }
    settle_new_human_managers(this);
    if read_i16(G_CURRENT_DATE) % 7 == 0 {
        weekly_board_confidence_update(this);
    }
    process_pending_departures(this);
    let list = read_usize(this + manager_manager_fields::VACANCY_LIST);
    list_rewind(list);
    loop {
        let e = exe::llist_get_next(list);
        if e == 0 {
            break;
        }
        let item = VacancyItem(e);
        if item.date_passed(0x16) {
            appoint_from_vacancy(this, list, item, item.person(), item.club());
        } else {
            vacancy_rumour(this, item);
        }
    }
    process_ban_appeals(this);
}

/// Comparer for `daily_update`: it moves managers, posts news and draws, so only the original runs (its effects are
/// checked by the career).
pub(crate) unsafe extern "thiscall" fn compare_daily_update(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_DAILY_UPDATE)(this)
}

// ---------- sx04b_2: the constructors of a nation's awards, the month ratings and the month award (nation_awards.cpp, month_ratings.cpp, month_award.cpp) ----------

/// `NationAwards`' base vtable.
pub(crate) const NATION_AWARDS_BASE_VTABLE: u32 = 0x96da10;
/// `MONTH_AWARD`'s vtable.
pub(crate) const MONTH_AWARD_VTABLE: u32 = 0x96da00;
/// The award type byte (`award_base::TYPE`) of a month award.
pub(crate) const MONTH_AWARD_KIND: u8 = 2;
/// `MONTH_AWARD::load_from_disk(file)` (0x766f50, month_award.cpp, thiscall, ret 4).
const FN_MONTH_AWARD_LOAD_FROM_DISK: usize = 0x766f50;
/// `MONTH_RATINGS::load_from_disk(file)` (0x767b20, month_ratings.cpp, thiscall, ret 4).
const FN_MONTH_RATINGS_LOAD_FROM_DISK: usize = 0x767b20;

/// Builds the base of every nation's awards object: the base vtable, no award table (`nation_awards_fields::TABLE`),
/// no awards (`COUNT`, i16) and no month ratings (`MONTH_RATINGS`). Called first by each nation's constructor, when the game is
/// created or loaded. Returns `this`.
/// Original: `NationAwards::NationAwards` (0x768290, nation_awards.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn construct_nation_awards(this: usize) -> usize {
    covered!("sx04b_2 nation awards built");
    write_u32(this, NATION_AWARDS_BASE_VTABLE); // sx04b_2: the base vtable
    write_u32(this + nation_awards_fields::TABLE, 0);
    write_u16(this + nation_awards_fields::COUNT, 0);
    write_u32(this + nation_awards_fields::MONTH_RATINGS, 0); // sx04b_2: no month ratings yet
    this
}

/// Comparer for `construct_nation_awards`: side by side over the 0xe bytes of the base.
pub(crate) unsafe extern "thiscall" fn compare_construct_nation_awards(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_CONSTRUCT_NATION_AWARDS);
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&[(this, 0xe)], || o(this), || construct_nation_awards(this));
    SCOREBOARD.record(equal && r == n, || format!("construct_nation_awards original={:x?} rust={:x?}", mo, mr));
    r
}

/// Builds an empty month ratings table: no entries (+0 pointer, +4 count, +8 capacity), month −1
/// (`month_ratings_fields::MONTH`) and year 0 (`month_ratings_fields::YEAR`), so that the first day of play closes no
/// month; with a save file, the table is then read from it (`MONTH_RATINGS::load_from_disk`). Returns `this`.
/// Original: `MONTH_RATINGS::MONTH_RATINGS` (0x767af0, month_ratings.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn construct_month_ratings(this: usize, file: usize) -> usize {
    covered!("sx04b_2 month ratings built");
    write_u8(this + month_ratings_fields::MONTH, 0xff); // sx04b_2: no month yet
    write_u16(this + month_ratings_fields::YEAR, 0);
    write_u32(this, 0); // the entries
    write_u32(this + 4, 0); // the count
    write_u32(this + 8, 0); // the capacity
    if file != 0 {
        covered!("sx04b_2 month ratings loaded");
        let load: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_MONTH_RATINGS_LOAD_FROM_DISK);
        load(this, file);
    }
    this
}

/// Comparer for `construct_month_ratings`: side by side over the 0xf bytes of the table; with a save file (the load
/// allocates) only the original.
pub(crate) unsafe extern "thiscall" fn compare_construct_month_ratings(this: usize, file: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_CONSTRUCT_MONTH_RATINGS);
    if file != 0 {
        return o(this, file);
    }
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&[(this, 0xf)], || o(this, file), || construct_month_ratings(this, file));
    SCOREBOARD.record(equal && r == n, || format!("construct_month_ratings original={:x?} rust={:x?}", mo, mr));
    r
}

/// Builds a month award: the AWARD base (`AWARD::AWARD` with the nation index, the award index, the staff competition,
/// the reputations, the club competition, the file and the player flag), the `MONTH_AWARD_VTABLE`, the type
/// `MONTH_AWARD_KIND`, and one month slot (no winner, no club: −1) for each month from (`start_month`,
/// `start_year`) up to (`end_month`, `end_year`), at most twelve: the slot count (`MONTH_AWARD_SLOT_COUNT`) goes up
/// with each slot; month 11 is followed by month 0 of the next year. The first slot is always written; the end month
/// is compared signed, the years unsigned (16 bits). With a save file the slots are then read from it
/// (`MONTH_AWARD::load_from_disk`). Returns `this`.
/// Original: `MONTH_AWARD::MONTH_AWARD` (0x766e10, month_award.cpp, thiscall, ret 0x30).
pub(crate) unsafe extern "thiscall" fn construct_month_award(this: usize, nation: u32, index: u32, staff_comp: u32, national_reputation: u32, world_reputation: u32, club_comp: u32, start_month: u32, start_year: u32, end_month: u32, end_year: u32, file: usize, for_players: u32) -> usize {
    award_construct_base(this, [nation, index, staff_comp, national_reputation, world_reputation, club_comp, file as u32, for_players]);
    covered!("sx04b_2 month award built");
    write_u32(this, MONTH_AWARD_VTABLE);
    write_u8(this + award_base::TYPE, MONTH_AWARD_KIND);
    write_u8(this + MONTH_AWARD_SLOT_COUNT, 0);
    let (mut month, mut year) = (start_month as u8 as i8, start_year as u16);
    let end_year = end_year as u16;
    loop {
        let slot = this.wrapping_add((read_i8(this + MONTH_AWARD_SLOT_COUNT) as isize as usize).wrapping_mul(MONTH_AWARD_SLOT_SIZE));
        write_u8(slot + month_award_fields::SLOT_MONTH, month as u8);
        write_u16(slot + month_award_fields::SLOT_YEAR, year);
        write_u32(slot + month_award_fields::SLOT_WINNER_ID, u32::MAX);
        write_u32(slot + month_award_fields::SLOT_CLUB_ID, u32::MAX);
        let count = read_u8(this + MONTH_AWARD_SLOT_COUNT).wrapping_add(1);
        month = month.wrapping_add(1);
        if month > 11 { // sx04b_2: December is followed by January
            month = 0;
            year = year.wrapping_add(1);
        }
        write_u8(this + MONTH_AWARD_SLOT_COUNT, count);
        if (year == end_year && month > end_month as u8 as i8) || year > end_year || count as i8 >= 12 {
            break;
        }
    }
    if file != 0 {
        covered!("sx04b_2 month award loaded");
        let load: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_MONTH_AWARD_LOAD_FROM_DISK);
        load(this, file);
    }
    this
}

/// Comparer for `construct_month_award`: side by side over the 0xa2 bytes of the award (base, slots and count); with a
/// save file (the load allocates) only the original.
pub(crate) unsafe extern "thiscall" fn compare_construct_month_award(this: usize, nation: u32, index: u32, staff_comp: u32, national_reputation: u32, world_reputation: u32, club_comp: u32, start_month: u32, start_year: u32, end_month: u32, end_year: u32, file: usize, for_players: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize, u32) -> usize>(IDX_CONSTRUCT_MONTH_AWARD);
    let original = || o(this, nation, index, staff_comp, national_reputation, world_reputation, club_comp, start_month, start_year, end_month, end_year, file, for_players);
    if file != 0 {
        return original();
    }
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&[(this, MONTH_AWARD_SLOT_COUNT + 1)], original, || {
        construct_month_award(this, nation, index, staff_comp, national_reputation, world_reputation, club_comp, start_month, start_year, end_month, end_year, file, for_players)
    });
    SCOREBOARD.record(equal && r == n, || format!("construct_month_award(from {}/{} to {}/{}) original={:x?} rust={:x?}", start_month as u8, start_year as u16, end_month as u8, end_year as u16, mo, mr));
    r
}

// ---------- sx04b_1: the month award's winner news and its text (month_award.cpp) ----------

/// The news type of a month award's winner (category 2, written by `AWARD_MANAGER::write_award_news`).
const MONTH_AWARD_NEWS_TYPE: u32 = 0x1b;
/// The version mark the news item carries in slot 9; the text writer rewords nothing while it is there.
const MONTH_AWARD_NEWS_VERSION: u32 = 0x420e9;
/// A month award's staff competition record (`TStaffComp`, award_fields +5): +4 name, +0x37 its gender, +0x38 short
/// name, +0x52 its gender, +0x57 nation.
const STAFF_COMP_NATION: usize = 0x57;

/// Posts the news of the winner of one month slot of a month award.
///
/// The news item (type `MONTH_AWARD_NEWS_TYPE`, local) carries: 0 the award's nation id (`award_base::NATION_ID`)
/// and 1 its index in the nation's awards (`award_base::INDEX`), which `AWARD_MANAGER::write_award_news` uses to find
/// the award again; 2 the winner (`month_award_fields::SLOT_WINNER_ID`) and 3..5 the `names::ID` of his first, second and common names (read without
/// a null check, as the original does); 6 the club id (`SLOT_CLUB_ID`); 7 the month (signed byte) and 8 the year
/// (unsigned word) of the slot; 9 `MONTH_AWARD_NEWS_VERSION`; 10 a draw `utils_random(3)` that picks the text
/// template; 11 whether the winner has no player sheet; with a sheet, 12 his position mask
/// (`exe::formation_get_position_mask`) and 13 his strengths (`exe::staff_get_player_strength_flags` with the slot's
/// club); 14 whether he is a player-manager (`job_for_club()`). Then the receivers: with a transfer manager
/// (`G_TRANSFER_MANAGER`) and a player sheet, the clubs interested in the player
/// (`exe::transfer_manager_collect_player_news_recipients`, kind 0x48, up to 30). The item goes (`news_post`) to the
/// managers of the award's league (`award_base::CLUB_COMP`), or, without one, to those of the staff competition's
/// nation (`award_base::STAFF_COMP`, `STAFF_COMP_NATION`); then to each interested club not playing in the award's
/// league (`club::DIVISION`).
/// Defect of the original: with a player sheet and no club (slot club id −1) the strengths are read with the record
/// before the club table (`G_CLUB_TABLE` − `club::SIZE`); the copy does the same.
/// Original: `MONTH_AWARD::post_winner_news` (0x767330, month_award.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn month_award_post_winner_news(this: usize, slot: u32) {
    month_award_winner_news(this, slot, &mut |item, to| news_post(item, to));
}

/// The winner news of `month_award_post_winner_news`, its items going to `sink` (the game's `news_post`, or a
/// comparer's rehearsal).
pub(crate) unsafe fn month_award_winner_news(this: usize, slot: u32, sink: NewsSink) {
    let v = this.wrapping_add((slot as u8 as i8 as isize as usize).wrapping_mul(11));
    let winner = read_i32(v + month_award_fields::SLOT_WINNER_ID);
    let person = staff_at(winner as isize);
    covered!("sx04b_1 winner news");
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let item = local.as_mut_ptr() as usize;
    exe::new_newsitem(item, MONTH_AWARD_NEWS_TYPE, 0);
    exe::newsitem_set_slot(item, 0, read_u32(this + award_base::NATION_ID));
    exe::newsitem_set_slot(item, 1, read_u32(this + award_base::INDEX));
    exe::newsitem_set_slot(item, 2, winner as u32);
    exe::newsitem_set_slot(item, 3, read_u32(person.first_name() + names::ID));
    exe::newsitem_set_slot(item, 4, read_u32(person.second_name() + names::ID));
    exe::newsitem_set_slot(item, 5, read_u32(person.common_name() + names::ID));
    exe::newsitem_set_slot(item, 6, read_u32(v + month_award_fields::SLOT_CLUB_ID));
    exe::newsitem_set_slot(item, 7, read_i8_as_i32(v + month_award_fields::SLOT_MONTH) as u32);
    exe::newsitem_set_slot(item, 8, read_u16(v + month_award_fields::SLOT_YEAR) as u32);
    exe::newsitem_set_slot(item, 9, MONTH_AWARD_NEWS_VERSION);
    exe::newsitem_set_slot(item, 0xa, exe::utils_random(3) as u32); // sx04b_1: the template draw
    let sheet = person.player();
    exe::newsitem_set_slot(item, 0xb, sheet.is_null() as u32);
    if !sheet.is_null() {
        covered!("sx04b_1 winner with a player sheet");
        exe::newsitem_set_slot(item, 0xc, exe::formation_get_position_mask(sheet) as i16 as i32 as u32);
        let club = club_at(read_i32(v + month_award_fields::SLOT_CLUB_ID) as isize);
        exe::newsitem_set_slot(item, 0xd, exe::staff_get_player_strength_flags(person, club));
    }
    exe::newsitem_set_slot(item, 0xe, (person.job_for_club() == job_kind::PLAYER_MANAGER) as u32);
    let mut clubs = [0usize; 30];
    let mut n: i8 = 0;
    let tm = read_usize(G_TRANSFER_MANAGER);
    if tm != 0 && !person.player().is_null() {
        n = exe::transfer_manager_collect_player_news_recipients(tm, person, 0x48, clubs.as_mut_ptr(), 0x1e) as i8;
    }
    let league = read_usize(this + award_base::CLUB_COMP);
    let comp = read_usize(this + award_base::STAFF_COMP);
    if league != 0 {
        sink(item, NewsPost::Division(league));
    } else if comp != 0 {
        sink(item, NewsPost::Nation(read_usize(comp + STAFF_COMP_NATION)));
    }
    for &club in clubs.iter().take(n.max(0) as usize) {
        if read_usize(club + club::DIVISION) != read_usize(this + award_base::CLUB_COMP) { // sx04b_1: clubs of other leagues
            sink(item, NewsPost::Club(club));
        }
    }
}

/// Comparer for `month_award_post_winner_news`: the news is posted once, so the original runs and the copy is
/// rehearsed from the same random generator (`rehearse_news`: it fills the item and draws without posting); the
/// rehearsal's draws must be the start of the original's.
pub(crate) unsafe extern "thiscall" fn compare_month_award_post_winner_news(this: usize, slot: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_MONTH_AWARD_POST_WINNER_NEWS);
    let mut planned = Vec::new();
    let (same, from_original, from_copy) = rehearse_news(|| o(this, slot), || month_award_winner_news(this, slot, &mut |_, to| planned.push(to)));
    SCOREBOARD.record(same, || format!("month_award_post_winner_news(slot {}) draws original={:?} rust={:?}, rust posts {:?}", slot as u8, from_original, from_copy, planned));
}

/// Writes the headline (`body` 0) or the body of a month award's winner news into `out` (the slots are those of
/// `month_award_post_winner_news`).
///
/// The winner's name is formatted twice from the three name ids (`exe::database_format_name_from_ids`, 0xc8 bytes):
/// long (style 1) and short (style 5). When the winner has a player sheet and the item carries
/// `MONTH_AWARD_NEWS_VERSION`, his position name (`exe::write_position_name`) is written first, with its gender.
/// The headline: by (winner / (draw + 1)) % 6, "<player> takes award", "<player> named as <award>", "Award for
/// <player>", "<player> scoops prize", "<player> wins <month> award", or (5, or a negative remainder) "<award> award
/// decided" (the award by the short name of the staff competition, +5). The body: without a club "<player> has won the
/// <award> award for the month of <month>"; with a club, when the winner has a player sheet, the item carries the mark
/// and the draw is not 0, the player's description (`exe::write_player_description`, replaced by "player-boss" for a
/// player-manager) goes in "<club> <description> <player> is the winner of <month> <award> award" (draw 1) or "The
/// winner of <month> <award> award is <club> <description> <player>" (draw 2); otherwise "<club> <player> has won the
/// <award> award for <month>". The names, the genders and the month (`exe::date_get_month_name`) go in the order the
/// original pushes them. Then the links: the club (short name, 0x7d5), the winner's long and short names (0x7d7) when
/// his names still match the item's ids (`exe::database_staff_matches_names`), and the award's long and short names
/// (0x7d4). Returns the last link's result.
/// Defect of the original: the description's gender goes as a whole dword of which only the low byte was written (the
/// rest is stack leftover); the description reads the byte only.
/// Original: `MONTH_AWARD::write_news` (0x767580, month_award.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn month_award_write_news(this: usize, news: usize, out: usize, body: u32) -> i32 {
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let winner = slot(2) as i32;
    let ids = [slot(3), slot(4), slot(5)];
    let club_id = slot(6) as i32;
    let month = slot(7) as u8 as u32;
    let reworded = slot(9) != MONTH_AWARD_NEWS_VERSION;
    let draw = slot(0xa) as u8 as i8;
    let no_player = slot(0xb);
    let positions = slot(0xc);
    let strengths = slot(0xd);
    let player_boss = slot(0xe);
    let mut description = [0u8; 0xc8];
    let desc = description.as_mut_ptr() as usize;
    let mut gender: u8 = 0;
    if no_player == 0 && !reworded {
        gender = exe::write_position_name(positions, desc, 0xc8, 1);
    }
    let mut long_name = [0u8; 0xc8];
    let mut short_name = [0u8; 0xc8];
    exe::database_format_name_from_ids(ids[0], ids[1], ids[2], long_name.as_mut_ptr(), 0xc8, 1, winner as u32);
    exe::database_format_name_from_ids(ids[0], ids[1], ids[2], short_name.as_mut_ptr(), 0xc8, 5, winner as u32);
    let (long, short) = (long_name.as_ptr() as usize, short_name.as_ptr() as usize);
    let comp = read_usize(this + award_base::STAFF_COMP); // a TStaffComp: its names as in `competition`
    let fmt = news_text_format();
    let buf = NEWS_TEXT_BUFFER;
    if body != 0 {
        covered!("sx04b_1 body");
        let comp_gender = read_i8(comp + competition::GENDER_NAME) as i32 as u32;
        let comp_name = comp + competition::NAME;
        if club_id == -1 {
            let month_name = exe::date_get_month_name(month);
            // "<player> has won the <award> award for the month of <month>."
            fmt(buf, 0xa5ad8c, 0x64, comp_gender, comp_gender, 0x64, 0x64, long, comp_name, month_name);
        } else if no_player == 0 && draw != 0 && !reworded {
            covered!("sx04b_1 body with the description");
            let club = club_at(club_id as isize);
            gender = exe::write_player_description(winner, positions, strengths, desc, 0xc8, gender as u32, 1);
            if player_boss != 0 {
                fmt(desc, 0xa5b0f8); // "player-boss"
            }
            let desc_gender = gender as i8 as i32 as u32;
            let club_gender = read_i8(club.addr() + club::SHORT_GENDER_NAME) as i32 as u32;
            let club_name = club.addr() + club::SHORT_NAME;
            let month_name = exe::date_get_month_name(month);
            if draw == 1 { // sx04b_1: the club-first template
                // "<club> <description> <player> is the winner of <month> <award> award."
                fmt(buf, 0xa5b004, club_gender, club_gender, desc_gender, desc_gender, 0x64, 0x64, comp_gender, comp_gender, club_name, desc, long, month_name, comp_name);
            } else {
                // "The winner of <month> <award> award is <club> <description> <player>."
                fmt(buf, 0xa5af10, 0x64, 0x64, comp_gender, comp_gender, club_gender, club_gender, desc_gender, desc_gender, month_name, comp_name, club_name, desc, long);
            }
        } else {
            let club = club_at(club_id as isize);
            let club_gender = read_i8(club.addr() + club::SHORT_GENDER_NAME) as i32 as u32;
            let month_name = exe::date_get_month_name(month);
            // "<club> <player> has won the <award> award for <month>."
            fmt(buf, 0xa5ae44, club_gender, club_gender, 0x64, comp_gender, comp_gender, 0x64, 0x64, club.addr() + club::SHORT_NAME, long, comp_name, month_name);
        }
    } else {
        covered!("sx04b_1 headline");
        let award_gender = read_i8(comp + competition::SHORT_GENDER_NAME) as i32 as u32;
        let award = comp + competition::SHORT_NAME;
        match (winner / (draw as i32 + 1)) % 6 {
            0 => fmt(buf, 0xa5ad60, short), // "<player> takes award"
            1 => fmt(buf, 0xa5ad04, award_gender, award_gender, short, award), // "<player> named as <award>"
            2 => fmt(buf, 0xa5acd8, short), // "Award for <player>"
            3 => fmt(buf, 0xa5acac, short), // "<player> scoops prize"
            4 => fmt(buf, 0xa5ac5c, 0x64, 0x64, short, exe::date_get_month_name(month)), // "<player> wins <month> award"
            _ => fmt(buf, 0xa5ac18, award_gender, award_gender, award), // "<award> award decided"
        };
    }
    exe::text_set_string(out, buf as *const u8);
    if club_id != -1 {
        let club = club_at(club_id as isize);
        exe::text_add_link(out, (club.addr() + club::SHORT_NAME) as *const u8, 0x7d5, club);
    }
    let person = staff_at(winner as isize);
    if exe::database_staff_matches_names(person, ids[0], ids[1], ids[2], 0usize) != 0 {
        exe::text_add_link(out, long as *const u8, 0x7d7, person);
        exe::text_add_link(out, short as *const u8, 0x7d7, person);
    }
    exe::text_add_link(out, (comp + competition::NAME) as *const u8, 0x7d4, comp);
    exe::text_add_link(out, (comp + competition::SHORT_NAME) as *const u8, 0x7d4, comp)
}

/// Comparer for `month_award_write_news`: side by side into two Texts (`compare_text_writer`: the text, the links and
/// the random generator).
pub(crate) unsafe extern "thiscall" fn compare_month_award_write_news(this: usize, news: usize, out: usize, body: u32) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> i32>(IDX_MONTH_AWARD_WRITE_NEWS);
    let mut r = 0;
    compare_text_writer(out, || r = o(this, news, out, body), |t| {
        month_award_write_news(this, news, t, body);
    }, || format!("month_award_write_news(winner {}, body {})", exe::newsitem_get_slot(news, 2) as i32, body));
    r
}

// ---------- sx04b_3: a nation's matches in its month ratings and the player of the year from the year ratings (nation_awards.cpp) ----------

/// `MONTH_RATINGS::record_match_ratings(fixture, record)` (0x767ee0, month_ratings.cpp, thiscall, ret 8).
const FN_MONTH_RATINGS_RECORD_MATCH_RATINGS: usize = 0x767ee0;
/// `YEAR_RATINGS::collect_best_players(out, max, nation, club nation, excluded nation, home nation, division, mode,
/// max age)` (0x931ae0, year_ratings.cpp, thiscall, ret 0x24): how many were collected (ax).
const FN_YEAR_RATINGS_COLLECT_BEST_PLAYERS: usize = 0x931ae0;
/// `AWARD_SHORTLIST::clear()` (0x418320, thiscall), `add_entry(person, club, f32 score)` (0x4178a0, thiscall, ret
/// 0xc) and `announce(date, nation id, award index)` (0x417a20, thiscall, ret 0xc).
pub(crate) const FN_AWARD_SHORTLIST_CLEAR: usize = 0x418320;
const FN_AWARD_SHORTLIST_ADD_ENTRY: usize = 0x4178a0;
const FN_AWARD_SHORTLIST_ANNOUNCE: usize = 0x417a20;

/// Passes a played match to the nation's month ratings (`MONTH_RATINGS::record_match_ratings` with the same fixture
/// and record), when the nation has month ratings (`nation_awards_fields::MONTH_RATINGS`). The leagues' virtual
/// `record_match_ratings` (England's and Finland's at 0x5a96b0) call it for each league match.
/// Original: `NationAwards::record_match_ratings` (0x7683a0, nation_awards.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn nation_awards_record_match_ratings(this: usize, fixture: usize, record: usize) {
    let ratings = read_usize(this + nation_awards_fields::MONTH_RATINGS);
    if ratings == 0 {
        return;
    }
    covered!("sx04b_3 match to the month ratings");
    let f: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_MONTH_RATINGS_RECORD_MATCH_RATINGS);
    f(ratings, fixture, record); // sx04b_3: the month ratings
}

/// Comparer for `nation_awards_record_match_ratings`: the ratings take each match once, so only the original runs (the
/// month ratings routine it calls has its own comparer).
pub(crate) unsafe extern "thiscall" fn compare_nation_awards_record_match_ratings(this: usize, fixture: usize, record: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_NATION_AWARDS_RECORD_MATCH_RATINGS)(this, fixture, record)
}

/// Writes the three winners `people` into a year award record for the year of `today`: each one's name ids, staff id
/// and club id (−1 without a club); the scores are left as they are (the original never writes them).
unsafe fn year_record_of(today: usize, people: [Staff; 3]) -> [u8; YEAR_RECORD_SIZE] {
    let mut record = [0u8; YEAR_RECORD_SIZE];
    record[..2].copy_from_slice(&read_u16(today + cm_date::YEAR).to_le_bytes());
    for (k, person) in people.into_iter().enumerate() {
        let club = person.club_job();
        let club_id = if club.is_null() { u32::MAX } else { read_u32(club.addr()) };
        year_record_entry(&mut record, k, person, None, club_id);
    }
    record
}

/// A year ratings entry's score for a shortlist, by the collecting `mode`: the mode's weighted rating sum (0, 1 or 2)
/// or, for any other mode, 2 × goals + assists, over the matches, as the x87 rounds it to an f32.
unsafe fn ratings_shortlist_score(entry: YearEntry, mode: u8) -> f32 {
    let total = match mode {
        0..=2 => entry.rating_sum(mode as usize),
        _ => entry.assists() + entry.goals() * 2,
    };
    (total as f64 / entry.matches() as i32 as f64) as f32
}

/// The player of the year from a year ratings table (see `nation_awards_award_player_of_year_from_ratings`), the
/// winners record going to `sink` and the shortlist announced only when `announce`.
pub(crate) unsafe fn nation_awards_award_from_ratings_with(this: usize, today: usize, award: usize, ratings: usize, filters: [u32; 7], announce: bool, sink: YearAwardSink) {
    let shortlist = read_usize(award + AWARD_SHORTLIST);
    // how many: the shortlist's capacity (the word over a dword whose high half is 0), or 3
    let wanted: u32 = if shortlist != 0 { read_u16(shortlist + award_shortlist_fields::CAPACITY) as u32 } else { 3 };
    let n = wanted as u16 as i16;
    let buf = exe::crt_malloc((n as i32 * 4) as u32);
    if buf == 0 {
        tel26_error_box(NATION_AWARDS_CPP, 0xab);
        return;
    }
    if ratings == 0 {
        // defect of the original: the block is not freed
        tel26_error_box(NATION_AWARDS_CPP, 0xb2);
        return;
    }
    covered!("sx04b_3 player of the year from ratings");
    let collect: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32, u32, u32, u32) -> u16 = core::mem::transmute(FN_YEAR_RATINGS_COLLECT_BEST_PLAYERS);
    let [nation, club_nation, excluded, home, division, mode, max_age] = filters;
    let found = collect(ratings, buf, wanted, nation, club_nation, excluded, home, division, mode, max_age) as i16;
    if found < n { // sx04b_3: enough candidates
        exe::crt_free(buf);
        return;
    }
    let entry = |k: usize| YearEntry(read_usize(buf + 4 * k));
    if shortlist != 0 {
        covered!("sx04b_3 shortlist filled");
        let clear: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_AWARD_SHORTLIST_CLEAR);
        clear(shortlist);
        let add: unsafe extern "thiscall" fn(usize, usize, usize, f32) -> u32 = core::mem::transmute(FN_AWARD_SHORTLIST_ADD_ENTRY);
        for k in 0..n.max(0) as usize {
            let person = entry(k).person();
            add(shortlist, person.addr(), person.club_job().addr(), ratings_shortlist_score(entry(k), mode as u8));
        }
        if announce {
            let f: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(FN_AWARD_SHORTLIST_ANNOUNCE);
            f(shortlist, today, read_u32(this + nation_awards_fields::NATION_ID), read_u32(award + award_base::INDEX));
        }
    } else {
        covered!("sx04b_3 winners recorded");
        // without a shortlist n is 3: one entry per winner (the original would put any further one in the third)
        let people = [0, 1, 2].map(|k| entry(k).person());
        sink(award, &year_record_of(today, people));
    }
    exe::crt_free(buf);
}

/// Gives a year award from a year ratings table: the best players of `ratings` (`YEAR_RATINGS::collect_best_players`
/// with the seven filters: nation, club nation, excluded nation, home nation, division, mode, maximum age), as many as
/// the award's shortlist (`AWARD_SHORTLIST`) holds (its capacity) or 3 without one, into a block (`exe::crt_malloc`). Fewer found:
/// nothing. With a shortlist, it is cleared and each candidate is added (`AWARD_SHORTLIST::add_entry`: the person,
/// his club, the score of `ratings_shortlist_score`), then announced with `today`, the nation's id
/// (`nation_awards_fields::NATION_ID`) and the award's index (`award_base::INDEX`). Without one, the three winners (names, staff id, club id or −1) go with the year of `today` to
/// `YEAR_AWARD::record_winners`. The block is freed. Out of memory: the nation_awards.cpp 171 box; no ratings: the 178
/// box. Defects of the original: with no ratings the block is not freed; the record's score dwords are never written
/// (the history keeps stack garbage).
/// Original: `NationAwards::award_player_of_year_from_ratings` (0x7683c0, nation_awards.cpp, thiscall, ret 0x28).
pub(crate) unsafe extern "thiscall" fn nation_awards_award_player_of_year_from_ratings(this: usize, today: usize, award: usize, ratings: usize,
    nation: u32, club_nation: u32, excluded: u32, home: u32, division: u32, mode: u32, max_age: u32) {
    nation_awards_award_from_ratings_with(this, today, award, ratings, [nation, club_nation, excluded, home, division, mode, max_age], true,
        &mut |award, record| record_year_award(award, record));
}

/// A shortlist's count, votes and entries (sorted, since the announcement sorts them by name).
unsafe fn shortlist_contents(shortlist: usize) -> (Vec<u8>, Vec<Vec<u8>>) {
    use award_shortlist_fields as asl;
    let head = (asl::COUNT..0x28).map(|i| read_u8(shortlist + i)).collect();
    let mut entries: Vec<Vec<u8>> = (0..read_i16(shortlist + asl::COUNT).max(0) as usize)
        .map(|k| (0..asl::ENTRY_SIZE).map(|i| read_u8(read_usize(shortlist + asl::ENTRIES) + k * asl::ENTRY_SIZE + i)).collect())
        .collect();
    entries.sort();
    (head, entries)
}

/// Comparer for `nation_awards_award_player_of_year_from_ratings`. Errors (no ratings): only the original. With a
/// shortlist: the copy fills it as a dry run (no announcement), its contents are kept and the shortlist put back; the
/// original runs and must leave the same count, votes and entries (in any order: the announcement sorts them by
/// name). Without one: the copy runs as a dry run (no record), then the original; its newest year award history
/// record must be the planned one, without the score dwords the original never writes.
pub(crate) unsafe extern "thiscall" fn compare_nation_awards_award_player_of_year_from_ratings(this: usize, today: usize, award: usize, ratings: usize,
    nation: u32, club_nation: u32, excluded: u32, home: u32, division: u32, mode: u32, max_age: u32) {
    use award_shortlist_fields as asl;
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32, u32, u32, u32, u32)>(IDX_NATION_AWARDS_AWARD_PLAYER_OF_YEAR_FROM_RATINGS);
    let original = || o(this, today, award, ratings, nation, club_nation, excluded, home, division, mode, max_age);
    let filters = [nation, club_nation, excluded, home, division, mode, max_age];
    if ratings == 0 {
        return original();
    }
    let shortlist = read_usize(award + AWARD_SHORTLIST);
    if shortlist != 0 {
        let regions = [(shortlist, 0x28), (read_usize(shortlist + asl::ENTRIES), read_i16(shortlist + asl::CAPACITY).max(0) as usize * asl::ENTRY_SIZE)];
        let before = crate::comparison::MemorySnapshot::read(&regions);
        crate::rng::pause_trace(|| nation_awards_award_from_ratings_with(this, today, award, ratings, filters, false, &mut |_, _| {}));
        let planned = shortlist_contents(shortlist);
        before.write_back();
        original();
        let got = shortlist_contents(shortlist);
        SCOREBOARD.record(got == planned, || format!("nation_awards_award_player_of_year_from_ratings(award {:#x}) shortlist original {:02x?} rust {:02x?}", award, got, planned));
        return;
    }
    let mut planned = None;
    crate::rng::pause_trace(|| nation_awards_award_from_ratings_with(this, today, award, ratings, filters, false, &mut |_, r| planned = Some(r.to_vec())));
    let list = read_usize(read_usize(G_AWARD_MANAGER) + award_manager_consts::YEAR_AWARD_FILE);
    let garbage: Vec<(usize, usize)> = (0..3).map(|k| (2 + k * YEAR_RECORD_ENTRY + YEAR_RECORD_SCORE, 4)).collect();
    compare_award_record("nation_awards_award_player_of_year_from_ratings", list, award + YEAR_AWARD_HISTORY_FIRST, award + YEAR_AWARD_HISTORY_LAST, planned, &garbage, original);
}

// ---------- batch 337: the news text on how a club's players did for their nation (national_teams.cpp) ----------

/// One argument of a club-players-international template, in the order the original pushes it. The genders and the
/// fixture label's gender are pushed twice (sign-extended), as every `{}` pair of the template reads them.
#[derive(Clone, Copy)]
enum IntlArg {
    /// the club's `gender_name` (+0x37), twice
    ClubGender,
    /// the national side's `short_gender_name` (+0x52), twice
    NationGender,
    /// the opponent side's `short_gender_name`, twice
    OpponentGender,
    /// the fixture label's gender (`exe::comp_util_get_label_gender`), twice
    FixtureGender,
    /// how many players the club had, minus one (once, a `%d`)
    Others,
    /// the club's name (+4)
    ClubName,
    /// the subject's name, mode 1: the first player (no highlight) or the highlighted player
    Subject,
    /// the first player's name, mode 5
    PlayerShort,
    /// the highlighted player's name, mode 5
    HighlightShort,
    /// the list of the club's players ("A and B", "A, B and C"...)
    PlayerList,
    /// the national side's name
    Nation,
    /// the opponent side's name
    Opponent,
    /// the score line, highest first ("2-1")
    Score,
    /// the fixture label ("friendly" lowercased)
    Fixture,
}

// The argument lists of the templates (the subject is the player or the highlighted player).
const INTL_FORM_FULL: &[IntlArg] = &[IntlArg::ClubGender, IntlArg::NationGender, IntlArg::FixtureGender, IntlArg::OpponentGender, IntlArg::ClubName, IntlArg::Subject, IntlArg::Nation, IntlArg::Score, IntlArg::Fixture, IntlArg::Opponent];
const INTL_FORM_FULL_PLAYER_SHORT: &[IntlArg] = &[IntlArg::ClubGender, IntlArg::NationGender, IntlArg::FixtureGender, IntlArg::OpponentGender, IntlArg::ClubName, IntlArg::Subject, IntlArg::Nation, IntlArg::Score, IntlArg::Fixture, IntlArg::Opponent, IntlArg::PlayerShort];
const INTL_FORM_FULL_HIGHLIGHT_SHORT: &[IntlArg] = &[IntlArg::ClubGender, IntlArg::NationGender, IntlArg::FixtureGender, IntlArg::OpponentGender, IntlArg::ClubName, IntlArg::Subject, IntlArg::Nation, IntlArg::Score, IntlArg::Fixture, IntlArg::Opponent, IntlArg::HighlightShort];
const INTL_FORM_NO_FIXTURE: &[IntlArg] = &[IntlArg::ClubGender, IntlArg::NationGender, IntlArg::OpponentGender, IntlArg::ClubName, IntlArg::Subject, IntlArg::Nation, IntlArg::Score, IntlArg::Opponent];
const INTL_FORM_NO_FIXTURE_PLAYER_SHORT: &[IntlArg] = &[IntlArg::ClubGender, IntlArg::NationGender, IntlArg::OpponentGender, IntlArg::ClubName, IntlArg::Subject, IntlArg::Nation, IntlArg::Score, IntlArg::Opponent, IntlArg::PlayerShort];
const INTL_FORM_NO_FIXTURE_SCORE_LAST: &[IntlArg] = &[IntlArg::ClubGender, IntlArg::NationGender, IntlArg::OpponentGender, IntlArg::ClubName, IntlArg::Subject, IntlArg::Nation, IntlArg::Opponent, IntlArg::Score];
const INTL_FORM_NO_FIXTURE_SCORE_LAST_PLAYER_SHORT: &[IntlArg] = &[IntlArg::ClubGender, IntlArg::NationGender, IntlArg::OpponentGender, IntlArg::ClubName, IntlArg::Subject, IntlArg::Nation, IntlArg::Opponent, IntlArg::Score, IntlArg::PlayerShort];
const INTL_FORM_NO_CLUB: &[IntlArg] = &[IntlArg::NationGender, IntlArg::OpponentGender, IntlArg::Subject, IntlArg::Nation, IntlArg::Opponent, IntlArg::Score];
const INTL_FORM_NO_CLUB_SCORE_FIRST: &[IntlArg] = &[IntlArg::NationGender, IntlArg::OpponentGender, IntlArg::Subject, IntlArg::Nation, IntlArg::Score, IntlArg::Opponent];
const INTL_FORM_NATION_FIRST: &[IntlArg] = &[IntlArg::NationGender, IntlArg::FixtureGender, IntlArg::OpponentGender, IntlArg::Nation, IntlArg::Subject, IntlArg::Score, IntlArg::Fixture, IntlArg::Opponent];
const INTL_FORM_SUBJECT_FIRST: &[IntlArg] = &[IntlArg::NationGender, IntlArg::FixtureGender, IntlArg::OpponentGender, IntlArg::Subject, IntlArg::Nation, IntlArg::Score, IntlArg::Fixture, IntlArg::Opponent];
const INTL_FORM_LIST: &[IntlArg] = &[IntlArg::ClubGender, IntlArg::NationGender, IntlArg::FixtureGender, IntlArg::OpponentGender, IntlArg::ClubName, IntlArg::PlayerList, IntlArg::Nation, IntlArg::Score, IntlArg::Fixture, IntlArg::Opponent];
const INTL_FORM_COUNT: &[IntlArg] = &[IntlArg::ClubGender, IntlArg::NationGender, IntlArg::FixtureGender, IntlArg::OpponentGender, IntlArg::Others, IntlArg::ClubName, IntlArg::Nation, IntlArg::Score, IntlArg::Fixture, IntlArg::Opponent];

/// A template and its arguments.
type IntlLine = (usize, &'static [IntlArg]);

/// One rule of a template tree: when `code & mask == mask` (the first that matches), the template for a draw of 0
/// (`zero`) or any other draw (`other`).
struct IntlRule {
    mask: u8,
    zero: IntlLine,
    other: IntlLine,
}

const fn intl_rule(mask: u8, zero: IntlLine, other: IntlLine) -> IntlRule {
    IntlRule { mask, zero, other }
}

/// A template tree: the rules by the player's performance code, and the lines when none matches (or the news item
/// carries no codes), for a draw of 0 and another draw.
struct IntlTree {
    rules: &'static [IntlRule],
    fallback: (IntlLine, IntlLine),
}

/// The templates of a lone player (no highlighted player): won, drawn, lost.
const INTL_PLAYER_TREES: [IntlTree; 3] = {
    [
        IntlTree {
            rules: &[
                intl_rule(0x44, (0xa5f638, INTL_FORM_FULL_PLAYER_SHORT), (0xa5f638, INTL_FORM_FULL_PLAYER_SHORT)),
                intl_rule(0x14, (0xa5f504, INTL_FORM_FULL_PLAYER_SHORT), (0xa5f504, INTL_FORM_FULL_PLAYER_SHORT)),
                intl_rule(0x40, (0xa5f410, INTL_FORM_FULL), (0xa5f410, INTL_FORM_FULL)),
                intl_rule(0x24, (0xa5f2c8, INTL_FORM_FULL_PLAYER_SHORT), (0xa5f2c8, INTL_FORM_FULL_PLAYER_SHORT)),
                intl_rule(0x20, (0xa5f1f0, INTL_FORM_NATION_FIRST), (0xa5f104, INTL_FORM_FULL)),
                intl_rule(0x08, (0xa5f03c, INTL_FORM_NATION_FIRST), (0xa5ef58, INTL_FORM_FULL)),
                intl_rule(0x12, (0xa5eea4, INTL_FORM_NO_CLUB), (0xa5ed70, INTL_FORM_FULL_PLAYER_SHORT)),
                intl_rule(0x10, (0xa5ecc8, INTL_FORM_NO_CLUB), (0xa5eb8c, INTL_FORM_FULL_PLAYER_SHORT)),
                intl_rule(0x02, (0xa5eae8, INTL_FORM_NO_CLUB), (0xa5ea34, INTL_FORM_NO_FIXTURE_SCORE_LAST)),
                intl_rule(0x04, (0xa5e998, INTL_FORM_NO_CLUB), (0xa5e8dc, INTL_FORM_NO_FIXTURE_SCORE_LAST)),
                intl_rule(0x01, (0xa5e814, INTL_FORM_SUBJECT_FIRST), (0xa5e814, INTL_FORM_SUBJECT_FIRST)),
            ],
            fallback: ((0xa5e72c, INTL_FORM_FULL), (0xa5e678, INTL_FORM_NO_FIXTURE_SCORE_LAST)), // sx15_3: a lone player in a win
        },
        IntlTree {
            rules: &[
                intl_rule(0x14, (0xa5e540, INTL_FORM_FULL_PLAYER_SHORT), (0xa5e540, INTL_FORM_FULL_PLAYER_SHORT)),
                intl_rule(0x40, (0xa5e44c, INTL_FORM_FULL), (0xa5e44c, INTL_FORM_FULL)),
                intl_rule(0x24, (0xa5e2f0, INTL_FORM_FULL_PLAYER_SHORT), (0xa5e2f0, INTL_FORM_FULL_PLAYER_SHORT)),
                intl_rule(0x20, (0xa5e218, INTL_FORM_NATION_FIRST), (0xa5e12c, INTL_FORM_FULL)),
                intl_rule(0x08, (0xa5e064, INTL_FORM_NATION_FIRST), (0xa5df80, INTL_FORM_FULL)),
                intl_rule(0x12, (0xa5dea0, INTL_FORM_SUBJECT_FIRST), (0xa5dd68, INTL_FORM_FULL_PLAYER_SHORT)),
                intl_rule(0x10, (0xa5dcc0, INTL_FORM_NO_CLUB_SCORE_FIRST), (0xa5db80, INTL_FORM_FULL_PLAYER_SHORT)),
                intl_rule(0x02, (0xa5dadc, INTL_FORM_NO_CLUB_SCORE_FIRST), (0xa5da10, INTL_FORM_NO_FIXTURE)),
                intl_rule(0x04, (0xa5d974, INTL_FORM_NO_CLUB_SCORE_FIRST), (0xa5d87c, INTL_FORM_NO_FIXTURE_PLAYER_SHORT)),
                intl_rule(0x01, (0xa5d7b0, INTL_FORM_SUBJECT_FIRST), (0xa5d7b0, INTL_FORM_SUBJECT_FIRST)),
            ],
            fallback: ((0xa5d6c8, INTL_FORM_FULL), (0xa5d5e4, INTL_FORM_FULL)),
        },
        IntlTree {
            rules: &[
                intl_rule(0x24, (0xa5d494, INTL_FORM_FULL_PLAYER_SHORT), (0xa5d494, INTL_FORM_FULL_PLAYER_SHORT)),
                intl_rule(0x20, (0xa5d3b4, INTL_FORM_NATION_FIRST), (0xa5d2b8, INTL_FORM_FULL)),
                intl_rule(0x08, (0xa5d1f0, INTL_FORM_NATION_FIRST), (0xa5d0fc, INTL_FORM_FULL)),
                intl_rule(0x02, (0xa5d058, INTL_FORM_NO_CLUB_SCORE_FIRST), (0xa5cf9c, INTL_FORM_NO_FIXTURE)),
                intl_rule(0x04, (0xa5cec8, INTL_FORM_SUBJECT_FIRST), (0xa5cdc4, INTL_FORM_NO_FIXTURE_PLAYER_SHORT)),
                intl_rule(0x01, (0xa5ccf8, INTL_FORM_SUBJECT_FIRST), (0xa5ccf8, INTL_FORM_SUBJECT_FIRST)),
            ],
            fallback: ((0xa5cc18, INTL_FORM_FULL), (0xa5cb4c, INTL_FORM_NO_FIXTURE)),
        },
    ]
};

/// The templates of a highlighted player (named captain or man of the match): won, drawn, lost.
const INTL_HIGHLIGHT_TREES: [IntlTree; 3] = {
    [
        IntlTree {
            rules: &[
                intl_rule(0x44, (0xa62d4c, INTL_FORM_FULL_PLAYER_SHORT), (0xa62d4c, INTL_FORM_FULL_PLAYER_SHORT)),
                intl_rule(0x14, (0xa62c08, INTL_FORM_FULL_PLAYER_SHORT), (0xa62c08, INTL_FORM_FULL_PLAYER_SHORT)),
                intl_rule(0x40, (0xa62afc, INTL_FORM_FULL), (0xa62afc, INTL_FORM_FULL)),
                intl_rule(0x24, (0xa629a4, INTL_FORM_FULL_PLAYER_SHORT), (0xa629a4, INTL_FORM_FULL_PLAYER_SHORT)),
                intl_rule(0x20, (0xa628c4, INTL_FORM_NATION_FIRST), (0xa627c0, INTL_FORM_FULL)),
                intl_rule(0x08, (0xa626f0, INTL_FORM_NATION_FIRST), (0xa625f4, INTL_FORM_FULL)),
                intl_rule(0x12, (0xa62534, INTL_FORM_NO_CLUB), (0xa623e4, INTL_FORM_FULL_HIGHLIGHT_SHORT)),
                intl_rule(0x10, (0xa62334, INTL_FORM_NO_CLUB), (0xa621f0, INTL_FORM_FULL_HIGHLIGHT_SHORT)),
                intl_rule(0x02, (0xa62144, INTL_FORM_NO_CLUB), (0xa62074, INTL_FORM_NO_FIXTURE_SCORE_LAST)),
                intl_rule(0x04, (0xa61fd0, INTL_FORM_NO_CLUB), (0xa61ed0, INTL_FORM_NO_FIXTURE_SCORE_LAST_PLAYER_SHORT)),
                intl_rule(0x01, (0xa61e00, INTL_FORM_SUBJECT_FIRST), (0xa61e00, INTL_FORM_SUBJECT_FIRST)),
            ],
            fallback: ((0xa61d10, INTL_FORM_FULL), (0xa61c4c, INTL_FORM_NO_FIXTURE_SCORE_LAST)),
        },
        IntlTree {
            rules: &[
                intl_rule(0x14, (0xa61b04, INTL_FORM_FULL_PLAYER_SHORT), (0xa61b04, INTL_FORM_FULL_PLAYER_SHORT)),
                intl_rule(0x40, (0xa619f8, INTL_FORM_FULL), (0xa619f8, INTL_FORM_FULL)),
                intl_rule(0x24, (0xa6189c, INTL_FORM_FULL_PLAYER_SHORT), (0xa6189c, INTL_FORM_FULL_PLAYER_SHORT)),
                intl_rule(0x20, (0xa617bc, INTL_FORM_NATION_FIRST), (0xa616b4, INTL_FORM_FULL)),
                intl_rule(0x08, (0xa615e4, INTL_FORM_NATION_FIRST), (0xa614e4, INTL_FORM_FULL)),
                intl_rule(0x12, (0xa613f8, INTL_FORM_SUBJECT_FIRST), (0xa612a4, INTL_FORM_FULL_HIGHLIGHT_SHORT)),
                intl_rule(0x10, (0xa611cc, INTL_FORM_SUBJECT_FIRST), (0xa61084, INTL_FORM_FULL_HIGHLIGHT_SHORT)),
                intl_rule(0x02, (0xa60fb0, INTL_FORM_SUBJECT_FIRST), (0xa60eb4, INTL_FORM_FULL)),
                intl_rule(0x04, (0xa60de8, INTL_FORM_SUBJECT_FIRST), (0xa60cbc, INTL_FORM_FULL_PLAYER_SHORT)),
                intl_rule(0x01, (0xa60be8, INTL_FORM_SUBJECT_FIRST), (0xa60be8, INTL_FORM_SUBJECT_FIRST)),
            ],
            fallback: ((0xa60af8, INTL_FORM_FULL), (0xa60a04, INTL_FORM_FULL)),
        },
        IntlTree {
            rules: &[
                intl_rule(0x24, (0xa608a8, INTL_FORM_FULL_PLAYER_SHORT), (0xa608a8, INTL_FORM_FULL_PLAYER_SHORT)),
                intl_rule(0x20, (0xa607c4, INTL_FORM_NATION_FIRST), (0xa606b0, INTL_FORM_FULL)),
                intl_rule(0x08, (0xa605e0, INTL_FORM_NATION_FIRST), (0xa604d4, INTL_FORM_FULL)),
                intl_rule(0x02, (0xa60428, INTL_FORM_NO_CLUB_SCORE_FIRST), (0xa60350, INTL_FORM_NO_FIXTURE)),
                intl_rule(0x04, (0xa60274, INTL_FORM_SUBJECT_FIRST), (0xa60168, INTL_FORM_NO_FIXTURE_PLAYER_SHORT)),
                intl_rule(0x01, (0xa60094, INTL_FORM_SUBJECT_FIRST), (0xa60094, INTL_FORM_SUBJECT_FIRST)),
            ],
            fallback: ((0xa5ffa4, INTL_FORM_FULL), (0xa5fea4, INTL_FORM_FULL)),
        },
    ]
};

/// The player list templates of the lone-player text (two, three, four players) and of the highlighted text (the two
/// or three players after the highlighted one).
const INTL_LIST_TWO: usize = 0xa62f1c;
const INTL_LIST_THREE: usize = 0xa62ea8;
const INTL_LIST_FOUR: usize = 0xa5fce0;

/// "%d other players" from the club (more than four), the "and X" sentence (plain, or after a win when the second
/// player scored or stood out), the list sentence, the score line and the two headlines.
const INTL_OTHERS: usize = 0xa5fdc0;
const INTL_AND_PLAYER: usize = 0xa5fe24;
const INTL_AND_PLAYER_WIN: usize = 0xa5fe64;
const INTL_AND_LIST: usize = 0xa5fd70;
const INTL_SCORE_LINE: usize = 0x9c116c;
const INTL_HEADLINE_PLAYER: usize = 0xa5ca9c;
const INTL_HEADLINE_CLUB: usize = 0xa5caf0;
/// "Friendly", lowercased at the start when it is the fixture label.
const INTL_FRIENDLY: usize = 0x9db958;

/// The slot that ends the players in the news item (`send_club_international_report_news`).
const INTL_CODES_MARKER: u32 = 0x420e9;

/// The slots of the club international report news (0x24), as the body reads them: the match record, the highlighted
/// player (id and the three name ids; id −1 when he is not from the club), how many of the club's players there were,
/// the club, up to four players (only when there were at most four), the marker, the draw and the players' codes (only
/// with the marker).
struct IntlReport {
    record: [u8; 0x20],
    highlight: [u32; 4],
    count: i8,
    club: u32,
    players: [[u32; 4]; 4],
    /// the slot after the players is not the marker (more than four players: the players are not read, so the marker
    /// is not where the original looks)
    no_codes: bool,
    draw: u32,
    codes: [u32; 4],
}

impl IntlReport {
    /// How many players' slots the loops read: none above four, otherwise the count (at most four).
    fn players_read(&self) -> usize {
        if self.count > 4 { 0 } else { self.count.clamp(0, 4) as usize }
    }

    unsafe fn read(news: usize) -> IntlReport {
        let mut r = IntlReport { record: [0; 0x20], highlight: [0; 4], count: 0, club: 0, players: [[0; 4]; 4], no_codes: false, draw: 0, codes: [0; 4] };
        exe::date_date_54a110(r.record.as_mut_ptr() as usize);
        let mut k: u8 = 0;
        exe::read_match_record_from_news(news, &mut k, r.record.as_mut_ptr());
        let mut next = || {
            let v = exe::newsitem_get_slot(news, k as u32);
            k = k.wrapping_add(1);
            v
        };
        for v in r.highlight.iter_mut() {
            *v = next();
        }
        r.count = next() as i8;
        r.club = next();
        for i in 0..r.players_read() {
            for v in r.players[i].iter_mut() {
                *v = next();
            }
        }
        r.no_codes = next() != INTL_CODES_MARKER;
        if !r.no_codes {
            r.draw = next();
            for i in 0..r.players_read() {
                r.codes[i] = next();
            }
        }
        r
    }

    /// The highlighted text with more than four players: the original picks its template by a draw it never read (a
    /// stack dword left by earlier calls); the copy uses 0.
    fn draw_uninitialized(&self) -> bool {
        self.highlight[0] as i32 != -1 && self.no_codes
    }

    fn record(&self) -> RecordMatch {
        RecordMatch(self.record.as_ptr() as usize)
    }

    unsafe fn sides(&self) -> (Club, Club) {
        intl_sides(self.record())
    }

    /// 0 won, 1 drawn, 2 lost, by the national side's goals.
    unsafe fn result(&self) -> usize {
        let (ours, theirs) = self.record().goals();
        if ours > theirs { 0 } else if ours == theirs { 1 } else { 2 }
    }

    unsafe fn b_international(&self) -> bool {
        intl_b_international(self.record())
    }
}

/// The national side (+0x11 of the record) and its opponent.
unsafe fn intl_sides(record: RecordMatch) -> (Club, Club) {
    (club_at(read_i32(record.0 + 0x11) as isize), record.opponent())
}

/// Whether the match was in the B internationals (the sides' links are 0x7d6 instead of 0x7d5, and the headline names
/// the B side).
unsafe fn intl_b_international(record: RecordMatch) -> bool {
    record.comp_index() as u32 == read_u32(KEY_CLUB_COMP_B_INTERNATIONAL)
}

/// The buffers of the body text.
struct IntlTexts {
    player: [[u8; 0x78]; 4],
    player_short: [[u8; 0x78]; 4],
    highlight: [u8; 0x12c],
    highlight_short: [u8; 0x12c],
    nation: [u8; 0x12c],
    opponent: [u8; 0x12c],
    score: [u8; 0xc],
    fixture: [u8; 0xc8],
    fixture_gender: i8,
    list: [u8; 0x3e8],
}

/// The national side's name for the text: the nation's main side (`exe::database_get_national_team`), short form.
unsafe fn intl_side_name(side: Club, buf: &mut [u8; 0x12c]) {
    let team = exe::database_get_national_team(side.nation());
    exe::club_get_national_team_name(team, 0usize, buf.as_mut_ptr() as usize, 0x12c, 1, 0);
}

/// Writes one template into `NEWS_TEXT_BUFFER` (or `out` for the list) with its arguments.
unsafe fn intl_format(out: usize, template: usize, form: &[IntlArg], subject: usize, r: &IntlReport, t: &IntlTexts) {
    let club = club_at(r.club as i32 as isize);
    let (nation, opponent) = r.sides();
    let mut v = [0usize; 16];
    let mut n = 0;
    let mut push = |x: usize| {
        v[n] = x;
        n += 1;
    };
    for a in form {
        match a {
            IntlArg::ClubGender => {
                push(club.gender_name() as i32 as usize);
                push(club.gender_name() as i32 as usize);
            }
            IntlArg::NationGender => {
                push(nation.short_gender_name() as i8 as i32 as usize);
                push(nation.short_gender_name() as i8 as i32 as usize);
            }
            IntlArg::OpponentGender => {
                push(opponent.short_gender_name() as i8 as i32 as usize);
                push(opponent.short_gender_name() as i8 as i32 as usize);
            }
            IntlArg::FixtureGender => {
                push(t.fixture_gender as i32 as usize);
                push(t.fixture_gender as i32 as usize);
            }
            IntlArg::Others => push((r.count as i32 - 1) as usize),
            IntlArg::ClubName => push(club.name_addr()),
            IntlArg::Subject => push(subject),
            IntlArg::PlayerShort => push(t.player_short[0].as_ptr() as usize),
            IntlArg::HighlightShort => push(t.highlight_short.as_ptr() as usize),
            IntlArg::PlayerList => push(t.list.as_ptr() as usize),
            IntlArg::Nation => push(t.nation.as_ptr() as usize),
            IntlArg::Opponent => push(t.opponent.as_ptr() as usize),
            IntlArg::Score => push(t.score.as_ptr() as usize),
            IntlArg::Fixture => push(t.fixture.as_ptr() as usize),
        }
    }
    news_text_format()(out, template, v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7], v[8], v[9], v[10], v[11], v[12], v[13], v[14], v[15]);
}

/// The template a tree picks for a performance code and a draw (the first rule whose mask the code holds; without
/// codes, the fallback).
fn intl_pick(tree: &IntlTree, code: u8, no_codes: bool, draw: u32) -> IntlLine {
    let by_draw = |zero: IntlLine, other: IntlLine| if draw == 0 { zero } else { other };
    if !no_codes {
        if let Some(rule) = tree.rules.iter().find(|rule| code & rule.mask == rule.mask) {
            return by_draw(rule.zero, rule.other);
        }
    }
    by_draw(tree.fallback.0, tree.fallback.1)
}

/// The headline: one player ("X selected for England match", the highlighted player's or the lone player's name, mode
/// 5, with his link when the names still match) or the club ("Leeds players selected in England match", with the
/// club's link); then the national side's link (0x7d6 in the B internationals, else 0x7d5).
unsafe fn intl_headline(news: usize, out: usize) {
    let mut record = [0u8; 0x20];
    exe::date_date_54a110(record.as_mut_ptr() as usize);
    let mut k: u8 = 0;
    exe::read_match_record_from_news(news, &mut k, record.as_mut_ptr());
    let mut next = || {
        let v = exe::newsitem_get_slot(news, k as u32);
        k = k.wrapping_add(1);
        v
    };
    let mut player: [u32; 4] = [next(), next(), next(), next()];
    let mut count = next() as i8;
    let club = club_at(next() as i32 as isize);
    if player[0] as i32 != -1 {
        count = count.wrapping_add(1); // the highlighted player counts again
    }
    let b_side = intl_b_international(RecordMatch(record.as_ptr() as usize));
    let nation = intl_sides(RecordMatch(record.as_ptr() as usize)).0;
    let link = if b_side { 0x7d6 } else { 0x7d5 };
    let mut nation_name = [0u8; 0x12c];
    if count == 1 || (player[0] as i32 > -1 && count == 2) {
        covered!("sx15_3 headline player");
        if player[0] as i32 == -1 {
            player = [next(), next(), next(), next()];
        }
        let mut name = [0u8; 0x12c];
        exe::database_format_name_from_ids(player[1], player[2], player[3], name.as_mut_ptr(), 0x12c, 5, player[0]);
        exe::club_get_national_team_name(nation, b_side as usize, nation_name.as_mut_ptr() as usize, 0x12c, 1, 1);
        let g = nation.short_gender_name() as i8 as i32;
        news_text_format()(NEWS_TEXT_BUFFER, INTL_HEADLINE_PLAYER, g, g, name.as_ptr() as usize, nation_name.as_ptr() as usize);
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        let p = staff_at(player[0] as i32 as isize);
        if exe::database_staff_matches_names(p, player[1], player[2], player[3], 0usize) != 0 {
            exe::text_add_link(out, name.as_ptr(), 0x7d7, p);
        }
    } else {
        covered!("sx15_3 headline club");
        exe::club_get_national_team_name(nation, b_side as usize, nation_name.as_mut_ptr() as usize, 0x12c, 1, 1);
        let (gc, gn) = (club.short_gender_name() as i8 as i32, nation.short_gender_name() as i8 as i32);
        news_text_format()(NEWS_TEXT_BUFFER, INTL_HEADLINE_CLUB, gc, gc, gn, gn, club.addr() + club::SHORT_NAME, nation_name.as_ptr() as usize);
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        exe::text_add_link(out, (club.addr() + club::SHORT_NAME) as *const u8, 0x7d5, club);
    }
    exe::text_add_link(out, nation_name.as_ptr(), link, nation);
}

/// The body's shared parts: the players' names (modes 1 and 5), both sides' names, the fixture label and its gender
/// ("Friendly" lowercased) and the score line, highest first.
unsafe fn intl_body_texts(r: &IntlReport) -> IntlTexts {
    let mut t = IntlTexts {
        player: [[0; 0x78]; 4],
        player_short: [[0; 0x78]; 4],
        highlight: [0; 0x12c],
        highlight_short: [0; 0x12c],
        nation: [0; 0x12c],
        opponent: [0; 0x12c],
        score: [0; 0xc],
        fixture: [0; 0xc8],
        fixture_gender: 0,
        list: [0; 0x3e8],
    };
    for i in 0..r.players_read() {
        let [id, first, second, common] = r.players[i];
        exe::database_format_name_from_ids(first, second, common, t.player[i].as_mut_ptr(), 0x78, 1, id);
        exe::database_format_name_from_ids(first, second, common, t.player_short[i].as_mut_ptr(), 0x78, 5, id);
    }
    let (nation, opponent) = r.sides();
    intl_side_name(nation, &mut t.nation);
    intl_side_name(opponent, &mut t.opponent);
    // the round and stage words go as the dwords at +0xc and +0xe, as the original pushes them
    let (comp, round, stage) = (r.record().comp(), read_u32(r.record().0 + 0xc), read_u32(r.record().0 + 0xe));
    exe::comp_util_format_comp_stage_round_label(comp, round, stage, 1, t.fixture.as_mut_ptr(), 0xc8);
    t.fixture_gender = exe::comp_util_get_label_gender(comp as u32, round, stage, 1) as i8;
    if news_streq(t.fixture.as_ptr() as usize, INTL_FRIENDLY) {
        t.fixture[0] = b'f';
    }
    let (ours, theirs) = r.record().goals();
    let (high, low) = if ours < theirs { (theirs, ours) } else { (ours, theirs) };
    news_text_format()(t.score.as_mut_ptr() as usize, INTL_SCORE_LINE, high as i32, low as i32);
    t
}

/// The body without a highlighted player: two to four players make a list first; one player (or two when the first
/// has a code) gets the lone-player tree; three or four (or two without a code) the list sentence by the result; more
/// than four the count sentence by the result. Two players then add "and X" (the win form when the second player's
/// code holds 0x7a and the side won).
unsafe fn intl_body_players(r: &IntlReport, t: &mut IntlTexts, out: usize) {
    let player: [usize; 4] = core::array::from_fn(|i| t.player[i].as_ptr() as usize);
    let p = |i: usize| player[i];
    let list = t.list.as_mut_ptr() as usize;
    match r.count {
        2 => { news_text_format()(list, INTL_LIST_TWO, p(0), p(1)); }
        3 => { news_text_format()(list, INTL_LIST_THREE, p(0), p(1), p(2)); }
        4 => { news_text_format()(list, INTL_LIST_FOUR, p(0), p(1), p(2), p(3)); }
        _ => {}
    }
    let result = r.result();
    let single = r.count == 1 || (r.count == 2 && r.codes[0] != 0);
    if single {
        covered!("sx15_3 lone player");
        let (template, form) = intl_pick(&INTL_PLAYER_TREES[result], r.codes[0] as u8, r.no_codes, r.draw);
        intl_format(NEWS_TEXT_BUFFER, template, form, p(0), r, t);
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        if r.count == 2 {
            let (ours, theirs) = r.record().goals();
            let template = if r.codes[1] as u8 & 0x7a == 0 || ours <= theirs { INTL_AND_PLAYER } else { INTL_AND_PLAYER_WIN };
            news_text_format()(NEWS_TEXT_BUFFER, template, p(1));
            exe::text_append_string(out, NEWS_TEXT_BUFFER);
        }
    } else if r.count == 2 || r.count == 3 || r.count == 4 {
        covered!("sx15_3 player list");
        let template = [0xa5f968, 0xa5f878, 0xa5f788][result]; // sx15_3: the list sentence by the result
        intl_format(NEWS_TEXT_BUFFER, template, INTL_FORM_LIST, 0, r, t);
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    } else {
        covered!("sx15_3 player count");
        intl_format(NEWS_TEXT_BUFFER, [0xa5fc08, 0xa5fb30, 0xa5fa58][result], INTL_FORM_COUNT, 0, r, t);
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    }
}

/// The body with a highlighted player: three or four players make a list of the others first; the highlighted tree by
/// the result, the highlighted player's code and the draw; then two players add "and X", three or four "and the
/// list", more than four "and N other players from the club"; then the highlighted player's links (modes 1 and 5)
/// when his names still match.
unsafe fn intl_body_highlight(r: &IntlReport, t: &mut IntlTexts, out: usize) {
    let player: [usize; 4] = core::array::from_fn(|i| t.player[i].as_ptr() as usize);
    let p = |i: usize| player[i];
    let list = t.list.as_mut_ptr() as usize;
    match r.count {
        3 => { news_text_format()(list, INTL_LIST_TWO, p(1), p(2)); }
        4 => { news_text_format()(list, INTL_LIST_THREE, p(1), p(2), p(3)); }
        _ => {}
    }
    let [id, first, second, common] = r.highlight;
    exe::database_format_name_from_ids(first, second, common, t.highlight.as_mut_ptr(), 0x12c, 1, id);
    exe::database_format_name_from_ids(first, second, common, t.highlight_short.as_mut_ptr(), 0x12c, 5, id);
    covered!("sx15_3 highlighted player");
    let (template, form) = intl_pick(&INTL_HIGHLIGHT_TREES[r.result()], r.codes[0] as u8, r.no_codes, r.draw);
    intl_format(NEWS_TEXT_BUFFER, template, form, t.highlight.as_ptr() as usize, r, t);
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    let (ours, theirs) = r.record().goals();
    match r.count {
        2 => {
            let template = if r.codes[1] as u8 & 0x7a == 0 || ours <= theirs { INTL_AND_PLAYER } else { INTL_AND_PLAYER_WIN };
            news_text_format()(NEWS_TEXT_BUFFER, template, p(1));
            exe::text_append_string(out, NEWS_TEXT_BUFFER);
        }
        3 | 4 => {
            news_text_format()(NEWS_TEXT_BUFFER, INTL_AND_LIST, t.list.as_ptr() as usize);
            exe::text_append_string(out, NEWS_TEXT_BUFFER);
        }
        n if n > 4 => {
            covered!("sx15_3 other players");
            let club = club_at(r.club as i32 as isize);
            let g = club.short_gender_name() as i8 as i32;
            news_text_format()(NEWS_TEXT_BUFFER, INTL_OTHERS, g, g, n as i32 - 1, club.addr() + club::SHORT_NAME);
            exe::text_append_string(out, NEWS_TEXT_BUFFER);
        }
        _ => {}
    }
    let staff = staff_at(id as i32 as isize);
    if exe::database_staff_matches_names(staff, first, second, common, 0usize) != 0 {
        exe::text_add_link(out, t.highlight.as_ptr(), 0x7d7, staff);
        exe::text_add_link(out, t.highlight_short.as_ptr(), 0x7d7, staff);
    }
}

/// The body's links: each player read whose names still match (modes 1 and 5), the score (when the fixture is still in
/// the match stats), the club, and both sides (0x7d6 in the B internationals, else 0x7d5).
unsafe fn intl_body_links(r: &IntlReport, t: &IntlTexts, out: usize) {
    for i in 0..r.players_read() {
        let [id, first, second, common] = r.players[i];
        let staff = staff_at(id as i32 as isize);
        if exe::database_staff_matches_names(staff, first, second, common, 0usize) != 0 {
            exe::text_add_link(out, t.player[i].as_ptr(), 0x7d7, staff);
            exe::text_add_link(out, t.player_short[i].as_ptr(), 0x7d7, staff);
        }
    }
    r.record().link_score(out, &t.score);
    let club = club_at(r.club as i32 as isize);
    exe::text_add_link(out, club.name_addr() as *const u8, 0x7d5, club);
    let (nation, opponent) = r.sides();
    let link = if r.b_international() { 0x7d6 } else { 0x7d5 };
    exe::text_add_link(out, t.nation.as_ptr(), link, nation);
    exe::text_add_link(out, t.opponent.as_ptr(), link, opponent);
}

/// Writes the headline or the body of the news on how a club's players did for their nation (news 0x24, sent to the
/// club's manager by `send_club_international_report_news`).
///
/// The news item carries the match record, the highlighted player (the national side's highlight when he is from the
/// club, else −1), how many of the club's players took part, the club, up to four players, the marker 0x420e9, a
/// draw of 0 or 1 and each player's performance code (`match_performance_code`: 1 missed a penalty, 2/8/0x20/0x40
/// scored one/two/three/more, 4 left the pitch early, 0x10 man of the match).
///
/// Headline (`full` = 0): `intl_headline`. Body: the players' names, both sides' names (the nation's main side),
/// the fixture label (form 1, "friendly" lowercased) and the score line; then `intl_body_highlight` when there is a
/// highlighted player, else `intl_body_players`; then the links (`intl_body_links`). The templates are picked from the
/// trees by the result (won, drawn, lost), the first rule whose mask the first player's code holds (0x44, 0x14, 0x40,
/// 0x24, 0x20, 8, 0x12, 0x10, 2, 4, 1; not all in every tree), and the draw.
///
/// Defects of the original: with more than four players of the club, the players' slots are not read, so the marker is
/// not where the original looks for it: there are no codes, the texts use the count sentence and, with a highlighted
/// player, the template is picked by a draw that was never read (an uninitialised stack dword; the copy uses 0 and the
/// comparer leaves that case to the original).
/// Original: `NationalTeams::write_club_players_international_text` (0x770d20, national_teams.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_club_players_international_text(_this: usize, news: usize, full: u32, out: usize) {
    if full == 0 {
        intl_headline(news, out);
        return;
    }
    let r = IntlReport::read(news);
    let mut t = intl_body_texts(&r);
    if r.highlight[0] as i32 == -1 {
        intl_body_players(&r, &mut t, out);
    } else {
        intl_body_highlight(&r, &mut t, out);
    }
    intl_body_links(&r, &t, out);
}

/// Comparer for `write_club_players_international_text`: side by side into two Texts (`compare_text_writer`); the
/// highlighted body with more than four players (a template picked by an uninitialised stack dword) runs only the
/// original.
pub(crate) unsafe extern "thiscall" fn compare_write_club_players_international_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(IDX_WRITE_CLUB_PLAYERS_INTERNATIONAL_TEXT);
    if full != 0 && IntlReport::read(news).draw_uninitialized() {
        covered!("sx15_3 uninitialized draw");
        return o(this, news, full, out);
    }
    compare_text_writer(out, || o(this, news, full, out), |t| write_club_players_international_text(this, news, full, t),
        || format!("write_club_players_international_text(news {:#x}, full {})", news, full));
}

// ---------- batch 337: the national call-up news text (national_teams.cpp) ----------

/// Up to three called-up players read from a call-up news item (slots 10.., four per player: id, first, second and
/// common name ids) with their names formatted, the repeated ones dropped.
pub(crate) struct CallUpPlayers {
    pub(crate) ids: [u32; 3],
    pub(crate) first: [u32; 3],
    pub(crate) second: [u32; 3],
    pub(crate) common: [u32; 3],
    pub(crate) names: [[u8; 200]; 3],
}

/// Reads the `count` (< 4) players of the call-up news: player k from slot 10 + 4k (id, first, second, common name
/// ids), his name formatted into `names[i]` (`exe::database_format_name_from_ids`, mode 1 in the body, 5 in the
/// headline). A player whose id repeats an earlier one is read but then overwritten by the next one, and still counts
/// as a read: the loop runs until `count` different players are kept, reading further slots. Returns the players and
/// the number of repeats.
pub(crate) unsafe fn read_call_up_players(news: usize, count: i8, body: bool) -> (CallUpPlayers, i8) {
    let mut p = CallUpPlayers { ids: [0; 3], first: [0; 3], second: [0; 3], common: [0; 3], names: [[0; 200]; 3] };
    let mut slot: u8 = 10;
    let mut repeats: i8 = 0;
    let mut i: i8 = 0;
    while i < count {
        let k = i as usize;
        p.ids[k] = exe::newsitem_get_slot(news, slot as u32);
        p.first[k] = exe::newsitem_get_slot(news, slot.wrapping_add(1) as u32);
        p.second[k] = exe::newsitem_get_slot(news, slot.wrapping_add(2) as u32);
        p.common[k] = exe::newsitem_get_slot(news, slot.wrapping_add(3) as u32);
        slot = slot.wrapping_add(4);
        let mode = if body { 1 } else { 5 };
        exe::database_format_name_from_ids(p.first[k], p.second[k], p.common[k], p.names[k].as_mut_ptr(), 200, mode, p.ids[k]);
        if (0..i).any(|j| p.ids[j as usize] == p.ids[k]) {
            covered!("sx15_4 repeated player");
            i -= 1; // the next player takes this place
            repeats += 1;
        }
        i += 1;
    }
    (p, repeats)
}

/// Links each called-up player's name (0x7d7) when the person still has those names
/// (`exe::database_staff_matches_names`).
pub(crate) unsafe fn link_call_up_players(out: usize, p: &CallUpPlayers, count: i8) {
    for k in 0..count.max(0) as usize {
        let person = staff_at(p.ids[k] as i32 as isize);
        if exe::database_staff_matches_names(person, p.first[k], p.second[k], p.common[k], 0usize) != 0 {
            exe::text_add_link(out, p.names[k].as_ptr(), 0x7d7, person);
        }
    }
}

/// The number of players in words with a capital first letter (`exe::utils_number_to_words`, 0x32 bytes, then
/// `exe::utils_char_to_upper` on the first byte).
pub(crate) unsafe fn players_in_words(count: i8) -> [u8; 0x34] {
    let mut words = [0u8; 0x34];
    exe::utils_number_to_words(count as i32, words.as_mut_ptr(), 0x32, 1);
    words[0] = exe::utils_char_to_upper(read_i32(words.as_ptr() as usize));
    words
}

/// The opponents of the call-up as one phrase in `out`: one club, its short name; two, "A and B" (0xa5c134), or one
/// name when they are the same club; three, "A, B and C" (0xa5c0f8), with the repeats folded into the two-club phrase
/// or one name. With three where the first two are the same club, the second slot takes the third club (the links
/// read it later). Other counts leave `out` as it is.
pub(crate) unsafe fn write_call_up_opponents(clubs: &mut [Club; 3], count: i8, out: &mut [u8; 300]) {
    let fmt = news_text_format();
    let gender = |c: Club| c.short_gender_name() as i8 as i32;
    let short = |c: Club| c.addr() + club::SHORT_NAME;
    let copy = |c: Club, out: &mut [u8; 300]| {
        let name = core::slice::from_raw_parts(short(c) as *const u8, 26);
        let n = name.iter().position(|&b| b == 0).unwrap_or(25) + 1;
        out[..n].copy_from_slice(&name[..n]);
    };
    let two = |a: Club, b: Club, out: &mut [u8; 300]| {
        fmt(out.as_mut_ptr() as usize, 0xa5c134, gender(a), gender(a), gender(b), gender(b), short(a), short(b));
    };
    match count {
        1 => copy(clubs[0], out),
        2 if clubs[0] == clubs[1] => copy(clubs[0], out),
        2 => two(clubs[0], clubs[1], out),
        3 if clubs[0] == clubs[1] => {
            clubs[1] = clubs[2];
            if clubs[0] != clubs[2] {
                two(clubs[0], clubs[1], out);
            } else {
                copy(clubs[0], out);
            }
        }
        3 if clubs[0] == clubs[2] => two(clubs[0], clubs[1], out),
        3 if clubs[1] == clubs[2] => two(clubs[1], clubs[2], out),
        3 => {
            covered!("sx15_4 three opponents");
            let (a, b, c) = (clubs[0], clubs[1], clubs[2]);
            fmt(out.as_mut_ptr() as usize, 0xa5c0f8, gender(a), gender(a), gender(b), gender(b), gender(c), gender(c), short(a), short(b), short(c));
        }
        _ => {}
    }
}

/// The call-up news without opponents (slot 2 set): the headline "<national team> name squad for <competition>"
/// (0xa5c15c) with the national team (long name) and competition links, or the body by the player count: one
/// (0xa5c444), two (0xa5c378), three (0xa5c280) by name, more (0xa5c1c0) with the count in words and the club; then the
/// player links (or the club's, 0x7d5, with four or more), the national team's short name (0x7d5) and the
/// competition (2000).
pub(crate) unsafe fn write_call_up_without_opponents(out: usize, body: bool, team: Club, club_: Club, comp: Competition, p: &CallUpPlayers, count: i8) {
    let fmt = news_text_format();
    let tg = team.gender_name() as i32;
    let cg = comp.short_gender_name() as i8 as i32;
    let (tname, cname) = (team.name_addr(), comp.short_name_addr());
    if !body {
        fmt(NEWS_TEXT_BUFFER, 0xa5c15c, tg, tg, cg, cg, tname, cname);
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        exe::text_add_link(out, tname as *const u8, 0x7d5, team);
        exe::text_add_link(out, cname as *const u8, 2000, comp);
        return;
    }
    let n = |k: usize| p.names[k].as_ptr() as usize;
    match count {
        1 => { fmt(NEWS_TEXT_BUFFER, 0xa5c444, tg, tg, cg, cg, n(0), tname, cname); }
        2 => { fmt(NEWS_TEXT_BUFFER, 0xa5c378, tg, tg, cg, cg, n(0), n(1), tname, cname); }
        3 => { fmt(NEWS_TEXT_BUFFER, 0xa5c280, tg, tg, cg, cg, n(0), n(1), n(2), tname, cname); }
        _ => {
            let words = players_in_words(count);
            let g = club_.short_gender_name() as i8 as i32;
            fmt(NEWS_TEXT_BUFFER, 0xa5c1c0, g, g, tg, tg, cg, cg, words.as_ptr() as usize, club_.addr() + club::SHORT_NAME, tname, cname);
        }
    }
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    if count < 4 {
        link_call_up_players(out, p, count);
    } else {
        exe::text_add_link(out, (club_.addr() + club::SHORT_NAME) as *const u8, 0x7d5, club_);
    }
    exe::text_add_link(out, (team.addr() + club::SHORT_NAME) as *const u8, 0x7d5, team);
    exe::text_add_link(out, cname as *const u8, 2000, comp);
}

/// The headline of the call-up news with opponents: one player "<name> selected by <national team>" (0xa5b710 for the
/// first team, 0xa5b6b4 for the second, 0xa5b654 when `exe::club_is_national_team` says the national team is not a
/// first or second side [inference]), with the player's link written before the text; more players "<club> players
/// called up by <national team>" (0xa5b5f0, 0xa5b588, 0xa5b51c the same way). Then the club's link (0x7d5) and the
/// national team's (0x7d5 first team, 0x7d6 second).
pub(crate) unsafe fn write_call_up_headline(out: usize, team: Club, club_: Club, second: i32, p: &CallUpPlayers, count: i8) {
    let fmt = news_text_format();
    let tg = team.short_gender_name() as i8 as i32;
    let tshort = team.addr() + club::SHORT_NAME;
    let special = exe::club_is_national_team(team) != 0;
    if count == 1 {
        let model = if special { 0xa5b654 } else if second == 0 { 0xa5b710 } else { 0xa5b6b4 };
        fmt(NEWS_TEXT_BUFFER, model, tg, tg, p.names[0].as_ptr() as usize, tshort);
        link_call_up_players(out, p, 1); // before the text, like the original
    } else {
        let model = if special { 0xa5b51c } else if second == 0 { 0xa5b5f0 } else { 0xa5b588 }; // sx15_4: the players headline
        let g = club_.short_gender_name() as i8 as i32;
        fmt(NEWS_TEXT_BUFFER, model, g, g, tg, tg, club_.addr() + club::SHORT_NAME, tshort);
    }
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    exe::text_add_link(out, (club_.addr() + club::SHORT_NAME) as *const u8, 0x7d5, club_);
    exe::text_add_link(out, tshort as *const u8, if second == 0 { 0x7d5 } else { 0x7d6 }, team);
}

/// The body of the call-up news with opponents, by the player count and the opponent count (`opponents`, slot 6):
/// - one player: one opponent 0xa5c050 (with the opponent's gender), up to three 0xa5bfa0, more 0xa5bf18 (without the
///   opponents);
/// - two: 0xa5be40, 0xa5bd60, 0xa5bca8; three: 0xa5bba4, 0xa5ba98, 0xa5b9b4, the same way;
/// - four or more (the count in words and the club): 0xa5b8e8, 0xa5b814, 0xa5b768.
/// `team_name` is the national team's name (`exe::club_get_national_team_name`), `versus` the opponents phrase.
/// Then the player links (or the club's), the national team's name (0x7d5, second team 0x7d6) and each opponent's.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn write_call_up_body(out: usize, team: Club, club_: Club, second: i32, p: &CallUpPlayers, count: i8,
    opponents: i8, clubs: &[Club; 3], team_name: &[u8; 100], versus: &[u8; 300]) {
    let fmt = news_text_format();
    let tg = team.short_gender_name() as i8 as i32;
    let og = clubs[0].short_gender_name() as i8 as i32;
    let (tn, vs) = (team_name.as_ptr() as usize, versus.as_ptr() as usize);
    let n = |k: usize| p.names[k].as_ptr() as usize;
    match count {
        1 if opponents == 1 => { fmt(NEWS_TEXT_BUFFER, 0xa5c050, tg, tg, og, og, n(0), tn, vs); }
        1 if opponents < 4 => { fmt(NEWS_TEXT_BUFFER, 0xa5bfa0, tg, tg, n(0), tn, vs); }
        1 => { fmt(NEWS_TEXT_BUFFER, 0xa5bf18, tg, tg, n(0), tn); }
        2 if opponents == 1 => { fmt(NEWS_TEXT_BUFFER, 0xa5be40, tg, tg, og, og, n(0), n(1), tn, vs); }
        2 if opponents < 4 => { fmt(NEWS_TEXT_BUFFER, 0xa5bd60, tg, tg, 100, n(0), n(1), tn, vs); }
        2 => { fmt(NEWS_TEXT_BUFFER, 0xa5bca8, tg, tg, n(0), n(1), tn); }
        3 if opponents == 1 => { fmt(NEWS_TEXT_BUFFER, 0xa5bba4, tg, tg, og, og, n(0), n(1), n(2), tn, vs); }
        3 if opponents < 4 => { fmt(NEWS_TEXT_BUFFER, 0xa5ba98, tg, tg, 100, n(0), n(1), n(2), tn, vs); }
        3 => { fmt(NEWS_TEXT_BUFFER, 0xa5b9b4, tg, tg, n(0), n(1), n(2), tn); }
        _ => {
            let words = players_in_words(count);
            let w = words.as_ptr() as usize;
            let g = club_.short_gender_name() as i8 as i32;
            let cshort = club_.addr() + club::SHORT_NAME;
            if opponents == 1 {
                fmt(NEWS_TEXT_BUFFER, 0xa5b8e8, g, g, tg, tg, og, og, w, cshort, tn, vs);
            } else if opponents < 4 {
                fmt(NEWS_TEXT_BUFFER, 0xa5b814, g, g, tg, tg, w, cshort, tn, vs); // sx15_4: many players, the opponents
            } else {
                fmt(NEWS_TEXT_BUFFER, 0xa5b768, g, g, tg, tg, w, cshort, tn);
            }
        }
    }
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    if count < 4 {
        link_call_up_players(out, p, count);
    } else {
        exe::text_add_link(out, (club_.addr() + club::SHORT_NAME) as *const u8, 0x7d5, club_);
    }
    let kind = if second == 0 { 0x7d5 } else { 0x7d6 };
    exe::text_add_link(out, team_name.as_ptr(), kind, team);
    if opponents < 4 {
        for c in clubs.iter().take(opponents.max(0) as usize) {
            exe::text_add_link(out, (c.addr() + club::SHORT_NAME) as *const u8, kind, *c);
        }
    }
}

/// Writes the headline or the body of the news (type 0x1d) that a club's players were called up by a national team.
///
/// The slots (`national_teams_post_call_up_news`): 0 the national team (club id), 1 the player's club, 2 no opponents,
/// 3 the competition, 4 the player count, 5 the second team, 6 the opponent count, 7..9 the opponents (club ids),
/// 10.. four per player (id and name ids). With fewer than four players their names are read
/// (`read_call_up_players`), repeats dropped from the count.
///
/// Without opponents (slot 2 set) the text names the competition (`write_call_up_without_opponents`). Otherwise the
/// national team's name is formatted (`exe::club_get_national_team_name`, 100 bytes, with the second-team flag), the
/// opponents read when there are one to three (`write_call_up_opponents`) and the headline
/// (`write_call_up_headline`) or the body (`write_call_up_body`) written.
///
/// Defect of the original: with opponents and none to read (slot 6 zero or negative), the bodies that name the
/// opponents (up to three) append a 300-byte stack buffer that was never written (stack garbage); the copy writes an
/// empty phrase and the comparer leaves that case to the original.
/// Original: `NationalTeams::write_call_up_news_text` (0x76e3b0, national_teams.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_call_up_news_text(_this: usize, news: usize, body: u32, out: usize) {
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let team = club_at(slot(0) as i32 as isize);
    let club_ = club_at(slot(1) as i32 as isize);
    let no_opponents = slot(2) as u8 != 0;
    let comp = Competition(club_comp_record(slot(3) as usize));
    let mut count = slot(4) as u8 as i8;
    let second = slot(5) as u8 as i8 as i32;
    let body = body != 0;
    let mut players = CallUpPlayers { ids: [0; 3], first: [0; 3], second: [0; 3], common: [0; 3], names: [[0; 200]; 3] };
    if count < 4 {
        let (p, repeats) = read_call_up_players(news, count, body);
        players = p;
        count -= repeats;
    }
    if no_opponents {
        covered!("sx15_4 call-up without opponents");
        write_call_up_without_opponents(out, body, team, club_, comp, &players, count);
        return;
    }
    let mut team_name = [0u8; 100];
    exe::club_get_national_team_name(team, second as usize, team_name.as_mut_ptr() as usize, 100, 2, 1);
    let opponents = slot(6) as u8 as i8;
    let mut clubs = [Club(0); 3];
    if opponents < 4 {
        for k in 0..opponents.max(0) as usize {
            clubs[k] = club_at(slot(7 + k as u32) as i32 as isize);
        }
    }
    let mut versus = [0u8; 300]; // the original leaves it unwritten with no opponent (defect)
    write_call_up_opponents(&mut clubs, opponents, &mut versus);
    if !body {
        covered!("sx15_4 call-up headline");
        write_call_up_headline(out, team, club_, second, &players, count);
    } else {
        covered!("sx15_4 call-up body");
        write_call_up_body(out, team, club_, second, &players, count, opponents, &clubs, &team_name, &versus);
    }
}

/// Comparer for `write_call_up_news_text`: side by side into two Texts (`compare_text_writer`). A body with opponents
/// and none to read appends the original's unwritten stack buffer (see the defect): only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_write_call_up_news_text(this: usize, news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(IDX_WRITE_CALL_UP_NEWS_TEXT);
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    if body != 0 && slot(2) as u8 == 0 && slot(6) as u8 as i8 <= 0 {
        covered!("sx15_4 unwritten opponents");
        return o(this, news, body, out);
    }
    compare_text_writer(out, || o(this, news, body, out), |t| write_call_up_news_text(this, news, body, t),
        || format!("write_call_up_news_text(team {}, club {}, players {}, opponents {}, body {})", slot(0) as i32, slot(1) as i32, slot(4) as u8 as i8, slot(6) as u8 as i8, body));
}

// ---------- batch 337: the news of the nations that qualified for a finals (national_teams.cpp) ----------

/// Takes from the new qualifiers list (`national_teams_fields::NEW_QUALIFIERS`, 8-byte items: national team, finals)
/// the national teams of the first finals still in it, clearing their items, into `teams` (club ids); slot 0 of
/// `news` gets the finals' id. Returns the finals (0 when none is left) and the count.
pub(crate) unsafe fn take_qualifiers_of_one_finals(list: Llist, news: usize, teams: &mut [u32; 256]) -> (usize, i8) {
    let mut finals = 0usize;
    let mut count: i8 = 0;
    list.set_cursor(list.head());
    let mut item = exe::llist_get_next(list);
    while item != 0 {
        if read_usize(item) != 0 {
            if finals == 0 {
                finals = read_usize(item + 4);
                exe::newsitem_set_slot(news, 0, read_u32(finals));
                count = 0;
            }
            if read_usize(item + 4) == finals {
                teams[count as u8 as usize] = read_u32(read_usize(item));
                count = count.wrapping_add(1);
                write_usize(item, 0); // taken
            }
        }
        item = exe::llist_get_next(list);
    }
    (finals, count)
}

/// Orders the qualifiers by reputation (`.reputation()` as a signed word), highest first, by swapping pairs (i, j)
/// with i < j < count − 1: the last one never moves (defect of the original: with two qualifiers nothing is sorted).
pub(crate) unsafe fn sort_qualifiers_by_reputation(teams: &mut [u32; 256], count: i8) {
    let n = count as i32;
    let mut i: i8 = 0;
    while (i as i32) < n - 2 {
        let mut j = i + 1;
        while (j as i32) < n - 1 {
            let (a, b) = (teams[i as u8 as usize], teams[j as u8 as usize]);
            if club_at(a as i32 as isize).reputation() < club_at(b as i32 as isize).reputation() {
                teams[i as u8 as usize] = b;
                teams[j as u8 as usize] = a;
            }
            j += 1;
        }
        i += 1;
    }
}

/// Posts one news item (type 0x2c) per finals with the nations that qualified for it and empties the list.
///
/// With new qualifiers (`national_teams_fields::NEW_QUALIFIERS`, an LLIST of national team and finals), one local news
/// item is built (`exe::new_newsitem`, 0x2c) and, while the list has a finals not yet taken
/// (`take_qualifiers_of_one_finals`): slot 0 the finals' id, 1 the count, 2 `utils_random(4)` (the headline wording),
/// 3 `utils_random(2)` (the body wording), 4 and 5 the next hosts of that finals (`exe::hostcountry_find_next_hosts`,
/// 0 and −1 when none), 6 the competition object's year (`cup::YEAR` in `G_COMP_OBJECT_TABLE`) and from 7 the
/// national teams (club ids), ordered by reputation (`sort_qualifiers_by_reputation`). The European Championship
/// (`KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP`) goes to the managers of Europe (`exe::news_send_to_continent_managers`,
/// the `KEY_CONTINENT_EUROPE` record), any other finals to every manager (`exe::news_send_to_all_managers`). The same
/// news item carries over between finals (slots past the count keep the earlier ones). Finally the list is cleared.
/// Original: `NationalTeams::post_qualified_nations_news` (0x7779e0, national_teams.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn post_qualified_nations_news(this: usize) {
    let list = Llist(this + national_teams_fields::NEW_QUALIFIERS);
    if read_i16(list.addr()) == 0 {
        return;
    }
    let mut news = [0u32; 0x38];
    let nt = news.as_mut_ptr() as usize;
    exe::new_newsitem(nt, 0x2c, 0);
    let mut teams = [0u32; 256];
    loop {
        let (finals, count) = take_qualifiers_of_one_finals(list, nt, &mut teams);
        if finals == 0 {
            break;
        }
        covered!("sx15_5 qualified nations news");
        exe::newsitem_set_slot(nt, 1, count as i32 as u32);
        exe::newsitem_set_slot(nt, 2, exe::utils_random(4) as u32);
        exe::newsitem_set_slot(nt, 3, exe::utils_random(2) as u32); // sx15_5: the body wording
        let (mut host, mut co_host) = (0u32, u32::MAX);
        exe::hostcountry_find_next_hosts(read_usize(G_HOST_COUNTRY), read_u32(finals), &mut host, &mut co_host);
        exe::newsitem_set_slot(nt, 4, host);
        exe::newsitem_set_slot(nt, 5, co_host);
        let comp = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(finals).wrapping_mul(4)));
        exe::newsitem_set_slot(nt, 6, read_i16(comp + cup::YEAR) as i32 as u32);
        sort_qualifiers_by_reputation(&mut teams, count);
        let mut k: i8 = 0;
        while k < count {
            exe::newsitem_set_slot(nt, k.wrapping_add(7), teams[k as u8 as usize]);
            k += 1;
        }
        if read_u32(finals) == read_u32(KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP) {
            let europe = read_usize(G_CONTINENT_TABLE).wrapping_add(read_usize(KEY_CONTINENT_EUROPE).wrapping_mul(0xc6));
            exe::news_send_to_continent_managers(read_usize(G_NEWS), nt, europe);
        } else {
            exe::news_send_to_all_managers(read_usize(G_NEWS), nt);
        }
    }
    exe::llist_clear(list);
}

/// Comparer for `post_qualified_nations_news`: it draws, sends news and empties the list (it cannot run twice), so only
/// the original runs; the plan checked afterwards is the empty list.
pub(crate) unsafe extern "thiscall" fn compare_post_qualified_nations_news(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_POST_QUALIFIED_NATIONS_NEWS)(this);
    let left = read_i16(this + national_teams_fields::NEW_QUALIFIERS);
    SCOREBOARD.record(left == 0, || format!("post_qualified_nations_news: {} qualifiers left in the list", left));
}

/// The slots of a qualified nations news item, as the writer reads them.
pub(crate) struct QualifiedNations {
    pub(crate) comp: Competition,
    pub(crate) count: i32,
    pub(crate) headline_kind: i32,
    pub(crate) body_kind: i32,
    pub(crate) host: i32,
    pub(crate) co_host: i32,
    pub(crate) year: i32,
    pub(crate) teams: Vec<Club>,
}

/// The national teams after the first one as one phrase in `out`: one 0xa641a8, two 0xa64164, three 0xa64100 by
/// name; with four the last three (0xa64100), five the last four (0xa6407c), more the 2nd to 6th (0xa63fdc) (the first
/// one is named on its own in those bodies). Nothing for a count below 1.
pub(crate) unsafe fn qualified_teams_phrase(q: &QualifiedNations, out: &mut [u8; 1000]) {
    let fmt = news_text_format();
    let o = out.as_mut_ptr() as usize;
    let g = |k: usize| q.teams[k].short_gender_name() as i8 as i32;
    let n = |k: usize| q.teams[k].addr() + club::SHORT_NAME;
    match q.count {
        1 => { fmt(o, 0xa641a8, g(0), g(0), n(0)); }
        2 => { fmt(o, 0xa64164, g(0), g(0), g(1), g(1), n(0), n(1)); }
        3 => { fmt(o, 0xa64100, g(0), g(0), g(1), g(1), g(2), g(2), n(0), n(1), n(2)); }
        4 => { fmt(o, 0xa64100, g(1), g(1), g(2), g(2), g(3), g(3), n(1), n(2), n(3)); }
        5 => { fmt(o, 0xa6407c, g(1), g(1), g(2), g(2), g(3), g(3), g(4), g(4), n(1), n(2), n(3), n(4)); }
        c if c > 5 => { fmt(o, 0xa63fdc, g(1), g(1), g(2), g(2), g(3), g(3), g(4), g(4), g(5), g(5), n(1), n(2), n(3), n(4), n(5)); }
        _ => {}
    }
}

/// The headline by the drawn wording (slot 2): 0 by the count (one 0xa6425c, two 0xa64208, more 0xa641c8 with the
/// count in words), 1 the competition (0xa642a0), 2 with a single host its name (0xa642e8), else 0xa64348, 3 0xa64384.
pub(crate) unsafe fn write_qualified_nations_headline(q: &QualifiedNations, words: &[u8; 0x78]) {
    let fmt = news_text_format();
    let g = |k: usize| q.teams[k].short_gender_name() as i8 as i32;
    let n = |k: usize| q.teams[k].addr() + club::SHORT_NAME;
    let host = Nation(nation_by_id(q.host as usize));
    match q.headline_kind {
        0 if q.count == 1 => { fmt(NEWS_TEXT_BUFFER, 0xa6425c, g(0), g(0), n(0), q.year); }
        0 if q.count == 2 => { fmt(NEWS_TEXT_BUFFER, 0xa64208, g(0), g(0), g(1), g(1), n(0), n(1)); }
        0 => { fmt(NEWS_TEXT_BUFFER, 0xa641c8, words.as_ptr() as usize, q.year); }
        1 => {
            let cg = q.comp.short_gender_name() as i8 as i32;
            fmt(NEWS_TEXT_BUFFER, 0xa642a0, cg, cg, q.comp.short_name_addr(), q.year);
        }
        2 if q.co_host < 0 => {
            let hg = host.short_gender_name() as i8 as i32;
            fmt(NEWS_TEXT_BUFFER, 0xa642e8, g(0), g(0), hg, hg, n(0), host.addr() + nation::SHORT_NAME, q.year);
        }
        2 => { fmt(NEWS_TEXT_BUFFER, 0xa64348, g(0), g(0), n(0), q.year); }
        _ => { fmt(NEWS_TEXT_BUFFER, 0xa64384, g(0), g(0), n(0), q.year); }
    }
}

/// The body by the drawn wording (slot 3, 0 or else) and the count (more than six, four to six, one, two or three),
/// each with one host or two (slot 5 ≥ 0): the first team by name, the competition (long name, gender +0x37), the
/// hosts, the year and the phrase of the others (`qualified_teams_phrase`); wording 0 with many teams also the count in
/// words. Models (one host / two): wording 0: 0xa6357c/0xa63694, 0xa6332c/0xa63440, 0xa631cc/0xa63268,
/// 0xa63034/0xa630ec; wording 1: 0xa63d88/0xa63e9c, 0xa63b40/0xa63c50, 0xa639a8/0xa63a60, 0xa637d4/0xa638a8.
pub(crate) unsafe fn write_qualified_nations_body(q: &QualifiedNations, words: &[u8; 0x78], others: &[u8; 1000]) {
    let fmt = news_text_format();
    let (t0g, t0n) = (q.teams[0].short_gender_name() as i8 as i32, q.teams[0].addr() + club::SHORT_NAME);
    let cg = q.comp.gender_name() as i8 as i32;
    let cn = q.comp.name_addr();
    let host = Nation(nation_by_id(q.host as usize));
    let co_host = Nation(nation_by_id(q.co_host as usize));
    let (hg, hn) = (host.short_gender_name() as i8 as i32, host.addr() + nation::SHORT_NAME);
    let (h2g, h2n) = (co_host.short_gender_name() as i8 as i32, co_host.addr() + nation::SHORT_NAME);
    let (w, l, y) = (words.as_ptr() as usize, others.as_ptr() as usize, q.year);
    let two_hosts = q.co_host >= 0;
    let many = |one: usize, two: usize| {
        if q.body_kind == 0 {
            if two_hosts {
                fmt(NEWS_TEXT_BUFFER, two, 100, t0g, t0g, cg, cg, hg, hg, h2g, h2g, 100, w, t0n, y, cn, hn, h2n, l);
            } else {
                fmt(NEWS_TEXT_BUFFER, one, 100, t0g, t0g, cg, cg, hg, hg, 100, w, t0n, y, cn, hn, l);
            }
        } else if two_hosts {
            fmt(NEWS_TEXT_BUFFER, two, t0g, t0g, cg, cg, hg, hg, h2g, h2g, 100, t0n, y, cn, hn, h2n, l);
        } else {
            fmt(NEWS_TEXT_BUFFER, one, t0g, t0g, cg, cg, hg, hg, 100, t0n, y, cn, hn, l);
        }
    };
    let (zero, one) = (q.body_kind == 0, q.body_kind != 0);
    match q.count {
        c if c > 6 && zero => many(0xa6357c, 0xa63694),
        c if c > 6 => many(0xa63d88, 0xa63e9c),
        c if c > 3 && zero => many(0xa6332c, 0xa63440),
        c if c > 3 => many(0xa63b40, 0xa63c50),
        1 => {
            let (m1, m2) = if one { (0xa639a8, 0xa63a60) } else { (0xa631cc, 0xa63268) };
            if two_hosts {
                fmt(NEWS_TEXT_BUFFER, m2, t0g, t0g, cg, cg, hg, hg, h2g, h2g, t0n, y, cn, hn, h2n);
            } else {
                fmt(NEWS_TEXT_BUFFER, m1, t0g, t0g, cg, cg, hg, hg, t0n, y, cn, hn);
            }
        }
        _ => {
            let (m1, m2) = if one { (0xa637d4, 0xa638a8) } else { (0xa63034, 0xa630ec) }; // sx15_5: two or three teams
            if two_hosts {
                fmt(NEWS_TEXT_BUFFER, m2, 100, cg, cg, hg, hg, h2g, h2g, l, y, cn, hn, h2n);
            } else {
                fmt(NEWS_TEXT_BUFFER, m1, 100, cg, cg, hg, hg, l, y, cn, hn);
            }
        }
    }
}

/// Writes the headline or the body of the news (type 0x2c) listing the nations that qualified for a finals.
///
/// Reads the slots (`post_qualified_nations_news`): 0 the finals (club competition id), 1 the count, 2 and 3 the drawn
/// wordings, 4 and 5 the hosts (nation ids, −1 none), 6 the year and 7.. the national teams (club ids, by reputation),
/// and the count in words (`exe::utils_number_to_words`, 0x78 bytes). The headline: `write_qualified_nations_headline`;
/// the body: the others' phrase (`qualified_teams_phrase`) and `write_qualified_nations_body`. Then the text and the
/// links: each team's long and short name (0x7d5), each host's short name to its national team (0x7d5,
/// `exe::database_get_national_team`) and the competition's long and short name (2000).
/// Original: `NationalTeams::write_qualified_nations_text` (0x777ca0, national_teams.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_qualified_nations_text(_this: usize, news: usize, body: u32, out: usize) {
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let count = slot(1) as i32;
    let mut q = QualifiedNations {
        comp: Competition(club_comp_record(slot(0) as usize)),
        count,
        headline_kind: slot(2) as i32,
        body_kind: slot(3) as i32,
        host: slot(4) as i32,
        co_host: slot(5) as i32,
        year: slot(6) as i32,
        teams: Vec::new(),
    };
    let mut k: i8 = 7;
    while (k as i32) < count + 7 {
        q.teams.push(club_at(slot(k as u8 as u32) as i32 as isize));
        k = k.wrapping_add(1);
    }
    // ponytail: the poster always sends at least one team; with none the original would read stale stack, the copy club 0
    q.teams.resize(q.teams.len().max(1), club_at(0));
    let mut words = [0u8; 0x78];
    exe::utils_number_to_words(count, words.as_mut_ptr(), 0x78, 0);
    if body == 0 {
        covered!("sx15_5 qualified nations headline");
        write_qualified_nations_headline(&q, &words);
    } else {
        covered!("sx15_5 qualified nations body");
        let mut others = [0u8; 1000];
        qualified_teams_phrase(&q, &mut others);
        write_qualified_nations_body(&q, &words, &others);
    }
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    let mut k: i8 = 0;
    while (k as i32) < count {
        let t = q.teams[k as u8 as usize];
        exe::text_add_link(out, t.name_addr() as *const u8, 0x7d5, t);
        exe::text_add_link(out, (t.addr() + club::SHORT_NAME) as *const u8, 0x7d5, t);
        k = k.wrapping_add(1);
    }
    for id in [q.host, q.co_host] {
        if id > -1 {
            let nation = nation_by_id(id as usize);
            exe::text_add_link(out, (nation + nation::SHORT_NAME) as *const u8, 0x7d5, exe::database_get_national_team(nation));
        }
    }
    exe::text_add_link(out, q.comp.name_addr() as *const u8, 2000, q.comp); // sx15_5: the competition links
    exe::text_add_link(out, q.comp.short_name_addr() as *const u8, 2000, q.comp);
}

/// Comparer for `write_qualified_nations_text`: side by side into two Texts (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_qualified_nations_text(this: usize, news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(IDX_WRITE_QUALIFIED_NATIONS_TEXT);
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    compare_text_writer(out, || o(this, news, body, out), |t| write_qualified_nations_text(this, news, body, t),
        || format!("write_qualified_nations_text(finals {}, count {}, kinds {} {}, hosts {} {}, body {})", slot(0) as i32, slot(1) as i32, slot(2) as i32, slot(3) as i32, slot(4) as i32, slot(5) as i32, body));
}

// ---------- batch 337: the national teams leaving a competition and the club fixtures that clash with call-ups (national_teams.cpp) ----------

/// Every national team of a competition leaves it: for each club of the competition's list (`for_each_comp_club`,
/// stage 0) that is a national team (`exe::database_is_national_team`), its nation's current competition
/// (`national_squad_record::CURRENT_COMPETITION`) becomes −1 and, with a manager manager (`G_MANAGER_MANAGER`), the FA
/// reviews its manager's performance in the competition (`review_national_team_performance`).
/// Original: `NationalTeams::clear_competition_for_comp_clubs` (0x775420, national_teams.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn clear_competition_for_comp_clubs(this: usize, comp: usize) {
    let object = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(comp).wrapping_mul(4)));
    for_each_comp_club(object, 0, |team| {
        let nation = Club(team).nation();
        if exe::database_is_national_team(team) == 0 {
            return;
        }
        covered!("sx15_6 national team leaves the competition");
        write_i32(nation_record_by_id(this, nation.addr()) + national_squad_record::CURRENT_COMPETITION, -1); // sx15_6: no current competition
        let manager_manager = read_usize(G_MANAGER_MANAGER);
        if manager_manager != 0 {
            let review: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_MANAGER_MANAGER_REVIEW_NATIONAL_TEAM_PERFORMANCE);
            review(manager_manager, comp, team);
        }
    });
}

/// `MANAGER_MANAGER::review_national_team_performance` (thiscall(manager manager, competition, national team), ret 8).
const FN_MANAGER_MANAGER_REVIEW_NATIONAL_TEAM_PERFORMANCE: usize = 0x6a5610;

/// Comparer for `clear_competition_for_comp_clubs`: the FA review draws and may sack managers (it cannot run twice),
/// so only the original runs; the plan checked afterwards: no national team of the competition has a current
/// competition.
pub(crate) unsafe extern "thiscall" fn compare_clear_competition_for_comp_clubs(this: usize, comp: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_CLEAR_COMPETITION_FOR_COMP_CLUBS)(this, comp);
    let object = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(comp).wrapping_mul(4)));
    let mut left = Vec::new();
    crate::rng::pause_trace(|| for_each_comp_club(object, 0, |team| {
        let nation = Club(team).nation();
        if exe::database_is_national_team(team) != 0 && read_i32(nation_record_by_id(this, nation.addr()) + national_squad_record::CURRENT_COMPETITION) != -1 {
            left.push(read_u32(team));
        }
    }));
    SCOREBOARD.record(left.is_empty(), || format!("clear_competition_for_comp_clubs({}): national teams still in a competition {:?}", read_i32(comp), left));
}

/// The clubs whose fixture near `date` should move: those marked with a fixture around the date
/// (`national_teams_fields::CLUB_MARKS`, 1 or 2) that have more than two players away on duty dated from the day before
/// to the day after (the call-up entries of `national_teams_fields::CLUB_CALL_UP_RECORDS`, sorted by date: the counts
/// of the entries from `before` on are added until one after `after`). Club ids below the non-national club count.
pub(crate) unsafe fn clubs_with_clashing_call_ups(this: usize, before: &[u8; 8], after: &[u8; 8]) -> Vec<usize> {
    let n = (read_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_usize(G_NATION_COUNT) as i32).wrapping_mul(2));
    let date = |d: &[u8; 8]| (u16::from_le_bytes([d[2], d[3]]), u16::from_le_bytes([d[0], d[1]]) as i16);
    let ((by, bd), (ay, ad)) = (date(before), date(after));
    let mut clubs = Vec::new();
    for i in 0..n.max(0) as usize {
        let mark = read_u8(read_usize(this + national_teams_fields::CLUB_MARKS) + i);
        if mark != 1 && mark != 2 {
            continue;
        }
        let rec = read_usize(this + national_teams_fields::CLUB_CALL_UP_RECORDS) + 6 * i;
        let entries = read_i8(rec + 5);
        let mut away: u8 = 0;
        let mut k: i8 = 0;
        while k < entries {
            let e = read_usize(rec).wrapping_add(9 * k as i32 as usize);
            let (ey, ed) = (read_u16(e + 2), read_i16(e));
            k += 1;
            if by > ey || (by == ey && bd > ed) {
                continue; // before the window
            }
            if ay < ey || (ay == ey && ad < ed) {
                break; // past the window: the rest is later
            }
            away = away.wrapping_add(read_u8(e + 8));
        }
        if away as i8 > 2 { // sx15_6: more than two players away
            clubs.push(i);
        }
    }
    clubs
}

/// Moves the fixtures of the clubs that would play near `date` with more than two players away on international duty.
///
/// The window is the day before and the day after (`exe::date_sub_days`, `exe::date_add_days`). Without the club marks
/// (`national_teams_fields::CLUB_MARKS`) they are built for the date, the day before and the day after
/// (`exe::nationalteams_mark_clubs_with_fixtures`). Then each club of `clubs_with_clashing_call_ups` has its fixture
/// near the date rescheduled by the fixture manager (`FIXTURE_MANAGER::reschedule_club_fixtures_near_date` on
/// `G_FIXTURE_MANAGER`, with the club record and the date).
/// Original: `NationalTeams::reschedule_clashing_club_fixtures` (0x7777e0, national_teams.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn reschedule_clashing_club_fixtures(this: usize, date: usize) {
    let (mut before, mut after) = ([0u8; 8], [0u8; 8]);
    exe::date_sub_days(date, before.as_mut_ptr() as usize, 1);
    exe::date_add_days(date, after.as_mut_ptr() as usize, 1);
    if read_usize(this + national_teams_fields::CLUB_MARKS) == 0 {
        covered!("sx15_6 marks the clubs of three days");
        exe::nationalteams_mark_clubs_with_fixtures(this, date);
        let (mut day, mut next, mut copy) = ([0u8; 8], [0u8; 8], [0u8; 8]);
        exe::date_sub_days(date, day.as_mut_ptr() as usize, 1);
        exe::nationalteams_mark_clubs_with_fixtures(this, day.as_mut_ptr() as usize);
        let r = exe::date_add_days(day.as_mut_ptr() as usize, next.as_mut_ptr() as usize, 2);
        exe::date_operator_3d(day.as_mut_ptr() as usize, copy.as_mut_ptr() as usize, r);
        exe::nationalteams_mark_clubs_with_fixtures(this, day.as_mut_ptr() as usize);
    }
    let reschedule: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_FIXTURE_MANAGER_RESCHEDULE_CLUB_FIXTURES_NEAR_DATE);
    for i in clubs_with_clashing_call_ups(this, &before, &after) {
        covered!("sx15_6 club fixture moved");
        reschedule(read_usize(G_FIXTURE_MANAGER), club_at(i as isize).addr(), date);
    }
}

/// `FIXTURE_MANAGER::reschedule_club_fixtures_near_date` (thiscall(fixture manager, club, date), ret 8).
const FN_FIXTURE_MANAGER_RESCHEDULE_CLUB_FIXTURES_NEAR_DATE: usize = 0x5abb10;

/// Comparer for `reschedule_clashing_club_fixtures`: it builds the club marks and moves fixtures (it cannot run twice),
/// so only the original runs; the plan checked afterwards: the club marks exist (the routine builds them when missing).
pub(crate) unsafe extern "thiscall" fn compare_reschedule_clashing_club_fixtures(this: usize, date: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_RESCHEDULE_CLASHING_CLUB_FIXTURES)(this, date);
    let marks = read_usize(this + national_teams_fields::CLUB_MARKS);
    SCOREBOARD.record(marks != 0, || format!("reschedule_clashing_club_fixtures(day {}): no club marks", read_i16(date)));
}

// --- sx01e_1: Media::post_player_praise_story (0x744240, media.cpp) ---

/// The kinds of the praise story (news item 0xbdd, slot 5), as the text writer reads them (`praise_body`).
const PRAISE_FUTURE_INTERNATIONAL: u8 = 0x29;
const PRAISE_YOUNGSTER: u8 = 0x2a;
const PRAISE_PRAISED: u8 = 0x2b;
const PRAISE_CALL_UP: u8 = 0x2c;
const PRAISE_ON_THE_RISE: u8 = 0x2d;
const PRAISE_BIG_MOVE: u8 = 0x2e;
const PRAISE_ATTITUDE: u8 = 0x2f;
const PRAISE_WORLD_CLASS: u8 = 0x30;
const PRAISE_WORLD_CLASS_B: u8 = 0x31;
const PRAISE_IMPRESSES: u8 = 0x32;
const PRAISE_WORLD_STAGE: u8 = 0x33;
const PRAISE_COMPETITION: u8 = 0x34;

/// `NATIONAL_TEAMS::get_player_squad_class` (thiscall(national teams, person), ret 4, the class in al): 2 when the
/// person's national job (+0x24) passes 0x5409b0, otherwise bit 3 of the byte +0x5e; -1 without a national job.
const FN_NATIONAL_TEAMS_GET_PLAYER_SQUAD_CLASS: usize = 0x76e070;

/// The person's national squad class (`FN_NATIONAL_TEAMS_GET_PLAYER_SQUAD_CLASS` on `G_NATIONAL_TEAMS`).
unsafe fn national_squad_class(person: Staff) -> u8 {
    let f: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_NATIONAL_TEAMS_GET_PLAYER_SQUAD_CLASS);
    f(read_usize(G_NATIONAL_TEAMS), person.addr())
}

/// The low nibble of the person's byte `staff_state::STATUS_FLAGS` in the 79-byte staff state table.
unsafe fn staff_state_nibble(person: Staff) -> i32 {
    let row = read_usize(G_STAFF_STATE_TABLE).wrapping_add((read_i32(person.addr()) as usize).wrapping_mul(staff_state::SIZE));
    (read_u8(row + staff_state::STATUS_FLAGS) & 0xf) as i32
}

/// Whether the player has no transfer offer (`exe::transfer_manager_get_offers_for_player` with 0, 0).
unsafe fn has_no_offers(player: Staff) -> bool {
    exe::transfer_manager_get_offers_for_player(read_usize(G_TRANSFER_MANAGER), player, 0usize, 0) == 0
}

/// The praise story's kind and the draw of slot 8 (the kinds of a young player). Below 22 without caps: a current
/// reputation above 1.3 × the club's with random(ambition) > random(20) and no offer, or the grievance bit 1 of the
/// contract with no offer and the state nibble at least random(ambition / 2): the big move (draw 6); otherwise in the
/// national squad (class 1 or 2) with a home reputation above 4250 and above 30 × the nation's, the future
/// international, else the youngster (draw 4). `contract` is null for a national team: the original reads an
/// uninitialised stack slot there (here the bit is clear).
unsafe fn praise_kind_young(player: Staff, club: Club, contract: Contract) -> (u8, u32) {
    let sheet = player.player();
    let current = sheet.current_reputation() as i32;
    if club.reputation() as f64 * read_f64(rdata::F64_1_3) < current as f64 {
        let ambition = exe::utils_random(player.ambition() as i32);
        if ambition > exe::utils_random(0x14) && has_no_offers(player) {
            covered!("sx01e_1 praise big move");
            return (PRAISE_BIG_MOVE, exe::utils_random(6) as u32);
        }
    }
    if !contract.is_null() && contract.grievances() & 1 != 0 && has_no_offers(player) && staff_state_nibble(player) >= exe::utils_random(player.ambition() as i32 / 2) {
        covered!("sx01e_1 praise big move by grievance");
        return (PRAISE_BIG_MOVE, exe::utils_random(6) as u32);
    }
    let home = sheet.home_reputation() as i32;
    let in_squad = national_squad_class(player) == 1 || national_squad_class(player) == 2;
    let kind = if in_squad && home > 0x109a && home > player.nation().reputation() as i32 * 30 { PRAISE_FUTURE_INTERNATIONAL } else { PRAISE_YOUNGSTER };
    (kind, exe::utils_random(4) as u32)
}

/// The praise story's kind and the draw of slot 8 (the kinds of an older or capped player), in this order: the call-up
/// (a home reputation above 0.8 ×, a world one above 0.4 × and the club's above 0.65 × the nation's reputation, under
/// 28, no caps, random(4) and no national job), on the rise (under 24 with a current reputation below 6000), world
/// class (under 27 − random(4), current above 8000, world below 7000, ability above random(20) + 155, caps below
/// random(5) + 10, club reputation above random(2000) + 6500), world class B (under 33, current above 9000, world
/// above 8500, ability above random(20) + 175, club above random(2000) + 7500) each with draw(3), else the praise
/// (draw 8).
unsafe fn praise_kind_senior(player: Staff, club: Club) -> (u8, u32) {
    let sheet = player.player();
    let age = player.age() as i8 as i32;
    let nation = player.nation().reputation() as f64;
    let club_reputation = club.reputation() as i32;
    if nation * read_f64(rdata::F64_0_8) < sheet.home_reputation() as f64
        && nation * read_f64(rdata::F64_0_4) < sheet.world_reputation() as f64
        && age < 0x1c
        && nation * read_f64(rdata::F64_0_65) < club_reputation as f64
        && player.int_apps() == 0
        && exe::utils_random(4) != 0
        && player.national_job().is_null()
    {
        return (PRAISE_CALL_UP, exe::utils_random(3) as u32);
    }
    let current = sheet.current_reputation() as i32;
    if age < 0x18 && current < 6000 {
        return (PRAISE_ON_THE_RISE, exe::utils_random(3) as u32);
    }
    let limit = 0x1b - exe::utils_random(4);
    if age < limit
        && current > 8000
        && (sheet.world_reputation() as i32) < 7000
        && sheet.current_ability() as i32 > exe::utils_random(0x14) + 0x9b
        && (player.int_apps() as i32) < exe::utils_random(5) + 10
        && club_reputation > exe::utils_random(2000) + 0x1964
    {
        covered!("sx01e_1 praise world class");
        return (PRAISE_WORLD_CLASS, exe::utils_random(3) as u32);
    }
    if age < 0x21
        && current > 9000
        && sheet.world_reputation() as i32 > 0x2134
        && sheet.current_ability() as i32 > exe::utils_random(0x14) + 0xaf
        && club_reputation > exe::utils_random(2000) + 0x1d4c
    {
        return (PRAISE_WORLD_CLASS_B, exe::utils_random(3) as u32);
    }
    (PRAISE_PRAISED, exe::utils_random(8) as u32)
}

/// The praise story's kind and the draw of slot 8. The world stage and the competition keep their kind (draw 4).
/// Otherwise, with random(3) ≠ 0 and today before the day he joined the club plus 30 + random(150) days, he impresses
/// (draw 3); then the young kinds (under 22 without caps, `praise_kind_young`); then his attitude (over 30, leadership
/// and professionalism above 15, more than 400 days at the club, a current reputation above 7250 and random(3) = 0),
/// which keeps the first draw (`first_draw`, random(2)); then the older kinds (`praise_kind_senior`).
unsafe fn praise_story_kind(player: Staff, club: Club, kind: u8, contract: Contract, first_draw: u32) -> (u8, u32) {
    if kind == PRAISE_WORLD_STAGE || kind == PRAISE_COMPETITION {
        return (kind, exe::utils_random(4) as u32);
    }
    if exe::utils_random(3) != 0 {
        let mut settled = [0u8; 8];
        let days = exe::utils_random(0x96) + 0x1e;
        let settled = exe::date_add_days(player.addr() + staff::DATE_JOINED_CLUB, settled.as_mut_ptr() as usize, days as u32);
        if media_date_before(G_CURRENT_DATE, settled) {
            covered!("sx01e_1 praise impresses");
            return (PRAISE_IMPRESSES, exe::utils_random(3) as u32);
        }
    }
    let age = player.age() as i8 as i32;
    if age < 0x16 && player.int_apps() == 0 {
        return praise_kind_young(player, club, contract);
    }
    let sheet = player.player();
    if age > 0x1e
        && sheet.leadership() > 0xf
        && exe::date_days_since(G_CURRENT_DATE, read_u32(player.addr() + staff::DATE_JOINED_CLUB), read_u32(player.addr() + staff::DATE_JOINED_CLUB_LEAP_YEAR)) > 400
        && sheet.current_reputation() > 0x1c52
        && player.professionalism() > 0xf
        && exe::utils_random(3) == 0
    {
        covered!("sx01e_1 praise attitude");
        return (PRAISE_ATTITUDE, first_draw);
    }
    praise_kind_senior(player, club)
}

/// The slots after the match (from 0x11): with a match 1, the match record and the shoot-out comes later; without
/// one 0. Then the transfer slots: with a future transfer (`exe::transfer_manager_get_future_transfer`) the buying
/// club's id (+8 of the offer, `exe::transfer_manager_get_offer_slot`), 1 and the transfer date (day signed, year);
/// otherwise his club's id (-1 without one), 0, -1, -1. Then the state nibble without a future transfer (0 with one).
unsafe fn praise_story_match_and_transfer(s: &mut NewsSlots, player: Staff, club: Club, fixture: usize) {
    s.k = 0x11;
    if fixture != 0 {
        let mut record = [0u8; 0x20]; // the 0x1f-byte match record
        exe::date_date_54a110(record.as_mut_ptr() as usize);
        s.put(1);
        exe::fill_match_record_from_fixture(fixture, club, record.as_mut_ptr() as usize);
        exe::write_match_record_to_news(record.as_mut_ptr() as usize, s.n, &mut s.k);
    } else {
        s.put(0);
    }
    let transfers = read_usize(G_TRANSFER_MANAGER);
    let mut future = 0usize;
    if exe::transfer_manager_get_future_transfer(transfers, player, &mut future as *mut usize as u32) != 0 {
        covered!("sx01e_1 praise with a future transfer");
        let offer = read_usize(exe::transfer_manager_get_offer_slot(transfers, read_i32(future + 8)));
        s.put(read_u32(offer + 8)); // the buying club's id
        s.put(1);
        s.put(read_i16(future) as i32 as u32); // the transfer date: day
        s.put(read_u16(future + 2) as u32); // year
    } else {
        s.put(if player.club_job().is_null() { u32::MAX } else { read_u32(player.club_job().addr()) });
        s.put(0);
        s.put(u32::MAX);
        s.put(u32::MAX);
    }
    s.put(if exe::transfer_manager_get_future_transfer(transfers, player, 0) == 0 { staff_state_nibble(player) as u32 } else { 0 });
}

/// Posts the praise story about a player (news item 0xbdd), raises his reputations and lets the computer manager
/// answer it at once.
///
/// Nothing without the player, his `.player()` sheet, a staff id other than 0 (a defect of the original: the person
/// with id 0 is never praised), a `.current_reputation()` of at least 3500, his `.nation()`, the club, its `.manager()`
/// and `.nation()`. A club that is not a national team (`exe::database_is_national_team`) needs a division other than
/// 0x9cf764 and the player's contract with it (`exe::contract_manager_get_contract_for_club`).
///
/// The slots: 0..3 the player and his names (the common name -1 when he has none), 4 the club id, 5 the kind
/// (`praise_story_kind`; the world stage 0x33 and the competition 0x34 passed in are kept); nothing when the story is
/// already logged (`exe::media_has_logged_story` with 0xbdd and the kind); 6 the board confidence below 4000
/// (`board_record::CONFIDENCE`), 7 his nation id, 8 the kind's draw, 9..12 the club's manager and his names, 13 the
/// computer manager's response (`exe::media_pick_ai_manager_response`, for a club that is not human; otherwise 0), 14
/// his strength flags (`exe::staff_get_player_strength_flags`), 15 his position mask (`exe::formation_get_position_mask`),
/// 16 the competition id: for the competition the fixture's (or, without one, as the world stage, the nation's current
/// competition, `exe::nationalteams_get_current_competition`, nothing without it), otherwise the club's division; the
/// match and transfer slots (`praise_story_match_and_transfer`); the former international pundit and his names
/// (`exe::media_pick_former_international_pundit`, -1 × 4 without one), the next hosts of the competition
/// (`exe::hostcountry_find_next_hosts`, 0 and -1 by default), the shoot-out (1 won, 2 lost, otherwise 0), the pundit's
/// job and club (club job first, then national job; -1 × 2 without), the outlet (`exe::media_pick_outlet_type_for_person`),
/// the pundit of the outlet (`exe::punditlist_pick_random_pundit` with the club's nation), his age and whether he is a
/// goalkeeper (20).
///
/// The world stage goes to every manager when his world reputation plus the club's is above 13750, otherwise to the
/// continent of the club's nation; the competition to every manager; the others to every human club following him
/// (`exe::transfer_manager_collect_player_news_recipients` with 0x25a, up to 30). Then the story is logged (`log_story`
/// with the response), his current and home reputations rise by 200 and the world one by 100 (each up to 10000), he
/// goes onto the clubs' shortlists (`exe::shortlist_manager_add_player_to_club_shortlists`), the chairman of a random
/// club of the same nation favours him (`exe::staff_add_favoured_staff`) and, for a club that is not human, the
/// computer manager's response is applied (`exe::media_apply_player_story_response`).
///
/// Called by the daily loop (0x5d02ab, 0x5d05f7, 0x5d0e1a) and the match day (0x6b715b); behind the post-match gate
/// (`POS_PROLOGUES`, match_day.rs).
/// Original: `Media::post_player_praise_story` (0x744240, media.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn post_player_praise_story(this: usize, player: usize, club: usize, kind: u32, fixture: usize) {
    let (player, club) = (Staff(player), Club(club));
    if player.is_null() || player.player().is_null() || read_i32(player.addr()) == 0 || player.player().current_reputation() < 0xdac || player.nation().is_null() {
        return;
    }
    if club.is_null() || club.manager().is_null() || club.nation().is_null() {
        return;
    }
    let mut contract = Contract(0);
    if exe::database_is_national_team(club) == 0 {
        let division = club.division();
        if division.is_null() || read_i32(division.addr()) == read_i32(0x9cf764) {
            return; // 0x9cf764: the id of a division that gets no praise
        }
        contract = Contract(exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), player, club));
        if contract.is_null() {
            return;
        }
    }
    covered!("sx01e_1 praise story");
    let first_draw = exe::utils_random(2) as u32;
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let mut s = NewsSlots { n: local.as_mut_ptr() as usize, k: 0 };
    exe::new_newsitem(s.n, 0xbdd, 0);
    set_person_name_slots_or_none(s.n, 0, player);
    s.k = 4;
    s.put(read_u32(club.addr()));
    let (kind, draw) = praise_story_kind(player, club, kind as u8, contract, first_draw);
    s.put(kind as i8 as i32 as u32);
    if exe::media_has_logged_story(this, player, club, 0xbdd, kind as u32, core::ptr::null()) != 0 {
        return;
    }
    s.put((read_i16(manager_manager_row(club) + board_record::CONFIDENCE) < 4000) as u32);
    s.put(read_u32(player.nation().addr()));
    s.put(draw);
    s.put_person(club.manager());
    let response = if media_is_human_club(club.addr(), 0, 1) == 0 { exe::media_pick_ai_manager_response(this, club.manager(), player, kind as u32) } else { 0 };
    s.put(response);
    s.put(exe::staff_get_player_strength_flags(player, club));
    s.put(exe::formation_get_position_mask(player.player()) as i16 as i32 as u32);
    let comp_id = if kind == PRAISE_COMPETITION && fixture != 0 {
        read_u32(fixture) // fixture::COMP_ID
    } else if kind == PRAISE_WORLD_STAGE || kind == PRAISE_COMPETITION {
        let teams = read_usize(G_NATIONAL_TEAMS);
        if exe::nationalteams_get_current_competition(teams, club.nation()) == 0 {
            return;
        }
        read_u32(exe::nationalteams_get_current_competition(teams, club.nation()))
    } else {
        read_u32(club.division().addr())
    };
    s.put(comp_id);
    praise_story_match_and_transfer(&mut s, player, club, fixture);
    let pundit = Staff(exe::media_pick_former_international_pundit(this, club, player));
    if pundit.is_null() {
        for _ in 0..4 {
            s.put(u32::MAX);
        }
    } else {
        s.put_person(pundit);
    }
    let (mut hosts, mut next_hosts) = (0u32, u32::MAX);
    exe::hostcountry_find_next_hosts(read_usize(G_HOST_COUNTRY), comp_id, &mut hosts, &mut next_hosts);
    s.put(hosts);
    s.put(next_hosts);
    s.put(if fixture != 0 { news_shootout_result(Fixture(fixture), club).unwrap_or(0) } else { 0 });
    if !pundit.is_null() && !pundit.club_job().is_null() {
        s.put(pundit.job_for_club() as i8 as i32 as u32);
        s.put(read_u32(pundit.club_job().addr()));
    } else if !pundit.is_null() && !pundit.national_job().is_null() {
        s.put(pundit.job_for_nation() as i8 as i32 as u32);
        s.put(read_u32(pundit.national_job().addr()));
    } else {
        s.put(u32::MAX);
        s.put(u32::MAX);
    }
    let outlet = exe::media_pick_outlet_type_for_person(player);
    s.put(outlet);
    let nation = club.nation();
    s.put(exe::punditlist_pick_random_pundit(read_usize(G_PUNDIT_LIST), club, outlet, if nation.is_null() { -1 } else { read_i32(nation.addr()) }) as u32);
    s.put(player.age() as i8 as i32 as u32);
    s.put((player.player().goalkeeper() == 0x14) as u32);
    let mut clubs = [0usize; 30];
    let count = if read_usize(G_TRANSFER_MANAGER) != 0 { player_news_recipients(player, 0x25a, clubs.as_mut_ptr(), 0x1e) as i8 } else { 0 };
    let news = read_usize(G_NEWS);
    if kind == PRAISE_WORLD_STAGE {
        if player.player().world_reputation() as i32 + club.reputation() as i32 > 0x35b6 {
            exe::news_send_to_all_managers(news, s.n);
        } else {
            exe::news_send_to_continent_managers(news, s.n, nation.continent());
        }
    } else if kind == PRAISE_COMPETITION {
        exe::news_send_to_all_managers(news, s.n);
    } else {
        for &c in clubs.iter().take(count.max(0) as usize) {
            exe::news_send_to_club_manager(news, s.n, c);
        }
    }
    log_story(this, player.addr(), club.addr(), 0xbdd, kind as u32, response);
    let sheet = player.player();
    sheet.set_current_reputation(sheet.current_reputation().wrapping_add(200).min(10000));
    sheet.set_home_reputation(sheet.home_reputation().wrapping_add(200).min(10000));
    sheet.set_world_reputation(sheet.world_reputation().wrapping_add(100).min(10000));
    exe::shortlist_manager_add_player_to_club_shortlists(read_usize(read_usize(G_TRANSFER_MANAGER) + transfer_manager_fields::SHORTLIST_MANAGER), player, 1);
    let other = club_at(exe::utils_random(read_i32(G_CLUB_COUNT)) as isize);
    if !other.nation().is_null() && other.nation().addr() == club.nation().addr() {
        exe::staff_add_favoured_staff(other.chairman(), player, 1);
    }
    if media_is_human_club(club.addr(), 0, 1) == 0 {
        exe::media_apply_player_story_response(this, player, club, kind as u32, response);
    }
}

/// Comparer for `post_player_praise_story`: it draws, posts news and changes reputations, so it cannot run twice; only
/// the original runs (in the comparison mode the post-match gate of 0x744240 is installed first and keeps the address,
/// see the item).
pub(crate) unsafe extern "thiscall" fn compare_post_player_praise_story(this: usize, player: usize, club: usize, kind: u32, fixture: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, usize)>(IDX_POST_PLAYER_PRAISE_STORY)(this, player, club, kind, fixture)
}

// --- sx01e_2: Media::create_player_media_story (0x72c1f0, media.cpp) ---

/// `Media::create_player_outburst_story` (0x733a60, thiscall(media, record, fixture), ret 8; sx01d_5).
const FN_MEDIA_CREATE_PLAYER_OUTBURST_STORY: usize = 0x733a60;

/// Hands a player's outburst at the club to `create_player_outburst_story` (through the exe, as the original), with
/// a record on the stack: +0 the player's id, +4 the club id, +0xc 0, +0xe 0 (no assistant's report), +0xf -1 (no
/// other person), +0x13 the outburst kind (0x1e or 0x52); no match.
unsafe fn post_player_outburst_record(this: usize, player: Staff, club: Club, kind: u16) {
    let mut story = [0u8; 0x17];
    let r = story.as_mut_ptr() as usize;
    write_u32(r, read_u32(player.addr()));
    write_u32(r + 4, read_u32(club.addr()));
    write_u32(r + 0xf, u32::MAX);
    write_u16(r + 0x13, kind);
    let f: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_MEDIA_CREATE_PLAYER_OUTBURST_STORY);
    f(this, r, 0);
}

/// The player's temper score for the media story: with one of the contract grievances 0x30321, 3 for a
/// professionalism of 1, 2 below 6, 1 below 15, +1 for an ambition above 15, +2 for a temperament of 1 and +1 below 6,
/// +1 for a pressure below 10; without them, 1 for a professionalism of 1.
unsafe fn media_story_temper(player: Staff, contract: Contract) -> i8 {
    let professionalism = player.professionalism();
    if contract.is_null() || contract.grievances() & 0x30321 == 0 {
        return (professionalism == 1) as i8;
    }
    let mut score: i8 = if professionalism == 1 {
        3
    } else if professionalism < 6 {
        2
    } else if professionalism < 0xf {
        1
    } else {
        0
    };
    if player.ambition() > 0xf {
        score += 1;
    }
    if player.temperament() == 1 {
        score += 2;
    }
    if player.temperament() < 6 {
        score += 1;
    }
    if player.pressure() < 10 {
        score += 1;
    }
    score
}

/// The first story log record (`media_fields::STORY_LOG_COUNT` records of 0x17 bytes) about the player
/// (`story_log_entry::PERSON_A`) answered with response 2.
unsafe fn has_logged_response_two(this: usize, player: Staff) -> bool {
    let n = read_i32(this + media_fields::STORY_LOG_COUNT);
    (0..n.max(0) as usize).any(|k| {
        let r = read_usize(this).wrapping_add(k * 0x17);
        read_i32(r + story_log_entry::PERSON_A) == read_i32(player.addr()) && read_u8(r + story_log_entry::RESPONSE) == 2
    })
}

/// The kind of the player's media story, refined from the kind asked for (0 = criticism of form, 1, 2 = criticism of
/// contract talk, 0xd, 0x23...). `None` when the player blew up instead (`post_player_outburst_record` with 0x1e).
///
/// 1. For kinds 0, 1, 2 and 0x23, a professionalism below the temper score + 6, loyalty below 15 and temperament below
///    10: when random(morale) < random(-75) and random(professionalism) < random(score + 10), kind 2 becomes 0x1e with
///    random(3) = 0, otherwise random(4) = 0 gives 0x20 and anything else the outburst.
/// 2. A logged story about him answered with 2 adds 5 to the score.
/// 3. Loyalty below 15, temperament below 10, professionalism below score + 6 and random(professionalism) <
///    random(score + 7): kinds 0, 2 and 0xd become 0x21; the others, with a logged story answered with 2 and a manager
///    at the club for more than 45 days, 0x22.
/// 4. Otherwise kinds 0 and 2: younger than 18 + random(2), a current reputation below random(7000) and a deserved
///    squad status of 1, 2 or 3 (`exe::contract_manager_get_deserved_squad_status`) give 7; otherwise a goalkeeper over
///    34, a defender over 33 or any other player over 32 gives 4 with random(3) = 0.
/// 5. The contract grievances 0x2000 or 8 with random(professionalism) < random(10) + 5 give 0x21; otherwise kind 2
///    with a wage above 7500 and a squad status 0x10, 0x20 or 0x30 gives 9 when random(consistency) < random(25).
/// 6. A goalkeeper's kinds 0, 2 and 0xd become 0xe.
unsafe fn media_story_kind(this: usize, player: Staff, club: Club, contract: Contract, kind: u32, score: &mut i8) -> Option<u32> {
    let sheet = player.player();
    let mut kind = kind;
    let is = |kind: u32, kinds: &[u16]| kinds.contains(&(kind as u16));
    let professionalism = player.professionalism() as i32;
    if is(kind, &[0, 2, 0x23, 1]) && professionalism < *score as i32 + 6 && player.loyalty() < 0xf && player.temperament() < 10 {
        let morale = exe::utils_random(contract.morale() as i32);
        if morale < exe::utils_random(-0x4b) {
            let limit = exe::utils_random(*score as i32 + 10);
            if exe::utils_random(professionalism) < limit {
                if exe::utils_random(3) == 0 && kind as u16 == 2 {
                    kind = 0x1e;
                } else if exe::utils_random(4) == 0 {
                    kind = 0x20;
                } else {
                    covered!("sx01e_2 media story outburst");
                    post_player_outburst_record(this, player, club, 0x1e);
                    return None;
                }
            }
        }
    }
    if has_logged_response_two(this, player) {
        *score += 5;
    }
    let mut refined = None;
    if player.loyalty() < 0xf && player.temperament() < 10 && professionalism < *score as i32 + 6 {
        let limit = exe::utils_random(*score as i32 + 7);
        if exe::utils_random(professionalism) < limit {
            if is(kind, &[0, 2, 0xd]) {
                refined = Some(0x21);
            } else if has_logged_response_two(this, player) {
                let manager = club.manager();
                if !manager.is_null() && exe::date_days_since(G_CURRENT_DATE, read_u32(manager.addr() + staff::DATE_JOINED_CLUB), read_u32(manager.addr() + staff::DATE_JOINED_CLUB_LEAP_YEAR)) > 0x2d {
                    refined = Some(0x22);
                }
            }
        }
    }
    if let Some(k) = refined {
        kind = k;
    } else if is(kind, &[0, 2])
        && (player.age() as i8 as i32) < exe::utils_random(2) + 0x12
        && (sheet.current_reputation() as i32) < exe::utils_random(7000)
        && (1..=3).any(|s| exe::contract_manager_get_deserved_squad_status(read_usize(G_CONTRACT_MANAGER), player, contract) == s)
    {
        kind = 7;
    } else if is(kind, &[0, 2]) {
        let age = player.age() as i8;
        let old = if sheet.goalkeeper() == 0x14 { age > 0x22 } else if sheet.defender() == 0x14 { age > 0x21 } else { age > 0x20 };
        if old && exe::utils_random(3) == 0 {
            kind = 4;
        }
    }
    let grievances = contract.grievances();
    if (grievances & 0x2000 != 0 || grievances & 8 != 0) && exe::utils_random(professionalism) < exe::utils_random(10) + 5 {
        kind = 0x21;
    } else if kind as u16 == 2 && contract.wage() > 0x1d4c && matches!(contract.squad_status() & 0xf0, 0x30 | 0x10 | 0x20) {
        if exe::utils_random(sheet.consistency() as i32) < exe::utils_random(0x19) {
            kind = 9;
        }
    }
    if sheet.goalkeeper() == 0x14 && is(kind, &[0, 2, 0xd]) {
        kind = 0xe;
    }
    Some(kind)
}

/// The variant of the media story (slot 5), by kind: 0 random(12); 1 random(4) within a year at the club, then
/// random(2) + 4 up to 32 or random(3) + 4 older; 2 random(1); 3 random(5) for a capped player under 32 with a current
/// reputation above 6250, otherwise random(4) + 1; 4 and 0x20 random(3); 5, 7, 8 and 0x22 random(2); 9 from the rival
/// manager (without one random(4) + 2; random(his +0x5d) < random(20) gives random(2), otherwise random(6)), a 3 under
/// 21 with a current reputation above 6250 becoming random(2) + 4; 0xd and 0x21 random(4); 0xe by consistency below 6
/// (random(4) within a year, random(5) after) or not (random(2) + 2 within a year, random(3) + 2 after); 0x11
/// random(3); 0x23 3 up to 23 with a current reputation up to 6250, otherwise random(3); the others 0.
unsafe fn media_story_variant(player: Staff, kind: u32, rival: i32) -> i32 {
    let sheet = player.player();
    let age = player.age() as i8;
    let days_at_club = || exe::date_days_since(G_CURRENT_DATE, read_u32(player.addr() + staff::DATE_JOINED_CLUB), read_u32(player.addr() + staff::DATE_JOINED_CLUB_LEAP_YEAR));
    let random = |n: i32| exe::utils_random(n);
    match kind as u16 as i16 {
        0 => random(12),
        1 => {
            if days_at_club() < 0x16d {
                random(4)
            } else if age > 0x20 {
                random(3) + 4
            } else {
                random(2) + 4
            }
        }
        2 => random(1),
        3 => {
            if player.int_apps() != 0 && age < 0x20 && sheet.current_reputation() > 0x186a {
                random(5)
            } else {
                random(4) + 1
            }
        }
        4 | 0x20 => random(3),
        5 | 7 | 8 | 0x22 => random(2),
        9 => {
            let v = if rival == -1 {
                random(4) + 2
            } else {
                let temper = random(staff_at(rival as isize).temperament() as i32);
                if temper < random(0x14) { random(2) } else { random(6) }
            };
            if v == 3 && age < 0x15 && sheet.current_reputation() > 0x186a {
                random(2) + 4
            } else {
                v
            }
        }
        0xd | 0x21 => random(4),
        0xe => {
            let settled = days_at_club() > 0x16d;
            match (sheet.consistency() < 6, settled) {
                (true, false) => random(4),
                (true, true) => random(5),
                (false, false) => random(2) + 2,
                (false, true) => random(3) + 2,
            }
        }
        0x11 => random(3),
        0x23 => {
            if age > 0x17 || sheet.current_reputation() > 0x186a {
                random(3)
            } else {
                3
            }
        }
        _ => 0,
    }
}

/// The team story that may replace the player's media story when his weakness flags have 0x200200 and none of 0x180:
/// with his attribute 0xc above random(4) + the club reputation / 1250 + 8 and the club's division object in
/// `G_COMP_OBJECT_TABLE`, either his outburst (kind 0x52: loyalty below 15, professionalism below the score + 5 and
/// random(loyalty − trunc(morale × −0.1) + professionalism) < random(50)), or, with the division's stats (+8), a
/// stat 3 of the club up to 0, its stat 1 × 1.4 above its stat 10 and random(4) = 0, when he holds a selection slot
/// 0..10 (`exe::tactics_find_selection_slot`) whose position (`exe::formation_get_slot_position` of the club's tactic)
/// has 0x40, the team story 0x52 about him (`exe::media_post_team_media_story`). True when one of them was posted.
unsafe fn media_story_team_takeover(this: usize, player: Staff, club: Club, contract: Contract, flags: u32, score: i8) -> bool {
    if flags & 0x200200 != 0x200200 || flags & 0x180 != 0 {
        return false;
    }
    let threshold = exe::utils_random(4) + club.reputation() as i32 / 1250 + 8;
    if (exe::staff_get_player_attribute(player, 0xc) as i32) <= threshold {
        return false;
    }
    let division = club.division();
    if division.is_null() {
        return false;
    }
    let comp_object = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(4 * read_usize(division.addr())));
    if comp_object == 0 {
        return false;
    }
    let loyalty = player.loyalty() as i32;
    let professionalism = player.professionalism() as i32;
    if loyalty < 0xf && professionalism < score as i32 + 5 {
        let morale = ftol(contract.morale() as f64 * read_f64(rdata::F64_NEG_0_1));
        let temper = exe::utils_random(loyalty - morale + professionalism);
        if temper < exe::utils_random(0x32) {
            covered!("sx01e_2 media story team outburst");
            post_player_outburst_record(this, player, club, 0x52);
            return true;
        }
    }
    let stats = read_usize(comp_object + 8);
    if stats == 0 || exe::comp_stats_get_club_stat(stats, club, 3) > read_f32(rdata::F32_0) as f64 {
        return false;
    }
    let lost = exe::comp_stats_get_club_stat(stats, club, 10);
    if exe::comp_stats_get_club_stat(stats, club, 1) * read_f64(rdata::F64_1_4) <= lost || exe::utils_random(4) != 0 {
        return false;
    }
    let slot = exe::tactics_find_selection_slot(read_usize(G_TACTICS), club, player, 0) as i8;
    if !(0..0xb).contains(&slot) {
        return false;
    }
    let tactic = exe::tactics_get_club_tactic(read_usize(G_TACTICS), club, 0);
    if exe::formation_get_slot_position(tactic, slot as u32, 0) & 0x40 == 0 {
        return false;
    }
    covered!("sx01e_2 media story becomes a team story");
    exe::media_post_team_media_story(this, club, 0x52, player.addr() as u32, 0usize, 0, -1);
    true
}

/// The slots from 0x1c: with a match (and a kind other than 0x13) 1, the match record and the shoot-out
/// (`NewsSlots::put_match`), his strength flags, his position mask and the pundit; otherwise 0, the strength flags,
/// the position mask, the pundit, his natural position mask (`exe::staff_get_natural_position_mask`), the club's
/// injured squad count (the original asks `exe::injury_get_player_injury_status` about the player himself once per
/// filled squad slot, a defect: the count is 0 or the number of filled slots), whether the computer manager's man
/// handling is below 10, and the other person with his names (-1 × 4 without him).
unsafe fn media_story_tail_slots(s: &mut NewsSlots, record: &mut [u8; 0x20], player: Staff, club: Club, kind: u32, fixture: usize, person: Staff, pundit: u32) {
    s.k = 0x1c;
    if fixture != 0 && kind as u16 != 0x13 {
        covered!("sx01e_2 media story after a match");
        s.put(1);
        s.put_match(record, Fixture(fixture), club);
        s.put(exe::staff_get_player_strength_flags(player, club));
        s.put(exe::formation_get_position_mask(player.player()) as i16 as i32 as u32);
        s.put(pundit);
        return;
    }
    s.put(0);
    s.put(exe::staff_get_player_strength_flags(player, club));
    s.put(exe::formation_get_position_mask(player.player()) as i16 as i32 as u32);
    s.put(pundit);
    s.put(exe::staff_get_natural_position_mask(player));
    let mut injured: i8 = 0;
    for k in 0..50 {
        if club.squad(k) != 0 && exe::injury_get_player_injury_status(read_usize(G_INJURY), player) == 2 {
            injured += 1;
        }
    }
    s.put(injured as i32 as u32);
    let manager = club.manager();
    let careless = media_is_human_club(club.addr(), 0, 1) == 0 && !manager.is_null() && !manager.non_player().is_null() && (manager.non_player().man_handling() as i8) < 10;
    s.put(careless as u32);
    if person.is_null() {
        for _ in 0..4 {
            s.put(u32::MAX);
        }
    } else {
        s.put_person(person);
    }
}

/// Creates a media story about a player (news item 0xbd0), refining the kind asked for from his form, contract and
/// character, and the computer manager's response.
///
/// Nothing without the player, his `.player()` sheet with a `.current_reputation()` of at least 3500, his
/// `.nation()`, the club, its `.nation()`, `.stadium()` with a city and `.manager()`. The contract is his with his
/// club job's club or, failing that, with the club (`exe::contract_manager_get_contract_for_club`); nothing without
/// one. Without a club job the original first reads an uninitialised stack slot as the contract (a defect; here
/// none). The rival manager comes from `exe::media_pick_rival_manager_id`.
///
/// The kind (`media_story_kind`, which may post the outburst instead), then nothing when the story is already logged
/// (`exe::media_has_logged_story` with 0xbd0). The slots: 0..3 the player and his names (the common name -1 when he
/// has none), 4 the club id, 5 the variant (`media_story_variant`), 6 the outlet
/// (`exe::media_pick_outlet_type_for_person`), 7 his nation id, 8 the stadium's city id, 9 the kind, 10 the computer
/// manager's response (`exe::media_pick_ai_manager_response`, for a club that is not human; otherwise 0), 11..14 the
/// rival manager and his names (-1 × 4 without one), 15 the contract's `.unhappy_flag()` up to 1, 16 the high nibble
/// of the `.squad_status()`, 17 his age, 18 his weakness flags (`staff_get_player_weakness_flags`) without 8 when his
/// stat 6 (`exe::playerstats_get_stat`) is below 20. Then the team takeover (`media_story_team_takeover`) may post
/// another story and stop. 0x17..0x1a the club's manager and his names, 0x1b the rival manager's club id (-1), and the
/// tail (`media_story_tail_slots`) with the pundit (`exe::punditlist_pick_random_pundit` with the club's nation).
///
/// It goes to every human club following the player (`exe::transfer_manager_collect_player_news_recipients` with
/// 0x2da, up to 30) and is logged (`log_story` with the response). For a kind other than 0x13 with random(pressure)
/// < random(5), or with a morale below −70 − professionalism, the contract gets the event flag 0x40000
/// (`exe::staff_contract_apply_event_flags`); the morale moves by random(pressure − 30), within ±100. Kinds 0x1e and
/// 0x20 let the squad react (`exe::media_apply_squad_reaction_to_player_story`); the others, when random(150) is below
/// the chairman's interference and the contract has grievances, make the chairman dislike him
/// (`exe::staff_add_disliked_staff`). Last, for a club that is not human, the computer manager's response is applied
/// (`exe::media_apply_player_story_response`).
///
/// Called by the post-match events (0x6b7012, 0x6b75fd, 0x6b796e), the daily loop (0x5cff53, 0x5d0a64...), 0x4194e0
/// and 0x7581b2; behind the post-match gate (`POS_PROLOGUES`, match_day.rs).
/// Original: `Media::create_player_media_story` (0x72c1f0, media.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn create_player_media_story(this: usize, player: usize, club: usize, kind: u32, fixture: usize, person: usize) {
    let mut record = [0u8; 0x20]; // the 0x1f-byte match record
    exe::date_date_54a110(record.as_mut_ptr() as usize);
    let (player, club, person) = (Staff(player), Club(club), Staff(person));
    if player.is_null() || player.player().is_null() || player.player().current_reputation() < 0xdac || player.nation().is_null() {
        return;
    }
    if club.is_null() || club.nation().is_null() || club.stadium().is_null() || read_usize(club.stadium().addr() + stadium::CITY) == 0 || club.manager().is_null() {
        return;
    }
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let mut s = NewsSlots { n: local.as_mut_ptr() as usize, k: 0 };
    exe::new_newsitem(s.n, 0xbd0, 0);
    set_person_name_slots_or_none(s.n, 0, player);
    s.k = 4;
    s.put(read_u32(club.addr()));
    let contracts = read_usize(G_CONTRACT_MANAGER);
    let mut contract = Contract(0);
    if !player.club_job().is_null() {
        contract = Contract(exe::contract_manager_get_contract_for_club(contracts, player, player.club_job()));
    }
    if contract.is_null() {
        contract = Contract(exe::contract_manager_get_contract_for_club(contracts, player, club));
        if contract.is_null() {
            return;
        }
    }
    let manager = club.manager();
    let rival = exe::media_pick_rival_manager_id(this, club);
    let mut score = media_story_temper(player, contract);
    let Some(kind) = media_story_kind(this, player, club, contract, kind, &mut score) else {
        return;
    };
    if exe::media_has_logged_story(this, player, club, 0xbd0, kind, core::ptr::null()) != 0 {
        return;
    }
    covered!("sx01e_2 media story");
    s.put(media_story_variant(player, kind, rival) as u32);
    let outlet = exe::media_pick_outlet_type_for_person(player);
    s.put(outlet);
    s.put(read_u32(player.nation().addr()));
    s.put(read_u32(read_usize(club.stadium().addr() + stadium::CITY)));
    s.put(kind as u16 as i16 as i32 as u32);
    let response = if media_is_human_club(club.addr(), 0, 1) == 0 { exe::media_pick_ai_manager_response(this, manager, player, kind) } else { 0 };
    s.put(response);
    if rival == -1 {
        for _ in 0..4 {
            s.put(u32::MAX);
        }
    } else {
        let r = staff_at(rival as isize);
        s.put(rival as u32);
        for name in [r.first_name(), r.second_name(), r.common_name()] {
            s.put(read_u32(name + names::ID));
        }
    }
    s.put((contract.unhappy_flag() <= 1) as u32);
    s.put((contract.squad_status() >> 4) as u32);
    s.put(player.age() as i8 as i32 as u32);
    let mut flags = staff_get_player_weakness_flags(player.addr(), club.addr());
    let stat = exe::playerstats_get_stat(read_usize(G_PLAYER_STATS), player.player(), 6, 1, 6.5);
    if flags & 8 != 0 && stat < read_f32(rdata::F32_20) as f64 {
        flags ^= 8;
    }
    s.put(flags);
    if media_story_team_takeover(this, player, club, contract, flags, score) {
        return;
    }
    s.k = 0x17;
    s.put_person(manager);
    let rival_club = if rival > -1 { staff_at(rival as isize).club_job() } else { Club(0) };
    s.put(if rival_club.is_null() { u32::MAX } else { read_u32(rival_club.addr()) });
    let nation = club.nation();
    let pundit = exe::punditlist_pick_random_pundit(read_usize(G_PUNDIT_LIST), club, outlet, if nation.is_null() { -1 } else { read_i32(nation.addr()) }) as u32;
    media_story_tail_slots(&mut s, &mut record, player, club, kind, fixture, person, pundit);
    let mut clubs = [0usize; 30];
    let count = if read_usize(G_TRANSFER_MANAGER) != 0 { player_news_recipients(player, 0x2da, clubs.as_mut_ptr(), 0x1e) as i8 } else { 0 };
    for &c in clubs.iter().take(count.max(0) as usize) {
        exe::news_send_to_club_manager(read_usize(G_NEWS), s.n, c);
    }
    log_story(this, player.addr(), club.addr(), 0xbd0, kind, response);
    let professionalism = player.professionalism() as i32;
    let pressured = kind as u16 != 0x13 && exe::utils_random(player.pressure() as i32) < exe::utils_random(5);
    if pressured || (contract.morale() as i32) < -0x46 - professionalism {
        exe::staff_contract_apply_event_flags(contract, 0x40000, 0);
    }
    let change = exe::utils_random(player.pressure() as i32 - 0x1e) as i8;
    let morale = change as i32 + contract.morale() as i32;
    contract.set_morale(if morale > 100 { 100 } else if morale < -100 { -100 } else { change.wrapping_add(contract.morale()) });
    if kind as u16 == 0x1e || kind as u16 == 0x20 {
        exe::media_apply_squad_reaction_to_player_story(this, kind, player, club, core::ptr::null_mut(), u32::MAX);
    } else {
        let chairman = club.chairman();
        if !chairman.is_null() && !chairman.non_player().is_null() && (chairman.non_player().interference() as i32) > exe::utils_random(0x96) && contract.grievances() != 0 {
            exe::staff_add_disliked_staff(chairman, player, 1);
        }
    }
    if media_is_human_club(club.addr(), 0, 1) == 0 {
        exe::media_apply_player_story_response(this, player, club, kind, response);
    }
}

/// Comparer for `create_player_media_story`: it draws, posts news and changes the contract, so it cannot run twice;
/// only the original runs (in the comparison mode the post-match gate of 0x72c1f0 is installed first and keeps the
/// address, see the item).
pub(crate) unsafe extern "thiscall" fn compare_create_player_media_story(this: usize, player: usize, club: usize, kind: u32, fixture: usize, person: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, usize, usize)>(IDX_CREATE_PLAYER_MEDIA_STORY)(this, player, club, kind, fixture, person)
}
