# Batch 176 — injury recovery, the person in the transfer list and the discipline penalty

**Routines:**
- `recuperar_lesao()` = 0x62bd10 (injury.cpp, thiscall): the recovery day (below −1000 in x87 with −0.0035 and 2.5; otherwise the pace or a draw);
- `pessoa_na_lista_de_transferencias()` = 0x8c1870 (transfer_manager.cpp, thiscall): is the person in list +0x92c?;
- `punicao_da_tabela()` = 0x55cae0 (discipline.cpp, thiscall): the penalty at the table row and column, if the type matches;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (6):** 5 caught. `t_item` escaped: the transfer manager list +0x92c never holds the person looked up in the automatic career (the routine always returns 0); goes to the broad career. (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (034ff3eb3e55dfd3).

**Redo**, on top of batch 175's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 176`
