# Transfer save pairs made by the game itself

26/10/2026. Lucas's request (through the orchestrator), so that CMRTE can implement transfers in the save:
"before/after" pairs with **one** operation between them, done by the game's own routines, headless, in the lab.
Cross-checked against `D:\Sistemas\CM3SaveEditor\docs\TRANSFERENCIA.md` (read only; another repository).

Convention: **[observed]** = read in the written save, in `ddraw.log` or in the disassembly; **[inference]** = a
conclusion drawn from what was observed.

## Method

- **Base:** a copy of `jogo\FeriasTesteSave.sav` (MD5 `33fda465193b5e8e6d7ace93ecd44559`, checked before and after
  every load and at the end: untouched) **[observed]**. Game date 205/2001 (24/07/2001), human manager = person
  54147, club **4504** (England, nation 61). Minimum database: 54,148 people, 6,603 clubs, 41,974 contracts.
- **Game:** a private copy of `jogo_testes` in `%TEMP%\cm0102_transf\jogo` (robocopy taken while no game was open
  in `jogo_testes`; no saves, `.tmp` files or dumps), only with the diagnostic DLL, `cm0102-1080.exe -windowed
  -seed 12345 -load <copy>`, `CM_FERIAS=1`, `CM_JANELA=960x540`, `CM_OCULTO=1`, **original mode**
  (`CM_ORIGINAL=1`), no clicks, inside the lab lock (the `simular.py` bench pointed at the copy: `game.cfg`, `*.tmp`
  and `hall_of_fame.bin` given back). Type 3 (bit 0x8000 of `game.cfg`, already set in the copy).
- **Diagnostic DLL:** `copias\transf\rust` (HEAD's `rust\` + `src\transferir.rs`; outside git, nothing in
  `rust\src`). On the exit of 0x8387e0 (after `-load` and the `CM_FERIAS` holiday), the trigger
  `CM_TRANSFERIR="X SSSSSSS CCCCC NNNNNNNNN"` (fixed width: an environment of the same size keeps the heap the same)
  runs operation X with person S, club C and number N, saves through `save_game_data` 0x82e430 (`par.sav`, id 0x16)
  and ends the process. `X = -` does nothing: it is the "before" (same day, same save path).
- **Determinism:** three "before" saves (`antes1`, `antes2` and `antes3`, the last with the DLL already rebuilt) came
  out **identical** (MD5 `35448cd3…`) **[observed]**: in the pairs every difference comes from the operation; no mask
  was needed. The "before" is **not** `FeriasTesteSave`: loading and saving alone already change 40 items
  (`general.dat`, `tactics.dat` +2,320, `fixtures_2001.tmp`, competitions…) **[observed]**. Always compare with
  `antes1.sav`.
- **Tools** (in `pares_transferencia\ferramentas`, outside git): `pares.py` (runs the cases), `diff.py` (item by item
  difference, decoding `staff`, `club`, `contract`, `general`, `finance` and `player`), `verif.py` (C1–C8 counts),
  `analise.py` (candidate choice), `copiar.ps1` (the game copy). The full differences are in the `*.diff.txt` files,
  and the counts in `coerencias.txt`.

## The pairs

Folder `D:\Sistemas\CM0102\pares_transferencia\` (in `.gitignore`). The "before" of all of them is `antes1.sav`.

| Case | "After" file | Bytes | MD5 | Operation |
|---|---|---|---|---|
| before | `antes1.sav` (= `antes2`, `antes3`) | 137,618,548 | `35448cd3a6e8c5dcef97920e09b9581d` | none |
| (a) purchase with a fee | `a_compra.sav` | 137,618,995 | `e1d29378733c7b39252309e3d7e151cc` | player 28775 from Oldham (4221) to 4504, fee 55,056 |
| (b) release (termination) | `b_dispensa.sav` | 137,618,585 | `07f48166e770216d03cdd7666e89139b` | player 17198 of 4504 released |
| (b2) sale with a fee | `b2_venda.sav` | 137,619,046 | `ab476c2a70d6b9f47225ca751d48d5d3` | player 29006 of 4504 to Liverpool (3439), fee 21,016 |
| (c) backroom staff | `c_olheiro.sav` | 137,618,967 | `446c1853fb3fb0c8e2e14e5f386014ff` | scout 15, without a club, hired by 4504 |
| (d) loan | `d_emprestimo.sav` | 137,618,722 | `4def2083f12d739b221e022217dd9129` | player 29141 from Luton (3529) loaned to 4504 for 150 days |
| (e) free player | `e_livre.sav` | 137,619,014 | `a8b72dd38040fd146b17b86e2ea5968d` | player 1, without a club, signed by 4504 with no fee |

Before it worked, the sale of 29006 (and of 24372) to Oldham **collapsed** (`check_offer_restrictions` refused,
probably the buyer's budget): the save came out without the transfer and was discarded.

## The routines and the arguments

Pointers: `TM = [0xde1f54]` (TRANSFER_MANAGER), `CM = [0xae1050]` (CONTRACT_MANAGER), person = `[0xae23bc] + 0x6e·id`,
club = `[0xae23b4] + 0x245·id`, today = `0xae2c90` (8 bytes). Names from `referencias\cm0102-function-map`;
conventions checked in the disassembly of the callers **[observed]**.

| Case | Calls (in this order) |
|---|---|
| (a), (b2), (e) | `TRANSFER_MANAGER::add_config_future_transfer` 0x8bf060 (thiscall `TM`; person, buying club, `&today`, fee, −1, 0) with fee −1 = the player's value (+0x52) in cases (a) and (b2), and 0 in case (e); then `process_due_future_transfers` 0x8cbbe0 (thiscall `TM`), which checks permits and restrictions and calls `post_transfer_completed_news` 0x8dbef0 and **`finalise_transfer` 0x8c9390** (→ `move_player_to_club` 0x8c9720 → `CONTRACT_MANAGER::sign_contract` 0x4cf720). It is the path of pre-arranged transfers (called from 0x7a4cfc, the player configuration file ones); it ends in the same `finalise_transfer` that completes an offer accepted on screen. |
| (b) | The "Yes" steps of *Terminate Contract* (`contract_screens_handle_terminate_contract_screen` 0x4e5610): `CONTRACT_MANAGER::remove_contract` 0x4d0b50 (thiscall `CM`; person); `database_remove_staff_from_club` 0x53c190 (cdecl; club, person, 0, 1, 0); `TRANSFER_MANAGER::free_transfer_staff` 0x8c1af0 (thiscall `TM`; person, club, 0, 0, 1 = record only); human club: training `0x8b2880` (thiscall `[0xde1f44]`; club, 0) (and, for a scout, `0x7f7270` with `[0xb74448]`). |
| (c) | The same 0x8bf060 + 0x8cbbe0 as (a), fee 0, with the **job 11 hard-coded in the routine replaced by 9** during the call (the three immediates at 0x8bf1f3, 0x8bf202 and 0x8bf2fc: the offer's job (+0x11), the one passed to `set_to_desired_contract` and the one passed to `add_contract_offer`), restored right after. Reason: the human's backroom hiring (0x4e4ea0) builds a transfer offer (a person from outside the club always goes through one), and `resolve_contract_offers` 0x4ce4e0 **deletes** a contract offer without a transfer offer (+0x59 = −1) from a club other than the current contract's: the first attempt (`set_to_desired_contract` + `add_contract_offer` + `resolve_contract_offers`) wrote a save identical to the "before" **[observed]**. Every new person entering a club goes through `move_player_to_club` → `sign_contract` (the only callers of `sign_contract` are it, `complete_accepted_contract_offer` (renewal) and a repair in the profile screen) **[observed: refs]**. |
| (d) | `TRANSFER_MANAGER::add_config_loan` 0x8bf400 (thiscall `TM`; person, club, `&today`, `&(today + 150 days)`) → `sign_loan_contract` 0x4d01e0. It is the configuration file loan: the wage paid is 0 % (`init_from_current_contract` with 0), not a negotiated one. |

## What changed, item by item

Only the listed items changed; the other container items (127 to 136 of the 142; 15 changed in (a), 6 in (b), 14 in
(b2), 10 in (c), 8 in (d) and 13 in (e)) came out **identical** byte for byte **[observed]**. No item was created or
removed (142 / 142). The `general.dat` header (date, counts) does not change in any case.

### Core (what every case of entering or leaving a club writes)

| File | Field | (a) purchase / (b2) sale | (b) release | (c) scout / (e) free | (d) loan |
|---|---|---|---|---|---|
| `club.dat[A]` | the person's squad slot | `p → −1` **without compacting** (A 4221 slot 34; 4504 slot 5) | `p → −1` (slot 34), without compacting | — | **nothing** (the owner 3529 keeps the player in the squad) |
| `club.dat[B]` | **first** free slot of the group | squad: 4504 slot 1; Liverpool slot 37 (the first −1) | — | scouts[3] (c), squad[1] (e) | squad[1] |
| `club.dat[A]`, `[B]` | i16 at +128 | ±1 (A −1, B +1) | — | (e) +2; (c) nothing | — |
| `staff.dat[p]` | +57 club | A → B | A → −1 | −1 → B | owner → **loan club** |
| | +62..+69 joined (date + i32 leap) | → 205/2001, leap 0 | unchanged | → 205/2001 | unchanged |
| | +70..+77 leaves | → end of the new contract | unchanged | → end of the new contract | unchanged (owner contract end) |
| | +78 wage | → new contract's wage (250 → 959; 50 → 3,757) | unchanged | 0 → contract wage (1,216; 384) | unchanged |
| | +82 value | recalculated (55,056 → 31,285) | unchanged | (e) 0 → 28,059; (c) nothing | unchanged |
| | +94 (1 byte) | (a) 2 → 1; (b2) already 1 | unchanged | (e) 2 → 1 | unchanged |
| | +61 job | unchanged | **unchanged** (11) | unchanged (9 and 11) | unchanged |
| `contract.dat` | record | **the same index rewritten** (21368, 21527): +4 club, +8, +12, bonuses, +37 start = today, +45 end, +53, +54..72, +79 | **+0 person = −1** (the record stays as a free slot; `n` unchanged; +4 keeps the old club) | **new at the end** (index 41974 = old `n`; `n` 41974 → 41975) | **new at the end** (41974), type 0x46, wage 0, +28 = `01 01`; the owner's one **unchanged** (type 0x2) |
| `general.dat`, entry of `p` (0xfa7 + 79·p) | +0 contract | unchanged (same index) | → −1 | −1 → 41974 | unchanged |
| | +4 loan | — | — | — | −1 → 41974 |
| | +8 (byte) pending offers | +1 (see "leftovers") | — | +1 | — |
| | +10 (byte) | 0 → 13 (a); 3 → 9 (b2); 0 → 9 (e) | — | — | — |
| | +34 i32 previous club | −1 → A | — | — | — |
| | +38 (byte) | 0 → 5 | — | — | — |

Dates: `i16 day of year, i16 year, i32 leap` **[observed]**. New contracts: start = today (205/2001); end = today
+ N·365 − 1 day (204/2006, 204/2005); 2 to 5 years drawn by 0x8bf060 when not given.

### What the game also writes (outside the core)

| Item | (a) | (b) | (b2) | (c) | (d) | (e) |
|---|---|---|---|---|---|---|
| `finance.dat` (359-byte records, one per club and national team: 7,029) | A: +0 balance +55,056, +120/+280 transfer income = 55,056; B: +0 balance, +12 wage bill, +20, +112/+272 spent = 55,056, +156/+316 signing-on fees (+3,126 = contract +8) | B: +12 wage bill, +20 | both clubs, as (a) | B: +12, +20 | B: +12, +20 | B: +0 balance −2,798 (= contract +8), +12, +20, +156/+316 +2,798 |
| `transfer.dat` | +118 bytes; counts at +0 and +4 +1 | **+37** (1 past transfer record, `add_past_transfer`, `free_transfer_staff` with record) | +122 | +114 | — | +114 |
| `contract.dat` tail (after the records) | +111 (1 contract offer of 0x6f bytes) | — | +111 | +111 | — | +111 |
| `news.dat` (fixed size 22,504) | 1 news item (220 bytes) | — | 437 bytes | 1 news item | — | 1 news item |
| `staff records.dat`, `club records.dat` | yes | — | yes | `staff records` | — | `staff records` |
| `player.dat` (record = staff +97, 70 bytes) | +69 | — | +9..+16, +69 | — | — | +11..+12, +69 |
| `player stats.dat` | yes | — | yes (player bytes → 0) | — | yes (player bytes → 0) | yes |
| `tactics.dat` | the player leaves A's tactic (id → −1) | — | — | — | leaves the owner's tactic | — |
| `man_conf.dat` | yes | — | yes | — | — | yes |
| `scout_manager.dat` | — | — | — | 10 bytes (the new scout) | — | — |
| `staff history.tmp` | +171 | — | +171 | +114 | — | +114 |
| `player stats history.tmp` | +47 | — | +94 | — | +94 | +47 |

**Leftovers of the configuration path [inference]:** in the cases that go through 0x8bf060, the contract offer it
creates stays pending (`contract.dat` tail +111 bytes, `general +8` +1) and the transfer offer stays in
`transfer.dat` marked as completed (state 0x14, removed in 10 days by 0x8c6220). In the normal flow (offer accepted on
screen) the contract offer is consumed when the player accepts. Not a coherence rule: the editor should not create
them.

## Mandatory coherence (what the editor has to write) [inference from what was observed]

1. **Clubs:** leaving A = the slot becomes −1, **without compacting** the list; entering B = the **first** −1 slot of
   the job's group. Loan: the owner **keeps** the slot.
2. **Person:** +57 = new club (or −1, or the loan club); on entry, +62 = today, +70 = new contract end (+74 leap
   with it), +78 = contract wage; on release and on loan, only +57.
3. **Contract:** with a current contract, the **same record** is rewritten (the index in `general.dat` does not
   change); without a contract, the new record goes to the **first free slot (+0 = −1)** or, with none, to the
   **end** (`n` + 1; `sign_contract` 0x4cf720 looks for a free slot before growing **[observed in the disassembly]**);
   loan = new record, pointed to by `general +4`. Release = `+0 = −1` in the record and `general +0 = −1` (the record
   **does not disappear**, the size of `contract.dat` does not change).
4. **`general.dat` index:** +0/+4 consistent with 3. Bytes +8, +10, +34, +38 change in the game, but outside the
   core.
5. **Tactics:** the game removes the player from the old club's tactic (`tactics.dat`, id → −1) **[observed in (a)
   and (d)]**; advisable, so as not to leave an outside player in the tactic.
6. The rest (finances, news, `transfer.dat`, histories, `player.dat`, statistics, `man_conf.dat`, `+82`, `+94`,
   club +128) is what the game **also** writes; whether the editor imitates it is Lucas's call (TRANSFERENCIA §7
   item 9).

## Cross-check with C1–C9 (TRANSFERENCIA.md §4)

`verif.py` counts in `antes1` → each "after" (`coerencias.txt`) **[observed]**:

| Rule | Measured in "before" | Result | Status |
|---|---|---|---|
| C1 one slot per group, one club | 54 people in 2 clubs (base exceptions) | same in all; **(d) +1** (the loaned player in both squads) | **matches** (loan exception foreseen in §2.1) |
| C2 +57 = c → slot in c | 11 | same in all | **matches** |
| C3 +57 = −1 → no slot and no contract | 1 | same (the released player leaves the slot and the index) | **matches** |
| C4 index ↔ contracts | 0 errors, 0 free slots | 0 errors in all; **(b) creates 1 record with +0 = −1, not pointed to** | **partly diverges:** "every contract is pointed to" only holds for occupied records; the game leaves free records (+0 = −1) and reuses them. The checker has to accept +0 = −1 |
| C5 contract +4 = +57 | 186 (B clubs, loanees) | same in all | **matches**; in (b) the free record keeps the old +4 (irrelevant, +0 = −1) |
| C6 +70 = contract +45 | 0 | 0 in all | **matches** (the game rewrites +70 on entry) |
| C7 lineup ⊂ squad | 0 | 0 | **matches** (empty lineup in this save) |
| C8 no repeats in a list | 0 | 0 | **matches** |
| C9 minimal difference | — | the game changes 6 to 15 items (table above) | **diverges by definition**: the game's "after" is not minimal; the editor must be compared with the pair only on the core (TRANSFERENCIA §6 step 8) |

Answers to TRANSFERENCIA §7 **[observed]**: (1) enters the 1st free slot; no compaction. (2) with a contract: same
record rewritten, same index, same size; without a contract: new record at the end (or in the 1st free slot);
release: record marked free, not deleted. (3) +62 = today; +70 = new +45; +37 = today (this path has no earlier
agreement date); +45 = today + years. (4) released player: +61, +78, +82 stay. (6) loanee: only +57, `general +4`
and the loan record (type 0x46); the owner keeps the slot. **Diverges from §1.1:** +78 (wage) **is** rewritten with
the contract wage on entry; +94 changes (2 → 1) when a player joins; club +128 (i16) changes by ±1. **Diverges from
§1.3:** type 0x42 (bit 0x40) shows up on **new** contracts (c, e) and on the sale one (b2), not only on the owner's
contract of a loanee; and the owner's contract did **not** get bit 0x40 with the loan (d).

## Limits

- Cases (a), (b2), (c) and (e) use the **pre-arranged** transfer path (0x8bf060 + 0x8cbbe0), not the multi-day screen
  negotiation (offer → club response → contract → confirmation); the end is the same (`finalise_transfer`), but
  agreement dates, clauses and the pending offer may differ from a purchase made on screen **[inference]**. The loan
  (d) is the configuration file one (0 % wage).
- Case (c) changes, only during the call, the hard-coded job of 0x8bf060 (11 → 9) in three code immediates.
- The "fee" of (a) and (b2) is the player's value (+0x52), with no instalments or clauses.
- Not covered: assistant with an occupied slot (3b), player-manager, B/reserve clubs, a 50-man squad, a Bosman move of
  someone still under contract (0x8bf060 with fee 0 and a contract marks the old contract to end).
- `transfer.dat`, `news.dat`, `staff records.dat` and the histories were not decoded (only size and position).
