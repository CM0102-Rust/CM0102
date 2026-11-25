# Mutacoes (rx13_5): o genero so muda o artigo do texto da noticia, que nao volta a simulacao; acusa o comparador (o
# byte lado a lado) a cada noticia de lesao, na lingua do jogo de teste (fora das linguas 9, 3 e 5: o ramo do 0).
MUTACOES = [
    ('rx13_5_outra_lingua_com_genero',
     '_ => return 0, // rx13_5: no gender',
     '_ => return 1, // mutation',
     'rx13_5 injury name gender'),
]
