# Batch 164 — the league calendar

**Routines:**
- `girar_rodada()` = 0x68a030 (league.cpp, cdecl): the next position in the calendar rotation (special league, jumps of 3 by draw, wrapping at n);
- `registrar_jogo_da_rodada()` = 0x68a1c0 (league.cpp, cdecl): stores the round match (three bytes and a dword);
- `marcar_ano_no_calendario()` = 0x5af180 (fix_man.cpp, cdecl): marks the match year in both clubs' entries (year × 2 + bit, 2001 rule);

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (7):** 5 caught. `g_meio` escaped: the automatic career only plays English leagues, without the rotation's 0x80 flag; not even redone stronger (`g_meio_b`, `mutacoes/leva164b.py`) is it caught. `g_volta` escaped: wrapping at the end of the rotation does not happen in the career; not even redone stronger (`g_volta_b`) is it caught. (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (034ff3eb3e55dfd3).

**Redo**, on top of batch 163's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 164`
