// ---------- sx21b_2: a transfer offer: building a computer bid (TRANSFER_OFFER::build_bid, transfer_offer.cpp) ----------

/// `TRANSFER_MANAGER::find_highest_offer_for_player` (thiscall(player, club), ret 8; Rust, hooked).
const FN_TRANSFER_MANAGER_FIND_HIGHEST_OFFER_FOR_PLAYER: usize = 0x8cc930;
/// `TRANSFER_OFFER::restructure_fee_into_clauses` (thiscall(player, seller, club), ret 0xc; item sx21b_1).
const FN_TRANSFER_OFFER_RESTRUCTURE_FEE_INTO_CLAUSES: usize = 0x8e8330;

/// What `build_bid` keeps between its pieces (the original's locals).
struct Bid {
    o: Offer,
    player: Staff,
    club: Club,
    /// The shortlist slot the bid is for (may be 0).
    slot: usize,
    /// The price ceiling (-1: none); the wealth scaling rewrites it, and the last check reads the rewritten one.
    max_fee: i32,
    out: *mut usize,
    /// The player's contract (`exe::contract_manager_get_contract`).
    contract: usize,
    /// The club of the player's contract, without a type-5 contract.
    seller: Club,
    /// The asking price (or the release fee).
    asking: i32,
    /// A transfer-listed player of a human club: the most the seller is pushed down to (-1: none).
    asking_cap: i32,
    /// The asking price with the seller's terms minus without them; later the highest rival offer's worth.
    diff: i32,
    /// The buyer's valuation of the player (`exe::shortlist_manager_value_player_for_club`).
    valuation: i32,
    /// The seller in financial crisis was weighed (no rival offer check).
    crisis_weighed: bool,
    /// Clauses were set.
    clauses_set: bool,
}

/// Where a piece of `build_bid` leaves it.
enum BidEnd {
    /// The bid is made (the last checks run).
    Settle,
    /// No bid: the contract offer (`*out`) is freed and cleared, 0.
    Drop,
    /// No bid: 0, `*out` left as it is.
    Fail,
}

impl Bid {
    unsafe fn player_slot(&self) -> bool {
        self.slot != 0 && read_u8(self.slot) == 0xb
    }
    unsafe fn slot_type(&self) -> u8 {
        ShortlistSlot(self.slot).flags() & 0x1e
    }
}

/// `(utils_random(n) + add) * mul` in a byte, as the original's `inc al` / `imul cl`.
unsafe fn bid_draw(n: i32, add: u8, mul: u8) -> i8 {
    (exe::utils_random(n) as u8).wrapping_add(add).wrapping_mul(mul) as i8
}

/// A games clause worth `r` percent of the fee, grown by the games it waits for: `r * fee / (100 - games * 2/3)`.
unsafe fn bid_clause_amount(r: i32, fee: i32, games: i8) -> i32 {
    ftol(r as f64 * fee as f64 / (read_f64(rdata::F64_100) - games as f64 * read_f64(rdata::F64_0_6666666667)))
}

/// The player hears of the bid: a player above `utils_random(3000) + 4000` current reputation, for a slot type under
/// 8, whose contract's club is in the bidding club's division, with one chance in `chance`.
unsafe fn bid_player_aware(b: &Bid, chance: i32) -> u8 {
    let p = b.player;
    if p.player().addr() == 0 || b.slot == 0 || b.slot_type() >= 8 {
        return 0;
    }
    let mark = exe::utils_random(0xbb8) + 0xfa0;
    if p.player().current_reputation() as i32 <= mark {
        return 0;
    }
    if game_club_of_contract(game_get_contract_and_store(p.addr())).addr() == 0 {
        return 0;
    }
    if game_club_of_contract(game_get_contract_and_store(p.addr())).division().addr() != b.club.division().addr() {
        return 0;
    }
    covered!("sx21b_2 the player may hear of the bid");
    (exe::utils_random(chance) == 0) as u8 // sx21b_2: the player hears of the bid
}

/// A human club bidding again: the offer copies the club's last bid for the player, and its exchange players are
/// checked again (still the buyer's, not on loan, players, and in no other deal than this one).
unsafe fn bid_copy_human_offer(b: &mut Bid, existing: usize) {
    let o = b.o.addr();
    for k in [0usize, 4, 8, 0xc] {
        write_u32(o + k, read_u32(existing + k));
    }
    write_u8(o + offer::TERMS_STATE, read_u8(existing + offer::TERMS_STATE));
    write_u8(o + offer::JOB_TYPE, read_u8(existing + offer::JOB_TYPE));
    let mut tmp = [0u8; 8];
    exe::date_operator_3d(o + offer::DUE_DATE, tmp.as_mut_ptr() as usize, existing + offer::DUE_DATE);
    write_u8(o + offer::DUE_SLICE, read_u8(existing + offer::DUE_SLICE));
    for k in [offer::FEE, offer::SLOT_INDEX, offer::SERIAL, offer::NEGOTIATION_INDEX] {
        write_u32(o + k, read_u32(existing + k));
    }
    for k in [offer::OFFER_TYPE, offer::STATUS, offer::PLAYER_AWARE, offer::RESPONSE_ROUND, offer::PRE_CONTRACT] {
        write_u8(o + k, read_u8(existing + k));
    }
    write_u8(o + offer::COMPLETION_TIMING, (read_u8(o + offer::COMPLETION_TIMING) & 0xf0) | (read_u8(existing + offer::COMPLETION_TIMING) & 0xf));
    write_u32(o + offer::OFFER_KIND, (read_u32(o + offer::OFFER_KIND) & !0xf) | (read_u32(existing + offer::OFFER_KIND) & 0xf));
    for k in [offer::APPEARANCE_FEE, offer::INTERNATIONAL_FEE] {
        write_u32(o + k, read_u32(existing + k));
        write_u8(o + k + 4, read_u8(existing + k + 4));
    }
    write_u8(o + offer::SELL_ON_PERCENT, read_u8(existing + offer::SELL_ON_PERCENT));
    write_u8(o + offer::INSTALMENTS, read_u8(existing + offer::INSTALMENTS));
    covered!("sx21b_2 human club bids again");
    b.o.set_player_aware(bid_player_aware(b, 3));
    b.o.set_negotiation_index(usize::MAX);
    b.o.set_serial(usize::MAX);
    b.o.set_terms_state(0);
    b.o.set_status(0);
    b.o.set_response_round(0);
    write_u32(o + offer::OFFER_KIND, read_u32(o + offer::OFFER_KIND) & !0xf);
    b.o.set_completion_timing(b.o.completion_timing() & 0xf0);
    b.o.set_pre_contract(0);
    write_u32(o, 0); // the rival offer list
    if read_u32(o + offer::EXCHANGE_PLAYERS) != 0 {
        bid_rebuild_exchange_list(b, existing);
    }
}

/// The exchange players of the copied bid, again: a new list with those still the buyer's (`club` of the contract),
/// not on loan nor on a type-5 contract, in a player job (`job_for_club()` 0xb), and either in no other offer or in a
/// single exchange offer that is for this same target; each node is copied with its byte +4 cleared. An empty list is
/// freed.
unsafe fn bid_rebuild_exchange_list(b: &mut Bid, existing: usize) {
    let o = b.o.addr();
    let mut deals = [0u8; 0x10];
    let deals_list = deals.as_mut_ptr() as usize;
    exe::new_llist(deals_list);
    let fresh = exe::crt_operator_new(0xe);
    write_usize(o + offer::EXCHANGE_PLAYERS, if fresh != 0 { exe::new_llist(fresh) } else { 0 });
    let old = Llist(read_usize(existing + offer::EXCHANGE_PLAYERS));
    old.set_cursor(old.head());
    loop {
        let d = exe::llist_get_next(old);
        if d == 0 {
            break;
        }
        let pl = staff_at(read_i32(d) as isize);
        if game_club_of_contract(game_get_contract_and_store(pl.addr())).addr() != b.club.addr() {
            continue;
        }
        if exe::contract_manager_get_loan_contract(read_usize(G_CONTRACT_MANAGER), pl) != 0 {
            continue;
        }
        let c = Contract(game_get_contract_and_store(pl.addr()));
        if !c.is_null() && c.contract_type() & 0x3f == 5 {
            continue;
        }
        if pl.job_for_club() != 0xb {
            continue;
        }
        let tm = read_usize(G_TRANSFER_MANAGER);
        if exe::transfer_manager_get_offers_for_player(tm, pl, 0usize, 0) != 0 || exe::transfer_manager_get_exchange_offers_for_player(tm, pl, deals_list) != 0 {
            let dl = Llist(deals_list);
            if dl.count() != 1 {
                continue;
            }
            dl.set_cursor(dl.head());
            let e = exe::llist_get_next(dl);
            let other = read_usize(exe::transfer_manager_get_offer_slot(tm, read_i32(e)));
            if read_u32(other + offer::TARGET_STAFF_ID) != read_u32(b.player.addr()) {
                continue;
            }
        }
        covered!("sx21b_2 exchange player kept");
        let mut node = [0u8; 7];
        core::ptr::copy_nonoverlapping(d as *const u8, node.as_mut_ptr(), 7);
        node[4] = 0;
        exe::llist_add_tail(read_usize(o + offer::EXCHANGE_PLAYERS), node.as_mut_ptr() as usize, 7);
    }
    let list = read_usize(o + offer::EXCHANGE_PLAYERS);
    if read_u16(list) == 0 {
        if list != 0 {
            exe::drop_llist(list);
            exe::crt_operator_delete(list);
        }
        write_usize(o + offer::EXCHANGE_PLAYERS, 0);
    }
    exe::drop_llist(deals_list);
}

/// A computer club takes a player at the end of his contract: a new contract offer (`STAFF_CONTRACT`, 0x50 bytes, into
/// `*out`) through `exe::contract_manager_offer_contract(player, club, contract, -1, 1.0, 2)`. Without `out`: no bid.
/// Refused: the contract offer is freed (but `*out` keeps the freed pointer: a defect of the original) and no bid.
unsafe fn bid_pre_contract(b: &mut Bid) -> BidEnd {
    if b.out.is_null() {
        return BidEnd::Fail;
    }
    let sc = exe::crt_operator_new(0x50);
    let sc = if sc != 0 { exe::new_staff_contract(sc) } else { 0 };
    *b.out = sc;
    if exe::contract_manager_offer_contract(read_usize(G_CONTRACT_MANAGER), b.player, b.club.addr() as u32, sc as u32, -1, 1.0, 2) == 0 {
        if *b.out != 0 {
            exe::crt_operator_delete(*b.out);
        }
        return BidEnd::Fail;
    }
    covered!("sx21b_2 pre-contract offered");
    b.o.set_pre_contract(1);
    BidEnd::Settle
}

/// The asking price, the asking cap and the difference of the seller's terms, for a player slot: a human seller's
/// asking price (`exe::contract_manager_get_human_asking_price`), and when he is transfer-listed the cap
/// `max(asking / 3, asking - 1500000)`; any other seller's asking price with and without his terms
/// (`exe::contract_manager_calc_asking_price(.., 0, 1, 1|0)`). Not a player slot: 0. An active release clause below the
/// asking price replaces it (and marks the offer kind 1).
unsafe fn bid_asking_price(b: &mut Bid) -> BidEnd {
    let mut asking = 0;
    if b.player_slot() {
        if b.player.player().addr() == 0 {
            return BidEnd::Fail;
        }
        let cm = read_usize(G_CONTRACT_MANAGER);
        if b.seller.addr() != 0 && exe::human_manager_is_human_club(b.seller, 0, 1, 0, 0) != 0 {
            asking = exe::contract_manager_get_human_asking_price(cm, b.seller, b.club, b.player);
            b.asking = asking;
            if exe::staff_is_transfer_listed(b.player, 0usize) != 0 {
                covered!("sx21b_2 listed by a human seller");
                b.asking_cap = core::cmp::max(asking / 3, asking.wrapping_sub(0x16e360)); // sx21b_2: a third, or 1.5 million off
            }
        } else {
            asking = exe::contract_manager_calc_asking_price(cm, b.seller, b.club, b.player, 0, 1, 1);
            b.asking = asking;
            b.diff = asking.wrapping_sub(exe::contract_manager_calc_asking_price(cm, b.seller, b.club, b.player, 0, 1, 0));
        }
    } else {
        b.asking = 0;
    }
    let c = b.contract;
    if c != 0 && exe::staff_contract_has_active_release_clause(c) != 0 && asking > Contract(c).release_fee() as i32 {
        covered!("sx21b_2 release clause below the asking price");
        b.asking = Contract(c).release_fee() as i32;
        let kind = b.o.offer_kind() as u32;
        write_u32(b.o.addr() + offer::OFFER_KIND, (kind & 0xffff_fff1) | 1);
    }
    BidEnd::Settle
}

/// The buyer's valuation (`exe::shortlist_manager_value_player_for_club(club, player, slot, &_, -1, 1)`, player slots
/// only), cut for a seller in financial crisis (`finance::CRISIS_STATE` 1): to the asking price when it is below both
/// (the valuation and the player's worth: `exe::contract_manager_calc_player_value` for a human seller, `staff::VALUE`
/// otherwise), else to that worth when the valuation reaches it.
unsafe fn bid_valuation(b: &mut Bid) {
    b.valuation = 0;
    if !b.player_slot() {
        return;
    }
    let sm = read_usize(read_usize(G_TRANSFER_MANAGER) + transfer_manager_fields::SHORTLIST_MANAGER);
    let mut scratch = 0i32;
    b.valuation = exe::shortlist_manager_value_player_for_club(sm, b.club, b.player, b.slot, &mut scratch, -1, 1);
    let seller = b.seller;
    if seller.addr() == 0 {
        return;
    }
    let ft = read_usize(G_FINANCE_TABLE);
    if exe::financetable_get_club_finance(ft, seller) == 0 || Finance(exe::financetable_get_club_finance(ft, seller)).crisis_state() != 1 {
        return;
    }
    covered!("sx21b_2 seller in crisis");
    let cm = read_usize(G_CONTRACT_MANAGER);
    let worth = || exe::contract_manager_calc_player_value(cm, b.player, seller, 0, -1, 0);
    if exe::human_manager_is_human_club(seller, 0, 1, 0, 0) != 0 {
        let w = worth();
        let least = if b.valuation < w { b.valuation } else { worth() };
        if b.asking < least {
            b.valuation = b.asking;
        } else if b.valuation >= worth() {
            b.valuation = worth();
        }
    } else {
        let value = b.player.value() as i32;
        let least = if b.valuation < value { b.valuation } else { value };
        if b.asking < least {
            b.valuation = b.asking;
        } else if b.valuation >= value {
            b.valuation = value;
        }
    }
    b.crisis_weighed = true;
}

/// A rich buyer and a poor seller spread a big fee (above `utils_random(1000000)`): the factor starts at 2.0 (0.75 for
/// an English or Scottish buyer), times the buyer's band (-2: 2.5, -1: 1.5, 0: 0.8, 1: 0.4, 2: 0.1) and the seller's
/// (-2: 0.1, -1: 0.4, 0: 0.8, 1: 1.5, 2: 2.5); above 3.0: 24 instalments and the valuation, the fee and the ceiling
/// times 5/3; above 2.4: 18 and 10/7; above 1.6: 12 and 1.25; above 1.0: 6 and 10/9.
unsafe fn bid_spread_fee(b: &mut Bid) {
    if b.seller.addr() == 0 || b.o.fee() as i32 <= exe::utils_random(0xf4240) {
        return;
    }
    let nation = read_u32(b.club.nation().addr());
    covered!("sx21b_2 big fee spread");
    let mut factor: f64 = if nation == read_u32(KEY_NATION_ENGLAND) || nation == read_u32(KEY_NATION_SCOTLAND) { 0.75 } else { 2.0 }; // sx21b_2: British buyers spread less
    let ft = read_usize(G_FINANCE_TABLE);
    let buyer_band = exe::finance_get_wealth_band(exe::financetable_get_club_finance(ft, b.club)) as i8;
    let k = match buyer_band {
        -2 => Some(rdata::F64_2_5),
        -1 => Some(rdata::F64_1_5),
        0 => Some(rdata::F64_0_8),
        1 => Some(rdata::F64_0_4),
        2 => Some(rdata::F64_0_1),
        _ => None,
    };
    if let Some(k) = k {
        factor *= read_f64(k);
    }
    let seller_band = exe::finance_get_wealth_band(exe::financetable_get_club_finance(ft, b.seller)) as i8;
    let k = match seller_band {
        -2 => Some(rdata::F64_0_1),
        -1 => Some(rdata::F64_0_4),
        0 => Some(rdata::F64_0_8),
        1 => Some(rdata::F64_1_5),
        2 => Some(rdata::F64_2_5),
        _ => None,
    };
    if let Some(k) = k {
        factor *= read_f64(k);
    }
    let (instalments, scale) = if factor > read_f64(rdata::F64_3) {
        covered!("sx21b_2 24 instalments");
        (0x18, rdata::F64_1_666666667) // sx21b_2: 24 instalments
    } else if factor > read_f64(0x970448) { // 2.4
        (0x12, 0x970440) // 18 instalments, 10/7
    } else if factor > read_f64(rdata::F64_1_6) {
        (0xc, rdata::F64_1_25)
    } else if factor > read_f64(rdata::F64_1) {
        (6, rdata::F64_1_111111111)
    } else {
        return;
    };
    b.o.set_instalments(instalments);
    let s = read_f64(scale);
    b.valuation = ftol(b.valuation as f64 * s);
    b.o.set_fee(ftol(b.o.fee() as i32 as f64 * s) as usize);
    b.max_fee = ftol(b.max_fee as f64 * s);
}

/// The international clause of a capped-to-be player: a nation above 5000 reputation, fewer than 5 caps, and either a
/// home reputation that makes him a candidate (`nation reputation < home reputation * 10/13`) or a national team job;
/// under `utils_random(4) + 25` years: one cap and 5 to 10% of the fee for an uncapped player one time in 5, else 10
/// caps and 5 to 15%.
unsafe fn bid_international_clause(b: &mut Bid) {
    let p = b.player;
    let nation = p.nation();
    let reputation = nation.reputation();
    if reputation <= 0x1388 || p.int_apps() >= 5 {
        return;
    }
    if !((reputation as f64) < p.player().home_reputation() as f64 * read_f64(0x970438)) { // 10/13
        let job = p.national_job();
        if job.addr() == 0 || exe::club_is_national_team(job) == 0 {
            return;
        }
    }
    if p.age() as i8 as i32 >= exe::utils_random(4) + 0x19 {
        return;
    }
    b.clauses_set = true;
    let fee = b.o.fee() as i32;
    if p.int_apps() == 0 && exe::utils_random(5) == 0 {
        covered!("sx21b_2 one-cap clause");
        let r = exe::utils_random(6) + 5;
        b.o.set_international_fee(ftol(r as f64 * fee as f64 * read_f64(rdata::F64_0_01)) as usize);
        b.o.set_international_games(1);
    } else {
        let r = exe::utils_random(0xb) + 5;
        b.o.set_international_fee(ftol(r as f64 * fee as f64 * read_f64(rdata::F64_0_01)) as usize);
        b.o.set_international_games(0xa); // sx21b_2: ten caps
    }
}

/// The clauses of a bid cut down to the ceiling while the asking price stays within reach: the international clause;
/// an appearance clause when the international one is below `utils_random(fee / 8)`, for a slot of type 0xa, one time
/// in 2; a sell-on share (5 to 25%, the lower of two draws) when both clauses are below another such draw, for a slot
/// of type 0xa or a player under `28 - utils_random(20)`; with none of the three, an appearance clause when a draw on
/// the slot's price beats one on 67% of its valuation.
unsafe fn bid_clauses_within_reach(b: &mut Bid) {
    bid_international_clause(b);
    let fee = b.o.fee() as i32;
    if (b.o.international_fee() as i32) < exe::utils_random(fee / 8) && b.slot_type() == 0xa && exe::utils_random(2) != 0 {
        covered!("sx21b_2 appearance clause within reach");
        b.clauses_set = true;
        let games = bid_draw(5, 1, 10);
        b.o.set_appearance_games(games as u8);
        let r = exe::utils_random(0xb) + 5;
        b.o.set_appearance_fee(bid_clause_amount(r, fee, games) as usize);
    }
    let clauses = (b.o.appearance_fee() as i32).wrapping_add(b.o.international_fee() as i32);
    if clauses < exe::utils_random(fee / 8) && (b.slot_type() == 0xa || (b.player.age() as i8 as i32) < 0x1c - exe::utils_random(0x14)) {
        covered!("sx21b_2 sell-on share within reach");
        b.clauses_set = true;
        let first = bid_draw(5, 1, 5);
        let second = bid_draw(5, 1, 5);
        b.o.set_sell_on_percent(core::cmp::min(first, second) as u8);
    }
    if b.o.international_fee() != 0 || b.o.appearance_fee() != 0 || b.o.sell_on_percent() != 0 {
        return;
    }
    let s = ShortlistSlot(b.slot);
    let on_valuation = exe::utils_random(ftol(s.valuation() as f64 * read_f64(rdata::F64_0_67)));
    if exe::utils_random(s.price()) > on_valuation {
        covered!("sx21b_2 appearance clause on the slot's price");
        b.clauses_set = true;
        let games = bid_draw(5, 1, 10);
        b.o.set_appearance_games(games as u8);
        let r = exe::utils_random(0xb) + 5;
        b.o.set_appearance_fee(bid_clause_amount(r, fee, games) as usize);
    }
}

/// The share picked from two draws: the higher when 75% of the asking price is below `against`, else the lower.
unsafe fn bid_pick_share(b: &Bid, first: i8, second: i8, against: f64) -> (i8, bool) {
    let higher = b.asking as f64 * read_f64(rdata::F64_0_75) < against;
    (if higher { core::cmp::max(first, second) } else { core::cmp::min(first, second) }, higher)
}

/// The appearance clause cut to `games * 30000 + asking`, or to `clause * diff / max(asking, 10000)` when lower.
unsafe fn bid_cap_appearance(b: &Bid) {
    let floor = core::cmp::max(b.asking, 0x2710);
    let games = b.o.appearance_games() as i8 as i32;
    let by_games = games.wrapping_mul(30000).wrapping_add(b.asking);
    let by_terms = (b.o.appearance_fee() as i32).wrapping_mul(b.diff) / floor;
    b.o.set_appearance_fee(core::cmp::min(by_games, by_terms) as usize);
}

/// The asking price out of reach of the ceiling (1.4 times the ceiling below it, or 1.2 times with the seller's terms
/// mattering): for a player slot of type 0xa, of type 8 for a player under 28, or with terms that matter, the bid
/// becomes the ceiling with clauses: a candidate international (as in `bid_international_clause`, without the age
/// draw): an international clause of the larger of 75% of the fee and 85% of the terms' difference, an appearance
/// clause after 30 to 50 games (5 to 15%) and a sell-on share (0 to 25%); an uncapped player otherwise: an appearance
/// clause after 10 to 50 games (10 to 20%) and a share (0 to 25%); any other player: an appearance clause after 30 to
/// 50 games (5 to 15%) and a share (5 to 25%). The share is the higher draw when 75% of the asking price is below the
/// terms' difference (then the appearance clause is capped, `bid_cap_appearance`, except for the last case).
/// Defect of the original: in the candidate international's case the difference is read as a double over the two
/// dwords of the difference and the asking price (a stale stack offset), not as the difference.
unsafe fn bid_clauses_out_of_reach(b: &mut Bid) -> BidEnd {
    if !b.player_slot() {
        return BidEnd::Fail;
    }
    let t = b.slot_type();
    if !(t == 0xa || (t == 8 && (b.player.age() as i8) < 0x1c) || b.diff != 0) {
        return BidEnd::Fail;
    }
    b.clauses_set = true;
    let p = b.player;
    let fee = b.o.fee() as i32;
    let reputation = p.nation().reputation();
    if reputation > 0x1388 && p.int_apps() == 0 {
        let job = p.national_job();
        let candidate = (reputation as f64) < p.player().home_reputation() as f64 * read_f64(0x970438)
            || (job.addr() != 0 && exe::club_is_national_team(job) != 0);
        if candidate {
            covered!("sx21b_2 out of reach: candidate international");
            let by_fee = fee as f64 * read_f64(rdata::F64_0_75);
            let by_terms = b.diff as f64 * read_f64(rdata::F64_0_85);
            b.o.set_international_fee(ftol(if by_fee > by_terms { by_fee } else { by_terms }) as usize);
            b.o.set_international_games(0xa);
            let games = bid_draw(3, 3, 10);
            b.o.set_appearance_games(games as u8);
            let r = exe::utils_random(0xb) + 5;
            b.o.set_appearance_fee(bid_clause_amount(r, fee, games) as usize);
            let first = exe::utils_random(6).wrapping_mul(5) as i8;
            let second = exe::utils_random(6).wrapping_mul(5) as i8;
            let stale = f64::from_bits(((b.asking as u32 as u64) << 32) | b.diff as u32 as u64);
            let (share, higher) = bid_pick_share(b, first, second, stale);
            b.o.set_sell_on_percent(share as u8);
            if higher {
                bid_cap_appearance(b);
            }
        } else {
            covered!("sx21b_2 out of reach: uncapped player");
            let games = bid_draw(5, 1, 10);
            b.o.set_appearance_games(games as u8);
            let r = exe::utils_random(0xb) + 0xa;
            b.o.set_appearance_fee(bid_clause_amount(r, fee, games) as usize);
            let first = exe::utils_random(6).wrapping_mul(5) as i8;
            let second = exe::utils_random(6).wrapping_mul(5) as i8;
            let (share, higher) = bid_pick_share(b, first, second, b.diff as f64);
            b.o.set_sell_on_percent(share as u8); // sx21b_2: the uncapped player's share
            if higher {
                bid_cap_appearance(b);
            }
        }
    } else {
        covered!("sx21b_2 out of reach: other player");
        let games = bid_draw(3, 3, 10);
        b.o.set_appearance_games(games as u8);
        let r = exe::utils_random(0xb) + 5;
        b.o.set_appearance_fee(bid_clause_amount(r, fee, games) as usize);
        let first = bid_draw(5, 1, 5);
        let second = bid_draw(5, 1, 5);
        let (share, _) = bid_pick_share(b, first, second, b.diff as f64);
        b.o.set_sell_on_percent(share as u8);
    }
    BidEnd::Settle
}

/// The fee against the ceiling: a ceiling below the fee (not -1) wins. With an asking cap, the fee is the lower of the
/// cap and the fee, without instalments, and at most the ceiling. Otherwise the fee is the ceiling and the clauses
/// make up for it: out of reach (`bid_clauses_out_of_reach`) when 1.4 times the ceiling is below the asking price or
/// 1.2 times is below it with terms that matter; within reach (`bid_clauses_within_reach`) from a ceiling of 100000
/// with 1.2 times still below the asking price, for a player slot and a player with a nation.
unsafe fn bid_fee_under_ceiling(b: &mut Bid) -> BidEnd {
    let ceiling = b.max_fee;
    if ceiling <= -1 || ceiling >= b.o.fee() as i32 {
        return BidEnd::Settle;
    }
    if b.asking_cap > -1 {
        covered!("sx21b_2 asking cap under the ceiling");
        b.o.set_fee(core::cmp::min(b.asking_cap, b.o.fee() as i32) as usize);
        b.o.set_instalments(0);
        if b.o.fee() as i32 > ceiling {
            b.o.set_fee(ceiling as usize);
        }
        return BidEnd::Settle;
    }
    let asking = b.asking as f64;
    b.o.set_fee(ceiling as usize);
    if ceiling as f64 * read_f64(rdata::F64_1_4) < asking {
        return bid_clauses_out_of_reach(b);
    }
    let near = ceiling as f64 * read_f64(rdata::F64_1_2);
    if near < asking && b.diff != 0 {
        return bid_clauses_out_of_reach(b);
    }
    if ceiling < 0x186a0 || !(near < asking) || !b.player_slot() || b.player.nation().addr() == 0 {
        return BidEnd::Settle;
    }
    covered!("sx21b_2 clauses within reach");
    bid_clauses_within_reach(b);
    BidEnd::Settle
}

/// The fee against the buyer's valuation and the highest rival bid (`find_highest_offer_for_player`, refused, and only
/// without the crisis weighing: its total worth; otherwise -1). A fee up to 1.25 times the valuation must stay above
/// 1.2 times the rival bid, or there is no bid. A valuation below 75% of the fee (and at least 40%), with no clauses
/// yet, for a player slot of type 0xa or of type 8 under 28, sets clauses and gives no bid (the next round bids with
/// them). Otherwise a valuation below the fee becomes the fee, raised toward the ceiling by the slot's price ratio
/// (with a games clause one time in 2, `bid_appearance_by_ratio`, else half of it on the fee) and at most the asking
/// price; a fee below 1.2 times the rival bid rises to it, unless that passes the valuation (no bid).
unsafe fn bid_settle_fee(b: &mut Bid) -> BidEnd {
    let tm = read_usize(G_TRANSFER_MANAGER);
    let highest: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_TRANSFER_MANAGER_FIND_HIGHEST_OFFER_FOR_PLAYER);
    let h = highest(tm, b.player.addr(), b.club.addr());
    b.diff = if h != 0 && Offer(h).status() == 2 && !b.crisis_weighed { exe::transfer_offer_get_total_value(h, 0, 0, 0, 1) } else { -1 };
    let fee = b.o.fee() as i32;
    let (f, v) = (fee as f64, b.valuation as f64);
    if b.valuation <= fee && !(f * read_f64(rdata::F64_0_8) > v) {
        return if b.diff as f64 * read_f64(rdata::F64_1_2) <= f { BidEnd::Settle } else { BidEnd::Drop };
    }
    if !(f * read_f64(rdata::F64_0_75) < v) && !b.clauses_set && b.player_slot() {
        if f * read_f64(rdata::F64_0_4) > v {
            return BidEnd::Drop;
        }
        let t = b.slot_type();
        if !(t == 0xa || (t == 8 && (b.player.age() as i8) < 0x1c)) {
            return BidEnd::Drop;
        }
        bid_clauses_for_next_round(b, fee);
        return BidEnd::Fail;
    }
    if b.valuation < fee {
        let s = ShortlistSlot(b.slot);
        if fee < b.max_fee && b.slot != 0 && s.valuation() != 0 && !b.clauses_set && read_u8(b.slot) == 0xb {
            let q = s.price() as f64 / s.valuation() as f64;
            let mut ratio = q as f32;
            if q < read_f64(rdata::F64_0_05) {
                ratio = 0.05;
            }
            if ratio as f64 > read_f64(rdata::F64_0_75) {
                ratio = 0.75;
            }
            let room = b.max_fee.wrapping_sub(b.o.fee() as i32);
            if exe::utils_random(2) != 0 {
                bid_appearance_by_ratio(b, ratio, room);
            } else {
                covered!("sx21b_2 half the room on the fee");
                b.valuation = b.valuation.wrapping_add(ftol(room as f64 * ratio as f64 * read_f32(rdata::F32_0_5) as f64));
            }
            if b.valuation > b.asking {
                b.valuation = b.asking;
            }
        }
        b.o.set_fee(b.valuation as usize);
        return BidEnd::Settle;
    }
    let rival = b.diff as f64 * read_f64(rdata::F64_1_2);
    if f < rival {
        covered!("sx21b_2 fee raised over the rival bid");
        b.o.set_fee(ftol(rival) as usize);
        if b.o.fee() as i32 > b.valuation {
            return BidEnd::Drop;
        }
    }
    BidEnd::Settle
}

/// The games clause from the slot's price ratio: `10 / ratio * 0.1` games in tens, between 20 and 100 and then in
/// tens between 10 and 50; the amount is the room to the ceiling times the ratio, at most `games * 30000`.
unsafe fn bid_appearance_by_ratio(b: &mut Bid, ratio: f32, room: i32) {
    covered!("sx21b_2 appearance clause by the price ratio");
    let g = ftol(read_f32(rdata::F32_10) as f64 / ratio as f64 * read_f32(rdata::F32_0_1000000015) as f64) as i8 as i32 * 10;
    let g = g.clamp(0x14, 0x64) as u8 as i8 as i32;
    let games = ((g / 10) * 10).clamp(0xa, 0x32);
    b.o.set_appearance_games(games as u8);
    let amount = room as f64 * ratio as f64;
    let cap = games.wrapping_mul(30000) as f32;
    b.o.set_appearance_fee(ftol(if amount < cap as f64 { amount } else { cap as f64 }) as usize);
}

/// Clauses for a fee well above the valuation (set before giving up on this round): a candidate international: 75% of
/// the fee after 10 caps, an appearance clause after 30 to 70 games (5 to 15%) and a share (0 to 25%, the lower of two
/// draws); an uncapped player: an appearance clause after 10 to 50 games (10 to 20%) and a share (0 to 25%); any other
/// player: an appearance clause after 30 to 50 games (5 to 15%) and a share (5 to 25%).
unsafe fn bid_clauses_for_next_round(b: &mut Bid, fee: i32) {
    let p = b.player;
    let reputation = p.nation().reputation();
    let (games, r, first, second);
    if reputation > 0x1388 && p.int_apps() == 0 {
        let job = p.national_job();
        if (reputation as f64) < p.player().home_reputation() as f64 * read_f64(0x970438) || (job.addr() != 0 && exe::club_is_national_team(job) != 0) {
            covered!("sx21b_2 next round: international clause");
            b.o.set_international_fee(ftol(fee as f64 * read_f64(rdata::F64_0_75)) as usize);
            b.o.set_international_games(0xa);
            games = bid_draw(5, 3, 10);
            b.o.set_appearance_games(games as u8);
            r = exe::utils_random(0xb) + 5;
        } else {
            games = bid_draw(5, 1, 10);
            b.o.set_appearance_games(games as u8);
            r = exe::utils_random(0xb) + 0xa;
        }
        b.o.set_appearance_fee(bid_clause_amount(r, fee, games) as usize);
        first = exe::utils_random(6).wrapping_mul(5) as i8;
        second = exe::utils_random(6).wrapping_mul(5) as i8;
    } else {
        covered!("sx21b_2 next round: other player");
        games = bid_draw(3, 3, 10);
        b.o.set_appearance_games(games as u8);
        r = exe::utils_random(0xb) + 5;
        b.o.set_appearance_fee(bid_clause_amount(r, fee, games) as usize);
        first = bid_draw(5, 1, 5);
        second = bid_draw(5, 1, 5);
    }
    b.o.set_sell_on_percent(core::cmp::min(first, second) as u8);
}

/// The money: the buyer's transfer budget must cover the fee (or no bid). An active release clause below 1.05 times
/// the fee settles it: no clauses and the release fee. Otherwise, for no slot or a player slot,
/// `restructure_fee_into_clauses(player, seller, club)`.
unsafe fn bid_afford(b: &mut Bid) -> BidEnd {
    let fin = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), b.club);
    if exe::finance_get_transfer_budget(fin, b.player.addr() as u32, 0, 0usize, 1, 0) < b.o.fee() as i32 as i64 {
        covered!("sx21b_2 the buyer cannot afford it");
        return BidEnd::Fail;
    }
    let c = b.contract;
    if c != 0 && exe::staff_contract_has_active_release_clause(c) != 0 {
        if (Contract(c).release_fee() as i32 as f64) < b.o.fee() as i32 as f64 * read_f64(rdata::F64_1_05) {
            covered!("sx21b_2 the release fee is paid");
            b.o.set_appearance_fee(0);
            b.o.set_international_fee(0);
            b.o.set_sell_on_percent(0);
            b.o.set_fee(Contract(c).release_fee());
        }
    } else if b.slot == 0 || read_u8(b.slot) == 0xb {
        let restructure: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(FN_TRANSFER_OFFER_RESTRUCTURE_FEE_INTO_CLAUSES);
        restructure(b.o.addr(), b.player.addr(), b.seller.addr(), b.club.addr());
    }
    BidEnd::Settle
}

/// The last checks of a bid: the ceiling (5000 for a ceiling under 5000 and a player with a value); when the move
/// happens (`offer::COMPLETION_TIMING`: 3 at the contract's end for a pre-contract, 5 with a compensation fee when the
/// FIFA protected period does not apply); `exe::transfer_manager_can_sign_player` (a refusal other than 9: no bid);
/// the timing 0 for a signable player without a contract offer, otherwise 1 for timing 0 and 4 for timing 3; no club
/// of the contract makes it a pre-contract, which pays the compensation fee unless the timing is 5.
unsafe fn bid_finish(b: &mut Bid) -> u32 {
    let ceiling = b.max_fee;
    if ceiling != -1 {
        let limit = if ceiling < 0x1388 && b.player.value() as i32 > 0 { 0x1388 } else { ceiling };
        if limit > -1 && b.o.fee() as i32 > limit {
            return bid_drop(b);
        }
    }
    let mut date = [read_u32(G_CURRENT_DATE), read_u32(G_CURRENT_DATE + 4)];
    let date_addr = date.as_mut_ptr() as usize;
    let out = b.out;
    let tm = read_usize(G_TRANSFER_MANAGER);
    let has_offer = !out.is_null() && *out != 0;
    let player_job = || exe::database_is_player_job(b.player.job_for_club() as i8) != 0;
    if has_offer && player_job() && b.contract != 0 {
        covered!("sx21b_2 pre-contract timing");
        b.o.set_completion_timing((b.o.completion_timing() & 0xf3) | 3);
        let mut tmp = [0u8; 8];
        exe::date_operator_3d(date_addr, tmp.as_mut_ptr() as usize, b.contract + contract::CONTRACT_EXPIRES);
        if exe::staff_contract_check_fifa_protected_period(b.contract, b.club, 0usize, 0usize) == 0 {
            b.o.set_fee(exe::transfer_manager_calc_compensation_fee(tm, b.player, b.club, 0) as usize);
            b.o.set_completion_timing((b.o.completion_timing() & 0xf5) | 5);
        }
    }
    let seller = contract_club_of(b.player.addr());
    let mut refusal = 0u16;
    let ok = exe::transfer_manager_can_sign_player(tm, b.club, date_addr, b.player, seller, 1, &mut refusal);
    if ok == 0 && refusal != 9 {
        covered!("sx21b_2 the club cannot sign him");
        if !out.is_null() {
            if *out != 0 {
                exe::crt_operator_delete(*out);
            }
            *out = 0;
        }
        return 0;
    }
    let has_offer = !out.is_null() && *out != 0;
    if !has_offer && player_job() && ok != 0 {
        b.o.set_completion_timing(b.o.completion_timing() & 0xf0);
    } else if player_job() {
        let t = b.o.completion_timing();
        if t & 0xf == 0 {
            b.o.set_completion_timing((t & 0xf1) | 1);
        } else if t & 0xf == 3 {
            b.o.set_completion_timing((t & 0xf4) | 4);
        }
    }
    if contract_club_of(b.player.addr()) == 0 {
        covered!("sx21b_2 no club: pre-contract");
        b.o.set_pre_contract(1);
    }
    if b.o.pre_contract() != 0 && b.o.completion_timing() & 0xf != 5 {
        b.o.set_fee(exe::transfer_manager_calc_compensation_fee(tm, b.player, b.club, 1) as usize);
    }
    1
}

/// No bid: the contract offer (`*out`) is freed and cleared; 0.
unsafe fn bid_drop(b: &mut Bid) -> u32 {
    if !b.out.is_null() {
        if *b.out != 0 {
            exe::crt_operator_delete(*b.out);
        }
        *b.out = 0;
    }
    0
}

/// Fills an offer for a player from a buying club: the fee from the asking price, the valuation and the ceiling, the
/// clauses, the instalments, and the contract offer of a pre-contract; 1 when the bid stands.
///
/// `TRANSFER_OFFER::build_bid(player, club, slot, max_fee, pre_contract, out)`. Without a player or a club: the
/// transfer_offer.cpp line 0x154 box and 0. The offer starts empty (`bid_player_aware` with one chance in 20; the
/// buyer and target ids, `pre_contract`, the slot's job or 0xb, no lists, clauses of 10 games, no fee; `*out` = 0).
/// A human buyer (`exe::human_manager_is_human_club`): his last transfer bid for the player
/// (`exe::transfer_manager_find_club_offer_for_player`), unless a pre-contract, is copied (`bid_copy_human_offer`);
/// without one the fee is the player's value, at most the transfer budget. A computer buyer: a compensation deal
/// (`exe::shortlist_manager_is_compensation_deal`) or a player free to leave at the end of his contract
/// (`player_club_uses_bosman`) becomes a pre-contract (`bid_pre_contract`) unless a contract of a type other than 1,
/// 3 or 5 holds a player slot without a deal; otherwise the fee is worked out: `bid_asking_price`, `bid_valuation`,
/// `bid_spread_fee`, `bid_fee_under_ceiling`, `bid_settle_fee` and `bid_afford`. Every bid ends in `bid_finish`.
/// Defects of the original: a refused pre-contract leaves `*out` pointing at the freed contract offer; the candidate
/// international's share reads a stale stack double (`bid_clauses_out_of_reach`).
/// Original: `TRANSFER_OFFER::build_bid` (0x8e5d60, transfer_offer.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn transfer_offer_build_bid(this: usize, player: usize, club: usize, slot: usize, max_fee: i32, pre_contract: u8, out: *mut usize) -> u32 {
    if player == 0 || club == 0 {
        show_error_at(0xa9e318, 0x154);
        return 0;
    }
    let mut b = Bid {
        o: Offer(this), player: Staff(player), club: Club(club), slot, max_fee, out, contract: 0, seller: Club(0),
        asking: 0, asking_cap: -1, diff: 0, valuation: 0, crisis_weighed: false, clauses_set: false,
    };
    let o = b.o;
    o.set_player_aware(bid_player_aware(&b, 0x14));
    write_u32(this, 0); // the rival offer list
    write_u32(this + offer::EXCHANGE_PLAYERS, 0);
    o.set_buyer_club_id(read_usize(club));
    o.set_target_staff_id(read_usize(player));
    b.contract = game_get_contract_and_store(player);
    o.set_pre_contract(pre_contract);
    o.set_job_type(if slot != 0 { read_u8(slot) } else { 0xb });
    o.set_negotiation_index(usize::MAX);
    o.set_serial(usize::MAX);
    write_u32(this + offer::OFFER_KIND, read_u32(this + offer::OFFER_KIND) & !0xf);
    o.set_offer_type(0);
    o.set_appearance_fee(0);
    o.set_appearance_games(0xa);
    o.set_international_fee(0);
    o.set_international_games(0xa);
    o.set_sell_on_percent(0);
    o.set_instalments(0);
    o.set_status(0);
    o.set_terms_state(0);
    o.set_response_round(0);
    o.set_completion_timing(o.completion_timing() & 0xf0);
    o.set_fee(0);
    if !out.is_null() {
        *out = 0;
    }
    let end = if exe::human_manager_is_human_club(b.club, 0, 1, 0, 0) != 0 {
        let existing = exe::transfer_manager_find_club_offer_for_player(read_usize(G_TRANSFER_MANAGER), b.player, b.club, 1);
        if existing != 0 && Offer(existing).offer_type() == 0 && o.pre_contract() == 0 {
            bid_copy_human_offer(&mut b, existing);
        } else {
            covered!("sx21b_2 human club bids the value");
            o.set_fee(b.player.value());
            let fin = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), b.club);
            let budget = exe::finance_get_transfer_budget(fin, player as u32, 0, 0usize, 1, 0) as i32;
            if o.fee() as i32 > budget {
                o.set_fee(budget as usize);
            }
        }
        BidEnd::Settle
    } else {
        bid_by_computer(&mut b)
    };
    match end {
        BidEnd::Settle => bid_finish(&mut b),
        BidEnd::Drop => bid_drop(&mut b),
        BidEnd::Fail => 0,
    }
}

/// A computer club's bid: the pre-contract or the worked-out fee (see `transfer_offer_build_bid`).
unsafe fn bid_by_computer(b: &mut Bid) -> BidEnd {
    let sm = read_usize(read_usize(G_TRANSFER_MANAGER) + transfer_manager_fields::SHORTLIST_MANAGER);
    let mut compensation = 0i32;
    let deal = exe::shortlist_manager_is_compensation_deal(sm, b.slot, b.contract, b.club, &mut compensation);
    let free = deal == 0 || exe::transfer_manager_player_club_uses_bosman(read_usize(G_TRANSFER_MANAGER), b.player) != 0;
    if b.slot != 0 && free {
        let c = Contract(b.contract);
        let held = !c.is_null() && read_u8(b.slot) == 0xb && !matches!(c.contract_type() & 0x3f, 1 | 3 | 5);
        if !(held && deal == 0) {
            return bid_pre_contract(b);
        }
    }
    b.seller = Club(contract_club_of(b.player.addr()));
    if let BidEnd::Fail = bid_asking_price(b) {
        return BidEnd::Fail;
    }
    bid_valuation(b);
    b.o.set_fee(b.asking as usize);
    bid_spread_fee(b);
    match bid_fee_under_ceiling(b) {
        BidEnd::Settle => {}
        end => return end,
    }
    match bid_settle_fee(b) {
        BidEnd::Settle => {}
        end => return end,
    }
    bid_afford(b)
}

/// Comparer for `transfer_offer_build_bid`: side by side over the 0x41-byte offer, `*out`, `G_CONTRACT_LOOKUP_RESULT`
/// and the random generator. When the original allocated (an exchange list kept, or a contract offer in `*out`), the
/// copy cannot run again: only the original.
pub(crate) unsafe extern "thiscall" fn compare_transfer_offer_build_bid(this: usize, player: usize, club: usize, slot: usize, max_fee: i32, pre_contract: u8, out: *mut usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, i32, u8, *mut usize) -> u32>(%%I0%%);
    let mut regions = vec![(this, 0x41usize), (G_CONTRACT_LOOKUP_RESULT, 4)];
    regions.extend_from_slice(&crate::rng::STATE_REGIONS);
    if !out.is_null() {
        regions.push((out as usize, 4));
    }
    let before = crate::comparison::MemorySnapshot::read(&regions);
    let r = o(this, player, club, slot, max_fee, pre_contract, out);
    if read_u32(this + offer::EXCHANGE_PLAYERS) != 0 || (!out.is_null() && *out != 0) {
        return r;
    }
    let after = crate::comparison::MemorySnapshot::read(&regions);
    before.write_back();
    let c = crate::rng::pause_trace(|| transfer_offer_build_bid(this, player, club, slot, max_fee, pre_contract, out));
    let mine = crate::comparison::MemorySnapshot::read(&regions);
    after.write_back();
    let equal = after == mine;
    SCOREBOARD.record(equal && r == c, || format!("transfer_offer_build_bid(offer {:#x}, player {:#x}, club {:#x}, slot {:#x}, max {}, pre {}) original={} {:x?} rust={} {:x?}",
        this, player, club, slot, max_fee, pre_contract, r, after, c, mine));
    r
}
