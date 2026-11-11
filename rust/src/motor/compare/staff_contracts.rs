use super::*;

// The comparers of the routines in ../staff_contracts.rs.

/// Comparer for `has_active_release_clause`.
pub(crate) unsafe extern "thiscall" fn compare_has_active_release_clause(c: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_HAS_ACTIVE_RELEASE_CLAUSE)(c);
    let n = has_active_release_clause(c);
    SCOREBOARD.record(r == n, || format!("has_active_release_clause({:x}) original={} rust={}", c, r, n));
    r
}

/// Comparer for `check_fifa_protected_period`.
pub(crate) unsafe extern "thiscall" fn compare_check_fifa_protected_period(c: usize, x: usize, in_the_limit: *mut u32, years: *mut u8) -> u32 {
    // the outputs as they were before: the copy starts from them (the original only writes `anos` in the period)
    let (mut nl, mut na) = (if in_the_limit.is_null() { 0 } else { *in_the_limit }, if years.is_null() { 0 } else { *years });
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut u32, *mut u8) -> u32>(IDX_CHECK_FIFA_PROTECTED_PERIOD)(c, x, in_the_limit, years);
    let (ol, oa) = (if in_the_limit.is_null() { 0 } else { *in_the_limit }, if years.is_null() { 0 } else { *years });
    let n = check_fifa_protected_period(
        c,
        x,
        if in_the_limit.is_null() { core::ptr::null_mut() } else { &mut nl },
        if years.is_null() { core::ptr::null_mut() } else { &mut na },
    );
    let ok = r == n && (in_the_limit.is_null() || ol == nl) && (years.is_null() || oa == na);
    SCOREBOARD.record(ok, || format!("check_fifa_protected_period({:x}) original={} {} {} rust={} {} {}", c, r, ol, oa, n, nl, na));
    r
}

/// Comparer for `staff_contract_is_bonus_type_allowed`.
pub(crate) unsafe extern "C" fn compare_staff_contract_is_bonus_type_allowed(person: usize, kind: u32) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize, u32) -> u32>(IDX_STAFF_CONTRACT_IS_BONUS_TYPE_ALLOWED)(person, kind);
    let n = staff_contract_is_bonus_type_allowed(person, kind);
    SCOREBOARD.record(r == n, || format!("staff_contract_is_bonus_type_allowed({:x}, {}) original={} rust={}", person, kind as u8, r, n));
    r
}

/// Comparer for `reset_morale_state`.
pub(crate) unsafe extern "thiscall" fn compare_reset_morale_state(this: usize, person: usize, club: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_RESET_MORALE_STATE);
    let regions = morale_reset_regions(this, read_usize(this));
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, person, club), || reset_morale_state(this, person, club));
    SCOREBOARD.record(igual, || format!("reset_morale_state(person {}) {:x?} x {:x?}", read_usize(person), mo, mr));
}

/// Comparer for `init_from_current_contract`.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_init_from_current_contract(this: usize, person: usize, club: usize, end: usize, percentage: u32, without_promotion: u32, multa: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32)>(IDX_INIT_FROM_CURRENT_CONTRACT);
    if exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person) == 0 {
        return o(this, person, club, end, percentage, without_promotion, multa);
    }
    let regions = morale_reset_regions(this, read_usize(person));
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&regions,
        || o(this, person, club, end, percentage, without_promotion, multa),
        || init_from_current_contract(this, person, club, end, percentage, without_promotion, multa));
    SCOREBOARD.record(igual, || {
        format!("init_from_current_contract(person {}, percentage {}, {}, {}) {:x?} x {:x?}", read_usize(person), percentage as u8 as i8, without_promotion as u8, multa as u8, mo, mr)
    });
}

/// Comparer for `get_expiry_band`.
pub(crate) unsafe extern "thiscall" fn compare_get_expiry_band(this: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u8>(IDX_GET_EXPIRY_BAND)(this);
    let n = get_expiry_band(this);
    SCOREBOARD.record(r == n, || format!("get_expiry_band(person {}) original={} rust={}", read_usize(this), r, n));
    r
}

/// Comparer for `clear_all_grievances`.
pub(crate) unsafe extern "thiscall" fn compare_clear_all_grievances(this: usize, check: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_CLEAR_ALL_GRIEVANCES);
    if check != 0 {
        return o(this, check);
    }
    let mut reg = vec![(this + contract::GRIEVANCES, 4)];
    if let Some(c) = database_staff_state_record(staff_of_contract(Contract(this))) {
        reg.push((c, 79));
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(this, check), || clear_all_grievances(this, check));
    SCOREBOARD.record(igual, || format!("clear_all_grievances({:x}) {:x?} x {:x?}", this, mo, mr));
}

/// Comparer for `check_transfer_request_withdrawal`.
pub(crate) unsafe extern "thiscall" fn compare_check_transfer_request_withdrawal(this: usize, ctx: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_CHECK_TRANSFER_REQUEST_WITHDRAWAL);
    let sheet = read_u32(G_CONTRACT_LOOKUP_RESULT);
    let retira = should_withdraw_transfer_request(Contract(this)).is_some();
    write_u32(G_CONTRACT_LOOKUP_RESULT, sheet);
    if retira {
        return o(this, ctx);
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&[(G_CONTRACT_LOOKUP_RESULT, 4), (this, 0x50)], || o(this, ctx), || check_transfer_request_withdrawal(this, ctx));
    SCOREBOARD.record(igual, || format!("check_transfer_request_withdrawal({:x}, {}) {:x?} x {:x?}", this, ctx, mo, mr));
}

/// Comparer for `set_flags`.
pub(crate) unsafe extern "thiscall" fn compare_set_flags(c: usize, marks: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_SET_FLAGS);
    if marks as i32 > 0x1000 {
        return o(c, marks);
    }
    let mut regions = vec![(c + contract::FLAGS, 4)];
    regions.extend(crate::rng::STATE_REGIONS);
    let before = crate::comparison::MemorySnapshot::read(&regions);
    MISS_TRAINING_DRY_RUN.store(true, Relaxed);
    MISS_TRAINING_DECISION.store(0, Relaxed);
    crate::rng::pause_trace(|| crate::rng::without_counting(|| set_flags(c, marks)));
    MISS_TRAINING_DRY_RUN.store(false, Relaxed);
    let pediu = MISS_TRAINING_DECISION.load(Relaxed) != 0;
    let mr = crate::comparison::MemorySnapshot::read(&regions);
    before.write_back();
    o(c, marks);
    let mo = crate::comparison::MemorySnapshot::read(&regions);
    let same = if pediu { mo.0[0] == mr.0[0] } else { mo == mr };
    SCOREBOARD.record(same, || format!("set_flags({:#x}, {:#x}) pediu={} original={:x?} rust={:x?}", c, marks, pediu, mo, mr));
}

/// Comparer for `get_contract_satisfaction`.
pub(crate) unsafe extern "thiscall" fn compare_get_contract_satisfaction(this: usize, person: usize) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_GET_CONTRACT_SATISFACTION);
    let mut regions = vec![(G_CONTRACT_LOOKUP_RESULT, 4usize), (this, 0x50)];
    regions.extend(crate::rng::STATE_REGIONS);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(
        &regions,
        || o(this, person),
        || crate::rng::without_counting(|| get_contract_satisfaction(this, person)),
    );
    SCOREBOARD.record(igual && r == n, || {
        format!("get_contract_satisfaction(person {:#x}) original={} {:x?} rust={} {:x?}", if person != 0 { read_usize(person) } else { 0 }, r, mo, n, mr)
    });
    r
}

/// Comparer for `set_default_wage`.
pub(crate) unsafe extern "thiscall" fn compare_set_default_wage(this: usize, person: usize) -> u32 {
    let this = Contract(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_SET_DEFAULT_WAGE);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(this.addr() + contract::WAGE, 4)], || o(this.addr(), person), || set_default_wage(this.addr(), person));
    SCOREBOARD.record(igual && r == n, || format!("set_default_wage({:x}) original={} rust={} {:x?} x {:x?}", person, r, n, mo, mr));
    r
}

/// Comparer for `construct_staff_contract`.
pub(crate) unsafe extern "thiscall" fn compare_construct_staff_contract(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_CONSTRUCT_STAFF_CONTRACT);
    let (ro, rr, igual, mo, mr) = crate::comparison::side_by_side(&[(this, 0x4f)], || o(this), || construct_staff_contract(this));
    SCOREBOARD.record(igual && ro == rr, || format!("construct_staff_contract({:#x}) original={:x?} rust={:x?}", this, mo, mr));
    ro
}
