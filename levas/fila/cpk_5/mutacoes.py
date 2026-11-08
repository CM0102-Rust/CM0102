# Mutacoes. O continente com o clube de fora (so o modo Rust e a carreira: o comparando roda a original; a noticia vai
# para a caixa do humano: PODE ESCAPAR da impressao). A competicao sem a nacao (vai para o continente ou para todos;
# idem). A oferta pelo clube do humano com a importancia trocada (o comparando acusa na primeira oferta pelos
# jogadores do humano; a carreira, se a importancia mudar a caixa ou as ferias).
MUTACOES = [
    ('cpk_continente_clube', 'if n != 0 && cpk5_d(n + 0x71) == continente { // o continente da nação do clube',
     'if n != 0 && cpk5_d(n + 0x71) != continente { // mutacao: os de fora'),
    ('cpk_regiao_nacao', 'let nacao = cpk5_d(comp + 0x5d);',
     'let nacao = 0usize; // mutacao: sem a nacao'),
    ('cpk_importante_oferta', 'manager_of_news_club(noticia, 0x19) == pessoa // o técnico do clube da oferta',
     'manager_of_news_club(noticia, 0x19) != pessoa // mutacao: trocada'),
]
