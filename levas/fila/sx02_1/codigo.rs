/// How far news about a player reaches the human managers (flag 0x10 of `collect_player_news_recipients`), worked out
/// from his reputations: every human manager, those of the same continent, of the same nation (England and Scotland
/// count as one), or those whose club plays in the player's club's division.
#[derive(Default)]
pub(crate) struct PlayerNewsReach {
    everyone: bool,
    continental: bool,
    national: bool,
    division: bool,
    /// The continent of the player's nation and of his club job's nation.
    continents: [usize; 2],
    /// The player's nation (by home reputation) and his contract club's nation (by current reputation or form).
    nations: [usize; 2],
    /// The division of the player's contract club (not a type-5 contract).
    league: usize,
}

/// The nation of the player's contract club (not a type-5 contract), 0 without one; two contract lookups, as the
/// original does (the first only checks that there is a club).
unsafe fn player_contract_club_nation(player: Staff) -> usize {
    if contract_club_excluding_type5(player.addr()) == 0 {
        return 0;
    }
    Club(contract_club_excluding_type5(player.addr())).nation().addr()
}

/// The reach of news about `player` (see `collect_player_news_recipients`, flag 0x10).
unsafe fn player_news_reach(player: Staff, flags: u32) -> PlayerNewsReach {
    let mut r = PlayerNewsReach::default();
    if contract_club_excluding_type5(player.addr()) != 0 {
        r.league = Club(contract_club_excluding_type5(player.addr())).division().addr();
    }
    let sheet = player.player();
    let non_player = player.non_player();
    let (current, home, world) = if !sheet.is_null() {
        (sheet.current_reputation(), sheet.home_reputation(), sheet.world_reputation())
    } else if !non_player.is_null() {
        (non_player.current_reputation(), non_player.home_reputation(), non_player.world_reputation())
    } else {
        (0, 0, 0)
    };
    // flag 0x200: the higher thresholds (everyone, continent, nation) and the gentler form slope
    let wide = flags & 0x200 != 0;
    let (everyone, continent, nation, slope) = if wide {
        (0x251c, 0x2328, 0x206c, read_f64(0x970430)) // f64 0.00015
    } else {
        (0x23f0, 0x206c, 0x1c20, read_f64(rdata::F64_0_00025))
    };
    // in form: the average rating (PlayerStats category 2, field 0x12) above `base` - current reputation x slope
    let in_form = |base: f64| {
        !sheet.is_null() && exe::playerstats_get_stat(read_usize(G_PLAYER_STATS), sheet, 2, 0x12, 6.5) > base - current as i32 as f64 * slope
    };
    if world > everyone {
        r.everyone = true;
    } else if world > continent {
        r.continental = true;
        let n = player.nation();
        if !n.is_null() {
            r.continents[0] = n.continent();
        }
        let club = player.club_job();
        if !club.is_null() && !club.nation().is_null() {
            r.continents[1] = club.nation().continent();
        }
    } else if home > nation || current > nation {
        covered!("sx02_1 national reach");
        r.national = true; // sx02_1: national by reputation
        if home > nation {
            r.nations[0] = player.nation().addr();
        }
        if current > nation {
            r.nations[1] = player_contract_club_nation(player);
        }
    } else if in_form(read_f64(0x970428)) && sheet.current_reputation() > 3000 && flags & 0x80 != 0 {
        // f64 9.25: a player in form with some reputation reaches his club's nation
        r.national = true;
        r.nations[1] = player_contract_club_nation(player);
    } else if flags & 0x40 != 0 && r.league != 0 {
        let reputation = Competition(r.league).reputation() as i32;
        let current = current as i32;
        // f64 9.0 for the form
        r.division = if wide {
            current > (reputation * 750).max(5000) || in_form(read_f64(0x96d230)) // sx02_1: division by reputation
        } else {
            (current > reputation * 350 && current > 2500) || in_form(read_f64(0x96d230)) // sx02_1: division form
        };
    }
    r
}

/// Whether the reach takes the human `manager` of `club`.
unsafe fn player_news_reach_takes(r: &PlayerNewsReach, manager: Staff, club: Club) -> bool {
    let (nation, club_nation) = (manager.nation(), club.nation());
    if r.everyone {
        return true;
    }
    if r.continental {
        let a = if nation.is_null() { 0 } else { nation.continent() };
        let b = if club_nation.is_null() { 0 } else { club_nation.continent() };
        return a != 0 && r.continents.contains(&a) || b != 0 && r.continents.contains(&b);
    }
    if r.national {
        let (n, c) = (nation.addr(), club_nation.addr());
        if n != 0 && r.nations.contains(&n) || c != 0 && r.nations.contains(&c) {
            return true;
        }
        if n == 0 {
            return false;
        }
        // England and Scotland reach each other through the manager's own nation
        let (id, england, scotland) = (read_usize(n), read_usize(KEY_NATION_ENGLAND), read_usize(KEY_NATION_SCOTLAND));
        let any_is = |key: usize| r.nations.iter().any(|&x| x != 0 && read_usize(x) == key);
        return id == england && any_is(scotland) || id == scotland && any_is(england);
    }
    r.division && club.division().addr() == r.league
}

/// Appends `club` to the recipients array `out` at `n` and counts it.
unsafe fn push_recipient(out: *mut usize, n: &mut i8, club: usize) {
    *out.add(*n as usize) = club;
    *n += 1;
}

/// Lists the human-managed clubs that should receive news about a player, into `out` (up to `max`), and returns how
/// many.
///
/// - Flag 2: the club of the player's contract (any contract type, `contract_club_any_type`, which stores the lookup in
/// `G_CONTRACT_LOOKUP_RESULT`) when a human manages it (`exe::news_club_has_human_manager` on `G_NEWS`) goes first.
/// - Flag 8: the clubs shortlisting him (`exe::shortlist_manager_collect_interested_clubs` of the manager's
/// `shortlist_manager()`, with 1, 1, 0, into a local list) managed by a human; past `max`, the "Error" box (line 0x3aa4)
/// for each one left out.
/// - Flag 0x10: each of the 16 human manager slots (the last 16 people of the table, from the last one) with a
/// `classification()` and a `club_job()`, when the player's reach (`player_news_reach`) takes him. From his
/// reputations (the player sheet's, otherwise the non-player's; flag 0x200 raises the thresholds): world reputation
/// above 9500 (0x200) or 9200: everyone; above 9000 or 8300: the managers whose nation or club nation is on the
/// continent of the player's nation or of his club job's nation; home or current reputation above 8300 or 7200: the
/// managers whose nation or club nation is the player's nation (by home reputation) or his contract club's nation (by
/// current reputation), England and Scotland reaching each other; otherwise, a player in form (average rating above 9.25
/// - current reputation x 0.00015, or x 0.00025 without 0x200) with current reputation above 3000 and flag 0x80: the
/// contract club's nation; otherwise with flag 0x40: the managers whose club is in the player's contract club's division,
/// when his current reputation is above the division's reputation x 750 (at least 5000; with 0x200) or above the
/// division's reputation x 350 and 2500 (without), or he is in form against 9.0. A full list: the "Error" box (line
/// 0x3b43) and the slots stop.
/// - Then the duplicates are dropped: each later copy of an entry is replaced by the last entry.
///
/// Defect of the original: after a duplicate is replaced by the last entry, the next slot is checked, not the moved
/// entry, so a club listed three times (by flags 2, 8 and 0x10) can stay twice.
/// Original: `transfer_manager_collect_player_news_recipients` (0x8d1da0, transfer_manager.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn collect_player_news_recipients(this: usize, player: usize, flags: u32, out: *mut usize, max: u32) -> u8 {
    let this = TransferManagerFields(this);
    let player = Staff(player);
    let max = max as u8 as i8;
    let mut n: i8 = 0;
    let mut list = [0u8; 0x10];
    let l = Llist(list.as_mut_ptr() as usize);
    exe::new_llist(l);
    if flags & 2 != 0 && contract_club_any_type(player) != 0
        && exe::news_club_has_human_manager(read_usize(G_NEWS), contract_club_any_type(player)) != 0 {
        *out = contract_club_any_type(player);
        n = 1;
    }
    if flags & 8 != 0 {
        exe::shortlist_manager_collect_interested_clubs(this.shortlist_manager(), player, l, 1, 1, 0);
        l.set_cursor(l.head());
        loop {
            let e = exe::llist_get_next(l);
            if e == 0 {
                break;
            }
            let club = transfer_manager_club_by_id(read_usize(e + 4)); // list entry +4: the club id
            if exe::news_club_has_human_manager(read_usize(G_NEWS), club) != 0 {
                if n < max {
                    covered!("sx02_1 shortlisting human club");
                    push_recipient(out, &mut n, club.addr()); // sx02_1: a shortlisting human club
                } else {
                    show_error_box(0xa8d514, 0x3aa4);
                }
            }
        }
    }
    if flags & 0x10 != 0 {
        let reach = player_news_reach(player, flags);
        for k in 0..16 {
            if n == max {
                show_error_box(0xa8d514, 0x3b43);
                break;
            }
            let manager = staff_at((read_i32(G_STAFF_COUNT) - k - 1) as isize);
            if manager.classification() == 0 || manager.club_job().is_null() {
                continue;
            }
            if player_news_reach_takes(&reach, manager, manager.club_job()) {
                push_recipient(out, &mut n, manager.club_job().addr());
            }
        }
    }
    // duplicates: a later copy is replaced by the last entry (the moved entry is not checked again)
    let mut i: i8 = 0;
    if n - 1 > 0 {
        loop {
            let mut j = i + 1;
            while j < n {
                if *out.add(i as usize) == *out.add(j as usize) {
                    n -= 1;
                    *out.add(j as usize) = *out.add(n as usize);
                }
                j += 1;
            }
            i += 1;
            if i >= n - 1 {
                break;
            }
        }
    }
    exe::drop_llist(l);
    n as u8
}

/// Comparer for `collect_player_news_recipients`: side by side from the same state of `G_CONTRACT_LOOKUP_RESULT` and of
/// the output array (`max` entries); checks the count and the list.
pub(crate) unsafe extern "thiscall" fn compare_collect_player_news_recipients(this: usize, player: usize, flags: u32, out: *mut usize, max: u32) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u8>(%%I0%%);
    let slots = (max as u8 as i8).max(0) as usize;
    let regions = [(G_CONTRACT_LOOKUP_RESULT, 4usize), (out as usize, 4 * slots)];
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side_light(&regions, || o(this, player, flags, out, max),
        || collect_player_news_recipients(this, player, flags, out, max));
    SCOREBOARD.record(equal && r == n, || format!(
        "collect_player_news_recipients(player {:#x}, flags {:#x}): original {} rust {} {:x?} x {:x?}", player, flags, r, n, mo, mr));
    r
}
