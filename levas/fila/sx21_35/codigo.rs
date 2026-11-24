// ---------- sx21_35: a transfer offer: resetting it as a cash bid (transfer_offer.cpp) ----------

/// Resets an offer as a cash bid of `fee` by `club` for `player`; between clubs of the same division, a famous player
/// may hear of it (one chance in 20).
///
/// `TRANSFER_OFFER::init_transfer_offer(player, club, fee)`. Without a player or a club: the transfer_offer.cpp line
/// 0x4b1 box and nothing else. `offer::PLAYER_AWARE`: 1 when the player's club (the club of his contract, kept in
/// `G_CONTRACT_LOOKUP_RESULT`) is in the bidding club's `division()`, `exe::utils_random(20)` draws 0 and his
/// `current_reputation()` is above 7000; otherwise 0. Then: no lists (+0, `offer::EXCHANGE_PLAYERS`), the buyer
/// (`offer::BUYER_CLUB_ID`) and target (`offer::TARGET_STAFF_ID`) ids, `offer::APPEARANCE_GAMES` and
/// `offer::INTERNATIONAL_GAMES` 10, `offer::JOB_TYPE` 0xb, `offer::NEGOTIATION_INDEX` and `offer::SERIAL` -1,
/// `offer::FEE`, the low nibble of `offer::COMPLETION_TIMING` cleared, and `offer::OFFER_TYPE`, both clause amounts,
/// `offer::SELL_ON_PERCENT`, `offer::INSTALMENTS`, `offer::STATUS`, `offer::TERMS_STATE`, `offer::RESPONSE_ROUND` and
/// `offer::PRE_CONTRACT` at 0.
/// Original: `TRANSFER_OFFER::init_transfer_offer` (0x8e78f0, transfer_offer.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn init_transfer_offer(this: usize, player: usize, club: usize, fee: u32) {
    if player == 0 || club == 0 {
        show_error_at(0xa9e318, 0x4b1);
        return;
    }
    let o = Offer(this);
    let seller = game_club_of_contract(game_get_contract_and_store(player));
    let mut aware = 0;
    if seller.addr() != 0 && game_club_of_contract(game_get_contract_and_store(player)).division().addr() == Club(club).division().addr()
        && exe::utils_random(0x14) == 0 && Staff(player).player().current_reputation() > 0x1b58 { // sx21_35: a famous player
        covered!("sx21_35 the player hears of the bid");
        aware = 1;
    }
    o.set_player_aware(aware);
    write_u32(this, 0); // the rival offer list
    write_u32(this + offer::EXCHANGE_PLAYERS, 0);
    o.set_buyer_club_id(read_usize(club));
    o.set_target_staff_id(read_usize(player));
    o.set_appearance_games(0xa);
    o.set_international_games(0xa); // sx21_35: ten games for each clause
    o.set_job_type(0xb);
    o.set_negotiation_index(usize::MAX);
    o.set_serial(usize::MAX);
    o.set_fee(fee as usize);
    o.set_offer_type(0);
    o.set_appearance_fee(0);
    o.set_international_fee(0);
    o.set_sell_on_percent(0);
    o.set_instalments(0);
    o.set_status(0);
    o.set_terms_state(0);
    o.set_response_round(0);
    o.set_pre_contract(0);
    o.set_completion_timing(o.completion_timing() & 0xf0);
}

/// Comparer for `init_transfer_offer`: side by side over the 0x41-byte offer, `G_CONTRACT_LOOKUP_RESULT` and the random
/// generator.
pub(crate) unsafe extern "thiscall" fn compare_init_transfer_offer(this: usize, player: usize, club: usize, fee: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(%%I0%%);
    let mut regions = vec![(this, 0x41usize), (G_CONTRACT_LOOKUP_RESULT, 4)];
    regions.extend_from_slice(&crate::rng::STATE_REGIONS);
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, player, club, fee), || init_transfer_offer(this, player, club, fee));
    SCOREBOARD.record(equal, || format!("init_transfer_offer(offer {:#x}, player {:#x}, club {:#x}, fee {}) original={:x?} rust={:x?}", this, player, club, fee, mo, mr));
}
