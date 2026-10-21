# 204ª leva — as telas da partida (10): os painéis das zonas de ação e do relatório da partida e a tela dos jogos do dia

**Rotinas:**
- `painel_das_zonas_de_acao()` = 0x71f500 (match_screens.cpp, cdecl): desenha as zonas de ação: o seletor de período, o campo dividido em três zonas com as barras das cores dos dois clubes, ou a mensagem de que não há zonas;
- `painel_do_relatorio_da_partida()` = 0x721000 (match_screens.cpp, cdecl): desenha o relatório da partida: os minutos e os textos dos lances (criando o relatório fora da partida), ou a mensagem de que não há relatório;
- `montar_jogos_do_dia()` = 0x717db0 (match_screens.cpp, cdecl): monta a tela dos jogos do dia: título, árvore das competições do dia, a tabela das rodadas e partidas e a rolagem até a partida do humano;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (0):** 0 acusadas.
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (034ff3eb3e55dfd3).

**Refazer**, sobre o commit da 203ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 204`
