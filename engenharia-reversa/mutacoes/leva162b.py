# Mutacoes refeitas da leva162: invertem as condicoes da pessoa da vaga, para saber se os caminhos rodam.
# Uso: python mutacoes.py mutacoes\leva162b.py --carreira 2002-06-01 --confirmar 2001-08-20
MUTACOES = [
    ('p_ficha_b', "    if core::ptr::read_unaligned((p + 0x61) as *const usize) == 0 {\n        core::ptr::write_unaligned(casa, -1);", "    if core::ptr::read_unaligned((p + 0x61) as *const usize) != 0 {\n        core::ptr::write_unaligned(casa, -1);"),
    ('p_nacao_b', "        if nacao == 0 || core::ptr::read_unaligned((nacao + nation::THREE_LETTER_NAME) as *const u32) != core::ptr::read_unaligned((clube + club::NATION) as *const u32) {", "        if nacao == 0 || core::ptr::read_unaligned((nacao + nation::THREE_LETTER_NAME) as *const u32) == core::ptr::read_unaligned((clube + club::NATION) as *const u32) {"),
]
