# Mutacoes das regras da rodada, da chave do jogo e do pais da excursao. Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('r_ou', "    core::ptr::write_unaligned(marcas, m);\n    if m & 0x800 == 0 {", "    if m & 0x800 == 0 {"),
    ('c_lados', "        for o in [0, 2] {", "        for o in [2] {"),
    ('p_maior', "                if r > melhor {", "                if r < melhor {"),
]
