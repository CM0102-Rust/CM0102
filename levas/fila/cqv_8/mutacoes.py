# Mutacoes (cqv_8): cada construtor roda uma vez, na criacao da carreira de 8 ligas (contadores "<item> <copa> nova"); a
# copa montada um ano adiante leva as rodadas, os inscritos e os jogos para a temporada seguinte.
MUTACOES = [
    ('cqv8_copa_da_italia_um_ano_adiante',
     'cqv8_copa_da_italia_corpo(this, ano, dono) // cqv8: copa_da_italia',
     'cqv8_copa_da_italia_corpo(this, ano + 1, dono) // mutacao: um ano adiante',
     'cqv_8 copa_da_italia nova'),
    ('cqv8_copa_da_serie_c_um_ano_adiante',
     'cqv8_copa_da_serie_c_corpo(this, ano, dono) // cqv8: copa_da_serie_c',
     'cqv8_copa_da_serie_c_corpo(this, ano + 1, dono) // mutacao: um ano adiante',
     'cqv_8 copa_da_serie_c nova'),
]
