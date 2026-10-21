# Batch 203 — the match screens (11): the match stats, action zones and player ratings values

**Routines:**
- `valores_das_estatisticas_da_partida()` = 0x71e750 (match_screens.cpp, cdecl): sends the match stats cells the values that changed since the match state copy (param 0x1d) and updates the copy;
- `valores_das_zonas_de_acao()` = 0x720320 (match_screens.cpp, cdecl): updates the action zones: markers, possession, ball in play time, the three zone bars and the possession bar in the club colours; updates the state copy;
- `valores_das_notas_dos_jogadores()` = 0x723350 (match_screens.cpp, cdecl): updates the player ratings rows: cards, injury/dismissal/substitution minute, rating, goals and name colour; updates the state copy;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (0):** 0 caught.
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (034ff3eb3e55dfd3).

**Redo**, on top of batch 202's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 203`
