# Injury check in the game (CM3SaveEditor saves)

25/10/2026. Lucas's request (via the orchestrator): load the 5 injury check saves of the CM3SaveEditor (SGE)
in the lab and compare them with the predictions of `D:\Sistemas\CM3SaveEditor\docs\LESAO.md` §3, so the
editor can enable "Heal" in the save. Player: Darren Anderton (Tottenham, staff 52037), injured in the
starting save (type 26, record 338).

Convention: **[observed]** = read in the game's memory; **[inference]** = conclusion from what was observed
and from the disassembly.

## Method

- The save proof bench (`engenharia-reversa/prova_save.py`, load): the test game (`jogo_testes\cm0102-1080.exe`),
  the simulation's **original mode** (`CM_ORIGINAL=1`), `-load <copy>` + `CM_FERIAS=1`, hidden 960x540 window,
  no clicks, inside the lab lock; `game.cfg`, `hall_of_fame.bin`, `*.lng` and `*.tmp` saved and put back (the
  bench confirmed `game.cfg` restored in all 6 loads). The test game's DLL was not changed.
- A watcher (modelled on `renomear_no_save.py --carregar`) reads memory every 0.5 s without clicking and
  records every change of: the game date (`0xae2c90`), the 31-byte block of staff 52037
  (`[[0xb63d28]] + 31 × 52037`) and the 48-byte record pointed to by block+0x0E, plus record 338 (the injury's
  record in the starting save), and the error box (`0xae24d0`). The "computed state" is the rule of 0x62c400
  applied to the fields read (days since +0x20 against +0x2A/+0x28; record 0xFF = 0) **[inference]**.
- Throwaway script (outside the repository): `conferir_lesao.py` in the session scratchpad.
- Person 52037's identity checks out by the values: in the control, the block and record read in memory are
  exactly those of `LESAO.md` §3 (block dated 212/2001, +8 = 6613, +0xC = −273, +0xE = 338, +0x12/+0x13 = 26/1,
  +0x1D = 26, +0x1E = 50; record 338: type 26, start 192/2001, +0x28 = 99, +0x2A = 59) **[observed]**.
  (Reading the name through the name table came out empty; I did not pursue it.)

### Copies used (in `%TEMP%\cm0102_lesao\`, MD5 equal to the original before and after each load)

| Save | MD5 |
|---|---|
| `Testes Claude 2 - gravado pelo editor original.sav` (control, unedited starting save; from `CM3SaveEditor\jogo\saves\`) | `5f2ff69e646223747fe511c40dbee5eb` |
| `Lesao A_cmexplorer.sav` | `b435e0ccf7e74fedb35ab452eb23dd14` |
| `Lesao B_cmrte.sav` | `8ba221ede5985b26b2334f5b5f17ae8f` |
| `Lesao C_os_dois.sav` | `bb7be58037be7c572f84940b3aee6856` |
| `Lesao D_so_o_grau.sav` | `bd8e4d15e954cfe436061470bb316962` |
| `Lesao E_proposta.sav` | `b34eee8e1e34bc7bd0402e0eaf9a5098` |

### How the loads ended

All 6 loads (control included) loaded without a box, ran from 213/2001 (1 August) to 217/2001 and **all
stopped on the same date** (Monday 6 August 2001, evening) at the box
`v3.9.68 player_stats..cpp 3828 (PLAYER_STATS::create_stats() : Unable to allocat…` **[observed]**. Since the
unedited save stops the same way, the box is a limit of the bench with this large save (158,727 people,
240 MB) in simulation mode, not an effect of the edits **[inference]**. The 4 game days are enough for the
load, day 1 and a few days after.

## Result

| Save | Observed in memory | SGE prediction (`LESAO.md` §3) | Match? |
|---|---|---|---|
| Control (unedited) | On load, block dated 212/2001, +0xE = 338. At the 1st update (day 213) and every following day: still +0xE = 338, record 338 type 26, computed state 2 (out); +0x2C/+0x2E drop 1 a day (78/38 → 73/33); condition 66% → 65 → 64 → 63 → 62% | State 2, "Out for ~38 days", 66% | **Yes** |
| A (zeroes +0x28/+0x2A) | On load nothing changes (block from yesterday, +0xE = 338). At the 1st update, still on day 213: **the game heals him**: +0xE = −1, record 338 = 0xFF (free), **history created** (+0x14 = +0x18 = 1522416, were −1), +0x12/+0x13 = 26/1 kept; condition 66% → 72% and rising (94% on day 214, 100% on 215). On day 214 record 338 was already **reused** by another injury (type 31, start 214) | Healed by the game itself at the 1st query (block from yesterday), with history; record freed; recurrence stays; condition "near 66%" | **Yes** (the condition rises from 72% instead of staying at ~66%; the recovery news was not seen: no screen) |
| B (block like CMRTE) | On load and after: +0xE = −1, condition 100% (+8 = 10000), +0xC = −185 (then −198, −211…), +0x12/+0x13 = −1/0, +0x1D = −1. **Record 338 stays type 26 (occupied) with no owner**, decremented every day (+0x2C 78 → 73), never reused in the 4 days | No injury, ~100%, no history; record 338 orphaned; recurrence erased | **Yes** |
| C (A + B) | Same as B; orphan record 338 with +0x28/+0x2A = 0, type 26, decremented every day | Same as B | **Yes** |
| D (only +0x12 = 0xFF) | Same as the control in everything except +0x12 = −1: +0xE = 338, computed state 2, condition 66% → 62% | **Still injured** (~38 days) | **Yes** |
| E (proposal: record+0 = 0xFF, +0xE = −1, condition 10000) | On load: +0xE = −1, condition 100%, record 338 = 0xFF; +0x12/+0x13 = 26/1, history −1 (no node). **Right at load (day 213) record 338 was reused** by another injury (type 31, start 213); Anderton stays at +0xE = −1 and 100% for the 4 days | No injury, clean, "occupied ⇔ pointed to" invariant kept | **Yes** |

Notes:

1. Anderton's block is updated (block+0 = game date) on the load day and then every day, in all 6 saves
   **[observed]**: the "lazy" 0x62c680 runs early for him (some daily routine queries him).
2. In E and A the freed record goes back to the allocation pool and is reused within 0 or 1 day (allocation
   takes the first 0xFF) **[observed]**. The owner of the type 31 injury was not identified; it is not
   Anderton (his +0xE is −1) **[observed]**.
3. In B/C the orphan record had no visible effect in the 4 days **[observed]** (no box); it is a state the game
   never produces and a record lost forever **[inference]**.

## Recommendation for the editor's "Heal"

Use form **E** (the proposal of `LESAO.md` §4), which reproduces the game's cure `0x62e700` without the
history and without the news:

- 48-byte record pointed to by block+0x0E (only if 0 ≤ index < count): **+0x00 = 0xFF**;
- 31-byte block: **+0x0E..+0x11 = FF FF FF FF**;
- optional: condition **+0x08 = 10000** (and +0x0A = 10000 if it is not);
- do not touch +0x0C, +0x12/+0x13 (recurrence), +0x14/+0x18 (history), +0x1C..+0x1E, nor the record's
  dates/durations.

In the game it gave the same state the game's own cure leaves (compared with A, where the game healed him):
+0xE = −1, record free and reused, recurrence kept, nothing orphaned **[observed]**. The only difference from
the game's cure is that it does not write the history node (A got node 1522416); an injury healed by the
editor does not appear in the player's history **[inference]**. If the editor wants the history and the
news, the alternative is A (zero +0x28/+0x2A and let the game heal at the 1st update), which also worked, but
it depends on the block's date differing from the game date and brings the condition up by the formula
(72% → 100% in 2 days), not at once. **Not B/C** (orphan record, recurrence and history erased); **D does not
heal**.

For Lucas to check, with the screen (beyond the reach of this memory read): the profile ("None") and
Tottenham's injury list in E; the recovery news in A.
