# The Northern Irish First Division (nir_first.cpp, in leagues\): the constructor, the rules and the table.
# Whole-world front, round 2 (reader mwa, 12/11/2026), career with ENG IRL NIR to 2002-08-01. Uses mwa_9's helpers
# (construct_irish_league, set_up_irish_league_rules and their comparers): it goes in the same batch or after it.
# - 0x78b010 (NirFirst::NirFirst, 1 call): EngFirst::EngFirst's template (0x571680, same.py: only the vtable, the
#   discipline rule, the two routines, the file and the last line).
# - 0x78bd70 (NirFirst::set_up_rules, 2 calls); 0x78bc50 (NirFirst::create_table, 2 calls: ItaSerA's fill, the prize
#   fund, no TV money).
# Conventions: see mwa_9 (the same calls).
# Comparers: the constructor only the original, with the plan; the rules side by side outside the round table; the
# table only the original, with the plan.
# Determinism: nothing reads stack, heap or an address as a value.
LIGAS = 'ENG IRL NIR'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('construct_nir_first', 0x78b010, [0x6a, 0xff, 0x68, 0xa9, 0xe3, 0x95, 0x00]),
           ('nir_first_set_up_rules', 0x78bd70, [0x53, 0x56, 0x8b, 0xf1, 0xba, 0x04, 0x00, 0x00, 0x00]),
           ('nir_first_create_table', 0x78bc50, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a First Division da Irlanda do Norte: o construtor, as regras e a tabela',
        'the Northern Irish First Division: the constructor, the rules and the table',
        [('construct_nir_first', 'NirFirst::NirFirst', '0x78b010', 'nir_first.cpp', 'thiscall, ret 8',
          'regra: a First Division da Irlanda do Norte na criação da carreira: o ano e a competição do banco de dados, a regra de disciplina 36 (Irlanda do Norte) e, sem nada salvo, a liga e uma fase extra (a repescagem do acesso): as regras, a tabela, a tabela de jogos, a ordem, as estatísticas e as posições previstas',
          'rule: the Northern Irish First Division at career creation: the year and the database competition, discipline rule 36 (Northern Ireland) and, with nothing saved, the league and one extra stage (the promotion play-off): the rules, the table, the fixtures, the order, the statistics and the predicted positions'),
         ('nir_first_set_up_rules', 'NirFirst::set_up_rules', '0x78bd70', 'nir_first.cpp', 'thiscall, ret 0',
          'regra: a First Division da Irlanda do Norte: dez clubes que se enfrentam quatro vezes, 3 pontos por vitória e 1 por empate, desempate por saldo e gols marcados; o campeão sobe para a Premier e o vice joga a repescagem; sem rebaixamento (não há divisão abaixo); 3 reservas no banco e 3 substituições',
          'rule: the Northern Irish First Division: ten clubs meeting four times, 3 points for a win and 1 for a draw, ties split by goal difference and goals for; the champion goes up to the Premier Division and the runner-up plays the promotion play-off; no relegation (no division below); 3 substitutes on the bench and 3 substitutions'),
         ('nir_first_create_table', 'NirFirst::create_table', '0x78bc50', 'nir_first.cpp', 'thiscall, ret 0',
          'regra: a tabela da First Division da Irlanda do Norte tem dez clubes, os da divisão na ordem do banco de dados; cheia, o fundo de prêmios por uma cota de 300 por posição, pago pela classificação no fim da temporada',
          "rule: the Northern Irish First Division table has ten clubs, the division's in database order; when full, the prize fund for a 300 share per place, paid by table position at the season's end")])
CAMPOS = []
INTERESSA = []
