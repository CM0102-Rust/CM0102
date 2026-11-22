# Mutations (tel22_5). The automatic career opens no screen: the chain runs none; these prove the comparer in a
# session of Lucas's (JOGAR_COMPARANDO.bat), on the competition history screen:
# - the record text drawn 8 pixels further right (Records tab: the build comparer catches the display list);
# - one row fewer in the Winners table area (Winners tab, after its first build: the tab comparer catches it).
MUTACOES = [
    ('tel22_5_records_text_x', 'draw_wrapped(t, area, x + 8, 0x2ee', 'draw_wrapped(t, area, x + 16, 0x2ee'),
    ('tel22_5_winner_rows', 'let winner_rows = ((bottom - top - 0x10)', 'let winner_rows = ((bottom - top - 0x20)'),
]
