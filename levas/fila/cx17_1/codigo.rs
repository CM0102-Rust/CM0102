// ---------- cx17_1: the destructors of the South American World Cup qualifying and of the World Cup, and the South American progress text (wc_south_american_league.cpp, world_cup.cpp) ----------

/// The South American section's vtable (WcSouthAmericanLeague).
pub(crate) const WC_SOUTH_AMERICAN_LEAGUE_VTABLE: usize = 0x970e2c;

/// The World Cup's vtable (WorldCup).
pub(crate) const WORLD_CUP_VTABLE: usize = 0x9712f0;

/// Destroys the South American World Cup qualifying (thiscall, ret 0; the SEH frame only covers the base on the
/// exception path): its vtable back, the table freed (`exe::league_free_table` with 0), the round dates freed and
/// cleared, the statistics destroyed (`delete_comp_stats`, the pointer left), then League::~League (`exe::drop_league`).
/// The section has no stages of its own.
/// Original: `WcSouthAmericanLeague::~WcSouthAmericanLeague` (0x920500, wc_south_american_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_wc_south_american_league(this: usize) {
    covered!("cx17_1 south american qualifying destroyed");
    let league = League(this);
    write_usize(this, WC_SOUTH_AMERICAN_LEAGUE_VTABLE);
    exe::league_free_table(league, 0);
    if league.round_dates() != 0 {
        exe::crt_free(league.round_dates());
        league.set_round_dates(0);
    }
    delete_comp_stats(league.comp_stats());
    exe::drop_league(this);
}

/// Destroys the World Cup (thiscall, ret 0; SEH frame only for the unwinding): the body of the continental cups run on
/// the League base (`drop_stats_league`): its vtable back, the table and the round dates freed, the stages up to the
/// current one deleted and their vector freed and cleared, the statistics destroyed, then League::~League.
/// Original: `WorldCup::~WorldCup` (0x92c1d0, world_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_world_cup(this: usize) {
    covered!("cx17_1 world cup destroyed");
    drop_stats_league(this, WORLD_CUP_VTABLE);
}

/// The scalar deleting destructor of the South American qualifying (`delete_through` with
/// `WcSouthAmericanLeague::~WcSouthAmericanLeague`).
/// Original: `WcSouthAmericanLeague::scalar_deleting_destructor` (0x9204e0, wc_south_american_league.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_wc_south_american_league(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x920500)
}

/// The scalar deleting destructor of the World Cup (`delete_through` with `WorldCup::~WorldCup`).
/// Original: `WorldCup::scalar_deleting_destructor` (0x92c1b0, world_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_world_cup(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x92c1d0)
}

/// rule: a nation's progress in the South American qualifying reads "Qualified For World Cup" when its status is 7
/// (qualified) and "League Stage" otherwise
///
/// The text into `buf` (`FN_LANGLIB_SPRINTF`, translated): status byte 7, "Qualified {}<%s - Ignore>For World Cup"
/// (0xad4658) with the competition record's gender (`competition::GENDER_NAME`, signed) and the empty string; any other
/// status, "League Stage" (0xad4ddc). The round and the sub-round are not read, nor `this`. A buffer under 30 bytes
/// (signed size) shows the wc_south_american_league.cpp box (line 0x2ae) and writes nothing. Returns what the writer
/// returned.
/// Original: `WcSouthAmericanLeague::write_progress_text` (0x921390, wc_south_american_league.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn wc_south_american_league_write_progress_text(_this: usize, buf: usize, size: i32, _round: u32, _sub_round: u32, status: u32, comp: usize) -> i32 {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    if size < 0x1e {
        // simplification: after the box the original returns whatever the box left in eax; the callers do not read it
        show_error_at(wc_turnover_consts::WC_SOUTH_AMERICAN_LEAGUE_CPP, 0x2ae);
        return 0;
    }
    covered!("cx17_1 south american progress text");
    if status as u8 == 7 {
        return format(buf, 0xad4658, read_u8(comp + competition::GENDER_NAME) as i8 as i32, EMPTY_STRING); // "Qualified For World Cup"
    }
    format(buf, 0xad4ddc) // "League Stage"
}

/// Comparer for `wc_south_american_league_write_progress_text`: the original into the caller's buffer, the copy into a
/// local one of the same size (up to 4 KB) side by side; the texts must match. Below 30 bytes (the box) only the
/// original.
pub(crate) unsafe extern "thiscall" fn compare_wc_south_american_league_write_progress_text(this: usize, buf: usize, size: i32, round: u32, sub_round: u32, status: u32, comp: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32, u32, u32, usize) -> i32>(%%I4%%);
    let r = o(this, buf, size, round, sub_round, status, comp);
    if size < 0x1e || buf == 0 {
        return r;
    }
    let mut local = vec![0u8; (size as usize).min(0x1000)];
    wc_south_american_league_write_progress_text(this, local.as_mut_ptr() as usize, local.len() as i32, round, sub_round, status, comp);
    let c = |p: *const u8| core::ffi::CStr::from_ptr(p as *const core::ffi::c_char).to_bytes().to_vec();
    let (mo, mr) = (c(buf as *const u8), c(local.as_ptr()));
    SCOREBOARD.record(mo == mr, || format!("wc_south_american_league_write_progress_text(status {}): original {:?} rust {:?}", status as u8, String::from_utf8_lossy(&mo), String::from_utf8_lossy(&mr)));
    r
}

/// Comparers of the destructors: they free memory (they cannot run twice); only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_drop_wc_south_american_league(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_world_cup(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_wc_south_american_league(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I2%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_world_cup(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I3%%)(this, flags)
}
