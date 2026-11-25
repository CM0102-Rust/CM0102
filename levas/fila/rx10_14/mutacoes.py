# Mutacoes (rx10_14), com as 8 ligas ate 2003-08-15. Nenhuma abre caixa.
# - os dois grupos trocados (o terceiro clube de cada pote no segundo grupo e o segundo no terceiro): outros
#   confrontos em julho de 2003 (a carreira; o comparador nao, ele le o plano da mesma lista).
MUTACOES = [
    ('rx10_14_grupos_trocados',
     'write_usize(list + 4 * pot, read_usize(read_usize(this + 0xee) + 4 * (3 * pot + 1 + k as usize))); // rx10_14: the k-th next club of each pot',
     'write_usize(list + 4 * pot, read_usize(read_usize(this + 0xee) + 4 * (3 * pot + 2 - k as usize))); // mutation',
     'rx10_14 club world championship other groups'),
]
