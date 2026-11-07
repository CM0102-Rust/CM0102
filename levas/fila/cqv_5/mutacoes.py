# Mutacoes (cqv_5): cada construtor roda uma vez, na criacao da carreira de 8 ligas (contadores "<item> <copa> nova"); a
# copa montada um ano adiante leva as rodadas, os inscritos e os jogos para a temporada seguinte.
MUTACOES = [
    ('cqv5_supercopa_italiana_um_ano_adiante',
     'cqv5_supercopa_italiana_corpo(this, ano, dono) // cqv5: supercopa_italiana',
     'cqv5_supercopa_italiana_corpo(this, ano + 1, dono) // mutacao: um ano adiante',
     'cqv_5 supercopa_italiana nova'),
    ('cqv5_supercopa_da_serie_c1_um_ano_adiante',
     'cqv5_supercopa_da_serie_c1_corpo(this, ano, dono) // cqv5: supercopa_da_serie_c1',
     'cqv5_supercopa_da_serie_c1_corpo(this, ano + 1, dono) // mutacao: um ano adiante',
     'cqv_5 supercopa_da_serie_c1 nova'),
    ('cqv5_copa_dos_eua_um_ano_adiante',
     'cqv5_copa_dos_eua_corpo(this, ano, dono) // cqv5: copa_dos_eua',
     'cqv5_copa_dos_eua_corpo(this, ano + 1, dono) // mutacao: um ano adiante',
     'cqv_5 copa_dos_eua nova'),
]
