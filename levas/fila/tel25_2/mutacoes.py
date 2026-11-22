# Mutations (tel25_2). The automatic career opens no screen: the chain runs none; these prove the comparers in a
# session of Lucas's (JOGAR_COMPARANDO.bat; Training screen, the Coaches button, a category click):
# - the coaches panel's category toggle sets the flag instead of flipping it (caught by the input comparer when a lit
#   category is clicked);
# - the coaches panel's grey category text moved one column (caught by the builder comparer on an empty row).
MUTACOES = [
    ('tel25_2_toggle_sets', 'write_u8(flag, (read_u8(flag) == 0) as u8);', 'write_u8(flag, 1);'),
    ('tel25_2_grey_column', 'tel26_area_text([grid, category + 1, k, 1, 0, 0xc, 2,', 'tel26_area_text([grid, category, k, 1, 0, 0xc, 2,'),
]
