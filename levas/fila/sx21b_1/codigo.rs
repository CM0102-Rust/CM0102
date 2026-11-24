// ---------- sx21b_1: a transfer offer: its weighted worth and the fee moved into clauses (transfer_offer.cpp) ----------

/// An offer's worth as the selling club weighs it: the fee, the clauses discounted by the chance that they will be paid,
/// the sell-on share by the player's expected growth, the exchange players, and the whole scaled down by how badly the
/// seller needs the money now when the fee comes in instalments.
///
/// `TRANSFER_OFFER::get_weighted_value(fee_only, no_exchange, valued, with_instalments, with_sell_on)`. The target is
/// `staff_at(offer.target_staff_id())`; the seller is the club of his contract (`contract_club_of`: none, or a type-5
/// contract, gives no club). With `offer.job_type()` 0xb and a club job, his club's view of the player: `now` =
/// `exe::match_man_judge_player_ability(club.manager(), target, club, 2, player.current_reputation())` and `future` =
/// the same with mode 1 and `now * 50`; otherwise both 0. With a seller, `with_instalments` and a positive
/// `offer.instalments()`, the seller's wealth band (`exe::finance_get_wealth_band`) sets the whole-offer factor and the
/// clause factor: band -2: 0.1 and 0.5; band -1: 0.3 and 0.75; band 0: 0.9 under 7 instalments, 0.7 up to 12, 0.4 from
/// 13 (clauses 1.0); band 1: 0.9 above 12 instalments; otherwise 1.0 and 1.0.
/// Unless `fee_only` (and only for a player): the appearance clause adds `amount * (1 - games * 0.005) * clause
/// factor`; the international clause, for a player under 28 with no caps whose `future * 65` beats his nation's
/// reputation, adds `amount * min(0.75, (1 - |age - 24| / 15) * (1 - games * 0.005) * (excess * 0.001333 + 0.3)) *
/// clause factor`; with `with_sell_on` and `now < future * 0.8`, the sell-on share adds `value * share * (future -
/// now) / 33 * clause factor / 100`. Each term is truncated (`_ftol`) as it is added.
/// Unless `no_exchange`, each exchange player adds his `staff::VALUE`, or with `valued` the contract manager's valuation
/// (`exe::contract_manager_calc_player_value(.., player, 0, 0, -1, 0)`). The result: the total times the whole-offer
/// factor, truncated.
/// Original: `TRANSFER_OFFER::get_weighted_value` (0x8e7b50, transfer_offer.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn transfer_offer_get_weighted_value(this: usize, fee_only: u32, no_exchange: u32, valued: u32, with_instalments: u32, with_sell_on: u32) -> i32 {
    let o = Offer(this);
    let target = staff_at(o.target_staff_id() as isize);
    let seller = Club(contract_club_of(target.addr()));
    let age = target.age() as i8;
    let mut total = o.fee() as i32;
    let (mut now, mut future) = (0i16, 0i16);
    let club = target.club_job();
    if o.job_type() == 0xb && club.addr() != 0 {
        now = exe::match_man_judge_player_ability(club.manager(), target, club, 2, target.player().current_reputation());
        future = exe::match_man_judge_player_ability(club.manager(), target, club, 1, (now as i32).wrapping_mul(50) as i16);
    }
    let (mut offer_factor, mut clause_factor) = (1.0f32, 1.0f64);
    let instalments = o.instalments() as i8;
    if seller.addr() != 0 && with_instalments != 0 && instalments > 0 {
        let band = exe::finance_get_wealth_band(exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), seller)) as i8;
        match band {
            -2 => {
                covered!("sx21b_1 the poorest seller");
                (offer_factor, clause_factor) = (0.1, 0.5); // sx21b_1: the poorest seller
            }
            -1 => (offer_factor, clause_factor) = (0.3, 0.75),
            0 => offer_factor = if instalments < 7 { 0.9 } else if instalments < 13 { 0.7 } else { 0.4 },
            1 => {
                if instalments > 12 {
                    offer_factor = 0.9;
                }
            }
            _ => {}
        }
    }
    if fee_only == 0 && target.player().addr() != 0 {
        let appearance = o.appearance_fee() as i32;
        if appearance > 0 {
            let games = o.appearance_games() as i8 as f64;
            total = total.wrapping_add(ftol((read_f64(rdata::F64_1) - games * read_f64(rdata::F64_0_005)) * appearance as f64 * clause_factor));
        }
        let international = o.international_fee() as i32;
        let excess = || (future as i32).wrapping_mul(65) - target.nation().reputation() as i32;
        if international > 0 && target.int_apps() == 0 && age < 0x1c && excess() > 0 {
            covered!("sx21b_1 international clause weighed");
            let excess = excess();
            let by_age = read_f64(rdata::F64_1) - ((age as i32 - 0x18) as f64).abs() * read_f64(rdata::F64_0_06666666667);
            let by_games = read_f64(rdata::F64_1) - (o.international_games() as i8) as f64 * read_f64(rdata::F64_0_005);
            let by_rating = excess as f64 * read_f64(0x96d250) + read_f64(rdata::F64_0_3); // 0.0013333 a point
            let mut chance = by_age * (by_games * by_rating);
            if chance > read_f64(rdata::F64_0_75) { // sx21b_1: the chance is capped at 0.75
                chance = read_f64(rdata::F64_0_75);
            }
            total = total.wrapping_add(ftol(international as f64 * chance * clause_factor));
        }
        let share = o.sell_on_percent() as i8;
        if share > 0 && with_sell_on != 0 && (now as f64) < future as f64 * read_f64(rdata::F64_0_8) {
            covered!("sx21b_1 sell-on share weighed");
            let stake = (target.value() as i32).wrapping_mul(share as i32) as f64;
            let growth = (future as i32 - now as i32) as f64 * read_f64(0x970450); // 1/33 a point
            total = total.wrapping_add(ftol(stake * growth * clause_factor * read_f64(rdata::F64_0_01)));
        }
    }
    let list = o.exchange_players();
    if list.addr() != 0 && no_exchange == 0 {
        list.set_cursor(list.head());
        loop {
            let d = exe::llist_get_next(list);
            if d == 0 {
                break;
            }
            let player = staff_at(read_i32(d) as isize);
            total = total.wrapping_add(if valued != 0 {
                exe::contract_manager_calc_player_value(read_usize(G_CONTRACT_MANAGER), player, 0usize, 0, -1, 0)
            } else {
                player.value() as i32
            });
        }
    }
    ftol(total as f64 * offer_factor as f64)
}

/// Comparer for `transfer_offer_get_weighted_value`: the value, `G_CONTRACT_LOOKUP_RESULT` and the random generator,
/// side by side.
pub(crate) unsafe extern "thiscall" fn compare_transfer_offer_get_weighted_value(this: usize, fee_only: u32, no_exchange: u32, valued: u32, with_instalments: u32, with_sell_on: u32) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> i32>(%%I0%%);
    let mut regions = vec![(G_CONTRACT_LOOKUP_RESULT, 4usize)];
    regions.extend_from_slice(&crate::rng::STATE_REGIONS);
    let (r, c, equal, _, _) = crate::comparison::side_by_side(&regions, || o(this, fee_only, no_exchange, valued, with_instalments, with_sell_on),
        || transfer_offer_get_weighted_value(this, fee_only, no_exchange, valued, with_instalments, with_sell_on));
    SCOREBOARD.record(equal && r == c, || format!("transfer_offer_get_weighted_value(offer {:#x}, {}, {}, {}, {}, {}) original={} rust={}", this, fee_only, no_exchange, valued, with_instalments, with_sell_on, r, c));
    r
}

/// Moves part of a big fee into a clause or a sell-on share when the buying club cannot afford twice the fee and the
/// player still has growth ahead.
///
/// `TRANSFER_OFFER::restructure_fee_into_clauses(player, unused, club)`. Only for a player whose potential ability is
/// at least 25 above his current ability, and only when the offer has no sell-on share and no clauses yet, and the
/// club's transfer budget (`exe::finance_get_transfer_budget(.., 0, 0, 0, 1, 0)`, 64-bit) is at most twice the fee.
/// A club with a reputation under 15 (`club::REPUTATION`) facing a player whose current reputation in thousands
/// (toward zero) is more than 2 above it: the fee drops to 80% and the sell-on share becomes 20%. Otherwise, a capless
/// player above 4250 current reputation from a nation above 5000 reputation: an international clause of 30% of the fee
/// after 10 caps; or a player whose reputation in thousands is more than 3 below the club's: an appearance clause of
/// 30% after 20 games; either way the fee drops to 80%. Every amount is truncated (`_ftol`).
/// Original: `TRANSFER_OFFER::restructure_fee_into_clauses` (0x8e8330, transfer_offer.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn transfer_offer_restructure_fee_into_clauses(this: usize, player: usize, _unused: u32, club: usize) {
    let (o, p, club) = (Offer(this), Staff(player), Club(club));
    let sheet = p.player();
    if sheet.addr() == 0 || (sheet.potential_ability() as i32 - sheet.current_ability() as i32) < 0x19 {
        return;
    }
    if o.sell_on_percent() != 0 || o.international_fee() != 0 || o.appearance_fee() != 0 {
        return;
    }
    let fee = o.fee() as i32;
    let budget = exe::finance_get_transfer_budget(exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club), 0, 0, 0usize, 1, 0);
    if budget > fee.wrapping_mul(2) as i64 {
        return;
    }
    let fee_f = fee as f64;
    let reputation = club.reputation();
    let rating_gap = (sheet.current_reputation() as i32 / 1000) - reputation as i32;
    if reputation < 0xf && rating_gap > 2 {
        covered!("sx21b_1 sell-on share instead of fee");
        o.set_fee(ftol(fee_f * read_f64(rdata::F64_0_8)) as usize);
        o.set_sell_on_percent(0x14); // sx21b_1: a 20% sell-on share
        return;
    }
    let nation = p.nation();
    if nation.addr() != 0 && nation.reputation() > 0x1388 && sheet.current_reputation() > 0x109a && p.int_apps() == 0 {
        covered!("sx21b_1 international clause instead of fee");
        o.set_international_fee(ftol(fee_f * read_f64(rdata::F64_0_3)) as usize);
        o.set_international_games(0xa);
    } else if rating_gap < -3 {
        covered!("sx21b_1 appearance clause instead of fee");
        o.set_appearance_fee(ftol(fee_f * read_f64(rdata::F64_0_3)) as usize);
        o.set_appearance_games(0x14); // sx21b_1: twenty games for the appearance clause
    } else {
        return;
    }
    o.set_fee(ftol(fee_f * read_f64(rdata::F64_0_8)) as usize);
}

/// Comparer for `transfer_offer_restructure_fee_into_clauses`: the 0x41-byte offer, side by side.
pub(crate) unsafe extern "thiscall" fn compare_transfer_offer_restructure_fee_into_clauses(this: usize, player: usize, unused: u32, club: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I1%%);
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&[(this, 0x41usize)], || o(this, player, unused, club),
        || transfer_offer_restructure_fee_into_clauses(this, player, unused, club));
    SCOREBOARD.record(equal, || format!("transfer_offer_restructure_fee_into_clauses(offer {:#x}, player {:#x}, club {:#x}) original={:x?} rust={:x?}", this, player, club, mo, mr));
}
