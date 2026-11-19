# Mutacoes (mwg_14) (as oitavas da Copa de 2002 sao em junho, dentro da janela; os comparadores so rodam o original):
# - o 1o de D e o 1o de E trocam de lugar na lista das oitavas (outros confrontos, outra Copa: os resultados das
#   selecoes e dos jogadores mudam na carreira).
# advance_stage e record_season_honours ficam sem mutacao propria: pular a fase ou as honras derruba a Copa (sem
# oitavas, a fase 7 nula e lida sem teste) ou so muda o historico depois de junho.
MUTACOES = [
    ('mwg14_oitavas_trocadas',
     "(0, 0), (1, 1), (3, 0), (2, 1), (4, 0), (5, 1), (7, 0), (6, 1), // mwg14: the group winners and runners-up paired",
     "(0, 0), (1, 1), (4, 0), (2, 1), (3, 0), (5, 1), (7, 0), (6, 1), // mutation: D1 and E1 swapped",
     'mwg_14 world cup round of 16'),
]
