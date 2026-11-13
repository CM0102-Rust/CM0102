# The season turnover of the Russian and Turkish cups and the Greek Super Cup (rus_cup.cpp, tur_cup.cpp, gre_super.cpp).
# Reader mwd (12/11/2026), round 2 (1 call each, the turnover of summer 2002).
# - 0x7e9d10 RusCup::start_new_season, 0x8fcdb0 TurCup::start_new_season: the template of start_new_cup_season
#   (bra_cup.rs; checked instruction by instruction against JapEmpCup's 0x662950).
# - 0x5ec6b0 GreSuper::start_new_season: the template of usa_mls_all_stars_cup_start_new_season without the predicted
#   positions (checked against JapSuper's 0x6664e0: the same, without the virtual +0x5c).
# Conventions: thiscall, ret 0. Comparer: compare_cup_new_season (only the original; the year +1). Equal in
# cm0102-1080.exe.
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('rus_cup_start_new_season', 0x7e9d10, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('tur_cup_start_new_season', 0x8fcdb0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('gre_super_start_new_season', 0x5ec6b0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
_R = 'as tabelas da edição anterior liberadas, as estatísticas arquivadas, o ano seguinte, os inscritos, as rodadas, a árvore de jogos e as posições previstas'
_E = "last edition's tables freed, the statistics archived, the next year, the entrants, the rounds, the fixture tree and the predicted positions"
DOCS = ('a virada das copas da Rússia e da Turquia e da Supercopa da Grécia', 'the season turnover of the Russian and Turkish cups and the Greek Super Cup',
        [('rus_cup_start_new_season', 'RusCup::start_new_season', '0x7e9d10', 'rus_cup.cpp', 'thiscall, ret 0', 'regra: a virada da Copa da Rússia: ' + _R, 'rule: the Russian Cup season turnover: ' + _E),
         ('tur_cup_start_new_season', 'TurCup::start_new_season', '0x8fcdb0', 'tur_cup.cpp', 'thiscall, ret 0', 'regra: a virada da Copa da Turquia: ' + _R, 'rule: the Turkish Cup season turnover: ' + _E),
         ('gre_super_start_new_season', 'GreSuper::start_new_season', '0x5ec6b0', 'gre_super.cpp', 'thiscall, ret 0',
          'regra: a virada da Supercopa da Grécia: as tabelas da edição anterior liberadas, as estatísticas arquivadas, o ano seguinte, os dois clubes (o campeão da liga e o da copa), a rodada e a árvore de jogos; sem posições previstas',
          "rule: the Greek Super Cup season turnover: last edition's tables freed, the statistics archived, the next year, the two clubs (the league champion and the cup winner), the round and the fixture tree; no predicted positions")])
CAMPOS = [('copa +0x4c, +0xa7, +0xa3, +0xad, +8, +0x40, +0xab, +0xb1', 'cup +0x4c, +0xa7, +0xa3, +0xad, +8, +0x40, +0xab, +0xb1')]
