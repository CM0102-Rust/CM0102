# Batch 179 — the contract offers and the closed transfer offer

**Routines:**
- `oferta_de_contrato()` = 0x4cc9e0 (contract_manager.cpp, thiscall): the person's contract offer for the club, in the contract manager's list;
- `oferta_acertada()` = 0x4cca30 (contract_manager.cpp, thiscall): the person's agreed contract offer (state 2);
- `oferta_fechada()` = 0x8d0d90 (transfer_offer.cpp, stdcall): whether the transfer offer is closed (by its state and, in state 0xf, by its phase);

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (4):** 2 caught. `f_estado` escaped: in the career the transfer offers asked about are never in state 4. `f_fase` escaped: no offer in state 0xf is in phase 2 when it is asked about. (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (034ff3eb3e55dfd3).

**Redo**, on top of batch 178's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 179`
