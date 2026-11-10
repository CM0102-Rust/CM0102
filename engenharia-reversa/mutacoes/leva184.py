# Mutacoes da disciplina. Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('d_sem_regra', "    if k == -1 {\n        return;\n    }\n    let rules", "    if k <= 0 {\n        return;\n    }\n    let rules"),
    ('d_43', "    for k in 0..43usize {", "    for k in 0..42usize {"),
    ('d_tipo', "    write_u8(a, kind);", "    write_u8(a, kind.wrapping_add(1));"),
    ('d_valor', "    write_u32(a + ban_slot::VALUE, value);", "    write_u32(a + ban_slot::VALUE, value.wrapping_add(1));"),
]
