# Mutacoes da leva81 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva81.py
MUTACOES = [
    ('c_14e', "    let base = if segunda != 0 { 0x14e } else { 0x12d };", "    let base = if segunda != 0 { 0x14c } else { 0x12d };"),
    ('c_12d', "    let base = if segunda != 0 { 0x14e } else { 0x12d };", "    let base = if segunda != 0 { 0x14e } else { 0x12f };"),
    ('r_tipo', "    *((registro + if tipo == 1 { 0x37 } else { 0x52 }) as *const u8)", "    *((registro + if tipo != 0 { 0x37 } else { 0x52 }) as *const u8)"),
    ('r_nulo', "    if registro == 0 {\n        return 1;\n    }\n    *((registro", "    if registro == 0 {\n        return 0;\n    }\n    *((registro"),
    ('r_palavra', "    if a as u16 != 0 {\n        return texto(a, 0);\n    }", "    if a != 0 {\n        return texto(a, 0);\n    }"),
]
