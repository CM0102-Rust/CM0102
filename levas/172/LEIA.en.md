# Batch 172 — closing the engine (6): reading the commentary, the player name and the perceived ability

**Routines:**
- `proximo_evento()` = 0x6d78b0 (match_events.cpp, thiscall): the next commentary event to show, part by part of the text (split by |), advancing the queue reading;
- `nome_do_jogador_na_narracao()` = 0x6db2d0 (match_events.cpp, cdecl): the player name in the commentary: the short form, or the long one when another player of either club has the same short name;
- `habilidade_vista()` = 0x6e86d0 (match_man.cpp, cdecl): a player's current or potential ability as an observer sees it (reputation, noise and the observer's knowledge);

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (6):** 3 caught. `p_parte` escaped: 0x6d78b0 only runs with the commentary on screen, which the automatic career does not show (goes to a match watched by Lucas). `p_anel` escaped: 0x6d78b0 only runs with the commentary on screen, which the automatic career does not show. `n_longa` escaped: 0x6db2d0 only runs with the commentary on screen, which the automatic career does not show. (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (034ff3eb3e55dfd3).

**Redo**, on top of batch 171's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 172`
