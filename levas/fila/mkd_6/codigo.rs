// ---------- batch %%LEVA%%: the default wage of a person at a club, second copy (staff_contracts.cpp) ----------

/// The `.rdata` constants of the default wage that `rdata_consts.rs` does not name yet (cm0102.exe 3.9.68). The value
/// is still read from the exe.
#[allow(dead_code)]
pub(crate) mod unset_default_wage_rdata {
    pub(crate) const F64_250: usize = 0x968898; // f64 = 250.0
    pub(crate) const F64_350: usize = 0x96f870; // f64 = 350.0
    pub(crate) const F64_1500: usize = 0x96feb8; // f64 = 1500.0
    pub(crate) const F64_2750: usize = 0x96fec8; // f64 = 2750.0
    pub(crate) const F64_3250: usize = 0x96fec0; // f64 = 3250.0
    pub(crate) const F64_20000: usize = 0x968888; // f64 = 20000.0
    pub(crate) const F64_25000: usize = 0x9688d0; // f64 = 25000.0
    pub(crate) const F64_1_125: usize = 0x96a5a8; // f64 = 1.125
    pub(crate) const F64_0_00017: usize = 0x96fed0; // f64 = 0.00017
    pub(crate) const F64_0_00012: usize = 0x96ff10; // f64 = 0.00012
    pub(crate) const F64_1ENEG_09: usize = 0x96ff18; // f64 = 1e-09
    pub(crate) const F64_5ENEG_09: usize = 0x96ff28; // f64 = 5e-09
    pub(crate) const F64_2_5ENEG_08: usize = 0x96ff38; // f64 = 2.5e-08
    pub(crate) const F32_0_2: usize = 0x96b028; // f32 = 0.20000000298023224
    pub(crate) const F32_175: usize = 0x96fee8; // f32 = 175.0
    pub(crate) const F32_275: usize = 0x96ff20; // f32 = 275.0
    pub(crate) const F32_550: usize = 0x96fed8; // f32 = 550.0
    pub(crate) const F32_750: usize = 0x96ff30; // f32 = 750.0
    pub(crate) const F32_1_375: usize = 0x96fedc; // f32 = 1.375
    pub(crate) const F32_0_0125: usize = 0x96fee0; // f32 = 0.012500000186264515
    pub(crate) const F32_0_0004: usize = 0x96fee4; // f32 = 0.00039999998989515007
    pub(crate) const F32_8500: usize = 0x96d4e0; // f32 = 8500.0
    pub(crate) const F32_11000: usize = 0x96feec; // f32 = 11000.0
    pub(crate) const F32_20000: usize = 0x96ff00; // f32 = 20000.0
    pub(crate) const F32_25000: usize = 0x96ff34; // f32 = 25000.0
    pub(crate) const F32_30000: usize = 0x96fefc; // f32 = 30000.0
    pub(crate) const F32_40000: usize = 0x96fef0; // f32 = 40000.0
    pub(crate) const F32_45000: usize = 0x96fef4; // f32 = 45000.0
    pub(crate) const F32_50000: usize = 0x96ff04; // f32 = 50000.0
    pub(crate) const F32_52500: usize = 0x96fef8; // f32 = 52500.0
    pub(crate) const F32_55000: usize = 0x96ff08; // f32 = 55000.0
    pub(crate) const F32_65000: usize = 0x96ff0c; // f32 = 65000.0
    /// "STAFF_CONTRACT::default_staff_contract() - No player datafor player: '%s %s' - id %ld"
    pub(crate) const MSG_NO_PLAYER_DATA: usize = 0xa83968;
    /// the game's text buffer the message is written into
    pub(crate) const G_TEXT_BUFFER: usize = 0xde1f64;
}

/// `(v - t) × m + t` above `t`, in the f32 slot of the original ([esp+0x14]).
fn unset_default_wage_soft_cap(v: f32, t: f64, m: f64) -> f32 {
    if v as f64 > t {
        ((v as f64 - t) * m + t) as f32
    } else {
        v
    }
}

/// The league-standard factor of the minor nations: `base`, or (reputation / 50) / standard × 0.1 × `base` (at most 1)
/// when the club's reputation / 50 is above 10 × the standard of the nation (`nation::LEAGUE_STANDARD`, at least 1).
unsafe fn unset_default_wage_standard_factor(nation: Nation, r50: i32, base: f64) -> f64 {
    let ls = nation.league_standard() as i8;
    if r50 <= ls as i32 * 10 {
        return base;
    }
    let den = if ls < 1 { 1 } else { ls as i32 };
    let k = (r50 as f64 / den as f64) * read_f64(rdata::F64_0_1) * base;
    if k > read_f64(rdata::F64_1) {
        read_f64(rdata::F64_1)
    } else {
        k
    }
}

/// Returns (in st0, an f32 value) the default yearly wage of a person at a club: a second compiled copy of
/// `STAFF_CONTRACT::default_staff_contract` (0x864550) used to fill the wages that are not set (the regens' contract,
/// `set_default_wage`, and the initial shortlists, 0x83a1b0).
///
/// `(person, club, check, status, job, floor)`: first the contract (`exe::contract_manager_get_contract`, stored in
/// `G_CONTRACT_LOOKUP_RESULT`) and its club (none for a loan, type 5); the level = the club's reputation / 500 + 1 (byte
/// arithmetic; with a current club and a club of professional status 2, one less, within 1..4), or without a club the
/// world reputation of the player (or staff) sheet / 500, at least 1. The job is the person's job at the club (with a
/// current club), else his national job, else the club job, else 11 with a player sheet, else 9. By the job:
/// - 5..10 (staff): the level rises to the current club's (or the national side's, outside job 11) reputation / 500 + 1;
///   the wage = level × (current reputation³ × k with a staff sheet, else (id / staff count × 4 + 1) × m × 0.0001) ×
///   0.25, with k / m per job (2.5e-8 / 25000, 1e-8 / 10000, 1e-8 / 10000, 5e-9 / 5000, 1e-9 / 1000, 1e-9 / 1000) and the
///   floors and caps 750 (above 25000 → 150000), 500, 500, 275..5000, 250..1000, 200..1000; jobs 6 to 8 at another club
///   are worth 150000 when the current club's reputation / 500 + 4 is above the level (job 6 offering job 6 or 11 below
///   2500: × 1.35);
/// - 11, 12, 13, 15 (players): without a player sheet, a message and `rand() × 4 / 32768` × 550; with no status (−1), the
///   status by reputation and age (1 a star of 24..33, 2 above 0.75 × the club's, 4 above 0.5 or older than 24, 5 / 6
///   by potential × 50 against 0.75 × the club's); the base by position and loyalty (attacker 50000..65000, midfield
///   40000..52500, goalkeeper 20000..25000, defence 25000..30000; 8500 / 10000 for status 5..6 with low reputation,
///   10000 for coaches of 36 or more), raised 2.5..15 % in a major nation with the FIFA rules; score s = (current + home
///   + world + 4 × max(reputations, floor) + 50 × (ability + potential)) / 9 + the id draw, at least 1500 (× 1.05) for
///   a foreigner from another region; wage = base × s² × 1e-8, at least 175; × 0.75 or 0.5 by the nation's development;
///   × the player's reputation against the club's (0.85..1.15); caps by level (2500, 2750 or 3250 per level; 5000 soft
///   in England, Spain and Italy); a soft cap by position (40000..85000); × (1.375 − days at the club × 0.0125) for a
///   free agent (+60 days in season), at least 0.2;
/// - 14 and the others: 1000.
///
/// With a club and a player sheet: the club's offer by status (club reputation × 3 × (reputation × 0.0001..0.00017)⁶,
/// at most 1, × 0.05..1.5 by status and age; the average when it is higher), × 0.67..7.5 by status with grievance 0x10;
/// the nation's market (Japan floor level² × 10, USA × 0.75 and caps by age and league, the league-standard factor of
/// the minor nations and their soft caps); at most the wage of a current contract of status 7; the club's wage cap
/// (`exe::finance_get_max_wage`) when checking or without a club (reputation below 2250), and the ambition bonus
/// (1.05..1.15) of a star; the floors 50 / 75 (pro) or 100 / 75 (amateur, under 20). No club: the wage as computed.
/// Original: `staff_contract_calc_unset_default_wage` (0x862720, staff_contracts.cpp, stdcall, ret 0x18).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "stdcall" fn staff_contract_calc_unset_default_wage(person: usize, club: usize, check: u32, status: u32, job: u32, floor: u32) -> f64 {
    use unset_default_wage_rdata::*;
    let p = Staff(person);
    let c = Club(club);
    let contract = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p);
    write_usize(G_CONTRACT_LOOKUP_RESULT, contract);
    let cur = if contract == 0 || read_u8(contract + contract::CONTRACT_TYPE) & 0x3f == 5 {
        Club(0)
    } else {
        club_at(read_i32(contract + contract::CLUB) as isize)
    };
    let age = p.age() as i8;
    let sheet = p.player();
    let np = p.non_player();
    let frac = (read_i32(p.addr()) as f64 / read_i32(G_STAFF_COUNT) as f64) as f32; // fild id; fidiv count; fstp dword
    let job_arg = job as u8 as i8;
    let mut status = status; // [esp+0x40]: the byte can be rewritten (the dword goes to get_max_wage)
    // the level: the club's reputation / 500 + 1 (byte arithmetic)
    let mut lvl: i32;
    if !c.is_null() {
        lvl = ((c.reputation() as i32 / 500) as u8).wrapping_add(1) as i8 as i32;
        if !cur.is_null() && c.professional_status() == 2 {
            lvl -= 1;
            lvl = lvl.clamp(1, 4);
        }
    } else {
        let q = if !sheet.is_null() {
            Some(sheet.world_reputation() as i32 / 500)
        } else if !np.is_null() {
            Some(np.world_reputation() as i32 / 500)
        } else {
            None
        };
        lvl = match q {
            Some(q) => (if q < 1 { 1 } else { q }) as u8 as i8 as i32,
            None => 1,
        };
    }
    // the job the wage is computed for
    let jfc = p.job_for_club() as i8;
    let jfn = p.job_for_nation() as i8;
    let own_job: i8 = if !cur.is_null() && jfc != 0 {
        jfc
    } else if !p.national_job().is_null() && jfn != 0 {
        jfn
    } else if jfc != 0 {
        jfc
    } else if !sheet.is_null() {
        11
    } else if jfn != 0 {
        jfn
    } else {
        9
    };
    // the level rises to the current club's (or the national side's) reputation / 500 + 1 (staff jobs)
    let raise_level = |lvl: &mut i32| {
        if !cur.is_null() {
            let t = cur.reputation() as i32 / 500 + 1;
            if t > *lvl {
                *lvl = t;
                return;
            }
        }
        let nat = p.national_job();
        if !np.is_null() && !nat.is_null() && jfn != 11 {
            let t = nat.reputation() as i32 / 500 + 1;
            if t > *lvl {
                *lvl = t;
            }
        }
    };
    // the staff wage: level × (current reputation³ × k, or (the id draw × 4 + 1) × m × 0.0001) × 0.25
    let staff_wage = |lvl: i32, k: usize, m: usize| -> f32 {
        let x = if !np.is_null() {
            let r = np.current_reputation() as i32 as f64;
            r * r * r * read_f64(k)
        } else {
            ((frac as f64 * read_f32(rdata::F32_4) as f64 + read_f32(rdata::F32_1) as f64) * read_f32(m) as f64) * read_f64(rdata::F64_0_0001)
        };
        (lvl as f64 * x * read_f64(rdata::F64_0_25)) as f32
    };
    // another club's job when the current club's reputation / 500 + 4 is above the level
    let above_current = |lvl: i32| !cur.is_null() && cur != c && cur.reputation() as i32 / 500 + 4 > lvl;
    let a5_player_or_6 = job_arg == 11 || job_arg == 6;
    let mut v: f32;
    covered!("mkd_6 default wage");
    match own_job {
        5 => {
            raise_level(&mut lvl);
            v = staff_wage(lvl, F64_2_5ENEG_08, F32_25000);
            if read_f32(F32_750) <= v {
                if v > read_f32(F32_25000) {
                    v = 150000.0;
                }
            } else {
                v = 750.0;
            }
        }
        6 => {
            raise_level(&mut lvl);
            v = staff_wage(lvl, rdata::F64_1ENEG_08, rdata::F32_10000);
            if v < read_f32(rdata::F32_500) {
                v = 500.0;
            }
            if !cur.is_null() && cur != c {
                // at another club: 150000 unless the job offered is 6 or 11 and the level reaches the current club's
                if a5_player_or_6 && !above_current(lvl) {
                    if v < read_f32(rdata::F32_2500) {
                        v = (v as f64 * read_f64(rdata::F64_1_35)) as f32;
                    }
                } else {
                    covered!("mkd_6 another club's staff job");
                    v = 150000.0;
                }
            } else if !a5_player_or_6 && v > read_f32(F32_25000) {
                v = 150000.0;
            }
        }
        7 => {
            raise_level(&mut lvl);
            v = staff_wage(lvl, rdata::F64_1ENEG_08, rdata::F32_10000);
            if v < read_f32(rdata::F32_500) {
                v = 500.0;
            }
            if above_current(lvl) {
                v = 150000.0;
            } else if !a5_player_or_6 && v > read_f32(F32_25000) {
                v = 150000.0;
            }
        }
        8 => {
            raise_level(&mut lvl);
            v = staff_wage(lvl, F64_5ENEG_09, rdata::F32_5000);
            if read_f32(F32_275) <= v {
                if v > read_f32(rdata::F32_5000) {
                    v = 5000.0;
                }
            } else {
                v = 275.0;
            }
            if job_arg != 6 && job_arg != 11 && above_current(lvl) {
                v = 150000.0;
            }
        }
        9 | 10 => {
            raise_level(&mut lvl);
            v = staff_wage(lvl, F64_1ENEG_09, rdata::F32_1000);
            let floor_ = if own_job == 9 { rdata::F32_250 } else { rdata::F32_200 };
            if read_f32(floor_) <= v {
                if v > read_f32(rdata::F32_1000) {
                    v = 1000.0;
                }
            } else {
                v = read_f32(floor_);
            }
        }
        11 | 12 | 13 | 15 => {
            if sheet.is_null() {
                // the original writes the error into the game's text buffer and draws the wage
                let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
                format(G_TEXT_BUFFER, MSG_NO_PLAYER_DATA, read_u32(p.addr() + staff::FIRST_NAME), read_u32(p.addr() + staff::SECOND_NAME), read_i32(p.addr()));
                let r = exe::crt_rand();
                let q = (r as i64 * 4) / 0x8000; // __allmul, __alldiv
                v = (q as i32 as f64 * read_f32(F32_550) as f64) as f32;
            } else {
                v = unset_default_wage_player(p, c, cur, sheet, age, frac, lvl, job_arg, floor as u8 as i8, &mut status);
            }
        }
        _ => v = 1000.0,
    }
    if c.is_null() {
        return v as f64;
    }
    if !sheet.is_null() {
        // what the club would pay by squad status: club reputation × 3 × (reputation × k)⁶ (at most 1) × the status' factor
        let rep = c.reputation() as i32;
        let x = if c.professional_status() == 1 {
            rep as f64 * read_f64(F64_0_00017)
        } else if c.reputation() > 0x8ca {
            rep as f64 * read_f64(F64_0_00012)
        } else {
            rep as f64 * read_f64(rdata::F64_0_0001)
        };
        let mut pw = game_pow(x, read_f64(rdata::F64_6));
        if pw > read_f64(rdata::F64_1) {
            pw = 1.0;
        }
        let r3 = (rep * 3) as f64 * pw;
        let major = || club_is_in_major_nation(c.addr()) != 0;
        let st = status as u8 as i8;
        let y: Option<f64> = match st {
            1 => Some(r3 * read_f64(if age < 0x14 { rdata::F64_0_6 } else if age < 0x18 || !major() { rdata::F64_1_25 } else { rdata::F64_1_5 })),
            2 => Some(if age < 0x14 { r3 * read_f64(rdata::F64_0_3) } else if age < 0x18 || !major() { r3 * read_f64(rdata::F64_0_6) } else { r3 }),
            3 => {
                if age < 0x14 {
                    let y0 = r3 * read_f64(rdata::F64_0_25);
                    if (v as f64) < y0 {
                        v = ((v as f64 + y0) * read_f64(rdata::F64_0_5)) as f32;
                    }
                }
                Some(r3 * read_f64(if age < 0x18 || !major() { rdata::F64_0_3 } else { rdata::F64_0_6 }))
            }
            4 => Some(r3 * read_f64(if age < 0x14 { rdata::F64_0_1 } else if age < 0x18 || !major() { rdata::F64_0_15 } else { rdata::F64_0_3 })),
            5 => Some(r3 * read_f64(rdata::F64_0_05)),
            6 => Some(r3 * read_f64(rdata::F64_0_02)),
            _ => None,
        };
        if let Some(y) = y {
            if (v as f64) < y {
                covered!("mkd_6 club's offer by status");
                v = ((v as f64 + y) * read_f64(rdata::F64_0_5)) as f32; // mkd6: the average with the club's offer
            }
        }
        let k = (st as i32 - 1) as u32;
        let c2 = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p);
        if c2 != 0 && read_u8(c2 + contract::GRIEVANCES) & 0x10 != 0 && k <= 5 {
            let m = match k {
                0 | 1 => rdata::F64_0_67,
                2 => rdata::F64_0_85,
                3 => rdata::F64_2_5,
                4 => rdata::F64_3_5,
                _ => rdata::F64_7_5,
            };
            v = (v as f64 * read_f64(m)) as f32;
        }
    }
    // the market of the club's nation
    let nat = c.nation();
    if !nat.is_null() {
        let cj = p.club_job();
        let pn = p.nation();
        let applies = if !cj.is_null() {
            cj.nation() == nat && (pn == nat || p.loyalty() > 3 || p.ambition() < 0xf)
        } else {
            pn == nat
        };
        if applies {
            covered!("mkd_6 nation's market");
            let r50 = c.reputation() as i32 / 50;
            let id = read_u32(nat.addr());
            let is = |k: usize| id == read_u32(k);
            let prof = c.professional_status() == 1;
            if is(KEY_NATION_JAPAN) {
                let fl = lvl * lvl * 10;
                if (v as f64) < fl as f64 {
                    v = fl as f32;
                }
                v = unset_default_wage_soft_cap(v, read_f64(rdata::F64_7500), read_f64(rdata::F64_0_25));
            } else if is(KEY_NATION_ENGLAND) || is(KEY_NATION_ITALY) || is(KEY_NATION_SPAIN) {
            } else if is(KEY_NATION_UNITED_STATES) {
                v = (v as f64 * read_f64(rdata::F64_0_75)) as f32;
                let d = read_usize(c.addr() + club::DIVISION);
                if d != 0 && read_u32(d) == read_u32(KEY_CLUB_COMP_AMERICAN_A_LEAGUE) {
                    v = unset_default_wage_soft_cap(v, read_f64(F64_250), read_f64(rdata::F64_0_15));
                } else if pn == nat {
                    let young = age <= 0x1f && (age >= 0x18 || sheet.is_null() || sheet.world_reputation() >= 0xea6);
                    v = unset_default_wage_soft_cap(v, read_f64(if young { F64_2750 } else { rdata::F64_1250 }), read_f64(rdata::F64_0_2));
                } else {
                    v = unset_default_wage_soft_cap(v, read_f64(F64_3250), read_f64(rdata::F64_0_25));
                }
            } else {
                let scale = |v: f32, base: f64| (v as f64 * unset_default_wage_standard_factor(nat, r50, base)) as f32;
                if is(KEY_NATION_SOUTH_KOREA) {
                    v = scale(v, read_f64(rdata::F64_0_25));
                    v = unset_default_wage_soft_cap(v, read_f64(rdata::F64_500), read_f64(rdata::F64_0_5));
                    v = unset_default_wage_soft_cap(v, read_f64(F64_1500), read_f64(rdata::F64_0_5));
                } else if is(KEY_NATION_SCOTLAND) {
                    v = scale(v, read_f64(rdata::F64_0_35));
                    v = unset_default_wage_soft_cap(v, read_f64(rdata::F64_10000), read_f64(rdata::F64_0_25));
                } else if is(KEY_NATION_GERMANY) || is(KEY_NATION_FRANCE) || is(KEY_NATION_HOLLAND) {
                    v = scale(v, read_f64(rdata::F64_0_75));
                    v = unset_default_wage_soft_cap(v, read_f64(F64_20000), read_f64(rdata::F64_0_25));
                } else if is(KEY_NATION_SWEDEN) {
                    v = scale(v, read_f64(rdata::F64_0_25));
                    v = if prof {
                        unset_default_wage_soft_cap(v, read_f64(rdata::F64_500), read_f64(rdata::F64_0_2))
                    } else {
                        unset_default_wage_soft_cap(v, read_f64(rdata::F64_150), read_f64(rdata::F64_0_2))
                    };
                } else if is(KEY_NATION_WALES) || is(KEY_NATION_FINLAND) || is(KEY_NATION_REPUBLIC_OF_IRELAND) || is(KEY_NATION_NORTHERN_IRELAND) || is(KEY_NATION_AUSTRALIA) {
                    v = if prof {
                        unset_default_wage_soft_cap(v, read_f64(rdata::F64_200), read_f64(rdata::F64_0_15))
                    } else {
                        unset_default_wage_soft_cap(v, read_f64(rdata::F64_100), read_f64(rdata::F64_0_15))
                    };
                } else if is(KEY_NATION_DENMARK) || is(KEY_NATION_NORWAY) || is(KEY_NATION_POLAND) {
                    v = scale(v, read_f64(rdata::F64_0_5));
                    v = if prof {
                        unset_default_wage_soft_cap(v, read_f64(F64_350), read_f64(rdata::F64_0_1))
                    } else {
                        unset_default_wage_soft_cap(v, read_f64(F64_250), read_f64(rdata::F64_0_1))
                    };
                } else if is(KEY_NATION_BELGIUM) {
                    v = scale(v, read_f64(rdata::F64_0_45));
                    v = if prof {
                        unset_default_wage_soft_cap(v, read_f64(rdata::F64_500), read_f64(rdata::F64_0_2))
                    } else {
                        unset_default_wage_soft_cap(v, read_f64(F64_250), read_f64(rdata::F64_0_2))
                    };
                } else if is(KEY_NATION_PORTUGAL) || is(KEY_NATION_CROATIA) {
                    v = scale(v, read_f64(rdata::F64_0_5));
                    v = if prof {
                        unset_default_wage_soft_cap(v, read_f64(rdata::F64_500), read_f64(rdata::F64_0_2))
                    } else {
                        unset_default_wage_soft_cap(v, read_f64(F64_250), read_f64(rdata::F64_0_2))
                    };
                } else if is(KEY_NATION_BRAZIL) || is(KEY_NATION_ARGENTINA) {
                    v = scale(v, read_f64(rdata::F64_0_45));
                    v = unset_default_wage_soft_cap(v, read_f64(F64_25000), read_f64(rdata::F64_0_25));
                } else {
                    // the other nations: × 0.3, the soft cap tested on the unrounded product
                    let x = v as f64 * read_f64(rdata::F64_0_3);
                    v = x as f32;
                    if x > read_f64(rdata::F64_15000) {
                        v = ((v as f64 - read_f64(rdata::F64_15000)) * read_f64(rdata::F64_0_25) + read_f64(rdata::F64_15000)) as f32;
                    }
                }
            }
        }
    }
    if !sheet.is_null() {
        // at most the wage of a current contract of squad status 7
        if !cur.is_null() && c != cur {
            let k = exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), p, cur);
            if k != 0 && read_u8(k + contract::SQUAD_STATUS) & 0xf0 == 0x70 {
                let w = read_i32(k + contract::WAGE) as f64;
                if v as f64 > w {
                    v = w as f32;
                }
            }
        }
        let mut ambition = check == 0 && !cur.is_null();
        if check != 0 || (cur.is_null() && sheet.current_reputation() < 0x8ca) {
            // the club's wage cap
            let fin = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), c);
            if fin == 0 {
                return v as f64;
            }
            let mw = exe::finance_get_max_wage(fin, p, status) as f64;
            if mw < v as f64 {
                covered!("mkd_6 club's wage cap");
                v = mw as f32;
            }
            ambition = false;
        }
        if ambition
            && p.ambition() > 10
            && p.loyalty() < 6
            && sheet.current_reputation() > 0x1a5e
            && (sheet.attacker() == 0x14 || sheet.attacking_midfielder() == 0x14 || sheet.midfielder() == 0x14)
            && age > 0x14
        {
            exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), p, cur); // the result is not used
            let fin = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), c);
            if fin == 0 {
                return v as f64;
            }
            let mw = exe::finance_get_max_wage(fin, p, status) as f64;
            if v as f64 * read_f64(rdata::F64_1_25) < mw {
                let m = if sheet.attacking_midfielder() == 0x14 {
                    rdata::F64_1_1
                } else if sheet.midfielder() == 0x14 {
                    rdata::F64_1_05
                } else {
                    rdata::F64_1_15
                };
                v = (v as f64 * read_f64(m)) as f32;
            }
        }
    }
    // the floors
    if c.professional_status() != 1 {
        if c.reputation() > 0x8ca && (v as f64) < read_f32(rdata::F32_75) as f64 {
            return 75.0;
        }
        if (v as f64) < read_f32(rdata::F32_50) as f64 {
            return 50.0;
        }
        v as f64
    } else {
        if age < 0x14 && (v as f64) < read_f32(rdata::F32_75) as f64 {
            return 75.0;
        }
        if (v as f64) < read_f32(rdata::F32_100) as f64 {
            v = 100.0;
        }
        v as f64
    }
}

/// The player's wage of `staff_contract_calc_unset_default_wage` (jobs 11, 12, 13 and 15, with a player sheet): the
/// squad status by reputation and age when it is −1 (written back into `status`), the base by position and loyalty, the
/// score of the reputations and abilities, the nation's development, the reputation ratio, the caps by level and
/// position and the free agent's factor.
#[allow(clippy::too_many_arguments)]
unsafe fn unset_default_wage_player(p: Staff, c: Club, cur: Club, sheet: PlayerSheet, age: i8, frac: f32, lvl: i32, job_arg: i8, floor: i8, status: &mut u32) -> f32 {
    use unset_default_wage_rdata::*;
    let mut st = *status as u8 as i8;
    if st == -1 && !c.is_null() {
        // the squad status the club would give, by reputation and age
        let pr = sheet.current_reputation();
        let cr = c.reputation();
        st = if pr > cr && (0x18..0x22).contains(&age) {
            1
        } else {
            let d = cr as i32 as f64 * read_f64(rdata::F64_0_75);
            if pr as i32 as f64 > d && age >= 0x15 {
                2
            } else if (pr as i32 as f64 > cr as i32 as f64 * read_f64(rdata::F64_0_5) && age >= 0x11) || age > 0x18 {
                4
            } else if (sheet.potential_ability() as i32 * 50) as f64 > d {
                5
            } else {
                6
            }
        };
        *status = (*status & 0xffff_ff00) | st as u8 as u32;
    }
    // the base by position and loyalty
    let loy = p.loyalty();
    let fac: f32 = if (job_arg == 0xd || job_arg == 0xf) && age > 0x23 {
        read_f32(rdata::F32_10000)
    } else if (st == 6 || st == 5) && sheet.current_reputation() < 0x9c4 {
        read_f32(F32_8500)
    } else if (st == 6 || st == 5) && sheet.current_reputation() < 0x186a {
        read_f32(rdata::F32_10000)
    } else if sheet.attacker() >= 0xf {
        if sheet.current_reputation() > 0x1a5e {
            read_f32(if loy < 0xf { F32_65000 } else { F32_55000 })
        } else {
            read_f32(F32_50000)
        }
    } else if sheet.defensive_midfielder() >= 0xf || sheet.attacking_midfielder() >= 0xf || sheet.midfielder() >= 0xf {
        if sheet.current_reputation() > 0x1c52 {
            read_f32(if loy < 0xe { F32_52500 } else { F32_45000 })
        } else {
            read_f32(F32_40000)
        }
    } else if sheet.goalkeeper() == 0x14 {
        read_f32(if sheet.world_reputation() > 0x1e46 && loy < 3 { F32_25000 } else { F32_20000 })
    } else if sheet.sweeper() < 0xf && sheet.defender() == 0 {
        read_f32(F32_25000)
    } else if sheet.world_reputation() > 0x1d4c && loy < 6 {
        read_f32(F32_30000)
    } else {
        read_f32(F32_25000)
    };
    let mut base = fac as f64;
    if read_u8(G_FIFA_TRANSFER_RULES_IN_FORCE) == 1 && fac > read_f32(F32_11000) && !c.is_null() {
        if club_is_in_major_nation(c.addr()) != 0 {
            if sheet.current_reputation() > 0x1a5e {
                base = fac as f64 * read_f64(if loy == 1 { rdata::F64_1_15 } else if loy < 6 { F64_1_125 } else { rdata::F64_1_1 });
            } else if sheet.current_reputation() > 0x109a {
                base = fac as f64 * read_f64(rdata::F64_1_025);
            }
        }
    }
    // s = (current + home + world + 4 × the greatest (with the floor) + 50 × (ability + potential)) / 9 + the id draw
    let (rc, rh, rw) = (sheet.current_reputation(), sheet.home_reputation(), sheet.world_reputation());
    let mut mx = rc;
    if rh > mx {
        mx = rh;
    }
    if rw > mx {
        mx = rw;
    }
    if floor == 1 {
        if mx < 0x1c52 {
            mx = 0x1c52;
        }
    } else if floor == 2 && mx < 0x1a5e {
        mx = 0x1a5e;
    }
    let draw = ftol(frac as f64 * read_f32(rdata::F32_8) as f64 - read_f32(rdata::F32_4) as f64);
    let sum = (sheet.potential_ability() as i32 + sheet.current_ability() as i32) * 50 + mx as i32 * 4 + rh as i32 + rc as i32 + rw as i32;
    let mut s = (sum / 9).wrapping_add(draw) as u32;
    // a foreigner from another region: at least 1500, × 1.05
    let pn = p.nation();
    let foreign = !pn.is_null() && {
        let cn = c.nation();
        !cn.is_null()
            && pn.actual_region() != cn.actual_region()
            && (cur.is_null() || (!cur.nation().is_null() && cur.nation().actual_region() != cn.actual_region()))
    };
    if foreign {
        covered!("mkd_6 foreigner from another region");
        if s < 0x5dc {
            s = 0x5dc;
        } else {
            s = ftol(s as u64 as f64 * read_f64(rdata::F64_1_05)) as u32;
            if s <= 1 {
                s = 1;
            }
        }
    } else if s <= 1 {
        s = 1;
    }
    let sq = s.wrapping_mul(s) as u64 as f64;
    let x = base * sq * read_f64(rdata::F64_1ENEG_08); // mkd6: the wage by the score
    let mut v = x as f32;
    if x < read_f32(F32_175) as f64 {
        v = 175.0;
    }
    // the development of the current club's (or the person's) nation
    let dev = if !cur.is_null() && !cur.nation().is_null() {
        cur.nation().state_of_development() as i8
    } else if !pn.is_null() {
        pn.state_of_development() as i8
    } else {
        3
    };
    match dev {
        1 => {}
        2 => v = (v as f64 * read_f32(rdata::F32_0_75) as f64) as f32, // mkd6: the nation's development
        _ => v = (v as f64 * read_f32(rdata::F32_0_5) as f64) as f32,
    }
    // the player's reputation against the club's, within 0.85..1.15
    if !c.is_null() && c.reputation() != 0 {
        let mut r = rc as i32 as f64;
        if c.reputation() < 0x9c4 {
            r *= read_f32(F32_0_0004) as f64;
        } else {
            r /= (lvl * 500) as f64;
        }
        if r < read_f64(rdata::F64_0_85) {
            r = read_f32(rdata::F32_0_8500000238) as f64;
        }
        if r > read_f64(rdata::F64_1_15) {
            r = read_f32(rdata::F32_1_149999976) as f64;
        }
        v = (r * v as f64) as f32;
    }
    // the caps by level (soft in England, Spain and Italy)
    let hard = |v: &mut f32, cap: i32| {
        if *v as f64 > cap as f64 {
            *v = cap as f32;
        }
    };
    let soft = |v: &mut f32, cap: i32| {
        if *v as f64 > cap as f64 {
            *v = ((*v as f64 - cap as f64) * read_f64(rdata::F64_0_1) + cap as f64) as f32;
        }
    };
    if lvl < 10 {
        hard(&mut v, lvl * 2500);
    } else if lvl < 15 {
        hard(&mut v, lvl * 2750);
    } else {
        let n = c.nation(); // the original reads +0x53 of the club (a null club with a level of 15 would fault)
        let big = !n.is_null() && {
            let id = read_u32(n.addr());
            id == read_u32(KEY_NATION_ENGLAND) || id == read_u32(KEY_NATION_SPAIN) || id == read_u32(KEY_NATION_ITALY)
        };
        if !big {
            hard(&mut v, lvl * 3250);
        } else if c.plc() != 0 {
            soft(&mut v, lvl * 3250);
        } else {
            soft(&mut v, lvl * 5000);
        }
    }
    // the soft cap by position
    let mut lim: f32 = if sheet.attacker() == 0x14 {
        85000.0
    } else if sheet.attacking_midfielder() == 0x14 || sheet.midfielder() == 0x14 {
        75000.0
    } else if sheet.defensive_midfielder() == 0x14 {
        65000.0
    } else if sheet.defender() == 0x14 {
        50000.0
    } else {
        40000.0
    };
    if read_u8(G_FIFA_TRANSFER_RULES_IN_FORCE) == 1 && v > read_f32(F32_11000) && !c.is_null() && club_is_in_major_nation(c.addr()) != 0 && rc > 0x1c52 {
        lim = (lim as f64 * read_f64(rdata::F64_1_1)) as f32;
    }
    if v > lim {
        v = ((v as f64 - lim as f64) * read_f64(rdata::F64_0_2) + lim as f64) as f32;
    }
    // the free agent: × (1.375 − days at the club × 0.0125), at least 0.2 (veterans of the national side excepted)
    if p.club_job().is_null() {
        let veteran = age >= 0x21
            && p.int_apps() != 0
            && sheet.work_rate() as i32 + p.determination() as i32 <= 0x1e
            && read_i32(p.addr()) % 0xf != 0
            && c.reputation() <= 0x1194;
        if !veteran {
            covered!("mkd_6 free agent");
            let days = exe::date_days_since(G_CURRENT_DATE, read_u32(p.addr() + staff::DATE_JOINED_CLUB), read_u32(p.addr() + staff::DATE_JOINED_CLUB_LEAP_YEAR));
            let mut t = days as f32 as f64;
            if exe::club_is_in_season(c, G_CURRENT_DATE) != 0 {
                t += read_f32(rdata::F32_60) as f64;
            }
            t = read_f32(F32_1_375) as f64 - t * read_f32(F32_0_0125) as f64;
            if !(read_f32(F32_0_2) as f64 <= t) {
                t = read_f32(F32_0_2) as f64;
            }
            v = (t * v as f64) as f32;
        }
    }
    v
}

/// Comparer for `staff_contract_calc_unset_default_wage`: original and copy side by side over
/// `G_CONTRACT_LOOKUP_RESULT`, the start of the text buffer (the no-player message) and the generator's state.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "stdcall" fn compare_staff_contract_calc_unset_default_wage(person: usize, club: usize, check: u32, status: u32, job: u32, floor: u32) -> f64 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, usize, u32, u32, u32, u32) -> f64>(%%I0%%);
    let est = crate::rng::STATE_REGIONS;
    let regions = [(G_CONTRACT_LOOKUP_RESULT, 4usize), (unset_default_wage_rdata::G_TEXT_BUFFER, 0x80), est[0], est[1]];
    let (r, n, same, mo, mr) = crate::comparison::side_by_side_light(&regions,
        || o(person, club, check, status, job, floor),
        || crate::rng::without_counting(|| staff_contract_calc_unset_default_wage(person, club, check, status, job, floor)));
    SCOREBOARD.record(same && r.to_bits() == n.to_bits(), || format!("staff_contract_calc_unset_default_wage(person {:#x}, club {:#x}, {}, {}, {}, {}) original={} rust={} {:x?} x {:x?}",
        person, club, check, status as u8 as i8, job as u8, floor as u8, r, n, mo, mr));
    r
}
