# Mutacoes (rx12_13), pela carreira com a Grecia e a Croacia ate 2002-08-01: uma equipe da semana nunca montada (as
# reputacoes da selecao da semana deixam de subir).
MUTACOES = [
    ('rx12_13_grecia_so_primeira',
     '    nation_awards_team_of_the_week_of(this, award, &[2, 6]); // rx12_13: the Greek teams of the week',
     '    nation_awards_team_of_the_week_of(this, award, &[2]); // mutation'),
    ('rx12_13_croacia_sem_primeira',
     '    nation_awards_team_of_the_week_of(this, award, &[0, 3, 6]); // rx12_13: the Croatian teams of the week',
     '    nation_awards_team_of_the_week_of(this, award, &[3, 6]); // mutation'),
]
