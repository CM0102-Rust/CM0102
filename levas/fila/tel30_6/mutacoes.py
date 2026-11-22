# Mutations (tel30_6). The automatic career opens no screen: the chain runs none; these prove the comparer in a
# session of Lucas's (JOGAR_COMPARANDO.bat): the club screen > Squad, a click on the "Age" or "Wage" column header.
MUTACOES = [
    ('tel30_6_age_order', 'let d = read_i8(b + staff::AGE) as i32 - read_i8(a + staff::AGE) as i32;',
     'let d = read_i8(a + staff::AGE) as i32 - read_i8(b + staff::AGE) as i32;'),
    ('tel30_6_wage_order', 'let d = read_i32(cb + 0xc).wrapping_sub(read_i32(ca + 0xc));',
     'let d = read_i32(ca + 0xc).wrapping_sub(read_i32(cb + 0xc));'),
]
