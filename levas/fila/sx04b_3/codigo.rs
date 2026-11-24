// ---------- sx04b_3: a nation's matches in its month ratings and the player of the year from the year ratings (nation_awards.cpp) ----------

/// `MONTH_RATINGS::record_match_ratings(fixture, record)` (0x767ee0, month_ratings.cpp, thiscall, ret 8).
const FN_MONTH_RATINGS_RECORD_MATCH_RATINGS: usize = 0x767ee0;
/// `YEAR_RATINGS::collect_best_players(out, max, nation, club nation, excluded nation, home nation, division, mode,
/// max age)` (0x931ae0, year_ratings.cpp, thiscall, ret 0x24): how many were collected (ax).
const FN_YEAR_RATINGS_COLLECT_BEST_PLAYERS: usize = 0x931ae0;
/// `AWARD_SHORTLIST::clear()` (0x418320, thiscall), `add_entry(person, club, f32 score)` (0x4178a0, thiscall, ret
/// 0xc) and `announce(date, nation id, award index)` (0x417a20, thiscall, ret 0xc).
pub(crate) const FN_AWARD_SHORTLIST_CLEAR: usize = 0x418320;
const FN_AWARD_SHORTLIST_ADD_ENTRY: usize = 0x4178a0;
const FN_AWARD_SHORTLIST_ANNOUNCE: usize = 0x417a20;

/// Passes a played match to the nation's month ratings (`MONTH_RATINGS::record_match_ratings` with the same fixture
/// and record), when the nation has month ratings (`nation_awards_fields::MONTH_RATINGS`). The leagues' virtual
/// `record_match_ratings` (England's and Finland's at 0x5a96b0) call it for each league match.
/// Original: `NationAwards::record_match_ratings` (0x7683a0, nation_awards.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn nation_awards_record_match_ratings(this: usize, fixture: usize, record: usize) {
    let ratings = read_usize(this + nation_awards_fields::MONTH_RATINGS);
    if ratings == 0 {
        return;
    }
    covered!("sx04b_3 match to the month ratings");
    let f: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_MONTH_RATINGS_RECORD_MATCH_RATINGS);
    f(ratings, fixture, record); // sx04b_3: the month ratings
}

/// Comparer for `nation_awards_record_match_ratings`: the ratings take each match once, so only the original runs (the
/// month ratings routine it calls has its own comparer).
pub(crate) unsafe extern "thiscall" fn compare_nation_awards_record_match_ratings(this: usize, fixture: usize, record: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I0%%)(this, fixture, record)
}

/// Writes the three winners `people` into a year award record for the year of `today`: each one's name ids, staff id
/// and club id (−1 without a club); the scores are left as they are (the original never writes them).
unsafe fn year_record_of(today: usize, people: [Staff; 3]) -> [u8; YEAR_RECORD_SIZE] {
    let mut record = [0u8; YEAR_RECORD_SIZE];
    record[..2].copy_from_slice(&read_u16(today + cm_date::YEAR).to_le_bytes());
    for (k, person) in people.into_iter().enumerate() {
        let club = person.club_job();
        let club_id = if club.is_null() { u32::MAX } else { read_u32(club.addr()) };
        year_record_entry(&mut record, k, person, None, club_id);
    }
    record
}

/// A year ratings entry's score for a shortlist, by the collecting `mode`: the mode's weighted rating sum (0, 1 or 2)
/// or, for any other mode, 2 × goals + assists, over the matches, as the x87 rounds it to an f32.
unsafe fn ratings_shortlist_score(entry: YearEntry, mode: u8) -> f32 {
    let total = match mode {
        0..=2 => entry.rating_sum(mode as usize),
        _ => entry.assists() + entry.goals() * 2,
    };
    (total as f64 / entry.matches() as i32 as f64) as f32
}

/// The player of the year from a year ratings table (see `nation_awards_award_player_of_year_from_ratings`), the
/// winners record going to `sink` and the shortlist announced only when `announce`.
pub(crate) unsafe fn nation_awards_award_from_ratings_with(this: usize, today: usize, award: usize, ratings: usize, filters: [u32; 7], announce: bool, sink: YearAwardSink) {
    let shortlist = read_usize(award + AWARD_SHORTLIST);
    // how many: the shortlist's capacity (the word over a dword whose high half is 0), or 3
    let wanted: u32 = if shortlist != 0 { read_u16(shortlist + award_shortlist_fields::CAPACITY) as u32 } else { 3 };
    let n = wanted as u16 as i16;
    let buf = exe::crt_malloc((n as i32 * 4) as u32);
    if buf == 0 {
        tel26_error_box(NATION_AWARDS_CPP, 0xab);
        return;
    }
    if ratings == 0 {
        // defect of the original: the block is not freed
        tel26_error_box(NATION_AWARDS_CPP, 0xb2);
        return;
    }
    covered!("sx04b_3 player of the year from ratings");
    let collect: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32, u32, u32, u32) -> u16 = core::mem::transmute(FN_YEAR_RATINGS_COLLECT_BEST_PLAYERS);
    let [nation, club_nation, excluded, home, division, mode, max_age] = filters;
    let found = collect(ratings, buf, wanted, nation, club_nation, excluded, home, division, mode, max_age) as i16;
    if found < n { // sx04b_3: enough candidates
        exe::crt_free(buf);
        return;
    }
    let entry = |k: usize| YearEntry(read_usize(buf + 4 * k));
    if shortlist != 0 {
        covered!("sx04b_3 shortlist filled");
        let clear: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_AWARD_SHORTLIST_CLEAR);
        clear(shortlist);
        let add: unsafe extern "thiscall" fn(usize, usize, usize, f32) -> u32 = core::mem::transmute(FN_AWARD_SHORTLIST_ADD_ENTRY);
        for k in 0..n.max(0) as usize {
            let person = entry(k).person();
            add(shortlist, person.addr(), person.club_job().addr(), ratings_shortlist_score(entry(k), mode as u8));
        }
        if announce {
            let f: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(FN_AWARD_SHORTLIST_ANNOUNCE);
            f(shortlist, today, read_u32(this + nation_awards_fields::NATION_ID), read_u32(award + award_base::INDEX));
        }
    } else {
        covered!("sx04b_3 winners recorded");
        // without a shortlist n is 3: one entry per winner (the original would put any further one in the third)
        let people = [0, 1, 2].map(|k| entry(k).person());
        sink(award, &year_record_of(today, people));
    }
    exe::crt_free(buf);
}

/// Gives a year award from a year ratings table: the best players of `ratings` (`YEAR_RATINGS::collect_best_players`
/// with the seven filters: nation, club nation, excluded nation, home nation, division, mode, maximum age), as many as
/// the award's shortlist (`AWARD_SHORTLIST`) holds (its capacity) or 3 without one, into a block (`exe::crt_malloc`). Fewer found:
/// nothing. With a shortlist, it is cleared and each candidate is added (`AWARD_SHORTLIST::add_entry`: the person,
/// his club, the score of `ratings_shortlist_score`), then announced with `today`, the nation's id
/// (`nation_awards_fields::NATION_ID`) and the award's index (`award_base::INDEX`). Without one, the three winners (names, staff id, club id or −1) go with the year of `today` to
/// `YEAR_AWARD::record_winners`. The block is freed. Out of memory: the nation_awards.cpp 171 box; no ratings: the 178
/// box. Defects of the original: with no ratings the block is not freed; the record's score dwords are never written
/// (the history keeps stack garbage).
/// Original: `NationAwards::award_player_of_year_from_ratings` (0x7683c0, nation_awards.cpp, thiscall, ret 0x28).
pub(crate) unsafe extern "thiscall" fn nation_awards_award_player_of_year_from_ratings(this: usize, today: usize, award: usize, ratings: usize,
    nation: u32, club_nation: u32, excluded: u32, home: u32, division: u32, mode: u32, max_age: u32) {
    nation_awards_award_from_ratings_with(this, today, award, ratings, [nation, club_nation, excluded, home, division, mode, max_age], true,
        &mut |award, record| record_year_award(award, record));
}

/// A shortlist's count, votes and entries (sorted, since the announcement sorts them by name).
unsafe fn shortlist_contents(shortlist: usize) -> (Vec<u8>, Vec<Vec<u8>>) {
    use award_shortlist_fields as asl;
    let head = (asl::COUNT..0x28).map(|i| read_u8(shortlist + i)).collect();
    let mut entries: Vec<Vec<u8>> = (0..read_i16(shortlist + asl::COUNT).max(0) as usize)
        .map(|k| (0..asl::ENTRY_SIZE).map(|i| read_u8(read_usize(shortlist + asl::ENTRIES) + k * asl::ENTRY_SIZE + i)).collect())
        .collect();
    entries.sort();
    (head, entries)
}

/// Comparer for `nation_awards_award_player_of_year_from_ratings`. Errors (no ratings): only the original. With a
/// shortlist: the copy fills it as a dry run (no announcement), its contents are kept and the shortlist put back; the
/// original runs and must leave the same count, votes and entries (in any order: the announcement sorts them by
/// name). Without one: the copy runs as a dry run (no record), then the original; its newest year award history
/// record must be the planned one, without the score dwords the original never writes.
pub(crate) unsafe extern "thiscall" fn compare_nation_awards_award_player_of_year_from_ratings(this: usize, today: usize, award: usize, ratings: usize,
    nation: u32, club_nation: u32, excluded: u32, home: u32, division: u32, mode: u32, max_age: u32) {
    use award_shortlist_fields as asl;
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32, u32, u32, u32, u32)>(%%I1%%);
    let original = || o(this, today, award, ratings, nation, club_nation, excluded, home, division, mode, max_age);
    let filters = [nation, club_nation, excluded, home, division, mode, max_age];
    if ratings == 0 {
        return original();
    }
    let shortlist = read_usize(award + AWARD_SHORTLIST);
    if shortlist != 0 {
        let regions = [(shortlist, 0x28), (read_usize(shortlist + asl::ENTRIES), read_i16(shortlist + asl::CAPACITY).max(0) as usize * asl::ENTRY_SIZE)];
        let before = crate::comparison::MemorySnapshot::read(&regions);
        crate::rng::pause_trace(|| nation_awards_award_from_ratings_with(this, today, award, ratings, filters, false, &mut |_, _| {}));
        let planned = shortlist_contents(shortlist);
        before.write_back();
        original();
        let got = shortlist_contents(shortlist);
        SCOREBOARD.record(got == planned, || format!("nation_awards_award_player_of_year_from_ratings(award {:#x}) shortlist original {:02x?} rust {:02x?}", award, got, planned));
        return;
    }
    let mut planned = None;
    crate::rng::pause_trace(|| nation_awards_award_from_ratings_with(this, today, award, ratings, filters, false, &mut |_, r| planned = Some(r.to_vec())));
    let list = read_usize(read_usize(G_AWARD_MANAGER) + award_manager_consts::YEAR_AWARD_FILE);
    let garbage: Vec<(usize, usize)> = (0..3).map(|k| (2 + k * YEAR_RECORD_ENTRY + YEAR_RECORD_SCORE, 4)).collect();
    compare_award_record("nation_awards_award_player_of_year_from_ratings", list, award + YEAR_AWARD_HISTORY_FIRST, award + YEAR_AWARD_HISTORY_LAST, planned, &garbage, original);
}
