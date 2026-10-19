# 180ª leva — as telas da partida (1): o rótulo do controle, fechar a tela e a espera

**Rotinas:**
- `rotulo_do_controle()` = 0x725a70 (match_screens.cpp, cdecl): o rótulo do botão de controle da partida (esperando, começar, recomeçar, pausar ou retomar, continuar) e se ele fica desligado;
- `fechar_tela_da_partida()` = 0x71b120 (match_screens.cpp, cdecl): fecha a tela da partida: destrói e libera os parâmetros 0x2d e 0x1d do gerente de telas;
- `abrir_tela_de_espera()` = 0x727f00 (match_screens.cpp, cdecl): abre a tela de espera do técnico humano no gerente de telas;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (0):** 0 acusadas.
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (034ff3eb3e55dfd3).

**Refazer**, sobre o commit da 179ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 180`
