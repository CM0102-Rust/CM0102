# Mutacoes da leva108 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva108.py --jogo 2001-08-20
MUTACOES = [
    ('l_tab', "    *((0x9e7ad9 + kind as isize as usize * 13) as *const u8)", "    *((0x9e7ad8 + kind as isize as usize * 13) as *const u8)"),
    ('l_grau', "    if g > 1 { g as u8 } else { 1 }", "    if g > 2 { g as u8 } else { 1 }"),
    ('l_sem', "    if read_i8(r + physical_record::RECURRING_TYPE) <= -1 {", "    if read_i8(r + physical_record::RECURRING_TYPE) <= 0 {"),
]
