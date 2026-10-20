# Mutacoes do emprestimo, da lista de observados e do tecnico substituto. Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('e_vaga', "        core::ptr::write_unaligned((entrada_do_emprestimo(id) + 4) as *mut i32, -1);", "        core::ptr::write_unaligned((entrada_do_emprestimo(id) + 4) as *mut i32, 0);"),
    ('l_alvo', "            core::ptr::write_unaligned((v + 0x17) as *mut i32, -1);", "            core::ptr::write_unaligned((v + 0x17) as *mut i32, 0);"),
    ('s_empate', "        if melhor == 0 || reputacao(c) > reputacao(melhor) {", "        if melhor == 0 || reputacao(c) >= reputacao(melhor) {"),
]
