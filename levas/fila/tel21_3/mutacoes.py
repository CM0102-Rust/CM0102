# Mutacoes (tel21_3): a carreira automatica nao abre telas, entao a cadeia nao roda mutacoes nesta leva. Estas provam
# o comparador (compare_order!) numa sessao do Lucas com o JOGAR_COMPARANDO.bat:
# - o salario em ordem decrescente (busca, ordenar pela coluna de salario);
# - os nomes dos atributos de jogador ate 30 (tela de filtros da busca, pagina Attributes: o menu dos atributos).
MUTACOES = [
    ('tel21_3_salario_decrescente',
     "    let d = read_i32(ca + contract::WAGE).wrapping_sub(read_i32(cb + contract::WAGE));",
     "    let d = read_i32(cb + contract::WAGE).wrapping_sub(read_i32(ca + contract::WAGE)); // mutation: highest first",
     'tel21_3 sort by wage'),
    ('tel21_3_atributo_31_invalido',
     "search_compare_attribute_names(a as *const u8, b as *const u8, 0x1f, SEARCH_FN_PLAYER_ATTRIBUTE_NAME, 0)",
     "search_compare_attribute_names(a as *const u8, b as *const u8, 0x1e, SEARCH_FN_PLAYER_ATTRIBUTE_NAME, 0) // mutation: attribute 31 invalid",
     'tel21_3 sort player attribute names'),
]
