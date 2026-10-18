# 168ª leva — o fechamento do motor (2): os zagueiros do lado, o árbitro e o jogador da partida

**Rotinas:**
- `zagueiros_do_lado_sem_conflito()` = 0x6d5b20 (match_eng.cpp, thiscall): a 0x5b8dc0 na formação do lado;
- `construir_arbitro()` = 0x6e9460 (match_official.cpp, thiscall): o construtor do árbitro (zera +0 e +4);
- `destruir_jogador_da_partida()` = 0x6ebd60 (match_pl.cpp, thiscall): o destrutor do jogador da partida (zera +0x19e);

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (3):** 3 acusadas.
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (034ff3eb3e55dfd3).

**Refazer**, sobre o commit da 167ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 168`
