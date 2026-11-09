# Mutacoes refeitas da leva162: invertem as condicoes da pessoa da vaga, para saber se os caminhos rodam.
# Uso: python mutacoes.py mutacoes\leva162b.py --carreira 2002-06-01 --confirmar 2001-08-20
MUTACOES = [
    ('p_ficha_b', "    if p.player() == 0 {\n        core::ptr::write_unaligned(home, -1);", "    if p.player() != 0 {\n        core::ptr::write_unaligned(home, -1);"),
    ('p_nacao_b', "        if nation == 0 || core::ptr::read_unaligned((nation + nation::THREE_LETTER_NAME) as *const u32) != (club.nation().addr() as u32) {", "        if nation == 0 || core::ptr::read_unaligned((nation + nation::THREE_LETTER_NAME) as *const u32) == (club.nation().addr() as u32) {"),
]
