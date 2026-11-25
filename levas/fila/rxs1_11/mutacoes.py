# Mutação (rxs1_11), na gravação: o registro das transferências passadas com um byte a menos. Muda o transfer.dat do
# .qst gravado (a prova do save compara a gravação com e sem a rotina) e a carga lê fora de passo.
MUTACOES = [
    ('rxs1_11_passadas',
     '(f::PAST_TRANSFER_COUNT, f::PAST_TRANSFERS, 0x25, (0x3e7, 0xa8e63c), (0x3f0, 0xa8e5f8)), // rxs1_11: past transfers',
     '(f::PAST_TRANSFER_COUNT, f::PAST_TRANSFERS, 0x24, (0x3e7, 0xa8e63c), (0x3f0, 0xa8e5f8)), // mutation',
     'rxs1_11 transfer manager saved'),
]
