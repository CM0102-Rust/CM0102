# The constructors of the Greek First and Second Divisions and the Russian First Division (gre_prm.cpp, gre_second.cpp,
# rus_first.cpp). Reader mwd (12/11/2026), round 2 (1 call each, at career creation).
# - 0x5ea410 GrePrm::GrePrm, 0x5eb2b0 GreSecond::GreSecond, 0x7e9f30 RusFirst::RusFirst: the division constructor
#   template (construct_division_league, fra_second.rs), checked instruction by instruction against SpaSecond's
#   0x8539a0: only the vtable, the discipline rule (0x20 Greece, 0x27 Russia), the file and the box lines change.
# Conventions: thiscall(this, year, record), ret 8 (the callees as in construct_division_league).
# Comparer: compare_division_league_ctor (only the original runs). Equal in cm0102-1080.exe.
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('construct_gre_prm', 0x5ea410, [0x6a, 0xff, 0x68, 0xd9, 0xa9, 0x95, 0x00]),
           ('construct_gre_second', 0x5eb2b0, [0x6a, 0xff, 0x68, 0x29, 0xaa, 0x95, 0x00]),
           ('construct_rus_first', 0x7e9f30, [0x6a, 0xff, 0x68, 0x49, 0x09, 0x96, 0x00])]
_R = 'e, sem estado gravado, só a fase principal, as regras, a tabela, os jogos, a ordem, as estatísticas e as posições previstas'
_E = 'and, with no saved state, the main stage only, the rules, the table, the fixtures, the order, the statistics and the predicted positions'
DOCS = ('os construtores das divisões grega e russa', 'the constructors of the Greek and Russian divisions',
        [('construct_gre_prm', 'GrePrm::GrePrm', '0x5ea410', 'gre_prm.cpp', 'thiscall, ret 8', 'regra: a Primeira Divisão grega ao criar a carreira: a regra de disciplina 0x20 (a grega) ' + _R, 'rule: the Greek First Division at career creation: discipline rule 0x20 (the Greek one) ' + _E),
         ('construct_gre_second', 'GreSecond::GreSecond', '0x5eb2b0', 'gre_second.cpp', 'thiscall, ret 8', 'regra: a Segunda Divisão grega ao criar a carreira: a regra de disciplina 0x20 ' + _R, 'rule: the Greek Second Division at career creation: discipline rule 0x20 ' + _E),
         ('construct_rus_first', 'RusFirst::RusFirst', '0x7e9f30', 'rus_first.cpp', 'thiscall, ret 8', 'regra: a Primeira Divisão russa ao criar a carreira: a regra de disciplina 0x27 (a russa) ' + _R, 'rule: the Russian First Division at career creation: discipline rule 0x27 (the Russian one) ' + _E)])
CAMPOS = [('liga +4, +0x40, +0x50, +0x44, +0x30, +0x2c, +0xc, +8', 'league +4, +0x40, +0x50, +0x44, +0x30, +0x2c, +0xc, +8')]
