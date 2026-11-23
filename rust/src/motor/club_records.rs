use super::*;

/// Adds v to the byte at address a (wrapping).
pub(crate) unsafe fn add_u8_at(a: usize, v: u8) {
    write_u8(a, read_u8(a).wrapping_add(v));
}

/// Adds v to the unaligned u16 at address a (wrapping).
pub(crate) unsafe fn add_u16_at(a: usize, v: u16) {
    write_u16(a, read_u16(a).wrapping_add(v));
}

/// Adds v to the unaligned f64 at address a.
pub(crate) unsafe fn add_f64_at(a: usize, v: f64) {
    write_f64(a, v + read_f64(a));
}

/// PLAYERSTATS get_category_record(player, category).
pub(crate) unsafe fn player_category_record(this: usize, player: usize, cat: u8) -> usize {
    exe::playerstats_get_category_record(this, player, cat as u32)
}

/// The statistics category of side `lado` of a match (the `bl` of `0x7c0d70`, a match whose competition ≠ −2).
///
/// Friendly (`0x5c6710` of [`0xae2c74`]): 5 if the home side is a national team (`0x53b900`), otherwise 1. Reserves,
/// under-21 and B ([`0x9cf684`], [`0x9cf798`], [`0x9cf794`]), or the side's club without the mark `+0x40` (home) /
/// `+0x41` (away): 1.
///
/// By the type (`+0x42`) of the competition ([`0xadadfc`] by id `+0`): 4 → 5, 2 → 4, 1 → 2 if it counts (`0x4b0700`),
/// otherwise 3.
///
/// With no competition or any other type: the box (lines `0x103a`, `0x105d`; only with `avisar`), sets `erro` and
/// returns 2.
pub(crate) unsafe fn match_stats_category(p: Fixture, side: i32, warn: bool, error: &mut bool) -> u8 {
    let club = read_usize(p.addr() + if side == 0 { 0x1c } else { 0x20 });
    if exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), p.competition()) != 0 {
        return if exe::database_is_national_team(p.home_club()) != 0 { 5 } else { 1 };
    }
    let id = read_usize(p.competition().addr());
    if id == read_usize(KEY_CLUB_COMP_RESERVE) || id == read_usize(KEY_CLUB_COMP_UNDER_21_INTERNATIONAL) || id == read_usize(KEY_CLUB_COMP_B_INTERNATIONAL) {
        return 1;
    }
    if club == p.home_club().addr() && p.home_side_mark() == 0 {
        return 1;
    }
    if club == p.away_club().addr() && (p.away_side_mark() as u8) == 0 {
        return 1;
    }
    let comp = Cup(read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(p.addr()).wrapping_mul(4))));
    if comp.is_null() {
        *error = true;
        if warn {
            show_error_at(0xa78310, 0x103a);
        }
        return 2;
    }
    match comp.club_sides() {
        4 => 5,
        2 => 4,
        1 => {
            if warn {
                covered!("cpx_12 league category");
            }
            if exe::comp_util_is_league_division(read_usize(p.addr()) as i32) != 0 { 2 } else { 3 } // cpx12: the league category
        }
        _ => {
            *error = true;
            if warn {
                show_error_at(0xa78310, 0x105d);
            }
            2
        }
    }
}

/// Tells whether lineup row `k` of side `lado` of the match counts, and returns it.
///
/// Rows are `k` = 0..20, 0x49 bytes each; 11 and up are substitutes. A row counts when: a substitute actually came on
/// (+0x32 ≠ 0xff), the person (+0) and the player (+4) are ≠ −1, and the person is in the table ([0xae23bc], 0x6e
/// bytes; id below [0xae2364]). Returns (row, person, player data in [0xae23c4], 0x46 bytes).
pub(crate) unsafe fn match_lineup_row(game: usize, side: usize, k: usize) -> Option<(usize, usize, usize)> {
    let e = game + match_stats_record::SIDE_SIZE * side + match_stats_record::HOME_LINEUP + match_record_row::SIZE * k;
    if k > 10 && read_u8(e + lineup_row::ENTRY_MINUTE) == 0xff {
        return None;
    }
    let (sid, pid) = (read_usize(e) as i32, read_usize(e + lineup_row::PLAYER_ID) as i32);
    if sid == -1 || pid == -1 {
        return None;
    }
    let person = read_usize(G_STAFF_TABLE).wrapping_add((sid as usize).wrapping_mul(staff::SIZE));
    if person != 0 && read_usize(person) as i32 >= read_usize(G_STAFF_COUNT) as i32 {
        return None;
    }
    Some((e, person, read_usize(G_PLAYER_DATA_TABLE).wrapping_add((pid as usize).wrapping_mul(0x46))))
}

/// Categories updated per pass: the match's and the overall 6 (skipped for club and national friendlies).
pub(crate) fn stats_categories_for(cat: u8) -> [Option<u8>; 2] {
    [Some(cat), if cat == 5 || cat == 1 { None } else { Some(6) }] // cpx12: a categoria geral
}

/// The comparer's dry run of `update_league_position_records`: the copy runs without sorting the tables (the original routine, which runs
/// first, has already sorted them).
pub(crate) static POSITION_RECORDS_DRY_RUN: AtomicBool = AtomicBool::new(false);

/// CLUB_RECORDS get_season_record(club, create).
pub(crate) unsafe fn club_season_record(this: usize, club: usize, create: u32) -> usize {
    exe::club_records_get_season_record(this, club, create)
}

/// CLUB_RECORDS get_all_time_record(club, create).
pub(crate) unsafe fn club_all_time_record(this: usize, club: usize, create: u32) -> usize {
    exe::club_records_get_all_time_record(this, club, create)
}

/// Reputation (+0x69, i16) of competition id (0x6b-byte records in G_CLUB_COMP_TABLE).
pub(crate) unsafe fn competition_reputation(id: i32) -> i16 {
    read_i16(read_usize(G_CLUB_COMP_TABLE).wrapping_add((id as usize).wrapping_mul(0x6b)) + competition::REPUTATION)
}

/// For a human club or national team with 5+ seasons, queues the all-time record news (club id, type) unless already
/// queued at +0x24.
pub(crate) unsafe fn club_records_queue_record_news(this: usize, club: usize, summary: usize, kind: u8) {
    if exe::human_manager_is_human_club(club, 0, 1, 0, 0) == 0 && exe::database_is_national_team(club) == 0 {
        return;
    }
    if read_i16(summary + club_record_entry::SEASONS) < 5 {
        return;
    }
    let n = read_usize(this + club_records_fields::NEWS_QUEUE_COUNT) as i32;
    let mut e = read_usize(this + club_records_fields::NEWS_QUEUE);
    for _ in 0..n.max(0) {
        if read_usize(e) == read_usize(club) && read_u8(e + record_news_item::RECORD_TYPE) == kind {
            return;
        }
        e += 6;
    }
    exe::club_records_queue_record_news(this, read_usize(club) as u32, kind as u32);
}

/// The league and its following type-2 stages with the same number of rows (+0x3e).
pub(crate) unsafe fn league_stage_chain(liga0: League) -> Vec<usize> {
    let mut v = vec![liga0.addr()];
    let mut league = liga0.addr();
    loop {
        let k = read_u8(league + league::STAGE_INDEX) as i8 as i32;
        if k >= liga0.current_stage() as i32 {
            return v;
        }
        let f = League(exe::comp_get_stage(liga0, k + 1));
        if f.comp_type() != comp_type::LEAGUE || f.club_count() != read_i16(league + league::CLUB_COUNT) {
            return v;
        }
        v.push(f.addr());
        league = f.addr();
    }
}

/// `Club_Records::update_league_position_records(league, all_time)`: sorts the table of each league stage and stores
/// each club's best and worst league position for the season and, with the second argument, all time (queueing the news
/// item for a human club or national side).
///
/// A null league, or one whose stage index (`stage_index()`) is not `stage_index::MAIN`, gives the error boxes and 0.
/// In each league of the chain (`league_stage_chain`) it sorts the table (`exe::league_sort_table`) and, for each row
/// that has games (`played()`) and a club, through the season records (`club_season_record`) and the all-time records
/// (`club_all_time_record`, created when missing): best position (type 1, `exe::club_records_record_type_applies`) and
/// worst (type 2). The position is written when there is no record yet (−1), when the competition (the id of the first
/// league) has a higher reputation (best) or a lower one (worst), or when it is the same competition with a better or
/// worse position: it stores the position (+0 / `club_record::WORST_POSITION`), the competition
/// (`club_record::BEST_POSITION_COMP` / `club_record::WORST_POSITION_COMP`) and the year
/// (`club_record::BEST_POSITION_YEAR` / `club_record::WORST_POSITION_YEAR`; 0 for the season,
/// `exe::key_nation_get_club_season_year` for all time). For all time (with `always`; an id above [`G_CLUB_COMP_COUNT`]
/// also counts as "no record"), the news queue (`club_records_queue_record_news`). Returns 1.
/// Original: `Club_Records::update_league_position_records` (0x447430, club_records.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn update_league_position_records(this: usize, liga0: usize, always: u32) -> u32 {
    let liga0 = League(liga0);
    if liga0.is_null() || liga0.stage_index() != stage_index::MAIN {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_UPDATE_LEAGUE_POSITION_RECORDS)(this, liga0.addr(), always); // the boxes
    }
    let comp = read_usize(liga0.competition().addr()) as i32;
    for liga in league_stage_chain(liga0) {
        if !POSITION_RECORDS_DRY_RUN.load(Relaxed) {
            exe::league_sort_table(liga, 0);
        }
        let mut i = 0i32;
        while i < read_i16(liga + league::CLUB_COUNT) as i32 {
            let row = LeagueTableRow(read_usize(liga + league::TABLE).wrapping_add(league_table_row::SIZE * i as usize));
            let club = read_usize(row.addr());
            i += 1;
            if (row.played() as u8) == 0 || club == 0 {
                continue;
            }
            let t = club_season_record(this, club, 1);
            let s = club_all_time_record(this, club, 1);
            let summary = read_usize(this + club_records_fields::CLUB_ENTRIES).wrapping_add(read_usize(club).wrapping_mul(0x6a));
            let year = (exe::key_nation_get_club_season_year(club) as i16) as u16;
            let pos = row.position() as u8;
            let away = |v: i32| v < 0 || v > read_usize(G_CLUB_COMP_COUNT) as i32;
            if exe::club_records_record_type_applies(1, club, 1) != 0 {
                let v = read_usize(t + club_record::BEST_POSITION_COMP) as i32;
                if v < 0 || competition_reputation(comp) > competition_reputation(v) || (comp == v && pos < read_u8(t)) {
                    write_u8(t, pos);
                    write_i32(t + club_record::BEST_POSITION_COMP, comp);
                    write_u16(t + club_record::BEST_POSITION_YEAR, 0);
                }
                let v = read_usize(s + club_record::BEST_POSITION_COMP) as i32;
                if always != 0 && (away(v) || competition_reputation(comp) > competition_reputation(v) || (comp == v && pos < read_u8(s))) {
                    covered!("cpx_12 best position ever");
                    write_u8(s, pos);
                    write_i32(s + club_record::BEST_POSITION_COMP, comp);
                    write_u16(s + club_record::BEST_POSITION_YEAR, year);
                    club_records_queue_record_news(this, club, summary, 1);
                }
            }
            if exe::club_records_record_type_applies(2, club, 1) != 0 {
                let v = read_usize(t + club_record::WORST_POSITION_COMP) as i32;
                if v < 0 || competition_reputation(comp) < competition_reputation(v) || (comp == v && pos > read_u8(t + club_record::WORST_POSITION)) {
                    write_u8(t + club_record::WORST_POSITION, pos);
                    write_i32(t + club_record::WORST_POSITION_COMP, comp);
                    write_u16(t + club_record::WORST_POSITION_YEAR, 0);
                }
                let v = read_usize(s + club_record::WORST_POSITION_COMP) as i32;
                if always != 0 && (away(v) || competition_reputation(comp) < competition_reputation(v) || (comp == v && pos > read_u8(s + club_record::WORST_POSITION))) {
                    write_u8(s + club_record::WORST_POSITION, pos);
                    write_i32(s + club_record::WORST_POSITION_COMP, comp);
                    write_u16(s + club_record::WORST_POSITION_YEAR, year);
                    club_records_queue_record_news(this, club, summary, 2);
                }
            }
        }
    }
    1
}

/// O ensaio do comparador da `send_record_news`: a cópia roda sem preencher nem entregar as notícias e sem soltar a fila.
pub(crate) static RECORD_NEWS_DRY_RUN: AtomicBool = AtomicBool::new(false);

/// `Club_Records::send_record_news()` (SEH frame without a destructor): delivers the news item of each queued record
/// (human club or national side) and empties the queue.
///
/// Without a queue (`club_records_fields::NEWS_QUEUE`), returns 1. For each record in the queue (6 bytes: club id, type
/// at `record_news_item::RECORD_TYPE`, period at `record_news_item::PERIOD`) of a human club
/// (`FN_NEWS_CLUB_HAS_HUMAN_MANAGER`) or a national side (`exe::database_is_national_team`): the news item 0x3e8
/// (local) is filled by `exe::club_records_write_record_news_fields` with the season record (period 2,
/// `club_season_record`) or the all-time record (`club_all_time_record`), without creating them; it is delivered to the
/// nation (`exe::news_send_to_nation_managers`) of the national side or to the club's manager
/// (`exe::news_send_to_club_manager`). A queue with capacity (`club_records_fields::NEWS_QUEUE_CAPACITY`) of 100 or
/// more is freed (`exe::crt_free`) and `NEWS_QUEUE`/`NEWS_QUEUE_CAPACITY` are zeroed. The count
/// (`club_records_fields::NEWS_QUEUE_COUNT`) goes back to 0. Returns 1.
/// Original: `Club_Records::send_record_news` (0x448e30, club_records.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn send_record_news(this: usize) -> u32 {
    if read_usize(this + club_records_fields::NEWS_QUEUE) == 0 {
        return 1;
    }
    let human: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(FN_NEWS_CLUB_HAS_HUMAN_MANAGER);
    let mut i = 0i32;
    while i < read_usize(this + club_records_fields::NEWS_QUEUE_COUNT) as i32 {
        let e = read_usize(this + club_records_fields::NEWS_QUEUE) + 6 * i as usize;
        i += 1;
        let id = read_usize(e);
        let kind = read_u8(e + record_news_item::RECORD_TYPE);
        let club = Club(read_usize(G_CLUB_TABLE).wrapping_add(id).wrapping_add(id.wrapping_mul(0x244)));
        if human(club.addr()) == 0 && exe::database_is_national_team(club) == 0 {
            continue;
        }
        covered!("cpx_12 record news");
        let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
        let n = local.as_mut_ptr() as usize;
        exe::new_newsitem(n, 0x3e8, 0);
        let reg = if read_u8(e + record_news_item::PERIOD) == 2 { club_season_record(this, club.addr(), 0) } else { club_all_time_record(this, club.addr(), 0) };
        if RECORD_NEWS_DRY_RUN.load(Relaxed) {
            continue;
        }
        // simplification: the original pushes the local’s dword with the type in the low byte (the rest is leftover); 0x453120 reads the byte
        exe::club_records_write_record_news_fields(id, reg, kind as u32, n);
        if exe::database_is_national_team(club) != 0 {
            exe::news_send_to_nation_managers(read_usize(G_NEWS), n, club.nation().addr());
        } else {
            exe::news_send_to_club_manager(read_usize(G_NEWS), n, club);
        }
    }
    if read_usize(this + club_records_fields::NEWS_QUEUE_CAPACITY) as i32 >= 100 {
        if !RECORD_NEWS_DRY_RUN.load(Relaxed) {
            exe::crt_free(read_usize(this + club_records_fields::NEWS_QUEUE));
        }
        write_usize(this + club_records_fields::NEWS_QUEUE, 0);
        write_usize(this + club_records_fields::NEWS_QUEUE_CAPACITY, 0);
    }
    write_usize(this + club_records_fields::NEWS_QUEUE_COUNT, 0);
    1
}

/// The fans' rating of player `p` at the club from stats `st` (`pick_fans_player_of_year`).
///
/// Average (+0xe signed / games +0) + 0.025 × +2 + 1/60 × +6. The club likes him (`favourite_of`): +0.2; does not like him
/// (`rejected_by`): −2 (stored); he likes the club (`exe::staff_favours_club`): +0.2; does not like it (`exe::staff_dislikes_club`): −1.5 (only in the
/// comparison).
pub(crate) unsafe fn fan_rating(p: Staff, club: Club, st: usize) -> f64 {
    let average = read_i16(st + player_category_stats::RATING_TOTAL) as i32 as f64 / read_u8(st) as i32 as f64;
    let mut rating = average + read_u8(st + player_category_stats::GOALS) as i32 as f64 * read_f64(rdata::F64_0_025);
    rating = rating + read_u8(st + 6) as i32 as f64 * read_f64(rdata::F64_0_01666666667);
    if exe::club_favours_staff(p, club) != 0 {
        covered!("cpp_10 club favourite");
        rating = rating + read_f64(rdata::F64_0_2); // the club likes him
    } else if exe::club_dislikes_staff(p, club) != 0 {
        rating = rating - read_f64(rdata::F64_2);
    }
    if exe::staff_favours_club(p, club) != 0 {
        rating + read_f64(rdata::F64_0_2)
    } else if exe::staff_dislikes_club(p, club) != 0 {
        rating - read_f64(rdata::F64_1_5)
    } else {
        rating
    }
}

/// `Club_Records::pick_fans_player_of_year(club, record)`: picks the fans' player of the year from the club's squad:
/// the best score among the players with games above the minimum; stores his reference at `club_record::FANS_PLAYER` of
/// the record and returns 1, or 0.
///
/// Without games (signed `club_record_entry::SEASON_GAMES` of the club's 106-byte entry in [this +
/// `club_records_fields::CLUB_ENTRIES`]) and with reputation (`reputation()`) below 3000, returns 0. With the factor
/// 0.6, falling by 0.1 (`rdata::F64_0_1`) while nobody has been chosen and it is above 0.2 (`rdata::F64_0_2`): the
/// minimum number of games is `SEASON_GAMES` × factor (`ftol`); among the 50 squad slots (`squad()`), the players
/// (`player()`, id below [`G_STAFF_COUNT`]) of the club (`club_job()`) with statistic 6
/// (`exe::playerstats_get_category_record` over [`G_PLAYER_STATS`]) whose games are above 0 and above the minimum
/// (signed 16 bits), the one with the highest score (`fan_rating`: average + 0.025 × +2 + 1/60 × +6, club and player
/// likes; it must be above −1000). Nobody: 0; otherwise his reference (`exe::fill_staff_name_ref`) at
/// `club_record::FANS_PLAYER` of the record and 1.
/// Original: `Club_Records::pick_fans_player_of_year` (0x452da0, club_records.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn pick_fans_player_of_year(this: usize, club: usize, record: usize) -> u32 {
    let club = Club(club);
    let games_of_club = || read_u8(read_usize(this + club_records_fields::CLUB_ENTRIES).wrapping_add((read_i32(club.addr()) as usize).wrapping_mul(106)) + club_record_entry::SEASON_GAMES) as i8;
    if games_of_club() == 0 && club.reputation() < 0xbb8 {
        return 0;
    }
    let mut factor = 0.6f64;
    let mut best = -1000.0f64;
    let mut chosen = 0usize;
    while chosen == 0 {
        let minimum = ftol(games_of_club() as i32 as f64 * factor) as i16;
        for s in 0..50usize {
            let p = Staff(club.squad(s));
            if p.is_null() || read_i32(p.addr()) >= read_i32(G_STAFF_COUNT) {
                continue;
            }
            let player = p.player();
            if player.is_null() || p.club_job() != club {
                continue;
            }
            let st = exe::playerstats_get_category_record(read_usize(G_PLAYER_STATS), player, 6);
            if st == 0 {
                continue;
            }
            let games = read_u8(st);
            if (games as i16) < minimum || games == 0 {
                continue;
            }
            let rating = fan_rating(p, club, st);
            if rating > best {
                best = rating;
                chosen = p.addr();
            }
        }
        factor -= read_f64(rdata::F64_0_1);
        if !(factor > read_f64(rdata::F64_0_2)) {
            break;
        }
    }
    if chosen == 0 {
        return 0;
    }
    exe::fill_staff_name_ref(chosen, record + club_record::FANS_PLAYER);
    1
}

/// Transfer record (club id, type) is already in the news queue at [this+0x24] (count +0x2c).
pub(crate) unsafe fn record_news_queued(this: usize, id: usize, kind: u8) -> bool {
    let queue = read_usize(this + club_records_fields::NEWS_QUEUE);
    (0..read_i32(this + club_records_fields::NEWS_QUEUE_COUNT).max(0) as usize).any(|k| read_usize(queue + 6 * k) == id && read_u8(queue + 6 * k + 4) == kind)
}

/// Transfer value >= 1000 x (club reputation + 300).
pub(crate) unsafe fn is_big_transfer(value: i32, club: Club) -> bool {
    value >= (club.reputation() as i32 + 300).wrapping_mul(1000)
}

/// Copies the record block at offset d from o to a (value, 0x18-byte reference, club, dates, flag).
pub(crate) unsafe fn copy_transfer_record(a: usize, o: usize, d: usize) {
    let mut draft = [0u8; 8];
    for k in 0..5 {
        write_usize(a + d + transfer_fee_record::HOLDER * k, read_usize(o + d + transfer_fee_record::HOLDER * k));
    }
    exe::date_operator_3d(a + d + transfer_fee_record::HOLDER_DATE, (draft.as_mut_ptr()) as usize, o + d + transfer_fee_record::HOLDER_DATE);
    write_usize(a + d + transfer_fee_record::OTHER_CLUB_ID, read_usize(o + d + transfer_fee_record::OTHER_CLUB_ID));
    exe::date_operator_3d(a + d + transfer_fee_record::DATE, (draft.as_mut_ptr()) as usize, o + d + transfer_fee_record::DATE);
    write_u8(a + d + transfer_fee_record::FLAG, read_u8(o + d + transfer_fee_record::FLAG));
}

/// `Club_Records::update_transfer_fee_records(transfer)`: updates the buyer's paid record and the seller's received
/// record, for the season and all time (fee, reference, other club, date, loan).
///
/// The person (`target_staff_id()`) and the buying club (`buyer_club_id()`); the person's contract
/// (`exe::contract_manager_get_contract` over [`G_CONTRACT_MANAGER`], kept in [`G_CONTRACT_LOOKUP_RESULT`]) gives the
/// seller (`club()`). The buyer's season (`exe::club_records_get_season_record`) and all-time
/// (`exe::club_records_get_all_time_record`) records are fetched (created when missing); without a seller, returns 1. A
/// fee (`fee()`) above the season paid record (`club_record::FEE_PAID`) becomes the record: the fee, the person's
/// reference (`exe::fill_staff_name_ref`, at `FEE_PAID_HOLDER`), the seller (`FEE_PAID_OTHER_CLUB_ID`), today's date
/// (`FEE_PAID_DATE`) and the loan flag (`FEE_PAID_FLAG`: `exchange_players()` ≠ 0); if it is above the all-time record
/// it is copied there, and if it passes 1000 × (reputation + 300) (`is_big_transfer`): the record goes to the news
/// queue (`exe::club_records_queue_record_news`, 0x1c, if it is not there yet) and the chairman (`chairman()`,
/// `club::CHAIRMAN`) comes to favour the person (`exe::staff_add_favoured_staff`, forced). The same for the seller in
/// the received record (`club_record::FEE_RECEIVED`; the queue with 0x1d, without the favouring). Returns 1.
/// Original: `Club_Records::update_transfer_fee_records` (0x448760, club_records.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn update_transfer_fee_records(this: usize, t: usize) -> u32 {
    let t = Offer(t);
    let person = read_usize(G_STAFF_TABLE).wrapping_add(((t.target_staff_id() as i32) as usize).wrapping_mul(staff::SIZE));
    let buyer = Club((injury_club_by_index(t.buyer_club_id() as i32)).addr());
    let contract = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person));
    write_usize(G_CONTRACT_LOOKUP_RESULT, contract.addr());
    let seller = if !contract.is_null() { injury_club_by_index(contract.club() as i32) } else { Club(0) };
    let mut draft = [0u8; 8];
    let s = exe::club_records_get_season_record(this, buyer, 1);
    let a = exe::club_records_get_all_time_record(this, buyer, 1);
    if seller.is_null() {
        return 1;
    }
    let value = t.fee() as i32;
    if value != 0 && value > read_i32(s + club_record::FEE_PAID) {
        covered!("cpp_10 record fee paid");
        write_i32(s + club_record::FEE_PAID, value);
        exe::fill_staff_name_ref(person, s + club_record::FEE_PAID_HOLDER);
        write_usize(s + club_record::FEE_PAID_OTHER_CLUB_ID, read_usize(seller.addr()));
        exe::date_operator_3d(s + club_record::FEE_PAID_DATE, (draft.as_mut_ptr()) as usize, G_CURRENT_DATE);
        write_u8(s + club_record::FEE_PAID_FLAG, (!t.exchange_players().is_null()) as u8);
        if (t.fee() as i32) > read_i32(a + club_record::FEE_PAID) {
            copy_transfer_record(a, s, 0x313);
            if is_big_transfer(read_i32(a + club_record::FEE_PAID), buyer) {
                covered!("cpp_10 big transfer");
                if !record_news_queued(this, read_usize(buyer.addr()), 0x1c) {
                    exe::club_records_queue_record_news(this, read_usize(buyer.addr()) as u32, 0x1c);
                }
                exe::staff_add_favoured_staff(buyer.chairman().addr(), person, 1); // forced
            }
        }
    }
    let s = exe::club_records_get_season_record(this, seller, 1);
    let a = exe::club_records_get_all_time_record(this, seller, 1);
    let value = t.fee() as i32;
    if value == 0 || value <= read_i32(s + club_record::FEE_RECEIVED) {
        return 1;
    }
    write_i32(s + club_record::FEE_RECEIVED, value);
    exe::fill_staff_name_ref(person, s + club_record::FEE_RECEIVED_HOLDER);
    write_usize(s + club_record::FEE_RECEIVED_OTHER_CLUB_ID, read_usize(buyer.addr()));
    exe::date_operator_3d(s + club_record::FEE_RECEIVED_DATE, (draft.as_mut_ptr()) as usize, G_CURRENT_DATE);
    write_u8(s + club_record::FEE_RECEIVED_FLAG, (!t.exchange_players().is_null()) as u8);
    if (t.fee() as i32) > read_i32(a + club_record::FEE_RECEIVED) {
        copy_transfer_record(a, s, 0x33c);
        if is_big_transfer(read_i32(a + club_record::FEE_RECEIVED), seller) && !record_news_queued(this, read_usize(seller.addr()), 0x1d) {
            exe::club_records_queue_record_news(this, read_usize(seller.addr()) as u32, 0x1d);
        }
    }
    1
}

/// Record news (club id, type) is already in the 6-byte vector [this+0x24] (count +0x2c).
pub(crate) unsafe fn is_record_news_queued(this: usize, id: usize, kind: u8) -> bool {
    let n = read_usize(this + club_records_fields::NEWS_QUEUE_COUNT) as i32;
    let v = read_usize(this + club_records_fields::NEWS_QUEUE);
    (0..n.max(0) as usize).any(|k| read_usize(v + 6 * k) == id && *((v + 6 * k + 4) as *const u8) == kind)
}

/// `Club_Records::add_transfer_totals(fee, buyer, seller)`: adds the fee to the seller's season income and the buyer's
/// season spending, raises the all-time records and queues the record news (0x1f, 0x1e) above (`reputation()` + 500) ×
/// 1000.
///
/// The seller adds `value` to the income for the season (`club_record::TRANSFER_INCOME` of the season record
/// `exe::club_records_get_season_record`; −1 = nothing yet: the fee is stored) and zeroes the year
/// (`club_record::TRANSFER_INCOME_YEAR`); above the all-time record (`exe::club_records_get_all_time_record`, same
/// field), the record becomes the season one with the club's season year (`exe::key_nation_get_club_season_year`), and
/// with the total at least (club `reputation()` + 500) × 1000 and the year different from the record's, the record news
/// item 0x1f (`exe::club_records_queue_record_news`) if it is not in the queue yet. The buyer does the same for
/// spending (`club_record::TRANSFER_SPENDING`, year `club_record::TRANSFER_SPENDING_YEAR`, news 0x1e), but the minimum
/// is checked against the OLD record and without the year. The records are created when missing (second argument 1). A
/// null buyer or seller: nothing, 1. Returns 1.
/// Original: `Club_Records::add_transfer_totals` (0x448ad0, club_records.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn add_transfer_totals(this: usize, value: i32, buyer: usize, seller: usize) -> u32 {
    let seller = Club(seller);
    let buyer = Club(buyer);
    if buyer.is_null() || seller.is_null() {
        return 1;
    }
    let add = |field: usize| {
        let v = read_usize(field) as i32;
        write_usize(field, if v == -1 { value } else { v.wrapping_add(value) } as usize);
    };
    // the seller: the amount received
    let t = exe::club_records_get_season_record(this, seller, 1);
    let s = exe::club_records_get_all_time_record(this, seller, 1);
    add(t + club_record::TRANSFER_INCOME);
    write_i16(t + club_record::TRANSFER_INCOME_YEAR, 0);
    let year = (exe::key_nation_get_club_season_year(seller) as u32) as i16;
    if (read_usize(s + club_record::TRANSFER_INCOME) as i32) < read_usize(t + club_record::TRANSFER_INCOME) as i32 {
        let minimum = (seller.reputation() as i32 + 500).wrapping_mul(1000);
        if read_usize(t + club_record::TRANSFER_INCOME) as i32 >= minimum && year != read_i16(s + club_record::TRANSFER_INCOME_YEAR) && !is_record_news_queued(this, read_usize(seller.addr()), 0x1f) {
            covered!("cpm_6 record fee received news");
            exe::club_records_queue_record_news(this, read_usize(seller.addr()) as u32, 0x1f); // record fee received
        }
        write_usize(s + club_record::TRANSFER_INCOME, read_usize(t + club_record::TRANSFER_INCOME));
        write_i16(s + club_record::TRANSFER_INCOME_YEAR, year);
    }
    // the buyer: the amount spent
    let t = exe::club_records_get_season_record(this, buyer, 1);
    let s = exe::club_records_get_all_time_record(this, buyer, 1);
    add(t + club_record::TRANSFER_SPENDING);
    write_i16(t + club_record::TRANSFER_SPENDING_YEAR, 0);
    let year = (exe::key_nation_get_club_season_year(buyer) as u32) as i16;
    if (read_usize(t + club_record::TRANSFER_SPENDING) as i32) > read_usize(s + club_record::TRANSFER_SPENDING) as i32 {
        let minimum = (buyer.reputation() as i32 + 500).wrapping_mul(1000);
        // the original checks the minimum against the old record (the ecx that still holds the all-time one)
        if read_usize(s + club_record::TRANSFER_SPENDING) as i32 >= minimum && !is_record_news_queued(this, read_usize(buyer.addr()), 0x1e) {
            covered!("cpm_6 record fee paid news");
            exe::club_records_queue_record_news(this, read_usize(buyer.addr()) as u32, 0x1e); // record fee paid
        }
        write_usize(s + club_record::TRANSFER_SPENDING, read_usize(t + club_record::TRANSFER_SPENDING));
        write_i16(s + club_record::TRANSFER_SPENDING_YEAR, year);
    }
    1
}

/// `club_records_update_career_records(player, club, games, goals)`: updates the club's all-time league appearances
/// (+0x247) and goals (+0x261) records, with the player reference (`exe::fill_staff_name_ref`), when the record type
/// applies to the club (`exe::club_records_record_type_applies`).
///
/// In the club's all-time records (`exe::club_records_get_all_time_record`, created when missing), if type 0x17 applies
/// to the club (`exe::club_records_record_type_applies`) and `games` beats the record (+0x247; −1 = no record), the
/// record = `games` and the player's reference goes to +0x249 (`exe::fill_staff_name_ref`); the same with type 0x18 and
/// `goals` (+0x261, reference +0x263). Returns 1. A null player or club, or no record: the "Error" box (lines 0x6fa and
/// 0x702) and 0.
/// Original: `club_records_update_career_records` (0x448cb0, club_records.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn club_records_update_career_records(this: usize, player: usize, club: usize, games: u32, goals: u32) -> u32 {
    if player == 0 || club == 0 {
        show_error_at(0x98b6a0, 0x6fa);
        return 0;
    }
    let r = exe::club_records_get_all_time_record(this, club, 1);
    if r == 0 {
        show_error_at(0x98b6a0, 0x702);
        return 0;
    }
    for (tipo, campo, n) in [(0x17u32, 0x247usize, games as i16), (0x18, 0x261, goals as i16)] {
        if exe::club_records_record_type_applies(tipo, club, 1) != 0 {
            let v = read_i16(r + campo);
            if v == -1 || n > v {
                write_i16(r + campo, n);
                exe::fill_staff_name_ref(player, r + campo + 2);
            }
        }
    }
    1
}

/// `Club_Records::update_season_rating_record(club)`: the squad player with the best average rating (statistic 0x11)
/// among those who played at least half the games becomes the season rating record.
///
/// With games in the season (byte `club_record_entry::SEASON_GAMES` of the club's entry, [this +
/// `club_records_fields::CLUB_ENTRIES`] + id × 0x6a) and the season record (`exe::club_records_get_season_record`,
/// without creating), the record rating (`club_record::SEASON_RATING`) goes back to −1.0 and, over the 50 squad slots
/// (`squad()`), the player (not a human manager: id < [`G_STAFF_COUNT`]) with statistics (`player()`) at the club
/// (`club_job()` = the club or the one linked to it, `exe::club_get_linked_b_team`) and a season statistics record
/// (`exe::playerstats_get_category_record`, type 6) who played at least half the games (statistic 1, truncated,
/// `exe::player_stats_get_record_value` with 6.5) and has the highest average (statistic 0x11) becomes the record: the
/// rating (`SEASON_RATING`, float), the games (`club_record::SEASON_RATING_GAMES`), the reference (`SEASON_RATING` + 4,
/// `exe::fill_staff_name_ref`) and `club_record::SEASON_RATING_YEAR` = 0. Returns 1 if someone became the record.
/// Without games or without a record: 0. A null club: the original routine (box 0x16f2, 0).
/// Original: `Club_Records::update_season_rating_record` (0x452a90, club_records.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn update_season_rating_record(this: usize, club: usize) -> u32 {
    let club = Club(club);
    if club.is_null() {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_UPDATE_SEASON_RATING_RECORD)(this, club.addr()); // message box: original only
    }
    let entry = read_usize(this + club_records_fields::CLUB_ENTRIES).wrapping_add(read_usize(club.addr()).wrapping_mul(0x6a));
    if read_u8(entry + club_record_entry::SEASON_GAMES) == 0 {
        return 0;
    }
    let r = exe::club_records_get_season_record(this, club, 0);
    if r == 0 {
        return 0;
    }
    let rating = (r + club_record::SEASON_RATING) as *mut f32;
    write_usize(r + club_record::SEASON_RATING, 0xbf800000); // −1.0f
    let mut found = 0;
    for k in 0..50usize {
        let p = Staff(club.squad(k));
        if p.is_null() {
            continue;
        }
        let est = p.player();
        if est.is_null() || read_usize(p.addr()) as i32 >= read_usize(G_STAFF_COUNT) as i32 {
            continue;
        }
        if p.club_job() != club && p.club_job() != Club(exe::club_get_linked_b_team(club, (0) as *mut u32, 1)) {
            continue;
        }
        let reg = exe::playerstats_get_category_record(read_usize(G_PLAYER_STATS), est, 6);
        if reg == 0 {
            continue;
        }
        let jogou = ftol(exe::player_stats_get_record_value(reg, 1, 6.5)) as u8;
        if (jogou as i32) < read_i8(entry + club_record_entry::SEASON_GAMES) as i32 / 2 { // fewer than half the matches
            continue;
        }
        let average = exe::player_stats_get_record_value(reg, 0x11, 6.5);
        // fcomp with −1.0 (double) and fcom with the stored float: passes only with the higher average
        if *rating as f64 == read_f64(rdata::F64_NEG_1) || average > *rating as f64 {
            covered!("cpm_6 season rating record");
            *rating = average as f32;
            write_u8(r + club_record::SEASON_RATING_GAMES, jogou);
            exe::fill_staff_name_ref(p, r + club_record::SEASON_RATING + 4);
            write_i16(r + club_record::SEASON_RATING_YEAR, 0);
            found = 1;
        }
    }
    found
}

/// Club's 0x6a-byte entry in [this+0x1c] (+0 all-time index, +4 season index, -1 none).
pub(crate) unsafe fn club_records_entry(this: usize, club: usize) -> usize {
    read_usize(this + club_records_fields::CLUB_ENTRIES).wrapping_add(read_usize(club).wrapping_mul(0x6a))
}

/// club_records.cpp out-of-memory: shows the box at `linha` and exits fatally.
pub(crate) unsafe fn records_out_of_memory(row: u32) -> ! {
    show_error_at(0x98b6a0, row);
    fatal_exit()
}

/// `Club_Records::get_all_time_record(club, create)`: returns the club's all-time records: index +0 of its entry ([this
/// + `club_records_fields::CLUB_ENTRIES`] + id × 0x6a, `club_records_entry`) into the vector [this +
/// `club_records_fields::ALL_TIME_RECORDS`] of 0x38b-byte records.
///
/// Without an index (−1) and with `create`: the new index is the count (`club_records_fields::ALL_TIME_COUNT`, which
/// goes up), the vector grows by 50 at a time (capacity `club_records_fields::ALL_TIME_CAPACITY`, realloc
/// `exe::crt_realloc`; without memory, box 0x1694 and the end, `records_out_of_memory`) and the record is reset
/// (`exe::season_history_entry_clear`). Returns the record, or 0 without an index. A null club: the original routine
/// (box 0x1680, 0).
/// Original: `Club_Records::get_all_time_record` (0x452710, club_records.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn get_all_time_record(this: usize, club: usize, create: u32) -> usize {
    if club == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(IDX_GET_ALL_TIME_RECORD)(this, club, create);
    }
    let e = club_records_entry(this, club);
    if read_usize(e) as i32 == -1 && create != 0 {
        covered!("cpk_1 new all-time records");
        if read_usize(this + club_records_fields::ALL_TIME_COUNT) == read_usize(this + club_records_fields::ALL_TIME_CAPACITY) {
            let cap = read_usize(this + club_records_fields::ALL_TIME_CAPACITY).wrapping_add(50);
            write_usize(this + club_records_fields::ALL_TIME_CAPACITY, cap);
            let p = exe::crt_realloc(read_usize(this + club_records_fields::ALL_TIME_RECORDS), cap.wrapping_mul(0x38b) as u32);
            write_usize(this + club_records_fields::ALL_TIME_RECORDS, p);
            if p == 0 {
                records_out_of_memory(0x1694);
            }
        }
        let n = read_usize(this + club_records_fields::ALL_TIME_COUNT);
        write_usize(e, n);
        write_usize(this + club_records_fields::ALL_TIME_COUNT, n.wrapping_add(1)); // the all-time records calculation
        exe::season_history_entry_clear(read_usize(this + club_records_fields::ALL_TIME_RECORDS).wrapping_add(read_usize(e).wrapping_mul(0x38b)));
    }
    let i = read_usize(e) as i32;
    if i <= -1 {
        return 0;
    }
    read_usize(this + club_records_fields::ALL_TIME_RECORDS).wrapping_add((i as usize).wrapping_mul(0x38b))
}

/// `Club_Records::get_season_record(club, create)`: the same as `get_all_time_record` for the season records: index +4
/// of the entry, vector [this + `club_records_fields::SEASON_RECORDS`], capacity
/// `club_records_fields::SEASON_CAPACITY`, count `club_records_fields::SEASON_COUNT` (without memory, box 0x16c9 and
/// the end). A null club: the original routine (box 0x16b5, 0).
/// Original: `Club_Records::get_season_record` (0x4528d0, club_records.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn get_season_record(this: usize, club: usize, create: u32) -> usize {
    if club == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(IDX_GET_SEASON_RECORD)(this, club, create);
    }
    let e = club_records_entry(this, club) + 4;
    if read_usize(e) as i32 == -1 && create != 0 {
        covered!("cpk_1 new season records");
        if read_usize(this + club_records_fields::SEASON_COUNT) == read_usize(this + club_records_fields::SEASON_CAPACITY) {
            let cap = read_usize(this + club_records_fields::SEASON_CAPACITY).wrapping_add(50);
            write_usize(this + club_records_fields::SEASON_CAPACITY, cap);
            let p = exe::crt_realloc(read_usize(this + club_records_fields::SEASON_RECORDS), cap.wrapping_mul(0x38b) as u32);
            write_usize(this + club_records_fields::SEASON_RECORDS, p);
            if p == 0 {
                records_out_of_memory(0x16c9);
            }
        }
        let n = read_usize(this + club_records_fields::SEASON_COUNT);
        write_usize(e, n);
        write_usize(this + club_records_fields::SEASON_COUNT, n.wrapping_add(1)); // the season records count
        exe::season_history_entry_clear(read_usize(this + club_records_fields::SEASON_RECORDS).wrapping_add(read_usize(e).wrapping_mul(0x38b)));
    }
    let i = read_usize(e) as i32;
    if i <= -1 {
        return 0;
    }
    read_usize(this + club_records_fields::SEASON_RECORDS).wrapping_add((i as usize).wrapping_mul(0x38b))
}

/// `Club_Records::queue_record_news(id, type)`: appends (club id at +0, record type at `record_news_item::RECORD_TYPE`;
/// byte +5 is left as it is) to the 6-byte vector [this + `club_records_fields::NEWS_QUEUE`] of record news, read by
/// `send_record_news`.
///
/// When full (count `club_records_fields::NEWS_QUEUE_COUNT` = capacity `club_records_fields::NEWS_QUEUE_CAPACITY`) it
/// grows by 5 at a time (realloc `exe::crt_realloc`); without memory, box 0x174f and returns 0 (the game goes on,
/// without the vector). Returns 1.
/// Original: `Club_Records::queue_record_news` (0x452cb0, club_records.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn queue_record_news(this: usize, id: u32, kind: u32) -> u32 {
    covered!("cpk_1 record news");
    if read_usize(this + club_records_fields::NEWS_QUEUE_COUNT) == read_usize(this + club_records_fields::NEWS_QUEUE_CAPACITY) {
        let cap = read_usize(this + club_records_fields::NEWS_QUEUE_CAPACITY).wrapping_add(5);
        write_usize(this + club_records_fields::NEWS_QUEUE_CAPACITY, cap);
        let p = exe::crt_realloc(read_usize(this + club_records_fields::NEWS_QUEUE), cap.wrapping_mul(6) as u32);
        write_usize(this + club_records_fields::NEWS_QUEUE, p);
        if p == 0 {
            show_error_at(0x98b6a0, 0x174f);
            return 0;
        }
    }
    let n = read_usize(this + club_records_fields::NEWS_QUEUE_COUNT);
    let item = read_usize(this + club_records_fields::NEWS_QUEUE).wrapping_add(n.wrapping_mul(6));
    write_u32(item, id);
    write_u8(item + record_news_item::RECORD_TYPE, kind as u8);
    write_usize(this + club_records_fields::NEWS_QUEUE_COUNT, n.wrapping_add(1)); // one more news item
    1
}

/// Plan before the original: expected index, expected count and whether a new record is created.
pub(crate) unsafe fn record_index_plan(e: usize, count: usize, create: u32) -> (i32, usize, bool) {
    let i = read_usize(e) as i32;
    let n = read_usize(count);
    if i == -1 && create != 0 { (n as i32, n.wrapping_add(1), true) } else { (i, n, false) }
}

/// Checks the original's result against the plan (index, count, returned record, reset markers of a new one) and
/// records it in PLACAR.
pub(crate) unsafe fn check_record_plan(name: &str, club: usize, e: usize, count: usize, array: usize, plan: (i32, usize, bool), r: usize) {
    let (i, n, novo) = plan;
    let expected = if i <= -1 { 0 } else { read_usize(array).wrapping_add((i as usize).wrapping_mul(0x38b)) };
    let restarted = !novo || (r != 0 && read_usize(r + club_record::SEASON_RATING) == 0xbf800000 && read_usize(r + club_record::FEE_PAID) as i32 == -1);
    SCOREBOARD.record(r == expected && read_usize(e) as i32 == i && read_usize(count) == n && restarted, || {
        format!("{}(club {}) index {} (esperava {}), count {} (esperava {}), record {:x} (esperava {:x}), new={} restarted={}",
            name, read_usize(club), read_usize(e) as i32, i, read_usize(count), n, r, expected, novo, restarted)
    });
}

/// `Club_Records::offer_breaks_fee_record(offer, require)`: tells whether the offer fee (`fee()`) beats the fee-paid
/// record of the buying club (`buyer_club_id()`; the all-time records through `exe::club_records_get_all_time_record`,
/// creating them if needed).
///
/// It must be at least 50 × reputation (`reputation()`) + 500,000; with `exigir`, also at least reputation² × the
/// constant `rdata::F64_0_1` (`ftol`); and above zero and above the record (`club_record::FEE_PAID`). A null offer: 0.
/// Original: `Club_Records::offer_breaks_fee_record` (0x4486c0, club_records.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn offer_breaks_fee_record(this: usize, offer: usize, exigir: u32) -> u8 {
    let offer = Offer(offer);
    if offer.is_null() {
        return 0;
    }
    let club = Club(read_usize(G_CLUB_TABLE).wrapping_add(offer.buyer_club_id().wrapping_mul(club::SIZE)));
    let rec = exe::club_records_get_all_time_record(this, club, 1);
    if rec == 0 || club.is_null() {
        return 0;
    }
    let rep = club.reputation() as i32;
    let value = offer.fee() as i32;
    if value < rep.wrapping_mul(25).wrapping_add(250_000).wrapping_mul(2) {
        return 0; // below the minimum for the reputation
    }
    if exigir != 0 && value < ftol(rep as f64 * rep as f64 * read_f64(rdata::F64_0_1)) {
        return 0;
    }
    (value != 0 && value > read_usize(rec + club_record::FEE_PAID) as i32) as u8
}

/// `transfer_fee_record_construct`: constructs the highest transfer fee paid record: the dates at
/// `transfer_fee_record::HOLDER_DATE` and `transfer_fee_record::DATE` (`exe::date_date_54a110`, through
/// `init_default_date`).
///
/// The original's SEH frame only covers the 2nd date (`exe::date_date_54a110` does not fail). Returns `this`.
/// Original: `transfer_fee_record_construct` (0x4485d0, club_records.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn transfer_fee_record_construct(this: usize) -> usize {
    init_default_date(this + transfer_fee_record::HOLDER_DATE); // the date of the amount paid
    init_default_date(this + transfer_fee_record::DATE);
    this
}

/// constructs the highest fee received record: the dates at `transfer_fee_record::HOLDER_DATE` and
/// `transfer_fee_record::DATE` (`exe::date_date_54a110`, through `init_default_date`)
/// Original: `transfer_fee_record_construct_2` (0x448620, club_records.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn transfer_fee_record_construct_2(this: usize) -> usize {
    init_default_date(this + transfer_fee_record::HOLDER_DATE);
    init_default_date(this + transfer_fee_record::DATE); // the 2nd date of the amount received
    this
}

/// constructs the run record: the start (+4) and end (+0xc) dates (`exe::date_date_54a110`)
/// Original: `run_record_construct` (0x448670, club_records.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn run_record_construct(this: usize) -> usize {
    init_default_date(this + run_record::START_DATE); // the start of the sequence
    init_default_date(this + run_record::END_DATE);
    this
}

/// constructs the won run record: the start (+4) and end (+0xc) dates (`exe::date_date_54a110`)
/// Original: `won_run_record_construct` (0x450ad0, club_records.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn won_run_record_construct(this: usize) -> usize {
    init_default_date(this + run_record::START_DATE); // the start of the winning run
    init_default_date(this + run_record::END_DATE);
    this
}

/// constructs the lost run record: the start (+4) and end (+0xc) dates (`exe::date_date_54a110`)
/// Original: `lost_run_record_construct` (0x450b20, club_records.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn lost_run_record_construct(this: usize) -> usize {
    init_default_date(this + run_record::START_DATE);
    init_default_date(this + run_record::END_DATE); // the end of the losing run
    this
}

/// constructs the unbeaten run record: the start (+4) and end (+0xc) dates (`exe::date_date_54a110`)
/// Original: `unbeaten_run_record_construct` (0x450b70, club_records.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn unbeaten_run_record_construct(this: usize) -> usize {
    init_default_date(this + run_record::START_DATE); // the start of the unbeaten run
    init_default_date(this + run_record::END_DATE);
    this
}

/// constructs the winless run record: the start (+4) and end (+0xc) dates (`exe::date_date_54a110`)
/// Original: `winless_run_record_construct` (0x450bc0, club_records.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn winless_run_record_construct(this: usize) -> usize {
    init_default_date(this + run_record::START_DATE);
    init_default_date(this + run_record::END_DATE); // the end of the winless run
    this
}

/// constructs the goals in a match record: the holder date (`goals_in_match_record::HOLDER_DATE`) and the match date
/// (`goals_in_match_record::MATCH`) (`exe::date_date_54a110`, through `init_default_date`)
/// Original: `goals_in_match_record_construct` (0x450c10, club_records.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn goals_in_match_record_construct(this: usize) -> usize {
    init_default_date(this + goals_in_match_record::HOLDER_DATE);
    init_default_date(this + goals_in_match_record::MATCH); // the match date
    this
}

/// constructs the youngest player record: the age (+0), the holder date (`age_record::HOLDER_DATE`) and the match date
/// (`age_record::MATCH_DATE`) (`exe::date_date_54a110`, through `init_default_date`)
/// Original: `youngest_player_record_construct` (0x450c70, club_records.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn youngest_player_record_construct(this: usize) -> usize {
    init_default_date(this); // the age of the youngest
    init_default_date(this + age_record::HOLDER_DATE);
    init_default_date(this + age_record::MATCH_DATE);
    this
}

/// `oldest_player_record_construct`: constructs the oldest player record: the age (+0), the record holder's date
/// (`age_record::HOLDER_DATE`) and the match date (`age_record::MATCH_DATE`) (`exe::date_date_54a110`, through
/// `init_default_date`).
///
/// The original's SEH frame is not needed (`exe::date_date_54a110` does not fail). Returns `this`.
/// Original: `oldest_player_record_construct` (0x450cd0, club_records.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn oldest_player_record_construct(this: usize) -> usize {
    init_default_date(this);
    init_default_date(this + age_record::HOLDER_DATE); // the date of the oldest
    init_default_date(this + age_record::MATCH_DATE);
    this
}

/// Copies the 0x18-byte record holder from `origem` to `destino` (record_holder_assign `record_holder_assign`).
pub(crate) unsafe fn assign_record_holder(target: usize, source: usize) {
    exe::record_holder_assign(target, source);
}

/// copies the rating record (0x1f bytes): the rating (+0), the holder (`rating_record::HOLDER`, through
/// `assign_record_holder`), `rating_record::GAMES` and `rating_record::YEAR`
/// Original: `rating_record_assign` (0x448230, club_records.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn rating_record_assign(this: usize, source: usize) -> usize {
    write_u32(this, read_u32(source)); // a nota
    assign_record_holder(this + rating_record::HOLDER, source + rating_record::HOLDER);
    write_u8(this + rating_record::GAMES, read_u8(source + rating_record::GAMES)); // the rating record byte
    write_u16(this + rating_record::YEAR, read_u16(source + rating_record::YEAR));
    this
}

/// copies the man of the match record (0x1b bytes): the count (+0), the holder (`man_of_match_record::HOLDER`, through
/// `assign_record_holder`) and the year (`man_of_match_record::YEAR`)
/// Original: `man_of_match_record_assign` (0x4484d0, club_records.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn man_of_match_record_assign(this: usize, source: usize) -> usize {
    write_u8(this, read_u8(source)); // the man-of-the-match count
    assign_record_holder(this + man_of_match_record::HOLDER, source + man_of_match_record::HOLDER);
    write_u16(this + man_of_match_record::YEAR, read_u16(source + man_of_match_record::YEAR));
    this
}

/// copies the discipline record (0x1c bytes): yellows (+0), reds (`club_discipline_record::RED_CARDS`), the holder
/// (`club_discipline_record::HOLDER`, through `assign_record_holder`) and the year (`club_discipline_record::YEAR`)
/// Original: `discipline_record_assign` (0x448500, club_records.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn discipline_record_assign(this: usize, source: usize) -> usize {
    write_u8(this, read_u8(source));
    write_u8(this + club_discipline_record::RED_CARDS, read_u8(source + club_discipline_record::RED_CARDS)); // the reds
    assign_record_holder(this + club_discipline_record::HOLDER, source + club_discipline_record::HOLDER);
    write_u16(this + club_discipline_record::YEAR, read_u16(source + club_discipline_record::YEAR));
    this
}

/// copies the count record (0x1a bytes): the count (+0, word) and the holder (`count_record::HOLDER`, through
/// `exe::record_holder_assign`)
/// Original: `count_record_assign` (0x448530, club_records.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn count_record_assign(this: usize, source: usize) -> usize {
    write_u16(this, read_u16(source)); // a contagem
    exe::record_holder_assign(this + count_record::HOLDER, source + count_record::HOLDER);
    this
}

/// copies the fans player record (0x1a bytes): the holder (+0, through `exe::record_holder_assign`) and the year
/// (`fans_player_record::YEAR`)
/// Original: `fans_player_record_assign` (0x4485b0, club_records.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn fans_player_record_assign(this: usize, source: usize) -> usize {
    exe::record_holder_assign(this, source);
    write_u16(this + fans_player_record::YEAR, read_u16(source + fans_player_record::YEAR)); // the supporter’s year
    this
}

/// Constructs the default record date at `a` (`exe::date_date_54a110`).
pub(crate) unsafe fn init_default_date(a: usize) {
    exe::date_date_54a110(a);
}

/// constructs the tally record: the holder date at `tally_record::HOLDER_DATE` (`exe::date_date_54a110`, through
/// `init_default_date`)
/// Original: `tally_record_construct` (0x450c60, club_records.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn tally_record_construct(this: usize) -> usize {
    init_default_date(this + tally_record::HOLDER_DATE); // the date of the occurrences
    this
}

/// constructs the count record: the holder date at `count_record::HOLDER_DATE` (`exe::date_date_54a110`, through
/// `init_default_date`)
/// Original: `count_record_construct` (0x450d30, club_records.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn count_record_construct(this: usize) -> usize {
    init_default_date(this + count_record::HOLDER_DATE); // the counting date
    this
}

/// `transfer_fee_record_assign`: copies the transfer fee record (0x29 bytes) from `source`: five dwords (+0..+0x10: the
/// fee, the player, the other club...), the date at `transfer_fee_record::HOLDER_DATE`, the dword
/// `transfer_fee_record::OTHER_CLUB_ID`, the date at `transfer_fee_record::DATE` and the part-exchange flag
/// (`transfer_fee_record::FLAG`). Returns `this`.
/// Original: `transfer_fee_record_assign` (0x448550, club_records.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn transfer_fee_record_assign(this: usize, source: usize) -> usize {
    let d = |o: usize| write_u32(this + o, read_u32(source + o));
    for o in [0, 4, 8, 0xc, 0x10] {
        d(o); // the value and the ids
    }
    copy_record_date(this + transfer_fee_record::HOLDER_DATE, source + transfer_fee_record::HOLDER_DATE);
    d(transfer_fee_record::OTHER_CLUB_ID);
    copy_record_date(this + transfer_fee_record::DATE, source + transfer_fee_record::DATE);
    write_u8(this + transfer_fee_record::FLAG, read_u8(source + transfer_fee_record::FLAG)); // the swap flag
    this
}

/// `goals_in_match_record_assign`: copies the goals-in-a-match record (0x38 bytes) from `source`: the goals (+0, byte),
/// the record holder at `goals_in_match_record::HOLDER` (`exe::record_holder_assign`) and the match record at
/// `goals_in_match_record::MATCH` (`exe::match_record_assign`, both already in Rust). Returns `this`.
/// Original: `goals_in_match_record_assign` (0x451570, club_records.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn goals_in_match_record_assign(this: usize, source: usize) -> usize {
    write_u8(this, read_u8(source)); // the goals
    exe::record_holder_assign(this + goals_in_match_record::HOLDER, source + goals_in_match_record::HOLDER);
    exe::match_record_assign(this + goals_in_match_record::MATCH, source + goals_in_match_record::MATCH);
    this
}

/// Copies a date from `origem` to `destino` (Date::operator=).
pub(crate) unsafe fn copy_record_date(target: usize, source: usize) {
    let mut local = [0u8; 8];
    exe::date_operator_3d(target, (&mut local) as *mut [u8; 8] as usize, source);
}

/// copies the record holder: four identity fields and the date
/// Original: `record_holder_assign` (0x448490, club_records.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn record_holder_assign(this: usize, source: usize) -> usize {
    let d = |o: usize| write_u32(this + o, read_u32(source + o));
    d(0); // identification
    d(4);
    d(8);
    d(record_holder::STAFF_ID);
    copy_record_date(this + record_holder::DATE, source + record_holder::DATE);
    this
}

/// Copies the match record (0x1f bytes) from `source`: the date, the opponent, the score and the fixture fields.
///
/// The date at +0 (`copy_record_date`), then +8 (dword), `club_match_record::ROUND_PREFIX` and
/// `club_match_record::ROUND_NAME` (words), `club_match_record::VENUE` (byte), `club_match_record::CLUB_ID` and
/// `club_match_record::OPPONENT_ID` (dwords), `club_match_record::GOALS_FOR` and `club_match_record::GOALS_AGAINST`
/// (bytes) and `club_match_record::MATCH_STATS_SLOT` (dword). Returns `this`.
/// Original: `match_record_assign` (0x448110, club_records.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn match_record_assign(this: usize, source: usize) -> usize {
    let d = |o: usize| write_u32(this + o, read_u32(source + o));
    let w = |o: usize| write_u16(this + o, read_u16(source + o));
    let b = |o: usize| write_u8(this + o, read_u8(source + o));
    copy_record_date(this, source);
    d(8); // opponent
    w(club_match_record::ROUND_PREFIX);
    w(club_match_record::ROUND_NAME);
    b(club_match_record::VENUE);
    d(club_match_record::CLUB_ID);
    d(club_match_record::OPPONENT_ID);
    b(club_match_record::GOALS_FOR);
    b(club_match_record::GOALS_AGAINST);
    d(club_match_record::MATCH_STATS_SLOT);
    this
}

/// copies the run record: the game count and the start and end dates
/// Original: `run_record_assign` (0x448170, club_records.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn run_record_assign(this: usize, source: usize) -> usize {
    write_u32(this, read_u32(source)); // jogos
    copy_record_date(this + run_record::START_DATE, source + run_record::START_DATE);
    copy_record_date(this + run_record::END_DATE, source + run_record::END_DATE);
    this
}

/// −1.0f and −1 in the club record fields
/// Original: `reset_records` (0x452fe0, club_records.cpp, cdecl).
pub(crate) unsafe extern "C" fn reset_records(r: usize) -> u32 {
    if r == 0 {
        return original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_RESET_RECORDS)(r);
    }
    let d = |o: usize| write_u32(r + o, u32::MAX);
    write_u32(r + club_record::SEASON_RATING, 0xbf800000);
    for o in [1, 8, 0xe, 0x31, 0x54, 0x77, 0x85, 0xa4, 0xc3, 0xe2, 0x101, 0x118, 0x12c, 0x140, 0x154] {
        d(o);
    }
    for o in [0x168, 0x183, 0x19e, 0x1d6, 0x210, 0x22b] {
        write_u8(r + o, 0xff);
    }
    for o in [0x247, 0x261] {
        write_u16(r + o, 0xffff);
    }
    d(club_record::YOUNGEST_PLAYER_FIXTURE_ID);
    d(club_record::OLDEST_PLAYER_FIXTURE_ID);
    write_u16(r + club_record::INTERNATIONAL_CAPS, 0xffff);
    for o in [0x313, 0x33c, 0x365, 0x36b, 0x37d] {
        d(o);
    }
    1
}

/// builds the record holder: only the date at `record_holder::DATE` (`exe::date_date_54a110`)
/// Original: `record_holder_construct` (0x450d40, club_records.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn record_holder_construct(this: usize) -> usize {
    exe::date_date_54a110(this + record_holder::DATE); // the record holder’s date
    this
}

/// Updates both clubs' season and all-time records after a match, from attendance and scores to the players', queues
/// the record news and lets the club, its chairman, manager and players favour the players who did well.
///
/// `Club_Records::update_records_for_match(fixture, result)` (SEH frame without a destructor). A null fixture or result:
/// the error boxes (lines 0x10c and 0x11b) and 0 (the original); a reserve or minor competition
/// (`exe::comp_is_reserve_or_minor_comp` of [`G_RESERVE_COMP`]): 0. The match's reputation: 1, or the competition's
/// (`competition::REPUTATION`; × 1.5, truncated, in the round 0x96 without prefix), at least 1. Then for the home side
/// and the away side: rival (the opponent is one of `club::RIVAL1`..3), clash (`exe::comp_util_get_league_clash_type`
/// above 0), the season and all-time records (`club_season_record`/`club_all_time_record`, created when missing) and the
/// club's entry (`club_records_entry`); unless it is a friendly of a club that is not a national side, the season year
/// and the club's records (`update_club_match_records`); then for each of the 20 lineup rows that played
/// (`match_lineup_row`) the player's records (`update_player_match_records`) and the liking (`favoured_by_points`); out
/// of friendlies, the season's best average rating is reset (`club_record::SEASON_RATING` −1.0, recomputed by
/// `update_season_rating_record`). Every news item is queued only when the same club and type is not queued yet
/// (`queue_match_record_news`). Returns 1.
/// Original: `Club_Records::update_records_for_match` (0x444930, club_records.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn update_records_for_match(this: usize, fixture: usize, result: usize) -> u32 {
    let fixture = Fixture(fixture);
    let original = || original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_UPDATE_RECORDS_FOR_MATCH)(this, fixture.addr(), result);
    if fixture.is_null() {
        return original(); // the box (line 0x10c)
    }
    let f = fixture;
    if exe::comp_is_reserve_or_minor_comp(read_usize(G_RESERVE_COMP), f.competition()) != 0 {
        return 0;
    }
    if result == 0 {
        return original(); // the box (line 0x11b)
    }
    let mut rep: i16 = 1;
    let comp = f.competition();
    if !comp.is_null() {
        rep = comp.reputation();
        if f.round_prefix() == 0 && f.round_name() == 0x96 {
            covered!("gra_15 round 0x96");
            rep = ftol(rep as f64 * read_f64(rdata::F64_1_5)) as i16;
        }
        if rep < 1 {
            rep = 1;
        }
    }
    let mut view = [0u8; 0x20]; // the club's view of the match (club_match_record), built by the side that plays
    let view = view.as_mut_ptr() as usize;
    exe::date_date_54a110(view);
    for side in 0..2usize {
        let (club, opponent) = if side == 0 { (fixture.home_club().addr(), fixture.away_club().addr()) } else { (fixture.away_club().addr(), fixture.home_club().addr()) };
        let club = Club(club);
        let rival = club.rival1() == opponent || club.rival2() == opponent || club.rival3() == opponent;
        let clash = (exe::comp_util_get_league_clash_type(club, opponent) as i8) > 0;
        let season = club_season_record(this, club.addr(), 1);
        let all_time = club_all_time_record(this, club.addr(), 1);
        let entry = club_records_entry(this, club.addr());
        let mut year = 0u16;
        if exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), f.competition()) == 0 || exe::database_is_national_team(club) != 0 {
            year = exe::key_nation_get_club_season_year(club);
            update_club_match_records(this, f, result, club, entry, (season, all_time), view);
        }
        for k in 0..20 {
            let Some((row, person, pdata)) = match_lineup_row(result, side, k) else { continue };
            let c = PlayerRecordsCtx { this, club, entry, season, all_time, person: Staff(person), year, view };
            if let Some(points) = update_player_match_records(c, f, result, row, pdata) {
                favoured_by_points(club, Staff(person), points, rep, rival, clash);
            }
        }
        if exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), f.competition()) == 0 {
            write_u32(season + club_record::SEASON_RATING, 0xbf800000); // -1.0
        }
    }
    1
}

/// Comparer for `update_records_for_match`.
///
/// Side by side from the same state, after creating the four records in the original's order (home season, home
/// all-time, away season, away all-time: `get_season_record` and `get_all_time_record` allocate) and fetching them again
/// (a new one may move the others). Compared, each by name: the four records (0x38b bytes) and the two club entries
/// (0x6a), the generator (`crate::rng::STATE_REGIONS`), the favoured lists that the liking may change (each club's
/// three favoured and disliked staff, `club::FAV_STAFF1`, and those of its chairman, manager and 50 squad members,
/// `preferences::STAFF_FAVOURITE_STAFF1`), the news queue items the original appended (it may reallocate the queue:
/// the copy runs dry, `MATCH_RECORDS_NEWS_DRY_RUN`, looking only at the items queued before) and the return value. A
/// null fixture or result or a reserve competition: only the original.
pub(crate) unsafe extern "thiscall" fn compare_update_records_for_match(this: usize, fixture: usize, result: usize) -> u32 {
    let fixture = Fixture(fixture);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_UPDATE_RECORDS_FOR_MATCH);
    if fixture.is_null() || result == 0 || exe::comp_is_reserve_or_minor_comp(read_usize(G_RESERVE_COMP), fixture.competition()) != 0 {
        return o(this, fixture.addr(), result);
    }
    let clubs = [fixture.home_club(), fixture.away_club()];
    for c in clubs {
        club_season_record(this, c.addr(), 1);
        club_all_time_record(this, c.addr(), 1);
    }
    let mut named: Vec<(&str, Vec<(usize, usize)>)> = Vec::new();
    let mut likes = crate::rng::STATE_REGIONS.to_vec();
    for (side, c) in clubs.iter().enumerate() {
        let name = if side == 0 { ["home season", "home all-time", "home entry"] } else { ["away season", "away all-time", "away entry"] };
        named.push((name[0], vec![(club_season_record(this, c.addr(), 0), 0x38b)]));
        named.push((name[1], vec![(club_all_time_record(this, c.addr(), 0), 0x38b)]));
        named.push((name[2], vec![(club_records_entry(this, c.addr()), 0x6a)]));
        likes.push((c.addr() + club::FAV_STAFF1, 24));
        let staff = [c.chairman().addr(), c.manager().addr()].into_iter().chain((0..50).map(|k| c.squad(k)));
        for s in staff {
            if s != 0 && Staff(s).staff_preferences() != 0 {
                likes.push((Staff(s).staff_preferences() + preferences::STAFF_FAVOURITE_STAFF1, 24));
            }
        }
    }
    named.push(("generator and favoured lists", likes));
    let regions: Vec<(usize, usize)> = named.iter().flat_map(|(_, r)| r.iter().copied()).collect();
    let before_count = read_i32(this + club_records_fields::NEWS_QUEUE_COUNT);
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, fixture.addr(), result), || {
        *MATCH_RECORDS_NEWS_DRY_RUN.lock().unwrap() = Some((before_count, Vec::new()));
        crate::rng::without_counting(|| update_records_for_match(this, fixture.addr(), result))
    });
    let queued_rust = MATCH_RECORDS_NEWS_DRY_RUN.lock().unwrap().take().map(|(_, v)| v).unwrap_or_default();
    let queue = read_usize(this + club_records_fields::NEWS_QUEUE);
    let queued_orig: Vec<(u32, u8)> = (before_count.max(0)..read_i32(this + club_records_fields::NEWS_QUEUE_COUNT)).map(|k| (read_u32(queue + 6 * k as usize), read_u8(queue + 6 * k as usize + record_news_item::RECORD_TYPE))).collect();
    SCOREBOARD.record(equal && ro == rr && queued_orig == queued_rust, || {
        let flat = |m: &crate::comparison::MemorySnapshot| m.0.iter().flat_map(|(_, v)| v.iter().copied()).collect::<Vec<u8>>();
        let (a, b) = (flat(&mo), flat(&mr));
        let mut k = 0;
        let mut differ = Vec::new();
        for (name, r) in &named {
            let n: usize = r.iter().map(|x| x.1).sum();
            if let Some(off) = (0..n).find(|&i| a.get(k + i) != b.get(k + i)) {
                differ.push(format!("{} (+{:#x}: {:02x?} x {:02x?})", name, off, a.get(k + off), b.get(k + off)));
            }
            k += n;
        }
        format!("update_records_for_match(fixture {:#x}) returned {} x {}, differ in {:?}, news original {:?} rust {:?}", fixture.addr(), ro, rr, differ, queued_orig, queued_rust)
    });
    ro
}

/// Writes the headline or the body of the news that a club broke one of its records (news type 0x3e8, from
/// `send_record_news`).
///
/// `write_record_news_text(news, full, out)`. Without the Text, the error box (club_records.cpp, line 0xee7) and 0;
/// the Text is cleared and set empty; without the news item, the box (0xef3) and 0. The local record copy is
/// constructed as the original does (`RECORD_NEWS_CONSTRUCTORS`); slot 0 is the club id, slot 1 the record type, then
/// the type's fields (`read_record_news_fields`; an unknown type: the box at line 0x1b02, and on). A club id outside
/// the club table: the box (0xefe) and 0.
///
/// The headline (`full` = 0): type 32 "<club> <Fans Player of the Year> decided" (0x98de00, the award name 0x98de58),
/// with the club's link and the award's link (0x7e6, to the club); otherwise "New <club> record" for a national side
/// (`exe::database_is_national_team`, 0x98dde0) or "New <club> club record" (0x98ddb8), with the club's link. 1.
///
/// The body, by type, each only when its record holds (otherwise an empty text), with the club's short name and gender
/// (`club::SHORT_NAME`, `club::SHORT_GENDER_NAME`) and its link:
/// - 1, 2: the final league position (the ordinal of position + 1, `exe::utils_format_ordinal`) in the competition
///   (its short name and gender), record high (0x98dd28) or low (0x98dc98), when the competition is set; the
///   competition's link (0x7d0);
/// - 3, 5: the attendance at home to the opponent in the round (the label of form 2 and its gender), record high
///   (0x98dbf4) or low (0x98da98); 4: the gate receipts (`record_money_text`, flags 0x22, 0x98db3c); the opponent's
///   link and the round label's link (0x7d0, to the competition); 6: the average attendance (0x98da2c);
/// - 7, 8, 9: the record win, league win and defeat: the score "%d-%d" (the C sprintf; the club's goals first, in the
///   defeat the opponent's first), the template by the venue (at home, away, against); 10, 11: the highest scoring
///   game and league game, the score with the higher count first, the template by the result (win, defeat, draw);
///   the links of the opponent, the round and the score (0x7d3, when the fixture still matches);
/// - 12, 13: the runs of wins and defeats (more than one game) between the start and the end dates
///   (`exe::date_get_long_date_string_no_year`); 14, 15: unbeaten and without winning, from the start date;
/// - 16, 17: a player's goals and league goals in the season; 18: the goals in one match (with the match: venue,
///   opponent, round); 19: assists; 20: the average rating (above −1.0, "%4.2f" of the float) with the games; 21: man
///   of the match awards; 22: yellow and red cards; 23, 24: league appearances and goals in his club career; 25, 26:
///   the youngest and oldest appearance (the age by `database_format_years_between_dates` from the birth date to
///   the match, the template by the venue); 27: the most caps for the nation; 28, 29: the record purchase and sale
///   (the fee with flags 6, the other club, "part-exchange" when the flag is set); 32: the fans' player of the year
///   (the club's long name and gender, "Player of the Year" 0x98c380 with its link 0x7e6). The player's name (mode 1)
///   and his link (0x7d7) when his names still match;
/// - 30, 31: the season's transfer spending and income (flags 6).
///
/// An unknown type: the box (line 0x14a7) and 0; otherwise 1.
/// Original: `club_records_write_record_news_text` (0x44c650, club_records.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_record_news_text(_this: usize, news: usize, full: u32, out: usize) -> u32 {
    use record_news_fns::CLUB_RECORDS_CPP as FILE;
    let fmt = news_text_format();
    if out == 0 {
        show_error_at(FILE, 0xee7);
        return 0;
    }
    let clear: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_TEXT_CLEAR); // Text::clear
    clear(out);
    exe::text_set_string(out, EMPTY_STRING as *const u8);
    if news == 0 {
        show_error_at(FILE, 0xef3);
        return 0;
    }
    covered!("gra_18 record news");
    let mut record = [0u8; 0x390];
    let r = record.as_mut_ptr() as usize;
    for (o, f) in RECORD_NEWS_CONSTRUCTORS {
        let construct: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(f);
        construct(r + o);
    }
    let mut k: u8 = 1;
    let id = exe::newsitem_get_slot(news, 0) as i32;
    let kind = exe::newsitem_get_slot(news, k as u32) as u8;
    k += 1;
    let known = read_record_news_fields(news, kind, r, &mut k);
    if !known {
        show_error_at(FILE, 0x1b02);
    }
    if id < 0 || id > read_i32(G_CLUB_COUNT) - 1 {
        show_error_at(FILE, 0xefe);
        return 0;
    }
    let club = club_at(id as isize);
    let name = club.addr() + club::SHORT_NAME;
    let gender = (club.short_gender_name() as i8) as i32;
    if full == 0 {
        if kind == 0x20 {
            let mut award = [0u8; 0x100];
            fmt(award.as_mut_ptr() as usize, 0x98de58); // "Fans Player of the Year"
            fmt(NEWS_TEXT_BUFFER, 0x98de00, gender, gender, name, award.as_ptr() as usize);
            exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
            link_record_club(out, name, club);
            exe::text_add_link(out, award.as_ptr(), 0x7e6, club);
        } else {
            let headline = if exe::database_is_national_team(club) != 0 { 0x98dde0 } else { 0x98ddb8 };
            fmt(NEWS_TEXT_BUFFER, headline, gender, gender, name);
            exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
            link_record_club(out, name, club);
        }
        return 1;
    }
    if !known {
        show_error_at(FILE, 0x14a7);
        return 0;
    }
    covered!("gra_18 record news body");
    let mut text = [0u8; 1000];
    let tp = text.as_mut_ptr() as usize;
    let set = |out: usize| {
        exe::text_set_string(out, tp as *const u8);
    };
    let (mut b1, mut b2, mut b3) = ([0u8; 0xc8], [0u8; 0xc8], [0u8; 0xc8]);
    let mut score = [0u8; 20];
    let sp = score.as_mut_ptr() as usize;
    let crt_format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let comp_at = |idx: i32| read_usize(G_CLUB_COMP_TABLE).wrapping_add((idx as usize).wrapping_mul(0x6b));
    match kind {
        // the final league position, high and low
        1 | 2 => {
            let (pos, idx) = if kind == 1 { (read_u8(r), read_i32(r + 1)) } else { (read_u8(r + 7), read_i32(r + 8)) };
            if idx > -1 {
                let ordinal = exe::utils_format_ordinal(pos as i32 + 1, -1); // ponytail: the original copies it into 20 bytes first
                let comp = comp_at(idx);
                let cg = read_i8(comp + competition::SHORT_GENDER_NAME) as i32;
                let template = if kind == 1 { 0x98dd28 } else { 0x98dc98 };
                fmt(tp, template, gender, gender, cg, cg, name, ordinal, comp + competition::SHORT_NAME);
                set(out);
                link_record_club(out, name, club);
                exe::text_add_link(out, (comp + competition::SHORT_NAME) as *const u8, 0x7d0, comp);
            }
        }
        // the attendance high and low and the gate receipts, at home to the opponent in a round
        3 | 4 | 5 => {
            let (value_at, m) = match kind { 3 => (0xe, 0x12), 4 => (0x31, 0x35), _ => (0x54, 0x58) };
            let value = read_i32(r + value_at);
            if value > -1 {
                let m = RecordMatch(r + m);
                let opp = m.opponent();
                let og = (opp.short_gender_name() as i8) as i32;
                let rg = m.round_label(&mut b1);
                let oname = opp.addr() + club::SHORT_NAME;
                if kind == 4 {
                    record_money_text(value as u32, 0x22, &mut b2);
                    fmt(tp, 0x98db3c, gender, gender, og, og, rg, rg, name, b2.as_ptr() as usize, oname, b1.as_ptr() as usize);
                } else {
                    let template = if kind == 3 { 0x98dbf4 } else { 0x98da98 };
                    fmt(tp, template, gender, gender, og, og, rg, rg, name, value, oname, b1.as_ptr() as usize);
                }
                set(out);
                link_record_club(out, name, club);
                link_record_club(out, oname, opp);
                exe::text_add_link(out, b1.as_ptr(), 0x7d0, m.comp());
            }
        }
        // the average attendance
        6 => {
            let value = read_i32(r + 0x77);
            if value > -1 {
                fmt(tp, 0x98da2c, gender, gender, name, value);
                set(out);
                link_record_club(out, name, club);
            }
        }
        // the record win, league win and defeat, the highest scoring game and league game
        7..=11 => {
            let m = RecordMatch(r + [0x7d, 0x9c, 0xbb, 0xda, 0xf9][kind as usize - 7]);
            if m.comp_index() > -1 {
                let (g1, g2) = m.goals();
                let (a, b) = match kind {
                    9 => (g2, g1),
                    10 | 11 => (g1.max(g2), g1.min(g2)),
                    _ => (g1, g2),
                };
                crt_format(sp, 0x98c114, a as i32, b as i32); // "%d-%d"
                let opp = m.opponent();
                let og = (opp.short_gender_name() as i8) as i32;
                let oname = opp.addr() + club::SHORT_NAME;
                let rg = m.round_label(&mut b1);
                let by_venue = |home: usize, away: usize, against: usize| match m.venue() {
                    1 => home,
                    2 => away,
                    _ => against,
                };
                let by_result = |win: usize, defeat: usize, draw: usize| if g1 > g2 { win } else if g1 < g2 { defeat } else { draw };
                let template = match kind {
                    7 => by_venue(0x98d998, 0x98d908, 0x98d878),
                    8 => by_venue(0x98d7dc, 0x98d740, 0x98d6a4),
                    9 => by_venue(0x98d608, 0x98d570, 0x98d4d8),
                    10 => by_result(0x98d438, 0x98d394, 0x98d2f8),
                    _ => by_result(0x98d250, 0x98d1a4, 0x98d0fc),
                };
                fmt(tp, template, gender, gender, og, og, rg, rg, name, sp, oname, b1.as_ptr() as usize);
                set(out);
                link_record_club(out, name, club);
                link_record_club(out, oname, opp);
                exe::text_add_link(out, b1.as_ptr(), 0x7d0, m.comp());
                m.link_score(out, &score);
            }
        }
        // the runs: won, lost (start and end dates), unbeaten, without winning (start date)
        12..=15 => {
            let run = r + [0x118, 0x12c, 0x140, 0x154][kind as usize - 12];
            let games = read_i32(run);
            if games > 1 {
                let start = run + run_record::START_DATE;
                if kind <= 13 {
                    exe::date_get_long_date_string_no_year(start, b1.as_mut_ptr() as usize, 0xc8, 0xff);
                    exe::date_get_long_date_string_no_year(run + run_record::END_DATE, b2.as_mut_ptr() as usize, 0xc8, 0xff);
                    let template = if kind == 12 { 0x98d074 } else { 0x98cfec };
                    fmt(tp, template, gender, gender, 0x64, 0x64, name, games, b1.as_ptr() as usize, b2.as_ptr() as usize);
                } else {
                    exe::date_get_long_date_string_no_year(start, b1.as_mut_ptr() as usize, 0xc8, 0xff);
                    let template = if kind == 14 { 0x98cf80 } else { 0x98cf0c };
                    fmt(tp, template, gender, gender, 0x64, name, games, b1.as_ptr() as usize);
                }
                set(out);
                link_record_club(out, name, club);
            }
        }
        // the player records with a count: goals, league goals, assists, man of the match, cards, career
        16 | 17 | 19 | 21 | 22 | 23 | 24 | 27 => {
            let (count, holder) = match kind {
                16 => (read_i8(r + 0x168) as i32, 0x169),
                17 => (read_i8(r + 0x183) as i32, 0x184),
                19 => (read_i8(r + 0x1d6) as i32, 0x1d7),
                21 => (read_i8(r + 0x210) as i32, 0x211),
                22 => (read_i8(r + 0x22b) as i32, 0x22d),
                23 => (read_i16(r + 0x247) as i32, 0x249),
                24 => (read_i16(r + 0x261) as i32, 0x263),
                _ => (read_i16(r + 0x2f9) as i32, 0x2fb),
            };
            if count > -1 {
                let h = RecordHolder(r + holder);
                h.name(&mut b1);
                let player = b1.as_ptr() as usize;
                match kind {
                    16 => fmt(tp, 0x98ceac, 0x64, gender, gender, player, count, name),
                    17 => fmt(tp, 0x98ce44, 0x64, gender, gender, player, count, name),
                    19 => fmt(tp, 0x98cbb4, 0x64, gender, gender, count, player, name),
                    21 => fmt(tp, 0x98ca88, 0x64, gender, gender, count, player, name),
                    22 => fmt(tp, 0x98c9fc, 0x64, gender, gender, player, count, read_i8(r + 0x22c) as i32, name),
                    23 => fmt(tp, 0x98c980, 0x64, gender, gender, count, player, name),
                    24 => fmt(tp, 0x98c90c, 0x64, gender, gender, count, player, name),
                    _ => fmt(tp, 0x98c668, gender, gender, player, name, count),
                };
                set(out);
                link_record_club(out, name, club);
                h.link(out, &b1);
            }
        }
        // the goals in one match
        18 => {
            let goals = read_i8(r + 0x19e) as i32;
            if goals > -1 {
                let h = RecordHolder(r + 0x19f);
                h.name(&mut b1);
                let m = RecordMatch(r + 0x1b7);
                let opp = m.opponent();
                let og = (opp.short_gender_name() as i8) as i32;
                let oname = opp.addr() + club::SHORT_NAME;
                let rg = m.round_label(&mut b2);
                let template = match m.venue() { 1 => 0x98cd8c, 2 => 0x98ccd8, _ => 0x98cc24 };
                fmt(tp, template, 0x64, gender, gender, og, og, rg, rg, goals, b1.as_ptr() as usize, name, oname, b2.as_ptr() as usize);
                set(out);
                link_record_club(out, name, club);
                link_record_club(out, oname, opp);
                exe::text_add_link(out, b2.as_ptr(), 0x7d0, m.comp());
                h.link(out, &b1);
            }
        }
        // the average rating
        20 => {
            let rating = read_f32(r + club_record::SEASON_RATING);
            if rating as f64 > read_f64(rdata::F64_NEG_1) {
                let h = RecordHolder(r + 0x1f5);
                h.name(&mut b1);
                fmt(tp, 0x98cb10, 0x64, gender, gender, rating as f64, b1.as_ptr() as usize, read_i8(r + club_record::SEASON_RATING_GAMES) as i32, name);
                set(out);
                link_record_club(out, name, club);
                h.link(out, &b1);
            }
        }
        // the youngest and the oldest appearance
        25 | 26 => {
            let base = if kind == 25 { 0x27b } else { 0x2ba };
            let m = RecordMatch(r + base + 0x20);
            if m.comp_index() > -1 {
                let h = RecordHolder(r + base + 8);
                h.name(&mut b1);
                let years: unsafe extern "C" fn(usize, usize, usize, i32) -> u32 = core::mem::transmute(FN_DATABASE_FORMAT_YEARS_BETWEEN_DATES);
                years(m.0, r + base, sp, 5);
                let opp = m.opponent();
                let og = (opp.short_gender_name() as i8) as i32;
                let oname = opp.addr() + club::SHORT_NAME;
                let rg = m.round_label(&mut b2);
                let template = match m.venue() { 1 => 0x98c850, 2 => 0x98c798, _ => 0x98c6e0 };
                fmt(tp, template, 0x64, gender, gender, og, og, rg, rg, b1.as_ptr() as usize, sp, name, oname, b2.as_ptr() as usize);
                set(out);
                link_record_club(out, name, club);
                link_record_club(out, oname, opp);
                exe::text_add_link(out, b2.as_ptr(), 0x7d0, m.comp());
                h.link(out, &b1);
            }
        }
        // the record purchase and sale
        28 | 29 => {
            let base = if kind == 28 { club_record::FEE_PAID } else { club_record::FEE_RECEIVED };
            let fee = read_i32(r + base);
            if fee > -1 {
                let h = RecordHolder(r + base + 4);
                h.name(&mut b1);
                let other = club_at(read_i32(r + base + 0x1c) as isize);
                let xg = (other.short_gender_name() as i8) as i32;
                let xname = other.addr() + club::SHORT_NAME;
                record_money_text(fee as u32, 6, &mut b2);
                let exchange = read_u8(r + base + 0x28) != 0;
                let (money, player) = (b2.as_ptr() as usize, b1.as_ptr() as usize);
                if kind == 28 {
                    let template = if exchange { 0x98c5d8 } else { 0x98c554 };
                    fmt(tp, template, 0x64, xg, xg, gender, gender, money, player, xname, name);
                } else {
                    let template = if exchange { 0x98c4c8 } else { 0x98c448 };
                    fmt(tp, template, 0x64, gender, gender, xg, xg, money, player, name, xname);
                }
                set(out);
                link_record_club(out, name, club);
                link_record_club(out, xname, other);
                h.link(out, &b1);
            }
        }
        // the season's transfer spending and income
        30 | 31 => {
            let value = read_i32(r + if kind == 30 { club_record::TRANSFER_SPENDING } else { club_record::TRANSFER_INCOME });
            if value > -1 {
                record_money_text(value as u32, 6, &mut b1);
                fmt(tp, if kind == 30 { 0x98c3f0 } else { 0x98c394 }, gender, gender, b1.as_ptr() as usize, name);
                set(out);
                link_record_club(out, name, club);
            }
        }
        // the fans' player of the year
        _ => {
            let h = RecordHolder(r + club_record::FANS_PLAYER);
            if read_i32(h.0 + 0xc) > -1 {
                h.name(&mut b1);
                let long_name = club.addr() + club::NAME;
                let lg = club.gender_name() as i32;
                fmt(b3.as_mut_ptr() as usize, 0x98c380); // "Player of the Year"
                fmt(tp, 0x98c2f0, lg, lg, 0x64, b1.as_ptr() as usize, long_name, b3.as_ptr() as usize);
                set(out);
                link_record_club(out, long_name, club);
                exe::text_add_link(out, b3.as_ptr(), 0x7e6, club);
                h.link(out, &b1);
            }
        }
    }
    1
}

/// Comparer for `write_record_news_text`: the text side by side (`compare_text_writer`) and the return. Without the
/// Text or the news item, with a club id outside the table or an unknown type (the error boxes), only the original.
pub(crate) unsafe extern "thiscall" fn compare_write_record_news_text(this: usize, news: usize, full: u32, out: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize) -> u32>(IDX_WRITE_RECORD_NEWS_TEXT);
    if out == 0 || news == 0 {
        return o(this, news, full, out);
    }
    let id = exe::newsitem_get_slot(news, 0) as i32;
    let kind = exe::newsitem_get_slot(news, 1) as u8;
    if id < 0 || id > read_i32(G_CLUB_COUNT) - 1 || record_news_fields(kind).is_empty() {
        return o(this, news, full, out);
    }
    let (ro, rr) = (core::cell::Cell::new(0u32), core::cell::Cell::new(0u32));
    compare_text_writer(out, || ro.set(o(this, news, full, out)), |t| rr.set(write_record_news_text(this, news, full, t)),
        || format!("write_record_news_text(type {}, full {})", kind, full));
    SCOREBOARD.record(ro.get() == rr.get(), || format!("write_record_news_text(type {}) returned {} x {}", kind, ro.get(), rr.get()));
    ro.get()
}

/// Constructs a club's season history snapshot (0x389 bytes): runs the Date constructors of its records in the
/// original's order: the dates at +0x12 and +0x35 (`exe::date_date_54a110`); the match records at +0x58, +0x7d, +0x9c,
/// +0xbb, +0xda and +0xf9 (0x645800, which only constructs their Date); the run records at +0x118, +0x12c, +0x140
/// and +0x154 (`run_record_construct`); the record holders at +0x169, +0x184 and +0x19f (`record_holder_construct`),
/// the match record at +0x1b7; the holders at +0x1d7, +0x1f5, +0x211, +0x22d, +0x249 and +0x263; two blocks of a Date,
/// a holder at +8 and a match record at +0x20 (+0x27b and +0x2ba); the holder at +0x2fb; the transfer fee records at
/// +0x313 (`transfer_fee_record_construct`) and +0x33c (`transfer_fee_record_construct_2`); the holder at +0x371.
/// Returns `this`.
/// Original: `season_history_entry_construct` (0x448260, club_records.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn season_history_entry_construct(this: usize) -> usize {
    let mtch: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(record_news_fns::DATE_CONSTRUCT_645800);
    exe::date_date_54a110(this + 0x12);
    exe::date_date_54a110(this + 0x35);
    for o in [0x58, 0x7d, 0x9c, 0xbb, 0xda, 0xf9] {
        mtch(this + o);
    }
    for o in [0x118, 0x12c, 0x140, 0x154] {
        run_record_construct(this + o);
    }
    for o in [0x169, 0x184, 0x19f] {
        record_holder_construct(this + o);
    }
    mtch(this + 0x1b7);
    for o in [0x1d7, 0x1f5, 0x211, 0x22d, 0x249, 0x263] {
        record_holder_construct(this + o);
    }
    for b in [0x27b, 0x2ba] {
        covered!("sma_3 season snapshot");
        exe::date_date_54a110(this + b);
        record_holder_construct(this + b + 8); // sma3: the holder of the block
        mtch(this + b + 0x20);
    }
    record_holder_construct(this + 0x2fb);
    transfer_fee_record_construct(this + 0x313);
    transfer_fee_record_construct_2(this + 0x33c);
    record_holder_construct(this + 0x371);
    this
}

/// Comparer for `season_history_entry_construct`: side by side over the 0x389 bytes of the snapshot, and the return.
pub(crate) unsafe extern "thiscall" fn compare_season_history_entry_construct(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_SEASON_HISTORY_ENTRY_CONSTRUCT);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(this, 0x389)], || o(this), || season_history_entry_construct(this));
    SCOREBOARD.record(igual && r == n, || format!("season_history_entry_construct {:x?} x {:x?}", mo, mr));
    r
}

/// Writes the club id, the record type and the fields of one broken club record into the slots of a news item.
///
/// `club_records_write_record_news_fields(club_id, record, kind, news)`: without the record (error line 0x1829) or
/// the news item (0x1831), the game's error box and 0. Slot 0 the club id, slot 1 the type (`kind`, signed), then the
/// type's fields (`record_news_fields`) from slot 2 on (`exe::newsitem_set_slot`; the match, holder and run records
/// write their own slots and advance the index). A type outside 1..=32: the error box (line 0x1989) after slots 0 and
/// 1, and 0. Otherwise 1.
/// Original: `club_records_write_record_news_fields` (0x453120, club_records.cpp, cdecl).
pub(crate) unsafe extern "C" fn club_records_write_record_news_fields(club_id: u32, record: usize, kind: u32, news: usize) -> u32 {
    use record_news_fns::CLUB_RECORDS_CPP as FILE;
    if record == 0 {
        show_error_at(FILE, 0x1829);
        return 0;
    }
    if news == 0 {
        show_error_at(FILE, 0x1831);
        return 0;
    }
    let mut k: u8 = 1;
    exe::newsitem_set_slot(news, 0, club_id);
    let kind = kind as u8 as i8;
    exe::newsitem_set_slot(news, 1, kind as i32 as u32); // sma3: the record type
    k += 1;
    let fields = record_news_fields(kind as u8);
    if fields.is_empty() {
        show_error_at(FILE, 0x1989);
        return 0;
    }
    covered!("sma_3 record news fields");
    let run: unsafe extern "C" fn(usize, usize, *mut u8) -> u32 = core::mem::transmute(record_news_fns::WRITE_RUN_RECORD_TO_NEWS);
    let slot = |v: u32, k: &mut u8| {
        exe::newsitem_set_slot(news, *k as i8, v);
        *k = k.wrapping_add(1);
    };
    for f in fields {
        let r = record;
        match *f {
            RecordNewsField::Byte(o) => slot(read_u8(r + o) as u32, &mut k),
            RecordNewsField::SByte(o) => slot(read_i8(r + o) as i32 as u32, &mut k), // sma3: a signed byte field
            RecordNewsField::Word(o) => slot(read_u16(r + o) as u32, &mut k),
            RecordNewsField::SWord(o) => slot(read_i16(r + o) as i32 as u32, &mut k),
            RecordNewsField::Dword(o) => slot(read_u32(r + o), &mut k),
            RecordNewsField::Percent(o) => slot(ftol(read_f32(r + o) as f64 * read_f32(rdata::F32_100) as f64) as u32, &mut k), // sma3: the percentage
            RecordNewsField::Match(o) => {
                exe::write_match_record_to_news(r + o, news, &mut k);
            }
            RecordNewsField::Holder(o) => exe::write_staff_name_ref_to_news(r + o, news, &mut k),
            RecordNewsField::Run(o) => {
                run(r + o, news, &mut k);
            }
        }
    }
    1
}

/// Comparer for `club_records_write_record_news_fields`: side by side over the news item (0xe0 bytes) and the return;
/// without the record or the news item, or with a type outside 1..=32, only the original (the error box).
pub(crate) unsafe extern "C" fn compare_club_records_write_record_news_fields(club_id: u32, record: usize, kind: u32, news: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(u32, usize, u32, usize) -> u32>(IDX_CLUB_RECORDS_WRITE_RECORD_NEWS_FIELDS);
    if record == 0 || news == 0 || record_news_fields(kind as u8).is_empty() {
        return o(club_id, record, kind, news);
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(news, 0xe0)], || o(club_id, record, kind, news),
        || club_records_write_record_news_fields(club_id, record, kind, news));
    SCOREBOARD.record(igual && r == n, || format!("club_records_write_record_news_fields(kind {}) {:x?} x {:x?}", kind as u8, mo, mr));
    r
}

/// Queues the record news of `club` for the last of `kinds`, unless a news item of one of `kinds` (looked up in that
/// order, each over the whole queue) is already queued for the club. Returns the kind found, or `None` when it queued.
pub(crate) unsafe fn queue_match_record_news(this: usize, club: Club, kinds: &[u8]) -> Option<u8> {
    let id = read_u32(club.addr());
    let mut dry = MATCH_RECORDS_NEWS_DRY_RUN.lock().unwrap();
    let real = match &*dry {
        Some((n, _)) => *n,
        None => read_i32(this + club_records_fields::NEWS_QUEUE_COUNT),
    };
    let queue = read_usize(this + club_records_fields::NEWS_QUEUE);
    for &kind in kinds {
        let in_queue = (0..real.max(0) as usize).any(|k| read_u32(queue + 6 * k) == id && read_u8(queue + 6 * k + record_news_item::RECORD_TYPE) == kind);
        if in_queue || dry.as_ref().map_or(false, |(_, v)| v.contains(&(id, kind))) {
            return Some(kind);
        }
    }
    let kind = kinds[kinds.len() - 1];
    match &mut *dry {
        Some((_, v)) => v.push((id, kind)),
        None => {
            drop(dry);
            exe::club_records_queue_record_news(this, id, kind as u32);
        }
    }
    None
}

/// One more game in the run at `run` of the club's entry: the count goes up (the first game sets the start date) and
/// the end date is today ([`G_CURRENT_DATE`]). If the run beats the season record at `at` (type `kind`, when it
/// applies; −1 = no record), the season record takes the count and both dates; if it also beats the all-time record:
/// the news when the run is at least `news_from` games long and did not start on the all-time record's start date (the
/// same run already recorded), and the all-time record takes the count and the dates.
pub(crate) unsafe fn run_record(this: usize, club: Club, entry: usize, records: (usize, usize), kind: u8, run: usize, at: usize, news_from: i32) {
    let (season, all_time) = records;
    let n = read_i32(entry + run).wrapping_add(1);
    write_i32(entry + run, n);
    if n == 1 {
        copy_record_date(entry + run + run_record::START_DATE, G_CURRENT_DATE);
    }
    copy_record_date(entry + run + run_record::END_DATE, G_CURRENT_DATE);
    if exe::club_records_record_type_applies(kind as u32, club, 1) == 0 {
        return;
    }
    let s = read_i32(season + at);
    if s != -1 && read_i32(entry + run) <= s {
        return;
    }
    write_i32(season + at, read_i32(entry + run));
    copy_record_date(season + at + run_record::START_DATE, entry + run + run_record::START_DATE);
    copy_record_date(season + at + run_record::END_DATE, entry + run + run_record::END_DATE);
    let a = read_i32(all_time + at);
    if a != -1 && read_i32(entry + run) <= a {
        return;
    }
    if read_i32(entry + run) >= news_from {
        let (start, recorded) = (entry + run + run_record::START_DATE, all_time + at + run_record::START_DATE);
        if read_u16(recorded + 2) != read_u16(start + 2) || read_u16(recorded) != read_u16(start) {
            covered!("gra_15 run record ever");
            queue_match_record_news(this, club, &[kind]);
        }
    }
    write_i32(all_time + at, read_i32(entry + run));
    copy_record_date(all_time + at + run_record::START_DATE, entry + run + run_record::START_DATE);
    copy_record_date(all_time + at + run_record::END_DATE, entry + run + run_record::END_DATE);
}

/// The club's records of the match (the part of `update_records_for_match` before the players).
///
/// One more game this season (`club_record_entry::SEASON_GAMES`); the club's view of the match
/// (`exe::fill_match_record_from_fixture` into `view`). At home (`club_match_record::VENUE` 1): one more home game
/// (`club_entry_runs::HOME_GAMES`, `club_entry_runs::HOME_GAMES_TOTAL`), the attendance (`match_record::ATTENDANCE`)
/// added to the season's (`club_entry_runs::HOME_ATTENDANCE`) and the season average
/// (`club_match_records::AVERAGE_ATTENDANCE`) = total / home games, its year word 0. Then, each when its type applies
/// to the club (`exe::club_records_record_type_applies`, period 1), through `attendance_record`, `result_record` and
/// `run_record`:
/// - highest attendance, highest gate receipts (`match_record::GATE_RECEIPTS`) and lowest attendance, at home and with
///   a value above 0;
/// - biggest win (won, `match_record_is_bigger_win`; news by 6 goals or more), biggest league win (the same in a league
///   division, `exe::comp_util_is_league_division`; not again when the biggest win's news is queued), biggest defeat
///   (lost, `match_record_is_bigger_defeat`; 6 or more), highest scoring game (goals, `match_record_has_more_goals`;
///   news from 8 goals) and highest scoring league game;
/// - the runs: a win adds to the run of wins (news from 6) and ends the runs of defeats and without winning; a defeat
///   adds to the defeats (6) and ends the wins and the unbeaten run; a win or a draw adds to the unbeaten run (12) and
///   ends the defeats; a draw or a defeat adds to the run without winning (12) and ends the wins.
/// Goals are the club's view (`club_match_record::GOALS_FOR`/`GOALS_AGAINST`, signed bytes, the extra-time score if
/// any).
pub(crate) unsafe fn update_club_match_records(this: usize, f: Fixture, result: usize, club: Club, entry: usize, records: (usize, usize), view: usize) {
    use club_match_records as r;
    use club_record_kind as k;
    let (season, _) = records;
    write_u8(entry + club_record_entry::SEASON_GAMES, read_u8(entry + club_record_entry::SEASON_GAMES).wrapping_add(1));
    exe::fill_match_record_from_fixture(f, club, view);
    let attendance = read_i32(result + match_record::ATTENDANCE);
    if read_u8(view + club_match_record::VENUE) == 1 {
        covered!("gra_15 home match");
        let home = read_i8(entry + club_entry_runs::HOME_GAMES).wrapping_add(1);
        write_i16(entry + club_entry_runs::HOME_GAMES_TOTAL, read_i16(entry + club_entry_runs::HOME_GAMES_TOTAL).wrapping_add(1));
        write_i8(entry + club_entry_runs::HOME_GAMES, home);
        let total = read_i32(entry + club_entry_runs::HOME_ATTENDANCE).wrapping_add(attendance);
        write_i32(entry + club_entry_runs::HOME_ATTENDANCE, total);
        // ponytail: the original divides by zero (a fault) after 256 home games in a season; the copy writes 0
        write_i32(season + r::AVERAGE_ATTENDANCE, if home != 0 { total.wrapping_div(home as i32) } else { 0 });
        write_u16(season + r::AVERAGE_ATTENDANCE_YEAR, 0);
    }
    attendance_record(this, club, entry, records, view, k::HIGHEST_ATTENDANCE, r::HIGHEST_ATTENDANCE, attendance, |v, rec| v > rec);
    attendance_record(this, club, entry, records, view, k::HIGHEST_GATE, r::HIGHEST_GATE, read_i32(result + match_record::GATE_RECEIPTS), |v, rec| v > rec);
    attendance_record(this, club, entry, records, view, k::LOWEST_ATTENDANCE, r::LOWEST_ATTENDANCE, attendance, |v, rec| v < rec);
    let goals = || (read_i8(view + club_match_record::GOALS_FOR) as i32, read_i8(view + club_match_record::GOALS_AGAINST) as i32);
    let applies = |kind: u8| exe::club_records_record_type_applies(kind as u32, club, 1) != 0;
    let league = || exe::comp_util_is_league_division(read_i32(f.addr())) != 0;
    let (gf, ga) = goals();
    if applies(k::BIGGEST_WIN) && gf > ga {
        result_record(this, club, records, view, &[k::BIGGEST_WIN], r::BIGGEST_WIN, match_record_is_bigger_win, gf - ga >= 6);
    }
    if applies(k::BIGGEST_LEAGUE_WIN) && league() && gf > ga {
        covered!("gra_15 league win");
        result_record(this, club, records, view, &[k::BIGGEST_WIN, k::BIGGEST_LEAGUE_WIN], r::BIGGEST_LEAGUE_WIN, match_record_is_bigger_win, gf - ga >= 6);
    }
    if applies(k::BIGGEST_DEFEAT) && gf < ga {
        result_record(this, club, records, view, &[k::BIGGEST_DEFEAT], r::BIGGEST_DEFEAT, match_record_is_bigger_defeat, ga - gf >= 6);
    }
    if applies(k::HIGHEST_SCORING) && ga + gf > 0 {
        result_record(this, club, records, view, &[k::HIGHEST_SCORING], r::HIGHEST_SCORING, match_record_has_more_goals, ga + gf >= 8);
    }
    if applies(k::HIGHEST_SCORING_LEAGUE) && league() && ga + gf > 0 {
        result_record(this, club, records, view, &[k::HIGHEST_SCORING, k::HIGHEST_SCORING_LEAGUE], r::HIGHEST_SCORING_LEAGUE, match_record_has_more_goals, ga + gf >= 8);
    }
    use club_entry_runs as e;
    if gf > ga {
        covered!("gra_15 won");
        run_record(this, club, entry, records, k::WON_IN_ROW, e::WON_RUN, r::WON_IN_ROW, 6);
        write_i32(entry + e::LOST_RUN, 0);
        write_i32(entry + e::WINLESS_RUN, 0);
    } else if gf < ga {
        run_record(this, club, entry, records, k::LOST_IN_ROW, e::LOST_RUN, r::LOST_IN_ROW, 6);
        write_i32(entry + e::WON_RUN, 0);
        write_i32(entry + e::UNBEATEN_RUN, 0);
    }
    if gf >= ga {
        run_record(this, club, entry, records, k::UNBEATEN, e::UNBEATEN_RUN, r::UNBEATEN, 12);
        write_i32(entry + e::LOST_RUN, 0);
    }
    if gf <= ga {
        run_record(this, club, entry, records, k::WINLESS, e::WINLESS_RUN, r::WINLESS, 12);
        write_i32(entry + e::WON_RUN, 0);
    }
}

/// The player's records of the match (one lineup row of `update_records_for_match`); returns the points that decide
/// the liking for him (`favoured_by_points`), or `None` when it stops before them.
///
/// Most capped player (type 0x1b, all-time only): at a national side (`exe::database_is_national_team`) that is not a
/// second national side (`exe::club_is_national_team`), in a match with the home side mark (`fixture::HOME_SIDE_MARK`),
/// the person's caps (`staff::INT_APPS`) above the record (`club_record::INTERNATIONAL_CAPS`, −1 none): the news when
/// he is not the holder, and the record takes the caps and the holder. A friendly stops here. The player's statistics
/// record of the match category (5 at a national side, 6 otherwise, `exe::playerstats_get_category_record`; missing:
/// the "No player stats" box, as in the original) give the season counts (`exe::player_stats_get_record_value`, weight
/// 6.5). The points: a rating (`lineup_row::RATING`) above 7 gives (rating − 6)^4; with goals (`lineup_row::GOALS`):
/// (goals + 1)^4, the top scorer (0x10, the season's goals; news with 3+ seasons, more than 3 goals and a new holder or
/// season: +100), in a league division the top league scorer (0x11, `exe::playerstats_get_stat` category 2; news with 3+
/// seasons and a new holder or season: +100, but nothing when the top scorer's news is queued) and the most goals in a
/// match (0x12, the match's goals and the match; news from 4 goals: +50); with assists: (assists + 1)^3 and the most
/// assists (0x13; news with 3+ seasons and a new holder or season: +50); the man of the match
/// (`match_record::MAN_OF_THE_MATCH_ID`): +30 and the most man of the match awards (0x15; news when the season count is
/// above 3, with 3+ seasons and a new holder or season: +50); a card in the match: the worst discipline (0x16, red cards
/// × 3 + yellow cards; news with 3+ seasons, 2+ yellows, 2+ reds and a new holder or season); in a league division the
/// league appearances and goals (`league_tally`); the youngest and oldest players (`player_age_records`).
pub(crate) unsafe fn update_player_match_records(c: PlayerRecordsCtx, f: Fixture, result: usize, row: usize, pdata: usize) -> Option<i32> {
    use club_match_records as r;
    use club_record_kind as k;
    let (club, person) = (c.club, c.person);
    if exe::database_is_national_team(club) != 0 && exe::club_is_national_team(club) == 0 && f.home_side_mark() != 0 && c.applies(k::INTERNATIONAL_CAPS) {
        let caps = person.int_apps() as i16;
        let rec = read_i16(c.all_time + club_record::INTERNATIONAL_CAPS);
        if rec == -1 || caps > rec {
            covered!("gra_15 most capped");
            if read_u32(c.all_time + club_record::INTERNATIONAL_CAPS + 2 + record_holder::STAFF_ID) != read_u32(person.addr()) {
                queue_match_record_news(c.this, club, &[k::INTERNATIONAL_CAPS]);
            }
            write_i16(c.all_time + club_record::INTERNATIONAL_CAPS, caps);
            exe::fill_staff_name_ref(person, c.all_time + club_record::INTERNATIONAL_CAPS + 2);
        }
    }
    if exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), f.competition()) != 0 {
        return None;
    }
    let stats = exe::playerstats_get_category_record(read_usize(G_PLAYER_STATS), pdata, if exe::database_is_national_team(club) != 0 { 5 } else { 6 });
    if stats == 0 {
        let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
        let comp = f.competition().addr();
        sprintf(0xde1f64, 0x98b6ec, club.addr() + 4, if comp == 0 { 0x98b714 } else { comp + 4 }); // "No player stats for %s playing in %s"
        show_error_at(0x98b6a0, 0x31a);
    }
    let count = |which: u32| ftol(exe::player_stats_get_record_value(stats, which, 6.5)) as u8;
    let league = || exe::comp_util_is_league_division(read_i32(f.addr())) != 0;
    let mut points = 0i32;
    covered!("gra_15 player points");
    let rating = read_u8(row + lineup_row::RATING);
    if rating > 7 {
        points = ftol(game_pow((rating as i32 - 6) as f64, read_f64(rdata::F64_4)));
    }
    let goals = read_u8(row + lineup_row::GOALS);
    if goals != 0 {
        points = points.wrapping_add(ftol(game_pow((goals as i32 + 1) as f64, read_f64(rdata::F64_4))));
        let scored = count(2);
        c.tally(k::TOP_SCORER, r::TOP_SCORER, scored, &mut points, || {
            if c.seasoned() && scored > 3 && c.new_holder(r::TOP_SCORER) {
                covered!("gra_15 top scorer ever");
                queue_match_record_news(c.this, club, &[k::TOP_SCORER]);
                100
            } else {
                0
            }
        });
        if league() {
            let scored = ftol(exe::playerstats_get_stat(read_usize(G_PLAYER_STATS), pdata, 2, 2, 6.5)) as u8;
            c.tally(k::TOP_LEAGUE_SCORER, r::TOP_LEAGUE_SCORER, scored, &mut points, || {
                if c.seasoned() && c.new_holder(r::TOP_LEAGUE_SCORER) {
                    match queue_match_record_news(c.this, club, &[k::TOP_SCORER, k::TOP_LEAGUE_SCORER]) {
                        Some(k::TOP_SCORER) => 0,
                        _ => 100,
                    }
                } else {
                    0
                }
            });
        }
        if c.applies(k::GOALS_IN_MATCH) && PlayerRecordsCtx::byte_beats(c.season + r::GOALS_IN_MATCH, goals) {
            write_u8(c.season + r::GOALS_IN_MATCH, goals);
            exe::fill_staff_name_ref(person, c.season + r::GOALS_IN_MATCH + r::HOLDER);
            match_record_assign(c.season + r::GOALS_IN_MATCH + 0x19, c.view);
            if PlayerRecordsCtx::byte_beats(c.all_time + r::GOALS_IN_MATCH, goals) {
                if goals >= 4 {
                    covered!("gra_15 goals in a match ever");
                    queue_match_record_news(c.this, club, &[k::GOALS_IN_MATCH]);
                    points = points.wrapping_add(50);
                }
                write_u8(c.all_time + r::GOALS_IN_MATCH, goals);
                exe::fill_staff_name_ref(person, c.all_time + r::GOALS_IN_MATCH + r::HOLDER);
                match_record_assign(c.all_time + r::GOALS_IN_MATCH + 0x19, c.view);
            }
        }
    }
    let assists = read_u8(row + lineup_row::ASSISTS);
    if assists != 0 {
        points = points.wrapping_add(ftol(game_pow((assists as i32 + 1) as f64, read_f64(rdata::F64_3))));
        let made = count(5);
        c.tally(k::ASSISTS, r::ASSISTS, made, &mut points, || {
            if c.seasoned() && c.new_holder(r::ASSISTS) {
                queue_match_record_news(c.this, club, &[k::ASSISTS]);
                50
            } else {
                0
            }
        });
    }
    if read_u32(row + lineup_row::PLAYER_ID) == read_u32(result + match_record::MAN_OF_THE_MATCH_ID) {
        covered!("gra_15 man of the match");
        let awards = count(0xc);
        points = points.wrapping_add(30);
        c.tally(k::MAN_OF_MATCH, r::MAN_OF_MATCH, awards, &mut points, || {
            if read_i8(c.season + r::MAN_OF_MATCH) > 3 && c.seasoned() && c.new_holder(r::MAN_OF_MATCH) {
                queue_match_record_news(c.this, club, &[k::MAN_OF_MATCH]);
                50
            } else {
                0
            }
        });
    }
    if read_u8(row + lineup_row::YELLOW_CARDS) != 0 || read_u8(row + lineup_row::RED_CARD) != 0 {
        let (yellow, red) = (count(10), count(11));
        let score = red as i32 * 3 + yellow as i32;
        let beats = |rec: usize| read_i8(rec) == -1 || score > read_i8(rec + 1) as i32 * 3 + read_i8(rec) as i32;
        let at = r::DISCIPLINE;
        if c.applies(k::DISCIPLINE) && beats(c.season + at) {
            covered!("gra_15 discipline");
            write_u8(c.season + at, yellow);
            write_u8(c.season + at + 1, red);
            exe::fill_staff_name_ref(person, c.season + at + 2);
            write_u16(c.season + at + 0x1a, 0);
            if beats(c.all_time + at) {
                let other = read_u32(c.all_time + at + 2 + record_holder::STAFF_ID) != read_u32(person.addr()) || read_u16(c.all_time + at + 0x1a) != c.year;
                if c.seasoned() && yellow >= 2 && red >= 2 && other {
                    queue_match_record_news(c.this, club, &[k::DISCIPLINE]);
                }
                write_u8(c.all_time + at, yellow);
                write_u8(c.all_time + at + 1, red);
                exe::fill_staff_name_ref(person, c.all_time + at + 2);
                write_u16(c.all_time + at + 0x1a, c.year);
            }
        }
    }
    if league() {
        league_tally(c, k::LEAGUE_APPEARANCES, club_record::LEAGUE_APPEARANCES, 1, pdata, &mut points);
        if read_u8(row + lineup_row::GOALS) != 0 {
            league_tally(c, k::LEAGUE_GOALS, club_record::LEAGUE_GOALS, 2, pdata, &mut points);
        }
    }
    player_age_records(c, &mut points);
    Some(points)
}

/// The liking for a player who did well (the end of each lineup row of `update_records_for_match`).
///
/// The points (as a 16-bit value) must be 40 or more, the person in the staff table, the match's reputation `rep` 5 or
/// more and the person without a future transfer (`exe::transfer_manager_get_future_transfer`, when there is a transfer
/// manager). Against a rival (`club::RIVAL1`..3) the points × (rep × 0.08 + 1); in a clash (`clash`,
/// `exe::comp_util_get_league_clash_type` above 0) × (rep × 0.08 + 1) again; otherwise, not against a rival, when rep is
/// below the club's reputation / 500, × rep × 500 / reputation, kept within 0.2 and 1. Then three draws against
/// `utils_random(4000) + 250`: the chairman (`club::CHAIRMAN`) and the manager (`club::MANAGER`) favour the player
/// (`exe::staff_add_favoured_staff`, forced) and the club does (`exe::club_add_favoured_staff`); and each squad member
/// (`club::SQUAD`, 50) who is a player (`staff::PLAYER`) and not him, against `utils_random(7000) + 250`.
pub(crate) unsafe fn favoured_by_points(club: Club, person: Staff, points: i32, rep: i16, rival: bool, clash: bool) {
    let mut p = points;
    if (p as i16) < 40 {
        return;
    }
    if !person.is_null() && read_i32(person.addr()) >= read_i32(G_STAFF_COUNT) {
        return;
    }
    if rep < 5 {
        return;
    }
    let tm = read_usize(G_TRANSFER_MANAGER);
    if tm != 0 && exe::transfer_manager_get_future_transfer(tm, person, 0) != 0 {
        return;
    }
    covered!("gra_15 liking draws");
    let boost = |p: i32| ftol((rep as f64 * read_f64(rdata::F64_0_08) + read_f64(rdata::F64_1)) * (p as i16) as f64);
    if p as i16 != 0 {
        if rival {
            covered!("gra_15 against a rival");
            p = boost(p);
        }
        if clash {
            p = boost(p);
        } else if !rival {
            let reputation = club.reputation() as i32;
            if (rep as i32) < reputation / 500 {
                covered!("gra_15 small match");
                let share = ((rep as i32 * 500) / reputation) as f64;
                let share = if share < read_f64(rdata::F64_0_2) {
                    read_f64(rdata::F64_0_2)
                } else if share > read_f64(rdata::F64_1) {
                    read_f64(rdata::F64_1)
                } else {
                    share
                };
                p = ftol((p as i16) as f64 * share);
            }
        }
    }
    let s = (p as i16) as i32;
    if s > random(4000) + 250 {
        exe::staff_add_favoured_staff(club.chairman().addr(), person, 1);
    }
    if s > random(4000) + 250 {
        exe::staff_add_favoured_staff(club.manager().addr(), person, 1);
    }
    if s > random(4000) + 250 {
        covered!("gra_15 club favours");
        exe::club_add_favoured_staff(club, person, 1);
    }
    for k in 0..50 {
        let m = Staff(club.squad(k));
        if !m.is_null() && m.player().addr() != 0 && m != person && s > random(7000) + 250 {
            exe::staff_add_favoured_staff(m, person, 1);
        }
    }
}

/// A sum of money as the game shows it (`Cash::format`, 0x450aa0: `exe::cash_get_display_amount` with `flags`, then
/// `exe::cash_format_money` into 200 bytes).
pub(crate) unsafe fn record_money_text(value: u32, flags: u32, buf: &mut [u8; 0xc8]) {
    let cash = [value];
    exe::cash_format_money(buf.as_mut_ptr() as usize, 0xc8, exe::cash_get_display_amount(cash.as_ptr(), flags), flags);
}

/// The club's link (0x7d5), under the given name.
pub(crate) unsafe fn link_record_club(out: usize, name: usize, club: Club) {
    exe::text_add_link(out, name as *const u8, 0x7d5, club); // gra18: the club's link
}

/// Reads the broken record of type `kind` from the news item's slots into the local record copy `r`: the inverse of
/// `club_records_write_record_news_fields` (`record_news_fields`): bytes, words and dwords stored at their offsets, the
/// rating (type 20) as the slot × 0.01 in a float, the match, holder and run records by their readers; types 25 and 26
/// read the day and the year of the date at +0x27b / +0x2ba into a Date (`exe::new_date`, then
/// `exe::date_operator_3d`). Returns false for a type outside 1..=32.
pub(crate) unsafe fn read_record_news_fields(news: usize, kind: u8, r: usize, k: &mut u8) -> bool {
    use RecordNewsField::*;
    let fields = record_news_fields(kind);
    if fields.is_empty() {
        return false;
    }
    let slot = |k: &mut u8| {
        let v = exe::newsitem_get_slot(news, *k as u32);
        *k = k.wrapping_add(1);
        v
    };
    let mut rest = fields;
    if kind == 25 || kind == 26 {
        covered!("gra_18 record date");
        let base = if kind == 25 { 0x27b } else { 0x2ba };
        let (day, year) = (slot(k), slot(k));
        let mut date = [0u8; 8];
        let mut scratch = [0u8; 8];
        exe::new_date(date.as_mut_ptr() as usize, day, year);
        exe::date_operator_3d(r + base, scratch.as_mut_ptr() as usize, date.as_ptr() as usize);
        rest = &fields[2..];
    }
    let run: unsafe extern "C" fn(usize, *mut u8, usize) -> u32 = core::mem::transmute(FN_READ_RUN_RECORD_FROM_NEWS);
    for f in rest {
        match *f {
            Byte(o) | SByte(o) => write_u8(r + o, slot(k) as u8),
            Word(o) | SWord(o) => write_u16(r + o, slot(k) as u16),
            Dword(o) => write_u32(r + o, slot(k)),
            Percent(o) => write_f32(r + o, (slot(k) as i32 as f64 * read_f64(rdata::F64_0_01)) as f32),
            Match(o) => {
                exe::read_match_record_from_news(news, k, (r + o) as *mut u8);
            }
            Holder(o) => {
                exe::read_staff_name_ref_from_news(news, k, (r + o) as *mut u8);
            }
            Run(o) => {
                run(news, k, r + o);
            }
        }
    }
    true
}

/// The fields of each record type (1..=32) in the season record, in the original's order.
pub(crate) fn record_news_fields(kind: u8) -> &'static [RecordNewsField] {
    use RecordNewsField::*;
    match kind {
        1 => &[Byte(0), Dword(1), Word(5)],
        2 => &[Byte(7), Dword(8), Word(0xc)],
        3 => &[Dword(0xe), Match(0x12)],
        4 => &[Dword(0x31), Match(0x35)],
        5 => &[Dword(0x54), Match(0x58)],
        6 => &[Dword(0x77), Word(0x7b)],
        7 => &[Match(0x7d)],
        8 => &[Match(0x9c)],
        9 => &[Match(0xbb)],
        10 => &[Match(0xda)],
        11 => &[Match(0xf9)],
        12 => &[Run(0x118)],
        13 => &[Run(0x12c)],
        14 => &[Run(0x140)],
        15 => &[Run(0x154)],
        16 => &[SByte(0x168), Holder(0x169), Word(0x181)],
        17 => &[SByte(0x183), Holder(0x184), Word(0x19c)],
        18 => &[SByte(0x19e), Holder(0x19f), Match(0x1b7)],
        19 => &[SByte(0x1d6), Holder(0x1d7), Word(0x1ef)],
        20 => &[Percent(0x1f1), Holder(0x1f5), SByte(0x20d), Word(0x20e)],
        21 => &[SByte(0x210), Holder(0x211), Word(0x229)],
        22 => &[SByte(0x22b), SByte(0x22c), Holder(0x22d), Word(0x245)],
        23 => &[SWord(0x247), Holder(0x249)],
        24 => &[SWord(0x261), Holder(0x263)],
        25 => &[SWord(0x27b), Word(0x27d), Holder(0x283), Match(0x29b)],
        26 => &[SWord(0x2ba), Word(0x2bc), Holder(0x2c2), Match(0x2da)],
        27 => &[SWord(0x2f9), Holder(0x2fb)],
        28 => &[Dword(0x313), Holder(0x317), Dword(0x32f), SByte(0x33b)],
        29 => &[Dword(0x33c), Holder(0x340), Dword(0x358), SByte(0x364)],
        30 => &[Dword(0x365)],
        31 => &[Dword(0x36b)],
        32 => &[Holder(0x371)],
        _ => &[],
    }
}

/// The attendance records of a home match (types 3, 4 and 5): the value `value` of the match beats the season record at
/// `at` (`beats(value, record)`, or no record: −1) → the season record takes the value and the match (`match_record_assign`
/// of the club's view of the match); if it also beats the all-time record: the news when the club has played 50 or more
/// home games (`club_entry_runs::HOME_GAMES_TOTAL`), and the all-time record takes the value and the match.
pub(crate) unsafe fn attendance_record(this: usize, club: Club, entry: usize, records: (usize, usize), view: usize, kind: u8, at: usize, value: i32, beats: fn(i32, i32) -> bool) {
    let (season, all_time) = records;
    if exe::club_records_record_type_applies(kind as u32, club, 1) == 0 || read_u8(view + club_match_record::VENUE) != 1 || value <= 0 {
        return;
    }
    let s = read_i32(season + at);
    if s != -1 && !beats(value, s) {
        return;
    }
    write_i32(season + at, value);
    match_record_assign(season + at + 4, view);
    let a = read_i32(all_time + at);
    if a != -1 && !beats(value, a) {
        return;
    }
    covered!("gra_15 attendance record ever");
    if read_i16(entry + club_entry_runs::HOME_GAMES_TOTAL) >= 50 {
        queue_match_record_news(this, club, &[kind]);
    }
    write_i32(all_time + at, value);
    match_record_assign(all_time + at + 4, view);
}

/// A match record (types 7 to 0xb): the club's view of the match beats the season record at `at` (`better(view,
/// record)`, or no record: fixture id −1) → it is copied there; if it also beats the all-time record: with `news`, the
/// news (skipped when one of `kinds` is queued), and it is copied to the all-time record.
pub(crate) unsafe fn result_record(this: usize, club: Club, records: (usize, usize), view: usize, kinds: &[u8], at: usize, better: unsafe extern "C" fn(usize, usize) -> u32, news: bool) {
    let (season, all_time) = records;
    if read_i32(season + at + club_match_record::FIXTURE_ID) != -1 && better(view, season + at) == 0 {
        return;
    }
    match_record_assign(season + at, view);
    if read_i32(all_time + at + club_match_record::FIXTURE_ID) != -1 && better(view, all_time + at) == 0 {
        return;
    }
    if news {
        queue_match_record_news(this, club, kinds);
    }
    match_record_assign(all_time + at, view);
}

/// The youngest and oldest player records (types 0x19 and 0x1a, `PlayerRecordsCtx::age_beats`). The all-time news:
/// with 3 or more seasons, a youngest player under 20 or an oldest over 35 (in years today) who is not the all-time
/// holder; an oldest player's news multiplies the points by 1.25 (the points as a 16-bit value, truncated).
pub(crate) unsafe fn player_age_records(c: PlayerRecordsCtx, points: &mut i32) {
    use club_match_records as r;
    use club_record_kind as k;
    for (kind, at, younger) in [(k::YOUNGEST, r::YOUNGEST, true), (k::OLDEST, r::OLDEST, false)] {
        if !c.applies(kind) || !c.age_beats(c.season + at, younger) {
            continue;
        }
        c.take_age_record(c.season + at);
        if !c.age_beats(c.all_time + at, younger) {
            continue;
        }
        if c.seasoned() {
            let (years, _) = c.age_today();
            let notable = if younger { years < 20 } else { years > 35 };
            if notable && read_u32(c.all_time + at + r::AGE_HOLDER + record_holder::STAFF_ID) != read_u32(c.person.addr()) {
                queue_match_record_news(c.this, c.club, &[kind]);
                if !younger {
                    covered!("gra_15 oldest player ever");
                    *points = ftol((*points as i16) as f64 * read_f64(rdata::F64_1_25));
                }
            }
        }
        c.take_age_record(c.all_time + at);
    }
}

/// The 16-bit league tallies (types 0x17 appearances and 0x18 goals, `club_record::LEAGUE_APPEARANCES` /
/// `club_record::LEAGUE_GOALS`, the holder after the value): the player's league count at the club
/// (`get_club_league_stat`, field 1 or 2) beats the season record (−1 none, signed) → the season record takes it and
/// the holder; beating the all-time one too: with 3 or more seasons and another holder, the news and 100 points; the
/// all-time record takes the count and the holder.
pub(crate) unsafe fn league_tally(c: PlayerRecordsCtx, kind: u8, at: usize, field: i8, pdata: usize, points: &mut i32) {
    let value = ftol(get_club_league_stat(read_usize(G_PLAYER_STATS), pdata, field) as f64) as i16;
    if !c.applies(kind) {
        return;
    }
    let beats = |rec: usize| read_i16(rec) == -1 || value > read_i16(rec);
    if !beats(c.season + at) {
        return;
    }
    write_i16(c.season + at, value);
    exe::fill_staff_name_ref(c.person, c.season + at + 2);
    if !beats(c.all_time + at) {
        return;
    }
    if c.seasoned() && read_u32(c.all_time + at + 2 + record_holder::STAFF_ID) != read_u32(c.person.addr()) {
        covered!("gra_15 league tally ever");
        queue_match_record_news(c.this, c.club, &[kind]);
        *points = points.wrapping_add(100);
    }
    write_i16(c.all_time + at, value);
    exe::fill_staff_name_ref(c.person, c.all_time + at + 2);
}

/// The comparer's dry run of `update_records_for_match`: `Some((n, queued))` makes the copy look only at the first `n`
/// items of the real news queue and put its own in `queued` instead of the queue.
pub(crate) static MATCH_RECORDS_NEWS_DRY_RUN: std::sync::Mutex<Option<(i32, Vec<(u32, u8)>)>> = std::sync::Mutex::new(None);

/// What a player record needs: the records object, the club, its entry, the season and all-time records, the person,
/// the season year (`exe::key_nation_get_club_season_year`) and the club's view of the match.
#[derive(Clone, Copy)]
pub(crate) struct PlayerRecordsCtx {
    pub(crate) this: usize,
    pub(crate) club: Club,
    pub(crate) entry: usize,
    pub(crate) season: usize,
    pub(crate) all_time: usize,
    pub(crate) person: Staff,
    pub(crate) year: u16,
    pub(crate) view: usize,
}

impl PlayerRecordsCtx {
    pub(crate) unsafe fn applies(self, kind: u8) -> bool {
        exe::club_records_record_type_applies(kind as u32, self.club, 1) != 0
    }

    /// The club has 3 or more seasons recorded (`club_record_entry::SEASONS`).
    pub(crate) unsafe fn seasoned(self) -> bool {
        read_i16(self.entry + club_record_entry::SEASONS) >= 3
    }

    /// The all-time holder of the record at `at` is someone else or held it in another season.
    pub(crate) unsafe fn new_holder(self, at: usize) -> bool {
        use club_match_records as r;
        read_u32(self.all_time + at + r::HOLDER + record_holder::STAFF_ID) != read_u32(self.person.addr()) || read_u16(self.all_time + at + r::YEAR) != self.year
    }

    /// A one-byte season tally (`at`, −1 = none) beaten by `value` (signed comparison).
    pub(crate) unsafe fn byte_beats(rec: usize, value: u8) -> bool {
        read_i8(rec) == -1 || value as i32 > read_i8(rec) as i32
    }

    /// The byte tally record at `at` of type `kind` with `value`: the season record takes it, the holder and year 0;
    /// beating the all-time one too: `news()` decides the news (the extra points it returns are added to `points`) and
    /// the all-time record takes the value, the holder and the season year.
    pub(crate) unsafe fn tally(self, kind: u8, at: usize, value: u8, points: &mut i32, news: impl FnOnce() -> i32) {
        use club_match_records as r;
        if !self.applies(kind) || !Self::byte_beats(self.season + at, value) {
            return;
        }
        write_u8(self.season + at, value);
        exe::fill_staff_name_ref(self.person, self.season + at + r::HOLDER);
        write_u16(self.season + at + r::YEAR, 0);
        if !Self::byte_beats(self.all_time + at, value) {
            return;
        }
        *points = points.wrapping_add(news());
        write_u8(self.all_time + at, value);
        exe::fill_staff_name_ref(self.person, self.all_time + at + r::HOLDER);
        write_u16(self.all_time + at + r::YEAR, self.year);
    }

    /// The age (`exe::date_get_difference`: years word +2, days word +0) at `date` of someone born on `birth`.
    pub(crate) unsafe fn age(date: usize, birth: usize) -> (u16, i16) {
        let mut out = [0u8; 8];
        let p = exe::date_get_difference(date, out.as_mut_ptr(), read_u32(birth), read_u32(birth + 4));
        (read_u16(p + 2), read_i16(p))
    }

    /// The person's age today ([`G_CURRENT_DATE`]).
    pub(crate) unsafe fn age_today(self) -> (u16, i16) {
        Self::age(G_CURRENT_DATE, self.person.addr() + staff::DATE_OF_BIRTH)
    }

    /// The age record at `at` (youngest: `younger`, oldest: not) beaten by the person: the record's holder at its
    /// match is older (younger) in years, or in days within the same year; no record also beats it.
    pub(crate) unsafe fn age_beats(self, at: usize, younger: bool) -> bool {
        use club_match_records as r;
        if read_i32(at + r::AGE_MATCH + club_match_record::FIXTURE_ID) == -1 {
            return true;
        }
        let (ry, rd) = Self::age(at + r::AGE_MATCH, at);
        let (cy, cd) = self.age_today();
        if younger {
            if ry != cy { ry > cy } else { rd > cd }
        } else if ry != cy {
            ry < cy
        } else {
            rd < cd
        }
    }

    /// The age record: the birth date, the holder and the match, in the record at `at`.
    pub(crate) unsafe fn take_age_record(self, at: usize) {
        use club_match_records as r;
        copy_record_date(at, self.person.addr() + staff::DATE_OF_BIRTH);
        exe::fill_staff_name_ref(self.person, at + r::AGE_HOLDER);
        match_record_assign(at + r::AGE_MATCH, self.view);
    }
}

/// The constructors the original runs over its local copy of the broken record (0x38b bytes, the `club_record`
/// layout), in its order: the match records (0x645800: only their Date), the four run records, the tallies and the
/// counts, the goals-in-a-match and rating records, the youngest and oldest player records, the two transfer fee
/// records and the fans' player of the year. All thiscall, returning `this`.
pub(crate) const RECORD_NEWS_CONSTRUCTORS: [(usize, usize); 27] = [
    (0x12, 0x645800), (0x35, 0x645800), (0x58, 0x645800), (0x7d, 0x645800), (0x9c, 0x645800), (0xbb, 0x645800),
    (0xda, 0x645800), (0xf9, 0x645800),
    (0x118, 0x450ad0), (0x12c, 0x450b20), (0x140, 0x450b70), (0x154, 0x450bc0),
    (0x168, 0x450c60), (0x183, 0x450c60), (0x19e, 0x450c10), (0x1d6, 0x450c60), (0x1f1, 0x453c50), (0x210, 0x450c60),
    (0x22b, 0x450d30), (0x247, 0x450d30), (0x261, 0x450d30), (0x27b, 0x450c70), (0x2ba, 0x450cd0), (0x2f9, 0x450d30),
    (0x313, 0x4485d0), (0x33c, 0x448620), (0x371, 0x450d40),
];

/// `read_run_record_from_news(item, &slot, out)` (0x7e5570, record_utils.cpp; cdecl): the inverse of 0x7e5420.
pub(crate) const FN_READ_RUN_RECORD_FROM_NEWS: usize = 0x7e5570;

/// `database_format_years_between_dates(end, start, text, size)` (0x540340, Database.cpp; cdecl).
pub(crate) const FN_DATABASE_FORMAT_YEARS_BETWEEN_DATES: usize = 0x540340;

/// The empty string the original sets the Text to first (0xad9c64).
pub(crate) const EMPTY_STRING: usize = 0xad9c64;

/// A match record inside the local record copy (`read_match_record_from_news`, 0x1f bytes): the date (+0 day, +2
/// year), the club competition (+8), the round and stage words (+0xc, +0xe), the venue (+0x10: 1 at home, 2 away),
/// the club (+0x11) and its opponent (+0x15), their goals (+0x19, +0x1a) and the fixture index (+0x1b).
#[derive(Clone, Copy)]
pub(crate) struct RecordMatch(pub(crate) usize);

impl RecordMatch {
    pub(crate) unsafe fn comp_index(self) -> i32 {
        read_i32(self.0 + 8)
    }
    pub(crate) unsafe fn comp(self) -> usize {
        read_usize(G_CLUB_COMP_TABLE).wrapping_add((self.comp_index() as usize).wrapping_mul(0x6b))
    }
    pub(crate) unsafe fn venue(self) -> u8 {
        read_u8(self.0 + 0x10)
    }
    pub(crate) unsafe fn opponent(self) -> Club {
        club_at(read_i32(self.0 + 0x15) as isize)
    }
    pub(crate) unsafe fn goals(self) -> (i8, i8) {
        (read_i8(self.0 + 0x19), read_i8(self.0 + 0x1a))
    }
    /// The round label (`exe::comp_util_format_comp_stage_round_label`, form 2, 200 bytes) and its gender
    /// (`exe::comp_util_get_label_gender`).
    pub(crate) unsafe fn round_label(self, label: &mut [u8; 0xc8]) -> i32 {
        let (round, stage) = (read_u32(self.0 + 0xc), read_u32(self.0 + 0xe));
        exe::comp_util_format_comp_stage_round_label(self.comp(), round, stage, 2, label.as_mut_ptr(), 0xc8);
        exe::comp_util_get_label_gender(self.comp() as u32, round, stage, 2) as i8 as i32
    }
    /// The score's link (0x7d3) when the fixture still matches (`exe::match_stats_fixture_entry_matches`).
    pub(crate) unsafe fn link_score(self, out: usize, score: &[u8]) {
        let (day, year) = (read_u16(self.0) as i32, read_u16(self.0 + 2) as i32);
        let fixture = read_i32(self.0 + 0x1b);
        if exe::match_stats_fixture_entry_matches(read_usize(G_MATCH_STATS), fixture, read_i32(self.0 + 0x11), read_i32(self.0 + 0x15), day, year) != 0 {
            exe::text_add_link(out, score.as_ptr(), 0x7d3, fixture as u32 as usize);
        }
    }
}

/// One field of a broken record as the news item gets it.
#[derive(Clone, Copy)]
pub(crate) enum RecordNewsField {
    /// a byte, zero-extended
    Byte(usize),
    /// a signed byte
    SByte(usize),
    /// a word, zero-extended
    Word(usize),
    /// a signed word
    SWord(usize),
    /// a dword
    Dword(usize),
    /// a float × 100, truncated (`ftol`)
    Percent(usize),
    /// a match record (`exe::write_match_record_to_news`)
    Match(usize),
    /// a record holder's name reference (`exe::write_staff_name_ref_to_news`)
    Holder(usize),
    /// a run record (0x7e5420)
    Run(usize),
}
