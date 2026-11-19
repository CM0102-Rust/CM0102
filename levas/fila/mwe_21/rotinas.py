# The Portuguese Super Cup constructor (por_super.cpp; with mwe_17 the file is whole in Rust). Reader mwe (19/11/2026),
# whole-world round, continuation: runs once, at career creation (docs/cobertura_mundo.txt).
# - 0x7d2e40 PorSuper::PorSuper: construct_nation_cup (batch 300), the HolSuper body checked instruction by instruction
#   (no bench, discipline rule 23, the clubs PorSuper::select_teams 0x7d3260 of mwe_17; entrants and tie tree fail on a
#   negative value; no prize fund, no predicted positions).
# Conventions: thiscall, ret 8 (0x518640 Cup::Cup thiscall ret 0; 0x51fc00 push 1 -> thiscall ret 4; 0x518790 thiscall
# ret 0; 0x944cf1 cdecl; 0x49ee70 push this -> thiscall ret 4).
# Comparer: only the original runs (compare_nation_cup_ctor). No mutation: the comparer runs only the original and the
# fields the copy writes are the shared template's (already mutated in batch 300).
LIGAS = 'ENG SWE FIN NOR POR'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('construct_por_super', 0x7d2e40, [0x6a, 0xff, 0x68, 0x39, 0xfb, 0x95, 0x00])]
DOCS = ('o construtor da Supertaça portuguesa', 'the Portuguese Super Cup constructor',
        [('construct_por_super', 'PorSuper::PorSuper', '0x7d2e40', 'por_super.cpp', 'thiscall, ret 8',
          'regra: a Supertaça portuguesa nasce com a regra de disciplina 23; sem estado gravado, os dois clubes, a tabela de rodadas, a árvore de jogos e as estatísticas, sem posições previstas',
          'rule: the Portuguese Super Cup is born with discipline rule 23; with no saved state, its two clubs, the round table, the tie tree and the statistics, no predicted positions')])
CAMPOS = [('copa +0x50 (disciplina)', 'cup +0x50 (discipline)')]
