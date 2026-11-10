# Mutacoes da leva62 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva62.py
MUTACOES = [
    ('t_9ace', "        Some((_, r)) => read_u32(r + 0x9ace),", "        Some((_, r)) => read_u32(r + 0x9aca),"),
    ('t_400', "            if !club.is_null() && club.nation().number_staff() < 400 {", "            if !club.is_null() && club.nation().number_staff() <= 400 {"),
    ('t_saida1', "                if !out.is_null() {\n                    *out = 1;", "                if !out.is_null() {\n                    *out = 2;"),
    ('t_saida0', "    if !out.is_null() {\n        *out = 0;\n    }\n    match person_and_human_record", "    match person_and_human_record"),
    ('t_9a06', "            (read_u8(r + 0x9a06) != 0) as u8", "            (read_u8(r + 0x9a06) & 0x7f != 0) as u8"),
    ('t_atual', "    let p = if person != 0 { person } else { *((0xb74c26 + 768 *", "    let p = if person != 0 { person } else { *((0xb74c26 + 0 *"),
]
