# Mutacoes refeitas da leva150 (as que escaparam, mais fortes). Uso: python mutacoes.py mutacoes\leva150b.py --carreira 2002-06-01
MUTACOES = [
    ('a_dias_forte', "[Some(0x96d4c0usize), Some(0x96d4b8)", "[Some(0x968a90usize), Some(0x968a90)"),
    ('a_ja_jogou_sempre', "                if exe::transfer_manager_get_ineligibility_code(u(G_TRANSFER_MANAGER, 0), p, x) != 0 || exe::nationalteams_get_international_duty_club(", "                if true || exe::nationalteams_get_international_duty_club("),
    ('c_selecao_forte', "    let mut mask: u32 = if exe::database_is_national_team(club) != 0 { 0x208a09c } else { 0xca0ca };", "    let mut mask: u32 = if exe::database_is_national_team(club) != 0 { 0x208a09c } else { 0 };"),
    ('o_rodizio_nenhum', "        while r != 0 && *((f + 0x2e) as *mut u8) != 2 {", "        while r != 0 && *((f + 0x2e) as *mut u8) != 0 {"),
    ('g_postura3_forte', "                g(if e(0xe) > 0x10 { 0x20000 } else { 0x8000 });", "                g(if e(0xe) > 0x10 { 0x8000 } else { 0x20000 });"),
]
