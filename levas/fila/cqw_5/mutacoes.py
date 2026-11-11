# Mutacoes (cqw_5): cada construtor roda uma vez, na criacao da carreira de 8 ligas (contadores "<item> <copa> nova"); a
# copa montada um ano adiante leva as rodadas, os inscritos e os jogos para a temporada seguinte.
MUTACOES = [
    ('cqw5_supercopa_asiatica_um_ano_adiante',
     'build_asian_super_cup(Cup(this), year, record) // cqw5: the Asian Super Cup',
     'build_asian_super_cup(Cup(this), year + 1, record) // mutation: one year ahead',
     'cqw_5 asian super cup new'),
    ('cqw5_copa_interamericana_um_ano_adiante',
     'build_interamerican_cup(Cup(this), year, record) // cqw5: the Interamerican Cup',
     'build_interamerican_cup(Cup(this), year + 1, record) // mutation: one year ahead',
     'cqw_5 interamerican cup new'),
    ('cqw5_copa_mercosul_um_ano_adiante',
     'build_mercosur_cup(League(this), year, record) // cqw5: the Mercosur Cup',
     'build_mercosur_cup(League(this), year + 1, record) // mutation: one year ahead',
     'cqw_5 mercosur cup new'),
    ('cqw5_jogo_das_estrelas_um_ano_adiante',
     'build_mls_all_star_game(Cup(this), year, record) // cqw5: the MLS All-Star Game',
     'build_mls_all_star_game(Cup(this), year + 1, record) // mutation: one year ahead',
     'cqw_5 mls all-star game new'),
]
