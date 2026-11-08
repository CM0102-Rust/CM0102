# Mutacoes do contrato acertado, do jogo pela selecao e da punicao guardada. Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('c_clube', "    *club = (*(G_CLUB_TABLE as *const usize) as isize + *((contract + contract::CLUB) as *const i32) as isize * 581) as usize;", "    *club = (*(G_CLUB_TABLE as *const usize) as isize + *((contract + contract::CLUB) as *const i32) as isize * 580) as usize;"),
    ('s_bits', "    *(bits as *mut u8) |= 1;", "    *(bits as *mut u8) |= 2;"),
    ('p_vaga', "        if *(p as *const i32) == id && *((p + 4) as *const i8) == slot {", "        if *(p as *const i32) == id {"),
]
