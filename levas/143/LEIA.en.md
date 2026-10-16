# Batch 143 — the player move (0x6f09f0)

**Routine:** `mover_jogador()` = 0x6f09f0 (match_pl.cpp, 9 KB), the player's step to a grid cell with an action (dribble, pass, approach...). Without the ball, the tackle injury and the turn; with the ball, the attack, the approach commentary, the challenger (foul with event 0x82 or clean contest), possession; then the grid, the marked players, wear and the ball.

**How it was compared:** through the tape. Calls with effects outside the engine (commentary 0x6d6f70, 0x714d40, ball control, set piece, ball out, event 0x68d0d0 and the injury and foul calls) are units recorded from the original and replayed in the copy. Since 0x6f09f0 only runs as a unit of other tapes, the comparator records the unit and, inside it, runs its own comparison (nested tape, as in batch 142). Event 0x82 is built on the stack: 0x68d0d0 checks the contents, not the address.

**What went wrong along the way:**
- repeated name (`mover` already existed): it became `mover_jogador`;
- mask of the commentary's 5th argument: only the low byte is reliable (0xff);
- the area in the "loses it in the area" condition was the own one, not the attacked one;
- the first chain timed out on the Rust career: the Rust-mode dispatch sent 0x714d40 and 0x68d0d0 to the dividida's default call. It got its own dispatch (`despachar_movimento`) — new pitfall in `docs/COMO_CONTINUAR.md`.

**Result** (`cadeia.txt`): comparing to 2001-08-20, 0 divergences; mutations (22): 20 flagged and 2 gaps (`m_vaga`, the player with no grid slot; `m_adiantado`, the side with no furthest-forward player recorded — neither branch runs in the automatic career; in `docs/PENDENCIAS.md`); real game comparing to 2002-01-01, 0 divergences; Rust ×2 and original careers to 2002-06-01 with the reference fingerprint 459815f5e0dca92b.

**Redo:** on top of batch 142's commit, `python integrar.py` (uses `codigo.rs`), `cargo build --release`, then `bash engenharia-reversa/cadeia.sh 143 2001-10-01`.
