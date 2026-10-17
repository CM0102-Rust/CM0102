# Batch 155 — the match events reader (0x6d9e50)

**Routine:** `ler_eventos_cfg()` = 0x6d9e50 (match_events.cpp, ~5.2 KB): at startup, reads the `events_<language>.cfg` (or `events_eng.cfg`) into the table of the 677 event codes at [0xb68788]: each code's options, the sounds (weight, type, name) and the two commentary texts. Checks the texts' `<X` tokens and, with `-verifysounds`, the sound files.

**Alongside:** batch 154's read of the plays (0x6d9c40) got a real comparison, since it only reads a file. It proved that the `Area.cpp 507` error in an old match's report is not Rust's (0 divergences; it is the interface's 30-row limit at 1080, noted in the wiki for the screens stage).

**How it was compared:** no comparison of its own (it reads a file and allocates), through the career (Rust × original).

**Result** (`cadeia.txt`, `mutacoes_refeitas.txt`):
- Mutations (6 and 2 redone): 2 caught and 6 escaped. The sounds and options 0, 1 and 4 only serve the audio and the screen: gap in `docs/PENDENCIAS.en.md`. In Lucas's game, the commentary and sounds worked.
- Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3).
- Rust ×2 and original careers to 2002-06-01: all three equal (034ff3eb3e55dfd3).

**Redo:** on top of batch 154's commit (and the fix 601d37c):
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 155 -`
