# Mutations (mwe_4), in the rules set when the leagues are created (and at each turnover): the Superettan
# relegating 4 instead of 3 (who goes down at the end of 2001); the Portuguese Premier Division breaking ties by goal
# difference first (code 1 instead of 4: the table order with equal points in the 2001/02 season); the Liga de Honra
# promoting 2 instead of 3 (the table status bytes and who goes up at the turnover in June 2002).
MUTACOES = [
    ('mwe4_superettan_rebaixa_4',
     'v.push((league::RELEGATED_COUNT, 1, 3)); // mwe4: the Superettan relegates 3',
     'v.push((league::RELEGATED_COUNT, 1, 4)); // mutation: 4 relegated',
     'mwe_4 superettan rules'),
    ('mwe4_portuguesa_desempate_pelo_saldo',
     'v.push((league::TIEBREAKS, 1, 4)); // mwe4: the Portuguese Premier breaks ties head to head first',
     'v.push((league::TIEBREAKS, 1, 1)); // mutation: goal difference first',
     'mwe_4 portuguese premier rules'),
    ('mwe4_liga_de_honra_sobe_2',
     'v.push((league::PROMOTED_COUNT, 1, 3)); // mwe4: the Liga de Honra promotes 3',
     'v.push((league::PROMOTED_COUNT, 1, 2)); // mutation: 2 promoted',
     'mwe_4 liga de honra rules'),
]
