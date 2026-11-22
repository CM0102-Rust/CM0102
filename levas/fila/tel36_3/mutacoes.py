# Mutations (tel36_3). The automatic career opens no screen; these prove the comparer in a session of Lucas's (the
# screen: a player's profile > Action > "Request Physio Report", at a club with two or three physios).
# - the physio list starts one row further down;
# - choosing a physio keeps the old one.
MUTACOES = [
    ('tel36_3_list_row', 'let mut row = line + 2;\n        let d = current_human_display();', 'let mut row = line + 3;\n        let d = current_human_display();'),
    ('tel36_3_choice_kept', 'set_screen_param(1, tel26_item_value(item) as usize);\n            -11', 'set_screen_param(1, get_screen_param(1));\n            -11'),
]
