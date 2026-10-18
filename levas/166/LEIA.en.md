# Batch 166 — the human's scout, the human at the club and the contract category

**Routines:**
- `registro_do_observador()` = 0x7f6cd0 (scout_manager.cpp, thiscall): the human manager's scouting record 0..9 (0x2b0 bytes);
- `humano_manda_no_clube()` = 0x600c90 (human_manager.cpp, cdecl): does the current human manager run the club (or the nation's main team)?;
- `categoria_do_contrato()` = 0x4ca470 (contract_manager.cpp, stdcall): the player's category 1..8 in the contract (status, happiness, age, ambition, reputation);

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (7):** 3 caught. `h_forcar` escaped: counter "humano manda no clube" stays at zero: with no human manager the routine never returns 1 in the career. `c_k0` escaped: exact limit; not even redone stronger (`c_k0_b`, `mutacoes/leva166b.py`) is it caught: the status-0 range with happiness 40 to 75 does not appear. `c_k5` escaped: exact limit; redone as `c_k5_b` and caught. `c_tipo5` escaped: type-5 contracts do not appear in the career; not even redone stronger (`c_tipo5_b`) is it caught. (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (034ff3eb3e55dfd3).

**Redo**, on top of batch 165's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 166`
