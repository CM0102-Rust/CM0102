// ---------- sx04b_4: the year awards given from the year ratings' candidates and from a shortlist (nation_awards.cpp) ----------

/// `YEAR_RATINGS::collect_candidates(out, max, continent, positions, mode, excluded list)` (0x931d50, year_ratings.cpp,
/// thiscall, ret 0x18): how many were collected (ax).
const FN_YEAR_RATINGS_COLLECT_CANDIDATES: usize = 0x931d50;
/// `AWARD_SHORTLIST::pick_winners(out, count)` (0x417c50, award_shortlist.cpp, thiscall, ret 8): how many were
/// written (ax).
const FN_AWARD_SHORTLIST_PICK_WINNERS: usize = 0x417c50;

/// The top three of the year ratings' candidates (see `nation_awards_award_top_three_from_ratings`), the record going
/// to `sink`; the winners are added to `list` only when `add_to_list`.
pub(crate) unsafe fn nation_awards_award_top_three_with(today: usize, award: usize, ratings: usize, continent: u32, positions: u32, mode: u32, list: usize,
    add_to_list: bool, sink: YearAwardSink) {
    let buf = exe::crt_malloc(0xc);
    if buf == 0 {
        tel26_error_box(NATION_AWARDS_CPP, 0x153);
        return;
    }
    if ratings == 0 {
        // defect of the original: the block is not freed
        tel26_error_box(NATION_AWARDS_CPP, 0x15a);
        return;
    }
    let collect: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, usize) -> u16 = core::mem::transmute(FN_YEAR_RATINGS_COLLECT_CANDIDATES);
    if (collect(ratings, buf, 3, continent, positions, mode, list) as i16) < 3 { // sx04b_4: three candidates
        exe::crt_free(buf);
        return;
    }
    covered!("sx04b_4 top three from the ratings");
    let mut people = [Staff(0); 3];
    for (k, person) in people.iter_mut().enumerate() {
        *person = staff_at(read_i32(read_usize(buf + 4 * k)) as isize);
        if list != 0 && add_to_list {
            let mut local = person.addr();
            exe::llist_add_tail(list, &mut local as *mut usize as usize, 4);
        }
    }
    sink(award, &year_record_of(today, people));
    exe::crt_free(buf);
}

/// Gives a year award to the three best candidates of a year ratings table: `YEAR_RATINGS::collect_candidates` (3, of
/// the `continent`, the `positions`, by the sort `mode`, leaving out those already in `list`) into a 12-byte block.
/// Fewer than three: nothing. Each winner is added to `list` when there is one (`LLIST::add_tail`, the staff record
/// pointer); the three (names, staff id, club id or −1) go with the year of `today` to `YEAR_AWARD::record_winners`.
/// The block is freed. Out of memory: the nation_awards.cpp 339 box; no ratings: the 346 box. Defects of the
/// original: with no ratings the block is not freed; the record's score dwords are never written.
/// Original: `nation_awards_award_top_three_from_ratings` (0x768720, nation_awards.cpp, thiscall, ret 0x1c).
pub(crate) unsafe extern "thiscall" fn nation_awards_award_top_three_from_ratings(_this: usize, today: usize, award: usize, ratings: usize,
    continent: u32, positions: u32, mode: u32, list: usize) {
    nation_awards_award_top_three_with(today, award, ratings, continent, positions, mode, list, true, &mut |award, record| record_year_award(award, record));
}

/// The three winners of a year award's shortlist (see `nation_awards_award_from_shortlist`), the record going to
/// `sink`.
pub(crate) unsafe fn nation_awards_award_from_shortlist_with(today: usize, award: usize, sink: YearAwardSink) {
    let shortlist = read_usize(award + AWARD_SHORTLIST);
    if shortlist == 0 {
        tel26_error_box(NATION_AWARDS_CPP, 0x24c);
        return;
    }
    let buf = exe::crt_malloc(0xc);
    if buf == 0 {
        tel26_error_box(NATION_AWARDS_CPP, 0x254);
        return;
    }
    let pick: unsafe extern "thiscall" fn(usize, usize, u32) -> u16 = core::mem::transmute(FN_AWARD_SHORTLIST_PICK_WINNERS);
    let picked = pick(shortlist, buf, 3) as i16;
    let clear: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_AWARD_SHORTLIST_CLEAR);
    clear(shortlist);
    if picked < 3 { // sx04b_4: three winners
        exe::crt_free(buf);
        return;
    }
    covered!("sx04b_4 winners from the shortlist");
    let people = [0, 1, 2].map(|k| Staff(read_usize(buf + 4 * k)));
    sink(award, &year_record_of(today, people));
    exe::crt_free(buf);
}

/// Decides a year award from its shortlist (`AWARD_SHORTLIST` of the award): `AWARD_SHORTLIST::pick_winners` (the
/// computer clubs' votes) writes up to three staff records into a 12-byte block, and the shortlist is cleared. Fewer
/// than three: nothing. The three (names, staff id, club id or −1) go with the year of `today` to
/// `YEAR_AWARD::record_winners`; the block is freed. No shortlist: the nation_awards.cpp 588 box; out of memory: the 596
/// box. Defect of the original: the record's score dwords are never written.
/// Original: `nation_awards_award_from_shortlist` (0x768b20, nation_awards.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn nation_awards_award_from_shortlist(_this: usize, today: usize, award: usize) {
    nation_awards_award_from_shortlist_with(today, award, &mut |award, record| record_year_award(award, record));
}

/// The year award record's byte ranges the original never writes: each winner's score.
fn year_record_unwritten() -> Vec<(usize, usize)> {
    (0..3).map(|k| (2 + k * YEAR_RECORD_ENTRY + YEAR_RECORD_SCORE, 4)).collect()
}

/// Comparer for `nation_awards_award_top_three_from_ratings`: the copy runs as a dry run (no record, nothing added to
/// the list), then the original; its newest year award history record must be the planned one, without the scores.
/// No ratings (the error box): only the original.
pub(crate) unsafe extern "thiscall" fn compare_nation_awards_award_top_three_from_ratings(this: usize, today: usize, award: usize, ratings: usize,
    continent: u32, positions: u32, mode: u32, list: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32, usize)>(%%I0%%);
    if ratings == 0 {
        return o(this, today, award, ratings, continent, positions, mode, list);
    }
    let mut planned = None;
    crate::rng::pause_trace(|| nation_awards_award_top_three_with(today, award, ratings, continent, positions, mode, list, false, &mut |_, r| planned = Some(r.to_vec())));
    let history = read_usize(read_usize(G_AWARD_MANAGER) + award_manager_consts::YEAR_AWARD_FILE);
    compare_award_record("nation_awards_award_top_three_from_ratings", history, award + YEAR_AWARD_HISTORY_FIRST, award + YEAR_AWARD_HISTORY_LAST, planned,
        &year_record_unwritten(), || o(this, today, award, ratings, continent, positions, mode, list));
}

/// Comparer for `nation_awards_award_from_shortlist`: the votes draw and the shortlist is cleared, so the shortlist (its
/// 0x40 bytes and its entries) and the random generator are kept; the copy runs as a dry run (no record), they are put
/// back and the original runs; its newest year award history record must be the planned one, without the scores. No
/// shortlist (the error box): only the original.
pub(crate) unsafe extern "thiscall" fn compare_nation_awards_award_from_shortlist(this: usize, today: usize, award: usize) {
    use award_shortlist_fields as asl;
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I1%%);
    let shortlist = read_usize(award + AWARD_SHORTLIST);
    if shortlist == 0 {
        return o(this, today, award);
    }
    let mut regions = vec![(shortlist, 0x40), (read_usize(shortlist + asl::ENTRIES), read_i16(shortlist + asl::CAPACITY).max(0) as usize * asl::ENTRY_SIZE)];
    regions.extend_from_slice(&crate::rng::STATE_REGIONS);
    let before = crate::comparison::MemorySnapshot::read(&regions);
    let mut planned = None;
    crate::rng::pause_trace(|| crate::rng::without_counting(|| nation_awards_award_from_shortlist_with(today, award, &mut |_, r| planned = Some(r.to_vec()))));
    before.write_back();
    let history = read_usize(read_usize(G_AWARD_MANAGER) + award_manager_consts::YEAR_AWARD_FILE);
    compare_award_record("nation_awards_award_from_shortlist", history, award + YEAR_AWARD_HISTORY_FIRST, award + YEAR_AWARD_HISTORY_LAST, planned,
        &year_record_unwritten(), || o(this, today, award));
}
