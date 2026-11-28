// ---------- batch 347: setting up a match player (match_pl.cpp) ----------

/// The fixture club of `side` (0 home, otherwise away).
unsafe fn fixture_club_of_side(fixture: Fixture, side: i8) -> Club {
    if side != 0 { fixture.away_club() } else { fixture.home_club() }
}

/// Whether `club` names `other` among its three rivals.
unsafe fn has_rival(club: Club, other: usize) -> bool {
    other == club.rival1() || other == club.rival2() || other == club.rival3()
}

/// First part of the setup: the engine, the person, the index in the side's block (`match_player::SIDE_INDEX`), the
/// commentary id (`slot`) and the player record. A person without one (a non-player placed into the team) gets the
/// message "Non Playing member of staff placed into team" formatted into the global buffer 0xde1f64 (`FN_CRT_SPRINTF`,
/// never shown) and a random player record (random(`G_PLAYER_DATA_COUNT`), 70 bytes each). The match form is 10 for a
/// person outside the staff table, else the record's morale. Returns his contract with his club
/// (`exe::contract_manager_get_contract_for_club`), looked up only for a person in the staff table (or none); the club
/// comes from `match_player::SIDE` before this setup writes it (the original reads the side left by the previous
/// match: a defect, see the routine's header).
unsafe fn setup_identity(j: MatchPlayer, engine: Engine, person: Staff, side: u32, index: u32, slot: u8) -> usize {
    j.set_engine(engine);
    j.set_staff(person);
    write_u8(j.addr() + match_player::SIDE_INDEX, index as u8);
    j.set_commentary_id(slot);
    if !person.player().is_null() {
        j.set_player(person.player());
    } else {
        covered!("sx15_1 non-player in the team");
        let club = fixture_club_of_side(Fixture(engine.r#match()), side as u8 as i8);
        let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
        sprintf(0xde1f64, 0xa15ff4, read_u32(person.addr()), person.first_name(), read_usize(person.addr() + staff::SECOND_NAME), club.addr() + 4);
        let k = random(read_i32(G_PLAYER_DATA_COUNT));
        j.set_player(PlayerSheet(read_usize(G_PLAYER_DATA_TABLE).wrapping_add((k as usize).wrapping_mul(70))));
    }
    let outside = |p: Staff| read_i32(p.addr()) >= read_i32(G_STAFF_COUNT);
    if !j.staff().is_null() && outside(j.staff()) {
        j.set_match_form(10);
    } else {
        j.set_match_form(read_i8(j.player().addr() + player_sheet::PLAYER_MORALE));
    }
    if !person.is_null() && outside(person) {
        return 0;
    }
    let club = fixture_club_of_side(Fixture(engine.r#match()), j.side());
    exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), person, club)
}

/// Second part: the reputations, the club (`match_player::CLUB`, from the fixture by `side`), the condition (stamina and
/// its ceiling from `exe::injury_get_condition`), the weekly training (`exe::training_manager_update_player_training`),
/// the position (−2 out injured; `slot` 0..10 with the starting position from `exe::cm3_match_engine_get_player_position`;
/// otherwise −1 on the bench, with the match rating 6400 and no starting position), the minutes (−1), the confidence
/// (2 × (25 − temperament)), the temper (+10 when the opponent favours him, −20 when it dislikes him, −10 when the
/// opponent has his club as a rival) and the feeling for his own club (10, +30 favoured, −10 disliked, +5 when the
/// opponent has his club as a rival). Returns the opponent club.
unsafe fn setup_condition_and_feelings(j: MatchPlayer, engine: Engine, person: Staff, side: u32, slot: u8) -> Club {
    let sheet = j.player();
    j.set_new_current_reputation(sheet.current_reputation());
    j.set_new_home_reputation(sheet.home_reputation());
    j.set_new_world_reputation(sheet.world_reputation());
    let fixture = Fixture(engine.r#match());
    let (own, opponent) = if side as u8 != 0 { (fixture.away_club(), fixture.home_club()) } else { (fixture.home_club(), fixture.away_club()) };
    j.set_club(own.addr());
    let condition = exe::injury_get_condition(read_usize(G_INJURY), person, 0) as i32 as f32;
    j.set_stamina(condition);
    write_f32(j.addr() + match_player::STAMINA_CEILING, condition);
    exe::training_manager_update_player_training(read_usize(G_TRAINING_MANAGER), person, 0);
    j.set_stamina_boost(0.0);
    if exe::injury_get_player_injury_status(read_usize(G_INJURY), person) == 2 {
        j.set_position(-2);
    } else if (0..=10).contains(&(slot as i8)) {
        j.set_position(slot as i8);
        let start = if j.engine().is_null() { 0 } else { exe::cm3_match_engine_get_player_position(j.engine(), j) };
        j.set_starting_position(start);
    } else {
        j.set_position(-1);
        j.set_match_rating(0x1900);
        j.set_starting_position(0);
    }
    j.set_minute_on(-1);
    j.set_injury_minute(-1);
    j.set_minute_off(-1);
    j.set_minute_left_pitch(-1);
    let person = j.staff();
    j.set_confidence(((25 - person.temperament() as i32) * 2) as f32);
    j.set_match_temper(0);
    if exe::club_favours_staff(person, opponent) != 0 {
        j.set_match_temper(j.match_temper() + 10);
    } else if exe::club_dislikes_staff(person, opponent) != 0 {
        j.set_match_temper(j.match_temper() - 20);
    }
    if has_rival(opponent, j.club()) {
        j.set_match_temper(j.match_temper() - 10);
    }
    j.set_own_club_feeling(10);
    if exe::club_favours_staff(person, j.club()) != 0 {
        j.set_own_club_feeling(j.own_club_feeling() + 30);
    } else if exe::club_dislikes_staff(person, j.club()) != 0 {
        j.set_own_club_feeling(j.own_club_feeling() - 10);
    }
    if has_rival(opponent, j.club()) {
        j.set_own_club_feeling(j.own_club_feeling() + 5); // sx15_1: the derby feeling
    }
    opponent
}

/// The international or club record shown by the commentary (`match_player::DEBUT` and the five i16 of
/// `match_player::GOAL_COUNTS`, −1000 = none): a national side gets the debut (no caps) and the international goals in
/// the second word; a club in a selected league with a competition object (Japan only from 2002) gets the league
/// appearances (`exe::playerstats_get_stat`-free `get_club_league_stat` field 2) and career goals when the fixture is a
/// league division, the debut (no appearances for the club), the goals for the club and, for a player whose club job is
/// in season, his recent rating (`exe::playerstats_get_stat` 6/2 with 6.5 when stat 6/1 reaches 3.0). Only with the
/// match watched live, his side's mark on the fixture, a person in the staff table and not a friendly; otherwise only a
/// national side (not a second national team) in a fixture with the home mark gets its record. Everything else: none.
unsafe fn setup_record_for_commentary(j: MatchPlayer, engine: Engine, person: Staff, side: u8) {
    let counts = |k: usize| j.addr() + match_player::GOAL_COUNTS + 2 * k;
    let none = || {
        j.set_debut(0);
        for k in 0..5 {
            write_i16(counts(k), -1000);
        }
    };
    let national = || {
        j.set_debut((person.int_apps() == 0) as i8);
        write_i16(counts(0), -1000);
        write_u16(counts(1), person.int_goals() as u16);
        write_i16(counts(3), -1000);
        write_i16(counts(4), -1000);
        write_i16(counts(2), -1000);
    };
    let club = Club(j.club());
    let fixture = Fixture(engine.r#match());
    let mark = if side == 0 { fixture.home_side_mark() as u8 } else { fixture.away_side_mark() as u8 };
    let in_table = person.is_null() || read_i32(person.addr()) < read_i32(G_STAFF_COUNT);
    if engine.live_match() == 0 || mark == 0 || !in_table || fixture.category() == 1 {
        if exe::database_is_national_team(club) != 0 && exe::club_is_national_team(club) == 0 && fixture.home_side_mark() != 0 {
            return national();
        }
        return none();
    }
    if exe::club_is_national_team(club) != 0 {
        return none();
    }
    if exe::database_is_national_team(club) != 0 {
        return national();
    }
    let nation = club.nation();
    let division = read_usize(club.addr() + club::DIVISION);
    if read_i8(nation.addr() + nation::LEAGUE_SELECTED) == 0 || division == 0 || read_usize(read_usize(G_COMP_OBJECT_TABLE) + 4 * read_u32(division) as usize) == 0 {
        return none();
    }
    if read_u32(nation.addr()) == read_u32(KEY_NATION_JAPAN) && read_u16(G_CURRENT_DATE_YEAR) < 2002 {
        return none();
    }
    covered!("sx15_1 club record");
    write_i16(counts(1), -1000);
    let comp_id = fixture.comp_id();
    if comp_id > -1 && exe::comp_util_is_league_division(comp_id) != 0 {
        write_i16(counts(2), ftol_x87(get_club_league_stat(read_usize(G_PLAYER_STATS), j.player().addr(), 2) as f64) as i16);
        write_u16(counts(3), player_stats_get_career_goals(person.addr()));
    } else {
        write_i16(counts(2), -1000);
        write_i16(counts(3), -1000);
    }
    j.set_debut((exe::player_stats_get_apps_for_club(read_usize(G_PLAYER_STATS), person, club) == 0) as i8);
    write_u16(counts(0), player_stats_get_goals_for_club(person.addr(), club.addr()));
    write_i16(counts(4), -1000);
    let job = read_usize(person.addr() + staff::CLUB_JOB);
    if job != 0 && exe::club_is_in_season(job, G_CURRENT_DATE) != 0
        && exe::playerstats_get_stat(read_usize(G_PLAYER_STATS), j.player(), 6, 1, 6.5) >= read_f64(rdata::F64_3)
    {
        write_i16(counts(4), ftol_x87(exe::playerstats_get_stat(read_usize(G_PLAYER_STATS), j.player(), 6, 2, 6.5)) as i16);
    }
}

/// The importance factor (x87 doubles) of the match for the frustration: a national side 5.0; with a contract,
/// (morale × 0.005 + 1.0) × the squad status factor (status 1: 0.3, 2: 0.5, 3: 0.75, otherwise 3.0); otherwise 1.0.
unsafe fn match_importance(own: Club, contract: usize) -> f64 {
    if exe::database_is_national_team(own) != 0 {
        return 5.0;
    }
    if contract == 0 {
        return read_f64(rdata::F64_1);
    }
    let contract = Contract(contract);
    let status = match contract.squad_status() >> 4 {
        1 => read_f64(rdata::F64_0_3),
        2 => read_f64(rdata::F64_0_5),
        3 => read_f64(rdata::F64_0_75),
        _ => read_f64(rdata::F64_3),
    };
    (contract.morale() as f64 * read_f64(rdata::F64_0_005) + read_f64(rdata::F64_1)) * status
}

/// The stake of the match (`bl` of the original): with the home mark on the fixture, (the competition's reputation +
/// the opponent's reputation / 500) / 2, otherwise the competition's reputation byte; +10 when his club has the
/// opponent as a rival. A fixture whose competition id has no object becomes a friendly (the fixture's competition id
/// and object are rewritten to `KEY_CLUB_COMP_FRIENDLY`); −10 when his club is no longer in contention
/// (`exe::comp_is_club_still_in_contention`).
unsafe fn match_stake(j: MatchPlayer, engine: Engine) -> i8 {
    let fixture = Fixture(engine.r#match());
    let side = j.side();
    let own = fixture_club_of_side(fixture, side);
    let opponent = fixture_club_of_side(fixture, (side == 0) as i8);
    let competition = read_usize(fixture.addr() + fixture::COMPETITION);
    let mut stake = if fixture.home_side_mark() != 0 {
        ((read_i16(competition + competition::REPUTATION) as i32 + opponent.reputation() as i32 / 500) / 2) as i8
    } else {
        read_i8(competition + competition::REPUTATION)
    };
    if has_rival(own, opponent.addr()) {
        stake = stake.wrapping_add(10);
    }
    let comp_id = fixture.comp_id();
    let objects = read_usize(G_COMP_OBJECT_TABLE);
    if comp_id > -1 && read_usize(objects + 4 * comp_id as usize) == 0 {
        covered!("sx15_1 fixture becomes a friendly");
        let friendly = read_u32(KEY_CLUB_COMP_FRIENDLY);
        write_u32(fixture.addr(), friendly);
        write_usize(fixture.addr() + fixture::COMPETITION, read_usize(G_CLUB_COMP_TABLE).wrapping_add((friendly as usize).wrapping_mul(0x6b)));
    }
    let comp_id = Fixture(engine.r#match()).comp_id();
    if comp_id > -1 && exe::comp_is_club_still_in_contention(read_usize(objects + 4 * comp_id as usize), own) == 0 {
        stake = stake.wrapping_sub(10);
    }
    stake
}

/// The frustration roll's base (`match_player::FRUSTRATION`): the pressure value (pitch condition × 2000, or × (big
/// matches + 10) × 100 away or with a pitch handicap, × professionalism², × the importance, × the stake factor, ÷ the
/// stadium capacity or 1500) against random(2,000,000): above it random(10); else above random(1,000,000) random(5);
/// else 0 for a goalkeeper; else below random(100,000) random(−5); else above random(10,000) random(−10); else the
/// value left from the previous match (a defect). Returns the stake factor ((stake − 10) × 0.05 + 1.0)^6
/// (`game_pow`), to be bent towards 1 when above it.
unsafe fn frustration_base(j: MatchPlayer, engine: Engine, person: Staff, contract: usize) -> f64 {
    let fixture = Fixture(engine.r#match());
    let importance = match_importance(fixture_club_of_side(fixture, j.side()), contract);
    let stake = match_stake(j, engine);
    let factor = game_pow((stake as i32 - 10) as f64 * read_f64(rdata::F64_0_05) + read_f64(rdata::F64_1), read_f64(rdata::F64_6));
    let stadium = Fixture(engine.r#match()).stadium();
    let capacity = if stadium != 0 && read_i32(stadium + stadium::CAPACITY) > 0 { read_i32(stadium + stadium::CAPACITY) } else { 1500 };
    let pitch = read_i32(engine.addr() + engine::PITCH_CONDITION);
    let professionalism = person.professionalism() as f64;
    let base = if j.side() == 0 && engine.side_pitch_handicap() == 0 {
        pitch.wrapping_mul(2000)
    } else {
        (read_i8(j.player().addr() + player_sheet::IMPORTANT_MATCHES) as i32 + 10).wrapping_mul(pitch).wrapping_mul(100)
    };
    let pressure = random(ftol_x87(base as f64 * professionalism * professionalism * importance * factor / capacity as f64));
    if pressure > random(2_000_000) {
        j.set_frustration(random(10) as i8);
    } else if pressure > random(1_000_000) {
        j.set_frustration(random(5) as i8);
    } else if !j.engine().is_null() && exe::cm3_match_engine_get_player_position(j.engine(), j) == 1 {
        j.set_frustration(0);
    } else if pressure < random(100_000) {
        j.set_frustration(random(-5) as i8);
    } else if pressure > random(10_000) {
        j.set_frustration(random(-10) as i8); // sx15_1: the calm draw
    }
    factor
}

/// The frustration (`match_player::FRUSTRATION`) and, from it, the anger (`match_player::ANGER`): the base
/// (`frustration_base`; 0 and a factor of 1.0 for a match without fixture or a person outside the staff table), plus
/// random((big matches − 10) × the factor, bent to factor × 1/7 + 0.858 above 1), scaled by (1 − ability × 0.0025);
/// +10 when the opponent's manager dislikes him (home mark only); + random(his side's manager's motivating rating − 10)
/// when the club is not human and the side has a manager with staff, else + random(6); the league pressure (only when
/// the fixture is his division's, with happiness above 25: −1 by the winning run (> 5) or unbeaten run (> 9) against
/// professionalism < run + 10, or by the run (> 3) or unbeaten run (> 7) against professionalism < run + 6; −1 more
/// when random(professionalism) or random(pressure) is 0 with a run above 3 or unbeaten above 5; −1 when the club's
/// manager is retiring and random(professionalism) or random(loyalty) is 0); +1 for the goalkeeper of a human club;
/// +1 for a happy settled player (contract without grievances, morale above 25, not listed, club state byte 5 clear,
/// professionalism above 14 and random(loyalty) ≠ 0; for a computer club only below 5). Anger = (dirtiness +
/// frustration / 2 + 10) / 4.
unsafe fn setup_frustration(j: MatchPlayer, engine: Engine, person: Staff, contract: usize) {
    let factor = if no_match(engine.addr()) != 0 || (!person.is_null() && read_i32(person.addr()) >= read_i32(G_STAFF_COUNT)) {
        j.set_frustration(0);
        read_f64(rdata::F64_1)
    } else {
        let factor = frustration_base(j, engine, person, contract);
        if factor > read_f64(rdata::F64_1) { factor * read_f64(0x96d658) + read_f64(0x96d650) } else { factor } // f64 = 1/7, 0.858
    };
    let sheet = j.player();
    let big = read_i8(sheet.addr() + player_sheet::IMPORTANT_MATCHES) as i32 - 10;
    j.set_frustration(j.frustration().wrapping_add(random(ftol_x87(big as f64 * factor)) as i8));
    j.set_frustration(ftol_x87((read_f64(rdata::F64_1) - sheet.current_ability() as f64 * read_f64(rdata::F64_0_0025)) * j.frustration() as f64) as i8);
    let fixture = Fixture(j.engine().r#match());
    let side = j.side();
    if fixture.home_side_mark() != 0 {
        let opponent = fixture_club_of_side(fixture, (side == 0) as i8);
        if exe::staff_is_disliked_by(opponent.manager(), j.staff()) != 0 {
            j.set_frustration(j.frustration().wrapping_add(10));
        }
    }
    let own = fixture_club_of_side(Fixture(j.engine().r#match()), side);
    let manager = Engine((j.engine().addr() as isize + side as isize * 0xd0) as usize).side0_manager();
    let motivation = if exe::human_manager_is_human_club(own, 0, 1, 0, 0) == 0 && !manager.is_null() && !manager.non_player().is_null() {
        let np = manager.non_player();
        random(exe::non_player_attribute_rating(np, read_i8(np.addr() + non_player::MOTIVATING)) as i32 - 10)
    } else {
        random(6)
    };
    j.set_frustration(j.frustration().wrapping_add(motivation as i8));
    league_pressure(j, person, contract);
    let club = Club(j.club());
    if exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0 && !j.engine().is_null() && exe::cm3_match_engine_get_player_position(j.engine(), j) == 1 {
        j.set_frustration(j.frustration().wrapping_add(1));
    }
    settled_player_bonus(j, person, contract);
    let anger = (sheet.dirtiness() as i32 + j.frustration() as i32 / 2 + 10) / 4;
    j.set_anger(anger as i8);
}

/// The league pressure part of `setup_frustration`.
unsafe fn league_pressure(j: MatchPlayer, person: Staff, contract: usize) {
    let club = Club(j.club());
    if club.is_null() {
        return;
    }
    let division = read_usize(club.addr() + club::DIVISION);
    if division == 0 || contract == 0 {
        return;
    }
    if read_usize(j.engine().r#match() + fixture::COMPETITION) != division || Contract(contract).happiness() <= 25 {
        return;
    }
    let object = read_usize(read_usize(G_COMP_OBJECT_TABLE) + 4 * read_u32(division) as usize);
    if object == 0 || read_usize(object + 8) == 0 {
        return;
    }
    let frustrate = || j.set_frustration(j.frustration().wrapping_sub(1));
    let professionalism = person.professionalism();
    let entry = exe::comp_stats_find_club_entry(read_usize(object + 8), club);
    if entry != 0 {
        let run = read_i32(entry + comp_stats_club::WINNING_RUN);
        let unbeaten = read_i32(entry + comp_stats_club::UNBEATEN_RUN);
        let first = (run > 5 || unbeaten > 9) && professionalism < (run as i8).wrapping_add(10);
        if first || ((run > 3 || unbeaten > 7) && professionalism < (run as i8).wrapping_add(6)) {
            frustrate();
        }
        if (run > 3 || unbeaten > 5) && (random(professionalism as i32) == 0 || random(person.pressure() as i32) == 0) {
            frustrate();
        }
    }
    let manager = club.manager();
    if !manager.is_null() && exe::player_regen_get_retirement_status(read_usize(G_PLAYER_REGEN), manager) != 0
        && (random(professionalism as i32) == 0 || random(person.loyalty() as i32) == 0)
    {
        covered!("sx15_1 manager retiring");
        frustrate();
    }
}

/// The settled-player bonus of `setup_frustration`.
unsafe fn settled_player_bonus(j: MatchPlayer, person: Staff, contract: usize) {
    let club = Club(j.club());
    let objects = read_usize(G_COMP_OBJECT_TABLE);
    if club.is_null() || contract == 0 || objects == 0 {
        return;
    }
    let division = read_usize(club.addr() + club::DIVISION);
    if division == 0 || read_usize(objects + 4 * read_u32(division) as usize) == 0 {
        return;
    }
    if exe::human_manager_is_human_club(club, 0, 1, 0, 0) == 0 && j.frustration() >= 5 {
        return;
    }
    let contract = Contract(contract);
    if contract.grievances() != 0 || contract.morale() <= 25 || exe::staff_is_transfer_listed_or_requested(person, 0usize) != 0 {
        return;
    }
    let state = read_usize(G_CLUB_STATE_TABLE).wrapping_add((read_i32(club.addr()) as usize).wrapping_mul(6));
    if read_u8(state + 5) != 0 || person.professionalism() <= 14 {
        return;
    }
    if random(person.loyalty() as i32) != 0 {
        j.set_frustration(j.frustration().wrapping_add(1));
    }
}

/// Fills match player `j` from staff member `person` for one match on `engine`: identity and contract
/// (`setup_identity`), condition and feelings (`setup_condition_and_feelings`), the clears of the injury, substitution,
/// shot quality and byte +0x2f, the commentary record (`setup_record_for_commentary`), the strong foot
/// (`match_player::PREFERRED_FOOT`, 0 left when the left is better, and 20 minus the other foot), the name ids (+0x5d,
/// +0x61, +0x65: the +0x33 of the first, second and common name records, −1 none), the side, the consistency rolls
/// (random(20); random(35 − consistency) for a goalkeeper of 20, else random(2 × (30 − consistency))), the frustration
/// and anger (`setup_frustration`), the kit colour (+0x1ba: the bottom colour of the side's kit,
/// `exe::database_get_club_kit_colours`), the facing (180 for side 1, else 0), not busy, cell (0, 0) and action 0x65.
///
/// Defects of the original: the contract looked up at the start uses `match_player::SIDE` before it is written (the
/// side of the previous use of the record, the home club on a fresh one); the frustration keeps the previous match's
/// value when no roll fires; the 5750-reputation test of an earlier block (a flag that would give random(8) instead of
/// random(6) to a club without a manager) tests the address of the side's match manager object, never null, so it
/// never fires and is not ported.
/// Original: `MATCH_PLAYER::setup` (0x6eaea0, match_pl.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn match_player_setup(j: usize, engine: usize, person: usize, side: u32, index: u32, slot: u32) {
    let (j, engine, person) = (MatchPlayer(j), Engine(engine), Staff(person));
    let contract = setup_identity(j, engine, person, side, index, slot as u8);
    setup_condition_and_feelings(j, engine, person, side, slot as u8);
    j.set_injury_pending(0);
    j.set_substituted_off(0);
    j.set_shot_quality(0);
    write_u8(j.addr() + 0x2f, 0); // byte between COMMENTARY_ID and DEBUT
    setup_record_for_commentary(j, engine, person, side as u8);
    let sheet = j.player();
    let (left, right) = (read_i8(sheet.addr() + player_sheet::LEFT_FOOT), read_i8(sheet.addr() + player_sheet::RIGHT_FOOT));
    if left > right {
        j.set_preferred_foot(0);
        j.set_weak_foot_deficit(20 - right);
    } else {
        j.set_preferred_foot(1);
        j.set_weak_foot_deficit(20 - left);
    }
    let name_id = |name: usize| if name != 0 { read_u32(name + names::ID) } else { u32::MAX };
    write_u32(j.addr() + 0x5d, name_id(person.first_name())); // name ids of the commentary
    write_u32(j.addr() + 0x61, name_id(read_usize(person.addr() + staff::SECOND_NAME)));
    write_u32(j.addr() + 0x65, name_id(read_usize(person.addr() + staff::COMMON_NAME)));
    j.set_side(side as i8);
    j.set_consistency_roll(random(20) as i8);
    let consistency = sheet.consistency() as i32;
    j.set_inconsistency_swing(random(if sheet.goalkeeper() == 20 { 35 - consistency } else { (30 - consistency) * 2 }) as i8);
    setup_frustration(j, engine, person, contract);
    let kit = read_u8(j.engine().addr() + engine::SIDE_KIT + (j.side() != 0) as usize);
    let (mut front, mut bottom) = (slot, index);
    exe::database_get_club_kit_colours(j.club(), kit as u32, &mut front, &mut bottom);
    write_u16(j.addr() + 0x1ba, bottom as u16); // the side's kit colour
    j.set_facing(if side as u8 != 0 { 0xb4 } else { 0 });
    j.set_busy(0);
    j.set_cell_x(0);
    j.set_cell_y(0);
    j.set_action(0x65);
}

/// Comparer for `match_player_setup`: inside a tape (the team being placed) the call is one unit, compared on its own
/// inside; elsewhere side by side over the match player, the fixture head (the friendly rewrite), the global message
/// buffer, the random state and what the weekly training touches (its own comparer's regions). A player the training
/// comparer leaves to the original (outside the staff table, no player record, no schedule, position training items)
/// is only run by the original: the training then changes more than those regions.
pub(crate) unsafe extern "thiscall" fn compare_match_player_setup(j: usize, engine: usize, person: usize, side: u32, index: u32, slot: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32)>(IDX_MATCH_PLAYER_SETUP);
    let compare = || {
        let p = Staff(person);
        let training = read_usize(G_TRAINING_MANAGER);
        let mut q = 0usize;
        if p.is_null() || read_i32(p.addr()) >= read_i32(G_STAFF_COUNT) || p.player().is_null()
            || exe::training_manager_get_player_schedule(training, p) == 0
            || exe::training_manager_find_position_training(training, p, &mut q) > 0
        {
            return o(j, engine, person, side, index, slot);
        }
        let mut reg = crate::rng::STATE_REGIONS.to_vec();
        reg.extend([(j, MATCH_PLAYER_SIZE), (Engine(engine).r#match(), 0x18), (0xde1f64, 0x100), (G_CONTRACT_LOOKUP_RESULT, 4),
            (training_entry_of_staff(training, p), 6), (p.player().addr() + player_sheet::GOALKEEPER, 0x33)]);
        let injury = read_usize(G_INJURY);
        if injury != 0 {
            reg.push((read_usize(injury).wrapping_add((read_i32(p.addr()) as isize * 31 + (physical_record::PRONENESS as isize)) as usize), 1));
        }
        let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(j, engine, person, side, index, slot),
            || crate::rng::without_counting(|| match_player_setup(j, engine, person, side, index, slot)));
        SCOREBOARD.record(igual, || format!("match_player_setup(person {:#x}, side {}, slot {}) {:?}", person, side as u8, slot as i8, first_differences(&mo, &mr)));
    };
    if is_recording() {
        call_recording(FN_MATCH_PLAYER_SETUP, j, [engine as u32, person as u32, side, index, slot], || {
            compare();
            0
        });
        return;
    }
    compare();
}

// ---------- batch 347: the fights and the dissent of a match player (match_pl.cpp) ----------

/// Commentary with the player as subject: `code` with his commentary id and side (the original pushes -1, -1, 0 for
/// the last three arguments).
unsafe fn narrate_match_player(call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, j: MatchPlayer, code: u32) {
    let m = j.engine();
    call(FN_MATCH_EVENTS_ADD_EVENT, m.commentator(), [code, m.addr() as u32, j.commentary_id() as u32, j.side() as u8 as u32, u32::MAX]);
}

/// Whether the club of the side opposite to `j` (side 0: the away club, otherwise the home club) has a nation whose
/// first language is English: the referee's "off, off, off" line is only said for those.
unsafe fn opposing_club_speaks_english(j: MatchPlayer) -> bool {
    let fixture = Fixture(j.engine().r#match());
    let club = if j.side() == 0 { fixture.away_club() } else { fixture.home_club() };
    let nation = club.nation();
    !nation.is_null() && nation.first_language() == language::ENGLISH
}

/// The card shown to `j` after a dive or a reaction: yellow, second yellow, red or a ticking-off (cards 2, 3, 4 and
/// 9; anything else is not narrated). `factor` (the reaction only) scales the anger of his team-mates first
/// (`exe::cm3_match_engine_scale_side_anger_from_player`). random(8) ≠ 0 and `may_call_over` give, before the card,
/// "the referee calls him over", preceded by "off, off, off" (random(3) ≠ 0 and an English-speaking opposing club).
/// A second yellow or a red sends him off (`player_left_pitch_with`).
unsafe fn announce_card(j: MatchPlayer, card: u8, may_call_over: bool, react: bool, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let (code, factor) = match card {
        2 => (event::YELLOW_CARD, 0.85f32),
        3 => (event::SECOND_YELLOW_CARD, 0.75),
        4 => (event::SENT_OFF, 0.5),
        9 => (event::TICKING_OFF, 0.95),
        _ => return,
    };
    let m = j.engine();
    if react {
        exe::cm3_match_engine_scale_side_anger_from_player(m, j, factor);
    }
    if random(8) != 0 && may_call_over {
        if random(3) != 0 && opposing_club_speaks_english(j) {
            call(FN_MATCH_EVENTS_ADD_EVENT, m.commentator(), [event::SILENT_OFF_OFF_OFF, m.addr() as u32, 0, 0, u32::MAX]);
        }
        narrate_match_player(call, j, event::REFEREE_CALLS_PLAYER_OVER);
    }
    narrate_match_player(call, j, code);
    if card == 3 || card == 4 {
        covered!("sx15_2 sent off after the card");
        player_left_pitch_with(m, j, 0, call);
    }
}

/// A suspected dive by `j`: narrates "looked like a dive", asks the referee for the card of severity 6
/// (`engine_card_by_severity`, flag 0) and announces it (`announce_card`: the call-over may come; a second yellow or
/// a red sends him off).
/// Original: `MATCH_PLAYER::handle_suspected_dive` (0x716c90, match_pl.cpp, thiscall).
pub(crate) unsafe fn handle_suspected_dive_with(j: MatchPlayer, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    covered!("sx15_2 suspected dive");
    narrate_match_player(call, j, event::LOOKED_LIKE_A_DIVE);
    let mut card = 0u8;
    engine_card_by_severity(j.engine().addr(), j.addr(), &mut card, 6, 0); // sx15_2: the dive's severity
    announce_card(j, card, true, false, call);
}

/// Hook entry for `handle_suspected_dive`.
pub(crate) unsafe extern "thiscall" fn handle_suspected_dive(j: usize) {
    handle_suspected_dive_with(MatchPlayer(j), &mut |rotina, this, a| dispatch_fight_calls(rotina, this, a))
}

/// Whether `j` loses his head: random(20 − aggression) = 0 with aggression at least 5 (x87: the f32 20.0 minus the f32
/// aggression, truncated).
unsafe fn loses_head(j: MatchPlayer) -> bool {
    random(ftol_x87(read_f32(rdata::F32_20) as f64 - j.aggression() as f64)) == 0 && j.aggression() as f64 >= read_f64(rdata::F64_5)
}

/// The reaction of `j` to a referee's decision against him, by his anger checks (`exe::match_player_passes_anger_check`
/// with 5, 4, 2, 2 and 1, target 0): level 5 strikes the referee (severity 5) and level 4 pushes him (severity 4) when
/// he loses his head (`loses_head`), otherwise he kicks the ball away (severity 3, and the referee may call him over);
/// the first level 2 also kicks the ball away; the second level 2 (severity 2) and level 1 (severity 1) argue. The card
/// comes from the referee (`engine_card_by_severity` with `flag`) and is announced (`announce_card`, scaling his
/// team-mates' anger by the card). Then, in an English competition (the competition's nation is
/// `KEY_NATION_ENGLAND`), with the ball with the other side, a card other than 0 and 9, the ball on rows 1..10 and a
/// free kick pending (`restart::FREE_KICK` or `restart::INDIRECT_FREE_KICK`): the free kick moves one row towards his
/// goal (clamped 0..11, "moved back 10 yards") and, inside his own penalty area (`exe::is_in_own_penalty_area`),
/// becomes indirect (narrated only when it was direct).
/// Original: `MATCH_PLAYER::react_to_foul_decision` (0x716e10, match_pl.cpp, thiscall, ret 4).
pub(crate) unsafe fn react_to_foul_decision_with(j: MatchPlayer, flag: u32, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let passes = |level: i8| exe::match_player_passes_anger_check(j, level, 0usize) != 0;
    let mut argued = false;
    let mut kick_ball_away = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32| {
        narrate_match_player(call, j, event::PLAYER_KICKS_BALL_AWAY);
        argued = true;
        3u8
    };
    let severity = if passes(5) {
        if loses_head(j) {
            narrate_match_player(call, j, event::PLAYER_STRIKES_REFEREE);
            Some(5u8)
        } else {
            Some(kick_ball_away(call))
        }
    } else if passes(4) {
        if loses_head(j) {
            narrate_match_player(call, j, event::PLAYER_PUSHES_REFEREE);
            Some(4)
        } else {
            Some(kick_ball_away(call))
        }
    } else if passes(2) {
        Some(kick_ball_away(call))
    } else if passes(2) {
        narrate_match_player(call, j, event::PLAYER_ARGUES_WITH_REFEREE);
        Some(2)
    } else if passes(1) {
        narrate_match_player(call, j, event::PLAYER_ARGUES_WITH_REFEREE);
        Some(1) // sx15_2: arguing, severity 1
    } else {
        None
    };
    let mut card = 0u8;
    if let Some(s) = severity {
        engine_card_by_severity(j.engine().addr(), j.addr(), &mut card, s, flag as u8);
    }
    announce_card(j, card, argued, true, call);
    move_free_kick_back(j, card, call);
}

/// The English ten-yard rule after dissent (see `react_to_foul_decision_with`).
unsafe fn move_free_kick_back(j: MatchPlayer, card: u8, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let m = j.engine();
    let other = (j.side() == 0) as u32;
    if m.possession_side() as i32 != other as i32 || card == 0 || card == 9 {
        return;
    }
    if m.ball_y() <= 0 || m.ball_y() >= 11 {
        return;
    }
    if m.restart_type() != restart::FREE_KICK && m.restart_type() != restart::INDIRECT_FREE_KICK {
        return;
    }
    let competition = Fixture(m.r#match()).competition();
    if competition.is_null() || competition.nation().is_null() || read_u32(competition.nation().addr()) != read_u32(KEY_NATION_ENGLAND) {
        return;
    }
    covered!("sx15_2 free kick moved back");
    let y = if j.side() != 0 { m.ball_y() - 1 } else { m.ball_y() + 1 };
    m.set_ball_y(y.clamp(0, 11)); // sx15_2: one row towards his goal
    call(FN_MATCH_EVENTS_ADD_EVENT, m.commentator(), [event::FREEKICK_MOVED_BACK_10_YARDS, m.addr() as u32, u32::MAX, other, u32::MAX]);
    if exe::is_in_own_penalty_area(j.side() as u8, m.ball_x(), m.ball_y()) != 0 {
        if m.restart_type() == restart::FREE_KICK {
            call(FN_MATCH_EVENTS_ADD_EVENT, m.commentator(), [event::FREEKICK_BECOMES_INDIRECT, m.addr() as u32, u32::MAX, other, u32::MAX]);
        }
        m.set_restart_type(restart::INDIRECT_FREE_KICK);
    }
}

/// Hook entry for `react_to_foul_decision`.
pub(crate) unsafe extern "thiscall" fn react_to_foul_decision(j: usize, flag: u32) {
    react_to_foul_decision_with(MatchPlayer(j), flag, &mut |rotina, this, a| dispatch_fight_calls(rotina, this, a))
}

/// The provoked opponent's confidence (`match_player::CONFIDENCE`, f32): (`base` − temperament × `step`) × confidence,
/// floored at 2 × (25 − temperament) and capped at 100 (x87: the product kept unrounded for the comparisons).
unsafe fn provoke_confidence(e: MatchPlayer, base: f64, step: f64) {
    let temperament = e.staff().temperament() as i32;
    let mut v = (base - temperament as f64 * step) * e.confidence() as f64;
    let floor = ((25 - temperament) * 2) as f32;
    if v <= floor as f64 {
        v = floor as f64;
    }
    e.set_confidence(v as f32);
    if v > read_f64(rdata::F64_100) {
        e.set_confidence(100.0);
    }
}

/// The opponent `j` picks a fight with, in his own grid cell, and the act: the first occupant of the other side (with
/// staff and player records) for whom an anger check passes (`exe::match_player_passes_anger_check` with the occupant
/// as target). Level 4: headbutt (his side's anger × 2.25, `provoke_confidence` 4.0/0.16). Level 3: spit when
/// random(1000, or 300 when `j`'s nation does not speak English) < 10 (× 2.5), otherwise by random(10): 0..3 kick, 4..8
/// punch, 9 elbow (× 2.0), all with 4.0/0.16. Level 2: shove (random(2) ≠ 0 or already squaring up) or square up
/// (× 1.15, 3.1/0.1). Returns (commentary, move, opponent), the opponent null when none.
unsafe fn pick_fight_opponent(j: MatchPlayer) -> (u32, u8, MatchPlayer) {
    let m = j.engine();
    let cell = m.cell(j.cell_x(), j.cell_y()).addr();
    let other = (j.side() == 0) as usize;
    let count = || read_i8(cell + grid_cell::OCCUPANT_COUNT + other);
    let (mut code, mut mv, mut target) = (u32::MAX, 0u8, MatchPlayer(0));
    let mut k = 0usize;
    while code == u32::MAX && (k as i32) < count() as i32 {
        let e = MatchPlayer(read_usize(cell + grid_cell::OCCUPANTS + (other * 11 + k) * 4));
        k += 1;
        if e.is_null() || e.staff().is_null() || e.player().is_null() {
            continue;
        }
        let provoke = |code_mv: (u32, usize), factor: f32, base: f64, step: f64| {
            exe::cm3_match_engine_scale_side_anger(m, e.side(), factor);
            provoke_confidence(e, base, step);
            (code_mv.0, code_mv.1 as u8, e)
        };
        if exe::match_player_passes_anger_check(j, 4, e) != 0 {
            (code, mv, target) = provoke((event::PLAYER_HEADBUTTS_OPPONENT, moves::HEADBUTT), 2.25, 4.0, read_f64(0x96d330)); // f64 = 0.16
        } else if exe::match_player_passes_anger_check(j, 3, e) != 0 {
            let english = j.staff().nation().first_language() == language::ENGLISH;
            if random(if english { 1000 } else { 300 }) < 10 {
                (code, mv, target) = provoke((event::PLAYER_SPITS_AT_OPPONENT, moves::SPIT), 2.5, 4.0, read_f64(0x96d330));
            } else {
                let act = match random(10) {
                    0..=3 => (event::PLAYER_KICKS_OPPONENT, moves::KICK_OPPONENT),
                    4..=8 => (event::PLAYER_PUNCHES_OPPONENT, moves::PUNCH),
                    _ => (event::PLAYER_ELBOWS_OPPONENT, moves::ELBOW),
                };
                (code, mv, target) = provoke(act, 2.0, read_f64(rdata::F64_4), read_f64(0x96d330)); // sx15_2: the blows
            }
        } else if exe::match_player_passes_anger_check(j, 2, e) != 0 {
            let act = if random(2) != 0 || j.action() as usize == moves::SQUARE_UP {
                (event::PLAYER_PUSHES_OPPONENT, moves::SHOVE)
            } else {
                (event::PLAYER_SQUARES_UP_TO_OPPONENT, moves::SQUARE_UP)
            };
            (code, mv, target) = provoke(act, 1.15, read_f64(rdata::F64_3_1), read_f64(rdata::F64_0_1));
        }
    }
    (code, mv, target)
}

/// An angry `j` picks a fight in his grid cell (`pick_fight_opponent`); none: returns 0. Otherwise his action becomes
/// the move. At `depth` > 1, random(5) ≠ 0 and at least random(3) + 2 pending incidents: "a big punch-up" and the
/// depth becomes −50 (the chain goes on to −40); otherwise, at `depth` ≥ 0, the act is narrated with both players. The
/// players on the pitch who are not busy (`match_player::BUSY` < 2) pass the level-1 anger check and run to his cell
/// (`exe::match_player_move_to_grid_cell`, a tape unit). The referee judges the act (`exe::cm3_match_engine_referee_judge_tackle`,
/// the move and −1): with a verdict, the severity is added to his pending incidents, or a new incident (player,
/// opponent, severity, move, card) is queued. The opponent reacts in turn (this routine with `depth` + 1) while `depth`
/// is 0..9 or below −40 and fewer than 5 incidents are pending. Returns 1.
/// Original: `MATCH_PLAYER::start_fight` (0x716630, match_pl.cpp, thiscall, ret 4).
pub(crate) unsafe fn start_fight_with(j: MatchPlayer, depth: i8, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) -> u8 {
    let m = j.engine();
    let (code, mv, target) = pick_fight_opponent(j);
    if target.is_null() {
        return 0;
    }
    covered!("sx15_2 fight");
    j.set_action(mv as i8 as i16 as u16);
    let mut next = depth;
    let mut brawl = false;
    if depth > 1 && random(5) != 0 && m.pending_incident_count() as i32 >= random(3) + 2 {
        call(FN_MATCH_EVENTS_ADD_EVENT, m.commentator(), [event::BIG_PUNCH_UP, m.addr() as u32, u32::MAX, j.side() as u8 as u32, u32::MAX]);
        next = -50;
        brawl = true;
    }
    if !brawl && depth >= 0 {
        let pair = target.commentary_id() as u32 | (target.side() as u8 as u32) << 8;
        call(FN_MATCH_EVENTS_ADD_EVENT, m.commentator(), [code, m.addr() as u32, j.commentary_id() as u32, j.side() as u8 as u32, pair]);
    }
    for side in 0..2i8 {
        for k in 0..20 {
            let p = m.player(side, k);
            if (0..=10).contains(&p.position()) && p.busy() < 2 && p.addr() != j.addr() && exe::match_player_passes_anger_check(p, 1, 0usize) != 0 {
                call(FN_MATCH_PLAYER_MOVE_TO_GRID_CELL, p.addr(), [j.cell_x() as u8 as u32, j.cell_y() as u8 as u32, 0, 0, 0]);
            }
        }
    }
    let (mut card, mut severity, mut advantage) = (0u8, 0i8, 0u8);
    if exe::cm3_match_engine_referee_judge_tackle(m, j, target, mv as i8, -1, &mut card, &mut severity, &mut advantage) != 0 {
        queue_fight_incident(m, j, target, card, mv, severity);
    }
    if ((0..10).contains(&next) || next < -40) && m.pending_incident_count() < 5 {
        call(FN_MATCH_PLAYER_START_FIGHT, target.addr(), [next.wrapping_add(1) as u8 as u32, 0, 0, 0, 0]);
    }
    1
}

/// The referee's verdict on a fight: `severity` is added to every pending incident of `j`; when he has none, a new
/// incident (`j`, `target`, `severity`, the move, the card) is appended (11-byte entries from
/// `engine::PENDING_INCIDENTS`).
unsafe fn queue_fight_incident(m: Engine, j: MatchPlayer, target: MatchPlayer, card: u8, mv: u8, severity: i8) {
    let entry = |k: i8| Engine((m.addr() as isize + 11 * k as isize) as usize);
    let mut found = false;
    for k in 0..m.pending_incident_count() {
        if entry(k).pending_incidents() == j.addr() {
            entry(k).set_pending_incident_severity(entry(k).pending_incident_severity().wrapping_add(severity));
            found = true;
        }
    }
    if found {
        return;
    }
    let e = entry(m.pending_incident_count());
    e.set_pending_incidents(j.addr());
    e.set_pending_incident_victim(target.addr());
    e.set_pending_incident_reason(card as i8);
    e.set_pending_incident_play(mv as i8);
    e.set_pending_incident_severity(severity); // sx15_2: the new incident's severity
    m.set_pending_incident_count(m.pending_incident_count() + 1);
}

/// Hook entry for `start_fight`.
pub(crate) unsafe extern "thiscall" fn start_fight(j: usize, depth: u32) -> u8 {
    start_fight_with(MatchPlayer(j), depth as i8, &mut |rotina, this, a| dispatch_fight_calls(rotina, this, a))
}

/// Rust-mode dispatcher of the fights' injected calls: the moves to a grid cell and the next fight go to the game
/// routines; the rest (commentary, sending-off) as in the tackle (`dispatch_tackle_calls`).
pub(crate) unsafe fn dispatch_fight_calls(routine: usize, this: usize, a: [u32; 5]) -> u32 {
    match routine {
        FN_MATCH_PLAYER_MOVE_TO_GRID_CELL => exe::match_player_move_to_grid_cell(this, a[0] as i8, a[1] as i8),
        FN_MATCH_PLAYER_START_FIGHT => {
            let f: unsafe extern "thiscall" fn(usize, u32) -> u8 = core::mem::transmute(FN_MATCH_PLAYER_START_FIGHT);
            f(this, a[0]) as u32
        }
        _ => dispatch_tackle_calls(routine, this, a),
    }
}

/// Mask of the arguments the tape compares for each injected call of the fights.
fn fight_call_mask(routine: usize) -> [u32; 5] {
    match routine {
        0x6d6f70 => [0xffff, u32::MAX, 0xff, 0xff, 0xff],
        0x897760 => [u32::MAX, 0xff, u32::MAX, 0, 0],
        0x6f46b0 => [0xff, 0xff, 0, 0, 0],
        0x6e1aa0 => [0; 5],
        _ => [0xff, 0, 0, 0, 0],
    }
}

/// Regions compared for the fights: the random state, the whole engine, the two globals the sending-off touches and
/// the tactics rows (the captain) of both clubs.
unsafe fn fight_regions(m: Engine) -> Vec<(usize, usize)> {
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.extend([(m.addr(), ENGINE_SIZE), (G_MOUSE_LEFT_BUTTON_DOWN, 4), (G_CONTRACT_LOOKUP_RESULT, 4)]);
    let t = Fixture(m.r#match());
    for club in [t.home_club(), t.away_club()] {
        reg.extend(club_rows(club.addr()));
    }
    reg
}

/// The comparison gate of the three: inside another routine's tape that records them (`RECORD_716`) the call is one
/// unit, compared on its own inside; inside a tape that does not, the original runs (as the old gate did); outside any
/// tape, `compare` runs.
unsafe fn fight_gate(routine: usize, j: usize, a1: u32, original: impl FnOnce() -> u32, compare: impl FnOnce() -> u32) -> u32 {
    if !is_recording() {
        return compare();
    }
    if !RECORD_716.load(Relaxed) {
        return original();
    }
    call_recording(routine, j, [a1, 0, 0, 0, 0], compare)
}

/// Comparer for `start_fight`.
pub(crate) unsafe extern "thiscall" fn compare_start_fight(j: usize, depth: u32) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u8>(IDX_START_FIGHT);
    fight_gate(FN_MATCH_PLAYER_START_FIGHT, j, depth, || o(j, depth) as u32, || {
        let jp = MatchPlayer(j);
        let write = RECORD_716.swap(true, Relaxed);
        let (r, v, igual, sequencia, k) = compare_com_fita(fight_regions(jp.engine()), || {
            zero_stack_below();
            o(j, depth)
        }, |chamar| start_fight_with(jp, depth as i8, &mut |rotina, this, args| chamar(rotina, this, args, fight_call_mask(rotina))));
        RECORD_716.store(write, Relaxed);
        SCOREBOARD.record(igual && sequencia && r == v, || format!("start_fight(depth {}) original={} rust={} {} calls: {}", depth as i8, r, v, k, tape_detail()));
        r as u32
    }) as u8
}

/// Comparer for `handle_suspected_dive`.
pub(crate) unsafe extern "thiscall" fn compare_handle_suspected_dive(j: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_HANDLE_SUSPECTED_DIVE);
    fight_gate(FN_MATCH_PLAYER_HANDLE_SUSPECTED_DIVE, j, 0, || {
        o(j);
        0
    }, || {
        let jp = MatchPlayer(j);
        let write = RECORD_716.swap(true, Relaxed);
        let (_, _, igual, sequencia, k) = compare_com_fita(fight_regions(jp.engine()), || o(j), |chamar| {
            handle_suspected_dive_with(jp, &mut |rotina, this, args| chamar(rotina, this, args, fight_call_mask(rotina)))
        });
        RECORD_716.store(write, Relaxed);
        SCOREBOARD.record(igual && sequencia, || format!("handle_suspected_dive() {} calls: {}", k, tape_detail()));
        0
    });
}

/// Comparer for `react_to_foul_decision`.
pub(crate) unsafe extern "thiscall" fn compare_react_to_foul_decision(j: usize, flag: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_REACT_TO_FOUL_DECISION);
    fight_gate(FN_MATCH_PLAYER_REACT_TO_FOUL_DECISION, j, flag, || {
        o(j, flag);
        0
    }, || {
        let jp = MatchPlayer(j);
        let write = RECORD_716.swap(true, Relaxed);
        let (_, _, igual, sequencia, k) = compare_com_fita(fight_regions(jp.engine()), || {
            zero_stack_below();
            o(j, flag)
        }, |chamar| react_to_foul_decision_with(jp, flag, &mut |rotina, this, args| chamar(rotina, this, args, fight_call_mask(rotina))));
        RECORD_716.store(write, Relaxed);
        SCOREBOARD.record(igual && sequencia, || format!("react_to_foul_decision(flag {}) {} calls: {}", flag as u8, k, tape_detail()));
        0
    });
}
