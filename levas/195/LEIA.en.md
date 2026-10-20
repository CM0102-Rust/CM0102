# Batch 195 — the player's last rating and the player search slot and list type

**Routines:**
- `ultima_nota_do_jogador()` = 0x7c1a60 (player_stats.cpp, thiscall): the player's last match rating (6.5 with none);
- `vaga_da_busca()` = 0x7ae640 (player_search.cpp, thiscall): stores the player search slot (0 to 9);
- `tipo_da_lista_da_busca()` = 0x7ae6e0 (player_search.cpp, thiscall): stores the player search list type (1 to 4);

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (3):** 3 caught.
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (034ff3eb3e55dfd3).

**Redo**, on top of batch 194's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 195`
