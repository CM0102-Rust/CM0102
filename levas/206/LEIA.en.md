# Batch 206 — the match statistics: reading and writing the record in the temporary file and recording the match

**Routines:**
- `ler_registro_da_partida()` = 0x729600 (match_stats.cpp, thiscall): reads one match statistics record (0xd7a bytes) from match_stats_matches.tmp by index;
- `registrar_partida()` = 0x7293f0 (match_stats.cpp, thiscall): fills the record of a finished match and writes it, except for excluded matches;
- `gravar_registro_da_partida()` = 0x729820 (match_stats.cpp, thiscall): writes the events and the record into the fixture's slot (or the bucket cursor's slot, which wraps around) and updates the index;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (3):** 1 caught. `l_tamanho` escaped: reading the match record (0x729600) only runs when the report of a played match is opened; the automatic career opens no screens. `r_excluido` escaped: in the career the fixture is never the [0x9cf684] competition one, so the "or" is decided by its first half and the record +0x4c flag is never read. (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (034ff3eb3e55dfd3).

**Redo**, on top of batch 205's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 206`
