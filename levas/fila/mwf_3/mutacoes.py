# Mutacoes (mwf_3): o ano de cada virada sobe 2 em vez de 1 (as rodadas, os sorteios e as datas da temporada
# 2002/03 mudam de ano; acusadas pelo estado no fim da carreira, 2002-08-01).
MUTACOES = [
    ('mwf3_frasuper_ano_mais_2',
     'usa_mls_all_stars_cup_start_new_season(Cup(this), 0x5bf770, 0x9ccc48, [0x127, 0x12e, 0x135], 1, false, "mwf_3 French Super Cup rollover") // mwf3: the turnover of the French Super Cup',
     'usa_mls_all_stars_cup_start_new_season(Cup(this), 0x5bf770, 0x9ccc48, [0x127, 0x12e, 0x135], 2, false, "mwf_3 French Super Cup rollover") // mutation: the year plus 2',
     'mwf_3 French Super Cup rollover'),
    ('mwf3_itac1super_ano_mais_2',
     'usa_mls_all_stars_cup_start_new_season(Cup(this), 0x63c9d0, 0x9ea1c8, [0x10a, 0x111, 0x118], 1, false, "mwf_3 Serie C1 Super Cup rollover") // mwf3: the turnover of the Serie C1 Super Cup',
     'usa_mls_all_stars_cup_start_new_season(Cup(this), 0x63c9d0, 0x9ea1c8, [0x10a, 0x111, 0x118], 2, false, "mwf_3 Serie C1 Super Cup rollover") // mutation: the year plus 2',
     'mwf_3 Serie C1 Super Cup rollover'),
    ('mwf3_itasuper_ano_mais_2',
     'usa_mls_all_stars_cup_start_new_season(Cup(this), 0x65fa60, 0x9ea934, [0x125, 0x12c, 0x133], 1, false, "mwf_3 Italian Super Cup rollover") // mwf3: the turnover of the Italian Super Cup',
     'usa_mls_all_stars_cup_start_new_season(Cup(this), 0x65fa60, 0x9ea934, [0x125, 0x12c, 0x133], 2, false, "mwf_3 Italian Super Cup rollover") // mutation: the year plus 2',
     'mwf_3 Italian Super Cup rollover'),
]
