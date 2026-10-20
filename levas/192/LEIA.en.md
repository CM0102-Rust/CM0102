# Batch 192 — the retirement entry, the friendly competitions and the news slot

**Routines:**
- `entrada_da_aposentadoria()` = 0x7a7ed0 (player_regen.cpp, thiscall): the person's entry in the retirement list (by injury or already announced);
- `registrar_competicoes_do_amistoso()` = 0x5ca030 (friendly.cpp, thiscall): stores the friendly and tournament objects in the competition table;
- `vaga_da_noticia()` = 0x785830 (news.cpp, thiscall): the value of one of the 50 news item slots;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (3):** 2 caught. `a_torneio` escaped: friendly tournament slot 0 is never taken in the career. (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (034ff3eb3e55dfd3).

**Redo**, on top of batch 191's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 192`
