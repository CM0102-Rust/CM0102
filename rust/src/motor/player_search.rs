use super::*;

/// Person -> club of their contract (stored in G_CONTRACT_LOOKUP_RESULT), or 0.
pub(crate) unsafe fn player_search_contract_club(person: Staff) -> usize {
    // 0x4cf5a0: push person; mov ecx, [0xae1050] -> thiscall, ret 4
    let c = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person));
    write_usize(G_CONTRACT_LOOKUP_RESULT, c.addr());
    if !c.is_null() { (friendly_club_by_id(c.club())).addr() } else { 0 }
}

/// `player_search_record_scouted_club_match(game, result)`, called by the watched match (`player_search_scout_match`)
/// when the list follows a club (+0x190) that plays in it: records the match in the watched club's player histories.
/// `this` is the record of the watched club ([list+0x170]: +0 the club id, 50 slots of 0x1a2 bytes from +4). First the
/// slots of those who left the club are cleared (id −1, the counter +4 and bytes
/// `scouted_player_slot::HISTORY_START`/`scouted_player_slot::HISTORY_COUNT` zeroed); then, on the club's side of the
/// result (`match_record::ROW_STAFF_ID`, 20 records of `match_record_row::SIZE`, the away side at
/// `match_record::SIDE_SIZE`), each player who played (+4 ≠ −1; substitutes beyond the 11th only with a rating,
/// `minute_on()`) gets a line in the 10-match history of his slot (`player_search_find_or_claim_player_slot` with 1;
/// without a slot, the box of line 0x2065): goals, rating, date, opponent, score, competition, the side
/// (`comp_util_get_fixture_venue_side`) and the statistics; the type +0x2e (1 starter, 2 substitute, 5/6 with the
/// rating from `minute_off()`) and the rating +0x15. With the manager of a human club and the attribute masks set, the
/// manager comes to know the player (`exe::fog_of_war_map_raise_knowledge`, which draws).
/// Original: `player_search_record_scouted_club_match` (0x7bd310, player_search.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn player_search_record_scouted_club_match(this: usize, game: usize, result: usize) {
    let result = MatchRecord(result);
    let game = Fixture(game);
    let club_obs = friendly_club_by_id(read_usize(this));
    for i in 0..0x32usize {
        let v = this + 4 + i * 0x1a2;
        let id = read_i32(v);
        if id == -1 {
            continue;
        }
        let p = staff_records_staff_by_id(id as usize);
        if !p.is_null() && read_i32(p.addr()) >= read_i32(G_STAFF_COUNT) {
            continue;
        }
        let c = p.club_job().addr();
        if c != 0 && read_usize(c) == read_usize(this) {
            continue;
        }
        write_i32(v, -1);
        write_u16(v + 4, 0);
        write_u8(v + scouted_player_slot::HISTORY_START, 0);
        write_u8(v + scouted_player_slot::HISTORY_COUNT, 0);
    }
    // the club’s side: 0 if it is the match’s +0xc (the opponent is +0x10), otherwise 1 (opponent +0xc)
    let (lado, adversario): (i8, u32) = if game.home_club_id() == read_usize(this) {
        (0, game.away_club_id() as u32)
    } else {
        (1, game.home_club_id() as u32)
    };
    // 0x600a50: 5 pushes (club, 0, 1, 0, 0); add esp, 0x14 -> cdecl; 0x5afde0: push manager; mov ecx, [0xae2a60] ->
    // thiscall; 0x5b09d0: push 0; push player; mov ecx, eax -> thiscall, ret 8; 0x7bd700: push 1; push player; mov ecx,
    // this -> thiscall, ret 8; 0x4afa80: push jogo; push clube; add esp, 8 -> cdecl
    for j in 0..0x14i8 {
        let e = MatchRecordRow(result.addr() + match_record::ROW_STAFF_ID + lado as usize * match_record::SIDE_SIZE + j as usize * match_record_row::SIZE);
        let id = read_i32(e.addr());
        if id == -1 || read_i32(e.addr() + 4) == -1 {
            continue;
        }
        if j > 10 && (e.minute_on() as u8) == 0xff {
            continue;
        }
        let p = staff_records_staff_by_id(id as usize);
        if !p.is_null() && read_i32(p.addr()) >= read_i32(G_STAFF_COUNT) {
            continue;
        }
        if club_obs.manager().addr() != 0 && exe::human_manager_is_human_club(club_obs, 0, 1, 0, 0) != 0 && read_u8(G_ATTRIBUTE_MASKING_ENABLED) != 0 {
            let q = exe::fog_of_war_get_manager_map(read_usize(G_FOG_OF_WAR), club_obs.manager().addr());
            if q != 0 {
                exe::fog_of_war_map_raise_knowledge(q, p, 0);
            }
        }
        let r = exe::player_search_find_or_claim_player_slot(this, p, 1);
        if r == 0 {
            show_error_at(0xa701b0, 0x2065);
            return;
        }
        let how_many = read_u8(r + scouted_player_slot::HISTORY_COUNT);
        write_u16(r + 4, (read_i16(r + 4) as u16).wrapping_add(1));
        let mut k = read_u8(r + scouted_player_slot::HISTORY_START);
        if how_many == 0xa {
            // the history is full: the oldest row (+0x1a0) is the current one, and the start moves
            let n = k.wrapping_add(1);
            write_u8(r + scouted_player_slot::HISTORY_START, if n == 0xa { 0 } else { n });
        } else {
            k = k.wrapping_add(how_many);
            if (k as i8) >= 0xa {
                k = k.wrapping_add(0xf6);
            }
            write_u8(r + scouted_player_slot::HISTORY_COUNT, how_many.wrapping_add(1));
        }
        let l = r.wrapping_add((k as i8 as isize * 0x29) as usize);
        let por = |o: usize, v: u8| write_u8(l + o, v);
        por(0x17, e.assists());
        por(0x18, e.goals_conceded() as u8);
        write_u16(l + 6, (game.day() as i16) as u16);
        write_u16(l + 8, (game.year() as i16) as u16);
        por(0x16, e.goals());
        por(0x1b, e.match_grade());
        por(0x1a, e.red_card());
        por(0x19, e.yellow_cards());
        let (m, n) = if lado == 0 { (0x43, 0x44) } else { (0x44, 0x43) };
        por(0xe, read_u8(game.addr() + m));
        por(0xf, read_u8(game.addr() + n));
        write_u32(l + 0x10, game.match_stats_slot() as u32);
        por(0x14, exe::comp_util_get_fixture_venue_side(friendly_club_by_id(read_usize(this)), game) as u8);
        for t in 0..10 {
            por(0x1c + t, read_u8(e.addr() + match_record_row::PASS_TACKLE_COUNTS + t));
        }
        por(0x26, e.dribbles());
        por(0x27, e.shots());
        por(0x28, e.shots_on_target());
        por(0x29, e.fouls());
        por(0x2a, e.fouls_suffered());
        por(0x2b, e.offsides());
        por(0x2c, e.penalties_taken());
        por(0x2d, e.penalties_scored());
        por(0x2e, 0);
        let adv = if read_usize(game.addr()) == read_usize(KEY_CLUB_COMP_RESERVE) { adversario.wrapping_neg() } else { adversario };
        write_u32(l + 0xa, adv);
        if j < 0xb {
            por(0x2e, 1);
            let a = e.minute_left_pitch();
            if (a as i8) > -1 {
                por(0x15, a);
            } else if ((e.minute_off() as u8) as i8) > -1 {
                por(0x2e, 5);
                por(0x15, e.minute_off() as u8);
            } else {
                por(0x15, result.minute() as u8);
            }
        } else {
            por(0x2e, 2);
            let c = e.minute_on() as u8;
            if c == 0xff {
                por(0x15, 0);
            } else {
                let a = e.minute_left_pitch();
                if (a as i8) > -1 {
                    por(0x15, a.wrapping_sub(c));
                } else if ((e.minute_off() as u8) as i8) > -1 {
                    por(0x2e, 6);
                    por(0x15, (e.minute_off() as u8).wrapping_sub(c));
                } else {
                    por(0x15, (result.minute() as u8).wrapping_sub(c));
                }
            }
        }
    }
}

/// The dry run of `player_search_scout_match` (only in the comparer): the copy decides everything up to the report without the effects
/// (the history `player_search_record_scouted_club_match` and the report do not run) and notes whether it would reach the report.
pub(crate) static SCOUT_MATCH_DRY_RUN: AtomicBool = AtomicBool::new(false);
pub(crate) static SCOUT_MATCH_WOULD_REPORT: AtomicBool = AtomicBool::new(false);

/// `PLAYER_SEARCH::scout_match(match, result, out)` (SEH frame for the local list and dates), called by the watched
/// match (`scout_manager_scout_match`): the matching scout list (type 4) watches the played match and produces reports.
///
/// `this` is the scout's search list: +0 the owner, +0xa the type (4 = watch matches), +0x170 the watched club's
/// record, +0x174..+0x184 the scout's names, +0x180 his id, +0x190 the followed club, +0x195 the followed player,
/// +0x1a5/+0x1ad the start and the end. Without a match or result: the box of line 0xef1. Of type 4, with a scout that
/// checks out (`database_staff_matches_names`) and the result of a played match (+0x4c zero): the followed club, if it plays, gets the
/// history (`player_search_record_scouted_club_match`); within the dates (before the end only with a followed player), if the match is the followed
/// player's, it reports; otherwise the search must cover the competition's nation (`scout_covers_nation`) and the scout's club must
/// have asked for (`collect_club_scout_targets`, `has_scout_request` with 1) some player of one of the two clubs.
///
/// The report: the list's start moves to tomorrow; on both sides (+0x1c0 / +0x1c0+0x625, 20 records of 0x49), each
/// valid player of another club (substitutes beyond the 11th only with a rating) becomes known to the owner (`exe::fog_of_war_map_raise_knowledge`,
/// with the masks) and, if asked for (`has_scout_request` with 0), followed or with a seen reputation below 4000 (`database_get_staff_reputation_for_viewer`) and a
/// rating of at least 15 (90 with a result without ratings), becomes a report (`exe::player_search_report_on_player`, mode 0 followed, 1 asked for,
/// 2 with +5 ≥ 8) that goes into the output (0x68d0d0, 0x67 bytes) if byte +0x66 ended up ≥ 0. Returns 1 with a report,
/// otherwise 0.
/// Original: `PLAYER_SEARCH::scout_match` (0x7b5be0, player_search.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn player_search_scout_match(this: usize, game: usize, result: usize, out: usize) -> u32 {
    // 0x68d020 / 0x68d040: lea ecx, local -> thiscall
    let mut list = [0u8; 16];
    let l = list.as_mut_ptr() as usize;
    exe::new_llist(l);
    let r = scout_match_body(this, Fixture(game), result, out, l);
    exe::drop_llist(l);
    r
}

/// Body of player_search_scout_match over a built local list: checks the scout/search and produces scouting reports for
/// the match.
pub(crate) unsafe fn scout_match_body(this: usize, game: Fixture, result: usize, out: usize, l: usize) -> u32 {
    let result = MatchRecord(result);
    if game.is_null() || result.is_null() {
        show_error_at(0xa701b0, 0xef1);
        return 0;
    }
    let owner = staff_records_staff_by_id(read_usize(this));
    // the nation of the match’s competition (+0x18 → +0x38 → +0x1f) or the home club’s (+0x1c → +0x53)
    let mut nation = 0usize;
    let t = Stadium(game.stadium());
    if !t.is_null() && t.city() != 0 && read_usize(t.city() + city::NATION) != 0 {
        nation = read_usize(t.city() + city::NATION);
    }
    if nation == 0 {
        nation = game.home_club().nation().addr();
    }
    if read_u8(this + player_search_fields::LIST_TYPE) != 4 {
        return 0;
    }
    let id_scout = read_i32(this + player_search_fields::SCOUT_ID);
    if id_scout <= -1 {
        return 0;
    }
    // 0x53e990: 5 pushes; add esp, 0x14 -> cdecl
    if exe::database_staff_matches_names(staff_records_staff_by_id(id_scout as usize), read_usize(this + player_search_fields::SCOUT_NAME_REF) as u32, read_usize(this + player_search_fields::SCOUT_NAME_REF + 4) as u32, read_usize(this + player_search_fields::SCOUT_NAME_REF + 8) as u32, this + player_search_fields::SCOUT_NAME_REF + 0x10) == 0 {
        return 0;
    }
    let scout = Staff((staff_records_staff_by_id(read_i32(this + player_search_fields::SCOUT_ID) as usize)).addr());
    if scout.is_null() || result.no_statistics() != 0 {
        return 0;
    }
    let followed = read_i32(this + player_search_fields::ASSIGNED_CLUB);
    if followed > -1 && read_usize(this + player_search_fields::SCOUTED_CLUB_RECORD) != 0 && ((game.home_club_id() as i32) == followed || (game.away_club_id() as i32) == followed) {
        if !SCOUT_MATCH_DRY_RUN.load(Relaxed) {
            covered!("cqf_10 club history");
            // 0x7bd310: push resultado; push jogo; mov ecx, [this+0x170] -> thiscall, ret 8
            exe::player_search_record_scouted_club_match(read_usize(this + player_search_fields::SCOUTED_CLUB_RECORD), game, result.addr());
        }
    }
    // the dates: today before the start (+0x1a5: day, +0x1a7: year) leaves; before the end (+0x1ad/+0x1af) only with a player
    // accompanied; year unsigned, day signed, like the original
    let (dia, ano) = (read_i16(G_CURRENT_DATE), read_i16(G_CURRENT_DATE_YEAR) as u16);
    let (dia_ini, ano_ini) = (read_i16(this + player_search_fields::SCOUTING_START_DATE), read_i16(this + player_search_fields::SCOUTING_START_YEAR) as u16);
    if ano < ano_ini || (ano == ano_ini && dia < dia_ini) {
        return 0;
    }
    let (dia_fim, ano_fim) = (read_i16(this + player_search_fields::SCOUTING_END_DATE), read_i16(this + player_search_fields::SCOUTING_END_YEAR) as u16);
    let before_of_end = ano < ano_fim || (ano == ano_fim && dia < dia_fim);
    if before_of_end && read_i32(this + player_search_fields::ASSIGNED_COMP) == -1 {
        return 0;
    }
    let target = read_i32(this + player_search_fields::ASSIGNED_COMP);
    let of_target = target > -1 && read_i32(game.addr()) == target;
    if !of_target {
        // 0x7b7490: push nation; mov ecx, this -> thiscall, ret 4
        if exe::player_search_scout_covers_nation(this, nation) == 0 {
            return 0;
        }
        let request: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_SCOUT_MANAGER_HAS_SCOUT_REQUEST);
        // 0x7f9690: push &list; push scout’s club; mov ecx, [0xb74448] -> thiscall, ret 8; 0x68d1b0: lea ecx, list
        // -> thiscall; 0x7f8d70: push 1; push club; push player; mov ecx, [0xb74448] -> thiscall, ret 0xc
        exe::scout_manager_collect_club_scout_targets(read_usize(G_SCOUT_MANAGER), scout.club_job(), l);
        let count = read_i16(l);
        if count == 0 {
            return 0;
        }
        write_usize(l + llist::CURSOR, read_usize(l + 2)); // the cursor goes back to the head
        let mut free = true; // the original’s ebp: no request found
        // simplification: the original uses the same register (edi) to count the items and the 20 slots of each side; after
        // an item from one of the clubs the counter becomes the number of slots looked at in the last pass (the outer loop ends first)
        let mut i: i32 = 0;
        if count > 0 {
            loop {
                let item = Staff(exe::llist_get_next(l));
                if !item.is_null() {
                    let (m, v) = (game.home_club(), game.away_club());
                    let c39 = item.club_job();
                    let c24 = item.national_job();
                    if c39 == m || c39 == v || c24 == m || c24 == v {
                        let mut side: i8 = 0;
                        while free {
                            i = 0;
                            let mut k = 0usize;
                            while free {
                                let id = read_i32(result.addr() + match_record::ROW_STAFF_ID + side as usize * match_record::SIDE_SIZE + k);
                                if id != -1 && request(read_usize(G_SCOUT_MANAGER), (staff_records_staff_by_id(id as usize)).addr(), scout.club_job().addr(), 1) != 0 {
                                    free = false;
                                }
                                k += 0x49;
                                i += 1;
                                if k >= 0x5b4 {
                                    break;
                                }
                            }
                            side += 1;
                            if side >= 2 {
                                break;
                            }
                        }
                    }
                }
                i += 1;
                if i >= read_i16(l) as i32 {
                    break;
                }
            }
        }
        if free {
            return 0;
        }
    }
    covered!("cqf_10 report");
    // the start of the list goes to tomorrow (0x54c5b0 with 1, 0x417c10 writes at +0x1a5); the low byte of the copied date dword
    // becomes the counter of both sides and goes whole as the 7th argument of 0x7b62a0
    // 0x54c5b0: push 1; push &tmp; mov ecx, 0xae2c90 -> thiscall, ret 8; 0x417c10: push data; push &copia; lea ecx,
    // [this+0x1a5] -> thiscall, ret 8
    let (mut tmp, mut copia) = ([0u32; 2], [0u32; 2]);
    let tomorrow = exe::date_add_days(G_CURRENT_DATE, tmp.as_mut_ptr() as usize, 1); // cqf10: amanhã
    if SCOUT_MATCH_DRY_RUN.load(Relaxed) {
        SCOUT_MATCH_WOULD_REPORT.store(true, Relaxed);
        return 1;
    }
    exe::date_operator_3d(this + player_search_fields::SCOUTING_START_DATE, copia.as_mut_ptr() as usize, tomorrow);
    let mut mark = copia[0] & !0xff;
    let request: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_SCOUT_MANAGER_HAS_SCOUT_REQUEST);
    // 0x5afde0: push owner; mov ecx, [0xae2a60] -> thiscall, ret 4; 0x5b09d0: push scout; push player; mov ecx, eax ->
    // thiscall, ret 8; 0x7f8d70: push 0; push club; push player; mov ecx, [0xb74448] -> thiscall, ret 0xc; 0x5407e0:
    // push 0; push olheiro; push jogador; add esp, 0xc -> cdecl; 0x7b62a0: 7 pushes; mov ecx, this -> thiscall, ret 0x1c;
    // 0x68d0d0: push 0x67; push &record; mov ecx, output -> thiscall, ret 8
    // the local record (0x67 bytes), reused between players as in the original, with the parts that the constructor of the
    // original builds on entry: the date at +0 (0x54a110) and the two record holders at +0x1f and +0x37 (0x450d40); the rest
    // starts zeroed (in the original, a stack leftover that 0x7b62a0 overwrites)
    let mut record = [0u8; 0x68];
    let reg = record.as_mut_ptr() as usize;
    // lea ecx, local -> thiscall in both
    exe::date_date_54a110(reg);
    exe::record_holder_construct(reg + scout_report::PLAYER_NAME_REF);
    exe::record_holder_construct(reg + scout_report::SCOUT_NAME_REF);
    let club_scout = scout.club_job();
    for side in 0..2usize {
        let mut e = result.addr() + match_record::ROW_MINUTE_OFF + side * match_record::SIDE_SIZE;
        for j in 0..0x14i32 {
            let current = e;
            e += 0x49;
            if j > 10 && read_u8(current + 2) == 0xff {
                continue;
            }
            let id = read_i32(current - 0x30);
            if id == -1 {
                continue;
            }
            let p = Staff((staff_records_staff_by_id(id as usize)).addr());
            if !p.is_null() && read_i32(p.addr()) >= read_i32(G_STAFF_COUNT) {
                continue;
            }
            if player_search_contract_club(p) == club_scout.addr() || p.player().is_null() {
                continue;
            }
            if read_u8(G_ATTRIBUTE_MASKING_ENABLED) != 0 {
                let q = exe::fog_of_war_get_manager_map(read_usize(G_FOG_OF_WAR), owner);
                if q != 0 {
                    exe::fog_of_war_map_raise_knowledge(q, p, scout);
                }
            }
            let pediu = request(read_usize(G_SCOUT_MANAGER), p.addr(), club_scout.addr(), 0);
            if pediu == 0 {
                let target = read_i32(this + player_search_fields::ASSIGNED_COMP);
                if !(target != -1 && read_i32(game.addr()) == target) && exe::database_get_staff_reputation_for_viewer(p, scout, 0) >= 0xfa0 { // cqf10: perceived reputation
                    continue;
                }
            }
            // the rating: starters (up to the 11th) by rating +4, else +0, else the result’s (+0x4d); substitutes deduct the
            // entry (+2); result without ratings (+0x4c): a starter counts 90
            let rating_of = |x: usize| -> u8 {
                let a = read_u8(x + 4);
                if (a as i8) > -1 { a } else if (read_u8(x) as i8) > -1 { read_u8(x) } else { result.minute() as u8  }
            };
            let rating: u8 = if j < 0xb {
                if result.no_statistics() != 0 { 0x5a } else { rating_of(current) }
            } else {
                let s = read_u8(current + 2);
                if s == 0xff {
                    continue;
                }
                rating_of(current).wrapping_sub(s)
            };
            if (rating as i8) < 0xf { // cqf10: a nota mínima
                continue;
            }
            let target = read_i32(this + player_search_fields::ASSIGNED_COMP);
            let mode: u32 = if target > -1 && target == read_i32(game.addr()) {
                0
            } else if pediu != 0 {
                1
            } else {
                if read_u8(current + 5) < 8 {
                    continue;
                }
                2
            };
            covered!("cqf_10 evaluate player");
            let n = ((read_u8(current + 5) as i32 * 0x28 - 0x104).wrapping_mul(rating as i8 as i32)) / 2;
            exe::player_search_report_on_player(this, p, reg, n, mode, game, result.addr(), mark);
            if (read_u8(reg + scout_report::REPORT_MODE) as i8) > -1 {
                exe::llist_add_tail(out, reg, 0x67);
            }
        }
        mark = (mark & !0xff) | ((mark as u8).wrapping_add(1)) as u32;
    }
    1
}

/// `PLAYER_SEARCH::contains_player(person)` (SEH frame only for the two local dates, no destructor), called from the
/// human manager's scout list (`is_player_shortlisted`, which the player assessment and the interested clubs consult
/// every day) and from screens: looks the player up in the search results and returns 1 when found and not marked as
/// removed. The local 0x2d-byte key starts with the two default dates (+0x10 and `search_result::LAST_UPDATE_DATE`,
/// `exe::date_date_54a110`); a null person: the box of line 0x33e and 0. Without results
/// (`player_search_fields::RESULT_COUNT` = count): 0. The reference to the person's name in the key
/// (`exe::fill_staff_name_ref`: +0xc the id, the only field the comparison 0x7bce50 reads) and the binary search in the
/// results (`exe::crt_bsearch` with the vector `player_search_fields::RESULTS`, the count, 0x2d and 0x7bce50); found
/// and without the removed flag (`search_result_flags::REMOVED` of `search_result::FLAGS`), 1; otherwise 0.
/// Original: `PLAYER_SEARCH::contains_player` (0x7af600, player_search.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn contains_player(this: usize, person: usize) -> u32 {
    let mut key = [0u8; 0x30];
    let k = key.as_mut_ptr() as usize;
    exe::date_date_54a110(k + 0x10);
    exe::date_date_54a110(k + search_result::LAST_UPDATE_DATE);
    if person == 0 {
        show_error_at(0xa701b0, 0x33e);
        return 0;
    }
    if read_usize(this + player_search_fields::RESULT_COUNT) == 0 {
        covered!("cqa_6 empty search");
        return 0; // cqa6: without results, it is not there
    }
    exe::fill_staff_name_ref(person, k);
    let r = exe::crt_bsearch(k, read_usize(this + player_search_fields::RESULTS), read_usize(this + player_search_fields::RESULT_COUNT) as u32, 0x2d, 0x7bce50);
    if r == 0 {
        return 0;
    }
    if read_u8(r + search_result::FLAGS) & (search_result_flags::REMOVED as u8) != 0 { // cqa6: the removed one does not count
        covered!("cqa_6 removed");
        return 0;
    }
    covered!("cqa_6 found");
    1 // cqa6: achado
}

/// SCOUT_MANAGER get_search(i, manager).
pub(crate) unsafe fn get_search(this: usize, i: u8, m: Staff) -> usize {
    exe::scout_manager_get_search(this, i as u32, m)
}

/// PLAYER_SEARCH get_scout of the search.
pub(crate) unsafe fn search_scout(l: usize) -> usize {
    exe::player_search_get_scout(l)
}

/// PLAYER_SEARCH set_scout(search, scout).
pub(crate) unsafe fn set_search_scout(l: usize, scout: usize) {
    exe::player_search_set_scout(l, scout);
}

/// `player_search_find_or_claim_player_slot(person, reservar)`: returns the player's slot among the 50 of the watched
/// club's record (0x1a2 bytes from +4: +0 the id, −1 free), called by its match
/// (`player_search_record_scouted_club_match`) and by the screens (`find_club_search_entry`). The slot with the
/// person's id; otherwise, with `reservar`, the first free one gets the id (null person: −1), zeroes word +4 and bytes
/// +0x1a0 and +0x1a1 and is returned (none free: the box of line 0x210d and 0); without `reservar`, 0.
/// Original: `player_search_find_or_claim_player_slot` (0x7bd700, player_search.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn player_search_find_or_claim_player_slot(this: usize, person: usize, reservar: u32) -> usize {
    if person == 0 {
        // simplification: the original reads the person’s id before checking the pointer (it would crash); kept as in the original
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(IDX_PLAYER_SEARCH_FIND_OR_CLAIM_PLAYER_SLOT)(this, person, reservar);
    }
    let id = read_usize(person) as i32;
    let slot = |i: i8| this + 4 + i as usize * 0x1a2;
    let mut free: i8 = -1;
    for i in 0..0x32i8 {
        let v = read_usize(slot(i)) as i32;
        if v == id {
            return slot(i);
        }
        if reservar != 0 && free == -1 && v == -1 {
            free = i;
        }
    }
    if reservar == 0 {
        return 0;
    }
    if free == -1 {
        show_error_at(0xa701b0, 0x210d);
        return 0;
    }
    let a = slot(free);
    write_i32(a, id);
    write_u16(a + 4, 0);
    write_u8(a + scouted_player_slot::HISTORY_START, 0);
    write_u8(a + scouted_player_slot::HISTORY_COUNT, 0);
    a
}

/// `player_search_get_club_target_rating(club, *base, *forced)` (cdecl): the rating the club looks for in players: a
/// base from reputation, minimum attendance, division and continental competition, by bands, and adjustments for squad
/// size and wealth; at least 500. The base (without `forced`): the reputation (`reputation()`), the attendance one
/// (`min_attendance()` × 0.43 − 430, up to 10000, if greater than the reputation), the division one (`division()`:
/// `competition::REPUTATION` × 500, if greater than reputation / 500) and the continental one (`continental_comp_id()`
/// in `G_CLUB_COMP_TABLE`, 0x6b bytes: `reputation()` × 600, likewise), weighted and / 16; half at a non-professional
/// club (`professional_status()` ≠ `professional_status::PROFESSIONAL`). The rating comes from the base by bands (up to
/// 3000, 5000, 7000, 9000 and above). Without `forced`, by the squad (`database_count_club_squad`) and the wealth band
/// of the finances (`exe::financetable_get_club_finance` of `G_FINANCE_TABLE`, `exe::finance_get_wealth_band`) for
/// clubs whose division is in `G_COMP_OBJECT_TABLE`: a short squad (< 16) halves; < 20 halves, or 2/3 for the poor;
/// otherwise it shrinks by the shortage of players compared with the division (its `competition::REPUTATION` × 1.35 +
/// 20) and wealth pulls it (−2..2: × 0.909 or − 1000, × 0.952 or − 500, nothing, × 1.0125 or + 125, × 1.025 or + 250).
/// At least 500. The base goes into `*base` (if not null). The floating-point arithmetic is in f64 (the game's x87 at
/// 53 bits).
/// Original: `player_search_get_club_target_rating` (0x7bca90, player_search.cpp, cdecl).
pub(crate) unsafe extern "C" fn player_search_get_club_target_rating(club: usize, base: *mut i32, forced: *const i16) -> i32 {
    let club = Club(club);
    let li = club.continental_comp_id() as i32;
    let league = Competition(if li > -1 { (read_i32(G_CLUB_COMP_TABLE) as usize).wrapping_add((li as usize).wrapping_mul(0x6b)) } else { 0 });
    let nation = (club.division().addr() as i32) as usize;
    let mut esi: i32;
    if !forced.is_null() {
        esi = *forced as i32;
    } else {
        let rep = club.reputation() as i32;
        let t = ftol(club.min_attendance() as f64 * read_f64(rdata::F64_0_43) - read_f64(rdata::F64_430)) as i16 as i32;
        let ebx = (if rep > t { rep } else if t > 10000 { 10000 } else { t }) as i16 as i32;
        let mut n = rep;
        let mut l = rep;
        if nation != 0 {
            let c = read_i16(nation + competition::REPUTATION) as i32;
            if c > rep / 500 {
                n = c * 500;
                l = n;
            }
        }
        if !league.is_null() {
            let c = league.reputation() as i32;
            if c > rep / 500 {
                l = c * 600;
            }
        }
        let sum = if n < 8000 {
            covered!("cpu_13 base by reputation");
            n.wrapping_mul(3).wrapping_add(ebx).wrapping_add(l.wrapping_add(rep.wrapping_mul(2)).wrapping_mul(4))
        } else {
            ebx.wrapping_add(rep.wrapping_mul(7)).wrapping_add(n.wrapping_add(l).wrapping_mul(4))
        };
        esi = sum / 16; // cpu13: a base
    }
    if club.professional_status() != professional_status::PROFESSIONAL {
        esi /= 2;
    }
    let mut edi: i32 = if esi > 9000 {
        esi.wrapping_mul(3).wrapping_add(7500)
    } else if esi > 7000 {
        ftol(esi as f64 * read_f64(rdata::F64_4_75) - read_f64(rdata::F64_8250))
    } else if esi > 5000 {
        ftol(esi as f64 * read_f64(rdata::F64_2_5) + read_f64(rdata::F64_7500))
    } else if esi > 3000 {
        covered!("cpu_13 band from 3000 to 5000");
        esi.wrapping_mul(5).wrapping_sub(5000) // cpu13: the middle band
    } else {
        ftol(esi as f64 * read_f64(rdata::F64_1_6) + read_f64(rdata::F64_5000))
    };
    if forced.is_null() {
        let squad = exe::database_count_club_squad(club, 0, 0) as i8;
        let with_nation = |c: usize| c != 0 && read_i32((read_i32(G_COMP_OBJECT_TABLE) as usize).wrapping_add(read_i32(c) as usize * 4)) != 0;
        if squad < 16 && with_nation(nation) {
            edi /= 2;
        } else if squad < 20 && with_nation(nation) {
            let fin = exe::financetable_get_club_finance(read_i32(G_FINANCE_TABLE) as usize, club);
            if fin != 0 {
                match exe::finance_get_wealth_band(fin) as i8 {
                    -2 => edi /= 2,
                    -1 => edi = ftol(edi as f64 * read_f64(rdata::F64_0_6666666667)),
                    _ => {}
                }
            }
        } else {
            let nation = (club.division().addr() as i32) as usize;
            if with_nation(nation) {
                let t2 = ftol(read_i16(nation + competition::REPUTATION) as i32 as f64 * read_f64(rdata::F64_1_35) + read_f64(rdata::F64_20)) as u8;
                let foul = (squad as u8).wrapping_sub(t2) as i8;
                if foul < 0 {
                    let v = if foul > -10 { foul as i32 } else { -10 };
                    edi = ftol((v as f64 * read_f64(rdata::F64_0_05) + read_f64(rdata::F64_1)) * edi as f64);
                }
            }
            let fin = exe::financetable_get_club_finance(read_i32(G_FINANCE_TABLE) as usize, club);
            if fin != 0 {
                let v = edi as f64;
                match exe::finance_get_wealth_band(fin) as i8 {
                    -2 => {
                        let (x, y) = (v * read_f64(rdata::F64_0_9090909091), edi.wrapping_sub(1000) as f64);
                        edi = ftol(if x < y { x } else { y });
                    }
                    -1 => {
                        let (x, y) = (v * read_f64(rdata::F64_0_9523809524), edi.wrapping_sub(500) as f64);
                        edi = ftol(if x < y { x } else { y });
                    }
                    1 => {
                        let (x, y) = (v * read_f64(rdata::F64_1_0125), edi.wrapping_add(125) as f64);
                        edi = ftol(if x > y { x } else { y });
                    }
                    2 => {
                        covered!("cpu_13 rich club");
                        let (x, y) = (v * read_f64(rdata::F64_1_025), edi.wrapping_add(250) as f64); // cpu13: o rico;
                        edi = ftol(if x > y { x } else { y });
                    }
                    _ => {}
                }
            }
        }
    }
    if edi < 500 {
        edi = 500;
    }
    if !base.is_null() {
        *base = esi;
    }
    edi
}

/// match entry i (0x29 bytes) of the tracked player's ring of 10, or 0 past the count
/// Original: `player_search_slot_get_match_entry` (0x7bd830, player_search.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn player_search_slot_get_match_entry(this: usize, i: u32) -> usize {
    let i = i as u8 as i8;
    if i >= read_i8(this + scouted_player_slot::HISTORY_COUNT) {
        return 0;
    }
    let mut k = read_i8(this + scouted_player_slot::HISTORY_START) as i32 + i as i32;
    if k >= 10 {
        k -= 10;
    }
    (this as isize + k as isize * 0x29 + 6) as usize
}

/// the sum of the goals of the matches stored in the ring
/// Original: `player_search_slot_sum_goals` (0x7bd870, player_search.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn player_search_slot_sum_goals(this: usize) -> u16 {
    let n = *((this + scouted_player_slot::HISTORY_COUNT) as *const i8);
    let mut k = *((this + scouted_player_slot::HISTORY_START) as *const u8);
    let mut s: u16 = 0;
    for _ in 0..n.max(0) {
        let g = *((this as isize + (k as i8) as isize * 0x29 + 0x16) as *const i8);
        s = s.wrapping_add(g as i16 as u16);
        k = k.wrapping_add(1);
        if k == 10 {
            k = 0;
        }
    }
    s
}

/// `PLAYER_SEARCH::get_scout`: returns the search's scout: the index at `player_search_fields::SCOUT_ID` (> −1) in the
/// people vector `G_STAFF_TABLE` (`staff::SIZE` bytes), if it still matches the stored names and date
/// (`exe::database_staff_matches_names` with `player_search_fields::SCOUT_NAME_REF`, `SCOUT_NAME_REF` + 4,
/// `SCOUT_NAME_REF` + 8 and the date at `SCOUT_NAME_REF` + 0x10); otherwise 0.
/// Original: `PLAYER_SEARCH::get_scout` (0x7ae950, player_search.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn get_scout(this: usize) -> usize {
    let i = read_u32(this + player_search_fields::SCOUT_ID) as i32;
    if i <= -1 {
        return 0;
    }
    let person = (read_u32(G_STAFF_TABLE) as usize).wrapping_add((i as usize).wrapping_mul(staff::SIZE));
    if exe::database_staff_matches_names(person, read_u32(this + player_search_fields::SCOUT_NAME_REF), read_u32(this + player_search_fields::SCOUT_NAME_REF + 4), read_u32(this + player_search_fields::SCOUT_NAME_REF + 8), this + player_search_fields::SCOUT_NAME_REF + 0x10) == 0 {
        return 0; // the slot changed person
    }
    person
}

/// Rival club's manager (+0xcf) passes the draws: random(+0x5c) == 0 and random(+0x57) > 5.
pub(crate) unsafe fn rival_manager_passes_draws(c: Club) -> bool {
    friendly_random(read_i8((c.manager().addr() as u32) as usize + staff::SPORTSMANSHIP) as i32) == 0
        && friendly_random(read_i8((c.manager().addr() as u32) as usize + staff::AMBITION) as i32) > 5
}

// 0x53e990 only reads: compare the value
compare_value!(compare_get_scout, get_scout, IDX_GET_SCOUT, () -> usize);

/// stores the player search slot (0 to 9)
/// Original: `PLAYER_SEARCH::set_slot_index` (0x7ae640, player_search.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn set_slot_index(search: usize, slot: i8) -> u32 {
    if !(0..=9).contains(&slot) {
        return original_fn::<unsafe extern "thiscall" fn(usize, i8) -> u32>(IDX_SET_SLOT_INDEX)(search, slot);
    }
    write_i8(search + 4, slot);
    1
}

/// stores the player search list type (1 to 4)
/// Original: `PLAYER_SEARCH::set_list_type` (0x7ae6e0, player_search.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn set_list_type(search: usize, kind: i8) -> u32 {
    if !(1..=4).contains(&kind) {
        return original_fn::<unsafe extern "thiscall" fn(usize, i8) -> u32>(IDX_SET_LIST_TYPE)(search, kind);
    }
    write_i8(search + player_search_fields::LIST_TYPE, kind);
    1
}

/// `PLAYER_SEARCH::set_scout(scout)`: sets a new scout. If the scout changes (the current one through `get_scout`), it
/// stores his ids at `player_search_fields::SCOUT_NAME_REF` (`exe::fill_staff_name_ref`) or, without a scout, the index
/// `player_search_fields::SCOUT_ID` = −1; and the nation (`player_search_fields::ASSIGNED_NATION`), region
/// (`player_search_fields::ASSIGNED_REGION`) and competition (`player_search_fields::ASSIGNED_COMP`) assignments go
/// back to −1. Returns 1.
/// Original: `PLAYER_SEARCH::set_scout` (0x7ae8a0, player_search.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn set_scout(this: usize, scout: usize) -> u32 {
    if scout == get_scout(this) {
        return 1; // o mesmo olheiro
    }
    if scout != 0 {
        exe::fill_staff_name_ref(scout, this + player_search_fields::SCOUT_NAME_REF);
    } else {
        write_u32(this + player_search_fields::SCOUT_ID, u32::MAX);
    }
    write_u32(this + player_search_fields::ASSIGNED_NATION, u32::MAX);
    write_u8(this + player_search_fields::ASSIGNED_REGION, 0xff);
    write_u32(this + player_search_fields::ASSIGNED_COMP, u32::MAX);
    1
}

/// stores the human manager id in the player search
/// Original: `PLAYER_SEARCH::set_manager` (0x7ae570, player_search.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn set_manager(search: usize, person: usize) -> u32 {
    if person == 0 || read_i32(person) < read_i32(G_STAFF_COUNT) - 0x10 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_SET_MANAGER)(search, person);
    }
    write_i32(search, read_i32(person));
    1
}

/// `PLAYER_SEARCH::get_scout_assignment(club, nation, region, comp, base, opponent)`: with the scout checked, if any
/// assignment is set (club `player_search_fields::ASSIGNED_CLUB`, nation `player_search_fields::ASSIGNED_NATION`,
/// region `player_search_fields::ASSIGNED_REGION` byte, competition `player_search_fields::ASSIGNED_COMP` above −1;
/// base `player_search_fields::ASSIGNED_BASE`, opponent `player_search_fields::ASSIGNED_OPPONENT` non-null), copies the
/// six to the outputs and returns 1; none: 0. A scout that does not check out: the original routine ("Error" box and
/// 0).
/// Original: `PLAYER_SEARCH::get_scout_assignment` (0x7aefd0, player_search.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn get_scout_assignment(this: usize, club: *mut u32, nation: *mut u32, region: *mut u8, comp: *mut u32, base: *mut u32, opponent: *mut u32) -> u32 {
    if check_search_scout(this).is_none() {
        return original_fn::<unsafe extern "thiscall" fn(usize, *mut u32, *mut u32, *mut u8, *mut u32, *mut u32, *mut u32) -> u32>(IDX_GET_SCOUT_ASSIGNMENT)(this, club, nation, region, comp, base, opponent); // message box: original only
    }
    let marked = read_usize(this + player_search_fields::ASSIGNED_CLUB) as i32 > -1
        || read_usize(this + player_search_fields::ASSIGNED_NATION) as i32 > -1
        || read_i8(this + player_search_fields::ASSIGNED_REGION) > -1
        || read_usize(this + player_search_fields::ASSIGNED_COMP) as i32 > -1 // the competition
        || read_usize(this + player_search_fields::ASSIGNED_BASE) != 0
        || read_usize(this + player_search_fields::ASSIGNED_OPPONENT) != 0;
    if !marked {
        return 0;
    }
    club.write_unaligned(read_usize(this + player_search_fields::ASSIGNED_CLUB) as u32);
    nation.write_unaligned(read_usize(this + player_search_fields::ASSIGNED_NATION) as u32);
    *region = read_u8(this + player_search_fields::ASSIGNED_REGION);
    comp.write_unaligned(read_usize(this + player_search_fields::ASSIGNED_COMP) as u32);
    base.write_unaligned(read_usize(this + player_search_fields::ASSIGNED_BASE) as u32);
    opponent.write_unaligned(read_usize(this + player_search_fields::ASSIGNED_OPPONENT) as u32);
    1
}

/// `PLAYER_SEARCH::scout_covers_nation(nation)`: tells whether the scout search (type `player_search_fields::LIST_TYPE`
/// = 4) with the scout checked (index `player_search_fields::SCOUT_ID` > −1 and `exe::database_staff_matches_names`
/// with the names `player_search_fields::SCOUT_NAME_REF`..`SCOUT_NAME_REF` + 8 and the date `SCOUT_NAME_REF` + 0x10)
/// and with a club (`staff::CLUB_JOB`) covers `nation`. With an assignment set
/// (`exe::player_search_get_scout_assignment`, which copies the assignments
/// `player_search_fields::ASSIGNED_CLUB`..`player_search_fields::ASSIGNED_OPPONENT` onto themselves): the nation of the
/// assignment's club (`club::NATION` of club `player_search_fields::ASSIGNED_CLUB`), the assignment's nation
/// (`player_search_fields::ASSIGNED_NATION` = id +0 of the nation), the region
/// (`player_search_fields::ASSIGNED_REGION`, signed byte, = `actual_region()` of the nation), or with the base
/// (`player_search_fields::ASSIGNED_BASE`) the same region. Without an assignment: both nations with
/// `group_membership()` = `nation_group::EEC`, the same region (`actual_region()`) as the club's nation, the nation not
/// foreign to the scout (`database_is_staff_foreign` with 1) or the club's scouts covering the nation
/// (`exe::scout_manager_can_club_scout_nation` of `G_SCOUT_MANAGER`). Returns 1 or 0.
/// Original: `PLAYER_SEARCH::scout_covers_nation` (0x7b7490, player_search.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn scout_covers_nation(this: usize, nation: usize) -> u32 {
    let nation = Nation(nation);
    let mut person = 0usize;
    let i = read_usize(this + player_search_fields::SCOUT_ID) as i32;
    if i > -1 {
        let p = read_usize(G_STAFF_TABLE).wrapping_add((i as usize).wrapping_mul(staff::SIZE));
        if exe::database_staff_matches_names(p, read_usize(this + player_search_fields::SCOUT_NAME_REF) as u32, read_usize(this + player_search_fields::SCOUT_NAME_REF + 4) as u32, read_usize(this + player_search_fields::SCOUT_NAME_REF + 8) as u32, this + player_search_fields::SCOUT_NAME_REF + 0x10) != 0 {
            person = read_usize(G_STAFF_TABLE).wrapping_add(read_usize(this + player_search_fields::SCOUT_ID).wrapping_mul(staff::SIZE));
        }
    }
    if read_u8(this + player_search_fields::LIST_TYPE) != 4 || person == 0 || nation.is_null() {
        return 0;
    }
    let club = Club(read_usize(person + staff::CLUB_JOB));
    if club.is_null() {
        return 0;
    }
    if exe::player_search_get_scout_assignment(this, this + player_search_fields::ASSIGNED_CLUB, this + player_search_fields::ASSIGNED_NATION, this + player_search_fields::ASSIGNED_REGION, this + player_search_fields::ASSIGNED_COMP, this + player_search_fields::ASSIGNED_BASE, this + player_search_fields::ASSIGNED_OPPONENT) != 0 {
        let c = read_usize(this + player_search_fields::ASSIGNED_CLUB) as i32;
        if c > -1 && read_usize(read_usize(G_CLUB_TABLE).wrapping_add((c as usize).wrapping_mul(club::SIZE)) + club::NATION) == nation.addr() {
            return 1;
        }
        let n = read_usize(this + player_search_fields::ASSIGNED_NATION) as i32;
        if n > -1 && read_usize(nation.addr()) as i32 == n {
            return 1;
        }
        let r = read_u8(this + player_search_fields::ASSIGNED_REGION);
        if (r as i8) > -1 && nation.actual_region() == r {
            return 1;
        }
        // the original compares the same byte again when there is a base
        if read_usize(this + player_search_fields::ASSIGNED_BASE) != 0 && nation.actual_region() == r {
            return 1;
        }
        return 0;
    }
    let nc = club.nation();
    if nc.group_membership() == nation_group::EEC && nation.group_membership() == nation_group::EEC {
        return 1;
    }
    if nc.actual_region() == nation.actual_region() {
        covered!("cpn_7 same region");
        return 1; // the same region as the club’s nation
    }
    if exe::database_is_staff_foreign(person, nation, 1) == 0 {
        covered!("cpn_7 scout nation");
        return 1; // the nation is not foreign for the scout
    }
    (exe::scout_manager_can_club_scout_nation(read_usize(G_SCOUT_MANAGER), nation, club) != 0) as u32
}

