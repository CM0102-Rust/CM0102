*Portuguese version: [MAPA_MERCADO.md](MAPA_MERCADO.md).*

# Transfer market map (loans included)

Made on 11/11/2026 for the Rust Tweaks release 1.5 (rebalancing the market), by four readers of the original's disassembly and of the Rust code. Every claim is marked **[proved]** (disassembly or a test) or **[inferred]**. Routines not yet in Rust were ported as queue items (`levas/fila/mka_*`, `mkb_*`, `mkc_*`, `mkd_*`); their proof status is in each item's `PRONTO.txt`. Sources: `levas/reorganizacao/mercado/`.

**Updated 11/11/2026: inferred claims closed** (`levas/reorganizacao/mercado/fechamento_inferidos.en.md`): the [inferred] marks outside the routines still being read (0x4d1600, 0x85fc50, 0x864550, 0x4ce4e0, 0x4c7ac0, 0x862720, 0x844780) were proved in the disassembly or kept with the reason. Main points: the loan stuck in state 8 is a proved defect of the original, without holidays (section C §8); bit 4 of `contract::TRANSFER_STATUS` = automatic asking price, bit 8 = listed at the player's request; the squad statuses (high nibble of +0x4f); the `edi` of the youngsters filter is the person's id + the club's id; the future-transfer +0x10 = 1 when the person leaves; the rules of 0x83cb40 and 0x83f3a0 from items `mkb_2`/`mkb_3`; `get_max_wage` is not inverted.

Corrections found after the sections were written: 0x83f3a0 is called by `allocate_transfer_budget` 0x83b6e0 (0x83bab7), `update_staff_status` 0x843e10 (0x844108) and itself, not by 0x83cb40; 0x83cb40 is now item `mkb_2` and 0x83f3a0 item `mkb_3`; in 0x83cb40 the target base (+0x74, from 0x7bca90) sets the slot states and the cut (base/68 − 35), the rating itself (+0x64) prices the slots.

Sections: [A. The money side](#a) · [B. How the AI buys and sells](#b) · [C. Loans, windows and limits](#c) · [D. The player's side and who decides](#d)


<a id="a"></a>

## Market, section A: the money side (reader A, 11/11/2026)

How much a club can spend, how much it asks for a player and how much a player is worth to a buyer. Marks:
**[proved]** = read in the disassembly (or in Rust code already compared with the game); **[inferred]** = from
Ghidra's pseudo-C, from the use or from the community map, without proof.

### The routines

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

### 1. The transfer budget: `Finance::get_transfer_budget` (0x598800)

`thiscall(finance, player, add_listed, *adjusted, knowledge, pending)`, `ret 0x14`, returns an **i64** (edx:eax)
**[proved]**. 37 call sites (`refs.py`): the computer's bids (`TRANSFER_OFFER::build_bid` 0x8e5d60 and neighbours
0x8e6330, 0x8e74a2, 0x8e839c, 0x8ea2af, 0x8ecf7f..0x8ee61d), the shortlists (budget per slot, `shortlist_manager.rs`
2486), the board (`manager_manager.rs` 837 and 1820: funds requests, the veto), the finance events
(0x59e329..0x59ed38), the contracts (0x4d753c, 0x4de3a6), the screens (0x463af8, 0x6a3b87, 0x7b6d5a...). Today's Rust
callers pass `(fin, 0, 0, 0, 1, 0)`: no player, no listed players, nothing pending. The budget is computed for every
bid built (`TRANSFER_OFFER::build_bid` 0x8e5d60, which calls 0x598800 at 0x8e6330) and in each offer's restrictions
check (0x8cfe2d). The AI's new bids come only from `make_club_bids` 0x84a020 (single call at 0x5cdd6d, in the day loop),
at the rhythm of section B §2 (every day, or only when (random(2) + id) % 7 = 0), **without looking at the window**:
reason 9 (outside the window) of `can_club_bid_for_player` is accepted (offers outside the window are held by 0x8c4e10)
**[proved: refs of 0x598800 and of `add_transfer_offer` 0x8be300 (0x84a728 in 0x84a020; the others are reviews
0x8c0810/0x8c0f40, the loan 0x8bed60, 0x8bf060 and screens); 0x84a020 in Rust compared (cqe_3); the rhythm at
0x5cdc6e..0x5cdd6d]**.

The rule, in order **[proved]** (disassembly 0x598800..0x5997f9, 1240 instructions):

1. It starts at the balance (`finance::BALANCE` +0, i64). With `add_listed`: adds the value (`staff::VALUE` +0x52) of
   each of the 50 squad places (`club::SQUAD` +0xd7) whose person is transfer listed (0x5444a0).
2. `*adjusted` = 0.
3. With `pending` and a human club (0x600a50 with 1, 1): the club's future transfers (0x8ccb80, 27-byte rows: person
   +8, sale +0x10, fee +0x14, stage +0x19; see the +0x10 note below): a purchase adds its fee to the committed money (except the `player`
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

**The future-transfer field +0x10 [proved]:** 1 when the person **leaves** the club asked about, 0 when he **arrives**.
In `get_club_future_transfer_list` 0x8ccb80, for each open offer of the club (0x8ccbf3..0x8ccc1b, those it made; plus
the ones received, from 0x8c6100 at 0x8ccc4d; loans, `offer::OFFER_TYPE` +0x1f = 1, are skipped at 0x8ccc78), the
target's row (0x8ccdb7..0x8cce4d): +8 = the target (`offer+0xc`); if the club is the buyer (`[club] = offer+8`,
0x8ccdca..0x8ccdcf), +0xc = the target's current club (−1 none) and +0x10 = 0 (0x8ccdda/0x8cce02); otherwise +0xc = the
buyer and +0x10 = 1 (0x8cce0b..0x8cce0e); +0x14 = the fee (`offer+0x1b`); +0x18 = 4 for a pre-contract without a fee
(completion 3/4), 1 with an exchange, else 0; +0x19 = the stage through the table 0x8cd134/0x8cd114 (1 to 6, 0
otherwise); +0x1a = `offer+0x2d`; +0 the date. The exchange players' rows (0x8cce89..0x8ccf33) are the reverse: for the
buying club, +0x10 = 1 (it gives the exchange player away), otherwise 0; fee 0, +0x18 = 1. In step 3, +0x10 = 0 adds
the fee to the purchases; ≠ 0 takes the sale branch in stages 3 to 6.

#### Defects of the original
- **[proved] Negative balance in step 16:** with a player sheet, V becomes `_ftol(ratio × 0.65)` (the player's
  reputation over the club's, 0.15 to 0.85): V is lost and becomes 0. The budget becomes an eighth of itself when that
  is at most 125000, otherwise **0** (the cap 1.75 × 0). Without a player sheet, V × 0.65 **[proved]**: 0x59970b
  (+0x61 null) → 0x599759 `fild V` → 0x59975d `fmul [0x96a5d0]` (= 0.65, read from the exe) → `_ftol` 0x599763; with a
  sheet, the reputation ratio (0.15 [0x968f20] to 0.85 [0x969520]) takes V's place (0x599712..0x599757) and is
  multiplied by the same 0.65. That the intent was V × ratio × 0.65 is still **[inferred]** (intent cannot be proved
  from the code).
- **[proved] Step 3, a repeated person:** after finding the person in an earlier row, the wage taken off is that of
  the contract of the person in the row after the first match, not of this row.
- **[proved] Step 12:** the season's net spending enters by its low dword only (above 2.1 billion it turns negative
  and vanishes); the committed money S also comes off by its low dword.
- **[proved]** Step 4 returns the budget with no reserve for a club without a league: `club::DIVISION` (+0x57) null
  (0x598af3..0x598afc) or no object in `G_COMP_OBJECT_TABLE` (0x598b02..0x598b0d) jumps to the return 0x5997ec with step
  1's value; it is the same test the shortlist uses for "league not loaded" (0x83cc68..0x83cc86). Which clubs these are
  in a given career is still **[inferred]** (it depends on the loaded leagues; not counted).

#### Extension points
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

### 2. The asking price of a human club's player: `contract_manager_get_human_asking_price` (0x4d42d0)

`thiscall(contract manager, seller, buyer, player)`, `ret 0xc` **[proved]**. 9 callers: the computer's bid (0x8e64df,
after checking that the seller is human), the shortlists (0x8402e6, 0x840be3, 0x843387, 0x843551, 0x843afd,
0x84b03e), the media/screen (0x7b6dd5) and 0x8f4b8c. The community map calls the 1st argument `buying_club`: it is the
**seller** (the one 0x8e64c3 tests as human and that goes on as `param_1` of 0x4d1600) **[proved]**.

Rule **[proved]**: the player's main contract (in 0xae1080) gives the club; a club that is not human: the "Error" box
(contract_manager.cpp line 0x1f28) and returns `staff::VALUE`. The contract with that club (0x4cf5f0): none, type
(`contract::CONTRACT_TYPE` +0x35 & 0x3f) ≠ 2 or leaving on a Bosman (`contract::LEAVING_ON_BOSMAN` +0x49): 0. The
floor = the stored value (`staff::VALUE`), limited by the release fee (`contract::RELEASE_FEE` +0x21) when the clause
is active (0x867b50). The price = 0x4d1600(seller, buyer, player, 0, 1, 1), raised to the floor and limited by the
fee. Listed or requesting a move (0x544520), or without bit 4 of `contract::TRANSFER_STATUS` (+0x4e): the floor. **Bit
4 = automatic asking price [proved]**: without it the manager set the price by hand, and `staff::VALUE` (+0x52) is that
price. By squad status (`contract::SQUAD_STATUS` +0x4f & 0xf0): **0x10 Key and 0x50 Prospect** the price; **0x20 First
Team and 0x30 Rotation** the mean (signed, truncated); 0 Undefined, 0x40 Backup, 0x60 Youngster and 0x70+ Not Needed,
the floor **[proved]**.

- **Proof of bit 4:** the "Set Staff Transfer Status" screen (0x8f6770, transfer_screens.cpp): control 0xa comes from
  bit 4 (0x8f6923..0x8f6936: `and edx, 4; shr edx, 2`); the "Auto<…automatic asking price>" button (text at 0x8f6fe9)
  is event 5 of handler 0x8f7640, which toggles control 0xa and, when switching it on, puts `calc_player_value`
  0x4d0c60 in control 1 (the price) (0x8f772b..0x8f77bb); editing the price (event 0xb) switches control 0xa off
  (0x8f780d..0x8f781e); on confirmation control 0xa becomes bit 4 (0x8f80b0..0x8f80da). A new contract is born with
  +0x4e = 4 (0x866f14, in 0x866ee0, called by 0x4cfb13/0x4cfccb/0x4cfef4). With bit 4, the day (0x4c7f9e..0x4c7fbe)
  and 0x84ae17..0x84ae30 recompute `staff::VALUE` with 0x4d0c60; without it, they do not. `set_player_squad_status`
  0x8ce0e0 (screens only: 0x8efc88, 0x8f08b5, 0x8f0902, 0x8f0af6, 0x8f7fbc) writes the price at +0x52 (0x8ce1e9) and
  clears bit 4 when its 8th argument ≠ 0 and the club is not in the crisis state (0x8ce8af..0x8ce8bf).
- **Proof of the statuses (high nibble of +0x4f):** 0x867f6d..0x867ffa (staff_contracts.cpp) reads `+0x4f >> 4` and
  jumps through the table 0x868000 (0x867f81 Undefined, 0x867f90 Key, 0x867f97 First Team, 0x867fa6 Rotation, 0x867fb5
  Backup, 0x867fbc Prospect, 0x867fcb Youngster; above 6, "Not Needed" at 0x867fda). So 0x84af00's "status 6 or 7" is
  Youngster/Not Needed, and 0x83cb40 sets Youngster (6) under 20 and Not Needed (7) otherwise.

Note: inside 0x4d1600, a human seller whose manager is **on holiday** (0x5fd210, 0x5fea50 with option 0) and without
bit 4 returns the stored value at once **[proved: 0x4d16xx]**: in the test career (manager on holiday) the human
club's asking price is the stored value.

**Extension point:** the return; or the floor (`staff::VALUE` +0x52, which the game itself updates).

### 3. The asking price: `contract_manager_calc_asking_price` (0x4d1600) — map, no item

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

### 4. The player's value for a club: `value_player_for_club` (0x840d80, in Rust)

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

### 5. The player's value: `calc_player_value` (0x4d0c60, in Rust)

See the header in contract_manager.rs 902: base by position (6 to 13 million), peak age 28/29, the sheet's ratings,
the ability seen by the club's manager × 50 + reputation, the table 0x9a7f30 by age, the club's nation (development,
league standard), the contract days and the Bosman rule, Portugal/France/Korea, the release fee. It is the base of
everything above: 0x4d1600 adds a premium to it and 0x4d42d0 uses it as the floor (through `staff::VALUE`, which the
game stores from it). 58 callers.

### 6. Reputation in prices

- **Club reputation** (`club::REPUTATION` +0x80): in the budget, r in almost every step (r × m, attendance × r, r²,
  300·r, the 5000/3500 limits, the nation caps); in the asking price, the wealth table (r / 500), the 4001, 4250 and
  6751 thresholds and the "big club"; in the value for a club, the scouts' reputation < 3500 cuts.
- **League reputation** (`competition::REPUTATION` +0x69): in the budget (steps 10 and 14) and in 0x4d1600's league
  factor.
- **Player reputation** (`player_sheet::CURRENT_REPUTATION` +0xb, `WORLD_REPUTATION` +0xd): in the value (0x4d0c60)
  and the seen ability (0x6e86d0); in the budget only in the negative-balance defect.
- **Manager reputation** (`non_player::CURRENT_REPUTATION` +0xa): in the budget, the q of the soft cap and of the
  knowledge.

### 7. Item

- **`mka_1`** (`levas/fila/mka_1/`): `get_transfer_budget` (0x598800) and `get_human_asking_price` (0x4d42d0),
  side-by-side comparers from the draw state; mutations: Business + 1 instead of + 6, half the excess at the soft cap,
  the human price floor doubled. The proof is in `PRONTO.txt`.


<a id="b"></a>

## Market, section B: how the AI buys and sells (2026-11-11, reader B)

Part of the transfer market map asked for by Rust Tweaks (release 1.5; [BRIEF.en.md](BRIEF.en.md)). Addresses are VAs
of `cm0102.exe` 3.9.68. **[proved]** = read in the disassembly (`desmonta.py`) or in Rust code already compared with
the original; **[inferred]** = read only in Ghidra's pseudo-C (`pacotes/0x83cb40`, `pacotes/0x83f3a0`) or concluded
without direct proof. Field names are those of `rust/src/motor/fields.rs` and `records.rs`.

### 1. The routines

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

### 2. Who calls and when

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
  5/7 of those clubs **never** bid from their shortlist [proved: 0x5cdc6e..0x5cdc7b, random(2) once a day before the
  loop; 0x5cdd32..0x5cdd42 and 0x5cdd58.., (r + id) % 7. Effect proved for the shortlist: the shortlist's new bids and
  loan requests come only from 0x84a020 (0x842740 has a single caller, 0x84ab5a, inside 0x84a020; the AI's new bids,
  0x84a728); those clubs can still receive/answer offers and review their own. The intent ("once a week") is still
  inferred: it cannot be proved from the code].
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
  `move_player_to_club` 0x8c9720, `set_player_squad_status` 0x8ce0e0 and the person screens (2). The one the community
  map puts in `CONTRACT_MANAGER::save_to_disk` 0x4dc3a0 (0x4dd0a1) is the map's boundary [proved]: the 0x4dc3a0 range
  (3408 bytes) covers several routines; 0x4dd0a1 is in the handler 0x4dcd70 of the **"Transfer Request"** dialog
  (builder 0x4dcb30; both registered at 0x4dcab5/0x4dcaba; texts "Put <…> on the transfer list at his own request ?",
  "Yes", "No"). On "Yes": it clears the loan list (0x4dd060..0x4dd069), **sets bit 8** of +0x4e (0x4dd06c..0x4dd075;
  bit 8 = listed at the player's request), event 0xc (0x544710), forced showcase (0x842c00 with 1, at 0x4dd0a1) and
  0x4d71b0.
- **The AI transfer list**: `transfer_list_player` 0x842320 has 3 call sites: 0x83dc42 (inside `assess_club_squad`, the
  only AI one) and the screens 0x8f8030/0x8f8607 [proved]. **The loan list**: the only `or byte [r+0x4e], 2` of `.text`
  are 0x83e49c and 0x83e651 (`assess_club_squad`) and 0x8442c4 (`add_to_loan_list`, called only by the screens:
  0x8f806c, 0x8f8538); plus two through a register in `assess_club_squad` (0x83dd42, 0x83dd98) [proved by the scan of
  the whole `.text` for **byte** writes to `[reg + 0x4e]` (mov/or/and/add, immediate or register): bit 2 is only set at
  these five points; the other register writes are contract copies (0x4cc6dc, 0x4cc903, 0x4ce39f, 0x4cf9d8, 0x4cfc10,
  0x4cfdb9, 0x4d000f, 0x4d0546, 0x4d06a5, 0x5fbb51), clears (0x83d287 `& 0xfc`, 0x8442ef `& 0xfd`,
  0x4d8747/0x4dd069/0x784848/0x8f80a1 `& 0xfd`, `& 0xf7`, `& 0xfe`), constructor zeros (0x85e458, 0x85e4f4, 0x85e6a4,
  0x85e9db, 0x866eeb) or touch bits 1, 4 and 8. Still inferred: the wider writes covering +0x4e (265 `mov dword/word`
  at +0x4b..+0x4e in all structures) were not looked at one by one].

### 3. The rules

#### 3.1 The squad assessment (0x83cb40, original)

`assess_club_squad(this, entry, club, daily, reset)`, thiscall, ret 0x10. Order [proved: disassembly
0x83cb40..0x83f3a0 (3168 lines), read whole for item `mkb_2`; the cited points checked; where there is no address, the
text is the `mkb_2` header]:

1. DAYS (+8 of the entry) = 0 (0x83cbc4); reset is forced when slot 0's +0x23 is 0xff (0x83cbb4..0x83cbbb). Leaves with
   ACTIVE 0 (0x83cc16) if the club is a national team (0x53b900), without a nation, human with daily (0x600a50 with 0,
   1), `club::PROFESSIONAL_STATUS` = 3, or when `club_get_linked_b_team` 0x540a50 finds the pair and says this club **is
   the B one** (0x83cc04..0x83cc14: return ≠ 0 and the "is the A" flag 0): the A club runs the list [proved].
2. No bid in a slot: if ACTIVE was 2, it becomes 1 and reset = 1. If the nation's league is not loaded or the club's
   league is not active in `[0xadadfc]`, and the reputation < 4000: if random(reputation + 1000) < random(5000), ACTIVE =
   0 and leaves (small clubs of unloaded leagues switch their shortlist off) [proved: 0x83cc6b..0x83ccab].
3. With reset: the 40 slots are rebuilt (`shortlist_set_up_formation_slots` 0x83c090; each slot's status by its index
   and by the target rating `local_840` from `player_search_get_club_target_rating` 0x7bca90: up to 6000 or 8500, with
   or without a continental competition). Without reset: the slots are only sorted, `qsort(slots, 40, 0x25, 0x83b6c0)`
   at 0x83cfe5..0x83cff1 (order by the slot index) [proved].
4. A slot without a target, or with a target, no offer (`OFFER_REF` −1) and a passed date or retired (job 0x10), loses
   target, replacement and ceiling (the target, if in the staff pairs, gets event 3 in 0x843e10); its position goes to
   its status's list (2 and 3, 4, 5) [mkb_2, 0x83cff9..0x83d1c7].
5. **The squad** (`database_collect_club_squad` 0x541f90, 50): for each player with a sheet, job ≠ 0x10, not on the
   club's shortlist, with a contract at the club, no future transfer (0x8cca10) and no `contract::LEAVING_ON_BOSMAN`: at
   an AI club with no manager on holiday, **clears the transfer and loan marks** (`contract::TRANSFER_STATUS` & 0xfc),
   sets the status (`contract::SQUAD_STATUS` high nibble) 6 under 20 or 7 (AI club) and sends him to the first team
   (`move_to_first_team` 0x4cc0b0) [proved: 0x83d281..0x83d2d7]. Injury (`Injury::get_player_injury` 0x62ed70): the
   threshold is **150 days** (0x83d32d); **30** when random(recurrence level 0x62ece0 + 1) ≠ 0 (0x83d33e..0x83d34b);
   otherwise, injured with a score (0x6e6340) ≥ the target: **400** (0x83d37f..0x83d387); score ≥ ¾ of the target
   (0x969040 = 0.75): **300** (0x83d391..0x83d3a8). Days left ≥ threshold, or a retirement announcement (0x7a7ed0),
   sends him to the reserves (`move_to_reserves` 0x4cc0f0, at 0x83d3f4; with a B team only above 300 days,
   0x83d3d8..0x83d3ea) and, at a human club, clears the status (0x83d40e). The others go to the rest or to the list of
   those on loan elsewhere (0x4cf6d0 at 0x83d41b), and with `reset` happiness goes to 0 [proved].
6. Selection by status (`match_man_assign_players_to_positions` 0x6e2000, calls at 0x83d512, 0x83d636, 0x83d7f9): the
   rest goes to the status 2 and 3 positions, then 4; those on loan elsewhere join the rest; those older than
   `player_sheet::GOALKEEPER` / 10 + 22 (24 for a goalkeeper rated 20, 22 for the others) leave it (the swapped-in one
   is looked at again); the rest goes to the status 5 positions; the old ones come back [mkb_2, 0x83d474..0x83d8ac].
7. **The rest** (the end of the squad), for each player of the club, without a loan contract and not queued for a free
   transfer (0x8c1870) [mkb_2, 0x83d8bf..0x83de6b]: the threshold is ftol((age × 0.13 − 1.98) × base × 0.02), the cut
   base/68 − 35 and the level ftol(reputation × 0.25 + PA × 12.5 + CA × 12.5 + club reputation × 0.25); "surplus" =
   judged below the threshold (0x6e86d0 mode 1, 0x83da02) and a type 2 contract with less than professionalism +
   pressure + 450 days at the club or an end 450 days or more away, or `staff::VALUE` ≥ reputation × 50, or the value ≥
   the wages left (days × wage / 30). With a B team or at a human club, under 27 (goalkeeper 20) or 23, status 5 when ¾
   of the level passes the threshold + 20, else 6. The points already proved:
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
8. The slots receive the chosen [mkb_2, 0x83e185..0x83e66f]: the replacement is the pick; with no manager on holiday
   the contract takes the slot's status, except in list 2/3 over statuses 1 and 2; the slot date comes from his arrival
   (+120 days, with `reset` and a manager). Status 4: at slot 24, with a B team or at a human club and not a big club,
   the pick (not the manager) goes to the reserves. In lists 4 and 5 (reserves, rest):
   - list 4: **loan** if fatigue (`Injury::get_fatigue` 0x62c3d0) < −1000, `CURRENT_REPUTATION` < 4500,
     `player_sheet::WORK_RATE` (+0x44) < 3250 and happiness < −50 [proved: 0x83e46a..0x83e49c]. **Defect:** +0x44 is a
     signed byte compared with 3250 (0xcb2): always true; probably meant `WORLD_REPUTATION` (+0xd, word) [code proved:
     `movsx edx, byte [eax+0x44]; cmp edx, 0xcb2` at 0x83e482..0x83e486; the intent is still inferred: that
     `WORLD_REPUTATION` was meant cannot be proved from the code];
   - list 5: **loan** if fatigue < −500 − random(300) and happiness < 0 [proved: 0x83e613..0x83e651].
9. The player slots' price (`shortlist_slot::VALUATION`, `shortlist_slot::PRICE`) [mkb_2, 0x83e66f..0x83ebfb]: p =
   min(target, 36000) (× 0.75 in a goalkeeper slot with no replacement). Without a replacement: valuation p and price p
   × 2.5 + reputation, 2p + rep., p × 1.8 + rep., p × 0.8 or p × 0.5 by status 1, 2, 3, 4/5, other (× 3 in a goalkeeper
   slot). With one: to the team (2, 3, 4; with `reset` happiness + 50, 10 matches left out) or the reserves (5), scored
   for the position (in the first 200 days × (1.25 − days × 0.00125)), within −10000..60000; score ≥ p, no grievances
   0x100 or 1 and more than 364 days of contract: he stays the target. Otherwise happiness moves by ftol((1 − score/p)
   × −100) (with `reset`; statuses 2 and 3 at least 25 and the slot loses him; the others at least 0; status 5 gets
   status 6), and the price is (rep. × f + p × k) − score × k.
10. Staff slots (jobs 6 to 10) with the person in the job (assistant +0xd3, `club::COACHES`, `club::PHYSIOS`,
    `club::SCOUTS`): the sacking only happens with the local flag [esp+0x60] set (zeroed at 0x83cb6c), and it is only
    set at 0x83cc44..0x83cc64: **the list was at ACTIVE 2, with no bids, and the club is not human** (0x600a50 with 1,
    1). Then a `score_staff_for_job` 0x69d1b0 score below min(target/5, 5000) and no contract offer (0x4cc9e0):
    **sacked** (`MANAGER_MANAGER::remove_staff_from_club` 0x690890; 0x83ee28..0x83ee72, the same for the other 4 jobs at
    0x83ef46, 0x83f056, 0x83f15f, 0x83f275). Otherwise, price (threshold − score) × 2.5 [proved].

#### 3.2 Choosing the target (0x83f3a0, original)

`fill_shortlist_slot(this, slot, club, excluded)`, thiscall, ret 0xc, recursive [proved: disassembly
0x83f3a0..0x840d80 (2048 lines), read whole for item `mkb_3`; the cited points checked; where there is no address, the
text is the `mkb_3` header]:

1. Leaves: no nation; human (0x600a50 with 0, 1); a human (1, 1) on holiday with rule 4 = 1; a slot with an offer; a
   national team.
2. **The knowledge divisor k** [proved: 0x83f4d8..0x83f5b8]: 100; reputation < 3500 → (6025 − reputation) × 4;
   Belgium, Holland, Sweden, Denmark, Norway → 50. Each of the 7 scout places (`club::SCOUTS`): empty +5 (+10 with the
   league active); with a scout, + 30 − 2 × `chance_to_know`(scout, 6) − `chance_to_know`(scout, 5). k = k/10, minimum
   2. **Probable defect:** 0x544410 gets the constants 6 and 5, not the scout's attributes; the result depends only on
   his current ability (+4 of the non-player sheet): `JUDGEMENT`/`JUDGING_POTENTIAL` never count [code proved (mkb_3,
   0x83f4d8..0x83f5b8); the intent is still inferred: it cannot be proved from the disassembly].
3. Reputation factor: reputation × 0.0008; in ITA, ENG, GER, FRA and ESP with finances, by the wealth band (0x5986b0):
   band 1, × 1/1175 (× 1/1125 above 6250); band 2, × 1/1125 (× 1/1050 above 4750). The slot's ceiling
   (`max_fee_for_squad_status` 0x84b230).
4. Walks the position lists (`POSITION_LISTS_BY_PERSON`) of the slot's position. A candidate passes: not excluded, id <
   the number of people − 16, job ≠ 5, no job 5 at a national team (`job_for_club`/`job_for_nation`), no manager
   vacancy (0x69fc60); a player with a sheet, no retirement announcement (0x83fb53..0x83fb5d), not Saudi
   (0x83fb63..0x83fb73); status 5 (`(+0x24 & 0x1e) = 0xa`) and age > 23 skips (0x83fb79..0x83fb8d); purchase: the
   ceiling (+7) below **price × 1.3** (0x968de8) skips (0x83fb97..0x83fbad); loan: passes with a club and a price ≤ 0,
   otherwise must be loan listed (0x83fbb8..0x83fbd7) [proved]; not of the club itself; on fewer than 10 shortlists;
   arrived more than 200 days ago (or status 7, grievances, listed); not on another of the club's slots; work permit
   (0x84b180, 0x8c7560); for a loan, can loan (0x8d0040); a loan from a human only if listed (bit 2, 0x83feb8). A human
   on holiday with rule 4 = 2 only considers his scout list. Non-player: the rest of a type 2 contract (days × wage /
   7) ≥ the ceiling or > (reputation + 500) × 10 skips.
5. **Whom the club knows** [proved: 0x83fed8..0x8400d4]: known when (0x83fed8) the human's scout list has him, or
   (0x83feea..0x83fefd) (person id + club id) % k = 0. Otherwise, **of another nation** (0x83ff03) without caps (+0x22,
   0x83ff11) and whose club is not of the club's nation (0x83ff1c..0x83ff26): a player under 23, foreign for the club
   (0x53d980 with 1) and with world reputation (+0xd) < 3250 is skipped (0x83ff2c..0x83ff4f); from another region
   (+0x75) he needs club reputation ≥ 2750 and (his club with no nation or of the same region, or club reputation ≥
   4250, or the nation's `league_standard` +0x85 ≥ 12) (0x83ff55..0x83ffa3); and then his reputation as the manager
   sees it (0x5407e0) ≥ 2750 + random(2000) in status 5/6 slots (with world reputation ≥ 1750) or ≥ 3250 +
   random(2000) in the others (world ≥ 2750) (0x83ffa9..0x840058). **Of the same nation** (or with caps, or at a club
   of the nation; 0x840060): a non-player, a loan slot, no sheet, home reputation (+9) ≥ 3250, his club ≥ 3250 or 20
   years or more: known; otherwise the youngsters filter (section D §3, 0x8400ae: fixed per person-club pair).
6. Score and price: below the valuation or the score to beat − 10000 skips; a purchase at a seller in crisis: at most
   the seller's value; a compensation deal (0x84b6f0): its fee; otherwise the asking price (0x4d42d0 human at 0x8402e6,
   0x4d1600 AI at 0x8402fe); status 4 under 28 or status 5: the excess over the value adds min(asking − value,
   (rep./13000 + 0.5) × value, 2,500,000) to the offer; **the prediction 0x84af00** = 2 skips; a price above the ceiling
   skips; score × y, y = 1 − (asking − value)/(max price × factor) (above 1 → y/3 + 0.67; 0.01..3); injury > 15 days:
   − (days − 15)²; loan: the owner's prediction 2 skips, injury > 10 days − (days − 10)² × 200; without
   `can_club_bid_for_player` skips; finally score × (92 + random(17))/100.
7. **Willingness** (0x846920; non-player: 0x844780 at 0x840682); interest news with random(79 − rep./150) = 0 and
   (rep. > 5000 + random(2000) or a human club); willingness ≥ 3.15 (0x968e58): the desired wage (0x864550 at
   0x840893, as f32) × max(3.5/willingness, 0.9) above 1.2 × the wage cap (0x5968d0 at 0x84091d) skips; otherwise he is
   the best so far. The scout story (0x587220) by draw.
8. The chosen becomes the target (`shortlist_slot::TARGET_STAFF_ID`) and enters the shortlist (`add_shortlist_entry`
   0x8423f0). With nobody, a status 2/3 slot becomes a **loan slot** (bit 0) and starts again if (reputation² × 4e-9
   + 0.6) × value < price, or ceiling < 50000, or Italy with a draw and reputation 3000..6000; only with the season
   flag (key nation: between the season start − 28 and its end − 28; other nations always; non-player slot never), and
   the nation's league loaded **or** random(reputation) > random(20000). Otherwise the target stays −1.

#### 3.3 Bids and requests (0x84a020, Rust)

The header in `rust/src/motor/shortlist_manager.rs` (lines 38-97) is complete [proved: Rust compared, cqe_3]. In
short: an AI club (not a national team, not human, not a human on holiday with rule 4 = 1), active shortlist, DAYS <
100, with finances; the count = slots with a bid + squad; player slots skipped with count ≥ 50. **Purchase**: the slot
value (`shortlist_slot::PRICE`, ×2 with the 0x20 mark, at least 20000), ×3/×2 in an active league by reputation > 4750
and count < 25/35; replacement unable, injured (days² × reputation/500), morale, under offer; desired wage ≤ 1.3 × cap;
**random(value) > random(2 × reputation + 1000)** makes the bid. **Loan**: not a French amateur club; loan possible; a
human seller only with the player listed; **random(PRICE) > random(reputation/10)**, or on the second pair >
random(2 × reputation): the request (0x842740, which needs willingness ≥ 3.5).

#### 3.4 The seller's prediction (0x84af00, Rust)

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

#### 3.5 The buyer reviewing its bid (0x8c0810, original; item mkb_1)

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

#### 3.6 The seller's answer (0x8c8090, Rust)

Complete header in `rust/src/motor/transfer_manager.rs` (lines 520-565) [proved: Rust compared]. In short: a loan bid
refused without looking at anything (practically dead code, see §2); "must accept"
(`transfer_manager_selling_club_must_accept`) accepts; a human on holiday with rule 0 refuses; floor = asking price
(0x4d1600) or the best other offer; cash, exchange and clause options; the first whose value × 0.85 is below the asking
price accepts; the best × 0.75 below the asking price becomes a **counter offer** (except types 1 and 2); otherwise
refuses (reason 10 if the total < 0.55 of the value with a 2 million difference, or < 0.1 with 100 thousand).

#### 3.7 The showcase (0x842c00, Rust)

[proved: Rust compared, cpp_6] Only a player with a sheet and `CURRENT_REPUTATION` ≥ 5500 (0x157c), unless forced; no
retirement announced; a player job; not a national team manager; not Saudi; no future transfer; one who arrived less
than 200 days ago only with status 7 or grievances. Walks the clubs by reputation (`[G_CLUB_REPUTATION_ORDER]`) and
offers him (0x842e10, which uses 0x84af00) until he is on 5 shortlists (the count is the low nibble of +0xb of the
79-byte record), skipping national teams, the contract's club and his own. **So a player under 5500 reputation is never
offered**, and high-reputation clubs get him first.

### 4. Fields read and written

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

The chairman and the directors (`club::CHAIRMAN` +0xbf, `club::DIRECTORS` +0xc3) are **not read directly** by any of
this section's 7 routines [proved: for the 4 in Rust and 0x8c0810; and no access to `[club + 0xbf]` or `[club + 0xc3]`
in 0x83cb40..0x83f3a0 or in 0x83f3a0..0x840d80]. **But they come in through the calls:** 0x83cb40 calls
`update_club_target_reputation` 0x83bf40 (0x83cda7), which reads the chairman's Patience (+0x1d) and Resources (+0x20)
(section D §3); 0x83f3a0 calls `get_max_wage` 0x5968d0 (0x84091d: the chairman's Business +0xf and Resources) and the
asking price 0x4d1600 (0x8402fe, 0x840bfb). Of the manager only `non_player::YOUNGSTERS` (0x83f3a0) and his judgement in the rating
(`match_man_judge_player_ability` 0x6e86d0 gets `club::MANAGER`) count; of the scouts, only their current ability
(§3.2).

### 5. Constants (read from the exe)

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

### 6. Defects of the original

In items 2 to 5 the code is proved; the intent is still **[inferred]** in all of them: whether it was meant cannot be
proved from the disassembly.

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

### 7. Extension points (a small hook changes the policy)

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

### 8. What was not ported

- **0x83cb40** (10,336 bytes to the boundary, 3,136 instructions, a 0x8a4-byte frame, 45 calls, 2 still original) and
  **0x83f3a0** (6,624 bytes, 2,048 instructions, recursive, SEH frame): too big to port and prove in this round; each
  needs an item of its own and a reader with time for the proof. The map above (§3.1, §3.2) is the starting point;
  0x83f3a0 has the fragments 0x840000 and 0x840010 (community map) and the piece 0x840084, which are not routines.
- **0x8c0810**: item `mkb_1` (compiles; comparison, careers and mutations did **not** run: the laboratory slots were
  taken; see `levas/fila/mkb_1/PRONTO.txt`).


<a id="c"></a>

## Market, section C: loans, windows and limits (reader C, 11/11/2026)

*English version of [secao_c.md](secao_c.md); the Portuguese file is the original.*

Part of the market map asked for by Tweaks 1.5 ([BRIEF.en.md](BRIEF.en.md)). **[proved]** = read in the disassembly or
in Rust already compared (0 divergences); **[inferred]** = a conclusion from the code, not reproduced in the game.

### 1. The routines

| Address | Name | In Rust | Item |
|---|---|---|---|
| 0x842740 | `Shortlist_Manager::try_loan_offer_for_player` | yes (`try_loan_offer_for_player`) | — |
| 0x8d0590 | `TRANSFER_MANAGER::get_loan_period` | yes (`transfer_manager_get_loan_period`) | — |
| 0x8d0040 | `TRANSFER_MANAGER::can_loan_player` | yes (`transfer_manager_can_loan_player`) | — |
| 0x8d0530 | `TRANSFER_MANAGER::can_end_loan` | yes (`can_end_loan`) | — |
| 0x8c63a0 | `TRANSFER_MANAGER::process_due_offer` | yes (`process_due_offer`) | — |
| 0x8c5880 | `TRANSFER_MANAGER::daily_update` (the weekly sweep of the offers) | yes | — |
| 0x8c2180 | `TRANSFER_MANAGER::handle_player_accepted_terms` (the loan completes here) | yes | — |
| 0x8cb810 | `TRANSFER_MANAGER::add_past_transfer` | yes (`add_past_transfer`) | — |
| 0x8cd450 | `TRANSFER_MANAGER::check_squad_limits` | yes | — |
| 0x8c6bc0 | `TRANSFER_MANAGER::update_transfer_window_news` | yes | — |
| 0x8cf470 | `TRANSFER_MANAGER::find_next_window_open_date` | yes | — |
| 0x8c4e10 | `TRANSFER_MANAGER::hold_offers_at_window_close` | yes | — |
| 0x4d01e0 / 0x4d07e0 / 0x4ccca0 | `CONTRACT_MANAGER::sign_loan_contract` / `end_loan` / `schedule_loan_end` | yes | — |
| 0x8e8bd0 | `TRANSFER_RULES::get_loan_period` (the default period) | no | **mkc_1** |
| 0x8e8e50 | `TRANSFER_RULES::find_open_window` | no | **mkc_1** |
| 0x8e8c40 | `transfer_window_contains_date` | no | **mkc_1** |
| 0x5c1ed0 | `FRANCE_RULES::can_end_loan` | no | **mkc_2** (8 leagues) |
| 0x6618f0 | `ITALY_RULES::can_end_loan` | no | **mkc_2** (8 leagues) |
| 0x412dd0 | `AUSTRALIA_RULES::can_loan_player` | no | **mkc_2** (8 leagues) |
| 0x579c70 / 0x579d60 / 0x579fd0 / 0x57a020 | England: period, `can_loan_player`, `record_loan`, `daily_update` | yes | — |
| 0x7f6530 / 0x7f6650 | Scotland: period, `can_loan_player` | yes | — |
| 0x661770 / 0x63c3d0 / 0x859630 | `can_loan_player` of Italy, Ireland, Spain | yes | — |
| 0x40a600 / 0x40a610 | base `can_loan_player` / `can_end_loan` (`mov eax, 1; ret`) | no | cannot be detoured (2 bytes) |
| 0x5a9a80 / 0x5a9af0 / 0x5a9ba0 | Finland: period, `can_loan_player`, `can_end_loan` | no | out: does not run in the 8 leagues |
| 0x794760 / 0x902240 | Norway, Turkey `can_loan_player` | no | out (same) |
| 0x793870 / 0x793900 / 0x7ec8e0 / 0x90f1a0 | Northern Ireland (period, `can_loan_player`), Russia and Wales (period) | no | out (same) |
| 0x5e0210 / 0x63c530 / 0x63c4e0 | Germany `can_end_loan`, Ireland `can_end_loan` and `record_loan` | no | out (same) |
| 0x8e8560 / 0x8e8800 | base `can_sign_player` / `can_make_offer` (foreign player limits) | no | mapped here, not ported (they belong to signings, reader B) |
| 0x784100 / 0x7874b0 / 0x5fd2f0 | `NewsItem::auto_respond`, `News::auto_respond_all`, `HUMAN_MANAGER::set_on_holiday` | no | mapped (the holiday defect) |

### 2. The AI loan request (0x842740) [proved, Rust]

Called by `make_club_bids` (0x84a020, the AI's pass over its 40 shortlist slots), with `can_loan_player` checked
before by `fill_shortlist_slot` (0x83f3a0) and by `make_club_bids` itself (0x84aa2b). Order:

1. No owner (main contract 0x4cf5a0 null): the shortlist slot goes back to state 1; −1.
2. The player's interest in the club (`staff_interest_in_club`, 0x846920, with the slot's state) below **3.5**
   (`.rdata` 0x968e38): −1.
3. The period (`get_loan_period`, §3): months to the end `(end_year − year)·12 − month + end_month`, to the start the
   same, at least 1; start after the end: −1. **This is where the end-of-contract loan dies for the AI** (§7).
4. A club of Norway (`KEY_NATION_NORWAY`) with today's month in November to March: −1.
5. Interest without wage ≥ **2.75** (0x9690a8) and the club's nation not Spain, Portugal or Italy: counts the months to
   the start; otherwise to the end.
6. The return date: today's day (at most 28) in month today + months. `can_end_loan` (0x8d0530, flag 1) = 0: −1.
   Return after the end of the main contract (+0x2d/+0x2f): −1. Return − 4 weeks before the window date of the club's
   key nation (`[0xb63d8a + 72·key_nation_index_of_club]`): −1. Return before today + 3 weeks: −1.
7. Wage paid: 100% if the club's reputation (`club.reputation()`) beats the owner's by more than 2500, 0% if it is 1500
   or more below, otherwise 50%. The offer: `make_loan_offer` (0x8bed60).

The owner's answer is decided by 0x84af00 (`predict_selling_club_response`, another reader's section).

### 3. The loan period (0x8d0590) [proved, Rust]

`TRANSFER_MANAGER::get_loan_period(club, contract_club, player, &start_month, &start_year, &end_month, &end_year)`,
thiscall ret 0x1c. Callers: 0x842740 (AI), 0x8d0040 (`can_loan_player`), 0x8f9690 (the human's loan screen), 0x8f1c30
(answering a loan bid), 0x888260/0x889c30 (profile and comparison). 203,886 calls up to 2002-06-01 in the English
career (counters of item cqe_6).

1. The rule of the nation of the **borrowing club** (of the 28 at `TRANSFER_RULES` +0x8ac; without one, the first)
   gives the period through virtual +0x28 (`GET_LOAN_PERIOD`):
   - base 0x8e8bd0 (**mkc_1**): from next month to the season end month of the club's nation (0x4d45f0), this year if
     still ahead of the start, otherwise next year;
   - England 0x579c70: from next month to 3 months later; after 19 June and a player under 24, the base;
   - Finland 0x5a9a80: as the base; Russia 0x7ec8e0: start in 3 months, end at season end; Northern Ireland 0x793870:
     the same, and the start never passes the end; Wales 0x90f1a0: start = end (season end).
2. The player's main contract (0x4cf5a0). **No contract, a type (+0x35 & 0x3f) other than 2, or today after the
   contract end: the end year becomes 1972** (the year of the default date 0x54a110), an "impossible" sentinel.
   The types (screen 0x87cee0, jump table 0x87dd58) [proved]: 0 invalid, 1 monthly, **2 normal contract**, 5 trial,
   6/7 loan; bit 0x80 = part time.
3. End more than 5 years ahead: 1972.
4. While the date (today's day of the month, end month, end year) passes the contract end (+0x2d day of the year,
   +0x2f year), the end steps back one month and the start follows if it is after the end; the end year below the
   current one: 1972.

### 4. Can the club take him on loan? (0x8d0040) [proved, Rust]

`can_loan_player(club, player, message)`, thiscall ret 0xc, 1 = yes. Callers: 0x83f3a0 and 0x84a020 (AI, no message),
0x8f9690 (the human's screen, message at 0xde1f64).

1. No current club (contract other than type 5): 0. Already on loan (loan contract 0x4cf6d0): 0. Not a player
   (`job_for_club() != PLAYER`): 0.
2. `can_club_bid_for_player` (reasons 0x21 under negotiation, 0x27, 7, 1, 0x12/0x13, 0x19/0x28/0x1a nation): 0 with
   the text.
3. The rule of the **borrowing club's** nation, virtual +0x20 (`CAN_LOAN_PLAYER`) = 0: 0. By nation:
   - base 0x40a600 / Greece 0x5ed060: 1; Northern Ireland 0x793900: 1 (writes `[0xae1080]`);
   - England 0x579d60: same division (Premier only) 0x1d; **5 a season** (byte 0 of `[[0xde1f54]+0x91c] + id·16`,
     zeroed on 20 June by 0x57a020, counted by 0x579fd0) 0x1e; **more than 1** loanee from elsewhere already in the
     squad (the 3rd is refused) 0x26; outside the window (0x8e8e50) 9;
   - Australia 0x412dd0 (**mkc_2**), Norway 0x794760, Turkey 0x902240: outside the window, 9 ("The deadline for loans
     to ... clubs has passed."); Finland 0x5a9af0: the same and, for an AI club, only between days 90 and 300 of the
     year;
   - Italy 0x661770, Spain 0x859630, Scotland 0x7f6650, Ireland 0x63c3d0: in Rust (windows and division limits).
4. The rule of the **current club's** nation (the lender): none, 1.
5. The period (§3). **Start = end: 1** (nothing else). Otherwise, month by month from the start to the end
   (inclusive), the owner rule's virtual +0x24 (`CAN_END_LOAN`, flag 1); the first month that lets the loan end gives
   1; past the end: 0, "This loan deal cannot go ahead". By nation (+0x24):
   - base 0x40a610: 1 (England, Scotland, Spain...: the loop answers 1 on the first month);
   - France 0x5c1ed0 (**mkc_2**) and Ireland 0x63c530: the date inside a window of the nation;
   - Italy 0x6618f0 (**mkc_2**): with the flag, June; otherwise the date in the 1st or 2nd Italian window;
   - Finland 0x5a9ba0: refuses between 18 August and 15 November;
   - Germany 0x5e0210: without the flag, refuses ending loans between German clubs (with the flag, 1).

### 5. Windows [proved]

Transfer rules: +4 the window table (12 bytes: +0 opening, +6 closing; each date 6 bytes: +0 rule, +1 number, +2 the
weekday it moves to (0xff none) [proved: `transfer_window_get_date` 0x40a7d0 passes byte +2 sign-extended (0x40a7d1
`movsx ax, byte [ecx+2]`) as the 4th argument of `Date::Date` 0x549f70, which with a value ≠ 0xffff calls
`Date::get_nearest_weekday` 0x54a2d0 (0x54a0ea..0x54a0fc): the date moves to the nearest such weekday], +3 day, +4 month (0 = January), +5 1 opens / 0 closes), +8 how many, +0xa
the nation, +0x13 foreign player limit, +0x14 non-EU limit.

- `transfer_window_contains_date` 0x8e8c40 (**mkc_1**): the two dates in the date's year, both ends included; a window
  that wraps the year (opening month > closing month) is open from the opening to December and from January to the
  closing.
- `find_open_window` 0x8e8e50 (**mkc_1**): the first window with the date; **a nation without any window (Denmark) is
  always open**; with windows and none open, 0.
- Dates (constructors, Rust): England 2 June (Monday) to 26 March (Thursday); USA 7 January to 15 August; Australia 11
  June to 17 April; France 30/5–31/8, 1/9–20/12, 21/12–10/1; Finland 16/11–15/8; Croatia 1/6–30/3; Denmark none.
- Notices (0x8c6bc0, every day, slice 0): on the day, the opening/closing; the day after the opening, the offers that
  waited for that nation's window (0x8c4e10); a week before.

### 6. Limits that block a signing [proved]

- **Squad** (`check_squad_limits` 0x8cd450, Rust, from the offer restrictions 0x8cfc90 for the buyer and the seller):
  50 players (club + its B team), 5 coaches, 3 physios, 7 scouts and the assistant; above, an AI buyer without an
  exchange releases for free the cheapest player who can leave; otherwise reason 4 or 0x22 (+1 for the seller). For a
  loan, the completion (0x8c2180) refuses if the borrower's squad + 1 passes 50 (reason 4) and asks the borrower
  nation's `CAN_LOAN_PLAYER` again.
- **Foreigners** (base `can_sign_player` 0x8e8560, virtual +0x10, not ported): counts the foreigners in the squad (50
  slots +0xd7, `database_is_staff_foreign` 0x53d980) up to limit +0x13 (default 50; 0xff no limit): reached, reason
  0x14. A Turkish club outside the Turkish Premier: limit 0 (no foreigner). With the rules' nation in Europe
  (`KEY_CONTINENT_EUROPE`), the non-EU players (0x53d740) up to +0x14 (default 30): reason 0x15; Spain: 4 in the First
  Division, 3 in the Second, 0 elsewhere. Then the window: closed, 9. `can_make_offer` 0x8e8800 (+0xc) does the same
  count with the exchange players and only checks the window when the decider is the buyer. Overridden by: Japan (5
  foreigners), Portugal, Spain, Italy, Germany, Belgium, France, Argentina.

### 7. Suspected defect 1: the loan at the end of the contract

**The path [proved]:** §3 steps 2–4 and §4 step 5, with §2 step 3.

- A contract **not of type 2** (monthly 1, trial 5) or already expired: the period comes out with year 1972.
  - The AI (0x842740) computes negative months to the end: never asks.
  - `can_loan_player` **does not recognise the sentinel**: start ≠ end, so it tests the start month with the owner's
    `CAN_END_LOAN`; with an English owner (base = 1) **it returns 1**. The human's screen (0x8f9690) opens with fields
    5/6 (end) = the month and **1972** [proved: the values go to fields 3–6 and 0xc/0xd of the screen]. With a French/
    Italian/Irish/Finnish owner outside the window, "This loan deal cannot go ahead".
- A type 2 contract that **ends in the current month**: the end steps back to this month and the start follows (start
  = end):
  - `can_loan_player` returns 1 (start = end);
  - the AI computes 0 months to the end < 1: never asks;
  - the human gets a zero-month period (return this month) [what the screen does with it: inferred; screen only, no
    change to the automatic career: the builder 0x8f9690 puts the period in controls 3-6 and 0xc/0xd
    (0x8f980f..0x8f98ac), the handler is 0x8fac30 (registered at 0x8f9747) and the confirmation goes through 0x8fb5e0;
    not followed to where the 1972 end or the zero period becomes (or not) a refusal].
- A contract ending **next month**: a one-month loan; the AI asks if the return is 3 weeks away or more.

**A defect of the original?** Yes, of consistency [proved]: `get_loan_period` uses 1972 to say "impossible", but
`can_loan_player` only blocks through the owner's end rule and accepts start = end, so the human is allowed to open a
proposal with an impossible (or zero-month) end, while the AI silently refuses. That this is exactly the forum's "I
can't loan players at the end of their contract" (58.2) is **inferred**: only reproducing it in the game proves it.

**Small fix:** a detour on 0x8d0040 (Rust here already; in the Tweaks, prologue `6a ff 68 1b 40 96 00`, 7 bytes) that,
after the period, returns 0 with a clear message when the end year is the default date's (1972) or the end is before
the start; or, as a more generous policy, a detour on 0x8d0590 (prologue `83 ec 10 32 d2`) that calls the original and,
when the sentinel came from the contract (type ≠ 2 or stepping back below this year), sets the end to the contract end
month and the start to the month after today, if it fits.

### 8. Suspected defect 2: offers stuck while the manager is on holiday

**What the code says [proved]:**

- On holiday, `human_manager_is_human_club(club, 0, 1, 0, 0)` (0x600a50, checking holidays) says **no**: new offers
  take the AI branches of `process_due_offer` (0 → AI reply; 1 → `submit_loan_contract_offer`; 5/6 → contract to the
  player). State 0x11/0x12 uses `news_club_has_human_manager` (0x7875e0, **ignores** holidays): sends news 0xfa0 and
  waits 28 days.
- News delivered while the manager is away is answered at once (`News::deliver_to_manager` 0x785d40 →
  `NewsItem::auto_respond` 0x784100). On **starting** the holiday, `set_on_holiday` (0x5fd2f0, at 0x5fd3d9/0x5fd3ed)
  answers everything in the inbox (`auto_respond_all` 0x7874b0): only the items still in the 100-slot ring.
- `auto_respond` covers: 0xfa0 (offer; loan → 0x84af00 → accept 0x8c0660 / refuse 0x8bfa30), 0xfa2, 0xfa4 (only sets
  `RESPONSE_ROUND` = 3), 0xfa5 (confirmation → 0x8cfc90 → `club_rejects_offer`/`confirm_offer_by_club`), 0xfa8, 0xfab,
  0xfac (contract talks → `offer_contract_to_offer_player`), 0xfb4, 0xfb5 (loan request), 0xfb6, 0xfc0 (loan
  negotiation → `submit_loan_contract_offer`).
- `process_due_offer` **has no branch** for 8, 9, 0xb, 0xc, 0xd and 0x13 (0x13 belongs to the future transfers; 8 to
  the contract manager; 0xa to the permits). The weekly sweep of `daily_update` (day a multiple of 7) reschedules for
  tomorrow every overdue offer **except** 0x13, 7, 8 and 0xa: 9, 0xb, 0xc and 0xd come back every day and nothing
  happens. Proof: `process_due_offer` does not compare 8, 9, 0xb, 0xc, 0xd (the status comparisons are those of
  0x8c6708..0x8c6972 and there is no jump table); the sweep skips 0x13, 7, 8 and 0xa at 0x8c5964..0x8c5973. Writers
  (scan of `mov byte [reg+0x2c], imm`): 9 only at 0x8c6a8c (`process_due_offer`, human buyer); 0xb at 0x8c31e7
  (0x8c2f50); 0xc/0xd in 0x8c3310 (0x8c336e/0x8c335b). Register writes not scanned.

**The defect [mechanism proved; effect in the game inferred, not reproduced]:** an offer that was waiting for the human (9: the human buyer must offer the
contract; 0xb–0xd: the clubs' confirmations) when he goes on holiday only moves if its news item is still in the
inbox; if it left the 100-slot ring (or he deleted it), it stays put for the whole holiday, and the player stays
"under offer" (bit 0 of the 8-byte record in `PERSON_FLAGS`), which blocks other offers for him. It is a defect of the
original: the dispatch depends on the news item, not on the offer's state.

**And loans?** A loan never goes through 9 or 0xb–0xd: it completes the moment the player accepts (0x8c2180). The loan
states that wait for someone are 0 (human owner: state 4 in 14 days, it lapses), 1 (human borrower: state 4 in 10
days, it lapses), 0xf (removed on its return with round 3) and 8 (the player's decision, in the contract manager
0x4cc930/0x4cd9c0). The only one that **does not lapse** is 8, and the weekly sweep leaves it out.

**Defect of the original: the loan stuck in state 8 [proved, code path; not reproduced in the game].** A loan can stay
in 8 forever, **without holidays**:

1. The (AI) owner accepts: `contract_manager_schedule_decision_for_transfer` 0x4cc930, state 8 and **no
   rescheduling** (0x8c6577..0x8c6592 in `process_due_offer`; 0x8c0680/0x8c068b in `accept_loan_offer`). 0x4cc930 marks
   the (person, club) contract offer that has +0x62 = 0 and schedules the decision (0x4cc975 is the only write of 1 to
   +0x62 outside `add_contract_offer`).
2. The loan's contract offer is type 6 (`init_from_current_contract` 0x85e590: `& 0xc6 | 6`), and `add_contract_offer`
   0x4cc490 writes +0x62 = (type ≠ 6): 0 for the loan (0x4cc597..0x4cc5b7 for a new offer; 0x4cc741..0x4cc761 for an
   updated one).
3. The player's decision (0x4ce4e0) only looks at offers with +0x62 ≠ 0 (0x4ce63c..0x4ce641). If he picks **another**
   offer tied to a transfer, the loan's becomes state 1 and `handle_player_rejected_terms` 0x8c28c0
   (0x4ced1b..0x4ced32: the transfer goes to 0xf, round 3, back in 10 days); and the loan's contract offer is **held**
   in the list at +0x38 (0x4cf0b5..0x4cf13d, a club other than the chosen one's and the chosen one tied to a slot ≠ −1)
   and leaves the live list (0x4cf35d..0x4cf383).
4. If the chosen transfer is **withdrawn** at state ≥ 0xb (`withdraw_transfer_offer` 0x8bfcb0, 0x8bfd3a; the only
   caller of 0x4cf4a0, at 0x8bfd4a) while the loan still exists (the 10 days of step 3), `resubmit_held_offers`
   0x4cf4a0 puts the transfer from 0xf into **8** (0x4cf4fb..0x4cf501) and resubmits the offer with
   `add_contract_offer(…, decides 1, agreed 0)` (0x4cf505..0x4cf52a): type 6, so +0x62 = 0.
5. The scheduled decision skips that offer (step 3); 0x4cc930 is not called again (only on the state 0 acceptances);
   `process_due_offer` has no branch for 8; the sweep skips 8. **The loan stays in 8 for good** and the player stays
   "under offer" (bit 0 of the 8-byte record is only cleared with the offers resolved, and 8 does not count).

The decision is also put off 2 days at a time while the player has another offer in 0, 1, 5 or 6
(`player_has_pending_offer` 0x8cd150 at 0x4ce5ce), but that ends when the other one lapses. **The holidays play no
part** in this path (neither 0x4ce4e0 nor 0x4cf4a0 looks at the human). That it is xeno's item 88 is still
**[inferred]** (not reproduced in the game).

**Small fix (offer stuck on holiday):** in `process_due_offer` (Rust here; in the Tweaks, prologue
`6a ff 68 00 3d 96 00`), for state 9/0xb/0xc/0xd with the human involved on holiday (`news_club_has_human_manager` ≠ 0
and `human_manager_is_human_club(…, 0, 1, 0, 0)` = 0), do what `auto_respond` would: 9 →
`offer_contract_to_offer_player`; 0xb–0xd → `check_offer_restrictions` and `confirm_offer_by_club` or
`club_rejects_offer`.

**Small fix (stuck loan):** in `resubmit_held_offers` 0x4cf4a0, for a loan transfer (`offer_type` 1), call 0x4cc930
after the resubmission (or write +0x62 = 1); or, in `process_due_offer` (Rust), treat an overdue loan in 8 as a refusal
(`reject_loan_offer`).

### 9. How a loan ends and returns [proved, Rust]

- **Signing:** `handle_player_accepted_terms` (0x8c2180) → `sign_loan_contract` (0x4d01e0): the contract goes into the
  loan slot (+4 of the 79-byte record), the player joins the borrower's squad (job 0xb), bit 0x40 of the main contract
  = reserves; history `add_loan_records`; `record_loan` of the borrower's nation (England counts the season);
  `add_past_transfer` type 2; offer 0x14, removed in 5 days.
- **Natural end:** `CONTRACT_MANAGER::daily_update` (0x4c8114) calls `end_loan` (0x4d07e0) on the end date [inferred
  from the call site].
- **Early end:** the screen (0x8fbcc0) asks `can_end_loan` (0x8d0530: only when the player's owning club is in the same
  nation as the one asking; otherwise 1) → `schedule_loan_end` (0x4ccca0: the loan contract ends tomorrow, type 7,
  news 0xfb8 if a human is involved) → `end_loan` the next day.
- **Permanent transfer:** `move_player_to_club` (0x8c9d5d) ends the loan first.
- **`end_loan`:** takes him out of the borrower's squad and tactics, frees the loan contract, redoes the wage bills,
  returns him to the owner with the squad mark (with a reserve team: 2 if the main contract has bit 0x40, otherwise
  1), European registration 0x10 → 4.

### 10. The transfer history (0x8cb810) [proved]

A 37-byte record (0x25): +0 person, +4/+8/+0xc name ids, +0x10 destination club, +0x14 origin club, +0x18 fee, **+0x1c
the type**, +0x1d the date. The types, by their writers: **0 paid**, **1 with exchange players** (`move_player_to_club`
0x8c99a5: `exchange_players` ≠ 0), **2 loan** (0x8c2180), **3 release** (`free_transfer_staff` 0x8c1af0), **4 free**
(pre-contract with fee 0 and completion 3 or 4, 0x8c9996), **5 a non-player** (manager/staff: `move_player_to_club`
0x8c99bf and `MANAGER_MANAGER::daily_update` 0x690432; or record only in `free_transfer_staff`). Vector with capacity
+0x866, count `past_transfer_count()`, grows by 100.

### 11. Extension points (for the Tweaks)

| Where | What it changes | How |
|---|---|---|
| 0x8d0590 `get_loan_period` | default length; the end-of-contract sentinel | detour, call the original and adjust the 4 outputs |
| 0x8d0040 `can_loan_player` | who can borrow | detour: original + the new rule (e.g. refuse the sentinel; allow outside the window) |
| rule vtables (+0x20, +0x24, +0x28) | one nation's rule | swap the slot pointer in the nation's table (England 0x969f4c, Scotland 0x96f838, Italy 0x96c96c, Ireland 0x96c190, Spain 0x96fdec) |
| rules +4/+8 (memory) | the window dates | data, no code: rewrite the table after the constructors |
| rules +0x13/+0x14 (memory) | foreign/non-EU limits | data |
| `[[0xde1f54]+0x91c] + id·16` | the English season loan count | data (the limits 5 and 2 are immediates in 0x579d60) |
| 0x842740 (3.5 / 2.75 / 2500 / −1500 / 3 weeks) | how much the AI borrows | detour; do not touch the `.rdata` doubles (shared) |
| 0x8cb810 `add_past_transfer` | measuring | every completed move passes here with its type |
| 0x8c63a0 `process_due_offer` | offers stuck on holiday | detour (§8) |

### 12. Doubts

- The exact point of the human's screen that turns the 1972 end (or the zero-month period) into a refusal [still
  inferred, see §7].
- ~~Which state traps the loan on holiday~~: answered in §8 (state 8, without holidays, through the withdrawal of a
  held transfer).
- Do contract types 3 and 4 (N/A on the screen) exist in the data? [still inferred: needs a scan of the contracts of a
  save or of the database, outside the disassembly]
- The loop of `ENGLAND_RULES::can_loan_player` tests the id of the target `player`, not of the squad player, to skip
  invalid slots (the Rust reproduces it; no effect in practice) [proved: at 0x579ea8..0x579ec2 the test reads
  `[esp+0x84]` (the 2nd argument, the target player, also read at 0x579d65) and not `esi` (the slot's player); with a
  target of valid id the test always passes].


<a id="d"></a>

## Market, section D: the player's side and the people who decide (reader D, 2026-11-11)

Scope: the player's will to go or stay (loans included), the wage demand and the contract length, renewals, clauses,
the free-agent market and, across the whole market, where the chairman's and the staff's (manager, assistant, scouts,
directors) attributes are read. Marks: **[proved]** = disassembly, a scan of the exe or Rust code already compared
with 0 divergences; **[inferred]** = Ghidra pseudo-C or a structural reading, not checked instruction by instruction.

Sources: `rust/src/motor/{shortlist_manager,contract_manager,staff_contracts,transfer_manager,match_man,fog_of_war}.rs`
(the routines already in Rust, read from the code and the header), `pacotes/0x4ce4e0`, `pacotes/0x866830`, the pseudo-C
of 0x85fc50, 0x4c7ac0, 0x864550, 0x862720 and 0x844780 (`descompilar.py`), `refs.py` (callers) and a scan of the whole
`.text` (capstone) for byte reads of `[sheet + 0x0e..0x22]` after `mov r, [x + 0x69]` (`staff::NON_PLAYER`).

### 1. The routines

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

### 2. The rules

#### 2.1 The will to go or stay (0x846920, Rust) [proved]

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

#### 2.2 The player picks among contract offers (0x4ce4e0, original) [inferred]

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

#### 2.3 Wage demand and length [proved where in Rust; inferred elsewhere]

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

#### 2.4 Renewals and contract end (0x4c7ac0, original) [inferred from the call list]

Daily. For each contract: the end band (`get_expiry_band`, 0..14, ×11) against the dates +7..+182 days; retirement
(0x7a7eb0, ×10); human club (×12); the AI renewal offer (`offer_contract`, ×9); `wants_to_leave` (0x866f60); pending
transfer offers (0x8c5cf0, 0x8c5e40); Bosman (0x8d0a70, ×5; a nation with free transfers, 0x544670); FIFA protected
period (0x868210); active clause (0x867b50); contract end news (0x4d6790 ×12, 0x4da590 ×5, 0x4d6d70); the free
player goes onto the clubs' lists (`add_player_to_club_shortlists` 0x842c00, ×4) and into the free transfer queue
(0x8c16b0) or is released at once (`free_transfer_staff` 0x8c1af0); the loan end (0x4d07e0); and, at the end, the
decision dates (0x4cd9c0). Attributes: only `loyalty` (+0x59, ×2).

#### 2.5 Clauses [proved where in Rust]

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

#### 2.6 Free agents [proved where in Rust]

Who becomes free: the contract end in 0x4c7ac0 (Bosman by the nation, 0x8d0a70) and those released through the queue
(0x8c16b0 → `process_free_transfer_queue`, on the transfers' day: at a human club the board may block it, a draw adds
the manager to the disliked staff, compensation). Who signs them: the clubs' lists (`add_player_to_club_shortlists`,
`make_club_bids` 0x84a020; without a club the will gets × 1.75 and the base 2.75 + min(12, days² / 900)). At what
wage: the desired contract (0x85fc50) against the cap (`get_max_wage`).

### 3. Chairman's and staff's attributes in the market (the requester's question)

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
| Youngsters (+0x22), manager | 0x8400ae | `fill_shortlist_slot` 0x83f3a0 (rest at 0x840084) | no | a non-loan slot, under 20 with home reputation (+9) < 3250 and current club < 3250: goes on only if edi mod (25 − youngsters) = 0 (no non-player sheet on the manager, mod 15) [proved: 0x8400ae..0x8400d4]; `edi` = person id + club id (0x83fee7..0x83feee, the same sum as the % k test; no write to `edi` up to 0x8400b9 on the three paths 0x83ff0b, 0x83ff16, 0x83ff26) [proved]: **fixed per (person, club) pair**, the same youngster is always accepted or always ignored by the same club while the manager is the same; youngsters 20 lets through the pairs whose sum is a multiple of 5. Only someone who already failed the % k test gets here |
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

### 4. Constants (read from the exe)

0x846920: 3.5 (0x968e38), 2.75 (0x9690a8), 1.75 (0x968f98), 3.15 (0x968e58). 0x4ce4e0: 1/300 (0x968e60), 3.15
(0x968e58), 0.99 (0x968e50), 0.6 (0x968e48), 0.9 (0x9689d8), 0.8 (0x9689b0), 0.7 (0x968e40), 0.5 (0x968968).
0x866830: 0.8 / 0.3 / 0.2 / 0.4 / 0.7 / 0.5 (f64), 0.2222222 (f32 0x96ff84), f32 multipliers 1.01 (0x96ff7c), 1.05
(0x96d3b8), 1.1 (0x96a33c), 1.15 (0x96d390), 1.2 (0x96d7bc), 1.25 (0x96b010), 1.45 (0x96ff74), 1.5 (0x96b004), 1.7
(0x96ff78), 1.75 (0x96b00c), 1.8 (0x96d7b4), 1.95 (0x96ff80), 1.25 f64 (0x968928). 0x5968d0 (chairman): 1.05
(0x9689b8), 1.1 (0x9689c0), 0.95 (0x9689d0), 1.125 (0x96a5a8). Almost all are shared by dozens of routines: changing
`.rdata` changes the whole game.

### 5. Defects of the original

1. `calc_contract_value` (0x866830): without an offer it reads the offer's clauses through a null pointer
   (0x866df6); no caller passes null [proved]. Without a player sheet the age stands for "band − age" (a = age, 5
   after the limit) [proved]. The non-promotion clause (case 0) looks at the offer's minimum fee clause and uses the
   7.5 million band, the others 5 million [proved (mkd_1); the intent is still inferred: it cannot be proved from the
   code].
2. `staff_interest_in_joining_club`: at his own club it divides by a stack float nobody wrote (no effect, the path
   does not use the result; already noted in the Rust).
3. `get_max_wage`: a chairman with **little** business sense (< 6) pays **more** (× 1.05/× 1.1) and a good one (> 15)
   pays less (× 0.95) [proved]. **It is not inverted** by the code: in both money routines a chairman with high
   Business spends less: in the budget he keeps more weeks of wages in reserve (Business + 6, 0x598b24..0x598b2a) and
   in the wage cap he pays × 0.95 (0x597a0d). A prudent chairman, consistent in both [proved]. Whether that was meant is
   still unproved [inferred].
4. Rust documentation: the header of `young_player_adjustment` (shortlist_manager.rs) says "current ability seen", but
   it calls `perceived_ability` with `kind` 1, which is the potential (reads JudgingPotential +0x18) [proved].

### 6. Extension points (the smallest hook that changes the policy)

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

### 7. Queue items

- **mkd_1** — `STAFF_CONTRACT::calc_contract_value` (0x866830): `levas/fila/mkd_1/`. Compiles with the current engine
  (own copy, no warnings from the item). **Not proved in the game**: the laboratory lock was busy and the memory guard
  refused (2.3 GB free, minimum 4). See `PRONTO.txt`.

Not ported because of size (for another round): 0x85fc50 (10,896 bytes), 0x864550 (8,400), 0x862720 (7,728), 0x844780
(8,608), 0x4c7ac0 (9,376), 0x4ce4e0 (4,032). Screens only (no effect on the automatic career): 0x4d4660, 0x4d4830,
0x4d44e0, 0x867ad0, 0x85f7a0.
