// ---------- batch %%LEVA%%: the desired contract of a person at a club (staff_contracts.cpp) ----------

/// The `.rdata` constants and addresses of `staff_contract_set_to_desired_contract` that `rdata_consts.rs` does not
/// name yet (value in cm0102.exe 3.9.68; still read from the exe).
#[allow(dead_code)]
pub(crate) mod desired_contract_rdata {
    pub(crate) const F64_250: usize = 0x968898; // f64 = 250
    pub(crate) const F64_2500000: usize = 0x96a650; // f64 = 2500000
    pub(crate) const F64_NEG_0_25: usize = 0x96d218; // f64 = -0.25
    pub(crate) const F64_0_21: usize = 0x96d398; // f64 = 0.21
    pub(crate) const F32_8250: usize = 0x96d4e4; // f32 = 8250
    pub(crate) const F32_0_0666666701: usize = 0x96d888; // f32 = 0.06666667014360428
    pub(crate) const F32_0_125: usize = 0x96e2bc; // f32 = 0.125
    pub(crate) const F64_150000: usize = 0x96fe58; // f64 = 150000
    pub(crate) const F32_0_0133333337: usize = 0x96fe68; // f32 = 0.013333333656191826
    pub(crate) const F32_0_0199999996: usize = 0x96fe6c; // f32 = 0.019999999552965164
    pub(crate) const F32_0_0399999991: usize = 0x96fe70; // f32 = 0.03999999910593033
    pub(crate) const F32_1250: usize = 0x96fe74; // f32 = 1250
    pub(crate) const F32_0_0250000004: usize = 0x96fe78; // f32 = 0.02500000037252903
    pub(crate) const F32_NEG_0_5: usize = 0x96fe7c; // f32 = -0.5
    pub(crate) const F32_3200: usize = 0x96fe80; // f32 = 3200
    pub(crate) const F32_4100: usize = 0x96fe84; // f32 = 4100
    pub(crate) const F32_125000: usize = 0x96fe88; // f32 = 125000
    pub(crate) const F32_57500: usize = 0x96fe8c; // f32 = 57500
    pub(crate) const F32_17500: usize = 0x96fe90; // f32 = 17500
    pub(crate) const F32_75000: usize = 0x96fe94; // f32 = 75000
    pub(crate) const F32_35000: usize = 0x96fe98; // f32 = 35000
    pub(crate) const F32_250000: usize = 0x96fe9c; // f32 = 250000 (the "not available" marker)
    pub(crate) const F32_200000: usize = 0x96fea0; // f32 = 200000
    pub(crate) const F32_0_1500000060: usize = 0x96fea4; // f32 = 0.15000000596046448
    pub(crate) const F32_NEG_0_25: usize = 0x96fea8; // f32 = -0.25
    pub(crate) const F32_105000: usize = 0x96feac; // f32 = 105000
    pub(crate) const F32_85000: usize = 0x96feb0; // f32 = 85000
    /// "STAFF_CONTRACT::set_to_desired_contract() - Staff %d has no job for club"
    pub(crate) const NO_JOB_FORMAT: usize = 0xa8391c;
    /// "E:\dev\CM3\cm3 00-01\cm3\code\staff_contracts.cpp" (the source path of the error box).
    pub(crate) const SOURCE_PATH: usize = 0xa8360c;
    /// The game's text buffer (2000 bytes) the message above is formatted into.
    pub(crate) const TEXT_BUFFER: usize = 0xde1f64;
    /// The release clause byte at contract +0x1e (between `contract::MINIMUM_FEE_RC` and `contract::RELEGATION_RC`):
    /// the "not playing" clause [inference: set with the reputation against the club's level, minimum 1500000].
    pub(crate) const NON_PLAYING_RC: usize = 0x1e;
}

/// Fills `this` (a STAFF_CONTRACT) with the contract `person` wants at `club`: the signing fee (+8), the wage (+0xc),
/// the bonuses, the release clauses, the dates and the kind; returns the contract's value (an int) that the callers
/// compare.
///
/// Start: the release fee and the manager-job clause are zeroed; the person's contract
/// (`exe::contract_manager_get_contract`, kept in `G_CONTRACT_LOOKUP_RESULT`) gives the current club `cur` (none for a
/// contract of type 5); without `club`, the club is `cur` (or its linked B team, `exe::club_get_linked_b_team`, when
/// `cur` is not the A team); with `cur`, the current contract `cc` is the one with `cur`
/// (`exe::contract_manager_get_contract_for_club`). The contract gets `contract::CONTRACT_TYPE` = (old & 0x40) | 2,
/// `contract::LEAVING_ON_BOSMAN` 0, `contract::TRANSFER_ARRANGED_FOR` −1, the appearances 0, the person's id and the
/// squad number 0. A status of −1 for a player becomes the deserved one against `cc`
/// (`exe::contract_manager_get_deserved_squad_status`; with grievance 0x10 and not 1, 2) or, without `cc`, the status
/// for the club (`exe::shortlist_manager_get_squad_status_for_club`); it goes to the high nibble of
/// `contract::SQUAD_STATUS`; the grievances of `cc` are kept.
///
/// The level (a byte): with a club, reputation / 500 + 1 (professional status 2: minus one within 1..4) and the club's
/// id at `contract::CLUB`; without one, the world reputation of the player sheet / 500 (no non-player sheet) or the
/// national team's reputation / 500 + 1, at least 3, and the club −1. Internationals add ftol(league standard × 0.25 ×
/// caps × 0.125), at most 4 (0 → 1; no nation → 1). The reputation used by the clauses and the length is the player's
/// current reputation (a player job) or the non-player's current reputation, at least 1.
///
/// The signing fee (player job): `exe::contract_manager_calc_player_value` × 2 / 5, × 0.25 (no club job, current
/// reputation < 2750, club < 2000), × 0.5 (< 4250) or × 0.75 (< 4750); a renewal (club = `cur`): 0 under 19 with
/// < 3750, × 0.1 / 0.5 / 0.25 / 0.5 under 21 with < 4250 (by reputation and pressure), × 0.5 at a club below 3500,
/// then by the deserved status × 0.9 / 0.75 / 0.75 / 0.5 / 0.25; a new club: × 0.5 when `cur` is below 4250 and × 0.5
/// when its nation's league standard is below 10. Staff: 0.
///
/// No club job (`staff::JOB_FOR_CLUB` 0): the "has no job for club" message and the job 11 (player sheet) or 9.
///
/// Staff (not a player job, or no player sheet): w = the default wage (`exe::staff_contract_default_staff_contract`,
/// flag 0, kind 0); the will `exe::staff_interest_in_job_at_club` (written to `will`); w + 2500 with will < 1.75; the
/// value = w × max(3.5 / max(will, 0.1), 0.9); 250000 (the "not available" marker) for job 6 above 2 × the club's
/// reputation + 1200 and 4100, job 8 above reputation + 500 and 3200, job 9 above 0.75 × reputation + 250 and 2100, job
/// 10 the same with 1500, and anything above 250000.
///
/// Players: w = the default wage with kind 0 and job 11. A renewal asks flag 1 (at most the club's maximum wage) when it
/// is cheap: no grievance, no low-nibble mark in the staff state record, current reputation < 7250, fewer than loyalty
/// × 12 + 182 days to the end of `cc` and not leaving on a Bosman; or pressure < 6, under 24, no future transfer
/// (`exe::transfer_manager_get_future_transfer`), not listed (`exe::staff_is_transfer_listed_or_requested`), current
/// reputation < 3250 and < the club's + 250, fewer than 200 − pressure days; or grievances without the bits 0x161a8,
/// fewer than id % 35 + 20 days, no mark and current reputation + ambition × 20 < (loyalty + 170) × 25; or (loyalty
/// > 18, or below 5250 without the mark) with world reputation < 3500. A new club: kind 3 above 6750, else 0 (the tests
/// (100 − ambition − loyalty) × 125000 < 0 and (80 − ...) < 0, which would give kinds 1/2 and status 1/2, never hold
/// with attributes 1..20). Above the current wage: at most max(2 × wage, wage + 8000), and for the famous (world >
/// 6750, w > 10000) above that, 75000 (England, Spain, Italy) or 50000 when the club is `cur` or strong (comp or league
/// reputation × club reputation × 0.05 > 5750), 25% above it. The drive t = world + 25 × (ambition − loyalty) + 10 ×
/// pressure (16 bits): a star at a happy contract, or t above the club's reputation, or a renewal of a deserved star
/// with low loyalty, high ambition and pressure, with fewer than pressure × 10 + 525 days left, left out twice or world
/// ≥ 5750, at a club of a major nation (`club_is_in_major_nation`), 19 to 31 years: w rises to ref × x, ref = the
/// highest wage (`finance::HIGHEST_WAGE`) or 0.0001 × t × 85000 (England) / 105000 (Spain, Italy) when higher (60000
/// for a goalkeeper), x = (ambition − professionalism − loyalty + 4) × 0.0075 within −0.25..0.15, minus 0.1 / 0.05 by
/// age, minus 0.05 below the club's reputation − 1250, plus 0.45 (goalkeeper), 0.7 (defender), 1.0 (attacker, t >
/// 7250, loyalty < 6), 0.95 (t > 7250) or 0.9.
///
/// The will `exe::shortlist_manager_staff_interest_in_joining_club` (written to `will`), q = 3.5 / max(will, 0.1)
/// (stored as f32); with `cc`: grievance 2 caps q at 1.25; other grievances with will < 1.75 add 5000 to w for the
/// restless (pressure > 6, over 23, a future transfer, listed, current reputation > 3249 or above the club's + 250,
/// more than 200 − pressure days); without `cc`, w + 5000 with will < 1.75. q at least 0.9; a renewal caps q: no
/// contract 3.0; grievance 2 (or the young one with exactly 0x10) 1.2, grievance 1 1.75, 0x8400 1.5, others 4.0; no
/// grievance: 1.25 for the loyal (loyalty > id % 5 + 10) with fewer than 30 days, else by age and deserved status (under
/// 19 or over 34: 2.0 / 1.35 / 1.25 / 1.15; under 21 or over 31: 2.0 / 1.5 / 1.35 / 1.15; else 2.0) and 1.1 for the
/// young (< 20, ability < 125 − pressure − ambition, current reputation < 4250, fewer than 120 − pressure − ambition
/// days). The value = q × w; with `cc` (not `this`) paying more, at least its wage × the clubs' reputation ratio (at
/// least 0.5); with the grievance 0x1000000, at least 95% of the highest wage. The marker 250000: above 200000; above
/// 35000 over 34; above 75000 over 31; goalkeeper 20 above 105000; status 7 above 5000, 6 above 8250, 5 above 17500,
/// 4 above 35000, 3 above 57500 with world < 6750 (a human club, or current reputation ≤ 6250, or above 85000 / 105000
/// / 125000 by position and reputation), 2 above 125000 with world < 7250. A dearer current contract at another club
/// blends: value = ((20 − a) × its wage + a × min(w, max(w × 0.5, the maximum wage))) × 0.05, a = 0..20 (grievance
/// 0x100 +10, listed +10, wage above 40000, age over 33, unhappiness and morale), with `exe::finance_get_max_wage`
/// called two to four times.
///
/// Clauses (job 11 with a player sheet): +0x1d 0 and +0x20 1 for the old international (or a non-player sheet); the
/// non-promotion clause (+0x1c, the league's `league_vtable::HAS_AUTOMATIC_PROMOTION`), relegation (+0x1f,
/// `league_vtable::HAS_RELEGATION`), minimum fee (+0x1d) and not playing (+0x1e) with the fee (+0x21) from
/// `exe::contract_manager_calc_player_value` (minimums 150000 / 50000 / 150000 / 1500000); a clause the current
/// contract had at the same club and that is now dropped multiplies the value by 1.05 (1.15 for the minimum fee).
///
/// Dates (with a club): a signing elsewhere gets the window date (`find_next_window_open_date` with no offer,
/// `exe::transfer_manager_find_next_allowed_transfer_date` by the offer's slot), then `contract::DATE_STARTED` is set
/// to today anyway; the end is day `exe::utils_random(28)` + 1 of the season end month
/// (`exe::contract_manager_season_end_month`) of this year (next year when the key nation's season spans two years) +
/// ftol(L + 1) years, L by age and position (3, 4, 3 / 2 / 1.5, 4 or 1), × 0.75 with a clause, by the level against the
/// reputation and the loyalty, by the squad status, by the current reputation, × 0.5 in Finland and Turkey (at most 3),
/// at most 2 / 3 / 4 for a wage below 500 / 1000 / 2000, within 1..`G_MAX_CONTRACT_LENGTH_YEARS` − 1; the kind is 2
/// (with the bit 0x80 when there is no club job, the level is below 2 and a fixed draw by id is 0; in that last case,
/// up to 20 years, kind 1 without the years); a renewal never moves the end before `staff::DATE_EXPIRES_CLUB`.
///
/// Bonuses (+0x10 / +0x14 / +0x18) = ftol(value × draw × 0.025 + value × 0.025 + 25) for attacker / midfielder /
/// goalkeeper 20, else −1; the bit 0x80 of the kind is set below 125 at a club of professional status 2 and cleared
/// above 1250; `contract::WAGE` = ftol(value). The return: ftol(wage (× 0.5 for kind 1, × 1.05 with 0x80) + min(the
/// bonuses / 5 and the clause weights × value, value × 0.1)).
///
/// Defects of the original kept: with an offer without a slot (`exe::transfer_manager_get_offer_slot` 0) it reads the
/// address 0 (`&&` where `||` was meant) before the "Error" box; the reputation of the clauses and the length is left
/// unset (a stack leftover) for a person with a player sheet, a staff job and no non-player sheet (here 0, then 1); the
/// window date is overwritten with today; the kind 1/2 branch of a new club never runs.
/// Original: `STAFF_CONTRACT::set_to_desired_contract` (0x85fc50, staff_contracts.cpp, thiscall, ret 0x1c).
pub(crate) unsafe extern "thiscall" fn staff_contract_set_to_desired_contract(this: usize, person: usize, club: usize, offer: i32, status: u32, job: u32, reasons: usize, will: usize) -> i32 {
    use desired_contract_rdata::*;
    let f = |a: usize| read_f32(a) as f64;
    let d = |a: usize| read_f64(a);
    let ct = this;
    let cm = || read_usize(G_CONTRACT_MANAGER);
    let tm = || read_usize(G_TRANSFER_MANAGER);
    let shortlist = || read_usize(read_usize(G_TRANSFER_MANAGER) + transfer_manager_fields::SHORTLIST_MANAGER);
    let today = || (read_u32(G_CURRENT_DATE), read_u32(G_CURRENT_DATE_LEAP));
    let days_to = |c: usize| {
        let (lo, hi) = today();
        exe::date_days_since(c + contract::CONTRACT_EXPIRES, lo, hi)
    };
    // the club of the person's contract (none for type 5), with the lookup kept in G_CONTRACT_LOOKUP_RESULT
    let lookup = || {
        let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person);
        write_usize(G_CONTRACT_LOOKUP_RESULT, c);
        if c == 0 || read_u8(c + contract::CONTRACT_TYPE) & 0x3f == 5 {
            0
        } else {
            read_usize(G_CLUB_TABLE).wrapping_add(read_usize(c + contract::CLUB).wrapping_mul(club::SIZE))
        }
    };
    let is_player_job = |j: u32| exe::database_is_player_job(j as u8 as i8) != 0;
    let rep_of = |c: usize| read_i16(c + club::REPUTATION);
    let mut club = club;
    let mut status = status;
    let set_status = |s: &mut u32, b: u8| *s = *s & !0xff | b as u32;
    write_u32(ct + contract::RELEASE_FEE, 0);
    write_u8(ct + contract::MANAGER_JOB_RC, 0);
    let mut cc: usize = 0; // the contract with the current club
    let mut grievances: u32 = 0;
    let mut forced_kind: u8 = 0; // the original's [esp+0x3b]: set only by the new-club branch that never runs
    let cur = lookup();
    if club == 0 {
        club = cur;
        if cur != 0 {
            let mut is_a: u32 = 0;
            let linked = exe::club_get_linked_b_team(cur, &mut is_a, 1);
            if linked != 0 && is_a == 0 {
                club = linked;
            }
        }
    }
    if cur != 0 {
        cc = exe::contract_manager_get_contract_for_club(cm(), person, cur);
    }
    write_u8(ct + contract::LEAVING_ON_BOSMAN, 0);
    write_u32(ct + contract::TRANSFER_ARRANGED_FOR, u32::MAX);
    write_u16(ct + contract::CLUB_APPEARANCES, 0);
    write_u8(ct + contract::CONTRACT_TYPE, read_u8(ct + contract::CONTRACT_TYPE) & 0x40 | 2);
    write_u32(ct, read_u32(person));
    write_u8(ct + contract::SQUAD_NUMBER, 0);
    let frac = (read_i32(person) as f64 / read_i32(G_STAFF_COUNT) as f64) as f32; // the fixed draw by id
    let fr = frac as f64;
    let age = read_i8(person + staff::AGE);
    let sheet = read_usize(person + staff::PLAYER);
    let np = read_usize(person + staff::NON_PLAYER);
    let amb = read_i8(person + staff::AMBITION) as i32;
    let loy = read_i8(person + staff::LOYALTY) as i32;
    let pr = read_i8(person + staff::PRESSURE) as i32;
    // the squad status
    if status as u8 == 0xff && sheet != 0 {
        if cc != 0 {
            let mut s = exe::contract_manager_get_deserved_squad_status(cm(), person, cc);
            grievances = read_u32(cc + contract::GRIEVANCES);
            if grievances & 0x10 != 0 && s != 1 {
                s = 2;
            }
            set_status(&mut status, s);
        } else {
            let s = exe::shortlist_manager_get_squad_status_for_club(shortlist(), person, club, 0usize);
            set_status(&mut status, s);
        }
    } else if cc != 0 {
        grievances = read_u32(cc + contract::GRIEVANCES);
    }
    write_u8(ct + contract::SQUAD_STATUS, read_u8(ct + contract::SQUAD_STATUS) & 0xf | (status as u8) << 4);
    // the level
    let mut level: u8;
    if club != 0 {
        let q = (rep_of(club) as i32 / 500) as u8;
        level = q.wrapping_add(1);
        if read_u8(club + club::PROFESSIONAL_STATUS) == 2 {
            level = q;
            if (level as i8) < 1 {
                level = 1;
            } else if (level as i8) > 4 {
                level = 4;
            }
        }
        write_u32(ct + contract::CLUB, read_u32(club));
    } else {
        if np == 0 && sheet != 0 {
            level = (read_i16(sheet + player_sheet::WORLD_REPUTATION) as i32 / 500) as u8;
        } else {
            level = ((rep_of(read_usize(person + staff::NATIONAL_JOB)) as i32 / 500) as u8).wrapping_add(1);
            if (level as i8) < 3 {
                level = 3;
            }
        }
        write_u32(ct + contract::CLUB, u32::MAX);
    }
    let int_apps = read_u8(person + staff::INT_APPS);
    if int_apps != 0 {
        let nation = read_usize(person + staff::NATION);
        let mut add: u8 = 1;
        if nation != 0 {
            add = ftol(read_i8(nation + nation::LEAGUE_STANDARD) as f64 * f(rdata::F32_0_25) * (int_apps as f64 * f(F32_0_125))) as u8;
            if (add as i8) > 4 {
                add = 4;
            } else if add == 0 {
                add = 1;
            }
        }
        level = level.wrapping_add(add);
    }
    let lv = level as i8 as i32;
    // the reputation of the clauses and the length
    let mut rep48: i16 = 0; // the original leaves it unset with a sheet, a staff job and no non-player sheet
    if sheet != 0 && is_player_job(job) {
        rep48 = read_i16(sheet + player_sheet::CURRENT_REPUTATION);
    } else if np != 0 {
        rep48 = read_i16(np + non_player::CURRENT_REPUTATION);
    }
    if rep48 == 0 {
        rep48 = 1;
    }
    // the signing fee
    let fee = ct + contract::UNKNOWN;
    let scale_fee = |k: f64| write_i32(fee, ftol(read_i32(fee) as f64 * k));
    if sheet != 0 && is_player_job(job) {
        let v = exe::contract_manager_calc_player_value(cm(), person, club, 1, -1, 0);
        write_i32(fee, v.wrapping_mul(2) / 5);
        let cr = read_i16(sheet + player_sheet::CURRENT_REPUTATION);
        if read_usize(person + staff::CLUB_JOB) == 0 && cr < 2750 && rep_of(club) < 2000 {
            scale_fee(d(rdata::F64_0_25));
        } else if cr < 4250 {
            scale_fee(d(rdata::F64_0_5));
        } else if cr < 4750 {
            scale_fee(d(rdata::F64_0_75));
        }
        if club == cur {
            if age < 19 && cr < 3750 {
                write_i32(fee, 0);
            } else if age < 21 && cr < 4250 {
                let k = if pr >= 15 { rdata::F64_0_5 } else if cr < 2750 { rdata::F64_0_1 } else { rdata::F64_0_25 };
                scale_fee(d(k));
            } else if club != 0 && cr < 3500 && rep_of(club) < 3500 {
                scale_fee(d(rdata::F64_0_5));
            }
            let k = match exe::contract_manager_get_deserved_squad_status(cm(), person, cc) as i8 {
                1 => rdata::F64_0_9,
                2 | 3 => rdata::F64_0_75,
                4 => rdata::F64_0_5,
                _ => rdata::F64_0_25,
            };
            scale_fee(d(k));
        } else if cur != 0 {
            if rep_of(cur) < 4250 {
                scale_fee(d(rdata::F64_0_5));
            }
            let n = read_usize(cur + club::NATION);
            if n != 0 && read_i8(n + nation::LEAGUE_STANDARD) < 10 {
                scale_fee(d(rdata::F64_0_5));
            }
        }
    } else {
        write_i32(fee, 0);
    }
    // no club job
    let mut staff_path = false;
    if read_u8(person + staff::JOB_FOR_CLUB) == 0 {
        let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
        sprintf(TEXT_BUFFER, NO_JOB_FORMAT, read_i32(person));
        if sheet != 0 {
            write_u8(person + staff::JOB_FOR_CLUB, 0xb);
        } else {
            write_u8(person + staff::JOB_FOR_CLUB, 9);
            staff_path = true;
        }
    }
    if !staff_path && (sheet == 0 || !is_player_job(job)) {
        staff_path = true;
    }
    let mut v: f32; // the value ([esp+0x10])
    let mut w: f32; // the wage asked ([esp+0x24])
    if staff_path {
        covered!("mkd_3 staff contract");
        w = exe::staff_contract_default_staff_contract(ct, person, club, 0, status, job, 0) as f32;
        let will_v = exe::staff_interest_in_job_at_club(shortlist(), person, club, 0, job, reasons as u32) as f32;
        if will != 0 {
            write_f32(will, will_v);
        }
        let wv = will_v as f64;
        let mut q = d(rdata::F64_3_5) / if wv < d(rdata::F64_0_1) { d(rdata::F64_0_1) } else { wv };
        if wv < d(rdata::F64_1_75) {
            w = (w as f64 + f(rdata::F32_2500)) as f32;
        }
        if q < d(rdata::F64_0_9) {
            q = f(rdata::F32_0_8999999762);
        }
        v = (q * w as f64) as f32;
        let vv = v as f64;
        let rep = rep_of(club) as i32;
        let unavailable = match job as u8 {
            6 => ((rep * 2 + 1200) as f64) < vv && vv > f(F32_4100),
            8 => ((rep + 500) as f64) < vv && vv > f(F32_3200),
            9 => rep as f64 * d(rdata::F64_0_75) + d(F64_250) < vv && vv > f(rdata::F32_2100),
            10 => rep as f64 * d(rdata::F64_0_75) + d(F64_250) < vv && vv > f(rdata::F32_1500),
            _ => false,
        };
        if unavailable || vv > f(F32_250000) {
            v = 250000.0;
        }
    } else {
        covered!("mkd_3 player contract");
        let cr = read_i16(sheet + player_sheet::CURRENT_REPUTATION);
        let world = read_i16(sheet + player_sheet::WORLD_REPUTATION);
        let mark = || read_u8(staff_state_entry(Staff(person)) + staff_state::STATUS_FLAGS) & 0xf;
        if cur != 0 && cur == club {
            // a renewal: flag 1 (at most the club's maximum wage) when it is cheap
            let days = days_to(cc);
            let cheap = (grievances == 0 && mark() == 0 && cr < 7250 && days < loy * 12 + 182 && read_u8(cc + contract::LEAVING_ON_BOSMAN) == 0)
                || (pr < 6
                    && age < 24
                    && exe::transfer_manager_get_future_transfer(tm(), person, 0) == 0
                    && exe::staff_is_transfer_listed_or_requested(person, club) == 0
                    && cr < 3250
                    && cr < rep_of(club).wrapping_add(250)
                    && days_to(cc) < 200 - pr)
                || (grievances != 0
                    && grievances & 0x161a8 == 0
                    && days < read_i32(person) % 35 + 20
                    && mark() == 0
                    && (cr as i32 + amb * 20) < (loy + 170) * 25)
                || ((loy > 18 || (cr < 5250 && mark() == 0)) && world < 3500);
            w = exe::staff_contract_default_staff_contract(ct, person, club, cheap as u32, status, 0xb, 0) as f32;
        } else if (100 - amb - loy).wrapping_mul(125000) < 0 {
            set_status(&mut status, 1);
            let kind = if pr > 5 || world > 6250 { 1 } else { 2 };
            w = exe::staff_contract_default_staff_contract(ct, person, club, 0, 1, 0xb, kind) as f32;
            forced_kind = 1;
        } else if (80 - amb - loy).wrapping_mul(125000) < 0 {
            if status as u8 != 1 {
                set_status(&mut status, 2);
            }
            w = exe::staff_contract_default_staff_contract(ct, person, club, 0, status, 0xb, 2) as f32;
        } else {
            let kind = if cr > 6750 { 3 } else { 0 };
            w = exe::staff_contract_default_staff_contract(ct, person, club, 0, status, 0xb, kind) as f32;
        }
        // above the current wage
        if cc != 0 {
            let cw = read_i32(cc + contract::WAGE);
            if (cw as f64) < w as f64 {
                let mut cap = if cw.wrapping_mul(2) > cw.wrapping_add(8000) { cw.wrapping_mul(2) } else { cw.wrapping_add(8000) };
                if world > 6750 && w as f64 > f(rdata::F32_10000) && (cap as f64) < w as f64 {
                    let n = if club != 0 { read_usize(club + club::NATION) } else { 0 };
                    let key = if n != 0 { read_i32(n) } else { 0 };
                    let big: i32 = if n != 0 && (key == read_i32(KEY_NATION_ENGLAND) || key == read_i32(KEY_NATION_SPAIN) || key == read_i32(KEY_NATION_ITALY)) { 75000 } else { 50000 };
                    let strong = if cur == club {
                        true
                    } else if club != 0 {
                        let div = read_usize(club + club::DIVISION);
                        let n = read_usize(club + club::NATION);
                        let r = if div != 0 {
                            ftol((read_i16(div + competition::REPUTATION) as i32).wrapping_mul(rep_of(club) as i32) as f64 * d(rdata::F64_0_05)) as i16
                        } else if n != 0 {
                            ftol((read_i8(n + nation::LEAGUE_STANDARD) as i32).wrapping_mul(rep_of(club) as i32) as f64 * d(rdata::F64_0_05)) as i16
                        } else {
                            rep_of(club)
                        };
                        r > 5750
                    } else {
                        false
                    };
                    if strong {
                        cap = if cap >= big { ftol((w as f64 - big as f64) * d(rdata::F64_0_25) + big as f64) } else { big };
                    }
                }
                if w as f64 > cap as f64 {
                    w = cap as f32;
                }
            }
        }
        let fin = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
        let highest = if fin != 0 { read_i32(fin + finance::HIGHEST_WAGE) } else { 0 };
        // the drive
        let t = (world as i32 + (amb - loy) * 25 + pr * 10) as i16;
        let s = status as u8 as i8;
        let happy = |c: usize| c != 0 && read_i8(c + contract::HAPPINESS) > 50;
        let wants_more = (s == 1 && age > 18 && happy(cc))
            || (world > 4250 && (t > rep_of(club) || (t > 6750 && int_apps != 0 && s < 3 && s != 0)) && s != 7)
            || (cur == club
                && happy(cc)
                && exe::contract_manager_get_deserved_squad_status(cm(), person, cc) == 1
                && loy < 15
                && amb > 10
                && pr > 10
                && t > 6750
                && world > 4250);
        if wants_more
            && (cc == 0 || (days_to(cc) <= pr * 10 + 525 && (read_i8(cc + contract::MATCHES_LEFT_OUT) >= 2 || world >= 5750)))
            && club_is_in_major_nation(club) != 0
            && age > 18
            && age < 32
        {
            covered!("mkd_3 star wage");
            let (mut top, mut reference) = (highest, highest);
            if t > 7250 && read_usize(person + staff::PLAYER) != 0 && read_i8(sheet + player_sheet::GOALKEEPER) != 20 && loy != 20 {
                let x = t as i32 as f64 * d(rdata::F64_0_0001);
                let key = read_i32(read_usize(club + club::NATION));
                let k = if key == read_i32(KEY_NATION_ENGLAND) {
                    Some(F32_85000)
                } else if key == read_i32(KEY_NATION_SPAIN) || key == read_i32(KEY_NATION_ITALY) {
                    Some(F32_105000)
                } else {
                    None
                };
                if let Some(k) = k {
                    let e = ftol(x * f(k));
                    if highest < e {
                        top = e;
                        reference = e;
                    }
                }
            }
            if read_usize(person + staff::PLAYER) != 0 && read_i8(sheet + player_sheet::GOALKEEPER) == 20 && top > 60000 {
                reference = 60000;
            }
            let mut x = (amb - read_i8(person + staff::PROFESSIONALISM) as i32 - loy + 4) as f64 * d(rdata::F64_0_0075);
            if x < d(F64_NEG_0_25) {
                x = f(F32_NEG_0_25);
            } else if x > d(rdata::F64_0_15) {
                x = f(F32_0_1500000060);
            }
            if age > 29 {
                if t < 5250 {
                    x -= d(rdata::F64_0_1);
                } else if pr < 15 {
                    x -= d(rdata::F64_0_05);
                }
            } else if t < 4250 && pr < 15 {
                x -= d(rdata::F64_0_05);
            }
            if (cr as i32 + amb * 25) < rep_of(club).wrapping_sub(1250) as i32 {
                x -= d(rdata::F64_0_05);
            }
            x += if read_i8(sheet + player_sheet::GOALKEEPER) == 20 {
                d(rdata::F64_0_45)
            } else if read_i8(sheet + player_sheet::DEFENDER) == 20 {
                d(rdata::F64_0_7)
            } else if read_i8(sheet + player_sheet::ATTACKER) == 20 && t > 7250 && loy < 6 {
                d(rdata::F64_1)
            } else if t > 7250 {
                d(rdata::F64_0_95)
            } else {
                d(rdata::F64_0_9)
            };
            let y = x * reference as f64;
            if (w as f64) < y {
                w = y as f32;
            }
        }
        // the will
        let will_v = exe::shortlist_manager_staff_interest_in_joining_club(shortlist(), person, club, 0, 0, status, reasons as u32);
        if will != 0 {
            write_f32(will, will_v as f32);
        }
        let mut q = (d(rdata::F64_3_5) / if will_v < d(rdata::F64_0_1) { d(rdata::F64_0_1) } else { will_v }) as f32;
        let restless = if cc != 0 {
            if grievances & 2 != 0 {
                if q as f64 > d(rdata::F64_1_25) {
                    q = 1.25;
                }
                false
            } else {
                grievances != 0
                    && will_v < d(rdata::F64_1_75)
                    && (pr > 6
                        || age > 23
                        || exe::transfer_manager_get_future_transfer(tm(), person, 0) != 0
                        || exe::staff_is_transfer_listed_or_requested(person, club) != 0
                        || read_i16(read_usize(person + staff::PLAYER) + player_sheet::CURRENT_REPUTATION) > 3249
                        || read_i16(read_usize(person + staff::PLAYER) + player_sheet::CURRENT_REPUTATION) > rep_of(club).wrapping_add(250)
                        || days_to(cc) > 200 - pr)
            }
        } else {
            will_v < d(rdata::F64_1_75)
        };
        if restless {
            w = (w as f64 + f(rdata::F32_5000)) as f32; // mkd3: the restless ask 5000 more
        }
        let cap_q = |q: &mut f32, limit: usize, to: u32| {
            if *q as f64 > d(limit) {
                *q = f32::from_bits(to);
            }
        };
        if (q as f64) < d(rdata::F64_0_9) {
            q = f32::from_bits(0x3f666666);
        } else if cur == club {
            if cc == 0 {
                cap_q(&mut q, rdata::F64_3, 0x40400000);
            } else if grievances != 0 {
                if grievances & 2 != 0 || (age < 21 && cr < 4250 && grievances == 0x10) {
                    cap_q(&mut q, rdata::F64_1_2, 0x3f99999a);
                } else if grievances & 1 != 0 {
                    cap_q(&mut q, rdata::F64_1_75, 0x3fe00000);
                } else if grievances & 0x8400 != 0 {
                    cap_q(&mut q, rdata::F64_1_5, 0x3fc00000);
                } else {
                    cap_q(&mut q, rdata::F64_4, 0x40800000);
                }
            } else {
                covered!("mkd_3 renewal demand");
                let loyal = || loy as i8 > ((read_i32(person) % 5) as u8).wrapping_add(10) as i8 && days_to(cc) < 30;
                if loyal() {
                    cap_q(&mut q, rdata::F64_1_25, 0x3fa00000);
                } else if age < 19 || age > 34 {
                    match exe::contract_manager_get_deserved_squad_status(cm(), person, cc) as i8 {
                        1 => cap_q(&mut q, rdata::F64_2, 0x40000000),
                        2 => cap_q(&mut q, rdata::F64_1_35, 0x3faccccd),
                        3..=5 => cap_q(&mut q, rdata::F64_1_25, 0x3fa00000),
                        _ => cap_q(&mut q, rdata::F64_1_15, 0x3f933333),
                    }
                } else if age < 21 || age > 31 {
                    match exe::contract_manager_get_deserved_squad_status(cm(), person, cc) as i8 {
                        1 => cap_q(&mut q, rdata::F64_2, 0x40000000),
                        2 => cap_q(&mut q, rdata::F64_1_5, 0x3fc00000),
                        3..=5 => cap_q(&mut q, rdata::F64_1_35, 0x3faccccd),
                        _ => cap_q(&mut q, rdata::F64_1_15, 0x3f933333),
                    }
                } else if loyal() {
                    cap_q(&mut q, rdata::F64_1_25, 0x3fa00000);
                } else {
                    cap_q(&mut q, rdata::F64_2, 0x40000000);
                }
                if q as f64 > d(rdata::F64_1_1)
                    && age < 20
                    && (read_i16(sheet + player_sheet::CURRENT_ABILITY) as i32) < 125 - pr - amb
                    && cr < 4250
                    && days_to(cc) < 120 - pr - amb
                {
                    q = f32::from_bits(0x3f8ccccd);
                }
            }
        }
        v = (q as f64 * w as f64) as f32; // mkd3: the value asked
        w = v;
        // at least the current wage scaled by the clubs' reputations
        if cc != 0 && cc != ct {
            let cw = read_i32(cc + contract::WAGE) as f64;
            if cw > v as f64 {
                let mut r = f(rdata::F32_1);
                let (a, b) = (rep_of(club), rep_of(cur));
                if a < b {
                    let x = a as i32 as f64 / if b > 100 { b as i32 } else { 100 } as f64;
                    r = if x > d(rdata::F64_0_5) { x } else { d(rdata::F64_0_5) };
                }
                if cw * r > v as f64 {
                    v = (cw * r) as f32;
                }
            }
        }
        if grievances & 0x1000000 != 0 && highest as f64 * d(rdata::F64_0_95) > v as f64 {
            v = (read_i32(fin + finance::HIGHEST_WAGE) as f64 * f(rdata::F32_0_9499999881)) as f32;
        }
        // the "not available" markers
        let vv = v as f64;
        if vv > f(F32_200000) || (age > 34 && vv > f(F32_35000)) || (age > 31 && vv > f(F32_75000)) || (read_i8(sheet + player_sheet::GOALKEEPER) == 20 && vv > f(F32_105000)) {
            v = 250000.0;
        }
        let vv = v as f64;
        let mut done = true;
        match s {
            7 if vv > f(rdata::F32_5000) => v = 250000.0,
            6 if vv > f(F32_8250) => v = 250000.0,
            5 if vv > f(F32_17500) => v = 250000.0,
            4 if vv > f(F32_35000) => v = 250000.0,
            3 if vv > f(F32_57500) && world < 6750 => {
                let mut out = exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0;
                if !out {
                    let sh = read_usize(person + staff::PLAYER);
                    out = sh == 0 || {
                        let r = read_i16(sh + player_sheet::CURRENT_REPUTATION);
                        let limit = if read_i8(sh + player_sheet::ATTACKER) == 20 || read_i8(sh + player_sheet::ATTACKING_MIDFIELDER) == 20 {
                            if r > 7250 { F32_125000 } else { F32_105000 }
                        } else if r > 7250 {
                            F32_105000
                        } else {
                            F32_85000
                        };
                        r <= 6250 || vv > f(limit)
                    };
                }
                if out {
                    v = 250000.0;
                }
            }
            2 if vv > f(F32_125000) && world < 7250 => v = 250000.0,
            _ => done = false,
        }
        if !done {
            if vv > f(F32_250000) {
                v = 250000.0;
            } else if cc != 0 {
                let cw = read_i32(cc + contract::WAGE);
                if cw as f64 > w as f64 && cw != 0 && cur != club {
                    // a dearer contract at another club: blend toward its wage
                    covered!("mkd_3 dearer current contract");
                    let mut a = if read_u32(cc + contract::GRIEVANCES) & 0x100 != 0 { 10.0 } else { 0.0 };
                    if exe::staff_is_transfer_listed_or_requested(person, cur) != 0 {
                        a += d(rdata::F64_10);
                    }
                    if read_i32(cc + contract::WAGE) > 40000 {
                        a += (read_i32(cc + contract::WAGE) - 40000) as f64 * d(rdata::F64_0_00025);
                    }
                    if age > 33 {
                        let k = (38 - age as i32) as f64;
                        a += k + k;
                    }
                    let h = read_i8(cc + contract::HAPPINESS);
                    if h < 0 {
                        a -= h as f64 * d(rdata::F64_0_2);
                    }
                    if read_u32(cc + contract::GRIEVANCES) != 0 {
                        a -= read_i8(cc + contract::MORALE) as f64 * d(rdata::F64_0_2);
                    }
                    if d(rdata::F64_0) > a {
                        a = 0.0;
                    } else if d(rdata::F64_20) < a {
                        a = 20.0;
                    }
                    let half = w as f64 * d(rdata::F64_0_5);
                    let max_wage = || exe::finance_get_max_wage(exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club), person, status) as f64;
                    let m = if max_wage() < half { half } else { max_wage() };
                    let x = if (w as f64) < m {
                        w as f64
                    } else if max_wage() < half {
                        half
                    } else {
                        max_wage()
                    };
                    v = (((d(rdata::F64_20) - a) * read_i32(cc + contract::WAGE) as f64 + x * a) * d(rdata::F64_0_05)) as f32;
                }
            }
        }
    }
    // the release clauses
    let clause = |o: usize| ct + o;
    if read_u8(person + staff::JOB_FOR_CLUB) == 0xb && read_usize(person + staff::PLAYER) != 0 {
        let id = read_i32(person);
        let det = read_i8(person + staff::DETERMINATION) as i32;
        let prof = read_i8(person + staff::PROFESSIONALISM) as i32;
        if (age > 32 && rep48 > 3250 && id % 3 == 0 && age as i32 + prof + det + amb + id % 10 - 28 > 45) || read_usize(person + staff::NON_PLAYER) != 0 {
            write_u8(clause(contract::MINIMUM_FEE_RC), 0);
            write_u8(clause(contract::MANAGER_JOB_RC), 1);
        }
        write_u32(ct + contract::RELEASE_FEE, 0);
        // the club's league answers the virtual `slot` (promotion or relegation)
        let league_has = |slot: usize| {
            if club == 0 {
                return false;
            }
            let div = read_usize(club + club::DIVISION);
            if div == 0 {
                return false;
            }
            let comp = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((read_i32(div) as usize).wrapping_mul(4)));
            if comp == 0 || read_u8(comp + league::COMP_TYPE) != 2 {
                return false;
            }
            let m: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(comp) + slot));
            m(comp) != 0
        };
        let value_plus = |k: f64| {
            let v1 = exe::contract_manager_calc_player_value(cm(), person, club, 1, -1, 0);
            v1.wrapping_add(ftol(v1 as f64 * fr * k))
        };
        let extra = |val: i32| {
            let y = val as f64 * d(rdata::F64_0_95) + d(F64_150000);
            let z = val.wrapping_mul(2) as f64;
            let e = ftol(if y > z { y } else { z });
            if e > 2500000 {
                ftol(e as f64 * d(F64_0_21) - d(F64_2500000) + d(F64_2500000))
            } else {
                e
            }
        };
        let dropped = |o: usize| cc != 0 && read_u8(cc + o) != 0 && lookup() == club;
        let q50 = rep48 as i32 / 50;
        // non-promotion
        if rep48 > 2750 && ((lv * 13) as f64) < q50 as f64 + fr * f(rdata::F32_20) - (loy * 2) as f64 + (amb * 2) as f64 && league_has(league_vtable::HAS_AUTOMATIC_PROMOTION) {
            let mut val = value_plus(f(rdata::F32_0_25));
            if cc != 0 && read_u8(cc + contract::GRIEVANCES) & 1 == 0 {
                val = val.wrapping_add(extra(val));
            }
            write_u8(clause(contract::MINIMUM_FEE_RC), 0);
            write_u8(clause(contract::NON_PROMOTION_RC), 1);
            write_i32(ct + contract::RELEASE_FEE, if val < 150000 { 150000 } else { val });
        } else {
            if dropped(contract::NON_PROMOTION_RC) {
                v = (v as f64 * d(rdata::F64_1_05)) as f32;
            }
            write_u8(clause(contract::NON_PROMOTION_RC), 0);
        }
        // relegation
        if rep48 > 2250 && ((lv * 10) as f64) < (q50 + (amb - loy) * 3) as f64 + fr * f(rdata::F32_20) - age as f64 && league_has(league_vtable::HAS_RELEGATION) {
            let mut val = value_plus(f(F32_NEG_0_5));
            if val < 50000 {
                val = 50000;
            }
            if amb > 10 {
                val = ftol(val as f64 * d(if loy < 5 { rdata::F64_0_5 } else { rdata::F64_0_75 }));
            } else if loy < 15 {
                val = ftol(val as f64 * d(rdata::F64_0_85));
            }
            write_u8(clause(contract::MINIMUM_FEE_RC), 0);
            write_u8(clause(contract::RELEGATION_RC), 1);
            write_i32(ct + contract::RELEASE_FEE, val);
        } else {
            if dropped(contract::RELEGATION_RC) {
                v = (v as f64 * d(rdata::F64_1_05)) as f32;
            }
            write_u8(clause(contract::RELEGATION_RC), 0);
        }
        // minimum fee
        let min_fee = |val: i32| {
            write_u32(clause(contract::NON_PROMOTION_RC), 0);
            write_u8(clause(contract::MANAGER_JOB_RC), 0);
            write_u8(clause(contract::MINIMUM_FEE_RC), 1);
            write_u8(clause(contract::NON_PROMOTION_RC), 0);
            write_u8(clause(contract::RELEGATION_RC), 0);
            write_i32(ct + contract::RELEASE_FEE, if val < 150000 { 150000 } else { val });
        };
        if club == cur && grievances & 1 != 0 {
            min_fee(value_plus(f(rdata::F32_0_25)));
        } else {
            let mut set = false;
            if rep48 > 1250 && !(lookup() != 0 && lv >= 18) && ((lv * 10) as f64) < (amb * 4 - loy * 3 + q50 - age as i32) as f64 + fr * f(rdata::F32_10) {
                let mut val = value_plus(f(rdata::F32_0_5));
                if grievances == 0 {
                    val = val.wrapping_add(extra(val));
                }
                min_fee(val);
                set = true;
            }
            if !set && age < 24 {
                let ab = ftol((read_i16(sheet + player_sheet::CURRENT_ABILITY) as i32 + read_i16(sheet + player_sheet::POTENTIAL_ABILITY) as i32) as f64 * d(rdata::F64_0_5)) as i16 as i32;
                if ab - (loy * 3) as i16 as i32 + (amb * 2) as i16 as i32 > lv * 15 {
                    let mut val = value_plus(f(rdata::F32_0_5));
                    if grievances == 0 {
                        val = val.wrapping_add(extra(val));
                    }
                    min_fee(val);
                    set = true;
                }
            }
            if !set {
                if dropped(contract::MINIMUM_FEE_RC) {
                    v = (v as f64 * d(rdata::F64_1_15)) as f32;
                }
                write_u8(clause(contract::MINIMUM_FEE_RC), 0);
            }
        }
        // not playing
        if rep48 > 6500 && ((lv * 14) as f64) < (q50 - loy * 4) as f64 - fr * f(rdata::F32_20) {
            let mut val = value_plus(f(rdata::F32_0_5));
            if grievances == 0 {
                val = val.wrapping_add(extra(val));
            }
            write_u8(clause(contract::MINIMUM_FEE_RC), 0);
            write_u8(clause(NON_PLAYING_RC), 1);
            write_i32(ct + contract::RELEASE_FEE, if val < 1500000 { 1500000 } else { val });
        } else {
            write_u8(clause(NON_PLAYING_RC), 0);
        }
    } else {
        for o in contract::NON_PROMOTION_RC..=contract::MANAGER_JOB_RC {
            write_u8(clause(o), 0);
        }
    }
    // the dates and the length
    if club != 0 {
        let mut tmp = [0u8; 8];
        let tp = tmp.as_mut_ptr() as usize;
        if sheet != 0 && cur != club {
            if offer == -1 {
                find_next_window_open_date(tm(), club, ct + contract::DATE_STARTED);
            } else {
                let slot = exe::transfer_manager_get_offer_slot(tm(), offer);
                // defect of the original: `&&` for `||`; without a slot it reads the address 0 first
                if slot == 0 && core::ptr::read_volatile(core::hint::black_box(slot) as *const i32) == 0 {
                    let version: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
                    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
                    let (mut name, mut ext) = ([0u8; 256], [0u8; 256]);
                    exe::crt_strtok(SOURCE_PATH, 0, 0, name.as_mut_ptr(), ext.as_mut_ptr());
                    let ver = version(name.as_mut_ptr(), ext.as_mut_ptr(), 0xc4e);
                    sprintf(0xae24d0, 0x9870e8, ver, name.as_ptr(), ext.as_ptr(), 0xc4e);
                    exe::gui_show_message_box(0x9870e0, 0xae24d0, 0);
                    write_u32(G_MOUSE_LEFT_BUTTON_DOWN, 0);
                    exe::date_operator_3d(ct + contract::DATE_STARTED, tp, G_CURRENT_DATE);
                } else {
                    exe::transfer_manager_find_next_allowed_transfer_date(tm(), read_usize(slot), (ct + contract::DATE_STARTED) as *mut [u32; 2]);
                }
            }
        }
        exe::date_operator_3d(ct + contract::DATE_STARTED, tp, G_CURRENT_DATE);
        let month = exe::contract_manager_season_end_month(cm(), club);
        let day = exe::utils_random(0x1c) + 1;
        let mut dmy = [0u16; 4];
        let day_of_year = read_u16(exe::date_date_549f70(&mut dmy, day as u32, month as u32, 0x7ce, u32::MAX));
        let k = exe::key_nation_index_of_club(club) as i8 as isize;
        let mut year = read_u16(G_CURRENT_DATE_YEAR);
        if read_u32((G_LEAGUE_TABLE as isize + 72 * k) as usize + key_nation_record::SPLIT_SEASON_FLAG) != 0 {
            year = year.wrapping_add(1);
        }
        let sh = read_usize(person + staff::PLAYER);
        let mut l: f64 = if age < 19 {
            f(rdata::F32_3)
        } else if age < 29 {
            f(rdata::F32_4)
        } else if age < 34 {
            if sh != 0 && read_i8(sh + player_sheet::GOALKEEPER) > 15 {
                f(rdata::F32_3)
            } else if sh != 0 && read_i8(sh + player_sheet::DEFENDER) > 15 {
                f(rdata::F32_2)
            } else {
                f(rdata::F32_1_5)
            }
        } else if np != 0 && sh == 0 && read_u8(person + staff::JOB_FOR_CLUB) != 0xb {
            f(rdata::F32_4)
        } else {
            f(rdata::F32_1)
        };
        if read_i8(clause(contract::RELEGATION_RC)) > 0 || read_i8(clause(contract::NON_PROMOTION_RC)) > 0 {
            l *= d(rdata::F64_0_75);
        }
        if read_i8(clause(contract::MINIMUM_FEE_RC)) > 0 {
            l = f(rdata::F32_1);
        } else {
            let l500 = lv * 500;
            l *= l500 as f64 / if rep48 as i32 > l500 { rep48 as i32 } else { l500 } as f64;
            if age < 32 {
                l += (loy as f64 * d(rdata::F64_0_05) + d(rdata::F64_0_5)) * fr;
            }
        }
        if sheet != 0 {
            match status as u8 as i8 {
                1 => l *= d(rdata::F64_1_25),
                3 | 5 => {
                    l *= d(rdata::F64_0_75);
                    if age < 24 && lv > 10 && l < d(rdata::F64_2) {
                        l = f(rdata::F32_2);
                    }
                }
                4 => l *= d(rdata::F64_0_5),
                6 => {
                    l *= d(rdata::F64_0_5);
                    if l > d(rdata::F64_2) {
                        l = f(rdata::F32_2);
                    }
                }
                7 => l = f(rdata::F32_1),
                _ => {}
            }
            let r = read_i16(sheet + player_sheet::CURRENT_REPUTATION) as i32;
            if r > 2250 && r > lv * 675 {
                if r > 3250 && r > lv * 875 {
                    if age > 20 && age < 31 {
                        if loy < 15 {
                            l = f(rdata::F32_1);
                        } else {
                            l -= d(rdata::F64_2);
                        }
                    }
                } else if age <= 20 {
                    l -= d(rdata::F64_1);
                } else if age >= 27 {
                    if age > 29 && age < 32 && l < d(rdata::F64_2) {
                        l += d(rdata::F64_1);
                    }
                } else if loy < 5 {
                    l = f(rdata::F32_1);
                } else if l > f(rdata::F32_3) {
                    l -= d(rdata::F64_1);
                }
            }
        }
        if forced_kind != 0 {
            l = if l >= d(rdata::F64_3) {
                l + d(rdata::F64_1)
            } else if age < 27 {
                f(rdata::F32_5)
            } else if age < 30 {
                f(rdata::F32_4)
            } else {
                f(rdata::F32_3)
            };
        }
        let n = read_usize(club + club::NATION);
        if n != 0 && (read_i32(n) == read_i32(KEY_NATION_FINLAND) || read_i32(n) == read_i32(KEY_NATION_TURKEY)) {
            l *= d(rdata::F64_0_5);
            if l > d(rdata::F64_3) {
                l = f(rdata::F32_3);
            }
        }
        let vv = v as f64;
        let most = if vv < f(rdata::F32_500) {
            Some(rdata::F64_2)
        } else if vv < f(rdata::F32_1000) {
            Some(rdata::F64_3)
        } else if vv < f(rdata::F32_2000) {
            Some(rdata::F64_4)
        } else {
            None
        };
        if let Some(m) = most {
            if l >= d(m) {
                l = d(m);
            }
        }
        let max_years = read_i8(G_MAX_CONTRACT_LENGTH_YEARS) as i32;
        if l > (max_years - 1) as f64 {
            l = max_years as f64 - f(rdata::F32_1);
        } else if l < d(rdata::F64_1) {
            l = f(rdata::F32_1);
        }
        let kind_bits = |or: u8, and: u8| write_u8(ct + contract::CONTRACT_TYPE, read_u8(ct + contract::CONTRACT_TYPE) & and | or);
        let no_job_low = read_usize(person + staff::CLUB_JOB) == 0 && (level as i8) < 2;
        let mut add_years = true;
        if rep48 > (level as i8 as i16).wrapping_mul(500).wrapping_add(1250) {
            if no_job_low && ftol(fr * d(rdata::F64_4)) as u8 == 0 {
                kind_bits(0x82, 0xc2);
            } else {
                kind_bits(2, 0xc2);
            }
        } else if no_job_low && ftol(fr * d(rdata::F64_5)) as u8 == 0 {
            if age > 20 {
                kind_bits(0x82, 0xc2);
            } else {
                kind_bits(1, 0xc1);
                add_years = false;
            }
        } else {
            kind_bits(2, 0xc2);
        }
        if add_years {
            year = year.wrapping_add(ftol(l + d(rdata::F64_1)) as u16); // mkd3: the years of the contract
        }
        let mut end = [0u8; 8];
        let ep = end.as_mut_ptr() as usize;
        exe::new_date(ep, day_of_year as u32, year as u32);
        // a renewal never ends before the person's date with the club
        let pd = person + staff::DATE_EXPIRES_CLUB;
        let (py, ey) = (read_u16(pd + 2), read_u16(ep + 2));
        if (py > ey || (py == ey && read_i16(pd) > read_i16(ep))) && lookup() == club {
            exe::date_operator_3d(ep, tp, pd);
        }
        exe::date_operator_3d(ct + contract::CONTRACT_EXPIRES, tp, ep);
    }
    // the bonuses
    let vv = v as f64;
    let bonus = || ftol(vv * fr * f(F32_0_0250000004) + vv * f(F32_0_0250000004) + f(rdata::F32_25));
    let at = |o: usize| read_i8(sheet + o);
    if sheet != 0 {
        write_i32(ct + contract::GOAL_BONUS, if at(player_sheet::ATTACKER) >= 15 { bonus() } else { -1 });
        let outfield = at(player_sheet::MIDFIELDER) >= 15 || at(player_sheet::ATTACKING_MIDFIELDER) >= 15 || at(player_sheet::ATTACKER) >= 15;
        write_i32(ct + contract::ASSIST_BONUS, if outfield { bonus() } else { -1 });
        write_i32(ct + contract::CLEAN_SHEET_BONUS, if at(player_sheet::GOALKEEPER) == 20 { bonus() } else { -1 });
    }
    let kind = read_u8(ct + contract::CONTRACT_TYPE);
    if vv < f(rdata::F32_125) && club != 0 && read_u8(club + club::PROFESSIONAL_STATUS) == 2 {
        write_u8(ct + contract::CONTRACT_TYPE, kind | 0x80);
    } else if vv > f(F32_1250) && kind & 0x80 != 0 {
        write_u8(ct + contract::CONTRACT_TYPE, kind & 0x7f);
    }
    let wage = ftol(vv);
    write_i32(ct + contract::WAGE, wage);
    // the return: the wage and the bonuses and clauses, these at most a tenth of the value
    let kind = read_u8(ct + contract::CONTRACT_TYPE);
    let mut paid = wage as f64;
    if ((kind << 2) as i8 >> 2) == 1 {
        paid *= d(rdata::F64_0_5);
    }
    if kind & 0x80 != 0 {
        paid *= d(rdata::F64_1_05);
    }
    let mut extras = f(rdata::F32_0);
    if sheet != 0 {
        let outfield = at(player_sheet::MIDFIELDER) >= 15 || at(player_sheet::ATTACKING_MIDFIELDER) >= 15 || at(player_sheet::ATTACKER) >= 15;
        let keeper = at(player_sheet::GOALKEEPER) == 20;
        for i in 0..3 {
            let b = read_i32(ct + contract::GOAL_BONUS + 4 * i);
            if b > -1 {
                let x = (b / 5) as f64;
                let plus = if i < 2 { outfield } else { keeper };
                if plus {
                    extras += x;
                } else if i == 2 || keeper {
                    extras -= x;
                }
            }
        }
    }
    for c in 0..5 {
        if read_u8(clause(contract::NON_PROMOTION_RC + c)) != 0 {
            let fee = read_i32(ct + contract::RELEASE_FEE);
            let k = match c {
                1 if fee < 500000 => rdata::F32_0_1000000015,
                1 if fee < 2500000 => rdata::F32_0_05000000075_AT_96D61C,
                1 => F32_0_0133333337,
                2 if fee < 2500000 => rdata::F32_0_1000000015,
                2 => F32_0_0666666701,
                _ if fee < 500000 => F32_0_0666666701,
                _ if fee < 2500000 => F32_0_0399999991,
                _ => F32_0_0199999996,
            };
            extras += vv * f(k);
        }
    }
    if sheet == 0 {
        for o in contract::MINIMUM_FEE_RC..=contract::MANAGER_JOB_RC {
            write_u8(clause(o), 0);
        }
        write_i32(ct + contract::GOAL_BONUS, -1);
        write_i32(ct + contract::ASSIST_BONUS, -1);
        write_i32(ct + contract::CLEAN_SHEET_BONUS, -1);
    }
    let tenth = (vv * f(rdata::F32_0_1000000015)) as f32 as f64;
    ftol(if extras > tenth { tenth } else { extras } + paid)
}

/// Comparer for `staff_contract_set_to_desired_contract`: the original and the copy side by side from the same state of
/// the contract (0x50 bytes), the last contract looked up (`G_CONTRACT_LOOKUP_RESULT`), the game's generator, the text
/// buffer (the "no job for club" message), the person's `staff::JOB_FOR_CLUB` and the will and reasons the caller
/// passed; the contract, those and the returned value must match (the callers' comparers already run it twice).
pub(crate) unsafe extern "thiscall" fn compare_staff_contract_set_to_desired_contract(this: usize, person: usize, club: usize, offer: i32, status: u32, job: u32, reasons: usize, will: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, i32, u32, u32, usize, usize) -> i32>(%%I0%%);
    let mut regions = vec![(this, 0x50usize), (G_CONTRACT_LOOKUP_RESULT, 4), (desired_contract_rdata::TEXT_BUFFER, 0x50), (person + staff::JOB_FOR_CLUB, 1)];
    regions.extend(crate::rng::STATE_REGIONS);
    if will != 0 {
        regions.push((will, 4));
    }
    if reasons != 0 {
        regions.push((reasons, 4));
    }
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, person, club, offer, status, job, reasons, will),
        || crate::rng::without_counting(|| staff_contract_set_to_desired_contract(this, person, club, offer, status, job, reasons, will)));
    SCOREBOARD.record(same && r == n, || format!("staff_contract_set_to_desired_contract(person {:#x}, club {:x}, offer {}, status {:#x}, job {:#x}) original={} rust={} {:x?} x {:x?}",
        read_u32(person), club, offer, status, job, r, n, mo, mr));
    r
}
