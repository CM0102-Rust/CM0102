# Mutacoes da leva139 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva139.py --carreira 2002-06-01 --confirmar 2001-10-01
MUTACOES = [
    ('f_favorito', "    [0x9b, 0x9f, 0xa3].iter().any(|&o| *((b + o) as *const usize) == a) as u32", "    [0x9b, 0x9f].iter().any(|&o| *((b + o) as *const usize) == a) as u32"),
    ('f_rejeitado', "    [0xa7, 0xab, 0xaf].iter().any(|&o| *((b + o) as *const usize) == a) as u32", "    [0xa7, 0xab].iter().any(|&o| *((b + o) as *const usize) == a) as u32"),
    ('f_insatisfacao', "let r1 = sortear((a * b as i32 * c as i32 + 0x14) * 100);", "let r1 = sortear((a * b as i32 * c as i32 + 0x15) * 100);", "match_wrap_up insatisfeito"),
    ('f_rival', "if [0xb3, 0xb7, 0xbb].iter().any(|&o| d(meu, o) == outro) {\n                        x += 10;", "if [0xb3, 0xb7, 0xbb].iter().any(|&o| d(meu, o) == outro) {\n                        x += 11;"),
    ('f_gosta', "    } else if gosta(pessoa, outro) != 0 {\n                        x -= 20;", "    } else if gosta(pessoa, outro) != 0 {\n                        x -= 19;"),
    ('f_media', "media = if d(pt, 0x18) == d(meu, 0x69) { q - read_f64_at(0x967880) } else { q + read_f64_at(0x968918) };", "media = if d(pt, 0x18) == d(meu, 0x69) { q - read_f64_at(0x967880) * 1.01 } else { q + read_f64_at(0x968918) };", "match_wrap_up media do lado"),
    ('f_noventa', "    } else if *(m as *const u8) != 0 {\n                0x5a", "    } else if *(m as *const u8) != 0 {\n                0x59", "match_wrap_up media do lado"),
    ('f_minuto_reserva', "                fim.wrapping_sub(c)", "                fim.wrapping_sub(c).wrapping_add(1)", "match_wrap_up media do lado"),
    ('f_tipo6', "byte(p, 0x24) == 2 || byte(p, 0x23) != 0", "byte(p, 0x24) == 2 || byte(p, 0x23) == 0", "match_wrap_up evento tipo 6"),
    ('f_evento_jogador', "byte(p, 0x1b) < 7 && sortear(byte(p, 0x17) as i32) == 0", "byte(p, 0x1b) < 8 && sortear(byte(p, 0x17) as i32) == 0", "match_wrap_up evento do dia"),
    ('f_proposta', "if r1 > sortear(0x14) && byte(x, 0x14) > 5 {", "if r1 > sortear(0x15) && byte(x, 0x14) > 5 {", "match_wrap_up evento do dia"),
    ('f_lixo_evento', "            ev[0xd] = lado as u8;\n                            ev[0xf..0x13].copy_from_slice(&u32::MAX.to_le_bytes());\n                            ev[0x13..0x15].copy_from_slice(&11u16.to_le_bytes());", "            ev[0xd] = lado as u8;\n                            ev[0xf..0x13].copy_from_slice(&u32::MAX.to_le_bytes());\n                            ev[0x13..0x15].copy_from_slice(&11u16.to_le_bytes());\n                            ev[0x16] = 1;", "match_wrap_up insatisfeito"),
]
