# As fases das eliminatórias africanas: andar de fase, os grupos e a repescagem (wc_african_cup.cpp; leitor rx11, rodada
# 6, 25/11/2026). Simulação na campanha de 2006 (os grupos africanos de 2004 e 2005 na carreira de teste, que começa em
# 2001): LIGAS = 'ENG' e ATE = '2005-12-31', como os cx16_7..12.
# - 0x910920 WcAfricanCup::advance_stage thiscall ret 0 (virtual +0x28): na última fase ou além (+0x2c - 1, com sinal)
#   caixa 0x417; senão +0x30 + 1 e, pela fase nova, 0x9109e0 (0) ou 0x910d20 (6 com formato +0xba 2, 4 com formato 3),
#   chamadas diretas. Prólogo mov edx, [ecx+0x2c]; mov eax, [ecx+0x30] (6 bytes).
# - 0x9109e0 WcAfricanCup::create_group_stages thiscall ret 0 (quadro SEH só para o new): grupos x tamanho pelo formato
#   (0: 5x5; 1 e 3: 4x6; 2: 6x4; outro: contagens da pilha não iniciadas, DEFEITOS); malloc(4 x tamanho) (caixa 0x45e);
#   os vencedores da primeira fase (linhas da tabela de times +0xa7 com situação 1) nas vagas vazias da lista +0xb2
#   (+0xb6 vagas); qsort da lista com 0x4b07d0 (reputação); por grupo g: a tabela de rodadas (virtual +0x3c(g, &rodadas,
#   &rótulo, 0)), os times list[g + k x grupos], new 0xee e League_Stage 0x68b100 (ret 0x58: a seção, tamanho, os
#   times, 2, o registro, a tabela, rodadas, 3, 1, 0xa, {1,2,3,0}, {1,0,0,0}, o ano, g, o rótulo, 0xf, 4, 0, 0x24 ou 0x28
#   pelo formato, -1, 0, 2) em +0xc[g]; nulo: caixa 0x48b e fim (sem liberar); +0x30 = g, free(tabela), a notícia do
#   sorteio 0x684230. Por fim free(times). Prólogo push -1; push imm32 (7 bytes).
# - 0x910d20 WcAfricanCup::create_playoff_stage thiscall ret 0: n = 6 com formato 2, senão 4; malloc(4n) (caixa
#   0x4b3); os primeiros (linha 0 da tabela +0xb1) das fases 0..n-1; a tabela de rodadas (virtual +0x3c(n, ...)); new 0xb2
#   e Cup_Stage 0x522e00 (ret 0x3c: a seção, n, os times, rodadas, o registro, a tabela, o ano, n, 1, o rótulo, 0x14, 0,
#   0, 0, 0) em +0xc[n]; free dos dois; nulo: caixa 0x4ca; senão Cup::set_up_round 0x51c800 (push 0). Prólogo push -1;
#   push imm32 (7 bytes).
# Iguais no cm0102-1080.exe. Comparadores: só a original (montam fases), com o plano: a fase anda uma (ou vai ao último
# grupo); os grupos com os potes da lista ordenada; a repescagem com os primeiros dos grupos na ordem.
SAVE = False
LIGAS = 'ENG'
ATE = '2005-12-31'
ROTINAS = [('wc_african_cup_advance_stage', 0x910920, [0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30]),
           ('wc_african_cup_create_group_stages', 0x9109e0, [0x6a, 0xff, 0x68, 0xbe, 0x56, 0x96, 0x00]),
           ('wc_african_cup_create_playoff_stage', 0x910d20, [0x6a, 0xff, 0x68, 0xde, 0x56, 0x96, 0x00])]
DOCS = ('as fases das eliminatórias africanas: andar de fase, os grupos e a repescagem',
        'the African qualifying stages: moving on, the groups and the play-off',
        [('wc_african_cup_advance_stage', 'WcAfricanCup::advance_stage', '0x910920', 'wc_african_cup.cpp', 'thiscall, ret 0',
          'regra: as eliminatórias africanas andam uma fase por vez: da primeira fase à fase 0 monta todos os grupos; a repescagem vem na fase 6 (formato 2) ou 4 (formato 3); passar da última é erro',
          'rule: the African qualifying moves on one stage at a time: from the first round to stage 0 it builds all the groups; the play-off comes at stage 6 (format 2) or 4 (format 3); past the last stage it is an error'),
         ('wc_african_cup_create_group_stages', 'WcAfricanCup::create_group_stages', '0x9109e0', 'wc_african_cup.cpp', 'thiscall, ret 0',
          'regra: os grupos africanos são sorteados por potes: os vencedores da primeira fase entram na lista dos cabeças, a lista é ordenada pela reputação e o grupo g fica com os inscritos g, g + grupos, g + 2 x grupos...; cada grupo é uma liga de turno e returno (3 pontos a vitória, 1 o empate), com a notícia do sorteio',
          'rule: the African groups are drawn by pots: the first round winners join the seeded list, the list is ordered by reputation and group g takes entrants g, g + groups, g + 2 x groups...; each group is a round robin (3 points a win, 1 a draw), with its draw news'),
         ('wc_african_cup_create_playoff_stage', 'WcAfricanCup::create_playoff_stage', '0x910d20', 'wc_african_cup.cpp', 'thiscall, ret 0',
          'regra: a repescagem africana é uma fase de copa entre os primeiros dos grupos (quatro, ou seis no formato 2)',
          'rule: the African play-off is a knockout cup stage between the group winners (four, or six with format 2)')])
CAMPOS = []
INTERESSA = []
