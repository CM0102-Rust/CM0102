*English version of [secao_d.md](secao_d.md). Both are kept in sync; the Portuguese file is the original.*

# Market, section D: the player's side and the people who decide (reader D, 2026-11-11)

Scope: the player's will to go or stay (loans included), the wage demand and the contract length, renewals, clauses,
the free-agent market and, across the whole market, where the chairman's and the staff's (manager, assistant, scouts,
directors) attributes are read. Marks: **[proved]** = disassembly, a scan of the exe or Rust code already compared
with 0 divergences; **[inferred]** = Ghidra pseudo-C or a structural reading, not checked instruction by instruction.

Sources: `rust/src/motor/{shortlist_manager,contract_manager,staff_contracts,transfer_manager,match_man,fog_of_war}.rs`
(the routines already in Rust, read from the code and the header), `pacotes/0x4ce4e0`, `pacotes/0x866830`, the pseudo-C
of 0x85fc50, 0x4c7ac0, 0x864550, 0x862720 and 0x844780 (`descompilar.py`), `refs.py` (callers) and a scan of the whole
`.text` (capstone) for byte reads of `[sheet + 0x0e..0x22]` after `mov r, [x + 0x69]` (`staff::NON_PLAYER`).

## 1. The routines

| Address | Name | File | In Rust | Item | Callers / when |
|---|---|---|---|---|---|
| 0x846920 | `Shortlist_Manager::staff_interest_in_joining_club` (the will) | shortlist_manager.cpp | yes | — | bids (0x84a020, 0x842e10, 0x843800, 0x840084), loans (0x842740), accepting a bid (0x8c0080), response delay (0x8cc5b0), unsettled player (0x8c1290), exchange (0x8c8860), budget (0x83b6e0), contract decision (0x4ce4e0), desired wage (0x85fc50), scouts (0x7b0050, 0x7b62a0), filters and screens |
| 0x844780 | `staff_interest_in_job_at_club` (the staff's will) | shortlist_manager.cpp | no | — | 0x4ce4e0, 0x85fc50, `is_staff_suitable_for_job` (0x69f810), 0x840084, filters |
| 0x4ce4e0 | `CONTRACT_MANAGER::resolve_contract_offers` (the player picks an offer) | contract_manager.cpp | no | — | `process_due_decision_dates` (0x4cd9c0), type 3 decision dates |
| 0x85fc50 | `STAFF_CONTRACT::set_to_desired_contract` (wage demand and length) | staff_contracts.cpp | no | — | `offer_contract` (0x4cda70), `lowest_wage_demand` (0x4ce450), 0x4ce4e0, `make_club_bids` (0x84a020), `accept_transfer_bid` (0x8c0080), `move_player_to_club` (0x8c9720), `get_contract_satisfaction` (0x866620), `calc_contract_value` (0x866830), screens |
| 0x864550 | `STAFF_CONTRACT::default_staff_contract` (default wage) | staff_contracts.cpp | no | — | 0x846920 (wage gate), 0x85fc50 (×3), 0x844780, 0x842e10, 0x840084, `update_wage_bill` (0x598370), 0x85e960 |
| 0x862720 | `staff_contract_calc_unset_default_wage` | staff_contracts.cpp | no | — | `set_default_wage` (0x8626e0), `create_initial_shortlists` (0x83a1b0) |
| 0x8626e0 | `STAFF_CONTRACT::set_default_wage` | staff_contracts.cpp | yes | — | regens (0x7a70e4) |
| 0x866830 | `STAFF_CONTRACT::calc_contract_value` (value of a contract against another) | staff_contracts.cpp | no | **mkd_1** | `get_contract_satisfaction` (0x866620, ×2), 0x4ce4e0 (×2), 0x4d4830 (screen) |
| 0x866620 | `get_contract_satisfaction` (1..3) | staff_contracts.cpp | yes | — | the staff's daily pass (0x5ce2d0, random(50) = 0), screen |
| 0x4cda70 | `contract_manager_offer_contract` (the club's offer) | contract_manager.cpp | yes | — | 0x4c7ac0 (×9: renewals), screens |
| 0x4ce450 | `contract_manager_lowest_wage_demand` | contract_manager.cpp | yes | — | contract screen |
| 0x4c7ac0 | `CONTRACT_MANAGER::daily_update` (contract end, renewal, free agents) | contract_manager.cpp | no | — | `game_main_day_loop` (0x5cccd0), daily; `game_run_to_start_date` (0x5cc850) |
| 0x4cd9c0 | `process_due_decision_dates` | contract_manager.cpp | yes | — | 0x4c7ac0, daily |
| 0x4cd230 | `schedule_offer_decision` | contract_manager.cpp | yes | — | contract offers |
| 0x4cc490 | `add_contract_offer` | contract_manager.cpp | yes | — | 0x4cda70 |
| 0x866f60 | `STAFF_CONTRACT::wants_to_leave` (`staff_criterion`) | staff_contracts.cpp | yes | — | 0x4c7ac0 (×3) |
| 0x867240 | `check_transfer_request_withdrawal` | staff_contracts.cpp | yes | — | daily |
| 0x4d8610 | `handle_transfer_request` | contract_manager.cpp | yes | — | transfer request |
| 0x867700 | `STAFF_CONTRACT::apply_event_flags` (grievances, transfer request) | staff_contracts.cpp | no (only a diagnostic detour) | — | events |
| 0x4d4660 | `will_negotiate_offer` | contract_manager.cpp | no | — | screens only (0x4de5d0, 0x4e06e0, 0x4e30f0) |
| 0x4d4830 / 0x4d44e0 | `prefill_offer_from_pending` / `get_offer_job` | contract_manager.cpp | no | — | screens only |
| 0x4cd030 | `trigger_relegation_clauses` | contract_manager.cpp | yes | — | relegation (0x689c80 and others) |
| 0x4cced0 | `trigger_non_promotion_clauses` | contract_manager.cpp | no | — | `league_promote_and_relegate` (0x689c80), ArgPrm (0x4067f0) |
| 0x4cd120 | `reset_division_clauses` | contract_manager.cpp | no | — | `League::move_club_to_division` (0x6830b0) |
| 0x867b50 | `has_active_release_clause` | staff_contracts.cpp | yes | — | 0x4c7ac0, 0x4d0c60, 0x4d2070, 0x84af00, 0x8cd240, 0x8e5d60 |
| 0x868210 | `check_fifa_protected_period` | staff_contracts.cpp | yes | — | 0x4c7ac0, bid, exchange |
| 0x867d30 | `get_expiry_band` (0..14) | staff_contracts.cpp | yes | — | 0x4c7ac0 (×11) |
| 0x8cd240 | `transfer_manager_selling_club_must_accept` | transfer_manager.cpp | yes | — | bid response |
| 0x8caaf0 / 0x8d0dd0 / 0x8cb0f0 | next-sale percentage: record / find / pay | transfer_manager.cpp | yes | — | completed transfer (0x8c91e0) |
| 0x8cac40 / 0x8cad90 / 0x8cb340 / 0x8c39f0 | appearance money (club, national side), pay, after the match | transfer_manager.cpp | yes | — | end of each match |
| 0x867ad0 / 0x85f7a0 | `is_clause_allowed` / `add_release_clause` | staff_contracts.cpp | no | — | screens only |
| 0x8c16b0 / 0x8c18c0 / 0x8c1af0 | free transfer queue: queue / process / release | transfer_manager.cpp | yes | — | 0x4c7ac0; the transfers' day (0x8c5880) |
| 0x8d0a70 | `player_club_uses_bosman` | transfer_manager.cpp | yes | — | 0x4c7ac0, value, ~31 million calls |
| 0x842740 / 0x8d0590 / 0x8d0040 | loan: try, period, can loan | shortlist/transfer | yes | — | 0x84a020 |
| 0x8c9720 | `TRANSFER_MANAGER::move_player_to_club` | transfer_manager.cpp | no | — | completed transfer |
| 0x6e86d0 | `perceived_ability` (ability as an observer sees it) | match_man.cpp | yes | — | value (0x4d0c60), list (0x840d80), regens, engine |

## 2. The rules

### 2.1 The will to go or stay (0x846920, Rust) [proved]

`thiscall(list, person, club, wage, loan, status, &reasons)`, ret 0x18, returns a double in st0. Order of the
conditions (code in `shortlist_manager.rs`, compared with 0 divergences):

1. 0 without a player sheet, without the club's or the person's nation, with a future transfer arranged, or retiring.
2. The current contract (`contract_manager_get_contract`, in `G_CONTRACT_LOOKUP_RESULT`); a club without a nation:
   **3.5**; a type 5 (loan) contract counts as no club.
3. **The stay shortcut** (the club is his own): no grievances in groups 0x161a8, fewer than (id mod 35) + 20 days to
   the end, not on AI club lists, and current reputation + ambition × 20 < (loyalty + 170) × 25 + s (s = (club
   reputation − his)/2, halved with grievance 0x10, capped at loyalty × 10 + 1125): **3.5**.
4. Base: at his own club 3.5 (0 with grievances other than 2); elsewhere **2.75**; leaving on a Bosman,
   (1.1 − days × 0.002) × 2.75; without a club, 2.75 + min(12, days without a club² / 900).
5. Factors, in this order: reputations in a loan request (×1.025 to ×1.5); disliked club ×0.33; current club disliked
   ×1.25/×1.5/×2 (by pressure + loyalty + professionalism); current club favoured ×0.5; new club favoured ×3; over 34
   and another nation ×0.75; rivals (Celtic×Rangers 0.025/loyalty; Croatia×Yugoslavia; direct rival 0.2/loyalty,
   Italy 1 − loyalty × 0.02; same league 0.75/loyalty); the new manager favours ×2 / dislikes ×0.33; the current
   manager dislikes ×2 / favours ×0.5; the contract morale (`contract::MORALE`); a human club by the manager's loyalty
   (loyalty × 0.025 + 0.5); the deserved squad status (`get_deserved_squad_status`) and the offered one (`p5`, 1..7:
   ×1.54 down to ×0.002); the compared reputation (stadium, division, competition, manager, finances, weighted by
   `ambition` and `professionalism`); language fluency; nations and distance (cities within 40/80 km), with
   **adaptability** and **ambition**; a loan from another nation ×0.45/×0.08; time at the club by **loyalty**
   ((loyalty + 8) × 15 days without a grievance: 0); the nations' development; a young player whose contract runs to
   the end of the season does not leave.
6. **The wage gate** (only with `wage` ≠ 0, no loan and a will ≥ 1.75): the default wage
   (`default_staff_contract`, 0x864550) × 3.5 / will (minimum 0.9) against the club's cap (`finance_get_max_wage`,
   0x5968d0): demand × 0.65 above the cap → 0; between the cap and the demand, a mix by discontent (grievances,
   listing, happiness, morale, 0..20).

Attributes read: the person's `ambition` (+0x57), `loyalty` (+0x59), `pressure` (+0x5a), `professionalism` (+0x5b),
`adaptability` (+0x56), `int_apps` (+0x22); the new manager's `loyalty` (human club). **No `non_player` attribute**
(only the current manager's `CURRENT_REPUTATION`). Constants: 3.5 = `rdata::F64_3_5` (0x968e38, used in 25 places of
the exe, the match engine included: do not patch `.rdata`); 2.75 = `F64_2_75` (0x9690a8); 1.75 = `F64_1_75`
(0x968f98).

Caller thresholds: 0x4ce4e0 picks above 2.75 and 3.15 (`F64_3_15`, 0x968e58); `try_loan_offer_for_player` requires
3.5; `make_club_bids` uses 3.15 (`F64_3_15`).

### 2.2 The player picks among contract offers (0x4ce4e0, original) [inferred]

Per person, when the type 3 decision date is due (`process_due_decision_dates`, daily):

1. k = (professionalism − ambition − 9 + loyalty) × 1/300 (0x968e60).
2. With a pending transfer offer (0x8cd150): postpones the decision by 2 days and leaves.
3. For each of the person's offers in the list (+0x34) ready to decide: an offer without a transfer (+0x59 = −1) from
   a club that is not the contract's and type ≠ 5: removed. State 0:
   - type 5: will = 0x846920(…, loan = 1, status 8); type 6: with the status from `get_squad_status_for_club`;
   - the others: the desired contract at the club (0x85fc50 with −1, the offer's status and job +0x61); ratio =
     value(offer against desired) / value(desired against desired) − k (`calc_contract_value`, 0x866830); the will by
     0x846920 (player) or 0x844780 (staff; the average of both when the job is a player's); **renewal** (an offer from
     the current club): ratio ≥ 0.99 (0x968e50) and will ≤ 3.15 → will = 3.15; ratio < 0.6 (0x968e48) → 0;
   - refusal: a player job without a player sheet (0x7a6e20 false), or ratio > 0.5 needing a work permit (0x8c7560)
     with fewer than 90 days;
   - score = will × ratio, with the cuts 0.9 (0x9689d8), 0.8, 0.7 and 3.5; the highest above 2.75 is kept.
   State 2 (already accepted): no other one.
4. The chosen one becomes state 2 (news 0x7549b0, `set_agreed_contract_flag` 0x8c5410); the others state 1 (news
   0x4d49a0, or `handle_player_rejected_terms` 0x8c28c0 when tied to a transfer). None chosen and an offer from his
   own club: mark 0x80 in byte +0xb of the 79-byte record. For the chosen one (types ≠ 5, 6) with a score above
   0x9688b0, draws by loyalty (0x90cfc0) [inferred].

### 2.3 Wage demand and length [proved where in Rust; inferred elsewhere]

- **0x85fc50 `set_to_desired_contract`** (original, 10,896 bytes): the contract the person wants at the club. Reads
  `ambition`, `loyalty`, `pressure`, `professionalism`, `int_apps`; the player value (`calc_player_value` 0x4d0c60,
  ×7, which uses the manager's `JudgingPotential`, §3); the default wage (0x864550, ×3); the will (0x846920 and
  0x844780); the cap (`get_max_wage`, ×4); the window (0x8cf470, 0x8cf0e0); the month of the season end (0x4d45f0); a
  draw. Writes type 2 at +0x35, the status at +0x4f (high nibble), the wage/value at +8/+0xc, clauses and the end date.
  [inferred] An absolute cap of 250,000 in the staff branch (`fVar27 = 250000.0`). It is the best place to change the
  wage demand without porting: a detour that calls the original and adjusts +0xc/+8 on the way out.
- **0x864550 `default_staff_contract`** and **0x862720 `calc_unset_default_wage`** (originals): the default wage by
  ambition, loyalty, pressure, the major nation (0x549e50), the season (0x547770) and the cap (0x5968d0). [inferred]
- **0x4cda70 `offer_contract`** (Rust) [proved]: the club's offer = the desired one × factor; minimum release fee by
  the offer (bands 50 thousand / 1 million / 5 million); cap = `get_max_wage` × (factor + 1) × 0.5 (status 11) or
  `get_max_staff_wage`; **length by age** (age in days × k − a fitness/professionalism weight: above 34 the end next
  year, above 31 in 2 years, above 29 by the age year); the total limited above 30; an injury over 70 days: 0; with
  the FIFA rules, at most 5 years; above the cap × 1.4: 0.
- **0x866830 `calc_contract_value`** (item **mkd_1**) [proved by the disassembly]: wage + bonuses by position (mask
  0x5b7d20: goalkeeper clean sheet × 0.8; defender goal × 0.3 and assist × 0.2; midfielder × 0.4 and × 0.7; attacker ×
  0.8 and × 0.5) + `contract::UNKNOWN` / (days × 0.2222); world reputation > 4000 and reputation² × 0.01 > +8: half;
  each active clause multiplies (1.01..2 by the fees, below 1 M, 5 M (7.5 M in case 0) or above, and by age: < 24,
  24..28, > 28); length factor (0.75 / 0.875 / 1, × (1 − a × 0.01) past the position's age band, bands 33/31/29/28)
  raised to the difference of the end years, within 0.5..2.
- **0x866620 `get_contract_satisfaction`** (Rust) [proved]: 100 × value(current) / value(desired), with the allowance
  for a recent signing (15 × loyalty + 100 days); < 50 → 1, < 80 → 3, otherwise 2. Daily with random(50) = 0.

### 2.4 Renewals and contract end (0x4c7ac0, original) [inferred from the call list]

Daily. For each contract: the end band (`get_expiry_band`, 0..14, ×11) against the dates +7..+182 days; retirement
(0x7a7eb0, ×10); human club (×12); the AI renewal offer (`offer_contract`, ×9); `wants_to_leave` (0x866f60); pending
transfer offers (0x8c5cf0, 0x8c5e40); Bosman (0x8d0a70, ×5; a nation with free transfers, 0x544670); FIFA protected
period (0x868210); active clause (0x867b50); contract end news (0x4d6790 ×12, 0x4da590 ×5, 0x4d6d70); the free
player goes onto the clubs' lists (`add_player_to_club_shortlists` 0x842c00, ×4) and into the free transfer queue
(0x8c16b0) or is released at once (`free_transfer_staff` 0x8c1af0); the loan end (0x4d07e0); and, at the end, the
decision dates (0x4cd9c0). Attributes: only `loyalty` (+0x59, ×2).

### 2.5 Clauses [proved where in Rust]

- Relegation: `trigger_relegation_clauses` (Rust) moves `RELEGATION_RC` 1 → 2 for the 50 of the squad of the
  relegated club and its B team, with news. Non-promotion: 0x4cced0 (original), called on promotion/relegation
  (0x689c80) and in Argentina. `reset_division_clauses` 0x4cd120 (original) on a division change.
- Active: `has_active_release_clause` = non-promotion 2, minimum fee 1 or relegation 2. The seller must accept
  (`selling_club_must_accept`) with a fee ≥ the release fee (reason 0x1b) or a seller in crisis (0x1c). The player's
  value is capped by the release fee (`cap_value_at_release_fee`).
- Next-sale percentage (13 bytes, vector sorted by 0x8e54d0): paid from 1000 up, on the completed transfer. Appearance
  money: 1, 10, 20… 50 games for the club or the national side, after each match.
- Screens only: `is_clause_allowed` (by the league's promotion/relegation rule) and `add_release_clause` (a minimum
  fee clause excludes the others).

### 2.6 Free agents [proved where in Rust]

Who becomes free: the contract end in 0x4c7ac0 (Bosman by the nation, 0x8d0a70) and those released through the queue
(0x8c16b0 → `process_free_transfer_queue`, on the transfers' day: at a human club the board may block it, a draw adds
the manager to the disliked staff, compensation). Who signs them: the clubs' lists (`add_player_to_club_shortlists`,
`make_club_bids` 0x84a020; without a club the will gets × 1.75 and the base 2.75 + min(12, days² / 900)). At what
wage: the desired contract (0x85fc50) against the cap (`get_max_wage`).

## 3. Chairman's and staff's attributes in the market (the requester's question)

The requester's search **was not complete**: Patience, Resources, Youngsters, Judgement, JudgingPotential and
Interference are read in the market. ManHandling is not. Scan of the whole `.text` (byte reads of
`[non_player_sheet + d]`), each market hit checked in the disassembly [proved]:

| Attribute (+) | Where (instruction address) | Routine | In Rust | Effect |
|---|---|---|---|---|
| Patience (+0x1d), manager and chairman | 0x83bf92, 0x83bfd8 | `update_club_target_reputation` 0x83bf40 | yes | the signings' target reputation × (1 + (ambition − patience) × 0.0125 [manager] and / 60 [chairman]) |
| Resources (+0x20), chairman | 0x83c016 | same | yes | × (1 + (resources − 10) × 0.025) |
| Business (+0xf), chairman | 0x597a0d | `Finance::get_max_wage` 0x5968d0 | yes | < 6: wage cap × 1.05 (× 1.1 with a manager of ambition > 15 and reputation > 7250); > 15: × 0.95 |
| Resources (+0x20), chairman | 0x597a6e | same | yes | > 15 with the chairman's ambition > 15 (and +0x82 = 0): × 1.1 / × 1.125 / × 1.05 by the club's reputation (2750, 5750) and the player's (7250) |
| Business (+0xf), chairman | 0x598b24 | `Finance::get_transfer_budget` 0x598800 | no | budget divisor = business + 6 (+ (15 − ambition) with ambition > 15; 16 without a chairman; +5 in one nation) |
| Youngsters (+0x22), manager | 0x8400ae | `fill_shortlist_slot` 0x83f3a0 (rest at 0x840084) | no | a non-loan slot, under 20 with home reputation (+9) < 3250 and current club < 3250: goes on only if edi mod (25 − youngsters) = 0 (no manager, mod 15); youngsters 20 lets 1 in 5 through, 1 lets 1 in 24 [the test proved; what edi is, inferred: a counter or id] |
| Youngsters (+0x22), manager | 0x4d3ccb | `contract_manager_calc_asking_price` 0x4d1600/0x4d2070 | no | AI club (0x600a50 = 0): adds (youngsters − 10) to a threshold compared with the player's potential ability (sheet +7); below it, (potential − threshold) × 0.5 enters the price [the term proved; its role in the price, inferred] |
| JudgingPotential (+0x18), manager | via `perceived_ability` 0x6e86d0 (branch `kind` ≠ 2) | `calc_player_value` 0x4d0c60; `value_player_for_club` 0x840d80 (`young_player_adjustment`, under 27) | yes | the potential ability the manager sees enters the player value and the list |
| Judgement (+0x17) | 0x6e86d0 (`kind` = 2); `know_person` 0x5b0a28 | `perceived_ability`; fog of war (`FOG_OF_WAR_MAP::raise_knowledge`) | yes | the current ability seen; the observer's (scout's) knowledge rises |
| Interference (+0x16), chairman | 0x8c9a26 | `move_player_to_club` 0x8c9720 | no | a sale above 2.5 million and above (resources + ambition + 750) × club reputation: with interference > random(30), the chairman adds the player to his favoured (0x547a10) |
| Resources (+0x20), chairman | 0x8c99f6 | same | no | the threshold above |
| Interference (+0x16), chairman | 0x867941 | `apply_event_flags` 0x867700 | no | with interference > random(30), the chairman comes to dislike him (0x547af0) |
| Interference (+0x16), chairman | 0x58772c | `news_create_not_for_sale_story` 0x587600 | yes | only the "not for sale" news |
| ManHandling (+0x19) | — | none in the market | — | only AWOL, fines, media, screens, `score_staff_for_job` |

Outside the player market, in the **staff market** (`score_staff_for_job` 0x69d1b0, hiring managers and assistants,
`manager_manager.cpp`): Judgement, JudgingPotential, Youngsters, ManHandling, Interference and Patience (reads in
0x69d1b0..). And `game_daily_staff_update` 0x5ce2d0 reads Youngsters, ManHandling, Patience, Judgement and
Interference (daily morale; it calls the contract satisfaction).

## 4. Constants (read from the exe)

0x846920: 3.5 (0x968e38), 2.75 (0x9690a8), 1.75 (0x968f98), 3.15 (0x968e58). 0x4ce4e0: 1/300 (0x968e60), 3.15
(0x968e58), 0.99 (0x968e50), 0.6 (0x968e48), 0.9 (0x9689d8), 0.8 (0x9689b0), 0.7 (0x968e40), 0.5 (0x968968).
0x866830: 0.8 / 0.3 / 0.2 / 0.4 / 0.7 / 0.5 (f64), 0.2222222 (f32 0x96ff84), f32 multipliers 1.01 (0x96ff7c), 1.05
(0x96d3b8), 1.1 (0x96a33c), 1.15 (0x96d390), 1.2 (0x96d7bc), 1.25 (0x96b010), 1.45 (0x96ff74), 1.5 (0x96b004), 1.7
(0x96ff78), 1.75 (0x96b00c), 1.8 (0x96d7b4), 1.95 (0x96ff80), 1.25 f64 (0x968928). 0x5968d0 (chairman): 1.05
(0x9689b8), 1.1 (0x9689c0), 0.95 (0x9689d0), 1.125 (0x96a5a8). Almost all are shared by dozens of routines: changing
`.rdata` changes the whole game.

## 5. Defects of the original

1. `calc_contract_value` (0x866830): without an offer it reads the offer's clauses through a null pointer
   (0x866df6); no caller passes null [proved]. Without a player sheet the age stands for "band − age" (a = age, 5
   after the limit) [proved]. The non-promotion clause (case 0) looks at the offer's minimum fee clause and uses the
   7.5 million band, the others 5 million [proved; intent inferred].
2. `staff_interest_in_joining_club`: at his own club it divides by a stack float nobody wrote (no effect, the path
   does not use the result; already noted in the Rust).
3. `get_max_wage`: a chairman with **little** business sense (< 6) pays **more** (× 1.05/× 1.1) and a good one (> 15)
   pays less (× 0.95) [proved]; looks inverted [inferred].
4. Rust documentation: the header of `young_player_adjustment` (shortlist_manager.rs) says "current ability seen", but
   it calls `perceived_ability` with `kind` 1, which is the potential (reads JudgingPotential +0x18) [proved].

## 6. Extension points (the smallest hook that changes the policy)

1. **0x846920 (Rust)**: a factor on the return of `staff_interest_in_joining_club` changes the will on every path
   (bids, loans, renewal, AI lists). For groups (age, reputation, nation), multiply the return; the callers'
   thresholds (2.75, 3.15, 3.5) stay.
2. **0x85fc50 (original)**: a detour that calls the original and adjusts the wage (+0xc) and the value (+8) on the way
   out changes the wage demand without porting the 10,896 bytes.
3. **0x5968d0 (Rust)**: the club's wage cap (and where the chairman weighs in through Business/Resources).
4. **0x866830 (mkd_1)** and **0x866620 (Rust)**: the satisfaction (50/80) and the ratio of the renewal decision.
5. **0x4cda70 (Rust)**: the AI offer (factor, length by age, cap × 1.4).
6. **0x83bf40 (Rust)**: the target reputation, where the chairman's patience and resources come in.
7. **0x6e86d0 (Rust)**: how much the manager sees (Judgement/JudgingPotential).

## 7. Queue items

- **mkd_1** — `STAFF_CONTRACT::calc_contract_value` (0x866830): `levas/fila/mkd_1/`. Compiles with the current engine
  (own copy, no warnings from the item). **Not proved in the game**: the laboratory lock was busy and the memory guard
  refused (2.3 GB free, minimum 4). See `PRONTO.txt`.

Not ported because of size (for another round): 0x85fc50 (10,896 bytes), 0x864550 (8,400), 0x862720 (7,728), 0x844780
(8,608), 0x4c7ac0 (9,376), 0x4ce4e0 (4,032). Screens only (no effect on the automatic career): 0x4d4660, 0x4d4830,
0x4d44e0, 0x867ad0, 0x85f7a0.
