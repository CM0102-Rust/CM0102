# Mutations (mwe_15), at the Portuguese turnover in the summer of 2002: without the swap between the Premier Division
# and the Liga de Honra (who plays in each division in 2002/03 changes); the Premier Division two years ahead (its year
# and the 2002/03 schedule change).
MUTACOES = [
    ('mwe15_sem_troca_primeira_e_honra',
     'exe::league_promote_and_relegate(this, comp_by_global(KEY_CLUB_COMP_PORTUGUESE_PREMIER_DIVISION), comp_by_global(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION), 1, 1, -1, -1); // mwe15: the Premier Division and the Liga de Honra',
     '// mutation: without the swap',
     'mwe_15 portuguese promotion and relegation'),
    ('mwe15_primeira_portuguesa_dois_anos',
     'this.set_year(this.year().wrapping_add(1)); // mwe15: the Premier Division next year',
     'this.set_year(this.year().wrapping_add(2)); // mutation: two years ahead',
     'mwe_15 portuguese turnover'),
]
