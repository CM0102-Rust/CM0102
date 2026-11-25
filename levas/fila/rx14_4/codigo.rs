// ---------- rx14_4: cup and competition scalar deleting destructors ----------

/// The scalar deleting destructor of the Serie C Italian Cup: `ItaCCup::~ItaCCup` (0x63ceb0),
/// called through its detour, and, with bit 0 of `flags`, the object (`delete_through`). Returns the object.
/// Original: `ItaCCup::scalar_deleting_destructor` (0x63ce90, ita_c_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_ita_c_cup(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x63ceb0)
}

/// The scalar deleting destructor of the Italian Cup: `ItaCup::~ItaCup` (0x63eb70),
/// called through its detour, and, with bit 0 of `flags`, the object (`delete_through`). Returns the object.
/// Original: `ItaCup::scalar_deleting_destructor` (0x63eb50, ita_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_ita_cup(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x63eb70)
}

/// The scalar deleting destructor of the Italian Super Cup: `ItaSuper::~ItaSuper` (0x65f850),
/// called through its detour, and, with bit 0 of `flags`, the object (`delete_through`). Returns the object.
/// Original: `ItaSuper::scalar_deleting_destructor` (0x65f830, ita_super.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_ita_super(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x65f850)
}

/// The scalar deleting destructor of the Emperor's Cup: `JapEmpCup::~JapEmpCup` (0x661f20),
/// called through its detour, and, with bit 0 of `flags`, the object (`delete_through`). Returns the object.
/// Original: `JapEmpCup::scalar_deleting_destructor` (0x661f00, jap_emp_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_jap_emp_cup(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x661f20)
}

/// The scalar deleting destructor of the J League Cup: `JapJCup::~JapJCup` (0x665570),
/// called through its detour, and, with bit 0 of `flags`, the object (`delete_through`). Returns the object.
/// Original: `JapJCup::scalar_deleting_destructor` (0x665550, jap_j_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_jap_j_cup(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x665570)
}

/// The scalar deleting destructor of the Japanese Super Cup: `JapSuper::~JapSuper` (0x665f50),
/// called through its detour, and, with bit 0 of `flags`, the object (`delete_through`). Returns the object.
/// Original: `JapSuper::scalar_deleting_destructor` (0x665f30, jap_super.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_jap_super(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x665f50)
}

/// The scalar deleting destructor of the Korean FA Cup: `KorFaCup::~KorFaCup` (0x66a650),
/// called through its detour, and, with bit 0 of `flags`, the object (`delete_through`). Returns the object.
/// Original: `KorFaCup::scalar_deleting_destructor` (0x66a630, kor_fa_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_kor_fa_cup(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x66a650)
}

/// The scalar deleting destructor of the Korean League Cup: `KorLeagueCup::~KorLeagueCup` (0x66c160),
/// called through its detour, and, with bit 0 of `flags`, the object (`delete_through`). Returns the object.
/// Original: `KorLeagueCup::scalar_deleting_destructor` (0x66c140, kor_league_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_kor_league_cup(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x66c160)
}

/// Comparers of the scalar deleting destructors: they free memory (they cannot run twice); only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_delete_ita_c_cup(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I0%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_ita_cup(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I1%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_ita_super(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I2%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_jap_emp_cup(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I3%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_jap_j_cup(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I4%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_jap_super(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I5%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_kor_fa_cup(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I6%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_kor_league_cup(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I7%%)(this, flags)
}
