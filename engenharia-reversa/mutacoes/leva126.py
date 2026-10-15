# Mutacoes da leva126 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva126.py --jogo 2001-10-01
MUTACOES = [
    ('d_folego', "let folego = sortear(d.wrapping_mul(d) / 50000);\n    let (mut a", "let folego = sortear(d.wrapping_mul(d) / 40000);\n    let (mut a"),
    ('d_ataque', "let r = sortear(byte(m, 0x8ea9) as i32 * 50);", "let r = sortear(byte(m, 0x8ea9) as i32 * 40);"),
    ('d_limpa', "if !(nota < at && sortear(5) != 0) {", "if !(nota < at && sortear(4) != 0) {"),
    ('d_registra', "if sortear(0x1e) == 0 && arriscado == 0 {\n                    crate::comparacao::ramo(\"dividida so registra", "if sortear(0x1d) == 0 && arriscado == 0 {\n                    crate::comparacao::ramo(\"dividida so registra"),
    ('d_cansa', "*w(e, 0x35) = (*w(e, 0x35)).wrapping_sub(0x19);", "*w(e, 0x35) = (*w(e, 0x35)).wrapping_sub(0x18);"),
    ('d_limpa_folego', "{ 0x3b } else { 0x46 }", "{ 0x3b } else { 0x45 }"),
    ('d_nota', "nota.wrapping_sub(at).wrapping_sub(200).clamp(-200, 200)", "nota.wrapping_sub(at).wrapping_sub(199).clamp(-200, 200)"),
    ('d_chao', "dividida bola espirra\");\n            chamar(0x6cd770, m, [ub(j, 0x102), ub(j, 0x103), 1 | zero(j) << 8, 0, 0]);\n            if byte(m, 0x8eb2) == 0 {\n                let mut k = 3;", "dividida bola espirra\");\n            chamar(0x6cd770, m, [ub(j, 0x102), ub(j, 0x103), 1 | zero(j) << 8, 0, 0]);\n            if byte(m, 0x8eb2) == 0 {\n                let mut k = 2;"),
    ('d_ganho', "let ganho: i16 = if dominou { 0x78 } else { 0xc8 };\n        *w(e, 0x35)", "let ganho: i16 = if dominou { 0x78 } else { 0xc7 };\n        *w(e, 0x35)"),
    ('d_passa', "*w(j, 0x19c) = (*w(j, 0x19c)).wrapping_add(at.wrapping_sub(nota)).clamp(-200, 200);", "*w(j, 0x19c) = (*w(j, 0x19c)).wrapping_add(at.wrapping_sub(nota)).clamp(-199, 200);"),
    ('d_cresce', "dividida cresce\");\n        let p = f32_em(j + 0x1a2) as f64 * 1.0499999523162842;", "dividida cresce\");\n        let p = f32_em(j + 0x1a2) as f64 * 1.06;"),
    ('d_zera', "    *b(m, 0x8ea9) = 0;\n}\n\n/// Despacho real", "    *b(m, 0x8ea9) = 1;\n}\n\n/// Despacho real"),
]
