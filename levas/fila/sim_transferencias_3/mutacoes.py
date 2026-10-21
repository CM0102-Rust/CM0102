# Mutacoes do fechamento das propostas, da fila da transferencia livre e das negociacoes de troca. Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('f_estado', "    if r != 0 {\n        *((r + 0x50) as *mut u8) = 4;", "    if r != 0 {\n        *((r + 0x50) as *mut u8) = 3;"),
    ('l_motivo', "    r[4] = motivo;", "    r[4] = motivo ^ 1;"),
    ('n_contrato', "    *(0xae1080 as *mut usize) = c;\n    if c == 0 {\n        return 0;\n    }\n    let p =", "    *(0xae1080 as *mut usize) = c | 1;\n    if c == 0 {\n        return 0;\n    }\n    let p ="),
]
