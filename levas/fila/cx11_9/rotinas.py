# O progresso dos clubes da Serie C2 italiana, grupos A e B (ita_ser_c2a.cpp, ita_ser_c2b.cpp; leitor cx11, rodada 5,
# 24/11/2026). Simulacao com a Italia e as divisoes inferiores: a virtual 0x68 da 0x684cd0, a cada linha da tabela em
# cada atualizacao de status (a temporada inteira). Como no cx11_2, a C2 so existe com a base NORMAL ou MAXIMA (BASE
# abaixo): a prova pede DATABASE=NORMAL na carreira.
# - 0x6544b0 ItaSerC2a::set_club_progress e 0x658640 ItaSerC2b::set_club_progress thiscall(clube, fase, atual, melhor,
#   pior) ret 0x14 (virtual 0x68): o corpo do ItaSerC1a::set_club_progress (conferido instrucao a instrucao, as duas
#   tabelas de salto e os indices iguais): Comp_Stats::find_club_entry 0x4a4850 (push clube; mov ecx, [ecx+8] ->
#   thiscall, ret 4), grava +0x73, +0x74, +0x75: set_c1_club_progress. Prologo mov eax, [esp+4]; mov ecx, [ecx+8]
#   (7 bytes).
# Iguais no cm0102-1080.exe (inteiras). Comparadores: lado a lado nos 3 bytes da entrada (compare_c1_club_progress).
SAVE = False
LIGAS = 'ENG ITA'
ATE = '2002-09-30'
BASE = 'NORMAL'
ROTINAS = [('ita_ser_c2a_set_club_progress', 0x6544b0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08]),
           ('ita_ser_c2b_set_club_progress', 0x658640, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08])]
DOCS = ('o progresso dos clubes da Série C2 italiana, grupos A e B', 'the club progress of the Italian Serie C2 groups A and B',
        [('ita_ser_c2a_set_club_progress', 'ItaSerC2a::set_club_progress', '0x6544b0', 'ita_ser_c2a.cpp', 'thiscall, ret 0x14',
          'a previsão de colocação do clube na C2/A, a mesma regra da C1: na liga, melhor e pior colocação viram faixas (2 a 5 = a repescagem: melhor 2, pior 5; 14 a 17 = o play-out: melhor 14, pior 17); na repescagem, as colocações + 1; no play-out, quem fica fica em 14º e quem perde em 16º',
          "the club's place forecast in C2/A, the same rule as C1's: in the league, best and worst place become bands (2 to 5 = the play-off: best 2, worst 5; 14 to 17 = the play-out: best 14, worst 17); in the play-off, places + 1; in the play-out, the winner stays at 14 and the loser at 16"),
         ('ita_ser_c2b_set_club_progress', 'ItaSerC2b::set_club_progress', '0x658640', 'ita_ser_c2b.cpp', 'thiscall, ret 0x14',
          'a mesma regra na C2/B',
          'the same rule in C2/B')])
CAMPOS = []
INTERESSA = []
