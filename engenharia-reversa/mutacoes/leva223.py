# Mutações da 223a leva (itens db_q4, db_q5, db_q6). Uso: python mutacoes.py mutacoes\leva223.py --carreira 2002-06-01
MUTACOES = [
    ('dq4_temporada', '    tb as i32 - ta as i32 // a temporada mais nova primeiro', '    ta as i32 - tb as i32 // a temporada mais nova primeiro'),
    ('dq4_peso', '(0x23, 2), (0x24, 0)', '(0x23, 3), (0x24, 0)'),
    ('dq4_britanica', 'id == g(KEY_NATION_ENGLAND) || ', ''),
    ('dq5_basco', '    BASQUE_CLUBS.iter().any(|&a| id == g(a)) as u32', '    (!BASQUE_CLUBS.iter().any(|&a| id == g(a))) as u32'),
    ('dq5_listada', '*((v + 0x4e) as *const u8) & 1 == 0', '*((v + 0x4e) as *const u8) & 2 == 0'),
    ('dq5_emprestimo', '*((v + 0x4e) as *const u8) & 2 != 0) as u32', '*((v + 0x4e) as *const u8) & 1 != 0) as u32'),
    ('dq6_pedido', '*((v + 0x4e) as *const u8) & 8 != 0', '*((v + 0x4e) as *const u8) & 9 != 0'),
    ('dq6_nota', '(ability + 2 * (value as u8 as i8 as i32))', '(ability + 3 * (value as u8 as i8 as i32))'),
    ('dq6_reservas', '    1 // pode', '    0 // pode'),
]
