# Mutacoes da leva111 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva111.py --jogo 2001-08-20
MUTACOES = [
    ('a_y', "    if !(d8(0x96d220) <= y) {\n        y = -4.0;", "    if !(d8(0x96d220) <= y) {\n        y = -3.0;"),
    ('a_contrato', "            2 => d8(0x968950),", "            2 => d8(0x968930),"),
    ('a_dias', "    let f = if a > d8(0x968908) { (a - d8(0x968908)) * d8(0x96a6b8) + d8(0x967890) } else { 1.0 };", "    let f = if a > d8(0x968908) { (a - d8(0x968908)) * d8(0x96a6b8) + d8(0x967890) } else { 0.9 };"),
    ('a_enc', "    let base = read_i8(sheet + player_sheet::LEADERSHIP) as f64 * f - (enc * enc + d8(0x9688a0)) * d8(0x967880);", "    let base = read_i8(sheet + player_sheet::LEADERSHIP) as f64 * f - (enc + d8(0x9688a0)) * d8(0x967880);"),
    ('a_idade', "    } else if age < 0x15 {\n        Some(d8(0x969040))", "    } else if age < 0x16 {\n        Some(d8(0x969040))"),
    ('a_59', "    if a < ((0x32 - p.loyalty() as i32) * 10) as f64 {", "    if a < ((0x31 - p.loyalty() as i32) * 10) as f64 {"),
    ('a_moral', "    if morale < 10 {", "    if morale < 9 {"),
]
