# Mutacoes (cqx_12), na lista dos clubes da Copa da UEFA (3 chamadas) e nos semifinalistas eliminados das copas
# nacionais (8 chamadas: as semifinais das copas, na primavera de 2002). Nenhuma das duas muda a carreira ate junho (a
# lista vai para quem a pede; os semifinalistas so contam nas vagas da UEFA da temporada seguinte): sao para os
# comparadores lado a lado, e a confirmacao no modo comparando precisa chegar a data das chamadas (ver PRONTO.txt).
# Os registros da Mercosul sao passageiros (sem mutacao).
MUTACOES = [
    ('cqx12_lista_sem_o_visitante',
     'if away != -1 { // cqx12: the away club of the tie',
     'if false && away != -1 { // mutation: without the away club',
     'cqx_12 uefa cup round clubs'),
    ('cqx12_semifinalistas_trocados',
     'write_u32(e + 4, club_a); // cqx12: the first semi-final loser',
     'write_u32(e + 4, club_b); // mutation: the second in the first place',
     'cqx_12 semi-final losers'),
]
