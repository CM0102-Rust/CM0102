# Batch 154 — the recorded events and the network step (0x6d98b0, 0x6d9c40, 0x6b55c0)

**Routines:**
- `gravar_eventos()` = 0x6d98b0 (match_events.cpp): records the match events in the "events1/2.tmp" files in CM3_TEMP (human or computer), recreating the oldest when both are full.
- `ler_eventos()` = 0x6d9c40 (match_events.cpp): reads the events back (to replay the plays).
- `passo_em_rede()` = 0x6b55c0 (match_day.cpp): one step of the human's match on a network, by the clock and the humans' scoreboard.

**How it was compared:** no comparison of their own, through the career (Rust × original), with branch counters.

**Result** (`cadeia.txt`):
- Mutations (6): all 6 escaped. Recording runs in the career, but the files are not part of the fingerprints; reading back and the network step do not run. Two gaps in `docs/PENDENCIAS.en.md`; the test is in the game (replaying the plays of a watched match).
- Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3).
- Rust ×2 and original careers to 2002-06-01: all three equal (034ff3eb3e55dfd3).

**Redo:** on top of batch 153's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 154 -`
