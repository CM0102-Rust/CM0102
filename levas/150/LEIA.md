# 150ª leva — a escalação automática (0x6dbc50)

**Rotina:** `escalar_automatico()` = 0x6dbc50 (match_man.cpp, ~7,7 KB), a escalação que a comissão técnica faz contra o adversário.
- Fora do clube humano, compara o jogo de hoje com o próximo da lista (reputação da competição, rival, mesma cidade, dias até lá) para decidir se poupa titulares.
- Filtra os 50 jogadores das linhas (estado, suspensão, já jogou, impedido, emprestado), escala (0x6e2000), escolhe a formação, as três referências, o rodízio, o capitão, as instruções e as ordens gerais pela postura.
- Com ela, `match_man.cpp` fica 100% em Rust.

**Como foi portada:** os locais vivem num quadro de bytes com os mesmos deslocamentos da pilha original, zerado como a original determinizada (a 0x6dbc50 está nas `ZERADAS`). Assim os vetores se sobrepõem como lá, por exemplo o curto da 1ª vaga extra que a original nunca escreve.

**Como foi comparada:** sem comparação própria (ela aloca a lista e os vetores do rodízio e chama 24 rotinas ainda originais), pela carreira (Rust × original), como as das levas 145, 148 e 149. As rotinas que ela chama seguem comparadas.

**Resultado** (`cadeia.txt`, `mutacoes_refeitas.txt`):
- Mutações (17): 12 acusadas e 5 escaparam; as 5 refeitas mais fortes: as 5 acusadas (sem lacuna).
- Jogo real comparando até 2002-01-01: 0 divergências (6500 escalações pela sonda, nenhuma com lixo).
- Carreiras Rust ×2 e original até 2002-06-01: impressão 459815f5e0dca92b.

**Refazer:** sobre o commit da 149ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 150 -`
