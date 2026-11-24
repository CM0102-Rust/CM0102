// ---------- sbE_12: the Spanish B team clashes at the turnover; the mould of the Spanish and Portuguese ones (spa_first.cpp) ----------

/// A nation's B team rule at the turnover: the second division, the division below it played in groups (the league
/// itself for the first group and its stages 0, 1, ... for the others) and the boxes of its source file (the second
/// division row missing, a group row missing in the top division's pass and in the second division's pass).
pub(crate) struct BTeamLimits {
    pub(crate) second_key: usize,
    pub(crate) groups_key: usize,
    pub(crate) group_keys: &'static [usize],
    pub(crate) file: usize,
    pub(crate) rows: [u32; 3],
}

/// The Spanish B team rule: the Second Division, Segunda B in groups 1 to 4, spa_first.cpp.
pub(crate) const SPANISH_B_TEAM_LIMITS: BTeamLimits = BTeamLimits {
    second_key: KEY_CLUB_COMP_SPANISH_SECOND_DIVISION,
    groups_key: KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B,
    group_keys: &[KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B1, KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B2, KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B3, KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B4],
    file: 0xa82844,
    rows: [0x4c1, 0x4fc, 0x52e],
};

/// The leagues of the groups of `p` (the groups' league for the first, its stage k − 1 for group k).
unsafe fn b_team_group_leagues(p: &BTeamLimits) -> Vec<League> {
    let b = League(comp_by_global(p.groups_key));
    (0..p.group_keys.len()).map(|k| if k == 0 { b } else { League(read_usize(b.stages() + 4 * (k - 1))) }).collect()
}

/// The index of the group whose key has the id of the club's reserve division (`club::RESERVE_DIVISION`, read without a
/// null check), or None.
unsafe fn b_team_group_of(p: &BTeamLimits, club: usize) -> Option<usize> {
    let id = read_u32(read_usize(club + club::RESERVE_DIVISION));
    p.group_keys.iter().position(|&key| id == read_u32(key))
}

/// The body of the Spanish and Portuguese B team rules (`BTeamLimits`).
///
/// The top division's rows (16-bit index): a partner (`exe::club_get_linked_b_team`(club, &flag, 1), cdecl) with the
/// flag not 0 and a division. In the second division: its row there (League::find_club_row(partner, 0); none: the box
/// `rows[0]`); not already 3: a promoted one (status 0) counts a place to hand over; with the senior club relegated (3)
/// the row gets 0xfb, the second division sends the override news
/// (`exe::comp_send_promotion_relegation_override_news`(the second division; this competition, partner, club, 1)) and
/// one more club is saved, otherwise the row gets 0xfe. In the groups' division (with a reserve division) and the senior
/// club relegated: its row in its group (`b_team_group_of`; with no group the row of the last lookup is kept, at first
/// the last group's object itself: a defect of the original; none: the box `rows[1]`); not 3: a promoted one counts a
/// place for its group, and the row gets 0xfe.
///
/// The second division's rows (index in a dword): a partner in the groups' division with a reserve division: its group
/// row (as above; the box `rows[2]`); not 3: a promoted one counts for its group; with the senior club relegated (3) the
/// row gets 3 and the groups' division sends the override news (this competition, partner, club, 1), otherwise 0xfe.
///
/// Then, in the second division's rows while places or saves remain: an undecided row (0xff) takes a held-back place
/// (0); otherwise a relegated row (3) is saved (0xfe). In each group, its undecided rows take its places. Last, the
/// second division's rows marked 0xfb become 3.
pub(crate) unsafe fn resolve_b_team_clashes(this: League, p: &BTeamLimits) {
    let second = League(comp_by_global(p.second_key));
    let groups = b_team_group_leagues(p);
    let mut freed = vec![0i8; groups.len()];
    let (mut held, mut saved) = (0i8, 0i8);
    let mut found = groups[groups.len() - 1].addr(); // the original's register, kept from one lookup to the next
    let mut flag: u32 = 0;
    let status = |row: usize| read_u8(row + league_table_row::STATUS);
    let count_place = |freed: &mut Vec<i8>, partner: usize| {
        if let Some(g) = b_team_group_of(p, partner) {
            freed[g] += 1;
        }
    };
    let mut k: i16 = 0;
    while k < this.club_count() {
        let row = this.table() + k as usize * league_table_row::SIZE;
        let club = read_usize(row);
        k += 1;
        let partner = exe::club_get_linked_b_team(club, &mut flag, 1);
        if partner == 0 || flag == 0 || read_usize(partner + club::DIVISION) == 0 {
            continue;
        }
        let division = read_u32(read_usize(partner + club::DIVISION));
        if division == read_u32(p.second_key) {
            found = exe::league_find_club_row(second, partner, 0);
            if found == 0 {
                show_error_at(p.file, p.rows[0]);
                continue;
            }
            if status(found) == 3 {
                continue;
            }
            if status(found) == 0 {
                held += 1;
            }
            if status(row) == 3 {
                covered!("sbE_12 b team down with its senior club");
                write_u8(found + league_table_row::STATUS, 0xfb); // sbE_12: down with the senior club
                exe::comp_send_promotion_relegation_override_news(second, this.competition(), partner, club, 1);
                saved += 1;
            } else {
                write_u8(found + league_table_row::STATUS, 0xfe);
            }
            continue;
        }
        if read_usize(partner + club::RESERVE_DIVISION) == 0 || division != read_u32(p.groups_key) || status(row) != 3 {
            continue;
        }
        if let Some(g) = b_team_group_of(p, partner) {
            found = exe::league_find_club_row(groups[g], partner, 0);
        }
        if found == 0 {
            show_error_at(p.file, p.rows[1]);
            continue;
        }
        if status(found) == 3 {
            continue;
        }
        if status(found) == 0 {
            count_place(&mut freed, partner);
        }
        write_u8(found + league_table_row::STATUS, 0xfe);
    }
    let mut k: i32 = 0;
    while (k as i16) < second.club_count() {
        let row = second.table() + k as i16 as usize * league_table_row::SIZE;
        let club = read_usize(row);
        k += 1;
        let partner = exe::club_get_linked_b_team(club, &mut flag, 1);
        if partner == 0 || flag == 0 || read_usize(partner + club::DIVISION) == 0 || read_usize(partner + club::RESERVE_DIVISION) == 0
            || read_u32(read_usize(partner + club::DIVISION)) != read_u32(p.groups_key) {
            continue;
        }
        if let Some(g) = b_team_group_of(p, partner) {
            found = exe::league_find_club_row(groups[g], partner, 0);
        }
        if found == 0 {
            show_error_at(p.file, p.rows[2]);
            continue;
        }
        if status(found) == 3 {
            continue;
        }
        if status(found) == 0 {
            count_place(&mut freed, partner);
        }
        if status(row) == 3 {
            covered!("sbE_12 group b team down");
            write_u8(found + league_table_row::STATUS, 3);
            exe::comp_send_promotion_relegation_override_news(groups[0], this.competition(), partner, club, 1);
        } else {
            write_u8(found + league_table_row::STATUS, 0xfe);
        }
    }
    let mut k: i16 = 0;
    while k < second.club_count() && (held != 0 || saved != 0) {
        let row = second.table() + k as usize * league_table_row::SIZE;
        if held != 0 && status(row) == 0xff {
            write_u8(row + league_table_row::STATUS, 0); // sbE_12: the place handed over
            held -= 1;
        } else if saved != 0 && status(row) == 3 {
            write_u8(row + league_table_row::STATUS, 0xfe);
            saved -= 1;
        }
        k += 1;
    }
    for (g, &league) in groups.iter().enumerate() {
        let mut places = freed[g];
        let mut k: i16 = 0;
        while k < league.club_count() && places != 0 {
            let row = league.table() + k as usize * league_table_row::SIZE;
            if status(row) == 0xff {
                write_u8(row + league_table_row::STATUS, 0);
                places -= 1;
            }
            k += 1;
        }
    }
    let mut k: i16 = 0;
    while k < second.club_count() {
        let row = second.table() + k as usize * league_table_row::SIZE;
        if status(row) == 0xfb {
            write_u8(row + league_table_row::STATUS, 3);
        }
        k += 1;
    }
}

/// Comparer body of the B team rules: only the original runs (it marks rows and sends news); afterwards no second
/// division row may be left with the temporary mark 0xfb.
pub(crate) unsafe fn compare_b_team_clashes(name: &str, p: &BTeamLimits, original: impl FnOnce()) {
    original();
    let second = League(comp_by_global(p.second_key));
    let marked = (0..second.club_count().max(0) as usize).filter(|&k| read_u8(second.table() + k * league_table_row::SIZE + league_table_row::STATUS) == 0xfb).count();
    SCOREBOARD.record(marked == 0, || format!("{}: {} rows left marked 0xfb", name, marked));
}

/// Rule: at the Spanish turnover with Segunda B, no B team may end in the same division as its senior club. A B team
/// in the Second Division whose senior club goes down from the First is relegated with it (with the news); one that
/// would go up is held back, and its place goes to the next undecided club. A Segunda B team whose senior club goes down
/// from the First (or is in the Second) is held back from promotion (or, when the senior club comes down to Segunda B,
/// relegated with the news); its group's promotion place goes to the next undecided club. Each B team relegated from
/// the Second Division saves one relegated club.
///
/// `resolve_b_team_clashes` with `SPANISH_B_TEAM_LIMITS` (Segunda B in groups 1 to 4; the spa_first.cpp (0xa82844) boxes
/// at lines 0x4c1, 0x4fc and 0x52e).
/// Original: `SpaFirst::resolve_b_team_clashes` (0x850970, spa_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn spa_first_resolve_b_team_clashes(this: usize) {
    resolve_b_team_clashes(League(this), &SPANISH_B_TEAM_LIMITS);
}

/// Comparer for `spa_first_resolve_b_team_clashes` (`compare_b_team_clashes`).
pub(crate) unsafe extern "thiscall" fn compare_spa_first_resolve_b_team_clashes(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    compare_b_team_clashes("spa_first_resolve_b_team_clashes", &SPANISH_B_TEAM_LIMITS, || o(this));
}
