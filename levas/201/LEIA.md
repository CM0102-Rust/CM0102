# 201ª leva — as telas da partida (7): o Continuar da partida, abrir a escalação e a resposta da escalação

**Rotinas:**
- `responder_continuar_da_partida()` = 0x725ca0 (match_screens.cpp, cdecl): o Continuar da partida: o comando pendente da escalação (em rede, a vaga livre do clube ou a espera pelos outros), as partidas da rede e a escalação;
- `abrir_escalacao_da_partida()` = 0x725f70 (match_screens.cpp, cdecl): abre a tela da escalação da próxima partida do humano e grava o clube, a partida e se joga em casa;
- `responder_escalacao_da_partida()` = 0x727c30 (match_screens.cpp, cdecl): os botões da escalação: nomeia o capitão (o melhor titular) se faltar e deixa o comando pendente; os outros vão à resposta comum;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (0):** 0 acusadas.
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (034ff3eb3e55dfd3).

**Refazer**, sobre o commit da 200ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 201`
