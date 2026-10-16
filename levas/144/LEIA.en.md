# Batch 144 — the player on the ball (0x7109f0)

**Routine:** `conduzir()` = 0x7109f0 (match_pl.cpp, 13 KB), the decision of whoever has the ball. On set pieces, corners, throw-ins and free kicks pass to the chosen team-mate (0x6c6a30) or to a drawn cell; a penalty or a free kick near goal is shot. In play it adds up the shot, pass and dribble scores: distance to goal, pressure (0x6bcbd0, which fills a list on the stack), score, keeper, the position's marks and the clock. Then it plays a long ball (0x6fa490), goes for a dividida (0x70ec80), shoots (0x702050), passes (0x6fa250) or dribbles (0x6f09f0). In mode 4 it is the keeper in the shoot-out, and it returns 1 when he stays.

**How it was read:** the structure came from Ghidra's pseudocode and all the arithmetic from the assembly. There are 52 `ftol` calls and 59 draws, and the pseudocode hides the floating-point expressions. The operand order of `fsubp` came from the bytes (DE E9: st1 − st0). Two score swaps (+0x10 and +0x14) were also corrected from the assembly, where a `push` comes before the stack access.

**How it was compared:** through the tape, with comparison inside the unit (it runs as a unit of the shoot-out and of the player decision). The pass, the long ball and the dividida run inside the copy, with their own masks, and their units go into the dribble's tape. The move, the commentary and the shot (0x702050, still original) enter as units. The pressure list lives on the stack, at a different address in the original and in the copy, so the copy into the list (0x68d0d0) enters without the address, through `lista_gravada`. The list head passed to the shot is ignored.

**Determinized original:** in mode 4, the pass score (read at 0x712ec4) and the team-mate (read at 0x713107) are never written. Also, the high part of the byte arguments to the pass and the shot comes with no value. So the routine joined `ZERADAS`. The original career with the zeroing kept the reference fingerprint.

**What went wrong along the way:**
- `na_propria_area` already existed and the new function became `na_area_defendida`;
- `lista_gravada` depended on `gravando()`, which is false in the copy's phase, and now depends only on the flag;
- the first compare game used the old DLL, because `simular.py` does not copy the built one. It became a pitfall in `docs/COMO_CONTINUAR.md`.

**Result** (`cadeia.txt`):
- Comparing to 2001-08-20: 0 divergences, going through 30 of the dribble's 32 branches.
- Mutations (24): 23 flagged and 1 gap (`c_goleiro`: the position-1 guard without the goalkeeper mark; in `docs/PENDENCIAS.md`).
- Real game comparing to 2002-01-01: 0 divergences.
- Rust ×2 and original careers to 2002-06-01: fingerprint 459815f5e0dca92b.

**Redo:** on top of batch 143's commit, run `python integrar.py` (uses `codigo.rs`) and `cargo build --release`, copy the DLL to `jogo_testes/` and run `bash engenharia-reversa/cadeia.sh 144 2001-10-01`.
