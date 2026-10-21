# Batch 199 — the match statistics: the player league stat, the fixture entry and the record bucket

**Routines:**
- `estatistica_da_liga_do_jogador()` = 0x7c47b0 (player_stats.cpp, thiscall): the player's league appearances (field 1) or league goals (field 2) at the club, as a float;
- `entrada_do_jogo_bate()` = 0x729500 (match_screens.cpp, thiscall): reports whether a match statistics entry holds the two given clubs in either order and the given date;
- `balde_do_registro()` = 0x729bd0 (match_screens.cpp, stdcall): the bucket (0 to 3) where the match statistics record is stored: competition type, national team and fixture flags;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (3):** 1 caught. `e_data` escaped: in the career the fixture entry second date never decides (when the first matches, the second matches too). `b_tipo` escaped: the record bucket branch without the +0x40 flag does not run in the automatic career. (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (034ff3eb3e55dfd3).

**Redo**, on top of batch 198's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 199`
