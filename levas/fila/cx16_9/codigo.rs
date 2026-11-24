// ---------- cx16_9: the World Cup record of the qualifying sections, the CONCACAF season reset and the European group sizes (wc_oceania_league.cpp, wc_concacaf_cup.cpp, wc_europe_league.cpp) ----------

/// rule: the competition the World Cup qualifying sections feed is the FIFA World Cup
///
/// The record of [`KEY_CLUB_COMP_FIFA_WORLD_CUP`] in [`G_CLUB_COMP_TABLE`] (0x6b bytes each). Shared by the CONCACAF,
/// European and Oceania vtables (+0x78); `this` is not read.
/// Original: `WcOceaniaLeague::get_finals_comp` (0x91e1c0, wc_oceania_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_oceania_league_get_finals_comp(_this: usize) -> usize {
    covered!("cx16_9 finals comp");
    read_usize(G_CLUB_COMP_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP).wrapping_mul(0x6b)) // cx16_9: the World Cup record
}

/// rule: the CONCACAF qualifying clears its run-by-stages flag (`cup::RUN_BY_STAGES` 0) and returns 1
/// Original: `WcConcacafCup::skip_season_reset` (0x915fe0, wc_concacaf_cup.cpp, thiscall, ret 0; vtable +0x2c).
pub(crate) unsafe extern "thiscall" fn wc_concacaf_cup_skip_season_reset(this: usize) -> u32 {
    covered!("cx16_9 concacaf reset");
    Cup(this).set_run_by_stages(0); // cx16_9: the flag cleared
    1
}

/// rule: the European qualifying groups (−1 is the section's own group, 0 on its stages) have five nations, with six
/// in groups 2, 4, 6 and 7 when one nation is exempt, in 2, 4 and 7 when two are, and in all but group 2 when three are
///
/// By the exempt count (`WC_EUROPE_EXEMPT`, signed byte): 0, 5; 1, 6 for groups 2, 4, 6, 7; 2, 6 for groups 2, 4, 7;
/// 3, 5 for group 2 and 6 for the others; any other count, the error box of line 0x25f and 5. `group` is the low byte
/// of the argument. Returns a word.
/// Original: `WcEuropeLeague::get_group_size` (0x918dd0, wc_europe_league.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_get_group_size(this: usize, group: u32) -> i16 {
    let g = group as u8;
    covered!("cx16_9 european group size");
    match read_u8(this + WC_EUROPE_EXEMPT) as i8 {
        0 => 5,
        1 => if matches!(g, 2 | 4 | 6 | 7) { 6 } else { 5 },
        2 => if matches!(g, 2 | 4 | 7) { 6 } else { 5 }, // cx16_9: two exempt, three groups of six
        3 => 5 + (g != 2) as i16,
        _ => {
            show_error_at(wc_cup_turnover_consts::WC_EUROPE_LEAGUE_CPP, 0x25f);
            5
        }
    }
}

/// Comparers: side by side on the result (and, for the reset, the byte it writes); the group size only off the error
/// path (a second box).
pub(crate) unsafe extern "thiscall" fn compare_wc_oceania_league_get_finals_comp(this: usize) -> usize {
    let a = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I0%%)(this);
    let b = wc_oceania_league_get_finals_comp(this);
    SCOREBOARD.record(a == b, || format!("wc_oceania_league_get_finals_comp({:#x}) original={:#x} rust={:#x}", this, a, b));
    a
}

pub(crate) unsafe extern "thiscall" fn compare_wc_concacaf_cup_skip_season_reset(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    let (a, b, same, mo, mr) = crate::comparison::side_by_side_light(&[(this + cup::RUN_BY_STAGES, 1)], || o(this), || wc_concacaf_cup_skip_season_reset(this));
    SCOREBOARD.record(same && a == b, || format!("wc_concacaf_cup_skip_season_reset({:#x}) original={} {:x?} rust={} {:x?}", this, a, mo, b, mr));
    a
}

pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_get_group_size(this: usize, group: u32) -> i16 {
    let a = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> i16>(%%I2%%)(this, group);
    if (read_u8(this + WC_EUROPE_EXEMPT) as i8) < 0 || read_u8(this + WC_EUROPE_EXEMPT) > 3 {
        return a;
    }
    let b = wc_europe_league_get_group_size(this, group);
    SCOREBOARD.record(a == b, || format!("wc_europe_league_get_group_size({:#x}, {}) exempt {} original={} rust={}", this, group as u8 as i8, read_u8(this + WC_EUROPE_EXEMPT), a, b));
    a
}
