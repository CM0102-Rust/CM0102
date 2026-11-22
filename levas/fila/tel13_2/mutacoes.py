# The key nation competitions (1): no mutation by the career (the screens batch runs none). For a session of Lucas's
# with JOGAR_COMPARANDO.bat, caught by the plan check of create_world_cup_qualifying_comps (key_nation_check_comps:
# the original runs and the entry is checked against the data): Start New Game -> any league -> Next (create_game).
MUTACOES = [
    ('tel13_2_ano_copa',
     '    season_year: EntryYear::Fixed(0x7cc),',
     '    season_year: EntryYear::Fixed(0x7cd), // tel13_2 mutation',
     'tel13_2 world cup qualifying'),
]
