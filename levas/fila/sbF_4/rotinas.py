# O calendario e os registros de carreira da Copa da Liga coreana (kor_league_cup.cpp; leitor sbF, rodada 5,
# 24/11/2026). Competicoes da Coreia; ATE 2002-08-01 (a copa e montada na criacao e jogada de marco a maio de 2002).
# - 0x66c240 KorLeagueCup::create_schedule thiscall(fase, &rodadas, &extra, &e_liga) ret 0x10: o molde da ItaCup
#   (create_schedule) com o mata-mata na fase 1, o rotulo 0x420, as caixas 0xbf e 0xfa; conferido por emulacao
#   (unicorn) nas fases -128..127, ano impar e par: as mesmas chamadas e os mesmos 16 campos por rodada.
# - 0x66d090 KorLeagueCup::update_staff_records_for_status thiscall(clube, desfecho, fase, _, partida, final) ret 0x18
#   (a virtual 0x44): o corpo da GreCup/ItaCup (conferido instrucao a instrucao contra a GreCup 0x5e9ca0) com o
#   mata-mata na fase 1, a rodada de classificacao 0x82 e 2 rodadas no peso (a ItaCup tem 5 fixo no
#   italian_cup_staff_records, a Welsh 0x78 fixo no welsh_cup_staff_records: por isso o group_cup_staff_records novo,
#   pub(crate); ao integrar, os dois podem passar a chama-lo).
# Prologos: mov al, [esp+4]; sub esp, 0x200 (10 bytes); mov al, [esp+0xc]; push esi (5 bytes). Iguais no 1080.
# Comparadores: o calendario lado a lado; os registros so a original (os historicos nao voltam a simulacao antes da
# data: sem mutacao propria, como no cx02_19).
LIGAS = 'ENG KOR ARG ITA FRA ESP AUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('kor_league_cup_create_schedule', 0x66c240, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('kor_league_cup_update_staff_records_for_status', 0x66d090, [0x8a, 0x44, 0x24, 0x0c, 0x56])]
DOCS = ('o calendário e os registros de carreira da Copa da Liga coreana',
        'the Korean League Cup schedule and staff records',
        [('kor_league_cup_create_schedule', 'KorLeagueCup::create_schedule', '0x66c240', 'kor_league_cup.cpp', 'thiscall, ret 0x10',
          'regra: os dois grupos de cinco da Copa da Liga coreana (o grupo A da própria copa e a fase 0, o grupo B) têm dez rodadas de 25 de março a 28 de abril, cada grupo uma fase de liga (é_liga = 1, rótulo 0x420 + fase); o mata-mata (fase 1): semifinais de jogo único em 5 de maio e a final em ida e volta (4 dias) a partir de 9 de maio; categoria 6, sem prêmio por rodada',
          'rule: the Korean League Cup two groups of five (the cup\'s own group A and stage 0, group B) play ten matchdays from 25 March to 28 April, each group a league stage (is_league = 1, label 0x420 + stage); the knockout (stage 1): single-match semi-finals on 5 May and a two-legged final (4 days apart) from 9 May; category 6, no prize per round'),
         ('kor_league_cup_update_staff_records_for_status', 'KorLeagueCup::update_staff_records_for_status', '0x66d090', 'kor_league_cup.cpp', 'thiscall, ret 0x18',
          'regra: na Copa da Liga coreana o técnico cujo clube passa do grupo ganha um registro de classificação (rodada 0x82, as semifinais) e o que cai, um de fracasso; no mata-mata (fase 1), a eliminação ou a classificação com o peso da rodada (de duas), o título ou o vice',
          'rule: in the Korean League Cup a manager whose club goes through its group gets a qualification record (round 0x82, the semi-finals) and one whose club goes out a failure record; in the knockout (stage 1), an elimination or a qualification weighted by the round (of two), the cup win or the runner-up place')])
CAMPOS = []
INTERESSA = []
