# Batch 153 — the match day setup (0x6b3c10, 0x714d30, 0x6d8c70)

**Routines:**
- `montar_o_dia()` = 0x6b3c10 (match_day.cpp): sets up the match day (pending matches of each list, grouped by list and competition, the global list and the day's interest).
- `lado_do_jogador()` = 0x714d30 (match_pl.cpp): the player's side (+0x27). With it, `match_pl.cpp` has no real routines left (the other 2 in the scoreboard are false boundaries inside 0x7109f0).
- `sons_terminados()` = 0x6d8c70 (match_events.cpp): have the marked events' sounds played?

**How it was compared:** no comparison of their own, through the career (Rust × original), with branch counters.

**Test alongside another project:** the `mutacoes.py` lock now only looks at the `cm0102.exe` in this folder (another project on the machine, CMRTE, has a test exe with the same name). The chain ran with it open and the times stayed equal to the previous batches (mutations 137–161 s, real game 1755 s, careers 140–160 s).

**Result** (`cadeia.txt`, `mutacoes_refeitas.txt`):
- Mutations (8): 4 caught and 4 escaped; by the branch counters, 3 gaps in `docs/PENDENCIAS.en.md` (the day's 1st round, the matches already marked -2, the event sounds).
- Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3).
- Rust ×2 and original careers to 2002-06-01: all three equal (034ff3eb3e55dfd3).

**Redo:** on top of batch 152's commit (and the extension point's):
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 153 -`
