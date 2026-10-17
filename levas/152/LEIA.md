# 152ª leva — o laço do dia de jogos (0x6b4360)

**Rotina:** `jogar_o_dia()` = 0x6b4360 (match_day.cpp, ~3,3 KB): joga o dia. Começa a próxima partida, liga os humanos às partidas deles (o ramo "rede"), cria e joga até o fim cada partida sem humano, escreve o placar com a porcentagem na barra, fecha cada partida e libera os resumos.

**Como foi comparada:** sem comparação própria, pela carreira (Rust × original), com contadores de ramo desde o início.

**Referência nova (importante):** com o laço do dia desviado, tudo o que ele chama passa a rodar na pilha particular da simulação, inclusive as rotinas originais que leem variável local sem valor. O lixo que elas leem mudou: a carreira mudou nos **dois** modos, igual em ambos. A original isolada e o Rust continuam idênticos; o que mudou foi a referência:
- Carreiras até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas 19414/b3ee24bdc00da55f, banco 7f2c15024cd99e7e (antes: 459815f5e0dca92b).
- Jogo real comparando até 2002-01-01: impressão 963cea289f055ac3 (antes: f2db52ca99e0789d).

**Resultado** (`cadeia.txt`, `mutacoes_refeitas.txt`):
- Mutações (8): 5 acusadas e 3 escaparam. A da fila era fraca e, refeita (sem a chamada), foi acusada. A das linhas do humano não roda na carreira automática (lacuna nova em `docs/PENDENCIAS.md`). A do contador de terminadas é a lacuna já anotada na 151ª.
- Jogo real comparando até 2002-01-01: 0 divergências.
- Carreiras Rust ×2 e original até 2002-06-01: as três iguais (034ff3eb3e55dfd3).

**Refazer:** sobre o commit da 151ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 152 -`
