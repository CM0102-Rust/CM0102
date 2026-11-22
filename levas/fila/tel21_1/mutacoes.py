# Mutacoes (tel21_1): a carreira automatica nao abre telas, entao a cadeia nao roda mutacoes nesta leva. Estas provam
# o comparador (compare_order!, o valor lado a lado) numa sessao do Lucas com o JOGAR_COMPARANDO.bat:
# - sem nota contando como nota normal (busca de jogadores, ordenar pela coluna da nota com algum jogador sem nota);
# - a situacao em ordem decrescente (ordenar pela coluna de situacao com dois codigos diferentes).
MUTACOES = [
    ('tel21_1_sem_nota_no_fim',
     "    if ra == 0xff && rb != 0xff {\n        return 1;\n    }",
     "    if ra == 0xfe && rb != 0xff { // mutation: the unrated no longer go last\n        return 1;\n    }",
     'tel21_1 sort by note rating'),
    ('tel21_1_situacao_decrescente',
     "let d = read_u8(a + search_result_keys::STATUS_CODE).wrapping_sub(read_u8(b + search_result_keys::STATUS_CODE)) as i8;",
     "let d = read_u8(b + search_result_keys::STATUS_CODE).wrapping_sub(read_u8(a + search_result_keys::STATUS_CODE)) as i8; // mutation: descending",
     'tel21_1 sort by status code'),
]
