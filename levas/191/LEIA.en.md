# Batch 191 — the agreed contract club, the national team appearance and the held ban

**Routines:**
- `clube_do_contrato_acertado()` = 0x8c5450 (transfer_manager.cpp, thiscall): the club of the person's agreed contract (clears the flag when the contract is gone);
- `registrar_jogo_pela_selecao()` = 0x8ce920 (transfer_manager.cpp, thiscall): records the player's national team match in the recent-matches bits;
- `punicao_guardada()` = 0x55cb60 (discipline.cpp, thiscall): the player's held ban in a slot, or none;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (3):** 1 caught. `c_clube` escaped: in the career the agreed contract club is never asked for with the flag set and the contract found. `p_vaga` escaped: no player has two held bans in different slots when the search runs. (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (034ff3eb3e55dfd3).

**Redo**, on top of batch 190's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 191`
