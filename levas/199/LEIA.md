# 199ª leva — as estatísticas da partida: a estatística de liga do jogador, a entrada do jogo e o balde do registro

**Rotinas:**
- `estatistica_da_liga_do_jogador()` = 0x7c47b0 (player_stats.cpp, thiscall): as partidas (campo 1) ou os gols (campo 2) de liga do jogador no clube, como número de ponto flutuante;
- `entrada_do_jogo_bate()` = 0x729500 (match_screens.cpp, thiscall): diz se uma entrada das estatísticas tem os dois clubes, em qualquer ordem, e as datas dadas;
- `balde_do_registro()` = 0x729bd0 (match_screens.cpp, stdcall): o balde (0 a 3) em que o registro de estatística do jogo é guardado: tipo da competição, seleção e marcas do jogo;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (3):** 1 acusadas. `e_data` escapou: na carreira a segunda data da entrada do jogo nunca decide (quando a primeira bate, a segunda também bate). `b_tipo` escapou: o ramo sem a marca +0x40 do balde do registro não roda na carreira automática. (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (034ff3eb3e55dfd3).

**Refazer**, sobre o commit da 198ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 199`
