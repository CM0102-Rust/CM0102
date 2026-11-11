*Portuguese version: [MAPA_MERCADO.md](MAPA_MERCADO.md).*

# Transfer market map (loans included)

Made on 11/11/2026 for the Rust Tweaks release 1.5 (rebalancing the market), by four readers of the original's disassembly and of the Rust code. Every claim is marked **[proved]** (disassembly or a test) or **[inferred]**. Routines not yet in Rust were ported as queue items (`levas/fila/mka_*`, `mkb_*`, `mkc_*`, `mkd_*`); their proof status is in each item's `PRONTO.txt`. Sources: `levas/reorganizacao/mercado/`.

**Updated 11/11/2026: inferred claims closed** (`levas/reorganizacao/mercado/fechamento_inferidos.en.md`): the [inferred] marks outside the routines still being read (0x4d1600, 0x85fc50, 0x864550, 0x4ce4e0, 0x4c7ac0, 0x862720, 0x844780) were proved in the disassembly or kept with the reason. Main points: the loan stuck in state 8 is a proved defect of the original, without holidays (section C §8); bit 4 of `contract::TRANSFER_STATUS` = automatic asking price, bit 8 = listed at the player's request; the squad statuses (high nibble of +0x4f); the `edi` of the youngsters filter is the person's id + the club's id; the future-transfer +0x10 = 1 when the person leaves; the rules of 0x83cb40 and 0x83f3a0 from items `mkb_2`/`mkb_3`; `get_max_wage` is not inverted.

**Updated 11/11/2026: reader D2 closed** (`levas/reorganizacao/mercado/fechamento_mkd2.en.md`): 0x4ce4e0 (the player picks an offer; now item `mkd_4`) and 0x4c7ac0 (contract end, AI renewals, free agents; not ported) proved in the disassembly (section D §2.2, §2.4, §2.6, §5, §7); the natural end of a loan too (section C §9).

**Updated 11/11/2026: reader D3 closed** (`levas/reorganizacao/mercado/fechamento_mkd3.en.md`): 0x862720 (the default wage of the initial shortlists and the regens; item `mkd_6`) and 0x844780 (the staff's will; item `mkd_7`) proved and ported (section D §1, §2.1b, §2.3, §5, §6, §7); no market routine is left only mapped.

**Updated 11/11/2026: the nations' loan rules closed** (`levas/reorganizacao/mercado/fechamento_mkc.en.md`): the 29 rules virtual tables checked against the code (section C §1); the shared routines (three run in the 8 leagues: 0x902240 through the USA, 0x90f1a0 through France and Australia, 0x7ec8e0 through Brazil); the periods, `can_loan_player` and `can_end_loan` of Finland, Germany, Ireland, Northern Ireland and Norway proved (§3, §4); the §7 correction; items `mkc_3` to `mkc_6`.

**Updated 11/11/2026: section A §3 closed** (`levas/reorganizacao/mercado/fechamento_mka.en.md`): 0x4d1600 (the asking price) disassembled whole and proved, now item `mka_2` (section A §3, 18 steps; corrections: morale was inverted, step 7 is the recent signing, the reasons 0x800/0x1000/0x2000, the return to the value without a shortlist entry, the goalkeeper's cap at 32); the manager's Youngsters at 0x4d3ccb proved in the youngster floor (section D §3).

**Updated 11/11/2026: reader D1 closed** (`levas/reorganizacao/mercado/fechamento_mkd1.en.md`): 0x864550 (the default wage) proved and ported as item `mkd_2`; 0x85fc50 (the desired contract) proved and later ported as item `mkd_3`, with five corrections in §2.3 (the cheap renewal requires not leaving on a Bosman; the 6th argument is the reasons; the q caps by grievance; the clause fees; finances +0x10 [inferred]); the "250,000 cap" and 150000 are "not available" markers (section D §2.3, §5, §6, §7).

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
| 0x4d1600 | `contract_manager_calc_asking_price` | no → **item `mka_2`** (disassembled whole, see §3) | `calc_asking_price` |
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
bit 4 returns the stored value at once **[proved: 0x4d1697..0x4d16e7]**: in the test career (manager on holiday) the human
club's asking price is the stored value.

**Extension point:** the return; or the floor (`staff::VALUE` +0x52, which the game itself updates).

### 3. The asking price: `contract_manager_calc_asking_price` (0x4d1600) — item **`mka_2`**

`thiscall(contract manager, seller, buyer, player, *reasons, adjust, flag)`, `ret 0x18`, 0x4d1600..0x4d4257
**[proved]**: disassembled whole, instruction by instruction (`desmonta.py jogo/cm0102.exe 0x4d1600 0x4d4260`, 3135
lines), with the x87 stack operands followed by hand and the magic-constant divisions checked by brute force (/50, /500,
/1000, /20, /670, /−1000). Item `levas/fila/mka_2` (compiled; the proof on the benches is left to the main session).
`reasons` is a `ushort*` (set with `or byte [edi+1], k`, cleared with `and word [edi], 0x800`); `adjust` is read as a
byte [0x4d19c6]; `flag` as a dword [0x4d3891]. 20 callers (`refs.py`): 0x4d4429, 0x4d739c, 0x4d8705, 0x696ec5,
0x7847d4, 0x7b6df1, 0x8402fe, 0x840bfb, 0x8433a8, 0x843574, 0x843b17, 0x84b0e0, 0x8c0adc, 0x8c1542, 0x8c8291,
0x8c8c6e, 0x8cd391, 0x8e6533, 0x8e6554, 0x8f4ba4. The routine draws nothing; it only writes `*reasons` (and
`G_CONTRACT_LOOKUP_RESULT` inside `calc_player_value`).

The rule, in order **[proved]** (addresses in brackets):

1. **Early exits** [0x4d160d..0x4d16e7]: `get_contract_for_club(this, player, seller)` [0x4d1663]; no seller, no
   contract, `+0x35 & 0x3f` ≠ 2 [0x4d1676] or `+0x49` ≠ 0 [0x4d1683]: 0. A human club (1, 1) [0x4d1697] on holiday
   (`HUMAN_MANAGER::is_on_holiday` [0x4d16b3], `get_holiday_option(0, manager)` ≠ 0 [0x4d16ce]) and without bit 4
   (`test [contract+0x4e], 4` [0x4d16d7]): `staff::VALUE` (+0x52). Value (step 6) 0 → 0 [0x4d1a69].
2. **The league factor L** = 1 − (20 − y)² × 0.2 × (1/60) [0x4d17fc..0x4d1812], between 0.1 and 1.1
   [0x4d1818..0x4d1852]. x = `nation::STATE_OF_DEVELOPMENT` (+0x7e), 3 outside 1..3 [0x4d170c..0x4d1728]. With a
   division: x = (x + 1) × 0.5, × 4 (`PROFESSIONAL_STATUS` 2) or × 7.5 (3) [0x4d172e..0x4d174e]; y = (division
   reputation² × 25 + club reputation) / (x × 1000) [0x4d1754..0x4d177a]. Without one: x × 2 or × 7.5, without the
   (x + 1)/2, and y = (`nation::LEAGUE_STANDARD` (+0x85)² × 25 + club reputation) / (x × 1000) [0x4d17b3..0x4d17f2]. No
   nation: y = 1. L multiplies the premium [0x4d23c1] and the reputation premium [0x4d2482] (it is used).
3. **Age:** A = (1 − (age − 28) × 0.1)^2.0 (`_CIpow`, exponent 2.0 [0x968950]) [0x4d1865..0x4d188a]; it multiplies the
   premium [0x4d23c5] and the reputation premium [0x4d2486]. Age = byte `staff+0x18`.
4. **Happiness and role** (role = `SQUAD_STATUS` >> 4, jump table 0x4d4258) [0x4d188e..0x4d1932]: 1 → max(75 − h, 0); 2
   and 5 → max(50 − h, 0); 3 → max(40 − h, 0); 4 → max(25 − h, 0); **6 → max(−h, 0)**; 7 → h = −100 and shortfall 0; 0
   and 8+ → shortfall 0 (h = `contract::HAPPINESS` +0x3d). **Absent without leave** (`AWOL::is_player_awol(player, 1)`
   [0x4d1944]): shortfall 0, the adjust switched off and h ≤ −5 × the seller manager's `non_player::DISCIPLINE` (+0x14)
   (h = −100 without a manager) [0x4d194d..0x4d19c4]; otherwise the adjust comes from the 5th argument [0x4d19c6].
5. **Morale** [0x4d19cd..0x4d1a1a]: M = 1 + morale × 1/300 [0x968e60] (positive) or × 1/134 [0x969090] (zero or
   negative); **× 1.2 without grievances** (`GRIEVANCES` +0x45 = 0) [0x968978] and **× 0.8333 with grievances**
   [0x969088] (the old map had it inverted).
6. **The value V** [0x4d1a5a]: the "seen" figure is the **potential** as seen: `perceived_ability(the seller's manager
   (+0xcf), player, seller, 1, rep)` [0x4d1a43], kind 1 (potential), rep = `player_sheet::CURRENT_REPUTATION` (+0xb) /
   2 + `POTENTIAL_ABILITY` (+7) × 25 in 16 bits [0x4d1a21..0x4d1a35]; then `calc_player_value(this, player, seller, 0,
   seen, 0)` (only the low 16 bits of "seen" count). "Seen" is the only place the seller manager's judgement enters.
7. **Recent signing** (formerly read as "contract running out") [0x4d1aff..0x4d1b7c]: the days are **since the player
   joined the club** (`staff::DATE_JOINED_CLUB` +0x3e/+0x40), d = 365 × (current year − joining year) + current day −
   joining day, between 121 and 365 (negative → 121). f by role (table 0x4d4274, f32): 2 → 6.0; 3 → 5.5; 4 and 5 → 3.5;
   the others → 7.5; roles 7 and 8 out [0x4d1abc..0x4d1ac8]. C = f − d / (365 / (f − 1)): f up to 4 months after
   joining, 1 after a year; above 2 it sets 0x100. Conditions: adjust on, an AI seller, not listed nor asking to leave
   (0x544520). C enters Q, the cap D²C, the premium (× min(C², 25)) and the cap 3V × D × C.
8. **High happiness** [0x4d1b80..0x4d1c11]: H = (shortfall/2 + h) × 0.01167 + 1.4165; with h ≥ 50 and an AI seller, H =
   ((shortfall/2 + h)² / 50 × 0.006 + 0.7) × H. Reasons: h > 60 → 0x200, otherwise h > 20 → 0x400.
9. **The seller's finances** [0x4d1c15..0x4d1d6c]: the table 0x9a7f48 (21 doubles, by reputation / 500): 50 thousand,
   100 thousand, 200, 300, 400, 500, 750 thousand, 1, 1.25, 1.5, 2, 4, 6, 8, 10, 12.5, 15, 17.5, 20, 22.5, 25 million.
   B = `finance::BALANCE` (0 without finances). B ≥ T: B / T; B < T: ((B − T)/T + x) / (x + 1), x = B × 1.25e-6 between
   0 and 10. G = (that + n² × 0.05) + h × 0.002, n = the low nibble of +0xb of the 79-byte record [what n means is still
   **inferred**: the knowledge/scouting level]. G > 1 → × 0.25 + 0.75; otherwise × 0.2125 + 0.7875. Floor h × 0.0025 +
   0.5 (when the floor wins there is **no** cap); otherwise a cap of 2.5. G also caps Q (≤ 6G).
10. **The buyer** [0x4d1d83..0x4d2002]: D = 1 without a buyer. h' = h, or ftol(h × 1.5) when h > 20 without grievances.
    Celtic × Rangers: D = max(h' × 0.04 + 2.3, 2.3); league clash (`comp_util_get_league_clash_type(buyer, seller)` ≠
    0): same city (`stadium::CITY` +0x38) max(h'/30 + 2, 2), otherwise max(h' × 0.02 + 1.6, 1.3); the seller's rival
    (`club::RIVAL1..3`): same city max(h' × 0.02 + 1.6, 1.3), otherwise max(h' × 0.01 + 1.3, 1.3); these three set
    **0x800** with D ≥ 1.3; same division (outside `KEY_CLUB_COMP_A_LOWER_DIVISION`): max(h' × 0.005 + 1, 1), sets
    **0x1000** with D > 1.3. (The reasons are 0x800 and 0x1000, not 8 and 0x10: the `or` is on the high byte.)
11. **Importance in the squad** [0x4d2006..0x4d211c]: k = reputation/50 + 1, threshold = k × 17 / 20. With `reasons`,
    role 5 sets **0x2000** and does not compute. Seen > threshold, age < 25, role < 7: with `reasons`, role 6 sets
    0x2000 and does not compute; otherwise S = 1 + ftol((seen − threshold) × k × 0.005) × 0.04, at least 1; **role 4
    also multiplies H by S**; S > 1.6 sets 0x2000.
12. **The price:**
    - Q = G × H × M × S × D × C, at most clamp(D² × C, **4.25 is the floor** [0x969008], 10) and at most 6G
      [0x4d2124..0x4d21b5]. With D = C = 1 the cap is 4.25.
    - R = ftol((current reputation + `HOME_REPUTATION`) × 0.3 + `WORLD_REPUTATION` × 0.4); e = (club rep + R − 4000) ×
      225 between 150 thousand and 2 million; E = (((1 + (R − (1.2 rep − 2000)) × R × 5e-8) × u + 100) − u) × 0.01, u =
      h × 0.5 + 50; above 1, E⁵; E between 1 and 3 [0x4d21bd..0x4d22f2].
    - raw premium p = ftol(V × Q − V). Negative: max(p, −0.4 V), at least −3 million. Positive: min(p × C, ftol(e × Q))
      × min(C², 25) × L × A; half if listed or asking to leave; up to 50 million [0x4d2307..0x4d2403].
    - price = V + premium; with E > 1, + ftol(L × A × t), t = ftol(price × E − price), above 5 million 0.625 t + 1.875
      million, above 10 million 0.125 t + 8.75 million; at least 1000 [0x4d2410..0x4d2499].
    - **Work permit** (`player_needs_work_permit` [0x4d24bb]): at most V. **Listed or asking to leave:** the reasons keep
      only 0x800 and the price ≤ `staff::VALUE` × D. **An A team with a linked B team** and a role outside 1, 5, 2, 3:
      the same [0x4d24c4..0x4d257a].
    - **Rich club floor:** an AI seller, reason 0x800 already set, reputation > 4250, age > 18, role ≠ 7, wealth band ≠
      −1 and −2: if the price < (seller rep + 25000) × 100, it becomes (rep **of the player's club** + 25000) × 100
      [0x4d2585..0x4d261b]. The band only filters (it is not "an adjustment by the band").
    - At most max(3V, V + 5 million) × D × C [0x4d261f..0x4d267a].
13. **Grievances** [0x4d267e..0x4d27e3]: price > V and `GRIEVANCES` & 0x16b: days to the contract end
    (`CONTRACT_EXPIRES` +0x2d/+0x2f); V' = V (≥ 365 days) or ftol(days/365 × V); w = days × 0.0027 − 0.485 between 0
    and 1; p' = ftol((1 − w) V' + w × price); ratio = (V' + p') / max(2 × B × D, 1) between 1 and 3; y = 1.375 − 0.375
    ratio; price = ftol((1 − y) V' + y p'). It also applies with more than a year of contract (then only the balance
    pulls).
14. **Ladders A and B and the return to the value** [0x4d27ff..0x4d387e]: the **seller's** shortlist entry for the
    player (`find_club_shortlist_entry(seller, player, 0, 1)`). "Big nation": reputation > 4000, nation in {England,
    Italy, Spain, France, Germany, Scotland, USA, Japan, Holland, Portugal}, (`LEAGUE_STANDARD` > 12 or reputation ≥
    7000), finances with a band ≠ −1/−2, the player neither listed (0x5444a0) nor listed at his request (0x544600).
    - **Ladder A** (the entry targets the player, or `staff::SQUAD_SELECTED_FOR` = 2 with a continental competition): the
      youngster step (under 25, role outside 6/7, seen × 50 > reputation): min(seen / (125 − morale/4) × price, price −
      |age − 20| × 1,333,333 + 5 million). Small club (rep ≤ 6750 and division reputation ≤ 15): with grievances and
      morale ≤ 0, only with grievances 0x101 and more than 425 days of contract: role 1 +375 thousand/×1.25 (same
      division +50 thousand/×1.05), role 2 +200 thousand/×1.2 (+37.5 thousand/×1.025); otherwise: role 1 +750
      thousand/×1.75 (+100 thousand/×1.1), 2 +400 thousand/×1.4 (+75 thousand/×1.075), 3 +150 thousand/×1.15 (+25
      thousand/×1.025). Big club: with grievances and morale ≤ −50 (0x101 and > 425 days): role 1 min(+2.5M, ×2)
      (+1M/×2), 2 +1.5M/×1.5 (+750 thousand/×1.75), 3 +1M/×1.25 (+400 thousand/×1.25); otherwise, with a buyer, > 225
      days, role ≠ 7, professionalism and loyalty > 5, current reputation > 6250, happiness > −loyalty, aged 23 to 30:
      +1M/×1.25 and, in the same division with a league clash, +400 thousand/×1.25; without serious grievances: role 1
      min(+5M, ×5) (+1M/×2), 2 +2.5M/×2.5 (+750 thousand/×1.75), 3 min(+1.5M, ×2) (+400 thousand/×1.4). Each step is
      min(price + K, price × f).
    - **Ladder B** (empty entry, target −1): the youngster step under 22; small club without (grievances and morale ≤
      0): role 1 +250 thousand/×1.25 (+50 thousand/×1.05); role 2, or another with happiness > 40 − loyalty, +100
      thousand/×1.1 (+25 thousand/×1.025). Big club: with grievances and morale ≤ 0 (0x101, > 425 days): role 1 +500
      thousand/×1.25 (+250 thousand/×1.1), 2 +1M/×1.2 (+200 thousand/×1.05), 3 +750 thousand/×1.15 (+150
      thousand/×1.02); otherwise (> 225 days, role ≠ 7, loyalty + professionalism > 10, current reputation > 5250,
      happiness > 10 − loyalty, aged 23 to 30) +750 thousand/×1.15 (+150 thousand/×1.02); without: role 1 min(+2M, ×2)
      (+100 thousand/×1.1), 2 +1.5M/×1.5 (+75 thousand/×1.075), 3 +1M/×1.5 (+20 thousand/×1.02).
    - A big nation with an entry targeting someone else: nothing.
    - **No entry** and outside the big nation (or a big nation without an entry that does not fall in ladder A), price >
      V: h < 60 → 0.67 V + 0.33 price; otherwise t = 1.27 − h × 0.01 and (1 − t) price + t V [0x4d3826..0x4d387e]. It
      is the common case outside the big leagues: the price goes two thirds of the way back to the value.
    - The contract flags (0x8676f0), loyalty, professionalism and morale are not in the ladders: they are in the
      youngster floor (step 15).
15. **The youngster floor and the manager's Youngsters** [0x4d3886..0x4d402a]: with `flag` (6th argument), age < 24,
    seen > 110, a nation and role ≠ 7: si = seen; outside goalkeepers, age factor a = 2.6 − 0.07 age; physical =
    ftol(((stamina × 1.2 + strength) × a + teamwork × 0.8) + work rate × 1.2 + jumping × 0.75); dribbling and finishing
    (attributes 11 and 12) × a, up to 20; by pace + acceleration: 40 → +15/+10/+8/+5/+3 by physical (85/75/65/50/35) and
    + (dribbling − 13)², + (finishing − 14)²; > 34 → +8/+5/+3/+1, (dribbling − 15)², 0.8 × (finishing − 15)²; > 25 →
    +5/+3/+1, −3 with physical < 35, 0.8 × (dribbling − 15)², 0.5 × (finishing − 15)²; < 15 → −5/−8/−10
    [0x4d38d3..0x4d3ba2]. With the entry targeting the player: + rep/1000. Role 1: + rep/500 (under 19 at least 121); 2
    and 3: + rep/670 (19+) or rep/500; 4 under 19: + rep/670; over 20 with current ability < 45: − rep/1000; seen > 120
    and `club::TRAINING` > 10: + training − 10 [0x4d3ba5..0x4d3c98]. **The Youngsters** [0x4d3c9c..0x4d3cd0]: with a
    manager (`club::MANAGER`) and a non-player sheet, at an **AI club** (`is_human_club(seller, 1, 1)` = 0): si +=
    `non_player::YOUNGSTERS` (+0x22) − 10 (in 16 bits); it is the manager's only input besides the judgement in "seen".
    Then, if the player joined the club after the manager and si < potential: + half the difference
    [0x4d3cd4..0x4d3d15]. si > 120: si ≤ clamp(rep × 0.0167 + 100, 150, 220); bonus (si > 150, an entry, rep ≥ 2500,
    league standard > 10) = (min(4 si − 600, 200))² × 250 × rep / 6000; Y = ftol((si² − 14400) × 250 × 0.6153846 +
    bonus); standard < 13 or status 3: × r/5250 (× 0.75 at status 3 below 5250; standard < 5 above 4000: 4000 + (r −
    4000)/5); grievances: × 0.75 (morale < −50), × 0.8 (0x800000), × 0.95 (contract flag 8), × 0.9 (1), × 0.9 (0x40);
    loyalty < 6 and professionalism < 10: × 0.9 (morale < 0) and × 0.9 (buyer 750 above); loyalty > 15 without negative
    morale: × 1.1. The price is at least Y, with reason 0x2000 [0x4d3d17..0x4d402a].
16. **Age cap** [0x4d402e..0x4d4153], table 0x4d4288: a goalkeeper (`GOALKEEPER` = 20) counts 3 years less, so his cap
    starts at **32** (not 26); 29: value + 5M; 30: +4M; 31: +3.25M; 32: +2.5M; 33: +1.75M; 34: +1M; 35: +500 thousand;
    36: +200 thousand; 37: +100 thousand; 38 to 45: the value; above 45, nothing.
17. **Portugal**, price > V: < 6 million, x = (price × 1.667e−7 × 0.6 + 0.3) × price; otherwise 0.9 × price; the price
    = max(V, x) [0x4d416b..0x4d41eb]. **South Korea**, price > V and > 50000: (price − 50000) × 0.15 + 50000
    [0x4d41ed..0x4d421d].
18. **The active release fee** [0x4d422a..0x4d423e]: `has_active_release_clause` and price ≥ `RELEASE_FEE` (+0x21) →
    the fee.

**Defects of the original [proved]:** in step 4 with an absence without leave, the high 16 bits of `[esp+0x24]` keep
garbage (the `movsx ax` over the manager pointer), but only the low 16 are read: no effect. The rich club floor compares
with the seller's reputation and writes with that of **the player's club** (`staff::CLUB_JOB`), which differs for a
loanee or a player of another club [0x4d25ea / 0x4d2603]. The B team's `b` is not initialised; no effect because the
callee writes it whenever it returns a club [0x5412a7, 0x54187c → 0x541e42].

**Extension points:** the return of 0x4d1600 (20 callers) and `*reasons` (0x100 recent, 0x200/0x400 happiness, 0x800
rival/derby, 0x1000 same division, 0x2000 important/youngster); the value call [0x4d1a5a] (V enters linearly: scaling V
scales everything); the 4.25 floor of the cap [0x969008] and 6G (with D = C = 1, Q ≤ 4.25: the premium reaches 3.25 V);
the return to the value without an entry [0x968f80 = 0.67, 0x968f78 = 0.33], the rule that weighs most outside the big
leagues; the Youngsters [0x4d3ccb] (`movsx cx, [eax+0x22]` and `lea esi, [esi+ecx−0xa]`, the −10 is an immediate); the
wealth table 0x9a7f48 and the age cap (immediates 0x4d4067..0x4d4133).

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
- **`mka_2`** (`levas/fila/mka_2/`): `calc_asking_price` (0x4d1600), from the whole disassembly (§3). Compiled; the
  proof on the benches is left to the main session.


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
| 0x5a9a80 / 0x5a9af0 / 0x5a9ba0 | Finland: period, `can_loan_player`, `can_end_loan` (only the Finnish table 0x96a75c; as the borrower's rule they do not run in the 8 leagues, `can_end_loan` only if the AI borrows from a Finnish club) [proved] | no | **mkc_4** |
| 0x902240 / 0x90f1a0 / 0x7ec8e0 | `can_loan_player` of Turkey **and the USA**; the "Welsh" period of **11 nations** (Australia, Belgium, Croatia, **France**, Germany, Greece, Holland, Ireland, Korea, Turkey, Wales); the "Russian" period of **Brazil**, Poland and Russia: **they run in the 8 leagues** [proved] | no | **mkc_3** (8 leagues) |
| 0x794760 / 0x793870 / 0x793900 | `can_loan_player` of Norway **and Holland**; Northern Ireland's period (only it); `can_loan_player` of Denmark, Korea, Northern Ireland and Wales [proved] | no | **mkc_5** |
| 0x5e0210 / 0x63c530 / 0x63c4e0 | Germany `can_end_loan`, Ireland `can_end_loan` and `record_loan` (only their own tables, 0x96b3cc and 0x96c190) [proved] | no | **mkc_6** |
| 0x5ed060 | `can_loan_player` of Greece, Croatia and Poland: `mov eax, 1; ret 0x10` (8 bytes) [proved: 0x5ed060..0x5ed065] | no | nothing to port |
| 0x8e8560 / 0x8e8800 | base `can_sign_player` / `can_make_offer` (foreign player limits) | no | mapped here, not ported (they belong to signings, reader B) |
| 0x784100 / 0x7874b0 / 0x5fd2f0 | `NewsItem::auto_respond`, `News::auto_respond_all`, `HUMAN_MANAGER::set_on_holiday` | no | mapped (the holiday defect) |

Items mkc_4, mkc_5 and mkc_6 run with LIGAS = `ENG FIN NOR NIR GER IRL`; mkc_3 with the 8 leagues. All compiled in a
copy (with mkc_1 and mkc_2), mutations compiled, nothing run in the game.

**The 29 rules virtual tables [proved].** The list above came from the community map (`*_RULES::` names); checked
against the code: every rules virtual table has 0x40a5f0 in slot +0x14, and a scan of `.rdata` for that dword finds
**29 tables** (28 nations + the base), each written by a single constructor (`refs.py`; 15 by the constructors already
in Rust, `construct_nation_rules`, the others by the community map's names in the constructors 0x5c1570 France,
0x7ec770 Russia, 0x555460 Denmark etc.). Loan slots: +0 `daily_update`, +8 `record_loan`, +0x20 `CAN_LOAN_PLAYER`,
+0x24 `CAN_END_LOAN`, +0x28 `GET_LOAN_PERIOD`; "base" = 0x40a600 / 0x40a610 / 0x8e8bd0, "none" = 0x583470.

| Table | Nation | +0 | +8 | +0x20 | +0x24 | +0x28 |
|---|---|---|---|---|---|---|
| 0x9674a0 | Argentina | 0x40a710 | none | base | base | base |
| 0x967830 | Australia | none | none | 0x412dd0 | base | **0x90f1a0** |
| 0x967c10 | Belgium | none | none | base | base | 0x90f1a0 |
| 0x968838 | Brazil | none | none | base | base | **0x7ec8e0** |
| 0x969394 | Croatia | none | none | 0x5ed060 | base | 0x90f1a0 |
| 0x969860 | Denmark | none | none | 0x793900 | base | base |
| 0x969f4c | England | 0x57a020 | 0x579fd0 | 0x579d60 | base | 0x579c70 |
| 0x96a188 | Europe (generic) | none | none | base | base | base |
| 0x96a75c | Finland | none | none | 0x5a9af0 | 0x5a9ba0 | 0x5a9a80 |
| 0x96ae48 | France | 0x5c1710 | none | **0x412dd0** | 0x5c1ed0 | **0x90f1a0** |
| 0x96b3cc | Germany | none | none | base | 0x5e0210 | 0x90f1a0 |
| 0x96b784 | Greece | none | none | 0x5ed060 | base | 0x90f1a0 |
| 0x96ba88 | Holland | none | none | 0x794760 | base | 0x90f1a0 |
| 0x96c190 | Ireland | **0x57a020** | 0x63c4e0 | 0x63c3d0 | 0x63c530 | 0x90f1a0 |
| 0x96c96c | Italy | none | none | 0x661770 | 0x6618f0 | base |
| 0x96ccfc | Japan | none | none | base | base | base |
| 0x96cfec | Korea | none | none | 0x793900 | base | 0x90f1a0 |
| 0x96df9c | Northern Ireland | none | none | 0x793900 | base | 0x793870 |
| 0x96dfe4 | Norway | none | none | 0x794760 | base | base |
| 0x96e61c | Poland | none | none | 0x5ed060 | base | 0x7ec8e0 |
| 0x96e9c0 | Portugal | none | none | base | base | base |
| 0x96f340 | Russia | none | none | 0x412dd0 | base | 0x7ec8e0 |
| 0x96f838 | Scotland | none | none | 0x7f6650 | base | 0x7f6530 |
| 0x96fdec | Spain | none | none | 0x859630 | base | base |
| 0x970368 | Sweden | none | none | base | base | base |
| 0x970458 | base `TRANSFER_RULES` | none | none | base | base | base |
| 0x97075c | Turkey | none | none | 0x902240 | base | 0x90f1a0 |
| 0x970a38 | USA | none | none | **0x902240** | base | base |
| 0x970a9c | Wales | none | none | 0x793900 | base | 0x90f1a0 |

Consequences: several "one nation" routines belong to several, and **three run in the 8 leagues** (0x902240 through
the USA, 0x90f1a0 through France and Australia, 0x7ec8e0 through Brazil). 0x412dd0 (`AUSTRALIA_RULES::can_loan_player`,
mkc_2) is also the `CAN_LOAN_PLAYER` of **France** and Russia (tables 0x96ae48 and 0x96f340 +0x20): in the 8-league
career it also runs for French clubs. Ireland uses the English 0x57a020 (already in Rust) as its `daily_update` (+0):
the Irish clubs' season loan count is also zeroed on 20 June.

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
   - The common part of the rules below [proved]: `thiscall(rule, club, a2, &start_month, &start_year, &end_month,
     &end_year, player)`, `ret 0x1c`; start = today's month (0x54cfb0 on 0xae2c90) + k; year = [0xae2c92]; start > 11
     (signed): −12 and year + 1; end month = `contract_manager_season_end_month(club)` (0x4d45f0 with ecx =
     [0xae1050]); start < end (signed): end year = start year, otherwise today's year + 1. `a2` and `player` are not
     read;
   - Finland 0x5a9a80 (**mkc_4**): k = 1, the same computation as 0x8e8bd0 (which zeroes the month instead of
     subtracting 12: same result) [proved: 0x5a9a80..0x5a9aeb];
   - "Russian" 0x7ec8e0 (**mkc_3**; Brazil, Poland and Russia): k = 3. **Latent defect**: with the start rolling into
     the next year (today October to December) and the season end month ≤ the start month, the end falls in the start's
     year and before it (or on it); whether it happens with these nations' data was not checked [proved in the code:
     0x7ec8f4 `add al, 3`, 0x7ec924..0x7ec947];
   - Northern Ireland 0x793870 (**mkc_5**, only it): k = 3 and then, start after the end (unsigned year, then signed
     month), the start becomes the end: the Russian one with the fix for the defect above [proved: 0x793885,
     0x7938d7..0x7938f3];
   - "Welsh" 0x90f1a0 (**mkc_3**; 11 nations: Australia, Belgium, Croatia, France, Germany, Greece, Holland, Ireland,
     Korea, Turkey, Wales): k = 1 and then start = end (month and year) [proved: 0x90f1b4, 0x90f1ff..0x90f206].
     Consequence: in `can_loan_player` (§4 step 5) start = end returns 1 at once, without asking the owner's
     `CAN_END_LOAN`, unless 0x8d0590 moves the end by the contract (steps 2–4); in the AI (0x842740) the months to the
     start and to the end are the same.
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
   - signature: `thiscall(rule, club, player, message, &reason)`, `ret 0x10` [proved];
   - base 0x40a600 (Argentina, Belgium, Brazil, Europe, Germany, Japan, Portugal, Sweden) and 0x5ed060 (Croatia,
     Greece, Poland): 1; 0x793900 (**mkc_5**; Denmark, Korea, Northern Ireland and Wales): looks up the player's main
     contract (0x4cf5a0), stores it in `[0xae1080]`, computes his club and does not use it; 1 [proved:
     0x793900..0x793934];
   - England 0x579d60: same division (Premier only) 0x1d; **5 a season** (byte 0 of `[[0xde1f54]+0x91c] + id·16`,
     zeroed on 20 June by 0x57a020, counted by 0x579fd0) 0x1e; **more than 1** loanee from elsewhere already in the
     squad (the 3rd is refused) 0x26; outside the window (0x8e8e50) 9;
   - Australia 0x412dd0 (**mkc_2**; also France and Russia), Norway 0x794760 (**mkc_5**; also Holland), Turkey
     0x902240 (**mkc_3**; also the USA): `find_open_window(today, 0)` (0x8e8e50); closed: with a message the text with
     the rule nation's adjective (+0xa, form 2; "The deadline for loans to ... clubs has passed."), reason 9 (even
     without a message) and 0; open: 1. The Norwegian one uses text 0x9cf088 (example "Dutch", same format); the
     Turkish one, with the window open, looks up the contract and writes `[0xae1080]` before the 1 [proved:
     0x794760..0x7947cb; 0x902240..0x9022da];
   - Finland 0x5a9af0 (**mkc_4**): a closed window as above (text 0x987d38). Open: with a club and not human
     (`human_manager_is_human_club(club, 1, 1, 0, 0)` = 0), today's day of the year (signed word at 0xae2c90; 0 = 1
     January) > 300 or < 90: **0 with no text and no reason**. 90 = 1 April, 300 = 28 October (31 March and 27 October in
     a leap year); together with the window (16/11–15/8), the Finnish AI only borrows from 1 April to 15 August [proved:
     0x5a9b52..0x5a9b89];
   - Italy 0x661770, Spain 0x859630, Scotland 0x7f6650, Ireland 0x63c3d0: in Rust (windows and division limits).
4. The rule of the **current club's** nation (the lender): none, 1.
5. The period (§3). **Start = end: 1** (nothing else). Otherwise, month by month from the start to the end
   (inclusive), the owner rule's virtual +0x24 (`CAN_END_LOAN`, flag 1); the first month that lets the loan end gives
   1; past the end: 0, "This loan deal cannot go ahead". By nation (+0x24; `thiscall(rule, club, player, message,
   date, flag)`, `ret 0x14`; callers of 0x8d0530: screen 0x8fa341 with flag 1 and no message, the end-loan screen
   0x8fbcf9 with **flag 0**, message 0xde1f64, today's date and club = the player's `club_job`, AI 0x8429ae with flag 1;
   0x8d0040 always with flag 1):
   - base 0x40a610: 1 (England, Scotland, Spain...: the loop answers 1 on the first month);
   - France 0x5c1ed0 (**mkc_2**) and Ireland 0x63c530 (**mkc_6**, the same code with the Irish key): a null date =
     today; `find_open_window(date, 0)`; closed: 0 and, with a message, the nation's adjective ([0x9cf2dc] for Ireland,
     form 10) in 0x9cd030 ("sent back", when `club` is the player's `club_job`) or 0x9ccfbc ("recalled"). The Irish
     window: 10 May to 31 January [proved: 0x63c530..0x63c5d9];
   - Italy 0x6618f0 (**mkc_2**): with the flag, June; otherwise the date in the 1st or 2nd Italian window;
   - Finland 0x5a9ba0 (**mkc_4**): refuses with 18/8 of the date's year **before** the date and the date **before**
     15/11 (strict at both ends; unsigned year, then signed day; dates by 0x549f70(18, 7, year, −1) and (15, 10, year,
     −1)). Texts 0x9cc1a4 ("cannot be sent back until 15th November") / 0x9cc170 ("recalled"), with no adjective. The
     date is not checked (a null one would crash; no caller passes null); the flag is not read [proved:
     0x5a9bb9..0x5a9ca5];
   - Germany 0x5e0210 (**mkc_6**): flag ≠ 0: 1. Otherwise it refuses when the player's club (`club_job`, +0x39) is
     German ([0x9cf314]) **or has no club or nation**; another country: 1. With a message, the German adjective (form 2)
     in 0x9cd5cc ("Players on loan from other German clubs cannot be sent back until the end of the loan.") or 0x9cd574
     ("...cannot be recalled."). Since 0x8d0530 only calls the rule with the owner and the club in the same nation, it
     is "between German clubs". **Only the end-loan screen (flag 0) refuses**: in the loan check (0x8d0040) and in the
     AI request Germany never refuses. The date is not read [proved: 0x5e0210..0x5e02bf; flag 0 at 0x8fbce2].
6. **Ireland's `record_loan`** 0x63c4e0 (**mkc_6**, +8 of the Irish table): the offer's club (id at +8 in the table
   [0xae23b4], 0x245 bytes) with nation id [0x9cf2dc] (Ireland): byte 0 of the 16-byte row `[[0xde1f54]+0x91c] + id·16`
   + 1. The same code as the English 0x579fd0 with the Irish key; it is the count that the Irish `can_loan_player`
   (0x63c3d0) caps at 5, zeroed on 20 June by 0x57a020 [proved: 0x63c4e0..0x63c521].

**Defects and surprises of the rules [proved in the code]:** (1) the "Russian" period (Brazil, Poland, Russia), with
the start in the next year and the season ending in January or February, puts the end before the start (or on it); the
Northern Irish copy fixes it, the Russian one does not; latent: it depends on these nations' season end month (not
checked). (2) A player without `club_job` (or with a club without a nation) falls into the German refusal with the
"recalled" text; in practice 0x8d0530 does not call the rule in that case. (3) Not a defect, but a surprise: the Welsh
period gives start = end for 11 nations, so for France and Australia (and the others) `can_loan_player` accepts without
the owner's rule.

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
    Italian/Irish owner outside the window, or a Finnish one in the autumn (18/8–15/11, §4), "This loan deal cannot go
    ahead"; a German owner never refuses there (flag 1). And, for a borrower under the Welsh period (11 nations, France
    and Australia included), the start equals the end and the owner is only asked if 0x8d0590 moves the end by the
    contract [proved, §3 and §4].
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
- **Natural end:** `CONTRACT_MANAGER::daily_update` 0x4c7ac0 (at slice 2) calls `end_loan` (0x4d07e0, at 0x4c8114) on
  the end date: the loan contract (index +4 of the 79-byte record) ending today; without a club, only the index becomes
  −1; otherwise news 6 (0x4da590, outside type 7) and `end_loan` [proved: 0x4c8077..0x4c8114; section D §2.4].
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
| rule vtables (+0x20, +0x24, +0x28) | one nation's rule | swap the slot pointer in the nation's table (the 29 tables in §1; e.g. England 0x969f4c, Scotland 0x96f838, Italy 0x96c96c, Ireland 0x96c190, Spain 0x96fdec): it changes only that nation; detouring the shared routine (e.g. 0x90f1a0) changes all that use it (11). The Irish season count sits in the same table as the English one (`[[0xde1f54]+0x91c] + id·16`) |
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
| 0x844780 | `staff_interest_in_job_at_club` (the staff's will, §2.1b) | shortlist_manager.cpp | no (item mkd_7) | **mkd_7** | 0x4ce92c (`resolve_contract_offers`, check 0), 0x860339 (0x85fc50), 0x69fc3d (`is_staff_suitable_for_job` 0x69f810: job 5, check 0), 0x840682 (0x840084: check 1), 0x807ebc (search filter 0x807450, check 1) [proved] |
| 0x4ce4e0 | `CONTRACT_MANAGER::resolve_contract_offers` (the player picks an offer) | contract_manager.cpp | no | **mkd_4** | `process_due_decision_dates` (0x4cd9c0, single caller at 0x4cda4c), type 3 decision dates |
| 0x85fc50 | `STAFF_CONTRACT::set_to_desired_contract` (wage demand and length) | staff_contracts.cpp | no (item mkd_3) | **mkd_3** | `offer_contract` (0x4cda70), `lowest_wage_demand` (0x4ce450), 0x4ce4e0, `make_club_bids` (0x84a020), `accept_transfer_bid` (0x8c0080), `move_player_to_club` (0x8c9720), `get_contract_satisfaction` (0x866620), `calc_contract_value` (0x866830), screens |
| 0x864550 | `STAFF_CONTRACT::default_staff_contract` (default wage) | staff_contracts.cpp | no (item mkd_2) | **mkd_2** | 0x846920 (wage gate, 0x849957), 0x85fc50 (×3: 0x860310, 0x8604ad, 0x860523), 0x844780 (0x84668f), 0x842e10 (0x84371d), 0x83f3a0 (0x840893), `update_wage_bill` (0x5983ee), 0x85e960 (0x85eca3) [proved: `refs.py`, 9 calls] |
| 0x862720 | `staff_contract_calc_unset_default_wage` (§2.3) | staff_contracts.cpp | no (item mkd_6) | **mkd_6** | `set_default_wage` (0x8626f6: person, `club_job`, 1, −1, 11, 0), `create_initial_shortlists` (0x83a2c3, only contracts with wage −1, with the contract's status) [proved] |
| 0x8626e0 | `STAFF_CONTRACT::set_default_wage` | staff_contracts.cpp | yes | — | regens (0x7a70e4) |
| 0x866830 | `STAFF_CONTRACT::calc_contract_value` (value of a contract against another) | staff_contracts.cpp | no | **mkd_1** | `get_contract_satisfaction` (0x866620, ×2), 0x4ce4e0 (×2), 0x4d4830 (screen) |
| 0x866620 | `get_contract_satisfaction` (1..3) | staff_contracts.cpp | yes | — | the staff's daily pass (0x5ce2d0, random(50) = 0), screen |
| 0x4cda70 | `contract_manager_offer_contract` (the club's offer) | contract_manager.cpp | yes | — | 0x4c7ac0 (×9: renewals), screens |
| 0x4ce450 | `contract_manager_lowest_wage_demand` | contract_manager.cpp | yes | — | contract screen |
| 0x4c7ac0 | `CONTRACT_MANAGER::daily_update` (contract end, renewal, free agents) | contract_manager.cpp | no | — (too big; see §2.4) | `game_main_day_loop` (0x5cd406) at every slice of the day: slice 0 the fixed dates, slice 2 the people; `game_run_to_start_date` (0x5cc9ef) |
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

#### 2.1b The staff's will (0x844780, original; item **mkd_7**) [proved]

The will of a person with a staff sheet to take a job (jobs, `enums.rs` `job_kind`: 5 manager, 6 assistant manager, 7
reserve team manager, 8 coach, 9 scout, 10 physio, 11 player, 12 player-manager, 13 player-assistant, 15 player-coach,
16 retired player, 17 media pundit). Ghidra's pseudo-C is misleading here: the `_ftol` calls show no argument and the
ambition weights (0x844a9f..0x844b25), the weighted reputations and the board's confidence disappear; everything below
is from the disassembly.

Convention: thiscall(shortlist manager, person, club, check, job, &reasons), `ret 0x14` (0x84680b), SEH frame, f64 in
st0; ecx is only read for the clubs' shortlists (+0xdc, 0x845809, 0x84600a). Callers (`refs.py`): 0x4ce92c
(`resolve_contract_offers`, check 0), 0x69fc3d (`is_staff_suitable_for_job` 0x69f810: job 5, check 0), 0x840682
(0x840084, the AI offers: check 1), 0x860339 (`set_to_desired_contract` 0x85fc50) and 0x807ebc (search filter
0x807450, check 1). Same in cm0102-1080.exe (0x21a0 bytes).

- **Callers:** **[proved]**, with the sites and arguments: 0x4ce92c, 0x860339, 0x69fc3d (job 5, check 0), 0x840682
  (check 1), 0x807ebc (search_filters.cpp, check 1) (proof: `refs.py`; 0x69fc2a..0x69fc3d, 0x84066f..0x840682,
  0x807ea5..0x807ebc)
- **The filters that give 0:** **[proved]**: no staff sheet, no club or person nation; **Dario Gradi, Pat Nevin and
  Kevin Keegan with a club job** (by the formatted name, 0x53dba0); club or national job 17; outside job 5, the last
  16 records (id ≥ count − 16); a national manager (job 5 for the nation); with a job offered ≠ 0: a manager (5/12
  at club or nation) with current reputation > 4250 or ability > 100 wanting a job other than 5; a physio (10) asked
  for another job, or someone with another job asked to be physio; a player (11) with a club asked by another club
  (outside jobs 5, 12, 15); a manager whose current club has a division of reputation greater or equal to the new
  one's, or a reputation above the new one's − 750; a club below 2750 of the current club's nation or that finds him
  foreign (0x53d980) (proof: 0x8447d2..0x844a84)
- **The weights:** **[proved]**: w_stadium = 1.4 − ambition × 0.025 − professionalism × 0.025; w_division = ambition
  × 0.02 + 0.8; w_competition = ambition × 0.05 + 0.5; w_manager = 0 for jobs 5 and 12, else 1; denominator = the
  sum + 1 (0.5 if ≤ 0) (proof: 0x844a87..0x844b25)
- **The current club:** **[proved]**: the `club_job`; without one, or when the club offered is a national side
  (0x53b900), the national side (except job 11 there); a current club without a nation: returns 3.5 (proof:
  0x844b30..0x844b8c)
- **The base:** **[proved]**: 3.5 at his own club; otherwise 0 with a current contract of the same language and
  grievance 0x20; 2.25 (jobs 5, 12) or 1.5, plus, without a club, (days since the later of the game's start
  [0xdd6ec8] and joining the club, at most 500)² / (current reputation + 900), at most 12 (proof:
  0x844b97..0x844d22)
- **Likes, rivals, managers:** **[proved]**: dislikes the club × 0.33 (2); the current one × 2 (4); favours the
  current one × 0.85 (current amateur and new professional) or × 0.5 (8 with a contract without Bosman); favours the
  new one × 3 (0x10); rival or league clash (0x4b02e0): Celtic × Rangers 0.5 − loyalty × 0.025, Italy 1 − loyalty /
  60, a rival of the same division 0.75 − loyalty × 0.025 (difference < 1250) or 0.8 − loyalty / 60, else 0.85 −
  loyalty × 0.025 (0x20 below 0.7); without a rival, Croatia × Yugoslavia × 0.05 / 0.1 (2); the new club's manager
  favours × 2 (0x40), dislikes × 0.33 (0x80), outside jobs 5 and 12; the current manager dislikes × 2 (0x100),
  favours × 0.5 (0x200); with a future transfer: 0 (0x10000000) (proof: 0x844d34..0x8450d3)
- **Morale and jobs:** **[proved]**: the current contract's morale: at his own club × (1 + morale × 0.0025), else ×
  (1 − morale × 0.0025) (0x400 above 1.5, 0x800 below 0.5); at his own club the routine jumps from here to the end.
  By the current job (table 0x846810): chairman × 0.01; directors × 0.01 (× 0.5 if offered manager at a club ≥
  6000); manager × 0.45 (0x4000000) or × 0.65 without a club; assistant × 1.5, reserves × 2.2, coach × (ambition ×
  0.05 + 0.8) (0x8000000 if offered manager); 16 (retired) × 10 / ambition; player and others × 10 (0x8000000). By
  the one offered (table 0x846850): 0 → 0; chairman × 10; directors × 1/0.85; manager × 1/0.45 (× 1.5 if already a
  manager); assistant × 0.5; reserves and coach × 0.4; scout and physio × 0.4 (already one of them) or 0.25, × 0.25
  with a player sheet; player and others × 0.1; a current manager × 0.4 in the other jobs. A manager with the
  board's confidence (+6 + +8 of the 0xb67a44 row + 500 × (loyalty + determination)) below 8000: × 1.4 (proof:
  0x845127..0x845438)
- **The weighted reputations:** **[proved]**: for the new club, e = ftol(1.5 × (w_comp × competition + w_div ×
  division + w_man × manager + w_stad × stadium + shortlists) / den − 5000), with the stadium and the division of
  0x846920, competition = 600 × the continental one's reputation (16 bits) or 0.8 × division up to 7000, the manager
  = his reputation as the person sees it (0x5407e0) weighed by the board's confidence (0.67 × +6 + 0.33 × +8),
  shortlists = 0.4 × the shortlists' reputation + 0.6 × the reputation by wealth band (0.7 / 0.85 / 1.25 min. 4250 /
  1.5 min. 5750), × 1.45 at a national side of his nation; a non-professional club: min(e/2, e − 5 × (international
  caps² + 400)), status 3 again min(e/2, e − 2000). For the current club the same with other constants: the manager
  by his current reputation (div × 0.75 without a manager), bands 0.75 / 0.9 / 1.25 min. 4750 / 1.5, national side ×
  1.35, the Intertoto rule. Without a current club: the average of the sheet's three reputations (proof:
  0x84543c..0x845955, 0x845bd8..0x846122)
- **Language, foreigner, time:** **[proved]**: × max(F × 0.1, (e + 5000)² / 15000 / (12000 − 200 F)) + 0.2 (F =
  fluency, 10 without a player sheet) at the new club (0x10000 below 0.6), ÷ the same at the current one; a
  foreigner (no club, or foreign in the club's nation with a nation other than the current club's): × 0.85 / 0.75 /
  0.65 / 0.5 / 0.01 by job when the club's manager is his compatriot, else × 0.25 / 0.3 / 0.225 / 0.2 / 0.001;
  without a club × 2.5; with a club, 0 below (loyalty + 8) × 15 days at the club, and t = min(days² / 20 / (loyalty
  + 6), 400) × 0.0025 (proof: 0x845955..0x845bd0, 0x846122..0x8461ce)
- **The scales' difference:** **[proved]**: a null division or the key division 0x9cf764: × 0.1; g = 1 + (club
  reputation − the sheet's average) × 0.000125; y = (e_new − e_current) × (ambition × 0.0877 + 0.75) going up, × 1.5
  going down (jobs other than 5/12), or for a manager × min(max(1.1 − will × 0.01, 0.1), 1) with a will > 10; y > 0
  with t < 1: × ((150 − min(y × 0.02, 100)) × t + y × 0.02) / 150 (0x100000 below 0.6); a manager offer to a
  non-manager, y < 0: y × max(1 − will × 0.04, 0.4) (coach, assistant) or max(1 − will × 0.1, 0.2); × max(1 + y /
  4500, 0.001) (0x400000 / 0x800000 / 0x200000 by ambition and the competitions); the nations' development: equal, a
  European going outside Europe × 0.4; different, × (dev_current² × div_new) / (max(div_current, 500) × dev_new²)
  within 0.01..10, × 0.2 if from another continent and < 1 (0x2000000); × g (0x800000 below 0.6) (proof:
  0x8461d7..0x846628)
- **The wage:** **[proved]**: when checking with a will ≥ 1.75: no finances, 0; q = max(3.5 / max(will, 0.1), 0.9);
  asked = q × the default wage (0x864550 with a local contract, check 0, status −1); above the staff cap (0x597db0):
  will × cap / asked and the asked wage becomes the cap; with a current contract: ratio = asked / max(wage, 10);
  below 0.1 or below 1, the will; above 10, × 10 / q; otherwise × ratio / q (proof: 0x84662e..0x8467ec)
- **In §2.1 (the call at 0x840682):** **[proved]**: 0x840682 calls 0x844780 with (person, club, 1, the vacancy's
  job, 0) (proof: 0x84066f..0x840682)

#### 2.2 The player picks among contract offers (0x4ce4e0, original; item **mkd_4**) [proved]

`thiscall(contract manager, person)`, `ret 4` (0x4cf493), SEH frame; single caller 0x4cda4c, in
`process_due_decision_dates` (0x4cd9c0), when a type 3 decision date falls due; the same in cm0102-1080.exe (0xfb6
bytes). Constants read from the exe: 1/300 (0x968e60), 3.15 (0x968e58), 0.99 (0x968e50), 0.6 (0x968e48), 0.7
(0x968e40), 3.5 (0x968e38), 0.9 (0x9689d8), 0.8 (0x9689b0), 0.5 (0x968968), 10 (0x9688b0), 40 (0x968e30), 5
(0x9688a0); 2.75 (0x4ce51b) and 3.15 (0x4ceaab) are immediates, not in `.rdata`. Order [proved, disassembly]:

1. k = (professionalism − ambition − 9 + loyalty) × 1/300: the int (+0x5b − +0x57 + +0x59 − 9) goes through `fild`
   and `fmul` 0x968e60 (0x4ce5a4..0x4ce5ca).
2. With a pending transfer offer (`player_has_pending_offer` 0x8cd150): today + 2 (0x54c5b0) and a type 3 decision
   date at the current slice (0x4cd6d0 with [0xae2c88]); nothing else (0x4ce5ce..0x4ce606).
3. For each of the person's offers with +0x62 ≠ 0 (0x4ce630..0x4ce6fa): an offer without a transfer (+0x59 = −1) from a
   club that is not the contract's (looked up with the store in [0xae1080]) and type ≠ 5: removed (taken out of the
   list, counting down byte +8 of the 79-byte record). State +0x50 ≠ 0 is skipped; = 2 sets "agreed" and in the end
   nothing is chosen (0x4ce701, 0x4cec69..0x4cec6e). State 0:
   - type 5: will = 0x846920(…, loan 1, status 8); type 6: with the status from `get_squad_status_for_club`. The will
     is the score, chosen **at once** when above the best and ≥ 3.15, before the refusals (0x4ce70c..0x4ce78c,
     0x4ce791..0x4ce827);
   - the others: the status (high nibble of +0x4f) first goes to byte +5 of the person's entry in the transfer offer
     (0x8c61e0, 0x8e82f0); the desired contract at the club (0x85fc50 with −1, the status and job +0x61) is **one**
     local contract reused from offer to offer; ratio = value(offer against desired) / value(desired against desired)
     − k (`calc_contract_value` 0x866830); a desired value ≤ 0 gives score 0 (0x4ce82c..0x4ce8f1);
   - the will: 0x846920 without a non-player sheet (+0x69 = 0) or with job 11; otherwise 0x844780 and, if
     `is_player_job` (0x53b620), (0x846920 + 0x844780) × 0.5 (0x4ce8d8..0x4ce9b8);
   - **renewal** (the contract's club, looked up again in [0xae1080], is the offer's): ratio ≥ 0.99 and will ≤ 3.15 →
     will = 3.15 (0x4ce9bc..0x4ceaab); the ratio < 0.6 → 0 cut applies to **all** plain offers, not only the renewal
     (0x4ceac5..0x4cead0);
   - refusal (0x4ceae2..0x4ceb70), for every type (loans and trials included): (a) a job other than plain player
     (0x53b610) for someone without a non-player sheet (+0x69 = 0) when no spare non-player record is left
     (`PLAYER_REGEN::has_spare_non_player_record` 0x7a6e20 = 0); (b) ratio > 0.5, a work permit needed (0x8c7560) and
     the end of **the offer's** contract under 90 days away (0x54cdb0 of +0x2d to today < 0x5a);
   - score (0x4ceb9b..0x4cec67): ratio ≥ 0.9 → will = max(will, 3.15); ratio > 0.8 and will > 3.15 → kept; ratio >
     0.7 and will > 3.5 → max(will, 3.15); otherwise turned down. Score = will × ratio; above the best (starting at
     2.75) → chosen; a renewal not chosen (or turned down) is remembered. Loans/trials (desired = 0) do not go through
     here.
   An agreed offer, or a person with a non-player sheet who is a candidate for a manager vacancy
   (`manager_manager_is_vacancy_candidate` 0x69fc60), chooses nothing (0x4cec90..0x4cecab).
4. The chosen one becomes state 2 (news 0x7549b0, `set_agreed_contract_flag` 0x8c5410); the others state 1 (news
   0x4d49a0, or `handle_player_rejected_terms` 0x8c28c0, which gets the chosen offer as its 3rd argument, when tied to
   a transfer); news 0x4d49a0 gets 1 only for the turned-down renewal of a player (job 11) when nothing was chosen
   (0x4cecaf..0x4ced73). With nothing chosen, a **turned-down** renewal sets bit 0x80 of byte +0xb of the 79-byte
   record; with a choice, the bit is **cleared** at the end, except when the completion fails (0x4cf071..0x4cf094,
   0x4cf42d..0x4cf447).
5. **The loyalty draws** (the chosen one of type ≠ 5, 6; 0x4cef64..0x4cf06f): score > 10 → n = max(40 − score, 5);
   `random(ftol(n))` < loyalty, the offer's club has a manager, and club reputation > 4500 or `random(4)` = 0 →
   `random(2)` ≠ 0: comes to favour the manager (`staff_add_favoured_staff`); = 0: the club (`staff_add_favoured_club`).
6. **Third pass** (0x4cf096..0x4cf3a2): with a choice, the same club's offers without a transfer **complete** the
   contract (`complete_accepted_contract_offer` 0x8c38b0 with the local copy; on success: a staff record for a staff job
   without a sheet, the coaches and the scouts' lists when the contract's club is human, the player value recomputed);
   they leave the list and clear the agreed flag; with a transfer they stay. Other clubs' offers leave (into the held
   list +0x38 when the chosen one has a transfer; the club into the list 0x8c2180 gets). With none chosen, every offer
   outside state 3 leaves. A failed completion ends there.
7. **End** (0x4cf3a8..0x4cf42d): chosen with a transfer → `handle_player_accepted_terms` (0x8c2180: person, slot, copy,
   the job dword +0x61, the clubs list), first a staff record for a staff job without a sheet; without a transfer →
   news 0x4d49a0 of the copy.

#### 2.3 Wage demand and length [proved]

- **0x85fc50 `set_to_desired_contract`** (original, 10,896 bytes; item **`mkd_3`**, ported from the whole
  disassembly) [proved, `pacotes/0x85fc50`]: the contract the person wants at the club.
  `thiscall(contract; person, club, offer, squad status, job, reasons, &will)`, `ret 0x1c` (0x862688), returns an int
  (the contract value). The 6th argument is not a "kind": it is the pointer to the reason bits, passed whole as the
  last argument of 0x844780 (0x860329) and 0x846920 (0x8609db); 0x85fc50 does not use it otherwise [proved]; arguments at [esp+0x278..0x290] after the prologue; C++ handler 0x962a76. The counts match:
  `calc_player_value` at 0x85ffc8, 0x8615e5, 0x8617c5, 0x8618cd, 0x8619ed, 0x861af2, 0x861c53; the default wage at
  0x860310, 0x8604ad, 0x860523; `get_max_wage` at 0x861274, 0x8612a2, 0x8612d4, 0x861302. The order:
  1. Clears +0x21 and +0x20; without a club, the club becomes the current one (or the linked B team,
     `club_get_linked_b_team` 0x540a50) (0x85fcb0..0x85fd18); the contract with the current club
     (`get_contract_for_club`, 0x85fd2b) is "the current contract".
  2. +0x35 = (+0x35 & 0x40) | 2 (type 2), +0x49 = 0, +0x4a = −1, +0x3b = 0, +0 = id, +0x3a = 0 (0x85fd34..0x85fd56).
  3. Status −1 with a sheet: the deserved one (`get_deserved_squad_status` 0x4ca470) against the current contract
     (grievance 0x10 and not 1 → 2) or `get_squad_status_for_club` (0x849f00); goes to the high nibble of +0x4f
     (0x85fd73..0x85fe40).
  4. Level (byte): with a club, reputation / 500 + 1 (professional status 2: −1, within 1..4), +4 = the club id;
     without a club, the national team's / 500 + 1 (at least 3) or the world reputation / 500; + ftol(league × 0.25 ×
     int_apps × 0.125) (1..4) for internationals (0x85fe3e..0x85ff42).
  5. A player's signing-on value (+8): `calc_player_value` / 5 (0x85ffc8..0x85ffe2), × 0.25 (no club_job, current
     < 2750, club < 2000), × 0.5 (< 4250) or × 0.75 (< 4750); renewal: 0 (< 19 years and < 3750), × 0.1/0.25/0.5 under
     21, × 0.5 at a club < 3500; × 0.9 / 0.75 / 0.75 / 0.5 / 0.25 by the deserved status (table 0x86268c); new club:
     × 0.5 (current < 4250) and × 0.5 (league < 10) (0x85ffe5..0x86016b). Staff: 0 (0x860177).
  6. Without `job_for_club`: the "has no job for club" message and +0x3d = 11 (with a sheet) or 9
     (0x86017e..0x8601a6, 0x8602f4).
  7. Staff (0x8602f8): w = the default wage (flag 0, kind 0); will = `staff_interest_in_job_at_club` (0x844780),
     written to *&will; w + 2500 with will < 1.75; value = w × max(0.9; 3.5 / max(will; 0.1)) (0x861333..0x86137e); job
     6: 250000 if > 2 × reputation + 1200 and > 4100; 8: > reputation + 500 and > 3200; 9: > 0.75 × reputation + 250 and
     > 2100; 10: the same with 1500; and always at most 250000 (0x861375..0x861492). **The "250,000 cap" is the "not
     available" marker (0x48742400 = 250000), not a wage cap.**
  8. Player, renewal at the own club (0x8601e2..0x86045a): the default wage with flag 1 (at most the club's cap) when
     the renewal is "cheap" (no grievance, without the 0xf mark of the 79-byte record 0xae2d0c, current < 7250, fewer
     than loyalty × 12 + 182 days to the end and **not** leaving on a Bosman (the first test requires +0x49 = 0 in the
     contract: `test al, al; je 0x86041f` at 0x860240..0x860245 [proved]); or pressure < 6, < 24 years, no future
     transfer, not listed, < 3250 and below the club's reputation + 250, < 200 − pressure days; or a grievance without
     the 0x161a8 bits and few days; or loyalty > 18 / current < 5250 without the mark, with world < 3500); else flag 0.
     New club (0x86045f..0x860528): kind 3 with current > 6750, else 0 (the tests ambition + loyalty > 80 and > 100,
     which would give kind 1/2 and status 1/2, never hold with attributes 1..20).
  9. Player: a demand above the current wage: at most max(2 × current; current + 8000), and for the famous (world >
     6750, > 10000) above that, the cap 75000 (England/Spain/Italy) or 50000, 25% above (0x86052c..0x860699). The
     ambition: t = world + 25 × (ambition − loyalty) + 10 × pressure (16 bits); the boost at a major nation for 19..31
     years (finances +0x10 [inferred: the map reads it as the wage budget; `fields.rs` names it
     `finance::HIGHEST_WAGE`, the highest wage on the payroll], up to 0.0001 × t × 85000/105000 in England/Spain/Italy, goalkeeper up to
     60000) × (0.9..1.0 + adjustments by age, position and (ambition − professionalism − loyalty + 4) × 0.0075 within
     −0.25..0.15) (0x8606c7..0x8609cc).
  10. Will = `staff_interest_in_joining_club` (0x846920), written to *&will; q = 3.5 / max(will; 0.1); w + 5000 with
      will < 1.75 (no current contract, or a grievance and young/listed/...); q at least 0.9; grievance 2 already gives
      1.25 at 0x860a35, for any current contract; on renewal, grievance 2, or exactly 0x10 with age < 21 and reputation
      < 4250, gives 1.2 (0x860b9d..0x860bcf); the caps 1.75 / 1.5 / 4.0 apply to grievances 1 / 0x8400 / others
      [proved]; 1.25/1.5/2.0/1.35/1.15 by age and deserved status
      (tables 0x86269c, 0x8626b0), 1.1 for the young (< 20, ability < 125 − pressure − ambition, current < 4250, < 120 −
      pressure − ambition days), 3.0 without a contract (0x8609ce..0x860e27); value = q × w (0x860e2f).
  11. At least the current wage × (club reputation / the current club's, at least 0.5) when the club is weaker
      (0x860e4b..0x860ed6); with grievance 0x1000000, up to 95% of finances +0x10 (wage budget or highest wage on the payroll, see step 9 [inferred])
      (0x860ed8..0x860f0c).
  12. "Not available" markers (250000): value > 200000; > 35000 at 35+; > 75000 at 32+; goalkeeper 20 > 105000; status
      7 > 5000, 6 > 8250, 5 > 17500, 4 > 35000, 3 > 57500 (world < 6750; human club `human_manager_is_human_club`
      0x600a50, or outside the 85000/105000/125000 bands), 2 > 125000 (world < 7250) (0x860f10..0x8610f0).
  13. A dearer current contract at another club: value = ((20 − a) × current wage + a × min(w; max(get_max_wage; w ×
      0.5))) × 0.05, a within 0..20 (grievance 0x100 +10, listed +10, wage > 40000, age 34+, happiness and morale)
      (0x86110b..0x86132a).
  14. Clauses (only a player with job 11): +0x1d=0/+0x20=1 for the veteran; non-promotion (+0x1c), relegation (+0x1f),
      minimum fee (+0x1d), non-playing (+0x1e), with the fee (+0x21) by `calc_player_value` and minimums 150000 / 50000
      / 150000 / 1500000; the league ones through the competition virtuals +0x98/+0x9c (type 2) (0x86149a..0x861d1b).
      The fees of the no-promotion, minimum-fee and no-play clauses [proved]: extra = ftol(max(value × 0.95 + 150000, 2 ×
      value)), a **maximum** (0x861636..0x861647); above 2,500,000 the extra becomes ftol(e × 0.21): the original subtracts and adds the same 2,500,000 (0x861657..0x861667), which looks like a slip for (e − 2,500,000) × 0.21 + 2,500,000; the fee drops from ~2.5M to ~525 thousand [proved; the intent, inferred];
      the extra applies only without a grievance (for no-promotion, without bit 1 of the current contract's grievance).
      The relegation clause uses value + ftol(value × random × −0.5), at least 50000, × 0.5 / 0.75 / 0.85 by ambition
      and loyalty.
  15. Dates (0x861d1f..0x862371): `date_started` (+0x25) by the window (0x8cf470, or 0x8cf0e0 by the offer) and right
      away rewritten with today (0x861e14); end = day `utils_random(0x1c)` + 1 of the season-end month (0x4d45f0) of the
      year (the next one, by the table 0xb63d77 of the key nation 0x669360) + ftol(L + 1) years; L by age (3; 4;
      3/2/1.5 by position; 1 or 4), × 0.75 with a clause, by level and loyalty, by status (table 0x8626c4), by
      reputation, Finland/Turkey × 0.5 (at most 3), wage < 500/1000/2000 → at most 2/3/4, at most [0xa83608] − 1 = 9, at
      least 1; type 2 with bit 0x80 (no `club_job`, level < 2 and ftol(fixed draw × 4, or × 5 below level × 500 + 1250
      of reputation) = 0); in that last case, up to 20 years, type 1 (+0x35 & 0xc1 | 1) with no extra years; on renewal
      the end never moves back (`date_expires_club` +0x46).
  16. Bonuses (+0x10/+0x14/+0x18) = ftol(value × fixed draw × 0.025 + value × 0.025 + 25) for attacker / midfield /
      goalkeeper 20, else −1; +0x35 |= 0x80 with value < 125 at a club of status 2, cleared above 1250; **+0xc =
      ftol(value)** (0x86237a..0x862493).
  17. Return: ftol(wage (× 0.5 for type 1, × 1.05 with 0x80) + min(sum of bonuses/5 and of clauses × 0.013..0.1;
      value × 0.1)) (0x862496..0x86266d).
- **0x864550 `default_staff_contract`** (original; item **`mkd_2`**, compiles, proof on the benches) [proved, whole
  disassembly, `pacotes/0x864550`]: the default wage. `stdcall(person, club, flag, squad status, job, kind)`, `ret 0x18`
  (0x8665b4), returns the float in st0 loaded from an f32 ([esp+0x4c], 0x8665a9). ecx is overwritten on entry (`mov
  ecx, [0xae1050]`, 0x864550): the thiscall wrapper in `exe_calls.rs` passes an ecx that does not count. The argument
  slots are reused as scratch, always after their last read. The order:
  1. The person's contract (`get_contract` 0x864562) goes to [0xae1080] (0x864569); the current club = the contract's,
     except type 5 (0x864570..0x864591).
  2. The level: with a club, reputation / 500 + 1 in a byte (0x8645d2..0x8645eb); with a current club and professional
     status 2, minus one, within 1..4 (0x8645fe..0x864613). Without a club, the world reputation of the player sheet
     (+0xd) or of the non-player sheet (+0xc) / 500, at least 1 (0x86461e..0x86465b).
  3. The job: `job_for_club` (+0x3d) when there is a current club; else `job_for_nation` (+0x28) with a national team
     (+0x24); else +0x3d; else 11 for a player; else +0x28; else 9 (0x864669..0x8646a2). Jump table 0x8665b8 on job − 5.
  4. Staff (jobs 5..10): the level rises to the current club's or the national team's (non-player, national job ≠ 11)
     (0x8646ba..0x864714); base = the non-player current reputation (+0xa)³ × k1 or, without a non-player sheet,
     (id / staff count × 4 + 1) × k2 × 0.0001; wage = level × base × 0.25 (0x864718..0x864761). By job: 5 (k1 2.5e-8
     0x96ff38, k2 25000): at least 750, above 25000 → **150000** (0x864767..0x8649f0); 6 and 7 (1e-8, 10000): at least
     500; 8 (5e-9, 5000): 275..5000; 9 (1e-9, 1000): 250..1000; 10: 200..1000 (0x864cd2..0x864ce3). With a current club
     other than the club: jobs 6, 7 and 8 give 150000 when the current club is more than three levels above (reputation
     / 500 + 4 > level) — for 6 also when the asked job is neither 11 nor 6; without that, 6 below 2500 × 1.35
     (0x86485b..0x8648c3, 0x86498e..0x8649ba, 0x864ade..0x864b2e); 6 and 7 otherwise: above 25000 → 150000 unless the
     asked job is 11 or 6 (0x8649c7..0x8649f0). Job 14 and the others: 1000 (0x864ce3). **150000 is the "not
     available" marker**, not a wage.
  5. Players (jobs 11, 12, 13, 15): without a sheet, the "No player data" message goes to the text 0xde1f64 and the
     wage is `rand() × 4 / 32768 × 550` (0, 550, 1100 or 1650; 0x865699..0x8656e1). With a sheet:
     - status −1 with a club: 1 (current reputation > the club's, 24..33 years), 2 (> 0.75 × the club's, 21+), 4 (> 0.5
       × the club's and 17+, or 25+), 5 (potential × 50 > 0.75 × the club's), 6 (0x864d40..0x864e00); the low byte of
       the argument is rewritten and goes whole to `get_max_wage`;
     - the base: kind 3 85000; kind 1 92500 (loyalty < 15 and pressure > 15) or 75000; kind 2 75000 (23..31 years) or
       82500; asked job 13 or 15 at 36+ 10000; status 5/6: 9350 (current reputation < 2500), 11000 (< 6250); attacker
       ≥ 15: 82500 / 70000 (reputation > 6750, < 31 years, loyalty < 15 / ≥ 15) or 55000; defensive, central or
       attacking midfielder ≥ 15: 67750 / 60000 (> 7250, < 31, loyalty < 14) or 44000; goalkeeper 20: 37500 (world
       > 7250, loyalty < 3, < 34) or 22000; sweeper and defender < 15: 27500; else 50000 (> 7500, loyalty < 6, < 33) or
       27500 (0x864e04..0x864fc6). **The kind 0x14244 of the third test (0x8651bc) never holds: a defect, the 6500
       floor does not exist;**
     - the young (19..31) not tied to the club or the club's nation (club reputation < 5750 or league < 12) with world
       > (loyalty + 230) × 25 − ambition × 10 rises to 75000 / 92500 / 105000 (0x864fca..0x8650c9);
     - FIFA rules ([0xae233c] = 1), above 11000, club of a major nation (0x549e50): × 1.15 / 1.125 / 1.1 (current
       reputation > 6750, by loyalty 1 / < 6 / other) or × 1.025 (> 4250) (0x8650cd..0x865141), not rounded to f32;
     - m = the highest reputation (current, home, world); world < 1750 and m > 3250: m = 3250 − ftol((m − 3250) ×
       −0.2); kind 1 at least 7250, kind 2 at least 6750; base 75000/92500/105000 at least 7250 (0x865145..0x865201);
     - u = (current + home + world + 4m + 50 × (ability + potential)) / 9 + ftol(id / count × 8 − 4); a foreigner
       (another region, `actual_region` +0x76, than the club's and the current club's): at least 1500 or × 1.05; at
       least 1 (0x865206..0x8652c3);
     - wage = base × u² × 1e-8, at least 175 (compared before rounding, 0x8652c8..0x8652f8); × 0.75 or × 0.5 by the
       `state_of_development` (+0x7e) of the current club's nation (or 1 with world > 4250, or the person's, or 3)
       (0x865300..0x865345); × world × 0.0004 (club < 2500) or world / (level × 500), within 0.85..1.15
       (0x865349..0x8653b7);
     - level caps: < 10 → 2500 × level; < 15 → 2750 × level; else 10% above 5250 × level (England, Spain, Italy with
       PLC +0x82), 7500 × level (without PLC) or 3250 × level (others) (0x8653c1..0x865513);
     - position cap: 20% above 150000 (attacker 20), 125000 (midfielders 20), 100000 (defender 20) or 80000; × 1.1 with
       FIFA at a major nation and current > 7250 (0x865515..0x8655ae);
     - without `club_job` (+0x39), except the old (33+) international with work rate + determination ≤ 30, id % 15 ≠ 0,
       at a club ≤ 4500: × (1.375 − d × 0.0125), at least 0.2, d = days since `date_joined_club` (+0x3e, at most 250),
       + 60 in season (0x547770), 20% above 30 (0x8655b2..0x865690).
  6. With a club (0x864ceb; without one it returns right away): for a sheet holder, r = (club reputation × 0.00017 /
     0.00012 / 0.0001)⁶ (pow 0x945ff0), at most 1; by status (table 0x8665e4) the wage goes half-way up to 3 ×
     reputation × r × (1: 0.6 / 1.5 / 1.25; 2: 0.3 / 1 / 0.6; 3: 0.25 and 0.6 / 0.3; 4: 0.1 / 0.3 / 0.15; 5: 0.05; 6:
     0.02, for < 20 years / 24+ at a major nation / others) (0x864cf7..0x86598a); with the contract grievance 0x10
     (+0x45): × 0.67 / 0.67 / 0.85 / 2.5 / 3.5 / 7.5 (table 0x8665fc, 0x86598c..0x8659ec).
  7. The club's nation, when the person works there (`club_job` of the same nation, and is from there or loyalty > 3
     or ambition < 15) or is from there without a club (0x8659f0..0x865a47): Japan at least level² × 10 and 25% above
     7500; England, Italy, Spain nothing; USA × 0.75, MLS 15% above 250, natives 20% above 1250 (32+ or < 24 with world
     < 3750) or 2750, foreigners 25% above 3250; Korea, Scotland, Germany/France/Holland, Sweden, Denmark/Norway,
     Belgium, Portugal/Croatia/Poland, Brazil/Argentina × the factor (club reputation / 50 / `league_standard`) × 0.1 × k
     (k = 0.25 / 0.25 / 0.75 / 0.25 / 0.5 / 0.45 / 0.5 / 0.45), at most 1, when reputation / 50 > league × 10, and
     each one's caps; Wales/Finland/Ireland/Northern Ireland/Australia only caps (200 / 100 × 0.15); the others × 0.3
     and 25% above 15000 (0x865a4d..0x86635b).
  8. Players: at most the wage of the contract with the current club of status 7 (`get_contract_for_club`,
     0x866377..); with `flag`, or without a current club and current < 2250, at most `get_max_wage` (without finances
     it returns the value as it is, without the minimums) (0x8663af..0x866414); without `flag` and with a current club,
     the ambitious (> 10), disloyal (< 6), famous (> 6750) attacker/midfielder/attacking midfielder 20 aged 21+: × 1.15/
     1.1 (attacking midfielder) or, if wage × 1.25 < maximum, × 1.1/1.05 (midfielder) or × 1.2/1.15 (0x866428..0x8664ff).
  9. Minimum: club of status 1: 75 under 20 years, else 100; others: 75 at a club > 2250, else 50 (0x866503..0x8665a9).
- **0x862720 `staff_contract_calc_unset_default_wage`** (original; item **`mkd_6`**, ported) [proved, disassembly;
  Ghidra's pseudo-C loses the FIFA adjustment (0x86315d..0x8631d9) and says `return 0` where the routine returns the
  value (0x864320)]: the default wage of the initial shortlists and of the regens.
  Convention: stdcall(person, club, check, status, job, floor), `ret 0x18` (0x8644e7), float in st0 (the f32 at
  [esp+0x14]); the first instruction overwrites ecx (both callers, thiscall, pass the contract in it and it is not
  read). Callers (`refs.py`): `set_default_wage` 0x8626f6 (person, `club_job`, 1, −1, 11, 0) and
  `create_initial_shortlists` 0x83a2c3 (person, club, 1, the high nibble of the contract's +0x4f with the rest of edx,
  11, 0), the latter for the contracts with wage −1 (0x83a2a8). Same in cm0102-1080.exe (0x1e30 bytes).
  Constants (read from the exe): 750 = 0x96ff30, 25000 = 0x96ff34 (f32); 500 = 0x96d1a8, 10000 = 0x96d1b0, 275 =
  0x96ff20, 5000 = 0x968db0, 250 = 0x96d210, 1000 = 0x96d2dc, 200 = 0x96d834, 2500 = 0x968dac (f32); 2.5e-8 =
  0x96ff38, 1e-8 = 0x968eb8, 5e-9 = 0x96ff28, 1e-9 = 0x96ff18, 0.0001 = 0x96a5c0, 1.35 = 0x969588 (f64); the bases
  50000 / 55000 / 65000 = 0x96ff04 / 0x96ff08 / 0x96ff0c, 40000 / 45000 / 52500 = 0x96fef0 / 0x96fef4 / 0x96fef8,
  20000 = 0x96ff00, 30000 = 0x96fefc, 8500 = 0x96d4e0, 11000 = 0x96feec, 175 = 0x96fee8, 550 = 0x96fed8, 1.375 =
  0x96fedc, 0.0125 = 0x96fee0, 0.0004 = 0x96fee4, 0.2 = 0x96b028 (f32); 0.00017 = 0x96fed0, 0.00012 = 0x96ff10, 6 =
  0x969000, 1.125 = 0x96a5a8, 2750 = 0x96fec8, 3250 = 0x96fec0, 1500 = 0x96feb8, 350 = 0x96f870 (f64). The immediates
  150000 (0x48127c00), 85000 (0x47a60400), 75000 (0x47927c00), 65000 (0x477de800), 50000 (0x47435000), 40000
  (0x471c4000), 75 (0x42960000), 50 (0x42480000) and 100 (0x42c80000) are not in `.rdata`.
  - **Callers:** **[proved]**, with the arguments: both pass check = 1 and job = 11; 0x83a1b0 only for contracts with
    wage −1, with the contract's status (proof: 0x8626e0..0x8626f6, 0x83a2a8..0x83a2c3)
  - **What it reads (corrected):** **corrected [proved]**: it reads neither pressure nor professionalism; it reads
    loyalty (base and market), ambition (market and bonus), determination and work rate (the veteran), international
    caps, age, the three reputations, ability and potential; it uses 0x549e50, 0x547770 (with today's date) and
    0x5968d0 with the status dword
  - **The contract and the current club:** **[proved]**: `get_contract` (stored in [0xae1080]); the contract's club,
    none for a loan (type 5) (proof: 0x862720..0x86276d)
  - **The level:** **[proved]**: with a club, reputation / 500 + 1 in byte arithmetic; with a current club and a club
    of professional status 2, one less, within 1..4; without a club, the world reputation of the player (or staff)
    sheet / 500, at least 1 (proof: 0x86279c..0x862838)
  - **The job computed:** **[proved]**: the club job (with a current club), else the national job, else the club job,
    else 11 with a player sheet, else the national job or 9; the `job` argument only matters for jobs 6 and 8 (6 or 11
    offered) and for coaches of 36 or more (13, 15) (proof: 0x86283c..0x86288e, table 0x8644ec)
  - **Staff (5 to 10):** **[proved]**: the level rises to the current club's reputation / 500 + 1 (or the national
    side's, outside job 11); wage = level × (current reputation³ × k with a staff sheet, else (id / number of people ×
    4 + 1) × m × 0.0001) × 0.25; manager: k 2.5e-8, m 25000, at least 750, above 25000 it becomes 150000; assistant
    and reserve manager: 1e-8 / 10000, at least 500; coach: 5e-9 / 5000, 275..5000; scout: 1e-9 / 1000, 250..1000;
    physio: 200..1000. At another club: the assistant is worth 150000 (except job 6/11 offered with the level reaching
    the current reputation / 500 + 4: below 2500, × 1.35); reserve manager and coach are worth 150000 when the current
    club's reputation / 500 + 4 is above the level; otherwise above 25000 becomes 150000 (job offered other than 6 and
    11) (proof: 0x862895..0x862f06)
  - **Player (11, 12, 13, 15) without a sheet:** **[proved]**: writes "No player data..." into 0xde1f64 (sprintf with
    dwords +4, +8 and the id) and returns `rand()` × 4 / 32768 × 550 (proof: 0x863660..0x8636b0)
  - **Player: status −1:** **[proved]**: with a club: 1 (current reputation above the club's, 24..33 years), 2 (above
    0.75 × the club's, 21+), 4 (above 0.5 × and 17+, or 25+), 5 (potential × 50 above 0.75 × the club's), 6; the
    argument's byte is rewritten and goes whole to `get_max_wage` (proof: 0x862f5b..0x86302f)
  - **Player: the base:** **[proved]**: job offered 13/15 at 36+: 10000; status 5/6: 8500 (current reputation < 2500)
    or 10000 (< 6250); attacker ≥ 15: 65000 / 55000 (reputation > 6750, loyalty < 15 / ≥ 15) or 50000; defensive,
    central or attacking midfielder ≥ 15: 52500 / 45000 (> 7250, loyalty < 14) or 40000; goalkeeper 20: 25000 (world >
    7750, loyalty < 3) or 20000; sweeper < 15 and defender 0: 25000; else 30000 (world > 7500, loyalty < 6) or 25000.
    With the FIFA rules ([0xae233c] = 1), a base above 11000 and a club of a major nation: × 1.15 / 1.125 / 1.1
    (reputation > 6750, loyalty 1 / < 6 / other) or × 1.025 (> 4250), not rounded to f32 (proof: 0x86303d..0x8631d9)
  - **Player: the score and the wage:** **[proved]**: m = the highest reputation (current, home, world), floor 7250
    (floor 1) or 6750 (floor 2); s = (current + home + world + 4m + 50 × (ability + potential)) / 9 + ftol(id / count
    × 8 − 4); a foreigner (region +0x76 other than the club's and the current club's): at least 1500, otherwise ×
    1.05; at least 1; wage = base × s² × 1e-8, at least 175 (compared before rounding); × 0.75 or × 0.5 by the
    development of the current club's (or the person's) nation; × current reputation × 0.0004 (club < 2500) or /
    (level × 500), within 0.85..1.15 (proof: 0x8631d9..0x8633b7)
  - **Player: the caps:** **[proved]**: level < 10: 2500 × level; < 15: 2750 × level; else 3250 × level, or 10% above
    (soft) 3250 × level (England, Spain, Italy with PLC) or 5000 × level (without PLC); position: 20% above 85000
    (attacker 20), 75000 (central or attacking midfielder 20), 65000 (defensive midfielder 20), 50000 (defender 20) or
    40000, × 1.1 with FIFA, a major nation and current > 7250 (proof: 0x8633bb..0x8635a9)
  - **Player without a club:** **[proved]**: without `club_job`, except the veteran (33+, international, work rate +
    determination ≤ 30, id % 15 ≠ 0, club ≤ 4500): × (1.375 − d × 0.0125), at least 0.2, d = days since +0x3e (no
    cap), + 60 in season (0x547770) (proof: 0x8635ad..0x86365b)
  - **With a club: the offer by status:** **[proved]**: with a player sheet, r = (club reputation × 0.00017 (amateur)
    / 0.00012 (> 2250) / 0.0001)⁶ (pow 0x945ff0), at most 1; by status (table 0x864518) the wage rises to the average
    with 3 × reputation × r × (1: 0.6 / 1.25 / 1.5; 2: 0.3 / 0.6 / 1; 3: 0.25 and then 0.3 / 0.6; 4: 0.1 / 0.15 / 0.3;
    5: 0.05; 6: 0.02, for < 20 years / < 24 or outside a major nation / others); with the contract's grievance 0x10: ×
    0.67 / 0.67 / 0.85 / 2.5 / 3.5 / 7.5 (table 0x864530) (proof: 0x862f12..0x8639b7)
  - **The market of the club's nation:** **[proved]**, the same as step 7 of 0x864550 in the map: when the person
    works there (`club_job` of the same nation and is from there or loyalty > 3 or ambition < 15) or is from there
    without a club; Japan floor level² × 10 and 25% above 7500; England, Italy, Spain nothing; USA × 0.75, MLS 15%
    above 250, natives 20% above 1250 (32+, or < 24 with world < 3750) or 2750, foreigners 25% above 3250; Korea,
    Scotland, Germany/France/Holland, Sweden, Denmark/Norway/Poland, Belgium, Portugal/Croatia, Brazil/Argentina × the
    league-standard factor (reputation / 50 / standard × 0.1 × k, at most 1, when reputation / 50 > standard × 10) and
    each one's caps; Wales, Finland, Ireland, Northern Ireland, Australia only caps; the others × 0.3 and 25% above
    15000 (compared before rounding) (proof: 0x8639bb..0x864291)
  - **The final limits:** **[proved]**: player: at most the wage of the current contract of status 7; when checking,
    or without a current club with reputation < 2250: at most `get_max_wage` (without finances it returns the value as
    it is, without the floors); without checking and with a current club, the ambitious (> 10), disloyal (< 6), famous
    (> 6750) attacker/midfielder/attacking midfielder 20 aged 21+: × 1.1 (attacking midfielder) / 1.05 (midfielder) /
    1.15 if wage × 1.25 < cap; floors: club of status 1: 75 under 20 years, else 100; others: 75 at a club > 2250,
    else 50; without a club: the value without floors (proof: 0x864295..0x8644e7)
  - **Differences from 0x864550:** **[proved]**, and the differences from 0x864550 (map, item mkd_2): lower bases
    (without kinds 1..3 of 75000..105000, without the young rising, without the world < 1750 cut, attacker
    50000..65000 instead of 55000..82500); level caps 3250 / 5000 instead of 5250 / 7500; position caps 40000..85000
    instead of 80000..150000; the ratio by the **current** reputation (0x864550 uses the world one); the development
    without the "1 with world > 4250"; the free agent without the day cap nor the 20% above 30; staff with the 150000
    (proof: comparison with the 0x864550 text in the map)
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

#### 2.4 Renewals and contract end (0x4c7ac0, original) [proved, disassembly; not ported]

`thiscall(contract manager, &date, slice (byte), flag (byte))`, `ret 0xc` (0x4c9e2e). Callers: `game_main_day_loop`
(0x5cd406: today, the current slice [0xae2c88], 1) at **every slice** of the day, and `game_run_to_start_date`
(0x5cc9ef). Constant: 0.004 = 0x968d60 (progress bar steps). Order [proved]:

- **The slices:** it runs at every slice; **slice 0** only recomputes the fixed dates (today + 7, 30, 60, 91, 121,
  152, 182, 365, 550, 1095 in 0xae1048, 0xae1030, 0xae1058, 0xae1010, 0xae1060, 0xae1040, 0xae1090, 0xae1068,
  0xae1008, 0xae1020); **slice 2** does the people pass (only with flag = 1); every slice ends with the decision dates
  (0x4cd9c0 with the date and the slice) (0x4c7b2e..0x4c7c74, 0x4c7c79, 0x4c7de0, 0x4c9e0e..0x4c9e15).
- **Contract talks** (slice 2): each talks record (21 bytes, +0x28/+0x2c) expires (25 − the manager's loyalty) × 2
  days after the date at +0xc; an expired one leaves (swapped with the last) and the array is re-sorted (qsort
  0x4dc970). It is the first use of loyalty (0x4c7c81..0x4c7d7b).
- **"Updating game data" bar**: on 31/12 (0x549f70 day 31, month 11), every 250 people (0x4c7b07..0x4c7b29,
  0x4c7d7e..0x4c7e2f).
- **Before the full load** ([0xdd711c] = 0, `G_GAME_DATA_LOADED_FLAG`): a contract ending today only becomes type 1
  (+0x35 = (x & 0xc1) | 1) (0x4c7e34..0x4c7ec0).
- **Player value recomputed** (0x4c7f65..0x4c8070): human club and a player job: with bit 4 of +0x4e and a sheet,
  value recomputed (0x4d0c60 flag 1); AI club and a player job: **on odd days of the year**, bit 4 set and value
  recomputed (flag 0); staff: every day; in the first two, capped by the active release fee (0x867b50). Without a
  contract or club: value 0.
- **Loan end** (0x4c8077..0x4c8114): the loan contract (index +4 of the 79-byte record) ending today: without a club,
  only the index becomes −1; otherwise news 6 (0x4da590, outside type 7) and `end_loan` 0x4d07e0.
- **Renewal before the transfer rules change** (0x4c8152..0x4c847a): before [0xae2330]
  (`G_TRANSFER_RULE_CHANGE_DATE`) and with a sheet, when that date falls on today + 7/30/42/60/70/91 (recomputed **per
  person**, in 0xae1070..0xae1088), the contract started over 180 days ago and ends before today + 1095: a club that
  does not dislike him and whose manager does not (0x5477d0, 0x547840), AI, age < 32 → by status: 1 offers (factor
  1.0); 2 and 3 with current reputation > 6750, or age < 24 and potential > 100; 5 with potential > 100; 4 never.
- **FIFA protected period** (0x868210; 0x4c847f..0x4c85fe): a type 2 contract, no loan, ending **after** today + 182,
  outside the protected period (0x868210 = 0 and the output ≠ 0), not retiring (0x7a7eb0), job 11: the AI (age < 32)
  renews by the same statuses (factor 1.0); human club: news 7 (0x4d6790; Bosman by the age at today + 182 ≥ 24 and
  0x8d0a70).
- **Type 1 (expired contract)** (0x4c8603..0x4c883b): no loan, 31 days past the end, wanting to leave (0x866f60) or
  leaving on a Bosman (+0x49), with no offers, exchanges or queue: news 1 to the human club; jobs 12, 13 (AI) and 15
  (AI) go back to 5, 6 and 8 and leave the squad; the others go to the **free transfer queue** (0x8c16b0 with 1, −1).
- **The band** (`get_expiry_band` 0x867d30) against +7..+182 (0x4c8876..0x4c88a5, 0x4c8a79..0x4c8abb): the odd bands
  (the exact day of 182, 152, 121, 91, 60, 30 and 7 days) put the player on the clubs' lists (0x842c00 with 1); band
  14 only with `random(20)` = 0; someone already leaving on a Bosman too, on the odd ones.
- **Leaving on a Bosman** (+0x49 = 1; 0x4c88b5..0x4c8a74): type 2, contract started over 93 days ago, inside the bands
  window (0x867d30 ≠ 0 and ≠ 14), not retiring, job 11, the current contract not a loan, a club with a nation; current
  reputation > 3750, world > 1750, **loyalty < 15**, age 19..30, the nation allows it (0x544670), and not (club > 4750
  and age < 23); and wanting to leave (0x866f60) or bit 8 of +0x4e. Then +0x49 = 1, bit 8 cleared, the clubs' lists
  (0x842c00) and news 0x4d6d70. It is the second use of loyalty.
- **The AI renewal offer** (`offer_contract` 0x4cda70, ×9; person, club 0, out 0, slot −1, factor, mode 2), only for
  an AI club and a player not retiring; the factor (f64) is a pushed immediate: with **550 days** left (0xae1008),
  status 1..3 and on his own club's list (0x849ce0): 1.0; **365 days** (0xae1068, or on the game's first day with an
  end before that): news 6 and, status 1..3 or 5 on the list: 1.0; bands **3 and 5** (152/121 days): status 1..3 on
  the list 1.2, off it 1.0; 4 and 5 → 1.0; band **1** (182): 1.1 on the list / 1.0; band **7** (91): 1.3 on the list
  / 1.0, 1.1 for 3..5; bands **9, 11** (60/30): 1.4 (1..2) and 1.2 (3..5) on the list, off it 1.0; band **13** (7
  days): the same, with news to the human club. When 0x4cda70 returns 0 on the 365-day path and on bands 1 and 7, a
  local type 2 offer becomes the refusal news (0x4d49a0 with 1); on bands 1 and 7 it also sets bit 0x80 of the 79-byte
  record (0x4c910e, 0x4c91e3, 0x4c93d0, 0x4c94a5). Proof: 0x4c8ac0..0x4c9a8b, 0x4c9b5f..0x4c9b71; factors 1.1 =
  0x3ff19999_9999999a, 1.2 = 0x3ff33333_33333333, 1.3 = 0x3ff4cccc_cccccccd, 1.4 = 0x3ff66666_66666666.
- **Listed without offers** (0x4c9a90..0x4c9b10): a player (job 11) transfer-listed (0x5444a0) with no offers,
  exchanges or queue goes into the free transfer queue.
- **The contract end on the day** (0x4c9b76..0x4c9d21): a type 5 (loan) ending today: news 5, stats archived
  (0x7c5fe0), the contract erased (both indexes −1), the wage bill (0x598370), out of the selections (0x896c60 ×2) and
  of the club (0x53c190). Any other type: becomes type 1; leaving on a Bosman, no loan and no offers: news 2 and
  **released at once** (`free_transfer_staff` 0x8c1af0).
- **Decision dates** (0x4c9d40..0x4c9e07): on days 36 and 200 of the year (slice 2), the processed ones leave the
  array; on every call, if new dates were added (+0x1c), the pending part is re-sorted (qsort with 0x4dc910) and +0x1c
  is reset.

Attributes: only `loyalty` (+0x59, ×2): talks (0x4c7cc3) and Bosman (0x4c8a01).

**Why it did not become an item:** 9,376 bytes (2,700 instructions, 10 jump tables), ~45 routines called (the progress
bar included), 9 renewal paths and the contract end; too big to prove with one comparison (it cannot run twice: news,
draws, queues). Proposal: port it in pieces through callers already in Rust (the player value, the loan end, the Bosman
rule and the renewals by band as helpers), with the loop last.

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

Who becomes free: the contract end in 0x4c7ac0 [proved, §2.4]: on the end day, only those leaving on a Bosman (+0x49,
the nation's rule 0x8d0a70) are released at once (`free_transfer_staff` 0x8c1af0); the others become type 1 and, 31
days later, go to the queue only if they want to leave (0x866f60) or leave on a Bosman; and those released through the
queue
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
| Youngsters (+0x22), manager | 0x4d3ccb | `contract_manager_calc_asking_price` 0x4d1600 (item `mka_2`) | no | in the youngster floor (section A §3 step 15: `flag` set, age < 24, seen > 110): with a manager with a non-player sheet, at an AI club (`is_human_club(seller, 1, 1)` = 0), si += youngsters − 10 (16 bits; `movsx cx, [eax+0x22]`, `lea esi, [esi+ecx−0xa]`); then, a player who joined after the manager and si < potential: + half the difference; si becomes the floor Y = ftol((si² − 14400) × 250 × 0.6153846 + bonus), with step 15's adjustments, and the price is at least Y (reason 0x2000) [proved: 0x4d3c9c..0x4d3cd0, 0x4d3cd4..0x4d3d15, 0x4d3d17..0x4d402a] |
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
5. 0x4ce4e0: the spare staff record is accepted up to **the count itself** (`cmp eax, [0xae2368]; jg` is the error, it
   should be `jge`) [proved: 0x4cf1b5].
6. 0x4ce4e0: with several offers from **the same club** and the chosen one without a transfer,
   `complete_accepted_contract_offer` runs **once per offer** (the loop calls it again for each) [proved:
   0x4cf0c9..0x4cf15c].
7. 0x4ce4e0: loans and trials are chosen before the refusals: a later refusal (work permit, no spare record) zeroes the
   stored score but does not undo the choice [proved: 0x4ce75c..0x4ce764 against 0x4ceb68].
8. 0x4ce4e0: the "renewal" marker and the will are stack variables that carry over between offers: the marker's first
   value is the numerator of k (0x4ce5bc), and an offer with a negative desired value would use a previous offer's
   [proved]; no effect in practice (the value is never negative) [inferred].
9. 0x4c7ac0: the six rule-change dates (0xae1070, 0xae1018, 0xae1038, 0xae1028, 0xae1078, 0xae1088) are recomputed
   **for every player** in the loop, not once a day (same result, wasted work) [proved: 0x4c81a3..0x4c827c].
10. 0x4c7ac0: the AI player's value is recomputed only on **odd** days of the year (`movsx eax, word [date]; and
    0x80000001`), the human one every day [proved: 0x4c7ffe..0x4c8010]; the intent [inferred].
11. 0x864550: `cmp edx, 0x14244` (0x8651bf) compares the kind byte with a number that does not fit in a byte: the 6500
    floor never applies [proved].
12. 0x864550: without a club, a player with a nation reads the club through a null pointer at 0x865270 (and at level
    ≥ 15 at 0x86541b); no caller passes a null club for a player [proved].
13. 0x85fc50: 0x861d64..0x861d70 has `if (!slot && !*slot)` instead of `||`: with an offer without a slot (0x8c61e0
    returns 0) it reads address 0 before the error box; with a slot and `*slot == 0` it passes 0 to 0x8cf0e0 [proved].
14. 0x85fc50: [esp+0x48] (the reputation used for the clauses and the length) is only written with a sheet + a player
    job or with a non-player sheet (0x85ff46..0x85ff7c); a person with a sheet, a staff job and no non-player sheet
    reads stack garbage (which becomes 1 if it is 0) [proved].
15. 0x85fc50: the tests (100 − ambition − loyalty) × 125000 < 0 and (80 − ...) < 0 (0x86045f..0x8604df) never hold
    with attributes 1..20: kind 1/2 and status 1/2 of that branch never happen [proved].
16. 0x85fc50: `date_started` computed from the window is overwritten with today right after (0x861e14) [proved].
17. 0x862720: **150000 is a marker**, like the 250000 of 0x85fc50: the manager above 25000 and the assistant / coach
    of a stronger club are worth 150000, not a wage **[proved]** 0x862bea, 0x862a80, 0x862bb4, 0x862d33.
18. 0x862720: The defender test is `!= 0` (0x86310f): every player has a defender rating, so the "sweeper < 15 and
    defender 0" branch never holds and the base stays at 25000 / 30000 **[proved]**; the same result in practice.
19. 0x862720: Without a club and with level 15 or more (world ≥ 7500), a player reads +0x53 of the null club
    (0x863403): an access fault **[proved]**; both callers pass a club (a regen has one) **[inferred]**.
20. 0x862720: Without a player sheet in a player job, the sprintf uses `%s` with the person's dwords +4 and +8
    (0x863660) **[proved]**; the wage comes from the CRT `rand()`, outside the game's generator.
21. 0x844780: **Three fixed names**: Dario Gradi, Pat Nevin and Kevin Keegan never leave a club job (will 0); the
    comparison is by the formatted name, so anyone with that name too **[proved]** 0x8447fb..0x8448f5.
22. 0x844780: **The last 16 person records** (id ≥ [0xae2364] − 16) only accept the manager's job **[proved]**
    0x84491a; [inferred] they are the regens' spare records.
23. 0x844780: **Job 0 only at his own club**: the job offered 0 gives 0 at another club (0x84532a), but at his own
    club the routine skips the job tables and computes the will **[proved]** 0x8451ca.
24. 0x844780: **The manager's asymmetry**: a new club without a manager weighs 0 on the scale; the current one without
    a manager weighs 0.75 × the division **[proved]** 0x8455f4 against 0x845ded; the wealth bands, the national side
    factor (1.45 against 1.35) and the Intertoto rule (only the current club) differ too **[proved]**.
25. 0x844780: **No finances, 0**: when checking with a will ≥ 1.75, a club without a finance record gives will 0, even
    a positive one **[proved]** 0x846671..0x846673.
26. 0x85fc50: in the no-promotion, minimum-fee and no-play clause fees, above 2,500,000 the extra becomes ftol(e ×
    0.21): the original subtracts and adds the same 2,500,000 (0x861657 `fild e`; `fmul 0.21` [0x96d398]; `fsub
    2500000` [0x96a650]; `fadd 2500000`; 0x861657..0x861667), which looks like a slip for (e − 2,500,000) × 0.21 +
    2,500,000; the fee drops from ~2.5M to ~525 thousand [proved; the intent, inferred].

### 6. Extension points (the smallest hook that changes the policy)

1. **0x846920 (Rust)**: a factor on the return of `staff_interest_in_joining_club` changes the will on every path
   (bids, loans, renewal, AI lists). For groups (age, reputation, nation), multiply the return; the callers'
   thresholds (2.75, 3.15, 3.5) stay.
2. **0x85fc50 (mkd_3; original in the exe)**: a detour that calls the original and adjusts the wage (+0xc) and the value (+8) on the way
   out changes the wage demand without porting the 10,896 bytes [proved: +0xc is written once, at the end (0x862493),
   and +8 before (0x85ffe2..0x86016b); the return does not reread +0xc]. Note: the return (the contract value) is
   computed with the wage from before the adjustment; a caller that uses the return instead of +0xc does not see the
   adjustment (check caller by caller before detouring).
3. **0x5968d0 (Rust)**: the club's wage cap (and where the chairman weighs in through Business/Resources).
4. **0x866830 (mkd_1)** and **0x866620 (Rust)**: the satisfaction (50/80) and the ratio of the renewal decision.
5. **0x4cda70 (Rust)**: the AI offer (factor, length by age, cap × 1.4).
6. **0x83bf40 (Rust)**: the target reputation, where the chairman's patience and resources come in.
7. **0x6e86d0 (Rust)**: how much the manager sees (Judgement/JudgingPotential).
8. **0x4ce4e0 (mkd_4, Rust once integrated)**: the 2.75 threshold, the 0.9/0.8/0.7 cuts, the renewal floor 3.15 and
   the personality k sit in one function (`choose_contract_offer`): the place to change the will to renew or to change
   club by groups (age, reputation), without touching `staff_interest_in_joining_club`.
9. **0x4cda70 in the AI renewals**: the factor (1.0..1.4) is an immediate pushed at each of the 9 calls in 0x4c7ac0; a
   detour on `contract_manager_offer_contract` (Rust) adjusting the factor by player changes all of them without
   porting 0x4c7ac0. The Bosman rule (loyalty < 15, age 19..30, reputations 3750/1750) sits in one stretch
   (0x4c89ea..0x4c8a4a).
10. **0x864550 (mkd_2)**: the default wage is a float returned in st0: a detour that calls the original and multiplies
    the result changes all callers (the demand, the will gate and the wage bill) in one place. With the Rust copy of
    mkd_2, the fine-tuning points are the base by position (step 5), u² and the caps by nation (step 7).
11. **0x862720 (mkd_6, Rust once integrated)**: the wage of the regens (`set_default_wage`) and of the initial
    shortlists; the bases by position, the s² score and the market by nation are in one function. For the Tweaks, a
    detour on the way out that scales the value for the regens only changes their wage without touching the desired
    contract (0x85fc50).
12. **0x844780 (mkd_7, Rust once integrated)**: the whole staff movement (managers, assistants, scouts) in one
    function: the base 1.5 / 2.25, the job tables (× 0.01 to × 10), the minimum time at the club ((loyalty + 8) × 15
    days), the board's confidence (8000) and the wage gate (1.75). For the Tweaks: changing how much managers change
    club by groups (reputation, age) without touching the players' will (0x846920).

### 7. Queue items

- **mkd_1** — `STAFF_CONTRACT::calc_contract_value` (0x866830): `levas/fila/mkd_1/`. Compiles with the current engine
  (own copy, no warnings from the item). **Not proved in the game**: the laboratory lock was busy and the memory guard
  refused (2.3 GB free, minimum 4). See `PRONTO.txt`.
- **mkd_4** — `CONTRACT_MANAGER::resolve_contract_offers` (0x4ce4e0, §2.2): `levas/fila/mkd_4/`. Compiled; the proof
  is left to the main session.
- **mkd_2** — `STAFF_CONTRACT::default_staff_contract` (0x864550, §2.3): `levas/fila/mkd_2/`. Compiles; the proof on
  the benches is left to the main session.
- **mkd_3** — `STAFF_CONTRACT::set_to_desired_contract` (0x85fc50, §2.3): `levas/fila/mkd_3/`. Ported from the whole
  disassembly; the proof on the benches is left to the main session.
- **mkd_6** — `staff_contract_calc_unset_default_wage` (0x862720, §2.3): `levas/fila/mkd_6/`. Ported; the proof on
  the benches is left to the main session.
- **mkd_7** — `staff_interest_in_job_at_club` (0x844780, §2.1b): `levas/fila/mkd_7/`. Ported; the proof on the
  benches is left to the main session.

Not ported because of size (for another round): only 0x4c7ac0 (9,376 bytes), proved in the disassembly (§2.4),
with the proposal to port it in pieces. No market routine is left only mapped. Screens only (no effect on the automatic career): 0x4d4660, 0x4d4830,
0x4d44e0, 0x867ad0, 0x85f7a0.
