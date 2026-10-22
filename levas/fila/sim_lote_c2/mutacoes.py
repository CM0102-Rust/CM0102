# Mutacoes das copias dos recordes do clube. Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('cr_recordista', "    d(0); // identificação", "    core::ptr::write_unaligned(this as *mut u32, core::ptr::read_unaligned(origem as *const u32) ^ 1); // identificação"),
    ('cr_partida', "    d(8); // adversário", "    core::ptr::write_unaligned((this + 8) as *mut u32, core::ptr::read_unaligned((origem + 8) as *const u32) ^ 1); // adversário"),
    ('cr_sequencia', "    *(this as *mut u32) = *(origem as *const u32); // jogos", "    *(this as *mut u32) = *(origem as *const u32) ^ 1; // jogos"),
]
