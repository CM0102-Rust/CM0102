# A Copa da Itália e a Copa da Itália da Série C: o mata-mata depois dos grupos (ita_cup.cpp, ita_c_cup.cpp, em cups\).
# Frente das competições (Agente D, 06/11/2026), carreira de 8 ligas (começa em janeiro de 2001).
# - 0x63f870 (ItaCup::create_knockout_stage, 630 bytes): os 8 campeões dos grupos e os 16 cabeças de chave; a fase 7.
# - 0x63dc10 (ItaCCup::create_knockout_stage, 975 bytes): os 16 campeões, os 6 melhores segundos (uma liga provisória
#   ordena os 16 segundos) e os 10 cabeças de chave; a fase 15.
# Ajudantes comuns: cqr9_tabela, cqr9_cabecas, cqr9_mata_mata (o Cup_Stage, como o play-off das Séries, 278a leva).
# Iguais no cm0102-1080.exe (as duas inteiras).
# Convenções conferidas (lição da 258a): as duas thiscall, ret 0 (do advance_stage). Dentro: 0x944e46 / 0x944cf1 /
# 0x9452ca (push; add esp -> cdecl); o método +0x3c da copa (push 0; push &extra; push &rodadas; push fase; mov ecx ->
# thiscall, ret 0x10); Cup_Stage::Cup_Stage 0x522e00 (15 pushes; mov ecx, nova -> thiscall, ret 0x3c); League_Stage
# 0x68b100 (22 pushes, ret 0x58); League::sort_table 0x6827d0 (push 0; mov ecx, liga -> thiscall, ret 4); o destrutor da
# liga provisória (a virtual 0: push 1; mov ecx -> thiscall); Cup::draw_round 0x51c410 (push 0; mov ecx, fase ->
# thiscall, ret 4); a caixa (aviso_em).
# Comparadores: só a original (alocam a fase e sorteiam a primeira rodada), conferindo o plano feito antes: a fase em
# [+0xc][fase] e os cabeças de chave tirados de +0xee.
# Determinismo: nada lê pilha, heap ou endereço como valor (os critérios de desempate {1, 2, 3, 0} vão num vetor local,
# como na original).
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqr9_copa_da_italia_mata_mata', 0x63f870, [0x6a, 0xff, 0x68, 0x2e, 0xb9, 0x95, 0x00]),
           ('cqr9_copa_da_serie_c_mata_mata', 0x63dc10, [0x6a, 0xff, 0x68, 0x9c, 0xb8, 0x95, 0x00])]
DOCS = ('a Copa da Itália e a da Série C: o mata-mata depois dos grupos',
        'the Italian Cup and the Serie C Cup: the knockout after the groups',
        [('cqr9_copa_da_italia_mata_mata', 'ItaCup::create_knockout_stage', '0x63f870', 'ita_cup.cpp', 'thiscall, ret 0',
          'regra: depois dos 8 grupos, o mata-mata da Copa da Itália tem 24 times: os 8 campeões dos grupos e os 16 cabeças de chave que entram direto (os primeiros inscritos ainda na lista: os clubes da Série A [inferência]); no sorteio os campeões vão com semente 3 e 2, os cabeças de chave com 2 e 1 (8 vagas por semente) [inferência: o pote de cada vaga]; a primeira rodada é sorteada na hora',
          'rule: after the 8 groups, the Italian Cup knockout has 24 teams: the 8 group winners and the 16 seeds entering directly (the first entrants still on the list: the Serie A clubs [inference]); in the draw the winners go with seed 3 and 2, the seeds with 2 and 1 (8 slots per seed) [inference: each slot\'s pot]; the first round is drawn at once'),
         ('cqr9_copa_da_serie_c_mata_mata', 'ItaCCup::create_knockout_stage', '0x63dc10', 'ita_c_cup.cpp', 'thiscall, ret 0',
          'regra: depois dos 16 grupos, o mata-mata da Copa da Série C tem 32 times: os 16 campeões, os 6 melhores segundos (os 16 segundos numa tabela, ordenados pelos critérios 1, 2, 3: pontos e desempates; o grupo deles marcado com 2 e a linha com 6) e 10 cabeças de chave que entram direto; sem sementes; a primeira rodada é sorteada na hora',
          'rule: after the 16 groups, the Serie C Cup knockout has 32 teams: the 16 winners, the 6 best runners-up (the 16 runners-up in a table sorted by criteria 1, 2, 3: points and tie-breakers; their group marked with 2 and the row with 6) and 10 seeds entering directly; no seeding; the first round is drawn at once')])
CAMPOS = [('copa +0xb1 tabela (linhas de 0x3b bytes: +0 clube, +5 e +9 zerados na cópia, +0x37 marca 6 do melhor segundo); ponteiro; lê e grava',
           'cup +0xb1 table (0x3b-byte rows: +0 club, +5 and +9 cleared in the copy, +0x37 best runner-up mark 6); pointer; reads and writes'),
          ('copa +0xc fases (as dos grupos e o mata-mata em 7 ou 15); ponteiro; lê e grava', 'cup +0xc stages (the group ones and the knockout at 7 or 15); pointer; reads and writes'),
          ('copa +0xee inscritos (até +0xf2); ponteiro; lê e zera', 'cup +0xee entrants (up to +0xf2); pointer; reads and clears'),
          ('copa +0xbe marca do grupo (2: tem melhor segundo); valor; grava', 'cup +0xbe group mark (2: has a best runner-up); value; writes')]
INTERESSA = []
