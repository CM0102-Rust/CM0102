# Mutacoes (mwf_11): o ano da virada da K League sobe 2 (as datas da nova temporada mudam de ano; acusada pelo estado
# no fim da carreira, 2002-08-01); a vitoria valendo 2 pontos na K League e na Copa da Liga coreana (a classificacao
# e quem passa mudam nos jogos de 2001/02; o comparador lado a lado das regras tambem as pega).
MUTACOES = [
    ('mwf11_k_league_ano_mais_2',
     'this.set_year(a.wrapping_add(1)); // mwf11: the year of the K League',
     'this.set_year(a.wrapping_add(2)); // mutation: the year plus 2',
     'mwf_11 k league rollover'),
    ('mwf11_k_league_vitoria_2',
     'write_u8_at(o, 0xc2, 3); // mwf11: the K League points per win',
     'write_u8_at(o, 0xc2, 2); // mutation: 2 points a win',
     'mwf_11 k league rules'),
    ('mwf11_copa_da_liga_vitoria_2',
     'write_u8_at(o, 0xc2, 3); // mwf11: the League Cup points per win',
     'write_u8_at(o, 0xc2, 2); // mutation: 2 points a win',
     'mwf_11 korean league cup rules'),
]
