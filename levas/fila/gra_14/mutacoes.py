# Mutacoes (gra_14). O texto nao volta para a simulacao: acusada pela comparacao (compare_text_writer compara o texto
# e os vinculos), como as de texto aceitas em 12/11/2026.
# - a proposta simples por taxa (vendedor de outro tecnico, tipo "outro", sem bandeiras) com o modelo da bandeira 2.
MUTACOES = [
    ('gra14_proposta_por_taxa',
     'fmt(NEWS_TEXT_BUFFER, 0xa9050c, g4, g4, g5, g5, n4, mp, sn5, np); // gra14: the plain offer for a fee',
     'fmt(NEWS_TEXT_BUFFER, 0xa905b0, g4, g4, g5, g5, n4, mp, sn5, np); // mutation: the template of flag 2',
     'gra_14 offer body'),
]
