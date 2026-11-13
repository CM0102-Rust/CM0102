# As viradas de temporada de três copas (Charity Shield, Copa da França, Copa da Liga francesa), no molde das copas (bra_cup_start_new_season / start_new_cup_season, já em Rust desde a 282a: só mudam o arquivo
# das caixas, as linhas e a rotina dos participantes; conferido instrução por instrução pelo norm.py do leitor mwf).
# Leitor mwf (12/11/2026), rodada 2 (o mundo inteiro até 2002-08-01): cada rotina roda 1 vez (a virada de meados de
# 2002; docs/cobertura_mundo.txt).
# Convenções: virtual +0x08 do objeto (mov ecx, esi; call [eax+8] no RulingBody / na liga que vira) -> thiscall, ret 0.
# Comparador: só a original (libera e refaz a copa), com o ano +1 (compare_cup_new_season).
LIGAS = 'ENG KOR ARG ITA FRA ESP AUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('eng_charity_start_new_season', 0x56d9b0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('fra_cup_start_new_season', 0x5bae50, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('fra_lge_cup_start_new_season', 0x5bd420, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as viradas da Charity Shield, da Copa da França e da Copa da Liga francesa',
        'the season turnovers of the Charity Shield, the Coupe de France and the French League Cup',
        [('eng_charity_start_new_season', 'EngCharity::start_new_season', '0x56d9b0', 'eng_charity.cpp', 'thiscall, ret 0', 'regra: a virada da temporada da Charity Shield refaz a copa: as tabelas da edição anterior liberadas, a temporada arquivada nas estatísticas, o ano mais um, os participantes (EngCharity::select_teams), as rodadas e a árvore de confrontos; as posições previstas', "rule: at the season turnover the Charity Shield is rebuilt: last edition's tables freed, the season archived in the statistics, the year plus one, the entrants (EngCharity::select_teams), the rounds and the tie tree; then the predicted positions"),
         ('fra_cup_start_new_season', 'FraCup::start_new_season', '0x5bae50', 'fra_cup.cpp', 'thiscall, ret 0', 'regra: a virada da temporada da Copa da França refaz a copa: as tabelas da edição anterior liberadas, a temporada arquivada nas estatísticas, o ano mais um, os participantes (FraCup::collect_entrants), as rodadas e a árvore de confrontos; as posições previstas', "rule: at the season turnover the Coupe de France is rebuilt: last edition's tables freed, the season archived in the statistics, the year plus one, the entrants (FraCup::collect_entrants), the rounds and the tie tree; then the predicted positions"),
         ('fra_lge_cup_start_new_season', 'FraLgeCup::start_new_season', '0x5bd420', 'fra_lge_cup.cpp', 'thiscall, ret 0', 'regra: a virada da temporada da Copa da Liga francesa refaz a copa: as tabelas da edição anterior liberadas, a temporada arquivada nas estatísticas, o ano mais um, os participantes (FraLgeCup::collect_entrants), as rodadas e a árvore de confrontos; as posições previstas', "rule: at the season turnover the French League Cup is rebuilt: last edition's tables freed, the season archived in the statistics, the year plus one, the entrants (FraLgeCup::collect_entrants), the rounds and the tie tree; then the predicted positions")])
CAMPOS = [('copa +0x4c, +0xa3, +0xa7, +0xab, +0xad, +0xb1, +0x40 (a virada); valor/ponteiro; grava', 'cup +0x4c, +0xa3, +0xa7, +0xab, +0xad, +0xb1, +0x40 (the turnover); value/pointer; writes')]
