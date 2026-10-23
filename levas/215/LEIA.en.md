# Batch 215 — the end of the match screens: the match the human leaves and the fixture of a statistics record

**Routines:**
- `largar_partida_assistida()` = 0x6b6970 (match_day.cpp, cdecl): the current human leaves the match he was watching; with no other human watching, the match runs to the end, is recorded and leaves the queue;
- `ficha_do_registro()` = 0x729e30 (match_stats.cpp, thiscall): finds in the fixture list the match of a statistics record (date, competition and club) and copies the fixture;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (0):** 0 caught.
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (b28542870027e432).

**Redo**, on top of batch 214's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 215`

**Fields touched:**
- [0xb77c7a] current slot (channel of the current human); value; reads (largar_partida_assistida)
- [0xb74c26 + 0x300 × slot] channel human manager; pointer; reads (largar_partida_assistida)
- [0xae23bc] staff table (110 bytes); pointer; reads (largar_partida_assistida)
- [0xae2364] staff count (the last 16 are the humans); value; reads (largar_partida_assistida)
- staff +0x5f: active human manager; value; reads (largar_partida_assistida)
- human record (0x765480) +0xe2: match he watches; pointer; reads (largar_partida_assistida)
- match +0x5b: summary (per-slot vectors at +5/+0x45/+0x85/+0xc5); pointer; reads, and writes 0 at the slot entry (largar_partida_assistida)
- summary +0x105..+0x108: counters of the four vectors; value; reads and writes −1 (largar_partida_assistida)
- match +0x57: match engine (0xf63b bytes); pointer; reads and writes (largar_partida_assistida)
- match +0x67: match state (2 or 8 = end); value; reads and writes (largar_partida_assistida)
- match +0x5f: statistics record (0xd7a bytes); pointer; writes (largar_partida_assistida)
- [0xb67a50] (MATCH_DAY) +0x25: unnamed (counts matches reaching state 2 or 8); value; writes +1 (largar_partida_assistida)
- [0xb6b7c0] match statistics (Match_Stats); pointer; reads (largar_partida_assistida)
- [0xde2f70] VIRTUAL_STAFF (match queue); pointer; reads (largar_partida_assistida)
- Match_Stats +0: record index (12 bytes each); pointer; reads (ficha_do_registro)
- Match_Stats +0x1c: record count; value; reads (ficha_do_registro)
- statistics index +0: competition; id; reads (ficha_do_registro)
- statistics index +4: one of the match clubs; id; reads (ficha_do_registro)
- statistics index +8/+0xa: match date; value; reads (ficha_do_registro)
- [0xae2a58] fixture list (FIXTURE_MANAGER); pointer; reads (ficha_do_registro)
- fixture (0x4f bytes) +0xc/+0x10: home and away club; id; reads, and copies the whole fixture (ficha_do_registro)

**Of interest to:** SGE
