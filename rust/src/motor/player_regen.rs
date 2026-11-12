use super::*;

/// Calls a one-argument thiscall routine by address (the person clean-up routines).
pub(crate) unsafe fn call_thiscall_1(routine: usize, this: usize, a: Staff) {
    let f: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(routine);
    f(this, a.addr());
}

/// Pointer to byte k of the person's 79-byte state record ([0xae2d0c] + 0x4f x id).
pub(crate) unsafe fn staff_state_byte(p: Staff, k: usize) -> *mut u8 {
    (read_usize(G_STAFF_STATE_TABLE) as isize + read_i32(p.addr()) as isize * (staff_state::SIZE as isize) + k as isize) as *mut u8
}

/// Integer times an .rdata double constant, truncated (_ftol) to 16 bits.
pub(crate) unsafe fn mul_rdata_const(v: i16, k: usize) -> i16 {
    ftol(v as f64 * read_f64(k)) as i16
}

/// Sets current ability (record +5) from PA (+7): trunc(random(trunc(PA x m1)) + PA x m2).
pub(crate) unsafe fn roll_current_ability(f: PlayerSheet, m1: usize, m2: usize) {
    let pa = f.potential_ability() as f64;
    let r = friendly_random(ftol(pa * read_f64(m1)));
    covered!("cqg_7 ability");
    f.set_current_ability(ftol(r as f64 + pa * read_f64(m2)) as i16); // cqg7: the ability drawn by the PA
}

/// Adjusts a 1..20 personality byte: above 15 drops random(cai); below a1 rises random(s1); below a2 rises random(s2).
pub(crate) unsafe fn adjust_personality_attribute(a: usize, cai: i32, a1: i8, s1: i32, a2: i8, s2: i32) {
    let v = read_i8(a);
    if v > 0xf {
        let r = friendly_random(cai);
        write_i8(a, read_i8(a).wrapping_sub(r as i8));
    } else if v < a1 {
        let r = friendly_random(s1);
        write_i8(a, read_i8(a).wrapping_add(r as i8));
    } else if s2 != 0 && v < a2 {
        let r = friendly_random(s2);
        write_i8(a, read_i8(a).wrapping_add(r as i8));
    }
}

/// If the byte is below lim, adds random(n).
pub(crate) unsafe fn add_random_if_below(a: usize, lim: i8, n: i32) {
    if read_i8(a) < lim {
        let r = friendly_random(n);
        write_i8(a, read_i8(a).wrapping_add(r as i8));
    }
}

/// Subtracts random(n) from the byte.
pub(crate) unsafe fn subtract_random(a: usize, n: i32) {
    let r = friendly_random(n);
    write_i8(a, read_i8(a).wrapping_sub(r as i8));
}

/// If current ability (+5) is above teto and random(n) != 0, sets it to teto.
pub(crate) unsafe fn cap_current_ability(f: PlayerSheet, ceiling: i16, n: i32) {
    if f.current_ability() > ceiling && friendly_random(n) != 0 {
        f.set_current_ability(ceiling);
    }
}

/// Rebuilds a retired person from the regen queue as a young player: player record, nation, names, languages, age and
/// ability, reputations, birth date, physical and mental attributes and, by chance, a club.
///
/// `PLAYER_REGEN::regenerate_as_youth_player(p)`, from the regen queue (`process_regen_queue`) for every young regen. A
/// person not retired (bits 0x30 of `staff_state::STATUS_FLAGS` of the 79-byte record) or with a club (`club_job()`):
/// it only clears the bits. Otherwise: leaves the national team (`national_teams_remove_player`), takes the front
/// record of the player record pool (`player_regen_fields::PLAYER_DATA_POOL`, 16-byte entries: record, nation
/// `regen_pool_item::NATION_ID`, second nation `regen_pool_item::SECOND_NATION_ID`;
/// `exe::player_regen_remove_player_data_from_pool`), becomes a player (`set_classification(2)`, `set_player` the
/// record) and the 79-byte record gets the regen mark (`staff_state_status_flags::REGEN_MARK`) and loses the others;
/// the person's lists and histories are cleared (fog, statistics, history, suspensions, injuries, dispute, discipline,
/// fitness, contracts); the record's nation (or, if its `nation::NUMBER_STAFF` is below 100, with 1/4, any one with
/// `number_staff()` below 100), the second nation (Spain `KEY_NATION_SPAIN` with 1/100 keeps the previous one; another
/// one other than the Basque Country `KEY_NATION_PAYS_BASQUE` with 2/3 none), the nation's names
/// (`exe::database_pick_first_name_for_nation`, `exe::database_pick_second_name_for_nation`,
/// `exe::database_get_empty_common_name`), the languages (all removed, `exe::playerstats_remove_language`; the nation's
/// by weight, `exe::player_stats_get_native_language_points`, level (weight − (20 − `adaptability()`) × 10) / 75 up to
/// 10, `exe::playerstats_add_language`), the joining date today, `job_for_club()`/`job_for_nation()` = 11
/// (`job_kind::PLAYER`). The age and ability: a small nation (`number_staff()` below 350, `nation::LEAGUE_SELECTED`
/// zero) by the `number_staff()` (23..27 or 17..23 years) and by the PA; otherwise by the record's position and the age
/// by random(10000) (15..18 rare, 16..18, 17..20, 21..26). The reputations (`home_reputation()`,
/// `current_reputation()`, `world_reputation()`), the ability caps, the birth year and day, the age
/// (`exe::database_get_staff_age`), the physical and mental attributes (`player_sheet::ACCELERATION`,
/// `player_sheet::PLAYER_PACE`, `player_sheet::AGILITY`, `player_sheet::JUMPING`, `player_sheet::NATURAL_FITNESS`,
/// `player_sheet::WORK_RATE`, `player_sheet::STAMINA`, `player_sheet::STRENGTH`, `player_sheet::ANTICIPATION`,
/// `player_sheet::DECISIONS`, `player_sheet::IMPORTANT_MATCHES`, `player_sheet::LEADERSHIP`,
/// `player_sheet::CONSISTENCY`) by age; under 19 and a nation with `nation::LEAGUE_SELECTED`, with 3/4, the best club
/// (`exe::player_regen_pick_best_club_for_player`) and the contract (`FN_PLAYER_REGEN_SIGN_FOR_CLUB`, news
/// `exe::player_regen_post_youth_promotion_news` at a human club; with a potential (`player_sheet::POTENTIAL_ABILITY`)
/// above 80, the chairman (`club::CHAIRMAN`) favours him when draw(100) is below his `interference()`,
/// `exe::staff_add_favoured_staff`); otherwise, by chance, a club of the nation
/// (`exe::player_regen_pick_club_from_nation_list`). Without a club and under 20, the reputations up to 1250 and
/// `world_reputation()` = 1. At the end, the attributes normalised (`exe::staff_normalise_player_physical_attributes`,
/// `exe::staff_normalise_player_attributes`).
/// Original: `PLAYER_REGEN::regenerate_as_youth_player` (0x7abac0, player_regen.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn regenerate_as_youth_player(this: usize, p: usize) {
    let p = Staff(p);
    let mark = staff_state_byte(p, staff_state::STATUS_FLAGS);
    let b = *mark;
    if b & 0x30 == 0 || !p.club_job().is_null() {
        *mark = b & 0xcf;
        return;
    }
    call_thiscall_1(0x775ba0, read_usize(G_NATIONAL_TEAMS), p);
    let bench = read_usize(this + player_regen_fields::PLAYER_DATA_POOL);
    let (ficha_id, nacao_id, nacao2_id) = (read_i32(bench), read_i32(bench + regen_pool_item::NATION_ID), read_i32(bench + regen_pool_item::SECOND_NATION_ID));
    exe::player_regen_remove_player_data_from_pool(this, ficha_id);
    p.set_classification(2);
    p.set_player(PlayerSheet(read_usize(G_PLAYER_DATA_TABLE).wrapping_add((ficha_id as isize * 0x46) as usize)));
    *staff_state_byte(p, staff_state::STATUS_FLAGS) |= staff_state_status_flags::REGEN_MARK;
    *staff_state_byte(p, 0xd) = 0;
    *staff_state_byte(p, 0xc) &= 0xfd;
    *staff_state_byte(p, staff_state::STATUS_FLAGS) &= 0x7f;
    *staff_state_byte(p, 0xc) &= 0xfe;
    *staff_state_byte(p, 9) = 0x9c;
    if read_usize(G_FOG_OF_WAR) != 0 && *(G_ATTRIBUTE_MASKING_ENABLED as *const u8) != 0 {
        call_thiscall_1(0x5afd00, read_usize(G_FOG_OF_WAR), p);
    }
    call_thiscall_1(0x7c6650, read_usize(G_PLAYER_STATS), p);
    call_thiscall_1(0x86fb80, read_usize(G_STAFF_RECORDS), p);
    call_thiscall_1(0x55e1d0, read_usize(G_DISCIPLINE), p);
    call_thiscall_1(0x631f90, read_usize(G_INJURY), p);
    call_thiscall_1(0x56a090, read_usize(G_DISPUTE), p);
    call_thiscall_1(0x556830, read_usize(G_DISCIPLINE), Staff(read_usize(p.addr())));
    call_thiscall_1(0x62bfa0, read_usize(G_INJURY), Staff(read_usize(p.addr())));
    call_thiscall_1(0x4d0b50, read_usize(G_CONTRACT_MANAGER), p);
    call_thiscall_1(0x4d0bd0, read_usize(G_CONTRACT_MANAGER), p);
    p.set_int_apps(0);
    p.set_int_goals(0);
    if !p.non_player().is_null() {
        p.set_non_player(NonPlayer((0) as usize));
    }
    *staff_state_byte(p, staff_state::STATUS_FLAGS) &= 0xf0;
    *staff_state_byte(p, 8) = 0;
    *staff_state_byte(p, 9) = 0x9c;
    let nations = read_usize(G_NATION_TABLE);
    let nation = nations.wrapping_add((nacao_id as isize * 0x122) as usize);
    p.set_nation(Nation(nation));
    if read_i32(nation + nation::NUMBER_STAFF) < 100 && friendly_random(4) == 0 {
        let k = friendly_random(read_i32(G_NATION_COUNT));
        let other = Nation(read_usize(G_NATION_TABLE).wrapping_add((k as isize * 0x122) as usize));
        if other.continent() != 0 && other.number_staff() < 100 {
            p.set_nation(other);
        }
    }
    if nacao2_id > 0 && nacao2_id < read_i32(G_NATION_COUNT) {
        if !(nacao_id == read_i32(KEY_NATION_SPAIN) && friendly_random(100) == 0) {
            if nacao2_id != read_i32(KEY_NATION_PAYS_BASQUE) && friendly_random(3) != 0 {
                p.set_second_nation(Nation((0) as usize));
            } else {
                p.set_second_nation(Nation(read_usize(G_NATION_TABLE).wrapping_add((nacao2_id as isize * 0x122) as usize)));
            }
        }
    } else {
        p.set_second_nation(Nation((0) as usize));
    }
    let v = exe::database_pick_first_name_for_nation(p.nation());
    p.set_first_name(v);
    let v = exe::database_pick_second_name_for_nation(p.nation());
    p.set_second_name(v);
    let v = exe::database_get_empty_common_name(p.nation());
    p.set_common_name(v);
    call_thiscall_1(0x5fff30, read_usize(G_HUMAN_MANAGER), p);
    p.set_date_joined_club_year(read_i16(G_CURRENT_DATE_YEAR) as u16);
    write_i16(p.addr() + staff::DATE_JOINED_CLUB, read_i16(G_CURRENT_DATE));
    p.set_club_job(Club((0) as usize));
    p.set_national_job(Club((0) as usize));
    p.set_wage((0) as usize);
    p.set_value((0) as usize);
    // the languages: the original pushes the argument’s dword with the low byte replaced (the calls read the byte)
    for k in 1..=0x40i8 {
        exe::playerstats_remove_language(read_usize(G_PLAYER_STATS), p, k);
    }
    let base = (0x14i16.wrapping_sub(p.adaptability() as i16)).wrapping_mul(10);
    for k in 1..=0x40i8 {
        let v = exe::player_stats_get_native_language_points(p, k);
        if v != 0 && v >= base {
            let q = (v as i32 - base as i32) / 0x4b;
            // simplification: the fluency goes in a stack dword with only the low byte written (0x7c6a30 reads the byte)
            let level = if q > 0xa { 0xa } else { q as u8 };
            exe::playerstats_add_language(read_usize(G_PLAYER_STATS), p, k as u8 as u32, level as u32, 1);
        }
    }
    p.set_job_for_club(job_kind::PLAYER);
    p.set_job_for_nation(job_kind::PLAYER);
    *staff_state_byte(p, staff_state::STATUS_FLAGS) &= 0xcf;
    let f = p.player();
    let nation = p.nation();
    let age: i8;
    if !nation.is_null() && *((nation.addr() + nation::LEAGUE_SELECTED) as *const u8) == 0 && nation.number_staff() < 0x15e {
        // the small nation: age by its reputation and ability by PA
        let r1 = friendly_random(nation.number_staff());
        let r2 = friendly_random(1000);
        age = if r1 < r2 {
            (friendly_random(5) as i8).wrapping_add(0x17)
        } else {
            (friendly_random(7) as i8).wrapping_add(0x11)
        };
        covered!("cqg_7 small nation");
        if age > 0x14 {
            roll_current_ability(f, 0x96a5d8, 0x9689e0);
        } else if age > 0x1a {
            roll_current_ability(f, 0x968918, 0x968e48);
        } else if age > 0x17 {
            roll_current_ability(f, 0x968fb0, 0x96a688);
        } else {
            roll_current_ability(f, 0x968e48, 0x9689e0);
        }
        let r = friendly_random(0x2ee);
        f.set_home_reputation((r as i16).wrapping_add(f.current_ability().wrapping_mul(0x1e)).wrapping_add(0xfa));
        let mut l121 = age <= 0x12;
        if !l121 && f.potential_ability() > 0x8c && f.current_ability() > 0x64 && f.home_reputation() < 0x1964 && age < 0x16 && friendly_random(4) != 0 {
            f.set_home_reputation(mul_rdata_const(f.home_reputation(), 0x968978));
        }
        if age <= 0x12 {
            l121 = true;
        }
        if l121 {
            let a = f.current_ability();
            let r = friendly_random(0xc8) as i16;
            let v = if a < 0x8c {
                r.wrapping_add(f.current_ability().wrapping_mul(5)).wrapping_add(1)
            } else {
                r.wrapping_add(f.current_ability().wrapping_add(0x19).wrapping_mul(10))
            };
            f.set_world_reputation(v);
        } else {
            let a = f.current_ability();
            if a < 0x78 {
                let r = friendly_random(0xc8) as i16;
                f.set_world_reputation(r.wrapping_add(f.current_ability().wrapping_mul(0xf)).wrapping_add(1));
            } else if age > 0x15 && a > 0x78 {
                let r = friendly_random(0xc8) as i16;
                f.set_world_reputation(r.wrapping_add(f.current_ability().wrapping_add(0xa).wrapping_mul(0x19)));
            } else {
                let r = friendly_random(0xc8) as i16;
                let sum = if a < 0x8c { 1 } else { 0xfa };
                f.set_world_reputation(r.wrapping_add(f.current_ability().wrapping_mul(0x14)).wrapping_add(sum));
            }
        }
        if f.home_reputation() < 0x8ca {
            let r = friendly_random(5);
            f.set_world_reputation((r as i16).wrapping_add(1));
        }
        if f.home_reputation() < f.world_reputation() {
            f.set_home_reputation(f.world_reputation().wrapping_add(0xa));
            f.set_world_reputation(1);
        }
        let r = friendly_random(0x32);
        f.set_current_reputation((r as i16).wrapping_add(f.home_reputation()));
        if age > 0x17 {
            if f.home_reputation() < 0x1770 {
                f.set_home_reputation(mul_rdata_const(f.home_reputation(), 0x969588));
            }
            if f.current_reputation() < 0x1770 {
                f.set_current_reputation(mul_rdata_const(f.current_reputation(), 0x969588));
            }
            let a = f.current_ability();
            if a < mul_rdata_const(a, 0x96a5d0) && friendly_random(4) != 0 {
                f.set_current_ability(mul_rdata_const(f.current_ability(), 0x96a5d0));
            }
        } else if age < 0x14 {
            if f.current_ability() < mul_rdata_const(f.potential_ability(), 0x968968) && friendly_random(4) != 0 {
                f.set_current_ability(mul_rdata_const(f.potential_ability(), 0x968968));
            }
            if age < 0x12 {
                cap_current_ability(f, 0x78, 0x96);
            } else {
                cap_current_ability(f, 0x78, 0x32);
            }
        } else {
            if f.home_reputation() < 0x1676 {
                f.set_home_reputation(mul_rdata_const(f.home_reputation(), 0x968928));
            }
            if f.current_reputation() < 0x1676 {
                f.set_current_reputation(mul_rdata_const(f.current_reputation(), 0x968928));
            }
            if f.current_ability() < mul_rdata_const(f.potential_ability(), 0x968e48) && friendly_random(4) != 0 {
                f.set_current_ability(mul_rdata_const(f.potential_ability(), 0x968e48));
            }
            cap_current_ability(f, 0x8c, 0x32);
        }
    } else {
        // the big nation or no nation: the ability by position and the age by random roll(10000)
        if (f.attacker() as u8) == 0x14 {
            roll_current_ability(f, 0x968e48, 0x968e80);
        } else if (f.defender() as u8) == 0x14 || (f.sweeper() as u8) == 0x14 {
            roll_current_ability(f, 0x96a5d8, 0x9689e0);
        } else if (f.goalkeeper() as u8) == 0x14 {
            roll_current_ability(f, 0x968968, 0x9689e0);
        } else {
            roll_current_ability(f, 0x968e48, 0x9689e0);
        }
        if friendly_random(0xfa) != 0 {
            let r = friendly_random(0x2ee) as i16;
            f.set_home_reputation(r.wrapping_add(f.current_ability().wrapping_mul(0x14)).wrapping_add(0xfa)); // cqg7: the +9 reputation of the regen
            covered!("cqg_7 regen injury_word_0c");
            let r = friendly_random(0x14) as i16;
            f.set_world_reputation(r.wrapping_add(f.current_ability().wrapping_mul(5)).wrapping_add(1));
        } else {
            let r = friendly_random(0x2ee) as i16;
            f.set_home_reputation(r.wrapping_add(f.current_ability().wrapping_mul(0xf)).wrapping_add(0x7d));
            f.set_world_reputation(1);
        }
        if f.home_reputation() < f.world_reputation() {
            f.set_home_reputation(f.world_reputation().wrapping_add(0xa));
            f.set_world_reputation(1);
        }
        let r = friendly_random(0x32);
        f.set_current_reputation((r as i16).wrapping_add(f.home_reputation()));
        let s = friendly_random(0x2710);
        covered!("cqg_7 big nation");
        if s == 0 {
            age = (friendly_random(4) as i8).wrapping_add(0xf);
            if age == 0xf {
                cap_current_ability(f, 0x50, 0xfa);
            } else {
                cap_current_ability(f, 0x64, 0x96);
            }
        } else if s < 0x1388 {
            age = (friendly_random(3) as i8).wrapping_add(0x10); // cqg7: the age from 16 to 18
            covered!("cqg_7 age 16 to 18");
            if age < 0x12 {
                cap_current_ability(f, 0x50, 0xfa);
            } else {
                cap_current_ability(f, 0x64, 0x96);
            }
        } else if s > 0x26f7 {
            age = (friendly_random(6) as i8).wrapping_add(0x15);
        } else {
            age = (friendly_random(4) as i8).wrapping_add(0x11);
            if age == 0x11 {
                cap_current_ability(f, 0x5a, 0xfa);
            } else {
                cap_current_ability(f, 0x6e, 0x96);
            }
        }
    }
    if exe::nation_is_major_nation(p.nation()) == 0 {
        if age < 0x19 && f.current_ability() < 0x78 {
            f.set_world_reputation(1);
        } else {
            f.set_world_reputation(mul_rdata_const(f.world_reputation(), 0x96a5d0));
        }
    }
    p.set_date_of_birth_year((read_i16(G_CURRENT_DATE_YEAR).wrapping_sub(age as i16)) as u16); // cqg7: the birth year
    let r = friendly_random(0x16d);
    p.set_date_of_birth(r as i16);
    let a = exe::database_get_staff_age(p, G_CURRENT_DATE as *const u16);
    p.set_age(a as u8 as i8 as i16);
    // +0x1b
    if f.potential_ability() > 0x64 && (f.attacker() >= 0xf || f.attacking_midfielder() >= 0xf) {
        if friendly_random(3) != 0 {
            let r = friendly_random(0xb);
            f.set_acceleration((r as i8).wrapping_add(0xa));
        } else if friendly_random(3) == 0 {
            let r = friendly_random(7);
            f.set_acceleration((r as i8).wrapping_add(0xe));
        } else {
            let r = friendly_random(0x10);
            f.set_acceleration((r as i8).wrapping_add(5));
        }
    } else if friendly_random(4) != 0 {
        let r = friendly_random(0xa);
        f.set_acceleration((r as i8).wrapping_add(8));
    } else {
        let r = friendly_random(0x10);
        f.set_acceleration((r as i8).wrapping_add(5));
    }
    adjust_personality_attribute(f.addr() + player_sheet::ACCELERATION, 5, 5, 0xa, 0xa, 8);
    // +0x36 by +0x1b
    let a = f.acceleration();
    let v = if a > 0xf {
        (friendly_random(0xb) as i8).wrapping_add(0xa)
    } else if a > 0xa {
        (friendly_random(0xa) as i8).wrapping_add(8)
    } else if a < 5 {
        (friendly_random(0xa) as i8).wrapping_add(1)
    } else if friendly_random(4) != 0 {
        (friendly_random(0x10) as i8).wrapping_add(5)
    } else {
        (friendly_random(0xa) as i8).wrapping_add(5)
    };
    f.set_player_pace(v);
    // +0x1d (the 20-rated goalkeeper with 4/5 by PA)
    if f.goalkeeper() == 0x14 && friendly_random(5) != 0 {
        let r = friendly_random(0x14);
        let v = if f.potential_ability() as i32 > r + 0x64 {
            (friendly_random(0xb) as i8).wrapping_add(0xa)
        } else {
            (friendly_random(0x10) as i8).wrapping_add(5)
        };
        f.set_agility(v);
    } else {
        let r = friendly_random(0x14);
        f.set_agility((r as i8).wrapping_add(1));
        adjust_personality_attribute(f.addr() + player_sheet::AGILITY, 6, 5, 6, 0, 0);
    }
    // +0x2e
    let mut k: i8 = -5;
    if f.defender() >= 0xf {
        k = if f.central() >= 0xf { 5 } else { 0 };
    } else if ((f.midfielder() >= 0xf || f.defensive_midfielder() >= 0xf) && f.central() >= 0xf)
        || f.goalkeeper() >= 0xf
        || f.attacker() >= 0xf
    {
        k = 0;
    }
    let r1 = friendly_random(k as i32 * 3);
    let r2 = friendly_random(0x14);
    f.set_jumping((r1 as i8).wrapping_add(r2 as i8));
    let a = f.jumping();
    f.set_jumping(if a < 1 { 1 } else if a > 0x14 { 0x14 } else { a });
    // +0x34
    let r = friendly_random(0x14);
    f.set_natural_fitness((r as i8).wrapping_add(1));
    adjust_personality_attribute(f.addr() + player_sheet::NATURAL_FITNESS, 6, 8, 0xa, 0, 0);
    // +0x44
    let r = friendly_random(0x14);
    f.set_work_rate((r as i8).wrapping_add(1));
    if f.work_rate() > 0xf && friendly_random(3) != 0 {
        subtract_random(f.addr() + player_sheet::WORK_RATE, 5);
    } else {
        let a = f.work_rate();
        if a < 5 {
            add_random_if_below(f.addr() + player_sheet::WORK_RATE, 5, 0xa);
        } else if a < 0xa {
            add_random_if_below(f.addr() + player_sheet::WORK_RATE, 0xa, 5);
        }
    }
    // +0x3c by age
    if age < 0x12 {
        let r = friendly_random(0x14);
        f.set_stamina((r as i8).wrapping_add(1));
        adjust_personality_attribute(f.addr() + player_sheet::STAMINA, 8, 5, 8, 0xa, 6);
    } else if age < 0x18 {
        let r = friendly_random(0x12);
        f.set_stamina((r as i8).wrapping_add(3));
        adjust_personality_attribute(f.addr() + player_sheet::STAMINA, 5, 5, 0xa, 0xa, 5);
    } else {
        let r = friendly_random(0xf);
        f.set_stamina((r as i8).wrapping_add(6));
        adjust_personality_attribute(f.addr() + player_sheet::STAMINA, 3, 0xa, 8, 0, 0);
    }
    if f.stamina() < 0xa && f.work_rate() > 0xf {
        let r = friendly_random(5);
        f.set_stamina(f.stamina().wrapping_add(r as i8));
    }
    // +0x3d by age
    let r = friendly_random(0x14);
    f.set_strength((r as i8).wrapping_add(1));
    if age < 0x12 {
        adjust_personality_attribute(f.addr() + player_sheet::STRENGTH, 0xa, 5, 6, 0, 0);
    } else if age < 0x18 {
        adjust_personality_attribute(f.addr() + player_sheet::STRENGTH, 8, 5, 8, 0, 0);
    } else {
        adjust_personality_attribute(f.addr() + player_sheet::STRENGTH, 5, 5, 0xa, 0, 0);
    }
    if (f.defender() == 0x14 || f.defensive_midfielder() == 0x14) && f.strength() < 0xa {
        let r = friendly_random(5);
        f.set_strength(f.strength().wrapping_add(r as i8));
    }
    if f.jumping() > 0xf && f.strength() < 0xa {
        let r = friendly_random(5);
        f.set_strength(f.strength().wrapping_add(r as i8));
    }
    // +0x1e, +0x24, +0x2c, +0x2f (and +0x21) by age
    if age < 0x18 {
        if age < 0x12 {
            subtract_random(f.addr() + player_sheet::ANTICIPATION, 0x28);
            subtract_random(f.addr() + player_sheet::DECISIONS, 0x28);
            let a = f.important_matches();
            if a > 0xf {
                subtract_random(f.addr() + player_sheet::IMPORTANT_MATCHES, 8);
            } else if a > 0xa {
                subtract_random(f.addr() + player_sheet::IMPORTANT_MATCHES, 5);
            }
            let a = f.leadership();
            if a > 0xf {
                subtract_random(f.addr() + player_sheet::LEADERSHIP, 0xa);
            } else if a > 0xa {
                subtract_random(f.addr() + player_sheet::LEADERSHIP, 8);
            }
        } else {
            subtract_random(f.addr() + player_sheet::ANTICIPATION, 0x14);
            subtract_random(f.addr() + player_sheet::DECISIONS, 0x14);
            if f.important_matches() > 0xf {
                subtract_random(f.addr() + player_sheet::IMPORTANT_MATCHES, 5);
            }
            let a = f.leadership();
            if a > 0xf {
                subtract_random(f.addr() + player_sheet::LEADERSHIP, 8);
            } else if a > 0xa {
                subtract_random(f.addr() + player_sheet::LEADERSHIP, 5);
            }
        }
        if f.leadership() > 0xf {
            subtract_random(f.addr() + player_sheet::LEADERSHIP, 8);
        }
    } else if age > 0x19 && age < 0x1d {
        add_random_if_below(f.addr() + player_sheet::STRENGTH, 5, 8);
        add_random_if_below(f.addr() + player_sheet::CONSISTENCY, 5, 8);
        add_random_if_below(f.addr() + player_sheet::STAMINA, 5, 8);
        subtract_random(f.addr() + player_sheet::ANTICIPATION, 0xa);
        subtract_random(f.addr() + player_sheet::DECISIONS, 0xa);
        add_random_if_below(f.addr() + player_sheet::IMPORTANT_MATCHES, 5, 8);
        let a = f.leadership();
        if a > 0xf {
            subtract_random(f.addr() + player_sheet::LEADERSHIP, 5);
        } else if a > 0xa {
            subtract_random(f.addr() + player_sheet::LEADERSHIP, 3);
        }
    }
    // the club
    let nation = p.nation();
    if age < 0x13 && !nation.is_null() && *((nation.addr() + nation::LEAGUE_SELECTED) as *const u8) != 0 && friendly_random(4) != 0 {
        let contratar: unsafe extern "stdcall" fn(usize, usize) -> u8 = core::mem::transmute(FN_PLAYER_REGEN_SIGN_FOR_CLUB);
        let c = exe::player_regen_pick_best_club_for_player(this, p);
        if c != 0 && contratar(p.addr(), c) != 0 {
            covered!("cqg_7 best club");
            let human: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(FN_NEWS_CLUB_HAS_HUMAN_MANAGER);
            if human(c) != 0 {
                exe::player_regen_post_youth_promotion_news(p, c);
            }
            if read_i16(p.player().addr() + 7) > 0x50 {
                let t = Staff(read_usize(c + club::CHAIRMAN));
                if !t.is_null() && !t.non_player().is_null() {
                    let r = friendly_random(100);
                    if t.non_player().interference() as i32 > r {
                        exe::staff_add_favoured_staff(t, p, 0);
                    }
                }
            }
        }
    } else if (!nation.is_null() && *((nation.addr() + nation::LEAGUE_SELECTED) as *const u8) == 0 && friendly_random(0xf) != 0) || friendly_random(0xa) == 0 {
        let c = exe::player_regen_pick_club_from_nation_list(this, p);
        if c != 0 {
            let contratar: unsafe extern "stdcall" fn(usize, usize) -> u8 = core::mem::transmute(FN_PLAYER_REGEN_SIGN_FOR_CLUB);
            contratar(p.addr(), c);
        }
    }
    if p.club_job().is_null() && age < 0x14 {
        let f = p.player();
        if f.current_reputation() > 0x4e2 {
            f.set_current_reputation(0x4e2);
        }
        if f.home_reputation() > 0x4e2 {
            f.set_home_reputation(0x4e2);
        }
        f.set_world_reputation(1);
    }
    exe::staff_normalise_player_physical_attributes(p);
    exe::staff_normalise_player_attributes(p);
}

/// Does the person's name k (+4 first, +8 second, +0xc common) match text (up to 26 chars)? Empty text always matches.
pub(crate) unsafe fn name_matches(p: usize, k: usize, text: usize) -> bool {
    read_u8(text) == 0 || strncmp(read_usize(p + k), text, 0x1a) == 0
}

/// Finds the person of a player_setup.cfg line by the three names: among all persons or, with the club name, in its
/// squad (when asked) and its manager.
///
/// `player_setup_find_staff(first, common, second, club, in_squad)` (cdecl). Without `club` (null or empty): the first
/// of the `G_STAFF_COUNT` persons (`G_STAFF_TABLE`, 0x6e bytes) whose non-empty names match (each name checked only if
/// both the text and the person's name exist). With `club`: the club with that name (`exe::club_get_english_name`, 51
/// characters); not found, 0. With `in_squad` 1: the first of the 50 squad slots (`club::SQUAD`) whose names match;
/// otherwise (or none in the squad), the club's manager (`club::MANAGER`) if the names match; otherwise 0.
/// Original: `player_setup_find_staff` (0x7a50f0, player_regen.cpp, cdecl).
pub(crate) unsafe extern "C" fn player_setup_find_staff(first: usize, common: usize, second: usize, club: usize, in_squad: u32) -> usize {
    if club == 0 || read_u8(club) == 0 {
        let mut p = read_usize(G_STAFF_TABLE);
        for _ in 0..read_i32(G_STAFF_COUNT).max(0) {
            let falha = (read_u8(first) != 0 && read_usize(p + staff::FIRST_NAME) != 0 && strncmp(read_usize(p + staff::FIRST_NAME), first, 0x1a) != 0)
                || (read_u8(second) != 0 && read_usize(p + staff::SECOND_NAME) != 0 && strncmp(read_usize(p + staff::SECOND_NAME), second, 0x1a) != 0) // cqh9: the second name
                || (read_u8(common) != 0 && read_usize(p + staff::COMMON_NAME) != 0 && strncmp(read_usize(p + staff::COMMON_NAME), common, 0x1a) != 0);
            if !falha {
                covered!("cqh_9 person by name");
                return p;
            }
            p += 0x6e;
        }
        return 0;
    }
    let mut found = 0usize;
    for k in 0..read_i32(G_CLUB_COUNT).max(0) {
        let c = read_usize(G_CLUB_TABLE).wrapping_add(k as usize * club::SIZE);
        if strncmp(exe::club_get_english_name(c), club, 0x33) == 0 {
            found = c;
            break;
        }
    }
    if found == 0 {
        return 0;
    }
    let c = found;
    if in_squad as u8 == 1 {
        for k in 0..0x32usize {
            let q = read_usize(c + club::SQUAD + 4 * k);
            if q != 0 && name_matches(q, 4, first) && name_matches(q, 8, second) && name_matches(q, 0xc, common) {
                covered!("cqh_9 club player by name");
                return q; // cqh9: the squad one by name
            }
        }
    }
    let t = read_usize(c + club::MANAGER);
    if t == 0 {
        return 0;
    }
    if name_matches(t, 4, first) && name_matches(t, 8, second) && name_matches(t, 0xc, common) {
        return t;
    }
    0
}

/// Copies the next quoted text of the line (skipping spaces and line breaks before it) and returns where to continue.
///
/// `player_setup_read_quoted_token(source, dest)` (cdecl): copies into `dest` the text between the first quotes of
/// `source`: spaces and '\r' at the start of the text and '
///
/// ' before it are skipped; it ends at the closing quote or at the zero (the destination gets the zero) and returns the
/// pointer two past the end.
/// Original: `player_setup_read_quoted_token` (0x7a5090, player_regen.cpp, cdecl).
pub(crate) unsafe extern "C" fn player_setup_read_quoted_token(source: usize, target: usize) -> usize {
    let (mut i, mut n, mut aberto) = (0usize, 0usize, false);
    loop {
        let c = read_u8(source + i);
        i += 1;
        if c == b'"' {
            if aberto {
                break;
            }
            aberto = true;
            continue;
        }
        if (c == b' ' || c == b'\r') && n == 0 {
            continue;
        }
        if c == b'\n' {
            if n == 0 {
                continue;
            }
        } else if c == 0 {
            break;
        }
        if aberto {
            covered!("cqh_9 quoted text");
            write_u8(target + n, c); // cqh9: the text character
            n += 1;
        }
    }
    write_u8(target + n, 0);
    source + i + 1
}

/// Os tipos de linha do player_setup.cfg (o texto no .rdata e o bit), na ordem da original (strncmp de 100); o último,
/// AWOL (0xa6b2f8), dá 0x40, e nenhum, 0.
pub(crate) const PLAYER_SETUP_LINE_TYPES: [(usize, u32); 14] = [
    (0xa6b3a0, 1),      // INJURY
    (0xa6b390, 2),      // MANAGER_CONF
    (0xa6b384, 0x200),  // RETIREMENT
    (0xa6b374, 4),      // INT_RETIREMENT
    (0xa6b364, 8),      // FUTURE_TRANSFER
    (0xa6b35c, 0x10),   // LOAN
    (0xa6b350, 0x100),  // NANDROLONE
    (0xa6b348, 0x100),  // DRUGS
    (0xa6b33c, 0x1000), // MISCONDUCT
    (0xa6b32c, 0x800),  // FALSE_PASSPORT
    (0xa6b324, 0x20),   // ARSE
    (0xa6b314, 0x2000), // PRESET_TACTICS
    (0xa6b308, 0x400),  // COLOURBLIND
    (0xa6b300, 0x80),   // GERMAN
];

/// Returns the type bit of a player_setup.cfg line.
///
/// `player_setup_type_from_name(name)` (cdecl): the bit of the line type (INJURY 1, MANAGER_CONF 2, RETIREMENT 0x200,
/// INT_RETIREMENT 4, FUTURE_TRANSFER 8, LOAN 0x10, NANDROLONE and DRUGS 0x100, MISCONDUCT 0x1000, FALSE_PASSPORT 0x800,
/// ARSE 0x20, PRESET_TACTICS 0x2000, COLOURBLIND 0x400, GERMAN 0x80, AWOL 0x40; none 0), used when applying the lines
/// (0x7a3a10) on game creation.
/// Original: `player_setup_type_from_name` (0x7a3140, player_regen.cpp, cdecl).
pub(crate) unsafe extern "C" fn player_setup_type_from_name(name: usize) -> u32 {
    for &(texto, bit) in PLAYER_SETUP_LINE_TYPES.iter() {
        if strncmp(name, texto, 100) == 0 {
            covered!("cqh_9 line type");
            return bit;
        }
    }
    if strncmp(name, 0xa6b2f8, 100) == 0 { 0x40 } else { 0 }
}

/// Pointer to the retirement flags byte (+0xb) of person id's 79-byte record in [0xae2d0c].
pub(crate) unsafe fn retirement_flags_byte(id: i32) -> *mut u8 {
    read_usize(G_STAFF_STATE_TABLE).wrapping_add((id as isize * (staff_state::SIZE as isize) + (staff_state::STATUS_FLAGS as isize)) as usize) as *mut u8
}

/// The person's contract (`exe::contract_manager_get_contract`) without storing it in [0xae1080].
pub(crate) unsafe fn contract_of_uncached(p: usize) -> usize {
    exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p)
}

/// Calls PLAYER_REGEN::remove_retirement (`remove_retirement`): removes the pending retirement (later ones shift up).
pub(crate) unsafe fn player_regen_remove_retirement(this: usize, item: usize) {
    exe::player_regen_remove_retirement(this, item);
}

/// Calls a thiscall(manager [g], person) routine.
pub(crate) unsafe fn call_on_global_manager(routine: usize, g: usize, p: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(routine);
    f(read_usize(g), p);
}

/// Coach attribute from retired player: min(max(P, X) x k, cap), truncated.
pub(crate) fn coach_attribute_capped(pv: f64, x: f64, k: f64, ceiling: f64) -> u16 {
    let m = if pv > x { pv } else { x };
    let v = m * k;
    ftol(if v < ceiling { v } else { ceiling }) as u16
}

/// Sets the three coach attributes (+0xa, +8, +0xc of non-player record nj) from the player's caps squared vs his
/// record fields, scaled and capped.
pub(crate) unsafe fn set_coach_attributes(p: Staff, f: PlayerSheet, nj: NonPlayer, k: f64, tetos: [f64; 3]) {
    let r = p.int_apps() as f64;
    let q = game_pow(r, 2.0);
    nj.set_current_reputation((coach_attribute_capped(q, f.current_reputation() as f64, k, tetos[0])) as i16);
    let q = game_pow(r, 2.0);
    nj.set_home_reputation((coach_attribute_capped(q, f.home_reputation() as f64, k, tetos[1])) as i16);
    let q = game_pow(r, 2.0) * read_f64(rdata::F64_0_75);
    nj.set_world_reputation((coach_attribute_capped(q, f.world_reputation() as f64, k, tetos[2])) as i16);
}

/// A retired player without a non-player record (+0x69) may become a coach by random rolls.
///
/// random(+0x5b) above random(100); random(ftol(player sheet's +0xb + reputation³)) above random(50000), or (the
/// nation's +0x8e, via +0x1a) × reputation above random(200000) + 200000; with more than 2 player sheets in reserve
/// (this +0x20): the first (this +0x1c; records of 0x44 bytes in [0xae23c0]) becomes the +0x69 and leaves the reserve
/// (`remove_spare_non_player_record`).
///
/// Goalkeeper (player sheet's +0xf 20): +0x10/+0x11 swapped when +0x10 is larger and, with random(4) zero, +0x11 =
/// random(50) + 50; the normalisation (`staff_normalise_non_player_attributes`) and the coach attributes (0x9/7500 with the player sheet's +0xb above
/// 7500; otherwise, with random(+0x57) ≠ 0, 0.75/6000/3750; otherwise 0.5/4500/1).
pub(crate) unsafe fn maybe_become_coach(this: usize, p: Staff) {
    let r = p.int_apps();
    let ra = friendly_random(p.professionalism() as i32);
    if ra <= friendly_random(100) {
        return;
    }
    let f = p.player();
    let t = ftol(f.current_reputation() as f64 + game_pow(r as f64, 3.0));
    let r1 = friendly_random(t);
    let passes = r1 > friendly_random(50000) || { // cqd10: the random draw to become a manager
        let r3 = friendly_random(0x30d40).wrapping_add(0x30d40);
        let v = (p.nation().reputation() as i32).wrapping_mul(p.int_apps() as i32);
        v > r3
    };
    if !passes || read_i32(this + player_regen_fields::SPARE_NON_PLAYER_COUNT) <= 2 {
        return;
    }
    covered!("cqd_10 becomes coach");
    let id = read_i32(read_usize(this + player_regen_fields::SPARE_NON_PLAYER_RECORDS));
    p.set_non_player(NonPlayer((read_usize(G_NON_PLAYER_TABLE).wrapping_add((id as isize * 0x44) as usize) as u32) as usize));
    exe::player_regen_remove_spare_non_player_record(this, read_i32(read_usize(this + player_regen_fields::SPARE_NON_PLAYER_RECORDS)) as i32 as u32);
    if p.player().goalkeeper() == 0x14 {
        let nj = p.non_player();
        let (a, b) = (nj.coaching(), nj.coaching_gks());
        if a > b {
            nj.set_coaching_gks(a);
            p.non_player().set_coaching(b);
            if friendly_random(4) == 0 {
                let v = (friendly_random(0x32) as u8).wrapping_add(0x32);
                write_u8(p.non_player().addr() + non_player::COACHING_GKS, v);
            }
        }
    }
    exe::staff_normalise_non_player_attributes(p);
    let f = p.player();
    let nj = p.non_player();
    if f.current_reputation() > 0x1d4c {
        set_coach_attributes(p, f, nj, read_f64(rdata::F64_0_9), [read_f64(rdata::F64_7500), read_f64(rdata::F64_7500), read_f64(rdata::F64_5750)]);
    } else if friendly_random(p.ambition() as i32) != 0 {
        set_coach_attributes(p, f, nj, read_f64(rdata::F64_0_75), [read_f64(rdata::F64_6000), read_f64(rdata::F64_6000), read_f64(rdata::F64_3750)]);
    } else {
        set_coach_attributes(p, f, nj, read_f64(rdata::F64_0_5), [read_f64(rdata::F64_4500), read_f64(rdata::F64_4500), read_f64(rdata::F64_1)]);
    }
}

/// Handles a due retirement item for person p: undoes it for young/contracted/transfer cases, otherwise posts news,
/// leaves jobs and frees the slot.
pub(crate) unsafe fn process_due_retirement(this: usize, item: usize, p: usize, slot: &mut [u32; 3]) {
    let id = read_i32(item);
    if read_i8(p + staff::AGE) < 0x1e && read_i8(item + retirement_entry::REASON) != 8 // cqd10: the young player does not retire
        && (read_usize(p + staff::CLUB_JOB) != 0 || *retirement_flags_byte(read_i32(p)) & 0xf != 0) {
        covered!("cqd_10 retirement undone");
        player_regen_remove_retirement(this, item);
        return;
    }
    if exe::contract_manager_get_loan_contract(read_usize(G_CONTRACT_MANAGER), p) != 0 || exe::transfer_manager_is_player_under_offer(read_usize(G_TRANSFER_MANAGER), p, 0) != 0 {
        *retirement_flags_byte(id) &= !staff_state_status_flags::RETIREMENT_STATE;
        player_regen_remove_retirement(this, item);
        return;
    }
    let old = read_i32(item + retirement_entry::CLUB_ID);
    let club = read_usize(p + staff::CLUB_JOB);
    let changed = if old != -1 { club != 0 && read_i32(club) != old } else { club != 0 };
    let mut fix = true;
    if changed {
        if exe::date_days_since(G_CURRENT_DATE, read_usize(p + staff::DATE_JOINED_CLUB) as u32, read_usize(p + staff::DATE_JOINED_CLUB_LEAP_YEAR) as u32) < 0xd2 && read_i8(p + staff::AGE) < 0x26 {
            fix = false;
            let c = contract_of_uncached(p);
            if c != 0 && exe::staff_is_transfer_listed_or_requested(p, 0) == 0 && read_usize(c + contract::GRIEVANCES) == 0 {
                covered!("cqd_10 changed club and stays");
                *retirement_flags_byte(id) &= 0xcf;
                player_regen_remove_retirement(this, item);
                return;
            }
        }
    }
    if fix && read_usize(p + staff::CLUB_JOB) == 0 && read_i32(item + retirement_entry::CLUB_ID) != -1 {
        write_i32(item + retirement_entry::CLUB_ID, -1);
    }
    // simplification: the original pushes the reason and the announced mark in registers with leftover high bytes; the
    // 0x7a92e0 reads the bytes
    exe::player_regen_post_retirement_news(p, read_u8(item + retirement_entry::REASON) as u32, read_u8(item + retirement_entry::ANNOUNCED) as u32);
    exe::contract_manager_remove_pending_offers_for_staff(read_usize(G_CONTRACT_MANAGER), p);
    if read_usize(p + staff::PLAYER) != 0 && read_usize(p + staff::NON_PLAYER) != 0 {
        if contract_of_uncached(p) != 0 {
            let clear: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_STAFF_CONTRACT_CLEAR_BONUSES_AND_CLAUSES);
            clear(contract_of_uncached(p));
        }
    } else {
        covered!("cqd_10 removes the contract");
        call_on_global_manager(0x4d0b50, G_CONTRACT_MANAGER, p); // cqd10: removes the contract
    }
    call_on_global_manager(0x775ba0, G_NATIONAL_TEAMS, p);
    let sm = read_usize(read_usize(G_TRANSFER_MANAGER) + transfer_manager_fields::SHORTLIST_MANAGER);
    if sm != 0 {
        let c = read_usize(p + staff::CLUB_JOB);
        if c != 0 {
            ((exe::shortlist_manager_update_staff_status(sm, p, c, 4, 2) as usize) as u8);
        }
        exe::shortlist_manager_suspend_clubs_shortlisting_player(sm, p, 0);
        let c = read_usize(p + staff::CLUB_JOB);
        if c != 0 {
            write_u8(read_usize(sm + shortlist_manager_fields::CLUB_SHORTLISTS).wrapping_add((read_i32(c) as isize * 9 + 8) as usize), 100);
        }
    }
    let mut bench = false;
    if read_usize(p + staff::PLAYER) != 0 {
        covered!("cqd_10 leaves the squad");
        exe::player_regen_retire_from_playing_staff(p); // cqd10: sai do elenco
        if read_usize(p + staff::CLUB_JOB) == 0 {
            call_on_global_manager(0x4d0b50, G_CONTRACT_MANAGER, p);
        }
        call_on_global_manager(0x4196a0, G_AWOL, p);
        call_on_global_manager(0x4197c0, G_AWOL, p);
        call_on_global_manager(0x56a090, G_DISPUTE, p);
        if read_usize(p + staff::PLAYER) != 0 {
            if read_usize(p + staff::NON_PLAYER) != 0 {
                bench = true;
            } else {
                maybe_become_coach(this, Staff(p));
            }
        }
    }
    if !bench {
        bench = read_usize(p + staff::NON_PLAYER) != 0 && read_usize(p + staff::PLAYER) != 0;
    }
    if bench {
        // the player with a non-player record: the record goes to the regen database and he stays a non-player
        covered!("cqd_10 player to the bench");
        call_on_global_manager(0x7c6650, G_PLAYER_STATS, p);
        call_on_global_manager(0x55e1d0, G_DISCIPLINE, p);
        call_on_global_manager(0x631f90, G_INJURY, p);
        let pais = read_usize(p + staff::NATION);
        if pais == 0 {
            let n = friendly_random(read_i32(G_NATION_COUNT));
            exe::player_regen_add_player_data_to_pool(this, read_i32(p), n as u32, u32::MAX);
        } else if read_usize(p + staff::SECOND_NATION) == 0 {
            exe::player_regen_add_player_data_to_pool(this, read_i32(p), read_usize(pais) as u32, u32::MAX);
        } else {
            exe::player_regen_add_player_data_to_pool(this, read_i32(p), read_usize(pais) as u32, read_usize(read_usize(p + staff::SECOND_NATION)) as u32);
        }
        write_u32(p + staff::PLAYER, 0);
        *retirement_flags_byte(read_i32(item)) &= 0xcf;
        write_u8(p + staff::CLASSIFICATION, 1);
        if read_usize(p + staff::CLUB_JOB) != 0 {
            let c = contract_of_uncached(p);
            if c != 0 {
                exe::staff_contract_generate_initial_contract(c, p, 0);
            }
        }
        player_regen_remove_retirement(this, item);
        return;
    }
    // the slot for the regen: today (up to age 29) or 1 to 65 days later (past day 365, random roll(35) of the year
    // seguinte)
    slot[0] = read_i32(item) as u32;
    let today = read_usize(G_CURRENT_DATE) as u32;
    let (dia, ano) = if read_i8(p + staff::AGE) < 0x1e {
        (today as u16, read_i16(G_CURRENT_DATE_YEAR) as u16)
    } else {
        let v = (friendly_random(0x41) as u32).wrapping_add(today).wrapping_add(1) as u16; // cqd10: the regen day
        covered!("cqd_10 regen day drawn");
        if v as i16 <= 0x16d {
            (v, read_i16(G_CURRENT_DATE_YEAR) as u16)
        } else {
            (friendly_random(0x23) as u16, (read_i16(G_CURRENT_DATE_YEAR) as u16).wrapping_add(1))
        }
    };
    slot[1] = (slot[1] & 0xffff_0000) | dia as u32;
    slot[1] = (slot[1] & 0xffff) | (ano as u32) << 16;
    let m = retirement_flags_byte(read_i32(item));
    *m = (*m & 0xef) | 0x20;
    let nj = read_usize(p + staff::NON_PLAYER);
    if nj != 0 {
        let clear: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_STAFF_RECORDS_CLEAR_MANAGER_RECORD);
        clear(read_usize(G_STAFF_RECORDS), nj);
        let c = read_usize(p + staff::CLUB_JOB);
        let job = |p: Staff| p.job_for_club() as i8;
        let exit = if c == 0 || job(Staff(p)) == 5 {
            true
        } else {
            if exe::human_manager_is_human_club(c, 1, 1, 0, 0) != 0 && !matches!(job(Staff(p)), 1 | 3 | 2 | 4) {
                true
            } else if friendly_random(4) != 0 {
                false
            } else {
                !matches!(job(Staff(p)), 1 | 3 | 2 | 4 | 0xa | 9)
            }
        };
        if exit {
            exe::player_regen_retire_from_non_playing_staff(p);
            exe::player_regen_queue_staff_for_regen(this, slot.as_ptr());
        } else {
            covered!("cqd_10 redone as non-player");
            exe::player_regen_regenerate_as_non_player(this, p);
        }
        player_regen_remove_retirement(this, item);
        return;
    }
    exe::player_regen_queue_staff_for_regen(this, slot.as_ptr());
    player_regen_remove_retirement(this, item);
}

/// Processes the day's pending retirements: drops some, retires the others (news, contract, national team, squad),
/// possibly turns the player into a coach, stores the record in the regen pool or queues the regen slot, and announces
/// the future ones.
///
/// `PLAYER_REGEN::process_retirements()`, every day from the day loop; SEH frame with no destructor on the normal path.
/// The local slot for the regen (id and the default date `exe::date_date_54a110`, which leaves leap flag 1 in it); it
/// sorts the pending retirements (`player_regen_fields::RETIREMENTS`, `player_regen_fields::RETIREMENT_COUNT` of 26
/// bytes: +0 the person, `retirement_entry::CLUB_ID` the club or −1,
/// `retirement_entry::ANNOUNCE_DATE`/`retirement_entry::ANNOUNCE_YEAR` the announcement day and year,
/// `retirement_entry::RETIRE_DATE`/`retirement_entry::RETIRE_YEAR` those of the retirement, `retirement_entry::REASON`
/// the reason, `retirement_entry::ANNOUNCED` announced) with `player_regen_compare_retirements` and walks through them
/// (the pointer only advances on an announcement: `remove_retirement` pulls the following ones). Due (the date has
/// passed; `process_due_retirement`): a young player (up to 29) with a club or flag, except reason 8, gives up; with a
/// contract record (`contract_record`) or under offer (`is_player_under_offer`), gives up (flags & 0xcf); one who moved
/// club less than 210 days ago, up to 37 years old, with a contract, off the list and without +0x45, stays; otherwise
/// the news (`player_regen_post_retirement_news`), the offers (`remove_pending_offers_for_staff`), the contract, the
/// national team (`national_teams_remove_player`), the watch lists, the squad (`retire_from_playing_staff`), the
/// absences (`clear_awol`, `clear_missed_training`) and the dispute (`remove_dispute`); the player may become a coach
/// (attributes from reputation and record); with a player record and a non-player record, the record goes to the pool
/// (`add_player_data_to_pool`) and the new contract (0x85e960); otherwise the regen slot (`queue_staff_for_regen`) and,
/// with a non-player record, he leaves the jobs (`player_regen_retire_from_non_playing_staff`) or is rebuilt as a
/// non-player (`exe::player_regen_regenerate_as_non_player`). Not due: with the announcement date passed and not
/// announced, the announcement news (`exe::player_regen_post_retirement_announcement_news`, if flags & 0x30 are not
/// 0x10), the announced flag and flags 0x10 (`staff_state_status_flags::RETIREMENT_ANNOUNCED`); otherwise it stops. At
/// the end it sorts the regen slots (`player_regen_fields::REGEN_QUEUE`, `player_regen_fields::REGEN_QUEUE_COUNT` of 12
/// bytes) with `player_regen_compare_queue_entries`.
/// Original: `PLAYER_REGEN::process_retirements` (0x7a59e0, player_regen.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn process_retirements(this: usize) {
    let mut slot = [0u32; 3]; // the person and the date (day, year, leap)
    exe::date_date_54a110(slot.as_mut_ptr() as usize + 4);
    exe::crt_qsort(read_usize(this + player_regen_fields::RETIREMENTS), read_usize(this + player_regen_fields::RETIREMENT_COUNT) as u32, 0x1a, 0x7ae2a0);
    let n = read_i32(this + player_regen_fields::RETIREMENT_COUNT);
    let mut item = read_usize(this + player_regen_fields::RETIREMENTS);
    let mut i = 0i32;
    while i < n {
        let today = read_usize(G_CURRENT_DATE) as u32;
        let (dia, ano) = (today as u16 as i16, (today >> 16) as u16);
        let (vdia, vano) = (read_i16(item + retirement_entry::RETIRE_DATE), read_i16(item + retirement_entry::RETIRE_YEAR) as u16);
        if ano > vano || (ano == vano && dia > vdia) {
            let p = staff_at(read_i32(item) as isize).addr();
            process_due_retirement(this, item, p, &mut slot);
        } else {
            if read_u8(item + retirement_entry::ANNOUNCED) != 0 {
                break;
            }
            let (adia, aano) = (read_i16(item + retirement_entry::ANNOUNCE_DATE), read_i16(item + retirement_entry::ANNOUNCE_YEAR) as u16);
            if !(ano > aano || (ano == aano && dia > adia)) {
                break;
            }
            covered!("cqd_10 retirement announced");
            let id = read_i32(item);
            if *retirement_flags_byte(id) & 0x30 != 0x10 {
                // simplification: the original pushes the reason in edx with leftover upper bytes; 0x7aa340 reads the byte
                exe::player_regen_post_retirement_announcement_news(staff_at(id as isize).addr(), read_u8(item + retirement_entry::REASON) as u32);
            }
            let id = read_i32(item);
            write_u8(item + retirement_entry::ANNOUNCED, 1);
            let m = retirement_flags_byte(id);
            *m = (*m & 0xdf) | staff_state_status_flags::RETIREMENT_ANNOUNCED;
            item += 0x1a;
        }
        i += 1;
    }
    exe::crt_qsort(read_usize(this + player_regen_fields::REGEN_QUEUE), read_usize(this + player_regen_fields::REGEN_QUEUE_COUNT) as u32, 0xc, 0x7ae240);
}

// simplification: calls the news (they allocate and deliver), the regen as a non-player, the new contracts and the lists: the path
// is only known by running and cannot run twice. Original only; the copy carries the branches and the career proves it.

/// Posts news item 0xbba about the retirement to the managers (all, the country, the division, the club or the
/// interested clubs, by fame) and pays the club insurance when injury retires a valuable player.
///
/// `player_regen_post_retirement_news(p, reason, announced)` (stdcall, ret 0xc; `process_retirements` puts this in ecx,
/// unused; SEH frame with no destructor on the normal path). No person, a player with job 8 or 5 (`job_for_club()`), no
/// club, or a club without a human manager (`FN_NEWS_CLUB_HAS_HUMAN_MANAGER`): nothing. News 0xbba (local): 0..3 the
/// person (id and the `names::ID` of `staff::FIRST_NAME`/`staff::SECOND_NAME`/`staff::COMMON_NAME`), 4 the reason, 5
/// announced, 6 the club (−1 if none), 7 the nation (`nation()`) if he has national caps (`int_apps()` ≠ 0), otherwise
/// −1, 8 the job (0xf becomes 8, 0xd becomes 6), 9 the national caps (`int_apps()`), 0xa `job_for_nation()`, 0xb player
/// record and non-player record, 0xc the age, 0xd the non-player record, 0xe the player record. The interested clubs
/// (`exe::transfer_manager_collect_player_news_recipients` on `G_TRANSFER_MANAGER` with 0xa, up to 30, only for a
/// player). A major nation (`league_standard()` above 8) and a star (`world_reputation()` of the player record above
/// 8500, or a manager of job 5 with non-player `world_reputation()` above 8750): to everyone
/// (`exe::news_send_to_all_managers`); a lesser star (`world_reputation()` above 7500, or job 5 with non-player
/// `current_reputation()` above 7250): to the managers of the nation and of the club's nation
/// (`exe::news_send_to_nation_managers`); a manager of job 5 at a club with a division: to those of the division
/// (`exe::news_send_to_division_managers`); a non-player: to the club (`exe::news_send_to_club_manager`); a player: to
/// the interested clubs. Reason 8 (injury) with `current_reputation()` of the player record above 4250 and a contract
/// club (`contract_of_uncached`, cached in `G_CONTRACT_LOOKUP_RESULT`): the insurance
/// (`exe::news_post_insurance_payout` with `value()`).
/// Original: `player_regen_post_retirement_news` (0x7a92e0, player_regen.cpp, stdcall).
pub(crate) unsafe extern "stdcall" fn player_regen_post_retirement_news(p: usize, reason: u32, announced: u32) {
    let p = Staff(p);
    if p.is_null() {
        return;
    }
    if !p.player().is_null() && matches!(p.job_for_club() as i8 , 8 | 5) {
        return;
    }
    let club = p.club_job();
    if club.is_null() {
        return;
    }
    let human: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(FN_NEWS_CLUB_HAS_HUMAN_MANAGER);
    if human(club.addr()) == 0 {
        return;
    }
    covered!("cqd_10 retirement news");
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let nw = local.as_mut_ptr() as usize;
    exe::new_newsitem(nw, 0xbba, 0);
    exe::newsitem_set_slot(nw, 0, read_usize(p.addr()) as u32);
    for k in 1..4usize {
        exe::newsitem_set_slot(nw, k as i8, read_usize(read_usize(p.addr() + staff::FIRST_NAME * k) + names::ID) as u32);
    }
    exe::newsitem_set_slot(nw, 4, reason as u8 as i8 as i32 as u32);
    exe::newsitem_set_slot(nw, 5, announced as u8 as i8 as i32 as u32);
    let c = p.club_job();
    exe::newsitem_set_slot(nw, 6, if !c.is_null() { read_usize(c.addr()) as u32 } else { u32::MAX });
    exe::newsitem_set_slot(nw, 7, if p.int_apps() != 0 { read_usize(p.nation().addr()) as u32 } else { u32::MAX });
    let job = match p.job_for_club() as i8  {
        0xf => 8,
        0xd => 6,
        v => v,
    };
    exe::newsitem_set_slot(nw, 8, job as i32 as u32);
    exe::newsitem_set_slot(nw, 9, p.int_apps() as u32);
    exe::newsitem_set_slot(nw, 0xa, (p.job_for_nation() as i8) as i32 as u32);
    exe::newsitem_set_slot(nw, 0xb, (!p.player().is_null() && !p.non_player().is_null()) as u32);
    exe::newsitem_set_slot(nw, 0xc, (p.age() as i8) as i32 as u32);
    exe::newsitem_set_slot(nw, 0xd, (!p.non_player().is_null()) as u32);
    exe::newsitem_set_slot(nw, 0xe, (!p.player().is_null()) as u32);
    let mut clubs = [0usize; 30];
    let tm = read_usize(G_TRANSFER_MANAGER);
    let n = if tm != 0 && !p.player().is_null() {
        exe::transfer_manager_collect_player_news_recipients(tm, p, 0xa, clubs.as_mut_ptr(), 0x1e) as i8
    } else {
        0
    };
    let mn = read_usize(G_NEWS);
    let pais = p.nation();
    let f = p.player();
    let nj = p.non_player();
    let cargo5 = || (p.job_for_club() as i8) == 5;
    if !pais.is_null() && (pais.league_standard() as i8) > 8
        && ((!f.is_null() && f.world_reputation() > 0x2134) || (!nj.is_null() && nj.world_reputation() > 0x222e && cargo5())) {
        exe::news_send_to_all_managers(mn, nw);
    } else if (!f.is_null() && f.world_reputation() > 0x1d4c) || (!nj.is_null() && nj.current_reputation() > 0x1c52 && cargo5()) {
        exe::news_send_to_nation_managers(mn, nw, pais);
        let c = p.club_job();
        if !c.is_null() && c.nation() != p.nation() {
            exe::news_send_to_nation_managers(read_usize(G_NEWS), nw, c.nation().addr());
        }
    } else {
        let c = p.club_job();
        if !c.is_null() && c.division().addr() != 0 && cargo5() {
            exe::news_send_to_division_managers(mn, nw, c.division().addr());
        } else if f.is_null() {
            exe::news_send_to_club_manager(mn, nw, c);
        } else if n > 0 {
            for k in 0..n as usize {
                exe::news_send_to_club_manager(read_usize(G_NEWS), nw, clubs[k]);
            }
        }
    }
    if reason as u8 == 8 {
        let f = p.player();
        if !f.is_null() && f.current_reputation() > 0x109a {
            let c = contract_of_uncached(p.addr());
            write_usize(G_CONTRACT_LOOKUP_RESULT, c);
            if c != 0 {
                let club = club_at(read_i32(c + 4) as isize).addr();
                if club != 0 {
                    covered!("cqd_10 retirement insurance");
                    exe::news_post_insurance_payout(club, p, p.value() as u32);
                }
            }
        }
    }
}

// simplification: allocates and delivers the news item (0x77bbe0, 0x786780...). Original only; the copy carries the branch.

/// Posts news item 0xbce about a youth player joining the club, with the assistant manager's or a coach's verdict on
/// his ability.
///
/// `player_regen_post_youth_promotion_news(j, c)` (stdcall, ret 8; SEH frame with no destructor on the normal path),
/// from the young regen (`regenerate_as_youth_player`). No player or club: nothing. News 0xbce (local): 0..3 the player
/// (id and the `names::ID` of `staff::FIRST_NAME`/`staff::SECOND_NAME`/`staff::COMMON_NAME`), 4 the club id, 5 the age,
/// 6 the club's training facilities (`training()`) above 15. The assistant manager (`assistant_manager()`) and the five
/// coaches (`coaches()`) give their opinion until the first one who gives a verdict: the perceived reputation =
/// random(2500) + club reputation (`reputation()`) / 2 + training × 12.5 (`rdata::F64_12_5`, truncated); the two
/// perceived abilities (`exe::match_man_judge_player_ability` with 2 and with 1) against reputation / 50 + 75 (verdict
/// 2) and reputation / 100 + 50 (1). With a verdict: 7 assistant?, 8..0xb who (id and names), 0xc and 0xd the verdicts;
/// otherwise −1. Delivered at the club's scope (`exe::news_send_by_scope`).
/// Original: `player_regen_post_youth_promotion_news` (0x7a8ba0, player_regen.cpp, stdcall).
pub(crate) unsafe extern "stdcall" fn player_regen_post_youth_promotion_news(j: usize, c: usize) {
    let j = Staff(j);
    let c = Club(c);
    if j.is_null() || c.is_null() {
        return;
    }
    covered!("cqd_10 promoted youngster news");
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let nw = local.as_mut_ptr() as usize;
    let (mut v1, mut v2) = (0i8, 0i8);
    let mut who = 0usize;
    exe::new_newsitem(nw, 0xbce, 0);
    exe::newsitem_set_slot(nw, 0, read_usize(j.addr()) as u32);
    for k in 1..4usize {
        exe::newsitem_set_slot(nw, k as i8, read_usize(read_usize(j.addr() + staff::FIRST_NAME * k) + names::ID) as u32);
    }
    exe::newsitem_set_slot(nw, 4, read_usize(c.addr()) as u32);
    exe::newsitem_set_slot(nw, 5, (j.age() as i8) as i32 as u32);
    exe::newsitem_set_slot(nw, 6, (c.training() > 0xf) as u32);
    let mut current = 0usize;
    let mut k = -1i8;
    loop {
        if current != 0 {
            break;
        }
        let t = if k == -1 { c.assistant_manager() } else { c.coaches(k as usize) };
        if t != 0 {
            let r = friendly_random(0x9c4);
            let v = r.wrapping_add(c.reputation() as i32 / 2);
            let x = ftol(v as f64 + c.training() as f64 * read_f64(rdata::F64_12_5));
            // simplification: the original pushes the whole eax of _ftol and that of the first view; 0x6e86d0 reads the word
            let a = exe::match_man_judge_player_ability(t, j, c, 2, x as i16);
            let b = exe::match_man_judge_player_ability(t, j, c, 1, a);
            let rep = c.reputation() as i32;
            let high = rep / 50 + 0x4b;
            if a as i32 > high {
                v1 = 2;
                who = t;
            } else if a as i32 > rep / 100 + 0x32 {
                v1 = 1;
                who = t;
            }
            if b as i32 > high {
                v2 = 2;
                who = t;
            } else if b as i32 > rep / 100 + 0x32 {
                v2 = 1;
                who = t;
            }
            current = who;
        }
        k += 1;
        if k >= 5 {
            break;
        }
    }
    if current != 0 {
        exe::newsitem_set_slot(nw, 7, (current == c.assistant_manager()) as u32);
        exe::newsitem_set_slot(nw, 8, read_usize(current) as u32);
        for k in 1..4usize {
            exe::newsitem_set_slot(nw, 8 + k as i8, read_usize(read_usize(current + 4 * k) + names::ID) as u32);
        }
        exe::newsitem_set_slot(nw, 0xc, v1 as i32 as u32);
        exe::newsitem_set_slot(nw, 0xd, v2 as i32 as u32);
    } else {
        for k in 7..14i8 {
            exe::newsitem_set_slot(nw, k, u32::MAX);
        }
    }
    exe::news_send_by_scope(read_usize(G_NEWS), nw, c, 0, 0, 0, 0);
}

// simplification: allocates and delivers the news and rolls the opinion: original only; the copy takes the branch.

/// First free (zero) slot of the club's 50 squad slots (+0xd7); 0x32 if none.
pub(crate) unsafe fn first_free_squad_slot(club: Club) -> usize {
    (0..0x32usize).find(|&k| (club.squad(k) as i32) == 0).unwrap_or(0x32)
}

/// Signs the regen for a club: the first free squad slot, the value, the contract end, the contract and the default
/// wage; returns 1 (0 without a slot).
///
/// `player_regen_sign_for_club(person, club)` (stdcall, ret 8; SEH frame only for the local dates), from
/// PLAYER_REGEN::regenerate_as_youth_player (`regenerate_as_youth_player`) for every young regen: in the first free
/// slot of the 50 in the squad (`first_free_squad_slot`, `set_squad`; no slot, 0) the person, her club (`set_club_job`)
/// and `set_wage(0)`; the value (`set_value`) through `exe::contract_manager_calc_player_value` (club, 1, −1, 0), the
/// squad (`exe::staff_set_squad_flags` with 1), one time in four (random(4) = 0, `exe::utils_random`)
/// `exe::staff_add_favoured_club` (person, club, 0); a club with `professional_status()` = 3
/// (`professional_status::AMATEUR`): value at most 5000. The contract end (`staff::DATE_EXPIRES_CLUB`): under 19
/// (`age()`), today's year + 2; otherwise + random(6) + 1; in the month of the club's season end
/// (`FN_CONTRACT_MANAGER_SEASON_END_MONTH`) and on day random(28) + 1 (`exe::date_date_549f70`). The contract
/// (`exe::contract_manager_add_staff_contract` with the value); a human club (`exe::human_manager_is_human_club` with
/// 1, 1): for the reserves (`exe::contract_manager_move_to_reserves`) and the contract bits `contract::SQUAD_STATUS`
/// (0x60 high) and `contract::CONTRACT_TYPE` (1 low). With a contract: the default wage
/// (`exe::staff_contract_set_default_wage`), `contract::SQUAD_NUMBER` = 0 and, outside the human club, the end
/// (`contract::CONTRACT_EXPIRES_YEAR`) at most today's year + 5 (with day `contract::CONTRACT_EXPIRES` = 0). At the end
/// the squad numbers (`G_SQUAD_MANAGER`, `exe::squad_manager_club_uses_squad_numbers` and
/// `exe::squad_manager_allocate_squad_number`). Returns 1.
/// Original: `player_regen_sign_for_club` (0x7a6f10, player_regen.cpp, stdcall, ret 8).
pub(crate) unsafe extern "stdcall" fn player_regen_sign_for_club(person: usize, club: usize) -> u8 {
    let person = Staff(person);
    let club = Club(club);
    let k = first_free_squad_slot(club);
    if k == 0x32 {
        return 0;
    }
    let manager = read_i32(G_CONTRACT_MANAGER) as usize;
    let month_of_end: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(FN_CONTRACT_MANAGER_SEASON_END_MONTH);
    let contract: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    covered!("cpv_9 regen signed");
    club.set_squad(k, (person.addr() as i32) as usize);
    person.set_club_job(Club((club.addr() as i32) as usize));
    person.set_wage((0) as usize);
    let v = exe::contract_manager_calc_player_value(manager, person, club, 1, -1, 0);
    person.set_value((v) as usize); // cpv9: the regen value
    exe::staff_set_squad_flags(person, 1);
    if exe::utils_random(4) == 0 { // cpv9: once in four
        covered!("cpv_9 one in four");
        exe::staff_add_favoured_club(person, club, 0);
    }
    if club.professional_status() == professional_status::AMATEUR && (person.value() as i32) > 0x1388 {
        person.set_value((0x1388) as usize);
    }
    // simplification: the original writes the year in the low word of the `clube` argument and pushes the dword; 0x549f70 reads 16 bits
    let year = read_u16(G_CURRENT_DATE_YEAR) as u32;
    let (mut d, mut ret) = ([0u8; 8], [0u8; 8]);
    let day_and_year = if (person.age() as i8) < 0x13 {
        covered!("cpv_9 youngster contract");
        let a = year.wrapping_add(2); // cpv9: o jovem por dois anos
        let month = month_of_end(club.addr());
        (exe::utils_random(0x1c), month, a)
    } else {
        let a = (exe::utils_random(6) as u32).wrapping_add(year).wrapping_add(1);
        let month = month_of_end(club.addr());
        (exe::utils_random(0x1c), month, a)
    };
    let day = (day_and_year.0 as u8).wrapping_add(1) as i8 as i16 as u16 as u32;
    let p = exe::date_date_549f70((d.as_mut_ptr()) as *mut [u16; 4], day, day_and_year.1, day_and_year.2, 0xffff_ffff) as *mut u8;
    (exe::date_operator_3d(person.addr() + staff::DATE_EXPIRES_CLUB, (ret.as_mut_ptr()) as usize, (p) as usize) as *mut u8);
    exe::contract_manager_add_staff_contract(manager, person, (person.value() as i32) as u32);
    if exe::human_manager_is_human_club(club, 1, 1, 0, 0) != 0 {
        exe::contract_manager_move_to_reserves(read_i32(G_CONTRACT_MANAGER) as usize, person, club);
        let c = contract(read_i32(G_CONTRACT_MANAGER) as usize, person.addr());
        write_u8(c + contract::SQUAD_STATUS, read_u8(c + contract::SQUAD_STATUS) & 0xf | 0x60);
        let c = contract(read_i32(G_CONTRACT_MANAGER) as usize, person.addr());
        write_u8(c + contract::CONTRACT_TYPE, read_u8(c + contract::CONTRACT_TYPE) & 0xc1 | 1);
    }
    let c = contract(read_i32(G_CONTRACT_MANAGER) as usize, person.addr());
    if c != 0 {
        exe::staff_contract_set_default_wage(c, person);
        write_u8(c + contract::SQUAD_NUMBER, 0);
        if exe::human_manager_is_human_club(club, 1, 1, 0, 0) == 0 {
            let a = read_u16(G_CURRENT_DATE_YEAR).wrapping_add(5);
            if read_u16(c + contract::CONTRACT_EXPIRES_YEAR) > a {
                write_u16(c + contract::CONTRACT_EXPIRES_YEAR, a);
                write_u16(c + contract::CONTRACT_EXPIRES, 0);
            }
        }
    }
    if exe::squad_manager_club_uses_squad_numbers(read_i32(G_SQUAD_MANAGER) as usize, club, (0) as u32) != 0 {
        exe::squad_manager_allocate_squad_number(read_i32(G_SQUAD_MANAGER) as usize, club, 0);
    }
    1
}

/// Replaces the person's pending retirement with a new one through injury (reason 8), random(days / 2) + 1 days ahead.
///
/// `PLAYER_REGEN::schedule_injury_retirement(person, days)`, from the new injury (`add_injury`) that ends the career;
/// SEH frame only for the local dates. It removes the person's pending retirement (the first with her id in the 26-byte
/// table `player_regen_fields::RETIREMENTS`, count `player_regen_fields::RETIREMENT_COUNT`;
/// `exe::player_regen_remove_retirement`) and adds (`exe::player_regen_add_retirement`) the new one: id, club id
/// (`club_job()`, or −1), today's date + random(days / 2) + 1 (`exe::utils_random`, `exe::date_add_days`) at +8 and
/// again at +0x10, reason 8 (+0x18) and 0 (+0x19).
/// Original: `PLAYER_REGEN::schedule_injury_retirement` (0x7a7f20, player_regen.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn schedule_injury_retirement(this: usize, person: usize, days: i16) {
    let person = Staff(person);
    let n = read_i32(this + player_regen_fields::RETIREMENT_COUNT);
    if !person.is_null() && n > 0 {
        let base = read_i32(this + player_regen_fields::RETIREMENTS) as usize;
        let id = read_i32(person.addr());
        if let Some(i) = (0..n as usize).find(|&i| read_i32(base + 0x1a * i) == id) {
            covered!("cpv_9 removes the pending retirement");
            exe::player_regen_remove_retirement(this, base + 0x1a * i);
        }
    }
    let mut r = [0u8; 0x1a];
    let club = (person.club_job().addr() as i32) as usize;
    let rp = r.as_mut_ptr();
    core::ptr::write_unaligned(rp as *mut i32, read_i32(person.addr()));
    core::ptr::write_unaligned(rp.add(4) as *mut i32, if club == 0 { -1 } else { read_i32(club) });
    let q = exe::utils_random(days as i32 / 2);
    let (mut t, mut t2) = ([0u8; 8], [0u8; 8]);
    let d = exe::date_add_days(G_CURRENT_DATE, (t.as_mut_ptr()) as usize, q.wrapping_add(1) as u32) as *mut u8;
    (exe::date_operator_3d((rp.add(8)) as usize, (t2.as_mut_ptr()) as usize, (d) as usize) as *mut u8);
    (exe::date_operator_3d((rp.add(0x10)) as usize, (t2.as_mut_ptr()) as usize, (rp.add(8)) as usize) as *mut u8);
    r[0x18] = 8;
    r[0x19] = 0;
    exe::player_regen_add_retirement(this, r.as_ptr());
}

/// Builds news 0xbbb (id, names, reason, contract club, age) and sends it to the managers of the interested clubs.
///
/// `player_regen_post_retirement_announcement_news(person, reason)` (stdcall, ret 8; SEH frame without destructor),
/// from PLAYER_REGEN::process_retirements (`process_retirements`): except for a player (`player()`) with job
/// `job_for_club()` 8 or 5, news 0xbbb in a local (`exe::new_newsitem` with 0) with data 0 = id, 1 to 3 = the names
/// (`names::ID` of +4, +8 and +0xc), 4 = reason, 5 = the id of the contract club (`exe::contract_manager_get_contract`,
/// cached in `G_CONTRACT_LOOKUP_RESULT`; club of the 0x245-byte record (`club_at`) by `club()`; a contract of type
/// `contract_type()` & 0x3f = 5 or no club: −1) and 6 = the age (`age()`); it goes to the managers of up to 30
/// interested clubs (`G_TRANSFER_MANAGER`, `exe::transfer_manager_collect_player_news_recipients` with 10;
/// `exe::news_send_to_club_manager`).
/// Original: `player_regen_post_retirement_announcement_news` (0x7aa340, player_regen.cpp, stdcall, ret 8).
pub(crate) unsafe extern "stdcall" fn player_regen_post_retirement_announcement_news(person: usize, reason: i8) {
    let person = Staff(person);
    if person.is_null() {
        return;
    }
    if (person.player().addr() as i32) != 0 && matches!(person.job_for_club(), 8 | 5) {
        return;
    }
    covered!("player_regen_post_retirement_announcement_news");
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0xbbb, 0);
    exe::newsitem_set_slot(n, 0, read_i32(person.addr()) as u32);
    for (k, o) in [(1i8, 4usize), (2, 8), (3, 0xc)] {
        exe::newsitem_set_slot(n, k, read_i32(read_i32(person.addr() + o) as usize + names::ID) as u32);
    }
    exe::newsitem_set_slot(n, 4, reason as i32 as u32);
    let c = Contract(exe::contract_manager_get_contract(read_i32(G_CONTRACT_MANAGER) as usize, person));
    write_i32(G_CONTRACT_LOOKUP_RESULT, c.addr() as i32);
    let mut club = -1i32;
    if !c.is_null() && c.contract_type() & 0x3f != 5 {
        let idx = (c.club() as i32) as u32;
        let record = club_at(idx as isize).addr();
        if record != 0 {
            club = read_i32(record);
        }
    }
    exe::newsitem_set_slot(n, 5, club as u32);
    exe::newsitem_set_slot(n, 6, (person.age() as i8) as i32 as u32);
    let mut list = [0usize; 0x1e];
    let q = if read_i32(G_TRANSFER_MANAGER) != 0 { exe::transfer_manager_collect_player_news_recipients(read_i32(G_TRANSFER_MANAGER) as usize, person, 0xa, list.as_mut_ptr(), 0x1e) } else { 0 };
    for i in 0..(q as i8).max(0) as usize {
        exe::news_send_to_club_manager(read_i32(G_NEWS) as usize, n, list[i]);
    }
}

/// Appends to the pool of free player records the player record id of a person who stops playing, with the nation and
/// second nation, growing the table by 250.
///
/// `PLAYER_REGEN::add_player_data_to_pool(person, nation, second)`: appends to the free player pool (16 bytes at
/// `player_regen_fields::PLAYER_DATA_POOL`, count `player_regen_fields::PLAYER_DATA_POOL_COUNT`, capacity
/// `player_regen_fields::PLAYER_DATA_POOL_CAPACITY`) the player record id (`player()`) of the person with index
/// `person` (`staff::SIZE`-byte record in `G_STAFF_TABLE`) at +0, the nation at `regen_pool_item::NATION_ID` and the
/// second nation at `regen_pool_item::SECOND_NATION_ID` (+0xc is left as it is). Count = capacity − 1: grows by 250
/// (realloc `crt_realloc`; out of memory, box 0xa0a and end, `regen_out_of_memory`). From process_retirements
/// (`process_retirements`), when a player who also has a non-player record stops playing.
/// Original: `PLAYER_REGEN::add_player_data_to_pool` (0x7a8350, player_regen.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn add_player_data_to_pool(this: usize, person: i32, nation: u32, second: u32) {
    if read_usize(this + player_regen_fields::PLAYER_DATA_POOL_COUNT) == read_usize(this + player_regen_fields::PLAYER_DATA_POOL_CAPACITY).wrapping_sub(1) {
        let p = crt_realloc(read_usize(this + player_regen_fields::PLAYER_DATA_POOL), read_usize(this + player_regen_fields::PLAYER_DATA_POOL_CAPACITY).wrapping_add(250) << 4);
        write_usize(this + player_regen_fields::PLAYER_DATA_POOL, p);
        write_usize(this + player_regen_fields::PLAYER_DATA_POOL_CAPACITY, read_usize(this + player_regen_fields::PLAYER_DATA_POOL_CAPACITY).wrapping_add(250));
        if p == 0 {
            regen_out_of_memory(0xa0a);
        }
    }
    covered!("cpu_9 player bench");
    let record = Staff(read_usize(G_STAFF_TABLE).wrapping_add((person as u32).wrapping_mul(staff::SIZE as u32) as usize));
    let item = read_usize(this + player_regen_fields::PLAYER_DATA_POOL).wrapping_add(read_usize(this + player_regen_fields::PLAYER_DATA_POOL_COUNT) << 4);
    write_usize(item, read_usize(record.player().addr()));
    write_usize(item + regen_pool_item::NATION_ID, nation as usize);
    write_usize(item + regen_pool_item::SECOND_NATION_ID, second as usize);
    write_usize(this + player_regen_fields::PLAYER_DATA_POOL_COUNT, read_usize(this + player_regen_fields::PLAYER_DATA_POOL_COUNT).wrapping_add(1));
}

/// whether the spare non-player record list holds any entry
/// Original: `PLAYER_REGEN::has_spare_non_player_record` (0x7a6e20, player_regen.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn has_spare_non_player_record(this: usize) -> u32 {
    let c = read_usize(this + player_regen_fields::SPARE_NON_PLAYER_COUNT) as u32;
    (c & 0xffff_ff00) | (c != 0) as u32
}

/// Looks up the person's contract (cached in [0xae1080]) and returns his contract with that club (`exe::contract_manager_get_contract_for_club`).
pub(crate) unsafe fn contract_with_current_club(person: Staff) -> usize {
    let c = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person));
    write_usize(G_CONTRACT_LOOKUP_RESULT, c.addr());
    let club = if !c.is_null() { read_usize(G_CLUB_TABLE).wrapping_add(c.club().wrapping_mul(club::SIZE)) } else { 0 };
    exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), person, club)
}

/// Status byte of the club contract (+0x3d); -100 without contract.
pub(crate) unsafe fn contract_status(c2: usize) -> i8 {
    let c2 = Contract(c2);
    if c2.is_null() {
        0x9cu8 as i8
    } else {
        c2.happiness()
    }
}

/// The retirement age limit for non-players, by job type; reproduces the original's stray stack byte when there is no
/// job.
///
/// Job type +0x3d; without it +0x28; without both, as in the original routine, the low byte of the output pointer,
/// which it reads from the wrong stack slot.
///
/// 1, 2 and 4: 70 + random(10); 8 and 9: 60 + random(5); the others: 61 + random(5) with a club (+0x39) or national
/// team (+0x24), otherwise 55 + random(10). Byte.
pub(crate) unsafe fn non_player_age_limit(person: Staff, out: usize) -> i8 {
    let mut t = person.job_for_club() as i8;
    if t == 0 {
        t = person.job_for_nation() as i8;
        if t == 0 {
            t = out as u8 as i8; // simplification: the original reads the low byte of the `saida` argument here
        }
    }
    let (n, base) = match t {
        1 | 2 | 4 => (10, 0x46u8),
        8 | 9 => (5, 0x3c),
        _ => {
            if !person.club_job().is_null() || !person.national_job().is_null() {
                (5, 0x3d)
            } else {
                (10, 0x37)
            }
        }
    };
    (random(n) as u8).wrapping_add(base) as i8
}

/// Player retirement age limit from his record: 34, 30 or 31 by attribute bytes +0xf/+0x15/+0x14.
pub(crate) unsafe fn player_age_limit(j: PlayerSheet) -> i8 {
    if j.goalkeeper() >= 15 {
        0x22
    } else if j.attacker() >= 15 {
        0x1e
    } else if j.attacking_midfielder() < 15 {
        0x1f
    } else {
        0x1e
    }
}

/// Fills the retirement output entry: reason (+0x18), +0x19 = 0, person id and club id (-1 without club).
pub(crate) unsafe fn fill_retirement_entry(person: Staff, out: usize, reason: u8) {
    let out = RetirementEntry(out);
    out.set_reason(reason);
    out.set_announced(0);
    write_u32(out.addr(), read_usize(person.addr()) as u32);
    let c = person.club_job();
    out.set_club_id(if !c.is_null() { read_usize(c.addr()) as u32 } else { u32::MAX });
}

/// Sets both output dates (+8 and +0x10) to tomorrow (day + 1, current year).
pub(crate) unsafe fn set_dates_tomorrow(out: usize) {
    let out = RetirementEntry(out);
    let (dia, ano) = (read_u16(G_CURRENT_DATE).wrapping_add(1), read_u16(G_CURRENT_DATE_YEAR));
    out.set_announce_date(dia);
    out.set_announce_year(ano);
    out.set_retire_date(dia);
    out.set_retire_year(ano);
}

/// For age retirement without a valid key nation: both dates today + random(20), wrapping to random(10) of next year.
pub(crate) unsafe fn set_dates_no_key_nation(out: usize) {
    let out = RetirementEntry(out);
    let a = (random(20) as u32).wrapping_add(read_usize(G_CURRENT_DATE) as u32) as u16;
    let year = read_u16(G_CURRENT_DATE_YEAR);
    if a as i16 > 0x16d {
        let d = random(10) as u16;
        out.set_retire_date(d);
        out.set_retire_year(year.wrapping_add(1));
        out.set_announce_date(d);
        out.set_announce_year(year.wrapping_add(1));
    } else {
        out.set_announce_date(a);
        out.set_announce_year(year);
        out.set_retire_date(a);
        out.set_retire_year(year);
    }
}

/// Sets the retirement dates on leaving (+8 the announcement, +0x10 the retirement; only day and year, the +4 of each
/// date stays as it is), by the key nation of the club (`key_nation_index_of_club`) or, with no club, of the nation (+0x1a, `key_nation_index_of_nation`);
/// with neither, tomorrow.
///
/// Index 0..0x22: the announcement on today + 1 + random draw(95) (past 365, day random draw(60) of the following
/// year); the retirement at the nation's season end (+0x15 of the 72-byte record at 0xb63d60) + random draw(30) (past
/// 365, random draw(30) of the following year; with +0x1d, one more year); a retirement before the announcement is
/// moved to the announcement, and if it is less than 45 days after it (`exe::date_days_since`) the announcement moves to it.
///
/// Index out of range: `fora` (each routine has its own).
pub(crate) unsafe fn set_retirement_dates(person: Staff, out: usize, away: unsafe fn(usize)) {
    let out = RetirementEntry(out);
    let club = person.club_job();
    let k = if !club.is_null() {
        exe::key_nation_index_of_club(club) as i8 
    } else {
        let nation = person.nation();
        if nation.is_null() {
            return set_dates_tomorrow(out.addr());
        }
        (exe::key_nation_index_of_nation(nation) as u32) as u8 as i8
    };
    if k < 0 || k >= 0x23 {
        return away(out.addr());
    }
    covered!("cpr_8 dates by nation");
    let year = read_u16(G_CURRENT_DATE_YEAR);
    let a = (random(95) as u32).wrapping_add(read_usize(G_CURRENT_DATE) as u32).wrapping_add(1) as u16;
    let (dia1, ano1) = if a as i16 > 0x16d { (random(60) as u16, year.wrapping_add(1)) } else { (a, year) };
    out.set_announce_year(ano1);
    out.set_announce_date(dia1);
    let t = 0xb63d60usize.wrapping_add((k as usize) * 0x48);
    let d2 = (random(30) as u16).wrapping_add(read_u16(t + key_nation_record::SEASON_END_DAY)); // the nation’s end of season
    out.set_retire_date(d2);
    let more_one = read_u8(t + key_nation_record::SEASON_SPANS_YEAR) != 0;
    if d2 as i16 > 0x16d {
        out.set_retire_date(random(30) as u16);
        out.set_retire_year(year.wrapping_add(if more_one { 2 } else { 1 }));
    } else {
        out.set_retire_year(if more_one { year.wrapping_add(1) } else { year });
    }
    let mut t8 = [0u8; 8];
    let (da, aa) = (out.retire_year(), out.announce_year());
    if da < aa || (da == aa && (out.retire_date() as i16) < (out.announce_date() as i16)) {
        exe::date_operator_3d(out.addr() + retirement_entry::RETIRE_DATE, (t8.as_mut_ptr()) as usize, out.addr() + retirement_entry::ANNOUNCE_DATE);
        return;
    }
    if exe::date_days_since(out.addr() + retirement_entry::RETIRE_DATE, read_usize(out.addr() + retirement_entry::ANNOUNCE_DATE) as u32, out.announce_date_leap_year() as u32) >= 0x2d {
        return;
    }
    exe::date_operator_3d(out.addr() + retirement_entry::ANNOUNCE_DATE, (t8.as_mut_ptr()) as usize, out.addr() + retirement_entry::RETIRE_DATE);
}

/// At a new game, decides whether a person with a club retires this season: non-players by the job age limit, players
/// by the profile limit, the ability, the club's competition and the contract status; fills the output and the dates.
///
/// `player_regen_check_initial_retirement(person, out)` (thiscall without reading this, ret 8): on a new game (from
/// `build_initial_retirement_list`), does the person with a club (`club_job()`) retire this season? The contract
/// (contract_with_current_club) and the status (contract_status). Without a player record (`player()`): without a
/// manager record (`non_player()`) always; with it, when past the limit (non_player_age_limit); reason 0x20. A player:
/// past the record's limit (player_age_limit), v = ⌊⌊1 − (age − limit) × 0.1 (`rdata::F64_0_1`)⌋ × current reputation
/// (`current_reputation()`)⌋ (16 bits); over 45, reason 1. A club without a competition (`division()`; without a club,
/// a nation without `league_selected()`): random(3) ≠ 0 retires, otherwise only a negative status with a club.
/// Otherwise: v ≥ 5000 or `current_ability()` > 80, no; `current_ability()` < 10 or v < 1500, yes; with a club, only a
/// negative status; without a club, reason 2 with random(25 × `stamina()`) = 0. Whoever retires: the output
/// (fill_retirement_entry: id, club, reason) and the announcement and retirement dates (set_retirement_dates, by the
/// key nation; without a valid key nation, tomorrow); returns 1.
/// Original: `player_regen_check_initial_retirement` (0x7a79c0, player_regen.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn player_regen_check_initial_retirement(_this: usize, person: usize, out: usize) -> u8 {
    let person = Staff(person);
    if person.club_job().is_null() {
        return 0;
    }
    let c2 = contract_with_current_club(person);
    let status = contract_status(c2);
    let age = person.age() as i8;
    let j = person.player();
    let reason: u8 = 'm: {
        if j.is_null() {
            if !person.non_player().is_null() && age <= non_player_age_limit(person, out) {
                return 0;
            }
            break 'm 0x20;
        }
        let lim = player_age_limit(j);
        if age <= lim {
            return 0;
        }
        let f = ftol(read_f64(rdata::F64_1) - (age as i32 - lim as i32) as f64 * read_f64(rdata::F64_0_1)) as i16;
        let v = ftol(f as f64 * j.current_reputation() as f64) as i16;
        if age > 0x2d {
            break 'm 1;
        }
        let club = person.club_job();
        let draws = if !club.is_null() {
            club.division().addr() == 0
        } else {
            let n = person.nation();
            !n.is_null() && n.league_selected() == 0
        };
        if draws {
            if random(3) != 0 {
                break 'm 1;
            }
            if club.is_null() || status >= 0 {
                return 0;
            }
            break 'm 1;
        }
        if v >= 0x1388 {
            return 0;
        }
        let hab = j.current_ability();
        if hab > 0x50 {
            return 0; // cpr8: high ability keeps the player
        }
        if hab < 0xa || v < 0x5dc {
            covered!("cpr_8 initial ability");
            break 'm 1;
        }
        if !club.is_null() {
            if status < 0 {
                break 'm 1;
            }
            return 0;
        }
        if random(25 * j.stamina() as i32) != 0 {
            return 0;
        }
        2
    };
    fill_retirement_entry(person, out, reason);
    set_retirement_dates(person, out, set_dates_tomorrow);
    1
}

/// In the search for new retirements, decides whether the person retires: like the initial check, with the limit
/// adjusted by contract and wage, unemployment, contract end and potential; without a key nation, dates drawn near
/// today.
///
/// `player_regen_check_retirement(person, out)` (thiscall without reading this, ret 8), from the search for new
/// retirements (`find_new_retirements`). Without a player record: as in `player_regen_check_initial_retirement` (reason
/// 0x20). A player above the record's limit: over 33, the limit drops 1 without a contract with the club, or with
/// `grievances()` in the contract and `attacker()`/`attacking_midfielder()` of the record = 20, or the high nibble of
/// `squad_status()` = 7; with a club and a contract with wage (`wage()`) above 750 × (`loyalty()` + 10) until after
/// this year (`contract_expires_year()`), it drops 1 (and 1 more with `loyalty()` < 6). v = ⌊(1 − (age − limit) × 0.075
/// (`rdata::F64_0_075`)) × current reputation (`current_reputation()`)⌋ (16 bits). Reason 1: over 45; over 34 without a
/// club and unemployed for more than 65 days (since `date_joined_club()`, `FN_DATE_DAYS_SINCE`); 3 years above the
/// limit, over 31 and the contract's `unhappy_flag()` > 5; 4 above: without a club; or a contract less than 300 days
/// from its end (`exe::staff_contract_days_to_end_date`), status below 50 and manager (`non_player()`) or `ambition()`
/// > random(20) with potential × 0.85 (`rdata::F64_0_85`) above the current ability (`potential_ability()` and
/// `current_ability()`). Then v > 5500, no. A club in a competition not running in the game (`G_COMP_OBJECT_TABLE`[id]
/// = 0, or without `division()`; without a club, a nation without `league_selected()`): random(3) = 0 retires; with a
/// club and a negative status too; otherwise only with v < 3500. Otherwise as in
/// `player_regen_check_initial_retirement` (`current_ability()` > 100 no; `current_ability()` < 10 or v < 1500 yes;
/// with a club only a negative status; without a club reason 2 with random(25 × `stamina()`) = 0). Whoever retires: the
/// output and the dates (without a valid key nation, set_dates_no_key_nation, near today); returns 1.
/// Original: `player_regen_check_retirement` (0x7a7310, player_regen.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn player_regen_check_retirement(_this: usize, person: usize, out: usize) -> u8 {
    let person = Staff(person);
    let c2 = Contract(contract_with_current_club(person));
    let status = contract_status(c2.addr());
    let age = person.age() as i8;
    let j = person.player();
    let reason: u8 = 'm: {
        if j.is_null() {
            if !person.non_player().is_null() && age <= non_player_age_limit(person, out) {
                return 0;
            }
            break 'm 0x20;
        }
        let mut lim = player_age_limit(j);
        if age <= lim {
            return 0;
        }
        if age > 0x21 {
            if c2.is_null() {
                lim = lim.wrapping_sub(1);
            } else if (c2.grievances() as usize) != 0
                && (j.attacker() == 0x14 || j.attacking_midfielder() == 0x14 || c2.squad_status() & 0xf0 == 0x70)
            {
                lim = lim.wrapping_sub(1);
            }
        }
        if !person.club_job().is_null() && !c2.is_null() {
            let x = person.loyalty() as i32;
            if ((c2.wage() as usize) as i32) > (x + 10).wrapping_mul(750) && (c2.contract_expires_year() as u16) > read_u16(G_CURRENT_DATE_YEAR) {
                lim = lim.wrapping_sub(1);
                if x < 6 {
                    lim = lim.wrapping_sub(1);
                }
            }
        }
        let v = ftol((read_f64(rdata::F64_1) - (age as i32 - lim as i32) as f64 * read_f64(rdata::F64_0_075)) * j.current_reputation() as f64) as i16;
        if age > 0x2d {
            break 'm 1;
        }
        if age > 0x22 && person.club_job().is_null() {
            let days: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
            if days(G_CURRENT_DATE, person.date_joined_club() as u32, person.date_joined_club_leap_year() as u32) > 0x41 {
                break 'm 1;
            }
        }
        let (i, l) = (age as i32, lim as i32);
        if i > l + 2 && age > 0x1f && !c2.is_null() && c2.unhappy_flag() > 5 {
            break 'm 1;
        }
        if i > l + 3 {
            if person.club_job().is_null() {
                break 'm 1;
            }
            if !c2.is_null() && ((exe::staff_contract_days_to_end_date(c2) as u32) as i16) < 300 && status < 0x32 {
                if !person.non_player().is_null() {
                    break 'm 1;
                }
                if person.ambition() as i32 > random(20) {
                    let (cur, pot) = (j.current_ability() as f64, j.potential_ability() as f64);
                    if !(pot * read_f64(rdata::F64_0_85) <= cur) {
                        break 'm 1;
                    }
                }
            }
        }
        if v > 0x157c {
            return 0;
        }
        let club = person.club_job();
        let draws = if !club.is_null() {
            let lg = club.division().addr();
            lg == 0 || read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(lg).wrapping_mul(4))) == 0
        } else {
            let n = person.nation();
            !n.is_null() && n.league_selected() == 0
        };
        if draws {
            covered!("cpr_8 out of the game");
            if random(3) == 0 {
                break 'm 1; // cpr8: one in three retires
            }
            if !person.club_job().is_null() && status < 0 {
                break 'm 1;
            }
            if v >= 0xdac {
                return 0;
            }
            break 'm 1;
        }
        let cur = j.current_ability();
        if cur > 0x64 {
            return 0;
        }
        if cur < 0xa || v < 0x5dc {
            break 'm 1;
        }
        if !club.is_null() {
            if status < 0 {
                break 'm 1;
            }
            return 0;
        }
        if random(25 * j.stamina() as i32) != 0 {
            return 0;
        }
        2
    };
    fill_retirement_entry(person, out, reason);
    set_retirement_dates(person, out, set_dates_no_key_nation);
    1
}

/// At a new game, tests every eligible person (`exe::player_regen_check_initial_retirement`), adds those who retire to
/// the list and sorts it.
///
/// `PLAYER_REGEN::build_initial_retirement_list`; SEH only for the local dates, with an empty destructor. On a new
/// game, each person (0x6e bytes in `G_STAFF_TABLE`, up to `G_STAFF_COUNT`, re-read at each step) with id below
/// `G_STAFF_COUNT` − 16, without bits 0x30 (`staff_state_status_flags::RETIREMENT_STATE`) of
/// `staff_state::STATUS_FLAGS` of the 79-byte record (`G_STAFF_STATE_TABLE`) and without a contract
/// (`exe::contract_manager_get_contract`) with `leaving_on_bosman()` is tested
/// (`exe::player_regen_check_initial_retirement`); whoever retires goes into the list
/// (`exe::player_regen_add_retirement`). At the end the qsort of the list (`player_regen_fields::RETIREMENTS`, count
/// `player_regen_fields::RETIREMENT_COUNT`, 26 bytes, `player_regen_compare_retirements`).
/// Original: `PLAYER_REGEN::build_initial_retirement_list` (0x7a58f0, player_regen.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn build_initial_retirement_list(this: usize) {
    covered!("cpr_8 initial list");
    // simplification: the original builds the entry’s two dates (0x54a110) and leaves the rest as it was on the stack; the
    // 0x7a79c0 writes everything 0x7a8510 copies before returning 1 (the third local date is never read: left out)
    let mut e = [0u8; 0x1a];
    exe::date_date_54a110((e.as_mut_ptr().add(8)) as usize);
    exe::date_date_54a110((e.as_mut_ptr().add(0x10)) as usize);
    let mut p = read_usize(G_STAFF_TABLE);
    let mut i = 0i32;
    while i < read_usize(G_STAFF_COUNT) as i32 {
        let id = read_usize(p) as i32;
        if id < (read_usize(G_STAFF_COUNT) as i32).wrapping_sub(0x10)
            && *((read_usize(G_STAFF_STATE_TABLE).wrapping_add((id as usize).wrapping_mul(staff_state::SIZE)) + staff_state::STATUS_FLAGS) as *const u8) & staff_state_status_flags::RETIREMENT_STATE == 0
        {
            let c = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p));
            if (c.is_null() || c.leaving_on_bosman() == 0) && exe::player_regen_check_initial_retirement(this, p, e.as_mut_ptr()) != 0 {
                exe::player_regen_add_retirement(this, e.as_ptr());
            }
        }
        i += 1;
        p = p.wrapping_add(0x6e);
    }
    exe::crt_qsort(read_usize(this + player_regen_fields::RETIREMENTS), read_usize(this + player_regen_fields::RETIREMENT_COUNT) as u32, 0x1a, 0x7ae2a0);
}

/// Regions the two retirement decisions touch: random state, [0xae1080], the output entry and `exe::date_days_since`'s static date.
pub(crate) unsafe fn retirement_decision_regions(out: usize) -> Vec<(usize, usize)> {
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.extend_from_slice(&[(G_CONTRACT_LOOKUP_RESULT, 4), (out, 0x1a), (G_DAYS_SINCE_SCRATCH_DATE, 8), (0xae2418, 1)]);
    reg
}

// simplification: the initial list appends (realloc) and rolls in each test: it does not run twice. Original only; the mode
// Rust and the career prove the copy.

/// Writes a zero dword at the address.
pub(crate) unsafe fn zero_dword(a: usize) {
    write_u32(a, 0);
}

/// Tests every eligible person for retirement by age (`exe::player_regen_check_retirement`) or unemployment
/// (`exe::player_regen_check_unemployed_retirement`), adds those who retire and sorts the list.
///
/// `PLAYER_REGEN::find_new_retirements` (on the days of `is_player_regen_day`); SEH only for the local dates, with an
/// empty destructor. Each person (0x6e bytes in `G_STAFF_TABLE`, up to the end computed with the count `G_STAFF_COUNT`
/// at the start). Excluded: under 30 years old (`staff::AGE`) at a club of a competition that runs in the game
/// (`G_COMP_OBJECT_TABLE`[id] of the competition `club::DIVISION`); id from `G_STAFF_COUNT` − 16 on (re-read at each
/// step); bits 0x30 (`staff_state_status_flags::RETIREMENT_STATE`) of `staff_state::STATUS_FLAGS` of the 79-byte record
/// (`G_STAFF_STATE_TABLE`); those already in the list (`player_regen_fields::RETIREMENTS`, 26 bytes, count
/// `player_regen_fields::RETIREMENT_COUNT`); a contract (`exe::contract_manager_get_contract`) with
/// `contract::LEAVING_ON_BOSMAN`. The others: retirement by age (`exe::player_regen_check_retirement`) or, failing
/// that, the unemployed one (`exe::player_regen_check_unemployed_retirement`); whoever retires goes into the list
/// (`exe::player_regen_add_retirement`). At the end the qsort of the list (`player_regen_compare_retirements`).
/// Original: `PLAYER_REGEN::find_new_retirements` (0x7a5790, player_regen.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn find_new_retirements(this: usize) {
    // simplification: the original builds the two entry dates (0x54a110) and leaves the rest as it was on the stack; the two
    // decisions write everything 0x7a8510 copies before returning 1 (the third local date is never read: left out)
    let mut e = [0u8; 0x1a];
    exe::date_date_54a110((e.as_mut_ptr().add(8)) as usize);
    exe::date_date_54a110((e.as_mut_ptr().add(0x10)) as usize);
    let mut p = read_usize(G_STAFF_TABLE);
    let end = p.wrapping_add(read_usize(G_STAFF_COUNT).wrapping_mul(staff::SIZE));
    while p < end {
        'x: {
            if read_i8(p + staff::AGE) < 0x1e {
                let c = read_usize(p + staff::CLUB_JOB);
                if c != 0 {
                    let lg = read_usize(c + club::DIVISION);
                    if lg != 0 && read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(lg).wrapping_mul(4))) != 0 {
                        break 'x;
                    }
                }
            }
            let id = read_usize(p) as i32;
            if id >= (read_usize(G_STAFF_COUNT) as i32).wrapping_sub(0x10) {
                break 'x;
            }
            if read_u8(read_usize(G_STAFF_STATE_TABLE).wrapping_add((id as usize).wrapping_mul(staff_state::SIZE)) + staff_state::STATUS_FLAGS) & staff_state_status_flags::RETIREMENT_STATE != 0 {
                break 'x;
            }
            let k = read_usize(this + player_regen_fields::RETIREMENT_COUNT) as i32;
            if k != 0 {
                let l = read_usize(this + player_regen_fields::RETIREMENTS);
                for i in 0..k.max(0) as usize {
                    if read_usize(l + i * 0x1a) as i32 == id {
                        break 'x;
                    }
                }
            }
            let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p);
            if c != 0 && *((c + contract::LEAVING_ON_BOSMAN) as *const u8) != 0 {
                break 'x;
            }
            if exe::player_regen_check_retirement(this, p, e.as_mut_ptr()) != 0 {
                exe::player_regen_add_retirement(this, e.as_ptr());
            } else if exe::player_regen_check_unemployed_retirement(p, e.as_mut_ptr()) != 0 {
                covered!("cpr_9 unemployed");
                exe::player_regen_add_retirement(this, e.as_ptr());
            }
        }
        p = p.wrapping_add(0x6e);
    }
    exe::crt_qsort(read_usize(this + player_regen_fields::RETIREMENTS), read_usize(this + player_regen_fields::RETIREMENT_COUNT) as u32, 0x1a, 0x7ae2a0);
}

/// Removes the person from the slot of job `cargo` (signed type) in club `c` (the person's club at +`campo`: 0x39 the
/// club, 0x24 the national team).
///
/// 1 chairman (`+0xbf`); 2 the three at `+0xc3`; 5 and 12 manager (`+0xcf`, only if it is him): for 5, if human (id
/// from [`0xae2364`] − 16 on), the club's offers fall (`0x8c4950`); he leaves the job (`0x69b790` with 3, 0, 0) and, if
/// the club can be managed (`0x544820`), the vacancy opens (`0x69ba50`); 6 assistant (`+0xd3`); 8 and 15 the five at
/// `+0x19f`, after distributing the coaches of the human club (`0x600a50` with 1, 1; `0x8b2880` with 0); 9 the seven at
/// `+0x1b3` and, at the human club, +`campo` = 0 and the manager's scout lists (`0x7f7270`); 10 the three at `+0x1cf`.
pub(crate) unsafe fn leave_job_slot(person: Staff, c: Club, job: i8, field: usize) {
    let clear = |base: usize, n: usize| {
        for k in 0..n {
            let a = c.addr() + base + 4 * k;
            if read_usize(a) == person.addr() {
                zero_dword(a);
            }
        }
    };
    let human = || {
        exe::human_manager_is_human_club(c, 1, (1) as u32, (0) as u32, 0) != 0
    };
    match job {
        1 => zero_dword(c.addr() + club::CHAIRMAN),
        2 => clear(club::DIRECTORS, 3),
        5 | 12 => {
            if c.manager() != person {
                return;
            }
            covered!("cpr_9 manager");
            if job == 5 && (read_usize(person.addr()) as i32) >= (read_usize(G_STAFF_COUNT) as i32).wrapping_sub(0x10) {
                exe::transfer_manager_cancel_club_offers(read_usize(G_TRANSFER_MANAGER), c);
            }
            exe::manager_manager_remove_club_manager(read_usize(G_MANAGER_MANAGER), c, 3, 0, 0);
            if (exe::club_can_be_managed(c) as u32) as u8 != 0 {
                exe::manager_manager_add_job_vacancy(read_usize(G_MANAGER_MANAGER), c, person); // cpr9: the manager slot opens
            }
        }
        6 => zero_dword(c.addr() + club::ASSISTANT_MANAGER),
        8 | 15 => {
            if human() {
                exe::training_manager_update_coach_assignments(read_usize(G_TRAINING_MANAGER), c, 0);
            }
            covered!("cpr_9 coach");
            clear(club::COACHES, 5); // cpr9: the coach slots
        }
        9 => {
            clear(club::SCOUTS, 7);
            if human() {
                zero_dword(person.addr() + field);
                let lists: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_SCOUT_MANAGER_SYNC_SCOUT_LISTS);
                lists(read_usize(G_SCOUT_MANAGER), c.manager().addr());
            }
        }
        10 => clear(club::PHYSIOS, 3),
        _ => {}
    }
}

/// removes the person from the job slots at the club and national team (chairman, directors, manager with the vacancy
/// opened, assistant, coaches, scouts, physios) and clears the club, the national team and the job types
/// Original: `player_regen_retire_from_non_playing_staff` (0x7aaeb0, player_regen.cpp, stdcall, ret 4).
pub(crate) unsafe extern "stdcall" fn player_regen_retire_from_non_playing_staff(person: usize) {
    let person = Staff(person);
    let c = person.club_job();
    if !c.is_null() {
        leave_job_slot(person, c, person.job_for_club() as i8 , 0x39);
        zero_dword(person.addr() + staff::CLUB_JOB);
    }
    let s = person.national_job();
    if !s.is_null() {
        leave_job_slot(person, s, person.job_for_nation() as i8 , 0x24);
        zero_dword(person.addr() + staff::NATIONAL_JOB);
    }
    person.set_job_for_club(job_kind::UNEMPLOYED);
    person.set_job_for_nation(job_kind::UNEMPLOYED);
}

// simplification: the search appends (realloc) and rolls on each test: it does not run twice. Original only; Rust mode and the
// career runs prove the copy.

/// Does leaving jobs only touch the person and the two clubs (not a club manager nor coach/scout jobs of a human club)?
pub(crate) unsafe fn job_exit_is_memory_only(person: usize) -> bool {
    for (campo, tipo) in [(0x39usize, 0x3dusize), (0x24, 0x28)] {
        let c = Club(read_usize(person + campo));
        if c.is_null() {
            continue;
        }
        match read_i8(person + tipo) {
            5 | 12 if c.manager().addr() == person => return false,
            8 | 15 | 9 if exe::human_manager_is_human_club(c, 1, (1) as u32, (0) as u32, 0) != 0 => return false,
            _ => {}
        }
    }
    true
}

/// Regions of leaving jobs: the person (0x6e) and his club and national team records (0x245).
pub(crate) unsafe fn job_exit_regions(person: Staff) -> Vec<(usize, usize)> {
    vec![(person.addr(), staff::SIZE), (person.club_job().addr(), club::SIZE), (person.national_job().addr(), club::SIZE)]
}

// simplification: the history goes to the file (does not run twice) and the copy without it only writes [0xae1080]: only the
// original; the copy takes the branch.

/// Scores a person as the donor of non-player data: −10000 with a non-player record, when retiring or retired, or under
/// 31; otherwise random draws and attribute terms.
///
/// `player_regen_score_non_player_candidate(person)` (stdcall, ret 4): the person's score as donor of the non-player
/// data (from `player_regen_pick_staff_for_non_player_data`, for each non-player person). −10000 with `non_player()`,
/// with bits 0x30 (`staff_state_status_flags::RETIREMENT_STATE`) of `staff_state::STATUS_FLAGS` of the 79-byte record
/// (`G_STAFF_STATE_TABLE`) or under 31 (`age()`). Otherwise random 150 × (age − 31); with a club (`club_job()`) with
/// reputation (`reputation()`) and a player record (`player()`), + 250 × (the player's current reputation
/// `current_reputation()` ÷ the club's reputation); with a player record, + random(⌊(`leadership()` ÷ 2)³⌋) (pow
/// `game_pow` and _ftol), +100 with `world_reputation()` > 7500 and +50 with `current_reputation()` > 7500; the
/// attributes + 5 × (3 × (`temperament()` + `pressure()`) + 2 × (`loyalty()` + `determination()`) + `adaptability()`) +
/// ⌊`professionalism()`²⌋ + ⌊`ambition()`²⌋. With a club: a random draw of the score (+75 if the club has no
/// `manager()`); without a club: + (`int_apps()`)², and the score doubles if the player's `world_reputation()` exceeds
/// a random(10000) (it reads the player record without checking, as the original does). Hot (every person on each
/// call): no counter.
/// Original: `player_regen_score_non_player_candidate` (0x7a7150, player_regen.cpp, stdcall, ret 4).
pub(crate) unsafe extern "stdcall" fn player_regen_score_non_player_candidate(person: usize) -> i32 {
    let person = Staff(person);
    if !person.non_player().is_null() {
        return -10000;
    }
    let id = read_usize(person.addr());
    if read_u8(read_usize(G_STAFF_STATE_TABLE).wrapping_add(id.wrapping_mul(staff_state::SIZE)) + staff_state::STATUS_FLAGS) & staff_state_status_flags::RETIREMENT_STATE != 0 {
        return -10000;
    }
    let age = (person.age() as i8) as i32;
    if age < 31 {
        return -10000;
    }
    let mut rating = random((age - 31) * 150); // a idade
    let club = person.club_job();
    if !club.is_null() {
        let injury_word_0c = club.reputation() as i32;
        let player = person.player();
        if injury_word_0c != 0 && !player.is_null() {
            rating = rating.wrapping_add(((player.current_reputation() as i32) / injury_word_0c).wrapping_mul(250));
        }
    }
    let player = person.player();
    if !player.is_null() {
        let v = (player.leadership() as i32) / 2;
        rating = rating.wrapping_add(random(ftol(game_pow(v as f64, 3.0))));
        if (player.world_reputation() as i32) > 7500 {
            rating = rating.wrapping_add(100);
        }
        if (player.current_reputation() as i32) > 7500 {
            rating = rating.wrapping_add(50);
        }
    }
    let quadrados = ftol(game_pow((person.professionalism() as i32) as f64, 2.0)).wrapping_add(ftol(game_pow((person.ambition() as i32) as f64, 2.0)));
    let sum = 3 * ((person.temperament() as i32) + (person.pressure() as i32)) + 2 * ((person.loyalty() as i32) + (person.determination() as i32)) + (person.adaptability() as i32);
    rating = rating.wrapping_add(sum * 5).wrapping_add(quadrados);
    let club = person.club_job();
    if !club.is_null() {
        if club.manager().addr() == 0 {
            rating = rating.wrapping_add(75);
        }
        return random(rating);
    }
    let b = person.int_apps() as i32;
    rating = rating.wrapping_add(b * b);
    let r = random(10000); // no club
    if (person.player().world_reputation() as i32) > r {
        rating = rating.wrapping_add(rating);
    }
    random(rating)
}

/// Effective date of a retirement list entry: announcement (+8) if not yet past the game date, else retirement (+0x10).
pub(crate) unsafe fn entry_effective_date(e: usize) -> usize {
    let e = RetirementEntry(e);
    let (dia, ano) = (read_i16_as_i32(G_CURRENT_DATE), read_u16(G_CURRENT_DATE_YEAR));
    let a = e.announce_year();
    if a < ano || (a == ano && ((e.announce_date() as i16) as i32) < dia) {
        e.addr() + retirement_entry::RETIRE_DATE
    } else {
        e.addr() + retirement_entry::ANNOUNCE_DATE
    }
}

/// qsort comparator of the retirement list: by the effective date, year and day; on a tie by id.
///
/// `player_regen_compare_retirements(a, b)` (cdecl): the qsort of the retirement list (0x1a-byte records; from `process_retirements`
/// every day, which stops at the first future entry, from the initial lists `find_new_retirements`/`build_initial_retirement_list` and from `remove_retirement`): by
/// the effective date (entry_effective_date: the announcement +8 if not yet past, otherwise the retirement +0x10), the
/// year unsigned and the day signed; on a tie by the id (+0). Without `a`, −1; without `b`, 1. Hot (qsort): no counter.
/// Original: `player_regen_compare_retirements` (0x7ae2a0, player_regen.cpp, cdecl).
pub(crate) unsafe extern "C" fn player_regen_compare_retirements(a: usize, b: usize) -> i32 {
    if a == 0 {
        return -1;
    }
    if b == 0 {
        return 1;
    }
    let (da, db) = (entry_effective_date(a), entry_effective_date(b));
    let (aa, ab) = (read_u16(da + 2), read_u16(db + 2));
    if aa != ab {
        return aa as i32 - ab as i32; // the earliest year first
    }
    let (ja, jb) = (read_i16_as_i32(da), read_i16_as_i32(db));
    if ja == jb {
        return (read_usize(a) as i32).wrapping_sub(read_usize(b) as i32);
    }
    ja - jb
}

/// qsort comparator of the regen queue: by year (+6), day (+4) and id (+0).
///
/// `player_regen_compare_queue_entries(a, b)` (cdecl): the qsort of the regen queue (0xc-byte records, at the end of
/// `process_retirements` every day; `process_regen_queue` creates the regens of the due entries): by year (+6, unsigned), then by day (+4,
/// signed), then by id (+0). Without `a`, −1; without `b`, 1. Hot (qsort): no counter.
/// Original: `player_regen_compare_queue_entries` (0x7ae240, player_regen.cpp, cdecl).
pub(crate) unsafe extern "C" fn player_regen_compare_queue_entries(a: usize, b: usize) -> i32 {
    if a == 0 {
        return -1;
    }
    if b == 0 {
        return 1;
    }
    let (aa, ab) = (read_u16(a + regen_queue_entry::REGEN_YEAR), read_u16(b + regen_queue_entry::REGEN_YEAR));
    if aa > ab {
        return 1; // o ano mais tarde depois
    }
    if aa < ab {
        return -1;
    }
    let (ja, jb) = (read_i16_as_i32(a + regen_queue_entry::REGEN_DATE), read_i16_as_i32(b + regen_queue_entry::REGEN_DATE));
    if ja == jb {
        return (read_usize(a) as i32).wrapping_sub(read_usize(b) as i32);
    }
    ja - jb
}

/// Removes person `p` from the team at `p + campo` (+0x39 the club, +0x24 the national team).
///
/// The tactic (`remove_player_from_selection` on [0xde1d08], with 0 and then with 1 as the 3rd argument) and the first of the 50 slots
/// (+0xd7) holding him, which is emptied. The team is re-read after each call, as in the original routine.
pub(crate) unsafe fn remove_from_team(p: Staff, field: usize) {
    exe::tactics_remove_player_from_selection(read_usize(G_TACTICS), read_usize(p.addr() + field), p, 0, 1);
    exe::tactics_remove_player_from_selection(read_usize(G_TACTICS), read_usize(p.addr() + field), p, 1, 1);
    let team = Club(read_usize(p.addr() + field));
    if let Some(k) = (0..0x32usize).find(|&k| team.squad(k) == p.addr()) {
        team.set_squad(k, 0);
    }
}

/// Formats the "already retired" message into buffer 0xde1f64 (first name, surname, id).
pub(crate) unsafe fn format_already_retired(p: Staff) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    format(0xde1f64, 0xa70150, p.first_name(), p.second_name(), read_usize(p.addr()));
}

/// Removes the player from his club and national team and changes the player job to the staff job; without a kept job,
/// clears the club and the national team.
///
/// `PLAYER_REGEN::retire_from_playing_staff(p)` (stdcall, ret 4): removes `p` from the club (`club_job()`) and the
/// national team (`national_job()`) (`remove_from_team`: the tactics and the 50 slots) and changes the player job to
/// the staff one (tables 0x7aae60 and 0x7aae90): by the club job (`job_for_club()`, signed byte; outside 0..16 the
/// default applies) 12 → 5 (`job_kind::MANAGER`), 13 → 6 (`job_kind::ASSISTANT_MANAGER`), 15 → 8 (`job_kind::COACH`)
/// (stays at the club), 0 and 16 (warning "already retired", `format_already_retired`) nothing, 1 to 10 the routine
/// ends there, the others 0x10 (retired, `job_kind::RETIRED_PLAYER`). Then by the national team job
/// (`job_for_nation()`) in the same way, but 12/13/15 write into `job_for_club()` (`set_job_for_club`; the original
/// writes there, not into `job_for_nation()`) and the default writes 0x10 into `job_for_nation()`. With no job kept,
/// the club (`set_club_job`) and the national team (`set_national_job`) are zeroed. No SEH, no random draw.
/// Original: `PLAYER_REGEN::retire_from_playing_staff` (0x7aace0, player_regen.cpp, stdcall, ret 4).
pub(crate) unsafe extern "stdcall" fn retire_from_playing_staff(p: usize) {
    let p = Staff(p);
    if !p.club_job().is_null() {
        remove_from_team(p, 0x39);
    }
    if !p.national_job().is_null() {
        remove_from_team(p, 0x24);
    }
    let stays_in_club = match (p.job_for_club() as i8) as i32 {
        0 => false,
        1..=10 => return, // staff: the original exits through the end of the routine
        12 => {
            p.set_job_for_club((job_kind::MANAGER) as u8);
            true
        }
        13 => {
            p.set_job_for_club((job_kind::ASSISTANT_MANAGER) as u8);
            true
        }
        15 => {
            p.set_job_for_club((job_kind::COACH) as u8);
            true
        }
        16 => {
            format_already_retired(p);
            false
        }
        _ => {
            covered!("cpn_4 retired player");
            p.set_job_for_club((job_kind::RETIRED_PLAYER) as u8); // aposentado
            false
        }
    };
    let stays_in_national_team = match (p.job_for_nation() as i8) as i32 {
        0 => false,
        1..=10 => return,
        12 => {
            p.set_job_for_club((job_kind::MANAGER) as u8); // in +0x3d, like the original
            true
        }
        13 => {
            p.set_job_for_club((job_kind::ASSISTANT_MANAGER) as u8);
            true
        }
        15 => {
            p.set_job_for_club((job_kind::COACH) as u8);
            true
        }
        16 => {
            format_already_retired(p);
            false
        }
        _ => {
            p.set_job_for_nation((job_kind::RETIRED_PLAYER) as u8);
            false
        }
    };
    if !stays_in_club {
        p.set_club_job(Club(0));
    }
    if !stays_in_national_team {
        p.set_national_job(Club(0));
    }
}

/// Youth with second nationality Basque Country may only join five specific clubs: true for other clubs.
pub(crate) unsafe fn is_basque_restricted(player: Staff, club: usize) -> bool {
    let e = player.second_nation();
    if e.is_null() || read_usize(e.addr()) != read_usize(KEY_NATION_PAYS_BASQUE) {
        return false;
    }
    let id = read_usize(club);
    ![0x9d0120usize, 0x9d0134, G_KEY_CLUB_REAL_SOCIEDAD_C_F, G_KEY_CLUB_REAL_SOCIEDAD_C_F_B, 0x9d0124].iter().any(|&g| read_usize(g) == id)
}

/// Can the club bid for the player (`can_club_bid_for_player` of the transfer manager, not ignoring open offers)?
pub(crate) unsafe fn can_club_bid(player: usize, club: usize) -> bool {
    let mut reason = 0i16;
    exe::transfer_manager_can_club_bid_for_player(read_usize(G_TRANSFER_MANAGER), player, club, (&mut reason) as *mut i16 as *mut u16, 0) != 0
}

/// Picks the best-scored club (`exe::player_regen_score_club_for_player`) among those where the youngster is not a
/// foreigner, that are not a B team, that may approach him and with a squad under 50.
///
/// `PLAYER_REGEN::pick_best_club_for_player(player)`: among the clubs (`G_CLUB_TABLE`, 0x245 bytes; the first
/// `G_CLUB_COUNT` − 2 × `G_NATION_COUNT`, re-read on every loop) where the player is not a foreigner
/// (`FN_DATABASE_IS_STAFF_FOREIGN` with the nation `club::NATION` and 1), that are not the B team of a pair
/// (`exe::club_get_linked_b_team` with 1, finding the partner with 0), not vetoed (`is_basque_restricted`), that may
/// approach him (`can_club_bid`) and with a squad (`exe::database_count_club_squad`) under 50, the one with the highest
/// score (`exe::player_regen_score_club_for_player`, random; compared in 16 bits, starting from −25000). None: 0. No
/// SEH; the partner local is only read when `exe::club_get_linked_b_team` finds it (and writes it).
/// Original: `PLAYER_REGEN::pick_best_club_for_player` (0x7ad0e0, player_regen.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn pick_best_club_for_player(_this: usize, player: usize) -> usize {
    let foreigner: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_DATABASE_IS_STAFF_FOREIGN);
    let total = || (read_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_usize(G_NATION_COUNT) as i32).wrapping_mul(2));
    let (mut melhor, mut melhor_nota) = (0usize, -25000i16);
    let mut eh_a = 0u32;
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i32;
    while i < total() {
        let candidate = foreigner(player, read_usize(club + club::NATION), 1) == 0
            && (exe::club_get_linked_b_team(club, &mut eh_a, 1) == 0 || eh_a != 0)
            && !is_basque_restricted(Staff(player), club);
        if candidate {
            let squad = exe::database_count_club_squad(club, 0, 0);
            if can_club_bid(player, club) && (squad as i8) < 0x32 {
                // simplification: the original pushes the squad byte with garbage in the high bits; 0x7ad220 reads only the byte
                let s = exe::player_regen_score_club_for_player(player, club, squad as u32) as i16;
                if s > melhor_nota {
                    covered!("cpn_4 better club for the youngster");
                    melhor = club;
                    melhor_nota = s;
                }
            }
        }
        i += 1;
        club += club::SIZE;
    }
    melhor
}

/// Nation of the youth's list: his nation, with Wales and Ireland mapped to England and Northern Ireland too with
/// random(5) < 2.
pub(crate) unsafe fn player_regen_youth_list_nation(player: Staff) -> usize {
    let n = read_usize(player.nation().addr());
    if n == read_usize(KEY_NATION_WALES) || n == read_usize(KEY_NATION_REPUBLIC_OF_IRELAND) || (n == read_usize(KEY_NATION_NORTHERN_IRELAND) && exe::utils_random(5) < 2) {
        return read_usize(KEY_NATION_ENGLAND);
    }
    n
}

/// Club list of nation n: [[this] + 4n].
pub(crate) unsafe fn nation_club_list(this: usize, n: usize) -> Llist {
    Llist(read_usize(read_usize(this).wrapping_add(n.wrapping_mul(4))))
}

/// Picks, from the club list of the youngster's nation, the club with the highest random draw of the score by
/// reputation against ability, among those that may approach him and with a squad under 50.
///
/// `PLAYER_REGEN::pick_club_from_nation_list(player)`: walks the club list of the youngster's nation
/// (`player_regen_youth_list_nation`, some nations mapped to [0x9cf2e4]; word +0 is the count, the cursor restarts from
/// the head (`set_cursor(head())`), the iterator `exe::llist_get_next` returns the id) and, among the clubs not vetoed
/// (`is_basque_restricted`), that may approach him (`can_club_bid`) and with a squad (`exe::database_count_club_squad`)
/// under 50, keeps the one with the highest random draw (`exe::utils_random`) of the score (5000 + reputation
/// `reputation()` minus its distances to the ability of the player record `player()`: `current_reputation()`,
/// `potential_ability()` × 50 and `current_ability()` × 50, in 16 bits), compared in 16 bits starting from −25000. No
/// list or an empty list: 0. A list that ends before the count: the "Error" box (line 0x19e9) and 0. No SEH.
/// Original: `PLAYER_REGEN::pick_club_from_nation_list` (0x7ad590, player_regen.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn pick_club_from_nation_list(this: usize, player: usize) -> usize {
    let player = Staff(player);
    let n = player_regen_youth_list_nation(player);
    let l = Llist((nation_club_list(this, n)).addr());
    if l.is_null() || read_i16(l.addr()) == 0 {
        return 0;
    }
    l.set_cursor(l.head());
    let (mut melhor, mut melhor_nota) = (0usize, -25000i16);
    if read_i16((nation_club_list(this, n)).addr()) <= 0 {
        return 0;
    }
    let mut i = 0i32;
    loop {
        let item = exe::llist_get_next(nation_club_list(this, n));
        if item == 0 {
            show_error_at(0xa6bf9c, 0x19e9);
            return 0;
        }
        let club = Club(read_usize(G_CLUB_TABLE).wrapping_add(read_usize(item).wrapping_mul(club::SIZE)));
        if !is_basque_restricted(player, club.addr()) && can_club_bid(player.addr(), club.addr()) && (exe::database_count_club_squad(club, 0, 0) as i8) < 0x32 {
            let sheet = player.player();
            let rep = club.reputation() as i32;
            let c = rep
                .wrapping_sub((sheet.current_reputation() as i32).wrapping_sub(rep).wrapping_abs())
                .wrapping_sub((sheet.potential_ability() as i32).wrapping_mul(50).wrapping_sub(rep).wrapping_abs())
                .wrapping_sub((sheet.current_ability() as i32).wrapping_mul(50).wrapping_sub(rep).wrapping_abs())
                .wrapping_add(0x1388);
            covered!("cpn_4 draw of the nation club");
            let rating = exe::utils_random(c as i16 as i32) as i16;
            if rating > melhor_nota {
                melhor = club.addr();
                melhor_nota = rating;
            }
        }
        i += 1;
        if i as i16 >= read_i16((nation_club_list(this, n)).addr()) {
            break;
        }
    }
    melhor
}

/// Calls the game's CRT realloc.
pub(crate) unsafe fn crt_realloc(p: usize, size: usize) -> usize {
    exe::crt_realloc(p, size as u32)
}

/// Out-of-memory fatal error for the regen lists (player_regen.cpp at the given line): message box and exit like the
/// original.
pub(crate) unsafe fn regen_out_of_memory(row: u32) -> ! {
    show_error_at(0xa6bf9c, row);
    fatal_exit()
}

/// Appends (id, nation, second nation) to the 16-byte spare non-player record list
/// (`player_regen_fields::SPARE_NON_PLAYER_RECORDS`); grows by 250 when the player pool
/// (`player_regen_fields::PLAYER_DATA_POOL_COUNT`/`player_regen_fields::PLAYER_DATA_POOL_CAPACITY`) is full, as in the
/// original.
///
/// `PLAYER_REGEN::add_spare_non_player_record(id, nation, second)`: appends (id +0, nation
/// `regen_pool_item::NATION_ID`, second nation `regen_pool_item::SECOND_NATION_ID`) to the 16-byte list of spare
/// non-player records (`player_regen_fields::SPARE_NON_PLAYER_RECORDS`, count
/// `player_regen_fields::SPARE_NON_PLAYER_COUNT`, capacity `player_regen_fields::SPARE_NON_PLAYER_CAPACITY`; taken out
/// by `remove_spare_non_player_record`). The original decides to grow by the free player pool (count
/// `player_regen_fields::PLAYER_DATA_POOL_COUNT` = capacity `player_regen_fields::PLAYER_DATA_POOL_CAPACITY` − 1), not
/// by this list: kept faithfully. Grows by 250 (realloc `crt_realloc`; out of memory, box 0x9c4 and end,
/// `regen_out_of_memory`).
/// Original: `PLAYER_REGEN::add_spare_non_player_record` (0x7a81a0, player_regen.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn add_spare_non_player_record(this: usize, id: u32, nation: u32, second: u32) {
    covered!("cpk_2 manager reservation");
    if read_usize(this + player_regen_fields::PLAYER_DATA_POOL_COUNT) == read_usize(this + player_regen_fields::PLAYER_DATA_POOL_CAPACITY).wrapping_sub(1) {
        let p = crt_realloc(read_usize(this + player_regen_fields::SPARE_NON_PLAYER_RECORDS), read_usize(this + player_regen_fields::SPARE_NON_PLAYER_CAPACITY).wrapping_add(250) << 4);
        write_usize(this + player_regen_fields::SPARE_NON_PLAYER_RECORDS, p);
        write_usize(this + player_regen_fields::SPARE_NON_PLAYER_CAPACITY, read_usize(this + player_regen_fields::SPARE_NON_PLAYER_CAPACITY).wrapping_add(250));
        if p == 0 {
            regen_out_of_memory(0x9c4);
        }
    }
    let n = read_usize(this + player_regen_fields::SPARE_NON_PLAYER_COUNT);
    let item = read_usize(this + player_regen_fields::SPARE_NON_PLAYER_RECORDS).wrapping_add(n << 4);
    write_usize(item, id as usize);
    write_usize(item + regen_pool_item::NATION_ID, nation as usize);
    write_usize(item + regen_pool_item::SECOND_NATION_ID, second as usize);
    write_usize(this + player_regen_fields::SPARE_NON_PLAYER_COUNT, n.wrapping_add(1)); // one more reserve
}

/// Appends the record fields to the 26-byte pending retirement table (`player_regen_fields::RETIREMENTS`), growing by
/// 250.
///
/// `PLAYER_REGEN::add_retirement(record)`: appends to the 26-byte table of pending retirements
/// (`player_regen_fields::RETIREMENTS`, count `player_regen_fields::RETIREMENT_COUNT`, capacity
/// `player_regen_fields::RETIREMENT_CAPACITY`) the record's fields +0, `club_id()` (dwords), `retirement_entry::REASON`
/// (byte), +8, +0xa, +0x10, +0x12 (words) and `retirement_entry::ANNOUNCED` (byte), in that order (+0xc..+0x10 and
/// +0x14..+0x18 are left as they are). Count = capacity − 1: grows by 250 (realloc `crt_realloc`; out of memory, box
/// 0xa53 and end, `regen_out_of_memory`).
/// Original: `PLAYER_REGEN::add_retirement` (0x7a8510, player_regen.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn add_retirement(this: usize, r: usize) {
    let r = RetirementEntry(r);
    covered!("cpk_2 retirement");
    let cap = read_usize(this + player_regen_fields::RETIREMENT_CAPACITY);
    if read_usize(this + player_regen_fields::RETIREMENT_COUNT) == cap.wrapping_sub(1) {
        let p = crt_realloc(read_usize(this + player_regen_fields::RETIREMENTS), cap.wrapping_add(250).wrapping_mul(26));
        write_usize(this + player_regen_fields::RETIREMENTS, p);
        write_usize(this + player_regen_fields::RETIREMENT_CAPACITY, read_usize(this + player_regen_fields::RETIREMENT_CAPACITY).wrapping_add(250));
        if p == 0 {
            regen_out_of_memory(0xa53);
        }
    }
    let item = || read_usize(this + player_regen_fields::RETIREMENTS).wrapping_add(read_usize(this + player_regen_fields::RETIREMENT_COUNT).wrapping_mul(26));
    write_usize(item(), read_usize(r.addr()));
    write_usize(item() + 4, r.club_id() as usize);
    write_u8(item() + retirement_entry::REASON, r.reason());
    for o in [8usize, 0xa, 0x10, 0x12] {
        write_u16(item() + o, read_u16(r.addr() + o));
    }
    write_u8(item() + retirement_entry::ANNOUNCED, r.announced());
    write_usize(this + player_regen_fields::RETIREMENT_COUNT, read_usize(this + player_regen_fields::RETIREMENT_COUNT).wrapping_add(1)); // one more retirement
}

/// Puts the person into the regen queue (`player_regen_fields::REGEN_QUEUE`) with the dates; the non-player data goes
/// into the spare list (`player_regen_fields::SPARE_NON_PLAYER_RECORDS`, random nation if none) and the player data
/// into the free pool (`player_regen_fields::PLAYER_DATA_POOL`).
///
/// `PLAYER_REGEN::queue_staff_for_regen(entry)`: the person with id +0 of the entry (0x6e-byte record in
/// `G_STAFF_TABLE`) enters the regen queue (12 bytes at `player_regen_fields::REGEN_QUEUE`, count
/// `player_regen_fields::REGEN_QUEUE_COUNT`, capacity `player_regen_fields::REGEN_QUEUE_CAPACITY`: id +0, words +4 and
/// +6 of the entry into `regen_queue_entry::REGEN_DATE` and `regen_queue_entry::REGEN_YEAR`). With non-player data
/// (`non_player()`): in the 16-byte list `player_regen_fields::SPARE_NON_PLAYER_RECORDS` (count
/// `player_regen_fields::SPARE_NON_PLAYER_COUNT`, capacity `player_regen_fields::SPARE_NON_PLAYER_CAPACITY`) their id
/// +0, the nation (`nation()`; without a nation, a random draw `exe::utils_random` among the `G_NATION_COUNT`) +4 and
/// the person's id `regen_pool_item::STAFF_ID`. With player data (`player()`): in the free player pool
/// `player_regen_fields::PLAYER_DATA_POOL` (count `player_regen_fields::PLAYER_DATA_POOL_COUNT`, capacity
/// `player_regen_fields::PLAYER_DATA_POOL_CAPACITY`) their id +0, the nation's +4 (without checking for null, as in the
/// original) and the person's `regen_pool_item::STAFF_ID`. Each list grows by 250 when count = capacity − 1 (realloc
/// `crt_realloc`; boxes 0xaac, 0xac5, 0xae9 and end, `regen_out_of_memory`; in the third the original checks the count,
/// not the pointer: kept faithfully). Id outside 0..=`G_STAFF_COUNT`: as in the original (box 0xa9d).
/// Original: `PLAYER_REGEN::queue_staff_for_regen` (0x7a87d0, player_regen.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn queue_staff_for_regen(this: usize, entry: usize) {
    let id = read_usize(entry) as i32;
    if id < 0 || id > read_i32(G_STAFF_COUNT) {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_QUEUE_STAFF_FOR_REGEN)(this, entry);
    }
    covered!("cpk_2 slot for the regen");
    let person = Staff(read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(staff::SIZE)));
    let cap = read_usize(this + player_regen_fields::REGEN_QUEUE_CAPACITY);
    if read_usize(this + player_regen_fields::REGEN_QUEUE_COUNT) == cap.wrapping_sub(1) {
        let p = crt_realloc(read_usize(this + player_regen_fields::REGEN_QUEUE), cap.wrapping_add(250).wrapping_mul(12));
        write_usize(this + player_regen_fields::REGEN_QUEUE, p);
        write_usize(this + player_regen_fields::REGEN_QUEUE_CAPACITY, read_usize(this + player_regen_fields::REGEN_QUEUE_CAPACITY).wrapping_add(250));
        if p == 0 {
            regen_out_of_memory(0xaac);
        }
    }
    let queue = read_usize(this + player_regen_fields::REGEN_QUEUE).wrapping_add(read_usize(this + player_regen_fields::REGEN_QUEUE_COUNT).wrapping_mul(12));
    write_usize(queue, id as usize);
    write_u16(queue + regen_queue_entry::REGEN_DATE, read_u16(entry + 4));
    write_u16(queue + regen_queue_entry::REGEN_YEAR, read_u16(entry + 6));
    write_usize(this + player_regen_fields::REGEN_QUEUE_COUNT, read_usize(this + player_regen_fields::REGEN_QUEUE_COUNT).wrapping_add(1)); // one more in the regen queue
    if !person.non_player().is_null() {
        if read_usize(this + player_regen_fields::SPARE_NON_PLAYER_COUNT) == read_usize(this + player_regen_fields::SPARE_NON_PLAYER_CAPACITY).wrapping_sub(1) {
            let p = crt_realloc(read_usize(this + player_regen_fields::SPARE_NON_PLAYER_RECORDS), read_usize(this + player_regen_fields::SPARE_NON_PLAYER_CAPACITY).wrapping_add(250) << 4);
            write_usize(this + player_regen_fields::SPARE_NON_PLAYER_RECORDS, p);
            write_usize(this + player_regen_fields::SPARE_NON_PLAYER_CAPACITY, read_usize(this + player_regen_fields::SPARE_NON_PLAYER_CAPACITY).wrapping_add(250));
            if p == 0 {
                regen_out_of_memory(0xac5);
            }
        }
        let item = || read_usize(this + player_regen_fields::SPARE_NON_PLAYER_RECORDS).wrapping_add(read_usize(this + player_regen_fields::SPARE_NON_PLAYER_COUNT) << 4);
        write_usize(item(), read_usize(person.non_player().addr()));
        let nation = person.nation();
        if !nation.is_null() {
            write_usize(item() + 4, read_usize(nation.addr()));
        } else {
            let s = exe::utils_random(read_i32(G_NATION_COUNT));
            write_usize(item() + 4, s as usize);
        }
        write_usize(item() + regen_pool_item::STAFF_ID, read_usize(person.addr()));
        write_usize(this + player_regen_fields::SPARE_NON_PLAYER_COUNT, read_usize(this + player_regen_fields::SPARE_NON_PLAYER_COUNT).wrapping_add(1));
    }
    if !person.player().is_null() {
        let cap = read_usize(this + player_regen_fields::PLAYER_DATA_POOL_CAPACITY);
        if read_usize(this + player_regen_fields::PLAYER_DATA_POOL_COUNT) == cap.wrapping_sub(1) {
            let p = crt_realloc(read_usize(this + player_regen_fields::PLAYER_DATA_POOL), cap.wrapping_add(250) << 4);
            write_usize(this + player_regen_fields::PLAYER_DATA_POOL, p);
            write_usize(this + player_regen_fields::PLAYER_DATA_POOL_CAPACITY, read_usize(this + player_regen_fields::PLAYER_DATA_POOL_CAPACITY).wrapping_add(250));
            if read_usize(this + player_regen_fields::PLAYER_DATA_POOL_COUNT) == 0 {
                regen_out_of_memory(0xae9); // the original checks the count (+0x40), not the pointer
            }
        }
        let item = || read_usize(this + player_regen_fields::PLAYER_DATA_POOL).wrapping_add(read_usize(this + player_regen_fields::PLAYER_DATA_POOL_COUNT) << 4);
        write_usize(item(), read_usize(person.player().addr()));
        write_usize(item() + 4, read_usize(person.nation().addr()));
        write_usize(item() + regen_pool_item::STAFF_ID, read_usize(person.addr()));
        write_usize(this + player_regen_fields::PLAYER_DATA_POOL_COUNT, read_usize(this + player_regen_fields::PLAYER_DATA_POOL_COUNT).wrapping_add(1));
    }
}

/// The 26 bytes `add_retirement` copies from a retirement record (others zeroed).
pub(crate) unsafe fn retirement_entry_fields(r: usize) -> [u8; 26] {
    let mut c = [0u8; 26];
    for (o, n) in [(0usize, 4usize), (4, 4), (8, 2), (0xa, 2), (0x10, 2), (0x12, 2), (0x18, 1), (0x19, 1)] {
        c[o..o + n].copy_from_slice(core::slice::from_raw_parts((r + o) as *const u8, n));
    }
    c
}

/// Removes a pending retirement: finds it (26 bytes), pulls the next ones up (`exe::retire_record_assign`), blanks the
/// last one (person −1) and lowers the count.
///
/// `PLAYER_REGEN::remove_retirement(key)`: searches (bsearch `exe::crt_bsearch` with
/// `player_regen_compare_retirements`) for the pending retirement in the vector (`player_regen_fields::RETIREMENTS`,
/// `player_regen_fields::RETIREMENT_COUNT` records of 26 bytes); found: pulls the following ones up one place
/// (`exe::retire_record_assign`, the record copy), the last one gets person −1 and `retirement_entry::REASON`
/// (`set_reason(0)`), +8, +0xa, +0x10, +0x12 zeroed, and the count goes down. Without re-sorting.
/// Original: `PLAYER_REGEN::remove_retirement` (0x7a8690, player_regen.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn remove_retirement(this: usize, key: usize) {
    let array = read_u32(this + player_regen_fields::RETIREMENTS) as usize;
    let s = exe::crt_bsearch(key, array, read_u32(this + player_regen_fields::RETIREMENT_COUNT), 0x1a, 0x7ae2a0);
    if s == 0 {
        return;
    }
    let mut k = ((s.wrapping_sub(array) as u32 as u64 * 0x4ec4ec4f) >> 35) as i32; // o lugar (/ 26)
    while k < read_u32(this + player_regen_fields::RETIREMENT_COUNT) as i32 - 1 {
        let r = RetirementEntry((read_u32(this + player_regen_fields::RETIREMENTS) as usize).wrapping_add(k as usize * 26));
        exe::retire_record_assign(r, r.addr() + retirement_entry::SIZE);
        k += 1;
    }
    let end = RetirementEntry((read_u32(this + player_regen_fields::RETIREMENTS) as usize).wrapping_add((read_u32(this + player_regen_fields::RETIREMENT_COUNT) as usize).wrapping_mul(26)).wrapping_sub(26));
    write_u32(end.addr(), u32::MAX); // the last one without a person
    end.set_reason(0);
    for o in [8usize, 0xa, 0x10, 0x12] {
        write_u16(end.addr() + o, 0);
    }
    write_u32(this + player_regen_fields::RETIREMENT_COUNT, read_u32(this + player_regen_fields::RETIREMENT_COUNT).wrapping_sub(1));
}

/// Is the person on the shortlist (`is_on_shortlist` on the shortlist manager [[0xde1f54]+0x856], no club)?
pub(crate) unsafe fn player_regen_is_on_shortlist(person: Staff) -> bool {
    exe::shortlist_manager_is_on_shortlist(read_u32(read_u32(G_TRANSFER_MANAGER) as usize + transfer_manager_fields::SHORTLIST_MANAGER) as usize, person, 0) != 0
}

/// Decides whether an unemployed person retires: a weak manager without a club, a player unemployed for over a year, or
/// anyone with no record; fills the entry (reason 0x10, the game date) and returns 1.
///
/// `player_regen_check_unemployed_retirement(person, out)` (stdcall, ret 8): whoever is on the shortlist
/// (`player_regen_is_on_shortlist`) does not retire. With a manager record (`non_player()`): without a club
/// (`club_job()`) and without `national_job()`, with `current_reputation()` of the record below 50 and random(5) = 0.
/// Without it, with a player record (`player()`): off the list (checked again), without a club and without
/// `national_job()`, unemployed for more than 365 days (365 × (year − year of `date_joined_club_year()`) − day of
/// `date_joined_club()` + day) and random(the player's current reputation, `player_sheet::CURRENT_REPUTATION`) < 750.
/// With neither record: always. Retires: the output (26 bytes) receives the person, reason 0x10 (`set_reason`),
/// `set_announced(0)`, the club (`set_club_id`, −1 if none) and the game date at +8 and +0x10; returns 1. Otherwise 0.
/// Original: `player_regen_check_unemployed_retirement` (0x7a8050, player_regen.cpp, stdcall, ret 8).
pub(crate) unsafe extern "stdcall" fn player_regen_check_unemployed_retirement(person: usize, out: usize) -> u8 {
    let out = RetirementEntry(out);
    let person = Staff(person);
    let mut without_record = false;
    if player_regen_is_on_shortlist(person) {
        return 0;
    }
    let manager = NonPlayer((person.non_player().addr() as u32) as usize);
    if manager.is_null() && (person.player().addr() as u32) == 0 {
        without_record = true;
    }
    let aposenta = if !manager.is_null() {
        if (person.club_job().addr() as u32) != 0 || (person.national_job().addr() as u32) != 0 {
            return 0;
        }
        manager.current_reputation() < 50 && friendly_random(5) == 0 // the weak manager
    } else if (person.player().addr() as u32) == 0 {
        false
    } else {
        if player_regen_is_on_shortlist(person) {
            return 0;
        }
        if (person.club_job().addr() as u32) != 0 || (person.national_job().addr() as u32) != 0 {
            false
        } else {
            let years = (read_u16(G_CURRENT_DATE_YEAR) as i32).wrapping_sub(person.date_joined_club_year() as i32);
            let days = years.wrapping_mul(365).wrapping_sub((person.date_joined_club() as i16) as i32).wrapping_add(read_i16(G_CURRENT_DATE) as i32);
            if days > 0x16d {
                let rating = read_i16((person.player().addr() as u32) as usize + player_sheet::CURRENT_REPUTATION) as i32;
                friendly_random(rating) < 0x2ee // the roll by the rating
            } else {
                false
            }
        }
    };
    if !aposenta && !without_record {
        return 0;
    }
    write_u32(out.addr(), read_u32(person.addr()));
    out.set_reason(0x10); // o motivo: desempregado
    out.set_announced(0);
    let club = (person.club_job().addr() as u32) as usize;
    out.set_club_id(if club != 0 { read_u32(club) } else { u32::MAX });
    for o in [8usize, 0x10] {
        write_u16(out.addr() + o, read_u16(G_CURRENT_DATE));
        write_u16(out.addr() + o + 2, read_u16(G_CURRENT_DATE_YEAR));
    }
    1
}

/// copies the regen queue entry (0xc bytes): the person (+0) and the regen date (+4)
/// Original: `regen_queue_entry_assign` (0x7a8b70, player_regen.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn regen_queue_entry_assign(this: usize, source: usize) -> usize {
    write_u32(this, read_u32(source)); // the regen’s person
    let mut local = [0u8; 8];
    exe::date_operator_3d(this + regen_queue_entry::REGEN_DATE, (&mut local) as *mut [u8; 8] as usize, source + regen_queue_entry::REGEN_DATE);
    this
}

/// Picks the non-player with the highest positive score (`exe::player_regen_score_non_player_candidate`) of the nation;
/// with none, of any nation; otherwise 0.
///
/// `player_regen_pick_staff_for_non_player_data(nation)`: among the non-player persons (`G_STAFF_TABLE`, 0x6e bytes,
/// `G_STAFF_COUNT` − 16 of them, count re-read on every loop), the one with the highest positive score from
/// `exe::player_regen_score_non_player_candidate` (random; the incoming ecx goes along, as in the original) of nation
/// `nation` (`staff::NATION`+0 = nation; initial score −1000); with none, the one with the highest positive score of
/// any nation (initial −15000); otherwise 0.
/// Original: `player_regen_pick_staff_for_non_player_data` (0x7a6e60, player_regen.cpp, stdcall, ret 4).
pub(crate) unsafe extern "thiscall" fn player_regen_pick_staff_for_non_player_data(this: usize, nation: usize) -> usize {
    let how_many = || read_i32(G_STAFF_COUNT).wrapping_sub(0x10);
    let base = read_usize(G_STAFF_TABLE);
    let (mut melhor, mut escolhida) = (-1000i32, 0usize);
    let mut i = 0;
    while i < how_many() {
        let p = base + 0x6e * i as usize;
        let n = read_usize(p + staff::NATION);
        if n != 0 && read_usize(n) == nation {
            let s = exe::player_regen_score_non_player_candidate(this, p);
            if s > melhor && s > 0 { // the nation’s best
                melhor = s;
                escolhida = p;
            }
        }
        i += 1;
    }
    if escolhida != 0 {
        return escolhida;
    }
    covered!("donor from any nation");
    melhor = -15000;
    let base = read_usize(G_STAFF_TABLE); // reread, as in the original
    i = 0;
    while i < how_many() {
        let p = base + 0x6e * i as usize;
        let s = exe::player_regen_score_non_player_candidate(this, p);
        if s > melhor && s > 0 {
            melhor = s;
            escolhida = p;
        }
        i += 1;
    }
    escolhida
}

compare_value!(compare_is_on_shortlist, is_on_shortlist, IDX_IS_ON_SHORTLIST, (jogador: usize, clube: usize) -> u32);

/// Removes the 16-byte record with key id from a regen list ([regen+lista], count [regen+quantos]), shifts the rest,
/// marks the last with -1 and decrements the count.
pub(crate) unsafe fn remove_from_list16(regen: usize, id: i32, list: usize, how_many: usize) {
    let n = || read_i32(regen + how_many);
    let base = || read_usize(regen + list);
    let Some(i) = (0..n().max(0) as usize).find(|&i| read_i32(base() + i * 16) == id) else {
        return;
    };
    let mut j = i;
    while (j as i32) < n() - 1 {
        core::ptr::copy((base() + (j + 1) * 16) as *const u8, (base() + j * 16) as *mut u8, 16);
        j += 1;
    }
    let last = (base() as isize + (n() as isize - 1) * 16) as usize;
    for o in [0, 4, 8] {
        write_i32(last + o, -1);
    }
    *((regen + how_many) as *mut i32) -= 1;
}

/// removes a non-player record from the spare list by id
/// Original: `PLAYER_REGEN::remove_spare_non_player_record` (0x7a82b0, player_regen.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn remove_spare_non_player_record(regen: usize, id: i32) {
    remove_from_list16(regen, id, 0x1c, 0x20);
}

/// removes the player data record from the free player pool
/// Original: `PLAYER_REGEN::remove_player_data_from_pool` (0x7a8470, player_regen.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn remove_player_data_from_pool(regen: usize, id: i32) {
    remove_from_list16(regen, id, 0x3c, 0x40);
}

/// Regions of a 16-byte regen list: the whole list and its counter.
pub(crate) unsafe fn list16_regions(regen: usize, list: usize, how_many: usize) -> Vec<(usize, usize)> {
    let n = read_i32(regen + how_many).max(0) as usize;
    let mut r = vec![(regen + how_many, 4)];
    if n > 0 {
        r.push((read_usize(regen + list), n * 16));
    }
    r
}

/// the person's entry in the retirement list (by injury or already announced)
/// Original: `PLAYER_REGEN::find_retirement_entry` (0x7a7ed0, player_regen.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn find_retirement_entry(regen: usize, id: *const i32, by_injury: u32) -> usize {
    let list = read_usize(regen + player_regen_fields::RETIREMENTS);
    let n = read_i32(regen + player_regen_fields::RETIREMENT_COUNT);
    for i in 0..n.max(0) as usize {
        let e = RetirementEntry(list + i * 0x1a);
        if read_i32(e.addr()) == *id {
            let ok = if by_injury != 0 { e.reason() == 8 } else { e.announced() != 0 };
            return if ok { e.addr() } else { 0 };
        }
    }
    0
}

/// Returns the random score of how well the club suits the youth player: reputation against the player's reputation and
/// ability, training facilities, human control and nation.
///
/// The score of `club` for the young `player` (random(score), `exe::utils_random`): 5000 + reputation (`reputation()`)
/// minus its distances to the player's current reputation (player record `player()`: `current_reputation()`) and to his
/// potential and current ability × 50 (`potential_ability()`, `current_ability()`); with `potential_ability()` above
/// random(20) + 100, the training facilities bonus (`training()`, 20 = guaranteed, otherwise random(`training()`) >
/// random(20)); a club of an active competition (`G_COMP_OBJECT_TABLE`) with `potential_ability()` > 50: the human club
/// state (`G_CLUB_STATE_TABLE`) and the reputation against the squad (`squad`); the manager's work with young players
/// (`manager()` → `non_player()`: `youngsters()`, `judging_potential()`) at a computer club
/// (`exe::human_manager_is_human_club`) or the human's bonus; the same nation (`nation()` of the player and of the
/// club) and the Welsh clubs (Swansea, Cardiff, Wrexham) and Scotland (`KEY_NATION_SCOTLAND`). Only the low 16 bits of
/// the score count.
/// Original: `player_regen_score_club_for_player` (0x7ad220, player_regen.cpp, stdcall).
pub(crate) unsafe extern "stdcall" fn player_regen_score_club_for_player(player: usize, club: usize, squad: i8) -> i32 {
    let player = Staff(player);
    let club = Club(club);
    let sheet = player.player();
    let rep = club.reputation() as i32;
    let mut s = rep
        .wrapping_sub((sheet.current_reputation() as i32 - rep).wrapping_abs())
        .wrapping_sub((sheet.potential_ability().wrapping_mul(0x32) as i32 - rep).wrapping_abs())
        .wrapping_sub((sheet.current_ability().wrapping_mul(0x32) as i32 - rep).wrapping_abs())
        .wrapping_add(0x1388);
    let juniores = club.training();
    if sheet.potential_ability() as i32 > exe::utils_random(0x14) + 0x64 {
        if juniores == 0x14 {
            s = s.wrapping_add(rep / 2 + 0x1b58);
        } else if exe::utils_random(juniores as i32) > exe::utils_random(0x14) {
            s = s.wrapping_add(rep / 2 + (juniores as i32 + 0x64) * 50);
        }
    }
    let comp = club.division().addr();
    if sheet.potential_ability() > 0x32 && comp != 0 && *((read_usize(G_COMP_OBJECT_TABLE) + 4 * read_u32(comp) as usize) as *const usize) != 0 {
        let estados = read_usize(G_CLUB_STATE_TABLE);
        if exe::human_manager_is_human_club(club, 1, 1, 0, 0) != 0 && estados != 0 && sheet.potential_ability() > 0x78 {
            let e = read_i8(estados + 6 * read_u32(club.addr()) as usize + 4);
            if e < 0 {
                s = s.wrapping_add(0x1f4);
            } else if e == 0 {
                s = s.wrapping_add(0xfa);
            }
        }
        let r = club.reputation();
        s = s.wrapping_add(if r > 0x1676 {
            if squad < 0x28 { 0x3e8 } else if squad < 0x2d { 0x2ee } else { 0xfa }
        } else if r > 0x109a {
            if squad < 0x23 { 0x2ee } else if squad < 0x28 { 0x1f4 } else { 0xfa }
        } else {
            0xfa
        });
    }
    let manager = club.manager();
    let x = NonPlayer(if !manager.is_null() { manager.non_player().addr() } else { 0 });
    if !x.is_null() && exe::human_manager_is_human_club(club, 1, 1, 0, 0) == 0 {
        s = s.wrapping_add((x.youngsters() as i32 - 10) * 30);
        if sheet.potential_ability() as i32 > exe::utils_random(0x4b) + 0x64 {
            let v = exe::non_player_attribute_rating(x, x.judging_potential()) as i8;
            s = s.wrapping_add((v as i32 - 10) * 60);
        }
        let v = exe::non_player_attribute_rating(x, x.judging_potential()) as i8;
        let t = v as i32 + x.youngsters() as i32 + juniores as i32;
        if t > 0x28 && club.reputation() > 0x109a && sheet.potential_ability() > 0x50 {
            if (s as i16) < club.reputation() {
                s = club.reputation() as i32;
            } else {
                s = s.wrapping_add(0xfa);
            }
        }
    } else if exe::human_manager_is_human_club(club, 1, 1, 0, 0) != 0 {
        s = s.wrapping_add(0xc8);
        if sheet.potential_ability() > 0x78 {
            if juniores == 0x14 && (s as i16) < club.reputation() {
                s = club.reputation() as i32;
            }
            s = s.wrapping_add(juniores as i32 * 25);
        }
    }
    let nation = player.nation();
    let nation_of_club = club.nation().addr();
    if nation == Nation(nation_of_club) {
        s = s.wrapping_add(0x3e8); // same nation
    }
    let g = |a: usize| read_u32(a);
    let (gales, inglaterra, escocia) = (g(KEY_NATION_WALES), g(KEY_NATION_ENGLAND), g(KEY_NATION_SCOTLAND));
    let id = g(club.addr());
    if id == g(0x9d0430) || id == g(0x9d0434) || (id == g(0x9d043c) && !nation.is_null()) {
        let n = g(nation.addr());
        if n == gales {
            s = s.wrapping_add(0x3e8);
        } else if n == inglaterra {
            s = s.wrapping_sub(0x2ee);
        }
    }
    if g(nation_of_club) == escocia {
        let n = g(nation.addr());
        if n == gales || n == inglaterra {
            s = s.wrapping_sub(0x2ee);
        } else if n == escocia {
            s = s.wrapping_add(0x3e8);
        }
    }
    exe::utils_random(s as i16 as i32)
}

/// Copies a pending retirement record: the person, the club, the two dates, the reason and the announced flag.
///
/// Copies the pending retirement (0x1a bytes) from `source`: the person's id (+0) and the club's (`club_id()`), the
/// dates at `retirement_entry::ANNOUNCE_DATE` and `retirement_entry::RETIRE_DATE` (`exe::date_operator_3d`, with the
/// return value in a local), the reason (`reason()`) and the announced flag (`announced()`). Returns `this`.
/// Original: `retire_record_assign` (0x7a8780, player_regen.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn retire_record_assign(this: usize, source: usize) -> usize {
    let source = RetirementEntry(source);
    let this = RetirementEntry(this);
    let mut local = [0u8; 8];
    write_u32(this.addr(), read_u32(source.addr()));
    this.set_club_id(source.club_id());
    exe::date_operator_3d(this.addr() + retirement_entry::ANNOUNCE_DATE, (&mut local) as *mut [u8; 8] as usize, source.addr() + retirement_entry::ANNOUNCE_DATE);
    exe::date_operator_3d(this.addr() + retirement_entry::RETIRE_DATE, (&mut local) as *mut [u8; 8] as usize, source.addr() + retirement_entry::RETIRE_DATE);
    this.set_reason(source.reason()); // motivo
    this.set_announced(source.announced());
    this.addr()
}

/// Returns the injury type of a player_setup.cfg INJURY line by name (76 names, from cold to arthritis); none: 0x4b.
///
/// `player_setup_injury_from_name(name)` (cdecl): the injury type (0..0x4c) by the name of the player_setup.cfg injury
/// line (strncmp with the 76 names, each by its own length, in order; the first that matches); none: 0x4b. Used when
/// applying the lines (0x7a3a10) on game creation. The return value counts through al (the original leaves the rest of
/// eax from strncmp).
/// Original: `player_setup_injury_from_name` (0x7a32e0, player_regen.cpp, cdecl).
pub(crate) unsafe extern "C" fn player_setup_injury_from_name(name: usize) -> u32 {
    for &(n, texto, tipo) in INJURY_NAMES.iter() {
        if strncmp(name, texto, n) == 0 {
            covered!("cqf_9 injury by name");
            return tipo as u32; // cqf9: the injury type by name
        }
    }
    if strncmp(name, 0xa6b3a8, 8) == 0 { 0x4c } else { 0x4b }
}

/// Builds one club list per nation; each club with a division joins its nation's list, except national teams and the
/// clubs at 0x9d00a4/0x9d00a8.
///
/// `PLAYER_REGEN::build_nation_club_lists()`; SEH frame for the `new`. [this] = 14 bytes per nation of `G_NATION_COUNT`
/// (`exe::crt_malloc`, no check), with a new list (`exe::crt_operator_new` of 0xe and `exe::new_llist`; out of memory,
/// 0) per nation; each club of `G_CLUB_COUNT` goes (`exe::llist_add_tail`, the 4 bytes of the id) into its nation's
/// list (`injury_youth_list_nation`: clubs with a division, a nation with +0x11c only with +0x80 up to 2, except
/// national teams and the clubs at 0x9d00a4/0x9d00a8).
/// Original: `PLAYER_REGEN::build_nation_club_lists` (0x7acfc0, player_regen.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn build_nation_club_lists(this: usize) {
    let table = exe::crt_malloc((read_i32(G_NATION_COUNT) as u32).wrapping_mul(14));
    write_usize(this, table);
    let mut i = 0i32;
    while i < read_i32(G_NATION_COUNT) {
        let obj = exe::crt_operator_new(0xe);
        let l = if obj != 0 { exe::new_llist(obj) } else { 0 };
        write_usize(read_usize(this).wrapping_add(i as usize * 4), l);
        i += 1;
    }
    let base = read_usize(G_CLUB_TABLE);
    let mut j = 0i32;
    while j < read_i32(G_CLUB_COUNT) {
        let club = base.wrapping_add((j as usize).wrapping_mul(club::SIZE));
        let nation = injury_youth_list_nation(Club(club));
        if nation != 0 {
            covered!("cpp_9 club in the list");
            let l = read_usize(read_usize(this).wrapping_add((read_i32(nation) as usize).wrapping_mul(4)));
            exe::llist_add_tail(l, club, 4);
        }
        j += 1;
    }
}

/// Processes the regen queue every day: due youth regens are created, and with more than 2 spare coach records, count −
/// 2 of them are given to persons who lend the data.
///
/// `PLAYER_REGEN::process_regen_queue()`, from the day loop (0x5cccd0), every day. The youth regen queue
/// (`player_regen_fields::REGEN_QUEUE`, 12 bytes: +0 id, `regen_queue_entry::REGEN_DATE` date; count
/// `player_regen_fields::REGEN_QUEUE_COUNT`): from the front, while the first one's date is before today
/// (`is_regen_entry_due`) and `player_regen_fields::PLAYER_DATA_POOL_COUNT` is not zero, the youth regen
/// (`exe::player_regen_regenerate_as_youth_player`) of the person (id outside 0..`G_STAFF_COUNT`: box "Invalid staff_id
/// for regen" and end), the following ones move up one place (`exe::regen_queue_entry_assign`) and the last gets id −1.
/// Then, with more than 2 spare coach records (`player_regen_fields::SPARE_NON_PLAYER_RECORDS`, count
/// `player_regen_fields::SPARE_NON_PLAYER_COUNT`), for count − 2 of them: the person lending the data
/// (`exe::player_regen_pick_staff_for_non_player_data` by the first one's nation +4) gets the spare's non-player record
/// (`set_non_player`, `G_NON_PLAYER_TABLE` + 0x44 × id); a former player: the swap of `coaching()`/`coaching_gks()` of
/// the rating 20, the normalisation (`exe::staff_normalise_non_player_attributes`), the job at the computer's club
/// (`place_coach_at_ai_club`) and the three ratings (`non_player::CURRENT_REPUTATION`, +8,
/// `non_player::WORLD_REPUTATION`) by the reputation (`current_reputation()`), +9 and `world_reputation()` of the
/// player record (band above 7500, or random(`ambition()`) and the other two; pow and bands; `rebuilt_coach_rating`); a
/// non-player: random(1500) and random(500). The spare leaves (`exe::player_regen_remove_spare_non_player_record`).
/// Original: `PLAYER_REGEN::process_regen_queue` (0x7a65d0, player_regen.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn process_regen_queue(this: usize) {
    let c0 = read_i32(this + player_regen_fields::REGEN_QUEUE_COUNT);
    let first = read_usize(this + player_regen_fields::REGEN_QUEUE);
    let mut k = 0i32;
    while k < c0 {
        if !is_regen_entry_due(first) || read_i32(this + player_regen_fields::PLAYER_DATA_POOL_COUNT) == 0 {
            break;
        }
        let id = read_i32(first);
        if id < 0 || id > read_i32(G_STAFF_COUNT) {
            let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
            format(0xde1f64, 0xa6bfcc, id);
            show_error_at(0xa6bf9c, 0x36e);
            return;
        }
        covered!("cpw_10 queued regen");
        exe::player_regen_regenerate_as_youth_player(this, read_usize(G_STAFF_TABLE).wrapping_add((id as u32).wrapping_mul(staff::SIZE as u32) as usize));
        let mut j = 0i32;
        while j < read_i32(this + player_regen_fields::REGEN_QUEUE_COUNT) - 1 {
            let e = read_usize(this + player_regen_fields::REGEN_QUEUE).wrapping_add((j as u32).wrapping_mul(0xc) as usize);
            exe::regen_queue_entry_assign(e, e + 0xc);
            j += 1;
        }
        let n = read_i32(this + player_regen_fields::REGEN_QUEUE_COUNT);
        let last = read_usize(this + player_regen_fields::REGEN_QUEUE).wrapping_add((n as u32).wrapping_mul(0xc) as usize).wrapping_sub(0xc);
        write_i32(last, -1);
        write_u16(last + regen_queue_entry::REGEN_DATE, 0);
        write_u16(last + regen_queue_entry::REGEN_YEAR, 0);
        write_i32(this + player_regen_fields::REGEN_QUEUE_COUNT, n - 1); // cpw10: a entrada sai
        k += 1;
    }
    let total = read_i32(this + player_regen_fields::SPARE_NON_PLAYER_COUNT);
    if total <= 2 {
        return;
    }
    for _ in 0..total - 2 {
        let s = Staff(exe::player_regen_pick_staff_for_non_player_data(this, read_usize(read_usize(this + player_regen_fields::SPARE_NON_PLAYER_RECORDS) + 4)));
        if s.is_null() {
            return;
        }
        covered!("cpw_10 reserve manager");
        let nj = NonPlayer(read_usize(G_NON_PLAYER_TABLE).wrapping_add((read_usize(read_usize(this + player_regen_fields::SPARE_NON_PLAYER_RECORDS)) as u32).wrapping_mul(0x44) as usize));
        s.set_non_player(nj);
        let sheet = s.player();
        if sheet.is_null() {
            let a = friendly_random(0x5dc).wrapping_add(1) as u16;
            write_u16(s.non_player().addr() + 8, a);
            let b = friendly_random(0x5dc).wrapping_add(1) as u16;
            write_u16(s.non_player().addr() + non_player::CURRENT_REPUTATION, b);
            let c = friendly_random(0x1f4).wrapping_add(1) as u16;
            write_u16(s.non_player().addr() + non_player::WORLD_REPUTATION, c);
        } else {
            if sheet.goalkeeper() == 0x14 {
                let (cl, dl) = (nj.coaching(), nj.coaching_gks());
                if cl > dl {
                    nj.set_coaching_gks(cl);
                    s.non_player().set_coaching(dl);
                    if friendly_random(4) == 0 {
                        let r = friendly_random(0x32) as u8;
                        write_u8(s.non_player().addr() + non_player::COACHING_GKS, r.wrapping_add(0x32));
                    }
                }
            }
            exe::staff_normalise_non_player_attributes(s);
            place_coach_at_ai_club(s);
            let rep = s.player().current_reputation();
            let (k, teto) = if rep > 0x1d4c {
                (0x9689d8, 0x9690b8)
            } else if friendly_random(s.ambition() as i32) != 0 {
                (0x969040, 0x96d300)
            } else {
                (0x968968, 0x96e288)
            };
            let a = rebuilt_coach_rating(s, rep, None, k, teto);
            write_u16(s.non_player().addr() + non_player::CURRENT_REPUTATION, a);
            let b = rebuilt_coach_rating(s, read_i16(s.player().addr() + 9), None, k, teto);
            write_u16(s.non_player().addr() + 8, b);
            let c = rebuilt_coach_rating(s, s.player().world_reputation(), Some(0x969040), k, teto);
            write_u16(s.non_player().addr() + non_player::WORLD_REPUTATION, c);
        }
        exe::player_regen_remove_spare_non_player_record(this, read_i32(read_usize(this + player_regen_fields::SPARE_NON_PLAYER_RECORDS)) as i32 as u32);
    }
}

/// Returns 1 when the person's id is on the retirement list, otherwise 0.
///
/// `PLAYER_REGEN::is_retirement_pending(person)`: 1 when the person's id (+0) is on the retirement list
/// (`player_regen_fields::RETIREMENTS`, 0x1a-byte records, count `player_regen_fields::RETIREMENT_COUNT` signed),
/// otherwise 0 (and 0 without a person). Used by MANAGER_MANAGER::is_staff_suitable_for_job 0x69fa4b: a non-player who
/// is about to retire is not suitable as a manager.
/// Original: `PLAYER_REGEN::is_retirement_pending` (0x7ae350, player_regen.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn is_retirement_pending(this: usize, person: usize) -> u8 {
    if person == 0 {
        return 0;
    }
    let n = read_usize(this + player_regen_fields::RETIREMENT_COUNT) as i32;
    let id = read_usize(person);
    let mut i = 0i32;
    while i < n {
        if read_usize(read_usize(this + player_regen_fields::RETIREMENTS) + 0x1a * i as usize) == id {
            covered!("cpq_10 retirement scheduled");
            return 1; // is in the list
        }
        i += 1;
    }
    0
}

/// Copies the first spare non-player record entry (id, nation, second nation) to the outputs and removes it from the
/// list.
///
/// `PLAYER_REGEN::take_spare_non_player_record(&id, &nation, &second)`, from contract_manager
/// (`exe::contract_manager_resolve_contract_offers`, two calls): copies the first item of the spare non-player record
/// pool (`player_regen_fields::SPARE_NON_PLAYER_RECORDS`: id, nation and second nation) into the three outputs and
/// removes it from the pool (PLAYER_REGEN::remove_spare_non_player_record
/// `exe::player_regen_remove_spare_non_player_record` with the id read from the output); returns its eax.
/// Original: `PLAYER_REGEN::take_spare_non_player_record` (0x7a6e30, player_regen.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn take_spare_non_player_record(this: usize, id: *mut u32, nation: *mut u32, second: *mut u32) -> u32 {
    covered!("cpt_10 reserved record");
    *id = read_usize(read_usize(this + player_regen_fields::SPARE_NON_PLAYER_RECORDS)) as u32;
    *nation = read_usize(read_usize(this + player_regen_fields::SPARE_NON_PLAYER_RECORDS) + 4) as u32;
    *second = read_usize(read_usize(this + player_regen_fields::SPARE_NON_PLAYER_RECORDS) + 8) as u32;
    exe::player_regen_remove_spare_non_player_record(this, *id)
}

/// bits 4..5 of byte +0xb of the person's 79-byte record
/// Original: `record_bits` (0x7a7eb0, player_regen.cpp, stdcall).
pub(crate) unsafe extern "stdcall" fn record_bits(p: *const i32) -> u8 {
    let r = read_usize(G_STAFF_STATE_TABLE).wrapping_add((*p as usize).wrapping_mul(staff_state::SIZE));
    (read_u8(r + staff_state::STATUS_FLAGS) >> 4) & 3
}

