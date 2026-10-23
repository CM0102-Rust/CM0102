# Batch 217 — the end of the match statistics: the constructor (with the CD check) and the destructor

**Routines:**
- `montar_estatisticas_da_partida()` = 0x729000 (match_stats.cpp, thiscall): checks for the CM01/02 CD and builds the statistics: loads from the save or creates the index, the buckets and the zeroed temporary file;
- `desmontar_estatisticas_da_partida()` = 0x7293c0 (match_stats.cpp, thiscall): frees the record index and clears its pointer and count;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (0):** 0 caught.
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (b28542870027e432).

**Redo**, on top of batch 216's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 217`

**Fields touched:**
- Match_Stats +0: record index (n × 12 bytes); pointer; writes (montar_estatisticas_da_partida); reads, frees and writes 0 (desmontar_estatisticas_da_partida)
- Match_Stats +4: index capacity (n); value; writes (montar_estatisticas_da_partida); writes 0 (desmontar_estatisticas_da_partida)
- Match_Stats +8..+0x14: cursors of the four buckets (k × n / 4); value; writes (montar_estatisticas_da_partida)
- Match_Stats +0x18: unnamed (cleared after writing the temporary file); value; writes 0 (montar_estatisticas_da_partida)
- Match_Stats +0x1c: record count (n); value; writes (montar_estatisticas_da_partida)
- match_stats_matches.tmp (CM3_TEMP): n zeroed 0xd7a-byte statistics records; file; writes (montar_estatisticas_da_partida)
- [0x729089]/[0x7290cb] code bytes of the CD check (nop/xor in the working exe); value; reads (montar_estatisticas_da_partida)

**Of interest to:** SGE
