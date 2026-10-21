# 205ª leva — as telas da partida (12): as linhas das estatísticas do time, o redesenho dos últimos placares e a espera dos outros jogadores

**Rotinas:**
- `preencher_estatisticas_do_time()` = 0x721cd0 (match_screens.cpp, cdecl): escreve as 20 linhas de jogador das estatísticas do time (22 colunas pela tabela do parâmetro 0x1c), montando as células ou atualizando o que mudou;
- `atualizar_ultimos_placares()` = 0x724520 (match_screens.cpp, cdecl): reescreve as linhas dos últimos placares em páginas: fase, jogo, placar, período, autores dos eventos, agregado e o rodapé da página;
- `esperar_os_outros_jogadores()` = 0x727fb0 (match_screens.cpp, cdecl): no jogo em rede, espera os outros humanos da mesma partida ou fecha as telas, avisa e abre a tela da partida;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (0):** 0 acusadas.
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (034ff3eb3e55dfd3).

**Refazer**, sobre o commit da 204ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 205`
