# Batch 149 — each side's selection and the match day (0x6db500, 0x6b4190, 0x6b42a0)

**Routines:**
- `escalar_lado()` = 0x6db500 (match_man.cpp): sets up one side's tactics and selects the team. At the human's club it uses the stored lines and the automatic selection (0x6dbc50) or the regulars; at the others, the automatic one.
- `encerrar_dia()` = 0x6b4190 (match_day.cpp): frees what the match day allocated, including the engines (0x6b97a0, from batch 148).
- `escolher_partida_do_dia()` = 0x6b42a0 (match_day.cpp): picks the human's match for the screen.

**Why 0x6dbc50 (the automatic selection) was left out:**
- It is 7.7 KB and calls 24 routines still in the original (dates, competitions, tactics screens).
- It allocates its own list and reads stack garbage (it is in `ZERADAS`).
- It is left for a batch of its own.

**How it was compared:** with no comparison of its own, through the career (Rust × original), like batches 145 and 148. The routines they call stay compared.

**Result** (`cadeia.txt`):
- Mutations (8): 7 flagged and 1 gap, the second field of the manager's record (in `docs/PENDENCIAS.en.md`).
- Real game comparing to 2002-01-01: 0 divergences.
- Rust ×2 and original careers to 2002-06-01: fingerprint 459815f5e0dca92b.

**Redo:** on top of batch 148's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 149 -`
