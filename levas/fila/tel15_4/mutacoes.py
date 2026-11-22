# Mutations (tel15_4). The automatic career opens no screen: the chain runs none; these prove the comparer in a
# session of Lucas's (JOGAR_COMPARANDO.bat): in a live match after minute 45, the club's screen > Tactics: swap a substitute onto a player (the substitution's minute) and the Goals column.
MUTACOES = [
    ('tel15_4_second_half_cap', '        0x3de => 0x5a,',
     '        0x3de => 0x5b,'),
    ('tel15_4_goals', '=> read_u8(mp + match_player::GOALS),',
     '=> read_u8(mp + match_player::ASSISTS),'),
]
