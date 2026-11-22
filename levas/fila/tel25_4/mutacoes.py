# Mutations (tel25_4). The automatic career opens no screen: the chain runs none; these prove the comparer in a
# session of Lucas's (JOGAR_COMPARANDO.bat; Training screen, edit a schedule): every build of the editor is compared.
# - the levels' links shifted by one (3..6 become 4..7);
# - Ok always enabled (the empty-name grey text never drawn; caught when the name box is emptied).
MUTACOES = [
    ('tel25_4_level_links', 'schedule_editor_choice(levels, col, row, level == value, col + 2, category);',
     'schedule_editor_choice(levels, col, row, level == value, col + 3, category);'),
    ('tel25_4_ok_always', '    let name = schedule + 9;\n    if read_u8(name) == 0 {', '    let name = schedule + 9;\n    if read_u8(name) == 0xff {'),
]
