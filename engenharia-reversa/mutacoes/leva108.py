# Mutacoes da leva108 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva108.py --jogo 2001-08-20
MUTACOES = [
    ('l_tab', "    read_u8(0x9e7ad9 + kind as isize as usize * 13)", "    read_u8(0x9e7ad8 + kind as isize as usize * 13)"),
    ('l_grau', "    if g > 1 { g as u8 } else { 1 }", "    if g > 2 { g as u8 } else { 1 }"),
    ('l_sem', "    if (r.recurring_type() as i8) <= -1 {", "    if (r.recurring_type() as i8) <= 0 {"),
]
