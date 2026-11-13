# Leitor mwc (12/11/2026), carreira do mundo inteiro (BEL HOL GER DEN) ate 2002-08-01: cada virada roda uma vez
# (contagem 1 na cobertura do mundo), no fim da temporada 2001/02. thiscall ret 0 (mov ecx; chamada pela tabela
# virtual +8). O corpo confere instrucao por instrucao com o JapSuper::start_new_season (usa_mls_all_stars_cup_
# start_new_season, ja no motor); nao depende do mwc_1. A mutacao (o ano +2) so o comparador pega (ele confere o ano +1).
# 
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('bel_super_start_new_season', 0x421540, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('hol_super_start_new_season', 0x5f8e40, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('ger_lge_cup_start_new_season', 0x5dc9e0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a virada das Supercopas da Bélgica e da Holanda e da Copa da Liga alemã',
        'the season turn of the Belgian and Dutch Super Cups and the German League Cup',
        [('bel_super_start_new_season', 'BelSuper::start_new_season', '0x421540', 'bel_super.cpp', 'thiscall, ret 0', 'regra: na virada a Supercopa da Bélgica é refeita: as tabelas da edição anterior liberadas, o histórico arquivado, o ano +1, os participantes (BelSuper::select_teams), as rodadas, a árvore de confrontos e as posições previstas', "rule: at the season turn the Belgian Super Cup is rebuilt: last edition's tables freed, the history archived, the year +1, the entrants (BelSuper::select_teams), the rounds, the tie tree and the predicted positions"),
         ('hol_super_start_new_season', 'HolSuper::start_new_season', '0x5f8e40', 'hol_super.cpp', 'thiscall, ret 0', 'regra: na virada a Supercopa da Holanda é refeita: as tabelas da edição anterior liberadas, o histórico arquivado, o ano +1, os participantes (HolSuper::select_teams), as rodadas, a árvore de confrontos (sem posições previstas)', "rule: at the season turn the Dutch Super Cup is rebuilt: last edition's tables freed, the history archived, the year +1, the entrants (HolSuper::select_teams), the rounds, the tie tree (no predicted positions)"),
         ('ger_lge_cup_start_new_season', 'GerLgeCup::start_new_season', '0x5dc9e0', 'ger_lge_cup.cpp', 'thiscall, ret 0', 'regra: na virada a Copa da Liga alemã é refeita: as tabelas da edição anterior liberadas, o histórico arquivado, o ano +1, os participantes (GerLgeCup::collect_entrants), as rodadas, a árvore de confrontos e as posições previstas', "rule: at the season turn the German League Cup is rebuilt: last edition's tables freed, the history archived, the year +1, the entrants (GerLgeCup::collect_entrants), the rounds, the tie tree and the predicted positions")])
