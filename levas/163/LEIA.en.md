# Batch 163 — the formation (flags and cells) and resetting the club records

**Routines:**
- `casa_guardada()` = 0x5b3c70 (formation.cpp, thiscall): the stored cell of the slot by attack and band;
- `ligar_marca_da_formacao()` = 0x5b50d0 (formation.cpp, thiscall): sets the flag in +0x584 clearing the same group and zeroes +0x187f;
- `reiniciar_recordes()` = 0x452fe0 (club_records.cpp, cdecl): −1.0f and −1 in the club record fields;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (5):** 5 caught.
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (034ff3eb3e55dfd3).

**Redo**, on top of batch 162's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 163`
