# Mutacoes (cqv_5): cada construtor roda uma vez, na criacao da carreira de 8 ligas (contadores "<item> <copa> new"); a
# copa montada um ano adiante leva as rodadas, os inscritos e os jogos para a temporada seguinte.
MUTACOES = [
    ('cqv5_supercopa_italiana_um_ano_adiante',
     'construct_ita_super_body(this, year, record) // cqv5: the Italian Super Cup',
     'construct_ita_super_body(this, year + 1, record) // mutation: one year ahead',
     'cqv_5 italian super cup new'),
    ('cqv5_supercopa_da_serie_c1_um_ano_adiante',
     'construct_ita_c1_super_body(this, year, record) // cqv5: the Serie C1 Super Cup',
     'construct_ita_c1_super_body(this, year + 1, record) // mutation: one year ahead',
     'cqv_5 serie c1 super cup new'),
    ('cqv5_copa_dos_eua_um_ano_adiante',
     'construct_usa_open_cup_body(this, year, record) // cqv5: the US Open Cup',
     'construct_usa_open_cup_body(this, year + 1, record) // mutation: one year ahead',
     'cqv_5 us open cup new'),
]
