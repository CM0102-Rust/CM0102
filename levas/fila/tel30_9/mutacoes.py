# Mutations (tel30_9). The automatic career opens no screen: the chain runs none; these prove the comparer in a
# session of Lucas's (JOGAR_COMPARANDO.bat): a player's profile > Action > "Request coach report" (command 0x2a of the
# profile), then a click on a coach and on "Select All".
MUTACOES = [
    ('tel30_9_coach_toggle', 'set_screen_param(1, (tel26_item_value(item as i16) ^ chosen) as usize);',
     'set_screen_param(1, (tel26_item_value(item as i16) | chosen) as usize);'),
    ('tel30_9_chosen_colour', 'let colour = if bit & chosen != 0 { yellow } else { grey };',
     'let colour = if bit & chosen != 0 { grey } else { grey };'),
]
