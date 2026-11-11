*English version of [fechamento_mkd2.md](fechamento_mkd2.md). Both are kept in sync; the Portuguese file is the original.*

# Closing the market map, reader D2: 0x4ce4e0 and 0x4c7ac0 (11/11/2026)

Routines: `CONTRACT_MANAGER::resolve_contract_offers` (0x4ce4e0, the person chooses among the offers, renewal included)
and `CONTRACT_MANAGER::daily_update` (0x4c7ac0, contract end, AI renewals, free agents). Everything below comes from the
disassembly of `jogo/cm0102.exe` (`desmonta.py`), with the instruction address; the constants read from the exe. For
each claim of the map (`docs/MAPA_MERCADO.en.md`, section D, §1, §2.2, §2.4, §2.6, §4, §5, §7): the old text, the new
one and the proof. The main session replaces [inferred] by [proved].

## 1. 0x4ce4e0 `resolve_contract_offers` — item **mkd_4** (ported)

Convention: thiscall(contract manager, person), `ret 4` (0x4cf493), SEH frame. Single caller: 0x4cda4c, inside
`process_due_decision_dates` (0x4cd9c0), when a type 3 decision date falls due. Same in cm0102-1080.exe (0xfb6 bytes).

Constants (read from the exe): 1/300 = 0x968e60 (0.0033333333333333335); 3.15 = 0x968e58; 0.99 = 0x968e50; 0.6 =
0x968e48; 0.7 = 0x968e40; 3.5 = 0x968e38; 0.0 = 0x9689a0; 0.9 = 0x9689d8; 0.8 = 0x9689b0; 0.5 = 0x968968; 10 =
0x9688b0; 40 = 0x968e30; 5 = 0x9688a0. The immediates 2.75 (`mov [esp+0x40], 0x40060000`, 0x4ce51b) and 3.15
(`0x40093333_33333333`, 0x4ceaab) are not in `.rdata`.

| Map claim (§2.2) | New text | Proof |
|---|---|---|
| 1. k = (professionalism − ambition − 9 + loyalty) × 1/300 | same, **[proved]**: the int (+0x5b − +0x57 + +0x59 − 9) goes through `fild` and `fmul` 0x968e60 | 0x4ce5a4..0x4ce5ca |
| 2. With a pending transfer offer, postpones 2 days and leaves | **[proved]**: `player_has_pending_offer` (0x8cd150); today + 2 (0x54c5b0) and a type 3 decision date at the current slice (0x4cd6d0 with [0xae2c88]); nothing else | 0x4ce5ce..0x4ce606 |
| 3. An offer without a transfer from a club that is not the contract's and type ≠ 5: removed | **[proved]**, only the person's offers with +0x62 ≠ 0; the contract is looked up with the store in [0xae1080]; the removal takes it out of the list and counts down byte +8 of the 79-byte record | 0x4ce630..0x4ce6fa |
| State 0 (the rest is skipped; state 2 marks "already agreed") | **[proved]**: +0x50 ≠ 0 skips; = 2 sets "agreed", and in the end nothing is chosen | 0x4ce701, 0x4cec69..0x4cec6e, 0x4cec90..0x4cecab |
| Type 5: will = 0x846920(…, loan 1, status 8); type 6 with `get_squad_status_for_club` | **[proved]**; also: the will is the offer's score, chosen **at once** when above the best and ≥ 3.15 (before the refusals: a later refusal zeroes the score, but the offer stays chosen) | 0x4ce70c..0x4ce78c, 0x4ce791..0x4ce827 |
| The others: desired (0x85fc50 with −1, status, job); ratio = value(offer)/value(desired) − k | **[proved]**: the status (high nibble of +0x4f) first goes to byte +5 of the person's entry in the transfer offer (0x8c61e0, 0x8e82f0); the desired is **one** local contract reused from offer to offer; both values are against the desired (`ecx` = desired, then the offer); a desired value ≤ 0 gives score 0 | 0x4ce82c..0x4ce8f1 |
| The will by 0x846920 (player) or 0x844780 (staff; averaged for a player job) | **[proved]**: 0x846920 without a non-player sheet (+0x69 = 0) or with job 11; otherwise 0x844780 and, if `is_player_job` (0x53b620), (0x846920 + 0x844780) × 0.5 | 0x4ce8d8..0x4ce9b8 |
| Renewal: ratio ≥ 0.99 and will ≤ 3.15 → 3.15; ratio < 0.6 → 0 | **[proved]**; **corrected**: the 0.6 cut applies to **all** plain offers, not only the renewal. Renewal = the contract's club (looked up again, [0xae1080]) is the offer's | 0x4ce9bc..0x4ceaab (renewal), 0x4ceac5..0x4cead0 (cut) |
| Refusal: a player job without a player sheet (0x7a6e20 false), or ratio > 0.5 with a permit and under 90 days | **corrected [proved]**: (a) a job **other than plain player** (`0x53b610`) for someone **without a non-player sheet** (+0x69 = 0) when **no spare non-player record** is left (`PLAYER_REGEN::has_spare_non_player_record` 0x7a6e20 = 0); (b) ratio > 0.5, a work permit needed (0x8c7560) and the end of **the offer's** contract under 90 days away (0x54cdb0 of +0x2d to today < 0x5a). Applies to every type (loans and trials included) | 0x4ceae2..0x4ceb70 |
| Score = will × ratio, with the cuts 0.9, 0.8, 0.7 and 3.5; the highest above 2.75 | **[proved]**: ratio ≥ 0.9 → will = max(will, 3.15); ratio > 0.8 and will > 3.15 → kept; ratio > 0.7 and will > 3.5 → max(will, 3.15); otherwise turned down. Score = will × ratio; above the best (starting at 2.75) → chosen; a renewal not chosen (or turned down) is remembered. Loan/trial offers (desired = 0) do not go through here | 0x4ceb9b..0x4cec67 |
| (new) | An agreed offer, or a person with a non-player sheet who is a candidate for a manager vacancy (`manager_manager_is_vacancy_candidate` 0x69fc60), chooses nothing **[proved]** | 0x4cec90..0x4cecab |
| 4. The chosen becomes state 2 (0x7549b0, 0x8c5410); the others 1 (0x4d49a0 or 0x8c28c0) | **[proved]**; `handle_player_rejected_terms` gets the chosen offer as its 3rd argument; news 0x4d49a0 gets 1 only for the turned-down renewal of a player (job 11) when nothing was chosen | 0x4cecaf..0x4ced73 |
| None chosen and an offer from his own club: mark 0x80 in byte +0xb | **[proved]**, more precisely: with nothing chosen, a **turned-down** renewal sets bit 0x80; with a choice, the bit is **cleared** at the end (except when the completion fails) | 0x4cf071..0x4cf094, 0x4cf42d..0x4cf447 |
| For the chosen one (types ≠ 5, 6) with a score above 0x9688b0, draws by loyalty [inferred] | **[proved]**: score > 10 → n = max(40 − score, 5); `random(ftol(n))` < loyalty, the offer's club has a manager, and club reputation > 4500 or `random(4)` = 0 → `random(2)` ≠ 0: comes to favour the manager (`staff_add_favoured_staff`); = 0: the club (`staff_add_favoured_club`) | 0x4cef64..0x4cf06f |
| (new) Third pass | **[proved]**: with a choice, the same club's offers without a transfer **complete** the contract (`complete_accepted_contract_offer` 0x8c38b0 with the local copy; on success: a staff record for a staff job without a sheet, the coaches and the scouts' lists when the contract's club is human, the player value recomputed); they leave the list and clear the agreed flag; with a transfer they stay. Other clubs' offers leave (into the held list +0x38 when the chosen one has a transfer; the club into the list 0x8c2180 gets). With none chosen, every offer outside state 3 leaves. A failed completion ends there | 0x4cf096..0x4cf3a2 |
| (new) End | **[proved]**: chosen with a transfer → `handle_player_accepted_terms` (0x8c2180: person, slot, copy, the job dword +0x61, the clubs list), first a staff record for a staff job without a sheet; without a transfer → news 0x4d49a0 of the copy | 0x4cf3a8..0x4cf42d |

### Defects found (0x4ce4e0)

1. The spare staff record is accepted up to **the count itself** (`cmp eax, [0xae2368]; jg` is the error, it should be
   `jge`) **[proved]** 0x4cf1b5.
2. With several offers from **the same club** and the chosen one without a transfer, `complete_accepted_contract_offer`
   runs **once per offer** (the loop calls it again for each) **[proved]** 0x4cf0c9..0x4cf15c.
3. Loans and trials are chosen before the refusals: a later refusal (work permit, no spare record) zeroes the stored
   score but does not undo the choice **[proved]** 0x4ce75c..0x4ce764 against 0x4ceb68.
4. The "renewal" marker and the will are stack variables that carry over between offers: the marker's first value is
   the numerator of k (0x4ce5bc), and an offer with a negative desired value would use a previous offer's **[proved]**;
   no effect in practice (the value is never negative) **[inferred]**.

### Extension points

- **0x4ce4e0 (mkd_4, Rust once integrated)**: the 2.75 threshold, the 0.9/0.8/0.7 cuts, the renewal floor 3.15 and the
  personality k sit in one function (`choose_contract_offer`): the place for the Tweaks to change the will to renew or
  to change club by groups (age, reputation), without touching `staff_interest_in_joining_club`.

## 2. 0x4c7ac0 `daily_update` — **not ported** (see §3)

Convention: thiscall(contract manager, &date, slice (byte), flag (byte)), `ret 0xc` (0x4c9e2e). Callers:
`game_main_day_loop` (0x5cd406: today, the current slice [0xae2c88], 1) at **every slice** of the day, and
`game_run_to_start_date` (0x5cc9ef). Constant: 0.004 = 0x968d60 (progress bar steps).

| Map claim (§2.4, §2.6) | New text | Proof |
|---|---|---|
| "Daily" | **corrected [proved]**: runs at every slice; **slice 0** only recomputes the fixed dates (today + 7, 30, 60, 91, 121, 152, 182, 365, 550, 1095 in 0xae1048, 0xae1030, 0xae1058, 0xae1010, 0xae1060, 0xae1040, 0xae1090, 0xae1068, 0xae1008, 0xae1020); **slice 2** does the people pass (only with flag = 1); every slice ends with the decision dates (0x4cd9c0 with the date and the slice) | 0x4c7b2e..0x4c7c74, 0x4c7c79, 0x4c7de0, 0x4c9e0e..0x4c9e15 |
| (new) Contract talks | **[proved]**: at slice 2, each talks record (21 bytes, +0x28/+0x2c) expires (25 − the manager's loyalty) × 2 days after the date at +0xc; an expired one leaves (swapped with the last) and the array is re-sorted (qsort 0x4dc970). It is the **first use of loyalty** | 0x4c7c81..0x4c7d7b |
| (new) "Updating game data" bar | **[proved]**: on 31/12 (0x549f70 day 31, month 11), every 250 people | 0x4c7b07..0x4c7b29, 0x4c7d7e..0x4c7e2f |
| (new) Before the full load | **[proved]**: with [0xdd711c] = 0 (`G_GAME_DATA_LOADED_FLAG`), a contract ending today only becomes type 1 (+0x35 = (x & 0xc1) \| 1) | 0x4c7e34..0x4c7ec0 |
| (new) Player value recomputed | **[proved]**: human club and a player job: with bit 4 of +0x4e and a sheet, value recomputed (0x4d0c60 flag 1); AI club and a player job: **on odd days of the year**, bit 4 set and value recomputed (flag 0); staff: every day; in the first two, capped by the active release fee (0x867b50). Without a contract or club: value 0 | 0x4c7f65..0x4c8070 |
| Loan end (0x4d07e0) | **[proved]**: the loan contract (index +4 of the 79-byte record) ending today: without a club, only the index becomes −1; otherwise news 6 (0x4da590, outside type 7) and `end_loan` | 0x4c8077..0x4c8114 |
| (new) Renewal before the transfer rules change | **[proved]**: before [0xae2330] (`G_TRANSFER_RULE_CHANGE_DATE`) and with a sheet, when that date falls on today + 7/30/42/60/70/91 (recomputed **per person**, in 0xae1070..0xae1088), the contract started over 180 days ago and ends before today + 1095: a club that does not dislike him and whose manager does not (0x5477d0, 0x547840), AI, age < 32 → by status: 1 offers (factor 1.0); 2 and 3 with current reputation > 6750, or age < 24 and potential > 100; 5 with potential > 100; 4 never | 0x4c8152..0x4c847a |
| FIFA protected period (0x868210) | **[proved]**: a type 2 contract, no loan, ending **after** today + 182, outside the protected period (0x868210 = 0 and the output ≠ 0), not retiring (0x7a7eb0), job 11: the AI (age < 32) renews by the same statuses (factor 1.0); human club: news 7 (0x4d6790; Bosman by the age at today + 182 ≥ 24 and 0x8d0a70) | 0x4c847f..0x4c85fe |
| (new) Type 1 (expired contract) | **[proved]**: no loan, 31 days past the end, wanting to leave (0x866f60) or leaving on a Bosman (+0x49), with no offers, exchanges or queue: news 1 to the human club; jobs 12, 13 (AI) and 15 (AI) go back to 5, 6 and 8 and leave the squad; the others go to the **free transfer queue** (0x8c16b0 with 1, −1) | 0x4c8603..0x4c883b |
| Bosman (0x8d0a70, a nation with free transfers 0x544670) | **[proved]**, the **leaving on a Bosman** rule (+0x49 = 1): type 2, contract started over 93 days ago, inside the bands window (0x867d30 ≠ 0 and ≠ 14), not retiring, job 11, the current contract not a loan, a club with a nation; current reputation > 3750, world > 1750, **loyalty < 15**, age 19..30, the nation allows it (0x544670), and not (club > 4750 and age < 23); and wanting to leave (0x866f60) or bit 8 of +0x4e. Then +0x49 = 1, bit 8 cleared, the clubs' lists (0x842c00) and news 0x4d6d70. It is the **second use of loyalty** | 0x4c88b5..0x4c8a74 |
| The band (0x867d30) against +7..+182 | **[proved]**: the odd bands (the exact day of 182, 152, 121, 91, 60, 30 and 7 days) put the player on the clubs' lists (0x842c00 with 1); band 14 only with `random(20)` = 0; someone already leaving on a Bosman too, on the odd ones | 0x4c8876..0x4c88a5, 0x4c8a79..0x4c8abb |
| The AI renewal offer (`offer_contract`, ×9) | **[proved]**, by the factor immediates (f64) pushed before 0x4cda70 (person, club 0, out 0, slot −1, factor, mode 2), only for an AI club and a player not retiring: with **550 days** left (0xae1008), status 1..3 and on his own club's list (0x849ce0): 1.0; **365 days** (0xae1068, or on the game's first day with an end before that): news 6 and, status 1..3 or 5 on the list: 1.0; bands **3 and 5** (152/121 days): status 1..3 on the list 1.2, off it 1.0; 4 and 5 → 1.0; band **1** (182): 1.1 on the list / 1.0; band **7** (91): 1.3 on the list / 1.0, 1.1 for 3..5; bands **9, 11** (60/30): 1.4 (1..2) and 1.2 (3..5) on the list, off it 1.0; band **13** (7 days): the same, with news to the human club. When 0x4cda70 returns 0 (no offer made) on the 365-day path and on bands 1 and 7, a local type 2 offer becomes the refusal news (0x4d49a0 with 1); on bands 1 and 7 it also sets bit 0x80 of the 79-byte record (0x4c910e, 0x4c91e3, 0x4c93d0, 0x4c94a5) | 0x4c8ac0..0x4c8b9f, 0x4c8ba4..0x4c8e0a, 0x4c8e0f..0x4c8ee3, 0x4c8ee8..0x4c91f1, 0x4c91f6..0x4c9550, 0x4c9555..0x4c9a8b, 0x4c9b5f..0x4c9b71; factors 1.1 = 0x3ff19999_9999999a, 1.2 = 0x3ff33333_33333333, 1.3 = 0x3ff4cccc_cccccccd, 1.4 = 0x3ff66666_66666666 |
| The contract end on the day | **[proved]**: a type 5 (loan) ending today: news 5, stats archived (0x7c5fe0), the contract erased (both indexes −1), the wage bill (0x598370), out of the selections (0x896c60 ×2) and of the club (0x53c190). Any other type: becomes type 1; leaving on a Bosman, no loan and no offers: news 2 and **released at once** (`free_transfer_staff` 0x8c1af0) | 0x4c9b76..0x4c9d21 |
| (new) Listed without offers | **[proved]**: a player (job 11) transfer-listed (0x5444a0) with no offers, exchanges or queue goes into the free transfer queue | 0x4c9a90..0x4c9b10 |
| (new) Decision dates | **[proved]**: on days 36 and 200 of the year (slice 2), the processed ones leave the array; on every call, if new dates were added (+0x1c), the pending part is re-sorted (qsort with 0x4dc910) and +0x1c is reset | 0x4c9d40..0x4c9dd5, 0x4c9dd8..0x4c9e07 |
| "Attributes: only loyalty (+0x59, ×2)" | **[proved]**: talks (0x4c7cc3) and Bosman (0x4c8a01) | — |

### Defects found (0x4c7ac0)

1. The six rule-change dates (0xae1070, 0xae1018, 0xae1038, 0xae1028, 0xae1078, 0xae1088) are recomputed **for every
   player** in the loop, not once a day (same result, wasted work) **[proved]** 0x4c81a3..0x4c827c.
2. The AI player's value is recomputed only on **odd** days of the year (`movsx eax, word [date]; and 0x80000001`), the
   human one every day **[proved]** 0x4c7ffe..0x4c8010; intent [inferred].

### Extension points

- The AI renewal factor (1.0..1.4) is an immediate pushed at each of the 9 calls of 0x4cda70: a detour on
  `contract_manager_offer_contract` (Rust) adjusting the factor by player changes all of them without porting 0x4c7ac0.
- The Bosman rule (loyalty < 15, age 19..30, reputations 3750/1750) sits in one stretch (0x4c89ea..0x4c8a4a).

## 3. Why 0x4c7ac0 did not become an item

It is 9,376 bytes (2,700 instructions, 10 jump tables), ~45 routines called (the UI progress bar included), 9 renewal
paths and the contract end; one routine per item, all at once, is too big to prove with one comparison (it cannot run
twice: news, draws, queues). Proposal for another round: port it in pieces through callers already in Rust (the
player value part, the loan end, the Bosman rule and the renewals by band as helpers), with the loop last.

## 4. What changes in the map

- §1: 0x4ce4e0 → item **mkd_4**; 0x4c7ac0 → "slice 0 dates, slice 2 people", no item (§3 above).
- §2.2: replace [inferred] by [proved] with the corrections of table 1 (the 0.6 cut for all; the refusal is a staff
  job without a sheet and no record; the loyalty draw proved; bit 0x80 cleared on a choice).
- §2.4: replace [inferred from the call list] by table 2 (the slices, the Bosman rule, the factors 1.0..1.4 by window,
  the 550/365 days, type 1 and the queue after 31 days, the value on odd days).
- §2.6: who becomes free: on the end day, only those leaving on a Bosman are released at once; the others become type
  1 and, 31 days later, go to the queue only if they want to leave.
- §5: add defects 1–4 of 0x4ce4e0 and 1–2 of 0x4c7ac0.
- §7: mkd_4 (compiled; proof by the main session).
