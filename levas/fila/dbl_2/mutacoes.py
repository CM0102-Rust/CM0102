# Mutacoes dos vinculos de opiniao. A de limpar grava -2 no valor dos dois vinculos (a criacao limpa o vinculo de
# cada contrato gerado; a tabela de contratos vai para o .qst). A de juntar grava o segundo vinculo sem o id e a de
# tirar esvazia o segundo vinculo so no valor: as duas so rodam com eventos e queixas de contrato, na carreira; nos
# dias ate a data de inicio da criacao do .qst talvez nenhum dispare: PODEM ESCAPAR da prova do save (lacuna); os
# comparadores delas rodam no jogo real comparando da cadeia.
MUTACOES = [
    ('dbl_limpar_vinculos', '            opiniao_gravar(c, k + 0x37, -1); // o valor do vínculo vazio', '            opiniao_gravar(c, k + 0x37, -2); // mutacao: o valor do vínculo vazio -2'),
    ('dbl_juntar_vinculo', '        opiniao_gravar(c, 0x3b, id); // o segundo vínculo', '        // mutacao: o segundo vínculo sem o id'),
    ('dbl_tirar_vinculo', '        opiniao_gravar(c, 0x3b, -1); // esvazia o segundo vínculo', '        // mutacao: o segundo vínculo fica com o id'),
]
