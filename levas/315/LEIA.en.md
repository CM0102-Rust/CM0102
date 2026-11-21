# Batch 315 — the turnovers of the Inter-American Cup and the Oceania Club Championship and the alternating Oceania host

**Routines:**
- `inter_amer_cup_start_new_season()` = 0x632910 (inter_amer_cup.cpp, thiscall, ret 0): the Inter-American Cup's turnover: the cup turnover template (tables freed, season archived, year + 1, the clubs, the round table and the tree with the boxes at lines 0x164, 0x16b and 0x172, and the predicted positions);
- `oceania_club_champ_start_new_season()` = 0x797d60 (oceania_club_champ.cpp, thiscall, ret 0): the Oceania Club Championship is every 2 years: at the turnover the table and the round dates freed, the season archived, the stages destroyed, the year + 2, the current stage −1, the draw still to do, the host alternated between Australia and New Zealand and the rules again;
- `alternate_oceania_club_champ_host()` = 0x5face0 (host_country.cpp, thiscall, ret 0): the Oceania Club Championship host alternates every edition: in each of its entries in the host table, Australia becomes New Zealand and any other host becomes Australia;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (1):** 1 caught.
- **Real game comparing to 2002-01-01:** 0 divergences (798998b1).
- **Rust ×2 and original careers to 2002-06-01:** Rust ×2 9f530e7a, original bd0bb86f (recorded; does not block the batch since 31/10/2026). Garbage detector: equal.

**Redo**, on top of batch 314's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 315`
