# Mutacoes (rx06_4), item de TELAS: a carreira automatica nao abre noticias; servem para provar o comparador numa
# sessao do Lucas jogando comparando (abrir a noticia de busca iniciada):
# - o titulo da busca por nacao com a frase da competicao (o comparador do texto acusa);
# - o corpo da busca de jovens com a frase do titulo (o comparador do texto acusa);
# - o vinculo da busca como vinculo de pessoa (o comparador acusa os vinculos).
MUTACOES = [
    ('rx06_4_titulo_nacao',
     'ScoutSearchSubject::Nation(_) => fmt(buf, 0xa70304, g, g, w), // "{}<%s - nationality>{} search started" (rx06_4)',
     'ScoutSearchSubject::Nation(_) => fmt(buf, 0xa702b4, g, g, w), // mutation: the competition sentence'),
    ('rx06_4_jovens',
     'story_lang!(w, 0xa70398); // "search for youth players"',
     'story_lang!(w, 0xa7029c); // mutation: the headline sentence'),
    ('rx06_4_vinculo',
     'exe::text_add_link(out, text.as_ptr(), 0x7e4, self.slot as i8 as i32 as usize);',
     'exe::text_add_link(out, text.as_ptr(), 0x7d7, self.slot as i8 as i32 as usize); // mutation: a person link'),
]
