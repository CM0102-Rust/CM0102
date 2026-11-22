# The Setup screens (8): no mutation by the career (the screens batch runs none). For a session of Lucas's with
# JOGAR_COMPARANDO.bat, caught by the side by side comparers:
# - the main nation's entry gives its END_DATE as the start date: Start New Game -> any league -> Next (create_game)
#   shows the divergence of setup_set_game_start_date;
# - ASIA drops Australia: caught by nation_matches_region_filter when a career is created from a setup file naming
#   ASIA (create_game_from_setup_file).
MUTACOES = [
    ('tel13_1_data_inicio',
     '            exe::date_operator_3d(G_GAME_START_DATE, tmp.as_mut_ptr() as usize, entry + key_nation_record::START_DATE);\n            if read_u32',
     '            exe::date_operator_3d(G_GAME_START_DATE, tmp.as_mut_ptr() as usize, entry + key_nation_record::END_DATE); // tel13_1 mutation\n            if read_u32',
     'tel13_1 game start date'),
    ('tel13_1_filtro_asia',
     '        return one_of(&[KEY_NATION_AUSTRALIA, KEY_NATION_JAPAN]); // "ASIA"',
     '        return one_of(&[KEY_NATION_JAPAN]); // "ASIA" tel13_1 mutation',
     'tel13_1 region filter'),
]
