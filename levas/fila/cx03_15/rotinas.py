# Os codigos de progresso das tres continentais sul-americanas (con_merc_cup.cpp, conmebol_liber.cpp, conmebol_merc.cpp;
# leitor cx03, rodada 5, 24/11/2026). Carreira de 8 ligas (o Brasil jogavel).
# - 0x4c2e40 ConMercCup::set_club_progress, 0x4c4610 ConmebolLiber::set_club_progress e 0x4c5ee0
#   ConmebolMerc::set_club_progress thiscall(clube, fase, atual, melhor, pior) ret 0x14 (a virtual 0x68): Comp_Stats::
#   find_club_entry 0x4a4850 (push clube; mov ecx, [this+8] -> thiscall, ret 4); na fase de grupos (abaixo de 3, 7 e 4)
#   cada codigo 1..4 por uma tabela (os saltos 0x4c2f20/30/40, 0x4c46ac e 0x4c5fa8/b8/c8 conferidos; a Libertadores usa
#   comparacoes no atual e no melhor: 0 e acima de 4 ficam, como nas tabelas); +0x73 atual, +0x74 melhor, +0x75 pior.
#   Prologo mov eax, [esp+4]; mov ecx, [ecx+8].
#   Merconorte: atual e melhor 1, 5, 9, 13; pior 3, 5, 9, 13. Libertadores: atual e melhor 17, 17, 17, 25; pior 9, 9, 17,
#   25. Mercosul: melhor 1, 1, 11, 16; atual e pior 5, 11, 11, 16.
# Usa compare_club_progress do cx02_4 e store_club_progress do cx03_9 (integrar depois deles).
# Iguais no cm0102-1080.exe (as tres inteiras).
# Comparadores: o progresso lado a lado nos tres bytes da entrada.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('con_merc_cup_set_club_progress', 0x4c2e40, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08]),
           ('conmebol_liber_set_club_progress', 0x4c4610, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08]),
           ('conmebol_merc_set_club_progress', 0x4c5ee0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08])]
DOCS = ('os códigos de progresso das três continentais sul-americanas', 'the progress codes of the three South American continental cups',
        [('con_merc_cup_set_club_progress', 'ConMercCup::set_club_progress', '0x4c2e40', 'con_merc_cup.cpp', 'thiscall, ret 0x14',
          'regra: nos grupos da Copa Merconorte (fases abaixo de 3) os lugares 1 a 4 viram os lugares 1, 5, 9 e 13 na copa no atual e no melhor, e 3, 5, 9 e 13 no pior; do mata-mata em diante os códigos ficam como vêm',
          'rule: in the Copa Merconorte groups (stages below 3) places 1 to 4 become the cup places 1, 5, 9 and 13 for the current and best codes, and 3, 5, 9 and 13 for the worst; from the knockout on the codes stay as they come'),
         ('conmebol_liber_set_club_progress', 'ConmebolLiber::set_club_progress', '0x4c4610', 'conmebol_liber.cpp', 'thiscall, ret 0x14',
          'regra: nos grupos da Libertadores (fases abaixo de 7) os lugares 1 a 3 viram o lugar 17 e o 4º o 25 no atual e no melhor; no pior 1º e 2º dão 9, 3º dá 17 e 4º dá 25; do mata-mata em diante os códigos ficam como vêm',
          'rule: in the Copa Libertadores groups (stages below 7) places 1 to 3 become the cup place 17 and place 4 the place 25 for the current and best codes; for the worst places 1 and 2 give 9, place 3 gives 17 and place 4 gives 25; from the knockout on the codes stay as they come'),
         ('conmebol_merc_set_club_progress', 'ConmebolMerc::set_club_progress', '0x4c5ee0', 'conmebol_merc.cpp', 'thiscall, ret 0x14',
          'regra: nos grupos da Mercosul (fases abaixo de 4) o melhor dá 1 para os lugares 1 e 2, 11 para o 3º e 16 para o 4º; o atual e o pior dão 5 para o 1º, 11 para o 2º e o 3º e 16 para o 4º; do mata-mata em diante os códigos ficam como vêm',
          'rule: in the Copa Mercosur groups (stages below 4) the best code gives 1 for places 1 and 2, 11 for place 3 and 16 for place 4; the current and worst codes give 5 for place 1, 11 for places 2 and 3 and 16 for place 4; from the knockout on the codes stay as they come')])
CAMPOS = []
INTERESSA = []
