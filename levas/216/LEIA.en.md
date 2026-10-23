# Batch 216 — whether the league has relegation, the league schedule round date and the league match importance; the club records copies of the record holder, the match record and the run record; the fixture watched by any human, the nickname given by the human and the pending retirement copy

**Routines:**
- `liga_tem_rebaixamento()` = 0x6847b0 (league.cpp, thiscall): whether the league has relegation places at the bottom of the table;
- `data_da_rodada_da_liga()` = 0x68a160 (league.cpp, cdecl): stores in a league schedule round record the date with a relative year, the code and a value;
- `importancia_do_jogo_da_liga()` = 0x68a3e0 (league.cpp, thiscall): the league match importance for the club: the league base, +1 early in the season or with the final-day margin;
- `copiar_recordista()` = 0x448490 (club_records.cpp, thiscall): copies the record holder: four identity fields and the date;
- `copiar_registro_de_partida()` = 0x448110 (club_records.cpp, thiscall): copies the match record: the date, the opponent, the score and the fixture fields;
- `copiar_registro_de_sequencia()` = 0x448170 (club_records.cpp, thiscall): copies the run record: the game count and the start and end dates;
- `humano_assiste_a_partida()` = 0x5fe420 (human_manager.cpp, thiscall): whether any of the active human managers chose to watch the fixture;
- `humano_deu_apelido()` = 0x600740 (human_manager.cpp, thiscall): whether the human manager (or the current one) gave the person a nickname, by binary search in his list;
- `copiar_aposentadoria_pendente()` = 0x7a8780 (player_regen.cpp, thiscall): copies the pending retirement record: the person, the club, the two dates, the reason and the announced flag;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (9):** 8 caught. `ha_motivo` escaped: the reason (+0x18) of the copied pending retirement is not read in a deciding way up to 2002-06-01, and in the real game comparing (to 2002-01-01) the copy catches no retirement whose reason changes (retirements are settled at season end). (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (b28542870027e432).

**Redo**, on top of batch 215's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 216`

**Fields touched:**
- league +0xc1: has relegation places; value; reads (liga_tem_rebaixamento)
- league schedule round (0x41 bytes) +0: day of year; value; writes (data_da_rodada_da_liga)
- league schedule round +2: relative year (year − base); value; writes (data_da_rodada_da_liga)
- league schedule round +4: unnamed (code); value; writes (data_da_rodada_da_liga)
- league schedule round +0x3d: unnamed; value; writes (data_da_rodada_da_liga)
- league +0x51: unnamed (gives the base importance: (x − 10) / 4 + 10); value; reads (importancia_do_jogo_da_liga)
- league +0x3e: club count; value; reads (importancia_do_jogo_da_liga)
- league +0xb1: table (59-byte rows); pointer; reads (importancia_do_jogo_da_liga)
- table row +0: club; pointer; reads (importancia_do_jogo_da_liga)
- table row +0x37: unnamed (mark; 0xff = none); value; reads (importancia_do_jogo_da_liga)
- league +0xa9: unnamed (rounds; test > 10); value; reads (importancia_do_jogo_da_liga)
- match +0x1c/+0x20: home and away club; pointer; reads (importancia_do_jogo_da_liga)
- match +0x34: unnamed (test < 3); value; reads (importancia_do_jogo_da_liga)
- record holder (0x18 bytes) +0..+0xc: four identity fields, each unnamed; value; reads and writes (copiar_recordista)
- record holder +0x10: date; value; reads and writes (copiar_recordista)
- record match entry (0x1f bytes) +0: date; value; reads and writes (copiar_registro_de_partida)
- record match entry +8: opponent; value; reads and writes (copiar_registro_de_partida)
- record match entry +0xc, +0xe, +0x10, +0x11, +0x15, +0x19, +0x1a, +0x1b: unnamed (score and fixture fields); value; reads and writes (copiar_registro_de_partida)
- run record (0x14 bytes) +0: game count; value; reads and writes (copiar_registro_de_sequencia)
- run record +4/+0xc: start and end dates; value; reads and writes (copiar_registro_de_sequencia)
- [0xae23bc] staff table (110 bytes); pointer; reads (humano_assiste_a_partida)
- [0xae2364] staff count (the last 16 are the humans); value; reads (humano_assiste_a_partida, humano_deu_apelido)
- staff +0x5f: active human manager; value; reads (humano_assiste_a_partida)
- staff +0: person id; id; reads (humano_deu_apelido)
- [0xb77c7a] current slot (channel of the current human); value; reads (humano_deu_apelido)
- [0xb74c26 + 0x300 × slot] channel human manager; pointer; reads (humano_deu_apelido)
- HUMAN_MANAGER +0: human records (0xa6ef bytes); pointer; reads (humano_deu_apelido)
- HUMAN_MANAGER +4: human count; value; reads (humano_deu_apelido)
- human record +0xa6e3: nickname list (0x1e-byte items, +0 person id); pointer; reads (humano_deu_apelido)
- human record +0xa6eb: nickname count; value; reads (humano_deu_apelido)
- pending retirement (0x1a bytes) +0: person; id; reads and writes (copiar_aposentadoria_pendente)
- pending retirement +4: club; id; reads and writes (copiar_aposentadoria_pendente)
- pending retirement +8/+0x10: two dates, each unnamed; value; reads and writes (copiar_aposentadoria_pendente)
- pending retirement +0x18: reason; value; reads and writes (copiar_aposentadoria_pendente)
- pending retirement +0x19: announced flag; value; reads and writes (copiar_aposentadoria_pendente)

**Of interest to:** CMRTE, SGE, GPF2
