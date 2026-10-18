# Batch 165 — extra tactic slots, national team ages and the human manager field

**Routines:**
- `vaga_extra_tatica()` = 0x897ee0 (tactics.cpp, thiscall): the two extra slots (+0x56, +0x5a) of the club row;
- `idades_da_selecao()` = 0x76d430 (national_teams.cpp, stdcall): the age limits (23/23 in youth competitions, 24/22, 26/22);
- `gravar_dado_do_humano()` = 0x5fdd30 (human_manager.cpp, thiscall): stores +0x9a65 of the human manager record (0xa6ef bytes);

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (5):** 4 caught. `v_desce` escaped: when the first extra slot is cleared, the second is already empty (−1) in AI clubs: moving −1 down or writing −1 is the same (the counters "vaga extra apagada" 65536 and "vaga extra gravada" 4096 show both paths run). (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (034ff3eb3e55dfd3).

**Redo**, on top of batch 164's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 165`
