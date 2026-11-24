# Mutacoes (sx04b_4), na janela ate 2002-08-01 (os premios de junho):
# - os candidatos dos continentes por outro modo de ordem (outros vencedores: o comparador acusa o registro; a
#   reputacao dos vencedores muda e a carreira tambem);
# - os vencedores da lista curta na ordem inversa (o comparador acusa o registro; o primeiro lugar muda de reputacao).
MUTACOES = [
    ('sx04b_4_outro_modo',
     'if (collect(ratings, buf, 3, continent, positions, mode, list) as i16) < 3 { // sx04b_4: three candidates',
     'if (collect(ratings, buf, 3, continent, positions, mode ^ 1, list) as i16) < 3 { // mutation: another order',
     'sx04b_4 top three from the ratings'),
    ('sx04b_4_lista_invertida',
     'let people = [0, 1, 2].map(|k| Staff(read_usize(buf + 4 * k)));',
     'let people = [2, 1, 0].map(|k| Staff(read_usize(buf + 4 * k))); // mutation: reversed',
     'sx04b_4 winners from the shortlist'),
]
