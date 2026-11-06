# A Copa da Itália e a Copa da Itália da Série C: a tabela do grupo 1 e a fase seguinte (ita_cup.cpp, ita_c_cup.cpp, em
# cups\). Frente das competições (Agente D, 06/11/2026), carreira de 8 ligas (começa em janeiro de 2001).
# - 0x63f250 (ItaCup::create_first_group_table, 245 bytes) e 0x63d5e0 (ItaCCup::init_first_group, 245 bytes): o mesmo
#   molde (4 ou 5 times), um ajudante comum (cqr7_grupo_um).
# - 0x63f5b0 (ItaCup::advance_stage, 144 bytes) e 0x63d950 (ItaCCup::advance_stage, 144 bytes): a virtual +0x28, o
#   mesmo molde (o mata-mata depois da fase 6 ou 14), um ajudante comum (cqr7_fase_seguinte).
# Iguais no cm0102-1080.exe (as quatro inteiras).
# Convenções conferidas (lição da 258a): as quatro thiscall, ret 0. Dentro: 0x944e46 (push; add esp, 4 -> cdecl);
# League::init_table_row 0x687430 (push 0; push 0; push clube; push índice; mov ecx, esi -> thiscall, ret 0x10: o
# índice vai no dword do byte local, com lixo em cima; a 0x687430 só lê o byte); League::send_group_draw_news 0x684230
# (mov ecx, esi -> thiscall, ret 0); os mata-matas 0x63f870 / 0x63dc10 (call direto com o ecx da copa -> thiscall, ret
# 0); a caixa (aviso_em).
# Comparadores: só a original (a 0x687430 aloca e a notícia vai aos humanos; o mata-mata aloca e sorteia), conferindo o
# plano feito antes: o grupo 1 com os primeiros inscritos nas linhas e as vagas zeradas; a fase +1 fora da última.
# Determinismo: nada lê pilha, heap ou endereço como valor (o lixo do dword empurrado não é lido).
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqr7_copa_da_italia_grupo_um', 0x63f250, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('cqr7_copa_da_serie_c_grupo_um', 0x63d5e0, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('cqr7_copa_da_italia_fase_seguinte', 0x63f5b0, [0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30]),
           ('cqr7_copa_da_serie_c_fase_seguinte', 0x63d950, [0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30])]
DOCS = ('a Copa da Itália e a da Série C: a tabela do grupo 1 e a fase seguinte',
        'the Italian Cup and the Serie C Cup: the group 1 table and the next stage',
        [('cqr7_copa_da_italia_grupo_um', 'ItaCup::create_first_group_table', '0x63f250', 'ita_cup.cpp', 'thiscall, ret 0',
          'regra: a Copa da Itália começa por grupos de 4; o grupo 1 é a própria copa (os outros 7 são fases à parte): os 4 primeiros da lista dos inscritos vão às linhas da tabela, saem da lista e cada humano recebe a notícia do grupo',
          'rule: the Italian Cup starts with groups of 4; group 1 is the cup object itself (the other 7 are separate stages): the first 4 entrants go to the table rows, leave the list, and each human gets the group news'),
         ('cqr7_copa_da_serie_c_grupo_um', 'ItaCCup::init_first_group', '0x63d5e0', 'ita_c_cup.cpp', 'thiscall, ret 0',
          'regra: a Copa da Itália da Série C começa por grupos de 5 (16 grupos); o grupo 1 é a própria copa: os 5 primeiros inscritos vão às linhas da tabela e saem da lista',
          'rule: the Serie C Italian Cup starts with groups of 5 (16 groups); group 1 is the cup object itself: the first 5 entrants go to the table rows and leave the list'),
         ('cqr7_copa_da_italia_fase_seguinte', 'ItaCup::advance_stage', '0x63f5b0', 'ita_cup.cpp', 'thiscall, ret 0',
          'regra: a fase da Copa da Itália avança uma; quando passa da fase 6 (os 8 grupos acabaram) monta o mata-mata (os 8 campeões e os 16 cabeças de chave); na última fase, a caixa de erro',
          'rule: the Italian Cup stage moves on by one; after stage 6 (the 8 groups are over) it builds the knockout (the 8 group winners and the 16 seeds); at the last stage, the error box'),
         ('cqr7_copa_da_serie_c_fase_seguinte', 'ItaCCup::advance_stage', '0x63d950', 'ita_c_cup.cpp', 'thiscall, ret 0',
          'regra: a fase da Copa da Série C avança uma; quando passa da fase 14 (os 16 grupos acabaram) monta o mata-mata de 32',
          'rule: the Serie C Cup stage moves on by one; after stage 14 (the 16 groups are over) it builds the 32-team knockout')])
CAMPOS = [('copa +0x3e times; valor; grava 4 ou 5 (grupo 1)', 'cup +0x3e teams; value; writes 4 or 5 (group 1)'),
          ('copa +0xb1 tabela (linhas de 0x3b bytes); ponteiro; grava (grupo 1)', 'cup +0xb1 table (0x3b-byte rows); pointer; writes (group 1)'),
          ('copa +0xee inscritos que esperam a fase (vetor de clubes); ponteiro; lê e zera (grupo 1)',
           'cup +0xee entrants waiting for a stage (club vector); pointer; reads and clears (group 1)'),
          ('copa +0x2c fases, +0x30 fase atual; valor; lê e grava +1 (fase seguinte)', 'cup +0x2c stages, +0x30 current stage; value; reads and writes +1 (next stage)')]
INTERESSA = []
