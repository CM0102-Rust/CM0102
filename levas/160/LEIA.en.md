# Batch 160 — player statistics and centre-backs in the formation

**Routines:**
- `estatistica_do_jogador()` = 0x7c2150 (player_stats.cpp, stdcall): statistic `qual` 1..22 of the record (bytes, percentages ×100.0f, per-match averages, the two ratings completed with under 20 matches ×0.05f), in x87 like the original;
- `media_das_notas()` = 0x7c1950 (player_stats.cpp, thiscall): the average of the record's five 4-bit ratings (zero counts 27), 16-bit, divided by 4;
- `zagueiros_sem_conflito()` = 0x5b8dc0 (formation.cpp, thiscall): 0 if any slot with an open side is a centre-back's or defensive midfielder's;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (7):** 7 caught.
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (034ff3eb3e55dfd3).

**Redo**, on top of batch 159's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 160`
