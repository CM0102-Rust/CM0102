# Batch 156 — the event text and the start of the network match (0x6d8d10, 0x6b5260)

**Routines:**
- `texto_do_evento()` = 0x6d8d10 (match_events.cpp): builds the event's commentary line, replacing the `events_*.cfg` tokens (player, club, competition, stage, number, referee) with prefix, suffix and upper case.
- `confirmar_partida_em_rede()` = 0x6b5260 (match_day.cpp): each human confirms the network match; with all of them, the engine is created and prepared for watching.

**How it was compared:** the event text got a real comparison (it only writes to the text buffers): in `JOGAR_COMPARANDO`, each commentary line is built by both versions and checked. The network confirmation, through the career.

**Result** (`cadeia.txt`):
- Mutations (3): all 3 escaped; both routines only run with the match screen (counters at zero), gap in `docs/PENDENCIAS.en.md`. The test is in Lucas's game.
- Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3).
- Rust ×2 and original careers to 2002-06-01: all three equal (034ff3eb3e55dfd3).

**Redo:** on top of batch 155's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 156 -`
