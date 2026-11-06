# A Copa da Itália e a Copa da Itália da Série C: a fase de cada grupo (ita_cup.cpp, ita_c_cup.cpp, em cups\). Frente
# das competições (Agente D, 06/11/2026), carreira de 8 ligas (começa em janeiro de 2001).
# - 0x63f640 (ItaCup::create_group_stage, 558 bytes) e 0x63d9e0 (ItaCCup::create_group_stage, 558 bytes): o mesmo molde
#   (só mudam o fonte e as linhas das caixas), um ajudante comum (cqr8_fase_do_grupo); o molde da fase dos grupos da Copa
#   do Mundo (0x92d540, 270a leva), com outra ordem no fim (a notícia antes de +0x30) e os times tirados de +0xee.
# Iguais no cm0102-1080.exe (as duas inteiras).
# Convenções conferidas (lição da 258a): as duas thiscall, ret 4 (g). Dentro: o método +0x3c da copa (push 0; push &id;
# push &rodadas; push g; mov ecx, esi -> thiscall, ret 0x10); 0x944e46 / 0x944cf1 / 0x9452ca (push; add esp -> cdecl);
# League_Stage::League_Stage 0x68b100 (22 pushes; mov ecx, nova -> thiscall, ret 0x58; os words e bytes vão com lixo em
# cima, que o construtor não lê: a cópia passa os valores limpos); League::send_group_draw_news 0x684230 (mov ecx, fase
# -> thiscall, ret 0); a caixa (aviso_em).
# Comparador: só a original (aloca a fase e manda a notícia), conferindo o plano feito antes: a fase em [+0xc][g], +0x30
# = g, os times e o ano da copa na fase, os clubes tirados de +0xee na tabela da fase.
# Determinismo: nada lê pilha, heap ou endereço como valor.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqr8_copa_da_italia_grupo', 0x63f640, [0x6a, 0xff, 0x68, 0x0e, 0xb9, 0x95, 0x00]),
           ('cqr8_copa_da_serie_c_grupo', 0x63d9e0, [0x6a, 0xff, 0x68, 0x6e, 0xb8, 0x95, 0x00])]
DOCS = ('a Copa da Itália e a da Série C: a fase de cada grupo',
        'the Italian Cup and the Serie C Cup: each group\'s stage',
        [('cqr8_copa_da_italia_grupo', 'ItaCup::create_group_stage', '0x63f640', 'ita_cup.cpp', 'thiscall, ret 4',
          'regra: os grupos 2 a 8 da Copa da Itália são ligas à parte (League_Stage) de 4 times: os 4 seguintes da lista dos inscritos, que saem dela, com as rodadas da tabela da copa para aquela fase, em turno único (1) e 2 pontos [inferência: os últimos argumentos do construtor]; cada humano recebe a notícia do grupo',
          'rule: groups 2 to 8 of the Italian Cup are separate leagues (League_Stage) of 4 teams: the next 4 entrants on the list, which leave it, with the rounds of the cup table for that stage, single round-robin (1) and 2 points [inference: the constructor\'s last arguments]; each human gets the group news'),
         ('cqr8_copa_da_serie_c_grupo', 'ItaCCup::create_group_stage', '0x63d9e0', 'ita_c_cup.cpp', 'thiscall, ret 4',
          'regra: os grupos 2 a 16 da Copa da Série C são ligas à parte de 5 times, tirados da lista dos inscritos na ordem, como na Copa da Itália',
          'rule: groups 2 to 16 of the Serie C Cup are separate leagues of 5 teams, taken from the entrants list in order, as in the Italian Cup')])
CAMPOS = [('copa +0x3e times por grupo; valor; lê', 'cup +0x3e teams per group; value; reads'),
          ('copa +0xee inscritos que esperam a fase (vetor de clubes); ponteiro; lê e zera', 'cup +0xee entrants waiting for a stage (club vector); pointer; reads and clears'),
          ('copa +0xc fases; ponteiro; grava a fase g', 'cup +0xc stages; pointer; writes stage g'),
          ('copa +0x30 fase atual; valor; grava g', 'cup +0x30 current stage; value; writes g'),
          ('copa +4, +0x40, +0x51, +0xbe, +0xc2..+0xc5, +0xd9; valores das regras passados à fase; lê',
           'cup +4, +0x40, +0x51, +0xbe, +0xc2..+0xc5, +0xd9; rule values passed to the stage; reads')]
INTERESSA = []
