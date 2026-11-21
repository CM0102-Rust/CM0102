# Mutacoes (gra_11). O texto nao volta para a simulacao: acusada pela comparacao (compare_text_writer compara o texto
# e os vinculos), como as de texto aceitas em 12/11/2026.
# - o vinculo do comprador no corpo trocado pelo do vendedor (todo corpo de transferencia acertada com vendedor).
MUTACOES = [
    ('gra11_vinculo_do_comprador',
     "news_text_link_club(out, buyer); // gra11: the buyer's link in the body",
     "news_text_link_club(out, seller); // mutation: the seller's link in place of the buyer's",
     'gra_11 finalised body'),
]
