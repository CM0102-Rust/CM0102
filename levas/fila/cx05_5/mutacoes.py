# Mutacoes (cx05_5), carreira de 8 ligas ate 2004-11-01. As viradas sao so a original no comparador: a mutacao e
# acusada pela carreira (a edicao seguinte vem no ano errado: as datas do sorteio e dos jogos mudam).
# - a Copa Africana de tres em tres anos;
# - a Copa da Asia de dois em dois anos;
# - a Eurocopa de cinco em cinco anos.
MUTACOES = [
    ('cx05_5_africana_3_anos',
     '    let years = 2i16; // cx05_5: the African Nations Cup every two years',
     '    let years = 3i16; // mutation'),
    ('cx05_5_asia_2_anos',
     '    let years = 4i16; // cx05_5: the Asian Cup every four years',
     '    let years = 2i16; // mutation'),
    ('cx05_5_eurocopa_5_anos',
     '    league.set_year(league.year().wrapping_add(4)); // cx05_5: the European Championship every four years',
     '    league.set_year(league.year().wrapping_add(5)); // mutation'),
]
