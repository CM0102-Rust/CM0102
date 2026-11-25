// ---------- rx08b_1: the coach's report on a player: the career stage, the days he needs, the verdicts and the news item (coach.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\coach.cpp" (the error boxes).
pub(crate) const COACH_CPP: usize = 0x99b3e0;

/// The coach report news item.
pub(crate) const COACH_REPORT_NEWS: u32 = 0x1391;

/// The player's position roles scored for the report: as he plays now and as he could become.
const COACH_ROLE_NOW: u32 = 0x1e82;
const COACH_ROLE_POTENTIAL: u32 = 0x1f02;

/// f64 = 1/48 (no name in `rdata`): the weight of pace and acceleration in a young player's potential.
const COACH_PACE_WEIGHT: usize = 0x9689c8;

/// rule: a player's career stage for a coach's report: 4 (veteran) past 115% of his position's age limit (33 for a
/// goalkeeper, 31 for a defender or defensive midfielder, 29 for a midfielder or attacking midfielder, 28 otherwise) or
/// past 34; 1 under 18; 2 under 24; 16 (established) with international caps or a reputation above 6000; 8 otherwise
///
/// `this.get_player_career_stage(player)`: no player or no player sheet (`staff::PLAYER`): 0. The age (`staff::AGE`,
/// signed byte) against the limit × 1.15 (`rdata::F64_1_15`; the limit by the sheet's 20s: `player_sheet::GOALKEEPER`,
/// then `DEFENDER` or `DEFENSIVE_MIDFIELDER`, then `MIDFIELDER` or `ATTACKING_MIDFIELDER`), then the bands; the caps are
/// `staff::INT_APPS`, the reputation `player_sheet::CURRENT_REPUTATION` (signed). Only `al`; `this` is not read.
/// Original: `coach_get_player_career_stage` (0x48af30, coach.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn coach_get_player_career_stage(_this: usize, player: usize) -> u8 {
    let player = Staff(player);
    if player.is_null() || player.player().is_null() {
        return 0;
    }
    covered!("rx08b_1 career stage");
    let sheet = player.player().addr();
    let best = |k: usize| read_i8(sheet + k) == 20;
    let limit: i32 = if best(player_sheet::GOALKEEPER) {
        0x21
    } else if best(player_sheet::DEFENDER) || best(player_sheet::DEFENSIVE_MIDFIELDER) {
        0x1f
    } else if best(player_sheet::MIDFIELDER) || best(player_sheet::ATTACKING_MIDFIELDER) {
        0x1d
    } else {
        0x1c
    };
    let age = read_i8(player.addr() + staff::AGE) as i32;
    if limit as f64 * read_f64(rdata::F64_1_15) < age as f64 || age > 0x22 {
        4
    } else if age < 0x12 {
        1
    } else if age < 0x18 {
        2
    } else if read_u8(player.addr() + staff::INT_APPS) != 0 || read_i16(sheet + player_sheet::CURRENT_REPUTATION) > 0x1770 {
        0x10
    } else {
        8
    }
}

/// rule: how many days a coach needs with a player at the club to judge him: none for a star (current ability above
/// 160, or potential above 160 with current above 100); a player on loan at a club, 5, 10 or 15 by the coach's ability
/// (above 140, above 80, the rest); a youth player (contract type 5), 0 to 9 days; otherwise 30 plus 0 to 34 plus that
/// base
///
/// `this.get_assessment_days(coach, player)`: no coach or no non-player sheet (`staff::NON_PLAYER`): the coach.cpp box
/// of line 0xabc and 180; no player or no player sheet: the box of line 0xac1 and 180. The base by the coach's
/// `non_player::CURRENT_ABILITY` (signed word). On loan (`exe::contract_manager_get_loan_contract`) with a contract at a
/// club (`database_club_of_contract`): the base. Otherwise the "random" part is (coach id + player id) mod 10 for a
/// youth contract (`exe::contract_manager_get_contract`, kept in `G_CONTRACT_LOOKUP_RESULT`, its type & 0x3f = 5) and
/// mod 35 otherwise (signed). `this` is not read.
/// Original: `coach_get_assessment_days` (0x48ba10, coach.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn coach_get_assessment_days(_this: usize, coach: usize, player: usize) -> i32 {
    let (coach, player) = (Staff(coach), Staff(player));
    if coach.is_null() || coach.non_player().is_null() {
        show_error_at(COACH_CPP, 0xabc);
        return 0xb4;
    }
    if player.is_null() || player.player().is_null() {
        show_error_at(COACH_CPP, 0xac1);
        return 0xb4;
    }
    covered!("rx08b_1 assessment days");
    let sheet = player.player().addr();
    let ability = read_i16(sheet + player_sheet::CURRENT_ABILITY);
    if ability > 0xa0 || (read_i16(sheet + player_sheet::POTENTIAL_ABILITY) > 0xa0 && ability > 0x64) {
        return 0;
    }
    let coach_ability = read_i16(coach.non_player().addr() + non_player::CURRENT_ABILITY);
    let base = if coach_ability > 0x8c { 5 } else if coach_ability <= 0x50 { 0xf } else { 10 }; // rx08b_1: the base by the coach's ability
    let manager = read_usize(G_CONTRACT_MANAGER);
    if exe::contract_manager_get_loan_contract(manager, player) != 0 && database_club_of_contract(player) != 0 {
        return base;
    }
    let ids = read_i32(coach.addr()).wrapping_add(read_i32(player.addr()));
    let contract = Contract(exe::contract_manager_get_contract(manager, player));
    write_usize(G_CONTRACT_LOOKUP_RESULT, contract.addr());
    if !contract.is_null() && contract.contract_type() & 0x3f == 5 {
        return ids.wrapping_rem(10);
    }
    ids.wrapping_rem(0x23) + base + 0x1e
}

/// When the verdict word has the bit `from`, takes it off and puts `to` on.
unsafe fn coach_demote(word: usize, from: u16, to: u16) {
    let v = read_u16(word);
    if v & from != 0 {
        write_u16(word, (v ^ from) | to);
    }
}

/// The young player's potential score (under 24): a fresh score in his potential role (`COACH_ROLE_POTENTIAL`) with
/// its own two draws, cut for a weak current score (below 3250: × 0.9; below 5000 past 19: × 0.95), scaled by pace and
/// acceleration below 24 (× (sum / 48 + 0.5)) and by the build below 50 (× (build / 100 + 0.5)), then raised for the
/// strongest builds (above 75 with pace above 30: × 1.2; above 65 or 50 with pace above 24: × 1.1 or × 1.05); each
/// step truncated.
unsafe fn coach_potential_score(coach: Staff, player: Staff, sheet: usize, now: i32, (age, pace, build): (i8, i8, i8)) -> i32 {
    let draw = random(500) - random(500);
    let mut potential = draw + lineup_slot_score(player.addr(), sheet, 0, 0, COACH_ROLE_POTENTIAL, 0, 0, coach.addr(), -1, 0, 0, 0, 0, 0, 0);
    let scale = |v: i32, k: f64| ftol(v as f64 * k);
    if now < 0xcb2 {
        potential = scale(potential, read_f64(rdata::F64_0_9));
    } else if now < 0x1388 && age > 0x13 {
        potential = scale(potential, read_f64(rdata::F64_0_95));
    }
    if pace < 0x18 {
        potential = scale(potential, pace as f64 * read_f64(COACH_PACE_WEIGHT) + read_f64(rdata::F64_0_5));
    }
    if build < 0x32 {
        potential = scale(potential, build as f64 * read_f64(rdata::F64_0_01) + read_f64(rdata::F64_0_5));
    }
    if build > 0x4b && pace > 0x1e {
        potential = scale(potential, read_f64(rdata::F64_1_2));
    } else if build > 0x41 && pace > 0x18 {
        potential = scale(potential, read_f64(rdata::F64_1_1));
    } else if build > 0x32 && pace > 0x18 {
        potential = scale(potential, read_f64(rdata::F64_1_05));
    }
    potential
}

/// The scores against the level the coach's club looks for (`target`), by the player's squad status at the club
/// (high nibble of `contract::SQUAD_STATUS`): 1 measures against 1.2 × target, 2 and 3 against the target, 4 and 5 the
/// current score against 0.8 × target and the potential against the target; a score below its bar is raised by 10%
/// (truncated). Other statuses leave them.
unsafe fn coach_squad_status_bars(status: i8, target: i32, now: &mut i32, potential: &mut i32) {
    let t = target as f64;
    let (bar_now, bar_potential) = match status {
        1 => (t * read_f64(rdata::F64_1_2), t * read_f64(rdata::F64_1_2)),
        2 | 3 => (t, t),
        4 | 5 => (t * read_f64(rdata::F64_0_8), t),
        _ => return,
    };
    if (*now as f64) < bar_now {
        *now = ftol(*now as f64 * read_f64(rdata::F64_1_1));
    }
    if (*potential as f64) < bar_potential {
        *potential = ftol(*potential as f64 * read_f64(rdata::F64_1_1));
    }
}

/// The current score's cuts for the player's standing: a reputation above 6250 needs 1.2 × target (× 1.05 below
/// it); a coach better than the player (ability above (player id mod 40) + 80) judges a current ability above 145
/// below 0.9 × target with × 1.15, and otherwise one above 125 below 0.8 × target with × 1.1 (truncated).
unsafe fn coach_standing_cuts(coach: Staff, player: Staff, sheet: usize, target: i32, now: &mut i32) {
    let t = target as f64;
    if read_i16(sheet + player_sheet::CURRENT_REPUTATION) > 0x186a && (*now as f64) < t * read_f64(rdata::F64_1_2) {
        *now = ftol(*now as f64 * read_f64(rdata::F64_1_05));
    }
    let ability = read_i16(sheet + player_sheet::CURRENT_ABILITY);
    let better_coach = || {
        let np = coach.non_player().addr();
        np != 0 && read_i16(np + non_player::CURRENT_ABILITY) as i32 > read_i32(player.addr()).wrapping_rem(0x28) + 0x50
    };
    if ability > 0x91 && better_coach() && (*now as f64) < t * read_f64(rdata::F64_0_9) {
        *now = ftol(*now as f64 * read_f64(rdata::F64_1_15));
        return;
    }
    if ability > 0x7d && better_coach() && (*now as f64) < t * read_f64(rdata::F64_0_8) {
        *now = ftol(*now as f64 * read_f64(rdata::F64_1_1));
    }
}

/// The verdict bit of a score against the target: 8 above 1.2 × target, 4 above it, `close` above 0.9 × target, and
/// for the rest `None` (the callers split it).
unsafe fn coach_band(score: i32, target: f64, close: u16) -> Option<u16> {
    let s = score as f64;
    if s > target * read_f64(rdata::F64_1_2) {
        Some(8)
    } else if s > target {
        Some(4)
    } else if s > target * read_f64(rdata::F64_0_9) {
        Some(close)
    } else {
        None
    }
}

/// rule: a coach's verdict on a player, two flag words: the current one by his score in his role (8 a star for the
/// club, 4 good enough, 0x200 nearly, 2 squad player, 1 not good enough) and the potential one for players up to 24
/// (8, 4, 0x100, 2 or 1 by the time at the club and the age; 0x10 past 24); then softened for a reserve player with a
/// resilient coach, a player left out, a player in form, a small squad; with marks for not knowing him yet (0x10 and
/// 0x20), transfer listed (0x40), not a player at the club (0x80), on loan or youth (0x40) and versatile (0x80)
///
/// `this.assess_player(coach, player, verdict, potential)`: both words zero. Days needed (`coach_get_assessment_days`)
/// against the days since the player joined (`exe::date_days_since` from `G_CURRENT_DATE`): fewer, verdict | 0x10 and
/// potential | 0x20. The coach's club (`staff::CLUB_JOB`); none: the coach.cpp box of line 0x8bd. The player's contract
/// with his club (`database_club_of_contract`, `exe::contract_manager_get_contract_for_club`); none: nothing more. The
/// target: `player_search_get_club_target_rating` of the coach's club, at least 1750. The original then works out an
/// ambition-weighted expectation it never reads; its two calls stay (`update_club_target_reputation` on the shortlist
/// manager and the target rating forced to it). The current score: two draws of 500 apart plus `lineup_slot_score` in
/// `COACH_ROLE_NOW`; the potential (`coach_potential_score`, under 24; 0 otherwise); `coach_squad_status_bars`;
/// `coach_standing_cuts`. Verdict: `coach_band` with 0x200, else 2 above 0.8 × target and 1 below. Potential: past 24
/// the word is 0x10 (it drops the 0x20); otherwise `coach_band` with 0x100, else above 0.8 × target 2 when he has been
/// 710 days or more at the club or is 20 or older and 0x100 otherwise, and below it 2 for under 1050 days under 20 or
/// under 710 days under 23, 1 otherwise. Then the softening (`coach_soften_verdicts`).
/// Original: `coach_assess_player` (0x48aff0, coach.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn coach_assess_player(this: usize, coach: usize, player: usize, verdict: usize, potential: usize) {
    let (coach, player) = (Staff(coach), Staff(player));
    write_u16(verdict, 0);
    write_u16(potential, 0);
    let needed = coach_get_assessment_days(this, coach.addr(), player.addr());
    let days = exe::date_days_since(G_CURRENT_DATE, read_u32(player.addr() + staff::DATE_JOINED_CLUB), read_u32(player.addr() + staff::DATE_JOINED_CLUB_LEAP_YEAR));
    if days < needed {
        write_u8(verdict, read_u8(verdict) | 0x10);
        write_u8(potential, read_u8(potential) | 0x20);
    }
    let club = coach.club_job().addr();
    if club == 0 {
        show_error_at(COACH_CPP, 0x8bd);
        return;
    }
    let at_club = database_club_of_contract(player);
    let sheet = player.player().addr();
    let contract = exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), player, at_club);
    if contract == 0 {
        return;
    }
    covered!("rx08b_1 player assessed");
    let byte = |k: usize| read_u8(sheet + k);
    let age = read_i8(player.addr() + staff::AGE);
    let build = [player_sheet::WORK_RATE, player_sheet::TEAMWORK, player_sheet::STRENGTH, player_sheet::STAMINA, player_sheet::JUMPING]
        .iter().fold(0u8, |s, &k| s.wrapping_add(byte(k))) as i8;
    let pace = byte(player_sheet::PLAYER_PACE).wrapping_add(byte(player_sheet::ACCELERATION)) as i8;
    let target = player_search_get_club_target_rating(club, core::ptr::null_mut(), core::ptr::null()).max(0x6d6);
    let shortlists = read_usize(read_usize(G_TRANSFER_MANAGER) + transfer_manager_fields::SHORTLIST_MANAGER);
    let forced = update_club_target_reputation(shortlists, club);
    player_search_get_club_target_rating(club, core::ptr::null_mut(), &forced as *const i32 as *const i16);
    let draw = random(500) - random(500);
    let mut now = draw + lineup_slot_score(player.addr(), sheet, 0, 0, COACH_ROLE_NOW, 0, 0, coach.addr(), -1, 0, 0, 0, 0, 0, 0);
    let mut potential_score = if age < 0x18 { coach_potential_score(coach, player, sheet, now, (age, pace, build)) } else { 0 };
    let status = (read_u8(contract + contract::SQUAD_STATUS) >> 4) as i8;
    coach_squad_status_bars(status, target, &mut now, &mut potential_score);
    if sheet != 0 {
        coach_standing_cuts(coach, player, sheet, target, &mut now);
    }
    let t = target as f64;
    let current = coach_band(now, t, 0x200).unwrap_or(if now as f64 > t * read_f64(rdata::F64_0_8) { 2 } else { 1 }); // rx08b_1: the current verdict
    write_u8(verdict, read_u8(verdict) | current as u8);
    write_u8(verdict + 1, read_u8(verdict + 1) | (current >> 8) as u8);
    if age > 0x18 {
        write_u16(potential, 0x10);
    } else {
        let joined = days;
        let bit = coach_band(potential_score, t, 0x100).unwrap_or_else(|| {
            if potential_score as f64 > t * read_f64(rdata::F64_0_8) {
                if joined >= 0x2c6 || age >= 0x14 { 2 } else { 0x100 }
            } else if (joined < 0x41a && age < 0x14) || (joined < 0x2c6 && age < 0x17) {
                2
            } else {
                1
            }
        });
        write_u8(potential, read_u8(potential) | bit as u8);
        write_u8(potential + 1, read_u8(potential + 1) | (bit >> 8) as u8);
    }
    coach_soften_verdicts(coach, player, sheet, Contract(contract), club, (verdict, potential));
}

/// The verdicts softened after the bands, in the original's order:
/// - squad status 3 with a coach of pressure or determination below 10, or status 1 or 2: a "1" verdict becomes 2
///   (status 1 and 2: the current one also gets 0x100 with a coach of determination and pressure above 10), and a "1"
///   potential becomes 2 (status 3 only under that coach);
/// - left out of more than one match (`contract::MATCHES_LEFT_OUT`): both "1" become 2;
/// - in form: the player's average rating over his last appearances (`exe::playerstats_get_stat` of `G_PLAYER_STATS`,
///   kind 6, 0x11, 6.5) against the league's (the stats' +0x10 / +0x18, 6.8 when there are none), with more than two
///   appearances (kind 6, 1, kept as an f32): more than 0.5 above, verdict 1 → 0x200 or 2 → 4, potential 1 → 0x100 or
///   2 → 4; more than 0.25 above, both "1" become 2;
/// - with more than five appearances and the contract's +0x3e below 3, or left out of more than two matches: a
///   rating above 7 turns a "1" or "2" verdict into 0x20; otherwise a "1" verdict becomes 0x102 with a coach of
///   determination above 10 and pressure above 15, 0x20 with determination and pressure below 10 and 6, 2 otherwise;
/// - transfer listed (`staff_is_transfer_listed`): verdict | 0x40; not a player at the club (`staff::JOB_FOR_CLUB` other
///   than 11): verdict | 0x80;
/// - on loan, or a youth contract (`exe::contract_manager_get_contract` type & 0x3f = 5, kept in
///   `G_CONTRACT_LOOKUP_RESULT`): potential | 0x40; the eleven position ratings (goalkeeper to free role, not wing back)
///   summing above 95: potential | 0x80;
/// - a squad of fewer than 15 (`database_count_club_squad` of the coach's club): both "1" become 2.
unsafe fn coach_soften_verdicts(coach: Staff, player: Staff, sheet: usize, contract: Contract, club: usize, (verdict, potential): (usize, usize)) {
    let pressure = read_i8(coach.addr() + staff::PRESSURE);
    let determination = read_i8(coach.addr() + staff::DETERMINATION);
    match (read_u8(contract.addr() + contract::SQUAD_STATUS) >> 4) as i8 {
        3 => {
            if pressure < 10 || determination < 10 {
                coach_demote(verdict, 1, 2);
                coach_demote(potential, 1, 2);
            }
        }
        1 | 2 => {
            if read_u16(verdict) & 1 != 0 {
                coach_demote(verdict, 1, 2);
                if determination > 10 && pressure > 10 {
                    write_u16(verdict, read_u16(verdict) | 0x100);
                }
            }
            coach_demote(potential, 1, 2);
        }
        _ => {}
    }
    if read_i8(contract.addr() + contract::MATCHES_LEFT_OUT) > 1 {
        coach_demote(verdict, 1, 2);
        coach_demote(potential, 1, 2);
    }
    let stats = read_usize(G_PLAYER_STATS);
    let appearances = exe::playerstats_get_stat(stats, sheet, 6, 1, 6.5) as f32;
    let rating = exe::playerstats_get_stat(stats, sheet, 6, 0x11, 6.5);
    if appearances > read_f32(rdata::F32_2) {
        let norm = || {
            let games = read_f64(stats + 0x18); // the league's rated appearances
            if games != read_f64(rdata::F64_0) { read_f64(stats + 0x10) / games } else { read_f64(rdata::F64_6_8) }
        };
        if norm() + read_f64(rdata::F64_0_5) < rating {
            covered!("rx08b_1 player in form");
            if read_u16(verdict) & 1 != 0 {
                coach_demote(verdict, 1, 0x200);
            } else {
                coach_demote(verdict, 2, 4);
            }
            if read_u16(potential) & 1 != 0 {
                coach_demote(potential, 1, 0x100);
            } else {
                coach_demote(potential, 2, 4);
            }
        } else if norm() + read_f64(rdata::F64_0_25) < rating {
            coach_demote(verdict, 1, 2);
            coach_demote(potential, 1, 2);
        }
    }
    let grumbles = read_i8(contract.addr() + 0x3e); // the contract's +0x3e (unhappiness) [inference]
    if (appearances > read_f32(rdata::F32_5) && grumbles < 3) || read_i8(contract.addr() + contract::MATCHES_LEFT_OUT) > 2 {
        if rating > read_f64(rdata::F64_7) {
            if read_u16(verdict) & 1 != 0 {
                coach_demote(verdict, 1, 0x20);
            } else {
                coach_demote(verdict, 2, 0x20);
            }
        } else if determination > 10 && pressure > 0xf {
            coach_demote(verdict, 1, 0x102);
        } else if determination < 10 && pressure < 6 {
            coach_demote(verdict, 1, 0x20);
        } else {
            coach_demote(verdict, 1, 2);
        }
    }
    if staff_is_transfer_listed(player.addr(), 0) != 0 {
        write_u8(verdict, read_u8(verdict) | 0x40);
    }
    if read_u8(player.addr() + staff::JOB_FOR_CLUB) != 0xb {
        write_u8(verdict, read_u8(verdict) | 0x80);
    }
    let manager = read_usize(G_CONTRACT_MANAGER);
    let loan_or_youth = exe::contract_manager_get_loan_contract(manager, player) != 0 || {
        let c = Contract(exe::contract_manager_get_contract(manager, player));
        write_usize(G_CONTRACT_LOOKUP_RESULT, c.addr());
        !c.is_null() && c.contract_type() & 0x3f == 5
    };
    if loan_or_youth {
        write_u8(potential, read_u8(potential) | 0x40);
    }
    let positions = (player_sheet::GOALKEEPER..=player_sheet::FREE_ROLE).filter(|&k| k != player_sheet::WING_BACK)
        .map(|k| read_i8(sheet + k) as i32).sum::<i32>();
    if positions > 0x5f {
        write_u8(potential, read_u8(potential) | 0x80);
    }
    if (database_count_club_squad(club, 0, 0) as i8) < 0xf {
        coach_demote(verdict, 1, 2);
        coach_demote(potential, 1, 2);
    }
}

/// rule: a coach's report on a player goes to the coach's club as a news item: who, how settled the player is (1
/// after 1000 days with loyalty above 10, 4 after (21 − adaptability) × 5 + 90 days, 2 before), his career stage, the
/// two verdicts, his traits, strengths and weaknesses, positions, age, his club's reputation and whether he has a
/// future transfer, is absent without leave or has a staff role
///
/// `this.post_report_news(coach, player)`: nothing without the coach, his club (`staff::CLUB_JOB`), the player or the
/// player's club. The days since the player joined (`exe::date_days_since` from `G_CURRENT_DATE`), his
/// `staff::LOYALTY` and `staff::ADAPTABILITY` (signed); `coach_get_player_career_stage`; `coach_assess_player` (the two
/// words); `exe::player_search_assess_player_traits(player, coach, ..)` (two words). A news item 0x1391
/// (`exe::new_newsitem`) with the slots (`exe::newsitem_set_slot`): 0 the coach's id, 1..3 the ids (+0x33) of his
/// name records, 4 the player's id, 5..7 his name records, 8 how settled, 9 the stage, 10 and 11 the verdict words,
/// 12 and 13 the trait words (all signed), 14 and 15 the strengths and weaknesses for the coach's club
/// (`staff_get_player_strength_flags`, `staff_get_player_weakness_flags`), 16 the position mask
/// (`exe::formation_get_position_mask`, signed word), 17 the age, 18 the player's club's reputation (signed word), 19
/// the coach's club's id, 20 a future transfer (`exe::transfer_manager_get_future_transfer`), 21 absent without leave
/// (`exe::awol_is_player_awol`, its `al`), 22 a non-player sheet; sent to the coach's club's manager
/// (`exe::news_send_to_club_manager`).
/// Original: `coach_post_report_news` (0x487c70, coach.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn coach_post_report_news(this: usize, coach: usize, player: usize) {
    let (coach, player) = (Staff(coach), Staff(player));
    if coach.is_null() || coach.club_job().is_null() || player.is_null() || player.club_job().is_null() {
        return;
    }
    covered!("rx08b_1 coach report posted");
    let days = exe::date_days_since(G_CURRENT_DATE, read_u32(player.addr() + staff::DATE_JOINED_CLUB), read_u32(player.addr() + staff::DATE_JOINED_CLUB_LEAP_YEAR));
    let settle_days = (0x15 - read_i8(player.addr() + staff::ADAPTABILITY) as i32) * 5 + 0x5a;
    let settled: i8 = if days > 0x3e8 && read_i8(player.addr() + staff::LOYALTY) > 10 { 1 } else if days >= settle_days { 4 } else { 2 }; // rx08b_1: how settled
    let stage = coach_get_player_career_stage(this, player.addr()) as i8;
    let (mut verdict, mut potential, mut traits) = (0u16, 0u16, [0i16; 2]);
    coach_assess_player(this, coach.addr(), player.addr(), &mut verdict as *mut u16 as usize, &mut potential as *mut u16 as usize);
    exe::player_search_assess_player_traits(player, coach, traits.as_mut_ptr(), traits.as_mut_ptr().add(1));
    let mut news = [0u32; 0x3a]; // the local news item (0xe0 bytes in the original)
    let n = news.as_mut_ptr() as usize;
    exe::new_newsitem(n, COACH_REPORT_NEWS, 0);
    let name_id = |s: Staff, field: usize| read_u32(read_usize(s.addr() + field) + 0x33); // the name record's id
    let club = coach.club_job().addr();
    let sheet = player.player().addr();
    let slots = [read_u32(coach.addr()), name_id(coach, staff::FIRST_NAME), name_id(coach, staff::SECOND_NAME), name_id(coach, staff::COMMON_NAME),
        read_u32(player.addr()), name_id(player, staff::FIRST_NAME), name_id(player, staff::SECOND_NAME), name_id(player, staff::COMMON_NAME),
        settled as i32 as u32, stage as i32 as u32, verdict as i16 as i32 as u32, potential as i16 as i32 as u32, traits[0] as i32 as u32, traits[1] as i32 as u32,
        staff_get_player_strength_flags(player.addr(), club),
        staff_get_player_weakness_flags(player.addr(), club),
        exe::formation_get_position_mask(sheet) as i16 as i32 as u32,
        read_i8(player.addr() + staff::AGE) as i32 as u32,
        read_i16(player.club_job().addr() + club::REPUTATION) as i32 as u32,
        read_u32(club),
        (exe::transfer_manager_get_future_transfer(read_usize(G_TRANSFER_MANAGER), player, 0) != 0) as u32,
        (exe::awol_is_player_awol(read_usize(G_AWOL), player, 0) != 0) as u32,
        !player.non_player().is_null() as u32];
    for (k, v) in slots.into_iter().enumerate() {
        exe::newsitem_set_slot(n, k as i8, v);
    }
    exe::news_send_to_club_manager(read_usize(G_NEWS), n, club);
}

/// Comparer for `coach_get_player_career_stage`: side by side on the answer (`al`).
pub(crate) unsafe extern "thiscall" fn compare_coach_get_player_career_stage(this: usize, player: usize) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(%%I1%%)(this, player);
    let r = coach_get_player_career_stage(this, player);
    SCOREBOARD.record(o == r, || format!("coach_get_player_career_stage (coach report): original {} rust {}", o, r));
    o
}

/// Comparer for `coach_get_assessment_days`: side by side on the answer and on the contract lookup it leaves; a
/// missing coach or player runs only the original (the box).
pub(crate) unsafe extern "thiscall" fn compare_coach_get_assessment_days(this: usize, coach: usize, player: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> i32>(%%I3%%);
    if coach == 0 || player == 0 || Staff(coach).non_player().is_null() || Staff(player).player().is_null() {
        return o(this, coach, player);
    }
    let (a, b, same, mo, mr) = crate::comparison::side_by_side(&[(G_CONTRACT_LOOKUP_RESULT, 4)], || o(this, coach, player), || coach_get_assessment_days(this, coach, player));
    SCOREBOARD.record(same && a == b, || format!("coach_get_assessment_days (coach report): original {} rust {} {:?}", a, b, first_differences(&mo, &mr)));
    a
}

/// Comparer for `coach_assess_player`: side by side on the two verdict words, the contract lookup and the random
/// generator (the copy's draws not counted).
pub(crate) unsafe extern "thiscall" fn compare_coach_assess_player(this: usize, coach: usize, player: usize, verdict: usize, potential: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize)>(%%I2%%);
    let mut regions = crate::rng::STATE_REGIONS.to_vec();
    regions.extend([(verdict, 2), (potential, 2), (G_CONTRACT_LOOKUP_RESULT, 4)]);
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, coach, player, verdict, potential),
        || crate::rng::without_counting(|| coach_assess_player(this, coach, player, verdict, potential)));
    SCOREBOARD.record(same, || format!("coach_assess_player (coach report, player {}): {:?}", read_i32(player), first_differences(&mo, &mr)));
}

/// Comparer for `coach_post_report_news`: it sends a news item (it cannot run twice), so only the original runs; its
/// parts are compared by their own comparers and the item by playing comparing.
pub(crate) unsafe extern "thiscall" fn compare_coach_post_report_news(this: usize, coach: usize, player: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I0%%)(this, coach, player);
}
