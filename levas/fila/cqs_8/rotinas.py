# As eliminatórias da Eurocopa: as regras e a data do sorteio, o tamanho de cada grupo, a situação de uma seleção e a
# tabela do grupo 1 (euro_champ_qual.cpp, reservado ao Agente D). Frente das competições (Agente D, 06/11/2026),
# carreira de 8 ligas (começa em janeiro de 2001).
# - 0x57e8f0 (EuroChampQual::set_up_rules, 224 bytes; 2001-000): as regras e a data do sorteio.
# - 0x57d9d0 (EuroChampQual::get_group_team_count, 216 bytes; 63 chamadas, no sorteio de 2002-067).
# - 0x580b50 (EuroChampQual::get_qualification_status, 36 bytes; 36 chamadas em 2002-067).
# - 0x57e7e0 (EuroChampQual::init_group_one_table, 261 bytes; 2002-067).
# Iguais no cm0102-1080.exe (as quatro inteiras).
# Convenções conferidas (lição da 258a): 0x57e8f0 e 0x57e7e0 thiscall, ret 0; 0x57d9d0 e 0x580b50 thiscall, ret 4.
# Dentro: Date 0x549f70 (push 5; push ano; push 2; push 0xa; lea ecx, local -> thiscall, ret 0x10); a virtual +0x3c
# (push 0; push &+0x3a; push &+0xa9; push -1; mov ecx -> thiscall, ret 0x10); Comp_Stats::get_club_stat 0x4a2e10 (push
# 0x12; push clube; mov ecx, [this+8] -> thiscall, ret 8, float em st0) e _ftol 0x945640 (o ftol do motor.rs); 0x57d9d0
# (push -1; mov ecx -> thiscall, ret 4); 0x944e46 (push; add esp -> cdecl); League::init_table_row 0x687430 (push 0;
# push 0; push clube; push índice; mov ecx -> thiscall, ret 0x10: só o byte do índice é lido); 0x684230 (thiscall, ret
# 0); a caixa (aviso_em).
# Comparadores: as regras lado a lado (os campos +0x3a..+0xc9; a tabela das rodadas, montada pela virtual original com
# sobras do malloc, só conferida como montada, e a da cópia liberada; na comparação de 06/11 os bytes dela diferiam); o tamanho e a situação pela devolução (o formato desconhecido, com a caixa, só a original); o grupo 1 só a
# original (aloca e manda a notícia), com o plano feito antes.
# Determinismo: nada lê pilha, heap ou endereço como valor (os lixos dos dwords empurrados não são lidos).
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqs8_eurocopa_regras', 0x57e8f0, [0x6a, 0xff, 0x68, 0xa8, 0x88, 0x95, 0x00]),
           ('cqs8_eurocopa_tamanho_do_grupo', 0x57d9d0, [0x0f, 0xbe, 0x81, 0xf6, 0x00, 0x00, 0x00]),
           ('cqs8_eurocopa_situacao', 0x580b50, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08]),
           ('cqs8_eurocopa_grupo_um', 0x57e7e0, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00])]
DOCS = ('as eliminatórias da Eurocopa: as regras e a data do sorteio, o tamanho dos grupos, a situação de uma seleção e o grupo 1',
        'the European Championship qualifiers: the rules and the draw date, the group sizes, a team\'s status and group 1',
        [('cqs8_eurocopa_regras', 'EuroChampQual::set_up_rules', '0x57e8f0', 'euro_champ_qual.cpp', 'thiscall, ret 0',
          'regra: as eliminatórias da Eurocopa são grupos em ida e volta (2 turnos), 3 pontos pela vitória e 1 pelo empate, desempates 1, 2, 4, 5, o primeiro de cada grupo vai direto e o segundo ao play-off, ninguém cai; o sorteio é em 10 de março do ano da copa, levado à sexta-feira [inferência]',
          'rule: the European Championship qualifiers are home-and-away groups (2 rounds), 3 points per win and 1 per draw, tie-breakers 1, 2, 4, 5, each group winner goes straight through and the runner-up to the play-off, nobody is relegated; the draw is on 10 March of the cup year, moved to the Friday [inference]'),
         ('cqs8_eurocopa_tamanho_do_grupo', 'EuroChampQual::get_group_team_count', '0x57d9d0', 'euro_champ_qual.cpp', 'thiscall, ret 4',
          'regra: no formato 1 os grupos 1, 3 e 6 têm 6 seleções e os outros 5; no formato 2 os grupos 1, 3, 6 e 8 têm 6 e os outros 5',
          'rule: in format 1 groups 1, 3 and 6 have 6 teams and the others 5; in format 2 groups 1, 3, 6 and 8 have 6 and the others 5'),
         ('cqs8_eurocopa_situacao', 'EuroChampQual::get_qualification_status', '0x580b50', 'euro_champ_qual.cpp', 'thiscall, ret 4',
          'regra: a situação de uma seleção nas eliminatórias: −2 quando a estatística 0x12 dela na copa é menor que 14 (a posição [inferência]), senão −3',
          'rule: a team\'s status in the qualifiers: −2 when its stat 0x12 in the cup is below 14 (the position [inference]), else −3'),
         ('cqs8_eurocopa_grupo_um', 'EuroChampQual::init_group_one_table', '0x57e7e0', 'euro_champ_qual.cpp', 'thiscall, ret 0',
          'regra: o grupo 1 das eliminatórias é a própria copa, com 5 seleções (as primeiras da lista sorteada), (5 − 1) × 2 rodadas em ida e volta, e a notícia do grupo',
          'rule: qualifier group 1 is the cup object itself, with 5 teams (the first of the drawn list), (5 − 1) × 2 home-and-away rounds, and the group news')])
CAMPOS = [('copa +0xc2..+0xc8 pontos e desempates, +0x3c turnos, +0x43, +0x49, +0x4a, +0xbe..+0xc1 vagas; valor; grava (regras)',
           'cup +0xc2..+0xc8 points and tie-breakers, +0x3c rounds, +0x43, +0x49, +0x4a, +0xbe..+0xc1 places; value; writes (rules)'),
          ('copa +0xb6 dia do sorteio, +0xb8; valor; grava (regras)', 'cup +0xb6 draw day, +0xb8; value; writes (rules)'),
          ('copa +0xba tabela das rodadas, +0xa9 rodadas, +0x3a id; ponteiro/valor; grava (regras)', 'cup +0xba round table, +0xa9 rounds, +0x3a id; pointer/value; writes (rules)'),
          ('copa +0xf6 formato (byte com sinal); valor; lê (tamanho do grupo)', 'cup +0xf6 format (signed byte); value; reads (group size)'),
          ('copa +0xf7 seleções sorteadas (vetor); ponteiro; lê (grupo 1). +0x3e, +0xa7, +0xb1; grava (grupo 1)', 'cup +0xf7 drawn teams (vector); pointer; reads (group 1). +0x3e, +0xa7, +0xb1; writes (group 1)')]
INTERESSA = []
