# Mutations (tel22_1). The automatic career opens no screen: the chain runs none; these prove the comparer in a
# session of Lucas's (JOGAR_COMPARANDO.bat):
# - the Results tab's round heading one column narrower (open a competition > Results, on a day with matches, the
#   second time the tab is built: caught by the build's display-list comparer);
# - the Print popup items enabled without the id (a competition with statistics: the build's comparer).
MUTACOES = [
    ('tel22_1_round_heading_width',
     'let x2 = (exe::font_text_width(3usize, JOB_TEXT as *const u8) + 0x73).min(0x2f1); // tel22_1 the round heading\'s width',
     'let x2 = (exe::font_text_width(3usize, JOB_TEXT as *const u8) + 0x72).min(0x2f1); // tel22_1 mutation'),
    ('tel22_1_print_item_id',
     'let (style, id, value) = if enabled { (1, 0x19, data) } else { (0x21, 0, 0) }; // tel22_1 a Print popup item',
     'let (style, id, value) = if enabled { (1, 0x18, data) } else { (0x21, 0, 0) }; // tel22_1 mutation'),
]
