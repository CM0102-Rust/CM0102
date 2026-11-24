# As fases das eliminatórias da CONCACAF e a repescagem delas, e a seleção europeia que joga a repescagem contra a Ásia
# (wc_concacaf_cup.cpp, wc_europe_league.cpp; leitor cx16, rodada 5, 24/11/2026). Simulação na campanha de 2004 (a
# repescagem da CONCACAF em maio de 2004; a europeia em novembro de 2005): ATE = '2005-12-31' (o pedido da rodada 5).
# - 0x917060 WcConcacafCup::advance_stage thiscall ret 0 (virtual +0x28): na última fase ou além (+0x2c - 1, com sinal),
#   caixa 0x5d7; senão +0x30 + 1 e, pela fase nova, 0x917110 (2), 0x917320 (3) ou 0x917830 (6), chamadas diretas.
#   Prólogo mov edx, [ecx+0x2c]; mov eax, [ecx+0x30] (6 bytes).
# - 0x917110 WcConcacafCup::create_play_off_stage thiscall ret 0 (quadro SEH só para o new): vetor de 6 (0x18; null:
#   caixa 0x5ff); a primeira cabeça de chave que sobra nas vagas 25..34 da lista (a vaga zerada), as linhas da tabela de
#   times da copa com situação 2 (os sobreviventes do mata-mata caribenho) e a linha 1 das tabelas das fases 0 e 1 (os
#   segundos dos grupos da América Central); a tabela de rodadas pela virtual +0x3c (2, &rodadas, &rótulo, 0); operator
#   new 0xb2 e Cup_Stage::Cup_Stage 0x522e00 (ret 0x3c: a seção, 6, os times, as rodadas, o registro, a tabela, o ano, 2,
#   4, o rótulo, +0x51, 0, 0, 0, 0) em +0xc[2]; o vetor e a tabela liberados; sem fase, caixa 0x629; com ela +0x30 = 2 e
#   Cup::set_up_round 0x51c800 (push 0). Prólogo mov eax, fs:[0] (6 bytes).
# - 0x91def0 WcEuropeLeague::get_asia_play_off_team thiscall ret 0: a primeira linha da tabela da fase dos segundos
#   (+0xc[+0x100]) cujo clube não está na tabela de times da repescagem (+0xc[+0x101], linhas de 6); nenhuma, 0. Prólogo
#   push ecx; movsx edx, byte [ecx+0x100] (8 bytes).
# Iguais no cm0102-1080.exe. Comparadores: as fases só a original contra o plano (a fase anda uma; a repescagem com as
# seis seleções do plano na ordem); a seleção da Ásia lado a lado (só lê).
SAVE = False
LIGAS = 'ENG'
ATE = '2005-12-31'
ROTINAS = [('wc_concacaf_cup_advance_stage', 0x917060, [0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30]),
           ('wc_concacaf_cup_create_play_off_stage', 0x917110, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('wc_europe_league_get_asia_play_off_team', 0x91def0, [0x51, 0x0f, 0xbe, 0x91, 0x00, 0x01, 0x00, 0x00])]
DOCS = ('as fases e a repescagem das eliminatórias da CONCACAF e a seleção europeia da repescagem contra a Ásia',
        'the CONCACAF qualifying stages and play-off and the European nation for the Asian play-off',
        [('wc_concacaf_cup_advance_stage', 'WcConcacafCup::advance_stage', '0x917060', 'wc_concacaf_cup.cpp', 'thiscall, ret 0',
          'regra: as eliminatórias da CONCACAF andam uma fase por vez: na fase 2 a repescagem, na 3 os três grupos semifinais (as fases 4 e 5 são os outros grupos), na 6 o grupo final; passar da última é erro',
          'rule: the CONCACAF qualifying moves on one stage at a time: to stage 2 the play-off, to 3 the three semi-final groups (stages 4 and 5 are the other groups), to 6 the final group; past the last stage it is an error'),
         ('wc_concacaf_cup_create_play_off_stage', 'WcConcacafCup::create_play_off_stage', '0x917110', 'wc_concacaf_cup.cpp', 'thiscall, ret 0',
          'regra: a repescagem da CONCACAF é uma fase de copa de seis: a próxima cabeça de chave, os sobreviventes do mata-mata caribenho e os segundos dos dois grupos da América Central',
          'rule: the CONCACAF play-off is a cup stage of six: the next seeded nation, the Caribbean knockout survivors and the two Central American group runners-up'),
         ('wc_europe_league_get_asia_play_off_team', 'WcEuropeLeague::get_asia_play_off_team', '0x91def0', 'wc_europe_league.cpp', 'thiscall, ret 0',
          'regra: a seleção europeia que joga a repescagem contra a Ásia é o segundo colocado mais bem classificado que não está na repescagem europeia',
          'rule: the European nation that plays the Asian section is the best-ranked group runner-up who is not in the European play-off')])
CAMPOS = []
INTERESSA = []
