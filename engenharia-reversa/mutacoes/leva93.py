# Mutacoes da leva93 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva93.py --jogo 2001-08-20
MUTACOES = [
    ('k_centro', "        *b(0x8ea7) = 4;\n        *b(0x8ea8) = 11;", "        *b(0x8ea7) = 4;\n        *b(0x8ea8) = 10;"),
    ('k_nibble', "            move_to_cell(j, (casa & 0xf) as i8, (casa >> 4) as i8);\n            linhas(this);", "            move_to_cell(j, (casa >> 4) as i8, (casa & 0xf) as i8);\n            linhas(this);"),
    ('k_passou', "                d >= 0\n", "                d > 0\n"),
    ('k_sai', "        let sai_jogando = lado_bola as i32 == l as i32;", "        let sai_jogando = lado_bola as i32 != l as i32;"),
    ('k_2b', "            write_2b(j, 0);", "            write_2b(j, 1);"),
    ('k_19a', "    *((quem + 0x19a) as *mut u16) = if byte(this, 0x8ea8) != 0 { 0xb4 } else { 0 };", "    *((quem + 0x19a) as *mut u16) = if byte(this, 0x8ea8) != 0 { 0xb5 } else { 0 };"),
]
