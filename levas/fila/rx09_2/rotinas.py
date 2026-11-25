# A tabela do grupo 1, a virada e a fase seguinte da Segunda Divisao B espanhola (spa_second_b.cpp; leitor rx09, rodada
# 6, 25/11/2026). So existe com a base NORMAL. Simulacao com a Espanha e a Suecia ate 2002-09-01.
# - 0x856cd0 SpaSecondB::load_group_clubs thiscall() ret 0 -> 0/1: +0x3e = 20; com o ano (palavra +0x40) 2001, cada clube
#   (os primeiros [0xae235c] - 2 x [0xae2350], a conta relida a cada volta) cuja divisao (+0x57) tem o id de um dos quatro
#   grupos (0x9cf62c..0x9cf638) passa a ter o grupo em +0x60 e o registro da liga em +0x57; malloc 59 x +0x3e (caixa
#   0x1cf e 0); as linhas (0x687430, push 0; push 0; push clube; push k) dos clubes com divisao e reserva, a reserva o
#   grupo B1 e a divisao a Segunda B (0x9cf7b8); o 21o: caixa 0x1e0, a divisao zerada e fim (o mesmo defeito do cx14_5).
#   Devolve (k == +0x3e).
# - 0x856f90 SpaSecondB::start_new_season thiscall() ret 0: o molde da TurSecondB (cx14) com as caixas 0x23b, 0x242,
#   0x249 e 0x24f e create_group_stage(k) para k = 0..2.
# - 0x857150 SpaSecondB::advance_stage thiscall() ret 0: na ultima fase a caixa 0x26d; senao +0x30 + 1; em 3 os grupos
#   do acesso (0x857570) e a repescagem (0x857860); em 8 a final da repescagem (0x857a00).
# Prologos sub esp, 0x200 (6 bytes) nas tres; iguais no cm0102-1080.exe.
# Comparadores: a tabela so a original e o plano conferido depois (2001 muda as divisoes dentro dela); a virada so a
# original (o ano + 1); a fase seguinte lado a lado quando nao constroi (compare_advance_stage_building).
# Usa o SEGUNDA_B_SOURCE do rx09_1.
SAVE = False
LIGAS = 'ENG ESP SWE'
ATE = '2002-09-01'
BASE = 'NORMAL'
ROTINAS = [('spa_second_b_load_group_clubs', 0x856cd0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('spa_second_b_start_new_season', 0x856f90, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('spa_second_b_advance_stage', 0x857150, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a tabela do grupo 1, a virada e a fase seguinte da Segunda Divisão B espanhola',
        'the group 1 table, the turnover and the next stage of the Spanish Second Division B',
        [('spa_second_b_load_group_clubs', 'SpaSecondB::load_group_clubs', '0x856cd0', 'spa_second_b.cpp', 'thiscall, ret 0',
          'regra: o grupo 1 tem 20 clubes, os da Segunda B cujo grupo é o 1, na ordem do banco; em 2001 o banco põe cada clube dos quatro grupos na Segunda B, guardando o grupo como divisão reserva; o 21º clube perde a divisão (defeito do original)',
          'rule: group 1 has 20 clubs, the Second Division B ones whose group is group 1, in database order; in 2001 the database puts every club of the four groups into the Second Division B, keeping the group as its reserve division; a 21st club loses its division (defect of the original)'),
         ('spa_second_b_start_new_season', 'SpaSecondB::start_new_season', '0x856f90', 'spa_second_b.cpp', 'thiscall, ret 0',
          'regra: na virada a Segunda B arquiva a temporada, apaga as fases, passa o ano e refaz as regras, a tabela do grupo 1, a ordem, os jogos, os grupos 2 a 4, as posições previstas e os árbitros',
          'rule: at the turnover the Second Division B archives the season, deletes the stages, moves the year on and rebuilds the rules, the group 1 table, the order, the fixtures, groups 2 to 4, the predicted positions and the officials'),
         ('spa_second_b_advance_stage', 'SpaSecondB::advance_stage', '0x857150', 'spa_second_b.cpp', 'thiscall, ret 0',
          'regra: depois dos quatro grupos vêm os quatro grupos do acesso e a repescagem do rebaixamento; depois dela, a final da repescagem',
          'rule: after the four groups come the four promotion play-off groups and the relegation play-off; after it, the relegation play-off final')])
CAMPOS = []
INTERESSA = []
