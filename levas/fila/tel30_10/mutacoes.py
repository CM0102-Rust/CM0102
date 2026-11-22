# Mutations (tel30_10). The records run in the simulation too (Comp_Stats), so the career's comparison catches these;
# in a session of Lucas's (JOGAR_COMPARANDO.bat) any competition screen opens through the opener.
MUTACOES = [
    ('tel30_10_rating_word', 'write_u16(this + 0x21, read_u16(src + 0x21));', 'write_u16(this + 0x21, read_u16(src + 0x1f));'),
    ('tel30_10_param_2d', 'set_screen_param(0x2d, 1);', 'set_screen_param(0x2d, 0);'),
]
