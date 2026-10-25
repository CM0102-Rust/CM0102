# In-game check of suspension removal

25/10/2026. Lucas's request (via the orchestrator): "Suspension: CM0102 proves it in the lab, without my
screen". Prove that removing a suspension the way the editor (`CM0102-Editor/CMRTE`) will do it works in the
game: the byte model of the accepted appeal (`docs/MAPA_DISCIPLINA.en.md` §2) applied to the `discipline.dat` of
copies of `jogo/FeriasTesteSave.sav`, loaded in the original game. Template: `docs/CONFERENCIA_LESAO.en.md`.

Convention: **[observed]** = read in the file or in game memory; **[inference]** = conclusion from what was
observed and from the disassembly.

## Method

- **Base:** copy of `jogo/FeriasTesteSave.sav` in `%TEMP%\cm0102_suspensao\`. The original was untouched
  (MD5 `33fda465193b5e8e6d7ace93ecd44559` before and after) **[observed]**. Type 3 container (uncompressed),
  142 items; `discipline.dat` of 1,602,112 bytes, 10 rule bodies **[observed]**.
- **The two people, in the copy's `discipline.dat`** **[observed]**:
  - **51013**: slot 3 = (rule 0 = FIFA, entry 36); entry +6 = 33; ban 33 = 1 match, 0 served, start 113/2001,
    rules `[−1, −1, −1, 0]`, reason 2 (red card), bits 4 (bit 1 clear); pair `(51013, 3)` in the per-nation index
    (nation 3 is staff +0x1a).
  - **29220**: slot 3 = (rule 0, entry 0); entry +6 = 0; ban 0 = **time ban, 304 days**, start 0/2001, rules
    `[−1, −1, −1, 0]`, reason 3 (conduct), bits 6 (bit 1: applies in every competition); pair `(29220, 61)` in
    the index (staff +0x1a = 61).
  - No pending bans for either (the save's only pending one belongs to 46487, in rule 6).
- **Generator:** `engenharia-reversa/remover_suspensao.py` (new). Reads `discipline.dat` in the **file** layout
  (MAPA §1: N × 29 records; K rule bodies, each `i32 n` + entries + `i16 p` + pending; K found as the only one
  that closes the item; `i16 b` + bans; `i16 m` + per-nation index; `i16 h` + stored slots), checks the item's
  round trip, applies the accepted appeal and rewrites the container with `renomear_no_save.regravar`, which was
  extended to items of **different size** (the size in the index changes and the following positions shift,
  for types 3 and 4). The output is checked with `renomear_no_save.conferir` (all the other 141 items identical
  byte for byte; `discipline.dat` equal to the expected one). Rewriting without changes gives the identical file
  (the original MD5) **[observed]**. With no person given, the script lists active bans, pending bans and the
  index.
- **Load:** throwaway script `conferir_suspensao.py` (session scratchpad; template `conferir_lesao.py`):
  `prova_save.py` bench, test game (`jogo_testes\cm0102-1080.exe`), **original mode**, `-load <copy>` +
  `CM_FERIAS=1`, hidden 960x540 window, no clicks, inside the lab lock (waited for the other test to release it).
  A watcher reads memory every 0.5 s and records every change of the MAPA §4 chain for both people, `D+0xa`,
  `D+0x12`, the per-nation index, rule 6's pending list and the error box (`0xae24d0`). `game.cfg`,
  `hall_of_fame.bin`, `*.lng` and `*.tmp` saved and restored by the bench.
- **`is_player_banned` 0x55d270:** not called from outside (the 0x7e85b0 it uses calls `update_ban_progress`
  0x556e90, which changes state). The rule was **reproduced** from the memory read **[inference over
  observed]**: first FIFA (rule 0) with flag 1, which only applies if the ban has bit 1 of +0x13; then the
  competition's rule with flag 0. Active = today ≥ start and (match ban: served + 1 ≤ matches; time ban: days since
  start < days). In the table, "FIFA" = the result of the first step (what applies in any competition); "in rule 0"
  = the second step for a FIFA competition.

### Copies

| Copy | Change | `discipline.dat` | MD5 |
|---|---|---|---|
| (a) `a_controle.sav` | none (copy of the original) | 1,602,112 | `33fda465193b5e8e6d7ace93ecd44559` |
| (b) `b_51013.sav` | 51013 removed | 1,602,104 | `5fdea32000ca00db90f255fcb37694a3` |
| (c) `c_29220.sav` | 29220 removed | 1,602,104 | `7416c30f23e5fd49efbe0521f5b226f8` |
| (d) `d_as_duas.sav` | both removed | 1,602,096 | `0cad2b1fe6c29e541ca1117a08aea050` |

The copies' MD5s did not change with the loads **[observed]**.

### How the loads ended

All 4 loads opened with no box, started on **205/2001** and ran to the stop date (215/2001), 23 to 29 s each;
no error box **[observed]**. Career fingerprint at the end: (a) and (b) `6efcb5697e0ca2a5` (equal);
(c) `261b980f1a3f191a`; (d) `b670115a2578bbb4` **[observed]**.

## Result

Readings at load (205/2001) and on each day up to 214/2001: the 10 readings of each copy gave the same state for
both people **[observed]**.

| Copy | Observed in memory (load and days 206..214) | Expected | Match? |
|---|---|---|---|
| (a) control | `D+0xa` = 85, `D+0x12` = 10, index with `(51013, 3)` and `(29220, 61)`. 51013: slot 3 → rule 0 entry 36, +6 = 33, ban 33 = `01 00 ffff 7100 d107 … ff ff ff 00 02 0100 04` (1 match, start 113/2001) → active in rule 0 = **yes**, FIFA = no (no bit 1). 29220: entry 0, +6 = 0, ban 0 = `ff 00 3001 0000 d107 … 03 0000 06` (304 days from 0/2001) → active = **yes**, FIFA = **yes** | both still banned | **Yes** |
| (b) 51013 | 51013: slot 3 → rule 0 entry 36 (record kept), yellows 1 (kept), **+6 = −1**, out of the index; `D+0x12` = 9, index = control's without `(51013, 3)`, in order; `D+0xa` = 85. 29220 as in control (banned) | 51013 not banned; 29220 banned | **Yes** |
| (c) 29220 | 29220: entry 0 kept, **+6 = −1**, out of the index; is_player_banned = **no** (FIFA and rule 0); `D+0x12` = 9. 51013 as in control | 29220 not banned; 51013 banned | **Yes** |
| (d) both | both with +6 = −1 and out of the index; `D+0x12` = 8; `D+0xa` = 85 | neither banned | **Yes** |

Notes:

1. **The effect shows in the simulation** **[observed]**: (c) and (d) have a fingerprint different from control;
   (b) gives the same. **[inference]**: 29220 has bit 1 (banned in any competition) and, without the ban, becomes
   selectable again and changes the matches; 51013 was only banned in FIFA competitions (bit 1 clear) and there was
   no FIFA match for him between 205 and 215, so nothing changes.
2. The game accepted the smaller `discipline.dat` (8 or 16 bytes less) and the container with shifted positions,
   with no box **[observed]**.
3. The removal is stable: the game does not recreate the ban or the index pair in the 10 days; the freed ban slot
   no longer shows through the person **[observed]**. `D+0xa` (the high-water mark) stays at 85, as in the game
   **[observed]**. Whether a new ban reused the freed slot in those days was not read **[not checked]**.
4. The save's only pending ban (46487, rule 6) came in on day 206 in all 4 copies, identically **[observed]**.

## The byte model the editor should write

Yes: the model used for the copies is what the editor should write (the game's accepted appeal,
`apply_appeal_result` 0x557000 → 0x7e91d0 + `release_ban` 0x556d00). In `discipline.dat`, for person `s`:

1. For each slot k = 0..3 of record `s × 29` with rule `rs ≠ −1`: entry `idx` (i32 at `5k+1`) of rule `rs`; only
   if `0 ≤ idx < n` and entry +0 == `s`.
2. Remove from **that rule's** pending list the 0x70 nodes with `+0x1c == s` (and its `i16 p` decreases).
3. If entry +6 (`b`) ≠ −1: in ban `b`, for each slot j with `+0xc+j ≠ −1`, the entry of `s` in rule `+0xc+j`
   (through slot j of the record) gets **+6 = −1**; if j = 3 (FIFA), remove the pair `(s, nation)` from the
   per-nation index (nation = staff +0x1a, or +0x1e if the pair is for the second nationality) and keep the index
   sorted by (nation, person) (`i16 m` −1). Ban `b` becomes **`+0 = 0xff`, `+2 = 0xffff`**; the other 16 bytes
   stay; `i16 b` (the count) **does not change**.
4. Do not touch: the person's record (slots, history +0x14/+0x18, +0x1c), the entry's yellows and counters,
   `ban_history.tmp`.

What the editor needs to know:

- **File layout ≠ memory:** in the file, each rule carries its entries and pending bans together, in the order of
  the existing rules (non-null only, without an index: the rule's position is found through the slots pointing
  back, or by increasing order); counts are `i32 n` (entries) and `i16` (pending, bans, index, stored). Memory's
  `D+0x12` is the file's `i16 m`.
- **The item changes size** (−8 bytes per index pair, −0x70 per pending ban): the editor must rewrite the
  container index (the item's size and the positions of the following items), for type 3 and type 4. The game
  accepted it **[observed]**.
- The per-nation index must stay sorted (the game uses `bsearch`; a missing pair when the game looks for it opens
  the `discipline.cpp 0x47d` box, but the editor removes the pair together with the ban, so nobody looks for it).
- **Not handled here (for the editor to refuse or handle separately):** a person with stored slots (loan, record
  +0x1c > 0): the entry is reached through the stored node (MAPA §2, "Careful"); the script refuses.
- Removing only one competition's ban is a different case (MAPA §2); this was full removal.

For Lucas to check, with a screen (beyond this reading): the player's profile without "Suspended" and a match
line-up with 29220.
