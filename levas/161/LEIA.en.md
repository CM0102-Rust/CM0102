# Batch 161 — shortlist slot, position code and staff criterion

**Routines:**
- `serve_para_a_vaga()` = 0x844300 (shortlist_manager.cpp, stdcall): the player fits shortlist slot 0..15 (rating 15 in sector and side; slot 15 needs a contract, an id and 0x600a50 at zero); returns only `al`;
- `codigo_da_posicao()` = 0x83bd00 (shortlist_manager.cpp, stdcall): the position code from the sector and side mask (28 combinations; the rest 0xe); returns only `al`;
- `criterio_da_comissao()` = 0x866f60 (staff_contracts.cpp, thiscall): the staff criterion: an 8-bit score (id mod 7, ratings, age, reputation, number of roles) decides by the roles whether it returns 1;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (9):** 6 caught. `s_id` escaped: it only changed the exact id limit (−16 to −17); redone as `s_id_b` in `mutacoes/leva161b.py` and caught (`mutacoes_refeitas.txt`). `c_idade` escaped: not even redone with +15 (`c_idade_b`) is it caught: no staff member has +0x44 between 51 and 75 in the data. `c_rep` escaped: it only changed the exact reputation 0x1c53; redone as `c_rep_b` and caught. (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (034ff3eb3e55dfd3).

**Redo**, on top of batch 160's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 161`
