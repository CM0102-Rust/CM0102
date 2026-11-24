# A noticia do destino na Copa da Serie C e os codigos de progresso das copas italianas (ita_c_cup.cpp, ita_cup.cpp;
# leitor cx02, rodada 5, 24/11/2026). Competicoes com a Italia (8 ligas, ate 2002-06-01: os grupos das duas copas sao
# no segundo semestre de 2001).
# - 0x63d4c0 ItaCCup::build_fate_news_text: igual a ItaCup::build_fate_news_text com o mata-mata na fase 15 e o modelo
#   "through to second round" (0x9a7e24); group_cup_fate_news_text do cx02_10.
# - 0x63e7f0 ItaCCup::set_club_progress (grupos < 15; tabelas melhor 1,33,59,75, pior 17,33,59,71, atual 1,33,59,75) e
#   0x640480 ItaCup::set_club_progress (grupos < 7; melhor 1,25,33,41, pior 17,25,33,41, atual 1,25,33,41):
#   set_group_cup_club_progress do cx02_11 e compare_club_progress do cx02_4.
#   Integrar depois do cx02_4, do cx02_10 e do cx02_11.
# Convencoes e prologos como no cx02_10 e no cx02_11. Iguais no cm0102-1080.exe.
# Comparadores: o texto lado a lado (compare_text_writer); o progresso lado a lado nos tres bytes da entrada.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('ita_c_cup_build_fate_news_text', 0x63d4c0, [0x8a, 0x44, 0x24, 0x0c, 0x3c, 0x0f]),
           ('ita_c_cup_set_club_progress', 0x63e7f0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08]),
           ('ita_cup_set_club_progress', 0x640480, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08])]
DOCS = ('a notícia do destino na Copa da Série C e os códigos de progresso das copas italianas',
        "the Serie C cup fate news and the Italian cups' progress codes",
        [('ita_c_cup_build_fate_news_text', 'ItaCCup::build_fate_news_text', '0x63d4c0', 'ita_c_cup.cpp', 'thiscall, ret 0x28',
          'regra: na Copa da Série C, quem passa de um grupo (fases 0 a 14) sem notícia ganha a linha "{clube} na segunda fase" com o link do clube; com notícia, a notícia genérica do grupo; a eliminação, a notícia genérica; no mata-mata (fase 15), o resultado do confronto ou a vitória/derrota',
          'rule: in the Serie C cup a club going through a group (stages 0 to 14) with no news item gets the line "{club} through to second round" with a link to the club; with a news item, the generic group news; an exit, the generic news; in the knockout (stage 15), the tie result or the win/defeat news'),
         ('ita_c_cup_set_club_progress', 'ItaCCup::set_club_progress', '0x63e7f0', 'ita_c_cup.cpp', 'thiscall, ret 0x14',
          'regra: na Copa da Série C, nos quinze grupos, os lugares 1 a 4 viram, nas estatísticas, as posições 1, 33, 59 e 75 (o pior lugar 1 vira 17 e o 4 vira 71); no mata-mata, como vêm',
          'rule: in the Serie C cup, in the fifteen groups, places 1 to 4 become positions 1, 33, 59 and 75 in the statistics (a worst place of 1 becomes 17 and of 4 becomes 71); in the knockout, as they come'),
         ('ita_cup_set_club_progress', 'ItaCup::set_club_progress', '0x640480', 'ita_cup.cpp', 'thiscall, ret 0x14',
          'regra: na Copa da Itália, nos sete grupos, os lugares 1 a 4 viram as posições 1, 25, 33 e 41 (o pior lugar 1 vira 17); no mata-mata, como vêm',
          'rule: in the Coppa Italia, in the seven groups, places 1 to 4 become positions 1, 25, 33 and 41 (a worst place of 1 becomes 17); in the knockout, as they come')])
CAMPOS = []
INTERESSA = []
