# Mutacoes (mwf_4): o ano de cada virada sobe 2 em vez de 1 (as rodadas, os sorteios e as datas da temporada
# 2002/03 mudam de ano; acusadas pelo estado no fim da carreira, 2002-08-01).
MUTACOES = [
    ('mwf4_korsupercup_ano_mais_2',
     'usa_mls_all_stars_cup_start_new_season(Cup(this), 0x66dc10, 0x9eb198, [0x134, 0x13b, 0x142], 1, false, "mwf_4 Korean Super Cup rollover") // mwf4: the turnover of the Korean Super Cup',
     'usa_mls_all_stars_cup_start_new_season(Cup(this), 0x66dc10, 0x9eb198, [0x134, 0x13b, 0x142], 2, false, "mwf_4 Korean Super Cup rollover") // mutation: the year plus 2',
     'mwf_4 Korean Super Cup rollover'),
    ('mwf4_spasuper_ano_mais_2',
     'usa_mls_all_stars_cup_start_new_season(Cup(this), 0x858aa0, 0xa82a58, [0x125, 0x12c, 0x133], 1, false, "mwf_4 Spanish Super Cup rollover") // mwf4: the turnover of the Spanish Super Cup',
     'usa_mls_all_stars_cup_start_new_season(Cup(this), 0x858aa0, 0xa82a58, [0x125, 0x12c, 0x133], 2, false, "mwf_4 Spanish Super Cup rollover") // mutation: the year plus 2',
     'mwf_4 Spanish Super Cup rollover'),
]
