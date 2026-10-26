# Mutacoes das regioes e do laco de opiniao. As tres so rodam nas telas (o filtro de regiao das telas de busca; a aba
# de transferencias do perfil da pessoa): a carreira oculta nao passa por elas e ESCAPAM (lacuna); os comparadores
# rodam no jogo real comparando, nas telas (ver o LEIA da leva).
MUTACOES = [
    ('dbt_regioes_pelo_nome', 'qsort(0xae2104, 14, 1, 0x545e00); // (dbt)', 'qsort(0xae2104, 13, 1, 0x545e00); // mutacao: a ultima regiao fora da ordem'),
    ('dbt_ordem_das_regioes', '    dbr_comparar_textos(na.as_ptr(), nb.as_ptr()) // (dbt)', '    dbr_comparar_textos(nb.as_ptr(), na.as_ptr()) // mutacao: a ordem invertida'),
    ('dbt_pessoa_da_opiniao', '        ler(0x3b) // (dbt)', '        ler(0x27) // mutacao: o indice do outro lado'),
]
