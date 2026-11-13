# The Irish Premier Division (ire_prm.cpp, in leagues\): the constructor, the rules and the table.
# Whole-world front, round 2 (reader mwa, 12/11/2026), career with ENG IRL NIR to 2002-08-01. Uses mwa_9's helpers
# (construct_irish_league, set_up_irish_league_rules and their comparers): it goes in the same batch or after it.
# - 0x639bb0 (IrePrm::IrePrm, 1 call): JapJ2::JapJ2's template with the ten 5-byte slots from +0xee set to −1 before
#   the load (IrePrm's own block).
# - 0x63a900 (IrePrm::set_up_rules, 2 calls); 0x63a740 (IrePrm::create_table, 2 calls: counts the division's clubs,
#   accepts 10 or 12, ItaSerA's fill, three relegated with twelve, the prize fund).
# Conventions: see mwa_9 (the same calls).
# Comparers: the constructor only the original, with the plan; the rules side by side outside the round table; the
# table only the original, with the plan (the count, the rows, the prize and the meetings).
# Determinism: nothing reads stack, heap or an address as a value.
LIGAS = 'ENG IRL NIR'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('construct_ire_prm', 0x639bb0, [0x6a, 0xff, 0x68, 0xa9, 0xb5, 0x95, 0x00]),
           ('ire_prm_set_up_rules', 0x63a900, [0x53, 0x56, 0x8b, 0xf1, 0xbb, 0x01, 0x00, 0x00, 0x00]),
           ('ire_prm_create_table', 0x63a740, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a Premier da Irlanda: o construtor, as regras e a tabela',
        'the Irish Premier Division: the constructor, the rules and the table',
        [('construct_ire_prm', 'IrePrm::IrePrm', '0x639bb0', 'ire_prm.cpp', 'thiscall, ret 8',
          'regra: a Premier da Irlanda na criação da carreira: o ano e a competição do banco de dados, a regra de disciplina 16 (Irlanda), as suas dez vagas próprias limpas e, sem nada salvo, uma liga simples: as regras, a tabela, a tabela de jogos, a ordem, as estatísticas e as posições previstas',
          'rule: the Irish Premier Division at career creation: the year and the database competition, discipline rule 16 (Ireland), its ten own slots cleared and, with nothing saved, a plain league: the rules, the table, the fixtures, the order, the statistics and the predicted positions'),
         ('ire_prm_set_up_rules', 'IrePrm::set_up_rules', '0x63a900', 'ire_prm.cpp', 'thiscall, ret 0',
          'regra: a Premier da Irlanda: 3 pontos por vitória e 1 por empate, desempate por saldo e gols marcados; o último cai para a First Division e o penúltimo joga a repescagem (com doze clubes, três caem e um joga a repescagem); 5 reservas no banco e 3 substituições; sem liga acima',
          'rule: the Irish Premier Division: 3 points for a win and 1 for a draw, ties split by goal difference and goals for; the last goes down to the First Division and the second last plays the play-off (with twelve clubs, three go down and one plays the play-off); 5 substitutes on the bench and 3 substitutions; no division above'),
         ('ire_prm_create_table', 'IrePrm::create_table', '0x63a740', 'ire_prm.cpp', 'thiscall, ret 0',
          'regra: a tabela da Premier da Irlanda tem os clubes da divisão na ordem do banco de dados; só aceita dez (que se enfrentam quatro vezes) ou doze (três vezes, com três rebaixados e um na repescagem); cheia, o fundo de prêmios por uma cota de 30.000 por posição, pago pela classificação no fim da temporada',
          "rule: the Irish Premier Division table takes the division's clubs in database order; only ten (meeting four times) or twelve (three times, with three relegated and one in the play-off) are accepted; when full, the prize fund for a 30,000 share per place, paid by table position at the season's end")])
CAMPOS = [('liga da Premier da Irlanda +0xee dez entradas de 5 bytes, o primeiro dword −1 na criação [inferência: estado próprio da liga]; valor; grava',
           'Irish Premier league +0xee ten 5-byte entries, the first dword −1 at creation [inference: the league\'s own state]; value; writes')]
INTERESSA = []
