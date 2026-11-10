# Mutacoes das ofertas de contrato. Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('c_clube', "    find_manager_offer(manager, |o| *(o as *const i32) == person && *((o + contract_offer::CLUB_ID) as *const i32) == club)", "    find_manager_offer(manager, |o| *(o as *const i32) == person)"),
    ('c_acertada', "*((o + contract_offer::STATE) as *const u8) == 2)", "*((o + contract_offer::STATE) as *const u8) >= 2)"),
    ('f_estado', "        2 | 3 | 4 | 0x10 | 0x14 => 1,", "        2 | 3 | 0x10 | 0x14 => 1,"),
    ('f_fase', "        0xf => (o.response_round() >= 3) as u32,", "        0xf => (o.response_round() >= 2) as u32,"),
]
