# The season turnover of three national cups (cro_cup.cpp, pol_cup.cpp, pol_lge_cup.cpp). Reader mwd (12/11/2026), round
# 2 (1 call each, the turnover of summer 2002).
# - 0x517d60 CroCup::start_new_season, 0x7c88a0 PolCup::start_new_season, 0x7cafe0 PolLgeCup::start_new_season: the
#   template of start_new_cup_season (bra_cup.rs; checked instruction by instruction against JapEmpCup's 0x662950: only
#   the file, the box lines and the entrants routine change).
# Conventions: thiscall, ret 0; the entrants by direct call (thiscall, ret 0), the virtuals +0x8c, +0x94 and +0x5c.
# Comparer: compare_cup_new_season (only the original; the year +1). Equal in cm0102-1080.exe.
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('cro_cup_start_new_season', 0x517d60, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('pol_cup_start_new_season', 0x7c88a0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('pol_lge_cup_start_new_season', 0x7cafe0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
_R = 'as tabelas da edição anterior liberadas, as estatísticas arquivadas, o ano seguinte, os inscritos, as rodadas, a árvore de jogos e as posições previstas'
_E = "last edition's tables freed, the statistics archived, the next year, the entrants, the rounds, the fixture tree and the predicted positions"
DOCS = ('a virada das copas da Croácia, da Polônia e da Liga polonesa', 'the season turnover of the Croatian, Polish and Polish League cups',
        [('cro_cup_start_new_season', 'CroCup::start_new_season', '0x517d60', 'cro_cup.cpp', 'thiscall, ret 0', 'regra: a virada da Copa da Croácia: ' + _R, 'rule: the Croatian Cup season turnover: ' + _E),
         ('pol_cup_start_new_season', 'PolCup::start_new_season', '0x7c88a0', 'pol_cup.cpp', 'thiscall, ret 0', 'regra: a virada da Copa da Polônia: ' + _R, 'rule: the Polish Cup season turnover: ' + _E),
         ('pol_lge_cup_start_new_season', 'PolLgeCup::start_new_season', '0x7cafe0', 'pol_lge_cup.cpp', 'thiscall, ret 0', 'regra: a virada da Copa da Liga polonesa: ' + _R, 'rule: the Polish League Cup season turnover: ' + _E)])
CAMPOS = [('copa +0x4c, +0xa7, +0xa3, +0xad, +8, +0x40, +0xab, +0xb1', 'cup +0x4c, +0xa7, +0xa3, +0xad, +8, +0x40, +0xab, +0xb1')]
