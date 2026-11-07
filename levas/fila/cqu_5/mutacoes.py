# Mutacoes (cqu_5): cada construtor roda uma vez, na criacao da carreira de 8 ligas (contadores "<item> <copa> nova"); a
# copa montada um ano adiante leva as rodadas, os inscritos e os jogos para a temporada seguinte.
MUTACOES = [
    ('cqu5_copa_da_inglaterra_um_ano_adiante',
     'cqu5_copa_da_inglaterra_corpo(this, ano, dono) // cqu5: copa_da_inglaterra',
     'cqu5_copa_da_inglaterra_corpo(this, ano + 1, dono) // mutacao: um ano adiante',
     'cqu_5 copa_da_inglaterra nova'),
    ('cqu5_copa_da_liga_inglesa_um_ano_adiante',
     'cqu5_copa_da_liga_inglesa_corpo(this, ano, dono) // cqu5: copa_da_liga_inglesa',
     'cqu5_copa_da_liga_inglesa_corpo(this, ano + 1, dono) // mutacao: um ano adiante',
     'cqu_5 copa_da_liga_inglesa nova'),
    ('cqu5_supercopa_inglesa_um_ano_adiante',
     'cqu5_supercopa_inglesa_corpo(this, ano, dono) // cqu5: supercopa_inglesa',
     'cqu5_supercopa_inglesa_corpo(this, ano + 1, dono) // mutacao: um ano adiante',
     'cqu_5 supercopa_inglesa nova'),
]
