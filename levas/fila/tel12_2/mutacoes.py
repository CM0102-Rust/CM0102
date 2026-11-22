# The Setup screens (6): no mutation by the career (the screens batch runs none). For a session of Lucas's with
# JOGAR_COMPARANDO.bat, caught by the comparer of the screen named:
# - Hall of Fame's ranks one column to the right: its display list differs at once (Setup Game -> Hall Of Fame).
MUTACOES = [
    ('tel12_2_posicao_coluna',
     '        cell(SCREEN_MANAGER, list, 0, row, 0x30, dark_blue, 0xc, 1, pale, ordinal);',
     '        cell(SCREEN_MANAGER, list, 1, row, 0x30, dark_blue, 0xc, 1, pale, ordinal); // tel12_2 mutation',
     'tel12_2 build hall of fame'),
]
