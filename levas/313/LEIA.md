# 313ª leva — a Copa da Coreia; a Supercopa da Coreia; os construtores da K League e da Copa da Liga coreana; os construtores da Primeira e da Segunda Divisão argentinas

**Rotinas:**
- `construct_kor_fa_cup()` = 0x66a460 (kor_fa_cup.cpp, thiscall, ret 8): regra: a Copa da Coreia ao criar a carreira: a regra de disciplina 42 e, sem estado gravado, os 32 inscritos, o fundo de prêmios de 200 mil, as cinco rodadas, a árvore de jogos, as estatísticas e as posições previstas (não escreve o banco);
- `kor_fa_cup_create_round_table()` = 0x66a720 (kor_fa_cup.cpp, thiscall, ret 0x10): regra: a Copa da Coreia tem cinco rodadas em jogo único: a primeira (32 clubes) sorteada em 21/10 e jogada em 30/10; as oitavas 31/10 e 3/11; as quartas 4/11 e 7/11; as semifinais 8/11 e 17/11; a final 18/11 e 25/11; sem prêmio por rodada (uma parte do fundo);
- `kor_fa_cup_collect_entrants()` = 0x66aaf0 (kor_fa_cup.cpp, thiscall, ret 0): regra: a Copa da Coreia inscreve 32 clubes: 22 clubes da nação fora da K League (até 100 achados, ordenados pela reputação e embaralhados), depois os da K League; devolve 1 com a lista cheia; defeito do original: copia 22 mesmo que ache menos;
- `construct_kor_super_cup()` = 0x66d820 (kor_super_cup.cpp, thiscall, ret 8): regra: a Supercopa da Coreia nasce com a regra de disciplina 42 e o banco (5 reservas, 3 substituições); sem estado gravado, os dois clubes, a tabela de rodadas, a árvore de jogos e as estatísticas, sem posições previstas;
- `kor_super_cup_create_round_table()` = 0x66dad0 (kor_super_cup.cpp, thiscall, ret 0x10): regra: a Supercopa da Coreia é um jogo único entre os dois clubes, sorteado em 1/2 e jogado em 10/3 (levado ao dia da semana), categoria 6, sem prêmio;
- `kor_super_cup_create_table()` = 0x66dc10 (kor_super_cup.cpp, thiscall, ret 0): regra: a Supercopa da Coreia é entre o campeão da K League e o da Copa da Coreia; faltando um, entra o primeiro clube da nação; se o mesmo clube ganhou os dois, entra o vice da Copa; sem ele, a caixa de erro;
- `construct_kor_league()` = 0x66ae70 (kor_league.cpp, thiscall, ret 8): regra: a K League ao criar a carreira: a regra de disciplina 42, só a liga (sem fases); sem estado gravado, as regras, a tabela com os dez clubes, os jogos, a ordem, as estatísticas e as posições previstas;
- `construct_kor_league_cup()` = 0x66bf00 (kor_league_cup.cpp, thiscall, ret 8): regra: a Copa da Liga coreana ao criar a carreira: 5 reservas e 3 substituições, a regra de disciplina 42, importância 12; sem estado gravado, duas fases (o grupo B e o mata-mata), os dez clubes, as estatísticas, as posições previstas, as regras, a tabela do grupo A (a própria copa), os jogos e a ordem dele e a fase do grupo B;
- `construct_arg_prm()` = 0x404250 (arg_prm.cpp, thiscall, ret 8): regra: a Primeira Divisão argentina ao criar a carreira: a regra de disciplina 3 e, sem estado gravado, importância 15, duas fases (Apertura e Clausura), as regras, a tabela, os jogos, as estatísticas, as duas fases de meio campeonato, a ordem, as médias do rebaixamento e as posições previstas;
- `construct_arg_second()` = 0x406bf0 (arg_second.cpp, thiscall, ret 8): regra: a Segunda Divisão argentina (Nacional B) ao criar a carreira: a regra de disciplina 3 e, sem estado gravado, importância 15, seis fases (as zonas, a copa do acesso e as repescagens), as regras, a tabela, os jogos, a ordem, as estatísticas, as médias do rebaixamento e as suas posições previstas;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (4):** 4 acusadas.
- **Jogo real comparando até 2002-01-01:** 0 divergências (0c0c36d5).
- **Carreiras Rust ×2 e original até 2002-06-01:** Rust ×2 8fc46a0d, original 29b211de (registrada; não barra a leva desde 31/10/2026). Detector de lixo: igual.

**Refazer**, sobre o commit da 312ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 313`

**Campos tocados:**
- copa +0x3e (inscritos), +0xa7 (6 bytes: clube, semente, situação); competição +0x5d (nação); clube +0x53 (nação), +0x57 (divisão)
- copa +0x49, +0x4a (banco), +0x50 (disciplina), +0x3e e +0xa7 (os dois clubes), rodada +0x11
- liga +0xd9 (flags), +0x2c e +0xc (fases), +0x49, +0x4a (banco), +0x50 (disciplina), +0x51 (importância)
- liga +0x2c e +0xc (fases), +0x50 (disciplina), +0x51 (importância), +8 (estatísticas)
