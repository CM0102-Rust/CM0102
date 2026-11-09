# Mutacoes do emprestimo, da lista de observados e do tecnico substituto. Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('e_vaga', "        core::ptr::write_unaligned((loan_state_entry(id) + 4) as *mut i32, -1);", "        core::ptr::write_unaligned((loan_state_entry(id) + 4) as *mut i32, 0);"),
    ('l_alvo', "            core::ptr::write_unaligned((v + shortlist_slot::OFFER_REF) as *mut i32, -1);", "            core::ptr::write_unaligned((v + shortlist_slot::OFFER_REF) as *mut i32, 0);"),
    ('s_empate', "        if best == 0 || injury_word_0c(c) > injury_word_0c(Staff(best)) {", "        if best == 0 || injury_word_0c(c.addr()) >= injury_word_0c(best) {"),
]
