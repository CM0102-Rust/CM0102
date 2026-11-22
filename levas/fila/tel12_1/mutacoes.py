# The Setup screens (5): no mutation by the career (the screens batch runs none). For a session of Lucas's with
# JOGAR_COMPARANDO.bat, caught by the comparer of the box named:
# - Reset with the wrong id: the Game Settings box's display list differs at once (Setup Game -> Game Settings).
MUTACOES = [
    ('tel12_1_reset_id',
     '    button(SCREEN_MANAGER, 0x261, y, 0x2de, y + 0x14, 0x30, grey, 0xc, 1, pale, SETUP_TEXT, 0x27, 0);',
     '    button(SCREEN_MANAGER, 0x261, y, 0x2de, y + 0x14, 0x30, grey, 0xc, 1, pale, SETUP_TEXT, 0x26, 0); // tel12_1 mutation',
     'tel12_1 build game settings'),
]
