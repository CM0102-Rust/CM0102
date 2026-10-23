# Mutações da 216a leva (itens sim_lote_c1, sim_lote_c2, sim_lote_c3). Uso: python mutacoes.py mutacoes\leva216.py --carreira 2002-06-01
MUTACOES = [
    ('lc_rebaixamento', '    (*((liga + 0xc1) as *const u8) != 0) as u32 // tem rebaixamento', '    (*((liga + 0xc1) as *const u8) == 0) as u32 // tem rebaixamento'),
    ('lc_ano_relativo', 'd[1].wrapping_sub(base as u16)); // ano relativo', 'd[1].wrapping_sub(base as u16).wrapping_add(1)); // ano relativo'),
    ('lc_importancia', '    imp = imp.wrapping_add(10); // importância base', '    imp = imp.wrapping_add(11); // importância base'),
    ('cr_recordista', '    d(0); // identificação', '    core::ptr::write_unaligned(this as *mut u32, core::ptr::read_unaligned(origem as *const u32) ^ 1); // identificação'),
    ('cr_partida', '    d(8); // adversário', '    core::ptr::write_unaligned((this + 8) as *mut u32, core::ptr::read_unaligned((origem + 8) as *const u32) ^ 1); // adversário'),
    ('cr_sequencia', '    *(this as *mut u32) = *(origem as *const u32); // jogos', '    *(this as *mut u32) = *(origem as *const u32) ^ 1; // jogos'),
    ('ha_ninguem_assiste', '    0 // ninguém assiste', '    1 // ninguém assiste'),
    ('ha_apelido', '    achou // apelido', '    achou ^ 1 // apelido'),
    ('ha_motivo', '*((this + 0x18) as *mut u8) = *((origem + 0x18) as *const u8); // motivo', '*((this + 0x18) as *mut u8) = *((origem + 0x18) as *const u8) ^ 1; // motivo'),
]
