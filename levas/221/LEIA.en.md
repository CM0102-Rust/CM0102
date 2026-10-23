# Batch 221 — the saved game: loading the whole game

**Routines:**
- `carregar_jogo()` = 0x82a9c0 (Setup.cpp, cdecl): loads the saved game: disk, memory, CD, the database (0x524d10), unpacks the *.tmp files, general.dat (version, summary, ids, dates, leagues, default nation, the runtime data of people, clubs and reputation, converting old saves), the subsystem constructors in the original order with disk and memory checks between them, the FIFA rules and the last save date;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Save proof** (the `.qst` saved in Rust mode with and without the batch's routines, outside the bytes that vary on their own, and the holiday save loaded to 2001-08-28 with and without them, same fingerprint; the mutations also went through it): EQUAL fora dos campos ignorados; carga EQUAL (parou em 2001-239, 728264576 sorteios, impressao 7a1167744a713c73).
- **Mutations (3):** 3 caught.
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (b28542870027e432).

**Redo**, on top of batch 220's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 221`

**Fields touched:**
- [0xdd7110] save version; value; writes (read from general.dat) and reads (carregar_jogo)
- general.dat: version, summary (0xf70/0xf74; +0xf6c play time → [0xdd7140]), [0xdd7120] unique id, [0xdd7114] database size, [0xdd6ec8] date, the game date (local → [0xae2c90] via 0x417c10), [0xae2c88] time, [0xae2c78] last date, [0xae2cfc] last time, [0xae2d00] and [0xae2d04] leagues, default nation (id → [0xae2d08] = [0xae23a8] + 0x122 × id, or 0), [0xae2d10]; bytes; reads from the save and writes (carregar_jogo)
- [0xae2d0c] people runtime data (0x4f × [0xae2d10]; old 0x26/0x27-byte saves converted: +0..+0xd, +0xe/+0x12 × 5 and +0x27..+0x37 × 2 = −1); pointer and bytes; allocates and writes (carregar_jogo)
- [0xae2d14] club runtime data (6 × [0xae2d18] = [0xae235c]), [0xae2d1c] reputation (4 × [0xae235c]); pointer and bytes; allocates and writes (carregar_jogo)
- the subsystem globals (PASSOS_DA_CARGA, from [0xde1d08] Tactics to [0xb74430] PunditList; FOG_OF_WAR [0xae2a60] is checked through [0xb6b7c8], the Media one); pointer; writes (carregar_jogo)
- person (0x6e) +0x5f: 5 in the last 16 becomes 0 (manager undone: names +4/+8/+0xc emptied, +0x10 date, +0x18 year 1966, +0x24 and +0x39 = 0); value; reads and writes (carregar_jogo)
- [0xdd7134] out of disk, [0xdd7138] out of memory, [0xdd711c] and [0xdd7130] game loaded, [0xa801bc] changes = 0, 0xdd70a0 last save date; value; writes (carregar_jogo)
- [0x9b5980] FIFA rules feature; [0xae233c] in force, [0xa83608] max contract length (10 or 5), [0xae2330] change date; value; reads and writes (carregar_jogo)

**Of interest to:** SGE, CMRTE
