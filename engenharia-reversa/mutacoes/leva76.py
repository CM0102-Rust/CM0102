# Mutacoes da leva76 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva76.py
MUTACOES = [
    ('l_9', "(*(person as *const i32) as usize).wrapping_mul(9));\n    let base", "(*(person as *const i32) as usize).wrapping_mul(8));\n    let base"),
    ('l_37', "        let e = base.wrapping_add(k * 37);", "        let e = base.wrapping_add(k * 36);"),
    ('l_40', "    for k in 0..40usize {\n        let e = base", "    for k in 0..39usize {\n        let e = base"),
    ('l_13', "(tambem_13 != 0 && *((e + 0x13) as *const u32) == id)", "(*((e + 0x13) as *const u32) == id)"),
    ('l_tipo', "        if home && (not_11 != 0) == (*(e as *const u8) != 11) {", "        if home && (not_11 != 0) == (*(e as *const u8) != 12) {"),
    ('b_40', "    *((this + 0x40) as *const u32)\n}", "    *((this + 0x44) as *const u32)\n}"),
]
