// ---------- cx16_5: the destructors of the CONCACAF, European and Oceania World Cup qualifying (wc_concacaf_cup.cpp, wc_europe_league.cpp, wc_oceania_league.cpp) ----------

/// Destroys the CONCACAF World Cup qualifying (thiscall, ret 0; the SEH frame only covers the base on the exception
/// path): its vtable back, the team, round and tie tables freed (`free_season_tables`, the pointers left), the stages
/// deleted and their vector freed and cleared (`delete_stages_and_vector`), the entrant list (+0xb2) freed when there
/// is one (left), the statistics destroyed (`delete_comp_stats`), then Cup::~Cup (`exe::drop_cup`).
/// Original: `WcConcacafCup::~WcConcacafCup` (0x914cc0, wc_concacaf_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_wc_concacaf_cup(this: usize) {
    covered!("cx16_5 concacaf qualifying destroyed");
    write_usize(this, 0x970c24); // the WcConcacafCup vtable
    free_season_tables(Cup(this));
    delete_stages_and_vector(League(this));
    let list = read_usize(this + wc_cup_turnover_consts::CUP_ENTRANT_LIST);
    if list != 0 {
        exe::crt_free(list);
    }
    delete_comp_stats(League(this).comp_stats());
    exe::drop_cup(this);
}

/// Destroys the European World Cup qualifying (thiscall, ret 0; SEH frame only for the unwinding): its vtable back, the
/// table freed (`exe::league_free_table` with 0), the round dates freed and cleared, the stages deleted and their vector
/// freed and cleared (`delete_stages_and_vector`), the list (`wc_cup_turnover_consts::EUROPE_LIST`) destroyed (LLIST::~LLIST
/// `exe::drop_llist` and `exe::crt_operator_delete`, left), the statistics destroyed (`delete_comp_stats`), the dwords
/// +0xf7 (the drawn groups) and +0xee (the entry pool) freed when there (left), then League::~League (`exe::drop_league`).
/// Original: `WcEuropeLeague::~WcEuropeLeague` (0x9184c0, wc_europe_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_wc_europe_league(this: usize) {
    covered!("cx16_5 european qualifying destroyed");
    let league = League(this);
    write_usize(this, 0x970cc4); // the WcEuropeLeague vtable
    exe::league_free_table(league, 0);
    if league.round_dates() != 0 {
        exe::crt_free(league.round_dates());
        league.set_round_dates(0);
    }
    delete_stages_and_vector(league);
    let list = read_usize(this + wc_cup_turnover_consts::EUROPE_LIST);
    if list != 0 {
        exe::drop_llist(list);
        exe::crt_operator_delete(list);
    }
    delete_comp_stats(league.comp_stats());
    for p in [read_usize(this + 0xf7), read_usize(this + 0xee)] {
        if p != 0 {
            exe::crt_free(p);
        }
    }
    exe::drop_league(this);
}

/// The Oceania section's vtable (WcOceaniaLeague).
pub(crate) const WC_OCEANIA_LEAGUE_VTABLE: usize = 0x970d78;

/// Destroys the Oceania World Cup qualifying (thiscall, ret 0; SEH frame only for the unwinding): its vtable back, the
/// table freed (`exe::league_free_table` with 0), the round dates freed and cleared, the entrant pool
/// (`wc_turnover_consts::ENTRANT_LIST`) freed when there (left), the stages deleted and their vector freed and cleared
/// (`delete_stages_and_vector`), the statistics destroyed (`delete_comp_stats`), then League::~League (`exe::drop_league`).
/// Original: `WcOceaniaLeague::~WcOceaniaLeague` (0x91e200, wc_oceania_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_wc_oceania_league(this: usize) {
    covered!("cx16_5 oceania qualifying destroyed");
    let league = League(this);
    write_usize(this, WC_OCEANIA_LEAGUE_VTABLE);
    exe::league_free_table(league, 0);
    if league.round_dates() != 0 {
        exe::crt_free(league.round_dates());
        league.set_round_dates(0);
    }
    let pool = read_usize(this + wc_turnover_consts::ENTRANT_LIST);
    if pool != 0 {
        exe::crt_free(pool);
    }
    delete_stages_and_vector(league);
    delete_comp_stats(league.comp_stats());
    exe::drop_league(this);
}

/// The scalar deleting destructor of the CONCACAF qualifying (`delete_through` with `WcConcacafCup::~WcConcacafCup`).
/// Original: `WcConcacafCup::scalar_deleting_destructor` (0x914ca0, wc_concacaf_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_wc_concacaf_cup(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x914cc0)
}

/// The scalar deleting destructor of the European qualifying (`delete_through` with `WcEuropeLeague::~WcEuropeLeague`).
/// Original: `WcEuropeLeague::scalar_deleting_destructor` (0x9184a0, wc_europe_league.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_wc_europe_league(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x9184c0)
}

/// The scalar deleting destructor of the Oceania qualifying (`delete_through` with `WcOceaniaLeague::~WcOceaniaLeague`).
/// Original: `WcOceaniaLeague::scalar_deleting_destructor` (0x91e1e0, wc_oceania_league.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_wc_oceania_league(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x91e200)
}

/// Comparers of the destructors: they free memory (they cannot run twice); only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_drop_wc_concacaf_cup(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_wc_europe_league(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_wc_oceania_league(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_wc_concacaf_cup(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I3%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_wc_europe_league(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I4%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_wc_oceania_league(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I5%%)(this, flags)
}

/// A World Cup qualifying progress text into `buf` (`FN_LANGLIB_SPRINTF`, cdecl, translated): the format alone or, for
/// a "Qualified ..." one (`comp` given), with the competition's gender byte (`competition::GENDER_NAME`, signed) and the
/// empty string 0xad9c64. Returns what the writer returned.
pub(crate) unsafe fn qualifying_progress_sprintf(buf: usize, format: usize, comp: Option<usize>) -> i32 {
    let f: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    match comp {
        Some(c) => f(buf, format, read_u8(c + competition::GENDER_NAME) as i8 as i32, 0xad9c64usize),
        None => f(buf, format),
    }
}

/// Writes a nation's CONCACAF qualifying progress into `buf`: by the round code `round` (a word) "First Round" (10),
/// "Second Round" (20), "Third Round" (30); by the stage label `stage` (a word, signed) "Central American Group Stage"
/// (0x473, 0x474); round 0x172 "Interzone Playoff"; labels 0x40c to 0x40e "Qualified For Final" with status 6, else
/// "Semi Final Group Stage"; label 0x96 "Qualified For World Cup" with status 7, else "Final Group Stage"; otherwise
/// nothing (`qualifying_progress_sprintf`). A buffer under 30 bytes shows the wc_concacaf_cup.cpp box (line 0x527) and
/// writes nothing. Returns the writer's result (0 when nothing is written; the original leaves garbage in eax). `this`
/// is not read.
/// Original: `WcConcacafCup::write_progress_text` (0x916af0, wc_concacaf_cup.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn wc_concacaf_cup_write_progress_text(_this: usize, buf: usize, size: i32, stage: u32, round: u32, status: u32, comp: usize) -> i32 {
    if size < 0x1e {
        show_error_at(wc_cup_turnover_consts::WC_CONCACAF_CUP_CPP, 0x527);
        return 0;
    }
    covered!("cx16_5 concacaf progress text");
    let (stage, round, status) = (stage as u16 as i16, round as u16, status as u8);
    let text = match round {
        0xa => 0x9a7c04,  // "First Round"
        0x14 => 0x9a7b94, // "Second Round"
        0x1e => 0x9a7b10, // "Third Round"
        _ if (0x473..=0x474).contains(&stage) => 0xad4a4c, // "Central American Group Stage"
        0x172 => 0x9a6d94, // "Interzone Playoff"
        _ if (0x40c..=0x40e).contains(&stage) => {
            // "Qualified For Final" / "Semi Final Group Stage"
            return if status == 6 { qualifying_progress_sprintf(buf, 0xad4634, Some(comp)) } else { qualifying_progress_sprintf(buf, 0xad4a34, None) };
        }
        _ if stage == 0x96 => {
            // "Qualified For World Cup" / "Final Group Stage"
            return if status == 7 { qualifying_progress_sprintf(buf, 0xad4658, Some(comp)) } else { qualifying_progress_sprintf(buf, 0xad4a20, None) };
        }
        _ => return 0,
    };
    qualifying_progress_sprintf(buf, text, None)
}

/// Writes a nation's European qualifying progress into `buf`: in a group (the stage label `stage`, a signed word, 0x3e8
/// to 0x3f5) "Qualified For World Cup" with status 7, "Playoff" with status 1, else "Group Stage"; outside, with the
/// round code 0xa0 (the play-off) "Qualified For World Cup" with status 1, else "Playoff"; otherwise nothing
/// (`qualifying_progress_sprintf`). A buffer under 30 bytes shows the wc_europe_league.cpp box (line 0x2fc). Returns the
/// writer's result (0 when nothing is written). `this` is not read.
/// Original: `WcEuropeLeague::write_progress_text` (0x9192c0, wc_europe_league.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_write_progress_text(_this: usize, buf: usize, size: i32, stage: u32, round: u32, status: u32, comp: usize) -> i32 {
    if size < 0x1e {
        show_error_at(wc_cup_turnover_consts::WC_EUROPE_LEAGUE_CPP, 0x2fc);
        return 0;
    }
    covered!("cx16_5 european progress text");
    let status = status as u8;
    if (0x3e8..=0x3f5).contains(&(stage as u16 as i16)) {
        return match status {
            7 => qualifying_progress_sprintf(buf, 0xad4658, Some(comp)), // "Qualified For World Cup"
            1 => qualifying_progress_sprintf(buf, 0x9a751c, None),       // "Playoff"
            _ => qualifying_progress_sprintf(buf, 0x99b800, None),       // "Group Stage"
        };
    }
    if round as u16 != 0xa0 {
        return 0;
    }
    if status == 1 { qualifying_progress_sprintf(buf, 0xad4658, Some(comp)) } else { qualifying_progress_sprintf(buf, 0x9a751c, None) }
}

/// Writes a nation's Oceania qualifying progress into `buf`: by the stage label `stage` (a signed word) "First Round"
/// (0x3e8, 0x3e9); label 0 (the final) "Qualified For Asia Playoff" with status 1, "Final" with 2, else "Semi Finals";
/// label 0xa0 (the play-off) "Qualified For World Cup" with status 1, else "Playoff"; otherwise nothing
/// (`qualifying_progress_sprintf`). A buffer under 30 bytes shows the wc_oceania_league.cpp box (line 0x290). Returns
/// the writer's result (0 when nothing is written). `this` and `round` are not read. Defect of the original
/// [inference]: the Oceania final's winner plays the South American section's nation, but the text says Asia.
/// Original: `WcOceaniaLeague::write_progress_text` (0x91edc0, wc_oceania_league.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn wc_oceania_league_write_progress_text(_this: usize, buf: usize, size: i32, stage: u32, _round: u32, status: u32, comp: usize) -> i32 {
    if size < 0x1e {
        show_error_at(wc_turnover_consts::WC_OCEANIA_LEAGUE_CPP, 0x290);
        return 0;
    }
    covered!("cx16_5 oceania progress text");
    let (stage, status) = (stage as u16 as i16, status as u8);
    match stage {
        0x3e8 | 0x3e9 => qualifying_progress_sprintf(buf, 0x9a7c04, None), // "First Round"
        0 => match status {
            1 => qualifying_progress_sprintf(buf, 0xad4cb4, Some(comp)), // "Qualified For Asia Playoff"
            2 => qualifying_progress_sprintf(buf, 0x9a757c, None),       // "Final"
            _ => qualifying_progress_sprintf(buf, 0x9a5cd8, None),       // "Semi Finals"
        },
        0xa0 => {
            if status == 1 { qualifying_progress_sprintf(buf, 0xad4658, Some(comp)) } else { qualifying_progress_sprintf(buf, 0x9a751c, None) }
        }
        _ => 0,
    }
}

/// Side-by-side comparer of a progress text writer: the original into the caller's buffer, the copy into a local one
/// of the same size (up to 4 KB) that starts as the caller's did; the texts must match. Below 30 bytes (the box) only
/// the original.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn compare_qualifying_progress_text(name: &str, buf: usize, size: i32, stage: u32, round: u32, status: u32, o: impl FnOnce() -> i32, c: impl FnOnce(usize, i32) -> i32) -> i32 {
    if size < 0x1e || buf == 0 {
        return o();
    }
    let n = (size as usize).min(0x1000);
    let mut local = core::slice::from_raw_parts(buf as *const u8, n).to_vec(); // the paths that write nothing leave it as it was
    let r = o();
    c(local.as_mut_ptr() as usize, n as i32);
    let text = |p: *const u8| core::ffi::CStr::from_ptr(p as *const core::ffi::c_char).to_bytes().to_vec();
    let (mo, mr) = (text(buf as *const u8), if local.contains(&0) { text(local.as_ptr()) } else { local.clone() });
    SCOREBOARD.record(mo == mr, || format!("{}(stage {:#x}, round {:#x}, status {}): original {:?} rust {:?}", name, stage as u16, round as u16, status as u8, String::from_utf8_lossy(&mo), String::from_utf8_lossy(&mr)));
    r
}

pub(crate) unsafe extern "thiscall" fn compare_wc_concacaf_cup_write_progress_text(this: usize, buf: usize, size: i32, stage: u32, round: u32, status: u32, comp: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32, u32, u32, usize) -> i32>(%%I6%%);
    compare_qualifying_progress_text("wc_concacaf_cup_write_progress_text", buf, size, stage, round, status, || o(this, buf, size, stage, round, status, comp),
        |b, n| wc_concacaf_cup_write_progress_text(this, b, n, stage, round, status, comp))
}

pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_write_progress_text(this: usize, buf: usize, size: i32, stage: u32, round: u32, status: u32, comp: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32, u32, u32, usize) -> i32>(%%I7%%);
    compare_qualifying_progress_text("wc_europe_league_write_progress_text", buf, size, stage, round, status, || o(this, buf, size, stage, round, status, comp),
        |b, n| wc_europe_league_write_progress_text(this, b, n, stage, round, status, comp))
}

pub(crate) unsafe extern "thiscall" fn compare_wc_oceania_league_write_progress_text(this: usize, buf: usize, size: i32, stage: u32, round: u32, status: u32, comp: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32, u32, u32, usize) -> i32>(%%I8%%);
    compare_qualifying_progress_text("wc_oceania_league_write_progress_text", buf, size, stage, round, status, || o(this, buf, size, stage, round, status, comp),
        |b, n| wc_oceania_league_write_progress_text(this, b, n, stage, round, status, comp))
}
