# Mutações da 224a leva (itens db_q7, db_q8, db_q9). Uso: python mutacoes.py mutacoes\leva224.py --carreira 2002-06-01
MUTACOES = [
    ('dq7_primeiro', 'for _ in 0..20 { // 20 rolls for the first name', 'for _ in 0..19 { // 20 rolls for the first name'),
    ('dq7_segundo', 'for _ in 0..20 { // 20 random rolls of the second name', 'for _ in 0..19 { // 20 random rolls of the second name'),
    ('dq7_copia', '        *((target + i) as *mut u8) = b;', '        *((target + i) as *mut u8) = b ^ 0x20;'),
    ('dq8_casa', '1 => (r(0x83), r(0x87)), // casa', '1 => (r(0x87), r(0x83)), // casa'),
    ('dq8_tecnico', '            return t; // the club’s manager', '            return 0; // the club’s manager'),
    ('dq8_ingles', 'club.wrapping_add(4); // language 0 or 1', 'club.wrapping_add(5); // language 0 or 1'),
    ('dq9_moral', '    let simples = morale.wrapping_add(d);', '    let simples = morale.wrapping_add(d).wrapping_add(1);'),
    ('dq9_vetor', '0x22, 0x11, 0x10, 0x17', '0x22, 0x10, 0x11, 0x17'),
    ('dq9_elenco', '    *p = mantem | e;', '    *p = mantem | e | 0x10;'),
]
