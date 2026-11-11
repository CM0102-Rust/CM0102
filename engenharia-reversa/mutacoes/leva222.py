# Mutações da 222a leva (itens db_q1, db_q2, db_q3). Uso: python mutacoes.py mutacoes\leva222.py --carreira 2002-06-01
MUTACOES = [
    ('dq1_mesma_nacao', '        return 2; // same nation', '        return 3; // same nation'),
    ('dq1_reputacao_atual', '        2 => r(0), // escopo 2: a atual', '        2 => r(2), // escopo 2: a atual'),
    ('dq1_estrangeira', '    1 // estrangeira', '    0 // estrangeira'),
    ('dq2_aniversario', '        years.wrapping_sub(1) // has not had a birthday yet', '        years.wrapping_sub(2) // has not had a birthday yet'),
    ('dq2_europa', '    (g(continent) == g(KEY_CONTINENT_EUROPE)) as u32 // Europa', '    (g(continent) != g(0x9cfa10)) as u32 // Europa'),
    ('dq2_ue', '    if n.group_membership() == nation_group::EEC { return 1; } // EU nationality', '    if n.group_membership() == 3 { return 1; } // EU nationality'),
    ('dq3_listada', 'v.transfer_status() & 9 != 0', 'v.transfer_status() & 8 != 0'),
    ('dq3_arredonda', ' + read_f64(rdata::F64_0_5); // arredonda', ' + 0.0; // arredonda'),
    ('dq3_atributo_da_pessoa', '    exe::staff_compute_player_attribute(person, person.player(), attribute)\n', '    exe::staff_compute_player_attribute(person, person.player(), attribute).wrapping_add(1)\n'),
]
