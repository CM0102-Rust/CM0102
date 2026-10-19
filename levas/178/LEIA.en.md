# Batch 178 — the game average, the contract value and the national team level

**Routines:**
- `media_do_jogo()` = 0x5d5a60 (game.cpp, thiscall): the f64 average +0x20/+0x28 or +0x10/+0x18 (zero divisor: 6.8);
- `valor_do_contrato()` = 0x4d0c20 (contract_manager.cpp, thiscall): dword +0xc of the person's contract record;
- `nivel_para_a_selecao()` = 0x76d810 (national_teams.cpp, stdcall): the player's level 1..3 for the national team by reputation and position ratings;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (5):** 4 caught. `m_padrao` escaped: the average divisor (+0x18/+0x28) is never zero when the career asks for the average: the default 6.8 never shows. (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (034ff3eb3e55dfd3).

**Redo**, on top of batch 177's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 178`
