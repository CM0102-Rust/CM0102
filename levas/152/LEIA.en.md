# Batch 152 — the match day loop (0x6b4360)

**Routine:** `jogar_o_dia()` = 0x6b4360 (match_day.cpp, ~3.3 KB): plays the day. Starts the next match, ties the humans to their matches (the "network" branch), creates and plays to the end each match without a human, writes the score with the percentage on the bar, closes each match and frees the summaries.

**How it was compared:** no comparison of its own, through the career (Rust × original), with branch counters from the start.

**New reference (important):** with the day loop hooked, everything it calls now runs on the simulation's private stack, including the original routines that read a local variable with no value. The garbage they read changed: the career changed in **both** modes, equally in both. The isolated original and Rust stay identical; what changed is the reference:
- Careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches 19414/b3ee24bdc00da55f, database 7f2c15024cd99e7e (before: 459815f5e0dca92b).
- Real game comparing to 2002-01-01: fingerprint 963cea289f055ac3 (before: f2db52ca99e0789d).

**Result** (`cadeia.txt`, `mutacoes_refeitas.txt`):
- Mutations (8): 5 caught and 3 escaped. The queue one was weak and, redone (without the call), was caught. The humans' lines one does not run in the automatic career (new gap in `docs/PENDENCIAS.en.md`). The finished counter one is the gap already noted in batch 151.
- Real game comparing to 2002-01-01: 0 divergences.
- Rust ×2 and original careers to 2002-06-01: all three equal (034ff3eb3e55dfd3).

**Redo:** on top of batch 151's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 152 -`
