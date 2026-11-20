# Mutacoes (mwa_23): grupos de 3 tambem nas fases 5 e 6 (dois inscritos ficam fora: outros jogos); o mata-mata pesado
# como uma rodada so (a reputacao dos tecnicos: pode escapar da carreira, o comparador e so da original); faixas de 4
# nas posicoes previstas (o comparador lado a lado pega).
MUTACOES = [
    ('mwa23_grupos_de_tres', 'let n = if (g as i32) >= 5 { 4 } else { 3 }; // mwa23: four clubs from stage 5', 'let n = 3; // mutation: three clubs everywhere', 'mwa_23 irish league cup group'),
    ('mwa23_mata_mata_de_uma_rodada', 'welsh_cup_staff_records(League(this), club, outcome, stage, game, a6, 7, 3) // mwa23: three knockout rounds', 'welsh_cup_staff_records(League(this), club, outcome, stage, game, a6, 7, 1) // mutation: one round', 'mwa_23 irish league cup staff records'),
    ('mwa23_faixas_de_4', 'italian_cup_predicted_positions(this, 0x9e9f4c, (0x3ee, 0x3f6), 7, 8, ', 'italian_cup_predicted_positions(this, 0x9e9f4c, (0x3ee, 0x3f6), 7, 4, ', 'mwa_23 irish league cup predicted'),
]
