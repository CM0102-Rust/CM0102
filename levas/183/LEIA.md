# 183ª leva — as telas da partida (3): a tela de detalhes da partida

**Rotinas:**
- `abrir_detalhes_da_partida()` = 0x71b4a0 (match_screens.cpp, cdecl): abre a tela de detalhes de uma partida e grava os parâmetros dela;
- `responder_detalhes_da_partida()` = 0x71bdc0 (match_screens.cpp, cdecl): a resposta da tela de detalhes a um clique (troca de aba, tática do clube, redesenho das abas, menu);
- `fechar_detalhes_da_partida()` = 0x71c170 (match_screens.cpp, cdecl): fecha a tela de detalhes: destrói e libera os objetos guardados nos parâmetros;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (0):** 0 acusadas.
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (034ff3eb3e55dfd3).

**Refazer**, sobre o commit da 182ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 183`
