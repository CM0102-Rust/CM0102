# Batch 181 — the player's media stories and the cup match importance

**Routines:**
- `noticia_com_resposta_2()` = 0x735780 (media.cpp, thiscall): whether the player has a story with response 2 in the media log;
- `tem_noticia()` = 0x735840 (media.cpp, thiscall): whether the player has any story in the media log;
- `importancia_da_partida()` = 0x5221f0 (Cup.cpp, thiscall): the cup match importance by round and the first-leg score difference;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (4):** 2 caught. `m_alguma` escaped: the media log stories asked about in the career never have response 0. `c_fase` escaped: equivalent mutation: at round 1, replacing it with 1 changes nothing (not a test gap). (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (034ff3eb3e55dfd3).

**Redo**, on top of batch 180's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 181`
