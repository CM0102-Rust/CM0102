# O calendario da Football Conference (eng_conf.cpp; leitor rx10, rodada 6, 25/11/2026). Carreira ENG com a base
# NORMAL (a Conference so existe com a divisao extra carregada; ver rx10_1). Roda pelas regras (virtual +0x3c, fase -1)
# na criacao do jogo; a fase 0 (o mata-mata) nenhuma regra da Conference pede (sem vagas de mata-mata).
# - 0x56de60 EngConf::create_schedule thiscall(fase, &rodadas, &extra, &liga) ret 0x10: fase -1: 42 rodadas (malloc
#   0xaaa, caixa 0xb1), o molde eng_second_set_christmas_rounds (21 rodadas, o caso de Natal pelo dia da rodada 20
#   contra 1/dez menos 16, tabela de salto 0x56ec94, caixa 0x13d no padrao, 19 rodadas); fase 0: 3 rodadas de copa
#   (malloc 0x138, caixa 0x19f, extra 0xa0, liga 0) pelo eng_cc_cup_write_cup_round; outras: 0. As rodadas tiradas
#   por leitura estatica (os push ate cada 0x68a160/0x68a1c0) e conferidas por emulacao (unicorn) no caso padrao: 42
#   de 42 iguais. Prologo mov al, [esp+4]; sub esp, 0x208 (10 bytes).
# Iguais no cm0102-1080.exe. Comparador: lado a lado (compare_league_round_table).
SAVE = False
BASE = 'NORMAL'
ROTINAS = [('eng_conf_create_schedule', 0x56de60, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x08, 0x02, 0x00, 0x00])]
DOCS = ('o calendário da Football Conference', 'the Football Conference schedule',
        [('eng_conf_create_schedule', 'EngConf::create_schedule', '0x56de60', 'eng_conf.cpp', 'thiscall, ret 0x10',
          'regra: a Conference tem 42 rodadas (22 clubes) de 15 de agosto a 1º de maio, aos sábados com rodadas no meio da semana; no Natal, duas rodadas (21 a 30 de dezembro) escolhidas pelo dia em que cai a rodada de 19 de dezembro; a fase 0 é um mata-mata de três rodadas em maio que as regras da Conference nunca pedem',
          'rule: the Conference has 42 rounds (22 clubs) from 15 August to 1 May, on Saturdays with midweek rounds; at Christmas, two rounds (21 to 30 December) chosen by the day the 19 December round falls on; stage 0 is a three-round play-off in May that the Conference rules never ask for')])
CAMPOS = []
INTERESSA = []
