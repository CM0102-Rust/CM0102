# Preferences and stadium in memory (for the live editor)

Survey so the editor can write, with the game running, the **staff preferences** (favourite and disliked clubs and people) and the **stadium fields** (city and attraction, reserve stadium). Read-only: disassembly of `cm0102.exe` 3.9.68 (`resolve_database_indexes` 0x5315a0, `save_database` 0x52dad0, `database_trim_staff_tables` 0x524d70, `staff_interest_in_job_at_club` 0x844780 and `Shortlist_Manager::staff_interest_in_joining_club` 0x846920), the Rust copies in `rust/src/motor.rs` (`resolver_indices_do_banco`, batch 233; `cortar_tabela_de_clubes`; the preference routines 0x547810..0x5480f0, batches 46, 72, 234, 235 and the `dbp_*`/`dbq_*` queue), `docs/FORMATOS.md` and the Bosman Edition creation inventory (§2d). The game was not opened.

Every statement is tagged **[observed]** (read in the disassembly or the code) or **[inference]** (deduced without direct proof). Addresses are VAs.

## Summary

- **In memory every link field is a pointer; the id only exists at +0 of each record.** Preferences: the 6 clubs (+0x04..+0x18) point into the club table, the 6 people (+0x1c..+0x30) into the staff table; +0x00 is the row's own id [observed].
- **Empty slot in memory = 0 (NULL). On disk = −1.** Never write −1 in memory [observed].
- **Write pointer = `[table] + id × size`**, with `0 ≤ id < [count]`, checking that the target's `+0` is that same `id` (it is what the save writes back) [observed].
- **The save turns pointer into id by reading the target's +0** (`[ptr]`), NULL becomes −1, writes, then calls `resolve` again [observed]. While saving, the fields hold **ids** in memory: the editor must not read or write during that window [round trip observed; the risk is inference].
- **Stadium:** `club+0x69` (stadium) and `club+0x6e` (reserve stadium) are pointers into the stadium table; `stadium+0x38` is a pointer to the city, and the attraction is `city+0x33` (byte). The attraction is read through the chain `club+0x69 → +0x38 → +0x33` in a person's willingness to join the club [observed].

## 1. The tables

| Table | Pointer | Count | Bytes | Source |
|---|---|---|---|---|
| staff | `[0xae23bc]` | `[0xae2364]` | 110 (0x6e) | [observed] |
| preferences | `[0xae23c8]` | `[0xae2370]` | 52 (0x34) | [observed] |
| clubs (and national teams) | `[0xae23b4]` | `[0xae235c]` | 581 (0x245) | [observed] |
| stadiums | `[0xae23b0]` | `[0xae2358]` | 78 (0x4e) | [observed] |
| cities | `[0xae23ac]` | `[0xae2354]` | 56 (0x38) | [observed] |
| nations | `[0xae23a8]` | `[0xae2350]` | 290 (0x122) | [observed] |

**Invariant `+0 == row`.** After the creation trims, each record's `+0` is its row index: the club trim (0x5267c0, copy `cortar_tabela_de_clubes`) writes `+0 = row` and remaps the club pointers of the preferences (+4..+0x18) to the new index; the staff trim 0x524d70 copies staff, player, non-player and preferences forward with `+0 = counter` (0x525ed9, 0x525f50) and remaps the staff pointers of the preferences (+0x1c..+0x30, 0x525d30..0x525dde) through its remap table [observed]. So `pointer = [table] + id × size` and `id = [pointer]` are the same relation.

**Fixed address during the career.** The tables are trimmed with an in-place `realloc` (asserting the same address) and carry reserve rows (1325 "virtual" staff, each already with its own preferences row, 0x526554..0x526636) [observed]. I found no `realloc` of these tables during the career [inference: I did not sweep every call to 0x945667]. Still, read `[0xae23c8]` etc. on every write; do not cache the address.

## 2. Preferences (`[0xae23c8]`, 52 bytes)

| Offset | Type in memory | Field | At load (0x5315a0) | Empty |
|---|---|---|---|---|
| +0x00 | `i32` **id** | row id (= index) | stays id | — |
| +0x04, +0x08, +0x0c | `Club*` | favourite clubs 1..3 | id → `[0xae23b4] + id × 581` | 0 |
| +0x10, +0x14, +0x18 | `Club*` | disliked clubs 1..3 | same | 0 |
| +0x1c, +0x20, +0x24 | `Staff*` | favourite people 1..3 | id → `[0xae23bc] + id × 110` | 0 |
| +0x28, +0x2c, +0x30 | `Staff*` | disliked people 1..3 | same | 0 |

Sources [observed]: `resolver_indices_do_banco` (all 12 with the "no ceiling" rule: negative → NULL; **id ≥ count becomes a pointer past the table**, no box), `save_database` 0x52e1db..0x52e324 (all 12: `ptr ? [ptr] : −1`), the query and write routines (below).

Who points to the row: `staff+0x65` (`Prefs*`). At load, negative id → NULL; id ≥ `[0xae2370]` → log entry, error box (Database.cpp line 0x1907) and NULL [observed]. Staff without preferences = `+0x65 == 0`; every routine tests that and does nothing [observed].

### How the game uses the slots [observed, Rust copies]

- **Query** (`gosta_de` 0x547810, `nao_gosta_de` 0x547840, `gosta_do_clube` 0x547d60, `nao_gosta_do_clube` 0x547d30, counts 0x547c90/0x547ce0): compares **pointer** with pointer in the 3 slots; slot 0 never matches.
- **Add** (0x547a10 favourite person, 0x547e70 disliked club and siblings): if already in the opposite list, removes it there and stops; otherwise the first 0 slot; no free slot and `forcar` set, a random slot (0x90cfc0, 3). Writes the **pointer** it received.
- **Remove** (0x547f50, 0x547c10, 0x547c50...) and **fade** (0x547f90, daily): writes 0 into the slot.
- **List** (0x5480f0, 0x548090): copies the non-null pointers in order.

Consequences for the editor: holes are normal (slot 1 empty with slot 2 filled is a state the game produces); repeats are tolerated (0x547e70 does not check repeats among the disliked); the game does not require symmetry with the club (`club+0x9b..+0xaf`, the club's favourite and disliked staff, also pointers) [observed].

### Writing with the game running

1. Person `p` (index), row `L = [p_ptr + 0x65]`; if `L == 0` the person has no preferences: **do not create one** (there is no spare row; rows are one per person) [1:1 observed in the trim; inference that there is no spare].
2. Club id `c`: check `0 ≤ c < [0xae235c]`, compute `P = [0xae23b4] + c × 581` and check `[P] == c`. Person id `s`: `0 ≤ s < [0xae2364]`, `P = [0xae23bc] + s × 110`, `[P] == s`.
3. Write the 4 bytes of `P` (or 0 to clear) at `L + offset`. **Never the id, never −1.** A raw id would be read as a pointer by the queries (never matches) and, worse, by the save (`[id]` = read at a low address, access violation) [observed: the save dereferences every non-null value].
4. To avoid states the game avoids, when making a club a favourite remove it from the disliked (and vice versa), as 0x547870..0x547e70 do [observed in the routines; optional for the game].
5. **When:** with the game idle waiting for the user (a screen open, no "continue" running). Not during the game's save: between the start of `save_database` and the final `resolve` the fields hold ids; a pointer written in that window would be treated as an id by `resolve` (no ceiling → wild pointer) [round trip observed; window is inference]. A 4-byte write is not atomic across processes if the field crosses a cache line (52- and 110-byte records are not aligned); the safest path is writing from inside the process, on the game's thread (extension point `rust/src/extensao.rs`) [inference].

### How it goes back to the save [observed, 0x52dad0]

`save_database` walks the table and, field by field, writes `[ptr]` (the target's +0) or −1; it writes table 22 with `gravar_preferencias` 0x54fd00 (0x530bab); then calls `resolve_database_indexes` to restore the pointers. So what goes into the `.sav` is the id found at +0 of the record pointed to. That is why pointing at the **start** of a valid record is enough; a pointer into the middle of a record saves 4 bytes of junk as the id and, on the next load, becomes a pointer outside the table (the 12 slots have no ceiling) [observed].

## 3. Stadium in memory

### The link fields

| Record | Offset | In memory | Target | Load rule (0x5315a0) | In the save |
|---|---|---|---|---|---|
| club | +0x69 | `Stadium*` (stadium) | `[0xae23b0] + id × 78` | negative → NULL; **id > `[0xae2358]`** → log and NULL (inclusive ceiling: id == count passes) | `[ptr]` / −1 |
| club | +0x6d | `u8` | owns the stadium | not a link | copied |
| club | +0x6e | `Stadium*` (reserve stadium) | same | as +0x69, no log | `[ptr]` / −1 (0x52dc2e) |
| stadium | +0x00 | `i32` **id** | — | stays id | — |
| stadium | +0x38 | `City*` | `[0xae23ac] + id × 56` | negative → NULL; id > `[0xae2354]` → NULL | `[ptr]` / −1 (0x52e1a1) |
| stadium | +0x48 | `Stadium*` (nearby stadium) | `[0xae23b0] + id × 78` | negative → NULL; id > `[0xae2358]` → **clears the city (+0x38) and +0x48 keeps the raw id** (original bug) | `[ptr]` / −1 (0x52e1ba) |
| city | +0x00 | `i32` **id** | — | stays id | — |
| city | +0x1f | `Nation*` | `[0xae23a8] + id × 290` | negative → NULL | `[ptr]` / −1 |
| city | +0x33 | `i8` | **attraction** | data, not a link | copied |

Sources: `resolver_indices_do_banco` and the `save_database` disassembly [observed]. Capacity, seating, ceiling, covered and heating (+0x3c..+0x4d) are data: see [OBRAS_DO_ESTADIO.en.md](OBRAS_DO_ESTADIO.en.md).

### Where the city attraction comes from

`staff_interest_in_job_at_club` 0x844780 (0x845452..0x845469, 0x845c5c) and `Shortlist_Manager::staff_interest_in_joining_club` 0x846920 (0x84797c..0x847993, 0x8489c2) do `stadium = [club+0x69]`; if not null, `city = [stadium+0x38]`; if not null, read the byte `city+0x33`; with 0 they take a branch based on the club's reputation (`club+0x80`) [observed]. Sweep of the whole `.text` (a +0x38 load followed, within 40 bytes, by a byte read at +0x33 off the same register): only those 4 places [observed]. I found no read of the attraction through the reserve stadium [observed in the sweep].

So to change a club's "attraction" the editor has three routes: (a) change the byte `city+0x33` (applies to every stadium in that city); (b) point `stadium+0x38` at another city (applies to every club using that stadium); (c) point `club+0x69` at another stadium. Only (a) involves no pointer change [observed].

### The reserve stadium (`club+0x6e`)

It is a pointer (or NULL) from load onwards and becomes an id again in the save [observed]. Readers outside load and save: the sweep of dword reads at +0x6e found candidates at 0x41c317 (passes the pointer to 0x41cdd0 with "CM3_PICTURES": the stadium picture, if the record is a club) and 0x49f8b1/0x49f90c, 0x6e62d8, 0x7e65c7, 0x7e69cd, without confirming the record is a club [inference]. The reserve stadium's **capacity**, when used, would come from `[club+0x6e]+0x3c` [inference: the 40-byte sweep found no chained +0x6e → +0x3c/+0x40 read].

### Writing with the game running

- **Attraction:** write the byte `city+0x33` (`[0xae23ac] + id × 56 + 0x33`); not a link, nothing to convert; the save copies it [observed]. The database's value range was not checked here.
- **Club stadium / reserve:** id `e` with `0 ≤ e < [0xae2358]` (**not** `≤`: the inclusive load ceiling lets `e == count` through, which points past the table), `P = [0xae23b0] + e × 78`, `[P] == e`; write `P` or 0. Two clubs may share a stadium (the game requires no exclusivity) [observed in the pointer access].
- **Stadium city:** id `k`, `0 ≤ k < [0xae2354]`, `P = [0xae23ac] + k × 56`, `[P] == k`; write `P` or 0.
- **Nearby stadium (+0x48):** only a valid pointer or 0. A raw id there (the original-bug case) makes the save dereference the id as an address [observed: the save reads `[ptr]` for every non-null; inference about the crash].
- Same timing and thread rules as §2.

## 4. Suggested lab proof (not run)

With a copy of the test save, load through `simular.py` and, from inside the DLL (no clicks): (1) check on every preferences row that each slot is 0 or a pointer to the start of a record of the right table with `[P] == (P − base) / size`, and the same for `club+0x69/+0x6e` and `stadium+0x38/+0x48`; (2) write a favourite club, a reserve stadium and an attraction by the rules above; (3) save the game, compare the `staff.dat` (table 22), `club.dat`, `stadium.dat` and `city.dat` items of the `.qst` with the expected values (the id in place of the pointer); (4) load again and repeat (1). The closest tool is `engenharia-reversa/prova_save.py`.
