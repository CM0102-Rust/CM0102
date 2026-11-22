# tel26_5: the comparer compares the display list when the screen's list already exists (a page change on the
# screen, or the screen drawn again); the first build is original only. The career opens no screen: the proof is
# Lucas's game with JOGAR_COMPARANDO.bat.
MUTACOES = [
    ('tel26_5_menu_id', '0, 2, k as usize, area);', '0, 2, k as usize + 1, area); // mutation: the menu ids shifted', 'tel26_5 manager stats selected statistic'),
    ('tel26_5_name_link', 'BUF, 0x7d7, s);', 'BUF, 0x7d5, s); // mutation: the name links as a club', 'tel26_5 manager stats selected statistic'),
]
