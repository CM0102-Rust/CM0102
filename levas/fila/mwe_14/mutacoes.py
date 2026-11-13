# Mutations (mwe_14), at the end of the 2001 season (the Finnish, Norwegian and Swedish turnovers): the top divisions
# two years ahead (one helper for the three: the 2002 schedule and year change); the Norwegian First and Second
# Division clubs not swapped (who plays in the 2002 First Division changes); the Ykkonen not turned over by the
# Veikkausliiga (its 2002 season is not built).
MUTACOES = [
    ('mwe14_primeiras_divisoes_dois_anos',
     'this.set_year(this.year().wrapping_add(1)); // mwe14: the top division next year',
     'this.set_year(this.year().wrapping_add(2)); // mutation: two years ahead',
     'mwe_14 swedish turnover'),
    ('mwe14_sem_troca_norueguesa',
     'call_on_league(0x792960, this); // mwe14: the Norwegian First and Second Division clubs swapped',
     '// mutation: without the swap',
     'mwe_14 norwegian turnover'),
    ('mwe14_sem_virada_da_ykkonen',
     'bra_nat_first_call_virtual_no_args(comp_by_global(KEY_CLUB_COMP_FINNISH_FIRST_DIVISION), comp_vtable::START_NEW_SEASON); // mwe14: the Ykkonen turnover',
     '// mutation: without the Ykkonen turnover',
     'mwe_14 finnish turnover'),
]
