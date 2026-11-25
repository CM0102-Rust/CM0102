# Mutacoes (sx09c_3) pela prova do save (mutacoes.py --prova-save N), sem o 4o campo (a base da prova roda com as
# rotinas desligadas):
# - a gravacao poe a quantidade de pessoas mais um no dispute.dat: o item sai com 4 bytes diferentes em qualquer save.
# A carga fica sem mutacao propria: o save de ferias provavelmente nao tem disputa (as marcas todas 0 e a lista vazia),
# e o construtor, quando a carga falha, refaz exatamente isso (calloc e lista vazia); uma mutacao na carga escaparia.
MUTACOES = [
    ('sx09c_pessoas_mais_um',
     'if exe::zwrite(G_STAFF_COUNT, 4, 1, z) != 1 { // sx09c_3: the people count',
     'let wrong = read_u32(G_STAFF_COUNT) + 1; if exe::zwrite(&wrong as *const u32 as usize, 4, 1, z) != 1 { // mutation'),
]
