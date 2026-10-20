# Mutacoes das clausulas do contrato. Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('l_venda', "    tirar_da_lista_de_pagamentos(gerente, id, 8, 0x880, 0x888, 9);", "    tirar_da_lista_de_pagamentos(gerente, id, 8, 0x880, 0x888, 5);"),
    ('l_jogo', "    tirar_da_lista_de_pagamentos(gerente, id, 2, 0x88c, 0x894, 0);", "    tirar_da_lista_de_pagamentos(gerente, id, 1, 0x88c, 0x894, 0);"),
    ('l_conta', "    *((gerente + quantos) as *mut i32) -= 1;", "    *((gerente + quantos) as *mut i32) -= (j > i + 1) as i32;"),
]
