use super::*;

/// Day/year is after the date at `data` (unsigned year +2, signed day +0).
pub(crate) unsafe fn scout_manager_date_after(day: i16, year: u16, data: usize) -> bool {
    let (d, a) = (read_i16(data), read_i16(data + cm_date::YEAR) as u16);
    year > a || (year == a && day > d)
}

/// The dry run of `0x7f8750`: the copy only prunes the requests (in its own vector) and stops before the reports.
pub(crate) static SCOUT_DAILY_DRY_RUN: AtomicBool = AtomicBool::new(false);

/// Prunes a scouting request (0x11 bytes: +0 the club, +4 the player, +8 the date, +0x10 how many reports); returns
/// (remove, process).
///
/// A request is removed if the club is not human, if the player belongs to the club itself (+0x39 or the contract's),
/// if it is more than a year old, if it has 3 reports, or (after 30 days) if the player is injured (0x62c400 = 2).
///
/// Between 30 days and a year the request is processed (and removed), except for a player without a nation or from a
/// club that plays in the next 14 days, which stays.
pub(crate) unsafe fn judge_scout_request(request: usize, catorze: usize) -> (bool, bool, usize, usize, usize) {
    // 0x600a50: 5 pushes (club, 1, 1, 0, 0); add esp, 0x14 -> cdecl; 0x4cf5a0: push player; mov ecx, [0xae1050] ->
    // thiscall, ret 4; 0x54c5b0: push n; push &output; lea ecx, [pedido+8] -> thiscall, ret 8; 0x62c400: push player;
    // mov ecx, [0xb63d28] -> thiscall, ret 4 (al); 0x4cf5f0: push club; push player; mov ecx, [0xae1050] -> thiscall,
    // ret 8; 0x5acbe0: 9 pushes; mov ecx, [0xae2a58] -> thiscall, ret 0x24 (ax)
    let player = Staff((staff_records_staff_by_id(read_usize(request + scout_request::PLAYER_ID))).addr());
    let club = friendly_club_by_id(read_usize(request));
    if exe::human_manager_is_human_club(club, 1, 1, 0, 0) == 0 {
        return (true, false, 0, 0, 0);
    }
    let manager = club.manager().addr();
    if player.club_job() == club {
        return (true, false, 0, 0, 0);
    }
    let c = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), player));
    write_usize(G_CONTRACT_LOOKUP_RESULT, c.addr());
    let of_contract = if !c.is_null() { friendly_club_by_id(c.club()) } else { Club(0) };
    if of_contract == club {
        return (true, false, 0, 0, 0);
    }
    let (dia, ano) = (read_i16(G_CURRENT_DATE), read_i16(G_CURRENT_DATE_YEAR) as u16);
    let mut t = [0u32; 2];
    let year_after = exe::date_add_days(request + scout_request::DATE, t.as_mut_ptr() as usize, 0x16d); // cqf12: um ano
    if scout_manager_date_after(dia, ano, year_after) {
        return (true, false, 0, 0, 0);
    }
    if (read_u8(request + scout_request::REPORT_COUNT) as i8) >= 3 { // cqf12: three reports
        return (true, false, 0, 0, 0);
    }
    let mut t2 = [0u32; 2];
    let month_after = exe::date_add_days(request + scout_request::DATE, t2.as_mut_ptr() as usize, 0x1e); // cqf12: trinta dias
    if !scout_manager_date_after(dia, ano, month_after) {
        return (false, false, 0, 0, 0);
    }
    if exe::injury_get_player_injury_status(read_usize(G_INJURY), player) == 2 {
        return (true, false, 0, 0, 0);
    }
    let seu = player.club_job();
    let mut nation = if !seu.is_null() { seu.nation().addr() } else { 0 };
    if nation == 0 {
        nation = player.nation().addr();
        if nation == 0 {
            return (false, false, 0, 0, 0);
        }
    }
    let mut direct = false;
    if !seu.is_null() {
        let k = Contract(exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), player, seu));
        if !k.is_null() {
            let s = k.squad_status() & 0xf0;
            direct = s == 0x60 || s == 0x70 || s == 0;
        }
    }
    if !direct && !seu.is_null() {
        // does the player’s club play from today to 14 days later? (no list; the high part of the 2nd dword is stack leftover in the
        // original, and 0x5acbe0 only reads the low word)
        let last = read_usize(catorze) as u32;
        let last_high = read_u32(catorze + 2);
        let a1 = (last_high & 0xffff_0000) | dia as u16 as u32;
        let a2 = ano as u32;
        if ((exe::fixture_manager_find_fixtures(read_usize(G_FIXTURE_MANAGER), a1, a2, last, last_high, read_usize(seu.addr()) as u32, 0, (-1) as i32 as u32, (-2) as i32 as u32, 1) as u32) as i16) != 0 {
            return (false, false, 0, 0, 0);
        }
    }
    (true, true, player.addr(), manager, nation)
}

/// The scout manager's daily update, on day slice 2: prunes the scouting requests (non-human club, player already at
/// the club, older than a year, 3 reports, injured); a request older than 30 days becomes up to 3 reports from the
/// manager's scouts covering the player's nation and leaves; then the reports, the request order and one search tick on
/// every human manager list.
///
/// Ret 0; SEH frame for the local list and the dates: `SCOUT_MANAGER::daily_update()`, from the day (only on slice 2 of
/// the day, `G_CURRENT_DAY_SLICE`; otherwise 0). Prunes the scouting requests (`scout_manager_fields::SCOUT_REQUESTS`,
/// count `scout_manager_fields::SCOUT_REQUEST_COUNT`, 0x11 bytes; the removed one gets the last one), and the processed
/// request becomes up to 3 reports from the scouts of the club's manager (of the 10 lists,
/// `exe::scout_manager_get_search`, the one that covers the player's nation, `exe::player_search_scout_covers_nation`):
/// `exe::player_search_report_on_player` with −1000, mode 1, and the record (0x67 bytes) goes into the local list if
/// byte `scout_report::REPORT_MODE` ended ≥ 0. Then the reports (`exe::scout_manager_send_scout_reports`), the request
/// order (`exe::crt_qsort` with 0x7fc270) and one search (`exe::player_search_run_scout_day`) on each of the 10 lists
/// of the human managers (the last 16 persons with `.classification()` = 1). Returns 1.
/// Original: `SCOUT_MANAGER::daily_update` (0x7f8750, scout_manager.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn scout_manager_daily_update(this: usize) -> u32 {
    // 0x68d020 / 0x68d040 / 0x54a110: lea ecx, local -> thiscall; 0x54c5b0: push 0xe; push &output; mov ecx, 0xae2c90
    let mut d14 = [0u32; 2];
    let catorze = d14.as_mut_ptr() as usize;
    exe::date_add_days(G_CURRENT_DATE, catorze, 0xe);
    // the report record (0x67 bytes) with the dates the original’s entry builds (+0, +0x2f, +0x47)
    let mut record = [0u8; 0x68];
    let reg = record.as_mut_ptr() as usize;
    exe::date_date_54a110(reg);
    exe::date_date_54a110(reg + scout_report::PLAYER_NAME_REF + 0x10);
    exe::date_date_54a110(reg + scout_report::SCOUT_NAME_REF + 0x10);
    let mut list = [0u8; 16];
    let l = list.as_mut_ptr() as usize;
    exe::new_llist(l);
    if read_u8(G_CURRENT_DAY_SLICE) != 2 {
        exe::drop_llist(l);
        return 0;
    }
    // 0x7f6cd0: push manager; push k; mov ecx, this -> thiscall, ret 8; 0x7b7490: push nation; mov ecx, list ->
    // thiscall, ret 4; 0x7b62a0: push 1; push 0; push 0; push 1; push −1000; push &record; push player; mov ecx, list
    // -> thiscall, ret 0x1c; 0x68d0d0: push 0x67; push &registro; lea ecx, local -> thiscall, ret 8; 0x7f9890: push
    // &local; mov ecx, this -> thiscall, ret 4; 0x945333 (qsort): 4 pushes; add esp, 0x10 -> cdecl; 0x7b3e20: mov ecx,
    // lista -> thiscall, ret 0
    let mut i: i32 = 0;
    while i < read_i32(this + scout_manager_fields::SCOUT_REQUEST_COUNT) {
        let request = read_usize(this + scout_manager_fields::SCOUT_REQUESTS) + i as usize * 0x11;
        let (tirar, processar, jogador, tecnico, nacao) = judge_scout_request(request, catorze);
        if processar && !SCOUT_DAILY_DRY_RUN.load(Relaxed) {
            covered!("cqf_12 request processed");
            let mut feitos: i8 = 0;
            for k in 0..10u32 {
                if feitos >= 3 { // cqf12: up to three reports
                    break;
                }
                let lo = exe::scout_manager_get_search(this, k, tecnico);
                if exe::player_search_scout_covers_nation(lo, nacao) != 0 {
                    exe::player_search_report_on_player(lo, jogador, reg, -1000, 1, 0, 0, 1);
                    feitos += 1;
                    if (read_u8(reg + scout_report::REPORT_MODE) as i8) > -1 {
                        covered!("cqf_12 request report");
                        exe::llist_add_tail(l, reg, 0x67);
                    }
                }
            }
        }
        if tirar {
            let n = read_i32(this + scout_manager_fields::SCOUT_REQUEST_COUNT);
            let base = read_usize(this + scout_manager_fields::SCOUT_REQUESTS);
            core::ptr::copy(base.wrapping_add((n as usize).wrapping_mul(0x11)).wrapping_sub(0x11) as *const u8, request as *mut u8, 0x11);
            write_i32(this + scout_manager_fields::SCOUT_REQUEST_COUNT, n - 1);
            i -= 1;
        }
        i += 1;
    }
    if SCOUT_DAILY_DRY_RUN.load(Relaxed) {
        exe::drop_llist(l);
        return 1;
    }
    exe::scout_manager_send_scout_reports(this, l);
    exe::crt_qsort(read_usize(this + scout_manager_fields::SCOUT_REQUESTS), read_i32(this + scout_manager_fields::SCOUT_REQUEST_COUNT) as u32, 0x11, 0x7fc270);
    for k in 0..0x10i32 {
        let p = Staff((staff_records_staff_by_id((read_i32(G_STAFF_COUNT) - k - 1) as usize)).addr());
        if p.classification() == 1 {
            covered!("cqf_12 human search");
            for j in 0..10u32 {
                exe::player_search_run_scout_day(exe::scout_manager_get_search(this, j, p)); // cqf12: the search of each list
            }
        }
    }
    exe::drop_llist(l);
    1
}

/// The record of the report news that `0x7f9a60` receives (player, club, how many).
///
/// It is enabled only in the comparer of `0x7f9890`, to check the original routine's groups.
pub(crate) static mut SCOUT_REPORT_GROUPS: Option<Vec<(u32, u32, u8)>> = None;

/// Copies the list records into an array, sorts it and sends one news item per run of the same player and club.
///
/// Ret 4: `SCOUT_MANAGER::send_scout_reports(list)`, from the scouts' day (`scout_manager_daily_update`) and from the
/// search (`scout_manager_scout_match`). With the list not empty (+0 count): a vector of count × 0x67
/// (`exe::crt_malloc`; out of memory, the box at line 0x7dd) with copies of the records, sorted (`exe::crt_qsort` with
/// 0x7fc2a0); each consecutive group of the same player (`scout_report::PLAYER_ID`) and the same club
/// (`scout_report::CLUB_ID`) becomes a report news item (`exe::scout_manager_send_scout_report_news` with the first one
/// and how many). Frees the vector (`exe::crt_free`). The list is left with its cursor at the start.
/// Original: `SCOUT_MANAGER::send_scout_reports` (0x7f9890, scout_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn send_scout_reports(this: usize, list: usize) {
    let list = Llist(list);
    let count = read_i16(list.addr());
    if count == 0 {
        return;
    }
    let n = count as i32;
    // 0x944e46 (malloc): push; add esp, 4 -> cdecl; 0x9452ca (free): push; add esp, 4 -> cdecl; 0x68d1b0: mov ecx, lista
    // -> thiscall; 0x945333: 4 pushes; add esp, 0x10 -> cdecl; 0x7f9a60: push quantos; push registro; mov ecx, this ->
    // thiscall, ret 8
    let v = exe::crt_malloc((n.wrapping_mul(0x67)) as u32);
    if v == 0 && n != 0 {
        show_error_at(0xa7b094, 0x7dd);
        return;
    }
    list.set_cursor(list.head());
    let mut dst = v;
    let mut item = exe::llist_get_next(list);
    while item != 0 {
        core::ptr::copy_nonoverlapping(item as *const u8, dst as *mut u8, 0x67);
        dst += 0x67;
        item = exe::llist_get_next(list);
    }
    exe::crt_qsort(v, (n) as u32, 0x67, 0x7fc2a0);
    let mut i: i32 = 0;
    while i < n {
        let r = v + i as usize * 0x67;
        let club_id = read_usize((friendly_club_by_id(read_usize(r + scout_report::CLUB_ID))).addr());
        let player_id = read_usize((staff_records_staff_by_id(read_usize(r + scout_report::PLAYER_ID))).addr());
        let start = i;
        let mut how_many: u8 = 1;
        let mut j = i + 1;
        while j < n {
            let s = v + j as usize * 0x67;
            if read_usize(s + scout_report::CLUB_ID) != club_id || read_usize(s + scout_report::PLAYER_ID) != player_id {
                break;
            }
            how_many = how_many.wrapping_add(1);
            j += 1;
            i += 1;
        }
        let first = v + start as usize * 0x67;
        if SCOUT_DAILY_DRY_RUN.load(Relaxed) {
            if let Some(g) = &mut *core::ptr::addr_of_mut!(SCOUT_REPORT_GROUPS) {
                g.push((read_usize(first + scout_report::PLAYER_ID) as u32, read_usize(first + scout_report::CLUB_ID) as u32, how_many));
            }
        } else {
            exe::scout_manager_send_scout_report_news(this, first, how_many as u32);
        }
        i += 1;
    }
    exe::crt_free(v);
}

/// Sends the 0x138e news item with up to 3 records, the scout, the match and the player's strengths for the club, to
/// the club's manager.
///
/// Ret 8; SEH frame for the local news item: `SCOUT_MANAGER::send_scout_report_news(r, how_many)`, from the reports
/// (`send_scout_reports`). News 0x138e (`exe::new_newsitem`): count (up to 3), `scout_report::REPORT_MODE`, the scout
/// (`exe::write_staff_name_ref_to_news` with `scout_report::PLAYER_NAME_REF`), `scout_report::PLAYER_AGE`,
/// `scout_report::POSITION_MASK`, club `scout_report::CLUB_ID`, the match data (`FN_WRITE_MATCH_RECORD_TO_NEWS`),
/// `scout_report::HAS_MATCH`, `scout_report::LEAGUE_MATCH`, `scout_report::ELIGIBILITY_NOTE`,
/// `scout_report::WORK_PERMIT_GRANTED`, from each of the records (`scout_report::SCOUT_NAME_REF` + 0x67 × k) +0xc, +0,
/// +4, +8 and +0x26 + 8 × +0x25, 0x420e9 and the player's (`scout_report::PLAYER_ID`) strengths for the club
/// (`exe::staff_get_player_strength_flags`); goes to the club's manager (`exe::news_send_to_club_manager`).
/// Original: `SCOUT_MANAGER::send_scout_report_news` (0x7f9a60, scout_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn send_scout_report_news(this: usize, r: usize, how_many: u32) {
    let _ = this;
    let n = (how_many as u8 as i8).min(3);
    let data: unsafe extern "C" fn(usize, usize, *mut u8) = core::mem::transmute(FN_WRITE_MATCH_RECORD_TO_NEWS);
    // 0x77bbe0: push 0; push 0x138e; lea ecx -> thiscall; 0x785790: push v; push k; lea ecx -> thiscall, ret 8; 0x7e5160
    // and 0x7e4de0: push &index; push &news; push record; add esp, 0xc -> cdecl; 0x546ab0: push club; push
    // player; add esp, 8 -> cdecl; 0x7861e0: push club; push &news; mov ecx, [0xb7431c] -> thiscall, ret 8
    let mut news = [0u32; 0x38];
    let nt = news.as_mut_ptr() as usize;
    exe::new_newsitem(nt, 0x138e, 0);
    exe::newsitem_set_slot(nt, (0) as u32 as i8, n as i32 as u32);
    let mut idx: u8 = 1;
    let pr = |idx: &mut u8, v: u32| {
        exe::newsitem_set_slot(nt, (*idx as u32) as u32 as i8, v);
        *idx = idx.wrapping_add(1);
    };
    pr(&mut idx, read_u8(r + scout_report::REPORT_MODE) as i8 as i32 as u32);
    exe::write_staff_name_ref_to_news(r + scout_report::PLAYER_NAME_REF, nt, &mut idx);
    pr(&mut idx, read_u8(r + scout_report::PLAYER_AGE) as i8 as i32 as u32);
    pr(&mut idx, read_i16(r + scout_report::POSITION_MASK) as i32 as u32);
    pr(&mut idx, read_usize(r + scout_report::CLUB_ID) as u32);
    data(r, nt, &mut idx);
    pr(&mut idx, read_usize(r + scout_report::HAS_MATCH) as u32);
    pr(&mut idx, read_usize(r + scout_report::LEAGUE_MATCH) as u32);
    pr(&mut idx, read_u8(r + scout_report::ELIGIBILITY_NOTE) as i8 as i32 as u32);
    pr(&mut idx, read_usize(r + scout_report::WORK_PERMIT_GRANTED) as u32);
    for k in 0..n.max(0) as usize {
        let p = r + scout_report::SCOUT_NAME_REF + k * 0x67;
        pr(&mut idx, read_usize(p + 0xc) as u32);
        pr(&mut idx, read_usize(p) as u32);
        pr(&mut idx, read_usize(p + 4) as u32);
        pr(&mut idx, read_usize(p + 8) as u32);
        pr(&mut idx, ((read_u8(p + 0x26) as i8 as i32) + (read_u8(p + 0x25) as i8 as i32) * 8) as u32);
    }
    pr(&mut idx, 0x420e9);
    let c = read_i32(r + scout_report::CLUB_ID);
    let club = if c > -1 { friendly_club_by_id(c as usize) } else { Club(0) };
    let k = idx;
    idx = idx.wrapping_add(1);
    let _ = idx;
    exe::newsitem_set_slot(nt, (k as u32) as u32 as i8, exe::staff_get_player_strength_flags(staff_records_staff_by_id(read_usize(r + scout_report::PLAYER_ID)), club));
    exe::news_send_to_club_manager(read_usize(G_NEWS), nt, friendly_club_by_id(read_usize(r + scout_report::CLUB_ID)));
}

/// Date r equals today's (year and day).
pub(crate) unsafe fn is_today(r: usize, today: usize) -> bool {
    read_u16(r + cm_date::YEAR) == read_u16(today + cm_date::YEAR) && read_i16(r) == read_i16(today)
}

/// Marks clubs with fixtures on date d, the day before and the day after, leaving d unchanged.
pub(crate) unsafe fn mark_clubs_three_days(this: usize, d: usize) {
    // 0x7775c0: push &data; mov ecx, edi -> thiscall, ret 4
    let (mut t, mut u) = ([0u8; 8], [0u8; 8]);
    let (pt, pu) = (t.as_mut_ptr() as usize, u.as_mut_ptr() as usize);
    covered!("cqc_12 marks the clubs");
    exe::nationalteams_mark_clubs_with_fixtures(this, d);
    exe::date_operator_3d(d, pu, exe::date_sub_days(d, pt, 1));
    exe::nationalteams_mark_clubs_with_fixtures(this, d);
    exe::date_operator_3d(d, pu, exe::date_add_days(d, pt, 2));
    exe::nationalteams_mark_clubs_with_fixtures(this, d);
    exe::date_operator_3d(d, pu, exe::date_sub_days(d, pt, 1));
}

/// The fixtures of list `l` (of the day two weeks from now): for each one, the national team call-ups (`select_squads_for_fixture`) and,
/// outside finals tournaments, the per-club call-up records.
///
/// Outside finals tournaments means the competition +0x14 is not one of the keys 0x9cf79c, 0x9cf7a0, 0x9cf7a4,
/// 0x9cf7a8, 0x9cf7ac, 0x9cf790, 0x9cf8a0, 0x9cf78c, 0x9cf964; with a home side that is not the B team (`exe::club_is_national_team`) and
/// mark +0x40: the players called up per club (`record_club_call_ups`) for both national teams.
///
/// With bit 0x200 of +0x4d, also those of the B teams (`exe::database_get_national_u21_team` of each one's nation +0x53) and, without it and with
/// 0x100, those of both with `b` = 1.
pub(crate) unsafe fn select_squads_for_fixtures(this: usize, l: Llist) {
    // 0x76bab0: push p; mov ecx, edi -> thiscall ret 4; 0x5409b0 e 0x53b4f0: push; add esp, 4 -> cdecl;
    // 0x7771c0: push p; push b; push time; mov ecx, edi -> thiscall ret 0xc
    l.set_cursor(l.head());
    let mut p = exe::llist_get_next(l);
    while p != 0 {
        covered!("cqc_12 calls up");
        exe::nationalteams_select_squads_for_fixture(this, p); // cqc12: the call-ups for the match
        let comp = read_usize(p + fixture::COMPETITION);
        let final_ = comp != 0
            && [KEY_CLUB_COMP_FIFA_WORLD_CUP, KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_OLYMPIC_GAMES, KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP, KEY_CLUB_COMP_COPA_AMERICA, KEY_CLUB_COMP_AFRICAN_CUP_OF_NATIONS, KEY_CLUB_COMP_ASIAN_CUP, KEY_CLUB_COMP_OCEANIA_NATIONS_CUP, KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_ASIAN_GAMES, KEY_CLUB_COMP_CONFEDERATIONS_CUP]
                .iter().any(|&k| read_usize(comp) == read_usize(k));
        if !final_ && exe::club_is_national_team(read_usize(p + fixture::HOME_CLUB)) == 0 && *((p + fixture::HOME_SIDE_MARK) as *const u8) != 0 {
            exe::nationalteams_record_club_call_ups(this, read_usize(p + fixture::HOME_CLUB), 0, p);
            exe::nationalteams_record_club_call_ups(this, read_usize(p + fixture::AWAY_CLUB), 0, p);
            let f = read_u16(p + fixture::SQUAD_RULES);
            if f & squad_rules::U21_SQUAD_MATCH != 0 {
                let b1 = exe::database_get_national_u21_team(read_usize(read_usize(p + fixture::HOME_CLUB) + club::NATION));
                exe::nationalteams_record_club_call_ups(this, b1, 0, p);
                let b2 = exe::database_get_national_u21_team(read_usize(read_usize(p + fixture::AWAY_CLUB) + club::NATION));
                exe::nationalteams_record_club_call_ups(this, b2, 0, p);
            } else if f & 0x100 != 0 {
                exe::nationalteams_record_club_call_ups(this, read_usize(p + fixture::HOME_CLUB), 1, p);
                exe::nationalteams_record_club_call_ups(this, read_usize(p + fixture::AWAY_CLUB), 1, p);
            }
        }
        p = exe::llist_get_next(l);
    }
}

/// Number of items in the list at [this] whose day (+0x10) equals `dia`, without moving the cursor.
pub(crate) unsafe fn count_items_on_day(this: usize, day: i16) -> u32 {
    let l = Llist(read_usize(this));
    if l.is_null() {
        return 0;
    }
    let mut n = 0;
    let mut in_the = l.head();
    while in_the != 0 {
        if read_i16(read_usize(in_the) + call_up_entry::DUTY_END_DAY) == day {
            n += 1;
        }
        in_the = read_usize(in_the + llist_node::NEXT);
    }
    n
}

/// Index (0..15) of the current human manager; Err without manager, Ok(None) when out of range.
pub(crate) unsafe fn current_human_index() -> Result<Option<usize>, ()> {
    let t = read_usize(G_SCREEN_RECORDS_MANAGER.wrapping_add((read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as i32 as usize).wrapping_mul(0x300)));
    if t == 0 {
        return Err(());
    }
    let i = read_u8(t).wrapping_sub(read_u8(G_STAFF_COUNT)).wrapping_add(0x10) as i8;
    Ok(if (0..=0xf).contains(&i) { Some(i as usize) } else { None })
}

/// Search record k (0x2b0 bytes) of session s.
pub(crate) unsafe fn search_record(s: usize, k: i32) -> usize {
    s.wrapping_add((k as usize).wrapping_mul(0x2b0))
}

/// Tells whether the current search of the current human manager session has no club (+0x191 = −2).
///
/// Ret 0: `SCOUT_MANAGER::current_search_has_no_club()`, from the search screens (0x482a9b). The session of the human
/// manager whose turn it is (`[this + scout_manager_fields::EDIT_SESSIONS + 4 × index]`): the current search (the
/// signed byte at +0 of the session) without a club (+0x191 = −2). No manager or no session: 0; index out of range: the
/// box (line 0x454; the original) and 0.
/// Original: `SCOUT_MANAGER::current_search_has_no_club` (0x7f8420, scout_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn current_search_has_no_club(this: usize) -> u32 {
    let i = match current_human_index() {
        Err(()) => return 0,
        Ok(None) => return original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CURRENT_SEARCH_HAS_NO_CLUB)(this), // a caixa
        Ok(Some(i)) => i,
    };
    let s = read_usize(this + scout_manager_fields::EDIT_SESSIONS + i * 4);
    if s == 0 {
        return 0;
    }
    (read_usize(search_record(s, read_i8(s) as i32) + 0x191) as i32 == -2) as u32
}

/// Tells whether one of the 10 searches of the current human manager session is for the club.
///
/// Ret 4: `SCOUT_MANAGER::is_club_in_session_searches(club)`, from the screens (0x482aaf, 0x483040, 0x7b34e1). Any of
/// the 10 searches of the session of the human manager whose turn it is has the club (+0x191 = +0 of the club): 1. No
/// manager or no session: 0; index out of range: the box (line 0x480; the original) and 0.
/// Original: `SCOUT_MANAGER::is_club_in_session_searches` (0x7f8500, scout_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn is_club_in_session_searches(this: usize, club: usize) -> u32 {
    let i = match current_human_index() {
        Err(()) => return 0,
        Ok(None) => return original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_IS_CLUB_IN_SESSION_SEARCHES)(this, club), // a caixa
        Ok(Some(i)) => i,
    };
    let s = read_usize(this + scout_manager_fields::EDIT_SESSIONS + i * 4);
    if s == 0 {
        return 0;
    }
    let id = read_usize(club);
    // simplification: the original tests the counter against 0xff (never) to use the current search (+0); the counter stays
    (0..10).any(|k| read_usize(search_record(s, k) + 0x191) == id) as u32
}

/// The scouts at the match: for each human manager, the 10 scouts in a randomly drawn order; one for the playing club
/// starts the assignment, and each watches the match; at the end the reports go out.
///
/// Ret 8; SEH frame for the local LLIST `exe::new_llist`/`exe::drop_llist`: `SCOUT_MANAGER::scout_match(p, result)`,
/// from the closing of the day's match (`close_day_match`, Rust). For each of the last 16 person records
/// (`G_STAFF_TABLE`, 0x6e bytes, from the last backwards) with `.classification()` = 1 (the human manager): in a zeroed
/// vector of 10 marks (`exe::crt_calloc`), the 10 scouts in a random order (`exe::utils_random` of 10 until an unmarked
/// one is found); each one's record (`exe::scout_manager_get_search`) with `player_search_fields::ASSIGNED_OPPONENT`,
/// in a fixture with flags `.home_side_mark()` and `.away_side_mark()` and of the manager's club (`.club_job()` =
/// `.home_club()` or `.away_club()` of the fixture), starts the scouting (`exe::player_search_start_scout_assignment`
/// with −1, −1, −1, 0, 1); and watches the match (`exe::player_search_scout_match`, the reports into the local list).
/// The vector is freed (`exe::crt_free`). Finally the reports go out (`exe::scout_manager_send_scout_reports`) and the
/// list is destroyed. Returns 1.
/// Original: `SCOUT_MANAGER::scout_match` (0x7f8c00, scout_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn scout_manager_scout_match(this: usize, p: usize, result: usize) -> u32 {
    let p = Fixture(p);
    // calloc: push 4; push 0xa; add esp, 8 -> cdecl (10, 4); random draw: push 0xa; add esp, 4 -> cdecl; free: cdecl;
    // 0x7f6cd0: push pessoa; push qual; mov ecx, [this] -> thiscall ret 8; 0x7ae9c0: 5 pushes; mov ecx, registro ->
    // thiscall ret 0x14; 0x7b5be0: push &list; push result; push match; mov ecx, record -> thiscall ret 0xc;
    // 0x7f9890: push &lista; mov ecx, this -> thiscall ret 4; 0x68d020/0x68d040: lea ecx, lista -> thiscall
    let mut l = [0u8; 0x10];
    let list = l.as_mut_ptr() as usize;
    exe::new_llist(list);
    for k in 0..16i32 {
        let idx = (read_usize(G_STAFF_COUNT) as i32).wrapping_sub(k).wrapping_sub(1);
        let person = Staff(read_usize(G_STAFF_TABLE).wrapping_add((idx as usize).wrapping_mul(staff::SIZE)));
        if person.classification() != 1 {
            continue;
        }
        covered!("cqc_13 human scouts");
        let marks = exe::crt_calloc(10, 4);
        for _ in 0..10 { // cqc13: os 10 observadores
            let q = loop {
                let q = exe::utils_random(10) as i8; // cqc13: o observador sorteado
                if read_usize(marks.wrapping_add((q as isize * 4) as usize)) == 0 {
                    break q; // cqc13: one not yet marked
                }
            };
            write_u32(marks.wrapping_add((q as isize * 4) as usize), 1);
            // simplification: the original pushes the dword with the observer byte and stack leftover above; 0x7f6cd0 reads the
            // byte
            let r = exe::scout_manager_get_search(this, q as u8 as u32, person);
            if read_usize(r + player_search_fields::ASSIGNED_OPPONENT) != 0 && p.home_side_mark() != 0 && (p.away_side_mark() as u8) != 0 {
                let club = person.club_job();
                if p.home_club() == club || p.away_club() == club {
                    exe::player_search_start_scout_assignment(r, u32::MAX, u32::MAX, u32::MAX, 0, 1);
                }
            }
            exe::player_search_scout_match(r, p, result, list);
        }
        exe::crt_free(marks);
    }
    exe::scout_manager_send_scout_reports(this, list);
    exe::drop_llist(list);
    1
}

/// Index (0..15) of the given manager (null: current human); None when missing or out of range.
pub(crate) unsafe fn scout_manager_human_manager_index(manager: usize) -> Option<usize> {
    let t = if manager != 0 {
        manager
    } else {
        read_usize(G_SCREEN_RECORDS_MANAGER.wrapping_add((read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as i32 as usize).wrapping_mul(0x300)))
    };
    if t == 0 {
        return None;
    }
    let i = read_u8(t).wrapping_sub(read_u8(G_STAFF_COUNT)).wrapping_add(0x10) as i8;
    if (0..=0xf).contains(&i) { Some(i as usize) } else { None }
}

/// Returns the human manager's searches edit session.
///
/// Ret 4: `SCOUT_MANAGER::get_edit_session(manager)`, from building the search's known players (0x7afeb7). The human
/// manager's edit session (`scout_manager_fields::EDIT_SESSIONS` + 4 × index). No manager or index out of range: the
/// box (lines 0xde and 0xe8; the original) and 0.
/// Original: `SCOUT_MANAGER::get_edit_session` (0x7f6be0, scout_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn get_edit_session(this: usize, manager: usize) -> usize {
    let Some(i) = scout_manager_human_manager_index(manager) else {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_GET_EDIT_SESSION)(this, manager); // a caixa
    };
    read_usize(this + scout_manager_fields::EDIT_SESSIONS + i * 4)
}

/// Stores the human manager's searches edit session and returns 1.
///
/// Ret 8: `SCOUT_MANAGER::set_edit_session(sessao, manager)`, from the search screens (0x8085bc, 0x812d88...). Writes
/// the session into `scout_manager_fields::EDIT_SESSIONS` + 4 × the manager's index and returns 1. No manager or index
/// out of range: the box (lines 0xb8 and 0xc2; the original) and 0.
/// Original: `SCOUT_MANAGER::set_edit_session` (0x7f6af0, scout_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn set_edit_session(this: usize, sessao: usize, manager: usize) -> u32 {
    let Some(i) = scout_manager_human_manager_index(manager) else {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_SET_EDIT_SESSION)(this, sessao, manager); // a caixa
    };
    write_usize(this + scout_manager_fields::EDIT_SESSIONS + i * 4, sessao);
    1
}

/// As datas estáticas da `exe::date_add_weeks` (valor e guarda) e da `exe::date_add_days`, 0x54c8e0 e `exe::date_days_since`.
pub(crate) const SCOUT_MANAGER_DATE_STATICS: [(usize, usize); 8] = [(G_ADD_DAYS_RESULT_DATE, 8), (0xae2469, 1), (G_SUB_DAYS_RESULT_DATE, 8), (0xae249e, 1), (G_ADD_WEEKS_RESULT_DATE, 8), (0xae2430, 1), (G_DAYS_SINCE_SCRATCH_DATE, 8), (0xae2418, 1)];

/// Body of Friendly::update_international_friendlies (`update_international_friendlies`); in dry run returns the (first entry, count) that would
/// go to `arrange_international_friendlies`.
pub(crate) unsafe fn update_friendlies_body(this: Friendly, dry_run: bool) -> Option<(usize, u32)> {
    // 0x54a110: thiscall(date) (lea ecx); 0x54c770: thiscall(0xae2c90, output, 0xc) (mov ecx, 0xae2c90; 2 push);
    // 0x68d080: thiscall(list) (mov ecx, list; no push); 0x945333: cdecl(array, count, size, comparison) (add esp,
    // 0x10); 0x5cbdd0: thiscall(entry, year) (mov ecx, entry; 1 push); 0x417c10: thiscall(destination, output, source)
    // (lea ecx, destination; 2 push); 0x946464: cdecl(key, array, count, 0x14, 0x5cc610) (add esp, 0x14); 0x456770 and
    // 0x456740: thiscall(data, baixo, alto) (lea ecx, data; 2 push); 0x5cbdf0: thiscall(this, primeira, quantas) (mov
    // ecx, esi; 2 push)
    // the search key: the date at +3, like the 20-byte entries (the original passes the local date − 3)
    let mut key = [0u8; 0x14];
    let d = key.as_mut_ptr() as usize + 3;
    exe::date_date_54a110(d);
    let mut lim = [0u8; 8];
    exe::date_add_weeks(G_CURRENT_DATE, &mut lim, 0xc); // cpy4: hoje + 12 semanas
    let today = read_u16(G_CURRENT_DATE);
    if today == 0 && !dry_run {
        covered!("cpy_4 year rollover");
        let mut i = 0i32;
        while i < read_usize(G_NATION_COUNT) as i32 {
            let l = read_usize(this.nation_lists().wrapping_add(i as usize * 4));
            if l != 0 {
                exe::llist_clear(l);
            }
            i += 1;
        }
        exe::crt_qsort(this.sorted_nations(), (this.sorted_nation_count() as usize) as u32, 4, 0x5cc5f0); // cpy4: the nations by reputation
        // the first World Cup year (year − 2 a multiple of 4) from this one; the high half of the dword the original adds is
        // stack leftover: here, zero
        let mut year = read_u16(G_CURRENT_DATE_YEAR) as u32;
        while ((year & 0xffff) as i32 - 2) % 4 != 0 {
            year = year.wrapping_add(1);
        }
        let mut k = 0i32;
        while k < (this.intl_date_count() as usize) as i32 {
            let e = this.intl_date_table().wrapping_add(k as usize * 0x14);
            let mut a = ((*((e + intl_date_entry::YEAR_OFFSET) as *const i8)) as i16 as u16 as u32).wrapping_add(year);
            if (a & 0xffff) as i32 - read_u16(G_CURRENT_DATE_YEAR) as i32 > 3 {
                a = a.wrapping_add(0xfffc);
            }
            exe::friendly_international_date_resolve(e, a);
            k += 1;
        }
        exe::crt_qsort(this.intl_date_table(), (this.intl_date_count() as usize) as u32, 0x14, 0x5cc610);
    }
    // the date of 0xdd6ec8 after today + 12 weeks: nothing
    let x_year = read_u16(G_GAME_START_DATE_YEAR);
    let l_year = core::ptr::read_unaligned(lim.as_ptr().add(2) as *const u16);
    if x_year > l_year || (x_year == l_year && *(G_GAME_START_DATE as *const i16) > core::ptr::read_unaligned(lim.as_ptr() as *const i16)) {
        return None;
    }
    if this.intl_last_day_done() as u32 == today as i16 as i32 as u32 {
        return None;
    }
    let mut lixo = [0u8; 8];
    exe::date_operator_3d(d, (&mut lixo) as *mut [u8; 8] as usize, lim.as_ptr() as usize);
    let base = this.intl_date_table();
    let n = this.intl_date_count() as usize;
    let p = exe::crt_bsearch(key.as_ptr() as usize, base, n as u32, 0x14, 0x5cc610);
    if p == 0 {
        return None;
    }
    let same = |e: usize| exe::date_operator_3c(d, read_usize(e + 3) as u32, read_usize(e + 7) as u32) == 0 && exe::date_operator_3e(d, read_usize(e + 3) as u32, read_usize(e + 7) as u32) == 0;
    // goes back to the first entry of the date
    let mut e = p;
    while e > base {
        if !same(e - 0x14) {
            break;
        }
        e -= 0x14;
    }
    let first = e;
    let last = base.wrapping_add(n.wrapping_mul(0x14)).wrapping_sub(0x14);
    let mut how_many = 0u32;
    while e <= last && same(e) {
        how_many += 1;
        e += 0x14;
    }
    if dry_run {
        return Some((first, how_many));
    }
    covered!("cpy_4 friendlies scheduled");
    exe::friendly_arrange_international_friendlies(this, first, how_many); // cpy4: schedule the friendlies of the date
    this.set_intl_last_day_done(read_u16(G_CURRENT_DATE));
    Some((first, how_many))
}

/// Index (id - G_STAFF_COUNT + 16) of the manager (null: current human) among the 16 humans; None on error.
pub(crate) unsafe fn manager_human_index(manager: usize) -> Option<usize> {
    let t = if manager != 0 {
        manager
    } else {
        read_usize(G_SCREEN_RECORDS_MANAGER.wrapping_add((read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as i32 as usize).wrapping_mul(0x300)))
    };
    if t == 0 {
        return None;
    }
    let i = (read_usize(t) as i32).wrapping_sub(read_usize(G_STAFF_COUNT) as i32).wrapping_add(0x10);
    if !(0..=0xf).contains(&i) {
        return None;
    }
    Some(i as usize)
}

/// Returns the list selected by the human manager (null: the current one).
///
/// Ret 4: `SCOUT_MANAGER::get_selected_list(manager)`, from the search screens (0x804248). The list chosen by the human
/// manager (byte at `scout_manager_fields::SELECTED_LISTS` + index). No manager or outside 0..15: the original routine
/// ("Error" box, 0xff).
/// Original: `SCOUT_MANAGER::get_selected_list` (0x7f6e30, scout_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn get_selected_list(this: usize, manager: usize) -> u8 {
    let Some(i) = manager_human_index(manager) else {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_GET_SELECTED_LIST)(this, manager); // message box: original only
    };
    read_u8(this + scout_manager_fields::SELECTED_LISTS + i)
}

/// Stores the list selected by the human manager (null: the current one).
///
/// Ret 8: `SCOUT_MANAGER::set_selected_list(list, manager)`, from the search screens (0x804460). Writes the chosen list
/// (byte) into `scout_manager_fields::SELECTED_LISTS` + the human manager's index and returns 1. No manager or outside
/// 0..15: the original routine ("Error" box, 0).
/// Original: `SCOUT_MANAGER::set_selected_list` (0x7f6f20, scout_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn set_selected_list(this: usize, list: u32, manager: usize) -> u32 {
    let Some(i) = manager_human_index(manager) else {
        return original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> u32>(IDX_SET_SELECTED_LIST)(this, list, manager); // message box: original only
    };
    write_u8(this + scout_manager_fields::SELECTED_LISTS + i, list as u8);
    1
}

/// Tells whether the fixture has a watched club or an available player with a scout request (at the club or national
/// side, not loaned out, injured, suspended or barred, and not yet played).
///
/// Ret 4: `SCOUT_MANAGER::fixture_has_scouted_player(fixture)` (from match_without_human
/// `exe::match_day_can_quick_simulate`): negative competition (+0), 0. A club of the watched list (static LLIST
/// `G_SCOUT_CLUB_TABLE_LIST`, cursor reset; +0 of the item) equal to `.home_club_id()` or `.away_club_id()` of the
/// fixture, 1. Then the scouting requests (`scout_manager_fields::SCOUT_REQUESTS`, 0x11 bytes, the person at +4, count
/// `scout_manager_fields::SCOUT_REQUEST_COUNT`, reread on each step): a person whose club (`.club_job()`) or national
/// side (`.national_job()`) is one of the two in the fixture (`.home_club()` first, then `.away_club()`), who is not
/// loaned out (`exe::transfer_manager_player_needs_work_permit`, outside a national side
/// `exe::database_is_national_team`), nor injured (`exe::injury_get_player_injury_status` = 2), nor suspended
/// (`exe::discipline_is_player_banned`), nor barred (`exe::nationalteams_get_international_duty_club`), and who has not
/// played yet (`exe::transfer_manager_get_ineligibility_code` = 0), gives 1. Otherwise 0.
/// Original: `SCOUT_MANAGER::fixture_has_scouted_player` (0x7f9720, scout_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn fixture_has_scouted_player(this: usize, fixture: usize) -> u32 {
    let fixture = Fixture(fixture);
    let (c1, c2) = (fixture.home_club(), fixture.away_club());
    if read_i32(fixture.addr()) < 0 {
        covered!("cps_8 negative competition");
        return 0; // cps8: negative competition
    }
    write_u32(G_SCOUT_CLUB_TABLE_LIST_CURSOR, read_usize(G_SCOUT_CLUB_TABLE_LIST_HEAD) as u32);
    let mut e = exe::llist_get_next(G_SCOUT_CLUB_TABLE_LIST);
    while e != 0 {
        let v = read_usize(e);
        if fixture.home_club_id() == v || fixture.away_club_id() == v {
            return 1;
        }
        e = exe::llist_get_next(G_SCOUT_CLUB_TABLE_LIST);
    }
    if read_usize(this + scout_manager_fields::SCOUT_REQUESTS) == 0 || read_i32(this + scout_manager_fields::SCOUT_REQUEST_COUNT) <= 0 {
        covered!("cps_8 no requests");
        return 0; // cps8: without scouting requests
    }
    let mut k = 0i32;
    while k < read_i32(this + scout_manager_fields::SCOUT_REQUEST_COUNT) {
        let id = read_i32(read_usize(this + scout_manager_fields::SCOUT_REQUESTS).wrapping_add(k as usize * 0x11) + 4);
        k += 1;
        if id <= -1 {
            continue;
        }
        let p = Staff(read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(staff::SIZE)));
        let (clube, sel) = (p.club_job(), p.national_job());
        let x = if c1 == clube || c1 == sel {
            c1
        } else if c2 == clube || c2 == sel {
            c2
        } else {
            continue;
        };
        if (exe::database_is_national_team(x) as i32) == 0 && exe::transfer_manager_player_needs_work_permit(read_usize(G_TRANSFER_MANAGER), p, x, 0) != 0 {
            continue;
        }
        if exe::injury_get_player_injury_status(read_usize(G_INJURY), p) == 2 {
            continue;
        }
        if exe::discipline_is_player_banned(read_usize(G_DISCIPLINE), p, fixture.competition(), 1, 0) != 0 {
            continue;
        }
        if exe::nationalteams_get_international_duty_club(read_usize(G_NATIONAL_TEAMS), p, G_CURRENT_DATE, x, 0, 0) != 0 {
            continue;
        }
        if exe::transfer_manager_get_ineligibility_code(read_usize(G_TRANSFER_MANAGER), p, fixture) == 0 {
            return 1;
        }
    }
    0
}

/// The current human's manager record ([0xb74c26 + 0x300 x current screen slot]).
pub(crate) unsafe fn scout_manager_current_human_manager() -> usize {
    let k = read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize;
    read_usize((0xb74c26isize + 0x300 * k) as usize)
}

/// Tells whether the player is on the human's scouting shortlist (through the open edit session or the first type 1
/// search).
///
/// Ret 8: `SCOUT_MANAGER::is_player_shortlisted(person, manager)` (from the interested clubs
/// `collect_interested_clubs`, from the status `shortlist_manager_estimate_squad_status`, from `is_player_under_offer`
/// and from screens): null person, the "Error" box (line 0x231) and 0; without a manager, that of the human whose turn
/// it is (without him, the box at line 0x23b and 0). The human's slot (byte +0 of the manager − byte of `G_STAFF_COUNT`
/// + 16, signed, 0..15): with the edit session open (`[this + slot × 4 + scout_manager_fields::EDIT_SESSIONS]`), its
/// answer (`exe::searcheditsession_is_player_on_short_list`); otherwise the first of the 10 searches
/// (`exe::scout_manager_get_search`) of type 1 (`player_search_fields::LIST_TYPE`) answers
/// (`exe::player_search_contains_player`). Without it, 0.
/// Original: `SCOUT_MANAGER::is_player_shortlisted` (0x7f7490, scout_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn is_player_shortlisted(this: usize, person: usize, manager: usize) -> u32 {
    if person == 0 {
        show_error_at(0xa7b094, 0x231);
        return 0;
    }
    let m = if manager != 0 { manager } else { scout_manager_current_human_manager() };
    if m == 0 {
        show_error_at(0xa7b094, 0x23b);
        return 0;
    }
    let slot = read_u8(m).wrapping_sub(read_u8(G_STAFF_COUNT)).wrapping_add(0x10) as i8;
    if !(0..=15).contains(&slot) {
        return 0;
    }
    let s = read_usize(this + slot as usize * 4 + scout_manager_fields::EDIT_SESSIONS);
    if s != 0 {
        return exe::searcheditsession_is_player_on_short_list(s, person);
    }
    // simplification: the original pushes a local dword with the index only in the low byte; 0x7f6cd0 reads only that byte
    for i in 0..10u32 {
        let l = exe::scout_manager_get_search(this, i, m);
        if l != 0 && *((l + player_search_fields::LIST_TYPE) as *const u8) == 1 {
            let r = exe::player_search_contains_player(l, person);
            covered!("cps_8 scout list");
            return r; // cps8: the reply of the scout list
        }
    }
    0
}

// simplification: the history goes to the file (does not run twice) and the copy writes nothing else: original only; the
// copy takes the branch.

/// Returns the human manager's scouting record 0..9 (0x2b0 bytes).
///
/// Ret 8: scouting record `which` 0..9 of human manager `manager` (without him, the one whose turn it is in `[0xb74c26
/// + 0x300 × G_SCREEN_MANAGER_CURRENT_SLOT]`): the index (id − `G_STAFF_COUNT` + 16) × 10 + `which`, up to `[t+8]` − 1,
/// with 0x2b0 bytes each starting at `[t]`. `which` out of range, no manager or index out of range: the original
/// routine (error box).
/// Original: `scout_record` (0x7f6cd0, scout_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn scout_record(t: usize, which: i8, manager: *const i32) -> usize {
    let mut p = manager;
    if (0..=9).contains(&which) && p.is_null() {
        let team = read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize;
        p = core::ptr::read_unaligned((0xb74c26 + team * 0x300) as usize as *const *const i32);
    }
    if !(0..=9).contains(&which) || p.is_null() {
        return original_fn::<unsafe extern "thiscall" fn(usize, i8, *const i32) -> usize>(IDX_SCOUT_RECORD)(t, which, manager);
    }
    let i = ((*p).wrapping_sub(read_i32(G_STAFF_COUNT)).wrapping_add(0x10)).wrapping_mul(10).wrapping_add(which as i32);
    if i < 0 || i > read_i32(t + 8).wrapping_sub(1) {
        return original_fn::<unsafe extern "thiscall" fn(usize, i8, *const i32) -> usize>(IDX_SCOUT_RECORD)(t, which, manager);
    }
    covered!("scout record");
    read_usize(t).wrapping_add(i as usize * 0x2b0)
}

/// Empties the list and adds the person of each scout request of the club; returns whether there is any.
///
/// Ret 8: `SCOUT_MANAGER::collect_club_scout_targets(club, list)`: with `list`, empties it (`exe::llist_clear`) and
/// adds (`exe::llist_add_tail`, 0x6e bytes) the person of each scouting request of club `*club`. Returns whether there
/// is any. Null club: 0 (the list stays empty).
/// Original: `SCOUT_MANAGER::collect_club_scout_targets` (0x7f9690, scout_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn collect_club_scout_targets(this: usize, club: *const u32, list: usize) -> u32 {
    if list != 0 {
        exe::llist_clear(list);
    }
    if club.is_null() {
        return 0;
    }
    let alvos = scouting_targets_of_club(this, *club);
    if list != 0 {
        for &p in &alvos {
            (exe::llist_add_tail(list, p, 0x6e) as u32);
        }
    }
    (!alvos.is_empty()) as u32
}

/// Removes a scout request: finds the request (club, person; 17 bytes), pulls the next ones up and sorts again
/// (0x7fc270); null club = the current human manager's club.
///
/// Ret 8: `SCOUT_MANAGER::remove_scout_request(person, club)`: a null club is that of the current human manager
/// (`[0xb74c26 + 0x300 × G_SCREEN_MANAGER_CURRENT_SLOT]`, `.club_job()`; without him, nothing). With the person,
/// searches (bsearch `crt_bsearch` with 0x7fc270, which reads only +0 club and +4 person; the key's default date at +8,
/// byte `scout_request::REPORT_COUNT` has no value and the copy uses zero) for the request in the vector
/// (`scout_manager_fields::SCOUT_REQUESTS`, `scout_manager_fields::SCOUT_REQUEST_COUNT` records of 17 bytes); found:
/// pulls the following ones up one place (+0, +4, the date +8 through `exe::date_operator_3d` and byte
/// `scout_request::REPORT_COUNT`), the count goes down and, if any remain, sorts again (qsort `media_crt_qsort`).
/// Original: `SCOUT_MANAGER::remove_scout_request` (0x7f8ee0, scout_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn remove_scout_request(this: usize, person: usize, club: usize) {
    let mut key = [0u8; 0x11];
    exe::date_date_54a110(key.as_mut_ptr() as usize + 8);
    let mut c = club;
    if c == 0 {
        let i = read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as i32;
        let manager = Staff(read_u32((0xb74c26 + i * 0x300) as usize) as usize); // the current human manager
        if manager.is_null() {
            return;
        }
        c = (manager.club_job().addr() as u32) as usize;
        if c == 0 {
            return;
        }
    }
    if person == 0 {
        return;
    }
    let id_club = if c != 0 { read_u32(c) } else { u32::MAX };
    key[..4].copy_from_slice(&id_club.to_le_bytes());
    key[4..8].copy_from_slice(&read_u32(person).to_le_bytes());
    let base = read_u32(this + scout_manager_fields::SCOUT_REQUESTS) as usize;
    let s = crt_bsearch(key.as_ptr() as usize, base, read_u32(this + scout_manager_fields::SCOUT_REQUEST_COUNT), 0x11, 0x7fc270);
    if s == 0 {
        return;
    }
    let mut k = ((s.wrapping_sub(base) as u32 as u64 * 0xf0f0f0f1) >> 36) as i32; // the place of the request
    while k < read_u32(this + scout_manager_fields::SCOUT_REQUEST_COUNT) as i32 - 1 {
        let r = (read_u32(this + scout_manager_fields::SCOUT_REQUESTS) as usize).wrapping_add(k as usize * 17);
        write_u32(r, read_u32(r + 0x11));
        write_u32(r + 4, read_u32(r + 0x15));
        let mut local = [0u8; 8];
        exe::date_operator_3d(r + 8, (&mut local) as *mut [u8; 8] as usize, r + 0x19);
        write_u8(r + scout_request::REPORT_COUNT, read_u8(r + 0x21));
        k += 1;
    }
    let n = read_u32(this + scout_manager_fields::SCOUT_REQUEST_COUNT).wrapping_sub(1);
    write_u32(this + scout_manager_fields::SCOUT_REQUEST_COUNT, n); // um pedido a menos
    if n != 0 {
        media_crt_qsort(read_u32(this + scout_manager_fields::SCOUT_REQUESTS) as usize, n as i32, 0x11, 0x7fc270);
    }
}

/// Tells whether the club can scout the nation: a poor (balance < 1,000,000) and small (reputation < 5000) club outside
/// the nation's region: 0; else 1.
///
/// Ret 8; `this` is not read: `SCOUT_MANAGER::can_club_scout_nation(nation, club)`: the club (or that of the human
/// whose turn it is) does not cover the nation only if it has finances (`exe::financetable_get_club_finance` on
/// `G_FINANCE_TABLE`) with a 64-bit balance below 1,000,000, reputation (`.reputation()`) below 5000 and the region
/// (`.actual_region()`) of its nation (`.nation()`) different from the region of `nation`: 0. Otherwise 1. No club at
/// all: the original routine ("Error" box and 0).
/// Original: `SCOUT_MANAGER::can_club_scout_nation` (0x7f9190, scout_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn can_club_scout_nation(this: usize, nation: usize, club: usize) -> u32 {
    let nation = Nation(nation);
    let c = Club(club_or_current_human(club));
    if c.is_null() {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_CAN_CLUB_SCOUT_NATION)(this, nation.addr(), club); // message box: original only
    }
    let f = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), c);
    if f != 0 && read_i64(f) < 1_000_000 && c.reputation() < 5000 {
        if c.nation().actual_region() != nation.actual_region() { // outside the region
            covered!("cpk_6 scout outside the region");
            return 0;
        }
    }
    1
}

/// Removes from the manager's scout lists the scouts no longer at the club and gives each club scout without a list the
/// first free list; with the edit session open, it refreshes the scouts.
///
/// Ret 4: `SCOUT_MANAGER::sync_scout_lists(manager)`, when the staff of the human club changes (leaving the post
/// `remove_staff_from_club` and the scouts' vacancy, staff transfers and signings). Without a manager, the human whose
/// turn it is; without him, the box at line 0x1ba and 0. Of the manager's 10 searches, the scout lists
/// (`player_search_fields::LIST_TYPE` = 4) whose scout (`search_scout`) is no longer a scout (`.job_for_club()` =
/// `job_kind::SCOUT`) of the manager's club (`.club_job()`) are left without a scout (`set_search_scout` with 0;
/// without a club, all of them; with a club, those without a scout stay as they are). With a club, each of the club's 7
/// scouts (`exe::database_get_club_scouts`) that has no list goes to the first scout list without a scout. Then the
/// manager's slot (byte +0 − byte of `G_STAFF_COUNT` + 16, 0..15; out of range, the box at line 0x212 and 0): with the
/// edit session open (`[this + slot × 4 + scout_manager_fields::EDIT_SESSIONS]`), it redoes the lists' scouts
/// (`exe::searcheditsession_refresh_scout_assignments`). Returns 1.
/// Original: `SCOUT_MANAGER::sync_scout_lists` (0x7f7270, scout_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn sync_scout_lists(this: usize, manager: usize) -> u32 {
    let m = Staff(if manager != 0 { manager } else { scout_manager_current_human_manager() });
    if m.is_null() {
        show_error_at(0xa7b094, 0x1ba);
        return 0;
    }
    let club = m.club_job();
    for i in 0..10u8 {
        let l = get_search(this, i, m);
        if l == 0 || read_u8(l + player_search_fields::LIST_TYPE) != 4 {
            continue;
        }
        let o = Staff(search_scout(l));
        if !club.is_null() {
            if o.is_null() || (o.club_job() == club && o.job_for_club() == job_kind::SCOUT) {
                continue;
            }
        }
        covered!("cqa_6 list without scout");
        set_search_scout(l, 0);
    }
    if !club.is_null() {
        let mut v = [0usize; 7];
        exe::database_get_club_scouts(club, v.as_mut_ptr());
        for &s in v.iter() {
            if s == 0 {
                continue;
            }
            let mut free: i8 = -1;
            let mut has = false;
            for i in 0..10u8 {
                let l = get_search(this, i, m);
                if l == 0 || read_u8(l + player_search_fields::LIST_TYPE) != 4 {
                    continue;
                }
                if search_scout(l) == s {
                    has = true;
                    break;
                }
                if search_scout(l) == 0 && free == -1 {
                    free = i as i8;
                }
            }
            if !has && free > -1 {
                covered!("cqa_6 scout in the free list");
                set_search_scout(get_search(this, free as u8, m), s);
            }
        }
    }
    let slot = read_u8(m.addr()).wrapping_sub(read_u8(G_STAFF_COUNT)).wrapping_add(0x10) as i8;
    if !(0..=15).contains(&slot) {
        show_error_at(0xa7b094, 0x212);
        return 0;
    }
    let s = read_usize(this + slot as usize * 4 + scout_manager_fields::EDIT_SESSIONS);
    if s != 0 {
        exe::searcheditsession_refresh_scout_assignments(s);
    }
    1
}

/// Returns the player's slot in the record of the human's search that scouts his club, or 0.
///
/// Ret 4: `SCOUT_MANAGER::find_club_search_entry(p)`, from the person and search screens (0x808540, 0x813210, 0x870650,
/// 0x87a510): without the human whose turn it is or without the person's club (`.club_job()`), 0. The human's slot
/// (byte +0 − byte of `G_STAFF_COUNT` + 16, 0..15; out of range, the box at line 0x4b5 and 0). The first of the 10
/// searches (`exe::scout_manager_get_search`) with the scouted club (`player_search_fields::ASSIGNED_CLUB`) equal to
/// the person's and its record (`player_search_fields::SCOUTED_CLUB_RECORD`) gives the person's slot in it
/// (`exe::player_search_find_or_claim_player_slot` without reserving); none, 0.
/// Original: `SCOUT_MANAGER::find_club_search_entry` (0x7f8610, scout_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn find_club_search_entry(this: usize, p: usize) -> usize {
    let p = Staff(p);
    let m = scout_manager_current_human_manager();
    if m == 0 || p.club_job().is_null() {
        return 0;
    }
    let slot = read_u8(m).wrapping_sub(read_u8(G_STAFF_COUNT)).wrapping_add(0x10) as i8;
    if !(0..=15).contains(&slot) {
        show_error_at(0xa7b094, 0x4b5);
        return 0;
    }
    for i in 0..10u32 {
        let l = exe::scout_manager_get_search(this, i, m);
        // simplification: the original reads +0x190 without checking the search (would crash with 0); here a null search does not count
        if l == 0 || read_usize(l + player_search_fields::ASSIGNED_CLUB) != read_usize(p.club_job().addr()) || read_usize(exe::scout_manager_get_search(this, i, m) + player_search_fields::SCOUTED_CLUB_RECORD) == 0 {
            continue;
        }
        return exe::player_search_find_or_claim_player_slot(read_usize(exe::scout_manager_get_search(this, i, m) + player_search_fields::SCOUTED_CLUB_RECORD), p, 0);
    }
    0
}

/// Writes the translated scout list name: Short List, Player Search, Staff Search or Scout #n.
///
/// `scout_manager_write_list_name(index, text, size)`, from the SCOUT_MANAGER constructor (0x7f6760) and from the
/// search screens. The translated name (`FN_LANGLIB_SPRINTF`) of list `index`: 0 "Short List", 1 "Player Search", 2
/// "Staff Search", 3..9 "Scout #<%d - number>" with `index` − 2. Returns 1. Index outside 0..9, null text or size below
/// 50: the original routine ("Error" box, 0).
/// Original: `scout_manager_write_list_name` (0x7fc150, scout_manager.cpp, cdecl).
pub(crate) unsafe extern "C" fn scout_manager_write_list_name(index: u32, text: usize, size: i32) -> u32 {
    // 0x66f4e0: cdecl(text, format, ...) (add esp, 8 or 0xc)
    let k = index as u8 as i8;
    if !(0..=9).contains(&k) || text == 0 || size < 0x32 {
        return original_fn::<unsafe extern "C" fn(u32, usize, i32) -> u32>(IDX_SCOUT_MANAGER_WRITE_LIST_NAME)(index, text, size); // message box: original only
    }
    let translate: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    match k {
        0 => translate(text, 0xa7dd94),
        1 => translate(text, 0xa7dd84),
        2 => translate(text, 0xa7dd74),
        _ => translate(text, 0xa7dd5c, k as i32 - 2),
    };
    1
}

/// Tells whether the club (or the current human's) asked to scout the player, with fewer than 3 reports and, if asked,
/// with the date already due.
///
/// Ret 0xc: `SCOUT_MANAGER::has_scout_request(player, club, so_vencidos)`: whether the club (null: that of the human
/// whose turn it is, `[G_SCREEN_RECORDS_MANAGER + 0x300 × G_SCREEN_MANAGER_CURRENT_SLOT]` `staff::CLUB_JOB`) asked for
/// the player to be scouted: binary search (`FN_CRT_BSEARCH` with 0x7fc270) by the pair (club id, player id) in the
/// requests (`scout_manager_fields::SCOUT_REQUESTS`, count `scout_manager_fields::SCOUT_REQUEST_COUNT`, 17 bytes) with
/// fewer than 3 reports (`scout_request::REPORT_COUNT`, signed); with `so_vencidos`, only if the request date
/// (`scout_request::DATE`: day, `scout_request::DATE_YEAR`: year) is before today (`G_CURRENT_DATE`, unsigned year,
/// signed day). Returns 1 or 0. Null player: the original routine (box at line 0x5da). The original's local date
/// (`exe::date_date_54a110`) only fills the key after the pair.
/// Original: `SCOUT_MANAGER::has_scout_request` (0x7f8d70, scout_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn has_scout_request(this: usize, player: usize, club: usize, so_vencidos: u32) -> u32 {
    if player == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32>(IDX_HAS_SCOUT_REQUEST)(this, player, club, so_vencidos);
    }
    let mut club = club;
    if club == 0 {
        let human = Staff(read_ptr(G_SCREEN_RECORDS_MANAGER + 0x300 * (read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize)));
        if human.is_null() {
            return 0;
        }
        club = read_ptr(human.addr() + staff::CLUB_JOB);
        if club == 0 {
            return 0;
        }
    }
    let list = read_ptr(this + scout_manager_fields::SCOUT_REQUESTS);
    let found = if list == 0 {
        0
    } else {
        let search: unsafe extern "C" fn(*const u8, usize, u32, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
        let mut key = [0u8; 16];
        (exe::date_date_54a110((key.as_mut_ptr().add(8)) as usize) as *mut u8);
        key[4..8].copy_from_slice(&read_u32(player).to_le_bytes());
        key[0..4].copy_from_slice(&read_u32(club).to_le_bytes());
        search(key.as_ptr(), list, read_u32(this + scout_manager_fields::SCOUT_REQUEST_COUNT), 0x11, 0x7fc270)
    };
    if found == 0 {
        covered!("cpq_12 without request");
        return 0; // no request
    }
    if read_i8(found + scout_request::REPORT_COUNT) >= 3 {
        return 0;
    }
    if so_vencidos != 0 {
        let year = read_u16(found + scout_request::DATE_YEAR);
        let today = read_u16(G_CURRENT_DATE_YEAR);
        if year > today {
            return 0;
        }
        if year == today && read_i16(found + scout_request::DATE) >= read_i16(G_CURRENT_DATE) {
            return 0;
        }
    }
    1
}

