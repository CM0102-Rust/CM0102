# Mutations (tel09_2): the screens batch runs no mutations in the chain; these are for a session of Lucas with
# JOGAR_COMPARANDO.bat: open a player's profile > History > View.
# - the Bans line of the View popup with value 0x10 instead of 8: another item value in the display list;
# - the "<<" button of the Playing Career view with command 0x3d instead of 0x3e: another item id (Playing Career).
MUTACOES = [
    ('tel09_2_menu_bans',
     '        history_tab_view_line(view, 8, menu, &mut title);\n',
     '        history_tab_view_line(view, 0x10, menu, &mut title); // tel09_2 mutation\n',
     'tel09_2 history tab'),
    ('tel09_2_botao_anterior',
     '(1u32, 0x2au32, 0x98e7c4usize, 0x3eu32)',
     '(1u32, 0x2au32, 0x98e7c4usize, 0x3du32)',
     'tel09_2 history tab playing career'),
]
