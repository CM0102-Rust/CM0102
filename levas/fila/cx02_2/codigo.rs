// ---------- cx02_2: the cup destructors (2) ----------

/// Destroys the Dutch Super Cup (`drop_national_cup`).
/// Original: `HolSuper::~HolSuper` (0x5f8a00, hol_super.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_hol_super(this: usize) {
    drop_national_cup(this, 0x96b9d4);
}

/// Destroys the Irish Challenge Cup (`drop_national_cup`).
/// Original: `IreChalCup::~IreChalCup` (0x634290, ire_chal_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_ire_chal_cup(this: usize) {
    drop_national_cup(this, 0x96bc2c);
}

/// Destroys the Leinster Cup (`drop_national_cup`).
/// Original: `IreLeinsterCup::~IreLeinsterCup` (0x636690, ire_leinster_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_ire_leinster_cup(this: usize) {
    drop_national_cup(this, 0x96bd80);
}

/// Destroys the Munster Cup (`drop_national_cup`).
/// Original: `IreMunsterCup::~IreMunsterCup` (0x638990, ire_munster_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_ire_munster_cup(this: usize) {
    drop_national_cup(this, 0x96bed4);
}

/// Destroys the Irish President Cup (`drop_national_cup`).
/// Original: `IrePresCup::~IrePresCup` (0x639460, ire_pres_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_ire_pres_cup(this: usize) {
    drop_national_cup(this, 0x96bf74);
}

/// Destroys the Serie C1 Super Cup (`drop_national_cup`).
/// Original: `ItaC1Super::~ItaC1Super` (0x63c7c0, ita_c1_super.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_ita_c1_super(this: usize) {
    drop_national_cup(this, 0x96c1c4);
}

/// Destroys the Italian Super Cup (`drop_national_cup`).
/// Original: `ItaSuper::~ItaSuper` (0x65f850, ita_super.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_ita_super(this: usize) {
    drop_national_cup(this, 0x96c8b8);
}

/// Destroys the Emperor Cup (`drop_national_cup`).
/// Original: `JapEmpCup::~JapEmpCup` (0x661f20, jap_emp_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_jap_emp_cup(this: usize) {
    drop_national_cup(this, 0x96c9a0);
}

/// Destroys the J League Cup (`drop_national_cup`).
/// Original: `JapJCup::~JapJCup` (0x665570, jap_j_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_jap_j_cup(this: usize) {
    drop_national_cup(this, 0x96cba8);
}

/// Destroys the Japanese Super Cup (`drop_national_cup`).
/// Original: `JapSuper::~JapSuper` (0x665f50, jap_super.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_jap_super(this: usize) {
    drop_national_cup(this, 0x96cc48);
}

/// Destroys the Korean FA Cup (`drop_national_cup`).
/// Original: `KorFaCup::~KorFaCup` (0x66a650, kor_fa_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_kor_fa_cup(this: usize) {
    drop_national_cup(this, 0x96cd30);
}

/// Destroys the Korean Super Cup (`drop_national_cup`).
/// Original: `KorSuperCup::~KorSuperCup` (0x66da00, kor_super_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_kor_super_cup(this: usize) {
    drop_national_cup(this, 0x96cf38);
}

/// Comparers of the destructors: they free memory (they cannot run twice); only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_drop_hol_super(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_ire_chal_cup(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_ire_leinster_cup(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_ire_munster_cup(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I3%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_ire_pres_cup(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I4%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_ita_c1_super(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I5%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_ita_super(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I6%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_jap_emp_cup(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I7%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_jap_j_cup(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I8%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_jap_super(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I9%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_kor_fa_cup(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I10%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_kor_super_cup(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I11%%)(this)
}
