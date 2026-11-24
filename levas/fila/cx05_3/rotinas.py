# As fases da Copa da Asia: os grupos B e C, o mata-mata e a passagem de fase (asia_nations.cpp; leitor cx05, rodada
# 5, 24/11/2026). Simulacao com as 8 ligas e ATE 2004-11-01 (os grupos B e C nascem no sorteio de 5 de junho de 2004,
# o mata-mata depois do grupo, em outubro de 2004; 0 chamadas na cobertura do mundo ate 2002-08-01).
# - 0x40f8a0 AsiaNations::create_group_stage thiscall(g) ret 4 (do dia do sorteio, g = 0 e 1): o molde da Copa America
#   (League_Stage::League_Stage 0x68b100, thiscall, ret 0x58, 22 argumentos; a virtual +0x3c: 4 pushes; mov ecx ->
#   thiscall, ret 0x10) com as sorteadas 4(g+1)..4(g+1)+3 (a lista guarda os grupos em sequencia). Prologo mov eax,
#   fs:[0] (6 bytes).
# - 0x40fa90 AsiaNations::create_knockout_stage thiscall() ret 0 (chamada pelo advance_stage ao chegar a fase 2): a
#   fase temporaria das terceiras (League_Stage, 3 times; League::sort_table 0x6827d0 push 0 -> thiscall, ret 4;
#   League::get_table_row 0x6826d0 push k -> thiscall, ret 4), o vetor das 8, Cup_Stage::Cup_Stage 0x522e00 (thiscall,
#   ret 0x3c, 15 argumentos), o destrutor da temporaria (virtual 0 com 1) e Cup::set_up_round 0x51c800 (push 0 ->
#   thiscall, ret 4). Prologo push -1; push imm32 (7 bytes).
# - 0x40f810 AsiaNations::advance_stage thiscall() ret 0 (virtual +0x28): con_merc_cup_advance_stage_cqs11_next_stage
#   com o mata-mata em 2 (0x40fa90 pelo endereco: o desvio leva a copia). Prologo mov edx, [ecx+0x2c]; mov eax,
#   [ecx+0x30] (6 bytes).
# Defeito do original: o create_knockout_stage nao confere o new da fase temporaria das terceiras (le a tabela dela
# mesmo nula: o jogo cai sem memoria); a copia faz o mesmo.
# Iguais no cm0102-1080.exe (as tres inteiras).
# Comparadores: so a original nas tres (alocam, sorteiam a primeira rodada), com o plano: a fase do grupo com as
# quatro sorteadas; o mata-mata em [+0xc][2] e +0x30 = 2; +0x30 sobe um.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ATE = '2004-11-01'
ROTINAS = [('asia_nations_create_group_stage', 0x40f8a0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('asia_nations_create_knockout_stage', 0x40fa90, [0x6a, 0xff, 0x68, 0x3c, 0x39, 0x95, 0x00]),
           ('asia_nations_advance_stage', 0x40f810, [0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30])]
DOCS = ('as fases da Copa da Ásia', 'the Asian Cup stages',
        [('asia_nations_create_group_stage', 'AsiaNations::create_group_stage', '0x40f8a0', 'asia_nations.cpp', 'thiscall, ret 4',
          'regra: os grupos B e C da Copa da Ásia são fases de liga com as quatro sorteadas de cada um, as regras e o ano da copa e o calendário do grupo; os técnicos recebem a notícia do sorteio',
          'rule: groups B and C of the Asian Cup are league stages with each group\'s four drawn nations, the cup\'s rules and year and the group\'s round table; the managers get the draw news'),
         ('asia_nations_create_knockout_stage', 'AsiaNations::create_knockout_stage', '0x40fa90', 'asia_nations.cpp', 'thiscall, ret 0',
          'regra: as quartas-de-final da Copa da Ásia têm os três primeiros e os três segundos dos grupos e as duas melhores terceiras (as três terceiras ranqueadas numa tabela à parte); na ordem 1º A, melhor 3ª, 1º B, 2ª melhor 3ª, 1º C, 2º A, 2º B, 2º C (os confrontos em pares nessa ordem [inferência]), em jogo único',
          'rule: the Asian Cup quarter-finals have the three group winners, the three runners-up and the two best thirds (the three thirds ranked in a separate table); in the order A winner, best third, B winner, second third, C winner, A, B and C runners-up (the ties in pairs in that order [inference]), single matches'),
         ('asia_nations_advance_stage', 'AsiaNations::advance_stage', '0x40f810', 'asia_nations.cpp', 'thiscall, ret 0',
          'regra: a Copa da Ásia passa de fase quando a atual acaba; ao chegar à fase 2 (depois dos três grupos) monta o mata-mata; passar da última fase é erro',
          'rule: the Asian Cup moves on one stage when the current one ends; on reaching stage 2 (after the three groups) it builds the knockout; going past the last stage is an error')])
CAMPOS = []
INTERESSA = []
