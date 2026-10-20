# 188ª leva — a lista de observados, a entrada do treino e a carga de treino

**Rotinas:**
- `lista_de_observados()` = 0x5d5ab0 (game.cpp, thiscall): o gerente da lista de observados guardado no gerente de transferências;
- `entrada_do_treino()` = 0x5d5ac0 (game.cpp, thiscall): a entrada de 6 bytes do jogador na tabela do treino;
- `gravar_carga_de_treino()` = 0x62c660 (injury.cpp, thiscall): grava a carga de treino do jogador no registro de condição;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (3):** 3 acusadas.
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (034ff3eb3e55dfd3).

**Refazer**, sobre o commit da 187ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 188`
