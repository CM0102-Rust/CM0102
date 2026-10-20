# Batch 190 — the loan contract, the shortlist bid markers and the stand-in manager

**Routines:**
- `tirar_contrato_de_emprestimo()` = 0x4d0bd0 (contract_manager.cpp, thiscall): frees the person's loan contract slot, if any;
- `desmarcar_lances_pelo_jogador()` = 0x84aeb0 (shortlist_manager.cpp, thiscall): clears the bid marker of the club shortlist slots that target the player;
- `tecnico_substituto()` = 0x600e90 (human_manager.cpp, cdecl): the club's assistant or, without one, the highest-reputation staff member;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (3):** 2 caught. `s_empate` escaped: the stand-in manager is only asked for the human club, and its staff never tie on reputation. (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (034ff3eb3e55dfd3).

**Redo**, on top of batch 189's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 190`
