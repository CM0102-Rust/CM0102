# 194ª leva — as listas de reserva do gerador de jogadores e o prêmio da liga

**Rotinas:**
- `tirar_reserva_de_tecnico()` = 0x7a82b0 (player_regen.cpp, thiscall): tira da lista de reserva o registro de não jogador de um id;
- `tirar_jogador_do_banco()` = 0x7a8470 (player_regen.cpp, thiscall): tira do banco de jogadores livres o registro dos dados de jogador;
- `premio_da_liga()` = 0x682f70 (league.cpp, thiscall): o prêmio total da liga (uma cota a mais por posição acima do último);

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (3):** 2 acusadas. `r_marca` escapou: o campo +8 da vaga que fica livre no fim da lista de reserva nunca é lido antes de ser regravado. (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (034ff3eb3e55dfd3).

**Refazer**, sobre o commit da 193ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 194`
