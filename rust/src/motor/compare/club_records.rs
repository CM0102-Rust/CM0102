use super::*;

// The comparers of the routines in ../club_records.rs.
// [pt] Os comparadores das rotinas de ../club_records.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `update_league_position_records`.
///
/// The original routine runs first (it sorts the tables, which `sort_table` checks on its own) and the copy afterwards, as
/// a dry run (without sorting), from the same records: the first 14 bytes of each chain club's season and all-time
/// records, the queue header (+0x24..+0x30) and the queue; and the return value.
///
/// A null league or one with +0x44 ≠ 0xff, a record to be created (`get_season_record`/`get_all_time_record` allocate) or a queue that may
/// grow: only the original.
///
/// [pt] Comparador: a original roda antes (ordena as tabelas, que a 0x6827d0 confere por conta própria) e a cópia depois,
/// em ensaio (sem ordenar), a partir dos mesmos recordes: os 14 primeiros bytes dos registros da temporada e de sempre
/// de cada clube da cadeia, o cabeçalho da fila (+0x24..+0x30) e a fila; e a devolução. Liga nula ou com +0x44 ≠ 0xff,
/// registro a criar (0x4528d0/0x452710 alocam) ou fila que pode crescer: só a original.
pub(crate) unsafe extern "thiscall" fn compare_update_league_position_records(this: usize, liga0: usize, always: u32) -> u32 {
    let liga0 = League(liga0);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_UPDATE_LEAGUE_POSITION_RECORDS);
    if liga0.is_null() || liga0.stage_index() != 0xff {
        return o(this, liga0.addr(), always); // simplification: the boxes, original only
    }
    let mut regions = vec![(this + club_records_fields::NEWS_QUEUE, 0xc)];
    let mut podem_entrar = 0i32;
    for liga in league_stage_chain(liga0) {
        for i in 0..read_i16(liga + league::CLUB_COUNT).max(0) as usize {
            let row = read_usize(liga + league::TABLE) + 0x3b * i;
            let club = read_usize(row);
            if read_u8(row + league_table_row::PLAYED) == 0 || club == 0 {
                continue;
            }
            let (t, s) = (club_season_record(this, club, 0), club_all_time_record(this, club, 0));
            if t == 0 || s == 0 {
                return o(this, liga0.addr(), always); // simplification: the record would be created, original only
            }
            regions.push((t, 0xe));
            regions.push((s, 0xe));
            if exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0 || exe::database_is_national_team(club) != 0 {
                podem_entrar += 2;
            }
        }
    }
    let queue = read_usize(this + club_records_fields::NEWS_QUEUE);
    if (read_usize(this + club_records_fields::NEWS_QUEUE_COUNT) as i32).wrapping_add(podem_entrar) > read_usize(this + club_records_fields::NEWS_QUEUE_CAPACITY) as i32 || (queue == 0 && podem_entrar > 0) {
        return o(this, liga0.addr(), always); // simplification: the queue may grow (reallocates), original only
    }
    if queue != 0 {
        regions.push((queue, 6 * read_usize(this + club_records_fields::NEWS_QUEUE_CAPACITY)));
    }
    let before = crate::comparison::MemorySnapshot::read(&regions);
    let ro = o(this, liga0.addr(), always);
    let mo = crate::comparison::MemorySnapshot::read(&regions);
    before.write_back();
    POSITION_RECORDS_DRY_RUN = true;
    let rr = crate::rng::pause_trace(|| update_league_position_records(this, liga0.addr(), always));
    POSITION_RECORDS_DRY_RUN = false;
    let mr = crate::comparison::MemorySnapshot::read(&regions);
    mo.write_back();
    SCOREBOARD.record(ro == rr && mo == mr, || format!("update_league_position_records({:x}, {:x}, {}) original={} {:x?} rust={} {:x?}", this, liga0.addr(), always, ro, mo, rr, mr));
    ro
}

/// Comparer for `send_record_news`.
///
/// [pt] Comparador: a cópia em ensaio (sem preencher nem entregar, sem soltar) roda antes, enquanto a fila está viva, e a
/// original depois a partir do mesmo cabeçalho (+0x24..+0x30): o cabeçalho e a devolução.
pub(crate) unsafe extern "thiscall" fn compare_send_record_news(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SEND_RECORD_NEWS);
    let regions = [(this + club_records_fields::NEWS_QUEUE, 0xc)];
    let before = crate::comparison::MemorySnapshot::read(&regions);
    RECORD_NEWS_DRY_RUN = true;
    let rr = crate::rng::pause_trace(|| send_record_news(this));
    RECORD_NEWS_DRY_RUN = false;
    let mr = crate::comparison::MemorySnapshot::read(&regions);
    before.write_back();
    let ro = o(this);
    let mo = crate::comparison::MemorySnapshot::read(&regions);
    SCOREBOARD.record(ro == rr && mo == mr, || format!("send_record_news({:x}) original={} {:x?} rust={} {:x?}", this, ro, mo, rr, mr));
    ro
}

/// Comparer for `pick_fans_player_of_year`.
///
/// [pt] Comparador: lado a lado pela referência do escolhido (registro +0x371, 0x18 bytes) e pela devolução.
pub(crate) unsafe extern "thiscall" fn compare_pick_fans_player_of_year(this: usize, club: usize, record: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_PICK_FANS_PLAYER_OF_YEAR);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(record + club_record::FANS_PLAYER, 0x18)], || o(this, club, record), || pick_fans_player_of_year(this, club, record));
    SCOREBOARD.record(igual && r == n, || format!("pick_fans_player_of_year(clube {:x}) original={} {:x?} rust={} {:x?}", read_usize(club), r, mo, n, mr));
    r
}

/// Comparer for `update_transfer_fee_records`.
///
/// The original routine puts items in the news queue (it reallocates) and the liking draws at random, so: the plan
/// first (the four records created in the same order as the original, and the expected paid and received records); the
/// original runs; it fetches the records again (the vector may have been reallocated) and checks the values and the
/// club of each record and [0xae1080].
///
/// [pt] Comparador: a original põe na fila de notícias (realoca) e o gostar sorteia: o plano antes (os quatro registros
/// criados na mesma ordem da original, e os recordes pago e recebido esperados); a original roda; pega os registros
/// de novo (o vetor pode ter sido realocado) e confere os valores e o clube de cada recorde e o [0xae1080].
pub(crate) unsafe extern "thiscall" fn compare_update_transfer_fee_records(this: usize, t: usize) -> u32 {
    let t = Offer(t);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_UPDATE_TRANSFER_FEE_RECORDS);
    let person = read_usize(G_STAFF_TABLE).wrapping_add(((t.target_staff_id() as i32) as usize).wrapping_mul(staff::SIZE));
    let buyer = injury_club_by_index((t.buyer_club_id() as i32));
    let contract = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person));
    let seller = if !contract.is_null() { injury_club_by_index((contract.club() as i32)) } else { 0 };
    if seller == 0 {
        let r = o(this, t.addr());
        SCOREBOARD.record(r == 1 && read_usize(G_CONTRACT_LOOKUP_RESULT) == 0, || format!("update_transfer_fee_records sem vendedor: {} [G_CONTRACT_LOOKUP_RESULT]={:x}", r, read_usize(G_CONTRACT_LOOKUP_RESULT)));
        return r;
    }
    let records = || [exe::club_records_get_season_record(this, buyer, 1), exe::club_records_get_all_time_record(this, buyer, 1), exe::club_records_get_season_record(this, seller, 1), exe::club_records_get_all_time_record(this, seller, 1)];
    let value = (t.fee() as i32);
    let rg = records();
    // (value, club) of each record: paid in the season and all-time by the buyer, received by the seller
    let mut plan = [(read_i32(rg[0] + 0x313), read_usize(rg[0] + 0x32f)), (read_i32(rg[1] + 0x313), read_usize(rg[1] + 0x32f)),
                     (read_i32(rg[2] + 0x33c), read_usize(rg[2] + 0x358)), (read_i32(rg[3] + 0x33c), read_usize(rg[3] + 0x358))];
    let mut queue = read_i32(this + club_records_fields::NEWS_QUEUE_COUNT);
    if value != 0 {
        if value > plan[0].0 {
            plan[0] = (value, read_usize(seller));
            if value > plan[1].0 {
                plan[1] = plan[0];
                if is_big_transfer(value, Club(buyer)) && !record_news_queued(this, read_usize(buyer), 0x1c) {
                    queue += 1;
                }
            }
        }
        if value > plan[2].0 {
            plan[2] = (value, read_usize(buyer));
            if value > plan[3].0 {
                plan[3] = plan[2];
                if is_big_transfer(value, Club(seller)) && !record_news_queued(this, read_usize(seller), 0x1d) {
                    queue += 1;
                }
            }
        }
    }
    let r = o(this, t.addr());
    let rg = records();
    let now = [(read_i32(rg[0] + 0x313), read_usize(rg[0] + 0x32f)), (read_i32(rg[1] + 0x313), read_usize(rg[1] + 0x32f)),
                 (read_i32(rg[2] + 0x33c), read_usize(rg[2] + 0x358)), (read_i32(rg[3] + 0x33c), read_usize(rg[3] + 0x358))];
    let queue_now = read_i32(this + club_records_fields::NEWS_QUEUE_COUNT);
    SCOREBOARD.record(r == 1 && now == plan && queue_now == queue && read_usize(G_CONTRACT_LOOKUP_RESULT) == contract.addr(), || {
        format!("update_transfer_fee_records(valor {}) plano {:x?} fila {} x {:x?} fila {}", value, plan, queue, now, queue_now)
    });
    r
}

/// Comparer for `add_transfer_totals`.
///
/// The records may be created (realloc) and the news item reallocates: the four records are requested beforehand, in
/// the original's order (seller: season, all-time; buyer: season, all-time; the second time only finds them), and if
/// the news queue has no room for two, only the original runs. Otherwise side by side over the four records (+0x365, 12
/// bytes), the news count and the next two slots.
///
/// [pt] Comparador: os registros podem ser criados (realloc) e a notícia realoca: os quatro registros são pedidos antes, na
/// ordem da original (vendedor: temporada, sempre; comprador: temporada, sempre; a segunda vez só acha), e com a fila
/// de notícias sem folga para duas, só a original. Lado a lado pelos quatro registros (+0x365, 12 bytes), a conta das
/// notícias e as duas vagas seguintes.
pub(crate) unsafe extern "thiscall" fn compare_add_transfer_totals(this: usize, value: i32, buyer: usize, seller: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, usize, usize) -> u32>(IDX_ADD_TRANSFER_TOTALS);
    if buyer == 0 || seller == 0 || (read_usize(this + club_records_fields::NEWS_QUEUE_CAPACITY).wrapping_sub(read_usize(this + club_records_fields::NEWS_QUEUE_COUNT)) as i32) < 2 {
        return o(this, value, buyer, seller); // simplification: with queue realloc, original only
    }
    let _ = [exe::club_records_get_season_record(this, seller, 1), exe::club_records_get_all_time_record(this, seller, 1), exe::club_records_get_season_record(this, buyer, 1), exe::club_records_get_all_time_record(this, buyer, 1)];
    // creating the buyer’s record may reallocate the vector and move the seller’s: fetch all four again (now it only finds)
    let regs = [exe::club_records_get_season_record(this, seller, 1), exe::club_records_get_all_time_record(this, seller, 1), exe::club_records_get_season_record(this, buyer, 1), exe::club_records_get_all_time_record(this, buyer, 1)];
    let queue = read_usize(this + club_records_fields::NEWS_QUEUE).wrapping_add(read_usize(this + club_records_fields::NEWS_QUEUE_COUNT).wrapping_mul(6));
    let reg = [(regs[0] + 0x365, 12), (regs[1] + 0x365, 12), (regs[2] + 0x365, 12), (regs[3] + 0x365, 12), (this + club_records_fields::NEWS_QUEUE_COUNT, 4), (queue, 12)];
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(this, value, buyer, seller), || add_transfer_totals(this, value, buyer, seller));
    SCOREBOARD.record(igual && r == n, || format!("add_transfer_totals({}, {}, {}) {:x?} x {:x?}", value, read_usize(buyer), read_usize(seller), mo, mr));
    r
}

/// Comparer for `club_records_update_career_records`.
///
/// [pt] Comparador: o registro de sempre é pedido antes (pode ser criado; a segunda vez só acha); lado a lado pelos dois
/// recordes e as referências (+0x247 a +0x27b). Nulos ou sem registro: só a original (a caixa).
pub(crate) unsafe extern "thiscall" fn compare_club_records_update_career_records(this: usize, player: usize, club: usize, games: u32, goals: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32>(IDX_CLUB_RECORDS_UPDATE_CAREER_RECORDS);
    if player == 0 || club == 0 {
        return o(this, player, club, games, goals);
    }
    let reg = exe::club_records_get_all_time_record(this, club, 1);
    if reg == 0 {
        return o(this, player, club, games, goals);
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(reg + club_record::LEAGUE_APPEARANCES, 0x34)], || o(this, player, club, games, goals), || club_records_update_career_records(this, player, club, games, goals));
    SCOREBOARD.record(igual && r == n, || format!("club_records_update_career_records({}, {}, {}, {}) {:x?} x {:x?}", read_usize(player), read_usize(club), games as i16, goals as i16, mo, mr));
    r
}

/// Comparer for `update_season_rating_record`.
///
/// [pt] Comparador: lado a lado pelo recorde de nota (+0x1f1 a +0x210 do registro da temporada, pedido antes sem criar).
pub(crate) unsafe extern "thiscall" fn compare_update_season_rating_record(this: usize, club: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_UPDATE_SEASON_RATING_RECORD);
    if club == 0 {
        return o(this, club);
    }
    let t = exe::club_records_get_season_record(this, club, 0);
    let reg: &[(usize, usize)] = if t == 0 { &[] } else { &[(t + club_record::SEASON_RATING, 0x1f)] };
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(reg, || o(this, club), || update_season_rating_record(this, club));
    SCOREBOARD.record(igual && r == n, || format!("update_season_rating_record({}) original={} rust={} {:x?} x {:x?}", read_usize(club), r, n, mo, mr));
    r
}

/// Comparer for `get_all_time_record`.
///
/// [pt] Comparador: a criação realoca e não pode rodar duas vezes: o plano da cópia antes, a original roda, confere.
pub(crate) unsafe extern "thiscall" fn compare_get_all_time_record(this: usize, club: usize, create: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(IDX_GET_ALL_TIME_RECORD);
    if club == 0 {
        return o(this, club, create);
    }
    let e = club_records_entry(this, club);
    let plan = record_index_plan(e, this + club_records_fields::ALL_TIME_COUNT, create);
    let r = o(this, club, create);
    check_record_plan("get_all_time_record", club, e, this + club_records_fields::ALL_TIME_COUNT, this + club_records_fields::ALL_TIME_RECORDS, plan, r);
    r
}

/// Comparador: o mesmo pelos recordes da temporada.
pub(crate) unsafe extern "thiscall" fn compare_get_season_record(this: usize, club: usize, create: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(IDX_GET_SEASON_RECORD);
    if club == 0 {
        return o(this, club, create);
    }
    let e = club_records_entry(this, club) + 4;
    let plan = record_index_plan(e, this + club_records_fields::SEASON_COUNT, create);
    let r = o(this, club, create);
    check_record_plan("get_season_record", club, e, this + club_records_fields::SEASON_COUNT, this + club_records_fields::SEASON_RECORDS, plan, r);
    r
}

/// Comparer for `queue_record_news`.
///
/// [pt] Comparador: a original acrescenta (pode realocar); confere a conta, o item novo (id, tipo) e a devolução.
pub(crate) unsafe extern "thiscall" fn compare_queue_record_news(this: usize, id: u32, kind: u32) -> u32 {
    let n = read_usize(this + club_records_fields::NEWS_QUEUE_COUNT);
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(IDX_QUEUE_RECORD_NEWS)(this, id, kind);
    if r == 0 {
        return r; // out of memory: the box has already shown
    }
    let item = read_usize(this + club_records_fields::NEWS_QUEUE).wrapping_add(n.wrapping_mul(6));
    let (gid, gtipo) = (core::ptr::read_unaligned(item as *const u32), *((item + record_news_item::RECORD_TYPE) as *const u8));
    SCOREBOARD.record(r == 1 && read_usize(this + club_records_fields::NEWS_QUEUE_COUNT) == n.wrapping_add(1) && gid == id && gtipo == kind as u8, || {
        format!("queue_record_news({}, {}) devolveu {}, conta {} -> {}, item ({}, {})", id, kind as u8, r, n, read_usize(this + club_records_fields::NEWS_QUEUE_COUNT), gid, gtipo)
    });
    r
}

/// Comparer for `offer_breaks_fee_record`.
///
/// [pt] Comparador: a 0x452710 cria os recordes do clube na primeira vez (a original primeiro; a cópia acha os mesmos):
/// pelo valor.
pub(crate) unsafe extern "thiscall" fn compare_offer_breaks_fee_record(this: usize, offer: usize, exigir: u32) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u8>(IDX_OFFER_BREAKS_FEE_RECORD)(this, offer, exigir);
    let n = offer_breaks_fee_record(this, offer, exigir);
    SCOREBOARD.record(r == n, || format!("offer_breaks_fee_record({:x}, {}) original={} rust={}", offer, exigir, r, n));
    r
}

/// Comparer for `transfer_fee_record_construct`.
pub(crate) unsafe extern "thiscall" fn compare_transfer_fee_record_construct(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_TRANSFER_FEE_RECORD_CONSTRUCT);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(this, 0x28)], || o(this), || transfer_fee_record_construct(this));
    SCOREBOARD.record(igual && r == n, || format!("transfer_fee_record_construct {:x?} x {:x?}", mo, mr));
    r
}

/// Comparer for `transfer_fee_record_construct_2`.
pub(crate) unsafe extern "thiscall" fn compare_transfer_fee_record_construct_2(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_TRANSFER_FEE_RECORD_CONSTRUCT_2);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(this, 0x28)], || o(this), || transfer_fee_record_construct_2(this));
    SCOREBOARD.record(igual && r == n, || format!("transfer_fee_record_construct_2 {:x?} x {:x?}", mo, mr));
    r
}

/// Comparer for `run_record_construct`.
pub(crate) unsafe extern "thiscall" fn compare_run_record_construct(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_RUN_RECORD_CONSTRUCT);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(this, 0x14)], || o(this), || run_record_construct(this));
    SCOREBOARD.record(igual && r == n, || format!("run_record_construct {:x?} x {:x?}", mo, mr));
    r
}

/// Comparer for `won_run_record_construct`.
pub(crate) unsafe extern "thiscall" fn compare_won_run_record_construct(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_WON_RUN_RECORD_CONSTRUCT);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(this, 0x14)], || o(this), || won_run_record_construct(this));
    SCOREBOARD.record(igual && r == n, || format!("won_run_record_construct {:x?} x {:x?}", mo, mr));
    r
}

/// Comparer for `lost_run_record_construct`.
pub(crate) unsafe extern "thiscall" fn compare_lost_run_record_construct(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_LOST_RUN_RECORD_CONSTRUCT);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(this, 0x14)], || o(this), || lost_run_record_construct(this));
    SCOREBOARD.record(igual && r == n, || format!("lost_run_record_construct {:x?} x {:x?}", mo, mr));
    r
}

/// Comparer for `unbeaten_run_record_construct`.
pub(crate) unsafe extern "thiscall" fn compare_unbeaten_run_record_construct(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_UNBEATEN_RUN_RECORD_CONSTRUCT);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(this, 0x14)], || o(this), || unbeaten_run_record_construct(this));
    SCOREBOARD.record(igual && r == n, || format!("unbeaten_run_record_construct {:x?} x {:x?}", mo, mr));
    r
}

/// Comparer for `winless_run_record_construct`.
pub(crate) unsafe extern "thiscall" fn compare_winless_run_record_construct(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_WINLESS_RUN_RECORD_CONSTRUCT);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(this, 0x14)], || o(this), || winless_run_record_construct(this));
    SCOREBOARD.record(igual && r == n, || format!("winless_run_record_construct {:x?} x {:x?}", mo, mr));
    r
}

/// Comparer for `goals_in_match_record_construct`.
pub(crate) unsafe extern "thiscall" fn compare_goals_in_match_record_construct(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_GOALS_IN_MATCH_RECORD_CONSTRUCT);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(this, 0x21)], || o(this), || goals_in_match_record_construct(this));
    SCOREBOARD.record(igual && r == n, || format!("goals_in_match_record_construct {:x?} x {:x?}", mo, mr));
    r
}

/// Comparer for `youngest_player_record_construct`.
pub(crate) unsafe extern "thiscall" fn compare_youngest_player_record_construct(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_YOUNGEST_PLAYER_RECORD_CONSTRUCT);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(this, 0x28)], || o(this), || youngest_player_record_construct(this));
    SCOREBOARD.record(igual && r == n, || format!("youngest_player_record_construct {:x?} x {:x?}", mo, mr));
    r
}

/// Comparer for `oldest_player_record_construct`.
pub(crate) unsafe extern "thiscall" fn compare_oldest_player_record_construct(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_OLDEST_PLAYER_RECORD_CONSTRUCT);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(this, 0x28)], || o(this), || oldest_player_record_construct(this));
    SCOREBOARD.record(igual && r == n, || format!("oldest_player_record_construct {:x?} x {:x?}", mo, mr));
    r
}

/// Comparer for `rating_record_assign`.
pub(crate) unsafe extern "thiscall" fn compare_rating_record_assign(this: usize, source: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_RATING_RECORD_ASSIGN);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(this, 0x1f)], || o(this, source), || rating_record_assign(this, source));
    SCOREBOARD.record(igual && r == n, || format!("rating_record_assign {:x?} x {:x?}", mo, mr));
    r
}

/// Comparer for `man_of_match_record_assign`.
pub(crate) unsafe extern "thiscall" fn compare_man_of_match_record_assign(this: usize, source: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_MAN_OF_MATCH_RECORD_ASSIGN);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(this, 0x1b)], || o(this, source), || man_of_match_record_assign(this, source));
    SCOREBOARD.record(igual && r == n, || format!("man_of_match_record_assign {:x?} x {:x?}", mo, mr));
    r
}

/// Comparer for `discipline_record_assign`.
pub(crate) unsafe extern "thiscall" fn compare_discipline_record_assign(this: usize, source: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_DISCIPLINE_RECORD_ASSIGN);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(this, 0x1c)], || o(this, source), || discipline_record_assign(this, source));
    SCOREBOARD.record(igual && r == n, || format!("discipline_record_assign {:x?} x {:x?}", mo, mr));
    r
}

/// Comparer for `count_record_assign`.
pub(crate) unsafe extern "thiscall" fn compare_count_record_assign(this: usize, source: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_COUNT_RECORD_ASSIGN);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(this, 0x1a)], || o(this, source), || count_record_assign(this, source));
    SCOREBOARD.record(igual && r == n, || format!("count_record_assign {:x?} x {:x?}", mo, mr));
    r
}

/// Comparer for `fans_player_record_assign`.
pub(crate) unsafe extern "thiscall" fn compare_fans_player_record_assign(this: usize, source: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_FANS_PLAYER_RECORD_ASSIGN);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(this, 0x1a)], || o(this, source), || fans_player_record_assign(this, source));
    SCOREBOARD.record(igual && r == n, || format!("fans_player_record_assign {:x?} x {:x?}", mo, mr));
    r
}

/// Comparer for `tally_record_construct`.
pub(crate) unsafe extern "thiscall" fn compare_tally_record_construct(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_TALLY_RECORD_CONSTRUCT);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(this, 0x19)], || o(this), || tally_record_construct(this));
    SCOREBOARD.record(igual && r == n, || format!("tally_record_construct {:x?} x {:x?}", mo, mr));
    r
}

/// Comparer for `count_record_construct`.
pub(crate) unsafe extern "thiscall" fn compare_count_record_construct(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_COUNT_RECORD_CONSTRUCT);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(this, 0x1a)], || o(this), || count_record_construct(this));
    SCOREBOARD.record(igual && r == n, || format!("count_record_construct {:x?} x {:x?}", mo, mr));
    r
}

/// Comparer for `transfer_fee_record_assign`.
pub(crate) unsafe extern "thiscall" fn compare_transfer_fee_record_assign(this: usize, source: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_TRANSFER_FEE_RECORD_ASSIGN);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(this, 0x29)], || o(this, source), || transfer_fee_record_assign(this, source));
    SCOREBOARD.record(igual && r == n, || format!("transfer_fee_record_assign {:x?} x {:x?}", mo, mr));
    r
}

/// Comparer for `goals_in_match_record_assign`.
pub(crate) unsafe extern "thiscall" fn compare_goals_in_match_record_assign(this: usize, source: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_GOALS_IN_MATCH_RECORD_ASSIGN);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(this, 0x38)], || o(this, source), || goals_in_match_record_assign(this, source));
    SCOREBOARD.record(igual && r == n, || format!("goals_in_match_record_assign {:x?} x {:x?}", mo, mr));
    r
}

/// Comparer for `record_holder_assign`.
pub(crate) unsafe extern "thiscall" fn compare_record_holder_assign(this: usize, source: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_RECORD_HOLDER_ASSIGN);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(this, 0x18)], || o(this, source), || record_holder_assign(this, source));
    SCOREBOARD.record(igual && r == n, || format!("record_holder_assign {:x?} x {:x?}", mo, mr));
    r
}

/// Comparer for `match_record_assign`.
pub(crate) unsafe extern "thiscall" fn compare_match_record_assign(this: usize, source: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_MATCH_RECORD_ASSIGN);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(this, 0x1f)], || o(this, source), || match_record_assign(this, source));
    SCOREBOARD.record(igual && r == n, || format!("match_record_assign {:x?} x {:x?}", mo, mr));
    r
}

/// Comparer for `run_record_assign`.
pub(crate) unsafe extern "thiscall" fn compare_run_record_assign(this: usize, source: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_RUN_RECORD_ASSIGN);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(this, 0x14)], || o(this, source), || run_record_assign(this, source));
    SCOREBOARD.record(igual && r == n, || format!("run_record_assign {:x?} x {:x?}", mo, mr));
    r
}

/// Comparer for `reset_records`.
pub(crate) unsafe extern "C" fn compare_reset_records(r: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_RESET_RECORDS);
    let reg = if r == 0 { vec![] } else { vec![(r, 0x381)] };
    let (a, v, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(r), || reset_records(r));
    SCOREBOARD.record(igual && a == v, || format!("reset_records({:#x}) original={} {:x?} rust={} {:x?}", r, a, mo, v, mr));
    a
}

/// Comparer for `record_holder_construct`.
pub(crate) unsafe extern "thiscall" fn compare_record_holder_construct(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_RECORD_HOLDER_CONSTRUCT);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(this + record_holder::DATE, 8)], || o(this), || record_holder_construct(this));
    SCOREBOARD.record(igual && r == n, || format!("record_holder_construct {:x?} x {:x?}", mo, mr));
    r
}
