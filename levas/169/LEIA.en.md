# Batch 169 — closing the engine (3): the commentary queue

**Routines:**
- `evento_na_frente()` = 0x6d5d00 (match_events.cpp, thiscall): puts an event at the front of the commentary queue (steps the 12-slot ring back, drops the repeat) and queues it via 0x6d6f70;
- `construir_fila_da_narracao()` = 0x6d5bf0 (match_events.cpp, thiscall): the commentary queue constructor: clears the queue, reads the events and, if asked, plays the crowd loop;
- `liberar_tabelas_de_eventos()` = 0x6d5b50 (match_events.cpp, cdecl): frees the loaded event tables and each record's texts;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (4):** 2 caught. `c_tempo` escaped: equivalent mutation: the initial +0x2db0 (125) is only compared with 20 and 101 and the match screens rewrite it. `l_fim` escaped: equivalent mutation: 0x6d9e50, which calls the freeing, rewrites 0xa1599c (the events language) right after. (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (034ff3eb3e55dfd3).

**Redo**, on top of batch 168's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 169`
