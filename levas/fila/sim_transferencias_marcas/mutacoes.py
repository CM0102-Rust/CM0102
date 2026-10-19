# Mutacoes das marcas do gerente de transferencias. Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('t_marca', "    *marcas_da_pessoa(gerente, id) |= 0x40;", "    *marcas_da_pessoa(gerente, id) |= 0x20;"),
    ('t_desmarca', "    *marcas_da_pessoa(gerente, id) &= 0xbf;", "    *marcas_da_pessoa(gerente, id) &= 0x3f;"),
    ('t_vaga', "    if k < *((tabela + 4) as *const i32) {", "    if k <= *((tabela + 4) as *const i32) {"),
]
