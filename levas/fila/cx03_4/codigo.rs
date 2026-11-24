// ---------- cx03_4: twelve cup scalar deleting destructors ----------

/// The scalar deleting destructor of the Spanish Super Cup: `SpaSuper::~SpaSuper` and, with bit 0 of `flags`, the object (`delete_through`).
/// Original: `SpaSuper::scalar_deleting_destructor` (0x858870, spa_super.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_spa_super(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x858890)
}

/// The scalar deleting destructor of the Turkish Cup: `TurCup::~TurCup` and, with bit 0 of `flags`, the object (`delete_through`).
/// Original: `TurCup::scalar_deleting_destructor` (0x8fc580, tur_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_tur_cup(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x8fc5a0)
}

/// The scalar deleting destructor of the MLS All-Star Game: `UsaMlsAllStars::~UsaMlsAllStars` and, with bit 0 of `flags`, the object (`delete_through`).
/// Original: `UsaMlsAllStars::scalar_deleting_destructor` (0x90a310, usa_mls_all_stars.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_usa_mls_all_stars(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x90a330)
}

/// The scalar deleting destructor of the US Open Cup: `UsaOpenCup::~UsaOpenCup` and, with bit 0 of `flags`, the object (`delete_through`).
/// Original: `UsaOpenCup::scalar_deleting_destructor` (0x90b080, usa_open_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_usa_open_cup(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x90b0a0)
}

/// The scalar deleting destructor of the Welsh Cup: `WelCup::~WelCup` and, with bit 0 of `flags`, the object (`delete_through`).
/// Original: `WelCup::scalar_deleting_destructor` (0x9245f0, wel_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_wel_cup(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x924610)
}

/// The scalar deleting destructor of the Asian Super Cup: `AsiaSuperCup::~AsiaSuperCup` and, with bit 0 of `flags`, the object (`delete_through`).
/// Original: `AsiaSuperCup::scalar_deleting_destructor` (0x410820, asia_super_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_asia_super_cup(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x410840)
}

/// The scalar deleting destructor of the CONCACAF Champions' Cup: `ConChamp::~ConChamp` and, with bit 0 of `flags`, the object (`delete_through`).
/// Original: `ConChamp::scalar_deleting_destructor` (0x4c0e10, con_champ.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_con_champ(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x4c0e30)
}

/// The scalar deleting destructor of the European Super Cup: `EurSuperCup::~EurSuperCup` and, with bit 0 of `flags`, the object (`delete_through`).
/// Original: `EurSuperCup::scalar_deleting_destructor` (0x57a2d0, eur_super_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_eur_super_cup(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x57a2f0)
}

/// The scalar deleting destructor of the Russian Cup: `RusCup::~RusCup` and, with bit 0 of `flags`, the object (`delete_through`).
/// Original: `RusCup::scalar_deleting_destructor` (0x7e9400, rus_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_rus_cup(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x7e9420)
}

/// The scalar deleting destructor of the Swedish Cup: `SweCup::~SweCup` and, with bit 0 of `flags`, the object (`delete_through`).
/// Original: `SweCup::scalar_deleting_destructor` (0x88ce00, swe_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_swe_cup(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x88ce20)
}

/// The scalar deleting destructor of the Asian Club Championship: `AsiaClubChamp::~AsiaClubChamp` and, with bit 0 of `flags`, the object (`delete_through`).
/// Original: `AsiaClubChamp::scalar_deleting_destructor` (0x40ad50, asia_club_champ.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_asia_club_champ(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x40ad70)
}

/// The scalar deleting destructor of the Asian Cup Winners' Cup: `AsiaCupWinner::~AsiaCupWinner` and, with bit 0 of `flags`, the object (`delete_through`).
/// Original: `AsiaCupWinner::scalar_deleting_destructor` (0x40ce50, asia_cup_winner.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_asia_cup_winner(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x40ce70)
}

/// The comparers of the destructors: they free memory (cannot run twice); only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_delete_spa_super(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I0%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_tur_cup(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I1%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_usa_mls_all_stars(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I2%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_usa_open_cup(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I3%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_wel_cup(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I4%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_asia_super_cup(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I5%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_con_champ(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I6%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_eur_super_cup(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I7%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_rus_cup(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I8%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_swe_cup(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I9%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_asia_club_champ(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I10%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_asia_cup_winner(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I11%%)(this, flags)
}
