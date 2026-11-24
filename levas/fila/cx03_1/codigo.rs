// ---------- cx03_1: the destructors of twelve national cups ----------

/// Destroys the Polish Cup (`drop_national_cup`).
/// Original: `PolCup::~PolCup` (0x7c7f70, pol_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_pol_cup(this: usize) {
    drop_national_cup(this, 0x96e2c0);
}

/// Destroys the Polish League Cup (`drop_national_cup`).
/// Original: `PolLgeCup::~PolLgeCup` (0x7ca6d0, pol_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_pol_lge_cup(this: usize) {
    drop_national_cup(this, 0x96e414);
}

/// Destroys the Polish Super Cup (`drop_national_cup`).
/// Original: `PolSuper::~PolSuper` (0x7ccb00, pol_super.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_pol_super(this: usize) {
    drop_national_cup(this, 0x96e568);
}

/// Destroys the Portuguese Cup (`drop_national_cup`).
/// Original: `PorCup::~PorCup` (0x7cdb70, por_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_por_cup(this: usize) {
    drop_national_cup(this, 0x96e650);
}

/// Destroys the Portuguese Super Cup (`drop_national_cup`).
/// Original: `PorSuper::~PorSuper` (0x7d3020, por_super.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_por_super(this: usize) {
    drop_national_cup(this, 0x96e90c);
}

/// Destroys the Scottish Challenge Cup (`drop_national_cup`).
/// Original: `ScoChalCup::~ScoChalCup` (0x7ecb70, sco_chal_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_sco_chal_cup(this: usize) {
    drop_national_cup(this, 0x96f374);
}

/// Destroys the Scottish Cup (`drop_national_cup`).
/// Original: `ScoFaCup::~ScoFaCup` (0x7ed500, sco_fa_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_sco_fa_cup(this: usize) {
    drop_national_cup(this, 0x96f414);
}

/// Destroys the Scottish League Cup (`drop_national_cup`).
/// Original: `ScoLgeCup::~ScoLgeCup` (0x7efbe0, sco_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_sco_lge_cup(this: usize) {
    drop_national_cup(this, 0x96f568);
}

/// Destroys the Spanish Cup (`drop_national_cup`).
/// Original: `SpaCup::~SpaCup` (0x84d150, spa_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_spa_cup(this: usize) {
    drop_national_cup(this, 0x96f9c8);
}

/// Destroys the Spanish Super Cup (`drop_national_cup`).
/// Original: `SpaSuper::~SpaSuper` (0x858890, spa_super.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_spa_super(this: usize) {
    drop_national_cup(this, 0x96fd38);
}

/// Destroys the Turkish Cup (`drop_national_cup`).
/// Original: `TurCup::~TurCup` (0x8fc5a0, tur_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_tur_cup(this: usize) {
    drop_national_cup(this, 0x97048c);
}

/// Destroys the MLS All-Star Game (`drop_national_cup`).
/// Original: `UsaMlsAllStars::~UsaMlsAllStars` (0x90a330, usa_mls_all_stars.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_usa_mls_all_stars(this: usize) {
    drop_national_cup(this, 0x9708f8);
}

/// The comparers of the destructors: they free memory (cannot run twice); only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_drop_pol_cup(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_pol_lge_cup(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_pol_super(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_por_cup(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I3%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_por_super(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I4%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_sco_chal_cup(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I5%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_sco_fa_cup(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I6%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_sco_lge_cup(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I7%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_spa_cup(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I8%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_spa_super(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I9%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_tur_cup(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I10%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_usa_mls_all_stars(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I11%%)(this)
}
