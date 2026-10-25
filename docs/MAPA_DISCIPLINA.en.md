# Discipline map (suspensions)

Survey so that the `CM0102-Editor/CMRTE` editor can offer "remove suspension". Read only: disassembly and pseudocode (Ghidra) of `cm0102.exe` 3.9.68, the community map (`referencias/cm0102-function-map`, `discipline.cpp` and `ruling_body.cpp`), the Rust copies from batches 176, 184 and 191 (`rust/src/motor.rs`), and a read of `discipline.dat` from a **copy** of `jogo/FeriasTesteSave.sav` (MD5 33fda465193b5e8e6d7ace93ecd44559, untouched) using the reader functions in `engenharia-reversa/prova_save.py`. The game was not run.

Each claim is tagged **[observed]** (read in the disassembly, the code or the file) or **[inference]** (deduced without direct proof). Addresses are VAs. "Person" = index into the person table (`[0xae23bc] + id * 0x6e`; a player is a person).

Correction to the brief: the subsystem **constructor** is 0x5554a0; 0x555dd0 is `save` (writes `discipline.dat`) and 0x555860 is `load` [observed]. `MAPA_SAVE.md` §2 lists 0x555dd0 in the table of save routines, which is correct there.

## 1. The structures

### The `Discipline` object (global pointer `[0xae28f0]`)

| Off. | Type | Field | Source |
|---|---|---|---|
| +0x00 | `RulingBody**` | table of 43 pointers, one per **discipline rule set** (ruling body: FIFA, confederations, national FAs); null when that body was not created | [observed] 0x55d420, 0x556240 |
| +0x04 | `BanRecord*` | array of **bans** (20 bytes each) | [observed] 0x556940, 0x556d00 |
| +0x08 | `i16` | array capacity (grows by 100, cap 32000) | [observed] 0x556940 |
| +0x0a | `i16` | used count (high-water mark: it never shrinks; a free slot is `+0 == 0xff` and `+2 == 0xffff`, reused by the next ban) | [observed] 0x556940, 0x556d00 |
| +0x0c | `{i32 person, i32 nation}*` | **nation index** of the FIFA bans (8 bytes each, sorted by nation then person, comparator 0x55e5f0) | [observed] 0x556940, 0x556d00, 0x557120 |
| +0x10 | `i16` | index capacity (grows by 20) | [observed] |
| +0x12 | `i16` | index count | [observed] |
| +0x14 | `u8[N][29]` | **per-person discipline record**, N = `[0xae2364]` (number of persons) | [observed] 0x5554a0, 0x55cb20 |
| +0x18 | `FILE_LLIST*` | `ban_history.tmp` (history of served bans) | [observed] 0x5554a0 |
| +0x1c | list (0xe bytes) | **held bans** of loaned players (10-byte nodes) | [observed] 0x5563f0, 0x556570, 0x55cb60 |

The game's list (0x68d020 and neighbours) has `+0 i16 count`, `+2 first node`, `+6 last node`, `+0xa cursor`; each node has `+0 data pointer`, `+4 i16 size`, `+6 next`, `+0xa previous`; `0x68d1b0` advances the cursor and returns the data, `0x68d1f0` removes the node whose data matches byte for byte [observed].

### The per-person record (29 bytes, at `[D+0x14] + id * 29`)

| Off. | Type | Field | Source |
|---|---|---|---|
| +0 + 5k (k = 0..3) | `i8` | **slot k**: the rule set (index 0..42 into table +0x00); −1 = free slot. This is the "type" of `disciplina_gravar_vaga` | [observed] 0x55cb20, 0x55cae0 |
| +1 + 5k | `i32` | the person's index in that rule set's entry array (`[body+0xb]`, 14 bytes per entry). This is the "value" | [observed] 0x7e82e0 |
| +0x14 | `i32` | first history node in `ban_history.tmp` (−1 = none) | [observed] 0x556e90, 0x58e340 |
| +0x18 | `i32` | last history node (−1 = none) | [observed] |
| +0x1c | `i8` | how many slots are held in list +0x1c (loan) | [observed] 0x5563f0, 0x55cb60 |

A person with nothing: the first 28 bytes are 0xff and +0x1c = 0 [observed] 0x5554a0, 0x556830.

Which slot each rule set uses is fixed in the rule set (`body+4`): FIFA (rule set 0) uses slot 3; the confederations (rule sets 1 and 2) slot 2; the national FAs slot 0 (some slot 1) [observed in constructors 0x7df180, 0x7dd400, 0x7e29e0, 0x7db100, 0x7dc3c0, 0x7e01b0; "national = 0 or 1" holds for the ones that could be read, inference for the rest]. In the test save: rule set 0 always in slot 3, 1 and 2 in slot 2, 6 and 42 in slot 0, 7 in slot 1 [observed].

### The rule set (`RulingBody`, 0x23 bytes, `[[D] + 4 * ruleset]`)

| Off. | Type | Field | Source |
|---|---|---|---|
| +0x00 | vtable | 43 classes (0x96e9f4..0x96ef34, base 0x96f104): +0 new season (0x7e7ed0; FIFA 0x7df3c0), +4 daily (0x7e7d50), +8 player incident in a match (creates the ban; specific to each rule set), +0x10 scope text, +0x14 "one booking from a ban", +0x1c save extra (`ret 4` in all of them: nothing more) | [observed] |
| +0x04 | `u8` | the slot of the person record this rule set uses | [observed] |
| +0x05 | `u8` | this rule set's index (0..42) | [observed] |
| +0x07 | list* | **pending**: decided bans that only take effect on a date (0x70-byte nodes) | [observed] 0x7e7d50, 0x7e91d0 |
| +0x0b | `Entry*` | per-person entry array (14 bytes each) | [observed] |
| +0x0f | `i32` | capacity | [observed] |
| +0x13 | `i32` | count | [observed] |
| +0x17 | `i32` | nation id of the rule set (−1 if not national) | [observed] 0x556270, constructors |
| +0x1b | `i32` | confederation id (−1 for FIFA) | [observed] |

A competition points to its rule set in byte **`+0x50`** (`[[0xadadfc] + 4 * comp_id] + 0x50`, 0xff = none) [observed] 0x556210, 0x55ca00, 0x55d270.

### The person's entry in the rule set (14 bytes, `[body+0xb] + index * 14`)

| Off. | Type | Field | Source |
|---|---|---|---|
| +0x0 | `i32` | person id (−1 = free entry) | [observed] 0x7e82e0, 0x7e7ed0 |
| +0x4 | `u8` | yellow cards accumulated this season (rule set 0x96ec74 warns "one booking away" at 4, 7, 10, 13, 16...) | [observed] 0x7df910 |
| +0x5 | `u8` | another season counter (reds or bans) | [inference]; zeroed at the new season [observed] |
| +0x6 | `i16` | **ban index** into array `[D+4]`; −1 = no ban in this rule set | [observed] 0x7e8590, 0x7e85b0 |
| +0x8 | `i16` | season counter | [inference]; zeroed at the new season [observed] |
| +0xa | 4 bytes | flags (bits 0..5 cleared at the new season) | [observed] 0x7e7ed0 |

### The ban (`BanRecord`, 20 bytes, `[D+4] + index * 20`)

| Off. | Type | Field | Source |
|---|---|---|---|
| +0x00 | `i8` | matches banned; −1 = time ban | [observed] 0x556e90, 0x7e85b0 |
| +0x01 | `i8` | matches already served | [observed] |
| +0x02 | `i16` | days banned (−1 = match ban) | [observed] |
| +0x04 | `TCMDate` (8) | start date (+4 day of year, +6 year, +8 leap flag) | [observed] 0x556c70, 0x7df789 |
| +0x0c | `i8[4]` | the rule set that applies in each slot (index = the body's slot; −1 = does not apply); +0xf ≠ −1 = FIFA ban, goes into the nation index | [observed] 0x556940, 0x556d00 and in the save |
| +0x10 | `u8` | reason: 0 points, 1 yellows, 2 red card(s), 3 misconduct, 4 successive yellows, 5 doping, 6 invalid passport, other = unprofessional behaviour | [observed] 0x55d0a0 |
| +0x11 | `i16` | amount for the reason (points, cards) | [observed] 0x55d0a0 |
| +0x13 | `u8` | bits: 0 = merged into an existing ban; 1 = applies in all competitions (checked against FIFA in `is_player_banned`); 2 = see note | bits 0 and 1 [observed] 0x556940, 0x55d270; bit 2 [inference] |

Free: `+0 = 0xff` and `+2 = 0xffff` [observed] 0x556d00.

In force today (`is_banned` 0x7e85b0): today (`[0xae2c90]` day, `[0xae2c92]` year) ≥ start **and** (match ban: `served + a ≤ matches`, with `a = 1` in the team-selection check; time ban: days since start < `+2`) [observed]. Before the start date the ban is already in the array but does not stop the player [observed].

### The pending node (0x70 bytes in list `body+7`)

`+0x00` a whole `BanRecord`; `+0x14` `TCMDate` it takes effect; `+0x1c` person id; `+0x20..` match data for the news item; `+0x6f` flag [observed 0x7e7d50; the fields after +0x20, inference]. Each day (vtable `+4`, 0x7e7d50), every pending node dated ≤ today becomes a ban (`add_ban` 0x556940) or is added to the existing one, posts the news item (0x557280, type 0x7d2) and leaves the list [observed].

The "in 14 days" start from the brief (0x7DD212, 0x7DF766, 0x7DB019) does **not** go through the pending list: the incident creates the ban directly with `add_ban` and start = today + 14 (`0x54c5b0`), reason 2, the FA's own rule set in slot 0 [observed at 0x7df755..0x7df7fb].

### `discipline.dat` (in the save)

Written by 0x555dd0 and read by 0x555860 (container item `discipline.dat`, id 0x16/4), in this order [observed]:

1. `N × 29` bytes: the per-person records (N = `[0xae2364]`, which is `staff.dat` size / 110 [observed in the save: 54148]).
2. For each **non-null** rule set, in order 0..42 (`0x7e7b80`): `i32 n`, `n × 14` entries, `i16 p`, `p × 0x70` pending nodes. The file does not say which rule sets exist: that depends on the loaded nations (`[0xae23a8] + nation * 0x122 + 0x11c`, checked in 0x55d420). Reading from outside, try K = 3..43 rule sets and keep the K for which the rest fits exactly (in the test save only K = 10 fits, with every id < N) [observed].
3. `i16 b`, `b × 20` bans.
4. `i16 m`, `m × 8` nation index.
5. `i16 h`, `h × 10` held bans: `+0 i32 person`, `+4 i8 slot`, `+5` the copy of the slot's 5 bytes [observed 0x5563f0].

In the test save: 10 rule sets (0, 1, 2, 6, 7, four empty ones and 42; entries 88, 143, 106, 241, 1448, 0, 0, 0, 0, 107), 1 pending node, 85 bans (one free), 10 in the nation index, 0 held [observed]. Example: person 51013 has slot 3 = (rule set 0, entry 36); FIFA entry 36 has `+6 = 33`; ban 33 is a 1-match red-card ban starting on day 113 of 2001, rule sets `[−1, −1, −1, 0]`, and the nation index has `(51013, 3)` [observed].

## 2. How the game ends a ban

All of them go through **`Discipline::release_ban(person, index)` = 0x556d00** [observed]:

1. For each slot j = 0..3 with `ban[0xc + j] ≠ −1`: rule set `R = [[D] + 4 * ban[0xc+j]]` gets 0x7e8590 (`R->entry(person)`, found through the record's slot `R+4` and checked against `R+5`, and sets `entry+6 = −1`).
2. If j = 3 (FIFA): removes `(person, nation)` from index `[D+0xc]` (nation = `*(int*)[[0xae23bc] + person*0x6e + 0x1a]`, or the second nationality at `+0x1e`); the last element fills the hole, `[D+0x12]` −1, and it re-sorts (`qsort` 0x945333, comparator 0x55e5f0).
3. The ban becomes free: `+0 = 0xff`, `+2 = 0xffff`. `[D+0xa]` does not change.

The person's record (slot and index) and the entry (accumulated cards) **stay** [observed].

Callers:

| Path | Routine | What it does first | Source |
|---|---|---|---|
| served | `update_ban_progress` 0x556e90 | counts the match (`+1`) or checks the end date; when served, writes the history (`0x58e340`, `ban_history.tmp`, record +0x14/+0x18) and releases | [observed] |
| appeal upheld | `apply_appeal_result` 0x557000 | **0x7e91d0** on the rule set (deletes the person's pending nodes there) and reduces matches/days or releases; no history | [observed] |
| referee assault | 0x55e300 | releases the current ones and creates a 90 or 180 day FIFA ban | [observed] |
| transfer | 0x556270 | releases those that do not apply at the new club and clears the slot | [observed] |
| person removed | 0x556830 | releases everything, clears history and held bans, record = 0xff | [observed] |
| new season | 0x7e7ed0 (vtable `+0`) | entry without a ban is dropped (slot = −1); a ban that started ≥ 2 years ago (4 for FIFA) is released and its entry dropped; newer ones stay, with counters zeroed; then **compacts** the array (0x7e9050) and rewrites the index in the slots | [observed] |

**For the editor, repeat the upheld appeal** (it is the game's own path to "take the ban away" without serving it) [the path is observed; choosing it, inference]:

```
D = [0xae28f0]; rec = [D+0x14] + s*29
for k in 0..3:
    rs = (i8) rec[5k];  if rs == -1: continue
    R = [[D] + 4*rs];  if R == 0: continue
    idx = (i32) rec[5k+1];  e = [R+0xb] + idx*14
    if idx < 0 or idx >= [R+0x13] or [e] != s: continue
    remove from list [R+7] the 0x70 nodes with [node+0x1c] == s      (0x7e91d0)
    b = (i16) [e+6];  if b != -1: release_ban(s, b) as above
```

To remove only the ban for one competition: `rs = [[0xadadfc] + 4*id + 0x50]` and slot `[R+4]`; remember that a ban with bit 1 (`+0x13`) also counts through FIFA (rule set 0) [observed 0x55d270].

Caution: the slot's index changes at the new season (compaction) and on load; always read it at the moment [observed]. A loaned player may have slots in the held list (`rec+0x1c > 0`): there the record's slot is 0xff, so `release_ban` does not find the entry through the slot; the editor must use the held node (`+5` rule set, `+6` index) and set `entry+6 = −1` directly [inference from the code of 0x7e82e0 and 0x556570].

## 3. Other places

| Place | Holds the ban? | Needs changing? |
|---|---|---|
| `ban_history.tmp` | only **served** bans (file list per person, `rec+0x14/+0x18`) | no [observed: only 0x556e90 writes it] |
| entry counters (`+4` yellows) | no; but if kept, the next yellow may create a new ban | optional [inference] |
| ban news item (type 0x7d2, "appeal" button) | a copy of the fields for the text | no: the appeal (0x557000) returns early if `entry+6 == −1` [observed] |
| club selection (`club+0x1e0`, 20 slots) | `is_player_banned` drops the banned player from it | no [observed 0x55d270] |
| team-selection check 0x7260f0 | calls `is_player_banned` (0x55d270) for the banned list; the codes from 0x8cf5e0 (`[0xde1f54]`, `get_ineligibility_code`) are registration/transfer rules, not discipline | no [observed; the meaning of codes 10/11, inference] |
| 79-byte record in `general.dat` | it is the per-person contract table (`0xAE2D0C`, `CAMPOS.md`), not discipline | no [observed] |
| `match_stats`, `player stats` (season cards) | statistics | no [inference: none of the ban routines reads them] |
| `staff` (0x6e) | only the nation (+0x1a/+0x1e) for the nation index | no |
| `fine.dat` (`[0xae2a3c]`) | fines, another subsystem | no |

## 4. Reading it in the running game's memory

```
D      = [0xae28f0]                       ; Discipline*
N      = [0xae2364]                       ; persons
person = [0xae23bc] + s*0x6e              ; [person] == s
rec    = [D+0x14] + s*29
  slot k: ruleset = (i8)[rec+5k], idx = (i32)[rec+5k+1]
R      = [[D] + 4*ruleset]                ; RulingBody*, may be 0
e      = [R+0xb] + idx*14                 ; check [e] == s and idx < [R+0x13]
b      = (i16)[e+6]                       ; -1 = nothing
ban    = [D+4] + b*20                     ; BanRecord
today  = (i16)[0xae2c90] day, (i16)[0xae2c92] year
competition rule set = (i8)[[[0xadadfc] + 4*id] + 0x50]
pending = list [R+7]: [+2] first node; node: [+0] data (0x70), [+6] next
```

All [observed]. To list every banned player: walk `[D+4]` up to `(i16)[D+0xa]` skipping free ones; the owner comes from the entries of the rule sets listed in `+0xc..+0xf` (the ban does not store the person) [observed].

## 5. In-game proof (done on 25/10/2026: see `CONFERENCIA_SUSPENSAO.en.md`, with `engenharia-reversa/remover_suspensao.py`)

Without clicks it would go like this: in a copy of the save, delete one ban in `discipline.dat` (the §2 steps, rewriting the item in an uncompressed type 3 container), load it with `-load` (`prova_save.py --sav`), let the holiday career run one day and check in memory (the §4 chain) that the entry still has `+6 = −1` and the array is consistent; and, to see the effect, that the player makes the automatic line-up of the next match. Writing the item back requires rewriting the container index (sizes and positions): that is tool work, so it was left out [inference].
