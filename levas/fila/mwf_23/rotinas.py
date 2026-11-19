# The constructors of the Argentine Premier and Second Divisions (arg_prm.cpp, arg_second.cpp). Reader mwf
# (19/11/2026), whole-world round, continuation: each runs once, at career creation (docs/cobertura_mundo.txt).
# - 0x404250 ArgPrm::ArgPrm: its own order (rules, table, fixtures, statistics, the two half-season stages, the sort,
#   the relegation averages, League::set_predicted_positions), read instruction by instruction.
# - 0x406bf0 ArgSecond::ArgSecond: the Brazilian state leagues' constructor (BraRegMinasGerais, cqt_2) with discipline
#   rule 3, six stages and ArgSecond::init_relegation_averages (0x4093f0) before its own set_predicted_positions
#   (0x409fe0); written as its own body (the template has no slot for the averages).
# The routines they call by address (set_up_rules, create_table, create_stages, init_relegation_averages,
# set_predicted_positions of both) stay in the original.
# Conventions: thiscall, ret 8 (0x682200 League::League thiscall ret 0; 0x687b10 push 1 -> thiscall ret 4; the league
# routines thiscall ret 0; 0x6835c0 thiscall ret 0; 0x6827d0 push 0 -> thiscall ret 4; 0x944cf1 cdecl; 0x49ee70 push
# league -> thiscall ret 4; 0x68a850 thiscall ret 0).
# Comparers: only the original runs (it allocates and reads the save file); the plan checked (compare_argentine_league_ctor).
# No mutation: the comparers run only the original, and the steps the copy could skip (the averages, the stages) would
# crash the career instead of changing its result.
LIGAS = 'ENG KOR ARG ITA FRA ESP AUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('construct_arg_prm', 0x404250, [0x6a, 0xff, 0x68, 0x69, 0x35, 0x95, 0x00]),
           ('construct_arg_second', 0x406bf0, [0x6a, 0xff, 0x68, 0xe9, 0x35, 0x95, 0x00])]
DOCS = ('os construtores da Primeira e da Segunda Divisão argentinas', 'the constructors of the Argentine Premier and Second Divisions',
        [('construct_arg_prm', 'ArgPrm::ArgPrm', '0x404250', 'arg_prm.cpp', 'thiscall, ret 8',
          'regra: a Primeira Divisão argentina ao criar a carreira: a regra de disciplina 3 e, sem estado gravado, importância 15, duas fases (Apertura e Clausura), as regras, a tabela, os jogos, as estatísticas, as duas fases de meio campeonato, a ordem, as médias do rebaixamento e as posições previstas',
          'rule: the Argentine Premier Division at career creation: discipline rule 3 and, with no saved state, importance 15, two stages (Apertura and Clausura), the rules, the table, the fixtures, the statistics, the two half-season stages, the order, the relegation averages and the predicted positions'),
         ('construct_arg_second', 'ArgSecond::ArgSecond', '0x406bf0', 'arg_second.cpp', 'thiscall, ret 8',
          'regra: a Segunda Divisão argentina (Nacional B) ao criar a carreira: a regra de disciplina 3 e, sem estado gravado, importância 15, seis fases (as zonas, a copa do acesso e as repescagens), as regras, a tabela, os jogos, a ordem, as estatísticas, as médias do rebaixamento e as suas posições previstas',
          'rule: the Argentine Second Division (Nacional B) at career creation: discipline rule 3 and, with no saved state, importance 15, six stages (the zones, the promotion cup and the play-offs), the rules, the table, the fixtures, the order, the statistics, the relegation averages and its own predicted positions')])
CAMPOS = [('liga +0x2c e +0xc (fases), +0x50 (disciplina), +0x51 (importância), +8 (estatísticas)',
           'league +0x2c and +0xc (stages), +0x50 (discipline), +0x51 (importance), +8 (statistics)')]
