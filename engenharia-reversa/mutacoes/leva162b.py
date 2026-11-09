# Mutacoes refeitas da leva162: invertem as condicoes da pessoa da vaga, para saber se os caminhos rodam.
# Uso: python mutacoes.py mutacoes\leva162b.py --carreira 2002-06-01 --confirmar 2001-08-20
MUTACOES = [
    ('p_ficha_b', "    if core::ptr::read_unaligned((p + staff::PLAYER) as *const usize) == 0 {\n        core::ptr::write_unaligned(home, -1);", "    if core::ptr::read_unaligned((p + staff::PLAYER) as *const usize) != 0 {\n        core::ptr::write_unaligned(home, -1);"),
    ('p_nacao_b', "        if nation == 0 || core::ptr::read_unaligned((nation + nation::THREE_LETTER_NAME) as *const u32) != core::ptr::read_unaligned((club.addr() + club::NATION) as *const u32) {", "        if nation == 0 || core::ptr::read_unaligned((nation + nation::THREE_LETTER_NAME) as *const u32) == core::ptr::read_unaligned((club.addr() + club::NATION) as *const u32) {"),
]
