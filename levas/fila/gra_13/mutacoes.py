# Mutacoes (gra_13). O texto nao volta para a simulacao: acusada pela comparacao (compare_text_writer compara o texto
# e os vinculos), como as de texto aceitas em 12/11/2026.
# - a contratacao por taxa sem troca (tipo "outro") com os clubes trocados.
MUTACOES = [
    ('gra13_contratou_por_taxa',
     '_ => { fmt(NEWS_TEXT_BUFFER, 0xa981a4, g4, g4, g5, g5, sn4, np, sn5, mp); } // gra13: signed for a fee',
     '_ => { fmt(NEWS_TEXT_BUFFER, 0xa981a4, g4, g4, g5, g5, sn5, np, sn4, mp); } // mutation: the clubs swapped',
     'gra_13 completed body'),
]
