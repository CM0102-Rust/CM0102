// ---------- batch %%LEVA%%: the competition screen opener and the season records (comp_screens.cpp) ----------

/// The competition screen's build, input and leave functions (passed to `ScreenManager::open_screen`).
pub(crate) const FN_COMP_SCREENS_DRAW_COMP_SCREEN: usize = 0x48fcd0;
pub(crate) const FN_COMP_SCREENS_HANDLE_COMP_SCREEN_CLICK: usize = 0x4965d0;
pub(crate) const FN_COMP_SCREENS_CLOSE_COMP_SCREEN: usize = 0x4985e0;

/// Opens the competition screen of a competition (`comp` points to its id, an index of `G_COMP_OBJECT_TABLE`) with the
/// two given parameters (2 and 3); with `restore` the 60 parameters of the current screen are saved first and 1, 0x1e
/// and 0x23 come back after the defaults. The parameters: 0 = the competition, 1 = 1, 2 and 3 the given ones,
/// 4..0x1d = 0, 0x23 = 3, 0x24..0x27 = 0, 0x1e = 0x11, 0x1f..0x22 = 0, 0x28 = 0x29 = 0, 0x2a = -1, 0x2b = 0, (the
/// restored ones), 0x2c = 0, 0x2d = 1, 0x2e..0x31 = 0, 0x32 = -1, 0x33..0x35 = 0, 0x36 = -1. Nothing more when the screen does not open (`ScreenManager::open_screen`
/// with the competition screen's functions, not single instance).
/// Original: `comp_screens_open_comp_screen` (0x48f4a0, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_open_comp_screen(comp: usize, second: usize, third: usize, restore: u32) {
    let mut saved = [0usize; 0x3c];
    if restore != 0 {
        for (k, v) in saved.iter_mut().enumerate() {
            *v = get_screen_param(k as u32);
        }
    }
    let object = read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_usize(comp) * 4);
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, FN_COMP_SCREENS_DRAW_COMP_SCREEN, FN_COMP_SCREENS_HANDLE_COMP_SCREEN_CLICK, 0,
        FN_COMP_SCREENS_CLOSE_COMP_SCREEN, 0) == 0 {
        return;
    }
    covered!("tel30_10 competition screen opened");
    set_screen_param(0, object);
    set_screen_param(1, 1);
    set_screen_param(2, second);
    set_screen_param(3, third);
    for k in 4..=0x1d {
        set_screen_param(k, 0);
    }
    set_screen_param(0x23, 3);
    for k in 0x24..=0x27 {
        set_screen_param(k, 0);
    }
    set_screen_param(0x1e, 0x11);
    for k in 0x1f..=0x22 {
        set_screen_param(k, 0);
    }
    set_screen_param(0x28, 0);
    set_screen_param(0x29, 0);
    set_screen_param(0x2a, usize::MAX);
    set_screen_param(0x2b, 0);
    if restore != 0 {
        set_screen_param(1, saved[1]);
        set_screen_param(0x1e, saved[0x1e]);
        set_screen_param(0x23, saved[0x23]);
    }
    set_screen_param(0x2c, 0);
    set_screen_param(0x2d, 1);
    for k in 0x2e..=0x31 {
        set_screen_param(k, 0);
    }
    set_screen_param(0x32, usize::MAX);
    for k in 0x33..=0x35 {
        set_screen_param(k, 0);
    }
    set_screen_param(0x36, usize::MAX);
}

/// `Date::Date()` (0x54a110) on `at`.
unsafe fn comp_records_date(at: usize) {
    exe::date_date_54a110(at);
}

/// Builds a competition's season records: the dates at +0x29, +0x4c, +0x75 and +0x94, the four run records at
/// +0xb3, +0xcb, +0xe3 and +0xfb (`run_record_construct` 0x448670), the date at +0x124, the record holder at +0x133
/// and the fixed fixture at +0x14b (`FixedFixture::FixedFixture` 0x645800), the record holders at +0x16b, +0x18d,
/// +0x1ad and +0x1cd, and the youngest and oldest player records at +0x1eb and +0x22a; returns the records.
/// Original: `Comp_Season_Records::Comp_Season_Records` (0x49f160, comp_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn comp_season_records_comp_season_records(this: usize) -> usize {
    for at in [0x29, 0x4c, 0x75, 0x94] {
        comp_records_date(this + at);
    }
    let run: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x448670usize);
    for at in [0xb3, 0xcb, 0xe3, 0xfb] {
        run(this + at);
    }
    comp_records_date(this + 0x124);
    exe::record_holder_construct(this + 0x133);
    let fixture: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x645800usize);
    fixture(this + 0x14b);
    for at in [0x16b, 0x18d, 0x1ad, 0x1cd] {
        exe::record_holder_construct(this + at);
    }
    comp_youngest_player_record_construct(this + 0x1eb);
    comp_oldest_player_record_construct(this + 0x22a);
    this
}

/// Builds an unbeaten run record: its two dates (+4, +0xc); returns the record.
/// Original: `comp_unbeaten_run_record_construct` (0x49f2a0, comp_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn comp_unbeaten_run_record_construct(this: usize) -> usize {
    comp_records_date(this + 4);
    comp_records_date(this + 0xc);
    this
}

/// Builds a winless run record: its two dates (+4, +0xc); returns the record.
/// Original: `comp_winless_run_record_construct` (0x49f2f0, comp_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn comp_winless_run_record_construct(this: usize) -> usize {
    comp_records_date(this + 4);
    comp_records_date(this + 0xc);
    this
}

/// Builds a goals in a match record: its two dates (+0x11, +0x19); returns the record.
/// Original: `comp_goals_in_match_record_construct` (0x49f340, comp_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn comp_goals_in_match_record_construct(this: usize) -> usize {
    comp_records_date(this + 0x11);
    comp_records_date(this + 0x19);
    this
}

/// Builds a youngest player record: its three dates (+0, +0x18, +0x20); returns the record.
/// Original: `comp_youngest_player_record_construct` (0x49f390, comp_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn comp_youngest_player_record_construct(this: usize) -> usize {
    comp_records_date(this);
    comp_records_date(this + 0x18);
    comp_records_date(this + 0x20);
    this
}

/// Builds an oldest player record: its three dates (+0, +0x18, +0x20); returns the record.
/// Original: `comp_oldest_player_record_construct` (0x49f3f0, comp_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn comp_oldest_player_record_construct(this: usize) -> usize {
    comp_records_date(this);
    comp_records_date(this + 0x18);
    comp_records_date(this + 0x20);
    this
}

/// Copies a tally record: the byte at +0, the record holder at +1 (`record_holder_assign`), the dword at +0x19 and the
/// word at +0x1d; returns the destination.
/// Original: `comp_tally_record_assign` (0x4a1f40, comp_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn comp_tally_record_assign(this: usize, src: usize) -> usize {
    write_u8(this, read_u8(src));
    exe::record_holder_assign(this + 1, src + 1);
    write_u32(this + 0x19, read_u32(src + 0x19));
    write_u16(this + 0x1d, read_u16(src + 0x1d));
    this
}

/// Copies a discipline record: the bytes at +0 and +1, the record holder at +2, the dword at +0x1a and the word at
/// +0x1e; returns the destination.
/// Original: `comp_discipline_record_assign` (0x4a1f70, comp_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn comp_discipline_record_assign(this: usize, src: usize) -> usize {
    write_u8(this, read_u8(src));
    write_u8(this + 1, read_u8(src + 1));
    exe::record_holder_assign(this + 2, src + 2);
    write_u32(this + 0x1a, read_u32(src + 0x1a));
    write_u16(this + 0x1e, read_u16(src + 0x1e));
    this
}

/// Copies a rating record: the dword at +0, the record holder at +4, the byte at +0x1c, the dword at +0x1d and the
/// word at +0x21; returns the destination.
/// Original: `comp_rating_record_assign` (0x4a2050, comp_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn comp_rating_record_assign(this: usize, src: usize) -> usize {
    write_u32(this, read_u32(src));
    exe::record_holder_assign(this + 4, src + 4);
    write_u8(this + 0x1c, read_u8(src + 0x1c));
    write_u32(this + 0x1d, read_u32(src + 0x1d));
    write_u16(this + 0x21, read_u16(src + 0x21));
    this
}

// simplification: opens a screen (the history allocates a node): it does not run twice. Original only; the plan: the
// competition screen current with the competition in parameter 0 and the caller's two parameters.
/// Comparer for `comp_screens_open_comp_screen`.
pub(crate) unsafe extern "C" fn compare_comp_screens_open_comp_screen(comp: usize, second: usize, third: usize, restore: u32) {
    original_fn::<unsafe extern "C" fn(usize, usize, usize, u32)>(%%I0%%)(comp, second, third, restore);
    let node = current_screen_node(G_SCREEN_MANAGER);
    if node == 0 || read_usize(node) != FN_COMP_SCREENS_DRAW_COMP_SCREEN {
        return;
    }
    let object = read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_usize(comp) * 4);
    let got = [get_screen_param(0), get_screen_param(2), get_screen_param(3), get_screen_param(0x2d), get_screen_param(0x36)];
    let want = [object, second, third, 1, usize::MAX];
    SCOREBOARD.record(got == want, || format!("comp_screens_open_comp_screen (competition screen): params original={:?} rust={:?}", got, want));
}

/// A record routine side by side over the object it builds or copies.
unsafe fn comp_records_compare(name: &str, at: usize, size: usize, original: impl FnOnce() -> usize, rust: impl FnOnce() -> usize) -> usize {
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&[(at, size)], original, rust);
    SCOREBOARD.record(same && r == n, || format!("{}({:#x}): differences {:?}", name, at, first_differences(&mo, &mr)));
    r
}

/// Comparer for `comp_season_records_comp_season_records`: side by side over the 0x252 bytes it builds.
pub(crate) unsafe extern "thiscall" fn compare_comp_season_records_comp_season_records(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I1%%);
    comp_records_compare("Comp_Season_Records::Comp_Season_Records", this, 0x252, || o(this), || comp_season_records_comp_season_records(this))
}

/// Comparer for `comp_unbeaten_run_record_construct`: side by side over its two dates.
pub(crate) unsafe extern "thiscall" fn compare_comp_unbeaten_run_record_construct(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I2%%);
    comp_records_compare("comp_unbeaten_run_record_construct", this, 0x14, || o(this), || comp_unbeaten_run_record_construct(this))
}

/// Comparer for `comp_winless_run_record_construct`: side by side over its two dates.
pub(crate) unsafe extern "thiscall" fn compare_comp_winless_run_record_construct(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I3%%);
    comp_records_compare("comp_winless_run_record_construct", this, 0x14, || o(this), || comp_winless_run_record_construct(this))
}

/// Comparer for `comp_goals_in_match_record_construct`: side by side over its two dates.
pub(crate) unsafe extern "thiscall" fn compare_comp_goals_in_match_record_construct(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I4%%);
    comp_records_compare("comp_goals_in_match_record_construct", this, 0x21, || o(this), || comp_goals_in_match_record_construct(this))
}

/// Comparer for `comp_youngest_player_record_construct`: side by side over its three dates.
pub(crate) unsafe extern "thiscall" fn compare_comp_youngest_player_record_construct(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I5%%);
    comp_records_compare("comp_youngest_player_record_construct", this, 0x28, || o(this), || comp_youngest_player_record_construct(this))
}

/// Comparer for `comp_oldest_player_record_construct`: side by side over its three dates.
pub(crate) unsafe extern "thiscall" fn compare_comp_oldest_player_record_construct(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I6%%);
    comp_records_compare("comp_oldest_player_record_construct", this, 0x28, || o(this), || comp_oldest_player_record_construct(this))
}

/// Comparer for `comp_tally_record_assign`: side by side over the 0x1f bytes copied.
pub(crate) unsafe extern "thiscall" fn compare_comp_tally_record_assign(this: usize, src: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I7%%);
    comp_records_compare("comp_tally_record_assign", this, 0x1f, || o(this, src), || comp_tally_record_assign(this, src))
}

/// Comparer for `comp_discipline_record_assign`: side by side over the 0x20 bytes copied.
pub(crate) unsafe extern "thiscall" fn compare_comp_discipline_record_assign(this: usize, src: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I8%%);
    comp_records_compare("comp_discipline_record_assign", this, 0x20, || o(this, src), || comp_discipline_record_assign(this, src))
}

/// Comparer for `comp_rating_record_assign`: side by side over the 0x23 bytes copied.
pub(crate) unsafe extern "thiscall" fn compare_comp_rating_record_assign(this: usize, src: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I9%%);
    comp_records_compare("comp_rating_record_assign", this, 0x23, || o(this, src), || comp_rating_record_assign(this, src))
}
