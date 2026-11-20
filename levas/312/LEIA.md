# 312ª leva — a Copa da Finlândia; a Copa da Noruega; a Taça de Portugal; o construtor da Supertaça portuguesa

**Rotinas:**
- `construct_fin_cup()` = 0x592250 (fin_cup.cpp, thiscall, ret 8): regra: a Copa da Finlândia ao criar a carreira: a regra de disciplina 11, 7 reservas e 3 substituições e, sem estado gravado, os 76 inscritos, o fundo de prêmios de 200 mil, as sete rodadas, a árvore de jogos, as estatísticas e as posições previstas;
- `fin_cup_create_round_table()` = 0x592510 (fin_cup.cpp, thiscall, ret 0x10): regra: a Copa da Finlândia tem sete rodadas em jogo único, decididas no dia: a primeira (64 clubes, os da Segunda Divisão e abaixo) sorteada em 15/4 e jogada em 29/4; a segunda 3/5 e 16/5; a terceira (entram os 8 da Primeira Divisão) 5/6 e 21/6; a quarta (entram os 4 primeiros da Premier) 22/6 e 25/7; as quartas 27/7 e 19/9; as semifinais 21/9 e 17/10; a final 18/10 e 10/11; sem prêmio por rodada (uma parte do fundo);
- `fin_cup_collect_entrants()` = 0x592a20 (fin_cup.cpp, thiscall, ret 0): regra: a Copa da Finlândia inscreve 76 clubes: 44 da Segunda Divisão e abaixo (até 80 achados, ordenados pela reputação e embaralhados), depois os da Primeira Divisão, depois os da Premier que terminaram abaixo do 5º lugar e por fim os 5 primeiros; os da Premier com semente 1; devolve 1 com a lista cheia; defeito do original: copia 44 mesmo que ache menos;
- `construct_nor_cup()` = 0x78f020 (nor_cup.cpp, thiscall, ret 8): regra: a Copa da Noruega ao criar a carreira: a regra de disciplina 22, 7 reservas e 3 substituições e, sem estado gravado, os 128 inscritos, o fundo de prêmios de 800 mil, as sete rodadas, a árvore de jogos, as estatísticas e as posições previstas;
- `nor_cup_create_round_table()` = 0x78f2e0 (nor_cup.cpp, thiscall, ret 0x10): regra: a Copa da Noruega tem sete rodadas: a primeira (128 clubes) sorteada em 7/4 e jogada em 5/5; a segunda 6/5 e 26/5; a terceira 27/5 e 2/6; a quarta (com jogo extra no empate) 3/6 e 30/6; as quartas 1/7 e 4/8; as semifinais 5/8 e 25/9; a final sorteada em 29/9 e jogada em 24 ou 31 de outubro (31/10 levado ao dia da semana; se isso passar de 1/11, o dia 24), com jogo extra 3 dias depois; sem prêmio por rodada (uma parte do fundo);
- `nor_cup_collect_entrants()` = 0x78f890 (nor_cup.cpp, thiscall, ret 0): regra: a Copa da Noruega inscreve 128 clubes, nenhum com "2" no nome curto (os times reservas): primeiro os da Premier (semente 1), da Primeira Divisão e dos oito grupos da Segunda; depois os da Terceira Divisão; por fim os clubes noruegueses sem divisão; o clube cuja divisão aponta para a própria copa tem a divisão apagada; devolve 1 com a lista cheia;
- `construct_por_cup()` = 0x7cd980 (por_cup.cpp, thiscall, ret 8): regra: a Taça de Portugal ao criar a carreira: a regra de disciplina 23 e, sem estado gravado, os 164 inscritos, o fundo de prêmios de 800 mil, as oito rodadas, a árvore de jogos, as estatísticas e as posições previstas (não escreve o banco);
- `por_cup_create_round_table()` = 0x7cdc40 (por_cup.cpp, thiscall, ret 0x10): regra: a Taça de Portugal tem oito rodadas em jogo único, com jogo extra no empate até as semifinais: a primeira (128 clubes das divisões de baixo) sorteada em 10/9 e jogada em 19/9; a segunda (entram os 18 da Segunda Divisão) 23/9 e 5/10; a terceira (entram 17 da Primeira) 18/10 e 14/11; a quarta (entra o último) 29/12 e 9/1; a quinta (15 clubes em 7 jogos, como o original) 10/1 e 26/1; as quartas 3/2 e 9/2; as semifinais 20/3 e 12/4; a final 13/4 e 21/5; sem prêmio por rodada (uma parte do fundo);
- `por_cup_collect_entrants()` = 0x7ce220 (por_cup.cpp, thiscall, ret 0): regra: a Taça de Portugal inscreve 164 clubes: até 128 das divisões de baixo (os grupos da Segunda Divisão B, depois a Terceira Divisão e os portugueses da divisão inferior), sem os times B que não se ligam a clube nenhum; depois os da Segunda Divisão e os da Primeira; por fim troca o último inscrito com um sorteado até 18 lugares antes; devolve 1 com a lista cheia;
- `construct_por_super()` = 0x7d2e40 (por_super.cpp, thiscall, ret 8): regra: a Supertaça portuguesa nasce com a regra de disciplina 23; sem estado gravado, os dois clubes, a tabela de rodadas, a árvore de jogos e as estatísticas, sem posições previstas;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (6):** 5 acusadas. `mwe19_reservas_entram` escapou: MUTACAO EQUIVALENTE NESTA BASE (medido). `nor_cup_collect_entrants` deixa de fora os times reservas, os clubes com "2" no nome curto (`crt_strstr(short_name, "2")`); a mutacao os deixa entrar. Medido: o contador `mwe_19 norwegian cup entrants` marca 1 execucao no jogo real da leva (a rotina roda na janela), e o comparador `compare_nor_cup_collect_entrants` -- lado a lado, com o gerador reposto, conferindo cada uma das entradas -- deu 0 divergencias com a mutacao aplicada (comparando ate a data da cadeia e na carga do save de ferias). Portanto, nesta base (DATABASE=MINIMUM), nenhum clube das divisoes norueguesas carregadas que a rotina percorre tem "2" no nome curto: a regra dos reservas so se distingue numa base que traga os times B noruegueses. (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (f1f23a87).
- **Carreiras Rust ×2 e original até 2002-06-01:** Rust ×2 ae792160, original ff0d5539 (registrada; não barra a leva desde 31/10/2026). Detector de lixo: igual.

**Refazer**, sobre o commit da 311ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 312`

**Campos tocados:**
- copa +0x3e (inscritos), +0xa7 (6 bytes: clube, semente, situação); clube +0x57 (divisão), +0x5f (posição na última temporada)
- copa +0x3e (inscritos), +0xa7 (6 bytes: clube, semente, situação); clube +0x38 (nome curto), +0x53 (nação), +0x57 (divisão)
- copa +0x3e (inscritos), +0xa7 (6 bytes: clube, semente, situação); clube +0x53 (nação), +0x57 (divisão)
- copa +0x50 (disciplina)
