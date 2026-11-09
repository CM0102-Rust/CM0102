# Mutacoes da leva166 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva166.py --carreira 2002-06-01
MUTACOES = [
    ('o_passo', "    (*(t as *const usize)).wrapping_add(i as usize * 0x2b0)", "    (*(t as *const usize)).wrapping_add(i as usize * 0x2b4)"),
    ('h_forcar', "            if force != 0 || exe::database_is_national_team(c) != 0 || exe::human_manager_controls_reserve_team(humanos, human) != 0 {", "            if force != 0 || exe::human_manager_controls_reserve_team(humanos, human) != 0 {"),
    ('h_9a01', "&& core::ptr::read_unaligned((*(humanos as *const usize) + i as usize * 0xa6ef + 0x9a01) as *const u32) != 0", "&& core::ptr::read_unaligned((*(humanos as *const usize) + i as usize * 0xa6ef + 0x9a01) as *const u32) == 0"),
    ('c_k0', "                if v > 0x4b { 1 } else if v >= 0x28 { 2 } else if v >= 10 { 3 } else { low(-30) }", "                if v > 0x4b { 1 } else if v >= 0x29 { 2 } else if v >= 10 { 3 } else { low(-30) }"),
    ('c_k4', "            } else if v > -30 {\n                4", "            } else if v > -31 {\n                4"),
    ('c_k5', "            } else if ambicao > 5 && rep(0xb) > 0x4e2 {", "            } else if ambicao > 4 && rep(0xb) > 0x4e2 {"),
    ('c_tipo5', "    if c.contract_type() & 0x3f == 5 {", "    if c.contract_type() & 0x3f == 6 {"),
]
