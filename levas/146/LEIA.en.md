# Batch 146 — the pass execution (0x6fad40)

**Routine:** `executar_passe()` = 0x6fad40 (match_pl.cpp, 22 KB, the largest ported so far). It executes every pass, cross, long ball, corner, throw-in and free kick:
- pass score by type and distance;
- challenger, with foul (card, penalty, event 0x82) or dispossession;
- without a challenger:
  - stamina and inaccurate pass (the target drawn by +0x4756's factor);
  - cost in +0x35;
  - approach and pass-type commentary (over 150 codes);
  - the next set piece;
  - the trace (0x6ccb60), which ends in a shot (0x70c040/0x70a910) or the ball in transit.

**First batch read from the package** (`engenharia-reversa/pacote.py`). Measurement agreed with Lucas:
- **Real size.** The package, through the real boundaries, measured 22 KB. `mapa_funcoes.csv` cut the routine at 0x6fe364 (and `ROTINAS.md` said 13.8 KB). Without this, the reading would have stopped halfway.
- **Old notes.** It brought the one on the output pointer (`COMO_CONTINUAR`) and the recording port that existed for the routine.
- **Callers.** It showed that the trace 0x6ccb60 is only called by 0x6fad40, which allowed making it a tape unit with no effect elsewhere.
- **Constants.** The already resolved values (1.2/1.4/1.6/1.8 of the pass factor, 0.95…0.875 of the shoot-out, 2.0/3.0 of the distances) saved trips to `.rdata`.
- **What did not help in this batch:** the fields section (who else reads and writes each offset). The routine was read from the assembly, and the fields were clear in the code itself.

**Estimated gain:** about 15 fewer lookups in the reading and one large mistake avoided, the size. The batch's time is still dominated by reading the assembly (~6,900 instructions) and the test chain (~3 h).

**How it was compared:** through the tape, with comparison inside the unit, because it runs as a unit of the pass 0x6fa250 and of the long ball 0x6fa490.
- Challenger, foul and dispossession are in their own versions. They differ from batch 143 in the write order, the penalty draws and event 0x82, which here does not depend on the card.
- The commentary by type is in tables: four groups of seven codes, picked by the output and +0x1a7, and the pass direction picks within the group.

**Result** (`cadeia.txt`):
- Comparing to 2001-08-20: 0 divergences, with 12 of 17 branches exercised.
- Mutations (25): 24 flagged by the career and 1 by the confirmation.
- Real game comparing to 2002-01-01: 0 divergences.
- Rust ×2 and original careers to 2002-06-01: fingerprint 459815f5e0dca92b.

**Redo:** on top of batch 145's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 146 2001-10-01`
