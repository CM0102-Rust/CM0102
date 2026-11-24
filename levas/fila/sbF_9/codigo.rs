// ---------- sbF_9: the FA Trophy round table and entrants (eng_fa_trophy.cpp) ----------

/// The FA Trophy rounds: the 3rd round drawn on 8 November and played on 25 November (32 clubs, 16 ties, all new,
/// replays), the 4th drawn 26 November and played 16 January, the 5th drawn 17 January and played 6 February, the
/// semi-finals drawn 7 February and played over two legs 7 days apart from 10 April, the final drawn 11 April and
/// played on 15 May at Wembley [inference: venue 0x4000]; the two-leg tie rules 2 up to the 5th round (4 and 3 after);
/// category 4 up to the 5th round and 6 after; no prize per round (−1). Rounds 0 to 3 also have the word +0x11 = 3
/// (`FA_TROPHY_ROUND_WORD_11`).
pub(crate) const FA_TROPHY_ROUNDS: [EngCcCupCupRound; 5] = [
    cql10r((8, 10, 0, 6), (0x19, 10, 0, 5, 1, 0), 0x14, (0, 0, 2), 4, (0x20, 0x10, 0x20, 0), (2, 1, 0), (0, 0xffff_ffff, 0)), // sbF9: the FA Trophy 3rd round
    cql10r((0x1a, 10, 0, 6), (0x10, 0, 1, 5, 1, 0), 0x1e, (0, 0, 2), 4, (0x10, 8, 0, 0), (2, 0, 0), (0, 0xffff_ffff, 0)),
    cql10r((0x11, 0, 1, 6), (6, 1, 1, 5, 1, 0), 0x28, (0, 0, 2), 4, (8, 4, 0, 0), (2, 0, 0), (0, 0xffff_ffff, 0)),
    cql10r((7, 1, 1, 6), (0xa, 3, 1, 5, 1, 0), 0x82, (0, 0, 4), 6, (4, 2, 0, 0), (1, 2, 7), (0, 0xffff_ffff, 0)),
    cql10r((0xb, 3, 1, 6), (0xf, 4, 1, 5, 1, 0x4000), 0x96, (0, 0, 3), 6, (2, 1, 0, 0), (1, 1, 0), (0, 0xffff_ffff, 0)),
];

/// The word the FA Trophy writes at +0x11 of rounds 0 to 3 (after `cup_round::TWO_LEG_TIE_RULES`; not named yet).
pub(crate) const FA_TROPHY_ROUND_WORD_11: u16 = 3;

/// The body of `eng_fa_trophy_create_round_table`: the single-stage table with `FA_TROPHY_ROUNDS` (the box at line 0xa4
/// of eng_fa_trophy.cpp) and +0x11 = 3 in rounds 0 to 3 (the original writes it among the round's fields).
pub(crate) unsafe fn build_fa_trophy_round_table(this: Cup, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let t = eng_cc_cup_create_single_stage_round_table(this, stage, rounds, extra, out, &FA_TROPHY_ROUNDS, 0x9c4338, 0xa4, "sbF_9 fa trophy rounds");
    if t != 0 {
        for r in 0..4usize {
            write_u16(t + r * cup_round::SIZE + 0x11, FA_TROPHY_ROUND_WORD_11); // +0x11: not named yet
        }
    }
    t
}

/// Rule: the FA Trophy (non-league clubs) has five rounds: 32 clubs in the 3rd round in late November, the 4th in
/// January, the 5th in February with replays, two-legged semi-finals in April and the final on 15 May at a neutral
/// ground [inference: Wembley]; category 4 up to the 5th round and 6 after; no prize per round.
///
/// Only stage −1: `out` = 0 when given, 5 rounds, extra 0, the table (`exe::crt_malloc` of 0x208; out of memory, the box
/// at line 0xa4 and 0) with `FA_TROPHY_ROUNDS` and the year sign-extended (checked by emulation of the original for
/// every stage −128..127 in an odd and an even year: the outputs, the size and every byte written). Other stages: 0.
/// Original: `EngFaTrophy::create_round_table` (0x570ec0, eng_fa_trophy.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn eng_fa_trophy_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    build_fa_trophy_round_table(Cup(this), stage, rounds, extra, out)
}

/// Side by side like `compare_cup_round_table`, also comparing the word +0x11 of each round.
pub(crate) unsafe extern "thiscall" fn compare_eng_fa_trophy_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    if stage as u8 != 0xff {
        let a = o(this, stage, rounds, extra, out);
        SCOREBOARD.record(a == 0, || format!("eng_fa_trophy_create_round_table(stage {}) returned {:#x}", stage as u8 as i8, a));
        return a;
    }
    let (mut r2, mut e2) = (*rounds, *extra);
    let mut s2 = if out.is_null() { 0 } else { *out };
    let reg = [(0xb67a34usize, 4usize)];
    let (a, b, equal, _, _) = crate::comparison::side_by_side(&reg, || o(this, stage, rounds, extra, out),
        || build_fa_trophy_round_table(Cup(this), stage, &mut r2, &mut e2, if out.is_null() { core::ptr::null_mut() } else { &mut s2 }));
    let outs = *rounds == r2 && *extra == e2 && (out.is_null() || *out == s2);
    let words = |t: usize| (0..4usize).map(|r| read_u16(t + r * cup_round::SIZE + 0x11)).collect::<Vec<u16>>();
    let table = if a != 0 && b != 0 { eng_cc_cup_written_round_bytes(a, *rounds) == eng_cc_cup_written_round_bytes(b, r2) && words(a) == words(b) } else { a == 0 && b == 0 };
    if b != 0 {
        exe::crt_free(b);
    }
    SCOREBOARD.record(equal && outs && table, || format!("eng_fa_trophy_create_round_table(stage {}): outs {} ({} x {}) table {}", stage as u8 as i8, outs, *rounds, r2, table));
    a
}

/// True when a club plays in a non-league division of the FA Trophy's nation: a division that is not one of the four
/// Football League divisions, the Conference or the amateur division.
pub(crate) unsafe fn fa_trophy_lower_club(this: Cup, c: usize) -> bool {
    let d = read_usize(c + club::DIVISION);
    let record = read_usize(this.addr() + 4); // the competition record (Comp +4)
    if d == 0 || read_usize(c + club::NATION) != read_usize(record + competition::NATION) {
        return false;
    }
    let id = read_usize(d);
    ![KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION, KEY_CLUB_COMP_ENGLISH_FIRST_DIVISION, KEY_CLUB_COMP_ENGLISH_SECOND_DIVISION, KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION,
        KEY_CLUB_COMP_ENGLISH_CONFERENCE, KEY_CLUB_COMP_A_LOWER_DIVISION].iter().any(|&k| id == read_usize(k))
}

/// Rule: the FA Trophy takes 32 clubs: the ten best clubs of the lower non-league divisions of its nation (the best
/// shuffled among them) and the Conference clubs; it fails (returns 0) when they do not fill the 32 places.
///
/// `cup::TEAM_COUNT` = 32 and the entrant table (`exe::crt_malloc` of 0xc0 into `cup::TEAM_TABLE`; out of memory, the
/// box at line 0x126 and −1). In club order (16-bit index): the clubs with a division, of the competition's nation
/// (`club::NATION` against `competition::NATION` of the cup's record), not in the Premier, First, Second or Third
/// Division, the Conference or the amateur division, into a local pool while it holds fewer than 100 (a signed byte);
/// the pool sorted with its best ten shuffled (`exe::comp_util_sort_and_shuffle_top_clubs`(pool, count, 10)) and its
/// first ten into entries 0 to 9 (status 0, class 0); then the Conference clubs (`KEY_CLUB_COMP_ENGLISH_CONFERENCE`)
/// from entry 10 until the 32 places are full. Returns 1 when they are, else 0.
/// Original: `EngFaTrophy::select_teams` (0x5712b0, eng_fa_trophy.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn eng_fa_trophy_select_teams(this: usize) -> i32 {
    let this = Cup(this);
    this.set_team_count(0x20);
    let t = exe::crt_malloc(0xc0);
    this.set_team_table(t);
    if t == 0 {
        show_error_at(0x9c4338, 0x126);
        return -1;
    }
    covered!("sbF_9 fa trophy entrants");
    // simplification: the original's pool is the uninitialised stack; with fewer than ten lower clubs it would take
    // leftovers where the copy takes nulls (only with the bigger databases, which have more)
    let mut pool = [0usize; 100];
    let mut count = 0i8;
    let mut k = 0i16;
    while (k as i32) < read_i32(G_CLUB_COUNT) {
        let c = club_at(k as isize).addr();
        if fa_trophy_lower_club(this, c) && count < 0x64 {
            pool[count as usize] = c;
            count += 1;
        }
        k = k.wrapping_add(1);
    }
    exe::comp_util_sort_and_shuffle_top_clubs(pool.as_mut_ptr() as usize, count as i16 as u32, 0xa); // sbF9: the best ten non-league clubs
    let mut n = 0i32;
    for &c in &pool[..0xa] {
        add_entrant(this, &mut n, c);
    }
    enter_clubs_pass(this, &mut n, || read_i32(G_CLUB_COUNT), true, |c, _| comp_id_at(c, club::DIVISION) == Some(read_usize(KEY_CLUB_COMP_ENGLISH_CONFERENCE)));
    (n == this.team_count() as i32) as i32
}

/// Comparer for `eng_fa_trophy_select_teams`: only the original (it allocates the table and shuffles); returns the same
/// as the plan (32 places filled), every Conference club of the plan in entries 10 on and ten other clubs before them.
pub(crate) unsafe extern "thiscall" fn compare_eng_fa_trophy_select_teams(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I1%%);
    let conference: Vec<usize> = (0..read_i32(G_CLUB_COUNT).max(0) as isize).map(|k| club_at(k).addr()).filter(|&c| comp_id_at(c, club::DIVISION) == Some(read_usize(KEY_CLUB_COMP_ENGLISH_CONFERENCE))).take(0x16).collect();
    let r = o(this);
    let t = Cup(this);
    let got: Vec<usize> = if t.team_table() != 0 { (0..0x20).map(|k| read_usize(t.team_table() + k * 6)).collect() } else { Vec::new() };
    let ok = r == (conference.len() == 0x16) as i32 && got.len() == 0x20 && got[0xa..0xa + conference.len()] == conference[..];
    SCOREBOARD.record(ok, || format!("eng_fa_trophy_select_teams({:#x}) returned {}; conference {}", this, r, conference.len()));
    r
}
