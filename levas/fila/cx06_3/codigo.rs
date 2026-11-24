// ---------- cx06_3: the European Championship qualifiers' constructor, entrants and predicted positions (euro_champ_qual.cpp) ----------

/// The qualifiers' edition: from `start`, up one year at a time while the remainder of (year − 1990) by 4 is not 0
/// (signed, as the original's `and 0x80000003` with its sign adjustment): the year of the next finals.
pub(crate) fn euro_qual_edition(start: i16) -> i16 {
    let mut year = start;
    while (year as i32 - 0x7c6) % 4 != 0 {
        year = year.wrapping_add(1);
    }
    year
}

/// Builds the European Championship qualifiers on the League base: League::League (`exe::new_league`), the competition
/// record in `league::COMPETITION`, the vtable 0x96a0d4, `club_sides` 4, `upper_division_id` and `lower_division_id`
/// −1, `active` 0, the drawn teams, the entrants, their count and the qualified flag (+0xfb) 0, the hosts count
/// (`euro_champ_qual::GROUP_FORMAT`) 1, `discipline_rule` 0, the last group stage index (+0xff) 7, the runners-up stage
/// index (+0x100) 8 and the play-off stage index (+0x101) 9, the edition `year` (the next finals year from `year` − 1,
/// `euro_qual_edition`), `league_flags` 0x24, the stored group results (+0x10, a new `LLIST` of 0xe bytes:
/// `exe::crt_operator_new` and `exe::new_llist`, or null) and `importance` 0xf. If the saved state loads
/// (`exe::league_load_from_temp_file`(this, 1) nonzero), returns this.
///
/// Otherwise: the rules (`euro_champ_qual_set_up_rules`; 0: the box at line 0x5e and this), the statistics
/// (`exe::crt_operator_new`(0x5ce) and `exe::new_comp_stats`; null: the box at line 0x66 and this), `stage_index`
/// `stage_index::MAIN`, `current_stage` −1, `stage_count` 10 and the stage vector (`exe::crt_malloc`(0x28),
/// unchecked). When the season `year` (signed word) is after the edition (only a 1999 start: the 1998 edition), the
/// Euro 2000 groups and results (EuroChampQual::create_groups_and_results 0x580bd0). When the edition is more than two
/// years after `year`, `active` 0 again. Returns this.
/// Original: `EuroChampQual::EuroChampQual` (0x57ce10, euro_champ_qual.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_euro_champ_qual(this: usize, year: u32, record: usize) -> usize {
    let l = League(this);
    exe::new_league(this);
    write_usize(this + league::COMPETITION, record);
    write_usize(this, 0x96a0d4); // the qualifiers' vtable
    l.set_club_sides(4);
    l.set_upper_division_id(-1);
    l.set_lower_division_id(0xffff_ffff);
    l.set_active(0);
    write_usize(this + euro_champ_qual::DRAWN_TEAMS, 0);
    write_usize(this + 0xee, 0); // the entrants
    write_u32(this + 0xf2, 0); // their count
    write_u32(this + 0xfb, 0); // the qualified flag
    write_u8(this + euro_champ_qual::GROUP_FORMAT, 1);
    l.set_discipline_rule(0);
    write_u8(this + 0xff, 7); // the last group stage index
    write_u8(this + 0x100, 8); // the runners-up stage index
    write_u8(this + 0x101, 9); // the play-off stage index
    let season = year as u16 as i16;
    l.set_year(euro_qual_edition(season.wrapping_sub(1)));
    l.set_league_flags(0x24);
    let p = exe::crt_operator_new(0xe);
    write_usize(this + 0x10, if p != 0 { exe::new_llist(p) } else { 0 }); // the stored group results
    l.set_importance(0xf);
    if exe::league_load_from_temp_file(this, 1) != 0 {
        return this;
    }
    if euro_champ_qual_set_up_rules(this) == 0 {
        show_error_at(0x9c4674, 0x5e); // euro_champ_qual.cpp
        return this;
    }
    let p = exe::crt_operator_new(0x5ce);
    let stats = if p != 0 { exe::new_comp_stats(p, this) } else { 0 };
    l.set_comp_stats(stats);
    if stats == 0 {
        show_error_at(0x9c4674, 0x66);
        return this;
    }
    covered!("cx06_3 euro qualifiers built");
    l.set_stage_index(stage_index::MAIN);
    l.set_current_stage(0xffff_ffff);
    l.set_stage_count(10);
    l.set_stages(exe::crt_malloc(0x28));
    if season > l.year() {
        covered!("cx06_3 the 1998 edition");
        let groups_and_results: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x580bd0usize); // EuroChampQual::create_groups_and_results
        groups_and_results(this);
    }
    if (l.year() as i32) - (season as i32) > 2 {
        l.set_active(0);
    }
    this
}

/// Lists the entrants of the European Championship qualifiers: the European nations' national teams but the hosts.
///
/// `euro_champ_qual::GROUP_FORMAT` counts the European hosts of the finals: without the host table
/// (`G_HOST_COUNTRY`), the box at line 0x4c3; otherwise HostCountry::find_hosts (`exe::hostcountry_find_hosts`: the
/// competition id in `KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP`, the edition `year()` + 2, the two host ids, 1) and
/// each host (up to the count, stopping at −1) whose nation's continent (`nation::CONTINENT`, its id) is
/// `KEY_CONTINENT_EUROPE` adds one. Below 0 or above 2: the box at line 0x4dd and 1. The entrant count (+0xf2) is 51 −
/// hosts; the old list (+0xee) is freed and a new one allocated (`exe::crt_malloc`; out of memory: the box at line 0x4ec
/// and 0); every European nation in table order (`G_NATION_TABLE`, `G_NATION_COUNT`), while the list has room, gives
/// its national team (`exe::database_get_national_team`) unless its id is one of the two hosts. A short list: the box at
/// line 0x506 and 0. The list is sorted by reputation (`exe::crt_qsort` with 0x4b07d0). Returns 1.
/// Original: `EuroChampQual::collect_entrants` (0x57f600, euro_champ_qual.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn euro_champ_qual_collect_entrants(this: usize) -> u32 {
    let l = League(this);
    write_u32(this + 0xf2, 0x33); // the entrant count
    write_u8(this + euro_champ_qual::GROUP_FORMAT, 0);
    let mut hosts = [-1i32; 2];
    let is_european = |nation: usize| {
        let continent = read_usize(nation + nation::CONTINENT);
        continent != 0 && read_u32(continent) == read_u32(KEY_CONTINENT_EUROPE)
    };
    let nation_at = |id: usize| read_usize(G_NATION_TABLE).wrapping_add(id.wrapping_mul(nation::SIZE));
    if read_usize(G_HOST_COUNTRY) == 0 {
        show_error_at(0x9c4674, 0x4c3); // euro_champ_qual.cpp
    } else {
        let (first, second) = hosts.split_at_mut(1);
        let n = exe::hostcountry_find_hosts(read_usize(G_HOST_COUNTRY), read_u32(KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP) as usize,
            l.year().wrapping_add(2) as u16 as u32, &mut first[0], &mut second[0], 1) as i8;
        let mut k = 0i8;
        while k < n {
            let Some(&host) = hosts.get(k as usize) else { break };
            if host == -1 {
                break;
            }
            if is_european(nation_at(host as usize)) {
                covered!("cx06_3 european host");
                write_u8(this + euro_champ_qual::GROUP_FORMAT, read_u8(this + euro_champ_qual::GROUP_FORMAT).wrapping_add(1));
            }
            k += 1;
        }
    }
    let european_hosts = read_u8(this + euro_champ_qual::GROUP_FORMAT) as i8;
    if !(0..=2).contains(&european_hosts) {
        show_error_at(0x9c4674, 0x4dd);
        write_u8(this + euro_champ_qual::GROUP_FORMAT, 1);
    }
    let count = 0x33 - read_u8(this + euro_champ_qual::GROUP_FORMAT) as i8 as i32;
    write_i32(this + 0xf2, count);
    if read_usize(this + 0xee) != 0 {
        exe::crt_free(read_usize(this + 0xee));
    }
    let list = exe::crt_malloc((count as u32).wrapping_mul(4));
    write_usize(this + 0xee, list); // the entrants
    if list == 0 {
        show_error_at(0x9c4674, 0x4ec);
        return 0;
    }
    let mut added = 0i32;
    let mut k: i16 = 0;
    while (k as i32) < read_i32(G_NATION_COUNT) {
        let nation = nation_at(k as usize);
        if is_european(nation) && added < read_i32(this + 0xf2) {
            let team = exe::database_get_national_team(nation);
            let id = read_i32(nation);
            if id != hosts[0] && id != hosts[1] {
                write_usize(read_usize(this + 0xee) + 4 * added as usize, team);
                added += 1;
            }
        }
        k = k.wrapping_add(1);
    }
    if added < read_i32(this + 0xf2) {
        show_error_at(0x9c4674, 0x506);
        return 0;
    }
    covered!("cx06_3 qualifiers entrants");
    exe::crt_qsort(read_usize(this + 0xee), added as u32, 4, 0x4b07d0usize); // by reputation, descending
    1
}

/// The predicted final position of the `k`-th entrant (by reputation) of the qualifiers, by the number of European
/// hosts (`format`): one host, 9 group winners (1), then the best runners-up and the play-off bands 10, 13 and 16 (3
/// each), then 19, 28 and 37 (9 each) and 46; two hosts, 1 (9), 10 (1), 11 (4), 15 (4), 19, 28, 37 (9 each) and 46. No
/// position for any other format.
pub(crate) fn euro_qual_predicted_band(format: u8, k: i16) -> Option<u32> {
    let bands: &[(i16, u32)] = match format {
        1 => &[(9, 1), (0xc, 0xa), (0xf, 0xd), (0x12, 0x10), (0x1b, 0x13), (0x24, 0x1c), (0x2d, 0x25)],
        2 => &[(9, 1), (0xa, 0xa), (0xe, 0xb), (0x12, 0xf), (0x1b, 0x13), (0x24, 0x1c), (0x2d, 0x25)],
        _ => return None,
    };
    Some(bands.iter().find(|b| k < b.0).map_or(0x2e, |b| b.1))
}

/// Sets the predicted final positions of the qualifiers' entrants: without statistics, the box at line 0x6f3. The
/// entrants (+0xee, count +0xf2) sorted by reputation (`exe::crt_qsort` with 0x4b07d0); the national teams freed from
/// their European competitions (NationalTeams::release_competitions_of_continent
/// `exe::nationalteams_release_competitions_of_continent` on the continent `KEY_CONTINENT_EUROPE`, 0xc6-byte records of
/// `G_CONTINENT_TABLE`, with 1); then each entrant k, in order, its band (`euro_qual_predicted_band`, Comp_Stats
/// set_club_final_position `exe::comp_stats_set_club_final_position`) and the qualifiers as its nation's current
/// competition (NationalTeams::set_current_competition `exe::nationalteams_set_current_competition`).
/// Original: `EuroChampQual::set_predicted_positions` (0x5806d0, euro_champ_qual.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn euro_champ_qual_set_predicted_positions(this: usize) {
    let l = League(this);
    if l.comp_stats() == 0 {
        show_error_at(0x9c4674, 0x6f3); // euro_champ_qual.cpp
        return;
    }
    exe::crt_qsort(read_usize(this + 0xee), read_u32(this + 0xf2), 4, 0x4b07d0usize);
    let europe = read_usize(G_CONTINENT_TABLE).wrapping_add((read_u32(KEY_CONTINENT_EUROPE) as usize).wrapping_mul(0xc6)); // continent records
    exe::nationalteams_release_competitions_of_continent(read_usize(G_NATIONAL_TEAMS), europe, 1);
    let mut k: i16 = 0;
    while (k as i32) < read_i32(this + 0xf2) {
        let team = Club(read_usize(read_usize(this + 0xee) + 4 * k as usize));
        if let Some(band) = euro_qual_predicted_band(read_u8(this + euro_champ_qual::GROUP_FORMAT), k) {
            covered!("cx06_3 qualifiers predicted band");
            exe::comp_stats_set_club_final_position(l.comp_stats(), team, band);
        }
        exe::nationalteams_set_current_competition(read_usize(G_NATIONAL_TEAMS), team.nation(), l.competition());
        k = k.wrapping_add(1);
    }
}

/// Comparer for `construct_euro_champ_qual`: only the original runs (it allocates and loads); the plan: this returned,
/// the vtable, the record, the edition year (`euro_qual_edition`; any year when loading a saved game) and importance
/// 0xf (`compare_nations_tournament_ctor`).
pub(crate) unsafe extern "thiscall" fn compare_construct_euro_champ_qual(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    compare_nations_tournament_ctor("construct_euro_champ_qual", League(this), record, 0x96a0d4, euro_qual_edition((year as u16 as i16).wrapping_sub(1)), || o(this, year, record))
}

/// Comparer for `euro_champ_qual_collect_entrants`: only the original runs (it frees and allocates the list); the plan:
/// on success the count is 51 minus the hosts (0 to 2), every entrant a national team, sorted by reputation
/// (`club::REPUTATION`, descending).
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_qual_collect_entrants(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%)(this);
    if r != 1 {
        return r;
    }
    let (list, count, hosts) = (read_usize(this + 0xee), read_i32(this + 0xf2), read_u8(this + euro_champ_qual::GROUP_FORMAT) as i32);
    let teams: Vec<Club> = (0..count.clamp(0, 64) as usize).map(|k| Club(read_usize(list + 4 * k))).collect();
    let sorted = teams.windows(2).all(|w| w[0].reputation() >= w[1].reputation());
    let ok = count == 0x33 - hosts && (0..=2).contains(&hosts) && teams.iter().all(|t| t.addr() != 0) && sorted;
    SCOREBOARD.record(ok, || format!("euro_champ_qual_collect_entrants({:#x}): {} entrants, {} hosts, sorted {}", this, count, hosts, sorted));
    r
}

/// Comparer for `euro_champ_qual_set_predicted_positions`: only the original runs (the national teams' competitions
/// change); the plan: by reputation the entrants' final positions make the bands (`predicted_by_reputation_check`).
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_qual_set_predicted_positions(this: usize) {
    let (list, count) = (read_usize(this + 0xee), read_i32(this + 0xf2));
    let clubs: Vec<usize> = if list != 0 && (0..=64).contains(&count) { (0..count as usize).map(|k| read_usize(list + 4 * k)).collect() } else { Vec::new() };
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this);
    let format = read_u8(this + euro_champ_qual::GROUP_FORMAT);
    let bands: Vec<u32> = (0..clubs.len() as i16).filter_map(|k| euro_qual_predicted_band(format, k)).collect();
    if bands.len() == clubs.len() {
        predicted_by_reputation_check("euro_champ_qual_set_predicted_positions", this, &clubs, &bands);
    }
}
