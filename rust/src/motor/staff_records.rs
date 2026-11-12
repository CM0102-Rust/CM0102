use super::*;

/// The dry run of the comparers.
///
/// The copies run without the histories (file), without the news (`0x75c390`), without ending the call-ups (`0x774f30`)
/// and without the renewal (`0x4cda70`), noting the contracts the renewal would touch.
pub(crate) static STAFF_HISTORY_DRY_RUN: AtomicBool = AtomicBool::new(false);
pub(crate) static mut DRY_RUN_RENEWED_CONTRACTS: Vec<usize> = Vec::new();
/// After the copy's last day count (`exe::date_days_since`), the rehearsal skipped a call of the original routine that also counts
/// days and leaves its static scratch date (0xae2470) in another year: the renewal (`contract_manager_offer_contract`, by age), the end of the
/// call-ups (`end_duty_for_competition`) or the praise news item (`queue_trophy_praise_story`). That date is then left out of the check.
pub(crate) static DRY_RUN_STRAY_SCRATCH_DATE: AtomicBool = AtomicBool::new(false);

/// Raises club reputation (club::REPUTATION) toward weight × 500, at most 750 at once, clamped to 1..10000.
pub(crate) unsafe fn raise_champion_reputation(club: Club, weight: i16, champion_of_league: bool) {
    let target = (weight as i32).wrapping_mul(500);
    let rep = club.reputation() as i32;
    if rep < target {
        let foul = target - rep;
        let sum = if foul > 0x2ee { 0x2ee } else { foul }; // cqd13: the rise cap
        let v = club.reputation().wrapping_add(sum as i16);
        let v = if v < 1 { 1 } else if v > 10000 { 10000 } else { v };
        if champion_of_league {
            covered!("cqd_13 league winner injury_word_0c");
        } else {
            covered!("cqd_13 cup winner injury_word_0c");
        }
        club.set_reputation(v); // cqd13: the reputation of the champion
    }
}

/// Adds d to the contract morale byte (+0x44), clamped to −100..100.
pub(crate) unsafe fn add_contract_morale(k: usize, d: i32) {
    let k = Contract(k);
    let c = k.morale();
    let x = c as i32 + d;
    k.set_morale(if x > 100 { 100 } else if x < -100 { -100 } else { c.wrapping_add(d as i8) });
}

/// Offers a contract renewal (`contract_manager_offer_contract`, factor 1.0) to an own-club player when the transfer manager exists, the
/// contract (+0x25) is over 300 days old and he is on the club shortlist; rehearsal only notes it.
pub(crate) unsafe fn offer_renewal_if_due(club: Club, p: Staff, k: Contract) {
    if read_usize(G_TRANSFER_MANAGER) == 0 {
        return;
    }
    DRY_RUN_STRAY_SCRATCH_DATE.store(false, Relaxed);
    if days_between(G_CURRENT_DATE, k.addr() + contract::DATE_STARTED) <= 300 {
        return;
    }
    // 0x849ce0: push 0; push 0; push p; push club; mov ecx, [[0xde1f54]+0x856] -> thiscall (ret 0x10)
    let v = ShortlistSlot(exe::shortlist_manager_find_club_shortlist_entry(read_usize(read_usize(G_TRANSFER_MANAGER) + transfer_manager_fields::SHORTLIST_MANAGER), club, p, 0, 0));
    if v.is_null() || (v.target_staff_id() as i32) != read_i32(p.addr()) {
        return;
    }
    if STAFF_HISTORY_DRY_RUN.load(Relaxed) {
        (*core::ptr::addr_of_mut!(DRY_RUN_RENEWED_CONTRACTS)).push(k.addr());
        DRY_RUN_STRAY_SCRATCH_DATE.store(true, Relaxed);
        return;
    }
    // 0x4cda70: push 1; push 0x3ff00000; push 0; push −1; push 0; push 0; push p; mov ecx, [0xae1050] -> thiscall
    // (p, 0, 0, −1, 1.0 in two dwords, 1)
    exe::contract_manager_offer_contract(read_usize(G_CONTRACT_MANAGER), p, 0, 0, -1, 1.0, 1);
}

/// Adds history entry e for each of the 20 lineup players (+0x190, 0x49 bytes, −1 empty) except non-players and the
/// manager.
pub(crate) unsafe fn add_lineup_histories(this: usize, club: Club, lineup: usize, e: &[u8; 0x40]) {
    for j in 0..0x14usize {
        let id = read_i32(lineup + 0x49 * j);
        if id == -1 {
            continue;
        }
        let p = read_usize(G_STAFF_TABLE).wrapping_add((id as u32).wrapping_mul(staff::SIZE as u32) as usize);
        if p != 0 && read_i32(p) >= read_i32(G_STAFF_COUNT) {
            continue;
        }
        if p == club.manager().addr() {
            continue;
        }
        if p == 0 {
            show_error_at(0xa83b78, 0x1010);
            continue;
        }
        append_person_history_by_id(!STAFF_HISTORY_DRY_RUN.load(Relaxed), this, read_usize(p), e);
    }
}

/// Adds a dated club history entry (type at +8, competition id at +9) to the club history FILE_LLIST [this+4]; skipped
/// in rehearsal.
pub(crate) unsafe fn add_club_history(this: usize, club: Club, kind: u8, comp: Competition) {
    let mut e = history_entry();
    stamp_game_date(e.as_mut_ptr());
    e[8] = kind;
    put_i32(&mut e, 9, read_i32(comp.addr()));
    if !STAFF_HISTORY_DRY_RUN.load(Relaxed) {
        let p = read_usize(this + staff_records_fields::CLUB_HISTORY_LISTS).wrapping_add(read_usize(club.addr()).wrapping_mul(8));
        exe::file_llist_add_node(read_usize(this + staff_records_fields::CLUB_HISTORY_FILE), p, p + 4, e.as_ptr());
    }
}

/// Queues the trophy praise media story (`queue_trophy_praise_story` on G_MEDIA); in rehearsal only marks the static date as loose.
pub(crate) unsafe fn post_trophy_praise(club: Club, comp: Competition, mark: u32) {
    if STAFF_HISTORY_DRY_RUN.load(Relaxed) {
        DRY_RUN_STRAY_SCRATCH_DATE.store(true, Relaxed);
        return;
    }
    // 0x75c390: push mark; push comp; push club; mov ecx, [0xb6b7c8] -> thiscall (ret 0xc)
    exe::media_queue_trophy_praise_story(read_usize(G_MEDIA), club, comp, mark);
}

/// Manager rating increments (k1, k2) by competition stage: 400/400, or 200 (foreign manager) / 200 in a stage-1
/// competition (+0x42 = 1).
pub(crate) unsafe fn manager_rating_gains(t: Staff, club: Club, id_comp: usize) -> (i32, i32) {
    let c = League(read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(4 * id_comp)));
    if (c.club_sides() as i8) != 1 {
        (400, 400)
    } else {
        // 0x53d980: 3 pushes; add esp, 0xc -> cdecl
        (if exe::database_is_staff_foreign(t, club.nation().addr(), 1) != 0 { 200 } else { 400 }, 200)
    }
}

/// `Staff_Records::add_league_champions_record(club, comp)`: records the league champion. SEH frame only for the local
/// dates. Called by the league champions at the end of each league (25 callers, one per league; 4 calls up to
/// 2002-06-01).
///
/// Null club or competition: the error box (line 0x21c); the competition without an object in `G_COMP_OBJECT_TABLE` (+
/// 4 × id): the error box (0x224). The reputation (up to weight × 500, +750 at most); with `.division()` and the club's
/// `.stadium()` block: `.max_attendance()` goes up weight × 20 (below 150000 and below `.attendance()` + 500) and the
/// block's `.expansion_capacity()` by 25 × (weight + 20) (below 120000). The players' threshold: ftol((`.club_count()`
/// − 1) × the object's `.legs()` × 0.33), in 16 bits.
///
/// With a manager (`.manager()`): his three ratings (`non_player::CURRENT_REPUTATION` weight × 400,
/// `non_player::HOME_REPUTATION` weight × k1, `non_player::WORLD_REPUTATION` weight × k2 in the `.non_player()` record,
/// if any) and the history (type 5: the club and the competition); his record (`exe::staff_records_get_manager_record`
/// with create): +1 in `.leagues_won()` and weight × 5000 in `.year_points()`, `.month_points()` and
/// `.season_points()`. The chairman (`.chairman()`).
///
/// The 50 in the squad (`.squad()`) with a player sheet, valid, of the club (`.club_job()`) and other than the manager:
/// the average 2/1 (`exe::playerstats_get_stat` with 6.5) above the threshold gives the history, morale +20
/// (`exe::staff_adjust_morale`) and +100 on the contract (`exe::contract_manager_get_contract_for_club`) with the
/// renewal; otherwise morale +10 and +50 on the contract. The club's history (type 5); with the media (`G_MEDIA`) and
/// weight above 3, the praise (`queue_trophy_praise_story` with 1) and, above 12, with 3.
/// Original: `Staff_Records::add_league_champions_record` (0x868c50, staff_records.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn add_league_champions_record(this: usize, club: usize, comp: usize) {
    let comp = Competition(comp);
    let club = Club(club);
    if club.is_null() || comp.is_null() {
        return show_error_at(0xa83b78, 0x21c);
    }
    let obj = League(read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(4 * read_usize(comp.addr()))));
    if obj.is_null() {
        return show_error_at(0xa83b78, 0x224);
    }
    let weight = comp.reputation();
    raise_champion_reputation(club, weight, true); // cqd13: the reputation of the league champion
    if club.division().addr() != 0 {
        let b = club.stadium();
        if !b.is_null() {
            let x = club.max_attendance();
            if x < 150000 && x < club.attendance().wrapping_add(500) {
                club.set_max_attendance(x.wrapping_add((weight as i32).wrapping_mul(20)));
            }
            let y = b.expansion_capacity();
            if y < 120000 {
                b.set_expansion_capacity(y.wrapping_add(((weight as i32).wrapping_add(20)).wrapping_mul(25)));
            }
        }
    }
    let n = ((obj.club_count() as i32) - 1).wrapping_mul(obj.legs() as i32);
    let threshold = ftol(n as f64 * read_f64(rdata::F64_0_33)) as i16; // the ftol stays in the word (movsx word afterwards);
    let mut e = history_entry();
    stamp_game_date(e.as_mut_ptr());
    e[0xc] = 5;
    put_i32(&mut e, 8, read_i32(club.addr()));
    put_i32(&mut e, 0xd, read_i32(comp.addr()));
    let t = club.manager();
    if !t.is_null() {
        let np = t.non_player();
        if !np.is_null() {
            let (k1, k2) = manager_rating_gains(t, club, read_usize(comp.addr()));
            raise_side_rating(np.addr() + non_player::CURRENT_REPUTATION, weight, 400);
            raise_side_rating(np.addr() + non_player::HOME_REPUTATION, weight, k1);
            raise_side_rating(np.addr() + non_player::WORLD_REPUTATION, weight, k2);
        }
        append_person_history_by_id(!STAFF_HISTORY_DRY_RUN.load(Relaxed), this, read_usize(t.addr()), &e);
        // 0x86f5b0: push 1; push manager; mov ecx, edi (the this) -> thiscall (ret 8)
        let r = ManagerRecord(exe::staff_records_get_manager_record(this, club.manager().addr(), 1));
        if !r.is_null() {
            r.set_leagues_won(r.leagues_won().wrapping_add(1));
            let v = (weight as i32).wrapping_mul(5000);
            r.set_year_points(r.year_points().wrapping_add(v));
            r.set_month_points(r.month_points().wrapping_add(v));
            r.set_season_points(r.season_points().wrapping_add(v));
        }
    }
    let chairman = club.chairman().addr();
    if chairman != 0 {
        append_person_history_by_id(!STAFF_HISTORY_DRY_RUN.load(Relaxed), this, read_usize(chairman), &e);
    }
    // 0x7c1e30: push 6,5f; push 1; push 2; push player sheet; mov ecx, [0xb74350] -> thiscall (ret 0x10, x87); 0x544710: 3
    // pushes; add esp, 0xc -> cdecl; 0x4cf5f0: push club; push p; mov ecx, [0xae1050] -> thiscall (ret 8)
    for j in 0..50usize {
        let p = Staff(club.squad(j));
        if p.is_null() || p.player().is_null() || read_i32(p.addr()) >= read_i32(G_STAFF_COUNT) || p.club_job() != club
            || p == club.manager()
        {
            continue;
        }
        let f = exe::playerstats_get_stat(read_usize(G_PLAYER_STATS), p.player(), 2, 1, 6.5);
        if f > threshold as f64 { // cqd13: acima do limiar
            covered!("cqd_13 champion standout player");
            append_person_history_by_id(!STAFF_HISTORY_DRY_RUN.load(Relaxed), this, read_usize(p.addr()), &e);
            exe::staff_adjust_morale(p, 0x14, 1); // cqd13: the morale of the standout player
            let k = exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), p, club);
            if k == 0 {
                continue;
            }
            covered!("cqd_13 standout player contract");
            add_contract_morale(k, 100); // cqd13: the contract of the standout player
            offer_renewal_if_due(club, p, Contract(k));
        } else {
            exe::staff_adjust_morale(p, 0xa, 1);
            let k = exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), p, club);
            if k != 0 {
                add_contract_morale(k, 50);
            }
        }
    }
    add_club_history(this, club, 5, comp);
    if read_usize(G_MEDIA) != 0 && weight > 3 {
        post_trophy_praise(club, comp, 1);
        if weight > 12 {
            post_trophy_praise(club, comp, 3);
        }
    }
}

/// `Staff_Records::add_cup_winners_record(club, game, fixture)` (the club, the fixture and the match record): records
/// the cup winner. SEH frame only for the local dates. Called by the cup finals (29 callers; 11 calls up to
/// 2002-06-01).
///
/// Nulls: the error box (line 0x51e). The reputation (as in the league); with a manager (`.manager()`) who has a
/// non-player record (`.non_player()`): the three ratings of the club's side in the match
/// (`match_record::HOME_MANAGER_CURRENT_REPUTATION` and the next two words, side `match_record::SIDE_SIZE` × away;
/// weight × 400, × k1, × k2); the manager's history (type 0: the club and the competition of the fixture's
/// `.competition()`); his record (`exe::staff_records_get_manager_record` with create): +1 in `.cups_won()` and
/// ftol(weight^4 × 0.5) (CRT pow 0x945ff0, `game_pow`) in `.year_points()`, `.month_points()` and, for a national team
/// (`is_national_team`), `.national_season_points()`, otherwise `.season_points()`. The chairman (`.chairman()`) and
/// the 20 of the line-up (`match_record::ROW_STAFF_ID`).
///
/// The 50 in the squad while the reputation does not exceed weight × 400: with a player sheet, valid and other than the
/// manager, morale +15 (`exe::staff_adjust_morale`) and +100 on the contract; with contract status (`.squad_status()`
/// >> 4) 1-3 or 5, the renewal. The call-ups of the stored competition (`exe::national_teams_is_finals_comp`) end
/// (`exe::nationalteams_end_duty_for_competition`); the club's history (type 0); with the media and weight above 10,
/// the praise (`queue_trophy_praise_story` with 1) and, with weight above 12 or reputation below 2500, with 3.
/// Original: `Staff_Records::add_cup_winners_record` (0x86a800, staff_records.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn add_cup_winners_record(this: usize, club: usize, game: usize, fixture: usize) {
    let fixture = MatchRecord(fixture);
    let game = Fixture(game);
    let club = Club(club);
    if club.is_null() || game.is_null() || fixture.is_null() {
        return show_error_at(0xa83b78, 0x51e);
    }
    let comp = game.competition();
    let weight = comp.reputation();
    raise_champion_reputation(club, weight, false);
    let mut e = history_entry();
    stamp_game_date(e.as_mut_ptr());
    e[0xc] = 0;
    put_i32(&mut e, 8, read_i32(club.addr()));
    put_i32(&mut e, 0xd, read_i32(comp.addr()));
    let t = club.manager();
    if !t.is_null() {
        if !t.non_player().is_null() {
            let (k1, k2) = manager_rating_gains(t, club, read_usize(game.addr()));
            let side = fixture.addr() + match_record::HOME_MANAGER_CURRENT_REPUTATION + match_record::SIDE_SIZE * (game.away_club() == club) as usize;
            raise_side_rating(side, weight, 400);
            raise_side_rating(side + 2, weight, k1);
            raise_side_rating(side + 4, weight, k2);
        }
        append_person_history_by_id(!STAFF_HISTORY_DRY_RUN.load(Relaxed), this, read_usize(club.manager().addr()), &e);
        let r = ManagerRecord(exe::staff_records_get_manager_record(this, club.manager().addr(), 1));
        if !r.is_null() {
            r.set_cups_won(r.cups_won().wrapping_add(1));
            // fild weight; fld 4.0; _CIpow (base in st1); fmul 0.5; ftol
            let v = ftol(game_pow(comp.reputation() as f64, read_f64(rdata::F64_4)) * read_f64(rdata::F64_0_5)); // cqd13: the title weight;
            r.set_year_points(r.year_points().wrapping_add(v));
            r.set_month_points(r.month_points().wrapping_add(v));
            if is_national_team(club.addr()) != 0 {
                r.set_national_season_points(r.national_season_points().wrapping_add(v));
            } else {
                r.set_season_points(r.season_points().wrapping_add(v));
            }
        }
    }
    let chairman = club.chairman().addr();
    if chairman != 0 {
        append_person_history_by_id(!STAFF_HISTORY_DRY_RUN.load(Relaxed), this, read_usize(chairman), &e);
    }
    let lineup = fixture.addr() + match_record::ROW_STAFF_ID + match_record::SIDE_SIZE * (club != game.home_club()) as usize;
    add_lineup_histories(this, club, lineup, &e);
    for j in 0..50usize {
        if (comp.reputation() as i32).wrapping_mul(400) < club.reputation() as i32 {
            break;
        }
        let p = Staff(club.squad(j));
        if p.is_null() || p.player().is_null() || read_i32(p.addr()) >= read_i32(G_STAFF_COUNT) || p == club.manager() {
            continue;
        }
        covered!("cqd_13 cup winner squad");
        exe::staff_adjust_morale(p, 0xf, 1); // cqd13: the morale of the cup winner player
        let k = Contract(exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), p, club));
        if k.is_null() {
            continue;
        }
        covered!("cqd_13 cup winner squad morale");
        add_contract_morale(k.addr(), 100); // cqd13: the contract morale of the cup winner
        let s = k.squad_status() >> 4;
        if s == 0 || (s > 3 && s != 5) {
            continue;
        }
        offer_renewal_if_due(club, p, k);
    }
    // 0x76d940 and 0x774f30: push comp; mov ecx, [0xb74318] (0x76d940 does not read ecx and only compares the id; ret 4 in both)
    if exe::national_teams_is_finals_comp(read_usize(G_NATIONAL_TEAMS), game.competition()) != 0 {
        if STAFF_HISTORY_DRY_RUN.load(Relaxed) {
            DRY_RUN_STRAY_SCRATCH_DATE.store(true, Relaxed);
        } else {
            exe::nationalteams_end_duty_for_competition(read_usize(G_NATIONAL_TEAMS), game.competition());
        }
    }
    add_club_history(this, club, 0, comp);
    if read_usize(G_MEDIA) != 0 && weight > 10 {
        post_trophy_praise(club, comp, 1);
        if comp.reputation() > 12 || club.reputation() < 0x9c4 {
            post_trophy_praise(club, comp, 3);
        }
    }
}

/// The dry-run comparer of the two champions (league and cup).
///
/// The manager's record is created beforehand (`0x86f5b0` with create, as the original routine does), the copy runs as
/// a dry run (without histories, news, call-ups and renewal; the morale random draws `0x544710` not counted) and the
/// original routine from the same state.
///
/// It compares: the club's reputation, the manager's record (0xd2 bytes), the ratings (the side's three in the match,
/// or `+8`..`+0xd` of the manager's non-player record), the club's `+0x7b` and `+0x44` of block `+0x69` (in the
/// league), the morale (`+0x45` of the player sheet) and the contract's morale (`+0x44`) of the 50 in the squad, and
/// the static date of `0x54cdb0`.
///
/// The morale of the contracts the renewal would touch (the original renews) and the random-draw state (the original's
/// news makes random draws) are not counted; the random draw state is put back to the previous one for the original.
/// The original's renewal also makes random draws and counts days (`0x54cdb0`): the morale of the player sheets after
/// the first renewed one is left out of the equality and checked against the range of `0x544710`; the static date is
/// left out when the dry run skipped, after the copy's last day count, an original call that counts days
/// (DRY_RUN_STRAY_SCRATCH_DATE). Seen in ENG in May 2002 and in the 8-league audit.
pub(crate) unsafe fn compare_champion_update(name: &str, this: usize, club: Club, notes: (usize, usize), league: bool, original: impl FnOnce(), copy: impl FnOnce()) {
    let mut fixas: Vec<(usize, usize)> = vec![(club.addr() + club::REPUTATION, 2), (G_DAYS_SINCE_SCRATCH_DATE, 8), notes];
    let t = club.manager().addr();
    if t != 0 {
        let record: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_STAFF_RECORDS_GET_MANAGER_RECORD);
        let r = record(this, t, 1);
        if r != 0 {
            fixas.push((r, 0xd2));
        }
    }
    if league && club.division().addr() != 0 && club.stadium().addr() != 0 {
        fixas.push((club.addr() + club::MAX_ATTENDANCE, 4));
        fixas.push((club.stadium().addr() + stadium::EXPANSION_CAPACITY, 4));
    }
    let mut contracts = Vec::new();
    let mut sheets = Vec::new(); // (morale from the record, contract or 0, previous morale), in squad order
    for j in 0..50usize {
        let p = Staff(club.squad(j));
        if p.is_null() || p.player().is_null() || read_i32(p.addr()) >= read_i32(G_STAFF_COUNT) {
            continue;
        }
        let k = exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), p, club);
        let m = p.player().addr() + player_sheet::PLAYER_MORALE;
        sheets.push((m, k, read_i8(m)));
        if k != 0 {
            contracts.push(k);
        }
    }
    let mut todas = fixas.clone();
    todas.extend(sheets.iter().map(|&(m, _, _)| (m, 1)));
    todas.extend(contracts.iter().map(|&k| (k + contract::MORALE, 1)));
    todas.extend(crate::rng::STATE_REGIONS.iter().copied());
    let before = crate::comparison::MemorySnapshot::read(&todas);
    (*core::ptr::addr_of_mut!(DRY_RUN_RENEWED_CONTRACTS)).clear();
    DRY_RUN_STRAY_SCRATCH_DATE.store(false, Relaxed);
    STAFF_HISTORY_DRY_RUN.store(true, Relaxed);
    crate::rng::without_counting(copy);
    STAFF_HISTORY_DRY_RUN.store(false, Relaxed);
    let renovados = core::mem::take(&mut *core::ptr::addr_of_mut!(DRY_RUN_RENEWED_CONTRACTS));
    let mut count = fixas;
    if DRY_RUN_STRAY_SCRATCH_DATE.load(Relaxed) {
        count.retain(|&(a, _)| a != G_DAYS_SINCE_SCRATCH_DATE);
    }
    // The original’s renewal rolls (0x4cda70): the players’ morale after the first renewed one (0x544710 rolls)
    // comes from another point of the random generator; his and the previous ones enter the count.
    let cut = renovados.first().and_then(|r| sheets.iter().position(|&(_, k, _)| k == *r)).unwrap_or(sheets.len());
    count.extend(sheets.iter().take(cut + 1).map(|&(m, _, _)| (m, 1)));
    count.extend(contracts.iter().filter(|k| !renovados.contains(k)).map(|&k| (k + contract::MORALE, 1)));
    // The later ones, by the range of 0x544710: morale m rises from 0 to 20 (+20 or +10 drawn), clamped to 1..20.
    let band = |&(m, _, m0): &(usize, usize, i8)| -> bool {
        let (m0, v) = (m0 as i32, read_i8(m) as i32);
        v >= m0.clamp(1, 20) && v <= (m0 + 20).clamp(1, 20)
    };
    let mut in_band = sheets.iter().skip(cut + 1).all(band);
    let mr = crate::comparison::MemorySnapshot::read(&count);
    before.write_back();
    original();
    in_band &= sheets.iter().skip(cut + 1).all(band);
    let mo = crate::comparison::MemorySnapshot::read(&count);
    SCOREBOARD.record(mo == mr && in_band, || format!("{}(club {:#x}) renovados {} (o first in the sheet {} de {}; away of the band: {}) original={:x?} rust={:x?}",
        name, club.addr(), renovados.len(), cut, sheets.len(), !in_band, mo, mr));
}

/// `Staff_Records::clear_human_manager_record(person)`: clears a human manager's record. Called by the human manager's
/// retirement (0x6b2b40) and by setup (0x82b9xx).
///
/// A person among the 16 humans (id from `G_STAFF_COUNT` − 16 to `G_STAFF_COUNT`): his record in vector
/// `staff_records_fields::MANAGER_RECORDS` (slot id − `G_STAFF_COUNT` + `G_NON_PLAYER_COUNT` + 16, 0xd2 bytes);
/// otherwise, as in the original, the person itself. Zeroes the 0xd2 bytes and puts −1 in +0x86, +0xc2, +0xc6, +0x4a,
/// +0x5e and +0x72.
/// Original: `Staff_Records::clear_human_manager_record` (0x86fc70, staff_records.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn clear_human_manager_record(this: usize, person: usize) {
    let id = read_i32(person);
    let lim = read_i32(G_STAFF_COUNT);
    let r = if id >= lim.wrapping_sub(0x10) && id < lim {
        let slot = id.wrapping_sub(lim).wrapping_add(read_i32(G_NON_PLAYER_COUNT)).wrapping_add(0x10);
        read_usize(this + staff_records_fields::MANAGER_RECORDS).wrapping_add((slot as usize).wrapping_mul(0xd2))
    } else {
        person
    };
    core::ptr::write_bytes(r as *mut u8, 0, 0xd2);
    for o in [0x86usize, 0xc2, 0xc6, 0x4a, 0x5e, 0x72] {
        write_i32(r + o, -1);
    }
}

/// Adds 1 to the i16 at address a.
pub(crate) unsafe fn increment_i16(a: usize) {
    write_i16(a, read_i16(a).wrapping_add(1));
}

/// Returns the club manager's points record (FN_STAFF_RECORDS_GET_MANAGER_RECORD with create), or 0 without a manager.
pub(crate) unsafe fn club_manager_record(this: usize, club: Club) -> ManagerRecord {
    let t = club.manager().addr();
    if t == 0 {
        return ManagerRecord(0);
    }
    ManagerRecord(exe::staff_records_get_manager_record(this, t, 1))
}

/// Points of one side in match m from four signed bytes: (((a + b) × 5 + (c − b)) × 2 + d) × 5.
pub(crate) unsafe fn side_match_points(m: usize, a: usize, b: usize, c: usize, d: usize) -> i32 {
    let x = read_i8_as_i32(m + a).wrapping_add(read_i8_as_i32(m + b));
    let y = read_u8(m + c).wrapping_sub(read_u8(m + b)) as i8 as i32;
    let x = x.wrapping_mul(5).wrapping_add(y);
    read_i8_as_i32(m + d).wrapping_add(x.wrapping_mul(2)).wrapping_mul(5)
}

/// The match rating for the manager.
///
/// The importance (`get_fixture_match_importance` of the match's competition, with `lado` and the output in `saida`; only the byte, signed) ×
/// (opponent reputation / 500) × the competition weight (+0x69 of [+0x14]) × (points + bonus) × 50, divided by 100000
/// (signed divisions truncated, like the original's magic multiplications; products in 32 bits).
pub(crate) unsafe fn match_rating_for_manager(p: Fixture, side: u32, out: &mut u32, opponent: Club, points: i32) -> i32 {
    let comp = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(4 * read_usize(p.addr())));
    let i = (exe::comp_get_fixture_match_importance(comp, p, side, out as *mut u32) as u32) as u8 as i8 as i32;
    let x = i
        .wrapping_mul(opponent.reputation() as i32 / 500)
        .wrapping_mul(p.competition().reputation() as i32)
        .wrapping_mul(points)
        .wrapping_mul(50);
    x / 100000
}

/// `Staff_Records::record_match_result(p, m)` (the fixture and the match): records a match in both managers' records.
/// Called by the closing of the day's match (`close_day_match`), for each game.
///
/// A friendly (`.category()` = `match_category::FRIENDLY`): nothing. The records of the two managers (+0xcf of the home
/// side `.home_club()` and of the away side `.away_club()`; `get_manager_record` with create, through
/// `club_manager_record`): +1 game (`manager_record::GAMES`); goals for (`.goals_for()`) and against
/// (`.goals_against()`) from the score (`.home_goals_extra_time()`/`.away_goals_extra_time()`, or
/// `.home_goals()`/`.away_goals()` with `.home_goals_extra_time()` = −1); win (`manager_record::WINS`, bonus 900), draw
/// (`manager_record::DRAWS`, 300 for both) or loss (`manager_record::LOSSES`). Each one's rating
/// (`get_fixture_match_importance` with 0 / 1, the opponent's reputation, the match points at +0x11f.. / +0x744..;
/// importance × opponent reputation × weight × points) goes to `.year_points()` and `.month_points()` (month and year);
/// in the club's league (`.division()` or `.reserve_division()` = the competition `.competition()`) also to the month's
/// points (`manager_record::MONTH_LEAGUE_POINTS`); for a national team (`is_national_team`) to
/// `.national_season_points()`, otherwise `.season_points()` (season).
///
/// Defect of the original, reproduced: for the away manager, `.national_season_points()`/`.season_points()` receive the
/// routine's [esp+0x10], which is the home manager's rating (or his bonus, without a record).
/// Original: `Staff_Records::record_match_result` (0x868830, staff_records.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn record_match_result(this: usize, p: usize, m: usize) {
    let p = Fixture(p);
    if p.category() == match_category::FRIENDLY {
        return;
    }
    let (casa, fora) = (p.home_club(), p.away_club());
    let a = ManagerRecord((club_manager_record(this, casa)).addr());
    let v = ManagerRecord((club_manager_record(this, fora)).addr());
    if !a.is_null() {
        increment_i16(a.addr() + manager_record::GAMES);
    }
    if !v.is_null() {
        increment_i16(v.addr() + manager_record::GAMES);
    }
    let (ga, gv) = if (p.home_goals_extra_time() as u8) == 0xff {
        ((p.home_goals() as i32), (p.away_goals() as i32))
    } else {
        ((p.home_goals_extra_time() as i32), (p.away_goals_extra_time() as i32))
    };
    if !a.is_null() {
        a.set_goals_for(a.goals_for().wrapping_add(ga));
        a.set_goals_against(a.goals_against().wrapping_add(gv));
    }
    if !v.is_null() {
        v.set_goals_for(v.goals_for().wrapping_add(gv));
        v.set_goals_against(v.goals_against().wrapping_add(ga));
    }
    let (mut ba, mut bv) = (0i32, 0i32);
    if ga > gv {
        ba = 900; // cqa11: the win bonus of the home team
        if !a.is_null() {
            covered!("cqa_11 home win");
            increment_i16(a.addr() + manager_record::WINS);
        }
        if !v.is_null() {
            increment_i16(v.addr() + manager_record::LOSSES);
        }
    } else if ga < gv {
        bv = 900;
        if !v.is_null() {
            increment_i16(v.addr() + manager_record::WINS);
        }
        if !a.is_null() {
            increment_i16(a.addr() + manager_record::LOSSES);
        }
    } else {
        ba = 300;
        bv = 300;
        if !a.is_null() {
            increment_i16(a.addr() + manager_record::DRAWS);
        }
        if !v.is_null() {
            increment_i16(v.addr() + manager_record::DRAWS);
        }
    }
    // the output of 0x48e190 goes in the slot of the original’s `jogo` argument, which starts with the match itself
    let mut out = p.addr() as u32;
    let mut stored = ba; // the original’s [esp+0x10]: the home side’s bonus, then its rating
    let comp = p.competition();
    if !a.is_null() {
        covered!("cqa_11 home side rating");
        let q = match_rating_for_manager(p, 0, &mut out, fora, side_match_points(m, 0x11f, 0x128, 0x127, 0x129).wrapping_add(ba));
        a.set_year_points(a.year_points().wrapping_add(q));
        a.set_month_points(a.month_points().wrapping_add(q));
        stored = q;
        if comp == casa.division() || comp == casa.reserve_division() {
            covered!("cqa_11 points of the month");
            add_to_i32(a.addr() + manager_record::MONTH_LEAGUE_POINTS, q); // cqa11: the month points of the home team
        }
        if is_national_team(casa.addr()) != 0 {
            a.set_national_season_points(a.national_season_points().wrapping_add(q));
        } else {
            a.set_season_points(a.season_points().wrapping_add(q));
        }
    }
    if !v.is_null() {
        let q = match_rating_for_manager(p, 1, &mut out, casa, side_match_points(m, 0x744, 0x74d, 0x74c, 0x74e).wrapping_add(bv));
        v.set_year_points(v.year_points().wrapping_add(q));
        v.set_month_points(v.month_points().wrapping_add(q));
        if comp == fora.division() || comp == fora.reserve_division() {
            add_to_i32(v.addr() + manager_record::MONTH_LEAGUE_POINTS, q);
        }
        if is_national_team(fora.addr()) != 0 {
            v.set_national_season_points(v.national_season_points().wrapping_add(stored));
        } else {
            v.set_season_points(v.season_points().wrapping_add(stored));
        }
    }
}

/// `Staff_Records::add_league_finish_records(league)`: at the end of the league, each club's final position in its
/// manager's history. SEH frame only for the local date. Called by the completed league (`league_check_complete`).
///
/// For each row of the table (`exe::league_get_table_row`, up to `.club_count()`), the history of the club's manager
/// (`.manager()`; type 0xa: the club id, the competition of `.competition()`, the stage `.stage_id()` and the position
/// k + 1), in the archive (0x58e340).
/// Original: `Staff_Records::add_league_finish_records` (0x86a070, staff_records.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn add_league_finish_records(this: usize, league: usize) {
    let league = League(league);
    covered!("cqa_11 end of the league");
    let mut e = history_entry();
    stamp_game_date(e.as_mut_ptr());
    e[0xc] = 0xa;
    put_i32(&mut e, 0x11, league.stage_id() as i32);
    put_i32(&mut e, 0xd, read_i32(league.competition().addr()));
    let mut k: i16 = 0;
    while k < league.club_count() {
        let club = Club(read_usize(exe::league_get_table_row(league, k as i32)));
        put_i32(&mut e, 0x15, k as i32 + 1);
        put_i32(&mut e, 8, read_i32(club.addr()));
        let t = club.manager().addr();
        if t != 0 {
            append_person_history_by_id(true, this, read_usize(t), &e);
        }
        k += 1;
    }
}

// simplification: only writes the history to the file (does not run twice): original only; the copy takes the branch.

/// O ensaio do comparador da `add_supporters_player_of_year_record`: a cópia sem o histórico (arquivo).
pub(crate) static SUPPORTERS_PLAYER_OF_YEAR_DRY_RUN: AtomicBool = AtomicBool::new(false);

/// `Staff_Records::add_supporters_player_of_year_record(person, club, a, b)`: the supporters' player of the year. SEH
/// frame only for the local date. Called by the end of season of the records (0x447880).
///
/// The person's history (type 0x35: the club id, `a` as unsigned 16 bits and `b`), in the archive (0x58e340); then, if
/// draw(21 − the person's `.loyalty()`) < draw(10), the person likes the club (`exe::staff_add_favoured_club` with
/// draw(2)) and the club likes him (`exe::club_add_favoured_staff` with draw(2)). Null person: the original (the error
/// box and the read of the null person's `.loyalty()`).
/// Original: `Staff_Records::add_supporters_player_of_year_record` (0x86dc30, staff_records.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn add_supporters_player_of_year_record(this: usize, person: usize, club: usize, a: u32, b: u32) {
    let person = Staff(person);
    if person.is_null() {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(IDX_ADD_SUPPORTERS_PLAYER_OF_YEAR_RECORD)(this, person.addr(), club, a, b);
    }
    let mut e = history_entry();
    stamp_game_date(e.as_mut_ptr());
    e[0xc] = 0x35;
    put_i32(&mut e, 8, read_i32(club));
    put_i32(&mut e, 0xd, (a & 0xffff) as i32);
    put_i32(&mut e, 0x11, b as i32);
    append_person_history_by_id(!SUPPORTERS_PLAYER_OF_YEAR_DRY_RUN.load(Relaxed), this, read_usize(person.addr()), &e);
    let r1 = friendly_random(0x15 - (person.loyalty() as i32));
    if r1 < friendly_random(10) {
        covered!("cqa_11 favourite goal_crowd_reaction");
        exe::staff_add_favoured_club(person, club, friendly_random(2) as u32);
        exe::club_add_favoured_staff(club, person, friendly_random(2) as u32);
    }
}

/// The comparers' rehearsal: the copies run without the history (file) and without ending the call-ups.
pub(crate) static CUP_PLACED_CLUB_DRY_RUN: AtomicBool = AtomicBool::new(false);

/// Appends history entry e to club id's list in the club history FILE_LLIST [this+4]; nothing in rehearsal (takes the
/// club id; skipped in rehearsal).
pub(crate) unsafe fn append_club_history_by_id(this: usize, id: usize, e: &[u8; 0x40]) {
    if CUP_PLACED_CLUB_DRY_RUN.load(Relaxed) {
        return;
    }
    let p = read_usize(this + staff_records_fields::CLUB_HISTORY_LISTS).wrapping_add(id.wrapping_mul(8));
    exe::file_llist_add_node(read_usize(this + staff_records_fields::CLUB_HISTORY_FILE), p, p + 4, e.as_ptr());
}

/// Raises a side rating word by ftol(weight × k × [0x96ff88]), floored at weight × k and capped at 10000.
pub(crate) unsafe fn raise_side_rating(a: usize, weight: i16, k: i32) {
    let sum = ftol((weight as i32).wrapping_mul(k) as f64 * read_f64(rdata::F64_0_1639344262));
    let x = read_i16(a).wrapping_add(sum as i16);
    write_i16(a, x);
    let floor = weight.wrapping_mul(k as i16);
    write_i16(a, if floor > x { floor } else if x > 10000 { 10000 } else { x });
}

/// The cup runner-up (type 1: threshold 500, cap 350, ratings 300/150) and third place (type 2: 500, 200, 200/100; the
/// 500 threshold verified at 0x86b774).
///
/// The club's reputation (+0x80) rises up to the competition's weight (+0x69 of [match+0x14]) × threshold, at most
/// `teto` at a time (between 1 and 10000); with a manager (+0xcf) who has a non-player record (+0x69), the three
/// ratings of the club's side in the match (+0x188/+0x18a/+0x18c, side 0x625 × away): weight × k, and in a stage-1
/// competition ([0xadadfc] +0x42 = 1) weight × half in the third and in the second if the manager is foreign (`database_is_staff_foreign`
/// with nation +0x53 and 1).
///
/// The histories (date, type, club id, competition): of the manager, the chairman (+0xbf) and the 20 of the side's
/// line-up (+0x190, 0x49 bytes, −1 empty; valid persons and excluding the manager); for third place, the call-ups end
/// in the stored competitions (0x76d940, `end_duty_for_nation_in_competition` with both clubs); finally the club history (type at +8, competition
/// at +9).
pub(crate) unsafe fn update_cup_placed_club(this: usize, club: Club, game: Fixture, fixture: MatchRecord, kind: u8) {
    let (limiar, teto, k, metade) = if kind == 1 { (500, 350, 300, 150) } else { (500, 200, 200, 100) };
    let comp = game.competition();
    let weight = comp.reputation();
    let target = (weight as i32).wrapping_mul(limiar); // cqa12: the reputation threshold
    let rep = club.reputation() as i32;
    if rep < target {
        let foul = target - rep;
        let sum = if foul > teto { teto } else { foul }; // cqa12: the rise cap
        let v = club.reputation().wrapping_add(sum as i16);
        let v = if v < 1 { 1 } else if v > 10000 { 10000 } else { v };
        if kind == 1 {
            covered!("cqa_12 runner-up injury_word_0c");
        } else {
            covered!("cqa_12 third place injury_word_0c");
        }
        club.set_reputation(v); // cqa12: the reputation of the placed team
    }
    let mut e = history_entry();
    stamp_game_date(e.as_mut_ptr());
    e[0xc] = kind;
    put_i32(&mut e, 8, read_i32(club.addr()));
    put_i32(&mut e, 0xd, read_i32(comp.addr()));
    let t = club.manager();
    if !t.is_null() {
        if !t.non_player().is_null() {
            let c = League(read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(4 * read_usize(game.addr()))));
            let (k1, k2) = if (c.club_sides() as i8) != 1 {
                (k, k)
            } else {
                (if exe::database_is_staff_foreign(t, club.nation().addr(), 1) != 0 { metade } else { k }, metade)
            };
            let side = fixture.addr() + match_record::HOME_MANAGER_CURRENT_REPUTATION + match_record::SIDE_SIZE * (game.away_club() == club) as usize;
            raise_side_rating(side, weight, k);
            raise_side_rating(side + 2, weight, k1);
            raise_side_rating(side + 4, weight, k2);
        }
        append_person_history_by_id(!CUP_PLACED_CLUB_DRY_RUN.load(Relaxed), this, read_usize(t.addr()), &e);
    }
    let chairman = club.chairman().addr();
    if chairman != 0 {
        append_person_history_by_id(!CUP_PLACED_CLUB_DRY_RUN.load(Relaxed), this, read_usize(chairman), &e);
    }
    let lineup = fixture.addr() + match_record::ROW_STAFF_ID + match_record::SIDE_SIZE * (club != game.home_club()) as usize;
    for j in 0..0x14usize {
        let id = read_i32(lineup + 0x49 * j);
        if id == -1 {
            continue;
        }
        let p = read_usize(G_STAFF_TABLE).wrapping_add((id as u32).wrapping_mul(staff::SIZE as u32) as usize);
        if p != 0 && read_i32(p) >= read_i32(G_STAFF_COUNT) {
            continue;
        }
        if p == club.manager().addr() {
            continue;
        }
        if p == 0 {
            show_error_at(0xa83b78, 0x1010);
            continue;
        }
        append_person_history_by_id(!CUP_PLACED_CLUB_DRY_RUN.load(Relaxed), this, read_usize(p), &e);
    }
    if kind == 2 && !CUP_PLACED_CLUB_DRY_RUN.load(Relaxed) {
        let stored: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(FN_NATIONAL_TEAMS_IS_FINALS_COMP);
        if stored(game.competition().addr()) != 0 {
            exe::nationalteams_end_duty_for_nation_in_competition(read_usize(G_NATIONAL_TEAMS), game.home_club(), game.competition());
            exe::nationalteams_end_duty_for_nation_in_competition(read_usize(G_NATIONAL_TEAMS), game.away_club(), game.competition());
        }
    }
    let mut e2 = history_entry();
    stamp_game_date(e2.as_mut_ptr());
    e2[8] = kind;
    put_i32(&mut e2, 9, read_i32(comp.addr()));
    append_club_history_by_id(this, read_usize(club.addr()), &e2);
}

/// the cup runner-up: the club reputation rises up to weight × 500 (at most 350 at once), its side's three match
/// ratings by the weight, and the manager, chairman, line-up and club histories
/// Original: `Staff_Records::add_cup_runners_up_record` (0x86b0b0, staff_records.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn add_cup_runners_up_record(this: usize, club: usize, game: usize, fixture: usize) {
    if club == 0 || game == 0 || fixture == 0 {
        return show_error_at(0xa83b78, 0x5f6);
    }
    update_cup_placed_club(this, Club(club), Fixture(game), MatchRecord(fixture), 1);
}

/// the cup third place: the same with weight × 200 (at most 200) and smaller ratings; ends the call-ups in the stored
/// competitions
/// Original: `Staff_Records::add_cup_third_place_record` (0x86b710, staff_records.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn add_cup_third_place_record(this: usize, club: usize, game: usize, fixture: usize) {
    if club == 0 || game == 0 || fixture == 0 {
        return show_error_at(0xa83b78, 0x67b);
    }
    update_cup_placed_club(this, Club(club), Fixture(game), MatchRecord(fixture), 2);
}

/// Rehearsal comparer for cup runner-up/third: copy without histories vs original from the same state, comparing club
/// reputation and the side's three ratings.
pub(crate) unsafe fn compare_cup_placed(name: &str, club: Club, game: Fixture, fixture: MatchRecord, original: impl FnOnce(), copy: impl FnOnce()) {
    if club.is_null() || game.is_null() || fixture.is_null() {
        return original();
    }
    let reg = [(club.addr() + club::REPUTATION, 2), (fixture.addr() + match_record::HOME_MANAGER_CURRENT_REPUTATION + match_record::SIDE_SIZE * (game.away_club() == club) as usize, 6)];
    let before = crate::comparison::MemorySnapshot::read(&reg);
    CUP_PLACED_CLUB_DRY_RUN.store(true, Relaxed);
    copy();
    CUP_PLACED_CLUB_DRY_RUN.store(false, Relaxed);
    let mr = crate::comparison::MemorySnapshot::read(&reg);
    before.write_back();
    original();
    let mo = crate::comparison::MemorySnapshot::read(&reg);
    SCOREBOARD.record(mo == mr, || format!("{}(club {:#x}, game {:#x}) original={:x?} rust={:x?}", name, club.addr(), game.addr(), mo, mr));
}

/// Copies date src into dst (`exe::date_operator_3d`).
pub(crate) unsafe fn staff_records_copy_date(dst: usize, src: usize) {
    let mut t = [0u8; 8];
    exe::date_operator_3d(dst, (t.as_mut_ptr()) as usize, src);
}

/// Days of date a minus date b (`exe::date_days_since`, b by value; writes the static date 0xae2470).
pub(crate) unsafe fn days_between(a: usize, b: usize) -> i32 {
    exe::date_days_since(a, read_usize(b) as u32, read_usize(b + 4) as u32)
}

/// `Staff_Records::record_manager_job_change(manager, club, signed)`: closes the manager's current spell, opens the new
/// one when hired and counts the spells. SEH frame only for the local dates. Called by the manager's departure
/// (`remove_club_manager`) and by the new manager (`reset_board_for_new_manager`).
///
/// Outside a national team (`is_national_team`), in the manager's record (`exe::staff_records_get_manager_record` with
/// create), when the game start (`G_GAME_START_DATE`) is not after today (`G_CURRENT_DATE`): the current spell
/// (`manager_record::CURRENT_CLUB` club, `manager_record::CURRENT_CLUB_SINCE` start) closes; the shortest
/// (`manager_record::SHORTEST_CLUB`, `manager_record::SHORTEST_START` start, `manager_record::SHORTEST_END` end) is
/// replaced by the current one if there is none or if the current one is shorter (`exe::date_days_since`), or only has
/// its end renewed if it is the same; the longest (`manager_record::LONGEST_CLUB`, `manager_record::LONGEST_START`,
/// `manager_record::LONGEST_END`) is replaced if the current one is longer (the end becomes
/// `manager_record::CURRENT_CLUB_END`) or if there is none (the end becomes today). Then `manager_record::CURRENT_CLUB`
/// = −1 and, with signed = 1, the new spell (the club id, today, `manager_record::CURRENT_CLUB_END` = 13/11/1975 from
/// `exe::date_date_549f70`) and +1 in `manager_record::CLUB_JOBS`. For a national team, signed = 1: +1 in
/// `manager_record::NATIONAL_JOBS`.
///
/// The histories: the manager's (type 0x96: club, signed), the club chairman's (`.chairman()`; type 0x97 with the
/// manager and the three keys) and the club's (type 0x96 in +8, the manager, the keys and signed). Null manager or
/// club: the error box (line 0x3b1).
/// Original: `Staff_Records::record_manager_job_change` (0x869aa0, staff_records.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn record_manager_job_change(this: usize, manager: usize, club: usize, signed: u32) {
    let club = Club(club);
    if club.is_null() || manager == 0 {
        return show_error_at(0xa83b78, 0x3b1);
    }
    let signed = signed as u8 as i8;
    if is_national_team(club.addr()) == 0 {
        let r = exe::staff_records_get_manager_record(this, manager, 1);
        if r != 0 {
            let today = G_CURRENT_DATE;
            let (ai, ah) = (read_u16(G_GAME_START_DATE + 2), read_u16(today + 2));
            let after = ai > ah || (ai == ah && read_i16(G_GAME_START_DATE) > read_i16(today));
            if !after {
                covered!("cqa_12 spell closed");
                if read_i32(r + manager_record::SHORTEST_CLUB) != -1 && read_i32(r + manager_record::CURRENT_CLUB) != -1 {
                    if read_i32(r + manager_record::SHORTEST_CLUB) == read_i32(r + manager_record::CURRENT_CLUB) && read_i16(r + manager_record::CURRENT_CLUB_SINCE_YEAR) == read_i16(r + manager_record::SHORTEST_START_YEAR) && read_i16(r + manager_record::CURRENT_CLUB_SINCE) == read_i16(r + manager_record::SHORTEST_START) {
                        staff_records_copy_date(r + manager_record::SHORTEST_END, today);
                    } else {
                        let current = days_between(today, r + manager_record::CURRENT_CLUB_SINCE);
                        if current < days_between(r + manager_record::SHORTEST_END, r + manager_record::SHORTEST_START) {
                            write_i32(r + manager_record::SHORTEST_CLUB, read_i32(r + manager_record::CURRENT_CLUB));
                            staff_records_copy_date(r + manager_record::SHORTEST_START, r + manager_record::CURRENT_CLUB_SINCE);
                            staff_records_copy_date(r + manager_record::SHORTEST_END, today);
                        }
                    }
                } else {
                    write_i32(r + manager_record::SHORTEST_CLUB, read_i32(r + manager_record::CURRENT_CLUB));
                    staff_records_copy_date(r + manager_record::SHORTEST_START, r + manager_record::CURRENT_CLUB_SINCE);
                    staff_records_copy_date(r + manager_record::SHORTEST_END, today);
                }
                if read_i32(r + manager_record::LONGEST_CLUB) != -1 && read_i32(r + manager_record::CURRENT_CLUB) != -1 {
                    let current = days_between(today, r + manager_record::CURRENT_CLUB_SINCE);
                    if current > days_between(r + manager_record::LONGEST_END, r + manager_record::LONGEST_START) {
                        write_i32(r + manager_record::LONGEST_CLUB, read_i32(r + manager_record::CURRENT_CLUB));
                        staff_records_copy_date(r + manager_record::LONGEST_START, r + manager_record::CURRENT_CLUB_SINCE);
                        staff_records_copy_date(r + manager_record::LONGEST_END, r + manager_record::CURRENT_CLUB_END);
                    }
                } else {
                    write_i32(r + manager_record::LONGEST_CLUB, read_i32(r + manager_record::CURRENT_CLUB));
                    staff_records_copy_date(r + manager_record::LONGEST_START, r + manager_record::CURRENT_CLUB_SINCE);
                    staff_records_copy_date(r + manager_record::LONGEST_END, today);
                }
            }
            write_i32(r + manager_record::CURRENT_CLUB, -1);
            if signed == 1 {
                covered!("cqa_12 new spell");
                write_i32(r + manager_record::CURRENT_CLUB, read_i32(club.addr()));
                staff_records_copy_date(r + manager_record::CURRENT_CLUB_SINCE, today);
                let mut t = [0u8; 8];
                let d = exe::date_date_549f70((t.as_mut_ptr()) as *mut [u16; 4], 0xd, 0xb, 0x7b7, u32::MAX);
                staff_records_copy_date(r + manager_record::CURRENT_CLUB_END, d);
                write_u8(r + manager_record::CLUB_JOBS, read_u8(r + manager_record::CLUB_JOBS).wrapping_add(1)); // cqa12: as passagens
            }
        }
    } else if signed == 1 {
        let r = exe::staff_records_get_manager_record(this, manager, 1);
        if r != 0 {
            write_u8(r + manager_record::NATIONAL_JOBS, read_u8(r + manager_record::NATIONAL_JOBS).wrapping_add(1));
        }
    }
    let mut e = history_entry();
    stamp_game_date(e.as_mut_ptr());
    e[0xc] = 0x96;
    put_i32(&mut e, 8, read_i32(club.addr()));
    put_i32(&mut e, 0xd, signed as i32);
    append_person_history_by_id(!CUP_PLACED_CLUB_DRY_RUN.load(Relaxed), this, read_usize(manager), &e);
    if club.chairman().addr() != 0 {
        let mut e2 = history_entry();
        stamp_game_date(e2.as_mut_ptr());
        e2[0xc] = 0x97;
        put_i32(&mut e2, 8, read_i32(club.addr()));
        put_i32(&mut e2, 0xd, signed as i32);
        put_person_keys(&mut e2, 0x11, manager);
        append_person_history_by_id(!CUP_PLACED_CLUB_DRY_RUN.load(Relaxed), this, read_usize(club.chairman().addr()), &e2);
    }
    let mut e3 = history_entry();
    stamp_game_date(e3.as_mut_ptr());
    e3[8] = 0x96;
    put_person_keys(&mut e3, 9, manager);
    put_i32(&mut e3, 0x19, signed as i32);
    append_club_history_by_id(this, read_usize(club.addr()), &e3);
}

/// `Staff_Records::add_team_of_year_record(person, club, award, a, b)`: the team of the year entry in the person's
/// history. SEH frame only for the local date, with an empty destructor. Called by TEAM_AWARD::record_winners 0x8affb0.
///
/// Null person or award: the "Error" box (line 0x7ee). Otherwise the person's history (type 0x32: the club id or −1,
/// the award id, `a` as unsigned 16 bits and `b` as a signed byte), in the archive (0x58e340).
/// Original: `Staff_Records::add_team_of_year_record` (0x86c730, staff_records.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn add_team_of_year_record(this: usize, person: usize, club: usize, prize: usize, a: u32, b: u32) {
    covered!("cps_10 team of the year");
    if person == 0 || prize == 0 {
        show_error_at(0xa83b78, 0x7ee);
        return;
    }
    let mut e = history_entry();
    stamp_game_date(e.as_mut_ptr());
    e[0xc] = 0x32;
    put_i32(&mut e, 8, staff_records_id_or_minus_one(club));
    put_i32(&mut e, 0xd, read_i32(prize));
    put_i32(&mut e, 0x11, (a & 0xffff) as i32);
    put_i32(&mut e, 0x15, b as u8 as i8 as i32);
    append_person_history_by_id(true, this, read_usize(person), &e);
}

// simplification: the history goes to the file (does not run twice) and the copy writes nothing else: original only; the
// copy takes the branch.

/// Returns the club manager's points record from G_STAFF_RECORDS (`get_manager_record`, creating it), or 0 without a manager (uses
/// G_STAFF_RECORDS instead of this).
pub(crate) unsafe fn club_manager_record_global(club: Club) -> ManagerRecord {
    let manager = club.manager().addr();
    if manager == 0 {
        return ManagerRecord(0);
    }
    ManagerRecord(exe::staff_records_get_manager_record(read_usize(G_STAFF_RECORDS), manager, 1))
}

/// `staff_records_compare_clubs_by_manager_points(a, b)`: the qsort order of the manager of the year candidates
/// (`get_manager_of_year_candidates`): orders two clubs by their manager's league season points, highest first.
///
/// The records of the two clubs' managers (club_manager_record_global); neither, 0; only `a`'s missing, 1; only `b`'s
/// missing, −1; with both, `b`'s league season points (`.season_points()`) minus `a`'s (more points first).
/// Original: `staff_records_compare_clubs_by_manager_points` (0x870220, staff_records.cpp, cdecl).
pub(crate) unsafe extern "C" fn staff_records_compare_clubs_by_manager_points(a: usize, b: usize) -> i32 {
    let (ca, cb) = (read_usize(a), read_usize(b));
    let ra = ManagerRecord((club_manager_record_global(Club(ca))).addr());
    let rb = ManagerRecord((club_manager_record_global(Club(cb))).addr());
    if ra.is_null() {
        return if !rb.is_null() { 1 } else { 0 };
    }
    if rb.is_null() {
        return -1;
    }
    covered!("cps_10 points of both");
    rb.season_points().wrapping_sub(ra.season_points()) // cps10: most points first
}

// simplification: 0x86f5b0 creates the missing record; the original runs first and the copy only finds the ones it created.

/// Adds v to the i32 at address a.
pub(crate) unsafe fn staff_records_add_i32(a: usize, v: i32) {
    write_i32(a, read_i32(a).wrapping_add(v));
}

/// A new 0x40-byte history entry on the stack: the date (`exe::date_date_54a110`, Date()) with the current game date copied in
/// (`exe::date_operator_3d`).
///
/// Simplification: the bytes the original routine does not write (its stack garbage) come out zeroed here; they only go
/// to the history file (FILE_LLIST), which the career never reads back.
pub(crate) unsafe fn dated_history_entry() -> [u8; 0x40] {
    let mut e = [0u8; 0x40];
    let mut t = [0u8; 8];
    exe::date_date_54a110((e.as_mut_ptr()) as usize);
    exe::date_operator_3d((e.as_mut_ptr()) as usize, (t.as_mut_ptr()) as usize, G_CURRENT_DATE);
    e
}

/// Writes u32 v at offset o of a history entry.
pub(crate) unsafe fn put_u32(e: &mut [u8; 0x40], o: usize, v: u32) {
    e[o..o + 4].copy_from_slice(&v.to_le_bytes());
}

/// Appends entry e to the list of id (head/tail at base + id × 8) in FILE_LLIST arq; nothing when not writing.
pub(crate) unsafe fn append_history(write: bool, arq: usize, base: usize, id: usize, e: &[u8; 0x40]) {
    if !write {
        return;
    }
    let p = base.wrapping_add(id.wrapping_mul(8));
    exe::file_llist_add_node(arq, p, p + 4, e.as_ptr());
}

/// Appends entry e to the person's history (file [this], lists [this+0xc]).
pub(crate) unsafe fn append_person_history(write: bool, this: usize, person: Staff, e: &[u8; 0x40]) {
    append_history(write, read_usize(this), read_usize(this + staff_records_fields::PERSON_HISTORY_LISTS), read_usize(person.addr()), e);
}

/// Appends a dated club history entry with type and competition id (file [this+4], lists [this+0x10]).
pub(crate) unsafe fn append_club_history(write: bool, this: usize, club: Club, kind: u8, comp: Competition) {
    let mut e = dated_history_entry();
    e[8] = kind;
    put_u32(&mut e, 9, read_usize(comp.addr()) as u32);
    append_history(write, read_usize(this + staff_records_fields::CLUB_HISTORY_FILE), read_usize(this + staff_records_fields::CLUB_HISTORY_LISTS), read_usize(club.addr()), &e);
}

/// Builds a dated person history entry with club id (+8), type (+0xc) and competition (+0xd).
pub(crate) unsafe fn person_history_entry(club: Club, kind: u8, comp: u32) -> [u8; 0x40] {
    let mut e = dated_history_entry();
    put_u32(&mut e, 8, read_usize(club.addr()) as u32);
    e[0xc] = kind;
    put_u32(&mut e, 0xd, comp);
    e
}

/// Iterates the club's 50 squad slots yielding players with a player record (+0x61), id below [0xae2364] and not the
/// manager.
pub(crate) unsafe fn counted_squad_players(club: Club) -> impl Iterator<Item = usize> {
    (0..50usize).filter_map(move |k| {
        let p = Staff(club.squad(k));
        if p.is_null() || p.player().is_null() || read_i32(p.addr()) >= read_i32(G_STAFF_COUNT) || p == club.manager() {
            None
        } else {
            Some(p.addr())
        }
    })
}

/// Read-only: true if `contract_manager_offer_contract` would run, i.e. transfer manager exists, contract older than 300 days (+0x25) and the
/// player is on the club shortlist.
pub(crate) unsafe fn will_renew_contract(club: Club, p: usize, c: Contract) -> bool {
    let gt = TransferManagerFields(read_usize(G_TRANSFER_MANAGER));
    if gt.is_null() {
        return false;
    }
    let days: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
    if days(G_CURRENT_DATE, c.date_started(), read_usize(c.addr() + contract::DATE_STARTED + 4)) <= 0x12c {
        return false;
    }
    let e = ShortlistSlot(exe::shortlist_manager_find_club_shortlist_entry(gt.shortlist_manager(), club, p, 0, 0));
    !e.is_null() && e.target_staff_id() == read_usize(p)
}

/// Offers a contract renewal (`contract_manager_offer_contract` on G_CONTRACT_MANAGER: player, no club or terms, slot −1, factor 1.0, mode 1).
pub(crate) unsafe fn offer_renewal(p: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize, u32, u32, i32, u32, u32, u32) -> u32 = core::mem::transmute(FN_CONTRACT_MANAGER_OFFER_CONTRACT);
    f(read_usize(G_CONTRACT_MANAGER), p, 0, 0, -1, 0, 0x3ff0_0000, 1);
}

/// Adjusts the person's morale by delta (FN_STAFF_ADJUST_MORALE with 1).
pub(crate) unsafe fn adjust_morale(p: usize, delta: i32) {
    exe::staff_adjust_morale(p, (delta) as u32, 1);
}

/// Raises a non-player rating i16 by ftol(x × weight × [0x96ff88]), floored at x × weight and capped at 10000.
pub(crate) unsafe fn raise_promotion_rating(a: usize, x: i32, weight: i32) {
    write_i16(a, read_i16(a).wrapping_add(ftol(x.wrapping_mul(weight) as f64 * read_f64(rdata::F64_0_1639344262)) as i16));
    let minimum = (x as i16).wrapping_mul(weight as i16);
    let v = read_i16(a);
    write_i16(a, if minimum > v { minimum } else if v > 0x2710 { 0x2710 } else { v });
}

/// Promotion of a club: reputation +500, max attendance, manager ratings, histories for manager, chairman and squad,
/// squad reputations and morale, renewals; no file histories when not writing.
pub(crate) unsafe fn apply_promotion(this: usize, club: Club, comp: Competition, weight: u32, write: bool) {
    if club.is_null() || comp.is_null() {
        show_error_at(0xa83b78, 0x301);
        return;
    }
    let e = person_history_entry(club, 8, read_usize(comp.addr()) as u32);
    club.set_reputation(club.reputation().wrapping_add(500));
    if club.reputation() > 0x2710 {
        club.set_reputation(0x2710);
    }
    let v = club.max_attendance();
    if v < 0x249f0 && v < club.attendance().wrapping_add(500) {
        club.set_max_attendance(v.wrapping_add(comp.reputation() as i32 * 10));
    }
    if club.manager().addr() != 0 {
        let manager = club.manager();
        if !manager.non_player().is_null() {
            let object = League(read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(comp.addr()).wrapping_mul(4))));
            let mut pela = comp.addr();
            if !object.is_null() && object.upper_division_id() > -1 {
                let cc = read_usize(G_CLUB_COMP_TABLE).wrapping_add((object.upper_division_id() as usize).wrapping_mul(0x6b));
                pela = if cc != 0 { cc } else { comp.addr() };
            }
            let (p1, p2) = if object.club_sides() as i8 != 1 {
                (400, 400)
            } else {
                (200, if exe::database_is_staff_foreign(manager, club.nation().addr(), 1) != 0 { 200 } else { 400 })
            };
            let x = read_i16(pela + competition::REPUTATION) as i32;
            let nj = || club.manager().non_player();
            covered!("cpq_3 manager promotion");
            raise_promotion_rating(nj().addr() + non_player::CURRENT_REPUTATION, x, 400);
            raise_promotion_rating(nj().addr() + 8, x, p2);
            raise_promotion_rating(nj().addr() + non_player::WORLD_REPUTATION, x, p1);
        }
        append_person_history(write, this, club.manager(), &e);
        let r = ManagerRecord(exe::staff_records_get_manager_record(this, club.manager().addr(), 1));
        if !r.is_null() {
            let points = (comp.reputation() as i32).wrapping_mul(weight as u8 as i8 as i32).wrapping_mul(2500) / 100;
            staff_records_add_i32(r.addr() + manager_record::YEAR_POINTS, points);
            staff_records_add_i32(r.addr() + manager_record::SEASON_POINTS, points);
            staff_records_add_i32(r.addr() + manager_record::MONTH_POINTS, points);
        }
    }
    if club.chairman().addr() != 0 {
        append_person_history(write, this, club.chairman(), &e);
    }
    for p in counted_squad_players(club) {
        adjust_morale(p, 0xf);
        let c = Contract((game_contract_for_club(p, club)).addr());
        if c.is_null() {
            continue;
        }
        let v = (c.morale() as u8) as i8;
        let s = v as i32 + 0x4b;
        c.set_morale((if s > 0x64 { 0x64 } else if s < -0x64 { 0x9c } else { v.wrapping_add(0x4b) as u8 }) as i8);
        let kind = c.squad_status() >> 4;
        if (kind == 0 || (kind > 3 && kind != 5)) || !will_renew_contract(club, p, c) {
            continue;
        }
        covered!("cpq_3 promotion renews");
        offer_renewal(p);
    }
    append_club_history(write, this, club, 8, comp);
}

/// `Staff_Records::add_promotion_record(club, comp, weight)`: records a promotion. SEH frame without a destructor.
///
/// The club's reputation (`.reputation()`) goes up 500 (up to 10000) and `.max_attendance()` goes up 10 × the
/// competition's importance (`.reputation()`) if it is below 150000 and below `.attendance()` + 500. The manager
/// (`.manager()`) with a non-player record: ratings `non_player::CURRENT_REPUTATION`, +8 and
/// `non_player::WORLD_REPUTATION` go up by the competition's importance (that of its club competition record
/// `.upper_division_id()`, if any; `competition::REPUTATION`) × 400, × 200/400 (foreigner in the league with
/// `.club_sides()` = 1, `exe::database_is_staff_foreign`) and × 200/400 (league with `.club_sides()` = 1) × 10/61, with
/// a minimum of importance × weight.
///
/// The manager's and chairman's (`.chairman()`) history (type 8), the manager's points
/// (`exe::staff_records_get_manager_record`: `manager_record::YEAR_POINTS`, `manager_record::SEASON_POINTS`,
/// `manager_record::MONTH_POINTS` add importance × weight × 25), morale +15 for each squad player
/// (`staff_adjust_morale`), the contract bond (`.morale()`) +75 (−100..100) and the renewal
/// (`contract_manager_offer_contract`) of the expiring contracts of types 1, 2, 3 and 5; finally the club's history
/// (type 8). Null club or competition: the "Error" box (line 0x301).
/// Original: `Staff_Records::add_promotion_record` (0x869480, staff_records.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn add_promotion_record(this: usize, club: usize, comp: usize, weight: u32) {
    covered!("cpq_3 promotion");
    apply_promotion(this, Club(club), Competition(comp), weight, true)
}

/// The memory regions that promotion or relegation touch (without the histories); `None` if any contract would be
/// renewed.
///
/// The random generator state (morale uses random draws), the club, the manager's non-player record, his points record
/// (created beforehand, as the original would), and the morale (+0x45) and reputations (+9..+0xe) of each squad player
/// and the link (+0x44) of his contract. `None` if any contract goes to renewal (`contract_manager_offer_contract` makes the offer: it does not
/// run twice; on promotion types 1, 2, 3 and 5, on relegation 1 and 2).
pub(crate) unsafe fn promotion_regions(this: usize, club: Club, promotion: bool) -> Option<Vec<(usize, usize)>> {
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.push((club.addr(), club::SIZE));
    let manager = club.manager();
    if !manager.is_null() {
        if !manager.non_player().is_null() {
            reg.push((manager.non_player().addr(), 0x10));
        }
        if promotion {
            let r = exe::staff_records_get_manager_record(this, manager, 1);
            if r != 0 {
                reg.push((r, 0xd2));
            }
        }
    }
    for p in counted_squad_players(club) {
        let j = PlayerSheet(read_usize(p + staff::PLAYER));
        reg.extend([(j.addr() + player_sheet::HOME_REPUTATION, 6), (j.addr() + player_sheet::PLAYER_MORALE, 1)]);
        let c = Contract((game_contract_for_club(p, club)).addr());
        if !c.is_null() {
            let kind = c.squad_status() >> 4;
            let renova = if promotion { kind != 0 && (kind <= 3 || kind == 5) } else { kind == 1 || kind == 2 };
            if renova && will_renew_contract(club, p, c) {
                return None;
            }
            reg.push((c.addr() + contract::MORALE, 1));
        }
    }
    Some(reg)
}

/// Adds v to a non-player rating i16, clamped to 1..10000.
pub(crate) unsafe fn add_rating_clamped(a: usize, v: i16) {
    write_i16(a, read_i16(a).wrapping_add(v));
    let n = read_i16(a);
    write_i16(a, if n < 1 { 1 } else if n > 0x2710 { 0x2710 } else { n });
}

/// Multiplies the player record reputations (+0xb current, +9 home, +0xd world) by k.
pub(crate) unsafe fn scale_player_reputations(j: usize, k: f64) {
    for o in [0xb, 9, 0xd] {
        write_i16(j + o, ftol(read_i16(j + o) as i32 as f64 * k) as i16);
    }
}

/// Relegation of a club: reputation −500 (min 750), manager ratings down, histories for manager, chairman and squad,
/// squad reputations and morale.
pub(crate) unsafe fn apply_relegation(this: usize, club: Club, comp: Competition, write: bool) {
    if club.is_null() || comp.is_null() {
        show_error_at(0xa83b78, 0x476);
        return;
    }
    let e = person_history_entry(club, 9, read_usize(comp.addr()) as u32);
    club.set_reputation(club.reputation().wrapping_sub(500));
    if club.reputation() < 0x2ee {
        club.set_reputation(0x2ee);
    }
    if club.manager().addr() != 0 {
        let manager = club.manager();
        if !manager.non_player().is_null() {
            let factor: i16 = if exe::database_is_staff_foreign(manager, club.nation().addr(), 1) != 0 { -8 } else { -16 };
            let nj = || club.manager().non_player();
            let x = || comp.reputation();
            covered!("cpq_3 manager relegation");
            add_rating_clamped(nj().addr() + non_player::CURRENT_REPUTATION, x().wrapping_mul(-16));
            add_rating_clamped(nj().addr() + 8, x().wrapping_mul(factor));
            add_rating_clamped(nj().addr() + non_player::WORLD_REPUTATION, x().wrapping_mul(-8));
        }
        append_person_history(write, this, club.manager(), &e);
    }
    if club.chairman().addr() != 0 {
        append_person_history(write, this, club.chairman(), &e);
    }
    for p in counted_squad_players(club) {
        adjust_morale(p, -0xf);
        let c = Contract((game_contract_for_club(p, club)).addr());
        if c.is_null() {
            continue;
        }
        let j = read_usize(p + staff::PLAYER);
        match c.squad_status() >> 4 {
            1 | 2 => {
                if j != 0 {
                    scale_player_reputations(j, read_f64(rdata::F64_0_9)); // 0,9
                }
                if will_renew_contract(club, p, c) {
                    covered!("cpq_3 relegation renews");
                    offer_renewal(p);
                }
            }
            3..=5 => {
                if j != 0 {
                    scale_player_reputations(j, read_f64(rdata::F64_0_95)); // 0,95
                }
            }
            _ => {}
        }
    }
    append_club_history(write, this, club, 9, comp);
}

/// `Staff_Records::add_relegation_record(club, comp)`: records a relegation. SEH frame without a destructor.
///
/// The club's reputation (`.reputation()`) drops 500 (minimum 750). The manager with a non-player record: ratings
/// `non_player::CURRENT_REPUTATION` (−16), +8 (−8 if a foreigner, `exe::database_is_staff_foreign`, or −16) and
/// `non_player::WORLD_REPUTATION` (−8) × the competition's importance (`.reputation()`), clamped to 1..10000; the
/// manager's and chairman's history (type 9). Each squad player: morale −15 (`staff_adjust_morale`) and, by contract
/// type (`.squad_status()` >> 4): 1 and 2, reputations × 0.9 and the renewal (`contract_manager_offer_contract`) if
/// expiring; 3 to 5, reputations × 0.95. Finally the club's history (type 9). Null club or competition: the "Error" box
/// (line 0x476).
/// Original: `Staff_Records::add_relegation_record` (0x86a1c0, staff_records.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn add_relegation_record(this: usize, club: usize, comp: usize) {
    covered!("cpq_3 relegation");
    apply_relegation(this, Club(club), Competition(comp), true)
}

/// Qualification of a club: person history entry (type 3, stage, base round) for the manager and adds reputation² ×
/// weight points to his record (national team or club totals).
pub(crate) unsafe fn record_qualification(this: usize, club: Club, comp: Competition, stage: u32, round: u32, weight: u32, write: bool) {
    if club.is_null() {
        show_error_at(0xa83b78, 0x703);
        return;
    }
    let mut e = person_history_entry(club, 3, if !comp.is_null() { read_usize(comp.addr()) as u32 } else { u32::MAX });
    put_u32(&mut e, 0x11, stage as u16 as i16 as i32 as u32);
    put_u32(&mut e, 0x15, exe::comp_util_get_base_round_id(round) as u16 as i16 as i32 as u32);
    if club.manager().addr() == 0 {
        return;
    }
    append_person_history(write, this, club.manager(), &e);
    let r = ManagerRecord(exe::staff_records_get_manager_record(this, club.manager().addr(), 1));
    if r.is_null() {
        return;
    }
    // simplification: the original does pow(x, 2.0) (0x945ff0): x × x, exact for the importance (16 bits)
    let x = comp.reputation() as f64;
    let points = ftol(x * x * (weight as u8 as i8 as i32) as f64);
    staff_records_add_i32(r.addr() + manager_record::YEAR_POINTS, points);
    staff_records_add_i32(r.addr() + manager_record::MONTH_POINTS, points);
    if exe::database_is_national_team(club) != 0 {
        covered!("cpq_3 qualified national team");
        staff_records_add_i32(r.addr() + manager_record::NATIONAL_SEASON_POINTS, points);
    } else {
        covered!("cpq_3 qualified club");
        staff_records_add_i32(r.addr() + manager_record::SEASON_POINTS, points);
    }
}

/// `Staff_Records::add_qualified_for_round_record(club, comp, stage, round, weight)`: SEH frame without a destructor.
///
/// With a manager (`.manager()`), his history (type 3: club, competition or −1, the signed stage and the base round
/// `exe::comp_util_get_base_round_id`) and the points (`exe::staff_records_get_manager_record`): importance² × weight
/// added to `manager_record::YEAR_POINTS` (year), `manager_record::MONTH_POINTS` (month) and, for a national team
/// (`exe::database_is_national_team`), `manager_record::NATIONAL_SEASON_POINTS` (cup), or for a club
/// `manager_record::SEASON_POINTS` (league). Null club: the "Error" box (line 0x703).
/// Original: `Staff_Records::add_qualified_for_round_record` (0x86bdd0, staff_records.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn add_qualified_for_round_record(this: usize, club: usize, comp: usize, stage: u32, round: u32, weight: u32) {
    record_qualification(this, Club(club), Competition(comp), stage, round, weight, true)
}

/// Adds sign-extended v to the i64 at address a.
pub(crate) unsafe fn staff_records_add_i64(a: usize, v: i32) {
    let x = read_i64(a);
    write_i64(a, x.wrapping_add(v as i64));
}

/// Writes i32 v at offset o of a history entry.
pub(crate) unsafe fn put_i32(e: &mut [u8; 0x40], o: usize, v: i32) {
    e[o..o + 4].copy_from_slice(&v.to_le_bytes());
}

/// Copies the current game date (0xae2c90) into the date at a.
pub(crate) unsafe fn stamp_game_date(a: *mut u8) {
    let mut t = [0u8; 8];
    exe::date_operator_3d((a) as usize, (t.as_mut_ptr()) as usize, G_CURRENT_DATE);
}

/// New 0x40-byte history entry on the stack with a constructed Date (`exe::date_date_54a110`).
pub(crate) unsafe fn history_entry() -> [u8; 0x40] {
    let mut e = [0u8; 0x40];
    exe::date_date_54a110((e.as_mut_ptr()) as usize);
    e
}

/// Appends entry e to person id's list in the person history FILE_LLIST [this] (heads at [this+0xc]); nothing when not
/// writing (takes the person id and this directly).
pub(crate) unsafe fn append_person_history_by_id(write: bool, this: usize, id: usize, e: &[u8; 0x40]) {
    if !write {
        return;
    }
    let p = read_usize(this + staff_records_fields::PERSON_HISTORY_LISTS).wrapping_add(id.wrapping_mul(8));
    exe::file_llist_add_node(read_usize(this), p, p + 4, e.as_ptr());
}

/// Returns the id of the person's employing club (staff::CLUB_JOB), or −1.
pub(crate) unsafe fn club_job_id(person: Staff) -> i32 {
    let c = person.club_job();
    if c.is_null() {
        -1
    } else {
        read_i32(c.addr())
    }
}

/// Returns the record's id, or −1 for a null pointer.
pub(crate) unsafe fn staff_records_id_or_minus_one(p: usize) -> i32 {
    if p == 0 {
        -1
    } else {
        read_i32(p)
    }
}

/// Writes the person's id and the +0x33 ids of its +4, +8, +0xc pointers into entry e from offset o.
pub(crate) unsafe fn put_person_keys(e: &mut [u8; 0x40], o: usize, p: usize) {
    put_i32(e, o, read_i32(p));
    for k in 0..3usize {
        put_i32(e, o + 4 + 4 * k, read_i32(read_usize(p + 4 + 4 * k) + names::ID));
    }
}

/// Calls f for each item of the transfer's swap list (LLIST at +4), resetting the cursor first.
pub(crate) unsafe fn for_each_swap_item(t: Offer, mut f: impl FnMut(usize)) {
    let l = t.exchange_players();
    if l.is_null() {
        return;
    }
    l.set_cursor((l.head() as u32) as usize);
    let mut e = exe::llist_get_next(t.exchange_players());
    while e != 0 {
        f(e);
        e = exe::llist_get_next(t.exchange_players());
    }
}

/// True if the job type is a player job (`exe::database_is_player_job`: 11, 12, 13 or 15).
pub(crate) unsafe fn staff_records_is_player_job(job: u8) -> bool {
    exe::database_is_player_job((job as u32) as i8) != 0 // simplification: the original pushes the register with the byte in al; 0x53b620 reads only the byte
}

/// Returns the manager's points record (FN_STAFF_RECORDS_GET_MANAGER_RECORD with create).
pub(crate) unsafe fn staff_records_manager_record(this: usize, manager: Staff) -> usize {
    exe::staff_records_get_manager_record(this, manager, 1)
}

/// Sides of a transfer: (person, his contract, selling club or 0, buying club).
pub(crate) unsafe fn transfer_parties(t: Offer) -> (usize, usize, usize, usize) {
    let person = staff_records_staff_by_id(t.target_staff_id());
    let c = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person));
    let seller = if !c.is_null() && c.contract_type() & 0x3f != 5 {
        read_usize(G_CLUB_TABLE).wrapping_add(c.club().wrapping_mul(club::SIZE))
    } else {
        0
    };
    (person.addr(), c.addr(), seller, read_usize(G_CLUB_TABLE).wrapping_add(t.buyer_club_id().wrapping_mul(club::SIZE)))
}

/// Records a transfer in the staff records: biggest purchase/sale and totals in the buying and selling managers'
/// records, plus history entries for the player and clubs; no file histories when not writing.
pub(crate) unsafe fn record_transfer(this: usize, t: Offer, write: bool) {
    let (pessoa, c, vendedor, comprador) = transfer_parties(t);
    write_usize(G_CONTRACT_LOOKUP_RESULT, c);
    let mut e = history_entry();
    let value = exe::transfer_offer_get_total_value(t, 1, 0, 0, 1);
    let with_swap = || (!t.exchange_players().is_null()) as i32;
    let kind = t.job_type() as i8 as i32;
    if comprador != 0 && read_usize(comprador + club::MANAGER) != 0 {
        let manager = read_usize(comprador + club::MANAGER);
        let r = if staff_records_is_player_job(t.job_type()) { staff_records_manager_record(this, Staff(manager)) } else { 0 };
        if r != 0 {
            covered!("cpq_4 purchase");
            if value > read_i32(r + manager_record::BIGGEST_PURCHASE_FEE) {
                covered!("cpq_4 biggest purchase");
                for k in 0..4usize {
                    write_i32(r + 4 * k, if k == 0 { read_i32(pessoa) } else { read_i32(read_usize(pessoa + staff::FIRST_NAME * k) + names::ID) });
                }
                stamp_game_date((r + manager_record::BIGGEST_PURCHASE_DATE) as *mut u8);
                write_i32(r + manager_record::BIGGEST_PURCHASE_FEE, value);
                write_u8(r + manager_record::BIGGEST_PURCHASE_SWAP, with_swap() as u8);
                write_i32(r + manager_record::BIGGEST_PURCHASE_SELLER, staff_records_id_or_minus_one(vendedor));
                write_i32(r + manager_record::BIGGEST_PURCHASE_BUYER, read_i32(comprador));
            }
            write_i16(r + manager_record::PLAYERS_BOUGHT, read_i16(r + manager_record::PLAYERS_BOUGHT).wrapping_add(1));
            staff_records_add_i64(r + manager_record::PURCHASES_TOTAL, value);
            if !t.exchange_players().is_null() {
                write_i16(r + manager_record::PLAYERS_SOLD, read_i16(r + manager_record::PLAYERS_SOLD).wrapping_add(read_i16(t.exchange_players().addr())));
                for_each_swap_item(t, |x| staff_records_add_i64(r + manager_record::SALES_TOTAL, read_i32((staff_records_staff_by_id(read_usize(x))).addr() + staff::VALUE)));
            }
        }
        // the buying manager’s history: the purchase and each player given in the swap
        stamp_game_date(e.as_mut_ptr());
        e[0xc] = 0x64;
        put_i32(&mut e, 8, club_job_id(Staff(manager)));
        put_person_keys(&mut e, 0xd, pessoa);
        put_i32(&mut e, 0x1d, value);
        put_i32(&mut e, 0x21, with_swap());
        put_i32(&mut e, 0x25, staff_records_id_or_minus_one(vendedor));
        put_i32(&mut e, 0x29, kind);
        append_person_history_by_id(write, this, read_usize(manager), &e);
        for_each_swap_item(t, |x| {
            e[0xc] = 0x65;
            put_person_keys(&mut e, 0xd, (staff_records_staff_by_id(read_usize(x))).addr());
            put_i32(&mut e, 0x1d, 0);
            put_i32(&mut e, 0x21, 1);
            put_i32(&mut e, 0x25, read_i32(vendedor));
            put_i32(&mut e, 0x29, kind);
            append_person_history_by_id(write, this, read_usize(manager), &e);
        });
    }
    if vendedor != 0 && read_usize(vendedor + club::MANAGER) != 0 {
        let manager = read_usize(vendedor + club::MANAGER);
        let r = if staff_records_is_player_job(read_u8(pessoa + staff::JOB_FOR_CLUB)) { staff_records_manager_record(this, Staff(manager)) } else { 0 };
        if r != 0 {
            covered!("cpq_4 sale");
            if value > read_i32(r + manager_record::BIGGEST_SALE_FEE) {
                for k in 0..4usize {
                    write_i32(r + manager_record::BIGGEST_SALE_PERSON + 4 * k, if k == 0 { read_i32(pessoa) } else { read_i32(read_usize(pessoa + staff::FIRST_NAME * k) + names::ID) });
                }
                stamp_game_date((r + manager_record::BIGGEST_SALE_DATE) as *mut u8);
                write_i32(r + manager_record::BIGGEST_SALE_FEE, value);
                write_u8(r + manager_record::BIGGEST_SALE_SWAP, with_swap() as u8);
                write_i32(r + manager_record::BIGGEST_SALE_SELLER, read_i32(vendedor));
                write_i32(r + manager_record::BIGGEST_SALE_BUYER, staff_records_id_or_minus_one(comprador));
            }
            write_i16(r + manager_record::PLAYERS_SOLD, read_i16(r + manager_record::PLAYERS_SOLD).wrapping_add(1));
            staff_records_add_i64(r + manager_record::SALES_TOTAL, value);
            if !t.exchange_players().is_null() {
                write_i16(r + manager_record::PLAYERS_BOUGHT, read_i16(r + manager_record::PLAYERS_BOUGHT).wrapping_add(read_i16(t.exchange_players().addr())));
                for_each_swap_item(t, |x| staff_records_add_i64(r + manager_record::PURCHASES_TOTAL, read_i32((staff_records_staff_by_id(read_usize(x))).addr() + staff::VALUE)));
            }
        }
        // the selling manager’s history: the sale and each player received in the swap
        stamp_game_date(e.as_mut_ptr());
        e[0xc] = 0x65;
        put_i32(&mut e, 8, club_job_id(Staff(manager)));
        put_person_keys(&mut e, 0xd, pessoa);
        put_i32(&mut e, 0x1d, value);
        put_i32(&mut e, 0x21, with_swap());
        put_i32(&mut e, 0x25, staff_records_id_or_minus_one(comprador));
        put_i32(&mut e, 0x29, kind);
        append_person_history_by_id(write, this, read_usize(manager), &e);
        for_each_swap_item(t, |x| {
            e[0xc] = 0x64;
            put_person_keys(&mut e, 0xd, (staff_records_staff_by_id(read_usize(x))).addr());
            put_i32(&mut e, 0x1d, 0);
            put_i32(&mut e, 0x21, 1);
            put_i32(&mut e, 0x25, read_i32(comprador));
            put_i32(&mut e, 0x29, 0xb);
            append_person_history_by_id(write, this, read_usize(manager), &e);
        });
    }
    // the history of the transferred player and that of each player in the swap
    stamp_game_date(e.as_mut_ptr());
    e[0xc] = 0x66;
    put_i32(&mut e, 8, club_job_id(Staff(pessoa)));
    put_i32(&mut e, 0xd, staff_records_id_or_minus_one(comprador));
    put_i32(&mut e, 0x11, value);
    put_i32(&mut e, 0x19, kind);
    put_i32(&mut e, 0x15, with_swap());
    if pessoa != 0 {
        append_person_history_by_id(write, this, read_usize(pessoa), &e);
    } else {
        show_error_at(0xa83b78, 0x1010);
    }
    for_each_swap_item(t, |x| {
        let q = Staff((staff_records_staff_by_id(read_usize(x))).addr());
        put_i32(&mut e, 8, read_i32(q.club_job().addr()));
        put_i32(&mut e, 0x11, 0);
        put_i32(&mut e, 0xd, read_i32(vendedor));
        put_i32(&mut e, 0x15, 1);
        put_i32(&mut e, 0x19, read_u8(x + 6) as i8 as i32);
        if !q.is_null() {
            append_person_history_by_id(write, this, read_usize(q.addr()), &e);
        } else {
            show_error_at(0xa83b78, 0x1010);
        }
    });
}

/// `Staff_Records::add_transfer_records(transfer)`: updates the buying and selling managers' records and the transfer
/// histories. SEH frame without a destructor.
///
/// The person's contract (`exe::contract_manager_get_contract`) goes to `G_CONTRACT_LOOKUP_RESULT` and gives the seller
/// (outside state 5); the offer's total value (`exe::transfer_offer_get_total_value` with 1, 0, 0, 1). The buyer's
/// manager (`club::MANAGER`), when the offer type is for a player (`.job_type()`, `exe::database_is_player_job`): in
/// the record (`get_manager_record`) the biggest signing (+0..+0x24, from `manager_record::BIGGEST_PURCHASE_DATE` to
/// `manager_record::BIGGEST_PURCHASE_BUYER`, if the value exceeds `manager_record::BIGGEST_PURCHASE_FEE`), the signings
/// (`manager_record::PLAYERS_BOUGHT`), the spending (`manager_record::PURCHASES_TOTAL`, 64 bits), the players given in
/// exchange (`manager_record::PLAYERS_SOLD`) and their value (`staff::VALUE` of each, in
/// `manager_record::SALES_TOTAL`). The seller's manager, when the person's job is player (`staff::JOB_FOR_CLUB`): the
/// biggest sale (`manager_record::BIGGEST_SALE_PERSON`..+0x49 if it exceeds `manager_record::BIGGEST_SALE_FEE`), the
/// sales (`manager_record::PLAYERS_SOLD`), the income (`manager_record::SALES_TOTAL`), the players received
/// (`manager_record::PLAYERS_BOUGHT`) and their value (`manager_record::PURCHASES_TOTAL`).
///
/// The histories (types 0x64 buy, 0x65 sale, 0x66 transfer) of the buyer's manager, the seller's manager, the person
/// and the exchange players, in the archive (0x58e340).
/// Original: `Staff_Records::add_transfer_records` (0x86cd70, staff_records.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn add_transfer_records(this: usize, t: usize) {
    record_transfer(this, Offer(t), true)
}

/// Builds a dated person history entry with club id (+8), type (+0xc), competition id or −1 (+0xd) and the signed stage
/// (+0x11).
pub(crate) unsafe fn stage_history_entry(club: Club, kind: u8, comp: usize, stage: u32) -> [u8; 0x40] {
    let mut e = history_entry();
    stamp_game_date(e.as_mut_ptr());
    put_i32(&mut e, 8, read_i32(club.addr()));
    e[0xc] = kind;
    put_i32(&mut e, 0xd, staff_records_id_or_minus_one(comp));
    put_i32(&mut e, 0x11, stage as u16 as i16 as i32);
    e
}

/// If the competition is the stored national-team finals competition (0x76d940 on G_NATIONAL_TEAMS), ends the nation's
/// call-ups for it (`end_duty_for_nation_in_competition`).
pub(crate) unsafe fn end_national_duty(club: Club, comp: usize) {
    if exe::national_teams_is_finals_comp(read_usize(G_NATIONAL_TEAMS), comp) != 0 {
        covered!("cpq_4 end of call-ups");
        exe::nationalteams_end_duty_for_nation_in_competition(read_usize(G_NATIONAL_TEAMS), club, comp);
    }
}

/// `Staff_Records::add_knocked_out_record(club, comp, stage, round, _)`: SEH frame without a destructor.
///
/// The manager's history (`.manager()`; type 4: club, competition or −1, the signed stage and the base round
/// `exe::comp_util_get_base_round_id`) and the end of the national team's call-ups in the stored competition. Null
/// club: the "Error" box (line 0x73b).
/// Original: `Staff_Records::add_knocked_out_record` (0x86c000, staff_records.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn add_knocked_out_record(this: usize, club: usize, comp: usize, stage: u32, round: u32, _x: u32) {
    let club = Club(club);
    if club.is_null() {
        return show_error_at(0xa83b78, 0x73b);
    }
    covered!("cpq_4 eliminated");
    let mut e = stage_history_entry(club, 4, comp, stage);
    put_i32(&mut e, 0x15, exe::comp_util_get_base_round_id(round) as u16 as i16 as i32);
    if club.manager().addr() != 0 {
        append_person_history_by_id(true, this, read_usize(club.manager().addr()), &e);
    }
    end_national_duty(club, comp);
}

// simplification: the history goes to the file and the call-ups end (they do not run twice) and the copy without them does not touch
// in nothing: original only; the copy carries the branches.

/// `Staff_Records::add_failed_to_qualify_record(club, comp, stage, _)`: SEH frame without a destructor.
///
/// The manager's history (type 7: club, competition or −1, the signed stage) and the end of the national team's
/// call-ups in the stored competition. Null club: the "Error" box (line 0x768).
/// Original: `Staff_Records::add_failed_to_qualify_record` (0x86c1d0, staff_records.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn add_failed_to_qualify_record(this: usize, club: usize, comp: usize, stage: u32, _x: u32) {
    let club = Club(club);
    if club.is_null() {
        return show_error_at(0xa83b78, 0x768);
    }
    covered!("cpq_4 not qualified");
    let e = stage_history_entry(club, 7, comp, stage);
    if club.manager().addr() != 0 {
        append_person_history_by_id(true, this, read_usize(club.manager().addr()), &e);
    }
    end_national_duty(club, comp);
}

// simplification: like the elimination: original only.

/// The points record of each manager (Staff_Records +8): 0xd2 bytes per non-player person ([0xae2368]) and for the 16
/// humans.
///
/// +0x86 person id (−1 empty), +0xb6 points for the year, +0xba/+0xbe points in the league and in the cup (+0xc2 and
/// +0xc6 their competitions, −1 none), +0xca/+0xce those for the month.
pub(crate) const MANAGER_RECORD_SIZE: usize = 0xd2;

/// Number of manager records: [0xae2368] (non-player count) + 16 human slots.
pub(crate) unsafe fn manager_record_count() -> i32 {
    read_i32(G_NON_PLAYER_COUNT).wrapping_add(0x10)
}

/// Returns manager record k in the records array [this+8] (MANAGER_RECORD_SIZE bytes each).
pub(crate) unsafe fn manager_record_at(this: usize, k: i32) -> usize {
    read_usize(this + staff_records_fields::MANAGER_RECORDS).wrapping_add((k as usize).wrapping_mul(MANAGER_RECORD_SIZE))
}

/// Returns the staff record of id ([G_STAFF_TABLE] + id × 0x6e; −1 gives the record before the table, as the original).
pub(crate) unsafe fn staff_records_staff_by_id(id: usize) -> Staff {
    Staff(read_usize(G_STAFF_TABLE).wrapping_add(id.wrapping_mul(staff::SIZE)))
}

/// Returns the month of a date (FN_DATE_GET_MONTH, low byte signed).
pub(crate) unsafe fn date_month(data: usize) -> i16 {
    (exe::date_get_month(data) as i8) as i16
}

/// `Staff_Records::daily_update(date)`: if the date's month (`exe::date_get_month`) or year (+2) are not the stored
/// ones (`staff_records_fields::CURRENT_MONTH` month, `staff_records_fields::CURRENT_YEAR` year), zeroes the month's
/// points (`manager_record::MONTH_POINTS`, `manager_record::MONTH_LEAGUE_POINTS`) of every record
/// (`staff_records_fields::MANAGER_RECORDS`) and stores the new month and year.
///
/// On the first day of the year (day +0 = 0) passes the year's points (`manager_record::YEAR_POINTS`) of each record
/// with a person (`manager_record::OWNER`) to the hall of fame (`exe::halloffame_add_calendar_year_entry` on
/// `G_HALL_OF_FAME`) and zeroes them.
/// Original: `Staff_Records::daily_update` (0x86dda0, staff_records.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn staff_records_daily_update(this: usize, data: usize) {
    if date_month(data) as i32 != read_i16(this + staff_records_fields::CURRENT_MONTH) as u16 as i32 || read_i16(data + 2) != read_i16(this + staff_records_fields::CURRENT_YEAR) {
        covered!("cpo_3 month rollover");
        let base = read_usize(this + staff_records_fields::MANAGER_RECORDS);
        let n = manager_record_count();
        for k in 0..n.max(0) as usize {
            let e = base.wrapping_add(k * MANAGER_RECORD_SIZE);
            write_usize(e + manager_record::MONTH_POINTS, 0);
            write_usize(e + manager_record::MONTH_LEAGUE_POINTS, 0); // the month’s points
        }
        let m = date_month(data);
        write_i16(this + staff_records_fields::CURRENT_MONTH, m);
        write_i16(this + staff_records_fields::CURRENT_YEAR, read_i16(data + 2));
    }
    if read_i16(data) == 0 {
        let base = read_usize(this + staff_records_fields::MANAGER_RECORDS);
        let mut k = 0i32;
        while k < manager_record_count() {
            let e = base.wrapping_add((k as usize).wrapping_mul(MANAGER_RECORD_SIZE));
            let id = read_usize(e + manager_record::OWNER);
            if id as i32 != -1 {
                exe::halloffame_add_calendar_year_entry(read_usize(G_HALL_OF_FAME), staff_records_staff_by_id(id), read_usize(e + manager_record::YEAR_POINTS));
            }
            write_usize(e + manager_record::YEAR_POINTS, 0);
            k += 1;
        }
    }
}

/// `Staff_Records::end_season_for_nation(entry)`: at the end of the competition's season passes its managers' league
/// and cup points to the hall of fame and clears them.
///
/// With the entry's competition (+0, id at its +0; no competition −1; no competition and +4 not equal to
/// `KEY_CONTINENT_EUROPE`, nothing), for each record (`staff_records_fields::MANAGER_RECORDS`): league (+0xc2) of the
/// competition: adds the cup (+0xbe) if it also belongs to that competition (+0xc6 = −1, +0xbe = 0), passes the points
/// (`manager_record::SEASON_POINTS`) to the hall of fame (`exe::halloffame_add_league_entry` on `G_HALL_OF_FAME`) if
/// the competition keeps them (`.league_selected()`), zeroes them and +0xc2 = −1; only the competition's cup: passes
/// +0xbe, zeroes it and +0xc6 = −1.
/// Original: `Staff_Records::end_season_for_nation` (0x86d6e0, staff_records.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn staff_records_end_season_for_nation(this: usize, entry: usize) {
    let p = read_usize(entry + 4);
    let pela_global = p != 0 && read_usize(p) == read_usize(KEY_CONTINENT_EUROPE);
    if !pela_global && read_usize(entry) == 0 {
        return;
    }
    let comp = read_usize(entry);
    let id = if comp != 0 { read_i32(comp) } else { -1 };
    let guarda = |c: Nation| !c.is_null() && c.league_selected() != 0;
    let base = read_usize(this + staff_records_fields::MANAGER_RECORDS);
    let mut k = 0i32;
    while k < manager_record_count() {
        let s = base.wrapping_add((k as usize).wrapping_mul(MANAGER_RECORD_SIZE)) + manager_record::SEASON_POINTS;
        let cup = read_i32(s + 0xc);
        if read_i32(s + 8) == id {
            covered!("cpo_3 end of the league");
            if cup == id {
                write_usize(s, read_usize(s).wrapping_add(read_usize(s + 4)));
                write_usize(s + 0xc, usize::MAX);
                write_usize(s + 4, 0);
            }
            if guarda(Nation(read_usize(entry))) {
                exe::halloffame_add_league_entry(read_usize(G_HALL_OF_FAME), staff_records_staff_by_id(read_usize(s - 0x34)), id, read_usize(s));
            }
            write_usize(s, 0);
            write_usize(s + 8, usize::MAX);
        } else if cup == id {
            if guarda(Nation(read_usize(entry))) {
                exe::halloffame_add_league_entry(read_usize(G_HALL_OF_FAME), staff_records_staff_by_id(read_usize(s - 0x34)), id, read_usize(s + 4));
            }
            write_usize(s + 4, 0);
            write_usize(s + 0xc, usize::MAX);
        }
        k += 1;
    }
}

/// `Staff_Records::find_manager_of_the_month(index)`: the league manager with the most points this month.
///
/// In the league (`G_COMP_OBJECT_TABLE` by the index; `.comp_type()` must be `comp_type::LEAGUE`, otherwise the "Error"
/// box and 0), among the clubs of the table (`.table()`, `league_table_row::SIZE` bytes, count `.club_count()`) with a
/// manager (`.manager()`) at the club (`.club_job()`) since two weeks ago (`exe::date_sub_weeks` of the game date; year
/// `.date_joined_club_year()` unsigned, day `.date_joined_club()` signed), the one with the most points in the month
/// (`.month_league_points()` of the record `exe::staff_records_get_manager_record`, created if missing), only above 0.
/// Returns the manager or 0.
/// Original: `Staff_Records::find_manager_of_the_month` (0x86f950, staff_records.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn find_manager_of_the_month(this: usize, index: i32) -> usize {
    let league = League(read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((index as usize).wrapping_mul(4))));
    if league.comp_type() != comp_type::LEAGUE {
        show_error_at(0xa83b78, 0x104e);
        return 0;
    }
    let mut best_points = 0i32;
    let mut best = 0usize;
    let mut k = 0i32;
    while k < league.club_count() as i32 {
        let club = Club(read_usize(league.table().wrapping_add((k as usize).wrapping_mul(league_table_row::SIZE))));
        let manager = club.manager();
        if !manager.is_null() && !manager.club_job().is_null() {
            let mut d = [0u32; 2];
            let q = exe::date_sub_weeks(G_CURRENT_DATE, (d.as_mut_ptr()) as *mut [u8; 8], (2) as u32); // two weeks ago
            let lo = read_usize(q) as u32;
            let year = (lo >> 16) as u16;
            let year_manager = (manager.date_joined_club_year() as i16) as u16;
            let recente = year < year_manager || (year == year_manager && (lo as u16 as i16) < (manager.date_joined_club() as i16));
            if recente {
                covered!("cpo_3 recent manager");
            } else {
                let r = ManagerRecord(exe::staff_records_get_manager_record(this, manager, 1));
                if !r.is_null() {
                    let points = r.month_league_points() as i32;
                    if points > best_points { // the best of the month
                        covered!("cpo_3 best of the month");
                        best_points = points;
                        best = club.manager().addr();
                    }
                }
            }
        }
        k += 1;
    }
    best
}

/// `Staff_Records::get_manager_of_year_candidates(clubs, n)` (called by 0x768940; SEH frame without an object with a
/// destructor): sorts the clubs and keeps up to three.
///
/// The qsort of the `n` clubs (`staff_records_compare_clubs_by_manager_points`) and, in order, up to three whose
/// manager (`club::MANAGER`) arrived (`.date_joined_club()`) before today − 28 weeks (`FN_DATE_SUB_WEEKS`; year
/// unsigned, day signed) and has a record (`exe::staff_records_get_manager_record` with 1) with games (`.games()`),
/// copied to the start of the vector itself. Returns how many.
/// Original: `Staff_Records::get_manager_of_year_candidates` (0x86fab0, staff_records.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn get_manager_of_year_candidates(this: usize, clubs: usize, n: u32) -> u8 {
    let weeks: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> usize = core::mem::transmute(FN_DATE_SUB_WEEKS);
    let mut lim = [0u8; 8];
    weeks(G_CURRENT_DATE, lim.as_mut_ptr(), 0x1c);
    let n = n as u16 as i16 as i32;
    exe::crt_qsort(clubs, n as u32, 4, 0x870220);
    let (ld, la) = (i16::from_le_bytes([lim[0], lim[1]]), u16::from_le_bytes([lim[2], lim[3]]));
    let mut k = 0u8;
    for i in 0..n.max(0) as usize {
        if k as i8 >= 3 {
            break;
        }
        let manager = Staff(read_usize(read_usize(clubs + 4 * i) + club::MANAGER));
        if manager.is_null() {
            continue;
        }
        let ta = manager.date_joined_club_year();
        if !(la > ta || (la == ta && ld > (manager.date_joined_club() as i16))) {
            continue;
        }
        let r = ManagerRecord(exe::staff_records_get_manager_record(this, manager, 1));
        if !r.is_null() && r.games() != 0 {
            write_u32(clubs + 4 * k as usize, read_usize(clubs + 4 * i) as u32);
            k += 1;
        }
    }
    k
}

/// `Staff_Records::get_manager_record(person, create)`: finds (or creates, or takes over) the manager's 210-byte record
/// (vector `staff_records_fields::MANAGER_RECORDS`).
///
/// Null person: the error box at line 0xf6e and 0. Free slot (`manager_record::OWNER` = −1): without `create`, 0; with
/// it, the slot takes the id. Someone else's slot: zeroes the record of that person (−1 in +0x86, +0xc2, +0xc6, +0x4a,
/// +0x5e, +0x72) and looks again.
///
/// Updates the nations of the club (`.club_job()`) and of the national team (`.national_job()`) in
/// `manager_record::SEASON_NATION`/`manager_record::NATIONAL_NATION`, zeroing the points
/// (`manager_record::SEASON_POINTS`/`manager_record::NATIONAL_SEASON_POINTS`) when they change. With the date of
/// `G_GAME_START_DATE` up to today: no spell (`manager_record::CURRENT_CLUB` = −1), opens the current club's one (since
/// the person's `staff::DATE_JOINED_CLUB`; `manager_record::CURRENT_CLUB_END` = the date 13/11/1975 from
/// `exe::date_date_549f70`); the longest (`manager_record::LONGEST_CLUB`,
/// `manager_record::LONGEST_START`..`manager_record::LONGEST_END`) and the shortest (`manager_record::SHORTEST_CLUB`,
/// `manager_record::SHORTEST_START`..`manager_record::SHORTEST_END`) through `exe::date_days_since`, today's in
/// `manager_record::LONGEST_END` / `manager_record::SHORTEST_END`. Returns the record.
/// Original: `Staff_Records::get_manager_record` (0x86f5b0, staff_records.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn get_manager_record(this: usize, person: usize, create: u32) -> usize {
    let person = Staff(person);
    if person.is_null() {
        let version: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
        let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
        let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
        exe::crt_strtok(0xa83b78, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
        let v = version(nome.as_mut_ptr(), ext.as_mut_ptr(), 0xf6e);
        format(0xae24d0, 0x9870e8, v, nome.as_ptr(), ext.as_ptr(), 0xf6eu32);
        exe::gui_show_message_box(0x9870e0, 0xae24d0, 0);
        write_u32(G_MOUSE_LEFT_BUTTON_DOWN, 0);
        return 0;
    }
    let r = loop {
        let Some(r) = manager_manager_manager_record(this, person) else {
            return 0;
        };
        let owner = read_u32(r + manager_record::OWNER) as i32;
        if owner == -1 {
            if create == 0 {
                return 0;
            }
            covered!("cpp_3 create record");
            write_u32(r + manager_record::OWNER, read_u32(person.addr())); // the slot stays with him
            break r;
        }
        if owner == read_u32(person.addr()) as i32 {
            break r;
        }
        // the slot belongs to someone else: zero his profile record and look again
        let z = (read_u32(this + staff_records_fields::MANAGER_RECORDS) as usize).wrapping_add((read_u32((person.non_player().addr() as u32) as usize) as usize).wrapping_mul(0xd2));
        core::ptr::write_bytes(z as *mut u8, 0, 0xd2);
        for o in [0x86, 0xc2, 0xc6, 0x4a, 0x5e, 0x72] {
            write_u32(z + o, u32::MAX);
        }
    };
    let club = (person.club_job().addr() as u32) as usize;
    if club != 0 {
        let nation = read_u32(club + club::NATION) as usize;
        if nation != 0 && read_u32(nation) != read_u32(r + manager_record::SEASON_NATION) {
            write_u32(r + manager_record::SEASON_NATION, read_u32(nation));
            write_u32(r + manager_record::SEASON_POINTS, 0);
        }
    }
    let national_team = Club((person.national_job().addr() as u32) as usize);
    if !national_team.is_null() {
        let nation = (national_team.nation().addr() as u32) as usize;
        if nation != 0 && read_u32(nation) != read_u32(r + manager_record::NATIONAL_NATION) {
            write_u32(r + manager_record::NATIONAL_NATION, read_u32(nation));
            write_u32(r + manager_record::NATIONAL_SEASON_POINTS, 0);
        }
    }
    let (dia, ano) = (read_i16(G_GAME_START_DATE), read_u16(G_GAME_START_DATE_YEAR));
    let today_year = read_u16(G_CURRENT_DATE_YEAR);
    if ano > today_year || (ano == today_year && dia > read_i16(G_CURRENT_DATE)) {
        return r; // the date in [0xdd6ec8] has not come yet
    }
    if read_u32(r + manager_record::CURRENT_CLUB) as i32 == -1 {
        let club = (person.club_job().addr() as u32) as usize;
        if club != 0 {
            covered!("cpp_3 open spell");
            write_u32(r + manager_record::CURRENT_CLUB, read_u32(club));
            contract_manager_assign_date(r + manager_record::CURRENT_CLUB_SINCE, person.addr() + staff::DATE_JOINED_CLUB);
            let mut local = [0u8; 8];
            let p = exe::date_date_549f70((&mut local) as *mut [u8; 8] as *mut [u16; 4], 0xd, 0xb, 0x7b7, (-1) as i32 as u32);
            contract_manager_assign_date(r + manager_record::CURRENT_CLUB_END, p);
        }
    }
    if read_u32(r + manager_record::LONGEST_CLUB) as i32 == -1 {
        write_u32(r + manager_record::LONGEST_CLUB, read_u32(r + manager_record::CURRENT_CLUB));
        contract_manager_assign_date(r + manager_record::LONGEST_START, r + manager_record::CURRENT_CLUB_SINCE);
        contract_manager_assign_date(r + manager_record::LONGEST_END, G_CURRENT_DATE);
    } else if read_u32(r + manager_record::CURRENT_CLUB) as i32 != -1 {
        let a = exe::date_days_since(r + manager_record::LONGEST_END, read_u32(r + manager_record::LONGEST_START), read_u32(r + manager_record::LONGEST_START_HIGH));
        let b = exe::date_days_since(G_CURRENT_DATE, read_u32(r + manager_record::CURRENT_CLUB_SINCE), read_u32(r + manager_record::CURRENT_CLUB_SINCE_HIGH));
        if b > a {
            write_u32(r + manager_record::LONGEST_CLUB, read_u32(r + manager_record::CURRENT_CLUB)); // a mais longa
            contract_manager_assign_date(r + manager_record::LONGEST_START, r + manager_record::CURRENT_CLUB_SINCE);
            contract_manager_assign_date(r + manager_record::LONGEST_END, G_CURRENT_DATE);
        }
    }
    if read_u32(r + manager_record::SHORTEST_CLUB) as i32 == -1 {
        write_u32(r + manager_record::SHORTEST_CLUB, read_u32(r + manager_record::CURRENT_CLUB));
        contract_manager_assign_date(r + manager_record::SHORTEST_START, r + manager_record::CURRENT_CLUB_SINCE);
        contract_manager_assign_date(r + manager_record::SHORTEST_END, G_CURRENT_DATE);
        return r;
    }
    let current = read_u32(r + manager_record::CURRENT_CLUB);
    if read_u32(r + manager_record::SHORTEST_CLUB) == current && read_i16(r + manager_record::CURRENT_CLUB_SINCE_YEAR) == read_i16(r + manager_record::SHORTEST_START_YEAR) && read_i16(r + manager_record::CURRENT_CLUB_SINCE) == read_i16(r + manager_record::SHORTEST_START) {
        contract_manager_assign_date(r + manager_record::SHORTEST_END, G_CURRENT_DATE); // the shortest is the current one: runs until today
        return r;
    }
    if current as i32 == -1 {
        return r;
    }
    let a = exe::date_days_since(r + manager_record::SHORTEST_END, read_u32(r + manager_record::SHORTEST_START), read_u32(r + manager_record::SHORTEST_START_HIGH));
    let b = exe::date_days_since(G_CURRENT_DATE, read_u32(r + manager_record::CURRENT_CLUB_SINCE), read_u32(r + manager_record::CURRENT_CLUB_SINCE_HIGH));
    if b < a {
        write_u32(r + manager_record::SHORTEST_CLUB, read_u32(r + manager_record::CURRENT_CLUB)); // a mais curta
        contract_manager_assign_date(r + manager_record::SHORTEST_START, r + manager_record::CURRENT_CLUB_SINCE);
        contract_manager_assign_date(r + manager_record::SHORTEST_END, G_CURRENT_DATE);
    }
    r
}

/// `Staff_Records::add_free_release_records(person)`: the free release histories. SEH frame only for the local date,
/// with an empty destructor.
///
/// The person's contract (`exe::contract_manager_get_contract`) goes to `G_CONTRACT_LOOKUP_RESULT` and gives the club
/// (`club::SIZE`-byte record of `G_CLUB_TABLE` through `.club()`). With a manager at the club (`.manager()`), his
/// history (type 0x68: the club, the person and her three ids +0x33 of +4, +8 and +0xc); then the person's (type 0x67:
/// the club or −1), in the archive (0x58e340). Null person: the "Error" box (line 0x1010).
/// Original: `Staff_Records::add_free_release_records` (0x86d800, staff_records.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn add_free_release_records(this: usize, person: usize) {
    covered!("cpr_9 released");
    let mut e = history_entry();
    let c = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person));
    write_usize(G_CONTRACT_LOOKUP_RESULT, c.addr());
    let club = Club(if !c.is_null() { read_usize(G_CLUB_TABLE).wrapping_add(c.club().wrapping_mul(club::SIZE)) } else { 0 });
    if !club.is_null() && club.manager().addr() != 0 {
        stamp_game_date(e.as_mut_ptr());
        e[0xc] = 0x68;
        put_i32(&mut e, 8, read_i32(club.addr()));
        put_person_keys(&mut e, 0xd, person);
        append_person_history_by_id(true, this, read_usize(club.manager().addr()), &e);
    }
    stamp_game_date(e.as_mut_ptr());
    e[0xc] = 0x67;
    put_i32(&mut e, 8, staff_records_id_or_minus_one(club.addr()));
    if person != 0 {
        append_person_history_by_id(true, this, read_usize(person), &e);
    } else {
        show_error_at(0xa83b78, 0x1010);
    }
}

/// `Staff_Records::add_loan_records(person, of, club)` (called by `handle_player_accepted_terms`, the accepted
/// transfer): the loan histories. SEH frame only for the local date, with an empty destructor.
///
/// With the club and its manager (`.manager()`), the manager's history (type 0x69: the club, the person and her three
/// ids +0x33 of +4, +8 and +0xc, and the id of `of`); then, in the same entry, the person's (type 0x6a: the club and
/// the id of `of`), in the archive (0x58e340). The club is read before the person check, as in the original; null
/// person: the "Error" box (line 0x1010).
/// Original: `Staff_Records::add_loan_records` (0x86da20, staff_records.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn add_loan_records(this: usize, person: usize, of: usize, club: usize) {
    let club = Club(club);
    covered!("cps_9 loan");
    let mut e = history_entry();
    if !club.is_null() && club.manager().addr() != 0 {
        let manager = club.manager().addr();
        stamp_game_date(e.as_mut_ptr());
        e[0xc] = 0x69;
        put_i32(&mut e, 8, read_i32(club.addr()));
        put_person_keys(&mut e, 0xd, person);
        put_i32(&mut e, 0x1d, read_i32(of));
        append_person_history_by_id(true, this, read_usize(manager), &e);
    }
    stamp_game_date(e.as_mut_ptr());
    e[0xc] = 0x6a;
    put_i32(&mut e, 8, read_i32(club.addr()));
    put_i32(&mut e, 0xd, read_i32(of));
    if person != 0 {
        append_person_history_by_id(true, this, read_usize(person), &e);
    } else {
        show_error_at(0xa83b78, 0x1010);
    }
}

/// `Staff_Records::rewind_staff_history(person)`: the person's history list (the 8-byte pair at
/// [`staff_records_fields::PERSON_HISTORY_LISTS`] + 8 × id) drops the last entry (`exe::file_llist_reset_last_offset`
/// on [this]: the second offset, +4 of the pair, = −1).
///
/// Null person: the original (fatal box: the game ends).
/// Original: `Staff_Records::rewind_staff_history` (0x86fb80, staff_records.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn rewind_staff_history(this: usize, person: usize) {
    if person == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_REWIND_STAFF_HISTORY)(this, person); // fatal box: original only
    }
    covered!("cpk_7 restart history");
    let par = read_usize(this + staff_records_fields::PERSON_HISTORY_LISTS).wrapping_add(read_usize(person).wrapping_mul(8));
    exe::file_llist_reset_last_offset(read_usize(this), par, par + 4); // the last offset
}

/// `Staff_Records::add_year_award_record(p, club, prize, a4, a5, credito)`: records a year award. SEH frame without a
/// destructor. Called by the recording of the year award winners (YEAR_AWARD::record_winners 0x9302f0: the first, the
/// second and the third).
///
/// Entry 0x34 of the person's history (game date, club or −1, the award id, a4 in the word, a5 and the credit as signed
/// bytes; 0x58e340). Manager (`.job_for_club()` or `.job_for_nation()` = 5 or 0xc): +1 in the word
/// `manager_record::AWARDS` of his record (`get_manager_record`).
///
/// A player with a player sheet (`.player()`) and a contract (`exe::contract_manager_get_contract`, other than type 5)
/// with the club (`exe::contract_manager_get_contract_for_club`): with the transfer manager, a contract of more than
/// 300 days and the player on the club's list (`exe::shortlist_manager_find_club_shortlist_entry`), the renewal
/// (`contract_manager_offer_contract`); then, only with credit 1, without the contract's `.grievances()` flag, with
/// `.loyalty()` above 15 and `.professionalism()` above 10, if draw(`.loyalty()`) is above 5 (a draw by loyalty): the
/// news of thanks to the manager (`exe::news_post_player_credits_manager_for_award`). No person or no award: the
/// original ("Error" box).
/// Original: `Staff_Records::add_year_award_record` (0x86c390, staff_records.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn add_year_award_record(this: usize, p: usize, club: usize, prize: usize, a4: u32, a5: u32, credito: u32) {
    if p == 0 || prize == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32)>(IDX_ADD_YEAR_AWARD_RECORD)(this, p, club, prize, a4, a5, credito);
    }
    add_year_award(this, Staff(p), club, prize, a4, a5, credito, true)
}

/// `Staff_Records::add_first_cap_record(person, opponent)` (called by the international result
/// `record_international_result`): the first cap history. SEH frame only for the local date, with an empty destructor.
///
/// The person's history (type 6: the contracting club `.club_job()` or −1, the opponent's id in the match and the age
/// `.age()`, signed), in the archive (0x58e340). The club is read before the person check, as in the original; null
/// person: the "Error" box (line 0x1010).
/// Original: `Staff_Records::add_first_cap_record` (0x86cc50, staff_records.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn add_first_cap_record(this: usize, person: usize, opponent: usize) {
    let person = Staff(person);
    covered!("cps_8 first call-up");
    let mut e = history_entry();
    stamp_game_date(e.as_mut_ptr());
    e[0xc] = 6;
    put_i32(&mut e, 8, club_job_id(person));
    put_i32(&mut e, 0xd, read_i32(opponent));
    put_i32(&mut e, 0x11, (person.age() as i8) as i32);
    if !person.is_null() {
        append_person_history_by_id(true, this, read_usize(person.addr()), &e);
    } else {
        show_error_at(0xa83b78, 0x1010);
    }
}

/// `Staff_Records::add_month_award_record(p, club, prize, a4, a5)`: writes the winner's 0x33 history entry. SEH frame
/// without a destructor.
///
/// Entry 0x33 of the person's history (game date, club or −1, the award id, a4 in the byte, a5 in the word; 0x58e340).
/// Manager (`.job_for_club()` or `.job_for_nation()` = 5 or 0xc): +1 in the word `manager_record::AWARDS` of his record
/// (`get_manager_record`).
///
/// A player with a player sheet (`.player()`) and a contract (`exe::contract_manager_get_contract`, other than type 5)
/// with the club (`exe::contract_manager_get_contract_for_club`): with the transfer manager, a contract of more than
/// 300 days and the player on the club's list (`exe::shortlist_manager_find_club_shortlist_entry`), the renewal
/// (`contract_manager_offer_contract`); then, without the contract's `.grievances()` flag, with `.loyalty()` above 15
/// and `.professionalism()` above 10, if draw(`.loyalty()`) is above 5: the news of thanks to the manager
/// (`exe::news_post_player_credits_manager_for_award`). No person or no award: the original ("Error" box).
/// Original: `Staff_Records::add_month_award_record` (0x86c8c0, staff_records.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn add_month_award_record(this: usize, p: usize, club: usize, prize: usize, a4: u32, a5: u32) {
    if p == 0 || prize == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32)>(IDX_ADD_MONTH_AWARD_RECORD)(this, p, club, prize, a4, a5);
    }
    add_month_award_record_body(this, Staff(p), club, prize, a4, a5, true)
}

/// clears the manager record and sets the owner, league and club ids to −1
/// Original: `Staff_Records::clear_manager_record` (0x86fc20, staff_records.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn clear_manager_record(records: usize, id: *const i32) {
    let r = friendly_manager_record(records, id);
    core::ptr::write_bytes(r as *mut u8, 0, 210);
    for o in [0x86, 0xc2, 0xc6, 0x4a, 0x5e, 0x72] {
        write_i32(r + o, -1);
    }
}

