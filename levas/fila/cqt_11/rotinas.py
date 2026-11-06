# O Campeonato de Clubes da Oceania: os 8 clubes, a fase seguinte e as semifinais (oceania_club_champ.cpp, em
# eurocomp\). Frente das competições (Agente E, 06/11/2026), carreira de 8 ligas (a Austrália jogável; a edição de
# janeiro de 2002 fica dentro da janela).
# - 0x798220 (OceaniaClubChamp::select_qualified_clubs, 1054 bytes; 1 chamada até 2002-06-01): os 8 e os pares
#   sorteados entre os grupos. Duas mutações (o campeão australiano e o par Fiji/Taiti no outro grupo).
# - 0x797df0 (OceaniaClubChamp::advance_stage): a fase seguinte (passageira).
# - 0x798050 (OceaniaClubChamp::create_knockout_stage): as semifinais cruzadas. Uma mutação (1º × 1º, 2º × 2º).
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas (lição da 258a): os 8 são thiscall, ret 0 (mov ecx, esi; call, do dia); a fase seguinte é a
# virtual +0x28 (thiscall, ret 0); as semifinais thiscall, ret 0 (call direto da fase seguinte). Dentro: 0x4afd80 (2
# pushes; add esp -> cdecl), 0x53d900 e 0x53d940 (push nação; add esp -> cdecl), 0x4afbd0 (3 pushes; add esp ->
# cdecl), 0x90cfc0 (push 2; add esp -> cdecl), a virtual +0x3c, 0x522e00 (15 pushes; mov ecx, novo -> thiscall, ret
# 0x3c), 0x51c800 (push 0; mov ecx, fase -> thiscall, ret 4), 0x944e46, 0x944cf1, 0x9452ca e a caixa.
# Comparadores: os 8 lado a lado (o +0xf2, o [0xb67a34] e o estado do sorteador; a cópia sorteia sem contar; a lista
# da cópia comparada nas 8 vagas e liberada); a fase seguinte e as semifinais só a original (alocam e montam a rodada),
# conferindo +0x30 e a fase nova com os 4 cruzados (lidos antes).
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
# - mutações (mutacoes.py --carreira 2002-06-01 com as 8 ligas, base f29c85195c2a534f): as três ACUSADAS (o campeão
#   australiano no outro grupo em 2002-006, Fiji/Taiti no outro grupo em 2002-009, as semifinais 1º × 1º em 2002-018).
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqt11_oceania_clubes', 0x798220, [0x81, 0xec, 0xfc, 0x03, 0x00, 0x00]),
           ('cqt11_oceania_fase_seguinte', 0x797df0, [0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30]),
           ('cqt11_oceania_semifinais', 0x798050, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('o Campeonato de Clubes da Oceania: os 8 clubes, a fase seguinte e as semifinais',
        'the Oceania Club Championship: the 8 clubs, the next stage and the semi-finals',
        [('cqt11_oceania_clubes', 'OceaniaClubChamp::select_qualified_clubs', '0x798220', 'oceania_club_champ.cpp', 'thiscall, ret 0', 'regra: o Campeonato de Clubes da Oceania tem 8 clubes em 4 pares, cada par dividido entre os grupos A e B por sorteio: o campeão da liga australiana (sem ele, o melhor australiano) e o melhor neozelandês; o melhor de Fiji e o do Taiti; dois melanésios de nações diferentes; dois polinésios de nações diferentes (cada lista ordenada pela reputação, com a faixa de cima embaralhada)', 'rule: the Oceania Club Championship has 8 clubs in 4 pairs, each pair split between groups A and B by a draw: the Australian league champion (without one, the best Australian club) and the best New Zealand club; the best of Fiji and of Tahiti; two Melanesian clubs of different nations; two Polynesian clubs of different nations (each list sorted by reputation, with the top band shuffled)'),
         ('cqt11_oceania_fase_seguinte', 'OceaniaClubChamp::advance_stage', '0x797df0', 'oceania_club_champ.cpp', 'thiscall, ret 0', 'regra: o Campeonato de Clubes da Oceania avança uma fase quando a atual acaba; depois dos grupos (ao chegar a 1) monta as semifinais; passar da última fase é erro', 'rule: the Oceania Club Championship moves on one stage when the current one ends; after the groups (on reaching 1) it builds the semi-finals; going past the last stage is an error'),
         ('cqt11_oceania_semifinais', 'OceaniaClubChamp::create_knockout_stage', '0x798050', 'oceania_club_champ.cpp', 'thiscall, ret 0', 'regra: as semifinais do Campeonato de Clubes da Oceania cruzam os grupos: 1º do A × 2º do B e 2º do A × 1º do B, numa fase de copa (a fase 1) com a primeira rodada montada na hora', 'rule: the Oceania Club Championship semi-finals cross the groups: A winner × B runner-up and A runner-up × B winner, in a cup stage (stage 1) with the first round set up at once')])
CAMPOS = [('clube +0x53 nação (+0 o id, +0x71 o continente; [0x9cfa18] a Oceania); ponteiro/id; lê (os 8)', 'club +0x53 nation (+0 the id, +0x71 the continent; [0x9cfa18] Oceania); pointer/id; reads (the 8)'),
          ('[0x9cf3f4] Nova Zelândia, [0x9cf224] Austrália, [0x9cf4b4] Taiti, [0x9cf2f8] Fiji, [0x9cf89c] a liga australiana; id; lê', '[0x9cf3f4] New Zealand, [0x9cf224] Australia, [0x9cf4b4] Tahiti, [0x9cf2f8] Fiji, [0x9cf89c] the Australian league; id; reads'),
          ('copa +0xee os 8 (o grupo A nas vagas 0..3, o B nas 4..7), +0xf2 a conta; ponteiro/valor; grava (os 8)', 'cup +0xee the 8 (group A in places 0..3, B in 4..7), +0xf2 their count; pointer/value; writes (the 8)'),
          ('copa +0x2c fases, +0x30 fase atual, +0xc fases ([1] as semifinais); valor/ponteiro; lê e grava (fase seguinte, semifinais)', 'cup +0x2c stages, +0x30 current stage, +0xc stages ([1] the semi-finals); value/pointer; reads and writes (next stage, semi-finals)')]
INTERESSA = ['CMRTE']
