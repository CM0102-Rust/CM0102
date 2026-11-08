# Mutacoes (cpy_2), na situacao da pessoa nas listas dos clubes (0x843e10, 4599 chamadas ate 2002-06-01 na cobertura
# de 17/10, pelas ofertas, contratos e regens). Sem preencher de novo a vaga esvaziada no evento 1 (0x83f3a0), o clube
# do computador fica sem alvo nela ate a proxima avaliacao: lances e transferencias mudam. Sem descontar a lista na
# conta da pessoa (nibble baixo de +0xb do registro de 79 bytes), a 0x842c00 (ate cinco clubes por jogador) e a busca
# dos alvos veem outra conta. O bloqueio do evento 5 com 120 dias em vez de 14 deixa a pessoa fora da vaga por mais
# tempo e o clube procura outro alvo.
MUTACOES = [
    ('cpy2_vaga_sem_refazer', '                preencher(this, v, clube, pessoa); // cpy2: a vaga é preenchida de novo',
     '                let _ = preencher; // mutacao: a vaga fica vazia',
     'cpy_2 vaga refeita'),
    ('cpy2_sem_descontar_lista', '            *r = (*r & 0xf0) | ((*r).wrapping_sub(1) & 0xf); // cpy2: uma lista a menos na conta da pessoa',
     '            let _ = r; // mutacao: a conta de listas da pessoa nao cai',
     'cpy_2 contagem da lista'),
    ('cpy2_bloqueio_de_120_dias', '                block_slot_for_person(v, entrada, pessoa, contrato, 0x54c5b0, 0xe); // cpy2: o bloqueio de 14 dias',
     '                block_slot_for_person(v, entrada, pessoa, contrato, 0x54c5b0, 0x78); // mutacao: 120 dias',
     'cpy_2 bloqueio de 14 dias'),
]
