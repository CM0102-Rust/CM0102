# Batch 145 — the engine loop (0x6b98c0)

**Routine:** `laco_do_motor()` = 0x6b98c0 (match_eng.cpp, 4.8 KB), the loop that runs `n` match instants and calls almost all the rest of the engine: punishments, shoot-out, marked incident, restart, clock and periods, possession, lines, marking and each active player's decision.

**How it was validated:** it is an orchestrator and does no arithmetic of its own; it only orders calls. A tape of it would snapshot the whole engine (63 KB) at every player decision, dozens of times per instant. So it has no side-by-side comparator: compare mode runs the original, and the routines it calls stay compared by their own comparators. The Rust copy is validated by the career, which changes entirely with any mistake in the loop. The mutations run by career only (`cadeia.sh 145 -`), plus an extra career with seed 777 to 2003-06-01. Calls go out by address (`em0`..`em4`) and each goes through its own hook.

**What went wrong along the way:** three mutations from the first round were equivalent or weak:
- −5 for −6 in a mark only tested as negative;
- the clock at kick-off is never 2;
- 19 versus 20 draw attempts.

They were redone in `leva145b.py` (output in `refeitas.txt`). The redone `l_limpeza` also escaped: the branch does not run, it is a gap.

**Result:**
- **Mutations** (18, output in `cadeia.txt` and `refeitas.txt`): 16 flagged and 2 gaps, in `docs/PENDENCIAS.md`:
  - `l_limpeza`: the on-field record with no engine;
  - `l_direcao`: the set-piece taker with no ball owner.
- **Real game comparing to 2002-01-01:** 0 divergences.
- **Careers to 2002-06-01** (Rust ×2 and original): fingerprint 459815f5e0dca92b.
- **Seed 777 to 2003-06-01** (Rust and original, `semente777.txt`): fingerprint 17dcb32ab70df0bd.

**Redo:** on top of batch 144's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 145 -`
