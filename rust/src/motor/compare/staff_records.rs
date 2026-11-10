use super::*;

// The comparers of the routines in ../staff_records.rs.
// [pt] Os comparadores das rotinas de ../staff_records.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `add_league_champions_record`.
pub(crate) unsafe extern "thiscall" fn compare_add_league_champions_record(this: usize, club: usize, comp: usize) {
    let club = Club(club);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_ADD_LEAGUE_CHAMPIONS_RECORD);
    if club.is_null() || comp == 0 || read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(4 * read_usize(comp))) == 0 {
        return o(this, club.addr(), comp);
    }
    let t = Staff(club.manager().addr());
    let notes = if !t.is_null() && !t.non_player().is_null() { (t.non_player().addr() + 8, 6) } else { (club.addr() + club::REPUTATION, 2) };
    compare_champion_update("add_league_champions_record", this, club, notes, true, || o(this, club.addr(), comp),
        || add_league_champions_record(this, club.addr(), comp));
}

/// Comparer for `add_cup_winners_record`.
pub(crate) unsafe extern "thiscall" fn compare_add_cup_winners_record(this: usize, club: usize, game: usize, partida: usize) {
    let partida = MatchRecord(partida);
    let game = Fixture(game);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(IDX_ADD_CUP_WINNERS_RECORD);
    if club == 0 || game.is_null() || partida.is_null() {
        return o(this, club, game.addr(), partida.addr());
    }
    let notes = (partida.addr() + match_record::HOME_MANAGER_CURRENT_REPUTATION + 0x625 * (game.away_club() == Club(club)) as usize, 6);
    compare_champion_update("add_cup_winners_record", this, Club(club), notes, false, || o(this, club, game.addr(), partida.addr()),
        || add_cup_winners_record(this, club, game.addr(), partida.addr()));
}

/// Comparer for `clear_human_manager_record`.
///
/// [pt] Comparador: lado a lado pelo registro (os 0xd2 bytes que a rotina zera).
pub(crate) unsafe extern "thiscall" fn compare_clear_human_manager_record(this: usize, person: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_CLEAR_HUMAN_MANAGER_RECORD);
    let id = read_i32(person);
    let lim = read_i32(G_STAFF_COUNT);
    let r = if id >= lim.wrapping_sub(0x10) && id < lim {
        let slot = id.wrapping_sub(lim).wrapping_add(read_i32(G_NON_PLAYER_COUNT)).wrapping_add(0x10);
        read_usize(this + staff_records_fields::MANAGER_RECORDS).wrapping_add((slot as usize).wrapping_mul(0xd2))
    } else {
        person
    };
    let ((), (), igual, mo, mr) = crate::comparison::side_by_side(&[(r, 0xd2)], || o(this, person),
        || clear_human_manager_record(this, person));
    SCOREBOARD.record(igual, || format!("clear_human_manager_record(pessoa {:#x}) {:x?} x {:x?}", person, mo, mr));
}

/// Comparer for `record_match_result`.
///
/// [pt] Comparador lado a lado nos dois registros (0xd2 bytes), criados antes pela 0x86f5b0 como a original faria (a segunda
/// chamada acha o mesmo registro). Jogo não disputado: só a original (não cria nada).
pub(crate) unsafe extern "thiscall" fn compare_record_match_result(this: usize, p: usize, m: usize) {
    let p = Fixture(p);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_RECORD_MATCH_RESULT);
    if p.category() == 1 {
        return o(this, p.addr(), m);
    }
    let a = club_manager_record(this, Club(p.home_club().addr()));
    let v = club_manager_record(this, Club(p.away_club().addr()));
    let mut reg = Vec::new();
    for r in [a, v] {
        if r != 0 {
            reg.push((r, 0xd2));
        }
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, p.addr(), m), || record_match_result(this, p.addr(), m));
    SCOREBOARD.record(igual, || format!("record_match_result(jogo {:#x}) original={:x?} rust={:x?}", p.addr(), mo, mr));
}

/// Comparer for `add_league_finish_records`.
pub(crate) unsafe extern "thiscall" fn compare_add_league_finish_records(this: usize, league: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_ADD_LEAGUE_FINISH_RECORDS)(this, league)
}

/// Comparer for `add_supporters_player_of_year_record`.
///
/// [pt] Comparador em ensaio: a cópia sem o histórico e a original a partir do mesmo estado (sorteio, as pessoas favoritas e
/// rejeitadas do clube +0x9b e os clubes favoritos e rejeitados da pessoa, preferências +0x65: +4..+0x18). Pessoa nula:
/// só a original.
pub(crate) unsafe extern "thiscall" fn compare_add_supporters_player_of_year_record(this: usize, person: usize, club: usize, a: u32, b: u32) {
    let person = Staff(person);
    let club = Club(club);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(IDX_ADD_SUPPORTERS_PLAYER_OF_YEAR_RECORD);
    if person.is_null() || club.is_null() {
        return o(this, person.addr(), club.addr(), a, b);
    }
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.push((club.addr() + club::FAV_STAFF1, 0x18));
    let pref = person.staff_preferences();
    if pref != 0 {
        reg.push((pref + preferences::STAFF_FAVOURITE_CLUBS1, 0x18));
    }
    let before = crate::comparison::MemorySnapshot::read(&reg);
    SUPPORTERS_PLAYER_OF_YEAR_DRY_RUN = true;
    crate::rng::pause_trace(|| crate::rng::without_counting(|| add_supporters_player_of_year_record(this, person.addr(), club.addr(), a, b)));
    SUPPORTERS_PLAYER_OF_YEAR_DRY_RUN = false;
    let mr = crate::comparison::MemorySnapshot::read(&reg);
    before.write_back();
    o(this, person.addr(), club.addr(), a, b);
    let mo = crate::comparison::MemorySnapshot::read(&reg);
    SCOREBOARD.record(mo == mr, || format!("add_supporters_player_of_year_record(pessoa {:#x}, clube {:#x}) original={:x?} rust={:x?}", person.addr(), club.addr(), mo, mr));
}

/// Comparer for `add_cup_runners_up_record`.
pub(crate) unsafe extern "thiscall" fn compare_add_cup_runners_up_record(this: usize, club: usize, game: usize, partida: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(IDX_ADD_CUP_RUNNERS_UP_RECORD);
    compare_cup_placed("add_cup_runners_up_record", Club(club), Fixture(game), MatchRecord(partida), || o(this, club, game, partida),
        || add_cup_runners_up_record(this, club, game, partida))
}

/// Comparer for `add_cup_third_place_record`.
pub(crate) unsafe extern "thiscall" fn compare_add_cup_third_place_record(this: usize, club: usize, game: usize, partida: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(IDX_ADD_CUP_THIRD_PLACE_RECORD);
    compare_cup_placed("add_cup_third_place_record", Club(club), Fixture(game), MatchRecord(partida), || o(this, club, game, partida),
        || add_cup_third_place_record(this, club, game, partida))
}

/// Comparer for `record_manager_job_change`.
///
/// A comparer in rehearsal mode: the manager's record created beforehand (`get_manager_record`, as the original routine would do on
/// the two paths that need it), the copy without the histories and the original from the same state (the record, 0xd2
/// bytes, and the static date of `exe::date_days_since`).
///
/// Nulls: only the original.
///
/// [pt] Comparador em ensaio: o registro do técnico criado antes (0x86f5b0, como a original faria nos dois caminhos que o
/// pedem), a cópia sem os históricos e a original a partir do mesmo estado (o registro, 0xd2 bytes, e a data estática da
/// 0x54cdb0). Nulos: só a original.
pub(crate) unsafe extern "thiscall" fn compare_record_manager_job_change(this: usize, manager: usize, club: usize, contratado: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(IDX_RECORD_MANAGER_JOB_CHANGE);
    if club == 0 || manager == 0 {
        return o(this, manager, club, contratado);
    }
    let mut reg = vec![(G_DAYS_SINCE_SCRATCH_DATE, 8usize)];
    if is_national_team(club) == 0 || contratado as u8 == 1 {
        let r = exe::staff_records_get_manager_record(this, manager, 1);
        if r != 0 {
            reg.push((r, 0xd2));
        }
    }
    let before = crate::comparison::MemorySnapshot::read(&reg);
    CUP_PLACED_CLUB_DRY_RUN = true;
    record_manager_job_change(this, manager, club, contratado);
    CUP_PLACED_CLUB_DRY_RUN = false;
    let mr = crate::comparison::MemorySnapshot::read(&reg);
    before.write_back();
    o(this, manager, club, contratado);
    let mo = crate::comparison::MemorySnapshot::read(&reg);
    SCOREBOARD.record(mo == mr, || format!("record_manager_job_change(tecnico {:#x}, clube {:#x}, {}) original={:x?} rust={:x?}", manager, club, contratado as u8, mo, mr));
}

/// Comparer for `add_team_of_year_record`.
pub(crate) unsafe extern "thiscall" fn compare_add_team_of_year_record(this: usize, person: usize, club: usize, prize: usize, a: u32, b: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32)>(IDX_ADD_TEAM_OF_YEAR_RECORD)(this, person, club, prize, a, b)
}

/// Comparer for `staff_records_compare_clubs_by_manager_points`.
pub(crate) unsafe extern "C" fn compare_staff_records_compare_clubs_by_manager_points(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_STAFF_RECORDS_COMPARE_CLUBS_BY_MANAGER_POINTS)(a, b);
    let n = staff_records_compare_clubs_by_manager_points(a, b);
    SCOREBOARD.record(r == n, || format!("staff_records_compare_clubs_by_manager_points(clubes {:x}, {:x}) original={} rust={}", read_usize(a), read_usize(b), r, n));
    r
}

/// Comparer for `add_promotion_record`.
///
/// [pt] Comparador: lado a lado pelas regiões (a cópia sem os históricos em arquivo, que a original já gravou, e sem contar
/// os sorteios); com renovação de contrato ou clube/competição nulos, só a original.
pub(crate) unsafe extern "thiscall" fn compare_add_promotion_record(this: usize, club: usize, comp: usize, weight: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(IDX_ADD_PROMOTION_RECORD);
    let reg = if club != 0 && comp != 0 { promotion_regions(this, Club(club), true) } else { None };
    let reg = match reg {
        Some(r) => r,
        None => return o(this, club, comp, weight),
    };
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, club, comp, weight),
        || crate::rng::without_counting(|| apply_promotion(this, Club(club), Competition(comp), weight, false)));
    SCOREBOARD.record(igual, || format!("add_promotion_record(clube {:x}, peso {}) {:x?} x {:x?}", club, weight as u8 as i8, mo, mr));
}

/// Comparer for `add_relegation_record`.
///
/// [pt] Comparador: como o do acesso (sem o registro de pontos, que o rebaixamento não usa).
pub(crate) unsafe extern "thiscall" fn compare_add_relegation_record(this: usize, club: usize, comp: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_ADD_RELEGATION_RECORD);
    let reg = if club != 0 && comp != 0 { promotion_regions(this, Club(club), false) } else { None };
    let reg = match reg {
        Some(r) => r,
        None => return o(this, club, comp),
    };
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, club, comp),
        || crate::rng::without_counting(|| apply_relegation(this, Club(club), Competition(comp), false)));
    SCOREBOARD.record(igual, || format!("add_relegation_record(clube {:x}) {:x?} x {:x?}", club, mo, mr));
}

/// Comparer for `add_qualified_for_round_record`.
///
/// [pt] Comparador: lado a lado pelo registro de pontos do técnico (criado antes, como a original faria), a cópia sem o
/// histórico em arquivo; sem clube ou sem técnico, só a original.
pub(crate) unsafe extern "thiscall" fn compare_add_qualified_for_round_record(this: usize, club: usize, comp: usize, stage: u32, round: u32, weight: u32) {
    let club = Club(club);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32)>(IDX_ADD_QUALIFIED_FOR_ROUND_RECORD);
    if club.is_null() || club.manager().addr() == 0 {
        return o(this, club.addr(), comp, stage, round, weight);
    }
    let r = exe::staff_records_get_manager_record(this, club.manager().addr(), 1);
    if r == 0 {
        return o(this, club.addr(), comp, stage, round, weight);
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(r, 0xd2)], || o(this, club.addr(), comp, stage, round, weight),
        || record_qualification(this, club, Competition(comp), stage, round, weight, false));
    SCOREBOARD.record(igual, || format!("add_qualified_for_round_record(clube {:x}, peso {}) {:x?} x {:x?}", club.addr(), weight as u8 as i8, mo, mr));
}

/// Comparer for `add_transfer_records`.
///
/// Side by side over global [0xae1080], the random generator state (the offer value may ask for the valuation), the
/// transfer list (the cursor) and the records of the two managers (created beforehand only when the original asks for
/// them); the copy runs without the file histories, which the original has already written, and without counting the
/// random draws.
///
/// [pt] Comparador: lado a lado pela global [0xae1080], o estado do sorteador (o valor da oferta pode pedir a avaliação), a
/// lista da troca (o cursor) e os registros dos dois técnicos (criados antes só quando a original os pede); a cópia
/// sem os históricos em arquivo, que a original já gravou, e sem contar os sorteios.
pub(crate) unsafe extern "thiscall" fn compare_add_transfer_records(this: usize, t: usize) {
    let t = Offer(t);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_ADD_TRANSFER_RECORDS);
    let (pessoa, _, vendedor, comprador) = transfer_parties(t);
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.push((G_CONTRACT_LOOKUP_RESULT, 4));
    if read_usize(t.addr() + 4) != 0 {
        reg.push((read_usize(t.addr() + 4), 0xe));
    }
    for (clube, cargo) in [(comprador, t.job_type()), (vendedor, read_u8(pessoa + staff::JOB_FOR_CLUB))] {
        if clube != 0 && read_usize(clube + club::MANAGER) != 0 && staff_records_is_player_job(cargo) {
            let r = staff_records_manager_record(this, Staff(read_usize(clube + club::MANAGER)));
            if r != 0 {
                reg.push((r, 0xd2));
            }
        }
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, t.addr()),
        || crate::rng::without_counting(|| record_transfer(this, t, false)));
    SCOREBOARD.record(igual, || format!("add_transfer_records(pessoa {:x}) {:x?} x {:x?}", pessoa, mo, mr));
}

/// Comparer for `add_knocked_out_record`.
pub(crate) unsafe extern "thiscall" fn compare_add_knocked_out_record(this: usize, club: usize, comp: usize, stage: u32, round: u32, x: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32)>(IDX_ADD_KNOCKED_OUT_RECORD)(this, club, comp, stage, round, x)
}

/// Comparer for `add_failed_to_qualify_record`.
pub(crate) unsafe extern "thiscall" fn compare_add_failed_to_qualify_record(this: usize, club: usize, comp: usize, stage: u32, x: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(IDX_ADD_FAILED_TO_QUALIFY_RECORD)(this, club, comp, stage, x)
}

/// Comparer for `staff_records_daily_update`.
///
/// [pt] Comparador: os pontos vão ao hall da fama (0x5ef270, soma): roda só a original e confere o plano: na virada, os pontos
/// do mês zerados e o mês e o ano guardados; no primeiro dia do ano, os pontos do ano zerados.
pub(crate) unsafe extern "thiscall" fn compare_staff_records_daily_update(this: usize, data: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_STAFF_RECORDS_DAILY_UPDATE);
    let virada = date_month(data) as i32 != read_i16(this + staff_records_fields::CURRENT_MONTH) as u16 as i32 || read_i16(data + 2) != read_i16(this + staff_records_fields::CURRENT_YEAR);
    let year = read_i16(data) == 0;
    let (m, a) = (date_month(data), read_i16(data + 2));
    o(this, data);
    if !virada && !year {
        return;
    }
    let mut ok = !virada || (read_i16(this + staff_records_fields::CURRENT_MONTH) == m && read_i16(this + staff_records_fields::CURRENT_YEAR) == a);
    let mut first = -1i32;
    for k in 0..manager_record_count().max(0) {
        let e = ManagerRecord(manager_record_at(this, k));
        let certo = (!virada || ((e.month_points() as usize) == 0 && e.month_league_points() == 0)) && (!year || (e.year_points() as usize) == 0);
        if !certo && first < 0 {
            first = k;
        }
        ok &= certo;
    }
    SCOREBOARD.record(ok, || format!("staff_records_daily_update(mês {}, ano {}) virada {} ano novo {}: registro {} não zerado", m, a, virada, year, first));
}

/// Comparer for `staff_records_end_season_for_nation`.
///
/// [pt] Comparador: os pontos vão ao hall da fama (0x5ef330, soma): roda só a original e confere o plano feito antes: os
/// quatro campos (+0xba, +0xbe, +0xc2, +0xc6) de cada registro.
pub(crate) unsafe extern "thiscall" fn compare_staff_records_end_season_for_nation(this: usize, entry: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_STAFF_RECORDS_END_SEASON_FOR_NATION);
    let p = read_usize(entry + 4);
    if !(p != 0 && read_usize(p) == read_usize(KEY_CONTINENT_EUROPE)) && read_usize(entry) == 0 {
        return o(this, entry);
    }
    let comp = read_usize(entry);
    let id = if comp != 0 { read_i32(comp) } else { -1 };
    let n = manager_record_count().max(0);
    let mut plan = Vec::with_capacity(n as usize);
    for k in 0..n {
        let s = manager_record_at(this, k) + 0xba;
        let (mut liga, mut copa, mut cl, mut cc) = (read_usize(s), read_usize(s + 4), read_i32(s + 8), read_i32(s + 0xc));
        if cl == id {
            if cc == id {
                cc = -1; // the cup added to the league, which goes to the hall of fame and is zeroed
                copa = 0;
            }
            liga = 0;
            cl = -1;
        } else if cc == id {
            copa = 0;
            cc = -1;
        }
        plan.push((liga, copa, cl, cc));
    }
    o(this, entry);
    let wrong = (0..n).find(|&k| {
        let s = manager_record_at(this, k) + 0xba;
        plan[k as usize] != (read_usize(s), read_usize(s + 4), read_i32(s + 8), read_i32(s + 0xc))
    });
    SCOREBOARD.record(wrong.is_none(), || format!("staff_records_end_season_for_nation(competição {}) registro {:?} difere do plano", id, wrong));
}

/// Comparer for `find_manager_of_the_month`.
///
/// [pt] Comparador: pelo valor (o registro 0x86f5b0 criado pela original já existe para a cópia).
pub(crate) unsafe extern "thiscall" fn compare_find_manager_of_the_month(this: usize, index: i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32) -> usize>(IDX_FIND_MANAGER_OF_THE_MONTH);
    let league = League(read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((index as usize).wrapping_mul(4))));
    if league.comp_type() != 2 {
        return o(this, index);
    }
    let r = o(this, index);
    let n = find_manager_of_the_month(this, index);
    SCOREBOARD.record(r == n, || format!("find_manager_of_the_month({}) original={:x} rust={:x}", index, r, n));
    r
}

/// Comparer for `get_manager_of_year_candidates`.
///
/// [pt] Comparador: lado a lado pelo vetor dos clubes e pela data estática da 0x54cab0, e a devolução (os registros dos
/// técnicos que a 0x86f5b0 cria na original já existem para a cópia).
pub(crate) unsafe extern "thiscall" fn compare_get_manager_of_year_candidates(this: usize, clubs: usize, n: u32) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u8>(IDX_GET_MANAGER_OF_YEAR_CANDIDATES);
    let k = (n as u16 as i16).max(0) as usize;
    let reg = [(clubs, 4 * k), (G_SUB_WEEKS_RESULT_DATE, 8), (0xae24aa, 1)];
    let (r, c, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, clubs, n), || get_manager_of_year_candidates(this, clubs, n));
    SCOREBOARD.record(igual && r == c, || format!("get_manager_of_year_candidates({}) original={} rust={} {:x?} x {:x?}", k, r, c, mo, mr));
    r
}

/// Comparer for `get_manager_record`.
///
/// [pt] Comparador: lado a lado pelo registro do técnico (e, quando a vaga é de outro, pelo registro que a original zera),
/// pela devolução. Pessoa nula (a caixa) e o humano sem situação: só a original.
pub(crate) unsafe extern "thiscall" fn compare_get_manager_record(this: usize, person: usize, create: u32) -> usize {
    let person = Staff(person);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(IDX_GET_MANAGER_RECORD);
    if person.is_null() {
        return o(this, person.addr(), create);
    }
    let Some(r) = manager_manager_manager_record(this, person) else {
        return o(this, person.addr(), create);
    };
    let mut reg = vec![(r, 0xd2)];
    let owner = read_u32(r + manager_record::OWNER) as i32;
    if owner != -1 && owner != read_u32(person.addr()) as i32 {
        let z = (read_u32(this + staff_records_fields::MANAGER_RECORDS) as usize).wrapping_add((read_u32((person.non_player().addr() as u32) as usize) as usize).wrapping_mul(0xd2));
        if z != r {
            reg.push((z, 0xd2));
        }
    }
    let (a, b, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(this, person.addr(), create), || get_manager_record(this, person.addr(), create));
    SCOREBOARD.record(igual && a == b, || format!("get_manager_record(pessoa {:x}, criar {}) original={:x} rust={:x} {:x?} x {:x?}", person.addr(), create, a, b, mo, mr));
    a
}

/// Comparer for `add_free_release_records`.
pub(crate) unsafe extern "thiscall" fn compare_add_free_release_records(this: usize, person: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_ADD_FREE_RELEASE_RECORDS)(this, person)
}

/// Comparer for `add_loan_records`.
pub(crate) unsafe extern "thiscall" fn compare_add_loan_records(this: usize, person: usize, de: usize, club: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(IDX_ADD_LOAN_RECORDS)(this, person, de, club)
}

/// Comparer for `rewind_staff_history`.
///
/// [pt] Comparador: lado a lado pelo par da pessoa (8 bytes; a 0x58f110 só grava o segundo).
pub(crate) unsafe extern "thiscall" fn compare_rewind_staff_history(this: usize, person: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_REWIND_STAFF_HISTORY);
    if person == 0 {
        return o(this, person);
    }
    let par = read_usize(this + staff_records_fields::PERSON_HISTORY_LISTS).wrapping_add(read_usize(person).wrapping_mul(8));
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&[(par, 8)], || o(this, person), || rewind_staff_history(this, person));
    SCOREBOARD.record(igual, || format!("rewind_staff_history({}) {:x?} x {:x?}", read_usize(person), mo, mr));
}

/// Comparer for `add_year_award_record`.
///
/// The manager side by side through his points record (created beforehand by `get_manager_record`, as the original routine would
/// do; the copy without the history entry in the file).
///
/// The player draws at random, renews the contract and sends a news item (it cannot run twice): only the original.
/// Without a person or without an award (the error box): only the original.
///
/// [pt] Comparador: o técnico lado a lado pelo registro de pontos dele (criado antes pela 0x86f5b0, como a original faria;
/// a cópia sem a entrada do histórico em arquivo). O jogador sorteia, renova contrato e manda notícia (não pode rodar
/// duas vezes): só a original. Sem pessoa ou sem prêmio (a caixa): só a original.
pub(crate) unsafe extern "thiscall" fn compare_add_year_award_record(this: usize, p: usize, club: usize, prize: usize, a4: u32, a5: u32, credito: u32) {
    let p = Staff(p);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32)>(IDX_ADD_YEAR_AWARD_RECORD);
    if p.is_null() || prize == 0 {
        return o(this, p.addr(), club, prize, a4, a5, credito);
    }
    let (t3d, t28) = (p.job_for_club(), p.job_for_nation());
    if !(t3d == 5 || t3d == 0xc || t28 == 5 || t28 == 0xc) {
        crate::comparison::branch("add_year_award_record player");
        return o(this, p.addr(), club, prize, a4, a5, credito);
    }
    let r = staff_records_manager_record(this, p);
    if r == 0 {
        return o(this, p.addr(), club, prize, a4, a5, credito);
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(r, 0xd2)], || o(this, p.addr(), club, prize, a4, a5, credito),
        || add_year_award(this, p, club, prize, a4, a5, credito, false));
    SCOREBOARD.record(igual, || format!("add_year_award_record({:x}, prêmio {:x}) {:x?} x {:x?}", p.addr(), read_usize(prize), mo, mr));
}

/// Comparer for `add_first_cap_record`.
pub(crate) unsafe extern "thiscall" fn compare_add_first_cap_record(this: usize, person: usize, opponent: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_ADD_FIRST_CAP_RECORD)(this, person, opponent)
}

/// Comparer for `add_month_award_record`.
///
/// The manager side by side on his points record (created beforehand by `get_manager_record`, as the original would; the copy
/// without the history entry in the file).
///
/// The player case draws at random, renews the contract and sends a news item (it cannot run twice): only the original
/// routine. No person or no award (the box): only the original.
///
/// [pt] Comparador: o técnico lado a lado pelo registro de pontos dele (criado antes pela 0x86f5b0, como a original faria;
/// a cópia sem a entrada do histórico em arquivo). O jogador sorteia, renova contrato e manda notícia (não pode rodar
/// duas vezes): só a original. Sem pessoa ou sem prêmio (a caixa): só a original.
pub(crate) unsafe extern "thiscall" fn compare_add_month_award_record(this: usize, p: usize, club: usize, prize: usize, a4: u32, a5: u32) {
    let p = Staff(p);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32)>(IDX_ADD_MONTH_AWARD_RECORD);
    if p.is_null() || prize == 0 {
        return o(this, p.addr(), club, prize, a4, a5);
    }
    let (t3d, t28) = (p.job_for_club(), p.job_for_nation());
    if !(t3d == 5 || t3d == 0xc || t28 == 5 || t28 == 0xc) {
        crate::comparison::branch("add_month_award_record player");
        return o(this, p.addr(), club, prize, a4, a5);
    }
    let r = staff_records_manager_record(this, p);
    if r == 0 {
        return o(this, p.addr(), club, prize, a4, a5);
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(r, 0xd2)], || o(this, p.addr(), club, prize, a4, a5),
        || add_month_award_record_body(this, p, club, prize, a4, a5, false));
    SCOREBOARD.record(igual, || format!("add_month_award_record({:x}, prêmio {:x}) {:x?} x {:x?}", p.addr(), read_usize(prize), mo, mr));
}

/// Comparer for `clear_manager_record`.
pub(crate) unsafe extern "thiscall" fn compare_clear_manager_record(records: usize, id: *const i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *const i32)>(IDX_CLEAR_MANAGER_RECORD);
    let r = friendly_manager_record(records, id);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(r, 210)], || o(records, id), || clear_manager_record(records, id));
    SCOREBOARD.record(igual, || format!("clear_manager_record({}) {:x?} x {:x?}", *id, mo, mr));
}
