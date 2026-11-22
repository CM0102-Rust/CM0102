# Mutacoes (tel21_2): a carreira automatica nao abre telas, entao a cadeia nao roda mutacoes nesta leva. Estas provam
# o comparador (compare_order!) numa sessao do Lucas com o JOGAR_COMPARANDO.bat (busca de comissao/jogadores):
# - a idade em ordem crescente (ordenar pela coluna de idade);
# - o jovem abaixo de 3750 sem o piso (ordenar pela reputacao com jogadores jovens de pouca reputacao).
MUTACOES = [
    ('tel21_2_idade_crescente',
     "    let d = age(b) - age(a);",
     "    let d = age(a) - age(b); // mutation: youngest first",
     'tel21_2 sort by age'),
    ('tel21_2_sem_piso_do_jovem',
     "if read_i8(search_result_person(e).addr() + staff::AGE) < 0x23 { SEARCH_REPUTATION_ESTABLISHED } else { r }",
     "if read_i8(search_result_person(e).addr() + staff::AGE) < 0x10 { SEARCH_REPUTATION_ESTABLISHED } else { r } // mutation: no floor for the young",
     'tel21_2 sort by reputation'),
]
