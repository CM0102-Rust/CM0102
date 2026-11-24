# Mutacoes (sx05b_23), pelo comparador lado a lado (o id da moeda) na carga das preferencias. A troca de bytes nao roda
# na carreira (so com arquivos de tatica da outra ordem de bytes): sem mutacao dela.
MUTACOES = [
    ('sx05b_23_id_errado',
     '    read_u8(CURRENCY_ID)\n}',
     '    read_u8(CURRENCY_ID) ^ 1 // mutation\n}',
     'sx05b_23 currency read'),
]
