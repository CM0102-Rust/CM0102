// ---------- batch 315: the season turnovers of the Inter-American Cup and the Oceania Club Championship; the Oceania host alternation (inter_amer_cup.cpp, oceania_club_champ.cpp, host_country.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\comp\eurocomp\inter_amer_cup.cpp" (the error boxes).
pub(crate) const INTER_AMER_CUP_CPP: usize = 0x9e9d94;
/// HostCountry::alternate_oceania_club_champ_host (thiscall, ret 0), called by the Oceania turnover on the host table.
pub(crate) const FN_HOSTCOUNTRY_ALTERNATE_OCEANIA_CLUB_CHAMP_HOST: usize = 0x5face0;
/// The size of a host table entry (`host_entry`).
pub(crate) const HOST_ENTRY_SIZE: usize = 0x22;

/// The season turnover of the Inter-American Cup: last season's tables freed, the season archived, the year moved on by
/// one and the clubs, the round table and the tie tree rebuilt; finally the predicted positions.
///
/// The cup turnover template (`start_new_cup_season`, checked instruction by instruction): the clubs from
/// InterAmerCup::select_teams (`FN_INTERAMERCUP_SELECT_TEAMS`, thiscall, ret 0) and the inter_amer_cup.cpp boxes at
/// lines 0x164, 0x16b and 0x172.
/// Original: `InterAmerCup::start_new_season` (0x632910, inter_amer_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn inter_amer_cup_start_new_season(this: usize) -> u32 {
    start_new_cup_season(Cup(this), FN_INTERAMERCUP_SELECT_TEAMS, INTER_AMER_CUP_CPP, [0x164, 0x16b, 0x172], 1, "mwg_11 inter-american cup rollover")
}

/// Comparer for `inter_amer_cup_start_new_season`: only the original runs; the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_inter_amer_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_INTER_AMER_CUP_START_NEW_SEASON);
    compare_cup_new_season("inter_amer_cup_start_new_season", Cup(this), || o(this))
}

/// Rule: the Oceania Club Championship is every 2 years: when it ends, the table and stages are undone, the season goes
/// to the statistics history, the year goes up 2, the host alternates between Australia and New Zealand and the rules
/// are set up again.
///
/// `OceaniaClubChamp::start_new_season()` (virtual +8): `.set_season_closed(0)`, the table freed
/// (`exe::league_free_table` League::free_table: push 0; mov ecx, this -> thiscall, ret 4), the round dates
/// `.round_dates()` freed (`exe::crt_free`) and zeroed, the season archived in the statistics
/// (`exe::comp_stats_archive_season_and_reset`: push 1; mov ecx, `.comp_stats()` -> thiscall, ret 4), the stages
/// 0..=`.current_stage()` (signed, read again on each pass) of `.stages()` destroyed (virtual 0 with 1: the destructor
/// that frees), the year `.year()` up 2, `.set_current_stage()` = −1, `.set_active(0)`, the host alternated
/// (`alternate_oceania_club_champ_host` on [`G_HOST_COUNTRY`], thiscall, ret 0) and the rules again (virtual
/// `comp_vtable::SET_UP_RULES`, thiscall, ret 0, without a box), whose eax it returns.
/// Original: `OceaniaClubChamp::start_new_season` (0x797d60, oceania_club_champ.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn oceania_club_champ_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    exe::league_free_table(this, 0);
    if this.round_dates() != 0 {
        exe::crt_free(this.round_dates());
        this.set_round_dates(0);
    }
    if this.comp_stats() != 0 {
        exe::comp_stats_archive_season_and_reset(this.comp_stats(), 1);
    }
    let mut i = 0i32;
    while (this.current_stage() as i32) >= 0 && i <= this.current_stage() as i32 {
        let f = read_usize(this.stages() + i as usize * 4);
        if f != 0 {
            let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(f)));
            destroy(f, 1);
        }
        i += 1;
    }
    covered!("mwg_11 oceania club championship rollover");
    this.set_year(this.year().wrapping_add(2)); // mwg11: the Oceania championship every 2 years
    this.set_current_stage(usize::MAX);
    this.set_active(0);
    let alternate: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_HOSTCOUNTRY_ALTERNATE_OCEANIA_CLUB_CHAMP_HOST);
    alternate(read_usize(G_HOST_COUNTRY));
    let rules: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::SET_UP_RULES));
    rules(this.addr())
}

// simplification: frees the table and destroys the stages: it does not run twice. Original only; the plan: the year
// up 2 and the season reopened (+0x4c = 0).
/// Comparer for `oceania_club_champ_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_oceania_club_champ_start_new_season(this: usize) -> u32 {
    let this = League(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_OCEANIA_CLUB_CHAMP_START_NEW_SEASON);
    let year = this.year();
    let r = o(this.addr());
    let ok = this.year() == year.wrapping_add(2) && this.season_closed() == 0;
    SCOREBOARD.record(ok, || format!("oceania_club_champ_start_new_season({:#x}): year {} -> {}, +0x4c {}", this.addr(), year, this.year(), this.season_closed()));
    r
}

/// Rule: the Oceania Club Championship's host alternates between Australia and New Zealand every edition.
///
/// `HostCountry::alternate_oceania_club_champ_host()` on the host table (`this`: +0 the 0x22-byte entries, +4 their
/// count, a signed word): each entry of the Oceania Club Championship (`host_entry::COMP` equal to
/// [`KEY_CLUB_COMP_OCEANIA_CLUB_CHAMPIONSHIP`]) gets New Zealand ([`KEY_NATION_NEW_ZEALAND`]) as its default host
/// (`host_entry::HOST`) when it was Australia ([`KEY_NATION_AUSTRALIA`]), and Australia otherwise (any other value,
/// −1 too). The keys are read again for each entry, as in the original.
/// Original: `HostCountry::alternate_oceania_club_champ_host` (0x5face0, host_country.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn alternate_oceania_club_champ_host(this: usize) {
    let entries = read_usize(this);
    let count = read_i16(this + 4) as i32;
    for k in 0..count.max(0) as usize {
        let e = entries + k * HOST_ENTRY_SIZE;
        if read_u32(e + host_entry::COMP) == read_u32(KEY_CLUB_COMP_OCEANIA_CLUB_CHAMPIONSHIP) {
            covered!("mwg_11 oceania host alternated");
            let australia = read_u32(KEY_NATION_AUSTRALIA);
            let host = if read_u32(e + host_entry::HOST) == australia { read_u32(KEY_NATION_NEW_ZEALAND) } else { australia }; // mwg11: Australia, then New Zealand
            write_u32(e + host_entry::HOST, host);
        }
    }
}

/// Comparer for `alternate_oceania_club_champ_host`: side by side over the host table's entries.
pub(crate) unsafe extern "thiscall" fn compare_alternate_oceania_club_champ_host(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ALTERNATE_OCEANIA_CLUB_CHAMP_HOST);
    let count = read_i16(this + 4).max(0) as usize;
    let regions = [(read_usize(this), count * HOST_ENTRY_SIZE)];
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(this), || alternate_oceania_club_champ_host(this));
    SCOREBOARD.record(equal, || format!("alternate_oceania_club_champ_host({} entries) original={:x?} rust={:x?}", count, mo, mr));
}
