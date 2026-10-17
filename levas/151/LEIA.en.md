# Batch 151 — the progress of the match day (0x6b5a70, 0x6b5ce0, 0x6b5f00)

**Routines:**
- `iniciar_proxima_partida()` = 0x6b5a70 (match_day.cpp): starts the day's next match (creates and prepares the engine); in the 1st round with none, moves to the 2nd.
- `andar_partida_do_dia()` = 0x6b5ce0 (match_day.cpp): one engine step of the current match, or to the end; when it finishes, creates the summary, records it and frees the engine.
- `fechar_partida_do_dia()` = 0x6b5f00 (match_day.cpp): copies the result into the match, records it in the competition and runs the post-match (players, events, contracts, suspensions, states, history, clubs).

**How it was compared:** no comparison of their own (they create and free engines and summaries), through the career (Rust × original), like batches 148 to 150.

**Branch counters from the start** (Lucas's idea, from this batch on): each mutated branch has a `ramo("...")`; the base run shows which ones run. Here it showed that the step by step and the human's round do not run in the automatic career.

**Result** (`cadeia.txt`, `mutacoes_refeitas.txt`):
- Mutations (14): 6 caught and 8 escaped. Two were weak and, redone stronger, were caught. One is equivalent (a redundant test). The other 5 became 4 gaps in `docs/PENDENCIAS.en.md`.
- Real game comparing to 2002-01-01: 0 divergences.
- Rust ×2 and original careers to 2002-06-01: fingerprint 459815f5e0dca92b.

**Redo:** on top of batch 150's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 151 -`
