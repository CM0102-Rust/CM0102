# Batch 150 — the automatic selection (0x6dbc50)

**Routine:** `escalar_automatico()` = 0x6dbc50 (match_man.cpp, ~7.7 KB), the selection the coaching staff makes against the opponent.
- Outside the human's club, compares today's match with the next one on the list (competition reputation, rival, same city, days until it) to decide whether to rest regulars.
- Filters the 50 players of the lines (state, suspension, already played, barred, on loan), selects (0x6e2000), picks the formation, the three references, the rotation, the captain, the instructions and the general orders by the stance.
- With it, `match_man.cpp` is 100% in Rust.

**How it was ported:** the locals live in a byte frame with the same offsets as the original stack, zeroed like the determinised original (0x6dbc50 is in `ZERADAS`). So the vectors overlap as there, e.g. the short of the 1st extra slot that the original never writes.

**How it was compared:** no comparison of its own (it allocates the list and the rotation vectors and calls 24 routines still in the original), through the career (Rust × original), like batches 145, 148 and 149. The routines it calls stay compared.

**Result** (`cadeia.txt`, `mutacoes_refeitas.txt`):
- Mutations (17): 12 caught and 5 escaped; the 5 redone stronger: all 5 caught (no gap).
- Real game comparing to 2002-01-01: 0 divergences (6500 selections by the probe, none with garbage).
- Rust ×2 and original careers to 2002-06-01: fingerprint 459815f5e0dca92b.

**Redo:** on top of batch 149's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 150 -`
