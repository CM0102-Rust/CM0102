# Batch 196 — the league head-to-head and the terms offered in the transfer

**Routines:**
- `ordem_do_confronto()` = 0x6834f0 (league.cpp, cdecl): the head-to-head mini table order: points and the league tie-breakers;
- `atualizar_confronto_direto()` = 0x6869a0 (league.cpp, thiscall): adds one match to the head-to-head record (played, result, goals and points);
- `marcar_termos_oferecidos()` = 0x8c1f80 (transfer_manager.cpp, thiscall): marks that the transfer offer had contract terms offered to the player;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (3):** 1 caught. `c_saldo` escaped: no career league breaks ties by head-to-head goal difference (the mini table does not decide). `c_empate` escaped: head-to-head draw points decide no standings in the career. (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (034ff3eb3e55dfd3).

**Redo**, on top of batch 195's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 196`
