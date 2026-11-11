# Reorganisation (2026-11-07/08): proofs and inputs

The reorganisation Lucas approved on 2026-11-07 stopped the batches and tidied the repository so anyone can read it:
(1) tool fixes; (2) the `cm0102-base` crate; (3) `motor.rs` split into one file per source file of the game;
(4) readability in rounds, each fixing what an outside reader (an agent that does not know the project nor read
Portuguese) pointed out. No round changes behaviour: it changes names, comments, where a routine lives. The exception is
round 27 (09/11/2026, checked signatures, below), which fixes wrong reads of the return value of exe functions.

## Round 27: signatures checked against the disassembly

53 exe functions were declared with different signatures in different places (calling convention, argument count,
float vs integer, return width). Three agents disassembled each one and its callers in the original exe (`ret N`, use
of ecx, return register) and wrote the true one in `assinaturas_conferidas.json` (with the evidence);
`involucros_exe.py` uses that signature for the `exe::` wrapper and converts the call sites to it.

- **Fixed (real bugs):** call sites that read the whole of eax from functions that only return `al` or `ax` (the upper
  bits are garbage): `transfer_manager_collect_*_news_recipients`, `match_player_decide_ball_action`,
  `injury_get_condition`, `human_manager_get_commentary_speed`, `database_get_staff_age`,
  `database_get_contract_rating_modifier`, `hostcountry_find_hosts`, `comp_util_get_fixture_venue_side` and others; each
  site now reads the right width and extends it.
- **Converted for clarity, same result:** `finance_get_transfer_budget` and `match_player_get_instructions` return 64 bits
  (edx:eax; the sites only read the low half); `formation_set_player_instruction` takes a `u64` (it used to be split into
  two `u32`).
- **Left as they are, harmless:** the `stdcall` sites of routines the original calls as `thiscall` but that never read
  ecx (the stack is the same); the extra arguments to `utils_get_version_string` (cdecl, the caller cleans up); `get_stat`
  read as `f32` in match_man's `instructions` (the original stores it as a dword between the calls; going through f64
  would round twice).
- **Mutations:** round 25 had left 48 mutations naming, in their replacement text, a local alias that no longer exists
  (`let _ = alias;`, which would not compile); `mutacoes_sem_apelidos.py` drops those names, and each changed mutation
  was compiled one by one.

## Readability batch 49: enum values by name

The outside reader's round 12 obstacle 2 (enum values without constants): two reader agents named the values of 35 enum
fields (396 constants, most verified by the strings the game prints for each value): job kinds, offer status, table
status, match category, result marks, round names, discipline rules, comp type, professional status, world regions,
languages, live match state, European registration... `aplicar_enums.py` writes `rust/src/motor/enums.rs` and turns 302
literals into names. Misleading locals renamed (`stadium`, `reputation`, `professional`, `manager_reputation`); `cargo
fix` again. Proof: aa7e9c95 ×2, detector equal, comparison 0; the 16 changed mutations compile.

## Readability batch 48: secondary records typed, review renames

The outside reader's round 12 obstacle 1 (secondary records still `usize`): the variables of `offer`, `league_table_row`,
`manager_record`, `cup_round`, `shortlist_slot`, `physical_record`, `stadium`, `match_record`, `cup_tie`,
`retirement_entry`, `live_match`, `match_record_row`, `injury_entry` and `comp_stats_club` get their record type and
their fields become methods (306 accesses). Three relative offsets the tool had wrapped as a record
(`ShortlistSlot(k * 0x25)`) went back to numbers. Renames checked in the disassembly: `manager_record::MONTH_AWARDS` ->
`AWARDS` (0x86c8c0 and 0x86c390 increment it), `club::EURO_FLAG` removed (synonym of `CONTINENTAL_COMP_ID`). Locals,
round 30 (16 names). Proof: aa7e9c95 ×2, detector equal, comparison 0; the 32 changed mutations compile (two replacements
followed the new type).

## Readability batches 42 to 47: field readers on the rest of the engine

Reader verdicts on 107 more files, from the shared helpers to the smallest competition files, with 2 to 4 files per
reader in the last ones: 42 (`shared`, `human_manager`, `fin_prm`, `officials_manager`), 43 (`scout_manager`, `comp`,
`friendly`, `awol`), 44 to 46 (8 files each) and 47 (the last 75 files with raw offsets). Dozens of new records (the
human manager record, the referee, the FIFA ranking, the file container, the awards, the World Cup qualifiers...).
Reviews: `official::RATING` -> `REPUTATION`, the referee record no longer treated as a player sheet (`renew_officials`,
`init_strictness`), the routine 0x5fd210 `manager_9a01` -> `is_on_holiday` (community name),
`comp_stats_club::DANGER_POSITION` -> `FINAL_POSITION` (written by `Comp_Stats::set_club_final_position`; four readers
found the old name wrong). `nomear_por_linha.py` no longer replaces inside strings. Outside reader
(`leitor_de_fora_rodada12.txt`): 3.5 -> 5.5. Joint proof: aa7e9c95 ×2, detector equal, comparison 0; the 68 changed
mutations compile (one replacement still named `official::RATING`, fixed).

## Readability batch 41: field readers on player stats, regens, stadiums and the World Cup

Reader verdicts on `player_stats.rs`, `player_regen.rs`, `comp/stadium.rs` and `comp/worldcup/world_cup.rs`. New
records: `stadium_venue`, `world_cup_fields`; the match statistics of `player_category_stats` and `lineup_row`,
`stadium::SIZE`, the retirement and regen bits of the staff state. A tool had written
`career_entry::STATS * HISTORY_ENTRY_SIZE` where the original means six entries: back to `6 * HISTORY_ENTRY_SIZE` (same
value). Proof: aa7e9c95 ×2, detector equal (it also covers batch 40), comparison 0; the 3 changed mutations compile.

## Readability batch 40: field readers on UEFA seeding, European cups, match day and training

Reader verdicts on `comp/uefa_seeding.rs`, `comp/eurocomp/european_cup.rs`, `match_day.rs` and `training_manager.rs`.
New records: `uefa_seeding_nation`, `european_club_entry`, `training_manager_fields`, `training_schedule`; the match
spectators' per-human vectors and the network update bits. Proof: aa7e9c95 ×2, comparison 0; the garbage detector did
not run (free memory under 4 GB with the readers running) and is covered by batch 41's proof, which contains this code;
the 7 changed mutations compile.

## Readability batch 39: field readers on contracts, scouting, tactics and UI areas

Reader verdicts on `contract_manager.rs`, `player_search.rs`, `tactics.rs` and `area.rs`. New records: `area_fields` (the
UI area of Area.cpp), `guio`, `undo_slot`, `tactics_row`, `scouted_player_slot`, `scouted_match_entry`, `search_result`;
match statistics in `match_record_row` and in the engine's per-side block. In the engine, `FOULS`/`CARDS` became
`YELLOW_CARDS`/`RED_CARDS`: `fill_match_record` copies them into the match record's yellow and red cards (the referee
rating reads "reds > 2" and "yellows > 5"). The injury attribute loss copy defect (0x62b7b0) went to
`DEFEITOS_DO_ORIGINAL`. Proof: aa7e9c95 ×2, detector equal, comparison 0; the changed mutations compile (one replacement
still named `engine::CARDS`, fixed).

## Readability batch 38: field readers on fixtures, leagues, media and shortlists

Reader verdicts on `comp/fix_man.rs`, `comp/league.rs`, `media.rs` and `shortlist_manager.rs`. New records:
`fixture_leg_check`, `fixed_fixture`, `h2h_entry`. Bits of the league flags, the tie rules, the transfer status, the AI
shortlist count in the staff state and two contract grievances (inference). Scoreboard of batches 34 to 38: raw offsets
4,495 -> 3,177; 109 new named fields (verified 14 -> 50) and 55 bits in 23 modules of `bits.rs`. Proof: aa7e9c95 ×2,
detector equal, comparison 0; the changed mutations compile.

## Readability batch 37: field readers on national teams, database, cups, formations

Reader verdicts on `national_teams.rs`, `database.rs`, `comp/cup.rs` and `formation.rs`. New records: `call_up_entry`,
`human_manager_record`, `staff_history`, `colour`; new fields in `national_squad_record` (squad size limits, reserves
by level), `formation_fields` (password, creator, slot coordinates, from the loader's error strings), `cup_round`/
`cup_tie` and `names` (NATION, WEIGHT). Bits of the cups' draw and tie rules, of the slot cells and of the squad rules.
Proof: aa7e9c95 ×2, detector equal (it also covers batch 36), comparison 0; the changed mutations compile.

## Readability batch 36: field readers on the match engine

Reader verdicts on `match_eng.rs` (175 raw -> 64), `match_man.rs` (173 -> 124; almost all the rest is the original
routine's emulated stack frame) and `match_pl.rs` (160 -> 47). New `engine` fields (SHOOTOUT_SHOTS, SHOTS_ON_TARGET,
SIDE_PITCH_HANDICAP and MATCH_ALREADY_OVER as inference), `match_player::STARTING_POSITION`, the `tactic_stats` record;
bits of the tactics (OFFSIDE_TRAP, PRESSING) and of `staff::PLAYING_SQUAD`. Engine +0x1a4 stays unnamed: the two
readers disagreed. Proof: aa7e9c95 ×2, comparison 0; the garbage detector did not run (free memory under 4 GB with the
readers running) and is covered by batch 37's proof, which contains this code; the 13 changed mutations compile.

## Readability batch 35: field readers on four files, flag bits by name

Field readers on `match_screens.rs` (185 -> 80 raw; 65 are drawing coordinates), `transfer_manager.rs` (122 -> 17),
`injury.rs` (115 -> 19) and a second pass on `manager_manager.rs`. New records: `screen_manager`, `display`,
`side_stats`, `recovery_news_entry`. `nomear_bits.py` names the bits of flag fields in the new `bits.rs`
(`expectation_flags::TITLE_CHALLENGE`...); `ajustar_constantes.py` adds the `as T` to a name used in another integer
type. Board record review: `SPENDING_CEILING` -> `SQUAD_INVESTMENT`, `CONFIDENCE_2/3` -> `SUPPORTERS_/STAFF_CONFIDENCE`.
The appeal defect (0x557000) went to `DEFEITOS_DO_ORIGINAL`. Proof: aa7e9c95 ×2, detector equal, comparison 0; the 11
changed mutations compile.

## Readability batch 34: field readers on manager_manager.rs (pilot of decision (e))

Two reader agents gave a verdict on each of the 205 raw `+ 0x..` of `manager_manager.rs` (record, name, width,
confidence with evidence): 125 existing names, 51 new (15 distinct fields, a new `names` module for TNames), 24 not
fields and 5 unknown. `nomear_por_linha.py` applies them per line; `fields.rs` carries the confidence mark. Raw offsets
in the file: 205 -> 33. The verdicts are kept in `leitores_de_campo/`. Outside reader
(`leitor_de_fora_piloto_campos.txt`): 3/10 before, 6/10 after; what is left are the unnamed bits inside flag fields,
fields still raw in the same record and magic limits in hex. Proof: aa7e9c95 ×2, detector equal, comparison 0; the 6
changed mutations compile.

## Readability batch 33: the compiler's own fixes

`cargo fix` applies the compiler's 975 suggestions (unnecessary parentheses, unused imports and `mut`): warnings go from
1,021 to 46 (what is left are values the faithful port assigns and never reads, and the borrowed `static mut`).
`espacos_do_cargo_fix.py` removes the spaces it leaves where it dropped parentheses (`x as u8 ;`), only on changed lines.
Both run at the end of `padronizar.sh`, so new batches enter without those warnings. 29 mutation anchors followed (2 by
hand) and one replacement that no longer compiled (`(x as i32) < 10`). Proof: aa7e9c95 ×2 (the same as batch 32),
detector equal, comparison 0.

## Readability batch 32: one safety contract, the engine's flags as atomics (decision (a))

`estaticos_atomicos.py`: the engine's 71 file-level scalar `static mut` (dry-run flags, trampolines) become atomics with
`Relaxed` (the game has a single thread), so reading them no longer needs `unsafe`. The 23 left are borrowed (`addr_of!`:
the comparers' tapes, the records a dry run builds) or read by the original routine table; the 36 in the rest of the
crate are the infrastructure the asm reads. `motor.rs` got one `# Safety` section for every `unsafe fn` of the engine
instead of one on each (checked 3.9.68 exe, the game's thread, live records as arguments). Proof: aa7e9c95 ×2 (the fingerprint moved from 7cc445c2, as it did in earlier batches: the generated code changed and so did the stack residue the original reads; the bar is Rust ×2 equal and comparison 0), detector
equal, comparison 0; no mutation names the changed statics and no anchor broke.

## Readability batch 31: a typed value keeps its type (outside reader, round 11)

`metodos_em_cadeia.py`: a typed record's field read through its address becomes the field's method, chains included
(`read_usize(person.club_job().addr() + club::MANAGER)` → `person.club_job().manager().addr()`), in two phases: the checked
form `Club::manager(EXPR)` compiles only when EXPR is a Club; then it becomes the method call. 429 accesses. The
hand-written strides left: club pointers step by `club::SIZE`, the remaining club index arithmetic uses `club_at(..)`, the
engine player blocks use `m.player(side, k)` and the match manager got `player(k)` over its 20-player block.
`PlayerStatsEntry::nova` → `new`. The `.addr()` left (7,788) is mostly the boundary: half are arguments of the hooked
routines and the comparers, which take `usize` because the exe calls them that way. Proof: 7cc445c2 ×2, detector equal,
comparison 0; the 14 changed mutations compile.

## Readability batch 30: the style step for new batches

`padronizar.sh` runs the readability tools over the code right after a batch is integrated (`publicar_leva.py` calls it):
first the compiler-driven fix of calls written against the old API, then `read_T`/`write_T`, the global tables, the reader
closures and the `.rdata` names, English locals and messages, named offsets, typed records and the cleanup. Every tool is
idempotent and backs out a function that does not compile; a second pass over the tree changes nothing. The first pass
over the current tree still typed a few accesses. Fixes on the way: `nomear_rdata.py` keeps every existing line (an
address can carry two names); a typed setter writes a bare literal as `0xffffu16 as i16` (a literal above `i16::MAX` typed
as `i16` does not compile). `COMO_ESCREVER_UM_ITEM` describes the typed style. Outside reader, round 11:
`leitor_de_fora_rodada11.txt`. Proof: 7cc445c2 ×2, detector equal, comparison 0; the 5 changed mutations compile.

## Readability batch 29: typed signatures and the last raw float constants

`tipar_registros.py --assinaturas` for the 25 record types in use and `--propagar` over them (`.addr()` 8,079 → 8,017: the
rest is functions with mixed callers). `expandir_leitoras.py` expands 596 calls of local closures that only read a fixed
exe address (`let k = |a: usize| read_f64(a); k(0x968dc0)`), and `nomear_rdata.py` names them by value: 63 new
constants, no raw float read left in the code. `nomear_rdata.py` kept only the constants of the current raw reads and
dropped the named ones: it now keeps them and only adds. Proof: 7cc445c2 ×2, detector equal, comparison 0; the 28
changed mutations compile.

## Readability batch 28: Portuguese function names and more locals

`nomes_em_ingles.py`: the functions and comparers left with Portuguese names got English ones (`compare_condicao` →
`compare_player_condition`, `compare_papel_ataque` → `compare_attacking_role`, `posicao_no_conteiner` →
`position_in_container`, `instalar_passagem` → `install_passthrough`, the `novo()` constructors → `new()`...), `IDX_`
constants included. `cm0102-base` keeps its names for now (Tweaks carries copies of it): an alias in the `use`.
`traduzir_locais.py` no longer blocks a name that only clashes with a record method or a path-qualified item, and
`partida` → `fixture` (`match` is a keyword): 178 more locals. The count no longer takes `total`, `time` and `media` as
Portuguese. Portuguese locals 474 → 243. Proof: 7cc445c2 ×2, detector equal, comparison 0; the 12 changed mutations
compile.

## Readability batch 27: the debug messages in English

`textos_em_ingles.py --todos`: the debug messages outside the comparers (the competition helpers: "returned", "stage",
"without stage", "differences in") go through the same translation; only texts with a `{}` placeholder and no path or
file name; accented words are looked up without the accent. 217 more messages; the Portuguese texts in the code went
from 777 to under 80 (mostly environment variable names). Proof: 7cc445c2 ×2, detector equal, comparison 0.

## Readability batch 26: the DLL's log in English

The log lines the tools read changed together with their readers, which accept both forms (old logs and diaries stay
readable): `DIVERGENCIA` → `DIVERGENCE`, `EXCECAO` → `EXCEPTION`, `N comparisons, N divergences`, `SIMULATION stopped
at .. draws= pointer= offset=`, `matches: N engines, fingerprint`, `memory: bank`, the diary `day .. draws= people=
memory= clocks=`, `branch <name>: N`, `coverage`, `allocator:/coverage: WARNING`, `<name>: SELFTEST N cases`; the
scoreboards are `engine`, `date` and `random`. The other ~70 messages (allocator, detours, resolution, msgbox,
simulation, stack probe, diag) too. `CM_AUTOTESTE` also takes `engine:N` and `database`. Readers updated: `simular.py`,
`mutacoes.py`, `cadeia.sh`, `fila_continua.sh`, `gen_docs.py`, `pendencias.py` and the diary readers
(`deixar_infeliz.py`, `leitoras.py`, `sonda_pilha.py`). Proof: 7cc445c2 ×2, detector equal, comparison 0 (`simular.py`
read the new lines: `engine: 1000000000 comparisons, 0 divergences`); the mutation test with the new log catches the
mutations.

## Readability batch 25: the comparers' messages in English

`textos_em_ingles.py`: 720 messages the comparers print on a divergence (`"semana({:?}) original={} rust={}"`) now name
the routine by its Rust function (inside `compare_X`, a Portuguese name at the start becomes X; elsewhere the
`docs/NOMES_ROTULOS.csv` map), and their Portuguese words go through the dictionary (expected, got, club, person, cell,
side, count...). No tool reads these texts; the log keywords the tools read change in a batch of their own. Proof:
7cc445c2 ×2, detector equal, comparison 0.

## Readability batch 24: the hooked routines named once, in English

Each line of the `routines!` table carried a Portuguese label for the log (`"motor_tabela_a"`) repeating what the Rust
function's name already says; the labels went and the macro takes the name from the function itself (`stringify!`), so
the log, `SEM_ROTINAS` and the tools use `table_a` (2,092 labels; the old → new map is in `docs/NOMES_ROTULOS.csv`, so
old logs and notes stay readable). `tabela_motor.py` reads both forms, `fila.py` writes the new one and `prova_save.py`
maps an old batch's labels through the CSV. The hand-written hook names (the gates, the simulation's clock, RAM and
disk, the stack probe) are in English too (Lucas's decision, 10/11: everything that can go to English goes). Proof:
7cc445c2 ×2, detector equal, comparison 0.

## Readability batch 23: the global club and person tables by name

`club_at(i)` and `staff_at(id)` (in `motor/mem.rs`) replace the hand arithmetic on the game's tables
(`read_usize(G_CLUB_TABLE) + i * 0x245` and variants, 27 places, `tabelas_globais.py`); a club pointer walking the table
uses `club::SIZE`. The parameter `percentual` had been renamed to `write_percent_pair` (the name of something else) in
six files: it is `percentage` now. A compile of all 3,250 mutations (in batches) found 20 replacement texts gone stale
over the batches (types, translated locals, transmute locals that went): fixed, all 20 compile. Proof: 7cc445c2 ×2,
detector equal, comparison 0.

## Readability batch 22: one memory dialect

`unificar_acessos.py`: 5,697 raw pointer accesses in 1,558 functions became the unaligned accessors of `motor/mem.rs`
(`*((E) as *const T)` → `read_T(E)`, `*((E) as *mut T) = V` → `write_T(E, V)`, `read_unaligned` and `write_unaligned`
likewise). Left: compound assignment (`+=` would evaluate E twice), a deref taken by reference and a cast of something
that is not an address; 159 functions that did not compile went back. Parentheses around a lone `read_T(..)` go. The
broken mutation anchors were redone by re-running the rewrite on the old function with the mutation applied; ten old
ambiguous anchors got unique context (81 → 71). Proof: 7cc445c2 ×2, detector equal, comparison 0; the 318 changed
mutations compile (in batches).

## Migration to typed records, batch 21: Portuguese locals and repeated transmutes

`traduzir_locais.py` got 160 words (round 20): 364 local names in 285 functions are now English (Portuguese locals 856 →
474; the count no longer takes `data` as Portuguese, since it reads as English and the translator keeps it).
`usar_involucros.py`: 186 inline `transmute`s that repeated an `exe::` wrapper with the same calling convention, the same
number of arguments and the same return type became the wrapper (25 functions went back: argument types the wrapper
takes differently). `compilar_mutacoes.py` checks the changed mutations in batches of non-overlapping anchors, one
`cargo check` per batch, and goes back to one by one for the functions an error lands in (proved with a deliberately
broken mutation). Proof: 7cc445c2 ×2, detector equal, comparison 0; the 45 changed mutations compile.

## Migration to typed records, batch 20: functions return their record type

`tipar_retornos.py`: 29 of our functions that returned a record as a bare `usize` (`contract_of_staff`,
`friendly_club_by_id`, `staff_records_staff_by_id`, `transfer_manager_main_contract_stored`...) return the type, voted by
how the callers use the value (`Club(f(..))` or `f(..) + club::FIELD`); the compiler fixes the rest. `--ajustar` now
handles an `if`/`else` whose branches ended up with different types. `limpar_tipos.py` drops a rewrap of the same type
(`Club(x.club().addr())` → `x.club()`) when the function still compiles (305 functions). Proof: 7cc445c2 ×2, detector
equal, comparison 0; the changed mutations compile.

## Migration to typed records, batch 19: ten more typed pointer fields

`ponteiros_registros.py` found ten more pointer fields by use: `staff::PLAYER` and `match_player::PLAYER` →
`PlayerSheet`, `staff::NON_PLAYER` and `match_manager::MANAGER_SHEET` → `NonPlayer`, `offer::EXCHANGE_PLAYERS` and the
friendly lists → `Llist`, `match_manager::ENGINE` → `Engine`, `CLUB` → `Club`, `FIXTURE` → `Fixture`. The generator
(`registros.py`) now keeps as a pointer a field typed in an earlier run (it generated `read_Club` when `ponteiros.json`
no longer listed it). `limpar_tipos.py` reads every type from `records.rs` and removes doubled parentheses around a chain
(`((sheet())).addr()` → `sheet().addr()`). Proof: 7cc445c2 ×2, detector equal, comparison 0; the 25 changed mutations
compile.

## Migration to typed records, batch 18: named offsets and eight more records

`nomear_deslocamentos.py`: in a function, a raw offset on a typed variable (`x.addr() + 0x27`, or a chain
`x.club_job().addr() + 0xcf`) becomes the field's name when the record's module has exactly one field at that offset;
`--acessos` turns the access into a method. 108 named (club 43, staff 30, engine 12, nation 9, competition 9...). The
records `TransferManagerFields`, `Llist`, `MatchRecord`, `ManagerRecord`, `Friendly`, `NonPlayer`, `MatchManager` and
`LeagueTableRow` went through `tipar_leva.sh`. Raw offsets 5,090 → 4,982. Mutation anchors 82 → 81 (the ambiguous
`l_conta` got context). Proof: 7cc445c2 ×2, detector equal, comparison 0; the 65 changed mutations compile.

## Migration to typed records, batch 17: offers, cups and player sheets

`tipar_leva.sh MODULE=Type ...` runs the full pattern of a batch for each pair (`tipar_registros.py MODULE`, `--exe`,
`--assinaturas`, `--ajustar`, `--acessos`, then `--propagar` and `limpar_tipos.py`). `Offer`, `Cup` and `PlayerSheet`
variables got their type: raw accesses to `offer` 923 → 178, `cup` 805 → 88, `player_sheet` 681 → 438.
`seguir_ancoras.py` gained the fallback for an anchor whose only change is a dropped `.addr()` (the function now takes
the type); 5 mutation replacements were fixed by hand. Proof: 7cc445c2 ×2, detector equal, comparison 0; the changed
mutations compile.

## Migration to typed records, batch 16: the engine's sub-structures (outside reader, round 10, item 5)

The match player and the grid cell get named access. `Engine::player(side, k)` returns match player k (0..20) of the
side (engine + `engine::PLAYERS` + 8920 × side + 0x1be × k) and replaces the ~200 hand computations
(`(m as isize + 0x4796 + 0x1be * (k + 20 * side)) as usize`, `side_player_slot`, the inside of `player_in_block`);
`engine.cell(x, y)` returns the `GridCell` (engine + `engine::GRID` + 90 × (12x + y)) and replaces `occupancy_grid_cell`
and the grid formulas (the neighbours become `home(±1)`). `jogador_do_motor.py` does the rewrite: raw forms become the
call, a typed receiver loses its `.addr()`, a raw `usize` gets `Engine(..)`, and the argument casts come from the
compiler's messages; ~20 cases by hand. Then `tipar_registros.py match_player` typed the variables that receive the
player (137 → 51 `.addr()` on `player(..)`). The commentary codes left behind also became names (the pitch zones,
`event::IN_LEFT_MIDFIELD`.., and the debut goal). No `0x4796`/`8920`/`0x22d8`/`0x215e`/`0x21b6` is left outside
`fields.rs`. Proof: 7cc445c2 ×2, detector equal, comparison 0; the changed mutation compiles.

## Migration to typed records, batch 15: the commentary codes (outside reader, round 10, item 4)

`aplicar_eventos.py` reads the identifiers of the game's own `Data/events_eng.cfg` (`# EVENT_HAND_BALL` above
`= 72, ...`; the code the exe passes to `Match_Events::add_event` is 8000 + the event number) and generates
`rust/src/motor/events.rs`: module `event`, 677 constants (`event::HAND_BALL = 8072`), name level "game" (the game's own
file names it). Only the identifiers go in, none of the commentary texts. Literals change ONLY where the value is
certainly a code: the argument in the position of a `code`/`codigo` (or `abertura`/`common`/`quieta`) parameter of the
narration closures and functions, and the first element of the array passed with `FN_MATCH_EVENTS_ADD_EVENT`; the other
numbers in the same range (the grid 0x21b6, reputations 8000, flags 0x2000) stay. 516 literals became names
(`narrate(call, event::SILENT_OFF_OFF_OFF, ..)`). Proof: 7cc445c2 ×2, detector equal, comparison 0; the 17 changed
mutations compile.

## Migration to typed records, batch 14b: the cleanup

`limpar_tipos.py`: `(x.f()).addr()` → `x.f().addr()` (387 → 2), `x == Club(0)` → `x.is_null()` (238 → 10) and
`a.addr() == b.addr()` → `a == b` when both sides have the same type (48 → 7; the compiler decides, and a function that
does not compile goes back to its previous text); the first two rewrites also apply to the mutation lists. Propagation
now recognises chains (`f(x.club_job().addr())`), 7 more functions. `.addr()` 6,215 → 6,086; what is left is mostly
arguments of functions that take `usize` with mixed callers (~3,200), field accesses that stay raw (~760: width, array,
compound write), unnamed offsets (~300) and casts/comparisons (~500). Proof: 7cc445c2 ×2, detector equal, comparison 0;
the 17 changed mutations compile.

## Migration to typed records, batch 14: typed pointer fields (outside reader, round 10, item 1)

`ponteiros_registros.py` finds, from how the code uses it, which record each field that returned `usize` points to (the
base of another record, wrapped in a type): 14 fields (`staff::CLUB_JOB` → `Club`, `match_player::ENGINE` → `Engine`,
`fixture::HOME_CLUB` → `Club`, `staff::NATION` → `Nation`, `engine::PENDING_PLAY_OFFENDER` → `MatchPlayer`...), in
`levas/reorganizacao/ponteiros.json`, read by `registros.py`. The compiler drove ~1,570 fixes (`--ajustar`, which now
also handles `Club + 0x..` and `club as u32`, applies only non-overlapping edits per pass and writes `else { T(if ..) }`
for `else if`); the new `--cadeias` step converts accesses through the typed chain (`person.club_job().manager()`).
Records implement `LowerHex` (the logs' `{:x}` keeps working). Typed chains 376 → 471; raw reads 19,118 → 18,924. Side
effect: `.addr()` went up (5,259 → 6,215), where the typed value goes into something that still wants `usize`
(functions and comparisons); the current propagation only recognises a variable's `x.addr()`, not a chain's.
Mutations: 13 replacements fixed; 12 anchors followed by hand. Proof: 7cc445c2 ×2, detector equal, comparison 0.

## Migration to typed records, batch 13: the match engine's magic numbers

Two agents read in the original the values of the enum-like fields of the engine and the match player and the move
codes (the `MOVE_SKILL` index), confirming them with the commentary texts (`events_eng.cfg`: the event number is the
commentary code minus 8000): 98 constants in `rust/src/motor/match_codes.rs`, generated by `aplicar_codigos.py` from
`levas/reorganizacao/codigos_motor.json` and `codigos_lance.json`, each with its meaning and reliability
(`restart::THROW_IN`, `ball_height::HELD_BY_GOALKEEPER`, `shootout::PENALTIES_KICKS`, `period::FIRST_HALF_END = 495`,
`moves::SLIDE_TACKLE`...); the player's `action` holds the move code itself and uses the same module. 182 literals
became names (`m.restart_type() == restart::OPEN_PLAY`). Also: array elements read or written through a pointer
(`*((j + match_player::MOVE_SKILL + 2 * 12) as *mut i16)` → `j.move_skill(12)`).

Wrong names the reading found in the copy, fixed: code 37 is a handball, not a header (variable `cabeca` →
`handball`; labels "shot header"/"play header" → "shot handball"/"play handball"); in the set-piece taker, the labels of
cases 2 (throw-in) and 4 (goal kick) were swapped ("dribble throw_in without teammate", "dribble goal_kick drawn").
Proof: 7cc445c2 ×2, detector equal, comparison 0; the 14 changed mutations compile.

## Migration to typed records, batch 12: the match engine (`Engine`)

The planned batch 12 (league subclasses and leftovers) was swapped, at Lucas's request after the review, for the match
engine, the largest object still untyped. `--fechos` now also expands one-parameter closures over a captured base
(`let b = |o: usize| (m + o) as *mut u8;` → `((m + o) as *mut u8)`; 29 functions), then the full package: 209 engine
variables in 198 functions (73 of them `this`), 143 `.addr()` removed from the wrappers, 42 functions take `Engine`
(109 calls fixed), propagation 5 more and 2 closures. Raw engine accesses 1,290 → 692. Five mutation anchors built with
`chr(10)` were fixed by hand. Proof: 7cc445c2 ×2, detector equal, comparison 0; the 115 changed mutations and the 7 at
risk compile.

## Migration to typed records, batch 11: increments, compound assignments, readers and read names

- The converter recognises the one-line add helpers by their shape (seven: `add_to_i32`, `fin_prm_add_i32`,
  `add_u8_at`...): `add_to_i32(x + F, v)` → `x.set_f(x.f().wrapping_add(v))`; compound assignments `*((x + F) as *mut T)
  += v` (and `-=`, `|=`, `&=`, `^=`) → `x.set_f(x.f().wrapping_add(v))`; and `--fechos` expands the generic read
  closures (`let e = |a| read_unaligned(a as *const T)`) when the argument is a field.
- The unnamed offsets of the nine records (53, without the 6 subclass ones already seen) read by three agents: 11 new
  names (in the match player `SPACE_ON_BALL`, `CONSISTENCY_ROLL`, `INCONSISTENCY_SWING`, `WEAK_FOOT_DEFICIT`,
  `OWN_CLUB_FEELING`, `MATCH_TEMPER`, `KEY_PASSES`, `DEBUT`, `POSITION_ADJUSTMENT`; in the contract `DATE_STARTED_YEAR`,
  `SQUAD_NUMBER`), each with its reliability: where the two readers agreed, verified; where they disagreed (+0x31),
  inferred. The `MOVE_SKILL` array elements become `p.move_skill(k)`. `aplicar_nomes_lidos.py` applies the verdicts.
- Raw hex on typed variables 262 → 107; raw fields via `.addr()` 741 → 625; `.addr()` outside the wrappers
  5,041 → 4,735. Proof: 7cc445c2 ×2, detector equal, comparison 0; the 22 changed mutations compile.

## Migration to typed records, batch 10: the closures

`tipar_registros.py --closures Type,..`: a local closure whose `usize` parameter is used as a record (`let zero =
|p: usize| (read_i8(p + match_player::SIDE) == 0) as u32`) takes the type (`|p: MatchPlayer| (p.side() == 0) as u32`);
the body is converted like a typed variable and the calls by the compiler (278 fixes). 81 closures. `.addr()` outside
the wrappers 5,160 → 5,041 (and 122 `Type(..)` where the call passed an untyped `usize`). Left as they are: the generic
`call` of the comparison recording and the comparers' `o` (the original routine), which take `usize` on purpose.
Mutations: 9 replacements fixed (they called the closure or the typed function with a `usize`). Proof: 7cc445c2 ×2,
detector equal, comparison 0.

## Migration to typed records, batch 9: contract, nation and finance

The full package for three medium records: 202 + 126 + 29 variables; 22 functions take `Contract`, 6 `Nation`, 3
`Finance`, and propagation 10 more; 81 `.addr()` removed from the wrappers. Raw accesses: contract 634 → 228, nation
311 → 92, finance 158 → 44. `.addr()` outside the wrappers 4,760 → 5,160. To review among the names:
`nation::THREE_LETTER_NAME` is compared with a nation pointer in `tactics.rs` (the field name must be wrong). Proof:
7cc445c2 ×2, detector equal, comparison 0; the 62 changed mutations and the 4 at risk compile (one fixed).

## Migration to typed records, batch 8: the player in the match (`MatchPlayer`)

The full package for `match_player`, with the methods' `this`: 294 variables in 239 functions, 222 `.addr()` removed
from the wrappers, 58 functions take `MatchPlayer` (240 calls fixed), propagation 13 more (76). New step,
`tipar_registros.py --fechos`: the local closures returning a pointer (`let b = |q: usize, o: usize| (q + o) as *mut u8;`,
used as `*b(j.addr(), match_player::X)`) are expanded at the call into the form the converter understands (27
functions), and the unused closure goes. Raw match-player accesses 2,552 → ~1,070 (the rest: closure parameters,
reassigned variables or several `let`). `.addr()` outside the wrappers 3,815 → 4,760. Mutations: 3 replacements
missing the now required `MatchPlayer(..)` were fixed. Proof: 7cc445c2 ×2, detector equal, comparison 0.

## Migration to typed records, batch 7: the fixture (`Fixture`)

The full package for `fixture`: 184 variables in 182 functions, 75 `.addr()` removed from the wrappers, 31 functions
take `Fixture` (67 calls fixed) and propagation, 7 more. Raw fixture accesses 1,307 → 365 (the rest use an expression
as base, not a variable). `.addr()` outside the wrappers 3,488 → 3,815. Before it: batch 6's finding investigated and
recorded in `docs/DEFEITOS_DO_ORIGINAL.en.md` (the daily friction uses the contract of squad slot 10, not the
player's). Proof: 7cc445c2 ×2, detector equal, comparison 0; the 43 changed mutations and the 4 at risk from `Fixture`
compile.

## Migration to typed records, batch 6: name reliability and the four records' residues

- **Name reliability** (Lucas's request): `confianca_campos.py` classifies each field by the origin its `fields.rs`
  comment already tells: community (the editors' layouts, 247), verified (`[verified: how]`, a second independent
  source, 9), read (a reading of the code with evidence, 1,334), inferred (`[inference]`, 142). It writes
  `docs/CONFIANCA_DOS_CAMPOS.en.md`; every method in `records.rs` carries the level (`Name: community.`).
- **Unnamed offsets** used on typed variables (30, 403 uses), checked by two agents: 17 already had a name, 7 new
  (`league::COMPETITION`, `COMP_STATS`, `ENTRANTS_ARE_QUALIFIERS`, two more from the `Comp` base copied from `cup::`;
  the year halves of the person's dates), 6 belong to subclasses (the same offset is something else in each kind of
  competition: they wait for the subclasses to have types). 306 raw hex offsets became names; raw hex on typed
  variables 209 → 107. To check: the club +0xff the original reads is squad slot 10, not the manager (+0xcf), perhaps
  an original defect.
- **Arrays:** `club.squad(i)`, `coaches(i)`, `scouts(i)`, `physios(i)` (the league's `TIEBREAKS` is read whole
  elsewhere and stays scalar).

`.addr()` outside the wrappers 3,683 → 3,488. Proof: 7cc445c2 ×2, detector equal, comparison 0; the changed mutations
compile.

## Migration to typed records, batch 5: the league and the competition

The full package (variables, accesses, wrappers, signatures, propagation) for `league` and `competition`, now also on
the methods' `this` (`let this = League(this);`): 424 + 50 variables, 142 + 5 functions take the type, propagation 17
more. New forms in the converter: `core::ptr::read_unaligned/write_unaligned((x + mod::F) as *T, v)`; a pointer write
whose value holds an already converted read replaces the inner edit (before, the two overlapped and broke the text: the
first write-out was undone). Raw league accesses 1,914 → 278, competition 119 → 47. `.addr()` outside the wrappers went
up (2,503 → 3,683): the typed `this` is passed on as `usize` to the original routines and read at unnamed offsets
(`this.addr() + 0xee`). Mutations: 57 anchors followed, 27 by the fix for the `.addr()` the signatures removed from the
code, 4 by hand; compiled the changed ones and the 93 whose replacement calls a typed function (5 fixed). Proof:
7cc445c2 ×2, detector equal, comparison 0.

## Migration to typed records, batch 4: propagation (club and person)

`tipar_registros.py --propagar Club,Staff`: a function of ours with a `usize` parameter that most calls fill with a
typed record (`f(club.addr())`) takes the type, even when it only passes it on; the body is converted like a typed
variable, and the calls by the compiler (`ajustar_pelo_compilador`: the span to fix is the one labelled "expected ..,
found ..", not the primary one, which can be the called function's name). Repeats until no function is left: 120 + 5
functions in two rounds, 515 fixes. `.addr()` outside the wrappers 2,875 → 2,503; 321 typed parameters. What is left:
array and other-width accesses (`club::SQUAD + v * 4`, ~450), reads at unnamed offsets (~450) and local closures
taking `usize` (~230). `compilar_mutacoes.py --lista` compiles a given list: the 20 mutations whose replacement calls a
function that now takes the type (one fixed). Proof: 7cc445c2 ×2, detector equal, comparison 0.

## Migration to typed records, batch 3: the person (`Staff`)

The full pattern in one batch: `tipar_registros.py staff` (532 variables in 479 functions), `--exe` (633 `.addr()`
removed from `exe::` arguments), `--assinaturas Staff` (74 functions take `Staff`, 191 calls fixed by the compiler) and
the new `--acessos MOD` mode over the variables already typed: a read narrower than the field is its low byte
(little-endian, no side effect), so `read_i8(p + staff::AGE)` (a 2-byte field the original reads as a byte) becomes
`(p.age() as i8)`; `read_i8_as_i32` becomes `(p.f() as i8 as i32)`; writes keep the exact width. Raw person accesses
2,262 → 195. The `.addr()` outside the wrappers went up (1,700 → 2,875): the person is passed to many functions that
take it as `usize` without reading a field (the residue batch propagates the type), and there are reads at unnamed
offsets (`read_usize(p.addr() + 0x61)`). Two round-25 mutations with an alias inside the replacement (`remove`,
`reputacao`) were fixed. Proof: 7cc445c2 ×2, detector equal, comparison 0; the changed mutations compile.

## Migration to typed records, batch 2: the club in signatures

- The `Addr` trait (records.rs): an address in the game's memory, `usize` or a typed record pointer. The `exe::`
  wrappers take `impl Addr` where the original passes a pointer (`involucros_exe.py`), so `exe::x(club)` needs no
  `.addr()` (442 removed in the code, 27 in the mutations: `tipar_registros.py --exe`). `i32` implements `Addr` only
  for the untyped `0`.
- Our functions that took a club as `usize` and typed it on entry now take `Club` (112;
  `tipar_registros.py --assinaturas Club`); the calls were fixed by the compiler (294: `y.addr()` → `y`, the rest
  `Club(..)`).
- `registros.py` keeps a field's type once generated (the usage count changes as the code moves to the methods).
- `seguir_ancoras.py COMMIT`: follows the mutation anchors broken by a mechanical change (12 followed; 3 replacement
  texts fixed by hand).

`.addr()` outside the wrappers: 2,246 → 1,700. What is left: functions that take the club as `usize` without reading
a field (they only pass it on) and arrays/other widths (`club.addr() + club::SQUAD + v * 4`). Proof: 7cc445c2 ×2,
detector equal, comparison 0; the 27 changed mutations compile.

## Migration to typed records, batch 1: the club

`engenharia-reversa/tipar_registros.py club`: in a function, the variable used as `x + club::FIELD` becomes a `Club`
(a parameter: `let x = Club(x);` at the top of the body; `let x = e;` becomes `let x = Club(e);`), the accesses become
methods (`x.reputation()`, `x.set_own_stadium(1)`; a pointer to another record: `x.stadium().addr()` for now),
`x == 0` becomes `x.is_null()` and the other uses `x.addr()`. Left as they are: variables reassigned or with several
`let` (107), accesses with another width and arrays (`club::SQUAD + v * 4`). 544 variables in 475 functions; 4
functions went back to their previous text (they did not compile). Raw club accesses 1,676 → 657; 881 methods; the
friction: the `.addr()` where the club goes to a function that still takes `usize` (gone once it takes the type).
Proof: 7cc445c2 ×2, detector equal, comparison 0; the 83 changed mutations compile (one, `cpo_abrir_a_vaga`, had an
alias from round 25 that had slipped through and was fixed).

## Round 29: typed records pilot

`engenharia-reversa/registros.py` generates `rust/src/motor/records.rs`: for each object in `fields.rs` (162), a
`#[repr(transparent)]` pointer type (`Club(usize)`, the same ABI as the raw address) with one getter and one setter per
field (1,576 typed), in the type the code already uses at that field (what round 28 checked against the original).
Fields pointing to another record return its type (`club.stadium()` is a `Stadium`; the `POINTERS` table). Pilot on
one function, `season_end_club_update` (fin_prm.rs, the stadium works): raw reads and writes 62 → 20, `+ 0x` offsets
11 → 1, pointer casts 10 → 2, right names (`cash` instead of `goal_difference`; `'works`); the friction: 39 `.addr()`
to pass the club to helpers that still take `usize` (they go when those take `Club`). Proof: 7cc445c2 ×2, detector
equal, comparison 0.

## Round 28: the automatic checker of signatures and field widths

Round 27 only caught the functions declared in different ways; one declared wrong in the same way everywhere slipped
through. Two tools now check everything against the original machine code:

- `engenharia-reversa/conferir_assinaturas.py`: every exe function the Rust code calls (1,435, counting the wrappers,
  the local declarations and the fixed addresses passed to helpers) against the routine and its callers in the
  original: the stack (`ret N` or the caller's `add esp`), ecx read before written (`thiscall`) and the return width
  (what the routine writes last before each `ret` and what the callers read: `al`, `ax`, `eax`, `edx`, `st0`).
  Validated against the 53 of round 27: it flags none of the true ones and catches every wrong one that was a real bug.
  It found 31 functions returning only `al`/`ax` declared `u32`: narrowed by `estreitar_retorno.py` (signed where the
  original callers use `movsx`), each existing call keeping the old type with an `as`. 24 ambiguous suspects were
  checked by agents: all harmless (return ignored or truncated), recorded as `accepted_sites` in
  `assinaturas_conferidas.json`. Result: 0 bugs. It runs on every batch in `publicar_leva.py` and stops the publication
  on a bug (exit code 1); the full list goes to `conferencia_assinaturas.tsv`.
- `engenharia-reversa/larguras_campos.py --original`: every access to a named field inside a ported routine against
  the widths the original routine uses at that displacement. Of the 8,964 checkable accesses, 8,886 match the original
  instruction; the other 78 were checked by agents (55 collisions with another object at the same displacement, 14
  fields merged into a wider or narrower access, 9 address only), none wrong (`larguras_conferidas.json`). Without
  `--original`, the list of fields read with different widths across uses (`larguras_campos.tsv`), for review. Known
  limit: some routine sizes in the community map are short (0x6c6a30, 0x6e2f20, 0x6ec010, 0x736720) and the matching
  access lies past the stated end.

Calls through pointers that only exist at run time (the game's tables and vtables) stay out: they cannot be checked
statically.

## The proof of each step

Each step was proved before its commit, on a bench (`engenharia-reversa/bancada.py`), with the DLL built from that step:

- `prova_*.txt`: two Rust careers to 2002-01-01 (`simular.py --repetir 2 --detector`): both equal and the garbage
  detector equal. The fingerprint alternates between two known forms (7cc445c2 and b38ccaa6) depending on how the
  compiler lays out the code (stack leftovers); under the 2026-10-31 criterion that is accepted.
- `comparar_*.txt`: comparison mode to 2002-01-01 (each rewritten routine runs next to the original); the last line is
  the number of divergences and exceptions in the log, which must be 0.

## The inputs (what the agents read and the tools applied)

| Files | What they are | Tool |
|---|---|---|
| `leitor_de_fora_rodada1..7.txt` | the outside reader's reports, round by round | — |
| `tipos_*.tsv`, `tipos2_*.tsv`, `tipos3_*.tsv` | the record or object each variable holds in each function (`file\tfunction\tvariable\tobject`) | `aplicar_tipos_variaveis.py` |
| `campos_motor/` | the fields of the match engine and of the in-match player, and the functions where `m`/`j` are something else | `aplicar_campos_motor.py`, `aplicar_tipos_variaveis.py` |
| `cabecalhos_en/` | the complete English of the headers whose detail was only in `[pt]` | `cabecalhos_completos.py` |
| `docs_en/` | the English of the other docs (`///`) that were only in Portuguese or short (key: the block's hash) | `docs_em_ingles.py` |
| `rodada5/` | the 5th typing round: variables (`tipos5_*`) and 835 proposed fields (`campos5_*`, 113 new objects) | `juntar_campos.py`, `aplicar_tipos_variaveis.py` |
| `rodada6/` | the 6th round (motor.rs, managers, match, comparers): `tipos6_*`, `campos6_*` | `juntar_campos.py`, `aplicar_tipos_variaveis.py` |
| `comentarios5/` | the English of 873 body comments | `aplicar_comentarios.py` |
| `conferencia_assinaturas.tsv`, `larguras_no_original.tsv`, `larguras_conferidas.json`, `larguras_campos.tsv` | the output of the signature checker and of the width checker, and the verdicts on the 78 widths | `conferir_assinaturas.py`, `larguras_campos.py` |
| `assinaturas_conferidas.json` | the true signature of 53 exe functions, checked against the disassembly (with the evidence) | `involucros_exe.py` |
| `objetos/` | the fields of 5 objects that had no module (the scheduled match, the match-day manager, Finance, Stadium, FIXTURE_MANAGER) and who holds them | `aplicar_tipos_variaveis.py` |

The tools are in `engenharia-reversa/`, and the readability steps after integrating a batch are in
[`docs/COMO_CONTINUAR.en.md`](../../docs/COMO_CONTINUAR.en.md). The scoreboard is in [`docs/LEGIBILIDADE.en.md`](../../docs/LEGIBILIDADE.en.md).
