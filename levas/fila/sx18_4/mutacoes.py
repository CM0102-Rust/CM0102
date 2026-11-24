# Mutacoes (sx18_4). As vagas so voltam ao texto das noticias (nao a simulacao): acusadas pelo comparador de cada
# rotina (as vagas gravadas pela original contra os valores da copia; o registro lido pela original contra o da copia),
# como as de texto aceitas em 12/11/2026. Todas em trechos que rodam em toda chamada.
# - os gols a favor gravados com sinal trocado de largura (o byte sem sinal);
# - os gols contra lidos da vaga dos gols a favor;
# - a referencia gravada sem o id da pessoa (o primeiro nome no lugar).
MUTACOES = [
    ('sx18_4_gols_sem_sinal',
     'read_i8(record + club_match_record::GOALS_FOR) as i32 as u32, // sx18_4: the goals for',
     'read_u8(record + club_match_record::GOALS_FOR) as u32 + 1, // mutation',
     'sx18_4 match record to news'),
    ('sx18_4_gols_contra',
     'write_u8(record + club_match_record::GOALS_AGAINST, v[7] as u8); // sx18_4: the goals against',
     'write_u8(record + club_match_record::GOALS_AGAINST, v[6] as u8); // mutation',
     'sx18_4 match record from news'),
    ('sx18_4_referencia_sem_id',
     '     read_u32(r + record_holder::STAFF_ID), day, year]',
     '     read_u32(r), day, year] // mutation',
     'sx18_4 name reference to news'),
]
