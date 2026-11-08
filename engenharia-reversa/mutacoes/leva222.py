# Mutações da 222a leva (itens db_q1, db_q2, db_q3). Uso: python mutacoes.py mutacoes\leva222.py --carreira 2002-06-01
MUTACOES = [
    ('dq1_mesma_nacao', '        return 2; // mesma nação', '        return 3; // mesma nação'),
    ('dq1_reputacao_atual', '        2 => r(0), // escopo 2: a atual', '        2 => r(2), // escopo 2: a atual'),
    ('dq1_estrangeira', '    1 // estrangeira', '    0 // estrangeira'),
    ('dq2_aniversario', '        years.wrapping_sub(1) // ainda não fez aniversário', '        years.wrapping_sub(2) // ainda não fez aniversário'),
    ('dq2_europa', '    (g(continent) == g(KEY_CONTINENT_EUROPE)) as u32 // Europa', '    (g(continent) != g(0x9cfa10)) as u32 // Europa'),
    ('dq2_ue', '    if *((n + 0x7f) as *const u8) == 2 { return 1; } // nacionalidade da UE', '    if *((n + 0x7f) as *const u8) == 3 { return 1; } // nacionalidade da UE'),
    ('dq3_listada', '*((v + 0x4e) as *const u8) & 9 != 0', '*((v + 0x4e) as *const u8) & 8 != 0'),
    ('dq3_arredonda', ' + read_f64_at(0x968968); // arredonda', ' + 0.0; // arredonda'),
    ('dq3_atributo_da_pessoa', '    calcular(person, core::ptr::read_unaligned((person + staff::PLAYER) as *const usize), attribute)\n', '    calcular(person, core::ptr::read_unaligned((person + staff::PLAYER) as *const usize), attribute).wrapping_add(1)\n'),
]
