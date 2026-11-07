# Mutacoes (cqu_8): cada construtor roda uma vez, na criacao da carreira de 8 ligas (contadores "<item> <copa> nova"); a
# copa montada um ano adiante leva as rodadas, os inscritos e os jogos para a temporada seguinte.
MUTACOES = [
    ('cqu8_copa_do_imperador_um_ano_adiante',
     'cqu8_copa_do_imperador_corpo(this, ano, dono) // cqu8: copa_do_imperador',
     'cqu8_copa_do_imperador_corpo(this, ano + 1, dono) // mutacao: um ano adiante',
     'cqu_8 copa_do_imperador nova'),
    ('cqu8_copa_da_liga_japonesa_um_ano_adiante',
     'cqu8_copa_da_liga_japonesa_corpo(this, ano, dono) // cqu8: copa_da_liga_japonesa',
     'cqu8_copa_da_liga_japonesa_corpo(this, ano + 1, dono) // mutacao: um ano adiante',
     'cqu_8 copa_da_liga_japonesa nova'),
    ('cqu8_supercopa_japonesa_um_ano_adiante',
     'cqu8_supercopa_japonesa_corpo(this, ano, dono) // cqu8: supercopa_japonesa',
     'cqu8_supercopa_japonesa_corpo(this, ano + 1, dono) // mutacao: um ano adiante',
     'cqu_8 supercopa_japonesa nova'),
]
