// ---------- cx09_1: the destructors of twelve leagues: Belgium, Brazil, England, Finland and Croatia ----------

/// The body of the destructors of the leagues without stage objects (thiscall, ret 0; SEH frame only for the base): the
/// class vtable back, the table freed (`exe::league_free_table` with 0), the round dates freed and zeroed
/// (`ROUND_DATES`), the statistics destroyed (left in place), then League::~League (`exe::drop_league`).
unsafe fn drop_plain_league(this: usize, vtable: usize) {
    let league = League(this);
    covered!("cx09_1 plain league destroyed");
    write_usize(this, vtable);
    exe::league_free_table(this, 0);
    if league.round_dates() != 0 {
        exe::crt_free(league.round_dates());
        league.set_round_dates(0);
    }
    delete_comp_stats(league.comp_stats());
    exe::drop_league(this);
}

/// Destroys the Belgian Third Division (`drop_group_cup` without an entrant list).
/// Original: `BelThird::~BelThird` (0x421940, bel_third.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_bel_third(this: usize) {
    drop_group_cup(this, 0x967b48, false);
}

/// Destroys the Brazilian Serie A (`drop_group_cup` without an entrant list).
/// Original: `BraNatFirst::~BraNatFirst` (0x429a90, bra_nat_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_bra_nat_first(this: usize) {
    drop_group_cup(this, 0x967d98, false);
}

/// Destroys the Brazilian Serie B (`drop_group_cup` without an entrant list).
/// Original: `BraNatSecond::~BraNatSecond` (0x42c080, bra_nat_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_bra_nat_second(this: usize) {
    drop_group_cup(this, 0x967e4c, false);
}

/// Destroys the Football Conference (`drop_group_cup` without an entrant list).
/// Original: `EngConf::~EngConf` (0x56dda0, eng_conf.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_eng_conf(this: usize) {
    drop_group_cup(this, 0x969a74, false);
}

/// Destroys the English First Division (`drop_group_cup` without an entrant list).
/// Original: `EngFirst::~EngFirst` (0x5718c0, eng_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_eng_first(this: usize) {
    drop_group_cup(this, 0x969c68, false);
}

/// Destroys the English Second Division (`drop_group_cup` without an entrant list).
/// Original: `EngSecond::~EngSecond` (0x5756e0, eng_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_eng_second(this: usize) {
    drop_group_cup(this, 0x969dd0, false);
}

/// Destroys the English Third Division (`drop_group_cup` without an entrant list).
/// Original: `EngThird::~EngThird` (0x577020, eng_third.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_eng_third(this: usize) {
    drop_group_cup(this, 0x969e84, false);
}

/// Destroys the Finnish First Division (`drop_group_cup` without an entrant list).
/// Original: `FinFirst::~FinFirst` (0x593100, fin_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_fin_first(this: usize) {
    drop_group_cup(this, 0x96a400, false);
}

/// Destroys the Brazilian Serie C: the pool of the clubs for the next stage (+0xee) freed (left in place), then the body of the
/// group cups (`drop_group_cup` without an entrant list; the original writes the vtable before the pool, which
/// changes nothing seen outside).
/// Original: `BraNatThird::~BraNatThird` (0x42d540, bra_nat_third.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_bra_nat_third(this: usize) {
    covered!("cx09_1 serie c destroyed");
    // the pool of the clubs for the next stage (+0xee), freed first and left in place
    let pool = read_usize(this + 0xee);
    if pool != 0 {
        exe::crt_free(pool);
    }
    drop_group_cup(this, 0x967f00, false);
}

/// Destroys the Croatian First Division (`drop_plain_league`).
/// Original: `CroA1::~CroA1` (0x514050, cro_a1.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_cro_a1(this: usize) {
    drop_plain_league(this, 0x9690c4);
}

/// Destroys the Croatian Second Division group A (`drop_plain_league`).
/// Original: `CroA2a::~CroA2a` (0x515550, cro_a2a.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_cro_a2a(this: usize) {
    drop_plain_league(this, 0x969178);
}

/// Destroys the Croatian Second Division group B (`drop_plain_league`).
/// Original: `CroA2b::~CroA2b` (0x516670, cro_a2b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_cro_a2b(this: usize) {
    drop_plain_league(this, 0x96922c);
}

/// Comparers of the destructors: they free memory (they cannot run twice); only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_drop_bel_third(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_bra_nat_first(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_bra_nat_second(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_eng_conf(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I3%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_eng_first(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I4%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_eng_second(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I5%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_eng_third(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I6%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_fin_first(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I7%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_bra_nat_third(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I8%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_cro_a1(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I9%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_cro_a2a(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I10%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_cro_a2b(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I11%%)(this)
}
