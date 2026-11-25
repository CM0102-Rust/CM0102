# Mutacao (rx11_8), pela carreira com as 8 ligas ate 2002-08-01: a equipe da semana da Primeira Divisao francesa
# montada sobre o premio da equipe do ano (7) no lugar do da semana (6): a da semana fica vazia e a do ano ganha
# escolhas toda semana.
MUTACOES = [
    ('rx11_8_equipe_do_ano',
     "nation_awards_team_of_the_week_of(this, award, &[6, 11, 16]); // rx11_8: the three divisions' team awards",
     "nation_awards_team_of_the_week_of(this, award, &[7, 11, 16]); // mutation",
     'rx11_8 french team of the week'),
]
