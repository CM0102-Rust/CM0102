# Batch 177 — the contract bit, the competition flag and the club record in the league

**Routines:**
- `bits_do_registro()` = 0x7a7eb0 (player_regen.cpp, stdcall): bits 4..5 of byte +0xb of the person's 79-byte record;
- `competicao_marcada()` = 0x684790 (league.cpp, thiscall): does the competition have +0xbe and flag 2 of +0xd9?;
- `registro_da_liga()` = 0x6826d0 (league.cpp, thiscall): the record (59 bytes) of table +0xb1;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (3):** 2 caught. `m_marca` escaped: in the English leagues the competitions with +0xbe set never have bits 1 and 2 of +0xd9 different (missing data; goes to the broad career). (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (034ff3eb3e55dfd3).

**Redo**, on top of batch 176's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 177`
