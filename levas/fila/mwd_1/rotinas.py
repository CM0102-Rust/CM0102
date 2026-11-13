# The Croatian leagues (cro_a1.cpp, cro_a2a.cpp, cro_a2b.cpp): the three constructors. Reader mwd (12/11/2026), round 2
# (the whole world, 1 call each, at career creation).
# - 0x513e10 CroA1::CroA1, 0x515310 CroA2a::CroA2a, 0x516430 CroA2b::CroA2b: the same 544-byte template (checked
#   instruction by instruction: only the vtable and the source file change; the box lines are the same). Like the
#   division constructors (construct_division_league, fra_second.rs), plus +0xee (the tie-break pair count) = 0 and
#   +0x4c (season closed) = 1 in a new league.
# Conventions checked at the original's call sites and inside: thiscall(this, year, record), ret 8; 0x682200
# League::League (thiscall, ret 0), 0x687b10 (push 1 -> thiscall, ret 4), the rules and table (direct calls, thiscall,
# ret 0), 0x6835c0 (thiscall, ret 0), 0x6827d0 (push 0 -> thiscall, ret 4), 0x944cf1 (cdecl), 0x49ee70 (push league ->
# thiscall, ret 4), 0x68a850 (thiscall, ret 0).
# Comparer: only the original runs (a constructor can't run twice); return, vtable, year, record, discipline and the
# new-league fields checked. Equal in cm0102-1080.exe.
# Depends on: LeagueDivision (fra_second.rs). construct_tiebreak_league is reused by mwd items of RusPrm.
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('construct_cro_a1', 0x513e10, [0x6a, 0xff, 0x68, 0x59, 0x73, 0x95, 0x00]),
           ('construct_cro_a2a', 0x515310, [0x6a, 0xff, 0x68, 0xe9, 0x73, 0x95, 0x00]),
           ('construct_cro_a2b', 0x516430, [0x6a, 0xff, 0x68, 0x59, 0x74, 0x95, 0x00])]
_R = 'regra: a liga croata ao criar a carreira: a regra de disciplina 0x21 (a croata) e, sem estado gravado, nenhum par de desempate (+0xee = 0), a temporada aberta (+0x4c = 1), só a fase principal, as regras, a tabela, os jogos, a ordem, as estatísticas e as posições previstas'
_E = 'rule: the Croatian league at career creation: discipline rule 0x21 (the Croatian one) and, with no saved state, no tie-break pairs (+0xee = 0), the season open (+0x4c = 1), the main stage only, the rules, the table, the fixtures, the order, the statistics and the predicted positions'
DOCS = ('as ligas croatas: os construtores', 'the Croatian leagues: the constructors',
        [('construct_cro_a1', 'CroA1::CroA1', '0x513e10', 'cro_a1.cpp', 'thiscall, ret 8', _R.replace('a liga croata', 'a Primeira Divisão croata'), _E.replace('the Croatian league', 'the Croatian First Division')),
         ('construct_cro_a2a', 'CroA2a::CroA2a', '0x515310', 'cro_a2a.cpp', 'thiscall, ret 8', _R.replace('a liga croata', 'a Segunda Divisão croata (grupo norte)'), _E.replace('the Croatian league', 'the Croatian Second Division (north group)')),
         ('construct_cro_a2b', 'CroA2b::CroA2b', '0x516430', 'cro_a2b.cpp', 'thiscall, ret 8', _R.replace('a liga croata', 'a Segunda Divisão croata (grupo sul)'), _E.replace('the Croatian league', 'the Croatian Second Division (south group)'))])
CAMPOS = [('liga +4 competição, +0x40 ano, +0x50 disciplina, +0x44, +0x30, +0x2c, +0xc, +0x4c, +0xee pares de desempate, +8 estatísticas', 'league +4 competition, +0x40 year, +0x50 discipline, +0x44, +0x30, +0x2c, +0xc, +0x4c, +0xee tie-break pairs, +8 statistics')]
