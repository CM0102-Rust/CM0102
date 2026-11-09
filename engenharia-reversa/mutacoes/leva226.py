# Mutações da 226a leva (itens db_q13, db_q14, db_q15). Uso: python mutacoes.py mutacoes\leva226.py --carreira 2002-06-01
MUTACOES = [
    ('dq13_adjetivo', '(index as u8 as i8 as i32).wrapping_sub(1) as u32;', '(index as u8 as i8 as i32).wrapping_sub(2) as u32;'),
    ('dq13_atributo', '    exe::staff_compute_non_player_attribute(person, not_player, attribute)\n', '    exe::staff_compute_non_player_attribute(person, not_player, attribute) ^ 1\n'),
    ('dq13_reduzida', 'ability * read_f64(rdata::F64_0_1) + (v + v)', 'ability * read_f64(rdata::F64_0_1) * 2.0 + (v + v)'),
    ('dq14_peso', 'for _ in 0..3 { // 3 weighted draws', 'for _ in 0..2 { // 3 weighted draws'),
    ('dq14_cores', '    let (a, b) = if owner != 0 {', '    let (b, a) = if owner != 0 {'),
    ('dq14_vaga', '        v == 0 || v == person\n', '        v == person\n'),
    ('dq15_cargo', '            *((person + staff::JOB_FOR_CLUB) as *mut u8) = c; // club and club role', '            *((person + 0x3d) as *mut u8) = c ^ 1; // club and club role'),
    ('dq15_data', 'core::ptr::read_unaligned((data + 4) as *const u32));', 'core::ptr::read_unaligned((data + 4) as *const u32) + 1);'),
    ('dq15_casa', '    match (0..n).find(|&i| core::ptr::read_unaligned(home(i)) == 0) {', '    match (0..n).rev().find(|&i| core::ptr::read_unaligned(home(i)) == 0) {'),
]
