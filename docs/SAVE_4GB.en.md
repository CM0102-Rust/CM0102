# Saves above 4 GB (model, not implemented)

Planning requested by Lucas (2026-10-25): model the changes to the DLL and to the file so the save can go past 4 GB **without recompiling the game as 64-bit**. Nothing here is implemented. The fix up to 4 GB, already in use, is in [SAVE_2GB.en.md](SAVE_2GB.en.md). The save functions and flow are in [MAPA_SAVE.en.md](MAPA_SAVE.en.md).

Marks: **[observed]** = seen in the disassembly of `cm0102.exe` 3.9.68, in the project code or in the files; **[inference]** = a conclusion or estimate not yet proven.

Sources: disassembly (`desmonta.py`, `refs.py`, `textos.py`), the community map (`referencias/cm0102-function-map`; its "size" is not in bytes), `rust/src/conteiner.rs`, `save/src/lib.rs`, and a read of **only the index** (header and entries, plus the 4 bytes of the game date at `general.dat` + 0xF68) of 8 of Lucas's saves. Nothing was copied, written or run.

## Summary

1. **The first real limit is memory, not the format** [inference]. The exe is not LARGEADDRESSAWARE [observed], so the process has 2 GB of address space. The save's `.dat` items are held in memory in full. If the measured proportion holds, they take ~1.3 GB when the file reaches 4 GB.
2. **The file passes 4 GB before any item passes 2 GB** [inference]. The largest growing item (`player stats history.tmp`) is ~25% of the save. It only reaches 2 GB when the file is around 6 to 8 GB.
3. For the **file** above 4 GB, with items below 2 GB, it is enough to change the 3 routines already in Rust and port 2 or 3 more container routines (phase A, ~1 batch). The `.tmp` files and the subsystem code do not change.
4. **Items** between 2 and 4 GB need `FILE_LLIST` (si/code/file_llist.CPP), `zread` and `utils_copy_file` ported (phase B, ~2 batches). An item above 4 GB needs a change to the `.dat` of 8 subsystems: not recommended.
5. **Rate** [inference]: 45 to 100 MB per season uncompressed. The file reaches 4 GB after 40 to 90 seasons (twice that with compression on). No real save measured here is over 280 MB. The "2.5 GB" ones are filler.

## 1. The limits, one by one

### (a) The container format [observed]

| Part | Content | Limit |
|---|---|---|
| +0, 12 bytes | type (3 = uncompressed, 4 = RLE), `a4` (0x16 `.sav`, 0x17 `.qst`), item count | all three u32 |
| +12, 1000 × 0x10c | per item: position u32 (+0), size u32 (+4), 260-byte name (+8) | **position up to 4 GB − 1**; size up to 4 GB − 1 (uncompressed in type 4) |
| fixed total | 0x416ec bytes of header and index, rewritten whole by `zclose` (0x932b00) | at most 999 items: `zopen` refuses from 0x3e7 |

- In type 4 the position is in the compressed file and the size is the uncompressed one. The 4 GB limit applies to the compressed position. Compression on = the same limit with a save ~2× smaller.
- **Room in the index** [observed in the 8 saves]: the longest name has 55 characters, and bytes +0x104..+0x10b of every used entry are zero, as are the free entries. There is room for a high part of the position without changing the entry size.
- Item count: 125 to 250 in the measured saves. It grows by ~1 per season (`fixtures_<year>.tmp` stays, one per year: Valencia has 2025 to 2029). The 999 limit would only be reached after hundreds of seasons [inference].

### (b) The container's arithmetic and the file APIs

| Routine | Address | Today | Above 4 GB in the file |
|---|---|---|---|
| `zopen` | 0x9326e0 | Rust (batch 158): unsigned position, seeking by `SetFilePointer` with the high part (`buscar_64`, `posicao_64`) [observed] | reads and writes `pos` as u32; **refuses** the item when the end of file passes u32 (log line "item alem de 4 GB recusado") [observed]. Goes to the end with the CRT's `fseek(f, 0, SEEK_END)` (0x946a22) |
| `zseek` | 0x933080 | Rust | `alvo`, `pos` and `tam` in u32 (`wrapping_add`) [observed]: needs u64 |
| `ztell` | 0x933170 | Rust | `posicao_64(...) as u32 - pos` [observed]: needs u64 before subtracting |
| `zread` | 0x932c30 | original | compares the position in the item (+8) with `size − n`, signed (`jle`/`jg` at 0x932c78/0x932c80) [observed]: **item limited to 2 GB**; does not use the file position |
| `zwrite` | 0x932e10 | original | adds to the size (u32), with no compare [observed]: item up to 4 GB |
| `zclose` | 0x932b00 | original | `fseek(f, 0, 0)` and rewrites the 0x416ec block from memory [observed]: no limit |
| `zextract_matching_files` | 0x933320 | original | reads its own header and index, requires type 3/4 (0x933442/0x933447) and opens each item through `zopen` [observed]: inherits `zopen` |
| `zadd_matching_files` | 0x933610 | original | `fread` of the `.tmp` + `zwrite` in 0x30d40 blocks [observed]: no limit of its own |
| `zget_matching_size` | 0x933860 | original | sums the sizes in an `int` (0x9339c9) [observed]: negative from 2 GB of `.tmp`, wraps at 4 GB. The free-space check in `load_game_data` then always passes or fails by mistake |
| `zget_archive_id` | 0x9331c0 | original | returns header dword +4; `check_saved_game_file` (0x838e10) only lists the save with 0x16 [observed in MAPA_SAVE] |

**CRT APIs** [observed]: `fseek` (0x946a22) and `ftell` (0x946aae) are signed 32-bit `long`. `_lseek` (0x94c552) calls `SetFilePointer(h, offset, NULL, method)`, without the high part. Therefore:
- `ftell` returns −1 from 2 GB;
- `fseek(f, 0, SEEK_END)` on a file above 4 GB asks Windows for a position that does not fit in 32 bits with no high pointer. According to the `SetFilePointer` documentation, the call fails [inference: not tested]. The Rust `zopen` takes that path today and would have to go to the end through `SetFilePointer` with the high part (or `SetFilePointerEx`);
- `fread`/`fwrite` move with the Windows file pointer (64-bit) and do not limit, as long as nobody calls the CRT's `_lseek` in between [inference; `buscar_64` already redoes what `fseek` does to the `FILE`].

### (c) The items extracted to `CM3_TEMP\*.tmp`

On load, `zextract("*.tmp")` writes each `.tmp` item as a file in that folder. From then on, the game reads and writes those files with the CRT (`fopen`/`fseek`/`ftell`, `long` positions). The `.dat` items do not go to the folder: `zread` reads them straight into memory (item (d)).

**The mechanism behind the growing ones: `FILE_LLIST`** (si/code/file_llist.CPP, 0x58e280–0x58f240) [observed]. It is a linked list in a file: each node = a fixed-size record + the offset of the next one (`int`, −1 = end). The heads (first/last offset, `int`) live in each player's or club's objects and are written into the save's `.dat` items.

| Function | Address | What it does with the position |
|---|---|---|
| `FILE_LLIST::FILE_LLIST` | 0x58e280 | stores the path and the record size (0x2b, 0x1b...) |
| `add_node` | 0x58e340 | `fopen("ab")`, `fseek(end)` + **`ftell`** (0x58e3f3/0x58e3f9) and stores the result as first and last **without checking for −1**; then `fopen("r+b")` + `fseek(last + size)` to link the node |
| `add_node_list` | 0x58e9b0 | the same for a whole list (5 `ftell`) |
| `reset_last_offset` | 0x58f110 | last = −1 |
| `get_nodes` | 0x58f120 | `fseek(offset)` and follows the "next" offsets up to the last |

**Breaking point:** once the file reaches 2 GB, `ftell` returns −1. The new node gets first = last = −1, which is the "empty list" mark. That player's history disappears with no error, and the file keeps growing with orphan nodes [observed in the code; the in-game effect is inference]. Reading nodes above 2 GB does not work either, because the offset is a signed `int`.

| File | Owner (constructor) | Measured size | Grows? |
|---|---|---|---|
| `player stats history.tmp` (+ `new_history.tmp`) | `PlayerStats` 0x7bfd50; recompacted on every load in `load_stats` 0x7c48b0 (`get_nodes` → `add_node_list` into `new_history.tmp` → `utils_copy_file` 0x90cc50 → delete) | 66.5 MB (Valencia, 2028) | **yes, the largest** |
| `club records history.tmp` | `Club_Records` 0x444520 | 9.1 MB | yes |
| `staff history.tmp`, `club history.tmp` | `Staff_Records` 0x868390 (two `FILE_LLIST`; ~60 `add_node` calls) | 6.4 MB / 0.1 MB | yes |
| `injury_history.tmp` | `Injury` 0x62a680 | 5.7 MB | yes |
| `ban_history.tmp` | `Discipline` 0x5554a0 | 0.4 MB | yes |
| `club cup/league history.tmp` | `Club_History` 0x442ab0 | 0.1 MB | little |
| `comp records history.tmp` | `Comp_Stats` 0x49ee70 | 0.1 MB | little |
| `past_team_awards.tmp`, `past_year_awards.tmp` | `AWARD_MANAGER` 0x413180 | < 30 KB | little |

**Other `.tmp` files with 32-bit positions that do not grow** [observed in the sizes]:
- `match_stats_matches.tmp` (`Match_Stats` 0x729000, `read_match_record` 0x729600, `write_match_record` 0x729820): fixed size, 34,500,000 bytes in every save from the original database and 69,000,000 in the CLZ ones;
- `human/computer events1/2.tmp` (`Match_Events::save_event_log` 0x6d98b0, `load_event_log` 0x6d9c40): up to ~2 MB, rotated;
- `fixtures_<year>.tmp` (`FIXTURE_MANAGER::save_fixtures` 0x5ac060), `transfers_<year>.tmp`, the competitions' `%s_%d.tmp`, `Friendly.tmp` (0x5c9650): at most ~1.7 MB per year. They grow in number, not in size.

**`utils_copy_file`** (0x90cc50, uses `ftell`/`fseek`) [observed]: called when `player stats history.tmp` is recompacted and in `load_database` (0x5286b1). It breaks when the copied file passes 2 GB.

**The size at which each one breaks:** every `FILE_LLIST` and `utils_copy_file` break when the **`.tmp` file reaches 2 GB** (position 0x80000000). In the container, `zread` limits an item to 2 GB. The first to get there is `player stats history.tmp`.

### (d) Process memory [header observed; the rest is inference]

| Exe | Characteristics | LARGEADDRESSAWARE |
|---|---|---|
| `cm0102.exe` 3.9.68 | 0x10f | no |
| `cm0102-1080.exe`, `cm0102-portatil.exe` | 0x10f | no |
| `clz_teste\cm0102.exe` (NickPatcher) | 0x10f | no |

Without the flag, the process has **2 GB** of address space. The flag is in the exe header and Windows reads it before loading any DLL: our DLL cannot turn it on.

- **Held in memory in full:** the database (the `save_database` tables) and the subsystems' `.dat` items (`transfer.dat`, `player stats.dat`, `staff.dat`, `club records.dat`, `contract.dat`...). Each one's `load` reads the whole item into its structures.
- **Kept on disk, read in pieces:** the `.tmp` files in `CM3_TEMP`. `FILE_LLIST::get_nodes` reads only the list asked for, and `Match_Stats` reads one record at a time.
- **Measured in the indexes:** the non-`.tmp` items add up to 108 MB in Valencia (39% of the save) and 186 MB in TesteTelas. Between Maior2GBPatcher (2026-004) and Valencia (2028-329), ~2/3 of the growth was in `.tmp` files and ~1/3 in `.dat` items.
- **Estimate** [inference]: at 4 GB of file, ~1.3 GB of `.dat` in memory, plus the game, the DLLs, the video surfaces and heap fragmentation. That is close to or above 2 GB. **Memory may be the wall before 4 GB**, and it needs to be measured (phase M, §3.2).

## 2. Measurement (index only)

Saves read (read only: header, index and the 4 date bytes; `FeriasTesteSave.sav` still has MD5 33fda465193b5e8e6d7ace93ecd44559, checked afterwards):

| Save | Game date | Type | Items | Size | Without filler | `.tmp` | `FILE_LLIST` | `player stats history.tmp` |
|---|---|---|---|---|---|---|---|---|
| `jogo\FeriasTesteSave.sav` | 2001-205 | 3 | 142 | 137.6 MB | 137.6 MB | 47.3 MB | 11.0 MB | 10.3 MB |
| `jogo\Lucas.sav` | 2001-354 | 3 | 125 | 124.3 MB | 124.3 MB | 44.3 MB | 8.7 MB | 7.9 MB |
| `jogo\Roteiro.sav` | 2001-213 | 3 | 248 | 240.0 MB | 240.0 MB | 64.5 MB | 24.9 MB | 22.7 MB |
| `jogo\Testes Claude 2.sav` | 2001-275 | 3 | 248 | 252.8 MB | 252.8 MB | 70.1 MB | 27.6 MB | 23.7 MB |
| `jogo\TesteMais2GB.sav` (the "2.55 GB") | 2001-272 | 3 | 249 | 2,550.9 MB | **252.4 MB** | 69.7 MB | 27.5 MB | 23.7 MB |
| `jogo\TesteTelas_Descomprimido.sav` | 2001-332 | 3 | 250 | 259.5 MB | 259.5 MB | 73.0 MB | 29.9 MB | 24.7 MB |
| `clz_teste\Maior2GBPatcher.sav` (CLZ) | 2026-004 | 3 | 138 | 2,445.8 MB | **147.3 MB** | 81.3 MB | 10.9 MB | 10.4 MB |
| `clz_teste\Valencia.sav` (CLZ) | 2028-329 | 3 | 201 | 279.0 MB | 279.0 MB | 171.0 MB | 88.6 MB | 66.5 MB |

[observed] The two saves "above 2 GB" are batch 158's test filler (`0000_enchimento.tmp`, 2,298,478,592 bytes, the first `.tmp`). The real content is 252 MB and 147 MB. **There is no real career above 2 GB among Lucas's files.**

[observed] Largest items in Valencia: `match_stats_matches.tmp` 69.0 MB (fixed), `player stats history.tmp` 66.5 MB, `transfer.dat` 14.9 MB, `club records.dat` 9.8 MB, `club records history.tmp` 9.1 MB, `player stats.dat` 6.9 MB, `staff.dat` 6.9 MB, `staff history.tmp` 6.4 MB, `injury_history.tmp` 5.7 MB. The 10 `.lng` files (1.9 to 2.0 MB each) and the name files do not change.

[observed] Compression: `TesteTelas_Comprimido.sav` (type 4) is 131.7 MB against 259.5 MB for `TesteTelas_Descomprimido.sav` (both with 250 items, saved 16 s apart; same state [inference]): 51%.

**Rate** [inference; few data points and different careers]:
- CLZ, the Valencia career (~3.4 seasons since ~July 2025): `FILE_LLIST` ~23 MB per season (`player stats history.tmp` ~17); whole save ~45 MB per season, assuming about 130 MB at the start.
- Original database with many leagues (the TesteTelas career, 2001-213 → 2001-332): +19.4 MB in 119 days, ~60 MB/year. The span is short and within the season.
- The two give a range of **45 to 100 MB per season** uncompressed. The figure depends more on the active leagues and the database size than on time. Growth should be about linear (records per player and per season), but regens piling up in `staff.dat` may speed it up.

| Limit | When [inference] |
|---|---|
| 2 GB in the file (solved by batch 158) | 20 to 45 seasons |
| the process's 2 GB of memory | to be measured; by the estimate, with the file between ~3 and 4 GB |
| **4 GB in the file** | 40 to 90 seasons uncompressed; ~80 to 180 with compression |
| 2 GB in one item (`player stats history.tmp`, 17 to 35 MB per season) | 55 to 115 seasons; the file would be ~6 to 8 GB |
| 999 items | hundreds of seasons |

## 3. The model of the change

### 3.1 New container format

The entry size (0x10c) and the 0x416ec block stay the same. That way the in-memory `ZFILE` structure (index at +0x25) does not change, and `zread`, `zwrite`, `zadd` and the RLE carry on unchanged.

**Model M (minimal):**
- version mark: header `a4` = **0x18** ("extended save"). Today it is 0x16. `zclose` rewrites the header from memory, so the mark persists on its own;
- high part of the position: u32 at **+0x108** of the entry (the last 4 bytes of the name field). Today it is zero in every measured save. With the mark, names are limited to 255 characters (`zopen` refuses longer ones);
- the type stays 3/4, because `zread`, `zwrite`, `zclose`, `zextract` and `zget_matching_size` test the type.

**Model R (robust, recommended):** M plus the following. In entries beyond 4 GB, the disk holds position = size = **0xFFFFFFFF**, and the real position (low +0x100, high +0x104) and size (+0x108) go into the tail of the name. The Rust `zopen` converts to the normal layout in memory, and the ported `zclose` converts back when writing. Old readers that only know the u32 see an absurd size and fail instead of silently reading garbage.

**Compatibility: write the old format while it fits.** `zopen` (mode 1) only sets the mark and writes the high part when an item's position passes u32. A save of up to 4 GB stays identical to today's and opens in any game, with or without the DLL (up to 2 GB).

**Opening without the DLL or in the tools:**

| Who | What happens | How to refuse clearly |
|---|---|---|
| game without the DLL (or with save2gb 1.0) | `check_saved_game_file` only accepts 0x16: **the save does not even show in the list** [inference from MAPA_SAVE]; no damage | there is no code to show a message. The way out is for the DLL to write `<save>.LEIA.txt` next to it ("this save is above 4 GB: it needs DLL 2.0") |
| save2gb 1.0 / current ddraw | same, not listed | — |
| nckstwrt's SaveChanger/Scouter (`referencias/CM0102Patcher`) | read the type (`== 4` → compressed, anything else → raw), skip +4 and use the u32 position [observed in `SaveReader2.cs`, `Scouter/SaveReader.cs`]. With **M**, they silently read garbage for items beyond 4 GB; with **R**, `new byte[-1]` throws before anything is written [inference] | model R |
| CM3SaveEditor, CMScout (sister projects) | read the u32 index | request through `ORQUESTRACAO.md` §4 (owner rule): recognize `a4 = 0x18` and refuse with a message, or read the high part |

### 3.2 The routines, by phase

**Phase A: file above 4 GB, items below 2 GB** (the case for the next ~50 years of play by the estimate).

| Routine | Address | Change |
|---|---|---|
| `zopen` | 0x9326e0 (Rust) | u64 position (high part at +0x108 with the mark); go to the end with `SetFilePointer` and the high pointer instead of the CRT `fseek`; set the mark when passing u32 instead of refusing; in R, convert the layout |
| `zseek` | 0x933080 (Rust) | `pos`, `alvo` and end of item in u64 |
| `ztell` | 0x933170 (Rust) | `posicao_64 − pos` in u64 (the result, inside the item, still fits in i32) |
| `zget_archive_id` | 0x9331c0 (port) | return 0x16 when the header has 0x18, so the file box lists the save (alternative: `check_saved_game_file` 0x838e10) |
| `zget_matching_size` | 0x933860 (port) | u64 sum, saturating at `i32::MAX` on the way back, so the space check does not pass by mistake (Lucas's decision, like the 2 GB one) |
| `zclose` | 0x932b00 (port, R only) | convert the layout before rewriting the index; includes the RLE tail (already planned in MAPA_SAVE item R1) |

`zextract` and `zadd` do not change: they open each item through `zopen`. Neither do `zread`/`zwrite`, as long as items stay below 2 GB.

**Phase M: memory** (before or along with A, depending on the measurement):
- measure the game's peak private memory while loading saves of different sizes, without clicks (`-load` + `CM_FERIAS`, `psutil`), and extrapolate;
- if it is the wall: set LARGEADDRESSAWARE **in the generated copies** (`patch_portatil.py`, `resolucao.py`; the originals do not change) and prove it with the DLL reserving the lower half of the address space at start, so the heap goes above 2 GB and exposes pointers treated as signed. Wide career Rust = original. On NickPatcher exes the DLL cannot reach the header: that is outside our reach.

**Phase B: items between 2 and 4 GB:**

| Routine | Address | Change |
|---|---|---|
| `FILE_LLIST` (block) | 0x58e280–0x58f240 (`add_node`, `add_node_list`, `reset_last_offset`, `get_nodes` and neighbours) | u32 offsets, with 0xFFFFFFFF as the end; position through `buscar_64`/`posicao_64` |
| callers | `Staff_Records` (~60 `add_node`), `PlayerStats` 0x7c5fe0 and 0x7c3e40, `Injury`, `Discipline`, `Club_Records`, `Club_History`, `Comp_Stats`, `AWARD_MANAGER` | audit: comparisons of the heads with −1 or signed (`jl`/`jg`). They only change if they compare |
| `zread` | 0x932c30 | unsigned comparisons of the position in the item |
| `utils_copy_file` | 0x90cc50 | 64-bit size and position |

**Phase C: an item above 4 GB:** the heads and the "next" offsets are `int` inside the `.dat` and `.tmp` files. It would mean changing the format of 8 subsystems (for example, storing the node index instead of the byte offset) and each one's `load`/`save`. **Not recommended**; by the estimate it is more than 100 seasons away.

### 3.3 What changes in `save\` (the save-only DLL, for NickPatcher exes)

- it uses the same `conteiner.rs` (`#[path]`): phase A's changes come in on their own. `zget_archive_id`, `zget_matching_size` (and `zclose` in R) get hooked as well;
- **new signature:** the range checked today is 0x9326e0–0x933860 and does not cover `zget_matching_size` (it starts at 0x933860). `TRECHOS` (DLL and `assinatura_save.py`) must be extended to 0x9339f5, plus 0x838e10 if that is the hook chosen. Then check 3.9.68, the copies and the CLZ exes again;
- version 2.0 in `save2gb.log` ("saves above 4 GB"), release v2.0 in the public repository `CM0102-Rust/cm0102-save2gb` (source and docs only change at releases), with a warning that a save above 4 GB no longer opens with 1.0 or without the DLL.

### 3.4 What changes in our `ddraw.dll`

- the same routines, in `motor.rs` (the prologue table) and in the comparator, with the new branches ("conteiner estendido", "parte alta lida");
- phase B comes in as batch items of the save type (FILE_LLIST: 3 to 6 routines). The comparator is the **file** one planned in MAPA_SAVE (original and Rust on copies, comparing the bytes);
- phase M, if approved, changes `patch_portatil.py`/`resolucao.py` (the flag in the copy's header) and adds a low-reservation mode to the DLL.

### 3.5 How to prove it (no clicks)

1. **Saving:** `save_grande.py` in the same mould, with **three 1.5 GB fillers** (`0000_a/b/c.tmp`, each below 2 GB, as in phase A), makes a `.qst` above 4.5 GB. `conferir()` reads the high part when the mark is there and checks the sequence of positions and the end of file. With the current DLL the item beyond 4 GB is refused; with the new one it comes out right.
2. **Loading:** a Python builder (new, in `engenharia-reversa/`) makes a synthetic `.sav` from a **copy** of `FeriasTesteSave.sav`. It puts in front a filler item that does not match `*.tmp` (for example `zz_enchimento.bin`, which `zextract` ignores) and marks the file as **sparse** on NTFS, so the 4.5 GB cost neither I/O nor disk (the shared machine suffers under heavy I/O). The proof is `prova_save.py --sav <synthetic>` up to 2001-08-28, which must give the same fingerprint as the normal load (7a1167744a713c73).
3. **Mutations:** zero the high part, do not set the mark, `zseek` in u32. Proof 2 must catch every one of them.
4. **Phase B:** a 2.5 GB synthetic `FILE_LLIST` (sparse, nodes only at the end), read and appended by the original and by Rust on copies.
5. **In the game, with Lucas:** only at the end, a real save with filler above 4 GB, saved and loaded (like batch 158's 2.55 GB test).

### 3.6 Risks

| Risk | Care |
|---|---|
| memory before the format | measure first (phase M); without LARGEADDRESSAWARE, a 4 GB save may fail to load even with the container right |
| the CRT's `fseek(SEEK_END)` above 4 GB | replace it in `zopen` with a seek using the high part; look for other `fseek`/`ftell` on the container path |
| the heap sometimes leaves garbage in the name tail | it is zero today (8 of 8 saves); the high part is only read with the mark |
| old tools reading garbage | model R; warning in the release; request to the sister projects |
| I/O and disk: saving copies every `.tmp` into the container, loading extracts everything, and the load recompacts `player stats history.tmp` (twice its size on disk) | proofs with sparse files; the lab lock; one heavy test at a time |
| the signature changes and save2gb 2.0 stops acting on an exe that alters 0x933860+ | check the known exes before the release |
| saving over it from a game without the DLL: the extended save does not even show, and `save_game_data` only deletes the old file if the name is the same | warning in `.LEIA.txt` |

### 3.7 Estimated cost [inference]

| Phase | Work | Batches |
|---|---|---|
| A (file > 4 GB) | 3 routines changed, 2 ported (3 in R), proofs 1 and 2, save2gb 2.0 | **1 batch** + 1 proof session + release |
| M (memory) | measurement; flag in the copies, low reservation and wide career | 1 measurement session; ~1 batch if needed |
| B (items 2 to 4 GB) | FILE_LLIST + `zread` + `utils_copy_file` + audit of the callers | **~2 batches** |
| C (item > 4 GB) | format of 8 subsystems | 5 or more; not recommended |

**Suggested order:** measure memory (M) → phase A in model R → B only when some real `player stats history.tmp` passes ~1 GB.

## Measurement on a long real save (2026-10-25)

Lucas provided `All Leagues - 3.9.68 - Compress.sav` (unpatched 3.9.68, "All Leagues" database), read only in the index and in `general.dat` [observed]:

- **Game date:** 2063, day 182, i.e. about 62 seasons.
- **File:** 1,431,514,996 bytes (type 4, compressed), with 564 items.
- **Sum of uncompressed items:** 2,147.8 MB, just above 2^31 (2,147.48 MB). The same save uncompressed would pass 2 GB, and would only open with save2gb.
- **Largest items:** `player stats history.tmp` 646.4 MB; `club records history.tmp` 474.2 MB; `injury_history.tmp` 258.5 MB; `staff history.tmp` 186.6 MB; `match_stats_matches.tmp` 69.0 MB; `contract.dat` 40.9 MB; `transfer.dat` 26.4 MB. The `.dat` files together stay well below the histories.

Average rate in this career [inference, a single point, assuming linear growth]: ~35 MB per season uncompressed (~23 MB compressed), below the estimate above. At that rate:

- the uncompressed file would reach 4 GB around season 115 (~2116); compressed, around season 175;
- the largest item (`player stats history.tmp`, ~10.4 MB per season) would reach 2 GB around season 190.

This save is the best real test case for phases A and B: it already has a 646 MB item and the sum at the 2 GB limit.
