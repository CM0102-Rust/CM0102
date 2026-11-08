# Mutacoes, todas no regen jovem posto no clube (0x7a6f10, pela 0x7abac0 em todo regen jovem da carreira: centenas ate
# 2002-06-01). O valor (+0x52) entra no contrato novo (0x4c7950) e o fim do contrato (+0x46) fica na ficha da pessoa
# (bloco das pessoas, na impressao diaria da memoria); a 0x547d90 uma vez em quatro mexe na pessoa e no clube. Os regens
# jovens tem menos de 19 anos: o ramo do jovem roda em quase todos.
MUTACOES = [
    ('cpv9_jovem_tres_anos',
     'let a = ano.wrapping_add(2); // cpv9: o jovem por dois anos',
     'let a = ano.wrapping_add(3); // mutacao: tres anos',
     'cpv_9 youngster contract'),
    ('cpv9_valor_mais_um',
     'cpv9_por(pessoa + 0x52, v); // cpv9: o valor do regen',
     'cpv9_por(pessoa + 0x52, v.wrapping_add(1)); // mutacao: valor + 1',
     'cpv_9 regen signed'),
    ('cpv9_metade_das_vezes',
     'if sorteio(4) == 0 { // cpv9: uma vez em quatro',
     'if sorteio(4) < 2 { // mutacao: metade das vezes',
     'cpv_9 one in four'),
]
