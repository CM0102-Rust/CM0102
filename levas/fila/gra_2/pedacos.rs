// ---------- the daily staff update (game.cpp): the pieces ported so far (NOT HOOKED, see FORA.md) ----------
//
// `game_daily_staff_update` (0x5ce2d0, game.cpp, cdecl, 30,544 bytes) runs every day over the staff table. These are
// the pieces of it read from the disassembly and ported byte by byte; the routine is not hooked until every piece is
// written (FORA.md lists the two still missing: the player's daily happiness and grievances, 0x5ce406..0x5d352d, and the
// three-days-a-year ability development, 0x5d3d09..0x5d5771). Compiled over HEAD 1b33a4990; never run.

/// The game.cpp source path (the error box).
const GAME_CPP_PATH: usize = 0x9cd2a4;
/// 1 until the day's first development builds the two ability tables (`G_PLAYER_ABILITY_VALUES_SORTED`,
/// `G_PLAYER_DATA_PTRS_BY_ABILITY`); byte.
const G_ABILITY_TABLES_PENDING: usize = 0x9cd236;
/// qsort comparer of two 16-bit abilities (game.cpp, cdecl).
const FN_COMPARE_ABILITY_VALUES: usize = 0x5d6200;
/// qsort comparer of two player data records by ability, then potential (`compare_players_by_ability`, hooked).
const FN_COMPARE_PLAYERS_BY_ABILITY: usize = 0x5d6220;

/// Adds `delta` to the contract's morale (`contract::MORALE`, signed byte), the sum held to -100..100: the clamp the
/// daily staff update repeats inline some sixty times (the sum in 32 bits; inside the range the byte gets
/// morale + delta).
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), e.g. 0x5cf8ff..0x5cf920.
#[allow(dead_code)]
pub(crate) unsafe fn staff_day_add_morale(contract: usize, delta: i32) {
    let m = read_i8(contract + contract::MORALE);
    let sum = (m as i32).wrapping_add(delta);
    if sum > 100 {
        write_u8(contract + contract::MORALE, 100);
    } else if sum < -100 {
        write_u8(contract + contract::MORALE, 0x9c);
    } else {
        write_u8(contract + contract::MORALE, (m as u8).wrapping_add(delta as u8));
    }
}

/// The day of a person without a player sheet (0x5d3367..0x5d352d). Without a non-player sheet nothing changes
/// (an unattached person goes through `staff_day_unattached_decline`, which then has nothing to do). With a
/// contract whose squad status (low nibble of `contract::SQUAD_STATUS`) is under 10, one draw in ftol(100 -
/// adaptability × 3.5) raises that nibble by one (the high nibble kept) and the person learns the language of the club's
/// nation (`nation::FIRST_LANGUAGE`): the first step adds it (`PlayerStats::add_language`), the next ones up to 10
/// improve it (`improve_language`), fluency 1, the list sorted. Unattached (no club): `staff_day_unattached_decline`.
/// At a club, except in the club job 5 [inference: the job byte `staff::JOB_FOR_CLUB`, 5 left alone]: the current
/// reputation becomes ftol(((club reputation + 50 × current ability) / 2) × 0.05 + current reputation × 0.95) (the
/// half truncated and taken to 16 bits); under 40 years old the current ability moves by -random(3) + random(6) three
/// days in four, else -random(6) + random(6); from 40 only one day in 20, by -random(6) + random(6) (the other days
/// keep it and skip the normalisation); capped at the potential and the attributes normalised
/// (`staff_normalise_non_player_attributes`).
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d3367..0x5d352d.
#[allow(dead_code)]
pub(crate) unsafe fn staff_day_non_player(p: usize, club: usize, contract: usize) {
    let np = read_usize(p + staff::NON_PLAYER);
    if np == 0 {
        if club == 0 {
            staff_day_unattached_decline(p, 0);
        }
        return;
    }
    if contract != 0 && (read_u8(contract + contract::SQUAD_STATUS) & 0xf) < 10 {
        let n = ftol(read_f64(rdata::F64_100) - read_i8(p + staff::ADAPTABILITY) as f64 * read_f64(rdata::F64_3_5));
        if exe::utils_random(n) == 0 {
            covered!("gra_2 non-player learns a language");
            let s = read_u8(contract + contract::SQUAD_STATUS);
            if s & 0xf < 10 {
                write_u8(contract + contract::SQUAD_STATUS, ((s.wrapping_add(1) ^ s) & 0xf) ^ s);
            }
            let level = read_u8(contract + contract::SQUAD_STATUS) & 0xf;
            let language = read_u8(read_usize(club + club::NATION) + nation::FIRST_LANGUAGE) as u32;
            if level == 1 {
                exe::playerstats_add_language(read_usize(G_PLAYER_STATS), p, language, 1, 1);
            } else if level <= 10 {
                improve_language(read_usize(G_PLAYER_STATS), p, language, 1, 1);
            }
        }
    }
    if club == 0 {
        staff_day_unattached_decline(p, 0);
        return;
    }
    if read_u8(p + staff::JOB_FOR_CLUB) == 5 {
        return;
    }
    let half = ((read_i16(club + club::REPUTATION) as i32).wrapping_add(50 * read_i16(np + non_player::CURRENT_ABILITY) as i32) / 2) as i16;
    let current = read_i16(np + non_player::CURRENT_REPUTATION);
    let r = ftol(half as f64 * read_f64(rdata::F64_0_05) + current as f64 * read_f64(rdata::F64_0_95));
    write_u16(np + non_player::CURRENT_REPUTATION, r as u16);
    let ability = np + non_player::CURRENT_ABILITY;
    if read_i8(p + staff::AGE) < 40 {
        let down = if exe::utils_random(4) != 0 { exe::utils_random(3) } else { exe::utils_random(6) };
        let a = read_u16(ability).wrapping_sub(down as u16);
        write_u16(ability, a.wrapping_add(exe::utils_random(6) as u16));
    } else {
        if exe::utils_random(20) != 0 {
            return;
        }
        covered!("gra_2 non-player over 40 ability moves");
        let down = exe::utils_random(6);
        let a = read_u16(ability).wrapping_sub(down as u16);
        write_u16(ability, a.wrapping_add(exe::utils_random(6) as u16));
    }
    let potential = read_i16(np + non_player::POTENTIAL_ABILITY);
    if read_i16(ability) > potential {
        write_i16(ability, potential);
    }
    exe::staff_normalise_non_player_attributes(p);
}

/// An unattached person loses reputation (0x5d3539..0x5d377a; also the end of `staff_day_non_player`). The days
/// since joining (or leaving) the last club, (year - year joined) × 365 - day joined + today's day, against a grace of
/// 60 days plus 2 per cap (`staff::INT_APPS`): within it nothing. A player (`pd`, the player sheet) under 525 days loses
/// random(days - grace), halved (truncated, 16 bits) when the caps beat random(30); with a loss the current reputation
/// becomes the larger of 12.5 × current ability and current - loss, the home reputation the larger of 12.5 × current
/// ability and the new current - loss, the world reputation the larger of 5 × current ability and the new current -
/// loss. A non-player sheet of someone without a national job, between 361 and 524 days: loss ftol(random(days - 360)
/// × 0.25), drawn again as random(loss) when the nation's league is not loaded (`nation::LEAGUE_SELECTED`), halved when
/// the caps beat random(30); with a loss, half of it comes off the non-player current reputation (floor 35 × current
/// ability), the home reputation becomes the larger of 35 × current ability and the new current - half, the world
/// reputation the larger of 5 × current ability and the new current - half.
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d3539..0x5d377a.
#[allow(dead_code)]
pub(crate) unsafe fn staff_day_unattached_decline(p: usize, pd: usize) {
    let days = (read_u16(G_CURRENT_DATE_YEAR) as i32)
        .wrapping_sub(read_u16(p + staff::DATE_JOINED_CLUB_YEAR) as i32)
        .wrapping_mul(365)
        .wrapping_sub(read_i16(p + staff::DATE_JOINED_CLUB) as i32)
        .wrapping_add(read_i16(G_CURRENT_DATE) as i32);
    let grace = read_u8(p + staff::INT_APPS) as i32 * 2 + 60;
    if days <= grace {
        return;
    }
    if pd != 0 && days < 525 {
        let mut loss = exe::utils_random(days - grace);
        if read_u8(p + staff::INT_APPS) as i32 > exe::utils_random(30) {
            loss = (loss as i16 as i32) / 2;
        }
        if (loss as i16) > 0 {
            covered!("gra_2 unattached player loses reputation");
            let loss = loss as i16 as i32;
            // fcom: the floor 12.5 x ability stays only when above the value (ties and NaN take the value)
            let floored = |x: i32| {
                let floor = read_i16(pd + player_sheet::CURRENT_ABILITY) as f64 * read_f64(rdata::F64_12_5);
                ftol(if floor > x as f64 { floor } else { x as f64 })
            };
            let current = floored((read_i16(pd + player_sheet::CURRENT_REPUTATION) as i32).wrapping_sub(loss));
            write_u16(pd + player_sheet::CURRENT_REPUTATION, current as u16);
            let home = floored((current as i16 as i32).wrapping_sub(loss));
            write_u16(pd + player_sheet::HOME_REPUTATION, home as u16);
            let x = (read_i16(pd + player_sheet::CURRENT_REPUTATION) as i32).wrapping_sub(loss);
            let five = read_i16(pd + player_sheet::CURRENT_ABILITY) as i32 * 5;
            write_u16(pd + player_sheet::WORLD_REPUTATION, if five > x { five } else { x } as u16);
        }
    }
    let np = read_usize(p + staff::NON_PLAYER);
    if np == 0 || read_u32(p + staff::NATIONAL_JOB) != 0 || days <= 360 || days >= 525 {
        return;
    }
    let mut loss = ftol(exe::utils_random(days - 360) as f64 * read_f64(rdata::F64_0_25));
    if read_u8(read_usize(p + staff::NATION) + nation::LEAGUE_SELECTED) == 0 {
        loss = exe::utils_random(loss as i16 as i32);
    }
    if read_u8(p + staff::INT_APPS) as i32 > exe::utils_random(30) {
        loss = (loss as i16 as i32) / 2;
    }
    if (loss as i16) <= 0 {
        return;
    }
    covered!("gra_2 unattached non-player loses reputation");
    let half = (loss as i16 as i32) / 2;
    let at_least = |floor: i32, x: i32| if floor > x { floor } else { x } as u16;
    let ability = |np: usize| read_i16(np + non_player::CURRENT_ABILITY) as i32;
    let current = |np: usize| read_i16(np + non_player::CURRENT_REPUTATION) as i32;
    write_u16(np + non_player::CURRENT_REPUTATION, at_least(35 * ability(np), current(np).wrapping_sub(half)));
    write_u16(np + non_player::HOME_REPUTATION, at_least(35 * ability(np), current(np).wrapping_sub(half)));
    write_u16(np + non_player::WORLD_REPUTATION, at_least(5 * ability(np), current(np).wrapping_sub(half)));
}

/// A player on a club's training schedule may come out of training below par (0x5d3786..0x5d38f9). The player's
/// schedule (`TRAINING_MANAGER::get_player_schedule`) must exist with its byte +0x15 set [inference: training today],
/// at a club, without a non-player sheet and not injured (`Injury::get_player_injury_status` other than 2). The odds
/// are ftol(professionalism³ + 30) (the game's pow), plus random(morale) with a contract, at least 20; one draw in them
/// (random below 1): with the injury object, no current injury (`Injury::get_player_injury`) and not AWOL, the match
/// condition (`Injury::get_match_condition`) drops by random(40) × 100 / (random(natural fitness), at least 1)
/// (`Injury::set_match_condition`, 16 bits); a drop of at least 1000 + random(750) leaving the condition at 8000 or
/// less, when the player is not away on international duty
/// (`NationalTeams::get_international_duty_club` with today's date), is reported (`AWOL::report_poor_fitness`, with the
/// club job).
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d3786..0x5d38f9.
#[allow(dead_code)]
pub(crate) unsafe fn staff_day_training_condition(p: usize, club: usize, contract: usize) {
    let schedule = exe::training_manager_get_player_schedule(read_usize(G_TRAINING_MANAGER), p);
    if club == 0
        || schedule == 0
        || read_u8(schedule + 0x15) == 0
        || read_usize(p + staff::NON_PLAYER) != 0
        || exe::injury_get_player_injury_status(read_usize(G_INJURY), p) == 2
    {
        return;
    }
    let mut odds = ftol(game_pow(read_i8(p + staff::PROFESSIONALISM) as f64, read_f64(rdata::F64_3)) + read_f64(rdata::F64_30));
    if contract != 0 {
        odds = odds.wrapping_add(exe::utils_random(read_i8(contract + contract::MORALE) as i32));
    }
    if odds < 20 {
        odds = 20;
    }
    if exe::utils_random(odds) >= 1 {
        return;
    }
    let injury = read_usize(G_INJURY);
    if injury == 0 || exe::injury_get_player_injury(injury, p) != 0 || exe::awol_is_player_awol(read_usize(G_AWOL), p, 0) != 0 {
        return;
    }
    covered!("gra_2 tired after training");
    let pd = read_usize(p + staff::PLAYER);
    let fitness = exe::utils_random(read_i8(pd + player_sheet::NATURAL_FITNESS) as i32) as i8;
    let divisor = if fitness < 1 { 1 } else { fitness as i32 };
    let drop = exe::utils_random(40).wrapping_mul(100) / divisor;
    let condition = get_match_condition(read_usize(G_INJURY), p, 0) as i16;
    let new = condition.wrapping_sub(drop as i16);
    exe::injury_set_match_condition(read_usize(G_INJURY), p, new, 0);
    let limit = exe::utils_random(750) + 1000;
    if (drop as i16 as i32) < limit || new > 8000 {
        return;
    }
    if exe::nationalteams_get_international_duty_club(read_usize(G_NATIONAL_TEAMS), p, G_CURRENT_DATE, club, 0, 0) != 0 {
        return;
    }
    covered!("gra_2 poor fitness reported");
    report_poor_fitness(read_usize(G_AWOL), p, read_usize(p + staff::CLUB_JOB));
}

/// The two tables of the day's ability redistribution, built by the day's first development (0x5d393e..0x5d3a0c):
/// while `G_ABILITY_TABLES_PENDING` is set, the current abilities of the whole player data table (+5 of each 0x46-byte
/// record) are copied to a new array (`crt_malloc`, 2 bytes each) and sorted (qsort, 16-bit compare 0x5d6200), and a
/// second array (4 bytes each) gets the address of each player data record in table order; the flag is cleared. A
/// failed allocation is the game.cpp error box at line 0x18d7 or 0x18e6 and the game ends (cleanup, exit(-1)). The
/// arrays are freed by whoever reads them [inference: not in this routine].
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d393e..0x5d3a0c.
#[allow(dead_code)]
pub(crate) unsafe fn staff_day_build_ability_tables() {
    if read_u8(G_ABILITY_TABLES_PENDING) == 0 {
        return;
    }
    let values = exe::crt_malloc((read_i32(G_PLAYER_DATA_COUNT) as u32).wrapping_mul(2));
    write_usize(G_PLAYER_ABILITY_VALUES_SORTED, values);
    if values == 0 {
        fatal_error_at(GAME_CPP_PATH, 0x18d7);
    }
    let table = read_usize(G_PLAYER_DATA_TABLE);
    let mut i = 0;
    while i < read_i32(G_PLAYER_DATA_COUNT) {
        write_u16(read_usize(G_PLAYER_ABILITY_VALUES_SORTED) + 2 * i as usize, read_u16(table + 0x46 * i as usize + 5));
        i += 1;
    }
    exe::crt_qsort(values, read_u32(G_PLAYER_DATA_COUNT), 2, FN_COMPARE_ABILITY_VALUES);
    let pointers = exe::crt_malloc(read_u32(G_PLAYER_DATA_COUNT).wrapping_mul(4));
    write_usize(G_PLAYER_DATA_PTRS_BY_ABILITY, pointers);
    if pointers == 0 {
        fatal_error_at(GAME_CPP_PATH, 0x18e6);
    }
    let mut i = 0;
    while i < read_i32(G_PLAYER_DATA_COUNT) {
        write_usize(read_usize(G_PLAYER_DATA_PTRS_BY_ABILITY) + 4 * i as usize, table + 0x46 * i as usize);
        i += 1;
    }
    write_u8(G_ABILITY_TABLES_PENDING, 0);
}

/// The age at which a player's development turns, by his best position (0x5d3a92..0x5d3af0): 31, or 28 with
/// attacker or attacking midfielder 15+, 29 with midfielder 15+, 30 with defensive midfielder 15+ (the first that
/// matches); then 33 for a goalkeeper 20, else 31 with sweeper 15+ or any defender rating. Defect of the original:
/// "defender != 0" (not "15+") sends nearly every outfield player back to 31, undoing the midfield and attack ages.
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d3a92..0x5d3af0.
#[allow(dead_code)]
pub(crate) unsafe fn staff_day_development_age(pd: usize) -> i8 {
    let mut age = 31;
    if read_i8(pd + player_sheet::ATTACKER) >= 15 || read_i8(pd + player_sheet::ATTACKING_MIDFIELDER) >= 15 {
        age = 28;
    } else if read_i8(pd + player_sheet::MIDFIELDER) >= 15 {
        age = 29;
    } else if read_i8(pd + player_sheet::DEFENSIVE_MIDFIELDER) >= 15 {
        age = 30;
    }
    if read_i8(pd + player_sheet::GOALKEEPER) == 20 {
        age = 33;
    } else if read_i8(pd + player_sheet::SWEEPER) >= 15 || read_i8(pd + player_sheet::DEFENDER) != 0 {
        age = 31; // defect of the original: any defender rating
    }
    age
}

/// One of a player's three development days a year, one time in ten: his personality drifts (0x5d3af0..0x5d3d09).
/// Ambition +1 under 25 (below 20, one in three) or -1 over 25 (above 1, one in three). Then, each one time in four,
/// half up and half down (random(2)), between 1 and 20: loyalty (at 20 going up, the club job becomes a favoured club,
/// `staff_add_favoured_club` with random(2); at 1 going down, it stops being one, `staff_remove_favoured_club`),
/// determination, pressure (up by 2 for a player over 22 with professionalism above 10, pressure under 10 and current
/// reputation above 6750; else up by 1 only with professionalism above 5; down by 2 from 3 for a player under 21 with
/// current reputation above 5250, else down by 1 only with professionalism under 15), professionalism (up by 2 from
/// under 10 past 28 years old, else 1; down by 2 from 3 for a player under 23 at a club job whose reputation is below
/// his current reputation (above 4250), else 1), temperament and sportsmanship. `age` is the age byte, `club` the
/// club job (`staff::CLUB_JOB`).
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d3af0..0x5d3d09.
#[allow(dead_code)]
pub(crate) unsafe fn staff_day_personality_drift(p: usize, age: i8, club: usize) {
    use exe::utils_random as random;
    if random(10) != 0 {
        return;
    }
    covered!("gra_2 personality drift");
    let ambition = p + staff::AMBITION;
    if age < 25 {
        if read_i8(ambition) < 20 && random(3) == 0 {
            write_u8(ambition, read_u8(ambition).wrapping_add(1));
        }
    } else if age > 25 && read_i8(ambition) > 1 && random(3) == 0 {
        write_u8(ambition, read_u8(ambition).wrapping_sub(1));
    }
    // loyalty
    if random(4) == 0 {
        let up = random(2) == 0;
        let v = read_i8(p + staff::LOYALTY);
        if up {
            if v < 20 {
                write_i8(p + staff::LOYALTY, v + 1);
            } else {
                let r = random(2);
                exe::staff_add_favoured_club(p, club, r as u32);
            }
        } else if v > 1 {
            write_i8(p + staff::LOYALTY, v - 1);
        } else {
            staff_remove_favoured_club(p, club);
        }
    }
    // determination
    if random(4) == 0 {
        let up = random(2) == 0;
        let v = read_i8(p + staff::DETERMINATION);
        if up {
            if v < 20 {
                write_i8(p + staff::DETERMINATION, v + 1);
            }
        } else if v > 1 {
            write_i8(p + staff::DETERMINATION, v - 1);
        }
    }
    // pressure
    if random(4) == 0 {
        let up = random(2) == 0;
        let v = read_i8(p + staff::PRESSURE);
        let pd = read_usize(p + staff::PLAYER);
        let professionalism = read_i8(p + staff::PROFESSIONALISM);
        if up {
            if v < 20 {
                if pd != 0 && age > 22 && professionalism > 10 && v < 10 && read_i16(pd + player_sheet::CURRENT_REPUTATION) > 0x1a5e {
                    write_i8(p + staff::PRESSURE, v + 2);
                } else if professionalism > 5 {
                    write_i8(p + staff::PRESSURE, v + 1);
                }
            }
        } else if v > 1 {
            if v > 2 && age < 21 && pd != 0 && read_i16(pd + player_sheet::CURRENT_REPUTATION) > 0x1482 {
                write_i8(p + staff::PRESSURE, v - 2);
            } else if professionalism < 15 {
                write_i8(p + staff::PRESSURE, v - 1);
            }
        }
    }
    // professionalism
    if random(4) == 0 {
        let up = random(2) == 0;
        let v = read_i8(p + staff::PROFESSIONALISM);
        if up {
            if v < 20 {
                write_i8(p + staff::PROFESSIONALISM, if v < 10 && age > 28 { v + 2 } else { v + 1 });
            }
        } else if v > 1 {
            let pd = read_usize(p + staff::PLAYER);
            let fall_two = club != 0 && pd != 0 && v > 2 && age < 23 && {
                let reputation = read_i16(pd + player_sheet::CURRENT_REPUTATION);
                reputation > 0x109a && read_i16(club + club::REPUTATION) < reputation
            };
            write_i8(p + staff::PROFESSIONALISM, if fall_two { v - 2 } else { v - 1 });
        }
    }
    // temperament, sportsmanship
    for field in [staff::TEMPERAMENT, staff::SPORTSMANSHIP] {
        if random(4) == 0 {
            let up = random(2) == 0;
            let v = read_i8(p + field);
            if up {
                if v < 20 {
                    write_i8(p + field, v + 1);
                }
            } else if v > 1 {
                write_i8(p + field, v - 1);
            }
        }
    }
}

/// A player may join the clubs' shortlists (0x5d578d..0x5d5864, every day for every player, after the development).
/// Interest: current reputation above 5000 + random(3000), or caps above random(10) with world reputation above
/// 3000; then, if the player's state byte has no low nibble set (`G_STAFF_STATE_TABLE` +0xb) [inference: not a
/// retired or virtual player] and he has grievances (`contract::GRIEVANCES`) or is listed or asked to leave
/// (`staff_is_transfer_listed_or_requested`), one time in 17 he is added. Otherwise random(current reputation) above
/// random(2,000,000) adds him, and so does one time in 20 at a club without a division (`club::DIVISION` 0). Added:
/// `SHORTLIST_MANAGER::add_player_to_club_shortlists` (the transfer manager's shortlist manager, the player, 1).
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d578d..0x5d5864.
#[allow(dead_code)]
pub(crate) unsafe fn staff_day_shortlist_check(p: usize, pd: usize, club: usize, contract: usize) {
    use exe::utils_random as random;
    let interest = read_i16(pd + player_sheet::CURRENT_REPUTATION) as i32 > random(3000) + 5000
        || (read_u8(p + staff::INT_APPS) as i32 > random(10) && read_i16(pd + player_sheet::WORLD_REPUTATION) > 3000);
    let add = 'decide: {
        if interest && read_u8(read_usize(G_STAFF_STATE_TABLE) + read_u32(p) as usize * 0x4f + 0xb) & 0xf == 0 {
            let unsettled = (contract != 0 && read_u32(contract + contract::GRIEVANCES) != 0) || exe::staff_is_transfer_listed_or_requested(p, 0usize) != 0;
            if unsettled && random(0x11) == 0 {
                covered!("gra_2 unsettled player shortlisted");
                break 'decide true;
            }
        }
        let a = random(read_i16(pd + player_sheet::CURRENT_REPUTATION) as i32);
        if a > random(2_000_000) {
            break 'decide true;
        }
        club != 0 && read_u32(club + club::DIVISION) == 0 && random(0x14) == 0
    };
    if add {
        covered!("gra_2 player shortlisted");
        let manager = transfer_manager_get_shortlist_manager(read_usize(G_TRANSFER_MANAGER));
        exe::shortlist_manager_add_player_to_club_shortlists(manager, p, 1);
    }
}

/// The end of the day, when some development moved an ability (0x5d587a..0x5d58e0): the player data records (the
/// array `G_PLAYER_DATA_PTRS_BY_ABILITY`) are sorted by current ability, then potential (qsort,
/// `compare_players_by_ability`), and the abilities sorted at the start of the day (`G_PLAYER_ABILITY_VALUES_SORTED`)
/// are handed back in that order, each capped at the record's potential: the spread of abilities stays the day's
/// [inference: what the original keeps by this].
/// Piece of: `game_daily_staff_update` (0x5ce2d0, game.cpp), 0x5d587a..0x5d58e0.
#[allow(dead_code)]
pub(crate) unsafe fn staff_day_hand_back_abilities() {
    exe::crt_qsort(read_usize(G_PLAYER_DATA_PTRS_BY_ABILITY), read_u32(G_PLAYER_DATA_COUNT), 4, FN_COMPARE_PLAYERS_BY_ABILITY);
    let mut i = 0usize;
    while (i as i32) < read_i32(G_PLAYER_DATA_COUNT) {
        let record = read_usize(read_usize(G_PLAYER_DATA_PTRS_BY_ABILITY) + 4 * i);
        write_u16(record + player_sheet::CURRENT_ABILITY, read_u16(read_usize(G_PLAYER_ABILITY_VALUES_SORTED) + 2 * i));
        let potential = read_i16(record + player_sheet::POTENTIAL_ABILITY);
        if read_i16(record + player_sheet::CURRENT_ABILITY) > potential {
            write_i16(record + player_sheet::CURRENT_ABILITY, potential);
        }
        i += 1;
    }
}
