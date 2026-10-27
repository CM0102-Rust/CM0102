# Map of competition statistics and the person's squad byte

Survey for the save editor (Lucas's request via the orchestrator, 26/10/2026): (A) where the save keeps who played for which club in each competition (the "already played for another club in this competition" rule, *cup-tied*) and how to release the player; (B) the value of people who are not players and the person's `+0x5e` byte (PlayingSquad).

Read-only: disassembly (`desmonta.py`, `refs.py`) and Ghidra pseudocode (`descompilar.py`) of `cm0102.exe` 3.9.68, the community map (`referencias/cm0102-function-map`, function and global names) and reading a **copy** of `jogo/FeriasTesteSave.sav` (MD5 33fda465193b5e8e6d7ace93ecd44559, checked before copying; the original was not touched). The game was not opened. Save date: 205/2001 (24/07/2001), start of the season.

Every statement is marked **[observed]** (read in the disassembly, the pseudocode or the file) or **[inference]** (deduction without direct proof). Addresses are VAs. "Person" = 110-byte record of `staff.dat` (in memory, `[0xae23bc] + id * 0x6e`).

## A. Who played for which club in each competition

### A.1 Where it lives

Each competition has a `Comp_Stats` object at `comp + 8` (`comp = [[0xadadfc] + 4 * competition_id]`) [observed 0x57a0c0..0x57a0c9]. It holds the team table, the **player table** and the competition records. There is no separate save item: `Comp_Stats` goes inside the competition's own `.tmp`, `<competition name>_<id>.tmp` (e.g. `Taça de Inglaterra_351.tmp`, `Copa do Brasil_68.tmp`) [observed].

| Who | Routine | What it does |
|---|---|---|
| writes the `.tmp` | `Cup::save_cup_to_disk` 0x5210f0, `League::save_to_temp_file` 0x6891c0 | file in `CM3_TEMP`, which `save_game_data` packs (`zadd "*.tmp"`, `MAPA_SAVE.en.md` §2) [observed] |
| reads the `.tmp` | `Cup::load_from_disk` 0x51fc00, `League::load_from_temp_file` 0x687b10 | builds the `Comp_Stats` (0x49ee70) and calls `Comp_Stats::load` 0x4ab890 [observed: calls at 0x51fd74/0x51fd8e and 0x687ca4/0x687cbe] |
| writes/reads the block | `Comp_Stats::save` 0x4ac4e0 / `Comp_Stats::load` 0x4ab890 | fields in the table below [observed] |

### A.2 Layout of the competition `.tmp` (start)

| Offset in the item | Size | Field | Source |
|---|---|---|---|
| 0x00 | 0x50 | name (`comp + 0x53`) | [observed 0x5210f0, 0x6891c0] |
| 0x50 | i32 | 1 = has `Comp_Stats` (block follows), 0 = none | [observed] |
| 0x54 | i32 | team table capacity (`+0x10`) | [observed 0x4ab914] |
| 0x58 | i32 | team count (`+0x14`) | [observed] |
| 0x5c | 118 × count | teams (0x76-byte records) | [observed] |
| … | i32 | player table **capacity** (`+0x1d`) | [observed 0x4abb3e] |
| … | i32 | player **count** (`+0x21`) | [observed] |
| … | 45 × count | **players** (0x2d-byte records) | [observed] |
| … | 100 | 25 i32 competition totals (`+0x56a`, summed at 0x4a25f0) | [observed] |
| … | 0x269 + 0x269 | records (`+0x26` and `+0x28f`) | [observed] |
| … | i32, i32, i16 | `+0x4f8`, `+0x4fc` (first/last node in `comp records history.tmp`), `+0x500` | [observed; meaning of the two i32, inference from `FILE_LLIST::add_node` in 0x4a1c50] |
| … | 0x5c, i32 | `+0x506`, `+0x562` | [observed] |
| … | i32 n, i32, n × 0x1c | run table (`+4`, `+8`, `[+0]`) | [observed] |
| … | rest | cup/league data (vtable method `+0x24` and the following fields) | [observed] |

The load allocates **capacity × 45** and reads **count** records (same for teams) [observed 0x4abb3e..0x4abc52]: in the file, capacity must be ≥ count, or the read overruns the allocated block. The game grows the table by 22 (0x4ad030); in the save every capacity is a multiple of 22 and within 22 of the count [observed]. With the current save version (`[0xdd7110]` = −0x3e047) the record is read whole (45 bytes) and the 100-byte block exists [observed in the version checks of 0x4ab890].

### A.3 The player record (45 bytes)

| Off. | Type | Field | Source |
|---|---|---|---|
| +0x00 | i32 | player id (`player.dat`, = the person's `staff +0x61`); **sort key** | [observed 0x4ad030; in the save, 1018 of 1018 match in the Copa do Brasil] |
| +0x04 | i32 | person id (`staff.dat`) | [observed 0x4a0bbc..0x4a0bc7] |
| +0x08 | i32 | **id of the club he played his last match in this competition for** (the tie) | [observed 0x4a0bce..0x4a0bdb: `[club]` of the player's side in the match, `match+0x1c` or `+0x20`] |
| +0x0c | u8 | appearances | [observed: +1 per match; the name, inference] |
| +0x0d | u8 | appearances off the bench (lineup slot 11..19) | [observed the slot test; the name, inference] |
| +0x0e | u8 | goals | [observed: adds `lineup+0x21`; inference that it is goals] |
| +0x1c | u8 | copied from `match+0x34` every match; 0xff on creation | [observed; meaning, inference] |
| others | — | other statistics summed by `record_match_result` | not decoded |

`add_player_entry` 0x4ad030 zeroes the record, writes +0 and +4, sets +0x1c = 0xff and marks the table unsorted (`+0x25 = 0xfe`) [observed]. `find_player_entry` 0x4a4a30 sorts by key +0 when `+0x25 ≠ 0xff` and does a binary search (comparator 0x4ad250: `+0x25 = 0xff` = by key) [observed]. `+0x25` is not written to the file: the constructor sets 0xfe and the load leaves it, so the first lookup after loading re-sorts [observed 0x49ee70, 0x4ab890]. In the save the table is already sorted [observed].

### A.4 Who writes and who reads

- **Writes:** only `Comp_Stats::record_match_result` 0x49f570 (called by `Cup::process_fixture` 0x51a150 and `League::add_result` 0x685d30): for every player who took the field, it finds the record (`find_player_entry`) or creates it (`add_player_entry`) and **rewrites +8 with the match club** every game [observed 0x4a0b66..0x4a0bdb].
- **Clears:** `Comp_Stats::archive_season_and_reset` 0x4a1c50, at the competition's season change: frees the table and sets capacity and count to 0 [observed]. The tie lasts one season.
- **Reads (the rule):** the `get_ineligibility_code` of the nation's transfer rules. The only ones that look at the table are these (the 7 callers of 0x4a4a30 are them, `record_match_result` and the `get_player_stat` screen) [observed]:

| Rule | Routine | Competitions | Code |
|---|---|---|---|
| England | 0x57a090 | FA Cup (`[0x9cf74c]`) and League Cup (`[0x9cf750]`) | 1 |
| Scotland | 0x7f65d0 | Scottish Cup (`[0x9cf75c]`) and League Cup (`[0x9cf760]`) | 1 |
| Ireland | 0x63c350 | `[0x9cf83c]`, `[0x9cf840]`, `[0x9cf844]` (League Cup, Senior Challenge Cup, Super Cup) | 1 |
| Northern Ireland | 0x7937f0 | `[0x9cf8c0]`, `[0x9cf8c4]`, `[0x9cf8c8]` (Irish Cup, Gold Cup, League Cup) | 1 |
| Brazil | 0x43f210 | every competition except the reserve one (`[0x9cf684]`) and friendlies (0x5c6710) | 4 |

Global names from the community `data-map`. The rule is the same in all five [observed]:

```
if the match has competition C (match+0x14) and C is in the list
   and the person has a player (staff+0x61 ≠ 0)
   and e = find_player_entry(comp(C)+8, player) exists
   and the person has a club (staff+0x39 ≠ 0)
   and e+8 ≠ current club id          → ineligible (code above)
else                                  → base rule (TRANSFER_RULES 0x8e8a60)
```

The code comes out through `TRANSFER_MANAGER::get_ineligibility_code` 0x8cf5e0 for automatic team selection (`MATCH_MAN::pick_team`), the team selection check (0x7260f0) and `is_player_available_for_match` 0x45cbe0 [observed: callers of 0x8cf5e0; the path to the nation's rule, from the community map]. The European rule (0x5832e0) does not use this table [observed].

In the test save: 58 competition items, all with `Comp_Stats`, totalling 5,913 player records; 412 have +8 different from the person's current club, but 386 of them are in national-team competitions (the match club is the national side, and no rule looks at those). The real club ties are in the Brazilian state leagues and the Copa do Brasil (Brazil rule), the Libertadores, the Intertoto and Korea. The English cups are still empty (24/07) [observed].

### A.5 How to release the player in a competition

**Recommended: rewrite the record's `+8` with the person's current club id (`staff +0x39`)** [the effect on the rule is observed; choosing this, inference]. It is what the game itself does at his next match for that club. The item keeps its size, the order stays (the key is +0) and the statistics stay (they now show under the new club).

```
item = "<name>_<id>.tmp";  if i32[0x50] == 0: no table, nothing to do
p = 0x54; nt = i32[p+4]; p += 8 + 118*nt
np = i32[p+4]; base = p + 8
for k in 0..np-1:  e = base + 45*k
    if i32[e+4] == person: i32[e+8] = staff[person].i32[0x39]
```

Alternative: delete the record (shift the following ones, `count − 1`, keep the key order and capacity ≥ count). The item shrinks by 45 bytes, the container index must be rebuilt, and his games and goals in the competition are lost. There is no other copy of the tie to fix: `player stats.dat` and `match_stats` take no part in the rule [observed: only the 5 `get_ineligibility_code` read +8].

Care: a type 4 (RLE) container must be recompressed when the item is rewritten; the rule applies only to the competition of that `.tmp`; at the new season the table is cleared anyway [observed].

## B. The value of non-players and the `+0x5e` byte

### B.1 The value (`staff +0x52`, i32)

It lives in the person record, `staff.dat +0x52` (the "+82 value" of `PARES_TRANSFERENCIA.en.md`) [observed]. The game **recomputes it every day**: `game_main_day_loop` calls `CONTRACT_MANAGER::daily_update` 0x4c7ac0 with the person pass on (0x5cd406), and it rewrites every person's `+0x52` [observed 0x4c7ec5..0x4c8070]:

| Situation (from the main contract, the person's `general.dat` +0 → `contract.dat`) | New value |
|---|---|
| no contract, or the contract's club does not exist | 0 |
| player job (11, 12, 13 or 15, 0x53b620), human club | recomputed only if bit 0x04 of `contract +0x4e` is set; then capped at the release clause if any (0x867b50, `contract +0x21`) |
| player job, computer club | odd day: sets bit 0x04, recomputes and caps at the clause; even day: untouched |
| **other job (non-player)** | **every day:** sets bit 0x04 and `value = contract_manager_calc_player_value` 0x4d0c60 |

For a person without a player (`staff +0x61` null), 0x4d0c60 returns 0 with no club and, with a club, the compensation `transfer_manager_calc_compensation_fee` 0x8d0780 (with "ignore new rules" = 1) [observed]. With `+0x69` (non-player) present, the formula is [observed 0x8d07cf..0x8d0849]:

```
c = main contract (CONTRACT_MANAGER::get_contract 0x4cf5a0)
if c null or (c+0x35 & 0x3f) != 2:  value = 0
days  = Date::days_since(c+0x2d end, today)         ; days until the contract ends
value = trunc(days * wage(c+0x0c) / 7)               ; the remaining wages
value = clamp(value, 1,000, 1,000,000)
```

In the file: `contract.dat` 80-byte record, `+12` wage, `+45` end (`i16 day, i16 year`), `+53` type (`PARES_TRANSFERENCIA.en.md`). **Checked in the test save:** of the 11,179 people with `+0x61 = −1` and `+0x69 ≠ −1`, 11,176 match the formula using today = 204/2001 (the last day processed before the saved date, 205); the 3 that do not joined or lost their contract on day 204/205 [observed]. With today = 205 only 5,858 match.

To write it without breaking anything: a non-player's `+0x52` is derived and the game overwrites it the next day [observed]. To get another value, change the contract (wage or end date); when editing the contract, rewriting `+0x52` with the formula above keeps the save coherent until the next day [inference]. A person with neither `+0x61` nor `+0x69` but with a contract would send 0x8d0780 down a path that calls 0x4d0c60 again [observed 0x8d08f6..0x8d098a]: do not create that combination [inference: endless recursion].

A player's value (`+0x61` present) is the 0x4d0c60 formula from reputation, position, age and nation (summary in `ENDERECOS_COMUNIDADE.en.md`); it is also redone by the daily pass (table above).

### B.2 The `+0x5e` byte (PlayingSquad)

Two independent pairs of bits [observed `staff_set_squad_flags` 0x540980: a value with bits 0–1 changes only bits 0–1 (keeps `& 0x0c`); otherwise it changes only bits 2–3 (keeps `& 0x03`)]:

| Bit | Value | Meaning | Source |
|---|---|---|---|
| 0 | 0x01 | club **first team** | [observed `get_squad_placement` 0x4cbb60: at a club with reserves it returns 2 = first team if bit 0 is set, 1 = reserves if not] |
| 1 | 0x02 | club **reserves** | [observed: first-team selection (`pick_team` 0x6dcaa8) skips anyone with 0x02; the profile (0x87090a) swaps 0x02 for 0x01 on "move to first team"] |
| 2 | 0x04 | called up to the national side in `staff +0x24` (senior or under-21, depending on the `+0x24` team) | [observed `national_teams_player_in_squad` 0x76da80: senior = `+0x24` is the 0x53b3d0 team and bit 0x04; under-21 = the 0x53b4f0 team and bit 0x04] |
| 3 | 0x08 | called up to the B national side | [observed 0x76da80 (slot 1 = senior team and bit 0x08), `get_player_squad_slot` 0x76e070] |
| 4–7 | — | unused | [observed: none of the 62 readers tests 0x10..0x80; some copy the whole byte (load, `Staff::operator=`, record shuffle)] |

The club has no separate reserve list: the `club.dat` squad is 50 slots, and first team/reserves is only this byte [inference from the `club.dat` layout in `PARES_TRANSFERENCIA.en.md`].

Writers: `staff_set_squad_flags` (16 calls, always with 1, 2, or the national value), the national-team routines (0x76c120, 0x76d880, 0x775c92, 0x776366, which clear bits 2–3), the database load and `database_trim_staff_tables` (0x52651b, sets 2) [observed]. On a transfer (`move_player_to_club`, 0x8ca7d9..0x8ca81f): computer club, club without reserves, or club with a B-team pair → 0x01; human club with reserves → 0x01 if the contract's squad status (`contract +0x4f >> 4`) is 1..4, otherwise 0x02 [observed; status names, inference].

The database load (`load_database`, 0x52d29e..0x52d2c1) turns 0x02 into 0x01 at a club that cannot have reserves (`database_club_has_reserve_squad` 0x542740 false) [observed; that this code also runs when loading a save, inference].

In the test save [observed]: players with a club: 0x01 (26,368), 0x02 (4,362), 0x05 (5,511 called up), 0x06 (316); non-players almost all 0x00/0x01 (the byte has no role for them); 0x08 does not occur (no B side called up).

**To write it without breaking anything** [inference from the observed]:

1. Touch only bits 0–1: `b = (b & 0x0c) | new`, with new = 0x01 (first team) or 0x02 (reserves). Never 0x03, and 0x02 only if the club can have reserves (0x542740: excludes B-team pairs, Spanish, German and Portuguese clubs and some national sides).
2. Do not touch bits 2–3: they go with `staff +0x24` and the `national_teams.dat` lists. To drop someone from a national squad, the game's path is `NationalTeams::remove_player_from_squad` 0x7762f0 (clears `+0x24`, the bits and the slots).
3. The club lineup (`club.dat`, 20 slots) should not hold a reserve in the first team: `pick_team` redoes the computer's; the human's is his own [inference].

## Open points

- Exact names of fields +0x0d..+0x2c of the competition player record and of the team record (118 bytes): not decoded.
- `database_club_has_reserve_squad` 0x542740 was not read inside (description from the community map).
- Whether the 0x52d29e code runs on a save load and not only on a fresh database: check with a probe once the load is in Rust.
