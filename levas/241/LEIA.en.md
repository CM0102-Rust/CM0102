# Batch 241 — the league table: sorting, the head-to-head mini table, freeing and checking completion; the league season end: the TV rights, the season honours and the competition history record; the competition helpers: the league clash type, the club strength score and the latest placed club; competitions: the arranged friendly of a club and the fixture list for a date; competitions: the next fixture by filter, the stadium in use on a day and the international friendly date

**Routines:**
- `ordenar_tabela_da_liga()` = 0x6827d0 (league.cpp, thiscall): sorts the table by the rules (+0xc5): rule 6 only sorts (0x683210); rule 4 builds the mini table and, when first, sorts by head-to-head (0x683410) and bubbles the ties; stores the position (+4) and the position history (+9); returns 1;
- `mini_tabela_do_confronto()` = 0x682b30 (league.cpp, thiscall): mini table of the clubs level on points with their summed head-to-head results, sorted by 0x6834f0; the position (+4) starts at the club count and drops at each separated pair;
- `liberar_tabela_da_liga()` = 0x687970 (league.cpp, thiscall): frees the +5 and +9 row buffers and the table (null: the league one, then clears +0x3e and +0xb1);
- `liga_completa()` = 0x689ad0 (league.cpp, thiscall): 1 when every club played the +0xa7 rounds (and the stages, if asked); at the end of the league, the records (0x86a070), the prize money (0x682e10), the notice (+0x94) and +0xdd;
- `pagar_direitos_de_tv()` = 0x683010 (league.cpp, thiscall): adds the amount to the TV revenue of every club in the table (with a filter, only those with +0x5b equal) and sends the news (0x48ea40);
- `registrar_honras_da_temporada()` = 0x684640 (league.cpp, thiscall): champion, runner-up and third into the history (0x4afce0), with the host nation team as the featured entry when the competition asks (+0x88, 0x5fa730, 0x53b3d0); a champion of this division clears +0x5b of the clubs pointing at it;
- `acrescentar_ao_historico()` = 0x4afce0 (comp_util.cpp, cdecl): stores competition, year and four values in the free row of the competition history (without one, grows 20 rows by 0x527f60); returns the row;
- `tipo_de_confronto_na_liga()` = 0x4b02e0 (comp_util.cpp, cdecl): 1 = title clash, 3 = zone clash, 0 = none, between two clubs of the same league, from the weighted statistics ratings (0x4a4850) against a third of the clubs or 0.2 × clubs + +0xbe (+ +0xbf);
- `forca_do_clube()` = 0x4c0720 (comp_util.cpp, cdecl): money (+0x65 / 400 or balance / 300) + 16 × reputation + the off-match ratings (0x6e6340) of the 15 picked (0x6e2000) from the squad; null club: −10,000,000;
- `clube_colocado_mais_recente()` = 0x4afd80 (comp_util.cpp, cdecl): sorts the competition history and returns the winner, runner-up or third (+0xa, +0xe, +0x12) of the first row of the competition; not found: 0;
- `amistoso_do_clube()` = 0x5c88f0 (friendly.cpp, thiscall, ret 8): the friendly of the club (+0xc) in the friendly list (+0x34c8): with a date, those on that date (0x4a7f20), the complete one (+0x10 and +0x14) or the second found at once, otherwise the last kept; without a date, the first incomplete one with +8;
- `lista_da_data()` = 0x5ae6a0 (fix_man.cpp, stdcall, ret 0xc): the fixture list (0x5abf10 of the manager [0xae2a58]) for the year (+2) and day (+0) of the date, in the given slice and group;
- `proxima_partida()` = 0x5ac920 (fix_man.cpp, thiscall, ret 0x2c): the next (or previous) fixture from the year/day/slice cursor, inclusive, by the competition (+0), stage (+0x34) and two club (+0xc/+0x10) filters, each -1 = any; up to 5 years; writes the cursor back and returns the fixture, or 0;
- `estadio_em_uso()` = 0x5ae870 (fix_man.cpp, thiscall, ret 0x10): 1 when a fixture (+0x18) in the day lists (groups 0 to 7 and the stadium one, 0x48df40) uses the stadium, in the given slice or in slices 0 to 2 (slice -1);
- `data_do_amistoso_internacional()` = 0x5cbdd0 (friendly.cpp, thiscall, ret 4): the date (+3) through 0x54a130 from the day (+1), month (+2), given year and weekday (+0); the high half of the arguments comes from the year (the caller eax);

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (15):** 11 caught. `cpa_ordem_posicao` escaped: ordenar_tabela_da_liga 0x6827d0: disabled routine (DESLIGADAS in motor.rs): no hook in any mode, the copy does not run. `cpa_completa_rodadas` escaped: 0x689ad0: disabled routine (DESLIGADAS in motor.rs): no hook in any mode, the copy does not run (disabled on 26/10/2026). `cpa_tv_valor` escaped: pagar_direitos_de_tv 0x683010 is not called before 2002-06-01 (the "liga direitos de tv" branch does not appear in the base); also escaped the game comparing and the pt load (0 divergences). `cpa_confronto_titulo` escaped: tipo_de_confronto_na_liga 0x4b02e0: disabled routine (DESLIGADAS in motor.rs): no hook in any mode, the copy does not run. (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (b28542870027e432).

**Redo**, on top of batch 240's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 241`

**Fields touched:**
- League +0x3e clubs (short), +0x3c legs, +0xa7 total rounds (short; −1 = compute), +0xad column, +0xb1 table, +0xc5 rules, +0xa3 own compare, +0x44 stage, +0xdd complete, +0xe6, +0xa9, +0x2c/+0x30/+0xc stages; reads and writes (all four)
- table row (0x3b bytes): +0 club, +4 position, +5 head-to-head (14-byte entries), +9 position history, +0x11 played, +0x19 points; reads and writes (ordenar_tabela_da_liga, mini_tabela_do_confronto)
- [0xb679fc] league being sorted, [0xb67a00] own compare; writes (ordenar_tabela_da_liga)
- [0xae2a38] finance table; club finance (0x167 bytes) +0, +0xb8, +0x158 (balance and TV revenue); writes (pagar_direitos_de_tv, through 0x5987c0)
- TClub +0x5b: competition (previous division); reads (pagar_direitos_de_tv) and clears (registrar_honras_da_temporada); TClub +0x57: division; reads
- [0xae23d4] / [0xae237c]: competition history (26 bytes: +4 competition, +8 year, +0xa/+0xe/+0x12/+0x16) and count; [0xae2c92] year; reads and writes (acrescentar_ao_historico)
- TNation (0x122 bytes, [0xae23a8]) +0x8e; reads (registrar_honras_da_temporada)
- TClub +0x57 division; reads. League ([0xadadfc][id]) +8 stats, +0x30 stages, +0x3e clubs, +0x43, +0xbe, +0xbf, virtual +0x98; reads. Stats entry +4 played, +0x72..+0x75; reads (tipo_de_confronto_na_liga)
- TClub +0x65 cash, +0x80 reputation; [0xae2a38] finances (64-bit balance); reads (forca_do_clube)
- [0xae23d4] / [0xae237c] competition history (26 bytes); re-sorts and reads (clube_colocado_mais_recente)
- friendly manager +0x34c8: friendly list; friendly +8, +0xc club, +0x10, +0x14; reads (amistoso_do_clube)
- date: +0 day, +2 year (shorts); reads (lista_da_data)
- fixture: +0 competition, +0xc/+0x10 clubs, +0x18 stadium (pointers), +0x34 stage (short); reads (proxima_partida, estadio_em_uso)
- fixture list: +2 head, +0xa cursor; writes the cursor (proxima_partida, estadio_em_uso)
- international date: +0 weekday, +1 day, +2 month (bytes), +3 date; writes the date (data_do_amistoso_internacional)

**Of interest to:** SGE, CMRTE
