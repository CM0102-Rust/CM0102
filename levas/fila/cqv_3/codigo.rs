// ---------- %%LEVA%%a leva: the Brazilian and J2 season turnover: promotion and relegation between the state leagues and their lower divisions, the 16 clubs entering the third division and the J2 turnover (bra_nat_first.cpp, jap_j2.cpp) ----------

/// The keys of the twelve state leagues (the league object in [[`G_COMP_OBJECT_TABLE`] + key × 4]) and those of each
/// one's lower division (the record in [`G_CLUB_COMP_TABLE`] + key × 0x6b), in the original's order.
const BRA_STATE_LEAGUE_KEYS: [usize; 12] = [
    KEY_CLUB_COMP_BRAZILIAN_SAO_PAULO_STATE_CHAMP, KEY_CLUB_COMP_BRAZILIAN_RIO_DE_JANEIRO_STATE_CHAMPIONS, KEY_CLUB_COMP_BRAZILIAN_MINAS_GERAIS_STATE_CHAMPIONSHI,
    KEY_CLUB_COMP_BRAZILIAN_PARANA_STATE_CHAMPIONSHIP, KEY_CLUB_COMP_BRAZILIAN_CENTRAL_STATE_CHAMPIONSHIP, KEY_CLUB_COMP_BRAZILIAN_PERNAMBUCO_STATE_CHAMPIONSHIP,
    KEY_CLUB_COMP_BRAZILIAN_BAHIA_STATE_CHAMPIONSHIP, KEY_CLUB_COMP_BRAZILIAN_SANTA_CATARINA_STATE_CHAMPIONS, KEY_CLUB_COMP_BRAZILIAN_GOIAS_STATE_CHAMPIONSHIP,
    KEY_CLUB_COMP_BRAZILIAN_NORTH_STATE_CHAMPIONSHIP, KEY_CLUB_COMP_BRAZILIAN_NORTHEAST_STATE_CHAMPIONSHIP, KEY_CLUB_COMP_BRAZILIAN_GAUCHO_STATE_CHAMPIONSHIP,
];
const BRA_STATE_LOWER_DIVISION_KEYS: [usize; 12] = [
    KEY_CLUB_COMP_BRAZILIAN_SAO_PAULO_LOWER_DIVISION, KEY_CLUB_COMP_BRAZILIAN_RIO_DE_JANEIRO_LOWER_DIVISION, KEY_CLUB_COMP_BRAZILIAN_MINAS_GERAIS_LOWER_DIVISION,
    KEY_CLUB_COMP_BRAZILIAN_PARANA_LOWER_DIVISION, KEY_CLUB_COMP_BRAZILIAN_CENTRAL_LOWER_DIVISION, KEY_CLUB_COMP_BRAZILIAN_PERNAMBUCO_LOWER_DIVISION,
    KEY_CLUB_COMP_BRAZILIAN_BAHIA_LOWER_DIVISION, KEY_CLUB_COMP_BRAZILIAN_SANTA_CATARINA_LOWER_DIVISION, KEY_CLUB_COMP_BRAZILIAN_GOIAS_LOWER_DIVISION,
    KEY_CLUB_COMP_BRAZILIAN_NORTH_LOWER_DIVISION, KEY_CLUB_COMP_BRAZILIAN_NORTHEAST_LOWER_DIVISION, KEY_CLUB_COMP_BRAZILIAN_GAUCHO_LOWER_DIVISION,
];

/// The competition object (league) of key `key`: [[`G_COMP_OBJECT_TABLE`] + 4 × [key]].
unsafe fn comp_object_of_key(key: usize) -> usize {
    read_usize(read_usize(G_COMP_OBJECT_TABLE) + 4 * read_usize(key))
}

/// What a club leaving a division goes through in the original (both relegations): if it cannot be managed
/// (`exe::club_can_be_managed`, cdecl, returns al = 0) and a human manager decides for it
/// (`exe::human_manager_is_human_club`(club, 1, 1, 0, 0), cdecl), the manager leaves
/// (`exe::manager_manager_remove_club_manager`([`G_MANAGER_MANAGER`], club, 1, 7, 0), thiscall, ret 0x10); then the
/// relegation clauses of the contracts (`exe::contract_manager_trigger_relegation_clauses`([`G_CONTRACT_MANAGER`],
/// club, the record of the division it left), thiscall, ret 8).
unsafe fn bra_nat_first_club_left_division(club: usize, record: usize) {
    if exe::club_can_be_managed(club) == 0 && exe::human_manager_is_human_club(club, 1, 1, 0, 0) != 0 {
        exe::manager_manager_remove_club_manager(read_usize(G_MANAGER_MANAGER), club, 1, 7, 0);
    }
    exe::contract_manager_trigger_relegation_clauses(read_usize(G_CONTRACT_MANAGER), club, record);
}

/// At the Brazilian season turnover, promotion and relegation between the twelve state leagues
/// (`BRA_STATE_LEAGUE_KEYS`) and their twelve lower divisions (`BRA_STATE_LOWER_DIVISION_KEYS`); as many clubs go up
/// from each lower division as its state league's `league::RELEGATED_COUNT` (signed byte).
/// 1) A list of 200 clubs (`exe::crt_malloc` of 0x320; out of memory: the error box of line 0x3d0 and return) with the
///    database clubs whose regional division (`club::RESERVE_DIVISION`) is one of the twelve lower ones, in database
///    order, up to 200.
/// 2) The list sorted by reputation and the first 75 shuffled (`exe::comp_util_sort_and_shuffle_top_clubs`(list, n,
///    25), cdecl: it draws).
/// 3) In list order, each club goes up to its division's state league while there are places
///    (`club::RESERVE_DIVISION` = the state league's record, its `league::COMPETITION`); if it has a national division
///    (`club::DIVISION`) and no manager (`club::MANAGER` = 0), it opens the job (`exe::manager_manager_add_job_vacancy`
///    ([`G_MANAGER_MANAGER`], club, 0), thiscall, ret 8).
/// 4) In each state league, the clubs of the last `league::RELEGATED_COUNT` table rows (`exe::league_get_table_row`)
///    go down to the lower division (`club::RESERVE_DIVISION` = its record) and leave the division
///    (`bra_nat_first_club_left_division`, with the state league's record).
/// 5) The list is freed (`exe::crt_free`).
/// Original: `bra_nat_first_exchange_state_league_clubs` (0x42b130, bra_nat_first.cpp, cdecl).
unsafe extern "C" fn bra_nat_first_exchange_state_league_clubs() {
    let leagues: [usize; 12] = core::array::from_fn(|i| comp_object_of_key(BRA_STATE_LEAGUE_KEYS[i]));
    let lower: [usize; 12] = core::array::from_fn(|i| comp_record_of_key(BRA_STATE_LOWER_DIVISION_KEYS[i]));
    let mut places: [i8; 12] = core::array::from_fn(|i| read_i8(leagues[i] + league::RELEGATED_COUNT));
    let list = exe::crt_malloc(0x320);
    if list == 0 {
        return show_error_at(0x98aa30, 0x3d0);
    }
    let mut n = 0i16;
    let mut i = 0usize;
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 && n < 0xc8 {
        let c = club_at(i as isize).addr();
        let d = read_usize(c + club::RESERVE_DIVISION);
        if d != 0 && lower.contains(&d) {
            write_usize(list + 4 * n as usize, c);
            n += 1;
        }
        i += 1;
    }
    exe::comp_util_sort_and_shuffle_top_clubs(list, n as u16 as u32, 0x19);
    for k in 0..n.max(0) as usize { // cqv3: the list in order
        let c = read_usize(list + 4 * k);
        let d = read_usize(c + club::RESERVE_DIVISION);
        let e = match lower.iter().position(|&b| b == d) {
            Some(e) => e,
            None => continue, // ponytail: every club of the list belongs to one of the twelve
        };
        if places[e] > 0 { // cqv3: the state league's promotion places
            covered!("cqv_3 promotion to the state league");
            write_usize(c + club::RESERVE_DIVISION, read_usize(leagues[e] + league::COMPETITION));
            if read_usize(c + club::DIVISION) != 0 && read_usize(c + club::MANAGER) == 0 {
                exe::manager_manager_add_job_vacancy(read_usize(G_MANAGER_MANAGER), c, 0usize);
            }
            places[e] -= 1;
        }
    }
    for e in 0..12 {
        let l = leagues[e];
        let total = read_i16(l + league::CLUB_COUNT) as i32;
        let mut s = total - read_i8(l + league::RELEGATED_COUNT) as i32;
        while s < read_i16(l + league::CLUB_COUNT) as i32 {
            let c = read_usize(exe::league_get_table_row(l, s));
            write_usize(c + club::RESERVE_DIVISION, lower[e]); // cqv3: the relegated club goes to the lower division
            covered!("cqv_3 relegated from the state league");
            bra_nat_first_club_left_division(c, read_usize(l + league::COMPETITION));
            s += 1;
        }
    }
    exe::crt_free(list);
}

/// The promotion comparer: runs only the original (it moves clubs between divisions, draws the shuffle and calls the
/// manager and contract code; it cannot run twice), with the plan made before: the clubs of each lower division in the
/// list (up to 200) and those of the last rows of each state league. Afterwards: exactly min(places, its clubs in the
/// list) clubs go up from each division (`club::RESERVE_DIVISION` = the state league's record) and its others stay;
/// those of the last rows are in the lower division.
unsafe extern "C" fn compare_bra_nat_first_exchange_state_league_clubs() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I0%%);
    let leagues: [usize; 12] = core::array::from_fn(|i| comp_object_of_key(BRA_STATE_LEAGUE_KEYS[i]));
    let lower: [usize; 12] = core::array::from_fn(|i| comp_record_of_key(BRA_STATE_LOWER_DIVISION_KEYS[i]));
    let mut theirs: Vec<Vec<usize>> = vec![Vec::new(); 12];
    let mut n = 0;
    let mut i = 0usize;
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 && n < 0xc8 {
        let c = club_at(i as isize).addr();
        if let Some(e) = lower.iter().position(|&b| b == read_usize(c + club::RESERVE_DIVISION) && b != 0) {
            theirs[e].push(c);
            n += 1;
        }
        i += 1;
    }
    let places: Vec<i32> = leagues.iter().map(|&l| read_i8(l + league::RELEGATED_COUNT) as i32).collect();
    let records: Vec<usize> = leagues.iter().map(|&l| read_usize(l + league::COMPETITION)).collect();
    let mut going_down: Vec<(usize, usize)> = Vec::new();
    for e in 0..12 {
        let l = leagues[e];
        let t = read_i16(l + league::CLUB_COUNT) as i32;
        for s in (t - places[e]).max(0)..t {
            going_down.push((read_usize(exe::league_get_table_row(l, s)), lower[e]));
        }
    }
    o();
    let mut errors = Vec::new();
    for e in 0..12 {
        let went_up = theirs[e].iter().filter(|&&c| read_usize(c + club::RESERVE_DIVISION) == records[e]).count();
        let stayed = theirs[e].iter().filter(|&&c| read_usize(c + club::RESERVE_DIVISION) == lower[e]).count();
        let expected = (places[e].max(0) as usize).min(theirs[e].len());
        if went_up != expected || went_up + stayed != theirs[e].len() {
            errors.push(format!("division {}: {} went up (expected {}), {} of {} stayed", e, went_up, expected, stayed, theirs[e].len()));
        }
    }
    for &(c, b) in &going_down {
        if read_usize(c + club::RESERVE_DIVISION) != b {
            errors.push(format!("relegated {:#x} in {:#x} (expected {:#x})", c, read_usize(c + club::RESERVE_DIVISION), b));
        }
    }
    SCOREBOARD.record(errors.is_empty(), || format!("bra_nat_first_exchange_state_league_clubs: {}", errors.join("; ")));
}

/// The third division league object and its record.
unsafe fn bra_nat_first_third_division() -> (usize, usize) {
    (comp_object_of_key(KEY_CLUB_COMP_BRAZILIAN_NATIONAL_THIRD_DIVISION), comp_record_of_key(KEY_CLUB_COMP_BRAZILIAN_NATIONAL_THIRD_DIVISION))
}

/// The Brazilian clubs without a national division, in database order, up to 160: a club (outside the national teams:
/// the ids below [`G_CLUB_COUNT`] − 2 × [`G_NATION_COUNT`]) with a nation (`club::NATION`), no division
/// (`club::DIVISION` = 0) and Brazil's nation (the nation id = [`KEY_NATION_BRAZIL`]).
unsafe fn bra_nat_first_clubs_without_division() -> Vec<usize> {
    let mut v = Vec::new();
    let mut i = 0i32;
    while i < read_usize(G_CLUB_COUNT) as i32 - 2 * read_usize(G_NATION_COUNT) as i32 && v.len() < 0xa0 {
        let c = club_at(i as isize).addr();
        let nation = read_usize(c + club::NATION);
        if nation != 0 && read_usize(c + club::DIVISION) == 0 && read_usize(nation) == read_usize(KEY_NATION_BRAZIL) {
            v.push(c);
        }
        i += 1;
    }
    v
}

/// At the Brazilian season turnover, the third division renews itself.
/// 1) A list of 160 (`exe::crt_malloc` of 0x280; out of memory: the error box of line 0x43a and return) with the
///    Brazilian clubs without a national division (`bra_nat_first_clubs_without_division`). Fewer than 16: the list is
///    freed and nothing changes.
/// 2) The list sorted by reputation and the first 48 shuffled (`exe::comp_util_sort_and_shuffle_top_clubs`(list, n,
///    16), cdecl: it draws); the first 16 enter the third division (`club::DIVISION` = the third division's record) and
///    those with a regional division (`club::RESERVE_DIVISION`) and no manager (`club::MANAGER` = 0) open the job
///    (`exe::manager_manager_add_job_vacancy`([`G_MANAGER_MANAGER`], club, 0), thiscall, ret 8).
/// 3) In the third division's table and in those of its seven groups (stages 0 to 6 of `league::STAGES`), the clubs of
///    the rows marked `league_table_row::STATUS` = 3 (eliminated in the first stage [inference]) leave the national
///    division (`club::DIVISION` = 0) and leave the division (`bra_nat_first_club_left_division`, with the record of the
///    table they leave).
/// 4) The list is freed (`exe::crt_free`).
/// Original: `bra_nat_first_refresh_third_division_clubs` (0x42b520, bra_nat_first.cpp, cdecl).
unsafe extern "C" fn bra_nat_first_refresh_third_division_clubs() {
    let (t, record) = bra_nat_first_third_division();
    let list = exe::crt_malloc(0x280);
    if list == 0 {
        return show_error_at(0x98aa30, 0x43a);
    }
    let v = bra_nat_first_clubs_without_division();
    for (k, &c) in v.iter().enumerate() {
        write_usize(list + 4 * k, c);
    }
    if v.len() < 0x10 {
        return exe::crt_free(list);
    }
    exe::comp_util_sort_and_shuffle_top_clubs(list, v.len() as u32, 0x10);
    for k in 0..0x10 { // cqv3: the 16 entering the third division
        let c = read_usize(list + 4 * k);
        write_usize(c + club::DIVISION, record);
        if read_usize(c + club::RESERVE_DIVISION) != 0 && read_usize(c + club::MANAGER) == 0 {
            exe::manager_manager_add_job_vacancy(read_usize(G_MANAGER_MANAGER), c, 0usize);
        }
    }
    covered!("cqv_3 third division renewed");
    for g in -1i32..7 {
        let s = if g == -1 { t } else { read_usize(read_usize(t + league::STAGES) + 4 * g as usize) };
        let mut e = 0i32;
        while e < read_i16(s + league::CLUB_COUNT) as i32 {
            let r = exe::league_get_table_row(s, e);
            if read_u8(r + league_table_row::STATUS) == 3 {
                let c = read_usize(r + league_table_row::CLUB);
                write_usize(c + club::DIVISION, 0); // cqv3: the eliminated club leaves the third division
                bra_nat_first_club_left_division(c, read_usize(s + league::COMPETITION));
            }
            e += 1;
        }
    }
    exe::crt_free(list);
}

/// The renewal comparer: runs only the original (it draws the shuffle and calls the manager and contract code; it
/// cannot run twice), with the plan made before: the clubs without a division (up to 160) and those of the rows
/// marked 3 in the eight tables. Afterwards, with 16 or more without a division: exactly 16 of them are in the third
/// division and the others without a division; the marked ones, without a division. With fewer than 16: nothing
/// changed for them.
unsafe extern "C" fn compare_bra_nat_first_refresh_third_division_clubs() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I1%%);
    let (t, record) = bra_nat_first_third_division();
    let v = bra_nat_first_clubs_without_division();
    let mut marked = Vec::new();
    if v.len() >= 0x10 {
        for g in -1i32..7 {
            let s = if g == -1 { t } else { read_usize(read_usize(t + league::STAGES) + 4 * g as usize) };
            for e in 0..read_i16(s + league::CLUB_COUNT) as i32 {
                let r = exe::league_get_table_row(s, e);
                if read_u8(r + league_table_row::STATUS) == 3 {
                    marked.push(read_usize(r + league_table_row::CLUB));
                }
            }
        }
    }
    o();
    let entered = v.iter().filter(|&&c| read_usize(c + club::DIVISION) == record).count();
    let outside = v.iter().filter(|&&c| read_usize(c + club::DIVISION) == 0).count();
    let expected = if v.len() >= 0x10 { 0x10 } else { 0 };
    let left = marked.iter().all(|&c| read_usize(c + club::DIVISION) == 0);
    SCOREBOARD.record(entered == expected && entered + outside == v.len() && left,
                     || format!("bra_nat_first_refresh_third_division_clubs: {} without a division, {} entered (expected {}), {} stayed out, {} marked, all out: {}", v.len(), entered, expected, outside, marked.len(), left));
}

/// The J2 season turnover. `league::SEASON_CLOSED` = 0 and the table freed (`exe::league_free_table`(this, 0)); the
/// round dates (`league::ROUND_DATES`) freed (`exe::crt_free`) and zeroed; with statistics (`league::COMP_STATS`),
/// `exe::comp_stats_archive_season_and_reset`(stats, 1); the year (`league::YEAR`, word) + 1; then, each with its own
/// error box (returning 0; the original returns the box's eax, which nobody reads) if it returns 0: the rules
/// (`comp_vtable::SET_UP_RULES`, thiscall, ret 0; line 0x196), the table (JapJ2::create_table 0x665030, thiscall, ret 0;
/// 0x19d), the fixtures (`exe::league_create_fixtures`; 0x1a3) and the sort (`exe::league_sort_table`(this, 0); 0x1aa);
/// the predicted positions (`comp_vtable::SET_PREDICTED_POSITIONS`), the board expectations
/// (`exe::league_set_club_season_expectations`) and the referee renewal (OfficialsManager::renew_officials
/// `FN_OFFICIALSMANAGER_RENEW_OFFICIALS`, thiscall([`G_OFFICIALS_MANAGER`], `league::COMPETITION`), ret 4), whose eax it
/// returns. The J2 has no stages besides the league.
/// Original: `JapJ2::start_new_season` (0x6651f0, jap_j2.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn jap_j2_start_new_season(this: usize) -> u32 {
    jap_j2_start_new_season_years(this, 1) // cqv3: the J2's next year
}

/// The body of `jap_j2_start_new_season`, moving the year by `years`.
unsafe fn jap_j2_start_new_season_years(this: usize, years: i16) -> u32 {
    write_u32(this + league::SEASON_CLOSED, 0);
    exe::league_free_table(this, 0usize);
    let l = read_usize(this + league::ROUND_DATES);
    if l != 0 {
        exe::crt_free(l);
        write_usize(this + league::ROUND_DATES, 0);
    }
    let stats = read_usize(this + league::COMP_STATS);
    if stats != 0 {
        exe::comp_stats_archive_season_and_reset(stats, 1);
    }
    let a = read_i16(this + league::YEAR);
    write_i16(this + league::YEAR, a.wrapping_add(years));
    covered!("cqv_3 j2 turnover");
    let set_up_rules: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::SET_UP_RULES));
    if set_up_rules(this) == 0 {
        show_error_at(0x9eaedc, 0x196);
        return 0;
    }
    let create_table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x665030usize);
    if create_table(this) == 0 {
        show_error_at(0x9eaedc, 0x19d);
        return 0;
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(0x9eaedc, 0x1a3);
        return 0;
    }
    if exe::league_sort_table(this, 0usize) == 0 {
        show_error_at(0x9eaedc, 0x1aa);
        return 0;
    }
    let set_predicted_positions: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::SET_PREDICTED_POSITIONS));
    set_predicted_positions(this);
    exe::league_set_club_season_expectations(this);
    // the full eax is returned, so the u8 wrapper `exe::officialsmanager_renew_officials` is not used
    let renew_officials: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_OFFICIALSMANAGER_RENEW_OFFICIALS);
    renew_officials(read_usize(G_OFFICIALS_MANAGER), read_usize(this + league::COMPETITION)) // cqv3: the J2 referee renewal
}

/// The turnover comparer: runs only the original (it frees the table and the round dates, allocates the new ones and
/// draws the fixtures and referees; it cannot run twice), checking what only it guarantees: the year + 1, the same
/// statistics block and, with the rules, table and fixtures built, the new table (`league::TABLE`).
unsafe extern "thiscall" fn compare_jap_j2_start_new_season(this: usize) -> u32 {
    let before = read_i16(this + league::YEAR);
    let stats = read_usize(this + league::COMP_STATS);
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%)(this);
    let after = read_i16(this + league::YEAR);
    let same = after == before.wrapping_add(1) && read_usize(this + league::COMP_STATS) == stats && read_usize(this + league::TABLE) != 0;
    SCOREBOARD.record(same, || format!("jap_j2_start_new_season({:#x}) year before={} after={} +8={:#x} +0xb1={:#x}", this, before, after, read_usize(this + league::COMP_STATS), read_usize(this + league::TABLE)));
    r
}
