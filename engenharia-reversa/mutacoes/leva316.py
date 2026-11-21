# Mutações da 316a leva (itens sma_9, sma_10). Uso: python mutacoes.py mutacoes\leva316.py --carreira 2002-06-01
MUTACOES = [
    ('sma9_nome_com_artigo', 'translate(buffer, INJURY_TYPE_NAMES[k as usize]) // sma9: the name without the article', 'translate(buffer, INJURY_TYPE_NAMES_WITH_ARTICLE[k as usize]) // mutation: always with the article', 'sma_9 light training body'),
    ('sma9_tempo_exato', 'format_duration_text(days as u16 as i16 as i32, time.as_mut_ptr() as usize, 0x32, 1); // sma9: the approximate time', 'format_duration_text(days as u16 as i16 as i32, time.as_mut_ptr() as usize, 0x32, 0); // mutation: the exact time', 'sma_9 light training body'),
    ('sma10_vinculo_do_clube', "if exe::crt_strstr(buf, c) != 0 { // sma10: the club's link when its name is in the text", "if exe::crt_strstr(buf, c) == 0 { // mutation: the club's link only when its name is not in the text", 'sma_10 departure body'),
]
