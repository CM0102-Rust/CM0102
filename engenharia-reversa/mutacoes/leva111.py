# Mutacoes da leva111 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva111.py --jogo 2001-08-20
MUTACOES = [
    ('a_y', "    if !(read_f64(rdata::F64_NEG_4) <= y) {\n        y = -4.0;", "    if !(read_f64(rdata::F64_NEG_4) <= y) {\n        y = -3.0;"),
    ('a_contrato', "            2 => read_f64(rdata::F64_2),", "            2 => read_f64(rdata::F64_3),"),
    ('a_dias', "    let f = if a > read_f64(rdata::F64_1000) { (a - read_f64(rdata::F64_1000)) * read_f64(rdata::F64_0_000125) + read_f64(rdata::F64_1) } else { 1.0 };", "    let f = if a > read_f64(rdata::F64_1000) { (a - read_f64(rdata::F64_1000)) * read_f64(rdata::F64_0_000125) + read_f64(rdata::F64_1) } else { 0.9 };"),
    ('a_enc', "    let base = sheet.leadership() as f64 * f - (enc * enc + read_f64(rdata::F64_5)) * read_f64(rdata::F64_0_1);", "    let base = sheet.leadership() as f64 * f - (enc + read_f64(rdata::F64_5)) * read_f64(rdata::F64_0_1);"),
    ('a_idade', "    } else if age < 0x15 {\n        Some(read_f64(rdata::F64_0_75))", "    } else if age < 0x16 {\n        Some(read_f64(rdata::F64_0_75))"),
    ('a_59', "    if a < ((0x32 - p.loyalty() as i32) * 10) as f64 {", "    if a < ((0x31 - p.loyalty() as i32) * 10) as f64 {"),
    ('a_moral', "    if morale < 10 {", "    if morale < 9 {"),
]
