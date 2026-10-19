# Batch 175 — the tactics table formation, the player's competition weight and the news field

**Routines:**
- `registro_da_formacao()` = 0x896570 (tactics.cpp, thiscall): the formation record (0x18e3 bytes) in the two tables;
- `peso_da_competicao()` = 0x7c6930 (player_stats.cpp, stdcall): the competition weight by the club's two competitions (500/1000/300/100);
- `gravar_dado_da_noticia()` = 0x785790 (news.cpp, thiscall): stores news field 0..49;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (6):** 5 caught. `f_limite` escaped: the limit only changes for formations beyond the default ones asked with the human flag (a human manager's own formations), which the automatic career does not use. (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (034ff3eb3e55dfd3).

**Redo**, on top of batch 174's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 175`
