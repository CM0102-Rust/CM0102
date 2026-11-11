// ---------- batch %%LEVA%%: the default wage of a person at a club (staff_contracts.cpp) ----------

/// The `.rdata` constants of `staff_contract_default_staff_contract` that `rdata_consts.rs` does not name yet (value in
/// cm0102.exe 3.9.68; still read from the exe).
#[allow(dead_code)]
pub(crate) mod default_wage_rdata {
    pub(crate) const F64_20000: usize = 0x968888; // f64 = 20000
    pub(crate) const F64_250: usize = 0x968898; // f64 = 250
    pub(crate) const F64_25000: usize = 0x9688d0; // f64 = 25000
    pub(crate) const F64_1_125: usize = 0x96a5a8; // f64 = 1.125
    pub(crate) const F32_0_2000000030: usize = 0x96b028; // f32 = 0.20000000298023224
    pub(crate) const F32_30: usize = 0x96d880; // f32 = 30
    pub(crate) const F64_350: usize = 0x96f870; // f64 = 350
    pub(crate) const F32_75000: usize = 0x96fe94; // f32 = 75000
    pub(crate) const F32_105000: usize = 0x96feac; // f32 = 105000
    pub(crate) const F32_85000: usize = 0x96feb0; // f32 = 85000
    pub(crate) const F64_1500: usize = 0x96feb8; // f64 = 1500
    pub(crate) const F64_3250: usize = 0x96fec0; // f64 = 3250
    pub(crate) const F64_2750: usize = 0x96fec8; // f64 = 2750
    pub(crate) const F64_0_00017: usize = 0x96fed0; // f64 = 0.00017
    pub(crate) const F32_550: usize = 0x96fed8; // f32 = 550
    pub(crate) const F32_1_375: usize = 0x96fedc; // f32 = 1.375
    pub(crate) const F32_0_0125: usize = 0x96fee0; // f32 = 0.012500000186264515
    pub(crate) const F32_0_0004: usize = 0x96fee4; // f32 = 0.00039999998989515007
    pub(crate) const F32_175: usize = 0x96fee8; // f32 = 175
    pub(crate) const F32_11000: usize = 0x96feec; // f32 = 11000
    pub(crate) const F32_50000: usize = 0x96ff04; // f32 = 50000
    pub(crate) const F32_55000: usize = 0x96ff08; // f32 = 55000
    pub(crate) const F64_0_00012: usize = 0x96ff10; // f64 = 0.00012
    pub(crate) const F64_1ENEG_09: usize = 0x96ff18; // f64 = 1e-09
    pub(crate) const F32_275: usize = 0x96ff20; // f32 = 275
    pub(crate) const F64_5ENEG_09: usize = 0x96ff28; // f64 = 5e-09
    pub(crate) const F32_750: usize = 0x96ff30; // f32 = 750
    pub(crate) const F32_25000: usize = 0x96ff34; // f32 = 25000
    pub(crate) const F64_2_5ENEG_08: usize = 0x96ff38; // f64 = 2.5e-08
    pub(crate) const F64_7250: usize = 0x96ff40; // f64 = 7250
    pub(crate) const F32_44000: usize = 0x96ff48; // f32 = 44000
    pub(crate) const F32_60000: usize = 0x96ff4c; // f32 = 60000
    pub(crate) const F32_67750: usize = 0x96ff50; // f32 = 67750
    pub(crate) const F32_27500: usize = 0x96ff54; // f32 = 27500
    pub(crate) const F32_22000: usize = 0x96ff58; // f32 = 22000
    pub(crate) const F32_37500: usize = 0x96ff5c; // f32 = 37500
    pub(crate) const F32_70000: usize = 0x96ff60; // f32 = 70000
    pub(crate) const F32_9350: usize = 0x96ff64; // f32 = 9350
    pub(crate) const F32_82500: usize = 0x96ff68; // f32 = 82500
    pub(crate) const F32_92500: usize = 0x96ff6c; // f32 = 92500
    /// "STAFF_CONTRACT::default_staff_contract() - No player datafor player: '%s %s' - id %ld"
    pub(crate) const NO_PLAYER_DATA_FORMAT: usize = 0xa83968;
    /// The game's text buffer (2000 bytes) that the message above is formatted into.
    pub(crate) const TEXT_BUFFER: usize = 0xde1f64;
}

/// The default wage (a float, per week) of `person` at `club`: what the person is worth as a wage there, before the
/// will and the negotiation. Called by the desired contract (`set_to_desired_contract`, three times), the will to join
/// (`staff_interest_in_joining_club`, the wage gate), the staff's will (0x844780), the bids (0x842e10, 0x83f3a0), the
/// wage bill (`update_wage_bill`) and 0x85e960.
///
/// First the person's contract (`exe::contract_manager_get_contract`, kept in `G_CONTRACT_LOOKUP_RESULT`) and its club
/// (`cur`, none for a contract of type 5). The level: with a club, reputation / 500 + 1 (a byte), and with a current
/// club and a club of professional status 2, minus one within 1..4; without a club, the world reputation of the player
/// (or non-player) sheet / 500, at least 1. The job: the club job (`job_for_club`) when there is a current club, else
/// the national job, else the club job, else 11 for a player, else the national job, else 9.
///
/// Staff jobs (5..10): the level rises to the current club's (reputation / 500 + 1) or, for a non-player with a
/// national job other than 11, to the national team's; the base is the non-player current reputation³ × k1 or, without
/// a non-player sheet, (id / staff count × 4 + 1) × k2 × 0.0001; the wage = level × base × 0.25, then per job: 5 at
/// least 750 and above 25000 → 150000; 6 and 7 at least 500; 8 within 275..5000; 9 within 250..1000; 10 within 200..1000.
/// For jobs 6, 7 and 8 with a current club other than `club`, 150000 (the marker of "not available") when the current
/// club is more than three levels above (job 6: also unless the asked job is 11 or 6; then below 2500 × 1.35), and for
/// 6 and 7 without that, above 25000 → 150000 unless the asked job is 11 or 6. Job 14 and the others: 1000.
///
/// Players (jobs 11, 12, 13, 15): without a player sheet the error message goes to the text buffer and the wage is a
/// draw `rand() × 4 / 32768 × 550`. With a sheet: an unknown squad status (−1) is worked out from the current
/// reputation against the club's reputation and the age (1 star, 2 important, 4 squad, 5 or 6 by the potential); the
/// base by `kind` (3: 85000; 1: 92500 with loyalty < 15 and pressure > 15, else 75000; 2: 75000 from 23 to 31 years,
/// else 82500), for an old (36+) coach asked as job 13 or 15 10000, else by the squad status (5 or 6: 9350 below 2500
/// of reputation, 11000 below 6250) and the best position (attacker, then midfielders, goalkeeper, defenders, with the
/// reputation, age and loyalty bands of the table); a young (19..31) player not tied to the club or its nation whose
/// world reputation passes (loyalty + 230) × 25 − ambition × 10 rises to 75000 / 92500 / 105000; with the FIFA rules,
/// above 11000 at a club of a major nation × 1.15 / 1.125 / 1.1 (current reputation above 6750, by loyalty) or × 1.025
/// (above 4250). Then the reputation sum u = (current + home + world + 4 × max + 50 × (ability + potential)) / 9 +
/// (id / staff count × 8 − 4), at least 1500 or × 1.05 for a foreigner (another region than the club's and the current
/// club's), at least 1; the wage = base × u² × 1e-8, at least 175; × 0.75 or × 0.5 by the state of development of the
/// nation; × the world reputation against the club's level (0.85..1.15); the level caps (2500, 2750 per level below 10
/// and 15; from 15 on, 10% above 5250 / 7500 per level in England, Spain and Italy and 3250 elsewhere); 20% above the
/// position cap (150000 attacker, 125000 midfielders, 100000 defender, 80000; × 1.1 with the FIFA rules at a major
/// nation); and, without a club job and unless he is an old (33+) international with little drive, a long-time member
/// (id % 15 ≠ 0) at a club of reputation up to 4500, × (1.375 − days at the club (at most 250, + 60 in season, 20% above
/// 30) × 0.0125), at least 0.2.
///
/// With a club: for players, the club's reputation^6 (× 0.00017 / 0.00012 / 0.0001) gives a floor by squad status
/// (the wage goes half-way up to 3 × reputation × it × 0.6..1.5 for stars, ..., × 0.02 for status 6), and a contract
/// with grievance 0x10 multiplies by 0.67 / 0.85 / 2.5 / 3.5 / 7.5 by status; then the club's nation (when the person
/// works there or is from there): Japan at least level² × 10 and 25% above 7500; England, Italy, Spain nothing; the
/// United States × 0.75 and caps by the league (MLS 250) and by age and origin (1250 / 2750 / 3250); South Korea,
/// Scotland, Germany, France, Holland, Sweden, Denmark, Norway, Belgium, Portugal, Croatia, Poland, Brazil, Argentina
/// × a factor by club reputation / 50 against the league standard × 10, with their own caps; Wales, Finland, Ireland,
/// Northern Ireland, Australia only caps; the others × 0.3 and 25% above 15000. For players: at most the wage of the
/// current club's contract with squad status 7 (when the current club is another); with `flag` or without a current
/// club (and current reputation below 2250) at most the club's maximum wage (`exe::finance_get_max_wage`; without
/// finances the wage is returned as it is); without `flag` and with a current club, an ambitious (> 10), not loyal
/// (< 6), reputed (> 6750) midfielder or attacker (a 20 in attacker, attacking midfielder or midfielder) over 20 gets
/// × 1.1..1.2 when the wage × 1.25 is below the maximum. At last the minimum: at a club of professional status 1, 75
/// under 20 years, else 100; elsewhere 75 at clubs of reputation above 2250, else 50.
///
/// Defects of the original: the third test of `kind` compares the byte with 0x14244 (never true: the 6500 floor never
/// applies); without a club, a player with a nation (or of level 15 or more) reads the club through a null pointer (no
/// caller passes it).
/// Original: `STAFF_CONTRACT::default_staff_contract` (0x864550, staff_contracts.cpp, stdcall, ret 0x18).
pub(crate) unsafe extern "stdcall" fn staff_contract_default_staff_contract(person: usize, club: usize, flag: u32, status: u32, job: u32, kind: u32) -> f32 {
    use default_wage_rdata::*;
    let f = |a: usize| read_f32(a) as f64;
    let d = |a: usize| read_f64(a);
    let p = Staff(person);
    let c = Club(club);
    let contract = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person);
    write_usize(G_CONTRACT_LOOKUP_RESULT, contract);
    let cur: usize = if contract == 0 || read_u8(contract + contract::CONTRACT_TYPE) & 0x3f == 5 {
        0
    } else {
        read_usize(G_CLUB_TABLE).wrapping_add(read_usize(contract + contract::CLUB).wrapping_mul(club::SIZE))
    };
    let id = read_i32(person);
    let age = p.age() as i8;
    let sheet = read_usize(person + staff::PLAYER);
    let np = read_usize(person + staff::NON_PLAYER);
    let frac = (id as f64 / read_i32(G_STAFF_COUNT) as f64) as f32;
    let rep_of = |cl: usize| read_i16(cl + club::REPUTATION) as i32;
    let mut status = status;
    // the level
    let mut level: i32;
    if club != 0 {
        level = ((rep_of(club) / 500) as u8).wrapping_add(1) as i8 as i32;
        if cur != 0 && c.professional_status() == 2 {
            level -= 1;
            if level < 1 {
                level = 1;
            } else if level > 4 {
                level = 4;
            }
        }
    } else if sheet != 0 || np != 0 {
        let q = if sheet != 0 { read_i16(sheet + player_sheet::WORLD_REPUTATION) as i32 / 500 } else { read_i16(np + non_player::WORLD_REPUTATION) as i32 / 500 };
        level = (q.max(1) as u8) as i8 as i32;
    } else {
        level = 1;
    }
    // the job
    let national = read_usize(person + staff::NATIONAL_JOB);
    let job_nation = p.job_for_nation() as i8;
    let job_club = p.job_for_club() as i8;
    let job_sel: i8 = if cur != 0 && job_club != 0 {
        job_club
    } else if national != 0 && job_nation != 0 {
        job_nation
    } else if job_club != 0 {
        job_club
    } else if sheet != 0 {
        0xb
    } else if job_nation != 0 {
        job_nation
    } else {
        9
    };
    let asked_job = job as u8 as i8;
    // staff jobs: the level rises to the current club's or the national team's
    let raise = |level: &mut i32| {
        if cur != 0 {
            let lv = rep_of(cur) / 500 + 1;
            if lv > *level {
                *level = lv;
                return;
            }
        }
        if np != 0 && national != 0 && job_nation != 0xb {
            let lv = rep_of(national) / 500 + 1;
            if lv > *level {
                *level = lv;
            }
        }
    };
    let staff_wage = |level: i32, k1: usize, k2: usize| -> f32 {
        let base = if np != 0 {
            let t = read_i16(np + non_player::CURRENT_REPUTATION) as f64;
            t * (t * t) * d(k1)
        } else {
            (frac as f64 * f(rdata::F32_4) + f(rdata::F32_1)) * f(k2) * d(rdata::F64_0_0001)
        };
        (level as f64 * base * d(rdata::F64_0_25)) as f32
    };
    let unavailable = 150000.0f32;
    let mut value: f32;
    match job_sel as i32 - 5 {
        0 => {
            covered!("mkd_2 staff wage");
            raise(&mut level);
            value = staff_wage(level, F64_2_5ENEG_08, F32_25000);
            if (value as f64) < f(F32_750) {
                value = 750.0;
            } else if value as f64 > f(F32_25000) {
                value = unavailable;
            }
        }
        1 | 2 => {
            covered!("mkd_2 staff wage");
            raise(&mut level);
            value = staff_wage(level, rdata::F64_1ENEG_08, rdata::F32_10000);
            if (value as f64) < f(rdata::F32_500) {
                value = 500.0;
            }
            if cur != 0 && cur != club {
                let above = rep_of(cur) / 500 + 4 > level;
                if job_sel == 6 {
                    if (asked_job == 0xb || asked_job == 6) && !above {
                        if (value as f64) < f(rdata::F32_2500) {
                            value = (value as f64 * d(rdata::F64_1_35)) as f32;
                        }
                    } else {
                        value = unavailable;
                    }
                } else if above {
                    value = unavailable;
                } else if asked_job != 0xb && asked_job != 6 && value as f64 > f(F32_25000) {
                    value = unavailable;
                }
            } else if asked_job != 0xb && asked_job != 6 && value as f64 > f(F32_25000) {
                value = unavailable;
            }
        }
        3 => {
            covered!("mkd_2 staff wage");
            raise(&mut level);
            value = staff_wage(level, F64_5ENEG_09, rdata::F32_5000);
            if (value as f64) < f(F32_275) {
                value = 275.0;
            } else if value as f64 > f(rdata::F32_5000) {
                value = 5000.0;
            }
            if cur != 0 && cur != club && asked_job != 6 && asked_job != 0xb && rep_of(cur) / 500 + 4 > level {
                value = unavailable;
            }
        }
        4 | 5 => {
            covered!("mkd_2 staff wage");
            raise(&mut level);
            value = staff_wage(level, F64_1ENEG_09, rdata::F32_1000);
            let floor = if job_sel == 9 { 250.0f32 } else { 200.0 };
            if (value as f64) < f(if job_sel == 9 { rdata::F32_250 } else { rdata::F32_200 }) {
                value = floor;
            } else if value as f64 > f(rdata::F32_1000) {
                value = 1000.0;
            }
        }
        6 | 7 | 8 | 10 => {
            if sheet == 0 {
                // the error message, then a draw
                let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
                format(TEXT_BUFFER, NO_PLAYER_DATA_FORMAT, read_u32(person + staff::FIRST_NAME), read_u32(person + staff::SECOND_NAME), id);
                let r = exe::crt_rand();
                value = ((r as i64 * 4 / 0x8000) as i32 as f64 * f(F32_550)) as f32;
            } else {
                covered!("mkd_2 player wage");
                value = player_default_wage(person, club, cur, &mut status, asked_job, kind as u8 as i8, level, frac);
            }
        }
        _ => value = 1000.0,
    }
    if club == 0 {
        return value;
    }
    let rep = rep_of(club);
    if sheet != 0 {
        // the floor by the club's reputation and the squad status
        let x = rep as f64 * if c.professional_status() == 1 {
            d(F64_0_00017)
        } else if rep > 2250 {
            d(F64_0_00012)
        } else {
            d(rdata::F64_0_0001)
        };
        let mut r = game_pow(x, d(rdata::F64_6));
        if r > d(rdata::F64_1) {
            r = 1.0;
        }
        let three = (rep * 3) as f64;
        let half_way = |value: &mut f32, y: f64| {
            if (*value as f64) < y {
                *value = ((*value as f64 + y) * d(rdata::F64_0_5)) as f32;
            }
        };
        let major = |age: i8| age >= 24 && club_is_in_major_nation(club) != 0;
        let s = status as u8 as i8 as i32 - 1;
        match s {
            0 => {
                let k = if age < 20 { d(rdata::F64_0_6) } else if major(age) { d(rdata::F64_1_5) } else { d(rdata::F64_1_25) };
                half_way(&mut value, three * r * k); // mkd2: the floor of the stars
            }
            1 => {
                let y = if age < 20 { three * r * d(rdata::F64_0_3) } else if major(age) { three * r } else { three * r * d(rdata::F64_0_6) };
                half_way(&mut value, y);
            }
            2 => {
                if age < 20 {
                    half_way(&mut value, three * r * d(rdata::F64_0_25));
                }
                let k = if major(age) { d(rdata::F64_0_6) } else { d(rdata::F64_0_3) };
                half_way(&mut value, three * r * k);
            }
            3 => {
                let k = if age < 20 { d(rdata::F64_0_1) } else if major(age) { d(rdata::F64_0_3) } else { d(rdata::F64_0_15) };
                half_way(&mut value, three * r * k);
            }
            4 => half_way(&mut value, three * r * d(rdata::F64_0_05)),
            5 => half_way(&mut value, three * r * d(rdata::F64_0_02)),
            _ => {}
        }
        let ct = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person);
        if ct != 0 && read_u8(ct + contract::GRIEVANCES) & 0x10 != 0 && (0..=5).contains(&s) {
            covered!("mkd_2 grievance");
            let k = [rdata::F64_0_67, rdata::F64_0_67, rdata::F64_0_85, rdata::F64_2_5, rdata::F64_3_5, rdata::F64_7_5][s as usize];
            value = (value as f64 * d(k)) as f32;
        }
    }
    // the club's nation, when the person works there or is from there
    let cn = read_usize(club + club::NATION);
    if cn != 0 {
        let e = read_usize(person + staff::CLUB_JOB);
        let sn = read_usize(person + staff::NATION);
        let go = if e == 0 {
            sn == cn
        } else {
            read_usize(e + club::NATION) == cn && (sn == cn || p.loyalty() > 3 || p.ambition() < 15)
        };
        if go {
            value = default_wage_by_nation(person, club, cn, sn, sheet, level, value);
        }
    }
    if sheet != 0 {
        if cur != 0 && club != cur {
            let ct = exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), person, cur);
            if ct != 0 && read_u8(ct + contract::SQUAD_STATUS) & 0xf0 == 0x70 {
                let w = read_i32(ct + contract::WAGE);
                if value as f64 > w as f64 {
                    value = w as f32;
                }
            }
        }
        let max_wage = || {
            let fin = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
            if fin == 0 { None } else { Some(exe::finance_get_max_wage(fin, person, status)) }
        };
        if flag != 0 || cur == 0 {
            if flag != 0 || read_i16(sheet + player_sheet::CURRENT_REPUTATION) < 2250 {
                let Some(mw) = max_wage() else { return value };
                if (mw as f64) < value as f64 {
                    value = mw as f32; // mkd2: the club's maximum wage
                }
            }
        } else {
            let at = |k: usize| read_i8(sheet + k);
            if p.ambition() > 10
                && p.loyalty() < 6
                && read_i16(sheet + player_sheet::CURRENT_REPUTATION) > 6750
                && (at(player_sheet::ATTACKER) == 20 || at(player_sheet::ATTACKING_MIDFIELDER) == 20 || at(player_sheet::MIDFIELDER) == 20)
                && age > 20
            {
                let Some(mw) = max_wage() else { return value };
                let loyal1 = p.loyalty() == 1;
                let k = if at(player_sheet::ATTACKING_MIDFIELDER) == 20 {
                    Some(if loyal1 { rdata::F64_1_15 } else { rdata::F64_1_1 })
                } else if value as f64 * d(rdata::F64_1_25) < mw as f64 {
                    if at(player_sheet::MIDFIELDER) == 20 {
                        Some(if loyal1 { rdata::F64_1_1 } else { rdata::F64_1_05 })
                    } else {
                        Some(if loyal1 { rdata::F64_1_2 } else { rdata::F64_1_15 })
                    }
                } else {
                    None
                };
                if let Some(k) = k {
                    covered!("mkd_2 ambitious star");
                    value = (value as f64 * d(k)) as f32;
                }
            }
        }
    }
    // the minimum
    if c.professional_status() == 1 {
        if age < 20 && (value as f64) < f(rdata::F32_75) {
            return 75.0;
        }
        if (value as f64) < f(rdata::F32_100) {
            value = 100.0;
        }
        value
    } else {
        if rep > 2250 && (value as f64) < f(rdata::F32_75) {
            return 75.0;
        }
        if (value as f64) < f(rdata::F32_50) {
            return 50.0;
        }
        value
    }
}

/// The player part of `staff_contract_default_staff_contract` (a player sheet, jobs 11, 12, 13, 15): the squad status
/// when unknown (written into the low byte of `status`), the base by `kind`, the job and the position, and the
/// adjustments up to the decline by days at the club (see the header of the routine).
unsafe fn player_default_wage(person: usize, club: usize, cur: usize, status: &mut u32, asked_job: i8, kind: i8, level: i32, frac: f32) -> f32 {
    use default_wage_rdata::*;
    let f = |a: usize| read_f32(a) as f64;
    let d = |a: usize| read_f64(a);
    let p = Staff(person);
    let c = Club(club);
    let age = p.age() as i8;
    let sheet = read_usize(person + staff::PLAYER);
    let at = |k: usize| read_i8(sheet + k);
    let w16 = |k: usize| read_i16(sheet + k);
    let sb = w16(player_sheet::CURRENT_REPUTATION);
    let sd = w16(player_sheet::WORLD_REPUTATION);
    let s9 = w16(player_sheet::HOME_REPUTATION);
    let loyalty = p.loyalty();
    // the squad status when unknown
    if *status as u8 as i8 == -1 && club != 0 {
        let rep = c.reputation();
        let new = if sb > rep && age >= 24 && age < 34 {
            1
        } else {
            let x = rep as f64 * d(rdata::F64_0_75);
            if sb as f64 > x && age >= 21 {
                2
            } else if (sb as f64 > rep as f64 * d(rdata::F64_0_5) && age >= 17) || age > 24 {
                4
            } else if (w16(player_sheet::POTENTIAL_ABILITY) as i32 * 50) as f64 > x {
                5
            } else {
                6
            }
        };
        *status = (*status & !0xff) | new;
    }
    let st_s = *status as u8 as i8;
    let v75 = f(F32_75000);
    let v92 = f(F32_92500);
    let v105 = f(F32_105000);
    // the base
    let base = if kind == 3 {
        F32_85000
    } else if kind == 1 {
        if loyalty < 15 && p.pressure() > 15 { F32_92500 } else { F32_75000 }
    } else if kind == 2 {
        if age > 22 && age < 32 { F32_75000 } else { F32_82500 }
    } else if (asked_job == 13 || asked_job == 15) && age > 35 {
        rdata::F32_10000
    } else if (st_s == 6 || st_s == 5) && sb < 2500 {
        F32_9350
    } else if (st_s == 6 || st_s == 5) && sb < 6250 {
        F32_11000
    } else if at(player_sheet::ATTACKER) >= 15 {
        if sb > 6750 && age < 31 {
            if loyalty < 15 { F32_82500 } else { F32_70000 }
        } else {
            F32_55000
        }
    } else if at(player_sheet::DEFENSIVE_MIDFIELDER) >= 15 || at(player_sheet::ATTACKING_MIDFIELDER) >= 15 || at(player_sheet::MIDFIELDER) >= 15 {
        if sb > 7250 && age < 31 {
            if loyalty < 14 { F32_67750 } else { F32_60000 }
        } else {
            F32_44000
        }
    } else if at(player_sheet::GOALKEEPER) == 20 {
        if sd > 7250 && loyalty < 3 && age < 34 { F32_37500 } else { F32_22000 }
    } else if at(player_sheet::SWEEPER) < 15 && at(player_sheet::DEFENDER) < 15 {
        F32_27500
    } else if sb > 7500 && loyalty < 6 && age < 33 {
        F32_50000
    } else {
        F32_27500
    };
    let mut st = f(base); // the x87 top of the stack (not always rounded to a float)
    let mut value = st as f32;
    // a young player not tied to the club or its nation, with a world reputation above his loyalty and ambition
    let mut free = true;
    if club != 0 {
        let cn = read_usize(club + club::NATION);
        if cn != 0 && (c.reputation() < 5750 || read_i8(cn + nation::LEAGUE_STANDARD) < 12) {
            let e = read_usize(person + staff::CLUB_JOB);
            if e == club || (e != 0 && read_usize(e + club::NATION) == cn) {
                free = false;
            }
        }
    }
    if age > 18 && age < 32 && free && sd as i32 > (loyalty as i32 + 230) * 25 - p.ambition() as i32 * 10 {
        let new = if at(player_sheet::GOALKEEPER) == 20 {
            if st < v75 { Some(v75) } else { None }
        } else if st == v92 {
            Some(v105)
        } else if st == v75 {
            Some(if sd > 7250 { v105 } else { v92 })
        } else if sd > 7250 {
            if st < v92 { Some(v92) } else { None }
        } else if st < v75 {
            Some(v75)
        } else {
            None
        };
        if let Some(n) = new {
            covered!("mkd_2 young star");
            st = n;
            value = n as f32;
        }
    }
    // the FIFA rules at a club of a major nation
    if read_u8(G_FIFA_TRANSFER_RULES_IN_FORCE) == 1 && st > f(F32_11000) && club != 0 {
        st = value as f64;
        if club_is_in_major_nation(club) != 0 {
            if sb > 6750 {
                st *= if loyalty == 1 { d(rdata::F64_1_15) } else if loyalty < 6 { d(F64_1_125) } else { d(rdata::F64_1_1) };
            } else if sb > 4250 {
                st *= d(rdata::F64_1_025);
            }
        }
    }
    // the reputation sum
    let mut m = if s9 > sb { s9 } else { sb };
    if sd > m {
        m = sd;
    }
    if sd < 1750 && m > 3250 {
        m = (3250 - ftol((m as f64 - d(F64_3250)) * d(rdata::F64_NEG_0_2))) as i16;
    }
    if kind == 1 {
        if m < 7250 {
            m = 7250;
        }
    } else if kind == 2 {
        if m < 6750 {
            m = 6750;
        }
    } // (the original's third test, kind == 0x14244, can never hold)
    if (st == v75 || st == v92 || st == v105) && m < 7250 {
        m = 7250;
    }
    let total = sb as i32 + s9 as i32 + sd as i32 + 4 * m as i32
        + 50 * (w16(player_sheet::POTENTIAL_ABILITY) as i32 + w16(player_sheet::CURRENT_ABILITY) as i32);
    let mut u = (total / 9).wrapping_add(ftol(frac as f64 * f(rdata::F32_8) - f(rdata::F32_4))) as u32;
    let sn = read_usize(person + staff::NATION);
    if sn != 0 {
        let cn = read_usize(club + club::NATION);
        if cn != 0 {
            let region = read_i8(cn + nation::ACTUAL_REGION);
            if read_i8(sn + nation::ACTUAL_REGION) != region {
                let foreign_too = if cur == 0 {
                    true
                } else {
                    let kn = read_usize(cur + club::NATION);
                    kn != 0 && read_i8(kn + nation::ACTUAL_REGION) != region
                };
                if foreign_too {
                    if u < 1500 {
                        u = 1500;
                    } else {
                        u = ftol(u as u64 as f64 * d(rdata::F64_1_05)) as u32;
                    }
                }
            }
        }
    }
    if u <= 1 {
        u = 1;
    }
    let product = st * u.wrapping_mul(u) as u64 as f64 * d(rdata::F64_1ENEG_08); // mkd2: the reputation squared
    value = product as f32;
    if product < f(F32_175) {
        value = 175.0;
    }
    // the state of development of the nation
    let curn = if cur != 0 { read_usize(cur + club::NATION) } else { 0 };
    let k: i8 = if curn != 0 {
        read_i8(curn + nation::STATE_OF_DEVELOPMENT)
    } else if sd > 4250 {
        1
    } else if sn != 0 {
        read_i8(sn + nation::STATE_OF_DEVELOPMENT)
    } else {
        3
    };
    if k != 1 {
        value = (value as f64 * if k == 2 { f(rdata::F32_0_75) } else { f(rdata::F32_0_5) }) as f32;
    }
    // the world reputation against the club's level
    if club != 0 && c.reputation() != 0 {
        let mut x = sd as f64;
        if c.reputation() < 2500 {
            x *= f(F32_0_0004);
        } else {
            x /= (level * 500) as f64;
        }
        if x < d(rdata::F64_0_85) {
            x = f(rdata::F32_0_8500000238);
        }
        if x > d(rdata::F64_1_15) {
            x = f(rdata::F32_1_149999976);
        }
        value = (x * value as f64) as f32;
    }
    // the level caps
    let squeeze = |value: &mut f32, cap: i32| {
        if *value as f64 > cap as f64 {
            *value = ((*value as f64 - cap as f64) * d(rdata::F64_0_1) + cap as f64) as f32;
        }
    };
    if level < 10 || level < 15 {
        let cap = if level < 10 { level * 2500 } else { level * 2750 };
        if value as f64 > cap as f64 {
            value = cap as f32;
        }
    } else {
        let cn = read_usize(club + club::NATION);
        let key = if cn != 0 { read_i32(cn) } else { 0 };
        if cn != 0 && (key == read_i32(KEY_NATION_ENGLAND) || key == read_i32(KEY_NATION_SPAIN) || key == read_i32(KEY_NATION_ITALY)) {
            squeeze(&mut value, if read_u8(club + club::PLC) != 0 { level * 5250 } else { level * 7500 });
        } else {
            squeeze(&mut value, level * 3250);
        }
    }
    // the cap of the position
    let mut cap: f32 = if at(player_sheet::ATTACKER) == 20 {
        150000.0
    } else if at(player_sheet::ATTACKING_MIDFIELDER) == 20 || at(player_sheet::MIDFIELDER) == 20 || at(player_sheet::DEFENSIVE_MIDFIELDER) == 20 {
        125000.0
    } else if at(player_sheet::DEFENDER) == 20 {
        100000.0
    } else {
        80000.0
    };
    if read_u8(G_FIFA_TRANSFER_RULES_IN_FORCE) == 1 && club != 0 && club_is_in_major_nation(club) != 0 && sb > 7250 {
        cap = (cap as f64 * d(rdata::F64_1_1)) as f32;
    }
    if value > cap {
        value = ((value as f64 - cap as f64) * d(rdata::F64_0_2) + cap as f64) as f32;
    }
    // the decline by the days at the club
    if read_usize(person + staff::CLUB_JOB) == 0 {
        let settled = age >= 33
            && p.int_apps() != 0
            && at(player_sheet::WORK_RATE) as i32 + p.determination() as i32 <= 30
            && read_i32(person) % 15 != 0
            && c.reputation() <= 4500;
        if !settled {
            covered!("mkd_2 days at the club");
            let days = exe::date_days_since(G_CURRENT_DATE, read_u32(person + staff::DATE_JOINED_CLUB), read_u32(person + staff::DATE_JOINED_CLUB_LEAP_YEAR));
            let mut x = if days as f64 > f(rdata::F32_250) { 250.0f32 } else { days as f32 } as f64;
            if exe::club_is_in_season(club, G_CURRENT_DATE) != 0 {
                x += f(rdata::F32_60);
            }
            if x > f(F32_30) {
                x = (x - f(F32_30)) * d(rdata::F64_0_2) + d(rdata::F64_30);
            }
            let mut k = f(F32_1_375) - x * f(F32_0_0125); // mkd2: the decline by days
            if f(F32_0_2000000030) > k {
                k = f(F32_0_2000000030);
            }
            value = (k * value as f64) as f32;
        }
    }
    value
}

/// The part of `staff_contract_default_staff_contract` by the club's nation `cn` (the person works there or is from
/// there; see the header of the routine).
unsafe fn default_wage_by_nation(person: usize, club: usize, cn: usize, sn: usize, sheet: usize, level: i32, value: f32) -> f32 {
    use default_wage_rdata::*;
    let d = |a: usize| read_f64(a);
    let c = Club(club);
    let mut value = value;
    let q = c.reputation() as i32 / 50;
    let key = read_i32(cn);
    let is = |k: usize| key == read_i32(k);
    let league = read_i8(cn + nation::LEAGUE_STANDARD);
    let over = q as i16 as i32 > league as i32 * 10;
    // the club's reputation against the league standard, within k..1
    let factor = |k: usize| {
        let mut x = d(k);
        if over {
            let dv = if league < 1 { 1 } else { league as i32 };
            x = q as i16 as f64 / dv as f64 * d(rdata::F64_0_1) * d(k);
            if x > d(rdata::F64_1) {
                x = d(rdata::F64_1);
            }
        }
        x
    };
    let scale = |value: &mut f32, x: f64| *value = (*value as f64 * x) as f32;
    let squeeze = |value: &mut f32, cap: usize, k: usize| {
        if *value as f64 > d(cap) {
            *value = ((*value as f64 - d(cap)) * d(k) + d(cap)) as f32;
        }
    };
    let pro = c.professional_status() == 1;
    let age = read_i8(person + staff::AGE);
    if is(KEY_NATION_JAPAN) {
        let floor = level * level * 10;
        if (value as f64) < floor as f64 {
            value = floor as f32;
        }
        squeeze(&mut value, rdata::F64_7500, rdata::F64_0_25);
    } else if is(KEY_NATION_ENGLAND) || is(KEY_NATION_ITALY) || is(KEY_NATION_SPAIN) {
    } else if is(KEY_NATION_UNITED_STATES) {
        scale(&mut value, d(rdata::F64_0_75));
        let comp = read_usize(club + club::DIVISION);
        if comp != 0 && read_i32(comp) == read_i32(KEY_CLUB_COMP_AMERICAN_A_LEAGUE) {
            squeeze(&mut value, F64_250, rdata::F64_0_15);
        } else if sn == cn {
            let sh = read_usize(person + staff::PLAYER);
            if age > 31 || (age < 24 && sh != 0 && read_i16(sh + player_sheet::WORLD_REPUTATION) < 3750) {
                squeeze(&mut value, rdata::F64_1250, rdata::F64_0_2);
            } else {
                squeeze(&mut value, F64_2750, rdata::F64_0_2);
            }
        } else {
            squeeze(&mut value, F64_3250, rdata::F64_0_25);
        }
    } else if is(KEY_NATION_SOUTH_KOREA) {
        scale(&mut value, factor(rdata::F64_0_25));
        squeeze(&mut value, rdata::F64_500, rdata::F64_0_5);
        squeeze(&mut value, F64_1500, rdata::F64_0_5);
    } else if is(KEY_NATION_SCOTLAND) {
        scale(&mut value, factor(rdata::F64_0_25));
        if over {
            squeeze(&mut value, rdata::F64_10000, rdata::F64_0_25);
        } else if sheet != 0 {
            let sd = read_i16(sheet + player_sheet::WORLD_REPUTATION);
            if sd < 1250 {
                squeeze(&mut value, rdata::F64_1250, rdata::F64_0_25);
            } else if (age < 20 || age > 32) && sd < 2750 {
                squeeze(&mut value, F64_3250, rdata::F64_0_25);
            } else {
                squeeze(&mut value, F64_7250, rdata::F64_0_25);
            }
        } else {
            squeeze(&mut value, F64_7250, rdata::F64_0_25);
        }
    } else if is(KEY_NATION_GERMANY) || is(KEY_NATION_FRANCE) || is(KEY_NATION_HOLLAND) {
        scale(&mut value, factor(rdata::F64_0_75));
        squeeze(&mut value, F64_20000, rdata::F64_0_25);
    } else if is(KEY_NATION_SWEDEN) {
        scale(&mut value, factor(rdata::F64_0_25));
        if pro { squeeze(&mut value, rdata::F64_500, rdata::F64_0_2) } else { squeeze(&mut value, rdata::F64_150, rdata::F64_0_2) }
    } else if is(KEY_NATION_WALES) || is(KEY_NATION_FINLAND) || is(KEY_NATION_REPUBLIC_OF_IRELAND) || is(KEY_NATION_NORTHERN_IRELAND) || is(KEY_NATION_AUSTRALIA) {
        if pro { squeeze(&mut value, rdata::F64_200, rdata::F64_0_15) } else { squeeze(&mut value, rdata::F64_100, rdata::F64_0_15) }
    } else if is(KEY_NATION_DENMARK) || is(KEY_NATION_NORWAY) {
        scale(&mut value, factor(rdata::F64_0_5));
        if pro { squeeze(&mut value, F64_350, rdata::F64_0_1) } else { squeeze(&mut value, F64_250, rdata::F64_0_1) }
    } else if is(KEY_NATION_BELGIUM) || is(KEY_NATION_PORTUGAL) || is(KEY_NATION_CROATIA) || is(KEY_NATION_POLAND) {
        scale(&mut value, factor(if is(KEY_NATION_BELGIUM) { rdata::F64_0_45 } else { rdata::F64_0_5 }));
        if pro { squeeze(&mut value, rdata::F64_500, rdata::F64_0_2) } else { squeeze(&mut value, F64_250, rdata::F64_0_2) }
    } else if is(KEY_NATION_BRAZIL) || is(KEY_NATION_ARGENTINA) {
        scale(&mut value, factor(rdata::F64_0_45));
        squeeze(&mut value, F64_25000, rdata::F64_0_25);
    } else {
        covered!("mkd_2 other nation");
        let pv = value as f64 * d(rdata::F64_0_3); // mkd2: the other nations
        value = pv as f32;
        if pv > d(rdata::F64_15000) {
            value = ((value as f64 - d(rdata::F64_15000)) * d(rdata::F64_0_25) + d(rdata::F64_15000)) as f32;
        }
    }
    value
}

/// Comparer for `staff_contract_default_staff_contract`: the original and the copy from the same state of the last
/// contract looked up (`G_CONTRACT_LOOKUP_RESULT`) and of the CRT's rand() (the draw of a player without a sheet), the
/// returned float bit for bit.
pub(crate) unsafe extern "stdcall" fn compare_staff_contract_default_staff_contract(person: usize, club: usize, flag: u32, status: u32, job: u32, kind: u32) -> f32 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, usize, u32, u32, u32, u32) -> f32>(%%I0%%);
    let regions = [(G_CONTRACT_LOOKUP_RESULT, 4), (0xad6c30, 4)];
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&regions, || o(person, club, flag, status, job, kind),
        || crate::rng::without_counting(|| staff_contract_default_staff_contract(person, club, flag, status, job, kind)));
    SCOREBOARD.record(same && r.to_bits() == n.to_bits(), || format!("staff_contract_default_staff_contract(person {:x}, club {:x}, {}, {:#x}, {:#x}, {:#x}) original={} rust={} {:x?} x {:x?}",
        person, club, flag, status, job, kind, r, n, mo, mr));
    r
}
