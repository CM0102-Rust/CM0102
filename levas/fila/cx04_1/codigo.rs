// ---------- cx04_1: the continental cups' destructors, the FIFA ranking table's and the file list's ----------

/// Destroys a continental competition run on the League base: its vtable back, the table freed (`exe::league_free_table`
/// with 0), the round dates freed and cleared, every stage up to the current one destroyed (virtual slot 0 with 1), the
/// stage vector freed and cleared, the statistics destroyed (left as they are), then League::~League
/// (`exe::drop_league`). The same body as the Brazilian Champions Cup's (`drop_bra_champ_cup`).
pub(crate) unsafe fn drop_stats_league(this: usize, vtable: usize) {
    let l = League(this);
    write_usize(this, vtable);
    exe::league_free_table(this, 0);
    if l.round_dates() != 0 {
        exe::crt_free(l.round_dates());
        l.set_round_dates(0);
    }
    let mut k = 0i32;
    while k <= l.current_stage() as i32 {
        let stage = read_usize(l.stages() + 4 * k as usize);
        if stage != 0 {
            let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(stage)));
            destroy(stage, 1);
        }
        k += 1;
    }
    if l.stages() != 0 {
        exe::crt_free(l.stages());
        l.set_stages(0);
    }
    delete_comp_stats(l.comp_stats());
    exe::drop_league(this);
}

/// Destroys the Champions League: the vtable 0x96a1d0 back, the team table and the round table freed, each round's tie
/// array (`cup::TIE_TABLES`, `cup::ROUND_COUNT` of them) and the array freed (all left as they are); then every stage
/// up to the current one (`cup::CURRENT_STAGE`) destroyed (virtual slot 0 with 1) and the stage vector freed and
/// cleared; the statistics destroyed; then the Cup base (`exe::drop_cup`). Unlike the national cups
/// (`drop_national_cup`), the Champions League owns its group stages.
/// Original: `EuropeanCup::~EuropeanCup` (0x583a00, european_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_european_cup(this: usize) {
    let cup = Cup(this);
    write_usize(this, 0x96a1d0); // the Champions League vtable
    for table in [cup.team_table(), cup.round_table()] {
        if table != 0 {
            exe::crt_free(table);
        }
    }
    let ties = cup.tie_tables();
    if ties != 0 {
        let mut r = 0i32;
        while r < cup.round_count() as i32 {
            let round = read_usize(ties + 4 * r as usize);
            if round != 0 {
                exe::crt_free(round);
            }
            r += 1;
        }
        exe::crt_free(cup.tie_tables());
    }
    let mut k = 0i32;
    while k <= cup.current_stage() as i32 {
        let stage = read_usize(cup.stages() + 4 * k as usize);
        if stage != 0 {
            let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(stage)));
            destroy(stage, 1);
        }
        k += 1;
    }
    if cup.stages() != 0 {
        exe::crt_free(cup.stages());
        cup.set_stages(0);
    }
    delete_comp_stats(League(this).comp_stats());
    exe::drop_cup(this);
}

/// Destroys the Inter-American Cup (`drop_national_cup`).
/// Original: `InterAmerCup::~InterAmerCup` (0x632270, inter_amer_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_inter_amer_cup(this: usize) {
    drop_national_cup(this, 0x96bad8);
}

/// Destroys the Intertoto Cup (`drop_national_cup`).
/// Original: `IntertotoCup::~IntertotoCup` (0x6331c0, intertoto_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_intertoto_cup(this: usize) {
    drop_national_cup(this, INTERTOTO_CUP_VTABLE);
}

/// Destroys the UEFA Cup (`drop_national_cup`).
/// Original: `UefaCup::~UefaCup` (0x9024f0, uefa_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_uefa_cup(this: usize) {
    drop_national_cup(this, UEFA_CUP_VTABLE);
}

/// Destroys the Intercontinental Cup (`drop_national_cup`).
/// Original: `WorldClubCup::~WorldClubCup` (0x92b6a0, world_club_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_world_club_cup(this: usize) {
    drop_national_cup(this, 0x971250);
}

/// Destroys the Oceania Club Championship (`drop_stats_league`).
/// Original: `OceaniaClubChamp::~OceaniaClubChamp` (0x797170, oceania_club_champ.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_oceania_club_champ(this: usize) {
    drop_stats_league(this, 0x96e018);
}

/// Destroys the Club World Championship (`drop_stats_league`).
/// Original: `WorldClubChamp::~WorldClubChamp` (0x9292d0, world_club_champ.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_world_club_champ(this: usize) {
    drop_stats_league(this, 0x97119c);
}

/// Destroys the FIFA ranking table: with a table (+0), each nation's results history (`fifa_ranking_entry::HISTORY`,
/// `G_NATION_COUNT` entries of 0x2c bytes) freed when present, then the table; nothing is cleared.
/// Original: `FIFA_RANKINGS::~FIFA_RANKINGS` (0x58cf20, fifa_rankings.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_fifa_rankings(this: usize) {
    let table = read_usize(this);
    if table == 0 {
        return;
    }
    let mut i = 0i32;
    while i < read_i32(G_NATION_COUNT) {
        let history = read_usize(read_usize(this) + i as usize * 0x2c + fifa_ranking_entry::HISTORY);
        if history != 0 {
            exe::crt_free(history);
        }
        i += 1;
    }
    exe::crt_free(read_usize(this));
}

/// Destroys a file-backed list: the backing file path (+0) that the constructor copied is freed when present.
/// Original: `FILE_LLIST::~FILE_LLIST` (0x58e330, fifa_rankings.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_file_llist(this: usize) {
    let path = read_usize(this);
    if path != 0 {
        exe::crt_free(path);
    }
}

/// Writes the name of a club's progress in the Champions League into `buf`: in a group stage of phase 1 or 2 (the parent
/// stage of `round`, `exe::comp_util_get_parent_stage_id`, 0x449 or 0x44a), "Qualified From Phase 1/2" (with the
/// competition's name gender, byte +0x37 of `comp`) when `status` is 0, 6, 7 or 8, otherwise "Phase 1/2 Group Stage";
/// elsewhere "Winners" for status 1; with no round nor sub-round, "Runners Up" for status 2 and "Third Placed" for
/// status 3; otherwise the stage and round names (`exe::comp_util_format_stage_round_name`(round, sub_round, buf, size,
/// 0)). The texts are translated (`FN_LANGLIB_SPRINTF`). A buffer under 30 bytes shows the european_cup.cpp box (line
/// 0x950) and writes nothing. Returns what the writer returned (the original leaves it in eax; 0 after the box).
/// Original: `EuropeanCup::write_progress_text` (0x586fa0, european_cup.cpp, thiscall, ret 0x18; `this` is not used).
pub(crate) unsafe extern "thiscall" fn european_cup_write_progress_text(_this: usize, buf: usize, size: i32, round: u32, sub_round: u32, status: u32, comp: usize) -> i32 {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    if size < 0x1e {
        show_error_at(0x9c4900, 0x950); // european_cup.cpp
        return 0;
    }
    let (round, sub_round, status) = (round as u16 as i16 as i32, sub_round as u16 as i16 as i32, status as u8);
    let qualified = matches!(status, 0 | 6 | 7 | 8);
    if exe::comp_util_get_parent_stage_id(round) == 0x449 {
        covered!("cx04_1 champions league phase 1 progress");
        // the competition name's gender (+0x37), read only when qualified
        return if qualified { format(buf, 0x9c4aac, read_u8(comp + 0x37) as i8 as i32, 0xad9c64usize) } else { format(buf, 0x9a6790) }; // "Qualified From Phase 1" / "Phase 1 Group Stage"
    }
    if exe::comp_util_get_parent_stage_id(round) == 0x44a {
        return if qualified { format(buf, 0x9c4a84, read_u8(comp + 0x37) as i8 as i32, 0xad9c64usize) } else { format(buf, 0x9a677c) }; // "Qualified From Phase 2" / "Phase 2 Group Stage"
    }
    if status == 1 {
        return format(buf, 0x99b7d0); // "Winners"
    }
    if round == 0 && sub_round == 0 {
        if status == 2 {
            return format(buf, 0x99b7c4); // "Runners Up"
        }
        if status == 3 {
            return format(buf, 0x99b7b4); // "Third Placed"
        }
    }
    exe::comp_util_format_stage_round_name(round, sub_round, buf, size as u32, 0)
}

/// Comparer for `european_cup_write_progress_text`: the original into the caller's buffer, the copy into a local one of
/// the same size (up to 4 KB) side by side; the texts must match. Below 30 bytes (the box) only the original.
pub(crate) unsafe extern "thiscall" fn compare_european_cup_write_progress_text(this: usize, buf: usize, size: i32, round: u32, sub_round: u32, status: u32, comp: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32, u32, u32, usize) -> i32>(%%I9%%);
    let r = o(this, buf, size, round, sub_round, status, comp);
    if size < 0x1e || buf == 0 {
        return r;
    }
    let mut local = vec![0u8; (size as usize).min(0x1000)];
    european_cup_write_progress_text(this, local.as_mut_ptr() as usize, local.len() as i32, round, sub_round, status, comp);
    let c = |p: *const u8| core::ffi::CStr::from_ptr(p as *const core::ffi::c_char).to_bytes().to_vec();
    let (mo, mr) = (c(buf as *const u8), c(local.as_ptr()));
    SCOREBOARD.record(mo == mr, || format!("european_cup_write_progress_text(round {:#x}, sub {:#x}, status {}): original {:?} rust {:?}", round as u16, sub_round as u16, status as u8, String::from_utf8_lossy(&mo), String::from_utf8_lossy(&mr)));
    r
}

/// Comparers of the destructors: they free memory (they cannot run twice); only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_drop_european_cup(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_inter_amer_cup(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_intertoto_cup(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_uefa_cup(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I3%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_world_club_cup(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I4%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_oceania_club_champ(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I5%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_world_club_champ(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I6%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_fifa_rankings(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I7%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_file_llist(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I8%%)(this)
}
