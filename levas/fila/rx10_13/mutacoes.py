# Mutacoes (rx10_13), com as 8 ligas ate 2003-08-15. Nenhuma abre caixa.
# - as posicoes escritas sem ordenar a lista: os lugares saem fora da ordem da reputacao e o embaralhamento recebe
#   outra ordem (a carreira: os potes do sorteio desempatam pela ordem da lista, e as expectativas dos clubes mudam).
#   O comparador nao a ve (so a original roda: ela embaralha). Pode escapar se nenhum desempate do sorteio mudar; nesse
#   caso a rotina fica provada so pela comparacao.
MUTACOES = [
    ('rx10_13_sem_ordenar',
     '    predicted_by_reputation_sort(read_usize(this + 0xee), read_u32(this + 0xf2)); // rx10_13: the twelve by reputation',
     '    // mutation: not sorted',
     'rx10_13 club world championship predicted'),
]
