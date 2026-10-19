# Mutacoes da disciplina. Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('d_sem_regra', "    if k == -1 {\n        return;\n    }\n    let regras", "    if k <= 0 {\n        return;\n    }\n    let regras"),
    ('d_43', "    for k in 0..43usize {", "    for k in 0..42usize {"),
    ('d_tipo', "    *(a as *mut u8) = tipo;", "    *(a as *mut u8) = tipo.wrapping_add(1);"),
    ('d_valor', "    core::ptr::write_unaligned((a + 1) as *mut u32, valor);", "    core::ptr::write_unaligned((a + 1) as *mut u32, valor.wrapping_add(1));"),
]
