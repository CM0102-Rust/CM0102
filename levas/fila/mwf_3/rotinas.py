# As viradas de temporada de três supercopas, no molde das copas (usa_mls_all_stars_start_new_season / usa_mls_all_stars_cup_start_new_season, sem posições previstas, já em Rust desde a 282a: só mudam o arquivo
# das caixas, as linhas e a rotina dos participantes; conferido instrução por instrução pelo norm.py do leitor mwf).
# Leitor mwf (12/11/2026), rodada 2 (o mundo inteiro até 2002-08-01): cada rotina roda 1 vez (a virada de meados de
# 2002; docs/cobertura_mundo.txt).
# Convenções: virtual +0x08 do objeto (mov ecx, esi; call [eax+8] no RulingBody / na liga que vira) -> thiscall, ret 0.
# Comparador: só a original (libera e refaz a copa), com o ano +1 (compare_cup_new_season).
LIGAS = 'ENG KOR ARG ITA FRA ESP AUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('fra_super_start_new_season', 0x5bf9a0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('ita_c1_super_start_new_season', 0x63caa0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('ita_super_start_new_season', 0x65fc90, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as viradas das supercopas da França, da Série C1 e da Itália',
        'the season turnovers of the French, Serie C1 and Italian Super Cups',
        [('fra_super_start_new_season', 'FraSuper::start_new_season', '0x5bf9a0', 'fra_super.cpp', 'thiscall, ret 0', 'regra: a virada da temporada da Supercopa da França refaz a copa: as tabelas da edição anterior liberadas, a temporada arquivada nas estatísticas, o ano mais um, os participantes (FraSuper::select_teams), as rodadas e a árvore de confrontos; sem posições previstas', "rule: at the season turnover the French Super Cup is rebuilt: last edition's tables freed, the season archived in the statistics, the year plus one, the entrants (FraSuper::select_teams), the rounds and the tie tree; no predicted positions"),
         ('ita_c1_super_start_new_season', 'ItaC1Super::start_new_season', '0x63caa0', 'ita_c1_super.cpp', 'thiscall, ret 0', 'regra: a virada da temporada da Supercopa da Série C1 refaz a copa: as tabelas da edição anterior liberadas, a temporada arquivada nas estatísticas, o ano mais um, os participantes (ItaC1Super::init_team_list), as rodadas e a árvore de confrontos; sem posições previstas', "rule: at the season turnover the Serie C1 Super Cup is rebuilt: last edition's tables freed, the season archived in the statistics, the year plus one, the entrants (ItaC1Super::init_team_list), the rounds and the tie tree; no predicted positions"),
         ('ita_super_start_new_season', 'ItaSuper::start_new_season', '0x65fc90', 'ita_super.cpp', 'thiscall, ret 0', 'regra: a virada da temporada da Supercopa da Itália refaz a copa: as tabelas da edição anterior liberadas, a temporada arquivada nas estatísticas, o ano mais um, os participantes (ItaSuper::init_team_list), as rodadas e a árvore de confrontos; sem posições previstas', "rule: at the season turnover the Italian Super Cup is rebuilt: last edition's tables freed, the season archived in the statistics, the year plus one, the entrants (ItaSuper::init_team_list), the rounds and the tie tree; no predicted positions")])
CAMPOS = [('copa +0x4c, +0xa3, +0xa7, +0xab, +0xad, +0xb1, +0x40 (a virada); valor/ponteiro; grava', 'cup +0x4c, +0xa3, +0xa7, +0xab, +0xad, +0xb1, +0x40 (the turnover); value/pointer; writes')]
