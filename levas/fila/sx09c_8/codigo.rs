// ---------- batch %%LEVA%%: whether a fine is justified (Fine.cpp) ----------

/// `Media::get_finable_story_kind(player)` (thiscall on `G_MEDIA`, ret 4 -> ax).
const FN_MEDIA_GET_FINABLE_STORY_KIND: usize = 0x7357c0;
/// `AWOL::has_fitness_incident(player)` (thiscall on `G_AWOL`, ret 4 -> al).
const FN_AWOL_HAS_FITNESS_INCIDENT: usize = 0x419070;

/// The size of a team block of the match record (`match_record::HOME_TEAM`, the away one right after it).
const FINE_MATCH_TEAM_SIZE: usize = 0x625;

/// The player's last match in the 8 weeks up to today: the club's fixtures from 8 weeks ago (`exe::date_sub_weeks`,
/// called twice as the original does: the year of the first, the day of the second) to today
/// (`exe::fixture_manager_find_fixtures` into `list`), walked back from the newest to the first played one
/// (`fixture` +0x43 not negative), its match record read (`exe::match_stats_read_match_record`) into `record`, and the
/// team block of the player's club (home or away). None when no match was played or the club played neither side.
unsafe fn fine_last_match_team(club: Club, contract: usize, list: usize, record: MatchRecord) -> Option<usize> {
    let (mut a, mut b) = ([0u8; 8], [0u8; 8]);
    let year_from = read_u16(exe::date_sub_weeks(G_CURRENT_DATE, &mut a, 8) + 2) as u32;
    let day_from = read_u16(exe::date_sub_weeks(G_CURRENT_DATE, &mut b, 8)) as u32;
    // the original pushes today's year in a dword whose upper word is the contract pointer's
    let year_to = (contract as u32 & 0xffff_0000) | read_u16(G_CURRENT_DATE_YEAR) as u32;
    exe::fixture_manager_find_fixtures(read_usize(G_FIXTURE_MANAGER), day_from, year_from, read_u16(G_CURRENT_DATE) as u32, year_to,
        read_u32(club.addr()), list, u32::MAX, u32::MAX, u32::MAX);
    if read_u16(list + llist::COUNT) == 0 {
        return None;
    }
    write_u32(list + llist::CURSOR, read_u32(list + llist::TAIL));
    let mut fixture = exe::llist_get_prev(list);
    while fixture != 0 && read_i8(fixture + 0x43) <= -1 {
        // +0x43: negative until the match is played
        fixture = exe::llist_get_prev(list);
    }
    if fixture == 0 {
        return None;
    }
    covered!("sx09c_8 last match read");
    exe::match_stats_read_match_record(read_usize(G_MATCH_STATS), read_u32(fixture + 8), record);
    let id = read_u32(club.addr());
    let side = if read_u32(record.addr() + match_record::HOME_CLUB_ID) == id {
        0
    } else if read_u32(record.addr() + match_record::AWAY_CLUB_ID) == id {
        1
    } else {
        return Some(0);
    };
    Some(record.addr() + match_record::HOME_TEAM + side * FINE_MATCH_TEAM_SIZE)
}

/// The row of `player` among the 20 of a team block, if any (the first that matches each time `rows` is walked).
unsafe fn fine_card_rows(team: usize, player: Staff) -> impl Iterator<Item = MatchRecordRow> {
    let id = read_u32(player.addr());
    (0..20usize).map(move |k| MatchRecordRow(team + match_record_team::ROWS + k * match_record_row::SIZE))
        .filter(move |r| read_u32(r.addr()) == id)
}

/// Whether an offence code is a sending-off for violence (0xb, 0xc).
fn fine_violent_offence(code: i8) -> bool {
    code == 0xb || code == 0xc
}

/// Reason 1 (and 5): misconduct on and off the pitch. Some(justified) when decided, None to go on to the draws.
unsafe fn fine_conduct_justified(player: Staff, contract: usize, reason: i8, list: usize, record: MatchRecord) -> Option<bool> {
    let story: unsafe extern "thiscall" fn(usize, usize) -> u16 = core::mem::transmute(FN_MEDIA_GET_FINABLE_STORY_KIND);
    if story(read_usize(G_MEDIA), player.addr()) != 0 {
        return Some(true); // a story in the press
    }
    let awol = read_usize(G_AWOL);
    if exe::awol_is_player_awol(awol, player, 0) != 0 {
        return Some(true); // sx09c_8: absent without leave
    }
    if exe::awol_has_missed_training(awol, player) != 0 && reason <= 3 {
        return Some(true);
    }
    let incident: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_AWOL_HAS_FITNESS_INCIDENT);
    if incident(awol, player.addr()) != 0 && (reason == 1 || reason == 2) {
        return Some(true);
    }
    let sheet = player.player();
    if !sheet.is_null() && Contract(contract).morale() < -25 && sheet.work_rate() < 10 && player.professionalism() < 10
        && sheet.current_reputation() as i32 > player.club_job().reputation() as i32 + 250 {
        return Some(true); // a star sulking
    }
    let team = fine_last_match_team(player.club_job(), contract, list, record)?;
    if team == 0 {
        return Some(false);
    }
    for row in fine_card_rows(team, player) {
        let yellow = row.yellow_cards();
        if yellow == 1 {
            if matches!(row.first_card_offence(), 1 | 2 | 4 | 5 | 6 | 8) && reason == 1 {
                return Some(true);
            }
            continue;
        }
        if fine_violent_offence(row.first_card_offence()) || fine_violent_offence(row.second_card_offence()) {
            return Some(true);
        }
        if yellow == 2 {
            if reason <= 2 {
                return Some(true); // sx09c_8: two bookings
            }
        } else if row.red_card() == 1 && yellow < 2 && reason <= 3 {
            return Some(true);
        }
    }
    None
}

/// Reasons 2 and 3: the last match's cards (2: also a straight red with up to reason 2; a second booking only counts
/// for reason 1, which never reaches here).
unsafe fn fine_cards_justified(player: Staff, contract: usize, reason: i8, list: usize, record: MatchRecord) -> bool {
    let Some(team) = fine_last_match_team(player.club_job(), contract, list, record) else { return false };
    if team == 0 {
        return false;
    }
    for row in fine_card_rows(team, player) {
        if fine_violent_offence(row.first_card_offence()) || fine_violent_offence(row.second_card_offence()) {
            return true; // sx09c_8: a violent sending-off
        }
        if reason != 2 {
            continue; // reason 3: only the violent ones
        }
        let yellow = row.yellow_cards();
        if yellow == 2 {
            if reason == 1 {
                return true;
            }
        } else if row.red_card() == 1 && yellow < 2 && reason <= 2 {
            return true;
        }
    }
    false
}

/// Reason 4: the last match rating (`player_stats_entry::LAST_RATING` & 0xf of the player's stats row): none, not
/// justified; below 4, justified; 4 with up to reason 2 and 5 with up to reason 1, justified; otherwise not.
unsafe fn fine_rating_justified(player: Staff, reason: i8) -> bool {
    let sheet = player.player();
    if sheet.is_null() {
        return false;
    }
    let stats = read_usize(read_usize(G_PLAYER_STATS) + 0x30).wrapping_add((read_u32(sheet.addr()) as usize).wrapping_mul(0x27)); // the stats rows
    if stats == 0 {
        return false;
    }
    let rating = read_u8(stats + player_stats_entry::LAST_RATING) & 0xf;
    if rating == 0 {
        return false;
    }
    rating < 4 || (rating == 4 && reason <= 2) || (rating == 5 && reason <= 1) // sx09c_8: a poor rating
}

/// Whether the fined player accepts the fine as fair (1) or contests it (0).
///
/// `Fine::is_fine_justified(player, period, reason)`, from `fine_player` only. Without a club or a contract: 1. A fine
/// still running for the same player (`[this]` list, the date at +6 after today): one he did not accept (+5 = 0) makes
/// this one unfair (0); one he accepted, unfair when random(professionalism) is 0. Then by reason:
/// - 1 and 5: a press story, AWOL, missed training (reason up to 3), a fitness incident (1, 2) or a sulking star
///   (contract morale below −25, work rate and professionalism below 10, reputation over his club's + 250): fair. Else
///   the last match in 8 weeks: a booking for an offence 1, 2, 4, 5, 6 or 8 (reason 1), a violent sending-off
///   (0xb, 0xc), two bookings (up to reason 2) or a straight red (up to reason 3): fair; a match of other clubs:
///   unfair. Then reason 1 with temperament + pressure above random(50): fair; finally random(100) = 0: fair.
/// - 2: the last match's violent sending-off or straight red: fair; otherwise unfair.
/// - 3: the last match's violent sending-off: fair; otherwise unfair.
/// - 4: the last match rating (`fine_rating_justified`).
/// - other: unfair.
///
/// The two team blocks of the local match record are built and destroyed as the original does
/// (`exe::crt_vector_constructor_iterator`), and the local fixture list emptied (`exe::drop_llist`).
/// Defects of the original: the fixture search gets today's year in a dword whose upper word is the contract pointer's
/// (kept), and the start day and year in registers with leftover upper words (the copy passes the words; the search
/// reads only them); the match record is read without checking the result.
/// Original: `Fine::is_fine_justified` (0x5a69b0, Fine.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn is_fine_justified(this: usize, player: usize, period: u32, reason: u32) -> u8 {
    let _ = period;
    let player = Staff(player);
    let reason = reason as u8 as i8;
    let mut record = [0u32; 0x360]; // the 0xd7a-byte match record
    let record = MatchRecord(record.as_mut_ptr() as usize);
    let sides = record.addr() + match_record::HOME_TEAM;
    exe::crt_vector_constructor_iterator(sides, FINE_MATCH_TEAM_SIZE as u32, 2, 0x453c30usize, 0x453c10usize);
    let mut list = [0u8; 14];
    let l = list.as_mut_ptr() as usize;
    exe::new_llist(l);
    let verdict = fine_verdict(this, player, reason, l, record);
    exe::drop_llist(l);
    exe::crt_vector_destructor_iterator(sides, FINE_MATCH_TEAM_SIZE as u32, 2, 0x453c10usize);
    verdict as u8
}

/// The body of `is_fine_justified` between the construction and the destruction of its locals.
unsafe fn fine_verdict(this: usize, player: Staff, reason: i8, list: usize, record: MatchRecord) -> bool {
    if player.club_job().is_null() {
        return true;
    }
    let contract = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), player);
    if contract == 0 {
        return true;
    }
    covered!("sx09c_8 fine verdict");
    let fines = read_usize(this);
    list_rewind(fines);
    let mut k = 0i32;
    while k < read_i16(fines + llist::COUNT) as i32 {
        let e = exe::llist_get_next(fines);
        if e == 0 {
            break;
        }
        if read_u32(e) == read_u32(player.addr()) {
            let (day, year) = (read_i16(e + 6), read_u16(e + 8));
            let today_year = read_u16(G_CURRENT_DATE_YEAR);
            if year > today_year || (year == today_year && day > read_i16(G_CURRENT_DATE)) {
                if read_u8(e + 5) == 0 {
                    return false; // sx09c_8: a running fine he contested
                }
                if exe::utils_random(player.professionalism() as i32) == 0 {
                    return false;
                }
            }
        }
        k += 1;
    }
    match reason {
        1 | 5 => {
            if let Some(v) = fine_conduct_justified(player, contract, reason, list, record) {
                return v;
            }
            if reason == 1 && player.temperament() as i32 + player.pressure() as i32 > exe::utils_random(0x32) {
                return true;
            }
            exe::utils_random(100) == 0
        }
        2 | 3 => fine_cards_justified(player, contract, reason, list, record),
        4 => fine_rating_justified(player, reason),
        _ => false,
    }
}

/// Comparer for `is_fine_justified`: side by side (the random generator restored between the two, the copy's draws not
/// counted); the answer and the generator's state must match.
pub(crate) unsafe extern "thiscall" fn compare_is_fine_justified(this: usize, player: usize, period: u32, reason: u32) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u8>(%%I0%%);
    let (r, n, same, _, _) = crate::comparison::side_by_side(&crate::rng::STATE_REGIONS, || o(this, player, period, reason),
        || crate::rng::without_counting(|| is_fine_justified(this, player, period, reason)));
    SCOREBOARD.record(same && r == n, || format!("is_fine_justified(reason {}) original {} rust {} rng equal {}", reason as u8 as i8, r, n, same));
    r
}
