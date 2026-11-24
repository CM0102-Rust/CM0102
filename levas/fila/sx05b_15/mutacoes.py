# Mutacoes (sx05b_15), pela carreira com a Turquia ate 2002-06-01: a equipe da semana da segunda divisao nunca
# montada (a do ano, de 30 de maio, sai de outros candidatos).
MUTACOES = [
    ('sx05b_15_so_primeira',
     '    nation_awards_team_of_the_week_of(this, award, &[2, 4]);',
     '    nation_awards_team_of_the_week_of(this, award, &[2]); // mutation'),
]
