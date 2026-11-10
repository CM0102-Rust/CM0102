use super::*;

// The comparers of the routines in ../comp/leagues/fin_prm.rs.
// [pt] Os comparadores das rotinas de ../comp/leagues/fin_prm.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `fin_prm_end_season_for_nation`.
///
/// The nation's end-of-season comparer: only the original routine runs, because it calls each club's end of season
/// (`season_end_club_update`), which draws at random and delivers news items to the human manager (the copy carries on in Rust mode,
/// with the counter).
///
/// It checks what the routine guarantees and no call undoes: the last club of the entry (the club order) ends up with
/// the year's totals (+0x110, 0x50 bytes) zeroed.
///
/// Ponytail: the earlier ones may gain values through the `season_end_club_update` of a later club (the B team that uses the A team's
/// record, the shared stadium); checking all of them requires planning those exceptions.
///
/// [pt] O comparador do fim de temporada da nação: só a original, porque ela chama o fim de temporada de cada clube
/// (0x59cfa0), que sorteia e entrega notícias ao técnico humano (a cópia segue no modo Rust, com o contador). Confere o
/// que a rotina garante e nenhuma chamada desfaz: o último clube da entrada (a ordem dos clubes) fica com os totais do
/// ano (+0x110, 0x50 bytes) zerados. Ponytail: os anteriores podem ganhar valores pela 0x59cfa0 de um clube seguinte
/// (o B que usa o registro do A, o estádio dividido); conferir todos pede o plano dessas exceções.
pub(crate) unsafe extern "thiscall" fn compare_fin_prm_end_season_for_nation(this: usize, entry: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_FIN_PRM_END_SEASON_FOR_NATION);
    if entry == 0 || exe::date_operator_3c(G_CURRENT_DATE, read_usize(G_GAME_START_DATE) as u32, read_usize(G_GAME_START_DATE_FLAGS) as u32) != 0 {
        return o(this, entry);
    }
    let mut last = None;
    let mut club = read_usize(G_CLUB_TABLE);
    for k in 0..read_i32(G_CLUB_COUNT).max(0) as usize {
        if exe::database_is_national_team(club) != 0 {
            break;
        }
        let nation = Nation(read_usize(club + club::NATION));
        if read_usize(club + club::DIVISION) != 0 && !nation.is_null() && (nation.league_selected() != 0 || read_i16(club + club::REPUTATION) >= 0x1388) && (0xb63d60 + (exe::key_nation_index_of_club(club) as i8 as i32) * 72) as usize == entry {
            last = Some(k);
        }
        club += 0x245;
    }
    o(this, entry);
    if let Some(k) = last {
        let f = read_usize(this) + k * 0x167 + 0x110;
        let zerado = core::slice::from_raw_parts(f as *const u8, 0x50).iter().all(|&x| x == 0);
        SCOREBOARD.record(zerado, || format!("fin_prm_end_season_for_nation(entrada {:#x}) clube {} sem zerar os totais do ano", entry, k));
    }
}

/// Comparer for `season_end_club_update` (comparison mode: see the header of motor.rs): the comparer of the club's
/// season end, side by side (the copy without counting the random draws).
///
/// Compared: the finances of the partner and of the club itself, the club, the stadium (78 bytes), the requests row
/// (0x49 bytes) and the random-draw state. The news (`0x59ffb0`, `0x59fae0`, `0x59fe10`) is built on the stack and only
/// delivered to the human manager (`0x7861e0`): with a computer manager it leaves nothing besides the random draw(2) of
/// the refusal, which both runs make.
///
/// Only the original routine: the human manager (the news goes into his inbox) and the shared stadium (it touches
/// another club and its finances).
///
/// [pt] O comparador do fim de temporada do clube: lado a lado (a cópia sem contar os sorteios) nas finanças do parceiro e do
/// próprio clube, no clube, no estádio (78 bytes), na linha dos pedidos (0x49 bytes) e no estado do sorteio. As notícias
/// (0x59ffb0, 0x59fae0, 0x59fe10) são montadas na pilha e só são entregues ao técnico humano (0x7861e0): com técnico da
/// máquina elas não deixam nada além do sorteio(2) da recusa, que as duas rodadas fazem. Só a original: o técnico
/// humano (a notícia entra na caixa dele) e o estádio dividido (mexe noutro clube e nas finanças dele).
pub(crate) unsafe extern "thiscall" fn compare_season_end_club_update(this: usize, club: usize) {
    let club = Club(club);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_SEASON_END_CLUB_UPDATE);
    if club.is_null() || exe::database_is_national_team(club) != 0 || fin_prm_has_human_manager(club) {
        return o(this, club.addr());
    }
    let mut a = 0u32;
    let fin = finance_of_parent_club(this, club, &mut a);
    let stadium = club.stadium().addr();
    if fin.is_null() || (stadium != 0 && club.own_stadium() == 0 && club.division().addr() != 0 && read_i32(fin.addr()) > 0x2160ec0) {
        return o(this, club.addr()); // simplification: the stadium purchase may touch the owner; original only
    }
    let own = read_usize(this).wrapping_add((read_i32(club.addr()) as u32).wrapping_mul(0x167) as usize);
    let mut reg = vec![(fin.addr(), 0x167), (club.addr(), 0x245), (board_requests_row(club), 0x49)];
    if own != fin.addr() {
        reg.push((own, 0x167));
    }
    if stadium != 0 {
        reg.push((stadium, 0x4e));
    }
    reg.extend(crate::rng::STATE_REGIONS);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, club.addr()), || crate::rng::without_counting(|| season_end_club_update(this, club.addr())));
    SCOREBOARD.record(igual, || format!("season_end_club_update(clube {:x}) original={:x?} rust={:x?}", club.addr(), mo, mr));
}

/// Comparer for `news_dividend`.
///
/// [pt] O comparador da notícia do dividendo: só a original (simplification: a notícia vai para a caixa do técnico humano e não
/// pode ser entregue duas vezes; com técnico da máquina não deixa nada).
pub(crate) unsafe extern "thiscall" fn compare_news_dividend(this: usize, club: usize, value: i32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, i32)>(IDX_NEWS_DIVIDEND)(this, club, value)
}

/// Comparer for `get_max_wage`.
///
/// [pt] O comparador do teto salarial: pela devolução (a rotina só lê; as chamadas de dentro também).
pub(crate) unsafe extern "thiscall" fn compare_get_max_wage(fin: usize, person: usize, job: u32) -> i32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> i32>(IDX_GET_MAX_WAGE)(fin, person, job);
    let n = get_max_wage(fin, person, job);
    SCOREBOARD.record(r == n, || format!("get_max_wage(fin {:x}, pessoa {:x}, cargo {}) original={} rust={}", fin, person, job as u8 as i8, r, n));
    r
}

/// Comparer for `news_ground_works`.
///
/// [pt] O comparador da notícia das obras: só a original (simplification: a notícia vai para a caixa do técnico humano e não pode
/// ser entregue duas vezes; com técnico da máquina não deixa nada).
pub(crate) unsafe extern "thiscall" fn compare_news_ground_works(this: usize, club: usize, value: u32, kind: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32)>(IDX_NEWS_GROUND_WORKS)(this, club, value, kind)
}

/// Comparer for `news_request_refused`.
///
/// [pt] O comparador da notícia da obra recusada: só a original (simplification: entrega ao técnico humano e sorteia; não pode
/// rodar duas vezes).
pub(crate) unsafe extern "thiscall" fn compare_news_request_refused(this: usize, club: usize, subject: u32, reason: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32)>(IDX_NEWS_REQUEST_REFUSED)(this, club, subject, reason)
}

/// Comparer for `save_finance_dat`.
///
/// [pt] Comparador da 0x59b3b0: só a original.
pub(crate) unsafe extern "thiscall" fn compare_save_finance_dat(this: usize, save: usize) {
    // simplification: writes to the save container (cannot run twice); the copy is proven by the save proof
    // (the .qst saved in rust mode with and without the batch, item by item)
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_SAVE_FINANCE_DAT)(this, save)
}

/// Comparer for `load_from_save`.
///
/// [pt] Comparador da 0x59b2a0: só a original.
pub(crate) unsafe extern "thiscall" fn compare_load_from_save(this: usize, save: usize) -> u32 {
    // simplification: reads from the container (the item position advances; cannot run twice); the copy is proved by loading
    // of the holiday save in the save proof (levas/N/CARGA)
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_LOAD_FROM_SAVE)(this, save)
}

/// Comparer for `process_match_income`.
///
/// [pt] Comparador da 0x59a5d0: lado a lado nos dois registros de finanças e no estado do sorteio (os sorteios de 10, dos
/// gastos e da manutenção). Sem resultado, sem partida, sem um dos clubes ou com seleção: só a original.
pub(crate) unsafe extern "thiscall" fn compare_process_match_income(table: usize, partida: usize, result: usize) {
    let partida = Fixture(partida);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_PROCESS_MATCH_INCOME);
    if result == 0 || partida.is_null() || partida.home_club().is_null() || partida.away_club().is_null() {
        return o(table, partida.addr(), result);
    }
    let (Some(fm), Some(fv)) = (match_club_finance(table, partida, 0x1c), match_club_finance(table, partida, 0x20)) else {
        return o(table, partida.addr(), result); // national team: the original does nothing
    };
    if fm == fv {
        return o(table, partida.addr(), result); // simplification: the same record on both sides, original only
    }
    let mut reg = vec![(fm, 0x167), (fv, 0x167)];
    reg.extend(crate::rng::STATE_REGIONS);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(table, partida.addr(), result), || crate::rng::without_counting(|| process_match_income(table, partida.addr(), result)));
    SCOREBOARD.record(igual, || format!("process_match_income(partida {:x}) original={:x?} rust={:x?}", partida.addr(), mo, mr));
}

/// Comparer for `calc_gate_receipts`.
///
/// [pt] Comparador da 0x59af40: a devolução (só lê; a 0x540a50 pode zerar a marca +0x244 do clube, igual nas duas). Sem
/// partida: só a original (a caixa).
pub(crate) unsafe extern "thiscall" fn compare_calc_gate_receipts(table: usize, partida: usize, attendance: i32) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32) -> i32>(IDX_CALC_GATE_RECEIPTS);
    if partida == 0 {
        return o(table, partida, attendance);
    }
    let r = o(table, partida, attendance);
    let n = calc_gate_receipts(table, partida, attendance);
    SCOREBOARD.record(r == n, || format!("calc_gate_receipts(partida {:x}, publico {}) original={} rust={}", partida, attendance, r, n));
    r
}

/// Comparer for `pay_league_fine`.
///
/// [pt] Comparador da 0x599850: lado a lado no registro de finanças (0x167 bytes).
pub(crate) unsafe extern "thiscall" fn compare_pay_league_fine(this: usize, value: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32)>(IDX_PAY_LEAGUE_FINE);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(this, 0x167)], || o(this, value), || pay_league_fine(this, value));
    SCOREBOARD.record(igual, || format!("pay_league_fine(valor {}) original={:x?} rust={:x?}", value, mo, mr));
}

/// Comparer for `try_finance_boost_event`.
pub(crate) unsafe extern "thiscall" fn compare_try_finance_boost_event(this: usize, club: usize, fin: usize) -> u32 {
    let club = Club(club);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_TRY_FINANCE_BOOST_EVENT);
    let presence = Staff(if !club.is_null() { club.chairman().addr() } else { 0 });
    let sheet = PlayerSheet(if !presence.is_null() { presence.non_player().addr() } else { 0 });
    if club.is_null() || fin == 0 {
        return o(this, club.addr(), fin); // simplification: without a club or without finances, original only
    }
    let mut extras = vec![(club.addr() + club::PLC, 1)];
    let mut check = Vec::new();
    if !sheet.is_null() {
        extras.push((sheet.addr() + player_sheet::BRAVERY, 1)); // the record’s +0x20 drops on every event with it
        check.push(1);
    }
    compare_rare_finance_event("try_finance_boost_event", fin, &extras, &check, || o(this, club.addr(), fin), || try_finance_boost_event(this, club.addr(), fin))
}

/// Comparer for `try_board_takeover`.
pub(crate) unsafe extern "thiscall" fn compare_try_board_takeover(this: usize, club: usize, fin: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_TRY_BOARD_TAKEOVER);
    if club == 0 || fin == 0 {
        return o(this, club, fin); // simplification: without a club or without finances, original only
    }
    compare_rare_finance_event("try_board_takeover", fin, &[], &[], || o(this, club, fin), || try_board_takeover(this, club, fin))
}

/// Comparer for `pay_other_costs`.
///
/// [pt] Comparador da 0x5999e0: lado a lado no registro de finanças (0x167 bytes).
pub(crate) unsafe extern "thiscall" fn compare_pay_other_costs(this: usize, value: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32)>(IDX_PAY_OTHER_COSTS);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(this, 0x167)], || o(this, value), || pay_other_costs(this, value));
    SCOREBOARD.record(igual, || format!("pay_other_costs(valor {}) original={:x?} rust={:x?}", value, mo, mr));
}

/// Comparer for `update_wage_bill`.
///
/// [pt] Comparador: lado a lado na folha (+0xc), no maior salário (+0x10), no teto (+0x14) e no último contrato consultado
/// ([0xae1080], que o salário padrão 0x864550 grava); as outras chamadas (contratos, teto 0x5968d0) só leem.
pub(crate) unsafe extern "thiscall" fn compare_update_wage_bill(this: usize) {
    let this = Finance(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_UPDATE_WAGE_BILL);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&[(this.addr() + finance::WAGE_BILL, 12), (G_CONTRACT_LOOKUP_RESULT, 4)], || o(this.addr()), || update_wage_bill(this.addr()));
    SCOREBOARD.record(igual, || format!("update_wage_bill(fin {:#x}) original={:x?} rust={:x?}", this.addr(), mo, mr));
}

/// Comparer for `refresh_all_wage_bills`.
///
/// [pt] Comparador: lado a lado na tabela de finanças inteira ([this], [0xae235c] × 0x167 bytes) e em [0xae1080].
pub(crate) unsafe extern "thiscall" fn compare_refresh_all_wage_bills(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_REFRESH_ALL_WAGE_BILLS);
    let n = (read_i32(G_CLUB_COUNT).max(0) as usize).wrapping_mul(0x167);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(read_usize(this), n), (G_CONTRACT_LOOKUP_RESULT, 4)], || o(this), || refresh_all_wage_bills(this));
    SCOREBOARD.record(igual, || format!("refresh_all_wage_bills({:#x}) original={:x?} rust={:x?}", this, mo, mr));
}

/// Comparer for `finance_get_days_to_season_start`.
///
/// [pt] Comparador: sem linha da nação, só a original (a caixa); senão lado a lado na saída `passou` e na data estática da
/// 0x54cdb0 (0xae2470), com a devolução.
pub(crate) unsafe extern "thiscall" fn compare_finance_get_days_to_season_start(this: usize, club: usize, passed: *mut u32) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut u32) -> i32>(IDX_FINANCE_GET_DAYS_TO_SEASON_START);
    if (exe::key_nation_index_of_club(club) as i8) < 0 {
        return o(this, club, passed);
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(passed as usize, 4), (G_DAYS_SINCE_SCRATCH_DATE, 8)], || o(this, club, passed), || finance_get_days_to_season_start(this, club, passed));
    SCOREBOARD.record(igual && r == n, || format!("finance_get_days_to_season_start(clube {:#x}) original={} {:x?} rust={} {:x?}", club, r, mo, n, mr));
    r
}

/// Comparer for `add_players_sold`.
///
/// [pt] Comparador: lado a lado no registro de finanças (0x167 bytes).
pub(crate) unsafe extern "thiscall" fn compare_add_players_sold(this: usize, low: u32, high: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32)>(IDX_ADD_PLAYERS_SOLD);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(this, 0x167)], || o(this, low, high), || add_players_sold(this, low, high));
    SCOREBOARD.record(igual, || format!("add_players_sold(fin {:#x}, {}) original={:x?} rust={:x?}", this, i64_from_halves(low, high), mo, mr));
}

/// Comparer for `pay_players_bought`.
///
/// [pt] Comparador: lado a lado no registro de finanças (0x167 bytes).
pub(crate) unsafe extern "thiscall" fn compare_pay_players_bought(this: usize, low: u32, high: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32)>(IDX_PAY_PLAYERS_BOUGHT);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(this, 0x167)], || o(this, low, high), || pay_players_bought(this, low, high));
    SCOREBOARD.record(igual, || format!("pay_players_bought(fin {:#x}, {}) original={:x?} rust={:x?}", this, i64_from_halves(low, high), mo, mr));
}

/// Comparer for `pay_sign_on_fee`.
///
/// [pt] Comparador: lado a lado no registro de finanças (0x167 bytes).
pub(crate) unsafe extern "thiscall" fn compare_pay_sign_on_fee(this: usize, value: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32)>(IDX_PAY_SIGN_ON_FEE);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(this, 0x167)], || o(this, value), || pay_sign_on_fee(this, value));
    SCOREBOARD.record(igual, || format!("pay_sign_on_fee(fin {:#x}, {}) original={:x?} rust={:x?}", this, value, mo, mr));
}

/// Comparer for `add_prize_money`.
///
/// [pt] Comparador: lado a lado no registro de finanças (0x167 bytes).
pub(crate) unsafe extern "thiscall" fn compare_add_prize_money(this: usize, value: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32)>(IDX_ADD_PRIZE_MONEY);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(this, 0x167)], || o(this, value), || add_prize_money(this, value));
    SCOREBOARD.record(igual, || format!("add_prize_money(fin {:#x}, {}) original={:x?} rust={:x?}", this, value, mo, mr));
}

/// Comparer for `add_tv_revenue`.
///
/// [pt] Comparador: lado a lado no registro de finanças (0x167 bytes).
pub(crate) unsafe extern "thiscall" fn compare_add_tv_revenue(this: usize, value: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32)>(IDX_ADD_TV_REVENUE);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(this, 0x167)], || o(this, value), || add_tv_revenue(this, value));
    SCOREBOARD.record(igual, || format!("add_tv_revenue(fin {:#x}, {}) original={:x?} rust={:x?}", this, value, mo, mr));
}

/// Comparer for `pay_wages`.
///
/// [pt] Comparador: lado a lado no registro de finanças (0x167 bytes).
pub(crate) unsafe extern "thiscall" fn compare_pay_wages(this: usize, value: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32)>(IDX_PAY_WAGES);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(this, 0x167)], || o(this, value), || pay_wages(this, value));
    SCOREBOARD.record(igual, || format!("pay_wages(fin {:#x}, {}) original={:x?} rust={:x?}", this, value, mo, mr));
}

/// Comparer for `add_weekly_merchandising_income`.
///
/// [pt] Comparador: lado a lado no registro de finanças do clube e no estado do sorteio.
pub(crate) unsafe extern "thiscall" fn compare_add_weekly_merchandising_income(this: usize, club: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_ADD_WEEKLY_MERCHANDISING_INCOME);
    let mut regions = vec![(fin_prm_club_finance(this, Club(club)), 0x167)];
    regions.extend(crate::rng::STATE_REGIONS);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, club), || crate::rng::without_counting(|| add_weekly_merchandising_income(this, club)));
    SCOREBOARD.record(igual, || format!("add_weekly_merchandising_income(clube {:#x}) original={:x?} rust={:x?}", club, mo, mr));
}

/// Comparer for `sell_season_tickets`.
///
/// [pt] Comparador: lado a lado no registro de finanças (this), nos lugares do estádio (+0x3c) e no estado do sorteio
/// (clube nulo: só a original).
pub(crate) unsafe extern "thiscall" fn compare_sell_season_tickets(this: usize, apply: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32)>(IDX_SELL_SEASON_TICKETS);
    let club = Club(read_usize(G_CLUB_TABLE).wrapping_add(read_usize(this + 8).wrapping_mul(club::SIZE)));
    if club.is_null() {
        return o(this, apply);
    }
    let mut regions = vec![(this, 0x167)];
    let est = club.stadium().addr();
    if est != 0 {
        regions.push((est + stadium::STADIUM_CAPACITY, 4));
    }
    regions.extend(crate::rng::STATE_REGIONS);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, apply), || crate::rng::without_counting(|| sell_season_tickets(this, apply)));
    SCOREBOARD.record(igual, || format!("sell_season_tickets(fin {:#x}, {}) original={:x?} rust={:x?}", this, apply, mo, mr));
}

/// Comparer for `upgrade_training_facilities`.
///
/// [pt] Comparador: lado a lado no nível do centro de treino (+0x7f) e no registro de finanças do clube.
pub(crate) unsafe extern "thiscall" fn compare_upgrade_training_facilities(this: usize, club: usize) {
    let club = Club(club);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_UPGRADE_TRAINING_FACILITIES);
    let regions = [(club.addr() + club::TRAINING, 1), (fin_prm_club_finance(this, club), 0x167)];
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, club.addr()), || upgrade_training_facilities(this, club.addr()));
    SCOREBOARD.record(igual, || format!("upgrade_training_facilities(clube {:#x}) original={:x?} rust={:x?}", club.addr(), mo, mr));
}

/// Comparer for `process_club_finance_events`.
pub(crate) unsafe extern "thiscall" fn compare_process_club_finance_events(this: usize, club: usize, week: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_PROCESS_CLUB_FINANCE_EVENTS)(this, club, week)
}

/// Comparer for `try_board_debt_payment`.
pub(crate) unsafe extern "thiscall" fn compare_try_board_debt_payment(this: usize, club: usize, fin: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_TRY_BOARD_DEBT_PAYMENT);
    compare_finance_event("try_board_debt_payment", fin, || o(this, club, fin), || try_board_debt_payment(this, club, fin))
}

/// Comparer for `try_cash_injection`.
pub(crate) unsafe extern "thiscall" fn compare_try_cash_injection(this: usize, club: usize, fin: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_TRY_CASH_INJECTION);
    compare_finance_event("try_cash_injection", fin, || o(this, club, fin), || try_cash_injection(this, club, fin))
}

/// Comparer for `get_max_staff_wage`.
///
/// [pt] Comparador: pelo valor.
pub(crate) unsafe extern "thiscall" fn compare_get_max_staff_wage(this: usize, job: i8) -> i32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, i8) -> i32>(IDX_GET_MAX_STAFF_WAGE)(this, job);
    let n = get_max_staff_wage(this, job);
    SCOREBOARD.record(r == n, || format!("get_max_staff_wage(fin {:#x}, cargo {}) original={} rust={}", this, job, r, n));
    r
}

/// Comparer for `pay_bonuses`.
///
/// [pt] Comparador: lado a lado no registro de finanças (0x167 bytes).
pub(crate) unsafe extern "thiscall" fn compare_pay_bonuses(this: usize, value: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32)>(IDX_PAY_BONUSES);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(this, 0x167)], || o(this, value), || pay_bonuses(this, value));
    SCOREBOARD.record(igual, || format!("pay_bonuses(fin {:#x}, {}) original={:x?} rust={:x?}", this, value, mo, mr));
}

/// Comparador: lado a lado no teto salarial (+0x14).
pub(crate) unsafe extern "thiscall" fn compare_adjust_max_wage(this: usize, write_percent_pair: i8) {
    let this = Finance(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8)>(IDX_ADJUST_MAX_WAGE);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(this.addr() + finance::WAGE_CEILING, 4)], || o(this.addr(), write_percent_pair), || adjust_max_wage(this.addr(), write_percent_pair));
    SCOREBOARD.record(igual, || format!("adjust_max_wage(fin {:#x}, {}) original={:x?} rust={:x?}", this.addr(), write_percent_pair, mo, mr));
}

/// Comparer for `get_wealth_band`.
///
/// [pt] Comparador: pelo valor.
pub(crate) unsafe extern "thiscall" fn compare_get_wealth_band(this: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u8>(IDX_GET_WEALTH_BAND)(this);
    let n = get_wealth_band(this);
    SCOREBOARD.record(r == n, || format!("get_wealth_band(financas {:#x}) original={} rust={}", this, r as i8, n as i8));
    r
}
