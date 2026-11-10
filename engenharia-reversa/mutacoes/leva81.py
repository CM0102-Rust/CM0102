# Mutacoes da leva81 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva81.py
MUTACOES = [
    ('c_14e', "    let base = if second != 0 { 0x14e } else { 0x12d };", "    let base = if second != 0 { 0x14c } else { 0x12d };"),
    ('c_12d', "    let base = if second != 0 { 0x14e } else { 0x12d };", "    let base = if second != 0 { 0x14e } else { 0x12f };"),
    ('r_tipo', "    read_u8(record + if kind == 1 { 0x37 } else { 0x52 })", "    read_u8(record + if kind != 0 { 0x37 } else { 0x52 })"),
    ('r_nulo', "    if record == 0 {\n        return 1;", "    if record == 0 {\n        return 0;"),
    ('r_palavra', "    if a as u16 != 0 {\n        return exe::comp_util_get_stage_name_gender(a, 0);\n    }", "    if a != 0 {\n        return exe::comp_util_get_stage_name_gender(a, 0);\n    }"),
]
