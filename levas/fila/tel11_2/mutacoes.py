# The Setup screens (2): no mutation by the career (the screens batch runs none). For a session of Lucas's with
# JOGAR_COMPARANDO.bat, caught by the comparer on the screen named:
# - Network Play's menu in 3 columns (Setup Game -> Network Play);
# - Manager Status's param 2 written as 1 more (the screen right after Start New Game -> Next): the display list
#   comparer covers the params block.
MUTACOES = [
    ('tel11_2_rede_3_colunas',
     '    menu(0x6e, 0x91, 0x30c, 0x217, 2, 6, 3, 4, base, 0, 0);',
     '    menu(0x6e, 0x91, 0x30c, 0x217, 3, 6, 3, 4, base, 0, 0); // tel11_2 mutation',
     'tel11_2 build network play'),
    ('tel11_2_parametro_2',
     '    exe::screenmanager_set_screen_param(sm, 2, read_usize(hm() + HUMAN_MANAGER_LOCKED_COUNT), 0); // tel11_2 param 2',
     '    exe::screenmanager_set_screen_param(sm, 2, read_usize(hm() + HUMAN_MANAGER_LOCKED_COUNT) + 1, 0); // tel11_2 mutation',
     'tel11_2 build manager status'),
]
