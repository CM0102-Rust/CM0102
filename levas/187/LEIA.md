# 187ª leva — as telas da partida (5): o painel dos últimos placares, a tela seguinte e a cor do título

**Rotinas:**
- `painel_dos_ultimos_placares()` = 0x723d20 (match_screens.cpp, cdecl): desenha o painel dos últimos placares: título, selo, grupos e partidas do dia, tabela e botões;
- `sem_tela_seguinte()` = 0x7febb0 (scrman.cpp, thiscall): o gerente de telas não tem tela seguinte para avançar;
- `cor_do_titulo()` = 0x839590 (Setup.cpp, cdecl): a cor do texto do título (duas cores pela marca);

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (0):** 0 acusadas.
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (034ff3eb3e55dfd3).

**Refazer**, sobre o commit da 186ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 187`
