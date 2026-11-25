# Mutacoes (rx13_4), pela carreira com o grupo da Argentina ate 2002-08-01 (a equipe da semana muda a reputacao dos
# escolhidos e o mercado):
# - so a primeira divisao tem equipe da semana (a da segunda nao sai).
MUTACOES = [
    ('rx13_4_so_a_primeira',
     'nation_awards_team_of_the_week_of(this, award, &[0, 1]) // rx13_4: both divisions',
     'nation_awards_team_of_the_week_of(this, award, &[0]) // mutation: the first division only',
     'rx13_4 argentine team of the week'),
]
