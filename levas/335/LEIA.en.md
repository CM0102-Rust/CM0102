# Batch 335 — the nation menu split in two parts

**Routines:**
- `nation_menu_split_construct()` = 0x66a280 (key_nation.cpp, thiscall, ret 0xc): builds the nation menu split: counts the key nation table nations chosen to be played (only those with bit 2, if asked) and, in the given order, the middle nation ((count + 1) / 2) gives the letter where the second part starts: the first letter of its short name or name;
- `nation_menu_split_format_header()` = 0x66a360 (key_nation.cpp, thiscall, ret 0xc): writes the header of one menu part: "Nations A-{letter}" for the first, "Nations {next letter}-Z" for the second; a text under 20 bytes: the "Error" box and nothing;
- `nation_menu_split_in_second_part()` = 0x66a430 (key_nation.cpp, thiscall, ret 4): tells whether the nation goes in the second part: the first letter of its short name (or name) after the split letter;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (3):** 3 caught.
- **Real game comparing to 2002-01-01:** 0 divergences (50d0e931).
- **Rust ×2 and original careers to 2002-06-01:** Rust ×2 73c66633, original 2520f435 (recorded; does not block the batch since 31/10/2026). Garbage detector: equal.

**Redo**, on top of batch 334's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 335`
