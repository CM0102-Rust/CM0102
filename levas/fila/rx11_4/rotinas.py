# Os códigos de progresso e a situação de classificação de uma seleção nas eliminatórias africanas (wc_african_cup.cpp;
# leitor rx11, rodada 6, 25/11/2026). Simulação na campanha de 2006 (2004 e 2005 na carreira de teste): LIGAS = 'ENG' e
# ATE = '2005-12-31', como os cx16_7..12. Usa o compare_progress_codes do cx16_7 e o qualifying_stat_byte do cx16_8:
# integrar depois deles (INTEGRAR_ANTES.txt).
# - 0x9115a0 WcAfricanCup::set_club_progress thiscall(clube, fase, atual, melhor, pior) ret 0x14 (virtual +0x68):
#   Comp_Stats::find_club_entry 0x4a4850 (push clube; ecx = [this+8]); os três códigos (+0x73 atual, +0x74 melhor, +0x75
#   pior) pelas faixas do formato +0xba e da fase (com sinal), das 12 tabelas de salto 0x911a78..0x911b5c (decodificadas
#   na desmontagem): primeira fase (-1), grupos, repescagem. Prólogo mov eax, [esp+4]; push ebx (5 bytes).
# - 0x911b80 WcAfricanCup::get_qualification_status thiscall(clube) ret 4 (virtual +0x70 das seções africana, asiática,
#   da CONCACAF e da Oceania): Comp_Stats::get_club_stat 0x4a2e10 (0x12) e _ftol; o byte = 1: -2, senão -3 (eax).
#   Prólogo mov eax, [esp+4]; mov ecx, [ecx+8] (7 bytes).
# - 0x911ba0 WcAfricanCup::get_qualification_status_change thiscall(clube) ret 4 (virtual +0x74): antes = stat 0x13,
#   agora = 0x12; agora = 1: -1 (sempre, diferente da Oceania); senão antes = 1. O byte em al. Prólogo push ebx; push esi;
#   push edi; mov edi, [esp+0x10] (7 bytes).
# Iguais no cm0102-1080.exe. Comparadores: lado a lado (os três códigos da entrada; o retorno das duas leitoras).
SAVE = False
LIGAS = 'ENG'
ATE = '2005-12-31'
ROTINAS = [('wc_african_cup_set_club_progress', 0x9115a0, [0x8b, 0x44, 0x24, 0x04, 0x53]),
           ('wc_african_cup_get_qualification_status', 0x911b80, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08]),
           ('wc_african_cup_get_qualification_status_change', 0x911ba0, [0x53, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x10])]
DOCS = ('os códigos de progresso e a situação de classificação de uma seleção nas eliminatórias africanas',
        "a nation's progress codes and qualification status in the African qualifying",
        [('wc_african_cup_set_club_progress', 'WcAfricanCup::set_club_progress', '0x9115a0', 'wc_african_cup.cpp', 'thiscall, ret 0x14',
          'regra: o progresso de uma seleção nas eliminatórias africanas: na primeira fase o vencedor é 1 no melhor e 21 (19 com seis grupos) no atual, o perdedor 25; nos grupos as posições viram as faixas da colocação final (cinco grupos: 1, 6, 11, 16, 21; quatro: 1, 5, 9, 13, 17, 21; seis: 1, 7, 13, 19); na repescagem o vencedor 1 e o perdedor 3 (4 com seis grupos)',
          'rule: the African qualifying progress of a nation: in the first round the winner is 1 at best and 21 (19 with six groups) now, the loser 25; in the groups the places map to bands of the final position (five groups: 1, 6, 11, 16, 21; four: 1, 5, 9, 13, 17, 21; six: 1, 7, 13, 19); in the play-off the winner 1 and the loser 3 (4 with six groups)'),
         ('wc_african_cup_get_qualification_status', 'WcAfricanCup::get_qualification_status', '0x911b80', 'wc_african_cup.cpp', 'thiscall, ret 4',
          'regra: a situação de uma seleção nas eliminatórias africanas, asiáticas, da CONCACAF e da Oceania é classificada (-2) enquanto o código de progresso é 1, e não classificada (-3) fora dele',
          "rule: a nation's status in the African, Asian, CONCACAF and Oceania qualifying is qualified (-2) while its progress code is 1, and not (-3) otherwise"),
         ('wc_african_cup_get_qualification_status_change', 'WcAfricanCup::get_qualification_status_change', '0x911ba0', 'wc_african_cup.cpp', 'thiscall, ret 4',
          'regra: nas eliminatórias africanas a situação cai (-1) sempre que o código é 1 e sobe (1) quando deixa o 1; senão fica (0)',
          'rule: in the African qualifying the status falls (-1) whenever the code is 1 and rises (1) when it leaves 1; otherwise it stays (0)')])
CAMPOS = []
INTERESSA = []
