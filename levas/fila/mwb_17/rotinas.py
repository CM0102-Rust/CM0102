# A virada da Copa da Liga escocesa (sco_lge_cup.cpp). Leitor mwb (12/11/2026), lista do mundo (Escócia e País de
# Gales). Só usa o que já está no motor.
# - 0x7f0930 (ScoLgeCup::start_new_season): o molde da virada das copas, mas volta a 40 vagas vazias (calloc) em vez de
#   chamar os inscritos (vêm na atualização do dia, mwb_16), zera o +0x45 e não refaz as posições previstas (devolve o
#   eax da árvore de confrontos). Roda uma vez, na virada escocesa de 2002; a 1ª rodada da edição nova é em 31/7/2002.
# Convenções conferidas: thiscall, ret 0 (0x9452ca e 0x945501 cdecl; 0x4a1c50 push 1 -> ret 4; as virtuais +0x8c e
# +0x94 thiscall, ret 0).
# Comparador: só a original (libera e realoca); o ano subiu um.
LIGAS = 'ENG SCO WAL'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('sco_lge_cup_start_new_season', 0x7f0930, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a virada da Copa da Liga escocesa',
        'the Scottish League Cup turnover',
        [('sco_lge_cup_start_new_season', 'ScoLgeCup::start_new_season', '0x7f0930', 'sco_lge_cup.cpp', 'thiscall, ret 0',
          'regra: na virada a Copa da Liga escocesa libera os times, as rodadas e os confrontos da edição passada, arquiva as estatísticas, passa ao ano seguinte e volta a 40 vagas vazias (os inscritos vêm quando a temporada começa); refaz as rodadas e a árvore de confrontos',
          "rule: at the turnover the Scottish League Cup frees last edition's teams, rounds and ties, archives the statistics, moves to the next year and goes back to 40 empty places (the entrants come when the season starts); it rebuilds the rounds and the fixture tree")])
