# Os clubes, o grupo A e a fase seguinte da Copa da Liga coreana (kor_league_cup.cpp). Leitor mwf (12/11/2026), rodada
# 2 (o mundo até 2002-08-01; docs/cobertura_mundo.txt: 2 vezes cada, no construtor no começo do jogo e na virada).
# - 0x66cf20 (KorLeagueCup::select_teams): os clubes da K League em ordem da tabela de clubes, ordenados pela posição
#   da última temporada (0x4b0780); a lista tem dez lugares e não confere o limite (um 11o clube escreve além dela:
#   defeito [inferência]); com menos de dez, a caixa (e segue).
# - 0x66c770 (KorLeagueCup::create_group_a_table): a própria copa é o grupo A, com as cabeças pares (lugares 0, 2, 4,
#   6, 8), cada uma tirada da lista; depois a notícia do sorteio.
# - 0x66cad0 (KorLeagueCup::advance_stage): a fase seguinte; ao sair da fase 0 para a 1 monta o mata-mata (0x66cd50).
# Convenções: mov ecx, esi; call -> thiscall, ret 0 (as três; a advance_stage é a virtual +0x28); dentro: 0x945501
# (push 10; push 4 -> cdecl), 0x945333 (qsort, 4 pushes -> cdecl), 0x944e46 (cdecl), 0x687430 (4 pushes -> thiscall,
# ret 0x10), 0x684230 (thiscall, ret 0).
# Comparadores: os três só a original, com o plano (a lista ordenada pelo mesmo comparador; as cabeças pares nas linhas
# em ordem e os lugares zerados; a fase igual na última ou +1).
LIGAS = 'ENG KOR ARG ITA FRA ESP AUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('kor_league_cup_select_teams', 0x66cf20, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('kor_league_cup_create_group_a_table', 0x66c770, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('kor_league_cup_advance_stage', 0x66cad0, [0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30])]
DOCS = ('os clubes, o grupo A e a fase seguinte da Copa da Liga coreana',
        'the Korean League Cup clubs, group A and next stage',
        [('kor_league_cup_select_teams', 'KorLeagueCup::select_teams', '0x66cf20', 'kor_league_cup.cpp', 'thiscall, ret 0',
          'regra: a Copa da Liga coreana tem os dez clubes da K League, ordenados pela posição da última temporada',
          "rule: the Korean League Cup has the ten K League clubs, sorted by last season's position"),
         ('kor_league_cup_create_group_a_table', 'KorLeagueCup::create_group_a_table', '0x66c770', 'kor_league_cup.cpp', 'thiscall, ret 0',
          'regra: o grupo A da Copa da Liga coreana (a própria copa) tem as cinco cabeças pares (1a, 3a, 5a, 7a e 9a da ordem)',
          'rule: Korean League Cup group A (the cup itself) has the five even-numbered seeds (1st, 3rd, 5th, 7th and 9th in the order)'),
         ('kor_league_cup_advance_stage', 'KorLeagueCup::advance_stage', '0x66cad0', 'kor_league_cup.cpp', 'thiscall, ret 0',
          'regra: depois dos grupos a Copa da Liga coreana passa ao mata-mata (montado ao sair da fase 0); passar da última fase é erro',
          'rule: after the groups the Korean League Cup goes on to the knockout (built when leaving stage 0); going past the last stage is an error')])
CAMPOS = [('copa coreana +0xee (lista dos dez clubes), +0xf2 (lugares); liga +0x3e, +0xb1, +0x30; valor/ponteiro; grava',
           'Korean cup +0xee (list of the ten clubs), +0xf2 (places); league +0x3e, +0xb1, +0x30; value/pointer; writes')]
