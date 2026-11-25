# Mutacoes (rx04_1). O texto nao volta a simulacao: as mutacoes sao acusadas pelos comparadores (a busca lado a lado e
# o langlib_sprintf), como diz o guia; na cadeia, roda-las com a comparacao ligada. O jogo de teste esta em portugues.
# - a traducao copiada do formato dos argumentos (todo texto traduzido muda);
# - a chave com os espacos (quase nenhum texto e achado);
# - a ordem das chaves invertida (a busca binaria se perde);
# - a lingua devolvida + 1.
MUTACOES = [
    ('rx041_traducao_do_formato',
     '        langlib_replace_string(entry + slot, read_usize(from + slot));',
     '        langlib_replace_string(entry + slot, read_usize(from + if slot == LANGLIB_ENTRY_TRANSLATION { LANGLIB_ENTRY_ARG_FORMAT } else { slot })); // mutation',
     'rx04_1 lookup found'),
    ('rx041_chave_com_espacos',
     "            b'{' | b'}' | b' ' => {}",
     "            b'{' | b'}' => {} // mutation: the spaces stay",
     'rx04_1 lookup in table'),
    ('rx041_ordem_invertida',
     '            return if c < d { -1 } else { 1 };',
     '            return if c < d { 1 } else { -1 }; // mutation',
     'rx04_1 compare keys'),
    ('rx041_lingua_mais_um',
     '    covered!("rx04_1 get language");\n    read_i32(G_LANGLIB_LANGUAGE)',
     '    covered!("rx04_1 get language");\n    read_i32(G_LANGLIB_LANGUAGE) + 1 // mutation',
     'rx04_1 get language'),
]
