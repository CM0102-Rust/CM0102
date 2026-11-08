# Mutacoes da leva83 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva83.py
MUTACOES = [
    ('p_110', "        .wrapping_add((*(pessoa as *const i32) as usize).wrapping_mul(110))\n        .wrapping_add((k as isize * 20) as usize)\n}", "        .wrapping_add((*(pessoa as *const i32) as usize).wrapping_mul(112))\n        .wrapping_add((k as isize * 20) as usize)\n}"),
    ('p_ligar', "    *((bloco_da_pessoa(this, pessoa, k) + 0x44) as *mut u8) = 1;", "    *((bloco_da_pessoa(this, pessoa, k) + 0x44) as *mut u8) = 2;"),
    ('p_sinal', "    *((bloco_da_pessoa(this, pessoa, k) + 0x44) as *const i8) as i32", "    *((bloco_da_pessoa(this, pessoa, k) + 0x44) as *const u8) as i32"),
    ('p_3a', "    bloco_da_pessoa(this, pessoa, k) + 0x3a", "    bloco_da_pessoa(this, pessoa, k) + 0x3c"),
    ('c_ff', "    if selecao == 0 {\n        return 0xff;", "    if selecao == 0 {\n        return 0;"),
    ('c_2', "    if segunda(selecao) != 0 {\n        return 2;", "    if segunda(selecao) != 0 {\n        return 1;"),
    ('c_bit', "    (*((pessoa + staff::PLAYING_SQUAD) as *const u8) >> 3) & 1", "    (*((pessoa + staff::PLAYING_SQUAD) as *const u8) >> 2) & 1"),
    ('g_9b', "    *(G_DATABASE_NAMES_LANGUAGE as *const u32)\n}", "    *(G_PLAYER_INIT_LEVEL_BASE_TABLE as *const u32)\n}"),
]
