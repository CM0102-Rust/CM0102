# parte 2 de 3 das escapadas da 320a (bancada 4)
MUTACOES = [
    ('tel39_7_sem_texto', '    if source != 0 {\n        text_set_string(this, source);', '    if source != 0 && false {\n        text_set_string(this, source);', 'tel39_7 construct a text'),
    ('tel41_2_group_filter', 'comp_stats_get_player_position_group(sheet) & groups as u16 != 0', 'comp_stats_get_player_position_group(sheet) & groups as u16 == 0'),
    ('tel41_4_fallback_best', '*best = read_u8(entry + comp_stats_club::BEST_POSITION); // tel41_4: fallback best', '*best = read_u8(entry + comp_stats_club::WORST_POSITION); // mutation: the worst as best'),
]
