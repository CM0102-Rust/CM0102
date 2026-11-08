use super::*;

// The comparers of the routines in ../club_records.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../club_records.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `update_league_position_records` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original roda antes (ordena as tabelas, que a 0x6827d0 confere por conta própria) e a cópia depois,
/// em ensaio (sem ordenar), a partir dos mesmos recordes: os 14 primeiros bytes dos registros da temporada e de sempre
/// de cada clube da cadeia, o cabeçalho da fila (+0x24..+0x30) e a fila; e a devolução. Liga nula ou com +0x44 ≠ 0xff,
/// registro a criar (0x4528d0/0x452710 alocam) ou fila que pode crescer: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_update_league_position_records(this: usize, liga0: usize, sempre: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_UPDATE_LEAGUE_POSITION_RECORDS);
    if liga0 == 0 || read_u8(liga0 + 0x44) != 0xff {
        return o(this, liga0, sempre); // simplification: as caixas, só a original
    }
    let human: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let national_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let mut regions = vec![(this + 0x24, 0xc)];
    let mut podem_entrar = 0i32;
    for liga in league_stage_chain(liga0) {
        for i in 0..read_i16(liga + 0x3e).max(0) as usize {
            let row = read_usize(liga + 0xb1) + 0x3b * i;
            let clube = read_usize(row);
            if read_u8(row + 0x11) == 0 || clube == 0 {
                continue;
            }
            let (t, s) = (club_season_record(this, clube, 0), club_all_time_record(this, clube, 0));
            if t == 0 || s == 0 {
                return o(this, liga0, sempre); // simplification: o registro seria criado, só a original
            }
            regions.push((t, 0xe));
            regions.push((s, 0xe));
            if human(clube, 0, 1, 0, 0) != 0 || national_team(clube) != 0 {
                podem_entrar += 2;
            }
        }
    }
    let queue = read_usize(this + 0x24);
    if (read_usize(this + 0x2c) as i32).wrapping_add(podem_entrar) > read_usize(this + 0x28) as i32 || (queue == 0 && podem_entrar > 0) {
        return o(this, liga0, sempre); // simplification: a fila pode crescer (realoca), só a original
    }
    if queue != 0 {
        regions.push((queue, 6 * read_usize(this + 0x28)));
    }
    let before = crate::comparacao::Memoria::ler(&regions);
    let ro = o(this, liga0, sempre);
    let mo = crate::comparacao::Memoria::ler(&regions);
    before.escrever();
    POSITION_RECORDS_DRY_RUN = true;
    let rr = crate::aleatorio::pausar_rastro(|| update_league_position_records(this, liga0, sempre));
    POSITION_RECORDS_DRY_RUN = false;
    let mr = crate::comparacao::Memoria::ler(&regions);
    mo.escrever();
    PLACAR.registrar(ro == rr && mo == mr, || format!("update_league_position_records({:x}, {:x}, {}) original={} {:x?} rust={} {:x?}", this, liga0, sempre, ro, mo, rr, mr));
    ro
}

/// Comparer for `send_record_news` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a cópia em ensaio (sem preencher nem entregar, sem soltar) roda antes, enquanto a fila está viva, e a
/// original depois a partir do mesmo cabeçalho (+0x24..+0x30): o cabeçalho e a devolução.
pub(crate) unsafe extern "thiscall" fn comparar_send_record_news(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SEND_RECORD_NEWS);
    let regions = [(this + 0x24, 0xc)];
    let before = crate::comparacao::Memoria::ler(&regions);
    RECORD_NEWS_DRY_RUN = true;
    let rr = crate::aleatorio::pausar_rastro(|| send_record_news(this));
    RECORD_NEWS_DRY_RUN = false;
    let mr = crate::comparacao::Memoria::ler(&regions);
    before.escrever();
    let ro = o(this);
    let mo = crate::comparacao::Memoria::ler(&regions);
    PLACAR.registrar(ro == rr && mo == mr, || format!("send_record_news({:x}) original={} {:x?} rust={} {:x?}", this, ro, mo, rr, mr));
    ro
}

/// Comparer for `pick_fans_player_of_year` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pela referência do escolhido (registro +0x371, 0x18 bytes) e pela devolução.
pub(crate) unsafe extern "thiscall" fn comparar_pick_fans_player_of_year(this: usize, clube: usize, record: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_PICK_FANS_PLAYER_OF_YEAR);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(record + 0x371, 0x18)], || o(this, clube, record), || pick_fans_player_of_year(this, clube, record));
    PLACAR.registrar(igual && r == n, || format!("pick_fans_player_of_year(clube {:x}) original={} {:x?} rust={} {:x?}", read_usize(clube), r, mo, n, mr));
    r
}

/// Comparer for `update_transfer_fee_records` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original põe na fila de notícias (realoca) e o gostar sorteia: o plano antes (os quatro registros
/// criados na mesma ordem da original, e os recordes pago e recebido esperados); a original roda; pega os registros
/// de novo (o vetor pode ter sido realocado) e confere os valores e o clube de cada recorde e o [0xae1080].
pub(crate) unsafe extern "thiscall" fn comparar_update_transfer_fee_records(this: usize, t: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_UPDATE_TRANSFER_FEE_RECORDS);
    let person = read_usize(G_STAFF_TABLE).wrapping_add((read_i32(t + 0xc) as usize).wrapping_mul(staff::SIZE));
    let buyer = injury_club_by_index(read_i32(t + 8));
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let contrato = contract_of(read_usize(G_CONTRACT_MANAGER), person);
    let seller = if contrato != 0 { injury_club_by_index(read_i32(contrato + 4)) } else { 0 };
    if seller == 0 {
        let r = o(this, t);
        PLACAR.registrar(r == 1 && read_usize(G_CONTRACT_LOOKUP_RESULT) == 0, || format!("update_transfer_fee_records sem vendedor: {} [G_CONTRACT_LOOKUP_RESULT]={:x}", r, read_usize(G_CONTRACT_LOOKUP_RESULT)));
        return r;
    }
    let season: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_CLUB_RECORDS_GET_SEASON_RECORD);
    let sempre: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_CLUB_RECORDS_GET_ALL_TIME_RECORD);
    let records = || [season(this, buyer, 1), sempre(this, buyer, 1), season(this, seller, 1), sempre(this, seller, 1)];
    let value = read_i32(t + 0x1b);
    let rg = records();
    // (valor, clube) de cada recorde: pago da temporada e de sempre do comprador, recebido do vendedor
    let mut plan = [(read_i32(rg[0] + 0x313), read_usize(rg[0] + 0x32f)), (read_i32(rg[1] + 0x313), read_usize(rg[1] + 0x32f)),
                     (read_i32(rg[2] + 0x33c), read_usize(rg[2] + 0x358)), (read_i32(rg[3] + 0x33c), read_usize(rg[3] + 0x358))];
    let mut queue = read_i32(this + 0x2c);
    if value != 0 {
        if value > plan[0].0 {
            plan[0] = (value, read_usize(seller));
            if value > plan[1].0 {
                plan[1] = plan[0];
                if is_big_transfer(value, buyer) && !record_news_queued(this, read_usize(buyer), 0x1c) {
                    queue += 1;
                }
            }
        }
        if value > plan[2].0 {
            plan[2] = (value, read_usize(buyer));
            if value > plan[3].0 {
                plan[3] = plan[2];
                if is_big_transfer(value, seller) && !record_news_queued(this, read_usize(seller), 0x1d) {
                    queue += 1;
                }
            }
        }
    }
    let r = o(this, t);
    let rg = records();
    let now = [(read_i32(rg[0] + 0x313), read_usize(rg[0] + 0x32f)), (read_i32(rg[1] + 0x313), read_usize(rg[1] + 0x32f)),
                 (read_i32(rg[2] + 0x33c), read_usize(rg[2] + 0x358)), (read_i32(rg[3] + 0x33c), read_usize(rg[3] + 0x358))];
    let queue_now = read_i32(this + 0x2c);
    PLACAR.registrar(r == 1 && now == plan && queue_now == queue && read_usize(G_CONTRACT_LOOKUP_RESULT) == contrato, || {
        format!("update_transfer_fee_records(valor {}) plano {:x?} fila {} x {:x?} fila {}", value, plan, queue, now, queue_now)
    });
    r
}

/// Comparer for `add_transfer_totals` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: os registros podem ser criados (realloc) e a notícia realoca: os quatro registros são pedidos antes, na
/// ordem da original (vendedor: temporada, sempre; comprador: temporada, sempre; a segunda vez só acha), e com a fila
/// de notícias sem folga para duas, só a original. Lado a lado pelos quatro registros (+0x365, 12 bytes), a conta das
/// notícias e as duas vagas seguintes.
pub(crate) unsafe extern "thiscall" fn comparar_add_transfer_totals(this: usize, value: i32, buyer: usize, seller: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, usize, usize) -> u32>(IDX_ADD_TRANSFER_TOTALS);
    if buyer == 0 || seller == 0 || (read_usize(this + 0x28).wrapping_sub(read_usize(this + 0x2c)) as i32) < 2 {
        return o(this, value, buyer, seller); // simplification: com realloc da fila, só a original
    }
    let season: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_CLUB_RECORDS_GET_SEASON_RECORD);
    let sempre: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_CLUB_RECORDS_GET_ALL_TIME_RECORD);
    let _ = [season(this, seller, 1), sempre(this, seller, 1), season(this, buyer, 1), sempre(this, buyer, 1)];
    // criar o registro do comprador pode realocar o vetor e mover o do vendedor: pega os quatro de novo (agora só acha)
    let regs = [season(this, seller, 1), sempre(this, seller, 1), season(this, buyer, 1), sempre(this, buyer, 1)];
    let queue = read_usize(this + 0x24).wrapping_add(read_usize(this + 0x2c).wrapping_mul(6));
    let reg = [(regs[0] + 0x365, 12), (regs[1] + 0x365, 12), (regs[2] + 0x365, 12), (regs[3] + 0x365, 12), (this + 0x2c, 4), (queue, 12)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, value, buyer, seller), || add_transfer_totals(this, value, buyer, seller));
    PLACAR.registrar(igual && r == n, || format!("add_transfer_totals({}, {}, {}) {:x?} x {:x?}", value, read_usize(buyer), read_usize(seller), mo, mr));
    r
}

/// Comparer for `club_records_update_career_records` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: o registro de sempre é pedido antes (pode ser criado; a segunda vez só acha); lado a lado pelos dois
/// recordes e as referências (+0x247 a +0x27b). Nulos ou sem registro: só a original (a caixa).
pub(crate) unsafe extern "thiscall" fn comparar_club_records_update_career_records(this: usize, player: usize, clube: usize, games: u32, goals: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32>(IDX_CLUB_RECORDS_UPDATE_CAREER_RECORDS);
    if player == 0 || clube == 0 {
        return o(this, player, clube, games, goals);
    }
    let sempre: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_CLUB_RECORDS_GET_ALL_TIME_RECORD);
    let reg = sempre(this, clube, 1);
    if reg == 0 {
        return o(this, player, clube, games, goals);
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(reg + 0x247, 0x34)], || o(this, player, clube, games, goals), || club_records_update_career_records(this, player, clube, games, goals));
    PLACAR.registrar(igual && r == n, || format!("club_records_update_career_records({}, {}, {}, {}) {:x?} x {:x?}", read_usize(player), read_usize(clube), games as i16, goals as i16, mo, mr));
    r
}

/// Comparer for `update_season_rating_record` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelo recorde de nota (+0x1f1 a +0x210 do registro da temporada, pedido antes sem criar).
pub(crate) unsafe extern "thiscall" fn comparar_update_season_rating_record(this: usize, clube: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_UPDATE_SEASON_RATING_RECORD);
    if clube == 0 {
        return o(this, clube);
    }
    let season: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_CLUB_RECORDS_GET_SEASON_RECORD);
    let t = season(this, clube, 0);
    let reg: &[(usize, usize)] = if t == 0 { &[] } else { &[(t + 0x1f1, 0x1f)] };
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(reg, || o(this, clube), || update_season_rating_record(this, clube));
    PLACAR.registrar(igual && r == n, || format!("update_season_rating_record({}) original={} rust={} {:x?} x {:x?}", read_usize(clube), r, n, mo, mr));
    r
}

/// Comparer for `get_all_time_record` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a criação realoca e não pode rodar duas vezes: o plano da cópia antes, a original roda, confere.
pub(crate) unsafe extern "thiscall" fn comparar_get_all_time_record(this: usize, clube: usize, create: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(IDX_GET_ALL_TIME_RECORD);
    if clube == 0 {
        return o(this, clube, create);
    }
    let e = club_records_entry(this, clube);
    let plan = record_index_plan(e, this + 0xc, create);
    let r = o(this, clube, create);
    check_record_plan("get_all_time_record", clube, e, this + 0xc, this + 4, plan, r);
    r
}

/// Comparador: o mesmo pelos recordes da temporada.
pub(crate) unsafe extern "thiscall" fn comparar_get_season_record(this: usize, clube: usize, create: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(IDX_GET_SEASON_RECORD);
    if clube == 0 {
        return o(this, clube, create);
    }
    let e = club_records_entry(this, clube) + 4;
    let plan = record_index_plan(e, this + 0x18, create);
    let r = o(this, clube, create);
    check_record_plan("get_season_record", clube, e, this + 0x18, this + 0x10, plan, r);
    r
}

/// Comparer for `queue_record_news` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original acrescenta (pode realocar); confere a conta, o item novo (id, tipo) e a devolução.
pub(crate) unsafe extern "thiscall" fn comparar_queue_record_news(this: usize, id: u32, kind: u32) -> u32 {
    let n = read_usize(this + 0x2c);
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(IDX_QUEUE_RECORD_NEWS)(this, id, kind);
    if r == 0 {
        return r; // sem memória: a caixa já saiu
    }
    let item = read_usize(this + 0x24).wrapping_add(n.wrapping_mul(6));
    let (gid, gtipo) = (core::ptr::read_unaligned(item as *const u32), *((item + 4) as *const u8));
    PLACAR.registrar(r == 1 && read_usize(this + 0x2c) == n.wrapping_add(1) && gid == id && gtipo == kind as u8, || {
        format!("queue_record_news({}, {}) devolveu {}, conta {} -> {}, item ({}, {})", id, kind as u8, r, n, read_usize(this + 0x2c), gid, gtipo)
    });
    r
}

/// Comparer for `offer_breaks_fee_record` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a 0x452710 cria os recordes do clube na primeira vez (a original primeiro; a cópia acha os mesmos):
/// pelo valor.
pub(crate) unsafe extern "thiscall" fn comparar_offer_breaks_fee_record(this: usize, offer: usize, exigir: u32) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u8>(IDX_OFFER_BREAKS_FEE_RECORD)(this, offer, exigir);
    let n = offer_breaks_fee_record(this, offer, exigir);
    PLACAR.registrar(r == n, || format!("offer_breaks_fee_record({:x}, {}) original={} rust={}", offer, exigir, r, n));
    r
}

/// Comparer for `transfer_fee_record_construct` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_transfer_fee_record_construct(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_TRANSFER_FEE_RECORD_CONSTRUCT);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x28)], || o(this), || transfer_fee_record_construct(this));
    PLACAR.registrar(igual && r == n, || format!("transfer_fee_record_construct {:x?} x {:x?}", mo, mr));
    r
}

/// Comparer for `transfer_fee_record_construct_2` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_transfer_fee_record_construct_2(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_TRANSFER_FEE_RECORD_CONSTRUCT_2);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x28)], || o(this), || transfer_fee_record_construct_2(this));
    PLACAR.registrar(igual && r == n, || format!("transfer_fee_record_construct_2 {:x?} x {:x?}", mo, mr));
    r
}

/// Comparer for `run_record_construct` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_run_record_construct(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_RUN_RECORD_CONSTRUCT);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x14)], || o(this), || run_record_construct(this));
    PLACAR.registrar(igual && r == n, || format!("run_record_construct {:x?} x {:x?}", mo, mr));
    r
}

/// Comparer for `won_run_record_construct` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_won_run_record_construct(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_WON_RUN_RECORD_CONSTRUCT);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x14)], || o(this), || won_run_record_construct(this));
    PLACAR.registrar(igual && r == n, || format!("won_run_record_construct {:x?} x {:x?}", mo, mr));
    r
}

/// Comparer for `lost_run_record_construct` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_lost_run_record_construct(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_LOST_RUN_RECORD_CONSTRUCT);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x14)], || o(this), || lost_run_record_construct(this));
    PLACAR.registrar(igual && r == n, || format!("lost_run_record_construct {:x?} x {:x?}", mo, mr));
    r
}

/// Comparer for `unbeaten_run_record_construct` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_unbeaten_run_record_construct(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_UNBEATEN_RUN_RECORD_CONSTRUCT);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x14)], || o(this), || unbeaten_run_record_construct(this));
    PLACAR.registrar(igual && r == n, || format!("unbeaten_run_record_construct {:x?} x {:x?}", mo, mr));
    r
}

/// Comparer for `winless_run_record_construct` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_winless_run_record_construct(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_WINLESS_RUN_RECORD_CONSTRUCT);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x14)], || o(this), || winless_run_record_construct(this));
    PLACAR.registrar(igual && r == n, || format!("winless_run_record_construct {:x?} x {:x?}", mo, mr));
    r
}

/// Comparer for `goals_in_match_record_construct` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_goals_in_match_record_construct(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_GOALS_IN_MATCH_RECORD_CONSTRUCT);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x21)], || o(this), || goals_in_match_record_construct(this));
    PLACAR.registrar(igual && r == n, || format!("goals_in_match_record_construct {:x?} x {:x?}", mo, mr));
    r
}

/// Comparer for `youngest_player_record_construct` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_youngest_player_record_construct(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_YOUNGEST_PLAYER_RECORD_CONSTRUCT);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x28)], || o(this), || youngest_player_record_construct(this));
    PLACAR.registrar(igual && r == n, || format!("youngest_player_record_construct {:x?} x {:x?}", mo, mr));
    r
}

/// Comparer for `oldest_player_record_construct` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_oldest_player_record_construct(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_OLDEST_PLAYER_RECORD_CONSTRUCT);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x28)], || o(this), || oldest_player_record_construct(this));
    PLACAR.registrar(igual && r == n, || format!("oldest_player_record_construct {:x?} x {:x?}", mo, mr));
    r
}

/// Comparer for `rating_record_assign` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_rating_record_assign(this: usize, source: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_RATING_RECORD_ASSIGN);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x1f)], || o(this, source), || rating_record_assign(this, source));
    PLACAR.registrar(igual && r == n, || format!("rating_record_assign {:x?} x {:x?}", mo, mr));
    r
}

/// Comparer for `man_of_match_record_assign` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_man_of_match_record_assign(this: usize, source: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_MAN_OF_MATCH_RECORD_ASSIGN);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x1b)], || o(this, source), || man_of_match_record_assign(this, source));
    PLACAR.registrar(igual && r == n, || format!("man_of_match_record_assign {:x?} x {:x?}", mo, mr));
    r
}

/// Comparer for `discipline_record_assign` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_discipline_record_assign(this: usize, source: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_DISCIPLINE_RECORD_ASSIGN);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x1c)], || o(this, source), || discipline_record_assign(this, source));
    PLACAR.registrar(igual && r == n, || format!("discipline_record_assign {:x?} x {:x?}", mo, mr));
    r
}

/// Comparer for `count_record_assign` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_count_record_assign(this: usize, source: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_COUNT_RECORD_ASSIGN);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x1a)], || o(this, source), || count_record_assign(this, source));
    PLACAR.registrar(igual && r == n, || format!("count_record_assign {:x?} x {:x?}", mo, mr));
    r
}

/// Comparer for `fans_player_record_assign` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_fans_player_record_assign(this: usize, source: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_FANS_PLAYER_RECORD_ASSIGN);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x1a)], || o(this, source), || fans_player_record_assign(this, source));
    PLACAR.registrar(igual && r == n, || format!("fans_player_record_assign {:x?} x {:x?}", mo, mr));
    r
}

/// Comparer for `tally_record_construct` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_tally_record_construct(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_TALLY_RECORD_CONSTRUCT);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x19)], || o(this), || tally_record_construct(this));
    PLACAR.registrar(igual && r == n, || format!("tally_record_construct {:x?} x {:x?}", mo, mr));
    r
}

/// Comparer for `count_record_construct` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_count_record_construct(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_COUNT_RECORD_CONSTRUCT);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x1a)], || o(this), || count_record_construct(this));
    PLACAR.registrar(igual && r == n, || format!("count_record_construct {:x?} x {:x?}", mo, mr));
    r
}

/// Comparer for `transfer_fee_record_assign` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_transfer_fee_record_assign(this: usize, source: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_TRANSFER_FEE_RECORD_ASSIGN);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x29)], || o(this, source), || transfer_fee_record_assign(this, source));
    PLACAR.registrar(igual && r == n, || format!("transfer_fee_record_assign {:x?} x {:x?}", mo, mr));
    r
}

/// Comparer for `goals_in_match_record_assign` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_goals_in_match_record_assign(this: usize, source: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_GOALS_IN_MATCH_RECORD_ASSIGN);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x38)], || o(this, source), || goals_in_match_record_assign(this, source));
    PLACAR.registrar(igual && r == n, || format!("goals_in_match_record_assign {:x?} x {:x?}", mo, mr));
    r
}

/// Comparer for `record_holder_assign` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_record_holder_assign(this: usize, source: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_RECORD_HOLDER_ASSIGN);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x18)], || o(this, source), || record_holder_assign(this, source));
    PLACAR.registrar(igual && r == n, || format!("record_holder_assign {:x?} x {:x?}", mo, mr));
    r
}

/// Comparer for `match_record_assign` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_match_record_assign(this: usize, source: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_MATCH_RECORD_ASSIGN);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x1f)], || o(this, source), || match_record_assign(this, source));
    PLACAR.registrar(igual && r == n, || format!("match_record_assign {:x?} x {:x?}", mo, mr));
    r
}

/// Comparer for `run_record_assign` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_run_record_assign(this: usize, source: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_RUN_RECORD_ASSIGN);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x14)], || o(this, source), || run_record_assign(this, source));
    PLACAR.registrar(igual && r == n, || format!("run_record_assign {:x?} x {:x?}", mo, mr));
    r
}

/// Comparer for `reset_records` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_reset_records(r: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_RESET_RECORDS);
    let reg = if r == 0 { vec![] } else { vec![(r, 0x381)] };
    let (a, v, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(r), || reset_records(r));
    PLACAR.registrar(igual && a == v, || format!("reset_records({:#x}) original={} {:x?} rust={} {:x?}", r, a, mo, v, mr));
    a
}

/// Comparer for `record_holder_construct` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_record_holder_construct(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_RECORD_HOLDER_CONSTRUCT);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this + 0x10, 8)], || o(this), || record_holder_construct(this));
    PLACAR.registrar(igual && r == n, || format!("record_holder_construct {:x?} x {:x?}", mo, mr));
    r
}
