// ---------- rx13_15: the Conference's exchange with its feeder divisions (eng_prm.cpp) ----------

/// The divisions whose clubs are not Conference candidates: the generic lower division, the four Football League
/// divisions and the Conference itself.
const CONFERENCE_FEEDER_EXCLUDED: [usize; 6] = [KEY_CLUB_COMP_A_LOWER_DIVISION, KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION, KEY_CLUB_COMP_ENGLISH_FIRST_DIVISION,
    KEY_CLUB_COMP_ENGLISH_SECOND_DIVISION, KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION, KEY_CLUB_COMP_ENGLISH_CONFERENCE];

/// Whether a club may go up to the Conference: it has a division and a nation, the division is not one of
/// `CONFERENCE_FEEDER_EXCLUDED` and the nation is England.
unsafe fn conference_candidate(c: Club) -> bool {
    let d = read_usize(c.addr() + club::DIVISION);
    let nation = read_usize(c.addr() + club::NATION);
    d != 0 && nation != 0 && !CONFERENCE_FEEDER_EXCLUDED.iter().any(|&k| read_u32(d) == read_u32(k)) && read_u32(nation) == read_u32(KEY_NATION_ENGLAND)
}

/// rule: at the season turnover with a played Conference, the best English club of each of up to three feeder
/// divisions below it goes up to the Conference (one per division, the strongest first), and the Conference's
/// relegated clubs go down into those divisions, the first relegated to the first division that sent a club up, and so
/// on, three at most
///
/// The Conference is the object of [`KEY_CLUB_COMP_ENGLISH_CONFERENCE`] (`G_COMP_OBJECT_TABLE`). A list of
/// `G_CLUB_COUNT` × 4 bytes (`exe::crt_malloc`; out of memory, the box of eng_prm.cpp line 0x2f9 and it returns) takes,
/// in club table order, every `conference_candidate`. None: the list is freed. Otherwise it is sorted
/// (`exe::comp_util_sort_and_shuffle_top_clubs`(list, n, 3)) and, in that order, a club whose division already sent one
/// is skipped; else its division takes the first free slot of three and the club goes up (League::move_club_to_division
/// `exe::league_move_club_to_division`(this, club, the Conference record, 1)); the loop ends when the third slot is
/// taken. Then, while a slot is left, every Conference row with `league_table_row::STATUS` 3 sends its club down to the
/// next slot's division (League::relegate_club_to_division `exe::league_relegate_club_to_division`(this, club, division,
/// 1)). The list is freed. Defect of the original: with fewer than three feeder divisions found, a relegated club gets
/// the empty slot (division 0); the original also keeps the first candidate's division in a register it never reads.
/// Original: `EngPrm::exchange_conference_feeder_clubs` (0x5750a0, eng_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn eng_prm_exchange_conference_feeder_clubs(this: usize) {
    let this = League(this);
    let conference = League(comp_by_global(KEY_CLUB_COMP_ENGLISH_CONFERENCE));
    let mut feeders = [0usize; 3];
    let list = exe::crt_malloc((read_i32(G_CLUB_COUNT) as u32).wrapping_mul(4));
    if list == 0 {
        show_error_at(0x9c43b0, 0x2f9);
        return;
    }
    let mut n = 0i16;
    for i in 0..read_i32(G_CLUB_COUNT).max(0) as usize {
        let c = Club(read_usize(G_CLUB_TABLE) + i * club::SIZE);
        if conference_candidate(c) {
            write_usize(list + n as usize * 4, c.addr());
            n = n.wrapping_add(1);
        }
    }
    if n == 0 {
        exe::crt_free(list);
        return;
    }
    covered!("rx13_15 conference feeders");
    exe::comp_util_sort_and_shuffle_top_clubs(list, n as u16 as u32, 3);
    let conference_record = comp_record_of_key(KEY_CLUB_COMP_ENGLISH_CONFERENCE);
    for k in 0..n.max(0) as usize {
        let c = read_usize(list + k * 4);
        let d = read_usize(c + club::DIVISION);
        if d != 0 && feeders.contains(&d) {
            continue; // rx13_15: one club per feeder division
        }
        let slot = feeders.iter().position(|&f| f == 0);
        if let Some(s) = slot {
            feeders[s] = d;
        }
        exe::league_move_club_to_division(this, c, conference_record, 1);
        if slot == Some(2) {
            break;
        }
    }
    let mut taken = 0usize;
    let mut i = 0i32;
    while i < conference.club_count() as i32 && taken < 3 {
        let row = exe::league_get_table_row(conference, i);
        if read_u8(row + league_table_row::STATUS) == 3 {
            covered!("rx13_15 conference relegated to a feeder");
            exe::league_relegate_club_to_division(this, read_usize(exe::league_get_table_row(conference, i)), feeders[taken], 1);
            taken += 1;
        }
        i += 1;
    }
    exe::crt_free(list);
}

/// Comparer for `eng_prm_exchange_conference_feeder_clubs`: only the original runs (it draws in the sort and moves
/// clubs); afterwards the first three Conference rows with status 3 must hold clubs no longer in the Conference.
pub(crate) unsafe extern "thiscall" fn compare_eng_prm_exchange_conference_feeder_clubs(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this);
    let conference = League(comp_by_global(KEY_CLUB_COMP_ENGLISH_CONFERENCE));
    let record = comp_record_of_key(KEY_CLUB_COMP_ENGLISH_CONFERENCE);
    let wrong: Vec<usize> = (0..conference.club_count().max(0) as i32)
        .map(|i| exe::league_get_table_row(conference, i))
        .filter(|&row| read_u8(row + league_table_row::STATUS) == 3)
        .take(3)
        .map(|row| read_usize(row))
        .filter(|&c| read_usize(c + club::DIVISION) == record)
        .collect();
    SCOREBOARD.record(wrong.is_empty(), || format!("eng_prm_exchange_conference_feeder_clubs({:#x}) relegated clubs still in the Conference: {:x?}", this, wrong));
}
