# 186ª leva — as telas da partida (4): abrir a tela da partida e as respostas dos painéis dos últimos placares e das estatísticas do time

**Rotinas:**
- `abrir_tela_da_partida()` = 0x718aa0 (match_screens.cpp, cdecl): abre a tela da partida ao vivo, grava os parâmetros dela e marca o humano como espectador;
- `responder_ultimos_placares_da_partida()` = 0x725860 (match_screens.cpp, cdecl): o painel dos últimos placares da tela da partida: a troca de página a cada 5 s e os cliques (competição, páginas, parar);
- `responder_estatisticas_do_time()` = 0x722dc0 (match_screens.cpp, cdecl): o painel das estatísticas do time: o redesenho com dados novos e os cliques que sobem e descem a linha;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (0):** 0 acusadas.
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (034ff3eb3e55dfd3).

**Refazer**, sobre o commit da 185ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 186`
