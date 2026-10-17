# Batch 147 — the shot (0x702050)

**Routines:**
- `chutar()` = 0x702050 (match_pl.cpp, 35 KB and 10,356 instructions, the largest ported so far). It executes every shot: the challenger with foul or dispossession, the commentary, offside, the interception on the way to goal, the outcome class and the goal (which can be disallowed), the save, the post or the shot wide, with hundreds of commentary lines.
- `atras_no_agregado()` = 0x6d5950 and `companheiro_livre()` = 0x6d5a70, two small queries that only the shot uses.

**How it was read:** from the package (`pacote.py`), with Ghidra's pseudocode for the structure and the assembly for every detail (stack offsets, order of the draws, jump tables checked byte by byte).
- Points the pseudocode hid:
  - reused stack variables: the type flag becomes the goal ratio in floating point, and the minute becomes the number of markers;
  - the division by 50 (not by 100) of the miss cost;
  - the `and dl` that gives 220 (not 476) in the on-target gain;
  - the offside search's argument in another order.
- The routine is recursive: the foul on a set-piece kick becomes a free kick (type 0x33) and modes 5 and 6 repeat the move.

**How it was compared:** through the tape, with comparison inside the unit, because it runs as a unit of the ball carrier 0x7109f0, of the shoot-out 0x6be710 and of itself.
- The old recording port goes away: the comparator records the unit whenever there is a tape.
- The interception 0x6cc540 runs inside the copy, because its comparator records no unit. The plays 0x70dfd0 are the units, with the mask only on the style's low byte (the original passes garbage in the high bytes).

**Result** (`cadeia.txt`, `comparar_0820.txt`):
- Comparing to 2001-08-20: 0 divergences, with 23 of 31 branches exercised.
- Mutations (26): 25 flagged by the career and 1 gap. It is `k_bloqueio`: the shot blocked on the way did not happen and is in `docs/PENDENCIAS.en.md`.
- Real game comparing to 2002-01-01: 0 divergences.
- Rust ×2 and original careers to 2002-06-01: fingerprint 459815f5e0dca92b.
- Scoreboard: the match engine went from 63.6% to 72.3%, and `match_pl.cpp` is at 99.3%.

**Redo:** on top of batch 146's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 147 2001-10-01`
