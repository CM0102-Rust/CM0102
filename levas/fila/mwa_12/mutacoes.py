# Mutacoes (mwa_12), na First Division da Irlanda, carreira ENG IRL NIR ate 2002-08-01:
# - o construtor monta a liga do ano seguinte;
# - dois sobem direto em vez de um (a virada de 2002 muda as divisoes);
# - nove ou dez clubes se enfrentam tres vezes (a liga de 2001-02 tem dez [inferencia]: menos rodadas).
MUTACOES = [
    ('mwa12_first_ano_seguinte',
     'construct_irish_league(League(this), year, record, &IRISH_FIRST_CONSTRUCTOR, "mwa_12 irish first new")',
     'construct_irish_league(League(this), year + 1, record, &IRISH_FIRST_CONSTRUCTOR, "mwa_12 irish first new")',
     'mwa_12 irish first new'),
    ('mwa12_dois_sobem',
     '(league::PROMOTED_COUNT, 1, 1), (league::PROMOTION_PLAYOFF_COUNT, 1, 1), (league::RELEGATION_PLAYOFF_COUNT, 1, 0), (league::RELEGATED_COUNT, 1, 0), // mwa12: one up, one to the play-off',
     '(league::PROMOTED_COUNT, 1, 2), (league::PROMOTION_PLAYOFF_COUNT, 1, 1), (league::RELEGATION_PLAYOFF_COUNT, 1, 0), (league::RELEGATED_COUNT, 1, 0), // mutation: two up',
     'mwa_12 irish first rules'),
    ('mwa12_tres_turnos',
     '9 | 10 => Some(4), // mwa12: nine or ten clubs meet four times',
     '9 | 10 => Some(3), // mutation: three times',
     'mwa_12 irish first table'),
]
