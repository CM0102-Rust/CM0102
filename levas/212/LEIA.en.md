# Batch 212 — the match record, the order of the events and the language a player loses

**Routines:**
- `preencher_registro_da_partida()` = 0x729ff0 (match_screens.cpp, thiscall): fills the match statistics record: clubs, dates, attendance, gate receipts, weather, sorted events, formation, captain and the 20 players of each side;
- `ordem_dos_eventos()` = 0x72b160 (match_screens.cpp, cdecl): orders two match events by type group, then by minute and arrival order (qsort comparator);
- `tirar_idioma_do_jogador()` = 0x7c6bf0 (player_stats.cpp, thiscall): removes a language from the player's language table (binary search) and shifts the later entries down;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (3):** 2 caught. `i_ultima` escaped: in the career 0x7c6bf0 is only called by the regens (languages 1..64 of the freshly created player, who has none yet): the search never finds the entry, so the last slot is never cleared (the comparator covers the whole table). (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (5afd987e1a1a3ef3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (b11bd5cb63f51d72).

**Redo**, on top of batch 211's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 212`
