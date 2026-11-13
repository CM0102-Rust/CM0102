// ---------- batch %%LEVA%%: creating one virtual player (virtual_staff.cpp) ----------

/// Constants of `create_virtual_player` that have no name elsewhere.
pub(crate) mod virtual_player_consts {
    /// The player record size (70 bytes, `G_PLAYER_DATA_TABLE`).
    pub(crate) const PLAYER_SIZE: usize = 0x46;
    /// First name and second name counts of the name tables (`G_FIRST_NAME_TABLE`, `G_SECOND_NAME_TABLE`).
    pub(crate) const FIRST_NAME_COUNT: usize = 0xae2398;
    pub(crate) const SECOND_NAME_COUNT: usize = 0xae239c;
    /// The name record size (60 bytes).
    pub(crate) const NAME_SIZE: usize = 60;
    /// f64 0.016 and 0.052 (the ability of a club and of a national team from its reputation).
    pub(crate) const F64_0_016: usize = 0x970a80;
    pub(crate) const F64_0_052: usize = 0x970a78;
    /// The Basque clubs whose virtual players get the Basque second nation (`KEY_NATION_PAYS_BASQUE`): Athletic
    /// Bilbao, its B side, Real Sociedad, its B side and Alavés (key ids).
    pub(crate) const BASQUE_CLUBS: [usize; 5] = [0x9d0120, 0x9d0134, 0x9d0128, 0x9d01a0, 0x9d0124];
    /// The most template players looked at for one with the group's positions.
    pub(crate) const TEMPLATE_TRIES: i32 = 1000;
}

/// Whether a template player record plays the position group (`get_player_position_group` numbering; 15 or more,
/// and exactly 20 for a goalkeeper).
unsafe fn template_fits_group(t: PlayerSheet, group: i32) -> bool {
    const GOOD: i8 = 15;
    match group {
        0 => t.goalkeeper() == 20, // mwg3: a template goalkeeper is a perfect one
        1 => t.defender() >= GOOD && t.left_side() >= GOOD,
        2 => t.defender() >= GOOD && t.right_side() >= GOOD,
        3 => t.defender() >= GOOD && t.central() >= GOOD,
        4 => t.midfielder() >= GOOD && t.left_side() >= GOOD,
        5 => t.midfielder() >= GOOD && t.central() >= GOOD,
        6 => t.midfielder() >= GOOD && t.right_side() >= GOOD,
        7 => t.attacker() >= GOOD,
        _ => false,
    }
}

/// Creates one virtual player of a club at the given staff and player record ids and puts him in the squad.
///
/// `VIRTUAL_STAFF::create_virtual_player(club, group, staff_id, player_id, youngster, reserve, squad_number)`. The
/// staff record gets its id and names: a first and a second name drawn for the club's nation
/// (`exe::database_pick_first_name_for_nation` / `_second_`), or from the whole name tables (draw over the counts)
/// when the club has none, and the common name table's first entry. Born on a drawn day (365) of the year 17 years
/// before the current one rounded down to a multiple of 3 for a youngster, or 17 to 30 years before (draw 14)
/// otherwise; the age (`exe::database_get_staff_age`), the club's nation, the Basque second nation at the Basque clubs
/// (`BASQUE_CLUBS`), no caps, empty national dates; at a national team (`exe::database_is_national_team`) the national
/// job 11 there, otherwise the club job 11; joined the club on a drawn day of the year before, contract to a drawn day
/// six years on; wage the club's reputation / 5, no value, adaptability 1 + draw(10), the other personality ratings
/// 10, classification 2, the player record, no preferences or non-player data, squad mark 1 (2 for a reserve), or at
/// a national team 4 (8). The player record is a copy of a template player drawn from the first 30000 records with
/// the group's positions (`template_fits_group`, up to 1000 draws, the last one kept otherwise); its current and
/// potential ability come from the club's reputation: at a club reputation / 25 − 100, above 5000 reputation × 0.016
/// + 20; at a national team reputation × 0.052 − 150, above 5000 reputation / 100 + 40; a youngster and a reserve
/// each lose 30 + draw(41); then the id, no reputations and the squad number when the club uses squad numbers
/// (`exe::squad_manager_club_uses_squad_numbers`). The staff record takes the first free of the 50 squad slots.
/// Original: `VIRTUAL_STAFF::create_virtual_player` (0x90e0a0, virtual_staff.cpp, thiscall, ret 0x1c).
pub(crate) unsafe extern "thiscall" fn create_virtual_player(_this: usize, club: usize, group: u32, staff_id: u32, player_id: u32, youngster: u32, reserve: u32, squad_number: u32) {
    use virtual_player_consts as vp;
    let c = Club(club);
    let cap = read_i32(G_PLAYER_DATA_COUNT).min(30000);
    let s = Staff(read_usize(G_STAFF_TABLE).wrapping_add(staff_id as usize * staff::SIZE));
    let p = PlayerSheet(read_usize(G_PLAYER_DATA_TABLE).wrapping_add(player_id as usize * vp::PLAYER_SIZE));
    let base_year = read_u16(G_CURRENT_DATE_YEAR) as i32 / 3 * 3;
    write_u32(s.addr(), staff_id);
    if !c.nation().is_null() {
        write_usize(s.addr() + staff::FIRST_NAME, exe::database_pick_first_name_for_nation(c.nation()));
        write_usize(s.addr() + staff::SECOND_NAME, exe::database_pick_second_name_for_nation(c.nation()));
    } else {
        covered!("mwg_3 club without a nation");
        let first = scaled_crt_rand(read_i32(vp::FIRST_NAME_COUNT));
        write_usize(s.addr() + staff::FIRST_NAME, read_usize(G_FIRST_NAME_TABLE).wrapping_add((first as usize).wrapping_mul(vp::NAME_SIZE)));
        let second = scaled_crt_rand(read_i32(vp::SECOND_NAME_COUNT));
        write_usize(s.addr() + staff::SECOND_NAME, read_usize(G_SECOND_NAME_TABLE).wrapping_add((second as usize).wrapping_mul(vp::NAME_SIZE)));
    }
    write_u32(s.addr() + staff::COMMON_NAME, read_u32(G_COMMON_NAME_TABLE));
    let mut date = [0u8; 8];
    let mut tmp = [0u8; 8];
    let d = date.as_mut_ptr() as usize;
    let t = tmp.as_mut_ptr() as usize;
    let born = if youngster != 0 {
        let year = base_year - 17;
        exe::new_date(d, scaled_crt_rand(365) as u32, year as u32)
    } else {
        let year = base_year - scaled_crt_rand(14) - 17;
        exe::new_date(d, scaled_crt_rand(365) as u32, year as u32)
    };
    exe::date_operator_3d(s.addr() + staff::DATE_OF_BIRTH, t, born);
    write_i16(s.addr() + staff::AGE, exe::database_get_staff_age(s, G_CURRENT_DATE as *const u16) as i16);
    write_usize(s.addr() + staff::NATION, c.nation().addr());
    let basque = vp::BASQUE_CLUBS.iter().any(|&k| read_u32(club) == read_u32(k));
    let second_nation = if basque {
        covered!("mwg_3 basque club");
        read_usize(G_NATION_TABLE).wrapping_add(read_u32(KEY_NATION_PAYS_BASQUE) as usize * nation::SIZE)
    } else {
        0
    };
    write_usize(s.addr() + staff::SECOND_NATION, second_nation);
    write_u8(s.addr() + staff::INT_APPS, 0);
    write_u8(s.addr() + staff::INT_GOALS, 0);
    let empty = exe::date_date_54a110(d);
    exe::date_operator_3d(s.addr() + staff::DATE_JOINED_NATION, t, empty);
    let empty = exe::date_date_54a110(d);
    exe::date_operator_3d(s.addr() + staff::DATE_JOINED_NATION + 8, t, empty);
    if exe::database_is_national_team(c) != 0 {
        write_usize(s.addr() + staff::CLUB_JOB, 0);
        write_usize(s.addr() + staff::NATIONAL_JOB, club);
        write_u8(s.addr() + staff::JOB_FOR_CLUB, 0);
        write_u8(s.addr() + staff::JOB_FOR_NATION, 0xb);
    } else {
        write_usize(s.addr() + staff::CLUB_JOB, club);
        write_usize(s.addr() + staff::NATIONAL_JOB, 0);
        write_u8(s.addr() + staff::JOB_FOR_CLUB, 0xb);
        write_u8(s.addr() + staff::JOB_FOR_NATION, 0);
    }
    let joined = exe::new_date(d, scaled_crt_rand(365) as u32, (base_year - 1) as u32);
    exe::date_operator_3d(s.addr() + staff::DATE_JOINED_CLUB, t, joined);
    let expires = exe::new_date(d, scaled_crt_rand(365) as u32, (base_year + 6) as u32);
    exe::date_operator_3d(s.addr() + staff::DATE_EXPIRES_CLUB, t, expires);
    write_i32(s.addr() + staff::VALUE, 0);
    write_i32(s.addr() + staff::WAGE, c.reputation() as i32 / 5);
    write_u8(s.addr() + staff::ADAPTABILITY, (scaled_crt_rand(10) as u8).wrapping_add(1));
    for f in [staff::AMBITION, staff::DETERMINATION, staff::LOYALTY, staff::PRESSURE, staff::PROFESSIONALISM, staff::SPORTSMANSHIP, staff::TEMPERAMENT] {
        write_u8(s.addr() + f, 10);
    }
    write_u8(s.addr() + staff::CLASSIFICATION, 2);
    write_usize(s.addr() + staff::PLAYER, p.addr());
    write_usize(s.addr() + staff::STAFF_PREFERENCES, 0);
    write_usize(s.addr() + staff::NON_PLAYER, 0);
    write_u8(s.addr() + staff::SQUAD_SELECTED_FOR, 0);
    let national = exe::database_is_national_team(c) != 0;
    let mark = match (national, reserve != 0) {
        (true, true) => 8,
        (true, false) => 4,
        (false, true) => 2,
        (false, false) => 1,
    };
    write_u8(s.addr() + staff::PLAYING_SQUAD, mark);
    let g = group as u8 as i8 as i32;
    let mut tries = 0;
    let template = loop {
        let k = scaled_crt_rand(cap);
        tries += 1;
        let t = PlayerSheet(read_usize(G_PLAYER_DATA_TABLE).wrapping_add((k as usize).wrapping_mul(vp::PLAYER_SIZE)));
        if template_fits_group(t, g) || tries >= vp::TEMPLATE_TRIES {
            break t;
        }
    };
    core::ptr::copy_nonoverlapping(template.addr() as *const u8, p.addr() as *mut u8, vp::PLAYER_SIZE);
    let rep = c.reputation() as i32;
    let mut ability = if exe::database_is_national_team(c) == 0 {
        if rep > 5000 {
            ftol(rep as f64 * read_f64(vp::F64_0_016) + read_f64(rdata::F64_20))
        } else {
            rep / 25 - 100 // mwg3: the ability of a club's virtual player
        }
    } else if rep > 5000 {
        rep / 100 + 40
    } else {
        ftol(rep as f64 * read_f64(vp::F64_0_052) - read_f64(rdata::F64_150))
    };
    if youngster != 0 {
        covered!("mwg_3 youngster");
        ability = ability.wrapping_add(-30 - scaled_crt_rand(41)); // mwg3: the youngster loses ability
    }
    if reserve != 0 {
        ability = ability.wrapping_add(-30 - scaled_crt_rand(41));
    }
    p.set_current_ability(ability as i16);
    p.set_potential_ability(ability as i16);
    write_u32(p.addr(), player_id);
    p.set_home_reputation(0);
    p.set_current_reputation(0);
    p.set_world_reputation(0);
    let number = if exe::squad_manager_club_uses_squad_numbers(read_usize(G_SQUAD_MANAGER), c, 0) != 0 { squad_number as u8 } else { 0 };
    write_u8(p.addr() + 4, number);
    if let Some(i) = (0..50).find(|&i| c.squad(i) == 0) {
        c.set_squad(i, s.addr());
    }
}

/// Comparer for `create_virtual_player`.
///
/// Side by side from the same state: the generator (`crate::rng::STATE_REGIONS`, with the CRT seed), the staff and
/// player records being created and the club's 50 squad slots.
pub(crate) unsafe extern "thiscall" fn compare_create_virtual_player(this: usize, club: usize, group: u32, staff_id: u32, player_id: u32, youngster: u32, reserve: u32, squad_number: u32) {
    let mut regions = crate::rng::STATE_REGIONS.to_vec();
    regions.push((read_usize(G_STAFF_TABLE) + staff_id as usize * staff::SIZE, staff::SIZE));
    regions.push((read_usize(G_PLAYER_DATA_TABLE) + player_id as usize * virtual_player_consts::PLAYER_SIZE, virtual_player_consts::PLAYER_SIZE));
    regions.push((club + club::SQUAD, 50 * 4));
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32, u32)>(%%I0%%);
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, club, group, staff_id, player_id, youngster, reserve, squad_number), || {
        crate::rng::without_counting(|| create_virtual_player(this, club, group, staff_id, player_id, youngster, reserve, squad_number))
    });
    SCOREBOARD.record(equal, || format!("create_virtual_player(club {:x}, group {:#x}, staff {}, player {}, youngster {}, reserve {}, number {}): original {:x?} rust {:x?}",
        club, group, staff_id, player_id, youngster, reserve, squad_number, mo, mr));
}
