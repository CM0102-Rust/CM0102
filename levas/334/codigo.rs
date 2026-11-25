/// Copies `bytes` bytes forward, a dword at a time and the rest a byte at a time (`rep movsd` then `movsw`/`movsb`):
/// the original's record copies, whose source and destination may overlap.
unsafe fn trim_copy_forward(dst: usize, src: usize, bytes: usize) {
    let mut k = 0;
    while k + 4 <= bytes {
        write_u32(dst + k, read_u32(src + k));
        k += 4;
    }
    while k < bytes {
        write_u8(dst + k, read_u8(src + k));
        k += 1;
    }
}

/// The person record with id `id` (the staff table before the cut, `staff::SIZE` bytes each).
unsafe fn trim_staff(id: i32) -> usize {
    read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(staff::SIZE))
}

/// The nation's reputation (+0x8e) clamped to 5..15, the size of its lists of best staff.
unsafe fn trim_nation_weight(nation: usize) -> i32 {
    let r = read_i16(nation + nation::REPUTATION);
    if r <= 5 { 5 } else if r as i32 >= 15 { 15 } else { r as i32 }
}

/// One of a nation's lists of best staff (`ids` at `list`, then the count and the slot of the weakest): while the list
/// has room (`cap`), the person goes in, and becomes the weakest when the list was empty or the person is weaker than
/// the weakest; when full, a person stronger than the weakest takes the weakest's slot and the weakest is searched
/// again from the start. `own` is the person's rating and `ability(id)` that of a listed person (signed).
unsafe fn trim_offer_to_list(list: usize, count_at: usize, weakest_at: usize, cap: i32, id: i32, own: i16, ability: impl Fn(i32) -> i16) {
    let n = read_i32(count_at);
    let weakest = || ability(read_i32(list + 4 * read_i32(weakest_at) as usize));
    if n < cap {
        write_i32(list + 4 * n as usize, id);
        if n == 0 || own < weakest() {
            write_i32(weakest_at, n);
        }
        write_i32(count_at, n + 1);
    } else if weakest() < own {
        write_i32(list + 4 * read_i32(weakest_at) as usize, id);
        for k in 0..n {
            if k == 0 || ability(read_i32(list + 4 * k as usize)) < weakest() {
                write_i32(weakest_at, k);
            }
        }
    }
}

/// The smaller databases (size not 1 or 2): each nation keeps its best non-players (up to weight × 10 / 15, by current
/// ability +4 of the non-player sheet) and its best players (up to weight × 100 / 15, by current ability of the
/// player sheet), the weight being the nation's reputation clamped to 5..15; those people are marked in `keep_flags`
/// (one dword per person id). The per-nation lists (0x1c8 bytes each: 10 non-player ids, count, weakest slot, 100
/// player ids, count, weakest slot) come from `calloc`; without them, the fatal Database.cpp box (line 0x2ad).
unsafe fn trim_mark_best_staff_per_nation(keep_flags: usize, last: usize) {
    let lists = exe::crt_calloc(read_u32(G_NATION_COUNT), 0x1c8);
    if lists == 0 {
        database_fatal_error(0x2ad);
    }
    let non_player_ability = |id: i32| read_i16(read_usize(trim_staff(id) + staff::NON_PLAYER) + non_player::CURRENT_ABILITY);
    let player_ability = |id: i32| read_i16(read_usize(trim_staff(id) + staff::PLAYER) + player_sheet::CURRENT_ABILITY);
    let mut p = read_usize(G_STAFF_TABLE);
    while p <= last {
        let nation = read_usize(p + staff::NATION);
        if nation != 0 {
            let l = lists + read_i32(nation) as usize * 0x1c8;
            let kind = read_u8(p + staff::CLASSIFICATION);
            let non_player = read_usize(p + staff::NON_PLAYER);
            if (kind == 1 || kind == 3) && non_player != 0 {
                let own = read_i16(non_player + non_player::CURRENT_ABILITY);
                trim_offer_to_list(l, l + 0x28, l + 0x2c, trim_nation_weight(nation) * 10 / 15, read_i32(p), own, non_player_ability);
            }
            let player = read_usize(p + staff::PLAYER);
            if (kind == 2 || kind == 3) && player != 0 {
                let weight = trim_nation_weight(read_usize(p + staff::NATION));
                let own = read_i16(player + player_sheet::CURRENT_ABILITY);
                trim_offer_to_list(l + 0x30, l + 0x1c0, l + 0x1c4, weight * 100 / 15, read_i32(p), own, player_ability);
            }
        }
        p += staff::SIZE;
    }
    for k in 0..read_i32(G_NATION_COUNT).max(0) as usize {
        let l = lists + k * 0x1c8;
        for (ids, count) in [(l, l + 0x28), (l + 0x30, l + 0x1c0)] {
            let mut i = 0;
            while i < read_i32(count) {
                write_u32(keep_flags + 4 * read_i32(ids + 4 * i as usize) as usize, 1);
                i += 1;
            }
        }
    }
    exe::crt_free(lists);
}

/// True when the person's first and second names are those of `first` and `second` (C strings in the exe).
unsafe fn trim_has_names(p: usize, first: usize, second: usize) -> bool {
    let s = |a: usize| core::ffi::CStr::from_ptr(a as *const core::ffi::c_char).to_bytes();
    s(read_usize(p + staff::FIRST_NAME)) == s(first) && s(read_usize(p + staff::SECOND_NAME)) == s(second)
}

/// Whether the cut keeps person `p` (not of type 6), in the original's order of tests and draws (`exe::utils_random`).
/// `dest` is the row the next kept person will take; `size` the database size option, `wide` true for sizes 1 and 2.
///
/// Kept: marked in `keep_flags` (the best per nation), the 16 human manager rows at the end, Collin Samuel and Devon
/// Mitchell. Then, for the smaller sizes, a nation, second nation or club nation whose league is selected
/// (`nation::LEAGUE_SELECTED`); for sizes 1 and 2 the same with draws: the nation selected and (a club out of the lowest
/// division key or 1 in 20), the second nation selected with a club out of it and 1 in 50, the club's nation selected and
/// (out of it or 1 in 35). A national team manager or assistant (job 5 or 12). A club of reputation 7500 or more (sizes 1
/// and 2: also its nation's league standard above 10 and 1 in 3). International caps (sizes 1 and 2: only a non-player
/// of a club of a selected nation). For the smaller sizes, a non-player with ability 150 or potential 150 (or −1, −2),
/// home reputation 160 or current 155; then a non-player of world reputation 150 or a player of current ability 150;
/// except for size 2, a player of potential 150 (or −1, −2), home reputation 160 or current 155; a player of world
/// reputation 150.
///
/// Dropped (sizes 1 and 2): when the nation of the `dest` row has a league standard below 6; a person of China, Mexico,
/// the Czech Republic, South Africa, South Korea, Yugoslavia, Austria or Switzerland, 19 in 20. Size 2: when the `dest`
/// row's nation has a standard below 10 and the person has a player sheet with potential below 40, or 14 in 15
/// otherwise. Sizes 1 and 2: when the `dest` row's player sheet has potential below 80, 11 in 12. The rest kept by
/// chance: 1 in 30 for size 4, 1 in 75 for size 1, 1 in 100 for size 2, never for the others.
///
/// Defect of the original: three tests read the nation and the player sheet of the `dest` row (a person already kept
/// or about to be overwritten), not those of the person judged.
unsafe fn trim_keeps_person(p: usize, dest: usize, size: u8, wide: bool, keep_flags: usize) -> bool {
    let id = read_i32(p);
    if read_u32(keep_flags + 4 * id as usize) != 0 || id >= read_i32(G_STAFF_COUNT) - 16 {
        return true;
    }
    if read_usize(p + staff::FIRST_NAME) != 0 && read_usize(p + staff::SECOND_NAME) != 0
        && (trim_has_names(p, 0x9b5ac0, 0x9b5ab8) || trim_has_names(p, 0x9b5ab0, 0x9b5aa4)) // "Collin" "Samuel", "Devon" "Mitchell"
    {
        covered!("sx08b_2 named exception kept");
        return true;
    }
    let random = |n: i32| exe::utils_random(n);
    let selected = |n: usize| n != 0 && read_u8(n + nation::LEAGUE_SELECTED) != 0;
    let nation = read_usize(p + staff::NATION);
    let second_nation = read_usize(p + staff::SECOND_NATION);
    let club = read_usize(p + staff::CLUB_JOB);
    // the club's division is the lowest one (the key) or missing; only asked with a club
    let lowest_division = || {
        let d = read_usize(club + club::DIVISION);
        d == 0 || read_i32(d) == read_i32(KEY_CLUB_COMP_A_LOWER_DIVISION)
    };
    let club_nation_selected = || club != 0 && selected(read_usize(club + club::NATION));
    if !wide {
        if selected(nation) || selected(second_nation) || club_nation_selected() {
            return true;
        }
    } else if selected(nation) && ((club != 0 && !lowest_division()) || random(0x14) == 0)
        || selected(second_nation) && club != 0 && !lowest_division() && random(0x32) == 0
        || club_nation_selected() && (!lowest_division() || random(0x23) == 0)
    {
        return true;
    }
    if read_usize(p + staff::NATIONAL_JOB) != 0 && matches!(read_u8(p + staff::JOB_FOR_NATION), 5 | 0xc) {
        return true;
    }
    let big_club = club != 0 && read_i16(club + club::REPUTATION) >= 0x1d4c;
    if big_club && !wide {
        return true;
    }
    if wide && club != 0 {
        let club_nation = read_usize(club + club::NATION);
        if club_nation != 0 && read_i8(club_nation + nation::LEAGUE_STANDARD) > 10 && read_i16(club + club::REPUTATION) >= 0x1d4c && random(3) == 0 {
            return true;
        }
    }
    let kind = read_u8(p + staff::CLASSIFICATION);
    let non_player = read_usize(p + staff::NON_PLAYER);
    let player = read_usize(p + staff::PLAYER);
    let np_at_least = |o: usize, v: i16| (kind == 1 || kind == 3) && non_player != 0 && read_i16(non_player + o) >= v;
    let pl_at_least = |o: usize, v: i16| (kind == 2 || kind == 3) && player != 0 && read_i16(player + o) >= v;
    let unknown = |a: usize| matches!(read_i16(a), -1 | -2); // potential not set
    if read_u8(p + staff::INT_APPS) != 0 {
        if !wide {
            return true;
        }
    }
    if wide {
        if club_nation_selected() && kind == 1 {
            return true;
        }
    } else if np_at_least(non_player::CURRENT_ABILITY, 0x96)
        || (kind == 1 || kind == 3) && non_player != 0
            && (read_i16(non_player + non_player::POTENTIAL_ABILITY) >= 0x96 || unknown(non_player + non_player::POTENTIAL_ABILITY))
        || np_at_least(non_player::HOME_REPUTATION, 0xa0)
        || np_at_least(non_player::CURRENT_REPUTATION, 0x9b)
    {
        return true;
    }
    if np_at_least(non_player::WORLD_REPUTATION, 0x96) || pl_at_least(player_sheet::CURRENT_ABILITY, 0x96) {
        return true;
    }
    if size != 2
        && ((kind == 2 || kind == 3) && player != 0
            && (read_i16(player + player_sheet::POTENTIAL_ABILITY) >= 0x96 || unknown(player + player_sheet::POTENTIAL_ABILITY))
            || pl_at_least(player_sheet::HOME_REPUTATION, 0xa0)
            || pl_at_least(player_sheet::CURRENT_REPUTATION, 0x9b))
    {
        return true;
    }
    if pl_at_least(player_sheet::WORLD_REPUTATION, 0x96) {
        return true;
    }
    let dest_nation = read_usize(dest + staff::NATION);
    if wide && dest_nation != 0 && read_i8(dest_nation + nation::LEAGUE_STANDARD) < 6 {
        return false;
    }
    if wide && nation != 0 {
        let k = read_i32(nation);
        let minor = [KEY_NATION_CHINA, KEY_NATION_MEXICO, KEY_NATION_CZECH_REPUBLIC, KEY_NATION_SOUTH_AFRICA,
            KEY_NATION_SOUTH_KOREA, KEY_NATION_YUGOSLAVIA, KEY_NATION_AUSTRIA, KEY_NATION_SWITZERLAND];
        if minor.iter().any(|&key| read_i32(key) == k) && random(0x14) != 0 {
            return false;
        }
    }
    if size == 2 && dest_nation != 0 && read_i8(dest_nation + nation::LEAGUE_STANDARD) < 10 && player != 0
        && (read_i16(player + player_sheet::POTENTIAL_ABILITY) < 0x28 || random(0xf) != 0)
    {
        return false;
    }
    if size == 1 || size == 2 {
        let dest_player = read_usize(dest + staff::PLAYER);
        if dest_player != 0 && read_i16(dest_player + player_sheet::POTENTIAL_ABILITY) < 0x50 && random(0xc) != 0 {
            return false;
        }
    }
    match size {
        4 => random(0x1e) == 0,
        1 => random(0x4b) == 0,
        2 => random(100) == 0,
        _ => false,
    }
}

/// Points every staff link of the other tables at the person's new row (`map[old id]`, 0 when cut): the clubs'
/// favoured and disliked staff, chairman, manager, assistant, directors, squad, coaches, scouts and physios; the
/// non-players' 8 staff links (+0x23..+0x3f); the preferences' 6 staff links (+0x1c..+0x30); the staff history's
/// person (+4). The staff competition history keeps ids (+0x12, +0x22, +0x32): the id at the new row, −1 when negative
/// or cut.
unsafe fn trim_remap_staff_links(map: usize) {
    let remap = |a: usize| {
        let s = read_usize(a);
        write_usize(a, if s == 0 { 0 } else { read_usize(map + 4 * read_i32(s) as usize) });
    };
    let each = |table: usize, count: usize, size: usize, fields: &[usize]| {
        let mut i = 0;
        while i < read_i32(count) {
            for &f in fields {
                remap(read_usize(table) + i as usize * size + f);
            }
            i += 1;
        }
    };
    let mut club_fields = vec![club::FAV_STAFF1, club::FAV_STAFF1 + 4, club::FAV_STAFF1 + 8, club::DIS_STAFF1, club::DIS_STAFF1 + 4,
        club::DIS_STAFF1 + 8, club::CHAIRMAN, club::MANAGER, club::ASSISTANT_MANAGER];
    for (start, n) in [(club::DIRECTORS, 3), (club::SQUAD, 50), (club::COACHES, 5), (club::SCOUTS, 7), (club::PHYSIOS, 3)] {
        club_fields.extend((0..n).map(|k| start + 4 * k));
    }
    each(G_CLUB_TABLE, G_CLUB_COUNT, club::SIZE, &club_fields);
    each(G_NON_PLAYER_TABLE, G_NON_PLAYER_COUNT, 0x44, &[0x23, 0x27, 0x2b, 0x2f, 0x33, 0x37, 0x3b, 0x3f]); // staff links
    each(G_PREFERENCES_TABLE, 0xae2370, 0x34, &[0x1c, 0x20, 0x24, 0x28, 0x2c, 0x30]); // preferences count; staff links
    each(G_STAFF_HISTORY_TABLE, G_STAFF_HISTORY_COUNT, 0x11, &[4]); // the person
    let mut i = 0;
    while i < read_i32(G_STAFF_COMP_HISTORY_COUNT) {
        for f in [0x12, 0x22, 0x32] { // winner ids
            let a = read_usize(G_STAFF_COMP_HISTORY_TABLE) + i as usize * 0x3a + f;
            let id = read_i32(a);
            let row = if id < 0 { 0 } else { read_usize(map + 4 * id as usize) };
            write_i32(a, if row == 0 { -1 } else { read_i32(row) });
        }
        i += 1;
    }
}

/// Moves each kept person to its new row (`map[id]`) with the new id, and its player sheet, non-player sheet and
/// preferences to the next rows of their tables (each with its new id, the person pointing at it). Returns the new
/// counts: (staff, players, non-players, preferences).
unsafe fn trim_compact_staff(map: usize) -> (i32, i32, i32, i32) {
    let (mut kept, mut players, mut non_players, mut prefs) = (0, 0, 0, 0);
    let (mut player_row, mut non_player_row, mut prefs_row) =
        (read_usize(G_PLAYER_DATA_TABLE), read_usize(G_NON_PLAYER_TABLE), read_usize(G_PREFERENCES_TABLE));
    let mut k = 0;
    while k < read_i32(G_STAFF_COUNT) {
        let dest = read_usize(map + 4 * k as usize);
        if dest != 0 {
            trim_copy_forward(dest, trim_staff(k), staff::SIZE);
            write_i32(dest, kept);
            kept += 1;
            for (field, row, size, n) in [(staff::PLAYER, &mut player_row, 0x46, &mut players),
                (staff::NON_PLAYER, &mut non_player_row, 0x44, &mut non_players),
                (staff::STAFF_PREFERENCES, &mut prefs_row, 0x34, &mut prefs)]
            {
                let sheet = read_usize(dest + field);
                if sheet != 0 {
                    trim_copy_forward(*row, sheet, size);
                    write_i32(*row, *n);
                    *n += 1;
                    write_usize(dest + field, *row);
                    *row += size;
                }
            }
        }
        k += 1;
    }
    (kept, players, non_players, prefs)
}

/// The rows for "virtual" staff: kept × 2 % (sizes 1 and 2) or 1 % (truncated); when that plus the kept and 1325 passes
/// the old count, the count becomes old + kept + 1325 (0 if negative) — a defect of the original (the guard was meant
/// to shrink it). Sets the new counts (staff kept + virtual, players and preferences + virtual, non-players + 10) and
/// reallocates the four tables (staff and players with 1325 spare rows); a table that moves is the fatal Database.cpp
/// box (lines 0x18e..0x191). Returns the virtual count.
unsafe fn trim_resize_tables(size: u32, old_count: i32, kept: i32, players: i32, non_players: i32, prefs: i32) -> i32 {
    let share = f64::from_bits(if matches!(size, 1 | 2) { 0x3f947ae147ae147b } else { 0x3f847ae147ae147b }); // 0.02, 0.01
    let mut virtual_staff = (kept as f64 * share) as i32;
    if virtual_staff + kept + 0x52d > old_count {
        virtual_staff = old_count + kept + 0x52d;
        if virtual_staff < 0 {
            virtual_staff = 0;
        }
    }
    write_i32(G_PLAYER_DATA_COUNT, virtual_staff + players);
    write_i32(G_STAFF_COUNT, kept + virtual_staff);
    write_i32(0xae2370, virtual_staff + prefs); // preferences count
    write_i32(G_NON_PLAYER_COUNT, non_players + 10);
    let count = |g: usize| read_i32(g) as u32;
    let moved = [
        (G_STAFF_TABLE, exe::crt_realloc(read_usize(G_STAFF_TABLE), (count(G_STAFF_COUNT) + 0x52d).wrapping_mul(staff::SIZE as u32))),
        (G_PLAYER_DATA_TABLE, exe::crt_realloc(read_usize(G_PLAYER_DATA_TABLE), (count(G_PLAYER_DATA_COUNT) + 0x52d).wrapping_mul(0x46))),
        (G_NON_PLAYER_TABLE, exe::crt_realloc(read_usize(G_NON_PLAYER_TABLE), count(G_NON_PLAYER_COUNT).wrapping_mul(0x44))),
        (G_PREFERENCES_TABLE, exe::crt_realloc(read_usize(G_PREFERENCES_TABLE), count(0xae2370).wrapping_mul(0x34))),
    ];
    for (row, (table, p)) in (0x18e..).zip(moved) {
        if p != read_usize(table) {
            database_fatal_error(row);
        }
    }
    virtual_staff
}

/// Fills the `virtual_staff` rows before the 16 human manager rows (from the end backwards): no names (−2), born on a
/// random day of 1982 (`exe::crt_rand` × 365 / 32768), the nation of a random kept person that has one (its staff count
/// +1), every link, date (the default date) and attribute cleared, job 11, playing squad 2, classification 2 (player);
/// the next player sheet and preferences row from the end of their tables. Preferences: 1 in 10 get as favourite club
/// the best of 50 random clubs of the nation (reputation + attendance). Player sheet: potential −2 in 1 of 100, else
/// −1 in 1 of 50, else 0; the rest 0.
unsafe fn trim_create_virtual_staff(virtual_staff: i32) {
    let count = read_i32(G_STAFF_COUNT);
    let mut temp = [0u8; 8];
    let set_date = |field: usize, date: usize, temp: &mut [u8; 8]| { exe::date_operator_3d(field, temp.as_mut_ptr() as usize, date); };
    for j in 0..virtual_staff {
        let row = count - j - 0x11;
        let s = trim_staff(row);
        write_i32(s, row);
        for f in [staff::FIRST_NAME, staff::SECOND_NAME, staff::COMMON_NAME] {
            write_i32(s + f, -2);
        }
        write_u16(s + staff::YEAR_OF_BIRTH, 0x7be);
        let day = ((exe::crt_rand() as i64 * 0x16d) / 0x8000) as u32;
        let mut birth = [0u8; 8];
        let d = exe::new_date(birth.as_mut_ptr() as usize, day, 0x7be);
        set_date(s + staff::DATE_OF_BIRTH, d, &mut temp);
        let nation = loop {
            let n = read_usize(trim_staff(exe::utils_random(read_i32(G_STAFF_COUNT) - virtual_staff - 0x10)) + staff::NATION);
            write_usize(s + staff::NATION, n);
            if n != 0 {
                break n;
            }
        };
        write_i32(nation + nation::NUMBER_STAFF, read_i32(nation + nation::NUMBER_STAFF) + 1);
        write_u32(s + staff::SECOND_NATION, 0);
        write_u8(s + staff::INT_APPS, 0);
        write_u8(s + staff::INT_GOALS, 0);
        write_u32(s + staff::NATIONAL_JOB, 0);
        write_u8(s + staff::JOB_FOR_NATION, 0);
        let mut default_date = |field: usize| {
            let mut default = [0u8; 8];
            let d = exe::date_date_54a110(default.as_mut_ptr() as usize);
            set_date(s + field, d, &mut temp);
        };
        default_date(staff::DATE_JOINED_NATION);
        default_date(0x31); // the national job's second date
        write_u32(s + staff::CLUB_JOB, 0);
        write_u8(s + staff::JOB_FOR_CLUB, 0xb);
        default_date(staff::DATE_JOINED_CLUB);
        default_date(staff::DATE_EXPIRES_CLUB);
        write_u32(s + staff::WAGE, 0);
        write_u32(s + staff::VALUE, 0);
        for f in staff::ADAPTABILITY..=staff::TEMPERAMENT {
            write_u8(s + f, 0);
        }
        write_u8(s + staff::PLAYING_SQUAD, 2);
        write_u8(s + staff::CLASSIFICATION, 2);
        write_u8(s + 0x60, 0); // the byte after the classification
        let player = read_usize(G_PLAYER_DATA_TABLE) + (read_i32(G_PLAYER_DATA_COUNT) - j - 1) as usize * 0x46;
        write_usize(s + staff::PLAYER, player);
        let prefs_id = read_i32(0xae2370) - j - 1; // preferences count
        let prefs = read_usize(G_PREFERENCES_TABLE) + prefs_id as usize * 0x34;
        write_u32(s + staff::NON_PLAYER, 0);
        write_usize(s + staff::STAFF_PREFERENCES, prefs);
        write_u8(s + staff::SQUAD_SELECTED_FOR, 0);
        write_i32(prefs, prefs_id);
        write_u32(prefs + preferences::STAFF_FAVOURITE_CLUBS1, 0);
        if exe::utils_random(10) == 0 {
            covered!("sx08b_2 virtual staff favourite club");
            let mut best = 0usize;
            for _ in 0..0x32 {
                let c = club_at(exe::utils_random(read_i32(G_CLUB_COUNT)) as isize).addr();
                let strength = |c: usize| read_i16(c + club::REPUTATION) as i32 + read_i32(c + club::ATTENDANCE);
                if read_usize(c + club::NATION) == read_usize(s + staff::NATION) && (best == 0 || strength(c) > strength(best)) {
                    best = c;
                }
            }
            write_usize(prefs + preferences::STAFF_FAVOURITE_CLUBS1, best);
        }
        for k in 0..11 {
            write_u32(prefs + 8 + 4 * k, 0); // the other preference slots
        }
        write_i32(player, read_i32(G_PLAYER_DATA_COUNT) - j - 1);
        write_u8(player + 4, 0); // the byte before the abilities
        write_u16(player + player_sheet::CURRENT_ABILITY, 0);
        let potential: i16 = if exe::utils_random(100) == 0 { -2 } else if exe::utils_random(0x32) == 0 { -1 } else { 0 };
        write_i16(player + player_sheet::POTENTIAL_ABILITY, potential);
        for f in [player_sheet::HOME_REPUTATION, player_sheet::CURRENT_REPUTATION, player_sheet::WORLD_REPUTATION] {
            write_u16(player + f, 0);
        }
        for k in player_sheet::GOALKEEPER..0x45 {
            write_u8(player + k, 0); // positions and attributes
        }
    }
}

/// Cuts the staff, player, non-player and preferences tables by the database size option at the creation of a game.
///
/// Without the nation table: the Database.cpp box (line 0x282) and nothing else. First the extra backroom staff
/// (`database_generate_extra_non_playing_staff`); then a row map (`new`, one pointer per person id; nothing at all
/// when it fails) and the keep flags (`calloc`; fatal box 0x2a2 without them). For sizes other than 1 and 2, the best
/// staff of each nation are marked (`trim_mark_best_staff_per_nation`). Each person gets a new row in table order
/// (`trim_keeps_person`) or is cut (type 6, the reserves, always). The links of the other tables are moved to the new
/// rows (`trim_remap_staff_links`), the people and their sheets compacted (`trim_compact_staff`), the tables resized
/// with the "virtual" staff rows (`trim_resize_tables`); the last 10 non-player rows are copies of random kept ones.
/// The nations' staff counts are recounted (people with a player sheet), the 16 human manager rows moved to the end of
/// the table (after the virtual rows, ids renumbered), the virtual staff created (`trim_create_virtual_staff`) and the
/// summary lines written to the status text (the staff one overwritten by the player one, and printed with 1325 in
/// place of the virtual count).
///
/// Defects of the original: see `trim_keeps_person` (rows read in place of the person) and `trim_resize_tables` (the
/// guard grows the virtual count); with fewer than 16 virtual rows the move of the human manager rows overlaps itself.
/// Original: `database_trim_staff_tables` (0x524d70, Database.cpp, cdecl).
pub(crate) unsafe extern "C" fn database_trim_staff_tables(size: u32) {
    let old_count = read_i32(G_STAFF_COUNT);
    let map = exe::crt_operator_new((old_count as u32).wrapping_mul(4));
    if map == 0 {
        return;
    }
    if read_usize(G_NATION_TABLE) == 0 {
        database_error_box(0x282);
        exe::crt_operator_delete(map);
        return;
    }
    let wide = matches!(size as u8, 1 | 2);
    database_generate_extra_non_playing_staff();
    let keep_flags = exe::crt_calloc(read_u32(G_STAFF_COUNT), 4);
    if keep_flags == 0 {
        database_fatal_error(0x2a2);
    }
    let last = trim_staff(read_i32(G_STAFF_COUNT) - 1);
    if !wide {
        trim_mark_best_staff_per_nation(keep_flags, last);
    }
    let mut dest = read_usize(G_STAFF_TABLE);
    let mut p = dest;
    while p <= last {
        let id = read_i32(p) as usize;
        let row = if read_u8(p + staff::CLASSIFICATION) == 6 {
            0
        } else if trim_keeps_person(p, dest, size as u8, wide, keep_flags) {
            dest += staff::SIZE;
            dest - staff::SIZE
        } else {
            0
        };
        write_usize(map + 4 * id, row);
        p += staff::SIZE;
    }
    exe::crt_free(keep_flags);
    trim_remap_staff_links(map);
    let (kept, players, non_players, prefs) = trim_compact_staff(map);
    let virtual_staff = trim_resize_tables(size, read_i32(G_STAFF_COUNT), kept, players, non_players, prefs);
    for i in 0..10 {
        let from = read_usize(G_NON_PLAYER_TABLE) + exe::utils_random(non_players) as usize * 0x44;
        trim_copy_forward(read_usize(G_NON_PLAYER_TABLE) + (read_i32(G_NON_PLAYER_COUNT) - i - 1) as usize * 0x44, from, 0x44);
    }
    exe::crt_operator_delete(map);
    for k in 0..read_i32(G_NATION_COUNT).max(0) as usize {
        write_u32(read_usize(G_NATION_TABLE) + k * nation::SIZE + nation::NUMBER_STAFF, 0);
    }
    let mut i = 0;
    while i < read_i32(G_STAFF_COUNT) - virtual_staff {
        let s = trim_staff(i);
        let nation = read_usize(s + staff::NATION);
        if read_usize(s + staff::PLAYER) != 0 && nation != 0 {
            write_i32(nation + nation::NUMBER_STAFF, read_i32(nation + nation::NUMBER_STAFF) + 1);
        }
        i += 1;
    }
    let mut row = read_i32(G_STAFF_COUNT) - 0x10;
    while row < read_i32(G_STAFF_COUNT) {
        trim_copy_forward(trim_staff(row), trim_staff(row - virtual_staff), staff::SIZE);
        write_i32(trim_staff(row), row);
        row += 1;
    }
    trim_create_virtual_staff(virtual_staff);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    format(0xde1f64, 0x9b5a7c, read_i32(G_STAFF_COUNT), 0x52d); // "Staff table has %d(+%d Virtual) entries"
    format(0xde1f64, 0x9b5a60, read_i32(G_PLAYER_DATA_COUNT)); // "Player table has %d entries"
}

// ponytail: cuts, compacts and reallocates the staff tables in place and draws; it cannot run twice. Only the
// original; the copy is proved by the save proof (the .qst carries the cut tables).
pub(crate) unsafe extern "C" fn compare_database_trim_staff_tables(size: u32) {
    original_fn::<unsafe extern "C" fn(u32)>(IDX_DATABASE_TRIM_STAFF_TABLES)(size)
}

// ---------- sx10_1: the fog of war in the save: building it and reading it (fog_of_war.cpp) ----------

/// The source path of fog_of_war.cpp, for the error boxes ("E:\dev\CM3\cm3 00-01\cm3\code\fog_of_war.cpp").
pub(crate) const FOG_OF_WAR_CPP: usize = 0x9cc27c;
/// The name of the fog of war item in the save archive: "fog.dat".
pub(crate) const FOG_DAT: usize = 0x9cc274;
/// The fog of war object holds one map pointer per human manager: 16 slots, the manager with id `id` at slot
/// `G_STAFF_COUNT - id - 1` (the human managers are the last people of the table).
pub(crate) const FOG_MAP_SLOTS: usize = 16;
/// The first data version whose saves carry fog.dat; older saves get fresh maps for their human managers.
const FOG_DAT_VERSION: i32 = 0x3e036;
/// FOG_OF_WAR::load (thiscall(fog, save) ret 4, 1 on success).
pub(crate) const FN_FOG_OF_WAR_LOAD: usize = 0x5af6e0;
/// FOG_OF_WAR::add_manager_map (thiscall(fog, manager) ret 4).
pub(crate) const FN_FOG_OF_WAR_ADD_MANAGER_MAP: usize = 0x5afd70;
/// FOG_OF_WAR_MAP::load (thiscall(map, file) ret 4, 1 on success).
pub(crate) const FN_FOG_OF_WAR_MAP_LOAD: usize = 0x5b0010;

/// A new fog of war map for `manager` (0: none), as the original's `new FOG_OF_WAR_MAP(manager)`: 8 bytes from
/// `operator new` and the map constructor (FOG_OF_WAR_MAP::FOG_OF_WAR_MAP 0x5aff00, thiscall ret 4); 0 when the
/// allocation fails.
unsafe fn fog_map_new(manager: usize) -> usize {
    const FOG_OF_WAR_MAP_NEW: usize = 0x5aff00; // FOG_OF_WAR_MAP::FOG_OF_WAR_MAP, thiscall(map, manager) ret 4
    let map = exe::crt_operator_new(8);
    if map == 0 {
        return 0;
    }
    let new: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FOG_OF_WAR_MAP_NEW);
    new(map, manager)
}

/// The person at `k` places from the end of the people table, where the 16 human managers live.
unsafe fn fog_human_slot_person(k: usize) -> Staff {
    staff_at((read_i32(G_STAFF_COUNT) as isize).wrapping_sub(k as isize).wrapping_sub(1))
}

/// Rule: the fog of war starts with no maps; with attribute masking on and a save to load, the maps come from the save,
/// and when that fails each human manager gets a fresh map.
///
/// `FOG_OF_WAR::FOG_OF_WAR(save)`: clears the 16 map slots. With attribute masking on (`G_ATTRIBUTE_MASKING_ENABLED`)
/// and a save, `FOG_OF_WAR::load(save)`; when it returns 0, each of the 16 people at the end of the table
/// (`G_STAFF_COUNT - k - 1`, k = 0..15) whose classification is 1 (a human manager) gets a map through
/// `FOG_OF_WAR::add_manager_map`. A new game passes no save and starts with no maps (the human managers get theirs when
/// they are added). Returns `this`.
/// Original: `FOG_OF_WAR::FOG_OF_WAR` (0x5af630, fog_of_war.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn fog_of_war_new(this: usize, save: usize) -> usize {
    for k in 0..FOG_MAP_SLOTS {
        write_usize(this + 4 * k, 0);
    }
    if read_u8(G_ATTRIBUTE_MASKING_ENABLED) == 0 || save == 0 {
        return this;
    }
    let load: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FOG_OF_WAR_LOAD);
    if load(this, save) != 0 { // sx10_1: the maps came from the save
        return this;
    }
    covered!("sx10_1 fresh maps after a failed load");
    let add: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_FOG_OF_WAR_ADD_MANAGER_MAP);
    for k in 0..FOG_MAP_SLOTS {
        let person = fog_human_slot_person(k);
        if person.classification() == 1 {
            add(this, person.addr());
        }
    }
    this
}

/// Comparer for `fog_of_war_new`.
///
/// It reads the save item and allocates the maps: it cannot run twice. Only the original runs; the copy is proven by
/// the save proof (the holiday save loaded with and without the routines, levas/N/CARGA) and by the career, which
/// builds the object at the game creation.
pub(crate) unsafe extern "thiscall" fn compare_fog_of_war_new(this: usize, save: usize) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_FOG_OF_WAR_NEW)(this, save)
}

/// Rule: reads the fog of war from the save; saves older than fog.dat give each human manager a fresh map.
///
/// `FOG_OF_WAR::load(save)`: with a data version (`G_GAME_DATA_VERSION`, absolute value) below 0x3e036, for each of the
/// 16 people at the end of the table that is a human manager (classification 1), a new map for that manager
/// (`new FOG_OF_WAR_MAP(manager)`) in the manager's slot, whatever attribute masking says; returns 1. Otherwise: no save:
/// the fog_of_war.cpp 0x99 box and 0. The "fog.dat" item of the save (`zopen` with 0, 0x16, 4; missing: the 0xa1 box and
/// 0). The masking flag (1 byte into `G_ATTRIBUTE_MASKING_ENABLED`; short: the 0xa7 box and 0). Masking off: the item
/// closed and 1. Masking on: for each slot a presence word (short: the 0xb8 box and 0); 0 empties the slot; otherwise a
/// new map without a manager goes in the slot (allocation failed: the 0xc4 box, every map freed as in the destructor,
/// and 0) and reads itself (`FOG_OF_WAR_MAP::load`; failed: the 0xd1 box, every map freed, the item closed and 0). Then
/// the item is closed and 1.
///
/// Defects of the original: the 0xa7, 0xb8 and 0xc4 failures return without closing the save item; freeing the maps
/// after a failure leaves the slots pointing at the freed maps; maps already in the slots are overwritten without being
/// freed.
/// Original: `FOG_OF_WAR::load` (0x5af6e0, fog_of_war.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn fog_of_war_load(this: usize, save: usize) -> u32 {
    if read_i32(G_GAME_DATA_VERSION).wrapping_abs() < FOG_DAT_VERSION {
        covered!("sx10_1 save without fog.dat");
        for k in 0..FOG_MAP_SLOTS {
            let person = fog_human_slot_person(k);
            if person.classification() != 1 || person.is_null() {
                continue;
            }
            let slot = read_i32(G_STAFF_COUNT).wrapping_sub(read_i32(person.addr())).wrapping_sub(1);
            write_usize(this.wrapping_add((slot as usize).wrapping_mul(4)), fog_map_new(person.addr()));
        }
        return 1;
    }
    if save == 0 {
        show_error_at(FOG_OF_WAR_CPP, 0x99);
        return 0;
    }
    let mut z = exe::zopen(save, FOG_DAT, 0, 0x16, 4);
    if z == 0 {
        show_error_at(FOG_OF_WAR_CPP, 0xa1);
        return 0;
    }
    if exe::zread(G_ATTRIBUTE_MASKING_ENABLED, 1, 1, z) != 1 {
        show_error_at(FOG_OF_WAR_CPP, 0xa7);
        return 0;
    }
    if read_u8(G_ATTRIBUTE_MASKING_ENABLED) == 0 {
        exe::zclose(&mut z);
        return 1; // sx10_1: masking off, nothing more in fog.dat
    }
    covered!("sx10_1 maps read from the save");
    let map_load: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FOG_OF_WAR_MAP_LOAD);
    for k in 0..FOG_MAP_SLOTS {
        let slot = this + 4 * k;
        let mut present: u32 = 0;
        if exe::zread(&mut present as *mut u32 as usize, 4, 1, z) != 1 {
            show_error_at(FOG_OF_WAR_CPP, 0xb8);
            return 0;
        }
        if present == 0 {
            write_usize(slot, 0);
            continue;
        }
        let map = fog_map_new(0);
        write_usize(slot, map);
        if map == 0 {
            show_error_at(FOG_OF_WAR_CPP, 0xc4);
            fog_free_maps(this);
            return 0;
        }
        if map_load(map, z) == 0 {
            show_error_at(FOG_OF_WAR_CPP, 0xd1);
            fog_free_maps(this);
            exe::zclose(&mut z);
            return 0;
        }
    }
    exe::zclose(&mut z);
    1
}

/// Comparer for `fog_of_war_load`.
///
/// It reads the save item and allocates the maps: it cannot run twice. Only the original runs; the copy is proven by
/// loading the holiday save in the save proof (levas/N/CARGA).
pub(crate) unsafe extern "thiscall" fn compare_fog_of_war_load(this: usize, save: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_FOG_OF_WAR_LOAD)(this, save)
}

/// Rule: a map is read back from fog.dat in the order it was written: its manager, then the knowledge of every person.
///
/// `FOG_OF_WAR_MAP::load(file)`: the manager's id (4 bytes; short: the fog_of_war.cpp 0x230 box and 0); an id below 0
/// or above `G_STAFF_COUNT`: the 0x235 box and 0; otherwise the map's manager (+4) is that person. Then the knowledge
/// grid (+0): `G_STAFF_COUNT` entries of 9 bytes (the level and the date it was last raised); short: the 0x23e box and
/// 0. Returns 1.
///
/// Defect of the original: an id equal to `G_STAFF_COUNT` is accepted and points one record past the table.
/// Original: `FOG_OF_WAR_MAP::load` (0x5b0010, fog_of_war.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn fog_of_war_map_load(this: usize, file: usize) -> u32 {
    let mut owner: i32 = 0;
    if exe::zread(&mut owner as *mut i32 as usize, 4, 1, file) != 1 {
        show_error_at(FOG_OF_WAR_CPP, 0x230);
        return 0;
    }
    if owner < 0 || owner > read_i32(G_STAFF_COUNT) {
        show_error_at(FOG_OF_WAR_CPP, 0x235);
        return 0;
    }
    covered!("sx10_1 map read");
    write_usize(this + 4, staff_at(owner as isize).addr()); // +4: the map's manager
    let count = read_u32(G_STAFF_COUNT);
    if exe::zread(read_usize(this), 9, count, file) != read_u32(G_STAFF_COUNT) { // +0: the knowledge grid
        show_error_at(FOG_OF_WAR_CPP, 0x23e);
        return 0;
    }
    1
}

/// Comparer for `fog_of_war_map_load`.
///
/// It reads the open save item: it cannot run twice. Only the original runs; the copy is proven by loading a save made
/// with attribute masking on.
pub(crate) unsafe extern "thiscall" fn compare_fog_of_war_map_load(this: usize, file: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_FOG_OF_WAR_MAP_LOAD)(this, file)
}

/// Frees every map of the fog of war: for each of the 16 slots that holds a map, its knowledge grid (+0, `free`, the
/// field zeroed) and the map itself (`operator delete`). The slots keep pointing at the freed maps. The body of the
/// destructor, also run inline by `FOG_OF_WAR::load` after a failure.
pub(crate) unsafe fn fog_free_maps(this: usize) {
    for k in 0..FOG_MAP_SLOTS {
        let map = read_usize(this + 4 * k);
        if map == 0 {
            continue;
        }
        let grid = read_usize(map);
        if grid != 0 {
            exe::crt_free(grid);
            write_usize(map, 0);
        }
        exe::crt_operator_delete(map);
    }
}

// ---------- sx10_2: the fog of war in the save: writing it and freeing it (fog_of_war.cpp) ----------

/// FOG_OF_WAR_MAP::save (thiscall(map, file) ret 4; its result is not used).
pub(crate) const FN_FOG_OF_WAR_MAP_SAVE: usize = 0x5b0170;

/// Rule: the fog of war goes into the save as the attribute masking flag and, when masking is on, a presence word and
/// the map of each of the 16 slots.
///
/// `FOG_OF_WAR::save(save)`: no save: the fog_of_war.cpp 0xf5 box. The "fog.dat" item of the save (`zopen` with 1,
/// 0x16, 4; none: the 0xfd box). The masking flag (`G_ATTRIBUTE_MASKING_ENABLED`, 1 byte; short: the 0x103 box).
/// Masking off: the item closed. Masking on: for each slot, a presence word 1 and the map (`FOG_OF_WAR_MAP::save`) when
/// it holds one, otherwise a word 0 (a short write: the 0x116 or 0x11f box); then the item is closed.
///
/// Defect of the original: after a failed write (0x103, 0x116, 0x11f) the item is left open.
/// Original: `FOG_OF_WAR::save` (0x5afad0, fog_of_war.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn fog_of_war_save(this: usize, save: usize) {
    let (present, absent): (u32, u32) = (1, 0);
    if save == 0 {
        show_error_at(FOG_OF_WAR_CPP, 0xf5);
        return;
    }
    let mut z = exe::zopen(save, FOG_DAT, 1, 0x16, 4);
    if z == 0 {
        show_error_at(FOG_OF_WAR_CPP, 0xfd);
        return;
    }
    if exe::zwrite(G_ATTRIBUTE_MASKING_ENABLED, 1, 1, z) != 1 { // sx10_2: the masking flag
        show_error_at(FOG_OF_WAR_CPP, 0x103);
        return;
    }
    if read_u8(G_ATTRIBUTE_MASKING_ENABLED) == 0 { // sx10_2: masking off, nothing more
        exe::zclose(&mut z);
        return;
    }
    covered!("sx10_2 maps written");
    let map_save: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_FOG_OF_WAR_MAP_SAVE);
    for k in 0..FOG_MAP_SLOTS {
        let map = read_usize(this + 4 * k);
        if map != 0 {
            if exe::zwrite(&present as *const u32 as usize, 4, 1, z) != 1 {
                show_error_at(FOG_OF_WAR_CPP, 0x116);
                return;
            }
            map_save(map, z);
        } else if exe::zwrite(&absent as *const u32 as usize, 4, 1, z) != 1 {
            show_error_at(FOG_OF_WAR_CPP, 0x11f);
            return;
        }
    }
    exe::zclose(&mut z);
}

/// Comparer for `fog_of_war_save`.
///
/// It writes the save item: it cannot run twice. Only the original runs; the copy is proven by the save proof (the
/// fog.dat item of the .qst written with and without the routines).
pub(crate) unsafe extern "thiscall" fn compare_fog_of_war_save(this: usize, save: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_FOG_OF_WAR_SAVE)(this, save)
}

/// Rule: a map goes into fog.dat as its manager's id and the knowledge of every person.
///
/// `FOG_OF_WAR_MAP::save(file)`: the id of the map's manager (+4; 4 bytes; short: the fog_of_war.cpp 0x254 box), then
/// the knowledge grid (+0): `G_STAFF_COUNT` entries of 9 bytes (short: the 0x25b box). The caller does not look at the
/// result.
/// Original: `FOG_OF_WAR_MAP::save` (0x5b0170, fog_of_war.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn fog_of_war_map_save(this: usize, file: usize) {
    let owner: i32 = read_i32(read_usize(this + 4)); // +4: the map's manager
    if exe::zwrite(&owner as *const i32 as usize, 4, 1, file) != 1 {
        show_error_at(FOG_OF_WAR_CPP, 0x254);
        return;
    }
    covered!("sx10_2 map written");
    let count = read_u32(G_STAFF_COUNT);
    if exe::zwrite(read_usize(this), 9, count, file) != read_u32(G_STAFF_COUNT) { // +0: the knowledge grid
        show_error_at(FOG_OF_WAR_CPP, 0x25b);
    }
}

/// Comparer for `fog_of_war_map_save`.
///
/// It writes the open save item: it cannot run twice. Only the original runs; the copy is proven by saving a game with
/// attribute masking on.
pub(crate) unsafe extern "thiscall" fn compare_fog_of_war_map_save(this: usize, file: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_FOG_OF_WAR_MAP_SAVE)(this, file)
}

/// Rule: destroying the fog of war frees every map and its knowledge grid.
///
/// `FOG_OF_WAR::~FOG_OF_WAR()`: for each of the 16 slots that holds a map, its knowledge grid (+0) is freed and zeroed
/// and the map deleted (`fog_free_maps`). The slots are not cleared; the caller (free_game_data 0x82ffd4) deletes the
/// object right after.
/// Original: `FOG_OF_WAR::~FOG_OF_WAR` (0x5af6a0, fog_of_war.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fog_of_war_destructor(this: usize) {
    fog_free_maps(this);
}

/// Comparer for `fog_of_war_destructor`.
///
/// It frees the maps: it cannot run twice. Only the original runs; the copy runs when a game is freed (leaving it,
/// loading another save).
pub(crate) unsafe extern "thiscall" fn compare_fog_of_war_destructor(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_FOG_OF_WAR_DESTRUCTOR)(this)
}

// --- sx01d_3: Media::Media (0x72b1d0), Media::~Media (0x72b200) and Media::load_media_dat (0x72b240), media.cpp ---

/// The source path of media.cpp in the "Error" box ("E:\dev\CM3\cm3 00-01\cm3\code\media.cpp").
const MEDIA_CPP_PATH: usize = 0xa17bec;
/// The save item "media.dat".
const MEDIA_DAT: usize = 0xa17c14;
/// `compare_id_pair_keys` (0x757ea0, cdecl), the order of the transfer interest table.
const MEDIA_INTEREST_ORDER: usize = 0x757ea0;
/// `compare_story_log_records` (0x757ee0, cdecl), the order of the story log.
const MEDIA_STORY_LOG_ORDER: usize = 0x757ee0;
/// The game data version from which the story log records are 23 bytes (before it, 22).
const MEDIA_LOG_23_BYTES_VERSION: i32 = 0x3e037;

/// Builds the media object: the story log and the transfer interest table empty (pointers, capacities and counts at
/// 0), then, with a save, `load_media_dat` reads them from it (its answer is not used). Returns the object.
/// Original: `Media::Media` (0x72b1d0, media.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn new_media(this: usize, save: usize) -> usize {
    for field in [media_fields::TRANSFER_INTERESTS, media_fields::INTEREST_CAPACITY, media_fields::INTEREST_COUNT,
                  media_fields::STORY_LOG, media_fields::STORY_LOG_CAPACITY, media_fields::STORY_LOG_COUNT] {
        write_u32(this + field, 0);
    }
    if save != 0 {
        covered!("sx01d_3 media from the save");
        load_media_dat(this, save);
    }
    this
}

/// Comparer for `new_media`: with a save it reads the save item and allocates, so it cannot run twice; only the
/// original runs. The copy is proven by the new career (Rust mode) and by loading the holiday save (levas/N/CARGA).
pub(crate) unsafe extern "thiscall" fn compare_new_media(this: usize, save: usize) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_NEW_MEDIA)(this, save)
}

/// Frees the media object's tables: the transfer interest table and the story log (`free`), each present one freed and
/// its pointer zeroed (the capacities and counts stay).
/// Original: `Media::~Media` (0x72b200, media.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn drop_media(this: usize) {
    for field in [media_fields::TRANSFER_INTERESTS, media_fields::STORY_LOG] {
        let table = read_usize(this + field);
        if table != 0 {
            exe::crt_free(table);
            write_usize(this + field, 0);
        }
    }
}

/// Comparer for `drop_media`: it frees, so it cannot run twice; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_drop_media(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_DROP_MEDIA)(this)
}

/// Converts the old 22-byte story log records (game data before `MEDIA_LOG_23_BYTES_VERSION`) into the 23-byte
/// `story_log_entry`: the date (`Date::operator=`), +8 and +0xc as they are, the byte +0x10, the story code +0x11 from
/// a byte to a word (sign-extended) and the dword +0x12 to +0x13.
unsafe fn media_convert_old_story_log(this: usize, old: usize) {
    let log = read_usize(this + media_fields::STORY_LOG);
    for k in 0..read_i32(this + media_fields::STORY_LOG_CAPACITY).max(0) as usize {
        let (o, n) = (old + k * 0x16, log + k * 0x17);
        write_u32(n + 0xc, read_u32(o + 0xc));
        let mut date = [0u8; 8];
        exe::date_operator_3d(n, date.as_mut_ptr() as usize, o);
        write_i16(n + 0x11, read_i8(o + 0x11) as i16);
        write_u32(n + 0x13, read_u32(o + 0x12));
        write_u8(n + 0x10, read_u8(o + 0x10));
        write_u32(n + 8, read_u32(o + 8));
    }
}

/// Reads the media object from the save's "media.dat": nothing (0) without a save or the item. The transfer interest
/// count (`media_fields::INTEREST_COUNT`, also the capacity), the 25-byte transfer interest records (`malloc`) sorted
/// with `compare_id_pair_keys`, the story log count (`STORY_LOG_COUNT`, also the capacity) and its records: 23 bytes,
/// or 22 bytes converted (`media_convert_old_story_log`) when `G_GAME_DATA_VERSION` (in absolute value) is below
/// 0x3e037; sorted with `compare_story_log_records`. The item closed, 1. A short read or a failed allocation (with a
/// count) shows the media.cpp "Error" box (lines 0xa5, 0xb1, 0xb9, 0xc4, 0xcf, 0xd8, 0xdf, 0xf5) and answers 0 with the
/// item left open (the original's leak).
/// Original: `Media::load_media_dat` (0x72b240, media.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn load_media_dat(this: usize, save: usize) -> u8 {
    if save == 0 {
        return 0;
    }
    let mut z = exe::zopen(save, MEDIA_DAT, 0, 0x16, 4);
    if z == 0 {
        return 0;
    }
    covered!("sx01d_3 load media.dat");
    let fail = |row: u32| {
        show_error_box(MEDIA_CPP_PATH, row);
        0u8
    };
    let count = this + media_fields::INTEREST_COUNT;
    if exe::zread(count, 4, 1, z) != 1 {
        return fail(0xa5);
    }
    write_u32(this + media_fields::INTEREST_CAPACITY, read_u32(count));
    let interests = exe::crt_malloc(read_u32(count).wrapping_mul(0x19));
    write_usize(this + media_fields::TRANSFER_INTERESTS, interests);
    if interests == 0 && read_u32(this + media_fields::INTEREST_CAPACITY) != 0 {
        return fail(0xb1);
    }
    let n = read_u32(count);
    if exe::zread(interests, 0x19, n, z) != n {
        return fail(0xb9);
    }
    exe::crt_qsort(interests, n, 0x19, MEDIA_INTEREST_ORDER);
    let logged = this + media_fields::STORY_LOG_COUNT;
    if exe::zread(logged, 4, 1, z) != 1 {
        return fail(0xc4);
    }
    write_u32(this + media_fields::STORY_LOG_CAPACITY, read_u32(logged));
    let log = exe::crt_malloc(read_u32(logged).wrapping_mul(0x17));
    write_usize(this + media_fields::STORY_LOG, log);
    if log == 0 && read_u32(this + media_fields::STORY_LOG_CAPACITY) != 0 {
        return fail(0xcf);
    }
    if read_i32(G_GAME_DATA_VERSION).wrapping_abs() < MEDIA_LOG_23_BYTES_VERSION {
        covered!("sx01d_3 old story log");
        let old = exe::crt_malloc(read_u32(this + media_fields::STORY_LOG_CAPACITY).wrapping_mul(0x16));
        if old == 0 && read_u32(this + media_fields::STORY_LOG_CAPACITY) != 0 {
            return fail(0xd8);
        }
        if exe::zread(old, 0x16, read_u32(logged), z) != read_u32(logged) {
            return fail(0xdf);
        }
        media_convert_old_story_log(this, old);
        exe::crt_free(old);
    } else if exe::zread(log, 0x17, read_u32(logged), z) != read_u32(logged) {
        return fail(0xf5);
    }
    exe::crt_qsort(read_usize(this + media_fields::STORY_LOG), read_u32(logged), 0x17, MEDIA_STORY_LOG_ORDER);
    exe::zclose(&mut z);
    1
}

/// Comparer for `load_media_dat`: it reads the save item and allocates, so it cannot run twice; only the original
/// runs. The copy is proven by loading the holiday save in the save proof (levas/N/CARGA).
pub(crate) unsafe extern "thiscall" fn compare_load_media_dat(this: usize, save: usize) -> u8 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_LOAD_MEDIA_DAT)(this, save)
}

// --- sx01d_4: Media::save_media_dat (0x72b5f0, media.cpp) ---

/// Writes the media object into the save's "media.dat": the transfer interest count (`media_fields::INTEREST_COUNT`),
/// its 25-byte records, the story log count (`STORY_LOG_COUNT`) and its 23-byte records; the item closed, 1. Without a
/// save (line 0x112), without the item (0x11a) or on a short write (0x121, 0x129, 0x130, 0x138) the media.cpp "Error"
/// box and 0, with the item left open (the original's leak).
/// Original: `Media::save_media_dat` (0x72b5f0, media.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn save_media_dat(this: usize, save: usize) -> u8 {
    const MEDIA_CPP_PATH: usize = 0xa17bec; // "E:\dev\CM3\cm3 00-01\cm3\code\media.cpp"
    let fail = |row: u32| {
        show_error_box(MEDIA_CPP_PATH, row);
        0u8
    };
    if save == 0 {
        return fail(0x112);
    }
    let mut z = exe::zopen(save, 0xa17c14usize, 1, 0x16, 4); // "media.dat"
    if z == 0 {
        return fail(0x11a);
    }
    covered!("sx01d_4 save media.dat");
    let interest_count = this + media_fields::INTEREST_COUNT;
    if exe::zwrite(interest_count, 4, 1, z) != 1 {
        return fail(0x121);
    }
    if exe::zwrite(read_usize(this + media_fields::TRANSFER_INTERESTS), 0x19, read_u32(interest_count), z) != read_u32(interest_count) {
        return fail(0x129);
    }
    let log_count = this + media_fields::STORY_LOG_COUNT;
    if exe::zwrite(log_count, 4, 1, z) != 1 {
        return fail(0x130);
    }
    if exe::zwrite(read_usize(this + media_fields::STORY_LOG), 0x17, read_u32(log_count), z) != read_u32(log_count) {
        return fail(0x138);
    }
    exe::zclose(&mut z);
    1
}

/// Comparer for `save_media_dat`: it writes the save item, so it cannot run twice; only the original runs. The copy is
/// proven by the save proof (the .qst written by the original and by the copy, item by item).
pub(crate) unsafe extern "thiscall" fn compare_save_media_dat(this: usize, save: usize) -> u8 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_SAVE_MEDIA_DAT)(this, save)
}

// ---------- batch 334: saving the injuries, the human managers and the key nation competitions (injury.cpp, human_manager.cpp, key_nation.cpp; sx12_5) ----------

/// Writes the injury state into the `injury.dat` item of a saved game.
///
/// `Injury::save(save)`: no container: the "Error" box (injury.cpp line 0x227). Opens `injury.dat` for writing
/// (`exe::zopen`; failing: the box, line 0x22f). Then, each write checked against its count, in order: the 31-byte
/// physical records (`injury_fields::PHYSICAL_RECORDS`, `G_STAFF_COUNT` of them; line 0x236), the injury slot count
/// (`injury_fields::INJURY_ENTRY_COUNT`; 0x23e) and the 0x30-byte injuries (`injury_fields::INJURY_ENTRIES`; 0x246),
/// the recovery news count (`injury_fields::RECOVERY_NEWS_COUNT`; 0x24e) and, when not zero, its 22-byte entries
/// (`injury_fields::RECOVERY_NEWS_QUEUE`; 0x259), the outbreak count (`injury_fields::OUTBREAK_COUNT`; 0x262) and,
/// when not zero, the 17-byte outbreaks (`injury_fields::OUTBREAKS`; 0x26e), the word count of the match knock list
/// (`injury_fields::MATCH_HIT_LIST`, an LLIST; 0 without a list; 0x27d) and, when not zero, each of its 6-byte
/// items from the first (`exe::llist_get_next`; 0x28f). A failed write shows the box with its line and closes the item
/// (`exe::zclose`); otherwise the item is closed at the end. Defect of the original: a failed write of a knock item
/// (line 0x28f) leaves the item open.
/// Original: `Injury::save` (0x62b2d0, injury.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn injury_save(this: usize, save: usize) {
    const FILE: usize = 0x9e7f28; // "...\injury.cpp"
    if save == 0 {
        show_error_at(FILE, 0x227);
        return;
    }
    let mut z = exe::zopen(save, 0x9e7f68usize, 1, 0x16, 4); // "injury.dat"
    if z == 0 {
        show_error_at(FILE, 0x22f);
        return;
    }
    covered!("sx12_5 injury save");
    let failed = |z: &mut usize, row: u32| {
        show_error_at(FILE, row);
        exe::zclose(z);
    };
    let staff = read_u32(G_STAFF_COUNT);
    if exe::zwrite(read_usize(this + injury_fields::PHYSICAL_RECORDS), 0x1f, staff, z) != read_u32(G_STAFF_COUNT) { // sx12_5: the physical records
        return failed(&mut z, 0x236);
    }
    if exe::zwrite(this + injury_fields::INJURY_ENTRY_COUNT, 4, 1, z) != 1 {
        return failed(&mut z, 0x23e);
    }
    let slots = read_u32(this + injury_fields::INJURY_ENTRY_COUNT);
    if exe::zwrite(read_usize(this + injury_fields::INJURY_ENTRIES), 0x30, slots, z) != read_u32(this + injury_fields::INJURY_ENTRY_COUNT) {
        return failed(&mut z, 0x246);
    }
    if exe::zwrite(this + injury_fields::RECOVERY_NEWS_COUNT, 4, 1, z) != 1 {
        return failed(&mut z, 0x24e);
    }
    let news = read_u32(this + injury_fields::RECOVERY_NEWS_COUNT);
    if news != 0 && exe::zwrite(read_usize(this + injury_fields::RECOVERY_NEWS_QUEUE), 0x16, news, z) != read_u32(this + injury_fields::RECOVERY_NEWS_COUNT) {
        return failed(&mut z, 0x259);
    }
    if exe::zwrite(this + injury_fields::OUTBREAK_COUNT, 4, 1, z) != 1 {
        return failed(&mut z, 0x262);
    }
    let outbreaks = read_u32(this + injury_fields::OUTBREAK_COUNT);
    if outbreaks != 0 && exe::zwrite(read_usize(this + injury_fields::OUTBREAKS), 0x11, outbreaks, z) != read_u32(this + injury_fields::OUTBREAK_COUNT) {
        return failed(&mut z, 0x26e); // sx12_5: the outbreaks
    }
    let list = read_usize(this + injury_fields::MATCH_HIT_LIST);
    let count: u16 = if list != 0 { read_u16(list + llist::COUNT) } else { 0 };
    if exe::zwrite(&count as *const u16 as usize, 2, 1, z) != 1 {
        return failed(&mut z, 0x27d);
    }
    if count != 0 {
        awol_llist_rewind(read_usize(this + injury_fields::MATCH_HIT_LIST));
        let mut item = exe::llist_get_next(read_usize(this + injury_fields::MATCH_HIT_LIST));
        while item != 0 {
            if exe::zwrite(item, 6, 1, z) != 1 {
                show_error_at(FILE, 0x28f); // the original leaves the item open here
                return;
            }
            item = exe::llist_get_next(read_usize(this + injury_fields::MATCH_HIT_LIST));
        }
    }
    exe::zclose(&mut z);
}

/// Writes every human manager record and the counts into the `human_manager.dat` item of a saved game; returns 1, or 0
/// after an error.
///
/// `HUMAN_MANAGER::save(save)`: no container: the "Error" box (human_manager.cpp line 0x916) and 0. Opens
/// `human_manager.dat` for writing (`exe::zopen`; failing: the box, line 0x91e, and 0). Writes the record count (+4;
/// failing: line 0x925, the item closed, 0). For each record (0xa6ef bytes from [this]), unchecked: the first two
/// tactic slots (2 × 0x1ecd), `human_manager_record::ON_HOLIDAY` (4), the dword at +0x9a05 (the two option bytes
/// `CONTROLS_RESERVE_TEAM`, `CONTROLS_ALL_NATIONAL_TEAMS` and two more), the draft contract (+0x9a09, 0x50), the two
/// offer pointers (+0x9a59, +0x9a5d), the dword +0x9a61, `HOLIDAY_STAMP`, `ACTIVE_TACTIC_SLOT` (1), the password
/// (+0x9a6a, 0x32) and its typed entry (+0x9a9c, 0x32), `COMPUTER_CONTROLLED`, the commentary speed (+0x9ad2, 2), the
/// watched fixture (`WATCHED_FIXTURE`, 0x4f), three folder paths, the first two with their four recent files
/// (+0x9ba8, 0x104; +0x9cac, 0x410; +0xa0bc, 0x104; +0xa1c0, 0x410; +0xa5d0, 0x104), the squad view (+0xa6d4, 0xf), the
/// five `HOLIDAY_OPTIONS`, the 16 `SAVED_LINEUP` pairs (8 bytes each), `NICKNAME_COUNT` and the 30-byte `NICKNAMES`.
/// Then the dwords +8 (line 0x94d) and +0xc (line 0x955), each failing with the item closed and 0; at the end, for each
/// record, the last three tactic slots (3 × 0x1ecd from slot 2, +0x3d9a), the item closed, 1.
/// Original: `HUMAN_MANAGER::save` (0x5ff9e0, human_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn human_manager_save(this: usize, save: usize) -> u32 {
    const FILE: usize = 0x9cf11c; // "...\human_manager.cpp"
    if save == 0 {
        show_error_at(FILE, 0x916);
        return 0;
    }
    let mut z = exe::zopen(save, 0x9cf164usize, 1, 0x16, 4); // "human_manager.dat"
    if z == 0 {
        show_error_at(FILE, 0x91e);
        return 0;
    }
    let failed = |z: &mut usize, row: u32| {
        show_error_at(FILE, row);
        exe::zclose(z);
        0
    };
    if exe::zwrite(this + 4, 4, 1, z) != 1 {
        return failed(&mut z, 0x925);
    }
    covered!("sx12_5 human manager save");
    // (offset in the record, item size, item count), in the original's order
    const FIELDS: [(usize, u32, u32); 23] = [
        (0, 0x1ecd, 2), // the first two tactic slots
        (human_manager_record::ON_HOLIDAY, 4, 1),
        (human_manager_record::CONTROLS_RESERVE_TEAM, 4, 1), // the option bytes
        (0x9a09, 0x50, 1), // the draft contract
        (0x9a59, 4, 1), // the bid offer
        (0x9a5d, 4, 1), // the offer to confirm
        (0x9a61, 4, 1),
        (human_manager_record::HOLIDAY_STAMP, 4, 1),
        (human_manager_record::ACTIVE_TACTIC_SLOT, 1, 1),
        (0x9a6a, 1, 0x32), // the password
        (0x9a9c, 1, 0x32), // the typed password
        (human_manager_record::COMPUTER_CONTROLLED, 4, 1),
        (0x9ad2, 2, 1), // the commentary speed
        (human_manager_record::WATCHED_FIXTURE, 0x4f, 1),
        (0x9ba8, 1, 0x104), // a folder path
        (0x9cac, 1, 0x410), // its four recent files
        (0xa0bc, 1, 0x104), // a folder path
        (0xa1c0, 1, 0x410), // its four recent files
        (0xa5d0, 1, 0x104), // a folder path
        (0xa6d4, 0xf, 1), // the squad view
        (human_manager_record::HOLIDAY_OPTIONS, 1, 5),
        (human_manager_record::SAVED_LINEUP, 8, 0x10),
        (human_manager_record::NICKNAME_COUNT, 4, 1),
    ];
    for k in 0..read_i32(this + 4).max(0) as usize {
        let record = human_manager_record_at(this, k);
        for &(at, size, n) in FIELDS.iter() {
            exe::zwrite(record + at, size, n, z); // sx12_5: the record's fields
        }
        exe::zwrite(read_usize(record + human_manager_record::NICKNAMES), 0x1e, read_u32(record + human_manager_record::NICKNAME_COUNT), z);
    }
    if exe::zwrite(this + 8, 4, 1, z) != 1 {
        return failed(&mut z, 0x94d);
    }
    if exe::zwrite(this + 0xc, 4, 1, z) != 1 {
        return failed(&mut z, 0x955);
    }
    for k in 0..read_i32(this + 4).max(0) as usize {
        exe::zwrite(human_manager_record_at(this, k) + 2 * 0x1ecd, 0x1ecd, 3, z); // the last three tactic slots
    }
    exe::zclose(&mut z);
    1
}

/// Tells every competition of each key nation row to save itself to its temporary file; returns 1.
///
/// `key_nation_save_all_comps()`: for each of the 35 rows of `G_LEAGUE_TABLE` with a competition array (+0x10, count at
/// +0xc), each competition that is not null gets its virtual `comp_vtable::SAVE_TO_TEMP_FILE` with 1. Called by the
/// game save (step 0x19 of 0x82e430).
/// Original: `key_nation_save_all_comps` (0x6692f0, key_nation.cpp, cdecl).
pub(crate) unsafe extern "C" fn key_nation_save_all_comps() -> u32 {
    for k in 0..0x23usize {
        let row = G_LEAGUE_TABLE + 72 * k;
        if read_usize(row + 0x10) == 0 {
            continue; // no competition array
        }
        let mut i = 0;
        while i < read_i32(row + 0xc) {
            let comp = read_usize(read_usize(row + 0x10) + 4 * i as usize);
            if comp != 0 {
                covered!("sx12_5 save competition");
                let f: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(read_usize(read_usize(comp) + comp_vtable::SAVE_TO_TEMP_FILE));
                f(comp, 1); // sx12_5: the competition's temporary file
            }
            i += 1;
        }
    }
    1
}

/// Comparer for `injury_save`: writes into the save container (cannot run twice); only the original runs, and the
/// copy is proven by the save proof (the `.qst` written with and without the batch, item by item).
pub(crate) unsafe extern "thiscall" fn compare_injury_save(this: usize, save: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_INJURY_SAVE)(this, save)
}

/// Comparer for `human_manager_save`: only the original runs (the save container); proven by the save proof.
pub(crate) unsafe extern "thiscall" fn compare_human_manager_save(this: usize, save: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_HUMAN_MANAGER_SAVE)(this, save)
}

/// Comparer for `key_nation_save_all_comps`: only the original runs (each competition writes its temporary file);
/// proven by the save proof.
pub(crate) unsafe extern "C" fn compare_key_nation_save_all_comps() -> u32 {
    original_fn::<unsafe extern "C" fn() -> u32>(IDX_KEY_NATION_SAVE_ALL_COMPS)()
}

// ---------- sx05b_8: building the AWOL state and loading it from a saved game (awol.cpp) ----------

/// The AWOL object's three record lists (`LLIST`, 0xe bytes each) and the size of their records: the absences (+0,
/// 12 bytes: staff id and the return date), the missed trainings (+4, 13 bytes) and the fitness incidents (+8, 17
/// bytes); the per-staff flag table is `awol_fields::PERSON_FLAGS`.
pub(crate) const AWOL_LISTS: [(usize, u32); 3] = [(0x0, 0xc), (0x4, 0xd), (0x8, 0x11)];
/// "E:\dev\CM3\cm3 00-01\cm3\code\awol.cpp" (the error boxes) and "awol.dat" (its file in the save).
pub(crate) const AWOL_CPP: usize = 0x9880f4;
pub(crate) const AWOL_DAT: usize = 0x9880e8;
/// `AWOL::load(save)` (0x418670, thiscall, ret 4: al nonzero when read), through its hook.
const FN_AWOL_LOAD: usize = 0x418670;

/// A new empty record list (`exe::new_llist` in 0xe bytes of `exe::crt_operator_new`), 0 when out of memory.
pub(crate) unsafe fn awol_new_list() -> usize {
    let p = exe::crt_operator_new(0xe);
    if p != 0 { exe::new_llist(p) } else { 0 }
}

/// Builds the AWOL state: everything 0; with a save, `AWOL::load`, and nothing more when it reads. Without a save, or
/// when the load fails (then its flag table and first list are freed first), a zeroed flag byte per staff member
/// (`exe::crt_calloc`(`G_STAFF_COUNT`, 1)) and three new empty lists. Returns this.
/// Defect of the original: a failed load frees only the first list; the second and third (when it built them) leak.
/// Original: `AWOL::AWOL` (0x418440, awol.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn awol_construct(this: usize, save: usize) -> usize {
    write_usize(this + awol_fields::PERSON_FLAGS, 0);
    for (off, _) in AWOL_LISTS {
        write_usize(this + off, 0);
    }
    if save != 0 {
        let load: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_AWOL_LOAD);
        if load(this, save) != 0 {
            return this;
        }
        covered!("sx05b_8 awol load failed");
        let flags = read_usize(this + awol_fields::PERSON_FLAGS);
        if flags != 0 {
            exe::crt_free(flags);
            write_usize(this + awol_fields::PERSON_FLAGS, 0);
        }
        let first = read_usize(this);
        if first != 0 {
            exe::drop_llist(first);
            exe::crt_operator_delete(first);
            write_usize(this, 0);
        }
    }
    write_usize(this + awol_fields::PERSON_FLAGS, exe::crt_calloc(read_u32(G_STAFF_COUNT), 1));
    for (off, _) in AWOL_LISTS {
        write_usize(this + off, awol_new_list());
    }
    this
}

/// Reads one count or record from awol.dat; false on a short read.
unsafe fn awol_read(dest: usize, size: u32, z: usize) -> bool {
    exe::zread(dest, size, 1, z) == 1
}

/// Loads the AWOL state from a save's awol.dat (`exe::zopen`(save, "awol.dat", 0, 0x16, 4); none, 0): the staff count
/// (zero counts as a failure), the flag table (`exe::crt_malloc` of that many bytes, read in one go), the absences
/// (count, then each 12-byte record appended to a new list), the missed trainings (13 bytes) and, only in saves of a
/// version above 0x3e005 (`G_GAME_DATA_VERSION`, by its absolute value), the fitness incidents (17 bytes; older saves
/// get an empty third list). Any short read closes the file and gives 0; otherwise the file is closed and 1.
/// Original: `AWOL::load` (0x418670, awol.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn awol_load(this: usize, save: usize) -> u8 {
    if save == 0 {
        return 0;
    }
    let mut z = exe::zopen(save, AWOL_DAT, 0, 0x16, 4);
    if z == 0 {
        return 0;
    }
    let ok = awol_load_lists(this, z);
    exe::zclose(&mut z);
    ok as u8
}

/// The body of `awol_load` once the file is open.
unsafe fn awol_load_lists(this: usize, z: usize) -> bool {
    let mut n = 0u32;
    let np = &mut n as *mut u32 as usize;
    if !awol_read(np, 4, z) || n == 0 {
        return false;
    }
    let flags = exe::crt_malloc(n);
    write_usize(this + awol_fields::PERSON_FLAGS, flags);
    if flags == 0 || exe::zread(flags, 1, n, z) != n || !awol_read(np, 4, z) {
        return false;
    }
    let version = read_i32(G_GAME_DATA_VERSION).unsigned_abs();
    for (k, &(off, size)) in AWOL_LISTS.iter().enumerate() {
        if k == 2 {
            write_usize(this + off, awol_new_list());
            if version <= 0x3e005 {
                return true;
            }
            covered!("sx05b_8 fitness incidents read");
            if !awol_read(np, 4, z) {
                return false;
            }
        } else {
            write_usize(this + off, awol_new_list());
        }
        let mut record = [0u8; 0x11];
        for _ in 0..(n as i32).max(0) {
            if !awol_read(record.as_mut_ptr() as usize, size, z) {
                return false;
            }
            exe::llist_add_tail(read_usize(this + off), record.as_ptr() as usize, size);
        }
        if k == 0 && !awol_read(np, 4, z) {
            return false;
        }
    }
    true
}

/// Comparer for `awol_construct`: it allocates (and may read the save): only the original runs; then the flag table
/// and the three lists must exist.
pub(crate) unsafe extern "thiscall" fn compare_awol_construct(this: usize, save: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_AWOL_CONSTRUCT)(this, save);
    let ok = r == this && read_usize(this + awol_fields::PERSON_FLAGS) != 0 && AWOL_LISTS.iter().all(|&(off, _)| read_usize(this + off) != 0);
    SCOREBOARD.record(ok, || format!("awol_construct(save {:#x}): flags {:#x} lists {:x?}", save, read_usize(this + awol_fields::PERSON_FLAGS),
        AWOL_LISTS.iter().map(|&(off, _)| read_usize(this + off)).collect::<Vec<_>>()));
    r
}

/// Comparer for `awol_load`: the save is read once; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_awol_load(this: usize, save: usize) -> u8 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_AWOL_LOAD)(this, save)
}

// ---------- sx05b_9: saving the AWOL state (awol.cpp) ----------

/// Saves the AWOL state to the save's awol.dat as `awol_load` reads it: without a save, the "Error" box (line 0x132)
/// and 0; the file (`exe::zopen`(save, "awol.dat", 1, 0x16, 4); none, the box of line 0x13a and 0, nothing to close);
/// the staff count (`G_STAFF_COUNT`, line 0x141), the flag table (that many bytes, line 0x148) and each of the three
/// lists: its count (the list's signed word, as a dword; lines 0x151, 0x16a, 0x183), then its records walked from the
/// head (`llist::CURSOR` = `llist::HEAD`, `exe::llist_get_next`: a missing record, lines 0x15a, 0x173, 0x18c; a short
/// write, 0x160, 0x179, 0x192) at 12, 13 and 17 bytes. Each failure after the file opens shows the box, closes it and
/// gives 0; otherwise it closes and gives 1.
/// Original: `AWOL::save` (0x418980, awol.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn awol_save(this: usize, save: usize) -> u8 {
    if save == 0 {
        show_error_at(AWOL_CPP, 0x132);
        return 0;
    }
    let mut z = exe::zopen(save, AWOL_DAT, 1, 0x16, 4);
    if z == 0 {
        show_error_at(AWOL_CPP, 0x13a);
        return 0;
    }
    let failed = awol_save_lists(this, z);
    if let Some(line) = failed {
        show_error_at(AWOL_CPP, line);
    }
    exe::zclose(&mut z);
    failed.is_none() as u8
}

/// The body of `awol_save` once the file is open: the line of the first failure, if any.
unsafe fn awol_save_lists(this: usize, z: usize) -> Option<u32> {
    if exe::zwrite(G_STAFF_COUNT, 4, 1, z) != 1 {
        return Some(0x141);
    }
    let staff = read_u32(G_STAFF_COUNT);
    if exe::zwrite(read_usize(this + awol_fields::PERSON_FLAGS), 1, staff, z) != staff {
        return Some(0x148);
    }
    const LINES: [(u32, u32, u32); 3] = [(0x151, 0x15a, 0x160), (0x16a, 0x173, 0x179), (0x183, 0x18c, 0x192)];
    for (&(off, size), &(count_line, missing_line, write_line)) in AWOL_LISTS.iter().zip(LINES.iter()) {
        let list = read_usize(this + off);
        let count = read_i16(list + llist::COUNT) as i32;
        if exe::zwrite(&count as *const i32 as usize, 4, 1, z) != 1 {
            return Some(count_line);
        }
        write_usize(list + llist::CURSOR, read_usize(list + llist::HEAD));
        let mut k = 0;
        while k < read_i16(read_usize(this + off) + llist::COUNT) as i32 {
            let record = exe::llist_get_next(read_usize(this + off));
            if record == 0 {
                return Some(missing_line);
            }
            if exe::zwrite(record, size, 1, z) != 1 {
                return Some(write_line);
            }
            k += 1;
        }
    }
    covered!("sx05b_9 awol saved");
    None
}

/// Comparer for `awol_save`: the save file is written once; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_awol_save(this: usize, save: usize) -> u8 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_AWOL_SAVE)(this, save)
}

// ---------- sx04b_6: the month award in the saved game and the month ratings' load (month_award.cpp, month_ratings.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\award\month_award.cpp" and "...\month_ratings.cpp" (the error boxes).
pub(crate) const MONTH_AWARD_CPP: usize = 0xa5abe4;
pub(crate) const MONTH_RATINGS_CPP: usize = 0xa5b234;
/// The boxes of a failed load: "Load Game Error" with "The save game is corrupted..." (`LOAD_ERROR`), or "Memory
/// Allocation Error" with its body (`MEMORY_ERROR`); title and body, both translated.
pub(crate) const LOAD_ERROR: (usize, usize) = (0x9af3e0, 0x9af334);
pub(crate) const MEMORY_ERROR: (usize, usize) = (0x9a8108, 0x9a7ff8);
/// The month ratings table (`MONTH_RATINGS`): the 12-byte entries (+0), their count (+4) and capacity (+8).
pub(crate) const MONTH_RATINGS_ENTRIES: usize = 0x0;
pub(crate) const MONTH_RATINGS_COUNT: usize = 0x4;
pub(crate) const MONTH_RATINGS_CAPACITY: usize = 0x8;
pub(crate) const MONTH_RATING_ENTRY_SIZE: u32 = 0xc;

/// The box of a failed load from a saved game: "v<version> <file>.<ext> <line> (<message>)" at 0xae24d0, the `box`'s
/// title (0xae28bc) and body with that text (0xae26c4), both translated, and the message box.
pub(crate) unsafe fn award_load_error_box(cpp: usize, line: u32, message: usize, (title, body): (usize, usize)) {
    let version: unsafe extern "C" fn(*mut u8, *mut u8, u32, usize) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let (mut name, mut ext) = ([0u8; 256], [0u8; 256]);
    exe::crt_strtok(cpp, 0usize, 0usize, name.as_mut_ptr(), ext.as_mut_ptr());
    let v = version(name.as_mut_ptr(), ext.as_mut_ptr(), line, message);
    comp_crt_format()(0xae24d0, 0x9a8120, v, name.as_ptr(), ext.as_ptr(), line, message); // "v%s %s.%s %d (%s)"
    comp_lang_format()(0xae28bc, title);
    comp_lang_format()(0xae26c4, body, 0xae24d0);
    exe::gui_show_message_box(0xae28bc, 0xae26c4, 0);
}

/// Reads a month award from the saved game: its twelve month slots (`MONTH_AWARD_SLOT_SIZE` bytes each, from
/// `month_award_fields::SLOT_WINNER_ID`) and its slot count (`MONTH_AWARD_SLOT_COUNT`, a byte), each with
/// `exe::zread`. Returns 1, or 0 after the Load Game Error box of the first short read (month_award.cpp 110 "cannot
/// read months", 117 "cannot read no of months").
/// Original: `MONTH_AWARD::load_from_disk` (0x766f50, month_award.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn month_award_load_from_disk(this: usize, file: usize) -> u32 {
    if exe::zread(this + month_award_fields::SLOT_WINNER_ID, MONTH_AWARD_SLOT_SIZE as u32, 12, file) != 12 {
        award_load_error_box(MONTH_AWARD_CPP, 0x6e, 0xa5abb0, LOAD_ERROR);
        return 0;
    }
    if exe::zread(this + MONTH_AWARD_SLOT_COUNT, 1, 1, file) != 1 { // sx04b_6: the slot count
        award_load_error_box(MONTH_AWARD_CPP, 0x75, 0xa5ab78, LOAD_ERROR);
        return 0;
    }
    covered!("sx04b_6 month award loaded");
    1
}

/// Comparer for `month_award_load_from_disk`: the load reads the file once; only the original runs, and a loaded
/// award must keep at most twelve slots.
pub(crate) unsafe extern "thiscall" fn compare_month_award_load_from_disk(this: usize, file: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_MONTH_AWARD_LOAD_FROM_DISK)(this, file);
    if r == 1 {
        let n = read_i8(this + MONTH_AWARD_SLOT_COUNT);
        SCOREBOARD.record((0..=12).contains(&n), || format!("month_award_load_from_disk({:#x}): {} slots", this, n));
    }
    r
}

/// Writes a month award to the saved game as `month_award_load_from_disk` reads it: the twelve month slots, then the
/// slot count (`exe::zwrite`); a short write shows the "Error" box (month_award.cpp 135, 142) and stops there.
/// Original: `MONTH_AWARD::save_to_disk` (0x767070, month_award.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn month_award_save_to_disk(this: usize, file: usize) {
    if exe::zwrite(this + month_award_fields::SLOT_WINNER_ID, MONTH_AWARD_SLOT_SIZE as u32, 12, file) != 12 { // sx04b_6: the twelve slots
        return tel26_error_box(MONTH_AWARD_CPP, 0x87);
    }
    if exe::zwrite(this + MONTH_AWARD_SLOT_COUNT, 1, 1, file) != 1 {
        return tel26_error_box(MONTH_AWARD_CPP, 0x8e);
    }
    covered!("sx04b_6 month award saved");
}

/// Comparer for `month_award_save_to_disk`: it writes the file once; only the original runs (the save proof compares
/// the written game).
pub(crate) unsafe extern "thiscall" fn compare_month_award_save_to_disk(this: usize, file: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_MONTH_AWARD_SAVE_TO_DISK)(this, file)
}

/// Reads the month ratings from the saved game: the entry count (4 bytes at `MONTH_RATINGS_COUNT`); with entries, the
/// capacity becomes the count, a block of count × 12 bytes is allocated (`exe::crt_malloc`) for the entries and they
/// are read; then the month (`month_ratings_fields::MONTH`, a byte) and the year (`month_ratings_fields::YEAR`, 2
/// bytes). Returns 1, or 0 after the box of the first failure: Load Game Error (month_ratings.cpp 86 "cannot read
/// number of players", 105 "cannot read players", 113 "cannot read current month", 120 "cannot read current year")
/// or Memory Allocation Error (98 "cannot malloc players array").
/// Original: `MONTH_RATINGS::load_from_disk` (0x767b20, month_ratings.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn month_ratings_load_from_disk(this: usize, file: usize) -> u32 {
    if exe::zread(this + MONTH_RATINGS_COUNT, 4, 1, file) != 1 {
        award_load_error_box(MONTH_RATINGS_CPP, 0x56, 0xa5b1f4, LOAD_ERROR);
        return 0;
    }
    let count = read_u32(this + MONTH_RATINGS_COUNT);
    if count != 0 {
        write_u32(this + MONTH_RATINGS_CAPACITY, count);
        let entries = exe::crt_malloc(count.wrapping_mul(MONTH_RATING_ENTRY_SIZE));
        write_usize(this + MONTH_RATINGS_ENTRIES, entries);
        if entries == 0 {
            award_load_error_box(MONTH_RATINGS_CPP, 0x62, 0xa5b1b4, MEMORY_ERROR);
            return 0;
        }
        if exe::zread(entries, MONTH_RATING_ENTRY_SIZE, read_u32(this + MONTH_RATINGS_COUNT), file) != read_u32(this + MONTH_RATINGS_COUNT) {
            award_load_error_box(MONTH_RATINGS_CPP, 0x69, 0xa5b17c, LOAD_ERROR);
            return 0;
        }
    }
    if exe::zread(this + month_ratings_fields::MONTH, 1, 1, file) != 1 { // sx04b_6: the ratings' month
        award_load_error_box(MONTH_RATINGS_CPP, 0x71, 0xa5b140, LOAD_ERROR);
        return 0;
    }
    if exe::zread(this + month_ratings_fields::YEAR, 2, 1, file) != 1 {
        award_load_error_box(MONTH_RATINGS_CPP, 0x78, 0xa5b104, LOAD_ERROR);
        return 0;
    }
    covered!("sx04b_6 month ratings loaded");
    1
}

/// Comparer for `month_ratings_load_from_disk`: the load reads the file once and allocates; only the original runs,
/// and loaded ratings must have their capacity at least their count.
pub(crate) unsafe extern "thiscall" fn compare_month_ratings_load_from_disk(this: usize, file: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_MONTH_RATINGS_LOAD_FROM_DISK)(this, file);
    if r == 1 {
        let (n, cap) = (read_u32(this + MONTH_RATINGS_COUNT), read_u32(this + MONTH_RATINGS_CAPACITY));
        SCOREBOARD.record(n == 0 || (cap == n && read_usize(this + MONTH_RATINGS_ENTRIES) != 0), || format!("month_ratings_load_from_disk({:#x}): count {} capacity {}", this, n, cap));
    }
    r
}

// ---------- batch 334: the national teams object built for a new game or from the saved game (national_teams.cpp) ----------

/// `FIFA_RANKINGS::FIFA_RANKINGS(save)` (thiscall, ret 4, returns this): the rankings, from the save when one is given.
const FN_FIFA_RANKINGS_CONSTRUCTOR: usize = 0x58cea0;
/// `NationalTeams::load_dat(save)` (thiscall, ret 4): this item's loader, called by the constructor through the exe.
const FN_NATIONALTEAMS_LOAD_DAT: usize = 0x7763b0;
/// `NationalTeams::init_tables()` (thiscall, ret 0): the tables of a new game (Rust since sma_4).
const FN_NATIONALTEAMS_INIT_TABLES: usize = 0x76a840;
/// "national_teams.dat", the national teams item of the saved game.
const NATIONAL_TEAMS_DAT: usize = 0xa63020;
/// The `jne` on the drive type of the constructor's CD check (nops in a no-CD exe such as cm0102-1080).
const NATIONAL_TEAMS_CD_DRIVE_TYPE_JUMP: usize = 0x76a543;
/// The `test eax, eax` on the found file of the constructor's CD check (`xor` in a no-CD exe).
const NATIONAL_TEAMS_CD_FOUND_TEST: usize = 0x76a586;

/// Builds the national teams object for a new game (`save` 0) or from the saved game.
///
/// `NationalTeams::NationalTeams(save)`: the new qualifiers list (`national_teams_fields::NEW_QUALIFIERS`, an inline
/// LLIST) is built; the CD is checked (`find_game_cd`: the first CD drive holding cm0102.exe, else the "Insert CD" box
/// and again); a new game loads the player setup (`player_setup_cfg_load(4, 0)`). The FIFA rankings
/// (`FIFA_RANKING`: `operator new(0xd)` and `FIFA_RANKINGS::FIFA_RANKINGS(save)`, 0 when the allocation fails), the
/// call-up list (`CALL_UP_LIST`) and the club marks (`CLUB_MARKS`) cleared, and the club call-up records
/// (`CLUB_CALL_UP_RECORDS`, `calloc(clubs without the national sides, 6)`; none: the national_teams.cpp box of line
/// 0x2163, and it goes on). With a save, `load_dat(save)`; without one, or when the load returns 0, `init_tables()`.
/// Returns this.
/// Original: `NationalTeams::NationalTeams` (0x76a4e0, national_teams.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn nationalteams_constructor(this: usize, save: usize) -> usize {
    use national_teams_fields as f;
    exe::new_llist(this + f::NEW_QUALIFIERS);
    find_game_cd(NATIONAL_TEAMS_CD_DRIVE_TYPE_JUMP, NATIONAL_TEAMS_CD_FOUND_TEST);
    if save == 0 {
        let player_setup: unsafe extern "C" fn(u32, u32) = core::mem::transmute(FN_PLAYER_SETUP_CFG_LOAD);
        player_setup(4, 0);
    }
    let rankings = exe::crt_operator_new(0xd);
    let rankings = if rankings != 0 {
        let construct: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FIFA_RANKINGS_CONSTRUCTOR);
        construct(rankings, save)
    } else {
        0
    };
    write_usize(this + f::FIFA_RANKING, rankings);
    write_usize(this + f::CALL_UP_LIST, 0);
    write_usize(this + f::CLUB_MARKS, 0);
    let records = exe::crt_calloc(club_call_up_record_count() as u32, 6);
    write_usize(this + f::CLUB_CALL_UP_RECORDS, records);
    if records == 0 {
        show_error_at(NATIONAL_TEAMS_CPP, 0x2163);
    }
    let load: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NATIONALTEAMS_LOAD_DAT);
    if save == 0 || load(this, save) == 0 { // sx15_8: a new game builds the tables
        let init: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_NATIONALTEAMS_INIT_TABLES);
        init(this);
    }
    this
}

/// Comparer for `nationalteams_constructor`: it allocates the object's tables and asks for the CD (it cannot run
/// twice), so only the original runs; the copy is proven by the save proof (a new game saved, and the holiday save
/// loaded).
pub(crate) unsafe extern "thiscall" fn compare_nationalteams_constructor(this: usize, save: usize) -> usize {
    // ponytail: builds the object once; the save proof checks the copy
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_NATIONALTEAMS_CONSTRUCTOR)(this, save)
}

/// One reserve pool read back from the saved game: its size (4 bytes into `this + count_field`), a new table of size
/// dwords (`malloc`, into `this + pool_field`) and the size person ids, each turned into the person (−1: none). False
/// after the national_teams.cpp box of the failing step (`lines`: the size, the allocation, the ids).
unsafe fn load_reserve_pool(this: usize, count_field: usize, pool_field: usize, z: usize, lines: [u32; 3]) -> bool {
    if exe::zread(this + count_field, 4, 1, z) != 1 {
        show_error_at(NATIONAL_TEAMS_CPP, lines[0]);
        return false;
    }
    let pool = exe::crt_malloc(read_u32(this + count_field) << 2);
    write_usize(this + pool_field, pool);
    if pool == 0 {
        show_error_at(NATIONAL_TEAMS_CPP, lines[1]);
        return false;
    }
    let count = read_i32(this + count_field);
    if exe::zread(pool, 4, count as u32, z) != count as u32 {
        show_error_at(NATIONAL_TEAMS_CPP, lines[2]);
        return false;
    }
    for k in 0..count {
        let slot = read_usize(this + pool_field) + k as usize * 4;
        let id = read_i32(slot);
        write_usize(slot, if id != -1 { staff_at(id as isize).addr() } else { 0 }); // sx15_8: id to person
    }
    true
}

/// Reads the national teams object back from the saved game.
///
/// `NationalTeams::load_dat(save)`: no save: 0, no box. The "national_teams.dat" item (`zopen` with 0, 0x16, 4); then
/// each step checked, a failure showing the national_teams.cpp box of its line and returning 0 with the item left
/// open:
/// - the nation records (`NATION_RECORDS`, `malloc(nations × 0x6e)`, none: line 0x1f83), the next competitions
///   (`NEXT_COMPETITIONS`, `malloc(nations × 4)`) and the Olympic marks (`OLYMPIC_NATION_MARKS`, `calloc(1, nations)`)
///   allocated, and the nations' records read (line 0x1f91);
/// - the senior reserve pool (size +0x14, table +0xc; lines 0x1f98, 0x1fa0, 0x1fa8) and the under-21 one (size +0x18,
///   table +0x10; lines 0x1fc0, 0x1fc8, 0x1fd0), person ids turned into persons (`load_reserve_pool`);
/// - the call-up count (2 bytes, line 0x1fe8); with a count, a new list (`operator new(0xe)` and `LLIST::LLIST`, 0 when
///   the allocation fails) into `CALL_UP_LIST` and that many 0x12-byte call-ups appended (line 0x1ffa);
/// - the club call-up records (`CLUB_CALL_UP_RECORDS`, 6 bytes per club without the national sides): when all were
///   read and there are clubs, each record with entries (+5) gets its capacity (+4) = the count and a new vector
///   (+0, `malloc(capacity × 9)`, none: line 0x2013) with that many 9-byte entries (line 0x201a); a short read of
///   the records skips them silently;
/// - the next competitions (4 bytes per nation; a short read fills them all with −1) and the Olympic marks (1 byte per
///   nation, unchecked).
/// The item closed; returns 1.
///
/// Defect of the original: a club record with no entries keeps the vector pointer it had when the game was saved (a
/// pointer of the old session); nothing reads it while the count is 0, but the destructor frees every non-zero
/// pointer. [inference: whether the game ever leaves a non-zero pointer with count 0]
/// Original: `NationalTeams::load_dat` (0x7763b0, national_teams.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn nationalteams_load_dat(this: usize, save: usize) -> u32 {
    use national_teams_fields as f;
    let clubs = club_call_up_record_count();
    if save == 0 {
        return 0;
    }
    let mut z = exe::zopen(save, NATIONAL_TEAMS_DAT, 0, 0x16, 4);
    let fail = |line: u32| {
        show_error_at(NATIONAL_TEAMS_CPP, line);
        0
    };
    if z == 0 {
        return fail(0x1f7a);
    }
    covered!("sx15_8 load national teams");
    let records = exe::crt_malloc(read_u32(G_NATION_COUNT).wrapping_mul(0x6e));
    write_usize(this + f::NATION_RECORDS, records);
    if records == 0 {
        return fail(0x1f83);
    }
    write_usize(this + f::NEXT_COMPETITIONS, exe::crt_malloc(read_u32(G_NATION_COUNT) << 2));
    write_usize(this + f::OLYMPIC_NATION_MARKS, exe::crt_calloc(1, read_u32(G_NATION_COUNT)));
    if exe::zread(read_usize(this + f::NATION_RECORDS), 0x6e, read_u32(G_NATION_COUNT), z) != read_u32(G_NATION_COUNT) {
        return fail(0x1f91);
    }
    // the two reserve pools: sizes at +0x14 and +0x18, tables at RESERVE_POOLS (+0xc) and +0x10
    if !load_reserve_pool(this, 0x14, f::RESERVE_POOLS, z, [0x1f98, 0x1fa0, 0x1fa8])
        || !load_reserve_pool(this, 0x18, f::RESERVE_POOLS + 4, z, [0x1fc0, 0x1fc8, 0x1fd0])
    {
        return 0;
    }
    let mut count = 0u16;
    if exe::zread(&mut count as *mut u16 as usize, 2, 1, z) != 1 {
        return fail(0x1fe8);
    }
    if count != 0 {
        let mut call_up = [0u8; 0x12];
        exe::date_date_54a110(call_up.as_mut_ptr() as usize + 8); // the call-up's date, read over right after
        let list = exe::crt_operator_new(0xe);
        write_usize(this + f::CALL_UP_LIST, if list != 0 { exe::new_llist(list) } else { 0 });
        for _ in 0..count as i16 {
            if exe::zread(call_up.as_mut_ptr() as usize, 0x12, 1, z) != 1 {
                return fail(0x1ffa);
            }
            exe::llist_add_tail(read_usize(this + f::CALL_UP_LIST), call_up.as_ptr() as usize, 0x12);
        }
    }
    if exe::zread(read_usize(this + f::CLUB_CALL_UP_RECORDS), 6, clubs as u32, z) == clubs as u32 && clubs > 0 {
        for k in 0..clubs {
            let rec = read_usize(this + f::CLUB_CALL_UP_RECORDS) + k as usize * 6;
            let entries = read_i8(rec + 5);
            if entries == 0 {
                continue;
            }
            write_i8(rec + 4, entries);
            let vector = exe::crt_malloc((read_i8(rec + 4) as i32 * 9) as u32);
            write_usize(rec, vector);
            if vector == 0 {
                return fail(0x2013);
            }
            if exe::zread(vector, 9, entries as i32 as u32, z) != entries as i32 as u32 {
                return fail(0x201a);
            }
        }
    }
    let next = read_usize(this + f::NEXT_COMPETITIONS);
    if exe::zread(next, 4, read_u32(G_NATION_COUNT), z) != read_u32(G_NATION_COUNT) {
        for k in 0..read_u32(G_NATION_COUNT) as usize {
            write_i32(next + k * 4, -1);
        }
    }
    exe::zread(read_usize(this + f::OLYMPIC_NATION_MARKS), 1, read_u32(G_NATION_COUNT), z);
    exe::zclose(&mut z);
    1 // sx15_8: loaded (the constructor does not rebuild the tables)
}

/// Comparer for `nationalteams_load_dat`: it reads the save's item (the position moves; it cannot run twice), so only
/// the original runs; the copy is proven by loading the holiday save in the save proof (levas/N/CARGA).
pub(crate) unsafe extern "thiscall" fn compare_nationalteams_load_dat(this: usize, save: usize) -> u32 {
    // ponytail: reads the container once; the save proof loads the holiday save with and without the batch
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_NATIONALTEAMS_LOAD_DAT)(this, save)
}

// ---------- batch 334: the national teams object written to the saved game and destroyed (national_teams.cpp) ----------

/// `FIFA_RANKINGS::save(save)` (thiscall, ret 4): the rankings' own item of the saved game.
const FN_FIFA_RANKINGS_SAVE: usize = 0x58dae0;
/// `FIFA_RANKINGS::~FIFA_RANKINGS()` (thiscall, ret 0).
const FN_FIFA_RANKINGS_DESTRUCTOR: usize = 0x58cf20;

/// One reserve pool written to the saved game: its size (4 bytes from `this + count_field`), then the table at
/// `this + pool_field` with each person turned into his id (none: −1), written as size dwords and turned back into
/// persons (−1: none). False after the national_teams.cpp box of the failing step (`lines`: the size, the ids), with
/// the table left as ids when the ids failed.
unsafe fn save_reserve_pool(this: usize, count_field: usize, pool_field: usize, z: usize, lines: [u32; 2]) -> bool {
    if exe::zwrite(this + count_field, 4, 1, z) != 1 {
        show_error_at(NATIONAL_TEAMS_CPP, lines[0]);
        return false;
    }
    let pool = read_usize(this + pool_field);
    for k in 0..read_i32(this + count_field).max(0) as usize {
        let person = read_usize(pool + k * 4);
        write_i32(pool + k * 4, if person != 0 { read_i32(person) } else { -1 }); // the person's id (+0)
    }
    let count = read_u32(this + count_field);
    if exe::zwrite(read_usize(this + pool_field), 4, count, z) != count {
        show_error_at(NATIONAL_TEAMS_CPP, lines[1]);
        return false;
    }
    let pool = read_usize(this + pool_field);
    for k in 0..(count as i32).max(0) as usize {
        let id = read_i32(pool + k * 4);
        write_usize(pool + k * 4, if id != -1 { staff_at(id as isize).addr() } else { 0 });
    }
    true
}

/// Writes the national teams object into the saved game.
///
/// `NationalTeams::save_dat(save)`: the "national_teams.dat" item (`zopen` with 1, 0x16, 4); then each step checked, a
/// failure showing the national_teams.cpp box of its line and returning with the item left open:
/// - the nations' records (`NATION_RECORDS`, nations × 0x6e bytes; open: line 0x2046, write: 0x204e);
/// - the senior reserve pool (size +0x14, table +0xc; lines 0x2055, 0x206e) and the under-21 one (size +0x18, table
///   +0x10; lines 0x2086, 0x209f), as person ids (`save_reserve_pool`);
/// - the call-up count (2 bytes: the list's count, 0 without a list; line 0x20bf) and, with a list, every 0x12-byte
///   call-up from its head (`LLIST::get_next`, the cursor reset before; line 0x20cd);
/// - the club call-up records (`CLUB_CALL_UP_RECORDS`, 6 bytes per club without the national sides; line 0x20d6) and,
///   for each record with entries (+5), its vector (+0) of that many 9-byte entries (line 0x20e3);
/// - the next competitions (`NEXT_COMPETITIONS`, 4 bytes per nation; line 0x20ec) and the Olympic marks
///   (`OLYMPIC_NATION_MARKS`, 1 byte per nation; line 0x20f3).
/// The item closed, then the FIFA rankings write their own item (`FIFA_RANKINGS::save(save)`).
/// Original: `NationalTeams::save_dat` (0x776a20, national_teams.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn nationalteams_save_dat(this: usize, save: usize) {
    use national_teams_fields as f;
    let clubs = club_call_up_record_count();
    let mut z = exe::zopen(save, NATIONAL_TEAMS_DAT, 1, 0x16, 4);
    let fail = |line: u32| show_error_at(NATIONAL_TEAMS_CPP, line);
    if z == 0 {
        return fail(0x2046);
    }
    covered!("sx15_9 save national teams");
    if exe::zwrite(read_usize(this + f::NATION_RECORDS), 0x6e, read_u32(G_NATION_COUNT), z) != read_u32(G_NATION_COUNT) {
        return fail(0x204e);
    }
    // the two reserve pools: sizes at +0x14 and +0x18, tables at RESERVE_POOLS (+0xc) and +0x10
    if !save_reserve_pool(this, 0x14, f::RESERVE_POOLS, z, [0x2055, 0x206e])
        || !save_reserve_pool(this, 0x18, f::RESERVE_POOLS + 4, z, [0x2086, 0x209f])
    {
        return;
    }
    let mut count = 0u32;
    let list = read_usize(this + f::CALL_UP_LIST);
    if list != 0 {
        count = read_u16(list + llist::COUNT) as u32;
        write_usize(list + llist::CURSOR, read_usize(list + llist::HEAD));
    }
    if exe::zwrite(&mut count as *mut u32 as usize, 2, 1, z) != 1 {
        return fail(0x20bf);
    }
    let list = read_usize(this + f::CALL_UP_LIST);
    if list != 0 {
        write_usize(list + llist::CURSOR, read_usize(list + llist::HEAD));
        let mut call_up = exe::llist_get_next(read_usize(this + f::CALL_UP_LIST));
        while call_up != 0 {
            if exe::zwrite(call_up, 0x12, 1, z) != 1 {
                return fail(0x20cd);
            }
            call_up = exe::llist_get_next(read_usize(this + f::CALL_UP_LIST));
        }
    }
    if exe::zwrite(read_usize(this + f::CLUB_CALL_UP_RECORDS), 6, clubs as u32, z) != clubs as u32 {
        return fail(0x20d6);
    }
    for k in 0..clubs.max(0) as usize {
        let rec = read_usize(this + f::CLUB_CALL_UP_RECORDS) + k * 6;
        let entries = read_i8(rec + 5) as i32;
        if entries != 0 && exe::zwrite(read_usize(rec), 9, entries as u32, z) != entries as u32 {
            return fail(0x20e3);
        }
    }
    if exe::zwrite(read_usize(this + f::NEXT_COMPETITIONS), 4, read_u32(G_NATION_COUNT), z) != read_u32(G_NATION_COUNT) {
        return fail(0x20ec);
    }
    let marks = read_u32(G_NATION_COUNT); // sx15_9: the Olympic marks, one byte per nation
    if exe::zwrite(read_usize(this + f::OLYMPIC_NATION_MARKS), 1, marks, z) != marks {
        return fail(0x20f3);
    }
    exe::zclose(&mut z);
    let rankings_save: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_FIFA_RANKINGS_SAVE);
    rankings_save(read_usize(this + f::FIFA_RANKING), save);
}

/// Comparer for `nationalteams_save_dat`: it writes the save's item (it cannot run twice), so only the original runs;
/// the copy is proven by the save proof (the .qst written with and without the batch).
pub(crate) unsafe extern "thiscall" fn compare_nationalteams_save_dat(this: usize, save: usize) {
    // ponytail: writes the container once; the save proof compares the .qst
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_NATIONALTEAMS_SAVE_DAT)(this, save)
}

/// Frees what the national teams object owns, without clearing the fields.
///
/// `NationalTeams::~NationalTeams()`: the FIFA rankings (`FIFA_RANKING`: destructor and `delete`), the call-up list
/// (`CALL_UP_LIST`: `LLIST::~LLIST` and `delete`), then `free` of the nation records, the next competitions and the two
/// reserve pools; each club call-up record's vector and then the records (`CLUB_CALL_UP_RECORDS`, the club count read
/// again at each step); the club marks (`CLUB_MARKS`) and the Olympic marks; last, the inline new qualifiers list
/// (`NEW_QUALIFIERS`, `LLIST::~LLIST`). Run by free_game_data (0x8301ca) when a game is closed (another one loaded,
/// back to the menu).
/// Original: `NationalTeams::~NationalTeams` (0x76a710, national_teams.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn nationalteams_destructor(this: usize) {
    use national_teams_fields as f;
    let rankings = read_usize(this + f::FIFA_RANKING);
    if rankings != 0 {
        let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_FIFA_RANKINGS_DESTRUCTOR);
        destroy(rankings);
        exe::crt_operator_delete(rankings);
    }
    let list = read_usize(this + f::CALL_UP_LIST);
    if list != 0 {
        exe::drop_llist(list);
        exe::crt_operator_delete(list);
    }
    for field in [f::NATION_RECORDS, f::NEXT_COMPETITIONS, f::RESERVE_POOLS, f::RESERVE_POOLS + 4] {
        let p = read_usize(this + field);
        if p != 0 {
            exe::crt_free(p);
        }
    }
    if read_usize(this + f::CLUB_CALL_UP_RECORDS) != 0 {
        covered!("sx15_9 national teams destroyed");
        let mut k = 0;
        while k < club_call_up_record_count() {
            let vector = read_usize(read_usize(this + f::CLUB_CALL_UP_RECORDS) + k as usize * 6);
            if vector != 0 {
                exe::crt_free(vector);
            }
            k += 1;
        }
        exe::crt_free(read_usize(this + f::CLUB_CALL_UP_RECORDS));
    }
    for field in [f::CLUB_MARKS, f::OLYMPIC_NATION_MARKS] {
        let p = read_usize(this + field);
        if p != 0 {
            exe::crt_free(p);
        }
    }
    exe::drop_llist(this + f::NEW_QUALIFIERS);
}

/// Comparer for `nationalteams_destructor`: it frees the tables (it cannot run twice), so only the original runs; the
/// copy runs in the save proof when the holiday save is loaded over a game (free_game_data destroys the object first).
pub(crate) unsafe extern "thiscall" fn compare_nationalteams_destructor(this: usize) {
    // ponytail: frees once; nothing to compare after it
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_NATIONALTEAMS_DESTRUCTOR)(this)
}

// --- sx09c_2: fine.dat in the save: Fine::load (0x5a5180) and Fine::save (0x5a53d0), Fine.cpp ---

/// The save item "fine.dat".
const FINE_DAT: usize = 0x9ca1d4;
/// The source path of Fine.cpp in the "Error" box ("E:\dev\CM3\cm3 00-01\cm3\code\Fine.cpp").
const FINE_CPP_PATH: usize = 0x9ca1e0;

/// Reads `count` records of `size` bytes from the open save item into the list `list` (`LLIST::add_tail` each); false at
/// the first short read (the records read so far stay in the list).
unsafe fn fine_read_records(z: usize, list: usize, count: i32, size: u32) -> bool {
    let mut record = [0u8; 0x11];
    for _ in 0..count.max(0) {
        if exe::zread(record.as_mut_ptr() as usize, size, 1, z) != 1 {
            return false;
        }
        exe::llist_add_tail(list, record.as_ptr() as usize, size);
    }
    true
}

/// Reads the Fine manager from the save's "fine.dat": without a save or the item, 0. The fine count (dword), a new list
/// (`operator new(0xe)` and `LLIST::LLIST`, null when the allocation fails) in +0 with that many 14-byte fines; the
/// length of the per-club fine count table (dword), the table (`malloc`, not checked) in +8 and its bytes; a second new
/// list in +4 with the coach feedback count (dword) of 17-byte records. The item closed, 1. A short read closes the
/// item and answers 0 (what was built stays: the constructor destroys the lists and builds new ones).
/// Original: `Fine::load` (0x5a5180, Fine.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn fine_load(this: usize, save: usize) -> u8 {
    if save == 0 {
        return 0;
    }
    let mut z = exe::zopen(save, FINE_DAT, 0, 0x16, 4);
    if z == 0 {
        return 0;
    }
    covered!("sx09c_2 load fine.dat");
    let mut count = 0i32;
    if exe::zread(&mut count as *mut i32 as usize, 4, 1, z) != 1 {
        exe::zclose(&mut z);
        return 0;
    }
    write_usize(this, club_history_new_list());
    if !fine_read_records(z, read_usize(this), count, 0xe) {
        exe::zclose(&mut z);
        return 0;
    }
    let mut length = 0u32;
    if exe::zread(&mut length as *mut u32 as usize, 4, 1, z) != 1 {
        exe::zclose(&mut z);
        return 0;
    }
    let counts = exe::crt_malloc(length);
    write_usize(this + 8, counts); // sx09c_2: the per-club fine counts
    if exe::zread(counts, 1, length, z) != length {
        exe::zclose(&mut z);
        return 0;
    }
    write_usize(this + 4, club_history_new_list());
    let mut reports = 0i32;
    if exe::zread(&mut reports as *mut i32 as usize, 4, 1, z) != 1 || !fine_read_records(z, read_usize(this + 4), reports, 0x11) {
        exe::zclose(&mut z);
        return 0;
    }
    exe::zclose(&mut z);
    1
}

/// Comparer for `fine_load`: it reads the save item and allocates, so it cannot run twice; only the original runs.
/// The copy is proven by loading the holiday save in the save proof (levas/N/CARGA).
pub(crate) unsafe extern "thiscall" fn compare_fine_load(this: usize, save: usize) -> u8 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_FINE_LOAD)(this, save)
}

/// Writes the records of `list` (from its head, `count` re-read each step) to the open save item, `size` bytes each;
/// the line of the "Error" box when a node is missing (`missing`) or a write is short (`short`).
unsafe fn fine_write_records(z: usize, list: Llist, size: u32, missing: u32, short: u32) -> Result<(), u32> {
    list.set_cursor(list.head());
    let mut k = 0i32;
    while k < list.count() as i32 {
        let record = exe::llist_get_next(list);
        if record == 0 {
            return Err(missing);
        }
        if exe::zwrite(record, size, 1, z) != 1 {
            return Err(short);
        }
        k += 1;
    }
    Ok(())
}

/// Writes the Fine manager to the save's "fine.dat": the fine count (the list's i16 as a dword), the 14-byte fines,
/// the club count (`G_CLUB_COUNT`, the global itself), the per-club fine count table (that many bytes), the coach
/// feedback count and its 17-byte records; the item closed, 1. Without a save (line 0x109) or the item (0x111): the
/// Fine.cpp "Error" box and 0. A short write or a missing node: the box (lines 0x119, 0x123, 0x129, 0x132, 0x13a,
/// 0x143, 0x14d, 0x153), the item closed and 0.
/// Original: `Fine::save` (0x5a53d0, Fine.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn fine_save(this: usize, save: usize) -> u8 {
    if save == 0 {
        show_error_box(FINE_CPP_PATH, 0x109);
        return 0;
    }
    let mut z = exe::zopen(save, FINE_DAT, 1, 0x16, 4);
    if z == 0 {
        show_error_box(FINE_CPP_PATH, 0x111);
        return 0;
    }
    covered!("sx09c_2 save fine.dat");
    let write = |z: usize| -> Result<(), u32> {
        let fines = Llist(read_usize(this));
        let n = fines.count() as i32;
        if exe::zwrite(&n as *const i32 as usize, 4, 1, z) != 1 {
            return Err(0x119);
        }
        fine_write_records(z, fines, 0xe, 0x123, 0x129)?;
        if exe::zwrite(G_CLUB_COUNT, 4, 1, z) != 1 {
            return Err(0x132);
        }
        if exe::zwrite(read_usize(this + 8), 1, read_u32(G_CLUB_COUNT), z) != read_u32(G_CLUB_COUNT) { // sx09c_2: the fine counts
            return Err(0x13a);
        }
        let feedback = Llist(read_usize(this + 4));
        let n = feedback.count() as i32;
        if exe::zwrite(&n as *const i32 as usize, 4, 1, z) != 1 {
            return Err(0x143);
        }
        fine_write_records(z, feedback, 0x11, 0x14d, 0x153)
    };
    let r = write(z);
    if let Err(row) = r {
        show_error_box(FINE_CPP_PATH, row);
    }
    exe::zclose(&mut z);
    r.is_ok() as u8
}

/// Comparer for `fine_save`: it writes into the save container, so it cannot run twice; only the original runs. The
/// copy is proven by the save proof (the .qst written with and without the batch, item by item).
pub(crate) unsafe extern "thiscall" fn compare_fine_save(this: usize, save: usize) -> u8 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_FINE_SAVE)(this, save)
}

// --- sx09c_3: dispute.dat in the save: DISPUTE::load (0x569910) and DISPUTE::save (0x569ae0), dispute.cpp ---

/// The save item "dispute.dat".
const DISPUTE_DAT: usize = 0x9c384c;
/// The source path of dispute.cpp in the "Error" box ("E:\dev\CM3\cm3 00-01\cm3\code\dispute.cpp").
const DISPUTE_CPP_PATH: usize = 0x9c3858;
/// The game data version from which saves carry dispute.dat (older ones have none).
const DISPUTE_DAT_VERSION: i32 = 0x3e046;

/// Reads the dispute object from the save's "dispute.dat": nothing (0) when the game data version (`G_GAME_DATA_VERSION`,
/// in absolute value) is below 0x3e046, without a save or without the item. The flag table length (dword; 0 closes
/// the item and answers 0), the table (`malloc` into +4; failing, the item closed and 0) and its bytes; the dispute
/// count (dword), a new list (`operator new(0xe)` and `LLIST::LLIST`, null when the allocation fails) in +0 and that
/// many 17-byte disputes (`LLIST::add_tail`). The item closed, 1. A short read closes the item and answers 0.
/// Original: `DISPUTE::load` (0x569910, dispute.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn dispute_load(this: usize, save: usize) -> u8 {
    if read_i32(G_GAME_DATA_VERSION).wrapping_abs() < DISPUTE_DAT_VERSION || save == 0 {
        return 0;
    }
    let mut z = exe::zopen(save, DISPUTE_DAT, 0, 0x16, 4);
    if z == 0 {
        return 0;
    }
    covered!("sx09c_3 load dispute.dat");
    let read = |z: usize| -> bool {
        let mut length = 0u32;
        if exe::zread(&mut length as *mut u32 as usize, 4, 1, z) != 1 || length == 0 {
            return false;
        }
        let flags = exe::crt_malloc(length);
        write_usize(this + 4, flags); // sx09c_3: the per-person dispute flags
        if flags == 0 || exe::zread(flags, 1, length, z) != length {
            return false;
        }
        let mut count = 0i32;
        if exe::zread(&mut count as *mut i32 as usize, 4, 1, z) != 1 {
            return false;
        }
        let list = club_history_new_list();
        write_usize(this, list);
        let mut record = [0u8; 0x11];
        for _ in 0..count.max(0) {
            if exe::zread(record.as_mut_ptr() as usize, 0x11, 1, z) != 1 {
                return false;
            }
            exe::llist_add_tail(list, record.as_ptr() as usize, 0x11);
        }
        true
    };
    let ok = read(z);
    exe::zclose(&mut z);
    ok as u8
}

/// Comparer for `dispute_load`: it reads the save item and allocates, so it cannot run twice; only the original runs.
/// The copy is proven by loading the holiday save in the save proof (levas/N/CARGA).
pub(crate) unsafe extern "thiscall" fn compare_dispute_load(this: usize, save: usize) -> u8 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_DISPUTE_LOAD)(this, save)
}

/// Writes the dispute object to the save's "dispute.dat": the staff count (`G_STAFF_COUNT`, the global itself), the flag
/// table (that many bytes), the dispute count (the list's i16 as a dword) and the 17-byte disputes (from the list's
/// head); the item closed, 1. Without a save (line 0xe9) or the item (0xf1): the dispute.cpp "Error" box and 0. A short
/// write or a missing node: the box (lines 0xf8, 0xff, 0x108, 0x111, 0x117), the item closed and 0.
/// Original: `DISPUTE::save` (0x569ae0, dispute.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn dispute_save(this: usize, save: usize) -> u8 {
    if save == 0 {
        show_error_box(DISPUTE_CPP_PATH, 0xe9);
        return 0;
    }
    let mut z = exe::zopen(save, DISPUTE_DAT, 1, 0x16, 4);
    if z == 0 {
        show_error_box(DISPUTE_CPP_PATH, 0xf1);
        return 0;
    }
    covered!("sx09c_3 save dispute.dat");
    let write = |z: usize| -> Result<(), u32> {
        if exe::zwrite(G_STAFF_COUNT, 4, 1, z) != 1 { // sx09c_3: the people count
            return Err(0xf8);
        }
        if exe::zwrite(read_usize(this + 4), 1, read_u32(G_STAFF_COUNT), z) != read_u32(G_STAFF_COUNT) { // sx09c_3: the flags
            return Err(0xff);
        }
        let list = Llist(read_usize(this));
        let n = list.count() as i32;
        if exe::zwrite(&n as *const i32 as usize, 4, 1, z) != 1 {
            return Err(0x108);
        }
        list.set_cursor(list.head());
        let mut k = 0i32;
        while k < list.count() as i32 {
            let record = exe::llist_get_next(list);
            if record == 0 {
                return Err(0x111);
            }
            if exe::zwrite(record, 0x11, 1, z) != 1 {
                return Err(0x117);
            }
            k += 1;
        }
        Ok(())
    };
    let r = write(z);
    if let Err(row) = r {
        show_error_box(DISPUTE_CPP_PATH, row);
    }
    exe::zclose(&mut z);
    r.is_ok() as u8
}

/// Comparer for `dispute_save`: it writes into the save container, so it cannot run twice; only the original runs.
/// The copy is proven by the save proof (the .qst written with and without the batch, item by item).
pub(crate) unsafe extern "thiscall" fn compare_dispute_save(this: usize, save: usize) -> u8 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_DISPUTE_SAVE)(this, save)
}
