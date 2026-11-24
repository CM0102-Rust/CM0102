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
    original_fn::<unsafe extern "C" fn(u32)>(%%I0%%)(size)
}
