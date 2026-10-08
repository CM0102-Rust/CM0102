# Mutacoes da leva62 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva62.py
MUTACOES = [
    ('t_9ace', "        Some((_, r)) => *((r + 0x9ace) as *const u32),", "        Some((_, r)) => *((r + 0x9aca) as *const u32),"),
    ('t_400', "            if clube != 0 && *((*((clube + 0x53) as *const usize) + 0x88) as *const i32) < 400 {", "            if clube != 0 && *((*((clube + 0x53) as *const usize) + 0x88) as *const i32) <= 400 {"),
    ('t_saida1', "                if !saida.is_null() {\n                    *saida = 1;", "                if !saida.is_null() {\n                    *saida = 2;"),
    ('t_saida0', "    if !saida.is_null() {\n        *saida = 0;\n    }\n    match pessoa_e_registro", "    match pessoa_e_registro"),
    ('t_9a06', "            (*((r + 0x9a06) as *const u8) != 0) as u8", "            (*((r + 0x9a06) as *const u8) & 0x7f != 0) as u8"),
    ('t_atual', "    let p = if pessoa != 0 { pessoa } else { *((0xb74c26 + 768 *", "    let p = if pessoa != 0 { pessoa } else { *((0xb74c26 + 0 *"),
]
