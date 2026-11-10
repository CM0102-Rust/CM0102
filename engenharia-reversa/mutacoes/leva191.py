# Mutacoes do contrato acertado, do jogo pela selecao e da punicao guardada. Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('c_clube', "    *club = (read_usize(G_CLUB_TABLE) as isize + (contract.club() as i32) as isize * 581) as usize;", "    *club = (read_usize(G_CLUB_TABLE) as isize + (contract.club() as i32) as isize * 580) as usize;"),
    ('s_bits', "    *(bits as *mut u8) |= 1;", "    *(bits as *mut u8) |= 2;"),
    ('p_vaga', "        if read_i32(p) == id && *((p + held_ban::SLOT) as *const i8) == slot {", "        if read_i32(p) == id {"),
]
