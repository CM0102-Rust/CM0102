# Mutacoes (mwb_11): a Liga de Gales com a disciplina das copas escocesas (outras suspensoes); 2 pontos por vitoria
# (outra classificacao); a virada pula um ano (so o comparador e a temporada seguinte).
MUTACOES = [
    ('mwb11_gales_disciplina_das_copas_escocesas', 'discipline: 0x28, // mwb11: the League of Wales discipline rule', 'discipline: 0x19, // mutation: the Scottish cups rule', 'mwb_11 new league of wales'),
    ('mwb11_gales_dois_pontos', '(league::WIN_POINTS, 1, 3), // mwb11: the League of Wales points for a win', '(league::WIN_POINTS, 1, 2), // mutation: 2 points a win', 'mwb_11 league of wales rules'),
    ('mwb11_gales_virada_pula_um_ano', '[0x1f9, 0x200, 0x206, 0x20d], true, 1, "mwb_11 league of wales turnover") // mwb11: the next League of Wales season', '[0x1f9, 0x200, 0x206, 0x20d], true, 2, "mwb_11 league of wales turnover") // mutation: two years on', 'mwb_11 league of wales turnover'),
]
