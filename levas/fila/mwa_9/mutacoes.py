# Mutacoes (mwa_9), na Premier da Irlanda do Norte, carreira ENG IRL NIR ate 2002-08-01:
# - o construtor monta a liga do ano seguinte (a temporada 2001-02 nao e jogada);
# - dois rebaixados em vez de um (a virada de 2002 muda as divisoes);
# - a cota do premio dez vezes maior (o dinheiro dos clubes no fim da temporada) [se escapar: o premio so e pago na
#   virada e o caixa dos clubes da Irlanda do Norte pode nao pesar antes de agosto; o comparador da tabela acusa].
MUTACOES = [
    ('mwa9_premier_ano_seguinte',
     'construct_irish_league(League(this), year, record, &NORTHERN_IRISH_PREMIER_CONSTRUCTOR, "mwa_9 northern irish premier new")',
     'construct_irish_league(League(this), year + 1, record, &NORTHERN_IRISH_PREMIER_CONSTRUCTOR, "mwa_9 northern irish premier new")',
     'mwa_9 northern irish premier new'),
    ('mwa9_dois_rebaixados',
     '(league::RELEGATION_PLAYOFF_COUNT, 1, 1), (league::RELEGATED_COUNT, 1, 1), // mwa9: one relegated, one in the play-off',
     '(league::RELEGATION_PLAYOFF_COUNT, 1, 1), (league::RELEGATED_COUNT, 1, 2), // mutation: two relegated',
     'mwa_9 northern irish premier rules'),
    ('mwa9_cota_dez_vezes',
     'set_prize_fund(League(this), 0x1f4); // mwa9: the Northern Irish Premier quota',
     'set_prize_fund(League(this), 0x1388); // mutation: ten times the quota',
     'mwa_9 northern irish premier table'),
]
