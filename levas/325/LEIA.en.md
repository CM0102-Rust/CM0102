# Batch 325 — the AI pre-season tournaments and tours; building the tactics store

**Routines:**
- `plan_preseason_tournaments_and_tours()` = 0x5c3040 (friendly.cpp, thiscall, ret 0xc): In the pre-season of each selected league (season not started yet and within 52 weeks of the end): from 2002 on, one tournament per nation: among the AI clubs without a date list and outside the Intertoto, the highest draw of the reputation band hosts (and a draw of 10 must be nonzero), taking three friendly opponents (up to 50 tries each, without a date list), all against all in the opponents lists, in a cup or a mini league (half and half); defect: the loop counter is not initialised before the first tournament and the bound is the low byte of the nation reputation. Every year, the tours: each AI club without opponents, with a league, with at most 4 friendlies and passing a draw of 3, given a tour nation, meets its clubs until it has 3 or 4 friendlies (band = reputation / 500 + draw, × 0.8 after each opponent, 10 below 1) and, with more than one opponent, the tour is scheduled;
- `construct_tactics()` = 0x893dc0 (tactics.cpp, thiscall, ret 4): Checks the CD (the first CD drive holding cm0102.exe; without it, the "Insert CD" box and again); zeroes the 16 undo slots; with the save, the tactics come from it. In a new game: the 28 AI formations and the 17 preset ones come from the CM3_TACTICS files, in order (442_wide.pct twice), each into the next slot while there is room (a file that fails does not count; without room, the box of its line; formations missing at the end, the box at 0x92 or 0xb4), and the presets get their display name ("4-4-2", "3-5-2 Defensive"...); an empty selection row per club plus 32 (0x91 bytes, all −1 after the first 2); the per-tactic statistics (45 of 0x28 bytes, zeroed with the index) and 9 constants at +0xc..+0x2c;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (5):** 5 caught.
- **Real game comparing to 2002-01-01:** 0 divergences (5828e965).
- **Rust ×2 and original careers to 2002-06-01:** Rust ×2 ac155f01, original 96da1fba (recorded; does not block the batch since 31/10/2026). Garbage detector: equal.

**Redo**, on top of batch 324's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 325`
