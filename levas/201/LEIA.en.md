# Batch 201 — the match screens (7): the match Continue, opening team selection and the team selection response

**Routines:**
- `responder_continuar_da_partida()` = 0x725ca0 (match_screens.cpp, cdecl): the match Continue: the pending team selection command (online, the club's free slot or waiting for the others), the network matches and team selection;
- `abrir_escalacao_da_partida()` = 0x725f70 (match_screens.cpp, cdecl): opens the Team Selection screen for the human's next match and stores the club, the match and the home flag;
- `responder_escalacao_da_partida()` = 0x727c30 (match_screens.cpp, cdecl): the Team Selection buttons: appoints the captain (the best starter) when missing and stores the pending command; the rest go to the common handler;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (0):** 0 caught.
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (034ff3eb3e55dfd3).

**Redo**, on top of batch 200's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 201`
