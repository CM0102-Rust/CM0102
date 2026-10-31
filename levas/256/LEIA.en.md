# Batch 256 — the FIFA ranking in the month: the matches of a national side in a period, the update of the 18th and the order of the results by points; the FIFA ranking in the match: the result in the national side's history, the national side's strength tier and the ranking scores; the key nation of the match: the active key nation of a competition or fixture, whether the competition is the World Cup and whether it is an international tournament; the competition's key nation and the FIFA ranking order: the key nation index of the competition, the order of the national sides in the ranking and the score comparison; the city weather: the daily weather, the weather.cfg region and the region period; the nation coefficients (the December bonus and the five-year history), the retirement day and the clamped two-byte sum; the neutral venue and the initial weather: the venue score for the fixture, the distance factor in the crowd and the weather.cfg reading; the coaches' strength in training: the coaching factor by category, the human manager's coach assignments and the training schedule load; training progress on the attributes: applying the progress of the five categories, finding the player's position training and whether he learns a position fast; the training week, the spare non-player record and the recurring injury; the pre-season friendlies: the friendlies day, the AI pre-season plan and the pre-season windows; the pre-season tours: booking the tour matches and the two orders of the friendly candidates; the cup's predicted positions: positions by exit round, sorting the clubs by strength and storing the club's position in the statistics

**Routines:**
- `jogos_da_selecao_no_periodo()` = 0x58d710 (fifa_rankings.cpp, thiscall, ret 0x14): counts the results of the national side's history between two months (year × 12 + month), from the end, for the work permit;
- `atualizar_ranking_da_fifa()` = 0x58d460 (fifa_rankings.cpp, thiscall, ret 4): on the 18th: drops the results older than a year, sorts the histories, rolls the yearly totals and redoes scores and positions;
- `ordem_dos_resultados_por_pontos()` = 0x58e220 (fifa_rankings.cpp, cdecl): qsort order of the histories: the points of b minus those of a (highest first);
- `resultado_no_historico()` = 0x58dea0 (fifa_rankings.cpp, thiscall, ret 0xc): appends the points, month and year of the match to the national side's history, growing the array by 5 entries;
- `faixa_da_selecao()` = 0x58e190 (fifa_rankings.cpp, thiscall, ret 4): the national side's strength tier from 1 to 4, by reputation and ranking position;
- `pontuacoes_do_ranking()` = 0x58dfc0 (fifa_rankings.cpp, thiscall): the year total from the best eight results and the score from the totals of this year and the five before;
- `nacao_chave_do_jogo()` = 0x48dd70 (comp.cpp, cdecl): the active key of the key nation that owns the fixture: by the competition nation, by the nations of the friendly clubs, 0 for the World Cup, 1 for national side tournaments;
- `e_copa_do_mundo()` = 0x669a10 (key_nation.cpp, cdecl): 1 when the competition id is the World Cup or one of its six qualifying sections;
- `e_torneio_internacional()` = 0x669ae0 (key_nation.cpp, cdecl): 1 when the competition id is one of the ten national side tournaments outside the World Cup;
- `indice_da_nacao_chave()` = 0x669470 (key_nation.cpp, cdecl): the competition's key nation row: the nation's, 0 for the World Cup, 1 for national side tournaments, 2 for the club world cups, the continent's or 3;
- `ordem_do_ranking()` = 0x58d610 (fifa_rankings.cpp, thiscall, ret 4): a new array with the ranking entries of the nations with a national side, sorted by score, and the count;
- `ordem_das_pontuacoes()` = 0x58e240 (fifa_rankings.cpp, cdecl): qsort order of the ranking: the highest score first;
- `tempo_das_cidades()` = 0x922ee0 (weather.cpp, thiscall): each city: the three readings become (reading + random(100)) × 0.5, clamped to 0..100; one time in ten (random(10) zero), the new wind = random(3);
- `ler_regiao_do_clima()` = 0x922c40 (weather.cpp, thiscall, ret 8): one weather.cfg line with the region id and name ("%ld %s") and its four periods (0x922d20) at +0x1e, +0x31, +0x44 and +0x57;
- `ler_periodo_do_clima()` = 0x922d20 (weather.cpp, thiscall, ret 8): one line with the name and 18 integers (the day, five W, five P, seven H): the period gets the day (word) and the 17 bytes; a null file is the "Error" box;
- `coeficientes_das_nacoes()` = 0x5d5f20 (game.cpp, cdecl): the first time, each nation history gets the coefficient at +0xa8; on the table day of the month, the weighted history sum in an array that is sorted and freed; on the December day, the current year gets reputation × 0.05 × (12 − mark) × 3 (mark < 12), the history moves one year and the current year and the mark are cleared;
- `dia_das_aposentadorias()` = 0x5d6250 (game.cpp, cdecl): 1 on days 0, 80, 173 and 250 of the year (the retirement scan days), otherwise 0;
- `soma_presa()` = 0x5d61d0 (game.cpp, cdecl): the sum of two signed bytes clamped between the given minimum and maximum;
- `nota_do_estadio_neutro()` = 0x85da00 (stadium.cpp, thiscall, ret 0x14): the English national stadium before 2004 scores 0; the score starts at 100000, changes with the city distances (flag 8) or with the places against the expected crowd (flag 0x20: national team 60000, otherwise the clubs crowd sum) or becomes random(places) (flag 4), and ends × (random(100) × 0.004 + 0.8);
- `fator_da_distancia()` = 0x85d8a0 (stadium.cpp, thiscall, ret 0x14): 1 − distance × k (squared without +0x3c), divided by (reputation + 1)² when far and with reputation; with minimums by the base, by type 10 (0.2), by both clubs of the same nation (0.05) and a general one;
- `ler_o_clima()` = 0x922810 (weather.cpp, thiscall): reads CM3_DATA\weather.cfg: the region count, the regions (0x922c40) and the city table, with the three readings ((random(100) + random(100)) × 0.5) and the wind (random(3)) drawn; a file or memory error stops the game;
- `fator_do_treinador()` = 0x8b41d0 (training_manager.cpp, thiscall, ret 8): the coaching factor of a training category: 100 without a manager or with a computer manager; at a human club, the average rating of the coach assignments (divided by the divisor for their number) × 40/7, at least 50;
- `comissao_do_humano()` = 0x8b4cd0 (training_manager.cpp, thiscall, ret 4): the coach assignment table (six 9-byte slots) of one of the 16 human managers; otherwise the error box and 0;
- `carga_do_programa()` = 0x8b4f60 (training_manager.cpp, thiscall): the training schedule load from its five category levels with fixed weights, up to 100;
- `aplicar_o_progresso()` = 0x8b2be0 (training_manager.cpp, thiscall, ret 8): adds each category's progress to the player's stored nibble; every two steps the category attributes move (1 to 20 ones by one, −125 to 125 ones by six); then the trained position or side goes to 20 when learnt, or back to 1 when the training stops;
- `achar_o_treino_de_posicao()` = 0x8b4b20 (training_manager.cpp, thiscall, ret 8): finds the player's first entry in the sorted position training list and returns how many are his (0 without the position training mark);
- `aprende_depressa()` = 0x8b4db0 (training_manager.cpp, thiscall, ret 4): 1 when one of the player position training entries has a learning rate above 50;
- `avancar_a_semana()` = 0x8b1730 (training_manager.cpp, thiscall, ret 4): on a day of the year that is a multiple of 7 the training week goes up by 1 (each player training is caught up by the missing weeks); returns the day / 7;
- `tirar_a_ficha_reservada()` = 0x7a6e30 (player_regen.cpp, thiscall, ret 0xc): copies the first spare non-player record entry (id, nation, second nation) to the outputs and removes it from the list;
- `lesao_que_volta()` = 0x62ed30 (injury.cpp, thiscall, ret 4): the player's recurring injury type, or 0xff when he has none or is not a player;
- `atualizar_amistosos_do_dia()` = 0x5c2460 (friendly.cpp, thiscall, ret 8): on the pre-season days plans the AI friendlies and cancels old tournaments; every day creates the planned friendlies, answers requests and expires tournaments, requests and invitations;
- `planejar_pre_temporada_da_ia()` = 0x5c27a0 (friendly.cpp, thiscall, ret 0): builds one candidate per club with a nation (band = reputation / 500), sorts by band and by nation, sets the windows and plans tournaments, tours and friendlies;
- `janelas_da_pre_temporada()` = 0x5c2d40 (friendly.cpp, thiscall, ret 8): for each nation in use, writes to its candidates the window up to the season start − 1 week (18 days before, at least 3) and gives a date list to the league's first and last club;
- `marcar_jogos_da_excursao()` = 0x5c4040 (friendly.cpp, thiscall, ret 0xc): trims the window by the first opponent, draws the start (up to 5 tries) and books one friendly per opponent, 2 days apart, storing the dates in both lists;
- `ordem_pela_faixa()` = 0x5c5270 (friendly.cpp, cdecl): candidate comparator by band (+5), highest first;
- `ordem_pela_nacao_e_faixa()` = 0x5c5290 (friendly.cpp, cdecl): comparator of candidate pointers by nation (+6) and, within a nation, by band (except 0xff);
- `posicoes_previstas_da_copa()` = 0x5223a0 (Cup.cpp, thiscall, ret 0): sorts the cup clubs by strength and gives each one the predicted position of the round it goes out in, in the cup statistics;
- `ordenar_clubes_pela_forca()` = 0x4c08f0 (comp_util.cpp, cdecl): sorts an array of clubs by strength (0x4c0720), strongest first;
- `gravar_posicao_do_clube()` = 0x4a2540 (comp_stats.cpp, thiscall, ret 8): finds or adds the club's record in the competition statistics and stores the position at +0x72;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (39):** 33 caught. `cpt1_um_jogo_a_mais` escaped: the count of international games only matters for the work permit of a non-EU player joining an English club, and no permit flipped before 2002-06-01 [inference]. `cpt1_sem_o_mes_final` escaped: the same work permit count; no permit flips before 2002-06-01 [inference]. `cpt1_no_maximo_oito` escaped: the same work permit count; no permit flips before 2002-06-01 [inference]. `cpt2_ano_seguinte` escaped: the result in the national team history (256 times) only feeds the work permit count (cpt_1), which flips no permit before 2002-06-01 [inference]. `cpt2_mes_seguinte` escaped: the same national team history, only for the work permit count [inference]. `cpt4_sem_a_eliminatoria_europeia` escaped: with the European qualifiers under the continent rules, no rule barred a national team player before 2002-06-01 (the agent foresaw the escape) [inference]. (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (bc5827d1).
- **Rust ×2 and original careers to 2002-06-01:** Rust ×2 345e9799, original 30385bae (recorded; does not block the batch since 31/10/2026).

**Redo**, on top of batch 255's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 256`

**Fields touched:**
- [0xb74318]+0x1c: the FIFA ranking (FIFA_RANKINGS); pointer; reads (work permit 0x8c7950)
- FIFA_RANKINGS +0: nation table (0x2c bytes per nation id); pointer; reads (all three)
- FIFA_RANKINGS +4: month of the yearly roll; value; reads (atualizar_ranking_da_fifa)
- FIFA_RANKINGS +5: date of the last update (8 bytes); value; writes (atualizar_ranking_da_fifa)
- ranking entry +4/+8/+0xa: history (5-byte entries: points +0, month +2, year +3), capacity and count; reads and writes (atualizar_ranking_da_fifa), reads (jogos_da_selecao_no_periodo)
- ranking entry +0x10/+0x12: position and previous position; value; writes (atualizar_ranking_da_fifa)
- ranking entry +0x14..+0x28: totals of this year and the five before; value; writes (atualizar_ranking_da_fifa)
- [0xae2350] number of nations; value; reads (atualizar_ranking_da_fifa)
- ranking entry +4/+8/+0xa: history, capacity and count; reads and writes (resultado_no_historico), reads (pontuacoes_do_ranking)
- history entry (5 bytes): points +0, month +2, year +3; value; writes (resultado_no_historico)
- ranking entry +0xc: score; f32; writes (pontuacoes_do_ranking)
- ranking entry +0x10: position; value; reads (faixa_da_selecao)
- ranking entry +0x14: year total; value; writes (pontuacoes_do_ranking); +0x18..+0x28 the five before; reads
- nation +0x8e: reputation; value; reads (faixa_da_selecao)
- fixture +0x28/+0x2a: year and day of year; value; reads (resultado_no_historico)
- [0xdd6ec8] ranking reference date; value; reads (pontuacoes_do_ranking)
- key nation table 0xb63d60 (35 × 72 bytes): nation +0, active key +0x14; reads (nacao_chave_do_jogo)
- nation +0x11c: nation chosen to play; value; reads (nacao_chave_do_jogo)
- competition +0 and +0x5d: id and nation; reads (all three)
- fixture +0x14, +0x1c, +0x20: competition, home and away club; pointer; reads (nacao_chave_do_jogo)
- club +0x53: nation; pointer; reads (nacao_chave_do_jogo)
- [0x9cf684] reserve competition; [0x9cf76c..0x9cf7ac], [0x9cf8a0], [0x9cf964] national side competition ids; id; reads
- key nation table 0xb63d60 (35 × 72 bytes): nation +0, continent +4; reads (indice_da_nacao_chave)
- competition +0, +0x59, +0x5d: id, continent and nation; reads (indice_da_nacao_chave)
- nation +0x71 (0x122-byte records at [0xae23a8]): the national side; pointer; reads (ordem_do_ranking)
- ranking entry +0xc: score; f32; reads (ordem_das_pontuacoes)
- [0x9cf6e4] and [0x9cf7bc] World Club Cup and FIFA Club World Championship; id; reads (indice_da_nacao_chave)
- Weather [0xde2f74] +4 city table (4 bytes per city: +0..+2 readings 0..100, +3 wind 0..2); pointer/values; reads and writes (tempo_das_cidades)
- [0xae2354] city count; value; reads (tempo_das_cidades)
- weather region (0x6e bytes) +0 id, +4 name, +0x1e/+0x31/+0x44/+0x57 the four periods; values; writes (ler_regiao_do_clima)
- weather period (0x13 bytes) +0 day (word), +2..+6 W, +7..+0xb P, +0xc..+0x12 H; values; writes (ler_periodo_do_clima)
- nation (0x122 bytes at [0xae23a8], count [0xae2350]) +0xa8 coefficient; double; reads (coeficientes_das_nacoes)
- nation +0xb0..+0xd0 five-year history, +0xd8 current year; doubles; reads and writes (coeficientes_das_nacoes)
- nation +0x8e reputation; value; reads (coeficientes_das_nacoes)
- nation +0x121 December bonus mark (< 12 earns it); value; reads and clears (coeficientes_das_nacoes)
- [0x9cd200] update day of each month (12 words), [0x9cd216] the December one, [0x9cd218] history to initialise; values; reads, clears [0x9cd218] (coeficientes_das_nacoes)
- stadium (0x4e bytes at [0xae23b0]) +0x38 city, +0x3c places; pointer/value; reads (nota_do_estadio_neutro)
- club +0x69 stadium, +0x73 average crowd; pointer/value; reads (nota_do_estadio_neutro)
- city +0x23/+0x2b coordinates; doubles; reads (nota_do_estadio_neutro, fator_da_distancia)
- nation +0x80 national stadium; pointer; reads for England [0x9cf2e4] (nota_do_estadio_neutro)
- fixture +0x1c/+0x20 clubs, +0x30 flags (0xa0), +0x3a type, +0x3c flag; pointers/values; reads (nota_do_estadio_neutro, fator_da_distancia)
- club nation (+0x53) +0x7e reputation; value; reads (fator_da_distancia)
- Weather [0xde2f74] +0 regions (0x6e bytes), +4 city table, +8 region count; pointers/value; writes (ler_o_clima)
- club +0xcf: manager; pointer; reads (fator_do_treinador)
- person +0: id (the last 16 of [0xae2364] are the human managers); id; reads (fator_do_treinador, comissao_do_humano)
- TRAINING_MANAGER +0x652d + 54 × human: coach assignments, six 9-byte slots (+0 coach id, −1 empty; +4 + category: 1 to 20 rating); id and value; reads (fator_do_treinador, comissao_do_humano)
- TRAINING_SCHEDULE +4..+8: levels of the five categories (0, 10, 25, 50); value; reads (carga_do_programa)
- person +0 and +0x61: id and player record; id and pointer; reads (aplicar_o_progresso, achar_o_treino_de_posicao)
- player record +0x1b, +0x1d, +0x22, +0x29, +0x2e, +0x34, +0x36, +0x3c, +0x3d, +0x3f, +0x40: 1 to 20 attributes; value; reads and writes (aplicar_o_progresso)
- player record +0x23, +0x24, +0x26, +0x27, +0x2a, +0x2b, +0x31, +0x32, +0x33, +0x35, +0x37, +0x38, +0x39, +0x3a, +0x3e, +0x41: −125 to 125 attributes; value; reads and writes (aplicar_o_progresso)
- player record +0xf..+0x15 and +0x17..+0x19: positions and sides (1 to 20); value; reads and writes 20 or 1 (aplicar_o_progresso)
- TRAINING_MANAGER +0x651d: 6-byte training entries by player record (+3 and +4 progress nibbles of categories 0, 1, 2 and 3; +5 low: category 4; +5 bit 0x10: has position training); value; reads and writes (aplicar_o_progresso), reads (achar_o_treino_de_posicao)
- TRAINING_MANAGER +0x6521 position training list (8-byte entries: +0 person id, +4 position or side mask, +6 state, +7 learning rate), +0x6525 count; pointer and value; reads (all three)
- [0xae2c90] game date (day of the year, 16 bits); value; reads (avancar_a_semana)
- TRAINING_MANAGER +0x688d: training week (16 bits); value; reads and writes (avancar_a_semana)
- PLAYER_REGEN +0x1c: spare non-player records (12-byte entries: id, nation, second nation); id; reads and removes the first (tirar_a_ficha_reservada)
- Injury +0 fitness records (31 bytes per person; +0x12 recurring injury type, 0xff for none); value; reads (lesao_que_volta)
- person +0 and +0x61: id and player record; id and pointer; reads (lesao_que_volta)
- Friendly +0xa7: the season start (date); +0xb1 the 127 tournaments (0x69 bytes: +0 in use, +4 the date, +0x65 the list); +0x34c8 the requests (+0x10/+0x14 the clubs, +0 the date); +0x34cc the invitations (10 of 0x19 bytes, date at +9, flag at +0x15); +0x34d0/+0x34d4 the replies; +0x34e8/+0x34ea the pre-season days; value and pointer; reads and writes (atualizar_amistosos_do_dia)
- friendly candidate (0x16 bytes) +0: club id; +5 the band; +6 nation id; +0xa/+0x12 the lists; +0xe/+0x10 the window; value and pointer; writes (planejar_pre_temporada_da_ia, janelas_da_pre_temporada)
- club +0x53: the nation; +0x80 the reputation (word); pointer and value; reads (planejar_pre_temporada_da_ia, janelas_da_pre_temporada)
- [0xb63d60] the 35 nation seasons (0x48 bytes: +0 the nation, +0x26 the league, +0x2a the start); [0xdd6ec8] the game start; value and pointer; reads
- friendly candidate (0x16 bytes) +0: club id; +5 the band (signed byte; 0xff = any); +6 nation id; +0xe/+0x10 the window (signed words); +0x12 the date list (8 bytes each); value and pointer; reads and writes the list (marcar_jogos_da_excursao, ordem_pela_faixa, ordem_pela_nacao_e_faixa)
- Friendly +0xa7: the season start (date); value; reads (marcar_jogos_da_excursao)
- Cup +8: the statistics (Comp_Stats); +0x3c the round count (word); +0x3e the club count (word); +0xa3 the rounds (0x68 bytes, +0x1a how many go out in it); +0xa7 the clubs (6 bytes, +0 the club); pointer and value; reads (posicoes_previstas_da_copa)
- club record in the competition statistics +0x72: the predicted position; value (byte); writes (gravar_posicao_do_clube)

**Of interest to:** CMRTE, GPF2, CMScout, SGE
