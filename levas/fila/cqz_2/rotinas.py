# As regras das fases finais da Eurocopa, da Copa das Confederações e da Copa do Mundo (euro_champ.cpp, fifa_confed.cpp,
# world_cup.cpp). Leitor cqz (12/11/2026), lista de 8 ligas.
# - 0x57b630 (EuroChamp::set_up_rules), 0x58bdb0 (FifaConfed::set_up_rules), 0x92d200 (WorldCup::set_up_rules).
# Campos conferidos na desmontagem (a ordem das escritas é a da original). A Eurocopa e a Copa do Mundo gravam também a
# data da semeadura (+0x13e/+0x140 na Eurocopa: 10/1; +0x18e/+0x190 na Copa: 15/12, ano −1) antes da do sorteio (10/2;
# 27/12, ano −1); a Copa das Confederações monta a tabela de rodadas ANTES da data (20/2) e grava +0xd9 = 0x28.
# Convenções conferidas: thiscall, ret 0 (chamadas pelo construtor com mov ecx); 0x549f70 (lea ecx, local; 4 pushes ->
# thiscall, ret 0x10); a virtual +0x3c (4 pushes; mov ecx -> thiscall, ret 0x10).
# Comparadores: só a original (a +0x3c aloca a tabela), com o plano: campos, as duas datas, +0xb8 e retorno 1.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('euro_champ_set_up_rules', 0x57b630, [0x6a, 0xff, 0x68, 0xd0, 0x87, 0x95, 0x00]),
           ('fifa_confed_set_up_rules', 0x58bdb0, [0x83, 0xec, 0x08, 0xb0, 0x02]),
           ('world_cup_set_up_rules', 0x92d200, [0x6a, 0xff, 0x68, 0xd0, 0x5f, 0x96, 0x00])]
DOCS = ('as regras da Eurocopa, da Copa das Confederações e da Copa do Mundo',
        'the rules of the European Championship, the Confederations Cup and the World Cup',
        [('euro_champ_set_up_rules', 'EuroChamp::set_up_rules', '0x57b630', 'euro_champ.cpp', 'thiscall, ret 0', 'regra: os grupos da Eurocopa são turno único, 3 pontos a vitória e 1 o empate, desempate 4, 1, 2, dois classificados por grupo, seleções, categoria 10, 7 reservas e 3 trocas; a semeadura é em 10 de janeiro e o sorteio em 10 de fevereiro do ano da fase final', 'rule: the European Championship groups are single round robin, 3 points a win and 1 a draw, tiebreak 4, 1, 2, two qualifiers per group, national teams, category 10, 7 substitutes and 3 changes; the seeding is on 10 January and the draw on 10 February of the finals year'),
         ('fifa_confed_set_up_rules', 'FifaConfed::set_up_rules', '0x58bdb0', 'fifa_confed.cpp', 'thiscall, ret 0', 'regra: os grupos da Copa das Confederações seguem as regras da Eurocopa; o sorteio é em 20 de fevereiro do ano da copa', 'rule: the Confederations Cup groups follow the European Championship rules; the draw is on 20 February of the cup year'),
         ('world_cup_set_up_rules', 'WorldCup::set_up_rules', '0x92d200', 'world_cup.cpp', 'thiscall, ret 0', 'regra: os grupos da Copa do Mundo seguem as regras da Eurocopa; a semeadura é em 15 de dezembro e o sorteio em 27 de dezembro do ano anterior à Copa, no dia exato', 'rule: the World Cup groups follow the European Championship rules; the seeding is on 15 December and the draw on 27 December of the year before the finals, on the exact day')])
CAMPOS = [('competição +0x3a, +0x3c, +0x43, +0x49, +0x4a, +0xbe..+0xc8, +0xd9 (regras), +0xb6/+0xb8 (sorteio), +0x13e/+0x140 (semeadura da Eurocopa), +0x18e/+0x190 (semeadura da Copa), +0xba; valor; grava', 'competition +0x3a, +0x3c, +0x43, +0x49, +0x4a, +0xbe..+0xc8, +0xd9 (rules), +0xb6/+0xb8 (draw), +0x13e/+0x140 (Euro seeding), +0x18e/+0x190 (World Cup seeding), +0xba; value; writes')]
