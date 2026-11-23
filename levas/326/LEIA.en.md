# Batch 326 — the European Championship qualifying draw

**Routines:**
- `euro_champ_qual_draw_groups()` = 0x57dab0 (euro_champ_qual.cpp, thiscall, ret 0): The groups draw of the European Championship qualifiers: in 1998, the fixed table (49 national teams in 9 groups, groups 1, 3, 6 and 8 with six); in other years, 9 groups (+0xff + 2) and up to six pots: the group order is shuffled by 20 drawn swaps and each group in turn takes the next national team while it has room; defect: each group room is that of the previous group (g − 1), so in format 1 the groups of six are 1, 3 and 6;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (2):** 2 caught.
- **Real game comparing to 2002-01-01:** deferred to the audit's accumulated comparison (decision of 5 Nov 2026).
- **Rust ×2 and original careers to 2002-06-01:** Rust ×2 5e6914ad, original 2c08aef5 (recorded; does not block the batch since 31/10/2026). Garbage detector: equal.

**Redo**, on top of batch 325's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 326`
