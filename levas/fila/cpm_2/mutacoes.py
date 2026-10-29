# Mutacoes (com o ramo de cada uma; o comparando roda a original nas duas que alocam: so o modo Rust e a carreira
# acusam). Os confrontos diretos com o proprio clube no lugar do adversario (nas ligas com a regra 4: a minitabela do
# desempate em pontos, 0x682b30, deixa de achar os jogos entre os empatados e a ordem da tabela muda). O vetor dos
# clubes sem o ultimo (poucas chamadas, pela vtable: PODE ESCAPAR). A posicao final uma abaixo (o comparando acusa
# pelo byte; a carreira pela ordem pela posicao da temporada passada, 0x4b0780, no fim da temporada: depois de
# 2002-06-01 PODE ESCAPAR).
MUTACOES = [
    ('cpm_confronto_adversario', 'cpm2_por(p + 14 * k, cpm2_d(tab + j as usize * CPM2_LINHA)); // o clube adversário',
     'cpm2_por(p + 14 * k, cpm2_d(tab + a as usize * CPM2_LINHA)); // mutacao: o proprio clube',
     'cpm_2 confrontos diretos'),
    ('cpm_vetor_conta', '*n = cpm2_w(this + 0x3e); // a conta',
     '*n = cpm2_w(this + 0x3e) - 1; // mutacao: sem o ultimo',
     'cpm_2 vetor dos clubes'),
    ('cpm_posicao_final', '*((c + 0x5f) as *mut u8) = (i as u8).wrapping_add(1); // a posição final',
     '*((c + 0x5f) as *mut u8) = (i as u8).wrapping_add(2); // mutacao: uma abaixo',
     'cpm_2 posicao final'),
]
