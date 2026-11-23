# escapadas da 324a para a comparacao longa (bancada 5)
MUTACOES = [
    ('gra12_acerto_comum', 'standard(0xa958d0); // gra12: the plain agreed terms', 'standard(0xa95e2c); // mutation: the agreed terms of another reason', 'gra_12 agreed terms'),
    ('gra14_proposta_por_taxa', 'fmt(NEWS_TEXT_BUFFER, 0xa9050c, g4, g4, g5, g5, n4, mp, sn5, np); // gra14: the plain offer for a fee', 'fmt(NEWS_TEXT_BUFFER, 0xa905b0, g4, g4, g5, g5, n4, mp, sn5, np); // mutation: the template of flag 2', 'gra_14 offer body'),
    ('gra1_sorteio_vinculo', 'news_link(out, drawn + club::SHORT_NAME, 0x7d5, drawn);', 'news_link(out, drawn + club::SHORT_NAME, 0x7d5, opponent);', 'gra_1 cup draw body'),
]
