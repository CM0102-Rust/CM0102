# Mutacoes da leva93 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva93.py --jogo 2001-08-20
MUTACOES = [
    ('k_centro', "        *b(0x8ea7) = 4;\n        *b(0x8ea8) = 11;", "        *b(0x8ea7) = 4;\n        *b(0x8ea8) = 10;"),
    ('k_nibble', "            move_to_cell(j, (home & 0xf) as i8, (home >> 4) as i8);\n            rows(this);", "            move_to_cell(j, (home >> 4) as i8, (home & 0xf) as i8);\n            rows(this);"),
    ('k_passou', "                d >= 0\n", "                d > 0\n"),
    ('k_sai', "        let sai_jogando = side_ball as i32 == l as i32;", "        let sai_jogando = side_ball as i32 != l as i32;"),
    ('k_2b', "            write_2b(j, 0);", "            write_2b(j, 1);"),
    ('k_19a', "    *((who + match_player::FACING) as *mut u16) = if byte(this, 0x8ea8) != 0 { 0xb4 } else { 0 };", "    *((who + match_player::FACING) as *mut u16) = if byte(this, 0x8ea8) != 0 { 0xb5 } else { 0 };"),
]
