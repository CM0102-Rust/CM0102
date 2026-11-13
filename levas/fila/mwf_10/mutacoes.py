# Mutacoes (mwf_10): o ano da virada da Serie B sobe 2 (as datas de 2002/03 mudam de ano; acusada pelo estado no fim
# da carreira, 2002-08-01); o vencedor do mata-mata da C1 B em 3o em vez de 2o (muda a ultima posicao gravada, que as
# posicoes previstas e as expectativas da temporada seguinte leem; o comparador lado a lado a pega).
MUTACOES = [
    ('mwf10_serie_b_ano_mais_2',
     'this.set_year(a.wrapping_add(1)); // mwf10: the year of Serie B',
     'this.set_year(a.wrapping_add(2)); // mutation: the year plus 2',
     'mwf_10 serie b rollover'),
    ('mwf10_vencedor_do_mata_mata_em_3o',
     '1 => c.set_last_position(2), // mwf10: the play-off winner 2nd',
     '1 => c.set_last_position(3), // mutation: the winner 3rd',
     'mwf_10 serie c1 b play-off places'),
]
