# Batch 194 — the player generator reserve lists and the league prize fund

**Routines:**
- `tirar_reserva_de_tecnico()` = 0x7a82b0 (player_regen.cpp, thiscall): removes a non-player record from the spare list by id;
- `tirar_jogador_do_banco()` = 0x7a8470 (player_regen.cpp, thiscall): removes the player data record from the free player pool;
- `premio_da_liga()` = 0x682f70 (league.cpp, thiscall): the league total prize fund (one more share per place above the last);

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (3):** 2 caught. `r_marca` escaped: field +8 of the slot freed at the end of the reserve list is never read before being rewritten. (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (034ff3eb3e55dfd3).

**Redo**, on top of batch 193's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 194`
