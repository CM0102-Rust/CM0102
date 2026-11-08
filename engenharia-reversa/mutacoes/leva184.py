# Mutacoes da disciplina. Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('d_sem_regra', "    if k == -1 {\n        return;\n    }\n    let rules", "    if k <= 0 {\n        return;\n    }\n    let rules"),
    ('d_43', "    for k in 0..43usize {", "    for k in 0..42usize {"),
    ('d_tipo', "    *(a as *mut u8) = kind;", "    *(a as *mut u8) = kind.wrapping_add(1);"),
    ('d_valor', "    core::ptr::write_unaligned((a + 1) as *mut u32, value);", "    core::ptr::write_unaligned((a + 1) as *mut u32, value.wrapping_add(1));"),
]
