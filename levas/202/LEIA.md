# 202ª leva — as telas da partida (8): os painéis das estatísticas do time, das notas dos jogadores e das estatísticas da partida

**Rotinas:**
- `painel_das_estatisticas_do_time()` = 0x7217a0 (match_screens.cpp, cdecl): desenha a tabela das estatísticas de um time (cabeçalho da página, setas, títulos das colunas) ou a mensagem de que não há estatísticas;
- `painel_das_notas_dos_jogadores()` = 0x722ed0 (match_screens.cpp, cdecl): desenha as notas dos jogadores dos dois times: camisa, nome (botão, com o capitão) e nota;
- `painel_das_estatisticas_da_partida()` = 0x71e040 (match_screens.cpp, cdecl): desenha as estatísticas da partida: 13 linhas rotuladas (chutes, escanteios, faltas, passes, desarmes, cabeçadas, cartões);

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (0):** 0 acusadas.
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (034ff3eb3e55dfd3).

**Refazer**, sobre o commit da 201ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 202`
