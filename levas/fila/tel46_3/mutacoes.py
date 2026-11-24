# Rede: so se prova numa partida em rede (duas maquinas). O comparador monta o pacote da copia antes (rascunho) e o
# confere com o que a original deixou no buffer: as duas mutacoes sao acusadas por ele numa sessao em rede.
MUTACOES = [
    ('tel46_3_sempre_tudo', '    let mask = if delta.0 != 0 && delta.1 < full_size { delta.0 } else { full }; // tel46_3: the smaller mask', '    let mask = full; // mutation: never the changes'),
    ('tel46_3_formato_de_cor', '    network_buffer_put_char(b, (read_u32(SCREEN_GREEN_MASK) == GREEN_MASK_565) as u32); // tel46_3: the colour format', '    network_buffer_put_char(b, (read_u32(SCREEN_GREEN_MASK) != GREEN_MASK_565) as u32); // mutation'),
]
