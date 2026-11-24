# Os codigos de progresso e a noticia do destino na Copa da Holanda (hol_cup.cpp; leitor cx02, rodada 5, 24/11/2026).
# Competicoes da Holanda; ATE 2002-08-01.
# - 0x5f5390 HolCup::set_club_progress (a virtual 0x68): set_group_cup_club_progress do cx02_11 com as tabelas lidas do
#   exe (melhor 1,1,47,77; pior 27,47,47,77; atual 17,17,47,77) e os grupos abaixo da fase 19; compare_club_progress do
#   cx02_4. Prologo mov eax, [esp+4]; mov ecx, [ecx+8].
# - 0x5f3c50 HolCup::build_fate_news_text: igual a ItaCup::build_fate_news_text com o mata-mata na fase 19 e o modelo
#   "through to second round" (0x9a7e24); group_cup_fate_news_text do cx02_10. Prologo mov al, [esp+0xc]; cmp al, 0x13.
# Integrar depois do cx02_4, do cx02_10 e do cx02_11. Iguais no cm0102-1080.exe.
# Comparadores: o progresso lado a lado nos tres bytes; o texto lado a lado (compare_text_writer).
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('hol_cup_set_club_progress', 0x5f5390, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08]),
           ('hol_cup_build_fate_news_text', 0x5f3c50, [0x8a, 0x44, 0x24, 0x0c, 0x3c, 0x13])]
DOCS = ('os códigos de progresso e a notícia do destino na Copa da Holanda', 'the Dutch Cup progress codes and fate news',
        [('hol_cup_set_club_progress', 'HolCup::set_club_progress', '0x5f5390', 'hol_cup.cpp', 'thiscall, ret 0x14',
          'regra: na Copa da Holanda, nos dezenove grupos, o 1º e o 2º lugares viram, nas estatísticas, a posição 17 (o melhor, 1; o pior, 27 e 47), o 3º 47 e o 4º 77; no mata-mata, como vêm',
          'rule: in the Dutch Cup, in the nineteen groups, 1st and 2nd place become position 17 in the statistics (the best one, 1; the worst, 27 and 47), 3rd 47 and 4th 77; in the knockout, as they come'),
         ('hol_cup_build_fate_news_text', 'HolCup::build_fate_news_text', '0x5f3c50', 'hol_cup.cpp', 'thiscall, ret 0x28',
          'regra: na Copa da Holanda, quem passa de um grupo (fases 0 a 18) sem notícia ganha a linha "{clube} na segunda fase" com o link do clube; com notícia, a notícia genérica do grupo; a eliminação, a notícia genérica; no mata-mata (fase 19), o resultado do confronto ou a vitória/derrota',
          'rule: in the Dutch Cup a club going through a group (stages 0 to 18) with no news item gets the line "{club} through to second round" with a link to the club; with a news item, the generic group news; an exit, the generic news; in the knockout (stage 19), the tie result or the win/defeat news')])
CAMPOS = []
INTERESSA = []
