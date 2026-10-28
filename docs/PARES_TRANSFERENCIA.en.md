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
| `club.dat[A]`, `[B]` | i16 at +128 | ±1 (A −1, B +1; varies with the player's reputation: see 2nd round) | — | (e) +2; (c) nothing | — |
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

## Second round: occupied slot, player-manager, B club and Bosman

26/10/2026, same method, same base and same game copy; the "before" is still `antes1.sav` (a 4th save with no
operation and the recompiled DLL, `antes4.sav`, came out **identical**, MD5 `35448cd3…`) **[observed]**. The
`FeriasTesteSave` was checked before and after each load and at the end: MD5 `33fda465…` untouched. Candidates chosen
by `ferramentas\analise2.py`; C1–C8 counts in `coerencias2.txt`; full differences in the `*.diff.txt` files.

The diagnostic DLL gained four operations (`S` and `D` became functions, doing the same as before): `R` (replacement:
`D` on the incumbent and `S` on the new one), `B` (Bosman pre-contract), `A` (move to the B team) and `K` (accepted
contract with a new job; not needed, `C` was enough).

### The pairs

| Case | "After" file | Bytes | MD5 | Items changed | Operation |
|---|---|---|---|---|---|
| (f1) assistant with the slot occupied | `f1_vaga_ocupada.sav` | 137,618,621 | `db35a7f2da67e97613ad6244ea5e180c` | 2 | assistant 487, no club, to 4504, which already has assistant 24253 |
| (f2) assistant replacement | `f2_substituicao.sav` | 137,618,924 | `95c7743e60b3317a2ebd87e0a0dc595f` | 10 | 24253 dismissed and 487 hired on the same day |
| (g1) player becomes player-coach | `g1_jogador_treinador.sav` | 137,618,532 | `354e33d9df980aca6363082fe4cb42ca` | 8 | player 3304 of 4504 renews with job 15 |
| (g2) player hired as assistant | `g2_jogador_auxiliar.sav` | 137,619,014 | `66e392706c099f700e21dd9b87c23eaa` | 12 | player 25549, no club, hired as assistant (6) by AFC Bournemouth (983, AI) |
| (h1) from the senior side to the B side | `h1_para_o_b.sav` | 137,618,548 | `45317be87f6e6461fbd3d5f145639361` | 2 | player 40851 from Real Madrid (4834) to Real Madrid B (4835) |
| (h2) signing by the B side | `h2_compra_pelo_b.sav` | 137,619,864 | `6d94b080f4121171f1a582af26f570e3` | 13 | player 1366, no club, signed by 4835 for no fee |
| (h3) sale of a B-side player | `h3_venda_do_b.sav` | 137,618,995 | `4e25861b219e3d3a20d3774c3f8c260d` | 15 | player 40740 (contract with 4834, in the 4835 squad) to 4504, fee 95,125 |
| (i1) Bosman pre-contract | `i1_precontrato.sav` | 137,618,740 | `ecbb07f7f6ff4a66b4fe17b9cb4a70a6` | 3 | player 26657 of Colchester (1451), contract until 272/2001, agrees to join 4504 on day 273/2001 |
| (i2) Bosman exit completed | `i2_bosman_saida.sav` | 137,619,038 | `e200bdd992e12f949656e97ab66ae430` | 15 | the same 26657 leaves for 4504, fee 0, still under contract |

### The routines and the arguments

| Case | Calls |
|---|---|
| (f1) | The 1st round `S`: 0x8bf060 (thiscall `TM`; person 487, club 4504, `&today`, 0, −1, 0) with the hard-coded job 11 changed to **6**, and 0x8cbbe0. |
| (f2) | `R`: the *Terminate Contract* steps of case (b) on incumbent 24253 (0x4d0b50, 0x53c190 (4504, 24253, 0, 1, 0), 0x8c1af0 (24253, 4504, 0, 0, 1), training 0x8b2880 (4504, 0)), then the same as (f1). |
| (g1) | `C`: `STAFF_CONTRACT` 0x85e410 + `set_to_desired_contract` 0x85fc50 (person 3304, club 4504, −1, 0, job **15**, 0, 0) + `add_contract_offer` 0x4cc490 (thiscall `CM`; offer, −1, −1, 15, 0, 0, 0) + `resolve_contract_offers` 0x4ce4e0 (thiscall `CM`; person). It is the *Offer Contract* screen renewal with the "Player/Coach" job; the player **accepted**, and `sign_contract` 0x4cf720 took the job-change branch (slot checked by 0x53b9a0, removed from the club by 0x53c190 and added again by 0x53bd10 with the new job) **[observed: disassembly and save]**. |
| (g2) | `S` with job **6** at an AI club. What turns it into 13 is `database_add_staff_to_club` 0x53bd10: non-human club, person with a player record (+0x61) and record +5 (i16) above 0x55 → job 6 becomes 13 (and 8 becomes 15), with a staff slot **and** a squad slot **[observed in the disassembly]**. |
| (h1) | `A`: `CONTRACT_MANAGER::move_to_reserves` 0x4cc0f0 (thiscall `CM`; person 40851, club 4834) → `toggle_squad_placement` 0x4cbd20. It is the squad action that moves the player to the linked B side. |
| (h2), (h3), (i2) | `T`: 0x8bf060 (person, buying club, `&today`, fee, −1, 0) + 0x8cbbe0, as in (a)/(e): fee 0 in (h2) and (i2), −1 (the player's value) in (h3). |
| (i1) | `B`: 0x8bf060 (thiscall `TM`; person 26657, club 4504, `&(contract end + 1 day)` = 273/2001, fee **0**, −1, 0), **without** 0x8cbbe0 (the date has not come yet). It is the date the game uses: `transfer_manager_get_offer_completion_date` 0x8cee70 gives, for type 3, the contract end + 1 day, and `confirm_offer_by_club` 0x8c3310 books the future transfer (`add_future_transfer` 0x8cb9a0) for that day **[observed in the disassembly]**. With fee 0, a current contract and a club nation under the Bosman rule (`nation_allows_free_transfers` 0x544670), 0x8bf060 sets contract +0x49 = 1. The agreement news (`post_transfer_agreed_news` 0x8e05d0) that the screen confirmation posts is not called. |

### What changed, item by item

**(f1) occupied slot: the game does not replace.** Only `news.dat` (1 news item, type 0xfb0, not the 0xfa8 of completed
transfers) and `transfer.dat` (+73) changed; `club.dat`, `staff.dat`, `contract.dat` and `general.dat` **identical**:
487 stays without a club and 24253 stays assistant **[observed]**. From the disassembly, `process_due_future_transfers`
calls `check_offer_restrictions` 0x8cfc90 → `transfer_manager_check_squad_limits` 0x8cd450, which refuses when the
offer's job is 6/13 and the club (or its linked B side) already has an assistant (reason 0x22/0x23: "too many staff
employed"); the offer is collapsed (`collapse_offer` 0x8c44e0) **[inference from the disassembly]**. The same holds for
full coaches (5), scouts (7) and physios (3); and `database_add_staff_to_club` returns 0 for an occupied single slot
(chairman, manager, assistant) without touching the incumbent. **No game path has the newcomer push the incumbent
out:** replacement is dismiss then hire, which is (f2).

**(f2) replacement (dismissal + hiring):**

| File | Field | Before → after |
|---|---|---|
| `club.dat[4504]` | assistant (+211) | 24253 → 487 (the same slot) |
| `staff.dat[24253]` (the incumbent) | +57 | 4504 → −1; **+61 (job 6), +62, +70, +78, +82 unchanged** |
| `staff.dat[487]` | +57; +62; +70 (+74 leap) | −1 → 4504; → 205/2001; → 204/2005 (+78 and +82 unchanged) |
| `general.dat` | 24253 +0; 487 +0, +8 | 18321 → −1; −1 → **18321**, +8 +1 |
| `contract.dat` | record 18321 | **reused**: it was 24253's, becomes 487's (club 4504, wage 450, 205/2001 → 204/2005, type 0x2); `n` unchanged (41,974) |
| `finance.dat[4504]` | +12, +20 | wage bill |
| `transfer.dat` | +151 | = 37 (past transfer record of the dismissed) + 114 (offer of the hired) |
| `training.dat` | 7 bytes | the new assistant in the human club's training |
| `news.dat`, `staff records.dat`, `staff history.tmp`, `contract.dat` tail (+111) | | as in the 1st round cases |

The contract record the dismissal marks free (+0 = −1) is the **first free one** the next hiring uses (`sign_contract`):
coherence rule 3 confirmed in practice **[observed]**.

**(g1) player becomes player-coach (same club):**

| File | Field | Before → after |
|---|---|---|
| `club.dat[4504]` | squad | **leaves slot 16 and enters slot 1** (the first free one): the game removes and re-adds; the squad is not compacted and the order changes |
| | coaches[2] (+423) | −1 → 3304 (the first free one) |
| `staff.dat[3304]` | +61 job | 11 → **15**; +62 (joined) **unchanged** (same club); +70 → 167/2007 (new end); +78 wage 650 → 1,094; +82 value recalculated; **+105 (i32) −1 → 5533** (not decoded) |
| `contract.dat` | record 2496 | the same, rewritten: +8 5,758, wage 1,094, start 205/2001, end 167/2007, +79 0x4a → 0x0a; size and `n` equal, no pending offer in the tail |
| `general.dat` | — | unchanged |
| `player.dat` | +69 | 10 → 20 |
| `finance.dat[4504]` | +0 balance, +12, +20, +156/+316 | signing bonus (= contract +8) and wage bill |
| `retire.dat` | −16 bytes | one record removed (the player had announced retirement) **[inference]** |
| `training.dat`, `news.dat` | | the new coach; 1 news item |

**(g2) free player hired as assistant by an AI club → player/assistant manager (13):**

| File | Field | Before → after |
|---|---|---|
| `club.dat[983]` | assistant (+211); squad[26] (+319) | −1 → 25549 in **both** slots (the squad one is the first free) |
| | i16 +128 | 2,999 → 3,051 |
| `staff.dat[25549]` | +57; +61; +62; +70; +78; +82 | −1 → 983; 11 → **13**; → 205/2001; → 204/2005; 10,000 → 684 (contract wage); 0 → 105,815 |
| `contract.dat` | | **new at the end** (41974; `n` +1), club 983, wage 684, release fee 250,000, type 0x2 |
| `general.dat` | +0, +8 | −1 → 41974; +1 |
| others | | `player.dat`, `player stats`, `staff records`, `finance.dat[983]`, `transfer.dat` +114, `man_conf.dat`, histories, tail +111 |

**(h1) senior side to B side (`move_to_reserves`): the smallest of all pairs.** Only 2 items:

| File | Field | Before → after |
|---|---|---|
| `club.dat[4834]` | squad[5] (+235) | 40851 → −1 (not compacted) |
| `club.dat[4835]` | squad[8] (+247) | −1 → 40851 (the first free one) |
| `staff.dat[40851]` | +57 | 4834 → **4835** |

The **contract does not change** (still with club 4834, same record, same type 0x2, no 0x40 bit), nor `general.dat`,
nor the person's dates and wage **[observed]**. It is exactly the pattern of the save's 186 "contract with the senior
side, person in the B side" cases (C5 186 → 187).

**(h2) signing by the B side:** core equal to (e) (free player): `club.dat[4835]` squad[8] −1 → 1366; +57 −1 → 4835;
+62/+70/+78/+82; contract **new at the end** (41974) **with club 4835** (the B side, not the senior one), wage 2,802,
+8 3,434; `general +0` −1 → 41974. But the money comes from the **senior side**: `finance.dat[4834]` +0 balance −3,434
and +156/+316 signing bonus (4835 has no change in `finance.dat`) **[observed]**. `club records.dat` grows 907 bytes.

**(h3) sale of a player in the B side (contract with the senior side):**

| File | Field | Before → after |
|---|---|---|
| `club.dat[4835]` | squad[3] | 40740 → −1 (leaves the **B side**, where he is) |
| `club.dat[4504]` | squad[1] | −1 → 40740 |
| `staff.dat[40740]` | +57, +62, +70, +78, +82 | 4835 → 4504; → 205/2001; → 204/2006; 0 → 1,216; recalculated |
| `contract.dat` | record 30537 | the same, rewritten: club **4834 → 4504**, wage 1,216, start 205/2001, end 204/2006, +79 0x6a → 0x20 |
| `general.dat` | +34 (previous club) | −1 → **4834** (the senior side, the contract's), not 4835 |
| `finance.dat` | 4834 (senior) receives; 4504 pays | 4834: +0 balance +95,125, +120/+280 = 95,125; 4504: +0, +12, +20, +112/+272 = 95,125, +156/+316 |

C5 186 → 185 (one "contract ≠ +57" case disappears). The fee and the "previous club" go to the contract owner.

**(i1) Bosman pre-contract (the agreement, before the date):** the player **does not leave** the club.

| File | Field | Before → after |
|---|---|---|
| `contract.dat` | record 19956, +73 (+0x49, "leaves under Bosman") | 0 → **1**; the rest of the record equal (end 272/2001, club 1451) |
| | tail | +111 (the contract offer with 4504, pending until the day) |
| `general.dat` | +8 | +1 |
| `transfer.dat` | +81 | the offer (type 3, "at contract end") and the future transfer for 273/2001 |

Nothing in `club.dat`, `staff.dat`, `finance.dat` **[observed]**. Contract byte +73 is `staff_contract_is_leaving_on_bosman`
(0x8857e0 reads +0x49) and the profile's "Leaving the club under the Bosman ruling on <date>".

**(i2) Bosman exit completed (contract still running, fee 0):** the same core as a purchase under contract (a):
`club.dat[1451]` squad[6] → −1, `club.dat[4504]` squad[1] → 26657, +128 −5/+5; +57 1451 → 4504; +62 → 205/2001;
+70 → 204/2005 (and the joined-date leap +66); +78 → 1,016; the **same record** 19956 rewritten (club 4504, wage 1,016,
+8 1,565, start 205/2001, end 204/2005, **+73 back to 0**); `general +34` = 1451, +38 = 1; `tactics.dat` drops the
player from 1451's tactic. **No money for 1451** (only 4504 pays the signing bonus, `finance.dat[4504]` +0/+156/+316
and the wage bill) **[observed]**. In the game the completion happens on day 273/2001 (contract already expired); here
it was on day 205, with the contract still running: the only difference is the date.

### Mandatory coherence (additions of this round) [inference from what was observed]

7. **Occupied single slot:** the editor must **not** push the incumbent out. The game refuses; replacement is dismissal
   (case b) + hiring. The dismissed incumbent gets +57 = −1, `general +0 = −1`, a free contract record, and **keeps**
   +61 (job), +62, +70, +78, +82 (confirms TRANSFERENCIA §3b, item 7; CMRTE zeroing the job is a bug).
8. **Player-manager/assistant/coach (12/13/15):** two slots at the **same** club (the staff one and the squad one). On a
   job change at the same club, the game removes the person from the squad and puts him back in the **first free
   slot** (the position changes); +62 unchanged; the contract is the same record rewritten.
9. **B club:** moving from the senior side to the B side (and back) touches **only** the two squad slots and +57; the
   contract stays with the senior side. Signing made by the B side: contract with the **B side** (+4 = B), money from
   the senior side. Sale of a B-side player: leaves the **B side** slot, the (senior side's) contract is rewritten for
   the buyer, the senior side receives the fee and becomes the previous club (`general +34`).
10. **Bosman:** the agreement writes only contract +73 = 1 (plus offer leftovers); the move on the day equals a transfer
    under contract with fee 0 (same record rewritten, +73 back to 0, no money to the old club).

### Cross-check with C1–C9 (2nd round)

`coerencias2.txt` **[observed]**: C1, C2, C3, C6, C7 and C8 **equal to the "before"** in all nine pairs (the
player/assistant of (g2) and the player-coach of (g1) are in two slots of the **same** club, which C1 accepts). C4:
`n` +1 in (g2) and (h2) (new contract at the end); (f2) reuses the free record, so 0 free records afterwards. C5: (h1)
+1 (186 → 187, the new "contract with the senior side, +57 in the B side" case) and (h3) −1 (185) — both **agree**
with the B club exception of §2.1. C9: the game changes 2 to 15 items; (h1) is the only pair whose "after" is
practically the core.

Answers to TRANSFERENCIA §7 **[observed]**: (4) replaced person: +61 **stays**, +78/+82 stay. (5) player-manager: staff
slot + squad slot at the same club; becoming a player-manager at the same club repositions the squad. (6) B club: see
coherence 9. **Diverges from §1.2/§2.1** on one point: a signing **by** the B side creates a contract with the B side
itself (+4 = B), so not every B-side player has a contract with the senior side. And the 1st round note on club +128
(i16) "±1" was too narrow: it varies with the player's reputation (0x8c9720 adds reputation/100 to the new club and
subtracts it from the old one; (i2) ±5, (g2) +52, (h2) +20).

## Limits

- Cases (a), (b2), (c) and (e) use the **pre-arranged** transfer path (0x8bf060 + 0x8cbbe0), not the multi-day screen
  negotiation (offer → club response → contract → confirmation); the end is the same (`finalise_transfer`), but
  agreement dates, clauses and the pending offer may differ from a purchase made on screen **[inference]**. The loan
  (d) is the configuration file one (0 % wage).
- Case (c) changes, only during the call, the hard-coded job of 0x8bf060 (11 → 9) in three code immediates.
- The "fee" of (a) and (b2) is the player's value (+0x52), with no instalments or clauses.
- 2nd round: (g2) and (h2)/(h3)/(i2) use the same pre-arranged path; (f1)/(f2) change the hard-coded job to 6; (i1)
  does not post the screen's agreement news; (i2) completes on the agreement day, not the day after the contract end.
  (g1) is the screen path (offer + the player's decision), which he accepted.
- Not covered: a 50-man squad; transfer of someone already player-manager (12/13/15) to another club; player-manager
  (12) by manager appointment (`MANAGER_MANAGER`); a B club whose senior side is **human**; reserves of English clubs
  (not separate clubs: only contract bit 0x40, `toggle_squad_placement` cases 1/2); the real arrival on day 273/2001.
- `transfer.dat`, `news.dat`, `staff records.dat` and the histories were not decoded (only size and position).

## Line-up: moving between the first team and the reserves (2026-10-28)

Request from CMRTE (via the orchestrator; Lucas's decision "(b)": remove from the line-up as the game does). Same bench,
with the diagnostic DLL's `L` trigger: `CM_TRANSFERIR="L SSSSSSS CCCCC NNNNNNNNN"`, N = times × 100000 + captain. In
all of them, the set-up through the tactics screen on the club's row (team 0): captain (0x897ee0 role 0), vice = the
first slot ≥ 0 that is not the captain (role 1), and the 5 set pieces with side 0 = captain and side 1 = vice
(0x897bb0). Then `times` calls of `toggle_squad_placement` 0x4cbd20 (thiscall `[0xae1050]`; person, person+0x39, &out,
as the screen at 0x888760). Differences in `pares_transferencia\escalacao_diffs.txt` (`ferramentas\escalacao.py`).

| Save | Operation | MD5 |
|---|---|---|
| `l_A.sav` | set-up (captain 3304, vice 18401) | `7742a9c6f368b68438b476e831185c53` |
| `l_B.sav` | set-up + 3304 down (placement 2 → 1) | `f604333055704f8df0f316c326b4c4f1` |
| `l_C.sav` | set-up + 3304 down and up | `ebdcb8dec8aa8c9f187778ad9b56f421` |
| `l_D3.sav` | set-up + 30530 down (slot 5, no role) | `2326106c1a24ef91d7cf0e47e5c9d638` |
| `l_D.sav` | set-up + 7923 up (reserve, in slot 3) | `fcbc9747c5882c718f1658841eb77ac5` |

**[observed]** Only `staff.dat` (the person's byte +0x5e: 0x01 first team ↔ 0x02 reserves) and `tactics.dat` change.
Going down only changes the **club's** row (team 0 = club id): the slot becomes −1; the captain leaves and the vice
moves up (vice −1); in each set piece side 1 moves up to side 0 (side 1 −1). Going up removes the player from the
manager's private row (team 1 = the reserves' line-up) and nothing comes back on the club's row. The row's +0x52 is not
touched. CMRTE reproduced the four cases byte for byte.

**Layout** **[observed]**: `tactics.dat` = n1 + n1 × 0x18e3, n2 + n2 × 0x18e3, n3 + n3 rows of 0x91, n4 + n4 × 0x28, 10
dwords (0x8995d0). Row: +0 word, +2 20 slots (ids), +0x52 a selected id, +0x56 captain, +0x5a vice, +0x5e set pieces
(side + type × 2) × 4, type 0..4, +0x86 11 bytes. A club's row = club id (also the computer's clubs, which have a
line-up and a captain); the human manager's = [0xae235c] + (people − manager id) − 1 (+ 15 for a national team), with
[0xae235c] = records of `club.dat` + `nat_club.dat` (7029 here). The row count varies (7077 in `FeriasTesteSave`, 7093
after loading and saving): do not derive [0xae235c] from it.
