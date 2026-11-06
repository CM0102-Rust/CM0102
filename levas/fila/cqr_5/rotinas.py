# A virada das copas: a Copa do Brasil, a Copa do Imperador e a Copa da Liga japonesa (bra_cup.cpp, jap_emp_cup.cpp,
# jap_j_cup.cpp, em cups\). Frente das competições (Agente D, 06/11/2026), carreira de 8 ligas (começa em janeiro de
# 2001).
# - 0x427d10 (BraCup::start_new_season, 425 bytes), 0x662950 (JapEmpCup::start_new_season, 425 bytes) e 0x665bc0
#   (JapJCup::start_new_season, 425 bytes): o slot 3 (+0x08) das copas de mata-mata, o mesmo molde conferido instrução a
#   instrução (só mudam o fonte e as linhas das caixas e a rotina dos inscritos): um ajudante comum (cqr5_virada).
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas (lição da 258a): as três thiscall, ret 0 (a virtual +0x08). Dentro: 0x9452ca (push; add esp, 4
# -> cdecl); 0x4a1c50 (push 1; mov ecx, [esi+8] -> thiscall, ret 4); os inscritos 0x427c20 / 0x6624f0 / 0x665a00 (mov
# ecx, esi; call -> thiscall, ret 0; devolução com sinal, test/jge); as virtuais +0x8c, +0x94 e +0x5c (mov edx, [esi];
# mov ecx, esi; call [edx+o] -> thiscall, sem argumentos); a caixa (aviso_em).
# Comparador: só a original (libera as tabelas e refaz os inscritos, as rodadas e a árvore: não roda duas vezes),
# conferindo o ano +1.
# Determinismo: nada lê pilha, heap ou endereço como valor.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqr5_virada_copa_do_brasil', 0x427d10, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqr5_virada_copa_do_imperador', 0x662950, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqr5_virada_copa_da_liga_japonesa', 0x665bc0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a virada das copas: a Copa do Brasil, a Copa do Imperador e a Copa da Liga japonesa',
        'the cups\' season turn: the Brazilian Cup, the Emperor\'s Cup and the Japanese League Cup',
        [('cqr5_virada_copa_do_brasil', 'BraCup::start_new_season', '0x427d10', 'bra_cup.cpp', 'thiscall, ret 0',
          'regra: na virada da temporada a Copa do Brasil é refeita do zero: as tabelas da edição anterior são liberadas, o histórico da temporada é arquivado (Comp_Stats), o ano sobe um e a copa monta de novo os seus inscritos (a tabela dos times), as rodadas e a árvore dos confrontos, e as posições previstas',
          'rule: at the season turn the Brazilian Cup is rebuilt from scratch: last edition\'s tables are freed, the season history is archived (Comp_Stats), the year goes up by one and the cup builds again its entrants (the team table), the rounds and the fixture tree, and the predicted positions'),
         ('cqr5_virada_copa_do_imperador', 'JapEmpCup::start_new_season', '0x662950', 'jap_emp_cup.cpp', 'thiscall, ret 0',
          'regra: a mesma virada na Copa do Imperador (Japão): as tabelas liberadas, o histórico arquivado, o ano +1, os inscritos (JapEmpCup::collect_entrants), as rodadas, a árvore e as posições previstas',
          'rule: the same turn in the Emperor\'s Cup (Japan): tables freed, history archived, year +1, the entrants (JapEmpCup::collect_entrants), the rounds, the tree and the predicted positions'),
         ('cqr5_virada_copa_da_liga_japonesa', 'JapJCup::start_new_season', '0x665bc0', 'jap_j_cup.cpp', 'thiscall, ret 0',
          'regra: a mesma virada na Copa da Liga japonesa (J-League Cup): as tabelas liberadas, o histórico arquivado, o ano +1, os 28 inscritos (JapJCup::collect_entrants), as rodadas, a árvore e as posições previstas',
          'rule: the same turn in the Japanese League Cup (J-League Cup): tables freed, history archived, year +1, the 28 entrants (JapJCup::collect_entrants), the rounds, the tree and the predicted positions')])
CAMPOS = [('copa +0x40 ano (word); valor; lê e grava +1 (virada)', 'cup +0x40 year (word); value; reads and writes +1 (season turn)'),
          ('copa +0x4c, +0xab, +0xb1 (byte); valor; grava 0 (virada)', 'cup +0x4c, +0xab, +0xb1 (byte); value; writes 0 (season turn)'),
          ('copa +0xa7 inscritos, +0xa3 rodadas, +0xad confrontos por rodada (+0x3c rodadas); ponteiro; libera e grava 0 (virada)',
           'cup +0xa7 entrants, +0xa3 rounds, +0xad fixtures per round (+0x3c rounds); pointer; frees and writes 0 (season turn)'),
          ('copa +8 estatísticas da competição (Comp_Stats); ponteiro; lê (virada)', 'cup +8 competition statistics (Comp_Stats); pointer; reads (season turn)')]
INTERESSA = []
