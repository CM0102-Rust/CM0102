# As posicoes previstas, os codigos de progresso e a noticia do destino na Copa da Liga coreana (kor_league_cup.cpp;
# leitor cx02, rodada 5, 24/11/2026). Competicoes da Coreia; ATE 2002-08-01.
# - 0x66d3a0 KorLeagueCup::set_predicted_positions thiscall() ret 0 (2 chamadas no mundo): o molde da WelPrmCup
#   (italian_cup_predicted_positions com o mata-mata na fase 1 e faixas de 2), linhas 0x38d e 0x395.
# - 0x66d650 KorLeagueCup::set_club_progress (a virtual 0x68): set_group_cup_club_progress do cx02_11 com as tabelas
#   lidas do exe (melhor 1,1,5,7,9; pior 3,3,5,7,9; atual 1,1,5,7,9) e o grupo na fase 0; compare_club_progress do cx02_4.
# - 0x66c650 KorLeagueCup::build_fate_news_text: igual a ItaCup::build_fate_news_text com o mata-mata na fase 1 e o
#   modelo "through to semi finals" (0x9a7dac); group_cup_fate_news_text do cx02_10.
# Integrar depois do cx02_4, do cx02_10 e do cx02_11. Prologos: sub esp, 0x220; mov eax, [esp+4]; mov ecx, [ecx+8];
# mov al, [esp+0xc]; cmp al, 1. Iguais no cm0102-1080.exe.
# Comparadores: as posicoes lado a lado; o progresso lado a lado nos tres bytes; o texto lado a lado.
LIGAS = 'ENG KOR ARG ITA FRA ESP AUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('kor_league_cup_set_predicted_positions', 0x66d3a0, [0x81, 0xec, 0x20, 0x02, 0x00, 0x00]),
           ('kor_league_cup_set_club_progress', 0x66d650, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08]),
           ('kor_league_cup_build_fate_news_text', 0x66c650, [0x8a, 0x44, 0x24, 0x0c, 0x3c, 0x01])]
DOCS = ('as posições previstas, os códigos de progresso e a notícia do destino na Copa da Liga coreana',
        'the Korean League Cup predicted positions, progress codes and fate news',
        [('kor_league_cup_set_predicted_positions', 'KorLeagueCup::set_predicted_positions', '0x66d3a0', 'kor_league_cup.cpp', 'thiscall, ret 0',
          'regra: as posições previstas da Copa da Liga coreana: os dez clubes pela reputação; o mata-mata (fase 1) dá 1 ao campeão e 2 ao vice e uma posição a quem cai na semifinal; os outros em faixas de 2',
          'rule: the Korean League Cup predicted positions: the ten clubs by reputation; the knockout (stage 1) gives 1 to the winner and 2 to the runner-up and a place to the semi-final losers; the rest in bands of 2'),
         ('kor_league_cup_set_club_progress', 'KorLeagueCup::set_club_progress', '0x66d650', 'kor_league_cup.cpp', 'thiscall, ret 0x14',
          'regra: na Copa da Liga coreana, nos dois grupos, os lugares 1 a 5 viram, nas estatísticas, as posições 1, 1 (o pior, 3), 5, 7 e 9; no mata-mata, como vêm',
          'rule: in the Korean League Cup, in the two groups, places 1 to 5 become positions 1, 1 (the worst, 3), 5, 7 and 9 in the statistics; in the knockout, as they come'),
         ('kor_league_cup_build_fate_news_text', 'KorLeagueCup::build_fate_news_text', '0x66c650', 'kor_league_cup.cpp', 'thiscall, ret 0x28',
          'regra: na Copa da Liga coreana, quem passa do grupo (fase 0) sem notícia ganha a linha "{clube} nas semifinais" com o link do clube; com notícia, a notícia genérica do grupo; a eliminação, a notícia genérica; no mata-mata (fase 1), o resultado do confronto ou a vitória/derrota',
          'rule: in the Korean League Cup a club going through the group (stage 0) with no news item gets the line "{club} through to semi finals" with a link to the club; with a news item, the generic group news; an exit, the generic news; in the knockout (stage 1), the tie result or the win/defeat news')])
CAMPOS = []
INTERESSA = []
