// ---------- sx21_2: the base transfer rules: who may not play a match, the days to the window's close and the destructor (transfer_offer.cpp) ----------

/// Why a player may not play a match under the base rules, or 0: a contract of type 5 (3) and the loan restrictions
/// against the clubs of the match (10 and 11).
///
/// `TRANSFER_RULES::get_ineligibility_code(player, fixture)`, the vtable slot
/// `transfer_rules_vtable::GET_INELIGIBILITY_CODE`, and the base answer of the countries' own versions. Unless
/// `transfer_rules::SKIP_CONTRACT_TYPE_5_INELIGIBILITY` is set, a match that is neither reserve nor minor
/// (`exe::comp_is_reserve_or_minor_comp` on `G_RESERVE_COMP`) nor friendly (`exe::friendly_is_friendly_competition` on
/// `G_FRIENDLY`), with the fixture's `competition()`: a player whose contract (`exe::contract_manager_get_contract`)
/// has type 5 (`contract_type()` & 0x3f) gives 3. Then a player on loan (`exe::contract_manager_get_loan_contract`):
/// not in a national team's match (`exe::database_is_national_team` of the home club), and only when his current club
/// (`club_job()`) plays the match. When the loan's byte +0x1c is 0 [inference: may play in every competition], a
/// competition that is neither of the club's divisions (`division()`, `last_division()`) nor friendly nor reserve or
/// minor gives 10. When the loan's byte +0x1d is 0 [inference: may play against his own club], a match against the
/// club that owns him (the club of his contract, kept in `G_CONTRACT_LOOKUP_RESULT`) gives 11. Otherwise 0.
/// Original: `TRANSFER_RULES::get_ineligibility_code` (0x8e8a60, transfer_offer.cpp, thiscall, ret 8, al).
pub(crate) unsafe extern "thiscall" fn transfer_rules_get_ineligibility_code(this: usize, player: usize, fixture: usize) -> u32 {
    let game = Fixture(fixture);
    let comp = || game.competition().addr();
    let reserve = |c: usize| exe::comp_is_reserve_or_minor_comp(read_usize(G_RESERVE_COMP), c) != 0;
    let friendly = |c: usize| exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), c) != 0;
    let contract = || exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), player);
    if read_u32(this + transfer_rules::SKIP_CONTRACT_TYPE_5_INELIGIBILITY) == 0 && !reserve(comp()) && !friendly(comp())
        && contract() != 0 && Contract(contract()).contract_type() & 0x3f == 5 {
        covered!("sx21_2 contract type 5");
        return 3; // sx21_2: contract type 5
    }
    let lc = exe::contract_manager_get_loan_contract(read_usize(G_CONTRACT_MANAGER), player);
    if lc == 0 || exe::database_is_national_team(game.home_club().addr()) != 0 {
        return 0;
    }
    let club = Staff(player).club_job();
    if club.addr() != game.home_club().addr() && club.addr() != game.away_club().addr() {
        return 0;
    }
    if read_u8(lc + 0x1c) == 0 { // the loan may not play in other competitions [inference]
        let c = comp();
        if c != club.division().addr() && c != club.last_division().addr() && !friendly(c) && !reserve(comp()) {
            covered!("sx21_2 loan outside the club's leagues");
            return 0xa;
        }
    }
    if read_u8(lc + 0x1d) != 0 { // the loan may play against his own club [inference]
        return 0;
    }
    if game.home_club().addr() == game_club_of_contract(game_get_contract_and_store(player)).addr()
        || game.away_club().addr() == game_club_of_contract(game_get_contract_and_store(player)).addr() {
        covered!("sx21_2 loan against his own club");
        return 0xb; // sx21_2: against the owning club
    }
    0
}

/// Comparer for `transfer_rules_get_ineligibility_code`: the code (al) and `G_CONTRACT_LOOKUP_RESULT`, side by side.
pub(crate) unsafe extern "thiscall" fn compare_transfer_rules_get_ineligibility_code(this: usize, player: usize, fixture: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I0%%);
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&[(G_CONTRACT_LOOKUP_RESULT, 4)], || o(this, player, fixture), || transfer_rules_get_ineligibility_code(this, player, fixture));
    SCOREBOARD.record(equal && ro as u8 == rr as u8, || format!("transfer_rules_get_ineligibility_code(player {:#x}, fixture {:#x}) original={} {:x?} rust={} {:x?}", player, fixture, ro as u8, mo, rr as u8, mr));
    ro
}

/// Days from today to the close of the transfer window open today, or 365 when none is open.
///
/// `TRANSFER_RULES::get_days_until_window_closes()`, from the transfer window news of the transfer manager. The first
/// window of the table (+4, count +8) that holds today (`exe::transfer_window_contains_date` with `G_CURRENT_DATE`):
/// its closing date (the second 6-byte date of the window, `exe::transfer_window_get_date`) in this year
/// (`G_CURRENT_DATE_YEAR`); when that date is after yesterday (`exe::date_sub_days` of today by 1, compared by year,
/// then by day of the year), the closing date of this year, otherwise of next year; the days from today to it
/// (`exe::date_days_since` on the closing date). Without an open window, 365 (in ax).
/// Original: `TRANSFER_RULES::get_days_until_window_closes` (0x8e8ed0, transfer_offer.cpp, thiscall, ret 0, ax).
pub(crate) unsafe extern "thiscall" fn transfer_rules_get_days_until_window_closes(this: usize) -> u32 {
    let n = read_u8(this + 8) as i8;
    let window = (0..(n.max(0) as usize)).map(|k| read_usize(this + 4) + k * 12).find(|&w| exe::transfer_window_contains_date(this, G_CURRENT_DATE, w) != 0);
    let Some(w) = window else {
        return 0x16d; // sx21_2: no window open: 365
    };
    let year = read_u16(G_CURRENT_DATE_YEAR) as u32;
    let (mut close, mut yesterday, mut target) = ([0u8; 8], [0u8; 8], [0u8; 8]);
    let c = exe::transfer_window_get_date(w + 6, close.as_mut_ptr() as usize, year);
    let (close_day, close_year) = (read_i16(c), read_u16(c + 2));
    let y = exe::date_sub_days(G_CURRENT_DATE, yesterday.as_mut_ptr() as usize, 1);
    let after = close_year > read_u16(y + 2) || (close_year == read_u16(y + 2) && close_day > read_i16(y));
    covered!("sx21_2 days to the close");
    let d = exe::transfer_window_get_date(w + 6, target.as_mut_ptr() as usize, if after { year } else { year + 1 }); // sx21_2: next year's close
    exe::date_days_since(d, read_u32(G_CURRENT_DATE), read_u32(G_CURRENT_DATE_LEAP)) as u32
}

/// Comparer for `transfer_rules_get_days_until_window_closes`: the days (ax) of both.
pub(crate) unsafe extern "thiscall" fn compare_transfer_rules_get_days_until_window_closes(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%)(this);
    let c = transfer_rules_get_days_until_window_closes(this);
    SCOREBOARD.record(r as u16 == c as u16, || format!("transfer_rules_get_days_until_window_closes(rules {:#x}) original={} rust={}", this, r as u16, c as u16));
    r
}

/// Destroys a transfer rules object: the base vtable back and the window table freed.
///
/// `TRANSFER_RULES::~TRANSFER_RULES()`: the vtable 0x970458 and, when the window table (+4) exists, `exe::crt_free` of
/// it (the pointer is left as it was).
/// Original: `TRANSFER_RULES::~TRANSFER_RULES` (0x8e8540, transfer_offer.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn destroy_transfer_rules(this: usize) {
    covered!("sx21_2 rules destroyed");
    write_u32(this, 0x970458);
    let t = read_usize(this + 4);
    if t != 0 {
        exe::crt_free(t);
    }
}

/// Comparer for `destroy_transfer_rules`: only the original (it frees memory).
pub(crate) unsafe extern "thiscall" fn compare_destroy_transfer_rules(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this)
}
