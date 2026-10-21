# 197ª leva — o registro do técnico humano (hora das férias e escalação guardada) e o técnico da busca

**Rotinas:**
- `hora_das_ferias()` = 0x5fdc30 (human_manager.cpp, thiscall): a hora guardada quando o técnico humano saiu de férias;
- `escalacao_guardada()` = 0x5fed50 (human_manager.cpp, thiscall): o endereço da escalação guardada do técnico humano (16 pares);
- `tecnico_da_busca()` = 0x7ae570 (player_search.cpp, thiscall): grava na busca de jogadores o id do técnico humano;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (3):** 3 acusadas.
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (034ff3eb3e55dfd3).

**Refazer**, sobre o commit da 196ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 197`
