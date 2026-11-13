# Mutacoes (mwf_2): o ano de cada virada sobe 2 em vez de 1 (as rodadas, os sorteios e as datas da temporada
# 2002/03 mudam de ano; acusadas pelo estado no fim da carreira, 2002-08-01).
MUTACOES = [
    ('mwf2_korfacup_ano_mais_2',
     'start_new_cup_season(Cup(this), 0x66aaf0, 0x9eb0e8, [0x185, 0x18c, 0x193], 1, "mwf_2 Korean FA Cup rollover") // mwf2: the turnover of the Korean FA Cup',
     'start_new_cup_season(Cup(this), 0x66aaf0, 0x9eb0e8, [0x185, 0x18c, 0x193], 2, "mwf_2 Korean FA Cup rollover") // mutation: the year plus 2',
     'mwf_2 Korean FA Cup rollover'),
]
