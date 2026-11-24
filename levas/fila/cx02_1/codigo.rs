// ---------- cx02_1: the cup destructors (1) ----------

/// Destroys the Danish Cup (`drop_national_cup`).
/// Original: `DenCup::~DenCup` (0x550ba0, den_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_den_cup(this: usize) {
    drop_national_cup(this, 0x969590);
}

/// Destroys the League Cup (`drop_national_cup`).
/// Original: `EngCcCup::~EngCcCup` (0x56c4a0, eng_cc_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_eng_cc_cup(this: usize) {
    drop_national_cup(this, 0x969934);
}

/// Destroys the Charity Shield (`drop_national_cup`).
/// Original: `EngCharity::~EngCharity` (0x56d570, eng_charity.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_eng_charity(this: usize) {
    drop_national_cup(this, 0x9699d4);
}

/// Destroys the FA Cup (`drop_national_cup`).
/// Original: `EngFaCup::~EngFaCup` (0x56f250, eng_fa_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_eng_fa_cup(this: usize) {
    drop_national_cup(this, 0x969b28);
}

/// Destroys the FA Trophy (`drop_national_cup`).
/// Original: `EngFaTrophy::~EngFaTrophy` (0x570df0, eng_fa_trophy.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_eng_fa_trophy(this: usize) {
    drop_national_cup(this, 0x969bc8);
}

/// Destroys the Finnish Cup (`drop_national_cup`).
/// Original: `FinCup::~FinCup` (0x592440, fin_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_fin_cup(this: usize) {
    drop_national_cup(this, 0x96a360);
}

/// Destroys the Coupe de France (`drop_national_cup`).
/// Original: `FraCup::~FraCup` (0x5ba320, fra_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_fra_cup(this: usize) {
    drop_national_cup(this, 0x96a864);
}

/// Destroys the French League Cup (`drop_national_cup`).
/// Original: `FraLgeCup::~FraLgeCup` (0x5bcb70, fra_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_fra_lge_cup(this: usize) {
    drop_national_cup(this, 0x96aad8);
}

/// Destroys the French Super Cup (`drop_national_cup`).
/// Original: `FraSuper::~FraSuper` (0x5bf500, fra_super.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_fra_super(this: usize) {
    drop_national_cup(this, 0x96ace0);
}

/// Destroys the German Cup (`drop_national_cup`).
/// Original: `GerCup::~GerCup` (0x5d8e10, ger_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_ger_cup(this: usize) {
    drop_national_cup(this, 0x96b044);
}

/// Destroys the German League Cup (`drop_national_cup`).
/// Original: `GerLgeCup::~GerLgeCup` (0x5dc2d0, ger_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_ger_lge_cup(this: usize) {
    drop_national_cup(this, 0x96b1b0);
}

/// Destroys the Greek Super Cup (`drop_national_cup`).
/// Original: `GreSuper::~GreSuper` (0x5ec220, gre_super.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_gre_super(this: usize) {
    drop_national_cup(this, 0x96b6d0);
}

/// Comparers of the destructors: they free memory (they cannot run twice); only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_drop_den_cup(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_eng_cc_cup(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_eng_charity(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_eng_fa_cup(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I3%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_eng_fa_trophy(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I4%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_fin_cup(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I5%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_fra_cup(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I6%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_fra_lge_cup(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I7%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_fra_super(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I8%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_ger_cup(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I9%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_ger_lge_cup(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I10%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_gre_super(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I11%%)(this)
}
