# Mutacoes (sx21_21). Com a Franca e a Italia carregadas: mudam quem os clubes franceses e italianos contratam
# (carreira e comparador).
MUTACOES = [
    ('sx21_21_franca_duas',
     'if count > 1 { // sx21_21: one of each in the restricted window',
     'if count > 2 { // mutation: two of each',
     'sx21_21 france offer over the window count'),
    ('sx21_21_italia_terceira_janela',
     'return refuse(); // sx21_21: the third window is for free transfers only',
     '// mutation: any signing in the third window',
     'sx21_21 italy third window not on a free'),
    ('sx21_21_italia_oferta_terceira',
     'return refuse(); // sx21_21: third window bids only on a free',
     '// mutation: any bid in the third window',
     'sx21_21 italy third window bid not on a free'),
]
