# Mutacoes (cqu_8): cada construtor roda uma vez, na criacao da carreira de 8 ligas (contadores "<item> <copa> nova"); a
# copa montada um ano adiante leva as rodadas, os inscritos e os jogos para a temporada seguinte.
MUTACOES = [
    ('cqu8_copa_do_imperador_um_ano_adiante',
     "construct_jap_emp_cup_body(Cup(this), year, record) // cqu8: the Emperor's Cup",
     'construct_jap_emp_cup_body(Cup(this), year + 1, record) // mutation: one year ahead',
     'cqu_8 emperors cup new'),
    ('cqu8_copa_da_liga_japonesa_um_ano_adiante',
     'construct_jap_j_cup_body(Cup(this), year, record) // cqu8: the J League Cup',
     'construct_jap_j_cup_body(Cup(this), year + 1, record) // mutation: one year ahead',
     'cqu_8 j league cup new'),
    ('cqu8_supercopa_japonesa_um_ano_adiante',
     'construct_jap_super_body(Cup(this), year, record) // cqu8: the Japanese Super Cup',
     'construct_jap_super_body(Cup(this), year + 1, record) // mutation: one year ahead',
     'cqu_8 japanese super cup new'),
]
