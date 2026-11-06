# O dia da Copa do Rei e o histórico da temporada de quatro copas (spa_cup.cpp, gre_cup.cpp, ire_lge_cup.cpp,
# ita_c_cup.cpp em cups\; european_cup.cpp em eurocomp\). Frente das competições (Agente D, 06/11/2026), carreira de 8
# ligas (começa em janeiro de 2001).
# - 0x84d740 (SpaCup::update_for_date, 457 bytes; todo dia: 2064 chamadas, a virada uma vez em 2001-000): o molde do dia
#   da Copa da Liga inglesa (0x56caf0, 276a leva).
# - 0x584ad0, 0x5e93e0, 0x637750, 0x63d770 (record_season_honours da Liga dos Campeões, da Copa da Grécia, da Copa da
#   Liga irlandesa e da Copa da Série C, 11 bytes cada: o salto para a virtual +0x30 da última fase): passageiras.
# Iguais no cm0102-1080.exe (as cinco inteiras).
# Convenções conferidas (lição da 258a): 0x84d740 thiscall, ret 8 (a virtual +0x14: push fase; push hoje; mov ecx);
# os quatro históricos thiscall, ret 0 (o jmp [edx+0x30] repassa a mesma pilha: a virtual +0x30 da fase, thiscall sem
# argumentos). Dentro: 0x9452ca (push; add esp, 4 -> cdecl); SpaCup::collect_entrants 0x84d910 (mov ecx, esi ->
# thiscall, ret 0; devolução com sinal); as virtuais +0x8c, +0x94, +0x5c (thiscall sem argumentos); Cup::update_for_date
# 0x51f890 (push fase; push hoje; mov ecx -> thiscall, ret 8); a caixa (aviso_em).
# Comparadores: só a original (a virada libera e refaz as tabelas, a atualização joga e sorteia; os históricos gravam nas
# estatísticas), o dia conferindo o plano (+0x45 vai a 1 quando a condição vale, senão fica).
# Determinismo: nada lê pilha, heap ou endereço como valor.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqt5_copa_do_rei_dia', 0x84d740, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqt5_historico_liga_dos_campeoes', 0x584ad0, [0x8b, 0x41, 0x0c, 0x8b, 0x48, 0x30]),
           ('cqt5_historico_copa_da_grecia', 0x5e93e0, [0x8b, 0x41, 0x0c, 0x8b, 0x48, 0x28]),
           ('cqt5_historico_copa_da_liga_irlandesa', 0x637750, [0x8b, 0x41, 0x0c, 0x8b, 0x48, 0x1c]),
           ('cqt5_historico_copa_da_serie_c', 0x63d770, [0x8b, 0x41, 0x0c, 0x8b, 0x48, 0x3c])]
DOCS = ('o dia da Copa do Rei e o histórico da temporada da Liga dos Campeões, da Copa da Grécia, da Copa da Liga irlandesa e da Copa da Série C',
        'the King\'s Cup day and the season history of the Champions League, the Greek Cup, the Irish League Cup and the Serie C Cup',
        [('cqt5_copa_do_rei_dia', 'SpaCup::update_for_date', '0x84d740', 'spa_cup.cpp', 'thiscall, ret 8',
          'regra: a Copa do Rei é refeita uma vez por edição, no primeiro dia (com a fase do dia) em que o ano da classificação da UEFA chega ao ano da copa: as tabelas antigas liberadas, os inscritos (os da Primera, da Segunda e da Segunda B e os clubes de baixo, pela regra da nação), as rodadas, a árvore e as posições previstas; todo dia a copa joga as suas rodadas',
          'rule: the King\'s Cup is rebuilt once per edition, on the first day (with the day phase) on which the UEFA ranking year reaches the cup year: old tables freed, the entrants (Primera, Segunda and Segunda B clubs and lower clubs, by the nation rule), the rounds, the tree and the predicted positions; every day the cup plays its rounds'),
         ('cqt5_historico_liga_dos_campeoes', 'EuropeanCup::record_season_honours', '0x584ad0', 'european_cup.cpp', 'thiscall, ret 0',
          'regra: o histórico da temporada da Liga dos Campeões é o da sua última fase (a 12: a final)',
          'rule: the Champions League season history is that of its last stage (stage 12: the final)'),
         ('cqt5_historico_copa_da_grecia', 'GreCup::record_season_honours', '0x5e93e0', 'gre_cup.cpp', 'thiscall, ret 0',
          'regra: o histórico da temporada da Copa da Grécia é o da fase 10 (a final)',
          'rule: the Greek Cup season history is that of stage 10 (the final)'),
         ('cqt5_historico_copa_da_liga_irlandesa', 'IreLgeCup::record_season_honours', '0x637750', 'ire_lge_cup.cpp', 'thiscall, ret 0',
          'regra: o histórico da temporada da Copa da Liga irlandesa é o da fase 7 (a final)',
          'rule: the Irish League Cup season history is that of stage 7 (the final)'),
         ('cqt5_historico_copa_da_serie_c', 'ItaCCup::record_season_honours', '0x63d770', 'ita_c_cup.cpp', 'thiscall, ret 0',
          'regra: o histórico da temporada da Copa da Série C é o do mata-mata (a fase 15)',
          'rule: the Serie C Cup season history is that of the knockout (stage 15)')])
CAMPOS = [('copa +0x45 já refeita nesta edição; valor; lê e grava 1 (dia)', 'cup +0x45 already rebuilt this edition; value; reads and writes 1 (day)'),
          ('[0xde1f58] +0x439 o ano da classificação da UEFA (word); valor; lê', '[0xde1f58] +0x439 the UEFA ranking year (word); value; reads'),
          ('copa +0xa7, +0xa3, +0xad as tabelas da edição; ponteiro; libera (dia)', 'cup +0xa7, +0xa3, +0xad the edition tables; pointer; frees (day)'),
          ('copa +0xc fases; ponteiro; lê a última (históricos)', 'cup +0xc stages; pointer; reads the last one (histories)')]
INTERESSA = []
