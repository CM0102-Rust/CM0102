# Mutacoes (mwb_17): a Copa da Liga escocesa pula um ano na virada (as datas da edicao nova um ano depois: a 1a rodada
# de 31/7/2002 nao acontece; e o comparador).
MUTACOES = [
    ('mwb17_copa_da_liga_pula_um_ano',
     'cup.set_year(cup.year().wrapping_add(1)); // mwb17: the next League Cup',
     'cup.set_year(cup.year().wrapping_add(2)); // mutation: two years on',
     'mwb_17 scottish league cup turnover'),
]
