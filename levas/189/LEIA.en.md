# Batch 189 — the friendly competition, the international date and the manager record

**Routines:**
- `e_competicao_do_amistoso()` = 0x5c2990 (friendly.cpp, thiscall): tells whether the competition is the friendlies one;
- `preencher_data_internacional()` = 0x5cb9b0 (friendly.cpp, thiscall): fills an international date: day, month, span, year, continent and competition;
- `limpar_registro_de_tecnico()` = 0x86fc20 (staff_records.cpp, thiscall): clears the manager record and sets the owner, league and club ids to −1;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (3):** 2 caught. `r_ids` escaped: the last spell id (+0x72) of a cleared manager record is never read in the career before being rewritten. (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (034ff3eb3e55dfd3).

**Redo**, on top of batch 188's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 189`
