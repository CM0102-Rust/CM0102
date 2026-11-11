*English version of [secao_b.md](secao_b.md). Both are kept in sync; the Portuguese file is the original.*

# Market, section B: how the AI buys and sells (2026-11-11, reader B)

Part of the transfer market map asked for by Rust Tweaks (release 1.5; [BRIEF.en.md](BRIEF.en.md)). Addresses are VAs
of `cm0102.exe` 3.9.68. **[proved]** = read in the disassembly (`desmonta.py`) or in Rust code already compared with
the original; **[inferred]** = read only in Ghidra's pseudo-C (`pacotes/0x83cb40`, `pacotes/0x83f3a0`) or concluded
without direct proof. Field names are those of `rust/src/motor/fields.rs` and `records.rs`.

## 1. The routines

| Address | Name (community map) | In Rust | Item |
|---|---|---|---|
| 0x84a020 | `Shortlist_Manager::make_club_bids` (the AI walks its 40 slots, bids and asks for loans) | yes | cqe_3 (batch 268) |
| 0x83f3a0 | `Shortlist_Manager::fill_shortlist_slot` (chooses a slot's target) | no | not ported (§8) |
| 0x8c8090 | `TRANSFER_MANAGER::ai_respond_to_bid` (an AI seller's answer to a bid) | yes | cqe_1 |
| 0x84af00 | `shortlist_manager_predict_selling_club_response` (accept, negotiate, refuse) | yes | cpo_9 |
| 0x8c0810 | `TRANSFER_MANAGER::review_ai_club_bid` (the buyer reviews its bid after the counter offer) | no | **mkb_1** (compiles; proof pending) |
| 0x842c00 | `Shortlist_Manager::add_player_to_club_shortlists` (the showcase) | yes | cpp_6 |
| 0x83cb40 | `Shortlist_Manager::assess_club_squad` (assesses the squad; puts players on the transfer and loan lists) | no | not ported (§8) |

Neighbours used by the rules below: 0x83a730 `update_club_shortlist` (Rust), 0x842320 `transfer_list_player` (Rust),
0x8442b0 `add_to_loan_list` (Rust, screens only), 0x842740 `try_loan_offer_for_player` (Rust), 0x846920
`staff_interest_in_joining_club` (Rust), 0x840d80 `value_player_for_club` (Rust), 0x83b6e0 `allocate_transfer_budget`
(Rust), 0x8c63a0 `process_due_offer` (Rust), 0x842e10 `try_add_player_to_club_shortlist` (original), 0x83a1b0
`create_initial_shortlists` and 0x84abe0 `rebuild_club_shortlist` (original).

## 2. Who calls and when

- **The club shortlist's rhythm** (day loop `game_main_day_loop` 0x5cccd0, 0x5cdc97..0x5cdd7d) [proved]: for each club
  with a nation, in the order of `[0xae2d1c]`, the counter `club_shortlist::DAYS` goes up 1 a day (up to 200). The list
  is redone (`update_club_shortlist` 0x83a730 → `assess_club_squad` with daily 1, reset 0 → `allocate_transfer_budget`)
  when the club has no bid in a slot (`count_slots_with_bids(club, −1)` = 0) and `club_shortlist::ACTIVE` = 2, or when
  DAYS > 30 + random(70) **and** random(5) = 0. At career creation `create_initial_shortlists` 0x83a1b0 (called by
  `game_run_to_start_date` 0x5cc850) calls `assess_club_squad` with daily 0, reset 1; so does `rebuild_club_shortlist`
  0x84abe0 (new manager, `reset_board_for_new_manager` 0x694c80, and the menu 0x7615d0).
- **The bids** (`make_club_bids` 0x84a020, called at 0x5cdd6d) [proved]: with the nation's league loaded
  (`nation::LEAGUE_SELECTED` +0x11c ≠ 0), every day, except a club with no league or in the league of
  `[KEY_CLUB_COMP_A_LOWER_DIVISION]` (0x9cf764); otherwise every day if the reputation is above 6250 (0x186a). In the
  other cases only when (random(2) + club id) % 7 = 0, with random(2) drawn **once a day** before the loop (0x5cdc76).
  **Defect:** the test does not use the date; only clubs with id ≡ 0 or 6 (mod 7) pass, each on half the days; the other
  5/7 of those clubs **never** bid from their shortlist [proved; effect: inferred, most likely meant "once a week"].
- **The answer to an AI bid** (`process_due_offer` 0x8c63a0) [proved]: status `offer::STATUS` 0 with an AI seller: loan
  (`offer::OFFER_TYPE` = 1) → 0x84af00(seller, player, 0, buyer, −1, 1): ≠ 0 refuses the loan (0x8bfa30), 0 goes on to
  acceptance (0x4cc930, status 8, and the player's willingness 0x8d8310); transfer → 0x8c8090. Status 1
  (`offer_status::ON_HOLD`, the counter offer) with an AI buyer: loan → 0x8c0f40; transfer → **0x8c0810** with
  keep_existing 0. Status 0x11/0x12: 0x84af00(seller, player, weighted value, buyer, −1, 0): 0 accepts (0x8c0080), 1 or
  2 refuses (0x8bf680) — here "negotiate" also becomes a refusal.
- **The prediction 0x84af00** has 7 call sites [proved, refs.py]: 0x784b4d and 0x784e0b (`NewsItem::auto_respond`
  0x784100, the holiday auto-answer), 0x84049c and 0x8405a2 (inside 0x83f3a0, fragment 0x840084), 0x8435c3
  (`try_add_player_to_club_shortlist` 0x842e10, the showcase), 0x8c6560 and 0x8c6a1b (`process_due_offer`). The request
  listed 5; 0x784b4d and 0x8c6a1b were missing.
- **0x8c0810** [proved]: 0x784bb1 (holiday), 0x8c2e9f (`handle_player_rejected_terms` 0x8c28c0), 0x8c685c
  (`process_due_offer`), 0x8f2cea (remove-exchange-player dialog 0x8f2870). 13 calls up to 2002-06-01 in the 17/10
  coverage.
- **0x8c8090** [proved]: 0x784b85 (holiday), 0x8c0527 (`accept_transfer_bid`), 0x8c2e3d (`handle_player_rejected_terms`),
  0x8c42fa (`remove_exchange_player`), 0x8c66fe (`process_due_offer`). The loan refusal at its top is only reachable
  from a caller that sends a loan, and `process_due_offer` sends loans to 0x84af00: the request's correction (07/11) is
  **right** [proved].
- **The showcase 0x842c00** [proved, refs.py]: 16 call sites, among them `transfer_list_player` 0x842320 (with news),
  `CONTRACT_MANAGER::daily_update` 0x4c7ac0 (3), `handle_transfer_request` 0x4d8610, `game_daily_staff_update` 0x5ce2d0
  (2), `Media::post_player_praise_story` 0x744240, `award_raise_winner_reputation` 0x412f00, `NewsItem::auto_respond`,
  `move_player_to_club` 0x8c9720, `set_player_squad_status` 0x8ce0e0 and the person screens (2). One in
  `CONTRACT_MANAGER::save_to_disk` 0x4dc3a0 (0x4dd0a1) is odd: maybe the map's boundary [inferred].
- **The AI transfer list**: `transfer_list_player` 0x842320 has 3 call sites: 0x83dc42 (inside `assess_club_squad`, the
  only AI one) and the screens 0x8f8030/0x8f8607 [proved]. **The loan list**: the only `or byte [r+0x4e], 2` of `.text`
  are 0x83e49c and 0x83e651 (`assess_club_squad`) and 0x8442c4 (`add_to_loan_list`, called only by the screens:
  0x8f806c, 0x8f8538); plus two through a register in `assess_club_squad` (0x83dd42, 0x83dd98) [proved by the scan;
  writes through other instruction forms were not scanned: inferred there are none].

## 3. The rules

### 3.1 The squad assessment (0x83cb40, original)

`assess_club_squad(this, entry, club, daily, reset)`, thiscall, ret 0x10. Order [proved where marked; the rest
pseudo-C, inferred]:

1. DAYS (+8 of the entry) = 0. Leaves (and ACTIVE = 0) if the club is a national team, without a nation, human with
   daily, or `club::PROFESSIONAL_STATUS` = 3; with a linked B team (`club_get_linked_b_team` 0x540a50) without the
   partner, leaves [inferred; the call at 0x83cc04 proved].
2. No bid in a slot: if ACTIVE was 2, it becomes 1 and reset = 1. If the nation's league is not loaded or the club's
   league is not active in `[0xadadfc]`, and the reputation < 4000: if random(reputation + 1000) < random(5000), ACTIVE =
   0 and leaves (small clubs of unloaded leagues switch their shortlist off) [proved: 0x83cc6b..0x83ccab].
3. With reset: the 40 slots are rebuilt (`shortlist_set_up_formation_slots` 0x83c090; each slot's status by its index
   and by the target rating `local_840` from `player_search_get_club_target_rating` 0x7bca90: up to 6000 or 8500, with
   or without a continental competition). Without reset: the slots are only sorted (qsort 0x945333) [inferred].
4. Slots without a target, or with a target, no offer and an expired date (or a target with job 0x10): target and
   replacement = −1, price = 0, and the player slots go to the lists of positions to fill by status (2/3, 4, 5)
   [inferred].
5. **The squad** (`database_collect_club_squad` 0x541f90, 50): for each player with a sheet, job ≠ 0x10, not on the
   club's shortlist, with a contract at the club, no future transfer (0x8cca10) and no `contract::LEAVING_ON_BOSMAN`: at
   an AI club with no manager on holiday, **clears the transfer and loan marks** (`contract::TRANSFER_STATUS` & 0xfc),
   sets the status (`contract::SQUAD_STATUS` high nibble) 6 under 20 or 7 (AI club) and sends him to the first team
   (`move_to_first_team` 0x4cc0b0) [proved: 0x83d281..0x83d2d7]. Injured (`Injury::get_player_injury` 0x62ed70) for more
   than 30/300/400 days (by the rating 0x6e6340 against the target rating × 0.75) goes to the reserves
   (`move_to_reserves` 0x4cc0f0); the others go to one of two lists (with or without a loan contract at another club)
   [inferred].
6. Selection by status (`match_man_assign_players_to_positions` 0x6e2000) for each list of slots; whoever is chosen
   leaves the list of the rest; those older than goalkeeper/10 + 22 go to the end [inferred].
7. **The rest** (the end of the squad), for each player of the club, without a loan contract and not queued for a free
   transfer (0x8c1870) [inferred as a whole; the marked points proved]:
   - a player below the manager's rating (`match_man_judge_player_ability` 0x6e86d0 < threshold) and (contract type 2
     with little time, or reputation × 50 ≤ `staff::VALUE`, or remaining wages/30 ≤ value) and the squad
     (`database_count_club_squad`) > reputation/500 + 20: with reset, happiness −100 (0x9c); at an AI club with rating 2
     < reputation/68 − 35 and (age > goalkeeper/5 + 21 or `player_sheet::CURRENT_REPUTATION` > club reputation / 2):
     **transfer list** (`transfer_list_player` 0x842320, with news only without reset) [proved: 0x83dba7..0x83dc42];
   - otherwise, AI, without reset, with no offers or exchange, and squad > reputation/500 + 25: **free transfer queue**
     (`queue_free_transfer` 0x8c16b0) [proved: 0x83db6b];
   - both go to the reserves (except a B team with world reputation ≥ 5000) [proved: 0x83dc47..0x83dc66];
   - a player **above** the threshold at an AI club: with reset, happiness −50 (0xce) [proved: 0x83dc88]; to the
     reserves; and, with negative happiness (`contract::HAPPINESS` < 0) and a player job, the **loan list** (bit
     `transfer_status::LOAN_LISTED`) for a goalkeeper rated 20 aged ≥ 20 with happiness ≤ age × 50/3 − 433, or another
     player aged ≥ 18 with happiness ≤ age × 50/3 − 400; under 24 also status 6 [proved: 0x83dce6..0x83ddb2]. E.g. at
     20 the goalkeeper needs −100 and an outfield player −34; at 24, −33 and +0 (any negative happiness); at 30, any
     negative. **A youngster only gets in when very unhappy.**
8. The slots receive the chosen: the replacement target and, with no manager on holiday, the contract's status from the
   slot's (except 1 and 2) [inferred]. In lists 4 and 5 (reserves, rest):
   - list 4: **loan** if fatigue (`Injury::get_fatigue` 0x62c3d0) < −1000, `CURRENT_REPUTATION` < 4500,
     `player_sheet::WORK_RATE` (+0x44) < 3250 and happiness < −50 [proved: 0x83e46a..0x83e49c]. **Defect:** +0x44 is a
     signed byte compared with 3250 (0xcb2): always true; probably meant `WORLD_REPUTATION` (+0xd, word) [code proved;
     intent inferred];
   - list 5: **loan** if fatigue < −500 − random(300) and happiness < 0 [proved: 0x83e613..0x83e651].
9. The player slots' price (`shortlist_slot::VALUATION`, `shortlist_slot::PRICE`) from the target rating (up to 36000)
   and the status; with the target's rating below, or grievances 0x100/1, or a contract with less than 365 days, the
   slot is redone with adjusted happiness [inferred].
10. Staff slots (jobs 6 to 10) with the person in the job (assistant +0xd3, `club::COACHES`, `club::PHYSIOS`,
    `club::SCOUTS`) scored by `score_staff_for_job` 0x69d1b0 against target rating/5 (up to 5000); below it and with no
    offer (0x4cc9e0), **sacked** (`MANAGER_MANAGER::remove_staff_from_club` 0x690890) [inferred].

### 3.2 Choosing the target (0x83f3a0, original)

`fill_shortlist_slot(this, slot, club, excluded)`, thiscall, ret 0xc, recursive [inferred except where marked]:

1. Leaves: no nation; human club; manager on holiday with rule 4 = 1; slot with an offer; national team.
2. **The knowledge divisor k** [proved: 0x83f4d8..0x83f5b8]: 100; reputation < 3500 → (6025 − reputation) × 4;
   Belgium, Holland, Sweden, Denmark, Norway → 50. Each of the 7 scout places (`club::SCOUTS`): empty +5 (+10 with the
   league active); with a scout, + 30 − 2 × `chance_to_know`(scout, 6) − `chance_to_know`(scout, 5). k = k/10, minimum
   2. **Probable defect:** 0x544410 gets the constants 6 and 5, not the scout's attributes; the result depends only on
   his current ability (+4 of the non-player sheet): `JUDGEMENT`/`JUDGING_POTENTIAL` never count [code proved; intent
   inferred].
3. Reputation factor (×0.0008, or ×1/1175, 1/1125, 1/1050 by the wealth band 0x5986b0 in ITA, ENG, GER, FRA, ESP), the
   slot's ceiling (`max_fee_for_squad_status` 0x84b230).
4. Walks the position lists (`POSITION_LISTS_BY_PERSON`) of the slot's position. A candidate passes: not excluded, job
   ≠ 5, no job 5 at a national team, no manager vacancy (0x69fc60); a player with a sheet, no retirement announced, not
   Saudi; under 24 in status-5 slots; **price × 1.3 ≤ the slot's ceiling** (`shortlist_slot::MAX_PRICE`) or loan
   listed; not of the club itself; on fewer than 10 shortlists; arrived more than 200 days ago (or status 7,
   grievances, listed); not on another of the club's slots; work permit (0x84b180, 0x8c7560); for a loan, can loan
   (0x8d0040) and a human's only if listed.
5. **Whom the club knows**: (person id + club id) % k = 0; or a foreigner of another nation older than 22, or world
   reputation > 3249, or not foreign (0x53d980), and the same region (+0x75 of the nation) or a club of reputation >
   2749 with…, and then the reputation as seen (0x5407e0) ≥ 2750 + random(2000) (slots 5/6) or ≥ 3250 + random(2000);
   a youngster (< 20) with home reputation < 3250 at a club < 3250: (id + club) % 15, or % (25 − the manager's
   `non_player::YOUNGSTERS`) = 0. The human's scouts (0x7f7490) count for a club on holiday.
6. The score (0x840d80, the player's value for the club) ≥ the slot's value and ≥ value − 10000; the asking price
   (0x4d1600, 0x4d42d0 human; compensation exchange 0x84b6f0); **the prediction 0x84af00** with the predicted bid: 2
   discards. Injury: (days − 15)² or (days − 10)² × 200 off.
7. The best by score with a draw; **willingness** (0x846920) ≥ 3.15 (0x968e58) and wages: (3.5/willingness, minimum
   0.9) × the desired wage (0x864550) ≤ 1.2 × the wage cap (0x5968d0). Interest news (0x754740) with draws.
8. The chosen becomes the target (`shortlist_slot::TARGET_STAFF_ID`) and enters the shortlist (`add_shortlist_entry`
   0x8423f0). With nobody, a status 2/3 slot becomes a **loan slot** (bit 0) and starts again if (reputation² × 4e-9
   + 0.6) × value < price, or ceiling < 50000, or Italy with a draw and reputation 3000..6000; an unloaded nation with
   random ≤ random(20000). Otherwise the target stays −1.

### 3.3 Bids and requests (0x84a020, Rust)

The header in `rust/src/motor/shortlist_manager.rs` (lines 38-97) is complete [proved: Rust compared, cqe_3]. In
short: an AI club (not a national team, not human, not a human on holiday with rule 4 = 1), active shortlist, DAYS <
100, with finances; the count = slots with a bid + squad; player slots skipped with count ≥ 50. **Purchase**: the slot
value (`shortlist_slot::PRICE`, ×2 with the 0x20 mark, at least 20000), ×3/×2 in an active league by reputation > 4750
and count < 25/35; replacement unable, injured (days² × reputation/500), morale, under offer; desired wage ≤ 1.3 × cap;
**random(value) > random(2 × reputation + 1000)** makes the bid. **Loan**: not a French amateur club; loan possible; a
human seller only with the player listed; **random(PRICE) > random(reputation/10)**, or on the second pair >
random(2 × reputation): the request (0x842740, which needs willingness ≥ 3.5).

### 3.4 The seller's prediction (0x84af00, Rust)

`stdcall(seller, player, offer i64, buyer, asked i64, loan)`, ret 0x20; 0 accepts, 1 negotiates, 2 refuses. Order
[proved: Rust compared, cpo_9, and its header]:

1. No seller, or the contract with him (`contract_manager_get_contract_for_club`) with `contract::LEAVING_ON_BOSMAN`:
   **0** (for a loan too).
2. The seller's manager is the player: 2.
3. No contract, or `contract::CONTRACT_TYPE` & 0x3f ≠ 2: loan **2**, purchase 0.
4. Loan: human seller: 0. Loan listed (`staff_is_loan_listed` 0x544590, value 2 of +0x4e) and the buyer's reputation ×
   1.25 > `CURRENT_REPUTATION`: 0. Status 6 or 7 (+0x4f) or a **goalkeeper rated 20** (`player_sheet::GOALKEEPER` =
   20), with the buyer's reputation > 0.75 × `CURRENT_REPUTATION`, **`contract::HAPPINESS` (+0x3d) < −50** and the
   seller's `club::CONTINENTAL_COMP_ID` ≠ `[KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE]`: 0. Otherwise 2.
5. Purchase: the price (human: 0.8 × the human's asking price; with no clause and on holiday with rule 0: 1 refuses, 2
   the value; AI: the asked price if ≥ 0, the value without a sheet, or `contract_manager_calc_asking_price`); offer ≥
   price: 0; human, offer > price − 1,500,000 and listed: 1; price × 0.7142857 > offer: 2; otherwise 1.

**The request's reading (MERCADO.md §2, EMPRESTIMOS.md) checks out**, with three corrections: (a) the "morale" of case 2
is the **happiness** `contract::HAPPINESS` (+0x3d), not the morale (+0x44); (b) "or goalkeeper" is a goalkeeper
**rated 20** in the position; (c) the early exits were missing: a seller contract of type ≠ 2 (or no contract)
**refuses** the loan, and a Bosman mark or no seller **accepts**. "Wages and period do not count" checks out: 0x84af00
reads neither [proved].

### 3.5 The buyer reviewing its bid (0x8c0810, original; item mkb_1)

The complete rule is in the header of `levas/fila/mkb_1/codigo.rs` [proved: disassembly 0x8c0810..0x8c0f38]. In
short: with keep_existing 0 it reviews the **counter offer** (the negotiation `offer::NEGOTIATION_INDEX`; without one,
status 0x10 and rescheduled); finds the buyer's slot (0x849ce0; none, withdraws); value = 0x840d80; price = weighted
value (0x8e7b50 with 0,1,1,1,1) + the asking price of each exchange player that fits (the others leave); **price >
value**: withdraws, marks the slot (0x843e10 event 1) and the player may be **unsettled** (`apply_event_flags`
0x800000) if he has been at the club more than (loyalty + 42) × 5 + professionalism days, and the buyer's reputation is
1250 above the seller's (or listed by request, flag 8, grievances 0x14), and the bid passes 17.5 million, 1.5 × the
level (value for the seller ×8 up to 500 thousand, (v − 500 thousand) × 2 + 2 million up to 2.5 million, (v − 2.5
million) × 1.25 + 6 million; times 0.9..2.5 by mood) or, above 250 thousand and the level, with random(temperament + 1)
= 0. **Within the value**: tops the fee up with the dropped exchange players (up to the value) and, if the budget
(`Finance::get_transfer_budget` 0x598800) covers the total (0x8e7a70), **bids again** (`add_transfer_offer` 0x8be300).

### 3.6 The seller's answer (0x8c8090, Rust)

Complete header in `rust/src/motor/transfer_manager.rs` (lines 520-565) [proved: Rust compared]. In short: a loan bid
refused without looking at anything (practically dead code, see §2); "must accept"
(`transfer_manager_selling_club_must_accept`) accepts; a human on holiday with rule 0 refuses; floor = asking price
(0x4d1600) or the best other offer; cash, exchange and clause options; the first whose value × 0.85 is below the asking
price accepts; the best × 0.75 below the asking price becomes a **counter offer** (except types 1 and 2); otherwise
refuses (reason 10 if the total < 0.55 of the value with a 2 million difference, or < 0.1 with 100 thousand).

### 3.7 The showcase (0x842c00, Rust)

[proved: Rust compared, cpp_6] Only a player with a sheet and `CURRENT_REPUTATION` ≥ 5500 (0x157c), unless forced; no
retirement announced; a player job; not a national team manager; not Saudi; no future transfer; one who arrived less
than 200 days ago only with status 7 or grievances. Walks the clubs by reputation (`[G_CLUB_REPUTATION_ORDER]`) and
offers him (0x842e10, which uses 0x84af00) until he is on 5 shortlists (the count is the low nibble of +0xb of the
79-byte record), skipping national teams, the contract's club and his own. **So a player under 5500 reputation is never
offered**, and high-reputation clubs get him first.

## 4. Fields read and written

| Record | Field (offset) | Routines | R/W |
|---|---|---|---|
| contract | `TRANSFER_STATUS` +0x4e (value 1 transfer listed, value 2 loan listed) | 0x83cb40 (clears and sets), 0x84af00, 0x84a020 | R/W |
| contract | `SQUAD_STATUS` +0x4f (high nibble 1..7) | 0x83cb40, 0x84af00, 0x84a020 | R/W |
| contract | `HAPPINESS` +0x3d, `UNHAPPY_FLAG` +0x3e, `MATCHES_LEFT_OUT` +0x3f | 0x83cb40 (with reset writes −100, −50, ±), 0x84af00 (reads) | R/W |
| contract | `CONTRACT_TYPE` +0x35, `LEAVING_ON_BOSMAN` +0x49, `GRIEVANCES` +0x45, `WAGE` +0xc, `CONTRACT_EXPIRES` +0x2d | 0x83cb40, 0x84af00, 0x8c0810, 0x83f3a0 | R |
| contract | event flag 0x800000 (`apply_event_flags`) | 0x8c0810 | W |
| person | `AGE` +0x18, `JOB_FOR_CLUB` +0x3d, `CLUB_JOB` +0x39, `DATE_JOINED_CLUB` +0x3e, `VALUE` +0x52, `AMBITION` +0x57, `LOYALTY` +0x59, `PRESSURE` +0x5a, `PROFESSIONALISM` +0x5b, `TEMPERAMENT` +0x5d, `PLAYER` +0x61, `NON_PLAYER` +0x69, `NATION` +0x1a | all | R |
| sheet | `CURRENT_REPUTATION` +0xb, `WORLD_REPUTATION` +0xd, `GOALKEEPER` +0xf, `HOME_REPUTATION` +9, `WORK_RATE` +0x44 (defect), `PLAYER_MORALE` +0x45 | 0x83cb40, 0x84af00, 0x84a020, 0x83f3a0, 0x842c00 | R |
| club | `REPUTATION` +0x80, `NATION` +0x53, `DIVISION` +0x57, `PROFESSIONAL_STATUS` +0x64, `MANAGER` +0xcf, `ASSISTANT_MANAGER` +0xd3, `SQUAD` +0xd7, `COACHES` +0x19f, `SCOUTS` +0x1b3, `PHYSIOS` +0x1cf, `CONTINENTAL_COMP_ID` +0x1db | all | R |
| staff | `non_player` +4 (the scout's ability), `YOUNGSTERS` +0x22 (manager) | 0x83f3a0 | R |
| club shortlist | `club_shortlist::ACTIVE` +4, `DAYS` +8; slot `shortlist_slot` (+0 job, +1 position, +3, +7, +0xb, +0xf, +0x13, +0x17, +0x1b, +0x23, +0x24) | 0x83cb40, 0x83f3a0, 0x84a020, 0x8c0810 | R/W |
| 79-byte record | `STATUS_FLAGS` +0xb (low nibble = on how many shortlists; 0x30 retirement) | 0x83f3a0, 0x842c00, 0x84a020 | R |
| offer | `NEGOTIATION_INDEX` +0x28, `STATUS` +0x2c, `FEE` +0x1b, `EXCHANGE_PLAYERS` +4, `OFFER_KIND` +0x31, `JOB_TYPE` +0x11 | 0x8c0810, 0x8c8090 | R/W |

The chairman and the directors (`club::CHAIRMAN` +0xbf, `club::DIRECTORS` +0xc3) are **not read** by any of this
section's 7 routines [proved for the 4 in Rust and for 0x8c0810; inferred for 0x83cb40 and 0x83f3a0 from the pseudo-C].
Of the manager only `non_player::YOUNGSTERS` (0x83f3a0) and his judgement in the rating
(`match_man_judge_player_ability` 0x6e86d0 gets `club::MANAGER`) count; of the scouts, only their current ability
(§3.2).

## 5. Constants (read from the exe)

| Address | Value | Use |
|---|---|---|
| 0x968928 | 1.25 (f64) | 0x84af00: buyer's reputation × 1.25; 0x8c0810: (v − 2.5 m) × 1.25 |
| 0x969040 | 0.75 | 0x84af00 (reputation in the 6/7 case); 0x83cb40 (rating × 0.75) |
| 0x968e58 | 3.15 | 0x83f3a0: minimum willingness |
| 0x968e38 | 3.5 | 0x83f3a0 (3.5/willingness); 0x842740 minimum loan willingness |
| 0x968de8 | 1.3 | 0x83f3a0: price × 1.3 ≤ slot ceiling; 0x84a020: wage ≤ 1.3 × cap |
| 0x968978 | 1.2 | 0x83f3a0: wage ≤ 1.2 × wage cap |
| 0x9689d8 / 0x96a32c | 0.9 | 0x83f3a0: minimum of 3.5/willingness |
| 0x96f898, 0x968e48 | 4e-9, 0.6 | 0x83f3a0: turning into a loan slot |
| 0x96a618, 0x96f8a8, 0x96f8b0, 0x96f8b8 | 0.0008, 1/1175, 1/1125, 1/1050 | 0x83f3a0: reputation factor |
| 0x968940 | 1.5 | 0x8c0810 (level × 1.5); 0x84a020 (morale) |
| 0x968f10, 0x96aff8, 0x968e90 | 8, 2,000,000, 6,000,000 | 0x8c0810: level |
| 0x96b000, 0x96aff0 (f32) | 500,000, 2,500,000 | 0x8c0810: level bands |
| 0x96a32c, 0x96b014, 0x968e70, 0x96a33c, 0x96b010, 0x96b004, 0x96b00c, 0x9689a8, 0x96b008 (f32) | 0.9 0.95 1 1.1 1.25 1.5 1.75 2 2.5 | 0x8c0810: mood factor |
| immediates | 5500 (0x157c), 5 clubs | 0x842c00 |
| immediates | 6250 (0x186a), % 7, 30 + random(70), random(5) | day loop |
| immediates | −1000 (0xfc18), 4500 (0x1194), 3250 (0xcb2), −50 (0xce), −500 − random(300) | 0x83cb40: loans |
| immediates | −433 (0x1b1), −400 (0x190), ages 18/20/24 | 0x83cb40: loan by happiness |
| immediates | 17,500,000, 250,000, 1250 | 0x8c0810 |

## 6. Defects of the original

1. **Bids from only 2/7 of the clubs** outside loaded leagues (or with no league, or in the league of key 0x9cf764)
   with reputation ≤ 6250: (random(2) + id) % 7 without the date [proved].
2. **0x83cb40, list-4 loan:** `WORK_RATE` (a byte) < 3250 is always true [proved; intent inferred].
3. **0x83f3a0, scouts:** `chance_to_know` with the constants 6 and 5; the scout's judging attributes never count [code
   proved; intent inferred].
4. **0x84af00:** with a seller contract of type ≠ 2 a loan is always refused and a purchase always accepted (any offer)
   [proved; whether intended, unknown].
5. **0x8c63a0, statuses 0x11/0x12:** the prediction "negotiate" (1) becomes a refusal [proved; intent inferred].
6. **The person repeated in the squad** (the request says it is born in 0x83cb40): **it is not born there**. The copy
   is written by `toggle_squad_placement` 0x4cbd20 (through `database_add_staff_to_club` 0x53bd10, which does not check
   presence), called by `move_to_first_team` 0x4cc0b0 and `move_to_reserves` 0x4cc0f0. 0x83cb40 is the most frequent
   **trigger**: at each assessment it sends every player of an AI club to the first team and then part of them to the
   reserves (0x83d2a5, 0x83d2d7, 0x83d3f4, 0x83dc66, 0x83dcbc, 0x83e456, 0x83e60e, 0x83e770, 0x83e80f, 0x83e84a)
   [proved]; with a linked B team (place 3/4) the person changes club and the copy appears
   (`docs/DEFEITOS_DO_ORIGINAL.md` line 102) [proved there]. `docs/COMISSAO_REPETIDA.md` is another repetition (coach
   places, 0x7a65d0), unrelated to 0x83cb40.

## 7. Extension points (a small hook changes the policy)

| Where | What changes | How |
|---|---|---|
| 0x84af00 (Rust; 7 call sites) | who lends and sells | wrap: call the original and turn 2 → 0 by the new rule (the Tweaks' `loan_rule = squad` already does) |
| `assess_club_squad` 0x83cb40, after the call at 0x83a763 (0x83a730 is Rust) | who goes on the transfer and loan lists | at the end of `update_club_shortlist`, walk the squad and set/clear `LOAN_LISTED` (youngsters with PA ≫ CA and few games) |
| 0x842c00 (Rust) | who is offered | the 5500 threshold and the 5-shortlist maximum |
| day loop 0x5cdd27..0x5cdd6a | who bids and how often | fix the % 7 (use the day) or the 6250 reputation |
| 0x5cdcee..0x5cdd15 | shortlist rhythm (30 + random(70), random(5)) | shorter for successful clubs |
| 0x83f3a0 divisor k (0x83f4d8..0x83f5b8) | how many players the club knows | pass the scout's `JUDGEMENT`/`JUDGING_POTENTIAL` |
| 0x83f3a0 willingness 3.15 (0x968e58) and 1.3/1.2 | who gets on the shortlist | the `.rdata` constants (exe copy) or the hook |
| 0x84a020 (Rust) draws | bid rhythm | `2 × reputation + 1000` and `reputation/10` |
| 0x8c0810 (mkb_1) | how much the buyer accepts to pay after the counter offer | `w > value` |

## 8. What was not ported

- **0x83cb40** (10,336 bytes to the boundary, 3,136 instructions, a 0x8a4-byte frame, 45 calls, 2 still original) and
  **0x83f3a0** (6,624 bytes, 2,048 instructions, recursive, SEH frame): too big to port and prove in this round; each
  needs an item of its own and a reader with time for the proof. The map above (§3.1, §3.2) is the starting point;
  0x83f3a0 has the fragments 0x840000 and 0x840010 (community map) and the piece 0x840084, which are not routines.
- **0x8c0810**: item `mkb_1` (compiles; comparison, careers and mutations did **not** run: the laboratory slots were
  taken; see `levas/fila/mkb_1/PRONTO.txt`).
