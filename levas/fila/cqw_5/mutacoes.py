# Mutacoes (cqw_5): cada construtor roda uma vez, na criacao da carreira de 8 ligas (contadores "<item> <copa> nova"); a
# copa montada um ano adiante leva as rodadas, os inscritos e os jogos para a temporada seguinte.
MUTACOES = [
    ('cqw5_supercopa_asiatica_um_ano_adiante',
     'cqw5_supercopa_asiatica_corpo(this, ano, dono) // cqw5: supercopa_asiatica',
     'cqw5_supercopa_asiatica_corpo(this, ano + 1, dono) // mutacao: um ano adiante',
     'cqw_5 supercopa_asiatica nova'),
    ('cqw5_copa_interamericana_um_ano_adiante',
     'cqw5_copa_interamericana_corpo(this, ano, dono) // cqw5: copa_interamericana',
     'cqw5_copa_interamericana_corpo(this, ano + 1, dono) // mutacao: um ano adiante',
     'cqw_5 copa_interamericana nova'),
    ('cqw5_copa_mercosul_um_ano_adiante',
     'cqw5_copa_mercosul_corpo(this, ano, dono) // cqw5: copa_mercosul',
     'cqw5_copa_mercosul_corpo(this, ano + 1, dono) // mutacao: um ano adiante',
     'cqw_5 copa_mercosul nova'),
    ('cqw5_jogo_das_estrelas_um_ano_adiante',
     'cqw5_jogo_das_estrelas_corpo(this, ano, dono) // cqw5: jogo_das_estrelas',
     'cqw5_jogo_das_estrelas_corpo(this, ano + 1, dono) // mutacao: um ano adiante',
     'cqw_5 jogo_das_estrelas nova'),
]
