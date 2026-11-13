# The Croatian leagues: the next stage and the third tier swap (cro_a1.cpp, cro_a2b.cpp). Reader mwd (12/11/2026),
# round 2 (1 call each: the end of the league in May 2002 and the turnover).
# - 0x514bd0 CroA1::advance_stage, 0x517180 CroA2b::advance_stage: the template of state_league_advance_stage
#   (bra_reg_bahia.rs), checked instruction by instruction (lines 0x1fa and 0x1e9; the play-offs 0x514c60 and 0x517210,
#   mwd_7).
# - 0x514ed0 cro_a1_swap_second_and_third_tier_clubs: the map says cdecl(void), but the routine passes its ecx (the
#   First Division, set by start_new_season: mov ecx, esi) as this to 0x6830b0 and 0x6831a0: thiscall, ret 0. The
#   relegation loop is bounded by the north group's club count for both groups (as the original; both have 16).
# Conventions: 0x944e46 (cdecl), 0x4afbd0 (3 pushes; add esp, 0xc -> cdecl), 0x6830b0 (push 1; push record; push club
# -> thiscall, ret 0xc), 0x6826d0 (push i -> thiscall, ret 4), 0x6831a0 (push 1; push record; push club -> thiscall,
# ret 0xc), 0x9452ca (cdecl).
# Comparers: the next stage side by side on +0x30 (ita_ser_c1a_compare_advance_stage; only the original when the
# play-off is built); the swap only the original (it draws), the relegated clubs checked in the lower division.
# Equal in cm0102-1080.exe.
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('cro_a1_advance_stage', 0x514bd0, [0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30]),
           ('cro_a2b_advance_stage', 0x517180, [0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30]),
           ('cro_a1_swap_second_and_third_tier_clubs', 0x514ed0, [0x81, 0xec, 0x0c, 0x02, 0x00, 0x00])]
DOCS = ('as ligas croatas: a fase seguinte e a troca com a terceira divisão', 'the Croatian leagues: the next stage and the third tier swap',
        [('cro_a1_advance_stage', 'CroA1::advance_stage', '0x514bd0', 'cro_a1.cpp', 'thiscall, ret 0',
          'regra: a fase seguinte da Primeira Divisão croata: na última fase, a caixa de erro; senão a fase atual sobe e, ao chegar a 0 (fim da liga), monta o jogo-desempate',
          'rule: the Croatian First Division next stage: at the last stage, the error box; otherwise the current stage goes up and, on reaching 0 (the end of the league), the tie-break play-off is built'),
         ('cro_a2b_advance_stage', 'CroA2b::advance_stage', '0x517180', 'cro_a2b.cpp', 'thiscall, ret 0',
          'regra: a fase seguinte do grupo sul da Segunda Divisão croata, como a da Primeira',
          "rule: the Croatian Second Division south group next stage, as the First Division's"),
         ('cro_a1_swap_second_and_third_tier_clubs', 'cro_a1_swap_second_and_third_tier_clubs', '0x514ed0', 'cro_a1.cpp', 'thiscall (ecx = a Primeira Divisão), ret 0',
          'regra: na virada croata, os quatro melhores clubes croatas da divisão inferior e dos quatro grupos da terceira divisão (ordenados por força, embaralhados entre os melhores) sobem à Segunda Divisão (grupo norte; o reagrupamento seguinte os divide), e os rebaixados (situação 3) dos dois grupos da Segunda descem à divisão inferior; com menos de 4 candidatos ninguém sobe nem desce',
          'rule: at the Croatian turnover, the four best Croatian clubs of the lower division and the four third division groups (sorted by strength, the best shuffled) go up into the Second Division (north group; the regrouping that follows splits them), and the relegated clubs (status 3) of both second division groups go down to the lower division; with fewer than 4 candidates nobody moves')])
CAMPOS = [('liga +0x2c, +0x30; clube +0x53 nação, +0x57 divisão; linha da tabela +0x37 situação', 'league +0x2c, +0x30; club +0x53 nation, +0x57 division; table row +0x37 status')]
