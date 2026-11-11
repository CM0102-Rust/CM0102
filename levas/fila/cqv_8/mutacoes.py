# Mutacoes (cqv_8): cada construtor roda uma vez, na criacao da carreira de 8 ligas (contadores "<item> <copa> new"); a
# copa montada um ano adiante leva as rodadas, os inscritos e os jogos para a temporada seguinte.
MUTACOES = [
    ('cqv8_copa_da_italia_um_ano_adiante',
     'ita_cup_constructor_body(this, year, owner) // cqv8: the Coppa Italia',
     'ita_cup_constructor_body(this, year + 1, owner) // mutacao: um ano adiante',
     'cqv_8 coppa italia new'),
    ('cqv8_copa_da_serie_c_um_ano_adiante',
     'ita_c_cup_constructor_body(this, year, owner) // cqv8: the Serie C cup',
     'ita_c_cup_constructor_body(this, year + 1, owner) // mutacao: um ano adiante',
     'cqv_8 serie c cup new'),
]
