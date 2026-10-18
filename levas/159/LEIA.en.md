# Batch 159 — the formation: player positions and pitch cells

First batch outside the match engine. The candidates come from the automatic career's coverage of 26 simulation modules (`docs/cobertura_simulacao.txt`, `python engenharia-reversa/candidatos.py --simulacao`).

**Routines** (`formation.cpp`):
- `mascara_de_posicoes()` = 0x5b7d20: the player's strong-position mask;
- `casa_no_campo()` = 0x5b7e60: the column and row of the position on the pitch;
- `vagas_sem_conflito()` = 0x5b8960: checks the formation's 11 slots.

**How it was compared:** real comparison on all three (`comparar_valor!` and `lado_a_lado_leve` on the output).

**Result** (`cadeia.txt`):
- **Mutations (7):** 5 caught. `m_nivel` and `v_a20` escaped: in the database the natural position is rated 20 and no slot with an open side is a midfielder's. It is a gap in `docs/PENDENCIAS.en.md`.
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (034ff3eb3e55dfd3).

**Redo**, on top of the commit of batch 158 and the save DLL:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 159`
