# Mutacoes (cpx_3), todas na transferencia efetivada (0x8c9390, 750 chamadas ate 2002-06-01 na cobertura de 17/10,
# pelo match_wrap_up da oferta 0x8c34e0). Sem o pagamento (0x8ca8a0: a primeira parcela entre os clubes e as clausulas
# guardadas), os saldos do comprador e do vendedor ficam outros e as verbas de transferencia mudam as ofertas seguintes.
# Sem a parte dos clubes anteriores (0x8e4f20: 5% do valor divididos entre os ate cinco clubes anteriores do jogador),
# os saldos desses clubes mudam. Sem a confianca das diretorias depois da transferencia (0x6978f0), a confianca do
# comprador e do vendedor fica outra e as demissoes e verbas mudam. As tres mudam saldos e decisoes dos clubes.
MUTACOES = [
    ('cpx3_sem_pagamento', '        com_a_oferta(0x8ca8a0, this); // cpx3: o pagamento da transferência',
     '        // mutacao: sem o pagamento',
     'cpx_3 paga a transferencia'),
    ('cpx3_sem_parte_dos_anteriores', '        com_a_oferta(0x8e4f20, this); // cpx3: a parte dos clubes anteriores',
     '        // mutacao: sem a parte dos clubes anteriores',
     'cpx_3 parte dos clubes anteriores'),
    ('cpx3_sem_confianca', '    confianca(cpx3_d(0xb67a44), o); // cpx3: a confiança das diretorias',
     '    let _ = confianca; // mutacao: sem a confianca das diretorias',
     'cpx_3 confianca das diretorias'),
]
