# Mutations (tel37_4). The automatic career opens no screen: the chain runs none; these prove the comparers in a
# session of Lucas's (JOGAR_COMPARANDO.bat):
# - a one-line text area puts the text one row lower (any screen with a title box);
# - the tab bar's second row takes one tab more (a tabbed screen with more than 5 tabs, e.g. a club's screens).
MUTACOES = [
    ('tel37_4_text_row', 'let mut row = rows.wrapping_sub(1) / 2;', 'let mut row = rows.wrapping_sub(1) / 2 + 1;'),
    ('tel37_4_tab_rows', 'let per_row = if n > 5 { if odd_first as u8 != 0 { (n + 1) / 2 } else { n / 2 } } else { n };',
     'let per_row = if n > 5 { if odd_first as u8 != 0 { (n + 1) / 2 } else { n / 2 - 1 } } else { n };'),
]
