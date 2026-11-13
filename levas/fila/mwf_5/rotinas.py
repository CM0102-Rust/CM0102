# As viradas de temporada das copas em que os clubes entram rodada a rodada: a Copa da Liga e a Copa da Inglaterra
# (FA Cup) e a Copa do Rei (eng_cc_cup.cpp, eng_fa_cup.cpp, spa_cup.cpp). Leitor mwf (12/11/2026), rodada 2 (o mundo
# até 2002-08-01): cada uma roda 1 vez, na virada de meados de 2002 (docs/cobertura_mundo.txt).
# - 0x56d130 (EngCcCup::start_new_season), 0x570390 (EngFaCup::start_new_season), 0x84df70 (SpaCup::start_new_season).
# As três são o mesmo corpo (conferido instrução por instrução pelo norm.py do leitor mwf): as liberações e o arquivo
# das estatísticas do molde das copas (start_new_cup_season), mas em vez da rotina dos participantes a tabela de times
# é alocada vazia com o mesmo tamanho (calloc(+0x3e, 6)); +0x45 = 0. A FA Cup zera também +0xac e +0xb2 (a pergunta da
# 5a rodada); só a Copa do Rei chama as posições previstas.
# Convenções: virtual +0x08 do objeto -> thiscall, ret 0; dentro: 0x945501 (push 6; push n; add esp, 8 -> cdecl),
# 0x4a1c50 (push 1; mov ecx -> thiscall, ret 4), as virtuais +0x8c, +0x94 e +0x5c (mov ecx, esi -> thiscall, ret 0).
# Comparador: só a original (libera e refaz a copa), com o ano +1 (compare_cup_new_season).
LIGAS = 'ENG KOR ARG ITA FRA ESP AUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('eng_cc_cup_start_new_season', 0x56d130, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('eng_fa_cup_start_new_season', 0x570390, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('spa_cup_start_new_season', 0x84df70, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as viradas da Copa da Liga, da FA Cup e da Copa do Rei',
        'the season turnovers of the League Cup, the FA Cup and the Copa del Rey',
        [('eng_cc_cup_start_new_season', 'EngCcCup::start_new_season', '0x56d130', 'eng_cc_cup.cpp', 'thiscall, ret 0',
          'regra: na virada da temporada a Copa da Liga inglesa é refeita: as tabelas da edição anterior liberadas, a temporada arquivada nas estatísticas, o ano mais um, uma tabela de times vazia do mesmo tamanho (os clubes entram rodada a rodada), as rodadas e a árvore de confrontos; sem posições previstas',
          "rule: at the season turnover the English League Cup is rebuilt: last edition's tables freed, the season archived in the statistics, the year plus one, an empty team table of the same size (the clubs come in round by round), the rounds and the tie tree; no predicted positions"),
         ('eng_fa_cup_start_new_season', 'EngFaCup::start_new_season', '0x570390', 'eng_fa_cup.cpp', 'thiscall, ret 0',
          'regra: a virada da FA Cup é a da Copa da Liga, e zera também a próxima rodada a sortear e a pergunta sobre entrar na 5a rodada; sem posições previstas',
          "rule: the FA Cup turnover is the League Cup's, and it also clears the next round to draw and the question about entering at the 5th round; no predicted positions"),
         ('spa_cup_start_new_season', 'SpaCup::start_new_season', '0x84df70', 'spa_cup.cpp', 'thiscall, ret 0',
          'regra: a virada da Copa do Rei é a da Copa da Liga inglesa, seguida das posições previstas',
          "rule: the Copa del Rey turnover is the English League Cup's, followed by the predicted positions")])
CAMPOS = [('copa +0x4c, +0xa3, +0xa7 (calloc de +0x3e × 6), +0xab, +0xac, +0xad, +0xb1, +0x45, +0x40; FA Cup +0xb2; valor/ponteiro; grava',
           'cup +0x4c, +0xa3, +0xa7 (calloc of +0x3e × 6), +0xab, +0xac, +0xad, +0xb1, +0x45, +0x40; FA Cup +0xb2; value/pointer; writes')]
