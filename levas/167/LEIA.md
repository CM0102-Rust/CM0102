# 167ª leva — o fechamento do motor (1): o gol da bola, a partida abandonada e as vagas do lado

**Rotinas:**
- `bola_na_area()` = 0x6d58f0 (match_eng.cpp, thiscall): 1 com a bola na área de cima, −1 na de baixo, senão 0; devolve só `al`;
- `partida_abandonada()` = 0x6d5930 (match_eng.cpp, thiscall): o minuto do abandono × 11 abaixo do relógio (16 bits); devolve só `al`;
- `vagas_do_lado_sem_conflito()` = 0x6d5af0 (match_eng.cpp, thiscall): a 0x5b8960 na formação do lado;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (4):** 3 acusadas. `a_onze` escapou: nenhuma partida é abandonada na carreira automática (o minuto do abandono fica em −1). (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (034ff3eb3e55dfd3).

**Refazer**, sobre o commit da 166ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 167`
