// ---------- batch %%LEVA%%: the value of a contract for the person, against an offer (staff_contracts.cpp) ----------

/// The `.rdata` floats of `staff_contract_calc_contract_value` that `rdata_consts.rs` does not name yet (value in
/// cm0102.exe 3.9.68; still read from the exe).
#[allow(dead_code)]
pub(crate) mod contract_value_rdata {
    pub(crate) const F32_0_2222222239: usize = 0x96ff84; // f32 = 0.2222222238779068
    pub(crate) const F32_1_200000048: usize = 0x96d7bc; // f32 = 1.2000000476837158
    pub(crate) const F32_1_700000048: usize = 0x96ff78; // f32 = 1.7000000476837158
    pub(crate) const F32_1_799999952: usize = 0x96d7b4; // f32 = 1.7999999523162842
    pub(crate) const F32_1_950000048: usize = 0x96ff80; // f32 = 1.9500000476837158
    pub(crate) const F32_1_00999999: usize = 0x96ff7c; // f32 = 1.0099999904632568
    pub(crate) const F32_1_450000048: usize = 0x96ff74; // f32 = 1.4500000476837158
}

/// The multiplier of one release clause of the contract (index `k` from `contract::NON_PROMOTION_RC`: 0 non-promotion,
/// 1 minimum fee, 2 non-playing, 3 relegation, 4 manager job), from the release fees of the contract (`cf`) and of the
/// offer (`of`) in the bands below 1 million, 5 million (7.5 million in case 0) and above, the offer's own clauses and
/// the person's age. Case 0 reads the offer's minimum fee clause, not its non-promotion one (as in the original).
pub(crate) unsafe fn contract_value_clause_factor(k: usize, offer: Contract, cf: i32, age: i8) -> f64 {
    use contract_value_rdata::*;
    const M1: i32 = 1_000_000;
    const M5: i32 = 5_000_000;
    const M7_5: i32 = 7_500_000;
    let f = |a: usize| read_f32(a) as f64;
    let of = || read_i32(offer.addr() + contract::RELEASE_FEE);
    // the three bands of cases 1 (24..28), 3 and 0: equal or higher offer fee 1.75 / 1.5 / 1.2, lower 1.7 / 1.45 / 1.15
    let banded = |top: i32| {
        let o = of();
        if cf < M1 && o < M1 {
            if o >= cf { f(rdata::F32_1_75) } else { f(F32_1_700000048) }
        } else if cf < top && o < top {
            if o >= cf { f(rdata::F32_1_5) } else { f(F32_1_450000048) }
        } else if o >= cf {
            f(F32_1_200000048)
        } else {
            f(rdata::F32_1_149999976)
        }
    };
    match k {
        0 => {
            if offer.minimum_fee_rc() as i8 > 0 { banded(M7_5) } else { f(F32_1_200000048) }
        }
        1 => {
            let o = of();
            if offer.minimum_fee_rc() as i8 <= 0 {
                if cf < M1 && o < M1 { f(F32_1_200000048) } else if cf < M5 && o < M5 { f(rdata::F32_1_049999952) } else { f(F32_1_00999999) }
            } else if age > 28 {
                if cf < M1 && o < M1 { f(rdata::F32_1_100000024) } else { f(rdata::F32_1_049999952) }
            } else if age < 24 {
                if cf < M1 && o < M1 {
                    if o < cf { f(F32_1_950000048) } else { 2.0 } // the original doubles (fadd st0, st0)
                } else if cf < M5 && o < M5 {
                    if o >= cf { f(rdata::F32_1_5) } else { f(F32_1_450000048) }
                } else {
                    f(rdata::F32_1_25)
                }
            } else {
                banded(M5)
            }
        }
        2 => {
            if cf < M5 && of() < M5 { f(F32_1_799999952) } else { f(F32_1_200000048) }
        }
        3 => {
            if offer.relegation_rc() as i8 <= 0 { f(rdata::F32_1_100000024) } else { banded(M5) }
        }
        _ => read_f64(rdata::F64_1_25),
    }
}

/// The value of contract `this` for `person`, against `offer` (another contract, usually the desired one): the wage
/// with the bonuses, the spread fee and the release clauses, times a length factor; used by the contract
/// satisfaction (`get_contract_satisfaction`) and by the decision on contract offers (0x4ce4e0).
///
/// The local contract is a copy of `offer` or, without an offer, the person's desired contract at the club of `this`
/// (`exe::staff_contract_set_to_desired_contract` with −1, −1 and 0xb); only its end year is read afterwards. Fewer
/// than 7 days to the end of `this` (`exe::date_days_since` of `contract::CONTRACT_EXPIRES`, a word): 0. The value
/// starts at the wage (`contract::WAGE`). With a player sheet (`.player()`), by the position mask
/// (`exe::formation_get_position_mask`): goalkeeper (bit 1) + clean sheet bonus × 0.8 (band 33); defender (6) + goal
/// bonus × 0.3 + assist bonus × 0.2 (band 31); midfielder (0x38) × 0.4 and × 0.7 (band 29); attacker (0x40) × 0.8 and
/// × 0.5 (band 28); a bonus of −1 does not count (the last matching group sets the band). Plus `contract::UNKNOWN` /
/// (days × 0.2222222). A sheet with world reputation (`.world_reputation()`) above 4000 and reputation² × 0.01 above
/// `contract::UNKNOWN`: × 0.5. Each active release clause of `this` (`contract::NON_PROMOTION_RC` + k above 0)
/// multiplies the value (`contract_value_clause_factor`). The length factor: a = band − age (no sheet: the age),
/// within −5..5; base 0.75 when the offer has a minimum fee clause, 0.875 when a > 0 and the offer has a relegation or
/// non-promotion clause, otherwise 1; with a < 0 and a sheet, × (1 − a × 0.01); raised (`game_pow`) to |end year of
/// `this` − end year of the local contract|, within 0.5..2. The value is divided by it when `this` ends in an earlier
/// year, otherwise multiplied; truncated (`ftol`). Defects of the original: without an offer it reads the clauses of
/// the offer through a null pointer (every caller passes one); without a sheet the age stands for "band − age".
/// Original: `STAFF_CONTRACT::calc_contract_value` (0x866830, staff_contracts.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn staff_contract_calc_contract_value(this: usize, person: usize, offer: usize) -> i32 {
    use contract_value_rdata::*;
    let c = Contract(this);
    let p = Staff(person);
    let o = Contract(offer);
    // the local STAFF_CONTRACT as the original sets it up
    let mut local = [0u8; 0x50];
    let l = local.as_mut_ptr() as usize;
    exe::date_date_54a110(l + contract::DATE_STARTED);
    exe::date_date_54a110(l + contract::CONTRACT_EXPIRES);
    write_i32(l + contract::CLUB, -1);
    write_i32(l + contract::TRANSFER_ARRANGED_FOR, -1);
    if offer == 0 {
        // 0x85fc50: push 0; push 0; push 0xb; push -1; push -1; push club; push person; lea ecx, local -> thiscall, ret 0x1c
        let club = read_usize(G_CLUB_TABLE).wrapping_add(c.club().wrapping_mul(club::SIZE));
        exe::staff_contract_set_to_desired_contract(l, p, club, -1, u32::MAX, 0xb, 0, 0);
    } else {
        // the original copies the offer field by field (the two dates through Date::operator=): the same 0x50 bytes
        core::ptr::copy_nonoverlapping(offer as *const u8, l as *mut u8, 0x50);
    }
    let days = exe::date_days_since(this + contract::CONTRACT_EXPIRES, read_u32(G_CURRENT_DATE), read_u32(G_CURRENT_DATE_LEAP)) as i16;
    if days < 7 {
        return 0;
    }
    covered!("mkd_1 contract value");
    let age = p.age() as i8;
    let wage = c.wage();
    let mut v32 = wage as f32; // [esp+0x84]
    let sheet = p.player();
    let mut band: i8 = -1;
    let mut st: f64;
    if !sheet.is_null() {
        let mask = exe::formation_get_position_mask(sheet) as u8;
        st = v32 as f64;
        if mask & 1 != 0 {
            let cs = c.clean_sheet_bonus();
            if cs > -1 {
                st = cs as f64 * read_f64(rdata::F64_0_8) + v32 as f64;
            }
            band = 0x21;
        }
        let bonus = |st: f64, kg: usize, ka: usize| {
            let mut s = st;
            if c.goal_bonus() > -1 {
                s += c.goal_bonus() as f64 * read_f64(kg);
            }
            if c.assist_bonus() > -1 {
                s += c.assist_bonus() as f64 * read_f64(ka);
            }
            s
        };
        if mask & 6 != 0 {
            st = bonus(st, rdata::F64_0_3, rdata::F64_0_2);
            band = 0x1f;
        }
        if mask & 0x38 != 0 {
            st = bonus(st, rdata::F64_0_4, rdata::F64_0_7);
            band = 0x1d;
        }
        if mask & 0x40 != 0 {
            st = bonus(st, rdata::F64_0_8, rdata::F64_0_5); // mkd1: the attacker's bonuses
            band = 0x1c;
        }
    } else {
        st = wage as f64; // the exact integer (the original keeps the fild on the stack)
    }
    let spread = read_i32(this + contract::UNKNOWN);
    st = st + spread as f64 / (days as i32 as f64 * read_f32(F32_0_2222222239) as f64); // mkd1: the spread fee
    v32 = st as f32;
    if !sheet.is_null() && sheet.world_reputation() > 4000 {
        let rep = sheet.world_reputation() as i32;
        st = v32 as f64;
        if game_pow(rep as f64, read_f64(rdata::F64_2)) * read_f64(rdata::F64_0_01) > spread as f64 {
            st *= read_f32(rdata::F32_0_5) as f64; // the original also stores it in the slot the clause loop then reuses
        }
    }
    // the release clauses of this contract
    let cf = read_i32(this + contract::RELEASE_FEE);
    for k in 0..5usize {
        if read_i8(this + contract::NON_PROMOTION_RC + k) <= 0 {
            continue;
        }
        covered!("mkd_1 release clause");
        st *= contract_value_clause_factor(k, o, cf, age);
    }
    v32 = st as f32;
    // the length factor
    let years = (read_u16(this + contract::CONTRACT_EXPIRES_YEAR) as i32 - read_u16(l + contract::CONTRACT_EXPIRES_YEAR) as i32) as f32;
    let mut a: i8 = if band != -1 { band.wrapping_sub(age) } else { age };
    a = a.clamp(-5, 5);
    let mut f: f32 = 1.0;
    if o.minimum_fee_rc() as i8 > 0 {
        f = 0.75;
    } else if a > 0 && (o.relegation_rc() as i8 > 0 || o.non_promotion_rc() as i8 > 0) {
        f = 0.875;
    }
    if a < 0 && !sheet.is_null() {
        covered!("mkd_1 older than the band");
        f = ((read_f64(rdata::F64_1) - a as i32 as f64 * read_f64(rdata::F64_0_01)) * f as f64) as f32;
    }
    let mut q = game_pow(f as f64, (years as f64).abs()); // mkd1: the length factor
    if q < read_f64(rdata::F64_0_5) {
        q = read_f64(rdata::F64_0_5);
    } else if q > read_f64(rdata::F64_2) {
        q = read_f64(rdata::F64_2);
    }
    ftol(if years < read_f32(rdata::F32_0) { v32 as f64 / q } else { q * v32 as f64 })
}

compare_value!(compare_staff_contract_calc_contract_value, staff_contract_calc_contract_value, %%I0%%, (person: usize, offer: usize) -> i32);
