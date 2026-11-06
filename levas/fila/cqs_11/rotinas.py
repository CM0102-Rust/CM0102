# Os convidados da Merconorte e a fase seguinte da Mercosul e da Copa América (con_merc_cup.cpp, conmebol_merc.cpp, em
# eurocomp\, e goldcup.cpp, em intercomp\). Frente das competições (Agente E, 06/11/2026), carreira de 8 ligas.
# - 0x4c2560 (ConMercCup::select_invited_clubs, 978 bytes; 1 chamada até 2002-06-01): os 16 convidados. As três
#   mutações.
# - 0x4c5140 (ConmebolMerc::advance_stage) e 0x5e1620 (GoldCup::advance_stage): a fase seguinte (passageiras).
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas (lição da 258a): os convidados são thiscall, ret 0 (mov ecx, esi; call, do dia da Merconorte);
# as fases seguintes são a virtual +0x28 (thiscall, ret 0). Dentro: 0x53b900 (push clube; add esp, 4 -> cdecl),
# 0x4afbd0 (3 pushes; add esp junto, 0x3c para as 5 -> cdecl), 0x4afd80 (2 pushes; add esp -> cdecl), 0x4afb40 (2
# pushes; add esp, 8 -> cdecl), 0x944e46, as rotinas do mata-mata (0x4c53d0 e 0x5e18a0: mov ecx -> thiscall, ret 0)
# e a caixa.
# Comparadores: os convidados lado a lado (o +0xf2, o [0xb67a34] e o estado do sorteador; a cópia sorteia sem contar;
# a lista da cópia comparada nas 16 vagas e liberada); as fases seguintes só a original (o mata-mata aloca e
# sorteia), conferindo +0x30.
# Determinismo: as listas das nações ficam na pilha da cópia zeradas (a original não as zera: com clubes que bastam
# só as vagas gravadas são lidas; o comparador confere a lista).
# Provas (06/11/2026, cópia fora do projeto em %TEMP%: motor.rs da principal com a 280a publicada e a 281a integrada,
# 1.986 rotinas, + os 12 itens cqr_10..13, cqs_10..13, cqt_10..13, 36 rotinas):
# - jogo comparando com ENG ESP ITA FRA BRA JPN AUS USA até 2001-10-01 só com os 4 cqr (parou em 2001-273) e até
#   2002-06-01 com os 12 itens (parou em 2002-151; as copas de janeiro e fevereiro de 2002 e a Libertadores de 2002
#   rodam depois de outubro): 0 linhas DIVERGENCIA, 0 EXCECAO, nenhuma caixa (data, motor e aleatorio: 0 divergências);
# - duas carreiras Rust com as 8 ligas até 2002-06-01 iguais entre si e ao detector (simular.py --repetir 2
#   --detector: "detector de lixo: igual"), impressão b690295afd222f6a (a mesma da base sem os itens: as cópias são
#   fiéis); todos os contadores das mutações rodaram;
# - mutações (mutacoes.py --carreira 2002-06-01 com as 8 ligas, base f29c85195c2a534f): as três ACUSADAS em 2001-213, o
#   dia dos convidados (a faixa do México de 1, o vice da MLS no lugar do campeão, o embaralhamento de 15). As fases
#   seguintes da Mercosul e da Copa América são passageiras (contadores 1 e 1: rodam).
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqs11_merconorte_convidados', 0x4c2560, [0x81, 0xec, 0x24, 0x04, 0x00, 0x00]),
           ('cqs11_mercosul_fase_seguinte', 0x4c5140, [0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30]),
           ('cqs11_copa_america_fase_seguinte', 0x5e1620, [0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30])]
DOCS = ('os convidados da Merconorte e a fase seguinte da Mercosul e da Copa América',
        'the Merconorte invited clubs and the Mercosur and Copa América next stage',
        [('cqs11_merconorte_convidados', 'ConMercCup::select_invited_clubs', '0x4c2560', 'con_merc_cup.cpp', 'thiscall, ret 0', 'regra: a Merconorte convida 16: os 4 melhores do México, os 5 da Colômbia, os 3 do Peru e os 2 da Bolívia (cada lista ordenada pela reputação, com a faixa de cima embaralhada: 3 vezes as vagas) e 2 da liga americana (com a competição dela: o campeão do último registro da MLS e o da outra competição dela, e o vice da MLS se forem o mesmo; sem ela, os 2 melhores clubes da MLS); a lista final é embaralhada (3 × 16 trocas) antes dos grupos', 'rule: the Merconorte invites 16: the best 4 of Mexico, 5 of Colombia, 3 of Peru and 2 of Bolivia (each list sorted by reputation, with the top band shuffled: 3 times the places) and 2 from the American league (with its competition: the latest MLS champion and that of its other competition, and the MLS runner-up if they are the same club; without it, the 2 best MLS clubs); the final list is shuffled (3 × 16 swaps) before the groups'),
         ('cqs11_mercosul_fase_seguinte', 'ConmebolMerc::advance_stage', '0x4c5140', 'conmebol_merc.cpp', 'thiscall, ret 0', 'regra: a Mercosul avança uma fase quando a atual acaba; depois das 4 fases de grupo (ao chegar a 4) monta as quartas; passar da última fase é erro', 'rule: the Mercosur moves on one stage when the current one ends; after the 4 group stages (on reaching 4) it builds the quarter-finals; going past the last stage is an error'),
         ('cqs11_copa_america_fase_seguinte', 'GoldCup::advance_stage', '0x5e1620', 'goldcup.cpp', 'thiscall, ret 0', 'regra: a Copa América avança uma fase quando a atual acaba; depois das 2 fases de grupo (ao chegar a 2) monta o mata-mata; passar da última fase é erro', 'rule: the Copa América moves on one stage when the current one ends; after the 2 group stages (on reaching 2) it builds the knockout; going past the last stage is an error')])
CAMPOS = [('clube +0x53 nação (o +0 dela é o id), +0x57 liga (o +0 dela é o id da competição); ponteiro; lê (convidados)', 'club +0x53 nation (its +0 is the id), +0x57 league (its +0 is the competition id); pointer; reads (invited)'),
          ('[0x9cf3d0] México, [0x9cf29c] Colômbia, [0x9cf428] Peru, [0x9cf254] Bolívia, [0x9cf590] a MLS, [0x9cf728] a competição da liga americana; id; lê', '[0x9cf3d0] Mexico, [0x9cf29c] Colombia, [0x9cf428] Peru, [0x9cf254] Bolivia, [0x9cf590] the MLS, [0x9cf728] the American league competition; id; reads'),
          ('copa +0xee lista dos convidados, +0xf2 a conta (16); ponteiro/valor; grava (convidados)', 'cup +0xee invited list, +0xf2 its count (16); pointer/value; writes (invited)'),
          ('copa +0x2c fases, +0x30 fase atual; valor; lê e grava (fases seguintes)', 'cup +0x2c stages, +0x30 current stage; value; reads and writes (next stages)')]
INTERESSA = ['CMRTE']
