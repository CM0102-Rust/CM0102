# Mutacao (rx08_6), ENG ate 2005-12-31: as selecoes do bolo nao entram nesta competicao (set_current_competition
# omitido): as convocacoes e os choques de data das selecoes europeias mudam de 2004 a 2005 (a carreira acusa; o
# comparador so confere a ordem do bolo).
MUTACOES = [
    ('rx08_6_sem_competicao',
     'exe::nationalteams_set_current_competition(read_usize(G_NATIONAL_TEAMS), nation.nation(), league.competition().addr()); // rx08_6: the nation plays this one',
     'let _ = nation; // rx08_6 mutation: the nation does not join this one',
     'rx08_6 european predicted positions'),
]
