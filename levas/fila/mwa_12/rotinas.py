# The Irish First Division (ire_first.cpp, in leagues\): the constructor, the rules and the table.
# Whole-world front, round 2 (reader mwa, 12/11/2026), career with ENG IRL NIR to 2002-08-01. Uses mwa_9's helpers
# and mwa_11's irish_division_club_count and compare_irish_counted_table: it goes in the same batch or after them.
# - 0x634c40 (IreFirst::IreFirst, 1 call): EngFirst::EngFirst's template (0x571680, same.py: only the vtable, the
#   discipline rule, the two routines, the file and the last line).
# - 0x635aa0 (IreFirst::set_up_rules, 2 calls); 0x635910 (IreFirst::create_table, 2 calls: counts the division's clubs,
#   accepts 9 to 12, ItaSerA's fill, no prize fund).
# Conventions: see mwa_9 (the same calls).
# Comparers: the constructor only the original, with the plan; the rules side by side outside the round table; the
# table only the original, with the plan.
# Determinism: nothing reads stack, heap or an address as a value.
LIGAS = 'ENG IRL NIR'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('construct_ire_first', 0x634c40, [0x6a, 0xff, 0x68, 0x89, 0xb3, 0x95, 0x00]),
           ('ire_first_set_up_rules', 0x635aa0, [0x53, 0x56, 0x8b, 0xf1, 0xb2, 0x03]),
           ('ire_first_create_table', 0x635910, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a First Division da Irlanda: o construtor, as regras e a tabela',
        'the Irish First Division: the constructor, the rules and the table',
        [('construct_ire_first', 'IreFirst::IreFirst', '0x634c40', 'ire_first.cpp', 'thiscall, ret 8',
          'regra: a First Division da Irlanda na criação da carreira: o ano e a competição do banco de dados, a regra de disciplina 16 (Irlanda) e, sem nada salvo, a liga e uma fase extra (a repescagem do acesso): as regras, a tabela, a tabela de jogos, a ordem, as estatísticas e as posições previstas',
          'rule: the Irish First Division at career creation: the year and the database competition, discipline rule 16 (Ireland) and, with nothing saved, the league and one extra stage (the promotion play-off): the rules, the table, the fixtures, the order, the statistics and the predicted positions'),
         ('ire_first_set_up_rules', 'IreFirst::set_up_rules', '0x635aa0', 'ire_first.cpp', 'thiscall, ret 0',
          'regra: a First Division da Irlanda: 3 pontos por vitória e 1 por empate, desempate por saldo e gols marcados; o campeão sobe para a Premier e o vice joga a repescagem; sem rebaixamento (não há divisão abaixo); 5 reservas no banco e 3 substituições',
          'rule: the Irish First Division: 3 points for a win and 1 for a draw, ties split by goal difference and goals for; the champion goes up to the Premier Division and the runner-up plays the promotion play-off; no relegation (no division below); 5 substitutes on the bench and 3 substitutions'),
         ('ire_first_create_table', 'IreFirst::create_table', '0x635910', 'ire_first.cpp', 'thiscall, ret 0',
          'regra: a tabela da First Division da Irlanda tem os clubes da divisão na ordem do banco de dados; só aceita de nove a doze: nove ou dez se enfrentam quatro vezes, onze ou doze três vezes; sem fundo de prêmios',
          "rule: the Irish First Division table takes the division's clubs in database order; only nine to twelve are accepted: nine or ten meet four times, eleven or twelve three times; no prize fund")])
CAMPOS = []
INTERESSA = []
