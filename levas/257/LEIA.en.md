# Batch 257 — the key nation rows: the row of a nation, the row of a club and the club's season year; the club season and the main nation: whether the club is in season and its dates, the main continent and the main nation; the player statistics: the statistic of a category and the combined ratings, the order of the languages and the order of the history by club; the contract clauses: the active release clause, the FIFA protected period and the allowed bonus type; the weather postponement: the decision to postpone or abandon the match, the club finances and the stadium booking order; the match weather: the wind, rain and temperature of the match, the temperature by band and the pitch condition; the club wealth and two orders: the finance band, the player order by ability and the stadium order by capacity; the default training schedules: the computer's seven and each human manager's ten at the start of the game, the schedule type and the human manager's schedule by slot; the players' initial training entries, the regen pool of free player records and whether there is a spare non-player record; the ruling bodies' bans: the red card ban, the body's new season and the compaction of its entries; the formation slots: storing the slot coordinate and choosing the positions, the duplicate position and the marking type; the league's team order for the fixture grid and the possible teams on each side of a cup fixture; the club's player search target rating and the tracked player's match history: the entry and the goal total

**Routines:**
- `linha_da_nacao()` = 0x669340 (key_nation.cpp, cdecl): the first row of the 35 key nation table holding the given nation, or 4;
- `linha_da_nacao_do_clube()` = 0x669360 (key_nation.cpp, cdecl): the row of the club's chosen nation; national side, 1; nation not chosen, the row of its continent, or 3;
- `ano_da_temporada_do_clube()` = 0x669950 (key_nation.cpp, cdecl): the club's season year: the previous year before the season start day of its row;
- `clube_na_temporada()` = 0x669bd0 (key_nation.cpp, cdecl): by the start and end days of the club nation row and today: whether it is in season and the start and end dates with the right year;
- `continente_principal()` = 0x6695c0 (key_nation.cpp, cdecl): the continent of the human's nation, or that of the first main nation of the table;
- `nacao_principal()` = 0x669600 (key_nation.cpp, cdecl): the human's nation, or the first main nation of the table;
- `estatistica_da_categoria()` = 0x7c1e30 (player_stats.cpp, thiscall): the requested statistic of the category record (without a record, the default for the rating and the average), or the combined ratings of categories 2 to 5;
- `ordem_dos_idiomas()` = 0x7c6d40 (player_stats.cpp, cdecl): the order of the language records: by person id and by language, with 0xff matching any language;
- `ordem_da_historia_por_clube()` = 0x7c6d90 (player_stats.cpp, cdecl): the order of the player history by the dword +0xa and then by +4;
- `clausula_de_rescisao_ativa()` = 0x867b50 (staff_contracts.cpp, thiscall): 1 when one of the three release clauses of the contract is active;
- `periodo_protegido()` = 0x868210 (staff_contracts.cpp, thiscall): whether the contract is still in the protected period of the new FIFA rules: 3 years under age 28 at signing, otherwise 2;
- `tipo_de_bonus_permitido()` = 0x867aa0 (staff_contracts.cpp, cdecl): whether the bonus type applies to the player: type 2 only for goalkeepers, the others only for non-goalkeepers;
- `adiar_pelo_clima()` = 0x923630 (weather.cpp, thiscall, ret 4): league matches of the home club only, at an uncovered stadium (+0x4c) with a city and places and outside the key nation season end; in the evening slot, random(1000/2500/5000 by the finance band) zero returns 0x12; by the weather (0x923010): rain 5 at 1 to 13 degrees in a stadium of up to 20000 places draws the postponement (9); rain 8 and 9 (ice), random chains returning 0, 5 or 6;
- `financas_do_clube()` = 0x5a0590 (finance.cpp, thiscall, ret 4): the finance record (0x167 bytes) by the club id; a B club with an A partner, the A one (0x540a50); null club, error box;
- `ordem_das_reservas_do_estadio()` = 0x85e380 (stadium.cpp, cdecl): bookings qsort: by date (+4), then by stadium id (+0);
- `clima_da_partida()` = 0x923010 (weather.cpp, thiscall, ret 8): the stadium city (else the home club one, else the capital); the city wind, the region and the period by the day; wind (0..4), rain (0, 1, 2, 3, 5) and temperature band by the period limits; below zero, the rain becomes snow/ice (6..9) by the city wind;
- `temperatura_do_jogo()` = 0x9232c0 (weather.cpp, thiscall, ret 8): (day × band) modulo 6/7/8 plus the band base (−5 to 35; other band −100); in the evening slot, − 2 below 10, otherwise × 0.8;
- `gramado_do_jogo()` = 0x923970 (weather.cpp, thiscall, ret 8): by the rain, the temperature, the stadium flag and places and a fixture remainder modulo 20: 0, 1, 2, 4, 8, 0x10, 0x20 or 0x40; covered stadium, 0;
- `faixa_de_riqueza()` = 0x5986b0 (finance.cpp, thiscall): national team, 0; by the 64-bit balance against the club reputation R: above 2500·R + 5,000,000, 2; above 1000·R + 3,500,000, 1; below −2,500,000 − 1500·R, −2; below −250·R (R < 5000) or (3750 − R)·1000, −1; otherwise 0;
- `ordem_por_habilidade()` = 0x5d6220 (game.cpp, cdecl): player data qsort: current ability (+5), then potential (+7), ascending;
- `ordem_pela_capacidade()` = 0x85e350 (stadium.cpp, cdecl): venue qsort: places (+0x3c) of the stadium (+0xc), largest first; without a stadium, 0;
- `programas_padrao()` = 0x8b3d40 (training_manager.cpp, thiscall): sets up the computer's seven schedules (type k in slot k) and the ten schedules of each of the 16 human managers, then gives the default names;
- `tipo_do_programa()` = 0x8b4e50 (training_manager.cpp, thiscall, ret 8): stores the schedule's slot and type and its five category levels from the type's mix (general, one strong category, goalkeepers or no training);
- `programa_do_humano()` = 0x8b3930 (training_manager.cpp, thiscall, ret 8): a human manager's schedule for slot 0 to 9 (otherwise 1); outside the 16 human managers, the error box and 0;
- `fichas_de_treino_iniciais()` = 0x8b3c50 (training_manager.cpp, thiscall): allocates the 6-byte training entry of every player data record and puts goalkeepers (goalkeeping rating 15 or more) on schedule 6 and the others on 1;
- `guardar_jogador_no_banco()` = 0x7a8350 (player_regen.cpp, thiscall, ret 0xc): appends to the pool of free player records the player data id of a person who stops playing, with the nation and second nation, growing the table by 250;
- `tem_reserva_de_tecnico()` = 0x7a6e20 (player_regen.cpp, thiscall): whether the spare non-player record list holds any entry;
- `suspensao_do_vermelho()` = 0x7e8e10 (ruling_body.cpp, thiscall, ret 0xc): builds the red card ban: the matches or days from the offence code (9 to 12) and the body scale, today's date and the player; 0 when there is no ban;
- `nova_temporada_do_orgao()` = 0x7e7ed0 (ruling_body.cpp, thiscall): at the new season, releases bans 2 (or 4) years old, frees the entries without a ban, clears the bookings of the others and compacts the table;
- `compactar_disciplina_do_orgao()` = 0x7e9050 (ruling_body.cpp, thiscall): removes the free entries from the body's discipline table, fixes the stored index of each moved player and reallocates with 100 spare rows;
- `coordenada_da_casa()` = 0x5b3560 (formation.cpp, thiscall, ret 0x10): stores the slot coordinate for the phase and ball zone and picks the general and extra positions closest to the slot's 12 coordinates;
- `posicao_repetida_na_formacao()` = 0x5b3460 (formation.cpp, thiscall, ret 8): finds another slot with the same general position and sets the output when its flag is lower;
- `tipo_de_marcacao()` = 0x5b57d0 (formation.cpp, thiscall, ret 8): sets one marking type of the slot (bits 0..2 or 3..4 of byte +0x5b4) and clears +0x187f;
- `sortear_ordem_dos_clubes()` = 0x686a70 (league.cpp, thiscall, ret 4): shuffles the league table with the seeded rand and puts clubs sharing a ground, clubs with linked grounds and Celtic and Rangers in paired slots;
- `clubes_do_lado_do_jogo()` = 0x521790 (Cup.cpp, thiscall, ret 0x10): the team on one side of the fixture or, if empty, the teams that can still reach it (count in global 0xae1880);
- `clubes_possiveis_do_jogo()` = 0x5219b0 (Cup.cpp, thiscall, ret 0x10): appends to the output the teams that can play the fixture, walking down the fixtures the empty sides come from;
- `nota_alvo_do_clube()` = 0x7bca90 (player_search.cpp, cdecl): the rating the club looks for: the base from reputation, money, nation and league, by bands, and the adjustments for squad size and wealth; at least 500;
- `jogo_do_historico()` = 0x7bd830 (player_search.cpp, thiscall, ret 4): match entry i (0x29 bytes) of the tracked player's ring of 10, or 0 past the count;
- `gols_do_historico()` = 0x7bd870 (player_search.cpp, thiscall, ret 0): the sum of the goals of the matches stored in the ring;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (35):** 32 caught. `cpu4_periodo_de_dois_anos` escaped: the age-based protected period only applies with the new FIFA rules ([0xae233c]), which are not on before 2002-06-01 in the career [inference]. `cpu9_goleiro_so_com_20` escaped: the goalkeeper branch runs 512 times, but the initial record slot only matters for the human club; for the computer the first training update (0x8b1750) redoes the slot with the same rule [inference]. `cpu11_sem_dobro` escaped: the doubled distance runs 4096 times, and the side-by-side comparator saw no difference either: on the loaded formations the mutation is equivalent (the chosen slots do not change) [inference]. (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (bc5827d1).
- **Rust ×2 and original careers to 2002-06-01:** Rust ×2 345e9799, original 24a0d51a (recorded; does not block the batch since 31/10/2026).

**Redo**, on top of batch 256's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 257`

**Fields touched:**
- key nation table 0xb63d60 (35 × 72 bytes): nation +0, continent +4, start day +0x15, dword +0x17; reads (all three)
- club +0x53: nation; pointer; reads (linha_da_nacao_do_clube)
- nation +0x11c: nation chosen to play; value; +0x71: continent; pointer; reads (linha_da_nacao_do_clube)
- [0xae2c90] day and [0xae2c92] year of today; value; reads (ano_da_temporada_do_clube)
- key nation table 0xb63d60 (35 × 72 bytes): nation +0, date +0x2a (start), date +0x39 (end); reads (clube_na_temporada)
- club +0x53: nation; pointer; reads (clube_na_temporada)
- nation +0x11c bit 2: main nation; +0x71: continent; reads (continente_principal, nacao_principal)
- [0xae2d08] human's nation; pointer; [0xae2c90]/[0xae2c92] day and year of today; reads
- person +0: id; value; reads (estatistica_da_categoria); [0xae236c] number of people with statistics; reads
- category record (0x20 bytes) +0: games; +0xe: total (rating); reads (estatistica_da_categoria)
- language record (6 bytes) +0: person id; +4: language; reads (ordem_dos_idiomas)
- history record +4 and +0xa; value; reads (ordem_da_historia_por_clube)
- contract +0x1c, +0x1d, +0x1f: release clauses; value; reads (clausula_de_rescisao_ativa)
- contract +0: person id; id; +0x25: contract date; value; reads (periodo_protegido)
- [0xae233c] new FIFA rules on; [0xae2330] rules date; value; reads (periodo_protegido)
- person +0x61: player record; pointer; record +0xf: position (0x14 goalkeeper); reads (tipo_de_bonus_permitido)
- fixture +0x14 competition, +0x18 stadium, +0x1c home club, +0x2a day, +0x3f day slot; pointers/values; reads (adiar_pelo_clima)
- stadium +0x38 city, +0x3c places, +0x4c covered, +0x4d flag; pointer/values; reads (adiar_pelo_clima)
- club +0 id, +0x57 league, +0x64 home flag; id/pointer/value; reads (adiar_pelo_clima, financas_do_clube)
- [0xb63d60] key nations (0x48 bytes) +0x39 end day, +0x3b year; values; reads (adiar_pelo_clima)
- club finance (0x167 bytes at [[0xae2a38]]) +0x166 counter; value; reads (adiar_pelo_clima)
- stadium booking +0 stadium id, +4 date; values; reads (ordem_das_reservas_do_estadio)
- Weather [0xde2f74] +0 regions (0x6e bytes), +4 city readings (4 bytes: rain, wind/temperature, city wind), +8 region count; reads (clima_da_partida)
- weather region +0x1e/+0x31/+0x44/+0x57 period days, period (19 bytes) +0x20..+0x30 limits; values; reads (clima_da_partida)
- city +0 id, +0x1f nation, +0x34 weather region; id/pointer/value; reads (clima_da_partida)
- nation +0x7a capital; pointer; reads (clima_da_partida)
- fixture +0 id, +0xc/+0x10 values, +0x18 stadium, +0x1c home club, +0x2a day, +0x3f slot; reads (clima_da_partida, temperatura_do_jogo, gramado_do_jogo)
- stadium +0x38 city, +0x3c places, +0x4c covered, +0x4d flag; reads (clima_da_partida, gramado_do_jogo)
- match engine +0x4756 match weather (wind, rain, temperature, city wind), +1 pitch; written through the match setup (clima_da_partida, gramado_do_jogo)
- club finance +0/+4 balance (64 bits), +8 club id; value/id; reads (faixa_de_riqueza)
- club +0x80 reputation; value; reads (faixa_de_riqueza)
- player data +5 current ability, +7 potential ability; values; reads (ordem_por_habilidade)
- venue entry +0xc stadium; pointer; stadium +0x3c places; value; reads (ordem_pela_capacidade)
- TRAINING_MANAGER +0 + 0x9b × slot: the 167 schedules (7 for the computer, then 10 per human); value; writes (programas_padrao), reads the address (programa_do_humano)
- TRAINING_SCHEDULE +0: slot (signed dword); +4..+8: levels of the five categories (0, 10, 25, 50); +0x15: type (−1 to 7); value; writes (tipo_do_programa)
- person +0: id (the last 16 of [0xae2364] are the human managers; 0x6e-byte record at [0xae23bc]); id; reads (programas_padrao, programa_do_humano)
- TRAINING_MANAGER +0x651d: 6-byte training entries per player data record (+0 schedule slot, 6 goalkeepers or 1 general; +5 bit 0x20); pointer and value; writes (fichas_de_treino_iniciais)
- player data record (0x46 bytes at [0xae23c4], [0xae236c] records) +0: id; +0xf: goalkeeping rating (signed); id and value; reads (fichas_de_treino_iniciais)
- person +0x61: player data record (id at +0); pointer and id; reads (guardar_jogador_no_banco)
- PLAYER_REGEN +0x3c pool of free player records (16 bytes: record id, nation, second nation), +0x40 count, +0x44 capacity; pointer, id and value; appends (guardar_jogador_no_banco)
- PLAYER_REGEN +0x20: count of the spare non-player records; value; reads (tem_reserva_de_tecnico)
- match player +0x2c: card type (0 or 1); +0x2e and +0x2f: offence code (9 to 12); +0: id; +0x48; value and id; reads (suspensao_do_vermelho)
- new ban (caller record) +0 matches, +1, +2 days, +4 date, +0x10 = 3, +0x1c player id, +0x6f; value; writes (suspensao_do_vermelho)
- RulingBody +4 slot and +5 type in the discipline index; +6 scale; +0xb entries (14 bytes), +0xf capacity, +0x13 count; +0x17 and +0x1b; value and pointer; reads (all three), writes (nova_temporada_do_orgao, compactar_disciplina_do_orgao)
- discipline entry +0 id (−1 free), +4 and +5 bookings, +6 ban (signed index, −1 none), +8 word, +0xd bits; id and value; reads and writes (nova_temporada_do_orgao, compactar_disciplina_do_orgao)
- [0xae28f0] discipline: +4 bans (20 bytes, +6 year); stored slots (0x55cb60/0x55cb20, +6 index); value; reads and writes (nova_temporada_do_orgao, compactar_disciplina_do_orgao)
- [0xae2c90] today's date (+2 year); value; reads (suspensao_do_vermelho, nova_temporada_do_orgao)
- Formation +0x12d: the general position of the 11 slots (word, sector and side mask); +0x143 the slot flag (byte); +0x14e the extra position (word); value; reads and writes (coordenada_da_casa, posicao_repetida_na_formacao)
- Formation +0x164: the slot coordinates (11 × 2 phases × 3 × 4 ball zones, dword: x 0..26 and y 0..35); value; reads and writes (coordenada_da_casa)
- Formation +0x5b4: the marking type of the 11 slots (bits 0..2 and 3..4); +0x187b the formation mask; +0x187f cleared on each change; value; writes (tipo_de_marcacao, coordenada_da_casa)
- [0x9cc368] the position table (words up to 0xffff); value; reads (coordenada_da_casa)
- League +0x3e: the club count (word); +0x40 the league seed (word); +0xb1 the table (59 bytes: +0 the club); +0xd9 the flags (0x100: slots from virtual +0xa8/+0xac); +4 the nation; pointer and value; reads and writes the table (sortear_ordem_dos_clubes)
- club +0x69: the stadium (+0x48 the linked stadium); [0x9cf5dc] Scotland; [0x9d0118]/[0x9d011c] Celtic and Rangers; [0xdd7120] the game seed; pointer and id; reads (sortear_ordem_dos_clubes)
- Cup +0x3c: the round count (word); +0xa7 the clubs; +0xad the fixtures of each round (0x35 bytes: +0/+2 the sides, word, −1 empty; +0xe/+0xa and +0x10/+0xc the round and fixture each side comes from); [0xae1880] the count; value and pointer; reads and writes the count (clubes_do_lado_do_jogo, clubes_possiveis_do_jogo)
- club +0x77: the money (dword); +0x80 the reputation (word); +0x57 the nation (+0x69 the reputation, word); +0x1db the league in [0xae23d0] (0x6b bytes, +0x69 the reputation); +0x64 professional (byte, 1); pointer and value; reads (nota_alvo_do_clube)
- [0xadadfc] the nations with a league (dword per id); [0xae2a38] the finances; pointer; reads (nota_alvo_do_clube)
- tracked player +6: the ring of 10 matches (0x29 bytes, +0x16 the goals, signed byte); +0x1a0 the start; +0x1a1 the count; value; reads (jogo_do_historico, gols_do_historico)

**Of interest to:** CMRTE, CMScout, SGE, GPF2
