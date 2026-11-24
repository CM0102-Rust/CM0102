// ---------- cx03_8: the round set-up of the Scottish Challenge Cup and the Welsh Cup, and the Scottish League Cup entrants (sco_chal_cup.cpp, wel_cup.cpp, sco_lge_cup.cpp) ----------

/// The rounds of the Scottish Challenge Cup and of the two other cups that share the virtual (vtables 0x96a360 and
/// 0x96cd30): a prize fund of 200,000 and the round table (`set_up_prize_rounds`). Returns 1.
/// Original: `ScoChalCup::set_up_rounds` (0x7ed2e0, sco_chal_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn sco_chal_cup_set_up_rounds(this: usize) -> i32 {
    covered!("cx03_8 cup rounds with a 200,000 prize fund");
    set_up_prize_rounds(Cup(this), 200_000)
}

/// The rounds of the Welsh Cup: a prize fund of 50,000 and the round table (`set_up_prize_rounds`). Returns 1.
/// Original: `WelCup::set_up_rounds` (0x924e00, wel_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wel_cup_set_up_rounds(this: usize) -> i32 {
    covered!("cx03_8 welsh cup rounds");
    set_up_prize_rounds(Cup(this), 50_000)
}

/// Whether the club plays in the Champions League or the UEFA Cup this season (`club.continental_comp_id()`).
unsafe fn in_uefa_club_cups(c: Club) -> bool {
    let id = c.continental_comp_id() as usize;
    id == read_usize(KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE) || id == read_usize(KEY_CLUB_COMP_UEFA_CUP)
}

/// One pass of the Scottish League Cup entrants over the club table: each club of `division` that is not in the
/// Champions League nor the UEFA Cup enters at slot `k` with the class `class(club)`. With `limited`, the pass stops
/// once the list is full; without it (the first three passes) the original does not check the list size.
unsafe fn sco_lge_cup_enter_division(this: Cup, k: &mut i32, division: usize, limited: bool, class: impl Fn(Club) -> u8) {
    let mut i: i16 = 0;
    while (i as i32) < read_i32(G_CLUB_COUNT) {
        if limited && *k >= this.team_count() as i32 {
            return;
        }
        let c = Club(eng_cc_cup_club_by_index(i));
        if division_comp_id(c.addr()) == Some(read_usize(division)) && !in_uefa_club_cups(c) {
            eng_cc_cup_write_entrant(this, *k, c.addr(), class(c));
            *k += 1;
        }
        i = i.wrapping_add(1);
    }
}

/// rule: the Scottish League Cup takes 40 clubs in 1999 and 42 after: the four Scottish divisions from the Third up
/// (the Third, Second and First whole, the Premier until the list is full), then the Scottish clubs playing in the
/// Champions League or the UEFA Cup, who get a bye; with more than 7 of those, the one whose European cup has the lowest
/// reputation loses the bye, one at a time, down to 7; the byes are capped at 12. Each club's draw class is its last
/// league position plus 0x50 in the Third, 0x3c in the Second, 0x28 in the First (0x28 alone for a club relegated from
/// the Premier) and 0x14 in the Premier; the European clubs 0.
///
/// `cup::TEAM_COUNT` = 0x28 when the year (`cup::YEAR`) is 1999, else 0x2a; the byes byte +0xb2 = 0; the list
/// (`exe::crt_malloc` of 6 per team into `cup::TEAM_TABLE`; out of memory, the box of line 0x191 and −1). The passes
/// (`sco_lge_cup_enter_division`) of the Third, Second and First (unchecked) and the Premier (until full); then the
/// European pass: each club with a division, while the list is not full, in one of the four Scottish divisions and
/// playing in the Champions League or the UEFA Cup: the byes byte + 1 and the club enters with class 0, stopping once the
/// list is full. The first `cup::TEAM_COUNT` − byes entries are sorted (`exe::crt_qsort`, 6-byte entries,
/// `cup_compare_entries_by_level_desc` 0x522190, cdecl). Then, while the byes (a signed byte) are above 7: among the
/// last byes entries the one whose European cup (`club.continental_comp_id()`, the record of `G_CLUB_COMP_TABLE`) has
/// the lowest reputation (the first of the lowest; none below 20000, or an empty range: the box of line 0x213 and the
/// loop ends) swaps its club with the first of those entries (only the club pointers), and the byes go down by one. A
/// list not full returns 0; otherwise byes above 12 become 12 and it returns 1.
/// The original reads the relegated club's last division without checking it (a club of the First with no last
/// division would crash it); the copy treats it as not relegated.
/// Original: `ScoLgeCup::collect_entrants` (0x7f0400, sco_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn sco_lge_cup_collect_entrants(this: usize) -> i32 {
    collect_scottish_league_cup_entrants(Cup(this)).0
}

/// The body of `sco_lge_cup_collect_entrants`: returns the result and the number of entrants.
unsafe fn collect_scottish_league_cup_entrants(this: Cup) -> (i32, i32) {
    let byes = this.addr() + 0xb2; // the signed count of byes for the clubs in the European cups
    this.set_team_count(if this.year() == 1999 { 0x28 } else { 0x2a });
    write_u8(byes, 0);
    let list = exe::crt_malloc(this.team_count() as i32 as u32 * 6);
    this.set_team_table(list);
    if list == 0 {
        show_error_at(0xa7af38, 0x191);
        return (-1, 0);
    }
    let mut k = 0i32;
    sco_lge_cup_enter_division(this, &mut k, KEY_CLUB_COMP_SCOTTISH_THIRD_DIVISION, false, |c| c.last_position().wrapping_add(0x50));
    sco_lge_cup_enter_division(this, &mut k, KEY_CLUB_COMP_SCOTTISH_SECOND_DIVISION, false, |c| c.last_position().wrapping_add(0x3c));
    sco_lge_cup_enter_division(this, &mut k, KEY_CLUB_COMP_SCOTTISH_FIRST_DIVISION, false, |c| {
        if came_from_division(c.addr(), KEY_CLUB_COMP_SCOTTISH_PREMIER_DIVISION) {
            covered!("cx03_8 relegated from the scottish premier");
            0x28
        } else {
            c.last_position().wrapping_add(0x28)
        }
    });
    sco_lge_cup_enter_division(this, &mut k, KEY_CLUB_COMP_SCOTTISH_PREMIER_DIVISION, true, |c| c.last_position().wrapping_add(0x14));
    let scottish = [KEY_CLUB_COMP_SCOTTISH_PREMIER_DIVISION, KEY_CLUB_COMP_SCOTTISH_FIRST_DIVISION, KEY_CLUB_COMP_SCOTTISH_SECOND_DIVISION, KEY_CLUB_COMP_SCOTTISH_THIRD_DIVISION];
    let mut i: i16 = 0;
    while (i as i32) < read_i32(G_CLUB_COUNT) {
        let c = Club(eng_cc_cup_club_by_index(i));
        if let Some(d) = division_comp_id(c.addr()) {
            if k != this.team_count() as i32 && scottish.iter().any(|&key| d == read_usize(key)) && in_uefa_club_cups(c) {
                covered!("cx03_8 scottish club in the european cups");
                write_u8(byes, read_u8(byes).wrapping_add(1));
                eng_cc_cup_write_entrant(this, k, c.addr(), 0);
                k += 1;
                if k == this.team_count() as i32 {
                    break;
                }
            }
        }
        i = i.wrapping_add(1);
    }
    exe::crt_qsort(this.team_table(), (this.team_count() as i32 - read_u8(byes) as i8 as i32) as u32, 6, 0x522190usize);
    while (read_u8(byes) as i8) > 7 {
        let count = read_u8(byes) as i8 as i32;
        let start = (k - count) as i16;
        let mut lowest: i16 = 20000;
        let mut best: i32 = -1;
        let mut c = start;
        if (c as i32) < k {
            while (c as i32) < k {
                let club = Club(read_usize(this.team_table() + c as usize * 6));
                let cup = Competition(read_usize(G_CLUB_COMP_TABLE).wrapping_add((club.continental_comp_id() as usize).wrapping_mul(0x6b)));
                if cup.reputation() < lowest {
                    lowest = cup.reputation();
                    best = c as i32;
                }
                c = c.wrapping_add(1);
            }
        }
        if best as i16 == -1 {
            show_error_at(0xa7af38, 0x213);
            break;
        }
        covered!("cx03_8 european bye given up");
        let first = this.team_table() + (k - count) as usize * 6;
        let other = this.team_table() + (best as i16) as usize * 6;
        let club = read_usize(first);
        write_usize(first, read_usize(other));
        write_usize(other, club);
        write_u8(byes, read_u8(byes).wrapping_sub(1));
    }
    if k != this.team_count() as i32 {
        return (0, k);
    }
    if (read_u8(byes) as i8) > 12 {
        write_u8(byes, 12);
    }
    (1, k)
}

pub(crate) unsafe extern "thiscall" fn compare_sco_chal_cup_set_up_rounds(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I0%%);
    compare_set_up_prize_rounds("sco_chal_cup_set_up_rounds", Cup(this), || o(this), || sco_chal_cup_set_up_rounds(this))
}

pub(crate) unsafe extern "thiscall" fn compare_wel_cup_set_up_rounds(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I1%%);
    compare_set_up_prize_rounds("wel_cup_set_up_rounds", Cup(this), || o(this), || wel_cup_set_up_rounds(this))
}

/// Comparer for `sco_lge_cup_collect_entrants`: side by side over the team count and the byes byte, the return and the
/// list (`eng_cc_cup_compare_entrant_list`, the copy's list freed).
pub(crate) unsafe extern "thiscall" fn compare_sco_lge_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I2%%);
    let cup = Cup(this);
    eng_cc_cup_compare_entrant_list("sco_lge_cup_collect_entrants", cup, &[(this + cup::TEAM_COUNT, 2), (this + 0xb2, 1)], || o(this),
        || collect_scottish_league_cup_entrants(cup), |_, n| 6 * n.max(0) as usize)
}
