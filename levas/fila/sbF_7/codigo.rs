// ---------- sbF_7: the Dutch Cup entrants and its first full day (hol_cup.cpp) ----------

/// The Dutch nation record (`G_NATION_TABLE` + [`KEY_NATION_HOLLAND`] × `nation::SIZE`), compared by address with a
/// club's `club::NATION`.
pub(crate) unsafe fn holland() -> usize {
    read_usize(G_NATION_TABLE) + read_usize(KEY_NATION_HOLLAND).wrapping_mul(nation::SIZE)
}

/// True when club `c` plays in the division whose id is in `key` (`club::DIVISION` set and its first dword equal).
pub(crate) unsafe fn plays_in(c: usize, key: usize) -> bool {
    comp_id_at(c, club::DIVISION) == Some(read_usize(key))
}

/// A Dutch amateur club: in the amateur division (`KEY_CLUB_COMP_A_LOWER_DIVISION`) and of the Dutch nation.
pub(crate) unsafe fn is_dutch_amateur(c: usize) -> bool {
    plays_in(c, KEY_CLUB_COMP_A_LOWER_DIVISION) && read_usize(c + club::NATION) == holland()
}

/// One pass over the club table (16-bit index against `G_CLUB_COUNT`): every club `keep` accepts is appended to the
/// entrant list while it holds fewer than `ita_cup_fields::ENTRANT_CLUB_COUNT`.
pub(crate) unsafe fn append_dutch_cup_entrants(this: usize, n: &mut i32, keep: impl Fn(usize) -> bool) {
    let mut k = 0i16;
    while (k as i32) < read_i32(G_CLUB_COUNT) {
        let c = club_at(k as isize).addr();
        if keep(c) && *n < read_i32(this + ita_cup_fields::ENTRANT_CLUB_COUNT) {
            write_usize(read_usize(this + ita_cup_fields::ENTRANT_CLUBS) + 4 * *n as usize, c);
            *n += 1;
        }
        k = k.wrapping_add(1);
    }
}

/// Rule: the Dutch Cup takes 86 clubs: the Eerste Divisie clubs and the Eredivisie clubs with no European cup, 50 Dutch
/// amateur clubs (the best ones, shuffled) placed in the gaps left among the professionals sorted by the longitude of
/// their city, the whole list shuffled in pots of eight, then the Eredivisie clubs in the Intertoto, the UEFA Cup and
/// the Champions League (in that order, at the end: they enter later), and any place still empty taken by another
/// Dutch amateur club.
///
/// `ita_cup_fields::ENTRANT_CLUB_COUNT` = 86; the list (`exe::crt_calloc`(4, 86) into `ita_cup_fields::ENTRANT_CLUBS`;
/// out of memory, the box at line 0x32c and 0). In club order (16-bit index): the Dutch amateurs into a local pool of
/// up to 300; the Dutch clubs of the Toto division (`KEY_CLUB_COMP_DUTCH_TOTO_DIVISION`); the KPN division clubs
/// (`KEY_CLUB_COMP_DUTCH_KPN_TELECOMPETITION`) with no continental competition (`club::CONTINENTAL_COMP_ID` = −1),
/// each appended while the list has room. Then the first 50 entries are appended again (no bound check), the list
/// sorted by longitude (`exe::crt_qsort`(list, n, 4, `database_compare_clubs_by_longitude` 0x53fe50)), every later
/// copy of an entry zeroed (16-bit indices); the amateur pool sorted with its best 50 shuffled
/// (`exe::comp_util_sort_and_shuffle_top_clubs`(pool, count, 50)) and its first 50 put, one each, into the first empty
/// entries of the n; the list shuffled in nine pots of eight (`exe::comp_util_shuffle_pointer_array`) and the rest
/// (n − 72 from entry 72); the KPN clubs in the Intertoto, the UEFA Cup and the Champions League appended, one pass
/// each. With fewer than 86, every empty entry takes the first Dutch amateur club not yet in the list (if any).
/// Returns 1.
/// Original: `HolCup::select_entrants` (0x5f4820, hol_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn hol_cup_select_entrants(this: usize) -> u32 {
    write_u32(this + ita_cup_fields::ENTRANT_CLUB_COUNT, 0x56);
    let list = exe::crt_calloc(4, 0x56);
    write_usize(this + ita_cup_fields::ENTRANT_CLUBS, list);
    if list == 0 {
        show_error_at(0x9cedd4, 0x32c);
        return 0;
    }
    covered!("sbF_7 dutch cup entrants");
    let entry = |i: i32| list.wrapping_add(4 * i as usize);
    // simplification: the original's pool is the uninitialised stack; with fewer than 50 Dutch amateur clubs it would
    // place leftovers where the copy places nulls (the database has more)
    let mut amateurs = [0usize; 300];
    let mut amateur_count = 0i16;
    let mut k = 0i16;
    while (k as i32) < read_i32(G_CLUB_COUNT) {
        let c = club_at(k as isize).addr();
        if is_dutch_amateur(c) && amateur_count < 0x12c {
            amateurs[amateur_count as usize] = c;
            amateur_count += 1;
        }
        k = k.wrapping_add(1);
    }
    let mut n = 0i32;
    append_dutch_cup_entrants(this, &mut n, |c| plays_in(c, KEY_CLUB_COMP_DUTCH_TOTO_DIVISION) && read_usize(c + club::NATION) == holland());
    append_dutch_cup_entrants(this, &mut n, |c| plays_in(c, KEY_CLUB_COMP_DUTCH_KPN_TELECOMPETITION) && Club(c).continental_comp_id() == u32::MAX);
    for k in 0..0x32 {
        write_usize(entry(n), read_usize(entry(k))); // sbF7: the first 50 again, to be zeroed as copies
        n += 1;
    }
    exe::crt_qsort(list, n as u32, 4, 0x53fe50usize);
    let mut i = 1i16;
    while ((i - 1) as i32) < n {
        let mut j = i;
        while (j as i32) < n {
            if read_usize(entry((i - 1) as i32)) == read_usize(entry(j as i32)) {
                write_usize(entry(j as i32), 0);
            }
            j = j.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    exe::comp_util_sort_and_shuffle_top_clubs(amateurs.as_mut_ptr() as usize, amateur_count as u32, 0x32);
    for &a in &amateurs[..0x32] {
        let mut e = 0i16;
        while (e as i32) < n {
            if read_usize(entry(e as i32)) == 0 {
                write_usize(entry(e as i32), a);
                break;
            }
            e = e.wrapping_add(1);
        }
    }
    for pot in 0..9usize {
        exe::comp_util_shuffle_pointer_array(list + 0x20 * pot, 8); // sbF7: the pots of eight
    }
    exe::comp_util_shuffle_pointer_array(list + 0x120, (n - 0x48) as u32);
    for key in [KEY_CLUB_COMP_INTER_TOTO_CUP, KEY_CLUB_COMP_UEFA_CUP, KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE] {
        append_dutch_cup_entrants(this, &mut n, |c| plays_in(c, KEY_CLUB_COMP_DUTCH_KPN_TELECOMPETITION) && Club(c).continental_comp_id() == read_u32(key));
    }
    let places = read_i32(this + ita_cup_fields::ENTRANT_CLUB_COUNT);
    if n < places {
        let mut e = 0i16;
        while (e as i32) < read_i32(this + ita_cup_fields::ENTRANT_CLUB_COUNT) {
            if read_usize(entry(e as i32)) == 0 {
                let places = read_i32(this + ita_cup_fields::ENTRANT_CLUB_COUNT);
                for k in 0..read_i32(G_CLUB_COUNT).max(0) {
                    let c = club_at(k as isize).addr();
                    if is_dutch_amateur(c) && !(0..places).any(|x| read_usize(entry(x)) == c) {
                        write_usize(entry(e as i32), c);
                        break;
                    }
                }
            }
            e = e.wrapping_add(1);
        }
    }
    1
}

/// Comparer for `hol_cup_select_entrants`: the plan (the professionals, each once, and the European clubs in order),
/// then only the original (it allocates the list and shuffles with the game's random numbers); returns 1, 86 places,
/// every planned professional in the list and the European clubs in the list in the planned order.
pub(crate) unsafe extern "thiscall" fn compare_hol_cup_select_entrants(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    let clubs: Vec<usize> = (0..read_i32(G_CLUB_COUNT).max(0) as isize).map(|k| club_at(k).addr()).collect();
    let pros: Vec<usize> = clubs.iter().copied().filter(|&c| (plays_in(c, KEY_CLUB_COMP_DUTCH_TOTO_DIVISION) && read_usize(c + club::NATION) == holland())
        || (plays_in(c, KEY_CLUB_COMP_DUTCH_KPN_TELECOMPETITION) && Club(c).continental_comp_id() == u32::MAX)).collect();
    let europe: Vec<usize> = [KEY_CLUB_COMP_INTER_TOTO_CUP, KEY_CLUB_COMP_UEFA_CUP, KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE].iter()
        .flat_map(|&key| clubs.iter().copied().filter(move |&c| plays_in(c, KEY_CLUB_COMP_DUTCH_KPN_TELECOMPETITION) && Club(c).continental_comp_id() == read_u32(key)))
        .collect();
    let r = o(this);
    let list = read_usize(this + ita_cup_fields::ENTRANT_CLUBS);
    let got: Vec<usize> = if list != 0 { (0..0x56).map(|k| read_usize(list + 4 * k)).collect() } else { Vec::new() };
    let at: Vec<Option<usize>> = europe.iter().map(|c| got.iter().position(|x| x == c)).collect();
    let ok = r == 1 && read_i32(this + ita_cup_fields::ENTRANT_CLUB_COUNT) == 0x56 && pros.len() + europe.len() <= 0x56
        && pros.iter().all(|c| got.contains(c)) && at.iter().all(|p| p.is_some()) && at.windows(2).all(|w| w[0] < w[1]);
    SCOREBOARD.record(ok, || format!("hol_cup_select_entrants({:#x}) returned {}; professionals {}, european {:?}", this, r, pros.len(), at));
    r
}

/// Rule: the Dutch Cup is set up on its first full day of the season (the game's current season equal to the cup's
/// year): the 86 entrants, the predicted positions, the cup's own group with its fixtures and order, and the 19 group
/// stages; then, every day, the league day.
///
/// Once (`league::ACTIVE` = 0), when the year at +0x439 of [`G_UEFA_SEEDING`] (a word, zero-extended) equals the cup's
/// year (sign-extended) and `full` ≠ 0: HolCup::select_entrants (0x5f4820, thiscall, ret 0, unchecked), the predicted
/// positions (virtual `comp_vtable::SET_PREDICTED_POSITIONS`), the cup's own group (HolCup::create_group_table
/// 0x5f3d70; 0: the box at line 0x224 and 0), the fixtures (`exe::league_create_fixtures`; 0: line 0x22a), the order
/// (`exe::league_sort_table`(this, 0); 0: line 0x231), the stages of groups 0 to 18 (HolCup::create_group_stage
/// 0x5f4220, push g -> thiscall, ret 4) and `league::ACTIVE` = 1. Then the league day (`exe::league_update_for_date`
/// (this, today, full), thiscall, ret 8), whose byte it returns (the original returns only al = 0 after a box; the
/// callers read the byte).
/// Original: `HolCup::update_for_date` (0x5f4020, hol_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn hol_cup_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let t = League(this);
    if t.active() == 0 && read_u16(read_usize(G_UEFA_SEEDING) + 0x439) as i32 == t.year() as i32 && full != 0 { // sbF7: the Dutch Cup season
        covered!("sbF_7 dutch cup set up");
        let call = |f: usize| {
            let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(f);
            f(this)
        };
        call(0x5f4820); // HolCup::select_entrants
        call(read_usize(read_usize(this) + comp_vtable::SET_PREDICTED_POSITIONS));
        if call(0x5f3d70) == 0 {
            // HolCup::create_group_table
            show_error_at(0x9cedd4, 0x224);
            return 0;
        }
        if exe::league_create_fixtures(t) == 0 {
            show_error_at(0x9cedd4, 0x22a);
            return 0;
        }
        if exe::league_sort_table(t, 0usize) == 0 {
            show_error_at(0x9cedd4, 0x231);
            return 0;
        }
        let group: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x5f4220usize); // HolCup::create_group_stage
        for g in 0..0x13 {
            group(this, g);
        }
        t.set_active(1);
    }
    exe::league_update_for_date(t, today, full) as u32
}

/// Comparer for `hol_cup_update_for_date`: only the original (it sets the cup up and plays the day); on the set-up day
/// (the copy's condition) `league::ACTIVE` = 1, the entrant list and the 19 group stages made and the current stage 18;
/// otherwise `league::ACTIVE` unchanged.
pub(crate) unsafe extern "thiscall" fn compare_hol_cup_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I1%%);
    let t = League(this);
    let before = t.active();
    let set_up = before == 0 && read_u16(read_usize(G_UEFA_SEEDING) + 0x439) as i32 == t.year() as i32 && full != 0;
    let r = o(this, today, full);
    let ok = if set_up {
        t.active() == 1 && read_usize(this + ita_cup_fields::ENTRANT_CLUBS) != 0 && (0..0x13usize).all(|g| read_usize(t.stages() + 4 * g) != 0) && t.current_stage() == 0x12
    } else {
        t.active() == before
    };
    SCOREBOARD.record(ok, || format!("hol_cup_update_for_date({:#x}, full {}) set up {} active {} -> {}", this, full, set_up, before, t.active()));
    r
}
