# Batch 174 — person in the manager list, the contract record and the daily wear

**Routines:**
- `pessoa_na_lista()` = 0x69fc60 (manager_manager.cpp, thiscall): is the person in the list (iterator 0x68d1b0, restored at the end)?;
- `ficha_de_contrato()` = 0x4cf6d0 (contract_manager.cpp, thiscall): the person's contract record (0x50 bytes);
- `desgaste_diario()` = 0x62c2f0 (injury.cpp, thiscall): the day's wear on word +0xc (age, draws of 25 and 13, goalkeeper cap);

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (7):** 7 caught.
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (034ff3eb3e55dfd3).

**Redo**, on top of batch 173's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 174`
