# Batch 148 — the engine is born, prepares the match and dies (0x6b7e50, 0x6b7f20, 0x6b97a0)

**Routines:**
- `construir_motor()` = 0x6b7e50: the engine constructor.
- `preparar_partida()` = 0x6b7f20 (6 KB): prepares each match. It sets up:
  - cleared state, weather, kits, referee and pitch;
  - the two squads (0x6db500, still in the original) and the commentator;
  - the side strength and the marking;
  - the deflected ball's angle table;
  - the pre-match event list: contract unhappiness, interest from another club, form and contract end.
- `destruir_motor()` = 0x6b97a0: the destructor.

With them, `match_eng.cpp` is 100% in Rust (117 of 117 routines).

**How it was compared:** with no comparison of its own, through the career (Rust × original), like batch 145's engine loop.
- The three allocate or free (the commentator, the event list), and the tape would have to reproduce the allocations.
- The routines they call stay compared.
- The angle table, which the original computes with `fpatan`, comes from Rust's `atan`. The career checks it: the mutation that changes an angle was flagged.

**Result** (`cadeia.txt`, `mutacoes_refeitas.txt`):
- Mutations (18 + 4 redone): 16 flagged and 2 gaps, both in `docs/PENDENCIAS.en.md`:
  - the rainy sky;
  - the unhappiness event's code.
- Real game comparing to 2002-01-01: 0 divergences.
- Rust ×2 and original careers to 2002-06-01: fingerprint 459815f5e0dca92b.
- Scoreboard: the match engine went to 73.9%.

**Redo:** on top of batch 147's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 148 -`
4. `python mutacoes.py mutacoes\leva148b.py --carreira 2002-06-01`
