# Batch 158 — saves above 2 GB (the game container)

**Routines** (`si/code/zipdir.cpp`, the container of `.sav` and `.qst` files):
- `abrir_no_conteiner()` = 0x9326e0 (`zopen`): opens an item for reading or writing;
- `buscar_no_conteiner()` = 0x933080 (`zseek`): moves within the item;
- `posicao_no_conteiner()` = 0x933170 (`ztell`): the current position in the item.

**The cause of the bug:** all three use signed 32-bit `fseek`/`ftell`. Above 2 GB, when writing, `ftell` returns −1 and the index gets position 0xffffffff; when reading, the seek fails. The index stores unsigned 32-bit positions, which go up to 4 GB.

**Deliberate difference from the original:**
- unsigned positions, and 64-bit seek and tell through Windows (`SetFilePointer`, with the same care the CRT takes with the `FILE`);
- the format does not change;
- above 4 GB, the item is refused without damaging the file.

**How it was proven:**
- **Mutations (4):** all 4 escaped. The automatic career neither saves nor loads, so it is a gap in `docs/PENDENCIAS.en.md`.
- **`engenharia-reversa/save_grande.py`:** a quick start with a 2.2 GB filler makes a 2.38 GB `.qst`, with no clicks.
  - In the original, the 45 items beyond 2 GB get position 0xffffffff.
  - With batch 158, the index comes out right.
- **Lucas's game:**
  - a 2.55 GB save, with 187 items beyond 2 GB, was saved and loaded with the DLL;
  - with the original routines (`CM_ORIGINAL=1`), loading fails with `Friendly.cpp`, at the first item beyond 2 GB.
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (034ff3eb3e55dfd3).

**Redo**, on top of batch 157's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 158 -`
4. `python engenharia-reversa/save_grande.py --modo original`, which should come out with a wrong index
5. `python engenharia-reversa/save_grande.py --modo rust`, which should come out with the index OK
