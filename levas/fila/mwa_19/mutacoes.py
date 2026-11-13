# Mutacoes (mwa_19), na Copa da Liga da Irlanda do Norte, carreira ENG IRL NIR ate 2002-08-01:
# - o mata-mata montado um grupo antes (na fase 2: o ultimo grupo fica sem classificados);
# - a virada sobe o ano em 2 (a edicao nova e depois da data final: se a carreira nao acusar, o comparador da virada
#   acusa).
MUTACOES = [
    ('mwa19_mata_mata_um_grupo_antes',
     'advance_cup_stage(League(this), 0xa69698, 0x21a, 3, 0x78d600) // mwa19: the Northern Irish League Cup knockout after the three groups',
     'advance_cup_stage(League(this), 0xa69698, 0x21a, 2, 0x78d600) // mutation: one group early',
     'cqr_7 italian cups knockout'),
    ('mwa19_virada_ano_mais_dois',
     'start_irish_league_cup_season(League(this), 0x78d800, 0x78cfd0, 0x78d3c0, 3, 0xa69698, [0x1ec, 0x1f3, 0x1f9, 0x200], 1)',
     'start_irish_league_cup_season(League(this), 0x78d800, 0x78cfd0, 0x78d3c0, 3, 0xa69698, [0x1ec, 0x1f3, 0x1f9, 0x200], 2)',
     'mwa_18 irish league cup turnover'),
]
