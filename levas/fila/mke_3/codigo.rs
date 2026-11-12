// ---------- batch %%LEVA%%: a person's starting contract (staff_contracts.cpp) ----------

/// The `.rdata` constants of the starting contract that `rdata_consts.rs` does not name yet (cm0102.exe 3.9.68).
#[allow(dead_code)]
pub(crate) mod initial_contract_rdata {
    pub(crate) const F32_32500: usize = 0x96fe64; // f32 = 32500.0
    pub(crate) const F32_21250: usize = 0x96fe60; // f32 = 21250.0
    pub(crate) const F64_150000: usize = 0x96fe58; // f64 = 150000.0
    pub(crate) const F64_9500000: usize = 0x96fe50; // f64 = 9500000.0
    pub(crate) const F64_0_21: usize = 0x96d398; // f64 = 0.21
    pub(crate) const F32_0_25: usize = 0x96d22c; // f32 = 0.25
    pub(crate) const F32_0_125: usize = 0x96e2bc; // f32 = 0.125
}

/// The league of a club (`club::DIVISION` in `G_COMP_OBJECT_TABLE`, a league: `COMP_TYPE` 2) answers a virtual slot
/// of `league_vtable` (`HAS_AUTOMATIC_PROMOTION`, `HAS_RELEGATION`) with ≠ 0; no club, division or league: false.
unsafe fn initial_contract_league_has(club: Club, slot: usize) -> bool {
    let div = read_usize(club.addr() + club::DIVISION);
    if div == 0 {
        return false;
    }
    let comp = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((read_i32(div) as usize).wrapping_mul(4)));
    if comp == 0 || read_u8(comp + 0x43) != 2 {
        return false;
    }
    let m: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(comp) + slot));
    m(comp) != 0
}

/// A bonus of the starting contract: wage / 20 − random(wage / 40) + 25.
unsafe fn initial_contract_bonus(wage: i32) -> i32 {
    let r = exe::utils_random(wage / 40);
    (wage / 20).wrapping_sub(r).wrapping_add(25)
}

/// Generates a person's starting contract (the contract record in ecx, the person, the value) from the database data
/// or, without a usable database wage, from the default contract.
///
/// `generate_initial_contract(person, value)`: the contract's flags reset (`CONTRACT_TYPE` bit 0x80, Bosman, transfer
/// arranged −1, squad status 2, appearances, squad number, +8, morale, grievances, `FLAGS`, `TRANSFER_STATUS` 4, +0x3e,
/// +0x3f) and the person's opinion links cleared (`exe::staff_clear_opinion_links`); with a club job of a nation, the
/// low nibble of `SQUAD_STATUS` is his fluency in its first language (`exe::playerstats_get_language_fluency`, at most
/// 10). The club is his club job, or the club it is the B team of (`exe::club_get_linked_b_team`). A value below 0 is
/// computed (`exe::contract_manager_calc_player_value`) and marks the contract as generated. Without a club: type 3
/// and nothing else. The level: club reputation / 500 + 1 (at a club of professional status 2, one less, between 1 and
/// 4), plus for an international min(4, league standard × 0.25 × caps × 0.125) (0 counts as 1; no nation, 1).
///
/// The database path (a given value, a database wage (`staff::WAGE`) not 0 and at most 15 × the current reputation):
/// type 2, the start the date he joined the club (`DATE_JOINED_CLUB`), with the years since then
/// (`exe::date_get_difference`): a wage above 32500 (over 1 year) or above 21250 for a defender of 20 starts in the
/// game's first year; otherwise the start goes back to today − 2 years (outside 24-29 years, over 2 years) or today − 3
/// (over 3 years); the end is the database's (`DATE_EXPIRES_CLUB`); the wage is the database's. The generated path: the
/// default contract (`exe::staff_contract_default_staff_contract`, job 11, kind 0) gives the wage used for the bonuses;
/// with the game started before today, the contract starts today and ends in a year (previous wage 0); otherwise it
/// starts when he joined (one year back after 2 years) or on 10 July 2000, type 2, the end from the database; the
/// contract wage stays −1 (the f32 −1.0 the original keeps for it).
///
/// The clauses, for a player sheet: manager job (`MANAGER_JOB_RC`) for an old (33+) player of reputation above 3250
/// with id % 3 = 0 and ambition + determination + professionalism + id % 10 + age − 28 above 45, or anyone with a
/// non-player sheet; a player (job 11): the minimum fee (`NON_PROMOTION_RC` = +0x1c) with reputation above 2750 when
/// random(20) + 2 × ambition − 3 × loyalty + reputation / 50 > 12 × level and the league has automatic promotion: fee
/// value + random(value / 4), at least 250000; +0x1e with reputation above 6500 when reputation / 50 − 4 × loyalty −
/// random(20) > 14 × level: v = value + random(value / 4), fee v + max(0.95 v + 150000, 2v) (above 12 million, × 0.21 +
/// 9.5 million), at least 250000; the relegation clause (`RELEGATION_RC`) with reputation above 1750 when reputation /
/// 50 − 2 × loyalty + 3 × ambition − random(20) > 10 × level and the league has relegation: value − random(value / 2),
/// at least 50000, × 0.5 (loyalty < 5, ambition > 10), × 0.75 (ambition > 10) or × 0.85 (loyalty < 15). Each clause
/// clears the minimum fee flag (+0x1d) and the last one sets the release fee. The bonuses from the wage: goals for an
/// attacker of 15+, assists for a midfielder, attacking midfielder or attacker of 15+, clean sheets for a goalkeeper of
/// 20 (wage / 20 − random(wage / 40) + 25; otherwise −1). Without a player sheet no clauses or bonuses. At a club of
/// professional status 2 the type gets bit 0x80; of status 3 the clauses and bonuses are cleared. Then the wage.
///
/// **Defect of the original:** the per-nation wage rules after the clauses (0x85f26d..0x85f74b: South Korea, Japan,
/// the United States and 20 more nations by level², the club's attendance and its professional status) never run: they
/// need the generated contract (the flag at 0) and a wage other than −1.0, and the generated path always keeps −1.0;
/// the copy leaves them out. A person with neither sheet reads an uninitialised reputation (the copy uses 0). Returns
/// the wage (the no-club path returns 0; the callers do not read it). The SEH frame only matters for exceptions.
/// Original: `STAFF_CONTRACT::generate_initial_contract` (0x85e960, staff_contracts.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn generate_initial_contract(this: usize, person: usize, value: i32) -> i32 {
    use initial_contract_rdata::*;
    let c = this;
    let p = Staff(person);
    let mut value = value;
    let mut from_db = true;
    let mut years = [0u8; 8];
    exe::date_date_54a110(years.as_mut_ptr() as usize);
    let mut july = [0u16; 4];
    exe::date_date_549f70(&mut july, 0xa, 6, 0x7d0, u32::MAX);
    write_u8(c + contract::CONTRACT_TYPE, read_u8(c + contract::CONTRACT_TYPE) & 0x7f);
    write_u8(c + contract::LEAVING_ON_BOSMAN, 0);
    write_i32(c + contract::TRANSFER_ARRANGED_FOR, -1);
    write_u8(c + contract::SQUAD_STATUS, (read_u8(c + contract::SQUAD_STATUS) & 0xf) | 0x20);
    write_u16(c + contract::CLUB_APPEARANCES, 0);
    write_u8(c + contract::SQUAD_NUMBER, 0);
    write_u32(c + contract::UNKNOWN, 0);
    let job_club = p.club_job();
    write_u8(c + contract::MORALE, 0);
    write_u8(c + contract::TRANSFER_STATUS, 0);
    write_u32(c + contract::GRIEVANCES, 0);
    exe::staff_clear_opinion_links(staff_at(read_i32(c) as isize));
    write_u32(c + contract::FLAGS, 0);
    write_u8(c + contract::TRANSFER_STATUS, 4);
    write_u8(c + contract::MATCHES_LEFT_OUT, 0);
    write_u8(c + contract::UNHAPPY_FLAG, 0);
    if !job_club.is_null() {
        let nation = read_usize(job_club.addr() + club::NATION);
        if nation != 0 {
            let f = exe::playerstats_get_language_fluency(read_usize(G_PLAYER_STATS), p, job_club, read_u8(nation + nation::FIRST_LANGUAGE) as u32, 0) as i8;
            let v = if f > 10 { 10 } else { f };
            let s = read_u8(c + contract::SQUAD_STATUS);
            write_u8(c + contract::SQUAD_STATUS, (s & 0xf0) | (v as u8 & 0xf));
        }
    }
    let mut club = job_club;
    if !club.is_null() {
        let mut out: u32 = 0;
        let parent = exe::club_get_linked_b_team(club, &mut out, 1);
        if parent != 0 && out == 0 {
            club = Club(parent);
        }
    }
    if value < 0 {
        from_db = false;
        value = exe::contract_manager_calc_player_value(read_usize(G_CONTRACT_MANAGER), p, club, 0, -1, 0);
    }
    if club.is_null() {
        write_u8(c + contract::CONTRACT_TYPE, (read_u8(c + contract::CONTRACT_TYPE) & 0xc3) | 3);
        return 0;
    }
    let sheet = p.player().addr();
    let mut level = ((club.reputation() as i16 as i32) / 500 + 1) as i8;
    if club.professional_status() == 2 {
        level = level.wrapping_sub(1);
        if level < 1 {
            level = 1;
        } else if level > 4 {
            level = 4;
        }
    }
    write_u32(c + contract::CLUB, read_u32(club.addr()));
    let caps = read_u8(p.addr() + staff::INT_APPS);
    if caps != 0 {
        let nation = read_usize(p.addr() + staff::NATION);
        let add: i8 = if nation == 0 {
            1
        } else {
            let std = read_i8(nation + nation::LEAGUE_STANDARD) as f64 * read_f32(F32_0_25) as f64;
            let a = ftol(std * (caps as f64 * read_f32(F32_0_125) as f64)) as i8;
            if a > 4 { 4 } else if a == 0 { 1 } else { a }
        };
        level = level.wrapping_add(add);
    }
    let rep: i16 = if sheet != 0 {
        read_i16(sheet + player_sheet::CURRENT_REPUTATION)
    } else if !p.non_player().is_null() {
        read_i16(p.non_player().addr() + 0xa)
    } else {
        0 // the original reads an uninitialised local
    };
    let wage_f: f32;
    let wage: i32;
    let db_wage = read_i32(p.addr() + staff::WAGE);
    if from_db && db_wage != 0 && db_wage <= rep as i32 * 15 {
        // the database contract
        wage_f = db_wage as f32;
        wage = db_wage;
        write_u8(c + contract::CONTRACT_TYPE, (read_u8(c + contract::CONTRACT_TYPE) & 0xc2) | 2);
        let joined = p.addr() + staff::DATE_JOINED_CLUB;
        exe::date_days_since(G_CURRENT_DATE, read_u32(joined), read_u32(joined + 4));
        let mut t = [0u8; 8];
        let mut u = [0u8; 8];
        let d = exe::date_get_difference(G_CURRENT_DATE, t.as_mut_ptr(), read_u32(joined), read_u32(joined + 4));
        exe::date_operator_3d(years.as_mut_ptr() as usize, u.as_mut_ptr() as usize, d);
        exe::date_operator_3d(c + contract::DATE_STARTED, u.as_mut_ptr() as usize, joined);
        let since = read_u16(years.as_ptr() as usize + 2);
        let sy = c + contract::DATE_STARTED_YEAR;
        if wage_f > read_f32(F32_32500) && since > 1 {
            write_u16(sy, read_u16(G_GAME_START_DATE + 2));
        } else if wage_f > read_f32(F32_21250) && sheet != 0 && read_i8(sheet + player_sheet::DEFENDER) == 20 {
            write_u16(sy, read_u16(G_GAME_START_DATE + 2));
        } else {
            let age = p.age() as i8;
            if !(0x18..=0x1d).contains(&age) && since > 2 {
                write_u16(sy, read_u16(G_CURRENT_DATE + 2).wrapping_sub(2));
            } else if since > 3 {
                write_u16(sy, read_u16(G_CURRENT_DATE + 2).wrapping_sub(3)); // mke3: the database start goes back 3 years
            }
        }
        exe::date_operator_3d(c + contract::CONTRACT_EXPIRES, u.as_mut_ptr() as usize, p.addr() + staff::DATE_EXPIRES_CLUB);
    } else {
        // the generated contract
        wage = ftol(exe::staff_contract_default_staff_contract(c, p, club, 0, u32::MAX, 0xb, 0));
        wage_f = -1.0; // the original stores 0xbf800000
        let mut u = [0u8; 8];
        if fill_slot_date_before(G_GAME_START_DATE, G_CURRENT_DATE) {
            write_u8(c + contract::CONTRACT_TYPE, (read_u8(c + contract::CONTRACT_TYPE) & 0xc2) | 2);
            exe::date_operator_3d(c + contract::DATE_STARTED, u.as_mut_ptr() as usize, G_CURRENT_DATE);
            write_u16(c + contract::CONTRACT_EXPIRES_YEAR, read_u16(G_CURRENT_DATE + 2).wrapping_add(1));
            write_u16(c + contract::CONTRACT_EXPIRES, read_u16(G_CURRENT_DATE));
            write_u32(c + contract::PREVIOUS_WAGE, 0);
        } else {
            let joined = p.addr() + staff::DATE_JOINED_CLUB;
            if fill_slot_date_before(joined, G_CURRENT_DATE) {
                exe::date_operator_3d(c + contract::DATE_STARTED, u.as_mut_ptr() as usize, joined);
                let s = c + contract::DATE_STARTED;
                let mut t = [0u8; 8];
                let d = exe::date_get_difference(G_CURRENT_DATE, t.as_mut_ptr(), read_u32(s), read_u32(s + 4));
                exe::date_operator_3d(years.as_mut_ptr() as usize, u.as_mut_ptr() as usize, d);
                if read_u16(years.as_ptr() as usize + 2) > 2 {
                    write_u16(c + contract::DATE_STARTED_YEAR, read_u16(G_CURRENT_DATE + 2).wrapping_sub(1));
                }
            } else {
                exe::date_operator_3d(c + contract::DATE_STARTED, u.as_mut_ptr() as usize, july.as_ptr() as usize);
            }
            write_u8(c + contract::CONTRACT_TYPE, (read_u8(c + contract::CONTRACT_TYPE) & 0xc2) | 2);
            exe::date_operator_3d(c + contract::CONTRACT_EXPIRES, u.as_mut_ptr() as usize, p.addr() + staff::DATE_EXPIRES_CLUB);
        }
        write_u32(c + contract::UNKNOWN, 0);
    }
    write_u32(c + contract::RELEASE_FEE, 0);
    write_u8(c + contract::MANAGER_JOB_RC, 0);
    write_u8(c + contract::MINIMUM_FEE_RC, 0);
    let clear = |c: usize| {
        for k in 0x1c..=0x20 {
            write_u8(c + k, 0);
        }
        write_i32(c + contract::GOAL_BONUS, -1);
        write_i32(c + contract::ASSIST_BONUS, -1);
        write_i32(c + contract::CLEAN_SHEET_BONUS, -1);
        write_u32(c + contract::RELEASE_FEE, 0);
    };
    if sheet != 0 {
        write_u8(c + contract::MANAGER_JOB_RC, 0);
        let age = p.age() as i8;
        let id = read_i32(p.addr());
        let mut manager_job = false;
        if age > 0x20 && rep > 0xcb2 && id % 3 == 0 {
            let s = read_i8(p.addr() + staff::PROFESSIONALISM) as i32 + read_i8(p.addr() + staff::DETERMINATION) as i32 + read_i8(p.addr() + staff::AMBITION) as i32 + id % 10 + age as i32 - 0x1c;
            manager_job = s > 0x2d;
        }
        if manager_job || !p.non_player().is_null() {
            covered!("mke_3 manager job clause");
            write_u8(c + contract::MINIMUM_FEE_RC, 0);
            write_u8(c + contract::MANAGER_JOB_RC, 1);
        }
        let amb = read_i8(p.addr() + staff::AMBITION) as i32;
        let loy = p.loyalty() as i32;
        let rep = rep as i32;
        let lv = level as i32;
        // the minimum fee (non-promotion) clause
        let mut set = false;
        if p.job_for_club() == 0xb && rep > 0xabe {
            let r = exe::utils_random(20);
            if r + (amb * 2 - loy * 3) + rep / 50 > lv * 12 && initial_contract_league_has(club, league_vtable::HAS_AUTOMATIC_PROMOTION) {
                let fee = (value.wrapping_add(exe::utils_random(value / 4))).max(250000);
                write_u8(c + contract::MINIMUM_FEE_RC, 0);
                write_u8(c + contract::NON_PROMOTION_RC, 1);
                write_i32(c + contract::RELEASE_FEE, fee);
                set = true;
                covered!("mke_3 non-promotion clause");
            }
        }
        if !set {
            write_u8(c + contract::NON_PROMOTION_RC, 0);
        }
        // the +0x1e clause
        set = false;
        if p.job_for_club() == 0xb && rep > 0x1964 {
            let r = exe::utils_random(20);
            if rep / 50 - loy * 4 - r > lv * 14 {
                let v = value.wrapping_add(exe::utils_random(value / 4));
                let a = v as f64 * read_f64(rdata::F64_0_95) + read_f64(F64_150000);
                let b = v.wrapping_mul(2) as f64;
                let mut y = ftol(if a > b { a } else { b });
                if y > 12_000_000 {
                    y = ftol(y as f64 * read_f64(F64_0_21) + read_f64(F64_9500000));
                }
                let fee = v.wrapping_add(y).max(250000);
                write_u8(c + contract::MINIMUM_FEE_RC, 0);
                write_u8(c + 0x1e, 1);
                write_i32(c + contract::RELEASE_FEE, fee);
                set = true;
                covered!("mke_3 clause 0x1e");
            }
        }
        if !set {
            write_u8(c + 0x1e, 0);
        }
        // the relegation clause
        set = false;
        if p.job_for_club() == 0xb && rep > 0x6d6 {
            let r = exe::utils_random(20);
            if rep / 50 - loy * 2 + amb * 3 - r > lv * 10 && initial_contract_league_has(club, league_vtable::HAS_RELEGATION) {
                let v = value.wrapping_sub(exe::utils_random(value / 2)).max(50000);
                let fee = if loy < 5 && amb > 10 {
                    ftol(v as f64 * read_f64(rdata::F64_0_5))
                } else if loy >= 5 && amb > 10 {
                    ftol(v as f64 * read_f64(rdata::F64_0_75))
                } else if loy < 15 {
                    ftol(v as f64 * read_f64(rdata::F64_0_85)) // mke3: the relegation fee × 0.85
                } else {
                    v
                };
                write_u8(c + contract::MINIMUM_FEE_RC, 0);
                write_u8(c + contract::RELEGATION_RC, 1);
                write_i32(c + contract::RELEASE_FEE, fee);
                set = true;
                covered!("mke_3 relegation clause");
            }
        }
        if !set {
            write_u8(c + contract::RELEGATION_RC, 0);
        }
        // the bonuses
        let goal = if read_i8(sheet + player_sheet::ATTACKER) >= 15 { initial_contract_bonus(wage) } else { -1 };
        write_i32(c + contract::GOAL_BONUS, goal);
        let assist = if read_i8(sheet + player_sheet::MIDFIELDER) >= 15 || read_i8(sheet + player_sheet::ATTACKING_MIDFIELDER) >= 15 || read_i8(sheet + player_sheet::ATTACKER) >= 15 {
            initial_contract_bonus(wage)
        } else {
            -1
        };
        write_i32(c + contract::ASSIST_BONUS, assist);
        let clean = if read_i8(sheet + player_sheet::GOALKEEPER) == 20 { initial_contract_bonus(wage) } else { -1 };
        write_i32(c + contract::CLEAN_SHEET_BONUS, clean);
    } else {
        clear(c);
    }
    match club.professional_status() {
        2 => write_u8(c + contract::CONTRACT_TYPE, read_u8(c + contract::CONTRACT_TYPE) | 0x80),
        3 => clear(c),
        _ => {}
    }
    let w = ftol(wage_f as f64);
    write_i32(c + contract::WAGE, w);
    w
}

/// Comparer for `generate_initial_contract`: side by side, the contract record (0x50 bytes), the person's 79-byte
/// state record (the opinion links), the cached contract and the generator's state restored between the two.
pub(crate) unsafe extern "thiscall" fn compare_generate_initial_contract(this: usize, person: usize, value: i32) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32) -> i32>(%%I0%%);
    let id = read_i32(this);
    if id < 0 || id >= read_i32(G_STAFF_COUNT) {
        return o(this, person, value);
    }
    let est = crate::rng::STATE_REGIONS;
    let regions = [(this, 0x50usize), (contract_manager_staff_state_record(id as usize), staff_state::SIZE), (G_CONTRACT_LOOKUP_RESULT, 4usize), est[0], est[1]];
    let (r, _n, same, mo, mr) = crate::comparison::side_by_side_light(&regions,
        || o(this, person, value),
        || crate::rng::without_counting(|| generate_initial_contract(this, person, value)));
    SCOREBOARD.record(same, || format!("generate_initial_contract(person {:#x}, value {}) {:x?} x {:x?}", person, value, mo, mr));
    r
}
