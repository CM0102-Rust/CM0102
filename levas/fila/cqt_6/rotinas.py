# A Copa dos Campeões do Brasil: a fase preliminar e a fase seguinte (bra_champ_cup.cpp, em cups\). Frente das
# competições (Agente D, 06/11/2026), carreira de 8 ligas (começa em janeiro de 2001).
# - 0x426130 (BraChampCup::create_preliminary_table, 241 bytes; 2001-167): a tabela dos 3 da preliminar.
# - 0x427320 (BraChampCup::start_preliminary_stage, 269 bytes; 2001-167): a preliminar (tabela, jogos e ordem).
# - 0x4263c0 (BraChampCup::advance_stage, 161 bytes; 2001-183 e 2001-200): os grupos e as semifinais.
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas (lição da 258a): as três thiscall, ret 0. Dentro: 0x944e46 (push; add esp -> cdecl);
# League::init_table_row 0x687430 (push 0; push 0; push clube; push índice; mov ecx -> thiscall, ret 0x10: só o byte do
# índice é lido); 0x684230 (thiscall, ret 0); a virtual +0x5c (thiscall); 0x426130 e League::create_fixtures 0x6835c0
# (mov ecx -> thiscall, ret 0); League::sort_table 0x6827d0 (push 0; mov ecx -> thiscall, ret 4); 0x426470 e 0x426720
# (call direto com o ecx da copa -> thiscall, ret 0); a caixa (aviso_em).
# Comparadores: só a original (alocam, mandam a notícia e sorteiam), com o plano: a tabela com os 3 da lista e eles
# zerados; +0x45 = 1 depois da preliminar; a fase +1 fora da última.
# Determinismo: nada lê pilha, heap ou endereço como valor (o lixo do dword do índice não é lido).
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqt6_campeoes_tabela_da_preliminar', 0x426130, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('cqt6_campeoes_preliminar', 0x427320, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqt6_campeoes_fase_seguinte', 0x4263c0, [0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30])]
DOCS = ('a Copa dos Campeões do Brasil: a fase preliminar e a fase seguinte',
        'the Brazilian Champions Cup: the preliminary stage and the next stage',
        [('cqt6_campeoes_tabela_da_preliminar', 'BraChampCup::create_preliminary_table', '0x426130', 'bra_champ_cup.cpp', 'thiscall, ret 0',
          'regra: a Copa dos Campeões começa por uma preliminar de 3 clubes (a própria copa é a liga): os 3 primeiros da lista dos inscritos vão às linhas da tabela e saem da lista; cada humano recebe a notícia do grupo',
          'rule: the Champions Cup starts with a 3-club preliminary round (the cup object is the league): the first 3 entrants go to the table rows and leave the list; each human gets the group news'),
         ('cqt6_campeoes_preliminar', 'BraChampCup::start_preliminary_stage', '0x427320', 'bra_champ_cup.cpp', 'thiscall, ret 0',
          'regra: a preliminar começa com as posições previstas, a tabela dos 3, os jogos (turno único das regras da copa) e a ordem da tabela; a copa fica marcada como começada (+0x45)',
          'rule: the preliminary starts with the predicted positions, the 3-club table, the fixtures (the cup rules\' single round) and the table order; the cup is marked as started (+0x45)'),
         ('cqt6_campeoes_fase_seguinte', 'BraChampCup::advance_stage', '0x4263c0', 'bra_champ_cup.cpp', 'thiscall, ret 0',
          'regra: a fase da Copa dos Campeões avança uma: no fim da preliminar vêm os grupos (dois, com o vencedor da preliminar); no fim dos grupos (fase 2), as semifinais; na última fase, a caixa de erro',
          'rule: the Champions Cup stage moves on by one: after the preliminary come the groups (two, with the preliminary winner); after the groups (stage 2), the semi-finals; at the last stage, the error box')])
CAMPOS = [('copa +0x14 lista dos inscritos (6 bytes: clube...); ponteiro; lê e zera (preliminar)', 'cup +0x14 entrants list (6 bytes: club...); pointer; reads and clears (preliminary)'),
          ('copa +0x3e times, +0xb1 tabela; valor/ponteiro; grava (preliminar)', 'cup +0x3e teams, +0xb1 table; value/pointer; writes (preliminary)'),
          ('copa +0x45 começou; valor; grava 1', 'cup +0x45 started; value; writes 1'),
          ('copa +0x2c fases, +0x30 fase atual; valor; lê e grava +1', 'cup +0x2c stages, +0x30 current stage; value; reads and writes +1')]
INTERESSA = []
