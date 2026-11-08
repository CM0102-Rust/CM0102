# Map of the save block

Survey for the save batches (agreed order: match_screens/match_day, **save**, Database, competitions). Read only: disassembly of `cm0102.exe` 3.9.68, the community map (`referencias/cm0102-function-map`), `docs/mapa_funcoes.csv` and the sibling project `D:\Sistemas\CM3SaveEditor`. The container format and the fix above 2 GB are in [SAVE_2GB.en.md](SAVE_2GB.en.md).

Sizes in bytes, measured from the entry to the last byte before the padding (the community map's "size" is not in bytes).

## 1. The functions

### si/code/zipdir.cpp (the container)

None has SEH. All use the game's file CRT (`fopen` 0x945b00, `fread` 0x945bfe, `fwrite` 0x945ce6, `fseek` 0x946a22, `fclose` 0x945a09). The open structure (`ZFILE`, 0xa3191 bytes, `new` 0x944cf1) holds the file at +0, mode +4, logical position +8, item +0xc, RLE state +0x10/+0x14/+0x18, header and index at +0x19/+0x25, read buffer +0x41705 and write buffer +0x72445 (0x30d40 bytes each).

| Address | Name | Bytes | What it does | Callers | Rust | Alloc / file / UI |
|---|---|---|---|---|---|---|
| 0x9326e0 | `zopen` | 1051 | opens an item for reading, or appends one for writing (creates the container with the header) | 114 calls in ~70 routines: the subsystems' `load`/`save`, `load_database`, `save_database`, `zextract`, `zadd` | **yes** (158th, `conteiner.rs`) | allocates 0xa3191; file |
| 0x932b00 | `zclose` | 294 | writing: flushes the buffer, closes the pending RLE run and rewrites the whole header + index (0x416ec bytes) at the start; closes and frees | the same, 288 calls | no | frees; file; single effect |
| 0x932c30 | `zread` | 468 | `fread` inside the item; type 4 decodes the RLE | 521 calls: subsystems, `db_files_read_*`, `load_game_data`, `zextract`, `zseek` | no | file |
| 0x932e10 | `zwrite` | 611 | `fwrite` into the item; adds to the size in the index; type 4 encodes the RLE | 327 calls: subsystems, `db_files_write_*`, `save_game_data`, `zadd` | no | file |
| 0x933080 | `zseek` | 239 | `fseek` inside the item; type 4 rewinds and rereads | `load_database` | **yes** | file |
| 0x933170 | `ztell` | 76 | `ftell` inside the item | `save_database` | **yes** | file |
| 0x9331c0 | `zget_archive_id` | 342 | reads the header and returns dword +4 (0x16 `.sav`, 0x17 `.qst`; 0x14 on failure) | `check_saved_game_file`, `check_quick_start_file` | no | allocates 0x416ec; file; error box |
| 0x933320 | `zextract_matching_files` (unpack) | 746 | for each item whose name matches the pattern: `zopen` + `zread` in 0x30d40 blocks + `fwrite` to a file in the folder | `load_game_data` (`*.tmp`), `load_database` (`*.lng`) | no | allocates 0x3f7a0 (0x944e46) + 0x416ec; creates files; error box |
| 0x933610 | `zadd_matching_files` (pack) | 582 | `file_find_first/next` (0x4c0aa0/0x4c0b50) on the folder; each file becomes an item: `zopen` for writing + `fread`/`zwrite` in 0x30d40 blocks + `zclose` | `save_game_data` (`*.tmp`, type 3 or 4), `save_database` (`*.lng`) | no | file; error box |
| 0x933860 | `zget_matching_size` | 405 | sums the sizes of the items matching the pattern (signed `int`: above 2 GB it goes negative) | `load_game_data` (disk space before unpacking) | no | allocates 0x416ec; file; error box |
| 0x933a00 | `zipdir_match_filename_pattern` | 79 | matches a name against a pattern with `*`, case-insensitive | `zextract`, `zget_matching_size` | no | pure |

The type 4 RLE has no routine of its own: it lives inside `zread`, `zwrite` and `zclose` (and `zseek`, already in Rust, rereads through `zread`).

### cm3/code/Setup.cpp and helpers (save and load)

| Address | Name | Bytes | What it does | Callers | SEH | Automatic career |
|---|---|---|---|---|---|---|
| 0x82e430 | `save_game_data(path, a4)` | 6927 | writes the whole save (flow in §2) | menu (`menubar_handle_command`), `create_game_from_setup_file` (`.qst`) | yes | only through `quick_start.cfg` (`save_grande.py`), type 3 |
| 0x82a9c0 | `load_game_data(path)` | 14950 | loads the whole save (§2) | Setup Game screen handler (0x81bd70, unnamed in the map: Restore and `-load`), network | yes (40 unwinds) | no; without clicks only with `-load <save>` |
| 0x82ff40 | `free_game_data(flag)` | 1500 | deletes every subsystem object and the `*.tmp`, resets the game state | `setup_run_front_end`, `create_game_from_setup_file`, `game_main_day_loop`, `game_fatal_cleanup`, network | no | **yes** (start-up) |
| 0x827110 | `create_game_from_setup_file(cfg, qst)` | 3129 | reads `auto_setup.cfg`/`quick_start.cfg`, creates the game and writes the `.qst` or adds the manager | `setup_run_front_end` | no | **yes** |
| 0x838e10 | `check_saved_game_file` | 152 | file dialog filter: accepts id 0x16 | file dialog (pointer) | no | no (clicks) |
| 0x839290 | `check_quick_start_file` | 165 | filter: accepts id 0x17 and the same version | file dialog (pointer) | no | no |
| 0x838eb0 | `valid_saved_game_version` | 977 | opens `general.dat`, reads version and summary, compares with the exe's | `check_quick_start_file` | yes | no |
| 0x8395b0 | `save_fifa_transfer_rules(zip)` | 421 | writes `fifa_transfer_rules.zip` (2 bytes) into the save | `save_game_data` | no | no |
| 0x839760 | `load_fifa_transfer_rules(zip)` | 436 | reads the same item; rule in force = state 5 | `load_game_data` | no | no |
| 0x591f80 | `build_save_game_summary` (file_screens.cpp) | 712 | builds the `general.dat` summary (nations, managers, date, version, play time) | `save_game_data` | no | no |
| 0x524d30 | `save_database_for_game` (Database.cpp) | 26 | calls `save_database` 0x52dad0 | `save_game_data` | no | no |
| 0x524d10 | `load_database_from_stream` (Database.cpp) | 21 | calls `load_database` 0x528540 | `load_game_data` | no | no |
| 0x90cd50 | `utils_clear_temp_folder` (utils.cpp) | 453 | deletes the files in `CM3_TEMP` in groups of 100 | `create_game`, `load_game_data`, `free_game_data`, `database_free_all_tables` | no | **yes** |

None of these is in Rust. Each subsystem's `load`/`save` routines (~30 pairs, listed in §2) belong to their own files; only `Match_Stats::load/save` (0x72aa80/0x72adf0) are already in Rust.

**Automatic career and zipdir:** the chain baseline records none of the 158th's `conteiner lido/gravado/criado` branches: the holiday career opens no item. The container only runs on save (menu, `.qst`) and on load.

## 2. The flows

### Save (`save_game_data`)

1. Local date, `[0xdd7110] = 0xfffc1fb9`. Under 25 MB free sets `[0xdd7134]` (Disk Full box); free memory < 1 sets `[0xdd7138]`.
2. Network game (bit 0x20000000 of `[0xae3138]+4`): writes to `temp_save_file` and renames at the end; otherwise removes the old save (`remove`).
3. Container type: bit 0x8000 of `[0xae3138]+4` (the `game.cfg` dword +4) set = **3** (uncompressed), clear = **4** (RLE). The header's `a4` is the 2nd argument: 0x16 `.sav`, 0x17 `.qst`.
4. Step 1: `save_database_for_game` → `save_database` writes the database tables (one item per table, `index.dat`) and packs the `*.lng` (`zadd`).
5. Step 2: `general.dat` (16 `zwrite`s: version, summary from `build_save_game_summary`, unique id, database size, date and time, last date and time, leagues, default nation, staff and club runtime data...). Then each subsystem, in this order (between each: disk space, memory and network message 0x802f80):

| Order | Routine | Goes to |
|---|---|---|
| 1 | `HUMAN_MANAGER::save` 0x5ff9e0 | `human_manager.dat` |
| 2 | `SCOUT_MANAGER::save` 0x7f8020 | `scout_manager.dat` |
| 3 | `News::save_to_zip` 0x787c30 | `news.dat` |
| 4 | method +0x1c of `[0xae2c74]` (Friendly, 0x5c9650) | `Friendly.tmp` |
| 5 | `UefaSeeding` 0x905510, `ConmebolSeeding` 0x4c6b70 | `uefa_seeding.tmp`, `conmebol_seeding.tmp` |
| 6 | `Staff_Records` 0x86ffd0, `Match_Stats` 0x72adf0, `PlayerStats` 0x7c55a0, `Club_History` 0x443f50, `Club_Records` 0x451ff0, `Tactics` 0x8995d0 | `staff records.dat`, `match_stats.dat`, `player stats.dat`, `club history.dat`, `club records.dat`, `tactics.dat` |
| 7 | `HostCountry` 0x5fb320 | `HostCountry.tmp` |
| 8 | `Discipline` 0x555dd0, `AWARD_MANAGER` 0x4141a0, `Fine` 0x5a53d0, `AWOL` 0x418980, `DISPUTE` 0x569ae0, `Notes` 0x794f00, `Injury` 0x62b2d0, `NationalTeams` 0x776a20 | `discipline.dat`, `awards.dat`, `fine.dat`, `awol.dat`, `dispute.dat`, `notes.dat`, `injury.dat`, `national_teams.dat` |
| 9 | `Stadium` 0x85bea0, `FIXTURE_MANAGER::save_fixtures` 0x5ac060, `key_nation_save_all_comps` 0x6692f0 | `Stadium.tmp`, `fixtures_<year>.tmp`, `club_dates.tmp`, `check_fixture_legs.tmp`, the competitions' `.tmp` |
| 10 | `CONTRACT_MANAGER` 0x4dc3a0, `FinanceTable` 0x59b3b0, `TRANSFER_MANAGER` 0x8bd4b0, `PLAYER_REGEN` 0x7ad7c0, `Media` 0x72b5f0, `FOG_OF_WAR` 0x5afad0, `MANAGER_MANAGER` 0x68e920, `OfficialsManager` 0x79bb80, `SimulatedStats` 0x84c680, `TRAINING_MANAGER` 0x8b1450, `SQUAD_MANAGER` 0x859cc0 | `contract.dat`, `finance.dat`, `transfer.dat`, `retire.dat`, `media.dat`, `fog.dat`, `man_conf.dat`, `off.dat`, `simulated_stats.dat`, `training.dat`, `squad_manager.dat` |

6. `zadd_matching_files(save, CM3_TEMP, "*.tmp", a4, type)`: every `.tmp` in the temporary folder (those from step 5 and those the game wrote along the way: `history.tmp`, `transfers_<year>.tmp`, `ban_history.tmp`, `injury_history.tmp`, `events*.tmp`...), in `FindFirstFile` order (by name, on NTFS).
7. `HallOfFame::save` 0x5f00c0 (outside the save), `save_fifa_transfer_rules` (the last item), network rename, last-save date at 0xdd70a0.

Real order checked on a 248-item save (CM3SaveEditor, `docs/FORMATOS.md` §4): 20 database tables, `index.dat`, 10 `.lng`, `general.dat`, the subsystems' 29 `.dat`, 186 `.tmp`, `fifa_transfer_rules.zip`.

### Load (`load_game_data`)

1. `CM3_TEMP` folder, CD (`Insert CD`), space and memory.
2. `database_free_all_tables` 0x528080 and `load_database_from_stream` → `load_database`: each table through `zopen`/`zread`/`zseek` and `zextract("*.lng")`.
3. `utils_clear_temp_folder`, `zget_matching_size(save, "*.tmp")` against free disk ("unable to unpack the temporary files"), `zextract_matching_files(save, CM3_TEMP, "*.tmp")`.
4. `general.dat` with `zread`, in the save order, with one error message per field.
5. Key nation table (0x667150), and the subsystem constructors, which read their own `.dat` from the save (they get the `ZipDir`) or their `.tmp` from the folder: `Tactics`, `HUMAN_MANAGER`, `SCOUT_MANAGER`, `News`, `Friendly`, `UefaSeeding`, `ConmebolSeeding`, `Staff_Records`, `Match_Stats`, `Club_Records`, `PlayerStats`, `Club_History`, `HostCountry`, `Discipline`, `AWARD_MANAGER`, `Fine`, `AWOL`, `DISPUTE`, `Notes`, `Injury`, `NationalTeams`, `VIRTUAL_STAFF`, `Stadium`, `FIXTURE_MANAGER`; `key_nation_create_all_comps` 0x669180 (competitions, from the `.tmp`) and `build_comp_object_table` 0x8386f0; then `FinanceTable`, `CONTRACT_MANAGER`, `TRANSFER_MANAGER`, `PLAYER_REGEN`, `MANAGER_MANAGER`, `Media`, `FOG_OF_WAR`, `OfficialsManager`, `SimulatedStats`, `Weather`, `TRAINING_MANAGER`, `PunditList`.
6. `load_fifa_transfer_rules`; without `auto_setup.cfg`, the "CD Not Required" box.

### The RLE (type 4)

| Byte written | Meaning |
|---|---|
| `b ≤ 0x80` | the byte itself (literal) |
| `b > 0x80` | `b − 0x80` copies of the next byte (1 to 0x7f) |

- **Encode (`zwrite`):** keeps the run byte at +0x18 and the count at +0x14, across calls. On a different byte: a run of 1 with a byte ≤ 0x80 is written as a literal; otherwise the pair (`0x80 + n`, byte). On reaching 0x7f, the pair is written and a new run starts. `zclose` closes the pending run the same way.
- **Decode (`zread`):** a run may span calls (+0x10 already delivered out of +0x14).
- The index holds the position in the file and the **uncompressed** size; the logical position (+8) counts uncompressed bytes.
- 0x30d40-byte buffers: when full, written out and restarted at byte 0.
- The header and the index stay raw (no RLE) at the start of the file.

## 3. What the CM3SaveEditor offers (read only, owner rule)

| What | Where | Use |
|---|---|---|
| Container reading (header, 1000 entries of 268 bytes, name) | `rust/cm3sav/src/lib.rs`: `Sav::new`, `n_arquivos`, `entradas` | checking the `zadd`/`zclose` output in tests (same format as the game) |
| Byte-for-byte round trip (extract the 248 items and rebuild from the manifest only: same SHA-256) | `testes/teste_container.py --viagem`, `engenharia-reversa/sav_lista.py` (validates the model), `sav_extrai.py`, `sav_info.py` | proves the table has 1000 slots, contiguous data, no tail: oracle for type 3 packing |
| Real item order and sizes | `docs/FORMATOS.md` §2–§4 | checking the order in §2 |
| RLE: only refuses type 4 (`teste_recusa_comprimido.py`) | — | nothing; the decoder is in CMScout: `D:\Sistemas\CMScout\rust\olheiro\src\rle.rs` (`descomprimir`, proven in `tests/comprimido.rs` against the uncompressed save of the same game). No sibling has an encoder: the only reference for `zwrite` is the game |
| Database record and `.dat` structures | `pessoa.rs`, `clube.rs`, `contrato.rs`, `nacao.rs`, `competicao.rs`, `arbitro.rs`, `preferencias.rs`, `scout.rs`, `aposentadoria.rs`; `docs/CAMPOS.md`, `docs/REGISTROS.md`; `extrair_finance/contract/injury/retiring` in `lib.rs` | the Database block and the subsystems' `.dat`, not this one |
| Not useful | `achar` (51 upper-case characters), `extrair_como_editor` (0xFFFA blocks), `gravar_como_editor` (bug E2), VB rounding | editor behaviour, not the game's (`zopen` compares the whole name with `strcmp`) |

The dword +4 that the editor calls unknown is `zopen`'s `a4`: 0x16 `.sav`, 0x17 `.qst` (`zget_archive_id`, `check_saved_game_file`).

## 4. Proposed batches

New item type (**save**), next to screens and simulation. 21 routines in 7 items and 4 batches.

| Batch | Item | Routines | Expected comparator |
|---|---|---|---|
| S1 | C1 container: queries | 0x933a00 pattern, 0x9331c0 id, 0x933860 size | pattern: pure `side_by_side` (+ self-test with names, `*`, case); id and size: both on the same file (read only), compare the result |
| S1 | C2 pack/unpack | 0x933320 `zextract`, 0x933610 `zadd`, 0x90cd50 clear `CM3_TEMP` | **file** (new): original and Rust on copies (folders A and B in `jogo_testes`), compare result, file list and bytes of the files produced |
| S1 | R1 RLE | 0x932c30 `zread`, 0x932e10 `zwrite`, 0x932b00 `zclose` | **file**: two `ZFILE`s on copies of the same container; compare buffer, result, the structure without the `FILE*` (+0) and, on `zclose`, the whole file. Self-test with types 3 and 4, runs of 1 to 300, bytes > 0x80, 0x30d40 buffer wraps, runs spanning calls; reading a real compressed save (copy of `TesteTelasComprimido.sav`) comparing every item |
| S2 | S1 checks | 0x838e10, 0x839290, 0x838eb0 | `side_by_side` on a fixed `.sav`/`.qst` (read only); self-test, since only the file dialog calls them |
| S2 | S2 single items | 0x8395b0, 0x839760 FIFA rules; 0x591f80 summary | rules: **file**; summary: `side_by_side` of the block (masking play time if it is wall clock) |
| S2 | S3 glue and cleanup | 0x524d30, 0x524d10, 0x82ff40 `free_game_data` | glue: pass-through (validated by save and load); `free_game_data`: orchestrator, career |
| S3 | S4 save | 0x82e430 `save_game_data`, 0x827110 `create_game_from_setup_file` | orchestrator without comparator: `.qst` written by the original and by Rust from the same state, compared byte for byte (types 3 and 4); `create_game_from_setup_file` also by the career |
| S4 | S5 load | 0x82a9c0 `load_game_data` | orchestrator: `-load <qst>` + holiday career, Rust print = original; mutations through the chain |

**Test bench for the block:** `engenharia-reversa/prova_save.py` (`docs/COMO_CONTINUAR.en.md`, "Save proof"): `quick_start.cfg` with compression on (`game.cfg` bit 0x8000 cleared, copy and restore) writes a type 4 `.qst` in both modes and compares item by item. The load cannot use the `.qst` (no manager, `-load` stops at "Enter Name"): it uses a `.sav` with a manager on holiday, made once by Lucas. The current career goes through no zipdir routine.

## 5. Risks

| Risk | Where | Care |
|---|---|---|
| Single effect | `zclose` (closes and frees), `zextract`/`zadd`/clear temp (create and delete files), `save_game_data` (deletes the old save), `load_game_data` and `free_game_data` (build and destroy everything) | compare on copies; orchestrators only through the career |
| Error boxes (0x5e8290) | `zget_archive_id`, `zextract`, `zadd`, `zget_matching_size`, Setup | the self-test must not hit error paths, or it hangs waiting for a click |
| Heap garbage in the file | `zopen` creating writes 0x416ec bytes of header + index from a non-zeroed allocation (the entries past the count); `zclose` rewrites them | today it comes out zero (large heap block); the comparator checks the whole header |
| `FindFirstFile` order | `zadd`, clear temp | Rust uses the same 0x4c0aa0/0x4c0b50 |
| Signed `int` | `zget_matching_size` above 2 GB goes negative and the space check always passes | keep it identical; a fix is a separate decision (like the 2 GB one) |
| Stack garbage | `save_game_data` (0x175c frame), `load_game_data`, `valid_saved_game_version` (SEH) | the routine-by-routine comparison by outcome (no zeroing since 2026-10-31: stack garbage is not compared) |
| Non-determinism | summary (play time), date and time in `general.dat`, disk space, CD | check on the first proof; mask if wall clock |
| Save DLL | `save/` uses the three routines in `conteiner.rs` and the signature of 0x9326e0–0x933860 | the new routines stay out of `save/`; the public repository only changes on releases |
| Large allocation | each `zopen` allocates ~650 KB, `zextract` another ~530 KB | changes the heap and the print: compare only in the save step |

Border with the Database block: `save_database` 0x52dad0, `load_database` 0x528540, `database_free_all_tables` 0x528080 and the `db_files_read_*`/`db_files_write_*` (Db_files.cpp, 0x54d5e0–0x550950), all on top of `zread`/`zwrite`.

## Lead: out-of-table ids in the saved squad (2026-10-23, from the Save Editor validator study)

Source: CM3SaveEditor\docs\VALIDADOR_ESTUDO.md (commit bd7959e), passed on by the orchestrator session; marked as inference there. The game itself writes into the club.dat squad ids ≥ N (outside the people table): 51 slots in TesteTelas.sav (club 1109, ids 159578..159597; club 3397, 158727..158734) and 22 in Valencia.sav. Between TesteTelas and TesteTelasComprimido, club.dat changes only in those slots (they become −1) and staff.dat stays the same: a reload between the two, with `resolve_database_indexes` emptying the slots. Hypothesis: youngsters created in the reserve rows (the 23,000 of type 6 or the 1,325 uncounted) that `save_database` (0x52dad0) does not carry into staff.dat, while their id stays in the squad. A candidate cause of the id corruption the Save Game Editor repairs (PROJETO.md, item 8). Check once save_database and the load are in Rust: the copy shows exactly which people go into staff.dat and which ids stay in the squads.

## 6. A competition's `.tmp`: cups and leagues (2026-10-30)

Each competition writes its state to a `.tmp` in the `CM3_TEMP` folder (`docs/.en.md` §2.11), and these files go into the save as items. The order below is that of `Cup::save_cup_to_disk` (0x5210F0), read from the disassembly. The load, `Cup::load_from_disk` (0x51FC00), reads in the same order. Offsets with `+` belong to the cup object (0xB2 bytes) and "u16"/"u32" are signed little-endian integers.

**Proof** [observed]: `engenharia-reversa/tmp_competicao.py <save>` follows this order and checks that each file ends exactly where the reading ends.
- **All 164 competition `.tmp` files** of `Testes Claude 2.sav`, `Roteiro` and `TesteTelas` close (58 in `FeriasTesteSave`, a small save). Only the 20 global tables and the `fixtures_<year>.tmp` files are left out.
- Among the cups (59 per save) are the super cups (1781 bytes), the domestic cups of 1 to 9 rounds (up to the French Cup, with 222 entrants) and the cups with stages. Among these, the Champions League has 8 league stages, the Swedish Cup 8 cup stages and the Asian Champions Cup has both types.
- The entrant ids are `club.dat` clubs: the English Super Cup gives Arsenal × Liverpool, the Intercontinental Bayern × Boca, the Portuguese Super Cup Sporting × Leixões.
- Mutations in the reader bring the count down:
  - in the cup: dropping +0x562 from the statistics, a 0x34-byte tie instead of 0x35, dropping +0xab: from 164 to 140, 82 and 105;
  - in the stages and the league table: a 0x40-byte league round, head-to-head with n records, dropping +0xDB, dropping +0x20 from the cup stage: to 57, 130, 57 and 137.
- The other 105 are leagues: the national divisions, the "cups" that are Leagues (below), the national-team tournaments and the qualifiers.
- Mutations in the league part (dropping +0xE6, +0xD1 with 4 bytes, writing the table even without teams, dropping +0x140 from EuroChamp) bring 164 down to 59, 146, 152 and 163.

| Order | Field | Size |
|---|---|---|
| 1 | file name (+0x53) | 0x50 |
| 2 | there is a statistics block (+0x08 ≠ 0) | u32 |
| 3 | the block, if any: `Comp_Stats::save` (0x4AC4E0), see below | variable |
| 4 | the derived class's own data: vtable slot 10 (+0x24); empty in the base (0x48CEA0 = `ret 4`); table below | variable |
| 5 | prize fund +0x28 | u32 |
| 6 | +0x52, +0x51, +0x4B | 3 × u8 |
| 7 | +0x4C, +0x24 (the competition's stadium, used by the home rule, §2.1 of the reference) | 2 × u32 |
| 8 | bench +0x49, substitutions +0x4A | 2 × u8 |
| 9 | +0x45 (the draw only runs when it is ≠ 0, 0x51C431) | u32 |
| 10 | +0xB1 | u8 |
| 11 | rounds +0x3C, entrants +0x3E, year +0x40 | 3 × u16 |
| 12 | +0xAB, +0xAC, type +0x42 | 3 × u8 |
| 13 | main stage code +0x3A | u16 |
| 14 | league above +0x1C, below +0x20 | 2 × u32 |
| 15 | **the pointer** of the average-points table +0x18 (the memory value, written as is) | u32 |
| 16 | average rows +0x34 and, if the pointer is not null, 0x13 bytes per row | u16 + variable |
| 17 | competition id (record +0, the same as in the file name) | u32 |
| 18 | rounds: 0x68 bytes each (layout in the reference, §2.4) | 0x68 × +0x3C |
| 19 | entrants (+0xA7): 6 bytes each; the club pointer becomes the **id** when saving and is turned back into a pointer afterwards | 6 × +0x3E |
| 20 | ties of each round (+0xAD): 0x35 bytes per match, as many as the round's +0x1A says | variable |
| 21 | stages +0x2C, current stage +0x30, +0x44 | u32 + u32 + u8 |
| 22 | for each stage from 0 up to the current one: type (+0x43 of the stage); if 1, cup stage: +0x3E, +0x3C, each round's +0x1A (u16) and `Cup_Stage::save_to_file` (0x523EB0); if 2, league stage: +0x3E, +0xA9 and `League_Stage::save_to_file` (0x68CA40) | variable |
| 23 | entry book: capacity +0x38, used +0x36 (u16 each) and, if it exists, 6 bytes per capacity slot, with the clubs as ids | variable |

**Statistics block** (`Comp_Stats::save`, a 0x5CE-byte object), in order: +0x10 (u32); +0x14 (u32, n) and n × 0x76 from +0x0C; +0x1D (u32); +0x21 (u32, n) and n × 0x2D from +0x19; 0x64 from +0x56A; 0x269 from +0x26 and 0x269 from +0x28F (the two record sets); +0x4F8, +0x4FC (u32); +0x500 (u16); 0x5C from +0x506; +0x562 (u32); +0x04 (u32, n); +0x08 (u32); n × 0x1C from +0x00. Without tables, 0x5B8 bytes.

**What it tells us** [inference, to be checked]:
- The `.tmp` is not a copy of the object: it is a list of chosen fields, and club pointers become ids. The exception is +0x18, written as a memory address; that is the "pointer" the other session found in some `.tmp` files.
- Tables are only written if the pointer exists, and the file does not keep that flag: the reader assumes an empty table when the count is 0, which closed in every case above.
- For the editor: in a simple cup, entrants and ties sit at computable positions, and swapping a club is swapping an id. Changing the number of rounds or entrants changes the sizes and requires rewriting the whole file.
- Still missing: the meaning of several fields known only by offset. The tie and the league round are in §6.1 and §6.2.

**Stages** [observed at 0x523EB0 and 0x68CA40; proven by the reader].
- Step 22 writes, for each stage, the type and a small header, and calls the stage's own save.
- **Cup stage** (`Cup_Stage::save_to_file`):
  - +0x28 (u32); +0x51, +0x49, +0x4A, +0xB1 (u8); +0x3C, +0x3E, +0x40 (u16); +0xAB, +0xAC, +0x44, +0x42 (u8); +0x3A (u16); +0x1C, +0x20 and the competition id (u32);
  - then the rounds (0x68), the entrants (6, club as id) and the ties (0x35 per match), as in the cup.
- **League stage** (`League_Stage::save_to_file`):
  - +0xE1 (u8); +0x28, +0xDD (u32); +0xA7 matches per team (u16); +0x51, +0x49, +0x4A (u8); +0x3C rounds of play, +0x3E teams, +0x40 year (u16); +0x43, +0x42 (u8); +0x3A (u16); the competition id (u32); +0xA9 rounds (u16);
  - points for a win and a draw +0xC2, +0xC3 and +0xC4 (u8); the four tie-breaks +0xC5 and the four places +0xBE (4 bytes each); the +0xD9 bits, +0xDB and +0xAB (u16); +0x1C, +0x20 (u32); +0x44 (u8);
  - the league rounds (0x41 × +0xA9);
  - the points average, as in the cup;
  - the table (0x3B per team, club as id);
  - a u32 saying whether there is head-to-head data (+5 of each row) and, if so, n − 1 records of 0x0E per team, with the opponents as ids;
  - a u32 saying whether there is the per-round position history (+9 of each row) and, if so, +0xA7 bytes per team. If +0xA7 is −1, the size is (n − 1) × +0x3C.
- **The same routine serves leagues**: the table, round and points fields are those of §2.1 of the competitions reference. It is the basis of the next phase, the league `.tmp` (`League::save_to_temp_file`, 0x6891C0).

**Class-specific data** (vtable slot 10, written right after the statistics) [observed; each file's class is the variant that closes]:

| Class | Routine | What it writes |
|---|---|---|
| `Cup` and most derived ones | 0x48CEA0 | nothing |
| `UefaCup`, `EngCcCup`, `ScoLgeCup` | 0x903340 | +0xB2 (u8) |
| `IntertotoCup` | 0x633F50 | +0xB2 (u32) |
| `EngFaCup` | 0x5708A0 | +0xB3, +0xB4, +0xB2 (u8, in that order) |
| `MiniCup` (4-club pre-season tournament) | 0x7664C0 | start date (8 bytes from +0xB2) and +0xBA (u16) |
| `WcAfricanCup` | 0x911110 | +0xBA (u8), count +0xB6 (u32) and that many clubs (u32, as ids) |
| `WcConcacafCup` | 0x917CD0 | +0xBA, +0xBB, +0xBC (u8), count +0xB6 (u32) and the clubs (u32, as ids) |

Several derived leagues also have their own slot 10 (`RusPrm`, `GreCup`, `EuroChamp`, `WorldCup`, `SpaFirst`, `HolFirst`...), for the league phase.

**Not every "cup" is a Cup**: the Italian Cup (`ItaCCup`), the Dutch Cup (`HolCup`), the Greek Cup (`GreCup`), the Irish, Northern Irish, Welsh and Korean league cups, the Libertadores, the Mercosur, the Merconorte and the national-team tournaments are League objects, written by `League::save_to_temp_file`. That is why they do not close by the cup order, but by the league one.

**Leagues** (`League::save_to_temp_file`, 0x6891C0) [observed; proven by the reader].
- **Order**: name (0x50), statistics flag and block, class data (slot 10), as in the cup. Then the header:
  - +0xE2 minimum capacity, +0xE4 minimum seats (u16); +0xE1 (u8); +0x28 prize, +0xDD (u32); +0xD1 (8 bytes); +0xA7 matches per team (u16);
  - +0x52, +0x51, +0x4B (u8); +0x4C, +0x24 (u32); +0x49 bench, +0x4A substitutions (u8); +0xB6, +0xB8 (u16); +0x45 (u32); +0xB5 (u8);
  - +0x3C rounds of play, +0x3E teams, +0x40 year (u16); +0x43, +0x42 (u8); +0x3A (u16); the competition id (u32); +0xA9 rounds (u16);
  - +0xC2, +0xC3, +0xC4 (u8); +0xC5 tie-breaks, +0xBE places (4 bytes each); +0xD9, +0xDB, +0xAB (u16); +0x1C, +0x20, +0xE6 (u32).
- Then the same table part as the league stage: rounds (0x41), average, table, head-to-head and position history. After that, the same stages and the same entry book as the cup.
- **Without teams** (+0x3E ≤ 0, tournament not yet built), the save skips the table and the two flags (`jle` at 0x68964F and 0x68CCE9). That is the case of the 1852-byte national-team tournaments.

**Derived leagues' own data** (slot 10):

| Class | Routine | What it writes |
|---|---|---|
| `League`, `SpaFirst` (its routine calls another object and writes nothing) | 0x48CEA0, 0x850950 | nothing |
| `RusPrm`, `CroA1`, `CroA2a/b`, `ItaSerA/B`, `WcSouthAmericanLeague` | 0x7EC1F0 | +0xEE (u8) |
| `HolFirst` | 0x5F75B0 | +0xEE..+0xF1 (4 × u8) |
| `MiniLeague` | 0x766D90 | 8 bytes from +0xEE |
| `BelSecond` | 0x420E90 | +0xF6..+0xF8 (u8) and 8 bytes |
| `FinFirst` | 0x594FD0 | 16 bytes from +0xEE |
| `BelThird` | 0x424280 | +0xFE, +0x100, +0x102 (u16) and 16 bytes |
| `SweSecond` | 0x893700 | 48 bytes from +0xEE |
| `IrePrm` | 0x63ACB0 | 50 bytes from +0xEE |
| `GreCup`, `HolCup` (0x5F5070), `IreLgeCup`, `ItaCCup`, `KorLeagueCup`, `NirLgeCup`, `WelLgeCup`, `WelPrmCup` | 0x5E9FB0 | the auxiliary list: count (+0xF2, u32) and the clubs (+0xEE, u32 as ids) |
| `EuroChamp` | 0x57C5D0 | 80 bytes from +0xEE, +0x13E, +0x140 (u16) |
| `WorldCup` | 0x92F150 | 160 bytes from +0xEE, +0x18E, +0x190 (u16) |
| `WcAsiaLeague`, `WcOceaniaLeague` | 0x914410 | +0xEE (u8), count (+0xF3, u32) and the clubs (u32 as ids) |
| `EuroChampQual` | 0x580410 | count (u32, rewritten at the end through `fsetpos`) and that many 0x4F records; +0xF6 (u8); +0xFB (u32) |
| `WcEuropeLeague` | 0x91AE80 | +0xF6 (u8); count and 0x4F records, as above; +0xFB (u32); the auxiliary list (+0xF2 count, +0xEE clubs) |

**The class is not in the file.** The game knows it because it builds each competition by id; the reader deduces it from the variant that closes. Two files close under two classes:
- the Dutch first division, under `HolFirst` and under `GreCup` (the 4 bytes work as an empty list);
- the Northern Irish League Cup, under `GreCup` and under `EuroChamp`.

The variant order resolves both in favour of the real class. A mutation that shortens a field is sometimes absorbed by another variant: +0xD1 with 4 bytes still lets 146 of 164 close. That is why the proof measures the drop in the count, not just zero.

### 6.1 The cup tie (0x35 bytes)

[observed in `Cup::map_fixture_tree` 0x518790 (each tie's reset), `Cup::set_home_teams_for_round` 0x51E870 (the home swap, which swaps the fields in pairs) and `Cup::decide_tie_winner` 0x51F2F0; meanings checked on real ties of the save: European Super Cup Liverpool × Bayern 0–0, 0–0 after extra time and 5–6 on penalties; UEFA Cup and Champions League two-legged ties]. "A" is the home side of the first match and "B" the away side.

| Off. | Size | Field |
|---|---|---|
| +0x00 / +0x02 | u16 / u16 | side A / side B entrant (index in the round's entry book, −1 = not yet known) |
| +0x04 / +0x05 | u8 / u8 | the result mark of side A / side B in the tie's last match: `process_fixture` copies here the match's +0x3D/+0x3E, put in A/B order (0x51A4EE..0x51A524) [observed]. The match gets those marks from `Cup::decide_fixture_winner` (0x51C020) and `decide_tie_winner` (0x51F2F0), following the match's +0x38 rules: 0x10, a draw is worth 1 and a win 2; 0x2, the winner in extra time; 0x2000, minus 3; 0x800/0x1000, walkover = 8 [observed; the meaning of each value, to be checked]. The reset does not clear the field: before the first match it holds whatever was in memory. The 6 seen in the European cups is one of those marks, not the entry class |
| +0x06 | u16 | next-round tie that receives the winner (−1 = none: final, or the place goes to a stage) |
| +0x08 | u16 | the winner's side in that tie (0 = A, 1 = B) |
| +0x0A / +0x0C | u16 / u16 | previous-round tie that side A / side B comes from (−1 = enters directly) |
| +0x0E / +0x10 | u16 / u16 | round that side A / side B comes from |
| +0x12 | u16 | next round (−1 = none) |
| +0x14..+0x1D | 10 × u8 | **first match**, in (A, B) pairs: normal time, half time, extra time, penalties, aggregate; 0xFF = did not happen |
| +0x1E..+0x27 | 10 × u8 | **second match** (second leg or replay), in pairs from its home side's view (B, A): the same five pairs; the aggregate only exists here |
| +0x28..+0x31 | 10 × u8 | **third match** (the round's +0x11 tie-break), same format |
| +0x32 | u8 | replays already played: `process_fixture` adds +0x32 and +0x33 to pick the block that receives the result (0 = first match, 1 = second, 2 = third; 0x51A3CE) |
| +0x33 | u8 | which match comes next (0 = the first; 1 = the second, written at 0x51A57F after the first) |
| +0x34 | u8 | state: 0 = matches not yet created (the draw only runs if the round's first tie is at 0); 3 = decided |

Example (UEFA Cup, first round): the first match holds `02 00 01 00 ff…` (2–0, half time 1–0) and the second `00 01 00 01 ff ff ff ff 00 03` (B 0–1 A, half time 0–1, aggregate B 0–3 A).

### 6.2 The league round (0x41 bytes)

[observed in `league_round_set_date` 0x68A160 and `league_round_set_match_slot` 0x68A1C0; checked on the 46 rounds of the save's English First Division: Saturdays from August to May, a bank-holiday Monday in the evening, TV slots on Sunday and Monday]

| Off. | Size | Field |
|---|---|---|
| +0x00 | u16 | day of the year (0 = 1 January), already moved to the requested weekday |
| +0x02 | u16 | year relative to the season's (+0x40 of the object): 0 or 1 |
| +0x04 | u8 | time of day: 0 morning, 1 afternoon, 2 evening |
| +0x05 + 7·i | 8 × 7 bytes | TV slots (i = 0..7): value (u32), weekday (u8, 0 = Monday; 0xFF ends the list), time of day (u8), option (u8, 0..4) |
| +0x3D | u32 | venue (the `docs/CAMPOS.md` table: 0 home side, 4 national stadium, 8 neutral...) |

## 7. The global `.tmp` files (2026-10-30)

The `.tmp` files that do not belong to a competition: 22 in the test save (25 in `TesteTelas`, which has one more fixtures year and two more files). Found through the exe references to each name; each layout comes from the routine that writes it.

**Proof** [observed]: `engenharia-reversa/tmp_globais.py <save>` reads each file in that order and checks that the reading ends at the last byte.
- They all close: 22 in `Testes Claude 2`, `Roteiro` and `FeriasTesteSave`, and 25 in `TesteTelas`. Together with the 164 competition ones (§6), **every `.tmp` in the save has a known layout**.
- Mutations (`staff history` record of 0x34, UEFA final table with 49 records, the friendlies' fixed record of 0x68, dropping the stadiums' final u32, match record of 0x4E, 9999 slots in `match_stats_matches`): each one brings down exactly the affected file (two fixtures years, for the match record).

**Linked-list histories** (`FILE_LLIST`, 0x58E280):
- The file is a sequence of nodes; each node is the record followed by the offset of the next node (i32, −1 on the last).
- `FILE_LLIST::add_node` (0x58E340) appends at the end and links the previous node.
- Several owners (each club, each person) keep their own list in the same file. Each list's first and last offsets live in the owner, outside the file.
- The record size is the constructor's 2nd argument:

| File | Record | Owner |
|---|---|---|
| `past_year_awards.tmp` | 0x4A | `AWARD_MANAGER` 0x413180 |
| `past_team_awards.tmp` | 0x205 | `AWARD_MANAGER` 0x413180 |
| `club league history.tmp` | 0x15 | `Club_History` 0x442AB0 |
| `club cup history.tmp` | 0x0B | `Club_History` 0x442AB0 |
| `club records history.tmp` | 0x38D | `Club_Records` 0x444520 |
| `comp records history.tmp` | 0x26B | `Comp_Stats` 0x49EE70 |
| `ban_history.tmp` | 0x14 | `Discipline` 0x5554A0 |
| `injury_history.tmp` | 0x2C | `Injury` 0x62A680 |
| `player stats history.tmp` | 0x2B | `PlayerStats` 0x7BFD50 (loading old saves converts 0x1B-byte records, `new_history.tmp`) |
| `staff history.tmp` | 0x35 | `Staff_Records` 0x868390 |
| `club history.tmp` | 0x31 | `Staff_Records` 0x868390 |

**Tables written in one go**:

| File | Routine | Layout |
|---|---|---|
| `fixtures_<year>.tmp` | `FIXTURE_MANAGER::save_fixtures` 0x5AC060 | count (u32, rewritten at the end) and one 0x4F-byte match record per match of the year. It is the NickDLL's `MATCH_DATA` (competitions reference, §2.8): club ids and pointers, dates, stage, match number, scores and substitutions |
| `club_dates.tmp` | `fix_man_save_club_dates` 0x5AF040 | one u16 per club, from [0xAE2A5C]; count = [0xAE235C] |
| `check_fixture_legs.tmp` | `fix_man_save_fixture_leg_checks` 0x5AF4A0 | count (u16) and 0x0F bytes per item |
| `HostCountry.tmp` | `HostCountry::save_temp_file` 0x5FB320 | count (u16) and 0x22 bytes per host |
| `conmebol_seeding.tmp` | `ConmebolSeeding::save_to_temp_file` 0x4C6B70 | count (+0x6E, u8), 0x0A per country, +0x6F (u16), 4 bytes |
| `uefa_seeding.tmp` | `UefaSeeding::save_to_temp_file` 0x905510 | count (+0x438, u8) and 0x12 per country (`UEFA_SEEDINGS`, reference §2.9), +0x439 (u16), count (+0x43F, u8) and that many u32 from [+0x43B], and 50 fixed 0x0C records from +0x440 |
| `Stadium.tmp` | `Stadium::save_to_temp_file` 0x85BEA0 | +0x08 (u32); count (u32) and 0x0C bytes each from [+0x00]; +0x14 (u32); count (u32); +0x18 (u8); 0x10 bytes each from [+0x0C], clubs as ids; the global u32 [0xA835A4] |
| `Friendly.tmp` | `Friendly::save_to_temp_file` 0x5C9650 | +0xA7 (8 bytes), +0xAF (u16), +0x34DC (u32), count (u32), +0x34E4 (u32) and 0x14 bytes each from [+0x34D8]; 127 fixed 0x69 records from +0xB1 (clubs as ids); four lists with the count (u32) rewritten at the end: [+0x34C8] of 0x19, [+0x34D0] of 1 byte (+0x64 of the item), one of 8 + 4 and [+0x34CC] of 0x107; +0x34F4 (u16); and one list per nation ([0xAE2350] nations), each with a count (u16) and u32 values |
| `computer events1/2.tmp`, `human events1/2.tmp` | `Match_Events::save_event_log` 0x6D98B0 | rolling file: a 3-u32 header (the 3rd, at +8, rewritten for each match) and each match's 0x0E-byte events, appended at the end. Where each match starts (offset, event count, which of the two files) lives in memory, not in the file |
| `match_stats_matches.tmp` | `Match_Stats` 0x729000 | 10000 fixed slots of 0xD7A bytes (34,500,000 bytes), read and written by index (0x729600 / 0x729820); the index lives in `match_stats.dat` |

`transfers_<year>.tmp`: `TRANSFER_MANAGER::clear_old_past_transfers` (0x8C3EB0) opens the year's file in "ab" and appends the old transfers that leave memory, 0x25 bytes each; `get_past_transfers` (0x8CC030) reads them back. Record: +0x00..+0x17 six u32 (person and clubs, to be named), +0x18 fee (u32), +0x1C type (u8), +0x1D date (the 8-byte `Date`: day of year u16, year u16, leap flag). Proven on the 32 files of the `All Leagues` save (§7.1): all on the grid, and each record's date falls in the year of the file name (2032 with the leap flag).

The transfer record (0x25 bytes) [observed in `transfer_screens_build_transfers_screen` 0x8F5080, which builds the transfers screen, and in the routine that writes it, 0x8C2789; counted on the 27,836 records of 2010 in the `All Leagues` save]:

| Off. | Size | Field |
|---|---|---|
| +0x00 | u32 | the person |
| +0x04 / +0x08 / +0x0C | 3 × u32 | first name, surname and common name ids (`database_format_name_from_ids`); 0 = no common name |
| +0x10 / +0x14 | 2 × u32 | club from / club to (−1 = no club) |
| +0x18 | u32 | fee |
| +0x1C | u8 | type (the screen shows the money amount for one type and a label for the others) |
| +0x1D | 8 bytes | date (`Date`) |

The types in the data: **0** is the only one with a fee (5,222 of 11,086 with fee > 0; 5,864 with destination −1, leaving without a club); **2**, fee 0, is what the negotiation writes for the exchange player (0x8C27B4); **3** always has origin −1 (signing someone without a club); **4**, fee 0 with both clubs [loan, inference]; **5** comes from `MANAGER_MANAGER::daily_update` (0x69043F), the managers (destination −1 on a sacking).

**Stack leftovers in the match record** (2026-11-01) [observed in the code]: whoever creates the match builds the record in a 0x4F local and does not write all of it, and the leftover goes into `fixtures_<year>.tmp`. Two saves of the same career can differ in these bits with no effect on the game. The save proof (`prova_save.py`) ignores them:
- **Friendly** (0x5C5F40): +0x24..+0x27 and bits 0xF8 of +0x4E (`sobra_do_amistoso`).
- **Cup match** (0x51A150, 0x51CB90): bits 0xF0 of +0x4E (`sobra_da_copa`). Nobody reads bits 14-15 of the +0x4D word; bits 12-13 only matter during the match, written at 0x6B8089 and read at 0x6B6062.

**The match category** (+0x3A of the match record), counted in the 2010, 2011 and 2014 fixtures of the `All Leagues` save:

| Value | Matches | Where it appears |
|---|---|---|
| 1 | 23,973 | club friendlies |
| 2 | 80,808 | league matches |
| 4 | 9,285 | domestic cup rounds |
| 5 | 768 | league play-offs and deciders (Brazilian 3rd division, MLS, Swedish 2nd, Belgian 3rd, Serie C1) |
| 6 | 1,017 | domestic cup and league cup final stages |
| 7 | 1,626 | secondary continental competitions (UEFA Cup, Intertoto, Libertadores, Mercosur, Merconorte) |
| 8 | 1,404 | Champions League and deciding stages (UEFA Cup, Libertadores, regional finals) |
| 9 | 1,520 | friendlies linked to national-team competitions (ids 433, 434) |
| 10 | 563 | national-team competitions (qualifiers, World Cup, continental) |
| 23 | 67 | Brazilian Champions Cup and Club World Cup |

In the match record (0x4F), +0x36, +0x38 and +0x3A receive the round rules: in leagues +0xDB, +0xAB and +0xC4; in cups +0x0B, +0x0D and +0x17. +0x3A is the match category (competitions reference, §2.1).

### 7.1 The CLZ save (NickPatcher and NickDLL)

[observed in `clz_teste\Maior2GBPatcher.sav`, read only, reading the index and the `.tmp` slices without loading the 2.3 GB]: a 2025–2026 career with 57 competitions. The readers close all 57 and the 18 global ones (only the 2 GB test's `0000_enchimento.tmp` is left out). Two differences from the original exe:
- `match_stats_matches.tmp` has 20000 slots of 0xD7A (the original, 10000); the reader takes the count from the size;
- the NickDLL's Intercontinental declares an entry-book capacity (+0x38) without allocating it, and the original's save only writes the book if the pointer exists (0x5216A1). Since the file does not keep that flag, the reader accepts a missing book when the file ends there.

`TesteMais2GB.sav` is the same 2001–2002 career as the other test saves plus the filler: it closes the same way (164 + 22).

**The `All Leagues - 3.9.68 - Compress.sav` save** (external, received on 2026-10-25, at the project root and outside git; 1.43 GB compressed, original 3.9.68 exe, all leagues): a career from 2001 to 2064, with 64 years of fixtures and 32 of archived transfers. The readers close **all 383 competitions and 119 global files**, with no adjustment, including classes the other saves lacked (MiniCup and MiniLeague, the pre-season tournaments, 22 of each; SweSecond).
