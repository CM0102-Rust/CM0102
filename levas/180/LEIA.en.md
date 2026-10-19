# Batch 180 — the match screens (1): the control label, closing the screen and the waiting screen

**Routines:**
- `rotulo_do_controle()` = 0x725a70 (match_screens.cpp, cdecl): the match control button label (waiting, kick off, restart, pause or resume, continue) and whether it is disabled;
- `fechar_tela_da_partida()` = 0x71b120 (match_screens.cpp, cdecl): closes the match screen: destroys and frees screen manager parameters 0x2d and 0x1d;
- `abrir_tela_de_espera()` = 0x727f00 (match_screens.cpp, cdecl): opens the human manager's waiting screen in the screen manager;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (0):** 0 caught.
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (034ff3eb3e55dfd3).

**Redo**, on top of batch 179's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 180`
