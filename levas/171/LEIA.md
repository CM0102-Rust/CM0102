# 171ª leva — o fechamento do motor (5): a narração combinada, o comentário de contexto e o jogador da partida

**Rotinas:**
- `combinar_com_o_anterior()` = 0x6d60b0 (match_events.cpp, thiscall): combina o evento novo da narração com o último da fila (devolve o evento combinado e copia jogador, lado e valor do anterior);
- `comentario_de_contexto()` = 0x6d4b70 (match_eng.cpp, thiscall): decide se um comentário de contexto (corrida da liga, notícias do clube) pode tocar agora;
- `construir_jogador_da_partida()` = 0x6eada0 (match_pl.cpp, thiscall): o construtor do jogador da partida (zera os campos, com os valores iniciais de alguns);

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (7):** 7 acusadas.
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (034ff3eb3e55dfd3).

**Refazer**, sobre o commit da 170ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 171`
