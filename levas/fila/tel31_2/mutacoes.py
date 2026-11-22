# Mutations (tel31_2), to prove the comparers in a session of Lucas's (JOGAR_COMPARANDO.bat):
# - the result records ignore the venue (records screen of a competition: the text comparer catches the names);
# - the rating record goes into the news ×10 instead of ×100 (record news of the simulation, news item side by side).
MUTACOES = [
    ('tel31_2_result_venue',
     'let (first, second) = if venue == 2 {',
     'let (first, second) = if venue == 0xff {'),
    ('tel31_2_news_rating_scale',
     'read_f32(rec + 0x189) as f64 * read_f32(0x968960) as f64',
     'read_f32(rec + 0x189) as f64 * 10.0'),
]
