# Mutacoes (cqu_9): cada construtor roda uma vez, na criacao da carreira de 8 ligas (contadores "<item> <copa> new"); a
# copa montada um ano adiante leva as rodadas, os inscritos e os jogos para a temporada seguinte.
MUTACOES = [
    ('cqu9_copa_da_uefa_um_ano_adiante',
     'construct_uefa_cup_body(this, year, record) // cqu9: the UEFA Cup',
     'construct_uefa_cup_body(this, year + 1, record) // mutation: one year ahead',
     'cqu_9 uefa cup new'),
    ('cqu9_supercopa_europeia_um_ano_adiante',
     'construct_eur_super_cup_body(this, year, record) // cqu9: the European Super Cup',
     'construct_eur_super_cup_body(this, year + 1, record) // mutation: one year ahead',
     'cqu_9 european super cup new'),
    ('cqu9_intertoto_um_ano_adiante',
     'construct_intertoto_cup_body(this, year, record) // cqu9: the Intertoto Cup',
     'construct_intertoto_cup_body(this, year + 1, record) // mutation: one year ahead',
     'cqu_9 intertoto cup new'),
]
