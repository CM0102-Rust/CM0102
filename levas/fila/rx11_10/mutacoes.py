# Mutacoes (rx11_10), pela carreira com 'ENG IRL NIR' ate 2002-06-01:
# - o tecnico do mes da Premier dado pela Primeira Divisao (o vencedor muda);
# - a equipe da semana da Premier montada sobre o premio do jogador do ano (5).
MUTACOES = [
    ('rx11_10_tecnico_da_primeira',
     "award_month_manager(nation_award(this, 10), KEY_CLUB_COMP_IRISH_PREMIER_DIVISION); // rx11_10: the Premier's manager of the month",
     "award_month_manager(nation_award(this, 10), KEY_CLUB_COMP_IRISH_FIRST_DIVISION); // mutation",
     'rx11_10 irish month awards'),
    ('rx11_10_equipe_no_premio_errado',
     "nation_awards_team_of_the_week_of(this, award, &[4, 11]); // rx11_10: the two divisions' team awards",
     "nation_awards_team_of_the_week_of(this, award, &[5, 11]); // mutation",
     'rx11_10 irish team of the week'),
]
