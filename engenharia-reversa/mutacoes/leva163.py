# Mutacoes da leva163 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva163.py --carreira 2002-06-01
MUTACOES = [
    ('g_indice', "    let i = high as usize + 4 * (low as usize + 3 * (attack as usize + 2 * slot as usize));", "    let i = high as usize + 4 * (low as usize + 3 * (slot as usize + 2 * attack as usize));"),
    ('m_grupo', "        0x10 | 0x20 | 0x40 => 0x70,", "        0x10 | 0x20 | 0x40 => 0x30,"),
    ('m_zera', "    write_u32(t + 0x187f, 0);", "    write_u32(t + 0x187f, 1);"),
    ('r_float', "    write_u32(r + club_record::SEASON_RATING, 0xbf800000);", "    write_u32(r + club_record::SEASON_RATING, 0x3f800000);"),
    ('r_byte', "    for o in [0x168, 0x183, 0x19e, 0x1d6, 0x210, 0x22b] {", "    for o in [0x168, 0x183, 0x19e, 0x1d6, 0x210] {"),
]
