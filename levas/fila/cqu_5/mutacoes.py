# Mutacoes (cqu_5): cada construtor roda uma vez, na criacao da carreira de 8 ligas (contadores "<item> <copa> nova"); a
# copa montada um ano adiante leva as rodadas, os inscritos e os jogos para a temporada seguinte.
MUTACOES = [
    ('cqu5_copa_da_inglaterra_um_ano_adiante',
     'construct_eng_fa_cup_body(Cup(this), year, record) // cqu5: the FA Cup',
     'construct_eng_fa_cup_body(Cup(this), year + 1, record) // mutation: one year ahead',
     'cqu_5 fa cup new'),
    ('cqu5_copa_da_liga_inglesa_um_ano_adiante',
     'construct_eng_cc_cup_body(Cup(this), year, record) // cqu5: the League Cup',
     'construct_eng_cc_cup_body(Cup(this), year + 1, record) // mutation: one year ahead',
     'cqu_5 league cup new'),
    ('cqu5_supercopa_inglesa_um_ano_adiante',
     'construct_eng_charity_body(Cup(this), year, record) // cqu5: the Charity Shield',
     'construct_eng_charity_body(Cup(this), year + 1, record) // mutation: one year ahead',
     'cqu_5 charity shield new'),
]
