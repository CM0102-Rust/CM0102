# Mutacoes refeitas da leva162: invertem as condicoes da pessoa da vaga, para saber se os caminhos rodam.
# Uso: python mutacoes.py mutacoes\leva162b.py --carreira 2002-06-01 --confirmar 2001-08-20
MUTACOES = [
    ('p_ficha_b', "    if p.player().is_null() {\n        core::ptr::write_unaligned(home, -1);", "    if !p.player().is_null() {\n        core::ptr::write_unaligned(home, -1);"),
    ('p_nacao_b', "        if national.is_null() || national.nation().addr() != club.nation().addr() {", "        if nation.is_null() || nation.three_letter_name() == (club.nation().addr() as u32) {"),
]
