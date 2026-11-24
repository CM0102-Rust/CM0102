# Mutacoes (sx09b_2).
# - a chave -1 deixa de casar com qualquer pessoa: o bsearch por nacao (nation_index_range) nao acha os suspensos da
#   FIFA nos jogos de selecao, que deixam de cumprir o jogo; o comparador acusa em cada busca com a chave.
# A metade e o zero da contagem de multas ficam sem mutacao: na carreira de ferias a contagem e 0 em todos os clubes
# (ninguem multa), e 0 pela metade ou zerado continua 0; o comparador lado a lado confere o byte a cada chamada (as duas
# rodam: a temporada nova chama a metade para cada clube e a saida de tecnico chama o zero).
MUTACOES = [
    ('sx09b_chave_sem_curinga',
     'if id_a == -1 || id_b == -1 { // sx09b_2: the key matches any person',
     'if false { // mutation',
     'sx09b_2 nation index key'),
]
