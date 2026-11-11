# Mutacoes da leva170 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva170.py --carreira 2002-06-01
MUTACOES = [
    ('b_bits', "    (read_u8(r + staff_state::STATUS_FLAGS) >> 4) & 3", "    (read_u8(r + staff_state::STATUS_FLAGS) >> 5) & 3"),
    ('m_marca', "    (t.promoted_count() != 0 && (t.league_flags() as u8) & 2 != 0) as u32", "    (t.promoted_count() != 0 && (t.league_flags() as u8) & 1 != 0) as u32"),
    ('l_passo', "    t.table().wrapping_add((i as usize).wrapping_mul(league_table_row::SIZE))", "    t.table().wrapping_add((i as usize).wrapping_mul(58))"),
]
