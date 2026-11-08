# Mutacoes das ofertas de contrato. Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('c_clube', "    find_manager_offer(manager, |o| *(o as *const i32) == person && *((o + 4) as *const i32) == clube)", "    find_manager_offer(manager, |o| *(o as *const i32) == person)"),
    ('c_acertada', "*((o + 0x50) as *const u8) == 2)", "*((o + 0x50) as *const u8) >= 2)"),
    ('f_estado', "        2 | 3 | 4 | 0x10 | 0x14 => 1,", "        2 | 3 | 0x10 | 0x14 => 1,"),
    ('f_fase', "        0xf => (*((o + 0x2e) as *const u8) >= 3) as u32,", "        0xf => (*((o + 0x2e) as *const u8) >= 2) as u32,"),
]
