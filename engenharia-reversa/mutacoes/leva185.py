# Mutacoes das marcas do gerente de transferencias. Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('t_marca', "    *staff_flags_ptr(TransferManagerFields(manager), id) |= 0x40;", "    *staff_flags_ptr(TransferManagerFields(manager), id) |= 0x20;"),
    ('t_desmarca', "    *staff_flags_ptr(TransferManagerFields(manager), id) &= 0xbf;", "    *staff_flags_ptr(TransferManagerFields(manager), id) &= 0x3f;"),
    ('t_vaga', "    if k < (table.offer_capacity() as i32) {", "    if k <= (table.offer_capacity() as i32) {"),
]
