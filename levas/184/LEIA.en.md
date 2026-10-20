# Batch 184 — discipline: the new season, the day and the table slot

**Routines:**
- `disciplina_temporada_nova()` = 0x556210 (discipline.cpp, thiscall): starts the new season of the competition's discipline rules (virtual method 0);
- `disciplina_do_dia()` = 0x556240 (discipline.cpp, thiscall): the day of each of the 43 discipline rules (virtual method 1);
- `disciplina_gravar_vaga()` = 0x55cb20 (discipline.cpp, thiscall): writes the type and value of one of the person's slots in the discipline table;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (4):** 2 caught. `d_sem_regra` escaped: the competition with discipline rules index 0 does not start a season in the career (league outside the loaded ones). `d_43` escaped: the 43rd discipline rule is empty or changes nothing in the loaded leagues. (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (034ff3eb3e55dfd3).

**Redo**, on top of batch 183's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 184`
