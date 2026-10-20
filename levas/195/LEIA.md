# 195ª leva — a última nota do jogador e a vaga e o tipo da busca de jogadores

**Rotinas:**
- `ultima_nota_do_jogador()` = 0x7c1a60 (player_stats.cpp, thiscall): a nota da última partida do jogador (6,5 sem nota);
- `vaga_da_busca()` = 0x7ae640 (player_search.cpp, thiscall): grava a vaga (0 a 9) da busca de jogadores;
- `tipo_da_lista_da_busca()` = 0x7ae6e0 (player_search.cpp, thiscall): grava o tipo de lista (1 a 4) da busca de jogadores;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (3):** 3 acusadas.
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (034ff3eb3e55dfd3).

**Refazer**, sobre o commit da 194ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 195`
