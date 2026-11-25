# As posicoes previstas e os codigos de progresso da Segunda Divisao sueca (swe_second.cpp; leitor rx09, rodada 6,
# 25/11/2026). So existe com a base NORMAL. Simulacao com a Espanha e a Suecia ate 2002-09-01.
# - 0x893470 SweSecond::set_predicted_positions thiscall() ret 0: sem estatisticas a caixa 0x48b; malloc 0x30 (caixa
#   0x493); g = -1..4, qsort com 0x4b07d0 e set_club_final_position 0x4a2540 com 6k + 1 (imul de 8 bits); free.
# - 0x8935e0 SweSecond::set_club_progress thiscall(clube, fase, atual, melhor, pior) ret 0x14: find_club_entry 0x4a4850
#   sem conferir; fase < 5: os tres x 5 - 4 (imul de 8 bits); fase 6: 1 -> 0x37, 7 -> 0x3d, 13 -> 0x43 em cada um;
#   senao como vieram.
# Prologos: sub esp, 0x204 (6 bytes); mov eax, [esp+4]; mov ecx, [ecx+8] (7 bytes). Iguais no cm0102-1080.exe.
# Comparadores: os dois lado a lado (o byte previsto de cada clube; os 3 bytes da entrada).
# Depende do cx14_3 (grouped_league_predicted_positions) e do rx09_7 (SWEDISH_SECOND_SOURCE).
SAVE = False
LIGAS = 'ENG ESP SWE'
ATE = '2002-09-01'
BASE = 'NORMAL'
ROTINAS = [('swe_second_set_predicted_positions', 0x893470, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('swe_second_set_club_progress', 0x8935e0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08])]
DOCS = ('as posições previstas e os códigos de progresso da Segunda Divisão sueca',
        'the predicted positions and the progress codes of the Swedish Second Division',
        [('swe_second_set_predicted_positions', 'SweSecond::set_predicted_positions', '0x893470', 'swe_second.cpp', 'thiscall, ret 0',
          'regra: as posições previstas da Segunda sueca: cada uma das seis seções é ordenada pela força e o k-ésimo clube de cada seção recebe a posição prevista 6k + 1',
          'rule: the predicted positions of the Swedish Second Division: each of the six sections is sorted by strength and the k-th club of each section gets the predicted position 6k + 1'),
         ('swe_second_set_club_progress', 'SweSecond::set_club_progress', '0x8935e0', 'swe_second.cpp', 'thiscall, ret 0x14',
          'regra: os códigos de progresso (atual, melhor, pior) de um clube da Segunda sueca por fase: nas seções cada código vira 5v − 4; na repescagem do rebaixamento os códigos 1, 7 e 13 viram 55, 61 e 67; no acesso e depois, como vieram',
          'rule: the progress codes (current, best, worst) of a Swedish Second Division club by stage: in the sections each code becomes 5v − 4; in the relegation play-off the codes 1, 7 and 13 become 55, 61 and 67; in the promotion play-off and later, as they came')])
CAMPOS = []
INTERESSA = []
