# Batch 171 — closing the engine (5): the combined commentary, the context commentary and the match player

**Routines:**
- `combinar_com_o_anterior()` = 0x6d60b0 (match_events.cpp, thiscall): combines the new commentary event with the last queued one (returns the combined event and copies the previous player, side and value);
- `comentario_de_contexto()` = 0x6d4b70 (match_eng.cpp, thiscall): decides whether a context commentary (league race, club news) may play now;
- `construir_jogador_da_partida()` = 0x6eada0 (match_pl.cpp, thiscall): the match player constructor (clears the fields, with initial values for some);

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (7):** 7 caught.
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (034ff3eb3e55dfd3).

**Redo**, on top of batch 170's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 171`
