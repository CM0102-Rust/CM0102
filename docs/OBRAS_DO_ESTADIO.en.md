# Stadium works (covered, heating, capacity)

A survey so the editor knows what the stadium works bytes mean and whether it is safe to write them. Starting point: at Flamengo (Maracanã) the editor read the undersoil heating (stadium +0x4d) as 1, outside the database's 0/255, after the board had announced works at the stadium. Read only: disassembly and pseudocode (Ghidra) of `cm0102.exe` 3.9.68, the community map (`referencias/cm0102-function-map`, `fin_prm.cpp`, `manager_manager.cpp`, `stadium.cpp`, `weather.cpp`), a scan of the +0x4c/+0x4d displacements across the whole `.text`, `jogo/Data/stadium.dat` and the `stadium.dat` of a **copy** of `jogo/FeriasTesteSave.sav` (MD5 33fda465193b5e8e6d7ace93ecd44559, untouched), read with the functions of `engenharia-reversa/prova_save.py`. The game was not run.

Every claim carries **[observed]** (read in the disassembly, the code or the file) or **[inference]** (deduction without direct proof). Addresses are VAs.

## Summary

- **There is no "works in progress" state in the stadium.** +0x4c (covered) and +0x4d (heating) are booleans: 0 = no, **any other value = yes**. The database uses 255; the game writes **1** when it installs. The Maracanã's 1 means "heating installed by the game", not "works in progress" [observed].
- The heating is installed **at once** at the end of the club's league season (0x59cfa0), charging 1.5 million and posting the news "Undersoil heating installed", whose body says "The board have announced plans to install undersoil heating..." — so it reads like an announcement, but the byte is already written when the news goes out [observed].
- The only "pending" item is the **expansion request approved by the board** (meeting with the board), kept in a bit of the club's record in `MANAGER_MANAGER`, not in the stadium; it is consumed at that same season end [observed]. There is no day counter anywhere [observed in the routines read].
- For the editor: show both bytes as yes/no (`!= 0`), accept 1 as "yes", write 255 for "yes" and 0 for "no". Writing at any time is consistent: there is no ongoing work that would overwrite it later (see §4).

## 1. The fields and their values

`TStadiums` record (78 bytes, `stadium.dat`; table `[0xae23b0]`, `[0xae2358]` records); the club points to its stadium at `club+0x69` (pointer) and says whether it owns it at `club+0x6d` [observed].

| Off. | Type | Field | Values | Source |
|---|---|---|---|---|
| +0x3c | `i32` | total capacity | places; changes with the works (§2) | [observed] |
| +0x40 | `i32` | seated places | ≤ capacity; changes with seating works | [observed] |
| +0x44 | `i32` | **expansion ceiling** (maximum allowed capacity) | expansion stops at `+0x44 − +0x3c`; 0 or less than the capacity = cannot expand | [observed] 0x59cfa0, 0x599e00 |
| +0x4c | `u8` | covered | **0 = no; ≠0 = yes** (database: 255; the game writes 1) | [observed] |
| +0x4d | `u8` | undersoil heating | **0 = no; ≠0 = yes** (database: 255; the game writes 1) | [observed] |

Counts [observed]: in `jogo/Data/stadium.dat` (8917 records) covered is 0 in 8872 and 255 in 45; heating is 0 in 8720 and 255 in 197; no other value. The test save's `stadium.dat` also has only 0 and 255 (no works happened). The Maracanã in the database: 70096 / 70096 / ceiling 0 / 0 / 0; in the test save the ceiling is already 825 (where that 825 comes from was not looked for).

Who **writes** stadium +0x4c/+0x4d, from the `.text` scan (instructions with displacement 0x4c/0x4d, setting aside the structures that are not the stadium) [observed]:

| Routine | Writes | When |
|---|---|---|
| 0x59cfa0 `FinanceTable::season_end_club_update` | +0x4d = 1; +0x4c = 1 | season end (§2) |
| 0x59ed70 `FinanceTable::process_club_finance_events` | +0x4d = 0 | prolonged financial crisis (§2) |

Apart from these, only the database and save loading (block copy). Every reader checked tests **zero/non-zero** (`test cl, cl` or `!= 0`); none compares with 1 or 255 [observed]: the club screen 0x460590 (texts "<%ld - capacity> all seater with under-soil heating", "...with under-soil heating" etc.), the weather 0x923630 and 0x923970, the finances 0x59b9c0, 0x59cfa0 and 0x59ed70. The other +0x4c/+0x4d accesses of the scan (match engine, match screens) belong to other structures or were not checked [inference for the unchecked ones].

## 2. The works routines

### Season end: 0x59cfa0 `FinanceTable::season_end_club_update`

Called by 0x59cdd0 `FinanceTable::end_season_for_nation` (for each league club, at the nation's season end) [observed]. Values: balance = `i64` at `finance+0` (0x167-byte record per club); every work adds its cost to two expenditure totals (`finance+0x8c` and `+0x12c`) and takes it from the balance [observed; the name of each total is inference]. Reputation = `club+0x80` (`i16`); training facilities = `club+0x7f`.

Order inside the routine [observed]:

1. Stadium purchase (non-owners with balance > 35 million): `club+0x6d = 1`, −20 million, news type 4 "Stadium purchase announced" (or type 5 for the seller). Does not touch +0x4c/+0x4d.
2. Training by reputation (raises `club+0x7f`, charges).
3. **Approved expansion request**: if bit 0x40 of the byte `[[0xb67a44]] + club * 0x49 + 2` is set and the club is human (0x600a50), it flags "force expansion" and **clears the bit** (`& 0xbf`).
4. **Heating**: if balance > 17.5 million, reputation > 7500, capacity > 25000 and `+0x4d == 0` → `+0x4d = 1`, −1.5 million, news type 6 (`0x59ffb0(club, stadium id, 6)`) and **leaves the works part** (skips the expansion).
5. **Roof**: otherwise, if balance > 75 million, reputation > 9250, capacity > 57500, training = 20 and `+0x4c == 0` → `+0x4c = 1`, −7.5 million, **no news**, and leaves the works part.
6. **Expansion**: needs balance ≥ 5.25 million (a human club also passes with balance + reputation × 1000 ≥ that; without money and with the request flagged, the works-cancelled news goes out) and capacity ≤ 125000. With the request flagged, high average attendance or capacity < 2 × reputation (reputation > 4250) → enlargement; otherwise, standing-to-seats conversion by chance:
   - all-seater stadium (`club+0x64 == 1`): up to 16000 places in multiples of 500, capped at `+0x44 − +0x3c` and at `max(+0x44/3, 5000)`; if ≥ 1000: `+0x3c += n`, `+0x40 += n`, cost `((n/1000 + 1) * 6000 + n) * 125`, news type 1 ("...plans to enlarge the stadium by N seats");
   - with standing terraces: same up to 21000, only `+0x3c += n`, cost `(n + (n/1000 + 1) * 1000) * 50`, news type 2 ("...increase the standing area ... another N supporters");
   - standing-to-seats conversion: `n ≤ +0x3c − +0x40`, ≥ 500 → `+0x3c += n`, `+0x40 += n` as decompiled, cost `(n/1000 + 1) * 50000 + n * 75`, news type 3 ("...install N seats within the stadium");
   - if no expansion was possible and capacity < 30000: `+0x44 += 1500` (the ceiling rises for next time); with the request flagged the works-cancelled news goes out (0x59fae0: "Stadium Expansion plans shelved"/"cancelled"/"Local council blocks planning permission").
7. Requested training (bit 0x80 of the same byte as step 3): done (0x59b630, news type 8) or cancelled, and the bit is cleared.

Consequence [observed]: if heating or the roof is installed at the same season end, the expansion approved by the board **does not happen** and its bit has already been cleared — the request is lost with no cancellation news.

### The news: 0x59ffb0 and 0x5a0050

`0x59ffb0(club, value, type)` creates news 0x177d; the text (0x5a0050) picks by type: 1/2/3 expansion (headline "Stadium expansion plans announced"), 4 purchase, 5 sale, **6 "Undersoil heating installed"** (body "The board have announced plans to install undersoil heating in the {stadium}..."), **7 "Stadium Facilities reduced"** (body "The pitch's undersoil heating has failed..."), 8 training [observed]. The "announced plans" wording is only text: all these news items go out **after** the stadium and the balance are written [observed].

### Crisis: 0x59ed70 `FinanceTable::process_club_finance_events`

The financial-strain counter `finance+0x166` rises while the club is in debt; when it reaches 51 or more (and the club is in an active league) [observed]:

- with heating (`+0x4d ≠ 0`) and reputation ≤ 4249: news type 7 and **`+0x4d = 0`**, training drops;
- otherwise (capacity ≥ 2751): closes part of a stand: `+0x3c −= x`, `+0x40 −= x` when it fits, **`+0x44 += x`**, news 0x59f900 ("stand closed"), counter back to 20.

The roof is never removed by the game [observed in the scan].

### The board request: 0x6a8810 `MANAGER_MANAGER::handle_board_request`

Option 1 "Expand the Stadium" [observed]: refuses if the request is already approved (bit 0x40, reply "already indicated that they are considering the expansion"), if the club does not own the ground (`+0x6d == 0`), if capacity is above 75000 or if money is short (2.5 / 5 / 7.5 million by capacity). On approval (board confidence and luck) it replies "plans afoot to expand the stadium" and **sets bit 0x40** at `[[0xb67a44]] + club * 0x49 + 2`. Nothing in the stadium changes at that moment; the work is step 3 → 6 of the season end. The request never touches +0x4c/+0x4d.

For the Maracanã (70096 places, ceiling 825 in the test save): even with the request approved, `+0x44 − +0x3c < 0` gives 0 places, capacity is above 30000 (the ceiling does not rise) and the works-cancelled news would go out [inference from the code and the test save values; Lucas's save was not read].

### Who reads (effects)

| Reader | Use | Source |
|---|---|---|
| 0x460590 club screen | capacity text "with under-soil heating", "all seater", covered | [observed] |
| 0x59b9c0 `FinanceTable::daily_update` | upkeep: 12500 if covered; +7500 (reputation < 4501) or +12500 if heated | [observed]; the period of that branch was not checked |
| 0x923630 `Weather::check_postponement` | covered → never postponed by weather; heated → escapes the frost/snow postponements | [observed] |
| 0x923970 `Weather::get_pitch_condition` | covered → no pitch flag; heating improves the pitch condition | [observed] |
| 0x59cfa0, 0x59ed70 | conditions to install/remove (above) | [observed] |

## 3. If the editor writes 0 or 255

No ongoing work is kept in the stadium, so there is nothing to complete, cancel or overwrite [observed: the only writes are instantaneous, §1].

| Write | Effect |
|---|---|
| 1 → 255 (or 255 → 1) | none: every reader only tests `≠ 0` [observed] |
| 1 → 0 (remove installed heating) | consistent; at the next season end, if the club still meets the conditions, the game reinstalls it, charges 1.5 million again and posts the news [observed] |
| 0 → 255 (give heating) | consistent; the game charges nothing, posts no news and does not install again (already ≠ 0); upkeep and the weather effect apply; in a crisis it may be removed (§2) [observed] |
| covered 0 ↔ 255 | same, never removed by the game [observed] |
| edit capacity/seats/ceiling with an approved expansion pending | consistent: the season end computes from the values at that time (it may yield 0 places and cancel) [observed] |

Detail: the flags belong to the stadium, not the club; two clubs sharing the same stadium (same `club+0x69`) see the same change [observed in the pointer access].

## 4. Recommendation for the editor

1. Read +0x4c and +0x4d as booleans (`≠ 0`). **Do not flag 1 as invalid**: it is the value the game itself writes [observed].
2. When writing, use 255 for "yes" and 0 for "no" (the database convention); if the user did not change the field, keep the byte as it is (no need to "normalise" 1 to 255; the game does not care, but it avoids a pointless difference in the save) [inference about convenience; the equivalence is observed].
3. No need to lock the fields "during works": there is no ongoing work in the stadium. UI hint: for value 1, show "yes (installed by the game)".
4. If the editor wants to show "works announced", the data is bit 0x40 (expansion approved) and bit 0x80 (training approved) of the byte `[[0xb67a44]] + club * 0x49 + 2`, valid until the end of the club's league season [observed in the code]. In the save, the record lives in the `man_conf.dat` item: `MANAGER_MANAGER::save_man_conf` (0x68e920) writes the number of clubs (u32, `[0xae235c]`) and then the whole `[[0xb67a44]]` table, 0x49 bytes per club, in id order (0x68eab8); the job vacancy lists follow. The club's byte is therefore at `4 + club * 0x49 + 2` in the item [observed: the code and the item sizes in two saves, 7,029 and 11,318 clubs; neither had the bit set when it was saved]. Changing the capacity while the bit is set is safe (§3).
5. Writing a capacity above the ceiling (+0x44) breaks nothing read here, but blocks expansions (the ceiling becomes the limit); when raising the capacity, raising the ceiling with it is what the game itself does in 0x599e00 [observed].
