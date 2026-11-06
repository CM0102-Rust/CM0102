# Os mata-matas da Libertadores e da Merconorte (conmebol_liber.cpp, con_merc_cup.cpp, em eurocomp\). Frente das
# competições (Agente E, 06/11/2026), carreira de 8 ligas (começa em janeiro de 2001).
# - 0x4c3d10 (ConmebolLiber::create_knockout_stage) e 0x4c2330 (ConMercCup::create_semi_final_stage): o mesmo molde
#   (cqs10_mata_mata, conferido instrução a instrução: mudam o tamanho da lista, as fases, o fonte e as linhas).
#   Duas mutações no molde (o seguinte de cada fase passa; o seguinte do grupo 1 passa) e uma na ordem das fases.
# - 0x4c20b0 (ConMercCup::advance_stage): a fase seguinte da Merconorte (passageira).
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas (lição da 258a): os dois mata-matas são thiscall, ret 0 (mov ecx, esi; call, da fase seguinte);
# a fase seguinte é a virtual +0x28 (thiscall, ret 0). Dentro: 0x6826d0 (push j; mov ecx, fase -> thiscall, ret 4),
# a virtual +0x3c (4 pushes; mov ecx -> thiscall, ret 0x10), 0x522e00 (15 pushes; mov ecx, novo -> thiscall, ret
# 0x3c), 0x51c410 (push 0; mov ecx, fase -> thiscall, ret 4), 0x944e46, 0x944cf1, 0x9452ca e a caixa.
# Comparadores: só a original (alocam, sorteiam a rodada), com o plano: o mata-mata confere a fase nova, os times, o
# +4 e os clubes da lista dela (+0xa7) contra os classificados lidos antes; a fase seguinte confere +0x30.
# Determinismo: nada lê pilha, heap ou endereço como valor.
# Provas (06/11/2026, cópia fora do projeto em %TEMP%: motor.rs da principal com a 280a publicada e a 281a integrada,
# 1.986 rotinas, + os 12 itens cqr_10..13, cqs_10..13, cqt_10..13, 36 rotinas):
# - jogo comparando com ENG ESP ITA FRA BRA JPN AUS USA até 2001-10-01 só com os 4 cqr (parou em 2001-273) e até
#   2002-06-01 com os 12 itens (parou em 2002-151; as copas de janeiro e fevereiro de 2002 e a Libertadores de 2002
#   rodam depois de outubro): 0 linhas DIVERGENCIA, 0 EXCECAO, nenhuma caixa (data, motor e aleatorio: 0 divergências);
# - duas carreiras Rust com as 8 ligas até 2002-06-01 iguais entre si e ao detector (simular.py --repetir 2
#   --detector: "detector de lixo: igual"), impressão b690295afd222f6a (a mesma da base sem os itens: as cópias são
#   fiéis); todos os contadores das mutações rodaram;
# - mutações (mutacoes.py --carreira 2002-06-01 com as 8 ligas, base f29c85195c2a534f): as três ACUSADAS, todas já no
#   mata-mata da Libertadores de 2001 (o seguinte de cada fase e as fases em ordem inversa em 2001-105, o seguinte do
#   grupo 1 em 2001-108: o molde é o mesmo nas duas copas, então essa também pega a Libertadores antes da Merconorte).
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqs10_libertadores_mata_mata', 0x4c3d10, [0x6a, 0xff, 0x68, 0xbe, 0x6a, 0x95, 0x00]),
           ('cqs10_merconorte_semifinais', 0x4c2330, [0x6a, 0xff, 0x68, 0x0e, 0x6a, 0x95, 0x00]),
           ('cqs10_merconorte_fase_seguinte', 0x4c20b0, [0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30])]
DOCS = ('os mata-matas da Libertadores e da Merconorte: os classificados dos grupos e a fase seguinte',
        'the Libertadores and Merconorte knockouts: the group qualifiers and the next stage',
        [('cqs10_libertadores_mata_mata', 'ConmebolLiber::create_knockout_stage', '0x4c3d10', 'conmebol_liber.cpp', 'thiscall, ret 0', 'regra: o mata-mata da Libertadores tem 16 clubes: os 2 primeiros (+0xbe) de cada um dos 8 grupos (o grupo 1 na ordem da tabela, os outros pela posição na tabela de cada fase), numa fase de copa (a fase 7) com a tabela de rodadas da copa e o ano dela; a primeira rodada é sorteada na hora', "rule: the Libertadores knockout has 16 clubs: the top 2 (+0xbe) of each of the 8 groups (group 1 in table order, the others by position in each stage's table), in a cup stage (stage 7) with the cup's round table and year; the first round is drawn at once"),
         ('cqs10_merconorte_semifinais', 'ConMercCup::create_semi_final_stage', '0x4c2330', 'con_merc_cup.cpp', 'thiscall, ret 0', 'regra: as semifinais da Merconorte têm 4 clubes: o primeiro (+0xbe = 1) de cada um dos 4 grupos, numa fase de copa (a fase 3); a primeira rodada é sorteada na hora', 'rule: the Merconorte semi-finals have 4 clubs: the winner (+0xbe = 1) of each of the 4 groups, in a cup stage (stage 3); the first round is drawn at once'),
         ('cqs10_merconorte_fase_seguinte', 'ConMercCup::advance_stage', '0x4c20b0', 'con_merc_cup.cpp', 'thiscall, ret 0', 'regra: a Merconorte avança uma fase quando a atual acaba; depois das 3 fases de grupo (ao chegar a 3) monta as semifinais; passar da última fase é erro', 'rule: the Merconorte moves on one stage when the current one ends; after the 3 group stages (on reaching 3) it builds the semi-finals; going past the last stage is an error')])
CAMPOS = [('copa +0xbe classificados por grupo (byte com sinal), +0xb1 tabela do grupo 1; valor/ponteiro; lê (mata-mata)', 'cup +0xbe qualifiers per group (signed byte), +0xb1 group 1 table; value/pointer; reads (knockout)'),
          ('copa +0xc fases (League_Stage dos grupos; o Cup_Stage do mata-mata gravado em [+0xc][7] ou [3]); ponteiro; lê e grava (mata-mata)', 'cup +0xc stages (group League_Stages; the knockout Cup_Stage written to [+0xc][7] or [3]); pointer; reads and writes (knockout)'),
          ('copa +0x2c fases, +0x30 fase atual; valor; lê e grava (fase seguinte)', 'cup +0x2c stages, +0x30 current stage; value; reads and writes (next stage)')]
INTERESSA = ['CMRTE']
