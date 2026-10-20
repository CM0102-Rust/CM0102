# 182ª leva — as telas da partida (2): a tela dos últimos placares

**Rotinas:**
- `abrir_ultimos_placares()` = 0x71b1b0 (match_screens.cpp, cdecl): abre a tela dos últimos placares de uma competição e grava os parâmetros dela;
- `montar_ultimos_placares()` = 0x71b300 (match_screens.cpp, cdecl): monta a tela dos últimos placares: fundo, título, menu, painel e botões de voltar e avançar;
- `responder_ultimos_placares()` = 0x71b3d0 (match_screens.cpp, cdecl): a resposta da tela dos últimos placares a um clique (troca de competição, redesenho, menu);

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (0):** 0 acusadas.
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (034ff3eb3e55dfd3).

**Refazer**, sobre o commit da 181ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 182`
