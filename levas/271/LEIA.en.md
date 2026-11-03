# Batch 271 — the contract signed and placed in the contract array (joining the club, loans with the length from today, the minimum end, the array slot keeping the old terms at the same club, a free slot or the end of the array, the value on the person and the club change); the youth regen: the retired person rebuilt as a new player from the record pool (nation, names, languages, age, ability, reputations, personality) and placed at a club; the dispute removal; the formation read from the .pct file (the AI and the human tactics) and the byte order swap of words and dwords; the new game officials: the ones kept per nation, the competition slots and the share of each; the slot order by competition level; the cancelled friendly, the national team fixture removed with it and the cancelled tournament; the order of the officials by nation and quality, both ways; the player stats archived into the history (the club change, the loan, the retirement) and the games and goals for the club recounted; the player_setup.cfg: the person by name (in the club or in all), the quoted text of the line and the line type by name

**Routines:**
- `assinar_contrato()` = 0x4cf720 (contract_manager.cpp, thiscall, ret 8): puts the person in the contract club (or cancels without space), handles loans (full squad, job 11, length from today), ensures 4 weeks of contract, stores the contract in the array (the old index, a free slot or the end, growing by 100), the value at person +0x4e, the payroll and the club change;
- `refazer_como_jovem()` = 0x7abac0 (player_regen.cpp, thiscall): rebuilds the retired person of the regen queue as a young player: the front record of the pool, the nation (or another small one), the second nation, the nation names and languages, the age and ability (by the small nation reputation or by position and by draw), the reputations and their caps, the birth, the personality attributes by age and, by chance, the best club or a club of the nation;
- `tirar_da_disputa()` = 0x56a090 (dispute.cpp, thiscall): clears the person dispute flag and removes her entry from the dispute list;
- `ler_formacao_do_disco()` = 0x5b5d10 (formation.cpp, thiscall): reads a formation from a .pct file (the identifier and version, the inverted password, the creator, the general and extra positions, the coordinates per phase and zone, the team and player instructions, the marking), fixes the instructions of the old versions, redoes the positions of the 11 slots from the coordinates and builds the coordinate map;
- `trocar_bytes_das_palavras()` = 0x43f2d0 (cash.cpp, cdecl): swaps the two bytes of each word of an array;
- `trocar_bytes_dos_dwords()` = 0x43f300 (cash.cpp, cdecl): reverses the four bytes of each dword of an array;
- `montar_os_arbitros()` = 0x79bf60 (officials_manager.cpp, thiscall): on a new game: reviews the officials, sorts them by nation and quality, keeps 6 per nation (61 in the youth club nations), builds the per-nation blocks with one slot per league (the demand from the competition), draws the missing ones and shares the nation officials among the slots;
- `ordem_das_vagas_de_competicao()` = 0x79d410 (officials_manager.cpp, cdecl): orders the official slots of a nation by competition level (the invalid ones last or first), tie by id;
- `cqg10_amistoso_cancelado()` = 0x5c6570 (friendly.cpp, thiscall, ret 8): a friendly of this competition: news 8 to the managers of both clubs that did not cancel and the friendly slot freed; of another competition, its tournament is cancelled; a fixture without a round leaves the calendar (the national team one through 0x5cb9e0);
- `cqg10_jogo_da_selecao_cancelado()` = 0x5cb9e0 (friendly.cpp, thiscall, ret 4): by the fixture flag, the home national team (or its nation's first team): its fixture on the previous day leaves the calendar; then the fixture itself leaves;
- `cqg10_torneio_cancelado()` = 0x5c8260 (friendly.cpp, thiscall, ret 8): news 0xb to the invited clubs (a tournament starting after a week ago); the tournament leaves the pending list; the created competition fixtures become friendlies again or leave; the competition is destroyed and its slot and owner are cleared;
- `ordem_dos_arbitros_decrescente()` = 0x79d4e0 (officials_manager.cpp, cdecl): orders the officials by nation and, within it, by quality (+0x28 × 50 + +0x22) from highest to lowest; tie by index, descending;
- `ordem_dos_arbitros_crescente()` = 0x79d480 (officials_manager.cpp, cdecl): orders the officials by nation and by quality from lowest to highest; tie by index, ascending;
- `arquivar_estatisticas()` = 0x7c5fe0 (player_stats.cpp, thiscall): when the player leaves a club: the stats of each category of the season go to the history file (with the year, the club, the fee and the kind of exit) and are cleared; an empty node when the last category is missing; and the games and goals for the club recounted from the history;
- `pessoa_pelo_nome()` = 0x7a50f0 (player_regen.cpp, cdecl): finds the person of a player_setup.cfg line by the three names: among all people or, with the club name, in its squad (when asked) and its manager;
- `texto_entre_aspas()` = 0x7a5090 (player_regen.cpp, cdecl): copies the next quoted text of the line (skipping spaces and line breaks before it) and returns where to continue;
- `tipo_da_linha()` = 0x7a3140 (player_regen.cpp, cdecl): the type bit of a player_setup.cfg line (injury, manager, retirement, transfer, loan, doping, misconduct, passport, tactics, colour blindness, language, absence);

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (21):** 21 caught.
- **Real game comparing to 2002-01-01:** 0 divergences (8845ebc9).
- **Rust ×2 and original careers to 2002-06-01:** Rust ×2 151bdf85, original 932f1944 (recorded; does not block the batch since 31/10/2026). Garbage detector: equal.

**Redo**, on top of batch 270's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 271`

**Fields touched:**
- contract manager +0 array of 0x50-byte contracts, +4 capacity, +8 count; pointer and value; reads and writes (assinar_contrato)
- contract +0 person (−1 free slot), +4 club, +0xc value [inference], +0x25 start, +0x2d end, +0x35 type (6 bits; 5 loan, 2) and bits, +0x3a, +0x3b, +0x3d type by job, +0x3e, +0x3f, +0x44 contract happiness, +0x45, +0x49, +0x4a, +0x4e, +0x4f high nibble of the job; id and value; reads and writes (assinar_contrato)
- person +0x1a nationality, +0x1e second nationality, +0x39 club, +0x3d job, +0x3e date (+0x40 year), +0x46 contract end, +0x4e contract value [inference], +0x61 player record; pointer and value; reads, writes +0x1e, +0x46 and +0x4e (assinar_contrato)
- [0xae2d0c] 79-byte record (+0 contract index); [0x9cf434] and [0x9cf260] fixed nations; [0xae23a8] nations (0x122 bytes); index; reads and writes (assinar_contrato)
- PLAYER_REGEN +0x3c the record pool (16-byte entries: +0 record id, +4 nation, +8 second nation); value; reads (refazer_como_jovem)
- [0xae2d0c] + 0x4f × id: +8 = 0, +9 = 0x9c, +0xb bits 0x30 retired (cleared), 0x40 the regen mark (docs/MARCA_DO_REGEN.md), +0xc bits 0/1, +0xd = 0; value; reads and writes
- person +4/+8/+0xc names, +0x10 birth day and +0x12 birth year, +0x18 age, +0x1a nation, +0x1e second nationality, +0x22 international caps (0), +0x23 (0), +0x24, +0x39 club, +0x3e/+0x40 the joined date, +0x4e, +0x52, +0x56, +0x5f kind (2 player), +0x61 record, +0x69 non-player record (0), +0x28/+0x3d = 11; value/pointer; writes
- player record (0x46 bytes at [0xae23c4]) +5 ability (CA), +7 PA, +9/+0xb/+0xd reputations [inference: current, home, world], +0xf/+0x10/+0x11/+0x12/+0x13/+0x14/+0x15/+0x19 positions, +0x1b, +0x1d, +0x1e, +0x21, +0x24, +0x2c, +0x2e, +0x2f, +0x34, +0x36, +0x3c, +0x3d, +0x44 attributes [inference: personality and mental ones]; value; writes
- nation +0x71 continent, +0x88 reputation, +0x11c the flag of nations with youth clubs [inference]; value; reads; [0x9cf488]/[0x9cf424] the Spain and Basque Country keys; id; reads
- DISPUTE ([0xae2a30]) +0 the dispute list (LLIST, +0 count, +2/+0xa cursor), +4 the flag table per id; value; reads and writes (tirar_da_disputa)
- formation +0 the name (from the file), +0x33 password (50, inverted), +0x65 creator (200), +0x12d general positions (11 words), +0x143 order (11), +0x14e extra positions (11 words), +0x164 coordinates (11 slots × 2 phases × 3 × 4 zones, column and row words), +0x584 team instructions (0x2000, 0x4000, 0x20000), +0x5b4 marking (11), +0x187b mask, +0x187f/+0x1883/+0x1887 flags, +0x188b player instructions (11 × 8); value; writes (ler_formacao_do_disco)
- official (0x2b bytes at [0xae23e8], count [0xae2390]): +0 index, +0x16 nation, +0x22/+0x28 the quality in the order; value/pointer; reads and writes (montar_os_arbitros)
- OfficialsManager +4 count, +8 flags (calloc), +0xc per-nation blocks (0x102 bytes: +0 nation, +4 officials, +8 first index, +0xc demand, +0x10 competitions, +0x12 15 slots of 16 bytes: +0 start, +4 share, +8 demand, +0xc competition); value; writes
- nation +0x8e the default official count (1..5; 0 becomes 1), +0x11c; value; reads and writes; competition (0x6b bytes at [0xae23d0]) +0x5d nation, +0x69 level; club competition ([0xadadfc][k]) +0x3e the official demand [inference]; value; reads
- Friendly +4 the friendly competition, +0xaf index of the first tournament competition, +0xb1 the tournaments (0x69 bytes), +0x34cc friendly slots, +0x34d0 pending tournament list; pointer/value; reads and writes (all three)
- tournament +0 owner, +4 start (day, year), +0x15 the 4 invited clubs, +0x25 replies (1 declined), +0x64 competition index, +0x65 the created competition; pointer/value; reads and writes (cqg10_torneio_cancelado)
- fixture +0 competition key, +0x14 competition, +0x1c/+0x20 clubs, +0x28 year, +0x2a day, +0x42/+0x43 round, +0x4d flag (0x100/0x200 national team); value/pointer; reads and writes (all three)
- [0xadadfc] competition table; pointer; writes 0 (cqg10_torneio_cancelado)
- official +0 index, +0x16 nation (+0 id), +0x22 rating, +0x28 attribute [inference: the two of the quality in the order]; value/pointer; reads
- PlayerStats ([0xb74350]) +4 32-byte category records (cleared), +0x30 39-byte records per player record (+0..+0x14 the six categories, −1 none, +0x18/+0x1c the first and last history node, +0x20 games and +0x22 goals for the club); value; reads and writes (arquivar_estatisticas)
- history node (0x2b): +0 year, +2 club, +6 the 32 category bytes (+6 games, +8 goals), +0x26 transfer fee, +0x2a category × 8 + kind (0 transfer, 1 end, 2 loan, 3 other); value; writes to the file
- person +0x39 club, +0x3e/+0x42 the club joined date, +0x61 record; [0xb63d8a] + 0x48 × nation row the season start (+0/+2) and end (+0xf/+0x11); value; reads
- person ([0xae23bc], 0x6e bytes) +4 first name, +8 second name, +0xc common name (name records that start with the text); pointer; reads (pessoa_pelo_nome)
- club ([0xae23b4], 0x245 bytes) the name (0x545880), +0xcf manager, +0xd7 squad (50); pointer; reads

**Of interest to:** CMRTE, SGE, CMScout, GPF2
