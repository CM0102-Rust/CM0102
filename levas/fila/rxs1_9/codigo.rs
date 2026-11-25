/// `database_shuffle_player_records` (0x533080, cdecl()).
pub(crate) const FN_DATABASE_SHUFFLE_PLAYER_RECORDS: usize = 0x533080;

/// `database_shuffle_squad_player_attributes` (0x533710, cdecl()).
pub(crate) const FN_DATABASE_SHUFFLE_SQUAD_PLAYER_ATTRIBUTES: usize = 0x533710;

/// `Staff::operator=` (0x5335f0, thiscall(dest, src) ret 4, returns dest).
pub(crate) const FN_STAFF_ASSIGN: usize = 0x5335f0;

/// The international caps and goals of a fake player, made plausible for his age, ability and reputation.
///
/// Caps (`staff::INT_APPS`): none yet, a player with `.current_ability()` above 140 and `staff::AGE` above 20 gets a
/// random(3) with one chance in 4 (random(4) = 0); with caps, by his age today (`exe::database_get_staff_age` with
/// `G_CURRENT_DATE`): below 20, below 30 with ability below 80, or below 26 with `.home_reputation()` below 4750 lose
/// them; below 25 with more than 10 caps random(5); otherwise caps − random(caps). Goals (`staff::INT_GOALS`), when he
/// has some: none without caps; more than 4 and neither attacker nor attacking midfielder (rating 20) gives 1; more goals
/// than caps gives random(caps) to an attacker and none otherwise; else one chance in 3 (random(3) = 0) of
/// random(3) + 1 more, otherwise 1 + random(goals) fewer (the byte wraps). No caps, no goals.
unsafe fn fake_player_caps(s: Staff) {
    let p = s.player();
    let age = || exe::database_get_staff_age(s, G_CURRENT_DATE as *const u16);
    let mut caps = read_u8(s.addr() + staff::INT_APPS);
    if caps == 0 {
        if p.current_ability() > 0x8c && read_i8(s.addr() + staff::AGE) > 0x14 && random(4) == 0 {
            caps = random(3) as u8;
        }
    } else if age() < 0x14 || (age() < 0x1e && p.current_ability() < 0x50) || (age() < 0x1a && p.home_reputation() < 0x128e) {
        caps = 0;
    } else if age() < 0x19 && caps > 10 {
        caps = random(5) as u8;
    } else {
        covered!("rxs1_9 fake player keeps fewer caps");
        caps = caps.wrapping_sub(random(caps as i32) as u8); // rxs1_9: fewer caps
    }
    write_u8(s.addr() + staff::INT_APPS, caps);
    let mut goals = read_u8(s.addr() + staff::INT_GOALS);
    if goals != 0 {
        let striker = read_i8(p.addr() + player_sheet::ATTACKER) == 0x14;
        goals = if caps == 0 {
            0
        } else if goals > 4 && !striker && read_i8(p.addr() + player_sheet::ATTACKING_MIDFIELDER) != 0x14 {
            1
        } else if goals > caps {
            if striker { random(caps as i32) as u8 } else { 0 }
        } else if random(3) != 0 {
            goals.wrapping_add(0xffu8.wrapping_sub(random(goals as i32) as u8))
        } else {
            goals.wrapping_add(random(3) as u8 + 1)
        };
        write_u8(s.addr() + staff::INT_GOALS, goals);
    }
    if caps == 0 {
        write_u8(s.addr() + staff::INT_GOALS, 0);
    }
}

/// Replaces the real people with fake ones ("Use Real Players: No", or the German setup flag).
///
/// `database_apply_fake_players()`, from the game setup: first the player records are shuffled
/// (`database_shuffle_player_records`); then every person with a nation gets a random first and second name of it
/// (`exe::database_pick_first_name_for_nation` / `..._second_name_...`, kept only when not empty) and the empty common
/// name (`free_record`); then the squad players' attributes are shuffled (`database_shuffle_squad_player_attributes`);
/// last, every person with a nation and a player sheet gets plausible caps and goals (`fake_player_caps`) and his
/// `staff::WAGE` and `staff::VALUE` zeroed.
/// Original: `database_apply_fake_players` (0x532e00, Database.cpp, cdecl).
pub(crate) unsafe extern "C" fn database_apply_fake_players() {
    let call = |f: usize| {
        let g: unsafe extern "C" fn() = core::mem::transmute(f);
        g()
    };
    call(FN_DATABASE_SHUFFLE_PLAYER_RECORDS);
    for k in 0..read_i32(G_STAFF_COUNT) {
        let s = staff_at(k as isize);
        let nation = read_usize(s.addr() + staff::NATION);
        if nation == 0 {
            continue;
        }
        let first = exe::database_pick_first_name_for_nation(nation);
        if first != 0 && read_u8(first) != 0 {
            write_usize(s.addr() + staff::FIRST_NAME, first);
        }
        let second = exe::database_pick_second_name_for_nation(nation);
        if second != 0 && read_u8(second) != 0 {
            write_usize(s.addr() + staff::SECOND_NAME, second);
        }
        write_usize(s.addr() + staff::COMMON_NAME, free_record());
    }
    call(FN_DATABASE_SHUFFLE_SQUAD_PLAYER_ATTRIBUTES);
    for k in 0..read_i32(G_STAFF_COUNT) {
        let s = staff_at(k as isize);
        if read_usize(s.addr() + staff::NATION) == 0 || s.player().is_null() {
            continue;
        }
        fake_player_caps(s);
        write_u32(s.addr() + staff::WAGE, 0);
        write_u32(s.addr() + staff::VALUE, 0);
    }
}

/// Comparer for `database_apply_fake_players`: the original only (it draws and rewrites the whole staff table once,
/// at the setup), then what it guarantees is checked: every person with a nation and a player sheet has no wage, no
/// value and no goals without caps.
pub(crate) unsafe extern "C" fn compare_database_apply_fake_players() {
    original_fn::<unsafe extern "C" fn()>(%%I0%%)(); // simplification: original only
    let bad = (0..read_i32(G_STAFF_COUNT)).map(|k| staff_at(k as isize)).find(|&s| {
        read_usize(s.addr() + staff::NATION) != 0 && !s.player().is_null()
            && (read_u32(s.addr() + staff::WAGE) != 0 || read_u32(s.addr() + staff::VALUE) != 0
                || (read_u8(s.addr() + staff::INT_APPS) == 0 && read_u8(s.addr() + staff::INT_GOALS) != 0))
    });
    SCOREBOARD.record(bad.is_none(), || format!("database_apply_fake_players: person {:#x} keeps a wage, a value or goals without caps", bad.map_or(0, |s| s.addr())));
}

/// Swaps the records of random pairs of players, keeping in place their ids, their club jobs, their abilities and, by
/// a coin, their dates of birth, when the nationality rules allow it.
///
/// `database_shuffle_player_records()`: staff count × 16 (`rdata::F64_16`, truncated) tries of two random people
/// (random(staff count) twice). A try counts when they differ, both have a player sheet (`staff::PLAYER`), both or
/// neither have a non-player sheet (`staff::NON_PLAYER`) and both have a club job (`staff::CLUB_JOB`). When the clubs'
/// nation (`club::NATION`) or division (`club::DIVISION`) differ, the two must have the same
/// `exe::database_is_staff_eu_national` (with 0) and each must pass `exe::database_is_staff_foreign` (his club's nation,
/// 1). Then the 110-byte records are swapped (the second through `Staff::operator=`), the ids (+0) and the club jobs
/// swapped back, with random(2) ≠ 0 the dates of birth swapped back too, and the current and potential abilities of
/// the two player sheets swapped (so the abilities stay with the record's place).
/// Original: `database_shuffle_player_records` (0x533080, Database.cpp, cdecl).
pub(crate) unsafe extern "C" fn database_shuffle_player_records() {
    let assign: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_STAFF_ASSIGN);
    let tries = ftol(read_i32(G_STAFF_COUNT) as f64 * read_f64(rdata::F64_16));
    let mut left = tries;
    while left > 0 {
        let (i, j) = (random(read_i32(G_STAFF_COUNT)), random(read_i32(G_STAFF_COUNT)));
        left -= 1;
        if i == j {
            continue;
        }
        let (a, b) = (staff_at(i as isize), staff_at(j as isize));
        if a.player().is_null() || b.player().is_null() {
            continue;
        }
        if (read_usize(a.addr() + staff::NON_PLAYER) != 0) != (read_usize(b.addr() + staff::NON_PLAYER) != 0) {
            continue;
        }
        let (ca, cb) = (read_usize(a.addr() + staff::CLUB_JOB), read_usize(b.addr() + staff::CLUB_JOB));
        if ca == 0 || cb == 0 {
            continue;
        }
        let (na, nb) = (read_u32(ca + club::NATION), read_u32(cb + club::NATION));
        if na != nb || read_u32(ca + club::DIVISION) != read_u32(cb + club::DIVISION) {
            if exe::database_is_staff_eu_national(a, 0usize) != exe::database_is_staff_eu_national(b, 0usize)
                || exe::database_is_staff_foreign(a, na as usize, 1) == 0
                || exe::database_is_staff_foreign(b, nb as usize, 1) == 0
            {
                continue;
            }
        }
        covered!("rxs1_9 player records swapped");
        let mut kept = [0u8; 0x6e]; // record a, by value
        core::ptr::copy_nonoverlapping(a.addr() as *const u8, kept.as_mut_ptr(), 0x6e);
        core::ptr::copy_nonoverlapping(b.addr() as *const u8, a.addr() as *mut u8, 0x6e);
        assign(b.addr(), kept.as_ptr() as usize);
        for field in [0, staff::CLUB_JOB] {
            let (x, y) = (read_u32(a.addr() + field), read_u32(b.addr() + field));
            write_u32(a.addr() + field, y);
            write_u32(b.addr() + field, x);
        }
        if random(2) != 0 {
            let born = read_u64(a.addr() + staff::DATE_OF_BIRTH); // rxs1_9: the dates of birth stay
            write_u64(a.addr() + staff::DATE_OF_BIRTH, read_u64(b.addr() + staff::DATE_OF_BIRTH));
            write_u64(b.addr() + staff::DATE_OF_BIRTH, born);
        }
        let (pa, pb) = (a.player(), b.player());
        let (current, potential) = (pa.current_ability(), pa.potential_ability());
        pa.set_current_ability(pb.current_ability());
        pa.set_potential_ability(pb.potential_ability());
        pb.set_current_ability(current);
        pb.set_potential_ability(potential);
    }
}

/// Comparer for `database_shuffle_player_records`: the original only (thousands of draws over the staff table), then
/// what it guarantees is checked: every record keeps its id (+0) and its club job.
pub(crate) unsafe extern "C" fn compare_database_shuffle_player_records() {
    let place = |k: i32| {
        let s = staff_at(k as isize);
        (read_u32(s.addr()), read_u32(s.addr() + staff::CLUB_JOB))
    };
    let before: Vec<(u32, u32)> = (0..read_i32(G_STAFF_COUNT)).map(place).collect();
    original_fn::<unsafe extern "C" fn()>(%%I1%%)(); // simplification: original only
    let bad = (0..before.len() as i32).find(|&k| place(k) != before[k as usize]);
    SCOREBOARD.record(bad.is_none(), || format!("database_shuffle_player_records: the record at {} changed its id or club job", bad.unwrap_or(-1)));
}

/// Swaps attributes between random similar squad mates of every club, then rebalances every player.
///
/// `database_shuffle_squad_player_attributes()`: for each club (`G_CLUB_COUNT`), 100 tries of two random squad places
/// (random(50) twice, `club::SQUAD`). Both must be people with a player sheet; with caps (`staff::INT_APPS`) on either,
/// their `.current_ability()` may differ by 10 at most; their `staff::AGE` by 4 at most. Then, each with a coin
/// (random(2) ≠ 0): pace and acceleration swapped; stamina swapped; jumping and heading swapped (only when the first is
/// not a defender rated 20, tested before the coin); and always the current and potential abilities swapped. Last,
/// every person with a player sheet is rebalanced (`exe::staff_normalise_player_physical_attributes`, then
/// `exe::staff_normalise_player_attributes`).
/// Original: `database_shuffle_squad_player_attributes` (0x533710, Database.cpp, cdecl).
pub(crate) unsafe extern "C" fn database_shuffle_squad_player_attributes() {
    let swap_byte = |a: PlayerSheet, b: PlayerSheet, field: usize| {
        let x = read_u8(a.addr() + field);
        write_u8(a.addr() + field, read_u8(b.addr() + field));
        write_u8(b.addr() + field, x);
    };
    for c in 0..read_i32(G_CLUB_COUNT) {
        let club = club_at(c as isize);
        for _ in 0..100 {
            let i = random(0x32);
            let a = Staff(read_usize(club.addr() + club::SQUAD + i as usize * 4));
            let j = random(0x32);
            let b = Staff(read_usize(club.addr() + club::SQUAD + j as usize * 4));
            if a.is_null() || a.player().is_null() || b.is_null() || b.player().is_null() {
                continue;
            }
            let (pa, pb) = (a.player(), b.player());
            let capped = read_u8(a.addr() + staff::INT_APPS) != 0 || read_u8(b.addr() + staff::INT_APPS) != 0;
            if capped && (pa.current_ability() as i32 - pb.current_ability() as i32).abs() > 10 {
                continue;
            }
            if (read_i8(a.addr() + staff::AGE) as i32 - read_i8(b.addr() + staff::AGE) as i32).abs() > 4 {
                continue;
            }
            if random(2) != 0 {
                swap_byte(pa, pb, player_sheet::PLAYER_PACE);
                swap_byte(pa, pb, player_sheet::ACCELERATION);
            }
            if random(2) != 0 {
                swap_byte(pa, pb, player_sheet::STAMINA);
            }
            if read_i8(pa.addr() + player_sheet::DEFENDER) != 0x14 && random(2) != 0 {
                swap_byte(pa, pb, player_sheet::JUMPING);
                swap_byte(pa, pb, player_sheet::HEADING);
            }
            covered!("rxs1_9 squad mates swap abilities");
            let (current, potential) = (pa.current_ability(), pa.potential_ability()); // rxs1_9: the abilities swapped
            pa.set_current_ability(pb.current_ability());
            pa.set_potential_ability(pb.potential_ability());
            pb.set_current_ability(current);
            pb.set_potential_ability(potential);
        }
    }
    for k in 0..read_i32(G_STAFF_COUNT) {
        let s = staff_at(k as isize);
        if !s.player().is_null() {
            exe::staff_normalise_player_physical_attributes(s);
            exe::staff_normalise_player_attributes(s);
        }
    }
}

/// Comparer for `database_shuffle_squad_player_attributes`: the original only (draws over every squad and rebalances
/// every player once, at the setup).
pub(crate) unsafe extern "C" fn compare_database_shuffle_squad_player_attributes() {
    original_fn::<unsafe extern "C" fn()>(%%I2%%)() // simplification: original only
}
