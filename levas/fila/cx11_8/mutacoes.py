# Mutacoes (cx11_8), com a Italia e a base NORMAL ate 2002-09-30:
# - as fases do fim da C2/B montadas pelo molde da C1 (create_end_of_league_stages), que antes poe o campeao na
#   Supercopa da Serie C1: o campeao da C2/B entra na vaga 1 da Supercopa (no lugar do campeao de um grupo da C1, ou o
#   da C1 no lugar dele, conforme quem acaba antes). Acusada se a Supercopa da C1 for jogada ate 2002-09-30; senao
#   escapa, e o avanco e as fases ficam provados pelo molde (cql_5 na C1) e pelas mutacoes do cx11_7 (o mesmo
#   auxiliar).
MUTACOES = [
    ('cx11_8_c2b_na_supercopa_da_c1',
     'create_serie_c2_playoff_stages(League(this), 0x9ea75c, [0x27b, 0x28f, 0x2b2]) // cx11_8: the C2/B end stages',
     'create_end_of_league_stages(League(this), 0x9ea75c, [0x27b, 0x28f, 0x2b2], true) // mutation: the C1 template with the Super Cup',
     'cx11_8 serie c2 b end stages'),
]
