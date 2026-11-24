# Mutacoes (sbA_6): a leitura do historico que perde registros muda as regras que olham o historico (o comparador do
# get_nodes, as duas listas lado a lado, as acusa na comparacao); o ultimo que nao vira -1 faz o proximo acrescimo
# escrever por cima do primeiro no.
MUTACOES = [
    ('sbA_6_le_so_o_primeiro',
     '    while next != -1 && first != last {',
     '    while next != -1 && first != last && false { // mutation'),
    ('sbA_6_ultimo_fica',
     '    *last = -1;\n}',
     '    let _ = last; // mutation\n}'),
]
