# The Setup screens (3): no mutation by the career (the screens batch runs none). For a session of Lucas's with
# JOGAR_COMPARANDO.bat, caught by the comparer on the screen named:
# - Select Team with the Next button enabled (Add Manager -> names -> nationality -> Next);
# - Select Nationality centring the chosen row one row lower (the first time the screen opens).
MUTACOES = [
    ('tel11_3_next_no_time',
     '    exe::gui_add_back_next_buttons(back, 0); // tel11_3 no next on Select Team',
     '    exe::gui_add_back_next_buttons(back, 1); // tel11_3 mutation',
     'tel11_3 build select team'),
    ('tel11_3_centro_da_lista',
     '        exe::screenmanager_set_screen_param(sm, 0, chosen_row.wrapping_sub((rows >> 1) as usize), 0); // tel11_3 the chosen row in the middle',
     '        exe::screenmanager_set_screen_param(sm, 0, chosen_row.wrapping_sub((rows >> 1) as usize) + 1, 0); // tel11_3 mutation',
     'tel11_3 build select nationality'),
]
