# O banco das copas francesas, o clube com o mando por estar numa divisao inferior e as rodadas da Copa da Liga francesa
# (fra_cup.cpp, fra_lge_cup.cpp; leitor cx02, rodada 5, 24/11/2026). Competicoes com a Franca (8 ligas); ATE 2002-08-01
# para alcancar a virada da Copa da Liga francesa (a set_up_rounds roda na virada).
# - 0x5bb100 FraCup::set_fixture_substitutes thiscall(partida) ret 4 (a virtual 0x7c da Copa da Franca e da Copa da Liga,
#   vtables 0x96a864 e 0x96aad8): a final (+0x30 = 0x96) com 5 reservas, as outras rodadas com +0x49 da copa; depois +0x4a
#   no nibble seguinte. Prologo mov eax, [esp+4]; cmp word [eax+0x30], 0x96 (10 bytes).
# - 0x5bb030 FraCup::club_in_higher_division thiscall(a, b) ret 8 -> clube (a virtual 0x90 da Cup): as chaves 0x9cf5cc
#   (Primeira), 0x9cf5d0 (Segunda), 0x9cf5d4 (National), 0x9cf5d8 (CFA); a reputacao (+0x69, word com sinal) da divisao
#   de b contra a do registro de competicao da chave ([0xae23d0] + chave x 0x6b). Prologo mov eax, [esp+4]; push esi.
# - 0x5bd5d0 FraLgeCup::set_up_rounds: premio 500.000 e a tabela (set_up_prize_rounds do cx02_4: integrar depois dele).
# Iguais no cm0102-1080.exe.
# Comparadores: a palavra +0x4d lado a lado; o clube devolvido (compare_value); as rodadas lado a lado (cx02_4).
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('fra_cup_set_fixture_substitutes', 0x5bb100, [0x8b, 0x44, 0x24, 0x04, 0x66, 0x81, 0x78, 0x30, 0x96, 0x00]),
           ('fra_cup_club_in_higher_division', 0x5bb030, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('fra_lge_cup_set_up_rounds', 0x5bd5d0, [0x56, 0x8b, 0xf1, 0x6a, 0x00])]
DOCS = ('o banco das copas francesas, o mando de quem joga numa divisão inferior e as rodadas da Copa da Liga francesa',
        "the French cups' bench, the home advantage of a lower-division club and the French League Cup rounds",
        [('fra_cup_set_fixture_substitutes', 'FraCup::set_fixture_substitutes', '0x5bb100', 'fra_cup.cpp', 'thiscall, ret 4',
          'regra: nas copas francesas a final tem 5 reservas no banco e as outras rodadas o número da copa; as substituições permitidas são as da copa',
          "rule: in the French cups the final names 5 substitutes and the other rounds the cup's number; the substitutions allowed are the cup's"),
         ('fra_cup_club_in_higher_division', 'FraCup::club_in_higher_division', '0x5bb030', 'fra_cup.cpp', 'thiscall, ret 8',
          'regra: na Copa da França, contra um clube da Primeira Divisão quem não é da Primeira nem da Segunda joga em casa; contra um da Segunda, quem está numa divisão de reputação abaixo da do National; contra um do National, quem está abaixo da CFA; nos outros casos ninguém',
          'rule: in the Coupe de France, against a First Division club a club from outside the First and Second Divisions plays at home; against a Second Division club, one in a division of lower reputation than the National; against a National club, one below the CFA; otherwise nobody'),
         ('fra_lge_cup_set_up_rounds', 'FraLgeCup::set_up_rounds', '0x5bd5d0', 'fra_lge_cup.cpp', 'thiscall, ret 0',
          'regra: a Copa da Liga francesa tem um prêmio total de 500.000; monta a tabela de rodadas',
          'rule: the French League Cup has a 500,000 prize fund; builds the round table')])
CAMPOS = []
INTERESSA = []
