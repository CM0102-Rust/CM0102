# Mutations (tel30_2). The automatic career opens no screen: the chain runs none; these prove the comparer in a
# session of Lucas's (JOGAR_COMPARANDO.bat) with the background pictures on: a club screen (a club without its own
# picture falls back to its stadium) and the board request screen (the boardroom section).
MUTACOES = [
    ('tel30_2_club_second_stadium', 'attempt(background_find_stadium_picture, read_usize(record + 0x6e))',
     'attempt(background_find_stadium_picture, read_usize(record + club::STADIUM))'),
    ('tel30_2_section_exact', '    if exact != 0 {\n        return 0;\n    }\n    background_set_random_picture();\n    1\n}',
     '    if exact == 0 {\n        return 0;\n    }\n    background_set_random_picture();\n    1\n}'),
]
