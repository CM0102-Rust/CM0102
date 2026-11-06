# O calendário das copas intercontinentais e da Intertoto: a Intertoto, a Copa Intercontinental, a Copa Interamericana
# e a Copa dos Campeões da CONCACAF (o slot +0x3c create_round_table; intertoto_cup.cpp, world_club_cup.cpp,
# inter_amer_cup.cpp, con_champ.cpp, em eurocomp\ e intercomp\). Frente das competições (Agente D, 06/11/2026),
# carreira de 8 ligas (começa em janeiro de 2001).
# - 0x633300 (IntertotoCup::create_round_table, 952 bytes; 1 chamada até 2002-06-01 com as 8 ligas): as 5 rodadas.
#   Uma mutação.
# - 0x92b770 (WorldClubCup::create_round_table, 308 bytes; 1 chamada): o jogo da Intercontinental. Uma mutação.
# - 0x632340 (InterAmerCup::create_round_table, 307 bytes; 1 chamada): a Interamericana (passageira).
# - 0x4c0f00 (ConChamp::create_round_table, 657 bytes; 2 chamadas): as 3 rodadas da CONCACAF. Uma mutação.
# Reaproveita do motor.rs os ajudantes da 276a (cql10_rodada, cql10_tabela, cql10_gravados, cql10_comparar, Cql10R): as
# quatro têm o corpo das copas inglesas; a da CONCACAF chama ainda a 0x521f10 (o dia 0 do confronto vazio) nas duas
# primeiras rodadas, entre o jogo e os campos (gravações que não se sobrepõem: a cópia a chama depois da rodada).
# Tabelas extraídas da desmontagem. Iguais no cm0102-1080.exe (as quatro inteiras).
# Convenções conferidas (lição da 258a): as quatro são a virtual +0x3c (push saida; push &extra; push &rodadas; push
# fase; mov ecx, copa -> thiscall, ret 0x10). Dentro: 0x944e46 (push tamanho; add esp, 4 -> cdecl), 0x521e60/0x521eb0
# (pela cql10_rodada), 0x521f10 (7 pushes -> cdecl: tabela, rodada, vaga 0, −1, −1, −1, 0) e a caixa (aviso_em).
# Comparadores: lado a lado como as copas inglesas (cql10_comparar: saídas e bytes gravados; a tabela da cópia
# liberada); na CONCACAF o dia 0 dos confrontos da tabela da original é conferido à parte contra o valor fixo.
# Determinismo: nada lê pilha, heap ou endereço como valor.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqp13_intertoto_tabela', 0x633300, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqp13_intercontinental_tabela', 0x92b770, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqp13_interamericana_tabela', 0x632340, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqp13_concacaf_tabela', 0x4c0f00, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('o calendário das copas intercontinentais e da Intertoto: a Intertoto, a Copa Intercontinental, a Copa Interamericana e a Copa dos Campeões da CONCACAF',
        'the calendar of the intercontinental cups and the Intertoto: the Intertoto Cup, the Intercontinental Cup, the Inter-American Cup and the CONCACAF Champions\' Cup',
        [('cqp13_intertoto_tabela', 'IntertotoCup::create_round_table', '0x633300', 'intertoto_cup.cpp', 'thiscall, ret 0x10',
          'regra: a Intertoto tem 5 rodadas em ida e volta: 1ª sorteada em 25/6 e jogada em 26/6 (40 times, volta 4 dias depois), 2ª em 3/7 (32 times: 20 que passaram e 12 novos), 3ª em 17/7 (24: 16 e 8 novos), semifinal em 28/7 e final em 10/8 (6 times: os 3 vencedores vão à Copa da UEFA [inferência]); categoria 7 nas três primeiras e 8 depois; sem prêmios',
          'rule: the Intertoto Cup has 5 two-legged rounds: 1st drawn on 25/6 and played on 26/6 (40 teams, return 4 days later), 2nd on 3/7 (32 teams: 20 through and 12 new), 3rd on 17/7 (24: 16 and 8 new), semi-final on 28/7 and final on 10/8 (6 teams: the 3 winners go to the UEFA Cup [inference]); category 7 in the first three and 8 after; no prizes'),
         ('cqp13_intercontinental_tabela', 'WorldClubCup::create_round_table', '0x92b770', 'world_club_cup.cpp', 'thiscall, ret 0x10',
          'regra: a Copa Intercontinental (o campeão europeu contra o sul-americano [inferência]) é um jogo único em 1º de dezembro, sorteio em 10 de outubro, mando 4 (Tóquio [inferência]), categoria 6, desempate 3',
          'rule: the Intercontinental Cup (the European against the South American champion [inference]) is a single match on 1 December, drawn on 10 October, venue 4 (Tokyo [inference]), category 6, tie-break 3'),
         ('cqp13_interamericana_tabela', 'InterAmerCup::create_round_table', '0x632340', 'inter_amer_cup.cpp', 'thiscall, ret 0x10',
          'regra: a Copa Interamericana é em ida e volta a partir de 26 de janeiro do ano seguinte (7 dias), sorteio em 10 de outubro, categoria 8, desempate de ida e volta 3',
          'rule: the Inter-American Cup is two-legged from 26 January of the following year (7 days), drawn on 10 October, category 8, two-leg tie-break 3'),
         ('cqp13_concacaf_tabela', 'ConChamp::create_round_table', '0x4c0f00', 'con_champ.cpp', 'thiscall, ret 0x10',
          'regra: a Copa dos Campeões da CONCACAF tem 8 times em jogo único: quartas sorteadas em 1/8 e jogadas em 28/8, semifinal em 1/9 e final em 4/9; categoria 8, desempate 3; sem prêmio por rodada (−1)',
          'rule: the CONCACAF Champions\' Cup has 8 teams in single matches: quarter-finals drawn on 1/8 and played on 28/8, semi-final on 1/9 and final on 4/9; category 8, tie-break 3; no per-round prize (−1)')])
CAMPOS = [('copa +0x40 ano da copa; valor (palavra com sinal); lê (as quatro)',
           'cup +0x40 cup year; value (signed word); reads (all four)'),
          ('rodada de copa (0x68 bytes) +0..+0x22, +0x23..+0x29 (o dia 0 do confronto, CONCACAF) e +0x5c..+0x67; valor; grava',
           'cup round (0x68 bytes) +0..+0x22, +0x23..+0x29 (pairing day 0, CONCACAF) and +0x5c..+0x67; value; writes')]
INTERESSA = ['CMRTE']
