# As regras de três torneios de seleções com grupos e mata-mata: a Copa da Ásia, a Copa das Nações da Oceania e o futebol
# olímpico (asia_nations.cpp, oceania_nations.cpp, olympics.cpp). Leitor cqz (12/11/2026), lista de 8 ligas.
# - 0x40f4c0 (AsiaNations::set_up_rules), 0x799690 (OceaniaNations::set_up_rules), 0x7a0230 (Olympics::set_up_rules).
# Corpo igual ao da Copa Africana (0x401ab0, já em Rust): os mesmos campos (AFRICAN_NATIONS_RULE_FIELDS; o +0xbe = 2
# na Oceania, não o 1 das eliminatórias), só a data muda: 5/5, 5/2 e 20/2 do ano da copa, dia da semana 5, +0xb8 = 0.
# Convenções conferidas: chamadas pelo construtor com mov ecx, this (thiscall, ret 0); dentro: 0x549f70 (lea ecx, local;
# 4 pushes -> thiscall, ret 0x10) e a virtual +0x3c (4 pushes; mov ecx -> thiscall, ret 0x10).
# Comparadores: só a original (a +0x3c aloca a tabela de rodadas), com o plano (campos, data, +0xb8, retorno 1).
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('asia_nations_set_up_rules', 0x40f4c0, [0x6a, 0xff, 0x68, 0xe8, 0x38, 0x95, 0x00]),
           ('oceania_nations_set_up_rules', 0x799690, [0x6a, 0xff, 0x68, 0xb8, 0xeb, 0x95, 0x00]),
           ('olympics_set_up_rules', 0x7a0230, [0x6a, 0xff, 0x68, 0xf8, 0xef, 0x95, 0x00])]
DOCS = ('as regras da Copa da Ásia, da Copa das Nações da Oceania e do futebol olímpico',
        'the rules of the Asian Cup, the Oceania Nations Cup and the Olympic football',
        [('asia_nations_set_up_rules', 'AsiaNations::set_up_rules', '0x40f4c0', 'asia_nations.cpp', 'thiscall, ret 0', 'regra: os grupos da Copa da Ásia são turno único, 3 pontos a vitória e 1 o empate, desempate 1, 2, 3, dois classificados por grupo, seleções, categoria 10, 7 reservas e 3 trocas; a data do sorteio é 5 do mês 5 do ano da copa, dia da semana 5', "rule: the Asian Cup groups are single round robin, 3 points a win and 1 a draw, tiebreak 1, 2, 3, two qualifiers per group, national teams, category 10, 7 substitutes and 3 changes; the draw date is 5 of month 5 of the cup year, weekday 5"),
         ('oceania_nations_set_up_rules', 'OceaniaNations::set_up_rules', '0x799690', 'oceania_nations.cpp', 'thiscall, ret 0', 'regra: os grupos da Copa das Nações da Oceania seguem as regras da Copa da Ásia; a data do sorteio é 5 do mês 2 do ano da copa, dia da semana 5', 'rule: the Oceania Nations Cup groups follow the Asian Cup rules; the draw date is 5 of month 2 of the cup year, weekday 5'),
         ('olympics_set_up_rules', 'Olympics::set_up_rules', '0x7a0230', 'olympics.cpp', 'thiscall, ret 0', 'regra: os grupos do futebol olímpico seguem as regras da Copa da Ásia; a data do sorteio é 20 do mês 2 do ano olímpico, dia da semana 5', 'rule: the Olympic football groups follow the Asian Cup rules; the draw date is 20 of month 2 of the Olympic year, weekday 5')])
CAMPOS = [('competição +0x3c, +0x43, +0x49, +0x4a, +0xbe..+0xc7 (regras), +0xb6/+0xb8 (data do sorteio), +0xba (tabela de rodadas); valor; grava', 'competition +0x3c, +0x43, +0x49, +0x4a, +0xbe..+0xc7 (rules), +0xb6/+0xb8 (draw date), +0xba (round table); value; writes')]
