# Mutacoes (cqu_9): cada construtor roda uma vez, na criacao da carreira de 8 ligas (contadores "<item> <copa> nova"); a
# copa montada um ano adiante leva as rodadas, os inscritos e os jogos para a temporada seguinte.
MUTACOES = [
    ('cqu9_copa_da_uefa_um_ano_adiante',
     'cqu9_copa_da_uefa_corpo(this, ano, dono) // cqu9: copa_da_uefa',
     'cqu9_copa_da_uefa_corpo(this, ano + 1, dono) // mutacao: um ano adiante',
     'cqu_9 copa_da_uefa nova'),
    ('cqu9_supercopa_europeia_um_ano_adiante',
     'cqu9_supercopa_europeia_corpo(this, ano, dono) // cqu9: supercopa_europeia',
     'cqu9_supercopa_europeia_corpo(this, ano + 1, dono) // mutacao: um ano adiante',
     'cqu_9 supercopa_europeia nova'),
    ('cqu9_intertoto_um_ano_adiante',
     'cqu9_intertoto_corpo(this, ano, dono) // cqu9: intertoto',
     'cqu9_intertoto_corpo(this, ano + 1, dono) // mutacao: um ano adiante',
     'cqu_9 intertoto nova'),
]
