# Mutacoes (mwa_18), na Copa da Liga da Irlanda, carreira ENG IRL NIR ate 2002-08-01:
# - o mata-mata montado um grupo antes (na fase 6: o ultimo grupo fica sem classificado);
# - a virada sobe o ano em 2 (a edicao nova e depois da data final: se a carreira nao acusar, o comparador da virada
#   acusa).
MUTACOES = [
    ('mwa18_mata_mata_um_grupo_antes',
     'advance_cup_stage(League(this), 0x9e9f4c, 0x212, 7, 0x637c00) // mwa18: the Irish League Cup knockout after the seven groups',
     'advance_cup_stage(League(this), 0x9e9f4c, 0x212, 6, 0x637c00) // mutation: one group early',
     'cqr_7 italian cups knockout'),
    ('mwa18_virada_ano_mais_dois',
     'start_irish_league_cup_season(League(this), 0x637dd0, 0x6375c0, 0x6379c0, 7, 0x9e9f4c, [0x1e4, 0x1eb, 0x1f1, 0x1f8], 1)',
     'start_irish_league_cup_season(League(this), 0x637dd0, 0x6375c0, 0x6379c0, 7, 0x9e9f4c, [0x1e4, 0x1eb, 0x1f1, 0x1f8], 2)',
     'mwa_18 irish league cup turnover'),
]
