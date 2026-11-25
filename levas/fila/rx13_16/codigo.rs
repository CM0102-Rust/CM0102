// ---------- rx13_16: the Spanish Segunda B's exchange with the lower division and its four groups; the Swedish Second Division's moves (spa_first.cpp, swe_prm.cpp) ----------

/// The `qsort` comparers of two club pointers by their stadium city's latitude and longitude.
pub(crate) const CLUB_LATITUDE_COMPARE: usize = 0x53fd40;
pub(crate) const CLUB_LONGITUDE_COMPARE: usize = 0x53fe50;
/// "E:\dev\CM3\cm3 00-01\cm3\code\comp\leagues\spa_first.cpp" (the error boxes).
const SPA_FIRST_CPP: usize = 0xa82844;

/// The club table's clubs (the national teams at its end, two per nation, left out), as the Spanish turnover reads it.
unsafe fn club_sides_count() -> i32 {
    read_i32(G_CLUB_COUNT) - 2 * read_i32(G_NATION_COUNT)
}

/// Group `k` of a league split in groups: the league itself for −1, its stage `k` otherwise (`league::STAGES`).
unsafe fn league_group(main: League, k: i8) -> League {
    if k < 0 { main } else { League(read_usize(main.stages() + k as usize * 4)) }
}

/// rule: at the Spanish turnover with a played Segunda B, the clubs relegated from its four groups go down to the
/// generic lower division, and as many Spanish clubs of the lower division (one more when a group had 19 clubs), the
/// strongest first, go up into the Segunda B; a reserve side whose first team is already in the Segunda B is skipped
///
/// Up to 200 candidates in club table order (`club_sides_count`): division [`KEY_CLUB_COMP_A_LOWER_DIVISION`] and
/// nation [`KEY_NATION_SPAIN`]. For the Segunda B ([`KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B`]) and its stages 0 to 2
/// (`league_group`): a group of 19 clubs sets the extra place; every row with `league_table_row::STATUS` 3 sends its club
/// down (`exe::league_relegate_club_to_division`(this, club, the lower division's record, 1)) and clears its
/// `club::RESERVE_DIVISION` [the dword at +0x60: here the Segunda B group]. The places are the relegated plus the extra
/// one. Fewer candidates: the box of spa_first.cpp line 0x3b7 (the relegations stay). Otherwise the candidates are
/// sorted (`exe::comp_util_sort_and_shuffle_top_clubs`(list, n, places)) and, in that order while places are left,
/// each goes up (`exe::league_move_club_to_division`(this, club, the Segunda B record, 1)), unless it is a B team
/// (`exe::club_get_linked_b_team`(club, &is_a, 1) gives a partner and is_a is 0) whose partner's division is the Segunda
/// B.
/// Original: `spa_first_exchange_second_division_b_clubs` (0x850300, spa_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn spa_first_exchange_second_division_b_clubs(this: usize) {
    let this = League(this);
    let mut list = [0usize; 200];
    let mut n = 0usize;
    for i in 0..club_sides_count().max(0) as usize {
        let c = read_usize(G_CLUB_TABLE) + i * club::SIZE;
        let d = read_usize(c + club::DIVISION);
        let nation = read_usize(c + club::NATION);
        if d != 0 && read_u32(d) == read_u32(KEY_CLUB_COMP_A_LOWER_DIVISION) && nation != 0 && read_u32(nation) == read_u32(KEY_NATION_SPAIN) {
            list[n] = c;
            n += 1;
            if n == 200 {
                break;
            }
        }
    }
    let second_b = League(comp_by_global(KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B));
    let mut places = 0i16;
    let mut extra = false;
    for k in -1i8..3 {
        let g = league_group(second_b, k);
        if g.club_count() == 0x13 {
            extra = true; // rx13_16: a group of 19
        }
        for r in 0..g.club_count().max(0) as usize {
            let row = g.table() + r * 0x3b;
            if read_u8(row + league_table_row::STATUS) == 3 {
                exe::league_relegate_club_to_division(this, read_usize(row), comp_record_of_key(KEY_CLUB_COMP_A_LOWER_DIVISION), 1);
                places += 1;
                write_u32(read_usize(row) + club::RESERVE_DIVISION, 0);
            }
        }
    }
    if extra {
        places += 1;
    }
    if (n as i16) < places {
        show_error_at(SPA_FIRST_CPP, 0x3b7);
        return;
    }
    covered!("rx13_16 segunda b exchange");
    exe::comp_util_sort_and_shuffle_top_clubs(list.as_mut_ptr() as usize, n as u32, places as u16 as u32);
    for &c in &list[..n] {
        if places == 0 {
            break;
        }
        let mut is_a = 0u32;
        let partner = exe::club_get_linked_b_team(c, &mut is_a, 1);
        if partner != 0 && is_a == 0 {
            let d = read_usize(partner + club::DIVISION);
            if d != 0 && read_u32(d) == read_u32(KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B) {
                covered!("rx13_16 b team kept down");
                continue;
            }
        }
        exe::league_move_club_to_division(this, c, comp_record_of_key(KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B), 1);
        places -= 1;
    }
}

/// rule: the 80 Segunda B clubs are dealt into its four groups of 20 by geography: sorted from south to north by their
/// stadium city, the 20 southernmost go to group 4; the other 60, sorted from west to east, 20 each to groups 1, 2 and
/// 3; with a number other than 80 nothing is dealt (an error box)
///
/// In club table order (`club_sides_count`, stopping at 80), the clubs whose division is the Segunda B. Not 80: the
/// box of spa_first.cpp line 0x3f7. Otherwise `exe::crt_qsort`(list, 80, 4, `CLUB_LATITUDE_COMPARE`), then the last 60
/// with `CLUB_LONGITUDE_COMPARE`; entry k's `club::RESERVE_DIVISION` [the dword at +0x60: the Segunda B group] becomes
/// the record of group 4 (k < 20), 1, 2 or 3.
/// Original: `spa_first_assign_second_division_b_groups` (0x850530, spa_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn spa_first_assign_second_division_b_groups(_this: usize) {
    let groups = [KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B4, KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B1,
        KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B2, KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B3].map(|k| comp_record_of_key(k));
    let mut list = [0usize; 80];
    let mut n = 0usize;
    for i in 0..club_sides_count().max(0) as usize {
        let c = read_usize(G_CLUB_TABLE) + i * club::SIZE;
        let d = read_usize(c + club::DIVISION);
        if d != 0 && read_u32(d) == read_u32(KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B) {
            list[n] = c;
            n += 1;
            if n == 80 {
                break;
            }
        }
    }
    if n != 80 {
        show_error_at(SPA_FIRST_CPP, 0x3f7);
        return;
    }
    covered!("rx13_16 segunda b groups");
    exe::crt_qsort(list.as_mut_ptr() as usize, 80, 4, CLUB_LATITUDE_COMPARE);
    exe::crt_qsort(list[20..].as_mut_ptr() as usize, 60, 4, CLUB_LONGITUDE_COMPARE); // rx13_16: the northern 60 by longitude
    for (k, &c) in list.iter().enumerate() {
        write_usize(c + club::RESERVE_DIVISION, groups[k / 20]);
    }
}

/// rule: at the Swedish turnover with a playable Second Division, the winners of its promotion play-off go up into it,
/// the clubs relegated from its four sections go down to the lower division, and the twelve clubs it kept for promotion
/// go up
///
/// The Second Division is the object of [`KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION`]. Its stage 6 (the play-off, a cup):
/// every entrant of `cup::TEAM_TABLE` (6-byte entries, `cup::TEAM_COUNT` of them) with the outcome byte (+5) 1 whose
/// club's division is not the Second Division's record goes up (`exe::league_move_club_to_division`(this, club, record,
/// 1)). Then the Second Division and its stages 0 to 3 (`league_group`): every row with `league_table_row::STATUS` 3
/// goes down (`exe::league_relegate_club_to_division`(this, club, the [`KEY_CLUB_COMP_SWEDISH_LOWER_DIVISION`] record,
/// 1)). Then the twelve club ids at +0xee (the clubs promoted into the Second Division) each go up to its record.
/// Original: `swe_prm_apply_second_division_moves` (0x890b40, swe_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn swe_prm_apply_second_division_moves(this: usize) {
    let this = League(this);
    let second = League(comp_by_global(KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION));
    let record = second.competition().addr();
    let play_off = read_usize(second.stages() + 0x18); // stage 6: the promotion play-off
    for k in 0..read_i16(play_off + cup::TEAM_COUNT).max(0) as usize {
        let e = read_usize(play_off + cup::TEAM_TABLE) + k * 6;
        let c = read_usize(e);
        if read_u8(e + 5) == 1 && read_usize(c + club::DIVISION) != record {
            covered!("rx13_16 swedish play-off winner up");
            exe::league_move_club_to_division(this, c, record, 1);
        }
    }
    for k in -1i8..4 {
        let g = league_group(second, k);
        for r in 0..g.club_count().max(0) as usize {
            let row = g.table() + r * 0x3b;
            if read_u8(row + league_table_row::STATUS) == 3 {
                exe::league_relegate_club_to_division(this, read_usize(row), comp_record_of_key(KEY_CLUB_COMP_SWEDISH_LOWER_DIVISION), 1);
            }
        }
    }
    covered!("rx13_16 swedish second division moves");
    for k in 0..0xc {
        let id = read_u32(second.addr() + 0xee + k * 4); // the twelve promoted club ids
        exe::league_move_club_to_division(this, club_at(id as isize), comp_record_of_key(KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION), 1);
    }
}

/// Comparer for `spa_first_exchange_second_division_b_clubs`: only the original runs (it draws in the sort and moves
/// clubs); afterwards no relegated row of the four groups may still hold a club in the Segunda B.
pub(crate) unsafe extern "thiscall" fn compare_spa_first_exchange_second_division_b_clubs(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this);
    let second_b = League(comp_by_global(KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B));
    let record = comp_record_of_key(KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B);
    let wrong: Vec<usize> = (-1i8..3).flat_map(|k| {
        let g = league_group(second_b, k);
        (0..g.club_count().max(0) as usize).map(move |r| g.table() + r * 0x3b)
    }).filter(|&row| read_u8(row + league_table_row::STATUS) == 3 && read_usize(read_usize(row) + club::DIVISION) == record)
        .map(|row| read_usize(row)).collect();
    SCOREBOARD.record(wrong.is_empty(), || format!("spa_first_exchange_second_division_b_clubs({:#x}) relegated clubs still in the Segunda B: {:x?}", this, wrong));
}

/// Comparer for `spa_first_assign_second_division_b_groups`: side by side over the +0x60 dword of the 80 clubs (the
/// copy's sorts on a copy of the list give the same order).
pub(crate) unsafe extern "thiscall" fn compare_spa_first_assign_second_division_b_groups(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    let regions: Vec<(usize, usize)> = (0..club_sides_count().max(0) as usize)
        .map(|i| read_usize(G_CLUB_TABLE) + i * club::SIZE)
        .filter(|&c| { let d = read_usize(c + club::DIVISION); d != 0 && read_u32(d) == read_u32(KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B) })
        .take(80)
        .map(|c| (c + club::RESERVE_DIVISION, 4))
        .collect();
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(this), || spa_first_assign_second_division_b_groups(this));
    SCOREBOARD.record(equal, || format!("spa_first_assign_second_division_b_groups {:?}", first_differences(&mo, &mr)));
}

/// Comparer for `swe_prm_apply_second_division_moves`: only the original runs (it moves clubs); afterwards the twelve
/// stored clubs must be in the Second Division.
pub(crate) unsafe extern "thiscall" fn compare_swe_prm_apply_second_division_moves(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this);
    let second = comp_by_global(KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION);
    let record = comp_record_of_key(KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION);
    let wrong: Vec<u32> = (0..0xc).map(|k| read_u32(second + 0xee + k * 4))
        .filter(|&id| read_usize(club_at(id as isize).addr() + club::DIVISION) != record).collect();
    SCOREBOARD.record(wrong.is_empty(), || format!("swe_prm_apply_second_division_moves({:#x}) stored clubs outside the Second Division: {:?}", this, wrong));
}
