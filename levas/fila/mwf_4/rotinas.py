# As viradas de temporada de duas supercopas, no molde das copas (usa_mls_all_stars_start_new_season / usa_mls_all_stars_cup_start_new_season, sem posições previstas, já em Rust desde a 282a: só mudam o arquivo
# das caixas, as linhas e a rotina dos participantes; conferido instrução por instrução pelo norm.py do leitor mwf).
# Leitor mwf (12/11/2026), rodada 2 (o mundo inteiro até 2002-08-01): cada rotina roda 1 vez (a virada de meados de
# 2002; docs/cobertura_mundo.txt).
# Convenções: virtual +0x08 do objeto (mov ecx, esi; call [eax+8] no RulingBody / na liga que vira) -> thiscall, ret 0.
# Comparador: só a original (libera e refaz a copa), com o ano +1 (compare_cup_new_season).
LIGAS = 'ENG KOR ARG ITA FRA ESP AUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('kor_super_cup_start_new_season', 0x66de90, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('spa_super_start_new_season', 0x858cd0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as viradas das supercopas da Coreia e da Espanha',
        'the season turnovers of the Korean and Spanish Super Cups',
        [('kor_super_cup_start_new_season', 'KorSuperCup::start_new_season', '0x66de90', 'kor_super_cup.cpp', 'thiscall, ret 0', 'regra: a virada da temporada da Supercopa da Coreia refaz a copa: as tabelas da edição anterior liberadas, a temporada arquivada nas estatísticas, o ano mais um, os participantes (KorSuperCup::create_table), as rodadas e a árvore de confrontos; sem posições previstas', "rule: at the season turnover the Korean Super Cup is rebuilt: last edition's tables freed, the season archived in the statistics, the year plus one, the entrants (KorSuperCup::create_table), the rounds and the tie tree; no predicted positions"),
         ('spa_super_start_new_season', 'SpaSuper::start_new_season', '0x858cd0', 'spa_super.cpp', 'thiscall, ret 0', 'regra: a virada da temporada da Supercopa da Espanha refaz a copa: as tabelas da edição anterior liberadas, a temporada arquivada nas estatísticas, o ano mais um, os participantes (SpaSuper::select_teams), as rodadas e a árvore de confrontos; sem posições previstas', "rule: at the season turnover the Spanish Super Cup is rebuilt: last edition's tables freed, the season archived in the statistics, the year plus one, the entrants (SpaSuper::select_teams), the rounds and the tie tree; no predicted positions")])
CAMPOS = [('copa +0x4c, +0xa3, +0xa7, +0xab, +0xad, +0xb1, +0x40 (a virada); valor/ponteiro; grava', 'cup +0x4c, +0xa3, +0xa7, +0xab, +0xad, +0xb1, +0x40 (the turnover); value/pointer; writes')]
