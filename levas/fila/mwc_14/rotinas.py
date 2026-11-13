# Leitor mwc (12/11/2026), carreira do mundo inteiro (BEL HOL GER DEN) ate 2002-08-01: cada virada roda uma vez
# (contagem 1 na cobertura do mundo), no fim da temporada 2001/02. thiscall ret 0 (mov ecx; chamada pela tabela
# virtual +8). O corpo confere instrucao por instrucao com o JapSuper::start_new_season (usa_mls_all_stars_cup_
# start_new_season, ja no motor); nao depende do mwc_1. A mutacao (o ano +2) so o comparador pega (ele confere o ano +1).
# 
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('bel_fa_cup_start_new_season', 0x41db90, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('den_cup_start_new_season', 0x551500, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('ger_cup_start_new_season', 0x5d9780, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a virada das copas da Bélgica, da Dinamarca e da Alemanha',
        'the season turn of the Belgian, Danish and German cups',
        [('bel_fa_cup_start_new_season', 'BelFaCup::start_new_season', '0x41db90', 'bel_fa_cup.cpp', 'thiscall, ret 0', 'regra: na virada a Copa da Bélgica é refeita: as tabelas da edição anterior liberadas, o histórico arquivado, o ano +1, os participantes (BelFaCup::collect_entrants), as rodadas, a árvore de confrontos e as posições previstas', "rule: at the season turn the Belgian Cup is rebuilt: last edition's tables freed, the history archived, the year +1, the entrants (BelFaCup::collect_entrants), the rounds, the tie tree and the predicted positions"),
         ('den_cup_start_new_season', 'DenCup::start_new_season', '0x551500', 'den_cup.cpp', 'thiscall, ret 0', 'regra: na virada a Copa da Dinamarca é refeita: as tabelas da edição anterior liberadas, o histórico arquivado, o ano +1, os participantes (DenCup::collect_entrants), as rodadas, a árvore de confrontos e as posições previstas', "rule: at the season turn the Danish Cup is rebuilt: last edition's tables freed, the history archived, the year +1, the entrants (DenCup::collect_entrants), the rounds, the tie tree and the predicted positions"),
         ('ger_cup_start_new_season', 'GerCup::start_new_season', '0x5d9780', 'ger_cup.cpp', 'thiscall, ret 0', 'regra: na virada a Copa da Alemanha é refeita: as tabelas da edição anterior liberadas, o histórico arquivado, o ano +1, os participantes (GerCup::collect_entrants), as rodadas, a árvore de confrontos e as posições previstas', "rule: at the season turn the German Cup is rebuilt: last edition's tables freed, the history archived, the year +1, the entrants (GerCup::collect_entrants), the rounds, the tie tree and the predicted positions")])
