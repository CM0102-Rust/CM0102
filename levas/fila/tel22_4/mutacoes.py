# Mutations (tel22_4). The automatic career opens no screen: the chain runs none; these prove the comparers in a
# session of Lucas's (JOGAR_COMPARANDO.bat):
# - the Average Points tab writes the stage plus one (Competition screen > Average Points > pick a stage);
# - the tab bar writes the next tab (Competition screen > click any other tab).
MUTACOES = [
    ('tel22_4_average_stage', 'let stage = (tel26_item_value(w) as i32 / 1000 - 1) as usize;',
     'let stage = (tel26_item_value(w) as i32 / 1000) as usize;'),
    ('tel22_4_tab_picked', 'set_screen_param(1, tel26_item_value(w) as usize);\n        return -11;',
     'set_screen_param(1, tel26_item_value(w) as usize + 1);\n        return -11;'),
]
