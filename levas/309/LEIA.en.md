# Batch 309 — loading the saved game's temporary files: the CONMEBOL and UEFA seedings and the stadium bookings

**Routines:**
- `conmebol_seeding_load_from_temp_file()` = 0x4c6970 (conmebol_seeding.cpp, thiscall, ret 0): reads CM3_TEMP\conmebol_seeding.tmp: the country count (+0x6e), 0xa bytes per country from +0 (the nation pointer +4 rebuilt from the id), +0x6f (u16) and, from data version 0x3e010 on, +0x71 (u32; before, −1); without the file (a new game), 0; on error the "Error" box without closing the file;
- `uefa_seeding_load_from_temp_file()` = 0x905230 (uefa_seeding.cpp, thiscall, ret 0): reads CM3_TEMP\uefa_seeding.tmp: the country count (+0x438), 0x12 bytes per country from +0 (the nation pointer +4 rebuilt from the id), +0x439 (u16), as many u32 as +0x43f into a new list at +0x43b and the 50 records of 0xc from +0x440; without the file (a new game), 0; on error the "Error" box without closing the file;
- `stadium_load_from_temp_file()` = 0x85ba40 (stadium.cpp, thiscall, ret 0): reads CM3_TEMP\Stadium.tmp: the bookings (capacity +8, count +4, 12 bytes each, in a new array at +0), the venues (capacity +0x14, count +0x10, the flag +0x18; 16 bytes each in a new array at +0xc, the stadium +0xc rebuilt from its id, −1 null) and, from data version 0x3e029 on, the stashed venue [0xa835a4]; without the file (a new game), 0; on error the "Error" box;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Save proof** (the `.qst` saved in Rust mode with and without the batch's routines, outside the bytes that vary on their own, and the holiday save loaded to 2001-08-28 with and without them, same fingerprint; the mutations also went through it): EQUAL fora dos campos ignorados; carga EQUAL (parou em 2001-239, 732460546 sorteios, impressao 3540600cc39540fa).
- **Mutations (3):** 3 caught.
- **Real game comparing to 2002-01-01:** 0 divergences (36dd02c3).
- **Rust ×2 and original careers to 2002-06-01:** Rust ×2 e9db46f8, original 60d7b94b (recorded; does not block the batch since 31/10/2026). Garbage detector: equal.

**Redo**, on top of batch 308's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 309`

**Fields touched:**
- ConmebolSeeding +0 (+4 the nation pointer rebuilt), +0x6e, +0x6f, +0x71; UefaSeeding +0 (+4), +0x438, +0x439, +0x43b, +0x43f, +0x440; Stadium +0, +4, +8, +0xc, +0x10, +0x14, +0x18; reads from the .tmp and writes

**Of interest to:** SGE
