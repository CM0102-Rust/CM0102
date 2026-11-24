# Os codigos de progresso das Copas da Liga da Irlanda e da Irlanda do Norte (ire_lge_cup.cpp, nir_lge_cup.cpp; leitor
# cx02, rodada 5, 24/11/2026). Competicoes da Irlanda e da Irlanda do Norte; ATE 2002-08-01.
# - 0x638690 IreLgeCup::set_club_progress e 0x78df40 NirLgeCup::set_club_progress thiscall(clube, fase, atual, melhor,
#   pior) ret 0x14 (a virtual 0x68, chamada por League::update_positions_and_status nos grupos e por
#   Cup::record_team_progress no mata-mata): Comp_Stats::find_club_entry 0x4a4850 (push clube; mov ecx, [this+8] ->
#   thiscall, ret 4); nos grupos (fase com sinal < 7 / < 3) cada codigo por uma tabela de salto (as tabelas lidas do exe:
#   0x638690 melhor 1,9,17,25, pior 5,9,17,25, atual 1,9,17,25; 0x78df40 melhor 1,1,9,13,17, pior 5,5,9,13,17, atual
#   1,5,9,13,17), fora delas o codigo como vem. O molde das sete (Ita, ItaC, Gre, Hol, Kor, Ire, Nir) e o mesmo; o
#   auxiliar set_group_cup_club_progress fica neste item (o cx02_12, o Gre, o Hol e o Kor o usam). Usa
#   compare_club_progress do cx02_4 (integrar depois dele). Prologo mov eax, [esp+4]; mov ecx, [ecx+8] (7 bytes).
# Iguais no cm0102-1080.exe.
# Comparadores: lado a lado nos tres bytes da entrada.
LIGAS = 'ENG IRL NIR'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('ire_lge_cup_set_club_progress', 0x638690, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08]),
           ('nir_lge_cup_set_club_progress', 0x78df40, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08])]
DOCS = ('os códigos de progresso das Copas da Liga da Irlanda e da Irlanda do Norte',
        "the Irish and Northern Irish League Cups' progress codes",
        [('ire_lge_cup_set_club_progress', 'IreLgeCup::set_club_progress', '0x638690', 'ire_lge_cup.cpp', 'thiscall, ret 0x14',
          'regra: na Copa da Liga irlandesa, nos sete grupos, o 1º, 2º, 3º e 4º lugares viram, nas estatísticas da copa, as posições 1, 9, 17 e 25 (o pior lugar 1 vira 5); no mata-mata, as posições como vêm',
          'rule: in the Irish League Cup, in the seven groups, 1st, 2nd, 3rd and 4th place become positions 1, 9, 17 and 25 in the cup statistics (a worst place of 1 becomes 5); in the knockout, the positions as they come'),
         ('nir_lge_cup_set_club_progress', 'NirLgeCup::set_club_progress', '0x78df40', 'nir_lge_cup.cpp', 'thiscall, ret 0x14',
          'regra: na Copa da Liga da Irlanda do Norte, nos três grupos, os lugares 1 a 5 viram as posições 1, 5, 9, 13 e 17 (o melhor lugar 2 vira 1 e o pior lugar 1 vira 5); no mata-mata, como vêm',
          'rule: in the Northern Irish League Cup, in the three groups, places 1 to 5 become positions 1, 5, 9, 13 and 17 (a best place of 2 becomes 1 and a worst place of 1 becomes 5); in the knockout, as they come')])
CAMPOS = []
INTERESSA = []
