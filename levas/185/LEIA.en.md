# Batch 185 — the agreed-contract flag and the offer slot

**Routines:**
- `marcar_contrato_acertado()` = 0x8c5410 (transfer_manager.cpp, thiscall): sets the person's agreed-contract flag (0x40) in the transfer manager;
- `desmarcar_contrato_acertado()` = 0x8c5430 (transfer_manager.cpp, thiscall): clears the person's agreed-contract flag;
- `vaga_da_oferta()` = 0x8c61e0 (transfer_manager.cpp, thiscall): the address of an offer slot (out of range, the table start);

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (3):** 1 caught. `t_desmarca` escaped: when the agreed-contract flag is cleared, the person never also has flag 0x80. `t_vaga` escaped: the career never asks for the offer slot equal to the count (out of range). (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (034ff3eb3e55dfd3).

**Redo**, on top of batch 184's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 185`
