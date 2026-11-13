# Mutacoes (mwa_10), na First Division da Irlanda do Norte, carreira ENG IRL NIR ate 2002-08-01:
# - o construtor monta a liga do ano seguinte;
# - dois sobem direto em vez de um (a virada de 2002 muda as divisoes);
# - a cota do premio dez vezes maior [se escapar: o premio so pesa na virada; o comparador da tabela acusa].
MUTACOES = [
    ('mwa10_first_ano_seguinte',
     'construct_irish_league(League(this), year, record, &NORTHERN_IRISH_FIRST_CONSTRUCTOR, "mwa_10 northern irish first new")',
     'construct_irish_league(League(this), year + 1, record, &NORTHERN_IRISH_FIRST_CONSTRUCTOR, "mwa_10 northern irish first new")',
     'mwa_10 northern irish first new'),
    ('mwa10_dois_sobem',
     '(league::PROMOTED_COUNT, 1, 1), (league::PROMOTION_PLAYOFF_COUNT, 1, 1), (league::RELEGATION_PLAYOFF_COUNT, 1, 0), (league::RELEGATED_COUNT, 1, 0), // mwa10: one up, one to the play-off',
     '(league::PROMOTED_COUNT, 1, 2), (league::PROMOTION_PLAYOFF_COUNT, 1, 1), (league::RELEGATION_PLAYOFF_COUNT, 1, 0), (league::RELEGATED_COUNT, 1, 0), // mutation: two up',
     'mwa_10 northern irish first rules'),
    ('mwa10_cota_dez_vezes',
     'set_prize_fund(League(this), 0x12c); // mwa10: the Northern Irish First quota',
     'set_prize_fund(League(this), 0xbb8); // mutation: ten times the quota',
     'mwa_10 northern irish first table'),
]
