use super::*;

/// Day/year is after the date at `data` (unsigned year +2, signed day +0).
///
/// [pt] `a` depois de `b` (datas: ano sem sinal em +2, dia com sinal em +0), como as comparações da original.
pub(crate) unsafe fn scout_manager_date_after(day: i16, year: u16, data: usize) -> bool {
    let (d, a) = (read_i16(data), read_i16(data + cm_date::YEAR) as u16);
    year > a || (year == a && day > d)
}

/// The dry run of `0x7f8750`: the copy only prunes the requests (in its own vector) and stops before the reports.
///
/// [pt] O ensaio da 0x7f8750: a cópia só poda os pedidos (num vetor dela) e para antes dos relatórios.
pub(crate) static mut SCOUT_DAILY_DRY_RUN: bool = false;

/// Prunes a scouting request (0x11 bytes: +0 the club, +4 the player, +8 the date, +0x10 how many reports); returns
/// (remove, process).
///
/// A request is removed if the club is not human, if the player belongs to the club itself (+0x39 or the contract's),
/// if it is more than a year old, if it has 3 reports, or (after 30 days) if the player is injured (0x62c400 = 2).
///
/// Between 30 days and a year the request is processed (and removed), except for a player without a nation or from a
/// club that plays in the next 14 days, which stays.
///
/// [pt] A poda de um pedido (0x11 bytes: +0 o clube, +4 o jogador, +8 a data, +0x10 quantos relatórios): devolve
/// (tirar, processar). Sai o pedido de clube que não é humano, de jogador do próprio clube (o +0x39 ou o do contrato),
/// com mais de um ano, com 3 relatórios, ou (depois de 30 dias) de jogador machucado (0x62c400 = 2); entre 30 dias e um
/// ano o pedido é processado (e sai), salvo jogador sem nação ou de clube que joga nos 14 dias seguintes, que fica.
pub(crate) unsafe fn judge_scout_request(request: usize, catorze: usize) -> (bool, bool, usize, usize, usize) {
    // 0x600a50: 5 pushes (club, 1, 1, 0, 0); add esp, 0x14 -> cdecl; 0x4cf5a0: push player; mov ecx, [0xae1050] ->
    // thiscall, ret 4; 0x54c5b0: push n; push &output; lea ecx, [pedido+8] -> thiscall, ret 8; 0x62c400: push player;
    // mov ecx, [0xb63d28] -> thiscall, ret 4 (al); 0x4cf5f0: push club; push player; mov ecx, [0xae1050] -> thiscall,
    // ret 8; 0x5acbe0: 9 pushes; mov ecx, [0xae2a58] -> thiscall, ret 0x24 (ax)
    let player = staff_records_staff_by_id(read_usize(request + scout_request::PLAYER_ID));
    let club = friendly_club_by_id(read_usize(request));
    if exe::human_manager_is_human_club(club, 1, 1, 0, 0) == 0 {
        return (true, false, 0, 0, 0);
    }
    let manager = read_usize(club + club::MANAGER);
    if read_usize(player + staff::CLUB_JOB) == club {
        return (true, false, 0, 0, 0);
    }
    let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), player);
    core::ptr::write_unaligned(G_CONTRACT_LOOKUP_RESULT as *mut usize, c);
    let of_contract = if c != 0 { friendly_club_by_id(read_usize(c + contract::CLUB)) } else { 0 };
    if of_contract == club {
        return (true, false, 0, 0, 0);
    }
    let (dia, ano) = (read_i16(G_CURRENT_DATE), read_i16(G_CURRENT_DATE_YEAR) as u16);
    let mut t = [0u32; 2];
    let year_after = exe::date_add_days(request + scout_request::DATE, t.as_mut_ptr() as usize, 0x16d); // cqf12: um ano
    if scout_manager_date_after(dia, ano, year_after) {
        return (true, false, 0, 0, 0);
    }
    if (read_u8(request + scout_request::REPORT_COUNT) as i8) >= 3 { // cqf12: três relatórios
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
    let seu = read_usize(player + staff::CLUB_JOB);
    let mut nation = if seu != 0 { read_usize(seu + club::NATION) } else { 0 };
    if nation == 0 {
        nation = read_usize(player + staff::NATION);
        if nation == 0 {
            return (false, false, 0, 0, 0);
        }
    }
    let mut direct = false;
    if seu != 0 {
        let k = exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), player, seu);
        if k != 0 {
            let s = read_u8(k + contract::SQUAD_STATUS) & 0xf0;
            direct = s == 0x60 || s == 0x70 || s == 0;
        }
    }
    if !direct && seu != 0 {
        // does the player’s club play from today to 14 days later? (no list; the high part of the 2nd dword is stack leftover in the
        // original, and 0x5acbe0 only reads the low word)
        let last = read_usize(catorze) as u32;
        let last_high = core::ptr::read_unaligned((catorze + 2) as *const u32);
        let a1 = (last_high & 0xffff_0000) | dia as u16 as u32;
        let a2 = ano as u32;
        if (exe::fixture_manager_find_fixtures(read_usize(G_FIXTURE_MANAGER), a1, a2, last, last_high, read_usize(seu) as u32, 0, (-1) as i32 as u32, (-2) as i32 as u32, 1) as i16) != 0 {
            return (false, false, 0, 0, 0);
        }
    }
    (true, true, player, manager, nation)
}

/// The scout manager's daily update, on day slice 2: prunes the scouting requests (non-human club, player already at
/// the club, older than a year, 3 reports, injured); a request older than 30 days becomes up to 3 reports from the
/// manager's scouts covering the player's nation and leaves; then the reports, the request order and one search tick on
/// every human manager list.
///
/// Ret 0; SEH frame for the local list and the dates: `SCOUT_MANAGER::daily_update()`, from the day (only on slice 2 of
/// the day, `[0xae2c88]`; otherwise 0). Prunes the scouting requests (+0x5c, count +0x64, 0x11 bytes; the removed one
/// gets the last one), and the processed request becomes up to 3 reports from the scouts of the club's manager (of the
/// 10 lists, `scout_record`, the one that covers the player's nation, `scout_covers_nation`): `exe::player_search_report_on_player` with −1000, mode 1, and the record
/// (0x67 bytes) goes into the local list if byte +0x66 ended ≥ 0. Then the reports (`send_scout_reports`), the request order
/// (`exe::crt_qsort` with 0x7fc270) and one search (`exe::player_search_run_scout_day`) on each of the 10 lists of the human managers (the last 16
/// persons with +0x5f = 1). Returns 1.
/// Original: `SCOUT_MANAGER::daily_update` (0x7f8750, scout_manager.cpp, thiscall, ret 0).
///
/// [pt] { 0x7f8750 } (thiscall, ret 0, scout_manager.cpp, quadro SEH da lista local e das datas): `SCOUT_MANAGER::
/// daily_update()`, pelo dia (só na fatia 2 do dia, [0xae2c88]; senão 0). Poda os pedidos de observação (+0x5c,
/// conta +0x64, 0x11 bytes; o tirado recebe o último), e o pedido processado vira até 3 relatórios dos olheiros do
/// técnico do clube (das 10 listas, 0x7f6cd0, a que cobre a nação do jogador, 0x7b7490): 0x7b62a0 com −1000, modo 1, e o
/// registro (0x67 bytes) entra na lista local se o byte +0x66 ficou ≥ 0. Depois os relatórios (0x7f9890), a ordem dos
/// pedidos (0x945333 com 0x7fc270) e uma busca (0x7b3e20) em cada uma das 10 listas dos técnicos humanos (as 16 últimas
/// pessoas com +0x5f = 1). Devolve 1.
pub(crate) unsafe extern "thiscall" fn scout_manager_daily_update(this: usize) -> u32 {
    // 0x68d020 / 0x68d040 / 0x54a110: lea ecx, local -> thiscall; 0x54c5b0: push 0xe; push &output; mov ecx, 0xae2c90
    let mut d14 = [0u32; 2];
    let catorze = d14.as_mut_ptr() as usize;
    exe::date_add_days(G_CURRENT_DATE, catorze, 0xe);
    // the report record (0x67 bytes) with the dates the original’s entry builds (+0, +0x2f, +0x47)
    let mut record = [0u8; 0x68];
    let reg = record.as_mut_ptr() as usize;
    exe::date_date_54a110(reg);
    exe::date_date_54a110(reg + 0x2f);
    exe::date_date_54a110(reg + 0x47);
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
        if processar && !SCOUT_DAILY_DRY_RUN {
            crate::comparison::branch("cqf_12 request processed");
            let mut feitos: i8 = 0;
            for k in 0..10u32 {
                if feitos >= 3 { // cqf12: up to three reports
                    break;
                }
                let lo = exe::scout_manager_get_search(this, k, tecnico);
                if exe::player_search_scout_covers_nation(lo, nacao) != 0 {
                    exe::player_search_report_on_player(lo, jogador, reg, -1000, 1, 0, 0, 1);
                    feitos += 1;
                    if (read_u8(reg + 0x66) as i8) > -1 {
                        crate::comparison::branch("cqf_12 request report");
                        exe::llist_add_tail(l, reg, 0x67);
                    }
                }
            }
        }
        if tirar {
            let n = read_i32(this + scout_manager_fields::SCOUT_REQUEST_COUNT);
            let base = read_usize(this + scout_manager_fields::SCOUT_REQUESTS);
            core::ptr::copy(base.wrapping_add((n as usize).wrapping_mul(0x11)).wrapping_sub(0x11) as *const u8, request as *mut u8, 0x11);
            core::ptr::write_unaligned((this + scout_manager_fields::SCOUT_REQUEST_COUNT) as *mut i32, n - 1);
            i -= 1;
        }
        i += 1;
    }
    if SCOUT_DAILY_DRY_RUN {
        exe::drop_llist(l);
        return 1;
    }
    exe::scout_manager_send_scout_reports(this, l);
    exe::crt_qsort(read_usize(this + scout_manager_fields::SCOUT_REQUESTS), (read_i32(this + scout_manager_fields::SCOUT_REQUEST_COUNT)) as u32, 0x11, 0x7fc270);
    for k in 0..0x10i32 {
        let p = staff_records_staff_by_id((read_i32(G_STAFF_COUNT) - k - 1) as usize);
        if read_u8(p + staff::CLASSIFICATION) == 1 {
            crate::comparison::branch("cqf_12 human search");
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
///
/// [pt] O registro das notícias de relatório que a 0x7f9a60 recebe (jogador, clube, quantos), ligado só no comparador da
/// 0x7f9890 para conferir os grupos da original.
pub(crate) static mut SCOUT_REPORT_GROUPS: Option<Vec<(u32, u32, u8)>> = None;

/// Copies the list records into an array, sorts it and sends one news item per run of the same player and club.
///
/// Ret 4: `SCOUT_MANAGER::send_scout_reports(list)`, from the scouts' day (`scout_manager_daily_update`) and from the search (`scout_manager_scout_match`).
/// With the list not empty (+0 count): a vector of count × 0x67 (`exe::crt_malloc`; out of memory, the box at line 0x7dd) with
/// copies of the records, sorted (`exe::crt_qsort` with 0x7fc2a0); each consecutive group of the same player (+0x2b) and the
/// same club (+0x50) becomes a report news item (`send_scout_report_news` with the first one and how many). Frees the vector
/// (`exe::crt_free`). The list is left with its cursor at the start.
/// Original: `SCOUT_MANAGER::send_scout_reports` (0x7f9890, scout_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x7f9890 } (thiscall, ret 4, scout_manager.cpp): `SCOUT_MANAGER::send_scout_reports(lista)`, pelo dia dos olheiros
/// (0x7f8750) e pela busca (0x7f8c00). Com a lista não vazia (+0 conta): um vetor de conta × 0x67 (0x944e46; sem
/// memória, a caixa da linha 0x7dd) com cópias dos registros, ordenado (0x945333 com 0x7fc2a0); cada grupo seguido do
/// mesmo jogador (+0x2b) e do mesmo clube (+0x50) vira uma notícia de relatório (0x7f9a60 com o primeiro e quantos).
/// Libera o vetor (0x9452ca). A lista fica com o cursor no começo.
pub(crate) unsafe extern "thiscall" fn send_scout_reports(this: usize, list: usize) {
    let count = read_i16(list);
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
    core::ptr::write_unaligned((list + llist::CURSOR) as *mut usize, read_usize(list + llist::HEAD));
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
        let club_id = read_usize(friendly_club_by_id(read_usize(r + scout_report::CLUB_ID)));
        let player_id = read_usize(staff_records_staff_by_id(read_usize(r + scout_report::PLAYER_ID)));
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
        if SCOUT_DAILY_DRY_RUN {
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
/// Ret 8; SEH frame for the local news item: `SCOUT_MANAGER::send_scout_report_news(record, count)`, from the reports
/// (`send_scout_reports`). News 0x138e (`construct_news_item`): count (up to 3), +0x66, the scout (`exe::write_staff_name_ref_to_news` with +0x1f), +0x59, +0x5a, club
/// +0x50, the match data (0x7e4de0), +0x5e, +0x62, +0x58, +0x54, from each of the records (+0x37 + 0x67 × k) +0xc, +0,
/// +4, +8 and +0x26 + 8 × +0x25, 0x420e9 and the player's (+0x2b) strengths for the club (`staff_get_player_strength_flags`); goes to the club's
/// manager (`send_to_club_manager`).
/// Original: `SCOUT_MANAGER::send_scout_report_news` (0x7f9a60, scout_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x7f9a60 } (thiscall, ret 8, scout_manager.cpp, quadro SEH da notícia local): `SCOUT_MANAGER::
/// send_scout_report_news(registro, quantos)`, pelos relatórios (0x7f9890). A notícia 0x138e (0x77bbe0): quantos (até
/// 3), +0x66, o olheiro (0x7e5160 com +0x1f), +0x59, +0x5a, o clube +0x50, os dados do jogo (0x7e4de0), +0x5e, +0x62,
/// +0x58, +0x54, de cada um dos registros (+0x37 + 0x67 × k) +0xc, +0, +4, +8 e +0x26 + 8 × +0x25, 0x420e9 e as forças
/// do jogador (+0x2b) para o clube (0x546ab0); vai ao técnico do clube (0x7861e0).
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
    let mut pr = |idx: &mut u8, v: u32| {
        exe::newsitem_set_slot(nt, (*idx as u32) as u32 as i8, v);
        *idx = idx.wrapping_add(1);
    };
    pr(&mut idx, read_u8(r + 0x66) as i8 as i32 as u32);
    exe::write_staff_name_ref_to_news(r + 0x1f, nt, &mut idx);
    pr(&mut idx, read_u8(r + 0x59) as i8 as i32 as u32);
    pr(&mut idx, read_i16(r + 0x5a) as i32 as u32);
    pr(&mut idx, read_usize(r + scout_report::CLUB_ID) as u32);
    data(r, nt, &mut idx);
    pr(&mut idx, read_usize(r + 0x5e) as u32);
    pr(&mut idx, read_usize(r + 0x62) as u32);
    pr(&mut idx, read_u8(r + 0x58) as i8 as i32 as u32);
    pr(&mut idx, read_usize(r + 0x54) as u32);
    for k in 0..n.max(0) as usize {
        let p = r + 0x37 + k * 0x67;
        pr(&mut idx, read_usize(p + 0xc) as u32);
        pr(&mut idx, read_usize(p) as u32);
        pr(&mut idx, read_usize(p + 4) as u32);
        pr(&mut idx, read_usize(p + 8) as u32);
        pr(&mut idx, ((read_u8(p + 0x26) as i8 as i32) + (read_u8(p + 0x25) as i8 as i32) * 8) as u32);
    }
    pr(&mut idx, 0x420e9);
    let c = read_i32(r + scout_report::CLUB_ID);
    let club = if c > -1 { friendly_club_by_id(c as usize) } else { 0 };
    let k = idx;
    idx = idx.wrapping_add(1);
    let _ = idx;
    exe::newsitem_set_slot(nt, (k as u32) as u32 as i8, exe::staff_get_player_strength_flags(staff_records_staff_by_id(read_usize(r + scout_report::PLAYER_ID)), club));
    exe::news_send_to_club_manager(read_usize(G_NEWS), nt, friendly_club_by_id(read_usize(r + scout_report::CLUB_ID)));
}

/// Date r equals today's (year and day).
///
/// [pt] A data `r` é a de hoje (o ano e o dia)?
pub(crate) unsafe fn is_today(r: usize, today: usize) -> bool {
    read_u16(r + cm_date::YEAR) == read_u16(today + cm_date::YEAR) && read_i16(r) == read_i16(today)
}

/// Marks clubs with fixtures on date d, the day before and the day after, leaving d unchanged.
///
/// [pt] Marca os clubes com jogo na data `d` e nos dias vizinhos (0x7775c0 na data, na véspera e no dia seguinte; a data
/// volta a ser a mesma no fim: 0x54c8e0/0x54c5b0 e 0x417c10).
pub(crate) unsafe fn mark_clubs_three_days(this: usize, d: usize) {
    // 0x7775c0: push &data; mov ecx, edi -> thiscall, ret 4
    let (mut t, mut u) = ([0u8; 8], [0u8; 8]);
    let (pt, pu) = (t.as_mut_ptr() as usize, u.as_mut_ptr() as usize);
    crate::comparison::branch("cqc_12 marks the clubs");
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
///
/// [pt] Os jogos da lista `l` (do dia daqui a duas semanas): para cada um, as convocações (0x76bab0) e, fora das fases
/// finais (a competição +0x14 numa das chaves 0x9cf79c, 0x9cf7a0, 0x9cf7a4, 0x9cf7a8, 0x9cf7ac, 0x9cf790, 0x9cf8a0,
/// 0x9cf78c, 0x9cf964), com mandante que não é a seleção B (0x5409b0) e a marca +0x40: os convocados por clube
/// (0x7771c0) das duas seleções; com o bit 0x200 de +0x4d, também os das seleções B (0x53b4f0 da nação +0x53 de cada
/// uma) e, sem ele e com o 0x100, os das duas com `b` = 1.
pub(crate) unsafe fn select_squads_for_fixtures(this: usize, l: usize) {
    // 0x76bab0: push p; mov ecx, edi -> thiscall ret 4; 0x5409b0 e 0x53b4f0: push; add esp, 4 -> cdecl;
    // 0x7771c0: push p; push b; push time; mov ecx, edi -> thiscall ret 0xc
    write_usize(l + llist::CURSOR, read_usize(l + llist::HEAD));
    let mut p = exe::llist_get_next(l);
    while p != 0 {
        crate::comparison::branch("cqc_12 calls up");
        exe::nationalteams_select_squads_for_fixture(this, p); // cqc12: the call-ups for the match
        let comp = read_usize(p + fixture::COMPETITION);
        let final_ = comp != 0
            && [KEY_CLUB_COMP_FIFA_WORLD_CUP, KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_OLYMPIC_GAMES, KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP, KEY_CLUB_COMP_COPA_AMERICA, KEY_CLUB_COMP_AFRICAN_CUP_OF_NATIONS, KEY_CLUB_COMP_ASIAN_CUP, KEY_CLUB_COMP_OCEANIA_NATIONS_CUP, KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_ASIAN_GAMES, KEY_CLUB_COMP_CONFEDERATIONS_CUP]
                .iter().any(|&k| read_usize(comp) == read_usize(k));
        if !final_ && exe::club_is_national_team(read_usize(p + fixture::HOME_CLUB)) == 0 && *((p + fixture::HOME_SIDE_MARK) as *const u8) != 0 {
            exe::nationalteams_record_club_call_ups(this, read_usize(p + fixture::HOME_CLUB), 0, p);
            exe::nationalteams_record_club_call_ups(this, read_usize(p + fixture::AWAY_CLUB), 0, p);
            let f = read_u16(p + fixture::SQUAD_RULES);
            if f & 0x200 != 0 {
                let b1 = exe::database_get_national_u21_team(read_usize(read_usize(p + fixture::HOME_CLUB) + 0x53));
                exe::nationalteams_record_club_call_ups(this, b1, 0, p);
                let b2 = exe::database_get_national_u21_team(read_usize(read_usize(p + fixture::AWAY_CLUB) + 0x53));
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
///
/// [pt] Quantos itens da lista em +0 são do dia `dia` (+0x10), percorrendo os nós sem mexer no cursor.
pub(crate) unsafe fn count_items_on_day(this: usize, day: i16) -> u32 {
    let l = read_usize(this);
    if l == 0 {
        return 0;
    }
    let mut n = 0;
    let mut no = read_usize(l + llist::HEAD);
    while no != 0 {
        if read_i16(read_usize(no) + 0x10) == day {
            n += 1;
        }
        no = read_usize(no + llist_node::NEXT);
    }
    n
}

/// Index (0..15) of the current human manager; Err without manager, Ok(None) when out of range.
///
/// [pt] O índice (0..15) do técnico humano da vez ([0xb74c26 + 0x300 × [0xb77c7a]]) pela conta de um byte da original
/// ((id − [0xae2364]) + 16 em 8 bits); Err(()) sem técnico, Ok(None) fora (a caixa).
pub(crate) unsafe fn current_human_index() -> Result<Option<usize>, ()> {
    let t = read_usize(G_SCREEN_RECORDS_MANAGER.wrapping_add((*(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as i32 as usize).wrapping_mul(0x300)));
    if t == 0 {
        return Err(());
    }
    let i = (*(t as *const u8)).wrapping_sub(*(G_STAFF_COUNT as *const u8)).wrapping_add(0x10) as i8;
    Ok(if (0..=0xf).contains(&i) { Some(i as usize) } else { None })
}

/// Search record k (0x2b0 bytes) of session s.
///
/// [pt] O registro de busca `k` (0x2b0 bytes) da sessão `s`.
pub(crate) unsafe fn search_record(s: usize, k: i32) -> usize {
    s.wrapping_add((k as usize).wrapping_mul(0x2b0))
}

/// Tells whether the current search of the current human manager session has no club (+0x191 = −2).
///
/// Ret 0: `SCOUT_MANAGER::current_search_has_no_club()`, from the search screens (0x482a9b). The session of the human
/// manager whose turn it is (`[this + 0xc + 4 × index]`): the current search (the signed byte at +0 of the session)
/// without a club (+0x191 = −2). No manager or no session: 0; index out of range: the box (line 0x454; the original)
/// and 0.
/// Original: `SCOUT_MANAGER::current_search_has_no_club` (0x7f8420, scout_manager.cpp, thiscall).
///
/// [pt] { 0x7f8420 } (thiscall, ret 0, scout_manager.cpp): `SCOUT_MANAGER::current_search_has_no_club()`, pelas telas das
/// buscas (0x482a9b). A sessão do técnico humano da vez ([this + 0xc + 4 × índice]): a busca atual (o byte com sinal
/// em +0 da sessão) sem clube (+0x191 = −2). Sem técnico ou sem sessão: 0; índice fora: a caixa (linha 0x454; a
/// original) e 0.
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
    (read_usize(search_record(s, *(s as *const i8) as i32) + 0x191) as i32 == -2) as u32
}

/// Tells whether one of the 10 searches of the current human manager session is for the club.
///
/// Ret 4: `SCOUT_MANAGER::is_club_in_session_searches(club)`, from the screens (0x482aaf, 0x483040, 0x7b34e1). Any of
/// the 10 searches of the session of the human manager whose turn it is has the club (+0x191 = +0 of the club): 1. No
/// manager or no session: 0; index out of range: the box (line 0x480; the original) and 0.
/// Original: `SCOUT_MANAGER::is_club_in_session_searches` (0x7f8500, scout_manager.cpp, thiscall).
///
/// [pt] { 0x7f8500 } (thiscall, ret 4, scout_manager.cpp): `SCOUT_MANAGER::is_club_in_session_searches(clube)`, pelas telas
/// (0x482aaf, 0x483040, 0x7b34e1). Alguma das 10 buscas da sessão do técnico humano da vez tem o clube (+0x191 = +0
/// do clube): 1. Sem técnico ou sem sessão: 0; índice fora: a caixa (linha 0x480; a original) e 0.
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
/// Ret 8; SEH frame for the local LLIST `exe::new_llist`/`exe::drop_llist`: `SCOUT_MANAGER::scout_match(fixture, result)`, from the
/// closing of the day's match (`close_day_match`, Rust). For each of the last 16 person records (`[0xae23bc]`, 0x6e bytes, from
/// the last backwards) with +0x5f = 1 (the human manager): in a zeroed vector of 10 marks (`exe::crt_calloc`), the 10 scouts in
/// a random order (`exe::utils_random` of 10 until an unmarked one is found); each one's record (`scout_record`) with +0x19d, in a
/// fixture with flags +0x40 and +0x41 and of the manager's club (+0x39 = +0x1c or +0x20 of the fixture), starts the
/// scouting (`exe::player_search_start_scout_assignment` with −1, −1, −1, 0, 1); and watches the match (`player_search_scout_match`, the reports into the local list). The
/// vector is freed (`exe::crt_free`). Finally the reports go out (`send_scout_reports`) and the list is destroyed. Returns 1.
/// Original: `SCOUT_MANAGER::scout_match` (0x7f8c00, scout_manager.cpp, thiscall).
///
/// [pt] { 0x7f8c00 } (thiscall, ret 8, scout_manager.cpp, quadro SEH da LLIST local 0x68d020/0x68d040):
/// `SCOUT_MANAGER::scout_match(partida, resultado)`, pelo fechamento da partida do dia (0x6b5f00, Rust). Para cada um
/// dos 16 últimos registros de pessoa ([0xae23bc], 0x6e bytes, do último para trás) com +0x5f = 1 (o técnico humano):
/// num vetor zerado de 10 marcas (0x945501), os 10 observadores numa ordem sorteada (0x90cfc0 de 10 até achar um não
/// marcado); o registro de cada um (0x7f6cd0) com +0x19d, numa partida com as marcas +0x40 e +0x41 e do clube do
/// técnico (+0x39 = +0x1c ou +0x20 da partida), começa a observação (0x7ae9c0 com −1, −1, −1, 0, 1); e observa a
/// partida (0x7b5be0, os relatórios na lista local). O vetor sai (0x9452ca). Por fim os relatórios vão (0x7f9890) e a
/// lista é destruída. Devolve 1.
pub(crate) unsafe extern "thiscall" fn scout_manager_scout_match(this: usize, p: usize, result: usize) -> u32 {
    // calloc: push 4; push 0xa; add esp, 8 -> cdecl (10, 4); random draw: push 0xa; add esp, 4 -> cdecl; free: cdecl;
    // 0x7f6cd0: push pessoa; push qual; mov ecx, [this] -> thiscall ret 8; 0x7ae9c0: 5 pushes; mov ecx, registro ->
    // thiscall ret 0x14; 0x7b5be0: push &list; push result; push match; mov ecx, record -> thiscall ret 0xc;
    // 0x7f9890: push &lista; mov ecx, this -> thiscall ret 4; 0x68d020/0x68d040: lea ecx, lista -> thiscall
    let mut l = [0u8; 0x10];
    let list = l.as_mut_ptr() as usize;
    exe::new_llist(list);
    for k in 0..16i32 {
        let idx = (read_usize(G_STAFF_COUNT) as i32).wrapping_sub(k).wrapping_sub(1);
        let person = read_usize(G_STAFF_TABLE).wrapping_add((idx as usize).wrapping_mul(staff::SIZE));
        if *((person + staff::CLASSIFICATION) as *const u8) != 1 {
            continue;
        }
        crate::comparison::branch("cqc_13 human scouts");
        let marks = exe::crt_calloc(10, 4);
        for _ in 0..10 { // cqc13: os 10 observadores
            let q = loop {
                let q = exe::utils_random(10) as i8; // cqc13: o observador sorteado
                if read_usize(marks.wrapping_add((q as isize * 4) as usize)) == 0 {
                    break q; // cqc13: one not yet marked
                }
            };
            core::ptr::write_unaligned(marks.wrapping_add((q as isize * 4) as usize) as *mut u32, 1);
            // simplification: the original pushes the dword with the observer byte and stack leftover above; 0x7f6cd0 reads the
            // byte
            let r = exe::scout_manager_get_search(this, q as u8 as u32, person);
            if read_usize(r + 0x19d) != 0 && *((p + fixture::HOME_SIDE_MARK) as *const u8) != 0 && *((p + fixture::AWAY_SIDE_MARK) as *const u8) != 0 {
                let club = read_usize(person + staff::CLUB_JOB);
                if read_usize(p + fixture::HOME_CLUB) == club || read_usize(p + fixture::AWAY_CLUB) == club {
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
///
/// [pt] O índice (0..15) do técnico `tecnico` (nulo: o humano da vez, [0xb74c26 + 0x300 × [0xb77c7a]]) pela conta de um
/// byte da original ((id − [0xae2364]) + 16 em 8 bits); None nos casos da caixa (sem técnico ou fora).
pub(crate) unsafe fn scout_manager_human_manager_index(manager: usize) -> Option<usize> {
    let t = if manager != 0 {
        manager
    } else {
        read_usize(G_SCREEN_RECORDS_MANAGER.wrapping_add((*(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as i32 as usize).wrapping_mul(0x300)))
    };
    if t == 0 {
        return None;
    }
    let i = (*(t as *const u8)).wrapping_sub(*(G_STAFF_COUNT as *const u8)).wrapping_add(0x10) as i8;
    if (0..=0xf).contains(&i) { Some(i as usize) } else { None }
}

/// Returns the human manager's searches edit session.
///
/// Ret 4: `SCOUT_MANAGER::get_edit_session(manager)`, from building the search's known players (0x7afeb7). The human
/// manager's edit session (+0xc + 4 × index). No manager or index out of range: the box (lines 0xde and 0xe8; the
/// original) and 0.
/// Original: `SCOUT_MANAGER::get_edit_session` (0x7f6be0, scout_manager.cpp, thiscall).
///
/// [pt] { 0x7f6be0 } (thiscall, ret 4, scout_manager.cpp): `SCOUT_MANAGER::get_edit_session(tecnico)`, pela montagem dos
/// conhecidos da busca (0x7afeb7). A sessão de edição do técnico humano (+0xc + 4 × índice). Sem técnico ou índice
/// fora: a caixa (linhas 0xde e 0xe8; a original) e 0.
pub(crate) unsafe extern "thiscall" fn get_edit_session(this: usize, manager: usize) -> usize {
    let Some(i) = scout_manager_human_manager_index(manager) else {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_GET_EDIT_SESSION)(this, manager); // a caixa
    };
    read_usize(this + scout_manager_fields::EDIT_SESSIONS + i * 4)
}

/// Stores the human manager's searches edit session and returns 1.
///
/// Ret 8: `SCOUT_MANAGER::set_edit_session(session, manager)`, from the search screens (0x8085bc, 0x812d88...). Writes
/// the session into +0xc + 4 × the manager's index and returns 1. No manager or index out of range: the box (lines 0xb8
/// and 0xc2; the original) and 0.
/// Original: `SCOUT_MANAGER::set_edit_session` (0x7f6af0, scout_manager.cpp, thiscall).
///
/// [pt] { 0x7f6af0 } (thiscall, ret 8, scout_manager.cpp): `SCOUT_MANAGER::set_edit_session(sessao, tecnico)`, pelas telas
/// das buscas (0x8085bc, 0x812d88...). Grava a sessão em +0xc + 4 × índice do técnico e devolve 1. Sem técnico ou
/// índice fora: a caixa (linhas 0xb8 e 0xc2; a original) e 0.
pub(crate) unsafe extern "thiscall" fn set_edit_session(this: usize, sessao: usize, manager: usize) -> u32 {
    let Some(i) = scout_manager_human_manager_index(manager) else {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_SET_EDIT_SESSION)(this, sessao, manager); // a caixa
    };
    core::ptr::write_unaligned((this + scout_manager_fields::EDIT_SESSIONS + i * 4) as *mut usize, sessao);
    1
}

/// As datas estáticas da `exe::date_add_weeks` (valor e guarda) e da `exe::date_add_days`, 0x54c8e0 e `exe::date_days_since`.
pub(crate) const SCOUT_MANAGER_DATE_STATICS: [(usize, usize); 8] = [(G_ADD_DAYS_RESULT_DATE, 8), (0xae2469, 1), (G_SUB_DAYS_RESULT_DATE, 8), (0xae249e, 1), (G_ADD_WEEKS_RESULT_DATE, 8), (0xae2430, 1), (G_DAYS_SINCE_SCRATCH_DATE, 8), (0xae2418, 1)];

/// Body of Friendly::update_international_friendlies (`update_international_friendlies`); in dry run returns the (first entry, count) that would
/// go to `arrange_international_friendlies`.
///
/// [pt] O corpo da 0x5cbb00. No `ensaio` (o comparador), nada se grava e nada se chama além das consultas: devolve o par
/// (primeira entrada, quantas) que iria à 0x5cbdf0, ou None se a original não a chamaria.
pub(crate) unsafe fn update_friendlies_body(this: usize, dry_run: bool) -> Option<(usize, u32)> {
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
        crate::comparison::branch("cpy_4 year rollover");
        let mut i = 0i32;
        while i < read_usize(G_NATION_COUNT) as i32 {
            let l = read_usize(read_usize(this + friendly::NATION_LISTS).wrapping_add(i as usize * 4));
            if l != 0 {
                exe::llist_clear(l);
            }
            i += 1;
        }
        exe::crt_qsort(read_usize(this + friendly::SORTED_NATIONS), read_usize(this + friendly::SORTED_NATION_COUNT) as u32, 4, 0x5cc5f0); // cpy4: the nations by reputation
        // the first World Cup year (year − 2 a multiple of 4) from this one; the high half of the dword the original adds is
        // stack leftover: here, zero
        let mut year = read_u16(G_CURRENT_DATE_YEAR) as u32;
        while ((year & 0xffff) as i32 - 2) % 4 != 0 {
            year = year.wrapping_add(1);
        }
        let mut k = 0i32;
        while k < read_usize(this + friendly::INTL_DATE_COUNT) as i32 {
            let e = read_usize(this + friendly::INTL_DATE_TABLE).wrapping_add(k as usize * 0x14);
            let mut a = ((*((e + 0xb) as *const i8)) as i16 as u16 as u32).wrapping_add(year);
            if (a & 0xffff) as i32 - read_u16(G_CURRENT_DATE_YEAR) as i32 > 3 {
                a = a.wrapping_add(0xfffc);
            }
            exe::friendly_international_date_resolve(e, a);
            k += 1;
        }
        exe::crt_qsort(read_usize(this + friendly::INTL_DATE_TABLE), read_usize(this + friendly::INTL_DATE_COUNT) as u32, 0x14, 0x5cc610);
    }
    // the date of 0xdd6ec8 after today + 12 weeks: nothing
    let x_year = read_u16(G_GAME_START_DATE_YEAR);
    let l_year = core::ptr::read_unaligned(lim.as_ptr().add(2) as *const u16);
    if x_year > l_year || (x_year == l_year && *(G_GAME_START_DATE as *const i16) > core::ptr::read_unaligned(lim.as_ptr() as *const i16)) {
        return None;
    }
    if read_u16(this + friendly::INTL_LAST_DAY_DONE) as u32 == today as i16 as i32 as u32 {
        return None;
    }
    let mut lixo = [0u8; 8];
    exe::date_operator_3d(d, (&mut lixo) as *mut [u8; 8] as usize, lim.as_ptr() as usize);
    let base = read_usize(this + friendly::INTL_DATE_TABLE);
    let n = read_usize(this + friendly::INTL_DATE_COUNT);
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
    crate::comparison::branch("cpy_4 friendlies scheduled");
    exe::friendly_arrange_international_friendlies(this, first, how_many); // cpy4: schedule the friendlies of the date
    *((this + friendly::INTL_LAST_DAY_DONE) as *mut u16) = read_u16(G_CURRENT_DATE);
    Some((first, how_many))
}

/// Index (id - G_STAFF_COUNT + 16) of the manager (null: current human) among the 16 humans; None on error.
///
/// [pt] O técnico (nulo: o humano da vez, [0xb74c26 + 0x300 × [0xb77c7a]]) e o índice dele entre os 16 humanos (id −
/// [0xae2364] + 16), ou None nos casos de erro da original (sem técnico ou fora de 0..15).
pub(crate) unsafe fn manager_human_index(manager: usize) -> Option<usize> {
    let t = if manager != 0 {
        manager
    } else {
        read_usize(G_SCREEN_RECORDS_MANAGER.wrapping_add((*(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as i32 as usize).wrapping_mul(0x300)))
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
/// manager (byte at +0x4c + index). No manager or outside 0..15: the original routine ("Error" box, 0xff).
/// Original: `SCOUT_MANAGER::get_selected_list` (0x7f6e30, scout_manager.cpp, thiscall).
///
/// [pt] { 0x7f6e30 } (thiscall, ret 4, scout_manager.cpp): `SCOUT_MANAGER::get_selected_list(tecnico)`, pelas telas de
/// busca (0x804248). A lista escolhida pelo técnico humano (byte em +0x4c + índice). Sem técnico ou fora de 0..15: a
/// original (caixa "Error", 0xff).
pub(crate) unsafe extern "thiscall" fn get_selected_list(this: usize, manager: usize) -> u8 {
    let Some(i) = manager_human_index(manager) else {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_GET_SELECTED_LIST)(this, manager); // message box: original only
    };
    *((this + scout_manager_fields::SELECTED_LISTS + i) as *const u8)
}

/// Stores the list selected by the human manager (null: the current one).
///
/// Ret 8: `SCOUT_MANAGER::set_selected_list(list, manager)`, from the search screens (0x804460). Writes the chosen list
/// (byte) into +0x4c + the human manager's index and returns 1. No manager or outside 0..15: the original routine
/// ("Error" box, 0).
/// Original: `SCOUT_MANAGER::set_selected_list` (0x7f6f20, scout_manager.cpp, thiscall).
///
/// [pt] { 0x7f6f20 } (thiscall, ret 8, scout_manager.cpp): `SCOUT_MANAGER::set_selected_list(lista, tecnico)`, pelas telas
/// de busca (0x804460). Grava a lista escolhida (byte) em +0x4c + índice do técnico humano e devolve 1. Sem técnico ou
/// fora de 0..15: a original (caixa "Error", 0).
pub(crate) unsafe extern "thiscall" fn set_selected_list(this: usize, list: u32, manager: usize) -> u32 {
    let Some(i) = manager_human_index(manager) else {
        return original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> u32>(IDX_SET_SELECTED_LIST)(this, list, manager); // message box: original only
    };
    *((this + scout_manager_fields::SELECTED_LISTS + i) as *mut u8) = list as u8;
    1
}

/// Tells whether the fixture has a watched club or an available player with a scout request (at the club or national
/// side, not loaned out, injured, suspended or barred, and not yet played).
///
/// Ret 4: `SCOUT_MANAGER::fixture_has_scouted_player(fixture)` (from match_without_human `exe::match_day_can_quick_simulate`): negative
/// competition (+0), 0. A club of the watched list (static LLIST 0xb74438, cursor reset; +0 of the item) equal to +0xc
/// or +0x10 of the fixture, 1. Then the scouting requests (`[this+0x5c]`, 0x11 bytes, the person at +4, count
/// `[this+0x64]`, reread on each step): a person whose club (+0x39) or national side (+0x24) is one of the two in the
/// fixture (+0x1c first, then +0x20), who is not loaned out (`player_needs_work_permit`, outside a national side `exe::database_is_national_team`), nor injured
/// (0x62c400 = 2), nor suspended (`is_player_suspended`), nor barred (`get_international_duty_club`), and who has not played yet (0x8cf5e0 = 0), gives 1.
/// Otherwise 0.
/// Original: `SCOUT_MANAGER::fixture_has_scouted_player` (0x7f9720, scout_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x7f9720 } (thiscall, ret 4, scout_manager.cpp): `SCOUT_MANAGER::fixture_has_scouted_player(partida)` (pela
/// match_without_human 0x6b66a0): competição (+0) negativa, 0. Um clube da lista vigiada (LLIST estática 0xb74438,
/// cursor reposto; +0 do item) igual a +0xc ou +0x10 da partida, 1. Depois os pedidos de observação ([this+0x5c],
/// 0x11 bytes, a pessoa em +4, conta [this+0x64], relidos a cada passo): a pessoa cujo clube (+0x39) ou seleção (+0x24)
/// é um dos dois da partida (+0x1c primeiro, depois +0x20), que não está cedida (0x8c7560, fora de seleção 0x53b900),
/// nem lesionada (0x62c400 = 2), nem suspensa (0x55d270), nem impedida (0x7757d0), e que ainda não jogou (0x8cf5e0
/// = 0), dá 1. Senão 0.
pub(crate) unsafe extern "thiscall" fn fixture_has_scouted_player(this: usize, partida: usize) -> u32 {
    let (c1, c2) = (read_usize(partida + fixture::HOME_CLUB), read_usize(partida + fixture::AWAY_CLUB));
    if read_i32(partida) < 0 {
        crate::comparison::branch("cps_8 negative competition");
        return 0; // cps8: negative competition
    }
    core::ptr::write_unaligned(G_SCOUT_CLUB_TABLE_LIST_CURSOR as *mut u32, read_usize(G_SCOUT_CLUB_TABLE_LIST_HEAD) as u32);
    let mut e = exe::llist_get_next(G_SCOUT_CLUB_TABLE_LIST);
    while e != 0 {
        let v = read_usize(e);
        if read_usize(partida + fixture::HOME_CLUB_ID) == v || read_usize(partida + fixture::AWAY_CLUB_ID) == v {
            return 1;
        }
        e = exe::llist_get_next(G_SCOUT_CLUB_TABLE_LIST);
    }
    if read_usize(this + scout_manager_fields::SCOUT_REQUESTS) == 0 || read_i32(this + scout_manager_fields::SCOUT_REQUEST_COUNT) <= 0 {
        crate::comparison::branch("cps_8 no requests");
        return 0; // cps8: without scouting requests
    }
    let mut k = 0i32;
    while k < read_i32(this + scout_manager_fields::SCOUT_REQUEST_COUNT) {
        let id = read_i32(read_usize(this + scout_manager_fields::SCOUT_REQUESTS).wrapping_add(k as usize * 0x11) + 4);
        k += 1;
        if id <= -1 {
            continue;
        }
        let p = read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(staff::SIZE));
        let (clube, sel) = (read_usize(p + staff::CLUB_JOB), read_usize(p + staff::NATIONAL_JOB));
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
        if exe::discipline_is_player_banned(read_usize(G_DISCIPLINE), p, read_usize(partida + fixture::COMPETITION), 1, 0) != 0 {
            continue;
        }
        if exe::nationalteams_get_international_duty_club(read_usize(G_NATIONAL_TEAMS), p, G_CURRENT_DATE, x, 0, 0) != 0 {
            continue;
        }
        if exe::transfer_manager_get_ineligibility_code(read_usize(G_TRANSFER_MANAGER), p, partida) == 0 {
            return 1;
        }
    }
    0
}

/// The current human's manager record ([0xb74c26 + 0x300 x current screen slot]).
///
/// [pt] O gerente do humano da vez ([0xb74c26 + 0x300 × (i16)[0xb77c7a]]).
pub(crate) unsafe fn scout_manager_current_human_manager() -> usize {
    let k = core::ptr::read_unaligned(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize;
    read_usize((0xb74c26isize + 0x300 * k) as usize)
}

/// Tells whether the player is on the human's scouting shortlist (through the open edit session or the first type 1
/// search).
///
/// Ret 8: `SCOUT_MANAGER::is_player_shortlisted(person, manager)` (from the interested clubs `collect_interested_clubs`, from the status
/// `shortlist_manager_estimate_squad_status`, from `is_player_under_offer` and from screens): null person, the "Error" box (line 0x231) and 0; without a manager, that
/// of the human whose turn it is (without him, the box at line 0x23b and 0). The human's slot (byte +0 of the manager −
/// byte of `[0xae2364]` + 16, signed, 0..15): with the edit session open (`[this + slot × 4 + 0xc]`), its answer
/// (`exe::searcheditsession_is_player_on_short_list`); otherwise the first of the 10 searches (`scout_record`) of type 1 (+0xa) answers (`contains_player`). Without it, 0.
/// Original: `SCOUT_MANAGER::is_player_shortlisted` (0x7f7490, scout_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x7f7490 } (thiscall, ret 8, scout_manager.cpp): `SCOUT_MANAGER::is_player_shortlisted(pessoa, gerente)` (pelos
/// clubes interessados 0x842180, pelo estatuto 0x849d60, pela 0x8cde70 e por telas): pessoa nula, a caixa "Error"
/// (linha 0x231) e 0; sem gerente, o do humano da vez (sem ele, a caixa da linha 0x23b e 0). A vaga do humano (byte
/// +0 do gerente − byte de [0xae2364] + 16, com sinal, 0..15): com a edição aberta ([this + vaga × 4 + 0xc]), a
/// resposta dela (0x804540); senão a primeira das 10 buscas (0x7f6cd0) de tipo 1 (+0xa) responde (0x7af600). Sem ela, 0.
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
    let slot = (*(m as *const u8)).wrapping_sub(*(G_STAFF_COUNT as *const u8)).wrapping_add(0x10) as i8;
    if !(0..=15).contains(&slot) {
        return 0;
    }
    let s = read_usize(this + slot as usize * 4 + 0xc);
    if s != 0 {
        return exe::searcheditsession_is_player_on_short_list(s, person);
    }
    // simplification: the original pushes a local dword with the index only in the low byte; 0x7f6cd0 reads only that byte
    for i in 0..10u32 {
        let l = exe::scout_manager_get_search(this, i, m);
        if l != 0 && *((l + 0xa) as *const u8) == 1 {
            let r = exe::player_search_contains_player(l, person);
            crate::comparison::branch("cps_8 scout list");
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
/// + 0x300 × [0xb77c7a]]`): the index (id − `[0xae2364]` + 16) × 10 + `which`, up to `[this+8]` − 1, with 0x2b0 bytes
/// each starting at `[this]`. `which` out of range, no manager or index out of range: the original routine (error box).
/// Original: `scout_record` (0x7f6cd0, scout_manager.cpp, thiscall).
///
/// [pt] { 0x7f6cd0 } (thiscall, ret 8, scout_manager.cpp): o registro de observação `qual` 0..9 do técnico
/// humano `tecnico` (sem ele, o da vez em [0xb74c26 + 0x300 × [0xb77c7a]]): o índice (id −
/// [0xae2364] + 16) × 10 + `qual`, até [this+8] − 1, com 0x2b0 bytes cada a partir de [this].
/// `qual` fora, sem técnico ou índice fora: a original (caixa de erro).
pub(crate) unsafe extern "thiscall" fn scout_record(t: usize, which: i8, manager: *const i32) -> usize {
    let mut p = manager;
    if (0..=9).contains(&which) && p.is_null() {
        let team = *(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize;
        p = core::ptr::read_unaligned((0xb74c26 + team * 0x300) as usize as *const *const i32);
    }
    if !(0..=9).contains(&which) || p.is_null() {
        return original_fn::<unsafe extern "thiscall" fn(usize, i8, *const i32) -> usize>(IDX_SCOUT_RECORD)(t, which, manager);
    }
    let i = ((*p).wrapping_sub(*(G_STAFF_COUNT as *const i32)).wrapping_add(0x10)).wrapping_mul(10).wrapping_add(which as i32);
    if i < 0 || i > (*((t + 8) as *const i32)).wrapping_sub(1) {
        return original_fn::<unsafe extern "thiscall" fn(usize, i8, *const i32) -> usize>(IDX_SCOUT_RECORD)(t, which, manager);
    }
    crate::comparison::branch("scout record");
    (*(t as *const usize)).wrapping_add(i as usize * 0x2b0)
}

/// Empties the list and adds the person of each scout request of the club; returns whether there is any.
///
/// Ret 8: `SCOUT_MANAGER::collect_club_scout_targets(club, list)`: with `list`, empties it (`manager_manager_clear`) and adds
/// (0x68d0d0, 0x6e bytes) the person of each scouting request of club `*club`. Returns whether there is any. Null club:
/// 0 (the list stays empty).
/// Original: `SCOUT_MANAGER::collect_club_scout_targets` (0x7f9690, scout_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x7f9690 } (thiscall, ret 8, scout_manager.cpp): `SCOUT_MANAGER::collect_club_scout_targets(clube, lista)`: com
/// `lista`, esvazia-a (0x68d080) e acrescenta (0x68d0d0, 0x6e bytes) a pessoa de cada pedido de observação do clube
/// `*clube`. Devolve se há algum. Clube nulo: 0 (a lista fica vazia).
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
/// (`[0xb74c26 + 0x300 × [0xb77c7a]]`, +0x39; without him, nothing). With the person, searches (bsearch `exe::crt_bsearch` with
/// 0x7fc270, which reads only +0 club and +4 person; the key's default date at +8, byte +0x10 has no value and the copy
/// uses zero) for the request in the vector (`[this+0x5c]`, `[this+0x64]` records of 17 bytes); found: pulls the
/// following ones up one place (+0, +4, the date +8 through `exe::date_operator_3d` and byte +0x10), the count goes down and, if any
/// remain, sorts again (qsort).
/// Original: `SCOUT_MANAGER::remove_scout_request` (0x7f8ee0, scout_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x7f8ee0 } (thiscall, ret 8, scout_manager.cpp): `SCOUT_MANAGER::remove_scout_request(pessoa, clube)`: o clube
/// nulo é o do técnico humano atual ([0xb74c26 + 0x300 × [0xb77c7a]], +0x39; sem ele, nada). Com a pessoa, busca
/// (bsearch 0x946464 com 0x7fc270, que lê só +0 clube e +4 pessoa; a data padrão da chave em +8, o byte +0x10 sem valor
/// e a cópia usa zero) o pedido no vetor ([this+0x5c], [this+0x64] registros de 17 bytes); achou: puxa os seguintes um
/// lugar (+0, +4, a data +8 pela 0x417c10 e o byte +0x10), a contagem desce e, sobrando algum, reordena (qsort).
pub(crate) unsafe extern "thiscall" fn remove_scout_request(this: usize, person: usize, club: usize) {
    let mut key = [0u8; 0x11];
    exe::date_date_54a110(key.as_mut_ptr() as usize + 8);
    let mut c = club;
    if c == 0 {
        let i = *(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as i32;
        let manager = read_u32((0xb74c26 + i * 0x300) as usize) as usize; // the current human manager
        if manager == 0 {
            return;
        }
        c = read_u32(manager + staff::CLUB_JOB) as usize;
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
        core::ptr::write_unaligned(r as *mut u32, read_u32(r + 0x11));
        core::ptr::write_unaligned((r + 4) as *mut u32, read_u32(r + 0x15));
        let mut local = [0u8; 8];
        exe::date_operator_3d(r + 8, (&mut local) as *mut [u8; 8] as usize, r + 0x19);
        *((r + 0x10) as *mut u8) = *((r + 0x21) as *const u8);
        k += 1;
    }
    let n = read_u32(this + scout_manager_fields::SCOUT_REQUEST_COUNT).wrapping_sub(1);
    core::ptr::write_unaligned((this + scout_manager_fields::SCOUT_REQUEST_COUNT) as *mut u32, n); // um pedido a menos
    if n != 0 {
        media_crt_qsort(read_u32(this + scout_manager_fields::SCOUT_REQUESTS) as usize, n as i32, 0x11, 0x7fc270);
    }
}

/// Tells whether the club can scout the nation: a poor (balance < 1,000,000) and small (reputation < 5000) club outside
/// the nation's region: 0; else 1.
///
/// Ret 8; `this` is not read: `SCOUT_MANAGER::can_club_scout_nation(nation, club)`: the club (or that of the human
/// whose turn it is) does not cover the nation only if it has finances (`get_club_finance` on `[0xae2a38]`) with a 64-bit balance
/// below 1,000,000, reputation (+0x80) below 5000 and the region (+0x76) of its nation (+0x53) different from the
/// region of `nation`: 0. Otherwise 1. No club at all: the original routine ("Error" box and 0).
/// Original: `SCOUT_MANAGER::can_club_scout_nation` (0x7f9190, scout_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x7f9190 } (thiscall, ret 8, scout_manager.cpp; `this` não é lido): `SCOUT_MANAGER::can_club_scout_nation(nacao,
/// clube)`: o clube (ou o do humano da vez) não cobre a nação só se tem as finanças (0x5a0590 em [0xae2a38]) com saldo
/// de 64 bits abaixo de 1.000.000, reputação (+0x80) abaixo de 5000 e a região (+0x76) da nação dele (+0x53) diferente
/// da região da `nacao`: 0. Senão 1. Sem clube nenhum: a original (caixa "Error" e 0).
pub(crate) unsafe extern "thiscall" fn can_club_scout_nation(this: usize, nation: usize, club: usize) -> u32 {
    let c = club_or_current_human(club);
    if c == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_CAN_CLUB_SCOUT_NATION)(this, nation, club); // message box: original only
    }
    let f = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), c);
    if f != 0 && core::ptr::read_unaligned(f as *const i64) < 1_000_000 && read_i16(c + club::REPUTATION) < 5000 {
        if *((read_usize(c + club::NATION) + 0x76) as *const u8) != *((nation + nation::ACTUAL_REGION) as *const u8) { // outside the region
            crate::comparison::branch("cpk_6 scout outside the region");
            return 0;
        }
    }
    1
}

/// Removes from the manager's scout lists the scouts no longer at the club and gives each club scout without a list the
/// first free list; with the edit session open, it refreshes the scouts.
///
/// Ret 4: `SCOUT_MANAGER::sync_scout_lists(manager)`, when the staff of the human club changes (leaving the post
/// `remove_staff_from_club` and the scouts' vacancy, staff transfers and signings). Without a manager, the human whose turn it is;
/// without him, the box at line 0x1ba and 0. Of the manager's 10 searches, the scout lists (+0xa = 4) whose scout
/// (`get_scout`) is no longer a scout (+0x3d = 9) of the manager's club (+0x39) are left without a scout (`set_scout` with 0;
/// without a club, all of them; with a club, those without a scout stay as they are). With a club, each of the club's 7
/// scouts (`database_get_club_scouts`) that has no list goes to the first scout list without a scout. Then the manager's slot (byte +0 −
/// byte of `[0xae2364]` + 16, 0..15; out of range, the box at line 0x212 and 0): with the edit session open (`[this +
/// slot × 4 + 0xc]`), it redoes the lists' scouts (`exe::searcheditsession_refresh_scout_assignments`). Returns 1.
/// Original: `SCOUT_MANAGER::sync_scout_lists` (0x7f7270, scout_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x7f7270 } (thiscall, ret 4, scout_manager.cpp): `SCOUT_MANAGER::sync_scout_lists(técnico)`, quando a comissão do
/// clube humano muda (a saída do cargo 0x690890 e a da vaga dos olheiros, as transferências e contratações de pessoal).
/// Sem técnico, o humano da vez; sem ele, a caixa da linha 0x1ba e 0. Das 10 buscas do técnico, as listas de olheiro
/// (+0xa = 4) cujo olheiro (0x7ae950) não é mais olheiro (+0x3d = 9) do clube do técnico (+0x39) ficam sem olheiro
/// (0x7ae8a0 com 0; sem clube, todas; com clube, as sem olheiro ficam como estão). Com clube, cada um dos 7 olheiros do
/// clube (0x53cd60) que não tem lista vai para a primeira lista de olheiro sem olheiro. Depois a vaga do técnico (byte
/// +0 − byte de [0xae2364] + 16, 0..15; fora, a caixa da linha 0x212 e 0): com a edição aberta ([this + vaga × 4 +
/// 0xc]), ela refaz os olheiros das listas (0x8043c0). Devolve 1.
pub(crate) unsafe extern "thiscall" fn sync_scout_lists(this: usize, manager: usize) -> u32 {
    let m = if manager != 0 { manager } else { scout_manager_current_human_manager() };
    if m == 0 {
        show_error_at(0xa7b094, 0x1ba);
        return 0;
    }
    let club = read_usize(m + staff::CLUB_JOB);
    for i in 0..10u8 {
        let l = get_search(this, i, m);
        if l == 0 || read_u8(l + 0xa) != 4 {
            continue;
        }
        let o = search_scout(l);
        if club != 0 {
            if o == 0 || (read_usize(o + staff::CLUB_JOB) == club && read_u8(o + staff::JOB_FOR_CLUB) == 9) {
                continue;
            }
        }
        crate::comparison::branch("cqa_6 list without scout");
        set_search_scout(l, 0);
    }
    if club != 0 {
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
                if l == 0 || read_u8(l + 0xa) != 4 {
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
                crate::comparison::branch("cqa_6 scout in the free list");
                set_search_scout(get_search(this, free as u8, m), s);
            }
        }
    }
    let slot = read_u8(m).wrapping_sub(read_u8(G_STAFF_COUNT)).wrapping_add(0x10) as i8;
    if !(0..=15).contains(&slot) {
        show_error_at(0xa7b094, 0x212);
        return 0;
    }
    let s = read_usize(this + slot as usize * 4 + 0xc);
    if s != 0 {
        exe::searcheditsession_refresh_scout_assignments(s);
    }
    1
}

/// Returns the player's slot in the record of the human's search that scouts his club, or 0.
///
/// Ret 4: `SCOUT_MANAGER::find_club_search_entry(person)`, from the person and search screens (0x808540, 0x813210,
/// 0x870650, 0x87a510): without the human whose turn it is or without the person's club (+0x39), 0. The human's slot
/// (byte +0 − byte of `[0xae2364]` + 16, 0..15; out of range, the box at line 0x4b5 and 0). The first of the 10
/// searches (`scout_record`) with the scouted club (+0x190) equal to the person's and its record (+0x170) gives the person's
/// slot in it (`player_search_find_or_claim_player_slot` without reserving); none, 0.
/// Original: `SCOUT_MANAGER::find_club_search_entry` (0x7f8610, scout_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x7f8610 } (thiscall, ret 4, scout_manager.cpp): `SCOUT_MANAGER::find_club_search_entry(pessoa)`, das telas da
/// pessoa e da busca (0x808540, 0x813210, 0x870650, 0x87a510): sem o humano da vez ou sem o clube da pessoa (+0x39), 0.
/// A vaga do humano (byte +0 − byte de [0xae2364] + 16, 0..15; fora, a caixa da linha 0x4b5 e 0). A primeira das 10
/// buscas (0x7f6cd0) com o clube observado (+0x190) igual ao da pessoa e o registro dele (+0x170) dá a vaga da pessoa
/// nele (0x7bd700 sem reservar); nenhuma, 0.
pub(crate) unsafe extern "thiscall" fn find_club_search_entry(this: usize, p: usize) -> usize {
    let m = scout_manager_current_human_manager();
    if m == 0 || read_usize(p + staff::CLUB_JOB) == 0 {
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
        if l == 0 || read_usize(l + 0x190) != read_usize(read_usize(p + staff::CLUB_JOB)) || read_usize(exe::scout_manager_get_search(this, i, m) + 0x170) == 0 {
            continue;
        }
        return exe::player_search_find_or_claim_player_slot(read_usize(exe::scout_manager_get_search(this, i, m) + 0x170), p, 0);
    }
    0
}

/// Writes the translated scout list name: Short List, Player Search, Staff Search or Scout #n.
///
/// `scout_manager_write_list_name(index, text, size)`, from the SCOUT_MANAGER constructor (0x7f6760) and from the
/// search screens. The translated name (0x66f4e0) of list `index`: 0 "Short List", 1 "Player Search", 2 "Staff Search",
/// 3..9 "Scout #<%d - number>" with `index` − 2. Returns 1. Index outside 0..9, null text or size below 50: the
/// original routine ("Error" box, 0).
/// Original: `scout_manager_write_list_name` (0x7fc150, scout_manager.cpp, cdecl).
///
/// [pt] { 0x7fc150 } (cdecl, scout_manager.cpp): `scout_manager_write_list_name(indice, texto, tamanho)`, pelo construtor do
/// SCOUT_MANAGER (0x7f6760) e pelas telas de busca. O nome traduzido (0x66f4e0) da lista `indice`: 0 "Short List", 1
/// "Player Search", 2 "Staff Search", 3..9 "Scout #<%d - number>" com `indice` − 2. Devolve 1. Índice fora de 0..9,
/// texto nulo ou tamanho abaixo de 50: a original (caixa "Error", 0).
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
/// Ret 0xc: `SCOUT_MANAGER::has_scout_request(player, club, only_due)`: whether the club (null: that of the human whose
/// turn it is, `[0xb74c26 + 0x300 × [0xb77c7a]]` +0x39) asked for the player to be scouted: binary search (`exe::crt_bsearch`
/// with 0x7fc270) by the pair (club id, player id) in the requests (+0x5c, count +0x64, 17 bytes) with fewer than 3
/// reports (+0x10, signed); with `only_due`, only if the request date (+8: day, +0xa: year) is before today
/// (`[0xae2c90]`, unsigned year, signed day). Returns 1 or 0. Null player: the original routine (box at line 0x5da).
/// The original's local date (`exe::date_date_54a110`) only fills the key after the pair.
/// Original: `SCOUT_MANAGER::has_scout_request` (0x7f8d70, scout_manager.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x7f8d70 } (thiscall, ret 0xc, scout_manager.cpp): `SCOUT_MANAGER::has_scout_request(jogador, clube, so_vencidos)`:
/// se o clube (nulo: o do humano da vez, [0xb74c26 + 0x300 × [0xb77c7a]] +0x39) pediu a observação do jogador: busca
/// binária (0x946464 com 0x7fc270) pelo par (id do clube, id do jogador) nos pedidos (+0x5c, conta +0x64, 17 bytes)
/// com menos de 3 relatórios (+0x10, com sinal); com `so_vencidos`, só se a data do pedido (+8: dia, +0xa: ano) é
/// anterior a hoje ([0xae2c90], ano sem sinal, dia com sinal). Devolve 1 ou 0. Jogador nulo: a original (caixa da
/// linha 0x5da). A data local da original (0x54a110) só ocupa a chave depois do par.
pub(crate) unsafe extern "thiscall" fn has_scout_request(this: usize, player: usize, club: usize, so_vencidos: u32) -> u32 {
    if player == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32>(IDX_HAS_SCOUT_REQUEST)(this, player, club, so_vencidos);
    }
    let mut club = club;
    if club == 0 {
        let human = read_ptr(G_SCREEN_RECORDS_MANAGER + 0x300 * (*(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize as usize));
        if human == 0 {
            return 0;
        }
        club = read_ptr(human + staff::CLUB_JOB);
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
        crate::comparison::branch("cpq_12 without request");
        return 0; // no request
    }
    if *((found + scout_request::REPORT_COUNT) as *const i8) >= 3 {
        return 0;
    }
    if so_vencidos != 0 {
        let year = core::ptr::read_unaligned((found + scout_request::DATE_YEAR) as *const u16);
        let today = *(G_CURRENT_DATE_YEAR as *const u16);
        if year > today {
            return 0;
        }
        if year == today && core::ptr::read_unaligned((found + scout_request::DATE) as *const i16) >= *(G_CURRENT_DATE as *const i16) {
            return 0;
        }
    }
    1
}

