# 206ª leva — as estatísticas da partida: ler e gravar o registro no arquivo temporário e registrar a partida

**Rotinas:**
- `ler_registro_da_partida()` = 0x729600 (match_stats.cpp, thiscall): lê um registro de estatística da partida (0xd7a bytes) do match_stats_matches.tmp pelo índice;
- `registrar_partida()` = 0x7293f0 (match_stats.cpp, thiscall): preenche o registro de uma partida que terminou e o grava, salvo os jogos excluídos;
- `gravar_registro_da_partida()` = 0x729820 (match_stats.cpp, thiscall): grava os eventos e o registro na vaga do jogo (ou na vaga do cursor do balde, que anda em círculo) e atualiza o índice;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (3):** 1 acusadas. `l_tamanho` escapou: ler o registro da partida (0x729600) só roda quando se abre o relatório de uma partida jogada; a carreira automática não abre telas. `r_excluido` escapou: na carreira o jogo nunca é o da competição [0x9cf684], então o "ou" decide na primeira metade e a marca +0x4c do registro nunca é lida. (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (034ff3eb3e55dfd3).

**Refazer**, sobre o commit da 205ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 206`
