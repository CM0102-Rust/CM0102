# Mutacoes (sx05b_14), pela carreira com a Turquia ate 2002-06-01 (os premios de 30 de maio).
# - o jogador do ano com a reputacao nacional de 300;
# - sem a equipe do ano da segunda divisao;
# - os premios em 30 de abril.
MUTACOES = [
    ('sx05b_14_reputacao_menor',
     '        AwardSpec::Year([n, 0, k(KEY_STAFF_COMP_TURKISH_PLAYER_OF_THE_YEAR), 3000, 1500,',
     '        AwardSpec::Year([n, 0, k(KEY_STAFF_COMP_TURKISH_PLAYER_OF_THE_YEAR), 300, 1500, // mutation\n           ',
     'sx05b_14 turkish awards built'),
    ('sx05b_14_sem_segunda',
     '        award_team_of_season(this, KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_A, nation_award(this, 5));',
     '        // mutation: no second division team of the year',
     'sx05b_14 turkish awards day'),
    ('sx05b_14_abril',
     '    if awards_day_is(today, 0x1e, 4) {\n        covered!("sx05b_14 turkish awards day");',
     '    if awards_day_is(today, 0x1e, 3) { // mutation\n        covered!("sx05b_14 turkish awards day");'),
]
