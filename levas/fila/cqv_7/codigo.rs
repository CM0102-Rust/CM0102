// ---------- %%LEVA%%a leva: a Copa da Inglaterra (FA Cup): os 152 inscritos (eng_fa_cup.cpp) ----------

/// True if the club's division id is one of the five English league divisions (the generic lower division
/// `KEY_CLUB_COMP_A_LOWER_DIVISION`, the Premier and the three Football League divisions) or the Conference
/// [inference from the 20 + 24 x 3 + 22].
unsafe fn fa_cup_is_league_division(id: usize) -> bool {
    [KEY_CLUB_COMP_A_LOWER_DIVISION, KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION, KEY_CLUB_COMP_ENGLISH_FIRST_DIVISION, KEY_CLUB_COMP_ENGLISH_SECOND_DIVISION, KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION, KEY_CLUB_COMP_ENGLISH_CONFERENCE].iter().any(|&g| id == read_usize(g))
}

/// The original keeps its two lists side by side on the stack (the Conference one, 22 places, and right after it the
/// non-league one, 300): the copy uses a single array, the second list from index 22, so it overflows as the original
/// would (it never does).
const FA_CUP_NON_LEAGUE_START: i32 = 22;

/// Removes club `c` from the list starting at `start` (`n` used): swaps it with the last one and decrements the count;
/// returns whether it was found.
fn fa_cup_remove_club(v: &mut [usize; 322], start: i32, n: &mut i32, c: usize) -> bool {
    let mut found = false;
    let mut j = 0i32;
    while (j as i16) < *n as i16 {
        if v[(start + j) as usize] == c {
            *n -= 1;
            v[(start + j) as usize] = v[(start + *n as i16 as i32) as usize];
            found = true;
        }
        j += 1;
    }
    found
}

/// Writes the club in slot `k` of the cup's team table (`cup::TEAM_TABLE`, 6-byte entries: the club, then +4 and +5
/// zeroed) and counts it.
unsafe fn fa_cup_add_entrant(cup: Cup, k: &mut i32, c: usize) {
    let t = cup.team_table() + *k as usize * 6;
    write_usize(t, c);
    write_u8(t + 4, 0);
    write_u8(t + 5, 0);
    *k += 1;
}

/// The clubs of division `key`, in club table order, except those in `skip`, until the team table is full
/// (k == `cup::TEAM_COUNT`).
unsafe fn fa_cup_add_division(cup: Cup, k: &mut i32, key: usize, skip: &[usize]) {
    let base = read_usize(G_CLUB_TABLE);
    let mut i: i16 = 0;
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        let c = base + i as u16 as usize * club::SIZE;
        if eng_auto_cup_is_in_comp(c, key) && !skip.contains(&c) {
            fa_cup_add_entrant(cup, k, c);
            if *k == cup.team_count() as i32 {
                return;
            }
        }
        i = i.wrapping_add(1);
    }
}

/// Builds the 152 FA Cup entrants. Called by the cup's daily update (`eng_fa_cup_update_for_date`, 0x570550) every 3
/// October; n = 1 up to 2002-06-01.
///
/// 1) The English clubs (`club::NATION` = `KEY_NATION_ENGLAND`) of the competition `KEY_CLUB_COMP_FIFA_CLUB_WORLD_CHAMPIONSHIP`
/// of the same year (`cup::YEAR`), searched among the [0xb63dfc] competitions of the list [0xb63e00] (the community's
/// `g_league_world_club_comp_count` and `g_league_world_club_comp_list`) by the id of its record (`league::COMPETITION`):
/// its list through the competition's virtual +0x58 (push 0; push &list; mov ecx -> thiscall, ret 8; returns how many;
/// the list is freed with `exe::crt_free`), up to 2 (the Club World Championship: Manchester United's withdrawal in
/// 1999/2000 [inference]). In 1999, with 1 found and `eng_fa_cup::FIFTH_ROUND_QUESTION_ASKED` != 2,
/// `eng_fa_cup::FIFTH_ROUND_ENTRY` = 1 and `eng_fa_cup::EXEMPTION_COUNT` = 0 (the club leaves the cup); otherwise
/// `FIFTH_ROUND_ENTRY` = 0 and `EXEMPTION_COUNT` = how many.
/// 2) `cup::TEAM_COUNT` = 152 and the table (`exe::crt_malloc` of 0x390, not zeroed) in `cup::TEAM_TABLE` (out of memory:
/// the box for line 0x1d2 and -1).
/// 3) With the nation rule (the competition's nation `league_selected` has `LOWER_DIVISIONS_LOADED`) or in 2001 (year
/// 0x7d1): the winner and runner-up of the last season of `KEY_CLUB_COMP_ENGLISH_FA_TROPHY` (`exe::comp_util_get_latest_placed_club`,
/// cdecl, on the record `G_CLUB_COMP_TABLE` + 0x6b x the key's id) get a place; otherwise only the first club of the
/// table, if it is in the Conference (the original tests the same club over and over without advancing its loop).
/// 4) In club table order: the Conference clubs (`KEY_CLUB_COMP_ENGLISH_CONFERENCE`) in one list; the English clubs
/// (`club::NATION` = the record `G_NATION_TABLE` + `nation::SIZE` x the England id) outside the league divisions, up to
/// 300, in another (the non-league clubs).
/// 5) The two prize places leave their own list: from the Conference, with 21 places instead of 22 for each one; from
/// the non-league list, with 37 instead of 38 (when not found, the first of the list is replaced by the last).
/// 6) Two Conference clubs drawn (`exe::utils_random`(n - 2), cdecl, twice) go to the last two positions; the
/// non-league clubs sorted by reputation with the best ones shuffled (`exe::comp_util_sort_and_shuffle_top_clubs`: push
/// places; push n; push list -> cdecl).
/// 7) The table: the non-league places (38), the Conference places (22), the two prize clubs, the Football League's
/// third (`KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION`), second and first divisions (except the clubs of step 1 and, in the
/// third, the prize clubs), each until the table is full; the last 24 (the first division) shuffled by 60 swaps of two
/// drawn positions (`exe::utils_random`(0x18) twice); the Premier (`KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION`); last,
/// the clubs of step 1. With `FIFTH_ROUND_ENTRY` = 1, `cup::TEAM_COUNT` drops by one. Returns 1 if the table is full
/// (k == `cup::TEAM_COUNT`), otherwise 0.
/// Original: `EngFaCup::collect_entrants` (0x56fab0, eng_fa_cup.cpp, thiscall, ret 0).
unsafe fn fa_cup_collect_entrants(this: usize) -> i32 {
    let cup = Cup(this);
    // 1) the English clubs of the Club World Championship of the same year
    let mut same_year = [0usize; 2];
    let mut nf: i16 = 0;
    let n_comp = read_usize(0xb63dfc) as i32;
    if n_comp > 0 {
        let v = read_usize(0xb63e00);
        let mut i: i16 = 0;
        let mut found = 0usize;
        loop {
            let c = read_usize(v + i as i32 as usize * 4);
            if read_usize(read_usize(c + league::COMPETITION)) == read_usize(KEY_CLUB_COMP_FIFA_CLUB_WORLD_CHAMPIONSHIP) {
                found = c;
                break;
            }
            i = i.wrapping_add(1);
            if i as i32 >= n_comp {
                break;
            }
        }
        if found != 0 && read_i16(found + cup::YEAR) == cup.year() {
            // the competition's virtual +0x58 (no name yet): fills a list of its clubs and returns how many
            let club_list: unsafe extern "thiscall" fn(usize, *mut usize, u32) -> i32 = core::mem::transmute(read_usize(read_usize(found) + 0x58));
            let mut list = 0usize;
            let n = club_list(found, &mut list, 0) as i16;
            let mut j: i16 = 0;
            while j < n && nf < 2 {
                let c = read_usize(list + j as i32 as usize * 4);
                let club_nation = read_usize(c + club::NATION);
                if club_nation != 0 && read_usize(club_nation) == read_usize(KEY_NATION_ENGLAND) {
                    same_year[nf as usize] = c;
                    nf += 1;
                }
                j += 1;
            }
            if list != 0 {
                exe::crt_free(list);
            }
        }
    }
    if cup.year() == 0x7cf && nf == 1 && read_u8(this + eng_fa_cup::FIFTH_ROUND_QUESTION_ASKED) != 2 {
        write_u8(this + eng_fa_cup::FIFTH_ROUND_ENTRY, nf as u8);
        write_u8(this + eng_fa_cup::EXEMPTION_COUNT, 0);
    } else {
        write_u8(this + eng_fa_cup::FIFTH_ROUND_ENTRY, 0);
        write_u8(this + eng_fa_cup::EXEMPTION_COUNT, nf as u8);
    }
    // 2) the table
    cup.set_team_count(0x98);
    let t = exe::crt_malloc(0x390);
    cup.set_team_table(t);
    if t == 0 {
        show_error_at(0x9c42cc, 0x1d2);
        return -1;
    }
    // 3) the prize places
    let base = read_usize(G_CLUB_TABLE);
    let mut a = 0usize;
    let mut b = 0usize;
    let cup_nation = read_usize(read_usize(this + league::COMPETITION) + competition::NATION);
    if read_u8(cup_nation + nation::LEAGUE_SELECTED) & league_selected::LOWER_DIVISIONS_LOADED != 0 || cup.year() == 0x7d1 {
        covered!("cqv_7 trophy winner and runner-up");
        let record = read_usize(G_CLUB_COMP_TABLE) + read_usize(KEY_CLUB_COMP_ENGLISH_FA_TROPHY).wrapping_mul(0x6b);
        a = exe::comp_util_get_latest_placed_club(record, 0); // cqv7: the winner
        let record = read_usize(G_CLUB_COMP_TABLE) + read_usize(KEY_CLUB_COMP_ENGLISH_FA_TROPHY).wrapping_mul(0x6b);
        b = exe::comp_util_get_latest_placed_club(record, 1);
    } else if read_usize(G_CLUB_COUNT) as i32 > 0 && eng_auto_cup_is_in_comp(base, KEY_CLUB_COMP_ENGLISH_CONFERENCE) {
        // simplification: the original repeats the test of the first club without advancing its loop; one test is enough
        a = base;
    }
    // 4) the Conference and the non-league clubs
    // simplification: the original does not zero the lists (stack leftovers); it only reads past what it filled when
    // clubs are missing (fewer than 38 non-league), which the career never has; the copy uses 0, which the original
    // also skips
    let mut v = [0usize; 322];
    let mut nconf = 0i32;
    let mut nout = 0i32;
    let england = read_usize(G_NATION_TABLE) + read_usize(KEY_NATION_ENGLAND).wrapping_mul(nation::SIZE);
    let mut i: i16 = 0;
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        let c = base + i as u16 as usize * club::SIZE;
        let l = read_usize(c + club::DIVISION);
        if l != 0 {
            let id = read_usize(l);
            if id == read_usize(KEY_CLUB_COMP_ENGLISH_CONFERENCE) {
                // simplification: the original does not check the size of the Conference list (22 places; the Conference has 22)
                v[nconf as i16 as usize] = c;
                nconf += 1;
            } else if !fa_cup_is_league_division(id) && read_usize(c + club::NATION) == england && (nout as i16) < 0x12c {
                v[(FA_CUP_NON_LEAGUE_START + nout as i16 as i32) as usize] = c;
                nout += 1;
            }
        }
        i = i.wrapping_add(1);
    }
    // 5) the prize clubs leave their own list
    let mut conf_places: i8 = 0x16;
    let mut non_league_places: i8 = 0x26;
    for (k, &p) in [a, b].iter().enumerate() {
        if p == 0 || read_usize(p + club::DIVISION) == 0 {
            continue;
        }
        let id = read_usize(read_usize(p + club::DIVISION));
        if id == read_usize(KEY_CLUB_COMP_ENGLISH_CONFERENCE) {
            fa_cup_remove_club(&mut v, 0, &mut nconf, p);
            if k == 0 {
                conf_places = 0x15;
            } else {
                conf_places -= 1;
            }
        } else if !fa_cup_is_league_division(id) {
            if !fa_cup_remove_club(&mut v, FA_CUP_NON_LEAGUE_START, &mut nout, p) {
                nout -= 1;
                v[FA_CUP_NON_LEAGUE_START as usize + k] = v[(FA_CUP_NON_LEAGUE_START + nout as i16 as i32) as usize];
            }
            if k == 0 {
                non_league_places = 0x25;
            } else {
                non_league_places -= 1;
            }
        }
    }
    // 6) two drawn Conference clubs go to the end; the non-league clubs by reputation
    let n = nconf as i16 as i32;
    let r1 = exe::utils_random(n - 2) as i8 as i32;
    let r2 = exe::utils_random(n - 2) as i8 as i32;
    v.swap(r1 as usize, (n - 1) as usize);
    v.swap(r2 as usize, (n - 2) as usize);
    // simplification: the original pushes the places in an edi with garbage in the high 16 bits; 0x4afbd0 reads the word
    exe::comp_util_sort_and_shuffle_top_clubs(v.as_mut_ptr() as usize + FA_CUP_NON_LEAGUE_START as usize * 4, nout as u32, non_league_places as i16 as u16 as u32); // cqv7: the non-league clubs by reputation
    // 7) the table
    let mut k = 0i32;
    for j in 0..(non_league_places as i32).max(0) {
        let c = v[(FA_CUP_NON_LEAGUE_START + j) as usize];
        if c != 0 {
            fa_cup_add_entrant(cup, &mut k, c);
        }
    }
    for j in 0..(conf_places as i32).max(0) {
        let c = v[j as usize];
        if c != 0 {
            fa_cup_add_entrant(cup, &mut k, c);
        }
    }
    if a != 0 {
        fa_cup_add_entrant(cup, &mut k, a);
    }
    if b != 0 {
        fa_cup_add_entrant(cup, &mut k, b);
    }
    let (x1, x2) = (same_year[0], same_year[1]);
    fa_cup_add_division(cup, &mut k, KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION, &[a, b, x1, x2]); fa_cup_add_division(cup, &mut k, KEY_CLUB_COMP_ENGLISH_SECOND_DIVISION, &[x1, x2]); // cqv7: the third and the second
    fa_cup_add_division(cup, &mut k, KEY_CLUB_COMP_ENGLISH_FIRST_DIVISION, &[x1, x2]);
    covered!("cqv_7 first division shuffled");
    for _ in 0..0x3c { // cqv7: 60 swaps in the first division
        let r1 = exe::utils_random(0x18) as i8 as i32;
        let r2 = exe::utils_random(0x18) as i8 as i32;
        let t = cup.team_table();
        let p1 = t + (k - r1 - 1) as usize * 6;
        let p2 = t + (k - r2 - 1) as usize * 6;
        let v1 = read_usize(p1);
        write_usize(p1, read_usize(p2));
        write_usize(p2, v1);
    }
    fa_cup_add_division(cup, &mut k, KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION, &[x1, x2]);
    let nb = read_i8(this + eng_fa_cup::EXEMPTION_COUNT);
    let mut j: i16 = 0;
    while j < nb as i16 {
        fa_cup_add_entrant(cup, &mut k, same_year[j as usize]);
        j += 1;
    }
    if read_u8(this + eng_fa_cup::FIFTH_ROUND_ENTRY) == 1 {
        cup.set_team_count(cup.team_count().wrapping_sub(1));
    }
    (k == cup.team_count() as i32) as i32
}

/// The FA Cup's 152 entrants: see `fa_cup_collect_entrants`.
/// Original: `EngFaCup::collect_entrants` (0x56fab0, eng_fa_cup.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn eng_fa_cup_collect_entrants(this: usize) -> i32 {
    fa_cup_collect_entrants(this)
}

/// Side by side from the same state (the random generator, `cup::TEAM_COUNT`, `eng_fa_cup::EXEMPTION_COUNT` and
/// `FIFTH_ROUND_ENTRY`, `G_MOUSE_LEFT_BUTTON_DOWN`): the original leaves its table in `cup::TEAM_TABLE`, the copy builds its own
/// (without counting the draws), compared on the filled slots (the club of each one; the table is not zeroed) and
/// freed; the return value too.
unsafe extern "thiscall" fn compare_eng_fa_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I0%%);
    let before = read_usize(this + cup::TEAM_TABLE);
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.extend([(this + cup::TEAM_COUNT, 2usize), (this + eng_fa_cup::EXEMPTION_COUNT, 2), (G_MOUSE_LEFT_BUTTON_DOWN, 4)]);
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let (a, b, same, _, _) = crate::comparison::side_by_side(&reg, || {
        let r = o(this);
        lo.set(read_usize(this + cup::TEAM_TABLE));
        write_usize(this + cup::TEAM_TABLE, before);
        r
    }, || {
        let r = crate::rng::without_counting(|| fa_cup_collect_entrants(this));
        lr.set(read_usize(this + cup::TEAM_TABLE));
        write_usize(this + cup::TEAM_TABLE, lo.get());
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let n = (read_i16(this + cup::TEAM_COUNT).max(0) as usize).min(0x98);
    let table = lo == 0 || lr == 0 || a != 1 || (0..n).all(|j| read_usize(lo + j * 6) == read_usize(lr + j * 6) && read_u16(lo + j * 6 + 4) == read_u16(lr + j * 6 + 4));
    if lr != 0 && lr != lo {
        exe::crt_free(lr);
    }
    SCOREBOARD.record(same && a == b && table, || format!("eng_fa_cup_collect_entrants({:#x}) returned {} x {} memory {} table {}", this, a, b, same, table));
    a
}
