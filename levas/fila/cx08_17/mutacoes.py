# Mutacoes (cx08_17), com a Belgica ate 2002-08-01:
# - no grupo da repescagem o codigo atual sobe dois (o comparador do progresso acusa).
# A noticia do campeao de periodo fica sem mutacao propria: o comparador roda so a original e a noticia nao volta a
# simulacao (trocar o tipo arrisca uma noticia sem modelo).
MUTACOES = [
    ('cx08_17_repescagem_mais_dois',
     '(c, b, w) = (c.wrapping_add(1), b.wrapping_add(1), w.wrapping_add(1));',
     '(c, b, w) = (c.wrapping_add(2), b.wrapping_add(1), w.wrapping_add(1)); // mutation',
     'cx08_17 belgian play-off progress'),
]
