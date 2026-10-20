# Batch 193 — the contract clauses: next sale percentage, appearance bonus and national team bonus

**Routines:**
- `tirar_percentual_da_proxima_venda()` = 0x8cb4f0 (transfer_manager.cpp, thiscall): removes the player's next sale percentage (flag and list record);
- `tirar_bonus_por_jogo()` = 0x8cb590 (transfer_manager.cpp, thiscall): removes the first-team appearance bonus (flag and list record);
- `tirar_bonus_pela_selecao()` = 0x8cb630 (transfer_manager.cpp, thiscall): removes the international appearance bonus (flag and list record);

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (3):** 0 caught. `l_venda` escaped: no contract clause (next sale percentage, appearance bonus, international bonus) is removed in the career up to 2002-06. `l_jogo` escaped: no contract clause (next sale percentage, appearance bonus, international bonus) is removed in the career up to 2002-06. `l_conta` escaped: no contract clause (next sale percentage, appearance bonus, international bonus) is removed in the career up to 2002-06. (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (034ff3eb3e55dfd3).

**Redo**, on top of batch 192's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 193`
