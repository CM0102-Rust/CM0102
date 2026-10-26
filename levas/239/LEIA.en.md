# Batch 239 — game creation: initialising the players and the staff

**Routines:**
- `inicializar_jogadores()` = 0x535a80 (Database.cpp, cdecl): prepares every person of the new game: missing names, dates, staff attributes, nation; for the player the abilities, potential, current ability, age, positions and missing attributes (from the most similar of 250 random players); for the non-player the reputations, attributes (template among 250), job and the famous names; then the ability-relative vectors and the normalisations; "Player Initialisation" bar;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Save proof** (the `.qst` saved in Rust mode with and without the batch's routines, outside the bytes that vary on their own; the mutations also went through it): EQUAL (16856054 bytes).
- **Mutations (5):** 5 caught.
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (b28542870027e432).

**Redo**, on top of batch 238's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 239`

**Fields touched:**
- TStaff +4/+8/+0xc names, +0x10 birth date, +0x18 age, +0x1a nation, +0x31/+0x3e/+0x46 dates, +0x39 club, +0x3d job, +0x56..+0x5d attributes; writes (inicializar_jogadores)
- TPlayer +5 current, +7 potential, +9/+0xb/+0xd abilities (×50), +0x45, positions, sides and attributes; writes (inicializar_jogadores)
- TNonPlayer +4/+6 abilities, +8/+0xa/+0xc reputations (×50), attributes, +0x23..+0x42 cleared, +0x43; writes (inicializar_jogadores)
- 79-byte per-person table [0xae2d0c]: +9 = 0x9c (player), +0xb &= 0xf0; writes (inicializar_jogadores)
- template vectors 0xae2044/0xae2114 (flag 0x9b59b8) and 0xae18bc/0xae2238 (0x9b59b9); writes (inicializar_jogadores)

**Of interest to:** SGE, CMRTE
