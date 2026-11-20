# Mutacoes (sma_9). O texto nao volta para a simulacao: acusadas pelos comparadores (injury_get_type_name lado a lado,
# a 0x631580 por compare_text_writer), como as de texto aceitas em 12/11/2026.
# - o nome sem artigo trocado pelo nome com artigo (a volta aos treinos leves de uma lesao de categoria 6);
# - o tempo previsto exato em vez de aproximado (toda volta aos treinos leves de lesao comum).
MUTACOES = [
    ('sma9_nome_com_artigo',
     'translate(buffer, INJURY_TYPE_NAMES[k as usize]) // sma9: the name without the article',
     'translate(buffer, INJURY_TYPE_NAMES_WITH_ARTICLE[k as usize]) // mutation: always with the article',
     'sma_9 light training body'),
    ('sma9_tempo_exato',
     'format_duration_text(days as u16 as i16 as i32, time.as_mut_ptr() as usize, 0x32, 1); // sma9: the approximate time',
     'format_duration_text(days as u16 as i16 as i32, time.as_mut_ptr() as usize, 0x32, 0); // mutation: the exact time',
     'sma_9 light training body'),
]
