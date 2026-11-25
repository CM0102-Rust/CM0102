# Mutacoes (rx12_10), pela carreira com a Belgica, a Holanda e a Dinamarca ate 2002-08-01: uma equipe da semana
# nunca montada (as reputacoes da selecao da semana deixam de subir).
MUTACOES = [
    ('rx12_10_belgica_sem_terceira',
     '    nation_awards_team_of_the_week_of(this, award, &[2, 6, 9]);',
     '    nation_awards_team_of_the_week_of(this, award, &[2, 6]); // mutation'),
    ('rx12_10_holanda_sem_segunda',
     '    nation_awards_team_of_the_week_of(this, award, &[3, 7]);',
     '    nation_awards_team_of_the_week_of(this, award, &[3]); // mutation'),
    ('rx12_10_dinamarca_sem_primeira',
     '    nation_awards_team_of_the_week_of(this, award, &[1, 4, 7]);',
     '    nation_awards_team_of_the_week_of(this, award, &[4, 7]); // mutation'),
]
