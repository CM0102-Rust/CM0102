# A Série C (bra_nat_third.cpp; agente C da frente das competições, 06/11/2026): os sete grupos da primeira fase, a
# segunda fase dos dezesseis e os dois grupos da terceira. A Série C tem 56 clubes: oito grupos de sete (a própria
# liga é o primeiro, e as fases 0 a 6 os outros sete), a segunda fase (fase 7, mata-mata dos dezesseis), a terceira
# (fases 8 e 9, dois grupos de quatro) e a final (fase 10, cqs_4). Na carreira de 8 ligas até 2002-06-01
# (docs/cobertura_competicoes.txt), cada grupo roda duas vezes (na criação da carreira e na virada para 2002) e a
# segunda e a terceira fases uma vez (a Série C de 2001).
# - 0x42ee80 BraNatThird::create_group_stage(g): um grupo da primeira fase.
# - 0x42e430 BraNatThird::create_second_round_stage: os dois primeiros de cada grupo, semeados pelos pontos.
# - 0x42e700 BraNatThird::create_third_round_groups: os oito vencedores, embaralhados em dois grupos.
# Iguais no cm0102-1080.exe (inteiras). Convenções conferidas nos pontos de chamada da original:
# - o grupo, do construtor (0x42d4cf) e da virada (0x42e0c7): push g; mov ecx, esi; call -> thiscall, ret 4;
# - a segunda e a terceira fases, da fase seguinte da Série C: mov ecx, liga; call, sem push -> thiscall, ret 0;
# - a virtual +0x3c (create_round_table): push 0; push &extra; push &rodadas; push g, 7, 8 ou 9; mov ecx, esi ->
#   thiscall, ret 0x10;
# - 0x944e46 (malloc): push n; call; add esp, 4 -> cdecl; 0x944cf1 (operator new): push 0xee ou 0xb2; add esp, 4 -> cdecl;
# - 0x68b100 (League_Stage::League_Stage): 22 pushes; mov ecx, eax -> thiscall, ret 0x58 (o grupo: a liga, 7, o vetor,
#   2, +4, a tabela, as rodadas, +0xc2, +0xc3, +0xc4, &+0xc5, &+0xbe, o ano, g, o extra, +0x51, 1, 0, +0xd9, −1, 0, 2; a
#   terceira fase: a liga, 4, a lista, 2, +4, a tabela, as rodadas, 3, 1, 5, &{1,2,4,0}, &{2,0,0,0}, o ano, 8 ou 9, o
#   extra, 0x14, 1, 0, 2, −1, 0, 2; a original empurra words e bytes com lixo nos bytes altos: a cópia os passa limpos);
# - 0x522e00 (Cup_Stage::Cup_Stage): 15 pushes; mov ecx, eax -> thiscall, ret 0x3c (a liga, 16, a lista, as rodadas, +4,
#   a tabela, o ano, 7, 1, o extra, 0x14, 0, 0, 0, sementes nulas);
# - 0x4afb40 (comp_util_shuffle_pointer_array): push 8; push lista; call; add esp, 8 -> cdecl (já em Rust; embaralha 8);
# - 0x9452ca (free): push; call; add esp -> cdecl;
# - a caixa de erro (0x944cff, 0x90d130, 0x944c9f, 0x5e8290): aviso_em, como nas levas anteriores.
# Comparadores: só a original (alocam; a terceira fase sorteia), com o plano feito antes (o grupo: os sete clubes da
# lista dos 56 na tabela da fase; a segunda fase: a mesma escolha numa cópia dos registros, os dezesseis na fase 7; a
# terceira: os oito vencedores nas fases 8 e 9, como conjunto).
# Determinismo: as listas vêm das tabelas e da lista dos 56; nada de pilha, heap ou endereço como valor.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqs3_grupo_da_serie_c', 0x42ee80, [0x6a, 0xff, 0x68, 0xbe, 0x45, 0x95, 0x00]),
           ('cqs3_segunda_fase_da_serie_c', 0x42e430, [0x6a, 0xff, 0x68, 0x4e, 0x45, 0x95, 0x00]),
           ('cqs3_terceira_fase_da_serie_c', 0x42e700, [0x6a, 0xff, 0x68, 0x7c, 0x45, 0x95, 0x00])]
DOCS = ('a Série C: os sete grupos da primeira fase, a segunda fase dos dezesseis e os dois grupos da terceira',
        'Brazilian Series C: the seven first-round groups, the sixteen-club second round and the two third-round groups',
        [('cqs3_grupo_da_serie_c', 'BraNatThird::create_group_stage', '0x42ee80', 'bra_nat_third.cpp', 'thiscall, ret 4',
          'Um grupo da primeira fase da Série C: sete clubes da lista dos 56 (as sete primeiras vagas são o grupo da própria liga; o grupo g leva as vagas 7(g+1) a 7(g+1)+6), em pontos corridos de ida e volta, com os pontos, os critérios de desempate e as vagas da liga; os dois primeiros de cada um dos oito grupos vão à segunda fase',
          'A Series C first-round group: seven clubs of the 56-club list (the first seven slots are the league\'s own group; group g takes slots 7(g+1) to 7(g+1)+6), home and away round robin, with the league points, tie-breakers and places; the top two of each of the eight groups go to the second round'),
         ('cqs3_segunda_fase_da_serie_c', 'BraNatThird::create_second_round_stage', '0x42e430', 'bra_nat_third.cpp', 'thiscall, ret 0',
          'A segunda fase da Série C: os dezesseis (os dois primeiros de cada grupo) semeados pelos pontos da primeira fase (no empate, o que vem antes: a liga, depois os grupos 0 a 6, o primeiro antes do segundo), num mata-mata em que o melhor enfrenta o 16º, o 2º o 15º e assim por diante [inferência: os pares seguem a lista]',
          'The Series C second round: the sixteen (the top two of each group) seeded by first-round points (ties go to the earlier one: the league, then groups 0 to 6, first before second), in a knockout where the best meets the 16th, the 2nd the 15th and so on [inference: pairs follow the list]'),
         ('cqs3_terceira_fase_da_serie_c', 'BraNatThird::create_third_round_groups', '0x42e700', 'bra_nat_third.cpp', 'thiscall, ret 0',
          'A terceira fase da Série C: os oito vencedores da segunda fase, sorteados (embaralhados) em dois grupos de quatro, de ida e volta, com 3 pontos por vitória, 1 por empate, os critérios de desempate {1, 2, 4} e duas vagas de cima em cada grupo [inferência: os dois primeiros de cada grupo sobem à Série B e os campeões dos grupos fazem a final]',
          'The Series C third round: the eight second-round winners, drawn (shuffled) into two groups of four, home and away, with 3 points for a win, 1 for a draw, tie-breakers {1, 2, 4} and two top places in each group [inference: the top two of each group go up to Series B and the group winners play the final]')])
CAMPOS = [('liga +0xee: a lista dos 56 clubes da Série C (só no construtor e na virada); ponteiro; lê', 'league +0xee: the 56-club Series C list (only in the constructor and the turnover); pointer; reads'),
          ('liga +0xb1: a tabela (linhas de 0x3b bytes: +0 o clube, +0x19 os pontos); ponteiro; lê as duas primeiras linhas da liga e dos grupos', 'league +0xb1: the table (0x3b-byte rows: +0 the club, +0x19 the points); pointer; reads the top two rows of the league and the groups'),
          ('liga +0xc: as fases (vetor de ponteiros); ponteiro; grava as vagas 0 a 9', 'league +0xc: the stages (pointer array); pointer; writes slots 0 to 9'),
          ('liga +0x30: a fase atual; valor; grava g, 7 e 9', 'league +0x30: the current stage; value; writes g, 7 and 9'),
          ('fase de copa +0xa7: os times (registros de 6 bytes: +0 o clube, +5 a marca de vencedor); ponteiro; lê (terceira fase)', 'cup stage +0xa7: the teams (6-byte records: +0 the club, +5 the winner mark); pointer; reads (third round)'),
          ('liga +0xbe, +0xc2..+0xc5, +0xd9, +0x40, +0x51, +4: vagas, pontos, categoria, desempate, bits de regra, ano, importância, registro; valor; lê', 'league +0xbe, +0xc2..+0xc5, +0xd9, +0x40, +0x51, +4: places, points, category, tie-breakers, rule bits, year, importance, record; value; reads')]
INTERESSA = []
