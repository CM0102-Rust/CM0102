# Mutations (tel22_2). The automatic career opens no screen: the chain runs none; these prove the comparer in a
# session of Lucas's (JOGAR_COMPARANDO.bat):
# - the league table's band divider with the other width (competition screen > Table of a league with promotion or
#   relegation bands);
# - one team fewer in " N teams to draw" (competition screen of a cup > Draw, a round still being drawn).
MUTACOES = [
    ('tel22_2_divider_width', '{ 0x30a } else { 0x30a - 0x16 }', '{ 0x30a - 0x16 } else { 0x30a }'),
    ('tel22_2_teams_to_draw', '* 2 - 2 + shown as i32;', '* 2 - 3 + shown as i32;'),
]
