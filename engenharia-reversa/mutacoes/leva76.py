# Mutacoes da leva76 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva76.py
MUTACOES = [
    ('l_9', "(read_i32(person) as usize).wrapping_mul(9));\n    let base", "(read_i32(person) as usize).wrapping_mul(8));\n    let base"),
    ('l_37', "        let e = ShortlistSlot(base.wrapping_add(k * 37));", "        let e = base.wrapping_add(k * 36);"),
    ('l_40', "    for k in 0..40usize {\n        let s = ShortlistSlot(read_usize(entry)", "    for k in 0..39usize {\n        let s = ShortlistSlot(read_usize(entry)"),
    ('l_13', "(tambem_13 != 0 && e.replacement_staff_id() == id)", "(e.replacement_staff_id() == id)"),
    ('l_tipo', "        if home && (not_11 != 0) == (read_u8(e.addr()) != 11) {", "        if home && (not_11 != 0) == (read_u8(e.addr()) != 12) {"),
    ('b_40', "    this.flags()\n}", "    *((this.addr() + contract::MORALE) as *const u32)\n}"),
]
