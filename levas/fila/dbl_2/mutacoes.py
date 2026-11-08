# Mutacoes dos vinculos de opiniao. A de limpar grava -2 no valor dos dois vinculos (a criacao limpa o contract_bond de
# cada contrato gerado; a tabela de contratos vai para o .qst). A de juntar grava o segundo contract_bond sem o id e a de
# tirar esvazia o segundo contract_bond so no valor: as duas so rodam com eventos e queixas de contrato, na carreira; nos
# dias ate a data de inicio da criacao do .qst talvez nenhum dispare: PODEM ESCAPAR da prova do save (lacuna); os
# comparadores delas rodam no jogo real comparando da cadeia.
MUTACOES = [
    ('dbl_limpar_vinculos', '            write_state_i32(c, k + 0x37, -1); // o valor do vínculo vazio', '            write_state_i32(c, k + 0x37, -2); // mutacao: o valor do vínculo vazio -2'),
    ('dbl_juntar_vinculo', '        write_state_i32(c, 0x3b, id); // o segundo vínculo', '        // mutacao: o segundo vínculo sem o id'),
    ('dbl_tirar_vinculo', '        write_state_i32(c, 0x3b, -1); // esvazia o segundo vínculo', '        // mutacao: o segundo vínculo fica com o id'),
]
