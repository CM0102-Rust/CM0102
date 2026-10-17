# Batch 157 — the commentary on the panel (0x6d7b00)

**Routine:** `narrar_no_painel()` = 0x6d7b00 (match_events.cpp, ~4.4 KB): during the watched match, writes the play on the commentary panel with the clubs' colours, wrapping or cutting the line by the width, and plays the play's and the crowd's sound.

**Milestone:** with it, **the match engine without the screens is 100% in Rust** (321 of 321 KB). What is left of the engine are the screens (`match_screens.cpp`), a stage of its own.

**How it was compared:** no comparison of its own (it draws and plays sound), through the career (Rust × original).

**Result** (`cadeia.txt`):
- Mutations (2): both escaped; the routine only runs with the match screen (counter at zero), gap in `docs/PENDENCIAS.en.md`. The test is watching matches in `JOGAR.bat`.
- Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3).
- Rust ×2 and original careers to 2002-06-01: all three equal (034ff3eb3e55dfd3).

**Redo:** on top of batch 156's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 157 -`
