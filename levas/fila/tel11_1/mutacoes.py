# The Setup screens (1). The automatic career goes through setup_run_front_end, setup_build_setup_game_screen and the
# "auto_setup.cfg" branch of setup_handle_setup_game_input, which these mutations do not touch: the chain does not run
# mutations in the screens batch. They are for a session of Lucas's with JOGAR_COMPARANDO.bat, where the comparer
# catches each one on the screen named:
# - the Setup Game menu in 3 columns instead of 2 (the opening screen, right away);
# - the Select League(s) list's first column 0x20 wide instead of 0x19 (Start New Game);
# - the extra division button (e.g. England's Conference on Select League(s)) leaving the load count unchanged when
#   switched off (click it twice).
MUTACOES = [
    ('tel11_1_menu_3_colunas',
     '    menu(0x6e, 0x91, 0x30c, 0x217, 2, 6, 3, 9, items.as_mut_ptr() as usize, 0, 0);',
     '    menu(0x6e, 0x91, 0x30c, 0x217, 3, 6, 3, 9, items.as_mut_ptr() as usize, 0, 0); // tel11_1 mutation',
     'tel11_1 build setup game'),
    ('tel11_1_coluna_da_lista',
     '    let mut list_columns: [u8; 7] = [0x19, 1, 0xd, 1, 0xf, 1, 0x13];',
     '    let mut list_columns: [u8; 7] = [0x20, 1, 0xd, 1, 0xf, 1, 0x13]; // tel11_1 mutation',
     'tel11_1 build select leagues'),
    ('tel11_1_divisao_extra_fica',
     '                setup_count(G_NATIONS_TO_LOAD_COUNT, -1); // tel11_1 the extra division leaves the load',
     '                setup_count(G_NATIONS_TO_LOAD_COUNT, 0); // tel11_1 mutation',
     'tel11_1 select leagues input'),
]
