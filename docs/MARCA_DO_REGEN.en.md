# The regen mark: bit 0x40 of the 79-byte record

Investigation of 27/10/2026, read-only (disassembly of `cm0102.exe` 3.9.68 and reading of saves; the game was not run), requested for the editors' "Regens only" filter. Today they decide who is a regen by comparing the save with the `Data` database. That fails with namesakes and with the 51,353 people in the factory database whose date of birth is unknown, whose day is drawn when the career is created. Tags: [observed] = seen in the binary or in the save; [inference] = conclusion without direct proof.

## Short answer

1. **There is a deterministic mark.** It is **bit 0x40 of byte +0xb** of the person's 79-byte record (table `[0xAE2D0C]`, saved in `general.dat`). Only the routine that creates a player regen (`PLAYER_REGEN::regenerate_as_youth_player` 0x7abac0) sets this bit, and nothing in the game clears it [observed].
2. **The mark survives saving and loading.** The table goes whole into `general.dat` and comes back whole on load [observed in the code and in three saves of the same career].
3. **The day is drawn when the career is created**, not when the `.sav` is loaded: `database_player_initialisation` 0x535a80, called only by `create_game` 0x828110. The year is not drawn: it comes from the database's +0x18, which the routine then replaces with the age. That is why name + year identifies the database person (§3).
4. **It does not record it.** The regen takes the slot (the id) of the retiree from the queue. The player record and the nation come from the first entry of a pool of records, which is deleted when used. Nothing keeps "in whose place" (§4).
5. **He does inherit, but from two different predecessors.** From the owner of the player record (B, a player who retired) the regen inherits the PA, the positions, the feet, the nation and the profile of the technical and hidden attributes; the CA is drawn as a fraction of that PA. From the owner of the slot (A, usually a retired non-player) he inherits only the id, the personality of the person record and the preferences. A and B are the same person only when the queue is short (§8).

## 1. Bit 0x40

### Where it is

| Place | Address |
|---|---|
| Memory | `[0xAE2D0C] + 79 × id + 0xb`, bit 0x40 (count in `[0xAE2D10]` = `[0xAE2364]`) |
| Save | item `general.dat`: count (i32) at **+0xfa3**, 79-byte records from **+0xfa7**, i.e. the byte `0xfa7 + 79 × id + 0xb` |

[observed] The count at +0xfa3 equals the number of people in `staff.dat` (size ÷ 110) in all 6 saves read (`Valencia.sav`, `FeriasTesteSave.sav`, `Lucas.sav`, `Roteiro.sav`, `TesteCMRTE.sav`, `TesteTelas.sav`). It is the same table that `CAMPOS.md` calls "contract indices per person" and that `PARES_TRANSFERENCIA.md` reads at `0xfa7 + 79·p`. In the same byte, bits 0x30 are the retirement state (0x10 announced, 0x20 retired; `player_regen_get_retirement_status` 0x7a7eb0 returns `(byte >> 4) & 3`). The 0x0f nibble and bit 0x80 have other uses.

**The mark is not in `staff.dat`.** An editor that only reads the 110-byte record does not see it.

### Who sets it

The whole `.text` was scanned (capstone; every access to `[0xAE2D0C]` and the byte, word and dword writes that cover +0xb). There are only two writes that set the bit [observed]:

**a) Creating the player regen**, 0x7abac0. It only goes on if the person is retired (`byte & 0x30 ≠ 0`) and has no club. Otherwise it clears bits 0x30 and returns (0x7abadc..0x7abaf1).

```
7abb2e  mov byte [esi+0x5f], 2          ; classification: player
7abb38  mov [esi+0x61], eax              ; player record from the pool
7abb4b  lea eax, [ecx+edx+0xb]           ; [0xAE2D0C] + 79·id + 0xb
7abb4f  mov cl, [ecx+edx+0xb]
7abb53  or  cl, 0x40                     ; <- the mark
7abb56  mov [eax], cl
```

Afterwards the same routine does `and 0x7f` (0x7abba2), `and 0xf0` (0x7abc94) and `and 0xcf` (0x7abe82). None of these masks clears 0x40.

**b) The `"ARSE"` line of `player_setup.cfg`**, in `player_setup_apply_line` 0x7a3a10: type 0x20 (0x7a324f..0x7a3261) does `or [record+0xb], bl` with `bl = 0x40` (0x7a3ad2) **on every person** (loop 0x7a3c0c..0x7a3c22). `create_game` reads the file with mask 0x203b, which includes 0x20 (0x828344). Neither the factory `player_setup.cfg` nor the CLZ one has this line [observed]. A modified database with it would mark everyone; the editor spots this case because the bit shows up on most people.

### Who does not clear it

- **Career creation:** each record starts with `+0xb = garbage & 0x80` (0x828548..0x828561), so bit 0x40 starts **cleared** on every person [observed].
- Every other write to the byte keeps 0x40: `and 0x7f` (0x4c7675, 0x4cf444, 0x7abba2), `or 0x80` (0x4c910e, 0x4cf091…), `and 0xcf` (0x7a5ae8, 0x7a6358, 0x7ab362…), `and 0xef / or 0x20` (0x7a63d7), `and 0xdf / or 0x10` (0x7a491d, 0x7a657c), `and 0xf0` (0x5361b1, 0x7ab396), `+1`/`−1` only in the 0x0f nibble (0x842728, 0x84402e) [observed].
- **Caveat:** the scan finds writes up to ~20 instructions after a read of `[0xAE2D0C]`. A pointer to the record held for longer would escape it. The saves (below) show no such case.

### Who reads it

`club_screen_draw_info_tab` 0x4617bf, `write_coach_report_news_text` 0x488207, `player_search_assess_player_traits` 0x7bdc3a, `apply_obscene_gesture_ban` 0x7db007 and `staff_screens_draw_profile_tab` 0x87b117/0x87b854 test the bit [observed]. That it means "young player generated by the game" is an **inference**: the bit's only normal source is the regen.

### Check in the saves

Scripts in `%TEMP%\marca` (read-only). "Matches the database" = same first, second and common name (as text) and same date (day of year and year). In CLZ the year is shifted by +24.

| Save (database) | People | With the bit | +0x5f classification of the marked | Marked that match the database | Namesakes only | Name not in the database |
|---|---|---|---|---|---|---|
| `clz_teste\Valencia.sav` (CLZ, 2028) | 62,487 | 3,857 | all 2, all with a player record | **0** | 419 | 3,438 |
| `jogo\TesteTelas.sav` (factory, 2002) | 158,727 | 492 | all 2 | **0** | 31 | 461 |
| `jogo\TesteCMRTE.sav` / `Roteiro.sav` | 158,727 | 13 / 1 | all 2 | — | — | — |
| `jogo\FeriasTesteSave.sav`, `Lucas.sav` | 54,148 / 42,314 | 0 | — | — | — | — |

- In `Valencia.sav`, 49,808 people without the bit match name + date with the database, and no marked one does. The 419 "namesakes only" are exactly the case that fools the editors today. The bit settles it without looking at the database.
- Regen ages at creation (+0x18 in `TesteTelas`): 15 to 27, mostly 16 to 19, as the routine draws them.

## 2. The mark on saving and loading

- **Saving:** `save_game_data` writes the whole table: `zwrite([0xAE2D0C], 0x4f, [0xAE2D10])` (0x82e934..0x82e944) [observed].
- **Loading:** `load_game_data` 0x82a9c0 allocates 79 × N (0x82af53..0x82af6d). In the current save version (`|[0xDD7110]|` = 0x3e047 ≥ 0x3e045), it reads each person's 79 bytes as they are (`zread(…, 0x4f, N)`, 0x82b634..0x82b643). No later step touches the bit. Only versions < 0x3e018 clear 0x80 (0x82b682). Saves from old versions convert field by field and also copy 0x40 (0x82b05e..0x82b069) [observed].
- **In practice:** `Roteiro.sav` → `TesteCMRTE.sav` → `TesteTelas.sav` (saved on 15/10, 19/10 and 26/10, same 158,727 people) have 1 → 13 → 492 marked. Every id marked in one save is still marked in the next, with the same name and birth date (1 of 1, 13 of 13) [observed]. That they are the same career is an inference from the same regens, whose names and dates are drawn.

## 3. Drawing the unknown birth date

**When:** when the career is created. `database_player_initialisation` 0x535a80 has a single caller: `create_game` at 0x828894 [observed]. Loading a `.sav` draws nothing: the dates come ready in the save's `staff.dat`.

**The database:** the factory one has 51,353 people with birth date `(day 31, year 1900, +0x14 = 1)`. In them +0x18 holds the real year in 26,327 cases (> 1900) and 0 or ≤ 1900 in 25,026. The CLZ database has 1,812 such people, with +0x14 = 0 [observed].

**What 0x535a80 does** [observed]:

- **Person with a player record** (0x536836..0x53689c): if the age (byte of the current year − byte of the year at +0x12) is < 14 or > 50, which always holds for 1900, the birth date becomes `date_set(day = rand(27)+1, month = rand(11), year)`:
  - year = +0x18 if +0x18 > 1900;
  - otherwise 1967 (if there is a non-player record) or 1980.
- **Non-player only** (0x537c71..0x537cbe): if the age is < 29 or > 75, the year = +0x18 if 1900 < +0x18 < 1967; otherwise 1940 + rand(26).
- `date_set` 0x54a130 writes the day of year at +0x10 and the year at +0x12, and **recomputes +0x14** (leap). The database's "1" is gone.
- At the end of each person, **+0x18 gets the age**. The year the database kept there is gone from memory and from the save.

**What is left to recognise the database person** [observed]:

- The drawn month and day always fall between day 1 and day 27 and between January and November. The draw never gives December or days 28 to 31.
- The year is not drawn when the database had +0x18.

In `TesteTelas.sav`, of the 35,185 people with a unique name in the database and an unknown birth date:

| Rule | People |
|---|---|
| year = database +0x18 | 16,785 |
| year 1967 or 1980 (player without +0x18) | 7,932 |
| year 1940..1965 (non-player) | 10,293 |
| Total by the rules | 35,010 (99.5%) |
| Day ≤ 27 and month ≤ November | all but 5 |

The rest (175) was not investigated [inference: namesake swapped by the database cut, or slot already reused]. The routine leaves no other mark on these people.

## 4. "In whose place"

- The regen queue has 12-byte entries (person id, date), created by `queue_staff_for_regen` 0x7a87d0 when a retiree does not carry on as a non-player. On the date, `process_regen_queue` 0x7a65d0 calls `0x7abac0(people + id × 0x6e)` (0x7a664f). The regen **is the same slot, the same id as the retiree**: the routine overwrites names, birth date, nation, record and club [observed].
- The player record and the nation come from the **first entry of the record pool** (`this+0x3c`, 16 bytes: record id, nation, second nation, and in 0x7a87d0 the owner's id). The entry is deleted right away (`remove_player_data_from_pool` 0x7a8470, called at 0x7abb1a). The record's owner may be a different retiree [observed].
- Nothing persistent links the regen to his predecessor. `retire.dat` only keeps the pending queues, and the entry leaves the queue when the regen is born (0x7a65d0) [observed].
- **For the editor:** within one save there is no way to know whose slot it was. Between two saves of the **same career**, the same id with a different name is the swap. Comparing with the database does not help: creation compacts the tables (`database_trim_staff_tables` 0x524d70), so save ids are not database ids. That also rules out the "id above the database count" clue: the regen has the retiree's id (in `Valencia.sav`, from 19 to 61,158).

## 5. Other people generated by the game (without the bit)

The bit marks only the **player** regen. The game makes three more groups, and none has a deterministic mark:

| Group | Routine | How it shows in the save |
|---|---|---|
| Non-player regen | `player_regen_regenerate_as_non_player` 0x7ab340: new names and birth date in the retiree's slot | +0x5f = 1, no bit |
| Extra staff at creation | `database_generate_extra_non_playing_staff` 0x527240 (0x527545…: +0x5f = 1) | non-players whose name is not in the database |
| Players made at creation | `database_trim_staff_tables` 0x524d70 (0x526385..0x52651f) | see below |

The players made at creation:

- take the ids right **before the 16 human managers**: `N − 17 − i`;
- names −2, drawn later in 0x535a80;
- birth year **1982** (0x7be, at 0x52638c/0x5263b6; in CLZ it shows as 2006 because of the +24 shift);
- job 0xb, +0x5e = 2, +0x5f = 2.

The total is 1% (size option 1 or 2: 2%) of the people kept (0x524dcf/0x524de1; ×0.01 or ×0.02, `__ftol`). Measured in the saves [observed]:

| Save | Ids | How many | Born in |
|---|---|---|---|
| `TesteTelas` | 157,140..158,710 | 1,571 | 1982 |
| `FeriasTesteSave` | 53,596..54,131 | 536 | 1982 |
| `Valencia` | 61,246..62,470 | 1,225 | 2006 |

In `TesteTelas` there is no player without the bit and without a name in the database outside that block. In `Valencia` there are 28, and by their names they look like real people spelled differently in the database [inference].

`+0x5f` is not a regen mark: it is the classification (0 human manager, 1 non-player, 2 player, 3 other). The type-6 reserve does not reach the saves (none of the 6 has a 6) [observed].

## 6. Recommendation for the editors

**Certain (from the code and checked in the saves):**

1. **Player regen** ⇔ bit 0x40 in `general.dat[0xfa7 + 79 × id + 0xb]`.
   - First check that `i32 general.dat[0xfa3]` = the number of people in `staff.dat` (size ÷ 110).
   - If the bit shows up on most people, the database used `"ARSE"` in `player_setup.cfg`: do not trust the filter, and warn.
   - No `Data` folder, names or dates needed. Namesakes and drawn birth dates stop mattering.
2. Live (game memory): the same bit at `[0xAE2D0C] + 79 × id + 0xb`.

**Strong heuristic (optional, for "made at creation"):** walk down from `N − 17` while the person is a player (+0x5f = 2) born in the made-up year (1982 in the factory database; 1982 + the shift in a database with shifted years) or marked with the bit. That stretch is the made-up block.
- The expected size is ~1% or 2% of the people.
- Over the years, retirements and regens erode the block: it is a heuristic, not a mark.

**Do not use as a criterion:**
- +0x5f (it is the classification);
- id range (regens reuse slots, and creation renumbers);
- +0x14 (inconsistent with the year in ~24 thousand database people as well);
- comparison with the database by name + date.

**If the editor still wants to match a person to the database** (for other purposes): use the name as text + year, and accept the drawn day/month when the database had `31/1900`. The year matches the database +0x18 when it is > 1900; otherwise it is 1967/1980 (player) or 1940..1965 (non-player). The day is between 1 and 27, and the month between January and November.

**Not marked:** non-player regens and the extra staff at creation. They have no mark. The "Regens only" filter should say that it covers only generated players.

## 7. When the regen entered (28/10/2026)

Requested for the "Joined" column of the regen list. Same method: disassembly and reading of saves, without running the game.

### What creation writes

`regenerate_as_youth_player` 0x7abac0 writes **one** date of the day: the club joining date [observed].

```
7abda8  mov ax, [0xae2c92]      ; current year
7abdb0  mov [esi+0x40], ax      ; club joining date: year
7abdb4  mov cx, [0xae2c90]      ; current day of year
7abdbd  mov [esi+0x3e], cx      ; club joining date: day
7abdc1  mov [esi+0x39], 0       ; no club yet
```

The rest of the routine writes no date of the day:

- The birth date is `+0x12 = current year − age` and `+0x10 = rand(365)` (0x7ac74e/0x7ac75d), with the age drawn between **15 and 27**.
- The 79-byte record only gets `+8 = 0`, `+9 = 0x9c` and `+0xd = 0`, which are not dates.
- When there is a club, `player_regen_sign_for_club` 0x7a6f10 puts the regen into a squad slot directly (0x7a6f6a). It does **not** call `database_add_staff_to_club` and does not touch +0x3e. It only writes the contract end (+0x46, 0x7a7061: current year + 2, or + 1..6 from age 19) and the contract (0x4c7950).
- So, as long as the regen has not changed club, +0x3e/+0x40 is **the day the game created him** [observed].

### When the date changes

The date goes into the save's `staff.dat` (+0x3e/+0x40, i16 day of year and i16 year) and comes back on load like any other person field. But several routines rewrite it [observed]:

- `database_add_staff_to_club` 0x53bd10 (0x53be14), when given a date: joining a club;
- `database_remove_staff_from_club` 0x53c190 (0x53c24c): leaving;
- `TRANSFER_MANAGER::free_transfer_staff` 0x8c1af0 (0x8c1d6c, writes the current day);
- `CONTRACT_MANAGER::sign_loan_contract` 0x4d01e0 (0x4d0431): loan;
- `MANAGER_MANAGER` 0x690472, 0x69b804 and 0x69f5d4: managers.

So after a transfer, a loan, a release, or the signing of a regen who had no club, the field becomes the **date he joined his current club**. A contract renewal does not touch it (below).

### Check in the saves

| Save | Regens | Joining year (+0x40) | No club |
|---|---|---|---|
| `Roteiro.sav` (today ≈ day 213 of 2001) | 1 | 2001 (day 204) | 1 |
| `TesteCMRTE.sav` (≈ day 293 of 2001) | 13 | all 2001, days 204 to 290 | 4 |
| `TesteTelas.sav` (saved again on 27/10 at 17:24, ≈ day 50 of 2002) | 533 | 27 in 2001, 506 in 2002 | 207 |
| `Valencia.sav` (CLZ, day 329 of 2028) | 3,857 | 431 in 2026, 1,514 in 2027, 1,912 in 2028 | 214 |

`TesteTelas.sav` changed during the investigation: it had 492 marked in §1 and 533 in this reading.

- **Every joining date falls after the career start** and does not go past "today" [observed]: 2001 onwards in the factory database, 2026 onwards in CLZ, which starts in 2025. Since the regen is born during the career and the rewrites use the current day, this holds by construction.
- **Roteiro → TesteCMRTE → TesteTelas sequence** (the 13 regens in common):
  - 2 changed club, both from "no club": ids 86141 (from 253/2001 to 348/2001) and 107241 (from 277/2001 to 340/2001). **The joining date changed in both.**
  - 11 did not change club, and the joining date stayed the same in all 11.
  - One of them (id 32095) had his contract end changed from 178/2003 to 160/2003, a renegotiation, and the joining date did **not** change.
- `Valencia.sav` has 118 regens with joining date day 0 of 2028 (1 January), a peak that fits transfers at the turn of the year [inference]. Those no longer show the creation date.

### Working out the season without the date

The creation age is drawn between 15 and 27, and the birth year is **exactly** `creation year − age` [observed; 0x7ac74e and the age-draw branches]. Hence:

- **creation year ∈ [birth + 15, birth + 27]**;
- and also **≤ joining year (+0x40)**, because every club change only moves the date forward;
- and **≥ the career start year**.

In the saves, `joining year − birth year` came out between 16 and 27; almost all fall between 16 and 20 (`Valencia`: 3,741 of 3,857). That is a distribution, not a rule: the birth date alone does not pin down the season [observed in the saves; using the most likely band is an inference].

Other possible sources, **not checked**: the first season of the player's history (`PlayerStats`) and the youth promotion news item (0xbce, 0x7a8ba0), which gets deleted along with old news.

### Recommended criterion for the column

1. **"Joined" = +0x3e/+0x40** (day of year and year), labelled "joined current club (or creation, if he never changed club)". It is the exact creation date as long as the regen has not changed club, and the editor cannot tell from a single save whether he did.
2. To stay honest, show the creation range next to it: **from max(career start, birth + 15) to min(+0x40, birth + 27)**.
3. If the editor keeps the career's saves (or a cache by id + name), use the **first** date seen for each regen. Between saves, the date only changes when the club changes; a renewal does not change it.

## 8. What the regen inherits (28/10/2026)

Lucas's question: "if the regen inherits nothing from the retiree, it makes no sense to link one to the other". Same method: disassembly and pseudocode of 0x7abac0 and of the queue and pool routines, plus reading of saves, without running the game. Scripts in `%TEMP%\marca` (`heranca.py`, `cruza_heranca.py`, `extra_heranca.py`, `b_depois.py`, `ficha_dividida.py`; read-only).

There are **two** predecessors, and "in the place of" mixes them up:

- **A, the owner of the slot:** the person at the head of the regen queue. The regen keeps his id (§4).
- **B, the owner of the record:** the player whose player record (`player.dat`, 70 bytes) is first in the pool. The regen keeps that record.

### How the queue and the pool are built

`PLAYER_REGEN::process_retirements` 0x7a59e0 handles each retiree along one of three paths [observed]:

| Who retires | Queue (12 bytes, `this+0x10`) | Pool of player records (16 bytes, `this+0x3c`) |
|---|---|---|
| Player who does not become a coach | goes in, through `queue_staff_for_regen` 0x7a87d0 (called at 0x7a64e2) | his record goes in: record id, nation (+0x1a), person id at +0xc (0x7a8ab0..0x7a8adc). +8 (second nation) is not written there. **His +0x61 is not cleared** |
| Player who becomes a coach (already has a non-player record, or gets one from the non-player pool by a draw on professionalism +0x5b, 0x7a5cdb..0x7a5d98) | does not go in | his record goes in, through `add_player_data_to_pool` 0x7a8350 (called at 0x7a6335: record, nation, second nation). Then `+0x61 = 0` and `+0x5f = 1`: he lives on as a non-player |
| Non-player | goes in, through 0x7a87d0 (0x7a64c9), or becomes a new non-player in the same slot (`regenerate_as_non_player` 0x7ab340) | nothing (the non-player record goes to the other pool, `this+0x1c`) |

- The queue date is today if the person is under 30, or today + 1 to 65 days (0x7a6397..0x7a63b4) [observed].
- The queue is sorted by date and then by id (`player_regen_compare_queue_entries` 0x7ae240, at the end of 0x7a59e0). The pool stays in arrival order; only 0x7a87d0 and 0x7a8350 add to it [observed; `refs.py`].
- `process_regen_queue` 0x7a65d0 goes on while the date at the head of the queue is before today (0x7a660b) **and** the pool is not empty (0x7a6618). For each head it calls 0x7abac0, which **always takes pool entry 0** (0x7abb0e..0x7abb16) and deletes it (0x7abb1a). There is no comparison at all between the slot and the record: not nation, not position, not level, not club [observed].

So:

- **B is always a player who retired**: either he is himself in the queue waiting for his turn, or he became a coach [observed in the code and in the saves].
- **A can be any retiree in the queue**, and with a backlog he is almost always a non-player. Non-players enter the queue without a record, so the pool becomes the bottleneck: each new record goes to the oldest slot [observed].
- **A = B only when the queue is short** and the head of the queue is the very player whose record is at the front of the pool [observed in the saves, below].

In the saves of the same career (§1):

| Pair of saves | Queue / pool in the first | New regens | B = A | A was a non-player | B active (not retiring) in the first |
|---|---|---|---|---|---|
| `Roteiro` → `TesteCMRTE` | 2 / 2 (the two records belong to the two people in the queue) | 12 | 2 | 10 | 10 (the other 2 are the B = A) |
| `TesteCMRTE` → `TesteTelas` | 847 / 0 | 520 | **0** | 518 | 520 (ages 25 to 50, 506 between 30 and 39) |

In `TesteCMRTE` the queue holds 847 waiting slots and the pool is empty. In `Valencia.sav` there are 26 slots and an empty pool [observed].

### What comes from B (the owner of the record)

0x7abac0 puts the pool record in +0x61 (0x7abb38) and **neither clears nor rebuilds it**. It rewrites only part of the record [observed: every write of the routine and of its callees]:

- **Drawn again:** CA (+5), the three reputations (+9, +0xb, +0xd), acceleration (+0x1b), agility (+0x1d), jumping (+0x2e), natural fitness (+0x34), pace (+0x36), stamina (+0x3c), strength (+0x3d) and work rate (+0x44).
- **Changed starting from B's value:** anticipation (+0x1e) and decisions (+0x24) lose a draw of up to 40, 20 or 10 depending on age; important matches (+0x2c) and leadership (+0x2f) lose a draw of up to 3 to 10; consistency (+0x21) only goes up, and only if it is < 5 in a regen aged 26 to 28.
- **Never written:** the PA (+7) is only read. Neither are the positions (+0xf..+0x1a), the feet (+0x30, +0x3b) and the other attributes.
- At the end, `staff_normalise_player_physical_attributes` 0x538ec0 and `staff_normalise_player_attributes` 0x538bc0 rebalance the attribute vector. They start from the values already in the record (`database_player_to_attribute_vector` 0x53a610 → `database_attribute_vector_to_player` 0x53aa10), that is, from B's.

**The CA comes from B's PA.** Example, in the branch for a nation with reputation < 350 (0x7abea0..0x7abffa): CA = PA/4 + draw(0.55·PA) from age 21, or PA/4 + draw(0.6·PA) up to age 20. The other branch (0x7ac3dd..) also uses only fractions of the PA (0.25, 0.55, 0.6…) [observed].

**The draw depends on B, not on A** [observed]:

- the age and the CA formulas depend on the nation (B's), its reputation (+0x88), the PA and the record's positions (goalkeeper +0xf, defender +0x11, attacker +0x15);
- names: from the nation, through `database_pick_first_name_for_nation` 0x539c60, 0x53a1b0 and 0x53a5b0;
- club: `pick_best_club_for_player` 0x7ad0e0 or `pick_club_from_nation_list` 0x7ad590, by the regen's nation and ability.

**Nation:** that of the pool entry, i.e. B's (0x7abcd4). If its reputation is < 100, there is a 1 in 4 chance of swapping it for a drawn nation, also with reputation < 100 (0x7abcd7..0x7abd1d). The second nation comes from the entry's +8:
- always, if it is the Basque Country;
- otherwise with a 1 in 3 chance;
- otherwise zero (0x7abd20..0x7abd6e).
- If the nation is Spain, 1 time in 100 the routine skips this step and +0x1e keeps A's second nation (0x7abd2c..0x7abd3f).

Checked on the 520 regens of `TesteCMRTE` → `TesteTelas`: B was found as the previous owner of the record for all of them [observed].

| Regen × B comparison | Equal |
|---|---|
| PA | 520 of 520 |
| 12 positions | 520 |
| left and right foot | 520 |
| nation | 520 (A's nation: only 14, by chance) |
| aggression, balance, bravery, dirtiness, flair, injury proneness | 520 each |
| versatility, consistency, important matches, handling, leadership, corners, free kicks, teamwork, technique | 259 to 454 each |
| other attributes | 10 to 63 each |
| CA | 14 |

- Mean difference per attribute (+0x1b..+0x44): **4.3** between the regen and B, against **14.0** between the regen and any record [observed].
- The pair `Roteiro` → `TesteCMRTE` shows the same in its 12 regens: PA, positions, feet and nation equal to B's; CA different; mean difference 4.2 against 15.6 [observed].

### What comes from A (the owner of the slot)

Besides the id, 0x7abac0 and its callees **do not write** these fields of the person record (110 bytes) [observed]:

- the personality +0x56..+0x5d (adaptability, ambition, determination, loyalty, pressure, professionalism, sportsmanship, temperament). Determination (+0x58) goes through the rebalancing in 0x53aa10, but starts from A's value;
- the preferences (+0x65: favourite and disliked clubs and people);
- +0x60 (ClubValuation).

A's adaptability still weighs on the draw of the regen's languages: the cut is (20 − adaptability) × 10 (0x7abded) [observed].

In the 520 regens:
- the personality without determination is A's in 514 [observed];
- determination, in 518 [observed];
- the preferences index, in 520 [observed];
- the 6 remaining differences were not investigated [inference: changes after creation].

The rest of A is cleared or replaced [observed]:

- written by 0x7abac0 itself: names, birth, age, nation and second nation, caps and goals for the national team (+0x22/+0x23), national team job (+0x24), club, wage, value and the non-player record (+0x69 = 0). The routine only goes on if A has no club (0x7abaec);
- cleared by the callees:

| What | Routine |
|---|---|
| statistics | `PlayerStats::reset_player` 0x7c6650 |
| history | 0x86fb80 |
| discipline | 0x55e1d0, 0x556830 |
| injuries | 0x631f90, 0x62bfa0 |
| contracts | 0x4d0b50, 0x4d0bd0 |
| national team | 0x775ba0 |
| fog of war | 0x5afd00 |
| nickname | 0x5fff30 |

**A's club, position and reputation play no part**: A already has no club, and his record, if he had one, went to the pool and may go to another regen [observed].

### The shared record in the save

Since 0x7a87d0 does not clear the +0x61 of the player who enters the queue, **while he waits for his turn the same record is pointed to by two people**: the retiree in the queue (B) and the regen already using it. When B's turn comes, 0x7abac0 replaces his +0x61 with the next record in the pool [observed].

| Save | Records with two owners | The other owner |
|---|---|---|
| `jogo\TesteTelas.sav` of 27/10 at 17:24 (SHA-256 `571a96aa…`; 533 regens, queue of 1,289) | 428 | retiree (bits 0x30 = 2), no club, in the queue |
| the Orchestrator's `TesteTelas-2026-10-27.sav`, from 17:21 (SHA-256 `89067368…`; 529 regens, queue of 1,290) | 426 | same |
| `Valencia.sav` | 24 | same |

The two `TesteTelas` are different saves of the same career; on the same file, the CMRTE count and this reading agree (426). In every save, every record with two owners belongs to a regen and to a retiree of that kind [observed].

**For the editor:** the screen of a retiree in the queue may show the regen's record. Editing that record changes both.

### Conclusion

- **There is a real ability link, yes, but with B, not with A.** The regen is the record of a player who retired (or became a coach), with the same PA, the same positions, the same feet and the same nation. The CA, age, names, physical attributes and part of the mental ones are drawn again, and the other attributes are rebalanced from B's [observed in the code and in the saves].
- **With A the link is the slot:** the id, the personality of the person record (including determination and adaptability, which weighs on the languages) and the preferences. Nothing of ability, position, nation or club [observed].
- **"So-and-so is the regen of Such-and-such" is true in B's sense**, the owner of the record: same potential, same player "as a youngster".
  - **In A's sense** (the id, the slot), it is a myth for ability and only holds for personality.
  - **The two only coincide when the queue is short.** In `TesteCMRTE` → `TesteTelas`, with 847 waiting slots, they coincided 0 times out of 520 [observed].
- **For the editor to link a regen to the "original":**
  - in a single save, B is the retiree in the queue who points to the same record (while he waits);
  - between two saves of the career, B is whoever held the record in the earlier save;
  - once B also becomes a regen, or if there is no earlier save, no trace is left [inference from the code: the pool entry is deleted when used, and B's +0x61 is replaced on his turn].
