*Portuguese version (the original): [fechamento_inferidos.md](fechamento_inferidos.md).*

# Closing the market map: the [inferred] marks outside the other readers' routines (11/11/2026)

Request: [BRIEF_FECHAMENTO.en.md](BRIEF_FECHAMENTO.en.md), steps 1 and 4 only (no porting). Outside this closing (other
readers): 0x4d1600, 0x85fc50, 0x864550, 0x4ce4e0, 0x4c7ac0, 0x862720, 0x844780. One entry per claim of
`docs/MAPA_MERCADO.md` (line numbers of the `.en.md`): the old text, the new text and the proof (instruction
addresses), or "still inferred: why". Disassembly of `jogo/cm0102.exe` by `engenharia-reversa/desmonta.py`; doubles
read from the exe. "Rust compared" = a routine already in Rust with 0 divergences in the comparison.

Where an entry cites the items `mkb_2` (0x83cb40) and `mkb_3` (0x83f3a0): those ports were made from the whole
disassembly (see their `PRONTO.txt`), and I checked the points cited here in the disassembly; the rest of the new text
is their header.

## Summary of what changed

- **The stuck loan exists, and the path is another one (no holidays):** a loan accepted by the owner (state 8) that the
  player turns down because he chose another offer is held; if that other transfer is withdrawn at the confirmation
  stage (state ≥ 0xb) within 10 days, the loan goes back to state 8 with its contract offer marked "does not count for
  the decision" (a type 6 contract). Nobody decides it again, `process_due_offer` has no branch for 8 and the weekly
  sweep skips 8: it is stuck for good, and the player stays "under offer" (C5).
- **Bit 4 of `contract::TRANSFER_STATUS` = automatic asking price** (the "Auto" button of the "Set Staff Transfer
  Status" screen); bit 8 = listed at the player's request (A4, B3).
- **Squad statuses (high nibble of +0x4f):** 0 Undefined, 1 Key, 2 First Team, 3 Rotation, 4 Backup, 5 Prospect, 6
  Youngster, 7 Not Needed (A4).
- **The `edi` of the youngsters filter (0x8400ae) is the person's id + the club's id**, fixed per pair: the same
  youngster is always known or always ignored by the same club (D1).
- **The future-transfer field +0x10 = 1 when the person leaves the club, 0 when he arrives** (A5).
- Corrections to 0x83cb40/0x83f3a0 (B4, B5): injury 150/30/400/300 days; staff sacked only when the list comes back from
  ACTIVE 2; the B club (not "without the partner") leaves at once; "whom the club knows" rewritten.
- The chairman enters 0x83cb40/0x83f3a0, but only through the calls (B8).
- 0x4dd0a1 is not in `save_to_disk`: it is the "Transfer Request" dialog (B2).

---

## Section A

### A1 (line 47) The AI's bids every day of the window
- **Old:** "The computer's bids run every day of the window for every computer club that buys **[inferred]** from the
  callers."
- **New:** The budget is computed for every bid built (`TRANSFER_OFFER::build_bid` 0x8e5d60, which calls 0x598800 at
  0x8e6330) and in each offer's restrictions check (0x8cfe2d). The AI's new bids come only from `make_club_bids`
  0x84a020 (single call at 0x5cdd6d, in the day loop), at the rhythm of section B §2 (every day, or only when
  (random(2) + id) % 7 = 0), **without looking at the window**: reason 9 (outside the window) of
  `can_club_bid_for_player` is accepted (offers outside the window are held by 0x8c4e10). **[proved]**
- **Proof:** refs of 0x598800 (37 sites) and of 0x8be300 `add_transfer_offer` (0x84a728 in 0x84a020; the others are
  reviews 0x8c0810/0x8c0f40, the loan 0x8bed60, 0x8bf060 and screens); 0x84a020 in Rust compared (cqe_3), its header
  says "the club can reach him (`can_club_bid_for_player`, or reason 9)"; the rhythm at 0x5cdc6e..0x5cdd6d.

### A2 (lines 108-111) Negative balance, non-player
- **Old:** "For a non-player, V × 0.65 (what looks like the intent) **[inferred]**."
- **New:** Without a player sheet, V × 0.65 **[proved]**: 0x59970b (+0x61 null) → 0x599759 `fild V` → 0x59975d `fmul
  [0x96a5d0]` (= 0.65, read from the exe) → `_ftol` 0x599763. With a sheet, the reputation ratio (0.15 [0x968f20] to
  0.85 [0x969520]) takes V's place (0x599712..0x599757) and is multiplied by the same 0.65. Still inferred only that the
  intent was V × ratio × 0.65 (intent cannot be proved from the code).

### A3 (line 116) Club without a league
- **Old:** "**[inferred]** Step 4 returns the budget with no reserve for a club without a league (background clubs with
  no competition)."
- **New:** Step 4 is proved: `club::DIVISION` (+0x57) null (0x598af3..0x598afc) or no object in `G_COMP_OBJECT_TABLE`
  (0x598b02..0x598b0d) jumps to the return 0x5997ec with step 1's value. The same test is what the shortlist uses for
  "league not loaded" (0x83cc68..0x83cc86). **Still inferred:** which clubs these are in a given career (the number
  depends on the loaded leagues; not counted).

### A4 (line 143) Bit 4 of TRANSFER_STATUS and the statuses 0x10/0x50
- **Old:** "or without bit 4 of `contract::TRANSFER_STATUS` (+0x4e) **[inferred: the manager set the price]**: the
  floor. By squad status (… & 0xf0): 0x10 and 0x50 the price; 0x20 and 0x30 the mean …" (and, in item mka_1, "0x10 and
  0x50 [inference: key players]").
- **New:** **Bit 4 = automatic asking price** **[proved]**. Without it the manager set the price by hand, and
  `staff::VALUE` (+0x52) is that price. The statuses: **0x10 Key and 0x50 Prospect** give the computed price; **0x20
  First Team and 0x30 Rotation**, the mean; 0 Undefined, 0x40 Backup, 0x60 Youngster and 0x70+ Not Needed, the floor.
  **[proved]**
- **Proof of bit 4:**
  - The "Set Staff Transfer Status" screen (0x8f6770, transfer_screens.cpp): control 0xa comes from bit 4
    (0x8f6923..0x8f6936: `and edx, 4; shr edx, 2`); the "Auto<…automatic asking price>" button (text at 0x8f6fe9) is
    event 5 of handler 0x8f7640, which toggles control 0xa and, when switching it on, puts `calc_player_value` 0x4d0c60
    in control 1 (the price) (0x8f772b..0x8f77bb); editing the price (event 0xb) switches control 0xa off
    (0x8f780d..0x8f781e); on confirmation control 0xa becomes bit 4 (0x8f80b0..0x8f80da).
  - A new contract is born with +0x4e = 4 (0x866f14, in 0x866ee0, called by 0x4cfb13/0x4cfccb/0x4cfef4).
  - With bit 4, the day (0x4c7f9e..0x4c7fbe) and 0x84ae17..0x84ae30 recompute `staff::VALUE` with 0x4d0c60; without it,
    they do not.
  - `set_player_squad_status` 0x8ce0e0 (screens only: 0x8efc88, 0x8f08b5, 0x8f0902, 0x8f0af6, 0x8f7fbc) writes the price
    at +0x52 (0x8ce1e9) and clears bit 4 when its 8th argument ≠ 0 and the club is not in the crisis state
    (0x8ce8af..0x8ce8bf).
- **Proof of the statuses:** 0x867f6d..0x867ffa (staff_contracts.cpp) reads `+0x4f >> 4` and jumps through the table
  0x868000 (`0x867f81 Undefined, 0x867f90 Key, 0x867f97 First Team, 0x867fa6 Rotation, 0x867fb5 Backup, 0x867fbc
  Prospect, 0x867fcb Youngster`; above 6, "Not Needed" at 0x867fda). Consequences for the map: 0x84af00's "status 6 or
  7" is Youngster/Not Needed; 0x83cb40 sets Youngster (6) under 20 and Not Needed (7) otherwise.

### A5 The future-transfer field +0x10 (the coordinator's request)
- **Old:** §1 step 3, "27-byte rows: person +8, sale +0x10, fee +0x14, stage +0x19" (in mka_1: "nonzero when the club
  sells the person [inference]").
- **New [proved]:** +0x10 = **1 when the person leaves the club asked about, 0 when he arrives**. In
  `get_club_future_transfer_list` 0x8ccb80, for each open offer of the club (0x8ccbf3..0x8ccc1b, those it made; plus the
  ones received, from 0x8c6100 at 0x8ccc4d; loans, `offer::OFFER_TYPE` +0x1f = 1, are skipped at 0x8ccc78), the
  target's row (0x8ccdb7..0x8cce4d): +8 = the target (`offer+0xc`); if the club is the buyer (`[club] = offer+8`,
  0x8ccdca..0x8ccdcf), +0xc = the target's current club (−1 none) and **+0x10 = 0** (0x8ccdda/0x8cce02); otherwise +0xc
  = the buyer and **+0x10 = 1** (0x8cce0b..0x8cce0e); +0x14 = the fee (`offer+0x1b`); +0x18 = 4 for a pre-contract
  without a fee (completion 3/4), 1 with an exchange, else 0; +0x19 = the stage through the table 0x8cd134/0x8cd114 (1
  to 6, 0 otherwise); +0x1a = `offer+0x2d`; +0 the date. The exchange players' rows (0x8cce89..0x8ccf33) are the
  reverse: for the buying club, +0x10 = 1 (it gives the exchange player away), otherwise 0; fee 0, +0x18 = 1. In
  0x598800 (mka_1), +0x10 = 0 adds the fee to the purchases; ≠ 0 takes the sale branch in stages 3 to 6.

## Section B

### B1 (line 296) The % 7
- **Old:** "[proved; effect: inferred, most likely meant "once a week"]".
- **New:** The test is proved (0x5cdc6e..0x5cdc7b: random(2) once a day, before the loop; 0x5cdd32..0x5cdd42 and
  0x5cdd58..: (r + id) % 7). The effect is proved for the shortlist: the shortlist's new bids and loan requests come
  only from 0x84a020 (0x842740 has a single caller, 0x84ab5a, inside 0x84a020; the AI's new bids, 0x84a728). Those
  clubs can still receive/answer offers and review their own. **Still inferred:** the intent ("once a week").

### B2 (line 318) 0x4dd0a1 in `save_to_disk`
- **Old:** "One in `CONTRACT_MANAGER::save_to_disk` 0x4dc3a0 (0x4dd0a1) is odd: maybe the map's boundary [inferred]."
- **New:** It is the community map's boundary **[proved]**: the 0x4dc3a0 range (3408 bytes) covers several routines;
  0x4dd0a1 is in the handler 0x4dcd70 of the **"Transfer Request"** dialog (builder 0x4dcb30; both registered at
  0x4dcab5/0x4dcaba; texts "Put <…> on the transfer list at his own request ?", "Yes", "No"). On "Yes": it clears the
  loan list (0x4dd060..0x4dd069), **sets bit 8** of +0x4e (0x4dd06c..0x4dd075, so bit 8 = listed at his request), event
  0xc (0x544710), forced showcase (0x842c00 with 1, at 0x4dd0a1) and 0x4d71b0.

### B3 (line 323) Other ways of writing bit 2
- **Old:** "writes through other instruction forms were not scanned: inferred there are none".
- **New:** Scan of the whole `.text` for **byte** writes to `[reg + 0x4e]` (mov/or/and/add, immediate or register)
  **[proved]**: bit 2 is only set at 0x83dd42/0x83dd98, 0x83e49c, 0x83e651 (0x83cb40) and 0x8442c4 (0x8442b0, screens).
  The other register writes are contract copies (0x4cc6dc, 0x4cc903, 0x4ce39f, 0x4cf9d8, 0x4cfc10, 0x4cfdb9, 0x4d000f,
  0x4d0546, 0x4d06a5, 0x5fbb51), clears (0x83d287 `& 0xfc`, 0x8442ef `& 0xfd`, 0x4d8747/0x4dd069/0x784848/0x8f80a1
  `& 0xfd`, `& 0xf7`, `& 0xfe`), constructor zeros (0x85e458, 0x85e4f4, 0x85e6a4, 0x85e9db, 0x866eeb) or touch bits 1,
  4 and 8. **Still inferred:** the wider writes covering +0x4e (265 `mov dword/word` at +0x4b..+0x4e in all structures)
  were not looked at one by one.

### B4 `assess_club_squad` 0x83cb40 (§3.1, the steps marked [inferred])
General proof: disassembly 0x83cb40..0x83f3a0 (3168 lines), read whole for item `mkb_2`; I checked the points below.

- **Step 1 (old: "with a linked B team … without the partner, leaves [inferred; the call at 0x83cc04 proved]").** New
  **[proved]**: DAYS = 0 (0x83cbc4); reset is forced when slot 0's +0x23 is 0xff (0x83cbb4..0x83cbbb); it leaves with
  ACTIVE 0 (0x83cc16) for a national team (0x53b900), no nation, a human club with `daily` (0x600a50 with 0, 1),
  `PROFESSIONAL_STATUS` 3, or when 0x540a50 finds the pair and says this club **is the B one** (0x83cc04..0x83cc14:
  return ≠ 0 and the "is the A" flag 0): the A club runs the list.
- **Step 3 (old: "Without reset: the slots are only sorted (qsort 0x945333) [inferred]").** New **[proved]**:
  `qsort(slots, 40, 0x25, 0x83b6c0)` at 0x83cfe5..0x83cff1 (order by the slot index).
- **Step 4 (old: slots without a target … [inferred]).** New (mkb_2 header, 0x83cff9..0x83d1c7): a slot without a
  target, or with a target, no offer (`OFFER_REF` −1) and a passed date or retired (job 0x10), loses target, replacement
  and ceiling (the target, if in the staff pairs, gets event 3 in 0x843e10); its position goes to its status's list (2
  and 3, 4, 5).
- **Step 5, the end (old: "Injured … for more than 30/300/400 days (by the rating 0x6e6340 against the target rating ×
  0.75) goes to the reserves; the others go to one of two lists [inferred]").** New **[proved]**: the threshold is
  **150 days** (0x83d32d); **30** when random(recurrence level 0x62ece0 + 1) ≠ 0 (0x83d33e..0x83d34b); otherwise, injured
  with a score (0x6e6340) ≥ the target: **400** (0x83d37f..0x83d387); score ≥ ¾ of the target (0x969040 = 0.75):
  **300** (0x83d391..0x83d3a8). Days left ≥ threshold, or a retirement announcement (0x7a7ed0), sends him to the
  reserves (0x83d3f4; with a B team only above 300 days, 0x83d3d8..0x83d3ea) and, at a human club, clears the status
  (0x83d40e). The others go to the rest or to the list of those on loan elsewhere (0x4cf6d0 at 0x83d41b), and with
  `reset` happiness goes to 0.
- **Step 6 (old: "those older than goalkeeper/10 + 22 go to the end [inferred]").** New (mkb_2, 0x83d474..0x83d8ac; the
  three 0x6e2000 calls at 0x83d512, 0x83d636, 0x83d7f9): the rest goes to the status 2 and 3 positions, then 4; those
  on loan elsewhere join the rest; those older than `player_sheet::GOALKEEPER` / 10 + 22 (24 for a goalkeeper rated 20,
  22 for the others) leave it (the swapped-in one is looked at again); the rest goes to the status 5 positions; the old
  ones come back.
- **Step 7, "the rest" (old: "[inferred as a whole; the marked points proved]").** New (mkb_2, 0x83d8bf..0x83de6b): the
  threshold is ftol((age × 0.13 − 1.98) × base × 0.02), the cut base/68 − 35 and the level ftol(reputation × 0.25 + PA ×
  12.5 + CA × 12.5 + club reputation × 0.25); "surplus" = judged below the threshold (0x6e86d0 mode 1, 0x83da02) and a
  type 2 contract with less than professionalism + pressure + 450 days at the club or an end 450 days or more away, or
  `staff::VALUE` ≥ reputation × 50, or the value ≥ the wages left (days × wage / 30). The points already proved in the
  map stay (0x83dba7..0x83dc42, 0x83db6b, 0x83dc47..0x83dc66, 0x83dc88, 0x83dce6..0x83ddb2). Added: with a B team or at
  a human club, under 27 (goalkeeper 20) or 23, status 5 when ¾ of the level passes the threshold + 20, else 6.
- **Step 8 (old: "the contract's status from the slot's (except 1 and 2) [inferred]").** New (mkb_2,
  0x83e185..0x83e66f): the replacement is the pick; with no manager on holiday the contract takes the slot's status,
  except in list 2/3 over statuses 1 and 2; the slot date comes from his arrival (+120 days, with `reset` and a
  manager). Status 4: at slot 24, with a B team or at a human club and not a big club, the pick (not the manager) goes
  to the reserves. The loans of lists 4 and 5 were already proved.
- **Step 9 (old: "[inferred]").** New (mkb_2, 0x83e66f..0x83ebfb): p = min(target, 36000) (× 0.75 in a goalkeeper slot
  with no replacement). Without a replacement: valuation p and price p × 2.5 + reputation, 2p + rep., p × 1.8 + rep., p ×
  0.8 or p × 0.5 by status 1, 2, 3, 4/5, other (× 3 in a goalkeeper slot). With one: to the team (2, 3, 4; with `reset`
  happiness + 50, 10 matches left out) or the reserves (5), scored for the position (in the first 200 days × (1.25 −
  days × 0.00125)), within −10000..60000; score ≥ p, no grievances 0x100 or 1 and more than 364 days of contract: he
  stays the target. Otherwise happiness moves by ftol((1 − score/p) × −100) (with `reset`; statuses 2 and 3 at least 25
  and the slot loses him; the others at least 0; status 5 gets status 6), and the price is (rep. × f + p × k) − score ×
  k.
- **Step 10 (old: "below it and with no offer (0x4cc9e0), sacked … [inferred]").** New **[proved]**: the sacking only
  happens with the local flag [esp+0x60] set (zeroed at 0x83cb6c), and it is only set at 0x83cc44..0x83cc64: **the list
  was at ACTIVE 2, with no bids, and the club is not human** (0x600a50 with 1, 1). Then a `score_staff_for_job`
  0x69d1b0 score below min(target/5, 5000) and no contract offer (0x4cc9e0): `remove_staff_from_club` 0x690890
  (0x83ee28..0x83ee72; the same for the other 4 jobs at 0x83ef46, 0x83f056, 0x83f15f, 0x83f275). Otherwise, price
  (threshold − score) × 2.5.

### B5 `fill_shortlist_slot` 0x83f3a0 (§3.2, "[inferred except where marked]")
General proof: disassembly 0x83f3a0..0x840d80 (2048 lines), read whole for item `mkb_3`; new text = the mkb_3 header;
I checked the points below.

- **Step 1 (exits).** Confirmed by mkb_3: no nation; human (0x600a50 with 0, 1); a human (1, 1) on holiday with rule 4
  = 1; a slot with an offer; a national team.
- **Step 3 (reputation factor).** Correction: reputation × 0.0008; in ITA, ENG, GER, FRA and ESP with finances, by the
  wealth band (0x5986b0): band 1, × 1/1175 (× 1/1125 above 6250); band 2, × 1/1125 (× 1/1050 above 4750).
- **Step 4 (filters).** Corrections **[proved]**: status 5 (`(+0x24 & 0x1e) = 0xa`) and age > 23 skips
  (0x83fb79..0x83fb8d); purchase: the ceiling (+7) below price × 1.3 (0x968de8) skips (0x83fb97..0x83fbad); loan: passes
  with a club and a price ≤ 0, otherwise must be loan listed (0x83fbb8..0x83fbd7). Also (mkb_3): id ≥ the number of
  people − 16; `job_for_club`/`job_for_nation` 5; a human on holiday with rule 4 = 2 only considers his scout list; a
  retirement announcement (0x83fb53..0x83fb5d); Saudi (0x83fb63..0x83fb73). Non-player: the rest of a type 2 contract
  (days × wage / 7) ≥ the ceiling or > (reputation + 500) × 10 skips. Loan from a human: only listed (bit 2, 0x83feb8).
- **Step 5 ("whom the club knows").** Rewritten **[proved: 0x83fed8..0x8400d4]**: known when (0x83fed8) the human's
  scout list has him, or (0x83feea..0x83fefd) (person id + club id) % k = 0. Otherwise, **of another nation** (0x83ff03)
  without caps (+0x22, 0x83ff11) and whose club is not of the club's nation (0x83ff1c..0x83ff26): a player under 23,
  foreign for the club (0x53d980 with 1) and with world reputation (+0xd) < 3250 is skipped (0x83ff2c..0x83ff4f); from
  another region (+0x75) he needs club reputation ≥ 2750 and (his club with no nation or of the same region, or club
  reputation ≥ 4250, or the nation's `league_standard` +0x85 ≥ 12) (0x83ff55..0x83ffa3); and then his reputation as the
  manager sees it (0x5407e0) ≥ 2750 + random(2000) in status 5/6 slots (with world reputation ≥ 1750) or ≥ 3250 +
  random(2000) in the others (world ≥ 2750) (0x83ffa9..0x840058). **Of the same nation** (or with caps, or at a club of
  the nation; 0x840060): a non-player, a loan slot, no sheet, home reputation (+9) ≥ 3250, his club ≥ 3250 or 20 years
  or more: known; otherwise the youngsters filter (D1).
- **Step 6 (score and price).** Correction (mkb_3): below the valuation or the score to beat − 10000 skips; a purchase
  at a seller in crisis: at most the seller's value; a compensation deal (0x84b6f0): its fee; otherwise the asking price
  (0x4d42d0 human at 0x8402e6, 0x4d1600 AI at 0x8402fe); status 4 under 28 or status 5: the excess over the value adds
  min(asking − value, (rep./13000 + 0.5) × value, 2,500,000) to the offer; prediction 0x84af00 = 2 skips; a price above
  the ceiling skips; score × y, y = 1 − (asking − value)/(max price × factor) (above 1 → y/3 + 0.67; 0.01..3); injury >
  15 days: − (days − 15)²; loan: the owner's prediction 2 skips, injury > 10 days − (days − 10)² × 200; without
  `can_club_bid_for_player` skips; finally score × (92 + random(17))/100.
- **Step 7 (willingness and wage).** Confirmed with additions (mkb_3): willingness 0x846920 (non-player: 0x844780 at
  0x840682); interest news with random(79 − rep./150) = 0 and (rep. > 5000 + random(2000) or a human club); willingness
  ≥ 3.15: the desired wage (0x864550 at 0x840893, as f32) × max(3.5/willingness, 0.9) above 1.2 × the cap (0x5968d0 at
  0x84091d) skips; otherwise he is the best so far. The scout story (0x587220) by draw.
- **Step 8 (loan slot).** Confirmed with the detail (mkb_3): only with the season flag (key nation: between the season
  start − 28 and its end − 28; other nations always; non-player slot never), and the nation's league loaded **or**
  random(reputation) > random(20000).

### B6 (line 375) WORK_RATE in list 4
- **Old:** "[code proved; intent inferred]".
- **Still inferred (intent).** The code is proved: `movsx edx, byte [eax+0x44]; cmp edx, 0xcb2` at 0x83e482..0x83e486
  (always below). That `WORLD_REPUTATION` (+0xd, word) was meant cannot be proved from the code.

### B7 (lines 392-394) `chance_to_know` with 6 and 5
- **Still inferred (intent).** The code is proved (mkb_3, 0x83f4d8..0x83f5b8).

### B8 (line 502) Chairman and directors in 0x83cb40 and 0x83f3a0
- **Old:** "not read … [inferred for 0x83cb40 and 0x83f3a0 from the pseudo-C]".
- **New:** **Not read directly** **[proved]**: no access to `[club + 0xbf]` or `[club + 0xc3]` in 0x83cb40..0x83f3a0 or
  in 0x83f3a0..0x840d80. **But they come in through the calls**: 0x83cb40 calls `update_club_target_reputation`
  0x83bf40 (0x83cda7), which reads the chairman's Patience (+0x1d) and Resources (+0x20) (section D §3); 0x83f3a0 calls
  `get_max_wage` 0x5968d0 (0x84091d: the chairman's Business +0xf and Resources) and the asking price 0x4d1600
  (0x8402fe, 0x840bfb).

### B9 (§6, items 2, 3, 4 and 5) The intents
- **Still inferred (intent)** in all of them: each one's code is proved; whether it was meant cannot be proved from the
  disassembly.

## Section C

### C1 (line 691) The +2 of the window dates
- **Old:** "+2 the weekday it moves to (0xff none) [inferred]".
- **New [proved]:** `transfer_window_get_date` 0x40a7d0 passes byte +2 sign-extended (0x40a7d1 `movsx ax, byte
  [ecx+2]`) as the 4th argument of `Date::Date` 0x549f70; there, a value ≠ 0xffff (−1) calls `Date::get_nearest_weekday`
  0x54a2d0 (0x54a0ea..0x54a0fc). So +2 = the weekday the date is moved to (the nearest one), 0xff = none.

### C2 (line 733) The zero-month period on the human's screen; and the 1972 doubt (§12)
- **Still inferred:** screen only (no change to the automatic career). The builder 0x8f9690 puts the period in
  controls 3-6 and 0xc/0xd (0x8f980f..0x8f98ac); the handler is 0x8fac30 (registered at 0x8f9747) and the confirmation
  goes through 0x8fb5e0; I did not follow it to where the 1972 end or the zero period becomes (or not) a refusal.

### C3 (line 739) The forum's "58.2"
- **Still inferred:** only reproducing it in the game proves it.

### C4 (§8) Offers stuck during the holidays (states 9, 0xb–0xd)
- **Old:** "The defect [inferred, not reproduced] …".
- **New:** the mechanism is proved; only the effect in the game is still inferred (not reproduced). `process_due_offer`
  does not compare 8, 9, 0xb, 0xc, 0xd (the status comparisons are those of 0x8c6708..0x8c6972 and there is no jump
  table); the weekly sweep skips 0x13, 7, 8 and 0xa (0x8c5964..0x8c5973). Writers (scan of `mov byte [reg+0x2c], imm`):
  9 only at 0x8c6a8c (`process_due_offer`, human buyer); 0xb at 0x8c31e7 (0x8c2f50); 0xc/0xd in 0x8c3310
  (0x8c336e/0x8c335b). Register writes not scanned.

### C5 (§8, "And loans?") The stuck loan
- **Old:** "The only one that does not lapse is 8 … a loan whose player decision never comes stays stuck. That this is
  xeno's item 88 is **inferred**; the exact path that loses the decision was not found."
- **New [proved, code path; not reproduced in the game]:** a loan can stay in state 8 forever, without holidays:
  1. The (AI) owner accepts: `contract_manager_schedule_decision_for_transfer` 0x4cc930, state 8 and **no
     rescheduling** (0x8c6577..0x8c6592 in `process_due_offer`; 0x8c0680/0x8c068b in `accept_loan_offer`). 0x4cc930
     marks the (person, club) contract offer that has +0x62 = 0 and schedules the decision (0x4cc975 is the only write
     of 1 to +0x62 outside `add_contract_offer`).
  2. The loan's contract offer is type 6 (`init_from_current_contract` 0x85e590: `& 0xc6 | 6`), and
     `add_contract_offer` 0x4cc490 writes +0x62 = (type ≠ 6): 0 for the loan (0x4cc597..0x4cc5b7 for a new offer;
     0x4cc741..0x4cc761 for an updated one).
  3. The player's decision (0x4ce4e0) only looks at offers with +0x62 ≠ 0 (0x4ce63c..0x4ce641). If he picks **another**
     offer tied to a transfer, the loan's becomes state 1 and `handle_player_rejected_terms` 0x8c28c0
     (0x4ced1b..0x4ced32: the transfer goes to 0xf, round 3, back in 10 days); and the loan's contract offer is **held**
     in the list at +0x38 (0x4cf0b5..0x4cf13d, a club other than the chosen one's and the chosen one tied to a slot ≠
     −1) and leaves the live list (0x4cf35d..0x4cf383).
  4. If the chosen transfer is **withdrawn** at state ≥ 0xb (`withdraw_transfer_offer` 0x8bfcb0, 0x8bfd3a; the only
     caller of 0x4cf4a0, at 0x8bfd4a) while the loan still exists (the 10 days of step 3), `resubmit_held_offers`
     0x4cf4a0 puts the transfer from 0xf into **8** (0x4cf4fb..0x4cf501) and resubmits the offer with
     `add_contract_offer(…, decides 1, agreed 0)` (0x4cf505..0x4cf52a): type 6, so +0x62 = 0.
  5. The scheduled decision skips that offer (step 3); 0x4cc930 is not called again (only on the state 0 acceptances);
     `process_due_offer` has no branch for 8; the sweep skips 8. **The loan stays in 8 for good** and the player stays
     "under offer" (bit 0 of the 8-byte record is only cleared with the offers resolved, and 8 does not count).
  The other loan states lapse (0 with a human owner: state 4, 14 days; 1 with a human buyer: state 4, 10 days; 0xf:
  removed on its return with round 3). The decision is also put off 2 days at a time while the player has another offer
  in 0, 1, 5 or 6 (`player_has_pending_offer` 0x8cd150 at 0x4ce5ce), but that ends when the other one lapses. **The
  holidays play no part** in this path (neither 0x4ce4e0 nor 0x4cf4a0 looks at the human). Still inferred that it is
  xeno's item 88. **Small fix:** in `resubmit_held_offers`, for a loan transfer (`offer_type` 1), call 0x4cc930 after
  the resubmission (or write +0x62 = 1), or in `process_due_offer` (Rust) treat an overdue loan in 8 as a refusal.

### C6 (line 791) The natural end of a loan
- Outside this closing: the call is in 0x4c7ac0 (another reader).

### C7 (line 830) The loop of `ENGLAND_RULES::can_loan_player`
- **Old:** "tests the id of the target `player`, not of the squad player … [inferred]".
- **New [proved]:** at 0x579ea8..0x579ec2 the test reads `[esp+0x84]` (the 2nd argument, the target player, also read
  at 0x579d65) and not `esi` (the slot's player); with a target of valid id the test always passes, with no effect.

### C8 (§12) The doubts
- 1972 on the screen: see C2 (still inferred).
- Which state traps the loan: answered in C5.
- Types 3 and 4 in the data: **still inferred**; needs a scan of the contracts of a save or of the database (outside the
  disassembly).

## Section D

### D1 (line 1019) The `edi` of the youngsters filter at 0x8400ae
- **Old:** "goes on only if edi mod (25 − youngsters) = 0 … [the test proved; what edi is, inferred: a counter or id]".
- **New [proved]:** `edi = person id + club id` (0x83fee7..0x83feee: `ebx` = the club from [ebp+0xc], `lea edi,
  [eax+ecx]` with `[esi]` and `[ebx]`), the same sum as the % k test (0x83fef1..0x83fefd); no write to `edi` between
  0x83feee and 0x8400b9 on any of the three paths reaching 0x840060 (0x83ff0b, 0x83ff16, 0x83ff26). So it is **fixed per
  (person, club) pair**: the same youngster is always accepted or always ignored by the same club while the manager is
  the same; Youngsters 20 accepts the pairs whose sum is a multiple of 5. Exact conditions (0x840060..0x8400d4): a
  player slot that is not a loan slot, with a sheet, home reputation (+9) < 3250, his club null or < 3250, age < 20;
  manager with a non-player sheet: % (25 − `YOUNGSTERS` +0x22) (0x8400ae..0x8400c0), otherwise % 15
  (0x8400c8..0x8400d4). And only someone who already failed the % k test gets here.

### D2 (line 1020) Youngsters at 0x4d3ccb
- Outside this closing (0x4d1600, another reader).

### D3 (line 1049) `calc_contract_value`, case 0
- **Still inferred (intent).** The code is proved (mkd_1).

### D4 (line 1053) `get_max_wage` "looks inverted"
- **Old:** "looks inverted [inferred]".
- **New:** the rule is proved; "inverted" **is not supported by the code**: in both money routines a chairman with high
  Business spends less: in the budget he keeps more weeks of wages in reserve (Business + 6, 0x598b24..0x598b2a) and
  in the wage cap he pays × 0.95 (0x597a0d). A prudent chairman, consistent in both. Whether that was meant is still
  unproved.

### D5 §2.2, §2.3, §2.4
- Outside this closing (0x4ce4e0, 0x85fc50, 0x864550, 0x862720, 0x4c7ac0).
