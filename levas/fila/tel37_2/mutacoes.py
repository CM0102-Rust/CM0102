# Mutations (tel37_2). The automatic career opens no screen: the chain runs none; these prove the comparers in a
# session of Lucas's (JOGAR_COMPARANDO.bat):
# - scrolling down stops one widget early (any long list scrolled with the wheel: the plan check of scroll_rows);
# - the fixed rectangle swaps x2 and y1 (any screen: process_command_packet places the fixed widgets).
MUTACOES = [
    ('tel37_2_scroll_one_short', 'if distance(k) >= delta {\n                    break;',
     'if distance(k) >= delta - 1 {\n                    break;'),
    ('tel37_2_fixed_rect_swap', 'write_i32(item + guio_rect::FIXED_X2, x2);',
     'write_i32(item + guio_rect::FIXED_X2, y1);'),
]
