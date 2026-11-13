# Mutacoes (mwf_1): o ano de cada virada sobe 2 em vez de 1 (as rodadas, os sorteios e as datas da temporada
# 2002/03 mudam de ano; acusadas pelo estado no fim da carreira, 2002-08-01).
MUTACOES = [
    ('mwf1_engcharity_ano_mais_2',
     'start_new_cup_season(Cup(this), 0x56d780, 0x9c425c, [0x128, 0x12f, 0x136], 1, "mwf_1 Charity Shield rollover") // mwf1: the turnover of the Charity Shield',
     'start_new_cup_season(Cup(this), 0x56d780, 0x9c425c, [0x128, 0x12f, 0x136], 2, "mwf_1 Charity Shield rollover") // mutation: the year plus 2',
     'mwf_1 Charity Shield rollover'),
    ('mwf1_fracup_ano_mais_2',
     'start_new_cup_season(Cup(this), 0x5baa50, 0x9ccb28, [0x23a, 0x241, 0x248], 1, "mwf_1 Coupe de France rollover") // mwf1: the turnover of the Coupe de France',
     'start_new_cup_season(Cup(this), 0x5baa50, 0x9ccb28, [0x23a, 0x241, 0x248], 2, "mwf_1 Coupe de France rollover") // mutation: the year plus 2',
     'mwf_1 Coupe de France rollover'),
    ('mwf1_fralgecup_ano_mais_2',
     'start_new_cup_season(Cup(this), 0x5bd160, 0x9ccb98, [0x1f9, 0x200, 0x207], 1, "mwf_1 French League Cup rollover") // mwf1: the turnover of the French League Cup',
     'start_new_cup_season(Cup(this), 0x5bd160, 0x9ccb98, [0x1f9, 0x200, 0x207], 2, "mwf_1 French League Cup rollover") // mutation: the year plus 2',
     'mwf_1 French League Cup rollover'),
]
