*English version of [secao_a.md](secao_a.md). Both are kept in sync; the Portuguese file is the original.*

# Market, section A: the money side (reader A, 11/11/2026)

How much a club can spend, how much it asks for a player and how much a player is worth to a buyer. Marks:
**[proved]** = read in the disassembly (or in Rust code already compared with the game); **[inferred]** = from
Ghidra's pseudo-C, from the use or from the community map, without proof.

## The routines

| Address | Name (maxevans60's map) | In Rust? | Item |
|---|---|---|---|
| 0x598800 | `Finance::get_transfer_budget` | no → **item `mka_1`** | `get_transfer_budget` |
| 0x4d42d0 | `contract_manager_get_human_asking_price` | no → **item `mka_1`** | `get_human_asking_price` |
| 0x4d1600 | `contract_manager_calc_asking_price` | no; **left out of this round** (11.3 KB of x87, see below) | — |
| 0x840d80 | `Shortlist_Manager::value_player_for_club` | yes (`value_player_for_club`, shortlist_manager.rs) | — |
| 0x4d0c60 | `CONTRACT_MANAGER::calc_player_value` | yes (`calc_player_value`, contract_manager.rs) | — |
| 0x544410 | `non_player_attribute_rating` | yes (`chance_to_know`, fog_of_war.rs) | — |
| 0x5a0590 | `FinanceTable::get_club_finance` | yes | — |
| 0x5986b0 | `Finance::get_wealth_band` | yes | — |
| 0x598370, 0x59cf50 | `update_wage_bill`, `refresh_all_wage_bills` (the wage bill the budget reads) | yes (fin_prm.rs) | — |
| 0x59a5d0, 0x59af40, 0x5987c0, 0x599a20, 0x59b7e0, 0x5999a0 | the income: gate, TV, season tickets, merchandising, prize money | yes (fin_prm.rs) | — |
| 0x599800, 0x599890 | `add_players_sold`, `pay_players_bought` (the season totals the budget reads) | yes | — |
| 0x8d0780, 0x8ca8a0, 0x8cd240 | the compensation, the payment, "the seller must accept" | queued (`cpq_6`) | — |
| 0x8ccb80 | `TRANSFER_MANAGER::get_club_future_transfer_list` | no (another section: transfers) | — |
| 0x4ca300 | `CONTRACT_MANAGER::get_current_contract_offers` | no (another section: contracts) | — |

## 1. The transfer budget: `Finance::get_transfer_budget` (0x598800)

`thiscall(finance, player, add_listed, *adjusted, knowledge, pending)`, `ret 0x14`, returns an **i64** (edx:eax)
**[proved]**. 37 call sites (`refs.py`): the computer's bids (`TRANSFER_OFFER::build_bid` 0x8e5d60 and neighbours
0x8e6330, 0x8e74a2, 0x8e839c, 0x8ea2af, 0x8ecf7f..0x8ee61d), the shortlists (budget per slot, `shortlist_manager.rs`
2486), the board (`manager_manager.rs` 837 and 1820: funds requests, the veto), the finance events
(0x59e329..0x59ed38), the contracts (0x4d753c, 0x4de3a6), the screens (0x463af8, 0x6a3b87, 0x7b6d5a...). Today's Rust
callers pass `(fin, 0, 0, 0, 1, 0)`: no player, no listed players, nothing pending. The computer's bids run every day
of the window for every computer club that buys **[inferred]** from the callers.

The rule, in order **[proved]** (disassembly 0x598800..0x5997f9, 1240 instructions):

1. It starts at the balance (`finance::BALANCE` +0, i64). With `add_listed`: adds the value (`staff::VALUE` +0x52) of
   each of the 50 squad places (`club::SQUAD` +0xd7) whose person is transfer listed (0x5444a0).
2. `*adjusted` = 0.
3. With `pending` and a human club (0x600a50 with 1, 1): the club's future transfers (0x8ccb80, 27-byte rows: person
   +8, sale +0x10, fee +0x14, stage +0x19): a purchase adds its fee to the committed money (except the `player`
   himself); a stage 3 to 6 sale takes its fee off (once per person) and the wage of the contract with the club
   (0x4cf5f0, `contract::WAGE` +0xc) off the wages. Then the open contract offers (0x4ca300, 111-byte copies: person
   +0, fee +8, wage +0xc, state +0x50; state 4 does not count): each adds its wage and its fee; a person already
   contracted to the club (main contract 0x4cf5a0, kept in `G_CONTRACT_LOOKUP_RESULT` 0xae1080) takes his current wage
   off. Both lists are freed (`free`).
4. A club without a league (`club::DIVISION` +0x57), or a league without an object in `G_COMP_OBJECT_TABLE`
   (0xadadfc): returns the value of step 1 (no reserve at all).
5. **The board's multiplier m** (signed byte): a chairman (`club::CHAIRMAN` +0xbf) with a sheet (`staff::NON_PLAYER`
   +0x69): `non_player::BUSINESS` (+0xf) + 6, minus (`staff::AMBITION` +0x57 − 15) when Ambition is above 15; no
   chairman or no sheet: 16. **+5** when the club's nation (`club::NATION` +0x53) is Germany (`KEY_NATION_GERMANY`
   [0x9cf314]). The request said "Business + 6, 16 with no chairman, +5 for the nation at [0x9cf314], an adjustment
   if staff+0x57 > 15": confirmed; the adjustment is the chairman's Ambition and only reduces m.
6. **The reserve:** m × (`finance::WAGE_BILL` +0xc + the wages of step 3 − `finance::INSTALMENT_BALANCE` +0x1c × ⅔
   [0x96a670]) comes off (64-bit _ftol). Weekly wage bill × (Business + 6): a Business 20 chairman keeps 26 weeks of
   wages; a Business 1 one, 7 weeks.
7. **The ground** (`club::STADIUM` +0x69, `stadium::CAPACITY` +0x3c): owned (`club::OWN_STADIUM` +0x6d): reputation
   (`club::REPUTATION` +0x80) above 5750 → 1.25 × seats × m comes off [0x96a5c8 = −1.25]; above 4250 → 1.15
   [0x96a668]; above 1250 and professional (`club::PROFESSIONAL_STATUS` +0x64 = 1) → 1.05 [0x96a660]; else 1 ×.
   Rented: 2 × seats × m. No ground: 2500 × m.
8. **Training** (`club::TRAINING` +0x7f, t): 200·t (t < 6), 500·(t − 2) (t < 16), 2000·(t − 12), × m × 0.25
   [0x9689e0], comes off.
9. **Reputation r:** r × m comes off; attendance (`club::ATTENDANCE` +0x73) × r × m × 0.0005 [0x96a658] (0.001
   [0x96a350] from r ≥ 4000) comes in.
10. With a nation: (the league's reputation, `competition::REPUTATION` +0x69)² × 5000 (England, Italy, Spain), 2500
    (Germany, France) or 750 comes in; r² × m × 0.001 (own ground) or 0.0005 comes in.
11. In a continental competition (`club::CONTINENTAL_COMP_ID` +0x1db ≠ −1) and professional: 300·r × 1 (England,
    Germany, Italy, France, Scotland, Spain), 0.65 (Holland, Sweden, Belgium, Portugal, Turkey), 0.35 (Denmark, Norway)
    or 0.5 comes in; 0 without a nation.
12. **The committed money** S = max(fees of step 3, 0) + max(0, the season's net spending
    (`finance::SEASON_PLAYERS_BOUGHT` +0x110 − `SEASON_PLAYERS_SOLD` +0x118, low dword only) − 8 million); the low
    dword of S comes off. If still positive: below 10% of the balance [0x967880] it becomes that 10% (at most 2.5
    million [0x96a650]); otherwise the **soft cap** Z = r² × 0.5, × q² when q = the manager's reputation
    (`non_player::CURRENT_REPUTATION` +0xa of `club::MANAGER` +0xcf) / r is below 1 (q at least 0.5 [0x968f18, f32]);
    Z at least balance − 10 million (− 15 million in Germany or without a nation); at a PLC (`club::PLC` +0x82), without
    a nation or in Sweden, Norway, the USA, Portugal, Denmark, Finland, Scotland, Turkey or Germany, at most 40 million
    − S (35 in Germany or without a nation); Z at least 50000. Above Z only 10% of the excess is kept.
13. The funds the board granted come in (`G_MANAGER_MANAGER` 0xb67a44 → `board_record::EXTRA_FUNDS_GRANTED` +0x28 of
    the club's 73-byte row).
14. Reputation above 5000: at least (200·r − 1 million) × league reputation / 17 [0x96a640] (and then at least
    50000). Below 3500: at most 500·r (professional) or 250·r. Never negative.
15. **By nation:** Sweden, Norway, Denmark, Wales, the USA, Finland, both Irelands and Argentina: halved (`_alldiv`
    0x945e80), and above 2.5 million 10% of the excess is kept; Belgium and Portugal: 25% above 15 million; Brazil:
    25% above 2.5 million; Germany, France, Scotland and Holland: 25% above 17.5 million. England, Italy and Spain:
    no cap.
16. **The human's knowledge:** with `player`, `knowledge`, attribute masking (`G_ATTRIBUTE_MASKING_ENABLED` 0x9cc270),
    the fog of war (`G_FOG_OF_WAR` 0xae2a60) and a human club (0x600a50 with 0, 1): the manager's knowledge level of
    the player (0x5afde0, 0x5b02f0) gives V: level 3, nothing changes; 2: r² × 0.04; 1: r² × 0.01 (league standard
    below 9 and r below 4250) or 0.02; 0: r² × 0.005 or 0.01. V × (the manager's reputation / r, 0.1 to 1) × (league
    standard × 0.05, 0.25 to 1); with a negative balance, see the defect below. Above V the budget becomes V + an
    eighth of the excess [0x96a628 = −0.125], at most 1.75 V when that passes 125000, and `*adjusted` = 1.

### Defects of the original
- **[proved] Negative balance in step 16:** with a player sheet, V becomes `_ftol(ratio × 0.65)` (the player's
  reputation over the club's, 0.15 to 0.85): V is lost and becomes 0. The budget becomes an eighth of itself when that
  is at most 125000, otherwise **0** (the cap 1.75 × 0). For a non-player, V × 0.65 (what looks like the intent)
  **[inferred]**.
- **[proved] Step 3, a repeated person:** after finding the person in an earlier row, the wage taken off is that of
  the contract of the person in the row after the first match, not of this row.
- **[proved] Step 12:** the season's net spending enters by its low dword only (above 2.1 billion it turns negative
  and vanishes); the committed money S also comes off by its low dword.
- **[inferred]** Step 4 returns the budget with no reserve for a club without a league (background clubs with no
  competition).

### Extension points
- **The multiplier m (step 5):** the only place the board (Business, Ambition) enters. In the Rust copy, one line; in
  the exe, the bytes `add al, 6` (0x598b2a) and `add al, 5` (0x598b56). Changing m moves the wage, ground, training and
  reputation reserves at once.
- **The return (0x5997de / 0x5997ec):** a detour that calls the original and scales the i64 by club group (nation,
  reputation, PLC) changes the policy without rewriting anything. Careful: the shortlists split the budget among the
  slots and the board compares it with 250 × reputation (manager_manager.rs 700).
- **The nation caps (step 15) and the soft cap (step 12):** immediate constants (2.5M, 15M, 17.5M, 40M, 35M) and
  `.rdata` doubles read from the exe (the Rust copy reads them from the exe; a `.rdata` patch works in both).
- **The 10%-of-balance floor:** `0x967880` is the generic 0.1 (used in hundreds of places): changing it only here
  needs the Rust copy.

## 2. The asking price of a human club's player: `contract_manager_get_human_asking_price` (0x4d42d0)

`thiscall(contract manager, seller, buyer, player)`, `ret 0xc` **[proved]**. 9 callers: the computer's bid (0x8e64df,
after checking that the seller is human), the shortlists (0x8402e6, 0x840be3, 0x843387, 0x843551, 0x843afd,
0x84b03e), the media/screen (0x7b6dd5) and 0x8f4b8c. The community map calls the 1st argument `buying_club`: it is the
**seller** (the one 0x8e64c3 tests as human and that goes on as `param_1` of 0x4d1600) **[proved]**.

Rule **[proved]**: the player's main contract (in 0xae1080) gives the club; a club that is not human: the "Error" box
(contract_manager.cpp line 0x1f28) and returns `staff::VALUE`. The contract with that club (0x4cf5f0): none, type
(`contract::CONTRACT_TYPE` +0x35 & 0x3f) ≠ 2 or leaving on a Bosman (`contract::LEAVING_ON_BOSMAN` +0x49): 0. The
floor = the stored value (`staff::VALUE`), limited by the release fee (`contract::RELEASE_FEE` +0x21) when the clause
is active (0x867b50). The price = 0x4d1600(seller, buyer, player, 0, 1, 1), raised to the floor and limited by the
fee. Listed or requesting a move (0x544520), or without bit 4 of `contract::TRANSFER_STATUS` (+0x4e) **[inferred: the
manager set the price]**: the floor. By squad status (`contract::SQUAD_STATUS` +0x4f & 0xf0): 0x10 and 0x50 the
price; 0x20 and 0x30 the mean (signed, truncated); the others, the floor.

Note: inside 0x4d1600, a human seller whose manager is **on holiday** (0x5fd210, 0x5fea50 with option 0) and without
bit 4 returns the stored value at once **[proved: 0x4d16xx]**: in the test career (manager on holiday) the human
club's asking price is the stored value.

**Extension point:** the return; or the floor (`staff::VALUE` +0x52, which the game itself updates).

## 3. The asking price: `contract_manager_calc_asking_price` (0x4d1600) — map, no item

`thiscall(contract manager, seller, buyer, player, *reasons, adjust, flag)`, `ret 0x18`, 0x4d1600..0x4d4258 (11.3 KB,
~3200 instructions, x87 with dozens of `_ftol` whose argument Ghidra hides) **[proved: size and convention]**. 20
callers: the transfer request (0x4d8705, already Rust), the board veto (0x696ec5, Rust), the human's price (0x4d4429),
the computer's bids (0x8e6533, 0x8e6554, 0x8c8c6e, 0x8c8291...), the restrictions check (0x8cd391), the shortlists and
the screens. It is **left out of this round**: a bit-faithful port needs the whole disassembly (the pseudo-C's `_ftol`
calls do not say what they truncate) and does not fit in the time; the map below is **[inferred]** from the pseudo-C,
except where marked.

The rule, broadly:
1. No seller, no contract with him, a contract that is not type 2 or a Bosman leaver: 0 **[proved]**. A human seller
   on holiday without bit 4: the stored value **[proved]**.
2. **League factor** (no visible later use; probably inside an `_ftol`): by `nation::STATE_OF_DEVELOPMENT` (+0x7e,
   1 to 3), `club::PROFESSIONAL_STATUS` (2 → ×2 or ×4, 3 → ×7.5) and (league reputation² × 25 + club reputation) /
   (factor × 1000); gives 1 − (20 − x)² × 0.2 / 60, between 0.1 and 1.1.
3. **Age** above 28: a power factor (0x945ff0).
4. **Happiness and role** (`contract::HAPPINESS` +0x3d, `SQUAD_STATUS` +0x4f >> 4): the shortfall up to 75/50/40/25/0
   for roles 1, 2 and 5, 3, 4, 6; role 7 → happiness −100. A player absent without leave (0x418f60): the seller
   manager's discipline zeroes the terms.
5. **Morale** (`contract::MORALE` +0x44): × 0.00746 (negative) or 0.00333; × 0.8333 without grievances (`GRIEVANCES`
   +0x45 = 0) or 1.2 with.
6. **The value** = `calc_player_value(player, seller, 0, ability seen by the manager (0x6e86d0), 0)` (0x4d1a5a); 0 →
   returns 0 **[proved]**.
7. **Contract running out** (with `adjust`, computer seller, not listed): a factor by role (7.5; 6.0; 5.5; 3.5 as
   f32) minus days to the end (121 to 365) / (365 / (f − 1)); above 2 sets reason 0x100.
8. **High happiness** (≥ 61 reason 0x200; 21..60 reason 0x400): (shortfall/2 + happiness) × 0.01167 + 1.4165, and
   above 49 at a computer club × (… × 0.006 + 0.7).
9. **The seller's finances** (0x5a0590): the balance against the table 0x9a7f48 by reputation / 500 (50 thousand to
   25 million): rich, balance / table; poor, damped. Added to happiness × 0.002 and the observation nibble² × 0.05
   (79-byte record +0xb): ≤ 1 → × 0.2125 + 0.7875, else × 0.25 + 0.75; at least happiness × 0.0025 + 0.5, at most 2.5.
10. **The buyer:** Celtic × Rangers (`g_key_club_glasgow_celtic`/`rangers` 0x9d0118/0x9d011c): 2.3 + …; a derby
    (0x4b02e0) or rivals (`club::RIVAL1..3` +0xb3/+0xb7/+0xbb), same stadium city or not: 1.3 to 2; same division
    (outside 0x9cf764): 1 + … × 0.005 (reasons 8 and 0x10).
11. **Squad importance:** ability above reputation/50 × 17/20, age < 25, role < 7: 1 + … × 0.04 (reason 0x20).
12. **The price** = value + value × (product of the factors, capped 4.25..10 and 6 × the finance factor), between −3
    and 50 million; minimum 1000. Work permit (0x8c7560) and listed: the reasons keep only 0x800. A rich club
    (0x5986b0) with reputation > 4250 and age > 18: adjusted by the wealth band.
13. **Grievances** (`GRIEVANCES` & 0x6b or 0x100) with less than a year of contract: the price goes down towards the
    value.
14. **Big nations** (England, Italy, Spain, Germany, Scotland, Japan, the USA, Holland, Portugal...) with reputation
    ≥ 4001: multipliers by role (1.25; 1.1; 1.4; 1.15; 1.5; 1.75; ×2 up to +2M; ×5 up to +5M) and 1.025..1.1 more when
    the buyer is in the same division; plus the contract flags (0x8676f0) and the staff ones (Loyalty +0x59,
    Professionalism +0x5b, morale).
15. **Age cap** **[proved in the pseudo-C, constants]**: 29 (26 for a goalkeeper, `player_sheet::GOALKEEPER` = 20,
    −3): value + 5M; 30: +4M; 31: +3.25M; 32: +2.5M; 33: +1.75M; 34: +1M; 35: +500k; 36: +200k; 37: +100k; 38 to 45:
    the value.
16. **Portugal** above the value: < 6M → × (price × 1.667e−7 × 0.6 + 0.3); else × 0.9. **South Korea** above the
    value and 50000: an adjustment.
17. **The active release fee** (0x867b50) caps the price **[proved]**.

**Extension points:** the return of 0x4d1600 (all 20 callers go through it; the reasons in `*reasons` explain a
refusal); the value call at 0x4d1a5a (the price is value + premium: scaling the value scales everything); the age cap
(step 15) and the fee (step 17).

## 4. The player's value for a club: `value_player_for_club` (0x840d80, in Rust)

Checked in the Rust (shortlist_manager.rs 1807, compared with the game) **[proved]**: the observer chosen for the
rating is the manager (`.manager()`) or the best of the 7 scouts (`club::SCOUTS` +0x1b3) by
`non_player_attribute_rating(sheet, 6 or 5)` (0x544410 = `chance_to_know`), which is **only the observer's general
ability** (`non_player::CURRENT_ABILITY` +4: x = (CA + 2 × level) × 0.05 + 10, then (x + 0.1x²)/3 + 0.5, between 1 and
20). The level is the constant 6 (flag 0x100) or 5 (flag 0x80, the manager with +5), not the judging attributes
(`JUDGEMENT` +0x17, `JUDGING_POTENTIAL` +0x18); the reputation scope takes 5 or 10 off, and with scope 3/4 and club
reputation < 3500 the scout does not even count. It matches the request. The judging attributes only enter the fog
(`know_person`, fog_of_war.rs, with `judgement()`).

The price the club would pay: the rating (`match_man_score_player_for_selection` with the observer), × 1.05 for a
player listed by another club; the fog (the nibble of +0xb of the 79-byte record) limits it; the cap by position (6 to
13 million) and the fraction; the base value; age above 28/32/19, foreigner (0x53d980); injury proneness; above 1.5
million, a seen potential below 145 pulls it to 1.5 million; the club's division; Portugal and France; with
`ceiling`, at most the slot's +7.

**Extension point:** the `non_player_attribute_rating(…, 6/5)` call is where to change the scout criterion (e.g. use
`JUDGING_POTENTIAL`); the return, to scale what the computer pays.

## 5. The player's value: `calc_player_value` (0x4d0c60, in Rust)

See the header in contract_manager.rs 902: base by position (6 to 13 million), peak age 28/29, the sheet's ratings,
the ability seen by the club's manager × 50 + reputation, the table 0x9a7f30 by age, the club's nation (development,
league standard), the contract days and the Bosman rule, Portugal/France/Korea, the release fee. It is the base of
everything above: 0x4d1600 adds a premium to it and 0x4d42d0 uses it as the floor (through `staff::VALUE`, which the
game stores from it). 58 callers.

## 6. Reputation in prices

- **Club reputation** (`club::REPUTATION` +0x80): in the budget, r in almost every step (r × m, attendance × r, r²,
  300·r, the 5000/3500 limits, the nation caps); in the asking price, the wealth table (r / 500), the 4001, 4250 and
  6751 thresholds and the "big club"; in the value for a club, the scouts' reputation < 3500 cuts.
- **League reputation** (`competition::REPUTATION` +0x69): in the budget (steps 10 and 14) and in 0x4d1600's league
  factor.
- **Player reputation** (`player_sheet::CURRENT_REPUTATION` +0xb, `WORLD_REPUTATION` +0xd): in the value (0x4d0c60)
  and the seen ability (0x6e86d0); in the budget only in the negative-balance defect.
- **Manager reputation** (`non_player::CURRENT_REPUTATION` +0xa): in the budget, the q of the soft cap and of the
  knowledge.

## 7. Item

- **`mka_1`** (`levas/fila/mka_1/`): `get_transfer_budget` (0x598800) and `get_human_asking_price` (0x4d42d0),
  side-by-side comparers from the draw state; mutations: Business + 1 instead of + 6, half the excess at the soft cap,
  the human price floor doubled. The proof is in `PRONTO.txt`.
