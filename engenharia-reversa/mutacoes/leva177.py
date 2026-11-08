# Mutacoes da leva170 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva170.py --carreira 2002-06-01
MUTACOES = [
    ('b_bits', "    (*((r + 0xb) as *const u8) >> 4) & 3", "    (*((r + 0xb) as *const u8) >> 5) & 3"),
    ('m_marca', "    (*((t + league::PROMOTED_COUNT) as *const u8) != 0 && *((t + league::LEAGUE_FLAGS) as *const u8) & 2 != 0) as u32", "    (*((t + league::PROMOTED_COUNT) as *const u8) != 0 && *((t + league::LEAGUE_FLAGS) as *const u8) & 1 != 0) as u32"),
    ('l_passo', "    core::ptr::read_unaligned((t + league::TABLE) as *const usize).wrapping_add((i as usize).wrapping_mul(59))", "    core::ptr::read_unaligned((t + league::TABLE) as *const usize).wrapping_add((i as usize).wrapping_mul(58))"),
]
