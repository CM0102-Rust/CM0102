# As posicoes previstas e o progresso dos clubes da Segunda Divisao argentina (arg_second.cpp; leitor cx08, rodada 5,
# 24/11/2026). Simulacao com a Argentina (as ligas do cx01_8, ate 2002-08-01): as previstas na criacao do jogo (o
# construtor do mwf_23) e na virada; o progresso a cada atualizacao de status.
# - 0x409fe0 ArgSecond::set_predicted_positions thiscall() ret 0 (virtual 0x5c): sem +8 a caixa 0x79e; malloc 0x64
#   (caixa 0x7a6) com os clubes da tabela; comp_util_sort_clubs_by_strength 0x4c08f0 (cdecl; lista, 25); para os 25
#   Comp_Stats::set_club_final_position 0x4a2540 (thiscall em +8; clube, lugar) com 1, 2, 2, 2, 4, 6, 6, 8, 8, 8 e trios
#   em 11, 14, 17, 20 e 23; free. Prologo sub esp, 0x200.
# - 0x40a240 ArgSecond::set_club_progress thiscall(clube, fase, atual, melhor, pior) ret 0x14: find_club_entry 0x4a4850;
#   fases 0 a 2 (com sinal) x 3 em byte (imul); fase 3 melhor = 1 e as tabelas de salto 0x40a334 (atual) e 0x40a324
#   (pior): 1, 2, 3, 4 -> 1, 2, 5, 8; as outras como vem; grava +0x73, +0x74, +0x75. Prologo mov eax, [esp+4];
#   mov ecx, [ecx+8] (7 bytes).
# Comparadores: as previstas a original e depois a copia por cima (as posicoes de cada clube iguais); o progresso lado a
# lado nos 3 bytes (compare_group_championship_progress do cx04_5).
# Depende do cx04_5: integrar depois.
# Iguais no cm0102-1080.exe.
SAVE = False
LIGAS = 'ENG KOR ARG ITA FRA ESP AUS'
ATE = '2002-08-01'
ROTINAS = [('arg_second_set_predicted_positions', 0x409fe0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('arg_second_set_club_progress', 0x40a240, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08])]
DOCS = ('as posições previstas e o progresso na Segunda Divisão argentina', 'the Argentine Second Division predicted positions and club progress',
        [('arg_second_set_predicted_positions', 'ArgSecond::set_predicted_positions', '0x409fe0', 'arg_second.cpp', 'thiscall, ret 0',
          'as posições previstas da Segunda Divisão argentina: os 25 clubes ordenados pela força ficam com 1, 2, 2, 2, 4, 6, 6, 8, 8, 8 e depois trios em 11, 14, 17, 20 e 23',
          'the Argentine Second Division predicted positions: the 25 clubs sorted by strength get 1, 2, 2, 2, 4, 6, 6, 8, 8, 8 and then threes at 11, 14, 17, 20 and 23'),
         ('arg_second_set_club_progress', 'ArgSecond::set_club_progress', '0x40a240', 'arg_second.cpp', 'thiscall, ret 0x14',
          'o progresso na Segunda Divisão argentina: na primeira fase e nas zonas cada código vezes três; na copa de acesso o melhor é 1 e o atual e o pior 1, 2, 3, 4 viram 1, 2, 5, 8; no resto como vêm',
          'the Argentine Second Division progress: in the first phase and the zones each code times three; in the promotion cup the best is 1 and the current and worst 1, 2, 3, 4 become 1, 2, 5, 8; elsewhere as they come')])
CAMPOS = []
INTERESSA = []
