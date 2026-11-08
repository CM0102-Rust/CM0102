# Mutacoes das clausulas do contrato. Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('l_venda', "    remove_from_payment_list(manager, id, 8, 0x880, 0x888, 9);", "    remove_from_payment_list(manager, id, 8, 0x880, 0x888, 5);"),
    ('l_jogo', "    remove_from_payment_list(manager, id, 2, 0x88c, 0x894, 0);", "    remove_from_payment_list(manager, id, 1, 0x88c, 0x894, 0);"),
    ('l_conta', "    *((manager + how_many) as *mut i32) -= 1;", "    *((manager + how_many) as *mut i32) -= (j > i + 1) as i32;"),
]
