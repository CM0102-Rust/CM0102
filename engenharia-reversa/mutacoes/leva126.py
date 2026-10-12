# Mutacoes da leva126 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva126.py --jogo 2001-10-01
MUTACOES = [
    ('d_folego', "let folego = sortear(d.wrapping_mul(d) / 50000);", "let folego = sortear(d.wrapping_mul(d) / 40000);"),
    ('d_ataque', "let r = sortear(byte(m, 0x8ea9) as i32 * 50);", "let r = sortear(byte(m, 0x8ea9) as i32 * 40);"),
    ('d_limpa', "if !(nota < at && sortear(5) != 0) {", "if !(nota < at && sortear(4) != 0) {"),
    ('d_registra', "if sortear(0x1e) == 0 && arriscado == 0 {", "if sortear(0x1d) == 0 && arriscado == 0 {"),
    ('d_cansa', "*w(e, 0x35) = (*w(e, 0x35)).wrapping_sub(0x19);", "*w(e, 0x35) = (*w(e, 0x35)).wrapping_sub(0x18);"),
    ('d_limpa_folego', "{ 0x3b } else { 0x46 }", "{ 0x3b } else { 0x45 }"),
    ('d_nota', "nota.wrapping_sub(at).wrapping_sub(200).clamp(-200, 200)", "nota.wrapping_sub(at).wrapping_sub(199).clamp(-200, 200)"),
    ('d_chao', "let mut k = 3;", "let mut k = 2;"),
    ('d_ganho', "let ganho: i16 = if dominou { 0x78 } else { 0xc8 };", "let ganho: i16 = if dominou { 0x78 } else { 0xc7 };"),
    ('d_passa', "*w(j, 0x19c) = (*w(j, 0x19c)).wrapping_add(at.wrapping_sub(nota)).clamp(-200, 200);", "*w(j, 0x19c) = (*w(j, 0x19c)).wrapping_add(at.wrapping_sub(nota)).clamp(-199, 200);"),
    ('d_cresce', "1.0499999523162842", "1.06"),
    ('d_zera', "    *b(m, 0x8ea9) = 0;\n}\n\n/// Despacho real", "    *b(m, 0x8ea9) = 1;\n}\n\n/// Despacho real"),
]
