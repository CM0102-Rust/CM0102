# Mutações da 319a leva (itens tel13_2, tel13_3, tel13_4). Uso: python mutacoes.py mutacoes\leva319.py --carreira 2002-06-01
MUTACOES = [
    ('tel13_2_ano_copa', '    season_year: EntryYear::Fixed(0x7cc),', '    season_year: EntryYear::Fixed(0x7cd), // tel13_2 mutation', 'tel13_2 world cup qualifying'),
    ('tel13_3_segunda_inglesa', '        (0xee, 0x5754a0, KEY_CLUB_COMP_ENGLISH_SECOND_DIVISION, EntryYear::Current(0), false),', '        (0xee, 0x5754a0, KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION, EntryYear::Current(0), false), // tel13_3 mutation', 'tel13_3 english'),
    ('tel13_4_liga_espanhola', '    league: EntryLink::Comp(KEY_CLUB_COMP_SPANISH_SUPER_CUP),', '    league: EntryLink::Comp(KEY_CLUB_COMP_SPANISH_CUP), // tel13_4 mutation', 'tel13_4 spanish'),
]
