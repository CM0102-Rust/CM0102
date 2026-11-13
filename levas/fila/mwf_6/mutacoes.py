# Mutacoes (mwf_6): o ano de cada virada sobe 2 em vez de 1 (as rodadas, os sorteios e as datas da temporada
# 2002/03 mudam de ano; acusadas pelo estado no fim da carreira, 2002-08-01).
MUTACOES = [
    ('mwf6_engfirst_ano_mais_2',
     'rebuild_season(this, 0x572bf0, 0x9c4374, [0x29e, 0x2a5, 0x2ab, 0x2b2], 1) { // mwf6: the turnover of the English First Division',
     'rebuild_season(this, 0x572bf0, 0x9c4374, [0x29e, 0x2a5, 0x2ab, 0x2b2], 2) { // mutation: the year plus 2',
     'mwf_6 English First Division rollover'),
    ('mwf6_engsecond_ano_mais_2',
     'rebuild_season(this, 0x576630, 0x9c43e8, [0x264, 0x26b, 0x271, 0x278], 1) { // mwf6: the turnover of the English Second Division',
     'rebuild_season(this, 0x576630, 0x9c43e8, [0x264, 0x26b, 0x271, 0x278], 2) { // mutation: the year plus 2',
     'mwf_6 English Second Division rollover'),
    ('mwf6_engthird_ano_mais_2',
     'rebuild_season(this, 0x577f70, 0x9c4424, [0x26c, 0x273, 0x279, 0x280], 1) { // mwf6: the turnover of the English Third Division',
     'rebuild_season(this, 0x577f70, 0x9c4424, [0x26c, 0x273, 0x279, 0x280], 2) { // mutation: the year plus 2',
     'mwf_6 English Third Division rollover'),
]
