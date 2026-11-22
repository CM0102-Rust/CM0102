# Mutacoes (tel21_5): a carreira automatica nao abre telas, entao a cadeia nao roda mutacoes nesta leva. Estas provam
# o comparador lado a lado pela lista de exibicao numa sessao do Lucas com o JOGAR_COMPARANDO.bat (Search > Filters):
# - o "+" do atributo fica botao ate 25 (pagina Attributes, um atributo com minimo 20);
# - a linha de Idade uma linha abaixo (pagina General de qualquer busca).
MUTACOES = [
    ('tel21_5_mais_ate_25',
     "        if minimum(a) < 0x14 {",
     "        if minimum(a) < 0x19 { // mutation: the plus button up to 25",
     'tel21_5 filters attributes page'),
    ('tel21_5_idade_abaixo',
     "    filters_screen_age_row(b, row + 1);",
     "    filters_screen_age_row(b, row + 2); // mutation: the Age row one row down",
     'tel21_5 filters general page'),
]
