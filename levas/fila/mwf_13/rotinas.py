# A virada e as rodadas do Auto Windscreens Shield (o troféu da Football League; eng_auto_cup.cpp, que com elas fica
# inteiro em Rust). Leitor mwf (12/11/2026), rodada 2 (o mundo até 2002-08-01; docs/cobertura_mundo.txt: cada uma 1
# vez, na virada inglesa de meados de 2002).
# - 0x56b4b0 (EngAutoCup::start_new_season): o molde das copas com o que o troféu tem a mais: libera também a lista
#   dos 56 escolhidos (+0xb2), destrói as fases das seções e grava +0x30 = −1; refaz os 56 (0x56b730), os
#   participantes (0x56b3c0), as rodadas, a árvore, a primeira fase de seção (0x56bab0), libera a lista de novo e
#   chama as posições previstas.
# - 0x56b700 (EngAutoCup::set_up_rounds, a virtual +0x8c): o prêmio de 400 mil e a tabela de rodadas.
# Convenções: virtuais (mov ecx, esi; call [eax+n], sem push) -> thiscall, ret 0; dentro: a virtual +0x3c (4 pushes
# -> thiscall, ret 0x10).
# Comparadores: a virada só a original (ano +1); as rodadas lado a lado como as da Copa do Brasil (o prêmio, as
# contagens e os bytes da tabela de rodadas; a tabela da cópia é liberada).
LIGAS = 'ENG KOR ARG ITA FRA ESP AUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('eng_auto_cup_start_new_season', 0x56b4b0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('eng_auto_cup_set_up_rounds', 0x56b700, [0x56, 0x8b, 0xf1, 0x6a, 0x00])]
DOCS = ('a virada e as rodadas do Auto Windscreens Shield',
        'the Auto Windscreens Shield season turnover and rounds',
        [('eng_auto_cup_start_new_season', 'EngAutoCup::start_new_season', '0x56b4b0', 'eng_auto_cup.cpp', 'thiscall, ret 0',
          'regra: a virada do Auto Windscreens Shield descarta as tabelas, os escolhidos e as fases das seções da edição anterior, arquiva a temporada, passa ao ano seguinte e escolhe de novo os 56 clubes, os participantes, as rodadas, a árvore e a primeira fase de seção; depois as posições previstas',
          "rule: the Auto Windscreens Shield turnover drops last edition's tables, selected clubs and section stages, archives the season, moves to the next year and selects the 56 clubs again, the entrants, the rounds, the tie tree and the first section stage; then the predicted positions"),
         ('eng_auto_cup_set_up_rounds', 'EngAutoCup::set_up_rounds', '0x56b700', 'eng_auto_cup.cpp', 'thiscall, ret 0',
          'regra: o Auto Windscreens Shield tem prêmio de 400 mil; depois a tabela de rodadas',
          'rule: the Auto Windscreens Shield has a prize fund of 400,000; then the round table')])
CAMPOS = [('copa +0x4c, +0xa3, +0xa7, +0xab, +0xad, +0xb1, +0xb2 (escolhidos), +0x30, +0x40 (a virada); +0x28 (prêmio), +0x3a, +0x3c; valor/ponteiro; grava',
           'cup +0x4c, +0xa3, +0xa7, +0xab, +0xad, +0xb1, +0xb2 (selected), +0x30, +0x40 (the turnover); +0x28 (prize fund), +0x3a, +0x3c; value/pointer; writes')]
