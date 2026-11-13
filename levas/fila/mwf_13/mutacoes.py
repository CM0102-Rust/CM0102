# Mutacoes (mwf_13): o ano da virada do trofeu sobe 2 (as datas de 2002/03 mudam de ano; acusada pelo estado no fim
# da carreira, 2002-08-01); o premio de 200 mil em vez de 400 mil (o premio pago nas rodadas muda; o comparador lado a
# lado das rodadas a pega na virada).
MUTACOES = [
    ('mwf13_trofeu_ano_mais_2',
     'this.set_year(a.wrapping_add(1)); // mwf13: the year of the Auto Windscreens Shield',
     'this.set_year(a.wrapping_add(2)); // mutation: the year plus 2',
     'mwf_13 auto windscreens rollover'),
    ('mwf13_premio_200_mil',
     'this.set_prize_fund(0x61a80); // mwf13: the Auto Windscreens Shield prize fund',
     'this.set_prize_fund(0x30d40); // mutation: 200,000',
     'mwf_13 auto windscreens rounds'),
]
