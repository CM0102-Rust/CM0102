# Mutacoes da leva64 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva64.py
MUTACOES = [
    ('l_tipo9', "    if kind == 1 || kind == 9 {", "    if kind == 1 {"),
    ('l_lado', "    let club = *((partida + if side != 0 { 0x20 } else { 0x1c }) as *const usize);", "    let club = *((partida + if side != 0 { 0x1c } else { 0x20 }) as *const usize);"),
    ('l_40', "    if exe::human_manager_is_human_club(club, 0, (byte(partida, fixture::HOME_SIDE_MARK) as u8) as u32, (0) as u32, 0) != 0 {", "    if exe::human_manager_is_human_club(club, 0, (0) as u32, (0) as u32, 0) != 0 {"),
    ('l_4000', "        (*((m as isize + 6371 * l + 0x9766) as *const u32) & 0x4000 != 0) as u8", "        (*((m as isize + 6371 * l + 0x9766) as *const u32) & 0x400 != 0) as u8"),
    ('l_modo', "        (mode == 3 || mode == 4) as u8", "        (mode == 3) as u8"),
]
