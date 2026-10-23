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
| S1 | C1 container: queries | 0x933a00 pattern, 0x9331c0 id, 0x933860 size | pattern: pure `lado_a_lado` (+ self-test with names, `*`, case); id and size: both on the same file (read only), compare the result |
| S1 | C2 pack/unpack | 0x933320 `zextract`, 0x933610 `zadd`, 0x90cd50 clear `CM3_TEMP` | **file** (new): original and Rust on copies (folders A and B in `jogo_testes`), compare result, file list and bytes of the files produced |
| S1 | R1 RLE | 0x932c30 `zread`, 0x932e10 `zwrite`, 0x932b00 `zclose` | **file**: two `ZFILE`s on copies of the same container; compare buffer, result, the structure without the `FILE*` (+0) and, on `zclose`, the whole file. Self-test with types 3 and 4, runs of 1 to 300, bytes > 0x80, 0x30d40 buffer wraps, runs spanning calls; reading a real compressed save (copy of `TesteTelasComprimido.sav`) comparing every item |
| S2 | S1 checks | 0x838e10, 0x839290, 0x838eb0 | `lado_a_lado` on a fixed `.sav`/`.qst` (read only); self-test, since only the file dialog calls them |
| S2 | S2 single items | 0x8395b0, 0x839760 FIFA rules; 0x591f80 summary | rules: **file**; summary: `lado_a_lado` of the block (masking play time if it is wall clock) |
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
| Stack garbage | `save_game_data` (0x175c frame), `load_game_data`, `valid_saved_game_version` (SEH) | zeroing probe (`ZERADAS`) before comparing |
| Non-determinism | summary (play time), date and time in `general.dat`, disk space, CD | check on the first proof; mask if wall clock |
| Save DLL | `save/` uses the three routines in `conteiner.rs` and the signature of 0x9326e0–0x933860 | the new routines stay out of `save/`; the public repository only changes on releases |
| Large allocation | each `zopen` allocates ~650 KB, `zextract` another ~530 KB | changes the heap and the print: compare only in the save step |

Border with the Database block: `save_database` 0x52dad0, `load_database` 0x528540, `database_free_all_tables` 0x528080 and the `db_files_read_*`/`db_files_write_*` (Db_files.cpp, 0x54d5e0–0x550950), all on top of `zread`/`zwrite`.

## Lead: out-of-table ids in the saved squad (2026-10-23, from the Save Editor validator study)

Source: CM3SaveEditor\docs\VALIDADOR_ESTUDO.md (commit bd7959e), passed on by the orchestrator session; marked as inference there. The game itself writes into the club.dat squad ids ≥ N (outside the people table): 51 slots in TesteTelas.sav (club 1109, ids 159578..159597; club 3397, 158727..158734) and 22 in Valencia.sav. Between TesteTelas and TesteTelasComprimido, club.dat changes only in those slots (they become −1) and staff.dat stays the same: a reload between the two, with `resolve_database_indexes` emptying the slots. Hypothesis: youngsters created in the reserve rows (the 23,000 of type 6 or the 1,325 uncounted) that `save_database` (0x52dad0) does not carry into staff.dat, while their id stays in the squad. A candidate cause of the id corruption the Save Game Editor repairs (PROJETO.md, item 8). Check once save_database and the load are in Rust: the copy shows exactly which people go into staff.dat and which ids stay in the squads.
