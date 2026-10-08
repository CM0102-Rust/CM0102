# Mutacoes da leva64 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva64.py
MUTACOES = [
    ('l_tipo9', "    if tipo == 1 || tipo == 9 {", "    if tipo == 1 {"),
    ('l_lado', "    let clube = *((partida + if lado != 0 { 0x20 } else { 0x1c }) as *const usize);", "    let clube = *((partida + if lado != 0 { 0x1c } else { 0x20 }) as *const usize);"),
    ('l_40', "    if humano(clube, 0, byte(partida, 0x40) as u8, 0, 0) != 0 {", "    if humano(clube, 0, 0, 0, 0) != 0 {"),
    ('l_4000', "        (*((m as isize + 6371 * l + 0x9766) as *const u32) & 0x4000 != 0) as u8", "        (*((m as isize + 6371 * l + 0x9766) as *const u32) & 0x400 != 0) as u8"),
    ('l_modo', "        (modo == 3 || modo == 4) as u8", "        (modo == 3) as u8"),
]
