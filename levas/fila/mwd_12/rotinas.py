# The constructors of the Turkish First and Second Divisions and the Russian Premier League (tur_first.cpp,
# tur_second.cpp, rus_prm.cpp). Reader mwd (12/11/2026), round 2 (1 call each, at career creation).
# - 0x8fcf60 TurFirst::TurFirst, 0x8fe6f0 TurSecond::TurSecond: the division constructor template
#   (construct_division_league), checked instruction by instruction against SpaSecond's 0x8539a0 (discipline 0x25).
# - 0x7eaa80 RusPrm::RusPrm: the Croatian template (construct_tiebreak_league, mwd_1), checked against CroA1's 0x513e10
#   (only the vtable, the discipline 0x27 and the file change; the same box lines).
# Conventions: thiscall(this, year, record), ret 8.
# Comparers: only the original runs (compare_division_league_ctor, compare_tiebreak_league_ctor). Equal in
# cm0102-1080.exe.
# Depends on: construct_tiebreak_league / compare_tiebreak_league_ctor (mwd_1).
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('construct_tur_first', 0x8fcf60, [0x6a, 0xff, 0x68, 0xc9, 0x4f, 0x96, 0x00]),
           ('construct_tur_second', 0x8fe6f0, [0x6a, 0xff, 0x68, 0x19, 0x50, 0x96, 0x00]),
           ('construct_rus_prm', 0x7eaa80, [0x6a, 0xff, 0x68, 0x99, 0x09, 0x96, 0x00])]
_R = 'e, sem estado gravado, só a fase principal, as regras, a tabela, os jogos, a ordem, as estatísticas e as posições previstas'
_E = 'and, with no saved state, the main stage only, the rules, the table, the fixtures, the order, the statistics and the predicted positions'
DOCS = ('os construtores das divisões turcas e da liga principal russa', 'the constructors of the Turkish divisions and the Russian Premier League',
        [('construct_tur_first', 'TurFirst::TurFirst', '0x8fcf60', 'tur_first.cpp', 'thiscall, ret 8', 'regra: a Primeira Divisão turca ao criar a carreira: a regra de disciplina 0x25 (a turca) ' + _R, 'rule: the Turkish First Division at career creation: discipline rule 0x25 (the Turkish one) ' + _E),
         ('construct_tur_second', 'TurSecond::TurSecond', '0x8fe6f0', 'tur_second.cpp', 'thiscall, ret 8', 'regra: a Segunda Divisão turca ao criar a carreira: a regra de disciplina 0x25 ' + _R, 'rule: the Turkish Second Division at career creation: discipline rule 0x25 ' + _E),
         ('construct_rus_prm', 'RusPrm::RusPrm', '0x7eaa80', 'rus_prm.cpp', 'thiscall, ret 8', 'regra: a liga principal russa ao criar a carreira: a regra de disciplina 0x27 (a russa) e, sem estado gravado, nenhum par de desempate (+0xee = 0), a temporada aberta (+0x4c = 1), só a fase principal, as regras, a tabela, os jogos, a ordem, as estatísticas e as posições previstas', 'rule: the Russian Premier League at career creation: discipline rule 0x27 (the Russian one) and, with no saved state, no tie-break pairs (+0xee = 0), the season open (+0x4c = 1), the main stage only, the rules, the table, the fixtures, the order, the statistics and the predicted positions')])
CAMPOS = [('liga +4, +0x40, +0x50, +0x44, +0x30, +0x2c, +0xc, +8, +0x4c, +0xee', 'league +4, +0x40, +0x50, +0x44, +0x30, +0x2c, +0xc, +8, +0x4c, +0xee')]
