# The Setup screens (7): no mutation by the career (the screens batch runs none). For a session of Lucas's with
# JOGAR_COMPARANDO.bat, caught by the comparer of select_default_league_by_language (side by side):
# - English picks France as the default league: Setup Game -> Start New Game (English game) shows the divergence.
MUTACOES = [
    ('tel12_3_liga_padrao',
     '        0 | 1 => KEY_NATION_ENGLAND,',
     '        0 | 1 => KEY_NATION_FRANCE, // tel12_3 mutation',
     'tel12_3 default league'),
]
